use std::fmt::Write as _;

use crate::{
    ScalarType,
    lowering::{
        emitter::Emitter,
        helpers::{
            block_may_fall_through, function_name, rust_element_type,
            rust_generic_parameter_declarations, rust_name, rust_value_type,
            union_type_name_for_span,
        },
    },
    semantics::{
        FunctionContract, InvocationMode, ValueType, binding_requires_mutable_storage,
        binding_storage_is_replaced, binding_store_value_is_read,
    },
    syntax::{SyntaxKind, SyntaxNode},
};

fn contains_unsafe_call(node: &SyntaxNode) -> bool {
    node.kind == SyntaxKind::UnsafeRustBlock
        || crate::syntax::call_is_unsafe(node)
        || node.children.iter().any(contains_unsafe_call)
}

fn collect_local_functions<'a>(node: &'a SyntaxNode, output: &mut Vec<&'a SyntaxNode>) {
    if node.kind == SyntaxKind::FunctionDeclaration {
        output.push(node);
    } else if node.kind != SyntaxKind::AnonymousFunction {
        for child in &node.children {
            collect_local_functions(child, output);
        }
    }
}
impl Emitter<'_> {
    pub(in crate::lowering::emitter) fn function(&mut self, node: &SyntaxNode) {
        self.emit_function(node, None);
    }

    fn local_function(&mut self, node: &SyntaxNode) {
        let outer_completion = std::mem::replace(&mut self.try_completion, false);
        let outer_loop = std::mem::replace(&mut self.in_loop, false);
        let outer_continue = self.continue_label.take();
        let outer_break = self.break_label.take();
        let outer_error = self.current_error.take();
        let outer_closure_depth = std::mem::replace(&mut self.closure_depth, 0);
        let outer_ranges = std::mem::take(&mut self.bounded_integer_ranges);
        let outer_list_borrows = std::mem::take(&mut self.list_append_borrows);
        let outer_captures = std::mem::take(&mut self.async_mutable_captures);
        let outer_fresh_lists = std::mem::take(&mut self.fresh_empty_lists);
        self.function(node);
        self.try_completion = outer_completion;
        self.in_loop = outer_loop;
        self.continue_label = outer_continue;
        self.break_label = outer_break;
        self.current_error = outer_error;
        self.closure_depth = outer_closure_depth;
        self.bounded_integer_ranges = outer_ranges;
        self.list_append_borrows = outer_list_borrows;
        self.async_mutable_captures = outer_captures;
        self.fresh_empty_lists = outer_fresh_lists;
    }

    fn consuming_method_needs_mutable_self(&self, node: &SyntaxNode) -> bool {
        fn rooted_in_this(emitter: &Emitter<'_>, node: &SyntaxNode) -> bool {
            if node.kind == SyntaxKind::Name {
                return emitter.text(node) == "this";
            }
            node.kind == SyntaxKind::MemberExpression
                && node
                    .children
                    .first()
                    .is_some_and(|receiver| rooted_in_this(emitter, receiver))
        }
        if node.kind == SyntaxKind::Assignment
            && node
                .children
                .first()
                .is_some_and(|target| rooted_in_this(self, target))
        {
            return true;
        }
        if node.kind == SyntaxKind::UnaryExpression
            && self.unary_operator(node).as_deref() == Some("move")
            && node
                .children
                .last()
                .is_some_and(|operand| rooted_in_this(self, operand))
        {
            return true;
        }
        if node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                callee.kind == SyntaxKind::MemberExpression
                    && callee
                        .children
                        .first()
                        .is_some_and(|receiver| rooted_in_this(self, receiver))
            })
        {
            return true;
        }
        node.children
            .iter()
            .any(|child| self.consuming_method_needs_mutable_self(child))
    }

    pub(super) fn object_method(&mut self, node: &SyntaxNode) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("object method must have an analyzed contract");
        let receiver = if contract.name == "destruct" {
            if contract.is_async {
                "mut self: &mut Self"
            } else {
                "&mut self"
            }
        } else {
            match contract.written_invocation_mode {
                InvocationMode::Consuming if self.consuming_method_needs_mutable_self(node) => {
                    "mut self"
                }
                InvocationMode::Consuming => "self",
                InvocationMode::Mutable => "&mut self",
                InvocationMode::Shared => "&self",
            }
        };
        self.emit_function_as(node, Some(receiver), None);
    }

    pub(super) fn object_static_method(&mut self, node: &SyntaxNode) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed static method must have a semantic contract");
        self.emit_function_as(
            node,
            None,
            Some(&format!("terrane_static_{}", rust_name(&contract.name))),
        );
    }
    pub(super) fn object_method_as(&mut self, node: &SyntaxNode, name: &str) {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("object method must have an analyzed contract");
        let receiver = if contract.name == "destruct" {
            if contract.is_async {
                "mut self: &mut Self"
            } else {
                "&mut self"
            }
        } else {
            match contract.written_invocation_mode {
                InvocationMode::Consuming if self.consuming_method_needs_mutable_self(node) => {
                    "mut self"
                }
                InvocationMode::Consuming => "self",
                InvocationMode::Mutable => "&mut self",
                InvocationMode::Shared => "&self",
            }
        };
        self.emit_function_as(node, Some(receiver), Some(name));
    }

    fn emit_function(&mut self, node: &SyntaxNode, receiver: Option<&str>) {
        self.emit_function_as(node, receiver, None);
    }

    fn emit_select_cursors(&mut self, function_span: crate::Span) {
        let selections = self
            .unit
            .selections
            .iter()
            .filter(|selection| {
                self.unit
                    .enclosing_function_spans
                    .get(&selection.span.start)
                    .copied()
                    .flatten()
                    == Some(function_span)
            })
            .map(|selection| selection.span.start)
            .collect::<Vec<_>>();
        for selection_start in selections {
            self.line(&format!(
                "let mut __terrane_select_cursor_{selection_start} = 0usize;"
            ));
        }
    }

    pub(in crate::lowering::emitter) fn invocation_scoped_type_generics(
        &self,
        node: &SyntaxNode,
        rust_type: &str,
        lifetimes: &[String],
    ) -> Vec<String> {
        let mut declarations = Vec::new();
        if node.kind == SyntaxKind::CallExpression
            && self.unit.selected_expression_types.contains_key(&(
                node.span.file,
                node.span.start,
                node.span.end,
            ))
            && let Some(callee) = node.children.first()
            && let Some(symbol) = self.package.resolve_name_at(
                self.unit,
                callee.span.start,
                &self.unit.source.text()[callee.span.start..callee.span.end],
            )
            && let Some(item) = self
                .package
                .projection
                .item(&symbol.namespace, &symbol.name)
            && let crate::rust_interop::projection::ProjectedKind::Function(function) = &item.kind
        {
            declarations.extend(
                function
                    .generic_parameters
                    .iter()
                    .filter(|parameter| {
                        rust_type
                            .split(|character: char| {
                                !character.is_ascii_alphanumeric() && character != '_'
                            })
                            .any(|token| token == parameter.name)
                    })
                    .map(|parameter| {
                        (
                            parameter.name.clone(),
                            parameter
                                .rust_bounds
                                .iter()
                                .map(|bound| {
                                    lifetimes.iter().enumerate().fold(
                                        bound.clone(),
                                        |bound, (index, lifetime)| {
                                            let binder = format!("for<{lifetime}>");
                                            let marker =
                                                format!("__terrane_lifetime_binder_{index}");
                                            bound
                                                .replace(&binder, &marker)
                                                .replace(lifetime, "'view")
                                                .replace(&marker, &binder)
                                        },
                                    )
                                })
                                .collect::<Vec<_>>(),
                        )
                    }),
            );
        }
        for child in &node.children {
            declarations.extend(
                self.invocation_scoped_type_generics(child, rust_type, lifetimes)
                    .into_iter()
                    .map(|declaration| {
                        if let Some((name, bounds)) = declaration.split_once(':') {
                            (
                                name.trim().to_owned(),
                                bounds
                                    .split(" + ")
                                    .map(str::trim)
                                    .map(str::to_owned)
                                    .collect(),
                            )
                        } else {
                            (declaration, Vec::new())
                        }
                    }),
            );
        }
        let mut merged =
            std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
        for (name, bounds) in declarations {
            merged.entry(name).or_default().extend(bounds);
        }
        if !lifetimes.is_empty() {
            for bounds in merged.values_mut() {
                bounds.insert("'view".to_owned());
            }
        }
        merged
            .into_iter()
            .map(|(name, bounds)| {
                if bounds.is_empty() {
                    name
                } else {
                    format!(
                        "{name}: {}",
                        bounds.into_iter().collect::<Vec<_>>().join(" + ")
                    )
                }
            })
            .collect()
    }
    fn emit_parameter_union_carriers(&mut self, contract: &FunctionContract) {
        for parameter in &contract.parameters {
            let Some(ValueType::Union(arms)) = self.unit.flow_binding_types.get(&parameter.span)
            else {
                continue;
            };
            let Some(source_type) = parameter.binding_value_type() else {
                continue;
            };
            let Some(index) = arms.iter().position(|arm| arm == &source_type) else {
                continue;
            };
            let name = self.storage_name_at(&parameter.name, parameter.span);
            let carrier = union_type_name_for_span(parameter.span);
            self.line(&format!(
                "let mut {name}: {carrier} = {carrier}::Arm{index}({});",
                self.parameter_source_name(&parameter.name, parameter.span)
            ));
        }
    }
    #[expect(
        clippy::too_many_lines,
        reason = "function lowering preserves one ordered signature and body pipeline"
    )]
    pub(in crate::lowering::emitter) fn emit_function_as(
        &mut self,
        node: &SyntaxNode,
        receiver: Option<&str>,
        name_override: Option<&str>,
    ) {
        self.fresh_empty_lists.clear();

        let contract = self
            .unit
            .functions
            .iter()
            .find(|item| item.span == node.span)
            .expect("analyzed function declaration must have a semantic contract");
        let mut return_type = contract.return_type.clone().map(|return_type| {
            if contract.is_static
                && let ValueType::Object(returned) = &return_type
                && contract.owner.as_deref() == Some(returned.name.as_str())
                && let Some(effective) = &self.current_object
            {
                ValueType::Object(effective.clone())
            } else {
                return_type
            }
        });
        if matches!(
            return_type,
            Some(ValueType::InvocationScopedNative {
                concrete: false,
                ..
            })
        ) {
            return_type = self
                .unit
                .invocation_scoped_function_results
                .get(&(contract.span.file, contract.span.start, contract.span.end))
                .cloned();
        }
        let scoped_lifetime = if let Some(ValueType::InvocationScopedNative {
            rust_type,
            lifetimes,
            ..
        }) = &mut return_type
        {
            for lifetime in lifetimes.iter() {
                *rust_type = rust_type.replace(lifetime, "'view");
            }
            Some("'view")
        } else {
            None
        };
        let reference_lender = self
            .unit
            .reference_return_lenders
            .get(&(contract.span.file, contract.span.start, contract.span.end))
            .copied();
        let scoped_type_generics = return_type
            .as_ref()
            .and_then(|return_type| match return_type {
                ValueType::InvocationScopedNative {
                    rust_type,
                    lifetimes,
                    ..
                } => Some(self.invocation_scoped_type_generics(node, rust_type, lifetimes)),
                _ => None,
            })
            .unwrap_or_default();
        let function_generics = if reference_lender.is_some() {
            "<'a>".to_owned()
        } else {
            let mut generics = Vec::new();
            if scoped_lifetime == Some("'view") {
                generics.push("'view".to_owned());
            }
            generics.extend(scoped_type_generics);
            generics.extend(rust_generic_parameter_declarations(
                self.package,
                &contract.generic_parameters,
            ));
            if generics.is_empty() {
                String::new()
            } else {
                format!("<{}>", generics.join(", "))
            }
        };
        if contract.is_unsafe || contains_unsafe_call(node) {
            self.line("#[allow(unsafe_code)]");
        }
        if receiver.is_none()
            && contract.owner.is_none()
            && (contract.name != "main"
                || self.package.artifact == crate::package::ArtifactKind::DynamicLibrary)
            && !self.unit.bundled
            && !self.package.function_is_referenced(contract.span)
        {
            self.line("#[allow(dead_code)]");
        }
        if contract.name == "destruct" && contract.is_async {
            self.line("#[allow(unused_mut)]");
        }
        self.line_start();
        let name =
            name_override.map_or_else(|| function_name(self.package, contract), str::to_owned);
        let async_main = contract.is_async && contract.name == "main" && receiver.is_none();
        write!(
            self.output,
            "{}{}{}fn {name}{}(",
            if contract.owner.is_some() || (receiver.is_none() && self.unit.bundled) {
                "pub "
            } else {
                ""
            },
            if contract.is_async && !async_main {
                "async "
            } else {
                ""
            },
            if contract.is_unsafe { "unsafe " } else { "" },
            function_generics,
        )
        .unwrap();
        if let Some(receiver) = receiver {
            self.output.push_str(receiver);
        }
        for (index, parameter) in contract.parameters.iter().enumerate() {
            if receiver.is_some() || index != 0 {
                self.output.push_str(", ");
            }
            let binding_type = parameter.binding_value_type();
            let native_mutable_reference = self
                .registry
                .mutable_native_callback_parameters
                .borrow()
                .contains(&(
                    (contract.span.file, contract.span.start, contract.span.end),
                    index,
                ))
                || parameter.mutable
                    && matches!(
                        &binding_type,
                        Some(ValueType::Reference(item))
                            if matches!(
                                item.value_type(),
                                ValueType::Object(identity)
                                    if identity.native_projection.is_some()
                                        || self.package.projection.item(&identity.namespace, &identity.name).is_some()
                            )
                    );
            let ty = match (&binding_type, reference_lender == Some(index)) {
                (Some(ValueType::Reference(item)), _) if native_mutable_reference => {
                    format!(
                        "&{}mut {}",
                        if reference_lender == Some(index) {
                            "'a "
                        } else if scoped_lifetime == Some("'view") {
                            "'view "
                        } else {
                            ""
                        },
                        rust_element_type(self.package, item.clone()),
                    )
                }
                (Some(ValueType::Reference(item)), true) => {
                    format!("&'a {}", rust_element_type(self.package, item.clone()))
                }
                (Some(ValueType::Reference(item)), false) if scoped_lifetime == Some("'view") => {
                    format!("&'view {}", rust_element_type(self.package, item.clone()))
                }
                _ => binding_type.map_or_else(
                    || "i128".to_owned(),
                    |value_type| rust_value_type(self.package, value_type),
                ),
            };
            let mutable = if parameter.mutable && !native_mutable_reference {
                "mut "
            } else {
                ""
            };
            let name = self.parameter_source_name(&parameter.name, parameter.span);
            write!(self.output, "{mutable}{name}: {ty}").unwrap();
        }
        self.output.push(')');
        let function_errors = contract.throws && contract.name != "main";
        if function_errors {
            let result = return_type.clone().map_or_else(
                || "()".to_owned(),
                |value_type| match value_type {
                    ValueType::Reference(item) if reference_lender.is_some() => {
                        format!("&'a {}", rust_element_type(self.package, item))
                    }
                    value_type => rust_value_type(self.package, value_type),
                },
            );
            write!(self.output, " -> Result<{result}, TerraneError>").unwrap();
        } else if let Some(return_type) = return_type.clone()
            && return_type != ValueType::Scalar(ScalarType::None)
        {
            let return_type = match return_type {
                ValueType::Reference(item) if reference_lender.is_some() => {
                    format!("&'a {}", rust_element_type(self.package, item))
                }
                return_type => rust_value_type(self.package, return_type),
            };
            write!(self.output, " -> {return_type}").unwrap();
        }
        let block = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block);
        if block.is_none_or(|block| block.children.is_empty())
            && contract.parameters.is_empty()
            && !async_main
            && !function_errors
        {
            self.output.push_str(" {}\n");
            return;
        }
        if async_main
            && block.is_none_or(|block| block.children.is_empty())
            && contract.parameters.is_empty()
        {
            self.output.push_str(" {\n");
            self.indent += 1;
            self.line("__terrane_run(async move {});");
            self.indent -= 1;
            self.line("}");
            return;
        }
        self.output.push_str(" {\n");
        if async_main {
            self.indent += 1;
            self.line("__terrane_run(async move {");
        }
        let outer_return_type = std::mem::replace(&mut self.return_type, return_type);
        let outer_function_errors = std::mem::replace(&mut self.function_errors, function_errors);
        let outer_propagation = std::mem::replace(&mut self.propagate_errors, function_errors);
        let outer_function = self.current_function.replace(format!(
            "{}::{}",
            self.unit.namespace.trim_end_matches('/'),
            contract.name
        ));
        let outer_parameter_types = std::mem::replace(
            &mut self.parameter_types,
            contract
                .parameters
                .iter()
                .filter_map(|parameter| {
                    parameter
                        .binding_value_type()
                        .map(|value_type| (parameter.name.clone(), value_type))
                })
                .collect(),
        );
        let previous_active_bindings = std::mem::take(&mut self.active_function_bindings);
        self.active_function_bindings = self
            .unit
            .typed_bindings
            .iter()
            .filter(|binding| {
                binding.scope == Some(contract.span)
                    && !matches!(binding.name.as_str(), "self" | "this")
                    && !contract
                        .parameters
                        .iter()
                        .any(|parameter| parameter.span == binding.span)
            })
            .map(|binding| binding.span)
            .collect();
        for parameter in &contract.parameters {
            if matches!(
                self.unit.flow_binding_types.get(&parameter.span),
                Some(ValueType::Union(_))
            ) {
                self.active_function_bindings.insert(parameter.span);
            }
        }
        let previous_local_functions = std::mem::take(&mut self.active_local_functions);
        let mut local_functions = Vec::new();
        if let Some(block) = block {
            collect_local_functions(block, &mut local_functions);
        }
        self.active_local_functions = local_functions
            .iter()
            .map(|function| function.span)
            .collect();
        self.indent += 1;
        for function in &local_functions {
            self.local_function(function);
        }
        self.emit_parameter_union_carriers(contract);
        self.emit_function_storage(contract);
        let unused_parameters = contract
            .parameters
            .iter()
            .filter(|parameter| {
                !binding_store_value_is_read(self.package, parameter.span, parameter.span)
            })
            .map(|parameter| {
                format!(
                    "&{}",
                    self.parameter_source_name(&parameter.name, parameter.span)
                )
            })
            .collect::<Vec<_>>();
        match unused_parameters.as_slice() {
            [] => {}
            [parameter] => self.line(&format!("let _ = {parameter};")),
            parameters => self.line(&format!("let _ = ({});", parameters.join(", "))),
        }
        self.emit_select_cursors(contract.span);
        if let Some(block) = block {
            self.block(block);
        }
        if function_errors
            && contract
                .return_type
                .clone()
                .is_none_or(|ty| ty == ValueType::Scalar(ScalarType::None))
            && block.is_none_or(block_may_fall_through)
        {
            self.line("Ok(())");
        }
        self.active_function_bindings = previous_active_bindings;
        self.active_local_functions = previous_local_functions;
        self.return_type = outer_return_type;
        self.function_errors = outer_function_errors;
        self.propagate_errors = outer_propagation;
        self.parameter_types = outer_parameter_types;
        self.current_function = outer_function;
        self.indent -= 1;
        if async_main {
            self.line("});");
            self.indent -= 1;
        }
        self.line("}");
    }

    fn emit_function_storage(&mut self, contract: &FunctionContract) {
        let unit = self.unit;
        for binding in &unit.typed_bindings {
            let key = (binding.span.file, binding.span.start, binding.span.end);
            if !self.active_function_bindings.contains(&binding.span)
                || contract
                    .parameters
                    .iter()
                    .any(|parameter| parameter.span == binding.span)
                || unit
                    .flow_binding_ids
                    .get(&key)
                    .is_some_and(|identity| *identity != binding.span)
                || !unit
                    .flow_binding_ids
                    .values()
                    .any(|identity| *identity == binding.span)
            {
                continue;
            }
            let name = self.binding_storage_name(binding);
            // Task expressions keep their concrete future type, as ordinary bindings do.
            let ty =
                (!matches!(self.flow_binding_type(binding), ValueType::Task(_, _))).then(|| {
                    self.binding_rust_type(
                        binding,
                        self.binding_scalar_storage_type(binding),
                        self.reference_backed(binding),
                    )
                });
            if self.binding_may_be_unassigned(binding) {
                if let Some(ty) = &ty {
                    self.line(&format!("let mut {name}: Option<{ty}> = None;"));
                } else {
                    self.line(&format!("let mut {name} = None;"));
                }
            } else {
                let requires_mutable = if self.reference_backed(binding) {
                    binding_storage_is_replaced(
                        self.package,
                        unit,
                        binding.span,
                        self.local_binding_closure_writes(),
                        true,
                    )
                } else {
                    binding_requires_mutable_storage(
                        self.package,
                        unit,
                        binding.span,
                        false,
                        self.local_binding_closure_writes(),
                    )
                };
                let mutable = if requires_mutable { "mut " } else { "" };
                if let Some(ty) = &ty {
                    self.line(&format!("let {mutable}{name}: {ty};"));
                } else {
                    self.line(&format!("let {mutable}{name};"));
                }
            }
        }
    }

    pub(in crate::lowering::emitter) fn mutable_native_reference_parameter(
        &self,
        contract: &FunctionContract,
        index: usize,
        binding_type: Option<&ValueType>,
    ) -> bool {
        (contract.parameters[index].mutable
            && matches!(
                binding_type,
                Some(ValueType::Reference(item))
                    if matches!(
                        item.value_type_ref(),
                        ValueType::Object(identity)
                            if identity.native_projection.is_some()
                                || self.package.projection.item(&identity.namespace, &identity.name).is_some()
                    )
            ))
            || self
                .registry
                .mutable_native_callback_parameters
                .borrow()
                .contains(&(
                    (contract.span.file, contract.span.start, contract.span.end),
                    index,
                ))
    }

    #[expect(
        clippy::too_many_lines,
        reason = "closure ownership, contracts, captures, and body lowering form one emission path"
    )]
    pub(in crate::lowering::emitter) fn anonymous_function(&mut self, node: &SyntaxNode) -> String {
        let contract = self
            .unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed closure must have a semantic contract");
        let parameter_layout = contract
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                let binding_type = parameter.binding_value_type();
                let native_mutable_reference =
                    self.mutable_native_reference_parameter(contract, index, binding_type.as_ref());
                let ty = match binding_type {
                    Some(ValueType::Reference(item)) if native_mutable_reference => {
                        format!("&mut {}", rust_element_type(self.package, item))
                    }
                    Some(value_type) => rust_value_type(self.package, value_type),
                    None => "i128".to_owned(),
                };
                let mutable = if parameter.mutable && !native_mutable_reference {
                    "mut "
                } else {
                    ""
                };
                (
                    format!(
                        "{mutable}{}",
                        self.parameter_source_name(&parameter.name, parameter.span)
                    ),
                    ty,
                )
            })
            .collect::<Vec<_>>();
        let parameters = parameter_layout
            .iter()
            .map(|(name, ty)| format!("{name}: {ty}"))
            .collect::<Vec<_>>()
            .join(", ");
        let parameter_names = parameter_layout
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        let parameter_types = parameter_layout
            .iter()
            .map(|(_, ty)| ty.clone())
            .collect::<Vec<_>>();
        let tuple_pattern = match parameter_names.as_slice() {
            [] => "()".to_owned(),
            [name] => format!("({name},)"),
            _ => format!("({})", parameter_names.join(", ")),
        };
        let tuple_type = match parameter_types.as_slice() {
            [] => "()".to_owned(),
            [ty] => format!("({ty},)"),
            _ => format!("({})", parameter_types.join(", ")),
        };
        let stateful_parameters = format!("{tuple_pattern}: {tuple_type}");
        let result = contract
            .return_type
            .clone()
            .unwrap_or(ValueType::Scalar(ScalarType::None));
        let result_type = if contract.throws {
            format!(
                "Result<{}, TerraneError>",
                rust_value_type(self.package, result.clone())
            )
        } else {
            rust_value_type(self.package, result.clone())
        };
        let outer_output = std::mem::take(&mut self.output);
        let outer_indent = self.indent;
        let outer_return_type = self.return_type.replace(result.clone());
        let outer_function_errors = std::mem::replace(&mut self.function_errors, contract.throws);
        let outer_propagation = std::mem::replace(&mut self.propagate_errors, contract.throws);
        let outer_parameter_types = std::mem::replace(
            &mut self.parameter_types,
            contract
                .parameters
                .iter()
                .filter_map(|parameter| {
                    parameter
                        .binding_value_type()
                        .map(|ty| (parameter.name.clone(), ty))
                })
                .collect(),
        );
        let outer_async_mutable_captures = std::mem::take(&mut self.async_mutable_captures);
        let outer_fresh_empty_lists = std::mem::take(&mut self.fresh_empty_lists);
        let outer_active_bindings = std::mem::take(&mut self.active_function_bindings);
        let outer_active_local_functions = std::mem::take(&mut self.active_local_functions);
        if contract.is_async && contract.written_invocation_mode == InvocationMode::Mutable {
            self.async_mutable_captures
                .extend(contract.captures.iter().cloned());
        }
        self.closure_depth += 1;
        self.indent = outer_indent + 1;
        self.emit_select_cursors(contract.span);
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            self.active_function_bindings = self
                .unit
                .typed_bindings
                .iter()
                .filter(|binding| {
                    binding.scope == Some(contract.span)
                        && !matches!(binding.name.as_str(), "self" | "this")
                        && !contract
                            .parameters
                            .iter()
                            .any(|parameter| parameter.span == binding.span)
                })
                .map(|binding| binding.span)
                .collect();
            for parameter in &contract.parameters {
                if matches!(
                    self.unit.flow_binding_types.get(&parameter.span),
                    Some(ValueType::Union(_))
                ) {
                    self.active_function_bindings.insert(parameter.span);
                }
            }
            let mut local_functions = Vec::new();
            collect_local_functions(block, &mut local_functions);
            self.active_local_functions = local_functions
                .iter()
                .map(|function| function.span)
                .collect();
            for function in &local_functions {
                self.local_function(function);
            }
            self.emit_parameter_union_carriers(contract);
            self.emit_function_storage(contract);
            self.block(block);
            if result == ValueType::Scalar(ScalarType::None) && block_may_fall_through(block) {
                self.line(if contract.throws { "Ok(())" } else { "()" });
            }
        }
        self.closure_depth -= 1;
        let body = std::mem::replace(&mut self.output, outer_output);
        self.indent = outer_indent;
        self.return_type = outer_return_type;
        self.function_errors = outer_function_errors;
        self.propagate_errors = outer_propagation;
        self.parameter_types = outer_parameter_types;
        self.async_mutable_captures = outer_async_mutable_captures;
        self.fresh_empty_lists = outer_fresh_empty_lists;
        self.active_function_bindings = outer_active_bindings;
        self.active_local_functions = outer_active_local_functions;
        let (mut captures, mut invocation_captures) =
            self.anonymous_function_captures(node, contract);
        let invocation_guard =
            if contract.is_async && contract.written_invocation_mode == InvocationMode::Mutable {
                captures.push_str("let __terrane_invocation = TerraneAsyncInvocationGate::new(); ");
                invocation_captures
                    .push_str("let __terrane_invocation = __terrane_invocation.share(); ");
                "let _invocation = __terrane_invocation.enter().await;\n"
            } else {
                ""
            };
        let constructor = match contract.written_invocation_mode {
            InvocationMode::Shared => "std::sync::Arc::new",
            InvocationMode::Mutable => {
                self.registry.uses_mutable_callable.set(true);
                "TerraneMutableCallable::new"
            }
            InvocationMode::Consuming => {
                self.registry.uses_consuming_callable.set(true);
                "TerraneConsumingCallable::new"
            }
        };
        let closure_parameters = if contract.written_invocation_mode == InvocationMode::Shared {
            parameters
        } else {
            stateful_parameters
        };
        if contract.is_async {
            format!(
                "{{ {captures}{constructor}(move |{closure_parameters}| -> std::pin::Pin<Box<dyn Future<Output = {result_type}> + Send>> {{ {invocation_captures}Box::pin(async move {{\n{invocation_guard}{body}{}}}) }}) }}",
                "    ".repeat(outer_indent)
            )
        } else {
            format!(
                "{{ {captures}{constructor}(move |{closure_parameters}| -> {result_type} {{\n{body}{}}}) }}",
                "    ".repeat(outer_indent)
            )
        }
    }

    fn anonymous_function_captures(
        &self,
        node: &SyntaxNode,
        contract: &FunctionContract,
    ) -> (String, String) {
        let mut captures = String::new();
        let mut invocation_captures = String::new();
        for capture in &contract.captures {
            let binding = self.unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == *capture && binding.is_visible_at(self.source.id(), node.span.start)
            });
            let name = binding.map_or_else(
                || rust_name(capture),
                |binding| self.binding_storage_name(binding).to_owned(),
            );
            let mutable = if contract.written_invocation_mode == InvocationMode::Mutable {
                "mut "
            } else {
                ""
            };
            let source = if capture == "this" { "self" } else { &name };
            let transfer =
                binding.is_some_and(|binding| self.value_type_owns_resource(&binding.value_type));
            let borrowed = binding.is_some_and(|binding| {
                matches!(binding.value_type, ValueType::Reference(_))
                    && self
                        .unit
                        .reference_provenance
                        .get(&(binding.span.start, binding.span.end))
                        .is_some_and(|provenance| {
                            !self.reference_owner_uses_shared_storage(provenance.owner)
                        })
            });
            if contract.is_async && contract.written_invocation_mode == InvocationMode::Mutable {
                write!(
                    captures,
                    "let {name} = TerraneAsyncMutableState::new({source}.clone()); "
                )
                .expect("writing to a String cannot fail");
                write!(invocation_captures, "let {name} = {name}.share(); ")
                    .expect("writing to a String cannot fail");
                continue;
            }
            if transfer || borrowed {
                write!(captures, "let {mutable}{name} = {source}; ")
                    .expect("writing to a String cannot fail");
            } else {
                write!(captures, "let {mutable}{name} = {source}.clone(); ")
                    .expect("writing to a String cannot fail");
            }
            if borrowed {
                write!(invocation_captures, "let {mutable}{name} = {name}; ")
                    .expect("writing to a String cannot fail");
            } else {
                write!(
                    invocation_captures,
                    "let {mutable}{name} = {name}.clone(); "
                )
                .expect("writing to a String cannot fail");
            }
        }
        (captures, invocation_captures)
    }

    pub(in crate::lowering::emitter) fn block(&mut self, block: &SyntaxNode) {
        for statement in &block.children {
            if statement.kind == SyntaxKind::FunctionDeclaration {
                if !self.active_local_functions.contains(&statement.span) {
                    self.local_function(statement);
                }
            } else {
                self.statement(statement);
            }
        }
    }
}
