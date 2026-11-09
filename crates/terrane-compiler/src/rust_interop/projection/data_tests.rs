use super::super::test_support::projected_function_item;
use super::*;

fn projection_with_operations(reverse: bool) -> Projection {
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
    normalize_projected_items(&mut items, &mut declined);
    normalize_projected_members(&mut items);
    Projection {
        cache_identity: "same-inputs".to_owned(),
        content_hash: String::new(),
        dependencies: vec![ProjectedDependency {
            name: "witness".to_owned(),
            package: "witness".to_owned(),
            version: "1.0.0".to_owned(),
            native_alias_identities: BTreeMap::new(),
            items,
            declined,
            associated_implementations: Vec::new(),
            partial_declines: Vec::new(),
        }],
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
fn member_order_replays() {
    let first = projection_with_operations(false);
    let mut reversed = projection_with_operations(true);
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
