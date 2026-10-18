use std::fs;
use std::io::{Read as _, Write as _};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(target_os = "linux")]
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(target_os = "linux")]
use std::time::{Duration, Instant};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct TempPackage(PathBuf);

impl TempPackage {
    fn new() -> Self {
        let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "terrane-cli-package-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        fs::write(
            path.join("package.toml"),
            "package = \"cli-package\"\nprelude = false\n[namespaces]\n\"cli/app\" = \"app\"\n\"cli/support\" = \"support\"\n",
        )
        .unwrap();
        fs::create_dir_all(path.join("app")).unwrap();
        fs::create_dir_all(path.join("support")).unwrap();
        fs::write(
            path.join("app/main.trn"),
            concat!(
                "namespace cli/app\n",
                "from /core/output import print\n",
                "function main;\n",
                "  print; 'manifest CLI'\n",
            ),
        )
        .unwrap();
        fs::write(
            path.join("support/support.trn"),
            "namespace cli/support\npublic constant value = 1\n",
        )
        .unwrap();
        Self(path)
    }
}

impl Drop for TempPackage {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[cfg(target_os = "linux")]
fn direct_children(process: u32) -> Vec<u32> {
    fs::read_to_string(format!("/proc/{process}/task/{process}/children"))
        .unwrap_or_default()
        .split_whitespace()
        .filter_map(|child| child.parse().ok())
        .collect()
}

#[cfg(target_os = "linux")]
fn wait_for_test_runner(process: u32, timeout: Duration) -> Option<u32> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        for child in direct_children(process) {
            let command_line = fs::read(format!("/proc/{child}/cmdline")).unwrap_or_default();
            if command_line
                .split(|byte| *byte == 0)
                .nth(1)
                .is_some_and(|argument| argument == b"0")
            {
                return Some(child);
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    None
}

#[test]
fn declaration_interface_export_survives_removal_of_original_source() {
    let package = TempPackage::new();
    fs::write(package.0.join("app/main.trn"), "namespace cli/app\n/// Public command documentation.\npublic function command int; count int = 7\n  return count\n").unwrap();
    let interface_path = package.0.join("library.interface.json");
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("declaration-interface")
        .arg(&package.0)
        .arg("--output")
        .arg(&interface_path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    fs::remove_dir_all(package.0.join("app")).unwrap();
    let interface = terrane_compiler::DeclarationInterface::from_json(
        &fs::read_to_string(interface_path).unwrap(),
    )
    .unwrap();
    let declaration = interface
        .declarations
        .iter()
        .find(|record| record.identity == "/cli/app::command")
        .unwrap();
    assert_eq!(
        declaration.documentation.as_deref(),
        Some("Public command documentation.")
    );
    assert_eq!(
        declaration.signature.as_ref().unwrap()["parameters"][0]["default"]["value"],
        "7"
    );
    assert!(
        package
            .0
            .join(".trn/terrane-projection.generated.trn")
            .is_file()
    );
    assert!(!package.0.join("terrane-projection.generated.trn").exists());
}

#[test]
fn implicit_source_does_not_reconcile_manifest_consumer_outputs() {
    let package = TempPackage::new();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../");
    let consumer = root.join("tests/fixtures/selected-function-consumer.py");
    fs::write(
        package.0.join("package.toml"),
        format!(
            "package = \"cli-package\"\n[namespaces]\n\"cli/app\" = \"app\"\n[consumers.cli]\ncommand = \"python3\"\nargs = [{consumer:?}]\ndeclarations = [\"/cli/app::command\"]\n"
        ),
    )
    .unwrap();
    fs::write(
        package.0.join("app/main.trn"),
        "namespace cli/app\nfunction command int;\n  return 0\nfunction main;\n  return\n",
    )
    .unwrap();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    let generated_test = package.0.join("tests/unit/generated.trn");
    fs::write(
        &generated_test,
        "namespace cli/app\nfrom /generated/consumer import run-command\nfunction test-generated;\n  result = run-command;\n",
    )
    .unwrap();
    let listed = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--list"])
        .arg(&package.0)
        .output()
        .unwrap();
    assert!(
        listed.status.success(),
        "{}",
        String::from_utf8_lossy(&listed.stderr)
    );
    assert!(
        String::from_utf8(listed.stdout)
            .unwrap()
            .contains("/cli/app::test-generated")
    );
    fs::remove_file(generated_test).unwrap();

    let invoke = |path: &Path| {
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg("check")
            .arg(path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    invoke(&package.0.join("package.toml"));
    let generated_dir = package.0.join(".trn/generated/cli");
    let generated = fs::read_dir(&generated_dir)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let contents = fs::read(&generated).unwrap();
    let scratch = package.0.join("scratch.trn");
    fs::write(&scratch, "namespace scratch\nfunction main;\n").unwrap();
    invoke(&scratch);
    assert_eq!(fs::read(&generated).unwrap(), contents);

    fs::write(
        package.0.join("package.toml"),
        "package = \"cli-package\"\n[namespaces]\n\"cli/app\" = \"app\"\n",
    )
    .unwrap();
    invoke(&package.0.join("package.toml"));
    assert!(!generated_dir.exists());
}

#[test]
fn projected_scoped_macro_token_mismatch_is_rejected_by_check() {
    let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    // Contained native Cargo commands hide /tmp outside their bound workspaces.
    let package = TempPackage(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target")
            .join(format!(
                "native-macro-rejection-{}-{serial}",
                std::process::id()
            )),
    );
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/native-macro-rejection");
    for relative in [
        "package.toml",
        "terrane-dependencies.lock",
        ".cargo/config.toml",
        "src/main.trn",
        "fixture-registry/terrane-scoped-rejection-witness-0.1.0/Cargo.toml",
        "fixture-registry/terrane-scoped-rejection-witness-0.1.0/src/lib.rs",
    ] {
        let destination = package.0.join(relative);
        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        fs::copy(fixture.join(relative), destination).unwrap();
    }
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("check")
        .arg(&package.0)
        .current_dir(&package.0)
        .output()
        .unwrap();
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(5), "{diagnostic}");
    assert!(diagnostic.contains("error[S9003]"), "{diagnostic}");
    assert!(diagnostic.contains("invalid format string"), "{diagnostic}");

    let source = package.0.join("src/main.trn");
    let valid = fs::read_to_string(&source)
        .unwrap()
        .replace("broken {", "valid {}");
    fs::write(source, valid).unwrap();
    let accepted = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("run")
        .arg(&package.0)
        .output()
        .unwrap();
    assert!(accepted.status.success(), "{accepted:?}");
    assert_eq!(accepted.stdout, b"20\n");
}

#[test]
fn manifest_file_and_package_directory_use_the_shared_cli_pipeline() {
    let package = TempPackage::new();
    let executable = env!("CARGO_BIN_EXE_terrane");

    let rust = Command::new(executable)
        .args(["rust", package.0.join("package.toml").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        rust.status.success(),
        "{}",
        String::from_utf8_lossy(&rust.stderr)
    );
    let generated = String::from_utf8(rust.stdout).unwrap();
    assert!(generated.contains("// Source: app/main.trn"));
    assert!(generated.contains("// Namespace: cli/app"));

    let run = Command::new(executable)
        .args(["run", package.0.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(String::from_utf8(run.stdout).unwrap(), "manifest CLI\n");

    let build = Command::new(executable)
        .args(["build", package.0.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let executable_path = PathBuf::from(String::from_utf8(build.stdout).unwrap().trim());
    let build_root = executable_path
        .ancestors()
        .find(|path| path.file_name().is_some_and(|name| name == ".trn"))
        .unwrap();
    assert_eq!(build_root.parent(), Some(package.0.as_path()));

    let relative_manifest_build = Command::new(executable)
        .current_dir(&package.0)
        .args(["build", "package.toml"])
        .output()
        .unwrap();
    assert!(
        relative_manifest_build.status.success(),
        "{}",
        String::from_utf8_lossy(&relative_manifest_build.stderr)
    );
    let generated_project = fs::read_dir(build_root.join("build"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let metadata = fs::read_to_string(generated_project.join("terrane-build.toml")).unwrap();
    assert!(metadata.contains("path = \"app/main.trn\""));
    assert!(metadata.contains("path = \"support/support.trn\""));
    assert_eq!(
        fs::read(generated_project.join("support/terrane-int-support/src/lib.rs")).unwrap(),
        fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../terrane-int-support/src/lib.rs")
        )
        .unwrap()
    );
    fs::remove_dir_all(build_root.join("cache/target")).unwrap();
    let cached_build = Command::new(executable)
        .args(["build", package.0.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(cached_build.status.success());
    assert_eq!(
        PathBuf::from(String::from_utf8(cached_build.stdout).unwrap().trim()),
        executable_path
    );
}

#[test]
fn dynamic_library_builds_warning_free_and_rejects_execution_commands() {
    let package = TempPackage::new();
    fs::write(
        package.0.join("package.toml"),
        "package = \"cli-library\"\nartifact = \"dynamic-library\"\nprelude = false\n[namespaces]\n\"cli/app\" = \"app\"\n\"cli/support\" = \"support\"\n",
    )
    .unwrap();
    let binary = env!("CARGO_BIN_EXE_terrane");
    let build = Command::new(binary)
        .arg("build")
        .arg(&package.0)
        .env("RUSTFLAGS", "-D warnings")
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let artifact = PathBuf::from(String::from_utf8(build.stdout).unwrap().trim());
    assert!(artifact.is_file(), "{}", artifact.display());
    assert_eq!(
        artifact
            .extension()
            .and_then(|extension| extension.to_str()),
        Some(std::env::consts::DLL_EXTENSION)
    );

    for command in [Some("run"), Some("debug"), None] {
        let mut invocation = Command::new(binary);
        if let Some(command) = command {
            invocation.arg(command);
        }
        let input = command.map_or_else(|| package.0.join("package.toml"), |_| package.0.clone());
        let output = invocation.arg(input).output().unwrap();
        assert_eq!(output.status.code(), Some(2), "{command:?}: {output:?}");
        assert_eq!(output.stdout, b"");
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("`run`, `debug`, and `profile record` require an executable package"),
            "{command:?}: {output:?}"
        );
    }
    let profile = Command::new(binary)
        .args(["profile", "record", "--cpu"])
        .arg(package.0.join("package.toml"))
        .output()
        .unwrap();
    assert_eq!(profile.status.code(), Some(2), "{profile:?}");
    assert!(
        String::from_utf8_lossy(&profile.stderr)
            .contains("`run`, `debug`, and `profile record` require an executable package")
    );
}

#[cfg(any(unix, all(target_os = "windows", target_env = "msvc")))]
#[test]
fn cached_dynamic_library_links_and_loads_without_shared_build_outputs() {
    let package = TempPackage::new();
    fs::write(
        package.0.join("package.toml"),
        "package = \"cached-library\"\nartifact = \"dynamic-library\"\nprelude = false\n[namespaces]\n\"cli/app\" = \"app\"\n[rust-modules]\nexports = \"exports.rs\"\n",
    )
    .unwrap();
    fs::write(
        package.0.join("exports.rs"),
        "#![allow(unsafe_code)]\n#[unsafe(no_mangle)]\npub extern \"C\" fn terrane_cache_answer() -> i32 { 42 }\n",
    )
    .unwrap();
    let build = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("build")
        .arg(&package.0)
        .output()
        .unwrap();
    assert!(build.status.success(), "{build:?}");
    let artifact = PathBuf::from(String::from_utf8(build.stdout).unwrap().trim());
    let slot = artifact.parent().unwrap();
    // A host must load solely from the public slot, not accidentally from Cargo's outputs.
    fs::remove_dir_all(package.0.join(".trn/cache/target")).unwrap();
    let source = package.0.join("host.rs");
    fs::write(
        &source,
        "unsafe extern \"C\" { fn terrane_cache_answer() -> i32; }\nfn main() { println!(\"{}\", unsafe { terrane_cache_answer() }); }\n",
    )
    .unwrap();
    let host = slot.join(format!("host{}", std::env::consts::EXE_SUFFIX));
    let mut compiler = Command::new("rustc");
    compiler
        .arg(&source)
        .args(["--edition=2024", "-Dwarnings"])
        .arg("-L")
        .arg(format!("native={}", slot.display()))
        .arg("-o")
        .arg(&host);
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    compiler.args(["-l", "dylib:+verbatim=terrane_program_application.dll.lib"]);
    #[cfg(unix)]
    compiler.args(["-l", "dylib=terrane_program_application"]);
    #[cfg(unix)]
    compiler
        .arg("-C")
        .arg(format!("link-arg=-Wl,-rpath,{}", slot.display()));
    let linked = compiler.output().unwrap();
    assert!(linked.status.success(), "{linked:?}");
    let loaded = Command::new(&host).output().unwrap();
    assert!(loaded.status.success(), "{loaded:?}");
    assert_eq!(String::from_utf8(loaded.stdout).unwrap().trim(), "42");
}

#[test]
fn projected_http_client_runs_against_a_loopback_server() {
    let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let package = TempPackage(root.join("target").join(format!(
        "projected-http-client-{}-{serial}",
        std::process::id()
    )));
    let fixture = root.join("tests/fixtures/projected-http-client");
    for relative in [
        "package.toml",
        ".cargo/config.toml",
        "fixture-registry/terrane-http-client-witness-0.1.0/Cargo.toml",
        "fixture-registry/terrane-http-client-witness-0.1.0/src/lib.rs",
    ] {
        let destination = package.0.join(relative);
        fs::create_dir_all(destination.parent().unwrap()).unwrap();
        fs::copy(fixture.join(relative), destination).unwrap();
    }
    fs::create_dir_all(package.0.join("src")).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    fs::write(
        package.0.join("src/main.trn"),
        format!(
            concat!(
                "namespace app\n",
                "from /deps/witness import get\n",
                "function main;\n",
                "    response = get; >{}\n",
                "    body string = response.text;\n",
                "    print; body\n",
            ),
            address
        ),
    )
    .unwrap();
    let server = std::thread::spawn(move || {
        let (mut connection, _) = listener.accept().unwrap();
        let mut request = [0_u8; 4096];
        let mut received = 0;
        while !request[..received].ends_with(b"\r\n\r\n") {
            let count = connection.read(&mut request[received..]).unwrap();
            assert!(
                count > 0,
                "client closed before sending complete request headers"
            );
            received += count;
        }
        assert!(request[..received].starts_with(b"GET / HTTP/1.1\r\n"));
        connection
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 21\r\nConnection: close\r\n\r\nprojected dependency\n")
            .unwrap();
    });
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["run", package.0.join("package.toml").to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    server.join().unwrap();
    assert_eq!(output.stdout, b"projected dependency\n\n");
}

const DETERMINISTIC_TEST_CASES: &str = concat!(
    "namespace cli/app\n",
    "from /core/errors import throwable\n",
    "from /core/process import exit, make-exit-status, native-text-value\n",
    "from /core/output import print\n",
    "from /core/streams import stderr\n",
    "from /core/testing import assert, assert-equal-int, assert-equal-string, fail, skip, test-arguments, test-deadline, test-failure, test-skip\n",
    "function test-pass none throws throwable;\n",
    "    supplied = test-arguments;\n",
    "    assert-equal-int; supplied.length, 1\n",
    "    first = native-text-value; supplied[0]\n",
    "    if first != none\n",
    "        assert-equal-string; first, 'controlled'\n",
    "    else\n",
    "        assert; false\n",
    "    deadline = test-deadline;\n",
    "    assert-equal-int; deadline.total-nanoseconds, 250000000\n",
    "    return none\n",
    "function test-fail none throws test-failure;\n",
    "    print; 'captured output'\n",
    "    errors = stderr;\n",
    "    written = errors.write-all; b'captured error\\n'\n",
    "    errors.close;\n",
    "    if written.failed\n",
    "        fail; written.message\n",
    "    assert-equal-int; 3, 4\n    return none\n",
    "function test-exit;\n",
    "    exit; (make-exit-status; 7)\n",
    "function test-ignored none throws test-skip;\n",
    "    skip; 'not available'\n    return none\n",
);

#[test]
fn native_test_command_reports_isolated_cases_deterministically_from_relative_paths() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("tests/unit/cases.trn"),
        DETERMINISTIC_TEST_CASES,
    )
    .unwrap();
    let report = package.0.join("test-report.json");
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .current_dir(package.0.parent().unwrap())
        .arg("test")
        .args([
            "--jobs",
            "2",
            "--timeout",
            "250ms",
            "--argument",
            "controlled",
            "--report",
        ])
        .arg(&report)
        .arg(package.0.file_name().unwrap())
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.find("/cli/app::test-pass").unwrap()
            < stdout.find("/cli/app::test-ignored").unwrap()
    );
    let report: serde_json::Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_eq!(report["schema_version"], "1.2.0");
    assert_eq!(report["run"]["status"], "completed");
    assert_eq!(report["cases"][0]["identity"], "/cli/app::test-pass");
    assert!(report["cases"][0]["stdout"].is_array());
    assert_eq!(report["cases"][1]["status"], "failed");
    assert_eq!(report["cases"][1]["cause"]["kind"], "assertion");
    assert_eq!(
        report["cases"][1]["cause"]["descriptor"],
        "/core/testing::test-failure"
    );
    assert_eq!(
        report["cases"][1]["cause"]["details"],
        serde_json::json!(["actual: 3", "expected: 4"])
    );
    assert!(
        report["cases"][1]["cause"]["source_frames"]
            .as_array()
            .unwrap()
            .last()
            .and_then(serde_json::Value::as_str)
            .is_some_and(|frame| frame.starts_with("/cli/app::test-fail (tests/unit/cases.trn:"))
    );
    assert_eq!(
        report["cases"][1]["stdout"],
        serde_json::json!(b"captured output\n")
    );
    assert_eq!(
        report["cases"][1]["stderr"],
        serde_json::json!(b"captured error\n")
    );
    assert_eq!(
        report["cases"][3]["cause"]["descriptor"],
        "/core/testing::test-skip"
    );
    assert_eq!(report["cases"][3]["cause"]["message"], "not available");
    assert_eq!(report["cases"][2]["status"], "failed");
    assert_eq!(report["cases"][3]["status"], "skipped");

    let listed = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--list", "--tier", "unit", "--filter", "pass"])
        .arg(&package.0)
        .output()
        .unwrap();
    assert!(listed.status.success());
    assert_eq!(
        String::from_utf8(listed.stdout).unwrap(),
        "/cli/app::test-pass [unit]\n"
    );

    assert_explicit_selectors(&package.0);

    let filtered = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--filter", "ignored"])
        .arg(&package.0)
        .output()
        .unwrap();
    assert!(filtered.status.success());
    assert!(
        String::from_utf8(filtered.stdout)
            .unwrap()
            .contains("skipped /cli/app::test-ignored"),
    );
}

fn assert_explicit_selectors(package: &Path) {
    for (option, pattern) in [
        ("--exact", "/cli/app::test-pass"),
        ("--glob", "*/app::test-p?ss"),
        ("--regex", "::test-p[a-z]+$"),
    ] {
        let selected = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg("test")
            .args(["--list", option, pattern])
            .arg(package)
            .output()
            .unwrap();
        assert!(selected.status.success(), "{option}");
        assert_eq!(
            String::from_utf8(selected.stdout).unwrap(),
            "/cli/app::test-pass [unit]\n",
            "{option}"
        );
    }
}

#[test]
fn tracked_native_testing_package_covers_all_tiers() {
    let package = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/native-testing");
    let listed = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .arg("--list")
        .arg(&package)
        .output()
        .unwrap();
    assert!(
        listed.status.success(),
        "{}",
        String::from_utf8_lossy(&listed.stderr)
    );
    assert_eq!(
        String::from_utf8(listed.stdout).unwrap(),
        concat!(
            "/sample::test-assertions-and-private-access [unit]\n",
            "/sample::test-custom-value-comparison [unit]\n",
            "/sample::test-controlled-async-time [unit]\n",
            "/sample::test-declared-throw-without-throw [unit]\n",
            "/sample::test-declared-throw-with-implicit-return [unit]\n",
            "/sample-integration::test-public-answer [integration]\n",
            "/sample-end-to-end::test-production-application [end-to-end]\n",
        )
    );

    let report_directory = TempPackage::new();
    let report = report_directory.0.join("tracked-report.json");
    let executed = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--jobs", "3", "--report"])
        .arg(&report)
        .arg(&package)
        .output()
        .unwrap();
    assert!(
        executed.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&executed.stdout),
        String::from_utf8_lossy(&executed.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_eq!(report["schema_version"], "1.2.0");
    assert_eq!(
        report["run"]["tiers"],
        serde_json::json!(["unit", "integration", "end-to-end"])
    );
    assert!(report["run"]["timeout_milliseconds"].is_u64());
    assert_eq!(report["compilation"].as_array().unwrap().len(), 3);
    assert!(
        report["compilation"]
            .as_array()
            .unwrap()
            .iter()
            .all(|tier| tier["status"] == "compiled")
    );
    assert_eq!(report["cases"].as_array().unwrap().len(), 7);
    assert!(
        report["cases"]
            .as_array()
            .unwrap()
            .iter()
            .all(|case| case["status"] == "passed")
    );
}

#[test]
fn native_test_listing_and_empty_selection_do_not_build_runners() {
    for arguments in [vec!["--list"], vec!["--filter", "does-not-exist"]] {
        let package = TempPackage::new();
        fs::create_dir_all(package.0.join("tests/unit")).unwrap();
        fs::write(
            package.0.join("tests/unit/case.trn"),
            "namespace cli/app\nfunction test-present;\n",
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg("test")
            .args(arguments)
            .arg(&package.0)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!package.0.join(".trn/build").exists());
    }
}

#[test]
fn end_to_end_artifact_passes_inputs_and_drains_large_output() {
    let package = TempPackage::new();
    fs::write(
        package.0.join("package.toml"),
        "package = \"fixture-pipes\"\nprelude = false\n[profile]\nname = \"testable\"\ncapabilities = [\"process\"]\n[namespaces]\n\"cli/app\" = \"app\"\n",
    )
    .unwrap();
    fs::write(
        package.0.join("app/main.trn"),
        concat!(
            "namespace cli/app\n",
            "from /core/output import print\n",
            "from /core/process import arguments, environment\n",
            "function main;\n",
            "    print; (arguments;).length\n",
            "    print; (environment;).length\n",
            "    rust\n",
            "        use std::io::Write as _;\n",
            "        let chunk = [b'x'; 8192];\n",
            "        let mut output = std::io::stdout().lock();\n",
            "        for _ in 0..128 { output.write_all(&chunk).unwrap(); }\n",
        ),
    )
    .unwrap();
    fs::create_dir_all(package.0.join("tests/end-to-end")).unwrap();
    fs::write(
        package.0.join("tests/end-to-end/application.trn"),
        concat!(
            "namespace cli/end-to-end\n",
            "from /core/process import environment-pair, native-text\n",
            "from /core/errors import throwable\n",
            "from /core/testing import application-artifact, assert, assert-equal-int, deny, fail\n",
            "from /core/testing/process import process-fixture, run-process\n",
            "function test-application none throws throwable;\n",
            "    artifact = application-artifact;\n",
            "    if artifact != none\n",
            "        fixture = instance process-fixture; artifact\n",
            "        fixture.arguments.append; (native-text; 'argument')\n",
            "        fixture.environment.append; (environment-pair; (native-text; 'FIXTURE'), (native-text; 'controlled'))\n",
            "        result = run-process; fixture\n",
            "        assert-equal-int; result.exit-code, 0\n",
            "        assert-equal-int; result.stdout.length, 1048576\n",
            "        assert; result.stdout[0] == 49 and result.stdout[1] == 10 and result.stdout[2] == 49 and result.stdout[3] == 10\n",
            "        assert; result.stdout-truncated\n",
            "        deny; result.stderr-truncated\n",
            "    else\n",
            "        fail; 'missing application artifact'\n",
            "    return none\n",
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--filter", "test-application", "--timeout", "5s"])
        .arg(&package.0)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8(output.stdout).unwrap().contains(
        "1 passed; 0 skipped; 0 failed; 0 timed out; 0 crashed; 0 infrastructure failed"
    ));
}

#[test]
fn native_test_timeout_terminates_the_isolated_process() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("tests/unit/timeout.trn"),
        "namespace cli/app\nfunction test-timeout;\n    while true\n        continue\n",
    )
    .unwrap();
    let report = package.0.join("timeout-report.json");

    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--timeout", "20ms", "--report"])
        .arg(&report)
        .arg(&package.0)
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8(output.stdout)
            .unwrap()
            .contains("timed-out /cli/app::test-timeout")
    );
    let report: serde_json::Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_eq!(report["cases"][0]["status"], "timed-out");
}

#[cfg(target_os = "linux")]
#[test]
fn native_test_signal_termination_reports_a_crashed_process() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("tests/unit/crash.trn"),
        "namespace cli/app\nfunction test-crash;\n    while true\n        continue\n",
    )
    .unwrap();
    let report = package.0.join("crash-report.json");

    let mut command = Command::new(env!("CARGO_BIN_EXE_terrane"));
    command
        .arg("test")
        .args(["--jobs", "1", "--timeout", "60s", "--report"])
        .arg(&report)
        .arg(&package.0)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut cli = command.spawn().unwrap();
    let runner = wait_for_test_runner(cli.id(), Duration::from_secs(180));
    let Some(runner) = runner else {
        let _ = cli.kill();
        let output = cli.wait_with_output().unwrap();
        panic!(
            "native test runner did not start\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    };
    let kill = Command::new("kill")
        .args(["-KILL", &runner.to_string()])
        .status()
        .expect("invoke platform signal command");
    if !kill.success() {
        let _ = cli.kill();
        let _ = cli.wait();
        panic!("platform signal command failed with {kill}");
    }
    let output = cli.wait_with_output().unwrap();

    assert_eq!(
        output.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("crashed /cli/app::test-crash"), "{stdout}");
    assert!(
        stdout.contains(
            "0 passed; 0 skipped; 0 failed; 0 timed out; 1 crashed; 0 infrastructure failed"
        ),
        "{stdout}"
    );
    let report: serde_json::Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_eq!(report["cases"][0]["status"], "crashed");
    assert_eq!(report["cases"][0]["cause"]["kind"], "crash");
    assert_eq!(
        report["cases"][0]["cause"]["message"],
        "test process terminated without an exit code"
    );
}

#[test]
fn native_test_compile_failures_write_structured_reports() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("src")).unwrap();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::create_dir_all(package.0.join("tests/integration")).unwrap();
    fs::write(
        package.0.join("package.toml"),
        "package = \"compile-report\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    )
    .unwrap();
    fs::write(
        package.0.join("src/library.trn"),
        "namespace app\npublic constant answer = 42\n",
    )
    .unwrap();
    fs::write(
        package.0.join("tests/unit/case.trn"),
        "namespace app\nfunction test-broken;\n  missing-name\n",
    )
    .unwrap();
    fs::write(
        package.0.join("tests/integration/case.trn"),
        "namespace app-tests\nfrom /app import answer\nfunction test-valid;\n  answer\n",
    )
    .unwrap();
    let report = package.0.join("report.json");

    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--report", report.to_str().unwrap()])
        .arg(&package.0)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(3));
    let report: serde_json::Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_eq!(report["schema_version"], "1.2.0");
    assert_eq!(report["run"]["status"], "compile-failed");
    assert_eq!(report["discovery"]["status"], "compile-failed");
    assert_ne!(
        report["discovery"]["diagnostics"].as_array().unwrap(),
        &Vec::<serde_json::Value>::new()
    );
    assert_eq!(
        report["compilation"]
            .as_array()
            .unwrap()
            .iter()
            .map(|tier| tier["status"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["not-run", "not-run"]
    );
    assert_eq!(report["cases"].as_array().unwrap().len(), 0);
}

#[test]
fn process_fixture_adapter_failures_are_infrastructure_failures() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("src")).unwrap();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("package.toml"),
        "package = \"fixture-infrastructure\"\nprelude = false\n[profile]\nname = \"testable\"\ncapabilities = [\"process\"]\n[namespaces]\napp = \"src\"\n",
    )
    .unwrap();
    fs::write(
        package.0.join("src/library.trn"),
        "namespace app\npublic constant available = true\n",
    )
    .unwrap();
    fs::write(
        package.0.join("tests/unit/fixture.trn"),
        concat!(
            "namespace app\n",
            "from /core/errors import throwable\n",
            "from /core/testing/process import process-fixture, run-process\n",
            "function test-missing-program none throws throwable;\n",
            "    run-process; (instance process-fixture; '/definitely/missing/terrane-test-program')\n",
            "    return none\n",
        ),
    )
    .unwrap();
    let report = package.0.join("report.json");

    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--report", report.to_str().unwrap()])
        .arg(&package.0)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(5));
    let report: serde_json::Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_eq!(report["cases"][0]["status"], "infrastructure-failed");
    assert_eq!(report["cases"][0]["cause"]["kind"], "infrastructure");
    assert_eq!(
        report["cases"][0]["cause"]["descriptor"],
        "/core/testing::test-infrastructure-failure"
    );
}

#[test]
fn typed_assert_throws_rejects_the_wrong_descriptor() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("src")).unwrap();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("package.toml"),
        "package = \"typed-throws\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    )
    .unwrap();
    fs::write(
        package.0.join("src/library.trn"),
        "namespace app\npublic constant available = true\n",
    )
    .unwrap();
    fs::write(
        package.0.join("tests/unit/throws.trn"),
        concat!(
            "namespace app\n",
            "from /core/errors import throwable\n",
            "from /core/testing import assert-throws, skip, test-failure, test-skip\n",
            "function skips none throws test-skip;\n",
            "    skip; 'not the expected descriptor'\n",
            "    return none\n",
            "function test-wrong-descriptor none throws throwable;\n",
            "    assert-throws; skips, test-failure.identity\n",
            "    return none\n",
        ),
    )
    .unwrap();
    let report = package.0.join("report.json");

    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--report", report.to_str().unwrap()])
        .arg(&package.0)
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_eq!(report["cases"][0]["cause"]["kind"], "assertion");
    assert!(
        report["cases"][0]["cause"]["message"]
            .as_str()
            .unwrap()
            .contains("expected throwable descriptor")
    );
}

#[test]
fn native_test_discovery_warnings_are_consistent_and_deduplicated() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("src")).unwrap();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::create_dir_all(package.0.join("tests/integration")).unwrap();
    fs::write(
        package.0.join("package.toml"),
        "package = \"warning-discovery\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    )
    .unwrap();
    fs::write(
        package.0.join("src/library.trn"),
        "namespace app\nprivate constant secret int = 7\npublic constant answer int = 42\n",
    )
    .unwrap();
    fs::write(
        package.0.join("tests/unit/unit.trn"),
        "namespace app\nfunction test-unit;\n    answer\n",
    )
    .unwrap();
    fs::write(
        package.0.join("tests/integration/integration.trn"),
        "namespace app-tests\nfrom /app import answer\nfunction test-integration;\n    ignored int = answer\n",
    )
    .unwrap();

    for arguments in [
        vec!["--list", "--tier", "unit"],
        vec!["--filter", "missing"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg("test")
            .args(arguments)
            .arg(&package.0)
            .output()
            .unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert_eq!(stderr.matches("binding `secret` is never read").count(), 1);
        assert!(stderr.contains("binding `ignored` is never read"));
        assert!(!stderr.contains("function `test-"));
    }

    let report_path = package.0.join("report.json");
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--filter", "test-unit", "--report"])
        .arg(&report_path)
        .arg(&package.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(stderr.matches("binding `secret` is never read").count(), 1);
    assert!(stderr.contains("binding `ignored` is never read"));
    assert!(!stderr.contains("function `test-"));
    let report: serde_json::Value =
        serde_json::from_slice(&fs::read(report_path).unwrap()).unwrap();
    assert_eq!(
        report["compilation"]
            .as_array()
            .unwrap()
            .iter()
            .map(|tier| (
                tier["tier"].as_str().unwrap(),
                tier["status"].as_str().unwrap()
            ))
            .collect::<Vec<_>>(),
        [("unit", "compiled"), ("integration", "not-run")]
    );
}

#[test]
fn native_test_timeout_errors_name_the_invalid_argument() {
    for arguments in [
        vec!["test", "--timeout"],
        vec!["test", "--timeout", "abc"],
        vec!["test", "--timeout", "0"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .args(arguments)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("--timeout"));
    }
}

#[test]
fn stable_build_roles_invalidate_changed_sources_and_isolate_tests() {
    let package = TempPackage::new();
    let build = package.0.join(".trn/build");
    let legacy = build.join("0".repeat(64));
    fs::create_dir_all(&legacy).unwrap();
    fs::write(legacy.join("terrane-build.toml"), "version = 1\n").unwrap();
    let unrelated = build.join("notes");
    fs::create_dir_all(&unrelated).unwrap();
    fs::write(unrelated.join("keep.txt"), "user-owned").unwrap();
    let unmarked_hash = build.join("1".repeat(64));
    fs::create_dir_all(&unmarked_hash).unwrap();
    fs::write(unmarked_hash.join("keep.txt"), "not-a-generated-crate").unwrap();
    let invoke = |command| {
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg(command)
            .arg(&package.0)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    assert_eq!(invoke("run"), "manifest CLI\n");
    assert!(!legacy.exists());
    assert_eq!(
        fs::read_to_string(unrelated.join("keep.txt")).unwrap(),
        "user-owned"
    );
    assert_eq!(
        fs::read_to_string(unmarked_hash.join("keep.txt")).unwrap(),
        "not-a-generated-crate"
    );
    fs::remove_dir_all(unrelated).unwrap();
    fs::remove_dir_all(unmarked_hash).unwrap();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("tests/unit/smoke.trn"),
        "namespace cli/app\nfunction test-smoke;\n  return\n",
    )
    .unwrap();
    let tests = invoke("test");
    assert!(tests.contains("passed /cli/app::test-smoke"), "{tests}");
    assert_eq!(invoke("run"), "manifest CLI\n");
    let application = package.0.join(".trn/build/application");
    fs::write(
        application.join("src/obsolete.rs"),
        "compile_error!(\"obsolete source\");",
    )
    .unwrap();
    fs::write(
        package.0.join("app/main.trn"),
        "namespace cli/app\nfrom /core/output import print\nfunction main;\n  print; 'changed'\n",
    )
    .unwrap();
    assert_eq!(invoke("run"), "changed\n");
    assert!(!application.join("src/obsolete.rs").exists());
    assert_eq!(invoke("run"), "changed\n");
    let mut roles = fs::read_dir(package.0.join(".trn/build"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    roles.sort();
    assert_eq!(roles, ["application", "test-unit"]);
}

#[test]
fn changed_native_inputs_invalidate_check_stamps_and_remove_unused_platform_support() {
    let package = TempPackage::new();
    let mut manifest = fs::read_to_string(package.0.join("package.toml")).unwrap();
    manifest.push_str("\n[rust-modules]\nadapter = \"rust/adapter.rs\"\n");
    fs::write(package.0.join("package.toml"), manifest).unwrap();
    fs::create_dir_all(package.0.join("rust")).unwrap();
    let adapter = package.0.join("rust/adapter.rs");
    fs::write(&adapter, "pub const VALUE: u8 = 1;\n").unwrap();
    fs::write(
        package.0.join("app/main.trn"),
        "namespace cli/app\nfrom /core/process import arguments\nfunction main;\n  ignored = arguments;\n",
    )
    .unwrap();
    let invoke = |command| {
        Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg(command)
            .arg(&package.0)
            .output()
            .unwrap()
    };
    let checked = invoke("check");
    assert!(checked.status.success(), "{checked:?}");
    let application = package.0.join(".trn/build/application");
    let stamp = application.join("artifacts/check-success");
    assert!(stamp.is_file());
    for name in ["terrane-platform-support", "terrane-signal-support"] {
        assert!(application.join("support").join(name).is_dir());
    }
    fs::write(
        package.0.join("app/main.trn"),
        "namespace cli/app\nfrom /core/output import print\nfunction main;\n  print; 'without platform support'\n",
    )
    .unwrap();
    fs::write(
        &adapter,
        "compile_error!(\"changed authored Rust must be checked\");\n",
    )
    .unwrap();
    let rejected = invoke("check");
    assert!(!rejected.status.success(), "{rejected:?}");
    assert!(
        String::from_utf8_lossy(&rejected.stderr).contains("changed authored Rust must be checked"),
        "{rejected:?}"
    );
    assert!(!stamp.exists());
    for name in ["terrane-platform-support", "terrane-signal-support"] {
        assert!(!application.join("support").join(name).exists());
    }
    fs::write(&adapter, "pub const VALUE: u8 = 2;\n").unwrap();
    let repaired = invoke("check");
    assert!(repaired.status.success(), "{repaired:?}");
    assert!(stamp.is_file());
    let run = invoke("run");
    assert!(run.status.success(), "{run:?}");
    assert_eq!(run.stdout, b"without platform support\n");
}

#[test]
fn native_test_tiers_restore_their_own_binary_from_shared_cargo_cache() {
    let package = TempPackage::new();
    fs::write(
        package.0.join("package.toml"),
        "package = \"tier-cache\"\nprelude = false\n[namespaces]\nsample = \"src\"\n",
    )
    .unwrap();
    fs::create_dir_all(package.0.join("src")).unwrap();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::create_dir_all(package.0.join("tests/integration")).unwrap();
    fs::create_dir_all(package.0.join("tests/end-to-end")).unwrap();
    fs::write(
        package.0.join("src/application.trn"),
        "namespace sample\nfunction main;\n  return\n",
    )
    .unwrap();
    for (path, namespace, name) in [
        ("tests/unit/case.trn", "sample", "test-unit"),
        (
            "tests/integration/case.trn",
            "sample-integration",
            "test-integration",
        ),
        (
            "tests/end-to-end/case.trn",
            "sample-end-to-end",
            "test-end-to-end",
        ),
    ] {
        fs::write(
            package.0.join(path),
            format!("namespace {namespace}\nfunction {name};\n  return\n"),
        )
        .unwrap();
    }
    let invoke = || {
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg("test")
            .arg(&package.0)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
        let text = String::from_utf8_lossy(&output.stdout);
        for case in [
            "/sample::test-unit",
            "/sample-integration::test-integration",
            "/sample-end-to-end::test-end-to-end",
        ] {
            assert!(text.contains(&format!("passed {case}")), "{text}");
        }
    };
    invoke();
    fs::remove_dir_all(package.0.join(".trn/build/test-unit/artifacts")).unwrap();
    invoke();
}
