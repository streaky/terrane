//! Content identities shared by compiler and native projection caches.
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

pub(crate) const COMPILER: &str = env!("TERRANE_COMPILER_CACHE_ID");

/// Cargo locks identify versions, but local path/patch packages can change within a version.
/// Hash their actual input trees, including transitive local dependencies. Registry packages
/// retain Cargo's checksum/lock identity; build outputs and VCS bookkeeping are not inputs.
pub(crate) fn local_rust_sources(workspace: &Path) -> Result<String, String> {
    let manifest = workspace
        .join("Cargo.toml")
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--offline",
            "--locked",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(&manifest)
        .current_dir(workspace)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "cannot fingerprint local Rust dependencies: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())?;
    let packages = metadata
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or("Cargo metadata has no packages")?;
    let mut paths = packages
        .iter()
        .filter(|package| {
            package
                .get("source")
                .is_some_and(serde_json::Value::is_null)
        })
        .filter_map(|package| package.get("manifest_path")?.as_str().map(PathBuf::from))
        .collect::<Vec<_>>();
    paths.sort();
    let mut hash = Sha256::new();
    // Hash effective configuration, including included config files and environment overrides.
    let configuration = Command::new("cargo")
        .arg(format!("+{}", terrane_rust_analysis::RUSTDOC_TOOLCHAIN))
        .args([
            "-Z",
            "unstable-options",
            "config",
            "get",
            "--format",
            "json",
        ])
        .current_dir(workspace)
        .output()
        .map_err(|error| error.to_string())?;
    if !configuration.status.success() {
        return Err(format!(
            "cannot fingerprint Cargo configuration: {}",
            String::from_utf8_lossy(&configuration.stderr).trim()
        ));
    }
    part(&mut hash, &configuration.stdout);
    let mut visited = BTreeSet::new();
    for path in paths {
        let parent = path.parent().ok_or("local Rust manifest has no parent")?;
        if parent == manifest.parent().unwrap() {
            continue;
        }
        tree(parent, &mut hash, &mut visited)?;
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn part(hash: &mut Sha256, bytes: &[u8]) {
    hash.update((bytes.len() as u64).to_le_bytes());
    hash.update(bytes);
}

fn tree(root: &Path, hash: &mut Sha256, visited: &mut BTreeSet<PathBuf>) -> Result<(), String> {
    if !visited.insert(root.canonicalize().map_err(|error| error.to_string())?) {
        return Ok(());
    }
    part(hash, root.as_os_str().as_encoded_bytes());
    let mut paths = fs::read_dir(root)
        .map_err(|error| error.to_string())?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|error| error.to_string())?;
    paths.sort();
    for path in paths {
        if path.is_dir() {
            if matches!(
                path.file_name().and_then(|name| name.to_str()),
                Some("target" | ".git" | ".trn")
            ) {
                continue;
            }
            tree(&path, hash, visited)?;
        } else {
            part(hash, path.as_os_str().as_encoded_bytes());
            let content =
                fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
            part(hash, &content);
        }
    }
    Ok(())
}
