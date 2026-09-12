use super::prelude::*;

pub(super) fn element_type(
    unit: &SemanticUnit,
    value: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<ElementType, SemanticFailure> {
    infer_value_type(unit, value, bindings)?
        .map(ElementType::new)
        .ok_or_else(|| {
            failure(
                &unit.source,
                "T0042",
                "collection items require a statically known value type",
                value.span,
            )
        })
}

pub(super) fn homogeneous_element_type(
    unit: &SemanticUnit,
    arguments: &SyntaxNode,
    bindings: &[TypedBinding],
    collection: &str,
) -> Result<ElementType, SemanticFailure> {
    let mut item_type = None;
    for argument in &arguments.children {
        let value = argument.children.last().unwrap_or(argument);
        let inferred = ElementType::new(erase_tuple_lengths(
            element_type(unit, value, bindings)?.value_type(),
        ));
        if item_type.is_some_and(|existing| existing != inferred) {
            return Err(failure(
                &unit.source,
                "T0042",
                format!("`{collection}` items must have one statically known type"),
                value.span,
            ));
        }
        item_type = Some(inferred);
    }
    item_type.ok_or_else(|| {
        failure(
            &unit.source,
            "T0043",
            format!("an empty `{collection}` requires an explicit item type"),
            arguments.span,
        )
    })
}

pub(super) fn collection_constructor_matches(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    expected: &ValueType,
    bindings: &[TypedBinding],
) -> bool {
    let Some(identity) = collection_constructor_identity(unit, node, bindings) else {
        return false;
    };
    let constructor = identity
        .strip_prefix("/core/collections::")
        .unwrap_or(identity);
    matches!(
        (constructor, expected),
        ("list", ValueType::List(_))
            | ("map", ValueType::Map(_, _))
            | ("unordered-map", ValueType::UnorderedMap(_, _))
            | ("set", ValueType::Set(_))
            | ("tuple", ValueType::Tuple(_, _))
            | ("unordered-set", ValueType::UnorderedSet(_))
            | ("entry", ValueType::Entry(_, _))
    )
}

pub(super) fn contextual_collection_constructor_matches(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    expected: &ValueType,
    bindings: &[TypedBinding],
) -> bool {
    if node.kind == SyntaxKind::GroupExpression
        && let [grouped] = node.children.as_slice()
    {
        return contextual_collection_constructor_matches(unit, grouped, expected, bindings);
    }
    collection_constructor_matches(unit, node, expected, bindings)
}

pub(super) fn validate_collection_constructor_items(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    expected: &ValueType,
    destination: &str,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let [_, arguments] = node.children.as_slice() else {
        return Ok(());
    };
    match expected {
        ValueType::List(item)
        | ValueType::Tuple(item, _)
        | ValueType::Set(item)
        | ValueType::UnorderedSet(item) => {
            for (index, argument) in arguments.children.iter().enumerate() {
                let value = argument.children.last().unwrap_or(argument);
                validate_collection_constructor_value(
                    unit,
                    value,
                    &item.value_type(),
                    &format!("{destination} item {}", index + 1),
                    bindings,
                )?;
            }
        }
        ValueType::Map(key, value) | ValueType::UnorderedMap(key, value) => {
            let entry_type = ValueType::Entry(key.clone(), value.clone());
            for (index, argument) in arguments.children.iter().enumerate() {
                let label = format!("{destination} entry {}", index + 1);
                if argument.children.len() >= 2 {
                    validate_value_destination(
                        &unit.source,
                        &unit.descriptors,
                        &format!("{label} key"),
                        key.value_type(),
                        ValueType::Scalar(ScalarType::String),
                        argument,
                        "T0002",
                    )?;
                    let entry_value = argument.children.last().unwrap_or(argument);
                    validate_collection_constructor_value(
                        unit,
                        entry_value,
                        &value.value_type(),
                        &format!("{label} value"),
                        bindings,
                    )?;
                } else {
                    let entry = argument.children.last().unwrap_or(argument);
                    validate_collection_constructor_value(
                        unit,
                        entry,
                        &entry_type,
                        &label,
                        bindings,
                    )?;
                }
            }
        }
        ValueType::Entry(key, value) => {
            let [key_argument, value_argument] = arguments.children.as_slice() else {
                return Err(failure(
                    &unit.source,
                    "T0045",
                    "`entry` requires exactly a key and value",
                    arguments.span,
                ));
            };
            let key_node = key_argument.children.last().unwrap_or(key_argument);
            let value_node = value_argument.children.last().unwrap_or(value_argument);
            validate_collection_constructor_value(
                unit,
                key_node,
                &key.value_type(),
                &format!("{destination} key"),
                bindings,
            )?;
            validate_collection_constructor_value(
                unit,
                value_node,
                &value.value_type(),
                &format!("{destination} value"),
                bindings,
            )?;
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn validate_collection_constructor_value(
    unit: &SemanticUnit,
    value: &SyntaxNode,
    expected: &ValueType,
    destination: &str,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    if value.kind == SyntaxKind::GroupExpression
        && let [grouped] = value.children.as_slice()
    {
        return validate_collection_constructor_value(
            unit,
            grouped,
            expected,
            destination,
            bindings,
        );
    }
    if value.kind == SyntaxKind::Name
        && matches!(expected, ValueType::Entry(_, _))
        && collection_constructor_identity(unit, value, bindings).is_some_and(|identity| {
            identity
                .strip_prefix("/core/collections::")
                .unwrap_or(identity)
                == "entry"
        })
    {
        return Err(failure(
            &unit.source,
            "T0045",
            "`entry` requires exactly a key and value",
            value.span,
        ));
    }
    if collection_constructor_matches(unit, value, expected, bindings) {
        return validate_collection_constructor_items(unit, value, expected, destination, bindings);
    }
    if let Some(actual) = infer_value_type(unit, value, bindings)? {
        validate_value_destination(
            &unit.source,
            &unit.descriptors,
            destination,
            expected.clone(),
            actual,
            value,
            "T0002",
        )?;
    }
    Ok(())
}

pub(super) fn empty_collection_identity<'a>(
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    bindings: &[TypedBinding],
) -> Option<&'a str> {
    let is_empty = if node.kind == SyntaxKind::Name {
        true
    } else {
        let [_, arguments] = node.children.as_slice() else {
            return None;
        };
        node.kind == SyntaxKind::CallExpression && arguments.children.is_empty()
    };
    is_empty.then(|| collection_constructor_identity(unit, node, bindings))?
}

pub(super) fn collection_constructor_identity<'a>(
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    bindings: &[TypedBinding],
) -> Option<&'a str> {
    let callee = if node.kind == SyntaxKind::Name {
        node
    } else {
        let [callee, _] = node.children.as_slice() else {
            return None;
        };
        (node.kind == SyntaxKind::CallExpression).then_some(callee)?
    };
    let identity = resolved_compiler_object_identity(unit, callee).or_else(|| {
        let name = node_text(&unit.source, callee);
        (!bindings.iter().rev().any(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), callee.span.start)
        }))
        .then_some(name)
    })?;
    matches!(
        identity
            .strip_prefix("/core/collections::")
            .unwrap_or(identity),
        "list" | "map" | "unordered-map" | "set" | "tuple" | "unordered-set" | "entry"
    )
    .then_some(identity)
}

