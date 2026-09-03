use super::prelude::*;

#[allow(clippy::too_many_lines)]
pub(super) fn infer_arithmetic_family_type(
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
    let MemberFamily::Arithmetic(family) = method.family else {
        return Ok(None);
    };
    let receiver = find_node_by_span(&unit.tree.root, method.receiver)
        .expect("bound arithmetic receiver belongs to this syntax tree");
    let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
    if matches!(receiver_type, Some(ValueType::Object(_))) {
        return Ok(None);
    }
    let Some(ValueType::Scalar(receiver_type)) = receiver_type else {
        return Err(failure(
            &unit.source,
            "T0036",
            format!("`.{}` requires an integer receiver", family.source_name()),
            receiver.span,
        ));
    };
    if !receiver_type.is_integer() {
        return Err(failure(
            &unit.source,
            "T0036",
            format!("`.{}` requires an integer receiver", family.source_name()),
            receiver.span,
        ));
    }
    if family == ArithmeticFamily::Negate
        && !matches!(
            receiver_type,
            ScalarType::Int
                | ScalarType::Int8
                | ScalarType::Int16
                | ScalarType::Int32
                | ScalarType::Int64
                | ScalarType::Int128
        )
    {
        return Err(failure(
            &unit.source,
            "T0037",
            "`.negate` is not available on unsigned integers",
            receiver.span,
        ));
    }
    let arguments = node.children.get(1);
    let arguments = arguments.map_or(&[][..], |arguments| arguments.children.as_slice());
    let expected = usize::from(family != ArithmeticFamily::Negate);
    if arguments.len() != expected {
        return Err(failure(
            &unit.source,
            "T0023",
            format!(
                "`.{}` requires exactly {expected} argument{}",
                family.source_name(),
                if expected == 1 { "" } else { "s" }
            ),
            node.span,
        ));
    }
    if let Some(argument) = arguments.first() {
        let argument = argument.children.last().unwrap_or(argument);
        let argument_type = infer_value_type(unit, argument, bindings)?;
        let valid = if matches!(
            family,
            ArithmeticFamily::ShiftLeft | ArithmeticFamily::ShiftRight
        ) {
            matches!(argument_type, Some(ValueType::Scalar(ty)) if ty.is_integer())
        } else {
            argument_type == Some(ValueType::Scalar(receiver_type))
                || contextual_constant(&unit.source, argument, receiver_type).is_some()
        };
        if !valid {
            return Err(failure(
                &unit.source,
                "T0028",
                format!(
                    "`.{}` argument is incompatible with `{receiver_type}`",
                    family.source_name()
                ),
                argument.span,
            ));
        }
    }
    let fixed = receiver_type != ScalarType::Int;
    let child_allowed = match method.child {
        "default" => true,
        "checked" => {
            fixed
                || matches!(
                    family,
                    ArithmeticFamily::Divide
                        | ArithmeticFamily::Remainder
                        | ArithmeticFamily::DivRem
                )
        }
        "wrap" => fixed && family != ArithmeticFamily::DivRem,
        "saturate" | "overflowing" => {
            fixed
                && !matches!(
                    family,
                    ArithmeticFamily::DivRem
                        | ArithmeticFamily::ShiftLeft
                        | ArithmeticFamily::ShiftRight
                )
        }
        _ => false,
    };
    if !child_allowed {
        return Err(failure(
            &unit.source,
            "T0029",
            format!(
                "`.{}.{}` is not available on `{receiver_type}`",
                family.source_name(),
                method.child
            ),
            callee.span,
        ));
    }
    let result = if method.child == "overflowing" {
        ValueType::OverflowResult(receiver_type)
    } else if family == ArithmeticFamily::DivRem {
        if method.child == "checked" {
            return Err(failure(
                &unit.source,
                "T0030",
                "`div-rem.checked` optional result values are not yet representable",
                callee.span,
            ));
        }
        ValueType::DivRemResult(receiver_type)
    } else if method.child == "checked" {
        ValueType::Optional(Box::new(ValueType::Scalar(receiver_type)))
    } else {
        ValueType::Scalar(receiver_type)
    };
    Ok(Some(result))
}

