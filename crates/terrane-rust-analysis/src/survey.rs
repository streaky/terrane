use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    AnalysisError, BoundQuestion, CallProbeReport, CallQuestion, Containment, ImplProbeReport,
    ImplQuestion, ProbeReport, ProjectionOracle, RUSTDOC_TOOLCHAIN,
    configure_projection_cargo_command, parse_rustdoc, public_paths,
};

/// Exact native graph and public API evidence for one selected package.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SurveyReport {
    pub manifest: String,
    pub selected_package: SurveyPackage,
    pub resolved_packages: Vec<SurveyPackage>,
    pub public_paths: Vec<String>,
    pub declarations: Vec<SurveyDeclaration>,
    pub discovery_failures: Vec<SurveyDiscoveryFailure>,
    pub target: String,
    pub build_toolchain: String,
    pub rustdoc_format: u32,
    pub rustdoc_toolchain: String,
    pub containment: Containment,
    pub probe_execution: SurveyProbeExecution,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SurveyPackage {
    pub identity: String,
    pub name: String,
    pub version: String,
    pub source: String,
    pub checksum: Option<String>,
    pub manifest: String,
    pub content_fingerprint: Option<String>,
    pub features: Vec<String>,
    pub dependencies: Vec<String>,
}

/// One effective public declaration with its rustdoc-native signature shape.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SurveyDeclaration {
    pub public_path: String,
    pub canonical_path: Vec<String>,
    pub kind: String,
    pub signature: serde_json::Value,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SurveyDiscoveryFailure {
    pub public_path: String,
    pub reason: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SurveyProbeRequest {
    #[serde(default)]
    pub bounds: Vec<BoundQuestion>,
    #[serde(default)]
    pub calls: Vec<CallQuestion>,
    #[serde(default)]
    pub implementations: Vec<ImplQuestion>,
}

/// Exact probes compile against the resolved graph and never execute package code.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SurveyProbeExecution {
    pub status: String,
    pub compile_only: bool,
    pub bounds: ProbeReport,
    pub calls: CallProbeReport,
    pub implementations: ImplProbeReport,
}

#[derive(Deserialize)]
struct Metadata {
    packages: Vec<MetadataPackage>,
    resolve: MetadataResolve,
}

#[derive(Deserialize)]
struct MetadataResolve {
    root: Option<String>,
    nodes: Vec<MetadataNode>,
}

