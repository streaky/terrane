use super::prelude::*;

pub(super) fn text_range_member_type(member_name: &str) -> Option<ValueType> {
    match member_name {
        "text" => Some(ValueType::Scalar(ScalarType::String)),
        "bytes" => Some(ValueType::TextRangeView(TextUnit::Bytes)),
        "scalars" => Some(ValueType::TextRangeView(TextUnit::Scalars)),
        "graphemes" => Some(ValueType::TextRangeView(TextUnit::Graphemes)),
        _ => None,
    }
}

pub(super) fn object_contract<'a>(
    unit: &'a SemanticUnit,
    identity: &ObjectIdentity,
) -> Option<&'a ObjectContract> {
    unit.objects
        .iter()
        .find(|object| object.identity == *identity)
}

pub(super) fn object_method_contract<'a>(
    unit: &'a SemanticUnit,
    object_identity: &ObjectIdentity,
    member: &str,
    is_static: bool,
) -> Option<&'a FunctionContract> {
    let object = object_contract(unit, object_identity)?;
    unit.functions
        .iter()
        .find(|function| {
            function.owner.as_deref() == Some(object.identity.name.as_str())
                && function.name == member
                && function.is_static == is_static
        })
        .or_else(|| {
            object
                .base
                .as_ref()
                .and_then(|base| object_method_contract(unit, base, member, is_static))
        })
}

pub(super) fn object_member_type(
    unit: &SemanticUnit,
    object_identity: &ObjectIdentity,
    member: &str,
    is_static: bool,
) -> Option<ValueType> {
    let object = object_contract(unit, object_identity)?;
    if let Some(field) = object
        .fields
        .iter()
        .find(|field| field.name == member && field.is_static == is_static)
    {
        return Some(field.value_type.clone());
    }
    if let Some(method) = object_method_contract(unit, object_identity, member, is_static) {
        let parameters = method
            .parameters
            .iter()
            .map(|parameter| parameter.value_type.clone().map(ElementType::new))
            .collect::<Option<Vec<_>>>()?;
        let result = ElementType::new(
            method
                .return_type
                .clone()
                .unwrap_or(ValueType::Scalar(ScalarType::None)),
        );
        return Some(if method.is_async {
            ValueType::AsyncFunction(parameters, result)
        } else {
            ValueType::Function(parameters, result)
        });
    }
    for used_trait in &object.traits {
        if let Some(trait_object) = unit.objects.iter().find(|candidate| {
            candidate.identity == *used_trait && candidate.kind == ObjectKind::Trait
        }) && let Some(found) =
            object_member_type(unit, &trait_object.identity, member, is_static)
        {
            return Some(found);
        }
    }
    object.base.as_ref().and_then(|base| {
        unit.objects
            .iter()
            .find(|candidate| candidate.identity == *base)
            .and_then(|base| object_member_type(unit, &base.identity, member, is_static))
    })
}

