use terrane_compiler::{Package, analyze};

fn rejects(source: &str, code: &str) {
    let failure = analyze(&Package::implicit("case.trn", source.to_owned()))
        .expect_err("invalid modifier contract must not be admitted");
    assert!(
        failure
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == code),
        "{:#?}",
        failure.diagnostics
    );
}

#[test]
fn modifier_name_alone_cannot_grant_structural_authority() {
    rejects(
        "namespace modifier-boundary\nclass custom-scope\nfunction main;\n    with custom-scope guard = (1) (\n        print; guard\n    )\n",
        "S2120",
    );
}

#[test]
fn valid_unimplemented_realization_is_not_a_noop() {
    rejects(
        "namespace modifier-boundary\nfrom /core/modifiers import modifier\n@[modifier; operation = 'future-storage']\nclass storage\n    size int = 8\nwith (storage; size = 64) global value int = 1\nfunction main;\n    print; value\n",
        "S2120",
    );
}

#[test]
fn modifier_arguments_obey_schema_before_operation_admission() {
    rejects(
        "namespace modifier-boundary\nfrom /core/modifiers import modifier\n@[modifier; operation = 'future-storage']\nclass storage\n    size int = 8\nwith (storage; size = 'not an integer') global value int = 1\nfunction main;\n    print; value\n",
        "S2113",
    );
}

#[test]
fn scoped_modifier_cannot_be_applied_without_a_region() {
    rejects(
        "namespace modifier-boundary\nfunction main;\n    with scoped-access value = 1\n    print; value\n",
        "S2120",
    );
}

#[test]
fn aliases_cannot_duplicate_a_nonrepeatable_modifier() {
    rejects(
        "namespace modifier-boundary\nfrom /core/modifiers import scoped-access as local-scope\nfunction main;\n    with local-scope, (local-scope;) guard = (1) (\n        print; guard\n    )\n",
        "S2120",
    );
}

#[test]
fn distinct_definitions_cannot_apply_conflicting_structural_operations() {
    rejects(
        "namespace modifier-boundary\nfrom /core/modifiers import modifier\n@[modifier; operation = 'scoped-access', structural = true]\nclass local-scope\nfunction main;\n    with local-scope, scoped-access guard = (1) (\n        print; guard\n    )\n",
        "S2120",
    );
}
