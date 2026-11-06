//! Source diagnostics for native values moved into lowering-owned try closures.
use super::super::prelude::*;

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
) -> Result<(), SemanticFailure> {
    let value = named_value(value);
    if let Some(ty @ ValueType::InvocationScopedNative { .. }) = unit.inferred_value_type(value)
        && !package.native_value_is_copyable(&ty)
        && let Some(binding) = super::binding_identity_at(unit, value)
        && binding.start < statement.span.start
        && rebinds(unit, statement, binding)
    {
        return Err(failure(
            &unit.source,
            "T0058",
            "a looped try closure cannot restore an outer native value after consuming it; acquire the value inside the try or move consumption outside the try",
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
) -> Result<(), SemanticFailure> {
    let receiver = if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && let Some(ty) = unit.inferred_value_type(receiver)
        && super::super::diagnostics::member_invocation_mode_at(
            package,
            unit,
            receiver,
            &ty,
            node_text(&unit.source, member),
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
        validate_moved_value(package, unit, statement, receiver)?;
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
                validate_moved_value(package, unit, statement, value)?;
            }
        }
    }
    for child in &node.children {
        if !matches!(
            child.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            validate_consumption(package, unit, statement, child)?;
        }
    }
    Ok(())
}

pub(super) fn validate(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    repeating: bool,
) -> Result<(), SemanticFailure> {
    let repeating = if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        false
    } else if matches!(
        node.kind,
        SyntaxKind::WhileStatement | SyntaxKind::ForStatement
    ) {
        repeating
            || node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
                .is_some_and(super::super::scope_validation::loop_body_may_repeat)
    } else {
        repeating
    };
    if repeating && node.kind == SyntaxKind::TryStatement {
        validate_consumption(package, unit, node, node)?;
    }
    for child in &node.children {
        validate(package, unit, child, repeating)?;
    }
    Ok(())
}
