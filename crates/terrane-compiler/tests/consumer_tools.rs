use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use terrane_compiler::{Package, compile_package};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "terrane-consumer-tools-{}-{}",
            std::process::id(),
            NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn write(&self, path: &str, text: &str) {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    fn caller(&self) -> Package {
        Package::load(self.0.join("app/package.toml")).unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn source_literal(value: &str) -> String {
    format!(
        "'{}'",
        value
            .replace('\\', "\\\\")
            .replace('\'', "\\'")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    )
}

fn response(identity: &str, source: &str) -> String {
    serde_json::json!({
        "format": 1,
        "generated_sources": [{"identity": identity, "source": source}],
        "diagnostics": [],
    })
    .to_string()
}

fn leaf_source(result_type: &str, value: &str) -> String {
    let generated = format!(
        "namespace generated/result\npublic function answer {result_type};\n    return {value}\n"
    );
    let generated_response = response("answer", &generated);
    let seed = format!(
        "namespace generated/seed\npublic function response string;\n    return {}\n",
        source_literal(&generated_response)
    );
    format!(
        "namespace leaf-tool\nfunction main;\n    print; {}\nmain\n",
        source_literal(&response("seed", &seed))
    )
}

#[test]
fn transitive_tools_refresh_contracts_and_reject_action_cycles() {
    let fixture = Fixture::new();
    fixture.write(
        "provider/package.toml",
        r#"package = "example/provider"
artifact = "library"
[namespaces]
"example/provider" = "src"
[consumer-tools]
generate = "consumer/package.toml"
leaf = "leaf/package.toml"
"#,
    );
    fixture.write(
        "provider/src/main.trn",
        "namespace example/provider\npublic function marker int;\n    return 0\n",
    );
    fixture.write(
        "provider/consumer/package.toml",
        r#"package = "primary-tool"
[namespaces]
primary-tool = "src"
[terrane-dependencies.example.provider]
path = ".."
[consumers.seed]
tool = "example/provider::leaf"
declarations = ["/primary-tool::main"]
"#,
    );
    fixture.write("provider/consumer/src/main.trn", "namespace primary-tool\nfrom /generated/seed import response\nfunction main;\n    print; (response;)\nmain\n");
    let leaf_manifest = r#"package = "leaf-tool"
[namespaces]
leaf-tool = "src"
"#;
    fixture.write("provider/leaf/package.toml", leaf_manifest);
    fixture.write("provider/leaf/src/main.trn", &leaf_source("int", "41"));
    fixture.write(
        "app/package.toml",
        r#"package = "consumer-app"
[namespaces]
consumer-app = "src"
[terrane-dependencies.example.provider]
path = "../provider"
[consumers.generate]
tool = "example/provider::generate"
declarations = ["/consumer-app::main"]
"#,
    );
    fixture.write("app/src/main.trn", "namespace consumer-app\nfrom /generated/result import answer\nfunction require-number; value int\n    print; value\nfunction main;\n    require-number; (answer;)\nmain\n");

    compile_package(&fixture.caller()).expect("nested tools build without bootstrap");

    // No application edit: a leaf tool source change must rebuild both tool actions
    // and replace the generated callable contract seen by the caller.
    fixture.write(
        "provider/leaf/src/main.trn",
        &leaf_source("string", "'changed'"),
    );
    let failure = compile_package(&fixture.caller())
        .expect_err("updated generated type rejects int argument");
    assert!(
        failure
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.as_ref() == "T0012")
    );

    fixture.write("provider/leaf/src/main.trn", &leaf_source("int", "42"));
    compile_package(&fixture.caller()).expect("corrected tool output recovers");

    fixture.write("provider/leaf/package.toml", &format!("{leaf_manifest}\n[terrane-dependencies.example.provider]\npath = \"..\"\n[consumers.cycle]\ntool = \"example/provider::generate\"\ndeclarations = [\"/leaf-tool::main\"]\n"));
    let cycle = compile_package(&fixture.caller()).expect_err("actual tool action cycle rejects");
    assert!(
        cycle
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code.as_ref() == "S2060"
                && diagnostic.message.contains("build-action cycle"))
    );
    assert!(
        fixture
            .0
            .join(Path::new("app/.trn/generated/generate/answer.trn"))
            .is_file()
    );
}