pub(super) fn infer_receiver_value_type(
    unit: &SemanticUnit,
    receiver: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    Ok(
        infer_value_type(unit, receiver, bindings)?.map(|value_type| match value_type {
            ValueType::Reference(item) | ValueType::SharedReference(item) => item.value_type(),
            value_type => value_type,
        }),
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "member inference keeps receiver precedence and diagnostics in one ordered dispatch"
)]
pub(super) fn infer_member_value_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let [receiver, member] = node.children.as_slice() else {
        return Ok(None);
    };
    let member_name = node_text(&unit.source, member);
    let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
    if let Some(ValueType::Descriptor(_)) = &receiver_type {
        return match member_name {
            "name" | "kind" | "identity" => Ok(Some(ValueType::Scalar(ScalarType::String))),
            _ => Err(failure(
                &unit.source,
                "T0071",
                format!("descriptor has no retained member `{member_name}`"),
                member.span,
            )),
        };
    }
    if matches!(
        receiver_type,
        Some(ValueType::Function(_, _) | ValueType::AsyncFunction(_, _))
    ) && matches!(
        member_name,
        "contracts" | "throwable-contract" | "escaping-throwables"
    ) {
        return Ok(Some(ValueType::Scalar(ScalarType::String)));
    }
    if let Some(result) = &receiver_type
        && matches!(
            result,
            ValueType::PlatformOpenResult
                | ValueType::PlatformReadResult
                | ValueType::PlatformWriteResult
                | ValueType::PlatformUnitResult
        )
    {
        let member_type = match (result, member_name) {
            (ValueType::PlatformOpenResult, "handle") => Some(ValueType::PlatformStreamHandle),
            (ValueType::PlatformReadResult, "data") => Some(ValueType::Scalar(ScalarType::Bytes)),
            (ValueType::PlatformReadResult | ValueType::PlatformWriteResult, "completed") => {
                Some(ValueType::Scalar(ScalarType::Int))
            }
            (ValueType::PlatformReadResult, "end")
            | (
                ValueType::PlatformOpenResult
                | ValueType::PlatformReadResult
                | ValueType::PlatformWriteResult
                | ValueType::PlatformUnitResult,
                "failed",
            ) => Some(ValueType::Scalar(ScalarType::Bool)),
            (
                ValueType::PlatformOpenResult
                | ValueType::PlatformReadResult
                | ValueType::PlatformWriteResult
                | ValueType::PlatformUnitResult,
                "message",
            ) => Some(ValueType::Scalar(ScalarType::String)),
            _ => None,
        };
        return member_type.map(Some).ok_or_else(|| {
            failure(
                &unit.source,
                "T0097",
                format!("`{result}` has no member `{member_name}`"),
                member.span,
            )
        });
    }
    if let Some(ValueType::TaskOutcome(result)) = &receiver_type {
        return match member_name {
            "completed" | "cancelled" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            "value" => Ok(Some(ValueType::Optional(Box::new(result.value_type())))),
            "error" => Ok(Some(ValueType::Optional(Box::new(ValueType::Object(
                ObjectIdentity::new("/core/errors", "TerraneError"),
            ))))),
            _ => Err(failure(
                &unit.source,
                "T0074",
                format!("task outcome has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::Object(object_name)) = &receiver_type {
        return object_member_type(unit, object_name, member_name, false)
            .map(Some)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0055",
                    format!(
                        "`{}` has no instance member `{member_name}`",
                        unit.objects
                            .iter()
                            .find(|object| object.identity == *object_name)
                            .map_or_else(
                                || diagnostic_object_identity(&unit.objects, object_name),
                                |object| object.name.clone()
                            )
                    ),
                    member.span,
                )
            });
    }
    let collection_method = matches!(
        (&receiver_type, member_name),
        (
            Some(ValueType::List(_) | ValueType::Tuple(_, _)),
            "append" | "set" | "get"
        ) | (
            Some(ValueType::Map(_, _) | ValueType::UnorderedMap(_, _)),
            "set" | "get" | "keys" | "values" | "entries"
        ) | (
            Some(ValueType::Set(_) | ValueType::UnorderedSet(_)),
            "add" | "contains" | "remove"
        )
    );
    let string_method = matches!(&receiver_type, Some(ValueType::Scalar(ScalarType::String)))
        && (StringFamily::from_source_name(member_name).is_some()
            || matches!(member_name, "concat" | "join"));
    let bytes_method = matches!(&receiver_type, Some(ValueType::Scalar(ScalarType::Bytes)))
        && matches!(member_name, "decode" | "concat");
    if collection_method || string_method || bytes_method {
        let family = if string_method {
            "string methods"
        } else if bytes_method {
            "bytes methods"
        } else {
            "collection methods"
        };
        return Err(failure(
            &unit.source,
            "T0018",
            format!(
                "{family} are not storable values before bound methods exist; \
                 method `.{member_name}` must be invoked with `;`"
            ),
            node.span,
        ));
    }
    if matches!(
        receiver_type,
        Some(
            ValueType::List(_)
                | ValueType::Map(_, _)
                | ValueType::Set(_)
                | ValueType::Tuple(_, _)
                | ValueType::UnorderedMap(_, _)
                | ValueType::UnorderedSet(_)
        )
    ) && member_name == "length"
    {
        return Ok(Some(ValueType::Scalar(ScalarType::Int)));
    }
    if let Some(ValueType::Entry(key, value)) = receiver_type {
        return Ok(match member_name {
            "key" => Some(key.value_type()),
            "value" => Some(value.value_type()),
            _ => None,
        });
    }
    if receiver_type == Some(ValueType::Scalar(ScalarType::String)) {
        let view = match member_name {
            "bytes" => Some(TextUnit::Bytes),
            "scalars" => Some(TextUnit::Scalars),
            "graphemes" => Some(TextUnit::Graphemes),
            _ => None,
        };
        if let Some(view) = view {
            return Ok(Some(ValueType::StringView(view)));
        }
    }
    if receiver_type == Some(ValueType::TextRange) {
        return Ok(text_range_member_type(member_name));
    }
    if matches!(receiver_type, Some(ValueType::TextRangeView(_)))
        && matches!(member_name, "start" | "end")
    {
        return Ok(Some(ValueType::Scalar(ScalarType::Int)));
    }
    if matches!(
        receiver_type,
        Some(
            ValueType::StringView(_)
                | ValueType::StringList
                | ValueType::TextRangeList
                | ValueType::Scalar(ScalarType::Bytes)
        )
    ) && member_name == "length"
    {
        return Ok(Some(ValueType::Scalar(ScalarType::Int)));
    }
    match (receiver_type.clone(), member_name) {
        (Some(ValueType::OverflowResult(ty)), "value")
        | (Some(ValueType::DivRemResult(ty)), "quotient" | "remainder") => {
            return Ok(Some(ValueType::Scalar(ty)));
        }
        (Some(ValueType::OverflowResult(_)), "overflowed") => {
            return Ok(Some(ValueType::Scalar(ScalarType::Bool)));
        }
        (Some(ValueType::OverflowResult(_) | ValueType::DivRemResult(_)), _) => {
            return Err(failure(
                &unit.source,
                "T0031",
                format!("result object has no member `.{member_name}`"),
                member.span,
            ));
        }
        _ => {}
    }
    if let Some(contract) = float_member_contract(member_name) {
        if let Some(ValueType::Scalar(receiver @ (ScalarType::Float32 | ScalarType::Float64))) =
            receiver_type.clone()
        {
            return Ok(Some(contract.member_type(receiver)));
        }
        return Err(failure(
            &unit.source,
            "T0013",
            format!("`.{member_name}` requires a floating receiver"),
            receiver.span,
        ));
    }
    if member_name == "type" {
        return Ok(None);
    }
    if member_name != "length" {
        return match receiver_type {
            Some(receiver_type) => Err(failure(
                &unit.source,
                "T0031",
                format!("`{receiver_type}` has no member `.{member_name}`"),
                member.span,
            )),
            None => Ok(None),
        };
    }
    if matches!(
        receiver_type,
        Some(ValueType::Scalar(ScalarType::String | ScalarType::Bytes))
    ) {
        return Ok(Some(ValueType::Scalar(ScalarType::Int)));
    }
    let message = receiver_type.map_or_else(
        || {
            "`.length` requires a receiver with a statically known sequence type; \
             add a collection type annotation"
                .to_owned()
        },
        |value_type| {
            format!("`.length` requires `string`, `bytes`, or a collection, found `{value_type}`")
        },
    );
    Err(failure(&unit.source, "T0013", message, receiver.span))
}