pub(crate) fn collection_member_call<'a>(
    unit: &SemanticUnit,
    callee: &'a SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<(&'a SyntaxNode, ValueType, String)>, SemanticFailure> {
    let [receiver, member] = callee.children.as_slice() else {
        return Ok(None);
    };
    if callee.kind != SyntaxKind::MemberExpression {
        return Ok(None);
    }
    if receiver.kind == SyntaxKind::MemberExpression
        && let [base, family] = receiver.children.as_slice()
        && let Some(receiver_type) = infer_receiver_value_type(unit, base, bindings)?
        && descriptor_is_collection(unit, &receiver_type)
        && descriptor_has_member(unit, &receiver_type, node_text(&unit.source, family))
    {
        return Ok(Some((
            base,
            receiver_type,
            format!(
                "{}.{}",
                node_text(&unit.source, family),
                node_text(&unit.source, member)
            ),
        )));
    }
    Ok(infer_receiver_value_type(unit, receiver, bindings)?
        .filter(|receiver_type| descriptor_is_collection(unit, receiver_type))
        .map(|receiver_type| {
            (
                receiver,
                receiver_type,
                node_text(&unit.source, member).to_owned(),
            )
        }))
}

fn sortable_list_item(item: &ElementType) -> Option<ScalarType> {
    match item.value_type() {
        ValueType::Scalar(scalar)
            if scalar.is_integer()
                || matches!(
                    scalar,
                    ScalarType::Float32 | ScalarType::Float64 | ScalarType::String
                ) =>
        {
            Some(scalar)
        }
        _ => None,
    }
}

