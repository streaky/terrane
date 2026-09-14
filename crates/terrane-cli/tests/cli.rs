use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
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

fn staged_hello() -> TemporaryDirectory {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance/run/hello");
    let directory = TemporaryDirectory::new("hello-fixture");
    fs::create_dir_all(directory.path()).unwrap();
    for name in ["case.trn", "stdout.txt"] {
        fs::copy(source.join(name), directory.path().join(name)).unwrap();
    }
    directory
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
    let directory = staged_hello();
    let hello = directory.path().join("case.trn");
    let rust = Command::new(binary)
        .args(["rust", hello.to_str().unwrap()])
        .output()
        .unwrap();
    let rust_again = Command::new(binary)
        .args(["rust", hello.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(rust.status.success());
    assert!(rust_again.status.success());
    assert_eq!(rust.stdout, rust_again.stdout);
    let displayed_rust = String::from_utf8(rust.stdout)
        .unwrap()
        .replace(terrane_compiler::VERSION, "<version>");
    let source = fs::read_to_string(&hello).unwrap();
    let standalone_rust = terrane_compiler::compile(&hello, source)
        .unwrap()
        .rust
        .replace(terrane_compiler::VERSION, "<version>");
    assert!(displayed_rust.starts_with(&standalone_rust));
    assert!(displayed_rust.contains("// Generated Rust form: standalone"));
    assert!(displayed_rust.contains("// Vendored support crates: terrane-int-support"));

    let check = Command::new(binary)
        .args(["check", hello.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(check.status.success());

    let build = Command::new(binary)
        .args(["build", hello.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(build.status.success());
    let executable = String::from_utf8(build.stdout).unwrap();
    assert!(Path::new(executable.trim()).is_file());
    let source_root = directory.path().canonicalize().unwrap();
    assert!(Path::new(executable.trim()).starts_with(source_root.join(".trn")));

    let run = Command::new(binary)
        .args(["run", hello.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(run.status.success());
    assert_eq!(
        run.stdout,
        fs::read(directory.path().join("stdout.txt")).unwrap()
    );
}

#[cfg(unix)]
#[test]
fn executable_shebang_script_runs_through_implicit_command() {
    use std::os::unix::fs::PermissionsExt as _;

    let binary = PathBuf::from(env!("CARGO_BIN_EXE_terrane"));
    let directory = TemporaryDirectory::new("executable-script");
    fs::create_dir_all(directory.path()).unwrap();
    let script = directory.path().join("thing.trn");
    fs::write(
        &script,
        "#!/usr/bin/env terrane\nfunction main;\n  print; >hello\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&script).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&script, permissions).unwrap();

    let mut path_entries = vec![binary.parent().unwrap().to_path_buf()];
    if let Some(path) = std::env::var_os("PATH") {
        path_entries.extend(std::env::split_paths(&path));
    }
    let output = Command::new(&script)
        .env("PATH", std::env::join_paths(path_entries).unwrap())
        .output()
        .unwrap();

    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"hello\n");
    assert!(output.stderr.is_empty(), "{output:?}");
}

#[test]
fn extensionless_source_and_package_paths_dispatch_consistently() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let directory = TemporaryDirectory::new("implicit-paths");
    fs::create_dir_all(directory.path()).unwrap();

    let script = directory.path().join("script");
    fs::write(
        &script,
        "#!/usr/bin/env terrane\nfunction main;\n  print; >source\n",
    )
    .unwrap();
    let source = Command::new(binary).arg(&script).output().unwrap();
    assert!(source.status.success(), "{source:?}");
    assert_eq!(source.stdout, b"source\n");

    for command in ["check", "rust", "build", "run"] {
        let explicit = Command::new(binary)
            .arg(command)
            .arg(&script)
            .output()
            .unwrap();
        assert!(explicit.status.success(), "{command}: {explicit:?}");
        if command == "run" {
            assert_eq!(explicit.stdout, source.stdout);
        }
    }

    let package_root = directory.path().join("package");
    fs::create_dir_all(package_root.join("src")).unwrap();
    fs::write(
        package_root.join("package.toml"),
        "package = \"implicit-package\"\n\n[namespaces]\napp = \"src\"\n",
    )
    .unwrap();
    fs::write(
        package_root.join("src/main.trn"),
        "namespace app\nfunction main;\n  print; >package\n",
    )
    .unwrap();
    let package = Command::new(binary)
        .arg(package_root.join("package.toml"))
        .output()
        .unwrap();
    assert!(package.status.success(), "{package:?}");
    assert_eq!(package.stdout, b"package\n");

    let directory_argument = Command::new(binary).arg(&package_root).output().unwrap();
    assert_eq!(directory_argument.status.code(), Some(2));
    assert!(directory_argument.stdout.is_empty());
    assert!(
        String::from_utf8(directory_argument.stderr)
            .unwrap()
            .starts_with("usage: terrane ")
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
fn invalid_rust_output_path_is_not_reported_as_a_canonical_compiler_defect() {
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["rust", "--output", "", hello().to_str().unwrap()])
        .output()
        .unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert_eq!(output.status.code(), Some(5));
    assert!(stderr.contains("error[S9002]"));
    assert!(!stderr.contains("error[S9004]"));
    assert!(stderr.contains("generated Rust output path has no file name"));
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
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("commands:"));
    assert!(help.contains("<file-or-manifest>"));
    assert!(!help.contains("<source.trn>"));

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

#[test]
fn external_tooling_clients_receive_versioned_protocol_frames() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let directory = TemporaryDirectory::new("tooling-client");
    fs::create_dir_all(directory.path()).unwrap();
    let request = serde_json::json!({
        "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
        "request_id": "open-one-shot",
        "operation": "open-snapshot",
        "sources": [{
            "uri": "file:///workspace/client.trn",
            "text": "namespace client\n\nfunction main;\n"
        }],
        "options": {
            "semantic": true,
            "generated": true,
            "generated_entrypoint": "generated/client.rs"
        }
    });
    let batch = serde_json::json!([
        request.clone(),
        {
            "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
            "request_id": "syntax-one-shot",
            "operation": "syntax",
            "snapshot_id": "$last",
            "uri": "file:///workspace/client.trn"
        },
        {
            "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
            "request_id": "generated-one-shot",
            "operation": "generated-rust",
            "snapshot_id": "$last",
            "uri": "file:///workspace/client.trn",
            "node_id": 0,
            "build_id": "$last-build"
        }
    ]);
    let request_path = directory.path().join("request.json");
    fs::write(&request_path, batch.to_string()).unwrap();
    let one_shot = Command::new(binary)
        .args(["query", "--request"])
        .arg(&request_path)
        .output()
        .unwrap();
    assert!(one_shot.status.success(), "{one_shot:?}");
    assert!(one_shot.stderr.is_empty(), "{one_shot:?}");
    let responses = String::from_utf8(one_shot.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(responses.len(), 3);
    assert_eq!(responses[0]["request_id"], "open-one-shot");
    assert_eq!(responses[1]["request_id"], "syntax-one-shot");
    assert_eq!(responses[2]["request_id"], "generated-one-shot");
    assert_eq!(
        responses[0]["schema_version"],
        terrane_compiler::tooling::SCHEMA_VERSION
    );
    assert!(
        responses[0]["snapshot_id"]
            .as_str()
            .unwrap()
            .starts_with("sha256:")
    );
    assert!(
        responses[0]["source_hash"]
            .as_str()
            .unwrap()
            .starts_with("sha256:")
    );
    assert_eq!(responses[1]["result"]["root"]["kind"], "CompilationUnit");
    assert!(
        responses[2]["result"]["known"]
            .as_array()
            .is_some_and(|locations| locations.iter().any(|location| {
                location["path"]
                    .as_str()
                    .is_some_and(|path| path.ends_with("generated/client.rs"))
            }))
    );
}

#[test]
fn tooling_stdio_returns_envelopes_for_malformed_requests() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let request = serde_json::json!({
        "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
        "request_id": "open-stdio",
        "operation": "open-snapshot",
        "sources": [{
            "uri": "file:///workspace/client.trn",
            "text": "namespace client\n\nfunction main;\n"
        }],
        "options": {"semantic": true, "generated": true}
    });
    let mut service = Command::new(binary)
        .args(["tooling", "--stdio"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = service.stdin.take().unwrap();
    writeln!(stdin, "{request}").unwrap();
    writeln!(
        stdin,
        "{}",
        serde_json::json!({
            "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
            "request_id": "syntax-stdio",
            "operation": "syntax",
            "snapshot_id": "$last",
            "uri": "file:///workspace/client.trn"
        })
    )
    .unwrap();
    writeln!(
        stdin,
        "{}",
        serde_json::json!({
            "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
            "request_id": "generated-stdio",
            "operation": "generated-rust",
            "snapshot_id": "$last",
            "uri": "file:///workspace/client.trn",
            "node_id": 0,
            "build_id": "$last-build"
        })
    )
    .unwrap();
    writeln!(
        stdin,
        "{}",
        serde_json::json!({
            "schema_version": "999.0",
            "request_id": "future-schema",
            "operation": "unknown-future-operation"
        })
    )
    .unwrap();
    writeln!(
        stdin,
        "{}",
        serde_json::json!({
            "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
            "request_id": 42,
            "operation": "syntax",
            "snapshot_id": "sha256:missing",
            "uri": "file:///workspace/client.trn",
            "node_id": null
        })
    )
    .unwrap();
    writeln!(stdin, "{{not-json").unwrap();
    drop(stdin);
    let output = service.wait_with_output().unwrap();
    assert!(output.status.success(), "{output:?}");
    let frames = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(frames.len(), 6);
    assert!(frames[0]["error"].is_null());
    assert_eq!(frames[1]["result"]["root"]["kind"], "CompilationUnit");
    assert!(frames[2]["result"]["known"].is_array());
    assert_eq!(frames[3]["error"]["code"], "unsupported-schema");
    assert_eq!(frames[4]["request_id"], "42");
    assert_eq!(frames[4]["error"]["code"], "invalid-json");
    assert_eq!(frames[5]["error"]["code"], "invalid-json");
}

#[test]
fn external_query_client_applies_a_multi_file_rename() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let directory = TemporaryDirectory::new("external-rename");
    let app = directory.path().join("app");
    let child_directory = app.join("child");
    fs::create_dir_all(&child_directory).unwrap();
    let manifest_path = directory.path().join("package.toml");
    let main_path = app.join("main.trn");
    let child_path = child_directory.join("child.trn");
    let request_path = directory.path().join("rename.json");
    let manifest = "package = \"external-rename\"\n[namespaces]\napp = \"app\"\n";
    let main = "namespace app\n\npublic function answer int;\n    return 1\n";
    let child = "namespace app/child\n\nfunction use-answer int;\n    return answer;\n";
    fs::write(&manifest_path, manifest).unwrap();
    fs::write(&main_path, main).unwrap();
    fs::write(&child_path, child).unwrap();
    let main_uri = format!("file://{}", main_path.display());
    let child_uri = format!("file://{}", child_path.display());
    let manifest_uri = format!("file://{}", manifest_path.display());
    let requests = serde_json::json!([
        {
            "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
            "request_id": "open-rename",
            "operation": "open-snapshot",
            "sources": [
                {"uri": main_uri, "text": main},
                {"uri": child_uri, "text": child}
            ],
            "manifest": {"uri": manifest_uri, "text": manifest},
            "options": {"semantic": true}
        },
        {
            "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
            "request_id": "propose-rename",
            "operation": "propose-rename",
            "snapshot_id": "$last",
            "uri": child_uri,
            "offset": child.rfind("answer").unwrap(),
            "new_name": "computed-answer"
        },
        {
            "schema_version": terrane_compiler::tooling::SCHEMA_VERSION,
            "request_id": "apply-rename",
            "operation": "apply-edits",
            "proposal_id": "$last-proposal"
        }
    ]);
    fs::write(&request_path, requests.to_string()).unwrap();
    let output = Command::new(binary)
        .args(["query", "--request"])
        .arg(&request_path)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let frames = String::from_utf8(output.stdout).unwrap();
    assert_eq!(frames.lines().count(), 3);
    assert!(
        fs::read_to_string(main_path)
            .unwrap()
            .contains("function computed-answer int")
    );
    assert!(
        fs::read_to_string(child_path)
            .unwrap()
            .contains("return computed-answer;")
    );
}

#[test]
fn source_formatter_checks_then_applies_an_idempotent_edit() {
    let binary = env!("CARGO_BIN_EXE_terrane");
    let directory = TemporaryDirectory::new("formatter");
    fs::create_dir_all(directory.path()).unwrap();
    let source = directory.path().join("format.trn");
    fs::write(&source, "function main;   \r\n    value int = 1   \r\n").unwrap();

    let check = Command::new(binary)
        .args(["fmt", "--check"])
        .arg(&source)
        .output()
        .unwrap();
    assert_eq!(check.status.code(), Some(1));
    assert_eq!(
        String::from_utf8(check.stdout).unwrap().trim(),
        source.display().to_string()
    );
    let apply = Command::new(binary)
        .arg("fmt")
        .arg(&source)
        .output()
        .unwrap();
    assert!(apply.status.success(), "{apply:?}");
    assert_eq!(
        fs::read_to_string(&source).unwrap(),
        "function main;\r\n    value int = 1\r\n"
    );
    let clean = Command::new(binary)
        .args(["fmt", "--check"])
        .arg(&source)
        .output()
        .unwrap();
    assert!(clean.status.success(), "{clean:?}");
    assert!(clean.stdout.is_empty());
}
