//! Rustdoc interface projection decisions.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;

use rustdoc_types::{
    Crate as RustdocCrate, GenericBound, GenericParamDefKind, Id, Item, ItemEnum, ItemSummary,
};

use super::{
    DeclinedItem, ProjectedAssociatedType, ProjectedDependency, ProjectedInterface,
    ProjectedInterfaceMethod, ProjectedItem, ProjectedKind, ProjectedParameter,
    ProjectedSupertrait, ProjectedType, Receiver, RustDependency, project_function_with_generics,
    project_type, render_generic_bound, render_resolved_path, type_contains_borrowed_ref,
};

pub(super) fn project_interface(
    declaration: &rustdoc_types::Trait,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    rust_path: &str,
) -> Result<ProjectedInterface, String> {
    project_interface_inner(declaration, index, paths, rust_path)
}

#[expect(
    clippy::too_many_lines,
    reason = "interface admission keeps inherited and member-level evidence together"
)]
fn project_interface_inner(
    declaration: &rustdoc_types::Trait,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    rust_path: &str,
) -> Result<ProjectedInterface, String> {
    if !declaration.generics.params.is_empty() {
        return Err("trait has generic or lifetime parameters".to_owned());
    }
    if !declaration.generics.where_predicates.is_empty() {
        return Err("trait has unsupported where predicates".to_owned());
    }
    let mut send = false;
    let mut sync = false;
    let mut requires_drop = false;
    let mut methods = Vec::new();
    let mut declined_methods = Vec::new();
    let mut associated_type = None;
    let mut supertraits = Vec::new();
    for bound in &declaration.bounds {
        let GenericBound::TraitBound { trait_, .. } = bound else {
            return Err("trait has a non-trait supertrait".to_owned());
        };
        let path = paths
            .get(&trait_.id)
            .map(|summary| summary.path.join("::"))
            .ok_or_else(|| "trait has an unresolved supertrait".to_owned())?;
        match path.as_str() {
            "core::marker::Send" | "std::marker::Send" => send = true,
            "core::marker::Sync" | "std::marker::Sync" => sync = true,
            "core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop" => {
                requires_drop = true;
            }
            _ => {
                let Some(Item {
                    inner: ItemEnum::Trait(supertrait),
                    ..
                }) = index.get(&trait_.id)
                else {
                    return Err(format!("supertrait `{path}` does not resolve to a trait"));
                };
                let supertrait_rust_path =
                    render_resolved_path(trait_, index, paths, &BTreeMap::new())?;
                let projected =
                    project_interface_inner(supertrait, index, paths, &supertrait_rust_path)?;
                send |= projected.send;
                sync |= projected.sync;
                requires_drop |= projected.requires_drop;
                if let Some(inherited_type) = projected.associated_type {
                    match &associated_type {
                        None => associated_type = Some(inherited_type),
                        Some(existing) if existing == &inherited_type => {}
                        Some(_) => {
                            return Err(
                                "trait closure contains more than one associated type".to_owned()
                            );
                        }
                    }
                }
                for method in projected.methods {
                    if !methods.contains(&method) {
                        methods.push(method);
                    }
                }
                for declined in projected.declined_methods {
                    if !declined_methods.contains(&declined) {
                        declined_methods.push(declined);
                    }
                }
                let direct = ProjectedSupertrait {
                    namespace: String::new(),
                    name: trait_
                        .path
                        .rsplit("::")
                        .next()
                        .unwrap_or(&trait_.path)
                        .to_owned(),
                    rust_path: supertrait_rust_path,
                };
                if !supertraits.contains(&direct) {
                    supertraits.push(direct);
                }
                for inherited in projected.supertraits {
                    if !supertraits.contains(&inherited) {
                        supertraits.push(inherited);
                    }
                }
            }
        }
    }
    for id in &declaration.items {
        let Some(item) = index.get(id) else {
            return Err("trait member is missing from rustdoc".to_owned());
        };
        let name = item
            .name
            .as_deref()
            .ok_or_else(|| "trait has an unnamed member".to_owned())?;
        let function = match &item.inner {
            ItemEnum::Function(function) => function,
            ItemEnum::AssocType {
                generics,
                bounds,
                type_,
                ..
            } => {
                if associated_type.is_some() {
                    return Err("trait closure contains more than one associated type".to_owned());
                }
                if !generics.params.is_empty() || !generics.where_predicates.is_empty() {
                    return Err(format!("associated type `{name}` is generic"));
                }
                if type_.is_some() {
                    return Err(format!("associated type `{name}` has a default"));
                }
                let bounds = bounds
                    .iter()
                    .map(|bound| render_generic_bound(bound, &[], index, paths, &BTreeMap::new()))
                    .collect::<Result<Vec<_>, _>>()?;
                associated_type = Some(ProjectedAssociatedType {
                    name: name.to_owned(),
                    rust_path: format!("{rust_path}::{name}"),
                    bounds,
                    docs: item.docs.clone(),
                });
                continue;
            }
            _ => return Err(format!("trait member `{name}` is not a receiver method")),
        };
        let provided = function.has_body;
        let member_path = format!("trait::{name}");
        if provided && !function.generics.where_predicates.is_empty() {
            declined_methods.push(DeclinedItem {
                rust_path: member_path,
                reason: "provided method has unsupported where predicates".to_owned(),
            });
            continue;
        }
        if function
            .sig
            .inputs
            .first()
            .is_none_or(|(name, _)| name != "self")
        {
            if provided {
                declined_methods.push(DeclinedItem {
                    rust_path: member_path,
                    reason: "provided associated function is not a receiver method".to_owned(),
                });
                continue;
            }
            return Err(format!("trait member `{name}` is an associated function"));
        }
        let supplied = associated_type
            .as_ref()
            .map(|associated| {
                BTreeMap::from([(
                    format!("Self::{}", associated.name),
                    ProjectedType::Associated(format!("Self::{}", associated.name)),
                )])
            })
            .unwrap_or_default();
        if let Some(parameter) = function.generics.params.first() {
            return Err(format!(
                "trait member `{name}`: open generic `{}`",
                parameter.name
            ));
        }
        let projected = match project_function_with_generics(
            function,
            index,
            paths,
            &BTreeMap::new(),
            Some(name),
            &supplied,
            false,
        ) {
            Ok(projected) => projected,
            Err(reason) if provided => {
                declined_methods.push(DeclinedItem {
                    rust_path: member_path,
                    reason,
                });
                continue;
            }
            Err(reason) => return Err(format!("trait member `{name}`: {reason}")),
        };
        if !provided
            && function
                .sig
                .output
                .as_ref()
                .is_some_and(type_contains_borrowed_ref)
        {
            return Err(format!(
                "trait member `{name}`: required borrowed results cannot be implemented by an owned source result"
            ));
        }
        if !provided && projected.error.is_some() {
            return Err(format!(
                "trait member `{name}`: required Result-returning methods are deferred"
            ));
        }
        if !provided
            && projected
                .parameters
                .iter()
                .any(|parameter| parameter.borrowed)
        {
            return Err(format!(
                "trait member `{name}`: required borrowed parameters are deferred"
            ));
        }
        debug_assert!(projected.receiver.is_some());
        methods.push(ProjectedInterfaceMethod {
            function: projected,
            provided,
            owner_rust_path: Some(rust_path.to_owned()),
            docs: item.docs.clone(),
        });
    }
    if methods.iter().any(|method| method.function.is_async) && !send {
        return Err(
            "asynchronous projected methods require a `Send` interface so cancellation cleanup can own and detach receiver state"
                .to_owned(),
        );
    }
    if methods.is_empty() {
        return Err("trait has no projectable receiver methods".to_owned());
    }
    Ok(ProjectedInterface {
        is_unsafe: declaration.is_unsafe,
        methods,
        associated_type,
        supertraits,
        send,
        sync,
        requires_drop,
        declined_methods,
    })
}
pub(super) fn projected_interface_impl_question(
    item: &ProjectedItem,
    interface: &ProjectedInterface,
) -> crate::rust_interop::ImplQuestion {
    let associated_bounds = interface.associated_type.as_ref().map(|associated| {
        let bounds = associated.bounds.join(" + ");
        if bounds.is_empty() {
            "'static".to_owned()
        } else {
            format!("'static + {bounds}")
        }
    });
    let generic = associated_bounds
        .as_ref()
        .map_or_else(String::new, |bounds| {
            format!("<TerraneAssociated: {bounds}>")
        });
    let implementation = if associated_bounds.is_some() {
        "TerraneProjectionImpl<TerraneAssociated>"
    } else {
        "TerraneProjectionImpl"
    };
    let mut source = if associated_bounds.is_some() {
        "struct TerraneProjectionImpl<T>(std::marker::PhantomData<fn() -> T>);\n".to_owned()
    } else {
        "struct TerraneProjectionImpl;\n".to_owned()
    };
    if interface.requires_drop {
        writeln!(
            source,
            "impl{generic} Drop for {implementation} {{ fn drop(&mut self) {{}} }}"
        )
        .expect("writing to a string cannot fail");
    }
    let mut traits = interface
        .supertraits
        .iter()
        .map(|supertrait| supertrait.rust_path.as_str())
        .collect::<Vec<_>>();
    traits.push(&item.rust_path);
    for trait_path in traits {
        writeln!(source, "impl{generic} {trait_path} for {implementation} {{")
            .expect("writing to a string cannot fail");
        if let Some(associated) = &interface.associated_type
            && associated
                .rust_path
                .strip_suffix(&format!("::{}", associated.name))
                == Some(trait_path)
        {
            writeln!(source, "type {} = TerraneAssociated;", associated.name)
                .expect("writing to a string cannot fail");
        }
        for method in interface.methods.iter().filter(|method| {
            !method.provided && method.owner_rust_path.as_deref() == Some(trait_path)
        }) {
            let function = &method.function;
            if function.is_async {
                source.push_str("async ");
            }
            write!(source, "fn {}(", function.name).expect("writing to a string cannot fail");
            source.push_str(match function.receiver {
                Some(Receiver::Borrow) => "&self",
                Some(Receiver::MutableBorrow) => "&mut self",
                Some(Receiver::Move) => "self",
                Some(Receiver::BoxedMove) => "self: Box<Self>",
                None => "",
            });
            for parameter in &function.parameters {
                let mut ty = parameter.ty.rust_type();
                if parameter.borrowed {
                    ty = format!(
                        "&{}{}",
                        if parameter.mutable_borrow { "mut " } else { "" },
                        ty
                    );
                }
                write!(source, ", {}: {ty}", parameter.name)
                    .expect("writing to a string cannot fail");
            }
            source.push(')');
            let result = function.error.as_ref().map_or_else(
                || function.result.rust_type(),
                |error| format!("Result<{}, {error}>", function.result.rust_type()),
            );
            if result != "()" {
                write!(source, " -> {result}").expect("writing to a string cannot fail");
            }
            source.push_str(" { todo!() }\n");
        }
        source.push_str("}\n");
    }
    source.push_str("fn main() {}\n");
    crate::rust_interop::ImplQuestion {
        label: item.rust_path.clone(),
        source,
    }
}

