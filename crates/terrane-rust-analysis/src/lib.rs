//! Shared native Rust graph indexing and exact compiler-probe support.
//!
//! Terrane language admission and lowering remain in `terrane-compiler`.
mod cargo_toolchain;
mod oracle;

use std::collections::{BTreeMap, BTreeSet, HashMap};

use rustdoc_types::{Crate as RustdocCrate, Id, Item, ItemEnum, Visibility};

pub use cargo_toolchain::{configure_cargo_command, configure_projection_cargo_command};
pub use oracle::*;

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
pub enum Containment {
    Enforced,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisError {
    pub message: String,
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AnalysisError {}

/// Parses and version-checks one Rustdoc JSON artifact.
///
/// # Errors
///
/// Returns an error when the bytes are not valid Rustdoc JSON for the pinned format.
pub fn parse_rustdoc(
    package: &str,
    bytes: &[u8],
    toolchain: &str,
) -> Result<RustdocCrate, AnalysisError> {
    let document: RustdocCrate = serde_json::from_slice(bytes).map_err(|error| AnalysisError {
        message: format!(
            "rustdoc JSON schema mismatch for `{package}`: {error}; expected rustdoc format {} from `{toolchain}`",
            rustdoc_types::FORMAT_VERSION
        ),
    })?;
    if document.format_version != rustdoc_types::FORMAT_VERSION {
        return Err(AnalysisError {
            message: format!(
                "rustdoc JSON schema mismatch for `{package}`: format {} is unsupported; expected format {} from `{toolchain}`",
                document.format_version,
                rustdoc_types::FORMAT_VERSION
            ),
        });
    }
    Ok(document)
}

pub fn prefer_public_path(paths: &mut BTreeMap<Id, String>, id: Id, candidate: String) {
    paths
        .entry(id)
        .and_modify(|current| {
            let candidate_prelude = candidate.split("::").any(|part| part == "prelude");
            let current_prelude = current.split("::").any(|part| part == "prelude");
            let candidate_depth = candidate.matches("::").count();
            let current_depth = current.matches("::").count();
            if (current_prelude && !candidate_prelude)
                || (candidate_prelude == current_prelude
                    && (candidate_depth < current_depth
                        || (candidate_depth == current_depth && candidate < *current)))
            {
                current.clone_from(&candidate);
            }
        })
        .or_insert(candidate);
}

#[must_use]
pub fn public_paths(document: &RustdocCrate) -> BTreeMap<Id, String> {
    let mut paths = BTreeMap::new();
    let Some(root) = document
        .paths
        .get(&document.root)
        .map(|summary| &summary.path)
    else {
        return paths;
    };
    visit(
        &document.index,
        document.root,
        root,
        &mut paths,
        &mut BTreeSet::new(),
    );
    paths
}

fn visit(
    index: &HashMap<Id, Item>,
    id: Id,
    prefix: &[String],
    paths: &mut BTreeMap<Id, String>,
    visiting: &mut BTreeSet<Id>,
) {
    if !visiting.insert(id) {
        return;
    }
    let Some(Item {
        inner: ItemEnum::Module(module),
        ..
    }) = index.get(&id)
    else {
        visiting.remove(&id);
        return;
    };
    for child in &module.items {
        let Some(item) = index
            .get(child)
            .filter(|item| item.visibility == Visibility::Public)
        else {
            continue;
        };
        if let ItemEnum::Use(import) = &item.inner {
            let Some(target) = import.id else {
                continue;
            };
            if import.is_glob {
                visit(index, target, prefix, paths, visiting);
            } else {
                let mut candidate = prefix.to_vec();
                candidate.push(import.name.clone());
                prefer_public_path(paths, target, candidate.join("::"));
            }
            continue;
        }
        let Some(name) = item.name.as_ref() else {
            continue;
        };
        let mut candidate = prefix.to_vec();
        candidate.push(name.clone());
        prefer_public_path(paths, *child, candidate.join("::"));
        if matches!(item.inner, ItemEnum::Module(_)) {
            visit(index, *child, &candidate, paths, visiting);
        }
    }
    visiting.remove(&id);
}
