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

pub(super) fn descriptor_contract<'a>(
    unit: &'a SemanticUnit,
    identity: &ObjectIdentity,
) -> Option<&'a DescriptorContract> {
    unit.descriptors
        .iter()
        .find(|object| object.identity == *identity)
}

/// Resolves one structural protocol member through the canonical descriptor contract.
pub(super) fn descriptor_protocol_method<'a>(
    unit: &'a SemanticUnit,
    identity: &ObjectIdentity,
    member: &str,
) -> Option<&'a FunctionContract> {
    object_method_contract(unit, identity, member, false)
}

pub(super) fn object_method_contract<'a>(
    unit: &'a SemanticUnit,
    identity: &ObjectIdentity,
    member: &str,
    is_static: bool,
) -> Option<&'a FunctionContract> {
    fn resolve<'a>(
        unit: &'a SemanticUnit,
        identity: &ObjectIdentity,
        member: &str,
        is_static: bool,
        visited: &mut BTreeSet<ObjectIdentity>,
    ) -> Option<&'a FunctionContract> {
        if !visited.insert(identity.clone()) {
            return None;
        }
        let object = descriptor_contract(unit, identity)?;
        if (if is_static {
            object.static_methods.contains(member)
        } else {
            object.methods.contains(member)
        }) && let Some(method) = unit.functions.iter().find(|function| {
            function.owner_identity.as_ref() == Some(&object.identity)
                && function.name == member
                && function.is_static == is_static
        }) {
            return Some(method);
        }
        object
            .traits
            .iter()
            .find_map(|used_trait| resolve(unit, used_trait, member, is_static, visited))
            .or_else(|| {
                object
                    .base
                    .as_ref()
                    .and_then(|base| resolve(unit, base, member, is_static, visited))
            })
            .or_else(|| {
                object
                    .interfaces
                    .iter()
                    .find_map(|interface| resolve(unit, interface, member, is_static, visited))
            })
    }

    resolve(unit, identity, member, is_static, &mut BTreeSet::new())
}

pub(super) fn object_field_type(
    unit: &SemanticUnit,
    object_identity: &ObjectIdentity,
    member: &str,
    is_static: bool,
) -> Option<ValueType> {
    let object = descriptor_contract(unit, object_identity)?;
    if (if is_static {
        object.static_members.contains(member)
    } else {
        object.members.contains(member)
    }) && let Some(field) = object
        .fields
        .iter()
        .find(|field| field.name == member && field.is_static == is_static)
    {
        return Some(field.value_type.clone());
    }
    for used_trait in &object.traits {
        if let Some(found) = object_field_type(unit, used_trait, member, is_static) {
            return Some(found);
        }
    }
    object
        .base
        .as_ref()
        .and_then(|base| object_field_type(unit, base, member, is_static))
}

fn optional_object_inner_has_member(
    unit: &SemanticUnit,
    receiver_type: &ValueType,
    member: &str,
) -> bool {
    let ValueType::Optional(inner) = receiver_type else {
        return false;
    };
    let ValueType::Object(identity) = inner.as_ref() else {
        return false;
    };
    (identity == &ObjectIdentity::new("/core/errors", "throwable")
        && matches!(member, "message" | "cause" | "render"))
        || object_member_type(unit, identity, member, false).is_some()
}