pub(super) fn infer_float_call_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(callee) = node.children.first() else {
        return Ok(None);
    };
    let Some([receiver, member]) = (callee.kind == SyntaxKind::MemberExpression)
        .then_some(callee.children.as_slice())
        .and_then(|children| <&[SyntaxNode; 2]>::try_from(children).ok())
    else {
        return Ok(None);
    };
    let member_name = node_text(&unit.source, member);
    let Some(contract) = float_member_contract(member_name) else {
        return Ok(None);
    };
    let Some(expected) = contract.arity else {
        return Ok(None);
    };
    let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
    let Some(ValueType::Scalar(receiver @ (ScalarType::Float32 | ScalarType::Float64))) =
        receiver_type
    else {
        return Err(failure(
            &unit.source,
            "T0013",
            format!("`.{member_name}` requires a floating receiver"),
            receiver.span,
        ));
    };
    let arguments = node.children.get(1);
    let arguments = arguments.map_or(&[][..], |arguments| arguments.children.as_slice());
    if arguments.len() != expected {
        return Err(failure(
            &unit.source,
            "T0023",
            format!(
                "`.{member_name}` requires exactly {expected} argument{}",
                if expected == 1 { "" } else { "s" }
            ),
            node.span,
        ));
    }
    for argument in arguments {
        let value = argument.children.last().unwrap_or(argument);
        if let Some(actual) = infer_value_type(unit, value, bindings)? {
            validate_value_destination(
                &unit.source,
                &unit.objects,
                "floating operation argument",
                ValueType::Scalar(receiver),
                actual,
                value,
                "T0013",
            )?;
        }
    }
    Ok(Some(contract.result_type(receiver)))
}

pub(crate) fn string_call_selection(
    source: &SourceFile,
    node: &SyntaxNode,
) -> Option<StringCallSelection> {
    let callee = node.children.first()?;
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    if callee.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let (receiver_span, family, child) = if receiver.kind == SyntaxKind::MemberExpression
        && let [nested_receiver, nested_family] = receiver.children.as_slice()
        && let Some(candidate) = StringFamily::from_source_name(node_text(source, nested_family))
        && candidate.has_children()
    {
        (
            nested_receiver.span,
            candidate,
            node_text(source, member).to_owned(),
        )
    } else {
        (
            receiver.span,
            StringFamily::from_source_name(node_text(source, member))?,
            "default".to_owned(),
        )
    };
    Some(StringCallSelection {
        receiver: receiver_span,
        family,
        child,
    })
}

