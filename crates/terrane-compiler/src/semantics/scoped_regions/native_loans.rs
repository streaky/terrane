//! Native loan origins for local initializers, reassignments and transfers.
use super::super::prelude::*;
use super::{
    NativePlaceLoan, binding_identity_at, call_returning_scoped_native, declared_bindings_at_node,
    liveness, scoped_native_lender_spans, validate_lender_stability,
    validate_local_lender_destinations, validate_native_return_escapes,
    validate_region_escape_node,
};
fn collect_producers<'a>(
    unit: &SemanticUnit,
    node: &'a SyntaxNode,
    binding: Span,
    producers: &mut Vec<&'a SyntaxNode>,
) {
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && (declared_bindings_at_node(unit, node).any(|declared| declared.span == binding)
            || node
                .children
                .first()
                .and_then(|target| binding_identity_at(unit, target))
                == Some(binding))
        && let Some(value) = node.children.last()
    {
        producers.push(value);
    }
    for child in &node.children {
        collect_producers(unit, child, binding, producers);
    }
}
fn find_function(node: &SyntaxNode, function: Span) -> Option<&SyntaxNode> {
    if node.kind == SyntaxKind::FunctionDeclaration && node.span == function {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_function(child, function))
}
pub(super) fn collect_native_view_origins(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    lenders: &mut BTreeSet<Span>,
    places: &mut Vec<super::NativeLenderPlace>,
) {
    if node.kind == SyntaxKind::CallExpression
        && infer_value_type(unit, node, &unit.typed_bindings)
            .ok()
            .flatten()
            .is_some_and(|value_type| super::field_has_native_loan(package, &value_type))
        && let Some((owner, path)) = super::super::calls::call_reference_origin(package, unit, node)
    {
        lenders.insert(owner);
        let place = (owner, path);
        if !places.contains(&place) {
            places.push(place);
        }
    }
    if node.kind == SyntaxKind::Name
        && let Some(identity) = binding_identity_at(unit, node)
        && infer_value_type(unit, node, &unit.typed_bindings)
            .ok()
            .flatten()
            .is_some_and(|ty| super::field_has_native_loan(package, &ty))
    {
        let origins =
            unit.flow_read_bindings
                .get(&(node.span.file, node.span.start, node.span.end));
        for origin in origins
            .into_iter()
            .flatten()
            .copied()
            .chain(origins.is_none().then_some(identity))
        {
            let Some(initializer) = super::find_binding_initializer(&unit.tree.root, origin) else {
                continue;
            };
            if initializer.span.end <= node.span.start {
                collect_noncopy_flow_lenders(package, unit, initializer.span, lenders);
                collect_native_view_origins(package, unit, initializer, lenders, places);
            }
        }
        let mut appended = Vec::new();
        collect_append_producers(unit, &unit.tree.root, identity, &mut appended);
        for producer in appended {
            if producer.span.end <= node.span.start {
                collect_noncopy_flow_lenders(package, unit, producer.span, lenders);
                collect_native_view_origins(package, unit, producer, lenders, places);
            }
        }
    }
    for child in &node.children {
        collect_native_view_origins(package, unit, child, lenders, places);
    }
}
fn has_exact_scoped_result(unit: &SemanticUnit, function: Span) -> bool {
    unit.functions.iter().any(|contract| {
        contract.span == function
            && matches!(
                contract.return_type,
                Some(ValueType::InvocationScopedNative { .. })
            )
    })
}

pub(super) fn validate_function_native_lenders(
    package: &SemanticPackage,
    unit: &SemanticUnit,
) -> Result<(), SemanticFailure> {
    validate_native_return_escapes(package, unit, &unit.tree.root, None)?;
    let mut flows = BTreeMap::new();
    for binding in &unit.typed_bindings {
        let ValueType::InvocationScopedNative {
            concrete: true,
            expression_scoped: false,
            region: Some((file, start, end)),
            lifetimes,
            rust_type,
            ..
        } = &binding.value_type
        else {
            continue;
        };
        if !lifetimes.iter().any(|lifetime| lifetime != "'static") {
            continue;
        }
        let function_span = Span {
            file: *file,
            start: *start,
            end: *end,
        };
        // Exact invocation-scoped callback results already have a distinct
        // higher-ranked lender/escape contract. They are not new local-loan
        // return declarations.
        if has_exact_scoped_result(unit, function_span) {
            continue;
        }
        let Some(function) = find_function(&unit.tree.root, function_span) else {
            continue;
        };
        let mut producers = Vec::new();
        collect_producers(unit, function, binding.span, &mut producers);
        for initializer in producers {
            // Pure transfers inherit the original producer through reaching
            // values; they do not create a second loan with stale lenders.
            let mut places = Vec::new();
            let lenders = if call_returning_scoped_native(unit, initializer, rust_type).is_none() {
                let mut transferred = initializer;
                while matches!(
                    transferred.kind,
                    SyntaxKind::GroupExpression | SyntaxKind::UnaryExpression
                ) && let Some(value) = transferred.children.last()
                {
                    transferred = value;
                }
                if binding_identity_at(unit, transferred).is_some() {
                    continue;
                }
                super::super::calls::call_reference_origin(package, unit, transferred).map(
                    |(owner, path)| {
                        places.push((owner, path));
                        vec![owner]
                    },
                )
            } else {
                scoped_native_lender_spans(package, unit, initializer, binding, &mut places, false)
            };
            let Some(mut lenders) = lenders else {
                return Err(failure(
                    &unit.source,
                    "T0119",
                    "cannot establish a local native value's source lifetime",
                    initializer.span,
                ));
            };
            lenders.sort_unstable();
            lenders.dedup();
            let live_after = flows
                .entry(function_span)
                .or_insert_with(|| liveness::Flow::new(package, unit, function))
                .live_origins(initializer.span);
            validate_lender_stability(
                package,
                unit,
                function_span,
                function,
                None,
                &lenders,
                NativePlaceLoan {
                    places: &places,
                    live_after: Some(&live_after),
                },
            )?;
            // Escape traversal expands the set with dependent locals. Those are
            // not additional owners that every sibling destination must outlive.
            validate_local_lender_destinations(unit, function_span, function, &lenders)?;
            validate_region_escape_node(package, unit, function_span, function, &mut lenders)?;
        }
    }
    validate_collection_native_lenders(package, unit, &mut flows)?;
    Ok(())
}

