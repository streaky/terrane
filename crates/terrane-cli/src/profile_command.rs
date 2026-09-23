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
    ATTRIBUTION_SCHEMA_VERSION, AllocationEvent, AllocationEvidence, AllocationSite,
    ArgumentPolicy, CapturedModule, CollectionConditions, CollectionLoss, CollectorIdentity,
    CpuEvidence, CpuSample, DEFAULT_MEMORY_INTERVAL, Disclosure, EvidenceKind, EvidenceUnit,
    MAX_ARTIFACT_BYTES, MAX_CAPTURED_SAMPLES, MAX_STACK_DEPTH, MemoryTimelineEvidence, NativeFrame,
    NativeSourceLocation, PrivacyDeclaration, ProcessMemorySample, ProfileArtifact, SCHEMA_VERSION,
};
use terrane_compiler::provenance::{BuildIdentity, BuildProvenance, hash_bytes};

use super::CliFailure;

const DEFAULT_FREQUENCY_HZ: u32 = 999;

#[derive(Clone, Debug, Eq, PartialEq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "privacy and auxiliary capture switches are independent"
)]
pub(super) struct RecordOptions {
    pub input: PathBuf,
    pub output: PathBuf,
    pub program_arguments: Vec<OsString>,
    pub retain_arguments: bool,
    pub embed_sources: bool,
    pub capture_cpu: bool,
    pub capture_memory_timeline: bool,
    pub capture_allocations: bool,
}

pub(super) fn parse_record(arguments: &[OsString]) -> Result<RecordOptions, CliFailure> {
    if arguments.get(1).and_then(|value| value.to_str()) != Some("record") {
        return Err(CliFailure::usage_with(
            "expected `terrane profile record` or `terrane profile show`",
        ));
    }
    let mut index = 2;
    let mut cpu = false;
    let mut memory_timeline = false;
    let mut allocations = false;
    let mut output = None;
    let mut retain_arguments = false;
    let mut embed_sources = false;
    while let Some(argument) = arguments.get(index).and_then(|value| value.to_str()) {
        match argument {
            "--cpu" => cpu = true,
            "--memory-timeline" => memory_timeline = true,
            "--allocations" => allocations = true,
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
            argument if argument.starts_with('-') => {
                return Err(CliFailure::usage_with(format!(
                    "unknown profile record option `{argument}`"
                )));
            }
            _ => break,
        }
        index += 1;
    }
    if !cpu && !allocations && !memory_timeline {
        return Err(CliFailure::usage_with(
            "`terrane profile record` requires `--cpu`, `--allocations`, or `--memory-timeline`",
        ));
    }
    if cpu && allocations {
        return Err(CliFailure::usage_with(
            "`--cpu` and `--allocations` are separate primary evidence kinds",
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
        capture_cpu: cpu,
        capture_memory_timeline: memory_timeline,
        capture_allocations: allocations,
    })
}

#[derive(Debug)]
struct ParsedPerfEvidence {
    modules: Vec<CapturedModule>,
    samples: Vec<CpuSample>,
    lost_events: u64,
    dropped_samples: u64,
    dropped_frames: u64,
}

#[derive(Debug)]
struct CapturedRun {
    status: ExitStatus,
    elapsed: Duration,
    evidence: ParsedPerfEvidence,
}

#[expect(
    clippy::too_many_lines,
    reason = "capture ownership and cleanup remain visible in one orchestration boundary"
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
    let collector_version = if options.capture_cpu {
        perf_version()?
    } else if options.capture_allocations {
        command_version("heaptrack", "--version")?
    } else {
        "procfs".to_owned()
    };
    let provenance = BuildProvenance::create(package, executable, build_root, build_identity)
        .map_err(CliFailure::backend)?;
    let raw_capture = build_root.join(format!("terrane-profile-{}.data", std::process::id()));
    let started = Instant::now();
    let (status, timeline, allocations) = if options.capture_cpu {
        let (status, timeline) = run_perf(
            executable,
            &options.program_arguments,
            &raw_capture,
            options.capture_memory_timeline,
        )?;
        (status, timeline, None)
    } else if options.capture_allocations {
        let allocation_capture = raw_capture.with_extension("heaptrack.zst");
        let (status, timeline) = run_heaptrack(
            executable,
            &options.program_arguments,
            &allocation_capture,
            options.capture_memory_timeline,
        )?;
        let allocations = parse_heaptrack_capture(&allocation_capture, build_root)?;
        let _ = fs::remove_file(&allocation_capture);
        (status, timeline, Some(allocations))
    } else {
        let (status, timeline) = run_memory_timeline(executable, &options.program_arguments)?;
        (status, timeline, None)
    };
    let elapsed = started.elapsed();
    let parsed = if options.capture_cpu {
        parse_capture(
            &raw_capture,
            executable,
            &provenance.native_module.content_hash,
        )
    } else {
        Ok(ParsedPerfEvidence {
            modules: vec![CapturedModule {
                path: executable.to_string_lossy().into_owned(),
                build_id: "not-collected".to_owned(),
                content_hash: provenance.native_module.content_hash.clone(),
                is_profiled_executable: true,
            }],
            samples: Vec::new(),
            lost_events: 0,
            dropped_samples: 0,
            dropped_frames: 0,
        })
    };
    let _ = fs::remove_file(&raw_capture);
    let evidence = parsed?;
    if options.capture_cpu && evidence.samples.is_empty() && !status.success() {
        return Err(CliFailure::backend(format!(
            "perf collection failed with {status} before recording usable CPU samples"
        )));
    }
    let capture = CapturedRun {
        status,
        elapsed,
        evidence,
    };
    let exit_code = command_exit_code(capture.status);
    let mut artifact = assemble_artifact(
        options,
        debug,
        executable,
        provenance,
        collector_version,
        capture,
        timeline,
        allocations,
    );
    artifact.fit_encoded_budget().map_err(CliFailure::backend)?;
    artifact.validate().map_err(CliFailure::backend)?;
    write_artifact(&options.output, &artifact)?;
    if options.capture_cpu {
        eprintln!(
            "recorded {} CPU samples ({} lost, {} samples dropped, {} frames dropped) in {}",
            artifact.evidence.samples.len(),
            artifact.evidence.loss.lost_events,
            artifact.evidence.loss.dropped_samples,
            artifact.evidence.loss.dropped_frames,
            options.output.display()
        );
    } else if let Some(allocations) = &artifact.allocations {
        eprintln!(
            "recorded {} allocations ({} allocated, {} retained at exit, {} peak live) in {}",
            allocations.allocation_count,
            format_bytes(allocations.allocated_bytes),
            format_bytes(allocations.retained_bytes_at_exit),
            format_bytes(allocations.peak_live_bytes),
            options.output.display()
        );
    } else {
        eprintln!(
            "recorded {} process-memory samples ({} missed intervals) in {}",
            artifact
                .memory_timeline
                .as_ref()
                .map_or(0, |timeline| timeline.samples.len()),
            artifact
                .memory_timeline
                .as_ref()
                .map_or(0, |timeline| timeline.missed_intervals),
            options.output.display()
        );
    }
    Ok(ExitCode::from(exit_code))
}

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "artifact assembly records the complete collector contract in one place"
)]
fn assemble_artifact(
    options: &RecordOptions,
    debug: DebugInformation,
    executable: &Path,
    provenance: BuildProvenance,
    collector_version: String,
    capture: CapturedRun,
    timeline: Option<MemoryTimelineEvidence>,
    allocations: Option<AllocationEvidence>,
) -> ProfileArtifact {
    let (exit_code, terminating_signal) = status_parts(capture.status);
    let retained_arguments = if options.retain_arguments {
        options
            .program_arguments
            .iter()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect()
    } else {
        Vec::new()
    };
    ProfileArtifact {
        schema_version: SCHEMA_VERSION.to_owned(),
        evidence_kind: if options.capture_cpu {
            EvidenceKind::CpuSamples
        } else if options.capture_allocations {
            EvidenceKind::Allocations
        } else {
            EvidenceKind::MemoryTimeline
        },
        evidence_unit: if options.capture_cpu {
            EvidenceUnit::SampleCount
        } else {
            EvidenceUnit::Bytes
        },
        attribution_schema_version: ATTRIBUTION_SCHEMA_VERSION.to_owned(),
        provenance,
        source_attribution: debug,
        collector: CollectorIdentity {
            name: if options.capture_cpu {
                "linux-perf"
            } else if options.capture_allocations {
                "heaptrack"
            } else {
                "linux-procfs"
            }
            .to_owned(),
            version: collector_version,
            raw_configuration: if options.capture_cpu {
                perf_record_configuration(
                    executable,
                    &options.program_arguments,
                    options.retain_arguments,
                )
            } else if options.capture_allocations {
                vec![
                    "heaptrack --record-only -o <capture> <workload> [arguments omitted by default]"
                        .to_owned(),
                ]
            } else {
                vec![format!(
                    "procfs interval={}ns",
                    duration_nanoseconds(DEFAULT_MEMORY_INTERVAL)
                )]
            },
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
            sample_frequency_hz: if options.capture_cpu {
                DEFAULT_FREQUENCY_HZ
            } else {
                0
            },
            sample_period: if options.capture_cpu {
                "frequency-derived"
            } else {
                "not-applicable"
            }
            .to_owned(),
            elapsed_nanoseconds: duration_nanoseconds(capture.elapsed),
            active_nanoseconds: duration_nanoseconds(capture.elapsed),
            warmup_nanoseconds: None,
            process_exit_code: exit_code,
            terminating_signal,
            interrupted: terminating_signal.is_some(),
        },
        privacy: PrivacyDeclaration {
            source_paths: Disclosure::Included,
            symbol_names: Disclosure::Included,
            arguments: if options.retain_arguments {
                ArgumentPolicy::Retained
            } else {
                ArgumentPolicy::Omitted
            },
            timing: Disclosure::Included,
            authored_sources: if options.embed_sources {
                Disclosure::Included
            } else {
                Disclosure::Omitted
            },
            compiler_sources: Disclosure::Included,
            generated_sources: Disclosure::Included,
        },
        evidence: CpuEvidence {
            loss: CollectionLoss {
                lost_events: capture.evidence.lost_events,
                captured_events: capture.evidence.samples.len() as u64,
                dropped_samples: capture.evidence.dropped_samples,
                dropped_frames: capture.evidence.dropped_frames,
            },
            modules: capture.evidence.modules,
            samples: capture.evidence.samples,
        },
        memory_timeline: timeline,
        allocations,
    }
}

