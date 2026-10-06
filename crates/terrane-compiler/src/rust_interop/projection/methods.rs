//! Rustdoc method, chain-owner, and output-alias projection decisions.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use rustdoc_types::{
    Function, GenericParamDef, GenericParamDefKind, Id, Impl, Item, ItemEnum, ItemSummary, Type,
    Visibility, WherePredicate,
};

use super::{
    ChainRole, ProjectedBoundaryCapabilities, ProjectedConstant, ProjectedFunction,
    ProjectedGenericParameter, ProjectedItem, ProjectedKind, ProjectedType, Receiver,
    RustDependency, SourceConstantCache, aliases, callable, canonicalize_rust_path, data,
    dependency_namespace, implementation_trait_path, project_function_with_generics,
    project_rust_constant_expression, project_type, projected_constant_name, render_resolved_path,
    render_rust_type, rewrite_rust_bound_root, rust_lifetimes, source_constant_expression,
    type_arguments,
};

pub(super) type ProjectedMethods = (
    Vec<ProjectedFunction>,
    Vec<(String, String, bool, Option<String>, ProjectedFunction)>,
    Vec<ProjectedConstant>,
    Vec<(String, String)>,
);

fn project_operation_owner_generics(
    implementation: &Impl,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Vec<ProjectedGenericParameter>, String> {
    let declaration = &implementation.generics;
    let has_owner_lifetimes = declaration
        .params
        .iter()
        .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }));
    for predicate in &declaration.where_predicates {
        if !matches!(
            predicate,
            WherePredicate::BoundPredicate {
                type_: Type::Generic(name),
                ..
            } if declaration.params.iter().any(|parameter| {
                parameter.name == *name
                    && matches!(
                        parameter.kind,
                        GenericParamDefKind::Type { is_synthetic: false, .. }
                    )
            })
        ) {
            return Err(
                "inherent impl predicate has no stable owner-generic projection".to_owned(),
            );
        }
    }
    let mut projected = BTreeMap::<String, ProjectedGenericParameter>::new();
    for parameter in &declaration.params {
        match &parameter.kind {
            GenericParamDefKind::Type {
                is_synthetic: false,
                ..
            } => {}
            GenericParamDefKind::Lifetime { outlives } if outlives.is_empty() => continue,
            _ => {
                return Err(
                    "inherent impl parameter has no stable owner-generic projection".to_owned(),
                );
            }
        }
        let rust_bounds = callable::render_generic_bounds_from_generics(
            parameter,
            declaration,
            &[],
            index,
            paths,
            generics,
        )?;
        if rust_bounds.iter().any(|bound| {
            (bound.starts_with('\'') && bound != "'static")
                || (has_owner_lifetimes
                    && rust_lifetimes(bound).iter().any(|lifetime| {
                        declaration.params.iter().any(|parameter| {
                            parameter.name == *lifetime
                                && matches!(parameter.kind, GenericParamDefKind::Lifetime { .. })
                        })
                    }))
        }) {
            return Err(
                "inherent impl lifetime bound has no stable owner-generic projection".to_owned(),
            );
        }
        // The self-type substitution maps impl binders to nominal or alias slots.
        // Concrete alias arguments need no helper parameter; Rust checks their bounds.
        let Some(selected) = generics.get(&parameter.name) else {
            if rust_bounds.is_empty() {
                continue;
            }
            return Err(format!(
                "inherent impl binder `{}` has no selected owner-generic slot",
                parameter.name
            ));
        };
        let ProjectedType::Generic(name) = selected else {
            if !selected.contains_open_generic() || rust_bounds.is_empty() {
                continue;
            }
            return Err(format!(
                "inherent impl binder `{}` has no stable owner-generic projection",
                parameter.name
            ));
        };
        let entry = projected
            .entry(name.clone())
            .or_insert_with(|| ProjectedGenericParameter {
                input_selected: false,
                name: name.clone(),
                rust_bounds: Vec::new(),
                default: None,
            });
        entry.rust_bounds.extend(rust_bounds);
        entry.rust_bounds.sort();
        entry.rust_bounds.dedup();
    }
    Ok(projected.into_values().collect())
}

