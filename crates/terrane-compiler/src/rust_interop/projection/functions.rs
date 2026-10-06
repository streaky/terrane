//! Admits projected Rust function signatures and invocation-scoped results.
use super::types::{invocation_scoped_sequence_impl_trait_input, structural_impl_trait_input};
use super::{
    BTreeMap, BTreeSet, ChainRole, Function, GenericBound, GenericMonomorphisations,
    GenericParamDefKind, HashMap, Id, Item, ItemEnum, ItemSummary, ProjectedExecutionRequirements,
    ProjectedFunction, ProjectedGenericParameter, ProjectedParameter, ProjectedType, Receiver,
    RequirementKnowledge, Type, concrete_into_future_output, default_generic_instantiation,
    generic_bounds, generic_monomorphisations, impl_trait_bounds, instantiated_nominal_name,
    project_associated_binding, project_borrowed_graph_type, project_callable_adapter_bounds,
    project_interface, project_invocation_scoped_type, project_struct_fields, project_type,
    projectable_interface_bound, receiver_kind, render_generic_bound, render_generic_bounds,
    render_resolved_path, resolved_name, resolved_path_name, type_contains_borrowed_ref,
    type_contains_lifetime_argument,
};
use super::{
    expand_output_alias, projected_error_name, safe_parameter_name, type_arguments,
    type_mentions_generic,
};

pub(super) fn project_function_with_generics(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    method_name: Option<&str>,
    supplied_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    project_function_inner(
        function,
        index,
        paths,
        public_paths,
        method_name,
        supplied_generics,
        allow_lifetime_output,
    )
}

pub(super) fn project_function(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    method_name: Option<&str>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    project_function_inner(
        function,
        index,
        paths,
        public_paths,
        method_name,
        &BTreeMap::new(),
        allow_lifetime_output,
    )
}

fn open_chain_generics(
    function: &Function,
    index: &HashMap<Id, Item>,
) -> Option<BTreeMap<String, ProjectedType>> {
    let output = function.sig.output.as_ref()?;
    if function
        .sig
        .inputs
        .first()
        .is_some_and(|(name, _)| name == "self")
    {
        return None;
    }
    if !matches!(output, Type::ResolvedPath(_)) || !type_contains_lifetime_argument(output) {
        return None;
    }
    if !type_arguments(output).into_iter().any(|argument| {
        matches!(
            argument,
            Type::QualifiedPath { self_type, .. }
                if matches!(self_type.as_ref(), Type::Generic(_))
        )
    }) {
        return None;
    }
    let has_terminal_associated_shape = function.generics.params.iter().any(|parameter| {
        generic_bounds(parameter, function).iter().any(|bound| {
            let GenericBound::TraitBound { trait_, .. } = bound else {
                return false;
            };
            let Some(Item {
                inner: ItemEnum::Trait(declaration),
                ..
            }) = index.get(&trait_.id)
            else {
                return false;
            };
            let associated = declaration
                .items
                .iter()
                .filter_map(|id| index.get(id).and_then(|item| item.name.as_deref()))
                .collect::<BTreeSet<_>>();
            ["Arguments", "QueryResult", "Row"]
                .into_iter()
                .all(|name| associated.contains(name))
        })
    });
    if !has_terminal_associated_shape {
        return None;
    }
    let generics = function
        .generics
        .params
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .map(|parameter| {
            (
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            )
        })
        .collect::<BTreeMap<_, _>>();
    (!generics.is_empty()).then_some(generics)
}

fn open_chain_result(
    output: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Option<ProjectedType> {
    let Type::ResolvedPath(path) = output else {
        return None;
    };
    let Item {
        inner: ItemEnum::Struct(structure),
        ..
    } = index.get(&path.id)?
    else {
        return None;
    };
    if !structure
        .generics
        .params
        .iter()
        .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }))
    {
        return None;
    }
    let base_rust_path = resolved_path_name(path, paths);
    let arguments = structure
        .generics
        .params
        .iter()
        .filter_map(|parameter| match parameter.kind {
            GenericParamDefKind::Type { .. } => generics.get(&parameter.name).cloned(),
            _ => None,
        })
        .collect::<Vec<_>>();
    let rust_identity = format!(
        "{}<{}>",
        base_rust_path,
        arguments
            .iter()
            .map(ProjectedType::rust_type)
            .collect::<Vec<_>>()
            .join(", ")
    );
    Some(ProjectedType::Foreign {
        rust_path: base_rust_path.clone(),
        name: instantiated_nominal_name(
            base_rust_path.rsplit("::").next().unwrap_or("chain"),
            &rust_identity,
            &arguments,
        ),
        base_rust_path,
        arguments,
    })
}

