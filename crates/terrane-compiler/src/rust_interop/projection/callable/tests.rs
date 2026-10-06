use super::*;
use rustdoc_types::{ItemKind, Trait};

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one identity matrix keeps callable and marker trait aliases together"
)]
fn builtin_callback_trait_recognition_uses_canonical_identity() {
    let fn_id = Id(1);
    let spoofed_fn_id = Id(2);
    let fn_mut_id = Id(3);
    let fn_once_id = Id(4);
    let send_id = Id(5);
    let spoofed_send_id = Id(6);
    let sync_id = Id(7);
    let clone_id = Id(8);
    let paths = HashMap::from([
        (
            fn_id,
            ItemSummary {
                crate_id: 0,
                path: vec![
                    "core".to_owned(),
                    "ops".to_owned(),
                    "function".to_owned(),
                    "Fn".to_owned(),
                ],
                kind: ItemKind::Trait,
            },
        ),
        (
            spoofed_fn_id,
            ItemSummary {
                crate_id: 1,
                path: vec!["witness".to_owned(), "Fn".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
        (
            fn_mut_id,
            ItemSummary {
                crate_id: 0,
                path: vec![
                    "core".to_owned(),
                    "ops".to_owned(),
                    "function".to_owned(),
                    "FnMut".to_owned(),
                ],
                kind: ItemKind::Trait,
            },
        ),
        (
            fn_once_id,
            ItemSummary {
                crate_id: 0,
                path: vec![
                    "core".to_owned(),
                    "ops".to_owned(),
                    "function".to_owned(),
                    "FnOnce".to_owned(),
                ],
                kind: ItemKind::Trait,
            },
        ),
        (
            send_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["core".to_owned(), "marker".to_owned(), "Send".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
        (
            spoofed_send_id,
            ItemSummary {
                crate_id: 1,
                path: vec!["witness".to_owned(), "Send".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
        (
            sync_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["core".to_owned(), "marker".to_owned(), "Sync".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
        (
            clone_id,
            ItemSummary {
                crate_id: 0,
                path: vec!["core".to_owned(), "clone".to_owned(), "Clone".to_owned()],
                kind: ItemKind::Trait,
            },
        ),
    ]);
    let path = |id, display: &str| RustdocPath {
        path: display.to_owned(),
        id,
        args: None,
    };

    assert_eq!(
        builtin_callable_mode(&path(fn_id, "renamed::Anything"), &paths),
        Some(InvocationMode::Shared)
    );
    assert_eq!(
        builtin_callable_mode(&path(fn_mut_id, "renamed::Anything"), &paths),
        Some(InvocationMode::Mutable)
    );
    assert_eq!(
        builtin_callable_mode(&path(fn_once_id, "renamed::Anything"), &paths),
        Some(InvocationMode::Consuming)
    );
    assert_eq!(
        builtin_callable_mode(&path(spoofed_fn_id, "Fn"), &paths),
        None
    );
    assert!(is_builtin_marker_trait(
        &path(send_id, "renamed::Anything"),
        &paths,
        "Send"
    ));
    assert!(!is_builtin_marker_trait(
        &path(spoofed_send_id, "Send"),
        &paths,
        "Send"
    ));
    assert!(is_builtin_marker_trait(
        &path(sync_id, "renamed::Anything"),
        &paths,
        "Sync"
    ));
    assert!(is_builtin_clone(
        &path(clone_id, "renamed::Anything"),
        &paths
    ));
    assert!(!is_builtin_clone(&path(spoofed_fn_id, "Clone"), &paths));
}

#[test]
fn custom_callable_traits_reject_restrictive_shapes() {
    let base = Trait {
        is_auto: false,
        is_unsafe: false,
        is_dyn_compatible: true,
        items: vec![Id(1)],
        generics: Generics {
            params: Vec::new(),
            where_predicates: Vec::new(),
        },
        bounds: Vec::new(),
        implementations: Vec::new(),
    };
    let index = HashMap::new();

    for declaration in [
        Trait {
            is_auto: true,
            ..base.clone()
        },
        Trait {
            is_unsafe: true,
            ..base.clone()
        },
        Trait {
            bounds: vec![GenericBound::Outlives("'static".to_owned())],
            ..base.clone()
        },
        Trait {
            items: vec![Id(1), Id(2)],
            ..base
        },
    ] {
        assert!(!has_supported_callable_trait_shape(&declaration, &index));
    }
}
