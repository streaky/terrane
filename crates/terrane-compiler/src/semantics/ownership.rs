use super::prelude::*;
mod exclusive_loans;
use exclusive_loans::{
    OwnerAccessWindow, first_owner_access_between, has_exclusive_lineage, owner_paths_overlap,
    owner_place_path, reference_descends_from,
};

fn ungroup(mut node: &SyntaxNode) -> &SyntaxNode {
    while node.kind == SyntaxKind::GroupExpression {
        let Some(child) = node.children.first() else {
            break;
        };
        node = child;
    }
    node
}

fn is_nonclone_field(unit: &SemanticUnit, value: &SyntaxNode) -> Result<bool, SemanticFailure> {
    let value = ungroup(value);
    if value.kind != SyntaxKind::MemberExpression {
        return Ok(false);
    }
    Ok(
        infer_value_type(unit, value, &unit.typed_bindings)?.is_some_and(|value_type| {
            matches!(value_type, ValueType::MutableReference(_))
                || (!matches!(
                    value_type,
                    ValueType::Reference(_) | ValueType::SharedReference(_)
                ) && value_type_contains_nonclone_foreign(unit, &value_type))
        }),
    )
}

fn validate_field_snapshot(unit: &SemanticUnit, value: &SyntaxNode) -> Result<(), SemanticFailure> {
    if is_nonclone_field(unit, value)? {
        return Err(failure(
            &unit.source,
            "T0135",
            "field snapshot cannot copy a non-Clone projected foreign value; borrow the field or use explicit `move` through a mutable receiver",
            ungroup(value).span,
        ));
    }
    Ok(())
}

pub(super) fn validate_async_field_borrow(
    unit: &SemanticUnit,
    value: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if is_nonclone_field(unit, value)? {
        validate_async_receiver_field_borrow(unit, value)?;
    }
    Ok(())
}

fn validate_async_receiver_field_borrow(
    unit: &SemanticUnit,
    value: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if ungroup(value).kind != SyntaxKind::MemberExpression {
        return Ok(());
    }
    let mut root = ungroup(value);
    while matches!(
        root.kind,
        SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
    ) {
        root = ungroup(&root.children[0]);
    }
    if root.kind == SyntaxKind::Name
        && node_text(&unit.source, root) == "this"
        && unit
            .functions_containing(value.span)
            .filter(|function| function.owner.is_some())
            .min_by_key(|function| function.span.end - function.span.start)
            .is_some_and(|function| {
                function.is_async
                    && !function.is_static
                    && function.written_invocation_mode != InvocationMode::Consuming
            })
    {
        return Err(SemanticFailure {
            source: unit.source.clone(),
            diagnostics: vec![Diagnostic::error(
                "T0135",
                "borrowing an inline field of an async receiver requires an unsupported scoped lender",
                ungroup(value).span,
            ).with_help(
                "call a synchronous helper through `this`; for copyable fields, bind an owned snapshot before awaiting; when mutation is permitted, explicitly take an optional field before use",
            )],
        });
    }
    Ok(())
}

