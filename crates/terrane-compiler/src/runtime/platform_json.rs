// Rust justification: ABI boundary to the bounded parser support layer.

fn terrane_json_parse(input: String, max_depth: terrane_int_support::Int, max_bytes: terrane_int_support::Int) -> terrane_document_support::DataResult {
    terrane_document_support::parse_json(&input, terrane_limit(&max_depth), terrane_limit(&max_bytes))
}

fn terrane_json_canonical(value: &terrane_document_support::DataResult) -> terrane_document_support::DataResult {
    terrane_document_support::canonical_json(value)
}
