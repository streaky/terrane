use super::prelude::*;

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
        self.node_shares_binding(unit, &unit.tree.root, binding)
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

    fn node_shares_binding(
        &self,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        binding: &TypedBinding,
    ) -> bool {
        if node.kind == SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last()
            && operand.kind == SyntaxKind::Name
            && unary_operator_text(unit, node).as_deref() == Some("shared ref")
            && self
                .resolve_name_at(
                    unit,
                    operand.span.start,
                    &unit.source.text()[operand.span.start..operand.span.end],
                )
                .and_then(|symbol| symbol.declaration_span)
                == Some(binding.span)
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| self.node_shares_binding(unit, child, binding))
    }
}