pub(super) fn infer_integer_coercion_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(callee) = node.children.first() else {
        return Ok(None);
    };
    let Some((source_node, policy)) = integer_coercion_call(&unit.source, callee) else {
        if let Some(member) = obsolete_integer_coercion_member(unit, callee) {
            return Err(failure(
                &unit.source,
                "T0017",
                format!(
                    "`{member}` is not valid syntax; use `.coerce.{}`",
                    match member {
                        "checked-coerce" => "checked",
                        "wrapping-coerce" => "wrap",
                        "saturating-coerce" => "saturate",
                        _ => unreachable!("obsolete coercion members are matched above"),
                    }
                ),
                callee.span,
            ));
        }
        if let Some(chain) = invalid_coercion_policy(unit, callee) {
            return Err(failure(
                &unit.source,
                "T0010",
                format!("`{chain}` is not an available coercion policy"),
                callee.span,
            ));
        }
        return Ok(None);
    };
    let Some(ValueType::Scalar(source_type)) =
        infer_receiver_value_type(unit, source_node, bindings)?
    else {
        return Err(failure(
            &unit.source,
            "T0009",
            "`.coerce` requires an integer source",
            source_node.span,
        ));
    };
    if !source_type.is_integer() {
        return Err(failure(
            &unit.source,
            "T0009",
            format!(
                "`{}` requires an integer source, found `{source_type}`",
                policy.invocation_name()
            ),
            source_node.span,
        ));
    }
    let destination_node = node
        .children
        .get(1)
        .and_then(|arguments| arguments.children.first())
        .and_then(|argument| argument.children.last())
        .ok_or_else(|| {
            failure(
                &unit.source,
                "T0008",
                format!(
                    "`{}` from `{source_type}` requires one integer destination",
                    policy.invocation_name()
                ),
                node.span,
            )
        })?;
    let destination_name = node_text(&unit.source, destination_node);
    let destination = unit
        .descriptor_alias_at(destination_name, destination_node.span.start)
        .ok_or_else(|| {
            failure(
                &unit.source,
                "T0008",
                format!(
                    "`{destination_name}` is not a supported destination for `{}` from `{source_type}`",
                    policy.invocation_name()
                ),
                destination_node.span,
            )
        })?;
    if !destination.is_integer() {
        return Err(failure(
            &unit.source,
            "T0008",
            format!(
                "`{destination}` is not a supported destination for `{}` from `{source_type}`",
                policy.invocation_name()
            ),
            destination_node.span,
        ));
    }
    let result = integer_coercion_result_type(source_type, destination, policy)
        .map_err(|message| failure(&unit.source, "T0010", message, destination_node.span))?;
    Ok(Some(result))
}

pub(super) fn integer_coercion_result_type(
    source: ScalarType,
    destination: ScalarType,
    policy: CoercionPolicy,
) -> Result<ValueType, String> {
    match (source, destination, policy) {
        (
            _,
            ScalarType::Int,
            CoercionPolicy::Checked | CoercionPolicy::Wrap | CoercionPolicy::Saturate,
        ) => Err(format!(
            "`.coerce.{}` from `{source}` requires a fixed-width integer destination",
            policy.source_name()
        )),
        (_, _, CoercionPolicy::Checked) => Ok(ValueType::Optional(Box::new(ValueType::Scalar(
            destination,
        )))),
        (_, _, CoercionPolicy::Default | CoercionPolicy::Wrap | CoercionPolicy::Saturate) => {
            Ok(ValueType::Scalar(destination))
        }
    }
}

pub(crate) fn bound_method(source: &SourceFile, callee: &SyntaxNode) -> Option<BoundMethod> {
    if callee.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    let member_name = node_text(source, member);
    let direct = match member_name {
        "coerce" => Some((MemberFamily::Coerce, "default")),
        "parse" => Some((MemberFamily::Parse, "default")),
        "radix" => Some((MemberFamily::Radix, "default")),
        name => ArithmeticFamily::from_source_name(name)
            .map(|family| (MemberFamily::Arithmetic(family), "default")),
    };
    if let Some((family, child)) = direct {
        return Some(BoundMethod {
            receiver: receiver.span,
            family,
            child,
        });
    }
    if receiver.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let [source_node, family_node] = receiver.children.as_slice() else {
        return None;
    };
    let selection = match (node_text(source, family_node), member_name) {
        ("coerce", "checked") => (MemberFamily::Coerce, "checked"),
        ("coerce", "wrap") => (MemberFamily::Coerce, "wrap"),
        ("coerce", "saturate") => (MemberFamily::Coerce, "saturate"),
        ("parse", "checked") => (MemberFamily::Parse, "checked"),
        (family, child @ ("checked" | "wrap" | "saturate" | "overflowing")) => {
            let child = match child {
                "checked" => "checked",
                "wrap" => "wrap",
                "saturate" => "saturate",
                "overflowing" => "overflowing",
                _ => unreachable!(),
            };
            (
                MemberFamily::Arithmetic(ArithmeticFamily::from_source_name(family)?),
                child,
            )
        }
        _ => return None,
    };
    Some(BoundMethod {
        receiver: source_node.span,
        family: selection.0,
        child: selection.1,
    })
}

