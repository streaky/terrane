use std::collections::BTreeSet;

use crate::{
    InvocationMode, Span,
    syntax::{SyntaxKind, SyntaxNode},
};

use super::{async_tasks::value_type_is_task_transferable, span_key};
use crate::semantics::{
    analysis::resolved_function_contract,
    diagnostics::{failure, node_text, unary_operator_text},
    expressions::infer_value_type,
    model::{
        FunctionContract, SemanticFailure, SemanticPackage, SemanticUnit, TaskTransferability,
        TypedBinding, ValueType,
    },
    namespaces::class_designator_identity,
    objects::{destination_projected_type, same_projected_native_family},
    ownership::{binding_initializer, find_node_by_span},
};

fn node_with_span(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    if node.span == span {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| node_with_span(child, span))
}

fn declaration_node_with_name_span(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && node.children.iter().any(|child| child.span == span)
    {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| declaration_node_with_name_span(child, span))
}

pub(crate) fn callback_contract<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    value: &SyntaxNode,
) -> Option<&'a FunctionContract> {
    if value.kind == SyntaxKind::Name {
        if let Some(contract) =
            resolved_function_contract(unit, node_text(&unit.source, value), value.span.start)
        {
            return Some(contract);
        }
        let declaration = package
            .resolve_name_at(unit, value.span.start, node_text(&unit.source, value))?
            .declaration_span?;
        let declaration_node = declaration_node_with_name_span(&unit.tree.root, declaration)
            .or_else(|| node_with_span(&unit.tree.root, declaration))?;
        let initializer = binding_initializer(declaration_node)?;
        if initializer.span.start >= value.span.start {
            return None;
        }
        return callback_contract(package, unit, initializer);
    }
    if value.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = value.children.as_slice()
        && let Ok(Some(ValueType::Object(owner))) =
            infer_value_type(unit, receiver, &unit.typed_bindings)
    {
        return unit.functions.iter().find(|contract| {
            contract.owner.as_deref() == Some(owner.name.as_str())
                && contract.name == node_text(&unit.source, member)
        });
    }
    unit.functions
        .iter()
        .find(|contract| contract.span == value.span)
}

fn captured_binding<'a>(
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    contract: &FunctionContract,
    name: &str,
) -> Option<&'a TypedBinding> {
    let declaration = package
        .resolve_name_at(unit, contract.span.start, name)?
        .declaration_span?;
    unit.typed_bindings
        .iter()
        .find(|binding| binding.span == declaration)
}
fn validate_projected_callback_contract(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    value: &SyntaxNode,
    contract: &FunctionContract,
    callback: &crate::rust_interop::projection::ProjectedType,
    consumed_once: &mut BTreeSet<(u32, usize, usize)>,
) -> Result<(), SemanticFailure> {
    let crate::rust_interop::projection::ProjectedType::Callback {
        invocation_mode,
        retained,
        send,
        parameter_borrows,
        parameter_rust_types,
        ..
    } = callback
    else {
        unreachable!("callback validation requires callback metadata");
    };
    let reject = |code, message| Err(failure(&unit.source, code, message, value.span));
    for (index, parameter) in contract.parameters.iter().enumerate() {
        if parameter_borrows.get(index) == Some(&true)
            && parameter_rust_types.get(index).is_some_and(|rust_type| {
                matches!(
                    syn::parse_str::<syn::Type>(rust_type),
                    Ok(syn::Type::Reference(reference)) if reference.mutability.is_none()
                )
            })
            && package
                .units
                .iter()
                .find(|source| source.source.id() == contract.span.file)
                .and_then(|source| {
                    source
                        .functions
                        .iter()
                        .find(|actual| actual.span == contract.span)
                })
                .and_then(|source| {
                    source
                        .parameters
                        .iter()
                        .find(|actual| actual.span == parameter.span)
                })
                .is_some_and(|source| source.mutable)
        {
            return Err(failure(
                &unit.source,
                "T0139",
                format!(
                    "mutating callback parameter `{}` requires a mutable native reference; the selected callback supplies a shared reference",
                    parameter.name
                ),
                value.span,
            ));
        }
    }
    if *send && contract.task_transferability == TaskTransferability::Local {
        return reject(
            "T0081",
            "projected callback requires a transferable callable",
        );
    }
    if *retained && contract.owner.is_some() && !contract.is_static {
        return reject(
            "T0082",
            "retained callback cannot borrow an object receiver",
        );
    }
    if *retained
        && contract.captures.iter().any(|capture| {
            captured_binding(package, unit, contract, capture)
                .is_some_and(|binding| matches!(binding.value_type, ValueType::Reference(_)))
        })
    {
        return reject("T0083", "retained callback cannot capture a borrowed value");
    }
    if *send
        && contract.captures.iter().any(|capture| {
            captured_binding(package, unit, contract, capture).is_some_and(|binding| {
                !value_type_is_task_transferable(package, &binding.value_type)
            })
        })
    {
        return reject(
            "T0084",
            "projected callback requires transferable captured values",
        );
    }
    if *invocation_mode == InvocationMode::Consuming
        && value.kind == SyntaxKind::Name
        && let Some(declaration) = package
            .resolve_name_at(unit, value.span.start, node_text(&unit.source, value))
            .and_then(|symbol| symbol.declaration_span)
        && unit
            .typed_bindings
            .iter()
            .any(|binding| binding.span == declaration)
        && !consumed_once.insert(span_key(declaration))
    {
        return reject("T0086", "one-shot projected callback was already consumed");
    }
    Ok(())
}

