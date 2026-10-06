use crate::rust_interop::projection::{
    Containment, Projection, ProjectionResolution, ProjectionSource,
};

use super::*;

#[expect(
    clippy::too_many_lines,
    reason = "one regression covers decline, private alias admission, facade visibility, and ambiguous-version rejection"
)]
#[test]
fn transitive_type_owner_uses_unique_locked_recursive_package() {
    let directory =
        std::env::temp_dir().join(format!("terrane-transitive-owner-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    fs::write(
            directory.join("Cargo.lock"),
            "version = 4\n\n[[package]]\nname = \"reqwest\"\nversion = \"0.12.28\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n\n[[package]]\nname = \"http\"\nversion = \"1.3.1\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n",
        )
        .unwrap();
    let dependency = |name: &str| RustDependency {
        name: name.to_owned(),
        package: name.to_owned(),
        version: "=1.0.0".to_owned(),
        features: Vec::new(),
        default_features: true,
        target: None,
        effects: Vec::new(),
    };
    let response_status = || ProjectedDependency {
        name: "reqwest".to_owned(),
        native_alias_identities: BTreeMap::new(),
        package: "reqwest".to_owned(),
        version: "0.12.28".to_owned(),
        items: vec![ProjectedItem {
            namespace: "/deps/reqwest".to_owned(),
            name: "status".to_owned(),
            rust_path: "reqwest::Response::status".to_owned(),
            docs: None,
            kind: ProjectedKind::Function(ProjectedFunction {
                native_owner: None,
                native_path: None,
                name: "status".to_owned(),
                generic_parameters: Vec::new(),
                operation_owner_generics: Vec::new(),
                rust_generic_arguments: Vec::new(),
                parameters: Vec::new(),
                result: ProjectedType::Foreign {
                    rust_path: "http::StatusCode".to_owned(),
                    name: "StatusCode".to_owned(),
                    base_rust_path: "http::StatusCode".to_owned(),
                    arguments: Vec::new(),
                },
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
        }],
        declined: Vec::new(),
        partial_declines: Vec::new(),
    };

    let mut undeclared = vec![response_status()];
    enforce_transitive_reachability(
        &mut undeclared,
        &[dependency("reqwest")],
        &[],
        &directory,
        false,
    )
    .unwrap();
    assert!(undeclared[0].items.is_empty());
    assert!(
        undeclared[0].declined[0]
            .reason
            .contains("references unreachable crate `http` at resolved version `1.3.1`")
    );

    let private_dependencies = recursive_owner_dependencies(
        &[response_status()],
        &[dependency("reqwest")],
        &directory,
        false,
    )
    .unwrap();
    assert_eq!(
        private_dependencies,
        vec![ProjectedBoundDependency {
            name: "__terrane_recursive_http".to_owned(),
            package: "http".to_owned(),
            version: "=1.3.1".to_owned(),
        }]
    );
    let mut private = vec![response_status()];
    rewrite_projected_owner_root(&mut private, "http", "__terrane_recursive_http");
    enforce_transitive_reachability(
        &mut private,
        &[dependency("reqwest")],
        &private_dependencies,
        &directory,
        false,
    )
    .unwrap();
    assert_eq!(private[0].items.len(), 1);
    let facade_projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "recursive-owner".to_owned(),
        content_hash: String::new(),
        dependencies: private.clone(),
        bound_dependencies: private_dependencies.clone(),
        containment: Containment::Enforced,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    assert!(facade_projection.item("/deps/http", "StatusCode").is_none());

    let mut declared = vec![response_status()];
    enforce_transitive_reachability(
        &mut declared,
        &[dependency("reqwest"), dependency("http")],
        &[],
        &directory,
        false,
    )
    .unwrap();
    assert_eq!(declared[0].items.len(), 1);
    fs::write(
            directory.join("Cargo.lock"),
            "version = 4\n\n[[package]]\nname = \"reqwest\"\nversion = \"0.12.28\"\n\n[[package]]\nname = \"http\"\nversion = \"1.3.1\"\n\n[[package]]\nname = \"http\"\nversion = \"0.2.12\"\n",
        )
        .unwrap();
    let mut conflicting = [response_status()];
    let error = enforce_transitive_reachability(
        &mut conflicting,
        &[dependency("reqwest"), dependency("http")],
        &[],
        &directory,
        false,
    )
    .unwrap_err();
    assert!(error.message.contains("http"));
    assert!(error.message.contains("0.2.12"));
    assert!(error.message.contains("1.3.1"));
    fs::remove_dir_all(directory).unwrap();
}
