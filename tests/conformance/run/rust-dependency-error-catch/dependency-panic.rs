#[cfg(test)]
mod dependency_panic_tests {
    #[test]
    fn preserves_payload_and_crossing_context() {
        let payload = std::panic::catch_unwind(|| panic!("fixture panic"))
            .expect_err("fixture must panic");
        let error = super::__terrane_dependency_panic(
            payload,
            "fixture-crate",
            "fixture_crate::explode",
        );
        assert_eq!(
            error.render(),
            "dependency-panic: Rust dependency `fixture-crate` member `fixture_crate::explode` panicked: fixture panic"
        );
    }
}
