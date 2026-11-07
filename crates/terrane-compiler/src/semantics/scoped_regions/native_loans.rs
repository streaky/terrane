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
fn collect_native_view_origins(
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
    for child in &node.children {
        collect_native_view_origins(package, unit, child, lenders, places);
    }
}
pub(super) fn validate_function_native_lenders(
    package: &SemanticPackage,
    unit: &SemanticUnit,
) -> Result<(), SemanticFailure> {
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
        if unit.functions.iter().any(|contract| {
            contract.span == function_span
                && matches!(
                    contract.return_type,
                    Some(ValueType::InvocationScopedNative { .. })
                )
        }) {
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
            if call_returning_scoped_native(unit, initializer, rust_type).is_none() {
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
                return Err(failure(
                    &unit.source,
                    "T0119",
                    "cannot establish a local native value's source lifetime",
                    initializer.span,
                ));
            }
            let mut places = Vec::new();
            let Some(mut lenders) =
                scoped_native_lender_spans(package, unit, initializer, binding, &mut places, false)
            else {
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
    validate_native_return_escapes(unit, &unit.tree.root, None)?;
    Ok(())
}

fn validate_collection_native_lenders(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    flows: &mut BTreeMap<Span, liveness::Flow>,
) -> Result<(), SemanticFailure> {
    for binding in &unit.typed_bindings {
        let is_collection = matches!(
            binding.value_type,
            ValueType::List(_)
                | ValueType::Set(_)
                | ValueType::UnorderedSet(_)
                | ValueType::Map(_, _)
                | ValueType::UnorderedMap(_, _)
        );
        let Some((file, start, end)) =
            super::scoped_region(&binding.value_type).filter(|_| is_collection)
        else {
            continue;
        };
        let function_span = Span { file, start, end };
        let Some(function) = find_function(&unit.tree.root, function_span) else {
            continue;
        };
        let mut producers = Vec::new();
        collect_producers(unit, function, binding.span, &mut producers);
        for initializer in producers {
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
            let mut lender_set = BTreeSet::new();
            let mut places = Vec::new();
            collect_native_view_origins(package, unit, initializer, &mut lender_set, &mut places);
            super::collect_flow_lenders(unit, initializer.span, &mut lender_set);
            if lender_set.is_empty() {
                return Err(failure(
                    &unit.source,
                    "T0119",
                    "cannot establish a local native value's source lifetime",
                    initializer.span,
                ));
            }
            let mut lenders = lender_set.into_iter().collect::<Vec<_>>();
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
            validate_local_lender_destinations(unit, function_span, function, &lenders)?;
            validate_region_escape_node(package, unit, function_span, function, &mut lenders)?;
        }
    }
    Ok(())
}
