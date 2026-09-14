use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::{
    CliCommand, CliFailure, GeneratedCrateOptions, emit_warnings, ensure_rust_toolchain,
    generated_crate_path, prepare_artifact, record_and_prune_generated_crates,
    write_generated_crate,
};
use terrane_compiler::{
    Package,
    testing::{TestCase, TestPackage, TestTier},
};
const CAPTURE_LIMIT: usize = 1024 * 1024;
const REPORT_SCHEMA_VERSION: &str = "1.0.0";

#[derive(Clone, Debug)]
struct TestOptions {
    input: PathBuf,
    filter: Option<String>,
    list: bool,
    fail_fast: bool,
    show_output: bool,
    tiers: BTreeSet<TestTier>,
    jobs: usize,
    timeout: Duration,
    report: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TestStatus {
    Passed,
    Failed,
    Skipped,
    TimedOut,
    Crashed,
    InfrastructureFailed,
}

impl TestStatus {
    fn name(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::TimedOut => "timed-out",
            Self::Crashed => "crashed",
            Self::InfrastructureFailed => "infrastructure-failed",
        }
    }

    fn successful(self) -> bool {
        matches!(self, Self::Passed | Self::Skipped)
    }
}

#[derive(Clone, Debug, Default)]
struct CapturedOutput {
    bytes: Vec<u8>,
    truncated: bool,
}

#[derive(Clone, Debug)]
struct TestResult {
    case: TestCase,
    status: TestStatus,
    duration: Duration,
    stdout: CapturedOutput,
    stderr: CapturedOutput,
    detail: Option<String>,
}

