use std::path::PathBuf;

const HELLO: &str = include_str!("../../../tests/conformance/run/hello/case.trn");
const ASYNC_AWAIT: &str = include_str!("../../../tests/conformance/run/async-await/case.trn");
const STRUCTURED_ERROR: &str =
    include_str!("../../../tests/conformance/run/structured-error-origin-and-frames/case.trn");

fn normalized_rust(rust: &str) -> String {
    rust.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn hello_lowers_deterministically() {
    let first = terrane_compiler::compile(PathBuf::from("case.trn"), HELLO.to_owned()).unwrap();
    let second = terrane_compiler::compile(PathBuf::from("case.trn"), HELLO.to_owned()).unwrap();
    assert_eq!(first.rust, second.rust);
    let first_files = first
        .rust_files_for(std::path::Path::new("generated/app.rs"))
        .unwrap();
    let second_files = second
        .rust_files_for(std::path::Path::new("generated/app.rs"))
        .unwrap();
    assert_eq!(first_files, second_files);
    assert_eq!(
        first_files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        ["generated/app.support.rs", "generated/app.rs"]
    );
    assert!(
        first
            .rust
            .contains("Hello from Terrane!\\n\\nTail strings make punctuation literal")
    );
}

#[cfg(unix)]
#[test]
fn generated_rust_paths_report_non_utf8_file_names() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    let compilation =
        terrane_compiler::compile(PathBuf::from("case.trn"), HELLO.to_owned()).unwrap();
    let path = PathBuf::from(OsString::from_vec(b"generated/\xff.rs".to_vec()));
    let error = compilation.rust_files_for(&path).unwrap_err();

    assert!(matches!(
        error,
        terrane_compiler::RustArtifactError::InvalidOutputPath(message)
            if message == "generated Rust output file name must be valid UTF-8"
    ));
}

#[test]
fn canonical_rust_requirement_accepts_formatted_lowering() {
    let compilation = terrane_compiler::compile_with_options(
        PathBuf::from("case.trn"),
        HELLO.to_owned(),
        terrane_compiler::CompilerOptions {
            require_canonical_rust: true,
            lint_name_style: false,
        },
    )
    .unwrap();

    let files = compilation
        .rust_files_for(std::path::Path::new("src/main.rs"))
        .unwrap();
    assert_eq!(
        files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        ["src/main.support.rs", "src/main.rs"]
    );
}

#[test]
fn compiler_runtime_support_uses_named_generated_files() {
    let compilation =
        terrane_compiler::compile(PathBuf::from("async-await.trn"), ASYNC_AWAIT.to_owned())
            .unwrap();
    let files = compilation
        .rust_files_for(std::path::Path::new("src/main.rs"))
        .unwrap();
    assert_eq!(
        files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        ["src/main.support.rs", "src/main.rs"]
    );
    let support = &files[0].contents;
    let entrypoint = &files[1].contents;
    assert!(support.contains("async fn __terrane_await"));
    assert!(!entrypoint.contains("async fn __terrane_await"));
    assert!(entrypoint.starts_with(
        "// Generated deterministically by Terrane 0.1.0.\n\
         include!(\"main.support.rs\");\n\
         // Source: async-await.trn\n\
         // Namespace: async-await\n"
    ));
}

#[test]
fn structured_error_infrastructure_is_separate_from_authored_lowering() {
    let compilation = terrane_compiler::compile(
        "structured-error-origin-and-frames.trn",
        STRUCTURED_ERROR.to_owned(),
    )
    .unwrap();
    let files = compilation
        .rust_files_for(std::path::Path::new("src/main.rs"))
        .unwrap();
    let support = &files[0].contents;
    let entrypoint = &files[1].contents;

    assert!(support.contains("struct TerraneError"));
    assert!(support.contains("static SITES:"));
    assert!(!entrypoint.contains("struct TerraneError"));
    assert!(!entrypoint.contains("static SITES:"));
    assert!(entrypoint.contains("fn main()"));
    assert!(compilation.rust.contains("struct TerraneError"));
    assert!(compilation.rust.contains("fn main()"));
}

