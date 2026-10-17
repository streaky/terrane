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
    pub(super) fn validate_shared_storage_reentry(
        &self,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        bindings: &[TypedBinding],
    ) -> Result<(), SemanticFailure> {
        if node.kind != SyntaxKind::CallExpression {
            return Ok(());
        }
        let [callee, arguments] = node.children.as_slice() else {
            return Ok(());
        };
        if callee.kind != SyntaxKind::MemberExpression {
            return Ok(());
        }
        let [receiver, _member] = callee.children.as_slice() else {
            return Ok(());
        };
        let receiver_type = infer_value_type(unit, receiver, bindings).ok().flatten();
        if !matches!(receiver_type, Some(ValueType::SharedReference(_))) {
            return Ok(());
        }
        let Some(contract) = function_contract_for_call(self, unit, callee).filter(|contract| {
            contract.owner.is_some()
                && !contract.is_static
                && self.units.iter().any(|source_unit| {
                    source_unit
                        .functions
                        .iter()
                        .any(|method| method.span == contract.span)
                })
        }) else {
            return Ok(());
        };
        if contract.is_async
            || !matches!(
                contract.written_invocation_mode,
                super::InvocationMode::Shared | super::InvocationMode::Mutable
            )
        {
            return Ok(());
        }
        let receiver_binding = self.storage_name_binding(unit, receiver);
        let receiver_owner = self.expression_storage_owner(unit, receiver, 0);
        let mut positional = 0;
        let mut named_seen = false;
        for argument in &arguments.children {
            let name = argument
                .children
                .first()
                .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
            let Ok(parameter) = resolve_call_parameter(
                unit,
                argument,
                name,
                contract,
                &mut positional,
                &mut named_seen,
            ) else {
                continue;
            };
            if !matches!(parameter.value_type, Some(ValueType::SharedReference(_))) {
                continue;
            }
            let value = argument.children.last().unwrap_or(argument);
            let same_binding = receiver_binding.is_some()
                && self.storage_name_binding(unit, value) == receiver_binding;
            let same_owner = receiver_owner.is_some()
                && self.expression_storage_owner(unit, value, 0) == receiver_owner;
            if same_binding || same_owner {
                return Err(failure(
                    &unit.source,
                    "T0143",
                    "a guarded shared-reference method cannot receive an alias of its receiver's shared storage",
                    value.span,
                ));
            }
        }
        Ok(())
    }
    fn storage_name_binding(&self, unit: &SemanticUnit, expression: &SyntaxNode) -> Option<Span> {
        let mut expression = expression;
        while expression.kind == SyntaxKind::GroupExpression {
            expression = expression.children.first()?;
        }
        if expression.kind != SyntaxKind::Name {
            return None;
        }
        self.resolve_name_at(
            unit,
            expression.span.start,
            node_text(&unit.source, expression),
        )
        .and_then(|symbol| symbol.declaration_span.or(symbol.binding_span))
    }

    fn expression_storage_owner(
        &self,
        unit: &SemanticUnit,
        expression: &SyntaxNode,
        depth: usize,
    ) -> Option<Span> {
        if depth > unit.typed_bindings.len() {
            return None;
        }
        if expression.kind == SyntaxKind::GroupExpression {
            return expression
                .children
                .first()
                .and_then(|child| self.expression_storage_owner(unit, child, depth));
        }
        if expression.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, expression).as_deref() == Some("shared ref")
        {
            return expression
                .children
                .last()
                .and_then(|child| self.expression_storage_owner(unit, child, depth));
        }
        if expression.kind != SyntaxKind::Name {
            return None;
        }
        let binding_span = self.storage_name_binding(unit, expression)?;
        let binding = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == binding_span)?;
        let declaration = find_node_by_span(&unit.tree.root, binding_span);
        let initialized = declaration.is_some_and(|declaration| {
            matches!(
                declaration.kind,
                SyntaxKind::Binding | SyntaxKind::Assignment
            )
        });
        let writes = self.storage_binding_writes(unit, &unit.tree.root, binding_span);
        if writes > 1 || (!initialized && writes > 0) {
            return None;
        }
        if matches!(binding.value_type, ValueType::SharedReference(_))
            && initialized
            && let Some(initializer) = declaration.and_then(binding_initializer)
        {
            // Only chase stable handle definitions; never reuse initial provenance
            // after replacement, including indirectly through another handle.
            return self.expression_storage_owner(unit, initializer, depth + 1);
        }
        Some(binding_span)
    }

    fn storage_binding_writes(&self, unit: &SemanticUnit, node: &SyntaxNode, owner: Span) -> usize {
        let written = matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && node.children.first().is_some_and(|left| {
                left.kind == SyntaxKind::Name
                    && self
                        .resolve_name_at(unit, left.span.start, node_text(&unit.source, left))
                        .and_then(|symbol| symbol.declaration_span.or(symbol.binding_span))
                        == Some(owner)
            });
        usize::from(written)
            + node
                .children
                .iter()
                .map(|child| self.storage_binding_writes(unit, child, owner))
                .sum::<usize>()
    }
}
