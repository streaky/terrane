//! Compose proven callable recipes with native owning trait-object carriers.
//! The carrier owns the adapter, never the invocation's borrowed event input.
use super::{
    BTreeMap, GenericBound, HashMap, Id, InvocationMode, Item, ItemSummary, OwningCarrier,
    ProjectedType,
};

pub(super) fn project_owned_callback(
    dynamic: &rustdoc_types::DynTrait,
    carrier: OwningCarrier,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    known: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedType>, String> {
    if dynamic
        .lifetime
        .as_deref()
        .is_some_and(|name| name != "'static" && name != "static")
    {
        return Err("borrowed trait objects cannot cross an owning projected boundary".to_owned());
    }
    let bounds = dynamic
        .traits
        .iter()
        .map(|poly| GenericBound::TraitBound {
            trait_: poly.trait_.clone(),
            generic_params: poly.generic_params.clone(),
            modifier: rustdoc_types::TraitBoundModifier::None,
        })
        .collect::<Vec<_>>();
    let direct = super::callable::project_callback_bounds(
        "TerraneOwnedCallback",
        &bounds,
        None,
        index,
        paths,
        known,
        true,
    )?;
    let selected = match direct {
        Some(callback) => Some(callback),
        None => super::callable::project_callable_adapter_bounds(
            "TerraneOwnedCallback",
            &bounds,
            index,
            paths,
            known,
            true,
        )?
        .map(|(callback, _, _)| callback),
    };
    let Some(mut selected) = selected else {
        return Ok(None);
    };
    super::callable::validate_owning_trait_arguments(dynamic, index, paths, known)?;
    let ProjectedType::Callback {
        carrier: selected_carrier,
        rust_name,
        invocation_mode,
        retained,
        send,
        sync,
        parameter_borrows,
        parameter_rust_types,
        ..
    } = &mut selected
    else {
        unreachable!("callable recipes select callback metadata")
    };
    if carrier == OwningCarrier::Arc && *invocation_mode != InvocationMode::Shared {
        return Err("Arc callbacks require shared invocation; mutable or consuming invocation needs an exclusive owning carrier".to_owned());
    }
    *selected_carrier = Some(carrier);
    *retained = true;
    // A marker on the object or on the principal trait remains an independent
    // capture requirement. Blanket implementation requirements are already kept.
    for poly in &dynamic.traits {
        let principal = &poly.trait_;
        let inherited = index
            .get(&principal.id)
            .and_then(|item| match &item.inner {
                rustdoc_types::ItemEnum::Trait(declaration) => Some(declaration.bounds.as_slice()),
                _ => None,
            })
            .unwrap_or_default();
        for bound in std::iter::once(GenericBound::TraitBound {
            trait_: principal.clone(),
            generic_params: Vec::new(),
            modifier: rustdoc_types::TraitBoundModifier::None,
        })
        .chain(inherited.iter().cloned())
        {
            if let Some((trait_, _)) = super::trait_bound_name(&bound) {
                *send |= super::callable::is_builtin_marker_trait(trait_, paths, "Send");
                *sync |= super::callable::is_builtin_marker_trait(trait_, paths, "Sync");
            }
        }
    }
    *parameter_borrows = parameter_rust_types
        .iter()
        .map(|ty| ty.trim_start().starts_with('&'))
        .collect();
    let native_bounds = bounds
        .iter()
        .map(|bound| super::render_generic_bound(bound, &[], index, paths, known))
        .collect::<Result<Vec<_>, _>>()?
        .join(" + ");
    *rust_name = format!("{}<dyn {native_bounds}>", carrier.rust_constructor());
    super::callback_inputs::normalize(&mut selected);
    Ok(Some(selected))
}

/// A provider's aliases are available only after cross-crate enrichment. Publish
/// the same callback plan in every parameter signature before source emission;
/// do not leave checking and lowering to rediscover a foreign alias differently.
pub(super) fn canonicalize_parameters(projection: &mut super::Projection) {
    let mut replacements = Vec::new();
    for (dependency_index, dependency) in projection.dependencies.iter().enumerate() {
        for (item_index, item) in dependency.items.iter().enumerate() {
            for (function_index, function) in
                super::source_rendering::projected_item_functions(item)
                    .into_iter()
                    .enumerate()
            {
                for (parameter_index, parameter) in function.parameters.iter().enumerate() {
                    if let Some(
                        mut canonical @ ProjectedType::Callback {
                            carrier: Some(_), ..
                        },
                    ) = projection.canonical_native_alias(&parameter.ty)
                    {
                        super::callback_inputs::normalize(&mut canonical);
                        replacements.push((
                            dependency_index,
                            item_index,
                            function_index,
                            parameter_index,
                            canonical,
                        ));
                    }
                }
            }
        }
    }
    for (dependency, item, function, parameter, canonical) in replacements {
        let kind = &mut projection.dependencies[dependency].items[item].kind;
        let function = match kind {
            super::ProjectedKind::Function(function) | super::ProjectedKind::Macro(function) => {
                function
            }
            super::ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | super::ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => methods
                .iter_mut()
                .chain(static_methods)
                .nth(function)
                .expect("recorded native function"),
            super::ProjectedKind::Interface(interface) => &mut interface.methods[function].function,
        };
        function.parameters[parameter].ty = canonical;
    }
}
