use super::super::prelude::*;

impl Emitter<'_> {
    fn task_binding_writes(&self, binding: &TypedBinding) -> Vec<&SyntaxNode> {
        fn collect<'a>(
            node: &'a SyntaxNode,
            unit: &SemanticUnit,
            identity: crate::Span,
            writes: &mut Vec<&'a SyntaxNode>,
        ) {
            let key = (node.span.file, node.span.start, node.span.end);
            if node.kind == SyntaxKind::Binding
                && unit.flow_binding_ids.get(&key) == Some(&identity)
                && unit
                    .flow_types
                    .get(&key)
                    .is_some_and(|value_type| matches!(value_type, ValueType::Task(_, _)))
                && let Some(value) = node.children.last()
            {
                writes.push(value);
            } else if node.kind == SyntaxKind::Assignment
                && let [target, value] = node.children.as_slice()
                && target.kind == SyntaxKind::Name
                && unit.flow_binding_ids.get(&(
                    target.span.file,
                    target.span.start,
                    target.span.end,
                )) == Some(&identity)
                && unit
                    .flow_types
                    .get(&key)
                    .is_some_and(|value_type| matches!(value_type, ValueType::Task(_, _)))
            {
                writes.push(value);
            }
            for child in &node.children {
                collect(child, unit, identity, writes);
            }
        }
        let mut writes = Vec::new();
        collect(&self.unit.tree.root, self.unit, binding.span, &mut writes);
        writes
    }

    // Storage ABI is uniform across all writes, unlike the flow-sensitive error set
    // at an individual await. A known-safe write can still occupy a Result slot.
    pub(super) fn task_value_throws(&self, value: &SyntaxNode) -> bool {
        self.task_value_throws_inner(value, &mut BTreeSet::new())
    }

    fn task_value_throws_inner(
        &self,
        value: &SyntaxNode,
        visited: &mut BTreeSet<crate::Span>,
    ) -> bool {
        let value = crate::semantics::task_transfer_source(self.unit, value);
        if value.kind == SyntaxKind::Name
            && let Some(binding) = self.local_typed_binding(value)
            && matches!(binding.value_type, ValueType::Task(_, _))
            && visited.insert(binding.span)
        {
            return self
                .task_binding_writes(binding)
                .into_iter()
                .any(|write| self.task_value_throws_inner(write, visited));
        }
        if value.kind == SyntaxKind::CallExpression
            && let Some(callee) = value.children.first()
        {
            let unsafe_call = crate::syntax::call_is_unsafe(value);
            let contract = self.contract_for_call(callee, unsafe_call);
            if self
                .projected_function_for_call(callee, unsafe_call)
                .is_some()
                || contract.as_ref().is_some_and(|contract| contract.throws)
                || (contract.is_none()
                    && self.value_type(callee).is_some_and(|value_type| {
                        matches!(value_type, ValueType::AsyncFunction(_, _, _, effects)
                            if effects.requires_throwing_abi())
                    }))
            {
                return true;
            }
        }
        !crate::semantics::task_expression_throwables(self.package, self.unit, value).is_empty()
    }

    fn task_binding_throws(&self, binding: &TypedBinding) -> bool {
        let mut visited = BTreeSet::from([binding.span]);
        self.task_binding_writes(binding)
            .into_iter()
            .any(|value| self.task_value_throws_inner(value, &mut visited))
    }
    pub(super) fn task_binding_rust_type(&self, binding: &TypedBinding) -> String {
        let ValueType::Task(result, transferability) = self.flow_binding_type(binding) else {
            unreachable!("task storage must retain its task value type")
        };
        let result = rust_element_type(self.package, result.clone());
        let output = if self.task_binding_throws(binding) {
            format!("Result<{result}, TerraneError>")
        } else {
            result
        };
        format!(
            "std::pin::Pin<Box<dyn Future<Output = {output}>{} + '_>>",
            if transferability == TaskTransferability::Local {
                ""
            } else {
                " + Send"
            }
        )
    }
    pub(super) fn task_binding_is_heterogeneous(&self, binding: &TypedBinding) -> bool {
        fn collect_origin(
            emitter: &Emitter<'_>,
            node: &SyntaxNode,
            origins: &mut std::collections::BTreeSet<crate::Span>,
            visited: &mut std::collections::BTreeSet<crate::Span>,
        ) {
            let node = crate::semantics::task_transfer_source(emitter.unit, node);
            if node.kind == SyntaxKind::CallExpression
                && let Some(callee) = node.children.first()
            {
                if let Some(contract) = emitter.contract_for_call(callee, false) {
                    origins.insert(contract.span);
                } else {
                    origins.insert(node.span);
                }
                return;
            }
            if node.kind == SyntaxKind::Name
                && let Some(source) = emitter.local_typed_binding(node)
                && matches!(source.value_type, ValueType::Task(_, _))
                && visited.insert(source.span)
            {
                // A transferred value retains its source slot's physical carrier,
                // including origins assigned later in that slot's lifetime.
                for value in emitter.task_binding_writes(source) {
                    collect_origin(emitter, value, origins, visited);
                }
            }
        }
        let mut origins = std::collections::BTreeSet::new();
        let mut visited = std::collections::BTreeSet::new();
        for write in self.task_binding_writes(binding) {
            collect_origin(self, write, &mut origins, &mut visited);
        }
        origins.len() > 1
    }

    pub(super) fn erase_task_write(
        &self,
        binding: &TypedBinding,
        value: &str,
        source: &SyntaxNode,
    ) -> String {
        let source = crate::semantics::task_transfer_source(self.unit, source);
        if !self.task_binding_is_heterogeneous(binding) {
            return value.to_owned();
        }
        let task_throws = self.task_binding_throws(binding);
        let source_throws = self.task_value_throws(source);
        if task_throws && !source_throws {
            format!(
                "{{ let __terrane_task_future = {value}; Box::pin(async move {{ Ok(__terrane_task_future.await) }}) }}"
            )
        } else if self
            .local_typed_binding(source)
            .is_some_and(|source| self.task_binding_is_heterogeneous(source))
        {
            value.to_owned()
        } else {
            format!("Box::pin({value})")
        }
    }
}
