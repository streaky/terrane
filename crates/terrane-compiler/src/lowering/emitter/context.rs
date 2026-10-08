use super::super::prelude::*;
use crate::Symbol;

fn compiler_singleton_rust_name(symbol: Option<&Symbol>) -> Option<String> {
    let symbol = symbol?;
    if let Some(encoding) = symbol.identity.strip_prefix("/core/encodings::") {
        let variant = match encoding {
            "utf8" => "Utf8",
            "utf16-le" => "Utf16Le",
            "utf16-be" => "Utf16Be",
            "utf32-le" => "Utf32Le",
            "utf32-be" => "Utf32Be",
            _ => return None,
        };
        return Some(format!("terrane_string_support::Encoding::{variant}"));
    }
    let policy = match symbol.compiler_identity() {
        "/core/concurrency::channel-block" => "Block",
        "/core/concurrency::channel-fail-send" => "FailSend",
        "/core/concurrency::channel-drop-newest" => "DropNewest",
        "/core/concurrency::channel-drop-oldest" => "DropOldest",
        _ => return None,
    };
    Some(format!("TerraneChannelOverflow::{policy}"))
}

impl<'unit> Emitter<'unit> {
    pub(super) fn name(&self, node: &SyntaxNode) -> String {
        let source_name = self.text(node);
        if source_name == "none" {
            return "()".to_owned();
        }
        if source_name == "this" {
            return self.receiver_capture_name(node);
        }
        if let Some(name) = self.narrowed_name(node) {
            return name;
        }
        if let Some((_, local)) = self
            .namespace_initializer
            .as_ref()
            .filter(|(name, _)| name == source_name)
        {
            return local.clone();
        }
        let resolved = self
            .package
            .resolve_name_at(self.unit, node.span.start, source_name);
        if let Some(singleton) = compiler_singleton_rust_name(resolved) {
            return singleton;
        }
        let Some(symbol) = self
            .package
            .resolve_name_at(self.unit, node.span.start, source_name)
        else {
            return rust_name(source_name);
        };
        if symbol.kind != SymbolKind::Binding {
            return rust_name(source_name);
        }
        if symbol.global {
            let storage = global_binding_name(&symbol.name);
            let failure = self.uninitialized_binding_failure(node);
            return format!(
                "{storage}.lock().expect(\"program-global lock poisoned\").clone().unwrap_or_else(|| {failure})"
            );
        }
        let Some(span) = symbol.declaration_span else {
            return rust_name(source_name);
        };
        if let Some(binding) = self.local_typed_binding(node)
            && self.binding_may_be_unassigned(binding)
            && !self.assignment_target
        {
            let access = self.available_storage_reference(binding, node, false);
            if self.reference_backed(binding) {
                return format!(
                    "({{ let __terrane_value = {access}.lock().expect(\"reference lock poisoned\").clone(); __terrane_value }})"
                );
            }
            if self.value_type_owns_resource(&self.flow_binding_type(binding)) {
                return access;
            }
            return if self
                .value_type(node)
                .as_ref()
                .is_some_and(rust_value_is_copy)
            {
                format!("*{access}")
            } else {
                format!("{access}.clone()")
            };
        }
        if let Some(binding) = self.local_typed_binding(node)
            && self.reference_backed(binding)
        {
            return format!(
                "({{ let __terrane_value = {}.lock().expect(\"reference lock poisoned\").clone(); __terrane_value }})",
                self.local_storage_name(node)
            );
        }
        let name = namespace_binding_name(span.file, &symbol.name);
        if self.lazy_namespace_binding_type(node).is_some() {
            format!("&*{name}")
        } else if self.is_namespace_binding_span(span) {
            name
        } else {
            self.local_storage_name(node)
        }
    }