#[derive(Deserialize)]
struct MetadataNode {
    id: String,
    dependencies: Vec<String>,
    features: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MetadataPackage {
    id: String,
    name: String,
    version: String,
    source: Option<String>,
    manifest_path: PathBuf,
    targets: Vec<MetadataTarget>,
}

#[derive(Debug, Deserialize)]
struct MetadataTarget {
    name: String,
    kind: Vec<String>,
}

#[derive(Deserialize)]
struct CargoLock {
    #[serde(default)]
    package: Vec<LockPackage>,
}

#[derive(Deserialize)]
struct LockPackage {
    name: String,
    version: String,
    source: Option<String>,
    checksum: Option<String>,
}

/// Resolves a locked Cargo graph for the host target and reports one library package.
///
/// # Errors
///
/// Returns an error when resolution, pinned Rustdoc generation, or decoding fails.
pub fn survey_package(
    manifest: &Path,
    package: Option<&str>,
) -> Result<SurveyReport, AnalysisError> {
    let target = host_target()?;
    survey_package_for_target(manifest, package, &target)
}

/// Resolves a locked Cargo graph for `target` and reports one library package.
///
/// # Errors
///
/// Returns an error when resolution, pinned Rustdoc generation, or decoding fails.
pub fn survey_package_for_target(
    manifest: &Path,
    package: Option<&str>,
    target: &str,
) -> Result<SurveyReport, AnalysisError> {
    survey_package_for_target_with_probes(manifest, package, target, &SurveyProbeRequest::default())
}

/// Resolves and surveys one package, compiling every supplied exact probe without executing it.
///
/// # Errors
///
/// Returns an error when graph discovery, Rustdoc generation, or probe execution cannot complete.
pub fn survey_package_for_target_with_probes(
    manifest: &Path,
    package: Option<&str>,
    target: &str,
    probes: &SurveyProbeRequest,
) -> Result<SurveyReport, AnalysisError> {
    let manifest = manifest
        .canonicalize()
        .map_err(io_error("canonicalize survey manifest"))?;
    let workspace = manifest.parent().ok_or_else(|| AnalysisError {
        message: format!(
            "survey manifest `{}` has no parent directory",
            manifest.display()
        ),
    })?;
    let metadata = cargo_metadata(&manifest, target)?;
    let selected = select_package(&metadata, package)?;
    let target_name = selected
        .targets
        .iter()
        .find(|candidate| candidate.kind.iter().any(|kind| kind == "lib"))
        .ok_or_else(|| AnalysisError {
            message: format!("selected package `{}` has no library target", selected.name),
        })?
        .name
        .clone();
    let closure = selected_closure(&metadata, &selected.id);
    let lock = cargo_lock(workspace)?;
    let packages = package_records(&metadata, &closure, workspace, &lock)?;
    let target_directory = workspace.join(".trn/dependencies/rust-survey/target");
    run_rustdoc(&manifest, selected, target, &target_directory)?;
    let rustdoc_path = target_directory
        .join(target)
        .join("doc")
        .join(format!("{target_name}.json"));
    let fallback = target_directory
        .join("doc")
        .join(format!("{target_name}.json"));
    let rustdoc_path = if rustdoc_path.exists() {
        rustdoc_path
    } else {
        fallback
    };
    let bytes = fs::read(&rustdoc_path).map_err(io_error("read generated survey rustdoc"))?;
    let document = parse_rustdoc(&selected.name, &bytes, RUSTDOC_TOOLCHAIN)?;
    let paths = public_paths(&document);
    let (declarations, discovery_failures) = declarations(&document, &paths)?;
    let public_paths = paths.into_values().collect::<Vec<_>>();
    let selected_identity = package_identity(selected);
    let oracle_identity = format!("survey-{selected_identity}-{target}");
    let oracle = ProjectionOracle::new(workspace, &oracle_identity, Containment::Unavailable);
    let bound_report = oracle.prove_bounds(&probes.bounds)?;
    let call_report = oracle.prove_calls(&probes.calls)?;
    let impl_report = oracle.prove_impls(&probes.implementations)?;
    let selected_package = packages
        .iter()
        .find(|record| record.identity == selected_identity)
        .cloned()
        .ok_or_else(|| AnalysisError {
            message: "selected package missing from resolved closure".to_owned(),
        })?;
    Ok(SurveyReport {
        manifest: "Cargo.toml".to_owned(),
        selected_package,
        resolved_packages: packages,
        public_paths,
        declarations,
        discovery_failures,
        target: target.to_owned(),
        build_toolchain: rustc_version()?,
        rustdoc_format: document.format_version,
        rustdoc_toolchain: RUSTDOC_TOOLCHAIN.to_owned(),
        containment: Containment::Unavailable,
        probe_execution: SurveyProbeExecution {
            status: if probes.bounds.is_empty()
                && probes.calls.is_empty()
                && probes.implementations.is_empty()
            {
                "not-requested"
            } else {
                "completed"
            }
            .to_owned(),
            compile_only: true,
            bounds: bound_report,
            calls: call_report,
            implementations: impl_report,
        },
    })
}

fn declarations(
    document: &rustdoc_types::Crate,
    paths: &BTreeMap<rustdoc_types::Id, String>,
) -> Result<(Vec<SurveyDeclaration>, Vec<SurveyDiscoveryFailure>), AnalysisError> {
    let mut declarations = Vec::new();
    let mut failures = Vec::new();
    for (id, public_path) in paths {
        let Some(item) = document.index.get(id) else {
            failures.push(SurveyDiscoveryFailure {
                public_path: public_path.clone(),
                reason: "public item is absent from the Rustdoc index".to_owned(),
            });
            continue;
        };
        let Some(canonical) = document.paths.get(id) else {
            failures.push(SurveyDiscoveryFailure {
                public_path: public_path.clone(),
                reason: "public item has no canonical Rustdoc path".to_owned(),
            });
            continue;
        };
        let signature = serde_json::to_value(&item.inner).map_err(|error| AnalysisError {
            message: format!("cannot serialize native declaration `{public_path}`: {error}"),
        })?;
        let kind = signature
            .as_object()
            .and_then(|object| object.keys().next())
            .cloned()
            .unwrap_or_else(|| "unknown".to_owned());
        declarations.push(SurveyDeclaration {
            public_path: public_path.clone(),
            canonical_path: canonical.path.clone(),
            kind,
            signature,
        });
    }
    declarations.sort_by(|left, right| left.public_path.cmp(&right.public_path));
    failures.sort_by(|left, right| left.public_path.cmp(&right.public_path));
    Ok((declarations, failures))
}

fn cargo_metadata(manifest: &Path, target: &str) -> Result<Metadata, AnalysisError> {
    let mut command = Command::new("cargo");
    configure_projection_cargo_command(&mut command);
    let output = command
        .args([
            "metadata",
            "--format-version",
            "1",
            "--offline",
            "--frozen",
            "--filter-platform",
            target,
            "--manifest-path",
        ])
        .arg(manifest)
        .output()
        .map_err(io_error("run Cargo metadata for survey"))?;
    if !output.status.success() {
        return Err(AnalysisError {
            message: format!(
                "Cargo metadata survey failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        });
    }
    serde_json::from_slice(&output.stdout).map_err(|error| AnalysisError {
        message: format!("invalid Cargo metadata survey output: {error}"),
    })
}

fn selected_closure(metadata: &Metadata, root: &str) -> BTreeSet<String> {
    let nodes = metadata
        .resolve
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let mut selected = BTreeSet::new();
    let mut pending = vec![root.to_owned()];
    while let Some(id) = pending.pop() {
        if !selected.insert(id.clone()) {
            continue;
        }
        if let Some(node) = nodes.get(id.as_str()) {
            pending.extend(node.dependencies.iter().cloned());
        }
    }
    selected
}

fn package_records(
    metadata: &Metadata,
    closure: &BTreeSet<String>,
    workspace: &Path,
    lock: &[LockPackage],
) -> Result<Vec<SurveyPackage>, AnalysisError> {
    let packages = metadata
        .packages
        .iter()
        .map(|package| (package.id.as_str(), package))
        .collect::<BTreeMap<_, _>>();
    let nodes = metadata
        .resolve
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let mut records = Vec::new();
    for id in closure {
        let Some(package) = packages.get(id.as_str()) else {
            continue;
        };
        let node = nodes.get(id.as_str());
        let mut dependencies = node
            .into_iter()
            .flat_map(|node| &node.dependencies)
            .filter_map(|dependency| packages.get(dependency.as_str()))
            .map(|dependency| package_identity(dependency))
            .collect::<Vec<_>>();
        dependencies.sort();
        dependencies.dedup();
        let mut features = node.map_or_else(Vec::new, |node| node.features.clone());
        features.sort();
        features.dedup();
        let locked = lock.iter().find(|locked| {
            locked.name == package.name
                && locked.version == package.version
                && locked.source == package.source
        });
        records.push(SurveyPackage {
            identity: package_identity(package),
            name: package.name.clone(),
            version: package.version.clone(),
            source: normalized_source(package),
            checksum: locked.and_then(|locked| locked.checksum.clone()),
            manifest: normalized_manifest(package, workspace),
            content_fingerprint: path_fingerprint(package)?,
            features,
            dependencies,
        });
    }
    records.sort_by(|left, right| left.identity.cmp(&right.identity));
    Ok(records)
}

fn package_identity(package: &MetadataPackage) -> String {
    format!("{}@{}", package.name, package.version)
}

fn normalized_source(package: &MetadataPackage) -> String {
    package.source.clone().unwrap_or_else(|| "path".to_owned())
}

fn normalized_manifest(package: &MetadataPackage, workspace: &Path) -> String {
    if let Ok(relative) = package.manifest_path.strip_prefix(workspace) {
        return format!("workspace/{}", relative.display());
    }
    if package.source.is_some() {
        return format!("registry/{}/Cargo.toml", package_identity(package));
    }
    format!("path/{}/Cargo.toml", package_identity(package))
}

fn path_fingerprint(package: &MetadataPackage) -> Result<Option<String>, AnalysisError> {
    if package.source.is_some() {
        return Ok(None);
    }
    let bytes = fs::read(&package.manifest_path).map_err(io_error("read path package manifest"))?;
    Ok(Some(format!("sha256:{:x}", Sha256::digest(bytes))))
}

fn cargo_lock(workspace: &Path) -> Result<Vec<LockPackage>, AnalysisError> {
    let contents = fs::read_to_string(workspace.join("Cargo.lock"))
        .map_err(io_error("read survey Cargo.lock"))?;
    let lock: CargoLock = toml::from_str(&contents).map_err(|error| AnalysisError {
        message: format!("invalid survey Cargo.lock: {error}"),
    })?;
    Ok(lock.package)
}

fn select_package<'a>(
    metadata: &'a Metadata,
    requested: Option<&str>,
) -> Result<&'a MetadataPackage, AnalysisError> {
    let Some(requested) = requested else {
        let Some(id) = metadata.resolve.root.as_ref() else {
            return Err(AnalysisError {
                message: "survey has no root package; pass --package name@version".to_owned(),
            });
        };
        return metadata
            .packages
            .iter()
            .find(|package| &package.id == id)
            .ok_or_else(|| AnalysisError {
                message: "Cargo metadata root is absent from its package list".to_owned(),
            });
    };
    let (name, version) = requested
        .split_once('@')
        .map_or((requested, None), |(name, version)| (name, Some(version)));
    let matches = metadata
        .packages
        .iter()
        .filter(|package| {
            package.name == name && version.is_none_or(|version| package.version == version)
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [package] => Ok(package),
        [] => Err(AnalysisError {
            message: format!("survey package `{requested}` was not found in Cargo metadata"),
        }),
        _ => Err(AnalysisError {
            message: format!("survey package `{name}` is ambiguous; pass name@version"),
        }),
    }
}

fn run_rustdoc(
    manifest: &Path,
    package: &MetadataPackage,
    target: &str,
    target_directory: &Path,
) -> Result<(), AnalysisError> {
    let mut command = Command::new("cargo");
    configure_projection_cargo_command(&mut command);
    let spec = package_identity(package);
    let output = command
        .arg(format!("+{RUSTDOC_TOOLCHAIN}"))
        .args([
            "rustdoc",
            "-p",
            &spec,
            "--lib",
            "--offline",
            "--frozen",
            "--target",
            target,
            "--manifest-path",
        ])
        .arg(manifest)
        .arg("--target-dir")
        .arg(target_directory)
        .args(["--", "-Z", "unstable-options", "--output-format", "json"])
        .output()
        .map_err(io_error("run Cargo rustdoc for survey"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(AnalysisError {
            message: format!(
                "Cargo rustdoc survey failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        })
    }
}

fn host_target() -> Result<String, AnalysisError> {
    let version = rustc_verbose_version()?;
    version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .map(ToOwned::to_owned)
        .ok_or_else(|| AnalysisError {
            message: "rustc -vV did not report its host target".to_owned(),
        })
}

fn rustc_version() -> Result<String, AnalysisError> {
    Ok(rustc_verbose_version()?
        .lines()
        .next()
        .unwrap_or("rustc unknown")
        .to_owned())
}

fn rustc_verbose_version() -> Result<String, AnalysisError> {
    let output = Command::new("rustc")
        .arg("-vV")
        .output()
        .map_err(io_error("read Rust toolchain identity"))?;
    if !output.status.success() {
        return Err(AnalysisError {
            message: "rustc -vV failed while identifying the survey toolchain".to_owned(),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn io_error(action: &'static str) -> impl FnOnce(std::io::Error) -> AnalysisError {
    move |error| AnalysisError {
        message: format!("cannot {action}: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(id: &str, name: &str, version: &str) -> MetadataPackage {
        MetadataPackage {
            id: id.to_owned(),
            name: name.to_owned(),
            version: version.to_owned(),
            source: Some("registry+test".to_owned()),
            manifest_path: PathBuf::from("unused"),
            targets: Vec::new(),
        }
    }

    fn metadata(packages: Vec<MetadataPackage>, root: Option<&str>) -> Metadata {
        Metadata {
            packages,
            resolve: MetadataResolve {
                root: root.map(ToOwned::to_owned),
                nodes: Vec::new(),
            },
        }
    }

    #[test]
    fn package_selection_requires_a_version_when_a_name_is_ambiguous() {
        let metadata = metadata(
            vec![
                package("one", "same", "1.0.0"),
                package("two", "same", "2.0.0"),
            ],
            None,
        );
        assert!(
            select_package(&metadata, Some("same"))
                .unwrap_err()
                .message
                .contains("ambiguous")
        );
        assert_eq!(
            select_package(&metadata, Some("same@1.0.0")).unwrap().id,
            "one"
        );
    }

    #[test]
    fn package_selection_distinguishes_missing_and_rootless_graphs() {
        let metadata = metadata(Vec::new(), None);
        assert!(
            select_package(&metadata, Some("missing"))
                .unwrap_err()
                .message
                .contains("not found")
        );
        assert!(
            select_package(&metadata, None)
                .unwrap_err()
                .message
                .contains("no root package")
        );
    }

    #[test]
    fn selected_closure_excludes_unrelated_workspace_packages() {
        let metadata = Metadata {
            packages: Vec::new(),
            resolve: MetadataResolve {
                root: Some("root".to_owned()),
                nodes: vec![
                    MetadataNode {
                        id: "root".to_owned(),
                        dependencies: vec!["used".to_owned()],
                        features: Vec::new(),
                    },
                    MetadataNode {
                        id: "used".to_owned(),
                        dependencies: Vec::new(),
                        features: Vec::new(),
                    },
                    MetadataNode {
                        id: "unrelated".to_owned(),
                        dependencies: Vec::new(),
                        features: Vec::new(),
                    },
                ],
            },
        };
        assert_eq!(
            selected_closure(&metadata, "root"),
            BTreeSet::from(["root".to_owned(), "used".to_owned()])
        );
    }
}
