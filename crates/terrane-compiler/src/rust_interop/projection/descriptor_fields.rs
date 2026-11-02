//! Project owned source backing and exact reconstruction recipes together.
use super::{
    BTreeMap, HashMap, Id, Item, ItemEnum, ItemSummary, ProjectedFieldConversion, ProjectedType,
    Type, canonicalize_rust_path, implements_trait, project_type, resolved_path_name,
    type_arguments, type_contains_lifetime_argument,
};
use ProjectedFieldConversion as Conversion;

pub(super) fn project_field(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<(ProjectedType, ProjectedFieldConversion), String> {
    match ty {
        Type::BorrowedRef {
            is_mutable: true, ..
        } => Err("mutable borrowed fields have no owned descriptor conversion".to_owned()),
        Type::BorrowedRef {
            lifetime, type_, ..
        } => {
            if lifetime
                .as_deref()
                .is_some_and(|lifetime| lifetime.trim_start_matches('\'') == "static")
            {
                return Err(
                    "static borrowed fields cannot be reconstructed from local backing storage"
                        .to_owned(),
                );
            }
            match type_.as_ref() {
                Type::Primitive(name) if name == "str" => {
                    Ok((ProjectedType::String, Conversion::StringBorrow))
                }
                Type::ResolvedPath(path)
                    if canonicalize_rust_path(&resolved_path_name(path, paths))
                        == "std::ffi::OsStr"
                        || resolved_path_name(path, paths) == "std::ffi::os_str::OsStr" =>
                {
                    Ok((ProjectedType::String, Conversion::OsStrBorrow))
                }
                Type::Slice(item)
                    if !type_contains_lifetime_argument(item)
                        && !matches!(item.as_ref(), Type::BorrowedRef { .. }) =>
                {
                    let (item, _) = project_field(item, index, paths, generics)?;
                    Ok((
                        ProjectedType::Sequence {
                            rust_path: format!("Vec<{}>", item.rust_type()),
                            item: Box::new(item),
                        },
                        Conversion::SliceBorrow,
                    ))
                }
                _ => Err("borrowed field has no stable owned backing conversion".to_owned()),
            }
        }
        Type::Tuple(items) if !items.is_empty() => {
            let fields = items
                .iter()
                .map(|item| project_field(item, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            if fields.windows(2).any(|pair| pair[0].0 != pair[1].0) {
                return Err("heterogeneous tuple has no Terrane tuple representation".to_owned());
            }
            let (items, recipes) = fields.into_iter().unzip();
            Ok((ProjectedType::Tuple(items), Conversion::Tuple(recipes)))
        }
        Type::ResolvedPath(_) => project_nominal_field(ty, index, paths, generics),
        _ if !type_contains_lifetime_argument(ty) => Ok((
            project_type(ty, index, paths, generics)?,
            Conversion::Identity,
        )),
        _ => Err("field has no recursive owned conversion".to_owned()),
    }
}

fn project_nominal_field(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<(ProjectedType, ProjectedFieldConversion), String> {
    let Type::ResolvedPath(path) = ty else {
        unreachable!("nominal field")
    };
    let canonical = canonicalize_rust_path(&resolved_path_name(path, paths));
    let arguments = type_arguments(ty);
    let container = canonical.rsplit("::").next().unwrap_or(&canonical);
    match (container, arguments.as_slice()) {
        ("Option", [inner]) => {
            let (item, recipe) = project_field(inner, index, paths, generics)?;
            if matches!(item, ProjectedType::Optional(_) | ProjectedType::None) {
                return Err(
                    "nested optional carriers have no distinct Terrane value representation"
                        .to_owned(),
                );
            }
            Ok((
                ProjectedType::Optional(Box::new(item)),
                Conversion::Optional(Box::new(recipe)),
            ))
        }
        ("Vec", [inner]) => {
            let (item, recipe) = project_field(inner, index, paths, generics)?;
            Ok((
                ProjectedType::Sequence {
                    rust_path: format!("Vec<{}>", item.rust_type()),
                    item: Box::new(item),
                },
                Conversion::Sequence(Box::new(recipe)),
            ))
        }
        ("HashMap" | "BTreeMap", [key, value]) => {
            let (key, key_recipe) = project_field(key, index, paths, generics)?;
            let (value, value_recipe) = project_field(value, index, paths, generics)?;
            let ordered = container == "BTreeMap";
            Ok((
                ProjectedType::Mapping {
                    rust_path: format!(
                        "std::collections::{container}<{}, {}>",
                        key.rust_type(),
                        value.rust_type()
                    ),
                    key: Box::new(key),
                    value: Box::new(value),
                    ordered,
                },
                Conversion::Mapping {
                    key: Box::new(key_recipe),
                    value: Box::new(value_recipe),
                },
            ))
        }
        _ if !type_contains_lifetime_argument(ty) => {
            let projected = project_type(ty, index, paths, generics)?;
            let conversion = if matches!(projected, ProjectedType::Foreign { .. }) {
                // These standard-library value contracts remain known even
                // when rustdoc only includes an external type summary.
                let known_copy =
                    canonical == "core::time::Duration" || canonical == "std::time::Duration";
                let known_clone = known_copy
                    || matches!(
                        canonical.as_str(),
                        "std::path::PathBuf" | "std::ffi::OsString" | "std::ffi::os_str::OsString"
                    );
                let implementations = match index.get(&path.id).map(|item| &item.inner) {
                    Some(ItemEnum::Struct(item)) => item.impls.as_slice(),
                    Some(ItemEnum::Enum(item)) => item.impls.as_slice(),
                    _ => &[],
                };
                Conversion::OwnedNative {
                    copyable: known_copy
                        || implements_trait(implementations, index, paths, "core::marker::Copy"),
                    cloneable: known_clone
                        || implements_trait(implementations, index, paths, "core::clone::Clone"),
                }
            } else {
                Conversion::Identity
            };
            Ok((projected, conversion))
        }
        _ => Err("lifetime-bearing native field has no recursive owned conversion".to_owned()),
    }
}
