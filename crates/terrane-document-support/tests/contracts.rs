use terrane_document_support::{
    canonical_json, parse_json, parse_url, parse_yaml, validate_mapping,
};

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
    assert!(duplicate.message.contains("duplicate key"));
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
        assert_eq!(parsed.expected, "one complete JSON value");
    }
    assert!(!parse_json(" \t{\"a\":1}\n ", 32, 1024).failed);
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
    assert!(overflow.message.contains("exponent"));
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
    assert!(unsafe_depth.message.contains("cannot exceed"));
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
    assert!(aliases.message.contains("alias node limit"));
    let tagged = parse_yaml("value: !execute command", 32, 1024, 1);
    assert!(tagged.failed);
    assert!(tagged.message.contains("tags are disabled"));
    let depth = parse_yaml("a: [[[[]]]]", 2, 1024, 64);
    assert!(depth.failed);
    assert!(depth.message.contains("depth limit"));
    let excessive_depth = parse_yaml("[]", 256, 1024, 64);
    assert!(excessive_depth.failed);
    assert!(excessive_depth.message.contains("cannot exceed 255"));
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
