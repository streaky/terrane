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
use base64::Engine as _;
use num_bigint::BigUint;

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

pub(super) fn run_adapter(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    if arguments.len() != 2 || arguments[1] != "--stdio" {
        return Err(CliFailure::usage());
    }
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut translator = Adapter::default();
    let mut outgoing_sequence = 1_i64;
    while let Some(request) = read_message(&mut reader).map_err(protocol_failure)? {
        let sequence = request["seq"].as_i64().unwrap_or(0);
        let command = request["command"].as_str().unwrap_or_default().to_owned();
        let result = translator.handle(
            &command,
            request
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({})),
        );
        match result {
            Ok(mut messages) => {
                for message in &mut messages {
                    message["seq"] = outgoing_sequence.into();
                    outgoing_sequence += 1;
                    if message["type"] == "response" {
                        message["request_seq"] = sequence.into();
                        message["command"] = command.clone().into();
                    }
                    write_message(&mut writer, message).map_err(protocol_failure)?;
                }
            }
            Err(error) => {
                let message = json!({
                    "seq": outgoing_sequence,
                    "type": "response",
                    "request_seq": sequence,
                    "command": command,
                    "success": false,
                    "message": error.message
                });
                outgoing_sequence += 1;
                write_message(&mut writer, &message).map_err(protocol_failure)?;
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

#[derive(Default)]
struct Adapter {
    backend: Option<Backend>,
    provenance: Option<ProvenanceManifest>,
    executable: Option<PathBuf>,
    launched: bool,
    variable_objects: BTreeMap<i64, String>,
    frame_contexts: BTreeMap<i64, StopContext>,
    variable_contexts: BTreeMap<i64, StopContext>,
    pending_launch_response: Option<i64>,
    pending_launch_context: Option<String>,
    breakpoints: BreakpointManager,
}

#[derive(Clone)]
struct StopContext {
    source_id: u32,
    position: usize,
    function_id: Option<String>,
    scope_ids: Vec<String>,
}

impl Adapter {
    #[expect(
        clippy::too_many_lines,
        reason = "central dispatch keeps DAP request correlation and debugger lifecycle ordering explicit"
    )]
    fn handle(&mut self, command: &str, arguments: Value) -> Result<Vec<Value>, CliFailure> {
        if matches!(
            command,
            "continue" | "next" | "stepIn" | "stepOut" | "configurationDone"
        ) {
            self.variable_objects.clear();
            self.frame_contexts.clear();
            self.variable_contexts.clear();
        }
        if command == "initialize" {
            let backend = self.backend.get_or_insert(Backend::start()?);
            let _ = backend.request("initialize", arguments)?;
            let mut messages = vec![
                response(json!({
                    "supportsConfigurationDoneRequest": true,
                    "supportsTerminateRequest": true,
                    "supportsRestartRequest": false,
                    "supportsConditionalBreakpoints": false,
                    "supportsLogPoints": false,
                    "supportsReadMemoryRequest": true,
                    "supportsDisassembleRequest": true,
                    "supportsSetVariable": false,
                    "supportsEvaluateForHovers": false,
                    "exceptionBreakpointFilters": []
                })),
                event("initialized", json!({})),
            ];
            messages.extend(take_backend_events(backend));
            return Ok(messages);
        }
        if command == "launch" || command == "attach" {
            let executable = arguments["program"]
                .as_str()
                .map(PathBuf::from)
                .ok_or_else(|| {
                    debugger_failure("launch/attach requires a native `program` path")
                })?;
            if !executable.is_file() {
                return Err(debugger_failure(format!(
                    "cannot {command} missing native program {}",
                    executable.display()
                )));
            }
            let attach_pid = arguments["pid"].as_u64().unwrap_or(0);
            let sidecar = arguments["terraneProvenance"]
                .as_str()
                .map_or_else(|| executable_sidecar(&executable), PathBuf::from);
            let loaded = load_provenance(&sidecar);
            let recorded_relocation = loaded
                .as_ref()
                .ok()
                .map(|provenance| provenance.relocation.clone());
            let translation = loaded.and_then(|provenance| {
                validate_provenance(
                    provenance,
                    &executable,
                    arguments.get("terraneRelocation"),
                )
            })
            .and_then(|provenance| {
                let stale =
                    provenance.validate_sources(Path::new(&provenance.relocation.source_root));
                if stale.is_empty() {
                    Ok(provenance)
                } else {
                    Err(debugger_failure(format!(
                        "Terrane source changed since this binary was built: {}; rebuild before placing source breakpoints",
                        stale.join(", ")
                    )))
                }
            });
            let mut backend_arguments = arguments;
            backend_arguments["program"] = executable.to_string_lossy().into_owned().into();
            for name in ["terraneProvenance", "terraneRelocation", "useBuildSnapshot"] {
                backend_arguments
                    .as_object_mut()
                    .expect("DAP arguments are an object")
                    .remove(name);
            }
            if let (Some(recorded), Ok(relocated)) = (&recorded_relocation, &translation) {
                let mut source_map = Vec::new();
                if recorded.build_root != relocated.relocation.build_root {
                    source_map.push(json!([
                        recorded.build_root,
                        relocated.relocation.build_root
                    ]));
                }
                if recorded.source_root != relocated.relocation.source_root {
                    source_map.push(json!([
                        recorded.source_root,
                        relocated.relocation.source_root
                    ]));
                }
                if !source_map.is_empty() {
                    backend_arguments["sourceMap"] = source_map.into();
                }
            }
            add_rust_lldb_init_commands(
                &mut backend_arguments,
                translation
                    .as_ref()
                    .ok()
                    .and_then(|provenance| discover_rust_lldb_formatter(&provenance.rust_sysroot))
                    .as_deref(),
            );
            if command == "launch"
                && let Ok(provenance) = &translation
            {
                backend_arguments["cwd"] = provenance.relocation.source_root.clone().into();
            }
            let (sequence, backend_response) = self
                .backend
                .as_mut()
                .ok_or_else(|| debugger_failure("initialize must precede launch or attach"))?
                .request_with_timeout(
                    command,
                    backend_arguments,
                    Some(Duration::from_millis(500)),
                )?;
            self.pending_launch_response = backend_response.is_none().then_some(sequence);
            self.pending_launch_context = backend_response.is_none().then(|| {
                if command == "attach" {
                    format!(
                        "attach to process {attach_pid} was rejected by the host or debugger backend"
                    )
                } else {
                    format!("launch of {} failed", executable.display())
                }
            });
            let backend_response = backend_response.unwrap_or_else(|| response(json!({})));
            self.launched = command == "launch";
            self.executable = Some(executable);
            let backend = self.backend.as_mut().expect("backend initialized above");
            let mut messages =
                with_backend_events(backend, response(backend_response["body"].clone()));
            match translation {
                Ok(provenance) => {
                    self.breakpoints.resolve_all(&provenance);
                    self.breakpoints.sync(backend, &provenance)?;
                    messages.push(event(
                        "terrane/fidelity",
                        json!({
                            "mode": "source",
                            "sourceTranslation": true,
                            "target": provenance.target,
                            "abiRecipe": provenance.abi_recipe,
                            "inlining": provenance.inlining,
                            "artifactProfile": provenance.artifact_profile,
                            "rustcRelease": provenance.rustc_release
                        }),
                    ));
                    self.provenance = Some(provenance);
                    messages.extend(self.breakpoints.verification_events());
                }
                Err(failure) => {
                    self.provenance = None;
                    messages.push(event(
                        "terrane/fidelity",
                        json!({
                            "mode": "native",
                            "sourceTranslation": false,
                            "reason": failure.message
                        }),
                    ));
                    messages.push(event(
                        "output",
                        json!({
                            "category": "console",
                            "output": format!(
                                "{}\nTerrane source translation is disabled; raw native debugging remains available.\n",
                                failure.message
                            )
                        }),
                    ));
                }
            }
            return Ok(messages);
        }
        if command == "setBreakpoints" {
            return self.set_breakpoints(&arguments);
        }
        if command == "stackTrace" {
            let provenance = self.provenance.clone();
            let (mut body, events) = {
                let backend = self.backend_mut()?;
                let backend_response = backend.request(command, arguments)?;
                backend_body_and_events(backend, &backend_response)
            };
            if let Some(provenance) = &provenance {
                self.frame_contexts = frame_stop_contexts(&body, provenance);
                translate_stack_frames(&mut body, provenance, false);
            }
            let mut messages = vec![response(body)];
            messages.extend(events);
            return Ok(messages);
        }
        if command == "scopes" {
            let frame_id = arguments["frameId"].as_i64().unwrap_or(0);
            let context = self.frame_contexts.get(&frame_id).cloned();
            let (body, events) = {
                let backend = self.backend_mut()?;
                let backend_response = backend.request(command, arguments)?;
                backend_body_and_events(backend, &backend_response)
            };
            if let Some(context) = context {
                for scope in body["scopes"].as_array().into_iter().flatten() {
                    if let Some(reference) = scope["variablesReference"].as_i64() {
                        self.variable_contexts.insert(reference, context.clone());
                    }
                }
            }
            let mut messages = vec![response(body)];
            messages.extend(events);
            return Ok(messages);
        }
        if command == "variables" {
            let parent_reference = arguments["variablesReference"].as_i64().unwrap_or(0);
            let provenance = self.provenance.clone();
            let stop_context = self.variable_contexts.get(&parent_reference).cloned();
            let selected_layout = provenance
                .as_ref()
                .is_some_and(supports_adaptive_int_layout);
            let (mut body, value_summaries, events) = {
                let backend = self.backend_mut()?;
                let backend_response = if provenance.is_some() {
                    request_terrane_variables(backend, arguments)?
                } else {
                    backend.request(command, arguments)?
                };
                let body = backend_response["body"].clone();
                let value_summaries = read_value_summaries(backend, &body, selected_layout);
                let (_, events) = backend_body_and_events(backend, &backend_response);
                (body, value_summaries, events)
            };
            if let Some(provenance) = &provenance {
                translate_variables(
                    &mut body,
                    provenance,
                    parent_reference,
                    &mut self.variable_objects,
                    &value_summaries,
                    stop_context.as_ref(),
                );
                if let Some(context) = stop_context {
                    for variable in body["variables"].as_array().into_iter().flatten() {
                        if let Some(reference) = variable["variablesReference"]
                            .as_i64()
                            .filter(|reference| *reference != 0)
                        {
                            self.variable_contexts.insert(reference, context.clone());
                        }
                    }
                }
            }
            let mut messages = vec![response(body)];
            messages.extend(events);
            return Ok(messages);
        }
        if command == "terrane/generatedSource" {
            return self.generated_source(&arguments);
        }
        if command == "terrane/nativeStackTrace" {
            let backend = self.backend_mut()?;
            let backend_response = backend.request("stackTrace", arguments)?;
            return Ok(vec![response(backend_response["body"].clone())]);
        }
        if command == "terrane/backend" {
            let backend = self.backend_mut()?;
            let expression = arguments["command"].as_str().unwrap_or_default();
            let backend_response = backend.request(
                "evaluate",
                json!({"expression": format!("`{expression}"), "context": "repl"}),
            )?;
            return Ok(vec![response(backend_response["body"].clone())]);
        }
        if matches!(command, "next" | "stepIn" | "stepOut")
            && let Some(provenance) = self.provenance.clone()
        {
            let thread_id = arguments["threadId"].as_i64().unwrap_or(1);
            let breakpoints = self.breakpoints.clone();
            let backend = self.backend_mut()?;
            let event = step_to_source(backend, &provenance, &breakpoints, command, thread_id)?;
            let mut messages = with_backend_events(backend, response(json!({})));
            messages.push(event);
            return Ok(messages);
        }
        if command == "disconnect" {
            let attached = !self.launched;
            let terminate = arguments["terminateDebuggee"]
                .as_bool()
                .unwrap_or(!attached)
                && !attached;
            let backend = self.backend_mut()?;
            let backend_response =
                backend.request("disconnect", json!({"terminateDebuggee": terminate}))?;
            return Ok(with_backend_events(
                backend,
                response(backend_response["body"].clone()),
            ));
        }
        let pending_launch = (command == "configurationDone")
            .then(|| self.pending_launch_response.take())
            .flatten();
        let pending_context = (command == "configurationDone")
            .then(|| self.pending_launch_context.take())
            .flatten();
        let backend = self.backend_mut()?;
        let backend_response = backend.request(command, arguments);
        if let Some(sequence) = pending_launch
            && let Err(failure) = backend.finish_request(sequence, Duration::from_millis(500))
        {
            return Err(contextual_debugger_failure(
                pending_context
                    .as_deref()
                    .unwrap_or("delayed launch or attach request failed"),
                &failure,
            ));
        }
        let backend_response = backend_response.map_err(|failure| {
            if let Some(context) = pending_context.as_deref() {
                contextual_debugger_failure(context, &failure)
            } else {
                failure
            }
        })?;
        let terminal_event = if matches!(
            command,
            "continue" | "next" | "stepIn" | "stepOut" | "configurationDone"
        ) {
            Some(backend.wait_for_event(&["stopped", "terminated"])?)
        } else {
            None
        };
        let mut messages = with_backend_events(backend, response(backend_response["body"].clone()));
        if let Some(event) = terminal_event {
            messages.push(event);
        }
        Ok(messages)
    }

    fn backend_mut(&mut self) -> Result<&mut Backend, CliFailure> {
        self.backend
            .as_mut()
            .ok_or_else(|| debugger_failure("initialize must be the first request"))
    }

    fn set_breakpoints(&mut self, arguments: &Value) -> Result<Vec<Value>, CliFailure> {
        let source_path = arguments["source"]["path"]
            .as_str()
            .map(PathBuf::from)
            .ok_or_else(|| debugger_failure("source breakpoint request requires a source path"))?;
        let requested = arguments["breakpoints"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|breakpoint| breakpoint["line"].as_u64())
            .filter_map(|line| usize::try_from(line).ok())
            .collect::<Vec<_>>();
        let source_path = self
            .provenance
            .as_ref()
            .and_then(|provenance| known_source_path(provenance, &source_path))
            .unwrap_or(source_path);
        self.breakpoints
            .replace_source(self.provenance.as_ref(), source_path.clone(), requested);
        if let (Some(provenance), Some(backend)) = (&self.provenance, self.backend.as_mut()) {
            self.breakpoints.sync(backend, provenance)?;
        }
        Ok(vec![response(json!({
            "breakpoints": self.breakpoints.dap_breakpoints(&source_path)
        }))])
    }

    fn generated_source(&self, arguments: &Value) -> Result<Vec<Value>, CliFailure> {
        let provenance = self
            .provenance
            .as_ref()
            .ok_or_else(|| debugger_failure("no debug provenance is loaded"))?;
        let path = arguments["path"]
            .as_str()
            .ok_or_else(|| debugger_failure("generated source request requires `path`"))?;
        let generated = provenance
            .debug
            .generated_files
            .iter()
            .find(|generated| generated.path == path)
            .ok_or_else(|| debugger_failure(format!("unknown generated source {path}")))?;
        let disk_path = generated_path(provenance, path);
        let disk_source = fs::read_to_string(&disk_path).ok().filter(|content| {
            terrane_compiler::debugging::hash_bytes(content.as_bytes()) == generated.content_hash
        });
        let content = disk_source
            .or_else(|| generated.embedded_source.clone())
            .ok_or_else(|| {
                debugger_failure(format!(
                    "generated source {} is unavailable and was not embedded",
                    disk_path.display()
                ))
            })?;
        Ok(vec![response(
            json!({"content": content, "mimeType": "text/x-rust"}),
        )])
    }
}

fn event(name: &str, body: Value) -> Value {
    let mut value = json!({"seq": 0, "type": "event", "event": name});
    value["body"] = body;
    value
}

fn response(body: Value) -> Value {
    let mut value = json!({"seq": 0, "type": "response", "success": true});
    value["body"] = body;
    value
}

fn take_backend_events(backend: &mut Backend) -> impl Iterator<Item = Value> + '_ {
    backend
        .events
        .drain(..)
        .filter(|event| !matches!(event["event"].as_str(), Some("initialized" | "breakpoint")))
}

