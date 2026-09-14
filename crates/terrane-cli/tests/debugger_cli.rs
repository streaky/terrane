#![cfg(target_os = "linux")]

use std::fs;
use std::io::{BufRead as _, BufReader, Read as _, Write as _};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::{Value, json};

static NEXT_TEMP: AtomicUsize = AtomicUsize::new(0);

struct DebugFixture {
    root: PathBuf,
    source: PathBuf,
}

impl DebugFixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "terrane-debugger-π-{}-{}",
            std::process::id(),
            NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        fs::create_dir_all(&root).unwrap();
        let source = root.join("scalar.trn");
        fs::write(
            &source,
            concat!(
                "namespace debugger\n",
                "from /core/output import print\n",
                "function main;\n",
                "  small = 41\n",
                "  wide = 9223372036854775808\n",
                "  big = 170141183460469231731687303715884105728\n",
                "  text = 'hello'\n",
                "  small = small + 1\n",
                "  print; small\n",
            ),
        )
        .unwrap();
        Self { root, source }
    }

    fn build(&self) -> (PathBuf, PathBuf) {
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .args(["debug", self.source.to_str().unwrap()])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                child.stdin.take().unwrap().write_all(b"continue\n")?;
                child.wait_with_output()
            })
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, b"42\r\n");
        let build = fs::read_dir(self.root.join(".trn/build"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| path.join("terrane-debug.json").is_file())
            .unwrap();
        let mut executable = build.join("artifacts/debug/terrane_program");
        executable.set_extension(std::env::consts::EXE_EXTENSION);
        (executable, build.join("terrane-debug.json"))
    }
}

impl Drop for DebugFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn cli_hits_source_breakpoint_and_inspects_preserved_scalar() {
    let fixture = DebugFixture::new();
    let commands = format!(
        "break {}:8\ncontinue\nlocals\nnext\ncontinue\n",
        fixture.source.display()
    );
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["debug", fixture.source.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(commands.as_bytes())?;
            child.wait_with_output()
        })
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"42\r\n");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("verified breakpoint"), "{stderr}");
    assert!(stderr.contains("/debugger::main"), "{stderr}");
    assert!(stderr.contains("small = 41"), "{stderr}");
    assert!(stderr.contains("wide = 9223372036854775808"), "{stderr}");
    assert!(
        stderr.contains("big = 170141183460469231731687303715884105728"),
        "{stderr}"
    );
    assert!(stderr.contains("text = \"hello\""), "{stderr}");
    assert!(stderr.contains(":9"), "{stderr}");
}

struct DapClient {
    child: Child,
    input: ChildStdin,
    output: BufReader<ChildStdout>,
    sequence: i64,
}

impl DapClient {
    fn start() -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .args(["debug-adapter", "--stdio"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
        let output = BufReader::new(child.stdout.take().unwrap());
        Self {
            child,
            input,
            output,
            sequence: 1,
        }
    }

    fn send(&mut self, command: &str, arguments: Value) -> i64 {
        let sequence = self.sequence;
        self.sequence += 1;
        let mut value = json!({"seq": sequence, "type": "request", "command": command});
        value["arguments"] = arguments;
        let bytes = serde_json::to_vec(&value).unwrap();
        write!(self.input, "Content-Length: {}\r\n\r\n", bytes.len()).unwrap();
        self.input.write_all(&bytes).unwrap();
        self.input.flush().unwrap();
        sequence
    }

    fn read(&mut self) -> Value {
        let mut length = None;
        loop {
            let mut header = String::new();
            assert_ne!(
                self.output.read_line(&mut header).unwrap(),
                0,
                "adapter closed stdout"
            );
            if header == "\r\n" {
                break;
            }
            if let Some(value) = header.trim().strip_prefix("Content-Length:") {
                length = Some(value.trim().parse::<usize>().unwrap());
            }
        }
        let mut bytes = vec![0; length.unwrap()];
        self.output.read_exact(&mut bytes).unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    fn response(&mut self, request: i64) -> Value {
        loop {
            let message = self.read();
            if message["type"] == "response" && message["request_seq"] == request {
                return message;
            }
        }
    }

    fn response_and_event(&mut self, request: i64, event: &str) -> (Value, Value, Vec<Value>) {
        let mut response = None;
        let mut selected = None;
        let mut messages = Vec::new();
        while response.is_none() || selected.is_none() {
            let message = self.read();
            if message["type"] == "response" && message["request_seq"] == request {
                response = Some(message.clone());
            }
            if message["type"] == "event" && message["event"] == event {
                selected = Some(message.clone());
            }
            messages.push(message);
        }
        (response.unwrap(), selected.unwrap(), messages)
    }
}

impl Drop for DapClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn adapter_keeps_debuggee_output_framed_and_maps_stack_frames() {
    let fixture = DebugFixture::new();
    let (executable, provenance) = fixture.build();
    let mut dap = DapClient::start();

    let initialize = dap.send(
        "initialize",
        json!({"adapterID": "terrane-test", "linesStartAt1": true, "columnsStartAt1": true, "pathFormat": "path"}),
    );
    let response = dap.response(initialize);
    assert!(response["success"].as_bool().unwrap());
    assert!(
        !response["body"]["supportsConditionalBreakpoints"]
            .as_bool()
            .unwrap()
    );
    let initialized = dap.read();
    assert_eq!(initialized["type"], "event");
    assert_eq!(initialized["event"], "initialized");

    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance,
            "stopOnEntry": true,
            "disableASLR": false
        }),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    let fidelity = dap.read();
    assert_eq!(fidelity["event"], "terrane/fidelity");
    assert_eq!(fidelity["body"]["mode"], "source");
    assert_eq!(fidelity["body"]["sourceTranslation"], true);

