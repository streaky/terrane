use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Package(PathBuf);
impl Package {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "terrane-compilation-cache-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("package.toml"),
            "package = 'cache-test'\nprelude = false\n[namespaces]\napp = 'src'\n",
        )
        .unwrap();
        Self(root)
    }
    fn build(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg("build")
            .arg(self.0.join("package.toml"))
            .output()
            .unwrap()
    }
    fn output(&self) -> String {
        let built = self.build();
        assert!(
            built.status.success(),
            "{}",
            String::from_utf8_lossy(&built.stderr)
        );
        let artifact = String::from_utf8(built.stdout).unwrap();
        let run = Command::new(artifact.trim()).output().unwrap();
        assert!(
            run.status.success(),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        String::from_utf8(run.stdout).unwrap()
    }
    fn source(&self, text: &str) {
        fs::write(self.0.join("src/main.trn"), text).unwrap();
    }
    fn cache_stamp(&self, name: &str) -> std::time::SystemTime {
        fs::metadata(self.0.join(format!(".trn/cache/{name}.json")))
            .unwrap()
            .modified()
            .unwrap()
    }
    fn mark_cache(&self, name: &str) -> std::time::SystemTime {
        fs::OpenOptions::new()
            .write(true)
            .open(self.0.join(format!(".trn/cache/{name}.json")))
            .unwrap()
            .set_modified(std::time::UNIX_EPOCH)
            .unwrap();
        self.cache_stamp(name)
    }
}
impl Drop for Package {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn warm_compilation_tracks_content_and_source_membership_and_recovers_corruption() {
    let package = Package::new();
    package
        .source("namespace app\nfrom /core/output import print\nfunction main;\n  print; 'one'\n");
    assert_eq!(package.output(), "one\n");
    assert_eq!(package.output(), "one\n");
    let compilation_stamp = package.mark_cache("compilation");
    let warm = package.build();
    assert!(
        warm.status.success(),
        "{}",
        String::from_utf8_lossy(&warm.stderr)
    );
    assert_eq!(package.cache_stamp("compilation"), compilation_stamp);
    // Same-size content edits must not depend on timestamps or file sizes.
    package
        .source("namespace app\nfrom /core/output import print\nfunction main;\n  print; 'two'\n");
    assert_eq!(package.output(), "two\n");
    fs::write(
        package.0.join("src/extra.trn"),
        "namespace app\nfunction extra;\n  return missing-binding\n",
    )
    .unwrap();
    let rejected = package.build();
    assert!(!rejected.status.success());
    assert_eq!(rejected.stdout, b"");
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("missing-binding"));
    fs::remove_file(package.0.join("src/extra.trn")).unwrap();
    assert_eq!(package.output(), "two\n");
    let cache = package.0.join(".trn/cache/compilation.json");
    assert!(cache.is_file());
    fs::write(&cache, "{ invalid cache").unwrap();
    assert_eq!(package.output(), "two\n");
    let mut entry: serde_json::Value = serde_json::from_slice(&fs::read(&cache).unwrap()).unwrap();
    entry["compiler"] = serde_json::Value::String("different-compiler".to_owned());
    fs::write(&cache, serde_json::to_vec(&entry).unwrap()).unwrap();
    assert_eq!(package.output(), "two\n");
}

#[test]
fn changed_imported_contract_rechecks_unchanged_caller() {
    let package = Package::new();
    fs::create_dir_all(package.0.join("helper")).unwrap();
    fs::write(
        package.0.join("package.toml"),
        "package = 'cache-test'\nprelude = false\n[namespaces]\napp = 'src'\nhelper = 'helper'\n",
    )
    .unwrap();
    package.source("namespace app\nfrom /core/output import print\nfrom /helper import echo\nfunction main;\n  print; (echo; 'one')\n");
    let helper = package.0.join("helper/echo.trn");
    fs::write(
        &helper,
        "namespace helper\npublic function echo string; value string\n  return value\n",
    )
    .unwrap();
    assert_eq!(package.output(), "one\n");
    assert_eq!(package.output(), "one\n");
    fs::write(
        &helper,
        "namespace helper\npublic function echo string; value int\n  return 'changed'\n",
    )
    .unwrap();
    let rejected = package.build();
    assert!(!rejected.status.success());
    assert_eq!(rejected.stdout, b"");
    let diagnostic = String::from_utf8_lossy(&rejected.stderr);
    assert!(
        diagnostic.contains("string") && diagnostic.contains("int"),
        "{diagnostic}"
    );
    fs::write(
        &helper,
        "namespace helper\npublic function echo string; value string\n  return 'fixed'\n",
    )
    .unwrap();
    assert_eq!(package.output(), "fixed\n");
}

