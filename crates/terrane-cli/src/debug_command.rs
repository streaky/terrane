use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ffi::OsString;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use terrane_compiler::debugging::{DebugAssociation, ProvenanceManifest};

use super::CliFailure;
use num_bigint::BigUint;
#[path = "debug_command_adapter.rs"]
pub(super) mod adapter;

use adapter::StopContext;
#[path = "debug_command_backend.rs"]
mod backend;
#[path = "debug_command_source.rs"]
mod source;
#[path = "debug_command_steps.rs"]
mod steps;
#[path = "debug_command_variables.rs"]
mod variables;

use backend::{Backend, add_rust_lldb_init_commands, discover_rust_lldb_formatter, event};
use source::{
    frame_stop_context, generated_path, known_source_path, resolve_breakpoint,
    translate_stack_frames,
};
#[cfg(test)]
use steps::parse_lldb_breakpoint_ids;
use steps::step_to_source;
use variables::{read_value_summaries, request_terrane_variables, translate_variables};

const MAX_RAW_STEPS: usize = 64;
const MAX_TEMPORARY_SEQUENCE_POINTS: usize = 512;
static NEXT_COMMAND_FILE: AtomicU64 = AtomicU64::new(1);

pub(super) fn write_provenance(
    build_root: &Path,
    executable: &Path,
    provenance: &ProvenanceManifest,
) -> Result<PathBuf, CliFailure> {
    let mut bytes = serde_json::to_vec_pretty(provenance)
        .map_err(|error| CliFailure::backend(format!("cannot encode debug provenance: {error}")))?;
    bytes.push(b'\n');
    let generated_sidecar = build_root.join("terrane-debug.json");
    let executable_sidecar = executable_sidecar(executable);
    super::write_if_changed(&generated_sidecar, &bytes)
        .map_err(|error| CliFailure::backend(format!("cannot write debug provenance: {error}")))?;
    super::write_if_changed(&executable_sidecar, &bytes).map_err(|error| {
        CliFailure::backend(format!("cannot write executable debug provenance: {error}"))
    })?;
    Ok(generated_sidecar)
}

fn executable_sidecar(executable: &Path) -> PathBuf {
    executable.with_extension(format!(
        "{}terrane-debug.json",
        executable
            .extension()
            .map_or_else(String::new, |extension| format!(
                "{}.",
                extension.to_string_lossy()
            ))
    ))
}

#[expect(
    clippy::too_many_lines,
    reason = "the compact interactive command grammar keeps debugger session state in one loop"
)]
pub(super) fn run_cli(
    executable: &Path,
    sidecar: &Path,
    arguments: &[OsString],
) -> Result<ExitCode, CliFailure> {
    let provenance = load_and_validate(sidecar, executable, None)?;
    let mut backend = Backend::start()?;
    backend.request(
        "initialize",
        json!({
            "adapterID": "terrane",
            "clientID": "terrane-cli",
            "linesStartAt1": true,
            "columnsStartAt1": true,
            "pathFormat": "path"
        }),
    )?;
    let mut backend_arguments = json!({
        "program": executable,
        "args": arguments,
        "cwd": provenance.relocation.source_root,
        "stopOnEntry": true,
        "disableASLR": false
    });
    add_rust_lldb_init_commands(
        &mut backend_arguments,
        discover_rust_lldb_formatter(&provenance.rust_sysroot).as_deref(),
    );
    let (launch_sequence, launch_response) = backend.request_with_timeout(
        "launch",
        backend_arguments,
        Some(Duration::from_millis(500)),
    )?;
    backend.request("configurationDone", json!({}))?;
    if launch_response.is_none() {
        backend.finish_request(launch_sequence, Duration::from_millis(500))?;
    }
    let stopped = backend.wait_for_event(&["stopped", "terminated"])?;
    backend.emit_debuggee_output()?;
    if stopped["event"] != "stopped" {
        return Ok(backend.debuggee_exit_code());
    }
    let mut thread_id = stopped["body"]["threadId"].as_i64().unwrap_or(1);
    let mut selected_frame_index = 0_usize;
    let mut breakpoints = BreakpointManager::default();
    eprintln!("Terrane debugger stopped at entry. Type `help` for commands.");
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        eprint!("(terrane-debug) ");
        io::stderr().flush().map_err(io_failure)?;
        input.clear();
        if stdin.read_line(&mut input).map_err(io_failure)? == 0 {
            backend.disconnect(true)?;
            return Ok(ExitCode::SUCCESS);
        }
        let command = input.trim();
        if command.is_empty() {
            continue;
        }
        match command.split_once(' ').unwrap_or((command, "")) {
            ("help", _) => eprintln!(
                "break <source>:<line> | breakpoints | delete <id|all> | disable <id|all> | enable <id|all> | continue | next | step | out | frames | frame <index> | source [radius] | locals | value <name> | generated [radius] | native | registers | lldb <command> | quit"
            ),
            ("break", location) => {
                let (path, line) = parse_breakpoint(location)?;
                let Some(path) = known_source_path(&provenance, &path) else {
                    return Err(debugger_failure(format!(
                        "{} is not an authored source in this exact debug build",
                        path.display()
                    )));
                };
                let id = breakpoints.add(&provenance, &path, line);
                breakpoints.sync(&mut backend, &provenance)?;
                breakpoints.print(id);
            }
            ("breakpoints", _) => breakpoints.print_all(),
            ("delete", selector) if !selector.is_empty() => {
                breakpoints.remove(selector)?;
                breakpoints.sync(&mut backend, &provenance)?;
            }
            ("disable", selector) if !selector.is_empty() => {
                breakpoints.set_enabled(selector, false)?;
                breakpoints.sync(&mut backend, &provenance)?;
            }
            ("enable", selector) if !selector.is_empty() => {
                breakpoints.set_enabled(selector, true)?;
                breakpoints.sync(&mut backend, &provenance)?;
            }
            ("continue", _) => {
                backend.request("continue", json!({"threadId": thread_id}))?;
                let event = backend.wait_for_event(&["stopped", "terminated"])?;
                backend.emit_debuggee_output()?;
                if event["event"] != "stopped" {
                    return Ok(backend.debuggee_exit_code());
                }
                thread_id = event["body"]["threadId"].as_i64().unwrap_or(thread_id);
                selected_frame_index = 0;
                show_top_frame(&mut backend, &provenance, thread_id)?;
            }
            ("next" | "step" | "out", _) => {
                let request = match command {
                    "next" => "next",
                    "step" => "stepIn",
                    _ => "stepOut",
                };
                let event =
                    step_to_source(&mut backend, &provenance, &breakpoints, request, thread_id)?;
                backend.emit_debuggee_output()?;
                if event["event"] != "stopped" {
                    return Ok(backend.debuggee_exit_code());
                }
                thread_id = event["body"]["threadId"].as_i64().unwrap_or(thread_id);
                selected_frame_index = 0;
                show_top_frame(&mut backend, &provenance, thread_id)?;
            }
            ("frames", _) => show_frames(&mut backend, &provenance, thread_id, false)?,
            ("frame", index) if !index.is_empty() => {
                selected_frame_index = parse_frame_index(index)?;
                show_selected_frame(&mut backend, &provenance, thread_id, selected_frame_index)?;
            }
            ("source", radius) => show_source(
                &mut backend,
                &provenance,
                thread_id,
                selected_frame_index,
                parse_context_radius(radius)?,
            )?,
            ("locals", _) => show_variables(
                &mut backend,
                &provenance,
                thread_id,
                selected_frame_index,
                false,
            )?,
            ("value", name) if !name.is_empty() => show_value(
                &mut backend,
                &provenance,
                thread_id,
                selected_frame_index,
                name,
            )?,
            ("generated", radius) => show_generated(
                &mut backend,
                &provenance,
                thread_id,
                selected_frame_index,
                parse_context_radius(radius)?,
            )?,
            ("native", _) => show_frames(&mut backend, &provenance, thread_id, true)?,
            ("registers", _) => show_variables(
                &mut backend,
                &provenance,
                thread_id,
                selected_frame_index,
                true,
            )?,
            ("lldb", expression) if !expression.is_empty() => {
                let response = backend.request(
                    "evaluate",
                    json!({"expression": format!("`{expression}"), "context": "repl"}),
                )?;
                eprintln!("{}", response["body"]["result"].as_str().unwrap_or(""));
            }
            ("quit" | "exit", _) => {
                backend.disconnect(true)?;
                return Ok(ExitCode::SUCCESS);
            }
            _ => eprintln!("unknown debugger command; type `help`"),
        }
    }
}

