use super::prelude::*;

pub(super) fn declarations(
    unit: &SemanticUnit,
    object: &SyntaxNode,
    kind: ObjectKind,
    visible: &BTreeMap<String, ScalarType>,
    visible_objects: &BTreeMap<String, ObjectIdentity>,
    generic_parameters: &mut Vec<GenericParameterContract>,
) -> Result<Vec<AssociatedTypeContract>, SemanticFailure> {
    let mut associated_types = Vec::new();
    let object_name =
        declaration_name(object, &unit.source).expect("object declaration has a name");
    let is_unsafe = kind == ObjectKind::Interface
        && object.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == "unsafe"
        });
    let owner = ObjectIdentity::new(&unit.namespace, &object_name).with_safety(is_unsafe);
    let Some(block) = object
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Block)
    else {
        return Ok(associated_types);
    };
    for association in block
        .children
        .iter()
        .filter(|child| child.kind == SyntaxKind::AssociatedTypeDeclaration)
    {
        let Some(type_name) = association
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
        else {
            continue;
        };
        let name = node_text(&unit.source, type_name).to_owned();
        if generic_parameters
            .iter()
            .any(|parameter| parameter.name == name)
        {
            return Err(failure(
                &unit.source,
                "T0130",
                format!("associated type `{name}` duplicates an explicit type parameter"),
                type_name.span,
            ));
        }
        if associated_types
            .iter()
            .any(|slot: &AssociatedTypeContract| slot.name == name)
        {
            return Err(failure(
                &unit.source,
                "T0130",
                format!("associated type `{name}` is declared more than once"),
                type_name.span,
            ));
        }
        let value_type = association
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::TypeExpression)
            .map(|ty| declared_value_type_with_visible_objects(unit, ty, visible, visible_objects))
            .transpose()?;
        match kind {
            ObjectKind::Interface if value_type.is_some() => {
                return Err(failure(
                    &unit.source,
                    "T0131",
                    "interfaces declare associated types without bindings",
                    association.span,
                ));
            }
            ObjectKind::Class if value_type.is_none() => {
                return Err(failure(
                    &unit.source,
                    "T0131",
                    "class associated types require a binding",
                    association.span,
                ));
            }
            ObjectKind::Interface => generic_parameters.push(GenericParameterContract {
                name: name.clone(),
                span: type_name.span,
                bound: None,
            }),
            ObjectKind::Class => {}
            _ => {
                return Err(failure(
                    &unit.source,
                    "T0131",
                    "associated types are supported only on classes and interfaces",
                    association.span,
                ));
            }
        }
        associated_types.push(AssociatedTypeContract {
            name,
            span: association.span,
            owner: owner.clone(),
            value_type,
        });
    }
    Ok(associated_types)
}

fn inherited_class_associated_types(
    base: &ObjectIdentity,
    classes: &BTreeMap<ObjectIdentity, DescriptorContract>,
    visiting: &mut BTreeSet<ObjectIdentity>,
) -> Vec<AssociatedTypeContract> {
    let identity = base.base();
    if !visiting.insert(identity.clone()) {
        return Vec::new();
    }
    let Some(descriptor) = classes.get(&identity) else {
        return Vec::new();
    };
    let mut slots = descriptor.base.as_ref().map_or_else(Vec::new, |parent| {
        inherited_class_associated_types(parent, classes, visiting)
    });
    slots.extend(descriptor.associated_types.iter().cloned());
    let substitutions = descriptor
        .generic_parameters
        .iter()
        .zip(&base.type_arguments)
        .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
        .collect::<BTreeMap<_, _>>();
    for slot in &mut slots {
        slot.value_type = slot
            .value_type
            .as_ref()
            .map(|value_type| substitute_value_type(value_type, &substitutions));
    }
    visiting.remove(&identity);
    slots
}

fn canonicalize_associated_values(package: &mut SemanticPackage) {
    let mut canonical_values = Vec::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        for (descriptor_index, descriptor) in unit.descriptors.iter().enumerate() {
            for (slot_index, slot) in descriptor.associated_types.iter().enumerate() {
                if let Some(value_type) = &slot.value_type {
                    let mut value_type = value_type.clone();
                    super::objects::canonicalize_native_value_type(package, &mut value_type);
                    canonical_values.push((unit_index, descriptor_index, slot_index, value_type));
                }
            }
        }
    }
    for (unit_index, descriptor_index, slot_index, value_type) in canonical_values {
        package.units[unit_index].descriptors[descriptor_index].associated_types[slot_index]
            .value_type = Some(value_type);
    }
}

