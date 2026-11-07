//! Written nominal applications must prove the same bounds as callable selections.
use super::capabilities::walk_value_type;
use super::prelude::*;

fn validate_annotation(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    annotation: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let aliases = visible_descriptor_aliases(
        &unit.descriptor_aliases,
        unit.source.id(),
        annotation.span.start,
    );
    let value = declared_value_type(unit, annotation, &aliases)?;
    let mut invalid = None;
    walk_value_type(&value, &mut |value| {
        if invalid.is_some() {
            return;
        }
        let ValueType::Object(identity) = value else {
            return;
        };
        if identity.native_projection.is_some()
            || package
                .projection
                .item(&identity.namespace, &identity.name)
                .is_some()
        {
            return;
        }
        let Some(descriptor) = unit
            .descriptors
            .iter()
            .find(|descriptor| descriptor.identity.base() == identity.base())
        else {
            return;
        };
        if !descriptor
            .generic_parameters
            .iter()
            .any(|parameter| parameter.bound.is_some())
        {
            return;
        }
        let substitutions = descriptor
            .generic_parameters
            .iter()
            .zip(&identity.type_arguments)
            .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
            .collect();
        if !super::generics::generic_bounds_satisfied(
            unit,
            &descriptor.generic_parameters,
            &substitutions,
            annotation.span.start,
        ) {
            invalid = Some(descriptor.name.clone());
        }
    });
    if let Some(name) = invalid {
        return Err(failure(
            &unit.source,
            "T0012",
            format!("written type arguments do not satisfy the bounds of `{name}`"),
            annotation.span,
        ));
    }
    Ok(())
}

fn visit(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    for child in &node.children {
        if child.kind == SyntaxKind::TypeExpression {
            if matches!(
                node.kind,
                SyntaxKind::Binding
                    | SyntaxKind::Assignment
                    | SyntaxKind::Parameter
                    | SyntaxKind::FunctionDeclaration
                    | SyntaxKind::AnonymousFunction
            ) {
                validate_annotation(package, unit, child)?;
            }
        } else {
            visit(package, unit, child)?;
        }
    }
    Ok(())
}

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        visit(package, unit, &unit.tree.root)?;
    }
    Ok(())
}