#[derive(Clone)]
struct LogicalBreakpoint {
    id: i64,
    source_path: PathBuf,
    requested_line: usize,
    enabled: bool,
    resolutions: Vec<BreakpointResolution>,
    verified: bool,
}

#[derive(Clone, Default)]
struct BreakpointManager {
    next_id: i64,
    by_source: BTreeMap<PathBuf, Vec<LogicalBreakpoint>>,
    backend_files: BTreeSet<String>,
    backend_ids: BTreeMap<i64, BTreeSet<i64>>,
}

impl BreakpointManager {
    fn add(&mut self, provenance: &ProvenanceManifest, path: &Path, line: usize) -> i64 {
        if let Some(existing) = self.by_source.get_mut(path).and_then(|breakpoints| {
            breakpoints
                .iter_mut()
                .find(|item| item.requested_line == line)
        }) {
            existing.enabled = true;
            existing.resolutions = resolve_breakpoint(provenance, path, line);
            return existing.id;
        }
        let id = self.allocate_id();
        self.by_source
            .entry(path.to_path_buf())
            .or_default()
            .push(LogicalBreakpoint {
                id,
                source_path: path.to_path_buf(),
                requested_line: line,
                enabled: true,
                resolutions: resolve_breakpoint(provenance, path, line),
                verified: false,
            });
        id
    }

    fn replace_source(
        &mut self,
        provenance: Option<&ProvenanceManifest>,
        path: PathBuf,
        lines: Vec<usize>,
    ) {
        let mut old_ids = self.by_source.remove(&path).unwrap_or_default();
        let mut replacements = Vec::with_capacity(lines.len());
        for line in lines {
            let id = old_ids
                .iter()
                .position(|item| item.requested_line == line)
                .map_or_else(|| self.allocate_id(), |index| old_ids.remove(index).id);
            replacements.push(LogicalBreakpoint {
                id,
                source_path: path.clone(),
                requested_line: line,
                enabled: true,
                resolutions: provenance
                    .map(|provenance| resolve_breakpoint(provenance, &path, line))
                    .unwrap_or_default(),
                verified: false,
            });
        }
        self.by_source.insert(path, replacements);
    }

    fn resolve_all(&mut self, provenance: &ProvenanceManifest) {
        for breakpoint in self.by_source.values_mut().flatten() {
            breakpoint.resolutions = resolve_breakpoint(
                provenance,
                &breakpoint.source_path,
                breakpoint.requested_line,
            );
            breakpoint.verified = false;
        }
    }

