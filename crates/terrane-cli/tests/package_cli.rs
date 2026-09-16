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
    let entrypoint = fs::read_to_string(generated_project.join("src/main.rs")).unwrap();
    assert!(generated_project.join("src/main.support.rs").is_file());
    assert!(entrypoint.contains("include!(\"main.support.rs\");"));
    assert!(entrypoint.contains("// Source: app/main.trn"));
    assert!(entrypoint.contains("// Source: support/support.trn"));
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
        assert!(output.stdout.is_empty());
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

#[test]
fn projected_reqwest_runs_against_a_loopback_server() {
    let package = TempPackage::new();
    fs::write(
        package.0.join("package.toml"),
        concat!(
            "package = \"reqwest-loopback\"\n",
            "[namespaces]\n",
            "app = \"app\"\n",
            "[rust-dependencies.reqwest]\n",
            "version = \"=0.12.23\"\n",
            "features = [\"blocking\", \"rustls-tls-webpki-roots\"]\n",
            "default-features = false\n",
        ),
    )
    .unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    fs::write(
        package.0.join("app/main.trn"),
        format!(
            concat!(
                "namespace app\n",
                "from /deps/reqwest/blocking import get\n",
                "function main;\n",
                "    response = get; >http://{}/\n",
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
        let _ = connection.read(&mut request).unwrap();
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
    let generated = fs::read_dir(package.0.join(".trn/build"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let manifest = fs::read_to_string(generated.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("default-features = false"));
    assert!(manifest.contains("\"blocking\""));
    assert!(manifest.contains("\"rustls-tls-webpki-roots\""));
}

#[test]
fn representative_dependency_projection_matches_reviewed_semantics() {
    let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let package = TempPackage(std::env::temp_dir().join(format!(
        "terrane-cli-projection-{}-{serial}",
        std::process::id()
    )));
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/projection/representative-dependency-stack");
    copy_fixture(&fixture, &package.0);
    let reviewed = fs::read(package.0.join("terrane-projection.lock")).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["check", package.0.to_str().unwrap()])
        .env("RUSTFLAGS", "-D warnings")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let regenerated = fs::read(package.0.join("terrane-projection.lock")).unwrap();
    assert_eq!(
        stable_projection_history(&regenerated),
        stable_projection_history(&reviewed),
        "the dependency projection drifted from its reviewed semantic history"
    );
    assert_projection_history_member(&reviewed, "axum", "/deps/axum", "Router::new");
    assert_projection_history_member(&reviewed, "reqwest", "/deps/reqwest", "Response.status");
    assert_reviewed_classification(&package.0, &fixture);
}

fn stable_projection_history(history: &[u8]) -> serde_json::Value {
    let mut history = serde_json::from_slice::<serde_json::Value>(history).unwrap();
    let object = history.as_object_mut().unwrap();
    object.remove("cache_identity");
    object.remove("content_hash");
    history
}

fn assert_projection_history_member(
    history: &[u8],
    dependency_name: &str,
    namespace: &str,
    member_name: &str,
) {
    let history = serde_json::from_slice::<serde_json::Value>(history).unwrap();
    let dependency = history["dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .find(|dependency| dependency["name"] == dependency_name)
        .unwrap();
    assert!(
        dependency["members"]
            .as_array()
            .unwrap()
            .iter()
            .any(|member| { member[0] == namespace && member[1] == member_name }),
        "{dependency_name} history does not record `{namespace}::{member_name}`"
    );
}

fn assert_reviewed_classification(package: &Path, fixture: &Path) {
    let classification = fs::read_to_string(fixture.join("classification.toml"))
        .unwrap()
        .parse::<toml::Value>()
        .unwrap();
    assert_classification_context(&classification);
    let artifact = fs::read_dir(package.join(".trn/dependencies"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_stem()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("projection-"))
                && path
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        })
        .unwrap();
    let projection =
        serde_json::from_slice::<serde_json::Value>(&fs::read(artifact).unwrap()).unwrap();
    let projected_dependencies = projection["dependencies"].as_array().unwrap();
    for expected in classification["dependency"].as_array().unwrap() {
        let name = expected["name"].as_str().unwrap();
        let actual = projected_dependencies
            .iter()
            .find(|dependency| dependency["name"] == name)
            .unwrap();
        assert_eq!(actual["version"], expected["version"].as_str().unwrap());
        assert!(!expected["classification"].as_str().unwrap().is_empty());
        assert!(expected["required-gaps"].is_array());
        let projected_count =
            usize::try_from(expected["projected-count"].as_integer().unwrap()).unwrap();
        let declined_count =
            usize::try_from(expected["declined-count"].as_integer().unwrap()).unwrap();
        let instance_count =
            usize::try_from(expected["instance-count"].as_integer().unwrap()).unwrap();
        let static_count = usize::try_from(expected["static-count"].as_integer().unwrap()).unwrap();
        let items = actual["items"].as_array().unwrap();
        let (actual_instance_count, actual_static_count, members) = projected_surface(items);
        assert_eq!(
            items.len(),
            projected_count,
            "{name} projected count drifted"
        );
        assert_eq!(
            actual["declined"].as_array().unwrap().len(),
            declined_count,
            "{name} declined count drifted"
        );
        assert_eq!(
            actual_instance_count, instance_count,
            "{name} instance method count drifted"
        );
        assert_eq!(
            actual_static_count, static_count,
            "{name} static method count drifted"
        );
        for field in ["canonical-members", "instance-members", "static-members"] {
            for member in expected[field].as_array().unwrap() {
                let member = member.as_str().unwrap();
                assert!(
                    members.iter().any(|candidate| candidate == member),
                    "{name} classified member `{member}` drifted"
                );
            }
        }
        if let Some(declines) = expected
            .get("declined-members")
            .and_then(toml::Value::as_array)
        {
            for declined in declines {
                let rust_path = declined["rust-path"].as_str().unwrap();
                let reason = declined["reason"].as_str().unwrap();
                assert!(
                    actual["declined"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|item| { item["rust_path"] == rust_path && item["reason"] == reason }),
                    "{name} decline `{rust_path}` no longer records `{reason}`"
                );
            }
        }
    }
}

fn assert_classification_context(classification: &toml::Value) {
    assert_eq!(classification["format"].as_integer(), Some(2));
    assert_eq!(
        classification["projection-source"].as_str(),
        Some("local-rustdoc")
    );
    let cost = &classification["cost"];
    for field in ["cold-wall-seconds", "warm-wall-seconds"] {
        assert!(cost[field].as_float().is_some_and(|value| value > 0.0));
    }
    for field in ["generated-rust-lines", "generated-rust-bytes"] {
        assert!(cost[field].as_integer().is_some_and(|value| value > 0));
    }
    for field in ["observed-on", "host", "method"] {
        assert!(!cost[field].as_str().unwrap().is_empty());
    }
}

fn projected_surface(items: &[serde_json::Value]) -> (usize, usize, Vec<String>) {
    let mut members = Vec::new();
    let mut instance_count = 0;
    let mut static_count = 0;
    for item in items {
        let base = format!(
            "{}::{}",
            item["namespace"].as_str().unwrap(),
            item["name"].as_str().unwrap()
        );
        members.push(base.clone());
        if let Some(foreign) = item["kind"].get("ForeignType") {
            let methods = foreign["methods"].as_array().unwrap();
            let static_methods = foreign["static_methods"].as_array().unwrap();
            instance_count += methods.len();
            static_count += static_methods.len();
            members.extend(
                methods
                    .iter()
                    .map(|method| format!("{base}.{}", method["name"].as_str().unwrap())),
            );
            members.extend(
                static_methods
                    .iter()
                    .map(|method| format!("{base}::{}", method["name"].as_str().unwrap())),
            );
        }
    }
    (instance_count, static_count, members)
}

fn copy_fixture(source: &std::path::Path, destination: &std::path::Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        if name == ".trn" {
            continue;
        }
        let target = destination.join(&name);
        if entry.file_type().unwrap().is_dir() {
            copy_fixture(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

const DETERMINISTIC_TEST_CASES: &str = concat!(
    "namespace cli/app\n",
    "from /core/errors import throwable\n",
    "from /core/process import exit, make-exit-status\n",
    "from /core/output import print\n",
    "from /core/streams import stderr\n",
    "from /core/time import clock, duration\n",
    "from /core/testing import advance-time, assert, assert-equal-bool, assert-equal-bytes, assert-equal-float64, assert-equal-int, assert-equal-string, assert-near, assert-none-bool, assert-none-bytes, assert-none-float64, assert-none-int, assert-none-string, assert-not-equal-bool, assert-not-equal-bytes, assert-not-equal-float64, assert-not-equal-int, assert-not-equal-string, assert-present-bool, assert-present-bytes, assert-present-float64, assert-present-int, assert-present-string, assert-throws, deny, fail, skip, temporary-directory, test-failure, test-skip\n",
    "function expected-error none throws test-failure;\n",
    "    fail; 'expected'\n    return none\n",
    "function test-pass none throws test-failure;\n",
    "    assert; true\n",
    "    deny; false\n",
    "    assert-equal-int; 4, 4\n",
    "    assert-not-equal-int; 4, 5\n",
    "    assert-equal-string; 'a', 'a'\n",
    "    assert-not-equal-string; 'a', 'b'\n",
    "    assert-equal-bool; true, true\n",
    "    assert-not-equal-bool; true, false\n",
    "    assert-equal-float64; 1.5, 1.5\n",
    "    assert-not-equal-float64; 1.5, 2.5\n",
    "    assert-equal-bytes; b'a', b'a'\n",
    "    assert-not-equal-bytes; b'a', b'b'\n",
    "    assert-present-string; 'present'\n",
    "    assert-none-string; none\n",
    "    assert-near; 1.0, 1.1, 0.2\n",
    "    assert-throws; expected-error, test-failure.identity\n",
    "    assert-present-string; (temporary-directory;)\n",
    "    assert-present-int; 1\n",
    "    assert-none-int; none\n",
    "    assert-present-bool; true\n",
    "    assert-none-bool; none\n",
    "    assert-present-float64; 1.0\n",
    "    assert-none-float64; none\n",
    "    assert-present-bytes; b'a'\n",
    "    assert-none-bytes; none\n",
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
    "function test-controlled-time none throws throwable;\n",
    "    started = clock::monotonic;\n",
    "    advance-time; 25\n",
    "    later = clock::monotonic;\n",
    "    elapsed = started.duration-until; ref later\n",
    "    assert-equal-int; elapsed.nanoseconds, 25\n",
    "    return none\n",
    "async function test-async none throws throwable;\n",
    "    advance-time; 25\n",
    "    started = clock::monotonic;\n",
    "    interval = duration::nanoseconds; 25\n",
    "    ignored = await clock::sleep; interval\n",
    "    later = clock::monotonic;\n",
    "    elapsed = started.duration-until; ref later\n",
    "    assert-equal-int; elapsed.nanoseconds, 25\n",
    "    return none\n",
);

#[test]
fn native_test_command_reports_isolated_cases_deterministically() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("tests/unit/cases.trn"),
        DETERMINISTIC_TEST_CASES,
    )
    .unwrap();
    let report = package.0.join("test-report.json");
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--jobs", "2", "--report"])
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
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.find("/cli/app::test-pass").unwrap()
            < stdout.find("/cli/app::test-ignored").unwrap()
    );
    assert!(
        stdout.contains(
            "3 passed; 1 skipped; 2 failed; 0 timed out; 0 crashed; 0 infrastructure failed"
        ),
        "{stdout}\n{}",
        String::from_utf8_lossy(&output.stderr)
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
        report["cases"][1]["cause"]["message"],
        "integer values differ"
    );
    assert_eq!(
        report["cases"][1]["cause"]["details"],
        serde_json::json!(["actual: 3", "expected: 4"])
    );
    assert!(
        !report["cases"][1]["cause"]["source_frames"]
            .as_array()
            .unwrap()
            .is_empty()
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
    assert_eq!(report["cases"][4]["status"], "passed");
    assert_eq!(report["cases"][5]["status"], "passed");

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

#[test]
fn native_test_relative_package_paths_preserve_failure_results() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("tests/unit/failure.trn"),
        concat!(
            "namespace cli/app\n",
            "from /core/testing import assert-equal-int, test-failure\n",
            "function test-relative none throws test-failure;\n",
            "    assert-equal-int; 41, 42\n",
            "    return none\n",
        ),
    )
    .unwrap();
    let parent = package.0.parent().unwrap();
    let relative = package.0.file_name().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .current_dir(parent)
        .arg("test")
        .args(["--filter", "relative"])
        .arg(relative)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(
        output.status.code(),
        Some(1),
        "{stdout}\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        stdout.contains("failed /cli/app::test-relative"),
        "{stdout}"
    );
    assert!(!stdout.contains("infrastructure-failed"), "{stdout}");
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
fn end_to_end_tests_receive_the_built_application_artifact() {
    let package = TempPackage::new();
    fs::write(
        package.0.join("app/main.trn"),
        concat!(
            "namespace cli/app\n",
            "from /core/output import print\n",
            "from /core/process import arguments, environment\n",
            "function main;\n",
            "    received-arguments = arguments;\n",
            "    received-environment = environment;\n",
            "    print; received-arguments.length\n",
            "    print; received-environment.length\n",
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
            "from /core/testing import application-artifact, assert-equal-int, fail\n",
            "from /core/testing/process import process-fixture, run-process\n",
            "function test-application none throws throwable;\n",
            "    artifact = application-artifact;\n",
            "    if artifact != none\n",
            "        fixture = instance process-fixture; artifact\n",
            "        fixture.arguments.append; (native-text; 'argument')\n",
            "        fixture.environment.append; (environment-pair; (native-text; 'FIXTURE'), (native-text; 'controlled'))\n",
            "        fixture.standard-input = b'input'\n",
            "        result = run-process; fixture\n",
            "        assert-equal-int; result.exit-code, 0\n",
            "        assert-equal-int; result.stdout.length, 4\n",
            "    else\n",
            "        fail; 'missing application artifact'\n",
            "    return none\n",
        ),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--filter", "test-application"])
        .arg(&package.0)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
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
    assert!(
        !report["discovery"]["diagnostics"]
            .as_array()
            .unwrap()
            .is_empty()
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
fn native_test_context_exposes_deadline_and_controlled_arguments() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("src")).unwrap();
    fs::create_dir_all(package.0.join("tests/unit")).unwrap();
    fs::write(
        package.0.join("package.toml"),
        "package = \"test-context\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    )
    .unwrap();
    fs::write(
        package.0.join("src/library.trn"),
        "namespace app\npublic constant answer = 42\n",
    )
    .unwrap();
    fs::write(
        package.0.join("tests/unit/context.trn"),
        concat!(
            "namespace app\n",
            "from /core/errors import throwable\n",
            "from /core/process import native-text-value\n",
            "from /core/testing import assert, assert-equal-int, assert-equal-string, test-arguments, test-deadline\n",
            "function test-context none throws throwable;\n",
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
        ),
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("test")
        .args(["--timeout", "250ms", "--argument", "controlled"])
        .arg(&package.0)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8(output.stdout).unwrap().contains(
        "1 passed; 0 skipped; 0 failed; 0 timed out; 0 crashed; 0 infrastructure failed"
    ));
}

#[test]
fn process_fixture_drains_large_output_while_the_child_runs() {
    let package = TempPackage::new();
    fs::create_dir_all(package.0.join("app")).unwrap();
    fs::create_dir_all(package.0.join("tests/end-to-end")).unwrap();
    fs::write(
        package.0.join("package.toml"),
        "package = \"fixture-pipes\"\nprelude = false\n[profile]\nname = \"testable\"\ncapabilities = [\"process\"]\n[namespaces]\napp = \"app\"\n",
    )
    .unwrap();
    fs::write(
        package.0.join("app/main.trn"),
        concat!(
            "namespace app\n",
            "from /core/output import print\n",
            "function main;\n",
            "    index int = 0\n",
            "    while index < 200000\n",
            "        print; 'output'\n",
            "        index++\n",
        ),
    )
    .unwrap();
    fs::write(
        package.0.join("tests/end-to-end/pipes.trn"),
        concat!(
            "namespace fixture-pipes\n",
            "from /core/errors import throwable\n",
            "from /core/testing import application-artifact, assert, assert-equal-int, deny, fail\n",
            "from /core/testing/process import process-fixture, run-process\n",
            "function test-large-output none throws throwable;\n",
            "    artifact = application-artifact;\n",
            "    if artifact != none\n",
            "        result = run-process; (instance process-fixture; artifact)\n",
            "        assert-equal-int; result.stdout.length, 1048576\n",
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
        .args(["--timeout", "5s"])
        .arg(&package.0)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
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
        assert!(stderr.starts_with("error: "));
        assert!(stderr.contains("--timeout"));
        assert!(stderr.contains("usage: terrane"));
    }
}
