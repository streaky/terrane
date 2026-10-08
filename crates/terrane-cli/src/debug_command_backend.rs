use super::{
    BTreeMap, BufReader, Child, ChildStdin, CliFailure, Command, Duration, ExitCode, Instant, Path,
    PathBuf, Receiver, RecvTimeoutError, Stdio, Value, VecDeque, Write, debugger_failure, io,
    io_failure, json, mpsc, protocol_failure, read_message, steps::lldb_quote, thread,
    write_message,
};

pub(super) fn event(name: &str, body: Value) -> Value {
    let mut value = json!({"seq": 0, "type": "event", "event": name});
    value["body"] = body;
    value
}

pub(super) fn response(body: Value) -> Value {
    let mut value = json!({"seq": 0, "type": "response", "success": true});
    value["body"] = body;
    value
}

pub(super) fn take_backend_events(backend: &mut Backend) -> impl Iterator<Item = Value> + '_ {
    backend
        .events
        .drain(..)
        .filter(|event| !matches!(event["event"].as_str(), Some("initialized" | "breakpoint")))
}

pub(super) fn with_backend_events(backend: &mut Backend, response: Value) -> Vec<Value> {
    let mut messages = vec![response];
    messages.extend(take_backend_events(backend));
    messages
}

pub(super) fn backend_body_and_events(
    backend: &mut Backend,
    response: &Value,
) -> (Value, Vec<Value>) {
    (
        response["body"].clone(),
        take_backend_events(backend).collect(),
    )
}

pub(super) struct Backend {
    child: Child,
    input: ChildStdin,
    output: Receiver<Result<Value, String>>,
    sequence: i64,
    pub(super) events: VecDeque<Value>,
    responses: BTreeMap<i64, Value>,
}
pub(super) fn backend_console_output(backend: &mut Backend, response: &Value) -> String {
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

pub(super) fn discover_rust_lldb_formatter(rust_sysroot: &str) -> Option<PathBuf> {
    let formatter = Path::new(rust_sysroot)
        .join("lib")
        .join("rustlib")
        .join("etc")
        .join("lldb_lookup.py");
    formatter.is_file().then_some(formatter)
}

pub(super) fn add_rust_lldb_init_commands(arguments: &mut Value, formatter: Option<&Path>) {
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
    pub(super) fn start() -> Result<Self, CliFailure> {
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

    pub(super) fn send(&mut self, command: &str, arguments: Value) -> Result<i64, CliFailure> {
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

    pub(super) fn request(&mut self, command: &str, arguments: Value) -> Result<Value, CliFailure> {
        self.request_with_timeout(command, arguments, None)?
            .1
            .ok_or_else(|| debugger_failure("lldb-dap request timed out"))
    }

    pub(super) fn request_with_timeout(
        &mut self,
        command: &str,
        arguments: Value,
        timeout: Option<Duration>,
    ) -> Result<(i64, Option<Value>), CliFailure> {
        let sequence = self.send(command, arguments)?;
        self.wait_for_response(sequence, timeout)
            .map(|response| (sequence, response))
    }

    pub(super) fn finish_request(
        &mut self,
        sequence: i64,
        timeout: Duration,
    ) -> Result<Value, CliFailure> {
        self.wait_for_response(sequence, Some(timeout))?
            .ok_or_else(|| {
                debugger_failure("lldb-dap launch response timed out after configuration")
            })
    }

    pub(super) fn wait_for_response(
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

    pub(super) fn wait_for_event(&mut self, names: &[&str]) -> Result<Value, CliFailure> {
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
    pub(super) fn emit_debuggee_output(&mut self) -> Result<(), CliFailure> {
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

    pub(super) fn debuggee_exit_code(&self) -> ExitCode {
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

    pub(super) fn disconnect(&mut self, terminate: bool) -> Result<(), CliFailure> {
        let _ = self.request("disconnect", json!({"terminateDebuggee": terminate}))?;
        Ok(())
    }
}

pub(super) fn validate_backend_response(message: Value) -> Result<Value, CliFailure> {
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
