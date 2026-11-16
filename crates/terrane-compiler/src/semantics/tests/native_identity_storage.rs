use super::*;
use std::cmp::Ordering;

const DEPTH: usize = 32;
const MAX_LAYER_BYTES: usize = 16 * 1024;

fn leaf(name: &str) -> ValueType {
    ValueType::Object(ObjectIdentity::new("test", name))
}

fn nested_layers() -> Vec<ObjectIdentity> {
    let mut child = leaf("leaf");
    let mut layers = Vec::with_capacity(DEPTH);
    for index in 0..DEPTH {
        let ordered = child.clone();
        let named = child.clone();
        let identity = ObjectIdentity::new("test", format!("Layer{index}"))
            .with_type_arguments(vec![ordered])
            .with_native_parameters(vec!["T".to_owned()])
            .with_native_arguments(BTreeMap::from([("T".to_owned(), named)]));

        // Bound every intermediate result so a regression fails at its first
        // explosive layer rather than allocating an unbounded nested tree.
        let debug = format!("{identity:?}");
        assert!(
            debug.len() <= MAX_LAYER_BYTES,
            "debug layer {index} grew to {} bytes",
            debug.len()
        );
        let encoded = serde_json::to_vec(&identity).unwrap();
        assert!(
            encoded.len() <= MAX_LAYER_BYTES,
            "serialized layer {index} grew to {} bytes",
            encoded.len()
        );

        child = ValueType::Object(identity.clone());
        layers.push(identity);
    }
    layers
}

#[test]
fn deep_clone_order_debug_bounded() {
    let layers = nested_layers();
    let identity = layers.last().unwrap();
    let cloned = identity.clone();

    assert_eq!(&cloned, identity);
    assert_eq!(cloned.cmp(identity), Ordering::Equal);
    assert_eq!(format!("{cloned:?}"), format!("{identity:?}"));
    assert!(format!("{identity:?}").len() <= MAX_LAYER_BYTES);
}

#[test]
fn deep_identity_serde_preserves_payload() {
    let layers = nested_layers();
    let identity = layers.last().unwrap();
    let bytes = serde_json::to_vec(identity).unwrap();
    let restored: ObjectIdentity = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(&restored, identity);
    assert_eq!(restored.native_parameters, identity.native_parameters);
    assert_eq!(restored.native_arguments, identity.native_arguments);
    assert_eq!(restored.type_arguments, identity.type_arguments);
}

#[test]
fn serde_keeps_distinct_native_binding() {
    let positional = leaf("first");
    let named = leaf("second");
    let identity = ObjectIdentity::new("test", "Container")
        .with_type_arguments(vec![positional.clone()])
        .with_native_parameters(vec!["T".to_owned()])
        .with_native_arguments(BTreeMap::from([("T".to_owned(), named.clone())]));
    let encoded = serde_json::to_vec(&identity).unwrap();
    let restored: ObjectIdentity = serde_json::from_slice(&encoded).unwrap();

    assert_eq!(restored.type_arguments, vec![positional]);
    assert_eq!(restored.native_arguments["T"], named);
    assert_ne!(restored.native_arguments["T"], restored.type_arguments[0]);
}

#[test]
fn serde_preserves_native_metadata() {
    let positional = ObjectIdentity::new("test", "argument")
        .with_native_parameters(vec!["positional".to_owned()]);
    let named = positional
        .clone()
        .with_native_parameters(vec!["named".to_owned()]);
    assert_eq!(positional, named);
    let identity = ObjectIdentity::new("test", "Container")
        .with_type_arguments(vec![ValueType::Object(positional)])
        .with_native_parameters(vec!["T".to_owned()])
        .with_native_arguments(BTreeMap::from([("T".to_owned(), ValueType::Object(named))]));
    let restored: ObjectIdentity =
        serde_json::from_slice(&serde_json::to_vec(&identity).unwrap()).unwrap();
    let ValueType::Object(positional) = &restored.type_arguments[0] else {
        panic!("positional object was not preserved");
    };
    let ValueType::Object(named) = &restored.native_arguments["T"] else {
        panic!("named object was not preserved");
    };
    assert_eq!(restored.native_parameters, vec!["T"]);
    assert_eq!(positional.native_parameters, vec!["positional"]);
    assert_eq!(named.native_parameters, vec!["named"]);
}

