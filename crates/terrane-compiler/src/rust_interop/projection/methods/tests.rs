use super::*;

#[test]
fn rust_protocol_plumbing_is_not_reported_as_a_callable_gap() {
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
