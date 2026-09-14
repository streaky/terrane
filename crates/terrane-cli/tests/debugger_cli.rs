#![cfg(target_os = "linux")]

use std::fs;
use std::io::{BufRead as _, BufReader, Read as _, Write as _};
use std::path::{Path, PathBuf};
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
        self.build_expecting(&[], b"42\r\n")
    }

    fn build_with_flags(&self, flags: &[&str]) -> (PathBuf, PathBuf) {
        self.build_expecting(flags, b"42\r\n")
    }

    fn build_expecting(&self, flags: &[&str], expected_stdout: &[u8]) -> (PathBuf, PathBuf) {
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg("debug")
            .args(flags)
            .arg(self.source.to_str().unwrap())
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
        assert_eq!(output.stdout, expected_stdout);
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
        "break {}:8\ncontinue\nframes\nframe 0\nsource 1\ngenerated 1\nlocals\nvalue text\nnext\ncontinue\n",
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
    assert!(
        stderr.contains(&format!(
            "verified breakpoint {}:8 (generated at ",
            fixture.source.display()
        )),
        "{stderr}"
    );
    assert!(
        !stderr.contains("To get started with the debug console"),
        "{stderr}"
    );
    assert!(stderr.contains("/debugger::main"), "{stderr}");
    assert!(stderr.contains("small = 41"), "{stderr}");
    assert!(stderr.contains("#0"), "{stderr}");
    assert!(stderr.contains(">     8 |   small = small + 1"), "{stderr}");
    assert!(stderr.contains("src/main.rs:"), "{stderr}");
    assert!(stderr.contains("wide = 9223372036854775808"), "{stderr}");
    assert!(
        stderr.contains("big = 170141183460469231731687303715884105728"),
        "{stderr}"
    );
    assert!(stderr.contains("text = \"hello\""), "{stderr}");
    assert!(stderr.contains(":9"), "{stderr}");
}

#[test]
fn cli_keeps_debuggee_text_that_resembles_debugger_output_on_stdout() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace output-debugger\n",
            "from /core/output import print\n",
            "function main;\n",
            "  print; 'verified breakpoint fake'\n",
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["debug", fixture.source.to_str().unwrap()])
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
    assert_eq!(output.stdout, b"verified breakpoint fake\r\n");
    assert!(
        !String::from_utf8_lossy(&output.stderr).contains("verified breakpoint fake"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn cli_resolves_relative_sources_from_the_invocation_directory_first() {
    let fixture = DebugFixture::new();
    let workspace = fixture.root.join("workspace");
    let source = workspace.join("nested/case.trn");
    fs::create_dir_all(source.parent().unwrap()).unwrap();
    fs::write(
        &source,
        concat!(
            "namespace relative-debugger\n",
            "from /core/output import print\n",
            "function main;\n",
            "  value = 7\n",
            "  print; value\n",
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["debug", "nested/case.trn"])
        .current_dir(&workspace)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child
                .stdin
                .take()
                .unwrap()
                .write_all(b"break nested/case.trn:4\ncontinue\nquit\n")?;
            child.wait_with_output()
        })
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(&format!("verified breakpoint {}:4", source.display())),
        "{stderr}"
    );
    assert!(stderr.contains("/relative-debugger::main"), "{stderr}");
}

#[test]
fn cli_expands_focused_values_without_exposing_secret_fields() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace debugger\n",
            "from /core/output import print\n",
            "class credentials\n",
            "  username string = 'alice'\n",
            "  token string = 'hidden' metadata (secret = true)\n",
            "function main;\n",
            "  auth = instance credentials;\n",
            "  print; auth.username\n",
        ),
    )
    .unwrap();
    let commands = format!(
        "break {}:8\ncontinue\nvalue auth\nquit\n",
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
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("auth = credentials"), "{stderr}");
    assert!(stderr.contains("username = \"alice\""), "{stderr}");
    assert!(stderr.contains("token = <secret>"), "{stderr}");
    assert!(!stderr.contains("token = \"hidden\""), "{stderr}");
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