fn with_backend_events(backend: &mut Backend, response: Value) -> Vec<Value> {
    let mut messages = vec![response];
    messages.extend(take_backend_events(backend));
    messages
}

fn backend_body_and_events(backend: &mut Backend, response: &Value) -> (Value, Vec<Value>) {
    (
        response["body"].clone(),
        take_backend_events(backend).collect(),
    )
}

struct Backend {
    child: Child,
    input: ChildStdin,
    output: Receiver<Result<Value, String>>,
    sequence: i64,
    events: VecDeque<Value>,
    responses: BTreeMap<i64, Value>,
}
fn backend_console_output(backend: &mut Backend, response: &Value) -> String {
    let mut output = response["body"]["result"]
        .as_str()
        .unwrap_or_default()
        .to_owned();
    let mut retained = VecDeque::new();
    while let Some(event) = backend.events.pop_front() {
        if event["event"] == "output" && event["body"]["category"] == "console" {
            output.push_str(event["body"]["output"].as_str().unwrap_or_default());
        } else {
            retained.push_back(event);
        }
    }
    backend.events = retained;
    output
}

fn discover_rust_lldb_formatter(rust_sysroot: &str) -> Option<PathBuf> {
    let formatter = Path::new(rust_sysroot)
        .join("lib")
        .join("rustlib")
        .join("etc")
        .join("lldb_lookup.py");
    formatter.is_file().then_some(formatter)
}

