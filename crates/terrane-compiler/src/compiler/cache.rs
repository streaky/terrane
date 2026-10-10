//! Successful compiler outputs are reusable only against the complete current input snapshot.
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::Digest as _;

use crate::{CompilerOptions, Package};

const FORMAT: u32 = 1;

struct Fingerprint(crate::cache_io::HashWriter);
impl Fingerprint {
    fn new() -> Self {
        Self(crate::cache_io::HashWriter::new())
    }
    fn part(&mut self, bytes: &[u8]) {
        self.0.part(bytes);
    }
    fn file(&mut self, path: &Path) -> Option<()> {
        self.part(path.as_os_str().as_encoded_bytes());
        match fs::read(path) {
            Ok(bytes) => {
                self.part(b"present");
                self.part(&bytes);
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => self.part(b"absent"),
            Err(_) => return None,
        }
        Some(())
    }
}

/// External inputs are captured independently of in-memory source and options.
#[derive(PartialEq, Eq)]
pub(super) struct Inputs(String);

pub(super) fn current(package: &Package) -> Option<Inputs> {
    if !crate::cache_scope::enabled() {
        return None;
    }
    let mut hash = Fingerprint::new();
    hash.part(crate::cache_identity::COMPILER.as_bytes());
    let root = std::path::absolute(&package.root).ok()?;
    hash.part(root.as_os_str().as_encoded_bytes());
    // Consumers are executed before compilation lookup; their actual generated output is part
    // of the expanded package key, so arbitrary consumer environment is not cached here.
    // Shell/tool environment is still relevant to Cargo and Rust dependency resolution.
    let mut environment = std::env::vars_os().collect::<Vec<_>>();
    environment.sort();
    let include_all_environment = !package.rust_dependencies.is_empty();
    for (name, value) in environment.into_iter().filter(|(name, _)| {
        include_all_environment
            || name.to_str().is_some_and(|name| {
                name.starts_with("CARGO_")
                    || name.starts_with("RUST")
                    || name.starts_with("TERRANE_")
                    || matches!(
                        name,
                        "PATH"
                            | "HOME"
                            | "CARGO_HOME"
                            | "RUSTUP_HOME"
                            | "RUSTUP_TOOLCHAIN"
                            | "CC"
                            | "CXX"
                            | "AR"
                            | "TARGET"
                            | "HOST"
                            | "PKG_CONFIG_PATH"
                            | "LIBRARY_PATH"
                    )
            })
    }) {
        hash.part(name.as_encoded_bytes());
        hash.part(value.as_encoded_bytes());
    }
    for ancestor in root.ancestors() {
        hash.file(&ancestor.join(".cargo/config"))?;
        hash.file(&ancestor.join(".cargo/config.toml"))?;
    }
    if let Some(home) = std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".cargo")))
    {
        hash.file(&home.join("config"))?;
        hash.file(&home.join("config.toml"))?;
    }
    for manifest in &package.dependency_manifests {
        hash.file(manifest)?;
    }
    for consumer in &package.consumer_configs {
        hash.file(&consumer.executable)?;
        for interface in &consumer.interfaces {
            hash.file(interface)?;
        }
    }
    for name in [
        "terrane-dependencies.lock",
        "terrane-projection.lock",
        ".trn/dependencies/Cargo.toml",
        ".trn/dependencies/Cargo.lock",
        ".trn/dependencies/.cargo/config",
        ".trn/dependencies/.cargo/config.toml",
        ".trn/dependencies/projection.json",
    ] {
        hash.file(&root.join(name))?;
    }
    if !package.rust_dependencies.is_empty() {
        let sources =
            crate::cache_identity::local_rust_sources(&root.join(".trn/dependencies")).ok()?;
        if fs::read(root.join(".trn/dependencies/local-source-identity"))
            .ok()?
            .as_slice()
            != sources.as_bytes()
        {
            return None;
        }
        hash.part(sources.as_bytes());
    }
    Some(Inputs(hash.0.finish_hex()))
}