fn input_has_owned_borrowed_view(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> bool {
    let resolved = match ty {
        Type::BorrowedRef {
            is_mutable: false,
            type_,
            ..
        } => type_.as_ref(),
        other => other,
    };
    let Type::ResolvedPath(path) = resolved else {
        return false;
    };
    let Some(Item {
        inner: ItemEnum::Struct(structure),
        attrs,
        ..
    }) = index.get(&path.id)
    else {
        return false;
    };
    !attrs
        .iter()
        .any(|attribute| matches!(attribute, rustdoc_types::Attribute::NonExhaustive))
        && default_generic_instantiation(structure, index, paths)
            .and_then(|generics| project_struct_fields(structure, index, paths, &generics))
            .is_ok_and(|(_, borrowed_view)| borrowed_view)
}

#[expect(
    clippy::too_many_lines,
    reason = "function projection keeps generic selection and exact parameter contracts together"
)]
pub(super) fn project_function_inner(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    method_name: Option<&str>,
    supplied_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    let open_chain = open_chain_generics(function, index);
    let GenericMonomorphisations {
        types: generic_types,
        destination_result,
        adapter_result_bounds,
    } = if let Some(types) = open_chain.clone() {
        GenericMonomorphisations {
            types,
            destination_result: None,
            adapter_result_bounds: BTreeMap::new(),
        }
    } else {
        generic_monomorphisations(function, index, paths, supplied_generics)
            .map_err(|reason| format!("generic selection: {reason}"))?
    };
    let borrows_input = function
        .sig
        .inputs
        .iter()
        .any(|(_, ty)| matches!(ty, Type::BorrowedRef { .. }));
    if function
        .sig
        .output
        .as_ref()
        .is_some_and(type_contains_borrowed_ref)
        && !borrows_input
    {
        return Err("borrowed result values require an invocation-scoped lender".to_owned());
    }
    let mut parameters: Vec<ProjectedParameter> = Vec::new();
    let mut receiver = None;
    for (parameter_index, (name, ty)) in function.sig.inputs.iter().enumerate() {
        let requires_static_reference = match ty {
            Type::BorrowedRef {
                lifetime: Some(lifetime),
                ..
            } => lifetime == "'static",
            Type::ResolvedPath(path)
                if index
                    .get(&path.id)
                    .is_some_and(|item| matches!(item.inner, ItemEnum::TypeAlias(_))) =>
            {
                let (effective_input, _) =
                    expand_output_alias(ty.clone(), index, paths, generic_types.clone())?;
                matches!(
                    &effective_input,
                    Type::BorrowedRef { lifetime: Some(lifetime), .. } if lifetime == "'static"
                )
            }
            _ => false,
        };
        if requires_static_reference {
            return Err(format!(
                "parameter `{name}` requires a static reference; Terrane arguments cannot guarantee a static lifetime"
            ));
        }
        if name == "self" {
            receiver = Some(receiver_kind(ty)?);
            continue;
        }
        if !matches!(ty, Type::BorrowedRef { .. }) && type_contains_borrowed_ref(ty) {
            return Err("nested borrowed parameter cannot cross a projected boundary".to_owned());
        }
        let invocation_scoped_input = allow_lifetime_output
            && type_contains_lifetime_argument(ty)
            && function
                .sig
                .output
                .as_ref()
                .is_some_and(type_contains_lifetime_argument);
        if type_contains_lifetime_argument(ty)
            && !input_has_owned_borrowed_view(ty, index, paths)
            && !invocation_scoped_input
        {
            return Err(
                "input contains a lifetime-bearing foreign value with no non-escaping Terrane call representation"
                    .to_owned(),
            );
        }
        let (projected_type, impl_trait_parameter, inline_adapter_bounds) = if let Some(bounds) =
            impl_trait_bounds(ty)
        {
            let generic = format!("TerraneImpl{parameter_index}");
            if let Some((callback, result_bounds, _)) =
                project_callable_adapter_bounds(&generic, bounds, index, paths, &generic_types)?
            {
                (callback, Some(generic), Some(result_bounds))
            } else if let Some(projected) = invocation_scoped_sequence_impl_trait_input(
                bounds,
                index,
                paths,
                public_paths,
                &generic_types,
            )? {
                (projected, None, None)
            } else if let Some(projected) = structural_impl_trait_input(bounds, paths) {
                (projected, Some(generic), None)
            } else if let Ok(projectable) = projectable_interface_bound(bounds, index, paths) {
                let Some(Item {
                    inner: ItemEnum::Trait(declaration),
                    ..
                }) = index.get(&projectable.id)
                else {
                    return Err("`impl Trait` input has an unresolved trait bound".to_owned());
                };
                if project_interface(declaration, index, paths, &projectable.path).is_err() {
                    return Err(
                        "`impl Trait` input bound is not a projectable interface".to_owned()
                    );
                }
                let rust_path = render_resolved_path(projectable, index, paths, &generic_types)?;
                (
                    ProjectedType::Foreign {
                        name: projectable
                            .path
                            .rsplit("::")
                            .next()
                            .unwrap_or(&projectable.path)
                            .to_owned(),
                        base_rust_path: rust_path.clone(),
                        rust_path,
                        arguments: Vec::new(),
                    },
                    Some(generic),
                    None,
                )
            } else {
                (ProjectedType::Generic(generic.clone()), Some(generic), None)
            }
        } else if invocation_scoped_input {
            (
                project_invocation_scoped_type(ty, index, paths, &generic_types)?,
                None,
                None,
            )
        } else {
            (
                project_type(ty, index, paths, &generic_types)
                    .map_err(|reason| format!("parameter `{name}`: {reason}"))?,
                None,
                None,
            )
        };
        let (borrowed, mutable_borrow) = match ty {
            Type::BorrowedRef { is_mutable, .. } => (true, *is_mutable),
            _ => (false, false),
        };
        if mutable_borrow && !matches!(projected_type, ProjectedType::Foreign { .. }) {
            return Err("mutable borrowed primitive parameters are not representable".to_owned());
        }
        let boxed_adapter = match &projected_type {
            ProjectedType::BoxedInterface {
                trait_path,
                auto_traits,
                associated_type,
                ..
            } => {
                let principal = associated_type.as_ref().map_or_else(
                    || trait_path.clone(),
                    |associated| {
                        format!(
                            "{trait_path}<{} = {}>",
                            associated.name,
                            associated.ty.rust_type()
                        )
                    },
                );
                Some((
                    format!("TerraneBoxed{parameter_index}"),
                    std::iter::once(principal)
                        .chain(auto_traits.iter().cloned())
                        .chain(std::iter::once("'static".to_owned()))
                        .collect::<Vec<_>>(),
                ))
            }
            _ => None,
        };
        let callable_adapter = inline_adapter_bounds
            .as_ref()
            .and_then(|bounds| match &projected_type {
                ProjectedType::Callback { result, .. } => match result.as_ref() {
                    ProjectedType::Generic(result) => Some((result.clone(), bounds.clone())),
                    _ => None,
                },
                _ => None,
            })
            .or_else(|| match (ty, &projected_type) {
                (Type::Generic(parameter), ProjectedType::Callback { result, .. }) => {
                    adapter_result_bounds
                        .get(parameter)
                        .and_then(|bounds| match result.as_ref() {
                            ProjectedType::Generic(result) => {
                                Some((result.clone(), bounds.clone()))
                            }
                            _ => None,
                        })
                }
                _ => None,
            });
        let generic_parameter = callable_adapter
            .as_ref()
            .map(|(name, _)| name.clone())
            .or(impl_trait_parameter)
            .or_else(|| boxed_adapter.as_ref().map(|(name, _)| name.clone()))
            .or_else(|| {
                function.generics.params.iter().find_map(|parameter| {
                    generic_types.get(&parameter.name).and_then(|projected| {
                        let exact_scoped_callback = matches!(
                            projected,
                            ProjectedType::Callback { result, .. }
                                if matches!(
                                    result.as_ref(),
                                    ProjectedType::InvocationScoped { .. }
                                )
                        );
                        ((matches!(projected, ProjectedType::Foreign { .. })
                            || exact_scoped_callback
                            || matches!(projected, ProjectedType::Generic(_))
                                && matches!(projected_type, ProjectedType::Generic(_)))
                            && type_mentions_generic(ty, &parameter.name))
                        .then(|| parameter.name.clone())
                    })
                })
            });
        let rendered_generic_bounds = if let Some((_, bounds)) = &callable_adapter {
            bounds.clone()
        } else if let Some((_, bounds)) = &boxed_adapter {
            bounds.clone()
        } else if let Some(name) = &generic_parameter {
            if name.starts_with("TerraneImpl") {
                impl_trait_bounds(ty)
                    .map(|bounds| {
                        bounds
                            .iter()
                            .map(|bound| {
                                render_generic_bound(
                                    bound,
                                    &function.generics.params,
                                    index,
                                    paths,
                                    &generic_types,
                                )
                            })
                            .collect()
                    })
                    .transpose()?
                    .unwrap_or_default()
            } else {
                let parameter = function
                    .generics
                    .params
                    .iter()
                    .find(|parameter| parameter.name == *name)
                    .expect("selected generic parameter must be declared");
                render_generic_bounds(parameter, function, index, paths, &generic_types)?
            }
        } else {
            Vec::new()
        };
        let associated_type = if let ProjectedType::BoxedInterface {
            associated_type, ..
        } = &projected_type
        {
            associated_type.clone()
        } else if let Some(name) = &generic_parameter {
            let bounds = if name.starts_with("TerraneImpl") {
                impl_trait_bounds(ty).map(<[_]>::to_vec)
            } else {
                function
                    .generics
                    .params
                    .iter()
                    .find(|parameter| parameter.name == *name)
                    .map(|parameter| generic_bounds(parameter, function))
            };
            bounds
                .as_deref()
                .and_then(|bounds| projectable_interface_bound(bounds, index, paths).ok())
                .map(|trait_| project_associated_binding(trait_, index, paths, &generic_types))
                .transpose()?
                .flatten()
        } else {
            None
        };
        let generic_interface = if let Some(name) = &generic_parameter {
            let bounds = if name.starts_with("TerraneImpl") {
                impl_trait_bounds(ty).map(<[_]>::to_vec)
            } else {
                function
                    .generics
                    .params
                    .iter()
                    .find(|parameter| parameter.name == *name)
                    .map(|parameter| generic_bounds(parameter, function))
            };
            bounds
                .as_deref()
                .and_then(|bounds| projectable_interface_bound(bounds, index, paths).ok())
                .and_then(|trait_| {
                    let Item {
                        inner: ItemEnum::Trait(declaration),
                        ..
                    } = index.get(&trait_.id)?
                    else {
                        return None;
                    };
                    project_interface(declaration, index, paths, &trait_.path)
                        .is_ok()
                        .then(|| paths.get(&trait_.id).map(|summary| summary.path.join("::")))
                        .flatten()
                })
        } else {
            None
        };
        let borrowed = borrowed
            || (generic_parameter
                .as_ref()
                .is_some_and(|name| name.starts_with("TerraneImpl"))
                && generic_interface.is_none()
                && matches!(projected_type, ProjectedType::String | ProjectedType::Bytes));
        let mut parameter_name = safe_parameter_name(name);
        if method_name.is_some_and(|function_name| function_name == parameter_name)
            || parameters
                .iter()
                .any(|parameter| parameter.name == parameter_name)
        {
            parameter_name = format!("{parameter_name}-value");
        }
        parameters.push(ProjectedParameter {
            name: parameter_name,
            ty: projected_type,
            borrowed,
            mutable_borrow,
            generic_parameter,
            generic_interface,
            generic_bounds: rendered_generic_bounds,
            associated_type,
        });
    }
    let (effective_output, returns_future, into_future, output_generic_types) =
        match function.sig.output.as_ref() {
            Some(output)
                if resolved_name(output, paths).as_deref()
                    == Some("futures_core::future::BoxFuture") =>
            {
                (
                    type_arguments(output).into_iter().next_back().cloned(),
                    true,
                    false,
                    generic_types.clone(),
                )
            }
            Some(output) => match concrete_into_future_output(output, index, paths, &generic_types)
            {
                Some((into_future_output, into_future_generics)) => {
                    (Some(into_future_output), true, true, into_future_generics)
                }
                None => (Some(output.clone()), false, false, generic_types.clone()),
            },
            None => (None, false, false, generic_types.clone()),
        };
    let (effective_output, output_generic_types) = match effective_output {
        Some(output) => {
            let (output, generics) =
                expand_output_alias(output, index, paths, output_generic_types)?;
            (Some(output), generics)
        }
        None => (None, output_generic_types),
    };
    if (function.header.is_async || returns_future)
        && effective_output
            .as_ref()
            .is_some_and(type_contains_borrowed_ref)
    {
        return Err("borrowed result values cannot cross a projected boundary".to_owned());
    }
    let mut error = None;
    let mut error_optional_depth = 0;
    let project_output = |ty: &Type| {
        if type_contains_borrowed_ref(ty) {
            return project_borrowed_graph_type(ty, index, paths, &output_generic_types);
        }
        if allow_lifetime_output && type_contains_lifetime_argument(ty) {
            project_invocation_scoped_type(ty, index, paths, &output_generic_types)
        } else {
            project_type(ty, index, paths, &output_generic_types)
        }
    };
    let result = if let Some(output) = effective_output.as_ref() {
        if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Result") || name == "Result")
        {
            let arguments = type_arguments(output);
            let value = arguments
                .first()
                .ok_or_else(|| "Result has no value type".to_owned())?;
            let projected = project_output(value)
                .map_err(|reason| format!("projected result value: {reason}"))?;
            if projected.rust_type().contains('&')
                && !matches!(
                    projected,
                    ProjectedType::InvocationScoped {
                        expression_scoped: true,
                        ..
                    } | ProjectedType::BorrowedString
                )
            {
                return Err("borrowed result values cannot cross a projected boundary".to_owned());
            }
            error = arguments
                .get(1)
                .and_then(|ty| projected_error_name(ty, output, paths))
                .or_else(|| Some("Error".to_owned()));
            projected
        } else if type_contains_borrowed_ref(output) {
            project_output(output)?
        } else if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Option") || name == "Option")
        {
            let arguments = type_arguments(output);
            let value = arguments
                .first()
                .ok_or_else(|| "Option has no value type".to_owned())?;
            if resolved_name(value, paths)
                .is_some_and(|name| name.ends_with("::Result") || name == "Result")
            {
                let arguments = type_arguments(value);
                let success = arguments
                    .first()
                    .ok_or_else(|| "nested Result has no value type".to_owned())?;
                let projected = project_output(success)
                    .map_err(|reason| format!("projected nested result value: {reason}"))?;
                if projected.rust_type().contains('&') {
                    return Err(
                        "borrowed nested result values cannot cross a projected boundary"
                            .to_owned(),
                    );
                }
                error = arguments
                    .get(1)
                    .and_then(|ty| projected_error_name(ty, value, paths))
                    .or_else(|| Some("Error".to_owned()));
                error_optional_depth = 1;
                ProjectedType::Optional(Box::new(projected))
            } else {
                ProjectedType::Optional(Box::new(
                    project_output(value)
                        .map_err(|reason| format!("projected optional value: {reason}"))?,
                ))
            }
        } else {
            project_output(output).or_else(|reason| {
                open_chain_result(output, index, paths, &output_generic_types)
                    .ok_or_else(|| format!("projected output: {reason}"))
            })?
        }
    } else {
        ProjectedType::None
    };
    if result.contains_opaque()
        && !matches!(result, ProjectedType::Foreign { .. })
        && !matches!(
            result,
            ProjectedType::Opaque {
                anonymous_chain: true,
                ..
            }
        )
    {
        return Err("producer-selected opaque result requires a named foreign owner".to_owned());
    }
    if open_chain.is_none()
        && allow_lifetime_output
        && let Some(Type::ResolvedPath(path)) = effective_output.as_ref()
        && let Some(Item {
            inner: ItemEnum::Struct(structure),
            ..
        }) = index.get(&path.id)
        && structure
            .generics
            .params
            .iter()
            .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }))
        && let ProjectedType::Foreign { arguments, .. } = &result
        && arguments.len()
            != structure
                .generics
                .params
                .iter()
                .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
                .count()
    {
        return Err("lifetime-bearing chain result has unresolved generic state".to_owned());
    }
    if matches!(result, ProjectedType::BoxedInterface { .. }) {
        return Err("boxed trait-object results cannot cross a projected boundary".to_owned());
    }
    let mut generic_parameters = function
        .generics
        .params
        .iter()
        .filter(|parameter| {
            matches!(
                generic_types.get(&parameter.name),
                Some(ProjectedType::Generic(_))
            )
        })
        .map(|parameter| {
            Ok(ProjectedGenericParameter {
                name: parameter.name.clone(),
                input_selected: parameters
                    .iter()
                    .any(|input| input.ty.contains_generic(&parameter.name)),
                rust_bounds: render_generic_bounds(
                    parameter,
                    function,
                    index,
                    paths,
                    &generic_types,
                )?,
                default: None,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    generic_parameters.extend(parameters.iter().filter_map(|parameter| {
        let name = match &parameter.ty {
            ProjectedType::Generic(name) if name.starts_with("TerraneImpl") => name,
            ProjectedType::Callback { result, .. }
                if parameter.generic_parameter.as_ref().is_some_and(|name| {
                    result.as_ref() == &ProjectedType::Generic(name.clone())
                }) =>
            {
                parameter.generic_parameter.as_ref().expect("checked above")
            }
            _ => return None,
        };
        Some(ProjectedGenericParameter {
            name: name.clone(),
            input_selected: true,
            rust_bounds: parameter.generic_bounds.clone(),
            default: None,
        })
    }));
    let mut merged_generic_parameters: Vec<ProjectedGenericParameter> = Vec::new();
    for generic in generic_parameters {
        if let Some(existing) = merged_generic_parameters
            .iter_mut()
            .find(|existing| existing.name == generic.name)
        {
            existing.input_selected |= generic.input_selected;
            existing.rust_bounds.extend(generic.rust_bounds);
            existing.rust_bounds.sort();
            existing.rust_bounds.dedup();
        } else {
            merged_generic_parameters.push(generic);
        }
    }
    let generic_parameters = merged_generic_parameters;
    let rust_generic_arguments = function
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Type {
                is_synthetic: false,
                ..
            } => Some(
                generic_types
                    .get(&parameter.name)
                    .cloned()
                    .unwrap_or_else(|| ProjectedType::Generic(parameter.name.clone())),
            ),
            _ => None,
        })
        .collect();
    let chain_role = matches!(
        &result,
        ProjectedType::InvocationScoped {
            expression_scoped: true,
            ..
        }
    )
    .then_some(ChainRole::Root);
    Ok(ProjectedFunction {
        operation_owner_generics: Vec::new(),
        name: method_name.unwrap_or_default().to_owned(),
        native_owner: None,
        native_path: None,
        parameters,
        result,
        generic_parameters,
        rust_generic_arguments,
        destination_result,
        error,
        is_async: function.header.is_async || returns_future,
        is_unsafe: function.header.is_unsafe,
        into_future,
        execution_requirements: (function.header.is_async || returns_future).then_some(
            ProjectedExecutionRequirements {
                runtime_context: RequirementKnowledge::Unknown,
                wake_support: RequirementKnowledge::Required,
                transfer: RequirementKnowledge::Unknown,
            },
        ),
        enum_operation: None,
        error_optional_depth,
        chain_role,
        receiver,
    })
}

pub(super) fn promote_async_endpoint_methods(methods: &mut [ProjectedFunction]) {
    let has_consuming_close = methods
        .iter()
        .any(|method| method.name == "close" && method.receiver == Some(Receiver::Move));
    if !has_consuming_close {
        return;
    }
    for method in methods.iter_mut() {
        if method.name == "next"
            && method.is_async
            && matches!(
                method.receiver,
                Some(Receiver::Borrow | Receiver::MutableBorrow)
            )
            && method.error.is_some()
            && let ProjectedType::Optional(item) = &method.result
        {
            method.result = ProjectedType::AsyncIterationStep(item.clone());
        }
    }
    for method in methods {
        if method.name == "send"
            && method.is_async
            && method.parameters.len() == 1
            && matches!(
                method.receiver,
                Some(Receiver::Borrow | Receiver::MutableBorrow)
            )
            && method.error.is_some()
            && method.result == ProjectedType::Bool
        {
            method.result = ProjectedType::AsyncSinkOutcome;
        }
    }
}
