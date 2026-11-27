//! Warms Cargo's package cache from an operator lock without building or projecting code.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) const DEPENDENCY_LOCK_FILE: &str = "terrane-dependencies.lock";
const PROJECTION_ROOT: &str = "terrane_dependency_projection";
const CRATES_IO: &str = "registry+https://github.com/rust-lang/crates.io-index";
static SCRATCH_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(crate) fn hydrate_locked_crates(root: &Path) -> Result<(), String> {
    let lock_path = root.join(DEPENDENCY_LOCK_FILE);
    let text = fs::read_to_string(&lock_path).map_err(|error| {
        if error.kind() == io::ErrorKind::NotFound {
            format!(
                "missing {DEPENDENCY_LOCK_FILE} at `{}`; use a package with a completed dependency lock",
                lock_path.display()
            )
        } else {
            format!("cannot read `{}`: {error}", lock_path.display())
        }
    })?;
    let lock = parse_lock(&text)
        .and_then(|lock| {
            validate_projection_root(&lock)?;
            Ok(lock)
        })
        .map_err(|error| format!("invalid dependency lock `{}`: {error}", lock_path.display()))?;
    let expected = registry_packages(&lock)
        .map_err(|error| format!("invalid dependency lock `{}`: {error}", lock_path.display()))?;
    if expected.is_empty() {
        // Local patches already refer to operator-owned files, not fetchable archives.
        return Ok(());
    }

    // Preserve Cargo's directory-based configuration (including path patches and source
    // replacements) without writing into an existing projection workspace or its lock.
    let scratch = Scratch::create(&root.join(".trn/dependencies"))?;
    let directory = &scratch.0;
    let mut manifest = format!(
        "[package]\nname = {PROJECTION_ROOT:?}\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[lib]\npath = \"hydrate.rs\"\n\n[workspace]\n\n[dependencies]\n"
    );
    for (i, &(name, version)) in expected.keys().enumerate() {
        writeln!(
            manifest,
            "locked_{i} = {{ package = {name:?}, version = \"={version}\", default-features = false }}"
        )
        .expect("writing to a string cannot fail");
    }
    for (name, contents) in [
        ("Cargo.toml", manifest.as_str()),
        ("hydrate.rs", ""),
        ("Cargo.lock", &text),
    ] {
        fs::write(directory.join(name), contents)
            .map_err(|error| format!("cannot stage Cargo hydration {name}: {error}"))?;
    }

    // The synthetic root requests archives rather than recreating projection features. Use
    // the input lock as Cargo's resolution seed, then assert the complete source-backed
    // package set and checksums before letting Cargo download from the synthetic graph.
    run_cargo(directory, &["update", "--workspace"])?;
    let resolved = fs::read_to_string(directory.join("Cargo.lock"))
        .map_err(|error| format!("cannot read staged Cargo resolution: {error}"))?;
    let resolved = parse_lock(&resolved)?;
    let actual = registry_packages(&resolved)?;
    if actual != expected {
        let changed = expected
            .iter()
            .find(|(key, checksum)| actual.get(*key) != Some(*checksum))
            .or_else(|| actual.iter().find(|(key, _)| !expected.contains_key(*key)))
            .map_or_else(
                || "unknown package".to_owned(),
                |(&(name, version), _)| format!("{name} {version}"),
            );
        return Err(format!(
            "locked Cargo hydration failed: staged resolution changed the locked identity or checksum for `{changed}`"
        ));
    }
    run_cargo(directory, &["fetch", "--locked"])
}

pub(crate) fn lock_contains_registry_package(root: &Path, name: &str, version: &str) -> bool {
    fs::read_to_string(root.join(DEPENDENCY_LOCK_FILE))
        .ok()
        .and_then(|text| parse_lock(&text).ok())
        .is_some_and(|lock| {
            validate_projection_root(&lock).is_ok()
                && registry_packages(&lock)
                    .is_ok_and(|packages| packages.contains_key(&(name, version)))
        })
}

#[derive(serde::Deserialize)]
struct Lock {
    version: u32,
    package: Vec<LockedPackage>,
}

#[derive(serde::Deserialize)]
struct LockedPackage {
    name: String,
    version: String,
    source: Option<String>,
    checksum: Option<String>,
}

