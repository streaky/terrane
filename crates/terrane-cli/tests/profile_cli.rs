#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);
static REAL_PROFILER_ACTIVE: AtomicBool = AtomicBool::new(false);

struct RealProfilerGuard;

impl RealProfilerGuard {
    fn acquire() -> Self {
        while REAL_PROFILER_ACTIVE
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            thread::sleep(Duration::from_millis(20));
        }
        Self
    }
}

impl Drop for RealProfilerGuard {
    fn drop(&mut self) {
        REAL_PROFILER_ACTIVE.store(false, Ordering::Release);
    }
}

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "terrane-profile-evidence-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(path.join("src")).unwrap();
        fs::write(
            path.join("package.toml"),
            "package = \"profile-evidence\"\n\n[namespaces]\nprofile = \"src\"\n",
        )
        .unwrap();
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn perf_available(directory: &Path) -> bool {
    let capture = directory.join("probe.data");
    let status = Command::new("perf")
        .args(["record", "-q", "-o"])
        .arg(&capture)
        .args(["-e", "cpu-clock:u", "--", "/usr/bin/true"])
        .status();
    let _ = fs::remove_file(capture);
    status.is_ok_and(|status| status.success())
}

fn workload(iterations: u64, exit: Option<i32>) -> String {
    let exit_import = if exit.is_some() {
        "from /core/process import exit, make-exit-status\n\n"
    } else {
        ""
    };
    let exit_statement = exit.map_or_else(String::new, |status| {
        format!("  exit; (make-exit-status; {status})\n")
    });
    format!(
        "namespace profile\n\n{exit_import}function main;\n  count int64 = {iterations}\n  index int64 = 0\n  total int64 = 0\n  while index < count\n    total = total + (index % 97)\n    index++\n  print; total\n{exit_statement}"
    )
}

fn record(directory: &TemporaryDirectory, output: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["profile", "record", "--cpu", "--embed-sources", "--output"])
        .arg(directory.path().join(output))
        .arg(directory.path().join("package.toml"))
        .arg("--")
        .arg("private-workload-argument")
        .output()
        .unwrap()
}

fn artifact(directory: &TemporaryDirectory, name: &str) -> serde_json::Value {
    serde_json::from_slice(&fs::read(directory.path().join(name)).unwrap()).unwrap()
}

fn wait_for_perf(process: u32, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        let children = fs::read_to_string(format!("/proc/{process}/task/{process}/children"))
            .unwrap_or_default();
        for child in children.split_whitespace() {
            let command = fs::read(format!("/proc/{child}/cmdline")).unwrap_or_default();
            if command
                .split(|byte| *byte == 0)
                .any(|argument| argument.ends_with(b"perf"))
            {
                return true;
            }
        }
        thread::sleep(Duration::from_millis(20));
    }
    false
}

fn contains_raw_capture(path: &Path) -> bool {
    fs::read_dir(path).is_ok_and(|entries| {
        entries.filter_map(Result::ok).any(|entry| {
            let path = entry.path();
            if path.is_dir() {
                contains_raw_capture(&path)
            } else {
                path.extension()
                    .is_some_and(|extension| extension == "data")
                    && path
                        .file_name()
                        .is_some_and(|name| name.to_string_lossy().starts_with("terrane-profile-"))
            }
        })
    })
}

