// Rust justification: ABI boundary to the opaque parser result representation.

fn terrane_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(0)
}
fn terrane_index(value: &terrane_int_support::Int) -> Option<usize> {
    value.as_usize()
}
fn terrane_empty_document() -> terrane_document_support::DataResult {
    terrane_document_support::parse_json("null", 0, 4)
}
fn terrane_make_document_none() -> terrane_document_support::DataResult {
    terrane_document_support::document_none()
}
fn terrane_make_document_bool(value: bool) -> terrane_document_support::DataResult {
    terrane_document_support::document_bool(value)
}
fn terrane_make_document_string(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_string(value)
}
fn terrane_make_document_integer(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_integer(&value)
}
fn terrane_make_document_decimal(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_decimal(&value)
}
fn terrane_make_document_list() -> terrane_document_support::DataResult {
    terrane_document_support::document_list()
}
fn terrane_document_list_append(
    list: &terrane_document_support::DataResult,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_list_append(list, value)
}
fn terrane_make_document_map() -> terrane_document_support::DataResult {
    terrane_document_support::document_map()
}
fn terrane_document_map_insert(
    map: &terrane_document_support::DataResult,
    key: String,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_map_insert(map, key, value)
}


fn terrane_data_failed(result: &terrane_document_support::DataResult) -> bool { result.failed }
fn terrane_data_message(result: &terrane_document_support::DataResult) -> String { result.message.clone() }
fn terrane_data_path(result: &terrane_document_support::DataResult) -> String { result.path.clone() }
fn terrane_data_expected(result: &terrane_document_support::DataResult) -> String { result.expected.clone() }
fn terrane_data_encoded(result: &terrane_document_support::DataResult) -> String { result.encoded.clone() }
fn terrane_document_kind(result: &terrane_document_support::DataResult) -> String { terrane_document_support::document_kind(result) }
fn terrane_document_text(result: &terrane_document_support::DataResult) -> String { terrane_document_support::document_text(result) }
fn terrane_document_coefficient(result: &terrane_document_support::DataResult) -> String {
    terrane_document_support::document_coefficient(result)
}
fn terrane_document_exponent(result: &terrane_document_support::DataResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(terrane_document_support::document_exponent(result))
}
fn terrane_document_length(result: &terrane_document_support::DataResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(i128::try_from(terrane_document_support::document_length(result)).expect("document length fits in i128"))
}
fn terrane_document_item(result: &terrane_document_support::DataResult, index: terrane_int_support::Int) -> terrane_document_support::DataResult {
    terrane_index(&index).map_or_else(
        || terrane_document_support::invalid_document_index(),
        |index| terrane_document_support::document_item(result, index),
    )
}
fn terrane_document_key(result: &terrane_document_support::DataResult, index: terrane_int_support::Int) -> String {
    terrane_index(&index).map_or_else(String::new, |index| terrane_document_support::document_key(result, index))
}
fn terrane_document_field(result: &terrane_document_support::DataResult, key: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_field(result, &key)
}
fn terrane_string_list(value: terrane_collection_support::List<String>) -> Vec<String> {
    value.into_iter().collect()
}

fn terrane_validate_mapping(
    result: &terrane_document_support::DataResult,
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
        result,
        &expected_kind,
        &required_fields,
        &declared_fields,
        &default_fields,
        &default_values,
        allow_unknown,
    )
}
