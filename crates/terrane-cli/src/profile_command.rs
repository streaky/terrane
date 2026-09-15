use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, ExitStatus, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use terrane_compiler::debugging::DebugInformation;
use terrane_compiler::profiling::{
    ATTRIBUTION_SCHEMA_VERSION, ArgumentPolicy, CapturedModule, CollectionConditions,
    CollectionLoss, CollectorIdentity, CpuEvidence, CpuSample, EvidenceKind, EvidenceUnit,
    MAX_CAPTURED_SAMPLES, MAX_STACK_DEPTH, NativeFrame, NativeSourceLocation, PrivacyDeclaration,
    ProfileArtifact, SCHEMA_VERSION,
};
use terrane_compiler::provenance::{BuildIdentity, BuildProvenance, hash_bytes};

use super::CliFailure;

const DEFAULT_FREQUENCY_HZ: u32 = 999;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RecordOptions {
    pub input: PathBuf,
    pub output: PathBuf,
    pub program_arguments: Vec<OsString>,
    pub retain_arguments: bool,
    pub embed_sources: bool,
}

pub(super) fn parse_record(arguments: &[OsString]) -> Result<RecordOptions, CliFailure> {
    if arguments.get(1).and_then(|value| value.to_str()) != Some("record") {
        return Err(CliFailure::usage_with(
            "expected `terrane profile record` or `terrane profile show`",
        ));
    }
    let mut index = 2;
    let mut cpu = false;
    let mut output = None;
    let mut retain_arguments = false;
    let mut embed_sources = false;
    while let Some(argument) = arguments.get(index).and_then(|value| value.to_str()) {
        match argument {
            "--cpu" => cpu = true,
            "--retain-arguments" => retain_arguments = true,
            "--embed-sources" => embed_sources = true,
            "-o" | "--output" if output.is_none() => {
                index += 1;
                output = Some(
                    arguments
                        .get(index)
                        .map(PathBuf::from)
                        .ok_or_else(CliFailure::usage)?,
                );
            }
            _ => break,
        }
        index += 1;
    }
    if !cpu {
        return Err(CliFailure::usage_with(
            "`terrane profile record` currently requires `--cpu`",
        ));
    }
    let input = arguments
        .get(index)
        .map(PathBuf::from)
        .ok_or_else(CliFailure::usage)?;
    index += 1;
    let program_arguments = if index == arguments.len() {
        Vec::new()
    } else if arguments
        .get(index)
        .is_some_and(|argument| argument == "--")
    {
        arguments[index + 1..].to_vec()
    } else {
        return Err(CliFailure::usage());
    };
    Ok(RecordOptions {
        input,
        output: output.unwrap_or_else(|| PathBuf::from("profile.trnprof")),
        program_arguments,
        retain_arguments,
        embed_sources,
    })
}

#[derive(Debug)]
struct ParsedPerfEvidence {
    modules: Vec<CapturedModule>,
    samples: Vec<CpuSample>,
    lost_events: u64,
    truncated_events: u64,
}

