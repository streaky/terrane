// Generated deterministically by Terrane <version>.
pub type TerranePlatformCapability = terrane_platform_support::Capability;
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
#[allow(dead_code)]
fn terrane_platform_cancellation_token() -> TerranePlatformCapability {
    terrane_platform_support::cancellation_token()
}
#[allow(dead_code)]
fn terrane_platform_no_resource() -> TerranePlatformCapability {
    TerranePlatformCapability::default()
}
#[allow(dead_code)]
fn terrane_platform_failed_result() -> TerranePlatformResult {
    TerranePlatformResult::error("uninitialized platform value")
}
#[allow(dead_code)]
fn terrane_platform_cancel(token: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::cancel(token)
}
#[allow(dead_code)]
fn terrane_platform_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
#[allow(dead_code)]
fn terrane_platform_result_resource_limit(result: &TerranePlatformResult) -> bool {
    result.resource_limit
}
#[allow(dead_code)]
fn terrane_platform_result_truncated(result: &TerranePlatformResult) -> bool {
    result.truncated
}
#[allow(dead_code)]
fn terrane_platform_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
#[allow(dead_code)]
fn terrane_platform_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_text(result: &TerranePlatformResult) -> String {
    result.text.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_detail(result: &TerranePlatformResult) -> String {
    result.detail.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_bytes(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_int(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
#[allow(dead_code)]
fn terrane_platform_result_bool(result: &TerranePlatformResult) -> bool {
    result.flag
}
#[allow(dead_code)]
fn terrane_platform_result_entries(result: &TerranePlatformResult) -> Vec<String> {
    result.entries.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_capability(
    result: &TerranePlatformResult,
) -> TerranePlatformCapability {
    result.capability.clone().unwrap_or_default()
}
pub fn terrane_platform_hex_encode(data: Vec<u8>) -> String {
    terrane_platform_support::hex_encode(&data)
}
pub fn terrane_platform_hex_decode(text: String) -> TerranePlatformResult {
    terrane_platform_support::hex_decode(&text)
}
pub fn terrane_platform_base64_encode(
    data: Vec<u8>,
    url_safe: bool,
    padded: bool,
) -> String {
    terrane_platform_support::base64_encode(&data, url_safe, padded)
}
pub fn terrane_platform_base64_decode(
    text: String,
    url_safe: bool,
    padded: bool,
) -> TerranePlatformResult {
    terrane_platform_support::base64_decode(&text, url_safe, padded)
}
// Source: case.trn
// Namespace: core-tool-imports
fn describe(enabled: bool, payload: Vec<u8>, small: i8) -> String {
    let _ = &small;
    terrane_platform_base64_encode(payload, enabled, enabled);
    return String::from("core tools are explicit");
}
fn main() {
    describe(true, Vec::from([97, 98, 99]), 1);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&String::from("core namespace import works"))
    );
}