#[test]
fn clone_mutation_is_copy_on_write() {
    let original =
        ObjectIdentity::new("test", "Container").with_type_arguments(vec![leaf("original")]);
    let mut changed = original.clone();
    changed.name = "Changed".to_owned();
    changed.type_arguments = vec![leaf("changed")];

    assert_eq!(original.name, "Container");
    assert_eq!(original.type_arguments, vec![leaf("original")]);
    assert_eq!(changed.name, "Changed");
    assert_eq!(changed.type_arguments, vec![leaf("changed")]);
}

#[test]
fn serde_rejects_bad_native_reference() {
    let encoded = serde_json::json!({
        "namespace": "test",
        "name": "Container",
        "is_unsafe": false,
        "application": null,
        "application_key": null,
        "type_arguments": [],
        "type_arguments_key": null,
        "native_projection": null,
        "native_arguments": { "T": { "Positional": 0 } },
        "native_arguments_key": null,
        "native_parameters": ["T"]
    });

    assert!(serde_json::from_value::<ObjectIdentity>(encoded).is_err());
}

#[test]
fn keys_and_wire_ignore_sharing_history() {
    let first = leaf("argument");
    let shared = ObjectIdentity::new("test", "Container")
        .with_type_arguments(vec![first.clone()])
        .with_native_arguments(BTreeMap::from([("T".to_owned(), first)]));
    let separate = ObjectIdentity::new("test", "Container")
        .with_type_arguments(vec![leaf("argument")])
        .with_native_arguments(BTreeMap::from([("T".to_owned(), leaf("argument"))]));
    assert_eq!(shared, separate);
    assert_eq!(format!("{shared:?}"), format!("{separate:?}"));
    assert_eq!(
        serde_json::to_vec(&shared).unwrap(),
        serde_json::to_vec(&separate).unwrap()
    );
    let outer = |inner| {
        ObjectIdentity::new("test", "Outer").with_type_arguments(vec![ValueType::Object(inner)])
    };
    assert_eq!(outer(shared), outer(separate));
}

#[test]
fn wrapped_native_arguments_stay_bounded() {
    let mut child = leaf("leaf");
    for index in 0..DEPTH {
        let wrapped = ValueType::List(ElementType::new(child));
        let identity = ObjectIdentity::new("test", format!("Layer{index}"))
            .with_type_arguments(vec![wrapped.clone()])
            .with_native_arguments(BTreeMap::from([("T".to_owned(), wrapped)]));
        let encoded = serde_json::to_value(&identity).unwrap();
        assert!(serde_json::to_vec(&encoded).unwrap().len() <= MAX_LAYER_BYTES);
        let restored: ObjectIdentity = serde_json::from_value(encoded).unwrap();
        assert_eq!(restored, identity);
        child = ValueType::Object(identity);
    }
}

#[test]
fn walk_keeps_hidden_selection() {
    let hidden = ValueType::TypeParameter("Hidden".to_owned());
    let mut child = ValueType::Object(
        ObjectIdentity::new("test", "Leaf")
            .with_native_arguments(BTreeMap::from([("Hidden".to_owned(), hidden.clone())])),
    );
    for _ in 0..DEPTH {
        child = ValueType::Object(
            ObjectIdentity::new("test", "Layer")
                .with_type_arguments(vec![child.clone()])
                .with_native_arguments(BTreeMap::from([("T".to_owned(), child)])),
        );
    }
    let mut visits = 0;
    let mut found_hidden = false;
    super::super::capabilities::walk_value_type(&child, &mut |value| {
        visits += 1;
        assert!(
            visits <= DEPTH + 2,
            "mirrored native arguments were revisited"
        );
        found_hidden |= value == &hidden;
    });
    assert!(found_hidden, "hidden native type parameter was omitted");
}
