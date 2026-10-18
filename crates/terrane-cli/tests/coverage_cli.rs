#![cfg(target_os = "linux")]

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "terrane-coverage-cli-{}-{}",
            std::process::id(),
            NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(path.join("app")).unwrap();
        fs::create_dir_all(path.join("tests/unit")).unwrap();
        fs::write(
            path.join("package.toml"),
            concat!(
                "package = \"coverage-regression\"\nprelude = false\n",
                "[namespaces]\n\"coverage/app\" = \"app\"\n",
                "[rust-modules]\nwitness = \"witness.rs\"\n",
            ),
        )
        .unwrap();
        fs::write(
            path.join("witness.rs"),
            concat!(
                "#[inline(never)]\n",
                "pub fn coverage_witness_count() -> i64 {\n",
                "    42_i64\n}\n",
            ),
        )
        .unwrap();
        fs::write(
            path.join("app/main.trn"),
            concat!(
                "namespace coverage/app\nfrom /core/output import print\n",
                "function main;\n  answer int64 = rust\n",
                "    crate::witness::coverage_witness_count()\n  print; answer\n",
            ),
        )
        .unwrap();
        fs::write(
            path.join("tests/unit/case.trn"),
            concat!(
                "namespace coverage/app\nfrom /core/errors import throwable\n",
                "from /core/testing import assert\n",
                "function test-counter none throws throwable;\n  answer int64 = rust\n",
                "    crate::witness::coverage_witness_count()\n  assert; answer == 42\n",
                "  return none\n",
            ),
        )
        .unwrap();
        Self(path)
    }

    fn command(&self, verb: &str, capture: &Path) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_terrane"));
        command
            .arg(verb)
            .arg(&self.0)
            .env("TERRANE_COVERAGE_DIR", capture)
            .env("LLVM_PROFILE_FILE", capture.join("profiles/%m-%p.profraw"));
        command
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn successful(output: Output) -> Output {
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn llvm_tools() -> Option<(PathBuf, PathBuf)> {
    let version = successful(Command::new("rustc").arg("-vV").output().unwrap());
    let version = String::from_utf8(version.stdout).unwrap();
    let llvm = version
        .lines()
        .find_map(|line| line.strip_prefix("LLVM version: "))
        .unwrap();
    let host = version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .unwrap();
    let sysroot = successful(
        Command::new("rustc")
            .args(["--print", "sysroot"])
            .output()
            .unwrap(),
    );
    let bundled = Path::new(String::from_utf8(sysroot.stdout).unwrap().trim())
        .join("lib/rustlib")
        .join(host)
        .join("bin");
    let find = |name: &str| {
        [bundled.join(name), PathBuf::from(name)]
            .into_iter()
            .find(|path| {
                Command::new(path)
                    .arg("--version")
                    .output()
                    .is_ok_and(|output| {
                        output.status.success()
                            && String::from_utf8_lossy(&output.stdout)
                                .contains(&format!("LLVM version {llvm}"))
                    })
            })
    };
    let tools = find("llvm-profdata").zip(find("llvm-cov"));
    if tools.is_none() {
        eprintln!(
            "coverage runtime regression requires LLVM tools matching rustc; install llvm-tools-preview"
        );
    }
    tools
}

fn assert_witness_count(capture: &Path, tools: &(PathBuf, PathBuf)) {
    let mut merge = Command::new(&tools.0);
    merge.arg("merge");
    for entry in fs::read_dir(capture.join("profiles")).unwrap() {
        merge.arg(entry.unwrap().path());
    }
    let profdata = capture.join("execution.profdata");
    successful(merge.arg("-o").arg(&profdata).output().unwrap());
    let mut export = Command::new(&tools.1);
    export.arg("export");
    let mut objects = fs::read_dir(capture.join("objects")).unwrap();
    export.arg(objects.next().unwrap().unwrap().path());
    for entry in objects {
        export.arg("-object").arg(entry.unwrap().path());
    }
    export.arg(format!("-instr-profile={}", profdata.display()));
    let data: serde_json::Value =
        serde_json::from_slice(&successful(export.output().unwrap()).stdout).unwrap();
    let counts: Vec<u64> = data["data"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|unit| unit["functions"].as_array().unwrap())
        .filter(|function| {
            function["name"]
                .as_str()
                .unwrap()
                .contains("coverage_witness_count")
        })
        .map(|function| function["count"].as_u64().unwrap())
        .collect();
    assert_eq!(
        counts,
        [1],
        "the authored native function must execute exactly once"
    );
}

#[test]
fn native_coverage_records_execution_on_cache_hits_and_in_cleared_test_children() {
    let Some(tools) = llvm_tools() else {
        return;
    };
    let fixture = Fixture::new();
    let first = fixture.0.join("first-capture");
    let output = successful(fixture.command("run", &first).output().unwrap());
    assert_eq!(output.stdout, b"42\n");
    assert_witness_count(&first, &tools);

    let build_target = fixture.0.join(".trn/cache/target");
    fs::remove_dir_all(&build_target).unwrap();
    let restored = fixture.0.join("restored-capture");
    let output = successful(fixture.command("run", &restored).output().unwrap());
    assert_eq!(output.stdout, b"42\n");
    assert!(
        !build_target.exists(),
        "a cache hit must not rebuild the generated crate"
    );
    assert_witness_count(&restored, &tools);

    let native_test = fixture.0.join("test-capture");
    let report = fixture.0.join("test-report.json");
    successful(
        fixture
            .command("test", &native_test)
            .arg("--report")
            .arg(&report)
            .output()
            .unwrap(),
    );
    let report: serde_json::Value = serde_json::from_slice(&fs::read(report).unwrap()).unwrap();
    assert_eq!(report["cases"][0]["status"], "passed");
    assert_witness_count(&native_test, &tools);
}

#[test]
fn coverage_debugger_honors_per_invocation_rust_configuration() {
    let fixture = Fixture::new();
    fs::write(
        fixture.0.join("witness.rs"),
        concat!(
            "#![allow(unexpected_cfgs)]\n",
            "pub fn coverage_witness_count() -> i64 {\n",
            "    if cfg!(coverage_debugger_caller) { 42_i64 } else { 41_i64 }\n}\n",
        ),
    )
    .unwrap();
    let mut child = fixture
        .command("debug", &fixture.0.join("debug-capture"))
        .env("RUSTFLAGS", "--cfg coverage_debugger_caller")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"continue\n")
        .unwrap();
    let output = successful(child.wait_with_output().unwrap());
    assert_eq!(
        output.stdout, b"42\r\n",
        "debuggee must use this invocation's cfg, not wrapper defaults"
    );
}