#[expect(
    clippy::too_many_arguments,
    reason = "recording binds one compiled artifact, its compiler metadata, and explicit CLI policy"
)]
pub(super) fn record(
    options: &RecordOptions,
    package: &terrane_compiler::Package,
    debug: DebugInformation,
    executable: &Path,
    build_root: &Path,
    build_identity: BuildIdentity,
) -> Result<ExitCode, CliFailure> {
    require_supported_host(&build_identity.target)?;
    let perf_version = perf_version()?;
    let provenance = BuildProvenance::create(package, executable, build_root, build_identity)
        .map_err(CliFailure::backend)?;
    let raw_capture = build_root.join(format!("terrane-profile-{}.data", std::process::id()));
    let started = Instant::now();
    let status = run_perf(executable, &options.program_arguments, &raw_capture)?;
    let elapsed = started.elapsed();
    let parsed = parse_capture(
        &raw_capture,
        executable,
        &provenance.native_module.content_hash,
    );
    let _ = fs::remove_file(&raw_capture);
    let evidence = parsed?;
    if evidence.samples.is_empty() && !status.success() {
        return Err(CliFailure::backend(format!(
            "perf collection failed with {status} before recording usable CPU samples"
        )));
    }
    let (exit_code, terminating_signal) = status_parts(&status);
    let retained_arguments = options
        .retain_arguments
        .then(|| {
            options
                .program_arguments
                .iter()
                .map(|argument| argument.to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    let artifact = ProfileArtifact {
        schema_version: SCHEMA_VERSION.to_owned(),
        evidence_kind: EvidenceKind::CpuSamples,
        evidence_unit: EvidenceUnit::SampleCount,
        attribution_schema_version: ATTRIBUTION_SCHEMA_VERSION.to_owned(),
        provenance,
        source_attribution: debug,
        collector: CollectorIdentity {
            name: "linux-perf".to_owned(),
            version: perf_version,
            raw_configuration: perf_record_configuration(executable, &options.program_arguments),
        },
        conditions: CollectionConditions {
            host: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
            target: "x86_64-unknown-linux-gnu".to_owned(),
            build_profile: terrane_compiler::profiling::CPU_ARTIFACT_PROFILE
                .id
                .to_owned(),
            workload: executable.to_string_lossy().into_owned(),
            arguments: retained_arguments,
            argument_policy: if options.retain_arguments {
                ArgumentPolicy::Retained
            } else {
                ArgumentPolicy::Omitted
            },
            included_processes: "launched-process-tree".to_owned(),
            included_threads: "all".to_owned(),
            sample_frequency_hz: DEFAULT_FREQUENCY_HZ,
            sample_period: "frequency-derived".to_owned(),
            elapsed_nanoseconds: duration_nanoseconds(elapsed),
            active_nanoseconds: duration_nanoseconds(elapsed),
            warmup_nanoseconds: None,
            process_exit_code: exit_code,
            terminating_signal,
            interrupted: terminating_signal.is_some(),
        },
        privacy: PrivacyDeclaration {
            source_paths: true,
            symbol_names: true,
            arguments: if options.retain_arguments {
                ArgumentPolicy::Retained
            } else {
                ArgumentPolicy::Omitted
            },
            timing: true,
            embedded_sources: options.embed_sources,
        },
        evidence: CpuEvidence {
            loss: CollectionLoss {
                lost_events: evidence.lost_events,
                captured_events: evidence.samples.len() as u64,
                truncated_events: evidence.truncated_events,
            },
            modules: evidence.modules,
            samples: evidence.samples,
        },
    };
    artifact.validate().map_err(CliFailure::backend)?;
    write_artifact(&options.output, &artifact)?;
    eprintln!(
        "recorded {} CPU samples ({} lost, {} truncated) in {}",
        artifact.evidence.samples.len(),
        artifact.evidence.loss.lost_events,
        artifact.evidence.loss.truncated_events,
        options.output.display()
    );
    Ok(ExitCode::from(
        exit_code
            .and_then(|code| u8::try_from(code).ok())
            .unwrap_or_else(|| terminating_signal.map_or(1, |signal| 128 + signal as u8)),
    ))
}

fn duration_nanoseconds(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

fn require_supported_host(target: &str) -> Result<(), CliFailure> {
    if cfg!(target_os = "linux")
        && cfg!(target_arch = "x86_64")
        && target == "x86_64-unknown-linux-gnu"
    {
        return Ok(());
    }
    Err(CliFailure::backend(format!(
        "CPU profiling supports Linux x86-64 host and target; found {}-{} targeting {target}",
        std::env::consts::OS,
        std::env::consts::ARCH
    )))
}

fn perf_version() -> Result<String, CliFailure> {
    let output = Command::new("perf")
        .arg("--version")
        .output()
        .map_err(|error| {
            CliFailure::backend(format!(
                "Linux perf is required for CPU profiling but could not be started: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(CliFailure::backend(
            "Linux perf did not report a usable version".to_owned(),
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn perf_record_configuration(executable: &Path, arguments: &[OsString]) -> Vec<String> {
    let mut configuration = vec![
        "record".to_owned(),
        "-e".to_owned(),
        "cpu-clock:u".to_owned(),
        "-F".to_owned(),
        DEFAULT_FREQUENCY_HZ.to_string(),
        "--call-graph".to_owned(),
        "dwarf,8192".to_owned(),
        "--buildid-all".to_owned(),
        "--".to_owned(),
        executable.to_string_lossy().into_owned(),
    ];
    configuration.extend(
        arguments
            .iter()
            .map(|argument| argument.to_string_lossy().into_owned()),
    );
    configuration
}

fn run_perf(
    executable: &Path,
    arguments: &[OsString],
    output: &Path,
) -> Result<ExitStatus, CliFailure> {
    let mut command = Command::new("perf");
    command
        .args(["record", "-q", "-o"])
        .arg(output)
        .args([
            "-e",
            "cpu-clock:u",
            "-F",
            &DEFAULT_FREQUENCY_HZ.to_string(),
            "--call-graph",
            "dwarf,8192",
            "--buildid-all",
            "--",
        ])
        .arg(executable)
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt as _;
        command.process_group(0);
    }
    let mut child = command.spawn().map_err(|error| {
        CliFailure::backend(format!("failed to start Linux perf collector: {error}"))
    })?;
    let signals = install_signal_flags()?;
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| CliFailure::backend(format!("failed to wait for perf: {error}")))?
        {
            return Ok(status);
        }
        #[cfg(unix)]
        for (signal, flag) in &signals {
            if flag.swap(false, Ordering::SeqCst) {
                let pid = i32::try_from(child.id()).map_err(|_| {
                    CliFailure::backend("perf process identity exceeds host range".to_owned())
                })?;
                nix::sys::signal::killpg(
                    nix::unistd::Pid::from_raw(pid),
                    nix::sys::signal::Signal::try_from(*signal).map_err(|error| {
                        CliFailure::backend(format!("cannot forward profiling signal: {error}"))
                    })?,
                )
                .map_err(|error| {
                    CliFailure::backend(format!(
                        "cannot forward signal to profile process: {error}"
                    ))
                })?;
            }
        }
        thread::sleep(Duration::from_millis(20));
    }
}

fn install_signal_flags() -> Result<Vec<(i32, Arc<AtomicBool>)>, CliFailure> {
    #[cfg(unix)]
    let signal_numbers = [
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGHUP,
        signal_hook::consts::SIGQUIT,
    ];
    #[cfg(not(unix))]
    let signal_numbers = [];
    signal_numbers
        .into_iter()
        .map(|signal| {
            let flag = Arc::new(AtomicBool::new(false));
            signal_hook::flag::register(signal, Arc::clone(&flag)).map_err(|error| {
                CliFailure::backend(format!("cannot install profiling signal handler: {error}"))
            })?;
            Ok((signal, flag))
        })
        .collect()
}

fn status_parts(status: &ExitStatus) -> (Option<i32>, Option<i32>) {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt as _;
        (status.code(), status.signal())
    }
    #[cfg(not(unix))]
    {
        (status.code(), None)
    }
}

fn parse_capture(
    capture: &Path,
    executable: &Path,
    executable_hash: &str,
) -> Result<ParsedPerfEvidence, CliFailure> {
    let build_ids = perf_build_ids(capture)?;
    let output = Command::new("perf")
        .args(["script", "-i"])
        .arg(capture)
        .args([
            "--inline",
            "--full-source-path",
            "--show-lost-events",
            "--ns",
            "-F",
            "pid,tid,time,event,ip,dsoff,sym,symoff,dso,srcline",
        ])
        .output()
        .map_err(|error| CliFailure::backend(format!("failed to decode perf capture: {error}")))?;
    if !output.status.success() {
        return Err(CliFailure::backend(format!(
            "perf could not decode CPU capture: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    parse_perf_script(
        &String::from_utf8_lossy(&output.stdout),
        build_ids,
        executable,
        executable_hash,
    )
    .map_err(CliFailure::backend)
}

fn perf_build_ids(capture: &Path) -> Result<BTreeMap<String, String>, CliFailure> {
    let output = Command::new("perf")
        .args(["buildid-list", "-i"])
        .arg(capture)
        .output()
        .map_err(|error| {
            CliFailure::backend(format!("failed to read perf module identities: {error}"))
        })?;
    if !output.status.success() {
        return Err(CliFailure::backend(format!(
            "perf could not list captured module identities: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let (build_id, path) = line.split_once(' ')?;
            Some((path.trim().to_owned(), build_id.to_owned()))
        })
        .collect())
}

fn parse_perf_script(
    script: &str,
    build_ids: BTreeMap<String, String>,
    executable: &Path,
    executable_hash: &str,
) -> Result<ParsedPerfEvidence, String> {
    let executable = executable
        .canonicalize()
        .unwrap_or_else(|_| executable.to_path_buf());
    let mut module_indices = BTreeMap::new();
    let mut modules = Vec::new();
    let mut samples = Vec::new();
    let mut current: Option<CpuSample> = None;
    let mut lost_events = 0_u64;
    let mut truncated_events = 0_u64;
    for line in script.lines() {
        if line.contains("LOST") && line.contains("events") {
            lost_events = lost_events.saturating_add(first_decimal(line).unwrap_or(1));
            continue;
        }
        if !line.starts_with(char::is_whitespace) && line.contains(": ") {
            if let Some(sample) = current.take() {
                samples.push(sample);
            }
            if samples.len() == MAX_CAPTURED_SAMPLES {
                truncated_events = truncated_events.saturating_add(1);
                current = None;
                continue;
            }
            let mut fields = line.split_whitespace();
            let identity = fields
                .next()
                .ok_or_else(|| format!("cannot decode perf process identity from `{line}`"))?;
            let (process_id, thread_id) = if let Some((process, thread)) = identity.split_once('/')
            {
                (process.parse().ok(), thread.parse().ok())
            } else {
                (
                    identity.parse().ok(),
                    fields.next().and_then(|value| value.parse().ok()),
                )
            };
            let process_id = process_id
                .ok_or_else(|| format!("cannot decode perf process identity from `{line}`"))?;
            let thread_id = thread_id
                .ok_or_else(|| format!("cannot decode perf thread identity from `{line}`"))?;
            let timestamp = fields
                .next()
                .and_then(|value| value.trim_end_matches(':').parse::<f64>().ok())
                .ok_or_else(|| format!("cannot decode perf timestamp from `{line}`"))?;
            current = Some(CpuSample {
                process_id,
                thread_id,
                monotonic_nanoseconds: (timestamp * 1_000_000_000.0).max(0.0) as u64,
                stack: Vec::new(),
                unreadable: false,
            });
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some(sample) = current.as_mut() else {
            continue;
        };
        if let Some((module_path, module_offset, symbol, symbol_offset)) = parse_frame(trimmed) {
            if sample.stack.len() == MAX_STACK_DEPTH {
                truncated_events = truncated_events.saturating_add(1);
                continue;
            }
            let module = if let Some(index) = module_indices.get(&module_path) {
                *index
            } else {
                let path = PathBuf::from(&module_path);
                let canonical = path.canonicalize().unwrap_or(path);
                let is_profiled_executable = canonical == executable;
                let content_hash = if is_profiled_executable {
                    executable_hash.to_owned()
                } else {
                    fs::read(&canonical)
                        .map(|bytes| hash_bytes(&bytes))
                        .unwrap_or_else(|_| "unavailable".to_owned())
                };
                let index = modules.len();
                modules.push(CapturedModule {
                    path: module_path.clone(),
                    build_id: build_ids
                        .get(&module_path)
                        .cloned()
                        .unwrap_or_else(|| "unavailable".to_owned()),
                    content_hash,
                    is_profiled_executable,
                });
                module_indices.insert(module_path, index);
                index
            };
            sample.stack.push(NativeFrame {
                module,
                module_offset,
                symbol,
                symbol_offset,
                generated_location: None,
                inline: false,
            });
        } else if let Some(location) = parse_source_location(trimmed)
            && let Some(frame) = sample.stack.last_mut()
        {
            frame.generated_location = Some(location);
        }
    }
    if let Some(sample) = current {
        samples.push(sample);
    }
    for sample in &mut samples {
        if sample.stack.is_empty() {
            sample.unreadable = true;
        }
    }
    Ok(ParsedPerfEvidence {
        modules,
        samples,
        lost_events,
        truncated_events,
    })
}

fn parse_frame(line: &str) -> Option<(String, u64, Option<String>, Option<u64>)> {
    let (prefix, module) = line.rsplit_once(" (")?;
    let module = module.strip_suffix(')')?;
    let (module_path, module_offset) = module.rsplit_once("+0x")?;
    let module_offset = u64::from_str_radix(module_offset, 16).ok()?;
    let mut fields = prefix.split_whitespace();
    let _instruction = fields.next()?;
    let symbol = fields.collect::<Vec<_>>().join(" ");
    let (symbol, symbol_offset) = if let Some((name, offset)) = symbol.rsplit_once("+0x") {
        (
            (name != "[unknown]").then(|| name.to_owned()),
            u64::from_str_radix(offset, 16).ok(),
        )
    } else {
        ((symbol != "[unknown]").then_some(symbol), None)
    };
    Some((module_path.to_owned(), module_offset, symbol, symbol_offset))
}

fn parse_source_location(line: &str) -> Option<NativeSourceLocation> {
    let (path, line) = line.rsplit_once(':')?;
    let line = line.parse().ok()?;
    Some(NativeSourceLocation {
        path: path.to_owned(),
        line,
        column: None,
    })
}

fn first_decimal(line: &str) -> Option<u64> {
    line.split(|character: char| !character.is_ascii_digit())
        .find(|field| !field.is_empty())?
        .parse()
        .ok()
}
pub(super) fn show(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let path = arguments
        .get(2)
        .map(PathBuf::from)
        .ok_or_else(CliFailure::usage)?;
    let mut format = "text";
    let mut limit = 50_usize;
    let mut index = 3;
    while let Some(argument) = arguments.get(index).and_then(|value| value.to_str()) {
        match argument {
            "--format" => {
                index += 1;
                format = arguments
                    .get(index)
                    .and_then(|value| value.to_str())
                    .filter(|value| matches!(*value, "text" | "json"))
                    .ok_or_else(CliFailure::usage)?;
            }
            "--limit" => {
                index += 1;
                limit = arguments
                    .get(index)
                    .and_then(|value| value.to_str())
                    .and_then(|value| value.parse().ok())
                    .filter(|limit| *limit > 0)
                    .ok_or_else(CliFailure::usage)?;
            }
            _ => return Err(CliFailure::usage()),
        }
        index += 1;
    }
    let artifact = read_artifact(&path)?;
    let mut costs = BTreeMap::<String, u64>::new();
    for sample in &artifact.evidence.samples {
        let label = sample
            .stack
            .first()
            .and_then(|frame| frame.symbol.as_deref())
            .unwrap_or("<native unavailable>");
        *costs.entry(label.to_owned()).or_default() += 1;
    }
    let mut rows = costs.into_iter().collect::<Vec<_>>();
    rows.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
    rows.truncate(limit);
    if format == "json" {
        let rows = rows
            .iter()
            .map(|(name, samples)| {
                serde_json::json!({
                    "frame": name,
                    "exclusive_samples": samples,
                    "percent_of_captured": (*samples as f64 * 100.0)
                        / artifact.evidence.loss.captured_events.max(1) as f64
                })
            })
            .collect::<Vec<_>>();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "schema_version": artifact.schema_version,
                "evidence_kind": artifact.evidence_kind,
                "unit": artifact.evidence_unit,
                "captured_events": artifact.evidence.loss.captured_events,
                "lost_events": artifact.evidence.loss.lost_events,
                "truncated_events": artifact.evidence.loss.truncated_events,
                "rows": rows
            }))
            .map_err(|error| CliFailure::backend(format!("cannot render profile JSON: {error}")))?
        );
    } else {
        println!(
            "CPU samples: {} captured, {} lost, {} truncated",
            artifact.evidence.loss.captured_events,
            artifact.evidence.loss.lost_events,
            artifact.evidence.loss.truncated_events
        );
        println!("exclusive  captured  native frame");
        for (name, samples) in rows {
            println!(
                "{samples:>9}  {:>7.2}%  {name}",
                samples as f64 * 100.0 / artifact.evidence.loss.captured_events.max(1) as f64
            );
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn read_artifact(path: &Path) -> Result<ProfileArtifact, CliFailure> {
    const MAX_ARTIFACT_BYTES: u64 = 512 * 1024 * 1024;
    let file = File::open(path).map_err(|error| {
        CliFailure::backend(format!(
            "cannot open profile artifact {}: {error}",
            path.display()
        ))
    })?;
    let length = file
        .metadata()
        .map_err(|error| {
            CliFailure::backend(format!(
                "cannot inspect profile artifact {}: {error}",
                path.display()
            ))
        })?
        .len();
    if length > MAX_ARTIFACT_BYTES {
        return Err(CliFailure::backend(format!(
            "profile artifact is {length} bytes; limit is {MAX_ARTIFACT_BYTES}"
        )));
    }
    let artifact: ProfileArtifact =
        serde_json::from_reader(BufReader::new(file)).map_err(|error| {
            CliFailure::backend(format!(
                "cannot decode profile artifact {}: {error}",
                path.display()
            ))
        })?;
    artifact.validate().map_err(CliFailure::backend)?;
    Ok(artifact)
}

fn write_artifact(path: &Path, artifact: &ProfileArtifact) -> Result<(), CliFailure> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|error| {
            CliFailure::backend(format!(
                "cannot create profile output directory {}: {error}",
                parent.display()
            ))
        })?;
    }
    let temporary = path.with_extension("trnprof.tmp");
    let file = File::create(&temporary).map_err(|error| {
        CliFailure::backend(format!(
            "cannot create profile artifact {}: {error}",
            temporary.display()
        ))
    })?;
    serde_json::to_writer_pretty(BufWriter::new(file), artifact)
        .map_err(|error| CliFailure::backend(format!("cannot encode profile artifact: {error}")))?;
    fs::rename(&temporary, path).map_err(|error| {
        CliFailure::backend(format!(
            "cannot publish profile artifact {}: {error}",
            path.display()
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_parser_separates_privacy_options_and_workload_arguments() {
        let options = parse_record(&[
            "profile".into(),
            "record".into(),
            "--cpu".into(),
            "--output".into(),
            "capture.trnprof".into(),
            "--retain-arguments".into(),
            "app".into(),
            "--".into(),
            "secret".into(),
        ])
        .unwrap();
        assert_eq!(options.input, Path::new("app"));
        assert_eq!(options.output, Path::new("capture.trnprof"));
        assert_eq!(options.program_arguments, [OsString::from("secret")]);
        assert!(options.retain_arguments);
    }

    #[test]
    fn perf_script_parser_retains_module_offsets_and_source_locations() {
        let script = "123/123 10.250000000: cpu-clock:u: \n\
\t    400123 hot+0x3 (/tmp/program+0x123)\n\
  /tmp/build/src/main.rs:42\n\
\t    7f00 [unknown] (/usr/lib/libc.so.6+0x100)\n\n";
        let evidence = parse_perf_script(
            script,
            BTreeMap::from([
                ("/tmp/program".to_owned(), "program-id".to_owned()),
                ("/usr/lib/libc.so.6".to_owned(), "libc-id".to_owned()),
            ]),
            Path::new("/tmp/program"),
            "sha256:program",
        )
        .unwrap();
        assert_eq!(evidence.samples.len(), 1);
        assert_eq!(evidence.samples[0].stack.len(), 2);
        assert_eq!(evidence.samples[0].stack[0].module_offset, 0x123);
        assert_eq!(
            evidence.samples[0].stack[0]
                .generated_location
                .as_ref()
                .unwrap()
                .line,
            42
        );
        assert_eq!(evidence.modules[0].build_id, "program-id");
        assert!(evidence.modules[0].is_profiled_executable);
    }
}
