//! Source diagnostics for native values moved into lowering-owned try closures.
use super::super::prelude::*;
use super::liveness::Flow;

fn named_value(mut node: &SyntaxNode) -> &SyntaxNode {
    while node.kind == SyntaxKind::GroupExpression && node.children.len() == 1 {
        node = &node.children[0];
    }
    node
}

fn rebinds(unit: &SemanticUnit, node: &SyntaxNode, binding: Span) -> bool {
    (node.kind == SyntaxKind::Assignment
        && node
            .children
            .first()
            .and_then(|target| super::binding_identity_at(unit, target))
            == Some(binding))
        || node
            .children
            .iter()
            .any(|child| rebinds(unit, child, binding))
}

fn validate_moved_value(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    value: &SyntaxNode,
    flow: &Flow,
    explicit: bool,
) -> Result<(), SemanticFailure> {
    let value = named_value(value);
    if let Some(ty) = unit.inferred_value_type(value)
        && matches!(
            ty,
            ValueType::InvocationScopedNative { .. }
                | ValueType::Object(_)
                | ValueType::TypeParameter(_)
        )
        && !package.native_value_is_copyable(&ty)
        && (explicit
            || application_is_resource_owning(package, &ty)
            || super::field_has_native_loan(package, &ty))
        && let Some(binding) = super::binding_identity_at(unit, value)
        && binding.start < statement.span.start
        && rebinds(unit, statement, binding)
        && (package
            .binding_events
            .get(&span_key(binding))
            .is_some_and(|events| {
                events.iter().any(|event| {
                    matches!(event, BindingEvent::Read { span, .. }
                    if span.file == value.span.file
                        && value.span.end <= span.start
                        && statement.children.iter().any(|clause| {
                            let separate_handler = clause.kind == SyntaxKind::FinallyClause
                                && !(clause.span.start <= value.span.start
                                    && value.span.end <= clause.span.end);
                            let catch_after_body = clause.kind == SyntaxKind::CatchClause
                                && statement.children.iter().any(|body| {
                                    body.kind == SyntaxKind::Block
                                        && body.span.start <= value.span.start
                                        && value.span.end <= body.span.end
                                });
                            (separate_handler || catch_after_body)
                                && clause.span.start <= span.start
                                && span.end <= clause.span.end
                        }))
                })
            })
            || flow.reads_after_without_rewrite(statement.span, binding))
    {
        return Err(failure(
            &unit.source,
            "T0058",
            "a try closure cannot restore an outer non-Copy value after consuming it; acquire and consume the value inside the try or move consumption outside the try",
            value.span,
        ));
    }
    Ok(())
}

fn validate_consumption(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    node: &SyntaxNode,
    flow: &Flow,
) -> Result<(), SemanticFailure> {
    let receiver = if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && let Some((receiver, ty, member)) =
            super::super::diagnostics::typed_member_call(unit, callee)
        && super::super::diagnostics::member_invocation_mode_at(
            package, unit, receiver, &ty, &member,
        ) == InvocationMode::Consuming
    {
        Some(receiver)
    } else if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("move")
    {
        node.children.last()
    } else {
        None
    };
    if let Some(receiver) = receiver {
        validate_moved_value(package, unit, statement, receiver, flow, true)?;
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && let Some(contract) = function_contract_for_call(package, unit, callee)
    {
        let projected = super::super::bindings::projected_function_for_call(
            package,
            unit,
            callee,
            crate::syntax::call_is_unsafe(node),
        );
        for (index, parameter) in contract.parameters.iter().enumerate() {
            let borrowed = projected
                .and_then(|function| function.parameters.get(index))
                .is_some_and(|parameter| parameter.borrowed)
                || parameter.element_value_type().is_some_and(|ty| {
                    matches!(
                        ty,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::SharedReference(_)
                    )
                });
            if !borrowed
                && let Some(value) = argument_for_parameter(unit, arguments, contract, index)
            {
                validate_moved_value(package, unit, statement, value, flow, false)?;
            }
        }
    }
    for child in &node.children {
        if !matches!(
            child.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            validate_consumption(package, unit, statement, child, flow)?;
        }
    }
    Ok(())
}

fn contains_try(node: &SyntaxNode) -> bool {
    node.kind == SyntaxKind::TryStatement
        || node.children.iter().any(|child| {
            !matches!(
                child.kind,
                SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
            ) && contains_try(child)
        })
}

fn validate_in_function(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    flow: &Flow,
) -> Result<(), SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        return validate(package, unit, node);
    }
    if node.kind == SyntaxKind::TryStatement {
        validate_consumption(package, unit, node, node, flow)?;
    }
    for child in &node.children {
        validate_in_function(package, unit, child, flow)?;
    }
    Ok(())
}

pub(super) fn validate(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) && contains_try(node)
    {
        let flow = Flow::new(package, unit, node);
        for child in &node.children {
            validate_in_function(package, unit, child, &flow)?;
        }
        return Ok(());
    }
    for child in &node.children {
        validate(package, unit, child)?;
    }
    Ok(())
}
