use super::super::test_support::projected_function_item;
use super::*;

fn projection_with_operations(reverse: bool) -> ProjectedDependency {
    let mut owner = projected_function_item("/deps/witness", "Value", "witness::Value");
    owner.kind = serde_json::from_value(serde_json::json!({
        "ForeignType": { "methods": [] }
    }))
    .unwrap();
    let mut operations = ["alpha", "zeta"]
        .into_iter()
        .map(|name| {
            let mut item = projected_function_item(
                "/deps/witness/value",
                name,
                &format!("<witness::Value as witness::Protocol>::{name}"),
            );
            let ProjectedKind::Function(function) = &mut item.kind else {
                unreachable!();
            };
            function.native_owner = Some("witness::Value".to_owned());
            function.receiver = Some(Receiver::Borrow);
            function.result = ProjectedType::Bool;
            ProjectedTraitOperation {
                item,
                fallback_namespace: "/deps/witness/value/trait/protocol".to_owned(),
            }
        })
        .collect::<Vec<_>>();
    if reverse {
        operations.reverse();
    }
    let mut items = vec![owner];
    let mut declined = Vec::new();
    merge_projected_trait_operations(&mut items, &mut declined, operations);
    let free_names = if reverse {
        ["alpha_free", "zeta_free"]
    } else {
        ["zeta_free", "alpha_free"]
    };
    items.extend(
        free_names.map(|name| {
            projected_function_item("/deps/witness", name, &format!("witness::{name}"))
        }),
    );
    ProjectedDependency {
        name: "witness".to_owned(),
        package: "witness".to_owned(),
        version: "1.0.0".to_owned(),
        native_alias_identities: BTreeMap::new(),
        items,
        declined,
        associated_implementations: Vec::new(),
        partial_declines: Vec::new(),
    }
}

fn projection(dependency: ProjectedDependency) -> Projection {
    Projection {
        cache_identity: "same-inputs".to_owned(),
        content_hash: String::new(),
        dependencies: vec![dependency],
        native_owner_aliases: BTreeMap::new(),
        containment: Containment::Enforced,
        bound_dependencies: Vec::new(),
        source: ProjectionSource::Local,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution::default(),
        removed: Vec::new(),
    }
}

#[test]
fn final_inventory_replays() {
    let mut first_dependency = projection_with_operations(false);
    let mut reversed_dependency = projection_with_operations(true);
    normalize_projected_inventory(&mut first_dependency.items);
    normalize_projected_inventory(&mut reversed_dependency.items);
    let first = projection(first_dependency);
    let mut reversed = projection(reversed_dependency);
    assert_eq!(
        projection_content_hash(&first).unwrap(),
        projection_content_hash(&reversed).unwrap()
    );
    let ProjectedKind::ForeignType { static_methods, .. } =
        &mut reversed.dependencies[0].items[0].kind
    else {
        panic!("expected native owner");
    };
    static_methods
        .iter_mut()
        .find(|method| method.name == "alpha")
        .unwrap()
        .result = ProjectedType::Int;
    assert_ne!(
        projection_content_hash(&first).unwrap(),
        projection_content_hash(&reversed).unwrap()
    );
}

#[test]
fn extractor_traits_keep_qualified_results() {
    let run = |reverse: bool| {
        let mut owner = projected_function_item("/deps/witness", "Value", "witness::Value");
        owner.kind = serde_json::from_value(serde_json::json!({
            "ForeignType": { "methods": [] }
        }))
        .unwrap();
        let mut operations = [
            ("Required", ProjectedType::Int),
            (
                "Optional",
                ProjectedType::Optional(Box::new(ProjectedType::Int)),
            ),
        ]
        .map(|(protocol, result)| {
            let mut item = projected_function_item(
                "/deps/witness/value",
                "extract",
                &format!("<witness::Value as witness::{protocol}>::extract"),
            );
            let ProjectedKind::Function(function) = &mut item.kind else {
                unreachable!();
            };
            function.native_owner = Some("witness::Value".to_owned());
            function.result = result;
            ProjectedTraitOperation {
                item,
                fallback_namespace: format!(
                    "/deps/witness/value/trait/{}",
                    protocol.to_lowercase()
                ),
            }
        });
        if reverse {
            operations.reverse();
        }
        let mut items = vec![owner];
        let mut declined = Vec::new();
        merge_projected_trait_operations(&mut items, &mut declined, operations.into());
        assert_eq!(declined, [] as [DeclinedItem; 0]);
        normalize_projected_inventory(&mut items);
        items
    };
    let items = run(false);
    assert_eq!(items, run(true));
    assert!(
        !items
            .iter()
            .any(|item| item.namespace == "/deps/witness/value")
    );
    for (protocol, expected) in [
        ("required", ProjectedType::Int),
        (
            "optional",
            ProjectedType::Optional(Box::new(ProjectedType::Int)),
        ),
    ] {
        let namespace = format!("/deps/witness/value/trait/{protocol}");
        let item = items
            .iter()
            .find(|item| item.namespace == namespace)
            .unwrap();
        let ProjectedKind::Function(function) = &item.kind else {
            panic!("expected projected extractor");
        };
        assert_eq!(function.result, expected);
    }
}