fn add_rust_lldb_init_commands(arguments: &mut Value, formatter: Option<&Path>) {
    let Some(formatter) = formatter else {
        return;
    };
    let Some(arguments) = arguments.as_object_mut() else {
        return;
    };
    let commands = arguments
        .entry("initCommands")
        .or_insert_with(|| Value::Array(Vec::new()));
    let Some(commands) = commands.as_array_mut() else {
        return;
    };
    commands.insert(
        0,
        format!(
            "?command script import {}",
            lldb_quote(&formatter.to_string_lossy())
        )
        .into(),
    );
    commands.insert(
        1,
        "?settings set target.process.unsupported-language-warnings false".into(),
    );
}

impl Backend {
    fn start() -> Result<Self, CliFailure> {
        let mut child = Command::new("lldb-dap")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|error| {
                debugger_failure(format!("cannot start selected lldb-dap backend: {error}"))
            })?;
        let input = child.stdin.take().expect("piped lldb-dap stdin");
        let stdout = child.stdout.take().expect("piped lldb-dap stdout");
        let (sender, output) = mpsc::channel();
        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                match read_message(&mut reader) {
                    Ok(Some(message)) => {
                        if sender.send(Ok(message)).is_err() {
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(error) => {
                        let _ = sender.send(Err(error.to_string()));
                        break;
                    }
                }
            }
        });
        Ok(Self {
            child,
            input,
            output,
            sequence: 1,
            events: VecDeque::new(),
            responses: BTreeMap::new(),
        })
    }

    fn send(&mut self, command: &str, arguments: Value) -> Result<i64, CliFailure> {
        let sequence = self.sequence;
        self.sequence += 1;
        let mut request = json!({
            "seq": sequence,
            "type": "request",
            "command": command,
        });
        request["arguments"] = arguments;
        write_message(&mut self.input, &request).map_err(protocol_failure)?;
        Ok(sequence)
    }

    fn request(&mut self, command: &str, arguments: Value) -> Result<Value, CliFailure> {
        self.request_with_timeout(command, arguments, None)?
            .1
            .ok_or_else(|| debugger_failure("lldb-dap request timed out"))
    }

    fn request_with_timeout(
        &mut self,
        command: &str,
        arguments: Value,
        timeout: Option<Duration>,
    ) -> Result<(i64, Option<Value>), CliFailure> {
        let sequence = self.send(command, arguments)?;
        self.wait_for_response(sequence, timeout)
            .map(|response| (sequence, response))
    }

    fn finish_request(&mut self, sequence: i64, timeout: Duration) -> Result<Value, CliFailure> {
        self.wait_for_response(sequence, Some(timeout))?
            .ok_or_else(|| {
                debugger_failure("lldb-dap launch response timed out after configuration")
            })
    }

    fn wait_for_response(
        &mut self,
        sequence: i64,
        timeout: Option<Duration>,
    ) -> Result<Option<Value>, CliFailure> {
        if let Some(response) = self.responses.remove(&sequence) {
            return validate_backend_response(response).map(Some);
        }
        let deadline = timeout.map(|timeout| Instant::now() + timeout);
        loop {
            let message = match deadline {
                Some(deadline) => {
                    let remaining = deadline.saturating_duration_since(Instant::now());
                    match self.output.recv_timeout(remaining) {
                        Ok(message) => message,
                        Err(RecvTimeoutError::Timeout) => return Ok(None),
                        Err(RecvTimeoutError::Disconnected) => {
                            return Err(debugger_failure("lldb-dap closed its protocol stream"));
                        }
                    }
                }
                None => self
                    .output
                    .recv()
                    .map_err(|_| debugger_failure("lldb-dap closed its protocol stream"))?,
            }
            .map_err(|error| protocol_failure(io::Error::other(error)))?;
            if message["type"] == "response" {
                let request_sequence = message["request_seq"].as_i64().unwrap_or(0);
                if request_sequence == sequence {
                    return validate_backend_response(message).map(Some);
                }
                self.responses.insert(request_sequence, message);
            } else if message["type"] == "event" {
                self.events.push_back(message);
            }
        }
    }

    fn wait_for_event(&mut self, names: &[&str]) -> Result<Value, CliFailure> {
        if let Some(index) = self.events.iter().position(|event| {
            event["event"]
                .as_str()
                .is_some_and(|name| names.contains(&name))
        }) {
            return Ok(self.events.remove(index).expect("queued event exists"));
        }
        loop {
            let message = self
                .output
                .recv()
                .map_err(|_| debugger_failure("lldb-dap closed before reporting process state"))?
                .map_err(|error| protocol_failure(io::Error::other(error)))?;
            if message["type"] == "event"
                && message["event"]
                    .as_str()
                    .is_some_and(|name| names.contains(&name))
            {
                return Ok(message);
            }
            if message["type"] == "event" {
                self.events.push_back(message);
            } else if message["type"] == "response" {
                let request_sequence = message["request_seq"].as_i64().unwrap_or(0);
                self.responses.insert(request_sequence, message);
            }
        }
    }
    fn emit_debuggee_output(&mut self) -> Result<(), CliFailure> {
        let mut retained = VecDeque::new();
        while let Some(event) = self.events.pop_front() {
            if event["event"] != "output" {
                retained.push_back(event);
                continue;
            }
            let output = event["body"]["output"].as_str().unwrap_or_default();
            if event["body"]["category"] == "stdout" {
                print!("{output}");
                io::stdout().flush().map_err(io_failure)?;
            } else {
                if output.starts_with("To get started with the debug console try ")
                    || output.starts_with("For more information visit https://lldb.llvm.org/")
                {
                    continue;
                }
                eprint!("{output}");
                if !output.ends_with('\n') {
                    eprintln!();
                }
                io::stderr().flush().map_err(io_failure)?;
            }
        }
        self.events = retained;
        Ok(())
    }

    fn debuggee_exit_code(&self) -> ExitCode {
        let code = self
            .events
            .iter()
            .rev()
            .find(|event| event["event"] == "exited")
            .and_then(|event| event["body"]["exitCode"].as_i64())
            .and_then(|code| u8::try_from(code).ok())
            .unwrap_or(1);
        ExitCode::from(code)
    }

    fn disconnect(&mut self, terminate: bool) -> Result<(), CliFailure> {
        let _ = self.request("disconnect", json!({"terminateDebuggee": terminate}))?;
        Ok(())
    }
}