/// Resolves the canonical `.coerce` callable family and its selected policy child.
///
/// The returned policy is shared semantic metadata for analysis and lowering; the
/// Rust helper names used after family erasure are not independent source members.
pub(crate) fn integer_coercion_call<'a>(
    source: &SourceFile,
    callee: &'a SyntaxNode,
) -> Option<(&'a SyntaxNode, CoercionPolicy)> {
    let method = bound_method(source, callee)?;
    if method.family != MemberFamily::Coerce {
        return None;
    }
    let policy = match method.child {
        "default" => CoercionPolicy::Default,
        child => CoercionPolicy::from_member(child)?,
    };
    let receiver = callee.children.first()?;
    let source_node = if method.child == "default" {
        receiver
    } else {
        receiver.children.first()?
    };
    Some((source_node, policy))
}

pub(super) fn invalid_coercion_policy(unit: &SemanticUnit, callee: &SyntaxNode) -> Option<String> {
    (coercion_family_receiver(unit, callee)
        && integer_coercion_call(&unit.source, callee).is_none())
    .then(|| {
        let callee_text = node_text(&unit.source, callee);
        let family_start = callee_text.find(".coerce").unwrap_or(0);
        callee_text[family_start..].to_owned()
    })
}

pub(super) fn coercion_family_receiver(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
    let [receiver, member] = node.children.as_slice() else {
        return false;
    };
    node.kind == SyntaxKind::MemberExpression
        && (node_text(&unit.source, member) == "coerce" || coercion_family_receiver(unit, receiver))
}

pub(super) fn member_family_receiver(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
    let [receiver, member] = node.children.as_slice() else {
        return false;
    };
    if node_text(&unit.source, member) == "remainder"
        && matches!(
            infer_value_type(unit, receiver, &unit.typed_bindings),
            Ok(Some(ValueType::DivRemResult(_)))
        )
    {
        return false;
    }
    node.kind == SyntaxKind::MemberExpression
        && matches!(
            node_text(&unit.source, member),
            "coerce"
                | "parse"
                | "radix"
                | "add"
                | "subtract"
                | "multiply"
                | "divide"
                | "remainder"
                | "div-rem"
                | "negate"
                | "shift-left"
                | "shift-right"
        )
}

pub(super) fn obsolete_integer_coercion_member<'a>(
    unit: &'a SemanticUnit,
    callee: &'a SyntaxNode,
) -> Option<&'a str> {
    let [_, member] = callee.children.as_slice() else {
        return None;
    };
    (callee.kind == SyntaxKind::MemberExpression)
        .then(|| node_text(&unit.source, member))
        .filter(|member| {
            matches!(
                *member,
                "checked-coerce" | "wrapping-coerce" | "saturating-coerce"
            )
        })
}

