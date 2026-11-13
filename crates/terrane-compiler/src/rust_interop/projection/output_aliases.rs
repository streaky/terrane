//! Imports output alias definitions with document-local identities before admission.
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

use rustdoc_types::{
    AssocItemConstraintKind, ExternalCrate, GenericArg, GenericArgs, GenericBound, GenericParamDef,
    GenericParamDefKind, Generics, Id, Item, ItemEnum, ItemKind, ItemSummary, Path as RustdocPath,
    Term, Type, WherePredicate,
};

use super::{Containment, ProjectionError, ReexportRustdoc, RustDependency, RustdocCrate};

pub(super) fn document_name(document: &RustdocCrate) -> Option<&str> {
    document
        .paths
        .get(&document.root)?
        .path
        .first()
        .map(String::as_str)
}

fn alias_references(document: &RustdocCrate, wanted: &BTreeSet<Vec<String>>) -> BTreeSet<Id> {
    let mut references = document
        .paths
        .iter()
        .filter_map(|(id, summary)| {
            (summary.kind == ItemKind::TypeAlias && wanted.contains(&summary.path)).then_some(*id)
        })
        .collect::<BTreeSet<_>>();
    for item in document.index.values() {
        if let ItemEnum::Function(function) = &item.inner
            && let Some(output) = &function.sig.output
        {
            visit_type(output, &mut |path| {
                references.insert(path.id);
            });
        }
    }
    let mut pending = references.iter().copied().collect::<Vec<_>>();
    while let Some(id) = pending.pop() {
        if let Some(Item {
            inner: ItemEnum::TypeAlias(alias),
            ..
        }) = document.index.get(&id)
        {
            let mut add = |path: &RustdocPath| {
                if references.insert(path.id) {
                    pending.push(path.id);
                }
            };
            visit_type(&alias.type_, &mut add);
            visit_generics(&alias.generics, &mut add);
        }
    }
    references.retain(|id| {
        !document.index.contains_key(id)
            && document
                .paths
                .get(id)
                .is_some_and(|summary| summary.kind == ItemKind::TypeAlias)
    });
    references
}

pub(super) fn extend_public_aliases(
    reexports: &[ReexportRustdoc],
    canonical: &mut [BTreeMap<String, String>],
) -> bool {
    let mut changed = false;
    for provider in reexports.iter().flat_map(|owner| &owner.providers) {
        for (native, public) in &provider.canonical_public_paths {
            // Signature-only source names are not actual Rust facade exports.
            let public = provider.rust_path_aliases.get(public).unwrap_or(public);
            if super::functions::native_output_container(native.split("::")).is_none()
                && let std::collections::btree_map::Entry::Vacant(entry) =
                    canonical[provider.dependency_index].entry(native.clone())
            {
                entry.insert(public.clone());
                changed = true;
            }
        }
    }
    changed
}

