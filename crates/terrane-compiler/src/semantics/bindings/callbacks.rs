use std::collections::BTreeSet;

use crate::{
    InvocationMode, Span,
    syntax::{SyntaxKind, SyntaxNode},
};

use super::{
    async_tasks::{
        AutoTraitObligation, value_type_is_owned_static, value_type_satisfies_auto_trait,
    },
    span_key,
};
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
    if value.kind == SyntaxKind::GroupExpression {
        return callback_contract(package, unit, value.children.first()?);
    }
    if value.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, value).as_deref() == Some("move")
    {
        return callback_contract(package, unit, value.children.last()?);
    }
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
fn callback_capture_trait_failure(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    contract: &FunctionContract,
    send: bool,
    sync: bool,
) -> Option<(&'static str, &'static str)> {
    for (required, obligation, code, message) in [
        (
            send,
            AutoTraitObligation::Send,
            "T0084",
            "projected callback requires transferable captured values",
        ),
        // Mutable/consuming bodies are stored behind exclusive ownership or a
        // Mutex: Send suffices for the carrier's Sync, unlike shared Fn bodies.
        (
            sync && contract.written_invocation_mode == InvocationMode::Shared,
            AutoTraitObligation::Sync,
            "T0085",
            "projected callback requires shareable captured values (`Sync`)",
        ),
    ] {
        if required
            && contract.captures.iter().any(|capture| {
                captured_binding(package, unit, contract, capture).is_some_and(|binding| {
                    !value_type_satisfies_auto_trait(package, &binding.value_type, obligation)
                })
            })
        {
            return Some((code, message));
        }
    }
    None
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
        sync,
        parameter_borrows,
        parameter_rust_types,
        ..
    } = callback
    else {
        unreachable!("callback validation requires callback metadata");
    };
    let reject = |code, message| Err(failure(&unit.source, code, message, value.span));
    if callback.has_unsupported_mutable_callback_borrow() {
        return reject(
            "T0119",
            "mutable callback inputs require an identity-preserving native representation",
        );
    }
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
                .is_some_and(super::super::model::ParameterContract::requires_mutable_reference)
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
                .is_some_and(|binding| !value_type_is_owned_static(&binding.value_type))
        })
    {
        return reject("T0083", "retained callback cannot capture a borrowed value");
    }
    if let Some((code, message)) =
        callback_capture_trait_failure(package, unit, contract, *send, *sync)
    {
        return reject(code, message);
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

pub(in crate::semantics) fn projected_receiver_chain_role(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    mut receiver: &SyntaxNode,
) -> Option<crate::rust_interop::projection::ChainRole> {
    while receiver.kind == SyntaxKind::GroupExpression {
        receiver = receiver.children.first()?;
    }
    projected_chain_role(package, unit, receiver)
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
        if matches!(
            projected_receiver_chain_role(package, unit, receiver),
            Some(
                crate::rust_interop::projection::ChainRole::Root
                    | crate::rust_interop::projection::ChainRole::Continue
            )
        ) {
            return Some(
                if matches!(
                    function.result,
                    crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                ) {
                    crate::rust_interop::projection::ChainRole::Continue
                } else {
                    crate::rust_interop::projection::ChainRole::Terminal
                },
            );
        }
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
    if node.kind == SyntaxKind::ReturnStatement
        && unit.invocation_scoped_result_at(node.span).is_some()
    {
        for value in &node.children {
            let mut value = value;
            while value.kind == SyntaxKind::GroupExpression && value.children.len() == 1 {
                value = &value.children[0];
            }
            if projected_chain_role(package, unit, value).is_some() {
                receivers.insert(span_key(value.span));
            }
        }
    }
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
        && node.children.first().is_some_and(|callee| {
            projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
                .is_some()
                || (unit.invocation_scoped_result_at(node.span).is_some()
                    && projected_macro_for_call(package, unit, callee).is_some())
        })
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
            ValueType::Reference(inner)
            | ValueType::MutableReference(inner)
            | ValueType::SharedReference(inner) => {
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
        | ValueType::TaskGroup(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner) => invocation_scoped_region(inner.value_type_ref()),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => invocation_scoped_region(key.value_type_ref())
            .or_else(|| invocation_scoped_region(value.value_type_ref())),
        ValueType::Tuple(item, _) => invocation_scoped_region(item.value_type_ref()),
        _ => None,
    }
}
fn projected_native_lifetimes(
    projected: &crate::rust_interop::projection::ProjectedType,
    lifetimes: &mut Vec<String>,
) {
    use crate::rust_interop::projection::ProjectedType;
    match projected {
        ProjectedType::Reference {
            inner, lifetime, ..
        } => {
            if let Some(lifetime) = lifetime {
                lifetimes.push(lifetime.clone());
            }
            projected_native_lifetimes(inner, lifetimes);
        }
        ProjectedType::Sequence { item, .. }
        | ProjectedType::Set { item, .. }
        | ProjectedType::Optional(item)
        | ProjectedType::AsyncIterationStep(item) => projected_native_lifetimes(item, lifetimes),
        ProjectedType::Mapping { key, value, .. } => {
            projected_native_lifetimes(key, lifetimes);
            projected_native_lifetimes(value, lifetimes);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                projected_native_lifetimes(item, lifetimes);
            }
        }
        ProjectedType::InvocationScoped {
            lifetimes: nested, ..
        } => lifetimes.extend(nested.iter().cloned()),
        _ => {}
    }
}