fn collect_noncopy_flow_lenders(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    source: Span,
    lenders: &mut BTreeSet<Span>,
) {
    let mut flow_lenders = BTreeSet::new();
    super::collect_flow_lenders(unit, source, &mut flow_lenders);
    lenders.extend(flow_lenders.into_iter().filter(|origin| {
        unit.typed_bindings
            .iter()
            .find(|binding| binding.span == *origin)
            .is_none_or(|binding| {
                let ty = unit
                    .flow_binding_types
                    .get(origin)
                    .unwrap_or(&binding.value_type);
                !package.native_value_is_copyable(ty)
            })
    }));
}

fn collect_append_producers<'a>(
    unit: &SemanticUnit,
    node: &'a SyntaxNode,
    binding: Span,
    producers: &mut Vec<&'a SyntaxNode>,
) {
    if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && callee.kind == SyntaxKind::MemberExpression
        && let Some(member) = callee.children.last()
        && matches!(node_text(&unit.source, member), "append" | "set")
        && let Some(receiver) = callee.children.first()
        && binding_identity_at(unit, receiver) == Some(binding)
        && let Some(arguments) = node.children.get(1)
        && let Some(argument) = arguments
            .children
            .get(usize::from(node_text(&unit.source, member) != "append"))
    {
        producers.push(argument.children.last().unwrap_or(argument));
    }
    for child in &node.children {
        collect_append_producers(unit, child, binding, producers);
    }
}

fn validate_collection_native_lenders(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    flows: &mut BTreeMap<Span, liveness::Flow>,
) -> Result<(), SemanticFailure> {
    for binding in &unit.typed_bindings {
        // Written native-view aliases can be the result of a source helper
        // without carrying an InvocationScopedNative wrapper.
        if matches!(binding.value_type, ValueType::InvocationScopedNative { .. })
            || !super::field_has_native_loan(package, &binding.value_type)
        {
            continue;
        }
        let function_span = super::scoped_region(&binding.value_type)
            .map(|(file, start, end)| Span { file, start, end })
            .or(binding.scope)
            .filter(|scope| scope.file == binding.span.file);
        let Some(function_span) = function_span else {
            continue;
        };
        // Exact callback results retain their higher-ranked lender contract.
        if has_exact_scoped_result(unit, function_span) {
            continue;
        }
        let Some(function) = find_function(&unit.tree.root, function_span) else {
            continue;
        };
        let mut producers = Vec::new();
        collect_producers(unit, function, binding.span, &mut producers);
        let mut append_producers = Vec::new();
        collect_append_producers(unit, function, binding.span, &mut append_producers);
        for (initializer, appended) in producers
            .into_iter()
            .map(|producer| (producer, false))
            .chain(
                append_producers
                    .into_iter()
                    .map(|producer| (producer, true)),
            )
        {
            let mut transferred = initializer;
            while matches!(
                transferred.kind,
                SyntaxKind::GroupExpression | SyntaxKind::UnaryExpression
            ) && let Some(value) = transferred.children.last()
            {
                transferred = value;
            }
            if !appended && binding_identity_at(unit, transferred).is_some() {
                continue;
            }
            let mut lender_set = BTreeSet::new();
            let mut places = Vec::new();
            collect_native_view_origins(package, unit, initializer, &mut lender_set, &mut places);
            collect_noncopy_flow_lenders(package, unit, initializer.span, &mut lender_set);
            if lender_set.is_empty() {
                let empty_constructor = super::super::collections::empty_collection_identity(
                    unit,
                    initializer,
                    &unit.typed_bindings,
                )
                .is_some();
                if empty_constructor {
                    continue;
                }
            }
            if lender_set.is_empty() {
                return Err(failure(
                    &unit.source,
                    "T0119",
                    "cannot establish a local native value's source lifetime",
                    initializer.span,
                ));
            }
            let mut lenders = lender_set.into_iter().collect::<Vec<_>>();
            let origin = initializer.span;
            let live_after = flows
                .entry(function_span)
                .or_insert_with(|| liveness::Flow::new(package, unit, function))
                .live_origins(origin);
            validate_lender_stability(
                package,
                unit,
                function_span,
                function,
                None,
                &lenders,
                NativePlaceLoan {
                    places: &places,
                    live_after: Some(&live_after),
                },
            )?;
            validate_local_lender_destinations(unit, function_span, function, &lenders)?;
            validate_region_escape_node(package, unit, function_span, function, &mut lenders)?;
        }
    }
    Ok(())
}