#[test]
fn bundled_standard_lowering_is_part_of_the_support_sidecar() {
    let compilation = terrane_compiler::compile(
        "process-user.trn",
        "namespace process-user\n\
         from /core/output import print\n\
         from /standard/process import host-name\n\
         function main;\n\
             name = host-name;\n\
             print; name.failed\n"
            .to_owned(),
    )
    .unwrap();
    let files = compilation
        .rust_files_for(std::path::Path::new("src/main.rs"))
        .unwrap();
    let support = &files[0].contents;
    let entrypoint = &files[1].contents;

    assert!(support.contains("// Source: standard/process.trn"));
    assert!(support.contains("// Namespace: standard/process"));
    assert!(!entrypoint.contains("// Source: standard/process.trn"));
    assert!(entrypoint.contains("// Source: process-user.trn"));
}

#[test]
fn projected_dependency_lowering_is_part_of_the_support_sidecar() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/conformance/run/rust-dependency-deferred-surface/package.toml");
    let package = terrane_compiler::Package::load(&manifest).unwrap();
    let compilation = terrane_compiler::compile_package(&package).unwrap();
    let files = compilation
        .rust_files_for(std::path::Path::new("src/main.rs"))
        .unwrap();
    let support = &files[0].contents;
    let entrypoint = &files[1].contents;

    assert!(support.contains("// Namespace: deps/bytes/bytes-mut"));
    assert!(!entrypoint.contains("// Namespace: deps/"));
    assert!(entrypoint.contains("// Namespace: app"));
}
#[test]
fn split_lowering_uses_the_requested_entrypoint_name() {
    let compilation =
        terrane_compiler::compile(PathBuf::from("case.trn"), HELLO.to_owned()).unwrap();
    let files = compilation
        .rust_files_for(std::path::Path::new("generated/inspectable.rs"))
        .unwrap();
    assert_eq!(
        files
            .iter()
            .map(|file| file.path.as_str())
            .collect::<Vec<_>>(),
        [
            "generated/inspectable.support.rs",
            "generated/inspectable.rs"
        ]
    );
    assert!(files[0].contents.is_empty());
    assert!(files[1].contents.starts_with(
        "// Generated deterministically by Terrane 0.1.0.\n\
         include!(\"inspectable.support.rs\");\n\
         // Source: case.trn\n"
    ));
}

#[test]
fn platform_support_requirement_comes_from_lowering_metadata() {
    let literal = terrane_compiler::compile(
        "literal.trn",
        "namespace literal\nfunction main;\n    value = 'terrane_platform_support::'\n".to_owned(),
    )
    .unwrap();
    assert!(!literal.requires_platform_support);

    let process = terrane_compiler::compile(
        "process.trn",
        "namespace process\nfrom /standard/process import host-name\nfunction main;\n    name = host-name;\n"
            .to_owned(),
    )
    .unwrap();
    assert!(process.requires_platform_support);
}

#[test]
fn inferred_local_reassignment_lowers_as_assignment() {
    let source = "namespace inferred\nfunction main;\n  total = 5\n  total = total + 1\n";
    let compilation = terrane_compiler::compile("inferred.trn", source.to_owned()).unwrap();

    assert!(compilation.rust.contains(
        "let mut total: terrane_int_support::Int = terrane_int_support::Int::from(5_i128);"
    ));
    assert!(!compilation.rust.contains("let _ = &total;"));
    assert!(
        compilation
            .rust
            .contains("total = total.clone() + terrane_int_support::Int::from(1_i128);")
    );
}

#[test]
fn annotated_replacement_lowers_as_source_ordered_shadowing() {
    let source = concat!(
        "namespace replacement\n",
        "from /core/types import int8\n",
        "function main;\n",
        "  value int8 = 12\n",
        "  value int = value.coerce; int\n",
        "  print; value\n",
        "function second;\n",
        "  value int8 = 7\n",
        "  print; value\n",
        "function blocks;\n",
        "  if true\n",
        "    value int8 = 1\n",
        "    value int = value.coerce; int\n",
        "  if true\n",
        "    value int8 = 2\n",
    );
    let compilation = terrane_compiler::compile("replacement.trn", source.to_owned()).unwrap();

    let replacement_consumptions = compilation
        .rust
        .lines()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|lines| {
            lines[0].trim() == "let _ = &value;" && lines[1].trim_start().starts_with("let value:")
        })
        .count();
    assert_eq!(replacement_consumptions, 2);
}

#[test]
fn rejects_duplicate_declarations() {
    let cases = [
        (
            "namespace hello",
            "S0005",
            "duplicate namespace declaration",
        ),
        ("function main;", "S2005", "duplicate declaration `main`"),
    ];

    for (construct, code, message) in cases {
        let source = HELLO.replacen(construct, &format!("{construct}\n{construct}"), 1);
        let diagnostics = terrane_compiler::compile("duplicate.trn", source).unwrap_err();
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == code && diagnostic.message == message)
        );
    }
}