fn call_for_scoped_native<'a>(
    unit: &SemanticUnit,
    node: &'a SyntaxNode,
    rust_type: &str,
) -> Option<&'a SyntaxNode> {
    if unit
        .projected_call_specializations
        .get(&(node.span.file, node.span.start, node.span.end))
        .is_some_and(|specialization| {
            matches!(
                &specialization.projected_result,
                crate::rust_interop::projection::ProjectedType::InvocationScoped {
                    rust_type: result,
                    ..
                } if result == rust_type
            )
        })
    {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| call_for_scoped_native(unit, child, rust_type))
}

fn scoped_native_lender_bindings(
    unit: &SemanticUnit,
    binding: &TypedBinding,
    region: Span,
) -> Option<Vec<Span>> {
    scoped_native_lender_bindings_inner(unit, binding, region, &mut BTreeSet::new())
}

fn collect_scoped_receiver_lenders(
    unit: &SemanticUnit,
    callee: &SyntaxNode,
    lifetimes: &[String],
    region: Span,
    visited: &mut BTreeSet<Span>,
    lender_spans: &mut BTreeSet<Span>,
    matched_lifetimes: &mut BTreeSet<String>,
) -> Option<()> {
    if let Some(receiver) = callee.children.first()
        && let Some(ValueType::InvocationScopedNative {
            lifetimes: receiver_lifetimes,
            ..
        }) = infer_value_type(unit, receiver, &unit.typed_bindings)
            .ok()
            .flatten()
    {
        for lifetime in lifetimes.iter().filter(|lifetime| {
            lifetime.as_str() != "'static" && receiver_lifetimes.contains(lifetime)
        }) {
            matched_lifetimes.insert(lifetime.clone());
            collect_transitive_lender_bindings(unit, receiver.span, region, visited, lender_spans)?;
        }
    }
    Some(())
}

fn collect_scoped_call_receiver(
    unit: &SemanticUnit,
    callee: &SyntaxNode,
    region: Span,
    visited: &mut BTreeSet<Span>,
    lender_spans: &mut BTreeSet<Span>,
) -> Option<()> {
    if let Some(receiver) = callee.children.first() {
        collect_transitive_lender_bindings(unit, receiver.span, region, visited, lender_spans)?;
    }
    Some(())
}

