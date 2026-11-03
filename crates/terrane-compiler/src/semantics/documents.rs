use super::prelude::*;

const DOCUMENT_DECODABLE_NAMESPACE: &str = "/core/documents";

pub(crate) fn supported_document_field(
    unit: &SemanticUnit,
    value_type: &ValueType,
    visiting: &mut BTreeSet<String>,
) -> bool {
    match value_type {
        ValueType::Scalar(scalar) => {
            matches!(
                scalar,
                ScalarType::None
                    | ScalarType::Bool
                    | ScalarType::String
                    | ScalarType::Float32
                    | ScalarType::Float64
            ) || scalar.is_integer()
        }
        ValueType::Optional(inner) => supported_document_field(unit, inner, visiting),
        ValueType::List(item) | ValueType::Tuple(item, _) => {
            supported_document_field(unit, item.value_type_ref(), visiting)
        }
        ValueType::Map(key, value) | ValueType::UnorderedMap(key, value)
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
                .descriptors
                .iter()
                .find(|object| object.identity.base() == identity.base())
                .is_some_and(|object| {
                    object.interfaces.iter().any(|interface| {
                        interface
                            == &ObjectIdentity::new(
                                DOCUMENT_DECODABLE_NAMESPACE,
                                "document-decodable",
                            )
                    }) && object.base.is_none()
                        && !object.resource_owning
                        && !unit
                            .descriptors
                            .iter()
                            .any(|candidate| candidate.base.as_ref() == Some(&object.identity))
                        && !unit.functions.iter().any(|function| {
                            function.owner_identity.as_ref() == Some(&object.identity)
                                && function.name == "construct"
                        })
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

pub(crate) fn supported_document_encodable_field(
    unit: &SemanticUnit,
    value_type: &ValueType,
    visiting: &mut BTreeSet<String>,
) -> bool {
    match value_type {
        ValueType::Scalar(scalar) => {
            matches!(
                scalar,
                ScalarType::None
                    | ScalarType::Bool
                    | ScalarType::String
                    | ScalarType::Float32
                    | ScalarType::Float64
            ) || scalar.is_integer()
        }
        ValueType::Optional(inner) => supported_document_encodable_field(unit, inner, visiting),
        ValueType::List(item) | ValueType::Tuple(item, _) => {
            supported_document_encodable_field(unit, item.value_type_ref(), visiting)
        }
        ValueType::Map(key, value) | ValueType::UnorderedMap(key, value)
            if key.value_type_ref() == &ValueType::Scalar(ScalarType::String) =>
        {
            supported_document_encodable_field(unit, value.value_type_ref(), visiting)
        }
        ValueType::Object(identity) => {
            let key = format!("{}::{}", identity.namespace, identity.name);
            if !visiting.insert(key.clone()) {
                return false;
            }
            let supported = unit
                .descriptors
                .iter()
                .find(|object| object.identity.base() == identity.base())
                .is_some_and(|object| {
                    if object.interfaces.iter().any(|interface| {
                        interface
                            == &ObjectIdentity::new(DOCUMENT_DECODABLE_NAMESPACE, "serializable")
                    }) {
                        return !object.resource_owning
                            && object.base.is_none()
                            && !unit.descriptors.iter().any(|candidate| {
                                candidate.base.as_ref() == Some(&object.identity)
                            });
                    }
                    object.interfaces.iter().any(|interface| {
                        interface
                            == &ObjectIdentity::new(
                                DOCUMENT_DECODABLE_NAMESPACE,
                                "document-encodable",
                            )
                    }) && object.base.is_none()
                        && !object.resource_owning
                        && !unit
                            .descriptors
                            .iter()
                            .any(|candidate| candidate.base.as_ref() == Some(&object.identity))
                        && object
                            .fields
                            .iter()
                            .filter(|field| !field.is_static)
                            .all(|field| {
                                !field.metadata.secret
                                    && field.metadata.visibility == "public"
                                    && supported_document_encodable_field(
                                        unit,
                                        &field.value_type,
                                        visiting,
                                    )
                            })
                });
            visiting.remove(&key);
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
        let mut destination = infer_value_type(unit, descriptor, bindings)?.ok_or_else(|| {
            failure(&unit.source, "T0114", "typed document decoding requires a destination descriptor or a typed fallback value", descriptor.span)
        })?;
        if let ValueType::Descriptor(name) = &destination {
            destination = ScalarType::from_source_name(name)
                .map(ValueType::Scalar)
                .ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0114",
                        "unsupported document destination descriptor",
                        descriptor.span,
                    )
                })?;
        }
        if !supported_document_field(unit, &destination, &mut BTreeSet::new()) {
            return Err(failure(
                &unit.source,
                "T0114",
                format!("typed document decoding cannot construct `{destination}`"),
                descriptor.span,
            ));
        }
        return Ok(Some(ValueType::DocumentDecodeOutcome(ElementType::new(
            destination,
        ))));
    };
    let object = unit
        .descriptors
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
        || unit
            .descriptors
            .iter()
            .any(|candidate| candidate.base.as_ref() == Some(&object.identity))
        || unit.functions.iter().any(|function| {
            function.owner_identity.as_ref() == Some(&object.identity)
                && function.name == "construct"
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

pub(super) fn infer_typed_document_encode(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(source) = typed_document_encode_source(unit, node, bindings)? else {
        return Ok(None);
    };
    let source_type = infer_value_type(unit, source, bindings)?.ok_or_else(|| {
        failure(
            &unit.source,
            "T0114",
            "typed document encoding requires a supported value",
            source.span,
        )
    })?;
    if !matches!(source_type, ValueType::Object(_)) {
        if !supported_document_encodable_field(unit, &source_type, &mut BTreeSet::new()) {
            return Err(failure(
                &unit.source,
                "T0114",
                format!("typed document encoding does not support `{source_type}`"),
                source.span,
            ));
        }
        return Ok(Some(ValueType::DocumentDecodeOutcome(ElementType::new(
            ValueType::Scalar(ScalarType::String),
        ))));
    }
    let ValueType::Object(class_identity) = source_type else {
        unreachable!()
    };
    let object = unit
        .descriptors
        .iter()
        .find(|object| object.identity.base() == class_identity.base())
        .expect("a class value resolves to its object contract");
    if object.interfaces.iter().any(|interface| {
        interface == &ObjectIdentity::new(DOCUMENT_DECODABLE_NAMESPACE, "serializable")
    }) && !object.resource_owning
        && object.base.is_none()
        && !unit
            .descriptors
            .iter()
            .any(|candidate| candidate.base.as_ref() == Some(&object.identity))
    {
        return Ok(Some(ValueType::DocumentDecodeOutcome(ElementType::new(
            ValueType::Scalar(ScalarType::String),
        ))));
    }
    if !object.interfaces.iter().any(|interface| {
        interface == &ObjectIdentity::new(DOCUMENT_DECODABLE_NAMESPACE, "document-encodable")
    }) {
        return Err(failure(
            &unit.source,
            "T0114",
            format!(
                "class `{}` must explicitly implement `document-encodable`",
                object.name
            ),
            source.span,
        ));
    }
    if object.base.is_some()
        || object.resource_owning
        || unit
            .descriptors
            .iter()
            .any(|candidate| candidate.base.as_ref() == Some(&object.identity))
    {
        return Err(failure(
            &unit.source,
            "T0114",
            "derived document encoding requires a non-owning class without inheritance; use an explicit serializable codec",
            source.span,
        ));
    }
    if let Some(field) = object
        .fields
        .iter()
        .filter(|field| !field.is_static)
        .find(|field| {
            field.metadata.secret
                || field.metadata.visibility != "public"
                || !supported_document_encodable_field(
                    unit,
                    &field.value_type,
                    &mut BTreeSet::new(),
                )
        })
    {
        return Err(failure(
            &unit.source,
            "T0114",
            format!(
                "field `{}` is secret or has unsupported type `{}` and cannot be document-encoded",
                field.name, field.value_type
            ),
            field.span,
        ));
    }
    Ok(Some(ValueType::DocumentDecodeOutcome(ElementType::new(
        ValueType::Scalar(ScalarType::String),
    ))))
}

fn typed_document_encode_source<'a>(
    unit: &SemanticUnit,
    node: &'a SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<&'a SyntaxNode>, SemanticFailure> {
    let [callee, arguments] = node.children.as_slice() else {
        return Ok(None);
    };
    let Some(identity) = resolved_compiler_identity(unit, callee) else {
        return Ok(None);
    };
    let options_type = match identity {
        "/core/documents/json::encode-typed-json" => {
            ObjectIdentity::new("/core/documents/json", "json-options")
        }
        "/core/documents/yaml::encode-typed-yaml" => {
            ObjectIdentity::new("/core/documents/yaml", "yaml-options")
        }
        _ => return Ok(None),
    };
    let [source, options] = arguments.children.as_slice() else {
        return Err(failure(
            &unit.source,
            "T0114",
            "typed document encoding requires a value and encoder options",
            node.span,
        ));
    };
    let source = source.children.last().unwrap_or(source);
    let options = options.children.last().unwrap_or(options);
    if infer_value_type(unit, options, bindings)? != Some(ValueType::Object(options_type)) {
        return Err(failure(
            &unit.source,
            "T0114",
            "typed document encoding received incompatible options",
            options.span,
        ));
    }
    Ok(Some(source))
}

/// Derived codecs cannot slice a base class, including subclasses in another unit.
pub(crate) fn polymorphic_document_type(
    package: &SemanticPackage,
    value_type: &ValueType,
    visiting: &mut BTreeSet<String>,
    custom_encoding: bool,
) -> bool {
    match value_type {
        ValueType::Optional(inner) => {
            polymorphic_document_type(package, inner, visiting, custom_encoding)
        }
        ValueType::List(item)
        | ValueType::Tuple(item, _)
        | ValueType::Map(_, item)
        | ValueType::UnorderedMap(_, item) => {
            polymorphic_document_type(package, item.value_type_ref(), visiting, custom_encoding)
        }
        ValueType::Object(identity) => {
            let descriptors = || package.units.iter().flat_map(|unit| &unit.descriptors);
            let object = descriptors().find(|object| object.identity == *identity);
            if custom_encoding
                && object.is_some_and(|object| {
                    object.interfaces.iter().any(|interface| {
                        interface.namespace == "/core/documents" && interface.name == "serializable"
                    })
                })
            {
                // Explicit encoding defines its own representation; do not apply
                // derived field slicing policy to it or its unencoded fields.
                return false;
            }
            if !visiting.insert(identity.to_string()) {
                return false;
            }
            let result = descriptors().any(|candidate| {
                candidate.base.as_ref().is_some_and(|base| {
                    base.namespace == identity.namespace && base.name == identity.name
                })
            }) || object.is_some_and(|object| {
                object
                    .fields
                    .iter()
                    .filter(|field| !field.is_static)
                    .any(|field| {
                        polymorphic_document_type(
                            package,
                            &field.value_type,
                            visiting,
                            custom_encoding,
                        )
                    })
            });
            visiting.remove(&identity.to_string());
            result
        }
        _ => false,
    }
}

pub(super) fn validate_package_document_codecs(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && let Some(identity) = resolved_compiler_identity(unit, callee)
        {
            let (argument, custom_encoding) = match identity {
                "/core/documents/json::encode-typed-json"
                | "/core/documents/yaml::encode-typed-yaml" => (arguments.children.first(), true),
                "/core/documents/json::decode-typed-json"
                | "/core/documents/yaml::decode-typed-yaml" => (arguments.children.get(1), false),
                _ => (None, false),
            };
            if let Some(argument) = argument {
                let value = argument.children.last().unwrap_or(argument);
                let value_type = class_designator_identity(unit, value)
                    .map(ValueType::Object)
                    .or(infer_value_type(unit, value, &unit.typed_bindings)?);
                if value_type.as_ref().is_some_and(|value_type| {
                    polymorphic_document_type(
                        package,
                        value_type,
                        &mut BTreeSet::new(),
                        custom_encoding,
                    )
                }) {
                    return Err(failure(
                        &unit.source,
                        "T0114",
                        "derived document codecs exclude inheritance across all package units; use an explicit codec",
                        value.span,
                    ));
                }
            }
        }
        for child in &node.children {
            visit(package, unit, child)?;
        }
        Ok(())
    }
    for unit in &package.units {
        visit(package, unit, &unit.tree.root)?;
    }
    Ok(())
}