fn validate_channel_endpoint_extraction(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    extracted: &mut BTreeSet<(u32, usize, usize, String)>,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = node.children.as_slice()
        && receiver.kind == SyntaxKind::Name
        && matches!(node_text(&unit.source, member), "sender" | "receiver")
        && let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, receiver)
                && binding.is_visible_at(unit.source.id(), receiver.span.start)
        })
        && matches!(binding.value_type, ValueType::ChannelPair(_))
        && !extracted.insert((
            binding.span.file,
            binding.span.start,
            binding.span.end,
            node_text(&unit.source, member).to_owned(),
        ))
    {
        return Err(failure(
            &unit.source,
            "T0110",
            format!(
                "channel pair `{}` endpoint `{}` was already moved",
                binding.name,
                node_text(&unit.source, member)
            ),
            node.span,
        ));
    }
    Ok(())
}

fn validate_projected_borrowed_async_call(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    immediately_awaited: bool,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::CallExpression
        && !immediately_awaited
        && let [callee, _] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && receiver.kind == SyntaxKind::Name
        && package
            .projection
            .has_borrowed_async_method_named(node_text(&unit.source, member))
        && let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, receiver)
                && binding.is_visible_at(unit.source.id(), receiver.span.start)
        })
        && let ValueType::Object(identity) = &binding.value_type
        && let Some(method) = package.projection.method(
            &identity.namespace,
            &identity.name,
            node_text(&unit.source, member),
            false,
        )
        && method.is_async
        && matches!(
            method.receiver,
            Some(
                crate::rust_interop::projection::Receiver::Borrow
                    | crate::rust_interop::projection::Receiver::MutableBorrow
            )
        )
    {
        return Err(failure(
            &unit.source,
            "T0106",
            "borrowed projected async operation must be awaited immediately",
            node.span,
        ));
    }
    Ok(())
}

fn projected_chain_role(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Option<crate::rust_interop::projection::ChainRole> {
    if node.kind != SyntaxKind::CallExpression {
        return None;
    }
    let [callee, _] = node.children.as_slice() else {
        return None;
    };
    let function =
        projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))?;
    function.chain_role.or_else(|| {
        let receiver = callee.children.first()?;
        matches!(
            infer_value_type(unit, receiver, &unit.typed_bindings)
                .ok()
                .flatten(),
            Some(ValueType::InvocationScopedNative {
                expression_scoped: true,
                ..
            })
        )
        .then_some(crate::rust_interop::projection::ChainRole::Terminal)
    })
}

