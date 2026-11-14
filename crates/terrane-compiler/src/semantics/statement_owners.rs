//! Statement-scoped loans of owning handles, never payload/guard lifetime extension.
use std::collections::{BTreeMap, BTreeSet};

use super::{
    expressions::infer_value_type,
    model::{
        EvaluationKind, ReferenceAuthority, SemanticPackage, SemanticUnit, StatementOwnerLoan,
        ValueType,
    },
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
fn ungroup(mut node: &SyntaxNode) -> &SyntaxNode {
    while node.kind == SyntaxKind::GroupExpression {
        let [inner] = node.children.as_slice() else {
            break;
        };
        node = inner;
    }
    node
}

struct Facts {
    barriers: Vec<Span>,
    captured: BTreeSet<Span>,
    shared_storage: BTreeMap<Span, bool>,
}

/// Scalar reads/operations establish a bounded continuation. Unknown callee,
/// conversion, native, transfer and suspension effects retain independent ownership.
fn bounded_continuation(unit: &SemanticUnit, value: &SyntaxNode, facts: &Facts) -> bool {
    if !matches!(
        infer_value_type(unit, value, &unit.typed_bindings),
        Ok(Some(ValueType::Scalar(_)))
    ) {
        return false;
    }
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
    unit.flow_read_bindings
        .range(lower..=upper)
        .all(|(span, reads)| {
            !contains(
                value.span,
                Span {
                    file: span.0,
                    start: span.1,
                    end: span.2,
                },
            ) || reads.iter().all(|read| {
                unit.typed_bindings
                    .iter()
                    .find(|binding| binding.span == *read)
                    .is_some_and(|binding| matches!(binding.value_type, ValueType::Scalar(_)))
            })
        })
}

fn stable_owner(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    owner: Span,
    region: Span,
    facts: &mut Facts,
) -> bool {
    let Some(binding) = unit
        .typed_bindings
        .iter()
        .find(|binding| binding.span == owner)
    else {
        return false;
    };
    let Some(scope) = binding.scope else {
        return false;
    };
    if !contains(scope, region)
        || binding.visible_from > region.start
        || unit.completion_capture_slots.contains(&owner)
        || facts.captured.contains(&owner)
    {
        return false;
    }
    let referent = match &binding.value_type {
        ValueType::SharedReference(item) => item.value_type_ref(),
        value @ ValueType::Object(_) => value,
        _ => return false,
    };
    // Native owners can add conversion/ABI effects not proved by a scalar source RHS.
    if referent.is_mutable_native_reference_target() {
        return false;
    }
    if !*facts
        .shared_storage
        .entry(owner)
        .or_insert_with(|| package.reference_owner_uses_shared_storage(unit, owner))
    {
        return false;
    }
    let Some(function) = unit.containing_function(region) else {
        return false;
    };
    if unit
        .containing_function(owner)
        .map(|contract| contract.span)
        != Some(function.span)
    {
        return false;
    }
    // Read-only lender metadata (including owned shared parameters) does not permit
    // replacing the handle. A live exclusive path still needs independent staging.
    !unit.reference_provenance.values().any(|provenance| {
        provenance.owner == owner
            && provenance.authority == ReferenceAuthority::Exclusive
            && provenance
                .lifetime_end
                .is_none_or(|end| end.end >= region.start)
    })
}

fn visit(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    facts: &mut Facts,
    loans: &mut BTreeMap<Key, StatementOwnerLoan>,
) {
    if node.kind == SyntaxKind::Assignment
        && let [target, value] = node.children.as_slice()
        && let target = ungroup(target)
        && target.kind == SyntaxKind::MemberExpression
        && let [receiver, _] = target.children.as_slice()
        && let name = ungroup(receiver)
        && name.kind == SyntaxKind::Name
        && bounded_continuation(unit, value, facts)
        && let Some(owner) = unit.flow_binding_ids.get(&key(name.span)).copied()
        && stable_owner(package, unit, owner, node.span, facts)
    {
        loans.insert(
            key(name.span),
            StatementOwnerLoan {
                owner,
                through: node.span,
            },
        );
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
        };
        let mut loans = BTreeMap::new();
        visit(package, unit, &unit.tree.root, &mut facts, &mut loans);
        package.units[index].statement_owner_loans = loans;
    }
}