fn editor_fixture() -> (Fixture, &'static str) {
    let fixture = Fixture::new();
    fixture.write(
        "provider/package.toml",
        r#"package = "example/provider"
artifact = "library"
[namespaces]
"example/provider" = "src"
[consumer-tools]
generate = "consumer/package.toml"
"#,
    );
    fixture.write(
        "provider/src/main.trn",
        "namespace example/provider\npublic function marker int;\n    return 0\n",
    );
    fixture.write(
        "protocol/package.toml",
        include_str!("../../../tools/annotation-consumers/protocol/package.toml"),
    );
    fixture.write(
        "protocol/src/main.trn",
        include_str!("../../../tools/annotation-consumers/protocol/src/main.trn"),
    );
    fixture.write(
        "provider/consumer/package.toml",
        r#"package = "editor-tool"
[namespaces]
editor-tool = "src"
[terrane-dependencies.terrane.annotation-consumer-protocol]
path = "../../protocol"
"#,
    );
    fixture.write("provider/consumer/src/main.trn", r"namespace editor-tool
from /terrane/annotation-consumer-protocol import read-request, get, at, text, emit-source
from /core/filesystem import filesystem-capability, filesystem-read-bounded
from /core/filesystem/paths import path
from /core/encodings import utf8
function main;
    request = read-request;
    declaration = at; (get; request, 'declarations'), 0
    authority = filesystem-capability;
    loaded = filesystem-read-bounded; authority, (instance path; 'mode.txt'), 32
    mode = loaded.data.decode; utf8
    if (text; (get; declaration, 'documentation')) == 'integer' and mode == 'integer'
        emit-source; 'answer', 'namespace generated/result\npublic function answer int;\n    return 41\n'
    else
        emit-source; 'answer', 'namespace generated/result\npublic function answer string;\n    return \'changed\'\n'
main
");
    fixture.write(
        "app/package.toml",
        r#"package = "consumer-app"
[namespaces]
consumer-app = "src"
[terrane-dependencies.example.provider]
path = "../provider"
[consumers.generate]
tool = "example/provider::generate"
declarations = ["/consumer-app::main"]
"#,
    );
    let original = "namespace consumer-app\nfrom /generated/result import answer\nfunction require-number; value int\n    print; value\n/// integer\nfunction main;\n    require-number; (answer;)\nmain\n";
    fixture.write("app/src/main.trn", original);
    fixture.write("app/mode.txt", "integer");
    fixture.write(
        "app/.trn/generated/retained/build.trn",
        "retained build output",
    );
    (fixture, original)
}

fn open_editor_snapshot(
    engine: &mut terrane_compiler::tooling::ToolingEngine,
    fixture: &Fixture,
    text: &str,
    trusted: bool,
    generated: bool,
) -> terrane_compiler::tooling::SnapshotMetadata {
    use terrane_compiler::tooling::{SnapshotOptions, SourceInput};
    let manifest_path = fixture.0.join("app/package.toml");
    engine
        .open_snapshot(
            vec![SourceInput {
                uri: format!("file://{}", fixture.0.join("app/src/main.trn").display()),
                text: text.to_owned(),
            }],
            Some(SourceInput {
                uri: format!("file://{}", manifest_path.display()),
                text: fs::read_to_string(&manifest_path).unwrap(),
            }),
            None,
            SnapshotOptions {
                semantic: true,
                allow_consumer_execution: trusted,
                generated,
                ..SnapshotOptions::default()
            },
        )
        .unwrap()
}

#[test]
fn editor_consumers_track_overlays_and_external_inputs_without_build_writes() {
    use terrane_compiler::tooling::{Availability, ToolingEngine};
    let (fixture, original) = editor_fixture();
    let app_path = fixture.0.join("app/src/main.trn");
    let app_uri = format!("file://{}", app_path.display());
    let open = |engine: &mut ToolingEngine, text: &str, trusted, generated| {
        open_editor_snapshot(engine, &fixture, text, trusted, generated)
    };
    let mut engine = ToolingEngine::default();
    let untrusted = open(&mut engine, original, false, false);
    let diagnostics = engine
        .syntax(&untrusted.snapshot_id, &app_uri, None)
        .unwrap()
        .diagnostics;
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "S2060")
    );
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "S2060"
            && diagnostic.span.is_none()
            && diagnostic.message.contains("package.toml")
            && diagnostic.message.contains("generate")
    }));
    assert!(!fixture.0.join("provider/consumer/.trn").exists());

    let accepted = open(&mut engine, original, true, false);
    let diagnostics = engine
        .syntax(&accepted.snapshot_id, &app_uri, None)
        .unwrap()
        .diagnostics;
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    let call = original.rfind("answer").unwrap();
    let Availability::Known(definition) = engine
        .definition(&accepted.snapshot_id, &app_uri, call + 1)
        .unwrap()
    else {
        panic!("generated callable definition resolves");
    };
    let generated_path = Path::new(definition.uri.strip_prefix("file://").unwrap());
    assert!(generated_path.starts_with(fixture.0.join("app/.trn/editor-generated")));
    assert!(generated_path.is_file());
    let Availability::Known(references) = engine
        .references(&accepted.snapshot_id, &app_uri, call + 1)
        .unwrap()
    else {
        panic!("generated callable references resolve");
    };
    assert!(
        references
            .iter()
            .any(|location| location.uri == app_uri && location.span.start == call)
    );

    let overlay = original.replace("/// integer", "/// text");
    let changed = open(&mut engine, &overlay, true, false);
    assert_ne!(changed.snapshot_id, accepted.snapshot_id);
    assert!(
        engine
            .syntax(&changed.snapshot_id, &app_uri, None)
            .unwrap()
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "T0012")
    );
    assert_eq!(fs::read_to_string(&app_path).unwrap(), original);
    assert_eq!(
        engine
            .definition(&accepted.snapshot_id, &app_uri, call + 1)
            .unwrap(),
        Availability::Known(definition)
    );

    fixture.write("app/mode.txt", "text");
    let external = open(&mut engine, original, true, false);
    assert_ne!(external.snapshot_id, accepted.snapshot_id);
    assert!(
        engine
            .syntax(&external.snapshot_id, &app_uri, None)
            .unwrap()
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "T0012")
    );
    fixture.write("app/mode.txt", "integer");
    let generated = open(&mut engine, original, true, true);
    assert!(matches!(generated.build_id, Availability::Known(_)));
    assert_eq!(
        fs::read_to_string(fixture.0.join("app/.trn/generated/retained/build.trn")).unwrap(),
        "retained build output"
    );
    assert!(
        !fixture
            .0
            .join("app/.trn/generated/generate/answer.trn")
            .exists()
    );
}