fn scoped_native_lender_bindings_inner(
    unit: &SemanticUnit,
    binding: &TypedBinding,
    region: Span,
    visited: &mut BTreeSet<Span>,
) -> Option<Vec<Span>> {
    if !visited.insert(binding.span) {
        return None;
    }
    let region_node = find_node_by_span(&unit.tree.root, region)?;
    if !matches!(
        region_node.kind,
        SyntaxKind::WithRegion | SyntaxKind::FunctionDeclaration
    ) {
        return None;
    }
    let binding_node = find_node_by_span(&unit.tree.root, binding.span)?;
    let expression = binding_node.children.last()?;
    let ValueType::InvocationScopedNative {
        rust_type,
        lifetimes: output_lifetimes,
        ..
    } = &binding.value_type
    else {
        return None;
    };
    let mut lender_spans = BTreeSet::new();
    if let Some(call) = call_for_scoped_native(unit, expression, rust_type) {
        let specialization = unit.projected_call_specializations.get(&(
            call.span.file,
            call.span.start,
            call.span.end,
        ))?;
        let crate::rust_interop::projection::ProjectedType::InvocationScoped { lifetimes, .. } =
            &specialization.projected_result
        else {
            return None;
        };
        let callee = call.children.first()?;
        let mut matched_lifetimes = BTreeSet::new();
        if let Some(arguments) = call.children.get(1) {
            for (parameter, argument) in specialization
                .projected_parameters
                .iter()
                .zip(&arguments.children)
            {
                let mut parameter_lifetimes = Vec::new();
                projected_native_lifetimes(&parameter.ty, &mut parameter_lifetimes);
                if let Some(lifetime) = &parameter.borrow_lifetime {
                    parameter_lifetimes.push(lifetime.clone());
                }
                for lifetime in lifetimes.iter().filter(|lifetime| {
                    lifetime.as_str() != "'static"
                        && parameter_lifetimes.contains(lifetime)
                        && !(lifetime.as_str() == "'_"
                            && specialization.projected_receiver_lifetime.as_deref() == Some("'_"))
                }) {
                    matched_lifetimes.insert(lifetime.clone());
                    collect_transitive_lender_bindings(
                        unit,
                        argument.span,
                        region,
                        visited,
                        &mut lender_spans,
                    )?;
                }
            }
        }
        if let Some(receiver_lifetime) = &specialization.projected_receiver_lifetime {
            for lifetime in lifetimes
                .iter()
                .filter(|lifetime| lifetime.as_str() != "'static" && *lifetime == receiver_lifetime)
            {
                matched_lifetimes.insert(lifetime.clone());
                collect_scoped_call_receiver(unit, callee, region, visited, &mut lender_spans)?;
            }
        }
        collect_scoped_receiver_lenders(
            unit,
            callee,
            lifetimes,
            region,
            visited,
            &mut lender_spans,
            &mut matched_lifetimes,
        )?;
        if lifetimes
            .iter()
            .any(|lifetime| lifetime != "'static" && !matched_lifetimes.contains(lifetime))
        {
            return None;
        }
    } else {
        collect_transitive_lender_bindings(
            unit,
            expression.span,
            region,
            visited,
            &mut lender_spans,
        )?;
    }
    (!output_lifetimes
        .iter()
        .any(|lifetime| lifetime != "'static")
        || !lender_spans.is_empty())
    .then(|| lender_spans.into_iter().collect())
}

fn collect_transitive_lender_bindings(
    unit: &SemanticUnit,
    source: Span,
    region: Span,
    visited: &mut BTreeSet<Span>,
    lenders: &mut BTreeSet<Span>,
) -> Option<()> {
    let mut origins = BTreeSet::new();
    collect_flow_lenders(unit, source, &mut origins);
    if origins.is_empty() {
        return None;
    }
    for origin in origins {
        if let Some(binding) = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == origin)
            && invocation_scoped_region(&binding.value_type).is_some()
        {
            let nested = scoped_native_lender_bindings_inner(unit, binding, region, visited)?;
            lenders.extend(nested);
        } else {
            lenders.insert(origin);
        }
    }
    Some(())
}