fn disambiguate_static_method_owner_generics(
    implementation: &Impl,
    function: &Function,
    mut generics: BTreeMap<String, ProjectedType>,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> BTreeMap<String, ProjectedType> {
    // Keep owner and method binders distinct before name-keyed signature projection.
    let method_names = function
        .generics
        .params
        .iter()
        .map(|parameter| parameter.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut used_names = method_names
        .iter()
        .map(|name| (*name).to_owned())
        .collect::<BTreeSet<_>>();
    used_names.extend(
        implementation
            .generics
            .params
            .iter()
            .map(|parameter| parameter.name.clone()),
    );
    used_names.extend(generics.values().filter_map(|generic| match generic {
        ProjectedType::Generic(name) => Some(name.clone()),
        _ => None,
    }));
    for parameter in &implementation.generics.params {
        let GenericParamDefKind::Type {
            is_synthetic: false,
            ..
        } = &parameter.kind
        else {
            continue;
        };
        let Some(ProjectedType::Generic(owner_slot)) = generics.get(&parameter.name) else {
            continue;
        };
        if !method_names.contains(owner_slot.as_str()) {
            continue;
        }
        let owner_slot = owner_slot.clone();
        let mut suffix = 0;
        let mut renamed = format!("__TerraneOwner_{owner_slot}__{suffix}");
        while used_names.contains(&renamed) {
            suffix += 1;
            renamed = format!("__TerraneOwner_{owner_slot}__{suffix}");
        }
        used_names.insert(renamed.clone());
        generics.remove(&owner_slot);
        generics.insert(parameter.name.clone(), ProjectedType::Generic(renamed));
    }
    if let Ok(owner) = project_type(&implementation.for_, index, paths, &generics) {
        generics.insert("Self".to_owned(), owner);
    }
    generics
}

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "rustdoc impl traversal keeps one shared context for trait, inherent, and source-backed member decisions"
)]
pub(super) fn project_methods(
    impl_ids: &[Id],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    owner_rust_path: &str,
    owner_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
    source_constants: &mut SourceConstantCache,
) -> ProjectedMethods {
    let mut candidates = Vec::new();
    let mut trait_methods = Vec::new();
    let mut constants = Vec::new();
    let mut declined = Vec::new();
    for impl_id in impl_ids {
        let Some(Item {
            inner: ItemEnum::Impl(implementation),
            ..
        }) = index.get(impl_id)
        else {
            continue;
        };
        if implementation.is_negative || implementation.is_synthetic {
            continue;
        }
        let inherent = implementation.trait_.is_none();
        let mut implementation_generics =
            aliases::implementation_owner_generics(implementation, index, owner_generics);
        let native_owner = if matches!(implementation.for_, Type::Generic(_))
            && implementation.blanket_impl.is_some()
        {
            owner_rust_path.to_owned()
        } else {
            render_rust_type(&implementation.for_, index, paths, &implementation_generics)
                .unwrap_or_else(|_| owner_rust_path.to_owned())
        };
        if inherent
            && let Ok(owner) =
                project_type(&implementation.for_, index, paths, &implementation_generics)
        {
            implementation_generics.insert("Self".to_owned(), owner);
        }
        if let Some(Type::Generic(generic)) = &implementation.blanket_impl
            && let Some(owner) = owner_generics.get("Self")
        {
            implementation_generics.insert(generic.clone(), owner.clone());
        }
        for item_id in &implementation.items {
            let Some(Item {
                name: Some(name),
                inner:
                    ItemEnum::AssocType {
                        type_: Some(type_), ..
                    },
                ..
            }) = index.get(item_id)
            else {
                continue;
            };
            if let Ok(projected) = project_type(type_, index, paths, &implementation_generics) {
                implementation_generics.insert(format!("Self::{name}"), projected);
            }
        }
        for method_id in &implementation.items {
            let Some(item) = index.get(method_id) else {
                continue;
            };
            if inherent && item.visibility != Visibility::Public {
                continue;
            }
            let Some(name) = item.name.as_deref() else {
                continue;
            };
            if let ItemEnum::AssocConst { type_, value } = &item.inner {
                if !inherent {
                    continue;
                }
                match project_type(type_, index, paths, &implementation_generics) {
                    Ok(ty) => {
                        let source_expression = value
                            .as_deref()
                            .filter(|value| *value != "_")
                            .map(str::to_owned)
                            .or_else(|| {
                                source_constant_expression(
                                    item,
                                    &native_owner,
                                    name,
                                    source_constants,
                                )
                            });
                        let terrane_value = source_expression
                            .as_deref()
                            .and_then(project_rust_constant_expression);
                        constants.push(ProjectedConstant {
                            name: projected_constant_name(name),
                            rust_name: name.to_owned(),
                            rust_path: if native_owner.contains('<') {
                                format!("<{native_owner}>::{name}")
                            } else {
                                format!("{native_owner}::{name}")
                            },
                            ty,
                            source_expression,
                            terrane_value,
                        });
                    }
                    Err(reason) => declined.push((name.to_owned(), reason)),
                }
                continue;
            }
            let ItemEnum::Function(function) = &item.inner else {
                declined.push((
                    name.to_owned(),
                    "item kind has no Terrane member projection".to_owned(),
                ));
                continue;
            };
            let mut enriched_function = function.clone();
            if implementation.blanket_impl.is_some() {
                for parameter in &implementation.generics.params {
                    if !enriched_function
                        .generics
                        .params
                        .iter()
                        .any(|candidate| candidate.name == parameter.name)
                    {
                        enriched_function.generics.params.push(parameter.clone());
                    }
                }
                enriched_function
                    .generics
                    .where_predicates
                    .extend(implementation.generics.where_predicates.iter().cloned());
            }
            let function = &enriched_function;
            if !inherent {
                let Some(trait_) = implementation.trait_.as_ref() else {
                    continue;
                };
                let Some((trait_path, local_trait)) = implementation_trait_path(trait_, paths)
                else {
                    declined.push((
                        name.to_owned(),
                        "trait method has no canonical Rust trait path".to_owned(),
                    ));
                    continue;
                };
                let public_trait_path = public_paths
                    .get(&trait_.id)
                    .cloned()
                    .unwrap_or_else(|| trait_path.clone());
                let mut trait_render_generics = implementation_generics.clone();
                for parameter in &implementation.generics.params {
                    if matches!(parameter.kind, GenericParamDefKind::Type { .. }) {
                        trait_render_generics
                            .entry(parameter.name.clone())
                            .or_insert_with(|| ProjectedType::Generic(parameter.name.clone()));
                    }
                }
                let rendered_trait =
                    render_resolved_path(trait_, index, paths, &trait_render_generics)
                        .unwrap_or_else(|_| trait_path.clone());
                let trait_implementation_path = rendered_trait.find('<').map_or_else(
                    || trait_path.clone(),
                    |start| format!("{trait_path}{}", &rendered_trait[start..]),
                );
                match project_function_with_generics(
                    function,
                    index,
                    paths,
                    public_paths,
                    Some(name),
                    &implementation_generics,
                    allow_lifetime_output,
                ) {
                    Ok(mut method) => {
                        data::rebind_method_owner(
                            &mut method,
                            implementation,
                            index,
                            paths,
                            &implementation_generics,
                        );
                        method.native_owner = Some(native_owner.clone());
                        let trait_rust_path = if local_trait {
                            let package_root = trait_implementation_path
                                .split("::")
                                .next()
                                .unwrap_or_default();
                            let dependency_root =
                                owner_rust_path.split("::").next().unwrap_or_default();
                            rewrite_rust_bound_root(
                                &trait_implementation_path,
                                package_root,
                                dependency_root,
                            )
                        } else {
                            trait_implementation_path.replace('-', "_")
                        };
                        let trait_rust_path = canonicalize_rust_path(&trait_rust_path);
                        method.native_path =
                            Some(format!("<{owner_rust_path} as {trait_rust_path}>::{name}"));
                        if implementation.blanket_impl.is_some()
                            && !(method.destination_result.is_some()
                                && method.receiver == Some(Receiver::Move)
                                && method.parameters.is_empty())
                        {
                            continue;
                        }
                        trait_methods.push((
                            trait_implementation_path,
                            public_trait_path,
                            local_trait,
                            item.docs.clone(),
                            method,
                        ));
                    }
                    Err(reason) => {
                        if implementation.blanket_impl.is_none()
                            && !is_internal_rust_protocol_method(&trait_path, name)
                        {
                            declined.push((name.to_owned(), reason));
                        }
                    }
                }
                continue;
            }
            let static_owner_generics = inherent
                && implementation.blanket_impl.is_none()
                && !function.sig.inputs.iter().any(|(name, _)| name == "self");
            let method_generics = if static_owner_generics {
                disambiguate_static_method_owner_generics(
                    implementation,
                    function,
                    implementation_generics.clone(),
                    index,
                    paths,
                )
            } else {
                implementation_generics.clone()
            };
            let method_native_owner = if static_owner_generics {
                render_rust_type(&implementation.for_, index, paths, &method_generics)
                    .unwrap_or_else(|_| native_owner.clone())
            } else {
                native_owner.clone()
            };
            match project_function_with_generics(
                function,
                index,
                paths,
                public_paths,
                Some(name),
                &method_generics,
                allow_lifetime_output,
            ) {
                Ok(mut method) => {
                    data::rebind_method_owner(
                        &mut method,
                        implementation,
                        index,
                        paths,
                        &method_generics,
                    );
                    method.native_owner = Some(method_native_owner);
                    if static_owner_generics {
                        match project_operation_owner_generics(
                            implementation,
                            index,
                            paths,
                            &method_generics,
                        ) {
                            Ok(owner_generics) => method.operation_owner_generics = owner_generics,
                            Err(reason) => {
                                declined.push((name.to_owned(), reason));
                                continue;
                            }
                        }
                    }
                    candidates.push(method);
                }
                Err(reason) => declined.push((name.to_owned(), reason)),
            }
        }
    }
    let mut methods = Vec::new();
    while let Some(method) = candidates.pop() {
        let name = method.name.clone();
        let mut matching = vec![method];
        let mut candidate_index = 0;
        while candidate_index < candidates.len() {
            if candidates[candidate_index].name == name {
                matching.push(candidates.swap_remove(candidate_index));
            } else {
                candidate_index += 1;
            }
        }
        if matching.len() == 1 {
            methods.push(matching.pop().expect("one matching method"));
            continue;
        }
        declined.extend(matching.into_iter().map(|method| {
            (
                method.name,
                "multiple inherent methods with the same name are not projectable".to_owned(),
            )
        }));
    }
    methods.sort_by(|left, right| left.name.cmp(&right.name));
    constants.sort_by(|left, right| left.name.cmp(&right.name));
    (methods, trait_methods, constants, declined)
}
fn is_internal_rust_protocol_method(trait_path: &str, method: &str) -> bool {
    (method == "fmt"
        && (trait_path.ends_with("::fmt::Debug") || trait_path.ends_with("::fmt::Display")))
        || (method == "hash" && trait_path.ends_with("::hash::Hash"))
}
#[expect(
    clippy::too_many_arguments,
    reason = "chain-owner projection needs the resolved dependency and all Rustdoc path contexts"
)]
pub(super) fn project_chain_owner(
    dependency: &RustDependency,
    function: &Function,
    result: &ProjectedType,
    invocation_scoped: bool,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    source_constants: &mut SourceConstantCache,
) -> Option<ProjectedItem> {
    let output = function.sig.output.as_ref()?;
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
    let ProjectedType::Foreign {
        rust_path,
        name,
        base_rust_path: _,
        arguments,
    } = result
    else {
        return None;
    };
    if !result.contains_opaque()
        && !structure
            .generics
            .params
            .iter()
            .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }))
    {
        return None;
    }
    let mut owner_generics = BTreeMap::new();
    owner_generics.insert("Self".to_owned(), result.clone());
    for (parameter, argument) in structure
        .generics
        .params
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .zip(arguments)
    {
        owner_generics.insert(parameter.name.clone(), argument.clone());
    }
    let (mut methods, _, _, _) = project_methods(
        &structure.impls,
        index,
        paths,
        public_paths,
        rust_path,
        &owner_generics,
        true,
        source_constants,
    );
    methods.retain_mut(|method| {
        if method.receiver.is_none() {
            return false;
        }
        method.chain_role = Some(if method.result == *result {
            ChainRole::Continue
        } else {
            ChainRole::Terminal
        });
        true
    });
    if !invocation_scoped
        && methods
            .iter()
            .all(|method| method.chain_role != Some(ChainRole::Terminal))
    {
        return None;
    }
    let source_path = paths.get(&path.id)?.path.clone();
    let namespace = dependency_namespace(
        dependency,
        &source_path[..source_path.len().saturating_sub(1)],
    );
    Some(ProjectedItem {
        namespace,
        name: name.clone(),
        rust_path: rust_path.clone(),
        docs: Some("chain-only; value must terminate within one expression".to_owned()),
        kind: ProjectedKind::ForeignType {
            constructor: None,
            methods,
            static_methods: Vec::new(),
            constants: Vec::new(),
            boundary: ProjectedBoundaryCapabilities::default(),
            fields: Vec::new(),
            borrowed_view: false,
            native_view_type: None,
            enum_payload: None,
            generic_parameters: Vec::new(),
            displayable: false,
            cloneable: false,
            send: false,
            sync: false,
        },
    })
}

