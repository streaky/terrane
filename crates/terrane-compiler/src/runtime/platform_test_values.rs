fn terrane_test_render_int(value: terrane_int_support::Int) -> String {
    value.to_string()
}

fn terrane_test_render_float64(value: f64) -> String {
    value.to_string()
}

fn terrane_test_render_bytes(value: Vec<u8>) -> String {
    format!("0x{}", terrane_hex(&value))
}

fn terrane_test_render_bool(value: bool) -> String {
    value.to_string()
}

fn terrane_test_deadline_nanoseconds() -> terrane_int_support::Int {
    let value = std::env::var("TERRANE_TEST_DEADLINE_NANOSECONDS")
        .ok()
        .and_then(|value| value.parse::<u128>().ok())
        .unwrap_or_default();
    terrane_int_support::Int::from_u128(value)
}

fn terrane_test_time_advance(nanoseconds: terrane_int_support::Int) -> bool {
    let Some(nanoseconds) = terrane_int_support::checked_coerce::<u64>(&nanoseconds) else {
        return false;
    };

    if TERRANE_TEST_TIME_NANOS
        .try_update(
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            |current| current.checked_add(nanoseconds),
        )
        .is_err()
    {
        return false;
    }
    true
}
