//! Reject inline source-class layouts that cannot have a finite Rust representation.
use super::capabilities::walk_value_type;
use super::prelude::*;

fn application_size(identity: &ObjectIdentity) -> usize {
    let mut size = 0;
    for argument in identity.type_arguments.iter().chain(
        identity
            .application
            .iter()
            .map(Box::as_ref)
            .take(usize::from(identity.type_arguments.is_empty())),
    ) {
        walk_value_type(argument, &mut |_| size += 1);
    }
    size
}

fn inline_cycle(
    package: &SemanticPackage,
    value: &ValueType,
    span: Span,
    active: &mut Vec<(ObjectIdentity, usize)>,
    complete: &mut BTreeSet<ObjectIdentity>,
) -> Option<(Span, String)> {
    match value {
        ValueType::Optional(inner) => inline_cycle(package, inner, span, active, complete),
        ValueType::Union(arms) => arms
            .iter()
            .find_map(|arm| inline_cycle(package, arm, span, active, complete)),
        ValueType::Object(identity) => {
            let descriptor = package
                .units
                .iter()
                .flat_map(|unit| &unit.descriptors)
                .find(|candidate| {
                    candidate.kind == ObjectKind::Class
                        && candidate.identity.base() == identity.base()
                        && candidate.identity.native_projection.is_none()
                        && package
                            .projection
                            .item(&identity.namespace, &identity.name)
                            .is_none()
                })?;
            if complete.contains(identity) {
                return None;
            }
            let size = application_size(identity);
            // Repeated generic wrappers such as Box<Box<int>> are finite when
            // their arguments strictly shrink. Re-entering a declaration with
            // equal or larger arguments cannot terminate an inline layout.
            if active.iter().any(|(owner, previous_size)| {
                owner.base() == identity.base() && size >= *previous_size
            }) {
                return Some((span, descriptor.name.clone()));
            }
            active.push((identity.clone(), size));
            let substitutions = descriptor
                .generic_parameters
                .iter()
                .zip(
                    identity.type_arguments.iter().chain(
                        identity
                            .application
                            .iter()
                            .map(Box::as_ref)
                            .take(usize::from(identity.type_arguments.is_empty())),
                    ),
                )
                .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
                .collect();
            for effective in effective_object_fields(package, descriptor)
                .into_iter()
                .filter(|field| !field.is_static)
            {
                let field_type = substitute_value_type(&effective.value_type, &substitutions);
                if let Some(cycle) =
                    inline_cycle(package, &field_type, effective.span, active, complete)
                {
                    return Some(cycle);
                }
            }
            active.pop();
            complete.insert(identity.clone());
            None
        }
        // Collections, references and callable carriers provide indirection.
        _ => None,
    }
}

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    let mut complete = BTreeSet::new();
    for descriptor in package
        .units
        .iter()
        .flat_map(|unit| &unit.descriptors)
        .filter(|descriptor| descriptor.kind == ObjectKind::Class)
    {
        let identity = descriptor.identity.clone().with_type_arguments(
            descriptor
                .generic_parameters
                .iter()
                .map(|parameter| ValueType::TypeParameter(parameter.name.clone()))
                .collect(),
        );
        if let Some((span, name)) = inline_cycle(
            package,
            &ValueType::Object(identity),
            descriptor.span,
            &mut Vec::new(),
            &mut complete,
        ) {
            let unit = package
                .units
                .iter()
                .find(|unit| unit.source.id() == span.file)
                .expect("inline field retains its source unit");
            return Err(failure(
                &unit.source,
                "T0140",
                format!(
                    "class `{name}` has an infinitely sized inline recursive field; use a collection or another supported indirect representation"
                ),
                span,
            ));
        }
    }
    Ok(())
}
