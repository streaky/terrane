use std::collections::BTreeMap;

use super::model::{ObjectIdentity, SemanticPackage, ValueType};
use super::objects::closed_projected_value_type;

fn projected_contract<'a>(
    projection: &'a crate::rust_interop::projection::Projection,
    namespace: &str,
    function: &super::model::FunctionContract,
) -> Option<&'a crate::rust_interop::projection::ProjectedFunction> {
    use crate::rust_interop::projection::ProjectedKind;
    if let Some(owner) = &function.owner_identity {
        let ProjectedKind::ForeignType {
            methods,
            static_methods,
            constructor,
            ..
        } = &projection.item(&owner.namespace, &owner.name)?.kind
        else {
            return None;
        };
        methods
            .iter()
            .chain(static_methods)
            .chain(constructor)
            .find(|projected| projected.name == function.name)
    } else {
        let ProjectedKind::Function(projected) = &projection.item(namespace, &function.name)?.kind
        else {
            return None;
        };
        Some(projected)
    }
}

/// Concrete signature aliases retain the ordinary projected class identity and its payload contract.
pub(super) fn normalize_contracts(package: &mut SemanticPackage) {
    let selections = package
        .units
        .iter()
        .flat_map(|unit| &unit.descriptors)
        .filter_map(|descriptor| {
            let identity = &descriptor.identity;
            let projected = package
                .projection
                .projected_type(&identity.namespace, &identity.name)?;
            let (namespace, name) = package.projection.owner_for_projected_type(&projected)?;
            package
                .projection
                .projected_constructor(&namespace, &name)?;
            let ValueType::Object(selected) = closed_projected_value_type(package, &projected)?
            else {
                return None;
            };
            selected.native_projection.as_ref()?;
            Some((identity.clone(), selected))
        })
        .collect::<BTreeMap<_, _>>();
    let mut signatures = Vec::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        for (function_index, function) in unit.functions.iter().enumerate() {
            let projected = projected_contract(&package.projection, &unit.namespace, function);
            let Some(projected) = projected else {
                continue;
            };
            for (parameter_index, (parameter, native)) in function
                .parameters
                .iter()
                .zip(&projected.parameters)
                .enumerate()
            {
                if let Some(current) = &parameter.value_type
                    && let Some(selected) = selected_signature_type(package, current, &native.ty)
                {
                    signatures.push((unit_index, function_index, Some(parameter_index), selected));
                }
            }
            if let Some(current) = &function.return_type
                && let Some(selected) = selected_signature_type(package, current, &projected.result)
            {
                signatures.push((unit_index, function_index, None, selected));
            }
        }
    }
    for unit in &mut package.units {
        for descriptor in &mut unit.descriptors {
            for field in &mut descriptor.fields {
                normalize_type(&mut field.value_type, &selections);
            }
        }
        for function in &mut unit.functions {
            for parameter in &mut function.parameters {
                if let Some(value_type) = &mut parameter.value_type {
                    normalize_type(value_type, &selections);
                }
            }
            if let Some(value_type) = &mut function.return_type {
                normalize_type(value_type, &selections);
            }
        }
    }
    for (unit_index, function_index, parameter_index, selected) in signatures {
        let function = &mut package.units[unit_index].functions[function_index];
        if let Some(parameter_index) = parameter_index {
            function.parameters[parameter_index].value_type = Some(selected);
        } else {
            function.return_type = Some(selected);
        }
    }
}

fn selected_signature_type(
    package: &SemanticPackage,
    current: &ValueType,
    projected: &crate::rust_interop::projection::ProjectedType,
) -> Option<ValueType> {
    use crate::rust_interop::projection::ProjectedType;
    match (current, projected) {
        (ValueType::Object(_), ProjectedType::Foreign { .. }) => {
            let selected = closed_projected_value_type(package, projected)?;
            let ValueType::Object(identity) = &selected else {
                return None;
            };
            (!identity.native_arguments.is_empty()).then_some(selected)
        }
        (ValueType::Optional(current), ProjectedType::Optional(native)) => Some(
            ValueType::Optional(Box::new(selected_signature_type(package, current, native)?)),
        ),
        (ValueType::List(current), ProjectedType::Sequence { item, .. }) => {
            Some(ValueType::List(super::model::ElementType::new(
                selected_signature_type(package, current.value_type_ref(), item)?,
            )))
        }
        _ => None,
    }
}

fn normalize_type(
    value_type: &mut ValueType,
    selections: &BTreeMap<ObjectIdentity, ObjectIdentity>,
) {
    match value_type {
        ValueType::Object(identity) => {
            if let Some(selected) = selections.get(identity) {
                identity.clone_from(selected);
            }
        }
        ValueType::Optional(inner) => normalize_type(inner, selections),
        ValueType::Reference(inner)
        | ValueType::SharedReference(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner) => {
            normalize_type(inner.value_type_mut(), selections);
        }
        ValueType::Map(key, value)
        | ValueType::UnorderedMap(key, value)
        | ValueType::Entry(key, value) => {
            normalize_type(key.value_type_mut(), selections);
            normalize_type(value.value_type_mut(), selections);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                normalize_type(parameter.value_type_mut(), selections);
            }
            normalize_type(result.value_type_mut(), selections);
        }
        _ => {}
    }
}