    let set_breakpoints = dap.send(
        "setBreakpoints",
        json!({
            "source": {"path": fixture.source},
            "breakpoints": [{"line": 8}],
            "sourceModified": false
        }),
    );
    let response = dap.response(set_breakpoints);
    assert!(response["success"].as_bool().unwrap());
    assert!(
        response["body"]["breakpoints"][0]["verified"]
            .as_bool()
            .unwrap()
    );

    let configuration = dap.send("configurationDone", json!({}));
    let (_, stopped, _) = dap.response_and_event(configuration, "stopped");
    let thread = stopped["body"]["threadId"].as_i64().unwrap();

    let continue_request = dap.send("continue", json!({"threadId": thread}));
    let (_, stopped, _) = dap.response_and_event(continue_request, "stopped");
    let thread = stopped["body"]["threadId"].as_i64().unwrap();
    let stack = dap.send(
        "stackTrace",
        json!({"threadId": thread, "startFrame": 0, "levels": 1}),
    );
    let stack = dap.response(stack);
    assert_eq!(
        stack["body"]["stackFrames"][0]["source"]["path"],
        fixture.source.to_string_lossy().as_ref()
    );
    assert_eq!(stack["body"]["stackFrames"][0]["line"], 8);
    assert_eq!(stack["body"]["stackFrames"][0]["name"], "/debugger::main");
    let next_request = dap.send("next", json!({"threadId": thread}));
    let (_, stopped, _) = dap.response_and_event(next_request, "stopped");
    let thread = stopped["body"]["threadId"].as_i64().unwrap();
    let stack = dap.send(
        "stackTrace",
        json!({"threadId": thread, "startFrame": 0, "levels": 1}),
    );
    let stack = dap.response(stack);
    assert_eq!(stack["body"]["stackFrames"][0]["line"], 9);

    let continue_request = dap.send("continue", json!({"threadId": thread}));
    let (_, _, messages) = dap.response_and_event(continue_request, "terminated");
    assert!(
        messages
            .iter()
            .all(|message| message["type"] == "response" || message["type"] == "event")
    );
    assert!(messages.iter().any(|message| {
        message["event"] == "output"
            && message["body"]["category"] == "stdout"
            && message["body"]["output"]
                .as_str()
                .is_some_and(|output| output.contains("42"))
    }));
}

#[test]
fn adapter_preserves_raw_native_debugging_for_mismatched_provenance() {
    let fixture = DebugFixture::new();
    let (_, provenance) = fixture.build();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let launch = dap.send(
        "launch",
        json!({"program": "/bin/true", "terraneProvenance": provenance}),
    );
    let response = dap.response(launch);
    assert!(response["success"].as_bool().unwrap());
    let fidelity = dap.read();
    assert_eq!(fidelity["event"], "terrane/fidelity");
    assert_eq!(fidelity["body"]["mode"], "native");
    assert_eq!(fidelity["body"]["sourceTranslation"], false);
    let warning = dap.read();
    assert_eq!(warning["event"], "output");
    let output = warning["body"]["output"].as_str().unwrap();
    assert!(output.contains("raw native debugging remains available"));
    assert!(output.contains('\n'));
    assert!(!output.contains("\\n"));
}

#[test]
fn adapter_disables_source_translation_for_stale_sources() {
    let fixture = DebugFixture::new();
    let (executable, provenance) = fixture.build();
    fs::write(&fixture.source, "namespace changed\nfunction main;\n").unwrap();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let launch = dap.send(
        "launch",
        json!({"program": executable, "terraneProvenance": provenance}),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    let fidelity = dap.read();
    assert_eq!(fidelity["event"], "terrane/fidelity");
    assert_eq!(fidelity["body"]["mode"], "native");
    assert!(
        fidelity["body"]["reason"]
            .as_str()
            .unwrap()
            .contains("source changed since this binary was built")
    );
    let warning = dap.read();
    assert_eq!(warning["event"], "output");
    assert!(
        warning["body"]["output"]
            .as_str()
            .unwrap()
            .contains("source changed since this binary was built")
    );
}

#[test]
fn cli_steps_across_async_function_boundaries_in_source_space() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace async-debugger\n",
            "from /core/output import print\n",
            "async function answer int;\n",
            "  return 42\n",
            "async function main;\n",
            "  value int = await answer;\n",
            "  print; value\n",
        ),
    )
    .unwrap();
    let commands = format!(
        "break {}:6\ncontinue\nframes\nnext\ncontinue\n",
        fixture.source.display()
    );
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["debug", fixture.source.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(commands.as_bytes())?;
            child.wait_with_output()
        })
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"42\r\n");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("/async-debugger::main"), "{stderr}");
    assert!(stderr.contains(":7"), "{stderr}");
}