#[test]
fn editor_reports_consumer_build_errors_at_their_real_origin() {
    use terrane_compiler::tooling::ToolingEngine;
    let (fixture, original) = editor_fixture();
    let app_uri = format!("file://{}", fixture.0.join("app/src/main.trn").display());
    let mut engine = ToolingEngine::default();
    fixture.write(
        "provider/consumer/src/main.trn",
        "namespace editor-tool\nfunction main;\n    unknown-generator;\nmain\n",
    );
    let failed = open_editor_snapshot(&mut engine, &fixture, original, true, false);
    let tool_path = fixture
        .0
        .join("provider/consumer/src/main.trn")
        .display()
        .to_string();
    let diagnostics = engine
        .syntax(&failed.snapshot_id, &app_uri, None)
        .unwrap()
        .diagnostics;
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == "error"
                && diagnostic.message.contains(&tool_path)
                && diagnostic.span.as_ref().is_some_and(|span| span.start == 0)),
        "{diagnostics:?}"
    );
}

fn editor_files(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut files = Vec::new();
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            files.extend(editor_files(&entry.path()));
        } else if entry.file_type().unwrap().is_file() {
            files.push((entry.path(), fs::read(entry.path()).unwrap()));
        }
    }
    files.sort();
    files
}

#[cfg(unix)]
#[test]
fn symlinked_encoded_workspace_keeps_authored_rename_identity() {
    use terrane_compiler::tooling::{SnapshotOptions, SourceInput, ToolingEngine};
    let fixture = Fixture::new();
    let text = "namespace alias-app\nfunction answer int;\n    return 42\nfunction main;\n    print; (answer;)\n";
    fixture.write(
        "real/package.toml",
        "package = 'alias-app'\n[namespaces]\nalias-app = 'src'\n",
    );
    fixture.write("real/src/main.trn", text);
    std::os::unix::fs::symlink(fixture.0.join("real"), fixture.0.join("alias space #")).unwrap();
    let uri = format!(
        "file://{}",
        fixture.0.join("alias space #/src/main.trn").display()
    )
    .replace(' ', "%20")
    .replace('#', "%23");
    let manifest_uri = format!(
        "file://{}",
        fixture.0.join("alias space #/package.toml").display()
    )
    .replace(' ', "%20")
    .replace('#', "%23");
    let mut engine = ToolingEngine::default();
    let snapshot = engine
        .open_snapshot(
            vec![SourceInput {
                uri: uri.clone(),
                text: text.to_owned(),
            }],
            Some(SourceInput {
                uri: manifest_uri,
                text: fs::read_to_string(fixture.0.join("real/package.toml")).unwrap(),
            }),
            None,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        )
        .unwrap();
    assert_eq!(
        snapshot
            .sources
            .iter()
            .filter(|source| source.uri.ends_with("/src/main.trn"))
            .count(),
        1
    );
    let proposal = engine
        .propose_rename(
            &snapshot.snapshot_id,
            &uri,
            text.find("answer").unwrap() + 1,
            "renamed",
        )
        .unwrap();
    assert!(
        proposal.preview_diagnostics.is_empty(),
        "{:?}",
        proposal.preview_diagnostics
    );
    assert_eq!(proposal.replacements.len(), 2);
    assert!(
        proposal
            .replacements
            .iter()
            .all(|replacement| replacement.uri == uri)
    );
}

