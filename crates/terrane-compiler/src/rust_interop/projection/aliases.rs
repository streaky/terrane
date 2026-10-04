use super::{
    BTreeMap, DeclinedItem, GenericParamDefKind, HashMap, Id, Item, ItemEnum, ItemSummary,
    ProjectedDependency, ProjectedFunction, ProjectedItem, ProjectedKind, ProjectedTraitOperation,
    ProjectedType, ReexportRustdoc, RustDependency, RustdocCrate, Type, extern_rust_path,
    merge_projected_trait_operations, owner_trait_namespace, project_methods,
    project_struct_fields, project_type, render_rust_type, rustdoc_public_paths,
    trait_fallback_namespace,
};

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
    for (dependency_index, (dependency, source, public_paths)) in rustdocs.iter().enumerate() {
        let source_paths = &documents[dependency_index].1;
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
            let Ok(ProjectedType::Foreign {
                rust_path,
                base_rust_path,
                arguments,
                ..
            }) = project_type(&alias.type_, &source.index, source_paths, &alias_generics)
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
            let alias_path = extern_rust_path(dependency, public_path);
            let Some(item) = projected[dependency_index]
                .items
                .iter_mut()
                .find(|item| item.rust_path == alias_path)
            else {
                continue;
            };
            generics.insert(
                "Self".to_owned(),
                ProjectedType::Foreign {
                    rust_path: alias_path.clone(),
                    base_rust_path: alias_path.clone(),
                    arguments: Vec::new(),
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
                    matches!(implementation.for_, Type::Generic(_))
                        && implementation.blanket_impl.is_some()
                        || render_rust_type(
                            &implementation.for_,
                            &owner_document.index,
                            owner_paths,
                            &generics,
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

fn alias_generics(
    alias: &rustdoc_types::TypeAlias,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<BTreeMap<String, ProjectedType>> {
    let mut generics = BTreeMap::new();
    for parameter in &alias.generics.params {
        let GenericParamDefKind::Type {
            default: Some(default),
            ..
        } = &parameter.kind
        else {
            return None;
        };
        generics.insert(
            parameter.name.clone(),
            project_type(default, index, paths, &generics).ok()?,
        );
    }
    Some(generics)
}