    fn sync(
        &mut self,
        backend: &mut Backend,
        provenance: &ProvenanceManifest,
    ) -> Result<(), CliFailure> {
        let mut grouped = BTreeMap::<String, BTreeMap<usize, BTreeSet<i64>>>::new();
        for breakpoint in self
            .by_source
            .values()
            .flatten()
            .filter(|item| item.enabled)
        {
            for resolution in &breakpoint.resolutions {
                grouped
                    .entry(resolution.generated_path.clone())
                    .or_default()
                    .entry(resolution.generated_line)
                    .or_default()
                    .insert(breakpoint.id);
            }
        }

        let files = self
            .backend_files
            .iter()
            .cloned()
            .chain(grouped.keys().cloned())
            .collect::<BTreeSet<_>>();
        let mut verified = BTreeSet::<i64>::new();
        let mut backend_ids = BTreeMap::<i64, BTreeSet<i64>>::new();
        for file in &files {
            let points = grouped.get(file);
            let lines = points
                .into_iter()
                .flat_map(BTreeMap::keys)
                .copied()
                .collect::<Vec<_>>();
            let backend_response = backend.request(
                "setBreakpoints",
                json!({
                    "source": {"path": generated_path(provenance, file)},
                    "breakpoints": lines.iter().map(|line| json!({"line": line})).collect::<Vec<_>>(),
                    "sourceModified": false
                }),
            )?;
            for (index, line) in lines.iter().enumerate() {
                if backend_response["body"]["breakpoints"][index]["verified"]
                    .as_bool()
                    .unwrap_or(false)
                    && let Some(ids) = points.and_then(|points| points.get(line))
                {
                    verified.extend(ids.iter().copied());
                    if let Some(backend_id) =
                        backend_response["body"]["breakpoints"][index]["id"].as_i64()
                    {
                        for id in ids {
                            backend_ids.entry(*id).or_default().insert(backend_id);
                        }
                    }
                }
            }
        }
        backend
            .events
            .retain(|event| !matches!(event["event"].as_str(), Some("initialized" | "breakpoint")));
        for breakpoint in self.by_source.values_mut().flatten() {
            breakpoint.verified = verified.contains(&breakpoint.id);
        }
        self.backend_ids = backend_ids;
        self.backend_files = grouped.into_keys().collect();
        Ok(())
    }

    fn backend_ids_at(&self, generated_file: &str, generated_line: usize) -> Vec<i64> {
        self.by_source
            .values()
            .flatten()
            .filter(|breakpoint| {
                breakpoint.enabled
                    && breakpoint.resolutions.iter().any(|resolution| {
                        Path::new(generated_file).ends_with(&resolution.generated_path)
                            && resolution.generated_line == generated_line
                    })
            })
            .flat_map(|breakpoint| {
                self.backend_ids
                    .get(&breakpoint.id)
                    .into_iter()
                    .flatten()
                    .copied()
            })
            .collect()
    }

    fn dap_breakpoints(&self, source: &Path) -> Vec<Value> {
        self.by_source
            .get(source)
            .into_iter()
            .flatten()
            .map(LogicalBreakpoint::dap_value)
            .collect()
    }

    fn verification_events(&self) -> Vec<Value> {
        self.by_source
            .values()
            .flatten()
            .map(|breakpoint| {
                event(
                    "breakpoint",
                    json!({
                        "reason": "changed",
                        "breakpoint": breakpoint.dap_value()
                    }),
                )
            })
            .collect()
    }

    fn remove(&mut self, selector: &str) -> Result<(), CliFailure> {
        if selector == "all" {
            self.by_source.clear();
            return Ok(());
        }
        let id = parse_breakpoint_id(selector)?;
        let mut found = false;
        for breakpoints in self.by_source.values_mut() {
            let before = breakpoints.len();
            breakpoints.retain(|breakpoint| breakpoint.id != id);
            found |= breakpoints.len() != before;
        }
        if !found {
            return Err(debugger_failure(format!("no breakpoint #{id}")));
        }
        Ok(())
    }

    fn set_enabled(&mut self, selector: &str, enabled: bool) -> Result<(), CliFailure> {
        let mut found = false;
        for breakpoint in self.by_source.values_mut().flatten() {
            if selector == "all" || parse_breakpoint_id(selector).ok() == Some(breakpoint.id) {
                breakpoint.enabled = enabled;
                found = true;
            }
        }
        if !found {
            return Err(debugger_failure(format!(
                "no breakpoint matching `{selector}`"
            )));
        }
        Ok(())
    }

    fn print(&self, id: i64) {
        if let Some(breakpoint) = self
            .by_source
            .values()
            .flatten()
            .find(|breakpoint| breakpoint.id == id)
        {
            eprintln!("{}", breakpoint.cli_description());
        }
    }

    fn print_all(&self) {
        let mut breakpoints = self.by_source.values().flatten().collect::<Vec<_>>();
        breakpoints.sort_by_key(|breakpoint| breakpoint.id);
        if breakpoints.is_empty() {
            eprintln!("no source breakpoints");
        }
        for breakpoint in breakpoints {
            eprintln!("{}", breakpoint.cli_description());
        }
    }

    fn contains_backend_id(&self, id: i64) -> bool {
        self.backend_ids.values().any(|ids| ids.contains(&id))
    }

    fn allocate_id(&mut self) -> i64 {
        self.next_id += 1;
        self.next_id
    }
}

impl LogicalBreakpoint {
    fn resolved_line(&self) -> usize {
        self.resolutions
            .first()
            .map_or(self.requested_line, |resolution| resolution.source_line)
    }

    fn dap_value(&self) -> Value {
        json!({
            "id": self.id,
            "verified": self.verified,
            "line": self.resolved_line(),
            "source": {"path": self.source_path},
            "message": self.message()
        })
    }