pub(super) fn resolved_nominal_id(ty: &Type, index: &HashMap<Id, Item>) -> Option<Id> {
    let Type::ResolvedPath(path) = ty else {
        return None;
    };
    match index.get(&path.id).map(|item| &item.inner) {
        Some(ItemEnum::TypeAlias(alias)) => resolved_nominal_id(&alias.type_, index),
        Some(_) => Some(path.id),
        None => None,
    }
}

pub(super) fn alias_type_substitutions(
    ty: &Type,
    parameters: &[GenericParamDef],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<BTreeMap<String, ProjectedType>, String> {
    let arguments = type_arguments(ty);
    let mut substitutions = generics.clone();
    for (position, parameter) in parameters
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .enumerate()
    {
        let projected = if let Some(argument) = arguments.get(position) {
            project_type(argument, index, paths, generics)?
        } else if let GenericParamDefKind::Type {
            default: Some(default),
            ..
        } = &parameter.kind
        {
            project_type(default, index, paths, &substitutions)?
        } else {
            return Err(format!(
                "type alias parameter `{}` has no argument or default",
                parameter.name
            ));
        };
        substitutions.insert(parameter.name.clone(), projected);
    }
    Ok(substitutions)
}

pub(super) fn expand_output_alias(
    mut output: Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    mut generics: BTreeMap<String, ProjectedType>,
) -> Result<(Type, BTreeMap<String, ProjectedType>), String> {
    let mut visited = BTreeSet::new();
    loop {
        let Type::ResolvedPath(path) = &output else {
            return Ok((output, generics));
        };
        if !visited.insert(path.id) {
            return Err("recursive Rust type alias in projected output".to_owned());
        }
        let Some(Item {
            inner: ItemEnum::TypeAlias(alias),
            ..
        }) = index.get(&path.id)
        else {
            return Ok((output, generics));
        };
        generics =
            alias_type_substitutions(&output, &alias.generics.params, index, paths, &generics)?;
        output = alias.type_.clone();
    }
}
#[cfg(test)]
mod tests;
