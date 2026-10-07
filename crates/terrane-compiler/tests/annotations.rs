use terrane_compiler::{CompileTimeValue, DeclarationInterface, Package, analyze};

const SOURCE: &str =
    include_str!("../../../tests/conformance/run/declaration-annotations/case.trn");

fn metadata(text: &str) -> terrane_compiler::SemanticPackage {
    analyze(&Package::implicit("case.trn", text.to_owned())).expect("annotation source analyzes")
}

fn declaration<'a>(
    semantic: &'a terrane_compiler::SemanticPackage,
    suffix: &str,
) -> &'a terrane_compiler::DeclarationMetadata {
    semantic
        .declarations()
        .find(|record| record.identity == format!("/declaration-annotations::{suffix}"))
        .unwrap()
}

#[test]
fn annotations_resolve_typed_data_in_order_without_runtime_embedding() {
    let semantic = metadata(SOURCE);
    let parent = declaration(&semantic, "parent");
    assert_eq!(
        parent.documentation.as_deref(),
        Some("Parent documentation.\n\n    An indented example.")
    );
    assert_eq!(parent.annotations.len(), 3);
    assert_eq!(
        parent.annotations[0].payload,
        CompileTimeValue::Map(vec![(
            CompileTimeValue::String("value".into()),
            CompileTimeValue::String("shared".into())
        )])
    );
    assert_eq!(
        parent.annotations[1].payload,
        CompileTimeValue::Map(vec![(
            CompileTimeValue::String("value".into()),
            CompileTimeValue::String("second".into())
        )])
    );
    let CompileTimeValue::Map(data) = &parent.annotations[2].payload else {
        panic!("typed payload")
    };
    assert!(data.contains(&(
        CompileTimeValue::String("keys".into()),
        CompileTimeValue::Set(vec![
            CompileTimeValue::Integer("1".into()),
            CompileTimeValue::Integer("2".into())
        ])
    )));
    assert!(data.contains(&(
        CompileTimeValue::String("target".into()),
        CompileTimeValue::Descriptor("/core/types::int".into())
    )));
    assert!(data.contains(&(
        CompileTimeValue::String("pair".into()),
        CompileTimeValue::Tuple(vec![
            CompileTimeValue::Integer("5".into()),
            CompileTimeValue::Integer("6".into())
        ])
    )));
    assert!(data.contains(&(
        CompileTimeValue::String("scale".into()),
        CompileTimeValue::Float("0.5".into())
    )));
    assert!(data.contains(&(
        CompileTimeValue::String("enabled".into()),
        CompileTimeValue::Boolean(true)
    )));
    assert!(data.contains(&(
        CompileTimeValue::String("raw".into()),
        CompileTimeValue::Bytes(vec![65, 66])
    )));
    for (name, identity) in [
        ("callable-target", "/declaration-annotations::helper"),
        ("none-target", "/core/types::none"),
    ] {
        let CompileTimeValue::Map(payload) = &declaration(&semantic, name).annotations[0].payload
        else {
            panic!("descriptor payload")
        };
        assert!(payload.contains(&(
            CompileTimeValue::String("target".into()),
            CompileTimeValue::Descriptor(identity.into())
        )));
    }
    assert_eq!(
        declaration(&semantic, "helper::value")
            .documentation
            .as_deref(),
        Some("Parameter documentation.")
    );
    assert_eq!(declaration(&semantic, "helper::value").annotations.len(), 1);
    assert_eq!(declaration(&semantic, "parent::field").annotations.len(), 1);
    let compilation = terrane_compiler::compile("case.trn", SOURCE.to_owned()).unwrap();
    assert!(
        !compilation
            .rust
            .contains("UNIQUE_COMPILE_TIME_ANNOTATION_PAYLOAD")
    );
}

