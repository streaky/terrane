use super::prelude::*;

fn collect_shared_operands(unit: &SemanticUnit, node: &SyntaxNode, operands: &mut Vec<Span>) {
    if node.kind == SyntaxKind::UnaryExpression
        && let Some(operand) = node.children.last()
        && operand.kind == SyntaxKind::Name
        && unary_operator_text(unit, node).as_deref() == Some("shared ref")
    {
        operands.push(operand.span);
    }
    for child in &node.children {
        collect_shared_operands(unit, child, operands);
    }
}

impl SemanticPackage {
    pub(crate) fn binding_uses_shared_storage(
        &self,
        unit: &SemanticUnit,
        binding: &TypedBinding,
    ) -> bool {
        if matches!(
            binding.value_type,
            ValueType::Reference(_) | ValueType::SharedReference(_)
        ) {
            return false;
        }
        // Cache only syntax: reaching bindings can change as flow analysis is rebuilt.
        unit.shared_reference_operands
            .get_or_init(|| {
                let mut operands = Vec::new();
                collect_shared_operands(unit, &unit.tree.root, &mut operands);
                operands
            })
            .iter()
            .any(|operand| {
                self.resolve_name_at(
                    unit,
                    operand.start,
                    &unit.source.text()[operand.start..operand.end],
                )
                .and_then(|symbol| symbol.declaration_span)
                    == Some(binding.span)
            })
    }

    pub(crate) fn reference_owner_uses_shared_storage(
        &self,
        unit: &SemanticUnit,
        owner: Span,
    ) -> bool {
        unit.typed_bindings
            .iter()
            .find(|binding| binding.span == owner)
            .is_some_and(|binding| {
                matches!(binding.value_type, ValueType::SharedReference(_))
                    || self.binding_uses_shared_storage(unit, binding)
            })
    }
}
