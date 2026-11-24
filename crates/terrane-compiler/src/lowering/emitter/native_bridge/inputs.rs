//! Stage source operands in evaluation order, adapting them to the selected native ABI.
use super::InvocationSite;
use crate::{
    lowering::{
        emitter::native_arguments::{NativeArgumentContext, NativeArgumentLifetime},
        prelude::*,
    },
    rust_interop::projection::{ChainRole, ProjectedField, ProjectedFunction, ProjectedParameter},
    semantics::ProjectedCallSpecialization,
};
pub(super) struct OperandAbi<'a> {
    pub(super) projected_function: Option<&'a ProjectedFunction>,
    pub(super) projected_parameters: Option<&'a [ProjectedParameter]>,
    pub(super) specialization: Option<&'a ProjectedCallSpecialization>,
    pub(super) contract: Option<&'a FunctionContract>,
    pub(super) struct_fields: Option<&'a [ProjectedField]>,
    pub(super) native_struct_view: Option<bool>,
    pub(super) projected_interface_dispatch: bool,
}
pub(super) struct PreparedInputs {
    pub(super) values: Vec<String>,
    pub(super) shared_argument_setup: String,
    pub(super) native_conversion_inputs: Vec<(String, String)>,
    pub(super) native_field_values: Option<Vec<Option<String>>>,
    pub(super) native_field_converted: Vec<bool>,
    pub(super) shared_async_receiver: Option<String>,
    pub(super) guarded_method_receiver: Option<(String, String)>,
    pub(super) native_owned_argument_future: bool,
}
impl Emitter<'_> {
    fn native_macro_expression(&mut self, node: &SyntaxNode, joint_native_graph: bool) -> String {
        use crate::rust_interop::projection::ProjectedType;
        let mut node = node;
        while node.kind == SyntaxKind::GroupExpression && node.children.len() == 1 {
            node = &node.children[0];
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && let Some(item) =
                crate::semantics::projected_macro_for_call(self.package, self.unit, callee)
        {
            let path = item.rust_path.clone();
            let values = arguments
                .children
                .iter()
                .map(|argument| {
                    self.native_macro_expression(
                        argument.children.last().unwrap_or(argument),
                        joint_native_graph,
                    )
                })
                .collect::<Vec<_>>();
            return format!("{path}!({})", values.join(", "));
        }
        let parameter =
            crate::semantics::macro_argument(self.package, self.unit, node, 0, joint_native_graph)
                .expect("semantic macro proof records a concrete native argument");
        if matches!(&parameter.ty, ProjectedType::Generic(name) if name == "__MacroTokens") {
            return self.expression(node);
        }
        if node.kind == SyntaxKind::Literal {
            let token = native_macro_literal(self.text(node));
            return match parameter.ty {
                ProjectedType::Int => format!("{token}_i64"),
                ProjectedType::Float => format!("{token}_f64"),
                _ => token,
            };
        }
        if parameter.borrowed {
            let operand = node
                .children
                .last()
                .filter(|_| node.kind == SyntaxKind::UnaryExpression)
                .unwrap_or(node);
            if let Some(value) =
                self.narrowed_storage_name(operand, false, parameter.mutable_borrow)
            {
                return value;
            }
            let value = self.native_receiver_expression(operand, parameter.mutable_borrow);
            return format!(
                "&{}({value})",
                if parameter.mutable_borrow { "mut " } else { "" }
            );
        }
        let value = self.expression(node);
        projected_argument_expression(&value, &parameter.ty, self.package)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "keep operand evaluation, slot assignment and guard scope in the same ordered pass"
    )]
    pub(super) fn prepare_invocation_inputs(
        &mut self,
        site: &InvocationSite<'_>,
        abi: &OperandAbi<'_>,
        argument_values: Vec<&SyntaxNode>,
        argument_first: bool,
        macro_graph: Option<bool>,
        region_producer: bool,
    ) -> PreparedInputs {
        let InvocationSite {
            node,
            callee,
            arguments,
            ..
        } = *site;
        let OperandAbi {
            projected_function,
            projected_parameters,
            specialization,
            contract,
            struct_fields,
            native_struct_view,
            projected_interface_dispatch,
        } = *abi;
        let projected_chain_role = projected_function.and_then(|f| f.chain_role);
        let projected_chain_root = projected_chain_role == Some(ChainRole::Root);
        let native_struct_construction = struct_fields.is_some();
        let is_macro = macro_graph.is_some();
        let joint_macro_graph = macro_graph == Some(true);
        let shared_async_method = contract.as_ref().is_some_and(|contract| {
            projected_function.is_none()
                && contract.owner.is_some()
                && !contract.is_static
                && contract.is_async
                && matches!(
                    contract.written_invocation_mode,
                    InvocationMode::Shared | InvocationMode::Mutable
                )
        }) && matches!(callee.children.as_slice(), [receiver, _] if self.receiver_needs_live_guard(receiver));
        let shared_async_receiver = if shared_async_method
            && let [receiver, _] = callee.children.as_slice()
        {
            if self.shared_async_receiver && self.text(receiver) == "this" {
                Some(
                    "let __terrane_shared_call_handle = __terrane_shared_handle.clone(); "
                        .to_owned(),
                )
            } else if let Some(storage) = self.static_storage_name(receiver) {
                Some(format!("let __terrane_shared_call_handle = {storage}; "))
            } else {
                match self.value_type(receiver) {
                    Some(ValueType::SharedReference(_)) => Some(format!(
                        "let __terrane_shared_call_handle = ({}).clone(); ",
                        self.expression(receiver)
                    )),
                    Some(ValueType::Reference(_))
                        if self.reference_uses_shared_storage(receiver) =>
                    {
                        Some(format!(
                            "let __terrane_shared_call_handle = ({}).upgrade().expect(\"reference expired\"); ",
                            self.expression(receiver)
                        ))
                    }
                    _ if self.reference_backed_name(receiver).is_some() => Some(format!(
                        "let __terrane_shared_call_handle = {}.clone(); ",
                        self.raw_storage_name(receiver)
                    )),
                    _ => None,
                }
            }
        } else {
            None
        };

        let guarded_method_receiver = if contract.as_ref().is_some_and(|contract| {
            contract.owner.is_some()
                && !contract.is_static
                && !contract.is_async
                && matches!(
                    contract.written_invocation_mode,
                    InvocationMode::Shared | InvocationMode::Mutable
                )
        }) && let [receiver, _] = callee.children.as_slice()
            && self.receiver_needs_live_guard(receiver)
        {
            if let Some(function) = projected_function
                && (region_producer || function.result.needs_borrowed_leaf_conversion())
            {
                Some(self.projected_receiver_guard(
                    receiver,
                    function.receiver
                        == Some(crate::rust_interop::projection::Receiver::MutableBorrow),
                ))
            } else {
                Some(self.live_receiver_parts(receiver))
            }
        } else {
            None
        };
        let mut shared_argument_setup = String::new();
        let mut shared_argument_index = 0;
        let prior_read_guards = self.borrowed_receiver_overrides.len();
        let retained_native_loans = projected_function
            .is_some_and(|function| function.is_async || function.chain_role.is_some());
        let native_owned_argument_future = projected_function
            .is_some_and(|function| function.is_async)
            && (self.direct_await_operand != Some(node.span)
                || arguments.children.iter().any(|argument| {
                    self.receiver_needs_live_guard(argument.children.last().unwrap_or(argument))
                }));
        if (!projected_interface_dispatch
            || contract.as_ref().is_some_and(|contract| {
                contract
                    .parameters
                    .iter()
                    .enumerate()
                    .all(|(index, _)| self.interface_read_parameter(contract, index).is_some())
            }))
            && guarded_method_receiver.is_none()
            && !retained_native_loans
            && projected_parameters.as_ref().is_some_and(|parameters| {
                parameters.iter().all(|parameter| {
                    parameter.preserves_borrowed_storage() && !parameter.mutable_borrow
                })
            })
        {
            self.prepare_read_guards(
                arguments
                    .children
                    .iter()
                    .map(|argument| argument.children.last().unwrap_or(argument)),
                &mut shared_argument_setup,
            );
        }
        let mut native_field_values = None;
        let mut native_field_converted = Vec::new();
        let mut native_conversion_inputs = Vec::new();
        // Choose the ABI before emitting operands. Contract/callable paths must
        // not first emit a discarded, destination-free copy of every subtree.
        let mut values;
        if let Some(contract) = contract {
            let mut ordered = vec![None; contract.parameters.len()];
            if native_struct_construction {
                native_field_converted.resize(contract.parameters.len(), false);
            }
            let mut variadic_values = Vec::new();
            let variadic_index = contract
                .parameters
                .iter()
                .position(|parameter| parameter.variadic);
            let mut positional = 0;
            for argument in &arguments.children {
                let named = argument
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
                let index = named.map_or_else(
                    || {
                        let index = variadic_index
                            .filter(|variadic| positional >= *variadic)
                            .unwrap_or(positional);
                        positional += 1;
                        index
                    },
                    |name| {
                        contract
                            .parameters
                            .iter()
                            .position(|parameter| parameter.name == self.text(name))
                            .or_else(|| {
                                // Named slots can use the declared field order even
                                // when the result is not a plain projected object.
                                let fields = struct_fields.or_else(|| {
                                    if callee.kind != SyntaxKind::ConstructionExpression {
                                        return None;
                                    }
                                    let identity =
                                        &self.class_designator(callee.children.first()?)?.identity;
                                    self.package
                                        .projection
                                        .projected_struct(&identity.namespace, &identity.name)
                                        .map(|(_, fields, _)| fields)
                                })?;
                                fields
                                    .iter()
                                    .position(|field| field.name == self.text(name))
                            })
                            .expect("validated named argument")
                    },
                );
                let value = argument.children.last().unwrap_or(argument);
                let parameter = &contract.parameters[index];
                if self.borrows_text_parameter(contract, index) {
                    let expression = if let Some(literal) = self.borrowed_string_literal(value) {
                        literal
                    } else if self.read_borrow_is_stable_in(value, node) {
                        let (setup, reference) = self.operand_reference_parts(value, false);
                        shared_argument_setup.push_str(&setup);
                        format!("AsRef::<str>::as_ref({reference})")
                    } else {
                        let expression =
                            self.expression_as(value, ValueType::Scalar(ScalarType::String));
                        let stem =
                            if crate::semantics::interpolatable::borrows_key(self.unit, contract) {
                                "interpolation_key"
                            } else {
                                "borrowed_text"
                            };
                        let name = self.fresh_read_binding_name(stem);
                        write!(shared_argument_setup, "let {name} = {expression}; ").unwrap();
                        format!("AsRef::<str>::as_ref(&{name})")
                    };
                    ordered[index] = Some(expression);
                    continue;
                }
                let projected_parameter =
                    projected_parameters.and_then(|parameters| parameters.get(index));
                let borrowed_facade_parameter = projected_interface_dispatch
                    && self.interface_read_parameter(contract, index).is_some();
                let borrowed_input_abi = !projected_interface_dispatch || borrowed_facade_parameter;
                let specialized_parameter = specialization
                    .and_then(|specialization| specialization.value_parameters.get(index))
                    .and_then(Option::as_ref);
                let fallback_parameter = if specialized_parameter.is_none() {
                    parameter.element_value_type()
                } else {
                    None
                };
                let lowered = self.lower_native_argument(
                    &NativeArgumentContext {
                        contract,
                        parameter_index: index,
                        operand: value,
                        arguments,
                        projected: projected_parameter,
                        semantic_type: specialized_parameter.or(fallback_parameter.as_ref()),
                        substitutions: specialization
                            .map(|specialization| &specialization.substitutions),
                        borrowed_input_abi,
                        guarded_receiver: guarded_method_receiver.is_some(),
                        lifetime: if native_owned_argument_future {
                            NativeArgumentLifetime::OwnedFuture
                        } else if retained_native_loans {
                            NativeArgumentLifetime::RetainedBorrow
                        } else {
                            NativeArgumentLifetime::Invocation
                        },
                    },
                    &mut shared_argument_setup,
                    &mut shared_argument_index,
                );
                let borrowed_expression = lowered.mode.borrows_input();
                let snapshot_expression = lowered.mode.owns_lender();
                let has_invocation_callback = lowered.has_invocation_callback;
                let mut input = value;
                while input.kind == SyntaxKind::GroupExpression
                    && let [grouped] = input.children.as_slice()
                {
                    input = grouped;
                }
                let unclosed_native_graph = input.kind == SyntaxKind::CallExpression
                    && input.children.first().is_some_and(|callee| {
                        self.projected_function_for_call(
                            callee,
                            crate::syntax::call_is_unsafe(input),
                        )
                        .is_some_and(|function| {
                            matches!(
                                function.chain_role,
                                Some(
                                    crate::rust_interop::projection::ChainRole::Root
                                        | crate::rust_interop::projection::ChainRole::Continue
                                )
                            )
                        })
                    });
                let expression = if !borrowed_expression
                    && !snapshot_expression
                    && !has_invocation_callback
                    && guarded_method_receiver.is_none()
                    && !shared_async_method
                    && !unclosed_native_graph
                    && !matches!(
                        self.value_type(value),
                        Some(ValueType::InvocationScopedNative {
                            expression_scoped: true,
                            ..
                        })
                    )
                    && (native_struct_construction
                        || projected_function.is_some_and(|function| {
                            !function.is_async
                                && !function.into_future
                                && function.chain_role.is_none()
                        })) {
                    // Source completion/await belongs to the caller, not the
                    // native conversion Result closure or panic catcher.
                    let source = format!("__terrane_conversion_input_{index}");
                    native_conversion_inputs.push((source.clone(), lowered.expression));
                    source
                } else {
                    lowered.expression
                };
                let expression = if borrowed_expression
                    && projected_parameter.is_some_and(|parameter| {
                        parameter.preserves_borrowed_storage()
                            && matches!(
                                parameter.ty,
                                crate::rust_interop::projection::ProjectedType::Sequence { .. }
                            )
                    }) {
                    if self.native_field_type(value).is_some_and(|field| matches!(
                        field, crate::rust_interop::projection::ProjectedType::Sequence { item, .. }
                            if item.has_identity_representation()
                    )) {
                        expression
                    } else {
                        format!("({expression}).as_vec()")
                    }
                } else if let Some(projected) = projected_parameter.filter(|parameter| {
                    !has_invocation_callback
                        && !projected_interface_dispatch
                        && !matches!(
                            parameter.ty,
                            crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                        )
                        && (parameter.generic_parameter.is_none()
                            || matches!(
                                parameter.ty,
                                crate::rust_interop::projection::ProjectedType::Callback { .. }
                            ))
                        && (specialization
                            .is_some_and(|specialization| specialization.direct_projected_call)
                            || projected_chain_role.is_some()
                            || callee.kind == SyntaxKind::MemberExpression
                            || native_struct_view == Some(false))
                }) {
                    if native_struct_construction {
                        native_field_converted[index] = true;
                    }
                    // Give an optional source carrier its declared ABI before
                    // mapping its payload into native storage. This also types
                    // an absent carrier without depending on closure inference.
                    let expression = if native_struct_construction
                        && let Some(source_type @ ValueType::Optional(_)) =
                            specialized_parameter.or(fallback_parameter.as_ref())
                    {
                        format!(
                            "{{ let __terrane_field_argument: {} = {expression}; __terrane_field_argument }}",
                            rust_value_type(self.package, source_type.clone())
                        )
                    } else {
                        expression
                    };
                    let sequence_conversion = match (&projected.ty, self.value_type(value)) {
                        (
                            crate::rust_interop::projection::ProjectedType::Sequence {
                                rust_path,
                                item,
                            },
                            Some(ValueType::List(actual)),
                        ) if (rust_path.starts_with("alloc::vec::Vec<")
                            || rust_path.starts_with("std::vec::Vec<"))
                            && (item.has_identity_representation()
                                || matches!(
                                    actual.value_type_ref(),
                                    ValueType::InvocationScopedNative { .. }
                                )) =>
                        {
                            let convert_items = match (item.as_ref(), actual.value_type_ref()) {
                                (
                                    crate::rust_interop::projection::ProjectedType::InvocationScoped {
                                        name: expected,
                                        ..
                                    },
                                    ValueType::InvocationScopedNative { family, .. },
                                ) => expected != &family.name,
                                _ => false,
                            };
                            let consume = if matches!(
                                actual.value_type_ref(),
                                ValueType::InvocationScopedNative { .. }
                            ) {
                                "into_unique_vec"
                            } else {
                                "into_vec"
                            };
                            Some(if projected.borrowed && !convert_items {
                                format!("({expression}).as_vec()")
                            } else if convert_items {
                                format!(
                                    "{expression}.{consume}().into_iter().map(Into::into).collect::<Vec<_>>()"
                                )
                            } else {
                                format!("{expression}.{consume}()")
                            })
                        }
                        _ => None,
                    };
                    sequence_conversion.unwrap_or_else(|| {
                        if projected_function.is_some_and(|function| {
                            !function.is_async && !function.into_future && function.error.is_some()
                        }) && guarded_method_receiver.is_none()
                            && !shared_async_method
                            && matches!(projected.ty, crate::rust_interop::projection::ProjectedType::RustInt(_))
                        {
                            // Checked scalar conversion raises a source error, not
                            // the dependency's Result error. Finish it before invocation.
                            let converted = projected_argument_expression(&expression, &projected.ty, self.package);
                            let fallible = self.fallible(format!(
                                "(|| -> Result<_, crate::TerraneForeignError> {{ Ok({converted}) }})()"
                            ), value);
                            let name = format!("__terrane_checked_native_input_{index}");
                            native_conversion_inputs.push((name.clone(), fallible));
                            return name;
                        }
                        if (projected_function
                            .is_some_and(|function| function.is_async || function.into_future)
                            || guarded_method_receiver.is_some()
                            || shared_async_method)
                            && !projected.ty.has_identity_representation()
                        {
                            // Convert owned inputs before constructing a borrowing
                            // future or acquiring a checked shared receiver guard.
                            let source = format!("__terrane_source_input_{index}");
                            writeln!(shared_argument_setup, "let {source} = {expression};")
                                .expect("writing cannot fail");
                            let converted =
                                projected_argument_expression(&source, &projected.ty, self.package);
                            let converted = format!(
                                "(|| -> Result<_, crate::TerraneForeignError> {{ Ok({converted}) }})()"
                            );
                            self.fallible(converted, value)
                        } else {
                            projected_argument_expression(&expression, &projected.ty, self.package)
                        }
                    })
                } else {
                    expression
                };
                let expression = if let Some(parameter) = projected_parameter.filter(|parameter| {
                    parameter.borrowed
                        && !parameter.mutable_borrow
                        && !borrowed_expression
                        && !projected_interface_dispatch
                        && (specialization
                            .is_some_and(|specialization| specialization.direct_projected_call)
                            || projected_chain_root
                            || callee.kind == SyntaxKind::MemberExpression
                            || matches!(
                                parameter.ty,
                                crate::rust_interop::projection::ProjectedType::Foreign { .. }
                            ))
                }) {
                    match &parameter.ty {
                        crate::rust_interop::projection::ProjectedType::String
                            if parameter.generic_parameter.is_some() =>
                        {
                            format!("({expression}).as_str()")
                        }
                        crate::rust_interop::projection::ProjectedType::Bytes
                            if parameter.generic_parameter.is_some() =>
                        {
                            format!("({expression}).as_slice()")
                        }
                        crate::rust_interop::projection::ProjectedType::Sequence {
                            item, ..
                        } if item.has_identity_representation() => expression,
                        crate::rust_interop::projection::ProjectedType::Foreign { .. } => self
                            .staged_native_argument_reference(
                                value,
                                false,
                                &mut shared_argument_setup,
                                &mut shared_argument_index,
                            ),
                        _ => format!("&{expression}"),
                    }
                } else {
                    expression
                };
                let expression = if native_owned_argument_future && !snapshot_expression {
                    let name = format!("__terrane_native_future_argument_{shared_argument_index}");
                    shared_argument_index += 1;
                    write!(shared_argument_setup, "let {name} = {expression}; ")
                        .expect("writing to a String cannot fail");
                    name
                } else {
                    expression
                };
                let expression = if guarded_method_receiver.is_some() || shared_async_method {
                    let name = format!("__terrane_shared_argument_{shared_argument_index}");
                    shared_argument_index += 1;
                    write!(shared_argument_setup, "let {name} = {expression}; ").unwrap();
                    name
                } else {
                    expression
                };
                if parameter.variadic {
                    variadic_values.push(expression);
                } else {
                    ordered[index] = Some(expression);
                }
            }
            if let Some(index) = variadic_index {
                ordered[index] = Some(format!(
                    "terrane_collection_support::List::new(vec![{}])",
                    variadic_values.join(", ")
                ));
            }
            self.append_defaults(contract, &mut ordered);
            if guarded_method_receiver.is_some() || shared_async_method {
                for expression in ordered.iter_mut().flatten() {
                    let name = format!("__terrane_shared_argument_{shared_argument_index}");
                    shared_argument_index += 1;
                    write!(shared_argument_setup, "let {name} = {expression}; ").unwrap();
                    *expression = name;
                }
            }
            if native_struct_construction {
                native_field_values = Some(ordered);
                values = Vec::new();
            } else {
                values = ordered.into_iter().flatten().collect();
            }
        } else if !is_macro
            && let Some(
                ValueType::Function(parameters, _, _)
                | ValueType::AsyncFunction(parameters, _, _, _),
            ) = self.value_type(callee)
        {
            let variadic = parameters
                .last()
                .filter(|parameter| parameter.is_variadic());
            let fixed = parameters.len() - usize::from(variadic.is_some());
            values = arguments
                .children
                .iter()
                .take(fixed)
                .zip(&parameters)
                .map(|(argument, parameter)| {
                    let value = argument.children.last().unwrap_or(argument);
                    let mutable_native_reference = parameter.requires_mutable_reference();
                    if mutable_native_reference {
                        let receiver = if value.kind == SyntaxKind::UnaryExpression
                            && self.unary_operator(value).as_deref() == Some("mutable ref")
                        {
                            value.children.last().unwrap_or(value)
                        } else {
                            value
                        };
                        let native = self.native_receiver_expression(receiver, true);
                        format!("&mut {native}")
                    } else {
                        self.expression_as(value, parameter.value_type())
                    }
                })
                .collect();
            if let Some(parameter) = variadic {
                let tail = arguments
                    .children
                    .iter()
                    .skip(fixed)
                    .map(|argument| {
                        self.expression_as(
                            argument.children.last().unwrap_or(argument),
                            parameter.value_type(),
                        )
                    })
                    .collect::<Vec<_>>();
                values.push(format!(
                    "terrane_collection_support::List::new(vec![{}])",
                    tail.join(", ")
                ));
            }
        } else {
            values = argument_values
                .into_iter()
                .map(|value| {
                    if is_macro {
                        self.native_macro_expression(value, joint_macro_graph)
                    } else {
                        self.expression(value)
                    }
                })
                .collect::<Vec<_>>();
        }
        self.borrowed_receiver_overrides.truncate(prior_read_guards);
        if argument_first {
            for value in &mut values {
                let name = self.fresh_read_binding_name("interpolation_argument");
                write!(shared_argument_setup, "let {name} = {value}; ").unwrap();
                *value = name;
            }
        }
        PreparedInputs {
            values,
            shared_argument_setup,
            native_conversion_inputs,
            native_field_values,
            native_field_converted,
            shared_async_receiver,
            guarded_method_receiver,
            native_owned_argument_future,
        }
    }
}