fn launch_fidelity(executable: &Path, provenance: &Path) -> Value {
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance,
            "stopOnEntry": true
        }),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    let fidelity = loop {
        let message = dap.read();
        if message["event"] == "terrane/fidelity" {
            break message;
        }
    };
    let disconnect = dap.send("disconnect", json!({"terminateDebuggee": true}));
    assert!(dap.response(disconnect)["success"].as_bool().unwrap());
    fidelity
}

#[test]
fn debug_rejects_release_profile_instead_of_claiming_source_fidelity() {
    let fixture = DebugFixture::new();
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["debug", "--release"])
        .arg(&fixture.source)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(stderr.contains("debug --release"));
    assert!(stderr.contains("compiler-owned unoptimized debug profile"));
}
#[expect(
    clippy::too_many_lines,
    reason = "one protocol transcript verifies ordering, breakpoint lifecycle, frames, values, output, and disconnect"
)]
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
    assert!(response["body"].get("supportsCancelRequest").is_none());
    let initialized = dap.read();
    assert_eq!(initialized["type"], "event");
    assert_eq!(initialized["event"], "initialized");

    let set_breakpoints = dap.send(
        "setBreakpoints",
        json!({
            "source": {"path": fixture.source},
            "breakpoints": [{"line": 8}, {"line": 9}],
            "sourceModified": false

        }),
    );
    let pending = dap.response(set_breakpoints);
    assert!(pending["success"].as_bool().unwrap());
    assert_eq!(pending["body"]["breakpoints"].as_array().unwrap().len(), 2);
    assert!(
        pending["body"]["breakpoints"]
            .as_array()
            .unwrap()
            .iter()
            .all(|breakpoint| {
                !breakpoint["verified"].as_bool().unwrap()
                    && breakpoint["message"].as_str().unwrap().contains("pending")
            })
    );

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
    let mut fidelity = None;
    let mut changed = Vec::new();
    while fidelity.is_none() || changed.len() < 2 {
        let message = dap.read();
        if message["event"] == "terrane/fidelity" {
            fidelity = Some(message);
        } else if message["event"] == "breakpoint" {
            changed.push(message);
        }
    }
    let fidelity = fidelity.unwrap();
    assert_eq!(fidelity["body"]["mode"], "source");
    assert_eq!(fidelity["body"]["sourceTranslation"], true);
    assert!(
        fidelity["body"]["abiRecipe"]
            .as_str()
            .unwrap()
            .starts_with("terrane-rust-x86_64-linux-gnu-v2@sha256:")
    );
    assert_eq!(fidelity["body"]["artifactProfile"], "terrane-debug-v1");
    assert_eq!(
        fidelity["body"]["inlining"],
        "compiler-default-at-opt-level-0"
    );
    assert!(
        fidelity["body"]["rustcRelease"]
            .as_str()
            .is_some_and(|release| release.starts_with("rustc "))
    );
    assert!(changed.iter().all(|message| {
        message["body"]["reason"] == "changed" && message["body"]["breakpoint"]["verified"] == true
    }));
    let message = changed[0]["body"]["breakpoint"]["message"]
        .as_str()
        .unwrap();
    assert!(message.contains(fixture.source.to_string_lossy().as_ref()));
    assert!(message.contains("generated location src/main.rs:"));

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
    let clear_breakpoints = dap.send(
        "setBreakpoints",
        json!({
            "source": {"path": fixture.source},
            "breakpoints": [],
            "sourceModified": false
        }),
    );
    let response = dap.response(clear_breakpoints);
    assert!(response["success"].as_bool().unwrap());
    assert!(
        response["body"]["breakpoints"]
            .as_array()
            .unwrap()
            .is_empty()
    );

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
fn adapter_accepts_source_breakpoints_after_launch_before_configuration_done() {
    let fixture = DebugFixture::new();
    let (executable, provenance) = fixture.build();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");

    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance,
            "stopOnEntry": true
        }),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    let breakpoint = dap.send(
        "setBreakpoints",
        json!({
            "source": {"path": fixture.source},
            "breakpoints": [{"line": 8}],
            "sourceModified": false
        }),
    );
    let breakpoint = dap.response(breakpoint);
    assert!(breakpoint["success"].as_bool().unwrap());
    assert_eq!(breakpoint["body"]["breakpoints"][0]["verified"], true);

    let configuration = dap.send("configurationDone", json!({}));
    let (_, entry, _) = dap.response_and_event(configuration, "stopped");
    let thread = entry["body"]["threadId"].as_i64().unwrap();
    let continue_request = dap.send("continue", json!({"threadId": thread}));
    let (_, stopped, _) = dap.response_and_event(continue_request, "stopped");
    let stack = dap.send(
        "stackTrace",
        json!({"threadId": stopped["body"]["threadId"], "startFrame": 0, "levels": 1}),
    );
    let stack = dap.response(stack);
    assert_eq!(stack["body"]["stackFrames"][0]["line"], 8);

    let disconnect = dap.send("disconnect", json!({"terminateDebuggee": true}));
    assert!(dap.response(disconnect)["success"].as_bool().unwrap());
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
    let fidelity = loop {
        let message = dap.read();
        if message["event"] == "terrane/fidelity" {
            break message;
        }
    };
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
    let fidelity = loop {
        let message = dap.read();
        if message["event"] == "terrane/fidelity" {
            break message;
        }
    };
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
    assert_eq!(
        output.stdout,
        b"42\r\n",
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("/async-debugger::main"), "{stderr}");
    assert!(stderr.contains("#0"), "{stderr}");
    assert!(stderr.contains(":7"), "{stderr}");
}

