//! Captured reference loans follow their actual carrier scope. Named carriers
//! retain planned last-use cleanup; inline pipeline operands live through traversal.
use super::super::prelude::*;
use super::{
    FunctionKey, OwnerAccessWindow, borrow_origin, first_owner_access_between,
    infer_reference_return_lenders, last_callable_carrier_use, owner_paths_overlap,
    reference_descends_from, ungrouped_expression,
};
use std::cell::OnceCell;

pub(super) fn mutable_reference_captures<'a>(
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    closure: &SyntaxNode,
    contract: &'a FunctionContract,
    provenances: &'a BTreeMap<(usize, usize), ReferenceProvenance>,
) -> impl Iterator<Item = (Span, &'a ReferenceProvenance)> {
    let position = closure.span.start;
    contract.captures.iter().filter_map(move |capture| {
        let span = package
            .resolve_name_at(unit, position, capture)?
            .declaration_span?;
        let binding = unit.typed_bindings.iter().find(|binding| {
            binding.span == span && matches!(binding.value_type, ValueType::MutableReference(_))
        })?;
        provenances
            .get(&(binding.span.start, binding.span.end))
            .map(|provenance| (binding.span, provenance))
    })
}

fn cleanup_end(unit: &SemanticUnit, carrier: Span) -> Option<usize> {
    let recorded_end = |carrier| {
        unit.callable_drops_after
            .iter()
            .filter(|(_, carriers)| carriers.contains(&carrier))
            .map(|(boundary, _)| *boundary)
            .max_by_key(|boundary| boundary.2 - boundary.1)
            .map(|boundary| boundary.2)
    };
    recorded_end(carrier).or_else(|| {
        let last_use = last_callable_carrier_use(
            unit,
            &unit.tree.root,
            carrier,
            &mut BTreeSet::from([carrier]),
        )?;
        let final_carrier = unit
            .flow_binding_ids
            .get(&(last_use.file, last_use.start, last_use.end))
            .copied()
            .unwrap_or(carrier);
        recorded_end(final_carrier)
    })
}

fn capture_scope<'a>(
    unit: &'a SemanticUnit,
    node: &'a SyntaxNode,
    closure: Span,
) -> Option<(&'a SyntaxNode, usize)> {
    if node.span.file != closure.file
        || node.span.start > closure.start
        || node.span.end < closure.end
    {
        return None;
    }
    if let Some(scope) = node
        .children
        .iter()
        .find_map(|child| capture_scope(unit, child, closure))
    {
        return Some(scope);
    }
    if node.kind == SyntaxKind::PipelineExpression
        && node.children.iter().skip(1).any(|stage| {
            stage
                .children
                .get(1)
                .and_then(|arguments| arguments.children.first())
                .map(ungrouped_expression)
                .is_some_and(|operand| {
                    operand.kind == SyntaxKind::AnonymousFunction && operand.span == closure
                })
        })
    {
        return Some((node, node.span.end));
    }
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && node
            .children
            .last()
            .map(ungrouped_expression)
            .is_some_and(|value| {
                value.kind == SyntaxKind::AnonymousFunction && value.span == closure
            })
    {
        let carrier = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == node.span)?;
        return cleanup_end(unit, carrier.span).map(|end| (&unit.tree.root, end));
    }
    None
}

/// Forming a later capture is itself a loan, even if its body only observes.
/// Reborrowing through the active observer remains permitted.
pub(super) fn conflicts_with_window(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    closure: &SyntaxNode,
    window: OwnerAccessWindow<'_>,
) -> bool {
    let Some(contract) = super::super::bindings::callback_contract(package, unit, closure) else {
        return false;
    };
    if window.writes_only && contract.written_invocation_mode == InvocationMode::Shared {
        return false;
    }
    mutable_reference_captures(package, unit, closure, contract, window.provenances).any(
        |(capture, provenance)| {
            !reference_descends_from(capture, window.observer, window.provenances)
                && provenance.owner == window.owner
                && owner_paths_overlap(&provenance.path, window.borrowed_path)
        },
    )
}

pub(super) fn validate(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    return_lenders: &OnceCell<BTreeMap<FunctionKey, usize>>,
) -> Result<(), SemanticFailure> {
    for child in &node.children {
        validate(package, unit, child, return_lenders)?;
    }
    let Some(contract) = (node.kind == SyntaxKind::AnonymousFunction)
        .then(|| super::super::bindings::callback_contract(package, unit, node))
        .flatten()
        .filter(|contract| contract.written_invocation_mode != InvocationMode::Consuming)
    else {
        return Ok(());
    };
    let Some((root, end)) = capture_scope(unit, &unit.tree.root, node.span) else {
        return Ok(());
    };
    for (_, provenance) in
        mutable_reference_captures(package, unit, node, contract, &unit.reference_provenance)
    {
        let writes_only = contract.written_invocation_mode == InvocationMode::Shared;
        // The pipeline source has already been acquired, but its lender remains
        // part of the traversal. Earlier count/value observations do not.
        let source_conflict = (root.kind == SyntaxKind::PipelineExpression && !writes_only)
            .then(|| root.children.first())
            .flatten()
            .and_then(|source| {
                let source_loan = borrow_origin(
                    package,
                    unit,
                    source,
                    &unit.reference_provenance,
                    return_lenders.get_or_init(|| infer_reference_return_lenders(package)),
                )?;
                (source_loan.owner == provenance.owner
                    && owner_paths_overlap(&source_loan.path, &provenance.path))
                .then_some(source.span)
            });
        let conflict = source_conflict.or_else(|| {
            first_owner_access_between(
                package,
                unit,
                root,
                OwnerAccessWindow {
                    owner: provenance.owner,
                    borrowed_path: &provenance.path,
                    observer: node.span,
                    provenances: &unit.reference_provenance,
                    writes_only,
                    start: node.span.end,
                    end,
                },
            )
        });
        if let Some(conflict) = conflict {
            let owner_name = unit
                .typed_bindings
                .iter()
                .find(|binding| binding.span == provenance.owner)
                .map_or("<unknown>", |binding| binding.name.as_str());
            return Err(failure(
                &unit.source,
                "T0149",
                format!("cannot access owner `{owner_name}` while a captured reference is live"),
                conflict,
            ));
        }
    }
    Ok(())
}