pub(super) struct ProjectedTraitOperation {
    pub(super) item: ProjectedItem,
    pub(super) fallback_namespace: String,
}

pub(super) fn owner_trait_namespace(owner_namespace: &str, owner_name: &str) -> String {
    format!(
        "{owner_namespace}/{}",
        owner_name.to_lowercase().replace('_', "-")
    )
}

pub(super) fn trait_fallback_namespace(owner_namespace: &str, trait_path: &str) -> String {
    let segments = trait_path
        .split("::")
        .map(|segment| {
            let mut normalized = String::new();
            let mut separator = false;
            for character in segment.chars() {
                if character.is_ascii_alphanumeric() {
                    normalized.push(character.to_ascii_lowercase());
                    separator = false;
                } else if !normalized.is_empty() && !separator {
                    normalized.push('-');
                    separator = true;
                }
            }
            normalized.trim_end_matches('-').to_owned()
        })
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("/");
    format!("{owner_namespace}/trait/{segments}")
}

pub(super) fn trait_operation_docs(rust_path: &str, docs: Option<&str>) -> String {
    let provenance =
        format!("Projected Rust trait operation `{rust_path}` for this concrete owner.");
    docs.map_or(provenance.clone(), |docs| format!("{provenance}\n\n{docs}"))
}