    fn message(&self) -> String {
        let Some(resolution) = self.resolutions.first() else {
            return "pending: no executable sequence point is currently loaded".to_owned();
        };
        let locations = if self.resolutions.len() > 1 {
            format!("; {} native locations", self.resolutions.len())
        } else {
            String::new()
        };
        format!(
            "{} at {}:{}; generated location {}:{}{}",
            resolution.message,
            self.source_path.display(),
            resolution.source_line,
            resolution.generated_path,
            resolution.generated_line,
            locations
        )
    }

    fn cli_description(&self) -> String {
        let state = if !self.enabled {
            "disabled"
        } else if self.verified {
            "verified"
        } else {
            "unverified"
        };
        let Some(resolution) = self.resolutions.first() else {
            return format!(
                "#{:<3} {state} breakpoint {}:{}: pending executable sequence point",
                self.id,
                self.source_path.display(),
                self.requested_line
            );
        };
        let adjusted = if resolution.source_line == self.requested_line {
            String::new()
        } else {
            format!(
                " adjusted to {}:{}",
                self.source_path.display(),
                resolution.source_line
            )
        };
        let locations = if self.resolutions.len() > 1 {
            format!(", {} native locations", self.resolutions.len())
        } else {
            String::new()
        };
        format!(
            "#{:<3} {state} breakpoint {}:{}{} (generated at {}:{}{})",
            self.id,
            self.source_path.display(),
            self.requested_line,
            adjusted,
            resolution.generated_path,
            resolution.generated_line,
            locations
        )
    }
}

fn parse_breakpoint_id(value: &str) -> Result<i64, CliFailure> {
    value
        .trim_start_matches('#')
        .parse()
        .map_err(|_| debugger_failure("breakpoint selector must be an id or `all`"))
}

#[derive(Clone)]
struct BreakpointResolution {
    generated_path: String,
    generated_line: usize,
    source_line: usize,
    message: String,
}

fn show_top_frame(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
) -> Result<(), CliFailure> {
    let response = backend.request(
        "stackTrace",
        json!({"threadId": thread_id, "startFrame": 0, "levels": 1}),
    )?;
    let mut body = response["body"].clone();
    translate_stack_frames(&mut body, provenance, false);
    if let Some(frame) = body["stackFrames"]
        .as_array()
        .and_then(|frames| frames.first())
    {
        eprintln!(
            "{} at {}:{}",
            frame["name"].as_str().unwrap_or("<native>"),
            frame["source"]["path"].as_str().unwrap_or("<unknown>"),
            frame["line"].as_u64().unwrap_or(0)
        );
    }
    Ok(())
}

fn show_frames(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    native: bool,
) -> Result<(), CliFailure> {
    let response = backend.request("stackTrace", json!({"threadId": thread_id}))?;
    let mut body = response["body"].clone();
    translate_stack_frames(&mut body, provenance, native);
    let frames = body["stackFrames"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|frame| native || frame["presentationHint"] != "subtle");
    for (index, frame) in frames.enumerate() {
        eprintln!(
            "#{index:<3} {} at {}:{}",
            frame["name"].as_str().unwrap_or("<native>"),
            frame["source"]["path"].as_str().unwrap_or("<unknown>"),
            frame["line"].as_u64().unwrap_or(0)
        );
    }
    Ok(())
}

fn mapped_frames(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
) -> Result<Vec<(Value, Value)>, CliFailure> {
    let response = backend.request("stackTrace", json!({"threadId": thread_id}))?;
    let native = response["body"]["stackFrames"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut translated = response["body"].clone();
    translate_stack_frames(&mut translated, provenance, false);
    let translated = translated["stackFrames"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    Ok(native
        .into_iter()
        .zip(translated)
        .filter(|(_, frame)| frame["presentationHint"] != "subtle")
        .collect())
}

fn selected_frame(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    index: usize,
) -> Result<(Value, Value), CliFailure> {
    mapped_frames(backend, provenance, thread_id)?
        .into_iter()
        .nth(index)
        .ok_or_else(|| debugger_failure(format!("no mapped Terrane frame #{index}")))
}

fn print_frame(index: usize, frame: &Value) {
    eprintln!(
        "#{index:<3} {} at {}:{}",
        frame["name"].as_str().unwrap_or("<native>"),
        frame["source"]["path"].as_str().unwrap_or("<unknown>"),
        frame["line"].as_u64().unwrap_or(0)
    );
}

fn show_selected_frame(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    index: usize,
) -> Result<(), CliFailure> {
    let (_, frame) = selected_frame(backend, provenance, thread_id, index)?;
    print_frame(index, &frame);
    Ok(())
}

fn show_source(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    frame_index: usize,
    radius: usize,
) -> Result<(), CliFailure> {
    let (_, frame) = selected_frame(backend, provenance, thread_id, frame_index)?;
    let path = frame["source"]["path"]
        .as_str()
        .map(PathBuf::from)
        .ok_or_else(|| debugger_failure("selected frame has no Terrane source"))?;
    show_source_context(&path, frame["line"].as_u64().unwrap_or(0), radius)
}

fn show_variables(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    frame_index: usize,
    registers: bool,
) -> Result<(), CliFailure> {
    let (frame, _) = selected_frame(backend, provenance, thread_id, frame_index)?;
    let frame_id = frame["id"]
        .as_i64()
        .ok_or_else(|| debugger_failure("selected thread has no frame"))?;
    let stop_context = frame_stop_context(&frame, provenance);
    let scopes = backend.request("scopes", json!({"frameId": frame_id}))?;
    let mut variable_objects = BTreeMap::new();
    for scope in scopes["body"]["scopes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|scope| {
            scope["name"].as_str().is_some_and(|name| {
                name.eq_ignore_ascii_case(if registers { "registers" } else { "locals" })
            })
        })
    {
        let reference = scope["variablesReference"].as_i64().unwrap_or(0);
        let response = if registers {
            backend.request("variables", json!({"variablesReference": reference}))?
        } else {
            request_terrane_variables(
                backend,
                json!({"variablesReference": reference}),
                provenance,
            )?
        };
        let mut body = response["body"].clone();
        if !registers {
            let value_summaries =
                read_value_summaries(backend, &body, supports_adaptive_int_layout(provenance));
            translate_variables(
                &mut body,
                provenance,
                reference,
                &mut variable_objects,
                &value_summaries,
                stop_context.as_ref(),
            );
        }
        for variable in body["variables"].as_array().into_iter().flatten() {
            eprintln!(
                "{} = {}",
                variable["name"].as_str().unwrap_or("?"),
                variable["value"]
                    .as_str()
                    .unwrap_or("<unavailable debug information>")
            );
        }
    }
    Ok(())
}

fn show_value(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    frame_index: usize,
    name: &str,
) -> Result<(), CliFailure> {
    let (frame, _) = selected_frame(backend, provenance, thread_id, frame_index)?;
    let frame_id = frame["id"]
        .as_i64()
        .ok_or_else(|| debugger_failure("selected thread has no frame"))?;
    let stop_context = frame_stop_context(&frame, provenance);
    let scopes = backend.request("scopes", json!({"frameId": frame_id}))?;
    let mut variable_objects = BTreeMap::new();
    for scope in scopes["body"]["scopes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|scope| {
            scope["name"]
                .as_str()
                .is_some_and(|name| name.eq_ignore_ascii_case("locals"))
        })
    {
        let reference = scope["variablesReference"].as_i64().unwrap_or(0);
        let response = request_terrane_variables(
            backend,
            json!({"variablesReference": reference}),
            provenance,
        )?;
        let mut body = response["body"].clone();
        let summaries =
            read_value_summaries(backend, &body, supports_adaptive_int_layout(provenance));
        translate_variables(
            &mut body,
            provenance,
            reference,
            &mut variable_objects,
            &summaries,
            stop_context.as_ref(),
        );
        if let Some(variable) = body["variables"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|variable| variable["name"].as_str() == Some(name))
        {
            let mut traversal = ValueTraversal {
                variable_objects: &mut variable_objects,
                visited: std::collections::BTreeSet::new(),
                remaining: 256,
                stop_context: stop_context.as_ref(),
            };
            print_value_tree(backend, provenance, variable, &mut traversal, 0)?;
            return Ok(());
        }
    }
    Err(debugger_failure(format!(
        "selected frame has no local named `{name}`"
    )))
}