#[test]
fn inherited_fields_keep_origins_and_overrides_do_not_inherit_annotations() {
    let semantic = metadata(SOURCE);
    let base = declaration(&semantic, "parent");
    let child = declaration(&semantic, "child");
    assert!(child.annotations.is_empty());
    let base_field = base
        .fields
        .iter()
        .find(|field| field["name"] == "field")
        .unwrap();
    let inherited = child
        .fields
        .iter()
        .find(|field| field["name"] == "field")
        .unwrap();
    assert_eq!(inherited["origin"], base_field["origin"]);
    assert_eq!(
        declaration(&semantic, "parent::inherited")
            .annotations
            .len(),
        1
    );
    assert!(
        declaration(&semantic, "override-child::inherited")
            .annotations
            .is_empty()
    );
    assert!(
        !semantic
            .declarations()
            .any(|record| record.identity.ends_with("::child::field"))
    );
}

#[test]
fn interface_edits_invalidate_fingerprints_without_changing_nominal_contracts() {
    let first = metadata(SOURCE);
    for edited in [
        SOURCE.replace(
            "UNIQUE_COMPILE_TIME_ANNOTATION_PAYLOAD",
            "EDITED_COMPILE_TIME_ANNOTATION_PAYLOAD",
        ),
        SOURCE.replace("Parent documentation.", "Edited documentation."),
    ] {
        let second = metadata(&edited);
        assert_eq!(
            first
                .units
                .iter()
                .flat_map(|unit| &unit.descriptors)
                .find(|value| value.name == "parent")
                .unwrap()
                .identity,
            second
                .units
                .iter()
                .flat_map(|unit| &unit.descriptors)
                .find(|value| value.name == "parent")
                .unwrap()
                .identity
        );
        assert_ne!(
            first.declaration_interface().fingerprint,
            second.declaration_interface().fingerprint
        );
    }
}

#[test]
fn source_independent_export_preserves_public_data_and_redacts_private_defaults() {
    let interface = metadata(SOURCE).declaration_interface();
    let json = interface.to_json().unwrap();
    let restored = DeclarationInterface::from_json(&json).unwrap();
    assert_eq!(restored, interface);
    assert!(!restored.declarations.iter().any(
        |record| record.identity.ends_with("::secret") || record.identity.contains("::hidden")
    ));
    let parent = restored
        .declarations
        .iter()
        .find(|record| record.identity.ends_with("::parent"))
        .unwrap();
    assert_eq!(parent.annotations.len(), 3);
    assert!(!parent.fields.iter().any(|field| field["name"] == "secret"));
    let credential = parent
        .fields
        .iter()
        .find(|field| field["name"] == "credential")
        .unwrap();
    assert!(credential["default"].is_null());
    assert!(!json.contains("private initializer"));
    assert!(!json.contains("secret initializer"));
    let mut tampered: serde_json::Value = serde_json::from_str(&json).unwrap();
    tampered["declarations"][0]["documentation"] = serde_json::json!("changed after signing");
    assert!(DeclarationInterface::from_json(&tampered.to_string()).is_err());
}

#[test]
fn nullable_schema_defaults_and_explicit_empty_prose_are_distinct() {
    let source = "namespace nullable-metadata\nfrom /core/annotations import annotation, annotation-target\nfrom /core/collections import list\n@[annotation; targets = (list; (instance annotation-target::class;))]\nclass prose\n    value string|none = none\n@[prose;]\nclass fallback\n@[prose; '']\nclass explicit\n";
    let semantic = analyze(&Package::implicit("case.trn", source.to_owned())).unwrap();
    let payloads = semantic
        .declarations()
        .filter(|record| {
            record.identity.ends_with("::fallback") || record.identity.ends_with("::explicit")
        })
        .map(|record| &record.annotations[0].payload)
        .collect::<Vec<_>>();
    assert_eq!(
        payloads,
        vec![
            &CompileTimeValue::Map(vec![(
                CompileTimeValue::String("value".into()),
                CompileTimeValue::None
            )]),
            &CompileTimeValue::Map(vec![(
                CompileTimeValue::String("value".into()),
                CompileTimeValue::String("".into())
            )])
        ]
    );
}