fn validate_backend_response(message: Value) -> Result<Value, CliFailure> {
    if message["success"].as_bool().unwrap_or(false) {
        return Ok(message);
    }
    let detail = message["message"]
        .as_str()
        .or_else(|| {
            message
                .pointer("/body/error/format")
                .and_then(Value::as_str)
        })
        .map_or_else(
            || format!("lldb-dap request failed: {}", message["body"]),
            str::to_owned,
        );
    Err(debugger_failure(detail))
}

impl Drop for Backend {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
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

fn normalized_source_path(provenance: &ProvenanceManifest, path: &Path) -> PathBuf {
    let rooted = if path.is_absolute() {
        path.to_path_buf()
    } else {
        Path::new(&provenance.relocation.source_root).join(path)
    };
    rooted
        .canonicalize()
        .unwrap_or_else(|_| lexical_normalize(&rooted))
}

fn requested_source_paths(provenance: &ProvenanceManifest, path: &Path) -> Vec<PathBuf> {
    if path.is_absolute() {
        return vec![normalized_source_path(provenance, path)];
    }
    let mut candidates = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        let rooted = cwd.join(path);
        candidates.push(
            rooted
                .canonicalize()
                .unwrap_or_else(|_| lexical_normalize(&rooted)),
        );
    }
    let package_relative = normalized_source_path(provenance, path);
    if !candidates.contains(&package_relative) {
        candidates.push(package_relative);
    }
    candidates
}

fn lexical_normalize(path: &Path) -> PathBuf {
    use std::path::Component;

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() && !path.is_absolute() {
                    normalized.push(component);
                }
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component);
            }
        }
    }
    normalized
}

fn known_source_path(provenance: &ProvenanceManifest, requested: &Path) -> Option<PathBuf> {
    let authored = provenance
        .debug
        .sources
        .iter()
        .map(|source| normalized_source_path(provenance, Path::new(&source.uri)))
        .collect::<BTreeSet<_>>();
    requested_source_paths(provenance, requested)
        .into_iter()
        .find(|candidate| authored.contains(candidate))
}

fn source_matches(provenance: &ProvenanceManifest, requested: &Path, uri: &str) -> bool {
    let source = normalized_source_path(provenance, Path::new(uri));
    requested_source_paths(provenance, requested)
        .into_iter()
        .any(|candidate| candidate == source)
}

fn all_source_associations(
    provenance: &ProvenanceManifest,
) -> impl Iterator<Item = (&str, &DebugAssociation)> {
    provenance.debug.generated_files.iter().flat_map(|file| {
        file.associations
            .iter()
            .map(move |association| (file.path.as_str(), association))
    })
}

