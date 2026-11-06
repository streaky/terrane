use super::prelude::*;

fn read_only_lender<'a>(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    place: &'a SyntaxNode,
) -> Option<&'a SyntaxNode> {
    match unit.inferred_value_type(place) {
        Some(ValueType::SharedReference(_) | ValueType::MutableReference(_)) => return None,
        Some(ValueType::Reference(_)) => {
            let shared = super::ownership::expression_provenance(
                package,
                unit,
                place,
                &unit.reference_provenance,
                &unit.reference_return_lenders,
            )
            .is_some_and(|provenance| {
                package.reference_owner_uses_shared_storage(unit, provenance.owner)
            });
            return (!shared).then_some(place);
        }
        _ => {}
    }
    if matches!(
        place.kind,
        SyntaxKind::MemberExpression | SyntaxKind::IndexExpression | SyntaxKind::GroupExpression
    ) {
        return place
            .children
            .first()
            .and_then(|receiver| read_only_lender(package, unit, receiver));
    }
    None
}

pub(super) fn mutable_reference_creation_is_valid(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    place: &SyntaxNode,
) -> bool {
    match unit.inferred_value_type(place) {
        Some(ValueType::MutableReference(_)) => return true,
        Some(ValueType::Reference(_) | ValueType::SharedReference(_)) => return false,
        _ => {}
    }
    // Every projection must preserve authority: an owned outer object does not
    // grant exclusive access through one of its read-only or shared fields.
    if matches!(
        place.kind,
        SyntaxKind::GroupExpression | SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
    ) {
        return place
            .children
            .first()
            .is_some_and(|receiver| mutable_reference_creation_is_valid(package, unit, receiver));
    }
    if place.kind != SyntaxKind::Name {
        return false;
    }
    let name = node_text(&unit.source, place);
    unit.typed_bindings
        .iter()
        .filter(|binding| {
            binding.name == name && binding.is_visible_at(place.span.file, place.span.start)
        })
        .max_by_key(|binding| binding.visible_from)
        .is_some_and(|binding| {
            !super::bindings::declaration_is_constant_in_unit(unit, binding.span)
                && !package.reference_owner_uses_shared_storage(unit, binding.span)
        })
}

pub(super) fn validate_reference_writes(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("mutable ref")
            && let Some(place) = node.children.last()
            && !mutable_reference_creation_is_valid(package, unit, place)
        {
            return Err(SemanticFailure {
                source: unit.source.clone(),
                diagnostics: vec![Diagnostic::error(
                    "T0148",
                    "mutable reference requires a mutable owned place or an existing mutable reference",
                    place.span,
                )],
            });
        }
        let target = match node.kind {
            SyntaxKind::Assignment => node.children.first().filter(|target| {
                matches!(
                    target.kind,
                    SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
                ) || matches!(
                    unit.inferred_value_type(target),
                    Some(ValueType::MutableReference(_))
                )
            }),
            SyntaxKind::PostfixExpression => node.children.first(),
            SyntaxKind::CallExpression => node.children.first().and_then(|callee| {
                let (receiver, ty, member) = typed_member_call(unit, callee)?;
                (super::diagnostics::member_invocation_mode_at(
                    package, unit, receiver, &ty, &member,
                ) == InvocationMode::Mutable)
                    .then_some(receiver)
            }),
            _ => None,
        };
        if let Some(target) = target
            && let Some(lender) = read_only_lender(package, unit, target)
        {
            return Err(SemanticFailure {
                source: unit.source.clone(),
                diagnostics: vec![Diagnostic::error(
                    "T0147",
                    "cannot mutate a value through an ordinary read-only reference",
                    lender.span,
                ).with_help("use `mutable ref T` for explicit borrowed mutation, an owned value, or `shared ref T` for shared mutation")],
            });
        }
        for child in &node.children {
            visit(package, unit, child)?;
        }
        Ok(())
    }
    for unit in &package.units {
        visit(package, unit, &unit.tree.root)?;
    }
    Ok(())
}
