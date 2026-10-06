//! Projects Rust types and borrowed/invocation-scoped type shapes.
use super::{
    AssocItemConstraintKind, BTreeMap, GenericArg, GenericArgs, GenericBound, GenericParamDefKind,
    HashMap, Id, Item, ItemEnum, ItemSummary, ProjectedAssociatedBinding, ProjectedType,
    RustdocPath, Term, Type, alias_type_substitutions, instantiated_nominal_name, nominal_generics,
    project_interface, render_generic_bound, render_resolved_path, render_rust_type,
    resolved_path_name, rust_lifetimes, rust_path_owner, trait_bound_name,
    type_contains_lifetime_argument,
};

pub(super) fn projectable_interface_bound<'a>(
    bounds: &'a [GenericBound],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<&'a RustdocPath, String> {
    let candidates = bounds
        .iter()
        .filter_map(|bound| {
            let (trait_, generic_params) = trait_bound_name(bound)?;
            if !generic_params.is_empty() {
                return Some(Err(
                    "higher-ranked interface bound is not projectable".to_owned()
                ));
            }
            let auto = paths
                .get(&trait_.id)
                .map(|summary| summary.path.join("::"))
                .is_some_and(|path| {
                    matches!(path.as_str(), "core::marker::Send" | "core::marker::Sync")
                });
            if auto { None } else { Some(Ok(trait_)) }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let [trait_] = candidates.as_slice() else {
        return Err("generic input requires one projectable interface bound".to_owned());
    };
    let Some(Item {
        inner: ItemEnum::Trait(declaration),
        ..
    }) = index.get(&trait_.id)
    else {
        return Err("generic input has an unresolved interface bound".to_owned());
    };
    project_interface(declaration, index, paths, &trait_.path)
        .map_err(|_| "generic input bound is not a projectable interface".to_owned())?;

    Ok(trait_)
}
pub(super) fn invocation_scoped_sequence_impl_trait_input(
    bounds: &[GenericBound],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedType>, String> {
    let Some(trait_) = bounds.iter().find_map(|bound| {
        let (trait_, _) = trait_bound_name(bound)?;
        let path = paths
            .get(&trait_.id)
            .map_or_else(|| trait_.path.clone(), |summary| summary.path.join("::"));
        matches!(
            path.as_str(),
            "core::iter::traits::collect::IntoIterator"
                | "std::iter::IntoIterator"
                | "core::iter::IntoIterator"
        )
        .then_some(trait_)
    }) else {
        return Ok(None);
    };
    let Some(GenericArgs::AngleBracketed { constraints, .. }) = trait_.args.as_deref() else {
        return Ok(None);
    };
    let Some(item_type) = constraints.iter().find_map(|constraint| {
        (constraint.name == "Item")
            .then_some(&constraint.binding)
            .and_then(|binding| match binding {
                AssocItemConstraintKind::Equality(Term::Type(ty)) => Some(ty),
                _ => None,
            })
    }) else {
        return Ok(None);
    };
    if !type_contains_lifetime_argument(item_type) {
        return Ok(None);
    }
    let mut rendering_paths = paths.clone();
    for (id, public_path) in public_paths {
        if let Some(summary) = rendering_paths.get_mut(id) {
            summary.path = public_path.split("::").map(str::to_owned).collect();
        }
    }
    if let Type::ResolvedPath(item_path) = item_type
        && let Some(name) = item_path.path.rsplit("::").next()
        && let Some(public_path) = public_paths
            .values()
            .filter(|path| path.rsplit("::").next() == Some(name))
            .min_by_key(|path| path.matches("::").count())
        && let Some(summary) = rendering_paths.get_mut(&item_path.id)
    {
        summary.path = public_path.split("::").map(str::to_owned).collect();
    }
    let item = project_invocation_scoped_type(item_type, index, &rendering_paths, generics)?;
    Ok(Some(ProjectedType::Sequence {
        rust_path: format!("std::vec::Vec<{}>", item.rust_type()),
        item: Box::new(item),
    }))
}

pub(super) fn structural_impl_trait_input(
    bounds: &[GenericBound],
    paths: &HashMap<Id, ItemSummary>,
) -> Option<ProjectedType> {
    let mut candidates = bounds.iter().filter_map(|bound| {
        let (trait_, generic_params) = trait_bound_name(bound)?;
        if !generic_params.is_empty() {
            return None;
        }
        let trait_path = paths
            .get(&trait_.id)
            .map_or_else(|| trait_.path.clone(), |summary| summary.path.join("::"));
        (!matches!(
            trait_path.as_str(),
            "core::marker::Send" | "core::marker::Sync"
        ))
        .then_some((trait_path, trait_))
    });
    let (trait_path, trait_) = candidates.next()?;
    if candidates.next().is_some()
        || !matches!(
            trait_path.as_str(),
            "core::convert::AsRef" | "std::convert::AsRef"
        )
    {
        return None;
    }
    let GenericArgs::AngleBracketed { args, constraints } = trait_.args.as_deref()? else {
        return None;
    };
    if !constraints.is_empty() {
        return None;
    }
    let [GenericArg::Type(target)] = args.as_slice() else {
        return None;
    };
    match target {
        Type::Primitive(name) if name == "str" => Some(ProjectedType::String),
        Type::ResolvedPath(path)
            if paths
                .get(&path.id)
                .map_or_else(|| path.path.clone(), |summary| summary.path.join("::"))
                == "std::path::Path" =>
        {
            Some(ProjectedType::String)
        }
        _ => None,
    }
}

pub(super) fn project_borrowed_graph_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    let projected = project_type(ty, index, paths, generics)?;
    if matches!(projected, ProjectedType::String) {
        return Ok(ProjectedType::BorrowedString);
    }
    let rust_type = if let Type::BorrowedRef {
        type_,
        lifetime,
        is_mutable,
    } = ty
        && matches!(type_.as_ref(), Type::QualifiedPath { .. })
    {
        format!(
            "&{}{}{}",
            lifetime
                .as_ref()
                .map_or(String::new(), |lifetime| format!("{lifetime} ")),
            if *is_mutable { "mut " } else { "" },
            projected.rust_type()
        )
    } else {
        render_rust_type(ty, index, paths, generics)?
    };
    Ok(ProjectedType::InvocationScoped {
        name: if matches!(projected, ProjectedType::Optional(_)) {
            "borrowed-option".to_owned()
        } else {
            projected.terrane_name()
        },
        lifetimes: rust_lifetimes(&rust_type),
        rust_type,
        expression_scoped: true,
        owned: Box::new(projected),
    })
}

pub(super) fn project_invocation_scoped_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    let projected = project_type(ty, index, paths, generics)?;
    if let ProjectedType::Sequence { rust_path, .. } = &projected
        && let Some(item) = type_arguments(ty).into_iter().next()
    {
        return Ok(ProjectedType::Sequence {
            rust_path: rust_path.clone(),
            item: Box::new(project_invocation_scoped_type(
                item, index, paths, generics,
            )?),
        });
    }
    let rust_type = render_rust_type(ty, index, paths, generics)?;
    let lifetimes = rust_lifetimes(&rust_type);
    if lifetimes.is_empty() {
        return Ok(projected);
    }
    Ok(ProjectedType::InvocationScoped {
        name: projected.terrane_name(),
        rust_type,
        lifetimes,
        expression_scoped: false,
        owned: Box::new(projected),
    })
}

pub(super) fn project_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    match ty {
        Type::BorrowedRef { type_, .. } => project_type(type_, index, paths, generics),
        Type::Generic(generic) => generics.get(generic).cloned().ok_or_else(|| {
            if generic == "Self" {
                "receiver type used outside receiver position".to_owned()
            } else {
                format!("unbounded generic `{generic}`")
            }
        }),
        Type::Primitive(primitive) => match primitive.as_str() {
            "bool" => Ok(ProjectedType::Bool),
            "str" => Ok(ProjectedType::String),
            "f32" => Ok(ProjectedType::Float32),
            "f64" => Ok(ProjectedType::Float),
            "char" => Ok(ProjectedType::Char),
            "i64" => Ok(ProjectedType::Int),
            "i8" | "i16" | "i32" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
            | "usize" => Ok(ProjectedType::RustInt(primitive.clone())),
            "unit" => Ok(ProjectedType::None),
            other => Err(format!("unsupported primitive `{other}`")),
        },
        Type::Tuple(types) if types.is_empty() => Ok(ProjectedType::None),
        Type::Tuple(types) => {
            let items = types
                .iter()
                .map(|item| project_type(item, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            if items.windows(2).any(|pair| pair[0] != pair[1]) {
                return Err("heterogeneous tuple has no Terrane tuple representation".to_owned());
            }
            Ok(ProjectedType::Tuple(items))
        }
        Type::FunctionPointer(function) if !function.generic_params.is_empty() => {
            Err("higher-ranked function type is not projectable".to_owned())
        }
        Type::ResolvedPath(path) => project_resolved_type(ty, path, index, paths, generics),
        Type::QualifiedPath {
            name,
            args,
            self_type,
            ..
        } if args.is_none()
            && matches!(self_type.as_ref(), Type::Generic(self_) if self_ == "Self") =>
        {
            generics
                .get(&format!("Self::{name}"))
                .cloned()
                .ok_or_else(|| format!("unresolved associated type `Self::{name}`"))
        }
        Type::QualifiedPath {
            name,
            args,
            self_type,
            ..
        } if args.is_none()
            && matches!(self_type.as_ref(), Type::Generic(owner) if generics.contains_key(owner)) =>
        {
            let Type::Generic(owner) = self_type.as_ref() else {
                unreachable!("qualified generic owner was matched above");
            };
            Ok(ProjectedType::Associated(format!("{owner}::{name}")))
        }
        Type::ImplTrait(bounds) => Ok(ProjectedType::Opaque {
            anonymous_chain: bounds.iter().any(|bound| {
                trait_bound_name(bound).is_some_and(|(path, _)| {
                    resolved_path_name(path, paths)
                        .rsplit("::")
                        .next()
                        .is_some_and(|name| matches!(name, "Fn" | "FnMut" | "FnOnce"))
                })
            }),
            bounds: bounds
                .iter()
                .filter_map(|bound| render_generic_bound(bound, &[], index, paths, generics).ok())
                .collect(),
        }),
        Type::DynTrait(_) => {
            Err("trait objects require an owning `Box<dyn Trait>` parameter".to_owned())
        }
        _ => Err("type has no stable Rust path".to_owned()),
    }
}
pub(super) fn project_associated_binding(
    trait_: &RustdocPath,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedAssociatedBinding>, String> {
    let Some(GenericArgs::AngleBracketed { constraints, .. }) = trait_.args.as_deref() else {
        return Ok(None);
    };
    let mut bindings = constraints.iter().map(|constraint| {
        if constraint.args.is_some() {
            return Err("generic associated type bindings are not supported".to_owned());
        }
        match &constraint.binding {
            AssocItemConstraintKind::Equality(Term::Type(ty)) => {
                project_type(ty, index, paths, generics).map(|ty| ProjectedAssociatedBinding {
                    name: constraint.name.clone(),
                    ty: Box::new(ty),
                })
            }
            _ => Err("associated types require an exact type binding".to_owned()),
        }
    });
    let binding = bindings.next().transpose()?;
    if bindings.next().is_some() {
        return Err("more than one associated binding is not supported".to_owned());
    }
    Ok(binding)
}

pub(super) fn project_dyn_interface(
    dynamic: &rustdoc_types::DynTrait,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    if dynamic
        .lifetime
        .as_deref()
        .is_some_and(|lifetime| lifetime != "'static" && lifetime != "static")
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
    let trait_ = projectable_interface_bound(&bounds, index, paths)?;
    let mut base_trait = trait_.clone();
    let principal_rust_path = render_resolved_path(trait_, index, paths, generics)?;
    base_trait.args = None;
    let trait_path = render_resolved_path(&base_trait, index, paths, generics)?;
    let associated_type = project_associated_binding(trait_, index, paths, generics)?;
    let Some(Item {
        inner: ItemEnum::Trait(declaration),
        ..
    }) = index.get(&trait_.id)
    else {
        return Err("trait object principal does not resolve to a trait".to_owned());
    };
    let projected_interface = project_interface(declaration, index, paths, &trait_path)?;
    match (
        projected_interface.associated_type.as_ref(),
        associated_type.as_ref(),
    ) {
        (Some(expected), Some(binding)) if expected.name == binding.name => {}
        (Some(expected), Some(binding)) => {
            return Err(format!(
                "boxed projected interface binds `{}` but requires `{}`",
                binding.name, expected.name
            ));
        }
        (Some(expected), None) => {
            return Err(format!(
                "boxed projected interface requires an exact `{}` binding",
                expected.name
            ));
        }
        (None, Some(binding)) => {
            return Err(format!(
                "boxed projected interface has an unexpected `{}` binding",
                binding.name
            ));
        }
        (None, None) => {}
    }
    let mut auto_traits = dynamic
        .traits
        .iter()
        .filter_map(|poly| {
            let canonical = paths.get(&poly.trait_.id)?.path.join("::");
            matches!(
                canonical.as_str(),
                "core::marker::Send"
                    | "std::marker::Send"
                    | "core::marker::Sync"
                    | "std::marker::Sync"
            )
            .then(|| canonical)
        })
        .collect::<Vec<_>>();
    auto_traits.sort();
    auto_traits.dedup();
    let dynamic_bounds = std::iter::once(principal_rust_path.as_str())
        .chain(auto_traits.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" + ");
    Ok(ProjectedType::BoxedInterface {
        rust_path: format!("dyn {dynamic_bounds}"),
        trait_path,
        name: trait_
            .path
            .rsplit("::")
            .next()
            .unwrap_or(&trait_.path)
            .to_owned(),
        auto_traits,
        associated_type,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "resolved Rust type classification keeps canonical paths and recursive shape checks together"
)]
fn project_resolved_type(
    ty: &Type,
    path: &RustdocPath,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    if let Some(Item {
        inner: ItemEnum::TypeAlias(alias),
        ..
    }) = index.get(&path.id)
    {
        let substitutions =
            alias_type_substitutions(ty, &alias.generics.params, index, paths, generics)?;
        return project_type(&alias.type_, index, paths, &substitutions);
    }
    let resolved = resolved_path_name(path, paths);
    if matches!(
        resolved.as_str(),
        "alloc::string::String" | "std::string::String"
    ) {
        return Ok(ProjectedType::String);
    }
    let arguments = type_arguments(ty);
    if matches!(
        resolved.as_str(),
        "core::option::Option" | "std::option::Option"
    ) {
        let inner = arguments
            .first()
            .ok_or_else(|| "Option has no value type".to_owned())?;
        return Ok(ProjectedType::Optional(Box::new(project_type(
            inner, index, paths, generics,
        )?)));
    }
    if matches!(resolved.as_str(), "alloc::boxed::Box" | "std::boxed::Box") {
        let inner = arguments
            .first()
            .ok_or_else(|| "Box has no value type".to_owned())?;
        let Type::DynTrait(dynamic) = inner else {
            return Err("only owning projected interface boxes are supported".to_owned());
        };
        let ProjectedType::BoxedInterface {
            rust_path: dynamic,
            trait_path,
            name,
            auto_traits,
            associated_type,
        } = project_dyn_interface(dynamic, index, paths, generics)?
        else {
            unreachable!("dynamic interface projection returns its boxed-interface shape");
        };
        return Ok(ProjectedType::BoxedInterface {
            rust_path: format!("Box<{dynamic}>"),
            trait_path,
            name,
            auto_traits,
            associated_type,
        });
    }
    if matches!(resolved.as_str(), "alloc::vec::Vec" | "std::vec::Vec") {
        let item = arguments
            .first()
            .ok_or_else(|| "Vec has no item type".to_owned())?;
        let item = project_type(item, index, paths, generics)?;
        if item == ProjectedType::RustInt("u8".to_owned()) {
            return Ok(ProjectedType::Bytes);
        }
        let rust_path = render_resolved_path(path, index, paths, generics)?;
        return Ok(ProjectedType::Sequence {
            rust_path,
            item: Box::new(item),
        });
    }
    let rust_path = render_resolved_path(path, index, paths, generics)?;
    if matches!(
        resolved.as_str(),
        "std::collections::HashMap"
            | "std::collections::hash::map::HashMap"
            | "alloc::collections::BTreeMap"
            | "alloc::collections::btree::map::BTreeMap"
    ) {
        let [key, value] = arguments.as_slice() else {
            return Err("map type does not have exactly two type arguments".to_owned());
        };
        let key = project_type(key, index, paths, generics)?;
        if !key.is_terrane_scalar() {
            return Err("map key is not a Terrane scalar".to_owned());
        }
        return Ok(ProjectedType::Mapping {
            rust_path,
            key: Box::new(key),
            value: Box::new(project_type(value, index, paths, generics)?),
            ordered: resolved.contains("BTree"),
        });
    }
    if matches!(
        resolved.as_str(),
        "std::collections::HashSet"
            | "std::collections::hash::set::HashSet"
            | "alloc::collections::BTreeSet"
            | "alloc::collections::btree::set::BTreeSet"
    ) {
        let item = arguments
            .first()
            .ok_or_else(|| "set has no item type".to_owned())?;
        let item = project_type(item, index, paths, generics)?;
        if !item.is_terrane_scalar() {
            return Err("set item is not a Terrane scalar".to_owned());
        }
        return Ok(ProjectedType::Set {
            rust_path,
            item: Box::new(item),
            ordered: resolved.contains("BTree"),
        });
    }
    let short = resolved
        .rsplit("::")
        .next()
        .unwrap_or(&resolved)
        .split_once('<')
        .map_or_else(
            || resolved.rsplit("::").next().unwrap_or(&resolved),
            |(name, _)| name,
        )
        .to_owned();
    let mut projected_arguments = arguments
        .into_iter()
        .map(|argument| project_type(argument, index, paths, generics))
        .collect::<Result<Vec<_>, _>>()?;
    let mut substitutions = generics.clone();
    if let Some(declaration_generics) = nominal_generics(index.get(&path.id)) {
        let parameters = declaration_generics
            .params
            .iter()
            .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
            .collect::<Vec<_>>();
        for (parameter, argument) in parameters.iter().zip(&projected_arguments) {
            substitutions.insert(parameter.name.clone(), argument.clone());
        }
        for parameter in parameters.iter().skip(projected_arguments.len()) {
            let GenericParamDefKind::Type {
                default: Some(default),
                ..
            } = &parameter.kind
            else {
                break;
            };
            let argument = project_type(default, index, paths, &substitutions)?;
            substitutions.insert(parameter.name.clone(), argument.clone());
            projected_arguments.push(argument);
        }
    }
    let base_rust_path = rust_path
        .split_once('<')
        .map_or_else(|| rust_path.clone(), |(base, _)| base.to_owned());
    let rust_path = if projected_arguments.is_empty() {
        base_rust_path.clone()
    } else {
        format!(
            "{base_rust_path}<{}>",
            projected_arguments
                .iter()
                .map(ProjectedType::rust_type)
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let name = instantiated_nominal_name(&short, &rust_path, &projected_arguments);
    Ok(ProjectedType::Foreign {
        rust_path,
        name,
        base_rust_path,
        arguments: projected_arguments,
    })
}

pub(super) fn immediate_generic_input(ty: &Type, generic: &str) -> bool {
    match ty {
        Type::Generic(name) => name == generic,
        Type::BorrowedRef { type_, .. } => {
            matches!(type_.as_ref(), Type::Generic(name) if name == generic)
        }
        _ => false,
    }
}

fn resolved_error_name(ty: &Type, paths: &HashMap<Id, ItemSummary>) -> Option<String> {
    let Type::ResolvedPath(path) = ty else {
        return None;
    };
    let resolved = resolved_path_name(path, paths);
    let written = path.path.replace("crate::", "");
    Some(
        [resolved, written]
            .into_iter()
            .max_by_key(|candidate| candidate.matches("::").count())
            .expect("two error type spellings are available"),
    )
}

pub(super) fn projected_error_name(
    error: &Type,
    result: &Type,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<String> {
    let error = resolved_error_name(error, paths)?;
    if rust_path_owner(&error).is_some() {
        return Some(error);
    }
    let Type::ResolvedPath(result) = result else {
        return Some(error);
    };
    let resolved = resolved_path_name(result, paths);
    let written = result.path.replace("crate::", "");
    let alias = [resolved, written]
        .into_iter()
        .max_by_key(|candidate| candidate.matches("::").count())
        .expect("two result type spellings are available");
    let Some(owner) = alias.strip_suffix("::Result") else {
        return Some(error);
    };
    if matches!(owner, "std::result" | "core::result") {
        Some(error)
    } else {
        Some(format!("{owner}::Error"))
    }
}

pub(super) fn type_contains_borrowed_ref(ty: &Type) -> bool {
    match ty {
        Type::BorrowedRef { .. } => true,
        Type::ResolvedPath(_) => type_arguments(ty)
            .into_iter()
            .any(type_contains_borrowed_ref),
        Type::Tuple(items) => items.iter().any(type_contains_borrowed_ref),
        Type::Slice(item)
        | Type::Array { type_: item, .. }
        | Type::Pat { type_: item, .. }
        | Type::RawPointer { type_: item, .. } => type_contains_borrowed_ref(item),
        // Borrowed values inside callable signatures are governed by the
        // callable boundary rather than escaping through the outer parameter.
        Type::FunctionPointer(_)
        | Type::DynTrait(_)
        | Type::Generic(_)
        | Type::Primitive(_)
        | Type::ImplTrait(_)
        | Type::Infer => false,
        Type::QualifiedPath { self_type, .. } => type_contains_borrowed_ref(self_type),
    }
}

pub(super) fn type_arguments(ty: &Type) -> Vec<&Type> {
    let Type::ResolvedPath(path) = ty else {
        return Vec::new();
    };
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