struct ValueTraversal<'a> {
    variable_objects: &'a mut BTreeMap<i64, String>,
    visited: std::collections::BTreeSet<i64>,
    remaining: usize,
    stop_context: Option<&'a StopContext>,
}

fn print_value_tree(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    variable: &Value,
    traversal: &mut ValueTraversal<'_>,
    depth: usize,
) -> Result<(), CliFailure> {
    if traversal.remaining == 0 {
        eprintln!("{}<truncated>", "  ".repeat(depth));
        return Ok(());
    }
    traversal.remaining -= 1;
    eprintln!(
        "{}{} = {}",
        "  ".repeat(depth),
        variable["name"].as_str().unwrap_or("?"),
        variable["value"]
            .as_str()
            .unwrap_or("<unavailable debug information>")
    );
    let reference = variable["variablesReference"].as_i64().unwrap_or(0);
    if reference == 0 {
        return Ok(());
    }
    if depth >= 6 {
        eprintln!("{}<maximum depth reached>", "  ".repeat(depth + 1));
        return Ok(());
    }
    if !traversal.visited.insert(reference) {
        eprintln!("{}<cycle>", "  ".repeat(depth + 1));
        return Ok(());
    }
    let response = request_terrane_variables(
        backend,
        json!({"variablesReference": reference}),
        provenance,
    )?;
    let mut body = response["body"].clone();
    let summaries = read_value_summaries(backend, &body, supports_adaptive_int_layout(provenance));
    translate_variables(
        &mut body,
        provenance,
        reference,
        traversal.variable_objects,
        &summaries,
        traversal.stop_context,
    );
    for child in body["variables"].as_array().into_iter().flatten() {
        print_value_tree(backend, provenance, child, traversal, depth + 1)?;
    }
    Ok(())
}

fn show_generated(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    frame_index: usize,
    radius: usize,
) -> Result<(), CliFailure> {
    let (frame, _) = selected_frame(backend, provenance, thread_id, frame_index)?;
    let path = frame["source"]["path"]
        .as_str()
        .map(PathBuf::from)
        .ok_or_else(|| debugger_failure("selected frame has no generated source"))?;
    show_source_context(&path, frame["line"].as_u64().unwrap_or(0), radius)
}

fn show_source_context(path: &Path, line: u64, radius: usize) -> Result<(), CliFailure> {
    let line = usize::try_from(line)
        .ok()
        .filter(|line| *line > 0)
        .ok_or_else(|| debugger_failure("selected frame has no source line"))?;
    let contents = fs::read_to_string(path).map_err(|error| {
        debugger_failure(format!(
            "cannot read source context {}: {error}",
            path.display()
        ))
    })?;
    let lines = contents.lines().collect::<Vec<_>>();
    let start = line.saturating_sub(radius).max(1);
    let end = line.saturating_add(radius).min(lines.len());
    eprintln!("{}:{line}", path.display());
    for current in start..=end {
        eprintln!(
            "{} {current:>5} | {}",
            if current == line { ">" } else { " " },
            lines[current - 1]
        );
    }
    Ok(())
}

fn parse_frame_index(index: &str) -> Result<usize, CliFailure> {
    index
        .parse()
        .map_err(|_| debugger_failure("frame index must be a non-negative integer"))
}