fn command_exit_code(status: ExitStatus) -> u8 {
    let (exit_code, terminating_signal) = status_parts(status);
    exit_code
        .and_then(|code| u8::try_from(code).ok())
        .or_else(|| {
            terminating_signal
                .and_then(|signal| u8::try_from(signal).ok())
                .and_then(|signal| 128_u8.checked_add(signal))
        })
        .unwrap_or(1)
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

fn perf_program() -> OsString {
    std::env::var_os("TERRANE_PERF").unwrap_or_else(|| OsString::from("perf"))
}

fn perf_version() -> Result<String, CliFailure> {
    let output = Command::new(perf_program())
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

fn command_version(program: &str, argument: &str) -> Result<String, CliFailure> {
    let output = Command::new(program)
        .arg(argument)
        .output()
        .map_err(|error| {
            CliFailure::backend(format!(
                "{program} is required for profiling but could not be started: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(CliFailure::backend(format!(
            "{program} did not report a usable version"
        )));
    }
    let text = if output.stdout.is_empty() {
        &output.stderr
    } else {
        &output.stdout
    };
    Ok(String::from_utf8_lossy(text).trim().to_owned())
}

fn perf_record_configuration(
    executable: &Path,
    arguments: &[OsString],
    retain_arguments: bool,
) -> Vec<String> {
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
    if retain_arguments {
        configuration.extend(
            arguments
                .iter()
                .map(|argument| argument.to_string_lossy().into_owned()),
        );
    } else if !arguments.is_empty() {
        configuration.push("<arguments omitted>".to_owned());
    }
    configuration
}

fn run_perf(
    executable: &Path,
    arguments: &[OsString],
    output: &Path,
    capture_memory_timeline: bool,
) -> Result<(ExitStatus, Option<MemoryTimelineEvidence>), CliFailure> {
    let signals = install_signal_flags()?;
    let mut command = Command::new(perf_program());
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
    let started = Instant::now();
    let mut timeline = capture_memory_timeline.then(|| MemoryTimelineEvidence {
        sampling_interval_nanoseconds: duration_nanoseconds(DEFAULT_MEMORY_INTERVAL),
        missed_intervals: 0,
        samples: Vec::new(),
    });
    loop {
        if let Some(timeline) = &mut timeline {
            sample_process_memory(child.id(), started, timeline);
        }
        if let Some(status) = child
            .try_wait()
            .map_err(|error| CliFailure::backend(format!("failed to wait for perf: {error}")))?
        {
            return Ok((status, timeline));
        }
        #[cfg(unix)]
        for entry in &signals.0 {
            if entry.flag.swap(false, Ordering::SeqCst) {
                let pid = i32::try_from(child.id()).map_err(|_| {
                    CliFailure::backend("perf process identity exceeds host range".to_owned())
                })?;
                nix::sys::signal::killpg(
                    nix::unistd::Pid::from_raw(pid),
                    nix::sys::signal::Signal::try_from(entry.signal).map_err(|error| {
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
        thread::sleep(DEFAULT_MEMORY_INTERVAL);
    }
}

fn run_heaptrack(
    executable: &Path,
    arguments: &[OsString],
    output: &Path,
    capture_memory_timeline: bool,
) -> Result<(ExitStatus, Option<MemoryTimelineEvidence>), CliFailure> {
    let mut child = Command::new("heaptrack")
        .arg("--record-only")
        .arg("--output")
        .arg(output.with_extension(""))
        .arg(executable)
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| {
            CliFailure::backend(format!(
                "heaptrack is required for allocation profiling but could not be started: {error}"
            ))
        })?;
    let started = Instant::now();
    let mut timeline = capture_memory_timeline.then(|| MemoryTimelineEvidence {
        sampling_interval_nanoseconds: duration_nanoseconds(DEFAULT_MEMORY_INTERVAL),
        missed_intervals: 0,
        samples: Vec::new(),
    });
    loop {
        if let Some(timeline) = &mut timeline {
            sample_process_memory(child.id(), started, timeline);
        }
        if let Some(status) = child.try_wait().map_err(|error| {
            CliFailure::backend(format!("failed to wait for heaptrack: {error}"))
        })? {
            if !output.is_file() {
                return Err(CliFailure::backend(format!(
                    "heaptrack exited with {status} without producing allocation evidence"
                )));
            }
            return Ok((status, timeline));
        }
        thread::sleep(DEFAULT_MEMORY_INTERVAL);
    }
}

fn parse_heaptrack_capture(
    capture: &Path,
    build_root: &Path,
) -> Result<AllocationEvidence, CliFailure> {
    let summary = Command::new("heaptrack_print")
        .args(["--file"])
        .arg(capture)
        .args(["--peak-limit", "1", "--sub-peak-limit", "1"])
        .output()
        .map_err(|error| {
            CliFailure::backend(format!("failed to analyze heaptrack capture: {error}"))
        })?;
    if !summary.status.success() {
        return Err(CliFailure::backend(format!(
            "heaptrack_print rejected allocation capture: {}",
            String::from_utf8_lossy(&summary.stderr).trim()
        )));
    }
    let summary = String::from_utf8_lossy(&summary.stdout);
    let allocation_count = summary_metric(&summary, "calls to allocation functions:")
        .ok_or_else(|| CliFailure::backend("heaptrack omitted allocation count".to_owned()))?;
    let temporary_allocation_count =
        summary_metric(&summary, "temporary memory allocations:").unwrap_or(0);
    let retained_bytes_at_exit = summary_bytes(&summary, "total memory leaked:").unwrap_or(0);
    let raw = raw_heaptrack_traffic(capture)?;
    let mut sites = BTreeMap::<Vec<String>, AllocationSite>::new();
    for (cost, field) in [
        ("allocations", "allocation_count"),
        ("leaked", "retained_bytes"),
        ("peak", "peak_live_bytes"),
    ] {
        let folded = build_root.join(format!("heaptrack-{cost}-{}.folded", std::process::id()));
        let status = Command::new("heaptrack_print")
            .args(["--file"])
            .arg(capture)
            .args(["--flamegraph-cost-type", cost, "--print-flamegraph"])
            .arg(&folded)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|error| {
                CliFailure::backend(format!("failed to render heaptrack {cost} stacks: {error}"))
            })?;
        if status.success() {
            merge_folded_allocation_sites(&folded, field, &mut sites)?;
        }
        let _ = fs::remove_file(folded);
    }
    let summary_mismatch =
        allocation_count != raw.allocation_count || retained_bytes_at_exit != raw.retained_bytes;
    Ok(AllocationEvidence {
        allocation_count: raw.allocation_count,
        allocated_bytes: raw.allocated_bytes,
        freed_bytes: raw.freed_bytes,
        temporary_allocation_count,
        retained_bytes_at_exit: raw.retained_bytes,
        peak_live_bytes: raw.peak_live_bytes,
        unmatched_transitions: raw.unmatched_transitions,
        partial: raw.partial || summary_mismatch,
        collector_data_format: "heaptrack-1.5-normalized-events-v1".to_owned(),
        sites: sites.into_values().collect(),
        events: raw.events,
    })
}

fn summary_metric(summary: &str, label: &str) -> Option<u64> {
    summary.lines().find_map(|line| {
        let value = line.trim().strip_prefix(label)?.trim();
        value
            .split_whitespace()
            .next()?
            .replace(',', "")
            .parse()
            .ok()
    })
}

fn summary_bytes(summary: &str, label: &str) -> Option<u64> {
    let value = summary
        .lines()
        .find_map(|line| line.trim().strip_prefix(label).map(str::trim))?;
    parse_human_bytes(value.split_whitespace().next()?)
}

fn parse_human_bytes(value: &str) -> Option<u64> {
    let split = value
        .find(|character: char| !character.is_ascii_digit() && character != '.')
        .unwrap_or(value.len());
    let (whole, fraction) = value[..split]
        .split_once('.')
        .unwrap_or((&value[..split], ""));
    let whole = whole.parse::<u64>().ok()?;
    let (fraction_value, fraction_scale) = fraction
        .bytes()
        .take(3)
        .filter(u8::is_ascii_digit)
        .fold((0_u64, 1_u64), |(value, scale), digit| {
            (value * 10 + u64::from(digit - b'0'), scale * 10)
        });
    let multiplier = match &value[split..] {
        "B" | "" => 1_u64,
        "K" | "KB" => 1024,
        "M" | "MB" => 1024 * 1024,
        "G" | "GB" => 1024 * 1024 * 1024,
        _ => return None,
    };
    whole.checked_mul(multiplier)?.checked_add(
        fraction_value
            .checked_mul(multiplier)?
            .checked_add(fraction_scale / 2)?
            .checked_div(fraction_scale)?,
    )
}

#[derive(Debug)]
struct RawAllocationTraffic {
    allocation_count: u64,
    allocated_bytes: u64,
    freed_bytes: u64,
    retained_bytes: u64,
    peak_live_bytes: u64,
    unmatched_transitions: u64,
    partial: bool,
    events: Vec<AllocationEvent>,
}

fn raw_heaptrack_traffic(capture: &Path) -> Result<RawAllocationTraffic, CliFailure> {
    let output = Command::new("zstdcat")
        .arg(capture)
        .output()
        .map_err(|error| {
            CliFailure::backend(format!(
                "failed to decode heaptrack allocation events: {error}"
            ))
        })?;
    if !output.status.success() {
        return Err(CliFailure::backend(
            "zstdcat rejected heaptrack allocation evidence".to_owned(),
        ));
    }
    Ok(parse_raw_heaptrack_events(&String::from_utf8_lossy(
        &output.stdout,
    )))
}

fn parse_raw_heaptrack_events(raw: &str) -> RawAllocationTraffic {
    let mut allocation_info = Vec::<(u64, u64)>::new();
    let mut live = BTreeMap::<u64, Vec<usize>>::new();
    let mut events = Vec::new();
    let mut timestamp = 0_u64;
    let mut live_bytes = 0_u64;
    let mut peak_live_bytes = 0_u64;
    let mut allocated_bytes = 0_u64;
    let mut freed_bytes = 0_u64;
    let mut unmatched_transitions = 0_u64;
    let mut partial = false;
    for line in raw.lines() {
        let mut fields = line.split_whitespace();
        match fields.next() {
            Some("a") => {
                let Some(size) = fields.next().and_then(parse_hex) else {
                    partial = true;
                    continue;
                };
                let Some(trace_index) = fields.next().and_then(parse_hex) else {
                    partial = true;
                    continue;
                };
                allocation_info.push((size, trace_index));
            }
            Some("+") => {
                let Some(info_index) = fields.next().and_then(parse_hex) else {
                    partial = true;
                    continue;
                };
                let Some(&(size_bytes, trace_index)) = usize::try_from(info_index)
                    .ok()
                    .and_then(|index| allocation_info.get(index))
                else {
                    unmatched_transitions = unmatched_transitions.saturating_add(1);
                    partial = true;
                    continue;
                };
                allocated_bytes = allocated_bytes.saturating_add(size_bytes);
                live_bytes = live_bytes.saturating_add(size_bytes);
                peak_live_bytes = peak_live_bytes.max(live_bytes);
                if events.len() == MAX_CAPTURED_SAMPLES {
                    partial = true;
                    continue;
                }
                let event_index = events.len();
                events.push(AllocationEvent {
                    allocation_id: event_index as u64,
                    size_bytes,
                    alignment_bytes: None,
                    process_id: None,
                    thread_id: None,
                    monotonic_timestamp: timestamp,
                    trace_index,
                    freed_at: None,
                });
                live.entry(info_index).or_default().push(event_index);
            }
            Some("-") => {
                let Some(info_index) = fields.next().and_then(parse_hex) else {
                    partial = true;
                    continue;
                };
                let Some(event_index) = live.get_mut(&info_index).and_then(Vec::pop) else {
                    unmatched_transitions = unmatched_transitions.saturating_add(1);
                    partial = true;
                    continue;
                };
                let event = &mut events[event_index];
                event.freed_at = Some(timestamp);
                freed_bytes = freed_bytes.saturating_add(event.size_bytes);
                live_bytes = live_bytes.saturating_sub(event.size_bytes);
            }
            Some("c") => {
                if let Some(value) = fields.next().and_then(parse_hex) {
                    timestamp = value;
                } else {
                    partial = true;
                }
            }
            _ => {}
        }
    }
    RawAllocationTraffic {
        allocation_count: events.len() as u64,
        allocated_bytes,
        freed_bytes,
        retained_bytes: live_bytes,
        peak_live_bytes,
        unmatched_transitions,
        partial,
        events,
    }
}

fn parse_hex(value: &str) -> Option<u64> {
    u64::from_str_radix(value, 16).ok()
}

fn merge_folded_allocation_sites(
    path: &Path,
    field: &str,
    sites: &mut BTreeMap<Vec<String>, AllocationSite>,
) -> Result<(), CliFailure> {
    let folded = fs::read_to_string(path).map_err(|error| {
        CliFailure::backend(format!(
            "cannot read heaptrack folded stacks `{}`: {error}",
            path.display()
        ))
    })?;
    for line in folded.lines() {
        let Some((stack, value)) = line.rsplit_once(' ') else {
            continue;
        };
        let Ok(value) = value.parse::<u64>() else {
            continue;
        };
        let stack = stack.split(';').map(str::to_owned).collect::<Vec<_>>();
        let site = sites
            .entry(stack.clone())
            .or_insert_with(|| AllocationSite {
                stack,
                allocation_count: 0,
                allocated_bytes: 0,
                retained_bytes: 0,
                peak_live_bytes: 0,
            });
        match field {
            "allocation_count" => site.allocation_count = value,
            "retained_bytes" => site.retained_bytes = value,
            "peak_live_bytes" => site.peak_live_bytes = value,
            _ => unreachable!("known heaptrack cost field"),
        }
    }
    Ok(())
}

fn run_memory_timeline(
    executable: &Path,
    arguments: &[OsString],
) -> Result<(ExitStatus, Option<MemoryTimelineEvidence>), CliFailure> {
    let mut child = Command::new(executable)
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| {
            CliFailure::backend(format!("failed to start profiled workload: {error}"))
        })?;
    let started = Instant::now();
    let mut timeline = MemoryTimelineEvidence {
        sampling_interval_nanoseconds: duration_nanoseconds(DEFAULT_MEMORY_INTERVAL),
        missed_intervals: 0,
        samples: Vec::new(),
    };
    loop {
        sample_process_memory(child.id(), started, &mut timeline);
        if let Some(status) = child
            .try_wait()
            .map_err(|error| CliFailure::backend(format!("failed to wait for workload: {error}")))?
        {
            return Ok((status, Some(timeline)));
        }
        thread::sleep(DEFAULT_MEMORY_INTERVAL);
    }
}

fn sample_process_memory(process_id: u32, started: Instant, timeline: &mut MemoryTimelineEvidence) {
    let Some(sample) = read_process_memory(process_id, started) else {
        timeline.missed_intervals = timeline.missed_intervals.saturating_add(1);
        return;
    };
    if timeline.samples.len() == MAX_CAPTURED_SAMPLES {
        timeline.missed_intervals = timeline.missed_intervals.saturating_add(1);
    } else {
        timeline.samples.push(sample);
    }
}

fn read_process_memory(process_id: u32, started: Instant) -> Option<ProcessMemorySample> {
    let process_id = profiled_child_process(process_id).unwrap_or(process_id);
    let status = fs::read_to_string(format!("/proc/{process_id}/status")).ok()?;
    let value = |name| {
        status.lines().find_map(|line| {
            let value = line.strip_prefix(name)?.split_whitespace().next()?;
            value.parse::<u64>().ok()?.checked_mul(1024)
        })
    };
    Some(ProcessMemorySample {
        monotonic_nanoseconds: duration_nanoseconds(started.elapsed()),
        process_id,
        rss_bytes: value("VmRSS:"),
        pss_bytes: None,
        private_bytes: None,
        shared_bytes: value("RssFile:"),
        anonymous_bytes: value("RssAnon:"),
        file_backed_bytes: value("RssFile:"),
        minor_faults: None,
        major_faults: None,
    })
}

struct SignalFlag {
    signal: i32,
    flag: Arc<AtomicBool>,
    registration: Option<signal_hook::SigId>,
}

struct SignalFlags(Vec<SignalFlag>);

impl Drop for SignalFlags {
    fn drop(&mut self) {
        for entry in &mut self.0 {
            if let Some(registration) = entry.registration.take() {
                signal_hook::low_level::unregister(registration);
            }
        }
    }
}

fn profiled_child_process(process_id: u32) -> Option<u32> {
    let children =
        fs::read_to_string(format!("/proc/{process_id}/task/{process_id}/children")).ok()?;
    children.split_whitespace().next()?.parse().ok()
}

fn install_signal_flags() -> Result<SignalFlags, CliFailure> {
    #[cfg(unix)]
    let signal_numbers = [
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGHUP,
        signal_hook::consts::SIGQUIT,
    ];
    #[cfg(not(unix))]
    let signal_numbers = [];
    let mut flags = SignalFlags(Vec::new());
    for signal in signal_numbers {
        let flag = Arc::new(AtomicBool::new(false));
        let registration =
            signal_hook::flag::register(signal, Arc::clone(&flag)).map_err(|error| {
                CliFailure::backend(format!("cannot install profiling signal handler: {error}"))
            })?;
        flags.0.push(SignalFlag {
            signal,
            flag,
            registration: Some(registration),
        });
    }
    Ok(flags)
}

fn status_parts(status: ExitStatus) -> (Option<i32>, Option<i32>) {
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
    let output = Command::new(perf_program())
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
        &build_ids,
        executable,
        executable_hash,
    )
    .map_err(CliFailure::backend)
}

fn perf_build_ids(capture: &Path) -> Result<BTreeMap<String, String>, CliFailure> {
    let output = Command::new(perf_program())
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
    build_ids: &BTreeMap<String, String>,
    executable: &Path,
    executable_hash: &str,
) -> Result<ParsedPerfEvidence, String> {
    let executable = executable
        .canonicalize()
        .unwrap_or_else(|_| executable.to_path_buf());
    let mut captured_modules = CapturedModules::new(build_ids, &executable, executable_hash);
    let mut samples = Vec::new();
    let mut current: Option<CpuSample> = None;
    let mut lost_events = 0_u64;
    let mut dropped_samples = 0_u64;
    let mut dropped_frames = 0_u64;
    let mut retained_sample_bytes = 0_u64;
    let mut pending_inline = Vec::<PendingInlineFrame>::new();
    for line in script.lines() {
        if let Some(lost) = parse_lost_events(line) {
            lost_events = lost_events.saturating_add(lost);
            continue;
        }
        let header = line.trim_start();
        if is_perf_sample_header(header) {
            dropped_frames = dropped_frames.saturating_add(pending_inline.len() as u64);
            pending_inline.clear();
            if let Some(sample) = current.take() {
                retain_sample(
                    sample,
                    &mut samples,
                    &mut retained_sample_bytes,
                    &mut dropped_samples,
                    (MAX_CAPTURED_SAMPLES, MAX_ARTIFACT_BYTES),
                )?;
            }
            current = Some(parse_sample_header(header)?);
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some(sample) = current.as_mut() else {
            continue;
        };
        match parse_frame(trimmed) {
            Some(ParsedFrame::Inline(frame)) => pending_inline.push(frame),
            Some(ParsedFrame::Concrete(frame)) => record_concrete_frame(
                frame,
                sample,
                &mut pending_inline,
                &mut captured_modules,
                &mut dropped_frames,
            ),
            None => {
                if let Some((location, inline)) = parse_source_location(trimmed) {
                    if inline {
                        if let Some(frame) = pending_inline.last_mut() {
                            frame.generated_location = Some(location);
                        }
                    } else if let Some(frame) = sample.stack.last_mut()
                        && captured_modules.modules[frame.module].is_profiled_executable
                    {
                        frame.generated_location = Some(location);
                    }
                }
            }
        }
    }
    dropped_frames = dropped_frames.saturating_add(pending_inline.len() as u64);
    if let Some(sample) = current {
        retain_sample(
            sample,
            &mut samples,
            &mut retained_sample_bytes,
            &mut dropped_samples,
            (MAX_CAPTURED_SAMPLES, MAX_ARTIFACT_BYTES),
        )?;
    }
    for sample in &mut samples {
        if sample.stack.is_empty() {
            sample.unreadable = true;
        }
    }
    captured_modules.ensure_profiled_executable();
    let modules = captured_modules.modules;
    Ok(ParsedPerfEvidence {
        modules,
        samples,
        lost_events,
        dropped_samples,
        dropped_frames,
    })
}
#[derive(Default)]
struct ByteCounter(u64);

impl std::io::Write for ByteCounter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        self.0 = self
            .0
            .checked_add(buffer.len() as u64)
            .ok_or_else(|| std::io::Error::other("profile sample size overflow"))?;
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn retain_sample(
    sample: CpuSample,
    samples: &mut Vec<CpuSample>,
    retained_bytes: &mut u64,
    dropped_samples: &mut u64,
    limits: (usize, u64),
) -> Result<(), String> {
    let mut counter = ByteCounter::default();
    serde_json::to_writer(&mut counter, &sample)
        .map_err(|error| format!("cannot measure captured CPU sample: {error}"))?;
    let next_bytes = retained_bytes
        .checked_add(counter.0)
        .and_then(|bytes| bytes.checked_add(u64::from(!samples.is_empty())))
        .ok_or_else(|| "profile sample size overflow".to_owned())?;
    if samples.len() == limits.0 || next_bytes > limits.1 {
        *dropped_samples = dropped_samples.saturating_add(1);
    } else {
        samples.push(sample);
        *retained_bytes = next_bytes;
    }
    Ok(())
}

fn is_perf_sample_header(line: &str) -> bool {
    let mut fields = line.split_whitespace();
    let Some((process_id, thread_id)) = fields.next().and_then(|value| value.split_once('/'))
    else {
        return false;
    };
    process_id.parse::<u32>().is_ok()
        && thread_id.parse::<u32>().is_ok()
        && fields
            .next()
            .and_then(|value| value.strip_suffix(':'))
            .and_then(parse_timestamp_nanoseconds)
            .is_some()
}

fn parse_sample_header(line: &str) -> Result<CpuSample, String> {
    let mut fields = line.split_whitespace();
    let identity = fields
        .next()
        .ok_or_else(|| format!("cannot decode perf process identity from `{line}`"))?;
    let (process_id, thread_id) = if let Some((process, thread)) = identity.split_once('/') {
        (process.parse().ok(), thread.parse().ok())
    } else {
        (
            identity.parse().ok(),
            fields.next().and_then(|value| value.parse().ok()),
        )
    };
    let process_id =
        process_id.ok_or_else(|| format!("cannot decode perf process identity from `{line}`"))?;
    let thread_id =
        thread_id.ok_or_else(|| format!("cannot decode perf thread identity from `{line}`"))?;
    let timestamp = fields
        .next()
        .and_then(|value| parse_timestamp_nanoseconds(value.trim_end_matches(':')))
        .ok_or_else(|| format!("cannot decode perf timestamp from `{line}`"))?;
    Ok(CpuSample {
        process_id,
        thread_id,
        monotonic_nanoseconds: timestamp,
        stack: Vec::new(),
        unreadable: false,
    })
}

fn parse_timestamp_nanoseconds(value: &str) -> Option<u64> {
    let (seconds, fraction) = value.split_once('.').unwrap_or((value, ""));
    let seconds = seconds.parse::<u64>().ok()?;
    let mut fraction = fraction.chars().take(9).collect::<String>();
    fraction.extend(std::iter::repeat_n('0', 9 - fraction.len()));
    seconds
        .checked_mul(1_000_000_000)?
        .checked_add(fraction.parse().ok()?)
}

struct CapturedModules<'a> {
    indices: BTreeMap<String, usize>,
    modules: Vec<CapturedModule>,
    build_ids: &'a BTreeMap<String, String>,
    executable: &'a Path,
    executable_hash: &'a str,
}

impl<'a> CapturedModules<'a> {
    fn new(
        build_ids: &'a BTreeMap<String, String>,
        executable: &'a Path,
        executable_hash: &'a str,
    ) -> Self {
        Self {
            indices: BTreeMap::new(),
            modules: Vec::new(),
            build_ids,
            executable,
            executable_hash,
        }
    }

    fn index(&mut self, module_path: &str) -> usize {
        if let Some(index) = self.indices.get(module_path) {
            return *index;
        }
        let path = PathBuf::from(module_path);
        let canonical = path.canonicalize().unwrap_or(path);
        let is_profiled_executable = canonical == self.executable;
        let content_hash = if is_profiled_executable {
            self.executable_hash.to_owned()
        } else if let Ok(bytes) = fs::read(&canonical) {
            hash_bytes(&bytes)
        } else {
            "unavailable".to_owned()
        };
        let index = self.modules.len();
        self.modules.push(CapturedModule {
            path: module_path.to_owned(),
            build_id: self
                .build_ids
                .get(module_path)
                .cloned()
                .unwrap_or_else(|| "unavailable".to_owned()),
            content_hash,
            is_profiled_executable,
        });
        self.indices.insert(module_path.to_owned(), index);
        index
    }

    fn ensure_profiled_executable(&mut self) {
        if self
            .modules
            .iter()
            .any(|module| module.is_profiled_executable)
        {
            return;
        }
        let path = self.executable.to_string_lossy().into_owned();
        self.modules.push(CapturedModule {
            build_id: self
                .build_ids
                .get(&path)
                .cloned()
                .unwrap_or_else(|| "unavailable".to_owned()),
            path,
            content_hash: self.executable_hash.to_owned(),
            is_profiled_executable: true,
        });
    }
}

fn record_concrete_frame(
    frame: ConcreteFrame,
    sample: &mut CpuSample,
    pending_inline: &mut Vec<PendingInlineFrame>,
    modules: &mut CapturedModules<'_>,
    dropped_frames: &mut u64,
) {
    let module = modules.index(&frame.module_path);
    let is_profiled_executable = modules.modules[module].is_profiled_executable;
    for inline in pending_inline.drain(..) {
        if inline.instruction == frame.instruction {
            push_native_frame(
                sample,
                NativeFrame {
                    module,
                    module_offset: frame.module_offset,
                    symbol: inline.symbol,
                    symbol_offset: inline.symbol_offset,
                    generated_location: inline
                        .generated_location
                        .filter(|_| is_profiled_executable),
                    inline: true,
                },
                dropped_frames,
            );
        } else {
            *dropped_frames = dropped_frames.saturating_add(1);
        }
    }
    push_native_frame(
        sample,
        NativeFrame {
            module,
            module_offset: frame.module_offset,
            symbol: frame.symbol,
            symbol_offset: frame.symbol_offset,
            generated_location: None,
            inline: false,
        },
        dropped_frames,
    );
}

#[derive(Debug)]
enum ParsedFrame {
    Concrete(ConcreteFrame),
    Inline(PendingInlineFrame),
}

#[derive(Debug)]
struct ConcreteFrame {
    instruction: u64,
    module_path: String,
    module_offset: u64,
    symbol: Option<String>,
    symbol_offset: Option<u64>,
}

#[derive(Debug)]
struct PendingInlineFrame {
    instruction: u64,
    symbol: Option<String>,
    symbol_offset: Option<u64>,
    generated_location: Option<NativeSourceLocation>,
}

fn parse_frame(line: &str) -> Option<ParsedFrame> {
    if let Some((prefix, module)) = line.rsplit_once(" (")
        && let Some(module) = module.strip_suffix(')')
        && let Some((module_path, module_offset)) = module.rsplit_once("+0x")
    {
        let (instruction, symbol, symbol_offset) = parse_instruction_and_symbol(prefix)?;
        return Some(ParsedFrame::Concrete(ConcreteFrame {
            instruction,
            module_path: module_path.to_owned(),
            module_offset: u64::from_str_radix(module_offset, 16).ok()?,
            symbol,
            symbol_offset,
        }));
    }
    let (instruction, symbol, symbol_offset) = parse_instruction_and_symbol(line)?;
    Some(ParsedFrame::Inline(PendingInlineFrame {
        instruction,
        symbol,
        symbol_offset,
        generated_location: None,
    }))
}

fn parse_instruction_and_symbol(line: &str) -> Option<(u64, Option<String>, Option<u64>)> {
    let mut fields = line.split_whitespace();
    let instruction = u64::from_str_radix(fields.next()?.trim_start_matches("0x"), 16).ok()?;
    let symbol = fields.collect::<Vec<_>>().join(" ");
    let (symbol, symbol_offset) = if let Some((name, offset)) = symbol.rsplit_once("+0x") {
        (
            (name != "[unknown]").then(|| name.to_owned()),
            u64::from_str_radix(offset, 16).ok(),
        )
    } else {
        ((symbol != "[unknown]").then_some(symbol), None)
    };
    Some((instruction, symbol, symbol_offset))
}

fn parse_source_location(line: &str) -> Option<(NativeSourceLocation, bool)> {
    let (line, inline) = line
        .strip_suffix(" (inlined)")
        .map_or((line, false), |line| (line, true));
    let (path, line) = line.rsplit_once(':')?;
    let line = line.parse().ok()?;
    if path == "??" || line == 0 {
        return None;
    }
    Some((
        NativeSourceLocation {
            path: path.to_owned(),
            line,
            column: None,
        },
        inline,
    ))
}

fn parse_lost_events(line: &str) -> Option<u64> {
    let mut fields = line.split_whitespace();
    while let Some(field) = fields.next() {
        if field == "LOST" {
            return fields.next()?.parse().ok();
        }
    }
    None
}

fn push_native_frame(sample: &mut CpuSample, frame: NativeFrame, dropped_frames: &mut u64) {
    if sample.stack.len() == MAX_STACK_DEPTH {
        *dropped_frames = dropped_frames.saturating_add(1);
    } else {
        sample.stack.push(frame);
    }
}
#[derive(Debug)]
struct ShowOptions {
    path: PathBuf,
    format: String,
    limit: usize,
    focus: Option<(PathBuf, usize)>,
    generated: bool,
    native: bool,
    source_root: Option<PathBuf>,
    build_root: Option<PathBuf>,
}

pub(super) fn show(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let options = parse_show(arguments)?;
    let artifact = read_artifact(&options.path)?;
    if artifact.evidence_kind == EvidenceKind::MemoryTimeline {
        return show_memory_timeline(&artifact, &options);
    }
    if artifact.evidence_kind == EvidenceKind::Allocations {
        return show_allocations(&artifact, &options);
    }
    let source_root = options
        .source_root
        .clone()
        .unwrap_or_else(|| PathBuf::from(&artifact.provenance.relocation.source_root));
    let build_root = options
        .build_root
        .clone()
        .unwrap_or_else(|| PathBuf::from(&artifact.provenance.relocation.build_root));
    let mut report = terrane_compiler::profiling::attribute(&artifact, &source_root, &build_root);
    let executable = Path::new(&artifact.conditions.workload);
    let relocated_executable = build_root
        .join("artifacts")
        .join("terrane-profile")
        .join(&artifact.provenance.native_module.file_name);
    let executable = if executable.is_file() {
        executable
    } else {
        relocated_executable.as_path()
    };
    if let Err(reason) = artifact.provenance.validate_executable(executable) {
        "reduced-native".clone_into(&mut report.fidelity);
        report.fidelity_reasons.push(reason);
    }
    validate_captured_modules(&artifact, &mut report);
    if let Some((path, line)) = &options.focus {
        report.rows.retain(|row| {
            row.source.as_ref().is_some_and(|source| {
                Path::new(&source.source_uri).ends_with(path)
                    && source.line <= *line
                    && *line <= source.end_line
            })
        });
        let labels = report
            .rows
            .iter()
            .map(|row| row.label.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        retain_call_tree(&mut report.call_tree, &labels);
        report.flame_graph.retain(|stack| {
            stack
                .frames
                .iter()
                .any(|frame| labels.contains(frame.as_str()))
        });
    }
    report.rows.truncate(options.limit);
    limit_row_expansions(&mut report.rows, options.limit);
    relativize_native_modules(
        &mut report.rows,
        &build_root,
        Path::new(&artifact.provenance.relocation.build_root),
    );
    limit_call_tree(&mut report.call_tree, options.limit);
    report.flame_graph.truncate(options.limit);
    if options.format == "json" {
        let mut output_report = report.clone();
        if !options.generated {
            for row in &mut output_report.rows {
                row.generated.clear();
            }
        }
        if !options.native {
            for row in &mut output_report.rows {
                row.native.clear();
            }
        }
        let (memory_timeline, memory_timeline_samples_total) =
            limited_timeline(artifact.memory_timeline.as_ref(), options.limit);
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "schema_version": artifact.schema_version,
                "evidence_kind": artifact.evidence_kind,
                "unit": artifact.evidence_unit,
                "conditions": artifact.conditions,
                "privacy": artifact.privacy,
                "memory_timeline": memory_timeline,
                "memory_timeline_samples_total": memory_timeline_samples_total,
                "report": output_report,
                "generated_expansion": options.generated,
                "native_expansion": options.native
            }))
            .map_err(|error| CliFailure::backend(format!("cannot render profile JSON: {error}")))?
        );
    } else {
        render_text_report(&artifact, &report, &options);
    }
    Ok(ExitCode::SUCCESS)
}
fn show_allocations(
    artifact: &ProfileArtifact,
    options: &ShowOptions,
) -> Result<ExitCode, CliFailure> {
    let allocations = artifact
        .allocations
        .as_ref()
        .expect("validated allocation profile includes allocation evidence");
    if options.format == "json" {
        let mut output = allocations.clone();
        let event_total = output.events.len();
        let site_total = output.sites.len();
        output.events.truncate(options.limit);
        let (memory_timeline, memory_timeline_samples_total) =
            limited_timeline(artifact.memory_timeline.as_ref(), options.limit);
        output.sites.truncate(options.limit);
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "schema_version": artifact.schema_version,
                "evidence_kind": artifact.evidence_kind,
                "unit": artifact.evidence_unit,
                "conditions": artifact.conditions,
                "privacy": artifact.privacy,
                "allocations": output,
                "allocation_events_total": event_total,
                "allocation_sites_total": site_total,
                "memory_timeline": memory_timeline,
                "memory_timeline_samples_total": memory_timeline_samples_total,
            }))
            .map_err(|error| CliFailure::backend(format!("cannot render profile JSON: {error}")))?
        );
        return Ok(ExitCode::SUCCESS);
    }
    println!(
        "allocation traffic: {} allocations, {} allocated, {} freed",
        allocations.allocation_count,
        format_bytes(allocations.allocated_bytes),
        format_bytes(allocations.freed_bytes)
    );
    println!(
        "lifetime: {} temporary allocations, {} retained at exit, {} peak live",
        allocations.temporary_allocation_count,
        format_bytes(allocations.retained_bytes_at_exit),
        format_bytes(allocations.peak_live_bytes)
    );
    println!(
        "transition fidelity: {} ({} unmatched)",
        if allocations.partial {
            "partial"
        } else {
            "complete"
        },
        allocations.unmatched_transitions
    );
    println!("allocation stack                                      count    retained   peak live");
    let mut sites = allocations.sites.iter().collect::<Vec<_>>();
    sites.sort_by_key(|site| std::cmp::Reverse(site.allocation_count));
    for site in sites.into_iter().take(options.limit) {
        println!(
            "{:<52} {:>8} {:>11} {:>11}",
            site.stack.last().map_or("[unknown]", String::as_str),
            site.allocation_count,
            format_bytes(site.retained_bytes),
            format_bytes(site.peak_live_bytes)
        );
    }
    Ok(ExitCode::SUCCESS)
}
fn limited_timeline(
    timeline: Option<&MemoryTimelineEvidence>,
    limit: usize,
) -> (Option<MemoryTimelineEvidence>, usize) {
    let total = timeline.map_or(0, |timeline| timeline.samples.len());
    let mut output = timeline.cloned();
    if let Some(timeline) = &mut output {
        timeline.samples.truncate(limit);
    }
    (output, total)
}