pub(crate) fn object_member_type(
    unit: &SemanticUnit,
    object_identity: &ObjectIdentity,
    member: &str,
    is_static: bool,
) -> Option<ValueType> {
    let object = descriptor_contract(unit, object_identity)?;
    if let Some(field_type) = object_field_type(unit, object_identity, member, is_static) {
        return Some(field_type);
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
            ValueType::AsyncFunction(parameters, result, method.task_transferability)
        } else {
            ValueType::Function(parameters, result)
        });
    }
    for used_trait in &object.traits {
        if let Some(trait_object) = unit.descriptors.iter().find(|candidate| {
            candidate.identity == *used_trait && candidate.kind == ObjectKind::Trait
        }) && let Some(found) =
            object_member_type(unit, &trait_object.identity, member, is_static)
        {
            return Some(found);
        }
    }
    object.base.as_ref().and_then(|base| {
        unit.descriptors
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
    if let Some(ValueType::Descriptor(identity)) = &receiver_type {
        let descriptor = descriptor_contract_by_identity(unit, identity)
            .expect("descriptor value types always retain a canonical contract");
        if let Some(field) = descriptor
            .fields
            .iter()
            .find(|field| field.is_static && field.name == member_name)
        {
            return Ok(Some(field.value_type.clone()));
        }
        return match member_name {
            "name" | "kind" | "identity" => Ok(Some(ValueType::Scalar(ScalarType::String))),
            "field-count" => Ok(Some(ValueType::Scalar(ScalarType::Int))),
            "inherently-identity-bearing" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            "field-names" | "field-external-names" => Ok(Some(ValueType::StringList)),
            "field-defaulted" | "field-optional" | "field-secret" => Ok(Some(ValueType::List(
                ElementType::new(ValueType::Scalar(ScalarType::Bool)),
            ))),
            _ => Err(failure(
                &unit.source,
                "T0071",
                format!("descriptor has no retained member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::DocumentDecodeOutcome(value)) = &receiver_type {
        return match member_name {
            "failed" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            "value" => Ok(Some(value.value_type())),
            "diagnostics" => Ok(Some(ValueType::List(ElementType::new(
                ValueType::DocumentDiagnostic,
            )))),
            _ => Err(failure(
                &unit.source,
                "T0115",
                format!("document decode outcome has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if receiver_type == Some(ValueType::DocumentDiagnostic) {
        return match member_name {
            "path" | "expected" | "actual-kind" | "reason" | "message" | "source"
            | "field-source" => Ok(Some(ValueType::Scalar(ScalarType::String))),
            _ => Err(failure(
                &unit.source,
                "T0115",
                format!("document diagnostic has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if matches!(
        receiver_type,
        Some(ValueType::Function(_, _) | ValueType::AsyncFunction(_, _, _))
    ) && matches!(
        member_name,
        "contracts" | "throwable-contract" | "escaping-throwables"
    ) {
        return Ok(Some(ValueType::Scalar(ScalarType::String)));
    }
    if member_name != "type"
        && matches!(
            &receiver_type,
            Some(ValueType::Object(identity))
                if identity == &ObjectIdentity::new("/core/errors", "throwable")
        )
    {
        return match member_name {
            "message" => Ok(Some(ValueType::Scalar(ScalarType::String))),
            "cause" => Ok(Some(ValueType::Optional(Box::new(ValueType::Object(
                ObjectIdentity::new("/core/errors", "throwable"),
            ))))),
            "render" => Ok(Some(ValueType::Function(
                Vec::new(),
                ElementType::new(ValueType::Scalar(ScalarType::String)),
            ))),
            _ => Err(failure(
                &unit.source,
                "T0055",
                format!("`throwable` has no instance member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::ChannelPair(item)) = &receiver_type {
        return match member_name {
            "sender" => Ok(Some(ValueType::ChannelSender(item.clone()))),
            "receiver" => Ok(Some(ValueType::ChannelReceiver(item.clone()))),
            _ => Err(failure(
                &unit.source,
                "T0108",
                format!("channel pair has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::ChannelSender(item)) = &receiver_type {
        return match member_name {
            "send" => Ok(Some(ValueType::AsyncFunction(
                vec![item.clone()],
                ElementType::new(ValueType::ChannelSendOutcome(item.clone())),
                TaskTransferability::Local,
            ))),
            "close" => Ok(Some(ValueType::Function(
                Vec::new(),
                ElementType::new(ValueType::Scalar(ScalarType::None)),
            ))),
            _ => Err(failure(
                &unit.source,
                "T0108",
                format!("channel sender has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::ChannelReceiver(item)) = &receiver_type {
        return match member_name {
            "receive" => Ok(Some(ValueType::AsyncFunction(
                Vec::new(),
                ElementType::new(ValueType::ChannelReceiveOutcome(item.clone())),
                TaskTransferability::Local,
            ))),
            "close" => Ok(Some(ValueType::Function(
                Vec::new(),
                ElementType::new(ValueType::List(item.clone())),
            ))),
            _ => Err(failure(
                &unit.source,
                "T0108",
                format!("channel receiver has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::ChannelSendOutcome(item)) = &receiver_type {
        return match member_name {
            "accepted" | "closed" | "dropped" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            "rejected-value" | "dropped-value" => {
                Ok(Some(ValueType::Optional(Box::new(item.value_type()))))
            }
            _ => Err(failure(
                &unit.source,
                "T0108",
                format!("channel send outcome has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::ChannelReceiveOutcome(item)) = &receiver_type {
        return match member_name {
            "available" | "closed" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            "value" => Ok(Some(ValueType::Optional(Box::new(item.value_type())))),
            _ => Err(failure(
                &unit.source,
                "T0108",
                format!("channel receive outcome has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::IterationStep(item)) = &receiver_type {
        return match member_name {
            "item" | "end" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            "value" => Ok(Some(ValueType::Optional(Box::new(item.value_type())))),
            _ => Err(failure(
                &unit.source,
                "T0087",
                format!("iteration step has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if let Some(ValueType::AsyncIterationStep(item)) = &receiver_type {
        return match member_name {
            "item" | "end" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            "value" => Ok(Some(ValueType::Optional(Box::new(item.value_type())))),
            _ => Err(failure(
                &unit.source,
                "T0087",
                format!("async iteration step has no member `{member_name}`"),
                member.span,
            )),
        };
    }
    if receiver_type == Some(ValueType::AsyncSinkOutcome) {
        return match member_name {
            "accepted" | "closed" => Ok(Some(ValueType::Scalar(ScalarType::Bool))),
            _ => Err(failure(
                &unit.source,
                "T0088",
                format!("async sink outcome has no member `{member_name}`"),
                member.span,
            )),
        };
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
    if let Some(ValueType::Object(object_name)) = &receiver_type
        && let Some(member_type) = object_member_type(unit, object_name, member_name, false)
    {
        return Ok(Some(member_type));
    }
    if let Some(ValueType::Object(identity)) = &receiver_type
        && let Some(removed) = unit.removed_projected_member(identity, member_name, false)
    {
        return Err(failure(
            &unit.source,
            "S2031",
            format!(
                "Rust dependency member `{}.{member_name}` was projected by version {} but is absent from version {}",
                identity.name, removed.previous_version, removed.current_version
            ),
            member.span,
        ));
    }
    if member_name == "type" {
        return Ok(receiver_type.map(|value_type| {
            ValueType::Descriptor(canonical_descriptor_identity(unit, &value_type))
        }));
    }
    if let Some(ValueType::Object(object_name)) = &receiver_type {
        return Err(failure(
            &unit.source,
            "T0055",
            format!(
                "`{}` has no instance member `{member_name}`",
                unit.descriptors
                    .iter()
                    .find(|object| object.identity == *object_name)
                    .map_or_else(
                        || diagnostic_object_identity(&unit.descriptors, object_name),
                        |object| object.name.clone()
                    )
            ),
            member.span,
        ));
    }
    if receiver_type.as_ref().is_some_and(|value_type| {
        descriptor_method_requires_invocation(unit, value_type, member_name)
    }) {
        return Ok(None);
    }
    if member_name == "length"
        && receiver_type
            .as_ref()
            .is_some_and(|value_type| descriptor_has_member(unit, value_type, member_name))
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
        | (Some(ValueType::DivRemResult(ty)), "quotient" | "remainder")
        | (Some(ValueType::FloatDecomposition(ty)), "mantissa") => {
            return Ok(Some(ValueType::Scalar(ty)));
        }
        (Some(ValueType::FloatDecomposition(_)), "exponent") => {
            return Ok(Some(ValueType::Scalar(ScalarType::Int32)));
        }
        (Some(ValueType::OverflowResult(_)), "overflowed") => {
            return Ok(Some(ValueType::Scalar(ScalarType::Bool)));
        }
        (
            Some(
                ValueType::OverflowResult(_)
                | ValueType::DivRemResult(_)
                | ValueType::FloatDecomposition(_),
            ),
            _,
        ) => {
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
        if let Some(ValueType::Scalar(receiver)) = receiver_type.clone()
            && descriptor_has_member(unit, &ValueType::Scalar(receiver), member_name)
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
    if member_name != "length" {
        return match receiver_type {
            Some(receiver_type) => {
                let mut diagnostic = Diagnostic::error(
                    "T0031",
                    format!("`{receiver_type}` has no member `.{member_name}`"),
                    member.span,
                );
                if optional_object_inner_has_member(unit, &receiver_type, member_name) {
                    diagnostic = diagnostic.with_help(format!(
                        "`.{member_name}` requires narrowing; bind the optional-producing expression \
                         to a name, guard that name with `!= none`, then select `.{member_name}` from \
                         the narrowed name"
                    ));
                }
                Err(SemanticFailure {
                    source: unit.source.clone(),
                    diagnostics: vec![diagnostic],
                })
            }
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
    let Some(parameters) = contract.parameters else {
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
    if arguments.len() != parameters.len() {
        return Err(failure(
            &unit.source,
            "T0023",
            format!(
                "`.{member_name}` requires exactly {} argument{}",
                parameters.len(),
                if parameters.len() == 1 { "" } else { "s" }
            ),
            node.span,
        ));
    }
    for (argument, parameter) in arguments.iter().zip(parameters) {
        let value = argument.children.last().unwrap_or(argument);
        if let Some(actual) = infer_value_type(unit, value, bindings)? {
            let expected = match parameter {
                FloatMemberArgument::Receiver => ValueType::Scalar(receiver),
                FloatMemberArgument::Int32 => ValueType::Scalar(ScalarType::Int32),
            };
            validate_value_destination(
                &unit.source,
                &unit.descriptors,
                "floating operation argument",
                expected,
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
    let member_path = if child == "default" {
        family.to_owned()
    } else {
        format!("{family}.{child}")
    };
    let receiver_valid = subject_type
        .as_ref()
        .is_some_and(|value_type| descriptor_has_method(unit, value_type, family));
    if !receiver_valid {
        return Err(failure(
            &unit.source,
            "T0032",
            format!("`.{family}` is not available on this receiver"),
            subject.span,
        ));
    }
    let Some(operation) = descriptor_operation(
        unit,
        subject_type.as_ref().expect("validated receiver"),
        &member_path,
    ) else {
        return Err(failure(
            &unit.source,
            "T0034",
            format!("`.{family}.{child}` is not available"),
            node.span,
        ));
    };
    let arguments = node
        .children
        .get(1)
        .map_or(&[][..], |arguments| arguments.children.as_slice());
    let (minimum, maximum) = match operation {
        "string.trim"
        | "string.upper"
        | "string.upper.first"
        | "string.upper.words"
        | "string.lower"
        | "string.lower.first"
        | "string.normalise.nfc"
        | "string.normalise.nfd"
        | "string.normalise.nfkc"
        | "string.normalise.nfkd"
        | "string.case-fold" => (0, 0),
        "string.trim.start" | "string.trim.end" => (0, 1),
        "string.replace" => (2, 2),
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
        let expected = if matches!(operation, "string.encode" | "bytes.decode") {
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
    let result = match operation {
        "string.contains" | "string.contains.start" | "string.contains.end" => {
            ValueType::Scalar(ScalarType::Bool)
        }
        "string.find" => ValueType::Optional(Box::new(ValueType::TextRange)),
        "string.find.all" => ValueType::TextRangeList,
        "string.find.count" => ValueType::Scalar(ScalarType::Int),
        "string.split" => ValueType::StringList,
        "string.encode" => ValueType::Scalar(ScalarType::Bytes),
        "bytes.decode"
        | "string.case-fold"
        | "string.replace"
        | "string.trim"
        | "string.trim.start"
        | "string.trim.end"
        | "string.upper"
        | "string.upper.first"
        | "string.upper.words"
        | "string.lower"
        | "string.lower.first"
        | "string.normalise.nfc"
        | "string.normalise.nfd"
        | "string.normalise.nfkc"
        | "string.normalise.nfkd" => ValueType::Scalar(ScalarType::String),
        _ => return Ok(None),
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
            Some(ValueType::Task(result, _)) => Ok(result.value_type()),
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
        if operator == "shared ref" && matches!(operand, ValueType::Reference(_)) {
            return Err(operator_failure(
                unit,
                node,
                "`shared ref` cannot promote a non-owning reference to shared ownership",
            ));
        }
        return Ok(match operator.as_str() {
            "ref" => match operand {
                ValueType::Reference(item) | ValueType::SharedReference(item) => {
                    ValueType::Reference(item)
                }
                value_type => ValueType::Reference(ElementType::new(value_type)),
            },
            "shared ref" => match operand {
                ValueType::SharedReference(item) => ValueType::SharedReference(item),
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