#[test]
fn real_cpu_profiles_distinguish_slow_and_corrected_exact_builds() {
    let _serial = RealProfilerGuard::acquire();
    let directory = TemporaryDirectory::new();
    assert!(
        perf_available(directory.path()),
        "Linux x86-64 profiling evidence requires an available perf collector"
    );
    let source = directory.path().join("src/α.trn");
    fs::write(&source, workload(300_000_000, None)).unwrap();
    let slow = record(&directory, "slow.trnprof");
    assert!(slow.status.success(), "{slow:?}");

    fs::write(&source, workload(50_000_000, None)).unwrap();
    let corrected = record(&directory, "corrected.trnprof");
    assert!(corrected.status.success(), "{corrected:?}");

    let slow_stdout = String::from_utf8(slow.stdout).unwrap();
    let corrected_stdout = String::from_utf8(corrected.stdout).unwrap();
    assert_ne!(slow_stdout, corrected_stdout);

    let slow = artifact(&directory, "slow.trnprof");
    let corrected = artifact(&directory, "corrected.trnprof");
    assert_ne!(
        slow["provenance"]["native_module"]["content_hash"],
        corrected["provenance"]["native_module"]["content_hash"]
    );
    assert!(
        slow["conditions"]["elapsed_nanoseconds"].as_u64().unwrap()
            > corrected["conditions"]["elapsed_nanoseconds"]
                .as_u64()
                .unwrap()
    );
    assert!(
        slow["evidence"]["loss"]["captured_events"]
            .as_u64()
            .unwrap()
            > corrected["evidence"]["loss"]["captured_events"]
                .as_u64()
                .unwrap()
    );
    assert_eq!(slow["conditions"]["argument_policy"], "omitted");
    assert!(!slow.to_string().contains("private-workload-argument"));

    let shown = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["profile", "show"])
        .arg(directory.path().join("slow.trnprof"))
        .args(["--format", "json", "--focus", "src/α.trn:7"])
        .output()
        .unwrap();
    assert!(shown.status.success(), "{shown:?}");
    let report: serde_json::Value = serde_json::from_slice(&shown.stdout).unwrap();
    let captured = report["report"]["captured_samples"].as_u64().unwrap();
    let buckets = report["report"]["buckets"].as_object().unwrap();
    assert_eq!(
        buckets
            .values()
            .map(|value| value.as_u64().unwrap())
            .sum::<u64>(),
        captured
    );
    assert!(
        buckets["exact-authored"].as_u64().unwrap() * 100 >= captured * 95,
        "{report}"
    );
    assert_eq!(report["report"]["fidelity"], "exact-build-source");
    assert!(
        report["report"]["rows"]
            .as_array()
            .unwrap()
            .iter()
            .any(|row| row["source"]["source_uri"] == "src/α.trn")
    );
}

#[test]
fn nonzero_workload_still_publishes_usable_capture() {
    let _serial = RealProfilerGuard::acquire();
    let directory = TemporaryDirectory::new();
    assert!(
        perf_available(directory.path()),
        "Linux x86-64 profiling evidence requires an available perf collector"
    );
    fs::write(
        directory.path().join("src/main.trn"),
        workload(50_000_000, Some(7)),
    )
    .unwrap();

    let recorded = record(&directory, "failed.trnprof");

    assert_eq!(recorded.status.code(), Some(7), "{recorded:?}");
    let artifact = artifact(&directory, "failed.trnprof");
    assert_eq!(artifact["conditions"]["process_exit_code"], 7);
    assert!(
        artifact["evidence"]["loss"]["captured_events"]
            .as_u64()
            .unwrap()
            > 0
    );
}

#[test]
fn interruption_forwards_to_the_workload_and_finalizes_the_capture() {
    let _serial = RealProfilerGuard::acquire();
    let directory = TemporaryDirectory::new();
    assert!(
        perf_available(directory.path()),
        "Linux x86-64 profiling evidence requires an available perf collector"
    );
    fs::write(
        directory.path().join("src/main.trn"),
        workload(100_000_000_000, None),
    )
    .unwrap();
    let output = directory.path().join("interrupted.trnprof");
    let mut child = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["profile", "record", "--cpu", "--embed-sources", "--output"])
        .arg(&output)
        .arg(directory.path().join("package.toml"))
        .spawn()
        .unwrap();
    assert!(
        wait_for_perf(child.id(), Duration::from_secs(60)),
        "profiler did not start perf collector"
    );
    thread::sleep(Duration::from_millis(200));
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(i32::try_from(child.id()).unwrap()),
        nix::sys::signal::Signal::SIGINT,
    )
    .unwrap();

    let status = child.wait().unwrap();

    assert_eq!(status.code(), Some(130));
    let artifact: serde_json::Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
    assert_eq!(artifact["conditions"]["interrupted"], true);
    assert_eq!(artifact["conditions"]["terminating_signal"], 2);
    assert!(
        artifact["evidence"]["loss"]["captured_events"]
            .as_u64()
            .unwrap()
            > 0
    );
    assert!(!contains_raw_capture(directory.path()));
}