/// Definitions are imported, never inferred from a name. All patches are planned
/// against immutable documents so a Rustdoc id can only resolve in its own index.
pub(super) fn hydrate(
    rustdocs: &mut [(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    reexports: &mut [ReexportRustdoc],
    owners: &mut BTreeMap<String, RustdocCrate>,
    workspace: &Path,
    metadata: &serde_json::Value,
    target: &str,
    containment: Containment,
) -> Result<bool, ProjectionError> {
    let mut attempted = BTreeSet::new();
    let mut wanted = BTreeSet::new();
    loop {
        let documents = rustdocs
            .iter()
            .map(|(_, doc, _)| doc)
            .chain(reexports.iter().map(|owner| &owner.document))
            .chain(owners.values())
            .collect::<Vec<_>>();
        let mut needed = BTreeSet::new();
        let mut expanded = false;
        for document in &documents {
            // Include imported alias targets even when the defining crate has no
            // function using that alias itself.
            for id in alias_references(document, &wanted) {
                let summary = &document.paths[&id];
                expanded |= wanted.insert(summary.path.clone());
                if let Some(owner) = document.external_crates.get(&summary.crate_id)
                    && !matches!(owner.name.as_str(), "core" | "std" | "alloc")
                    && !documents
                        .iter()
                        .any(|doc| document_name(doc) == Some(owner.name.as_str()))
                    && !attempted.contains(&owner.name)
                {
                    needed.insert(owner.name.clone());
                }
            }
        }
        if needed.is_empty() && !expanded {
            break;
        }
        for name in needed {
            attempted.insert(name.clone());
            let Ok((package, version)) =
                super::reexports::resolved_library_package(metadata, &name)
            else {
                // Admission reports the exact unresolved alias at its use, rather
                // than treating an unavailable definition as a nominal result.
                continue;
            };
            let document = super::reexports::cached_owner_rustdoc(
                workspace,
                &format!("{package}@{version}"),
                &name,
                &package,
                target,
                containment,
                metadata,
            )?;
            owners.insert(name, document);
        }
    }
    let documents = rustdocs
        .iter()
        .map(|(_, doc, _)| doc)
        .chain(reexports.iter().map(|owner| &owner.document))
        .chain(owners.values())
        .collect::<Vec<_>>();
    let patches = documents
        .iter()
        .map(|document| plan(document, &documents, &wanted, metadata))
        .collect::<Vec<_>>();
    let changed = patches.iter().any(|patch| !patch.items.is_empty());
    for (document, patch) in rustdocs
        .iter_mut()
        .map(|(_, doc, _)| doc)
        .chain(reexports.iter_mut().map(|owner| &mut owner.document))
        .chain(owners.values_mut())
        .zip(patches)
    {
        document.index.extend(patch.items);
        document.paths.extend(patch.paths);
        document.external_crates.extend(patch.crates);
    }
    Ok(changed)
}

#[derive(Default)]
struct Patch {
    items: HashMap<Id, Item>,
    paths: HashMap<Id, ItemSummary>,
    crates: HashMap<u32, ExternalCrate>,
}

struct Import<'a> {
    target: &'a RustdocCrate,
    patch: Patch,
    next_id: u32,
    next_crate: u32,
}

fn plan(
    target: &RustdocCrate,
    documents: &[&RustdocCrate],
    wanted: &BTreeSet<Vec<String>>,
    metadata: &serde_json::Value,
) -> Patch {
    let mut import = Import {
        target,
        patch: Patch::default(),
        next_id: target
            .index
            .keys()
            .chain(target.paths.keys())
            .map(|id| id.0)
            .max()
            .unwrap_or(0)
            + 1,
        next_crate: target.external_crates.keys().copied().max().unwrap_or(0) + 1,
    };
    let mut pending = alias_references(target, wanted)
        .into_iter()
        .collect::<Vec<_>>();
    let mut visited = BTreeSet::new();
    while let Some(id) = pending.pop() {
        if !visited.insert(id) {
            continue;
        }
        let Some(summary) = target
            .paths
            .get(&id)
            .or_else(|| import.patch.paths.get(&id))
        else {
            continue;
        };
        if summary.kind != ItemKind::TypeAlias {
            continue;
        }
        if let Some(owner) = summary.path.first()
            && !matches!(owner.as_str(), "std" | "core" | "alloc")
            && super::reexports::resolved_library_package(metadata, owner).is_err()
        {
            continue;
        }
        // Prefer the declared defining document. Supplemental copies of the
        // same locked crate can use different Rustdoc IDs; raw AST equality
        // across those documents is not an ambiguity test.
        let Some((source, definition)) = documents.iter().find_map(|document| {
            document
                .paths
                .iter()
                .find_map(|(source_id, source_summary)| {
                    (source_summary.path == summary.path
                        && source_summary.kind == ItemKind::TypeAlias)
                        .then(|| document.index.get(source_id))
                        .flatten()
                        .filter(|item| matches!(item.inner, ItemEnum::TypeAlias(_)))
                        .map(|item| (*document, item))
                })
        }) else {
            continue;
        };
        let mut item = definition.clone();
        item.id = id;
        item.crate_id = summary.crate_id;
        item.links.clear();
        let ItemEnum::TypeAlias(alias) = &mut item.inner else {
            unreachable!()
        };
        let mut error = false;
        let mut remap = |path: &mut RustdocPath| {
            if let Some(mapped) = import.path(source, path.id) {
                path.id = mapped;
                pending.push(mapped);
            } else {
                error = true;
            }
        };
        map_type(&mut alias.type_, &mut remap);
        map_generics(&mut alias.generics, &mut remap);
        if !error {
            import.patch.items.insert(id, item);
        }
    }
    import.patch
}

impl Import<'_> {
    fn path(&mut self, source: &RustdocCrate, id: Id) -> Option<Id> {
        let summary = source.paths.get(&id)?;
        if let Some((id, _)) = self
            .target
            .paths
            .iter()
            .chain(&self.patch.paths)
            .find(|(_, candidate)| candidate.path == summary.path && candidate.kind == summary.kind)
        {
            return Some(*id);
        }
        let mut summary = summary.clone();
        let name = if summary.crate_id == 0 {
            document_name(source)?
        } else {
            &source.external_crates.get(&summary.crate_id)?.name
        };
        summary.crate_id = if document_name(self.target) == Some(name) {
            0
        } else if let Some((id, _)) = self
            .target
            .external_crates
            .iter()
            .chain(&self.patch.crates)
            .find(|(_, ext)| ext.name == name)
        {
            *id
        } else {
            let external = source.external_crates.get(&summary.crate_id)?.clone();
            let id = self.next_crate;
            self.next_crate += 1;
            self.patch.crates.insert(id, external);
            id
        };
        let id = Id(self.next_id);
        self.next_id += 1;
        self.patch.paths.insert(id, summary);
        Some(id)
    }
}

