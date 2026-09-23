#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
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

fn heaptrack_available() -> bool {
    Command::new("heaptrack")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
        && Command::new("heaptrack_print")
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success())
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
    let source_row = report["report"]["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["source"]["source_uri"] == "src/α.trn")
        .unwrap();
    assert_eq!(source_row["quality"], "exact-authored");
    assert!(
        source_row["inclusive_samples"].as_u64().unwrap() * 100 >= captured * 95,
        "{report}"
    );
    assert!(buckets["native-only"].as_u64().unwrap() > 0);
    assert_eq!(report["report"]["fidelity"], "exact-build-source");
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
    let path = directory.path().join("failed.trnprof");
    let mut artifact = artifact(&directory, "failed.trnprof");
    assert_eq!(artifact["conditions"]["process_exit_code"], 7);
    assert!(
        artifact["evidence"]["loss"]["captured_events"]
            .as_u64()
            .unwrap()
            > 0
    );
    artifact["evidence"]["modules"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({
            "path": "/missing/libfixture.so",
            "build_id": "fixture",
            "content_hash": "sha256:fixture",
            "is_profiled_executable": false
        }));
    fs::write(&path, serde_json::to_vec(&artifact).unwrap()).unwrap();
    let shown = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["profile", "show"])
        .arg(&path)
        .args(["--format", "json"])
        .output()
        .unwrap();
    assert!(shown.status.success(), "{shown:?}");
    let report: serde_json::Value = serde_json::from_slice(&shown.stdout).unwrap();
    assert_eq!(report["report"]["fidelity"], "exact-build-source");
    assert_eq!(
        report["report"]["native_fidelity"],
        "reduced-native-modules"
    );
    assert!(
        report["report"]["native_fidelity_reasons"][0]
            .as_str()
            .unwrap()
            .contains("/missing/libfixture.so")
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

#[test]
fn allocation_capture_separates_churn_from_retained_bytes() {
    if !heaptrack_available() {
        return;
    }
    let _guard = RealProfilerGuard::acquire();
    let directory = TemporaryDirectory::new();
    fs::write(
        directory.path().join("src/main.trn"),
        "namespace profile\n\
         from /core/collections import list\n\
         function main;\n\
           index int = 0\n\
           while index < 10000\n\
             temporary list of int = list; index, index + 1, index + 2\n\
             index++\n\
           retained list of int = list\n\
           index = 0\n\
           while index < 10000\n\
             retained.append; index\n\
             index++\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args([
            "profile",
            "record",
            "--allocations",
            "--memory-timeline",
            "--output",
        ])
        .arg(directory.path().join("allocation.trnprof"))
        .arg(directory.path().join("package.toml"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let evidence = artifact(&directory, "allocation.trnprof");
    let allocations = &evidence["allocations"];
    let allocated = allocations["allocated_bytes"].as_u64().unwrap();
    let freed = allocations["freed_bytes"].as_u64().unwrap();
    let retained = allocations["retained_bytes_at_exit"].as_u64().unwrap();
    assert!(allocated > retained);
    assert!(freed > retained);
    assert!(retained > 0);
    assert_eq!(allocated, freed + retained);
    assert!(!allocations["partial"].as_bool().unwrap());
    assert!(evidence["memory_timeline"]["samples"].as_array().is_some());
    let threshold = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args([
            "profile",
            "show",
            "allocation.trnprof",
            "--max-retained-bytes",
            "0",
        ])
        .current_dir(directory.path())
        .output()
        .unwrap();
    assert_eq!(threshold.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&threshold.stdout).contains("retained at exit"));

    let comparison = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args([
            "profile",
            "show",
            "allocation.trnprof",
            "--compare",
            "allocation.trnprof",
            "--format",
            "json",
            "--limit",
            "1",
        ])
        .current_dir(directory.path())
        .output()
        .unwrap();
    assert!(comparison.status.success());
    let report: serde_json::Value = serde_json::from_slice(&comparison.stdout).unwrap();
    assert_eq!(report["comparison"]["allocated_bytes_delta"], 0);
    assert_eq!(report["comparison"]["retained_bytes_delta"], 0);
    assert_eq!(report["allocations"]["events"].as_array().unwrap().len(), 1);
}

#[test]
fn missing_and_permission_denied_collectors_fail_explicitly() {
    let directory = TemporaryDirectory::new();
    fs::write(directory.path().join("src/main.trn"), workload(10, None)).unwrap();
    let missing = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["profile", "record", "--cpu", "--output"])
        .arg(directory.path().join("missing.trnprof"))
        .arg(directory.path().join("package.toml"))
        .env("TERRANE_PERF", directory.path().join("missing-perf"))
        .output()
        .unwrap();
    assert!(!missing.status.success());
    assert!(
        String::from_utf8_lossy(&missing.stderr)
            .contains("Linux perf is required for CPU profiling but could not be started")
    );

    let denied_perf = directory.path().join("denied-perf");
    fs::write(
        &denied_perf,
        "#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then echo 'perf version fixture'; exit 0; fi\necho 'perf permission denied fixture' >&2\nexit 255\n",
    )
    .unwrap();
    fs::set_permissions(&denied_perf, fs::Permissions::from_mode(0o755)).unwrap();
    let denied = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["profile", "record", "--cpu", "--output"])
        .arg(directory.path().join("denied.trnprof"))
        .arg(directory.path().join("package.toml"))
        .env("TERRANE_PERF", &denied_perf)
        .output()
        .unwrap();
    assert!(!denied.status.success());
    assert!(String::from_utf8_lossy(&denied.stderr).contains("perf permission denied fixture"));
}