fn collect_flow_lenders(unit: &SemanticUnit, source: Span, lenders: &mut BTreeSet<Span>) {
    for (&(file, start, end), origins) in &unit.flow_read_bindings {
        if file == source.file && start >= source.start && end <= source.end {
            lenders.extend(origins);
        }
    }
}
fn lender_is_transferable(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    span: Span,
    borrower_region: Span,
) -> bool {
    let Some(binding) = unit
        .typed_bindings
        .iter()
        .find(|binding| binding.span == span)
    else {
        return false;
    };
    let Some(scope) = binding.scope else {
        return false;
    };
    if scope.file != borrower_region.file
        || scope.start > borrower_region.start
        || scope.end < borrower_region.end
    {
        return false;
    }
    match &binding.value_type {
        ValueType::Reference(inner) | ValueType::SharedReference(inner) => {
            super::async_tasks::value_type_satisfies_auto_trait(
                package,
                inner.value_type_ref(),
                super::async_tasks::AutoTraitObligation::Sync,
            )
        }
        ValueType::MutableReference(inner) => super::async_tasks::value_type_satisfies_auto_trait(
            package,
            inner.value_type_ref(),
            super::async_tasks::AutoTraitObligation::Send,
        ),
        ValueType::InvocationScopedNative { .. } => {
            super::async_tasks::value_type_satisfies_auto_trait(
                package,
                &binding.value_type,
                super::async_tasks::AutoTraitObligation::Send,
            )
        }
        value_type => {
            super::async_tasks::value_type_satisfies_auto_trait(
                package,
                value_type,
                super::async_tasks::AutoTraitObligation::Send,
            ) && super::async_tasks::value_type_satisfies_auto_trait(
                package,
                value_type,
                super::async_tasks::AutoTraitObligation::Sync,
            )
        }
    }
}

fn scoped_native_transferable(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    binding: &TypedBinding,
) -> bool {
    let ValueType::InvocationScopedNative {
        concrete: true,
        region: Some((file, start, end)),
        ..
    } = &binding.value_type
    else {
        return false;
    };
    let Some(scope) = binding.scope else {
        return false;
    };
    let Some(region) = nearest_scoped_access_region(&unit.tree.root, binding.span) else {
        return false;
    };
    let lender_region = Span::new(*file, *start, *end);
    if scope.file != region.file
        || scope.start < region.start
        || scope.end > region.end
        || lender_region.file != region.file
        || lender_region.start > region.start
        || lender_region.end < region.end
        || !super::async_tasks::value_type_satisfies_auto_trait(
            package,
            &binding.value_type,
            super::async_tasks::AutoTraitObligation::Send,
        )
    {
        return false;
    }
    let Some(lenders) = scoped_native_lender_bindings(unit, binding, region) else {
        return false;
    };
    lenders
        .iter()
        .all(|lender| lender_is_transferable(package, unit, *lender, region))
}

fn nearest_scoped_access_region(root: &SyntaxNode, span: Span) -> Option<Span> {
    fn visit(node: &SyntaxNode, span: Span, nearest: &mut Option<Span>) {
        if matches!(
            node.kind,
            SyntaxKind::WithRegion | SyntaxKind::FunctionDeclaration
        ) && node.span.file == span.file
            && node.span.start <= span.start
            && node.span.end >= span.end
            && nearest
                .is_none_or(|current| node.span.end - node.span.start < current.end - current.start)
        {
            *nearest = Some(node.span);
        }
        for child in &node.children {
            visit(child, span, nearest);
        }
    }
    let mut nearest = None;
    visit(root, span, &mut nearest);
    nearest
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

fn lender_writes_before<'a>(
    unit: &SemanticUnit,
    node: &'a SyntaxNode,
    binding: Span,
    before: usize,
    values: &mut Vec<&'a SyntaxNode>,
) {
    if node.span.start >= before
        || matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        )
    {
        return;
    }
    if node.kind == SyntaxKind::Assignment
        && let [target, value] = node.children.as_slice()
        && target.kind == SyntaxKind::Name
        && visible_binding(unit, target).is_some_and(|actual| actual.span == binding)
    {
        values.push(value);
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && receiver.kind == SyntaxKind::Name
        && visible_binding(unit, receiver).is_some_and(|actual| {
            actual.span == binding && matches!(actual.value_type, ValueType::List(_))
        })
        && matches!(node_text(&unit.source, member), "append" | "set")
    {
        values.extend(
            arguments
                .children
                .iter()
                .filter_map(|argument| argument.children.last()),
        );
    }
    for child in &node.children {
        lender_writes_before(unit, child, binding, before, values);
    }
}

