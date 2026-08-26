fn terrane_yaml_parse(input: String, max_depth: terrane_int_support::Int, max_bytes: terrane_int_support::Int, max_aliases: terrane_int_support::Int) -> terrane_document_support::DataResult {
    terrane_document_support::parse_yaml(&input, terrane_limit(&max_depth), terrane_limit(&max_bytes), terrane_limit(&max_aliases))
}
