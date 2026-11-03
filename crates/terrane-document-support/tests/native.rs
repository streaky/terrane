use serde::Deserialize;
use std::collections::BTreeMap;
use terrane_document_support::{
    Document, JsonDocument, document_to_json_value, from_document, json_value_to_document,
    parse_json_document, serialize_document, to_document,
};

#[test]
fn exact_json_adapter_and_direct_native_conversion_preserve_numeric_contracts() {
    let value = Document::Map(BTreeMap::from([
        (
            "decimal".into(),
            Document::Decimal("1.2500000000000000001".into()),
        ),
        (
            "integer".into(),
            Document::Integer("123456789012345678901234567890".into()),
        ),
    ]));
    assert_eq!(
        serde_json::to_string(&JsonDocument(&value)).unwrap(),
        r#"{"decimal":1.2500000000000000001,"integer":123456789012345678901234567890}"#
    );
    let native =
        serde_json::json!({"$serde_json::private::Number":"7", "n":9_007_199_254_740_993_u64});
    let document = json_value_to_document(&native).unwrap();
    assert_eq!(document_to_json_value(&document).unwrap(), native);
    assert!(document_to_json_value(&Document::Decimal("1e100001".into())).is_err());
    assert!(
        from_document::<u128>(&Document::Integer(
            "340282366920938463463374607431768211456".into()
        ))
        .is_err()
    );
}

#[derive(Debug, Deserialize, PartialEq)]
struct Payload<'a> {
    label: &'a str,
    value: f64,
    count: u64,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", content = "payload")]