fn parse_context_radius(radius: &str) -> Result<usize, CliFailure> {
    let radius = if radius.is_empty() {
        3
    } else {
        radius
            .parse()
            .map_err(|_| debugger_failure("context radius must be an integer from 0 through 20"))?
    };
    if radius > 20 {
        return Err(debugger_failure(
            "context radius must be an integer from 0 through 20",
        ));
    }
    Ok(radius)
}

fn parse_breakpoint(location: &str) -> Result<(PathBuf, usize), CliFailure> {
    let (path, line) = location
        .rsplit_once(':')
        .ok_or_else(|| debugger_failure("breakpoint must be <source>:<line>"))?;
    let line = line
        .parse()
        .map_err(|_| debugger_failure("breakpoint line must be a positive integer"))?;
    Ok((PathBuf::from(path), line))
}

fn load_provenance(sidecar: &Path) -> Result<ProvenanceManifest, CliFailure> {
    let bytes = fs::read(sidecar).map_err(|error| {
        debugger_failure(format!(
            "Terrane translation unavailable: cannot read provenance sidecar {}: {error}; raw native debugging remains available",
            sidecar.display()
        ))
    })?;
    serde_json::from_slice(&bytes).map_err(|error| {
        debugger_failure(format!(
            "Terrane translation unavailable: invalid provenance sidecar {}: {error}; raw native debugging remains available",
            sidecar.display()
        ))
    })
}

fn load_and_validate(
    sidecar: &Path,
    executable: &Path,
    relocation: Option<&Value>,
) -> Result<ProvenanceManifest, CliFailure> {
    validate_provenance(load_provenance(sidecar)?, executable, relocation)
}

fn validate_provenance(
    mut provenance: ProvenanceManifest,
    executable: &Path,
    relocation: Option<&Value>,
) -> Result<ProvenanceManifest, CliFailure> {
    if let Some(relocation) = relocation {
        if let Some(build_root) = relocation["buildRoot"].as_str() {
            provenance.relocation.build_root =
                canonical_relocation_root(Path::new(build_root), "build")?;
        }
        if let Some(source_root) = relocation["sourceRoot"].as_str() {
            provenance.relocation.source_root =
                canonical_relocation_root(Path::new(source_root), "source")?;
        }
    }
    if provenance.schema_version != terrane_compiler::debugging::SCHEMA_VERSION {
        return Err(debugger_failure(format!(
            "unsupported debug provenance schema {}",
            provenance.schema_version
        )));
    }
    if provenance.compiler_version != terrane_compiler::VERSION {
        return Err(debugger_failure(format!(
            "unsupported debug provenance compiler {}",
            provenance.compiler_version
        )));
    }
    let expected_recipe = terrane_compiler::debugging::abi_recipe_for_toolchain(
        &provenance.target,
        &provenance.rustc_release,
    );
    if expected_recipe == "unsupported" || provenance.abi_recipe != expected_recipe {
        return Err(debugger_failure(format!(
            "unsupported debug target, toolchain, or ABI recipe: {} / {}",
            provenance.target, provenance.abi_recipe
        )));
    }
    let profile = terrane_compiler::debugging::DEBUG_ARTIFACT_PROFILE;
    if provenance.artifact_profile != profile.id
        || provenance.optimization != profile.optimization
        || provenance.debug_information != profile.debug_information
        || provenance.inlining != profile.inlining
        || provenance.stripping != profile.stripping
    {
        return Err(debugger_failure(format!(
            "unsupported debug artifact profile {}: optimization={}, debug-information={}, inlining={}, stripping={}",
            provenance.artifact_profile,
            provenance.optimization,
            provenance.debug_information,
            provenance.inlining,
            provenance.stripping
        )));
    }
    provenance
        .validate_executable(executable)
        .map_err(debugger_failure)?;
    for generated in &provenance.debug.generated_files {
        let path = generated_path(&provenance, &generated.path);
        let actual = fs::read(&path)
            .ok()
            .filter(|bytes| {
                terrane_compiler::debugging::hash_bytes(bytes) == generated.content_hash
            })
            .or_else(|| {
                generated
                    .embedded_source
                    .as_deref()
                    .map(str::as_bytes)
                    .map(Vec::from)
            });
        if actual
            .as_deref()
            .map(terrane_compiler::debugging::hash_bytes)
            .as_deref()
            != Some(generated.content_hash.as_str())
        {
            return Err(debugger_failure(format!(
                "Terrane translation unavailable: generated source identity mismatch for {}; raw native debugging remains available",
                path.display()
            )));
        }
    }
    Ok(provenance)
}

fn supports_adaptive_int_layout(provenance: &ProvenanceManifest) -> bool {
    provenance.target == "x86_64-unknown-linux-gnu"
        && provenance.abi_recipe
            == terrane_compiler::debugging::abi_recipe_for_toolchain(
                &provenance.target,
                &provenance.rustc_release,
            )
}

fn canonical_relocation_root(path: &Path, kind: &str) -> Result<String, CliFailure> {
    path.canonicalize()
        .map(|path| path.to_string_lossy().into_owned())
        .map_err(|error| {
            debugger_failure(format!(
                "Terrane translation unavailable: cannot canonicalize relocated {kind} root {}: {error}; raw native debugging remains available",
                path.display()
            ))
        })
}

fn read_message(reader: &mut impl BufRead) -> io::Result<Option<Value>> {
    let mut content_length = None;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? == 0 {
            return Ok(None);
        }
        if header == "\r\n" || header == "\n" {
            break;
        }
        if let Some(value) = header.trim().strip_prefix("Content-Length:") {
            content_length = value.trim().parse::<usize>().ok();
        }
    }
    let length = content_length.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "DAP message has no Content-Length",
        )
    })?;
    let mut bytes = vec![0; length];
    reader.read_exact(&mut bytes)?;
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn write_message(writer: &mut impl Write, value: &Value) -> io::Result<()> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    write!(writer, "Content-Length: {}\r\n\r\n", bytes.len())?;
    writer.write_all(&bytes)?;
    writer.flush()
}

