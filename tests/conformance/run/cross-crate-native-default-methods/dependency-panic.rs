#[cfg(test)]
mod dependency_panic_tests {
    #[test]
    fn generic_member_retains_concrete_display_label() {
        assert!(super::generic_panic_attributed(witness::Wrapped(9_u8)));
    }

    #[test]
    fn free_function_uses_canonical_package_display_label() {
        assert!(super::free_panic_attributed());
    }

    #[test]
    fn generic_free_package_label() {
        assert!(super::generic_free_panic_attributed());
    }
}
