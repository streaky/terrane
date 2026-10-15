use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use terrane_compiler::{Package, analyze};

static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target")
            .join(format!(
                "native-runtime-demand-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join(".trn/dependencies/.cargo")).unwrap();
        for name in ["witness", "legacy"] {
            fs::create_dir_all(root.join(name).join("src")).unwrap();
        }
        fs::write(root.join("package.toml"), concat!(
            "package = \"runtime-demand\"\n[namespaces]\napp = \"src\"\n",
            "[rust-dependencies.multi-runtime-witness]\nversion = \"=0.1.0\"\neffects = [\"build\"]\n"
        )).unwrap();
        // The legacy crate witnesses a distinct owner; no legacy networking API is used.
        fs::write(
            root.join("legacy/Cargo.toml"),
            "[package]\nname = \"tokio\"\nversion = \"0.2.25\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(root.join("legacy/src/lib.rs"), "pub struct Legacy;\n").unwrap();
        fs::write(root.join("witness/Cargo.toml"), concat!(
            "[package]\nname = \"multi-runtime-witness\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
            "[dependencies]\ntokio = \"=1.47.1\"\nlegacy = { package = \"tokio\", version = \"=0.2.25\" }\n"
        )).unwrap();
        fs::write(
            root.join("witness/src/lib.rs"),
            "pub fn value() -> i64 { 42 }\n",
        )
        .unwrap();
        fs::write(root.join(".trn/dependencies/.cargo/config.toml"), format!(
            "[patch.crates-io]\nmulti-runtime-witness = {{ path = {} }}\ntokio = {{ path = {} }}\n",
            toml::Value::String(root.join("witness").to_string_lossy().into_owned()),
            toml::Value::String(root.join("legacy").to_string_lossy().into_owned())
        )).unwrap();
        Self(root)
    }

    fn package(&self, source: &str) -> Package {
        fs::write(self.0.join("src/main.trn"), source).unwrap();
        Package::load(&self.0).unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn ordinary_networking_allows_multiple_tokio_owners_but_native_transfer_rejects_them() {
    let fixture = Fixture::new();
    let package = fixture.package(concat!(
        "namespace app\nfrom /core/networking import listen-tcp\n",
        "async function main;\n    listener = await listen-tcp; '127.0.0.1:0'\n    listener.close;\n"
    ));
    let analyzed = analyze(&package).expect("ordinary networking must not demand native ownership");
    assert_eq!(analyzed.projection.bound_dependencies.len(), 0);
    let lock: toml::Value = toml::from_str(
        &fs::read_to_string(fixture.0.join(".trn/dependencies/Cargo.lock")).unwrap(),
    )
    .unwrap();
    let versions = lock["package"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|package| package["name"].as_str() == Some("tokio"))
        .map(|package| package["version"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(versions, ["0.2.25", "1.47.1"]);

    let package = fixture.package(concat!(
        "namespace app\nfrom /core/networking import listen-tcp\n",
        "async function main;\n    listener = await listen-tcp; '127.0.0.1:0'\n    native = listener.into-native;\n"
    ));
    let error = analyze(&package).expect_err("native transfer cannot choose an ambiguous owner");
    assert!(error.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "S2028"
            && diagnostic
                .message
                .contains("requires one resolved Tokio owner")
    }));
}
