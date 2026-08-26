fn terrane_json_parse(input: String, reject_duplicates: bool, max_depth: terrane_int_support::Int, max_bytes: terrane_int_support::Int) -> terrane_document_support::DataResult {
    terrane_document_support::parse_json(&input, reject_duplicates, terrane_limit(&max_depth), terrane_limit(&max_bytes))
}

fn terrane_json_canonical(input: String) -> terrane_document_support::DataResult {
    terrane_document_support::canonical_json(&input)
}
