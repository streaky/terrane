use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_TEMPORARY_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "terrane-{label}-{}-{}",
            std::process::id(),
            NEXT_TEMPORARY_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        if self.0.exists() {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
}

fn hello() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance/run/hello/case.trn")
}

fn structured_error() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/conformance/run/structured-error-origin-and-frames/case.trn")
}

#[test]
fn all_commands_share_the_hello_pipeline() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let rust = Command::new(binary)
        .args(["rust", hello().to_str().unwrap()])
        .output()
        .unwrap();
    let rust_again = Command::new(binary)
        .args(["rust", hello().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(rust.status.success());
    assert!(rust_again.status.success());
    assert_eq!(rust.stdout, rust_again.stdout);
    let displayed_rust = String::from_utf8(rust.stdout)
        .unwrap()
        .replace(terrane_compiler::VERSION, "<version>");
    let authored_rust = fs::read_to_string(hello().parent().unwrap().join("lower.rs")).unwrap();
    assert!(displayed_rust.starts_with(&authored_rust));
    assert!(displayed_rust.contains("// Generated Rust form: standalone"));
    assert!(displayed_rust.contains("// Vendored support crates: terrane-int-support"));

    let check = Command::new(binary)
        .args(["check", hello().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(check.status.success());

    let build = Command::new(binary)
        .args(["build", hello().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(build.status.success());
    let executable = String::from_utf8(build.stdout).unwrap();
    assert!(Path::new(executable.trim()).is_file());
    let source_root = hello().parent().unwrap().canonicalize().unwrap();
    assert!(Path::new(executable.trim()).starts_with(source_root.join(".trn")));

    let run = Command::new(binary)
        .args(["run", hello().to_str().unwrap()])
        .output()
        .unwrap();
    assert!(run.status.success());
    assert_eq!(
        run.stdout,
        fs::read(hello().parent().unwrap().join("stdout.txt")).unwrap()
    );
}

#[test]
fn rust_output_writes_clean_authored_lowering_and_support_sidecar() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let directory = TemporaryDirectory::new("rust-output");
    let output = directory.path().join("nested/application.rs");
    let lowered = Command::new(binary)
        .args([
            "rust",
            "--output",
            output.to_str().unwrap(),
            structured_error().to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(lowered.status.success(), "{lowered:?}");
    assert!(lowered.stdout.is_empty());
    let entrypoint = fs::read_to_string(&output).unwrap();
    let support = fs::read_to_string(output.with_file_name("application.support.rs")).unwrap();
    assert!(entrypoint.contains("include!(\"application.support.rs\");"));
    assert!(entrypoint.contains("// Namespace: structured-error-origin-and-frames"));
    assert!(entrypoint.contains("fn main()"));
    assert!(!entrypoint.contains("struct TerraneError"));
    assert!(support.contains("struct TerraneError"));
    assert!(support.contains("static SITES:"));
}

#[test]
fn output_options_are_rejected_outside_rust_and_when_repeated() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    for command in ["check", "build", "run"] {
        for flag in ["-o", "--output"] {
            let output = Command::new(binary)
                .args([command, flag, "generated.rs", hello().to_str().unwrap()])
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(2), "{command} {flag}");
            assert!(String::from_utf8(output.stderr).unwrap().contains("usage:"));
        }
    }
    let repeated = Command::new(binary)
        .args([
            "rust",
            "-o",
            "first.rs",
            "--output",
            "second.rs",
            hello().to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert_eq!(repeated.status.code(), Some(2));
    assert!(
        String::from_utf8(repeated.stderr)
            .unwrap()
            .contains("usage:")
    );
}

#[test]
fn help_succeeds_and_extra_arguments_are_rejected() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let help = Command::new(binary).arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(
        String::from_utf8(help.stdout)
            .unwrap()
            .contains("commands:")
    );

    let extra = Command::new(binary)
        .args(["check", hello().to_str().unwrap(), "unexpected"])
        .output()
        .unwrap();
    assert_eq!(extra.status.code(), Some(2));
    assert!(
        String::from_utf8(extra.stderr)
            .unwrap()
            .starts_with("usage:")
    );
}

#[test]
fn canonical_rust_requirement_preserves_successful_rust_output() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let ordinary = Command::new(binary)
        .args(["rust", hello().to_str().unwrap()])
        .output()
        .unwrap();
    let canonical = Command::new(binary)
        .args([
            "rust",
            "--require-canonical-rust",
            hello().to_str().unwrap(),
        ])
        .output()
        .unwrap();

    assert!(ordinary.status.success());
    assert!(canonical.status.success());
    assert_eq!(canonical.stdout, ordinary.stdout);
    assert!(canonical.stderr.is_empty());
}

#[test]
fn failures_use_distinct_exit_codes_and_compiler_diagnostics() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let missing = Command::new(binary)
        .args(["check", "missing.trn"])
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(3));
    let missing_stderr = String::from_utf8(missing.stderr).unwrap();
    assert!(missing_stderr.contains("missing.trn: error[S0000]"));
    assert!(!missing_stderr.contains("missing.trn:1:1"));

    let invalid_path = std::env::temp_dir().join(format!(
        "terrane-invalid-{}-{}.trn",
        std::process::id(),
        std::thread::current().name().unwrap_or("cli")
    ));
    fs::write(
        &invalid_path,
        "namespace invalid\nfunction main;\n  missing;\n",
    )
    .unwrap();
    let invalid = Command::new(binary)
        .args(["check", invalid_path.to_str().unwrap()])
        .output()
        .unwrap();
    fs::remove_file(invalid_path).unwrap();
    assert_eq!(invalid.status.code(), Some(3));
    assert!(
        String::from_utf8(invalid.stderr)
            .unwrap()
            .contains("unresolved name `missing`")
    );
}

#[test]
fn uncaught_source_errors_render_causes_and_terrane_frames() {
    let directory = std::env::temp_dir().join(format!(
        "terrane-runtime-error-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("cli")
    ));
    fs::create_dir_all(&directory).unwrap();
    let source = directory.join("case.trn");
    fs::write(
        &source,
        concat!(
            "namespace runtime-error\n",
            "from /core/errors import arithmetic-overflow, coercion-error\n",
            "function inner int throws arithmetic-overflow;\n",
            "  throw arithmetic-overflow\n",
            "function outer int throws coercion-error;\n",
            "  try\n",
            "    return inner;\n",
            "  catch arithmetic-overflow\n",
            "    throw coercion-error\n",
            "function main;\n",
            "  outer;\n",
        ),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["run", source.to_str().unwrap()])
        .output()
        .unwrap();
    fs::remove_dir_all(&directory).unwrap();

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.starts_with("coercion-error: coercion has no compatible result\n"));
    assert!(stderr.contains("caused by: arithmetic-overflow"));
    assert!(stderr.contains("at /runtime-error::inner (case.trn:4:3-4:28)"));
    assert!(stderr.contains("at /runtime-error::outer (case.trn:7:12-7:18)"));
    assert!(stderr.contains("at /runtime-error::outer (case.trn:9:5-9:25)"));
    assert!(stderr.contains("at /runtime-error::main (case.trn:11:3-11:9)"));
    assert!(!stderr.contains("panicked"));
    assert!(!stderr.contains("src/authored"));
}
