use std::io::{BufRead, BufReader, Read, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

use serde_json::{Value, json};

fn send(stdin: &mut ChildStdin, message: &Value) {
    let body = message.to_string();
    write!(stdin, "Content-Length: {}\r\n\r\n{body}", body.len()).unwrap();
    stdin.flush().unwrap();
}

fn receive(stdout: &mut BufReader<ChildStdout>) -> Value {
    let mut content_length = None;
    loop {
        let mut header = String::new();
        stdout.read_line(&mut header).unwrap();
        assert!(!header.is_empty(), "language server closed stdout");
        if header == "\r\n" {
            break;
        }
        if let Some(value) = header.strip_prefix("Content-Length: ") {
            content_length = Some(value.trim().parse::<usize>().unwrap());
        }
    }
    let mut body = vec![0; content_length.unwrap()];
    stdout.read_exact(&mut body).unwrap();
    serde_json::from_slice(&body).unwrap()
}

fn receive_response(stdout: &mut BufReader<ChildStdout>, id: u64) -> Value {
    loop {
        let message = receive(stdout);
        if message.get("id").and_then(Value::as_u64) == Some(id) {
            return message;
        }
    }
}

fn receive_notification(stdout: &mut BufReader<ChildStdout>, method: &str) -> Value {
    loop {
        let message = receive(stdout);
        if message.get("method").and_then(Value::as_str) == Some(method) {
            return message;
        }
    }
}

#[test]
fn serves_semantic_tokens_for_an_open_document() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_terrane-language-server"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {"capabilities": {}}
        }),
    );
    let initialized = receive_response(&mut stdout, 1);
    assert_eq!(initialized["result"]["capabilities"]["textDocumentSync"], 1);
    assert_eq!(
        initialized["result"]["capabilities"]["semanticTokensProvider"]["full"],
        true
    );

    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "method": "initialized", "params": {}}),
    );
    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///tmp/editor.trn",
                    "languageId": "terrane",
                    "version": 1,
                    "text": "function inline string;\n  return >hello\n\nfunction block string;\n  return >>\n    first\n    second\n"
                }
            }
        }),
    );
    let diagnostics = receive_notification(&mut stdout, "textDocument/publishDiagnostics");
    assert_eq!(diagnostics["params"]["version"], 1);
    assert_eq!(
        diagnostics["params"]["diagnostics"][0]["code"],
        json!("S2002")
    );
    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/semanticTokens/full",
            "params": {"textDocument": {"uri": "file:///tmp/editor.trn"}}
        }),
    );
    let tokens = receive_response(&mut stdout, 2);
    let data = tokens["result"]["data"].as_array().unwrap();
    assert!(!data.is_empty());
    assert_eq!(data.len() % 5, 0);
    assert_eq!(tokens["result"]["resultId"], "1");
    assert_eq!(
        data.chunks(5)
            .filter(|token| token[3].as_u64() == Some(3))
            .count(),
        4
    );

    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "id": 3, "method": "shutdown", "params": null}),
    );
    let _ = receive_response(&mut stdout, 3);
    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "method": "exit", "params": null}),
    );
    drop(stdin);
    assert!(child.wait().unwrap().success());
}

#[test]
fn serves_an_open_file_outside_its_package_namespace_roots() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_terrane-language-server"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    let repository = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root");
    let path = repository.join("projects/godot-test/tests/unit/godot-view.trn");
    let uri = format!("file://{}", path.display());
    let text = std::fs::read_to_string(path).expect("Godot unit test source");

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {"capabilities": {}}
        }),
    );
    let _ = receive_response(&mut stdout, 1);
    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "method": "initialized", "params": {}}),
    );
    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": uri,
                    "languageId": "terrane",
                    "version": 1,
                    "text": text
                }
            }
        }),
    );
    let diagnostics = receive_notification(&mut stdout, "textDocument/publishDiagnostics");
    assert_eq!(diagnostics["params"]["version"], 1);
    assert!(
        diagnostics["params"]["diagnostics"]
            .as_array()
            .is_some_and(|items| items.iter().all(|item| item["code"] != "S2053"))
    );

    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "id": 2, "method": "shutdown", "params": null}),
    );
    let _ = receive_response(&mut stdout, 2);
    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "method": "exit", "params": null}),
    );
    drop(stdin);
    assert!(child.wait().unwrap().success());
}