fn resolve_breakpoint(
    provenance: &ProvenanceManifest,
    requested_path: &Path,
    line: usize,
) -> Vec<BreakpointResolution> {
    let source = provenance
        .debug
        .sources
        .iter()
        .find(|source| source_matches(provenance, requested_path, &source.uri));
    let Some(source) = source else {
        return Vec::new();
    };
    let function = provenance
        .debug
        .functions
        .iter()
        .filter(|function| {
            function.source.source_id == source.id
                && function.source.line <= line
                && function.source.end_line >= line
        })
        .min_by_key(|function| function.source.end - function.source.start);
    let scope = provenance
        .debug
        .scopes
        .iter()
        .filter(|scope| {
            scope.source.source_id == source.id
                && scope.source.line <= line
                && scope.source.end_line >= line
                && function
                    .is_none_or(|function| scope.function_id.as_deref() == Some(&function.id))
        })
        .min_by_key(|scope| scope.source.end - scope.source.start);
    let mut candidates = all_source_associations(provenance)
        .filter(|(_, association)| {
            association.sequence_point
                && association
                    .causes
                    .iter()
                    .any(|cause| cause.source_id == source.id && cause.line == line)
        })
        .collect::<Vec<_>>();
    let adjusted = candidates.is_empty();
    if adjusted {
        let (Some(function), Some(scope)) = (function, scope) else {
            return Vec::new();
        };
        candidates = all_source_associations(provenance)
            .filter(|(_, association)| {
                association.sequence_point
                    && association.function_id.as_deref() == Some(&function.id)
                    && association.scope_ids.contains(&scope.id)
                    && association
                        .causes
                        .iter()
                        .any(|cause| cause.source_id == source.id && cause.line >= line)
            })
            .collect();
        let Some(distance) = candidates
            .iter()
            .flat_map(|(_, association)| {
                association
                    .causes
                    .iter()
                    .filter(|cause| cause.source_id == source.id && cause.line >= line)
                    .map(|cause| cause.line - line)
            })
            .min()
        else {
            return Vec::new();
        };
        candidates.retain(|(_, association)| {
            association.causes.iter().any(|cause| {
                cause.source_id == source.id && cause.line >= line && cause.line - line == distance
            })
        });
    }
    candidates
        .into_iter()
        .map(|(path, association)| {
            let source_line = association.causes.first().map_or(line, |cause| cause.line);
            BreakpointResolution {
                generated_path: path.to_owned(),
                generated_line: association.generated.line,
                source_line,
                message: if adjusted {
                    format!("adjusted to executable line {source_line}")
                } else {
                    "exact executable sequence point".to_owned()
                },
            }
        })
        .collect()
}

fn generated_path(provenance: &ProvenanceManifest, relative: &str) -> PathBuf {
    Path::new(&provenance.relocation.build_root).join(relative)
}

fn association_for_frame<'a>(
    provenance: &'a ProvenanceManifest,
    frame: &Value,
) -> Option<&'a DebugAssociation> {
    let path = frame["source"]["path"].as_str()?;
    let line = frame["line"]
        .as_u64()
        .and_then(|line| usize::try_from(line).ok())?;
    provenance
        .debug
        .generated_files
        .iter()
        .find(|file| Path::new(path).ends_with(&file.path))
        .and_then(|file| {
            file.associations.iter().find(|association| {
                association.sequence_point && association.generated.line == line
            })
        })
}

fn translate_stack_frames(body: &mut Value, provenance: &ProvenanceManifest, include_native: bool) {
    let Some(frames) = body["stackFrames"].as_array_mut() else {
        return;
    };
    for frame in frames.iter_mut() {
        let Some(association) = association_for_frame(provenance, frame) else {
            if !include_native {
                frame["presentationHint"] = "subtle".into();
            }
            continue;
        };
        let Some(cause) = association.causes.first() else {
            continue;
        };
        let Some(source) = provenance
            .debug
            .sources
            .iter()
            .find(|source| source.id == cause.source_id)
        else {
            continue;
        };
        frame["source"] = json!({"name": Path::new(&source.uri).file_name().unwrap_or_default(), "path": Path::new(&provenance.relocation.source_root).join(&source.uri)});
        frame["line"] = cause.line.into();
        frame["column"] = cause.column.into();
        if let Some(function) = association.function_id.as_deref().and_then(|id| {
            provenance
                .debug
                .functions
                .iter()
                .find(|function| function.id == id)
        }) {
            frame["name"] = function.name.clone().into();
        }
    }
}

fn frame_stop_context(frame: &Value, provenance: &ProvenanceManifest) -> Option<StopContext> {
    let association = association_for_frame(provenance, frame)?;
    let cause = association.causes.first()?;
    Some(StopContext {
        source_id: cause.source_id,
        position: cause.start,
        function_id: association.function_id.clone(),
        scope_ids: association.scope_ids.clone(),
    })
}

fn frame_stop_contexts(
    body: &Value,
    provenance: &ProvenanceManifest,
) -> BTreeMap<i64, StopContext> {
    body["stackFrames"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|frame| {
            Some((
                frame["id"].as_i64()?,
                frame_stop_context(frame, provenance)?,
            ))
        })
        .collect()
}

