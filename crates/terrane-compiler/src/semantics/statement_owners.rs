//! Statement-scoped loans of owning handles, never payload/guard lifetime extension.
use std::collections::{BTreeMap, BTreeSet};

use super::{
    expressions::infer_value_type,
    model::{
        EvaluationKind, ReferenceAuthority, SemanticPackage, SemanticUnit, StatementOwnerLoan,
        ValueType,
    },
    types::ungrouped_expression,
};
use crate::{
    Span,
    syntax::{SyntaxKind, SyntaxNode},
};

type Key = (u32, usize, usize);
fn key(span: Span) -> Key {
    (span.file, span.start, span.end)
}
fn contains(outer: Span, inner: Span) -> bool {
    outer.file == inner.file && outer.start <= inner.start && inner.end <= outer.end
}
struct Facts {
    barriers: Vec<Span>,
    captured: BTreeSet<Span>,
    shared_storage: BTreeMap<Span, Option<usize>>,
    declaration_scalars: BTreeMap<Span, bool>,
}

/// Scalar reads/operations establish a bounded continuation. Unknown callee,
/// conversion, native, transfer and suspension effects retain independent ownership.
fn bounded_continuation(unit: &SemanticUnit, value: &SyntaxNode, facts: &mut Facts) -> bool {
    let start = facts
        .barriers
        .partition_point(|span| span.start < value.span.start);
    if facts.barriers[start..]
        .iter()
        .take_while(|span| span.start <= value.span.end)
        .any(|span| contains(value.span, *span))
    {
        return false;
    }
    let lower = (value.span.file, value.span.start, 0);
    let upper = (value.span.file, value.span.end, usize::MAX);
    let reads_are_scalar = unit
        .flow_read_bindings
        .range(lower..=upper)
        .all(|(span, _)| {
            !contains(
                value.span,
                Span {
                    file: span.0,
                    start: span.1,
                    end: span.2,
                },
            ) || unit.flow_binding_ids.get(span).is_some_and(|owner| {
                if let Some(value_type) = unit.flow_binding_types.get(owner) {
                    matches!(value_type, ValueType::Scalar(_))
                } else {
                    // Parameters do not need a flow-storage type override.
                    // Cache their declaration type using the resolved storage ID.
                    *facts.declaration_scalars.entry(*owner).or_insert_with(|| {
                        unit.typed_bindings
                            .iter()
                            .find(|binding| binding.span == *owner)
                            .is_some_and(|binding| {
                                matches!(binding.value_type, ValueType::Scalar(_))
                            })
                    })
                }
            })
        });
    reads_are_scalar
        && matches!(
            infer_value_type(unit, value, &unit.typed_bindings),
            Ok(Some(ValueType::Scalar(_)))
        )
}

fn stable_owner(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    owner: Span,
    region: Span,
    facts: &mut Facts,
) -> Option<usize> {
    // Cache the physical binding lookup once per candidate owner. The completed
    // semantic unit keeps this index stable through lowering.
    let index = (*facts.shared_storage.entry(owner).or_insert_with(|| {
        unit.typed_bindings
            .iter()
            .position(|binding| binding.span == owner)
            .filter(|_| package.reference_owner_uses_shared_storage(unit, owner))
    }))?;
    let binding = &unit.typed_bindings[index];
    let scope = binding.scope?;
    if !contains(scope, region)
        || binding.visible_from > region.start
        || unit.completion_capture_slots.contains(&owner)
        || facts.captured.contains(&owner)
    {
        return None;
    }
    let referent = match &binding.value_type {
        ValueType::SharedReference(item) => item.value_type_ref(),
        value @ ValueType::Object(_) => value,
        _ => return None,
    };
    // Native owners can add conversion/ABI effects not proved by a scalar source RHS.
    if referent.is_mutable_native_reference_target() {
        return None;
    }
    let function = unit.containing_function(region)?;
    if unit
        .containing_function(owner)
        .map(|contract| contract.span)
        != Some(function.span)
    {
        return None;
    }
    // Read-only lender metadata (including owned shared parameters) does not permit
    // replacing the handle. A live exclusive path still needs independent staging.
    (!unit.reference_provenance.values().any(|provenance| {
        provenance.owner == owner
            && provenance.authority == ReferenceAuthority::Exclusive
            && provenance
                .lifetime_end
                .is_none_or(|end| end.end >= region.start)
    }))
    .then_some(index)
}

fn visit(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    facts: &mut Facts,
    loans: &mut BTreeMap<Key, StatementOwnerLoan>,
) {
    if node.kind == SyntaxKind::Assignment
        && let Some(value) = node.children.last()
        && let Some(target) = node.assignment_target()
        && let target = ungrouped_expression(target)
        && target.kind == SyntaxKind::MemberExpression
        && let Some(receiver) = target.children.first()
        && let name = ungrouped_expression(receiver)
        && name.kind == SyntaxKind::Name
        && let Some(owner) = unit.flow_binding_ids.get(&key(name.span)).copied()
        && let Some(binding_index) = stable_owner(package, unit, owner, node.span, facts)
        && bounded_continuation(unit, value, facts)
    {
        loans.insert(key(name.span), StatementOwnerLoan { binding_index });
    }
    for child in &node.children {
        visit(package, unit, child, facts, loans);
    }
}

pub(super) fn record(package: &mut SemanticPackage) {
    for index in 0..package.units.len() {
        let unit = &package.units[index];
        let mut barriers: Vec<_> = unit
            .evaluation_steps
            .iter()
            .filter(|step| step.kind != EvaluationKind::ShortCircuitRhs)
            .map(|step| step.span)
            .collect();
        barriers.sort_unstable_by_key(|span| (span.start, span.end));
        let captured = unit
            .local_functions()
            .flat_map(|function| {
                function.captures.iter().filter_map(|name| {
                    package
                        .resolve_name_at(unit, function.span.start, name)
                        .and_then(|symbol| symbol.declaration_span)
                })
            })
            .collect();
        let mut facts = Facts {
            barriers,
            captured,
            shared_storage: BTreeMap::new(),
            declaration_scalars: BTreeMap::new(),
        };
        let mut loans = BTreeMap::new();
        visit(package, unit, &unit.tree.root, &mut facts, &mut loans);
        package.units[index].statement_owner_loans = loans;
    }
}
#[cfg(test)]
mod tests;
