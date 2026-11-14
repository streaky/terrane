use super::{
    GenericArg, GenericArgs, GenericBound, GenericParamDefKind, HashMap, Id, Impl, Item, ItemEnum,
    ItemSummary, ProjectedBoundaryCapabilities, ProjectedEnumPayloadConversion, Receiver,
    RustdocPath, Type,
};
pub(super) fn descriptive_rust_identity(identity: &str) -> String {
    let mut words = identity
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty());
    let Some(first) = words.next() else {
        return String::new();
    };
    let mut result = if first.starts_with(|character: char| character.is_ascii_digit()) {
        format!("n{first}")
    } else {
        first.to_owned()
    };
    for word in words {
        result.push('-');
        if word.starts_with(|character: char| character.is_ascii_digit()) {
            result.push('n');
        }
        result.push_str(word);
    }
    result
}

pub(super) fn receiver_kind(
    ty: &Type,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<Receiver, String> {
    match ty {
        Type::Generic(name) if name == "Self" => Ok(Receiver::Move),
        Type::BorrowedRef {
            is_mutable, type_, ..
        } if matches!(type_.as_ref(), Type::Generic(name) if name == "Self") => {
            Ok(if *is_mutable {
                Receiver::MutableBorrow
            } else {
                Receiver::Borrow
            })
        }
        Type::ResolvedPath(path)
            if paths.get(&path.id).is_some_and(|item| {
                matches!(
                    item.path.join("::").as_str(),
                    "alloc::boxed::Box" | "std::boxed::Box"
                )
            }) && matches!(
                path.args.as_deref(),
                Some(GenericArgs::AngleBracketed { args, .. })
                    if matches!(args.as_slice(), [GenericArg::Type(Type::Generic(name))] if name == "Self")
            ) =>
        {
            Ok(Receiver::BoxedMove)
        }
        _ => Err(
            "receiver is not plain `self`, `&self`, `&mut self`, or `self: Box<Self>`".to_owned(),
        ),
    }
}

pub(super) fn implementation_trait_path(
    trait_path: &RustdocPath,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<(String, bool)> {
    let summary = paths.get(&trait_path.id)?;
    let path = summary.path.join("::");
    (!path.is_empty()).then_some((path, summary.crate_id == 0))
}

pub(super) fn implements_trait(
    impls: &[Id],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    expected: &str,
) -> bool {
    impls.iter().any(|id| {
        let Some(Item {
            inner:
                ItemEnum::Impl(Impl {
                    trait_: Some(trait_),
                    ..
                }),
            ..
        }) = index.get(id)
        else {
            return false;
        };
        paths
            .get(&trait_.id)
            .is_some_and(|summary| summary.path.join("::") == expected)
    })
}

pub(super) fn implements_unconditionally_copy(
    impls: &[Id],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    owner: Id,
    owner_parameters: &[rustdoc_types::GenericParamDef],
) -> bool {
    if owner_parameters.iter().any(|parameter| {
        !matches!(
            &parameter.kind,
            GenericParamDefKind::Lifetime { outlives } if outlives.is_empty()
        )
    }) {
        return false;
    }
    impls.iter().any(|id| {
        let Some(Item {
            inner:
                ItemEnum::Impl(implementation @ Impl {
                    trait_: Some(trait_),
                    ..
                }),
            ..
        }) = index.get(id)
        else {
            return false;
        };
        let Type::ResolvedPath(path) = &implementation.for_ else {
            return false;
        };
        if implementation.is_negative || path.id != owner {
            return false;
        }
        // Every owner lifetime must stay independent: `View<'static>` or
        // `Pair<'a, 'a>` is a specialization, not evidence for the whole family.
        let parameters = &implementation.generics.params;
        let arguments_are_lifetimes = match path.args.as_deref() {
            None => owner_parameters.is_empty() && parameters.is_empty(),
            Some(GenericArgs::AngleBracketed { args, constraints }) => {
                constraints.is_empty()
                    && args.len() == owner_parameters.len()
                    && parameters.len() == owner_parameters.len()
                    && args.iter().enumerate().all(|(index, argument)| {
                        let GenericArg::Lifetime(name) = argument else { return false };
                        parameters.iter().any(|parameter| parameter.name == *name)
                            && !args[..index].iter().any(|previous| {
                                matches!(previous, GenericArg::Lifetime(previous) if previous == name)
                            })
                    })
            }
            _ => false,
        };
        arguments_are_lifetimes
            && implementation.generics.where_predicates.is_empty()
            && implementation.generics.params.iter().all(|parameter| {
                matches!(
                    &parameter.kind,
                    GenericParamDefKind::Lifetime { outlives } if outlives.is_empty()
                )
            })
            && paths
                .get(&trait_.id)
                .is_some_and(|summary| summary.path.join("::") == "core::marker::Copy")
    })
}

/// Keep implementation-only shared-owner modules out of projected identities.
pub(super) fn normalize_shared_owner_path(path: &mut String) {
    let Some((root, remainder)) = path.split_once("::") else {
        return;
    };
    if !matches!(root, "alloc" | "std") {
        return;
    }
    let Some((module, item)) = remainder
        .strip_prefix("rcs::")
        .and_then(|path| path.split_once("::"))
    else {
        return;
    };
    let owner = item.split([':', '<']).next().unwrap_or(item);
    let public = match (module, owner) {
        ("arc", "Arc" | "Weak") => "sync",
        ("rc", "Rc" | "Weak") => "rc",
        _ => return,
    };
    let start = root.len() + 2;
    let end = start + "rcs::".len() + module.len();
    path.replace_range(start..end, public);
}

pub(super) fn resolved_path_name(path: &RustdocPath, paths: &HashMap<Id, ItemSummary>) -> String {
    let mut name = paths
        .get(&path.id)
        .map(|summary| summary.path.join("::"))
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| path.path.replace("crate::", ""));
    normalize_shared_owner_path(&mut name);
    name
}
pub(super) fn impl_trait_bounds(ty: &Type) -> Option<&[GenericBound]> {
    match ty {
        Type::ImplTrait(bounds) => Some(bounds),
        Type::BorrowedRef { type_, .. } => match type_.as_ref() {
            Type::ImplTrait(bounds) => Some(bounds),

            _ => None,
        },
        _ => None,
    }
}

pub(super) fn type_implements_deref_target(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    target_matches: impl Fn(&Type) -> bool,
) -> bool {
    let Type::ResolvedPath(path) = ty else {
        return false;
    };
    let Some(item) = index.get(&path.id) else {
        return false;
    };
    let implementations = match &item.inner {
        ItemEnum::Struct(structure) => &structure.impls,
        ItemEnum::Enum(enumeration) => &enumeration.impls,
        _ => return false,
    };
    implementations.iter().any(|implementation| {
        let Some(Item {
            inner:
                ItemEnum::Impl(Impl {
                    trait_: Some(trait_),
                    items,
                    ..
                }),
            ..
        }) = index.get(implementation)
        else {
            return false;
        };
        if paths
            .get(&trait_.id)
            .is_none_or(|summary| summary.path.join("::") != "core::ops::deref::Deref")
        {
            return false;
        }
        items.iter().any(|item| {
            matches!(
                index.get(item),
                Some(Item {
                    name: Some(name),
                    inner: ItemEnum::AssocType { type_: Some(target), .. },
                    ..
                }) if name == "Target" && target_matches(target)
            )
        })
    })
}

pub(super) fn type_implements_generic_trait(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    expected_trait: &str,
    argument_matches: impl Fn(&Type, &HashMap<Id, ItemSummary>) -> bool,
) -> bool {
    let Type::ResolvedPath(path) = ty else {
        return false;
    };
    let Some(item) = index.get(&path.id) else {
        return false;
    };
    let implementations = match &item.inner {
        ItemEnum::Struct(structure) => &structure.impls,
        ItemEnum::Enum(enumeration) => &enumeration.impls,
        _ => return false,
    };
    implementations.iter().any(|implementation| {
        let Some(Item {
            inner:
                ItemEnum::Impl(Impl {
                    trait_: Some(trait_),
                    ..
                }),
            ..
        }) = index.get(implementation)
        else {
            return false;
        };
        if paths
            .get(&trait_.id)
            .is_none_or(|summary| summary.path.join("::") != expected_trait)
        {
            return false;
        }
        let Some(arguments) = trait_.args.as_deref() else {
            return false;
        };
        let GenericArgs::AngleBracketed { args, .. } = arguments else {
            return false;
        };
        matches!(args.as_slice(), [GenericArg::Type(argument)] if argument_matches(argument, paths))
    })
}

pub(super) fn project_boundary_capabilities(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> ProjectedBoundaryCapabilities {
    ProjectedBoundaryCapabilities {
        string_constructor: type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::From",
            is_rust_string_type,
        )
        .then_some(ProjectedEnumPayloadConversion::Into),
        bytes_constructor: type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::From",
            is_rust_byte_vector_type,
        )
        .then_some(ProjectedEnumPayloadConversion::Into),
        string_extraction: if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::AsRef",
            |argument, _| matches!(argument, Type::Primitive(name) if name == "str"),
        ) {
            Some(ProjectedEnumPayloadConversion::AsRefString)
        } else {
            type_implements_deref_target(
                ty,
                index,
                paths,
                |target| matches!(target, Type::Primitive(name) if name == "str"),
            )
            .then_some(ProjectedEnumPayloadConversion::DerefString)
        },
        bytes_extraction: if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::AsRef",
            |argument, _| {
                matches!(
                    argument,
                    Type::Slice(item) if matches!(item.as_ref(), Type::Primitive(name) if name == "u8")
                )
            },
        ) {
            Some(ProjectedEnumPayloadConversion::AsRefBytes)
        } else {
            type_implements_deref_target(ty, index, paths, |target| {
                matches!(
                    target,
                    Type::Slice(item)
                        if matches!(item.as_ref(), Type::Primitive(name) if name == "u8")
                )
            })
            .then_some(ProjectedEnumPayloadConversion::DerefBytes)
        },
    }
}

pub(super) fn is_rust_string_type(ty: &Type, paths: &HashMap<Id, ItemSummary>) -> bool {
    matches!(
        ty,
        Type::ResolvedPath(path)
            if matches!(
                resolved_path_name(path, paths).as_str(),
                "alloc::string::String" | "std::string::String" | "std::string::string::String"
            )
    )
}

pub(super) fn is_rust_byte_vector_type(ty: &Type, paths: &HashMap<Id, ItemSummary>) -> bool {
    let Type::ResolvedPath(path) = ty else {
        return false;
    };
    if !matches!(
        resolved_path_name(path, paths).as_str(),
        "alloc::vec::Vec" | "std::vec::Vec" | "std::vec::vec::Vec"
    ) {
        return false;
    }
    matches!(
        path.args.as_deref(),
        Some(GenericArgs::AngleBracketed { args, .. })
            if matches!(args.as_slice(), [GenericArg::Type(Type::Primitive(name))] if name == "u8")
    )
}

pub(super) fn resolved_name(ty: &Type, paths: &HashMap<Id, ItemSummary>) -> Option<String> {
    match ty {
        Type::ResolvedPath(path) => Some(resolved_path_name(path, paths)),
        _ => None,
    }
}
#[cfg(test)]
mod tests;
