use terrane_compiler::{SourceFile, lexer::lex, parser::parse};

fn reject_source(body: &str, expected: &str) {
    let source = format!("namespace interpolation-rejection\n{body}\n");
    let errors = terrane_compiler::compile("interpolation-rejection.trn", source)
        .expect_err("invalid interpolation must be rejected");
    assert!(
        errors.iter().any(|error| error.code == expected),
        "{errors:?}"
    );
}

#[test]
fn invalid_interpreted_escapes_are_lexical_errors() {
    for literal in [r#""\q""#, r#""\u{}""#, r#""\u{D800}""#, r#""\u{110000}""#] {
        let source = SourceFile::new(0, "escape.trn".into(), format!("value = {literal}"));
        let errors = lex(&source).expect_err("invalid interpreted escape");
        assert!(
            errors.iter().any(|error| error.code == "L0013"),
            "{errors:?}"
        );
    }
}

#[test]
fn nested_single_literals_keep_unknown_escapes() {
    let source = SourceFile::new(
        0,
        "nested.trn".into(),
        r#"value = "{format; '\q'}""#.to_owned(),
    );
    let parsed = parse(
        &source,
        lex(&source).expect("inner single quotes preserve backslashes"),
    );
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
}

#[test]
fn objects_require_render_not_debug_or_to_string() {
    reject_source(
        "class item\n    function to-string string;\n        return 'not interpolation'\nfunction main;\n    value = instance item;\n    print; \"{value}\"",
        "T0055",
    );
}

#[test]
fn renderer_result_must_be_synchronous_string() {
    for declaration in [
        "function render int;\n        return 1",
        "async function render string;\n        return 'asynchronous'",
    ] {
        reject_source(
            &format!(
                "class item\n    {declaration}\nfunction main;\n    value = instance item;\n    print; \"{{value}}\""
            ),
            "T0251",
        );
    }
}

#[test]
fn incompatible_registration_does_not_fall_back_to_method() {
    reject_source(
        concat!(
            "from /core/modifiers import interpolation-renderer\n",
            "class item\n    function render string; flag string = ''\n        return flag\n",
            "function number int; value item\n    return 1\n",
            "function main;\n    with (interpolation-renderer; key = 'overview') renderer function from item to int = number\n",
            "    value = instance item;\n    print; \"{value !: overview}\""
        ),
        "T0251",
    );
}

#[test]
fn duplicate_renderer_keys_are_rejected() {
    reject_source(
        concat!(
            "from /core/modifiers import interpolation-renderer\n",
            "function text string; value int\n    return value.to-string;\n",
            "function main;\n",
            "    with (interpolation-renderer; key = 'number') first function from int to string = text\n",
            "    with (interpolation-renderer; key = 'number') second function from int to string = text\n",
            "    print; \"{1 !: number}\""
        ),
        "T0250",
    );
}

#[test]
fn conditional_registration_assignment_requires_all_paths() {
    reject_source(
        concat!(
            "from /core/modifiers import interpolation-renderer\n",
            "function text string; value int\n    return value.to-string;\n",
            "function choose string; selected bool\n",
            "    with (interpolation-renderer; key = 'number') renderer function from int to string\n",
            "    if selected\n        renderer = text\n",
            "    return \"{1 !: number}\""
        ),
        "T0252",
    );
}
