use terrane_document_support::{
    canonical_json, parse_json, parse_json_document, parse_url, parse_yaml, validate_mapping,
};

#[test]
fn json_depth_counts_scalar_children_and_containers_consistently() {
    for input in ["0", "[]", "{}"] {
        assert!(!parse_json(input, 0, 1024).failed, "{input}");
    }
    for input in ["[0]", "[null]", "[true]", "[\"x\"]", "{\"x\":0}", "[[]]"] {
        assert!(parse_json(input, 0, 1024).failed, "{input}");
        assert!(!parse_json(input, 1, 1024).failed, "{input}");
    }
    assert!(parse_json("{\"x\":[0]}", 1, 1024).failed);
    assert!(!parse_json("{\"x\":[0]}", 2, 1024).failed);
}

#[test]
fn raw_json_keeps_marker_like_objects_and_exact_numbers() {
    let cases = [
        (
            r#"{"$serde_json::private::Number":"7","sibling":2}"#,
            r#"{"$serde_json::private::Number":"7","sibling":2}"#,
        ),
        (
            r#"{"nested":{"$serde_json::private::Number":7,"child":[1.2500,9007199254740993]},"tail":true}"#,
            r#"{"nested":{"$serde_json::private::Number":7,"child":[1.25,9007199254740993]},"tail":true}"#,
        ),
        (
            r#"{"$serde_json::private::Number":null}"#,
            r#"{"$serde_json::private::Number":null}"#,
        ),
    ];
    for (input, expected) in cases {
        let parsed = parse_json(input, 32, 1024);
        assert!(!parsed.failed, "{}: {}", input, parsed.message);
        assert_eq!(parsed.encoded, expected);
    }
    let duplicate = parse_json(
        r#"{"nested":{"$serde_json::private::Number":1,"$serde_json::private::Number":2}}"#,
        32,
        1024,
    );
    assert!(duplicate.failed);
}