fn attach_unique_trait_operation(
    items: &mut [ProjectedItem],
    operation: &ProjectedTraitOperation,
    primary_counts: &BTreeMap<(String, String), usize>,
) -> bool {
    let key = (
        operation.item.namespace.clone(),
        operation.item.name.clone(),
    );
    if primary_counts.get(&key).copied().unwrap_or_default() != 1 {
        return false;
    }
    let ProjectedKind::Function(function) = &operation.item.kind else {
        return false;
    };
    let Some((methods, static_methods)) = items.iter_mut().find_map(|item| {
        if owner_trait_namespace(&item.namespace, &item.name) != operation.item.namespace {
            return None;
        }
        match &mut item.kind {
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => Some((methods, static_methods)),
            _ => None,
        }
    }) else {
        return false;
    };
    let instance = function.receiver.is_some();
    let candidates = if instance {
        &mut *methods
    } else {
        &mut *static_methods
    };
    if candidates
        .iter()
        .any(|candidate| candidate.name == function.name)
    {
        return false;
    }
    let mut function = function.clone();
    if function.native_path.is_none() {
        function.native_path = Some(operation.item.rust_path.clone());
    }
    // Borrowed trait operations historically also have an explicit-receiver
    // associated form, e.g. Row::try_get(row, column). Keep both contracts;
    // neither is an authored-class terrane_static_* shim.
    if let Some(receiver @ (Receiver::Borrow | Receiver::MutableBorrow)) = function.receiver
        && !static_methods
            .iter()
            .any(|candidate| candidate.name == function.name)
        && let Some(owner) = function.native_owner.as_deref()
    {
        let mut associated = function.clone();
        associated.receiver = None;
        let base = owner.split_once('<').map_or(owner, |(base, _)| base);
        associated.parameters.insert(
            0,
            ProjectedParameter {
                borrowed_str: false,
                name: "receiver".to_owned(),
                ty: ProjectedType::Foreign {
                    rust_path: owner.to_owned(),
                    base_rust_path: base.to_owned(),
                    name: base.rsplit("::").next().unwrap_or(base).to_owned(),
                    arguments: Vec::new(),
                },
                borrowed: true,
                mutable_borrow: receiver == Receiver::MutableBorrow,
                generic_parameter: None,
                generic_bounds: Vec::new(),
                generic_interface: None,
                associated_type: None,
            },
        );
        static_methods.push(associated);
    }
    if instance {
        methods.push(function);
    } else {
        static_methods.push(function);
    }
    true
}