#[test]
fn cli_next_returns_from_a_helpers_final_sequence_point() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace debugger\n",
            "function answer int;\n",
            "  return 42\n",
            "from /core/output import print\n",
            "function main;\n",
            "  value int = answer;\n",
            "  print; value\n",
        ),
    )
    .unwrap();
    let commands = format!(
        "break {}:3\ncontinue\nframes\nnext\nframes\ncontinue\n",
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
    assert!(stderr.contains("/debugger::answer"), "{stderr}");
    assert!(stderr.contains("/debugger::main"), "{stderr}");
    assert!(stderr.contains(":6"), "{stderr}");
}

#[test]
fn cli_preserves_breakpoints_and_reenters_loop_sequence_points() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace loop\n",
            "from /core/output import print\n",
            "function helper int;\n",
            "  x = 1\n",
            "  return x + 1\n",
            "function main;\n",
            "  total = 0\n",
            "  i = 0\n",
            "  while i < 3\n",
            "    total = total + i\n",
            "    i = i + 1\n",
            "  y = helper;\n",
            "  print; total\n",
            "  print; y\n",
        ),
    )
    .unwrap();
    let commands = format!(
        "break {}:10\nbreak {}:13\ncontinue\nframes\nnext\nframes\nlldb breakpoint list -b\nnext\nframes\nnext\nframes\ndisable 1\ncontinue\nframes\nbreakpoints\ndelete all\ncontinue\n",
        fixture.source.display(),
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
    assert_eq!(
        output.stdout,
        b"3\r\n2\r\n",
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.matches(":10").count() >= 2, "{stderr}");
    assert!(stderr.contains(":11"), "{stderr}");
    assert!(stderr.contains(":9"), "{stderr}");
    assert!(stderr.contains(":13"), "{stderr}");
    assert!(stderr.contains("#1   disabled breakpoint"), "{stderr}");
    assert!(stderr.contains("#2   verified breakpoint"), "{stderr}");
    let native_listing = stderr
        .split("Current breakpoints:")
        .nth(1)
        .and_then(|listing| listing.split("(terrane-debug)").next())
        .expect("LLDB breakpoint list follows source stepping");
    assert!(native_listing.contains("\n1:"), "{native_listing}");
    assert!(native_listing.contains("\n2:"), "{native_listing}");
    assert!(!native_listing.contains("\n3:"), "{native_listing}");
}