    fn receiver_capture_name(&self, node: &SyntaxNode) -> String {
        if self.closure_depth == 0 {
            return "self".to_owned();
        }
        let closure = self
            .unit
            .functions
            .iter()
            .filter(|function| {
                function.is_anonymous
                    && function.span.start <= node.span.start
                    && node.span.end <= function.span.end
                    && function.captures.iter().any(|capture| capture == "this")
            })
            .min_by_key(|function| function.span.end - function.span.start);
        if let Some(closure) = closure
            && let Some(binding) = self.unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == "this"
                    && binding.is_visible_at(self.source.id(), closure.span.start)
            })
        {
            return self.binding_storage_name(binding).to_owned();
        }
        "this".to_owned()
    }

    pub(super) fn raw_storage_name(&self, node: &SyntaxNode) -> String {
        let source_name = self.text(node);
        if source_name == "this" && self.closure_depth == 0 {
            "self".to_owned()
        } else {
            self.local_storage_name(node)
        }
    }

    pub(super) fn uninitialized_binding_failure(&self, node: &SyntaxNode) -> String {
        let (line, column) = self.source.line_column(node.span.start);
        format!(
            "__terrane_uninitialized_binding({:?}, {:?}, {line}, {column})",
            self.text(node),
            display_path(self.source.path())
        )
    }

    pub(super) fn namespace_name(&self, node: &SyntaxNode) -> String {
        self.package
            .resolve_name_at(self.unit, node.span.start, self.text(node))
            .and_then(|symbol| {
                symbol
                    .declaration_span
                    .map(|span| namespace_binding_name(span.file, &symbol.name))
            })
            .unwrap_or_else(|| rust_name(self.text(node)))
    }
    pub(super) fn local_binding_identity(&self, node: &SyntaxNode) -> Option<crate::Span> {
        let symbol_span = self
            .package
            .resolve_name_at(self.unit, node.span.start, self.text(node))
            .filter(|symbol| symbol.kind == SymbolKind::Binding && !symbol.global)?
            .declaration_span?;
        Some(
            self.unit
                .flow_binding_ids
                .get(&(node.span.file, node.span.start, node.span.end))
                .copied()
                .unwrap_or(symbol_span),
        )
    }

    pub(super) fn binding_storage_name(&self, binding: &TypedBinding) -> &'unit str {
        crate::lowering::binding_storage_rust_name(self.unit, binding)
    }

    pub(super) fn parameter_source_name(&self, name: &str, span: crate::Span) -> String {
        let unit = if self.unit.source.id() == span.file {
            self.unit
        } else {
            self.package
                .units
                .iter()
                .find(|unit| unit.source.id() == span.file)
                .unwrap_or(self.unit)
        };
        crate::lowering::parameter_source_rust_name(unit, name, span)
    }

    pub(super) fn storage_name_at(&self, name: &str, span: crate::Span) -> String {
        crate::lowering::StorageNames::for_unit(self.unit)
            .at(span)
            .map_or_else(|| rust_name(name), str::to_owned)
    }

    pub(super) fn local_storage_name(&self, node: &SyntaxNode) -> String {
        self.local_binding_identity(node).map_or_else(
            || rust_name(self.text(node)),
            |span| self.storage_name_at(self.text(node), span),
        )
    }

    pub(super) fn local_typed_binding(&self, node: &SyntaxNode) -> Option<&TypedBinding> {
        let identity = self.local_binding_identity(node)?;
        self.unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == identity)
    }

    pub(super) fn binding_scalar_storage_type(&self, binding: &TypedBinding) -> Option<ScalarType> {
        binding
            .storage_type
            .filter(|_| !matches!(self.flow_binding_type(binding), ValueType::Union(_)))
            .filter(|_| {
                !self.reference_backed(binding) && !self.binding_is_reference_owner(binding)
            })
            .filter(|_| {
                !binding_span_is_mutated(
                    self.package,
                    self.unit,
                    binding.span,
                    true,
                    ClosureWrites::Include,
                )
            })
    }

    pub(super) fn binding_may_be_unassigned(&self, binding: &TypedBinding) -> bool {
        self.unit
            .flow_availability
            .iter()
            .any(|(key, availability)| {
                *availability == crate::semantics::FlowAvailability::MayBeUnassigned
                    && self.unit.flow_binding_ids.get(key) == Some(&binding.span)
            })
    }

    pub(super) fn initializer_consumes_binding(
        &self,
        node: &SyntaxNode,
        identity: crate::Span,
    ) -> bool {
        if node.kind == SyntaxKind::UnaryExpression
            && self.unary_operator(node).as_deref() == Some("move")
            && let Some(mut operand) = node.children.last()
        {
            while operand.kind == SyntaxKind::GroupExpression
                && let Some(inner) = operand.children.first()
            {
                operand = inner;
            }
            if self.local_binding_identity(operand) == Some(identity) {
                return true;
            }
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && self.local_binding_identity(receiver) == Some(identity)
            && (self
                .contract_for_call(callee, false)
                .is_some_and(|contract| {
                    contract.written_invocation_mode == InvocationMode::Consuming
                })
                || self
                    .receiver_value_type(receiver)
                    .is_some_and(|value_type| {
                        let ValueType::Object(object) = value_type else {
                            return false;
                        };
                        self.package
                            .projection
                            .method(&object.namespace, &object.name, self.text(member), false)
                            .is_some_and(|method| {
                                matches!(
                                    method.receiver,
                                    Some(crate::rust_interop::projection::Receiver::Move)
                                )
                            })
                    }))
        {
            return true;
        }
        if matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            return node.kind == SyntaxKind::AnonymousFunction
                && self
                    .unit
                    .functions
                    .iter()
                    .find(|contract| contract.span == node.span)
                    .is_some_and(|contract| {
                        self.unit
                            .typed_bindings
                            .iter()
                            .find(|binding| binding.span == identity)
                            .is_some_and(|binding| {
                                contract.captures.contains(&binding.name)
                                    && self.value_type_owns_resource(&binding.value_type)
                            })
                    });
        }
        node.children
            .iter()
            .any(|child| self.initializer_consumes_binding(child, identity))
    }

    pub(super) fn binding_transferred_to_callable(
        &self,
        node: &SyntaxNode,
        identity: crate::Span,
        before: usize,
    ) -> bool {
        if node.span.start >= before {
            return false;
        }
        if matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) && !(node.span.start <= identity.start && identity.end <= node.span.end)
        {
            return false;
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && let Some(name_index) = node.children.iter().position(|child| child.kind == SyntaxKind::Name)
            && let Some(initializer) = binding_initializer(node, name_index)
            && initializer.kind == SyntaxKind::MemberExpression
            && let Some(receiver) = initializer.children.first()
            && receiver.kind == SyntaxKind::Name
            && self.local_binding_identity(receiver) == Some(identity)
            && self.contract_for_call(initializer, false).is_some()
            && !self.binding_value_is_reused(receiver)
            && !self.non_consuming_capture_read(receiver)
            && self.local_typed_binding(receiver).is_some_and(|binding| {
                !self.reference_backed(binding) && !self.binding_may_be_unassigned(binding)
            })
            && self.value_type(receiver).is_some_and(|value_type| {
                !rust_value_is_copy(&value_type)
                    && !matches!(&value_type, ValueType::Object(object) if self.object_requires_separation(object))
            })
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| self.binding_transferred_to_callable(child, identity, before))
    }

    pub(super) fn available_storage_reference(
        &self,
        binding: &TypedBinding,
        node: &SyntaxNode,
        mutable: bool,
    ) -> String {
        let storage = self.local_storage_name(node);
        if !self.binding_may_be_unassigned(binding) {
            return format!("&{}{storage}", if mutable { "mut " } else { "" });
        }
        let access = if mutable { "as_mut" } else { "as_ref" };
        if self
            .unit
            .flow_availability
            .get(&(node.span.file, node.span.start, node.span.end))
            == Some(&crate::semantics::FlowAvailability::MayBeUnassigned)
        {
            let failure = self.uninitialized_binding_failure(node);
            format!("{storage}.{access}().unwrap_or_else(|| {failure})")
        } else {
            format!("{storage}.{access}().expect(\"flow-proven available binding\")")
        }
    }

    pub(super) fn owned_storage_value(&self, node: &SyntaxNode) -> String {
        let storage = self.raw_storage_name(node);
        let Some(binding) = self
            .local_typed_binding(node)
            .filter(|binding| self.binding_may_be_unassigned(binding))
        else {
            return storage;
        };
        if rust_value_is_copy(&self.flow_binding_type(binding)) {
            return format!(
                "*({})",
                self.available_storage_reference(binding, node, false)
            );
        }
        if self
            .unit
            .flow_availability
            .get(&(node.span.file, node.span.start, node.span.end))
            == Some(&crate::semantics::FlowAvailability::MayBeUnassigned)
        {
            format!(
                "{storage}.take().unwrap_or_else(|| {})",
                self.uninitialized_binding_failure(node)
            )
        } else {
            format!("{storage}.take().expect(\"flow-proven availability\")")
        }
    }
    pub(super) fn is_throwable_value(&self, node: &SyntaxNode) -> bool {
        matches!(
            self.value_type(node),
            Some(ValueType::Object(identity))
                if identity.namespace == "/core/errors" && identity.name == "throwable"
        )
    }

    fn list_append_binding(&self, node: &SyntaxNode) -> Option<crate::Span> {
        let [callee, arguments] = node.children.as_slice() else {
            return None;
        };
        let [receiver, member] = callee.children.as_slice() else {
            return None;
        };
        (node.kind == SyntaxKind::CallExpression
            && callee.kind == SyntaxKind::MemberExpression
            && receiver.kind == SyntaxKind::Name
            && self.text(member) == "append"
            && arguments.children.len() == 1)
            .then_some(())?;
        let binding = self.local_typed_binding(receiver)?;
        matches!(binding.value_type, ValueType::List(_)).then_some(binding.span)
    }

    pub(super) fn append_only_list_bindings(
        &self,
        condition: &SyntaxNode,
        block: &SyntaxNode,
    ) -> Vec<crate::Span> {
        fn collect(
            emitter: &Emitter<'_>,
            node: &SyntaxNode,
            statement_position: bool,
            candidates: &mut Vec<crate::Span>,
        ) {
            if statement_position
                && let Some(binding) = emitter.list_append_binding(node)
                && !candidates.contains(&binding)
            {
                candidates.push(binding);
            }
            for child in &node.children {
                collect(emitter, child, node.kind == SyntaxKind::Block, candidates);
            }
        }

        fn uses(
            emitter: &Emitter<'_>,
            node: &SyntaxNode,
            statement_position: bool,
            binding: crate::Span,
        ) -> (usize, usize) {
            let reference = usize::from(
                node.kind == SyntaxKind::Name
                    && emitter
                        .local_typed_binding(node)
                        .is_some_and(|candidate| candidate.span == binding),
            );
            let append = usize::from(
                statement_position && emitter.list_append_binding(node) == Some(binding),
            );
            node.children
                .iter()
                .fold((reference, append), |(references, appends), child| {
                    let (child_references, child_appends) =
                        uses(emitter, child, node.kind == SyntaxKind::Block, binding);
                    (references + child_references, appends + child_appends)
                })
        }

        let mut candidates = Vec::new();
        collect(self, block, false, &mut candidates);
        candidates.retain(|binding| {
            let Some(binding) = self
                .unit
                .typed_bindings
                .iter()
                .find(|candidate| candidate.span == *binding)
                .filter(|candidate| {
                    candidate.is_visible_at(self.source.id(), condition.span.start)
                })
            else {
                return false;
            };
            if self
                .unit
                .flow_availability
                .iter()
                .any(|(key, availability)| {
                    key.0 == block.span.file
                        && block.span.start <= key.1
                        && key.2 <= block.span.end
                        && *availability == crate::semantics::FlowAvailability::MayBeUnassigned
                        && self.unit.flow_binding_ids.get(key) == Some(&binding.span)
                })
            {
                return false;
            }
            let (condition_references, _) = uses(self, condition, false, binding.span);
            let (references, appends) = uses(self, block, false, binding.span);
            condition_references == 0 && appends > 0 && references == appends
        });
        candidates
    }

    fn contains_loop_early_exit(&self, node: &SyntaxNode, loop_depth: usize) -> bool {
        if matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            return false;
        }
        if matches!(
            node.kind,
            SyntaxKind::ReturnStatement | SyntaxKind::ThrowStatement
        ) || (node.kind == SyntaxKind::BreakStatement && loop_depth == 0)
            || (node.kind == SyntaxKind::CallExpression
                && node
                    .children
                    .first()
                    .is_some_and(|callee| self.is_builtin(callee, "/core/process::exit")))
        {
            return true;
        }
        let child_loop_depth = loop_depth
            + usize::from(matches!(
                node.kind,
                SyntaxKind::WhileStatement | SyntaxKind::ForStatement
            ));
        node.children
            .iter()
            .any(|child| self.contains_loop_early_exit(child, child_loop_depth))
    }

    pub(super) fn while_capacity_hint(
        &self,
        condition: &SyntaxNode,
        block: &SyntaxNode,
    ) -> Option<(String, String)> {
        self.list_append_capacity_hint(condition, None, block)
            .map(|(_, start, end, _)| (start, end))
    }

    pub(super) fn for_capacity_hint(
        &self,
        condition: &SyntaxNode,
        update: &SyntaxNode,
        block: &SyntaxNode,
    ) -> Option<(String, String)> {
        self.list_append_capacity_hint(condition, Some(update), block)
            .map(|(_, start, end, _)| (start, end))
    }

    fn list_append_capacity_hint(
        &self,
        condition: &SyntaxNode,
        update: Option<&SyntaxNode>,
        block: &SyntaxNode,
    ) -> Option<(String, String, String, bool)> {
        fn mutation_count(
            emitter: &Emitter<'_>,
            node: &SyntaxNode,
            binding: &TypedBinding,
        ) -> usize {
            let own = usize::from(
                matches!(
                    node.kind,
                    SyntaxKind::Assignment | SyntaxKind::PostfixExpression
                ) && node.children.first().is_some_and(|target| {
                    emitter
                        .local_typed_binding(target)
                        .is_some_and(|target_binding| target_binding.span == binding.span)
                }),
            );
            own + node
                .children
                .iter()
                .map(|child| mutation_count(emitter, child, binding))
                .sum::<usize>()
        }

        fn is_direct_increment(
            emitter: &Emitter<'_>,
            statement: &SyntaxNode,
            binding: &TypedBinding,
        ) -> bool {
            statement.kind == SyntaxKind::PostfixExpression
                && statement.children.first().is_some_and(|target| {
                    emitter
                        .local_typed_binding(target)
                        .is_some_and(|target_binding| target_binding.span == binding.span)
                })
                && emitter.source.text()[statement.span.start..statement.span.end]
                    .trim_end()
                    .ends_with("++")
        }

        let [left, right] = condition.children.as_slice() else {
            return None;
        };
        (self.source.text()[left.span.end..right.span.start].trim() == "<").then_some(())?;
        let binding = self.local_typed_binding(left)?;
        let ValueType::Scalar(storage) = binding.value_type else {
            return None;
        };
        let (signed, width) = fixed_integer_shape(storage)?;
        let declaration = find_node_by_span(&self.unit.tree.root, binding.span)?;
        let name_index = declaration
            .children
            .iter()
            .position(|child| child.kind == SyntaxKind::Name)?;
        let initializer = binding_initializer(declaration, name_index)?;
        let ContextualConstant::Integer(lower) =
            contextual_constant(self.source, initializer, storage)?.ok()?
        else {
            return None;
        };
        (lower == BigInt::from(0_u8)).then_some(())?;
        let update_mutations = update.map_or(0, |update| mutation_count(self, update, binding));
        (mutation_count(self, block, binding) + update_mutations == 1).then_some(())?;
        let direct_increment_count = update.map_or_else(
            || {
                block
                    .children
                    .iter()
                    .filter(|statement| is_direct_increment(self, statement, binding))
                    .count()
            },
            |update| usize::from(is_direct_increment(self, update, binding)),
        );
        (direct_increment_count == 1).then_some(())?;
        (!self.contains_loop_early_exit(block, 0)).then_some(())?;

        let end = if right.kind == SyntaxKind::Name {
            let upper = self.local_typed_binding(right)?;
            let ValueType::Scalar(upper_type) = upper.value_type else {
                return None;
            };
            fixed_integer_shape(upper_type)?;
            let upper_update_mutations =
                update.map_or(0, |update| mutation_count(self, update, upper));
            (mutation_count(self, block, upper) + upper_update_mutations == 0).then_some(())?;
            self.name(right)
        } else {
            (right.kind == SyntaxKind::Literal).then_some(())?;
            format!(
                "({} as {}{width})",
                self.text(right),
                if signed { "i" } else { "u" }
            )
        };
        Some((
            self.local_storage_name(left),
            self.name(left),
            end,
            self.binding_may_be_unassigned(binding),
        ))
    }

    pub(super) fn fresh_lists_referenced_by(&self, node: &SyntaxNode) -> Vec<crate::Span> {
        fn uses_binding(emitter: &Emitter<'_>, node: &SyntaxNode, binding: &TypedBinding) -> bool {
            (node.kind == SyntaxKind::Name
                && emitter
                    .local_typed_binding(node)
                    .is_some_and(|resolved| resolved.span == binding.span))
                || node
                    .children
                    .iter()
                    .any(|child| uses_binding(emitter, child, binding))
        }

        self.fresh_empty_lists
            .iter()
            .copied()
            .filter(|span| {
                self.unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == *span)
                    .is_some_and(|binding| uses_binding(self, node, binding))
            })
            .collect()
    }

    pub(super) fn iterator_list_builder(
        &self,
        condition: &SyntaxNode,
        block: &SyntaxNode,
    ) -> Option<IteratorListBuilder> {
        fn contains_await(emitter: &Emitter<'_>, node: &SyntaxNode) -> bool {
            (node.kind == SyntaxKind::UnaryExpression
                && emitter.unary_operator(node).as_deref() == Some("await"))
                || node
                    .children
                    .iter()
                    .any(|child| contains_await(emitter, child))
        }
        let (index, index_read, end, index_optional) =
            self.list_append_capacity_hint(condition, None, block)?;
        let [left, right] = condition.children.as_slice() else {
            return None;
        };
        let index_binding = self.local_typed_binding(left)?;
        if right.kind == SyntaxKind::Name {
            let end_binding = self.local_typed_binding(right)?;
            (end_binding.value_type == index_binding.value_type).then_some(())?;
        }
        let [prefix @ .., append, increment] = block.children.as_slice() else {
            return None;
        };
        prefix
            .iter()
            .all(|statement| statement.kind == SyntaxKind::Binding)
            .then_some(())?;
        (increment.kind == SyntaxKind::PostfixExpression
            && increment.children.first().is_some_and(|target| {
                self.local_typed_binding(target)
                    .is_some_and(|binding| binding.span == index_binding.span)
            })
            && self.source.text()[increment.span.start..increment.span.end]
                .trim_end()
                .ends_with("++"))
        .then_some(())?;
        let binding = self.list_append_binding(append)?;
        self.fresh_empty_lists.contains(&binding).then_some(())?;
        let append_bindings = self.inactive_list_append_bindings(condition, block);
        (append_bindings.as_slice() == [binding]).then_some(())?;
        let [_, arguments] = append.children.as_slice() else {
            return None;
        };
        let [value] = arguments.children.as_slice() else {
            return None;
        };
        (!contains_await(self, block)).then_some(())?;
        (!(self.propagate_errors || self.function_errors || self.try_completion)
            || !self.expression_throws_synchronously(block))
        .then_some(())?;
        Some(IteratorListBuilder {
            binding,
            index,
            index_read,
            index_optional,
            end,
            prefix: prefix.to_vec(),
            append: append.clone(),
            value: value.children.last().unwrap_or(value).clone(),
        })
    }

    pub(super) fn binding_has_bounded_integer_range(&self, node: &SyntaxNode) -> bool {
        self.local_typed_binding(node).is_some_and(|binding| {
            self.bounded_integer_ranges
                .iter()
                .rev()
                .any(|range| range.binding == binding.span)
        })
    }

    pub(super) fn bounded_integer_range(
        &self,
        condition: &SyntaxNode,
        block: &SyntaxNode,
    ) -> Option<BoundedIntegerRange> {
        fn mutation_count(
            emitter: &Emitter<'_>,
            node: &SyntaxNode,
            binding: &TypedBinding,
        ) -> usize {
            let own = usize::from(
                matches!(
                    node.kind,
                    SyntaxKind::Assignment | SyntaxKind::PostfixExpression
                ) && node.children.first().is_some_and(|target| {
                    emitter
                        .local_typed_binding(target)
                        .is_some_and(|target_binding| target_binding.span == binding.span)
                }),
            );
            own + node
                .children
                .iter()
                .map(|child| mutation_count(emitter, child, binding))
                .sum::<usize>()
        }
        let [left, right] = condition.children.as_slice() else {
            return None;
        };
        (self.source.text()[left.span.end..right.span.start].trim() == "<").then_some(())?;
        let binding = self.local_typed_binding(left)?;
        let ValueType::Scalar(storage) = binding.value_type else {
            return None;
        };
        fixed_integer_shape(storage)?;
        let ContextualConstant::Integer(upper) =
            contextual_constant(self.source, right, storage)?.ok()?
        else {
            return None;
        };
        let declaration = find_node_by_span(&self.unit.tree.root, binding.span)?;
        let name_index = declaration
            .children
            .iter()
            .position(|child| child.kind == SyntaxKind::Name)?;
        let initializer = binding_initializer(declaration, name_index)?;
        let ContextualConstant::Integer(lower) =
            contextual_constant(self.source, initializer, storage)?.ok()?
        else {
            return None;
        };
        (lower <= upper).then_some(())?;

        let direct_increment_count = block
            .children
            .iter()
            .filter(|statement| {
                statement.kind == SyntaxKind::PostfixExpression
                    && statement.children.first().is_some_and(|target| {
                        self.local_typed_binding(target)
                            .is_some_and(|target_binding| target_binding.span == binding.span)
                    })
                    && self.source.text()[statement.span.start..statement.span.end]
                        .trim_end()
                        .ends_with("++")
            })
            .count();
        (direct_increment_count == 1).then_some(())?;

        (mutation_count(self, &self.unit.tree.root, binding) == 1).then_some(())?;
        Some(BoundedIntegerRange {
            binding: binding.span,
            lower,
            upper,
        })
    }

    pub(super) fn bounded_float_conversion_is_exact(
        &self,
        node: &SyntaxNode,
        destination: ScalarType,
    ) -> bool {
        let Some(binding) = self.local_typed_binding(node) else {
            return false;
        };
        let precision = match destination {
            ScalarType::Float32 => 24,
            ScalarType::Float64 => 53,
            _ => return false,
        };
        let limit = BigInt::from(1_u8) << precision;
        self.bounded_integer_ranges.iter().rev().any(|range| {
            range.binding == binding.span && range.lower >= -&limit && range.upper <= limit
        })
    }

    pub(super) fn small_int_binding(&self, node: &SyntaxNode) -> Option<ScalarType> {
        if node.kind != SyntaxKind::Name {
            return None;
        }
        self.local_typed_binding(node)
            .filter(|binding| !self.is_namespace_binding_span(binding.span))
            .and_then(|binding| self.binding_scalar_storage_type(binding))
    }

    pub(super) fn lazy_namespace_binding_type(&self, node: &SyntaxNode) -> Option<ValueType> {
        if self
            .namespace_initializer
            .as_ref()
            .is_some_and(|(name, _)| name == self.text(node))
        {
            return None;
        }
        let symbol = self
            .package
            .resolve_name_at(self.unit, node.span.start, self.text(node))?;
        if symbol.global {
            return None;
        }
        let span = symbol.declaration_span?;
        if !self.is_namespace_binding_span(span) {
            return None;
        }
        let owner = self
            .package
            .units
            .iter()
            .find(|unit| unit.source.id() == span.file)?;
        owner
            .typed_bindings
            .iter()
            .find(|binding| binding.span == span)
            .map(|binding| binding.value_type.clone())
    }

    pub(super) fn is_namespace_binding_span(&self, span: crate::Span) -> bool {
        self.package
            .units
            .iter()
            .find(|unit| unit.source.id() == span.file)
            .is_some_and(|unit| {
                unit.tree.root.children.iter().any(|candidate| {
                    candidate.span == span
                        && matches!(candidate.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
                })
            })
    }

    pub(super) fn append_defaults(
        &self,
        contract: &FunctionContract,
        values: &mut [Option<String>],
    ) {
        if values.iter().all(Option::is_some) {
            return;
        }
        let Some(owner) = self
            .package
            .units
            .iter()
            .find(|unit| unit.source.id() == contract.span.file)
        else {
            return;
        };
        let Some(function) = find_node(
            &owner.tree.root,
            SyntaxKind::FunctionDeclaration,
            contract.span,
        ) else {
            return;
        };
        let Some(parameters) = function
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::ParameterList)
        else {
            return;
        };
        for (index, parameter) in parameters.children.iter().enumerate() {
            if values[index].is_some() {
                continue;
            }
            if let Some(default) = parameter.children.last().filter(|child| {
                !matches!(
                    child.kind,
                    SyntaxKind::Name | SyntaxKind::TypeExpression | SyntaxKind::VariadicMarker
                )
            }) {
                let destination = contract.parameters[index].value_type.clone();
                let value = destination
                    .and_then(|destination| match destination {
                        ValueType::Scalar(destination) => {
                            contextual_constant(&owner.source, default, destination)
                                .and_then(Result::ok)
                                .map(|constant| lower_contextual_constant(constant, destination))
                        }
                        _ => None,
                    })
                    .unwrap_or_else(|| literal_or_text(&owner.source, default));
                values[index] = Some(value);
            }
        }
    }

    pub(super) fn is_builtin(&self, node: &SyntaxNode, identity: &str) -> bool {
        let SyntaxKind::Name = node.kind else {
            return false;
        };
        self.package
            .resolve_name_at(self.unit, node.span.start, self.text(node))
            .is_some_and(|symbol| symbol.compiler_identity() == identity)
    }

    pub(super) fn unary_operator(&self, node: &SyntaxNode) -> Option<String> {
        node.children
            .iter()
            .find(|child| child.kind == SyntaxKind::UnaryOperator)
            .map(|operator| {
                self.text(operator)
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
    }

    pub(super) fn object_field(&self, receiver: &SyntaxNode, name: &str) -> bool {
        let Some(ValueType::Object(identity)) = self.receiver_value_type(receiver) else {
            return false;
        };
        self.unit
            .descriptors
            .iter()
            .find(|object| object.identity == identity)
            .is_some_and(|object| {
                effective_object_fields(self.package, object)
                    .iter()
                    .any(|field| field.name == name)
            })
    }

    pub(super) fn callable_object_field(&self, receiver: &SyntaxNode, name: &str) -> bool {
        let Some(ValueType::Object(identity)) = self.receiver_value_type(receiver) else {
            return false;
        };
        let has_method = self
            .package
            .units
            .iter()
            .flat_map(|unit| &unit.functions)
            .any(|method| method.owner_identity.as_ref() == Some(&identity) && method.name == name);
        !has_method
            && self
                .package
                .units
                .iter()
                .flat_map(|unit| &unit.descriptors)
                .find(|object| object.identity == identity)
                .is_some_and(|object| {
                    effective_object_fields(self.package, object)
                        .iter()
                        .any(|field| {
                            field.name == name
                                && matches!(field.value_type, ValueType::Function(..))
                        })
                })
    }

    pub(super) fn wrapped_object_field(&self, receiver: &SyntaxNode, name: &str) -> bool {
        let Some(ValueType::Object(identity)) = self.receiver_value_type(receiver) else {
            return false;
        };
        let Some(object) = self
            .unit
            .descriptors
            .iter()
            .find(|object| object.identity == identity)
        else {
            return false;
        };
        !(object_descendants(self.unit, object).is_empty()
            || self.text(receiver) == "this" && self.current_object.is_some())
            && effective_object_fields(self.package, object)
                .iter()
                .any(|field| field.name == name)
    }

    pub(super) fn value_type_owns_resource(&self, value_type: &ValueType) -> bool {
        if let ValueType::Union(arms) = value_type {
            return arms.iter().any(|arm| self.value_type_owns_resource(arm));
        }
        if matches!(
            value_type,
            ValueType::Function(_, _, effects)
                | ValueType::AsyncFunction(_, _, _, effects)
                if effects.modes.written == InvocationMode::Consuming
        ) {
            return true;
        }
        crate::semantics::application_is_resource_owning(self.package, value_type)
    }

    pub(super) fn object_requires_separation(&self, identity: &ObjectIdentity) -> bool {
        let Some(object) = self
            .unit
            .descriptors
            .iter()
            .find(|object| object.identity == *identity)
        else {
            return false;
        };
        effective_object_methods(self.unit, object)
            .iter()
            .any(|method| method.name == "destruct")
            || object_descendants(self.unit, object)
                .iter()
                .any(|descendant| {
                    effective_object_methods(self.unit, descendant)
                        .iter()
                        .any(|method| method.name == "destruct")
                })
            || (object.kind == ObjectKind::Interface
                && self.unit.descriptors.iter().any(|candidate| {
                    candidate.interfaces.contains(&object.identity)
                        && (effective_object_methods(self.unit, candidate)
                            .iter()
                            .any(|method| method.name == "destruct")
                            || object_descendants(self.unit, candidate)
                                .iter()
                                .any(|descendant| {
                                    effective_object_methods(self.unit, descendant)
                                        .iter()
                                        .any(|method| method.name == "destruct")
                                }))
                }))
    }

    pub(super) fn reference_storage_expression(&mut self, operand: &SyntaxNode) -> String {
        if self.reference_backed_name(operand).is_some() {
            format!("({}).clone()", self.local_storage_name(operand))
        } else {
            format!(
                "std::sync::Arc::new(std::sync::Mutex::new({}))",
                self.expression(operand)
            )
        }
    }

    pub(super) fn narrowed_optional_name(&self, operand: &SyntaxNode) -> Option<ValueType> {
        if operand.kind != SyntaxKind::Name {
            return None;
        }
        narrowed_value_type(self.unit, operand, &self.unit.typed_bindings).or_else(|| {
            self.parameter_types
                .iter()
                .rev()
                .find(|(name, _)| name == self.text(operand))
                .and_then(|(_, value_type)| {
                    narrowed_optional_type(self.unit, operand, value_type.clone())
                })
        })
    }

    pub(super) fn reference_address_expression(&mut self, operand: &SyntaxNode) -> String {
        if operand.kind == SyntaxKind::GroupExpression
            && let Some(inner) = operand.children.first()
        {
            return self.reference_address_expression(inner);
        }
        if self.reference_backed_name(operand).is_some() {
            return format!(
                "std::sync::Arc::downgrade(&{})",
                self.reference_storage_expression(operand)
            );
        }
        if matches!(
            self.value_type(operand),
            Some(ValueType::SharedReference(_))
        ) {
            return format!("std::sync::Arc::downgrade(&{})", self.expression(operand));
        }
        if self
            .value_type(operand)
            .is_some_and(|value_type| !rust_value_is_copy(&value_type))
            && let Some(reference) = self.narrowed_storage_name(operand, false, false)
        {
            return reference;
        }
        if self.narrowed_optional_name(operand).is_some() {
            let source_name = self.name(operand);
            return format!("&*{source_name}.as_ref().expect(\"semantic optional narrowing\")");
        }
        if operand.kind == SyntaxKind::Name {
            if let Some(binding) = self.local_typed_binding(operand)
                && self.binding_may_be_unassigned(binding)
            {
                return self.available_storage_reference(binding, operand, false);
            }
            return format!("&{}", self.raw_storage_name(operand));
        }
        if operand.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = operand.children.as_slice()
        {
            let receiver = self.expression(receiver);
            return format!("&({receiver}).{}", rust_name(self.text(member)));
        }
        if operand.kind == SyntaxKind::IndexExpression
            && let [receiver, index] = operand.children.as_slice()
            && matches!(
                self.receiver_value_type(receiver),
                Some(ValueType::List(_) | ValueType::Tuple(_, _))
            )
        {
            let receiver = self.expression(receiver);
            let index = self.expression_as(index, ValueType::Scalar(ScalarType::Int));
            let converted_index = self.fallible(
                format!("terrane_collection_support::index_from_int(&({index}))"),
                operand,
            );
            let item = self.fallible(
                format!(
                    "({receiver}).get(__terrane_index).ok_or_else(|| terrane_collection_support::IndexError::from_usize(__terrane_index))"
                ),
                operand,
            );
            return format!("{{ let __terrane_index = {converted_index}; {item} }}");
        }
        if operand.kind == SyntaxKind::IndexExpression
            && let [receiver, key] = operand.children.as_slice()
            && let Some(ValueType::Map(key_type, _) | ValueType::UnorderedMap(key_type, _)) =
                self.receiver_value_type(receiver)
        {
            let receiver = self.expression(receiver);
            let key = self.expression_as(key, key_type.value_type());
            return self.fallible(
                format!("({receiver}).get(&({key})).ok_or(terrane_collection_support::MissingKey)"),
                operand,
            );
        }
        let message = format!(
            "validated reference expression `{}` has no native address lowering",
            self.text(operand)
        );
        self.failure.get_or_insert(LoweringFailure {
            span: operand.span,
            message,
        });
        "{ compile_error!(\"internal reference-address lowering failure\") }".to_owned()
    }

    pub(super) fn reference_backed_name(&self, node: &SyntaxNode) -> Option<&TypedBinding> {
        if node.kind != SyntaxKind::Name {
            return None;
        }
        let span = self
            .package
            .resolve_name_at(self.unit, node.span.start, self.text(node))?
            .declaration_span?;
        self.unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == span && self.reference_backed(binding))
    }

    pub(super) fn reference_backed(&self, binding: &TypedBinding) -> bool {
        if matches!(
            binding.value_type,
            ValueType::Reference(_) | ValueType::SharedReference(_)
        ) {
            return false;
        }
        self.node_references_binding(&self.unit.tree.root, binding)
    }

    pub(super) fn reference_uses_shared_storage(&self, node: &SyntaxNode) -> bool {
        let provenance = self
            .unit
            .reference_provenance
            .get(&(node.span.start, node.span.end))
            .or_else(|| {
                self.local_typed_binding(node).and_then(|binding| {
                    self.unit
                        .reference_provenance
                        .get(&(binding.span.start, binding.span.end))
                })
            });
        provenance
            .is_some_and(|provenance| self.reference_owner_uses_shared_storage(provenance.owner))
    }

    pub(super) fn reference_owner_uses_shared_storage(&self, owner: crate::Span) -> bool {
        self.unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == owner)
            .is_some_and(|binding| {
                matches!(binding.value_type, ValueType::SharedReference(_))
                    || self.reference_backed(binding)
            })
    }

    pub(super) fn binding_is_reference_owner(&self, binding: &TypedBinding) -> bool {
        self.unit
            .reference_provenance
            .values()
            .any(|provenance| provenance.owner == binding.span)
    }

    pub(super) fn node_references_binding(
        &self,
        node: &SyntaxNode,
        binding: &TypedBinding,
    ) -> bool {
        if node.kind == SyntaxKind::UnaryExpression
            && self.unary_operator(node).as_deref() == Some("shared ref")
            && let Some(operand) = node.children.last()
            && operand.kind == SyntaxKind::Name
            && self
                .package
                .resolve_name_at(self.unit, operand.span.start, self.text(operand))
                .and_then(|symbol| symbol.declaration_span)
                == Some(binding.span)
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| self.node_references_binding(child, binding))
    }

    pub(super) fn text(&self, node: &SyntaxNode) -> &str {
        &self.source.text()[node.span.start..node.span.end]
    }
    pub(super) fn rust_block_body(&self, node: &SyntaxNode) -> String {
        let body = self
            .text(node)
            .split_once('\n')
            .map_or("", |(_, body)| body);
        let lines = body.lines().collect::<Vec<_>>();
        let indentation = lines
            .iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.len() - line.trim_start_matches([' ', '\t']).len())
            .min()
            .unwrap_or(0);
        lines
            .into_iter()
            .map(|line| line.get(indentation..).unwrap_or(line))
            .collect::<Vec<_>>()
            .join("\n")
    }
    pub(super) fn rust_block_clone_prelude(&self, node: &SyntaxNode) -> String {
        let identifiers = crate::rust_ir::rust_syntactic_identifiers(&self.rust_block_body(node));
        self.unit
            .typed_bindings
            .iter()
            .filter(|binding| {
                binding.scope.is_some()
                    && binding.is_visible_at(self.unit.source.id(), node.span.start)
                    && !matches!(binding.value_type, ValueType::Reference(_))
                    && !rust_value_is_copy(&binding.value_type)
                    && identifiers.contains(crate::lowering::binding_storage_rust_name(
                        self.unit, binding,
                    ))
            })
            .map(|binding| {
                let name = crate::lowering::binding_storage_rust_name(self.unit, binding);
                format!("let {name} = {name}.clone();")
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    pub(super) fn inline_rust_expression(&self, node: &SyntaxNode) -> String {
        let body = self.rust_block_body(node);
        let prelude = self.rust_block_clone_prelude(node);
        if node.kind == SyntaxKind::RustBlock
            && prelude.is_empty()
            && syn::parse_str::<syn::Expr>(&body).is_ok()
        {
            return format!("({body})");
        }
        let boundary = if node.kind == SyntaxKind::UnsafeRustBlock {
            "unsafe "
        } else {
            ""
        };
        format!("{boundary}{{ {prelude} {body} }}")
    }

    pub(super) fn inline_rust_statement(&mut self, node: &SyntaxNode) {
        if node.kind == SyntaxKind::UnsafeRustBlock {
            self.line("unsafe {");
        } else {
            self.line("{");
        }
        self.indent += 1;
        let prelude = self.rust_block_clone_prelude(node);
        if !prelude.is_empty() {
            self.line(&prelude);
        }
        for line in self.rust_block_body(node).lines() {
            self.line(line);
        }
        self.indent -= 1;
        self.line("}");
    }

    pub(super) fn control_condition(&mut self, mut node: &SyntaxNode) -> String {
        while node.kind == SyntaxKind::GroupExpression
            && let [grouped] = node.children.as_slice()
        {
            node = grouped;
        }
        let object_truth = matches!(self.value_type(node), Some(ValueType::Object(_)));
        let expression = self.expression(node);
        if object_truth {
            format!("({expression}).truth()")
        } else if node.kind == SyntaxKind::BinaryExpression {
            Self::unwrapped_expression(expression)
        } else {
            expression
        }
    }

    pub(super) fn line_start(&mut self) {
        for _ in 0..self.indent {
            self.output.push_str("    ");
        }
    }

    pub(super) fn debug_point(&mut self, node: &SyntaxNode, role: &str) {
        if !self.debug_information {
            return;
        }
        let comment = format!(
            "/* terrane-debug-point:{}:{}:{}:{role} */",
            node.span.file, node.span.start, node.span.end
        );
        self.line(&format!("__terrane_debug_point!({comment:?});"));
    }

    pub(super) fn narrowed_storage_name(
        &self,
        node: &SyntaxNode,
        owned: bool,
        mutable: bool,
    ) -> Option<String> {
        if self.assignment_target {
            return None;
        }
        let key = (node.span.file, node.span.start, node.span.end);
        let identity = *self.unit.flow_binding_ids.get(&key)?;
        let physical = self.unit.flow_binding_types.get(&identity).or_else(|| {
            self.unit
                .typed_bindings
                .iter()
                .find(|binding| binding.span == identity)
                .map(|binding| &binding.value_type)
        })?;
        let value_type = self.unit.flow_types.get(&key)?;
        let variant = match physical {
            ValueType::Union(arms) => {
                let index = arms.iter().position(|arm| arm == value_type)?;
                format!("{}::Arm{index}", union_type_name_for_span(identity))
            }
            ValueType::Optional(inner) if inner.as_ref() == value_type => "Some".to_owned(),
            _ => return None,
        };
        let copy = rust_value_is_copy(value_type) || matches!(value_type, ValueType::Reference(_));
        let take = owned
            && !mutable
            && !copy
            && !self.value_type_owns_resource(value_type)
            && !self.binding_value_is_reused(node)
            && !self.non_consuming_capture_read(node);
        let access = if take {
            self.owned_storage_value(node)
        } else {
            self.local_typed_binding(node).map_or_else(
                || {
                    format!(
                        "{}{}",
                        if mutable { "&mut " } else { "&" },
                        self.local_storage_name(node)
                    )
                },
                |binding| self.available_storage_reference(binding, node, mutable),
            )
        };
        let value = format!(
            "match {access} {{ {variant}(value) => value, _ => unreachable!(\"flow-proven storage refinement\") }}"
        );
        Some(if mutable || take {
            format!("({value})")
        } else if copy {
            format!("*({value})")
        } else if owned && !self.value_type_owns_resource(value_type) {
            format!("({value}).clone()")
        } else {
            format!("({value})")
        })
    }

    fn narrowed_name(&self, node: &SyntaxNode) -> Option<String> {
        if let Some(value) = self.narrowed_storage_name(node, false, false) {
            return Some(value);
        }
        let source_name = self.text(node);
        let narrowed = (!self.assignment_target)
            .then(|| {
                narrowed_value_type(self.unit, node, &self.unit.typed_bindings).or_else(|| {
                    self.parameter_types
                        .iter()
                        .rev()
                        .find(|(name, _)| name == source_name)
                        .and_then(|(_, value_type)| {
                            narrowed_optional_type(self.unit, node, value_type.clone())
                        })
                })
            })
            .flatten();
        if let Some(narrowed) = narrowed {
            let access = format!(
                "{}.as_ref().expect(\"semantic optional narrowing\")",
                self.local_storage_name(node)
            );
            return Some(if rust_value_is_copy(&narrowed) {
                format!("*{access}")
            } else {
                format!("{access}.clone()")
            });
        }
        None
    }

    pub(super) fn line(&mut self, text: &str) {
        self.line_start();
        self.output.push_str(text);
        self.output.push('\n');
    }
}
