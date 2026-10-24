#[cfg(test)]
mod dependency_panic_tests {
    #[test]
    fn generic_member_retains_concrete_display_label() {
        assert!(super::generic_panic_attributed(witness::Wrapped(9_u8)));
    }
}
