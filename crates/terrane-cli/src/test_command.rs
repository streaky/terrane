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
const REPORT_SCHEMA_VERSION: &str = "1.1.0";

#[derive(Clone, Debug)]
enum TestSelector {
    Substring(String),
    Exact(String),
    Glob(String),
    Regex(regex::Regex),
}

impl TestSelector {
    fn matches(&self, identity: &str) -> bool {
        match self {
            Self::Substring(pattern) => identity.contains(pattern),
            Self::Exact(pattern) => identity == pattern,
            Self::Glob(pattern) => glob_matches(pattern.as_bytes(), identity.as_bytes()),
            Self::Regex(pattern) => pattern.is_match(identity),
        }
    }

    fn report(&self) -> serde_json::Value {
        let (mode, pattern) = match self {
            Self::Substring(pattern) => ("substring", pattern.as_str()),
            Self::Exact(pattern) => ("exact", pattern.as_str()),
            Self::Glob(pattern) => ("glob", pattern.as_str()),
            Self::Regex(pattern) => ("regex", pattern.as_str()),
        };
        serde_json::json!({ "mode": mode, "pattern": pattern })
    }
}

fn glob_matches(pattern: &[u8], value: &[u8]) -> bool {
    let mut reachable = vec![false; value.len() + 1];
    reachable[0] = true;
    for token in pattern {
        if *token == b'*' {
            for index in 1..=value.len() {
                reachable[index] |= reachable[index - 1];
            }
        } else {
            for index in (1..=value.len()).rev() {
                reachable[index] =
                    reachable[index - 1] && (*token == b'?' || *token == value[index - 1]);
            }
            reachable[0] = false;
        }
    }
    reachable[value.len()]
}

#[derive(Clone, Debug)]
struct TestOptions {
    input: PathBuf,
    filter: Option<TestSelector>,
    list: bool,
    fail_fast: bool,
    show_output: bool,
    tiers: BTreeSet<TestTier>,
    jobs: usize,
    timeout: Duration,
    report: Option<PathBuf>,
    arguments: Vec<OsString>,
}

