use super::{
    AssocItemConstraintKind, BTreeMap, CallableAdapterCandidate, Function, GenericArg, GenericArgs,
    GenericBound, GenericParamDef, GenericParamDefKind, Generics, HashMap, Id, InvocationMode,
    Item, ItemEnum, ItemSummary, ProjectedType, RustdocPath, Term, Type, WherePredicate,
    future_output, generic_bounds, generic_bounds_from_generics, project_invocation_scoped_type,
    project_type, render_resolved_path, render_rust_type, resolved_path_name, trait_bound_name,
    type_contains_lifetime_argument,
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