fn moved_binding_failure(
    unit: &SemanticUnit,
    binding: usize,
    span: Span,
    already: bool,
) -> SemanticFailure {
    let binding = &unit.typed_bindings[binding];
    if matches!(binding.value_type, ValueType::MutableReference(_)) {
        return SemanticFailure {
            source: unit.source.clone(),
            diagnostics: vec![
                Diagnostic::error(
                    "T0058",
                    format!("`{}` was moved and is no longer available", binding.name),
                    span,
                )
                .with_help("bind a new name with `mutable ref` to establish another loan"),
            ],
        };
    }
    let already = if already { "already " } else { "" };
    failure(
        &unit.source,
        "T0058",
        format!(
            "`{}` was {already}moved and is unavailable until rebound",
            binding.name
        ),
        span,
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "move provenance and its control-flow join remain one auditable analysis"
)]
pub(super) fn validate_moves(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn binding_at(unit: &SemanticUnit, name: &str, position: usize) -> Option<usize> {
        let identity = unit
            .flow_binding_ids
            .get(&(unit.source.id(), position, position + name.len()))
            .copied();
        identity
            .and_then(|span| {
                unit.typed_bindings
                    .iter()
                    .position(|binding| binding.span == span)
            })
            .or_else(|| {
                unit.typed_bindings
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, binding)| {
                        binding.name == name && binding.is_visible_at(unit.source.id(), position)
                    })
                    .map(|(index, _)| index)
            })
    }

    fn noncopyable_value_type(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        value_type: &ValueType,
    ) -> bool {
        if let ValueType::Union(arms) = value_type {
            return arms
                .iter()
                .any(|arm| noncopyable_value_type(package, unit, arm));
        }
        matches!(
            value_type,
            ValueType::MutableReference(_)
                | ValueType::Task(_, _)
                | ValueType::ScopedTask(_, _)
                | ValueType::TaskGroup(_)
                | ValueType::Iterator(_)
                | ValueType::InvocationScopedNative { .. }
        ) || matches!(
            value_type,
            ValueType::Function(_, _, effects)
                | ValueType::AsyncFunction(_, _, _, effects)
                if effects.modes.written == InvocationMode::Consuming
        ) || super::objects::application_is_resource_owning(package, value_type)
            || !super::objects::value_type_is_cloneable(package, unit, value_type)
            || matches!(
                value_type,
                ValueType::Object(identity) if source_iterator_object(unit, identity)
            )
    }

    fn noncopyable_binding(package: &SemanticPackage, unit: &SemanticUnit, binding: usize) -> bool {
        fn binding_is_noncopyable(
            package: &SemanticPackage,
            unit: &SemanticUnit,
            binding_index: usize,
            visiting: &mut BTreeSet<Span>,
        ) -> bool {
            let binding = &unit.typed_bindings[binding_index];
            if !visiting.insert(binding.span) {
                return true;
            }
            let value_type = unit
                .flow_binding_types
                .get(&binding.span)
                .unwrap_or(&binding.value_type);
            let noncopyable = noncopyable_value_type(package, unit, value_type)
                || binding_source_initializer(unit, binding).is_some_and(|initializer| {
                    initializer_captures_noncopyable(package, unit, initializer, visiting)
                });
            visiting.remove(&binding.span);
            noncopyable
        }

        fn initializer_captures_noncopyable(
            package: &SemanticPackage,
            unit: &SemanticUnit,
            node: &SyntaxNode,
            visiting: &mut BTreeSet<Span>,
        ) -> bool {
            if node.kind == SyntaxKind::AnonymousFunction
                && let Some(contract) = super::bindings::callback_contract(package, unit, node)
                && contract.captures.iter().any(|capture| {
                    package
                        .resolve_name_at(unit, node.span.start, capture)
                        .and_then(|symbol| symbol.declaration_span)
                        .and_then(|span| {
                            let captured_index = unit
                                .typed_bindings
                                .iter()
                                .position(|binding| binding.span == span)?;
                            Some(binding_is_noncopyable(
                                package,
                                unit,
                                captured_index,
                                visiting,
                            ))
                        })
                        .unwrap_or(false)
                })
            {
                return true;
            }
            node.children
                .iter()
                .any(|child| initializer_captures_noncopyable(package, unit, child, visiting))
        }

        binding_is_noncopyable(package, unit, binding, &mut BTreeSet::new())
    }

    fn resource_type(package: &SemanticPackage, value_type: &ValueType) -> bool {
        match value_type {
            ValueType::Union(arms) => arms.iter().any(|arm| resource_type(package, arm)),
            _ => super::objects::application_is_resource_owning(package, value_type),
        }
    }

    fn resource_binding(package: &SemanticPackage, unit: &SemanticUnit, binding: usize) -> bool {
        let binding = &unit.typed_bindings[binding];
        resource_type(
            package,
            unit.flow_binding_types
                .get(&binding.span)
                .unwrap_or(&binding.value_type),
        )
    }

    fn source_iterator_object(unit: &SemanticUnit, identity: &ObjectIdentity) -> bool {
        object_method_contract(unit, identity, "next", false).is_some_and(|function| {
            matches!(function.return_type, Some(ValueType::IterationStep(_)))
        })
    }

    fn method_consumes_receiver(
        package: &SemanticPackage,
        object_identity: &ObjectIdentity,
        method_name: &str,
    ) -> bool {
        package
            .projection
            .method(
                &object_identity.namespace,
                &object_identity.name,
                method_name,
                false,
            )
            .is_some_and(|method| {
                method
                    .receiver
                    .is_some_and(crate::rust_interop::projection::Receiver::consumes)
            })
            || method_contract(package, object_identity, method_name, false)
                .is_some_and(|method| method.written_invocation_mode == InvocationMode::Consuming)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "move traversal keeps scope transitions and diagnostics in one ordered dispatch"
    )]
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        moved: &mut BTreeSet<usize>,
        declaration_name: bool,
        resource_objects: &BTreeSet<(u32, usize, usize)>,
    ) -> Result<(), SemanticFailure> {
        if matches!(
            node.kind,
            SyntaxKind::Binding | SyntaxKind::Assignment | SyntaxKind::ReturnStatement
        ) && let Some(value) = node.children.last()
        {
            validate_field_snapshot(unit, value)?;
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && (callee.kind == SyntaxKind::MemberExpression
                || arguments.children.iter().any(|argument| {
                    argument.children.last().is_some_and(|value| {
                        matches!(
                            value.kind,
                            SyntaxKind::MemberExpression | SyntaxKind::GroupExpression
                        )
                    })
                }))
        {
            let projected = super::bindings::projected_function_for_call(
                package,
                unit,
                callee,
                crate::syntax::call_is_unsafe(node),
            );
            if let Some(function) = projected
                && (function.is_async || function.into_future)
                && function.receiver.is_some()
                && !function
                    .receiver
                    .is_some_and(crate::rust_interop::projection::Receiver::consumes)
                && callee.kind == SyntaxKind::MemberExpression
                && let Some(receiver) = callee.children.first()
            {
                validate_async_field_borrow(unit, receiver)?;
            }
            if let Some(contract) = function_contract_for_call(package, unit, callee) {
                if projected.is_none()
                    && contract.owner.is_some()
                    && !contract.is_static
                    && contract.is_async
                    && matches!(
                        contract.written_invocation_mode,
                        InvocationMode::Shared | InvocationMode::Mutable
                    )
                    && callee.kind == SyntaxKind::MemberExpression
                    && let Some(receiver) = callee.children.first()
                    && matches!(
                        infer_value_type(unit, receiver, &unit.typed_bindings)?,
                        Some(ValueType::Object(_))
                    )
                {
                    validate_async_receiver_field_borrow(unit, receiver)?;
                }
                for (index, parameter) in contract.parameters.iter().enumerate() {
                    let borrowed = projected
                        .and_then(|function| function.parameters.get(index))
                        .is_some_and(|parameter| parameter.borrowed)
                        || parameter.element_value_type().is_some_and(|value_type| {
                            matches!(
                                value_type,
                                ValueType::Reference(_)
                                    | ValueType::MutableReference(_)
                                    | ValueType::SharedReference(_)
                            )
                        });
                    if let Some(value) = argument_for_parameter(unit, arguments, contract, index) {
                        if borrowed {
                            validate_async_field_borrow(unit, value)?;
                        } else {
                            validate_field_snapshot(unit, value)?;
                        }
                    }
                }
            }
        }
        if matches!(
            node.kind,
            SyntaxKind::RustBlock | SyntaxKind::UnsafeRustBlock
        ) {
            let identifiers =
                crate::rust_ir::rust_free_value_identifiers(node_text(&unit.source, node));
            if let Some((_, binding)) = identifiers
                .iter()
                .filter_map(|identifier| {
                    crate::lowering::rust_value_binding_at(unit, node.span.start, identifier)
                })
                .find(|(binding_index, _)| {
                    let binding = &unit.typed_bindings[*binding_index];
                    let value_type = unit
                        .flow_binding_types
                        .get(&binding.span)
                        .unwrap_or(&binding.value_type);
                    !matches!(
                        value_type,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::SharedReference(_)
                    ) && noncopyable_binding(package, unit, *binding_index)
                })
            {
                return Err(failure(
                    &unit.source,
                    "T0134",
                    format!(
                        "inline Rust cannot directly access non-copyable binding `{}`; use a typed Terrane adapter",
                        binding.name
                    ),
                    node.span,
                ));
            }
            return Ok(());
        }
        if matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            let mut nested_moved = moved.clone();
            for child in &node.children {
                visit(
                    package,
                    unit,
                    child,
                    &mut nested_moved,
                    false,
                    resource_objects,
                )?;
            }
            if node.kind == SyntaxKind::AnonymousFunction {
                let captured = nested_moved
                    .difference(moved)
                    .copied()
                    .filter(|binding| unit.typed_bindings[*binding].scope != Some(node.span))
                    .collect::<Vec<_>>();
                moved.extend(captured);
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last()
            && unary_operator_text(unit, node).as_deref() == Some("move")
            && operand.kind == SyntaxKind::Name
        {
            let name = node_text(&unit.source, operand);
            if let Some(binding) = binding_at(unit, name, operand.span.start)
                && !moved.insert(binding)
            {
                return Err(moved_binding_failure(unit, binding, operand.span, true));
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last()
            && unary_operator_text(unit, node).as_deref() == Some("await")
            && operand.kind == SyntaxKind::Name
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, operand), operand.span.start)
            && matches!(
                unit.typed_bindings[binding].value_type,
                ValueType::Task(_, _)
            )
        {
            visit(package, unit, operand, moved, false, resource_objects)?;
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, callee), callee.span.start)
            && matches!(
                &unit.typed_bindings[binding].value_type,
                ValueType::Function(_, _, effects)
                    | ValueType::AsyncFunction(_, _, _, effects)
                    if effects.modes.written == InvocationMode::Consuming
            )
        {
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && receiver.kind == SyntaxKind::Name
            && let Ok(Some(ValueType::Object(identity))) =
                infer_value_type(unit, receiver, &unit.typed_bindings)
            && matches!(
                object_field_type(
                    unit,
                    &identity,
                    node_text(&unit.source, member),
                    false,
                ),
                Some(
                    ValueType::Function(_, _, effects)
                        | ValueType::AsyncFunction(_, _, _, effects)
                ) if effects.modes.written == InvocationMode::Consuming
            )
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, receiver), receiver.span.start)
        {
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && receiver.kind == SyntaxKind::Name
            && node_text(&unit.source, member) == "drain"
            && matches!(
                infer_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(ValueType::TaskGroup(_)))
            )
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, receiver), receiver.span.start)
        {
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && matches!(
                infer_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(ValueType::TaskScope | ValueType::TaskGroup(_)))
            )
            && matches!(node_text(&unit.source, member), "spawn" | "join")
            && let Some(argument) = arguments.children.first()
            && let value = argument.children.last().unwrap_or(argument)
            && value.kind == SyntaxKind::Name
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, value), value.span.start)
            && match node_text(&unit.source, member) {
                "spawn" => matches!(
                    unit.typed_bindings[binding].value_type,
                    ValueType::Task(_, _)
                ),
                "join" => matches!(
                    unit.typed_bindings[binding].value_type,
                    ValueType::ScopedTask(_, _)
                ),
                _ => false,
            }
        {
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member, ..] = callee.children.as_slice()
            && receiver.kind != SyntaxKind::Name
            && let Ok(Some(ValueType::Object(object_name))) =
                infer_value_type(unit, receiver, &unit.typed_bindings)
            && method_consumes_receiver(package, &object_name, node_text(&unit.source, member))
            && package
                .projection
                .method(
                    &object_name.namespace,
                    &object_name.name,
                    node_text(&unit.source, member),
                    false,
                )
                .is_none_or(|method| method.chain_role.is_none())
            && resolved_object_span(package, &object_name)
                .is_some_and(|span| resource_objects.contains(&span_key(span)))
        {
            return Err(failure(
                &unit.source,
                "T0101",
                "a resource-consuming call requires a named binding; move the member into a binding first",
                receiver.span,
            ));
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member, ..] = callee.children.as_slice()
            && receiver.kind == SyntaxKind::Name
            && node_text(&unit.source, member) == "close"
            && matches!(
                infer_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(
                    ValueType::ChannelSender(_) | ValueType::ChannelReceiver(_)
                ))
            )
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, receiver), receiver.span.start)
        {
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member, ..] = callee.children.as_slice()
            && receiver.kind == SyntaxKind::Name
            && matches!(
                infer_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(ValueType::Object(object_name)))
                    if method_consumes_receiver(
                        package,
                        &object_name,
                        node_text(&unit.source, member),
                    )
            )
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, receiver), receiver.span.start)
            && resource_binding(package, unit, binding)
        {
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && let Some(parameters) = function_parameters(package, unit, callee)
        {
            for (index, argument) in arguments.children.iter().enumerate() {
                let Some(parameter) = parameters
                    .get(index)
                    .or_else(|| parameters.last().filter(|parameter| parameter.variadic))
                else {
                    continue;
                };
                let Some(expected) = parameter.element_value_type() else {
                    continue;
                };
                let Some(value) = argument.children.last() else {
                    continue;
                };
                let expects_named_resource = match &expected {
                    ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
                    ValueType::Object(name) => resolved_object_span(package, name)
                        .is_some_and(|span| resource_objects.contains(&span_key(span))),
                    ValueType::Function(_, _, effects)
                    | ValueType::AsyncFunction(_, _, _, effects) => {
                        effects.modes.written == InvocationMode::Consuming
                    }
                    _ => false,
                };
                if expects_named_resource
                    && matches!(
                        value.kind,
                        SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
                    )
                    && !value.children.first().is_some_and(|receiver| {
                        receiver.kind == SyntaxKind::Name
                            && node_text(&unit.source, receiver) == "this"
                    })
                {
                    return Err(failure(
                        &unit.source,
                        "T0101",
                        "resource transfer requires a named binding",
                        value.span,
                    ));
                }
            }
            let projected = super::bindings::projected_function_for_call(
                package,
                unit,
                callee,
                crate::syntax::call_is_unsafe(node),
            );
            let transferred = arguments
                .children
                .iter()
                .zip(parameters)
                .enumerate()
                .filter_map(|(index, (argument, parameter))| {
                    let projected_parameter = projected.and_then(|function| {
                        argument
                            .children
                            .first()
                            .filter(|name| {
                                argument.children.len() > 1 && name.kind == SyntaxKind::Name
                            })
                            .and_then(|name| {
                                function.parameters.iter().find(|candidate| {
                                    candidate.name == node_text(&unit.source, name)
                                })
                            })
                            .or_else(|| function.parameters.get(index))
                    });
                    if projected_parameter.is_some_and(|parameter| parameter.borrowed) {
                        return None;
                    }
                    parameter
                        .value_type
                        .as_ref()
                        .filter(|value_type| {
                            matches!(
                                value_type,
                                ValueType::PlatformStreamHandle
                                    | ValueType::PlatformResourceHandle
                                    | ValueType::Object(_)
                                    | ValueType::Function(_, _, _)
                                    | ValueType::AsyncFunction(_, _, _, _)
                                    | ValueType::InvocationScopedNative { .. }
                            )
                        })
                        .and_then(|_| argument.children.last())
                        .filter(|value| value.kind == SyntaxKind::Name)
                        .and_then(|value| {
                            binding_at(unit, node_text(&unit.source, value), value.span.start)
                        })
                        .filter(|binding| noncopyable_binding(package, unit, *binding))
                })
                .collect::<Vec<_>>();
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.extend(transferred);
            return Ok(());
        }
        if node.kind == SyntaxKind::Name && !declaration_name {
            let name = node_text(&unit.source, node);
            if let Some(binding) = binding_at(unit, name, node.span.start)
                && moved.contains(&binding)
            {
                return Err(moved_binding_failure(unit, binding, node.span, false));
            }
            return Ok(());
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
            let transferred = node
                .children
                .last()
                .filter(|initializer| initializer.kind == SyntaxKind::Name)
                .and_then(|initializer| {
                    binding_at(
                        unit,
                        node_text(&unit.source, initializer),
                        initializer.span.start,
                    )
                })
                .filter(|binding| noncopyable_binding(package, unit, *binding));
            let mut skipped_name = false;
            for child in &node.children {
                if !skipped_name && child.kind == SyntaxKind::Name {
                    skipped_name = true;
                    // Write-through assignment uses the reference handle, not a new binding.
                    if node.kind != SyntaxKind::Assignment
                        || !matches!(
                            unit.inferred_value_type(child),
                            Some(ValueType::MutableReference(_))
                        )
                    {
                        continue;
                    }
                }
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            if let Some(binding) = transferred {
                moved.insert(binding);
            }
            let assignment_target = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name);
            let target_binding = assignment_target.and_then(|target| {
                let identity = unit
                    .flow_binding_ids
                    .get(&(target.span.file, target.span.start, target.span.end))
                    .copied();
                identity
                    .and_then(|span| {
                        unit.typed_bindings
                            .iter()
                            .position(|binding| binding.span == span)
                    })
                    .or_else(|| {
                        unit.typed_bindings
                            .iter()
                            .position(|binding| binding.span == node.span)
                    })
                    .or_else(|| {
                        binding_at(unit, node_text(&unit.source, target), target.span.start)
                    })
            });
            if let Some(binding) = target_binding {
                let identity = unit
                    .flow_binding_ids
                    .get(&(node.span.file, node.span.start, node.span.end))
                    .copied();
                let binding = identity
                    .and_then(|span| {
                        unit.typed_bindings
                            .iter()
                            .position(|binding| binding.span == span)
                    })
                    .unwrap_or(binding);
                moved.remove(&binding);
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::TryStatement {
            let entry = moved.clone();
            let try_block = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block);
            let mut try_state = entry.clone();
            if let Some(block) = try_block {
                visit(
                    package,
                    unit,
                    block,
                    &mut try_state,
                    false,
                    resource_objects,
                )?;
            }
            let mut branches = Vec::new();
            if try_block.is_some_and(super::scope_validation::block_may_fall_through) {
                branches.push(try_state.clone());
            }
            for clause in node
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::CatchClause)
            {
                if let Some(block) = clause
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)
                {
                    let mut branch = try_state.clone();
                    visit(package, unit, block, &mut branch, false, resource_objects)?;
                    if super::scope_validation::block_may_fall_through(block) {
                        branches.push(branch);
                    }
                }
            }
            if let Some(finally) = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::FinallyClause)
                .and_then(|clause| clause.children.first())
            {
                for branch in &mut branches {
                    visit(package, unit, finally, branch, false, resource_objects)?;
                }
                if !super::scope_validation::block_may_fall_through(finally) {
                    branches.clear();
                }
            }
            moved.clear();
            moved.extend(branches.into_iter().flatten());
            return Ok(());
        }
        if node.kind == SyntaxKind::MatchStatement {
            let Some(scrutinee) = node.children.first() else {
                return Ok(());
            };
            let mut entry = moved.clone();
            visit(
                package,
                unit,
                scrutinee,
                &mut entry,
                false,
                resource_objects,
            )?;
            let explicit_move = scrutinee.kind == SyntaxKind::UnaryExpression
                && unary_operator_text(unit, scrutinee).as_deref() == Some("move");
            let explicit_borrow = scrutinee.kind == SyntaxKind::UnaryExpression
                && matches!(
                    unary_operator_text(unit, scrutinee).as_deref(),
                    Some("ref" | "mutable ref")
                );
            let operand = if explicit_move || explicit_borrow {
                scrutinee.children.last().unwrap_or(scrutinee)
            } else {
                scrutinee
            };
            let typed_borrow =
                infer_value_type(unit, operand, &unit.typed_bindings)?.is_some_and(|value_type| {
                    matches!(
                        value_type,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::SharedReference(_)
                    )
                });
            if !explicit_borrow
                && !typed_borrow
                && operand.kind == SyntaxKind::Name
                && let Some(binding) =
                    binding_at(unit, node_text(&unit.source, operand), operand.span.start)
                && (explicit_move || noncopyable_binding(package, unit, binding))
            {
                entry.insert(binding);
            }
            let mut branches = Vec::new();
            for case in node.children.iter().skip(1) {
                let mut branch = entry.clone();
                if let Some(body) = case
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)
                {
                    visit(package, unit, body, &mut branch, false, resource_objects)?;
                    if super::scope_validation::block_may_fall_through(body) {
                        branches.push(branch);
                    }
                }
            }
            moved.clear();
            moved.extend(branches.into_iter().flatten());
            return Ok(());
        }
        if node.kind == SyntaxKind::SwitchStatement {
            let mut entry = moved.clone();
            if let Some(selector) = node.switch_selector() {
                visit(package, unit, selector, &mut entry, false, resource_objects)?;
            }
            let mut branches = Vec::new();
            for case in node.switch_cases() {
                if let Some(header) = case
                    .children
                    .first()
                    .filter(|header| header.kind == SyntaxKind::ArgumentList)
                {
                    for expression in &header.children {
                        visit(
                            package,
                            unit,
                            expression,
                            &mut entry,
                            false,
                            resource_objects,
                        )?;
                    }
                } else if let Some(header) = case
                    .children
                    .first()
                    .filter(|header| header.kind != SyntaxKind::MatchCatchAll)
                {
                    visit(package, unit, header, &mut entry, false, resource_objects)?;
                }
                let mut branch = entry.clone();
                if let Some(body) = case.children.last() {
                    visit(package, unit, body, &mut branch, false, resource_objects)?;
                    if super::scope_validation::block_may_fall_through(body) {
                        branches.push(branch);
                    }
                }
            }
            if !node.switch_has_default() {
                branches.push(entry);
            }
            moved.clear();
            moved.extend(branches.into_iter().flatten());
            return Ok(());
        }
        if node.kind == SyntaxKind::SelectStatement {
            let mut entry = moved.clone();
            for case in &node.children {
                if let Some(header) = case.children.first() {
                    visit(package, unit, header, &mut entry, false, resource_objects)?;
                }
            }
            let mut branches = Vec::with_capacity(node.children.len());
            for case in &node.children {
                let mut branch = entry.clone();
                if let Some(body) = case.children.last() {
                    visit(package, unit, body, &mut branch, false, resource_objects)?;
                }
                branches.push(branch);
            }
            moved.clear();
            moved.extend(branches.into_iter().flatten());
            return Ok(());
        }

        if node.kind == SyntaxKind::IfStatement {
            let mut entry = moved.clone();
            for child in &node.children {
                if !matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause) {
                    visit(package, unit, child, &mut entry, false, resource_objects)?;
                }
            }
            let mut branches = Vec::new();
            let mut has_else = false;
            for child in &node.children {
                if matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause) {
                    has_else |= child.kind == SyntaxKind::ElseClause;
                    let mut branch = entry.clone();
                    visit(package, unit, child, &mut branch, false, resource_objects)?;
                    if super::scope_validation::block_may_fall_through(child) {
                        branches.push(branch);
                    }
                }
            }
            if !has_else {
                branches.push(entry);
            }
            moved.clear();
            moved.extend(branches.into_iter().flatten());
            return Ok(());
        }
        if matches!(
            node.kind,
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement
        ) {
            let mut entry = moved.clone();
            let body = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block);
            let collection = (node.kind == SyntaxKind::ForStatement)
                .then(|| {
                    node.children
                        .iter()
                        .rev()
                        .find(|child| child.kind != SyntaxKind::Block)
                })
                .flatten();
            let loop_bindings = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::ForTarget)
                .into_iter()
                .flat_map(|target| {
                    target.children.iter().filter_map(|name| {
                        let span = unit
                            .flow_binding_ids
                            .get(&(name.span.file, name.span.start, name.span.end))
                            .copied()
                            .unwrap_or(name.span);
                        unit.typed_bindings
                            .iter()
                            .position(|binding| binding.span == span)
                    })
                })
                .collect::<BTreeSet<_>>();
            for child in &node.children {
                if Some(child) != body {
                    visit(package, unit, child, &mut entry, false, resource_objects)?;
                }
            }
            if let Some(collection) = collection
                && collection.kind == SyntaxKind::Name
                && let Some(binding) = binding_at(
                    unit,
                    node_text(&unit.source, collection),
                    collection.span.start,
                )
                && noncopyable_binding(package, unit, binding)
            {
                entry.insert(binding);
            }
            if let Some(body) = body {
                let mut after_iteration = entry.clone();
                visit(
                    package,
                    unit,
                    body,
                    &mut after_iteration,
                    false,
                    resource_objects,
                )?;
                // Loop targets are rebound for each iteration; other move state carries across.
                let mut next_iteration = after_iteration.clone();
                for binding in loop_bindings {
                    next_iteration.remove(&binding);
                }
                visit(
                    package,
                    unit,
                    body,
                    &mut next_iteration,
                    false,
                    resource_objects,
                )?;
                entry.extend(after_iteration);
            }
            *moved = entry;
            return Ok(());
        }
        for (index, child) in node.children.iter().enumerate() {
            if !super::scope_flow::is_flow_value_child(node, index, child) {
                continue;
            }
            visit(package, unit, child, moved, false, resource_objects)?;
        }
        Ok(())
    }

    super::field_moves::validate(package)?;
    let resource_objects = package
        .units
        .iter()
        .flat_map(|unit| unit.descriptors.iter())
        .filter(|object| object.resource_owning)
        .map(|object| span_key(object.span))
        .collect();
    for unit in &package.units {
        visit(
            package,
            unit,
            &unit.tree.root,
            &mut BTreeSet::new(),
            false,
            &resource_objects,
        )?;
    }
    Ok(())
}