struct TestRecord {
    outcome: String,
    descriptor: String,
    message: String,
    details: Vec<String>,
    source_frames: Vec<String>,
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
struct TestCause {
    kind: &'static str,
    descriptor: Option<String>,
    message: String,
    details: Vec<String>,
    source_frames: Vec<String>,
}
#[derive(Clone, Debug)]
struct TestResult {
    case: TestCase,
    status: TestStatus,
    duration: Duration,
    stdout: CapturedOutput,
    stderr: CapturedOutput,
    cause: Option<TestCause>,
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
    let compiled_tiers = match terrane_compiler::compile_test_package(
        &test_package,
        terrane_compiler::CompilerOptions::default(),
    ) {
        Ok(compiled) => compiled,
        Err(failure) => {
            if let Some(path) = &options.report {
                write_compile_failure_report(path, &failure, &options)?;
            }
            return Err(CliFailure::compilation(failure));
        }
    };
    for tier in &compiled_tiers {
        emit_warnings(&tier.compilation);
    }
    let all_cases = compiled_tiers
        .iter()
        .flat_map(|tier| tier.cases.iter().cloned())
        .collect::<Vec<_>>();
    let cases = all_cases
        .into_iter()
        .filter(|case| {
            (options.tiers.is_empty() || options.tiers.contains(&case.tier))
                && options
                    .filter
                    .as_ref()
                    .is_none_or(|filter| filter.matches(&case.identity))
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
    let mut executables = std::collections::BTreeMap::new();
    for tier in &compiled_tiers {
        executables.insert(
            tier.tier,
            build_native_compilation(&tier.package, &tier.compilation)?,
        );
    }
    let run_root = test_package.package.root.join(".trn/test/run");
    fs::create_dir_all(&run_root).map_err(|error| {
        CliFailure::backend(format!(
            "cannot create isolated test run directory: {error}"
        ))
    })?;
    let results = execute_cases(
        &executables,
        application_artifact.as_deref(),
        &run_root,
        &cases,
        &options,
    );
    let _ = fs::remove_dir_all(&run_root);
    render_human_report(&results, options.show_output);
    if let Some(path) = &options.report {
        write_machine_report(path, &results, &options)?;
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
    let mut test_arguments = Vec::new();
    let mut index = 1;
    while index < arguments.len() {
        let argument = arguments[index].to_str().ok_or_else(CliFailure::usage)?;
        match argument {
            "--list" => list = true,
            "--fail-fast" => fail_fast = true,
            "--show-output" => show_output = true,
            "--filter" | "--exact" | "--glob" | "--regex" => {
                index += 1;
                if filter.is_some() {
                    return Err(CliFailure::usage());
                }
                let pattern = arguments
                    .get(index)
                    .and_then(|value| value.to_str())
                    .ok_or_else(CliFailure::usage)?;
                filter = Some(match argument {
                    "--filter" => TestSelector::Substring(pattern.to_owned()),
                    "--exact" => TestSelector::Exact(pattern.to_owned()),
                    "--glob" => TestSelector::Glob(pattern.to_owned()),
                    "--regex" => TestSelector::Regex(
                        regex::Regex::new(pattern).map_err(|_| CliFailure::usage())?,
                    ),
                    _ => unreachable!("matched selector option"),
                });
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
            "--argument" => {
                index += 1;
                test_arguments.push(
                    arguments
                        .get(index)
                        .cloned()
                        .ok_or_else(CliFailure::usage)?,
                );
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
        arguments: test_arguments,
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
    executables: &std::collections::BTreeMap<TestTier, PathBuf>,
    application_artifact: Option<&Path>,
    run_root: &Path,
    cases: &[TestCase],
    options: &TestOptions,
) -> Vec<TestResult> {
    if options.fail_fast || options.jobs == 1 {
        let mut results = Vec::new();
        for (index, case) in cases.iter().enumerate() {
            let result = execute_case(
                executables,
                application_artifact,
                run_root,
                index,
                case,
                options.timeout,
                &options.arguments,
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
                        executables,
                        application_artifact,
                        run_root,
                        index,
                        case,
                        options.timeout,
                        &options.arguments,
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
    executables: &std::collections::BTreeMap<TestTier, PathBuf>,
    application_artifact: Option<&Path>,
    run_root: &Path,
    work_index: usize,
    case: &TestCase,
    timeout: Duration,
    arguments: &[OsString],
) -> TestResult {
    let directory = run_root.join(format!("{}-{work_index}", std::process::id()));
    let _ = fs::remove_dir_all(&directory);
    if let Err(error) = fs::create_dir_all(&directory) {
        return infrastructure_result(case, &format!("cannot create test directory: {error}"));
    }
    let started = Instant::now();
    let Some(executable) = executables.get(&case.tier) else {
        return infrastructure_result(case, "test tier runner is unavailable");
    };
    let mut command = Command::new(executable);
    command
        .arg(case.selector.to_string())
        .args(arguments)
        .current_dir(&directory)
        .env_clear()
        .env("TERRANE_TEST_ID", &case.identity)
        .env("TERRANE_TEST_TIER", case.tier.name())
        .env("TERRANE_TEST_SEED", deterministic_seed(&case.identity))
        .env("TMPDIR", &directory)
        .env(
            "TERRANE_TEST_DEADLINE_NANOSECONDS",
            timeout.as_nanos().to_string(),
        )
        .env("TERRANE_TEST_RESULT", directory.join(".terrane-result"))
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
            return infrastructure_result(case, &format!("cannot start test process: {error}"));
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
    let record = fs::read_to_string(directory.join(".terrane-result"))
        .ok()
        .map(|record| parse_test_record(&record));
    let _ = fs::remove_dir_all(&directory);
    let (test_status, failure_cause) =
        classify_test_result(status.as_ref(), timed_out, record, timeout);
    TestResult {
        case: case.clone(),
        status: test_status,
        duration: started.elapsed(),
        stdout,
        stderr,
        cause: failure_cause,
    }
}

fn timeout_result(timeout: Duration) -> (TestStatus, Option<TestCause>) {
    (
        TestStatus::TimedOut,
        Some(TestCause {
            kind: "timeout",
            descriptor: None,
            message: format!(
                "deadline exceeded after {} milliseconds",
                timeout.as_millis()
            ),
            details: Vec::new(),
            source_frames: Vec::new(),
        }),
    )
}

fn classify_test_result(
    process_status: Option<&std::process::ExitStatus>,
    timed_out: bool,
    record: Option<Result<TestRecord, String>>,
    timeout: Duration,
) -> (TestStatus, Option<TestCause>) {
    let exit_code = process_status.and_then(std::process::ExitStatus::code);
    if timed_out {
        return timeout_result(timeout);
    }
    if process_status.is_some_and(std::process::ExitStatus::success) {
        return (TestStatus::Passed, None);
    }
    if matches!(exit_code, Some(101 | 102)) {
        return match record {
            Some(Ok(record))
                if (exit_code == Some(102) && record.outcome == "skipped")
                    || (exit_code == Some(101) && record.outcome == "failed") =>
            {
                let skipped = record.outcome == "skipped";
                let infrastructure =
                    record.descriptor == "/core/testing::test-infrastructure-failure";
                (
                    if skipped {
                        TestStatus::Skipped
                    } else if infrastructure {
                        TestStatus::InfrastructureFailed
                    } else {
                        TestStatus::Failed
                    },
                    Some(TestCause {
                        kind: if skipped {
                            "skip"
                        } else if record.descriptor == "/core/testing::test-failure" {
                            "assertion"
                        } else if infrastructure {
                            "infrastructure"
                        } else {
                            "uncaught-throwable"
                        },
                        descriptor: Some(record.descriptor),
                        message: record.message,
                        details: record.details,
                        source_frames: record.source_frames,
                    }),
                )
            }
            Some(Ok(_)) => (
                TestStatus::InfrastructureFailed,
                Some(protocol_cause(
                    "test result outcome disagrees with runner exit status",
                )),
            ),
            Some(Err(error)) => (
                TestStatus::InfrastructureFailed,
                Some(protocol_cause(&format!(
                    "invalid test result record: {error}"
                ))),
            ),
            None => (
                TestStatus::InfrastructureFailed,
                Some(protocol_cause("test runner exited without a result record")),
            ),
        };
    }
    if process_status.is_some_and(|status| status.code().is_none()) {
        return (
            TestStatus::Crashed,
            Some(TestCause {
                kind: "crash",
                descriptor: None,
                message: "test process terminated without an exit code".to_owned(),
                details: Vec::new(),
                source_frames: Vec::new(),
            }),
        );
    }
    if exit_code == Some(103) || process_status.is_none() {
        return (
            TestStatus::InfrastructureFailed,
            Some(protocol_cause("test runner dispatch failed")),
        );
    }
    (
        TestStatus::Failed,
        Some(TestCause {
            kind: "exit",
            descriptor: None,
            message: format!(
                "test process exited with status {}",
                exit_code.map_or_else(|| "unknown".to_owned(), |code| code.to_string())
            ),
            details: Vec::new(),
            source_frames: Vec::new(),
        }),
    )
}

fn infrastructure_result(case: &TestCase, detail: &str) -> TestResult {
    TestResult {
        case: case.clone(),
        status: TestStatus::InfrastructureFailed,
        duration: Duration::ZERO,
        stdout: CapturedOutput::default(),
        stderr: CapturedOutput::default(),
        cause: Some(protocol_cause(detail)),
    }
}

fn protocol_cause(message: &str) -> TestCause {
    TestCause {
        kind: "infrastructure",
        descriptor: None,
        message: message.to_owned(),
        details: Vec::new(),
        source_frames: Vec::new(),
    }
}

fn parse_test_record(record: &str) -> Result<TestRecord, String> {
    let mut lines = record.lines();
    if lines.next() != Some("1") {
        return Err("unsupported protocol version".to_owned());
    }
    let outcome = lines.next().ok_or_else(|| "missing outcome".to_owned())?;
    let descriptor = decode_hex(
        lines
            .next()
            .ok_or_else(|| "missing descriptor".to_owned())?,
    )?;
    let message = decode_hex(lines.next().ok_or_else(|| "missing message".to_owned())?)?;
    let detail_count = lines
        .next()
        .ok_or_else(|| "missing detail count".to_owned())?
        .parse::<usize>()
        .map_err(|_| "invalid detail count".to_owned())?;
    let frame_count = lines
        .next()
        .ok_or_else(|| "missing frame count".to_owned())?
        .parse::<usize>()
        .map_err(|_| "invalid frame count".to_owned())?;
    let details = lines
        .by_ref()
        .take(detail_count)
        .map(decode_hex)
        .collect::<Result<Vec<_>, _>>()?;
    let frames = lines
        .by_ref()
        .take(frame_count)
        .map(decode_hex)
        .collect::<Result<Vec<_>, _>>()?;
    if details.len() != detail_count || frames.len() != frame_count || lines.next().is_some() {
        return Err("detail or frame count does not match record".to_owned());
    }
    Ok(TestRecord {
        outcome: outcome.to_owned(),
        descriptor,
        message,
        details,
        source_frames: frames,
    })
}

fn decode_hex(value: &str) -> Result<String, String> {
    if !value.len().is_multiple_of(2) {
        return Err("odd-length hexadecimal field".to_owned());
    }
    let (pairs, remainder) = value.as_bytes().as_chunks::<2>();
    if !remainder.is_empty() {
        return Err("odd-length hexadecimal field".to_owned());
    }
    let bytes = pairs
        .iter()
        .map(|pair| {
            let text = std::str::from_utf8(pair).expect("ASCII hexadecimal pair");
            u8::from_str_radix(text, 16).map_err(|_| "invalid hexadecimal field".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    String::from_utf8(bytes).map_err(|_| "result field is not UTF-8".to_owned())
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
                println!(
                    "--- stderr{} ---\n{}",
                    if result.stderr.truncated {
                        " (truncated)"
                    } else {
                        ""
                    },
                    String::from_utf8_lossy(&result.stderr.bytes)
                );
            }
            if let Some(cause) = &result.cause {
                println!(
                    "--- cause: {}{} ---\n{}",
                    cause.kind,
                    cause
                        .descriptor
                        .as_deref()
                        .map_or_else(String::new, |descriptor| format!(" ({descriptor})")),
                    cause.message
                );
                for detail in &cause.details {
                    println!("{detail}");
                }
                for frame in &cause.source_frames {
                    println!("at {frame}");
                }
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

fn report_run_metadata(options: &TestOptions, status: &str) -> serde_json::Value {
    serde_json::json!({
        "status": status,
        "package": options.input,
        "filter": options.filter.as_ref().map(TestSelector::report),
        "tiers": options.tiers.iter().map(|tier| tier.name()).collect::<Vec<_>>(),
        "jobs": options.jobs,
        "timeout_milliseconds": options.timeout.as_millis().to_string(),
        "fail_fast": options.fail_fast,
        "show_output": options.show_output,
    })
}

fn write_compile_failure_report(
    path: &Path,
    failure: &terrane_compiler::CompilationFailure,
    options: &TestOptions,
) -> Result<(), CliFailure> {
    let diagnostics = failure
        .diagnostics
        .iter()
        .map(|diagnostic| {
            serde_json::json!({
                "code": diagnostic.code,
                "message": diagnostic.message,
                "severity": format!("{:?}", diagnostic.severity).to_ascii_lowercase(),
                "source": diagnostic.primary.map(|span| serde_json::json!({
                    "path": failure.source.path(),
                    "start": span.start,
                    "end": span.end,
                })),
            })
        })
        .collect::<Vec<_>>();
    write_report_value(
        path,
        &serde_json::json!({
            "schema_version": REPORT_SCHEMA_VERSION,
            "run": report_run_metadata(options, "compile-failed"),
            "compilation": {
                "status": "compile-failed",
                "diagnostics": diagnostics,
            },
            "cases": [],
        }),
    )
}

fn write_machine_report(
    path: &Path,
    results: &[TestResult],
    options: &TestOptions,
) -> Result<(), CliFailure> {
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
                "cause": result.cause.as_ref().map(|cause| serde_json::json!({
                    "kind": cause.kind,
                    "descriptor": cause.descriptor,
                    "message": cause.message,
                    "details": cause.details,
                    "source_frames": cause.source_frames,
                })),
                "stdout": result.stdout.bytes,
                "stdout_truncated": result.stdout.truncated,
                "stderr": result.stderr.bytes,
                "stderr_truncated": result.stderr.truncated,
            })
        })
        .collect::<Vec<_>>();
    let report = serde_json::json!({
        "schema_version": REPORT_SCHEMA_VERSION,
        "run": report_run_metadata(options, "completed"),
        "cases": cases,
    });
    write_report_value(path, &report)
}

fn write_report_value(path: &Path, report: &serde_json::Value) -> Result<(), CliFailure> {
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
        serde_json::to_vec_pretty(report).expect("test report is serializable"),
    )
    .map_err(|error| CliFailure::backend(format!("cannot write test report: {error}")))
}