fn parse_lock(text: &str) -> Result<Lock, String> {
    let lock: Lock = toml::from_str(text).map_err(|error| error.to_string())?;
    if lock.version != 4 {
        return Err(format!(
            "unsupported Cargo lock format {} (expected 4)",
            lock.version
        ));
    }
    Ok(lock)
}

fn validate_projection_root(lock: &Lock) -> Result<(), String> {
    let mut roots = lock.package.iter().filter(|package| {
        package.source.is_none() && package.name == PROJECTION_ROOT && package.version == "0.0.0"
    });
    if roots.next().is_none() || roots.next().is_some() || lock.package.len() <= 1 {
        return Err("expected a completed dependency projection lock with a single projection root and at least one dependency".to_owned());
    }
    Ok(())
}

fn registry_packages(lock: &Lock) -> Result<BTreeMap<(&str, &str), &str>, String> {
    let mut packages = BTreeMap::new();
    for package in &lock.package {
        let Some(source) = package.source.as_deref() else {
            continue;
        };
        if source != CRATES_IO {
            return Err(format!(
                "unsupported Cargo source `{source}` for `{}`",
                package.name
            ));
        }
        let name = &package.name;
        let version = &package.version;
        if name.is_empty()
            || !name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"-_".contains(&byte))
            || version.is_empty()
            || !version
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b".-+".contains(&byte))
        {
            return Err(format!("invalid locked Cargo identity `{name} {version}`"));
        }
        let checksum = package
            .checksum
            .as_deref()
            .filter(|checksum| {
                checksum.len() == 64 && checksum.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
            .ok_or_else(|| format!("missing or invalid Cargo checksum for `{name} {version}`"))?;
        if packages
            .insert((name.as_str(), version.as_str()), checksum)
            .is_some()
        {
            return Err(format!(
                "duplicate locked Cargo identity `{name} {version}`"
            ));
        }
    }
    Ok(packages)
}

fn run_cargo(directory: &Path, arguments: &[&str]) -> Result<(), String> {
    offline_first(|offline| {
        let mut command = Command::new("cargo");
        crate::rust_interop::configure_projection_cargo_command(&mut command);
        command.args(arguments).current_dir(directory);
        if offline {
            command.arg("--offline");
        }
        command
            .output()
            .map_err(|error| format!("cannot run Cargo hydration: {error}"))
    })
}

fn offline_first(mut execute: impl FnMut(bool) -> Result<Output, String>) -> Result<(), String> {
    if execute(true)?.status.success() {
        return Ok(());
    }
    // Cargo's failure text is not stable. Let the second attempt report the error if
    // access to the network does not resolve it, while respecting Cargo's own config.
    let output = execute(false)?;
    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "locked Cargo hydration failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

struct Scratch(PathBuf);

impl Scratch {
    fn create(parent: &Path) -> Result<Self, String> {
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create Cargo hydration parent: {error}"))?;
        loop {
            let sequence = SCRATCH_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = parent.join(format!("fetch-{}-{sequence}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => (),
                Err(error) => {
                    return Err(format!("cannot create Cargo hydration workspace: {error}"));
                }
            }
        }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::process::ExitStatusExt as _;

    fn cargo_output(exit: i32, stderr: &str) -> Output {
        Output {
            status: std::process::ExitStatus::from_raw(exit << 8),
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }

    #[test]
    fn successful_hydration_stays_offline() {
        offline_first(|offline| {
            assert!(offline);
            Ok(cargo_output(0, ""))
        })
        .unwrap();
    }

    #[test]
    fn retry_reports_online_error() {
        for exit in [0, 101] {
            let mut attempts = [true, false].into_iter();
            let result = offline_first(|offline| {
                assert_eq!(attempts.next(), Some(offline));
                Ok(if offline {
                    cargo_output(101, "unrecognized wording")
                } else {
                    cargo_output(exit, "online diagnostic")
                })
            });
            assert_eq!(attempts.next(), None);
            if exit == 0 {
                result.unwrap();
            } else {
                assert_eq!(
                    result.unwrap_err(),
                    "locked Cargo hydration failed: online diagnostic"
                );
            }
        }
    }
}