#[expect(
    clippy::too_many_lines,
    reason = "value translation keeps privacy, scope, layout, and response bounds in one reviewable boundary"
)]
fn translate_variables(
    body: &mut Value,
    provenance: &ProvenanceManifest,
    parent_reference: i64,
    variable_objects: &mut BTreeMap<i64, String>,
    value_summaries: &BTreeMap<String, String>,
    stop_context: Option<&StopContext>,
) {
    const MAX_VARIABLES: usize = 100;
    const MAX_VALUE_BYTES: usize = 4_096;
    let mut bindings = BTreeMap::new();
    for binding in &provenance.debug.bindings {
        let visible = stop_context.is_none_or(|context| {
            binding.source.source_id == context.source_id
                && binding.visible_from <= context.position
                && binding.visible_until >= context.position
                && binding.function_id == context.function_id
                && binding
                    .scope_id
                    .as_ref()
                    .is_none_or(|scope| context.scope_ids.contains(scope))
        });
        if !visible {
            continue;
        }
        let rank = stop_context
            .and_then(|context| {
                binding
                    .scope_id
                    .as_ref()
                    .and_then(|scope| context.scope_ids.iter().position(|id| id == scope))
            })
            .unwrap_or(0);
        let replace = bindings.get(binding.rust_name.as_str()).is_none_or(
            |(current_rank, current): &(usize, &terrane_compiler::debugging::DebugBinding)| {
                (rank, binding.visible_from) > (*current_rank, current.visible_from)
            },
        );
        if replace {
            bindings.insert(binding.rust_name.as_str(), (rank, binding));
        }
    }
    let parent_object = variable_objects.get(&parent_reference).and_then(|id| {
        provenance
            .debug
            .objects
            .iter()
            .find(|object| object.id == *id)
    });
    let Some(variables) = body["variables"].as_array_mut() else {
        return;
    };
    let mut last_logical_variable = BTreeMap::new();
    for (index, variable) in variables.iter().enumerate() {
        let backend_name = variable["name"].as_str().unwrap_or_default();
        let logical_name = backend_name
            .split_once(" @ ")
            .map_or(backend_name, |(name, _)| name);
        if bindings.contains_key(logical_name) {
            last_logical_variable.insert(logical_name.to_owned(), index);
        }
    }
    let mut index = 0;
    variables.retain(|variable| {
        let backend_name = variable["name"].as_str().unwrap_or_default();
        let logical_name = backend_name
            .split_once(" @ ")
            .map_or(backend_name, |(name, _)| name);
        let retain = last_logical_variable
            .get(logical_name)
            .is_none_or(|last| *last == index);
        index += 1;
        retain
    });
    let truncated_variables = variables.len().saturating_sub(MAX_VARIABLES);
    variables.truncate(MAX_VARIABLES);
    for variable in variables.iter_mut() {
        let backend_name = variable["name"].as_str().unwrap_or_default();
        let rust_name = backend_name
            .split_once(" @ ")
            .map_or(backend_name, |(name, _)| name);
        let presentation = parent_object
            .and_then(|object| {
                object
                    .fields
                    .iter()
                    .find(|field| field.rust_name == rust_name)
                    .map(|field| {
                        (
                            field.name.clone(),
                            field.object_id.clone(),
                            field.secret,
                            field.type_name.as_str(),
                        )
                    })
            })
            .or_else(|| {
                bindings.get(rust_name).map(|(_, binding)| {
                    (
                        binding.name.clone(),
                        binding.object_id.clone(),
                        false,
                        binding.type_name.as_str(),
                    )
                })
            });
        if let Some((name, object_id, secret, type_name)) = presentation {
            let is_object = object_id.is_some();
            let object_name = debug_object_name(object_id.as_deref(), type_name);
            variable["name"] = name.into();
            if secret {
                variable["value"] = "<secret>".into();
                variable["variablesReference"] = 0.into();
                variable["memoryReference"] = Value::Null;
                variable["evaluateName"] = Value::Null;
            } else {
                let reference = variable["variablesReference"].as_i64().unwrap_or(0);
                if reference != 0
                    && let Some(object_id) = object_id
                {
                    variable_objects.insert(reference, object_id);
                }
                let raw = variable["value"].as_str().unwrap_or_default();
                if is_object {
                    variable["value"] = object_name.into();
                } else if type_name == "Scalar(Int)" {
                    if let Some(summary) = variable["memoryReference"]
                        .as_str()
                        .and_then(|reference| value_summaries.get(reference))
                    {
                        variable["value"] = summary.clone().into();
                    }
                    variable["variablesReference"] = 0.into();
                } else if matches!(type_name, "Scalar(String)" | "Bytes")
                    && let Some(summary) = variable["memoryReference"]
                        .as_str()
                        .and_then(|reference| value_summaries.get(reference))
                {
                    variable["value"] = summary.clone().into();
                    variable["variablesReference"] = 0.into();
                } else if raw.contains("optimized out") {
                    variable["value"] = "<optimized out>".into();
                } else if raw.contains("unavailable") {
                    variable["value"] = "<unavailable debug information>".into();
                }
            }
        }
        if let Some(value) = variable["value"].as_str()
            && value.len() > MAX_VALUE_BYTES
        {
            let mut end = MAX_VALUE_BYTES;
            while !value.is_char_boundary(end) {
                end -= 1;
            }
            variable["value"] = format!("{}… <truncated>", &value[..end]).into();
        }
    }
    if truncated_variables != 0 {
        variables.push(json!({
            "name": "…",
            "value": format!("<truncated: {truncated_variables} more values; request another DAP page>"),
            "variablesReference": 0
        }));
    }
}
fn debug_object_name(object_id: Option<&str>, fallback: &str) -> String {
    object_id
        .and_then(|id| id.rsplit_once("::").map(|(_, name)| name))
        .unwrap_or(fallback)
        .to_owned()
}

fn request_terrane_variables(backend: &mut Backend, arguments: Value) -> Result<Value, CliFailure> {
    let _ = backend.request(
        "evaluate",
        json!({"expression": "`type category disable Rust", "context": "repl"}),
    );
    let result = backend.request("variables", arguments);
    let _ = backend.request(
        "evaluate",
        json!({"expression": "`type category enable Rust", "context": "repl"}),
    );
    result
}

fn read_value_summaries(
    backend: &mut Backend,
    body: &Value,
    selected_layout: bool,
) -> BTreeMap<String, String> {
    let mut summaries = BTreeMap::new();
    for variable in body["variables"].as_array().into_iter().flatten() {
        let Some(reference) = variable["memoryReference"].as_str() else {
            continue;
        };
        let type_name = variable["type"].as_str().unwrap_or_default();
        let has_recipe = type_name == "terrane_int_support::Int"
            || type_name.contains("string::String")
            || type_name.contains("Vec<u8");
        if !has_recipe {
            continue;
        }
        if !selected_layout {
            continue;
        }
        let summary = if type_name == "terrane_int_support::Int" {
            decode_adaptive_int(backend, reference)
        } else if type_name.contains("string::String") {
            decode_string(backend, reference)
        } else {
            decode_bytes(backend, reference)
        };
        summaries.insert(
            reference.to_owned(),
            summary.unwrap_or_else(|failure| failure),
        );
    }
    summaries
}

fn decode_string(backend: &mut Backend, reference: &str) -> Result<String, String> {
    let (bytes, truncated) = decode_vec_bytes(backend, reference)?;
    let text = String::from_utf8(bytes)
        .map_err(|_| "<unsupported layout: invalid string bytes>".to_owned())?;
    Ok(if truncated {
        format!("{text:?}… <truncated>")
    } else {
        format!("{text:?}")
    })
}

fn decode_bytes(backend: &mut Backend, reference: &str) -> Result<String, String> {
    use std::fmt::Write as _;

    let (bytes, truncated) = decode_vec_bytes(backend, reference)?;
    let mut rendered = String::from("b'");
    for byte in bytes {
        write!(rendered, "\\\\x{byte:02x}").expect("writing to a string cannot fail");
    }
    rendered.push('\'');
    if truncated {
        rendered.push_str("… <truncated>");
    }
    Ok(rendered)
}

fn decode_vec_bytes(backend: &mut Backend, reference: &str) -> Result<(Vec<u8>, bool), String> {
    const MAX_BYTES: usize = 4_096;
    let response = backend
        .request(
            "readMemory",
            json!({"memoryReference": reference, "offset": 0, "count": 24}),
        )
        .map_err(|failure| format!("<unavailable debug information: {}>", failure.message))?;
    let encoded = response["body"]["data"]
        .as_str()
        .ok_or_else(|| "<unavailable debug information>".to_owned())?;
    let header = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "<unsupported layout: invalid vector memory>".to_owned())?;
    if header.len() < 24 {
        return Err("<unavailable debug information>".to_owned());
    }
    let word = |index: usize| {
        let mut bytes = [0_u8; 8];
        bytes.copy_from_slice(&header[index * 8..index * 8 + 8]);
        u64::from_ne_bytes(bytes)
    };
    let pointer = word(1);
    let length = usize::try_from(word(2))
        .map_err(|_| "<unsupported layout: invalid vector length>".to_owned())?;
    let selected = length.min(MAX_BYTES);
    if selected == 0 {
        return Ok((Vec::new(), false));
    }
    let response = backend
        .request(
            "readMemory",
            json!({
                "memoryReference": format!("0x{pointer:x}"),
                "offset": 0,
                "count": selected
            }),
        )
        .map_err(|failure| format!("<unavailable debug information: {}>", failure.message))?;
    let encoded = response["body"]["data"]
        .as_str()
        .ok_or_else(|| "<unavailable debug information>".to_owned())?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "<unsupported layout: invalid vector contents>".to_owned())?;
    Ok((bytes, selected < length))
}

