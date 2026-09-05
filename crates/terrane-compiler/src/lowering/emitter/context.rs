use super::super::prelude::*;

impl Emitter<'_> {
    pub(super) fn name(&self, node: &SyntaxNode) -> String {
        let source_name = self.text(node);
        if source_name == "none" {
            return "()".to_owned();
        }
        if source_name == "this" {
            return if self.closure_depth == 0 {
                "self"
            } else {
                "this"
            }
            .to_owned();
        }
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
                rust_name(source_name)
            );
            return if matches!(narrowed, ValueType::Scalar(_)) {
                format!("*{access}")
            } else {
                access
            };
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
        let encoding = resolved
            .and_then(|symbol| symbol.identity.strip_prefix("/core/encodings::"))
            .and_then(|name| match name {
                "utf8" => Some("Utf8"),
                "utf16-le" => Some("Utf16Le"),
                "utf16-be" => Some("Utf16Be"),
                "utf32-le" => Some("Utf32Le"),
                "utf32-be" => Some("Utf32Be"),
                _ => None,
            });
        if let Some(encoding) = encoding {
            return format!("terrane_string_support::Encoding::{encoding}");
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
            let failure = self.uninitialized_global_failure(node);
            return format!(
                "{storage}.lock().expect(\"program-global lock poisoned\").clone().unwrap_or_else(|| {failure})"
            );
        }
        let Some(span) = symbol.declaration_span else {
            return rust_name(source_name);
        };
        if let Some(binding) = self
            .unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == span)
            && self.reference_backed(binding)
        {
            return format!(
                "({{ let __terrane_value = {}.lock().expect(\"reference lock poisoned\").clone(); __terrane_value }})",
                rust_name(source_name)
            );
        }
        let name = namespace_binding_name(span.file, &symbol.name);
        if self.lazy_namespace_binding_type(node).is_some() {
            format!("&*{name}")
        } else if self.is_namespace_binding_span(span) {
            name
        } else {
            rust_name(source_name)
        }
    }

    pub(super) fn uninitialized_global_failure(&self, node: &SyntaxNode) -> String {
        let (line, column) = self.source.line_column(node.span.start);
        format!(
            "__terrane_uninitialized_global({:?}, {:?}, {line}, {column})",
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

    pub(super) fn local_typed_binding(&self, node: &SyntaxNode) -> Option<&TypedBinding> {
        (node.kind == SyntaxKind::Name)
            .then(|| {
                self.unit.typed_bindings.iter().rev().find(|binding| {
                    binding.name == self.text(node)
                        && binding.is_visible_at(self.source.id(), node.span.start)
                        && !self.is_namespace_binding_span(binding.span)
                })
            })
            .flatten()
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
    }

    pub(super) fn for_capacity_hint(
        &self,
        condition: &SyntaxNode,
        update: &SyntaxNode,
        block: &SyntaxNode,
    ) -> Option<(String, String)> {
        self.list_append_capacity_hint(condition, Some(update), block)
    }

    fn list_append_capacity_hint(
        &self,
        condition: &SyntaxNode,
        update: Option<&SyntaxNode>,
        block: &SyntaxNode,
    ) -> Option<(String, String)> {
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
            rust_name(&upper.name)
        } else {
            (right.kind == SyntaxKind::Literal).then_some(())?;
            format!(
                "({} as {}{width})",
                self.text(right),
                if signed { "i" } else { "u" }
            )
        };
        Some((rust_name(&binding.name), end))
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
        (node.kind == SyntaxKind::Name)
            .then(|| {
                self.unit
                    .typed_bindings
                    .iter()
                    .rev()
                    .find(|binding| {
                        binding.name == self.text(node)
                            && binding.is_visible_at(self.source.id(), node.span.start)
                            && !self.is_namespace_binding_span(binding.span)
                            && !binding_span_is_mutated(self.package, self.unit, binding.span, true)
                    })
                    .and_then(|binding| binding.storage_type)
            })
            .flatten()
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
                !matches!(child.kind, SyntaxKind::Name | SyntaxKind::TypeExpression)
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

    pub(super) fn callable_object_field(&self, receiver: &SyntaxNode, name: &str) -> bool {
        let Some(ValueType::Object(identity)) = self.receiver_value_type(receiver) else {
            return false;
        };
        self.unit
            .objects
            .iter()
            .find(|object| object.identity == identity)
            .is_some_and(|object| {
                !self
                    .package
                    .units
                    .iter()
                    .flat_map(|unit| &unit.functions)
                    .any(|method| method.owner.is_some() && method.name == name)
                    && effective_object_fields(self.unit, object)
                        .iter()
                        .any(|field| {
                            field.name == name
                                && matches!(field.value_type, ValueType::Function(_, _))
                        })
            })
    }

    pub(super) fn wrapped_object_field(&self, receiver: &SyntaxNode, name: &str) -> bool {
        let Some(ValueType::Object(identity)) = self.receiver_value_type(receiver) else {
            return false;
        };
        let Some(object) = self
            .unit
            .objects
            .iter()
            .find(|object| object.identity == identity)
        else {
            return false;
        };
        !(object_descendants(self.unit, object).is_empty()
            || self.text(receiver) == "this" && self.current_object.is_some())
            && effective_object_fields(self.unit, object)
                .iter()
                .any(|field| field.name == name)
    }

    pub(super) fn value_type_owns_resource(&self, value_type: &ValueType) -> bool {
        let ValueType::Object(identity) = value_type else {
            return false;
        };
        self.package
            .units
            .iter()
            .flat_map(|unit| &unit.objects)
            .any(|object| object.identity == *identity && object.resource_owning)
    }

    pub(super) fn object_owns_resource(&self, identity: &ObjectIdentity) -> bool {
        self.package
            .units
            .iter()
            .flat_map(|unit| &unit.objects)
            .find(|object| object.identity == *identity)
            .is_some_and(|object| object.resource_owning)
    }

    pub(super) fn object_requires_separation(&self, identity: &ObjectIdentity) -> bool {
        let Some(object) = self
            .unit
            .objects
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
                && self.unit.objects.iter().any(|candidate| {
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
            rust_name(self.text(operand))
        } else {
            self.expression(operand)
        }
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

    pub(super) fn node_references_binding(
        &self,
        node: &SyntaxNode,
        binding: &TypedBinding,
    ) -> bool {
        if node.kind == SyntaxKind::UnaryExpression
            && matches!(
                self.unary_operator(node).as_deref(),
                Some("ref" | "shared ref")
            )
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

    pub(super) fn control_condition(&mut self, mut node: &SyntaxNode) -> String {
        while node.kind == SyntaxKind::GroupExpression
            && let [grouped] = node.children.as_slice()
        {
            node = grouped;
        }
        let expression = self.expression(node);
        if node.kind == SyntaxKind::BinaryExpression {
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

    pub(super) fn line(&mut self, text: &str) {
        self.line_start();
        self.output.push_str(text);
        self.output.push('\n');
    }
}