fn collect_chain_receivers(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    receivers: &mut BTreeSet<(u32, usize, usize)>,
) {
    if node.kind == SyntaxKind::Block {
        for statement in &node.children {
            let mut value = statement;
            while value.kind == SyntaxKind::GroupExpression
                && let [inner] = value.children.as_slice()
            {
                value = inner;
            }
            if value.kind == SyntaxKind::CallExpression
                && matches!(
                    infer_value_type(unit, value, &unit.typed_bindings)
                        .ok()
                        .flatten(),
                    Some(ValueType::InvocationScopedNative {
                        expression_scoped: true,
                        ..
                    })
                )
            {
                receivers.insert(span_key(value.span));
            }
        }
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, _] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, _] = callee.children.as_slice()
        && projected_chain_role(package, unit, node).is_some()
    {
        let mut receiver = receiver;
        while receiver.kind == SyntaxKind::GroupExpression
            && let [inner] = receiver.children.as_slice()
        {
            receiver = inner;
        }
        receivers.insert(span_key(receiver.span));
    }
    if node.kind == SyntaxKind::CallExpression
        && projected_function_for_call(
            package,
            unit,
            node.children.first().expect("call expression has a callee"),
            crate::syntax::call_is_unsafe(node),
        )
        .is_some()
        && let Some(arguments) = node.children.get(1)
    {
        for argument in &arguments.children {
            let mut value = argument.children.last().unwrap_or(argument);
            while value.kind == SyntaxKind::GroupExpression
                && let Some(inner) = value.children.first()
            {
                value = inner;
            }
            if matches!(
                projected_chain_role(package, unit, value),
                Some(
                    crate::rust_interop::projection::ChainRole::Root
                        | crate::rust_interop::projection::ChainRole::Continue
                )
            ) {
                receivers.insert(span_key(value.span));
            }
        }
    }
    for child in &node.children {
        collect_chain_receivers(package, unit, child, receivers);
    }
}

pub(crate) fn projected_macro_for_call<'a>(
    package: &'a SemanticPackage,
    unit: &SemanticUnit,
    callee: &SyntaxNode,
) -> Option<&'a crate::rust_interop::projection::ProjectedItem> {
    if callee.kind != SyntaxKind::Name {
        return None;
    }
    let symbol =
        package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
    package
        .projection
        .item(&symbol.namespace, &symbol.name)
        .filter(|item| {
            matches!(
                item.kind,
                crate::rust_interop::projection::ProjectedKind::Macro(_)
            )
        })
}

pub(in crate::semantics) fn projected_function_for_call<'a>(
    package: &'a SemanticPackage,
    unit: &SemanticUnit,
    callee: &SyntaxNode,
    is_unsafe: bool,
) -> Option<&'a crate::rust_interop::projection::ProjectedFunction> {
    let is_unsafe = is_unsafe || crate::syntax::call_is_unsafe(callee);
    let mut callee = callee;
    while matches!(
        callee.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression | SyntaxKind::AppliedType
    ) {
        callee = callee.children.first()?;
    }
    if callee.kind == SyntaxKind::ConstructionExpression {
        let identity = class_designator_identity(unit, callee.children.first()?)?;
        return package
            .projection
            .projected_constructor(&identity.namespace, &identity.name);
    }
    if callee.kind == SyntaxKind::Name {
        let lookup_name = if is_unsafe {
            format!("unsafe::{}", node_text(&unit.source, callee))
        } else {
            node_text(&unit.source, callee).to_owned()
        };
        let symbol = package.resolve_name_at(unit, callee.span.start, &lookup_name)?;
        return package
            .projection
            .item(&symbol.namespace, &symbol.name)
            .and_then(|item| match &item.kind {
                crate::rust_interop::projection::ProjectedKind::Function(function)
                | crate::rust_interop::projection::ProjectedKind::Macro(function) => Some(function),
                _ => None,
            });
    }
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    let (identity, is_static) = if callee.kind == SyntaxKind::StaticMemberExpression {
        (class_designator_identity(unit, receiver)?, true)
    } else {
        let receiver_type = infer_value_type(unit, receiver, &unit.typed_bindings)
            .ok()
            .flatten()?;
        let identity = match receiver_type {
            ValueType::Object(identity)
            | ValueType::InvocationScopedNative {
                family: identity, ..
            } => identity,
            ValueType::Reference(inner) | ValueType::SharedReference(inner) => {
                let ValueType::Object(identity) = inner.value_type() else {
                    return None;
                };
                identity
            }
            _ => return None,
        };
        (identity, false)
    };
    package.projection.method_for_native(
        &identity.namespace,
        &identity.name,
        identity.native_projection.as_deref(),
        node_text(&unit.source, member),
        is_static,
        is_unsafe,
    )
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum InvocationScopedRegion {
    Unbound,
    Expression,
    Bound((u32, usize, usize)),
}

fn invocation_scoped_region(value_type: &ValueType) -> Option<InvocationScopedRegion> {
    match value_type {
        ValueType::InvocationScopedNative {
            expression_scoped: true,
            ..
        } => Some(InvocationScopedRegion::Expression),
        ValueType::InvocationScopedNative { region, .. } => Some(region.map_or(
            InvocationScopedRegion::Unbound,
            InvocationScopedRegion::Bound,
        )),
        ValueType::Optional(inner) => invocation_scoped_region(inner),
        ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::Reference(inner)
        | ValueType::SharedReference(inner) => invocation_scoped_region(inner.value_type_ref()),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => invocation_scoped_region(key.value_type_ref())
            .or_else(|| invocation_scoped_region(value.value_type_ref())),
        ValueType::Tuple(item, _) => invocation_scoped_region(item.value_type_ref()),
        _ => None,
    }
}

