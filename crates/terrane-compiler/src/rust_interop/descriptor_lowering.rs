//! Owned descriptor storage is distinct from its short-lived native borrow graph.
//! Source containers are converted once at construction; a call lends borrowed
//! leaves and either copies/clones reusable owned leaves or moves consumed ones.
use crate::rust_interop::projection::{ProjectedFieldConversion as Conversion, ProjectedType};
#[path = "descriptor_staging.rs"]
mod staging;

pub(super) fn staged_field(
    source: &str,
    owner: &str,
    index: usize,
    ty: &ProjectedType,
    conversion: &Conversion,
) -> Option<(String, String)> {
    if !staging::required(conversion) {
        return None;
    }
    let stage = format!("__terrane_stage_{}_{index}", owner.trim_start_matches('_'));
    let owners = staging::owners(source, conversion);
    Some((
        format!("let mut {stage} = {owners}; "),
        staging::native_view(&stage, ty, conversion),
    ))
}

pub(super) fn source_type(ty: &ProjectedType) -> String {
    match ty {
        ProjectedType::Optional(inner) => format!("Option<{}>", source_type(inner)),
        ProjectedType::Sequence { item, .. } => {
            format!("terrane_collection_support::List<{}>", source_type(item))
        }
        ProjectedType::Tuple(items) => format!(
            "terrane_collection_support::Tuple<{}>",
            source_type(&items[0])
        ),
        ProjectedType::Mapping {
            key,
            value,
            ordered,
            ..
        } => format!(
            "terrane_collection_support::{}Map<{}, {}>",
            if *ordered { "" } else { "Unordered" },
            source_type(key),
            source_type(value)
        ),
        ProjectedType::Char => "String".to_owned(),
        _ => ty.rust_type(),
    }
}

fn copy_value(ty: &ProjectedType) -> bool {
    match ty {
        ProjectedType::Bool
        | ProjectedType::None
        | ProjectedType::FixedInt(_)
        | ProjectedType::RustInt(_)
        | ProjectedType::Float
        | ProjectedType::Float32
        | ProjectedType::Char => true,
        ProjectedType::Optional(inner) => copy_value(inner),
        ProjectedType::Tuple(items) => items.iter().all(copy_value),
        _ => false,
    }
}

fn borrows(conversion: &Conversion) -> bool {
    match conversion {
        Conversion::Identity | Conversion::OptionalOwned | Conversion::OwnedNative { .. } => false,
        Conversion::Optional(inner) | Conversion::Sequence(inner) => borrows(inner),
        Conversion::Tuple(items) => items.iter().any(borrows),
        Conversion::Mapping { key, value } => borrows(key) || borrows(value),
        _ => true,
    }
}

/// `source` names native owned backing storage, never a temporary source value.
/// Moving only independent owned fields preserves lenders in other fields.
pub(super) fn native_field(
    source: &str,
    ty: &ProjectedType,
    conversion: &Conversion,
    consuming: bool,
) -> String {
    if let Conversion::OwnedNative { copyable, .. } = conversion {
        return if consuming || *copyable {
            source.to_owned()
        } else {
            format!("({source}).clone()")
        };
    }
    if !borrows(conversion) {
        return if consuming || copy_value(ty) {
            source.to_owned()
        } else {
            format!("({source}).clone()")
        };
    }
    match conversion {
        Conversion::StringBorrow => format!("({source}).as_str()"),
        Conversion::OsStrBorrow => format!("std::ffi::OsStr::new(&({source}))"),
        Conversion::SliceBorrow => format!("({source}).as_slice()"),
        Conversion::Optional(inner) => {
            let ProjectedType::Optional(item) = ty else {
                unreachable!("optional recipe type")
            };
            let value = native_field("(*value)", item, inner, false);
            format!("({source}).as_ref().map(|value| {value})")
        }
        Conversion::Tuple(recipes) => {
            let ProjectedType::Tuple(items) = ty else {
                unreachable!("tuple recipe type")
            };
            let values = items
                .iter()
                .zip(recipes)
                .enumerate()
                .map(|(index, (item, recipe))| {
                    native_field(&format!("({source}).{index}"), item, recipe, consuming)
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!("({values},)")
        }
        Conversion::Sequence(inner) => {
            let ProjectedType::Sequence { item, .. } = ty else {
                unreachable!("sequence recipe type")
            };
            let value = native_field("(*item)", item, inner, false);
            format!("({source}).iter().map(|item| {value}).collect()")
        }
        Conversion::Mapping { key, value } => {
            let ProjectedType::Mapping {
                key: key_type,
                value: value_type,
                ..
            } = ty
            else {
                unreachable!("mapping recipe type")
            };
            let key = native_field("(*key)", key_type, key, false);
            let value = native_field("(*value)", value_type, value, false);
            format!("({source}).iter().map(|(key, value)| ({key}, {value})).collect()")
        }
        Conversion::Identity | Conversion::OptionalOwned | Conversion::OwnedNative { .. } => {
            unreachable!("owned recipe handled above")
        }
    }
}