#[expect(
    clippy::too_many_lines,
    reason = "binary inference keeps operator precedence, optional equality, and numeric promotion auditable"
)]
pub(super) fn infer_binary_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<ValueType, SemanticFailure> {
    let [left_node, right_node] = node.children.as_slice() else {
        return Err(operator_failure(
            unit,
            node,
            "binary operator requires two operands",
        ));
    };
    let left = infer_receiver_value_type(unit, left_node, bindings)?;
    let right = infer_receiver_value_type(unit, right_node, bindings)?;
    let operator = unit.source.text()[left_node.span.end..right_node.span.start].trim();
    if operator == "is"
        && (node_text(&unit.source, left_node).trim() == "none"
            || node_text(&unit.source, right_node).trim() == "none")
    {
        return Err(failure(
            &unit.source,
            "T0038",
            "`is none` is invalid; type membership is written `is a none`",
            node.span,
        ));
    }
    if operator == "is" {
        return Ok(ValueType::Scalar(ScalarType::Bool));
    }
    if matches!(operator, "==" | "!=")
        && ((matches!(left, Some(ValueType::Optional(_)))
            && node_text(&unit.source, right_node).trim() == "none")
            || (matches!(right, Some(ValueType::Optional(_)))
                && node_text(&unit.source, left_node).trim() == "none"))
    {
        return Ok(ValueType::Scalar(ScalarType::Bool));
    }
    if matches!(operator, "==" | "!=")
        && let (Some(ValueType::Object(left)), Some(ValueType::Object(right))) = (&left, &right)
        && left == right
        && unit.comparable_foreign_objects.contains(left)
    {
        return Ok(ValueType::Scalar(ScalarType::Bool));
    }
    let comparison = matches!(operator, "==" | "!=" | "<" | "<=" | ">" | ">=");
    let contextual_numeric = matches!(
        operator,
        "+" | "-" | "*" | "/" | "%" | "&" | "^" | "|" | "==" | "!=" | "<" | "<=" | ">" | ">="
    );
    if contextual_numeric
        && let Some(ValueType::Scalar(left_type)) = left
        && is_numeric(left_type)
        && contextual_constant(&unit.source, right_node, left_type)
            .transpose()?
            .is_some()
    {
        return Ok(ValueType::Scalar(if comparison {
            ScalarType::Bool
        } else {
            left_type
        }));
    }
    if contextual_numeric
        && let Some(ValueType::Scalar(right_type)) = right
        && is_numeric(right_type)
        && contextual_constant(&unit.source, left_node, right_type)
            .transpose()?
            .is_some()
    {
        return Ok(ValueType::Scalar(if comparison {
            ScalarType::Bool
        } else {
            right_type
        }));
    }
    let (Some(ValueType::Scalar(left)), Some(ValueType::Scalar(right))) = (left, right) else {
        return Err(operator_failure(
            unit,
            node,
            "operator requires scalar operands",
        ));
    };
    let same = left == right;
    if contextual_numeric && left != right && left.is_integer() && right.is_integer() {
        if contextual_constant(&unit.source, right_node, right).is_some() {
            contextual_constant(&unit.source, right_node, left).expect(
                "integer constant expression remains contextual across integer destinations",
            )?;
        }
        if contextual_constant(&unit.source, left_node, left).is_some() {
            contextual_constant(&unit.source, left_node, right).expect(
                "integer constant expression remains contextual across integer destinations",
            )?;
        }
    }
    if contextual_numeric && left != right && left.is_integer() && right.is_integer() {
        return Ok(ValueType::Scalar(if comparison {
            ScalarType::Bool
        } else {
            promoted_integer_type(left, right)
        }));
    }
    let numeric =
        |ty: ScalarType| ty.is_integer() || matches!(ty, ScalarType::Float32 | ScalarType::Float64);
    let result = match operator {
        "+" | "-" | "*" | "/" | "%" if same && numeric(left) => left,
        "<<" | ">>" if left.is_integer() && right.is_integer() => left,
        "&" | "^" | "|" if same && left.is_integer() => left,
        "and" | "or" if left == ScalarType::Bool && right == ScalarType::Bool => ScalarType::Bool,
        "==" | "!=" if same => ScalarType::Bool,
        "<" | "<=" | ">" | ">=" if same && (numeric(left) || left == ScalarType::String) => {
            ScalarType::Bool
        }
        _ => {
            return Err(operator_failure(
                unit,
                node,
                format!("operator `{operator}` is not defined for `{left}` and `{right}`"),
            ));
        }
    };
    Ok(ValueType::Scalar(result))
}

pub(super) fn operator_failure(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    message: impl Into<String>,
) -> SemanticFailure {
    failure(&unit.source, "T0011", message, node.span)
}

pub(super) fn infer_literal_type(unit: &SemanticUnit, node: &SyntaxNode) -> Option<ScalarType> {
    infer_literal_type_from_source(&unit.source, node)
}