fn projected_native_name(projected: &crate::rust_interop::projection::ProjectedType) -> String {
    match projected {
        crate::rust_interop::projection::ProjectedType::InvocationScoped { name, .. } => {
            name.clone()
        }
        _ => projected.rust_type(),
    }
}

fn visible_binding<'a>(
    unit: &'a SemanticUnit,
    name_node: &SyntaxNode,
) -> Option<&'a crate::semantics::model::TypedBinding> {
    let name = node_text(&unit.source, name_node);
    unit.typed_bindings
        .iter()
        .filter(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), name_node.span.start)
        })
        .max_by_key(|binding| binding.visible_from)
}

fn local_lender_name(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contract: &FunctionContract,
    function_span: Span,
) -> Option<(String, Span)> {
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        let binding = visible_binding(unit, node)?;
        let externally_lent = unit
            .reference_provenance
            .get(&(binding.span.start, binding.span.end))
            .is_some_and(|provenance| {
                provenance.external_lender && provenance.lender_parameter.is_some()
            });
        if !externally_lent
            && !contract
                .parameters
                .iter()
                .any(|parameter| parameter.name == name)
            && binding.scope == Some(function_span)
            && invocation_scoped_region(&binding.value_type).is_none()
        {
            return Some((name.to_owned(), binding.span));
        }
    }
    node.children
        .iter()
        .find_map(|child| local_lender_name(unit, child, contract, function_span))
}

fn local_lender_in_scoped_expression(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contract: &FunctionContract,
    function_span: Span,
) -> Option<(String, Span)> {
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && (projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
            .is_some_and(|function| {
                matches!(
                    function.result,
                    crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                )
            })
            || (callee.kind == SyntaxKind::Name
                && resolved_function_contract(
                    unit,
                    node_text(&unit.source, callee),
                    callee.span.start,
                )
                .is_some_and(|function| {
                    matches!(
                        function.return_type,
                        Some(ValueType::InvocationScopedNative { .. })
                    )
                })))
    {
        let lender = arguments
            .children
            .iter()
            .find_map(|argument| local_lender_name(unit, argument, contract, function_span));
        if lender.is_some() {
            return lender;
        }
    }
    node.children.iter().find_map(|child| {
        local_lender_in_scoped_expression(package, unit, child, contract, function_span)
    })
}

