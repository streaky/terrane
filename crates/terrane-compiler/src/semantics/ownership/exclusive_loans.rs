use super::{
    BTreeMap, BTreeSet, InvocationMode, ReferenceAuthority, ReferenceProjection,
    ReferenceProvenance, SemanticPackage, SemanticUnit, Span, SyntaxKind, SyntaxNode, ValueType,
    argument_for_parameter, function_contract_for_call, node_text, typed_member_call,
};

pub(super) fn owner_place_path(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    owner: Span,
    provenances: &BTreeMap<(usize, usize), ReferenceProvenance>,
) -> Option<Vec<ReferenceProjection>> {
    match node.kind {
        SyntaxKind::Name => {
            let declaration = package
                .resolve_name_at(unit, node.span.start, node_text(&unit.source, node))?
                .declaration_span?;
            if declaration == owner {
                return Some(Vec::new());
            }
            let provenance = provenances.get(&(declaration.start, declaration.end))?;
            (provenance.owner == owner).then(|| provenance.path.clone())
        }
        SyntaxKind::GroupExpression => node
            .children
            .first()
            .and_then(|child| owner_place_path(package, unit, child, owner, provenances)),
        SyntaxKind::MemberExpression => {
            let [receiver, member] = node.children.as_slice() else {
                return None;
            };
            let mut path = owner_place_path(package, unit, receiver, owner, provenances)?;
            path.push(ReferenceProjection::Field(
                node_text(&unit.source, member).to_owned(),
            ));
            Some(path)
        }
        SyntaxKind::IndexExpression => {
            let [receiver, _] = node.children.as_slice() else {
                return None;
            };
            let mut path = owner_place_path(package, unit, receiver, owner, provenances)?;
            path.push(ReferenceProjection::Element);
            Some(path)
        }
        _ => None,
    }
}

pub(super) fn owner_paths_overlap(
    left: &[ReferenceProjection],
    right: &[ReferenceProjection],
) -> bool {
    for (left, right) in left.iter().zip(right) {
        match (left, right) {
            (ReferenceProjection::Field(left), ReferenceProjection::Field(right))
                if left != right =>
            {
                return false;
            }
            (left, right) if left != right => return true,
            _ => {}
        }
    }
    true
}

fn root_place_binding(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    mut node: &SyntaxNode,
) -> Option<Span> {
    loop {
        match node.kind {
            SyntaxKind::MemberExpression
            | SyntaxKind::IndexExpression
            | SyntaxKind::GroupExpression => node = node.children.first()?,
            SyntaxKind::Name => {
                return package
                    .resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
                    .and_then(|symbol| symbol.declaration_span);
            }
            _ => return None,
        }
    }
}

pub(super) fn reference_descends_from(
    mut reference: Span,
    ancestor: Span,
    provenances: &BTreeMap<(usize, usize), ReferenceProvenance>,
) -> bool {
    // A chain cannot contain more distinct parents than the provenance graph.
    for _ in 0..=provenances.len() {
        if reference == ancestor {
            return true;
        }
        let Some(parent) = provenances
            .get(&(reference.start, reference.end))
            .and_then(|provenance| provenance.parent_observer)
        else {
            return false;
        };
        reference = parent;
    }
    false
}

#[derive(Clone, Copy)]
pub(super) struct OwnerAccessWindow<'a> {
    pub(super) owner: Span,
    pub(super) borrowed_path: &'a [ReferenceProjection],
    pub(super) observer: Span,
    pub(super) provenances: &'a BTreeMap<(usize, usize), ReferenceProvenance>,
    pub(super) writes_only: bool,
    pub(super) start: usize,
    pub(super) end: usize,
}

pub(super) fn first_owner_access_between(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    window: OwnerAccessWindow<'_>,
) -> Option<Span> {
    if node.span.end <= window.start || node.span.start >= window.end {
        return None;
    }
    let mut end = window.end;
    let through_reference = |expression: &SyntaxNode| {
        root_place_binding(package, unit, expression)
            .is_some_and(|root| reference_descends_from(root, window.observer, window.provenances))
    };
    let overlaps_owner = |expression: &SyntaxNode| {
        !through_reference(expression)
            && owner_place_path(package, unit, expression, window.owner, window.provenances)
                .is_some_and(|path| owner_paths_overlap(window.borrowed_path, &path))
    };
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
    {
        if let Some((receiver, ty, member)) = typed_member_call(unit, callee) {
            if through_reference(receiver) {
                end = end.max(node.span.end);
            } else if super::super::diagnostics::member_invocation_mode_at(
                package, unit, receiver, &ty, &member,
            ) == InvocationMode::Mutable
                && overlaps_owner(receiver)
            {
                return Some(receiver.span);
            }
        }
        if let Some(contract) = function_contract_for_call(package, unit, callee) {
            let specialization = unit.projected_call_specializations.get(&(
                node.span.file,
                node.span.start,
                node.span.end,
            ));
            for (index, parameter) in contract.parameters.iter().enumerate() {
                let Some(argument) = argument_for_parameter(unit, arguments, contract, index)
                else {
                    continue;
                };
                let mutable = parameter.requires_mutable_reference()
                    || specialization
                        .and_then(|plan| plan.projected_parameters.get(index))
                        .is_some_and(|parameter| parameter.mutable_borrow);
                let borrowed =
                    mutable || matches!(parameter.value_type, Some(ValueType::Reference(_)));
                if borrowed && through_reference(argument) {
                    end = end.max(node.span.end);
                } else if mutable && overlaps_owner(argument) {
                    return Some(argument.span);
                }
            }
        }
    }
    if node.kind == SyntaxKind::Assignment
        && let Some(target) = node.children.first()
    {
        if through_reference(target) {
            end = end.max(node.span.end);
        } else if overlaps_owner(target) {
            return Some(target.span);
        }
        return node.children.iter().skip(1).find_map(|child| {
            first_owner_access_between(package, unit, child, OwnerAccessWindow { end, ..window })
        });
    }
    if let Some(path) = owner_place_path(package, unit, node, window.owner, window.provenances) {
        if through_reference(node) {
            return None;
        }
        return (!window.writes_only && owner_paths_overlap(window.borrowed_path, &path))
            .then_some(node.span);
    }
    let children = match node.kind {
        SyntaxKind::ForStatement
            if node.children.first().is_some_and(|target| {
                target.span.file == window.observer.file
                    && target.span.start <= window.observer.start
                    && window.observer.end <= target.span.end
            }) =>
        {
            // The collection expression acquires the iterator loan; it is not
            // an independent owner access after the element binding exists.
            &node.children[2..]
        }
        SyntaxKind::Argument => &node.children[node.children.len().saturating_sub(1)..],
        SyntaxKind::MemberExpression => &node.children[..node.children.len().min(1)],
        _ => &node.children,
    };
    children.iter().find_map(|child| {
        first_owner_access_between(package, unit, child, OwnerAccessWindow { end, ..window })
    })
}

pub(super) fn has_exclusive_lineage(
    provenance: &ReferenceProvenance,
    provenances: &BTreeMap<(usize, usize), ReferenceProvenance>,
) -> bool {
    let mut current = provenance;
    let mut visited = BTreeSet::new();
    loop {
        if current.authority == ReferenceAuthority::Exclusive {
            return true;
        }
        let Some(parent) = current.parent_observer else {
            return false;
        };
        if !visited.insert(parent) {
            return false;
        }
        let Some(parent) = provenances.get(&(parent.start, parent.end)) else {
            return false;
        };
        current = parent;
    }
}
