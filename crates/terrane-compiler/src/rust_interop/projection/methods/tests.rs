use super::*;

#[test]
fn protocol_plumbing_is_not_a_gap() {
    assert!(is_internal_rust_protocol_method("core::fmt::Debug", "fmt"));
    assert!(is_internal_rust_protocol_method(
        "core::fmt::Display",
        "fmt"
    ));
    assert!(is_internal_rust_protocol_method("core::hash::Hash", "hash"));
    assert!(!is_internal_rust_protocol_method(
        "core::convert::From",
        "from"
    ));
}

#[test]
fn substitution_keeps_function_binders() {
    let source = "(Owner<'owner>, for<'call> fn(&'call str, &'other str) -> &'other str)";
    let substitutions = BTreeMap::from([
        ("'owner".to_owned(), "'scope".to_owned()),
        ("'other".to_owned(), "'stored".to_owned()),
        ("'call".to_owned(), "'wrong".to_owned()),
    ]);
    let actual: syn::Type =
        syn::parse_str(&substitute_rust_lifetimes(source, &substitutions)).unwrap();
    let expected: syn::Type =
        syn::parse_str("(Owner<'scope>, for<'call> fn(&'call str, &'stored str) -> &'stored str)")
            .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(
        lifetime_occurrences(source).unwrap(),
        ["'owner", "'other", "'other"],
    );
}

#[test]
fn substitution_keeps_trait_binders() {
    let source = "dyn for<'call> Fn(&'call str, &'owner str) -> &'call str + 'environment";
    let substitutions = BTreeMap::from([
        ("'owner".to_owned(), "'scope".to_owned()),
        ("'environment".to_owned(), "'static".to_owned()),
        ("'call".to_owned(), "'wrong".to_owned()),
    ]);
    let actual: syn::Type =
        syn::parse_str(&substitute_rust_lifetimes(source, &substitutions)).unwrap();
    let expected: syn::Type =
        syn::parse_str("dyn for<'call> Fn(&'call str, &'scope str) -> &'call str + 'static")
            .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(
        lifetime_occurrences(source).unwrap(),
        ["'owner", "'environment"]
    );
}