fn decode_adaptive_int(backend: &mut Backend, reference: &str) -> Result<String, String> {
    const NICHE: u64 = 1_u64 << 63;
    let response = backend
        .request(
            "readMemory",
            json!({"memoryReference": reference, "offset": 0, "count": 32}),
        )
        .map_err(|failure| format!("<unavailable debug information: {}>", failure.message))?;
    let encoded = response["body"]["data"]
        .as_str()
        .ok_or_else(|| "<unavailable debug information>".to_owned())?;
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "<unsupported layout: invalid adaptive-int memory>".to_owned())?;
    if bytes.len() < 32 {
        return Err("<unavailable debug information>".to_owned());
    }
    let word = |index: usize| {
        u64::from_le_bytes(
            bytes[index * 8..index * 8 + 8]
                .try_into()
                .expect("bounded word slice"),
        )
    };
    match word(0) {
        NICHE => Ok(word(1).cast_signed().to_string()),
        tag if tag == NICHE + 1 => {
            let value = i128::from_le_bytes(bytes[16..32].try_into().expect("wide payload"));
            Ok(value.to_string())
        }
        _ => {
            let length = usize::try_from(word(2))
                .map_err(|_| "<unsupported layout: invalid bigint length>".to_owned())?;
            if length > 1_024 {
                return Err(format!("<truncated: adaptive int has {length} limbs>"));
            }
            let pointer = word(1);
            let magnitude = backend
                .request(
                    "readMemory",
                    json!({
                        "memoryReference": format!("0x{pointer:x}"),
                        "offset": 0,
                        "count": length.saturating_mul(8)
                    }),
                )
                .map_err(|failure| {
                    format!("<unavailable debug information: {}>", failure.message)
                })?;
            let encoded = magnitude["body"]["data"]
                .as_str()
                .ok_or_else(|| "<unavailable debug information>".to_owned())?;
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .map_err(|_| "<unsupported layout: invalid bigint memory>".to_owned())?;
            let magnitude = BigUint::from_bytes_le(&bytes).to_string();
            match word(3) & 0xff {
                0 => Ok(format!("-{magnitude}")),
                1 => Ok("0".to_owned()),
                2 => Ok(magnitude),
                _ => Err("<unsupported layout: unknown bigint sign>".to_owned()),
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct StopLocation {
    frame_depth: usize,
    generated_path: String,
    generated_line: usize,
    function_id: Option<String>,
}

fn mapped_stop_location(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
) -> Result<Option<StopLocation>, CliFailure> {
    let response = backend.request(
        "stackTrace",
        json!({"threadId": thread_id, "startFrame": 0}),
    )?;
    let frames = response["body"]["stackFrames"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let frame_depth = response["body"]["totalFrames"]
        .as_u64()
        .and_then(|depth| usize::try_from(depth).ok())
        .unwrap_or(frames.len());
    Ok(frames.into_iter().find_map(|frame| {
        let association = association_for_frame(provenance, &frame)?;
        Some(StopLocation {
            frame_depth,
            generated_path: frame["source"]["path"].as_str()?.to_owned(),
            generated_line: association.generated.line,
            function_id: association.function_id.clone(),
        })
    }))
}

#[expect(
    clippy::too_many_lines,
    reason = "temporary breakpoint ownership and cleanup remain visibly paired"
)]
fn temporary_sequence_step(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    breakpoints: &BreakpointManager,
    thread_id: i64,
) -> Result<Option<Value>, CliFailure> {
    let response = backend.request(
        "stackTrace",
        json!({"threadId": thread_id, "startFrame": 0}),
    )?;
    let Some(frames) = response["body"]["stackFrames"].as_array() else {
        return Ok(None);
    };
    let Some(frame) = frames.first() else {
        return Ok(None);
    };
    let Some(current) = association_for_frame(provenance, frame) else {
        return Ok(None);
    };
    let origin_depth = response["body"]["totalFrames"]
        .as_u64()
        .and_then(|depth| usize::try_from(depth).ok())
        .unwrap_or(frames.len());
    let caller = frames
        .get(1)
        .and_then(|frame| association_for_frame(provenance, frame));
    let mut targets = BTreeSet::new();
    for file in &provenance.debug.generated_files {
        for association in &file.associations {
            if !association.sequence_point {
                continue;
            }
            let same_current_point = association.causes.iter().any(|candidate| {
                current.causes.iter().any(|cause| {
                    candidate.source_id == cause.source_id && candidate.line == cause.line
                })
            }) || (file.path
                == frame["source"]["path"].as_str().unwrap_or_default()
                && association.generated.line == current.generated.line);
            let same_function = association.function_id == current.function_id;
            let caller_function = caller.is_some_and(|caller| {
                association.function_id == caller.function_id
                    && association.generated.line != caller.generated.line
            });
            if !same_current_point && (same_function || caller_function) {
                targets.insert((file.path.clone(), association.generated.line));
            }
        }
    }
    if targets.is_empty() {
        return Ok(None);
    }
    if targets.len() > MAX_TEMPORARY_SEQUENCE_POINTS {
        return Ok(None);
    }
    let target_locations = targets
        .iter()
        .map(|(path, line)| (lexical_normalize(&generated_path(provenance, path)), *line))
        .collect::<BTreeSet<_>>();
    let current_file = frame["source"]["path"].as_str().unwrap_or_default();
    let suspended_ids = breakpoints.backend_ids_at(current_file, current.generated.line);

    let existing_ids = backend_breakpoint_ids(backend)?;
    let command_file = TemporaryBreakpointCommands::write(provenance, &targets)?;
    let response = backend.request(
        "evaluate",
        json!({
            "expression": format!(
                "`command source -s 0 {}",
                lldb_quote(&command_file.path.to_string_lossy())
            ),
            "context": "repl"
        }),
    )?;
    let _ = backend_console_output(backend, &response);
    let temporary_ids = backend_breakpoint_ids(backend)?
        .difference(&existing_ids)
        .copied()
        .collect::<Vec<_>>();
    if temporary_ids.len() != targets.len() {
        let _ = delete_backend_breakpoints(backend, &temporary_ids);
        let _ = set_backend_breakpoints_enabled(backend, &suspended_ids, true);
        return Ok(None);
    }
    if let Err(failure) = set_backend_breakpoints_enabled(backend, &suspended_ids, false) {
        let _ = delete_backend_breakpoints(backend, &temporary_ids);
        let _ = set_backend_breakpoints_enabled(backend, &suspended_ids, true);
        return Err(failure);
    }
    let result = (|| {
        backend.request(
            "continue",
            json!({"threadId": thread_id, "singleThread": false}),
        )?;
        loop {
            let mut event = backend.wait_for_event(&["stopped", "terminated"])?;
            if event["event"] != "stopped" {
                return Ok(event);
            }
            let reason = event["body"]["reason"].as_str().unwrap_or_default();
            if reason != "breakpoint" {
                return Ok(event);
            }
            let hit_ids = event["body"]["hitBreakpointIds"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_i64)
                .collect::<BTreeSet<_>>();
            if hit_ids
                .iter()
                .any(|id| breakpoints.contains_backend_id(*id))
            {
                return Ok(event);
            }
            let location = mapped_stop_location(backend, provenance, thread_id)?;
            let at_temporary_location = location.as_ref().is_some_and(|location| {
                target_locations.contains(&(
                    lexical_normalize(Path::new(&location.generated_path)),
                    location.generated_line,
                ))
            });
            let hit_temporary = hit_ids.iter().any(|id| temporary_ids.contains(id))
                || (hit_ids.is_empty() && at_temporary_location);
            if !hit_temporary {
                return Ok(event);
            }
            if location.is_some_and(|location| location.frame_depth <= origin_depth) {
                event["body"]["reason"] = "step".into();
                if let Some(body) = event["body"].as_object_mut() {
                    body.remove("description");
                    body.remove("hitBreakpointIds");
                }
                return Ok(event);
            }
            backend.request(
                "continue",
                json!({"threadId": thread_id, "singleThread": false}),
            )?;
        }
    })();
    if !temporary_ids.is_empty() {
        let _ = delete_backend_breakpoints(backend, &temporary_ids);
    }
    let _ = set_backend_breakpoints_enabled(backend, &suspended_ids, true);
    result.map(Some)
}

struct TemporaryBreakpointCommands {
    path: PathBuf,
}

impl TemporaryBreakpointCommands {
    fn write(
        provenance: &ProvenanceManifest,
        targets: &BTreeSet<(String, usize)>,
    ) -> Result<Self, CliFailure> {
        let sequence = NEXT_COMMAND_FILE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "terrane-debug-breakpoints-{}-{sequence}.lldb",
            std::process::id()
        ));
        let commands = targets
            .iter()
            .map(|(relative, line)| {
                let path = generated_path(provenance, relative);
                format!(
                    "breakpoint set --file {} --line {line}",
                    lldb_quote(&path.to_string_lossy())
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&path, format!("{commands}\n")).map_err(|error| {
            debugger_failure(format!(
                "cannot write temporary debugger commands {}: {error}",
                path.display()
            ))
        })?;
        Ok(Self { path })
    }
}

impl Drop for TemporaryBreakpointCommands {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn backend_breakpoint_ids(backend: &mut Backend) -> Result<BTreeSet<i64>, CliFailure> {
    let response = backend.request(
        "evaluate",
        json!({"expression": "`breakpoint list -b", "context": "repl"}),
    )?;
    Ok(
        parse_lldb_breakpoint_ids(&backend_console_output(backend, &response))
            .into_iter()
            .collect(),
    )
}

fn delete_backend_breakpoints(backend: &mut Backend, ids: &[i64]) -> Result<(), CliFailure> {
    if ids.is_empty() {
        return Ok(());
    }
    let ids = ids.iter().map(i64::to_string).collect::<Vec<_>>().join(" ");
    backend.request(
        "evaluate",
        json!({
            "expression": format!("`breakpoint delete {ids}"),
            "context": "repl"
        }),
    )?;
    Ok(())
}

fn set_backend_breakpoints_enabled(
    backend: &mut Backend,
    ids: &[i64],
    enabled: bool,
) -> Result<(), CliFailure> {
    if ids.is_empty() {
        return Ok(());
    }
    let ids = ids.iter().map(i64::to_string).collect::<Vec<_>>().join(" ");
    backend.request(
        "evaluate",
        json!({
            "expression": format!(
                "`breakpoint {} {ids}",
                if enabled { "enable" } else { "disable" }
            ),
            "context": "repl"
        }),
    )?;
    Ok(())
}

fn parse_lldb_breakpoint_ids(output: &str) -> Vec<i64> {
    output
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let identifier = line
                .strip_prefix("Breakpoint ")
                .unwrap_or(line)
                .split_once(':')?
                .0;
            identifier.parse().ok()
        })
        .collect()
}

fn step_to_source(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    breakpoints: &BreakpointManager,
    command: &str,
    thread_id: i64,
) -> Result<Value, CliFailure> {
    if command == "next"
        && let Some(event) = temporary_sequence_step(backend, provenance, breakpoints, thread_id)?
    {
        return Ok(event);
    }
    let origin = mapped_stop_location(backend, provenance, thread_id)?;
    let suspended_ids = origin.as_ref().map_or_else(Vec::new, |origin| {
        breakpoints.backend_ids_at(&origin.generated_path, origin.generated_line)
    });
    set_backend_breakpoints_enabled(backend, &suspended_ids, false)?;
    let result = (|| {
        let mut native_command = command;
        for _ in 0..MAX_RAW_STEPS {
            backend.request(
                native_command,
                json!({"threadId": thread_id, "singleThread": true, "granularity": "statement"}),
            )?;
            let event = backend.wait_for_event(&["stopped", "terminated"])?;
            if event["event"] != "stopped" {
                return Ok(event);
            }
            let reason = event["body"]["reason"].as_str().unwrap_or_default();
            if !matches!(reason, "step" | "entry" | "") {
                return Ok(event);
            }
            if command == "stepOut" {
                native_command = "next";
            }
            let Some(current) = mapped_stop_location(backend, provenance, thread_id)? else {
                continue;
            };
            let Some(origin) = &origin else {
                return Ok(event);
            };
            let changed_point = current.generated_path != origin.generated_path
                || current.generated_line != origin.generated_line
                || current.function_id != origin.function_id;
            let reached_source_target = match command {
                "next" => {
                    current.frame_depth < origin.frame_depth
                        || (current.frame_depth == origin.frame_depth && changed_point)
                }
                "stepOut" => current.frame_depth < origin.frame_depth,
                _ => current.frame_depth > origin.frame_depth || changed_point,
            };
            if reached_source_target {
                return Ok(event);
            }
        }
        eprintln!(
            "source progress unavailable after {MAX_RAW_STEPS} bounded native steps; exposing the native stop"
        );
        Ok(json!({
            "seq": 0,
            "type": "event",
            "event": "stopped",
            "body": {"reason": "step", "threadId": thread_id}
        }))
    })();
    let restored = set_backend_breakpoints_enabled(backend, &suspended_ids, true);
    match result {
        Ok(event) => {
            restored?;
            Ok(event)
        }
        Err(failure) => Err(failure),
    }
}

fn lldb_quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
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
            request_terrane_variables(backend, json!({"variablesReference": reference}))?
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
        let response =
            request_terrane_variables(backend, json!({"variablesReference": reference}))?;
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
    let response = request_terrane_variables(backend, json!({"variablesReference": reference}))?;
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
    use super::*;

    fn provenance() -> ProvenanceManifest {
        let mut provenance: ProvenanceManifest = serde_json::from_value(json!({
            "schema_version": "1.2",
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
                "schema_version": "1.2",
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
