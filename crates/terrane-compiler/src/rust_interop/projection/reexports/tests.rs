use std::collections::{BTreeMap, HashMap};

use rustdoc_types::{Crate as RustdocCrate, ExternalCrate, Id, ItemKind, ItemSummary, Target};
use serde_json::json;

use super::super::test_support::dependency;
use super::*;

#[test]
fn facade_aliases_do_not_rewrite_unrelated_provider_fragments() {
    let declared = BTreeMap::from([(
        "std::io::error::Error".to_owned(),
        "std::io::Error".to_owned(),
    )]);
    let facade = ReexportProvider {
        dependency_index: 0,
        public_paths: BTreeMap::new(),
        canonical_public_paths: BTreeMap::from([(
            "std::io::error::Error".to_owned(),
            "facade::Error".to_owned(),
        )]),
        rust_path_aliases: BTreeMap::new(),
    };
    let unrelated = ReexportProvider {
        dependency_index: 1,
        public_paths: BTreeMap::new(),
        canonical_public_paths: BTreeMap::new(),
        rust_path_aliases: BTreeMap::new(),
    };

    assert_eq!(
        provider_fragment_public_paths(&declared, &facade)["std::io::error::Error"],
        "facade::Error"
    );
    assert_eq!(
        provider_fragment_public_paths(&declared, &unrelated)["std::io::error::Error"],
        "std::io::Error"
    );
}

#[test]
fn external_owner_resolution_declines_ambiguous_library_targets() {
    let metadata = json!({
        "packages": [
            {
                "name": "owner-one",
                "version": "1.0.0",
                "targets": [{"name": "shared_owner", "kind": ["lib"]}]
            },
            {
                "name": "owner-two",
                "version": "2.0.0",
                "targets": [{"name": "shared_owner", "kind": ["lib"]}]
            }
        ]
    });

    let reason = resolved_library_package(&metadata, "shared_owner").unwrap_err();
    assert!(reason.contains("ambiguous"));
    assert!(reason.contains("owner-one@1.0.0"));
    assert!(reason.contains("owner-two@2.0.0"));
}
#[test]
fn builtin_owner_reexports_never_request_supplemental_rustdoc() {
    let root = Id(0);
    let external_error = Id(1);
    let dependency = dependency("facade", "facade", &[]);
    let document = RustdocCrate {
        root,
        crate_version: Some("1.0.0".to_owned()),
        includes_private: false,
        index: HashMap::new(),
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
                external_error,
                ItemSummary {
                    crate_id: 1,
                    path: vec!["std".to_owned(), "io".to_owned(), "Error".to_owned()],
                    kind: ItemKind::Struct,
                },
            ),
        ]),
        external_crates: HashMap::from([(
            1,
            ExternalCrate {
                name: "std".to_owned(),
                html_root_url: None,
                path: std::path::PathBuf::from("/std.rlib"),
            },
        )]),
        target: Target {
            triple: "x86_64-unknown-linux-gnu".to_owned(),
            target_features: Vec::new(),
        },
        format_version: rustdoc_types::FORMAT_VERSION,
    };
    let rustdocs = vec![(
        &dependency,
        document,
        BTreeMap::from([(external_error, "facade::Error".to_owned())]),
    )];
    let mut declines = vec![Vec::new()];
    let workspace =
        std::env::temp_dir().join(format!("terrane-builtin-owner-{}", std::process::id()));

    let supplemental = external_reexport_rustdocs(
        &workspace,
        &rustdocs,
        &[],
        &json!({"packages": []}),
        "x86_64-unknown-linux-gnu",
        Containment::Unavailable,
        None,
        &mut declines,
    )
    .unwrap();

    assert!(supplemental.is_empty());
    assert!(declines[0].is_empty());
    assert!(!workspace.exists());
}
#[test]
fn owner_rustdoc_identity_mismatch_regenerates_usable_cache() {
    let workspace = std::env::temp_dir().join(format!(
        "terrane-owner-rustdoc-cache-{}",
        std::process::id()
    ));
    let package = workspace.join("owner");
    fs::create_dir_all(package.join("src")).unwrap();
    fs::write(
        workspace.join("Cargo.toml"),
        "[workspace]\nmembers = [\"owner\"]\nresolver = \"2\"\n",
    )
    .unwrap();
    fs::write(
        workspace.join("Cargo.lock"),
        "version = 4\n\n[[package]]\nname = \"owner-fixture\"\nversion = \"1.0.0\"\n",
    )
    .unwrap();
    fs::write(
        package.join("Cargo.toml"),
        "[package]\nname = \"owner-fixture\"\nversion = \"1.0.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    fs::write(
        package.join("src/lib.rs"),
        "pub struct UsableOwnerType;\npub fn usable_owner_function() -> bool { true }\n",
    )
    .unwrap();
    fs::create_dir_all(workspace.join("owner-rustdoc")).unwrap();
    fs::write(
        workspace.join("owner-rustdoc/owner_fixture.json"),
        br#"{"terrane_cache_identity":"stale","terrane_cache_package":"owner-fixture@1.0.0","terrane_cache_crate":"owner_fixture"}"#,
    )
    .unwrap();

    let document = super::cached_owner_rustdoc(
        &workspace,
        "owner-fixture@1.0.0",
        "owner_fixture",
        "owner-fixture",
        "x86_64-unknown-linux-gnu",
        Containment::Unavailable,
    )
    .unwrap();
    let cached = super::cached_owner_rustdoc(
        &workspace,
        "owner-fixture@1.0.0",
        "owner_fixture",
        "owner-fixture",
        "x86_64-unknown-linux-gnu",
        Containment::Unavailable,
    )
    .unwrap();
    assert!(
        cached
            .index
            .values()
            .any(|item| item.name.as_deref() == Some("UsableOwnerType"))
    );
    assert!(
        cached
            .index
            .values()
            .any(|item| item.name.as_deref() == Some("usable_owner_function"))
    );
    assert!(
        document
            .index
            .values()
            .any(|item| item.name.as_deref() == Some("UsableOwnerType"))
    );
    assert!(
        document
            .index
            .values()
            .any(|item| item.name.as_deref() == Some("usable_owner_function"))
    );
    let cache: serde_json::Value = serde_json::from_slice(
        &fs::read(workspace.join("owner-rustdoc/owner_fixture.json")).unwrap(),
    )
    .unwrap();
    assert_ne!(cache["terrane_cache_identity"], "stale");
    assert_eq!(cache["terrane_cache_package"], "owner-fixture@1.0.0");
    assert_eq!(cache["terrane_cache_crate"], "owner_fixture");
    fs::remove_dir_all(workspace).unwrap();
}
