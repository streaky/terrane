//! Shared native Rust graph indexing and exact compiler-probe support.
//!
//! Terrane language admission and lowering remain in `terrane-compiler`.
mod cargo_toolchain;
mod oracle;
mod survey;

use std::collections::{BTreeMap, BTreeSet};

use rustdoc_types::{Crate as RustdocCrate, Id, Item, ItemEnum, ItemKind, Visibility};

pub use cargo_toolchain::{configure_cargo_command, configure_projection_cargo_command};
pub use oracle::*;
pub use survey::{
    StableProbeReport, SurveyDeclaration, SurveyDeclarationMember, SurveyDiscoveryFailure,
    SurveyPackage, SurveyProbeExecution, SurveyProbeRequest, SurveyPublicApiItem, SurveyReport,
    survey_complete_package_for_target, survey_core_declarations, survey_package,
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

#[derive(Default)]
struct PublicSurface {
    paths: BTreeMap<Id, String>,
    external_reexports: Vec<ExternalReexport>,
}

#[must_use]
pub fn public_paths(document: &RustdocCrate) -> BTreeMap<Id, String> {
    public_surface(document).paths
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ExternalReexport {
    pub crate_name: String,
    pub canonical_path: String,
    pub public_path: String,
    pub expands_descendants: bool,
}

#[must_use]
pub fn external_reexports(document: &RustdocCrate) -> Vec<ExternalReexport> {
    public_surface(document).external_reexports
}

fn public_surface(document: &RustdocCrate) -> PublicSurface {
    let Some(root) = document
        .paths
        .get(&document.root)
        .map(|summary| &summary.path)
    else {
        return PublicSurface::default();
    };
    let mut surface = PublicSurface::default();
    visit_public_surface(
        document,
        document.root,
        root,
        &mut surface,
        &mut BTreeSet::new(),
    );
    surface.external_reexports.sort();
    surface.external_reexports.dedup();
    surface
}

fn visit_public_surface(
    document: &RustdocCrate,
    id: Id,
    prefix: &[String],
    surface: &mut PublicSurface,
    visiting: &mut BTreeSet<Id>,
) {
    if !visiting.insert(id) {
        return;
    }
    let Some(Item {
        inner: ItemEnum::Module(module),
        ..
    }) = document.index.get(&id)
    else {
        visiting.remove(&id);
        return;
    };
    for child in &module.items {
        let Some(item) = document
            .index
            .get(child)
            .filter(|item| item.visibility == Visibility::Public && !is_hidden_surface(item))
        else {
            continue;
        };
        if let ItemEnum::Use(import) = &item.inner {
            let Some(target) = import.id else {
                continue;
            };
            let mut candidate = prefix.to_vec();
            if !import.is_glob {
                candidate.push(import.name.clone());
                prefer_public_path(&mut surface.paths, target, candidate.join("::"));
            }
            if let Some(summary) = document
                .paths
                .get(&target)
                .filter(|summary| summary.crate_id != 0)
            {
                if let Some(external) = document.external_crates.get(&summary.crate_id) {
                    surface.external_reexports.push(ExternalReexport {
                        crate_name: external.name.clone(),
                        canonical_path: summary.path.join("::"),
                        public_path: candidate.join("::"),
                        expands_descendants: import.is_glob || summary.kind == ItemKind::Module,
                    });
                }
            } else if import.is_glob {
                visit_public_surface(document, target, prefix, surface, visiting);
            } else if document
                .index
                .get(&target)
                .is_some_and(|target| matches!(target.inner, ItemEnum::Module(_)))
            {
                visit_public_surface(document, target, &candidate, surface, visiting);
            }
            continue;
        }
        let Some(name) = item.name.as_ref() else {
            continue;
        };
        let mut candidate = prefix.to_vec();
        candidate.push(name.clone());
        prefer_public_path(&mut surface.paths, *child, candidate.join("::"));
        if matches!(item.inner, ItemEnum::Module(_)) {
            visit_public_surface(document, *child, &candidate, surface, visiting);
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
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

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

    fn ordinary_surface_items() -> Vec<(Id, Item)> {
        let root = Id(0);
        let hidden_use = Id(1);
        let visible_use = Id(2);
        let stripped_module = Id(3);
        let visible_glob = Id(4);
        let hidden_target = Id(10);
        let visible_target = Id(11);
        vec![
            (
                root,
                module_item(
                    root,
                    "sample",
                    vec![hidden_use, visible_use, visible_glob, Id(6)],
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
                    ItemEnum::Primitive(rustdoc_types::Primitive {
                        name: "recovered".to_owned(),
                        impls: Vec::new(),
                    }),
                ),
            ),
        ]
    }

    fn doc_hidden_surface_items() -> Vec<(Id, Item)> {
        let module = Id(5);
        let glob = Id(6);
        let target = Id(12);
        let mut module_item = module_item(module, "hidden", vec![target], false, true);
        module_item
            .attrs
            .push(rustdoc_types::Attribute::Other("#[doc(hidden)]".to_owned()));
        vec![
            (module, module_item),
            (
                glob,
                use_item(glob, "hidden", "hidden", module, true, false),
            ),
            (
                target,
                item(
                    target,
                    Some("DocHidden"),
                    Vec::new(),
                    ItemEnum::Primitive(rustdoc_types::Primitive {
                        name: "doc_hidden".to_owned(),
                        impls: Vec::new(),
                    }),
                ),
            ),
        ]
    }

    fn hidden_surface_document() -> (RustdocCrate, Id, Id, Id) {
        use rustdoc_types::{ItemKind, ItemSummary, Target};

        let root = Id(0);
        let mut index = ordinary_surface_items();
        index.extend(doc_hidden_surface_items());
        let document = RustdocCrate {
            root,
            crate_version: Some("1.0.0".to_owned()),
            includes_private: false,
            index: HashMap::from_iter(index),
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
        (document, Id(10), Id(11), Id(12))
    }

    #[test]
    fn public_paths_skip_hidden_bindings_but_keep_visible_reexports() {
        let (document, hidden_target, visible_target, doc_hidden_target) =
            hidden_surface_document();
        assert_eq!(
            public_paths(&document),
            BTreeMap::from([
                (hidden_target, "sample::Recovered".to_owned()),
                (visible_target, "sample::Visible".to_owned()),
                (doc_hidden_target, "sample::DocHidden".to_owned()),
            ])
        );
        assert!(!is_hidden_surface(&document.index[&Id(3)]));
        assert!(is_hidden_surface(&document.index[&Id(5)]));
    }

    #[test]
    fn named_local_module_reexports_traverse_the_reexported_namespace() {
        let (mut document, _, _, _) = hidden_surface_document();
        let named_use = Id(20);
        let module = Id(21);
        let value = Id(22);
        let ItemEnum::Module(root) = &mut document.index.get_mut(&document.root).unwrap().inner
        else {
            panic!("fixture root is a module");
        };
        root.items.push(named_use);
        document.index.insert(
            named_use,
            use_item(named_use, "implementation", "api", module, false, false),
        );
        document.index.insert(
            module,
            module_item(module, "implementation", vec![value], false, false),
        );
        document.index.insert(
            value,
            item(
                value,
                Some("Value"),
                Vec::new(),
                ItemEnum::Primitive(rustdoc_types::Primitive {
                    name: "value".to_owned(),
                    impls: Vec::new(),
                }),
            ),
        );

        let paths = public_paths(&document);
        assert_eq!(paths.get(&module).map(String::as_str), Some("sample::api"));
        assert_eq!(
            paths.get(&value).map(String::as_str),
            Some("sample::api::Value")
        );
    }

    #[test]
    fn external_module_and_glob_reexports_retain_every_public_prefix() {
        use std::path::PathBuf;

        use rustdoc_types::{ExternalCrate, ItemSummary, Target};

        let root = Id(0);
        let prelude = Id(1);
        let named_use = Id(2);
        let glob_use = Id(3);
        let external_module = Id(4);
        let document = RustdocCrate {
            root,
            crate_version: Some("1.0.0".to_owned()),
            includes_private: false,
            index: HashMap::from([
                (
                    root,
                    module_item(root, "facade", vec![named_use, prelude], true, false),
                ),
                (
                    prelude,
                    module_item(prelude, "prelude", vec![glob_use], false, false),
                ),
                (
                    named_use,
                    use_item(
                        named_use,
                        "owner::values",
                        "values",
                        external_module,
                        false,
                        false,
                    ),
                ),
                (
                    glob_use,
                    use_item(
                        glob_use,
                        "owner::values",
                        "values",
                        external_module,
                        true,
                        false,
                    ),
                ),
            ]),
            paths: HashMap::from([
                (
                    root,
                    ItemSummary {
                        crate_id: 0,
                        path: vec!["facade".to_owned()],
                        kind: ItemKind::Module,
                    },
                ),
                (
                    external_module,
                    ItemSummary {
                        crate_id: 1,
                        path: vec!["owner".to_owned(), "values".to_owned()],
                        kind: ItemKind::Module,
                    },
                ),
            ]),
            external_crates: HashMap::from([(
                1,
                ExternalCrate {
                    name: "owner".to_owned(),
                    html_root_url: None,
                    path: PathBuf::from("/owner.rlib"),
                },
            )]),
            target: Target {
                triple: "x86_64-unknown-linux-gnu".to_owned(),
                target_features: Vec::new(),
            },
            format_version: 57,
        };

        assert_eq!(
            external_reexports(&document),
            vec![
                ExternalReexport {
                    crate_name: "owner".to_owned(),
                    canonical_path: "owner::values".to_owned(),
                    public_path: "facade::prelude".to_owned(),
                    expands_descendants: true,
                },
                ExternalReexport {
                    crate_name: "owner".to_owned(),
                    canonical_path: "owner::values".to_owned(),
                    public_path: "facade::values".to_owned(),
                    expands_descendants: true,
                },
            ]
        );
    }

    #[test]
    fn crate_manifest_does_not_depend_on_compiler_or_cli() {
        let manifest = include_str!("../Cargo.toml");
        assert!(!manifest.contains("terrane-compiler"));
        assert!(!manifest.contains("terrane-cli"));
    }
}
