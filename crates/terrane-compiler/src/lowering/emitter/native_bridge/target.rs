use super::InvocationSite;
use crate::lowering::emitter::members::GuardScope;
use crate::lowering::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum CallableField {
    Static,
    Instance,
}

pub(super) struct TargetContext<'a> {
    pub(super) contract: Option<&'a FunctionContract>,
    pub(super) native_macro_path: Option<&'a str>,
    pub(super) callable_field: Option<CallableField>,
    /// Declared operation: use its native path, owner, and owner generic binders.
    pub(super) operation: Option<&'a crate::rust_interop::projection::ProjectedFunction>,
    /// Receiver-lifetime-instantiated operation for this invocation's ABI.
    pub(super) projected_function: Option<&'a crate::rust_interop::projection::ProjectedFunction>,
    pub(super) specialization: Option<&'a crate::semantics::ProjectedCallSpecialization>,
    pub(super) shared_async_receiver: bool,
    pub(super) guarded_method_receiver: Option<&'a (String, String)>,
    pub(super) native_owned_argument_future: bool,
    pub(super) is_unsafe: bool,
}

pub(super) struct InvocationTarget {
    pub(super) name: String,
    pub(super) native_input_setup: String,
    pub(super) native_receiver_setup: String,
    pub(super) native_member_receiver: Option<String>,
}

