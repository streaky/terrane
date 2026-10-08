use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Span,
    syntax::{SyntaxKind, SyntaxNode},
};

use super::{BindingEvent, projected_function_for_call, span_key};
use crate::semantics::{
    diagnostics::{failure, node_text, unary_operator_text},
    expressions::{call_base_contract, infer_value_type},
    generics::substitute_value_type,
    model::{
        FunctionContract, ObjectKind, SemanticFailure, SemanticPackage, SemanticUnit,
        TaskTransferability, TypedBinding, ValueType,
    },
    objects::effective_object_fields,
    ownership::find_node_by_span,
};

pub(super) fn collect_suspension_points(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    spans: &mut Vec<Span>,
) {
    if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("await")
    {
        spans.push(node.span);
    }
    for child in &node.children {
        collect_suspension_points(unit, child, spans);
    }
}

pub(super) fn moves_binding_between(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    owner_span: Span,
    after: usize,
    before: usize,
) -> bool {
    if node.span.start >= after
        && node.span.end <= before
        && node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("move")
        && node.children.last().is_some_and(|operand| {
            operand.kind == SyntaxKind::Name
                && package
                    .resolve_name_at(unit, operand.span.start, node_text(&unit.source, operand))
                    .and_then(|symbol| symbol.declaration_span)
                    == Some(owner_span)
        })
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| moves_binding_between(package, unit, child, owner_span, after, before))
}

pub(super) fn reference_has_stable_local_owner(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    contract: &FunctionContract,
    reference: &TypedBinding,
) -> bool {
    let Some(declaration) = find_node_by_span(&unit.tree.root, reference.span) else {
        return false;
    };
    let Some(initializer) = declaration.children.last() else {
        return false;
    };
    if initializer.kind != SyntaxKind::UnaryExpression
        || unary_operator_text(unit, initializer).as_deref() != Some("ref")
    {
        return false;
    }
    let Some(source) = initializer
        .children
        .last()
        .filter(|source| source.kind == SyntaxKind::Name)
    else {
        return false;
    };
    let Some(owner_span) = package
        .resolve_name_at(unit, source.span.start, node_text(&unit.source, source))
        .and_then(|symbol| symbol.declaration_span)
    else {
        return false;
    };
    if !unit.typed_bindings.iter().any(|owner| {
        owner.span == owner_span
            && owner.span.start >= contract.span.start
            && owner.span.end <= contract.span.end
    }) {
        return false;
    }
    !moves_binding_between(
        package,
        unit,
        &unit.tree.root,
        owner_span,
        reference.visible_from,
        contract.span.end,
    ) && package
        .binding_events
        .get(&span_key(owner_span))
        .is_none_or(|events| {
            !events.iter().any(|event| {
                matches!(
                    event,
                    BindingEvent::Write { span, .. } if span.start >= reference.visible_from
                )
            })
        })
}

#[derive(Clone, Copy)]
pub(in crate::semantics) enum AutoTraitObligation {
    Send,
    Sync,
}

