use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::oracle::cargo_output;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    AnalysisError, BUILD_TOOLCHAIN, BoundQuestion, CallProbeEvidence, CallQuestion, Containment,
    ImplProbeEvidence, ImplQuestion, ProbeEvidence, ProjectionOracle, RUSTDOC_TOOLCHAIN,
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
    pub bounds: StableProbeReport<ProbeEvidence>,
    pub calls: StableProbeReport<CallProbeEvidence>,
    pub implementations: StableProbeReport<ImplProbeEvidence>,
}

/// Semantic probe evidence excludes run-to-run timing telemetry.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StableProbeReport<E> {
    pub evidence: Vec<E>,
    pub compiled_probe_count: usize,
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

fn is_library_target(target: &MetadataTarget) -> bool {
    const LIBRARY_KINDS: [&str; 6] = ["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro"];
    target
        .kind
        .iter()
        .any(|kind| LIBRARY_KINDS.contains(&kind.as_str()))
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
    survey_package_with_policy(manifest, package, target, probes, Containment::Unavailable)
}

/// Resolves and surveys one package using the requested native-process containment tier.
///
/// # Errors
///
/// Returns an error when graph discovery, Rustdoc generation, or probe execution cannot complete.
pub fn survey_package_with_policy(
    manifest: &Path,
    package: Option<&str>,
    target: &str,
    probes: &SurveyProbeRequest,
    containment: Containment,
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
        .find(|candidate| is_library_target(candidate))
        .ok_or_else(|| AnalysisError {
            message: format!("selected package `{}` has no library target", selected.name),
        })?
        .name
        .clone();
    let closure = selected_closure(&metadata, &selected.id);
    let lock = cargo_lock(workspace)?;
    let packages = package_records(&metadata, &closure, workspace, &lock)?;
    let target_directory = workspace.join(".trn/dependencies/rust-survey/target");
    run_rustdoc(&manifest, selected, target, &target_directory, containment)?;
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
    let oracle = ProjectionOracle::new(workspace, &oracle_identity, containment);
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
        build_toolchain: BUILD_TOOLCHAIN.to_owned(),
        rustdoc_format: document.format_version,
        rustdoc_toolchain: RUSTDOC_TOOLCHAIN.to_owned(),
        containment,
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
            bounds: StableProbeReport {
                evidence: bound_report.evidence,
                compiled_probe_count: bound_report.compiled_probe_count,
            },
            calls: StableProbeReport {
                evidence: call_report.evidence,
                compiled_probe_count: call_report.compiled_probe_count,
            },
            implementations: StableProbeReport {
                evidence: impl_report.evidence,
                compiled_probe_count: impl_report.compiled_probe_count,
            },
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
        let Some(canonical) = document.paths.get(id) else {
            failures.push(SurveyDiscoveryFailure {
                public_path: public_path.clone(),
                reason: "public item has no canonical Rustdoc path".to_owned(),
            });
            continue;
        };
        let Some(item) = document.index.get(id) else {
            let reason = missing_declaration_reason(
                document
                    .external_crates
                    .get(&canonical.crate_id)
                    .map(|external| external.name.as_str()),
            );
            failures.push(SurveyDiscoveryFailure {
                public_path: public_path.clone(),
                reason,
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

fn missing_declaration_reason(external_crate: Option<&str>) -> String {
    external_crate.map_or_else(
        || "public item is absent from the Rustdoc index".to_owned(),
        |external| {
            format!(
                "declaration belongs to external crate `{external}`; survey that closure package for its signature"
            )
        },
    )
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
    let root = package
        .manifest_path
        .parent()
        .ok_or_else(|| AnalysisError {
            message: format!(
                "path package manifest `{}` has no package root",
                package.manifest_path.display()
            ),
        })?;
    let mut files = Vec::new();
    collect_package_files(root, root, &mut files)?;
    files.sort();
    let mut hash = Sha256::new();
    for path in files {
        let relative = path.strip_prefix(root).map_err(|error| AnalysisError {
            message: format!("cannot normalize path package input: {error}"),
        })?;
        let metadata =
            fs::symlink_metadata(&path).map_err(io_error("read path package input metadata"))?;
        hash.update(relative.to_string_lossy().as_bytes());
        hash.update([0]);
        if metadata.file_type().is_symlink() {
            hash.update(b"symlink");
            hash.update(
                fs::read_link(&path)
                    .map_err(io_error("read path package symlink"))?
                    .to_string_lossy()
                    .as_bytes(),
            );
        } else {
            hash.update(b"file");
            hash.update(fs::read(&path).map_err(io_error("read path package input"))?);
        }
        hash.update([0]);
    }
    Ok(Some(format!("sha256:{:x}", hash.finalize())))
}

fn collect_package_files(
    root: &Path,
    directory: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), AnalysisError> {
    let mut entries = fs::read_dir(directory)
        .map_err(io_error("read path package directory"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(io_error("read path package entry"))?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let relative = path.strip_prefix(root).map_err(|error| AnalysisError {
            message: format!("cannot normalize path package entry: {error}"),
        })?;
        let first = relative
            .components()
            .next()
            .and_then(|component| match component {
                std::path::Component::Normal(name) => name.to_str(),
                _ => None,
            });
        if matches!(first, Some(".git" | ".trn" | "target")) {
            continue;
        }
        let file_type = entry
            .file_type()
            .map_err(io_error("read path package entry type"))?;
        if file_type.is_dir() {
            collect_package_files(root, &path, files)?;
        } else if file_type.is_file() || file_type.is_symlink() {
            files.push(path);
        }
    }
    Ok(())
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
    containment: Containment,
) -> Result<(), AnalysisError> {
    let workspace = manifest.parent().ok_or_else(|| AnalysisError {
        message: format!(
            "survey manifest `{}` has no parent directory",
            manifest.display()
        ),
    })?;
    let spec = package_identity(package);
    let manifest = manifest.to_string_lossy();
    let target_directory = target_directory.to_string_lossy();
    let arguments = [
        "rustdoc",
        "-p",
        spec.as_str(),
        "--lib",
        "--offline",
        "--frozen",
        "--target",
        target,
        "--manifest-path",
        manifest.as_ref(),
        "--target-dir",
        target_directory.as_ref(),
        "--",
        "-Z",
        "unstable-options",
        "--output-format",
        "json",
    ];
    let output = cargo_output(workspace, &arguments, containment)?;
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

fn rustc_verbose_version() -> Result<String, AnalysisError> {
    let output = Command::new("rustc")
        .arg(format!("+{BUILD_TOOLCHAIN}"))
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

    fn temporary_directory(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "terrane-survey-{name}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&path).unwrap();
        path
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

    #[test]
    fn cargo_library_target_kinds_include_explicit_crate_types() {
        for kind in ["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro"] {
            assert!(is_library_target(&MetadataTarget {
                name: "sample".to_owned(),
                kind: vec![kind.to_owned()],
            }));
        }
        assert!(!is_library_target(&MetadataTarget {
            name: "sample".to_owned(),
            kind: vec!["bin".to_owned()],
        }));
    }

    #[test]
    fn external_reexports_name_their_declaring_crate() {
        assert_eq!(
            missing_declaration_reason(Some("godot_core")),
            "declaration belongs to external crate `godot_core`; survey that closure package for its signature"
        );
        assert_eq!(
            missing_declaration_reason(None),
            "public item is absent from the Rustdoc index"
        );
    }

    #[test]
    fn normalized_manifests_cover_workspace_registry_and_external_paths() {
        let workspace = Path::new("/work");
        let mut sample = package("sample", "sample", "1.0.0");
        sample.manifest_path = PathBuf::from("/work/member/Cargo.toml");
        assert_eq!(
            normalized_manifest(&sample, workspace),
            "workspace/member/Cargo.toml"
        );
        sample.manifest_path = PathBuf::from("/cargo/sample/Cargo.toml");
        assert_eq!(
            normalized_manifest(&sample, workspace),
            "registry/sample@1.0.0/Cargo.toml"
        );
        sample.source = None;
        assert_eq!(
            normalized_manifest(&sample, workspace),
            "path/sample@1.0.0/Cargo.toml"
        );
    }

    #[test]
    fn path_fingerprint_tracks_sources_but_ignores_build_output() {
        let root = temporary_directory("fingerprint");
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname='sample'\nversion='1.0.0'\n",
        )
        .unwrap();
        fs::write(root.join("src/lib.rs"), "pub fn value() -> u8 { 1 }\n").unwrap();
        let mut sample = package("sample", "sample", "1.0.0");
        sample.source = None;
        sample.manifest_path = root.join("Cargo.toml");
        let first = path_fingerprint(&sample).unwrap();
        fs::write(root.join("src/lib.rs"), "pub fn value() -> u8 { 2 }\n").unwrap();
        let second = path_fingerprint(&sample).unwrap();
        assert_ne!(first, second);
        fs::create_dir_all(root.join("target")).unwrap();
        fs::write(root.join("target/noise"), "ignored").unwrap();
        assert_eq!(second, path_fingerprint(&sample).unwrap());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn package_records_retain_checksum_features_and_dependency_identity() {
        let mut root = package("root-id", "root", "1.0.0");
        root.targets.push(MetadataTarget {
            name: "root".to_owned(),
            kind: vec!["lib".to_owned()],
        });
        let dependency = package("dep-id", "dep", "2.0.0");
        let metadata = Metadata {
            packages: vec![root, dependency],
            resolve: MetadataResolve {
                root: Some("root-id".to_owned()),
                nodes: vec![
                    MetadataNode {
                        id: "root-id".to_owned(),
                        dependencies: vec!["dep-id".to_owned()],
                        features: vec!["z".to_owned(), "a".to_owned(), "a".to_owned()],
                    },
                    MetadataNode {
                        id: "dep-id".to_owned(),
                        dependencies: Vec::new(),
                        features: Vec::new(),
                    },
                ],
            },
        };
        let lock = vec![LockPackage {
            name: "root".to_owned(),
            version: "1.0.0".to_owned(),
            source: Some("registry+test".to_owned()),
            checksum: Some("checksum".to_owned()),
        }];
        let records = package_records(
            &metadata,
            &BTreeSet::from(["root-id".to_owned(), "dep-id".to_owned()]),
            Path::new("/workspace"),
            &lock,
        )
        .unwrap();
        let root = records.iter().find(|record| record.name == "root").unwrap();
        assert_eq!(root.checksum.as_deref(), Some("checksum"));
        assert_eq!(root.features, ["a", "z"]);
        assert_eq!(root.dependencies, ["dep@2.0.0"]);
    }

    #[test]
    fn declarations_separate_local_signatures_from_external_owners() {
        use std::collections::HashMap;

        use rustdoc_types::{
            Crate, ExternalCrate, Id, Item, ItemEnum, ItemKind, ItemSummary, Primitive, Target,
            Visibility,
        };

        let local = Id(1);
        let external = Id(2);
        let document = Crate {
            root: Id(0),
            crate_version: Some("1.0.0".to_owned()),
            includes_private: false,
            index: HashMap::from([(
                local,
                Item {
                    id: local,
                    crate_id: 0,
                    name: Some("Local".to_owned()),
                    span: None,
                    visibility: Visibility::Public,
                    docs: None,
                    links: HashMap::new(),
                    attrs: Vec::new(),
                    deprecation: None,
                    inner: ItemEnum::Primitive(Primitive {
                        name: "local".to_owned(),
                        impls: Vec::new(),
                    }),
                },
            )]),
            paths: HashMap::from([
                (
                    local,
                    ItemSummary {
                        crate_id: 0,
                        path: vec!["sample".to_owned(), "Local".to_owned()],
                        kind: ItemKind::Primitive,
                    },
                ),
                (
                    external,
                    ItemSummary {
                        crate_id: 1,
                        path: vec!["owner".to_owned(), "External".to_owned()],
                        kind: ItemKind::Primitive,
                    },
                ),
            ]),
            external_crates: HashMap::from([(
                1,
                ExternalCrate {
                    name: "owner".to_owned(),
                    html_root_url: None,
                    path: PathBuf::from("/registry/owner.rlib"),
                },
            )]),
            target: Target {
                triple: "x86_64-unknown-linux-gnu".to_owned(),
                target_features: Vec::new(),
            },
            format_version: 57,
        };
        let paths = BTreeMap::from([
            (local, "sample::Local".to_owned()),
            (external, "sample::External".to_owned()),
        ]);
        let (declarations, failures) = declarations(&document, &paths).unwrap();
        assert_eq!(declarations.len(), 1);
        assert_eq!(declarations[0].public_path, "sample::Local");
        assert_eq!(failures.len(), 1);
        assert!(failures[0].reason.contains("external crate `owner`"));
    }
}
