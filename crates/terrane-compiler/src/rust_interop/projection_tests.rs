use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;

use rustdoc_types::{
    Crate as RustdocCrate, ExternalCrate, GenericArg, GenericArgs, GenericBound, Generics, Id,
    ItemKind, ItemSummary, Path as RustdocPath, Target, Trait, TraitBoundModifier, Type,
};
use serde_json::json;

use super::{
    ArtifactDependency, Containment, DEPENDENCY_LOCK_FILE, DeclinedItem, InvocationMode,
    NamespaceOverlay, PartialCallbackShape, PartialProjection, PartialProjectionRecord,
    ProjectedBoundDependency, ProjectedBoundaryCapabilities, ProjectedDependency,
    ProjectedFunction, ProjectedInterface, ProjectedItem, ProjectedKind, ProjectedMemberDemands,
    ProjectedParameter, ProjectedType, Projection, ProjectionArtifact, ProjectionDemandSites,
    ProjectionHistory, ProjectionResolution, ProjectionSource, Receiver, ReexportProvider,
    ResolutionOutcome, apply_namespace_overlays, apply_projection_history, builtin_callable_mode,
    collect_source_foreign, decline_functions_with_missing_generic_interfaces,
    decline_unproven_projected_interfaces, enforce_transitive_reachability,
    external_reexport_rustdocs, foreign_aliases, generated_projection_units,
    has_supported_callable_trait_shape, instantiated_nominal_name, instantiated_type_name,
    is_builtin_clone, is_builtin_marker_trait, is_internal_rust_protocol_method,
    mark_cache_record_used, namespace_overlays_from_metadata, parse_rustdoc,
    persist_dependency_lock, project_type, projectable_interface_bound, projection_content_hash,
    provider_fragment_public_paths, prune_projection_cache, receiver_kind,
    recursive_owner_dependencies, resolve, resolved_library_package, rewrite_projected_owner_root,
    rewrite_rust_bound_root, seed_dependency_lock, selected_target, validate_projection_artifact,
};
#[test]
fn rust_protocol_plumbing_is_not_reported_as_a_callable_gap() {
    assert!(is_internal_rust_protocol_method("core::fmt::Debug", "fmt"));
    assert!(is_internal_rust_protocol_method(
        "core::fmt::Display",
        "fmt"
    ));
    assert!(is_internal_rust_protocol_method("core::hash::Hash", "hash"));
    assert!(!is_internal_rust_protocol_method(
        "core::convert::From",
        "from"
    ));
}

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
use crate::RustDependency;
use terrane_rust_analysis::prefer_public_path;
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

fn projected_foreign_type_item(
    namespace: &str,
    name: &str,
    rust_path: &str,
    method_name: &str,
) -> ProjectedItem {
    let ProjectedKind::Function(method) = projected_function_item(
        namespace,
        method_name,
        &format!("{rust_path}::{method_name}"),
    )
    .kind
    else {
        unreachable!();
    };
    ProjectedItem {
        namespace: namespace.to_owned(),
        name: name.to_owned(),
        rust_path: rust_path.to_owned(),
        docs: None,
        kind: ProjectedKind::ForeignType {
            constructor: None,
            methods: vec![method],
            static_methods: Vec::new(),
            constants: Vec::new(),
            boundary: ProjectedBoundaryCapabilities::default(),
            fields: Vec::new(),
            borrowed_view: false,
            native_view_type: None,
            enum_payload: None,
            generic_parameters: Vec::new(),
            displayable: false,
            cloneable: false,
            send: false,
            sync: false,
        },
    }
}

#[test]
fn canonical_projected_item_name_wins_over_provider_path_hash() {
    let point = projected_foreign_type_item("/deps/iced", "Point", "iced::Point", "distance");
    let mut rectangle =
        projected_foreign_type_item("/deps/iced", "Rectangle", "iced::Rectangle", "center");
    let ProjectedKind::ForeignType { methods, .. } = &mut rectangle.kind else {
        unreachable!();
    };
    methods[0].result = ProjectedType::Foreign {
        rust_path: "iced::Point".to_owned(),
        name: "Point-provider-path-hash".to_owned(),
        base_rust_path: "iced::Point".to_owned(),
        arguments: Vec::new(),
    };
    let all_items = vec![&point, &rectangle];

    let foreign = collect_source_foreign(&all_items, &all_items, &ProjectedMemberDemands::new());

    assert_eq!(
        foreign.get("iced::Point").map(String::as_str),
        Some("Point")
    );
}

#[test]
fn generic_foreign_name_is_disambiguated_only_on_collision() {
    let arc_custom = "std::sync::Arc<iced::theme::Custom>";
    let arc_name = instantiated_type_name("Arc", arc_custom);
    let aliases = foreign_aliases(&BTreeMap::from([(arc_custom.to_owned(), arc_name)]));

    assert_eq!(aliases.get(arc_custom).map(String::as_str), Some("Arc"));

    let arc_other = "std::sync::Arc<witness::Other>";
    let aliases = foreign_aliases(&BTreeMap::from([
        (
            arc_custom.to_owned(),
            instantiated_type_name("Arc", arc_custom),
        ),
        (
            arc_other.to_owned(),
            instantiated_type_name("Arc", arc_other),
        ),
    ]));
    assert_eq!(aliases[arc_custom], "Arc-of-iced-theme-Custom");
    assert_eq!(aliases[arc_other], "Arc-of-witness-Other");
}

#[test]
fn instantiated_foreign_names_describe_their_canonical_arguments() {
    assert_eq!(
        instantiated_type_name(
            "RangeInclusive",
            "core::ops::range::RangeInclusive<iced::Degrees>"
        ),
        "RangeInclusive-of-iced-Degrees"
    );
    let degrees = ProjectedType::Foreign {
        rust_path: "iced::Degrees".to_owned(),
        name: "Degrees".to_owned(),
        base_rust_path: "iced::Degrees".to_owned(),
        arguments: Vec::new(),
    };
    assert_eq!(
        instantiated_nominal_name(
            "RangeInclusive",
            "core::ops::range::RangeInclusive<iced::Degrees>",
            &[degrees]
        ),
        "RangeInclusive-of-iced-Degrees"
    );
    let open = instantiated_nominal_name(
        "Query",
        "sqlx_core::query::Query<'q, DB, A>",
        &[
            ProjectedType::Generic("DB".to_owned()),
            ProjectedType::Generic("A".to_owned()),
        ],
    );
    assert!(open.starts_with("Query-"));
    assert!(!open.starts_with("Query-of-"));
    let application = ProjectedType::Opaque {
        anonymous_chain: false,
        bounds: vec![
            "iced_program::Program<State = State, Message = Message, Theme = Theme>".to_owned(),
        ],
    };
    assert_eq!(
        instantiated_nominal_name("Application", "iced::Application<opaque>", &[application]),
        "Application-of-Program-State-Message-Theme"
    );
    assert_eq!(
        instantiated_type_name(
            "Result",
            "core::result::Result<alloc::vec::Vec<iced::Point>, iced::Error>"
        ),
        "Result-of-alloc-vec-Vec-iced-Point-and-iced-Error"
    );
}