enum Tagged {
    Value { timestamp: f64 },
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum Untagged {
    Number(f64),
    Record { timestamp: f64 },
}

#[test]
fn borrowed_document_decodes_native_records_tagged_and_untagged_fractional_values() {
    let document = parse_json_document(
        r#"{"label":"kept","value":123.456,"count":9007199254740993}"#,
        8,
        1024,
    )
    .unwrap();
    let decoded: Payload<'_> = from_document(&document).unwrap();
    assert_eq!(
        decoded,
        Payload {
            label: "kept",
            value: 123.456,
            count: 9_007_199_254_740_993
        }
    );
    let Document::Map(fields) = &document else {
        panic!("expected map");
    };
    let Document::String(label) = &fields["label"] else {
        panic!("expected string");
    };
    assert_eq!(decoded.label.as_ptr(), label.as_ptr());
    let tagged = parse_json_document(
        r#"{"kind":"Value","payload":{"timestamp":123.456}}"#,
        8,
        1024,
    )
    .unwrap();
    assert_eq!(
        from_document::<Tagged>(&tagged).unwrap(),
        Tagged::Value { timestamp: 123.456 }
    );
    let untagged = parse_json_document(r#"{"timestamp":123.456}"#, 8, 1024).unwrap();
    assert_eq!(
        from_document::<Untagged>(&untagged).unwrap(),
        Untagged::Record { timestamp: 123.456 }
    );
    assert_eq!(
        from_document::<Untagged>(&Document::Decimal("123.456".into())).unwrap(),
        Untagged::Number(123.456)
    );
}

#[test]
fn native_float_targets_round_directly_and_integer_expansion_is_bounded() {
    let midpoint = Document::Decimal("1.000000059604644775390625000000000001".into());
    assert_eq!(
        from_document::<f32>(&midpoint).unwrap().to_bits(),
        1.0f32.to_bits() + 1
    );
    assert_eq!(
        from_document::<f32>(&Document::Decimal("1e-1000".into())).unwrap(),
        0.0
    );
    assert!(from_document::<f32>(&Document::Integer("1e1000".into())).is_err());
    assert!(from_document::<u8>(&Document::Integer("256".into())).is_err());
    assert!(from_document::<i64>(&Document::Decimal("1.25".into())).is_err());
    assert!(from_document::<u128>(&Document::Integer("1e100000".into())).is_err());
    assert_eq!(
        from_document::<u64>(&Document::Integer("1e+3".into())).unwrap(),
        1000
    );
}

#[derive(serde::Serialize)]
struct NativeRecord<'a> {
    name: &'a str,
    values: (i128, f32),
    optional: Option<bool>,
}

#[derive(serde::Serialize)]
enum NativeEnum {
    Unit,
    Tuple(i32, &'static str),
    Record { enabled: bool },
}

#[test]
fn direct_native_encoder_covers_serde_shapes_and_exact_numbers() {
    #[derive(Deserialize)]
    struct Decoded {
        name: String,
        values: (i128, f32),
        optional: Option<bool>,
    }
    let input = NativeRecord {
        name: "native",
        values: (i128::MAX, f32::from_bits(0x3f80_0001)),
        optional: None,
    };
    let document = to_document(&input).unwrap();
    let decoded: Decoded = from_document(&document).unwrap();
    assert_eq!(decoded.name, "native");
    assert_eq!(decoded.values.0, i128::MAX);
    assert_eq!(decoded.values.1.to_bits(), 0x3f80_0001);
    assert_eq!(decoded.optional, None);
    assert_eq!(
        to_document(&NativeEnum::Unit).unwrap(),
        Document::String("Unit".into())
    );
    assert_eq!(
        to_document(&NativeEnum::Tuple(4, "x")).unwrap(),
        Document::Map(BTreeMap::from([(
            "Tuple".into(),
            Document::List(vec![
                Document::Integer("4".into()),
                Document::String("x".into())
            ])
        )]))
    );
    assert_eq!(
        to_document(&NativeEnum::Record { enabled: true }).unwrap(),
        Document::Map(BTreeMap::from([(
            "Record".into(),
            Document::Map(BTreeMap::from([("enabled".into(), Document::Bool(true))]))
        )]))
    );
    assert!(serialize_document(&f64::NAN).failed);
}

#[test]
fn direct_native_encoder_rejects_bad_maps_and_nonfinite_numbers() {
    struct Duplicate;
    impl serde::Serialize for Duplicate {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            use serde::ser::SerializeMap;
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("same", &1)?;
            map.serialize_entry("same", &2)?;
            map.end()
        }
    }
    assert!(
        to_document(&Duplicate)
            .unwrap_err()
            .to_string()
            .contains("duplicate")
    );
    assert!(to_document(&f32::INFINITY).is_err());
}

#[test]
fn direct_native_encoder_rejects_non_string_keys_and_bounded_output() {
    let numeric_keys = BTreeMap::from([(1u8, "value")]);
    assert!(to_document(&numeric_keys).is_err());
    let oversized = "x".repeat(16_777_216);
    assert!(to_document(&oversized).is_err());
}

#[test]
fn native_sequences_require_complete_consumption() {
    #[derive(Deserialize)]
    enum Tagged {
        Pair(u8, u8),
    }
    let document = parse_json_document("[1,2,3]", 8, 64).unwrap();
    assert!(from_document::<(u8, u8)>(&document).is_err());
    assert!(from_document::<[u8; 2]>(&document).is_err());
    assert_eq!(from_document::<[u8; 3]>(&document).unwrap(), [1, 2, 3]);
    let tagged = parse_json_document(r#"{"Pair":[1,2,3]}"#, 8, 64).unwrap();
    assert!(from_document::<Tagged>(&tagged).is_err());
    let valid = parse_json_document(r#"{"Pair":[1,2]}"#, 8, 64).unwrap();
    let Tagged::Pair(first, second) = from_document(&valid).unwrap();
    assert_eq!((first, second), (1, 2));
}

#[test]
fn native_map_keys_support_borrowing_and_string_newtypes() {
    #[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    struct Key<'a>(#[serde(borrow)] &'a str);
    let document = parse_json_document(r#"{"alpha":7}"#, 8, 64).unwrap();
    assert_eq!(
        from_document::<BTreeMap<&str, u8>>(&document).unwrap(),
        BTreeMap::from([("alpha", 7)])
    );
    assert_eq!(
        from_document::<BTreeMap<Key<'_>, u8>>(&document).unwrap(),
        BTreeMap::from([(Key("alpha"), 7)])
    );
}

#[test]
fn native_json_conversions_enforce_depth() {
    let mut document = Document::None;
    for depth in 0..512 {
        document = if depth % 2 == 0 {
            Document::List(vec![document])
        } else {
            Document::Map(BTreeMap::from([("child".into(), document)]))
        };
    }
    let native = document_to_json_value(&document).unwrap();
    assert_eq!(json_value_to_document(&native).unwrap(), document);
    assert!(document_to_json_value(&Document::List(vec![document])).is_err());
    assert!(json_value_to_document(&serde_json::Value::Array(vec![native])).is_err());
}