fn binding_at<'a>(unit: &'a SemanticUnit, node: &SyntaxNode) -> Option<&'a TypedBinding> {
    (node.kind == SyntaxKind::Name).then_some(())?;
    let identity = unit
        .flow_binding_ids
        .get(&(node.span.file, node.span.start, node.span.end))
        .copied();
    identity
        .and_then(|span| {
            unit.typed_bindings
                .iter()
                .find(|binding| binding.span == span)
        })
        .or_else(|| {
            unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == node_text(&unit.source, node)
                    && binding.is_visible_at(unit.source.id(), node.span.start)
            })
        })
}

fn span_is_parameter(node: &SyntaxNode, span: Span) -> bool {
    (node.kind == SyntaxKind::Parameter
        && node.span.start <= span.start
        && node.span.end >= span.end)
        || node
            .children
            .iter()
            .any(|child| span_is_parameter(child, span))
}

fn first_owner_lifetime_end(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    owner: Span,
    after: usize,
    provenances: &BTreeMap<(usize, usize), ReferenceProvenance>,
    mutations_end_lifetime: bool,
) -> Option<Span> {
    fn root_name(mut node: &SyntaxNode) -> Option<&SyntaxNode> {
        loop {
            match node.kind {
                SyntaxKind::MemberExpression | SyntaxKind::IndexExpression => {
                    node = node.children.first()?;
                }
                SyntaxKind::GroupExpression => node = node.children.first()?,
                _ => return (node.kind == SyntaxKind::Name).then_some(node),
            }
        }
    }

    let target = match node.kind {
        SyntaxKind::Binding | SyntaxKind::Assignment | SyntaxKind::PostfixExpression => {
            node.children.first()
        }
        SyntaxKind::UnaryExpression
            if unary_operator_text(unit, node).as_deref() == Some("move") =>
        {
            node.children.last()
        }
        SyntaxKind::CallExpression if mutations_end_lifetime => {
            let lender_reference = unit.typed_bindings.iter().any(|binding| {
                binding.span == owner
                    && matches!(
                        binding.value_type,
                        ValueType::Reference(_) | ValueType::MutableReference(_)
                    )
            });
            if lender_reference {
                None
            } else {
                node.children.first().and_then(|callee| {
                    let (receiver, ty, member) = typed_member_call(unit, callee)?;
                    let family = member
                        .split_once('.')
                        .map_or(member.as_str(), |(family, _)| family);
                    (member_invocation_mode(package, unit, &ty, family) == InvocationMode::Mutable)
                        .then_some(receiver)
                })
            }
        }
        _ => None,
    };
    let target_root = target.and_then(root_name);
    let target_declaration = target_root.and_then(|target| {
        package
            .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            .and_then(|symbol| symbol.declaration_span)
    });
    let target_path =
        target.and_then(|target| owner_place_path(package, unit, target, owner, provenances));
    let writes_through_reference = target_declaration
        .and_then(|declaration| provenances.get(&(declaration.start, declaration.end)))
        .is_some_and(|provenance| provenance.owner == owner);
    let replaces_whole_owner = target_declaration == Some(owner)
        && target_path
            .as_deref()
            .is_some_and(<[ReferenceProjection]>::is_empty);
    let mutates_owner = mutations_end_lifetime
        && target_root.is_some_and(|target| {
            unit.flow_binding_ids
                .get(&(target.span.file, target.span.start, target.span.end))
                == Some(&owner)
                || target_declaration == Some(owner)
        });
    if node.span.start >= after
        && !writes_through_reference
        && (unit.flow_replacements.get(&node.span) == Some(&owner)
            || replaces_whole_owner
            || mutates_owner)
    {
        return Some(node.span);
    }
    node.children.iter().find_map(|child| {
        first_owner_lifetime_end(
            package,
            unit,
            child,
            owner,
            after,
            provenances,
            mutations_end_lifetime,
        )
    })
}

type FunctionKey = (u32, usize, usize);

fn contract_key(contract: &FunctionContract) -> FunctionKey {
    (contract.span.file, contract.span.start, contract.span.end)
}

fn package_function_contract<'a>(
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    call: &SyntaxNode,
) -> Option<&'a FunctionContract> {
    let callee = call.children.first()?;
    let declaration = match callee.kind {
        SyntaxKind::Name => package
            .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
            .and_then(|symbol| symbol.declaration_span),
        _ => None,
    };
    if let Some(declaration) = declaration {
        return package.units.iter().find_map(|candidate| {
            candidate
                .functions
                .iter()
                .find(|contract| contract.span == declaration)
        });
    }
    if callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && let Ok(Some(ValueType::Object(identity))) =
            infer_receiver_value_type(unit, receiver, &unit.typed_bindings)
    {
        return object_method_contract(unit, &identity, node_text(&unit.source, member), false);
    }
    None
}

pub(super) fn argument_for_parameter<'a>(
    unit: &SemanticUnit,
    arguments: &'a SyntaxNode,
    contract: &FunctionContract,
    parameter_index: usize,
) -> Option<&'a SyntaxNode> {
    let parameter = contract.parameters.get(parameter_index)?;
    if let Some(named) = arguments.children.iter().find(|argument| {
        argument.children.len() > 1
            && argument
                .children
                .first()
                .is_some_and(|name| node_text(&unit.source, name) == parameter.name)
    }) {
        return named.children.last();
    }
    arguments
        .children
        .iter()
        .filter(|argument| argument.children.len() <= 1)
        .nth(parameter_index)
        .and_then(|argument| argument.children.last().or(Some(argument)))
}

fn binding_source_initializer<'a>(
    unit: &'a SemanticUnit,
    binding: &TypedBinding,
) -> Option<&'a SyntaxNode> {
    fn find(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
        if node.kind == SyntaxKind::Binding
            && node.span.start <= span.start
            && node.span.end >= span.end
        {
            return node
                .children
                .last()
                .filter(|child| child.kind != SyntaxKind::TypeExpression);
        }
        node.children.iter().find_map(|child| find(child, span))
    }
    find(&unit.tree.root, binding.span)
}

fn symbolic_lender_parameter(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contract: &FunctionContract,
    known: &BTreeMap<FunctionKey, usize>,
    visited_bindings: &mut BTreeSet<FunctionKey>,
) -> Option<usize> {
    match node.kind {
        SyntaxKind::GroupExpression
        | SyntaxKind::MemberExpression
        | SyntaxKind::IndexExpression => {
            return node.children.first().and_then(|child| {
                symbolic_lender_parameter(package, unit, child, contract, known, visited_bindings)
            });
        }
        SyntaxKind::UnaryExpression
            if matches!(
                unary_operator_text(unit, node).as_deref(),
                Some("ref" | "mutable ref")
            ) =>
        {
            return node.children.last().and_then(|child| {
                symbolic_lender_parameter(package, unit, child, contract, known, visited_bindings)
            });
        }
        SyntaxKind::CallExpression => {
            let callee = package_function_contract(package, unit, node)?;
            let lender = *known.get(&contract_key(callee))?;
            let argument = argument_for_parameter(unit, node.children.get(1)?, callee, lender)?;
            return symbolic_lender_parameter(
                package,
                unit,
                argument,
                contract,
                known,
                visited_bindings,
            );
        }
        SyntaxKind::Name => {}
        _ => return None,
    }
    let binding = binding_at(unit, node)?;
    if let Some(index) = contract.parameters.iter().position(|parameter| {
        parameter.name == binding.name
            && matches!(
                parameter.value_type,
                Some(
                    ValueType::Reference(_)
                        | ValueType::MutableReference(_)
                        | ValueType::SharedReference(_)
                )
            )
    }) {
        return Some(index);
    }
    if !visited_bindings.insert((binding.span.file, binding.span.start, binding.span.end)) {
        return None;
    }
    binding_source_initializer(unit, binding).and_then(|initializer| {
        symbolic_lender_parameter(
            package,
            unit,
            initializer,
            contract,
            known,
            visited_bindings,
        )
    })
}

fn function_return_values<'a>(
    node: &'a SyntaxNode,
    function: Span,
    values: &mut Vec<&'a SyntaxNode>,
) {
    if node.span.file != function.file {
        return;
    }
    let inside = function.start <= node.span.start && node.span.end <= function.end;
    let enclosing = node.span.start <= function.start && function.end <= node.span.end;
    if (!inside && !enclosing)
        || (node.span != function
            && matches!(
                node.kind,
                SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
            )
            && !enclosing)
    {
        return;
    }
    if inside
        && node.kind == SyntaxKind::ReturnStatement
        && let Some(value) = node.children.first()
    {
        values.push(value);
    }
    for child in &node.children {
        function_return_values(child, function, values);
    }
}