#[test]
fn cli_step_out_returns_to_the_exact_caller() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace depth\n",
            "from /core/output import print\n",
            "function inner int;\n",
            "  return 2\n",
            "function outer int;\n",
            "  value = inner;\n",
            "  return value\n",
            "function main;\n",
            "  before = 1\n",
            "  result = outer;\n",
            "  print; before + result\n",
        ),
    )
    .unwrap();
    let commands = format!(
        "break {}:6\ncontinue\nout\nframes\ncontinue\n",
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
    assert_eq!(output.stdout, b"3\r\n");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("/depth::outer"), "{stderr}");
    assert!(stderr.contains("/depth::main"), "{stderr}");
    assert!(stderr.contains(":11"), "{stderr}");
    assert!(!stderr.contains("/depth::inner at"), "{stderr}");
}

#[test]
fn adapter_surfaces_backend_launch_failures() {
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");

    let launch = dap.send(
        "launch",
        json!({
            "program": "/definitely/missing/terrane-program",
            "stopOnEntry": true
        }),
    );
    let response = dap.response(launch);
    assert_eq!(response["success"], false);
    assert!(
        response["message"]
            .as_str()
            .is_some_and(|message| !message.is_empty())
    );
}

#[test]
fn adapter_reports_the_delayed_launch_failure_after_configuration_done() {
    let fixture = DebugFixture::new();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");

    let launch = dap.send(
        "launch",
        json!({
            "program": fixture.source,
            "stopOnEntry": true
        }),
    );
    let launch = dap.response(launch);
    assert!(launch["success"].as_bool().unwrap(), "{launch}");
    let configuration = dap.send("configurationDone", json!({}));
    let configuration = dap.response(configuration);
    assert_eq!(configuration["success"], false);
    let message = configuration["message"].as_str().unwrap();
    assert!(
        message.contains(fixture.source.to_string_lossy().as_ref()),
        "{message}"
    );
    assert_eq!(
        message.matches("<debugger>: error[S5001]:").count(),
        1,
        "{message}"
    );
}

#[test]
fn cli_returns_the_debuggee_exit_status() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace exit-debugger\n",
            "from /core/process import exit, make-exit-status\n",
            "function main;\n",
            "  exit; (make-exit-status; 7)\n",
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["debug", fixture.source.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().unwrap().write_all(b"continue\n")?;
            child.wait_with_output()
        })
        .unwrap();

    assert_eq!(
        output.status.code(),
        Some(7),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn adapter_renders_explicitly_embedded_generated_source_when_build_tree_is_unavailable() {
    let fixture = DebugFixture::new();
    let (executable, provenance) = fixture.build_with_flags(&["--embed-generated-sources"]);
    let generated = provenance.parent().unwrap().join("src/main.rs");
    let expected = fs::read_to_string(&generated).unwrap();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance,
            "stopOnEntry": true
        }),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    loop {
        if dap.read()["event"] == "terrane/fidelity" {
            break;
        }
    }
    fs::remove_file(&generated).unwrap();
    let request = dap.send("terrane/generatedSource", json!({"path": "src/main.rs"}));
    let response = dap.response(request);
    assert_eq!(response["body"]["content"], expected);
    let disconnect = dap.send("disconnect", json!({"terminateDebuggee": true}));
    assert!(dap.response(disconnect)["success"].as_bool().unwrap());
}

#[test]
fn debug_build_embeds_sources_only_when_requested() {
    let fixture = DebugFixture::new();
    let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .args(["debug", "--embed-sources", fixture.source.to_str().unwrap()])
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
    let provenance = fs::read_dir(fixture.root.join(".trn/build"))
        .unwrap()
        .map(|entry| entry.unwrap().path().join("terrane-debug.json"))
        .find(|path| path.is_file())
        .unwrap();
    let provenance: Value = serde_json::from_slice(&fs::read(provenance).unwrap()).unwrap();
    assert_eq!(
        provenance["debug"]["sources"][0]["embedded_source"],
        fs::read_to_string(&fixture.source).unwrap()
    );
}