pub(super) fn merge_projected_trait_operations(
    items: &mut Vec<ProjectedItem>,
    declined: &mut Vec<DeclinedItem>,
    operations: Vec<ProjectedTraitOperation>,
) {
    let mut occupied = items
        .iter()
        .map(|item| (item.namespace.clone(), item.name.clone()))
        .collect::<BTreeSet<_>>();
    let mut primary_counts = BTreeMap::new();
    for operation in &operations {
        *primary_counts
            .entry((
                operation.item.namespace.clone(),
                operation.item.name.clone(),
            ))
            .or_insert(0usize) += 1;
    }
    let mut remaining = Vec::new();
    for operation in operations {
        if attach_unique_trait_operation(items, &operation, &primary_counts) {
            continue;
        }
        remaining.push(operation);
    }
    let operations = remaining;
    let (primary, mut fallback): (Vec<_>, Vec<_>) = operations.into_iter().partition(|operation| {
        let key = (
            operation.item.namespace.clone(),
            operation.item.name.clone(),
        );
        primary_counts.get(&key).copied().unwrap_or_default() == 1 && !occupied.contains(&key)
    });
    for operation in primary {
        occupied.insert((
            operation.item.namespace.clone(),
            operation.item.name.clone(),
        ));
        items.push(operation.item);
    }
    for operation in &mut fallback {
        if let ProjectedKind::Function(function) = &mut operation.item.kind
            && let Some(receiver) = function.receiver.take()
            && let Some(owner) = function.native_owner.as_deref()
        {
            let base_rust_path = owner.split_once('<').map_or(owner, |(base, _)| base);
            function.parameters.insert(
                0,
                ProjectedParameter {
                    borrowed_str: false,
                    name: "receiver".to_owned(),
                    ty: ProjectedType::Foreign {
                        rust_path: owner.to_owned(),
                        name: base_rust_path
                            .rsplit("::")
                            .next()
                            .unwrap_or(base_rust_path)
                            .to_owned(),
                        base_rust_path: base_rust_path.to_owned(),
                        arguments: Vec::new(),
                    },
                    generic_parameter: None,
                    generic_interface: None,
                    generic_bounds: Vec::new(),
                    associated_type: None,
                    borrowed: !receiver.consumes(),
                    mutable_borrow: receiver == Receiver::MutableBorrow,
                },
            );
        }
        operation
            .item
            .namespace
            .clone_from(&operation.fallback_namespace);
    }
    let mut fallback_counts = BTreeMap::new();
    for operation in &fallback {
        *fallback_counts
            .entry((
                operation.item.namespace.clone(),
                operation.item.name.clone(),
            ))
            .or_insert(0usize) += 1;
    }
    for operation in fallback {
        let key = (
            operation.item.namespace.clone(),
            operation.item.name.clone(),
        );
        if fallback_counts.get(&key).copied().unwrap_or_default() == 1 && !occupied.contains(&key) {
            occupied.insert(key);
            items.push(operation.item);
        } else {
            declined.push(DeclinedItem {
                rust_path: operation.item.rust_path,
                reason:
                    "trait method conflicts within its concrete owner and trait-qualified namespace"
                        .to_owned(),
            });
        }
    }
}
#[expect(
    clippy::too_many_lines,
    reason = "one pass must classify every concrete provided-method candidate before resolving collisions"
)]
pub(super) fn project_external_provided_trait_methods(
    projected: &mut [ProjectedDependency],
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    canonical_public_paths: &[BTreeMap<String, String>],
    include_defaults: bool,
) {
    for (dependency_index, (dependency, document, _)) in rustdocs.iter().enumerate() {
        let mut paths = document.paths.clone();
        for summary in paths.values_mut() {
            if let Some(public_path) =
                canonical_public_paths[dependency_index].get(&summary.path.join("::"))
            {
                summary.path = public_path.split("::").map(str::to_owned).collect();
            }
        }
        let mut additions = Vec::new();
        for item in document.index.values() {
            let ItemEnum::Impl(implementation) = &item.inner else {
                continue;
            };
            let Some(trait_reference) = implementation.trait_.as_ref() else {
                continue;
            };
            let Some(trait_summary) = document.paths.get(&trait_reference.id) else {
                continue;
            };
            let explicit_method_names = implementation
                .items
                .iter()
                .filter_map(|id| document.index.get(id))
                .filter_map(|item| {
                    matches!(item.inner, ItemEnum::Function(_))
                        .then(|| item.name.clone())
                        .flatten()
                })
                .collect::<BTreeSet<_>>();
            if trait_summary.crate_id == 0
                || implementation.provided_trait_methods.is_empty()
                    && explicit_method_names.is_empty()
            {
                continue;
            }
            let trait_path = trait_summary.path.join("::");
            let Some((
                trait_dependency_index,
                trait_id,
                trait_document,
                trait_public_paths,
                trait_declaration,
            )) = rustdocs.iter().enumerate().find_map(
                |(trait_dependency_index, (_, trait_document, trait_public_paths))| {
                    trait_document.index.iter().find_map(|(id, item)| {
                        let ItemEnum::Trait(declaration) = &item.inner else {
                            return None;
                        };
                        let source_path = trait_document
                            .paths
                            .get(id)
                            .filter(|summary| summary.crate_id == 0)
                            .map(|summary| summary.path.join("::"))?;
                        let public_path = trait_public_paths
                            .get(id)
                            .cloned()
                            .unwrap_or_else(|| source_path.clone());
                        (public_path == trait_path || source_path == trait_path).then_some((
                            trait_dependency_index,
                            id,
                            trait_document,
                            trait_public_paths,
                            declaration,
                        ))
                    })
                },
            )
            else {
                continue;
            };
            let mut implementation_generics = BTreeMap::new();
            for parameter in &implementation.generics.params {
                if matches!(parameter.kind, GenericParamDefKind::Type { .. }) {
                    implementation_generics.insert(
                        parameter.name.clone(),
                        ProjectedType::Generic(parameter.name.clone()),
                    );
                }
            }
            let Ok(mut owner) = project_type(
                &implementation.for_,
                &document.index,
                &paths,
                &implementation_generics,
            ) else {
                continue;
            };
            let ProjectedType::Foreign { rust_path, .. } = &owner else {
                continue;
            };
            let owner_constructor = crate::rust_ir::rust_type_constructor(rust_path)
                .unwrap_or_else(|| rust_path.clone());
            let owner_constructor = super::extern_rust_path(dependency, &owner_constructor);
            let Some(owner_item) = projected[dependency_index].items.iter().find(|item| {
                crate::rust_ir::rust_type_constructor(&item.rust_path)
                    .as_deref()
                    .is_some_and(|constructor| constructor == owner_constructor)
            }) else {
                continue;
            };
            if let ProjectedType::Foreign {
                name,
                rust_path,
                base_rust_path,
                ..
            } = &mut owner
            {
                name.clone_from(&owner_item.name);
                let base = crate::rust_ir::rust_type_constructor(&owner_item.rust_path)
                    .unwrap_or_else(|| owner_item.rust_path.clone());
                let arguments = rust_path.find('<').map_or("", |start| &rust_path[start..]);
                base_rust_path.clone_from(&base);
                *rust_path = format!("{base}{arguments}");
            }
            let ProjectedType::Foreign { rust_path, .. } = &owner else {
                unreachable!("native impl owner remains a foreign type");
            };
            let owner_rust_path = rust_path.clone();
            let owner_namespace = owner_trait_namespace(&owner_item.namespace, &owner_item.name);
            let mut supplied = implementation_generics.clone();
            supplied.insert("Self".to_owned(), owner.clone());
            for (parameter, argument) in trait_declaration
                .generics
                .params
                .iter()
                .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
                .zip(super::callable::resolved_path_type_arguments(
                    trait_reference,
                ))
            {
                if let Ok(projected_type) =
                    project_type(argument, &document.index, &paths, &supplied)
                {
                    supplied.insert(parameter.name.clone(), projected_type);
                }
            }
            for associated_id in &implementation.items {
                let Some(Item {
                    name: Some(name),
                    inner:
                        ItemEnum::AssocType {
                            type_: Some(type_), ..
                        },
                    ..
                }) = document.index.get(associated_id)
                else {
                    continue;
                };
                if let Ok(projected_type) = project_type(type_, &document.index, &paths, &supplied)
                {
                    supplied.insert(format!("Self::{name}"), projected_type);
                }
            }
            supplied.insert(
                "__terrane_external_associated_bounds".to_owned(),
                ProjectedType::None,
            );
            let namespace = owner_namespace;
            let rendered_trait =
                render_resolved_path(trait_reference, &document.index, &paths, &supplied).ok();
            let trait_arguments = rendered_trait
                .as_deref()
                .and_then(|path| path.find('<').map(|start| &path[start..]))
                .unwrap_or_default();
            let public_trait_path = format!(
                "{}{trait_arguments}",
                trait_public_paths
                    .get(trait_id)
                    .unwrap_or(&trait_path)
                    .replace('-', "_")
            );
            let mut trait_paths = trait_document.paths.clone();
            for summary in trait_paths.values_mut() {
                if let Some(public_path) =
                    canonical_public_paths[trait_dependency_index].get(&summary.path.join("::"))
                {
                    summary.path = public_path.split("::").map(str::to_owned).collect();
                }
            }
            let mut trait_signature_public_paths = trait_public_paths.clone();
            for (id, public_path) in &mut trait_signature_public_paths {
                if let Some(source_path) = trait_document
                    .paths
                    .get(id)
                    .map(|summary| summary.path.join("::"))
                    && let Some(canonical_path) =
                        canonical_public_paths[trait_dependency_index].get(&source_path)
                {
                    public_path.clone_from(canonical_path);
                }
            }
            let mut method_candidates = implementation
                .provided_trait_methods
                .iter()
                .filter(|_| include_defaults)
                .cloned()
                .collect::<BTreeSet<_>>();
            method_candidates.extend(explicit_method_names.iter().cloned());
            for method_name in method_candidates {
                let is_explicit = explicit_method_names.contains(&method_name);
                let method_rust_path =
                    format!("<{owner_rust_path} as {public_trait_path}>::{method_name}");
                if is_explicit
                    && projected[dependency_index]
                        .items
                        .iter()
                        .any(|item| item.rust_path == method_rust_path)
                {
                    continue;
                }
                if is_explicit {
                    let failed_item_path = format!("{}::{method_name}", owner_item.rust_path);
                    if !projected[dependency_index]
                        .declined
                        .iter()
                        .any(|declined| declined.rust_path == failed_item_path)
                    {
                        continue;
                    }
                }

                let Some(Item {
                    inner: ItemEnum::Function(function),
                    docs,
                    ..
                }) = trait_declaration.items.iter().find_map(|method_id| {
                    trait_document
                        .index
                        .get(method_id)
                        .filter(|method| method.name.as_deref() == Some(method_name.as_str()))
                })
                else {
                    if !is_explicit {
                        projected[dependency_index].declined.push(DeclinedItem {
                            rust_path: method_rust_path,
                            reason: "provided trait method declaration is unavailable".to_owned(),
                        });
                    }
                    continue;
                };
                let mut enriched_function = function.clone();
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
                let mut method = match project_function_with_generics(
                    &enriched_function,
                    &trait_document.index,
                    &trait_paths,
                    &trait_signature_public_paths,
                    Some(&method_name),
                    &supplied,
                    false,
                ) {
                    Ok(method) => method,
                    Err(reason) => {
                        if !is_explicit {
                            projected[dependency_index].declined.push(DeclinedItem {
                                rust_path: method_rust_path,
                                reason,
                            });
                        }
                        continue;
                    }
                };
                let native_generics = function
                    .generics
                    .params
                    .iter()
                    .filter(|parameter| {
                        matches!(
                            parameter.kind,
                            GenericParamDefKind::Type {
                                is_synthetic: false,
                                ..
                            }
                        )
                    })
                    .map(|parameter| parameter.name.as_str())
                    .collect::<BTreeSet<_>>();
                method.rust_generic_arguments.retain(|argument| {
                    !matches!(argument, ProjectedType::Generic(name) if !native_generics.contains(name.as_str()))
                });
                method.native_owner = Some(owner_rust_path.clone());
                if is_explicit {
                    let failed_item_path = format!("{}::{method_name}", owner_item.rust_path);
                    projected[dependency_index]
                        .declined
                        .retain(|declined| declined.rust_path != failed_item_path);
                }
                additions.push(ProjectedTraitOperation {
                    fallback_namespace: trait_fallback_namespace(&namespace, &trait_path),
                    item: ProjectedItem {
                        namespace: namespace.clone(),
                        name: method_name.clone(),
                        rust_path: method_rust_path.clone(),
                        docs: Some(trait_operation_docs(&method_rust_path, docs.as_deref())),
                        kind: ProjectedKind::Function(method),
                    },
                });
            }
        }
        merge_projected_trait_operations(
            &mut projected[dependency_index].items,
            &mut projected[dependency_index].declined,
            additions,
        );
    }
}
