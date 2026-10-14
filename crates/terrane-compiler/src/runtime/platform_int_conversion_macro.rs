macro_rules! terrane_platform_i128 {
    ($value:expr, $label:literal) => {
        match terrane_platform_i128(&$value, $label) {
            Ok(value) => value,
            Err(error) => return error,
        }
    };
}
