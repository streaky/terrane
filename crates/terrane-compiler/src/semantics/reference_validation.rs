use super::prelude::*;

fn read_only_lender<'a>(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    place: &'a SyntaxNode,
) -> Option<&'a SyntaxNode> {
    match unit.inferred_value_type(place) {
        Some(ValueType::SharedReference(_)) => return None,
        Some(ValueType::Reference(inner)) => {
            if inner.value_type_ref().is_mutable_native_reference_target() {
                return None;
            }
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

pub(super) fn validate_reference_writes(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        let target = match node.kind {
            SyntaxKind::Assignment => node.children.first().filter(|target| {
                matches!(
                    target.kind,
                    SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
                )
            }),
            SyntaxKind::PostfixExpression => node.children.first(),
            SyntaxKind::CallExpression => node.children.first().and_then(|callee| {
                let (receiver, ty, member) = typed_member_call(unit, callee)?;
                (member_invocation_mode(package, unit, &ty, &member) == InvocationMode::Mutable)
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
                ).with_help("ordinary source `ref T` borrows are read-only; mutate an owned value, or use an explicit `shared ref` owner for shared mutation")],
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