pub(super) fn infer_literal_type_from_source(
    source: &SourceFile,
    node: &SyntaxNode,
) -> Option<ScalarType> {
    if node.kind == SyntaxKind::UnaryExpression {
        return node
            .children
            .last()
            .and_then(|child| infer_literal_type_from_source(source, child));
    }
    if node.kind != SyntaxKind::Literal {
        return None;
    }

    let text = node_text(source, node);
    match text {
        "true" | "false" => Some(ScalarType::Bool),
        value if value.starts_with("b'") => Some(ScalarType::Bytes),
        value if value.starts_with(['\'', '"', '>']) => Some(ScalarType::String),
        value if value.contains('.') => Some(ScalarType::Float64),
        _ => Some(ScalarType::Int),
    }
}

pub(crate) fn contextual_constant(
    source: &SourceFile,
    node: &SyntaxNode,
    destination: ScalarType,
) -> Option<Result<ContextualConstant, SemanticFailure>> {
    if !is_numeric(destination) {
        return None;
    }
    contextual_constant_value(source, node, destination).map(|result| {
        result.and_then(|value| {
            match &value {
                ContextualConstant::Integer(integer) => {
                    check_integer_range(source, destination, integer, node.span)?;
                }
                ContextualConstant::Float32(value) if !value.is_finite() => {
                    return Err(invalid_floating_constant(source, destination, node.span));
                }
                ContextualConstant::Float64(value) if !value.is_finite() => {
                    return Err(invalid_floating_constant(source, destination, node.span));
                }
                ContextualConstant::Float32(_) | ContextualConstant::Float64(_) => {}
            }
            Ok(value)
        })
    })
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "storage selection owns the optional inferred recursive type"
)]
pub(super) fn small_int_storage(
    unit: &SemanticUnit,
    value: &SyntaxNode,
    inferred: Option<ValueType>,
) -> Option<ScalarType> {
    if let Some(Ok(ContextualConstant::Integer(integer))) =
        contextual_constant(&unit.source, value, ScalarType::Int)
        && integer.to_i64().is_some()
    {
        return Some(ScalarType::Int64);
    }
    matches!(
        inferred,
        Some(ValueType::Scalar(
            ScalarType::Int8
                | ScalarType::Int16
                | ScalarType::Int32
                | ScalarType::Int64
                | ScalarType::Uint8
                | ScalarType::Uint16
                | ScalarType::Uint32
        ))
    )
    .then_some(ScalarType::Int64)
}

pub(super) fn contextual_constant_value(
    source: &SourceFile,
    node: &SyntaxNode,
    destination: ScalarType,
) -> Option<Result<ContextualConstant, SemanticFailure>> {
    let result = match node.kind {
        SyntaxKind::GroupExpression => {
            return node
                .children
                .first()
                .and_then(|child| contextual_constant_value(source, child, destination));
        }
        SyntaxKind::UnaryExpression => {
            let operand = node.children.last()?;
            let value = contextual_constant_value(source, operand, destination)?;
            value.map(|value| match value {
                ContextualConstant::Integer(value) => ContextualConstant::Integer(-value),
                ContextualConstant::Float32(value) => ContextualConstant::Float32(-value),
                ContextualConstant::Float64(value) => ContextualConstant::Float64(-value),
            })
        }
        SyntaxKind::BinaryExpression => {
            let [left, right] = node.children.as_slice() else {
                return None;
            };
            let operator = source.text()[left.span.end..right.span.start].trim();
            let valid = if destination.is_integer() {
                matches!(
                    operator,
                    "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>"
                )
            } else {
                matches!(operator, "+" | "-" | "*" | "/" | "%")
            };
            if !valid {
                return None;
            }
            let left = contextual_constant_value(source, left, destination)?;
            let right = contextual_constant_value(source, right, destination)?;
            match (left, right) {
                (Ok(ContextualConstant::Integer(left)), Ok(ContextualConstant::Integer(right))) => {
                    fold_integer_constant(source, node.span, operator, left, right)
                }
                (Ok(ContextualConstant::Float32(left)), Ok(ContextualConstant::Float32(right))) => {
                    Ok(ContextualConstant::Float32(fold_float32_constant(
                        operator, left, right,
                    )))
                }
                (Ok(ContextualConstant::Float64(left)), Ok(ContextualConstant::Float64(right))) => {
                    Ok(ContextualConstant::Float64(fold_float64_constant(
                        operator, left, right,
                    )))
                }
                (Err(error), _) | (_, Err(error)) => Err(error),
                _ => return None,
            }
        }
        SyntaxKind::Literal
            if infer_literal_type_from_source(source, node).is_some_and(is_numeric) =>
        {
            contextual_literal(source, node, destination)
        }
        _ => return None,
    };
    Some(result)
}