#[expect(
    clippy::too_many_lines,
    reason = "one process-level scenario proves negotiated positions and shared analysis features"
)]
#[test]
fn shared_snapshot_serves_navigation_formatting_and_utf8_positions() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_terrane-language-server"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = BufReader::new(child.stdout.take().unwrap());
    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "capabilities": {
                    "general": {"positionEncodings": ["utf-8"]}
                }
            }
        }),
    );
    let initialized = receive_response(&mut stdout, 1);
    assert_eq!(
        initialized["result"]["capabilities"]["positionEncoding"],
        "utf-8"
    );
    assert_eq!(
        initialized["result"]["capabilities"]["definitionProvider"],
        true
    );
    assert_eq!(
        initialized["result"]["capabilities"]["documentFormattingProvider"],
        true
    );
    assert_eq!(
        initialized["result"]["capabilities"]["implementationProvider"],
        true
    );
    assert_eq!(
        initialized["result"]["capabilities"]["codeActionProvider"],
        true
    );
    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "method": "initialized", "params": {}}),
    );
    let source = "namespace query\n\nfunction answer int;   \n    return 42\n\nasync function main;\n    value int = answer;\n\ninterface worker\n    function work int;\n\nclass machine implements worker\n    function work int;\n        return 1\n\nfunction inspect int;\n    item = instance machine;\n    result int = item.work;\n    return result\n";
    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {
                "textDocument": {
                    "uri": "file:///tmp/navigation.trn",
                    "languageId": "terrane",
                    "version": 7,
                    "text": source
                }
            }
        }),
    );
    let diagnostics = receive_notification(&mut stdout, "textDocument/publishDiagnostics");
    assert_eq!(diagnostics["params"]["diagnostics"], json!([]));

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "textDocument/definition",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "position": {"line": 6, "character": 16}
            }
        }),
    );
    let definition = receive_response(&mut stdout, 2);
    assert_eq!(
        definition["result"]["range"]["start"],
        json!({"line": 2, "character": 9})
    );
    assert_eq!(
        definition["result"]["range"]["end"],
        json!({"line": 2, "character": 15})
    );

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "textDocument/references",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "position": {"line": 6, "character": 16},
                "context": {"includeDeclaration": true}
            }
        }),
    );
    let references = receive_response(&mut stdout, 3);
    assert!(references["result"].as_array().unwrap().len() >= 2);

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 4,
            "method": "textDocument/formatting",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "options": {"tabSize": 4, "insertSpaces": true}
            }
        }),
    );
    let formatting = receive_response(&mut stdout, 4);
    let edits = formatting["result"].as_array().unwrap();
    assert_eq!(edits.len(), 1);
    assert!(!edits[0]["newText"].as_str().unwrap().contains(";   \n"));

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 5,
            "method": "textDocument/documentSymbol",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"}
            }
        }),
    );
    let symbols = receive_response(&mut stdout, 5);
    let names = symbols["result"]
        .as_array()
        .unwrap()
        .iter()
        .map(|symbol| symbol["name"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(names.contains(&"answer"));
    assert!(names.contains(&"main"));
    assert!(!names.contains(&"async"));

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 6,
            "method": "textDocument/implementation",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "position": {"line": 9, "character": 13}
            }
        }),
    );
    let implementations = receive_response(&mut stdout, 6);
    assert_eq!(
        implementations["result"][0]["range"]["start"],
        json!({"line": 12, "character": 13})
    );

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 7,
            "method": "textDocument/codeAction",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "range": {
                    "start": {"line": 0, "character": 0},
                    "end": {"line": 0, "character": 0}
                },
                "context": {"diagnostics": []}
            }
        }),
    );
    let actions = receive_response(&mut stdout, 7);
    assert_eq!(actions["result"][0]["kind"], "source");
    assert_eq!(actions["result"][0]["title"], "Format Terrane document");

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 8,
            "method": "terrane/generatedRust",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "position": {"line": 6, "character": 16}
            }
        }),
    );
    let generated = receive_response(&mut stdout, 8);
    assert!(
        generated["result"]["availability"]["known"]
            .as_array()
            .is_some_and(|locations| !locations.is_empty())
    );

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 9,
            "method": "textDocument/definition",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "position": {"line": 6, "character": 16}
            }
        }),
    );
    let definition_after_generation = receive_response(&mut stdout, 9);
    assert_eq!(
        definition_after_generation["result"]["range"]["start"],
        json!({"line": 2, "character": 9})
    );

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 10,
            "method": "textDocument/rename",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "position": {"line": 9, "character": 13},
                "newName": "execute"
            }
        }),
    );
    let contract_rename = receive_response(&mut stdout, 10);
    let contract_edits = contract_rename["result"]["documentChanges"][0]["edits"]
        .as_array()
        .expect("contract rename edits");
    assert_eq!(contract_edits.len(), 3);

    send(
        &mut stdin,
        &json!({
            "jsonrpc": "2.0",
            "id": 11,
            "method": "textDocument/rename",
            "params": {
                "textDocument": {"uri": "file:///tmp/navigation.trn"},
                "position": {"line": 9, "character": 13},
                "newName": "function"
            }
        }),
    );
    let invalid_rename = receive_response(&mut stdout, 11);
    assert_eq!(invalid_rename["error"]["code"], -32602);
    assert!(
        invalid_rename["error"]["message"]
            .as_str()
            .is_some_and(|message| message.contains("invalid-name"))
    );

    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "id": 12, "method": "shutdown", "params": null}),
    );
    let _ = receive_response(&mut stdout, 12);
    send(
        &mut stdin,
        &json!({"jsonrpc": "2.0", "method": "exit", "params": null}),
    );
    drop(stdin);
    assert!(child.wait().unwrap().success());
}
