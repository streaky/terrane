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
