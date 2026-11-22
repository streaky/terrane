// Rust justification: ABI boundary to the bounded writer shared by JSON and YAML.
pub fn terrane_json_stringify(value: &terrane_document_support::DataResult, max_depth: terrane_int_support::Int, max_bytes: terrane_int_support::Int) -> terrane_document_support::DataResult {
    terrane_document_support::stringify_json_bounded(value, terrane_limit(&max_depth), terrane_limit(&max_bytes))
}