fn validate_list_sort(
    unit: &SemanticUnit,
    item: &ElementType,
    arguments: &SyntaxNode,
    callee_span: Span,
    method: &str,
) -> Result<(), SemanticFailure> {
    if !arguments.children.is_empty() {
        return Err(failure(
            &unit.source,
            "T0045",
            format!("`.{method}` accepts no arguments"),
            arguments.span,
        ));
    }
    sortable_list_item(item).map(|_| ()).ok_or_else(|| {
        failure(
            &unit.source,
            "T0013",
            format!("`.{method}` requires a list of non-optional integers, floats, or strings"),
            callee_span,
        )
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "collection construction inference centralizes one compiler-owned object family"
)]
pub(super) fn infer_collection_call_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let [callee, arguments] = node.children.as_slice() else {
        return Ok(None);
    };
    if let Some((_receiver, receiver_type, member)) =
        collection_member_call(unit, callee, bindings)?
        && member.contains('.')
    {
        if !descriptor_has_member(unit, &receiver_type, &member) {
            return Err(failure(
                &unit.source,
                "T0031",
                format!("collection has no member `.{member}`"),
                callee.span,
            ));
        }
        if matches!(member.as_str(), "get.checked" | "remove.checked") {
            let [argument] = arguments.children.as_slice() else {
                return Err(failure(
                    &unit.source,
                    "T0045",
                    format!("`.{member}` requires exactly one argument"),
                    arguments.span,
                ));
            };
            let argument = argument.children.last().unwrap_or(argument);
            return match (receiver_type, member.as_str()) {
                (ValueType::List(item) | ValueType::Tuple(item, _), "get.checked") => {
                    validate_collection_constructor_value(
                        unit,
                        argument,
                        &ValueType::Scalar(ScalarType::Int),
                        "collection index",
                        bindings,
                    )?;
                    Ok(Some(ValueType::Optional(Box::new(item.value_type()))))
                }
                (
                    ValueType::Map(key, value) | ValueType::UnorderedMap(key, value),
                    "get.checked" | "remove.checked",
                ) => {
                    validate_collection_constructor_value(
                        unit,
                        argument,
                        &key.value_type(),
                        "map key",
                        bindings,
                    )?;
                    Ok(Some(ValueType::Optional(Box::new(value.value_type()))))
                }
                _ => Ok(None),
            };
        }
        if member == "sort.descending"
            && let ValueType::List(item) = receiver_type
        {
            validate_list_sort(unit, &item, arguments, callee.span, &member)?;
            return Ok(Some(ValueType::List(item)));
        }
    }
    if callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && node_text(&unit.source, member) == "through"
        && (resolved_compiler_object_identity(unit, receiver) == Some("/core/collections::range")
            || (node_text(&unit.source, receiver) == "range"
                && !bindings.iter().rev().any(|binding| {
                    binding.name == "range"
                        && binding.is_visible_at(unit.source.id(), receiver.span.start)
                })))
    {
        return Ok(Some(ValueType::Range));
    }
    if callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && node_text(&unit.source, member) == "end"
        && resolved_compiler_object_identity(unit, receiver)
            == Some("/core/collections::iteration-step")
    {
        if !arguments.children.is_empty() {
            return Err(failure(
                &unit.source,
                "T0045",
                "`iteration-step.end` accepts no arguments",
                arguments.span,
            ));
        }
        return Ok(Some(ValueType::IterationEnd));
    }
    if callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && node_text(&unit.source, member) == "next"
        && let Some(ValueType::Iterator(item)) =
            infer_receiver_value_type(unit, receiver, bindings)?
    {
        if !arguments.children.is_empty() {
            return Err(failure(
                &unit.source,
                "T0045",
                "iterator `.next` accepts no arguments",
                arguments.span,
            ));
        }
        return Ok(Some(ValueType::IterationStep(item)));
    }
    if callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && let Some(receiver_type) = infer_receiver_value_type(unit, receiver, bindings)?
        && descriptor_is_collection(unit, &receiver_type)
    {
        let member = node_text(&unit.source, member);
        let operation = descriptor_operation(unit, &receiver_type, member);
        if operation.is_none() {
            if matches!(receiver_type, ValueType::Tuple(_, _))
                && matches!(member, "append" | "set" | "clear" | "add" | "remove")
            {
                return Err(failure(
                    &unit.source,
                    "T0048",
                    "tuple items and length are fixed at construction",
                    callee.span,
                ));
            }
            return Ok(None);
        }
        if operation == Some("collection.remove")
            && matches!(
                receiver_type,
                ValueType::Map(_, _) | ValueType::UnorderedMap(_, _)
            )
        {
            let [argument] = arguments.children.as_slice() else {
                return Err(failure(
                    &unit.source,
                    "T0045",
                    "map `.remove` requires exactly one key",
                    arguments.span,
                ));
            };
            let (ValueType::Map(key, _) | ValueType::UnorderedMap(key, _)) = &receiver_type else {
                unreachable!("map removal receiver was validated");
            };
            validate_collection_constructor_value(
                unit,
                argument.children.last().unwrap_or(argument),
                &key.value_type(),
                "map removal key",
                bindings,
            )?;
        }
        if operation == Some("collection.sort")
            && let ValueType::List(item) = &receiver_type
        {
            validate_list_sort(unit, item, arguments, callee.span, "sort")?;
        }
        return Ok(
            match (receiver_type, operation.expect("validated operation")) {
                (
                    ValueType::List(item),
                    "collection.append" | "collection.set" | "collection.clear" | "collection.sort",
                ) => Some(ValueType::List(item)),
                (ValueType::List(item), "collection.remove") => Some(item.value_type()),
                (ValueType::Map(key, value), "collection.set") => Some(ValueType::Map(key, value)),
                (ValueType::UnorderedMap(key, value), "collection.set") => {
                    Some(ValueType::UnorderedMap(key, value))
                }
                (
                    ValueType::Map(_, value) | ValueType::UnorderedMap(_, value),
                    "collection.remove",
                ) => Some(value.value_type()),
                (ValueType::Set(item), "collection.add") => Some(ValueType::Set(item)),
                (ValueType::UnorderedSet(item), "collection.add") => {
                    Some(ValueType::UnorderedSet(item))
                }
                (
                    ValueType::Set(_) | ValueType::UnorderedSet(_),
                    "collection.contains" | "collection.remove",
                ) => Some(ValueType::Scalar(ScalarType::Bool)),
                (ValueType::Map(key, _) | ValueType::UnorderedMap(key, _), "collection.keys") => {
                    Some(ValueType::List(key))
                }
                (
                    ValueType::Map(_, value) | ValueType::UnorderedMap(_, value),
                    "collection.values",
                ) => Some(ValueType::List(value)),
                (
                    ValueType::Map(key, value) | ValueType::UnorderedMap(key, value),
                    "collection.entries",
                ) => Some(ValueType::List(ElementType::new(ValueType::Entry(
                    key, value,
                )))),
                _ => None,
            },
        );
    }
    let identity = resolved_compiler_object_identity(unit, callee);
    let source_name = node_text(&unit.source, callee);
    let name = identity
        .and_then(|identity| identity.strip_prefix("/core/collections::"))
        .unwrap_or(source_name);
    let shadowed = bindings.iter().rev().any(|binding| {
        binding.name == source_name && binding.is_visible_at(unit.source.id(), callee.span.start)
    });
    if shadowed {
        return Ok(None);
    }
    if let Some(expected) = bindings
        .iter()
        .filter(|binding| {
            binding.span.start <= node.span.start && node.span.end <= binding.span.end
        })
        .min_by_key(|binding| binding.span.end - binding.span.start)
        .map(|binding| &binding.value_type)
        && collection_constructor_matches(unit, node, expected, bindings)
    {
        return Ok(Some(expected.clone()));
    }
    let result = match name {
        "list" => ValueType::List(homogeneous_element_type(unit, arguments, bindings, name)?),
        "tuple" => ValueType::Tuple(
            homogeneous_element_type(unit, arguments, bindings, name)?,
            Some(arguments.children.len()),
        ),
        "set" => {
            let Some(item) = homogeneous_element_type(unit, arguments, bindings, name)?.scalar()
            else {
                return Err(failure(
                    &unit.source,
                    "T0044",
                    "set keys must be immutable scalar values",
                    arguments.span,
                ));
            };
            ValueType::Set(ElementType::new(ValueType::Scalar(item)))
        }
        "unordered-set" => {
            let Some(item) = homogeneous_element_type(unit, arguments, bindings, name)?.scalar()
            else {
                return Err(failure(
                    &unit.source,
                    "T0044",
                    "unordered-set keys must be immutable scalar values",
                    arguments.span,
                ));
            };
            ValueType::UnorderedSet(ElementType::new(ValueType::Scalar(item)))
        }
        "entry" => {
            let [key, value] = arguments.children.as_slice() else {
                return Err(failure(
                    &unit.source,
                    "T0045",
                    "`entry` requires exactly a key and value",
                    arguments.span,
                ));
            };
            let Some(key) =
                element_type(unit, key.children.last().unwrap_or(key), bindings)?.scalar()
            else {
                return Err(failure(
                    &unit.source,
                    "T0044",
                    "entry keys must be immutable scalar values",
                    key.span,
                ));
            };
            let value = element_type(unit, value.children.last().unwrap_or(value), bindings)?;
            ValueType::Entry(ElementType::new(ValueType::Scalar(key)), value)
        }
        "map" | "unordered-map" => {
            let mut key_type = None;
            let mut value_type = None;
            for argument in &arguments.children {
                let value_node = argument.children.last().unwrap_or(argument);
                let inferred = element_type(unit, value_node, bindings)?;
                let (key, value) = match inferred.value_type() {
                    ValueType::Entry(key, value) if argument.children.len() < 2 => (key, value),
                    _ => (
                        ElementType::new(ValueType::Scalar(ScalarType::String)),
                        inferred,
                    ),
                };
                if key_type.as_ref().is_some_and(|existing| existing != &key) {
                    return Err(failure(
                        &unit.source,
                        "T0042",
                        "map keys must have one statically known type",
                        value_node.span,
                    ));
                }
                if value_type
                    .as_ref()
                    .is_some_and(|existing| existing != &value)
                {
                    return Err(failure(
                        &unit.source,
                        "T0042",
                        "map values must have one statically known type",
                        value_node.span,
                    ));
                }
                key_type = Some(key);
                value_type = Some(value);
            }
            let key = key_type.ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0043",
                    "an empty map requires explicit key and value types",
                    arguments.span,
                )
            })?;
            let value = value_type.expect("a map key and value are inferred together");
            if name == "map" {
                ValueType::Map(key, value)
            } else {
                ValueType::UnorderedMap(key, value)
            }
        }
        "iteration-step" => {
            let [argument] = arguments.children.as_slice() else {
                return Err(failure(
                    &unit.source,
                    "T0045",
                    "`iteration-step` requires exactly one item",
                    arguments.span,
                ));
            };
            ValueType::IterationStep(element_type(
                unit,
                argument.children.last().unwrap_or(argument),
                bindings,
            )?)
        }
        "range" => ValueType::Range,
        _ => return Ok(None),
    };
    Ok(Some(result))
}

