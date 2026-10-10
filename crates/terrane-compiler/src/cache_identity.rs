//! Content identities shared by compiler and native projection caches.
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cache_io::HashWriter;

pub(crate) const COMPILER: &str = env!("TERRANE_COMPILER_CACHE_ID");

/// Hash local package contents, excluding Cargo outputs and compiler/VCS bookkeeping.
/// Cargo's package include/exclude rules do not cover all build-script input files, so
/// other directories remain conservatively included; large path dependencies cost more.
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
    let mut hash = HashWriter::new();
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
    hash.part(&configuration.stdout);
    let mut visited = BTreeSet::new();
    for path in paths {
        let parent = path.parent().ok_or("local Rust manifest has no parent")?;
        if parent == manifest.parent().unwrap() {
            continue;
        }
        tree(&mut hash, parent, &mut visited)?;
    }
    Ok(hash.finish_hex())
}

fn tree(hash: &mut HashWriter, root: &Path, visited: &mut BTreeSet<PathBuf>) -> Result<(), String> {
    if !visited.insert(root.canonicalize().map_err(|error| error.to_string())?) {
        return Ok(());
    }
    hash.part(root.as_os_str().as_encoded_bytes());
    let mut paths = fs::read_dir(root)
        .map_err(|error| error.to_string())?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<std::io::Result<Vec<_>>>()
        .map_err(|error| error.to_string())?;
    paths.sort();
    for path in paths {
        if matches!(
            path.file_name().and_then(|name| name.to_str()),
            Some("target" | ".git" | ".trn")
        ) {
            continue;
        }
        if path.is_dir() {
            tree(hash, &path, visited)?;
        } else {
            hash.part(path.as_os_str().as_encoded_bytes());
            let content =
                fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
            hash.part(&content);
        }
    }
    Ok(())
}