enum LocalNativeLender<'a> {
    Binding(&'a str, Span),
    Temporary(Span),
}

impl<'a> LocalNativeLender<'a> {
    fn into_diagnostic(self) -> (&'a str, Span) {
        match self {
            Self::Binding(name, span) => (name, span),
            Self::Temporary(span) => ("temporary value", span),
        }
    }
}

pub(in crate::semantics) fn native_expression_has_temporary_lender(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> bool {
    let Some(contract) = unit.containing_function(node.span) else {
        return false;
    };
    matches!(
        local_lender_name(
            package,
            unit,
            node,
            contract,
            contract.span,
            &mut BTreeSet::new(),
        ),
        Some(LocalNativeLender::Temporary(_))
    )
}

fn local_lender_name<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    contract: &FunctionContract,
    function_span: Span,
    visited: &mut BTreeSet<Span>,
) -> Option<LocalNativeLender<'a>> {
    if node.kind == SyntaxKind::GroupExpression && node.children.len() == 1 {
        return local_lender_name(
            package,
            unit,
            &node.children[0],
            contract,
            function_span,
            visited,
        );
    }
    // Literal native inputs have static backing, not a temporary owned lender.
    if !(matches!(node.kind, SyntaxKind::Name | SyntaxKind::Literal)
        || node.kind == SyntaxKind::UnaryExpression
            && matches!(
                unary_operator_text(unit, node).as_deref(),
                Some("ref" | "mutable ref")
            ))
        && infer_value_type(unit, node, &unit.typed_bindings)
            .ok()
            .flatten()
            .is_some_and(|value| {
                !matches!(
                    value,
                    ValueType::Reference(_)
                        | ValueType::MutableReference(_)
                        | ValueType::SharedReference(_)
                        | ValueType::InvocationScopedNative { .. }
                )
            })
    {
        return Some(LocalNativeLender::Temporary(node.span));
    }
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        let binding = visible_binding(unit, node)?;
        if contract.parameters.iter().any(|parameter| {
            parameter.name == name
                && matches!(
                    parameter.value_type,
                    Some(ValueType::Reference(_) | ValueType::MutableReference(_))
                )
        }) || unit
            .reference_provenance
            .get(&(binding.span.start, binding.span.end))
            .is_some_and(|provenance| {
                provenance.external_lender && provenance.lender_parameter.is_some()
            })
        {
            return None;
        }
        if !binding.scope.is_some_and(|scope| {
            scope.file == function_span.file
                && function_span.start <= scope.start
                && scope.end <= function_span.end
        }) {
            return None;
        }
        if invocation_scoped_region(&binding.value_type).is_none() {
            return Some(LocalNativeLender::Binding(name, binding.span));
        }
        if !visited.insert(binding.span) {
            return None;
        }
        let declaration = declaration_node_with_name_span(&unit.tree.root, binding.span)
            .or_else(|| find_node_by_span(&unit.tree.root, binding.span))?;
        if let Some(initializer) = binding_initializer(declaration)
            && let Some(lender) =
                callback_return_lender(package, unit, initializer, contract, function_span, visited)
        {
            return Some(lender);
        }
        // A returned aggregate or reassigned view also retains dependencies
        // introduced after its initializer. Resolve the receiver's binding,
        // not just its spelling, and stay within this function and use point.
        let function = find_node_by_span(&unit.tree.root, function_span)?;
        let mut values = Vec::new();
        for child in &function.children {
            lender_writes_before(unit, child, binding.span, node.span.start, &mut values);
        }
        return values.into_iter().find_map(|value| {
            callback_return_lender(package, unit, value, contract, function_span, visited)
        });
    }
    callback_return_lender(package, unit, node, contract, function_span, visited)
}

