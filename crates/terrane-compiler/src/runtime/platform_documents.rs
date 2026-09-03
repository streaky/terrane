// Rust justification: ABI boundary to the opaque parser result representation.

pub fn terrane_index(value: &terrane_int_support::Int) -> Option<usize> {
    value.as_usize()
}
pub fn terrane_empty_document() -> terrane_document_support::DataResult {
    terrane_document_support::parse_json("null", 0, 4)
}
pub fn terrane_make_document_none() -> terrane_document_support::DataResult {
    terrane_document_support::document_none()
}
pub fn terrane_make_document_bool(value: bool) -> terrane_document_support::DataResult {
    terrane_document_support::document_bool(value)
}
pub fn terrane_make_document_string(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_string(value)
}
pub fn terrane_make_document_integer(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_integer(&value)
}
pub fn terrane_make_document_decimal(value: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_decimal(&value)
}
pub fn terrane_make_document_list() -> terrane_document_support::DataResult {
    terrane_document_support::document_list()
}
pub fn terrane_document_list_append(
    list: &terrane_document_support::DataResult,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_list_append(list, value)
}
pub fn terrane_make_document_map() -> terrane_document_support::DataResult {
    terrane_document_support::document_map()
}
pub fn terrane_document_map_insert(
    map: &terrane_document_support::DataResult,
    key: String,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_map_insert(map, key, value)
}


pub fn terrane_data_failed(result: &terrane_document_support::DataResult) -> bool { result.failed }
pub fn terrane_data_message(result: &terrane_document_support::DataResult) -> String { result.message.clone() }
pub fn terrane_data_path(result: &terrane_document_support::DataResult) -> String { result.path.clone() }
pub fn terrane_data_expected(result: &terrane_document_support::DataResult) -> String { result.expected.clone() }
pub fn terrane_data_encoded(result: &terrane_document_support::DataResult) -> String { result.encoded.clone() }
pub fn terrane_document_kind(result: &terrane_document_support::DataResult) -> String { terrane_document_support::document_kind(result) }
pub fn terrane_document_text(result: &terrane_document_support::DataResult) -> String { terrane_document_support::document_text(result) }
pub fn terrane_document_coefficient(result: &terrane_document_support::DataResult) -> String {
    terrane_document_support::document_coefficient(result)
}
pub fn terrane_document_exponent(result: &terrane_document_support::DataResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(terrane_document_support::document_exponent(result))
}
pub fn terrane_document_length(result: &terrane_document_support::DataResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(i128::try_from(terrane_document_support::document_length(result)).expect("document length fits in i128"))
}
pub fn terrane_document_item(result: &terrane_document_support::DataResult, index: terrane_int_support::Int) -> terrane_document_support::DataResult {
    terrane_index(&index).map_or_else(
        || terrane_document_support::invalid_document_index(),
        |index| terrane_document_support::document_item(result, index),
    )
}
pub fn terrane_document_key(result: &terrane_document_support::DataResult, index: terrane_int_support::Int) -> String {
    terrane_index(&index).map_or_else(String::new, |index| terrane_document_support::document_key(result, index))
}
pub fn terrane_document_field(result: &terrane_document_support::DataResult, key: String) -> terrane_document_support::DataResult {
    terrane_document_support::document_field(result, &key)
}
pub fn terrane_string_list(value: terrane_collection_support::List<String>) -> Vec<String> {
    value.into_iter().collect()
}

pub fn terrane_validate_mapping(
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