fn show_memory_timeline(
    artifact: &ProfileArtifact,
    options: &ShowOptions,
) -> Result<ExitCode, CliFailure> {
    let timeline = artifact
        .memory_timeline
        .as_ref()
        .expect("validated memory profile includes a timeline");
    if options.format == "json" {
        let mut output = timeline.clone();
        let sample_total = output.samples.len();
        output.samples.truncate(options.limit);
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "schema_version": artifact.schema_version,
                "evidence_kind": artifact.evidence_kind,
                "unit": artifact.evidence_unit,
                "conditions": artifact.conditions,
                "privacy": artifact.privacy,
                "timeline": output,
                "timeline_samples_total": sample_total,
            }))
            .map_err(|error| CliFailure::backend(format!("cannot render profile JSON: {error}")))?
        );
        return Ok(ExitCode::SUCCESS);
    }
    let resident_peak = timeline
        .samples
        .iter()
        .filter_map(|sample| sample.rss_bytes)
        .max();
    let proportional_peak = timeline
        .samples
        .iter()
        .filter_map(|sample| sample.pss_bytes)
        .max();
    println!(
        "process-memory timeline: {} samples, {} missed intervals; {} ms interval",
        timeline.samples.len(),
        timeline.missed_intervals,
        timeline.sampling_interval_nanoseconds / 1_000_000
    );
    println!(
        "RSS peak: {}; PSS peak: {}",
        resident_peak.map_or_else(|| "unavailable".to_owned(), format_bytes),
        proportional_peak.map_or_else(|| "unavailable".to_owned(), format_bytes)
    );
    println!("timestamp (ns)  process  RSS        PSS");
    for sample in timeline.samples.iter().take(options.limit) {
        println!(
            "{:>14}  {:>7}  {:>9}  {:>9}",
            sample.monotonic_nanoseconds,
            sample.process_id,
            sample
                .rss_bytes
                .map_or_else(|| "unavailable".to_owned(), format_bytes),
            sample
                .pss_bytes
                .map_or_else(|| "unavailable".to_owned(), format_bytes),
        );
    }
    Ok(ExitCode::SUCCESS)
}

