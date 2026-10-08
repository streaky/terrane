use std::{
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const HELLO: &str = include_str!("../../../tests/conformance/run/hello/case.trn");
const ASYNC_AWAIT: &str = include_str!("../../../tests/conformance/run/async-await/case.trn");
const STRUCTURED_ERROR: &str =
    include_str!("../../../tests/conformance/run/structured-error-origin-and-frames/case.trn");

#[test]
fn hello_lowers_deterministically() {
    let first = terrane_compiler::compile(PathBuf::from("case.trn"), HELLO.to_owned()).unwrap();
    let second = terrane_compiler::compile(PathBuf::from("case.trn"), HELLO.to_owned()).unwrap();
    assert_eq!(first.rust, second.rust);
    let first_files = first.rust_files_for(Path::new("generated/app.rs")).unwrap();
    let second_files = second
        .rust_files_for(Path::new("generated/app.rs"))
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
            lint_unused_functions: false,
            debug_build: terrane_compiler::DebugBuild::Disabled,
        },
    )
    .unwrap();

    let files = compilation
        .rust_files_for(Path::new("src/main.rs"))
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
        .rust_files_for(Path::new("src/main.rs"))
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
        .rust_files_for(Path::new("src/main.rs"))
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
fn bundled_core_lowering_is_part_of_the_support_sidecar() {
    let compilation = terrane_compiler::compile(
        "process-user.trn",
        "namespace process-user\n\
         from /core/output import print\n\
         from /core/process import process-host-name\n\
         function main;\n\
             name = process-host-name;\n\
             print; name.failed\n"
            .to_owned(),
    )
    .unwrap();
    let files = compilation
        .rust_files_for(Path::new("src/main.rs"))
        .unwrap();
    let support = &files[0].contents;
    let entrypoint = &files[1].contents;

    assert!(support.contains("// Source: core/process.trn"));
    assert!(support.contains("// Namespace: core/process"));
    assert!(!entrypoint.contains("// Source: core/process.trn"));
    assert!(entrypoint.contains("// Source: process-user.trn"));
}

#[test]
fn projected_dependency_lowering_is_part_of_the_support_sidecar() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/conformance/run/rust-dependency-deferred-surface");
    let staged = std::env::temp_dir().join(format!(
        "terrane-projected-pipeline-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(staged.join("src")).unwrap();
    for relative in ["package.toml", "src/main.trn", "terrane-projection.lock"] {
        let destination = staged.join(relative);
        fs::copy(fixture.join(relative), destination).unwrap();
    }
    let package = terrane_compiler::Package::load(staged.join("package.toml")).unwrap();
    let compilation = terrane_compiler::compile_package(&package).unwrap();
    let files = compilation
        .rust_files_for(Path::new("src/main.rs"))
        .unwrap();
    let support = &files[0].contents;
    let entrypoint = &files[1].contents;

    assert!(support.contains("// Namespace: deps/bytes"));
    assert!(!entrypoint.contains("// Namespace: deps/"));
    assert!(entrypoint.contains("// Namespace: app"));
    fs::remove_dir_all(staged).unwrap();
}
#[test]
fn split_lowering_uses_the_requested_entrypoint_name() {
    let compilation =
        terrane_compiler::compile(PathBuf::from("case.trn"), HELLO.to_owned()).unwrap();
    let files = compilation
        .rust_files_for(Path::new("generated/inspectable.rs"))
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
        "namespace process\nfrom /core/process import process-host-name\nfunction main;\n    name = process-host-name;\n"
            .to_owned(),
    )
    .unwrap();
    assert!(process.requires_platform_support);
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