#[test]
fn inventory_member_admission_requires_syntax_valid_rendering() {
    let ProjectedKind::Function(valid) =
        projected_function_item("/deps/witness", "ready", "witness::ready").kind
    else {
        unreachable!();
    };
    assert_eq!(super::inventory_member_syntax_gap(&valid), None);

    let ProjectedKind::Function(reserved) =
        projected_function_item("/deps/witness", "to", "witness::to").kind
    else {
        unreachable!();
    };
    assert_eq!(
        super::inventory_member_syntax_gap(&reserved).as_deref(),
        Some("expected `;` before function parameters")
    );
}

#[test]
fn projected_type_lookup_is_scoped_to_canonical_namespace() {
    let item = |namespace: &str, rust_path: &str| {
        let mut item = projected_function_item(namespace, "make", rust_path);
        let ProjectedKind::Function(function) = &mut item.kind else {
            unreachable!();
        };
        function.result = ProjectedType::Foreign {
            rust_path: rust_path.to_owned(),
            name: "Message".to_owned(),
            base_rust_path: rust_path.to_owned(),
            arguments: Vec::new(),
        };
        item
    };
    let projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "canonical-identities".to_owned(),
        content_hash: String::new(),
        dependencies: vec![
            ProjectedDependency {
                name: "one".to_owned(),
                package: "one".to_owned(),
                version: "1.0.0".to_owned(),
                items: vec![item("/deps/one", "one::Message")],
                declined: Vec::new(),
                partial_declines: Vec::new(),
            },
            ProjectedDependency {
                name: "two".to_owned(),
                package: "two".to_owned(),
                version: "1.0.0".to_owned(),
                items: vec![item("/deps/two", "two::Message")],
                declined: Vec::new(),
                partial_declines: Vec::new(),
            },
        ],
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::default(),
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };

    assert_eq!(
        projection
            .projected_type("/deps/two", "Message")
            .map(|ty| ty.rust_type()),
        Some("two::Message".to_owned())
    );
    assert!(projection.projected_type("/app", "Message").is_none());
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