#[test]
fn adapter_reports_unsupported_debug_profiles_as_native_fidelity() {
    let fixture = DebugFixture::new();
    let (executable, provenance_path) = fixture.build();
    let mut provenance: Value =
        serde_json::from_slice(&fs::read(&provenance_path).unwrap()).unwrap();
    provenance["optimization"] = "3".into();
    fs::write(&provenance_path, serde_json::to_vec(&provenance).unwrap()).unwrap();

    let fidelity = launch_fidelity(&executable, &provenance_path);
    assert_eq!(fidelity["body"]["mode"], "native");
    assert!(
        fidelity["body"]["reason"]
            .as_str()
            .unwrap()
            .contains("unsupported debug artifact profile")
    );
}

#[test]
fn adapter_rejects_malformed_schema_hash_and_abi_provenance() {
    let fixture = DebugFixture::new();
    let (executable, provenance_path) = fixture.build();
    let original: Value = serde_json::from_slice(&fs::read(&provenance_path).unwrap()).unwrap();

    let malformed = fixture.root.join("malformed.json");
    fs::write(&malformed, b"{not-json").unwrap();
    let fidelity = launch_fidelity(&executable, &malformed);
    assert!(
        fidelity["body"]["reason"]
            .as_str()
            .unwrap()
            .contains("invalid provenance sidecar")
    );

    let cases = [
        ("schema", "unsupported debug provenance schema"),
        ("generated-hash", "generated source identity mismatch"),
        (
            "executable-hash",
            "debug provenance does not match executable",
        ),
        ("abi", "unsupported debug target, toolchain, or ABI recipe"),
    ];
    for (case, expected) in cases {
        let mut provenance = original.clone();
        match case {
            "schema" => provenance["schema_version"] = "999".into(),
            "generated-hash" => {
                provenance["debug"]["generated_files"][0]["content_hash"] = "sha256:nope".into();
            }
            "executable-hash" => {
                provenance["native_module"]["content_hash"] = "sha256:nope".into();
            }
            "abi" => provenance["abi_recipe"] = "terrane-test-wrong-abi".into(),
            _ => unreachable!(),
        }
        let path = fixture.root.join(format!("{case}.json"));
        fs::write(&path, serde_json::to_vec(&provenance).unwrap()).unwrap();
        let fidelity = launch_fidelity(&executable, &path);
        assert_eq!(fidelity["body"]["mode"], "native");
        assert!(
            fidelity["body"]["reason"]
                .as_str()
                .unwrap()
                .contains(expected),
            "{case}: {fidelity}"
        );
    }
}

#[test]
fn cli_next_ignores_temporary_breakpoint_hits_in_recursive_deeper_frames() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace debugger\n",
            "from /core/output import print\n",
            "function descend int; depth int\n",
            "  if depth == 0\n",
            "    return 0\n",
            "  next-depth = depth - 1\n",
            "  child = descend; next-depth\n",
            "  return child + 1\n",
            "function main;\n",
            "  result = descend; 2\n",
            "  print; result\n",
        ),
    )
    .unwrap();
    let commands = format!(
        "break {}:7\ncontinue\ndisable 1\nnext\nsource 0\nframes\nquit\n",
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
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains(">     8 |   return child + 1"), "{stderr}");
    assert!(stderr.contains("#0   /debugger::descend"), "{stderr}");
    assert!(stderr.contains("#1   /debugger::main"), "{stderr}");
    assert!(!stderr.contains("#1   /debugger::descend"), "{stderr}");
}