pub(super) fn contextual_literal(
    source: &SourceFile,
    node: &SyntaxNode,
    destination: ScalarType,
) -> Result<ContextualConstant, SemanticFailure> {
    let text = node_text(source, node).replace('_', "");
    let decimal = text.contains('.');
    if destination.is_integer() {
        let value = if decimal {
            let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
            if !fraction.chars().all(|digit| digit == '0') {
                return Err(failure(
                    source,
                    "T0003",
                    format!("constant `{text}` is not an exact `{destination}` value"),
                    node.span,
                ));
            }
            BigInt::parse_bytes(whole.as_bytes(), 10).expect("validated decimal integer constant")
        } else {
            parse_integer_source_text(source, node).expect("validated integer constant")
        };
        Ok(ContextualConstant::Integer(value))
    } else if decimal {
        if destination == ScalarType::Float32 {
            let value = text
                .parse::<f32>()
                .map_err(|_| invalid_floating_constant(source, destination, node.span))?;
            Ok(ContextualConstant::Float32(value))
        } else {
            let value = text
                .parse::<f64>()
                .map_err(|_| invalid_floating_constant(source, destination, node.span))?;
            Ok(ContextualConstant::Float64(value))
        }
    } else {
        let integer =
            parse_integer_source_text(source, node).expect("validated whole-number constant");
        if destination == ScalarType::Float32 {
            let value = integer
                .to_f32()
                .filter(|value| BigInt::from_f32(*value).as_ref() == Some(&integer))
                .ok_or_else(|| invalid_floating_constant(source, destination, node.span))?;
            Ok(ContextualConstant::Float32(value))
        } else {
            let value = integer
                .to_f64()
                .filter(|value| BigInt::from_f64(*value).as_ref() == Some(&integer))
                .ok_or_else(|| invalid_floating_constant(source, destination, node.span))?;
            Ok(ContextualConstant::Float64(value))
        }
    }
}

pub(super) fn invalid_floating_constant(
    source: &SourceFile,
    destination: ScalarType,
    span: Span,
) -> SemanticFailure {
    failure(
        source,
        "T0003",
        format!("constant is not a finite exact `{destination}` value"),
        span,
    )
}

pub(super) fn fold_integer_constant(
    source: &SourceFile,
    span: Span,
    operator: &str,
    left: BigInt,
    right: BigInt,
) -> Result<ContextualConstant, SemanticFailure> {
    let value = match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" if right != BigInt::from(0_u8) => {
            let quotient = &left / &right;
            let remainder = &left % &right;
            if remainder < BigInt::from(0_u8) {
                if right < BigInt::from(0_u8) {
                    quotient + 1
                } else {
                    quotient - 1
                }
            } else {
                quotient
            }
        }
        "%" if right != BigInt::from(0_u8) => {
            let remainder = &left % &right;
            if remainder < BigInt::from(0_u8) {
                if right < BigInt::from(0_u8) {
                    remainder - right
                } else {
                    remainder + right
                }
            } else {
                remainder
            }
        }
        "&" => left & right,
        "|" => left | right,
        "^" => left ^ right,
        "<<" | ">>" => {
            let Some(count) = right.to_usize() else {
                return Err(failure(
                    source,
                    "T0011",
                    "constant shift count cannot be represented on this target",
                    span,
                ));
            };
            if operator == "<<" {
                left << count
            } else {
                left >> count
            }
        }
        _ => {
            return Err(failure(
                source,
                "T0011",
                "invalid constant arithmetic",
                span,
            ));
        }
    };
    Ok(ContextualConstant::Integer(value))
}

pub(super) fn fold_float32_constant(operator: &str, left: f32, right: f32) -> f32 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        "%" => left % right,
        _ => unreachable!("validated constant floating operator"),
    }
}

pub(super) fn fold_float64_constant(operator: &str, left: f64, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        "%" => left % right,
        _ => unreachable!("validated constant floating operator"),
    }
}