fn format_bytes(bytes: u64) -> String {
    format!("{bytes} B")
}
fn validate_captured_modules(
    artifact: &ProfileArtifact,
    report: &mut terrane_compiler::profiling::AttributionReport,
) {
    for module in &artifact.evidence.modules {
        if module.is_profiled_executable || module.content_hash == "unavailable" {
            continue;
        }
        let path = Path::new(&module.path);
        let matches = fs::read(path).is_ok_and(|bytes| hash_bytes(&bytes) == module.content_hash);
        if !matches {
            "reduced-native-modules".clone_into(&mut report.native_fidelity);
            report.native_fidelity_reasons.push(format!(
                "captured external module `{}` is missing or changed; native offsets remain visible, but exact module identity is unavailable",
                module.path
            ));
        }
    }
    report.native_fidelity_reasons.sort();
    report.native_fidelity_reasons.dedup();
}

fn parse_show(arguments: &[OsString]) -> Result<ShowOptions, CliFailure> {
    let path = arguments
        .get(2)
        .map(PathBuf::from)
        .ok_or_else(CliFailure::usage)?;
    let mut options = ShowOptions {
        path,
        format: "text".to_owned(),
        limit: 50,
        focus: None,
        generated: false,
        native: false,
        source_root: None,
        build_root: None,
    };
    let mut index = 3;
    while let Some(argument) = arguments.get(index).and_then(|value| value.to_str()) {
        match argument {
            "--generated" => options.generated = true,
            "--native" => options.native = true,
            "--format" => {
                index += 1;
                arguments
                    .get(index)
                    .and_then(|value| value.to_str())
                    .filter(|value| matches!(*value, "text" | "json"))
                    .ok_or_else(CliFailure::usage)?
                    .clone_into(&mut options.format);
            }
            "--limit" => {
                index += 1;
                options.limit = arguments
                    .get(index)
                    .and_then(|value| value.to_str())
                    .and_then(|value| value.parse().ok())
                    .filter(|limit| *limit > 0)
                    .ok_or_else(CliFailure::usage)?;
            }
            "--focus" => {
                index += 1;
                let value = arguments
                    .get(index)
                    .and_then(|value| value.to_str())
                    .ok_or_else(CliFailure::usage)?;
                let (path, line) = value.rsplit_once(':').ok_or_else(CliFailure::usage)?;
                options.focus = Some((
                    PathBuf::from(path),
                    line.parse().map_err(|_| CliFailure::usage())?,
                ));
            }
            "--source-root" => {
                index += 1;
                options.source_root = Some(
                    arguments
                        .get(index)
                        .map(PathBuf::from)
                        .ok_or_else(CliFailure::usage)?,
                );
            }
            "--build-root" => {
                index += 1;
                options.build_root = Some(
                    arguments
                        .get(index)
                        .map(PathBuf::from)
                        .ok_or_else(CliFailure::usage)?,
                );
            }
            _ => return Err(CliFailure::usage()),
        }
        index += 1;
    }
    Ok(options)
}

