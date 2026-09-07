use super::prelude::*;

const DOCUMENT_DECODABLE_NAMESPACE: &str = "/core/documents";

fn supported_document_field(
    unit: &SemanticUnit,
    value_type: &ValueType,
    visiting: &mut BTreeSet<String>,
) -> bool {
    match value_type {
        ValueType::Scalar(scalar) => {
            matches!(
                scalar,
                ScalarType::Bool | ScalarType::String | ScalarType::Float32 | ScalarType::Float64
            ) || scalar.is_integer()
        }
        ValueType::Optional(inner) => supported_document_field(unit, inner, visiting),
        ValueType::List(item) | ValueType::Tuple(item, _) => {
            supported_document_field(unit, item.value_type_ref(), visiting)
        }
        ValueType::Map(key, value)
            if key.value_type_ref() == &ValueType::Scalar(ScalarType::String) =>
        {
            supported_document_field(unit, value.value_type_ref(), visiting)
        }
        ValueType::Object(identity) => {
            let identity_key = format!("{}::{}", identity.namespace, identity.name);
            if !visiting.insert(identity_key.clone()) {
                return false;
            }
            let supported = unit
                .objects
                .iter()
                .find(|object| object.identity == *identity)
                .is_some_and(|object| {
                    object.interfaces.iter().any(|interface| {
                        interface
                            == &ObjectIdentity::new(
                                DOCUMENT_DECODABLE_NAMESPACE,
                                "document-decodable",
                            )
                    }) && object.base.is_none()
                        && !object.resource_owning
                        && object
                            .fields
                            .iter()
                            .filter(|field| !field.is_static)
                            .all(|field| {
                                supported_document_field(unit, &field.value_type, visiting)
                            })
                });
            visiting.remove(&identity_key);
            supported
        }
        _ => false,
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "typed decode validation keeps the complete call and destination contract together"
)]
pub(super) fn infer_typed_document_decode(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let [callee, arguments] = node.children.as_slice() else {
        return Ok(None);
    };
    let Some(identity) = resolved_compiler_identity(unit, callee) else {
        return Ok(None);
    };
    let expected_options = match identity {
        "/core/documents/json::decode-typed-json" => {
            ObjectIdentity::new("/core/documents/json", "json-options")
        }
        "/core/documents/yaml::decode-typed-yaml" => {
            ObjectIdentity::new("/core/documents/yaml", "yaml-options")
        }
        _ => return Ok(None),
    };
    let values = arguments
        .children
        .iter()
        .map(|argument| argument.children.last().unwrap_or(argument))
        .collect::<Vec<_>>();
    let [source, descriptor, options, allow_unknown] = values.as_slice() else {
        return Err(failure(
            &unit.source,
            "T0114",
            "typed document decoding requires source, destination descriptor, parser options, and allow-unknown",
            node.span,
        ));
    };
    if infer_value_type(unit, source, bindings)? != Some(ValueType::Scalar(ScalarType::String)) {
        return Err(failure(
            &unit.source,
            "T0114",
            "typed document decoding requires a string source",
            source.span,
        ));
    }
    if infer_value_type(unit, options, bindings)? != Some(ValueType::Object(expected_options)) {
        return Err(failure(
            &unit.source,
            "T0114",
            "typed document decoding received incompatible parser options",
            options.span,
        ));
    }
    if infer_value_type(unit, allow_unknown, bindings)? != Some(ValueType::Scalar(ScalarType::Bool))
    {
        return Err(failure(
            &unit.source,
            "T0114",
            "typed document decoding requires a boolean allow-unknown policy",
            allow_unknown.span,
        ));
    }
    let Some(class_identity) = class_designator_identity(unit, descriptor) else {
        return Err(failure(
            &unit.source,
            "T0114",
            "typed document decoding destination must be a concrete class descriptor",
            descriptor.span,
        ));
    };
    let object = unit
        .objects
        .iter()
        .find(|object| object.identity == class_identity)
        .expect("a class designator resolves to its object contract");
    if !object.interfaces.iter().any(|interface| {
        interface == &ObjectIdentity::new(DOCUMENT_DECODABLE_NAMESPACE, "document-decodable")
    }) {
        return Err(failure(
            &unit.source,
            "T0114",
            format!(
                "class `{}` must explicitly implement `document-decodable`",
                object.name
            ),
            descriptor.span,
        ));
    }
    if object.base.is_some()
        || object.resource_owning
        || unit.functions.iter().any(|function| {
            function.owner.as_deref() == Some(object.name.as_str()) && function.name == "construct"
        })
    {
        return Err(failure(
            &unit.source,
            "T0114",
            format!(
                "class `{}` has custom construction or ownership semantics and must implement `deserializable` manually",
                object.name
            ),
            descriptor.span,
        ));
    }
    let mut visiting = BTreeSet::from([class_identity.to_string()]);
    if let Some(field) = object
        .fields
        .iter()
        .filter(|field| !field.is_static)
        .find(|field| !supported_document_field(unit, &field.value_type, &mut visiting))
    {
        return Err(failure(
            &unit.source,
            "T0114",
            format!(
                "field `{}` has type `{}` which typed document decoding cannot construct",
                field.name, field.value_type
            ),
            field.span,
        ));
    }
    Ok(Some(ValueType::DocumentDecodeOutcome(ElementType::new(
        ValueType::Object(class_identity),
    ))))
}
