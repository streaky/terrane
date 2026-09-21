//! Shared native Rust graph indexing and exact compiler-probe support.
//!
//! Terrane language admission and lowering remain in `terrane-compiler`.
mod cargo_toolchain;
mod oracle;
mod survey;

use std::collections::{BTreeMap, BTreeSet, HashMap};

use rustdoc_types::{Crate as RustdocCrate, Id, Item, ItemEnum, Visibility};

pub use cargo_toolchain::{configure_cargo_command, configure_projection_cargo_command};
pub use oracle::*;
pub use survey::{
    StableProbeReport, SurveyDeclaration, SurveyDiscoveryFailure, SurveyPackage,
    SurveyProbeExecution, SurveyProbeRequest, SurveyReport, survey_package,
    survey_package_for_target, survey_package_for_target_with_probes, survey_package_with_policy,
};

/// Stable Rust release used to build generated Terrane applications.
pub const BUILD_TOOLCHAIN: &str = env!("CARGO_PKG_RUST_VERSION");

/// Nightly toolchain pinned to the Rustdoc JSON format and native probe cache identity.
pub const RUSTDOC_TOOLCHAIN: &str = "nightly-2026-04-29";

/// Rustdoc arguments shared by compiler projection and native ecosystem surveys.
pub const RUSTDOC_JSON_ARGS: &[&str] = &["-Z", "unstable-options", "--output-format", "json"];

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
    public_path_index(document).0
}

/// IDs whose public binding resolves through a hidden Rustdoc definition.
#[must_use]
pub fn hidden_public_definitions(document: &RustdocCrate) -> BTreeSet<Id> {
    public_path_index(document).1
}

fn public_path_index(document: &RustdocCrate) -> (BTreeMap<Id, String>, BTreeSet<Id>) {
    let mut paths = BTreeMap::new();
    let mut hidden_definitions = BTreeSet::new();
    let Some(root) = document
        .paths
        .get(&document.root)
        .map(|summary| &summary.path)
    else {
        return (paths, hidden_definitions);
    };
    visit(
        &document.index,
        document.root,
        root,
        false,
        &mut paths,
        &mut hidden_definitions,
        &mut BTreeSet::new(),
    );
    (paths, hidden_definitions)
}

