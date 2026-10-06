use std::collections::{BTreeMap, BTreeSet, HashMap};

use rustdoc_types::{
    GenericArg, GenericArgs, GenericBound, Generics, Id, ItemKind, ItemSummary,
    Path as RustdocPath, TraitBoundModifier, Type,
};
use serde_json::json;

use super::{
    Containment, DeclinedItem, InvocationMode, PartialCallbackShape, PartialProjection,
    PartialProjectionRecord, ProjectedBoundaryCapabilities, ProjectedDependency, ProjectedItem,
    ProjectedKind, ProjectedMemberDemands, ProjectedType, Projection, ProjectionDemandSites,
    ProjectionResolution, ProjectionSource, Receiver, collect_source_foreign,
    generated_projection_units, project_type, projectable_interface_bound, rewrite_rust_bound_root,
};

use super::test_support::projected_function_item;
use terrane_rust_analysis::prefer_public_path;

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
fn canonical_public_reexports_share_demanded_members() {
    let facade = projected_foreign_type_item("/deps/api", "Value", "api::Value", "read");
    let provider = projected_foreign_type_item("/deps/owner", "Value", "owner::Value", "read");
    let demanded = ProjectedMemberDemands::from([(
        (provider.namespace.clone(), provider.name.clone()),
        BTreeSet::from(["read".to_owned()]),
    )]);
    let projection = Projection {
        native_owner_aliases: BTreeMap::from([(
            "api::Value".to_owned(),
            "owner::Value".to_owned(),
        )]),
        cache_identity: "public-member-demands".to_owned(),
        content_hash: String::new(),
        dependencies: [("api", facade.clone()), ("owner", provider.clone())]
            .into_iter()
            .map(|(name, item)| ProjectedDependency {
                name: name.to_owned(),
                package: name.to_owned(),
                version: "0.1.0".to_owned(),
                items: vec![item],
                declined: Vec::new(),
                native_alias_identities: BTreeMap::new(),
                partial_declines: Vec::new(),
            })
            .collect(),
        bound_dependencies: Vec::new(),
        containment: Containment::Enforced,
        source: ProjectionSource::default(),
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    };
    assert_eq!(
        projection.owner_for_projected_type(&ProjectedType::Foreign {
            rust_path: provider.rust_path.clone(),
            base_rust_path: provider.rust_path.clone(),
            name: provider.name.clone(),
            arguments: Vec::new(),
        }),
        Some((provider.namespace.clone(), provider.name.clone()))
    );
    assert_eq!(
        projection.owner_for_projected_type(&ProjectedType::Foreign {
            rust_path: facade.rust_path.clone(),
            base_rust_path: facade.rust_path.clone(),
            name: facade.name.clone(),
            arguments: Vec::new(),
        }),
        Some((facade.namespace.clone(), facade.name.clone()))
    );
    let expanded = super::source_rendering::source_member_demands(
        &projection,
        &[&facade, &provider],
        &demanded,
    );
    assert_eq!(
        expanded[&(facade.namespace, facade.name)],
        BTreeSet::from(["read".to_owned()])
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
                native_alias_identities: BTreeMap::new(),
                package: "one".to_owned(),
                version: "1.0.0".to_owned(),
                items: vec![item("/deps/one", "one::Message")],
                declined: Vec::new(),
                partial_declines: Vec::new(),
            },
            ProjectedDependency {
                name: "two".to_owned(),
                native_alias_identities: BTreeMap::new(),
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
            native_alias_identities: BTreeMap::new(),
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
            native_alias_identities: BTreeMap::new(),
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
        native_alias_identities: BTreeMap::new(),
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
        native_alias_identities: BTreeMap::new(),
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