fn render_text_report(
    artifact: &ProfileArtifact,
    report: &terrane_compiler::profiling::AttributionReport,
    options: &ShowOptions,
) {
    let elapsed_seconds = artifact.conditions.elapsed_nanoseconds / 1_000_000_000;
    let elapsed_milliseconds = artifact.conditions.elapsed_nanoseconds % 1_000_000_000 / 1_000_000;
    println!(
        "CPU samples: {} captured, {} lost, {} samples dropped, {} frames dropped; {} Hz; {}.{:03}s elapsed",
        report.captured_samples,
        report.lost_samples,
        report.dropped_samples,
        report.dropped_frames,
        artifact.conditions.sample_frequency_hz,
        elapsed_seconds,
        elapsed_milliseconds
    );
    println!(
        "collector: {} {}; build: {}; source fidelity: {}; native fidelity: {}",
        artifact.collector.name,
        artifact.collector.version,
        artifact.provenance.artifact_profile,
        report.fidelity,
        report.native_fidelity
    );
    for reason in &report.fidelity_reasons {
        println!("  source fidelity: {reason}");
    }
    for reason in &report.native_fidelity_reasons {
        println!("  native fidelity: {reason}");
    }
    println!("\nexclusive accounting (CPU sample count)");
    for quality in [
        terrane_compiler::profiling::AttributionQuality::ExactAuthored,
        terrane_compiler::profiling::AttributionQuality::SharedOrAmbiguous,
        terrane_compiler::profiling::AttributionQuality::RuntimeAssociated,
        terrane_compiler::profiling::AttributionQuality::GeneratedOnly,
        terrane_compiler::profiling::AttributionQuality::NativeOnly,
        terrane_compiler::profiling::AttributionQuality::Unavailable,
    ] {
        let samples = report.buckets.get(&quality).copied().unwrap_or(0);
        println!(
            "{samples:>9} {:>8}  {}",
            percentage(samples, report.captured_samples),
            quality.label()
        );
    }
    println!("\nhottest source groups (CPU sample count)");
    println!("exclusive inclusive  quality               source group");
    for row in &report.rows {
        println!(
            "{:>9} {:>9}  {:<20}  {}",
            row.exclusive_samples,
            row.inclusive_samples,
            row.quality.label(),
            row.label
        );
        if options.generated {
            for generated in &row.generated {
                println!(
                    "    generated {}:{} bytes {}..{}",
                    generated.path, generated.line, generated.start, generated.end
                );
            }
            render_omitted(
                "generated constituents",
                row.generated_total,
                row.generated.len(),
            );
        }
        for cause in &row.related_causes {
            let uri = artifact
                .source_attribution
                .sources
                .iter()
                .find(|source| source.id == cause.source_id)
                .map_or("<unknown source>", |source| source.uri.as_str());
            println!(
                "    related {uri}:{}:{} bytes {}..{}",
                cause.line, cause.column, cause.start, cause.end
            );
        }
        render_omitted(
            "related source causes",
            row.related_causes_total,
            row.related_causes.len(),
        );
        if options.native {
            for native in &row.native {
                println!(
                    "    native {}+0x{:x} {}",
                    native.module,
                    native.module_offset,
                    native.symbol.as_deref().unwrap_or("<unknown>")
                );
            }
            render_omitted("native constituents", row.native_total, row.native.len());
        }
    }
    println!("\ncall tree (inclusive CPU sample count; root to leaf)");
    render_call_tree(&report.call_tree, 0);
    println!("\nflame graph (folded stacks; weight = CPU sample count)");
    for stack in &report.flame_graph {
        println!("{} {}", stack.frames.join(";"), stack.samples);
    }
}

