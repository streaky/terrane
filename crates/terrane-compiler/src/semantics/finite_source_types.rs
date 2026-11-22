//! Reject inline source-class layouts that cannot have a finite Rust representation.
use super::capabilities::walk_value_type;
use super::prelude::*;

type ClassKey<'a> = (&'a str, &'a str, bool);

fn class_key(identity: &ObjectIdentity) -> ClassKey<'_> {
    (&identity.namespace, &identity.name, identity.is_unsafe)
}

struct Layouts<'a> {
    package: &'a SemanticPackage,
    classes: BTreeMap<ClassKey<'a>, &'a DescriptorContract>,
}

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
    layouts: &Layouts<'_>,
    value: &ValueType,
    span: Span,
    active: &mut Vec<(ObjectIdentity, usize)>,
    complete: &mut BTreeSet<ObjectIdentity>,
) -> Option<(Span, String)> {
    match value {
        ValueType::Optional(inner) => inline_cycle(layouts, inner, span, active, complete),
        ValueType::Union(arms) => arms
            .iter()
            .find_map(|arm| inline_cycle(layouts, arm, span, active, complete)),
        ValueType::Object(identity) => {
            let descriptor = layouts.classes.get(&class_key(identity))?;
            if complete.contains(identity) {
                return None;
            }
            let size = application_size(identity);
            // Repeated generic wrappers such as Box<Box<int>> are finite when
            // their arguments strictly shrink. Re-entering a declaration with
            // equal or larger arguments cannot terminate an inline layout.
            if active.iter().any(|(owner, previous_size)| {
                class_key(owner) == class_key(identity) && size >= *previous_size
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
            for effective in effective_object_fields(layouts.package, descriptor)
                .into_iter()
                .filter(|field| !field.is_static)
            {
                let field_type = substitute_value_type(&effective.value_type, &substitutions);
                if let Some(cycle) =
                    inline_cycle(layouts, &field_type, effective.span, active, complete)
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
    let roots = || {
        package
            .units
            .iter()
            .flat_map(|unit| &unit.descriptors)
            .filter(|descriptor| descriptor.kind == ObjectKind::Class)
    };
    let mut classes = BTreeMap::new();
    for descriptor in roots().filter(|descriptor| descriptor.identity.native_projection.is_none()) {
        let key = class_key(&descriptor.identity);
        classes.entry(key).or_insert(descriptor);
    }
    classes.retain(|&(namespace, name, _), _| package.projection.item(namespace, name).is_none());
    let layouts = Layouts { package, classes };
    let mut complete = BTreeSet::new();
    for descriptor in roots() {
        let identity = descriptor.identity.clone().with_type_arguments(
            descriptor
                .generic_parameters
                .iter()
                .map(|parameter| ValueType::TypeParameter(parameter.name.clone()))
                .collect(),
        );
        if let Some((span, name)) = inline_cycle(
            &layouts,
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
