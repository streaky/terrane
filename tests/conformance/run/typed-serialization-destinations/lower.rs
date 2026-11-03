// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: typed-serialization-destinations
fn main() {
    let options: JsonOptions;
    let scalar: TerraneDocumentDecodeOutcome<String>;
    let numbers: TerraneDocumentDecodeOutcome<
        terrane_collection_support::List<terrane_int_support::Int>,
    >;
    let keys: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Map<String, terrane_int_support::Int>,
    >;
    let pair: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >;
    let wrong: TerraneDocumentDecodeOutcome<
        terrane_collection_support::Tuple<terrane_int_support::Int>,
    >;
    let prototype: Option<String>;
    let nulled: TerraneDocumentDecodeOutcome<Option<String>>;
    let present: TerraneDocumentDecodeOutcome<Option<String>>;
    let invalid: TerraneDocumentDecodeOutcome<Option<String>>;
    let encoded: TerraneDocumentDecodeOutcome<String>;
    let encoded_yaml: TerraneDocumentDecodeOutcome<String>;
    let encoded_none: TerraneDocumentDecodeOutcome<String>;
    options = default_json_options();
    scalar = {
        let __terrane_document_source = &String::from("\"hello\"");
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            String,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || <String>::default(),
            __terrane_document_allow_unknown,
            "case.trn:8:14",
            "string",
            None,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(scalar.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&scalar.value
        .clone())
    );
    numbers = {
        let __terrane_document_source = &String::from("[3,5]");
        let __terrane_document_fallback = &terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(vec![terrane_int_support::Int::from(0_i128)]);
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::List<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:10:15",
            "list of int",
            None,
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(numbers.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_0 = &numbers.value.clone();
        __terrane_raised(__terrane_receiver_value_0
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:11:33-11:49 */)), 0 /* terrane-site: case.trn:11:33-11:49 */) }), String::from("|"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_1 = &numbers
        .value.clone(); __terrane_raised(__terrane_receiver_value_1
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:11:56-11:72 */)), 1 /* terrane-site: case.trn:11:56-11:72 */) })
    );
    keys = {
        let __terrane_document_source = &String::from("left: 9");
        let __terrane_document_fallback = &terrane_collection_support::Map::<
            String,
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, terrane_int_support::Int
                >::new(String::from(""), terrane_int_support::Int::from(0_i128))
            ],
        );
        let __terrane_document_options = &default_yaml_options();
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Map<String, terrane_int_support::Int>,
        >(
            terrane_document_support::parse_yaml_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
                terrane_limit(&__terrane_document_options.max_alias_nodes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:12:12",
            "map of string, int",
            None,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(keys.diagnostics.length() != 0)),
        String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_2 = &keys.value.clone();
        __terrane_raised(__terrane_receiver_value_2.get_or_error(&String::from("left")),
        2 /* terrane-site: case.trn:13:30-13:48 */) })
    );
    pair = {
        let __terrane_document_source = &String::from("[4,6]");
        let __terrane_document_fallback = &terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128)
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:14:12",
            "tuple of int",
            Some(2),
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(pair.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_3 = &pair.value.clone();
        __terrane_raised(__terrane_receiver_value_3
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        3 /* terrane-site: case.trn:15:30-15:43 */)), 3 /* terrane-site: case.trn:15:30-15:43 */) }), String::from("|"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_4 = &pair
        .value.clone(); __terrane_raised(__terrane_receiver_value_4
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:15:50-15:63 */)), 4 /* terrane-site: case.trn:15:50-15:63 */) })
    );
    wrong = {
        let __terrane_document_source = &String::from("[4]");
        let __terrane_document_fallback = &terrane_collection_support::Tuple::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(1_i128),
                terrane_int_support::Int::from(2_i128)
            ],
        );
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            terrane_collection_support::Tuple<terrane_int_support::Int>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:16:13",
            "tuple of int",
            Some(2),
        )
    };
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&(wrong.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&{ let
        __terrane_receiver_value_5 = &wrong.value.clone();
        __terrane_raised(__terrane_receiver_value_5
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:17:31-17:45 */)), 5 /* terrane-site: case.trn:17:31-17:45 */) }), String::from("|"),
        terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_6 = &wrong
        .value.clone(); __terrane_raised(__terrane_receiver_value_6
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        6 /* terrane-site: case.trn:17:52-17:66 */)), 6 /* terrane-site: case.trn:17:52-17:66 */) })
    );
    prototype = Some(String::from("fallback"));
    nulled = {
        let __terrane_document_source = &String::from("null");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:19:14",
            "string|none",
            None,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(nulled.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &nulled
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    present = {
        let __terrane_document_source = &String::from("\"present\"");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:21:15",
            "string|none",
            None,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(present.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &present
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    invalid = {
        let __terrane_document_source = &String::from("false");
        let __terrane_document_fallback = &prototype;
        let __terrane_document_options = &options;
        let __terrane_document_allow_unknown = false;
        __terrane_decode_document::<
            Option<String>,
        >(
            terrane_document_support::parse_json_document(
                __terrane_document_source,
                terrane_limit(&__terrane_document_options.max_depth),
                terrane_limit(&__terrane_document_options.max_bytes),
            ),
            || (*__terrane_document_fallback).clone(),
            __terrane_document_allow_unknown,
            "case.trn:23:15",
            "string|none",
            None,
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(invalid.diagnostics.length() !=
        0)), String::from("|"), { let mut __terrane_text = String::new(); match &invalid
        .value.clone() { Some(__terrane_present) => {
        terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
        __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text }
    );
    encoded = {
        let __terrane_document_source = &terrane_collection_support::List::<
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_int_support::Int::from(3_i128),
                terrane_int_support::Int::from(5_i128)
            ],
        );
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:25:15",
            "list of int",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded.diagnostics.length() !=
        0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded.value
        .clone())
    );
    encoded_yaml = {
        let __terrane_document_source = &terrane_collection_support::Map::<
            String,
            terrane_int_support::Int,
        >::new(
            vec![
                terrane_collection_support::Entry::< String, terrane_int_support::Int
                >::new(String::from("left"), terrane_int_support::Int::from(9_i128))
            ],
        );
        let __terrane_document_options = &default_yaml_options();
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:27:20",
            "map of string, int",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded_yaml.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded_yaml
        .value.clone())
    );
    encoded_none = {
        let __terrane_receiver_value_7 = &nulled.value.clone();
        let __terrane_document_source = &__terrane_receiver_value_7;
        let __terrane_document_options = &options;
        __terrane_encode_document(
            __terrane_document_source,
            terrane_limit(&__terrane_document_options.max_depth),
            terrane_limit(&__terrane_document_options.max_bytes),
            "case.trn:29:20",
            "string|none",
        )
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(encoded_none.diagnostics.length()
        != 0)), String::from("|"), terrane_scalar_support::scalar_text(&encoded_none
        .value.clone())
    );
}
