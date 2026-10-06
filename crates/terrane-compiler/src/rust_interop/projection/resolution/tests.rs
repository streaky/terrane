use std::fs;

use serde_json::json;

use crate::rust_interop::projection::*;

use super::super::cargo_workspace::DEPENDENCY_LOCK_FILE;
use super::super::test_support::dependency;
use super::*;

#[test]
fn failed_impl_witness_declines_bound_functions_with_the_unproven_interface() {
    let mut dependencies = vec![ProjectedDependency {
        name: "witness".to_owned(),
        native_alias_identities: BTreeMap::new(),
        package: "witness".to_owned(),
        version: "1.0.0".to_owned(),
        items: vec![
            ProjectedItem {
                namespace: "/deps/witness".to_owned(),
                name: "Rejected".to_owned(),
                rust_path: "witness::Rejected".to_owned(),
                docs: None,
                kind: ProjectedKind::Interface(ProjectedInterface {
                    is_unsafe: false,
                    methods: Vec::new(),
                    associated_type: None,
                    supertraits: Vec::new(),
                    send: false,
                    sync: false,
                    requires_drop: false,
                    declined_methods: Vec::new(),
                }),
            },
            ProjectedItem {
                namespace: "/deps/witness".to_owned(),
                name: "Proven".to_owned(),
                rust_path: "witness::Proven".to_owned(),
                docs: None,
                kind: ProjectedKind::Interface(ProjectedInterface {
                    is_unsafe: false,
                    methods: Vec::new(),
                    associated_type: None,
                    supertraits: Vec::new(),
                    send: false,
                    sync: false,
                    requires_drop: false,
                    declined_methods: Vec::new(),
                }),
            },
            ProjectedItem {
                namespace: "/deps/witness".to_owned(),
                name: "rejected_total".to_owned(),
                rust_path: "witness::rejected_total".to_owned(),
                docs: None,
                kind: ProjectedKind::Function(ProjectedFunction {
                    native_owner: None,
                    native_path: None,
                    name: "rejected_total".to_owned(),
                    generic_parameters: Vec::new(),
                    operation_owner_generics: Vec::new(),
                    rust_generic_arguments: Vec::new(),
                    parameters: vec![ProjectedParameter {
                        name: "value".to_owned(),
                        ty: ProjectedType::Generic("T".to_owned()),
                        borrowed: false,
                        mutable_borrow: false,
                        generic_parameter: Some("T".to_owned()),
                        generic_bounds: vec!["witness::Rejected".to_owned()],
                        generic_interface: Some("witness::Rejected".to_owned()),
                        associated_type: None,
                    }],
                    result: ProjectedType::Int,
                    destination_result: None,
                    error: None,
                    is_async: false,
                    is_unsafe: false,
                    into_future: false,
                    execution_requirements: None,
                    enum_operation: None,
                    error_optional_depth: 0,
                    chain_role: None,
                    receiver: None,
                }),
            },
        ],
        declined: Vec::new(),
        partial_declines: Vec::new(),
    }];
    let evidence = vec![crate::rust_interop::ImplProbeEvidence {
        question: crate::rust_interop::ImplQuestion {
            label: "witness::Rejected".to_owned(),
            source: "compile_error!(\"witness failure\");".to_owned(),
        },
        answer: crate::rust_interop::ProbeAnswer::No,
    }];

    decline_unproven_projected_interfaces(&mut dependencies, &evidence);
    decline_functions_with_missing_generic_interfaces(&mut dependencies);

    assert_eq!(dependencies[0].items[0].rust_path, "witness::Proven");
    assert_eq!(dependencies[0].declined[0].rust_path, "witness::Rejected");
    assert_eq!(
        dependencies[0].declined[0].reason,
        "trait implementation signature is not representable against the resolved dependency"
    );
    assert_eq!(
        dependencies[0].declined[1],
        DeclinedItem {
            rust_path: "witness::rejected_total".to_owned(),
            reason: "generic input references declined interface `witness::Rejected`".to_owned(),
        }
    );
}
#[test]
fn dependency_free_resolution_records_its_outcome() {
    let projection = resolve(std::path::Path::new("."), &[], None).unwrap();
    assert_eq!(
        projection.resolution.outcome,
        ResolutionOutcome::NoDependencies
    );
    assert!(projection.resolution.events.is_empty());
    assert_eq!(
        projection.content_hash,
        projection_content_hash(&projection).unwrap()
    );
}
#[test]
fn resolve_regenerates_projection_after_identity_mismatch() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/projection-identity-regression")
        .join(std::process::id().to_string());
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/conformance/run/projected-macro-reexport");
    fs::create_dir_all(root.join(".cargo")).unwrap();
    fs::write(
        root.join(".cargo/config.toml"),
        "[patch.crates-io]\nterrane-reexport-facade-witness = { path = \"fixture-registry/terrane-reexport-facade-witness-0.1.0\" }\nterrane-reexport-owner-witness = { path = \"fixture-registry/terrane-reexport-owner-witness-0.1.0\" }\nterrane-generate-api-witness = { path = \"fixture-registry/terrane-generate-api-witness-0.1.0\" }\n",
    )
    .unwrap();
    for package in [
        "terrane-reexport-facade-witness",
        "terrane-reexport-owner-witness",
        "terrane-generate-api-witness",
    ] {
        let source = fixture
            .join("fixture-registry")
            .join(format!("{package}-0.1.0"));
        let destination = root
            .join("fixture-registry")
            .join(format!("{package}-0.1.0"));
        fs::create_dir_all(destination.join("src")).unwrap();
        fs::copy(source.join("Cargo.toml"), destination.join("Cargo.toml")).unwrap();
        fs::copy(source.join("src/lib.rs"), destination.join("src/lib.rs")).unwrap();
    }
    fs::write(
        root.join(DEPENDENCY_LOCK_FILE),
        "version = 4\n\n[[package]]\nname = \"terrane-generate-api-witness\"\nversion = \"0.1.0\"\n\n[[package]]\nname = \"terrane-reexport-facade-witness\"\nversion = \"0.1.0\"\ndependencies = [\n \"terrane-reexport-owner-witness\",\n]\n\n[[package]]\nname = \"terrane-reexport-owner-witness\"\nversion = \"0.1.0\"\ndependencies = [\n \"terrane-generate-api-witness\",\n]\n",
    )
    .unwrap();
    let dependencies = [
        "terrane-reexport-facade-witness",
        "terrane-reexport-owner-witness",
    ]
    .map(|name| {
        let mut dependency = dependency(name, name, &[]);
        dependency.version = "=0.1.0".to_owned();
        dependency
    });

    let first = resolve(&root, &dependencies, None).unwrap();
    assert!(first.dependencies.iter().any(|dependency| {
        dependency
            .items
            .iter()
            .any(|item| item.name == "GeneratedValue")
    }));
    let cache_path = root.join(".trn/dependencies/projection.json");
    let mut cache: serde_json::Value =
        serde_json::from_slice(&fs::read(&cache_path).unwrap()).unwrap();
    cache["cache_identity"] = json!("stale-projection-identity");
    fs::write(&cache_path, serde_json::to_vec(&cache).unwrap()).unwrap();

    let regenerated = resolve(&root, &dependencies, None).unwrap();
    assert_eq!(
        regenerated.resolution.outcome,
        ResolutionOutcome::LocalRustdoc
    );
    assert!(regenerated.dependencies.iter().any(|dependency| {
        dependency
            .items
            .iter()
            .any(|item| item.name == "GeneratedValue")
    }));
    let cache: serde_json::Value = serde_json::from_slice(&fs::read(&cache_path).unwrap()).unwrap();
    assert_ne!(cache["cache_identity"], "stale-projection-identity");
    fs::remove_dir_all(root).unwrap();
}