fn visit_type(ty: &Type, visit: &mut impl FnMut(&RustdocPath)) {
    match ty {
        Type::ResolvedPath(path) => visit_path(path, visit),
        Type::DynTrait(value) => {
            for bound in &value.traits {
                visit_path(&bound.trait_, visit);
                visit_parameters(&bound.generic_params, visit);
            }
        }
        Type::FunctionPointer(function) => {
            visit_parameters(&function.generic_params, visit);
            for (_, input) in &function.sig.inputs {
                visit_type(input, visit);
            }
            if let Some(output) = &function.sig.output {
                visit_type(output, visit);
            }
        }
        Type::Tuple(items) => {
            for item in items {
                visit_type(item, visit);
            }
        }
        Type::Slice(inner)
        | Type::Array { type_: inner, .. }
        | Type::Pat { type_: inner, .. }
        | Type::RawPointer { type_: inner, .. }
        | Type::BorrowedRef { type_: inner, .. } => visit_type(inner, visit),
        Type::ImplTrait(bounds) => visit_bounds(bounds, visit),
        Type::QualifiedPath {
            args,
            self_type,
            trait_,
            ..
        } => {
            if let Some(args) = args {
                visit_args(args, visit);
            }
            visit_type(self_type, visit);
            if let Some(path) = trait_ {
                visit_path(path, visit);
            }
        }
        Type::Generic(_) | Type::Primitive(_) | Type::Infer => {}
    }
}
fn visit_path(path: &RustdocPath, visit: &mut impl FnMut(&RustdocPath)) {
    visit(path);
    if let Some(args) = &path.args {
        visit_args(args, visit);
    }
}
fn visit_args(args: &GenericArgs, visit: &mut impl FnMut(&RustdocPath)) {
    match args {
        GenericArgs::AngleBracketed { args, constraints } => {
            for arg in args {
                if let GenericArg::Type(ty) = arg {
                    visit_type(ty, visit);
                }
            }
            for constraint in constraints {
                if let Some(args) = &constraint.args {
                    visit_args(args, visit);
                }
                match &constraint.binding {
                    AssocItemConstraintKind::Equality(Term::Type(ty)) => visit_type(ty, visit),
                    AssocItemConstraintKind::Constraint(bounds) => visit_bounds(bounds, visit),
                    AssocItemConstraintKind::Equality(Term::Constant(_)) => {}
                }
            }
        }
        GenericArgs::Parenthesized { inputs, output } => {
            for input in inputs {
                visit_type(input, visit);
            }
            if let Some(output) = output {
                visit_type(output, visit);
            }
        }
        GenericArgs::ReturnTypeNotation => {}
    }
}
fn visit_bounds(bounds: &[GenericBound], visit: &mut impl FnMut(&RustdocPath)) {
    for bound in bounds {
        if let GenericBound::TraitBound {
            trait_,
            generic_params,
            ..
        } = bound
        {
            visit_path(trait_, visit);
            visit_parameters(generic_params, visit);
        }
    }
}
fn visit_parameters(params: &[GenericParamDef], visit: &mut impl FnMut(&RustdocPath)) {
    for param in params {
        match &param.kind {
            GenericParamDefKind::Type {
                bounds, default, ..
            } => {
                visit_bounds(bounds, visit);
                if let Some(default) = default {
                    visit_type(default, visit);
                }
            }
            GenericParamDefKind::Const { type_, .. } => visit_type(type_, visit),
            GenericParamDefKind::Lifetime { .. } => {}
        }
    }
}
fn visit_generics(generics: &Generics, visit: &mut impl FnMut(&RustdocPath)) {
    visit_parameters(&generics.params, visit);
    for predicate in &generics.where_predicates {
        match predicate {
            WherePredicate::BoundPredicate {
                type_,
                bounds,
                generic_params,
            } => {
                visit_type(type_, visit);
                visit_bounds(bounds, visit);
                visit_parameters(generic_params, visit);
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                visit_type(lhs, visit);
                if let Term::Type(ty) = rhs {
                    visit_type(ty, visit);
                }
            }
            WherePredicate::LifetimePredicate { .. } => {}
        }
    }
}

