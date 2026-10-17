use super::super::prelude::{
    ContextualConstant, ScalarType, SemanticUnit, SyntaxKind, SyntaxNode, contextual_constant,
};

/// Proves that a supported three-clause counter loop executes at least once.
pub(super) fn definitely_entered(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
    if node.kind != SyntaxKind::ForStatement {
        return false;
    }
    let [initializer, condition, _increment, body] = node.children.as_slice() else {
        return false;
    };
    if initializer.kind != SyntaxKind::Assignment || body.kind != SyntaxKind::Block {
        return false;
    }
    let [counter, initial_value] = initializer.children.as_slice() else {
        return false;
    };
    if counter.kind != SyntaxKind::Name
        || unit.source.text()[counter.span.end..initial_value.span.start].trim() != "="
    {
        return false;
    }
    let [left, right] = condition.children.as_slice() else {
        return false;
    };
    if condition.kind != SyntaxKind::BinaryExpression || left.kind != SyntaxKind::Name {
        return false;
    }
    let counter_binding =
        unit.flow_binding_ids
            .get(&(counter.span.file, counter.span.start, counter.span.end));
    let condition_binding =
        unit.flow_binding_ids
            .get(&(left.span.file, left.span.start, left.span.end));
    if counter_binding.is_none() || counter_binding != condition_binding {
        return false;
    }
    let Some(Ok(ContextualConstant::Integer(initial))) =
        contextual_constant(&unit.source, initial_value, ScalarType::Int)
    else {
        return false;
    };
    let Some(Ok(ContextualConstant::Integer(bound))) =
        contextual_constant(&unit.source, right, ScalarType::Int)
    else {
        return false;
    };
    // BinaryExpression stores operands only; the parser represents the operator
    // in the source span between them (the same convention used by inference).
    let operator = unit.source.text()[left.span.end..right.span.start].trim();
    match operator {
        "<" => initial < bound,
        "<=" => initial <= bound,
        _ => false,
    }
}
