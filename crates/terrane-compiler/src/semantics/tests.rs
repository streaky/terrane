#[cfg(test)]
mod name_style_tests {
    use super::super::prelude::*;

    #[test]
    fn compiler_owned_declarations_require_kebab_case() {
        let package = Package::implicit(
            "main.trn",
            "namespace app\nfunction NotKebab;\n  return\nfunction main;\n  return\n".to_owned(),
        );
        let mut semantic = analyze(&package).unwrap();
        semantic.units[0].bundled = true;

        let failure = validate_compiler_owned_names(&semantic.units).unwrap_err();
        assert_eq!(failure.diagnostics[0].code, "S2018");
        assert_eq!(
            failure.diagnostics[0].message,
            "compiler-owned declaration `NotKebab` is not kebab-case"
        );
    }

    #[test]
    fn authored_name_style_is_an_opt_in_warning() {
        let package = Package::implicit(
            "main.trn",
            "namespace app\nfunction main;\n  Answer = 42\n  print; Answer\n".to_owned(),
        );
        let semantic = analyze(&package).unwrap();

        assert!(warnings(&semantic, false).is_empty());
        let diagnostics = warnings(&semantic, true);
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "S2018"
                && diagnostic.message == "declared name `Answer` is not kebab-case"
                && diagnostic.severity == crate::Severity::Warning
        }));
    }

    #[test]
    fn object_union_arm_identity_follows_import_aliases() {
        let package = Package::implicit(
            "main.trn",
            concat!(
                "namespace app\n",
                "from /core/errors import throwable as first, throwable as second\n",
                "function main;\n",
                "  return\n",
            )
            .to_owned(),
        );
        let semantic = analyze(&package).unwrap();
        let unit = &semantic.units[0];
        let first = unit.source.text().find("first").unwrap();
        let second = unit.source.text().find("second").unwrap();

        assert_eq!(
            union_arm_identity(&semantic, unit, "first", first),
            union_arm_identity(&semantic, unit, "second", second),
        );
    }

    #[test]
    fn arbitrary_object_optional_types_are_semantic_values() {
        let package = Package::implicit(
            "main.trn",
            "namespace app\nclass widget\nfunction maybe widget|none;\n  return none\nfunction main;\n  value widget|none = maybe;\n  return\n".to_owned(),
        );
        let semantic = analyze(&package).unwrap();
        let maybe = semantic.units[0]
            .functions
            .iter()
            .find(|function| function.name == "maybe")
            .unwrap();

        assert!(matches!(
            &maybe.return_type,
            Some(ValueType::Optional(inner))
                if matches!(inner.as_ref(), ValueType::Object(identity) if identity.name == "widget")
        ));
    }
}