#[expect(
    clippy::too_many_lines,
    reason = "Recursive auto-trait proof cases stay aligned with the complete semantic value shape"
)]
pub(in crate::semantics) fn value_type_satisfies_auto_trait(
    package: &SemanticPackage,
    value_type: &ValueType,
    obligation: AutoTraitObligation,
) -> bool {
    fn satisfies(
        package: &SemanticPackage,
        value_type: &ValueType,
        obligation: AutoTraitObligation,
        visiting: &mut BTreeSet<String>,
    ) -> bool {
        match value_type {
            ValueType::Reference(_) => false,
            ValueType::Object(identity) => {
                if let Some(capability) = package.native_capability(identity) {
                    return match obligation {
                        AutoTraitObligation::Send => capability.send,
                        AutoTraitObligation::Sync => capability.sync,
                    };
                }
                if let Some((send, sync)) = package
                    .projection
                    .foreign_auto_traits(&identity.namespace, &identity.name)
                {
                    return match obligation {
                        AutoTraitObligation::Send => send,
                        AutoTraitObligation::Sync => sync,
                    };
                }
                let key = format!("{}::{}", identity.namespace, identity.name);
                if !visiting.insert(key.clone()) {
                    return true;
                }
                let result = package
                    .units
                    .iter()
                    .flat_map(|unit| &unit.descriptors)
                    .find(|object| object.identity.base() == identity.base())
                    .is_some_and(|object| match object.kind {
                        ObjectKind::Interface => true,
                        ObjectKind::Class => {
                            let substitutions = object
                                .generic_parameters
                                .iter()
                                .zip(&identity.type_arguments)
                                .map(|(parameter, argument)| {
                                    (parameter.name.clone(), argument.clone())
                                })
                                .collect();
                            effective_object_fields(package, object)
                                .into_iter()
                                .all(|field| {
                                    field.is_static
                                        || satisfies(
                                            package,
                                            &substitute_value_type(
                                                &field.value_type,
                                                &substitutions,
                                            ),
                                            obligation,
                                            visiting,
                                        )
                                })
                        }
                        ObjectKind::Enum => package
                            .units
                            .iter()
                            .flat_map(|unit| &unit.source_enums)
                            .find(|enumeration| enumeration.identity.base() == identity.base())
                            .is_some_and(|enumeration| {
                                let substitutions = object
                                    .generic_parameters
                                    .iter()
                                    .zip(&identity.type_arguments)
                                    .map(|(parameter, argument)| {
                                        (parameter.name.clone(), argument.clone())
                                    })
                                    .collect();
                                enumeration.variants.iter().all(|variant| {
                                    variant.payload.iter().all(|field| {
                                        satisfies(
                                            package,
                                            &substitute_value_type(
                                                &field.value_type,
                                                &substitutions,
                                            ),
                                            obligation,
                                            visiting,
                                        )
                                    })
                                })
                            }),
                        ObjectKind::Trait | ObjectKind::Type => false,
                    });
                visiting.remove(&key);
                result
            }
            ValueType::Optional(inner) => satisfies(package, inner, obligation, visiting),
            ValueType::Iterator(inner)
            | ValueType::IterationStep(inner)
            | ValueType::List(inner)
            | ValueType::Set(inner)
            | ValueType::Tuple(inner, _)
            | ValueType::UnorderedSet(inner)
            | ValueType::TaskOutcome(inner) => {
                satisfies(package, inner.value_type_ref(), obligation, visiting)
            }
            ValueType::SharedReference(inner) => match obligation {
                AutoTraitObligation::Send => {
                    satisfies(
                        package,
                        inner.value_type_ref(),
                        AutoTraitObligation::Send,
                        visiting,
                    ) && satisfies(
                        package,
                        inner.value_type_ref(),
                        AutoTraitObligation::Sync,
                        visiting,
                    )
                }
                AutoTraitObligation::Sync => {
                    satisfies(package, inner.value_type_ref(), obligation, visiting)
                }
            },
            ValueType::Map(key, value)
            | ValueType::Entry(key, value)
            | ValueType::UnorderedMap(key, value) => {
                satisfies(package, key.value_type_ref(), obligation, visiting)
                    && satisfies(package, value.value_type_ref(), obligation, visiting)
            }
            ValueType::AsyncFunction(_, _, transferability, _)
            | ValueType::Task(_, transferability)
            | ValueType::ScopedTask(_, transferability) => match obligation {
                AutoTraitObligation::Send => *transferability != TaskTransferability::Local,
                AutoTraitObligation::Sync => false,
            },
            _ => true,
        }
    }

    satisfies(package, value_type, obligation, &mut BTreeSet::new())
}

