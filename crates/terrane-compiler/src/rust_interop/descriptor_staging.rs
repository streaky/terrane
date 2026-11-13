//! Consuming mixed containers must separate stable lenders from moved payloads.
//! A temporary owner graph holds borrowed leaves in place and wraps independent
//! owned subgraphs in Option so the view can take them without cloning or Default.
use crate::rust_interop::projection::{ProjectedFieldConversion as Conversion, ProjectedType};

fn has_owned_leaf(conversion: &Conversion) -> bool {
    match conversion {
        Conversion::Identity | Conversion::OptionalOwned | Conversion::OwnedNative { .. } => true,
        Conversion::Optional(inner) | Conversion::Sequence(inner) => has_owned_leaf(inner),
        Conversion::Tuple(items) => items.iter().any(has_owned_leaf),
        Conversion::Mapping { key, value } => has_owned_leaf(key) || has_owned_leaf(value),
        _ => false,
    }
}

pub(super) fn required(conversion: &Conversion) -> bool {
    conversion.has_borrowed_leaves() && has_owned_leaf(conversion)
}

pub(super) fn owners(source: &str, conversion: &Conversion) -> String {
    if !conversion.has_borrowed_leaves() {
        return format!("Some({source})");
    }
    match conversion {
        Conversion::Optional(inner) => {
            format!("({source}).map(|owned| {})", owners("owned", inner))
        }
        Conversion::Tuple(items) => {
            let names = (0..items.len())
                .map(|index| format!("owned_{index}"))
                .collect::<Vec<_>>();
            let values = names
                .iter()
                .zip(items)
                .map(|(name, recipe)| owners(name, recipe))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{ let ({},) = {source}; ({values},) }}", names.join(", "))
        }
        Conversion::Sequence(inner) => format!(
            "({source}).into_iter().map(|owned| {}).collect::<Vec<_>>()",
            owners("owned", inner)
        ),
        Conversion::Mapping { key, value } => format!(
            "({source}).into_iter().map(|(key, value)| ({}, {})).collect::<Vec<_>>()",
            owners("key", key),
            owners("value", value)
        ),
        _ => source.to_owned(),
    }
}

pub(super) fn native_view(stage: &str, ty: &ProjectedType, conversion: &Conversion) -> String {
    if !conversion.has_borrowed_leaves() {
        return format!("({stage}).take().expect(\"descriptor owned leaf is transferred once\")");
    }
    match conversion {
        Conversion::Optional(inner) => {
            let ProjectedType::Optional(ty) = ty else {
                unreachable!("optional recipe type")
            };
            format!(
                "({stage}).as_mut().map(|stage| {})",
                native_view("(*stage)", ty, inner)
            )
        }
        Conversion::Tuple(recipes) => {
            let ProjectedType::Tuple(items) = ty else {
                unreachable!("tuple recipe type")
            };
            let values = items
                .iter()
                .zip(recipes)
                .enumerate()
                .map(|(index, (ty, recipe))| native_view(&format!("({stage}).{index}"), ty, recipe))
                .collect::<Vec<_>>()
                .join(", ");
            format!("({values},)")
        }
        Conversion::Sequence(inner) => {
            let ProjectedType::Sequence { item, .. } = ty else {
                unreachable!("sequence recipe type")
            };
            format!(
                "({stage}).iter_mut().map(|stage| {}).collect()",
                native_view("(*stage)", item, inner)
            )
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
            format!(
                "({stage}).iter_mut().map(|(key, value)| ({}, {})).collect()",
                native_view("(*key)", key_type, key),
                native_view("(*value)", value_type, value)
            )
        }
        _ => super::native_field(stage, ty, conversion, false),
    }
}