#[test]
fn rejects_mixed_indentation() {
    let source = HELLO.replace("  print", " \tprint");
    let diagnostics = terrane_compiler::compile("mixed.trn", source).unwrap_err();
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "L0003")
    );
}

#[test]
fn blank_lines_do_not_select_indentation_style() {
    let source = HELLO
        .replace(
            "function main;\n  print; >>",
            "function main;\n \n\tprint; >>",
        )
        .replace("\n    Hello from Terrane!", "\n\t\tHello from Terrane!")
        .replace("\n    Tail strings", "\n\t\tTail strings");
    terrane_compiler::compile("blank-indent.trn", source).unwrap();
}

#[test]
fn permits_a_comment_after_a_closed_quote() {
    let source = HELLO.replace(
        "print; >>\n    Hello from Terrane!\n\n    Tail strings make punctuation literal: >, #, \"quotes\".",
        "print; 'hello' # trailing comment",
    );
    let compilation = terrane_compiler::compile("trailing-comment.trn", source).unwrap();
    assert!(compilation.rust.contains("String::from(\"hello\")"));
}

#[test]
fn compilation_failure_owns_the_original_source() {
    let source = "namespace app\nfunction main;\n  print; missing\n".to_owned();
    let failure = terrane_compiler::compile("owned.trn", source.clone()).unwrap_err();
    assert_eq!(failure.source.text(), source);
    assert_eq!(failure.source.path(), PathBuf::from("owned.trn").as_path());
    assert!(failure.iter().any(|diagnostic| diagnostic.code == "S2013"));
}

#[test]
fn tail_string_preserves_every_remaining_character() {
    let source = HELLO.replace(
        "print; >>\n    Hello from Terrane!\n\n    Tail strings make punctuation literal: >, #, \"quotes\".",
        "print; >Hello! From, \"Terrane\"! >> # literal",
    );
    let compilation = terrane_compiler::compile("tail.trn", source).unwrap();
    assert!(
        compilation
            .rust
            .contains("Hello! From, \\\"Terrane\\\"! >> # literal")
    );
}

#[test]
fn tail_string_can_be_empty() {
    let source = HELLO.replace(
        "print; >>\n    Hello from Terrane!\n\n    Tail strings make punctuation literal: >, #, \"quotes\".",
        "print; >",
    );
    let compilation = terrane_compiler::compile("empty-tail.trn", source).unwrap();
    assert!(compilation.rust.contains("String::from(\"\")"));
}

#[test]
fn tail_string_preserves_leading_whitespace() {
    let source = HELLO.replace(
        "print; >>\n    Hello from Terrane!\n\n    Tail strings make punctuation literal: >, #, \"quotes\".",
        "print; > hello",
    );
    let compilation = terrane_compiler::compile("leading-space.trn", source).unwrap();
    assert!(compilation.rust.contains("String::from(\" hello\")"));
}
#[test]
fn block_string_can_be_empty() {
    let source = HELLO.replace(
        "print; >>\n    Hello from Terrane!\n\n    Tail strings make punctuation literal: >, #, \"quotes\".",
        "print; >>",
    );
    let compilation = terrane_compiler::compile("string.trn", source).unwrap();
    assert!(compilation.rust.contains("String::from(\"\")"));
}

#[test]
fn rejects_trailing_content_after_block_marker() {
    let source = HELLO.replace("print; >>", "print; >> ");
    let diagnostics = terrane_compiler::compile("marker.trn", source).unwrap_err();
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "L0008" && diagnostic.message.contains("final content")
    }));
}

#[test]
fn rejects_unresolved_name() {
    let source = HELLO.replace(
        "print; >>\n    Hello from Terrane!\n\n    Tail strings make punctuation literal: >, #, \"quotes\".",
        "print; missing",
    );
    let diagnostics = terrane_compiler::compile("name.trn", source).unwrap_err();
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "S2013")
    );
}

#[test]
fn rejects_unresolved_call_argument() {
    let source = HELLO.replace(
        "print; >>\n    Hello from Terrane!\n\n    Tail strings make punctuation literal: >, #, \"quotes\".",
        "print; hello",
    );
    let diagnostics = terrane_compiler::compile("call.trn", source).unwrap_err();
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "S2013")
    );
}

#[test]
fn compilation_uses_the_shared_parser_before_semantics() {
    let source = HELLO.replace("function main", "function main; ,");
    let diagnostics = terrane_compiler::compile("syntax.trn", source).unwrap_err();
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "S1007")
    );
    assert!(
        !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "S0005")
    );
}