pub(super) fn value_type_is_task_transferable(
    package: &SemanticPackage,
    value_type: &ValueType,
) -> bool {
    match value_type {
        ValueType::Reference(_) => false,
        ValueType::Optional(inner) => value_type_is_task_transferable(package, inner),
        ValueType::Union(arms) => arms
            .iter()
            .all(|arm| value_type_is_task_transferable(package, arm)),
        ValueType::Object(identity) => {
            if !identity.namespace.starts_with("/deps/") {
                true
            } else if let Some(capability) = package.native_capability(identity) {
                capability.send
            } else {
                package
                    .projection
                    .projected_type_is_send(&identity.namespace, &identity.name)
            }
        }
        ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::UnorderedSet(inner)
        | ValueType::TaskOutcome(inner)
        | ValueType::SharedReference(inner) => {
            value_type_is_task_transferable(package, inner.value_type_ref())
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_is_task_transferable(package, key.value_type_ref())
                && value_type_is_task_transferable(package, value.value_type_ref())
        }
        ValueType::AsyncFunction(_, _, transferability, _)
        | ValueType::Task(_, transferability)
        | ValueType::ScopedTask(_, transferability) => {
            *transferability != TaskTransferability::Local
        }
        _ => true,
    }
}

pub(in crate::semantics) fn value_type_is_owned_static(value_type: &ValueType) -> bool {
    match value_type {
        ValueType::Reference(_) => false,
        ValueType::Optional(inner) => value_type_is_owned_static(inner),
        ValueType::Union(arms) => arms.iter().all(value_type_is_owned_static),
        ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::UnorderedSet(inner)
        | ValueType::TaskOutcome(inner)
        | ValueType::SharedReference(inner) => value_type_is_owned_static(inner.value_type_ref()),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_is_owned_static(key.value_type_ref())
                && value_type_is_owned_static(value.value_type_ref())
        }
        _ => true,
    }
}

fn async_boundary_transferability(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    function_span: Span,
) -> TaskTransferability {
    if node.span.start < function_span.start || node.span.end > function_span.end {
        return TaskTransferability::Transferable;
    }
    if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && call_base_contract(unit, node, &unit.typed_bindings)
            .ok()
            .flatten()
            .is_some_and(|contract| {
                contract.is_async && contract.task_transferability == TaskTransferability::Local
            })
    {
        return if projected_function_for_call(
            package,
            unit,
            callee,
            crate::syntax::call_is_unsafe(node),
        )
        .is_some()
        {
            TaskTransferability::RustProven
        } else {
            TaskTransferability::Local
        };
    }
    node.children
        .iter()
        .map(|child| async_boundary_transferability(package, unit, child, function_span))
        .min_by_key(|transferability| match transferability {
            TaskTransferability::Local => 0,
            TaskTransferability::RustProven => 1,
            TaskTransferability::Transferable => 2,
        })
        .unwrap_or(TaskTransferability::Transferable)
}

pub(in crate::semantics) fn infer_task_transferability(package: &mut SemanticPackage) {
    for unit_index in 0..package.units.len() {
        let updates = {
            let unit = &package.units[unit_index];
            let mut suspensions = Vec::new();
            collect_suspension_points(unit, &unit.tree.root, &mut suspensions);
            unit.functions
                .iter()
                .filter(|contract| contract.is_async)
                .map(|contract| {
                    let boundary = async_boundary_transferability(
                        package,
                        unit,
                        &unit.tree.root,
                        contract.span,
                    );
                    let values_transfer = contract.parameters.iter().all(|parameter| {
                        parameter.value_type.as_ref().is_none_or(|value_type| {
                            value_type_is_task_transferable(package, value_type)
                        })
                    }) && unit.typed_bindings.iter().all(|binding| {
                        if binding.span.start < contract.span.start
                            || binding.span.end > contract.span.end
                            || value_type_is_task_transferable(package, &binding.value_type)
                        {
                            return true;
                        }
                        let Some(events) = package.binding_events.get(&span_key(binding.span))
                        else {
                            return true;
                        };
                        let live_across_suspension = suspensions.iter().any(|suspension| {
                            suspension.start >= binding.visible_from
                                && suspension.end <= contract.span.end
                                && events.iter().any(|event| {
                                    matches!(
                                        event,
                                        BindingEvent::Read { span, .. }
                                            if span.start > suspension.end
                                    )
                                })
                        });
                        !live_across_suspension
                    });
                    let transferability = if unit.namespace.starts_with("/deps/")
                        || boundary == TaskTransferability::Local
                        || !values_transfer
                    {
                        TaskTransferability::Local
                    } else {
                        boundary
                    };
                    (contract.span, transferability)
                })
                .collect::<Vec<_>>()
        };
        for contract in &mut package.units[unit_index].functions {
            if let Some((_, transferability)) =
                updates.iter().find(|(span, _)| *span == contract.span)
            {
                contract.task_transferability = *transferability;
            }
        }
    }
    for unit in &mut package.units {
        for contract in &mut unit.functions {
            if !contract.is_async {
                continue;
            }
            contract.execution_requirements.tasks.local =
                contract.task_transferability == TaskTransferability::Local;
            contract.execution_requirements.tasks.transferable =
                contract.task_transferability != TaskTransferability::Local;
        }
    }
    synchronize_execution_requirements(package);
}