pub(super) fn run_tests(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let options = parse_test_options(arguments)?;
    let test_package = TestPackage::load(&options.input).map_err(|errors| CliFailure {
        code: 3,
        message: errors
            .into_iter()
            .map(|error| error.diagnostic.render(&error.source))
            .collect(),
    })?;
    let (compilation, all_cases) = terrane_compiler::compile_test_package(
        &test_package,
        terrane_compiler::CompilerOptions::default(),
    )
    .map_err(CliFailure::compilation)?;
    emit_warnings(&compilation);
    let cases = all_cases
        .into_iter()
        .filter(|case| {
            (options.tiers.is_empty() || options.tiers.contains(&case.tier))
                && options
                    .filter
                    .as_ref()
                    .is_none_or(|filter| case.identity.contains(filter))
        })
        .collect::<Vec<_>>();
    if options.list {
        for case in &cases {
            println!("{} [{}]", case.identity, case.tier.name());
        }
        return Ok(ExitCode::SUCCESS);
    }
    if cases.is_empty() {
        println!("0 tests selected");
        return Ok(ExitCode::SUCCESS);
    }

    let application_artifact = if cases.iter().any(|case| case.tier == TestTier::EndToEnd) {
        let application_package = Package::load(&options.input).map_err(|errors| CliFailure {
            code: 3,
            message: errors
                .into_iter()
                .map(|error| error.diagnostic.render(&error.source))
                .collect(),
        })?;
        let application = terrane_compiler::compile_package(&application_package)
            .map_err(CliFailure::compilation)?;
        Some(build_native_compilation(
            &application_package,
            &application,
        )?)
    } else {
        None
    };
    let executable = build_test_runner(&test_package, &compilation)?;
    let run_root = test_package.package.root.join(".trn/test/run");
    fs::create_dir_all(&run_root).map_err(|error| {
        CliFailure::backend(format!(
            "cannot create isolated test run directory: {error}"
        ))
    })?;
    let results = execute_cases(
        &executable,
        application_artifact.as_deref(),
        &run_root,
        &cases,
        &options,
    );
    render_human_report(&results, options.show_output);
    if let Some(path) = &options.report {
        write_machine_report(path, &results)?;
    }
    if results
        .iter()
        .any(|result| result.status == TestStatus::InfrastructureFailed)
    {
        Ok(ExitCode::from(5))
    } else if results.iter().any(|result| !result.status.successful()) {
        Ok(ExitCode::from(1))
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

fn parse_test_options(arguments: &[OsString]) -> Result<TestOptions, CliFailure> {
    let mut input = None;
    let mut filter = None;
    let mut list = false;
    let mut fail_fast = false;
    let mut show_output = false;
    let mut tiers = BTreeSet::new();
    let mut jobs = std::thread::available_parallelism().map_or(1, usize::from);
    let mut timeout = Duration::from_secs(30);
    let mut report = None;
    let mut index = 1;
    while index < arguments.len() {
        let argument = arguments[index].to_str().ok_or_else(CliFailure::usage)?;
        match argument {
            "--list" => list = true,
            "--fail-fast" => fail_fast = true,
            "--show-output" => show_output = true,
            "--filter" => {
                index += 1;
                filter = Some(
                    arguments
                        .get(index)
                        .and_then(|value| value.to_str())
                        .ok_or_else(CliFailure::usage)?
                        .to_owned(),
                );
            }
            "--tier" => {
                index += 1;
                let tier = match arguments.get(index).and_then(|value| value.to_str()) {
                    Some("unit") => TestTier::Unit,
                    Some("integration") => TestTier::Integration,
                    Some("end-to-end") => TestTier::EndToEnd,
                    _ => return Err(CliFailure::usage()),
                };
                tiers.insert(tier);
            }
            "--jobs" => {
                index += 1;
                jobs = arguments
                    .get(index)
                    .and_then(|value| value.to_str())
                    .and_then(|value| value.parse::<usize>().ok())
                    .filter(|value| *value > 0)
                    .ok_or_else(CliFailure::usage)?;
            }
            "--timeout" => {
                index += 1;
                timeout = parse_duration(
                    arguments
                        .get(index)
                        .and_then(|value| value.to_str())
                        .ok_or_else(CliFailure::usage)?,
                )
                .ok_or_else(CliFailure::usage)?;
            }
            "--report" => {
                index += 1;
                report = Some(
                    arguments
                        .get(index)
                        .map(PathBuf::from)
                        .ok_or_else(CliFailure::usage)?,
                );
            }
            _ if argument.starts_with('-') || input.is_some() => return Err(CliFailure::usage()),
            _ => input = Some(PathBuf::from(&arguments[index])),
        }
        index += 1;
    }
    Ok(TestOptions {
        input: input.ok_or_else(CliFailure::usage)?,
        filter,
        list,
        fail_fast,
        show_output,
        tiers,
        jobs,
        timeout,
        report,
    })
}

fn parse_duration(value: &str) -> Option<Duration> {
    if let Some(milliseconds) = value.strip_suffix("ms") {
        milliseconds
            .parse::<u64>()
            .ok()
            .filter(|value| *value > 0)
            .map(Duration::from_millis)
    } else if let Some(seconds) = value.strip_suffix('s') {
        seconds
            .parse::<u64>()
            .ok()
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
    } else {
        value
            .parse::<u64>()
            .ok()
            .filter(|value| *value > 0)
            .map(Duration::from_secs)
    }
}

fn build_test_runner(
    test_package: &TestPackage,
    compilation: &terrane_compiler::Compilation,
) -> Result<PathBuf, CliFailure> {
    build_native_compilation(&test_package.package, compilation)
}

fn build_native_compilation(
    package: &Package,
    compilation: &terrane_compiler::Compilation,
) -> Result<PathBuf, CliFailure> {
    ensure_rust_toolchain(package.build_toolchain)?;
    let rust_files = compilation
        .rust_files_for(Path::new("src/main.rs"))
        .map_err(CliFailure::rust_artifact)?;
    let uses_platform_support = compilation.requires_platform_support;
    let uses_async_runtime = compilation.requires_async_runtime;
    let uses_tokio_sync = rust_files
        .iter()
        .any(|file| file.contents.contains("tokio::sync::"));
    let crate_dir = generated_crate_path(
        &package.root,
        &rust_files,
        uses_platform_support,
        uses_async_runtime,
        &compilation.rust_dependencies,
        package.build_toolchain,
    )?;
    write_generated_crate(
        &crate_dir,
        &rust_files,
        &package.units,
        &compilation.rust_dependencies,
        GeneratedCrateOptions {
            panic: package.profile.panic,
            uses_platform_support,
            uses_async_runtime,
            uses_tokio_sync,
            build_toolchain: package.build_toolchain,
        },
    )?;
    record_and_prune_generated_crates(&crate_dir)?;
    let target_dir = package.root.join(".trn/cache/target");
    prepare_artifact(
        CliCommand::Build,
        &crate_dir,
        &target_dir,
        &rust_files,
        &package.units,
        !compilation.rust_dependencies.is_empty(),
        compilation.dependency_containment,
        false,
    )?
    .ok_or_else(|| CliFailure::backend("native build produced no executable".to_owned()))
}

fn execute_cases(
    executable: &Path,
    application_artifact: Option<&Path>,
    run_root: &Path,
    cases: &[TestCase],
    options: &TestOptions,
) -> Vec<TestResult> {
    if options.fail_fast || options.jobs == 1 {
        let mut results = Vec::new();
        for (index, case) in cases.iter().enumerate() {
            let result = execute_case(
                executable,
                application_artifact,
                run_root,
                index,
                case,
                options.timeout,
            );
            let stop = options.fail_fast && !result.status.successful();
            results.push(result);
            if stop {
                break;
            }
        }
        return results;
    }
    let next = Arc::new(AtomicUsize::new(0));
    let results = Arc::new(Mutex::new(Vec::with_capacity(cases.len())));
    std::thread::scope(|scope| {
        for _ in 0..options.jobs.min(cases.len()) {
            let next = Arc::clone(&next);
            let results = Arc::clone(&results);
            scope.spawn(move || {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(case) = cases.get(index) else {
                        break;
                    };
                    let result = execute_case(
                        executable,
                        application_artifact,
                        run_root,
                        index,
                        case,
                        options.timeout,
                    );
                    results
                        .lock()
                        .expect("test results lock poisoned")
                        .push((index, result));
                }
            });
        }
    });
    let mut indexed = Arc::into_inner(results)
        .expect("test workers released result storage")
        .into_inner()
        .expect("test results lock poisoned");
    indexed.sort_by_key(|(index, _)| *index);
    indexed.into_iter().map(|(_, result)| result).collect()
}

fn execute_case(
    executable: &Path,
    application_artifact: Option<&Path>,
    run_root: &Path,
    index: usize,
    case: &TestCase,
    timeout: Duration,
) -> TestResult {
    let directory = run_root.join(format!("{}-{index}", std::process::id()));
    let _ = fs::remove_dir_all(&directory);
    if let Err(error) = fs::create_dir_all(&directory) {
        return infrastructure_result(case, format!("cannot create test directory: {error}"));
    }
    let started = Instant::now();
    let mut command = Command::new(executable);
    command
        .arg(index.to_string())
        .current_dir(&directory)
        .env_clear()
        .env("TERRANE_TEST_ID", &case.identity)
        .env("TERRANE_TEST_TIER", case.tier.name())
        .env("TERRANE_TEST_SEED", deterministic_seed(&case.identity))
        .env("TMPDIR", &directory)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(artifact) = application_artifact {
        command.env("TERRANE_TEST_ARTIFACT", artifact);
    }
    let spawn = command.spawn();
    let mut child = match spawn {
        Ok(child) => child,
        Err(error) => {
            let _ = fs::remove_dir_all(&directory);
            return infrastructure_result(case, format!("cannot start test process: {error}"));
        }
    };
    let stdout = child.stdout.take().expect("piped test stdout");
    let stderr = child.stderr.take().expect("piped test stderr");
    let stdout_reader = std::thread::spawn(move || read_bounded(stdout));
    let stderr_reader = std::thread::spawn(move || read_bounded(stderr));
    let (status, timed_out) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (Some(status), false),
            Ok(None) if started.elapsed() < timeout => std::thread::sleep(Duration::from_millis(5)),
            Ok(None) => {
                let _ = child.kill();
                break (child.wait().ok(), true);
            }
            Err(_) => break (None, false),
        }
    };
    let stdout = stdout_reader.join().unwrap_or_default();
    let stderr = stderr_reader.join().unwrap_or_default();
    let _ = fs::remove_dir_all(&directory);
    let exit_code = status.as_ref().and_then(std::process::ExitStatus::code);
    let status = if timed_out {
        TestStatus::TimedOut
    } else if status
        .as_ref()
        .is_some_and(std::process::ExitStatus::success)
    {
        TestStatus::Passed
    } else if stderr
        .bytes
        .windows(b"TERRANE_TEST_SKIP:".len())
        .any(|part| part == b"TERRANE_TEST_SKIP:")
    {
        TestStatus::Skipped
    } else if status
        .as_ref()
        .is_some_and(|status| status.code().is_none())
    {
        TestStatus::Crashed
    } else if status.is_some() {
        TestStatus::Failed
    } else {
        TestStatus::InfrastructureFailed
    };
    let detail = match status {
        TestStatus::TimedOut => Some(format!(
            "deadline exceeded after {} milliseconds",
            timeout.as_millis()
        )),
        TestStatus::Crashed => Some("test process terminated without an exit code".to_owned()),
        TestStatus::Failed => Some(format!(
            "test process exited with status {}",
            exit_code.map_or_else(|| "unknown".to_owned(), |code| code.to_string())
        )),
        TestStatus::Skipped => Some("test requested an explicit skip".to_owned()),
        TestStatus::Passed | TestStatus::InfrastructureFailed => None,
    };
    TestResult {
        case: case.clone(),
        status,
        duration: started.elapsed(),
        stdout,
        stderr,
        detail,
    }
}

