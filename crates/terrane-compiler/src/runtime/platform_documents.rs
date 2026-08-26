fn terrane_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(usize::MAX)
}

fn terrane_data_failed(result: &terrane_document_support::DataResult) -> bool { result.failed }
fn terrane_data_message(result: &terrane_document_support::DataResult) -> String { result.message.clone() }
fn terrane_data_path(result: &terrane_document_support::DataResult) -> String { result.path.clone() }
fn terrane_data_expected(result: &terrane_document_support::DataResult) -> String { result.expected.clone() }
fn terrane_data_encoded(result: &terrane_document_support::DataResult) -> String { result.encoded.clone() }
fn terrane_document_kind(encoded: &String) -> String { terrane_document_support::document_kind(encoded) }
fn terrane_document_text(encoded: &String) -> String { terrane_document_support::document_text(encoded) }
fn terrane_document_length(encoded: &String) -> terrane_int_support::Int {
    terrane_int_support::Int::from(i128::try_from(terrane_document_support::document_length(encoded)).expect("document length fits in i128"))
}
fn terrane_document_item(encoded: &String, index: terrane_int_support::Int) -> terrane_document_support::DataResult {
    terrane_document_support::document_item(encoded, terrane_limit(&index))
}
fn terrane_document_key(encoded: &String, index: terrane_int_support::Int) -> String {
    terrane_document_support::document_key(encoded, terrane_limit(&index))
}
fn terrane_document_field(encoded: &String, key: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_field(encoded, &key)
}
fn terrane_string_list(value: terrane_collection_support::List<String>) -> Vec<String> {
    (0..usize::try_from(value.length()).expect("list length fits in usize"))
        .filter_map(|index| value.get(index).cloned())
        .collect()
}

fn terrane_validate_mapping(
    encoded: &String,
    expected_kind: String,
    required_fields: terrane_collection_support::List<String>,
    declared_fields: terrane_collection_support::List<String>,
    default_fields: terrane_collection_support::List<String>,
    default_values: terrane_collection_support::List<String>,
    allow_unknown: bool,
) -> terrane_document_support::DataResult {
    let required_fields = terrane_string_list(required_fields);
    let declared_fields = terrane_string_list(declared_fields);
    let default_fields = terrane_string_list(default_fields);
    let default_values = terrane_string_list(default_values);
    terrane_document_support::validate_mapping(
        encoded,
        &expected_kind,
        &required_fields,
        &declared_fields,
        &default_fields,
        &default_values,
        allow_unknown,
    )
}