fn local_lender_in_scoped_expression<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    contract: &FunctionContract,
    function_span: Span,
) -> Option<LocalNativeLender<'a>> {
    callback_return_lender(
        package,
        unit,
        node,
        contract,
        function_span,
        &mut BTreeSet::new(),
    )
}

fn callback_return_lender<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    contract: &FunctionContract,
    function_span: Span,
    visited: &mut BTreeSet<Span>,
) -> Option<LocalNativeLender<'a>> {
    if node.kind == SyntaxKind::Name {
        return local_lender_name(package, unit, node, contract, function_span, visited);
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
    {
        if let Some(function) =
            projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
            && let crate::rust_interop::projection::ProjectedType::InvocationScoped {
                lifetimes: output_lifetimes,
                ..
            } = &function.result
        {
            if output_lifetimes
                .iter()
                .all(|lifetime| lifetime == "'static")
            {
                return None;
            }
            for (index, parameter) in function.parameters.iter().enumerate() {
                let mut lifetimes = Vec::new();
                projected_native_lifetimes(&parameter.ty, &mut lifetimes);
                if let Some(lifetime) = &parameter.borrow_lifetime {
                    lifetimes.push(lifetime.clone());
                }
                if lifetimes.iter().any(|lifetime| {
                    lifetime != "'static"
                        && output_lifetimes.contains(lifetime)
                        && !(lifetime == "'_"
                            && function.receiver_lifetime.as_deref() == Some("'_"))
                }) && let Some(argument) =
                    projected_argument_for_parameter(unit, &function.parameters, arguments, index)
                    && let Some(lender) =
                        local_lender_name(package, unit, argument, contract, function_span, visited)
                {
                    return Some(lender);
                }
            }
            if function.receiver.is_some()
                && callee.kind == SyntaxKind::MemberExpression
                && let Some(receiver) = callee.children.first()
                && let Some(lender) =
                    local_lender_name(package, unit, receiver, contract, function_span, visited)
            {
                return Some(lender);
            }
            return None;
        }
        // A native owned result breaks the input loans. Do not mistake an
        // owned conversion inside the returned graph for an escaping borrow.
        if projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
            .is_some()
        {
            return None;
        }
        if callee.kind == SyntaxKind::Name
            && let Some(function) =
                resolved_function_contract(unit, node_text(&unit.source, callee), callee.span.start)
            && matches!(
                function.return_type,
                Some(ValueType::InvocationScopedNative { .. })
            )
        {
            return arguments.children.iter().find_map(|argument| {
                local_lender_name(package, unit, argument, contract, function_span, visited)
            });
        }
    }
    node.children.iter().find_map(|child| {
        callback_return_lender(package, unit, child, contract, function_span, visited)
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
    if node.kind == SyntaxKind::ReturnStatement
        && let Some(contract) = unit.containing_function(node.span)
        && matches!(
            contract.return_type,
            Some(ValueType::InvocationScopedNative { .. })
        )
        && let Some((name, span)) =
            local_lender_in_scoped_expression(package, unit, node, contract, contract.span)
                .map(LocalNativeLender::into_diagnostic)
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
        && let Some(declaration) = declaration_node_with_name_span(&unit.tree.root, binding.span)
            .or_else(|| find_node_by_span(&unit.tree.root, binding.span))
        && let Some(initializer) = binding_initializer(declaration)
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
        // An implicit assignment introduces its binding only after the
        // initializer. Prefer that declaration before resolving a reassignment.
        let destination = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == node.span)
            .or_else(|| {
                node.children
                    .first()
                    .filter(|target| target.kind == SyntaxKind::Name)
                    .and_then(|target| {
                        let name = node_text(&unit.source, target);
                        unit.typed_bindings
                            .iter()
                            .filter(|binding| {
                                binding.name == name
                                    && binding.is_visible_at(unit.source.id(), target.span.start)
                            })
                            .max_by_key(|binding| binding.visible_from)
                    })
            });
        let destination_region = destination
            .and_then(|binding| binding.scope)
            .map(|span| (span.file, span.start, span.end));
        let destination_within_region = destination_region.is_some_and(|(file, start, end)| {
            matches!(region, InvocationScopedRegion::Bound((lender_file, lender_start, lender_end))
                if file == lender_file && start >= lender_start && end <= lender_end)
        });
        let region_local_function = local_span.is_some_and(|span| {
            unit.functions.iter().any(|function| {
                function.span == span
                    && matches!(
                        function.return_type,
                        Some(ValueType::InvocationScopedNative { .. })
                    )
            })
        });
        if (region != InvocationScopedRegion::Unbound && !destination_within_region)
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
        && let Some(binding) = unit
            .typed_bindings
            .iter()
            .filter(|binding| binding.is_visible_at(unit.source.id(), node.span.start))
            .max_by_key(|binding| binding.visible_from)
            .filter(|binding| {
                invocation_scoped_region(&binding.value_type).is_some()
                    && !scoped_native_transferable(package, unit, binding)
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

pub(in crate::semantics) fn projected_argument_for_parameter<'a>(
    unit: &SemanticUnit,
    parameters: &[crate::rust_interop::projection::ProjectedParameter],
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
                parameters
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
            let Some(value) =
                projected_argument_for_parameter(unit, &function.parameters, arguments, index)
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
            let callback_parameter = unit
                .projected_call_specializations
                .get(&span_key(node.span))
                .and_then(|selection| selection.projected_parameters.get(index))
                .unwrap_or(parameter);
            let mut callback = &callback_parameter.ty;
            while let crate::rust_interop::projection::ProjectedType::Optional(inner) = callback {
                callback = inner;
            }
            if matches!(
                callback,
                crate::rust_interop::projection::ProjectedType::Callback { .. }
            ) && let Some(contract) = callback_contract(package, unit, value)
            {
                validate_projected_callback_contract(
                    package,
                    unit,
                    value,
                    contract,
                    callback,
                    consumed_once,
                )?;
            }
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

fn validate_owned_callable_captures(
    package: &SemanticPackage,
    unit: &SemanticUnit,
) -> Result<(), SemanticFailure> {
    for contract in &unit.functions {
        if contract.captures.is_empty() {
            continue;
        }
        let Some(node) = find_node_by_span(&unit.tree.root, contract.span) else {
            continue;
        };
        // Borrowed-environment carriers have a distinct lexical runtime with
        // no Send bound. Consult the finalized callable type, not its syntax.
        if node.kind == SyntaxKind::AnonymousFunction
            && infer_value_type(unit, node, &unit.typed_bindings)?
                .as_ref()
                .is_some_and(|value| !value_type_is_owned_static(value))
        {
            continue;
        }
        if let Some((code, _)) = callback_capture_trait_failure(package, unit, contract, true, true)
        {
            let message = if code == "T0084" {
                "owned callable requires transferable captured values (`Send`)"
            } else {
                "owned shared callable requires shareable captured values (`Sync`)"
            };
            return Err(failure(&unit.source, code, message, contract.span));
        }
    }
    Ok(())
}

pub(in crate::semantics) fn validate_projected_callback_arguments(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        validate_owned_callable_captures(package, unit)?;
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
