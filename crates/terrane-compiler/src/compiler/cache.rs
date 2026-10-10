//! Successful compiler outputs are reusable only against the complete current input snapshot.
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{CompilerOptions, Package};

const FORMAT: u32 = 1;
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct Fingerprint(Sha256);
impl Fingerprint {
    fn part(&mut self, bytes: &[u8]) {
        self.0.update((bytes.len() as u64).to_le_bytes());
        self.0.update(bytes);
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
impl std::fmt::Write for Fingerprint {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        self.0.update(text.as_bytes());
        Ok(())
    }
}
impl io::Write for Fingerprint {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.update(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// In-memory package content is authoritative (including editor overlays), not filesystem mtimes.
pub(super) fn key(package: &Package, options: CompilerOptions) -> Option<String> {
    let mut hash = Fingerprint(Sha256::new());
    hash.part(crate::cache_identity::COMPILER.as_bytes());
    std::fmt::Write::write_fmt(&mut hash, format_args!("{package:?}\0")).ok()?;
    serde_json::to_writer(&mut hash, &options).ok()?;
    hash.part(b"options-end");
    let root = std::path::absolute(&package.root).ok()?;
    hash.part(root.as_os_str().as_encoded_bytes());
    let mut environment = std::env::vars_os().collect::<Vec<_>>();
    environment.sort();
    for (name, value) in environment {
        // Shell bookkeeping and progress settings cannot change semantic compilation.
        if matches!(
            name.to_str(),
            Some(
                "_" | "PWD"
                    | "OLDPWD"
                    | "SHLVL"
                    | "CARGO_MAKEFLAGS"
                    | "MAKEFLAGS"
                    | "CARGO_TERM_PROGRESS_WHEN"
                    | "CARGO_TERM_PROGRESS_WIDTH"
            )
        ) {
            continue;
        }
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
        // Do not read or publish an output against dependency inputs that its projection
        // has not observed yet (including an edit arriving during compilation).
        if fs::read(root.join(".trn/dependencies/local-source-identity"))
            .ok()?
            .as_slice()
            != sources.as_bytes()
        {
            return None;
        }
        hash.part(sources.as_bytes());
    }
    Some(format!("{:x}", hash.0.finalize()))
}

#[derive(Deserialize, Serialize)]
struct Envelope<T> {
    format: u32,
    compiler: String,
    key: String,
    checksum: String,
    value: T,
}

pub(super) fn load<T: serde::de::DeserializeOwned + Serialize>(
    root: &Path,
    kind: &str,
    key: &str,
) -> Option<T> {
    let path = root.join(".trn/cache").join(format!("{kind}.json"));
    let bytes = fs::read(path).ok()?;
    let entry: Envelope<T> = serde_json::from_slice(&bytes).ok()?;
    if entry.format != FORMAT
        || entry.compiler != crate::cache_identity::COMPILER
        || entry.key != key
    {
        return None;
    }
    let value = serde_json::to_vec(&entry.value).ok()?;
    if format!("{:x}", Sha256::digest(value)) != entry.checksum {
        return None;
    }
    Some(entry.value)
}

pub(super) fn store<T: Serialize>(root: &Path, kind: &str, key: &str, value: &T) {
    let Ok(encoded) = serde_json::to_vec(value) else {
        return;
    };
    let checksum = format!("{:x}", Sha256::digest(&encoded));
    let entry = Envelope {
        format: FORMAT,
        compiler: crate::cache_identity::COMPILER.to_owned(),
        key: key.to_owned(),
        checksum,
        value,
    };
    let Ok(bytes) = serde_json::to_vec(&entry) else {
        return;
    };
    let parent = root.join(".trn/cache");
    if fs::create_dir_all(&parent).is_err() {
        return;
    }
    let target = parent.join(format!("{kind}.json"));
    let temp = parent.join(format!(
        ".{kind}-{}-{}.tmp",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    if let Ok(mut file) = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
    {
        let written = file.write_all(&bytes).is_ok();
        drop(file);
        if written {
            let _ = fs::rename(&temp, &target);
        }
        let _ = fs::remove_file(&temp);
    }
}

// Diagnostic codes have static ownership. Decode only compiler-known warning codes rather than
// leaking arbitrary strings into the process; a future unsupported code safely misses the cache.
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
                code: value.code,
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
                let code = match value.code.as_str() {
                    "S2018" => "S2018",
                    "W4001" => "W4001",
                    "W4002" => "W4002",
                    "W4003" => "W4003",
                    "W4004" => "W4004",
                    "W4005" => "W4005",
                    "W4006" => "W4006",
                    _ => return Err(serde::de::Error::custom("unknown cached warning code")),
                };
                Ok(crate::Diagnostic {
                    severity: crate::diagnostic::Severity::Warning,
                    code,
                    message: value.message,
                    primary: value.primary,
                    help: value.help,
                })
            })
            .collect()
    }
}