fn infer_reference_return_lenders(package: &SemanticPackage) -> BTreeMap<FunctionKey, usize> {
    let mut known = BTreeMap::new();
    loop {
        let mut changed = false;
        for unit in &package.units {
            for contract in unit.local_functions() {
                if !matches!(
                    contract.return_type,
                    Some(ValueType::Reference(_) | ValueType::MutableReference(_))
                ) || known.contains_key(&contract_key(contract))
                {
                    continue;
                }
                let mut returns = Vec::new();
                function_return_values(&unit.tree.root, contract.span, &mut returns);
                let lenders = returns
                    .iter()
                    .map(|value| {
                        symbolic_lender_parameter(
                            package,
                            unit,
                            value,
                            contract,
                            &known,
                            &mut BTreeSet::new(),
                        )
                    })
                    .collect::<Vec<_>>();
                let Some(Some(lender)) = lenders.first() else {
                    continue;
                };
                if lenders.iter().all(|candidate| *candidate == Some(*lender)) {
                    known.insert(contract_key(contract), *lender);
                    changed = true;
                }
            }
        }
        if !changed {
            return known;
        }
    }
}

fn owner_is_external_lender(unit: &SemanticUnit, position: usize, owner: Span) -> bool {
    unit.containing_function(Span::new(unit.source.id(), position, position))
        .is_some_and(|contract| owner.start < contract.span.start || owner.end > contract.span.end)
}

pub(super) fn expression_provenance(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    proven: &BTreeMap<(usize, usize), ReferenceProvenance>,
    return_lenders: &BTreeMap<FunctionKey, usize>,
) -> Option<ReferenceProvenance> {
    if let Some(binding) = binding_at(unit, node)
        && let Some(mut provenance) = proven.get(&(binding.span.start, binding.span.end)).cloned()
    {
        provenance.external_lender |=
            owner_is_external_lender(unit, node.span.start, provenance.owner);
        return Some(provenance);
    }
    if node.kind == SyntaxKind::UnaryExpression
        && let Some(operator) = unary_operator_text(unit, node)
        && matches!(operator.as_str(), "ref" | "mutable ref" | "shared ref")
    {
        let mut provenance = node
            .children
            .last()
            .and_then(|operand| borrow_origin(package, unit, operand, proven, return_lenders))?;
        provenance.authority = if operator == "mutable ref" {
            ReferenceAuthority::Exclusive
        } else {
            ReferenceAuthority::ReadOnly
        };
        return Some(provenance);
    }
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .and_then(|child| expression_provenance(package, unit, child, proven, return_lenders));
    }
    if node.kind == SyntaxKind::CallExpression {
        let contract = package_function_contract(package, unit, node)?;
        let lender = *return_lenders.get(&contract_key(contract))?;
        let argument = argument_for_parameter(unit, node.children.get(1)?, contract, lender)?;
        let mut provenance = borrow_origin(package, unit, argument, proven, return_lenders)?;
        provenance.path.push(ReferenceProjection::CallResult);
        return Some(provenance);
    }
    None
}

fn borrow_origin(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    proven: &BTreeMap<(usize, usize), ReferenceProvenance>,
    return_lenders: &BTreeMap<FunctionKey, usize>,
) -> Option<ReferenceProvenance> {
    if let Some(binding) = binding_at(unit, node)
        && let Some(existing) = proven.get(&(binding.span.start, binding.span.end))
    {
        let mut existing = existing.clone();
        existing.parent_observer = Some(binding.span);
        return Some(existing);
    }
    if let Some(mut existing) = expression_provenance(package, unit, node, proven, return_lenders) {
        if let Some(binding) = binding_at(unit, node)
            && matches!(
                binding.value_type,
                ValueType::Reference(_)
                    | ValueType::MutableReference(_)
                    | ValueType::SharedReference(_)
            )
        {
            existing.parent_observer = Some(binding.span);
        }
        return Some(existing);
    }
    if let Some(binding) = binding_at(unit, node) {
        return Some(ReferenceProvenance {
            owner: binding.span,
            authority: if matches!(binding.value_type, ValueType::MutableReference(_)) {
                ReferenceAuthority::Exclusive
            } else {
                ReferenceAuthority::ReadOnly
            },
            external_lender: owner_is_external_lender(unit, node.span.start, binding.span),
            lender_parameter: (matches!(
                binding.value_type,
                ValueType::Reference(_)
                    | ValueType::MutableReference(_)
                    | ValueType::SharedReference(_)
            ) && span_is_parameter(&unit.tree.root, binding.span))
            .then_some(binding.span),
            parent_observer: None,
            path: Vec::new(),
            lifetime_end: None,
        });
    }
    match node.kind {
        SyntaxKind::MemberExpression => {
            let [receiver, member] = node.children.as_slice() else {
                return None;
            };
            let mut provenance = borrow_origin(package, unit, receiver, proven, return_lenders)?;
            provenance.path.push(ReferenceProjection::Field(
                node_text(&unit.source, member).to_owned(),
            ));
            Some(provenance)
        }
        SyntaxKind::IndexExpression => {
            let receiver = node.children.first()?;
            if !matches!(
                infer_receiver_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(
                    ValueType::List(_)
                        | ValueType::Tuple(_, _)
                        | ValueType::Map(_, _)
                        | ValueType::UnorderedMap(_, _)
                ))
            ) {
                return None;
            }
            let mut provenance = borrow_origin(package, unit, receiver, proven, return_lenders)?;
            provenance.path.push(ReferenceProjection::Element);
            Some(provenance)
        }
        SyntaxKind::GroupExpression => node
            .children
            .first()
            .and_then(|child| borrow_origin(package, unit, child, proven, return_lenders)),
        _ => None,
    }
}

pub(super) fn validate_implicit_reference_argument(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    value: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    validate_async_field_borrow(unit, value)?;
    if let Some(ValueType::Object(identity)) = infer_value_type(unit, value, &unit.typed_bindings)?
        && package
            .projection
            .item(&identity.namespace, &identity.name)
            .is_none()
    {
        validate_async_receiver_field_borrow(unit, value)?;
    }
    if borrow_origin(
        package,
        unit,
        value,
        &unit.reference_provenance,
        &unit.reference_return_lenders,
    )
    .is_none()
    {
        return Err(failure(
            &unit.source,
            "T0064",
            "borrowing requires a supported owner-proven name, field, or collection element",
            value.span,
        ));
    }
    Ok(())
}

pub(super) fn validate_reference_expression_sites(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn visit(unit: &SemanticUnit, node: &SyntaxNode) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::ReturnStatement
            && let Some(value) = node.children.first()
        {
            let mut expression = value;
            while expression.kind == SyntaxKind::GroupExpression {
                let Some(inner) = expression.children.first() else {
                    break;
                };
                expression = inner;
            }
            if expression.kind == SyntaxKind::UnaryExpression
                && matches!(
                    unary_operator_text(unit, expression).as_deref(),
                    Some("ref" | "mutable ref")
                )
            {
                return Err(SemanticFailure {
                    source: unit.source.clone(),
                    diagnostics: vec![
                        Diagnostic::error(
                            "T0146",
                            "explicit `ref` is not allowed in a return expression",
                            value.span,
                        )
                        .with_help(
                            "borrowing is determined by the function declaration; remove `ref`",
                        ),
                    ],
                });
            }
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(arguments) = node.children.get(1)
        {
            for argument in &arguments.children {
                let mut value = argument.children.last().unwrap_or(argument);
                while value.kind == SyntaxKind::GroupExpression {
                    let Some(inner) = value.children.first() else {
                        break;
                    };
                    value = inner;
                }
                if value.kind == SyntaxKind::UnaryExpression
                    && matches!(
                        unary_operator_text(unit, value).as_deref(),
                        Some("ref" | "mutable ref")
                    )
                {
                    return Err(SemanticFailure {
                        source: unit.source.clone(),
                        diagnostics: vec![Diagnostic::error(
                            "T0146",
                            "explicit reference construction is not allowed at a call argument",
                            argument.span,
                        )
                        .with_help(
                            "borrowing is determined by the selected function declaration; remove the reference operator",
                        )],
                    });
                }
            }
        }
        for child in &node.children {
            visit(unit, child)?;
        }
        Ok(())
    }
    for unit in &package.units {
        visit(unit, &unit.tree.root)?;
    }
    Ok(())
}

fn validate_generic_reference_return_type(
    unit: &SemanticUnit,
    contract: Option<&FunctionContract>,
    value: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if let Some(expected @ ValueType::TypeParameter(_)) =
        contract.and_then(|contract| contract.return_type.as_ref())
        && let Some(ValueType::Reference(inner) | ValueType::MutableReference(inner)) =
            infer_value_type(unit, value, &unit.typed_bindings)?
        && inner.value_type_ref() == expected
    {
        return Err(SemanticFailure {
            source: unit.source.clone(),
            diagnostics: vec![Diagnostic::error(
                "T0015",
                format!("the declared return type `{expected}` is owned, but the returned value is `ref {expected}`"),
                value.span,
            ).with_help(format!(
                "declare the return type as `ref {expected}` to return a borrow, or explicitly create an owned copy before returning"
            ))],
        });
    }
    Ok(())
}

fn value_type_contains_borrowed_callable(value_type: &ValueType) -> bool {
    match value_type {
        ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects) => {
            effects.borrowed_environment
        }
        ValueType::Optional(inner) => value_type_contains_borrowed_callable(inner),
        ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::TaskGroup(inner)
        | ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner) => {
            value_type_contains_borrowed_callable(inner.value_type_ref())
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_contains_borrowed_callable(key.value_type_ref())
                || value_type_contains_borrowed_callable(value.value_type_ref())
        }
        ValueType::Union(arms) => arms.iter().any(value_type_contains_borrowed_callable),
        _ => false,
    }
}

fn reject_borrowed_callable_escape(
    unit: &SemanticUnit,
    value: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if !infer_value_type(unit, value, &unit.typed_bindings)?
        .as_ref()
        .is_some_and(value_type_contains_borrowed_callable)
    {
        return Ok(());
    }
    Err(SemanticFailure {
        source: unit.source.clone(),
        diagnostics: vec![Diagnostic::error(
            "T0151",
            "a callable that captures borrowed references cannot escape its lexical scope",
            value.span,
        ).with_help(
            "invoke the callable before the borrowed owner leaves scope, or pass an explicit reference parameter",
        )],
    })
}
fn validate_borrowed_callable_storage(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let (target, value) = match node.kind {
        SyntaxKind::Binding | SyntaxKind::Assignment => {
            (node.children.first(), node.children.last())
        }
        _ => return Ok(()),
    };
    let Some(value) = value else {
        return Ok(());
    };
    if !infer_value_type(unit, value, &unit.typed_bindings)?
        .as_ref()
        .is_some_and(value_type_contains_borrowed_callable)
    {
        return Ok(());
    }
    let escapes = match node.kind {
        SyntaxKind::Binding => unit.containing_function(node.span).is_none(),
        SyntaxKind::Assignment => {
            let Some(target) = target else {
                return Ok(());
            };
            match target.kind {
                SyntaxKind::Name => {
                    unit.containing_function(node.span).is_none()
                        || !package
                            .resolve_name_at(
                                unit,
                                target.span.start,
                                node_text(&unit.source, target),
                            )
                            .and_then(|symbol| symbol.declaration_span)
                            .is_some_and(|declaration| {
                                unit.typed_bindings
                                    .iter()
                                    .any(|binding| binding.span == declaration)
                            })
                }
                _ => true,
            }
        }
        _ => false,
    };
    if escapes {
        reject_borrowed_callable_escape(unit, value)?;
    }
    Ok(())
}

fn reference_is_captured_closure_lender(
    contract: Option<&FunctionContract>,
    provenance: Option<&ReferenceProvenance>,
) -> bool {
    contract.is_some_and(|contract| {
        contract.is_anonymous
            && provenance.is_some_and(|provenance| {
                provenance.external_lender
                    && !provenance.lender_parameter.is_some_and(|lender| {
                        contract.parameters.iter().any(|parameter| {
                            parameter.span.file == lender.file
                                && parameter.span.start <= lender.start
                                && lender.end <= parameter.span.end
                        })
                    })
            })
    })
}