#[test]
fn debugger_preserves_breakpoints_beyond_five_hundred_sequence_points() {
    let fixture = DebugFixture::new();
    let mut source =
        "namespace debugger\nfrom /core/output import print\nfunction main;\n  value = 0\n"
            .to_owned();
    for _ in 0..620 {
        source.push_str("  value = value + 1\n");
    }
    source.push_str("  print; value\n");
    fs::write(&fixture.source, source).unwrap();
    let (_, provenance_path) = fixture.build_expecting(&[], b"620\r\n");
    let provenance: Value = serde_json::from_slice(&fs::read(&provenance_path).unwrap()).unwrap();
    let sequence_points = provenance["debug"]["generated_files"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|file| file["associations"].as_array().unwrap())
        .filter(|association| association["sequence_point"] == true)
        .count();
    assert!(sequence_points > 500, "{sequence_points}");

    let target_line = 550;
    let following_line = target_line + 1;
    let commands = format!(
        "break {}:{target_line}\ncontinue\nnext\nsource 0\nquit\n",
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
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains(&format!(":{following_line}")), "{stderr}");
    assert!(
        stderr.contains(&format!(">   {following_line} |   value = value + 1")),
        "{stderr}"
    );
}

#[test]
fn cli_locals_follow_the_innermost_shadowed_binding() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace debugger\n",
            "from /core/output import print\n",
            "function main;\n",
            "  value int = 1\n",
            "  if value == 1\n",
            "    value int = 2\n",
            "    print; value\n",
            "  print; value\n",
        ),
    )
    .unwrap();
    let commands = format!(
        "break {}:7\nbreak {}:8\ncontinue\nlocals\ndisable 1\ncontinue\nlocals\nquit\n",
        fixture.source.display(),
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
    let stderr = String::from_utf8(output.stderr).unwrap();
    let inner = stderr
        .find("value = 2")
        .unwrap_or_else(|| panic!("inner binding is not visible:\n{stderr}"));
    let outer = stderr[inner + 1..]
        .find("value = 1")
        .unwrap_or_else(|| panic!("outer binding is not restored:\n{stderr}"));
    assert!(outer > 0);
}

#[test]
fn debugger_fixture_removes_its_temporary_build_tree_on_drop() {
    let root = {
        let fixture = DebugFixture::new();
        assert!(fixture.root.exists());

        fixture.root.clone()
    };
    assert!(!root.exists(), "fixture leaked {}", root.display());
}
#[test]
fn adapter_steps_in_and_out_through_standard_dap_requests() {
    let fixture = DebugFixture::new();
    fs::write(
        &fixture.source,
        concat!(
            "namespace debugger\n",
            "from /core/output import print\n",
            "function answer int;\n",
            "  value = 41\n",
            "  return value + 1\n",
            "function main;\n",
            "  result = answer;\n",
            "  print; result\n",
        ),
    )
    .unwrap();
    let (executable, provenance) = fixture.build();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let breakpoint = dap.send(
        "setBreakpoints",
        json!({
            "source": {"path": fixture.source},
            "breakpoints": [{"line": 7}],
            "sourceModified": false
        }),
    );
    assert!(dap.response(breakpoint)["success"].as_bool().unwrap());
    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance,
            "stopOnEntry": true
        }),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    let configuration = dap.send("configurationDone", json!({}));
    let (_, entry, _) = dap.response_and_event(configuration, "stopped");
    let thread = entry["body"]["threadId"].as_i64().unwrap();
    let continue_request = dap.send("continue", json!({"threadId": thread}));
    let (_, call_stop, _) = dap.response_and_event(continue_request, "stopped");
    let thread = call_stop["body"]["threadId"].as_i64().unwrap();

    let step_in = dap.send("stepIn", json!({"threadId": thread}));
    let (_, helper_stop, _) = dap.response_and_event(step_in, "stopped");
    assert_eq!(helper_stop["body"]["reason"], "step");
    let stack = dap.send(
        "stackTrace",
        json!({"threadId": thread, "startFrame": 0, "levels": 2}),
    );
    let stack = dap.response(stack);
    assert_eq!(stack["body"]["stackFrames"][0]["name"], "/debugger::answer");
    assert!(stack["body"]["stackFrames"][0]["line"].as_u64().unwrap() >= 4);

    let helper_breakpoint = dap.send(
        "setBreakpoints",
        json!({
            "source": {"path": fixture.source},
            "breakpoints": [{"line": 4}],
            "sourceModified": false
        }),
    );
    assert!(
        dap.response(helper_breakpoint)["success"]
            .as_bool()
            .unwrap()
    );

    let step_out = dap.send("stepOut", json!({"threadId": thread}));
    let (_, caller_stop, _) = dap.response_and_event(step_out, "stopped");
    assert_eq!(caller_stop["body"]["reason"], "step");
    let stack = dap.send(
        "stackTrace",
        json!({"threadId": thread, "startFrame": 0, "levels": 1}),
    );
    let stack = dap.response(stack);
    assert_eq!(stack["body"]["stackFrames"][0]["name"], "/debugger::main");
    assert!(stack["body"]["stackFrames"][0]["line"].as_u64().unwrap() >= 7);
    let disconnect = dap.send("disconnect", json!({"terminateDebuggee": true}));
    assert!(dap.response(disconnect)["success"].as_bool().unwrap());
}