#[test]
fn lowers_collection_and_three_clause_for_loops_without_losing_continue_updates() {
    let collection = terrane_compiler::compile(
        "collection.trn",
        "namespace app\nfunction main;\n  text string = 'ab'\n  for character in text\n    value = character\n"
            .to_owned(),
    )
    .unwrap();
    assert!(collection.rust.contains(
        "let mut __terrane_iterator_0 = terrane_collection_support::string_iterator(&text);"
    ));
    assert!(
        collection
            .rust
            .contains("terrane_collection_support::IterationStep::Item(item) => item,")
    );
    assert!(
        collection
            .rust
            .contains("terrane_collection_support::IterationStep::End => break,")
    );

    let clauses = terrane_compiler::compile(
        "clauses.trn",
        "namespace app\nfunction main;\n  for index = 0; index < 3; index++\n    if index == 1\n      continue\n"
            .to_owned(),
    )
    .unwrap();
    assert!(clauses.rust.contains("'__terrane_continue_0: {"));
    let continue_position = clauses.rust.find("break '__terrane_continue_0;").unwrap();
    let update_position = clauses
        .rust
        .find("index = index.clone() + terrane_int_support::Int::from(1_i128);")
        .unwrap();
    assert!(continue_position < update_position);
}

#[test]
fn eliminates_only_statically_impossible_fixed_integer_failures() {
    let proven = terrane_compiler::compile(
        "numeric-proofs.trn",
        concat!(
            "namespace numeric-proofs\n",
            "from /core/types import int64, float64\n",
            "function main;\n",
            "  index int64 = 0\n",
            "  while index < 100\n",
            "    arrival float64 = index\n",
            "    remainder int64 = index % 17\n",
            "    index++\n",
        )
        .to_owned(),
    )
    .unwrap();
    let proven = normalized_rust(&proven.rust);
    assert!(proven.contains("let arrival: f64 = index as f64;"));
    assert!(proven.contains("let remainder: i64 = index.rem_euclid(17);"));
    assert!(proven.contains("index = index + 1;"));
    assert!(!proven.contains("terrane_int_support::exact_fixed_f64(index)"));
    assert!(!proven.contains("terrane_int_support::fixed_remainder(index, 17)"));
    assert!(!proven.contains("terrane_int_support::fixed_addition(index, 1)"));

    let unproven = terrane_compiler::compile(
        "numeric-checks.trn",
        concat!(
            "namespace numeric-checks\n",
            "from /core/types import int64, float64\n",
            "function main;\n",
            "  index int64 = 0\n",
            "  divisor int64 = 17\n",
            "  while index <= 100\n",
            "    arrival float64 = index\n",
            "    remainder int64 = index % divisor\n",
            "    index++\n",
        )
        .to_owned(),
    )
    .unwrap();
    assert!(
        unproven
            .rust
            .contains("terrane_int_support::exact_fixed_f64(index)")
    );
    assert!(
        unproven
            .rust
            .contains("terrane_int_support::fixed_remainder(index, divisor)")
    );
    assert!(
        unproven
            .rust
            .contains("terrane_int_support::fixed_addition(index, 1)")
    );

    let mutated_before_loop = terrane_compiler::compile(
        "mutated-before-loop.trn",
        concat!(
            "namespace mutated-before-loop\n",
            "from /core/types import int64, float64\n",
            "function main;\n",
            "  index int64 = 0\n",
            "  index = 9007199254740993\n",
            "  while index < 100\n",
            "    arrival float64 = index\n",
            "    index++\n",
        )
        .to_owned(),
    )
    .unwrap();
    assert!(
        mutated_before_loop
            .rust
            .contains("terrane_int_support::exact_fixed_f64(index)")
    );
}

