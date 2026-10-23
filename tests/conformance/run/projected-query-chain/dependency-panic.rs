#[cfg(test)]
mod dependency_panic_tests {
    #[test]
    fn row_calls_attribute_native_panics() {
        for result in [
            super::trigger_static_panic(terrane_query_witness::panic_row()),
            super::trigger_instance_panic(terrane_query_witness::panic_row()),
        ] {
            let error = result.expect_err("the generated boundary must map the native panic");
            let rendered = error.render();
            assert!(
                rendered.contains("Rust dependency `terrane_query_witness`"),
                "{rendered}"
            );
            assert!(
                rendered.contains("member `terrane_query_witness::Row::try_get`"),
                "{rendered}"
            );
            assert!(rendered.contains("row accessor panic"), "{rendered}");
        }
    }
}
