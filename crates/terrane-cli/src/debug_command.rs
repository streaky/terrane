use std::collections::{BTreeMap, VecDeque};
use std::ffi::OsString;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitCode, Stdio};

use serde_json::{Value, json};
use terrane_compiler::debugging::{DebugAssociation, ProvenanceManifest};

use super::CliFailure;

const MAX_RAW_STEPS: usize = 64;

pub(super) fn write_provenance(
    build_root: &Path,
    executable: &Path,
    provenance: &ProvenanceManifest,
) -> Result<PathBuf, CliFailure> {
    let mut bytes = serde_json::to_vec_pretty(provenance)
        .map_err(|error| CliFailure::backend(format!("cannot encode debug provenance: {error}")))?;
    bytes.push(b'\n');
    let generated_sidecar = build_root.join("terrane-debug.json");
    let executable_sidecar = executable.with_extension(format!(
        "{}terrane-debug.json",
        executable
            .extension()
            .map_or_else(String::new, |extension| format!(
                "{}.",
                extension.to_string_lossy()
            ))
    ));
    super::write_if_changed(&generated_sidecar, &bytes)
        .map_err(|error| CliFailure::backend(format!("cannot write debug provenance: {error}")))?;
    super::write_if_changed(&executable_sidecar, &bytes).map_err(|error| {
        CliFailure::backend(format!("cannot write executable debug provenance: {error}"))
    })?;
    Ok(generated_sidecar)
}

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
    backend.send(
        "launch",
        json!({
            "program": executable,
            "args": arguments,
            "cwd": provenance.relocation.source_root,
            "stopOnEntry": true,
            "disableASLR": false
        }),
    )?;
    backend.request("configurationDone", json!({}))?;
    let stopped = backend.wait_for_event(&["stopped", "terminated"])?;
    backend.emit_debuggee_output()?;
    let mut thread_id = stopped["body"]["threadId"].as_i64().unwrap_or(1);
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
                "break <source>:<line> | continue | next | step | out | frames | native | locals | registers | generated | lldb <command> | quit"
            ),
            ("break", location) => {
                let (path, line) = parse_breakpoint(location)?;
                let resolutions = resolve_breakpoint(&provenance, &path, line);
                if resolutions.is_empty() {
                    eprintln!(
                        "unverified breakpoint at {}:{line}: no executable sequence point",
                        path.display()
                    );
                    continue;
                }
                for resolution in &resolutions {
                    let response = backend.request(
                        "setBreakpoints",
                        json!({
                            "source": {"path": generated_path(&provenance, &resolution.generated_path)},
                            "breakpoints": [{"line": resolution.generated_line}],
                            "sourceModified": false
                        }),
                    )?;
                    let verified = response["body"]["breakpoints"][0]["verified"]
                        .as_bool()
                        .unwrap_or(false);
                    eprintln!(
                        "{} breakpoint {}:{} -> {}:{}",
                        if verified { "verified" } else { "unverified" },
                        path.display(),
                        line,
                        resolution.generated_path,
                        resolution.generated_line
                    );
                }
            }
            ("continue", _) => {
                backend.request("continue", json!({"threadId": thread_id}))?;
                let event = backend.wait_for_event(&["stopped", "terminated"])?;
                backend.emit_debuggee_output()?;
                if event["event"] != "stopped" {
                    return Ok(ExitCode::SUCCESS);
                }
                thread_id = event["body"]["threadId"].as_i64().unwrap_or(thread_id);
                show_top_frame(&mut backend, &provenance, thread_id)?;
            }
            ("next" | "step" | "out", _) => {
                let request = match command {
                    "next" => "next",
                    "step" => "stepIn",
                    _ => "stepOut",
                };
                step_to_source(&mut backend, &provenance, request, thread_id)?;
                backend.emit_debuggee_output()?;
                show_top_frame(&mut backend, &provenance, thread_id)?;
            }
            ("frames", _) => show_frames(&mut backend, &provenance, thread_id, false)?,
            ("native", _) => show_frames(&mut backend, &provenance, thread_id, true)?,
            ("locals", _) => show_variables(&mut backend, &provenance, thread_id, false)?,
            ("registers", _) => show_variables(&mut backend, &provenance, thread_id, true)?,
            ("generated", _) => show_generated(&mut backend, &provenance, thread_id)?,
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
}

