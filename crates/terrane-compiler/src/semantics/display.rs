use super::diagnostics::node_text;
use super::member_inference::{descriptor_protocol_method, object_member_type};
use super::model::{FunctionContract, SemanticPackage, SemanticUnit, ValueType};
use crate::{
    InvocationMode, ScalarType,
    syntax::{SyntaxKind, SyntaxNode},
};

/// The same structural operation supplies explicit conversion and implicit text display.
pub(crate) fn display_method<'a>(
    unit: &'a SemanticUnit,
    value_type: &ValueType,
    at: usize,
) -> Option<&'a FunctionContract> {
    match value_type {
        ValueType::Reference(item) | ValueType::SharedReference(item) => {
            display_method(unit, item.value_type_ref(), at)
        }
        ValueType::TypeParameter(parameter) => {
            let bound = unit.type_parameter_at(at, parameter)?.bound.as_ref()?;
            display_method(unit, &ValueType::Object(bound.clone()), at)
        }
        ValueType::Object(identity) => {
            let method = descriptor_protocol_method(unit, identity, "to-string")?;
            let ValueType::Function(parameters, result, _) =
                object_member_type(unit, identity, "to-string", false)?
            else {
                return None;
            };
            (!method.is_unsafe
                && !method.is_async
                && method.written_invocation_mode == InvocationMode::Shared
                && method.generic_parameters.is_empty()
                && parameters.is_empty()
                && result.value_type_ref() == &ValueType::Scalar(ScalarType::String))
                .then_some(method)
        }
        _ => None,
    }
}

pub(crate) fn supports_display(unit: &SemanticUnit, value_type: &ValueType, at: usize) -> bool {
    match value_type {
        ValueType::Scalar(scalar) => *scalar != ScalarType::Bytes,
        ValueType::Descriptor(_) => true,
        ValueType::Reference(item) | ValueType::SharedReference(item) => {
            supports_display(unit, item.value_type_ref(), at)
        }
        ValueType::Union(arms) => arms.iter().all(|arm| supports_display(unit, arm, at)),
        ValueType::Optional(inner) => supports_display(unit, inner, at),
        _ => display_method(unit, value_type, at).is_some(),
    }
}

pub(super) fn implicit_display_methods<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    node: &SyntaxNode,
) -> Vec<&'a FunctionContract> {
    fn collect<'a>(
        unit: &'a SemanticUnit,
        value: &ValueType,
        at: usize,
        methods: &mut Vec<&'a FunctionContract>,
    ) {
        match value {
            ValueType::Union(arms) => {
                for arm in arms {
                    collect(unit, arm, at, methods);
                }
            }
            ValueType::Optional(inner) => collect(unit, inner, at, methods),
            _ => methods.extend(display_method(unit, value, at)),
        }
    }
    let operands: Vec<&SyntaxNode> = match node.kind {
        SyntaxKind::BinaryExpression if node.children.len() == 2 => {
            let left = &node.children[0];
            let right = &node.children[1];
            if unit.source.text()[left.span.end..right.span.start].trim() != "+"
                || super::expressions::infer_value_type(unit, node, &unit.typed_bindings)
                    .ok()
                    .flatten()
                    != Some(ValueType::Scalar(ScalarType::String))
            {
                return Vec::new();
            }
            vec![left, right]
        }
        SyntaxKind::CallExpression => {
            let [callee, arguments] = node.children.as_slice() else {
                return Vec::new();
            };
            let print = callee.kind == SyntaxKind::Name
                && package
                    .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
                    .is_some_and(|symbol| symbol.identity == "/core/output::print");
            let composition = if callee.kind == SyntaxKind::MemberExpression
                && let [receiver, member] = callee.children.as_slice()
            {
                matches!(node_text(&unit.source, member), "concat" | "join")
                    && super::member_inference::infer_receiver_value_type(
                        unit,
                        receiver,
                        &unit.typed_bindings,
                    )
                    .ok()
                    .flatten()
                        == Some(ValueType::Scalar(ScalarType::String))
            } else {
                false
            };
            if !print && !composition {
                return Vec::new();
            }
            arguments
                .children
                .iter()
                .map(|argument| argument.children.last().unwrap_or(argument))
                .collect()
        }
        _ => return Vec::new(),
    };
    let mut methods = Vec::new();
    for operand in operands {
        if let Ok(Some(value)) =
            super::expressions::infer_value_type(unit, operand, &unit.typed_bindings)
        {
            collect(unit, &value, operand.span.start, &mut methods);
        }
    }
    methods
}