#[allow(clippy::too_many_lines)]
pub(super) fn infer_string_call_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(selection) = string_call_selection(&unit.source, node) else {
        return Ok(None);
    };
    let subject = find_node_by_span(&unit.tree.root, selection.receiver)
        .expect("selected string receiver belongs to this syntax tree");
    let family = selection.family.source_name();
    let child = selection.child.as_str();
    let subject_type = transparent_value_type(infer_value_type(unit, subject, bindings)?);
    if matches!(subject_type, Some(ValueType::Object(_))) {
        return Ok(None);
    }
    let receiver_valid = match family {
        "decode" => subject_type == Some(ValueType::Scalar(ScalarType::Bytes)),
        _ => subject_type == Some(ValueType::Scalar(ScalarType::String)),
    };
    if !receiver_valid {
        return Err(failure(
            &unit.source,
            "T0032",
            format!("`.{family}` is not available on this receiver"),
            subject.span,
        ));
    }
    let arguments = node
        .children
        .get(1)
        .map_or(&[][..], |arguments| arguments.children.as_slice());
    let (minimum, maximum) = match (family, child) {
        ("trim", "default") | ("upper" | "lower" | "normalise" | "case-fold", _) => (0, 0),
        ("trim", "start" | "end") => (0, 1),
        ("replace", _) => (2, 2),
        _ => (1, 1),
    };
    if arguments.len() < minimum || arguments.len() > maximum {
        return Err(failure(
            &unit.source,
            "T0023",
            format!("`.{family}` received the wrong number of arguments"),
            node.span,
        ));
    }
    for argument in arguments {
        let argument = argument.children.last().unwrap_or(argument);
        let expected = if matches!(family, "encode" | "decode") {
            ValueType::Encoding
        } else {
            ValueType::Scalar(ScalarType::String)
        };
        if infer_value_type(unit, argument, bindings)? != Some(expected) {
            return Err(failure(
                &unit.source,
                "T0033",
                format!("`.{family}` received an incompatible argument"),
                argument.span,
            ));
        }
    }
    let result = match (family, child) {
        ("contains", "default" | "start" | "end") => ValueType::Scalar(ScalarType::Bool),
        ("find", "default") => ValueType::Optional(Box::new(ValueType::TextRange)),
        ("find", "all") => ValueType::TextRangeList,
        ("find", "count") => ValueType::Scalar(ScalarType::Int),
        ("split", "default") => ValueType::StringList,
        ("encode", "default") => ValueType::Scalar(ScalarType::Bytes),
        ("decode" | "case-fold" | "replace", "default")
        | ("trim", "default" | "start" | "end")
        | ("upper", "default" | "first" | "words")
        | ("lower", "default" | "first")
        | ("normalise", "nfc" | "nfd" | "nfkc" | "nfkd") => ValueType::Scalar(ScalarType::String),
        _ => {
            return Err(failure(
                &unit.source,
                "T0034",
                format!("`.{family}.{child}` is not available"),
                node.span,
            ));
        }
    };
    Ok(Some(result))
}
pub(super) fn infer_unary_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<ValueType, SemanticFailure> {
    let Some(operand_node) = node.children.last() else {
        return Err(operator_failure(
            unit,
            node,
            "unary operator requires an operand",
        ));
    };
    let operator = unary_operator_text(unit, node).unwrap_or_default();
    if operator == "await" {
        return match infer_value_type(unit, operand_node, bindings)? {
            Some(ValueType::Task(result)) => Ok(result.value_type()),
            _ => Err(operator_failure(
                unit,
                node,
                "`await` requires a task value",
            )),
        };
    }
    if matches!(operator.as_str(), "ref" | "shared ref" | "move") {
        let Some(operand) = infer_value_type(unit, operand_node, bindings)? else {
            return Err(operator_failure(
                unit,
                node,
                format!("operator `{operator}` requires a value operand"),
            ));
        };
        return Ok(match operator.as_str() {
            "ref" => match operand {
                ValueType::Reference(item) | ValueType::SharedReference(item) => {
                    ValueType::Reference(item)
                }
                value_type => ValueType::Reference(ElementType::new(value_type)),
            },
            "shared ref" => match operand {
                ValueType::Reference(item) | ValueType::SharedReference(item) => {
                    ValueType::SharedReference(item)
                }
                value_type => ValueType::SharedReference(ElementType::new(value_type)),
            },
            "move" => operand,
            _ => unreachable!(),
        });
    }
    let Some(ValueType::Scalar(operand)) = infer_receiver_value_type(unit, operand_node, bindings)?
    else {
        return Err(operator_failure(
            unit,
            node,
            "unary operator requires a scalar operand",
        ));
    };

    let valid = match operator.as_str() {
        "-" => operand.is_integer() || matches!(operand, ScalarType::Float32 | ScalarType::Float64),
        "~" => operand.is_integer(),
        "not" => operand == ScalarType::Bool,
        _ => false,
    };
    if !valid {
        return Err(operator_failure(
            unit,
            node,
            format!("operator `{operator}` is not defined for `{operand}`"),
        ));
    }
    Ok(ValueType::Scalar(if operator == "not" {
        ScalarType::Bool
    } else {
        operand
    }))
}
#[expect(
    clippy::too_many_lines,
    reason = "family receiver, callback, argument, and result contracts remain auditable together"
)]
pub(super) fn infer_parse_or_radix_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(callee) = node.children.first() else {
        return Ok(None);
    };
    let Some(method) = bound_method(&unit.source, callee) else {
        return Ok(None);
    };
    if matches!(
        method.family,
        MemberFamily::Coerce | MemberFamily::Arithmetic(_)
    ) {
        return Ok(None);
    }
    let arguments = node.children.get(1);
    let arguments = arguments.map_or(&[][..], |arguments| arguments.children.as_slice());
    if arguments.len() != 1 {
        return Err(failure(
            &unit.source,
            "T0023",
            format!(
                "`.{}` requires exactly one argument",
                match method.family {
                    MemberFamily::Parse => "parse",
                    MemberFamily::Radix => "radix",
                    MemberFamily::Coerce | MemberFamily::Arithmetic(_) => unreachable!(),
                }
            ),
            node.span,
        ));
    }
    let receiver = find_node_by_span(&unit.tree.root, method.receiver)
        .expect("bound method receiver belongs to this syntax tree");
    let argument = arguments[0].children.last().unwrap_or(&arguments[0]);
    if method.family == MemberFamily::Radix {
        let argument_type = infer_value_type(unit, argument, bindings)?;
        if !matches!(argument_type, Some(ValueType::Scalar(scalar)) if scalar.is_integer()) {
            return Err(failure(
                &unit.source,
                "T0024",
                "`.radix` requires an integer radix argument",
                argument.span,
            ));
        }
        let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
        return match receiver_type {
            Some(ValueType::Scalar(ScalarType::String)) => {
                Ok(Some(ValueType::Scalar(ScalarType::Int)))
            }
            Some(ValueType::Scalar(scalar)) if scalar.is_integer() => {
                Ok(Some(ValueType::Scalar(ScalarType::String)))
            }
            _ => Err(failure(
                &unit.source,
                "T0024",
                "`.radix` requires a string or numeric receiver",
                receiver.span,
            )),
        };
    }
    let receiver_type = infer_value_type(unit, receiver, bindings)?;
    if receiver_type != Some(ValueType::Scalar(ScalarType::String)) {
        return Err(failure(
            &unit.source,
            "T0024",
            "`.parse` requires a string receiver",
            receiver.span,
        ));
    }
    let callback = arguments[0].children.last().unwrap_or(&arguments[0]);
    if callback.kind != SyntaxKind::Name {
        return Err(failure(
            &unit.source,
            "T0025",
            "`.parse` requires a statically resolvable function name",
            callback.span,
        ));
    }
    let callback_name = node_text(&unit.source, callback);
    let Some(contract) = resolved_function_contract(unit, callback_name, callback.span.start)
    else {
        return Err(failure(
            &unit.source,
            "T0025",
            format!("`{callback_name}` does not resolve to a parse callback"),
            callback.span,
        ));
    };
    if contract.parameters.len() != 1
        || contract.parameters[0].value_type != Some(ValueType::Scalar(ScalarType::String))
        || !matches!(contract.return_type, Some(ValueType::Scalar(_)))
    {
        return Err(failure(
            &unit.source,
            "T0026",
            format!(
                "parse callback `{callback_name}` must take one `string` value and declare a scalar return"
            ),
            callback.span,
        ));
    }
    let Some(ValueType::Scalar(result)) = contract.return_type.clone() else {
        unreachable!("checked above")
    };
    Ok(Some(if method.child == "checked" {
        ValueType::Optional(Box::new(ValueType::Scalar(result)))
    } else {
        ValueType::Scalar(result)
    }))
}