fn copy_fixture(source: &std::path::Path, destination: &std::path::Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        if matches!(entry.file_name().to_str(), Some(".trn" | "target")) {
            continue;
        }
        let target = destination.join(entry.file_name());
        if entry.path().is_dir() {
            copy_fixture(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

#[test]
fn native_generic_contracts_survive_projection_cache_reloads() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let package = Package(workspace.join("target").join(format!(
        "native-generic-cache-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )));
    copy_fixture(
        &workspace.join("tests/conformance/run/native-generic-executor"),
        &package.0,
    );
    let accepted = fs::read_to_string(package.0.join("src/main.trn")).unwrap();
    let expected_output = fs::read_to_string(package.0.join("stdout.txt")).unwrap();
    let implicit_reference = r"namespace app
from /deps/witness import Connection, query
function implicit int64; connection ref Connection
    bound = query; 'implicit'
    return bound.fetch; connection
function main;
    connection = Connection::new;
    print; (implicit; connection)
";
    let missing_interface = r"namespace app
from /deps/witness import advance
class unrelated
    value int = 0
function main;
    value = instance unrelated;
    print; (advance; value)
";
    let wrong_owner_executor = r"namespace app
from /deps/witness import owner_query
function main;
    invalid = 'not an executor'
    print; ((owner_query; 'wrong-owner').fetch; invalid)
";
    for (source, expected_error, expected_output) in [
        (accepted.as_str(), None, expected_output.as_str()),
        (implicit_reference, None, "1\n"),
        (missing_interface, Some("T0121"), ""),
        (wrong_owner_executor, Some("T0119"), ""),
    ] {
        package.source(source);
        let projection = package.0.join(".trn/dependencies/projection.json");
        if projection.exists() {
            fs::remove_file(&projection).unwrap();
        }
        let mut cold_diagnostics = None;
        for _ in 0..2 {
            // Force semantic reanalysis while retaining the projection on the second pass.
            let snapshots = package.0.join(".trn/cache");
            if snapshots.exists() {
                fs::remove_dir_all(snapshots).unwrap();
            }
            let checked = Command::new(env!("CARGO_BIN_EXE_terrane"))
                .arg("check")
                .arg(&package.0)
                .output()
                .unwrap();
            let stderr = String::from_utf8(checked.stderr).unwrap();
            let diagnostics = stderr
                .lines()
                .filter(|line| line.contains("error["))
                .map(str::to_owned)
                .collect::<Vec<_>>();
            if let Some(code) = expected_error {
                assert!(!checked.status.success(), "{stderr}");
                assert!(stderr.contains(&format!("error[{code}]")), "{stderr}");
                if code == "T0119" {
                    assert!(stderr.contains("does not satisfy"), "{stderr}");
                }
                assert!(!stderr.contains("S9003"), "{stderr}");
            } else {
                assert!(checked.status.success(), "{stderr}");
            }
            assert!(projection.is_file(), "the warm pass must load a projection");
            if let Some(cold) = &cold_diagnostics {
                assert_eq!(&diagnostics, cold, "{stderr}");
            } else {
                cold_diagnostics = Some(diagnostics);
            }
        }
        if expected_error.is_none() {
            assert_eq!(package.output(), expected_output);
        }
    }
}

#[test]
fn native_receiver_predicates_survive_projection_cache_reloads() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let package = Package(workspace.join("target").join(format!(
        "native-receiver-cache-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )));
    copy_fixture(
        &workspace.join("tests/conformance/run/projected-concrete-trait-operations"),
        &package.0,
    );
    let accepted = fs::read_to_string(package.0.join("src/main.trn")).unwrap();
    let expected_output = fs::read_to_string(package.0.join("stdout.txt")).unwrap();
    let rejected =
        fs::read_to_string(workspace.join(
            "tests/conformance/reject/projected-receiver-predicate-owned-argument/src/main.trn",
        ))
        .unwrap();
    for (source, rejects) in [(accepted.as_str(), false), (rejected.as_str(), true)] {
        package.source(source);
        let projection = package.0.join(".trn/dependencies/projection.json");
        if projection.exists() {
            fs::remove_file(projection).unwrap();
        }
        for _ in 0..2 {
            let snapshots = package.0.join(".trn/cache");
            if snapshots.exists() {
                fs::remove_dir_all(snapshots).unwrap();
            }
            let checked = Command::new(env!("CARGO_BIN_EXE_terrane"))
                .arg("check")
                .arg(&package.0)
                .output()
                .unwrap();
            let stderr = String::from_utf8(checked.stderr).unwrap();
            if rejects {
                assert!(!checked.status.success(), "{stderr}");
                assert!(stderr.contains("error[T0119]"), "{stderr}");
                assert!(stderr.contains("borrowed = ref value"), "{stderr}");
            } else {
                assert!(checked.status.success(), "{stderr}");
            }
        }
        if !rejects {
            assert_eq!(package.output(), expected_output);
        }
    }
}

#[test]
fn native_borrowed_literals_survive_projection_cache_reloads() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let package = Package(workspace.join("target").join(format!(
        "native-literal-cache-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )));
    copy_fixture(
        &workspace.join("tests/conformance/run/native-borrowed-future"),
        &package.0,
    );
    let expected = fs::read_to_string(package.0.join("stdout.txt")).unwrap();
    for _ in 0..2 {
        let snapshots = package.0.join(".trn/cache");
        if snapshots.exists() {
            fs::remove_dir_all(snapshots).unwrap();
        }
        assert_eq!(package.output(), expected);
    }
}

#[test]
fn local_native_dependency_edits_invalidate_cached_projection_and_compilation() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    // Rustdoc sandbox fixtures must live below the workspace bind, not its private /tmp.
    let root = workspace.join("target").join(format!(
        "native-compilation-cache-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let package = Package(root);
    copy_fixture(
        &workspace.join("tests/conformance/run/projected-native-alias-identity"),
        &package.0,
    );
    let expected = fs::read_to_string(package.0.join("stdout.txt")).unwrap();
    assert_eq!(package.output(), expected);
    assert_eq!(package.output(), expected);
    let compilation_stamp = package.mark_cache("compilation");
    let warm = package.build();
    assert!(
        warm.status.success(),
        "{}",
        String::from_utf8_lossy(&warm.stderr)
    );
    assert_eq!(package.cache_stamp("compilation"), compilation_stamp);
    let main = package.0.join("src/main.trn");
    let authored = fs::read_to_string(&main).unwrap();
    fs::write(
        &main,
        format!("{authored}\n// source-only cache regression\n"),
    )
    .unwrap();
    let edited = package.build();
    assert!(
        edited.status.success(),
        "{}",
        String::from_utf8_lossy(&edited.stderr)
    );
    assert!(String::from_utf8_lossy(&edited.stderr).contains("reused native semantic queries"));
    assert_eq!(package.output(), expected);
    let owner = package
        .0
        .join("fixture-registry/terrane-native-alias-owner-0.1.0/src/lib.rs");
    let source = fs::read_to_string(&owner).unwrap();
    fs::write(&owner, source.replace("Holder::new(19)", "Holder::new(39)")).unwrap();
    assert_eq!(package.output(), expected.replace("19\n", "39\n"));
    fs::write(
        &owner,
        source.replace(
            "pub fn owner_byte(value: Holder<u8>) -> u8 {\n    value.into_value()\n}",
            "pub fn owner_byte(value: u16) -> u8 {\n    value as u8\n}",
        ),
    )
    .unwrap();
    // `rust` only emits compiler output: this must reject the obsolete caller before
    // generated Cargo compilation, not rely on rustc to discover stale type contracts.
    let emitted = Command::new(env!("CARGO_BIN_EXE_terrane"))
        .arg("rust")
        .arg(package.0.join("package.toml"))
        .output()
        .unwrap();
    assert!(!emitted.status.success());
    assert_eq!(emitted.stdout, b"");
}

#[test]
fn consumers_execute_with_cached_declarations_and_changed_output_rebuilds() {
    let package = Package::new();
    let consumer = package.0.join("consumer.py");
    fs::write(&consumer, r#"import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
request = json.load(sys.stdin)
assert request["format"] == 1
count = root / "executions"
count.write_text(str(int(count.read_text()) + 1 if count.exists() else 1))
value = int((root / "value").read_text())
source = f"namespace generated/cache\npublic function value int;\n  return {value}\n"
json.dump({"format": 1, "generated_sources": [{"identity": "value", "source": source}], "diagnostics": []}, sys.stdout)
"#).unwrap();
    fs::write(package.0.join("value"), "7").unwrap();
    fs::write(package.0.join("package.toml"), format!(
        "package = 'cache-test'\nprelude = false\n[namespaces]\napp = 'src'\n[consumers.cache]\ncommand = 'python3'\nargs = [{consumer:?}, {:?}]\ndeclarations = ['/app::seed']\n", package.0
    )).unwrap();
    package.source("namespace app\nfrom /core/output import print\nfrom /generated/cache import value\npublic function seed int;\n  return 1\nfunction main;\n  print; (value;)\n");
    assert_eq!(package.output(), "7\n");
    let declaration_stamp = package.mark_cache("declarations");
    let compilation_stamp = package.mark_cache("compilation");
    let warm = package.build();
    assert!(
        warm.status.success(),
        "{}",
        String::from_utf8_lossy(&warm.stderr)
    );
    assert_eq!(package.cache_stamp("declarations"), declaration_stamp);
    assert_eq!(package.cache_stamp("compilation"), compilation_stamp);
    assert_eq!(
        fs::read_to_string(package.0.join("executions")).unwrap(),
        "2"
    );
    fs::write(package.0.join("value"), "9").unwrap();
    assert_eq!(package.output(), "9\n");
    assert_eq!(package.cache_stamp("declarations"), declaration_stamp);
    assert_eq!(
        fs::read_to_string(package.0.join("executions")).unwrap(),
        "3"
    );
    // Consumer execution failure must not be hidden by either warm snapshot.
    fs::write(package.0.join("value"), "not an integer").unwrap();
    let failed = package.build();
    assert!(!failed.status.success());
    assert_eq!(failed.stdout, b"");
}

#[cfg(unix)]
fn rejecting_consumer(package: &Package, mutate_inputs: bool) {
    let manifest = package.0.join("package.toml");
    let mut text = fs::read_to_string(&manifest).unwrap();
    text.push_str("\n[consumers.generate]\ncommand = '/bin/sh'\nargs = ['consumer.sh']\ndeclarations = ['/app::main']\n");
    fs::write(manifest, text).unwrap();
    let mutation = if mutate_inputs {
        "printf 'changed compiler input' > .trn/projection-cache.json\n"
    } else {
        ""
    };
    fs::write(
        package.0.join("consumer.sh"),
        format!("cat > /dev/null\n{mutation}cat response.json\n"),
    )
    .unwrap();
    fs::write(
        package.0.join("response.json"),
        serde_json::json!({
            "format": 1, "generated_sources": [],
            "diagnostics": [{"declaration": null, "message": "input rejected"}]
        })
        .to_string(),
    )
    .unwrap();
}

#[cfg(unix)]
#[test]
fn rejected_consumers_reuse_metadata_but_still_refresh_output() {
    let package = Package::new();
    package.source("namespace app\nfrom /core/output import print\nfrom /generated/answer import answer\nfunction main;\n    print; (answer;)\n");
    rejecting_consumer(&package, false);
    let rejected = package.build();
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("error[S2061]: input rejected"));
    let cache = package.0.join(".trn/cache/declarations.json");
    assert!(
        cache.is_file(),
        "successful declaration analysis survives consumer rejection"
    );
    let cache_stamp = package.mark_cache("declarations");
    // A second rejection must consult the consumer but retain the loaded metadata
    // file, rather than deterministically recomputing and replacing its envelope.
    fs::write(
        package.0.join("response.json"),
        serde_json::json!({
            "format": 1, "generated_sources": [],
            "diagnostics": [{"declaration": null, "message": "second rejection"}]
        })
        .to_string(),
    )
    .unwrap();
    let rejected_again = package.build();
    assert!(!rejected_again.status.success());
    let diagnostic = String::from_utf8_lossy(&rejected_again.stderr);
    assert!(diagnostic.contains("S2061") && diagnostic.contains("second rejection"));
    assert_eq!(package.cache_stamp("declarations"), cache_stamp);
    for value in [42, 43] {
        fs::write(
            package.0.join("response.json"),
            serde_json::json!({
                "format": 1,
                "generated_sources": [{"identity": "answer", "source": format!(
                    "namespace generated/answer\npublic function answer int;\n    return {value}\n"
                )}],
                "diagnostics": [],
            })
            .to_string(),
        )
        .unwrap();
        assert_eq!(package.output(), format!("{value}\n"));
        assert_eq!(package.cache_stamp("declarations"), cache_stamp);
    }
}

#[cfg(unix)]
#[test]
fn rejected_consumer_input_mutations_do_not_publish_metadata() {
    let package = Package::new();
    package.source("namespace app\nfunction main int;\n    return 0\n");
    rejecting_consumer(&package, true);
    let rejected = package.build();
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("error[S2061]: input rejected"));
    assert_eq!(
        fs::read_to_string(package.0.join(".trn/projection-cache.json")).unwrap(),
        "changed compiler input"
    );
    assert!(
        !package.0.join(".trn/cache/declarations.json").exists(),
        "consumer mutation invalidates the bracketed compiler inputs"
    );
}