#[test]
fn bounded_json_writer_reports_output_depth_and_size() {
    use terrane_document_support::{Document, JsonWriter, write_json_bounded};
    let value = Document::List(vec![
        Document::String("a\nb".into()),
        Document::Map(std::collections::BTreeMap::from([(
            "n".into(),
            Document::Integer("12345678901234567890".into()),
        )])),
    ]);
    let encoded = write_json_bounded(&value, 8, 128).unwrap();
    assert_eq!(encoded, "[\"a\\nb\",{\"n\":12345678901234567890}]");
    assert!(write_json_bounded(&value, 1, 128).is_err());
    assert!(write_json_bounded(&value, 8, 8).is_err());

    let mut writer = JsonWriter::new(2, 32).unwrap();
    writer.begin_object().unwrap();
    writer.key("k").unwrap();
    writer.string("v").unwrap();
    writer.end_object().unwrap();
    assert_eq!(writer.finish().unwrap(), r#"{"k":"v"}"#);

    let mut failed = JsonWriter::new(1, 1).unwrap();
    assert!(failed.string("too large").is_err());
    assert!(failed.finish().is_err());

    assert_eq!(
        write_json_bounded(&Document::List(Vec::new()), 0, 2).unwrap(),
        "[]"
    );
    assert!(write_json_bounded(&Document::List(vec![Document::List(Vec::new())]), 0, 8,).is_err());
}

#[test]
fn json_preserves_exact_numbers_and_rejects_duplicates() {
    let parsed = parse_json(
        "{\"z\":1.2300,\"a\":123456789012345678901234567890}",
        32,
        1024,
    );
    assert!(!parsed.failed, "{}", parsed.message);
    assert_eq!(
        parsed.encoded,
        "{\"a\":1.2345678901234567890123456789e+29,\"z\":1.23}"
    );
    let duplicate = parse_json("{\"key\":1,\"key\":2}", 32, 1024);
    assert!(duplicate.failed);
}

#[test]
fn json_rejects_trailing_content_after_a_complete_value() {
    for input in [
        "{\"a\":1} garbage",
        "{\"a\":1}{\"b\":2}",
        "1 2",
        "null null",
    ] {
        let parsed = parse_json(input, 32, 1024);
        assert!(parsed.failed, "{input} parsed as {}", parsed.encoded);
    }
    assert!(!parse_json(" \t{\"a\":1}\n ", 32, 1024).failed);
}

#[test]
fn json_budgets_beyond_sixteen_mib_and_strict_invalid_syntax() {
    let input = format!("\"{}\"", "x".repeat(16 * 1024 * 1024 + 1));
    let parsed = parse_json(&input, 8, input.len());
    assert!(!parsed.failed, "{}", parsed.message);
    assert_eq!(parsed.encoded, input);
    let list = parse_json("[]", 8, 16);
    let appended = terrane_document_support::document_list_append(&list, &parsed);
    assert!(!appended.failed, "{}", appended.message);
    assert_eq!(
        appended
            .encoded
            .strip_prefix('[')
            .and_then(|text| text.strip_suffix(']')),
        Some(input.as_str()),
    );
    let document = parse_json_document(&input, 8, input.len()).unwrap();
    let native = serde_json::to_string(&terrane_document_support::JsonDocument(&document)).unwrap();
    assert_eq!(native, input);

    assert!(!parse_json("\"\\ud800\\udc00\"", 8, 1024).failed);
    for input in [
        "\u{feff}1",
        "\"\\ud800\"",
        "\"\\udfff\"",
        "\"\\ud800\\u0041\"",
        "01",
        "[01]",
    ] {
        let parsed = parse_json(input, 8, 1024);
        assert!(parsed.failed, "{input}");
    }
}

#[test]
fn custom_document_writer_uses_utf16_order_and_depth_budget() {
    use terrane_document_support::{Document, JsonWriter};
    let value = Document::Map(std::collections::BTreeMap::from([
        ("\u{e000}".into(), Document::None),
        ("\u{10000}".into(), Document::None),
    ]));
    let mut writer = JsonWriter::new(1, 128).unwrap();
    writer.document(&value).unwrap();
    assert_eq!(writer.finish().unwrap(), "{\"𐀀\":null,\"\":null}");

    let nested = Document::List(vec![Document::List(vec![Document::None])]);
    let mut writer = JsonWriter::new(1, 128).unwrap();
    assert!(writer.document(&nested).is_err());
}

#[test]
fn native_float_encoding_is_compact_and_round_trips_extremes() {
    for value in [1.0e300_f64, f64::from_bits(1)] {
        let result = terrane_document_support::serialize_document(&value);
        assert!(!result.failed, "{}", result.message);
        assert!(result.encoded.len() < 32, "{}", result.encoded);
        let decoded: f64 = serde_json::from_str(&result.encoded).unwrap();
        assert_eq!(decoded, value);
    }
    for value in [1.0e30_f32, f32::from_bits(1)] {
        let result = terrane_document_support::serialize_document(&value);
        assert!(!result.failed, "{}", result.message);
        assert!(result.encoded.len() < 32, "{}", result.encoded);
        let decoded: f32 = serde_json::from_str(&result.encoded).unwrap();
        assert_eq!(decoded, value);
    }
}

#[test]
fn canonical_numbers_are_valid_and_value_equivalent() {
    let first = parse_json("{\"😀\":1,\"a\":1.0e-1,\"€\":1e2,\"zero\":-0}", 32, 1024);
    let second = parse_json("{\"€\":100,\"😀\":1,\"a\":0.1,\"zero\":0}", 32, 1024);
    assert_eq!(
        canonical_json(&first).encoded,
        canonical_json(&second).encoded
    );
    assert_eq!(first.encoded, "{\"a\":0.1,\"zero\":0,\"€\":100,\"😀\":1}");
    assert!(!parse_json(&first.encoded, 32, 1024).failed);
    let overflow = parse_json("{\"n\":1e999999999999999999999}", 32, 1024);
    assert!(overflow.failed);
    let integer = parse_json("123456789012345678901234567890", 32, 1024);
    assert_eq!(terrane_document_support::document_kind(&integer), "integer");
    let reparsed = parse_json(&integer.encoded, 32, 1024);
    assert_eq!(
        terrane_document_support::document_kind(&reparsed),
        "integer"
    );
    let integral_exponent = parse_json("1e2", 32, 1024);
    assert_eq!(
        terrane_document_support::document_kind(&integral_exponent),
        "integer"
    );
    let unsafe_depth = parse_json("[]", 513, 1024);
    assert!(unsafe_depth.failed);
}

#[test]
fn mapping_accepts_optional_fields_applies_defaults_and_rejects_unknown_fields() {
    let required = vec!["name".to_owned()];
    let declared = vec![
        "name".to_owned(),
        "nickname".to_owned(),
        "active".to_owned(),
    ];
    let default_fields = vec!["active".to_owned()];
    let default_values = vec!["true".to_owned()];
    let input = parse_json("{\"name\":\"Ada\",\"nickname\":\"A\"}", 32, 1024);
    let mapped = validate_mapping(
        &input,
        "map",
        &required,
        &declared,
        &default_fields,
        &default_values,
        false,
    );
    assert!(!mapped.failed, "{}", mapped.message);
    assert_eq!(
        mapped.encoded,
        "{\"active\":true,\"name\":\"Ada\",\"nickname\":\"A\"}"
    );

    let input = parse_json("{\"name\":\"Ada\",\"extra\":1}", 32, 1024);
    let unknown = validate_mapping(
        &input,
        "map",
        &required,
        &declared,
        &default_fields,
        &default_values,
        false,
    );
    assert!(unknown.failed);
    assert_eq!(unknown.path, "$.extra");
}

#[test]
fn yaml_preserves_exact_scalars_and_enforces_limits_before_expansion() {
    let exact = parse_yaml(
        "integer: 123456789012345678901234567890\ndecimal: 3.141592653589793238462643383279",
        32,
        1024,
        64,
    );
    assert!(!exact.failed, "{}", exact.message);
    assert_eq!(
        exact.encoded,
        "{\"decimal\":3.141592653589793238462643383279,\"integer\":1.2345678901234567890123456789e+29}"
    );
    let ordinary = parse_yaml("glob: \"a * b * c\"", 32, 1024, 0);
    assert!(!ordinary.failed, "{}", ordinary.message);
    let aliases = parse_yaml(
        "leaf: &leaf [1, 2, 3, 4]\na: &a [*leaf, *leaf, *leaf, *leaf]\nb: [*a, *a, *a, *a]",
        32,
        1024,
        20,
    );
    assert!(aliases.failed);
    let tagged = parse_yaml("value: !execute command", 32, 1024, 1);
    assert!(tagged.failed);
    let depth = parse_yaml("a: [[[[]]]]", 2, 1024, 64);
    assert!(depth.failed);
    let excessive_depth = parse_yaml("[]", 256, 1024, 64);
    assert!(excessive_depth.failed);
    let deeply_nested = format!("{}null{}", "[".repeat(300), "]".repeat(300));
    let masked_depth = parse_yaml(&deeply_nested, 100, 1024, 64);
    assert!(masked_depth.failed);
    assert_eq!(masked_depth.message, "document depth limit exceeded");
    let ordinary_reuse = parse_yaml(
        "shared: &s {a: 1, b: 2, c: 3, d: 4, e: 5}\nx: *s\ny: *s\nz: *s\nq: *s\nr: *s\ns: *s\nt: *s\nu: *s\nv: *s\nw: *s\nxx: *s\nyy: *s",
        32,
        4096,
        65_536,
    );
    assert!(!ordinary_reuse.failed, "{}", ordinary_reuse.message);
}

#[test]
fn urls_apply_idna_keep_query_order_and_hide_credentials() {
    let parsed = parse_url("https://user:pass@bücher.example:443/a?x=1&x=2#f", "");
    assert!(!parsed.failed);
    assert_eq!(parsed.host, "xn--bcher-kva.example");
    assert_eq!(
        parsed.query_entries,
        [("x".into(), "1".into()), ("x".into(), "2".into())]
    );
    assert!(!parsed.display.contains("user"));
    assert!(!parsed.display.contains("pass"));
}

#[test]
fn numeric_writer_rejects_other_json_shapes() {
    for value in [
        "[0]", "{}", "null", "true", "\"12\"", "1 2", "01", "1.", " 1", "1 ", "-0 ",
    ] {
        let mut integer = terrane_document_support::JsonWriter::new(0, 64).unwrap();
        assert!(integer.integer(value).is_err(), "{value}");
        let mut decimal = terrane_document_support::JsonWriter::new(0, 64).unwrap();
        assert!(decimal.decimal(value).is_err(), "{value}");
    }
}

#[test]
fn normalized_exponents_remain_within_writable_bounds() {
    for value in ["10e100000", "0.1e-100000"] {
        assert!(parse_json_document(value, 0, 64).is_err(), "{value}");
    }
    for value in ["1e100000", "1e-100000", "10e99999", "0.1e-99999"] {
        let document = parse_json_document(value, 0, 64).unwrap();
        let encoded = terrane_document_support::write_json_bounded(&document, 0, 64).unwrap();
        assert_eq!(parse_json_document(&encoded, 0, 64).unwrap(), document);
    }
}
