use super::{
    BTreeMap, DeclinedItem, GenericParamDefKind, HashMap, Id, Item, ItemEnum, ProjectedDependency,
    ProjectedFunction, ProjectedItem, ProjectedKind, ProjectedTraitOperation, ProjectedType,
    ReexportRustdoc, RustDependency, RustdocCrate, Type, extern_rust_path,
    merge_projected_trait_operations, owner_trait_namespace, project_methods,
    project_struct_fields, project_type, render_rust_type, rustdoc_public_paths,
    trait_fallback_namespace,
};

impl super::Projection {
    /// Expand recorded aliases without confusing nominal reexports with type aliases.
    /// A missing argument/default or a recursive alias has no canonical selection.
    pub(crate) fn canonical_native_alias(&self, ty: &ProjectedType) -> Option<ProjectedType> {
        let mut expanded = false;
        let canonical = self.resolve_native_alias(
            ty.clone(),
            &BTreeMap::new(),
            &mut super::BTreeSet::new(),
            &mut expanded,
        )?;
        expanded.then_some(canonical)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "alias expansion walks the complete existing projected type graph"
    )]
    fn resolve_native_alias(
        &self,
        ty: ProjectedType,
        bindings: &BTreeMap<String, ProjectedType>,
        active: &mut super::BTreeSet<String>,
        expanded: &mut bool,
    ) -> Option<ProjectedType> {
        if let ProjectedType::Generic(parameter) = ty {
            return project_type(
                &Type::Generic(parameter),
                &HashMap::new(),
                &HashMap::new(),
                bindings,
            )
            .ok();
        }
        if matches!(ty, ProjectedType::Associated(_)) {
            return None;
        }
        let mut resolved = ty;
        match &mut resolved {
            ProjectedType::Foreign {
                rust_path,
                base_rust_path,
                arguments,
                name,
            } => {
                for argument in arguments.iter_mut() {
                    self.resolve_alias_child(argument, bindings, active, expanded)?;
                }
                let base = if base_rust_path.is_empty() {
                    crate::rust_ir::rust_type_constructor(rust_path)?
                } else {
                    base_rust_path.clone()
                };
                let canonical_base = self.canonical_native_type(&base).into_owned();
                let aliases = || {
                    self.dependencies
                        .iter()
                        .flat_map(|dependency| &dependency.native_alias_identities)
                };
                let alias = aliases().find(|(path, _)| *path == &base).or_else(|| {
                    aliases().find(|(path, _)| self.canonical_native_type(path) == canonical_base)
                });
                if let Some((_, alias)) = alias {
                    if arguments.len() > alias.generic_parameters.len()
                        || crate::rust_ir::rust_type_arguments(rust_path).len() > arguments.len()
                        || active.contains(&canonical_base)
                    {
                        return None;
                    }
                    // Defaults are a separate expansion edge: A<A<bool>> can be finite,
                    // whereas an omitted argument whose default is A recurses forever.
                    let default_key = (arguments.len() < alias.generic_parameters.len())
                        .then(|| format!("default:{canonical_base}"));
                    if let Some(key) = &default_key
                        && !active.insert(key.clone())
                    {
                        return None;
                    }
                    let mut substitutions = BTreeMap::new();
                    let mut supplied = std::mem::take(arguments).into_iter();
                    for (position, parameter) in alias.generic_parameters.iter().enumerate() {
                        let argument = if let Some(argument) = supplied.next() {
                            argument
                        } else {
                            let default = alias.generic_defaults.get(position)?.as_ref()?;
                            self.resolve_native_alias(
                                default.clone(),
                                &substitutions,
                                active,
                                expanded,
                            )?
                        };
                        substitutions.insert(parameter.clone(), argument);
                    }
                    if let Some(key) = default_key {
                        active.remove(&key);
                    }
                    active.insert(canonical_base.clone());
                    *expanded = true;
                    let result = self.resolve_native_alias(
                        alias.native_type.clone(),
                        &substitutions,
                        active,
                        expanded,
                    );
                    active.remove(&canonical_base);
                    return result;
                }
                *base_rust_path = canonical_base;
                // The arguments are authoritative metadata, not parsed Rust fragments.
                // Preserve a legacy path without arguments rather than erasing its selection.
                *rust_path = if arguments.is_empty() {
                    self.canonical_native_type(rust_path).into_owned()
                } else {
                    format!(
                        "{base_rust_path}<{}>",
                        arguments
                            .iter()
                            .map(ProjectedType::rust_type)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                *name = super::instantiated_nominal_name(
                    base_rust_path.rsplit("::").next().unwrap_or(name),
                    rust_path,
                    arguments,
                );
            }
            ProjectedType::Sequence { rust_path, item }
            | ProjectedType::Set {
                rust_path, item, ..
            } => {
                let old = item.rust_type();
                self.resolve_alias_child(item, bindings, active, expanded)?;
                *rust_path =
                    self.resolve_alias_rust(rust_path, bindings, &[(old, item.rust_type())]);
            }
            ProjectedType::Mapping {
                rust_path,
                key,
                value,
                ..
            } => {
                let old_key = key.rust_type();
                let old_value = value.rust_type();
                self.resolve_alias_child(key, bindings, active, expanded)?;
                self.resolve_alias_child(value, bindings, active, expanded)?;
                *rust_path = self.resolve_alias_rust(
                    rust_path,
                    bindings,
                    &[(old_key, key.rust_type()), (old_value, value.rust_type())],
                );
            }
            ProjectedType::Optional(inner)
            | ProjectedType::Reference { inner, .. }
            | ProjectedType::AsyncIterationStep(inner) => {
                self.resolve_alias_child(inner, bindings, active, expanded)?;
            }
            ProjectedType::Tuple(items) => {
                for item in items {
                    self.resolve_alias_child(item, bindings, active, expanded)?;
                }
            }
            ProjectedType::InvocationScoped {
                rust_type,
                owned,
                name,
                ..
            } => {
                let old = owned.rust_type();
                self.resolve_alias_child(owned, bindings, active, expanded)?;
                *rust_type =
                    self.resolve_alias_rust(rust_type, bindings, &[(old, owned.rust_type())]);
                *name = owned.terrane_name();
            }
            ProjectedType::BoxedInterface {
                rust_path,
                trait_path,
                associated_type,
                ..
            } => {
                let mut replacements = Vec::new();
                if let Some(binding) = associated_type {
                    let old = binding.ty.rust_type();
                    self.resolve_alias_child(&mut binding.ty, bindings, active, expanded)?;
                    replacements.push((old, binding.ty.rust_type()));
                }
                *rust_path = self.resolve_alias_rust(rust_path, bindings, &replacements);
                *trait_path = self.resolve_alias_rust(trait_path, bindings, &replacements);
            }
            ProjectedType::Callback {
                rust_name,
                parameters,
                result,
                native_bound,
                native_result,
                native_substitutions,
                parameter_rust_types,
                ..
            } => {
                let mut replacements = Vec::new();
                for parameter in parameters {
                    let old = parameter.rust_type();
                    self.resolve_alias_child(parameter, bindings, active, expanded)?;
                    replacements.push((old, parameter.rust_type()));
                }
                let old = result.rust_type();
                self.resolve_alias_child(result, bindings, active, expanded)?;
                replacements.push((old, result.rust_type()));
                for substitution in native_substitutions.values_mut() {
                    self.resolve_alias_child(substitution, bindings, active, expanded)?;
                }
                for rust in std::iter::once(rust_name)
                    .chain(native_bound.iter_mut())
                    .chain(native_result.iter_mut())
                    .chain(parameter_rust_types.iter_mut())
                {
                    *rust = self.resolve_alias_rust(rust, bindings, &replacements);
                }
            }
            _ => {}
        }
        Some(resolved)
    }

    fn resolve_alias_child(
        &self,
        child: &mut ProjectedType,
        bindings: &BTreeMap<String, ProjectedType>,
        active: &mut super::BTreeSet<String>,
        expanded: &mut bool,
    ) -> Option<()> {
        let ty = std::mem::replace(child, ProjectedType::None);
        *child = self.resolve_native_alias(ty, bindings, active, expanded)?;
        Some(())
    }

    fn resolve_alias_rust(
        &self,
        rust: &str,
        bindings: &BTreeMap<String, ProjectedType>,
        replacements: &[(String, String)],
    ) -> String {
        struct Replacements(Vec<(syn::Type, syn::Type)>);
        impl syn::fold::Fold for Replacements {
            fn fold_type(&mut self, ty: syn::Type) -> syn::Type {
                if let Some((_, replacement)) = self.0.iter().find(|(old, _)| old == &ty) {
                    return replacement.clone();
                }
                syn::fold::fold_type(self, ty)
            }
        }
        let mut substitutions = Replacements(
            replacements
                .iter()
                .filter(|(old, new)| old != new)
                .filter_map(|(old, new)| {
                    Some((syn::parse_str(old).ok()?, syn::parse_str(new).ok()?))
                })
                .collect(),
        );
        let mut rust = rust.to_owned();
        if !substitutions.0.is_empty()
            && let Ok(ty) = syn::parse_str::<syn::Type>(&rust)
        {
            let ty = syn::fold::Fold::fold_type(&mut substitutions, ty);
            rust = quote::ToTokens::to_token_stream(&ty).to_string();
        }
        if !bindings.is_empty() {
            rust = crate::rust_ir::instantiate_rust_generics(
                &rust,
                &bindings
                    .iter()
                    .map(|(name, ty)| (name.clone(), ty.rust_type()))
                    .collect(),
            );
        }
        if (!bindings.is_empty() || !substitutions.0.is_empty())
            && let Ok(ty) = syn::parse_str::<syn::Type>(&rust)
        {
            let file: syn::File = syn::parse_quote!(type __TerraneNative = #ty;);
            let rendered = prettyplease::unparse(&file);
            if let Some(rendered) = rendered
                .strip_prefix("type __TerraneNative = ")
                .and_then(|rendered| rendered.strip_suffix(";\n"))
            {
                rendered.clone_into(&mut rust);
            }
        }
        self.canonical_native_type(&rust).into_owned()
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "closed alias enrichment binds one provider owner before projecting its complete member graph"
)]
pub(super) fn project_closed_alias_members(
    projected: &mut [ProjectedDependency],
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    reexports: &[ReexportRustdoc],
    public_types: &BTreeMap<String, String>,
) {
    let documents = rustdocs
        .iter()
        .map(|(_, document, _)| document)
        .chain(reexports.iter().map(|owner| &owner.document))
        .map(|document| {
            let mut paths = document.paths.clone();
            for summary in paths.values_mut() {
                if let Some(public) = public_types.get(&summary.path.join("::")) {
                    summary.path = public.split("::").map(str::to_owned).collect();
                }
            }
            (document, paths, rustdoc_public_paths(document))
        })
        .collect::<Vec<_>>();
    let mut owners = BTreeMap::new();
    for (document_index, (document, paths, _)) in documents.iter().enumerate() {
        for (id, item) in &document.index {
            if matches!(item.inner, ItemEnum::Struct(_))
                && let Some(summary) = paths.get(id)
            {
                owners
                    .entry(summary.path.join("::"))
                    .or_insert((document_index, *id));
            }
        }
    }
    let direct_sources = rustdocs
        .iter()
        .enumerate()
        .map(|(index, (dependency, document, paths))| (index, index, *dependency, document, paths));
    let reexport_sources = reexports.iter().enumerate().flat_map(|(index, reexport)| {
        reexport.providers.iter().map(move |provider| {
            (
                provider.dependency_index,
                rustdocs.len() + index,
                rustdocs[provider.dependency_index].0,
                &reexport.document,
                &provider.public_paths,
            )
        })
    });
    for (dependency_index, document_index, dependency, source, public_paths) in
        direct_sources.chain(reexport_sources)
    {
        let source_paths = &documents[document_index].1;
        let mut trait_operations = Vec::new();
        for (id, public_path) in public_paths {
            let Some(Item {
                inner: ItemEnum::TypeAlias(alias),
                ..
            }) = source.index.get(id)
            else {
                continue;
            };
            let Some(alias_generics) = alias_generics(alias, &source.index, source_paths) else {
                continue;
            };
            let alias_parameters = alias_generics.parameters;
            let generic_defaults = alias_generics.defaults;
            let Ok(native_type) = project_type(
                &alias.type_,
                &source.index,
                source_paths,
                &alias_generics.bindings,
            ) else {
                continue;
            };
            // Identity does not depend on finding the target's member document.
            let alias_path = extern_rust_path(dependency, public_path);
            let alias_segments = public_path
                .split("::")
                .map(str::to_owned)
                .collect::<Vec<_>>();
            let alias_namespace = super::dependency_namespace(
                dependency,
                &alias_segments[..alias_segments.len().saturating_sub(1)],
            );
            let alias_name = alias_segments.last().map_or("", String::as_str);
            if let Some(item) = projected[dependency_index].items.iter_mut().find(|item| {
                item.rust_path == alias_path
                    || item.namespace == alias_namespace && item.name == alias_name
            }) && let ProjectedKind::ForeignType {
                generic_parameters, ..
            } = &mut item.kind
                && let Ok(parameters) = super::projected_nominal_generic_parameters(
                    &alias.generics,
                    &alias_generics.bindings,
                    &source.index,
                    source_paths,
                )
            {
                *generic_parameters = parameters;
            }
            projected[dependency_index].native_alias_identities.insert(
                alias_path.clone(),
                super::ProjectedNativeAlias {
                    native_type: native_type.clone(),
                    generic_parameters: alias_parameters,
                    generic_defaults,
                },
            );
            // Member signatures remain templates in the alias declaration's binders.
            // Defaults select applications later; they never erase member generics.
            let ProjectedType::Foreign {
                rust_path,
                base_rust_path,
                arguments,
                ..
            } = native_type
            else {
                continue;
            };
            let Some((document_index, owner_id)) = owners.get(&base_rust_path) else {
                continue;
            };
            let (owner_document, owner_paths, owner_public_paths) = &documents[*document_index];
            let ItemEnum::Struct(structure) = &owner_document.index[owner_id].inner else {
                continue;
            };
            let mut generics = structure
                .generics
                .params
                .iter()
                .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
                .zip(arguments.iter())
                .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
                .collect::<BTreeMap<_, _>>();
            if generics.len()
                != structure
                    .generics
                    .params
                    .iter()
                    .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
                    .count()
            {
                continue;
            }
            let Some(item) = projected[dependency_index].items.iter_mut().find(|item| {
                item.rust_path == alias_path
                    || item.namespace == alias_namespace && item.name == alias_name
            }) else {
                continue;
            };
            let alias_arguments = alias
                .generics
                .params
                .iter()
                .map(|parameter| alias_generics.bindings[&parameter.name].clone())
                .collect::<Vec<_>>();
            let alias_native = if alias_arguments.is_empty() {
                alias_path.clone()
            } else {
                format!(
                    "{alias_path}<{}>",
                    alias_arguments
                        .iter()
                        .map(ProjectedType::rust_type)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            generics.insert(
                "Self".to_owned(),
                ProjectedType::Foreign {
                    rust_path: alias_native,
                    base_rust_path: alias_path.clone(),
                    arguments: alias_arguments,
                    name: item.name.clone(),
                },
            );
            let mut constants = BTreeMap::new();
            let impls = structure
                .impls
                .iter()
                .copied()
                .filter(|id| {
                    let Some(Item {
                        inner: ItemEnum::Impl(implementation),
                        ..
                    }) = owner_document.index.get(id)
                    else {
                        return false;
                    };
                    let implementation_generics = implementation_owner_generics(
                        implementation,
                        &owner_document.index,
                        &generics,
                    );
                    matches!(implementation.for_, Type::Generic(_))
                        && implementation.blanket_impl.is_some()
                        || render_rust_type(
                            &implementation.for_,
                            &owner_document.index,
                            owner_paths,
                            &implementation_generics,
                        )
                        .is_ok_and(|owner| owner == rust_path)
                })
                .collect::<Vec<_>>();
            let (mut inherent, traits, projected_constants, declines) = project_methods(
                &impls,
                &owner_document.index,
                owner_paths,
                owner_public_paths,
                &rust_path,
                &generics,
                false,
                &mut constants,
            );
            let ProjectedKind::ForeignType {
                methods,
                static_methods,
                fields,
                constants,
                ..
            } = &mut item.kind
            else {
                continue;
            };
            let alias = generics["Self"].clone();
            for method in &mut inherent {
                retarget_method(method, &rust_path, &alias);
            }
            let (instance, associated) = inherent
                .into_iter()
                .partition(|method| method.receiver.is_some());
            *methods = instance;
            *static_methods = associated;
            *constants = projected_constants;
            if !owner_document.index[owner_id]
                .attrs
                .iter()
                .any(|attribute| matches!(attribute, rustdoc_types::Attribute::NonExhaustive))
                && let Ok((projected_fields, false)) =
                    project_struct_fields(structure, &owner_document.index, owner_paths, &generics)
            {
                *fields = projected_fields;
            }
            let owner_namespace = owner_trait_namespace(&item.namespace, &item.name);
            for (trait_path, _, _, docs, mut method) in traits {
                retarget_method(&mut method, &rust_path, &alias);
                let operation_path = method
                    .native_path
                    .clone()
                    .expect("trait operations retain their qualified native path");
                trait_operations.push(ProjectedTraitOperation {
                    fallback_namespace: trait_fallback_namespace(&owner_namespace, &trait_path),
                    item: ProjectedItem {
                        namespace: owner_namespace.clone(),
                        name: method.name.clone(),
                        rust_path: operation_path,
                        docs,
                        kind: ProjectedKind::Function(method),
                    },
                });
            }
            projected[dependency_index]
                .declined
                .extend(declines.into_iter().map(|(name, reason)| DeclinedItem {
                    rust_path: format!("{alias_path}::{name}"),
                    reason,
                }));
        }
        merge_projected_trait_operations(
            &mut projected[dependency_index].items,
            &mut projected[dependency_index].declined,
            trait_operations,
        );
    }
    // Alias targets are collected from provider Rustdoc after ordinary item
    // normalization; apply every declared Cargo root to this metadata too.
    for (dependency, _, _) in rustdocs {
        if dependency.package == dependency.name {
            continue;
        }
        let package_root = dependency.package.replace('-', "_");
        let dependency_root = dependency.name.replace('-', "_");
        for projected_dependency in projected.iter_mut() {
            for alias in projected_dependency.native_alias_identities.values_mut() {
                super::rewrite_projected_rust_root(
                    &mut alias.native_type,
                    &package_root,
                    &dependency_root,
                );
                for default in alias.generic_defaults.iter_mut().flatten() {
                    super::rewrite_projected_rust_root(default, &package_root, &dependency_root);
                }
            }
        }
    }
}

fn retarget_method(method: &mut ProjectedFunction, target: &str, alias: &ProjectedType) {
    method.native_owner = Some(alias.rust_type());
    retarget_type(&mut method.result, target, alias);
    for parameter in &mut method.parameters {
        retarget_type(&mut parameter.ty, target, alias);
    }
}

fn retarget_type(ty: &mut ProjectedType, target: &str, alias: &ProjectedType) {
    if matches!(ty, ProjectedType::Foreign { .. }) && ty.rust_type() == target {
        *ty = alias.clone();
        return;
    }
    match ty {
        ProjectedType::InvocationScoped {
            owned,
            name,
            rust_type,
            ..
        } => {
            retarget_type(owned, target, alias);
            *rust_type = rust_type.replace(target, &alias.rust_type());
            if !matches!(owned.as_ref(), ProjectedType::Optional(_)) {
                *name = owned.terrane_name();
            }
        }
        ProjectedType::Optional(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. }
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Reference { inner, .. } => retarget_type(inner, target, alias),
        ProjectedType::Mapping { key, value, .. } => {
            retarget_type(key, target, alias);
            retarget_type(value, target, alias);
        }
        ProjectedType::Tuple(items)
        | ProjectedType::Foreign {
            arguments: items, ..
        } => {
            for item in items {
                retarget_type(item, target, alias);
            }
        }
        ProjectedType::Callback {
            parameters, result, ..
        } => {
            for parameter in parameters {
                retarget_type(parameter, target, alias);
            }
            retarget_type(result, target, alias);
        }
        _ => {}
    }
}

pub(super) fn implementation_owner_generics(
    implementation: &rustdoc_types::Impl,
    index: &HashMap<Id, Item>,
    owner_generics: &BTreeMap<String, ProjectedType>,
) -> BTreeMap<String, ProjectedType> {
    let mut generics = owner_generics.clone();
    if let Type::ResolvedPath(path) = &implementation.for_
        && let Some(declaration) = super::nominal_generics(index.get(&path.id))
    {
        for (parameter, argument) in declaration
            .params
            .iter()
            .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
            .zip(super::type_arguments(&implementation.for_))
        {
            if let Type::Generic(binder) = argument
                && let Some(selected) = owner_generics.get(&parameter.name)
            {
                generics.insert(binder.clone(), selected.clone());
            }
        }
    }
    generics
}

struct AliasGenerics {
    bindings: BTreeMap<String, ProjectedType>,
    parameters: Vec<String>,
    defaults: Vec<Option<ProjectedType>>,
}

fn alias_generics(
    alias: &rustdoc_types::TypeAlias,
    index: &super::HashMap<super::Id, super::Item>,
    paths: &super::HashMap<super::Id, super::ItemSummary>,
) -> Option<AliasGenerics> {
    let mut bindings = BTreeMap::new();
    let mut parameters = Vec::new();
    let mut defaults = Vec::new();
    for parameter in &alias.generics.params {
        let GenericParamDefKind::Type { default, .. } = &parameter.kind else {
            return None;
        };
        let projected_default = default
            .as_ref()
            .map(|default| project_type(default, index, paths, &bindings))
            .transpose()
            .ok()?;
        bindings.insert(
            parameter.name.clone(),
            ProjectedType::Generic(parameter.name.clone()),
        );
        parameters.push(parameter.name.clone());
        defaults.push(projected_default);
    }
    Some(AliasGenerics {
        bindings,
        parameters,
        defaults,
    })
}

#[cfg(test)]
mod tests {
    use super::super::ItemSummary;
    use super::super::{
        Containment, ProjectedNativeAlias, Projection, ProjectionResolution, ProjectionSource,
    };
    use super::*;

    fn foreign(base: &str, arguments: Vec<ProjectedType>) -> ProjectedType {
        let rust_path = if arguments.is_empty() {
            base.to_owned()
        } else {
            format!(
                "{base}<{}>",
                arguments
                    .iter()
                    .map(ProjectedType::rust_type)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        ProjectedType::Foreign {
            name: base.rsplit("::").next().unwrap().to_owned(),
            base_rust_path: base.to_owned(),
            rust_path,
            arguments,
        }
    }

    fn alias(
        native_type: ProjectedType,
        parameters: &[&str],
        defaults: Vec<Option<ProjectedType>>,
    ) -> ProjectedNativeAlias {
        ProjectedNativeAlias {
            native_type,
            generic_parameters: parameters
                .iter()
                .map(|parameter| (*parameter).to_owned())
                .collect(),
            generic_defaults: defaults,
        }
    }

    fn projection(aliases: Vec<(&str, ProjectedNativeAlias)>) -> Projection {
        Projection {
            cache_identity: "alias-resolver".to_owned(),
            content_hash: String::new(),
            dependencies: vec![ProjectedDependency {
                name: "facade".to_owned(),
                package: "facade".to_owned(),
                version: "1.0.0".to_owned(),
                items: Vec::new(),
                declined: Vec::new(),
                native_alias_identities: aliases
                    .into_iter()
                    .map(|(path, alias)| (path.to_owned(), alias))
                    .collect(),
                partial_declines: Vec::new(),
            }],
            native_owner_aliases: BTreeMap::new(),
            containment: Containment::Unavailable,
            bound_dependencies: Vec::new(),
            source: ProjectionSource::default(),
            probes: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution::default(),
            removed: Vec::new(),
        }
    }

    #[test]
    fn alias_arguments_override_defaults_and_dependent_defaults_close_in_order() {
        let generic = ProjectedType::Generic("T".to_owned());
        let projection = projection(vec![(
            "facade::Alias",
            alias(
                foreign(
                    "owner::Pair",
                    vec![generic.clone(), ProjectedType::Generic("U".to_owned())],
                ),
                &["T", "U"],
                vec![Some(ProjectedType::Bool), Some(generic)],
            ),
        )]);
        for (arguments, expected) in [
            (Vec::new(), vec![ProjectedType::Bool, ProjectedType::Bool]),
            (
                vec![ProjectedType::String],
                vec![ProjectedType::String, ProjectedType::String],
            ),
            (
                vec![ProjectedType::String, ProjectedType::Int],
                vec![ProjectedType::String, ProjectedType::Int],
            ),
        ] {
            assert_eq!(
                projection
                    .canonical_native_alias(&foreign("facade::Alias", arguments))
                    .unwrap()
                    .rust_type(),
                foreign("owner::Pair", expected).rust_type(),
            );
        }
    }

    #[test]
    fn nested_aliases_expand_across_dependencies_and_canonical_owner_paths() {
        let mut projection = projection(vec![(
            "facade::Outer",
            alias(
                foreign(
                    "provider::Record",
                    vec![foreign(
                        "other::Inner",
                        vec![ProjectedType::Generic("T".to_owned())],
                    )],
                ),
                &["T"],
                vec![None],
            ),
        )]);
        let mut other = projection.dependencies[0].clone();
        other.name = "other".to_owned();
        other.native_alias_identities = BTreeMap::from([(
            "other::Inner".to_owned(),
            alias(
                foreign(
                    "provider::Value",
                    vec![ProjectedType::Generic("U".to_owned())],
                ),
                &["U"],
                vec![None],
            ),
        )]);
        projection.dependencies.push(other);
        projection.native_owner_aliases = BTreeMap::from([
            ("internal::Outer".to_owned(), "facade::Outer".to_owned()),
            ("provider::Record".to_owned(), "public::Record".to_owned()),
            ("provider::Value".to_owned(), "public::Value".to_owned()),
        ]);
        let input = foreign("internal::Outer", vec![ProjectedType::Bool]);
        assert_eq!(
            projection
                .canonical_native_alias(&input)
                .unwrap()
                .rust_type(),
            "public::Record<public::Value<bool>>",
        );
        let ProjectedType::Tuple(items) = projection
            .canonical_native_alias(&ProjectedType::Tuple(vec![input.clone(), input]))
            .unwrap()
        else {
            panic!("alias resolution must preserve the tuple constructor");
        };
        assert_eq!(
            items
                .iter()
                .map(ProjectedType::rust_type)
                .collect::<Vec<_>>(),
            [
                "public::Record<public::Value<bool>>",
                "public::Record<public::Value<bool>>",
            ],
        );
    }

    #[test]
    fn nested_container_aliases_preserve_constructor_and_concrete_arguments() {
        let projection = projection(vec![(
            "facade::Alias",
            alias(
                ProjectedType::Sequence {
                    rust_path: "std::vec::Vec<owner::Target<T>>".to_owned(),
                    item: Box::new(foreign(
                        "owner::Target",
                        vec![ProjectedType::Generic("T".to_owned())],
                    )),
                },
                &["T"],
                vec![None],
            ),
        )]);
        let canonical = projection
            .canonical_native_alias(&ProjectedType::Optional(Box::new(foreign(
                "facade::Alias",
                vec![ProjectedType::Bool],
            ))))
            .unwrap();
        assert_eq!(
            canonical.rust_type(),
            "Option<std::vec::Vec<owner::Target<bool>>>"
        );
    }

    #[test]
    fn unresolved_arguments_and_unrecorded_nominal_types_have_no_alias_selection() {
        let projection = projection(vec![(
            "facade::Alias",
            alias(
                foreign(
                    "owner::Record",
                    vec![ProjectedType::Generic("T".to_owned())],
                ),
                &["T"],
                vec![None],
            ),
        )]);
        for input in [
            foreign("owner::Record", vec![ProjectedType::Bool]),
            foreign("facade::Alias", Vec::new()),
            foreign(
                "facade::Alias",
                vec![ProjectedType::Bool, ProjectedType::Int],
            ),
            foreign(
                "facade::Alias",
                vec![ProjectedType::Generic("T".to_owned())],
            ),
        ] {
            assert_eq!(projection.canonical_native_alias(&input), None);
        }
        let mut missing_metadata = foreign("facade::Alias", vec![ProjectedType::Bool]);
        if let ProjectedType::Foreign { arguments, .. } = &mut missing_metadata {
            arguments.clear();
        }
        assert_eq!(projection.canonical_native_alias(&missing_metadata), None);
    }

    #[test]
    fn cycles_fail_even_when_they_expand_arguments_or_hide_inside_defaults() {
        for aliases in [
            vec![(
                "facade::A",
                alias(foreign("facade::A", Vec::new()), &[], Vec::new()),
            )],
            vec![
                (
                    "facade::A",
                    alias(foreign("facade::B", Vec::new()), &[], Vec::new()),
                ),
                (
                    "facade::B",
                    alias(foreign("facade::A", Vec::new()), &[], Vec::new()),
                ),
            ],
            vec![(
                "facade::A",
                alias(
                    foreign(
                        "facade::A",
                        vec![ProjectedType::Optional(Box::new(ProjectedType::Generic(
                            "T".to_owned(),
                        )))],
                    ),
                    &["T"],
                    vec![Some(ProjectedType::Bool)],
                ),
            )],
            vec![(
                "facade::A",
                alias(
                    foreign(
                        "owner::Record",
                        vec![ProjectedType::Generic("T".to_owned())],
                    ),
                    &["T"],
                    vec![Some(foreign("facade::A", Vec::new()))],
                ),
            )],
        ] {
            assert_eq!(
                projection(aliases).canonical_native_alias(&foreign("facade::A", Vec::new())),
                None
            );
        }
    }

    #[test]
    fn a_default_can_apply_the_same_alias_with_an_explicit_finite_argument() {
        let projection = projection(vec![(
            "facade::A",
            alias(
                foreign(
                    "owner::Record",
                    vec![ProjectedType::Generic("T".to_owned())],
                ),
                &["T"],
                vec![Some(foreign("facade::A", vec![ProjectedType::Bool]))],
            ),
        )]);
        assert_eq!(
            projection
                .canonical_native_alias(&foreign("facade::A", Vec::new()))
                .unwrap()
                .rust_type(),
            "owner::Record<owner::Record<bool>>",
        );
    }

    fn defaulted_alias_fixture() -> (RustDependency, RustdocCrate, BTreeMap<Id, String>) {
        let dependency = RustDependency {
            name: "facade".to_owned(),
            package: "facade".to_owned(),
            version: "=1.0.0".to_owned(),
            features: Vec::new(),
            default_features: true,
            target: None,
            effects: Vec::new(),
        };
        let alias_id = Id(1);
        let target_id = Id(2);
        let alias_item = Item {
            id: alias_id,
            crate_id: 0,
            name: Some("Alias".to_owned()),
            span: None,
            visibility: rustdoc_types::Visibility::Public,
            docs: None,
            links: HashMap::new(),
            attrs: Vec::new(),
            deprecation: None,
            inner: ItemEnum::TypeAlias(rustdoc_types::TypeAlias {
                type_: Type::ResolvedPath(rustdoc_types::Path {
                    path: "owner::Record".to_owned(),
                    id: target_id,
                    args: Some(Box::new(rustdoc_types::GenericArgs::AngleBracketed {
                        args: vec![rustdoc_types::GenericArg::Type(Type::Generic(
                            "T".to_owned(),
                        ))],
                        constraints: Vec::new(),
                    })),
                }),
                generics: rustdoc_types::Generics {
                    params: vec![rustdoc_types::GenericParamDef {
                        name: "T".to_owned(),
                        kind: GenericParamDefKind::Type {
                            bounds: Vec::new(),
                            default: Some(Type::Primitive("bool".to_owned())),
                            is_synthetic: false,
                        },
                    }],
                    where_predicates: Vec::new(),
                },
            }),
        };
        let mut root = alias_item.clone();
        root.id = Id(0);
        root.name = Some("facade".to_owned());
        root.inner = ItemEnum::Module(rustdoc_types::Module {
            is_crate: true,
            items: vec![alias_id],
            is_stripped: false,
        });
        let document = RustdocCrate {
            root: Id(0),
            crate_version: Some("1.0.0".to_owned()),
            includes_private: false,
            index: HashMap::from([(Id(0), root), (alias_id, alias_item)]),
            paths: HashMap::from([(
                target_id,
                ItemSummary {
                    crate_id: 1,
                    path: vec!["owner".to_owned(), "Record".to_owned()],
                    kind: rustdoc_types::ItemKind::Struct,
                },
            )]),
            external_crates: HashMap::new(),
            target: rustdoc_types::Target {
                triple: "x86_64-unknown-linux-gnu".to_owned(),
                target_features: Vec::new(),
            },
            format_version: rustdoc_types::FORMAT_VERSION,
        };
        (
            dependency,
            document,
            BTreeMap::from([(alias_id, "facade::Alias".to_owned())]),
        )
    }

    #[test]
    fn alias_identity_is_recorded_without_the_target_owner_document() {
        let (dependency, document, public_paths) = defaulted_alias_fixture();
        let mut projection = projection(Vec::new());
        project_closed_alias_members(
            &mut projection.dependencies,
            &[(&dependency, document, public_paths)],
            &[],
            &BTreeMap::new(),
        );
        assert!(projection.dependencies[0].items.is_empty());
        assert_eq!(
            projection
                .canonical_native_alias(&foreign("facade::Alias", Vec::new()))
                .unwrap()
                .rust_type(),
            "owner::Record<bool>",
        );
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "the regression constructs one complete Rustdoc alias/owner/member fixture"
    )]
    fn alias_member_templates_rebind_different_declaration_and_impl_parameter_names() {
        let (dependency, mut document, public_paths) = defaulted_alias_fixture();
        let owner_generics = rustdoc_types::Generics {
            params: vec![rustdoc_types::GenericParamDef {
                name: "B".to_owned(),
                kind: GenericParamDefKind::Type {
                    bounds: Vec::new(),
                    default: None,
                    is_synthetic: false,
                },
            }],
            where_predicates: Vec::new(),
        };
        let mut owner = document.index[&Id(1)].clone();
        owner.id = Id(2);
        owner.crate_id = 1;
        owner.name = Some("Record".to_owned());
        owner.inner = ItemEnum::Struct(rustdoc_types::Struct {
            kind: rustdoc_types::StructKind::Unit,
            generics: owner_generics.clone(),
            impls: vec![Id(3)],
        });
        let mut implementation = owner.clone();
        implementation.id = Id(3);
        implementation.name = None;
        let mut impl_generics = owner_generics;
        impl_generics.params[0].name = "U".to_owned();
        implementation.inner = ItemEnum::Impl(rustdoc_types::Impl {
            is_unsafe: false,
            generics: impl_generics,
            provided_trait_methods: Vec::new(),
            trait_: None,
            for_: Type::ResolvedPath(rustdoc_types::Path {
                path: "owner::Record".to_owned(),
                id: Id(2),
                args: Some(Box::new(rustdoc_types::GenericArgs::AngleBracketed {
                    args: vec![rustdoc_types::GenericArg::Type(Type::Generic(
                        "U".to_owned(),
                    ))],
                    constraints: Vec::new(),
                })),
            }),
            items: vec![Id(4)],
            is_negative: false,
            is_synthetic: false,
            blanket_impl: None,
        });
        let mut method = owner.clone();
        method.id = Id(4);
        method.name = Some("value".to_owned());
        method.inner = ItemEnum::Function(rustdoc_types::Function {
            sig: rustdoc_types::FunctionSignature {
                inputs: vec![(
                    "self".to_owned(),
                    Type::BorrowedRef {
                        lifetime: None,
                        is_mutable: false,
                        type_: Box::new(Type::Generic("Self".to_owned())),
                    },
                )],
                output: Some(Type::Generic("U".to_owned())),
                is_c_variadic: false,
            },
            generics: rustdoc_types::Generics {
                params: Vec::new(),
                where_predicates: Vec::new(),
            },
            header: rustdoc_types::FunctionHeader {
                is_const: false,
                is_unsafe: false,
                is_async: false,
                abi: rustdoc_types::Abi::Rust,
            },
            has_body: true,
        });
        document
            .index
            .extend([(Id(2), owner), (Id(3), implementation), (Id(4), method)]);
        let mut projection = projection(Vec::new());
        projection.dependencies[0].items.push(ProjectedItem {
            namespace: "/deps/facade".to_owned(),
            name: "Alias".to_owned(),
            rust_path: "facade::Alias".to_owned(),
            docs: None,
            kind: ProjectedKind::ForeignType {
                constructor: None,
                methods: Vec::new(),
                static_methods: Vec::new(),
                constants: Vec::new(),
                boundary: super::super::ProjectedBoundaryCapabilities::default(),
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
        });
        project_closed_alias_members(
            &mut projection.dependencies,
            &[(&dependency, document, public_paths)],
            &[],
            &BTreeMap::new(),
        );
        let ProjectedKind::ForeignType {
            methods,
            generic_parameters,
            ..
        } = &projection.dependencies[0].items[0].kind
        else {
            panic!("the alias retains its foreign member surface");
        };
        assert_eq!(methods.len(), 1);
        assert_eq!(methods[0].name, "value");
        assert_eq!(methods[0].result, ProjectedType::Generic("T".to_owned()));
        assert_eq!(generic_parameters.len(), 1);
        assert_eq!(generic_parameters[0].name, "T");
        assert_eq!(generic_parameters[0].default, Some(ProjectedType::Bool));
        assert_eq!(
            projection.dependencies[0].native_alias_identities["facade::Alias"]
                .native_type
                .rust_type(),
            "owner::Record<T>",
        );
        assert_eq!(
            projection
                .canonical_native_alias(&foreign("facade::Alias", vec![ProjectedType::Int]))
                .unwrap()
                .rust_type(),
            "owner::Record<i64>",
        );
    }

    #[test]
    fn public_reexport_aliases_record_the_defining_documents_generic_defaults() {
        let (dependency, document, _) = defaulted_alias_fixture();
        let mut facade = document.clone();
        facade.index.remove(&Id(1));
        let reexport = ReexportRustdoc {
            document,
            providers: vec![super::super::ReexportProvider {
                dependency_index: 0,
                public_paths: BTreeMap::from([(Id(1), "facade::extract::Alias".to_owned())]),
                canonical_public_paths: BTreeMap::new(),
                rust_path_aliases: BTreeMap::new(),
            }],
        };
        let mut projection = projection(Vec::new());
        project_closed_alias_members(
            &mut projection.dependencies,
            &[(&dependency, facade, BTreeMap::new())],
            &[reexport],
            &BTreeMap::new(),
        );
        let metadata =
            &projection.dependencies[0].native_alias_identities["facade::extract::Alias"];
        assert_eq!(metadata.generic_parameters, ["T"]);
        assert_eq!(metadata.generic_defaults, [Some(ProjectedType::Bool)]);
        assert_eq!(metadata.native_type.rust_type(), "owner::Record<T>");
        assert_eq!(
            projection
                .canonical_native_alias(&foreign("facade::extract::Alias", Vec::new()))
                .unwrap()
                .rust_type(),
            "owner::Record<bool>",
        );
        assert_eq!(
            projection
                .canonical_native_alias(&foreign(
                    "facade::extract::Alias",
                    vec![ProjectedType::String]
                ))
                .unwrap()
                .rust_type(),
            "owner::Record<String>",
        );
    }

    #[test]
    fn aliases_with_required_parameters_retain_the_source_declaration_binders() {
        let (dependency, mut document, public_paths) = defaulted_alias_fixture();
        let ItemEnum::TypeAlias(alias) = &mut document.index.get_mut(&Id(1)).unwrap().inner else {
            panic!("fixture contains a type alias");
        };
        let GenericParamDefKind::Type { default, .. } = &mut alias.generics.params[0].kind else {
            panic!("fixture contains a type parameter");
        };
        *default = None;
        let projected = super::super::project_rustdoc(
            &dependency,
            &document,
            &public_paths,
            &BTreeMap::new(),
            false,
        );
        let item = projected
            .items
            .iter()
            .find(|item| item.name == "Alias")
            .unwrap();
        let ProjectedKind::ForeignType {
            generic_parameters, ..
        } = &item.kind
        else {
            panic!("alias source declaration is retained");
        };
        assert_eq!(generic_parameters.len(), 1);
        assert_eq!(generic_parameters[0].name, "T");
        assert_eq!(generic_parameters[0].default, None);
    }
}