fn validate_reference_return(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    proven: &BTreeMap<(usize, usize), ReferenceProvenance>,
    return_lenders: &BTreeMap<FunctionKey, usize>,
) -> Result<(), SemanticFailure> {
    if node.kind != SyntaxKind::ReturnStatement {
        return Ok(());
    }
    let Some(value) = node.children.first() else {
        return Ok(());
    };
    let contract = unit.containing_function(value.span);
    reject_borrowed_callable_escape(unit, value)?;
    validate_generic_reference_return_type(unit, contract, value)?;
    if contract.is_some_and(|contract| {
        matches!(contract.return_type, Some(ValueType::MutableReference(_)))
    }) && !super::reference_validation::mutable_reference_creation_is_valid(package, unit, value)
    {
        return Err(failure(
            &unit.source,
            "T0148",
            "a mutable reference result requires an exclusive lender, not a read-only or shared reference",
            value.span,
        ));
    }
    let reference_return = contract.is_some_and(|contract| {
        matches!(
            contract.return_type,
            Some(ValueType::Reference(_) | ValueType::MutableReference(_))
        )
    });
    if !reference_return
        && !matches!(
            infer_value_type(unit, value, &unit.typed_bindings)?,
            Some(ValueType::Reference(_) | ValueType::MutableReference(_))
        )
    {
        return Ok(());
    }
    if let Some(expected @ ValueType::Scalar(_)) =
        contract.and_then(|contract| contract.return_type.as_ref())
        && let Some(
            ValueType::Reference(inner)
            | ValueType::MutableReference(inner)
            | ValueType::SharedReference(inner),
        ) = infer_value_type(unit, value, &unit.typed_bindings)?
        && matches!(inner.value_type_ref(), ValueType::Scalar(_))
        && value_types_compatible(&unit.descriptors, expected, inner.value_type_ref())
    {
        return Ok(());
    }
    let expected_lender = contract
        .and_then(|contract| {
            return_lenders
                .get(&contract_key(contract))
                .and_then(|index| contract.parameters.get(*index))
        })
        .and_then(|parameter| {
            unit.typed_bindings.iter().find(|binding| {
                binding.name == parameter.name
                    && binding.span.start >= parameter.span.start
                    && binding.span.end <= parameter.span.end
            })
        })
        .map(|binding| binding.span);
    let provenance =
        expression_provenance(package, unit, value, proven, return_lenders).or_else(|| {
            reference_return
                .then(|| borrow_origin(package, unit, value, proven, return_lenders))
                .flatten()
        });
    let requires_exclusive = contract.is_some_and(|contract| {
        matches!(contract.return_type, Some(ValueType::MutableReference(_)))
    });
    let parameter_lender = expected_lender.is_some()
        && provenance.as_ref().is_some_and(|provenance| {
            provenance.lender_parameter == expected_lender
                && (!requires_exclusive || provenance.authority == ReferenceAuthority::Exclusive)
        });
    if parameter_lender {
        return Ok(());
    }
    if reference_is_captured_closure_lender(contract, provenance.as_ref()) {
        return Err(SemanticFailure {
            source: unit.source.clone(),
            diagnostics: vec![
                Diagnostic::error(
                    "T0145",
                    "returning a reference borrowed from a callable's captured environment is not supported",
                    value.span,
                )
                .with_help(
                    "pass the owner as an explicit reference parameter, or return an owned value",
                ),
            ],
        });
    }
    Err(failure(
        &unit.source,
        "T0068",
        "returned reference is not proven to originate from one declared reference-parameter lender",
        value.span,
    ))
}

fn record_reference_assignment(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    proven: &mut BTreeMap<(usize, usize), ReferenceProvenance>,
    return_lenders: &BTreeMap<FunctionKey, usize>,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::Assignment
        && let [target, initializer] = node.children.as_slice()
        && target.kind == SyntaxKind::Name
        && let Some(binding) = binding_at(unit, target)
        && matches!(
            binding.value_type,
            ValueType::Reference(_)
                | ValueType::MutableReference(_)
                | ValueType::SharedReference(_)
        )
        && let Some(mut provenance) =
            expression_provenance(package, unit, initializer, proven, return_lenders)
    {
        let key = (binding.span.start, binding.span.end);
        if let Some(source) = binding_at(unit, initializer)
            && source.span != binding.span
            && proven.contains_key(&(source.span.start, source.span.end))
        {
            provenance.parent_observer = Some(source.span);
        }
        if let Some(initial) = proven.get(&key) {
            if package.reference_owner_uses_shared_storage(unit, initial.owner)
                != package.reference_owner_uses_shared_storage(unit, provenance.owner)
            {
                return Err(failure(
                    &unit.source,
                    "T0142",
                    "an ordinary observer cannot be reassigned between plain and shared-backed storage; use a separate observer binding",
                    initializer.span,
                ));
            }
        } else {
            // Inferred declarations are assignments. Later writes must not rebase the
            // declaration's provenance onto an owner absent at earlier reads.
            proven.insert(key, provenance);
        }
    }
    Ok(())
}

fn record_unary_reference_provenance(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    proven: &mut BTreeMap<(usize, usize), ReferenceProvenance>,
    return_lenders: &BTreeMap<FunctionKey, usize>,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::UnaryExpression
        && let Some(operand) = node.children.last()
        && let Some(operator) = unary_operator_text(unit, node)
        && matches!(operator.as_str(), "ref" | "mutable ref" | "shared ref")
    {
        validate_async_field_borrow(unit, operand)?;
        if let Some(ValueType::Object(identity)) =
            infer_value_type(unit, operand, &unit.typed_bindings)?
            && package
                .projection
                .item(&identity.namespace, &identity.name)
                .is_none()
        {
            validate_async_receiver_field_borrow(unit, operand)?;
        }
        let mut provenance = if let Some(provenance) =
            borrow_origin(package, unit, operand, proven, return_lenders)
        {
            provenance
        } else if operator == "shared ref"
            && ungroup(operand).kind == SyntaxKind::CallExpression
            && infer_value_type(unit, operand, &unit.typed_bindings)?.is_some_and(|ty| {
                !matches!(
                    ty,
                    ValueType::Reference(_)
                        | ValueType::MutableReference(_)
                        | ValueType::SharedReference(_)
                )
            })
        {
            ReferenceProvenance {
                owner: operand.span,
                authority: ReferenceAuthority::ReadOnly,
                external_lender: false,
                lender_parameter: None,
                parent_observer: None,
                path: Vec::new(),
                lifetime_end: None,
            }
        } else {
            return Err(failure(
                &unit.source,
                "T0064",
                format!(
                    "`{operator}` requires a supported owner-proven name, field, or collection element"
                ),
                operand.span,
            ));
        };
        provenance.authority = if operator == "mutable ref" {
            ReferenceAuthority::Exclusive
        } else {
            ReferenceAuthority::ReadOnly
        };
        proven.insert((node.span.start, node.span.end), provenance);
    }
    Ok(())
}

fn record_reference_provenance(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    proven: &mut BTreeMap<(usize, usize), ReferenceProvenance>,
    return_lenders: &BTreeMap<FunctionKey, usize>,
) -> Result<(), SemanticFailure> {
    record_unary_reference_provenance(package, unit, node, proven, return_lenders)?;
    if node.kind == SyntaxKind::Binding
        && let Some(binding) = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == node.span)
        && matches!(
            binding.value_type,
            ValueType::Reference(_)
                | ValueType::MutableReference(_)
                | ValueType::SharedReference(_)
        )
        && let Some(initializer) = node
            .children
            .last()
            .filter(|child| child.kind != SyntaxKind::TypeExpression)
        && let Some(mut provenance) =
            expression_provenance(package, unit, initializer, proven, return_lenders)
                .or_else(|| {
                    proven
                        .get(&(initializer.span.start, initializer.span.end))
                        .cloned()
                })
                .or_else(|| {
                    (initializer.kind == SyntaxKind::UnaryExpression
                        && matches!(
                            unary_operator_text(unit, initializer).as_deref(),
                            Some("ref" | "mutable ref" | "shared ref")
                        ))
                    .then(|| initializer.children.last())
                    .flatten()
                    .and_then(|operand| {
                        borrow_origin(package, unit, operand, proven, return_lenders)
                    })
                })
    {
        if let Some(source) = binding_at(unit, initializer)
            && proven.contains_key(&(source.span.start, source.span.end))
        {
            provenance.parent_observer = Some(source.span);
        }
        proven.insert((binding.span.start, binding.span.end), provenance);
    }
    record_reference_assignment(package, unit, node, proven, return_lenders)?;
    validate_borrowed_callable_storage(package, unit, node)?;
    validate_reference_return(package, unit, node, proven, return_lenders)?;
    if node.kind == SyntaxKind::ForStatement
        && let [target, collection, _block] = node.children.as_slice()
        && collection.kind == SyntaxKind::UnaryExpression
        && matches!(
            unary_operator_text(unit, collection).as_deref(),
            Some("ref" | "mutable ref")
        )
        && let Some(mut provenance) =
            expression_provenance(package, unit, collection, proven, return_lenders)
    {
        provenance.path.push(ReferenceProjection::Element);
        for binding in unit.typed_bindings.iter().filter(|binding| {
            binding.span.start >= target.span.start
                && binding.span.end <= target.span.end
                && matches!(
                    binding.value_type,
                    ValueType::Reference(_)
                        | ValueType::MutableReference(_)
                        | ValueType::SharedReference(_)
                )
        }) {
            proven.insert((binding.span.start, binding.span.end), provenance.clone());
        }
    }
    for child in &node.children {
        record_reference_provenance(package, unit, child, proven, return_lenders)?;
    }
    Ok(())
}
fn record_implicit_reference_owners(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    proven: &BTreeMap<(usize, usize), ReferenceProvenance>,
    return_lenders: &BTreeMap<FunctionKey, usize>,
    owners: &mut BTreeSet<(usize, usize)>,
) {
    if node.kind == SyntaxKind::CallExpression
        && !node.children.first().is_some_and(|callee| {
            super::namespaces::function_contract_for_call_with_safety(
                package,
                unit,
                callee,
                crate::syntax::call_is_unsafe(node),
            )
            .is_some_and(|contract| contract.projected_provided)
        })
        && let [_, arguments] = node.children.as_slice()
        && let Some(contract) = super::calls::selected_callable_contract(
            package,
            unit,
            node,
            crate::syntax::call_is_unsafe(node),
        )
        && !contract.projected_provided
    {
        let mut named_seen = false;
        let mut positional = 0usize;
        for argument in &arguments.children {
            let named = argument
                .children
                .first()
                .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
            // The ordinary call validator reports invalid argument layouts.
            let parameter = resolve_call_parameter(
                unit,
                argument,
                named,
                &contract,
                &mut positional,
                &mut named_seen,
            )
            .ok();
            if parameter.is_some_and(|parameter| {
                matches!(
                    parameter.value_type,
                    Some(ValueType::Reference(_) | ValueType::MutableReference(_))
                )
            }) {
                let value = argument.children.last().unwrap_or(argument);
                if let Some(origin) = borrow_origin(package, unit, value, proven, return_lenders) {
                    owners.insert((origin.owner.start, origin.owner.end));
                }
            }
        }
    } else if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && function_contract_for_call_with_safety(
            package,
            unit,
            callee,
            crate::syntax::call_is_unsafe(node),
        )
        .is_none()
        && let Ok(Some(
            ValueType::Function(parameters, _, _) | ValueType::AsyncFunction(parameters, _, _, _),
        )) = infer_value_type(unit, callee, &unit.typed_bindings)
    {
        for (index, argument) in arguments.children.iter().enumerate() {
            let parameter = parameters.get(index).or_else(|| {
                parameters
                    .last()
                    .filter(|parameter| parameter.is_variadic())
            });
            let value = argument.children.last().unwrap_or(argument);
            if parameter.is_some_and(|parameter| {
                matches!(
                    parameter.value_type_ref(),
                    ValueType::Reference(_) | ValueType::MutableReference(_)
                )
            }) && let Some(origin) = borrow_origin(package, unit, value, proven, return_lenders)
            {
                owners.insert((origin.owner.start, origin.owner.end));
            }
        }
    }
    for child in &node.children {
        record_implicit_reference_owners(package, unit, child, proven, return_lenders, owners);
    }
}