impl Emitter<'_> {
    #[expect(
        clippy::too_many_lines,
        reason = "target selection and receiver dispatch share call-lowering state"
    )]
    pub(super) fn invocation_target(
        &mut self,
        site: &InvocationSite<'_>,
        context: &TargetContext<'_>,
        shared_argument_setup: &mut String,
    ) -> InvocationTarget {
        let InvocationSite {
            node,
            callee,
            arguments,
            source_method_application,
            ..
        } = *site;
        let TargetContext {
            contract,
            native_macro_path,
            callable_field,
            operation,
            projected_function,
            specialization,
            shared_async_receiver,
            guarded_method_receiver,
            native_owned_argument_future,
            is_unsafe,
        } = *context;
        let projected_static_owner = contract.as_ref().and_then(|contract| {
            let owner = contract.owner.as_deref()?;
            let unit = self.package.units.iter().find(|unit| {
                unit.source.id() == contract.span.file && unit.namespace.starts_with("/deps/")
            })?;
            Some(
                unit.descriptors
                    .iter()
                    .find(|object| object.identity.name == owner)
                    .map_or(owner, |object| object.name.as_str()),
            )
        });
        let mut native_member_receiver = None;
        // Source receiver/argument evaluation belongs to the caller's error/async
        // context, not inside the native call's synchronous panic catcher.
        let stage_native_inputs = native_macro_path.is_none()
            && projected_function.is_some_and(|function| {
                !function.is_async && !function.into_future && function.chain_role.is_none()
            })
            && (arguments.children.iter().any(|argument| {
                self.contains_await(argument) || self.expression_throws_synchronously(argument)
            }) || (callee.kind == SyntaxKind::MemberExpression
                && callee.children.first().is_some_and(|receiver| {
                    // Keep temporary-owner borrow graphs in one expression.
                    !matches!(
                        self.value_type(receiver),
                        Some(ValueType::InvocationScopedNative {
                            expression_scoped: true,
                            ..
                        })
                    ) && (self.contains_await(receiver)
                        || self.expression_throws_synchronously(receiver))
                })));
        let mut native_input_setup = String::new();
        let mut native_receiver_setup = String::new();
        let mut native_input_index = 0;
        let mut stage_native_input = |value: String| {
            let name = format!("__terrane_native_input_{native_input_index}");
            native_input_index += 1;
            write!(native_input_setup, "let {name} = {value}; ").unwrap();
            name
        };
        let name = if let Some(path) = native_macro_path {
            path.to_owned()
        } else if let Some(function) = self.source_applied_function(callee) {
            function
        } else if callee.kind == SyntaxKind::ConstructionExpression {
            callee
                .children
                .first()
                .and_then(|designator| self.class_designator(designator))
                .map_or_else(String::new, |object| {
                    let identity = match self.value_type(node) {
                        Some(ValueType::Object(identity))
                            if identity.namespace == object.identity.namespace
                                && identity.name == object.identity.name
                                && identity.is_unsafe == object.identity.is_unsafe =>
                        {
                            identity
                        }
                        _ => object.identity.clone(),
                    };
                    format!(
                        "{}::terrane_construct",
                        rust_source_type_application(self.package, &identity)
                            .replacen('<', "::<", 1)
                    )
                })
        } else if callable_field == Some(CallableField::Static) {
            format!("({})", self.expression(callee))
        } else if let [receiver, member] = callee.children.as_slice()
            && callee.kind == SyntaxKind::StaticMemberExpression
            && let Some(object) = self.class_designator(receiver)
        {
            let applied_identity = crate::semantics::class_designator_identity(self.unit, receiver)
                .unwrap_or_else(|| object.identity.clone());
            let rust_type = rust_source_type_application(self.package, &applied_identity);
            if let Some(target) = specialization.and_then(|plan| plan.native_target.as_ref()) {
                target.clone()
            } else if specialization
                .is_some_and(|specialization| specialization.direct_projected_call)
                && let Some(item) = self
                    .package
                    .projection
                    .item(&object.identity.namespace, &object.identity.name)
                && let Some(projected) = operation
            {
                projected.native_path.clone().unwrap_or_else(|| {
                    format!(
                        "{}::{}",
                        crate::lowering::dependencies::rust_value_path(
                            projected.native_owner.as_deref().unwrap_or(&item.rust_path),
                        ),
                        rust_name(&projected.name)
                    )
                })
            } else if let Some(contract) = contract
                .as_ref()
                .filter(|_| projected_static_owner.is_some())
            {
                let name = crate::lowering::dependencies::projected_static_shim_name(
                    self.package,
                    contract,
                );
                let selected_identity = match self.value_type(node) {
                    Some(ValueType::Object(identity))
                        if contract.owner_identity.as_ref().is_some_and(|owner| {
                            identity.namespace == owner.namespace && identity.name == owner.name
                        }) =>
                    {
                        Some(identity)
                    }
                    _ => None,
                };
                let arguments = selected_identity
                    .as_ref()
                    .and_then(|identity| {
                        self.package
                            .projection
                            .item(&identity.namespace, &identity.name)
                    })
                    .and_then(|item| match &item.kind {
                        crate::rust_interop::projection::ProjectedKind::ForeignType {
                            generic_parameters,
                            ..
                        }
                        | crate::rust_interop::projection::ProjectedKind::Enum {
                            generic_parameters,
                            ..
                        } if !generic_parameters.is_empty() => {
                            selected_identity.as_ref().and_then(|identity| {
                                let generic_parameters = operation
                                    .filter(|projected| projected.native_owner.is_some())
                                    .map_or(generic_parameters.as_slice(), |projected| {
                                        projected.operation_owner_generics.as_slice()
                                    });
                                if generic_parameters.is_empty() {
                                    return None;
                                }
                                Some(
                                    generic_parameters
                                        .iter()
                                        .map(|parameter| {
                                            identity
                                                .native_arguments
                                                .get(&parameter.name)
                                                .and_then(|value| {
                                                    crate::semantics::destination_projected_type(
                                                        self.package,
                                                        value,
                                                    )
                                                    .ok()
                                                })
                                                .map_or_else(
                                                    || "_".to_owned(),
                                                    |projected| projected.rust_type(),
                                                )
                                        })
                                        .collect::<Vec<_>>(),
                                )
                            })
                        }
                        _ => None,
                    });
                arguments.map_or(name.clone(), |arguments| {
                    format!("{name}::<{}>", arguments.join(", "))
                })
            } else {
                format!(
                    "{}::terrane_static_{}",
                    rust_type.replacen('<', "::<", 1),
                    rust_name(self.text(member))
                )
            }
        } else if let [receiver, member] = callee.children.as_slice()
            && callable_field == Some(CallableField::Instance)
        {
            let mode = self
                .value_type(callee)
                .and_then(|value_type| match value_type {
                    ValueType::Function(_, _, effects)
                    | ValueType::AsyncFunction(_, _, _, effects) => Some(effects.modes.written),
                    _ => None,
                });
            if mode == Some(InvocationMode::Mutable) {
                // The owned mutable carrier synchronizes its environment through
                // &self. Borrow the stored payload without cloning its state or
                // requiring a mutable borrow of the initialization slot.
                if let Some((setup, place, _)) = self.projected_field_place_parts(
                    callee,
                    receiver,
                    member,
                    false,
                    GuardScope::Expression,
                ) {
                    shared_argument_setup.push_str(&setup);
                    place
                } else {
                    self.expression(callee)
                }
            } else {
                format!("({})", self.expression(callee))
            }
        } else if callee.kind == SyntaxKind::Name
            && matches!(
                self.value_type(callee),
                Some(ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects))
                    if effects.modes.written == InvocationMode::Consuming
            )
            && let Some(payload) = self.narrowed_storage_name(callee, true, false)
        {
            payload
        } else if let Some(contract) = &contract
            && contract.owner.is_none()
        {
            let lookup_name = is_unsafe.then(|| format!("unsafe::{}", self.text(callee)));
            self.package
                .resolve_name_at(
                    self.unit,
                    callee.span.start,
                    lookup_name.as_deref().unwrap_or_else(|| self.text(callee)),
                )
                .and_then(|symbol| {
                    self.package
                        .projection
                        .item(&symbol.namespace, &symbol.name)
                })
                .filter(|item| {
                    matches!(&item.kind, crate::rust_interop::projection::ProjectedKind::Function(function)
                        if specialization.is_some_and(|specialization| specialization.direct_projected_call)
                            || function.chain_role == Some(crate::rust_interop::projection::ChainRole::Root))
                })
                .map_or_else(
                    || function_name(self.package, contract),
                    |item| item.rust_path.clone(),
                )
        } else if let [receiver, _member] = callee.children.as_slice()
            && let Some(projected) = projected_function
            && let Some(identity) = self.projected_receiver_identity(receiver)
            && let Some(path) = self
                .package
                .projection
                .foreign_rust_path(&identity.namespace, &identity.name)
                .map(str::to_owned)
            && self
                .package
                .projection
                .borrowed_struct_view(&path)
                .is_some()
        {
            let consuming = projected
                .receiver
                .is_some_and(crate::rust_interop::projection::Receiver::consumes);
            let source = if consuming {
                self.consuming_native_receiver_expression(receiver)
            } else if let Some((_, receiver)) = &guarded_method_receiver {
                format!("&*({receiver})")
            } else {
                format!("&({})", self.native_receiver_expression(receiver, false))
            };
            let owner = format!("__terrane_descriptor_owner_{}", node.span.start);
            let view_name = format!("__terrane_descriptor_view_{}", node.span.start);
            write!(shared_argument_setup, "let {owner} = {source}; ").expect("writing cannot fail");
            let (setup, view) = projected_descriptor_view(self.package, &path, &owner, consuming)
                .expect("descriptor view has a native constructor");
            write!(shared_argument_setup, "{setup}let {view_name} = {view}; ")
                .expect("writing cannot fail");
            if let Some(native_path) = &projected.native_path {
                native_member_receiver = Some(if consuming {
                    view_name
                } else {
                    format!("&{view_name}")
                });
                native_path.clone()
            } else {
                format!("({view_name}).{}", rust_name(&projected.name))
            }
        } else if let [receiver, _member] = callee.children.as_slice()
            && let Some(projected) = projected_function
            && let Some(native_path) = &projected.native_path
        {
            let receiver = crate::semantics::ungrouped_expression(receiver);
            let stage_receiver = stage_native_inputs
                || (receiver.kind != SyntaxKind::Name
                    && projected.chain_role.is_none()
                    && !projected.is_async
                    && !projected.into_future
                    && !matches!(
                        self.value_type(receiver),
                        Some(ValueType::InvocationScopedNative {
                            expression_scoped: true,
                            ..
                        })
                    ));
            let receiver = match projected.receiver {
                Some(
                    mode @ (crate::rust_interop::projection::Receiver::Borrow
                    | crate::rust_interop::projection::Receiver::MutableBorrow),
                ) => {
                    let mutable = mode == crate::rust_interop::projection::Receiver::MutableBorrow;
                    if let Some((_, guard)) = &guarded_method_receiver {
                        let borrow = if mutable { "&mut " } else { "&" };
                        format!("{borrow}*({guard})")
                    } else {
                        let (setup, loan) =
                            self.native_receiver_loan(receiver, mutable, stage_receiver);
                        native_receiver_setup.push_str(&setup);
                        if stage_receiver {
                            // Establish the lender's type before a qualified native
                            // invocation imposes its deref target as a coercion context.
                            stage_native_input(loan)
                        } else {
                            loan
                        }
                    }
                }
                Some(crate::rust_interop::projection::Receiver::Move) => {
                    let value = self.consuming_native_receiver_expression(receiver);
                    if stage_receiver {
                        stage_native_input(value)
                    } else {
                        value
                    }
                }
                Some(crate::rust_interop::projection::Receiver::DerefMove(depth)) => {
                    let value = format!(
                        "{}({})",
                        "*".repeat(depth),
                        self.consuming_native_receiver_expression(receiver)
                    );
                    if stage_receiver {
                        stage_native_input(value)
                    } else {
                        value
                    }
                }
                Some(crate::rust_interop::projection::Receiver::BoxedMove) => {
                    let value = format!(
                        "Box::new({})",
                        self.consuming_native_receiver_expression(receiver)
                    );
                    if stage_receiver {
                        stage_native_input(value)
                    } else {
                        value
                    }
                }
                _ => self.receiver_expression(receiver),
            };
            let receiver = if native_owned_argument_future {
                Self::staged_native_future_receiver(&receiver, shared_argument_setup)
            } else {
                receiver
            };
            native_member_receiver = Some(receiver);
            native_path.clone()
        } else if let [receiver, _] = callee.children.as_slice()
            && let Some(
                crate::rust_interop::projection::ProjectedEnumOperation::SelectOptionalVariant {
                    path,
                },
            ) = projected_function.and_then(|operation| operation.enum_operation.as_ref())
        {
            native_member_receiver = Some(self.native_receiver_expression(receiver, false));
            format!("(|input| match input {{ Some({path}(value)) => Some(value), _ => None }})")
        } else if let [receiver, _member] = callee.children.as_slice()
            && let Some(projected) = projected_function
            && matches!(
                self.value_type(receiver),
                Some(ValueType::InvocationScopedNative {
                    expression_scoped: true,
                    ..
                })
            )
        {
            let mutable = matches!(
                projected.receiver,
                Some(crate::rust_interop::projection::Receiver::MutableBorrow)
            );
            let receiver = self.native_receiver_expression(receiver, mutable);
            let receiver = if native_owned_argument_future {
                Self::staged_native_future_receiver(
                    format_args!("&{}({receiver})", if mutable { "mut " } else { "" }),
                    shared_argument_setup,
                )
            } else if stage_native_inputs {
                stage_native_input(if mutable {
                    format!("&mut ({receiver})")
                } else {
                    format!("&({receiver})")
                })
            } else {
                receiver
            };
            format!("({receiver}).{}", projected.name)
        } else if shared_async_receiver
            && let Some(contract) = &contract
            && let [receiver, _member] = callee.children.as_slice()
        {
            let identity = self
                .receiver_value_type(receiver)
                .and_then(|value_type| match value_type {
                    ValueType::Object(identity) => Some(identity),
                    _ => None,
                })
                .expect("shared async receiver resolves to an object type");
            let owner =
                rust_source_type_application(self.package, &identity).replacen('<', "::<", 1);
            format!(
                "{owner}::__terrane_shared_async_{}",
                function_name(self.package, contract)
            )
        } else if contract
            .as_ref()
            .is_some_and(|contract| contract.owner.is_some())
            && let [receiver, _member] = callee.children.as_slice()
            && !self
                .value_type(receiver)
                .is_some_and(|receiver_type| match receiver_type {
                    ValueType::TypeParameter(_) => true,
                    ValueType::Reference(item)
                    | ValueType::MutableReference(item)
                    | ValueType::SharedReference(item) => {
                        matches!(item.value_type(), ValueType::TypeParameter(_))
                    }
                    _ => false,
                })
        {
            let contract = contract.as_ref().expect("method contract exists");
            let projected_receiver = self
                .value_type(receiver)
                .and_then(|value_type| match value_type {
                    ValueType::Object(identity) => Some(identity),
                    ValueType::Reference(item)
                    | ValueType::MutableReference(item)
                    | ValueType::SharedReference(item) => {
                        let ValueType::Object(identity) = item.value_type() else {
                            return None;
                        };
                        Some(identity.clone())
                    }
                    _ => None,
                })
                .and_then(|identity| {
                    self.package
                        .projection
                        .method(&identity.namespace, &identity.name, &contract.name, false)
                        .and_then(|method| method.receiver)
                })
                .or_else(|| projected_function.and_then(|method| method.receiver));
            let receiver_expression = if let Some((_, receiver)) = &guarded_method_receiver {
                receiver.clone()
            } else if !contract.is_async
                && matches!(
                    projected_receiver,
                    Some(
                        crate::rust_interop::projection::Receiver::Borrow
                            | crate::rust_interop::projection::Receiver::MutableBorrow
                    )
                )
            {
                let mutable = projected_receiver
                    == Some(crate::rust_interop::projection::Receiver::MutableBorrow);
                if stage_native_inputs {
                    let (setup, place) =
                        self.receiver_place_parts(receiver, mutable, GuardScope::Place);
                    native_receiver_setup.push_str(&setup);
                    format!("&{}({place})", if mutable { "mut " } else { "" })
                } else if let Some(narrowed) = self.narrowed_storage_name(receiver, false, mutable)
                {
                    narrowed
                } else {
                    let native = self.native_receiver_expression(receiver, mutable);
                    if mutable {
                        native
                    } else {
                        format!("&{native}")
                    }
                }
            } else if projected_receiver
                .is_some_and(crate::rust_interop::projection::Receiver::consumes)
                || contract.written_invocation_mode == InvocationMode::Consuming
            {
                let owned = self.consuming_native_receiver_expression(receiver);
                if projected_receiver == Some(crate::rust_interop::projection::Receiver::BoxedMove)
                {
                    format!("Box::new({owned})")
                } else if let Some(crate::rust_interop::projection::Receiver::DerefMove(depth)) =
                    projected_receiver
                {
                    format!("{}({owned})", "*".repeat(depth))
                } else {
                    owned
                }
            } else if contract.written_invocation_mode == InvocationMode::Shared
                && receiver.kind == SyntaxKind::IndexExpression
                && let Some((setup, place)) = self.borrowed_index_place_parts(receiver)
            {
                shared_argument_setup.push_str(&setup);
                place
            } else if contract.written_invocation_mode == InvocationMode::Mutable {
                if projected_function.is_none() && contract.owner.is_some() {
                    let (setup, place) =
                        self.receiver_place_parts(receiver, true, GuardScope::Expression);
                    shared_argument_setup.push_str(&setup);
                    place
                } else {
                    self.native_receiver_expression(receiver, true)
                }
            } else {
                self.receiver_expression(receiver)
            };
            let receiver = if self.text(receiver) == "this" && self.closure_depth == 0 {
                receiver_expression
            } else if contract.is_async {
                match projected_receiver {
                    Some(crate::rust_interop::projection::Receiver::MutableBorrow) => {
                        self.mutable_receiver_expression(receiver)
                    }
                    Some(crate::rust_interop::projection::Receiver::Borrow) => {
                        format!("&{}", self.native_receiver_expression(receiver, false))
                    }
                    Some(
                        crate::rust_interop::projection::Receiver::Move
                        | crate::rust_interop::projection::Receiver::BoxedMove,
                    ) => receiver_expression,
                    _ if contract.written_invocation_mode == InvocationMode::Mutable => {
                        format!("&mut {receiver_expression}")
                    }
                    _ if contract.written_invocation_mode == InvocationMode::Shared => {
                        format!("&{receiver_expression}")
                    }
                    _ => receiver_expression,
                }
            } else {
                receiver_expression
            };
            let receiver = if native_owned_argument_future {
                Self::staged_native_future_receiver(&receiver, shared_argument_setup)
            } else if stage_native_inputs {
                let receiver = if !contract.is_async
                    && projected_receiver
                        == Some(crate::rust_interop::projection::Receiver::MutableBorrow)
                {
                    format!("&mut ({receiver})")
                } else {
                    receiver
                };
                stage_native_input(receiver)
            } else {
                receiver
            };
            let method_name = if specialization
                .is_some_and(|specialization| specialization.direct_projected_call)
                && let Some(projected) = projected_function
            {
                rust_name(&projected.name)
            } else {
                function_name(self.package, contract)
            };
            format!("({receiver}).{method_name}")
        } else if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && self
                .value_type(receiver)
                .is_some_and(|receiver_type| match receiver_type {
                    ValueType::TypeParameter(_) => true,
                    ValueType::Reference(item)
                    | ValueType::MutableReference(item)
                    | ValueType::SharedReference(item) => {
                        matches!(item.value_type_ref(), ValueType::TypeParameter(_))
                    }
                    _ => false,
                })
            && contract
                .as_ref()
                .is_some_and(|contract| contract.owner.is_some())
        {
            let contract = contract.as_ref().expect("bounded method contract exists");
            let receiver_expression = match contract.written_invocation_mode {
                InvocationMode::Consuming => format!(
                    "Box::new({})",
                    self.consuming_native_receiver_expression(receiver)
                ),
                InvocationMode::Mutable => {
                    let (setup, place) =
                        self.receiver_place_parts(receiver, true, GuardScope::Expression);
                    shared_argument_setup.push_str(&setup);
                    place
                }
                InvocationMode::Shared => {
                    let (setup, place) =
                        self.receiver_place_parts(receiver, false, GuardScope::Expression);
                    shared_argument_setup.push_str(&setup);
                    place
                }
            };
            format!("({receiver_expression}).{}", rust_name(self.text(member)))
        } else {
            self.expression(callee)
        };
        let native_path_generics = projected_function
            .and_then(|projected| {
                projected
                    .native_path
                    .as_deref()
                    .or(projected.native_owner.as_deref())
            })
            .map_or_default(crate::rust_ir::rust_type_parameter_names);
        let name = if native_path_generics.is_empty() {
            name
        } else {
            let mut replacements = specialization.map_or_default(|specialization| {
                specialization
                    .substitutions
                    .iter()
                    .map(|(name, projected)| (name.clone(), projected.rust_type()))
                    .collect::<BTreeMap<_, _>>()
            });
            // The receiver's instantiated identity is the canonical source of owner
            // substitutions for trait-qualified calls whose method has no own generics.
            if let Some(projected) = projected_function
                && let Some(owner) = projected.native_owner.as_deref()
                && let [receiver, _member] = callee.children.as_slice()
                && let Some(identity) = self.projected_receiver_identity(receiver)
            {
                for generic in crate::rust_ir::rust_type_parameter_names(owner) {
                    let value = identity.native_arguments.get(&generic).or_else(|| {
                        identity
                            .native_parameters
                            .iter()
                            .position(|parameter| parameter == &generic)
                            .and_then(|index| identity.type_arguments.get(index))
                    });
                    if let Some(value) = value {
                        replacements
                            .entry(generic)
                            .or_insert_with(|| rust_value_type(self.package, value.clone()));
                    }
                }
            }
            crate::rust_ir::instantiate_rust_generics(&name, &replacements)
        };
        let name = self.specialize_target_name(
            name,
            node,
            projected_function,
            specialization,
            &native_path_generics,
            source_method_application,
        );
        InvocationTarget {
            name,
            native_input_setup,
            native_receiver_setup,
            native_member_receiver,
        }
    }

    fn contextual_native_result(&self, node: &SyntaxNode) -> Option<ValueType> {
        self.unit
            .enclosing_function_spans
            .get(&node.span.start)
            .copied()
            .flatten()
            .and_then(|span| {
                self.unit
                    .invocation_scoped_function_results
                    .get(&(span.file, span.start, span.end))
            })
            .cloned()
            .or_else(|| self.value_type(node))
    }

    fn specialize_target_name(
        &self,
        name: String,
        node: &SyntaxNode,
        projected_function: Option<&crate::rust_interop::projection::ProjectedFunction>,
        specialization: Option<&crate::semantics::ProjectedCallSpecialization>,
        native_path_generics: &BTreeSet<String>,
        source_method_application: Option<&SyntaxNode>,
    ) -> String {
        if let Some(specialization) =
            specialization.filter(|specialization| !specialization.generic_arguments.is_empty())
        {
            let mut generic_arguments = specialization
                .generic_arguments
                .iter()
                .enumerate()
                .filter_map(|(index, argument)| {
                    use crate::rust_interop::projection::ProjectedType;
                    let projected = projected_function;
                    let template =
                        projected.and_then(|function| function.rust_generic_arguments.get(index));
                    // Concrete UFCS owner arguments already belong to the trait path.
                    if matches!(template, Some(ProjectedType::Foreign { .. }))
                        && projected
                            .and_then(|function| function.native_path.as_deref())
                            .is_some_and(|path| path.starts_with('<'))
                    {
                        return None;
                    }
                    // `rust_generic_arguments` is the ordered list of actual native
                    // function generics; destination-only owner binders are not turbofish args.
                    let parameter_name = match template {
                        Some(ProjectedType::Generic(name)) => Some(name.as_str()),
                        None => return None,
                        _ => None,
                    };
                    if parameter_name.is_some_and(|name| native_path_generics.contains(name)) {
                        return None;
                    }
                    let callback_generic = match template {
                        Some(ProjectedType::Callback { .. }) => true,
                        Some(ProjectedType::Generic(name)) => projected.is_some_and(|function| {
                            function.parameters.iter().any(|parameter| {
                                parameter.generic_parameter.as_deref() == Some(name.as_str())
                                    && matches!(parameter.ty, ProjectedType::Callback { .. })
                            })
                        }),
                        _ => false,
                    };
                    let native = argument.rust_type();
                    Some(
                        if callback_generic
                            || projected.is_some_and(|function| {
                                function
                                    .generic_parameters
                                    .iter()
                                    .any(|parameter| parameter.name == native)
                            })
                        {
                            "_".to_owned()
                        } else {
                            self.package
                                .projection
                                .canonical_native_type(&native)
                                .into_owned()
                        },
                    )
                })
                .collect::<Vec<_>>();
            if let (
                Some(projected),
                Some(ValueType::InvocationScopedNative {
                    rust_type: actual, ..
                }),
            ) = (projected_function, self.contextual_native_result(node))
                && let crate::rust_interop::projection::ProjectedType::InvocationScoped {
                    rust_type: template,
                    ..
                } = &projected.result
            {
                let template_arguments = crate::rust_ir::rust_type_arguments(template);
                let actual_arguments = crate::rust_ir::rust_type_arguments(&actual);
                for argument in &mut generic_arguments {
                    if actual_arguments.contains(argument) {
                        continue;
                    }
                    if let Some((_, replacement)) = template_arguments
                        .iter()
                        .zip(&actual_arguments)
                        .find(|(template, _)| *template == argument)
                    {
                        argument.clone_from(replacement);
                    } else if template_arguments.contains(argument) {
                        "_".clone_into(argument);
                    }
                }
            }
            if generic_arguments.is_empty() {
                name
            } else {
                format!("{name}::<{}>", generic_arguments.join(", "))
            }
        } else if let Some((_, arguments)) = source_method_application
            .and_then(|application| self.source_applied_function_parts(application))
        {
            format!("{name}::<{}>", arguments.join(", "))
        } else {
            name
        }
    }
}
