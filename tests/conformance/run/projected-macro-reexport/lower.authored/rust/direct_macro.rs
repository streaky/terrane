use terrane_generate_api_witness::GeneratedValueApi;
#[derive(GeneratedValueApi)]
struct GeneratedValue {
    value: i64,
}
pub fn answer() -> i64 {
    GeneratedValue::standard().value()
}
