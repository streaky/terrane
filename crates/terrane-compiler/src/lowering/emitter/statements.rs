use super::super::prelude::*;

impl Emitter<'_> {
    pub(super) fn statement(&mut self, node: &SyntaxNode) {
        match node.kind {
            SyntaxKind::Binding => {
                if !self.global_assignment(node) {
                    self.binding(node);
                }
            }
            SyntaxKind::Assignment => self.assignment(node),
            SyntaxKind::CallExpression => {
                let expression = if let Some(expression) = self.collection_mutation_statement(node)
                {
                    expression
                } else {
                    self.discarded_call = Some(node.span);
                    let expression = self.expression(node);
                    self.discarded_call = None;
                    expression
                };
                self.line(&format!("{expression};"));
            }
            SyntaxKind::PostfixExpression => self.postfix(node),
            SyntaxKind::IfStatement => self.if_statement(node),
            SyntaxKind::WhileStatement => self.while_statement(node),
            SyntaxKind::ForStatement => self.for_statement(node),
            SyntaxKind::ReturnStatement => {
                let value = node.children.first().map_or_else(
                    || "()".to_owned(),
                    |value| {
                        let value = if let Some(return_type) = self.return_type.clone() {
                            self.expression_as(value, return_type)
                        } else {
                            self.expression(value)
                        };
                        Self::unwrapped_expression(value)
                    },
                );
                if self.try_completion {
                    self.line(&format!("return TerraneCompletion::Return({value});"));
                } else if self.function_errors {
                    self.line(&format!("return Ok({value});"));
                } else if self.propagate_errors {
                    self.line(&format!("return Ok(Some({value}));"));
                } else {
                    self.line(&format!("return {value};"));
                }
            }
            SyntaxKind::ThrowStatement => self.throw_statement(node),
            SyntaxKind::TryStatement => self.try_statement(node),
            SyntaxKind::BreakStatement if self.try_completion => {
                self.line("return TerraneCompletion::Break;");
            }
            SyntaxKind::BreakStatement => {
                if let Some(label) = &self.break_label {
                    self.line(&format!("break '{label};"));
                } else {
                    self.line("break;");
                }
            }
            SyntaxKind::ContinueStatement if self.try_completion => {
                self.line("return TerraneCompletion::Continue;");
            }
            SyntaxKind::ContinueStatement => {
                if let Some(label) = &self.continue_label {
                    self.line(&format!("break '{label};"));
                } else {
                    self.line("continue;");
                }
            }
            _ => {}
        }
    }

    pub(super) fn collection_mutation_statement(&mut self, node: &SyntaxNode) -> Option<String> {
        let [callee, arguments] = node.children.as_slice() else {
            return None;
        };
        let [receiver, member] = callee.children.as_slice() else {
            return None;
        };
        if callee.kind != SyntaxKind::MemberExpression {
            return None;
        }
        let receiver_type = self.receiver_value_type(receiver)?;
        let list_append_vector = (self.text(member) == "append")
            .then(|| self.local_typed_binding(receiver))
            .flatten()
            .and_then(|binding| {
                self.list_append_borrows
                    .iter()
                    .rev()
                    .find(|borrow| borrow.binding == binding.span)
                    .map(|borrow| borrow.vector.clone())
            });
        let receiver_value = self.receiver_guard_expression(receiver);
        let values = arguments
            .children
            .iter()
            .map(|argument| argument.children.last().unwrap_or(argument))
            .collect::<Vec<_>>();
        let mutation = match (receiver_type, self.text(member)) {
            (ValueType::List(item), "append") => {
                let value = self.expression_as(values[0], item.value_type());
                Some(list_append_vector.map_or_else(
                    || format!("({receiver_value}).append({value})"),
                    |vector| format!("{vector}.push({value})"),
                ))
            }
            (ValueType::List(item), "set") => {
                let index = self.expression_as(values[0], ValueType::Scalar(ScalarType::Int));
                let index = self.fallible(
                    format!("terrane_collection_support::index_from_int(&({index}))"),
                    node,
                );
                let value = self.expression_as(values[1], item.value_type());
                Some(self.fallible(format!("({receiver_value}).set({index}, {value})"), node))
            }
            (ValueType::Map(key, value) | ValueType::UnorderedMap(key, value), "set") => {
                Some(format!(
                    "({receiver_value}).set({}, {})",
                    self.expression_as(values[0], key.value_type()),
                    self.expression_as(values[1], value.value_type())
                ))
            }
            (ValueType::Set(item) | ValueType::UnorderedSet(item), "add") => Some(format!(
                "({receiver_value}).add({})",
                self.expression_as(values[0], item.value_type())
            )),
            _ => None,
        };
        mutation.map(|mutation| self.wrap_receiver_guard(receiver, mutation))
    }

    pub(super) fn assigned_binding(&self, left: &SyntaxNode) -> Option<TypedBinding> {
        (left.kind == SyntaxKind::Name)
            .then(|| {
                self.package
                    .resolve_name_at(self.unit, left.span.start, self.text(left))
            })
            .flatten()
            .and_then(|symbol| symbol.declaration_span)
            .and_then(|span| {
                self.unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == span)
            })
            .cloned()
    }

    pub(super) fn nested_static_assignment_target(&self, node: &SyntaxNode) -> Option<String> {
        let [receiver, member] = node.children.as_slice() else {
            return None;
        };
        if node.kind != SyntaxKind::MemberExpression {
            return None;
        }
        let mut target = if receiver.kind == SyntaxKind::StaticMemberExpression {
            let [class, field] = receiver.children.as_slice() else {
                return None;
            };
            let object = self.class_designator(class)?;
            let field_name = self.text(field);
            effective_object_fields(self.unit, object)
                .iter()
                .any(|candidate| candidate.is_static && candidate.name == field_name)
                .then(|| {
                    format!(
                        "{}.lock().expect(\"static field lock poisoned\")",
                        rust_static_field_name(self.package, &object.identity, field_name)
                    )
                })?
        } else {
            self.nested_static_assignment_target(receiver)?
        };
        write!(target, ".{}", rust_name(self.text(member))).unwrap();
        Some(target)
    }

    pub(super) fn assign_static_field(&mut self, left: &SyntaxNode, value: &str) -> bool {
        let [receiver, member] = left.children.as_slice() else {
            return false;
        };
        if left.kind != SyntaxKind::StaticMemberExpression {
            return false;
        }
        let Some(object) = self.class_designator(receiver) else {
            return false;
        };
        self.line(&format!(
            "{{ let __terrane_static_value = {value}; *{}.lock().expect(\"static field lock poisoned\") = __terrane_static_value; }}",
            rust_static_field_name(self.package, &object.identity, self.text(member))
        ));
        true
    }

    pub(super) fn assignment(&mut self, node: &SyntaxNode) {
        if self.global_assignment(node) {
            return;
        }
        if let [target, value] = node.children.as_slice()
            && target.kind == SyntaxKind::IndexExpression
            && let [receiver, index] = target.children.as_slice()
            && let Some(receiver_type) = self.receiver_value_type(receiver)
        {
            let receiver_value = self.receiver_guard_expression(receiver);
            match receiver_type {
                ValueType::List(item) => {
                    let index = self.expression_as(index, ValueType::Scalar(ScalarType::Int));
                    let index = self.fallible(
                        format!("terrane_collection_support::index_from_int(&({index}))"),
                        node,
                    );
                    let value = self.expression_as(value, item.value_type());
                    let mutation =
                        self.fallible(format!("({receiver_value}).set({index}, {value})"), node);
                    let mutation = self.wrap_receiver_guard(receiver, mutation);
                    self.line(&format!("let _ = {mutation};"));
                }
                ValueType::Map(key, value_type) | ValueType::UnorderedMap(key, value_type) => {
                    let key = self.expression_as(index, key.value_type());
                    let value = self.expression_as(value, value_type.value_type());
                    let mutation = format!("({receiver_value}).set({key}, {value})");
                    let mutation = self.wrap_receiver_guard(receiver, mutation);
                    self.line(&format!("let _ = {mutation};"));
                }
                _ => {}
            }
            return;
        }
        if self
            .unit
            .typed_bindings
            .iter()
            .any(|binding| binding.span == node.span)
        {
            self.binding(node);
            return;
        }
        let [left, right] = node.children.as_slice() else {
            return;
        };
        let assigned_binding = self.assigned_binding(left);
        let union_binding = self.union_binding(left);
        let value_type = assigned_binding
            .as_ref()
            .map(|binding| binding.value_type.clone())
            .or_else(|| self.value_type(left));
        let value = if let Some(binding) = union_binding {
            self.union_value(&binding, right)
        } else if let Some(value_type) = value_type {
            self.expression_as(right, value_type)
        } else {
            self.expression(right)
        };
        let value = Self::unwrapped_expression(value);
        if self.assign_static_field(left, &value) {
            return;
        }
        let reference_backed = assigned_binding
            .as_ref()
            .is_some_and(|binding| self.reference_backed(binding));
        let previous_assignment_target = self.assignment_target;
        self.assignment_target = true;
        let target = if reference_backed {
            format!(
                "*{}.lock().expect(\"reference lock poisoned\")",
                rust_name(self.text(left))
            )
        } else if let Some(target) = self.nested_static_assignment_target(left) {
            target
        } else if let [receiver, member] = left.children.as_slice()
            && left.kind == SyntaxKind::MemberExpression
            && self.wrapped_object_field(receiver, self.text(member))
        {
            format!(
                "*({}).terrane_field_{}_mut()",
                self.receiver_expression(receiver),
                rust_name(self.text(member))
            )
        } else {
            self.expression(left)
        };
        self.assignment_target = previous_assignment_target;
        self.line(&format!("{target} = {value};"));
        if let Some(binding) = &assigned_binding
            && !reference_backed
            && !binding_store_value_is_read(self.package, binding.span, node.span)
        {
            self.line(&format!("let _ = &mut {target};"));
        }
    }

    pub(super) fn error_kind(&self, node: &SyntaxNode) -> String {
        let descriptor = if node.kind == SyntaxKind::CallExpression {
            node.children.first().unwrap_or(node)
        } else {
            node
        };
        let descriptor = if descriptor.kind == SyntaxKind::ConstructionExpression {
            descriptor.children.first().unwrap_or(descriptor)
        } else {
            descriptor
        };
        self.package
            .resolve_name_at(self.unit, descriptor.span.start, self.text(descriptor))
            .map_or_else(
                || {
                    self.text(descriptor)
                        .trim()
                        .trim_start_matches('.')
                        .to_owned()
                },
                |symbol| symbol.name.clone(),
            )
    }

    pub(super) fn rust_error_kind(&self, node: &SyntaxNode) -> String {
        let descriptor = if node.kind == SyntaxKind::CallExpression {
            node.children.first().unwrap_or(node)
        } else {
            node
        };
        let descriptor = if descriptor.kind == SyntaxKind::ConstructionExpression {
            descriptor.children.first().unwrap_or(descriptor)
        } else {
            descriptor
        };
        if let Some(symbol) =
            self.package
                .resolve_name_at(self.unit, descriptor.span.start, self.text(descriptor))
        {
            if let Some(kind) = rust_builtin_error_kind(&symbol.name) {
                return kind.to_owned();
            }
            let descriptor = self
                .registry
                .register_descriptor(&symbol.identity, &symbol.name);
            return format!("Custom(DescriptorId({descriptor}))");
        }
        let name = self.error_kind(node);
        if let Some(kind) = rust_builtin_error_kind(&name) {
            kind.to_owned()
        } else {
            let identity = format!("{}::{name}", self.unit.namespace.trim_end_matches('/'));
            let descriptor = self.registry.register_descriptor(&identity, &name);
            format!("Custom(DescriptorId({descriptor}))")
        }
    }

    pub(super) fn throw_statement(&mut self, node: &SyntaxNode) {
        if let Some(current_error) = &self.current_error
            && node.children.is_empty()
        {
            let frame = self.error_site(node);
            let error = format!("{current_error}.clone().at({frame})");
            if self.try_completion {
                self.line(&format!("return TerraneCompletion::Error({error});"));
            } else if self.propagate_errors {
                self.line(&format!("return Err({error});"));
            } else {
                self.line(&format!("__terrane_uncaught({error});"));
            }
            return;
        }
        let Some(error_node) = node.children.first() else {
            return;
        };
        let kind = self.rust_error_kind(error_node);
        let origin = self.error_site(node);
        let mut error = if kind.starts_with("Custom(") {
            let value = self.expression(error_node);
            format!(
                "{{ let value = {value}; TerraneError::raised_with_message(TerraneErrorKind::{kind}, value.render(), {origin}) }}"
            )
        } else {
            format!("TerraneError::raised(TerraneErrorKind::{kind}, {origin})")
        };
        if let Some(current_error) = &self.current_error {
            error = format!("{error}.with_cause({current_error}.clone())");
        }
        if self.try_completion {
            self.line(&format!("return TerraneCompletion::Error({error});"));
        } else if self.propagate_errors {
            self.line(&format!("return Err({error});"));
        } else {
            self.line(&format!("__terrane_uncaught({error});"));
        }
    }

    #[expect(
        clippy::too_many_lines,
        reason = "the emitted try/catch/finally control flow is clearer as one auditable state machine"
    )]
    pub(super) fn try_statement(&mut self, node: &SyntaxNode) {
        let Some(block) = node.children.first() else {
            return;
        };
        let index = self.try_counter;
        self.try_counter += 1;
        let result = self.return_type.clone().map_or_else(
            || "()".to_owned(),
            |value_type| rust_value_type(self.package, value_type),
        );
        let mutable = if node
            .children
            .iter()
            .any(|child| child.kind == SyntaxKind::FinallyClause)
        {
            "mut "
        } else {
            ""
        };
        self.line(&format!(
            "let {mutable}__terrane_completion_{index}: TerraneCompletion<{result}> = (|| {{"
        ));
        self.indent += 1;
        self.line(&format!(
            "let __terrane_try_{index}: TerraneCompletion<{result}> = (|| {{"
        ));
        self.indent += 1;
        let outer_completion = std::mem::replace(&mut self.try_completion, true);
        let outer_propagation = std::mem::replace(&mut self.propagate_errors, true);
        let outer_function_errors = std::mem::replace(&mut self.function_errors, false);
        self.block(block);
        if block_may_fall_through(block) {
            self.line("TerraneCompletion::Normal");
        }
        self.function_errors = outer_function_errors;
        self.propagate_errors = outer_propagation;
        self.indent -= 1;
        self.line("})();");
        self.line(&format!("match __terrane_try_{index} {{"));
        self.indent += 1;
        self.line("TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),");
        self.line("TerraneCompletion::Break => return TerraneCompletion::Break,");
        self.line("TerraneCompletion::Continue => return TerraneCompletion::Continue,");
        self.line("TerraneCompletion::Normal => {}");
        self.line(&format!(
            "TerraneCompletion::Error(__terrane_error_{index}) => {{"
        ));
        self.indent += 1;
        self.line(&format!("let mut __terrane_handled_{index} = false;"));
        for clause in node
            .children
            .iter()
            .filter(|child| child.kind == SyntaxKind::CatchClause)
        {
            let descriptor = clause
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name);
            let condition = descriptor.map_or_else(
                || format!("!__terrane_handled_{index}"),
                |descriptor| {
                    let name = self.error_kind(descriptor);
                    if name == "error" {
                        format!("!__terrane_handled_{index}")
                    } else {
                        format!(
                            "!__terrane_handled_{index} && __terrane_error_{index}.kind == TerraneErrorKind::{}",
                            self.rust_error_kind(descriptor)
                        )
                    }
                },
            );
            self.line(&format!("if {condition} {{"));
            self.indent += 1;
            self.line(&format!("__terrane_handled_{index} = true;"));
            let outer_error = self
                .current_error
                .replace(format!("__terrane_error_{index}"));
            if let Some(catch_block) = clause.children.last() {
                self.block(catch_block);
            }
            self.current_error = outer_error;
            self.indent -= 1;
            self.line("}");
        }
        self.try_completion = outer_completion;
        self.line(&format!("if !__terrane_handled_{index} {{"));
        self.indent += 1;
        self.line(&format!(
            "return TerraneCompletion::Error(__terrane_error_{index});"
        ));
        self.indent -= 1;
        self.line("}");
        self.indent -= 1;
        self.line("}");
        self.indent -= 1;
        self.line("}");
        self.line("TerraneCompletion::Normal");
        self.indent -= 1;
        self.line("})();");
        if let Some(finally) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::FinallyClause)
            .and_then(|clause| clause.children.first())
        {
            self.line(&format!(
                "let __terrane_finally_{index}: TerraneCompletion<{result}> = (|| {{"
            ));
            self.indent += 1;
            let outer_completion = std::mem::replace(&mut self.try_completion, true);
            let outer_propagation = std::mem::replace(&mut self.propagate_errors, true);
            let outer_function_errors = std::mem::replace(&mut self.function_errors, false);
            self.block(finally);
            if block_may_fall_through(finally) {
                self.line("TerraneCompletion::Normal");
            }
            self.function_errors = outer_function_errors;
            self.propagate_errors = outer_propagation;
            self.try_completion = outer_completion;
            self.indent -= 1;
            self.line("})();");
            self.line(&format!("match __terrane_finally_{index} {{"));
            self.indent += 1;
            self.line("TerraneCompletion::Normal => {}");
            self.line(&format!(
                "replacement => __terrane_completion_{index} = replacement,"
            ));
            self.indent -= 1;
            self.line("}");
        }
        self.line(&format!("match __terrane_completion_{index} {{"));
        self.indent += 1;
        if statement_may_fall_through(node) {
            self.line("TerraneCompletion::Normal => {}");
        } else {
            self.line("TerraneCompletion::Normal => __terrane_generated_defect(\"non-fallthrough try completed normally\"),");
        }
        if self.try_completion {
            self.line(
                "TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),",
            );
            self.line("TerraneCompletion::Error(error) => return TerraneCompletion::Error(error),");
        } else if self.function_errors {
            self.line("TerraneCompletion::Return(value) => return Ok(value),");
            self.line("TerraneCompletion::Error(error) => return Err(error),");
        } else if self.propagate_errors {
            self.line("TerraneCompletion::Return(value) => return Ok(Some(value)),");
            self.line("TerraneCompletion::Error(error) => return Err(error),");
        } else {
            self.line("TerraneCompletion::Return(value) => return value,");
            self.line("TerraneCompletion::Error(error) => __terrane_uncaught(error),");
        }
        if self.in_loop {
            if let Some(label) = &self.break_label {
                self.line(&format!("TerraneCompletion::Break => break '{label},"));
            } else {
                self.line("TerraneCompletion::Break => break,");
            }
            if let Some(label) = &self.continue_label {
                self.line(&format!("TerraneCompletion::Continue => break '{label},"));
            } else {
                self.line("TerraneCompletion::Continue => continue,");
            }
        } else {
            self.line("TerraneCompletion::Break | TerraneCompletion::Continue => __terrane_generated_defect(\"loop control escaped a non-loop try\"),");
        }
        self.indent -= 1;
        self.line("}");
    }

    pub(super) fn binding(&mut self, node: &SyntaxNode) {
        let Some((name_index, name_node)) = node
            .children
            .iter()
            .enumerate()
            .find(|(_, child)| child.kind == SyntaxKind::Name)
        else {
            return;
        };
        let name = rust_name(self.text(name_node));
        let binding = self
            .unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == node.span);
        let reference_backed = binding.is_some_and(|binding| self.reference_backed(binding));
        let storage_type = binding
            .and_then(|binding| binding.storage_type)
            .filter(|_| !reference_backed)
            .filter(|_| !binding_span_is_mutated(self.package, self.unit, node.span, true));
        let ty = binding.map(|binding| {
            let value_type = if !binding.destination_arms.is_empty() {
                union_type_name(binding)
            } else if let Some(storage_type) = storage_type {
                rust_type(storage_type).to_owned()
            } else {
                rust_value_type(self.package, binding.value_type.clone())
            };
            if reference_backed {
                format!("std::sync::Arc<std::sync::Mutex<{value_type}>>")
            } else {
                value_type
            }
        });
        let initializer = binding_initializer(node, name_index);
        assert!(
            initializer.is_some() || !self.text(node).contains('='),
            "analyzed initialized value binding must have a selected initializer"
        );
        let mutable = !reference_backed
            && binding.is_some_and(|binding| {
                binding.mutable
                    && !matches!(
                        binding.value_type,
                        ValueType::Reference(_) | ValueType::SharedReference(_)
                    )
            });
        if self
            .package
            .is_lexical_replacement(self.unit, node.span, self.text(name_node))
        {
            self.line(&format!("let _ = &{name};"));
        }
        self.line_start();
        self.output.push_str("let ");
        if mutable {
            self.output.push_str("mut ");
        }
        self.output.push_str(&name);
        if let Some(ty) = ty {
            write!(self.output, ": {ty}").unwrap();
        }
        if let Some(initializer) = initializer {
            let value = if let Some(binding) = binding
                && !binding.destination_arms.is_empty()
            {
                self.union_value(binding, initializer)
            } else if let Some(storage_type) = storage_type {
                self.expression_as(initializer, ValueType::Scalar(storage_type))
            } else if let Some(binding) = binding {
                self.expression_as(initializer, binding.value_type.clone())
            } else {
                self.expression(initializer)
            };
            let value = Self::unwrapped_expression(value);
            let value = if reference_backed {
                format!("std::sync::Arc::new(std::sync::Mutex::new({value}))")
            } else {
                value
            };
            write!(self.output, " = {value}").unwrap();
        }
        self.output.push_str(";\n");
        if initializer.is_some() && !binding_store_value_is_read(self.package, node.span, node.span)
        {
            let borrow = if mutable { "&mut " } else { "&" };
            self.line(&format!("let _ = {borrow}{name};"));
        }
    }

    pub(super) fn postfix(&mut self, node: &SyntaxNode) {
        let Some(value) = node.children.first() else {
            return;
        };
        let operator = &self.source.text()[value.span.end..node.span.end];
        let addition = operator.trim() == "++";
        let value_type = self.value_type(value);
        if let Some(storage) = self.global_storage(value) {
            self.line("{");
            self.indent += 1;
            self.line(&format!(
                "let mut value = {storage}.lock().expect(\"program-global lock poisoned\");"
            ));
            let failure = self.uninitialized_global_failure(value);
            let current = format!("value.clone().unwrap_or_else(|| {failure})");
            let updated = self.postfix_updated_value(&current, value_type, addition, node);
            self.line(&format!("*value = Some({updated});"));
            self.indent -= 1;
            self.line("}");
            return;
        }
        let target = self.expression(value);
        let updated = self.postfix_updated_value(&target, value_type, addition, node);
        self.line(&format!("{target} = {updated};"));
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "the optional recursive value type is matched as a complete lowering decision"
    )]
    pub(super) fn postfix_updated_value(
        &self,
        value: &str,
        value_type: Option<ValueType>,
        addition: bool,
        node: &SyntaxNode,
    ) -> String {
        if value_type == Some(ValueType::Scalar(ScalarType::Int)) {
            let operator = if addition { "+" } else { "-" };
            return format!("{value}.clone() {operator} terrane_int_support::Int::from(1_i128)");
        }
        if matches!(value_type, Some(ValueType::Scalar(ty)) if ty.is_integer()) {
            if addition
                && node
                    .children
                    .first()
                    .is_some_and(|target| self.binding_has_bounded_integer_range(target))
            {
                return format!("{value} + 1");
            }
            let helper = if addition {
                "fixed_addition"
            } else {
                "fixed_subtraction"
            };
            return self.fallible(format!("terrane_int_support::{helper}({value}, 1)"), node);
        }
        unreachable!("semantic analysis validated postfix integer target");
    }

    pub(super) fn if_statement(&mut self, node: &SyntaxNode) {
        let Some(condition) = node.children.first() else {
            return;
        };
        let Some(block) = node.children.get(1) else {
            return;
        };
        let condition = self.control_condition(condition);
        self.line(&format!("if {condition} {{"));
        self.indent += 1;
        self.block(block);
        self.indent -= 1;
        for clause in node.children.iter().skip(2) {
            self.line_start();
            if clause.children.len() == 1 {
                self.output.push_str("} else {\n");
                self.indent += 1;
                self.block(&clause.children[0]);
                self.indent -= 1;
            } else if let [condition, block] = clause.children.as_slice() {
                let condition = self.control_condition(condition);
                writeln!(self.output, "}} else if {condition} {{").unwrap();
                self.indent += 1;
                self.block(block);
                self.indent -= 1;
            }
        }
        self.line("}");
    }
    fn inactive_list_append_bindings(
        &self,
        boundary: &SyntaxNode,
        block: &SyntaxNode,
    ) -> Vec<crate::Span> {
        self.append_only_list_bindings(boundary, block)
            .into_iter()
            .filter(|binding| {
                !self
                    .list_append_borrows
                    .iter()
                    .any(|borrow| borrow.binding == *binding)
            })
            .collect()
    }

    fn begin_list_append_region(
        &mut self,
        append_bindings: Vec<crate::Span>,
        capacity_hint: Option<&(String, String)>,
    ) -> usize {
        let prior_borrow_count = self.list_append_borrows.len();
        if append_bindings.is_empty() {
            return prior_borrow_count;
        }
        self.line("{");
        self.indent += 1;
        for binding_span in append_bindings {
            let binding = self
                .unit
                .typed_bindings
                .iter()
                .find(|binding| binding.span == binding_span)
                .expect("append-only list binding must remain available during lowering");
            let item_type = match &binding.value_type {
                ValueType::List(item) => rust_element_type(self.package, item.clone()),
                _ => unreachable!("append-only list binding must retain its list type"),
            };
            let preallocation_limit = super::LIST_PREALLOCATION_LIMIT_BYTES;
            let vector = format!("__terrane_list_append_{}", self.list_append_counter);
            self.list_append_counter += 1;
            self.line(&format!(
                "let {vector} = {}.make_unique();",
                rust_name(&binding.name)
            ));
            if let Some((start, end)) = capacity_hint {
                self.line(&format!(
                    "if let (Ok(__terrane_start), Ok(__terrane_end)) = (usize::try_from({start}), usize::try_from({end})) {{"
                ));
                self.indent += 1;
                self.line(&format!(
                    "let __terrane_capacity_limit = {preallocation_limit}usize / std::mem::size_of::<{item_type}>().max(1);"
                ));
                self.line(&format!(
                    "{vector}.reserve(__terrane_end.saturating_sub(__terrane_start).min(__terrane_capacity_limit));"
                ));
                self.indent -= 1;
                self.line("}");
            }
            self.list_append_borrows.push(ListAppendBorrow {
                binding: binding_span,
                vector,
            });
        }
        prior_borrow_count
    }

    fn end_list_append_region(&mut self, prior_borrow_count: usize) {
        if self.list_append_borrows.len() <= prior_borrow_count {
            return;
        }
        self.list_append_borrows.truncate(prior_borrow_count);
        self.indent -= 1;
        self.line("}");
    }

    pub(super) fn while_statement(&mut self, node: &SyntaxNode) {
        let [condition, block] = node.children.as_slice() else {
            return;
        };
        let bounded_range = self.bounded_integer_range(condition, block);
        let has_bounded_range = bounded_range.is_some();
        let append_bindings = self.inactive_list_append_bindings(condition, block);
        let capacity_hint = (!append_bindings.is_empty())
            .then(|| self.while_capacity_hint(condition, block))
            .flatten();
        let condition = self.control_condition(condition);
        let prior_borrow_count =
            self.begin_list_append_region(append_bindings, capacity_hint.as_ref());
        self.line(&format!("while {condition} {{"));
        self.indent += 1;
        let outer_continue = self.continue_label.take();
        let outer_break = self.break_label.take();
        let outer_loop = std::mem::replace(&mut self.in_loop, true);
        if let Some(range) = bounded_range {
            self.bounded_integer_ranges.push(range);
        }
        self.block(block);
        if has_bounded_range {
            self.bounded_integer_ranges.pop();
        }
        self.in_loop = outer_loop;
        self.break_label = outer_break;
        self.continue_label = outer_continue;
        self.indent -= 1;
        self.line("}");
        self.end_list_append_region(prior_borrow_count);
    }

    pub(super) fn for_statement(&mut self, node: &SyntaxNode) {
        match node.children.as_slice() {
            [target, collection, block] if target.kind == SyntaxKind::ForTarget => {
                let collection_type = self.value_type(collection);
                let append_bindings = self.inactive_list_append_bindings(collection, block);
                let collection = self.expression(collection);
                let loop_index = self.loop_counter;
                let iterator = format!("__terrane_iterator_{loop_index}");
                self.loop_counter += 1;
                let constructor = match collection_type {
                    Some(ValueType::Scalar(ScalarType::Bytes)) => {
                        format!("terrane_collection_support::bytes_iterator(&({collection}))")
                    }
                    Some(ValueType::Iterator(_)) => format!("&mut ({collection})"),
                    Some(
                        ValueType::List(_)
                        | ValueType::Map(_, _)
                        | ValueType::Set(_)
                        | ValueType::Tuple(_, _)
                        | ValueType::Range
                        | ValueType::UnorderedMap(_, _)
                        | ValueType::UnorderedSet(_),
                    ) => format!(
                        "terrane_collection_support::Iterable::terrane_iterator(&({collection}))"
                    ),
                    _ => format!("terrane_collection_support::string_iterator(&({collection}))"),
                };
                self.line(&format!("let mut {iterator} = {constructor};"));
                let prior_borrow_count = self.begin_list_append_region(append_bindings, None);
                self.line("loop {");
                self.indent += 1;
                self.iteration_target_bindings(target, &iterator, loop_index);
                let outer_continue = self.continue_label.take();
                let outer_break = self.break_label.take();
                let outer_loop = std::mem::replace(&mut self.in_loop, true);
                self.block(block);
                self.in_loop = outer_loop;
                self.break_label = outer_break;
                self.continue_label = outer_continue;
                self.indent -= 1;
                self.line("}");
                self.end_list_append_region(prior_borrow_count);
            }
            [initial, condition, update, block] => {
                self.statement(initial);
                let mut append_bindings = self.inactive_list_append_bindings(condition, block);
                let update_append_bindings = self.append_only_list_bindings(update, block);
                append_bindings.retain(|binding| update_append_bindings.contains(binding));
                let capacity_hint = (!append_bindings.is_empty())
                    .then(|| self.for_capacity_hint(condition, update, block))
                    .flatten();
                let condition = self.control_condition(condition);
                let prior_borrow_count =
                    self.begin_list_append_region(append_bindings, capacity_hint.as_ref());
                let loop_index = self.loop_counter;
                self.loop_counter += 1;
                let continue_label = format!("__terrane_continue_{loop_index}");
                let break_label = format!("__terrane_break_{loop_index}");
                self.line(&format!("'{break_label}: while {condition} {{"));
                self.indent += 1;
                self.line(&format!("'{continue_label}: {{"));
                self.indent += 1;
                let outer_continue = self.continue_label.replace(continue_label);
                let outer_break = self.break_label.replace(break_label);
                let outer_loop = std::mem::replace(&mut self.in_loop, true);
                self.block(block);
                self.in_loop = outer_loop;
                self.break_label = outer_break;
                self.continue_label = outer_continue;
                self.indent -= 1;
                self.line("}");
                self.statement(update);
                self.indent -= 1;
                self.line("}");
                self.end_list_append_region(prior_borrow_count);
            }
            _ => {}
        }
    }

    pub(super) fn iteration_target_bindings(
        &mut self,
        target: &SyntaxNode,
        iterator: &str,
        loop_index: usize,
    ) {
        match target.children.as_slice() {
            [name] => {
                let name_span = name.span;
                let mutable = if binding_span_is_mutated(self.package, self.unit, name.span, true) {
                    "mut "
                } else {
                    ""
                };
                let name = rust_name(self.text(name));
                self.line(&format!("let {mutable}{name} = match {iterator}.next() {{"));
                self.indent += 1;
                self.line("terrane_collection_support::IterationStep::Item(item) => item,");
                self.line("terrane_collection_support::IterationStep::End => break,");
                self.indent -= 1;
                self.line("};");
                if !binding_store_value_is_read(self.package, name_span, name_span) {
                    self.line(&format!("let _ = &{name};"));
                }
            }
            [key, value] => {
                let item = format!("__terrane_item_{loop_index}");
                self.line(&format!("let {item} = match {iterator}.next() {{"));
                self.indent += 1;
                self.line("terrane_collection_support::IterationStep::Item(item) => item,");
                self.line("terrane_collection_support::IterationStep::End => break,");
                self.indent -= 1;
                self.line("};");
                for (target, field) in [(key, "key"), (value, "value")] {
                    let target_span = target.span;
                    let mutable =
                        if binding_span_is_mutated(self.package, self.unit, target_span, true) {
                            "mut "
                        } else {
                            ""
                        };
                    let name = rust_name(self.text(target));
                    self.line(&format!("let {mutable}{name} = {item}.{field};"));
                    if !binding_store_value_is_read(self.package, target_span, target_span) {
                        self.line(&format!("let _ = &{name};"));
                    }
                }
            }
            _ => unreachable!("semantic analysis admitted invalid iteration target arity"),
        }
    }

    pub(super) fn escaped_construction(node: &SyntaxNode) -> ! {
        unreachable!(
            "construction expression at {}..{} escaped call lowering",
            node.span.start, node.span.end
        )
    }
}
