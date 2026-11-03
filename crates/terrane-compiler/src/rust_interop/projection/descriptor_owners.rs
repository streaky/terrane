//! Publish the concrete opaque owners used by descriptor fields as real nominal
//! projection items. A generated source-only placeholder is not enough: authors
//! must be able to import the type for a binding or a typed Rust expression.
use super::{BTreeMap, ProjectedBoundaryCapabilities, ProjectedItem, ProjectedKind, ProjectedType};

pub(super) fn add_backing_owners(items: &mut Vec<ProjectedItem>, namespace: &str) {
    let mut referenced = BTreeMap::new();
    for item in items.iter() {
        if let ProjectedKind::ForeignType {
            fields,
            borrowed_view: true,
            ..
        } = &item.kind
        {
            for field in fields {
                super::source_rendering::collect_foreign_type(&field.ty, &mut referenced);
            }
        }
    }
    for (rust_path, name) in referenced {
        if items.iter().any(|item| item.rust_path == rust_path)
            || items
                .iter()
                .any(|item| item.namespace == namespace && item.name == name)
        {
            continue;
        }
        items.push(ProjectedItem {
            namespace: namespace.to_owned(),
            name,
            rust_path,
            docs: None,
            kind: ProjectedKind::ForeignType {
                methods: Vec::new(),
                static_methods: Vec::new(),
                constants: Vec::new(),
                constructor: None,
                fields: Vec::new(),
                borrowed_view: false,
                native_view_type: None,
                enum_payload: None,
                generic_parameters: Vec::new(),
                boundary: ProjectedBoundaryCapabilities::default(),
                displayable: false,
                cloneable: false,
                send: false,
                sync: false,
            },
        });
    }
}

pub(super) fn result_retains_view(ty: &ProjectedType) -> bool {
    match ty {
        ProjectedType::Reference { .. } | ProjectedType::InvocationScoped { .. } => true,
        ProjectedType::Optional(inner) | ProjectedType::Sequence { item: inner, .. } => {
            result_retains_view(inner)
        }
        ProjectedType::Tuple(items)
        | ProjectedType::Foreign {
            arguments: items, ..
        } => items.iter().any(result_retains_view),
        ProjectedType::Mapping { key, value, .. } => {
            result_retains_view(key) || result_retains_view(value)
        }
        _ => false,
    }
}