pub(in crate::semantics) fn synchronize_execution_requirements(package: &mut SemanticPackage) {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .map(|contract| {
            (
                span_key(contract.span),
                (
                    contract.task_transferability,
                    contract.execution_requirements,
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for unit in &mut package.units {
        for contract in unit
            .function_aliases
            .values_mut()
            .chain(unit.function_contracts_by_span.values_mut())
        {
            if let Some((transferability, requirements)) = contracts.get(&span_key(contract.span)) {
                contract.task_transferability = *transferability;
                contract.execution_requirements = *requirements;
            }
        }
    }
    let mut requirements = crate::execution::ExecutionRequirements::default();
    for contract in package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .filter(|contract| {
            contract.is_async
                && (contract.name == "main" || package.function_is_referenced(contract.span))
        })
    {
        requirements.merge(contract.execution_requirements);
    }
    package.execution_requirements = requirements;
    if package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .any(|contract| contract.name == "main" && contract.is_async)
    {
        package.execution_requirements.runtime.context = true;
        package.execution_requirements.runtime.wake_support = true;
    }
    if package.units.iter().any(|unit| {
        unit.typed_bindings
            .iter()
            .any(|binding| binding.value_type == ValueType::TaskScope)
    }) {
        package.execution_requirements.runtime.wake_support = true;
    }
    if package.units.iter().any(|unit| {
        [
            "host-read-async",
            "host-tcp-connect-async",
            "host-tcp-connect-host-async",
            "host-tcp-accept-async",
            "host-tcp-read-async",
            "host-tcp-write-async",
            "host-udp-send-to-async",
            "host-udp-receive-from-async",
            "host-dns-lookup-async",
            "host-tls-client-async",
            "host-tls-read-async",
            "host-tls-write-async",
            "host-tls-shutdown-async",
            "host-time-sleep-until",
            "host-process-signal-next",
        ]
        .iter()
        .any(|name| unit.source.text().contains(name))
    }) {
        package.execution_requirements.runtime.context = true;
        package.execution_requirements.runtime.wake_support = true;
        package.execution_requirements.runtime.blocking_delegation = true;
    }
}

pub(in crate::semantics) fn validate_suspension_ownership(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        let mut awaits = Vec::new();
        collect_suspension_points(unit, &unit.tree.root, &mut awaits);
        for contract in unit.functions.iter().filter(|contract| contract.is_async) {
            for binding in unit.typed_bindings.iter().filter(|binding| {
                matches!(binding.value_type, ValueType::Reference(_))
                    && binding.span.start >= contract.span.start
                    && binding.span.end <= contract.span.end
            }) {
                let Some(events) = package.binding_events.get(&span_key(binding.span)) else {
                    continue;
                };
                if let Some(suspension) = awaits.iter().find(|suspension| {
                    suspension.start >= binding.visible_from
                        && suspension.end <= contract.span.end
                        && events.iter().any(|event| {
                            matches!(
                                event,
                                BindingEvent::Read { span, .. } if span.start > suspension.end
                            )
                        })
                        && !reference_has_stable_local_owner(package, unit, contract, binding)
                }) {
                    return Err(failure(
                        &unit.source,
                        "T0073",
                        format!(
                            "non-owning reference `{}` remains live across `await`; end its use before suspension or transfer owned state",
                            binding.name
                        ),
                        *suspension,
                    ));
                }
            }
        }
    }
    Ok(())
}

pub(in crate::semantics) fn validate_task_consumption(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn discarded_task(
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<Option<Span>, SemanticFailure> {
        for child in &node.children {
            if node.kind == SyntaxKind::Block
                && child.kind == SyntaxKind::CallExpression
                && matches!(
                    infer_value_type(unit, child, &unit.typed_bindings)?,
                    Some(ValueType::Task(_, _) | ValueType::ScopedTask(_, _))
                )
            {
                return Ok(Some(child.span));
            }
            if let Some(span) = discarded_task(unit, child)? {
                return Ok(Some(span));
            }
        }
        Ok(None)
    }

    fn consumed(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        binding: &TypedBinding,
        consuming: bool,
    ) -> bool {
        let move_operand = node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("move");
        let await_operand = node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("await");
        let assignment_value = node.kind == SyntaxKind::Assignment;
        let task_consumer = node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                let [receiver, member] = callee.children.as_slice() else {
                    return false;
                };
                callee.kind == SyntaxKind::MemberExpression
                    && matches!(
                        infer_value_type(unit, receiver, &unit.typed_bindings),
                        Ok(Some(ValueType::TaskScope))
                    )
                    && matches!(node_text(&unit.source, member), "join" | "spawn")
            });
        if consuming
            && node.kind == SyntaxKind::Name
            && node_text(&unit.source, node) == binding.name
            && unit
                .typed_bindings
                .iter()
                .rev()
                .find(|candidate| {
                    candidate.name == binding.name
                        && candidate.is_visible_at(unit.source.id(), node.span.start)
                })
                .is_some_and(|candidate| candidate.span == binding.span)
        {
            return true;
        }
        node.children.iter().enumerate().any(|(index, child)| {
            consumed(
                unit,
                child,
                binding,
                consuming
                    || move_operand
                    || await_operand
                    || (assignment_value && index == 1)
                    || (task_consumer && index == 1),
            )
        })
    }

    for unit in &package.units {
        if let Some(span) = discarded_task(unit, &unit.tree.root)? {
            return Err(failure(
                &unit.source,
                "T0076",
                "task must be awaited, joined, or bound for later consumption",
                span,
            ));
        }
        for binding in unit.typed_bindings.iter().filter(|binding| {
            matches!(
                binding.value_type,
                ValueType::Task(_, _) | ValueType::ScopedTask(_, _)
            )
        }) {
            if !consumed(unit, &unit.tree.root, binding, false) {
                return Err(failure(
                    &unit.source,
                    "T0076",
                    format!(
                        "task `{}` must be awaited or joined before its scope ends",
                        binding.name
                    ),
                    binding.span,
                ));
            }
        }
    }

    Ok(())
}
pub(in crate::semantics) fn validate_task_transferability(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn visit(unit: &SemanticUnit, node: &SyntaxNode) -> Result<Option<Span>, SemanticFailure> {
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && node_text(&unit.source, member) == "spawn"
            && infer_value_type(unit, receiver, &unit.typed_bindings)? == Some(ValueType::TaskScope)
            && let Some(arguments) = node.children.get(1)
            && let Some(argument) = arguments.children.first()
        {
            let callable = argument.children.last().unwrap_or(argument);
            if matches!(
                infer_value_type(unit, callable, &unit.typed_bindings)?,
                Some(
                    ValueType::AsyncFunction(_, _, TaskTransferability::Local, _)
                        | ValueType::Task(_, TaskTransferability::Local)
                )
            ) {
                return Ok(Some(callable.span));
            }
        }
        for child in &node.children {
            if let Some(span) = visit(unit, child)? {
                return Ok(Some(span));
            }
        }
        Ok(None)
    }

    if package.execution_strategy != crate::execution::ExecutionStrategy::Parallel {
        return Ok(());
    }
    for unit in &package.units {
        if let Some(span) = visit(unit, &unit.tree.root)? {
            return Err(failure(
                &unit.source,
                "T0104",
                "executor-local async callable cannot be spawned by the threaded executor",
                span,
            ));
        }
    }
    Ok(())
}