#[test]
fn adapter_serializes_disconnect_queued_during_active_source_step() {
    let fixture = DebugFixture::new();
    let (executable, provenance) = fixture.build();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance,
            "stopOnEntry": true
        }),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    let configuration = dap.send("configurationDone", json!({}));
    let (_, stopped, _) = dap.response_and_event(configuration, "stopped");
    let thread = stopped["body"]["threadId"].as_i64().unwrap();

    let step = dap.send("next", json!({"threadId": thread}));
    let disconnect = dap.send("disconnect", json!({"terminateDebuggee": true}));
    let mut step_response = None;
    let mut disconnect_response = None;
    while step_response.is_none() || disconnect_response.is_none() {
        let message = dap.read();
        if message["type"] != "response" {
            continue;
        }
        if message["request_seq"] == step {
            step_response = Some(message);
        } else if message["request_seq"] == disconnect {
            disconnect_response = Some(message);
        }
    }
    assert!(step_response.unwrap()["success"].as_bool().unwrap());
    assert!(disconnect_response.unwrap()["success"].as_bool().unwrap());
}

#[test]
fn adapter_preserves_fatal_native_stop_during_source_translation() {
    let fixture = DebugFixture::new();
    let (executable, provenance) = fixture.build();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance,
            "stopOnEntry": true
        }),
    );
    let (launch_response, _, _) = dap.response_and_event(launch, "terrane/fidelity");
    assert!(launch_response["success"].as_bool().unwrap());
    let configuration = dap.send("configurationDone", json!({}));
    let (_, entry, messages) = dap.response_and_event(configuration, "stopped");
    let process_id = messages
        .iter()
        .find(|message| message["event"] == "process")
        .and_then(|message| message["body"]["systemProcessId"].as_u64())
        .expect("lldb-dap reports the launched process id");
    let thread = entry["body"]["threadId"].as_i64().unwrap();
    let continue_request = dap.send("continue", json!({"threadId": thread}));
    let signal = Command::new("kill")
        .args(["-SEGV", &process_id.to_string()])
        .status()
        .unwrap();
    assert!(signal.success());
    let (response, stopped, _) = dap.response_and_event(continue_request, "stopped");
    assert!(response["success"].as_bool().unwrap());
    assert_ne!(stopped["body"]["reason"], "step");
    assert!(
        matches!(
            stopped["body"]["reason"].as_str(),
            Some("exception" | "signal" | "pause")
        ),
        "{stopped}"
    );
    let disconnect = dap.send("disconnect", json!({"terminateDebuggee": true}));
    assert!(dap.response(disconnect)["success"].as_bool().unwrap());
}