fn contextual_debugger_failure(context: &str, failure: &CliFailure) -> CliFailure {
    let detail = failure
        .message
        .trim()
        .strip_prefix("<debugger>: error[S5001]: ")
        .unwrap_or(failure.message.trim());
    debugger_failure(format!("{context}: {detail}"))
}

fn debugger_failure(message: impl Into<String>) -> CliFailure {
    CliFailure::diagnostic(PathBuf::from("<debugger>"), "S5001", message.into(), 5)
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "map_err transfers the owned protocol error into debugger diagnostics"
)]
fn protocol_failure(error: io::Error) -> CliFailure {
    debugger_failure(format!("debug adapter protocol failure: {error}"))
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "map_err transfers the owned I/O error into debugger diagnostics"
)]
fn io_failure(error: io::Error) -> CliFailure {
    debugger_failure(format!("debugger terminal I/O failure: {error}"))
}

#[cfg(test)]
mod tests {
    use crate::debug_command::adapter::Adapter;

    use super::*;

    fn provenance() -> ProvenanceManifest {
        let mut provenance: ProvenanceManifest = serde_json::from_value(json!({
            "schema_version": "1.3",
            "compiler_version": env!("CARGO_PKG_VERSION"),
            "rust_toolchain": "system",
            "target": "x86_64-unknown-linux-gnu",
            "rust_sysroot": "/tmp/sysroot",
            "rustc_release": "rustc fixture",
            "abi_recipe": "",
            "artifact_profile": "terrane-debug-v1",
            "optimization": "0",
            "debug_information": "full",
            "inlining": "compiler-default-at-opt-level-0",
            "stripping": "none",
            "inputs": [],
            "debug": {
                "schema_version": "1.3",
                "compiler_version": env!("CARGO_PKG_VERSION"),
                "sources": [{
                    "id": 1,
                    "uri": "source.trn",
                    "content_hash": "sha256:test"
                }],
                "generated_files": [{
                    "path": "src/main.rs",
                    "content_hash": "sha256:generated",
                    "associations": [{
                        "generated": {"start": 0, "end": 4, "line": 10, "column": 1, "end_line": 10, "end_column": 5},
                        "causes": [{"source_id": 1, "start": 20, "end": 24, "line": 4, "column": 3, "end_line": 4, "end_column": 7}],
                        "role": "user",
                        "sequence_point": true,
                        "function_id": "function",
                        "scope_ids": ["scope"]
                    }, {
                        "generated": {"start": 5, "end": 9, "line": 12, "column": 1, "end_line": 12, "end_column": 5},
                        "causes": [{"source_id": 1, "start": 20, "end": 24, "line": 4, "column": 3, "end_line": 4, "end_column": 7}],
                        "role": "user",
                        "sequence_point": true,
                        "function_id": "function",
                        "scope_ids": ["scope"]
                    }, {
                        "generated": {"start": 10, "end": 14, "line": 14, "column": 1, "end_line": 14, "end_column": 5},
                        "causes": [{"source_id": 1, "start": 40, "end": 44, "line": 6, "column": 3, "end_line": 6, "end_column": 7}],
                        "role": "user",
                        "sequence_point": true,
                        "function_id": "function",
                        "scope_ids": ["scope"]
                    }]
                }],
                "functions": [{
                    "id": "function",
                    "name": "/source::main",
                    "namespace": "/source",
                    "source": {"source_id": 1, "start": 10, "end": 50, "line": 2, "column": 1, "end_line": 6, "end_column": 1},
                    "rust_name": "main",
                    "is_async": false
                }],
                "scopes": [{
                    "id": "scope",
                    "function_id": "function",
                    "source": {"source_id": 1, "start": 10, "end": 50, "line": 2, "column": 1, "end_line": 6, "end_column": 1}
                }],
                "bindings": [{
                    "id": "binding",
                    "name": "credentials",
                    "rust_name": "credentials",
                    "source": {"source_id": 1, "start": 20, "end": 24, "line": 4, "column": 3, "end_line": 4, "end_column": 7},
                    "visible_from": 24,
                    "visible_until": 50,
                    "function_id": "function",
                    "scope_id": "scope",
                    "type_name": "Object(credentials)",
                    "object_id": "/source::credentials",
                    "mutable": false
                }],
                "objects": [{
                    "id": "/source::credentials",
                    "fields": [{
                        "name": "username",
                        "rust_name": "username",
                        "type_name": "Scalar(String)",
                        "secret": false
                    }, {
                        "name": "token",
                        "rust_name": "token",
                        "type_name": "Scalar(String)",
                        "secret": true
                    }]
                }]
            },
            "native_module": {"file_name": "program", "content_hash": "sha256:module"},
            "relocation": {"build_root": "/build", "source_root": "/source"}
        }))
        .unwrap();
        provenance.abi_recipe = terrane_compiler::debugging::abi_recipe_for_toolchain(
            &provenance.target,
            &provenance.rustc_release,
        );
        provenance
    }

