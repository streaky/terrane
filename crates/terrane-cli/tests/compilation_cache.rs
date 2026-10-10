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
    let warm = package.build();
    assert!(
        warm.status.success(),
        "{}",
        String::from_utf8_lossy(&warm.stderr)
    );
    assert!(String::from_utf8_lossy(&warm.stderr).contains("reusing unchanged compilation"));
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
    assert!(rejected.stdout.is_empty());
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
    assert!(rejected.stdout.is_empty());
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
    let warm = package.build();
    assert!(
        warm.status.success(),
        "{}",
        String::from_utf8_lossy(&warm.stderr)
    );
    assert!(String::from_utf8_lossy(&warm.stderr).contains("reusing unchanged compilation"));
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
    assert!(emitted.stdout.is_empty());
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
    let warm = package.build();
    assert!(
        warm.status.success(),
        "{}",
        String::from_utf8_lossy(&warm.stderr)
    );
    let progress = String::from_utf8_lossy(&warm.stderr);
    assert!(
        progress.contains("reusing declaration metadata"),
        "{progress}"
    );
    assert!(
        progress.contains("reusing unchanged compilation"),
        "{progress}"
    );
    assert_eq!(
        fs::read_to_string(package.0.join("executions")).unwrap(),
        "2"
    );
    fs::write(package.0.join("value"), "9").unwrap();
    assert_eq!(package.output(), "9\n");
    assert_eq!(
        fs::read_to_string(package.0.join("executions")).unwrap(),
        "3"
    );
    // Consumer execution failure must not be hidden by either warm snapshot.
    fs::write(package.0.join("value"), "not an integer").unwrap();
    let failed = package.build();
    assert!(!failed.status.success());
    assert!(failed.stdout.is_empty());
}