impl Adapter {
    fn handle(&mut self, command: &str, arguments: Value) -> Result<Vec<Value>, CliFailure> {
        if command == "initialize" {
            let backend = self.backend.get_or_insert(Backend::start()?);
            let _ = backend.request("initialize", arguments)?;
            return Ok(with_backend_events(
                backend,
                response(json!({
                    "supportsConfigurationDoneRequest": true,
                    "supportsCancelRequest": true,
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
            ));
        }
        if command == "launch" || command == "attach" {
            let executable = arguments["program"]
                .as_str()
                .map(PathBuf::from)
                .ok_or_else(|| {
                    debugger_failure("launch/attach requires a native `program` path")
                })?;
            let sidecar = arguments["terraneProvenance"]
                .as_str()
                .map(PathBuf::from)
                .unwrap_or_else(|| executable.with_extension("terrane-debug.json"));
            let translation = load_and_validate(
                &sidecar,
                &executable,
                arguments.get("terraneRelocation"),
            )
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
            for name in [
                "terraneProvenance",
                "terraneRelocation",
                "useBuildSnapshot",
            ] {
                backend_arguments
                    .as_object_mut()
                    .expect("DAP arguments are an object")
                    .remove(name);
            }
            if command == "launch"
                && let Ok(provenance) = &translation
            {
                backend_arguments["cwd"] = provenance.relocation.source_root.clone().into();
            }
            let backend = self
                .backend
                .as_mut()
                .ok_or_else(|| debugger_failure("initialize must precede launch or attach"))?;
            backend.send(command, backend_arguments)?;
            self.launched = command == "launch";
            self.executable = Some(executable);
            let mut messages = with_backend_events(backend, response(json!({})));
            match translation {
                Ok(provenance) => self.provenance = Some(provenance),
                Err(failure) => {
                    self.provenance = None;
                    messages.push(json!({
                        "seq": 0,
                        "type": "event",
                        "event": "output",
                        "body": {
                            "category": "console",
                            "output": format!(
                                "{}Terrane source translation is disabled; raw native debugging remains available.\\n",
                                failure.message
                            )
                        }
                    }));
                }
            }
            return Ok(messages);
        }
        if command == "setBreakpoints" {
            return self.set_breakpoints(arguments);
        }
        if command == "stackTrace" {
            let provenance = self.provenance.clone();
            let backend = self.backend_mut()?;
            let backend_response = backend.request(command, arguments)?;
            let mut body = backend_response["body"].clone();
            if let Some(provenance) = &provenance {
                translate_stack_frames(&mut body, provenance, false);
            }
            return Ok(with_backend_events(backend, response(body)));
        }
        if command == "variables" {
            let provenance = self.provenance.clone();
            let backend = self.backend_mut()?;
            let backend_response = backend.request(command, arguments)?;
            let mut body = backend_response["body"].clone();
            if let Some(provenance) = &provenance {
                translate_variables(&mut body, provenance);
            }
            return Ok(with_backend_events(backend, response(body)));
        }
        if command == "terrane/generatedSource" {
            return self.generated_source(arguments);
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
        let backend = self.backend_mut()?;
        let backend_response = backend.request(command, arguments)?;
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

    fn set_breakpoints(&mut self, arguments: Value) -> Result<Vec<Value>, CliFailure> {
        let provenance = self
            .provenance
            .as_ref()
            .ok_or_else(|| debugger_failure("launch or attach must precede source breakpoints"))?
            .clone();
        let source_path = arguments["source"]["path"]
            .as_str()
            .map(PathBuf::from)
            .ok_or_else(|| debugger_failure("source breakpoints require a source path"))?;
        let requested = arguments["breakpoints"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let mut returned = Vec::new();
        for breakpoint in requested {
            let line = breakpoint["line"]
                .as_u64()
                .and_then(|line| usize::try_from(line).ok())
                .unwrap_or(0);
            let resolutions = resolve_breakpoint(&provenance, &source_path, line);
            if resolutions.is_empty() {
                returned.push(json!({"verified": false, "line": line, "message": "no executable Terrane sequence point in this function/scope"}));
                continue;
            }
            for resolution in resolutions {
                let backend = self.backend_mut()?;
                let backend_response = backend.request(
                    "setBreakpoints",
                    json!({
                        "source": {"path": generated_path(&provenance, &resolution.generated_path)},
                        "breakpoints": [{"line": resolution.generated_line}],
                        "sourceModified": false
                    }),
                )?;
                let mut native = backend_response["body"]["breakpoints"][0].clone();
                native["line"] = resolution.source_line.into();
                native["source"] = json!({"path": source_path});
                native["message"] = resolution.message.into();
                returned.push(native);
            }
        }
        Ok(vec![response(json!({"breakpoints": returned}))])
    }

    fn generated_source(&self, arguments: Value) -> Result<Vec<Value>, CliFailure> {
        let provenance = self
            .provenance
            .as_ref()
            .ok_or_else(|| debugger_failure("no debug provenance is loaded"))?;
        let path = arguments["path"]
            .as_str()
            .ok_or_else(|| debugger_failure("generated source request requires `path`"))?;
        let path = generated_path(provenance, path);
        let content = fs::read_to_string(&path).map_err(|error| {
            debugger_failure(format!(
                "cannot read generated source {}: {error}",
                path.display()
            ))
        })?;
        Ok(vec![response(
            json!({"content": content, "mimeType": "text/x-rust"}),
        )])
    }
}

fn response(body: Value) -> Value {
    json!({"seq": 0, "type": "response", "success": true, "body": body})
}

fn with_backend_events(backend: &mut Backend, response: Value) -> Vec<Value> {
    let mut messages = vec![response];
    while let Some(event) = backend.events.pop_front() {
        messages.push(event);
    }
    messages
}

struct Backend {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    sequence: i64,
    events: VecDeque<Value>,
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
        let output = BufReader::new(child.stdout.take().expect("piped lldb-dap stdout"));
        Ok(Self {
            child,
            input,
            output,
            sequence: 1,
            events: VecDeque::new(),
        })
    }

    fn send(&mut self, command: &str, arguments: Value) -> Result<i64, CliFailure> {
        let sequence = self.sequence;
        self.sequence += 1;
        write_message(
            &mut self.input,
            &json!({
                "seq": sequence,
                "type": "request",
                "command": command,
                "arguments": arguments
            }),
        )
        .map_err(protocol_failure)?;
        Ok(sequence)
    }

    fn request(&mut self, command: &str, arguments: Value) -> Result<Value, CliFailure> {
        let sequence = self.send(command, arguments)?;
        loop {
            let message = read_message(&mut self.output)
                .map_err(protocol_failure)?
                .ok_or_else(|| debugger_failure("lldb-dap closed its protocol stream"))?;
            if message["type"] == "response" && message["request_seq"] == sequence {
                if !message["success"].as_bool().unwrap_or(false) {
                    return Err(debugger_failure(
                        message["message"]
                            .as_str()
                            .unwrap_or("lldb-dap request failed"),
                    ));
                }
                return Ok(message);
            }
            if message["type"] == "event" {
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
            let message = read_message(&mut self.output)
                .map_err(protocol_failure)?
                .ok_or_else(|| {
                    debugger_failure("lldb-dap closed before reporting process state")
                })?;
            if message["type"] == "event"
                && message["event"]
                    .as_str()
                    .is_some_and(|name| names.contains(&name))
            {
                return Ok(message);
            }
            if message["type"] == "event" {
                self.events.push_back(message);
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
                eprint!("{output}");
                io::stderr().flush().map_err(io_failure)?;
            }
        }
        self.events = retained;
        Ok(())
    }

    fn disconnect(&mut self, terminate: bool) -> Result<(), CliFailure> {
        let _ = self.request("disconnect", json!({"terminateDebuggee": terminate}))?;
        Ok(())
    }
}

impl Drop for Backend {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[derive(Clone)]
struct BreakpointResolution {
    generated_path: String,
    generated_line: usize,
    source_line: usize,
    message: String,
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
        let Some(function) = function else {
            return Vec::new();
        };
        candidates = all_source_associations(provenance)
            .filter(|(_, association)| {
                association.sequence_point
                    && association.function_id.as_deref() == Some(&function.id)
            })
            .collect();
        let Some(distance) = candidates
            .iter()
            .flat_map(|(_, association)| {
                association
                    .causes
                    .iter()
                    .map(|cause| cause.line.abs_diff(line))
            })
            .min()
        else {
            return Vec::new();
        };
        candidates.retain(|(_, association)| {
            association
                .causes
                .iter()
                .any(|cause| cause.line.abs_diff(line) == distance)
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
                    format!(
                        "requested line {line}; adjusted to declared sequence point {source_line}"
                    )
                } else {
                    format!("requested and resolved line {line}")
                },
            }
        })
        .collect()
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

fn source_matches(provenance: &ProvenanceManifest, requested: &Path, uri: &str) -> bool {
    requested == Path::new(uri)
        || requested.ends_with(uri)
        || Path::new(&provenance.relocation.source_root).join(uri) == requested
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
            file.associations
                .iter()
                .find(|association| association.generated.line == line)
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

fn translate_variables(body: &mut Value, provenance: &ProvenanceManifest) {
    const MAX_VARIABLES: usize = 100;
    const MAX_VALUE_BYTES: usize = 4_096;
    let bindings = provenance
        .debug
        .bindings
        .iter()
        .map(|binding| (binding.rust_name.as_str(), binding))
        .collect::<BTreeMap<_, _>>();
    let Some(variables) = body["variables"].as_array_mut() else {
        return;
    };
    variables.truncate(MAX_VARIABLES);
    for variable in variables {
        let binding = variable["name"]
            .as_str()
            .and_then(|name| bindings.get(name))
            .copied();
        if let Some(binding) = binding {
            variable["name"] = binding.name.clone().into();
            let raw = variable["value"].as_str().unwrap_or_default();
            if binding.type_name == "Scalar(Int)"
                && (raw.contains("terrane_int_support::Int") || raw == binding.type_name)
            {
                variable["value"] = "<unsupported layout: adaptive int>".into();
                variable["variablesReference"] = 0.into();
            } else if raw.contains("optimized out") {
                variable["value"] = "<optimized out>".into();
            } else if raw.contains("unavailable") {
                variable["value"] = "<unavailable debug information>".into();
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
}

fn step_to_source(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    command: &str,
    thread_id: i64,
) -> Result<(), CliFailure> {
    for _ in 0..MAX_RAW_STEPS {
        backend.request(
            command,
            json!({"threadId": thread_id, "singleThread": true, "granularity": "statement"}),
        )?;
        let event = backend.wait_for_event(&["stopped", "terminated"])?;
        if event["event"] != "stopped" {
            return Ok(());
        }
        let frames = backend.request(
            "stackTrace",
            json!({"threadId": thread_id, "startFrame": 0, "levels": 1}),
        )?;
        if frames["body"]["stackFrames"][0]
            .as_object()
            .is_some_and(|frame| {
                association_for_frame(provenance, &Value::Object(frame.clone())).is_some()
            })
        {
            return Ok(());
        }
    }
    eprintln!(
        "source progress unavailable after {MAX_RAW_STEPS} bounded native steps; exposing the native stop"
    );
    Ok(())
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
    for frame in body["stackFrames"].as_array().into_iter().flatten() {
        if !native && frame["presentationHint"] == "subtle" {
            continue;
        }
        eprintln!(
            "#{:<3} {} at {}:{}",
            frame["id"].as_i64().unwrap_or(0),
            frame["name"].as_str().unwrap_or("<native>"),
            frame["source"]["path"].as_str().unwrap_or("<unknown>"),
            frame["line"].as_u64().unwrap_or(0)
        );
    }
    Ok(())
}

fn selected_frame(backend: &mut Backend, thread_id: i64) -> Result<i64, CliFailure> {
    let response = backend.request(
        "stackTrace",
        json!({"threadId": thread_id, "startFrame": 0, "levels": 1}),
    )?;
    response["body"]["stackFrames"][0]["id"]
        .as_i64()
        .ok_or_else(|| debugger_failure("selected thread has no frame"))
}

fn show_variables(
    backend: &mut Backend,
    provenance: &ProvenanceManifest,
    thread_id: i64,
    registers: bool,
) -> Result<(), CliFailure> {
    let frame_id = selected_frame(backend, thread_id)?;
    let scopes = backend.request("scopes", json!({"frameId": frame_id}))?;
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
        let response = backend.request("variables", json!({"variablesReference": reference}))?;
        let mut body = response["body"].clone();
        if !registers {
            translate_variables(&mut body, provenance);
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

fn show_generated(
    backend: &mut Backend,
    _provenance: &ProvenanceManifest,
    thread_id: i64,
) -> Result<(), CliFailure> {
    let response = backend.request(
        "stackTrace",
        json!({"threadId": thread_id, "startFrame": 0, "levels": 1}),
    )?;
    let frame = &response["body"]["stackFrames"][0];
    let path = frame["source"]["path"]
        .as_str()
        .map(PathBuf::from)
        .ok_or_else(|| debugger_failure("selected frame has no generated source"))?;
    let line = frame["line"].as_u64().unwrap_or(0);
    eprintln!("{}:{line}", path.display());
    Ok(())
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

fn load_and_validate(
    sidecar: &Path,
    executable: &Path,
    relocation: Option<&Value>,
) -> Result<ProvenanceManifest, CliFailure> {
    let bytes = fs::read(sidecar).map_err(|error| debugger_failure(format!("Terrane translation unavailable: cannot read provenance sidecar {}: {error}; raw native debugging remains available", sidecar.display())))?;
    let mut provenance: ProvenanceManifest = serde_json::from_slice(&bytes).map_err(|error| debugger_failure(format!("Terrane translation unavailable: invalid provenance sidecar {}: {error}; raw native debugging remains available", sidecar.display())))?;
    if let Some(relocation) = relocation {
        if let Some(build_root) = relocation["buildRoot"].as_str() {
            provenance.relocation.build_root = build_root.to_owned();
        }
        if let Some(source_root) = relocation["sourceRoot"].as_str() {
            provenance.relocation.source_root = source_root.to_owned();
        }
    }
    if provenance.schema_version != terrane_compiler::debugging::SCHEMA_VERSION {
        return Err(debugger_failure(format!(
            "unsupported debug provenance schema {}",
            provenance.schema_version
        )));
    }
    provenance
        .validate_executable(executable)
        .map_err(debugger_failure)?;
    for generated in &provenance.debug.generated_files {
        let path = generated_path(&provenance, &generated.path);
        let actual = fs::read(&path)
            .ok()
            .map(|bytes| terrane_compiler::debugging::hash_bytes(&bytes));
        if actual.as_deref() != Some(generated.content_hash.as_str()) {
            return Err(debugger_failure(format!(
                "Terrane translation unavailable: generated source identity mismatch for {}; raw native debugging remains available",
                path.display()
            )));
        }
    }
    Ok(provenance)
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

fn debugger_failure(message: impl Into<String>) -> CliFailure {
    CliFailure::diagnostic(PathBuf::from("<debugger>"), "S5001", message.into(), 5)
}

fn protocol_failure(error: io::Error) -> CliFailure {
    debugger_failure(format!("debug adapter protocol failure: {error}"))
}

fn io_failure(error: io::Error) -> CliFailure {
    debugger_failure(format!("debugger terminal I/O failure: {error}"))
}