fn infrastructure_result(case: &TestCase, detail: String) -> TestResult {
    TestResult {
        case: case.clone(),
        status: TestStatus::InfrastructureFailed,
        duration: Duration::ZERO,
        stdout: CapturedOutput::default(),
        stderr: CapturedOutput::default(),
        detail: Some(detail),
    }
}

fn read_bounded(mut stream: impl std::io::Read) -> CapturedOutput {
    let mut captured = CapturedOutput::default();
    let mut buffer = [0_u8; 8192];
    while let Ok(read) = stream.read(&mut buffer) {
        if read == 0 {
            break;
        }
        let available = CAPTURE_LIMIT.saturating_sub(captured.bytes.len());
        captured
            .bytes
            .extend_from_slice(&buffer[..read.min(available)]);
        captured.truncated |= read > available;
    }
    captured
}

fn deterministic_seed(identity: &str) -> String {
    use sha2::{Digest as _, Sha256};
    let digest = Sha256::digest(identity.as_bytes());
    u64::from_le_bytes(digest[..8].try_into().expect("SHA-256 prefix width")).to_string()
}

fn render_human_report(results: &[TestResult], show_output: bool) {
    for result in results {
        println!(
            "{} {} ({:.3}s)",
            result.status.name(),
            result.case.identity,
            result.duration.as_secs_f64()
        );
        if show_output || !result.status.successful() {
            if !result.stdout.bytes.is_empty() {
                println!(
                    "--- stdout{} ---\n{}",
                    if result.stdout.truncated {
                        " (truncated)"
                    } else {
                        ""
                    },
                    String::from_utf8_lossy(&result.stdout.bytes)
                );
            }
            if !result.stderr.bytes.is_empty() {
                eprintln!(
                    "--- stderr{} ---\n{}",
                    if result.stderr.truncated {
                        " (truncated)"
                    } else {
                        ""
                    },
                    String::from_utf8_lossy(&result.stderr.bytes)
                );
            }
            if let Some(detail) = &result.detail {
                eprintln!("--- cause ---\n{detail}");
            }
        }
    }
    let passed = results
        .iter()
        .filter(|result| result.status == TestStatus::Passed)
        .count();
    let skipped = results
        .iter()
        .filter(|result| result.status == TestStatus::Skipped)
        .count();
    let failed = results.len() - passed - skipped;
    println!("{passed} passed; {skipped} skipped; {failed} failed");
}

fn write_machine_report(path: &Path, results: &[TestResult]) -> Result<(), CliFailure> {
    let cases = results
        .iter()
        .map(|result| {
            serde_json::json!({
                "identity": result.case.identity,
                "tier": result.case.tier.name(),
                "status": result.status.name(),
                "duration_nanoseconds": result.duration.as_nanos().to_string(),
                "source": {
                    "path": result.case.source_path,
                    "start": result.case.source_span.start,
                    "end": result.case.source_span.end,
                },
                "cause": result.detail,
                "stdout": result.stdout.bytes,
                "stdout_truncated": result.stdout.truncated,
                "stderr": result.stderr.bytes,
                "stderr_truncated": result.stderr.truncated,
            })
        })
        .collect::<Vec<_>>();
    let report = serde_json::json!({
        "schema_version": REPORT_SCHEMA_VERSION,
        "cases": cases,
    });
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| {
            CliFailure::backend(format!("cannot create test report directory: {error}"))
        })?;
    }
    fs::write(
        path,
        serde_json::to_vec_pretty(&report).expect("test report is serializable"),
    )
    .map_err(|error| CliFailure::backend(format!("cannot write test report: {error}")))
}
