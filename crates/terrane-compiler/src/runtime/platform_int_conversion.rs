pub fn terrane_platform_i128(
    value: &terrane_int_support::Int,
    label: &str,
) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| TerranePlatformResult::error(format!("{label} is outside the signed 128-bit platform range")))
}

pub fn terrane_platform_timeout_nanos(value: terrane_int_support::Int) -> i128 {
    if value <= terrane_int_support::Int::from(0_i128) {
        return 0;
    }
    i128::from(terrane_int_support::coerce::<u64>(&value).unwrap_or(u64::MAX))
}
macro_rules! terrane_platform_i128 {
    ($value:expr, $label:literal) => {
        match terrane_platform_i128(&$value, $label) {
            Ok(value) => value,
            Err(error) => return error,
        }
    };
}