pub(super) fn analyze_reference_provenance(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    let return_lenders = infer_reference_return_lenders(package);
    for unit in &mut package.units {
        unit.reference_return_lenders = return_lenders.clone();
    }
    for index in 0..package.units.len() {
        let proven = {
            let unit = &package.units[index];
            let mut proven = unit
                .typed_bindings
                .iter()
                .filter(|binding| {
                    matches!(
                        binding.value_type,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::SharedReference(_)
                    )
                })
                .filter(|binding| span_is_parameter(&unit.tree.root, binding.span))
                .map(|binding| {
                    (
                        (binding.span.start, binding.span.end),
                        ReferenceProvenance {
                            authority: if matches!(
                                binding.value_type,
                                ValueType::MutableReference(_)
                            ) {
                                ReferenceAuthority::Exclusive
                            } else {
                                ReferenceAuthority::ReadOnly
                            },
                            owner: binding.span,
                            lender_parameter: Some(binding.span),
                            external_lender: true,
                            parent_observer: None,
                            lifetime_end: None,
                            path: Vec::new(),
                        },
                    )
                })
                .collect();
            record_reference_provenance(
                package,
                unit,
                &unit.tree.root,
                &mut proven,
                &return_lenders,
            )?;
            let lifetime_ends = proven
                .iter()
                .map(|(&key, provenance)| {
                    let retained_shared = unit.typed_bindings.iter().any(|binding| {
                        (binding.span.start, binding.span.end) == key
                            && matches!(binding.value_type, ValueType::SharedReference(_))
                    });
                    (
                        key,
                        (!retained_shared)
                            .then(|| {
                                first_owner_lifetime_end(
                                    package,
                                    unit,
                                    &unit.tree.root,
                                    provenance.owner,
                                    key.0,
                                    &proven,
                                    !has_exclusive_lineage(provenance, &proven),
                                )
                            })
                            .flatten(),
                    )
                })
                .collect::<Vec<_>>();
            for (key, lifetime_end) in lifetime_ends {
                if let Some(provenance) = proven.get_mut(&key) {
                    provenance.lifetime_end = lifetime_end;
                }
            }
            let mut owners = BTreeSet::new();
            record_implicit_reference_owners(
                package,
                unit,
                &unit.tree.root,
                &proven,
                &return_lenders,
                &mut owners,
            );
            (proven, owners)
        };
        package.units[index].reference_provenance = proven.0;
        package.units[index].reference_owner_spans = proven.1;
        package.units[index].callable_drops_after.clear();
    }
    Ok(())
}

fn callable_drop_facts(unit: &SemanticUnit) -> BTreeMap<(u32, usize, usize), Vec<Span>> {
    let mut drops: BTreeMap<(u32, usize, usize), Vec<Span>> = BTreeMap::new();
    for carrier in unit.typed_bindings.iter().filter(|binding| {
        matches!(
            &binding.value_type,
            ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects)
                if effects.borrowed_environment && effects.modes.written != InvocationMode::Consuming
        )
    }) {
        let last_use = last_callable_carrier_use(
            unit, &unit.tree.root, carrier.span,
            &mut BTreeSet::from([carrier.span]),
        ).unwrap_or(carrier.span);
        let final_carrier = unit.flow_binding_ids
            .get(&(last_use.file, last_use.start, last_use.end))
            .copied().unwrap_or(carrier.span);
        let Some(boundary) = callable_cleanup_boundary(unit, last_use, final_carrier) else {
            continue;
        };
        if !super::scope_validation::statement_may_fall_through(boundary) {
            continue;
        }
        let boundary = boundary.span;
        let carriers = drops.entry((boundary.file, boundary.start, boundary.end)).or_default();
        if !carriers.contains(&final_carrier) {
            carriers.push(final_carrier);
        }
    }
    drops
}

pub(super) fn record_callable_drop_facts(package: &mut SemanticPackage) {
    for index in 0..package.units.len() {
        let drops = {
            let unit = &package.units[index];
            callable_drop_facts(unit)
        };
        package.units[index].callable_drops_after = drops;
    }
}

fn callable_cleanup_boundary(
    unit: &SemanticUnit,
    use_span: Span,
    carrier: Span,
) -> Option<&SyntaxNode> {
    fn enclosing_controls<'a>(
        node: &'a SyntaxNode,
        use_span: Span,
        controls: &mut Vec<&'a SyntaxNode>,
    ) {
        if node.span.start > use_span.start || node.span.end < use_span.end {
            return;
        }
        if matches!(
            node.kind,
            SyntaxKind::IfStatement
                | SyntaxKind::WhileStatement
                | SyntaxKind::ForStatement
                | SyntaxKind::TryStatement
                | SyntaxKind::SelectStatement
                | SyntaxKind::SwitchStatement
                | SyntaxKind::MatchStatement
        ) {
            controls.push(node);
        }
        for child in &node.children {
            enclosing_controls(child, use_span, controls);
        }
    }
    fn containing_statement(node: &SyntaxNode, use_span: Span) -> Option<&SyntaxNode> {
        let child = node
            .children
            .iter()
            .find(|child| child.span.start <= use_span.start && child.span.end >= use_span.end)?;
        containing_statement(child, use_span)
            .or_else(|| (node.kind == SyntaxKind::Block).then_some(child))
    }

    let mut controls = Vec::new();
    enclosing_controls(&unit.tree.root, use_span, &mut controls);
    controls
        .into_iter()
        .filter(|node| carrier.start < node.span.start)
        .max_by_key(|node| node.span.end - node.span.start)
        .map_or_else(|| containing_statement(&unit.tree.root, use_span), Some)
}

fn first_use_after(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    declaration: Span,
    position: usize,
) -> Option<Span> {
    if node.kind == SyntaxKind::Name
        && node.span.start >= position
        && package
            .resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
            .and_then(|symbol| symbol.declaration_span)
            == Some(declaration)
    {
        return Some(node.span);
    }
    node.children
        .iter()
        .find_map(|child| first_use_after(package, unit, child, declaration, position))
}

fn last_use_of_binding(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    declaration: Span,
    position: usize,
) -> Option<Span> {
    if node.kind == SyntaxKind::AnonymousFunction && declaration.start < node.span.start {
        return None;
    }
    if node.kind == SyntaxKind::Name
        && node.span.start >= position
        && package
            .resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
            .and_then(|symbol| symbol.declaration_span)
            == Some(declaration)
    {
        return Some(node.span);
    }
    node.children
        .iter()
        .filter_map(|child| last_use_of_binding(package, unit, child, declaration, position))
        .max_by_key(|span| span.start)
}

fn last_callable_carrier_use(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    carrier: Span,
    visited: &mut BTreeSet<Span>,
) -> Option<Span> {
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && let Some(source) = node
            .children
            .last()
            .filter(|source| source.kind == SyntaxKind::Name)
        && binding_at(unit, source).map(|binding| binding.span) == Some(carrier)
        && let Some(target) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
        && let Some(target_span) = super::bindings::declared_bindings_at_node(unit, node)
            .map(|binding| binding.span)
            .next()
            .or_else(|| binding_at(unit, target).map(|binding| binding.span))
    {
        return if visited.insert(target_span) {
            last_callable_carrier_use(unit, &unit.tree.root, target_span, visited)
                .or(Some(target.span))
        } else {
            Some(target.span)
        };
    }
    if node.kind == SyntaxKind::Name
        && node.span.start >= carrier.end
        && binding_at(unit, node).map(|binding| binding.span) == Some(carrier)
    {
        return Some(node.span);
    }
    node.children
        .iter()
        .filter_map(|child| last_callable_carrier_use(unit, child, carrier, visited))
        .max_by_key(|span| span.start)
}

fn closure_captures_binding(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    observer: &TypedBinding,
) -> bool {
    if node.kind == SyntaxKind::AnonymousFunction
        && let Some(contract) = super::bindings::callback_contract(package, unit, node)
        && contract.captures.iter().any(|capture| {
            package
                .resolve_name_at(unit, node.span.start, capture)
                .and_then(|symbol| symbol.declaration_span)
                == Some(observer.span)
        })
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| closure_captures_binding(package, unit, child, observer))
}

fn last_closure_carrier_use(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    observer: &TypedBinding,
) -> Option<Span> {
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && closure_captures_binding(package, unit, node, observer)
        && let Some(carrier) = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == node.span)
    {
        return last_callable_carrier_use(
            unit,
            &unit.tree.root,
            carrier.span,
            &mut BTreeSet::from([carrier.span]),
        )
        .or(Some(node.span));
    }
    node.children
        .iter()
        .filter_map(|child| last_closure_carrier_use(package, unit, child, observer))
        .max_by_key(|span| span.start)
}

fn later_span(left: Option<Span>, right: Option<Span>) -> Option<Span> {
    match (left, right) {
        (Some(left), Some(right)) => Some(if left.start >= right.start {
            left
        } else {
            right
        }),
        (Some(span), None) | (None, Some(span)) => Some(span),
        (None, None) => None,
    }
}

fn last_reference_dependent_use(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    observer: &TypedBinding,
    provenances: &BTreeMap<(usize, usize), ReferenceProvenance>,
    visited: &mut BTreeSet<(usize, usize)>,
) -> Option<Span> {
    let key = (observer.span.start, observer.span.end);
    if !visited.insert(key) {
        return None;
    }
    let mut last = last_use_of_binding(
        package,
        unit,
        &unit.tree.root,
        observer.span,
        observer.span.end,
    );
    for (&(start, end), provenance) in provenances {
        if provenance.parent_observer != Some(observer.span) {
            continue;
        }
        if let Some(child) = unit
            .typed_bindings
            .iter()
            .find(|binding| (binding.span.start, binding.span.end) == (start, end))
        {
            last = later_span(
                last,
                last_reference_dependent_use(package, unit, child, provenances, visited),
            );
        }
    }
    let callable_use = last_closure_carrier_use(package, unit, &unit.tree.root, observer);
    let last_use = later_span(last, callable_use)?;
    if callable_use == Some(last_use)
        && let Some(&(file, _, end)) = unit
            .callable_drops_after
            .keys()
            .filter(|&&(file, start, end)| {
                file == last_use.file && start <= last_use.start && last_use.end <= end
            })
            .max_by_key(|&&(_, start, end)| end - start)
    {
        return Some(Span {
            file,
            start: last_use.start,
            end,
        });
    }
    Some(last_use)
}

fn live_readonly_observer_conflict(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    observer: &TypedBinding,
    provenance: &ReferenceProvenance,
) -> Option<SemanticFailure> {
    if provenance.authority != ReferenceAuthority::ReadOnly
        || matches!(observer.value_type, ValueType::SharedReference(_))
    {
        return None;
    }
    let last_use = last_reference_dependent_use(
        package,
        unit,
        observer,
        &unit.reference_provenance,
        &mut BTreeSet::new(),
    )?;
    let exclusive = unit.typed_bindings.iter().find(|candidate| {
        candidate.span.start >= observer.span.end
            && candidate.span.start <= last_use.end
            && !reference_descends_from(candidate.span, observer.span, &unit.reference_provenance)
            && last_reference_dependent_use(
                package,
                unit,
                candidate,
                &unit.reference_provenance,
                &mut BTreeSet::new(),
            )
            .is_some()
            && unit
                .reference_provenance
                .get(&(candidate.span.start, candidate.span.end))
                .is_some_and(|loan| {
                    loan.authority == ReferenceAuthority::Exclusive
                        && loan.owner == provenance.owner
                        && owner_paths_overlap(&loan.path, &provenance.path)
                })
    })?;
    let owner_name = unit
        .typed_bindings
        .iter()
        .find(|binding| binding.span == provenance.owner)
        .map_or("<unknown>", |binding| binding.name.as_str());
    Some(failure(
        &unit.source,
        "T0149",
        format!("cannot access owner `{owner_name}` while an exclusive mutable reference is live"),
        exclusive.span,
    ))
}

fn expired_reference_use_failure(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    observer: &TypedBinding,
    provenance: &ReferenceProvenance,
) -> Option<SemanticFailure> {
    let lifetime_end = provenance.lifetime_end?;
    first_use_after(
        package,
        unit,
        &unit.tree.root,
        observer.span,
        lifetime_end.end,
    )?;
    let owner_name = unit
        .typed_bindings
        .iter()
        .find(|binding| binding.span == provenance.owner)
        .map_or("<unknown>", |binding| binding.name.as_str());
    let path = provenance
        .path
        .iter()
        .fold(String::new(), |mut path, projection| {
            match projection {
                ReferenceProjection::Field(member) => {
                    path.push('.');
                    path.push_str(member);
                }
                ReferenceProjection::Element => path.push_str("[element]"),
                ReferenceProjection::CallResult => path.push_str(" through call result"),
            }
            path
        });
    Some(failure(
        &unit.source,
        "T0059",
        format!(
            "reference to owner `{owner_name}{path}` remains in use after this mutation, move, or replacement"
        ),
        lifetime_end,
    ))
}
pub(super) fn validate_referenced_replacements(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        for (&observer_key, provenance) in &unit.reference_provenance {
            let Some(observer) = unit.typed_bindings.iter().find(|binding| {
                (binding.span.start, binding.span.end) == observer_key
                    && matches!(
                        binding.value_type,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::SharedReference(_)
                    )
            }) else {
                continue;
            };
            if let Some(conflict) =
                live_readonly_observer_conflict(package, unit, observer, provenance)
            {
                return Err(conflict);
            }
            if !matches!(observer.value_type, ValueType::SharedReference(_))
                && has_exclusive_lineage(provenance, &unit.reference_provenance)
                && provenance.owner != observer.span
                && let Some(last_use) = last_reference_dependent_use(
                    package,
                    unit,
                    observer,
                    &unit.reference_provenance,
                    &mut BTreeSet::new(),
                )
                && let Some(conflict) = first_owner_access_between(
                    package,
                    unit,
                    &unit.tree.root,
                    OwnerAccessWindow {
                        owner: provenance.owner,
                        borrowed_path: &provenance.path,
                        observer: observer.span,
                        provenances: &unit.reference_provenance,
                        writes_only: provenance.authority == ReferenceAuthority::ReadOnly,
                        start: observer.span.end,
                        end: last_use.end,
                    },
                )
            {
                let owner_name = unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == provenance.owner)
                    .map_or("<unknown>", |binding| binding.name.as_str());
                return Err(failure(
                    &unit.source,
                    "T0149",
                    format!(
                        "cannot access owner `{owner_name}` while an exclusive mutable reference is live"
                    ),
                    conflict,
                ));
            }
            if let Some(failure) =
                expired_reference_use_failure(package, unit, observer, provenance)
            {
                return Err(failure);
            }
        }
    }
    Ok(())
}

