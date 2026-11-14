//! Cargo package identities for document-local Rustdoc crate references.
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use rustdoc_types::ExternalCrate;
use serde_json::Value;

use super::{ProjectionError, RustdocCrate};

const ARTIFACTS: &str = "target/rustdoc-57/package-artifacts.json";

pub(super) fn read_artifacts(
    workspace: &Path,
) -> Result<BTreeMap<PathBuf, String>, ProjectionError> {
    match std::fs::read(workspace.join(ARTIFACTS)) {
        Ok(bytes) => Ok(serde_json::from_slice(&bytes).unwrap_or_default()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(BTreeMap::new()),
        Err(error) => Err(ProjectionError {
            message: format!("cannot read Rustdoc package provenance: {error}"),
        }),
    }
}

pub(super) fn record_artifacts(workspace: &Path, stdout: &[u8]) -> Result<(), ProjectionError> {
    let mut artifacts = read_artifacts(workspace)?;
    for line in stdout.split(|byte| *byte == b'\n') {
        let Ok(message) = serde_json::from_slice::<Value>(line) else {
            continue;
        };
        if message["reason"] != "compiler-artifact" {
            continue;
        }
        let (Some(package), Some(files)) = (
            message["package_id"].as_str(),
            message["filenames"].as_array(),
        ) else {
            continue;
        };
        for file in files.iter().filter_map(Value::as_str) {
            artifacts.insert(PathBuf::from(file), package.to_owned());
        }
    }
    let bytes = serde_json::to_vec(&artifacts).map_err(|error| ProjectionError {
        message: format!("cannot encode Rustdoc package provenance: {error}"),
    })?;
    super::write_cache_atomically(&workspace.join(ARTIFACTS), &[&bytes])
}

pub(super) struct Packages<'a> {
    metadata: &'a Value,
    artifacts: BTreeMap<PathBuf, String>,
    reachable: RefCell<BTreeMap<&'a str, BTreeSet<&'a str>>>,
}

impl<'a> Packages<'a> {
    pub(super) fn new(metadata: &'a Value, artifacts: BTreeMap<PathBuf, String>) -> Self {
        Self {
            metadata,
            artifacts,
            reachable: RefCell::new(BTreeMap::new()),
        }
    }

    pub(super) fn package(&self, identity: &str) -> Option<&'a Value> {
        self.metadata["packages"]
            .as_array()?
            .iter()
            .find(|package| package["id"].as_str() == Some(identity))
    }

    pub(super) fn direct(&self, name: &str) -> Option<&'a str> {
        let root = self.metadata["resolve"]["root"].as_str()?;
        let node = self.metadata["resolve"]["nodes"]
            .as_array()?
            .iter()
            .find(|node| node["id"].as_str() == Some(root))?;
        let cargo_name = name.replace('-', "_");
        node["deps"]
            .as_array()?
            .iter()
            .find(|dependency| dependency["name"].as_str() == Some(cargo_name.as_str()))?["pkg"]
            .as_str()
    }

    pub(super) fn library_name(&self, identity: &str) -> Option<&'a str> {
        self.package(identity)?["targets"]
            .as_array()?
            .iter()
            .find(|target| library_target(target))?["name"]
            .as_str()
    }

    pub(super) fn document(&self, document: &RustdocCrate) -> Option<&'a str> {
        let name = super::output_aliases::document_name(document)?;
        if matches!(name, "std" | "core" | "alloc") {
            return Some(match name {
                "std" => "std",
                "core" => "core",
                _ => "alloc",
            });
        }
        let version = document.crate_version.as_deref()?;
        let mut matches = self.metadata["packages"]
            .as_array()?
            .iter()
            .filter(|package| {
                package["version"].as_str() == Some(version) && library_named(package, name)
            });
        let package = matches.next()?;
        if matches.next().is_some() {
            return None;
        }
        package["id"].as_str()
    }

    pub(super) fn external(&self, referring: &str, external: &ExternalCrate) -> Option<&'a str> {
        match external.name.as_str() {
            "std" => return Some("std"),
            "core" => return Some("core"),
            "alloc" => return Some("alloc"),
            _ => {}
        }
        let referring = self.package(referring)?["id"].as_str()?;
        let mut closures = self.reachable.borrow_mut();
        let reachable = closures.entry(referring).or_insert_with(|| {
            let mut reachable = BTreeSet::new();
            let mut pending = vec![referring];
            while let Some(identity) = pending.pop() {
                if !reachable.insert(identity) {
                    continue;
                }
                let Some(nodes) = self.metadata["resolve"]["nodes"].as_array() else {
                    continue;
                };
                let Some(node) = nodes
                    .iter()
                    .find(|node| node["id"].as_str() == Some(identity))
                else {
                    continue;
                };
                if let Some(deps) = node["deps"].as_array() {
                    pending.extend(
                        deps.iter()
                            .filter_map(|dependency| dependency["pkg"].as_str()),
                    );
                }
            }
            reachable
        });
        // A renamed extern can have the same library name as another version.
        // Cargo's emitted artifact path disambiguates that exact dependency edge.
        if let Some(identity) = self.artifacts.get(&external.path)
            && reachable.contains(identity.as_str())
        {
            return self.package(identity)?["id"].as_str();
        }
        let mut candidates = reachable.iter().copied().filter(|identity| {
            self.package(identity)
                .is_some_and(|package| library_named(package, &external.name))
        });
        let identity = candidates.next()?;
        if candidates.next().is_some() {
            return None;
        }
        Some(identity)
    }

    pub(super) fn reference(
        &self,
        document: &RustdocCrate,
        package: &str,
        crate_id: u32,
    ) -> Option<&'a str> {
        if crate_id == 0 {
            match package {
                "std" => Some("std"),
                "core" => Some("core"),
                "alloc" => Some("alloc"),
                _ => self.package(package)?["id"].as_str(),
            }
        } else {
            self.external(package, document.external_crates.get(&crate_id)?)
        }
    }

    pub(super) fn root_external(&self, document: &RustdocCrate, package: &str) -> ExternalCrate {
        ExternalCrate {
            name: super::output_aliases::document_name(document)
                .unwrap_or_default()
                .to_owned(),
            html_root_url: None,
            path: self
                .artifacts
                .iter()
                .find(|(_, identity)| identity.as_str() == package)
                .map(|(path, _)| path.clone())
                .unwrap_or_default(),
        }
    }
}

fn library_named(package: &Value, name: &str) -> bool {
    package["targets"].as_array().is_some_and(|targets| {
        targets
            .iter()
            .any(|target| target["name"].as_str() == Some(name) && library_target(target))
    })
}

fn library_target(target: &Value) -> bool {
    target["kind"].as_array().is_some_and(|kinds| {
        kinds.iter().any(|kind| {
            kind.as_str().is_some_and(|kind| {
                matches!(
                    kind,
                    "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro"
                )
            })
        })
    })
}
