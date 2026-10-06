//! Owns serialized remote projection artifact envelopes and their resolution outcomes.
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::time::Duration;

use sha2::{Digest, Sha256};

use super::{
    Containment, PROJECTION_SCHEMA, Projection, ProjectionError, ProjectionSource,
    RUSTDOC_TOOLCHAIN, ResolutionEvent, ResolutionSource, ResolutionStatus, RustDependency,
    io_error,
};

use serde::{Deserialize, Serialize};

pub(super) fn fetch_remote_projection(
    identity: &str,
    target: &str,
    dependencies: &[RustDependency],
    containment: Containment,
) -> Result<PublishedProjection, ProjectionError> {
    let Some(base_url) = std::env::var_os("TERRANE_PROJECTION_ARTIFACT_URL") else {
        return Ok(PublishedProjection::Event(ResolutionEvent {
            source: ResolutionSource::PublishedArtifact,
            status: ResolutionStatus::Skipped,
            reason: "`TERRANE_PROJECTION_ARTIFACT_URL` is not configured".to_owned(),
        }));
    };
    let base_url = base_url.to_string_lossy();
    if !base_url.starts_with("https://") {
        return Err(ProjectionError {
            message:
                "`TERRANE_PROJECTION_ARTIFACT_URL` must name a trusted HTTPS artifact repository"
                    .to_owned(),
        });
    }
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let url = format!("{}/{}.json", base_url.trim_end_matches('/'), identity);
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .https_only(true)
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .into();
    let mut response = match agent.get(&url).call() {
        Ok(response) => response,
        Err(error) => {
            return Ok(PublishedProjection::Event(ResolutionEvent {
                source: ResolutionSource::PublishedArtifact,
                status: ResolutionStatus::Miss,
                reason: format!("artifact request `{url}` was unavailable: {error}"),
            }));
        }
    };
    let bytes = match response
        .body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_to_vec()
    {
        Ok(bytes) => bytes,
        Err(error) => {
            return Ok(PublishedProjection::Event(ResolutionEvent {
                source: ResolutionSource::PublishedArtifact,
                status: ResolutionStatus::Rejected,
                reason: format!("artifact body could not be read: {error}"),
            }));
        }
    };
    let artifact = match serde_json::from_slice::<ProjectionArtifact>(&bytes) {
        Ok(artifact) => artifact,
        Err(error) => {
            return Ok(PublishedProjection::Event(ResolutionEvent {
                source: ResolutionSource::PublishedArtifact,
                status: ResolutionStatus::Rejected,
                reason: format!("artifact JSON did not match the projection envelope: {error}"),
            }));
        }
    };
    match validate_projection_artifact(artifact, identity, target, dependencies, containment) {
        Ok(projection) => Ok(PublishedProjection::Hit(projection)),
        Err(reason) => Ok(PublishedProjection::Event(ResolutionEvent {
            source: ResolutionSource::PublishedArtifact,
            status: ResolutionStatus::Rejected,
            reason,
        })),
    }
}

fn artifact_dependency_mismatch(
    actual: &[ArtifactDependency],
    expected: &[ArtifactDependency],
) -> Option<String> {
    if actual.len() != expected.len() {
        return Some(format!(
            "dependency metadata mismatch: expected {} entries, found {}",
            expected.len(),
            actual.len()
        ));
    }
    for (actual, expected) in actual.iter().zip(expected) {
        if actual.name != expected.name
            || actual.package != expected.package
            || actual.version != expected.version
            || actual.effects != expected.effects
        {
            return Some(format!(
                "dependency metadata mismatch for `{}`",
                expected.name
            ));
        }
        if actual.features != expected.features
            || actual.default_features != expected.default_features
        {
            return Some(format!("feature metadata mismatch for `{}`", expected.name));
        }
        if actual.target != expected.target {
            return Some(format!(
                "dependency target mismatch for `{}`: expected `{}`, found `{}`",
                expected.name,
                expected.target.as_deref().unwrap_or("all targets"),
                actual.target.as_deref().unwrap_or("all targets")
            ));
        }
    }
    None
}

fn validate_projection_artifact(
    artifact: ProjectionArtifact,
    identity: &str,
    target: &str,
    dependencies: &[RustDependency],
    containment: Containment,
) -> Result<Projection, String> {
    let expected_dependencies = dependencies
        .iter()
        .map(ArtifactDependency::from)
        .collect::<Vec<_>>();
    let mismatch = if artifact.format != 1 {
        Some(format!(
            "envelope format {} is not supported",
            artifact.format
        ))
    } else if artifact.cache_identity != identity {
        Some("cache identity mismatch".to_owned())
    } else if artifact.target != target {
        Some(format!(
            "target mismatch: expected `{target}`, found `{}`",
            artifact.target
        ))
    } else if artifact.build_toolchain != crate::BUILD_TOOLCHAIN {
        Some("stable build toolchain mismatch".to_owned())
    } else if artifact.rustdoc_toolchain != RUSTDOC_TOOLCHAIN {
        Some("rustdoc toolchain mismatch".to_owned())
    } else if artifact.rustdoc_format != rustdoc_types::FORMAT_VERSION {
        Some("rustdoc format mismatch".to_owned())
    } else if artifact.projection_schema != PROJECTION_SCHEMA {
        Some("projection schema mismatch".to_owned())
    } else if let Some(reason) =
        artifact_dependency_mismatch(&artifact.dependencies, &expected_dependencies)
    {
        Some(reason)
    } else if artifact.projection.cache_identity != identity {
        Some("projection payload identity mismatch".to_owned())
    } else {
        None
    };
    if let Some(reason) = mismatch {
        return Err(reason);
    }
    let computed = projection_content_hash(&artifact.projection).map_err(|error| error.message)?;
    if artifact.content_hash != computed {
        return Err(format!(
            "content hash mismatch: envelope recorded `{}`, computed `{computed}`",
            artifact.content_hash
        ));
    }
    if artifact.projection.content_hash != artifact.content_hash {
        return Err("projection payload content hash does not match its envelope".to_owned());
    }
    let mut projection = artifact.projection;
    projection.source = ProjectionSource::Remote;
    projection.containment = containment;
    Ok(projection)
}