fn shared_object_targets(
    value_type: &ValueType,
    inside_shared: bool,
    targets: &mut BTreeSet<ObjectIdentity>,
) {
    match value_type {
        ValueType::SharedReference(item) => {
            shared_object_targets(&item.value_type(), true, targets);
        }
        ValueType::Object(identity) if inside_shared => {
            targets.insert(identity.clone());
        }
        ValueType::Optional(item) => shared_object_targets(item, inside_shared, targets),
        ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::AsyncIterationStep(item)
        | ValueType::ChannelPair(item)
        | ValueType::ChannelSender(item)
        | ValueType::ChannelReceiver(item)
        | ValueType::ChannelSendOutcome(item)
        | ValueType::ChannelReceiveOutcome(item)
        | ValueType::DocumentDecodeOutcome(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::UnorderedSet(item)
        | ValueType::Task(item, _)
        | ValueType::ScopedTask(item, _)
        | ValueType::TaskOutcome(item)
        | ValueType::TaskGroup(item)
        | ValueType::Reference(item) => {
            shared_object_targets(&item.value_type(), inside_shared, targets);
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            shared_object_targets(&key.value_type(), inside_shared, targets);
            shared_object_targets(&value.value_type(), inside_shared, targets);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                shared_object_targets(&parameter.value_type(), inside_shared, targets);
            }
            shared_object_targets(&result.value_type(), inside_shared, targets);
        }
        _ => {}
    }
}

fn shared_cycle_span(
    current: &ObjectIdentity,
    edges: &BTreeMap<ObjectIdentity, Vec<(ObjectIdentity, Span)>>,
    visiting: &mut BTreeSet<ObjectIdentity>,
    visited: &mut BTreeSet<ObjectIdentity>,
) -> Option<Span> {
    if !visiting.insert(current.clone()) {
        return None;
    }
    for (target, span) in edges.get(current).into_iter().flatten() {
        if visiting.contains(target) {
            return Some(*span);
        }
        if !visited.contains(target)
            && let Some(span) = shared_cycle_span(target, edges, visiting, visited)
        {
            return Some(span);
        }
    }
    visiting.remove(current);
    visited.insert(current.clone());
    None
}

pub(super) fn validate_shared_ownership_cycles(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    let mut edges = BTreeMap::<ObjectIdentity, Vec<(ObjectIdentity, Span)>>::new();
    for unit in &package.units {
        for descriptor in &unit.descriptors {
            if descriptor.kind != ObjectKind::Class {
                continue;
            }
            for field in &descriptor.fields {
                if field.is_static {
                    continue;
                }
                let mut targets = BTreeSet::new();
                shared_object_targets(&field.value_type, false, &mut targets);
                edges
                    .entry(descriptor.identity.clone())
                    .or_default()
                    .extend(targets.into_iter().map(|target| (target, field.span)));
            }
        }
    }
    let mut visited = BTreeSet::new();
    for identity in edges.keys() {
        if !visited.contains(identity)
            && let Some(span) =
                shared_cycle_span(identity, &edges, &mut BTreeSet::new(), &mut visited)
        {
            let source = package
                .units
                .iter()
                .find(|unit| unit.source.id() == span.file)
                .expect("shared ownership field belongs to a semantic unit");
            return Err(failure(
                &source.source,
                "T0069",
                "native target rejects descriptor fields whose `shared ref` ownership graph can form a strong cycle; use ordinary `ref` for a non-owning back-edge",
                span,
            ));
        }
    }
    Ok(())
}

fn validate_named_function_capture(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    function: Option<Span>,
    symbol: Option<&Symbol>,
) -> Result<(), SemanticFailure> {
    if let (Some(function), Some(symbol)) = (function, symbol) {
        validate_named_function_capture_symbol(unit, node, function, symbol)?;
    }
    Ok(())
}

fn validate_named_function_capture_symbol(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    function: Span,
    symbol: &Symbol,
) -> Result<(), SemanticFailure> {
    if symbol.kind != SymbolKind::Binding || symbol.global {
        return Ok(());
    }
    let Some(origin) = symbol.declaration_span else {
        return Ok(());
    };
    if origin.file != function.file
        || (function.start <= origin.start && origin.end <= function.end)
        || !unit
            .scopes
            .iter()
            .any(|scope| scope.span.start <= origin.start && origin.end <= scope.span.end)
    {
        return Ok(());
    }
    Err(failure(
        &unit.source,
        "S2062",
        format!(
            "named local function cannot capture enclosing binding `{}`; use an anonymous closure",
            symbol.name
        ),
        node.span,
    ))
}

fn validate_named_function_rust_capture(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    function: Span,
) -> Result<(), SemanticFailure> {
    let identifiers = crate::rust_ir::rust_free_value_identifiers(node_text(&unit.source, node));
    for (_, binding) in identifiers.iter().filter_map(|identifier| {
        crate::lowering::rust_value_binding_at(unit, node.span.start, identifier)
    }) {
        if let Some(symbol) = unit
            .scopes
            .iter()
            .flat_map(|scope| scope.symbols.values().flatten())
            .find(|symbol| {
                symbol.kind == SymbolKind::Binding && symbol.declaration_span == Some(binding.span)
            })
        {
            validate_named_function_capture_symbol(unit, node, function, symbol)?;
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "reference validation keeps ownership forms in one ordered syntax traversal"
)]
pub(super) fn validate_references(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        named_function: Option<Span>,
    ) -> Result<(), SemanticFailure> {
        let named_function = if node.kind == SyntaxKind::FunctionDeclaration {
            unit.scopes
                .iter()
                .find(|scope| scope.span == node.span && scope.parent.is_some())
                .map(|_| node.span)
        } else {
            named_function
        };
        match node.kind {
            SyntaxKind::RustBlock | SyntaxKind::UnsafeRustBlock => {
                if let Some(function) = named_function {
                    validate_named_function_rust_capture(unit, node, function)?;
                }
            }
            SyntaxKind::Name => {
                let name = node_text(&unit.source, node);
                let resolved = package.resolve_name_at(unit, node.span.start, name);
                validate_named_function_capture(unit, node, named_function, resolved)?;
                let unsafe_name = format!("unsafe::{name}");
                let unsafe_resolved = package.resolve_name_at(unit, node.span.start, &unsafe_name);
                let implicit_receiver = is_implicit_object_receiver(unit, node.span.start, name);
                if resolved.is_none()
                    && unsafe_resolved.is_none()
                    && !package.descriptor_constructs.contains_key(name)
                    && !implicit_receiver
                {
                    if namespace_chain(&unit.namespace)
                        .filter_map(|path| package.namespaces.get(&path))
                        .filter_map(|namespace| namespace.symbols.get(name))
                        .chain(package.globals.get(name))
                        .any(|symbol| {
                            symbol.kind == SymbolKind::Binding
                                && !symbol.available_in_function_body()
                        })
                    {
                        return Err(namespace_variable_reference_failure(
                            &unit.source,
                            name,
                            node.span,
                        ));
                    }
                    return Err(failure(
                        &unit.source,
                        "S2013",
                        format!("unresolved name `{name}`"),
                        node.span,
                    ));
                }
            }
            SyntaxKind::NamespaceDeclaration
            | SyntaxKind::ImportDeclaration
            | SyntaxKind::ParameterList
            | SyntaxKind::Parameter
            | SyntaxKind::TypeParameterList
            | SyntaxKind::TypeParameter
            | SyntaxKind::AssociatedTypeDeclaration
            | SyntaxKind::ForTarget
            | SyntaxKind::TypeExpression
            | SyntaxKind::UnionType
            | SyntaxKind::PrefixType
            | SyntaxKind::AppliedType
            | SyntaxKind::FunctionType => {}
            SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::AnonymousFunction
            | SyntaxKind::FunctionDeclaration
            | SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration
            | SyntaxKind::EnumDeclaration
            | SyntaxKind::EnumVariant => {
                let mut declaration_name_skipped = false;
                for child in &node.children {
                    if !declaration_name_skipped && child.kind == SyntaxKind::Name {
                        declaration_name_skipped = true;
                        if node.kind == SyntaxKind::Assignment
                            && !node
                                .children
                                .iter()
                                .any(|part| part.kind == SyntaxKind::TypeExpression)
                        {
                            validate_named_function_capture(
                                unit,
                                child,
                                named_function,
                                package.resolve_name_at(
                                    unit,
                                    child.span.start,
                                    node_text(&unit.source, child),
                                ),
                            )?;
                        }
                        continue;
                    }
                    visit(package, unit, child, named_function)?;
                }
            }
            SyntaxKind::CatchClause => {
                if let Some(descriptor) = node.children.first() {
                    visit(package, unit, descriptor, named_function)?;
                }
                if let Some(block) = node.children.last()
                    && block.kind == SyntaxKind::Block
                {
                    visit(package, unit, block, named_function)?;
                }
            }
            SyntaxKind::Argument => {
                for (index, child) in node.children.iter().enumerate() {
                    if index == 0 && node.children.len() > 1 && child.kind == SyntaxKind::Name {
                        continue;
                    }
                    visit(package, unit, child, named_function)?;
                }
            }
            SyntaxKind::MemberExpression
            | SyntaxKind::StaticMemberExpression
            | SyntaxKind::ConstructionExpression => {
                if let Some(receiver) = node.children.first()
                    && !(node.kind == SyntaxKind::StaticMemberExpression
                        && node_text(&unit.source, receiver) == "self")
                {
                    visit(package, unit, receiver, named_function)?;
                }
            }
            _ => {
                for child in &node.children {
                    visit(package, unit, child, named_function)?;
                }
            }
        }
        Ok(())
    }

    for unit in &package.units {
        for node in &unit.tree.root.children {
            visit(package, unit, node, None)?;
        }
    }
    Ok(())
}
fn capture_is_mutably_lent(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    closure: &SyntaxNode,
    capture_span: Span,
) -> bool {
    binding_span_is_mutated_in(
        package,
        unit,
        closure,
        capture_span,
        true,
        ClosureWrites::Include,
    )
}

fn captured_native_value_move(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    closure: &SyntaxNode,
    binding: &TypedBinding,
) -> Option<Span> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        closure: Span,
        binding: &TypedBinding,
    ) -> Option<Span> {
        if is_function_node(node) && node.span != closure {
            return None;
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && let Some(projected) = super::bindings::projected_function_for_call(
                package,
                unit,
                callee,
                crate::syntax::call_is_unsafe(node),
            )
            && let Some(contract) = function_contract_for_call_with_safety(
                package,
                unit,
                callee,
                crate::syntax::call_is_unsafe(node),
            )
        {
            let selected = unit.projected_call_specializations.get(&(
                node.span.file,
                node.span.start,
                node.span.end,
            ));
            for (index, parameter) in contract.parameters.iter().enumerate() {
                let native_parameter = selected
                    .and_then(|selected| selected.projected_parameters.get(index))
                    .or_else(|| projected.parameters.get(index));
                if native_parameter.is_some_and(
                    crate::rust_interop::projection::ProjectedParameter::borrows_argument,
                ) || parameter.element_value_type().is_some_and(|ty| {
                    matches!(
                        ty,
                        ValueType::Reference(_)
                            | ValueType::MutableReference(_)
                            | ValueType::SharedReference(_)
                    )
                }) {
                    continue;
                }
                let Some(mut value) = argument_for_parameter(unit, arguments, contract, index)
                else {
                    continue;
                };
                while matches!(
                    value.kind,
                    SyntaxKind::GroupExpression | SyntaxKind::UnaryExpression
                ) {
                    value = value.children.last()?;
                }
                if value.kind == SyntaxKind::Name
                    && package
                        .resolve_name_at(unit, value.span.start, node_text(&unit.source, value))
                        .is_some_and(|symbol| symbol.declaration_span == Some(binding.span))
                {
                    return Some(value.span);
                }
            }
        }
        node.children
            .iter()
            .find_map(|child| visit(package, unit, child, closure, binding))
    }
    visit(package, unit, closure, closure.span, binding)
}