#[test]
fn target_identity_reads_effective_cargo_configuration() {
    let directory =
        std::env::temp_dir().join(format!("terrane-projection-target-{}", std::process::id()));
    fs::create_dir_all(directory.join(".cargo")).unwrap();
    fs::write(
        directory.join(".cargo/config.toml"),
        "[build]\ntarget = \"wasm32-unknown-unknown\"\n",
    )
    .unwrap();
    assert_eq!(
        selected_target(&directory, "host: x86_64-unknown-linux-gnu").unwrap(),
        "wasm32-unknown-unknown"
    );
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn bound_alias_rewrite_only_changes_path_roots() {
    assert_eq!(
        rewrite_rust_bound_root(
            "for<'value> factory::Decode<'value> + outer::factory::Marker",
            "factory",
            "renamed",
        ),
        "for<'value> renamed::Decode<'value> + outer::factory::Marker"
    );
}

#[test]
fn equally_short_public_paths_use_lexical_tie_breaking() {
    let id = Id(1);
    let mut paths = BTreeMap::new();
    prefer_public_path(&mut paths, id, "crate::zeta::Item".to_owned());
    prefer_public_path(&mut paths, id, "crate::alpha::Item".to_owned());
    assert_eq!(paths[&id], "crate::alpha::Item");
}

#[test]
fn substantive_public_paths_beat_shorter_prelude_paths() {
    let id = Id(1);
    let mut paths = BTreeMap::new();
    prefer_public_path(&mut paths, id, "crate::prelude::Item".to_owned());
    prefer_public_path(&mut paths, id, "crate::algorithm::special::Item".to_owned());
    assert_eq!(paths[&id], "crate::algorithm::special::Item");
}

#[test]
fn projected_map_and_set_keys_require_scalars() {
    assert!(ProjectedType::String.is_terrane_scalar());
    assert!(ProjectedType::Bytes.is_terrane_scalar());
    assert!(!ProjectedType::Optional(Box::new(ProjectedType::String)).is_terrane_scalar());
    assert!(
        !ProjectedType::Sequence {
            rust_path: "alloc::vec::Vec<String>".to_owned(),
            item: Box::new(ProjectedType::String),
        }
        .is_terrane_scalar()
    );
    assert!(!ProjectedType::Tuple(vec![ProjectedType::String]).is_terrane_scalar());
}
#[test]
fn generated_projection_document_splits_into_one_namespace_units() {
    let document = "# report\n\
                        # Generated source unit: /deps/iced\n\
                        # Native path: iced::Element\n\
                        namespace deps/iced\n\
                        class Element\n\
                        \n\
                        # Generated source unit: /deps/iced/application\n\
                        namespace deps/iced/application\n\
                        interface ViewFn\n";
    let units = generated_projection_units(document).unwrap();
    assert_eq!(units.len(), 2);
    assert_eq!(units[0].namespace, "/deps/iced");
    assert_eq!(
        units[0].source,
        "# Native path: iced::Element\nnamespace deps/iced\nclass Element"
    );
    assert_eq!(units[1].namespace, "/deps/iced/application");
    assert_eq!(
        units[1].source,
        "namespace deps/iced/application\ninterface ViewFn"
    );
    assert_eq!(&document[units[0].start..][..20], "# Native path: iced:");
}

#[test]
fn generated_projection_unit_rejects_a_mismatched_namespace() {
    let error = generated_projection_units(
        "# Generated source unit: /deps/iced\nnamespace deps/other\nclass Element\n",
    )
    .unwrap_err();
    assert!(error.contains("does not begin with `namespace deps/iced`"));
}

#[test]
fn projected_callback_name_is_the_language_server_signature() {
    let callback = ProjectedType::Callback {
        rust_name: "F".to_owned(),
        native_bound: None,
        native_method: None,
        native_result: None,
        native_substitutions: BTreeMap::new(),
        parameters: vec![ProjectedType::String, ProjectedType::Bool],
        parameter_rust_types: vec!["String".to_owned(), "bool".to_owned()],
        parameter_borrows: vec![false, false],
        parameters_destination_selected: false,
        result: Box::new(ProjectedType::Int),
        invocation_mode: InvocationMode::Shared,
        is_async: true,
        retained: true,
        send: true,
        sync: true,
    };
    assert_eq!(
        callback.terrane_name(),
        "async function from string, bool to int"
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
#[expect(
    clippy::too_many_lines,
    reason = "one inventory scenario verifies direct, inherited, admitted, and unused ordering"
)]
fn unavailable_inventory_distinguishes_unused_and_demanded_declines() {
    let projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "inventory".to_owned(),
        content_hash: "content".to_owned(),
        dependencies: vec![ProjectedDependency {
            name: "witness".to_owned(),
            package: "witness".to_owned(),
            version: "1.0.0".to_owned(),
            items: vec![projected_function_item(
                "/deps/witness",
                "accepted",
                "witness::accepted",
            )],
            declined: vec![
                DeclinedItem {
                    rust_path: "witness::required_gap".to_owned(),
                    reason: "requires a borrowed result".to_owned(),
                },
                DeclinedItem {
                    rust_path: "witness::callback_gap".to_owned(),
                    reason: "requires an unsupported callback".to_owned(),
                },
                DeclinedItem {
                    rust_path: "witness::unused_gap".to_owned(),
                    reason: "requires an unsupported generic".to_owned(),
                },
            ],
            partial_declines: vec![
                PartialProjectionRecord {
                    rust_path: "witness::required_gap".to_owned(),
                    reason: "requires a borrowed result".to_owned(),
                    references: BTreeSet::from([
                        "witness::accepted".to_owned(),
                        "witness::callback_gap".to_owned(),
                    ]),
                    projection: PartialProjection::Function {
                        signature: "fn required_gap(callback: impl witness::Callback)".to_owned(),
                        generic_constraints: vec!["State: 'static".to_owned()],
                        callback_shapes: vec![PartialCallbackShape {
                            parameter: "callback".to_owned(),
                            contract: "witness::Callback".to_owned(),
                            methods: vec!["fn invoke(self: &Self, state: &State)".to_owned()],
                        }],
                    },
                },
                PartialProjectionRecord {
                    rust_path: "witness::callback_gap".to_owned(),
                    reason: "requires an unsupported callback".to_owned(),
                    references: BTreeSet::new(),
                    projection: PartialProjection::NominalType {
                        declaration: "interface callback-gap".to_owned(),
                        native_kind: "trait".to_owned(),
                        generic_parameters: vec!["State".to_owned()],
                    },
                },
            ],
        }],
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::default(),
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    let demands = ProjectionDemandSites::from([(
        ("/deps/witness".to_owned(), "required-gap".to_owned(), None),
        BTreeSet::from(["src/main.trn:7:9".to_owned()]),
    )]);

    let inventory = projection.unavailable_inventory(&demands);

    assert_eq!(inventory.len(), 3);
    assert_eq!(inventory[0].rust_path, "witness::callback_gap");
    assert!(inventory[0].required_by.is_empty());
    assert_eq!(
        inventory[0].required_by_contracts,
        BTreeSet::from(["witness::required_gap".to_owned()])
    );
    assert_eq!(inventory[1].rust_path, "witness::required_gap");
    assert_eq!(
        inventory[1].required_by,
        BTreeSet::from(["src/main.trn:7:9".to_owned()])
    );
    assert_eq!(inventory[2].rust_path, "witness::unused_gap");
    assert!(inventory[2].required_by.is_empty());

    let document = projection.documented_inventory(&demands);
    let required_heading = document
        .find("# Required unavailable projected declarations")
        .unwrap();
    let callback_gap = document
        .find("# Native path: witness::callback_gap")
        .unwrap();
    let required_gap = document
        .find("# Native path: witness::required_gap")
        .unwrap();
    let unused_heading = document
        .find("# Unused unavailable projected declarations")
        .unwrap();
    let unused_gap = document.find("# Native path: witness::unused_gap").unwrap();
    let projected_heading = document.find("# Projected Terrane declarations").unwrap();
    assert!(required_heading < callback_gap);
    assert!(callback_gap < required_gap);
    assert!(required_gap < projected_heading);
    assert!(projected_heading < unused_heading);
    assert!(unused_heading < unused_gap);
    assert!(document.contains(
        "# Function shape: fn required_gap(callback: impl witness::Callback)\n\
             # Generic constraint: State: 'static\n\
             # Callback parameter `callback`: witness::Callback\n\
             #   Required callback method: fn invoke(self: &Self, state: &State)"
    ));
    assert!(document.contains(
        "# Native path: witness::callback_gap\n\
             # Terrane namespace: /deps/witness\n\
             # Unavailable declaration: callback-gap\n\
             # Projection status: unavailable (required)"
    ));
    assert!(document.contains("# Required by projection contracts:\n# - witness::required_gap"));
    assert!(document.contains(
        "# Required admitted projected declarations\n\
             #\n\
             # Native path: witness::accepted\n\
             # Terrane namespace: /deps/witness\n\
             # Projected declaration: accepted\n\
             # Required by projection contracts:\n\
             # - witness::required_gap"
    ));
    assert!(document.contains(
        "# Generated partial contract: parameterized native trait template\n\
             # Native trait template parameters: State\n\
             # Terrane nominal declaration: none"
    ));
    assert!(!document.contains("interface callback-gap"));
    let units = generated_projection_units(&document).unwrap();
    assert!(units.is_empty());
}

#[test]
fn documented_inventory_populates_admitted_members_and_comments_declines() {
    let projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "members".to_owned(),
        content_hash: "content".to_owned(),
        dependencies: vec![ProjectedDependency {
            name: "witness".to_owned(),
            package: "witness".to_owned(),
            version: "1.0.0".to_owned(),
            items: vec![projected_foreign_type_item(
                "/deps/witness",
                "Widget",
                "witness::Widget",
                "ready",
            )],
            declined: vec![DeclinedItem {
                rust_path: "witness::Widget::blocked".to_owned(),
                reason: "requires an unsupported borrow".to_owned(),
            }],
            partial_declines: vec![PartialProjectionRecord {
                rust_path: "witness::Widget::blocked".to_owned(),
                reason: "requires an unsupported borrow".to_owned(),
                references: BTreeSet::new(),
                projection: PartialProjection::Function {
                    signature: "fn blocked(&self) -> &str".to_owned(),
                    generic_constraints: Vec::new(),
                    callback_shapes: Vec::new(),
                },
            }],
        }],
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::default(),
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };

    let document = projection.documented_inventory(&ProjectionDemandSites::new());

    assert!(document.contains("class Widget\n    function ready;"));
    assert!(document.contains("  # Unavailable native members retained for structure"));
    assert!(document.contains("  # blocked\n  # Native path: witness::Widget::blocked"));
    assert!(document.contains("  # Native signature: fn blocked(&self) -> &str"));
    assert!(document.contains("  # Projection gap: requires an unsupported borrow"));
}

#[test]
fn exact_decline_path_wins_over_conflicting_suffixes() {
    let dependency = |name: &str, rust_path: &str, reason: &str| ProjectedDependency {
        name: name.to_owned(),
        package: name.to_owned(),
        version: "1.0.0".to_owned(),
        items: Vec::new(),
        declined: vec![DeclinedItem {
            rust_path: rust_path.to_owned(),
            reason: reason.to_owned(),
        }],
        partial_declines: Vec::new(),
    };
    let dependencies = vec![
        dependency("one", "one::Type::build", "first reason"),
        dependency("two", "two::Type::build", "second reason"),
    ];
    assert_eq!(
        Projection::unique_declined_reason(&dependencies, Some("one::Type::build"), "::build"),
        Some("first reason")
    );
    assert_eq!(
        Projection::unique_declined_reason(&dependencies, None, "::build"),
        None
    );
}

