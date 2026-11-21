#[path = "src/development_fingerprint.rs"]
mod development_fingerprint;

use development_fingerprint::{compiler_input_fingerprint, compiler_input_paths};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest.parent().and_then(|path| path.parent()).unwrap();
    if let Ok(paths) = compiler_input_paths(root) {
        for path in paths {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    let canonical_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let fingerprint = compiler_input_fingerprint(&canonical_root);
    println!(
        "cargo:rustc-env=TERRANE_BUILD_REPOSITORY={}",
        canonical_root.display()
    );
    println!(
        "cargo:rustc-env=TERRANE_BUILD_INPUT_FINGERPRINT={:016x}",
        fingerprint.unwrap_or_default()
    );
    let identity = if cfg!(feature = "release-build") {
        "release".to_owned()
    } else {
        development_identity(&canonical_root)
            .unwrap_or_else(|| "development, Git metadata unavailable".to_owned())
    };
    println!("cargo:rustc-env=TERRANE_BUILD_IDENTITY={identity}");
}

fn development_identity(root: &Path) -> Option<String> {
    let dot_git = root.join(".git");
    if dot_git.is_file() {
        watch(&dot_git);
    } else if !dot_git.is_dir() {
        // Source archives nested in another checkout must not report its revision.
        return None;
    }

    watch(&git_path(root, "HEAD")?);
    let branch = git(root, &["symbolic-ref", "--quiet", "HEAD"]);
    if let Some(branch) = &branch {
        let mut watched_ref = git_path(root, branch)?;
        if !watched_ref.is_file() {
            // A packed or unborn branch can acquire a loose ref on the next commit.
            if let Some(packed_refs) = git_path(root, "packed-refs").filter(|path| path.is_file()) {
                watch(&packed_refs);
            }
            while !watched_ref.exists() {
                watched_ref = watched_ref.parent()?.to_path_buf();
            }
        }
        watch(&watched_ref);
    }
    if let Some(tables) = git_path(root, "reftable/tables.list").filter(|path| path.is_file()) {
        watch(&tables);
    }

    let commit = git(root, &["rev-parse", "--short", "HEAD"])?;
    let branch = branch.as_deref().map_or("detached HEAD", |name| {
        name.strip_prefix("refs/heads/").unwrap_or(name)
    });
    Some(format!("development, git {commit}, branch {branch}"))
}

fn git_path(root: &Path, name: &str) -> Option<PathBuf> {
    git(
        root,
        &["rev-parse", "--path-format=absolute", "--git-path", name],
    )
    .map(PathBuf::from)
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .into()
    })
}

fn watch(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
}