#[test]
fn lowers_scalar_membership_and_descriptor_identity_statically() {
    let source = concat!(
        "namespace descriptors\n",
        "from /core/types import int8 as byte, int8 as other-byte\n",
        "function accepts; item int\n",
        "  parameter-member = item is a int\n",
        "function main;\n",
        "  value = 1\n",
        "  member = value is a int\n",
        "  same-descriptor = byte is byte\n",
        "  different-alias = byte is other-byte\n",
        "  same-scalar = value is value\n",
        "  same-value-type = value.type is value.type\n",
        "  different-value-type = value.type is byte\n",
    );
    let compilation = terrane_compiler::compile("descriptors.trn", source.to_owned()).unwrap();
    let rust = normalized_rust(&compilation.rust);

    assert!(rust.contains("let member: bool = { let _ = &value; true };"));
    assert!(rust.contains("let parameter_member: bool = { let _ = &item; true };"));
    assert!(rust.contains("let same_descriptor: bool = { true };"));
    assert!(rust.contains("let different_alias: bool = { true };"));
    assert!(rust.contains("let same_scalar: bool = { let _ = value; let _ = value; false };"));
    assert!(rust.contains("let same_value_type: bool = { let _ = value; let _ = value; true };"));
    assert!(rust.contains("let different_value_type: bool = { let _ = value; false };"));
}

#[test]
fn lowers_named_arguments_into_parameter_order_with_defaults() {
    let source = concat!(
        "namespace calls\n",
        "function combine int; first int, second int = 2, third int = 3\n",
        "  return first + second + third\n",
        "function main;\n",
        "  result = combine; 1, third = 9\n",
    );
    let compilation = terrane_compiler::compile("calls.trn", source.to_owned()).unwrap();
    let rust = normalized_rust(&compilation.rust);
    assert!(rust.contains("let result: terrane_int_support::Int = combine("));
    assert!(rust.contains("terrane_int_support::Int::from(1_i128)"));
    assert!(rust.contains("terrane_int_support::Int::from(2_i128)"));
    assert!(rust.contains("terrane_int_support::Int::from(9_i128)"));
}

#[test]
fn does_not_lower_shadowing_functions_as_builtins() {
    let source = concat!(
        "namespace shadowing\n",
        "function print int; value int\n",
        "  return value\n",
        "function main;\n",
        "  result = print; 1\n",
    );
    let compilation = terrane_compiler::compile("shadowing.trn", source.to_owned()).unwrap();

    assert!(
        compilation
            .rust
            .contains("print(terrane_int_support::Int::from(1_i128))")
    );
    assert!(!compilation.rust.contains("println!"));
}

#[test]
fn unwraps_only_syntactic_condition_groups() {
    let source = concat!(
        "namespace conditions\n",
        "function main;\n",
        "  if ((true))\n",
        "    print; 'yes'\n",
    );
    let compilation = terrane_compiler::compile("conditions.trn", source.to_owned()).unwrap();

    assert!(compilation.rust.contains("if true {"));
}

#[test]
fn lowers_logical_combinations_of_integer_comparisons() {
    let source = concat!(
        "namespace conditions\n",
        "function main;\n",
        "  x int = 5\n",
        "  y int = 9\n",
        "  if x > 1 and y > 2\n",
        "    result = true\n",
    );
    let compilation = terrane_compiler::compile("conditions.trn", source.to_owned()).unwrap();

    assert!(compilation.rust.contains("if x > 1 && y > 2 {"));
}

#[test]
fn lowers_values_in_their_integer_destination_type() {
    let source = concat!(
        "namespace destinations\n",
        "function answer int;\n",
        "  return 41\n",
        "function main;\n",
        "  text = 'Terrane'\n",
        "  total int = text.length\n",
        "  total = total + 1\n",
    );
    let compilation = terrane_compiler::compile("destinations.trn", source.to_owned()).unwrap();
    let rust = normalized_rust(&compilation.rust);

    assert!(rust.contains("return terrane_int_support::Int::from(41_i128);"));
    assert!(rust.contains("let mut total: terrane_int_support::Int"));
    assert!(rust.contains("terrane_string_support::length(&text) as i128"));
    assert!(!compilation.rust.contains("let _ = &total;"));
    assert!(rust.contains("total = total.clone() + terrane_int_support::Int::from(1_i128);"));
}

#[test]
fn lowers_fixed_width_arithmetic_through_checked_runtime_operations() {
    let source = concat!(
        "namespace fixed\n",
        "from /core/types import int8\n",
        "function main;\n",
        "  left int8 = 120\n",
        "  right int8 = 10\n",
        "  sum int8 = left + right\n",
        "  quotient int8 = left / right\n",
        "  shifted int8 = left << right\n",
    );
    let compilation = terrane_compiler::compile("fixed.trn", source.to_owned()).unwrap();
    let rust = normalized_rust(&compilation.rust);

    assert!(rust.contains("terrane_int_support::fixed_addition(left, right)"));
    assert!(rust.contains("terrane_int_support::fixed_division(left, right)"));
    assert!(rust.contains("terrane_int_support::fixed_shift_left(left, &right)"));
}