#[test]
fn ambiguous_projected_type_identities_do_not_resolve() {
    let dependency = |name: &str, rust_path: &str| ProjectedDependency {
        name: name.to_owned(),
        package: name.to_owned(),
        version: "1.0.0".to_owned(),
        items: vec![ProjectedItem {
            namespace: "/deps/shared".to_owned(),
            name: "Generic".to_owned(),
            rust_path: rust_path.to_owned(),
            docs: None,
            kind: ProjectedKind::ForeignType {
                constructor: None,
                methods: Vec::new(),
                static_methods: Vec::new(),
                constants: Vec::new(),
                boundary: ProjectedBoundaryCapabilities::default(),
                fields: Vec::new(),
                borrowed_view: false,
                native_view_type: None,
                enum_payload: None,
                generic_parameters: Vec::new(),
                displayable: false,
                cloneable: false,
                send: false,
                sync: false,
            },
        }],
        declined: Vec::new(),
        partial_declines: Vec::new(),
    };
    let nested_dependency = |dependency_name: &str, rust_path: &str| {
        let mut dependency = dependency(dependency_name, rust_path);
        let mut item = projected_function_item("/deps/shared", dependency_name, "shared::make");
        let ProjectedKind::Function(function) = &mut item.kind else {
            unreachable!();
        };
        function.result = ProjectedType::Foreign {
            rust_path: rust_path.to_owned(),
            name: "Message".to_owned(),
            base_rust_path: rust_path.to_owned(),
            arguments: Vec::new(),
        };
        dependency.items = vec![item];
        dependency
    };
    let projection = |dependencies| Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "ambiguous-identities".to_owned(),
        content_hash: String::new(),
        dependencies,
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::default(),
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };

    let mut first = dependency("one", "one::Generic<A>");
    let ProjectedKind::ForeignType { send, .. } = &mut first.items[0].kind else {
        unreachable!();
    };
    *send = true;
    let direct = projection(vec![first, dependency("two", "two::Generic<B>")]);
    assert!(direct.item("/deps/shared", "Generic").is_none());
    assert_eq!(
        direct.item_ambiguity("/deps/shared", "Generic").as_deref(),
        Some("projected Rust types `one::Generic<A>`, `two::Generic<B>`")
    );
    assert!(direct.projected_type("/deps/shared", "Generic").is_none());
    assert!(!direct.projected_type_is_send("/deps/shared", "Generic"));
    let source_error = direct
        .source_for_imports(&BTreeMap::from([(
            "/deps/shared".to_owned(),
            BTreeSet::from(["Generic".to_owned()]),
        )]))
        .unwrap_err();
    assert_eq!(
        source_error,
        "projected import `/deps/shared::Generic` is ambiguous: projected Rust types `one::Generic<A>`, `two::Generic<B>`"
    );
    assert!(
        projection(vec![
            nested_dependency("one", "one::Message"),
            nested_dependency("two", "two::Message"),
        ])
        .projected_type("/deps/shared", "Message")
        .is_none()
    );
}

#[test]
fn receiver_kind_preserves_only_plain_self_receivers() {
    let borrowed = |is_mutable| Type::BorrowedRef {
        lifetime: None,
        is_mutable,
        type_: Box::new(Type::Generic("Self".to_owned())),
    };
    assert_eq!(
        receiver_kind(&borrowed(true)).unwrap(),
        Receiver::MutableBorrow
    );
    assert_eq!(receiver_kind(&borrowed(false)).unwrap(), Receiver::Borrow);
    assert_eq!(
        receiver_kind(&Type::Generic("Self".to_owned())).unwrap(),
        Receiver::Move
    );
    assert!(receiver_kind(&Type::Primitive("str".to_owned())).is_err());
}

#[test]
fn static_reference_aliases_cannot_bypass_parameter_admission() {
    let generics = Generics {
        params: Vec::new(),
        where_predicates: Vec::new(),
    };
    let reference = |lifetime: &str| Type::BorrowedRef {
        lifetime: Some(lifetime.to_owned()),
        is_mutable: false,
        type_: Box::new(Type::Primitive("str".to_owned())),
    };
    let alias_id = Id(42);
    let alias = rustdoc_types::Item {
        id: alias_id,
        crate_id: 0,
        name: Some("StaticText".to_owned()),
        span: None,
        visibility: rustdoc_types::Visibility::Public,
        docs: None,
        links: HashMap::new(),
        attrs: Vec::new(),
        deprecation: None,
        inner: rustdoc_types::ItemEnum::TypeAlias(rustdoc_types::TypeAlias {
            type_: reference("'static"),
            generics: generics.clone(),
        }),
    };
    let mut function = rustdoc_types::Function {
        sig: rustdoc_types::FunctionSignature {
            inputs: vec![(
                "text".to_owned(),
                Type::ResolvedPath(RustdocPath {
                    path: "StaticText".to_owned(),
                    id: alias_id,
                    args: None,
                }),
            )],
            output: None,
            is_c_variadic: false,
        },
        generics,
        header: rustdoc_types::FunctionHeader {
            is_const: false,
            is_unsafe: false,
            is_async: false,
            abi: rustdoc_types::Abi::Rust,
        },
        has_body: true,
    };
    let index = HashMap::from([(alias_id, alias)]);
    let failure = super::project_function(
        &function,
        &index,
        &HashMap::new(),
        &BTreeMap::new(),
        None,
        false,
    )
    .unwrap_err();
    assert!(failure.contains("requires a static reference"));

    function.sig.inputs[0].1 = reference("'input");
    let projected = super::project_function(
        &function,
        &index,
        &HashMap::new(),
        &BTreeMap::new(),
        None,
        false,
    )
    .unwrap();
    assert_eq!(projected.parameters[0].ty, ProjectedType::String);
    assert!(projected.parameters[0].borrowed);
}

