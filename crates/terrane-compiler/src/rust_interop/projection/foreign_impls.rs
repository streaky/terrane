use std::collections::{BTreeMap, BTreeSet};

use rustdoc_types::{Id, ItemEnum, Type};

use super::{
    DeclinedItem, ProjectedDependency, ProjectedItem, ProjectedKind, ProjectedTraitOperation,
    ProjectedType, ReexportRustdoc, RustDependency, RustdocCrate, SourceConstantCache,
    canonicalize_rust_path, extern_rust_path, merge_projected_trait_operations,
    owner_trait_namespace, project_methods, project_type, rewrite_rust_bound_root,
    trait_fallback_namespace, trait_operation_docs,
};

// A trait's defining crate can implement it for a nominal owner from another crate.
// Those impls are absent from the owner's Rustdoc impl list.
#[expect(
    clippy::too_many_lines,
    reason = "cross-document impl discovery collects owner operations before resolving collisions"
)]
pub(super) fn project_foreign_owner_impls(
    projected: &mut [ProjectedDependency],
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    canonical_public_paths: &[BTreeMap<String, String>],
    reexports: &[ReexportRustdoc],
) -> BTreeMap<String, String> {
    let mut additions: Vec<Vec<ProjectedTraitOperation>> =
        (0..projected.len()).map(|_| Vec::new()).collect();
    let mut declined: Vec<Vec<DeclinedItem>> = (0..projected.len()).map(|_| Vec::new()).collect();
    let mut seen = BTreeSet::new();
    let mut owner_aliases = BTreeMap::new();
    for (document_index, (dependency, document, public_paths)) in rustdocs.iter().enumerate() {
        for (id, public_path) in public_paths {
            if let Some(summary) = document.paths.get(id) {
                let source = summary.path.join("::");
                let canonical = canonical_public_paths[document_index]
                    .get(&source)
                    .unwrap_or(&source);
                owner_aliases.insert(extern_rust_path(dependency, public_path), canonical.clone());
            }
        }
    }
    for reexport in reexports {
        for provider in &reexport.providers {
            let dependency = rustdocs[provider.dependency_index].0;
            for (id, public_path) in &provider.public_paths {
                if let Some(summary) = reexport.document.paths.get(id) {
                    let source = summary.path.join("::");
                    let canonical = canonical_public_paths[provider.dependency_index]
                        .get(&source)
                        .unwrap_or(&source);
                    owner_aliases
                        .insert(extern_rust_path(dependency, public_path), canonical.clone());
                }
            }
        }
    }
    let canonical_owner = |path: &str| {
        let constructor =
            crate::rust_ir::rust_type_constructor(path).unwrap_or_else(|| path.to_owned());
        let base = owner_aliases
            .get(&constructor)
            .map_or(constructor.as_str(), String::as_str);
        let mut normalized = canonicalize_rust_path(base);
        for (dependency, _, _) in rustdocs {
            normalized = rewrite_rust_bound_root(
                &normalized,
                &dependency.name.replace('-', "_"),
                &dependency.package.replace('-', "_"),
            );
        }
        normalized
    };
    let mut owners: BTreeMap<String, Vec<(usize, usize)>> = BTreeMap::new();
    for (dependency_index, dependency) in projected.iter().enumerate() {
        for (item_index, item) in dependency.items.iter().enumerate() {
            if matches!(
                item.kind,
                ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
            ) {
                owners
                    .entry(canonical_owner(&item.rust_path))
                    .or_default()
                    .push((dependency_index, item_index));
            }
        }
    }
    for (document_index, (dependency, document, public_paths)) in rustdocs.iter().enumerate() {
        let mut paths = document.paths.clone();
        for summary in paths.values_mut() {
            if let Some(path) = canonical_public_paths[document_index].get(&summary.path.join("::"))
            {
                summary.path = path.split("::").map(str::to_owned).collect();
            }
        }
        let mut implementations = document.index.iter().collect::<Vec<_>>();
        implementations.sort_by_key(|(id, _)| id.0);
        let mut source_constants = SourceConstantCache::default();
        for (id, item) in implementations {
            let ItemEnum::Impl(implementation) = &item.inner else {
                continue;
            };
            if item.crate_id != 0
                || implementation.is_negative
                || implementation.is_synthetic
                || implementation.blanket_impl.is_some()
            {
                continue;
            }
            let (Type::ResolvedPath(owner_path), Some(trait_path)) =
                (&implementation.for_, &implementation.trait_)
            else {
                continue;
            };
            if document
                .paths
                .get(&owner_path.id)
                .is_none_or(|summary| summary.crate_id == 0)
                || document
                    .paths
                    .get(&trait_path.id)
                    .is_none_or(|summary| summary.crate_id != 0)
            {
                continue;
            }
            let Some(summary) = paths.get(&owner_path.id) else {
                continue;
            };
            let identity = canonical_owner(&summary.path.join("::"));
            let Some(matches) = owners.get(&identity) else {
                // An owner outside the projected nominal inventory is not a candidate.
                continue;
            };
            let owner = project_type(
                &implementation.for_,
                &document.index,
                &paths,
                &BTreeMap::new(),
            )
            .and_then(|owner| {
                if matches!(owner, ProjectedType::Foreign { .. }) {
                    Ok(owner)
                } else {
                    Err("owner has no nominal native representation".to_owned())
                }
            });
            let owner = match owner {
                Ok(owner) => owner,
                Err(reason) => {
                    decline_owner_implementation(
                        &mut declined,
                        projected,
                        matches,
                        implementation,
                        document,
                        &reason,
                    );
                    continue;
                }
            };
            for &(owner_index, item_index) in matches {
                let owner_item = &projected[owner_index].items[item_index];
                let mut owner = owner.clone();
                if let ProjectedType::Foreign {
                    name,
                    rust_path,
                    base_rust_path,
                    ..
                } = &mut owner
                {
                    name.clone_from(&owner_item.name);
                    rust_path.clone_from(&owner_item.rust_path);
                    *base_rust_path = crate::rust_ir::rust_type_constructor(&owner_item.rust_path)
                        .unwrap_or_else(|| owner_item.rust_path.clone());
                }
                let generics = BTreeMap::from([("Self".to_owned(), owner)]);
                let (_, methods, _, failures) = project_methods(
                    &[*id],
                    &document.index,
                    &paths,
                    public_paths,
                    &owner_item.rust_path,
                    &generics,
                    false,
                    &mut source_constants,
                );
                let namespace = owner_trait_namespace(&owner_item.namespace, &owner_item.name);
                for (trait_path, _, _, docs, mut method) in methods {
                    let trait_path = extern_rust_path(dependency, &trait_path);
                    let native_path = format!(
                        "<{} as {trait_path}>::{}",
                        owner_item.rust_path, method.name
                    );
                    if !seen.insert((owner_index, namespace.clone(), native_path.clone())) {
                        continue;
                    }
                    method.native_path = Some(native_path.clone());
                    additions[owner_index].push(ProjectedTraitOperation {
                        fallback_namespace: trait_fallback_namespace(&namespace, &trait_path),
                        item: ProjectedItem {
                            namespace: namespace.clone(),
                            name: method.name.clone(),
                            rust_path: native_path.clone(),
                            docs: Some(trait_operation_docs(&native_path, docs.as_deref())),
                            kind: ProjectedKind::Function(method),
                        },
                    });
                }
                // Keep failure inventory on the owner where a source demand resolves.
                for (name, reason) in failures {
                    declined[owner_index].push(DeclinedItem {
                        rust_path: format!("{}::{name}", owner_item.rust_path),
                        reason,
                    });
                }
            }
        }
    }
    for ((dependency, operations), failures) in projected.iter_mut().zip(additions).zip(declined) {
        dependency.declined.extend(failures);
        merge_projected_trait_operations(
            &mut dependency.items,
            &mut dependency.declined,
            operations,
        );
    }
    for canonical in owner_aliases.values_mut() {
        *canonical = canonicalize_rust_path(canonical);
        for (dependency, _, _) in rustdocs {
            *canonical = rewrite_rust_bound_root(
                canonical,
                &dependency.package.replace('-', "_"),
                &dependency.name.replace('-', "_"),
            );
        }
    }
    owner_aliases
}

fn decline_owner_implementation(
    declined: &mut [Vec<DeclinedItem>],
    projected: &[ProjectedDependency],
    owners: &[(usize, usize)],
    implementation: &rustdoc_types::Impl,
    document: &RustdocCrate,
    reason: &str,
) {
    for &(dependency_index, item_index) in owners {
        let owner = &projected[dependency_index].items[item_index];
        for method in implementation
            .items
            .iter()
            .filter_map(|id| document.index.get(id))
        {
            if matches!(method.inner, ItemEnum::Function(_))
                && let Some(name) = &method.name
            {
                declined[dependency_index].push(DeclinedItem {
                    rust_path: format!("{}::{name}", owner.rust_path),
                    reason: format!("foreign trait implementation owner: {reason}"),
                });
            }
        }
    }
}