fn render_omitted(label: &str, total: usize, displayed: usize) {
    if let Some(omitted) = total.checked_sub(displayed).filter(|omitted| *omitted > 0) {
        println!("    ... {omitted} more {label}");
    }
}

fn limit_row_expansions(rows: &mut [terrane_compiler::profiling::AttributionRow], limit: usize) {
    for row in rows {
        row.generated.sort_by(|left, right| {
            (&left.path, left.line, left.start, left.end).cmp(&(
                &right.path,
                right.line,
                right.start,
                right.end,
            ))
        });
        row.related_causes.sort_by_key(|cause| {
            (
                cause.source_id,
                cause.start,
                cause.end,
                cause.line,
                cause.column,
                cause.end_line,
                cause.end_column,
            )
        });
        row.native.sort_by(|left, right| {
            (&left.module, left.module_offset, &left.symbol).cmp(&(
                &right.module,
                right.module_offset,
                &right.symbol,
            ))
        });
        row.generated.truncate(limit);
        row.related_causes.truncate(limit);
        row.native.truncate(limit);
    }
}

fn relativize_native_modules(
    rows: &mut [terrane_compiler::profiling::AttributionRow],
    build_root: &Path,
    recorded_build_root: &Path,
) {
    for native in rows.iter_mut().flat_map(|row| &mut row.native) {
        let path = Path::new(&native.module);
        if let Ok(relative) = path.strip_prefix(build_root) {
            native.module = relative.display().to_string();
        } else if let Ok(relative) = path.strip_prefix(recorded_build_root) {
            native.module = relative.display().to_string();
        }
    }
}

