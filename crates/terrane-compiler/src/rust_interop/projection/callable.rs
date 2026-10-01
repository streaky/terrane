use super::{
    AssocItemConstraintKind, BTreeMap, BTreeSet, Function, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, GenericParamDefKind, Generics, HashMap, Id, InvocationMode, Item, ItemEnum,
    ItemSummary, ProjectedDestinationParameter, ProjectedDestinationResult, ProjectedType,
    RustdocPath, Term, Type, WherePredicate, immediate_generic_input,
    project_invocation_scoped_type, project_type, projectable_interface_bound,
    render_resolved_path, render_rust_type, resolved_path_name, type_arguments,
};
pub(super) fn builtin_callable_mode(
    path: &RustdocPath,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<InvocationMode> {
    match resolved_path_name(path, paths).as_str() {
        "core::ops::function::FnOnce" => Some(InvocationMode::Consuming),
        "core::ops::function::FnMut" => Some(InvocationMode::Mutable),
        "core::ops::function::Fn" => Some(InvocationMode::Shared),
        _ => None,
    }
}
pub(super) fn is_builtin_clone(path: &RustdocPath, paths: &HashMap<Id, ItemSummary>) -> bool {
    resolved_path_name(path, paths) == "core::clone::Clone"
}
pub(super) fn is_builtin_marker_trait(
    path: &RustdocPath,
    paths: &HashMap<Id, ItemSummary>,
    name: &str,
) -> bool {
    resolved_path_name(path, paths) == format!("core::marker::{name}")
}
pub(super) fn future_output_from_generics(
    name: &str,
    declaration_generics: &Generics,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedType>, String> {
    let Some(parameter) = declaration_generics
        .params
        .iter()
        .find(|parameter| parameter.name == name)
    else {
        return Ok(None);
    };
    let bounds = generic_bounds_from_generics(parameter, declaration_generics);
    for bound in bounds {
        let Some((trait_, generic_params)) = trait_bound_name(&bound) else {
            continue;
        };
        if resolved_path_name(trait_, paths) != "core::future::future::Future" {
            continue;
        }
        if !generic_params.is_empty() {
            return Err("higher-ranked callback future is not projectable".to_owned());
        }
        let Some(GenericArgs::AngleBracketed { constraints, .. }) = trait_.args.as_deref() else {
            return Err("callback future has no concrete Output type".to_owned());
        };
        let output = constraints.iter().find_map(|constraint| {
            if constraint.name != "Output" {
                return None;
            }
            let AssocItemConstraintKind::Equality(Term::Type(output)) = &constraint.binding else {
                return None;
            };
            Some(output)
        });
        return output
            .map(|output| project_type(output, index, paths, generics))
            .transpose();
    }
    Ok(None)
}
pub(super) fn project_callback_generic(
    parameter: &GenericParamDef,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    known: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedType>, String> {
    let bounds = generic_bounds(parameter, function);
    let callback = bounds.iter().find_map(|bound| {
        let (trait_, generic_params) = trait_bound_name(bound)?;
        Some((
            trait_,
            generic_params,
            builtin_callable_mode(trait_, paths)?,
        ))
    });
    let Some((trait_, generic_params, kind)) = callback else {
        return Ok(None);
    };
    if generic_params.iter().any(|parameter| {
        !matches!(
            parameter.kind,
            rustdoc_types::GenericParamDefKind::Lifetime { .. }
        )
    }) {
        return Err(format!(
            "callback generic `{}` uses higher-ranked type or const parameters",
            parameter.name
        ));
    }
    let Some(GenericArgs::Parenthesized { inputs, output }) = trait_.args.as_deref() else {
        return Err(format!(
            "callback generic `{}` bound `{}` has no concrete call signature",
            parameter.name, trait_.path
        ));
    };
    let parameters = inputs
        .iter()
        .map(|input| project_type(input, index, paths, known))
        .collect::<Result<Vec<_>, _>>()?;
    let direct_output = output.clone().unwrap_or(Type::Tuple(Vec::new()));
    let (result, is_async) =
        if !generic_params.is_empty() && type_contains_lifetime_argument(&direct_output) {
            (
                project_invocation_scoped_type(&direct_output, index, paths, known)
                    .map_err(|reason| format!("callback result: {reason}"))?,
                false,
            )
        } else if let Type::Generic(future) = &direct_output {
            if let Some(output) = future_output(future, function, index, paths, known)? {
                (output, true)
            } else {
                (project_type(&direct_output, index, paths, known)?, false)
            }
        } else {
            (project_type(&direct_output, index, paths, known)?, false)
        };
    let has_trait = |name: &str| {
        bounds.iter().any(|bound| {
            trait_bound_name(bound)
                .is_some_and(|(trait_, _)| is_builtin_marker_trait(trait_, paths, name))
        })
    };
    let retained = bounds
        .iter()
        .any(|bound| matches!(bound, GenericBound::Outlives(name) if name == "'static" || name == "static"));
    Ok(Some(ProjectedType::Callback {
        rust_name: parameter.name.clone(),
        native_bound: None,
        native_method: None,
        native_result: None,
        native_substitutions: BTreeMap::new(),
        parameters,
        parameter_rust_types: inputs
            .iter()
            .map(|input| render_rust_type(input, index, paths, known))
            .collect::<Result<Vec<_>, _>>()?,
        parameter_borrows: inputs
            .iter()
            .map(|input| matches!(input, Type::BorrowedRef { .. }))
            .collect(),
        result: Box::new(result),
        parameters_destination_selected: false,
        invocation_mode: kind,
        is_async,
        retained,
        send: has_trait("Send"),
        sync: has_trait("Sync"),
    }))
}
pub(super) fn resolved_path_type_arguments(path: &RustdocPath) -> Vec<&Type> {
    let Some(GenericArgs::AngleBracketed { args, .. }) = path.args.as_deref() else {
        return Vec::new();
    };
    args.iter()
        .filter_map(|argument| match argument {
            GenericArg::Type(ty) => Some(ty),
            _ => None,
        })
        .collect()
}
fn merge_destination_selected_callable_candidates(
    candidates: &[CallableAdapterCandidate],
) -> Option<CallableAdapterCandidate> {
    let (
        ProjectedType::Callback {
            rust_name,
            result,
            native_bound,
            native_method,
            native_result,
            invocation_mode,
            is_async,
            retained,
            send,
            sync,
            ..
        },
        result_bounds,
        defaults,
    ) = candidates.first()?
    else {
        return None;
    };
    if !candidates
        .iter()
        .all(|(candidate, candidate_bounds, candidate_defaults)| {
            matches!(
                candidate,
                ProjectedType::Callback {
                    rust_name: candidate_rust_name,
                    result: candidate_result,
                    invocation_mode: candidate_invocation_mode,
                    is_async: candidate_is_async,
                    retained: candidate_retained,
                    send: candidate_send,
                    sync: candidate_sync,
                    ..
                } if candidate_rust_name == rust_name
                    && candidate_result == result
                    && candidate_invocation_mode == invocation_mode
                    && candidate_is_async == is_async
                    && candidate_retained == retained
                    && candidate_send == send
                    && candidate_sync == sync
            ) && candidate_bounds == result_bounds
                && candidate_defaults == defaults
        })
    {
        return None;
    }
    Some((
        ProjectedType::Callback {
            rust_name: rust_name.clone(),
            native_bound: native_bound.clone(),
            native_method: native_method.clone(),
            native_result: native_result.clone(),
            native_substitutions: defaults.clone(),
            parameters: Vec::new(),
            parameter_borrows: Vec::new(),
            parameter_rust_types: Vec::new(),
            parameters_destination_selected: true,
            result: result.clone(),
            invocation_mode: *invocation_mode,
            is_async: *is_async,
            retained: *retained,
            send: *send,
            sync: *sync,
        },
        result_bounds.clone(),
        defaults.clone(),
    ))
}
fn apply_terminal_alias_defaults(
    output: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    known: &BTreeMap<String, ProjectedType>,
) -> (
    BTreeMap<String, ProjectedType>,
    BTreeMap<String, ProjectedType>,
) {
    let Type::ResolvedPath(path) = output else {
        return (known.clone(), BTreeMap::new());
    };
    let Some(Item {
        inner: ItemEnum::TypeAlias(alias),
        ..
    }) = index.get(&path.id)
    else {
        return (known.clone(), BTreeMap::new());
    };
    let arguments = resolved_path_type_arguments(path);
    let parameters = alias.generics.params.iter().filter(|parameter| {
        matches!(
            parameter.kind,
            GenericParamDefKind::Type {
                is_synthetic: false,
                ..
            }
        )
    });
    let mut selected = known.clone();
    let mut defaults = BTreeMap::new();
    for (parameter, argument) in parameters.zip(arguments) {
        let GenericParamDefKind::Type {
            default: Some(default),
            ..
        } = &parameter.kind
        else {
            continue;
        };
        let Type::Generic(argument_name) = argument else {
            continue;
        };
        if let Ok(projected) = project_type(default, index, paths, &selected) {
            defaults.insert(argument_name.clone(), projected.clone());
            if matches!(
                selected.get(argument_name),
                None | Some(ProjectedType::Generic(_))
            ) {
                selected.insert(argument_name.clone(), projected);
            }
        }
    }
    (selected, defaults)
}
pub(super) fn has_supported_callable_trait_shape(
    declaration: &rustdoc_types::Trait,
    index: &HashMap<Id, Item>,
) -> bool {
    !declaration.is_auto
        && declaration.bounds.is_empty()
        && declaration.items.len() == 1
        && declaration.items.first().is_some_and(|item_id| {
            matches!(
                index.get(item_id).map(|item| &item.inner),
                Some(ItemEnum::Function(function)) if !function.has_body
            )
        })
}
fn has_callable_blanket(
    declaration: &rustdoc_types::Trait,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> bool {
    declaration.implementations.iter().any(|implementation_id| {
        let Some(Item {
            inner: ItemEnum::Impl(implementation),
            ..
        }) = index.get(implementation_id)
        else {
            return false;
        };
        let Type::Generic(self_name) = &implementation.for_ else {
            return false;
        };
        implementation
            .generics
            .params
            .iter()
            .find(|candidate| candidate.name == *self_name)
            .is_some_and(|self_parameter| {
                generic_bounds_from_generics(self_parameter, &implementation.generics)
                    .iter()
                    .any(|candidate| {
                        trait_bound_name(candidate).is_some_and(|(trait_, _)| {
                            builtin_callable_mode(trait_, paths).is_some()
                        })
                    })
            })
    })
}
pub(super) fn project_callable_adapter_generic(
    parameter: &GenericParamDef,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    known: &BTreeMap<String, ProjectedType>,
) -> Result<Option<CallableAdapterCandidate>, String> {
    project_callable_adapter_bounds(
        &parameter.name,
        &generic_bounds(parameter, function),
        index,
        paths,
        known,
    )
}
pub(super) fn is_callback_parameter(parameter: &GenericParamDef, function: &Function) -> bool {
    generic_bounds(parameter, function).iter().any(|bound| {
        trait_bound_name(bound).is_some_and(|(trait_, _)| {
            matches!(
                trait_.path.rsplit("::").next(),
                Some("Fn" | "FnMut" | "FnOnce")
            )
        })
    })
}
pub(super) fn is_callback_future_parameter(name: &str, function: &Function) -> bool {
    let future_bound = function
        .generics
        .params
        .iter()
        .find(|parameter| parameter.name == name)
        .is_some_and(|parameter| {
            generic_bounds(parameter, function).iter().any(|bound| {
                trait_bound_name(bound)
                    .is_some_and(|(trait_, _)| trait_.path.rsplit("::").next() == Some("Future"))
            })
        });
    if !future_bound {
        return false;
    }
    function.generics.params.iter().any(|parameter| {
        let bounds = generic_bounds(parameter, function);
        bounds.iter().any(|bound| {
            let Some((trait_, _)) = trait_bound_name(bound) else {
                return false;
            };
            if !matches!(
                trait_.path.rsplit("::").next(),
                Some("Fn" | "FnMut" | "FnOnce")
            ) {
                return false;
            }
            matches!(
                trait_.args.as_deref(),
                Some(GenericArgs::Parenthesized {
                    output: Some(Type::Generic(output)),
                    ..
                }) if output == name
            )
        })
    })
}
pub(super) fn type_mentions_generic(ty: &Type, generic: &str) -> bool {
    match ty {
        Type::Generic(name) => name == generic,
        Type::ResolvedPath(path) => path
            .args
            .as_deref()
            .is_some_and(|arguments| generic_args_mention(arguments, generic)),
        Type::BorrowedRef { type_, .. }
        | Type::RawPointer { type_, .. }
        | Type::Slice(type_)
        | Type::Array { type_, .. }
        | Type::Pat { type_, .. } => type_mentions_generic(type_, generic),
        Type::Tuple(types) => types
            .iter()
            .any(|item| type_mentions_generic(item, generic)),
        _ => false,
    }
}
fn generic_args_mention(arguments: &GenericArgs, generic: &str) -> bool {
    match arguments {
        GenericArgs::AngleBracketed { args, constraints } => {
            args.iter().any(|argument| {
                matches!(argument, GenericArg::Type(ty) if type_mentions_generic(ty, generic))
            }) || constraints.iter().any(|constraint| match &constraint.binding {
                AssocItemConstraintKind::Equality(Term::Type(ty)) => {
                    type_mentions_generic(ty, generic)
                }
                AssocItemConstraintKind::Constraint(bounds) => bounds
                    .iter()
                    .any(|bound| generic_bound_mentions(bound, generic)),
                AssocItemConstraintKind::Equality(Term::Constant(_)) => false,
            })
        }
        GenericArgs::Parenthesized { inputs, output } => {
            inputs
                .iter()
                .any(|input| type_mentions_generic(input, generic))
                || output
                    .as_ref()
                    .is_some_and(|output| type_mentions_generic(output, generic))
        }
        GenericArgs::ReturnTypeNotation => false,
    }
}
fn generic_bound_mentions(bound: &GenericBound, generic: &str) -> bool {
    matches!(
        bound,
        GenericBound::TraitBound { trait_, .. }
            if trait_
                .args
                .as_deref()
                .is_some_and(|arguments| generic_args_mention(arguments, generic))
    )
}
pub(super) fn render_generic_bounds(
    parameter: &GenericParamDef,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Vec<String>, String> {
    let mut rendered = Vec::new();
    if let GenericParamDefKind::Type { bounds, .. } = &parameter.kind {
        for bound in bounds {
            rendered.push(render_generic_bound(
                bound,
                &function.generics.params,
                index,
                paths,
                generics,
            )?);
        }
    }
    for predicate in &function.generics.where_predicates {
        let WherePredicate::BoundPredicate {
            type_: Type::Generic(name),
            bounds,
            generic_params,
        } = predicate
        else {
            continue;
        };
        if name == &parameter.name {
            let outer_generic_params = function
                .generics
                .params
                .iter()
                .chain(generic_params)
                .cloned()
                .collect::<Vec<_>>();
            for bound in bounds {
                rendered.push(render_generic_bound(
                    bound,
                    &outer_generic_params,
                    index,
                    paths,
                    generics,
                )?);
            }
        }
    }
    rendered.sort();
    rendered.dedup();
    Ok(rendered)
}
pub(super) fn render_generic_bound(
    bound: &GenericBound,
    outer_generic_params: &[GenericParamDef],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    match bound {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            modifier,
        } => {
            let mut quantified = outer_generic_params
                .iter()
                .chain(generic_params)
                .filter_map(|parameter| {
                    matches!(parameter.kind, GenericParamDefKind::Lifetime { .. })
                        .then_some(parameter.name.as_str())
                })
                .collect::<Vec<_>>();
            quantified.sort_unstable();
            quantified.dedup();
            let higher_ranked = if quantified.is_empty() {
                String::new()
            } else {
                format!("for<{}> ", quantified.join(", "))
            };
            let modifier = match modifier {
                rustdoc_types::TraitBoundModifier::None => "",
                rustdoc_types::TraitBoundModifier::Maybe => "?",
                rustdoc_types::TraitBoundModifier::MaybeConst => "~const ",
            };
            Ok(format!(
                "{higher_ranked}{modifier}{}",
                render_resolved_path(trait_, index, paths, generics)?
            ))
        }
        GenericBound::Outlives(lifetime) => Ok(lifetime.clone()),
        GenericBound::Use(_) => {
            Err("precise-capturing generic bound has no stable projection".to_owned())
        }
    }
}
#[expect(
    clippy::too_many_lines,
    reason = "callable blanket recipe selection keeps candidate unification and proof metadata together"
)]
pub(super) fn project_callable_adapter_bounds(
    parameter_name: &str,
    bounds: &[GenericBound],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    known: &BTreeMap<String, ProjectedType>,
) -> Result<Option<CallableAdapterCandidate>, String> {
    let mut candidates = Vec::new();
    for bound in bounds {
        let Some((requested_trait, requested_higher_ranked)) = trait_bound_name(bound) else {
            continue;
        };
        let Some(Item {
            inner: ItemEnum::Trait(declaration),
            ..
        }) = index.get(&requested_trait.id)
        else {
            continue;
        };
        let unsupported_shape = builtin_callable_mode(requested_trait, paths).is_none()
            && !has_supported_callable_trait_shape(declaration, index);
        if unsupported_shape && !has_callable_blanket(declaration, index, paths) {
            continue;
        }
        let native_function = declaration.items.iter().find_map(|item_id| {
            let item = index.get(item_id)?;
            let ItemEnum::Function(function) = &item.inner else {
                return None;
            };
            Some((item.name.clone()?, function))
        });
        let (callback_known, terminal_defaults) = native_function
            .as_ref()
            .and_then(|(_, function)| function.sig.output.as_ref())
            .map_or_else(
                || (known.clone(), BTreeMap::new()),
                |output| apply_terminal_alias_defaults(output, index, paths, known),
            );
        if unsupported_shape {
            let invocation_scoped = native_function
                .as_ref()
                .and_then(|(_, function)| function.sig.output.as_ref())
                .and_then(|output| {
                    project_invocation_scoped_type(output, index, paths, &callback_known).ok()
                })
                .is_some_and(|result| matches!(result, ProjectedType::InvocationScoped { .. }));
            if invocation_scoped {
                return Err(format!(
                    "generic input `{parameter_name}` uses unsupported callable trait `{}`",
                    requested_trait.path
                ));
            }
            continue;
        }
        let native_method = native_function.as_ref().map(|(name, _)| name.clone());
        let native_result = native_function
            .and_then(|(_, function)| function.sig.output.as_ref())
            .and_then(|output| render_rust_type(output, index, paths, &callback_known).ok());
        let native_trait = render_resolved_path(requested_trait, index, paths, &callback_known)?;
        let native_bound = if requested_higher_ranked.is_empty() {
            native_trait
        } else {
            format!(
                "for<{}> {native_trait}",
                requested_higher_ranked
                    .iter()
                    .map(|parameter| parameter.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        for implementation_id in &declaration.implementations {
            let Some(Item {
                inner: ItemEnum::Impl(implementation),
                ..
            }) = index.get(implementation_id)
            else {
                continue;
            };
            let Type::Generic(self_name) = &implementation.for_ else {
                continue;
            };
            let Some(implemented_trait) = &implementation.trait_ else {
                continue;
            };
            if implemented_trait.id != requested_trait.id
                && resolved_path_name(implemented_trait, paths)
                    != resolved_path_name(requested_trait, paths)
            {
                continue;
            }
            let mut implementation_types = callback_known.clone();
            for impl_parameter in &implementation.generics.params {
                if matches!(impl_parameter.kind, GenericParamDefKind::Type { .. }) {
                    implementation_types.insert(
                        impl_parameter.name.clone(),
                        ProjectedType::Generic(format!(
                            "TerraneAdapter{parameter_name}{}",
                            impl_parameter.name
                        )),
                    );
                }
            }
            for (implementation_argument, requested_argument) in
                resolved_path_type_arguments(implemented_trait)
                    .into_iter()
                    .zip(resolved_path_type_arguments(requested_trait))
            {
                if let Type::Generic(name) = implementation_argument {
                    let projected = match requested_argument {
                        Type::Generic(requested) => known
                            .get(requested)
                            .cloned()
                            .unwrap_or_else(|| ProjectedType::Generic(requested.clone())),
                        _ => project_type(requested_argument, index, paths, known)
                            .map_err(|reason| format!("callable trait argument: {reason}"))?,
                    };
                    implementation_types.insert(name.clone(), projected);
                }
            }
            let Some(self_parameter) = implementation
                .generics
                .params
                .iter()
                .find(|candidate| candidate.name == *self_name)
            else {
                continue;
            };
            let self_bounds =
                generic_bounds_from_generics(self_parameter, &implementation.generics);
            let Some((call_trait, call_higher_ranked, mut invocation_mode)) =
                self_bounds.iter().find_map(|candidate| {
                    let (trait_, generic_params) = trait_bound_name(candidate)?;
                    let invocation_mode = builtin_callable_mode(trait_, paths)?;
                    Some((trait_, generic_params, invocation_mode))
                })
            else {
                continue;
            };
            if invocation_mode == InvocationMode::Consuming
                && self_bounds.iter().any(|candidate| {
                    trait_bound_name(candidate)
                        .is_some_and(|(trait_, _)| is_builtin_clone(trait_, paths))
                })
            {
                invocation_mode = InvocationMode::Shared;
            }
            if !call_higher_ranked.is_empty() {
                continue;
            }
            let Some(GenericArgs::Parenthesized { inputs, output }) = call_trait.args.as_deref()
            else {
                continue;
            };
            let parameters = inputs
                .iter()
                .map(|input| {
                    project_type(input, index, paths, &implementation_types)
                        .map_err(|reason| format!("callable input: {reason}"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let parameter_rust_types = inputs
                .iter()
                .map(|input| render_rust_type(input, index, paths, &implementation_types))
                .collect::<Result<Vec<_>, _>>()?;
            let unit = Type::Tuple(Vec::new());
            let direct_output = output.as_ref().unwrap_or(&unit);
            let (result, is_async) = if let Type::Generic(future) = direct_output
                && let Some(output) = future_output_from_generics(
                    future,
                    &implementation.generics,
                    index,
                    paths,
                    &implementation_types,
                )? {
                (output, true)
            } else {
                (
                    project_invocation_scoped_type(
                        direct_output,
                        index,
                        paths,
                        &implementation_types,
                    )
                    .map_err(|reason| format!("callable result: {reason}"))?,
                    false,
                )
            };
            let result_bounds = if let ProjectedType::Generic(result_name) = &result {
                let Some(result_parameter) =
                    implementation.generics.params.iter().find(|candidate| {
                        implementation_types.get(&candidate.name)
                            == Some(&ProjectedType::Generic(result_name.clone()))
                    })
                else {
                    continue;
                };
                generic_bounds_from_generics(result_parameter, &implementation.generics)
                    .iter()
                    .map(|result_bound| {
                        render_generic_bound(
                            result_bound,
                            &implementation.generics.params,
                            index,
                            paths,
                            &implementation_types,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()?
            } else if matches!(result, ProjectedType::InvocationScoped { .. }) {
                Vec::new()
            } else {
                continue;
            };
            let exact_invocation_result = matches!(result, ProjectedType::InvocationScoped { .. })
                || (matches!(result, ProjectedType::Generic(_))
                    && !requested_higher_ranked.is_empty()
                    && native_result.as_ref().is_some_and(|result| {
                        requested_higher_ranked
                            .iter()
                            .any(|parameter| result.contains(&parameter.name))
                    }));
            let candidate = (
                ProjectedType::Callback {
                    rust_name: parameter_name.to_owned(),
                    native_bound: exact_invocation_result.then(|| native_bound.clone()),
                    native_method: exact_invocation_result
                        .then(|| native_method.clone())
                        .flatten(),
                    native_result: exact_invocation_result
                        .then(|| native_result.clone())
                        .flatten(),
                    native_substitutions: terminal_defaults.clone(),
                    parameters,
                    parameter_rust_types,
                    parameter_borrows: if exact_invocation_result {
                        inputs
                            .iter()
                            .map(|input| matches!(input, Type::BorrowedRef { .. }))
                            .collect()
                    } else {
                        vec![false; inputs.len()]
                    },
                    parameters_destination_selected: false,
                    result: Box::new(result),
                    invocation_mode,
                    is_async,
                    retained: false,
                    send: self_bounds.iter().any(|candidate| {
                        trait_bound_name(candidate).is_some_and(|(trait_, _)| {
                            is_builtin_marker_trait(trait_, paths, "Send")
                        })
                    }),
                    sync: self_bounds.iter().any(|candidate| {
                        trait_bound_name(candidate).is_some_and(|(trait_, _)| {
                            is_builtin_marker_trait(trait_, paths, "Sync")
                        })
                    }),
                },
                result_bounds,
                terminal_defaults.clone(),
            );
            if !candidates.contains(&candidate) {
                candidates.push(candidate);
            }
        }
    }
    match candidates.as_slice() {
        [] => Ok(None),
        [candidate] => Ok(Some(candidate.clone())),
        _ => merge_destination_selected_callable_candidates(&candidates)
            .map(Some)
            .ok_or_else(|| {
                format!(
                    "generic input `{parameter_name}` has incompatible callable blanket implementations"
                )
            }),
    }
}
pub(super) fn trait_bound_name(bound: &GenericBound) -> Option<(&RustdocPath, &[GenericParamDef])> {
    let GenericBound::TraitBound {
        trait_,
        generic_params,
        ..
    } = bound
    else {
        return None;
    };
    Some((trait_, generic_params))
}

pub(super) fn generic_bounds_from_generics(
    parameter: &GenericParamDef,
    generics: &Generics,
) -> Vec<GenericBound> {
    let mut bounds = match &parameter.kind {
        GenericParamDefKind::Type { bounds, .. } => bounds.clone(),
        _ => Vec::new(),
    };
    for predicate in &generics.where_predicates {
        let WherePredicate::BoundPredicate {
            type_: Type::Generic(name),
            bounds: predicate_bounds,
            generic_params,
        } = predicate
        else {
            continue;
        };
        if name == &parameter.name {
            if !generic_params.is_empty() {
                bounds.push(GenericBound::Outlives("__higher_ranked__".to_owned()));
            }
            bounds.extend(predicate_bounds.iter().cloned());
        }
    }
    bounds
}

pub(super) fn generic_bounds(
    parameter: &GenericParamDef,
    function: &Function,
) -> Vec<GenericBound> {
    generic_bounds_from_generics(parameter, &function.generics)
}

pub(super) fn concrete_into_future_output(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    outer_generics: &BTreeMap<String, ProjectedType>,
) -> Option<(Type, BTreeMap<String, ProjectedType>)> {
    let Type::ResolvedPath(path) = ty else {
        return None;
    };
    let Item {
        inner: ItemEnum::Struct(structure),
        ..
    } = index.get(&path.id)?
    else {
        return None;
    };
    let mut concrete_generics = outer_generics.clone();
    for (parameter, argument) in structure
        .generics
        .params
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .zip(type_arguments(ty))
    {
        let projected = match argument {
            Type::Generic(name) => outer_generics
                .get(name)
                .cloned()
                .unwrap_or_else(|| ProjectedType::Generic(name.clone())),
            _ => project_type(argument, index, paths, outer_generics).ok()?,
        };
        concrete_generics.insert(parameter.name.clone(), projected);
    }
    for implementation_id in &structure.impls {
        let Some(Item {
            inner: ItemEnum::Impl(implementation),
            ..
        }) = index.get(implementation_id)
        else {
            continue;
        };
        let trait_path = implementation
            .trait_
            .as_ref()
            .and_then(|trait_| paths.get(&trait_.id))
            .map(|summary| summary.path.join("::"));
        if trait_path.as_deref() != Some("core::future::into_future::IntoFuture") {
            continue;
        }
        for item_id in &implementation.items {
            let Some(Item {
                name: Some(name),
                inner:
                    ItemEnum::AssocType {
                        type_: Some(output),
                        ..
                    },
                ..
            }) = index.get(item_id)
            else {
                continue;
            };
            if name == "Output" {
                return Some((output.clone(), concrete_generics));
            }
        }
    }
    None
}

pub(super) fn future_output(
    name: &str,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedType>, String> {
    future_output_from_generics(name, &function.generics, index, paths, generics)
}

type CallableAdapterCandidate = (ProjectedType, Vec<String>, BTreeMap<String, ProjectedType>);

pub(super) fn rust_lifetimes(rust_type: &str) -> Vec<String> {
    let mut lifetimes = Vec::new();
    let bytes = rust_type.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'\'' {
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        while index < bytes.len() && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
        {
            index += 1;
        }
        if index > start + 1 {
            let lifetime = rust_type[start..index].to_owned();
            if lifetime != "'static" && !lifetimes.contains(&lifetime) {
                lifetimes.push(lifetime);
            }
        }
    }
    lifetimes
}

pub(super) fn type_contains_lifetime_argument(ty: &Type) -> bool {
    match ty {
        Type::ResolvedPath(path) => path
            .args
            .as_deref()
            .is_some_and(generic_args_contain_lifetime),
        Type::QualifiedPath {
            args, self_type, ..
        } => {
            args.as_deref().is_some_and(generic_args_contain_lifetime)
                || type_contains_lifetime_argument(self_type)
        }
        Type::BorrowedRef { type_, .. }
        | Type::RawPointer { type_, .. }
        | Type::Slice(type_)
        | Type::Array { type_, .. }
        | Type::Pat { type_, .. } => type_contains_lifetime_argument(type_),
        Type::Tuple(types) => types.iter().any(type_contains_lifetime_argument),
        _ => false,
    }
}

fn generic_args_contain_lifetime(arguments: &GenericArgs) -> bool {
    match arguments {
        GenericArgs::AngleBracketed { args, constraints } => {
            args.iter().any(|argument| match argument {
                GenericArg::Lifetime(_) => true,
                GenericArg::Type(ty) => type_contains_lifetime_argument(ty),
                GenericArg::Const(_) | GenericArg::Infer => false,
            }) || constraints
                .iter()
                .any(|constraint| match &constraint.binding {
                    AssocItemConstraintKind::Equality(Term::Type(ty)) => {
                        type_contains_lifetime_argument(ty)
                    }
                    AssocItemConstraintKind::Constraint(bounds) => bounds.iter().any(|bound| {
                        matches!(bound, GenericBound::Outlives(_))
                            || matches!(
                                bound,
                                GenericBound::TraitBound { generic_params, trait_, .. }
                                    if !generic_params.is_empty()
                                        || trait_.args.as_deref().is_some_and(
                                            generic_args_contain_lifetime
                                        )
                            )
                    }),
                    AssocItemConstraintKind::Equality(Term::Constant(_)) => false,
                })
        }
        GenericArgs::Parenthesized { inputs, output } => {
            inputs.iter().any(type_contains_lifetime_argument)
                || output.as_ref().is_some_and(type_contains_lifetime_argument)
        }
        GenericArgs::ReturnTypeNotation => false,
    }
}

pub(super) struct GenericMonomorphisations {
    pub(super) types: BTreeMap<String, ProjectedType>,
    pub(super) destination_result: Option<ProjectedDestinationResult>,
    pub(super) adapter_result_bounds: BTreeMap<String, Vec<String>>,
}

#[expect(
    clippy::too_many_lines,
    reason = "generic selection handles callbacks, destination results, and closed impls in order"
)]
pub(super) fn generic_monomorphisations(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    supplied: &BTreeMap<String, ProjectedType>,
) -> Result<GenericMonomorphisations, String> {
    let mut result = supplied.clone();
    let mut destination_parameters = Vec::new();
    let mut destination_bound_roots = BTreeSet::new();
    let mut adapter_result_bounds = BTreeMap::new();
    // Keep input-selected value generics open until a Terrane call site supplies concrete
    // argument and callback types. Immediate projectable interface inputs retain their
    // established source-class adapter contract.
    for parameter in &function.generics.params {
        if result.contains_key(&parameter.name)
            || is_callback_future_parameter(&parameter.name, function)
            || is_callback_parameter(parameter, function)
        {
            continue;
        }
        let mentioned_inputs = function
            .sig
            .inputs
            .iter()
            .filter(|(_, ty)| type_mentions_generic(ty, &parameter.name))
            .collect::<Vec<_>>();
        if mentioned_inputs.is_empty() {
            let output_selected = function
                .sig
                .output
                .as_ref()
                .is_some_and(|output| type_mentions_generic(output, &parameter.name));
            result.insert(
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            );
            if output_selected
                && matches!(
                    parameter.kind,
                    GenericParamDefKind::Type {
                        is_synthetic: false,
                        ..
                    }
                )
            {
                let rust_bounds =
                    render_generic_bounds(parameter, function, index, paths, &result)?;
                destination_bound_roots
                    .extend(rust_bounds.iter().flat_map(|bound| rust_bound_roots(bound)));
                destination_parameters.push(ProjectedDestinationParameter {
                    name: parameter.name.clone(),
                    rust_bounds,
                });
            }
            continue;
        }
        let immediate_projectable = mentioned_inputs.len() == 1
            && immediate_generic_input(&mentioned_inputs[0].1, &parameter.name)
            && projectable_interface_bound(&generic_bounds(parameter, function), index, paths)
                .is_ok();
        if !immediate_projectable {
            result.insert(
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            );
        }
    }
    // Resolve callable parameters before unrelated generic parameters. A callback whose
    // signature mentions an open `T` must decline; it must not inherit a guessed closed
    // implementation selected while monomorphising `T`.
    for parameter in &function.generics.params {
        if let Some(callback) =
            project_callback_generic(parameter, function, index, paths, &result)?
        {
            result.insert(parameter.name.clone(), callback);
        }
    }
    for parameter in &function.generics.params {
        let refinable = matches!(
            result.get(&parameter.name),
            Some(
                ProjectedType::Generic(_)
                    | ProjectedType::Callback {
                        native_bound: None,
                        ..
                    }
            )
        );
        if !refinable {
            continue;
        }
        if let Some((callback, result_bounds, defaults)) =
            project_callable_adapter_generic(parameter, function, index, paths, &result)?
        {
            let open_callback_result = match &callback {
                ProjectedType::Callback { result, .. } => match result.as_ref() {
                    ProjectedType::Generic(name) => Some(name.as_str()),
                    _ => None,
                },
                _ => None,
            };
            for (name, default) in defaults {
                if open_callback_result != Some(name.as_str())
                    && matches!(result.get(&name), None | Some(ProjectedType::Generic(_)))
                {
                    result.insert(name, default);
                }
            }
            result.insert(parameter.name.clone(), callback);
            adapter_result_bounds.insert(parameter.name.clone(), result_bounds);
        }
    }
    for parameter in &function.generics.params {
        if result.contains_key(&parameter.name) {
            continue;
        }
        if project_callback_generic(parameter, function, index, paths, &result)?.is_some() {
            unreachable!("callback parameters were resolved in the first pass");
        }
        if is_callback_future_parameter(&parameter.name, function) {
            result.insert(parameter.name.clone(), ProjectedType::None);
            continue;
        }
        let GenericParamDefKind::Type {
            bounds,
            is_synthetic,
            ..
        } = &parameter.kind
        else {
            continue;
        };
        if *is_synthetic {
            result.insert(
                parameter.name.clone(),
                ProjectedType::Opaque {
                    anonymous_chain: false,
                    bounds: generic_bounds(parameter, function)
                        .iter()
                        .map(|bound| render_generic_bound(bound, &[], index, paths, &result))
                        .collect::<Result<Vec<_>, _>>()?,
                },
            );
            continue;
        }
        let caller_chosen_inputs = function
            .sig
            .inputs
            .iter()
            .filter(|(_, ty)| type_mentions_generic(ty, &parameter.name))
            .collect::<Vec<_>>();
        let caller_chosen_input = !caller_chosen_inputs.is_empty()
            && !function
                .sig
                .output
                .as_ref()
                .is_some_and(|output| type_mentions_generic(output, &parameter.name));
        if caller_chosen_input
            && (caller_chosen_inputs.len() != 1
                || !caller_chosen_inputs
                    .iter()
                    .all(|(_, ty)| immediate_generic_input(ty, &parameter.name)))
        {
            return Err(format!(
                "generic bound `{}` must appear in exactly one immediate input",
                parameter.name
            ));
        }
        if caller_chosen_input {
            let all_bounds = generic_bounds(parameter, function);
            if let Ok(trait_) = projectable_interface_bound(&all_bounds, index, paths) {
                let rust_path = render_resolved_path(trait_, index, paths, &result)?;
                result.insert(
                    parameter.name.clone(),
                    ProjectedType::Foreign {
                        name: trait_
                            .path
                            .rsplit("::")
                            .next()
                            .unwrap_or(&trait_.path)
                            .to_owned(),
                        base_rust_path: rust_path.clone(),
                        rust_path,
                        arguments: Vec::new(),
                    },
                );
                continue;
            }
        }
        let Some(GenericBound::TraitBound { trait_, .. }) = bounds.first() else {
            return Err(format!("open generic `{}`", parameter.name));
        };
        let Some(Item {
            inner: ItemEnum::Trait(trait_definition),
            ..
        }) = index.get(&trait_.id)
        else {
            return Err(format!(
                "generic bound for `{}` has no closed impl set",
                parameter.name
            ));
        };
        let mut candidates = trait_definition
            .implementations
            .iter()
            .filter_map(|id| index.get(id))
            .filter_map(|item| match &item.inner {
                ItemEnum::Impl(implementation) => Some(&implementation.for_),
                _ => None,
            })
            .filter_map(|ty| project_type(ty, index, paths, &BTreeMap::new()).ok())
            .collect::<Vec<_>>();
        let mut unique = Vec::new();
        for candidate in candidates.drain(..) {
            if !unique.contains(&candidate) {
                unique.push(candidate);
            }
        }
        let direct = unique
            .iter()
            .filter(|ty| {
                matches!(
                    ty,
                    ProjectedType::String
                        | ProjectedType::Bytes
                        | ProjectedType::Bool
                        | ProjectedType::Int
                        | ProjectedType::Float
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        let selected = if direct.is_empty() { &unique } else { &direct };
        let [chosen] = selected.as_slice() else {
            return Err(if selected.is_empty() {
                format!(
                    "generic bound for `{}` has no Terrane-representable impl",
                    parameter.name
                )
            } else {
                format!(
                    "generic bound for `{}` has {} viable Terrane representations and requires a caller-chosen type",
                    parameter.name,
                    selected.len()
                )
            });
        };
        result.insert(parameter.name.clone(), chosen.clone());
    }
    Ok(GenericMonomorphisations {
        types: result,
        destination_result: (!destination_parameters.is_empty()).then(|| {
            ProjectedDestinationResult {
                parameters: destination_parameters,
                bound_roots: destination_bound_roots.into_iter().collect(),
            }
        }),
        adapter_result_bounds,
    })
}
pub(super) fn rust_bound_roots(bound: &str) -> BTreeSet<String> {
    bound
        .split(['<', '>', ',', '=', '+', '(', ')'])
        .filter_map(|fragment| {
            let fragment = fragment
                .trim()
                .trim_start_matches('?')
                .trim_start_matches("~const ");
            let (root, _) = fragment.split_once("::")?;
            let root = root.split_whitespace().last().unwrap_or(root);
            (!root.starts_with('\'')
                && !root.is_empty()
                && root
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_'))
            .then(|| root.to_owned())
        })
        .collect()
}
