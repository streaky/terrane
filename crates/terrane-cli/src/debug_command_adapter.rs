use super::{
    BTreeMap, Backend, BreakpointManager, BufReader, CliFailure, Duration, ExitCode, OsString,
    Path, PathBuf, ProvenanceManifest, Value, add_rust_lldb_init_commands,
    backend::{backend_body_and_events, response, take_backend_events, with_backend_events},
    contextual_debugger_failure, debugger_failure, discover_rust_lldb_formatter, event,
    executable_sidecar, fs, generated_path, io, json, known_source_path, load_provenance,
    protocol_failure, read_message, read_value_summaries, request_terrane_variables,
    source::frame_stop_contexts,
    step_to_source, supports_adaptive_int_layout, translate_stack_frames, translate_variables,
    validate_provenance, write_message,
};

pub(crate) fn run_adapter(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
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
pub(super) struct Adapter {
    backend: Option<Backend>,
    provenance: Option<ProvenanceManifest>,
    executable: Option<PathBuf>,
    launched: bool,
    pub(super) variable_objects: BTreeMap<i64, String>,
    frame_contexts: BTreeMap<i64, StopContext>,
    variable_contexts: BTreeMap<i64, StopContext>,
    pending_launch_response: Option<i64>,
    pending_launch_context: Option<String>,
    breakpoints: BreakpointManager,
}

#[derive(Clone)]
pub(super) struct StopContext {
    pub(super) source_id: u32,
    pub(super) position: usize,
    pub(super) function_id: Option<String>,
    pub(super) scope_ids: Vec<String>,
}

impl Adapter {
    #[expect(
        clippy::too_many_lines,
        reason = "central dispatch keeps DAP request correlation and debugger lifecycle ordering explicit"
    )]
    pub(super) fn handle(
        &mut self,
        command: &str,
        arguments: Value,
    ) -> Result<Vec<Value>, CliFailure> {
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
                let backend_response = if let Some(provenance) = &provenance {
                    request_terrane_variables(backend, arguments, provenance)?
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
