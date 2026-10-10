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