    #[test]
    fn breakpoint_resolution_preserves_every_native_location_and_adjusts_in_function() {
        let provenance = provenance();
        let exact = resolve_breakpoint(&provenance, Path::new("/source/source.trn"), 4);
        assert_eq!(
            exact
                .iter()
                .map(|location| location.generated_line)
                .collect::<Vec<_>>(),
            [10, 12]
        );
        assert!(exact.iter().all(|location| location.source_line == 4));

        let adjusted = resolve_breakpoint(&provenance, Path::new("source.trn"), 5);
        assert_eq!(adjusted.len(), 1);
        assert!(
            adjusted
                .iter()
                .all(|location| location.message.contains("adjusted"))
        );
        assert_eq!(adjusted[0].source_line, 6);
        assert!(resolve_breakpoint(&provenance, Path::new("unknown.trn"), 4).is_empty());
    }

    #[test]
    fn rust_lldb_initialization_precedes_client_commands_when_available() {
        let mut arguments = json!({"initCommands": ["settings set target.language c++"]});
        add_rust_lldb_init_commands(
            &mut arguments,
            Some(Path::new(
                "/toolchain with spaces/lib/rustlib/etc/lldb_lookup.py",
            )),
        );
        assert_eq!(
            arguments["initCommands"],
            json!([
                "?command script import \"/toolchain with spaces/lib/rustlib/etc/lldb_lookup.py\"",
                "?settings set target.process.unsupported-language-warnings false",
                "settings set target.language c++"
            ])
        );

        let mut unavailable = json!({"program": "/tmp/program"});
        add_rust_lldb_init_commands(&mut unavailable, None);
        assert!(unavailable.get("initCommands").is_none());
    }

    #[test]
    fn variable_translation_redacts_secret_fields_before_frontend_exposure() {
        let provenance = provenance();
        let mut references = BTreeMap::new();
        let mut locals = json!({"variables": [{
            "name": "credentials",
            "value": "Credentials",
            "variablesReference": 7,
            "memoryReference": "0x1234",
            "evaluateName": "credentials"
        }]});
        translate_variables(
            &mut locals,
            &provenance,
            1,
            &mut references,
            &BTreeMap::new(),
            None,
        );
        assert_eq!(
            references.get(&7).map(String::as_str),
            Some("/source::credentials")
        );

        let mut fields = json!({"variables": [{
            "name": "username",
            "value": "visible",
            "variablesReference": 0
        }, {
            "name": "token",
            "value": "must-not-escape",

            "variablesReference": 9,
            "memoryReference": "0x2345",
            "evaluateName": "credentials.token"
        }]});
        translate_variables(
            &mut fields,
            &provenance,
            7,
            &mut references,
            &BTreeMap::new(),
            None,
        );
        assert_eq!(fields["variables"][0]["value"], "visible");
        assert_eq!(fields["variables"][1]["value"], "<secret>");
        assert_eq!(fields["variables"][1]["variablesReference"], 0);
        assert!(fields["variables"][1]["memoryReference"].is_null());
        assert!(fields["variables"][1]["evaluateName"].is_null());
    }

    #[test]
    fn variable_translation_selects_the_innermost_visible_shadow() {
        let mut provenance = provenance();
        let mut outer = provenance.debug.bindings[0].clone();
        outer.name = "outer".to_owned();
        outer.rust_name = "value".to_owned();
        outer.object_id = None;
        outer.type_name = "Scalar(Int64)".to_owned();
        outer.visible_from = 10;
        let mut inner = outer.clone();
        inner.name = "inner".to_owned();
        inner.scope_id = Some("inner-scope".to_owned());
        inner.visible_from = 25;
        provenance.debug.bindings = vec![outer, inner];
        let mut variables = json!({"variables": [{
            "name": "value",
            "value": "7",
            "variablesReference": 0
        }]});
        translate_variables(
            &mut variables,
            &provenance,
            1,
            &mut BTreeMap::new(),
            &BTreeMap::new(),
            Some(&StopContext {
                source_id: 1,
                position: 30,
                function_id: Some("function".to_owned()),
                scope_ids: vec!["scope".to_owned(), "inner-scope".to_owned()],
            }),
        );
        assert_eq!(variables["variables"][0]["name"], "inner");
    }

    #[test]
    fn scalar_values_remain_raw_without_a_matching_abi_recipe() {
        let mut provenance = provenance();
        let binding = &mut provenance.debug.bindings[0];
        binding.name = "value".to_owned();
        binding.rust_name = "value".to_owned();
        binding.object_id = None;
        binding.type_name = "Scalar(Int)".to_owned();
        let mut variables = json!({"variables": [{
            "name": "value",
            "value": "41",
            "variablesReference": 7,
            "memoryReference": "0x1234"
        }]});
        translate_variables(
            &mut variables,
            &provenance,
            1,
            &mut BTreeMap::new(),
            &BTreeMap::new(),
            None,
        );
        assert_eq!(variables["variables"][0]["value"], "41");
        assert_eq!(variables["variables"][0]["variablesReference"], 0);
    }

    #[test]
    fn batched_lldb_breakpoint_output_preserves_every_identifier() {
        assert_eq!(
            parse_lldb_breakpoint_ids(
                "Breakpoint 4: 1 location.\nBreakpoint 7: 2 locations.\nnot a breakpoint\n"
            ),
            [4, 7]
        );
    }

    #[test]
    fn executable_sidecars_preserve_native_extensions() {
        assert_eq!(
            executable_sidecar(Path::new("/tmp/program.exe")),
            Path::new("/tmp/program.exe.terrane-debug.json")
        );
        assert_eq!(
            executable_sidecar(Path::new("/tmp/program")),
            Path::new("/tmp/program.terrane-debug.json")
        );
    }

    #[test]
    fn resuming_invalidates_session_variable_handles_before_backend_io() {
        let mut adapter = Adapter::default();
        adapter.variable_objects.insert(7, "object".to_owned());
        assert!(adapter.handle("continue", json!({"threadId": 1})).is_err());
        assert!(adapter.variable_objects.is_empty());
    }
}
