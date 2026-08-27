// Delivery principle 9: opaque values cross the irreducible host boundary. Accessors form one
// shared ABI selected by several independently imported Terrane facilities.
type TerranePlatformCapability = terrane_platform_support::Capability;
type TerranePlatformResult = terrane_platform_support::ResultValue;
fn terrane_platform_i128(value: &terrane_int_support::Int, label: &str) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| TerranePlatformResult::error(format!("{label} is outside the signed 128-bit platform range")))
}
macro_rules! terrane_platform_i128 {
    ($value:expr, $label:literal) => {
        match terrane_platform_i128(&$value, $label) {
            Ok(value) => value,
            Err(error) => return error,
        }
    };
}
#[allow(dead_code)] fn terrane_platform_cancellation_token() -> TerranePlatformCapability { terrane_platform_support::cancellation_token() }
#[allow(dead_code)] fn terrane_platform_cancel(token: &TerranePlatformCapability) -> TerranePlatformResult { terrane_platform_support::cancel(token) }
#[allow(dead_code)] fn terrane_platform_result_failed(result: &TerranePlatformResult) -> bool { result.failed }
#[allow(dead_code)] fn terrane_platform_result_resource_limit(result: &TerranePlatformResult) -> bool { result.resource_limit }
#[allow(dead_code)] fn terrane_platform_result_truncated(result: &TerranePlatformResult) -> bool { result.truncated }
#[allow(dead_code)] fn terrane_platform_result_deadline_exceeded(result: &TerranePlatformResult) -> bool { result.deadline_exceeded }
#[allow(dead_code)] fn terrane_platform_result_message(result: &TerranePlatformResult) -> String { result.message.clone() }
#[allow(dead_code)] fn terrane_platform_result_text(result: &TerranePlatformResult) -> String { result.text.clone() }
#[allow(dead_code)] fn terrane_platform_result_detail(result: &TerranePlatformResult) -> String { result.detail.clone() }
#[allow(dead_code)] fn terrane_platform_result_bytes(result: &TerranePlatformResult) -> Vec<u8> { result.data.clone() }
#[allow(dead_code)] fn terrane_platform_result_int(result: &TerranePlatformResult) -> terrane_int_support::Int { terrane_int_support::Int::from(result.number) }
#[allow(dead_code)] fn terrane_platform_result_bool(result: &TerranePlatformResult) -> bool { result.flag }
#[allow(dead_code)] fn terrane_platform_result_entries(result: &TerranePlatformResult) -> Vec<String> { result.entries.clone() }
#[allow(dead_code)] fn terrane_platform_result_capability(result: &TerranePlatformResult) -> TerranePlatformCapability { result.capability.clone().unwrap_or_default() }