pub(super) fn parse_integer_source_text(source: &SourceFile, node: &SyntaxNode) -> Option<BigInt> {
    let compact = source.text()[node.span.start..node.span.end]
        .chars()
        .filter(|character| !character.is_whitespace() && *character != '_')
        .collect::<String>();
    let (negative, digits) = compact
        .strip_prefix('-')
        .map_or((false, compact.as_str()), |digits| (true, digits));
    let digits = digits.strip_prefix('+').unwrap_or(digits);
    let (radix, digits) = if let Some(digits) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        (16, digits)
    } else if let Some(digits) = digits
        .strip_prefix("0o")
        .or_else(|| digits.strip_prefix("0O"))
    {
        (8, digits)
    } else if let Some(digits) = digits
        .strip_prefix("0b")
        .or_else(|| digits.strip_prefix("0B"))
    {
        (2, digits)
    } else {
        (10, digits)
    };
    let value = BigInt::parse_bytes(digits.as_bytes(), radix)?;
    Some(if negative { -value } else { value })
}

pub(super) fn check_integer_range(
    source: &SourceFile,
    destination: ScalarType,
    value: &BigInt,
    span: Span,
) -> Result<(), SemanticFailure> {
    let bounds = match destination {
        ScalarType::Int8 => integer_bounds(8, true),
        ScalarType::Int16 => integer_bounds(16, true),
        ScalarType::Int32 => integer_bounds(32, true),
        ScalarType::Int64 => integer_bounds(64, true),
        ScalarType::Int128 => integer_bounds(128, true),
        ScalarType::Uint8 => integer_bounds(8, false),
        ScalarType::Uint16 => integer_bounds(16, false),
        ScalarType::Uint32 => integer_bounds(32, false),
        ScalarType::Uint64 => integer_bounds(64, false),
        ScalarType::Uint128 => integer_bounds(128, false),
        _ => return Ok(()),
    };
    if value < &bounds.0 || value > &bounds.1 {
        return Err(failure(
            source,
            "T0003",
            format!("constant `{value}` is outside the range of `{destination}`"),
            span,
        ));
    }
    Ok(())
}

pub(super) fn integer_bounds(bits: usize, signed: bool) -> (BigInt, BigInt) {
    if signed {
        let magnitude = BigInt::from(1_u8) << (bits - 1);
        (-&magnitude, magnitude - 1)
    } else {
        (BigInt::from(0_u8), (BigInt::from(1_u8) << bits) - 1)
    }
}

pub(crate) fn promoted_integer_type(left: ScalarType, right: ScalarType) -> ScalarType {
    if left == ScalarType::Int || right == ScalarType::Int {
        return ScalarType::Int;
    }
    let left_bounds = scalar_integer_bounds(left).expect("integer operand has bounds");
    let right_bounds = scalar_integer_bounds(right).expect("integer operand has bounds");
    [
        ScalarType::Int8,
        ScalarType::Uint8,
        ScalarType::Int16,
        ScalarType::Uint16,
        ScalarType::Int32,
        ScalarType::Uint32,
        ScalarType::Int64,
        ScalarType::Uint64,
        ScalarType::Int128,
        ScalarType::Uint128,
    ]
    .into_iter()
    .find(|candidate| {
        let bounds = scalar_integer_bounds(*candidate).expect("fixed integer has bounds");
        bounds.0 <= left_bounds.0
            && bounds.0 <= right_bounds.0
            && bounds.1 >= left_bounds.1
            && bounds.1 >= right_bounds.1
    })
    .unwrap_or(ScalarType::Int)
}

pub(super) fn scalar_integer_bounds(ty: ScalarType) -> Option<(BigInt, BigInt)> {
    match ty {
        ScalarType::Int8 => Some(integer_bounds(8, true)),
        ScalarType::Int16 => Some(integer_bounds(16, true)),
        ScalarType::Int32 => Some(integer_bounds(32, true)),
        ScalarType::Int64 => Some(integer_bounds(64, true)),
        ScalarType::Int128 => Some(integer_bounds(128, true)),
        ScalarType::Uint8 => Some(integer_bounds(8, false)),
        ScalarType::Uint16 => Some(integer_bounds(16, false)),
        ScalarType::Uint32 => Some(integer_bounds(32, false)),
        ScalarType::Uint64 => Some(integer_bounds(64, false)),
        ScalarType::Uint128 => Some(integer_bounds(128, false)),
        _ => None,
    }
}