fn visit(
    index: &HashMap<Id, Item>,
    id: Id,
    prefix: &[String],
    hidden_ancestor: bool,
    paths: &mut BTreeMap<Id, String>,
    hidden_definitions: &mut BTreeSet<Id>,
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
            .filter(|item| item.visibility == Visibility::Public && !is_hidden_surface(item))
        else {
            continue;
        };
        if let ItemEnum::Use(import) = &item.inner {
            let Some(target) = import.id else {
                continue;
            };
            let target_hidden =
                hidden_ancestor || index.get(&target).is_some_and(is_hidden_surface);
            if import.is_glob {
                visit(
                    index,
                    target,
                    prefix,
                    target_hidden,
                    paths,
                    hidden_definitions,
                    visiting,
                );
            } else {
                let mut candidate = prefix.to_vec();
                candidate.push(import.name.clone());
                prefer_public_path(paths, target, candidate.join("::"));
                if target_hidden {
                    hidden_definitions.insert(target);
                }
            }
            continue;
        }
        let Some(name) = item.name.as_ref() else {
            continue;
        };
        let mut candidate = prefix.to_vec();
        candidate.push(name.clone());
        prefer_public_path(paths, *child, candidate.join("::"));
        if hidden_ancestor {
            hidden_definitions.insert(*child);
        }
        if matches!(item.inner, ItemEnum::Module(_)) {
            visit(
                index,
                *child,
                &candidate,
                hidden_ancestor,
                paths,
                hidden_definitions,
                visiting,
            );
        }
    }
    visiting.remove(&id);
}
pub(crate) fn is_hidden_surface(item: &Item) -> bool {
    item.attrs.iter().any(|attribute| {
        matches!(
            attribute,
            rustdoc_types::Attribute::Other(text) if text == "#[doc(hidden)]"
        )
    }) || matches!(&item.inner, ItemEnum::Module(module) if module.is_stripped)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(
        id: Id,
        name: Option<&str>,
        attrs: Vec<rustdoc_types::Attribute>,
        inner: ItemEnum,
    ) -> Item {
        Item {
            id,
            crate_id: 0,
            name: name.map(str::to_owned),
            span: None,
            visibility: Visibility::Public,
            docs: None,
            links: HashMap::new(),
            attrs,
            deprecation: None,
            inner,
        }
    }

    fn module_item(id: Id, name: &str, items: Vec<Id>, is_crate: bool, is_stripped: bool) -> Item {
        item(
            id,
            Some(name),
            Vec::new(),
            ItemEnum::Module(rustdoc_types::Module {
                is_crate,
                items,
                is_stripped,
            }),
        )
    }

    fn use_item(id: Id, source: &str, name: &str, target: Id, is_glob: bool, hidden: bool) -> Item {
        let attrs = hidden
            .then(|| rustdoc_types::Attribute::Other("#[doc(hidden)]".to_owned()))
            .into_iter()
            .collect();
        item(
            id,
            None,
            attrs,
            ItemEnum::Use(rustdoc_types::Use {
                source: source.to_owned(),
                name: name.to_owned(),
                id: Some(target),
                is_glob,
            }),
        )
    }

    fn hidden_surface_document() -> (RustdocCrate, Id, Id) {
        use rustdoc_types::{ItemKind, ItemSummary, Primitive, Target};

        let root = Id(0);
        let hidden_use = Id(1);
        let visible_use = Id(2);
        let stripped_module = Id(3);
        let visible_glob = Id(4);
        let hidden_target = Id(10);
        let visible_target = Id(11);
        let document = RustdocCrate {
            root,
            crate_version: Some("1.0.0".to_owned()),
            includes_private: false,
            index: HashMap::from([
                (
                    root,
                    module_item(
                        root,
                        "sample",
                        vec![hidden_use, visible_use, stripped_module, visible_glob],
                        true,
                        false,
                    ),
                ),
                (
                    hidden_use,
                    use_item(
                        hidden_use,
                        "private::Hidden",
                        "Hidden",
                        hidden_target,
                        false,
                        true,
                    ),
                ),
                (
                    visible_use,
                    use_item(
                        visible_use,
                        "private::Visible",
                        "Visible",
                        visible_target,
                        false,
                        false,
                    ),
                ),
                (
                    stripped_module,
                    module_item(stripped_module, "private", vec![hidden_target], false, true),
                ),
                (
                    visible_glob,
                    use_item(
                        visible_glob,
                        "private",
                        "private",
                        stripped_module,
                        true,
                        false,
                    ),
                ),
                (
                    hidden_target,
                    item(
                        hidden_target,
                        Some("Recovered"),
                        Vec::new(),
                        ItemEnum::Primitive(Primitive {
                            name: "recovered".to_owned(),
                            impls: Vec::new(),
                        }),
                    ),
                ),
            ]),
            paths: HashMap::from([(
                root,
                ItemSummary {
                    crate_id: 0,
                    path: vec!["sample".to_owned()],
                    kind: ItemKind::Module,
                },
            )]),
            external_crates: HashMap::new(),
            target: Target {
                triple: "x86_64-unknown-linux-gnu".to_owned(),
                target_features: Vec::new(),
            },
            format_version: 57,
        };
        (document, hidden_target, visible_target)
    }

    #[test]
    fn public_paths_skip_hidden_bindings_but_keep_visible_reexports() {
        let (document, hidden_target, visible_target) = hidden_surface_document();
        assert_eq!(
            public_paths(&document),
            BTreeMap::from([
                (hidden_target, "sample::Recovered".to_owned()),
                (visible_target, "sample::Visible".to_owned()),
            ])
        );
        assert_eq!(
            hidden_public_definitions(&document),
            BTreeSet::from([hidden_target])
        );
    }

    #[test]
    fn crate_manifest_does_not_depend_on_compiler_or_cli() {
        let manifest = include_str!("../Cargo.toml");
        assert!(!manifest.contains("terrane-compiler"));
        assert!(!manifest.contains("terrane-cli"));
    }
}