fn collect_scoped_append_types(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    receiver_name: &str,
    results: &mut Vec<crate::rust_interop::projection::ProjectedType>,
) {
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && node_text(&unit.source, receiver) == receiver_name
        && node_text(&unit.source, member) == "append"
        && let Some(argument) = arguments.children.first()
    {
        let value = argument.children.last().unwrap_or(argument);
        if let Ok(Some(value_type)) = infer_value_type(unit, value, &unit.typed_bindings)
            && let Ok(projected) = destination_projected_type(package, &value_type)
            && !results
                .iter()
                .any(|result| same_projected_native_family(result, &projected))
        {
            results.push(projected);
        }
    }
    for child in &node.children {
        collect_scoped_append_types(package, unit, child, receiver_name, results);
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one syntax walk keeps invocation-region rejection rules in source order"
)]
fn validate_invocation_scoped_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::FunctionDeclaration
        && let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
        && !unit
            .source
            .path()
            .to_string_lossy()
            .starts_with("<terrane>/projected/")
        && let Some(parameter) = contract.parameters.iter().find(|parameter| {
            matches!(
                &parameter.value_type,
                Some(ValueType::InvocationScopedNative {
                    concrete: false,
                    ..
                })
            )
        })
    {
        return Err(failure(
            &unit.source,
            "T0119",
            format!(
                "parameter `{}` cannot use unresolved `host-invocation-scoped-native` as a source type",
                parameter.name
            ),
            parameter.span,
        ));
    }
    if node.kind == SyntaxKind::CallExpression
        && let Some(function_span) = unit
            .enclosing_function_spans
            .get(&node.span.start)
            .copied()
            .flatten()
        && let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.span == function_span)
        && let Some((name, span)) =
            local_lender_in_scoped_expression(package, unit, node, contract, function_span)
    {
        return Err(failure(
            &unit.source,
            "T0119",
            format!("invocation-scoped native result cannot borrow local value `{name}`"),
            span,
        ));
    }

    if node.kind == SyntaxKind::CallExpression
        && let [callee, _] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && receiver.kind == SyntaxKind::Name
        && node_text(&unit.source, member) == "append"
    {
        let receiver_name = node_text(&unit.source, receiver);
        let scoped_wildcard_list = unit
            .typed_bindings
            .iter()
            .filter(|binding| {
                binding.name == receiver_name
                    && binding.is_visible_at(unit.source.id(), receiver.span.start)
            })
            .max_by_key(|binding| binding.visible_from)
            .is_some_and(|binding| {
                matches!(
                    &binding.value_type,
                    ValueType::List(item)
                        if matches!(
                            item.value_type_ref(),
                            ValueType::InvocationScopedNative {
                                concrete: false, ..
                            }
                        )
                )
            });
        if scoped_wildcard_list
            && let Some(function_span) = unit
                .enclosing_function_spans
                .get(&node.span.start)
                .copied()
                .flatten()
            && let Some(function) = find_node_by_span(&unit.tree.root, function_span)
        {
            let mut item_types = Vec::new();
            collect_scoped_append_types(package, unit, function, receiver_name, &mut item_types);
            if item_types.len() > 1 {
                return Err(failure(
                    &unit.source,
                    "T0119",
                    format!(
                        "invocation-scoped list `{receiver_name}` cannot mix native element types"
                    ),
                    node.span,
                ));
            }
        }
    }

    if node.kind == SyntaxKind::Assignment
        && let [target, value] = node.children.as_slice()
        && target.kind == SyntaxKind::Name
        && let Some(binding) = visible_binding(unit, target)
        && let Some(declaration) = find_node_by_span(&unit.tree.root, binding.span)
        && let Some(initializer) = declaration.children.last()
        && let Some(initial_type) = infer_value_type(unit, initializer, &unit.typed_bindings)?
        && let Some(reassigned_type) = infer_value_type(unit, value, &unit.typed_bindings)?
        && let Ok(initial_projected) = destination_projected_type(package, &initial_type)
        && let Ok(reassigned_projected) = destination_projected_type(package, &reassigned_type)
        && matches!(
            (&initial_projected, &reassigned_projected),
            (
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. },
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
            )
        )
        && !same_projected_native_family(&initial_projected, &reassigned_projected)
    {
        return Err(failure(
            &unit.source,
            "T0019",
            format!(
                "cannot reassign invocation-scoped `{}` from `{}` to `{}`",
                binding.name,
                projected_native_name(&initial_projected),
                projected_native_name(&reassigned_projected)
            ),
            value.span,
        ));
    }

    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && let Some(value) = node.children.last()
        && let Some(value_type) = infer_value_type(unit, value, &unit.typed_bindings)?
        && let Some(region) = invocation_scoped_region(&value_type)
    {
        let local_span = unit
            .enclosing_function_spans
            .get(&node.span.start)
            .copied()
            .flatten();
        let local_region = local_span.map(|span| (span.file, span.start, span.end));
        let region_local_function = local_span.is_some_and(|span| {
            unit.functions.iter().any(|function| {
                function.span == span
                    && matches!(
                        function.return_type,
                        Some(ValueType::InvocationScopedNative { .. })
                    )
            })
        });
        if matches!(region, InvocationScopedRegion::Bound(region) if local_region != Some(region))
            || (region == InvocationScopedRegion::Unbound && !region_local_function)
            || region == InvocationScopedRegion::Expression
        {
            return Err(failure(
                &unit.source,
                "T0119",
                "invocation-scoped native value cannot escape into ordinary storage",
                value.span,
            ));
        }
    }
    if node.kind == SyntaxKind::AnonymousFunction
        && let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
        && let Some(capture) = contract.captures.iter().find(|capture| {
            unit.typed_bindings.iter().any(|binding| {
                binding.name == **capture
                    && binding.is_visible_at(unit.source.id(), node.span.start)
                    && invocation_scoped_region(&binding.value_type).is_some()
            })
        })
    {
        return Err(failure(
            &unit.source,
            "T0119",
            format!(
                "invocation-scoped native value `{capture}` cannot be captured outside its invocation region"
            ),
            node.span,
        ));
    }
    if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("await")
        && let Some(binding) = unit.typed_bindings.iter().find(|binding| {
            binding.is_visible_at(unit.source.id(), node.span.start)
                && invocation_scoped_region(&binding.value_type).is_some()
        })
    {
        return Err(failure(
            &unit.source,
            "T0119",
            format!(
                "invocation-scoped native value `{}` cannot remain live across suspension",
                binding.name
            ),
            node.span,
        ));
    }
    Ok(())
}

