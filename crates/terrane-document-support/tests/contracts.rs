use terrane_document_support::{
    canonical_json, parse_json, parse_url, parse_yaml, validate_mapping,
};

#[test]
fn json_preserves_exact_numbers_and_rejects_duplicates() {
    let parsed = parse_json("{\"z\":1.2300,\"a\":123456789012345678901234567890}", true, 32, 1024);
    assert!(!parsed.failed, "{}", parsed.message);
    assert_eq!(parsed.encoded, "{\"a\":123456789012345678901234567890,\"z\":1.23}");
    let duplicate = parse_json("{\"key\":1,\"key\":2}", true, 32, 1024);
    assert!(duplicate.failed);
    assert!(duplicate.message.contains("duplicate key"));
}

#[test]
fn canonical_order_is_utf16_and_repeatable() {
    let first = canonical_json("{\"😀\":1,\"a\":2,\"€\":3}");
    let second = canonical_json("{\"€\":3,\"😀\":1,\"a\":2}");
    assert_eq!(first.encoded, second.encoded);
    assert_eq!(first.encoded, "{\"a\":2,\"€\":3,\"😀\":1}");
}

#[test]
fn mapping_accepts_optional_fields_applies_defaults_and_rejects_unknown_fields() {
    let required = vec!["name".to_owned()];
    let declared = vec!["name".to_owned(), "nickname".to_owned(), "active".to_owned()];
    let default_fields = vec!["active".to_owned()];
    let default_values = vec!["true".to_owned()];
    let mapped = validate_mapping(
        "{\"name\":\"Ada\",\"nickname\":\"A\"}",
        "map",
        &required,
        &declared,
        &default_fields,
        &default_values,
        false,
    );
    assert!(!mapped.failed, "{}", mapped.message);
    assert_eq!(mapped.encoded, "{\"active\":true,\"name\":\"Ada\",\"nickname\":\"A\"}");

    let unknown = validate_mapping(
        "{\"name\":\"Ada\",\"extra\":1}",
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
fn yaml_limits_and_safe_tags_are_enforced() {
    let aliases = parse_yaml("root: &root [1]\na: *root\nb: *root", 32, 1024, 1);
    assert!(aliases.failed);
    assert!(aliases.message.contains("alias expansion limit"));
    let tagged = parse_yaml("value: !execute command", 32, 1024, 1);
    assert!(tagged.failed);
    assert!(tagged.message.contains("tags are disabled"));
}

#[test]
fn urls_apply_idna_keep_query_order_and_hide_credentials() {
    let parsed = parse_url("https://user:pass@bücher.example:443/a?x=1&x=2#f", "");
    assert!(!parsed.failed);
    assert_eq!(parsed.host, "xn--bcher-kva.example");
    assert_eq!(parsed.query_entries, [("x".into(), "1".into()), ("x".into(), "2".into())]);
    assert!(!parsed.display.contains("user"));
    assert!(!parsed.display.contains("pass"));
}