pub(super) fn projection_content_hash(projection: &Projection) -> Result<String, ProjectionError> {
    let payload = serde_json::to_vec(&(
        &projection.cache_identity,
        &projection.dependencies,
        &projection.native_owner_aliases,
        &projection.probes,
        projection.probe_wall_time_ms,
    ))
    .map_err(|error| ProjectionError {
        message: format!("cannot encode projection content hash: {error}"),
    })?;
    Ok(format!("{:x}", Sha256::digest(payload)))
}

pub(super) fn remove_legacy_projection_cache(directory: &Path) -> Result<(), ProjectionError> {
    for entry in fs::read_dir(directory).map_err(io_error("read dependency projection cache"))? {
        let entry = entry.map_err(io_error("read dependency projection cache entry"))?;
        let path = entry.path();
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        let json_stem = name.strip_suffix(".json").unwrap_or_default();
        let identity_stem = name.strip_suffix(".identity").unwrap_or_default();
        let legacy_projection = json_stem
            .strip_prefix("projection-")
            .is_some_and(is_legacy_cache_hash);
        let legacy_owner = json_stem
            .strip_prefix("owner-rustdoc-")
            .is_some_and(is_legacy_cache_hash)
            || identity_stem
                .strip_prefix("owner-rustdoc-")
                .is_some_and(is_legacy_cache_hash);
        if (legacy_projection || legacy_owner)
            && entry
                .file_type()
                .map_err(io_error("read legacy projection cache type"))?
                .is_file()
        {
            fs::remove_file(path).map_err(io_error("remove legacy projection cache"))?;
        }
    }
    Ok(())
}

fn is_legacy_cache_hash(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[derive(Deserialize)]
pub(super) struct ProjectionCacheIdentity<'a> {
    #[serde(borrow)]
    pub(super) cache_identity: &'a str,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(super) struct ProjectionArtifact {
    pub(super) format: u32,
    pub(super) cache_identity: String,
    pub(super) content_hash: String,
    pub(super) target: String,
    pub(super) build_toolchain: String,
    pub(super) rustdoc_toolchain: String,
    pub(super) rustdoc_format: u32,
    pub(super) projection_schema: String,
    pub(super) dependencies: Vec<ArtifactDependency>,
    pub(super) projection: Projection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(super) struct ArtifactDependency {
    pub(super) name: String,
    pub(super) package: String,
    pub(super) version: String,
    pub(super) features: Vec<String>,
    pub(super) default_features: bool,
    pub(super) target: Option<String>,
    pub(super) effects: Vec<String>,
}

impl From<&RustDependency> for ArtifactDependency {
    fn from(dependency: &RustDependency) -> Self {
        Self {
            name: dependency.name.clone(),
            package: dependency.package.clone(),
            version: dependency.version.clone(),
            features: dependency.features.clone(),
            default_features: dependency.default_features,
            target: dependency.target.clone(),
            effects: dependency.effects.clone(),
        }
    }
}

pub(super) enum PublishedProjection {
    Hit(Projection),
    Event(super::ResolutionEvent),
}
pub(super) fn write_cache_atomically(
    path: &Path,
    contents: &[&[u8]],
) -> Result<(), ProjectionError> {
    static TEMP_SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let parent = path.parent().ok_or_else(|| ProjectionError {
        message: format!("cache path `{}` has no parent directory", path.display()),
    })?;
    fs::create_dir_all(parent).map_err(io_error("create dependency cache directory"))?;
    let name = path.file_name().ok_or_else(|| ProjectionError {
        message: format!("cache path `{}` has no file name", path.display()),
    })?;
    let sequence = TEMP_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let temporary = parent.join(format!(
        ".{}.{}.{}.tmp",
        name.to_string_lossy(),
        std::process::id(),
        sequence
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(io_error("create temporary dependency cache"))?;
    for content in contents {
        if let Err(error) = file.write_all(content) {
            drop(file);
            let _ = fs::remove_file(&temporary);
            return Err(io_error("write temporary dependency cache")(error));
        }
    }
    drop(file);
    if let Err(error) = fs::rename(&temporary, path) {
        let _ = fs::remove_file(&temporary);
        return Err(io_error("replace dependency cache")(error));
    }
    Ok(())
}

pub(super) fn write_if_changed(path: &Path, content: &[u8]) -> Result<(), ProjectionError> {
    if fs::read(path).is_ok_and(|existing| existing == content) {
        return Ok(());
    }
    fs::write(path, content).map_err(io_error("write dependency projection input"))
}
#[cfg(test)]
mod tests;