fn projected_argument_for_parameter<'a>(
    unit: &SemanticUnit,
    function: &crate::rust_interop::projection::ProjectedFunction,
    arguments: &'a SyntaxNode,
    parameter_index: usize,
) -> Option<&'a SyntaxNode> {
    let mut positional = 0;
    arguments.children.iter().find_map(|argument| {
        let named = argument
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
        let argument_index = named.map_or_else(
            || {
                let current = positional;
                positional += 1;
                current
            },
            |name| {
                function
                    .parameters
                    .iter()
                    .position(|candidate| candidate.name == node_text(&unit.source, name))
                    .unwrap_or(usize::MAX)
            },
        );
        (argument_index == parameter_index).then(|| argument.children.last().unwrap_or(argument))
    })
}

fn validate_projected_callback_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    consumed_once: &mut BTreeSet<(u32, usize, usize)>,
    extracted_channel_endpoints: &mut BTreeSet<(u32, usize, usize, String)>,
    immediately_awaited: bool,
    chain_receivers: &BTreeSet<(u32, usize, usize)>,
) -> Result<(), SemanticFailure> {
    validate_invocation_scoped_node(package, unit, node)?;
    if matches!(
        projected_chain_role(package, unit, node),
        Some(
            crate::rust_interop::projection::ChainRole::Root
                | crate::rust_interop::projection::ChainRole::Continue
        )
    ) && !chain_receivers.contains(&span_key(node.span))
    {
        return Err(failure(
            &unit.source,
            "T0112",
            "chain-only dependency value must terminate within one expression",
            node.span,
        ));
    }
    validate_channel_endpoint_extraction(unit, node, extracted_channel_endpoints)?;
    validate_projected_borrowed_async_call(package, unit, node, immediately_awaited)?;
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && let Some(function) =
            projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
    {
        for (index, parameter) in function.parameters.iter().enumerate() {
            let Some(value) = projected_argument_for_parameter(unit, function, arguments, index)
            else {
                continue;
            };
            if matches!(
                parameter.ty,
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
            ) && let Some(actual) = infer_value_type(unit, value, &unit.typed_bindings)?
                && let Ok(actual_projected) = destination_projected_type(package, &actual)
                && matches!(
                    actual_projected,
                    crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                )
                && !same_projected_native_family(&parameter.ty, &actual_projected)
            {
                return Err(failure(
                    &unit.source,
                    "T0019",
                    format!(
                        "incompatible argument types: expected `{}`, found `{}`",
                        projected_native_name(&parameter.ty),
                        projected_native_name(&actual_projected)
                    ),
                    value.span,
                ));
            }
            if !matches!(
                parameter.ty,
                crate::rust_interop::projection::ProjectedType::Callback { .. }
            ) {
                continue;
            }
            let Some(contract) = callback_contract(package, unit, value) else {
                continue;
            };
            let callback_parameter = unit
                .projected_call_specializations
                .get(&span_key(node.span))
                .and_then(|selection| selection.projected_parameters.get(index))
                .unwrap_or(parameter);
            validate_projected_callback_contract(
                package,
                unit,
                value,
                contract,
                &callback_parameter.ty,
                consumed_once,
            )?;
        }
    }
    for child in &node.children {
        validate_projected_callback_node(
            package,
            unit,
            child,
            consumed_once,
            extracted_channel_endpoints,
            node.kind == SyntaxKind::UnaryExpression
                && unary_operator_text(unit, node).as_deref() == Some("await")
                || node.kind == SyntaxKind::GroupExpression && immediately_awaited,
            chain_receivers,
        )?;
    }
    Ok(())
}

pub(in crate::semantics) fn validate_projected_callback_arguments(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        let mut chain_receivers = BTreeSet::new();
        collect_chain_receivers(package, unit, &unit.tree.root, &mut chain_receivers);
        validate_projected_callback_node(
            package,
            unit,
            &unit.tree.root,
            &mut BTreeSet::new(),
            &mut BTreeSet::new(),
            false,
            &chain_receivers,
        )?;
    }
    Ok(())
}