fn retain_call_tree(
    nodes: &mut Vec<terrane_compiler::profiling::CallTreeNode>,
    labels: &std::collections::BTreeSet<&str>,
) {
    nodes.retain_mut(|node| {
        retain_call_tree(&mut node.children, labels);
        labels.contains(node.label.as_str()) || !node.children.is_empty()
    });
}

fn limit_call_tree(nodes: &mut Vec<terrane_compiler::profiling::CallTreeNode>, limit: usize) {
    nodes.truncate(limit);
    for node in nodes {
        limit_call_tree(&mut node.children, limit);
    }
}

fn render_call_tree(nodes: &[terrane_compiler::profiling::CallTreeNode], depth: usize) {
    for node in nodes {
        println!("{:>9}  {}{}", node.samples, "  ".repeat(depth), node.label);
        render_call_tree(&node.children, depth + 1);
    }
}

fn percentage(part: u64, total: u64) -> String {
    let basis_points = part
        .saturating_mul(10_000)
        .checked_div(total.max(1))
        .unwrap_or(0);
    format!("{}.{:02}%", basis_points / 100, basis_points % 100)
}

fn read_artifact(path: &Path) -> Result<ProfileArtifact, CliFailure> {
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
    serde_json::to_writer(BufWriter::new(file), artifact)
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
    fn record_parser_allows_memory_timeline_alone_or_alongside_cpu() {
        let timeline = parse_record(&[
            "profile".into(),
            "record".into(),
            "--memory-timeline".into(),
            "app".into(),
        ])
        .unwrap();
        assert!(!timeline.capture_cpu);
        assert!(timeline.capture_memory_timeline);

        let combined = parse_record(&[
            "profile".into(),
            "record".into(),
            "--cpu".into(),
            "--memory-timeline".into(),
            "app".into(),
        ])
        .unwrap();
        assert!(combined.capture_cpu);
        assert!(combined.capture_memory_timeline);
    }

    #[test]
    fn perf_script_parser_accepts_padded_pids_and_retains_frame_details() {
        let script = " 314007/314007 10.250000000: cpu-clock:u: \n\
\t    400123 hot+0x3\n\
  /tmp/build/src/main.rs:42 (inlined)\n\
\t    400123 inline_parent+0x1 (/tmp/program+0x123)\n\
  /tmp/build/src/main.rs:12\n\
\t    7f00 [unknown] (/usr/lib/libc.so.6+0x100)\n\
  /usr/lib/libc.c:99\n\
\t    7f01 foreign_symbol: helper+0x2 (/usr/lib/libc.so.6+0x101)\n\n\
PERF_RECORD_LOST 1 LOST 37 events\n";
        let evidence = parse_perf_script(
            script,
            &BTreeMap::from([
                ("/tmp/program".to_owned(), "program-id".to_owned()),
                ("/usr/lib/libc.so.6".to_owned(), "libc-id".to_owned()),
            ]),
            Path::new("/tmp/program"),
            "sha256:program",
        )
        .unwrap();
        assert_eq!(evidence.samples[0].stack.len(), 4);
        assert_eq!(evidence.lost_events, 37);
        assert_eq!(evidence.samples[0].stack[0].module_offset, 0x123);
        assert_eq!(
            evidence.samples[0].stack[0]
                .generated_location
                .as_ref()
                .unwrap()
                .line,
            42
        );
        assert!(evidence.samples[0].stack[0].inline);
        assert!(!evidence.samples[0].stack[1].inline);
        assert!(evidence.samples[0].stack[2].generated_location.is_none());
        assert_eq!(
            evidence.samples[0].stack[3].symbol.as_deref(),
            Some("foreign_symbol: helper")
        );
        assert_eq!(evidence.modules[0].build_id, "program-id");
        assert!(evidence.modules[0].is_profiled_executable);
    }

    #[test]
    fn omitted_arguments_do_not_leak_into_collector_configuration() {
        let arguments = [OsString::from("secret-token")];
        let omitted = perf_record_configuration(Path::new("/program"), &arguments, false);
        assert!(!omitted.iter().any(|value| value == "secret-token"));
        assert!(omitted.iter().any(|value| value == "<arguments omitted>"));

        let retained = perf_record_configuration(Path::new("/program"), &arguments, true);
        assert!(retained.iter().any(|value| value == "secret-token"));
    }

    #[test]
    fn show_parser_accepts_source_focus_and_lower_level_expansion() {
        let options = parse_show(&[
            "profile".into(),
            "show".into(),
            "capture.trnprof".into(),
            "--focus".into(),
            "src/α.trn:42".into(),
            "--generated".into(),
            "--native".into(),
            "--limit".into(),
            "7".into(),
        ])
        .unwrap();
        assert_eq!(options.focus, Some((PathBuf::from("src/α.trn"), 42)));
        assert!(options.generated);
        assert!(options.native);
        assert_eq!(options.limit, 7);
    }

    #[test]
    fn sample_and_frame_limits_have_independent_loss_counts() {
        let sample = parse_sample_header("1/1 1.0: cpu-clock:u:").unwrap();
        let mut samples = Vec::new();
        let mut retained_bytes = 0;
        let mut dropped_samples = 0;
        retain_sample(
            sample.clone(),
            &mut samples,
            &mut retained_bytes,
            &mut dropped_samples,
            (1, u64::MAX),
        )
        .unwrap();
        retain_sample(
            sample,
            &mut samples,
            &mut retained_bytes,
            &mut dropped_samples,
            (1, u64::MAX),
        )
        .unwrap();

        let mut byte_limited = Vec::new();
        let mut byte_count = 0;
        retain_sample(
            parse_sample_header("1/1 1.0: cpu-clock:u:").unwrap(),
            &mut byte_limited,
            &mut byte_count,
            &mut dropped_samples,
            (usize::MAX, 0),
        )
        .unwrap();
        assert!(byte_limited.is_empty());
        assert_eq!(dropped_samples, 2);
        assert_eq!(samples.len(), 1);

        let mut sample = samples.pop().unwrap();
        let frame = NativeFrame {
            module: 0,
            module_offset: 0,
            symbol: None,
            symbol_offset: None,
            generated_location: None,
            inline: false,
        };
        let mut dropped_frames = 0;
        for _ in 0..=MAX_STACK_DEPTH {
            push_native_frame(&mut sample, frame.clone(), &mut dropped_frames);
        }
        assert_eq!(sample.stack.len(), MAX_STACK_DEPTH);
        assert_eq!(dropped_frames, 1);
    }

    #[test]
    fn report_limits_bound_every_row_expansion_and_shorten_build_paths() {
        use terrane_compiler::debugging::SourceSpan;
        use terrane_compiler::profiling::{
            AttributionQuality, AttributionRow, GeneratedConstituent, NativeConstituent,
        };

        let mut rows = vec![AttributionRow {
            quality: AttributionQuality::NativeOnly,
            label: "hot".to_owned(),
            source: None,
            related_causes: (0..3)
                .rev()
                .map(|source_id| SourceSpan {
                    source_id,
                    start: 0,
                    end: 1,
                    line: 1,
                    column: 1,
                    end_line: 1,
                    end_column: 2,
                })
                .collect(),
            related_causes_total: 3,
            exclusive_samples: 1,
            inclusive_samples: 1,
            generated: (0..3)
                .rev()
                .map(|line| GeneratedConstituent {
                    path: "src/main.rs".to_owned(),
                    line,
                    start: 0,
                    end: 1,
                })
                .collect(),
            generated_total: 3,
            native: (0..3)
                .rev()
                .map(|module_offset| NativeConstituent {
                    module: "/build/artifacts/terrane-profile/program".to_owned(),
                    module_offset,
                    symbol: None,
                })
                .collect(),
            native_total: 3,
        }];

        limit_row_expansions(&mut rows, 2);
        relativize_native_modules(&mut rows, Path::new("/relocated"), Path::new("/build"));

        assert_eq!(rows[0].generated.len(), 2);
        assert_eq!(rows[0].related_causes.len(), 2);
        assert_eq!(rows[0].native.len(), 2);
        assert_eq!(rows[0].generated_total, 3);
        assert_eq!(rows[0].related_causes_total, 3);
        assert_eq!(rows[0].native_total, 3);
        assert_eq!(rows[0].generated[0].line, 0);
        assert_eq!(rows[0].related_causes[0].source_id, 0);
        assert_eq!(rows[0].native[0].module_offset, 0);
        assert_eq!(
            rows[0].native[0].module,
            "artifacts/terrane-profile/program"
        );
    }
    #[test]
    fn record_parser_keeps_allocation_and_cpu_evidence_separate() {
        let allocations = parse_record(&[
            "profile".into(),
            "record".into(),
            "--allocations".into(),
            "--memory-timeline".into(),
            "app".into(),
        ])
        .unwrap();
        assert!(allocations.capture_allocations);
        assert!(allocations.capture_memory_timeline);
        assert!(!allocations.capture_cpu);

        let error = parse_record(&[
            "profile".into(),
            "record".into(),
            "--cpu".into(),
            "--allocations".into(),
            "app".into(),
        ])
        .unwrap_err();
        assert!(error.message.contains("separate primary evidence kinds"));
    }

    #[test]
    fn parses_heaptrack_summary_units() {
        let summary = "calls to allocation functions: 14 (14000/s)\n\
                       temporary memory allocations: 2 (2000/s)\n\
                       peak heap memory consumption: 75.46K\n\
                       total memory leaked: 544B\n";
        assert_eq!(
            summary_metric(summary, "calls to allocation functions:"),
            Some(14)
        );
        assert_eq!(
            summary_metric(summary, "temporary memory allocations:"),
            Some(2)
        );
        assert_eq!(
            summary_bytes(summary, "peak heap memory consumption:"),
            Some(77_271)
        );
        assert_eq!(summary_bytes(summary, "total memory leaked:"), Some(544));
    }
    #[test]
    fn normalizes_heaptrack_transitions_and_address_reuse() {
        let raw = "v 10500 3\n\
                   a 10 1\n\
                   a 20 2\n\
                   c a\n\
                   + 0\n\
                   + 1\n\
                   c 14\n\
                   - 0\n\
                   c 1e\n\
                   + 0\n\
                   c 28\n\
                   - 1\n";
        let traffic = parse_raw_heaptrack_events(raw);
        assert_eq!(traffic.allocation_count, 3);
        assert_eq!(traffic.allocated_bytes, 0x40);
        assert_eq!(traffic.freed_bytes, 0x30);
        assert_eq!(traffic.retained_bytes, 0x10);
        assert_eq!(traffic.peak_live_bytes, 0x30);
        assert_eq!(traffic.unmatched_transitions, 0);
        assert!(!traffic.partial);
        assert_eq!(traffic.events[0].freed_at, Some(0x14));
        assert_eq!(traffic.events[2].freed_at, None);
    }
}
