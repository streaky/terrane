//! Publish opaque native backing owners as real importable nominal declarations.
//! A source-only placeholder cannot carry binding, native identity or capability
//! proofs. Descriptor fields and closed weak handles share this publication path.
use super::{BTreeMap, ProjectedBoundaryCapabilities, ProjectedItem, ProjectedKind, ProjectedType};

pub(super) fn add_backing_owners(items: &mut Vec<ProjectedItem>, namespace: &str) {
    let mut referenced = BTreeMap::new();
    let mut signature_types = BTreeMap::new();
    for item in items.iter() {
        collect_signature_types(&item.kind, &mut signature_types);
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
    signature_types.retain(|rust_path, _| is_weak_path(rust_path));
    let signature_aliases = super::source_rendering::foreign_aliases(&signature_types);
    for (rust_path, name) in signature_types {
        if weak_capabilities(&rust_path).is_some() {
            referenced.insert(
                rust_path.clone(),
                signature_aliases.get(&rust_path).cloned().unwrap_or(name),
            );
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
        let weak = weak_capabilities(&rust_path);
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
                cloneable: weak.is_some(),
                send: weak.is_some_and(|thread_safe| thread_safe),
                sync: weak.is_some_and(|thread_safe| thread_safe),
            },
        });
    }
}

fn collect_signature_types(kind: &ProjectedKind, foreign: &mut BTreeMap<String, String>) {
    use super::source_rendering::{collect_foreign_function, collect_foreign_type};
    match kind {
        ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
            collect_foreign_function(function, foreign);
        }
        ProjectedKind::ForeignType {
            methods,
            static_methods,
            constructor,
            fields,
            ..
        } => {
            for function in methods.iter().chain(static_methods).chain(constructor) {
                collect_foreign_function(function, foreign);
            }
            for field in fields {
                collect_foreign_type(&field.ty, foreign);
            }
        }
        ProjectedKind::Enum {
            methods,
            static_methods,
            variants,
            ..
        } => {
            for function in methods.iter().chain(static_methods) {
                collect_foreign_function(function, foreign);
            }
            for field in variants.iter().flat_map(|variant| &variant.fields) {
                collect_foreign_type(&field.ty, foreign);
            }
        }
        ProjectedKind::Interface(interface) => {
            for method in &interface.methods {
                collect_foreign_function(&method.function, foreign);
            }
        }
    }
}

// Weak<T> is always Clone; moving/sharing it across threads requires both
// Send and Sync on its pointee. The admission recipe preserves those exact
// dyn bounds rather than inferring them from a callback's invocation mode.
fn is_weak_path(rust_path: &str) -> bool {
    rust_path.starts_with("std::sync::Weak<") || rust_path.starts_with("alloc::sync::Weak<")
}

fn weak_capabilities(rust_path: &str) -> Option<bool> {
    if !is_weak_path(rust_path) {
        return None;
    }
    let syn::Type::Path(path) = syn::parse_str::<syn::Type>(rust_path).ok()? else {
        return None;
    };
    let owner = &path.path;
    if owner.segments.len() != 3
        || !(owner.segments[0].ident == "std" || owner.segments[0].ident == "alloc")
        || owner.segments[1].ident != "sync"
        || owner.segments[2].ident != "Weak"
    {
        return None;
    }
    let syn::PathArguments::AngleBracketed(arguments) = &owner.segments[2].arguments else {
        return None;
    };
    let syn::GenericArgument::Type(syn::Type::TraitObject(dynamic)) = arguments.args.first()?
    else {
        return None;
    };
    let has_marker = |marker: &str| {
        dynamic.bounds.iter().any(|bound| {
            let syn::TypeParamBound::Trait(bound) = bound else {
                return false;
            };
            let segments = &bound.path.segments;
            segments.len() == 3
                && (segments[0].ident == "std" || segments[0].ident == "core")
                && segments[1].ident == "marker"
                && segments[2].ident == marker
        })
    };
    Some(has_marker("Send") && has_marker("Sync"))
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