pub(super) fn validate_native_captures(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        for contract in &unit.functions {
            if contract.captures.is_empty()
                || contract.written_invocation_mode == InvocationMode::Consuming
            {
                continue;
            }
            let Some(closure) = find_node_by_span(&unit.tree.root, contract.span) else {
                continue;
            };
            for capture in &contract.captures {
                let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
                    binding.name == *capture
                        && binding.is_visible_at(unit.source.id(), contract.span.start)
                }) else {
                    continue;
                };
                let mutably_lent = capture_is_mutably_lent(package, unit, closure, binding.span);
                if matches!(binding.value_type, ValueType::MutableReference(_))
                    && mutably_lent
                    && contract.written_invocation_mode == InvocationMode::Shared
                {
                    return Err(failure(
                        &unit.source,
                        "T0144",
                        format!(
                            "callable `{}` mutates captured exclusive reference `{capture}` and must be declared mutable",
                            contract.name
                        ),
                        closure.span,
                    ));
                }
                if super::objects::application_is_resource_owning(package, &binding.value_type)
                    && let Some(span) = captured_native_value_move(package, unit, closure, binding)
                {
                    return Err(SemanticFailure {
                        source: unit.source.clone(),
                        diagnostics: vec![Diagnostic::error(
                            "T0144",
                            format!("repeatable callable `{}` cannot move captured owner `{capture}` into a native by-value parameter", contract.name),
                            span,
                        ).with_help(
                            "bind `observer = ref value` outside the callable and pass `observer` when borrowing is intended; otherwise use an explicit consuming callable when ownership permits",
                        )],
                    });
                }
                if !value_type_contains_nonclone_foreign(unit, &binding.value_type) {
                    continue;
                }
                let mutable_environment =
                    contract.written_invocation_mode == InvocationMode::Mutable;
                if !mutably_lent && !contract.is_async && !mutable_environment {
                    continue;
                }
                let operation = if mutably_lent {
                    "mutably lend captured native binding"
                } else if contract.is_async {
                    "snapshot non-Clone native binding for repeatable async invocation"
                } else {
                    "separate a mutable environment containing non-Clone native binding"
                };
                return Err(failure(
                    &unit.source,
                    "T0144",
                    format!(
                        "callable `{}` cannot {operation} `{capture}`; pass it as a reference parameter or use an explicit consuming callable when ownership permits",
                        contract.name
                    ),
                    contract.span,
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn namespace_variable_reference_failure(
    source: &SourceFile,
    name: &str,
    span: Span,
) -> SemanticFailure {
    SemanticFailure {
        source: source.clone(),
        diagnostics: vec![
            Diagnostic::error(
                "S2026",
                format!("namespace variable `{name}` cannot cross a function boundary"),
                span,
            )
            .with_help(format!(
                "pass `{name}` as a parameter or return it from a function"
            )),
        ],
    }
}

pub(super) fn namespace_variable_import_failure(
    source: &SourceFile,
    name: &str,
    span: Span,
) -> SemanticFailure {
    SemanticFailure {
        source: source.clone(),
        diagnostics: vec![
            Diagnostic::error(
                "S2026",
                format!("namespace variable `{name}` cannot be imported outside its namespace"),
                span,
            )
            .with_help(format!(
                "import a function that reads `{name}` and returns its value instead"
            )),
        ],
    }
}

pub(super) fn binding_initializer(node: &SyntaxNode) -> Option<&SyntaxNode> {
    let name_index = node
        .children
        .iter()
        .position(|child| child.kind == SyntaxKind::Name)?;
    node.children
        .iter()
        .enumerate()
        .rev()
        .find(|(index, child)| {
            *index != name_index
                && !matches!(
                    child.kind,
                    SyntaxKind::TypeExpression
                        | SyntaxKind::Visibility
                        | SyntaxKind::DeclarationQualifier
                        | SyntaxKind::FieldMetadata
                )
        })
        .map(|(_, child)| child)
}

#[expect(
    clippy::too_many_lines,
    reason = "the dependency graph construction and its diagnostics are one ordered validation pass"
)]
pub(super) fn validate_initializer_dependencies(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    type Key = (u32, usize, usize);

    fn key(span: Span) -> Key {
        (span.file, span.start, span.end)
    }

    fn collect_reads(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        reads: &mut Vec<(Key, Span)>,
        functions: &mut BTreeSet<Key>,
    ) {
        if node.kind == SyntaxKind::Argument && node.children.len() > 1 {
            if let Some(value) = node.children.last() {
                collect_reads(package, unit, value, reads, functions);
            }
            return;
        }
        if node.kind == SyntaxKind::Name {
            if let Some(symbol) =
                package.resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
                && let Some(span) = symbol.declaration_span
            {
                if symbol.kind == SymbolKind::Binding {
                    reads.push((key(span), node.span));
                } else if symbol.kind == SymbolKind::Function && functions.insert(key(span)) {
                    for owner in &package.units {
                        if let Some(function) = find_node_by_span(&owner.tree.root, span) {
                            collect_reads(package, owner, function, reads, functions);
                            break;
                        }
                    }
                }
            }
            return;
        }
        if matches!(
            node.kind,
            SyntaxKind::NamespaceDeclaration
                | SyntaxKind::ImportDeclaration
                | SyntaxKind::Parameter
                | SyntaxKind::ForTarget
                | SyntaxKind::TypeExpression
                | SyntaxKind::UnionType
                | SyntaxKind::PrefixType
                | SyntaxKind::AppliedType
                | SyntaxKind::FunctionType
        ) {
            return;
        }
        if matches!(
            node.kind,
            SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
        ) {
            if let Some(receiver) = node.children.first() {
                collect_reads(package, unit, receiver, reads, functions);
            }
            return;
        }
        for child in &node.children {
            collect_reads(package, unit, child, reads, functions);
        }
    }
    fn unresolved_name_span(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        name: &str,
    ) -> Option<Span> {
        if node.kind == SyntaxKind::Argument && node.children.len() > 1 {
            return node
                .children
                .last()
                .and_then(|value| unresolved_name_span(package, unit, value, name));
        }
        if matches!(
            node.kind,
            SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
        ) {
            return node
                .children
                .first()
                .and_then(|receiver| unresolved_name_span(package, unit, receiver, name));
        }
        if node.kind == SyntaxKind::Name
            && node_text(&unit.source, node) == name
            && package
                .resolve_name_at(unit, node.span.start, name)
                .is_none()
        {
            return Some(node.span);
        }
        node.children
            .iter()
            .find_map(|child| unresolved_name_span(package, unit, child, name))
    }
    fn validate_self_references(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        // A `global name = value` write is represented as a binding without a type annotation.
        // Its self-read is a normal update, not an initializer dependency; untyped declaration
        // self-references are still rejected by the top-level dependency graph below.
        if node.kind == SyntaxKind::Binding
            && let Some(initializer) = binding_initializer(node)
            && let Some(declaration) = declaration_from_syntax(unit, node)
            && (!declaration.global
                || node
                    .children
                    .iter()
                    .any(|child| child.kind == SyntaxKind::TypeExpression))
        {
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            let direct_unresolved_self =
                unresolved_name_span(package, unit, initializer, &declaration.name);
            if let Some(span) = reads
                .iter()
                .find(|(dependency, _)| *dependency == key(node.span))
                .map(|(_, span)| *span)
                .or(direct_unresolved_self)
            {
                return Err(failure(
                    &unit.source,
                    "S2023",
                    format!(
                        "binding `{}` cannot reference itself in its initializer",
                        declaration.name
                    ),
                    span,
                ));
            }
        }
        for child in &node.children {
            validate_self_references(package, unit, child)?;
        }
        Ok(())
    }

    fn find_cycle(
        current: Key,
        edges: &BTreeMap<Key, Vec<(Key, Span)>>,
        path: &mut BTreeSet<Key>,
    ) -> Option<Span> {
        if !path.insert(current) {
            return None;
        }
        for &(dependency, span) in edges.get(&current).into_iter().flatten() {
            if path.contains(&dependency) {
                return Some(span);
            }
            if let Some(span) = find_cycle(dependency, edges, path) {
                return Some(span);
            }
        }
        path.remove(&current);
        None
    }
    for unit in &package.units {
        validate_self_references(package, unit, &unit.tree.root)?;
    }

    let mut edges = BTreeMap::<Key, Vec<(Key, Span)>>::new();
    for unit in &package.units {
        for node in &unit.tree.root.children {
            if node.kind != SyntaxKind::Binding {
                continue;
            }
            let Some(declaration) = declaration_from_syntax(unit, node) else {
                continue;
            };
            let Some(initializer) = binding_initializer(node) else {
                continue;
            };
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            if reads
                .iter()
                .any(|(dependency, _)| *dependency == key(node.span))
            {
                let span = reads
                    .iter()
                    .find(|(dependency, _)| *dependency == key(node.span))
                    .expect("checked self-reference")
                    .1;
                return Err(failure(
                    &unit.source,
                    "S2023",
                    format!(
                        "binding `{}` cannot reference itself in its initializer",
                        declaration.name
                    ),
                    span,
                ));
            }
            edges.entry(key(node.span)).or_default().extend(reads);
        }
    }
    for unit in &package.units {
        for node in &unit.tree.root.children {
            if node.kind != SyntaxKind::Assignment {
                continue;
            }
            let Some(declaration) = declaration_from_syntax(unit, node) else {
                continue;
            };
            let name = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .expect("ordinary assignment has a name");
            if package
                .globals
                .get(&declaration.name)
                .is_some_and(|global| global.namespace == unit.namespace)
                && !declaration.global
            {
                return Err(SemanticFailure {
                    source: unit.source.clone(),
                    diagnostics: vec![
                        Diagnostic::error(
                            "S2021",
                            format!(
                                "plain namespace assignment cannot replace program-global binding `{}`",
                                declaration.name
                            ),
                            name.span,
                        )
                        .with_help(
                            "pass changing values through parameters and returns instead",
                        ),
                    ],
                });
            }
            let Some(target) = package.resolve_name_at(unit, name.span.start, &declaration.name)
            else {
                continue;
            };
            let Some(initializer) = binding_initializer(node) else {
                continue;
            };
            let Some(owner) = target.declaration_span else {
                continue;
            };
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            reads.retain(|(dependency, _)| *dependency != key(owner));
            edges.entry(key(owner)).or_default().extend(reads);
        }
    }
    for &start in edges.keys() {
        if let Some(span) = find_cycle(start, &edges, &mut BTreeSet::new()) {
            let source = package
                .units
                .iter()
                .find(|unit| unit.source.id() == span.file)
                .expect("dependency span belongs to a semantic unit");
            return Err(failure(
                &source.source,
                "S2024",
                "namespace binding initialization has a dependency cycle",
                span,
            ));
        }
    }
    Ok(())
}

pub(super) fn find_node_by_span(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    (node.span == span).then_some(node).or_else(|| {
        node.children
            .iter()
            .find_map(|child| find_node_by_span(child, span))
    })
}

pub(super) fn collect_evaluation_steps(
    source: &SourceFile,
    root: &SyntaxNode,
) -> Vec<EvaluationStep> {
    fn visit(
        source: &SourceFile,
        node: &SyntaxNode,
        conditional: bool,
        steps: &mut Vec<EvaluationStep>,
    ) {
        if node.kind == SyntaxKind::BinaryExpression
            && let [left, right] = node.children.as_slice()
        {
            visit(source, left, conditional, steps);
            let operator = source.text()[left.span.end..right.span.start].trim();
            let short_circuit = matches!(operator, "and" | "or");
            if short_circuit {
                steps.push(EvaluationStep {
                    kind: EvaluationKind::ShortCircuitRhs,
                    span: right.span,
                    conditional: true,
                });
            }
            visit(source, right, conditional || short_circuit, steps);
        } else {
            for child in &node.children {
                visit(source, child, conditional, steps);
            }
        }
        let kind = match node.kind {
            SyntaxKind::CallExpression => Some(EvaluationKind::Call),
            SyntaxKind::PostfixExpression => Some(EvaluationKind::PostfixUpdate),
            _ => None,
        };
        if let Some(kind) = kind {
            steps.push(EvaluationStep {
                kind,
                span: node.span,
                conditional,
            });
        }
    }

    let mut steps = Vec::new();
    visit(source, root, false, &mut steps);
    steps
}