fn map_type(ty: &mut Type, visit: &mut impl FnMut(&mut RustdocPath)) {
    match ty {
        Type::ResolvedPath(path) => map_path(path, visit),
        Type::DynTrait(value) => {
            for bound in &mut value.traits {
                map_path(&mut bound.trait_, visit);
                map_parameters(&mut bound.generic_params, visit);
            }
        }
        Type::FunctionPointer(function) => {
            map_parameters(&mut function.generic_params, visit);
            for (_, input) in &mut function.sig.inputs {
                map_type(input, visit);
            }
            if let Some(output) = &mut function.sig.output {
                map_type(output, visit);
            }
        }
        Type::Tuple(items) => {
            for item in items {
                map_type(item, visit);
            }
        }
        Type::Slice(inner)
        | Type::Array { type_: inner, .. }
        | Type::Pat { type_: inner, .. }
        | Type::RawPointer { type_: inner, .. }
        | Type::BorrowedRef { type_: inner, .. } => map_type(inner, visit),
        Type::ImplTrait(bounds) => map_bounds(bounds, visit),
        Type::QualifiedPath {
            args,
            self_type,
            trait_,
            ..
        } => {
            if let Some(args) = args {
                map_args(args, visit);
            }
            map_type(self_type, visit);
            if let Some(path) = trait_ {
                map_path(path, visit);
            }
        }
        Type::Generic(_) | Type::Primitive(_) | Type::Infer => {}
    }
}
fn map_path(path: &mut RustdocPath, visit: &mut impl FnMut(&mut RustdocPath)) {
    visit(path);
    if let Some(args) = &mut path.args {
        map_args(args, visit);
    }
}
fn map_args(args: &mut GenericArgs, visit: &mut impl FnMut(&mut RustdocPath)) {
    match args {
        GenericArgs::AngleBracketed { args, constraints } => {
            for arg in args {
                if let GenericArg::Type(ty) = arg {
                    map_type(ty, visit);
                }
            }
            for constraint in constraints {
                if let Some(args) = &mut constraint.args {
                    map_args(args, visit);
                }
                match &mut constraint.binding {
                    AssocItemConstraintKind::Equality(Term::Type(ty)) => map_type(ty, visit),
                    AssocItemConstraintKind::Constraint(bounds) => map_bounds(bounds, visit),
                    AssocItemConstraintKind::Equality(Term::Constant(_)) => {}
                }
            }
        }
        GenericArgs::Parenthesized { inputs, output } => {
            for input in inputs {
                map_type(input, visit);
            }
            if let Some(output) = output {
                map_type(output, visit);
            }
        }
        GenericArgs::ReturnTypeNotation => {}
    }
}
fn map_bounds(bounds: &mut [GenericBound], visit: &mut impl FnMut(&mut RustdocPath)) {
    for bound in bounds {
        if let GenericBound::TraitBound {
            trait_,
            generic_params,
            ..
        } = bound
        {
            map_path(trait_, visit);
            map_parameters(generic_params, visit);
        }
    }
}
fn map_parameters(params: &mut [GenericParamDef], visit: &mut impl FnMut(&mut RustdocPath)) {
    for param in params {
        match &mut param.kind {
            GenericParamDefKind::Type {
                bounds, default, ..
            } => {
                map_bounds(bounds, visit);
                if let Some(default) = default {
                    map_type(default, visit);
                }
            }
            GenericParamDefKind::Const { type_, .. } => map_type(type_, visit),
            GenericParamDefKind::Lifetime { .. } => {}
        }
    }
}
fn map_generics(generics: &mut Generics, visit: &mut impl FnMut(&mut RustdocPath)) {
    map_parameters(&mut generics.params, visit);
    for predicate in &mut generics.where_predicates {
        match predicate {
            WherePredicate::BoundPredicate {
                type_,
                bounds,
                generic_params,
            } => {
                map_type(type_, visit);
                map_bounds(bounds, visit);
                map_parameters(generic_params, visit);
            }
            WherePredicate::EqPredicate { lhs, rhs } => {
                map_type(lhs, visit);
                if let Term::Type(ty) = rhs {
                    map_type(ty, visit);
                }
            }
            WherePredicate::LifetimePredicate { .. } => {}
        }
    }
}

#[cfg(test)]
mod tests;