#[test]
fn adapter_debugs_relocated_exact_build_artifacts_and_sources() {
    let fixture = DebugFixture::new();
    let (executable, provenance) = fixture.build();
    let relocated_root = fixture.root.with_extension("relocated");
    let executable_relative = executable.strip_prefix(&fixture.root).unwrap().to_owned();
    let provenance_relative = provenance.strip_prefix(&fixture.root).unwrap().to_owned();
    let source_relative = fixture
        .source
        .strip_prefix(&fixture.root)
        .unwrap()
        .to_owned();
    fs::rename(&fixture.root, &relocated_root).unwrap();
    let executable = relocated_root.join(executable_relative);
    let provenance = relocated_root.join(provenance_relative);
    let source = relocated_root.join(source_relative);
    let build_root = provenance.parent().unwrap();

    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance,
            "terraneRelocation": {
                "sourceRoot": relocated_root,
                "buildRoot": build_root
            },
            "stopOnEntry": true
        }),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    let fidelity = loop {
        let message = dap.read();
        if message["event"] == "terrane/fidelity" {
            break message;
        }
    };
    assert_eq!(fidelity["body"]["mode"], "source");
    let breakpoint = dap.send(
        "setBreakpoints",
        json!({
            "source": {"path": source},
            "breakpoints": [{"line": 8}],
            "sourceModified": false
        }),
    );
    let breakpoint = dap.response(breakpoint);
    assert!(
        breakpoint["body"]["breakpoints"][0]["verified"]
            .as_bool()
            .unwrap()
    );
    let configuration = dap.send("configurationDone", json!({}));
    let (_, entry, _) = dap.response_and_event(configuration, "stopped");
    let thread = entry["body"]["threadId"].as_i64().unwrap();
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
        source.to_string_lossy().as_ref()
    );
    assert_eq!(stack["body"]["stackFrames"][0]["line"], 8);
    let disconnect = dap.send("disconnect", json!({"terminateDebuggee": true}));
    assert!(dap.response(disconnect)["success"].as_bool().unwrap());
    fs::remove_dir_all(relocated_root).unwrap();
}
#[test]
fn adapter_renders_split_generated_support_from_exact_build() {
    let fixture = DebugFixture::new();
    let (executable, provenance_path) = fixture.build();
    let provenance: Value = serde_json::from_slice(&fs::read(&provenance_path).unwrap()).unwrap();
    let support = provenance["debug"]["generated_files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|file| file["path"] != "src/main.rs")
        .expect("debug build contains a split generated support file");
    let support_path = support["path"].as_str().unwrap();
    let expected =
        fs::read_to_string(provenance_path.parent().unwrap().join(support_path)).unwrap();
    assert_eq!(
        support["content_hash"],
        terrane_compiler::debugging::hash_bytes(expected.as_bytes())
    );

    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let launch = dap.send(
        "launch",
        json!({
            "program": executable,
            "terraneProvenance": provenance_path,
            "stopOnEntry": true
        }),
    );
    assert!(dap.response(launch)["success"].as_bool().unwrap());
    let generated = dap.send("terrane/generatedSource", json!({"path": support_path}));
    let generated = dap.response(generated);
    assert_eq!(generated["body"]["content"], expected);
    let disconnect = dap.send("disconnect", json!({"terminateDebuggee": true}));
    assert!(dap.response(disconnect)["success"].as_bool().unwrap());
}

#[test]
fn adapter_reports_attach_host_policy_and_honors_disconnect_policy() {
    let mut target = Command::new("sleep").arg("30").spawn().unwrap();
    let mut dap = DapClient::start();
    let initialize = dap.send("initialize", json!({"adapterID": "terrane-test"}));
    assert!(dap.response(initialize)["success"].as_bool().unwrap());
    assert_eq!(dap.read()["event"], "initialized");
    let attach = dap.send(
        "attach",
        json!({
            "program": "/usr/bin/sleep",
            "pid": target.id()
        }),
    );
    let mut response = dap.response(attach);
    if response["success"].as_bool().unwrap() {
        let configuration = dap.send("configurationDone", json!({}));
        response = dap.response(configuration);
        if response["success"].as_bool().unwrap() {
            let disconnect = dap.send("disconnect", json!({"terminateDebuggee": false}));
            assert!(dap.response(disconnect)["success"].as_bool().unwrap());
        }
    }
    if !response["success"].as_bool().unwrap() {
        let message = response["message"].as_str().unwrap_or_default();
        assert!(
            message.contains("attach")
                || message.contains("operation")
                || message.contains("permission")
                || message.contains("process"),
            "{response}"
        );
    }
    target.kill().ok();
    target.wait().ok();
}