#[test]
fn editor_collection_preserves_live_engines_and_untracked_files() {
    use terrane_compiler::tooling::{Availability, ToolingEngine};
    let (fixture, original) = editor_fixture();
    let uri = format!("file://{}", fixture.0.join("app/src/main.trn").display());
    let mut first = ToolingEngine::default();
    let mut second = ToolingEngine::default();
    let first_snapshot = open_editor_snapshot(&mut first, &fixture, original, true, false);
    let second_snapshot = open_editor_snapshot(&mut second, &fixture, original, true, false);
    let definition_path = |engine: &ToolingEngine, snapshot: &str| {
        let Availability::Known(definition) = engine
            .definition(snapshot, &uri, original.rfind("answer").unwrap() + 1)
            .unwrap()
        else {
            panic!("generated definition resolves");
        };
        PathBuf::from(definition.uri.strip_prefix("file://").unwrap())
    };
    let first_path = definition_path(&first, &first_snapshot.snapshot_id);
    let second_path = definition_path(&second, &second_snapshot.snapshot_id);
    assert_ne!(first_path, second_path);
    let user_file = first_path.with_file_name("user.txt");
    fs::write(&user_file, "preserve me").unwrap();
    first.close_snapshot(&first_snapshot.snapshot_id).unwrap();
    assert!(!first_path.exists());
    assert_eq!(fs::read_to_string(&user_file).unwrap(), "preserve me");
    assert!(second_path.is_file());
    drop(second);
    assert!(!second_path.exists());
    assert_eq!(fs::read_to_string(&user_file).unwrap(), "preserve me");
}

#[test]
fn editor_previews_validate_generated_types_without_publishing() {
    use terrane_compiler::tooling::ToolingEngine;
    let (fixture, original) = editor_fixture();
    let app_uri = format!("file://{}", fixture.0.join("app/src/main.trn").display());
    let mut engine = ToolingEngine::default();
    let accepted = open_editor_snapshot(&mut engine, &fixture, original, true, false);
    let before_preview = editor_files(&fixture.0.join("app/.trn/editor-generated"));
    let metadata_start = original.find("/// integer").unwrap();
    let preview = engine
        .propose_edits(
            &accepted.snapshot_id,
            vec![terrane_compiler::tooling::Replacement {
                uri: app_uri.clone(),
                span: terrane_compiler::tooling::PublicSpan {
                    start: metadata_start,
                    end: metadata_start + "/// integer".len(),
                },
                text: "/// text".to_owned(),
            }],
        )
        .unwrap();
    assert!(
        preview
            .preview_diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "T0012")
    );
    assert_eq!(
        editor_files(&fixture.0.join("app/.trn/editor-generated")),
        before_preview
    );
}