fn class_descriptors(package: &SemanticPackage) -> BTreeMap<ObjectIdentity, DescriptorContract> {
    package
        .units
        .iter()
        .flat_map(|unit| {
            unit.descriptors
                .iter()
                .filter(|descriptor| descriptor.kind == ObjectKind::Class)
                .map(|descriptor| (descriptor.identity.base(), descriptor.clone()))
        })
        .collect()
}

fn interface_descriptors(
    package: &SemanticPackage,
) -> BTreeMap<ObjectIdentity, DescriptorContract> {
    package
        .units
        .iter()
        .flat_map(|unit| {
            unit.descriptors
                .iter()
                .filter(|descriptor| descriptor.kind == ObjectKind::Interface)
                .map(|descriptor| (descriptor.identity.base(), descriptor.clone()))
        })
        .collect()
}

fn inherit_class_bindings(
    package: &mut SemanticPackage,
    classes: &BTreeMap<ObjectIdentity, DescriptorContract>,
) -> Result<(), SemanticFailure> {
    for unit in &mut package.units {
        for class in unit
            .descriptors
            .iter_mut()
            .filter(|descriptor| descriptor.kind == ObjectKind::Class)
        {
            let inherited = class.base.as_ref().map_or_else(Vec::new, |base| {
                inherited_class_associated_types(base, classes, &mut BTreeSet::new())
            });
            for slot in inherited {
                if let Some(own) = class
                    .associated_types
                    .iter()
                    .find(|own| own.name == slot.name)
                {
                    if own.value_type != slot.value_type {
                        return Err(failure(
                            &unit.source,
                            "T0132",
                            format!(
                                "class `{}` has an associated type `{}` that conflicts with its inherited binding",
                                class.name, slot.name
                            ),
                            own.span,
                        ));
                    }
                } else {
                    class.associated_types.push(slot);
                }
            }
        }
    }
    Ok(())
}

fn close_one_class_interfaces(
    source: &SourceFile,
    class: &mut DescriptorContract,
    interfaces: &BTreeMap<ObjectIdentity, DescriptorContract>,
) -> Result<(), SemanticFailure> {
    let mut required_names = BTreeSet::new();
    for identity in &mut class.interfaces {
        let Some(interface) = interfaces.get(&identity.base()) else {
            continue;
        };
        if interface.associated_types.is_empty() {
            continue;
        }
        let source_generic_count =
            interface.generic_parameters.len() - interface.associated_types.len();
        let mut arguments = identity.type_arguments.clone();
        if arguments.is_empty()
            && let Some(application) = &identity.application
        {
            arguments.push(application.as_ref().clone());
        }
        if arguments.len() < source_generic_count {
            return Err(failure(
                source,
                "T0002",
                format!(
                    "interface `{}` requires {} explicit type argument{} before its associated type bindings",
                    interface.name,
                    source_generic_count,
                    if source_generic_count == 1 { "" } else { "s" }
                ),
                class.span,
            ));
        }
        if arguments.len() > source_generic_count {
            return Err(failure(
                source,
                "T0132",
                format!(
                    "associated types for `{}` are bound with `associated type Name = Type` in the class",
                    interface.name
                ),
                class.span,
            ));
        }
        for slot in &interface.associated_types {
            required_names.insert(slot.name.as_str());
            let Some(value_type) = class
                .associated_types
                .iter()
                .find(|binding| binding.name == slot.name)
                .and_then(|binding| binding.value_type.clone())
            else {
                return Err(failure(
                    source,
                    "T0132",
                    format!(
                        "class `{}` must bind associated type `{}` required by `{}`",
                        class.name, slot.name, interface.name
                    ),
                    class.span,
                ));
            };
            arguments.push(value_type);
        }
        *identity = identity.base().with_type_arguments(arguments);
    }
    if let Some(binding) = class.associated_types.iter().find(|binding| {
        binding.owner == class.identity && !required_names.contains(binding.name.as_str())
    }) {
        return Err(failure(
            source,
            "T0132",
            format!(
                "associated type `{}` is not required by an implemented interface",
                binding.name
            ),
            binding.span,
        ));
    }
    Ok(())
}

pub(super) fn close_class_interfaces(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    canonicalize_associated_values(package);
    let classes = class_descriptors(package);
    let interfaces = interface_descriptors(package);
    inherit_class_bindings(package, &classes)?;
    for unit in &mut package.units {
        let source = &unit.source;
        for class in unit
            .descriptors
            .iter_mut()
            .filter(|descriptor| descriptor.kind == ObjectKind::Class)
        {
            close_one_class_interfaces(source, class, &interfaces)?;
        }
    }
    Ok(())
}