pub(super) fn resolved_compiler_object_identity<'a>(
    unit: &'a SemanticUnit,
    node: &SyntaxNode,
) -> Option<&'a str> {
    let name = (node.kind == SyntaxKind::Name).then(|| node_text(&unit.source, node))?;
    lexical_scope_chain(unit, node.span.start).find_map(|scope| {
        scope.symbols.get(name)?.iter().rev().find_map(|symbol| {
            symbol
                .declaration_span
                .is_none_or(|span| span.end <= node.span.start)
                .then_some(symbol.identity.as_str())
        })
    })
}

pub(super) fn infer_iterator_call_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let [callee, arguments] = node.children.as_slice() else {
        return Ok(None);
    };
    let shadowed = bindings.iter().rev().any(|binding| {
        binding.name == node_text(&unit.source, callee)
            && binding.is_visible_at(unit.source.id(), callee.span.start)
    });
    if resolved_compiler_object_identity(unit, callee) != Some("/core/collections::iterator")
        && (node_text(&unit.source, callee) != "iterator" || shadowed)
    {
        return Ok(None);
    }
    let mut item_type = None;
    for argument in &arguments.children {
        let value = argument.children.last().unwrap_or(argument);
        let inferred = infer_value_type(unit, value, bindings)?.ok_or_else(|| {
            failure(
                &unit.source,
                "T0041",
                "iterator items require a statically known value type",
                value.span,
            )
        })?;
        let element = match inferred {
            ValueType::Scalar(ty) => ElementType::new(ValueType::Scalar(ty)),
            ValueType::TextRange => ElementType::new(ValueType::TextRange),
            other => {
                return Err(failure(
                    &unit.source,
                    "T0041",
                    format!("`iterator` cannot contain `{other}` values"),
                    value.span,
                ));
            }
        };
        if item_type.is_some_and(|existing| existing != element) {
            return Err(failure(
                &unit.source,
                "T0041",
                "iterator items must have one statically known type",
                value.span,
            ));
        }
        item_type = Some(element);
    }
    item_type.map(ValueType::Iterator).map(Some).ok_or_else(|| {
        failure(
            &unit.source,
            "T0041",
            "an empty iterator requires an explicit item type",
            node.span,
        )
    })
}
