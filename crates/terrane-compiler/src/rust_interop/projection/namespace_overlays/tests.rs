use super::*;
use crate::RustDependency;
use crate::rust_interop::projection::{
    ProjectedFunction, ProjectedItem, ProjectedKind, ProjectedType,
};

fn dependency(name: &str, package: &str, features: &[&str]) -> RustDependency {
    RustDependency {
        name: name.to_owned(),
        package: package.to_owned(),
        version: "=1.0.0".to_owned(),
        features: features
            .iter()
            .map(|feature| (*feature).to_owned())
            .collect(),
        default_features: true,
        target: None,
        effects: Vec::new(),
    }
}
#[test]
fn dependency_metadata_uses_resolved_default_features_and_renamed_packages_for_overlays() {
    let dependencies = [
        dependency("storage", "data-engine", &[]),
        dependency("overlays", "integration-overlays", &[]),
    ];
    let metadata = serde_json::json!({
        "packages": [
            {
                "id": "registry+data-engine@1.0.0",
                "name": "data-engine",
                "version": "1.0.0",
                "metadata": {}
            },
            {
                "id": "path+integration-overlays@1.0.0",
                "name": "integration-overlays",
                "version": "1.0.0",
                "metadata": {
                    "terrane": {
                        "namespace-overlays": [{
                            "module": "data_engine",
                            "target-package": "data-engine",
                            "feature": "data-engine"
                        }]
                    }
                }
            }
        ],
        "resolve": {
            "root": "path+root@0.1.0",
            "nodes": [
                {
                    "id": "path+root@0.1.0",
                    "deps": [
                        {"name": "storage", "pkg": "registry+data-engine@1.0.0"},
                        {"name": "overlays", "pkg": "path+integration-overlays@1.0.0"}
                    ]
                },
                {
                    "id": "registry+data-engine@1.0.0",
                    "features": []
                },
                {
                    "id": "path+integration-overlays@1.0.0",
                    "features": ["default", "data-engine"]
                }
            ]
        }
    });
    assert_eq!(
        namespace_overlays_from_metadata(&metadata, &dependencies).unwrap(),
        [NamespaceOverlay {
            provider_name: "overlays".to_owned(),
            provider_package: "integration-overlays".to_owned(),
            source_namespace: "/deps/overlays/data-engine".to_owned(),
            target_namespace: "/deps/storage".to_owned(),
        }]
    );
}
fn projected_function_item(namespace: &str, name: &str, rust_path: &str) -> ProjectedItem {
    ProjectedItem {
        namespace: namespace.to_owned(),
        name: name.to_owned(),
        rust_path: rust_path.to_owned(),
        docs: None,
        kind: ProjectedKind::Function(ProjectedFunction {
            native_owner: None,
            native_path: None,
            name: name.to_owned(),
            generic_parameters: Vec::new(),
            operation_owner_generics: Vec::new(),
            rust_generic_arguments: Vec::new(),
            parameters: Vec::new(),
            result: ProjectedType::None,
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
    }
}
#[test]
fn namespace_overlays_require_a_direct_target_and_reject_collisions() {
    let metadata = serde_json::json!({
        "packages": [{
            "id": "path+adapter@1.0.0",
            "name": "adapter",
            "version": "1.0.0",
            "metadata": {
                "terrane": {
                    "namespace-overlays": [{
                        "module": "upstream",
                        "target-package": "upstream",
                        "feature": "bridge"
                    }]
                }
            }
        }],
        "resolve": {
            "root": "path+root@0.1.0",
            "nodes": [
                {
                    "id": "path+root@0.1.0",
                    "deps": [{"name": "adapter", "pkg": "path+adapter@1.0.0"}]
                },
                {
                    "id": "path+adapter@1.0.0",
                    "features": ["bridge"]
                }
            ]
        }
    });
    let undeclared = namespace_overlays_from_metadata(
        &metadata,
        &[dependency("adapter", "adapter", &["bridge"])],
    )
    .unwrap_err();
    assert!(
        undeclared
            .message
            .contains("targets undeclared package `upstream`")
    );

    let overlay = NamespaceOverlay {
        provider_name: "adapter".to_owned(),
        provider_package: "adapter".to_owned(),
        source_namespace: "/deps/adapter/upstream".to_owned(),
        target_namespace: "/deps/upstream".to_owned(),
    };
    let mut projected = [
        ProjectedDependency {
            name: "upstream".to_owned(),
            native_alias_identities: BTreeMap::new(),
            package: "upstream".to_owned(),
            version: "1.0.0".to_owned(),
            items: vec![projected_function_item(
                "/deps/upstream",
                "open",
                "upstream::open",
            )],
            declined: Vec::new(),
            partial_declines: Vec::new(),
        },
        ProjectedDependency {
            name: "adapter".to_owned(),
            native_alias_identities: BTreeMap::new(),
            package: "adapter".to_owned(),
            version: "1.0.0".to_owned(),
            items: vec![projected_function_item(
                "/deps/adapter/upstream",
                "open",
                "adapter::upstream::open",
            )],
            declined: Vec::new(),
            partial_declines: Vec::new(),
        },
    ];
    let collision = apply_namespace_overlays(&mut projected, &[overlay]).unwrap_err();
    assert!(
        collision
            .message
            .contains("collides on `/deps/upstream::open`")
    );
}
