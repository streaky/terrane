// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    let large: f64;
    let small: f64;
    let options: JsonOptions;
    let encoded_large: TerraneDocumentDecodeOutcome<String>;
    let decoded_large: TerraneDocumentDecodeOutcome<f64>;
    let encoded_small: TerraneDocumentDecodeOutcome<String>;
    let decoded_small: TerraneDocumentDecodeOutcome<f64>;
    large = 1.0e300_f64;
    small = f64::from_bits(1);
    options = default_json_options();
    encoded_large = {
        let __terrane_document_source = &large;
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:9:21",
            "float64",
        )
    };
    decoded_large = {
        let __terrane_receiver_value_0 = &encoded_large.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_0;
        let __terrane_document_fallback = &large;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f64,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:10:21",
            "float64",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(encoded_large.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(terrane_string_support::length(&encoded_large
        .value.clone()) as i128) < terrane_int_support::Int::from(32_i128))),
        String::from("|"), terrane_scalar_support::scalar_text(&(decoded_large
        .diagnostics.length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&(decoded_large.value.clone() == large))
    );
    encoded_small = {
        let __terrane_document_source = &small;
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:12:21",
            "float64",
        )
    };
    decoded_small = {
        let __terrane_receiver_value_1 = &encoded_small.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_1;
        let __terrane_document_fallback = &small;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            f64,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:13:21",
            "float64",
            &TerraneDocumentShape {
                tuple_length: None,
                inner: None,
            },
        )
    };
    println!(
        "{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&(encoded_small.diagnostics
        .length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(terrane_string_support::length(&encoded_small
        .value.clone()) as i128) < terrane_int_support::Int::from(32_i128))),
        String::from("|"), terrane_scalar_support::scalar_text(&(decoded_small
        .diagnostics.length() != 0)), String::from("|"),
        terrane_scalar_support::scalar_text(&(decoded_small.value.clone() == small))
    );
}