/// In-memory source content is authoritative, including generated consumer output.
pub(super) fn key(package: &Package, options: CompilerOptions, inputs: &Inputs) -> Option<String> {
    let mut hash = crate::cache_io::HashWriter::new();
    hash.part(inputs.0.as_bytes());
    serde_json::to_writer(&mut hash, package).ok()?;
    hash.part(b"package-end");
    serde_json::to_writer(&mut hash, &options).ok()?;
    Some(hash.finish_hex())
}
#[derive(Deserialize, Serialize)]
struct Envelope<T> {
    format: u32,
    compiler: String,
    key: String,
    checksum: String,
    value: T,
}

pub(super) fn load<T: serde::de::DeserializeOwned>(
    root: &Path,
    kind: &str,
    key: &str,
) -> Option<T> {
    if !crate::cache_scope::enabled() {
        return None;
    }
    let bytes = fs::read(root.join(".trn/cache").join(format!("{kind}.json"))).ok()?;
    let mut deserializer = serde_json::Deserializer::from_slice(&bytes);
    let entry: Envelope<&serde_json::value::RawValue> =
        Envelope::deserialize(&mut deserializer).ok()?;
    deserializer.end().ok()?;
    if entry.format != FORMAT
        || entry.compiler != crate::cache_identity::COMPILER
        || entry.key != key
        || format!("{:x}", sha2::Sha256::digest(entry.value.get().as_bytes())) != entry.checksum
    {
        return None;
    }
    serde_json::from_str(entry.value.get()).ok()
}

pub(super) fn store<T: Serialize>(root: &Path, kind: &str, key: &str, value: &T) {
    if !crate::cache_scope::enabled() {
        return;
    }
    let Ok(encoded) = serde_json::to_vec(value) else {
        return;
    };
    let checksum = format!("{:x}", sha2::Sha256::digest(&encoded));
    let value = std::str::from_utf8(&encoded).expect("JSON serialization is UTF-8");
    let Ok(mut bytes) = serde_json::to_vec(&serde_json::json!({
        "format": FORMAT,
        "compiler": crate::cache_identity::COMPILER,
        "key": key,
        "checksum": checksum,
    })) else {
        return;
    };
    bytes.pop();
    bytes.extend_from_slice(b",\"value\":");
    bytes.extend_from_slice(value.as_bytes());
    bytes.push(b'}');
    crate::cache_io::atomic_write(
        &root.join(".trn/cache").join(format!("{kind}.json")),
        &bytes,
    );
}

// Diagnostic codes are owned by Diagnostic, so future codes round-trip without a cache-side list.
pub(super) mod warnings {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize)]
    struct Borrowed<'a> {
        code: &'a str,
        message: &'a str,
        primary: Option<crate::Span>,
        help: &'a Option<String>,
    }
    #[derive(Deserialize)]
    struct Stored {
        code: String,
        message: String,
        primary: Option<crate::Span>,
        help: Option<String>,
    }
    pub fn serialize<S: serde::Serializer>(
        values: &[crate::Diagnostic],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(values.len()))?;
        for value in values {
            seq.serialize_element(&Borrowed {
                code: value.code.as_ref(),
                message: &value.message,
                primary: value.primary,
                help: &value.help,
            })?;
        }
        seq.end()
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<crate::Diagnostic>, D::Error> {
        Vec::<Stored>::deserialize(deserializer)?
            .into_iter()
            .map(|value| {
                Ok(crate::Diagnostic {
                    severity: crate::diagnostic::Severity::Warning,
                    code: std::borrow::Cow::Owned(value.code),
                    message: value.message,
                    primary: value.primary,
                    help: value.help,
                })
            })
            .collect()
    }
}

#[cfg(test)]
#[test]
fn cached_warning_round_trips_new_owned_code() {
    #[derive(Serialize, Deserialize)]
    struct Snapshot(#[serde(with = "warnings")] Vec<crate::Diagnostic>);

    let original = Snapshot(vec![crate::Diagnostic::warning(
        "W4999",
        "new warning",
        crate::Span::new(1, 2, 3),
    )]);
    let bytes = serde_json::to_vec(&original).unwrap();
    let restored: Snapshot = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(restored.0[0].code.as_ref(), "W4999");
    assert_eq!(restored.0[0].primary, Some(crate::Span::new(1, 2, 3)));
}