#[test]
fn failed_impl_witness_declines_bound_functions_with_the_unproven_interface() {
    let mut dependencies = vec![ProjectedDependency {
        name: "witness".to_owned(),
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
#[expect(
    clippy::too_many_lines,
    reason = "one identity matrix keeps callable and marker trait aliases together"
)]
fn builtin_callback_trait_recognition_uses_canonical_identity() {
    let fn_id = Id(1);
    let spoofed_fn_id = Id(2);
    let fn_mut_id = Id(3);
    let fn_once_id = Id(4);
    let send_id = Id(5);
    let spoofed_send_id = Id(6);
    let sync_id = Id(7);
    let clone_id = Id(8);
    let paths = HashMap::from([
        (
            fn_id,
            ItemSummary {
                crate_id: 0,
                path: vec![
                    "core".to_owned(),
                    "ops".to_owned(),
                    "function".to_owned(),
                    "Fn".to_owned(),
                ],
                kind: ItemKind::Trait,
            },
        ),
        (
            spoofed_fn_id,
            ItemSummary {
                crate_id: 1,
                path: vec!["witness".to_owned(), "Fn".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
        (
            fn_mut_id,
            ItemSummary {
                crate_id: 0,
                path: vec![
                    "core".to_owned(),
                    "ops".to_owned(),
                    "function".to_owned(),
                    "FnMut".to_owned(),
                ],
                kind: ItemKind::Trait,
            },
        ),
        (
            fn_once_id,
            ItemSummary {
                crate_id: 0,
                path: vec![
                    "core".to_owned(),
                    "ops".to_owned(),
                    "function".to_owned(),
                    "FnOnce".to_owned(),
                ],
                kind: ItemKind::Trait,
            },
        ),
        (
            send_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["core".to_owned(), "marker".to_owned(), "Send".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
        (
            spoofed_send_id,
            ItemSummary {
                crate_id: 1,
                path: vec!["witness".to_owned(), "Send".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
        (
            sync_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["core".to_owned(), "marker".to_owned(), "Sync".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
        (
            clone_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["core".to_owned(), "clone".to_owned(), "Clone".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
    ]);
    let path = |id, display: &str| RustdocPath {
        path: display.to_owned(),
        id,
        args: None,
    };

    assert_eq!(
        builtin_callable_mode(&path(fn_id, "renamed::Anything"), &paths),
        Some(InvocationMode::Shared)
    );
    assert_eq!(
        builtin_callable_mode(&path(fn_mut_id, "renamed::Anything"), &paths),
        Some(InvocationMode::Mutable)
    );
    assert_eq!(
        builtin_callable_mode(&path(fn_once_id, "renamed::Anything"), &paths),
        Some(InvocationMode::Consuming)
    );
    assert_eq!(
        builtin_callable_mode(&path(spoofed_fn_id, "Fn"), &paths),
        None
    );
    assert!(is_builtin_marker_trait(
        &path(send_id, "renamed::Anything"),
        &paths,
        "Send"
    ));
    assert!(!is_builtin_marker_trait(
        &path(spoofed_send_id, "Send"),
        &paths,
        "Send"
    ));
    assert!(is_builtin_marker_trait(
        &path(sync_id, "renamed::Anything"),
        &paths,
        "Sync"
    ));
    assert!(is_builtin_clone(
        &path(clone_id, "renamed::Anything"),
        &paths
    ));
    assert!(!is_builtin_clone(&path(spoofed_fn_id, "Clone"), &paths));
}

#[test]
fn custom_callable_traits_reject_restrictive_shapes() {
    let base = Trait {
        is_auto: false,
        is_unsafe: false,
        is_dyn_compatible: true,
        items: vec![Id(1)],
        generics: Generics {
            params: Vec::new(),
            where_predicates: Vec::new(),
        },
        bounds: Vec::new(),
        implementations: Vec::new(),
    };
    let index = HashMap::new();

    for declaration in [
        Trait {
            is_auto: true,
            ..base.clone()
        },
        Trait {
            is_unsafe: true,
            ..base.clone()
        },
        Trait {
            bounds: vec![GenericBound::Outlives("'static".to_owned())],
            ..base.clone()
        },
        Trait {
            items: vec![Id(1), Id(2)],
            ..base
        },
    ] {
        assert!(!has_supported_callable_trait_shape(&declaration, &index));
    }
}

#[test]
fn instantiated_generic_paths_share_public_name_and_keep_native_identity() {
    let id = Id(1);
    let paths = HashMap::from([(
        id,
        ItemSummary {
            crate_id: 0,
            path: vec!["witness".to_owned(), "Wrapper".to_owned()],
            kind: ItemKind::Struct,
        },
    )]);
    let instantiated = |primitive: &str| {
        Type::ResolvedPath(RustdocPath {
            path: "witness::Wrapper".to_owned(),
            id,
            args: Some(Box::new(GenericArgs::AngleBracketed {
                args: vec![GenericArg::Type(Type::Primitive(primitive.to_owned()))],
                constraints: Vec::new(),
            })),
        })
    };
    let index = HashMap::new();
    let generics = BTreeMap::new();

    let left = project_type(&instantiated("u8"), &index, &paths, &generics).unwrap();
    let right = project_type(&instantiated("u16"), &index, &paths, &generics).unwrap();

    assert_ne!(left, right);
    assert!(matches!(
        left,
        ProjectedType::Foreign {
            rust_path, name, ..
        }
            if rust_path == "witness::Wrapper<u8>" && name == "Wrapper"
    ));
    assert!(matches!(
        right,
        ProjectedType::Foreign {
            rust_path, name, ..
        }
            if rust_path == "witness::Wrapper<u16>" && name == "Wrapper"
    ));
}
#[test]
fn standard_aggregates_project_recursively() {
    let vec_id = Id(1);
    let string_id = Id(2);
    let map_id = Id(3);
    let option_id = Id(4);
    let paths = HashMap::from([
        (
            vec_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["alloc".to_owned(), "vec".to_owned(), "Vec".to_owned()],
                kind: ItemKind::Struct,
            },
        ),
        (
            string_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["alloc".to_owned(), "string".to_owned(), "String".to_owned()],
                kind: ItemKind::Struct,
            },
        ),
        (
            map_id,
            ItemSummary {
                crate_id: 0,
                path: vec![
                    "std".to_owned(),
                    "collections".to_owned(),
                    "HashMap".to_owned(),
                ],
                kind: ItemKind::Struct,
            },
        ),
        (
            option_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["core".to_owned(), "option".to_owned(), "Option".to_owned()],
                kind: ItemKind::Enum,
            },
        ),
    ]);
    let resolved = |id, path: &str, arguments: Vec<Type>| {
        Type::ResolvedPath(RustdocPath {
            path: path.to_owned(),
            id,
            args: Some(Box::new(GenericArgs::AngleBracketed {
                args: arguments.into_iter().map(GenericArg::Type).collect(),
                constraints: Vec::new(),
            })),
        })
    };
    let string = resolved(string_id, "alloc::string::String", Vec::new());
    let vector = resolved(vec_id, "alloc::vec::Vec", vec![string.clone()]);
    let map = resolved(
        map_id,
        "std::collections::HashMap",
        vec![string, Type::Primitive("u16".to_owned())],
    );
    let optional = resolved(
        option_id,
        "core::option::Option",
        vec![Type::Primitive("u16".to_owned())],
    );
    let index = HashMap::new();
    let generics = BTreeMap::new();

    assert!(matches!(
        project_type(&vector, &index, &paths, &generics).unwrap(),
        ProjectedType::Sequence { item, .. } if *item == ProjectedType::String
    ));
    assert!(matches!(
        project_type(&map, &index, &paths, &generics).unwrap(),
        ProjectedType::Mapping { key, value, ordered: false, .. }
            if *key == ProjectedType::String
                && *value == ProjectedType::RustInt("u16".to_owned())
    ));
    assert_eq!(
        project_type(&optional, &index, &paths, &generics).unwrap(),
        ProjectedType::Optional(Box::new(ProjectedType::RustInt("u16".to_owned())))
    );
    assert_eq!(
        project_type(
            &Type::Tuple(vec![
                Type::Primitive("u16".to_owned()),
                Type::Primitive("u16".to_owned()),
            ]),
            &index,
            &paths,
            &generics,
        )
        .unwrap(),
        ProjectedType::Tuple(vec![
            ProjectedType::RustInt("u16".to_owned()),
            ProjectedType::RustInt("u16".to_owned()),
        ])
    );
}

#[test]
fn method_lookup_keeps_colliding_foreign_types_in_their_namespaces() {
    let projection: Projection = serde_json::from_value(json!({
        "cache_identity": "test",
        "containment": "Unavailable",
        "dependencies": [{
            "name": "witness",
            "package": "witness",
            "version": "1.0.0",
            "declined": [],

            "items": [
                {
                    "namespace": "/deps/witness/async",
                    "name": "Response",
                    "rust_path": "witness::async::Response",
                    "docs": null,
                    "kind": {"ForeignType": {"methods": [{
                        "name": "touch",
                        "parameters": [],
                        "result": "None",
                        "error": null,
                        "is_async": false,
                        "receiver": "Borrow"
                    }]}}
                },
                {
                    "namespace": "/deps/witness/blocking",
                    "name": "Response",
                    "rust_path": "witness::blocking::Response",
                    "docs": null,
                    "kind": {"ForeignType": {"methods": [{
                        "name": "touch",
                        "parameters": [],
                        "result": "None",
                        "error": null,
                        "is_async": false,
                        "receiver": "MutableBorrow"
                    }]}}
                }
            ]
        }]
    }))
    .unwrap();

    assert_eq!(
        projection
            .method("/deps/witness/async", "Response", "touch", false)
            .and_then(|method| method.receiver),
        Some(Receiver::Borrow)
    );
    assert_eq!(
        projection
            .method("/deps/witness/blocking", "Response", "touch", false)
            .and_then(|method| method.receiver),
        Some(Receiver::MutableBorrow)
    );
}
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

#[test]
fn durable_dependency_lock_seeds_and_records_the_complete_cargo_graph() {
    let root = std::env::temp_dir().join(format!(
        "terrane-durable-dependency-lock-{}",
        std::process::id()
    ));
    let workspace = root.join(".trn/dependencies");
    fs::create_dir_all(&workspace).unwrap();
    let resolved = b"version = 4\n\n[[package]]\nname = \"facade\"\nversion = \"1.0.0\"\n\n[[package]]\nname = \"owner\"\nversion = \"2.0.0\"\n";
    fs::write(workspace.join("Cargo.lock"), resolved).unwrap();
    persist_dependency_lock(&root, &workspace).unwrap();
    assert_eq!(fs::read(root.join(DEPENDENCY_LOCK_FILE)).unwrap(), resolved);
    fs::write(workspace.join("Cargo.lock"), b"stale").unwrap();
    seed_dependency_lock(&root, &workspace).unwrap();
    assert_eq!(fs::read(workspace.join("Cargo.lock")).unwrap(), resolved);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn function_signatures_keep_same_named_foreign_types_distinct() {
    let projection: Projection = serde_json::from_value(json!({
        "cache_identity": "test",
        "containment": "Unavailable",
        "dependencies": [{
            "name": "witness",
            "package": "witness",
            "version": "1.0.0",
            "declined": [],
            "items": [{
                "namespace": "/deps/witness",
                "name": "cross",
                "rust_path": "witness::cross",
                "docs": null,
                "kind": {"Function": {
                    "name": "cross",
                    "parameters": [
                        {
                            "name": "left",
                            "ty": {"Foreign": {
                                "rust_path": "witness::left::Response",
                                "name": "Response"
                            }},
                            "borrowed": false,
                            "mutable_borrow": false
                        },
                        {
                            "name": "right",
                            "ty": {"Foreign": {
                                "rust_path": "witness::right::Response",
                                "name": "Response"
                            }},
                            "borrowed": false,
                            "mutable_borrow": false
                        }
                    ],
                    "result": "None",
                    "error": null,
                    "is_async": false,
                    "receiver": null
                }}
            }]
        }]
    }))
    .unwrap();

    assert_eq!(
        projection.foreign_imports("/deps/witness"),
        BTreeMap::from([
            (
                "Response-hash-h20d8feeba582".to_owned(),
                "witness::left::Response".to_owned()
            ),
            (
                "Response-hash-h52b4adb0f322".to_owned(),
                "witness::right::Response".to_owned()
            )
        ])
    );
    let sources = projection
        .source_for_imports(&BTreeMap::from([(
            "/deps/witness".to_owned(),
            BTreeSet::from(["cross".to_owned()]),
        )]))
        .unwrap();
    assert!(sources[0].1.contains(
        "function cross; left Response-hash-h20d8feeba582, right Response-hash-h52b4adb0f322"
    ));
}

#[test]
fn projected_source_cycles_are_diagnostic() {
    let cycle = Projection::order_projected_sources(vec![
        (
            "/deps/one".to_owned(),
            String::new(),
            BTreeSet::from(["/deps/two".to_owned()]),
        ),
        (
            "/deps/two".to_owned(),
            String::new(),
            BTreeSet::from(["/deps/one".to_owned()]),
        ),
    ])
    .unwrap_err();

    assert_eq!(
        cycle,
        "projected dependency source namespaces contain an import cycle: \
             /deps/one -> [/deps/two]; /deps/two -> [/deps/one]; this is the recorded \
             `projection/mutually-referential-namespace-sources` limitation"
    );
}
#[test]
fn wider_primitives_project_without_narrowing() {
    let paths = HashMap::new();
    let index = HashMap::new();
    let generics = BTreeMap::new();

    assert_eq!(
        project_type(&Type::Primitive("u8".to_owned()), &index, &paths, &generics).unwrap(),
        ProjectedType::RustInt("u8".to_owned())
    );
    assert_eq!(
        project_type(
            &Type::Primitive("f32".to_owned()),
            &index,
            &paths,
            &generics
        )
        .unwrap(),
        ProjectedType::Float32
    );
    assert_eq!(
        project_type(
            &Type::Primitive("char".to_owned()),
            &index,
            &paths,
            &generics
        )
        .unwrap(),
        ProjectedType::Char
    );
}

#[test]
fn rustdoc_schema_mismatch_is_explicit() {
    let document = json!({
        "root": 0,
        "crate_version": "1.0.0",
        "includes_private": false,
        "index": {},
        "paths": {},
        "external_crates": {},
        "target": {"triple": "x86_64-unknown-linux-gnu", "target_features": []},
        "format_version": rustdoc_types::FORMAT_VERSION + 1
    });
    let dependency = RustDependency {
        name: "witness".to_owned(),
        package: "witness".to_owned(),
        version: "=1.0.0".to_owned(),
        features: Vec::new(),
        default_features: true,
        target: None,
        effects: Vec::new(),
    };

    let error = parse_rustdoc(&dependency, &serde_json::to_vec(&document).unwrap())
        .expect_err("an unsupported rustdoc schema must fail");

    assert!(error.message.contains("schema mismatch"));
    assert!(
        error
            .message
            .contains(&format!("format {}", rustdoc_types::FORMAT_VERSION + 1))
    );
    assert!(error.message.contains(&format!(
        "expected format {}",
        rustdoc_types::FORMAT_VERSION
    )));
}

#[test]
fn malformed_rustdoc_json_is_not_silently_declined() {
    let dependency = RustDependency {
        name: "witness".to_owned(),
        package: "witness".to_owned(),
        version: "=1.0.0".to_owned(),
        features: Vec::new(),
        default_features: true,
        target: None,
        effects: Vec::new(),
    };
    let malformed = format!(r#"{{"format_version":{}}}"#, rustdoc_types::FORMAT_VERSION);

    let error = parse_rustdoc(&dependency, malformed.as_bytes())
        .expect_err("malformed rustdoc JSON must fail");

    assert!(error.message.contains("schema mismatch"));
    assert!(error.message.contains(&format!(
        "expected rustdoc format {}",
        rustdoc_types::FORMAT_VERSION
    )));
}

#[test]
fn remote_artifact_requires_exact_projection_inputs() {
    let dependency = RustDependency {
        name: "witness".to_owned(),
        package: "witness-package".to_owned(),
        version: "=1.2.3".to_owned(),
        features: vec!["derive".to_owned()],
        default_features: false,
        target: Some("cfg(unix)".to_owned()),
        effects: vec!["filesystem".to_owned()],
    };
    let mut payload = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "exact".to_owned(),
        content_hash: String::new(),
        dependencies: Vec::new(),
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    payload.content_hash = projection_content_hash(&payload).unwrap();
    let artifact = ProjectionArtifact {
        format: 1,
        cache_identity: "exact".to_owned(),
        content_hash: payload.content_hash.clone(),
        target: "x86_64-unknown-linux-gnu".to_owned(),
        build_toolchain: crate::BUILD_TOOLCHAIN.to_owned(),
        rustdoc_toolchain: crate::RUSTDOC_TOOLCHAIN.to_owned(),
        rustdoc_format: rustdoc_types::FORMAT_VERSION,
        projection_schema: super::PROJECTION_SCHEMA.to_owned(),
        dependencies: vec![ArtifactDependency::from(&dependency)],
        projection: payload,
    };

    let projection = validate_projection_artifact(
        artifact.clone(),
        "exact",
        "x86_64-unknown-linux-gnu",
        std::slice::from_ref(&dependency),
        Containment::Unavailable,
    )
    .expect("exact artifact metadata must be admitted");
    assert_eq!(projection.source, ProjectionSource::Remote);
    assert_eq!(projection.containment, Containment::Unavailable);

    let reject = |candidate| {
        validate_projection_artifact(
            candidate,
            "exact",
            "x86_64-unknown-linux-gnu",
            std::slice::from_ref(&dependency),
            Containment::Unavailable,
        )
        .unwrap_err()
    };

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].version = "=9.9.9".to_owned();
    assert_eq!(
        reject(mismatched),
        "dependency metadata mismatch for `witness`"
    );

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].effects = vec!["network".to_owned()];
    assert_eq!(
        reject(mismatched),
        "dependency metadata mismatch for `witness`"
    );

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].features = vec!["different".to_owned()];
    assert_eq!(
        reject(mismatched),
        "feature metadata mismatch for `witness`"
    );

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].default_features = true;
    assert_eq!(
        reject(mismatched),
        "feature metadata mismatch for `witness`"
    );

    let mut mismatched = artifact.clone();
    mismatched.dependencies[0].target = Some("cfg(windows)".to_owned());
    assert_eq!(
        reject(mismatched),
        "dependency target mismatch for `witness`: expected `cfg(unix)`, found `cfg(windows)`"
    );

    let mut corrupt = artifact;
    corrupt.content_hash = "not-the-payload-hash".to_owned();
    let reason = validate_projection_artifact(
        corrupt,
        "exact",
        "x86_64-unknown-linux-gnu",
        &[dependency],
        Containment::Unavailable,
    )
    .unwrap_err();
    assert!(reason.contains("content hash mismatch"));
}

#[expect(
    clippy::too_many_lines,
    reason = "the history regression records a complete two-generation cache transition"
)]
#[test]
fn projection_history_retains_removed_members_across_checks() {
    let directory =
        std::env::temp_dir().join(format!("terrane-projection-history-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let dependency = |version: &str, items: Vec<ProjectedItem>| ProjectedDependency {
        name: "fixture".to_owned(),
        package: "fixture".to_owned(),
        version: version.to_owned(),
        items,
        declined: Vec::new(),
        partial_declines: Vec::new(),
    };
    let item = ProjectedItem {
        namespace: "/deps/fixture".to_owned(),
        name: "removed".to_owned(),
        rust_path: "fixture::removed".to_owned(),
        docs: None,
        kind: ProjectedKind::ForeignType {
            constructor: None,
            boundary: ProjectedBoundaryCapabilities::default(),
            fields: Vec::new(),
            borrowed_view: false,
            native_view_type: None,
            enum_payload: None,
            generic_parameters: Vec::new(),
            displayable: false,
            methods: vec![ProjectedFunction {
                native_owner: None,
                native_path: None,
                name: "read".to_owned(),
                generic_parameters: Vec::new(),
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
                receiver: Some(Receiver::Borrow),
            }],
            static_methods: vec![ProjectedFunction {
                native_owner: None,
                native_path: None,
                name: "create".to_owned(),
                generic_parameters: Vec::new(),
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
            }],
            constants: Vec::new(),
            cloneable: false,
            send: false,
            sync: false,
        },
    };
    let mut old = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "old".to_owned(),
        content_hash: String::new(),
        dependencies: vec![dependency("1.0.0", vec![item])],
        bound_dependencies: Vec::new(),
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    old.content_hash = projection_content_hash(&old).unwrap();
    apply_projection_history(&directory, &mut old).unwrap();
    let lock_path = directory.join("terrane-projection.lock");
    let mut reviewed: serde_json::Value =
        serde_json::from_slice(&fs::read(&lock_path).unwrap()).unwrap();
    let object = reviewed.as_object_mut().unwrap();
    for field in [
        "cache_identity",
        "projection_schema",
        "content_hash",
        "resolution",
    ] {
        object.remove(field);
    }
    fs::write(&lock_path, serde_json::to_vec_pretty(&reviewed).unwrap()).unwrap();
    let mut current = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "current".to_owned(),
        content_hash: String::new(),
        dependencies: vec![dependency("2.0.0", Vec::new())],
        bound_dependencies: Vec::new(),
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    current.content_hash = projection_content_hash(&current).unwrap();
    apply_projection_history(&directory, &mut current).unwrap();
    assert_eq!(
        current
            .removed
            .iter()
            .map(|item| item.name.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["removed", "removed.read", "removed::create"])
    );
    current.removed.clear();
    apply_projection_history(&directory, &mut current).unwrap();
    assert_eq!(current.removed.len(), 3);
    let persisted: serde_json::Value =
        serde_json::from_slice(&fs::read(&lock_path).unwrap()).unwrap();
    assert_eq!(persisted["dependencies"][0]["version"], "2.0.0");
    assert_eq!(persisted["removed"].as_array().unwrap().len(), 3);
    for field in [
        "cache_identity",
        "projection_schema",
        "content_hash",
        "resolution",
    ] {
        assert!(persisted.get(field).is_none(), "unexpected `{field}`");
    }
    assert_eq!(persisted["source"], "Local");
    assert_eq!(persisted["rustdoc_format"], rustdoc_types::FORMAT_VERSION);
    let reviewed_bytes = serde_json::to_vec(&persisted).unwrap();
    fs::write(&lock_path, &reviewed_bytes).unwrap();
    apply_projection_history(&directory, &mut current).unwrap();
    assert_eq!(fs::read(&lock_path).unwrap(), reviewed_bytes);
    current.dependencies[0].version = "2.0.1".to_owned();
    current.content_hash = projection_content_hash(&current).unwrap();
    apply_projection_history(&directory, &mut current).unwrap();
    let updated: serde_json::Value =
        serde_json::from_slice(&fs::read(&lock_path).unwrap()).unwrap();
    assert_eq!(updated["dependencies"][0]["version"], "2.0.1");
    assert_eq!(updated["removed"].as_array().unwrap().len(), 3);
    assert!(updated.get("cache_identity").is_none());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn projection_history_keeps_content_origin_across_cache_hits() {
    let directory =
        std::env::temp_dir().join(format!("terrane-projection-origin-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let generated = ProjectionResolution {
        outcome: ResolutionOutcome::LocalRustdoc,
        events: Vec::new(),
    };
    let mut projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "stable-identity".to_owned(),
        content_hash: String::new(),
        dependencies: Vec::new(),
        bound_dependencies: vec![ProjectedBoundDependency {
            name: "serde_core".to_owned(),
            package: "serde_core".to_owned(),
            version: "1.0.229".to_owned(),
        }],
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: generated.clone(),
        removed: Vec::new(),
    };
    projection.content_hash = projection_content_hash(&projection).unwrap();
    apply_projection_history(&directory, &mut projection).unwrap();
    projection.resolution = ProjectionResolution {
        outcome: ResolutionOutcome::ExactCache,
        events: Vec::new(),
    };
    apply_projection_history(&directory, &mut projection).unwrap();
    let history: ProjectionHistory =
        serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
            .unwrap();
    assert_eq!(history.resolution, Some(generated));
    assert_eq!(history.format, 4);
    assert_eq!(history.bound_dependencies, projection.bound_dependencies);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn legacy_format_three_history_migrates_without_losing_removed_members() {
    let legacy = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/projection-history/format-3.lock"
    ));
    let inventory: serde_json::Value = serde_json::from_str(legacy).unwrap();
    let expected = inventory["dependencies"][0]["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|member| {
            (
                member[0].as_str().unwrap().to_owned(),
                member[1].as_str().unwrap().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    let directory = std::env::temp_dir().join(format!(
        "terrane-projection-format-three-{}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).unwrap();
    fs::write(directory.join("terrane-projection.lock"), legacy).unwrap();
    let mut projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "format-three-migration".to_owned(),
        content_hash: String::new(),
        dependencies: vec![ProjectedDependency {
            name: "bytes".to_owned(),
            package: "bytes".to_owned(),
            version: "1.10.2".to_owned(),
            items: Vec::new(),
            declined: Vec::new(),
            partial_declines: Vec::new(),
        }],
        bound_dependencies: Vec::new(),
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution {
            outcome: ResolutionOutcome::LocalRustdoc,
            events: Vec::new(),
        },
        removed: Vec::new(),
    };
    projection.content_hash = projection_content_hash(&projection).unwrap();
    apply_projection_history(&directory, &mut projection).unwrap();
    assert_eq!(
        projection
            .removed
            .iter()
            .map(|member| (member.namespace.clone(), member.name.clone(),))
            .collect::<BTreeSet<_>>(),
        expected
    );
    let migrated: ProjectionHistory =
        serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
            .unwrap();
    assert_eq!(migrated.format, 4);
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn projection_history_migrates_provenance_and_detects_replay_drift() {
    let directory = std::env::temp_dir().join(format!(
        "terrane-projection-provenance-{}",
        std::process::id()
    ));
    fs::create_dir_all(&directory).unwrap();
    fs::write(
        directory.join("terrane-projection.lock"),
        b"{\"format\":1,\"dependencies\":[],\"removed\":[]}\n",
    )
    .unwrap();
    let mut projection = Projection {
        native_owner_aliases: BTreeMap::default(),
        cache_identity: "stable-identity".to_owned(),
        content_hash: String::new(),
        dependencies: Vec::new(),
        bound_dependencies: Vec::new(),
        containment: Containment::Unavailable,
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    projection.content_hash = projection_content_hash(&projection).unwrap();
    apply_projection_history(&directory, &mut projection).unwrap();

    let migrated: ProjectionHistory =
        serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
            .unwrap();
    assert_eq!(migrated.format, 4);
    assert_eq!(migrated.source, Some(ProjectionSource::Local));
    assert_eq!(migrated.rustdoc_format, Some(rustdoc_types::FORMAT_VERSION));
    assert_eq!(
        migrated.content_hash.as_deref(),
        Some(projection.content_hash.as_str())
    );

    projection.source = ProjectionSource::Remote;
    apply_projection_history(&directory, &mut projection).unwrap();
    let changed_source: ProjectionHistory =
        serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
            .unwrap();
    assert_eq!(changed_source.source, Some(ProjectionSource::Remote));

    projection.probe_wall_time_ms = 1;
    projection.content_hash = projection_content_hash(&projection).unwrap();
    let error = apply_projection_history(&directory, &mut projection).unwrap_err();
    assert!(error.message.contains("projection replay mismatch"));
    assert!(error.message.contains("previously produced"));
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn projection_cache_retains_bounded_families_and_unrelated_files() {
    let directory =
        std::env::temp_dir().join(format!("terrane-projection-prune-{}", std::process::id()));
    let retained = directory.join("projection-current.json");
    let unrelated = directory.join("Cargo.lock");
    fs::create_dir_all(&directory).unwrap();
    fs::write(&retained, b"current").unwrap();
    for index in 0..18 {
        fs::write(
            directory.join(format!("projection-previous-{index}.json")),
            b"previous",
        )
        .unwrap();
        let owner = directory.join(format!("owner-rustdoc-previous-{index}.json"));
        fs::write(&owner, b"owner").unwrap();
        fs::OpenOptions::new()
            .write(true)
            .open(owner)
            .unwrap()
            .set_times(
                fs::FileTimes::new()
                    .set_modified(std::time::UNIX_EPOCH + std::time::Duration::from_secs(index)),
            )
            .unwrap();
    }
    let reused_owner = directory.join("owner-rustdoc-previous-0.json");
    mark_cache_record_used(&reused_owner);
    fs::write(&unrelated, b"lock").unwrap();

    prune_projection_cache(&directory, &retained).unwrap();

    let family_count = |prefix: &str| {
        fs::read_dir(&directory)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with(prefix))
            })
            .count()
    };
    assert_eq!(
        family_count("projection-"),
        super::MAX_PROJECTION_CACHE_RECORDS
    );
    assert_eq!(
        family_count("owner-rustdoc-"),
        super::MAX_OWNER_RUSTDOC_CACHE_RECORDS
    );
    assert!(reused_owner.exists());
    assert!(!directory.join("owner-rustdoc-previous-1.json").exists());
    assert!(!directory.join("owner-rustdoc-previous-2.json").exists());
    assert!(retained.exists());
    assert!(unrelated.exists());
    fs::remove_dir_all(directory).unwrap();
}

#[test]
fn five_owner_rustdoc_working_set_survives_pruning() {
    let directory = std::env::temp_dir().join(format!(
        "terrane-owner-rustdoc-working-set-{}",
        std::process::id()
    ));
    let retained = directory.join("projection-current.json");
    fs::create_dir_all(&directory).unwrap();
    fs::write(&retained, b"current").unwrap();
    let owners = (0..5)
        .map(|index| directory.join(format!("owner-rustdoc-{index}.json")))
        .collect::<Vec<_>>();
    for owner in &owners {
        fs::write(owner, b"owner").unwrap();
    }

    prune_projection_cache(&directory, &retained).unwrap();

    assert!(owners.iter().all(|owner| owner.exists()));
    fs::remove_dir_all(directory).unwrap();
}
#[test]
fn dynamic_trait_shape_rejects_multiple_non_auto_traits() {
    let bound = |id, path: &str| GenericBound::TraitBound {
        trait_: RustdocPath {
            path: path.to_owned(),
            id: Id(id),
            args: None,
        },
        generic_params: Vec::new(),
        modifier: TraitBoundModifier::None,
    };
    let bounds = [bound(1, "witness::One"), bound(2, "witness::Two")];
    let error = projectable_interface_bound(&bounds, &HashMap::new(), &HashMap::new()).unwrap_err();
    assert_eq!(
        error,
        "generic input requires one projectable interface bound"
    );
}
