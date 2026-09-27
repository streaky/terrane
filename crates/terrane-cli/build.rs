#[path = "src/development_fingerprint.rs"]
mod development_fingerprint;

use development_fingerprint::{dirty_fingerprint, git_output};
use std::path::{Path, PathBuf};

fn main() {
    let manifest = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest.parent().and_then(Path::parent).unwrap();
    println!(
        "cargo:rerun-if-changed={}",
        root.join("Cargo.toml").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        root.join("Cargo.lock").display()
    );
    println!("cargo:rerun-if-changed={}", root.join("crates").display());
    if let Some(head_path) = git_output(root, &["rev-parse", "--git-path", "HEAD"]) {
        let head_path = String::from_utf8_lossy(&head_path).trim().to_owned();
        println!("cargo:rerun-if-changed={}", root.join(head_path).display());
    }
    if let Some(reference) = git_output(root, &["symbolic-ref", "-q", "HEAD"]) {
        let reference = String::from_utf8_lossy(&reference).trim().to_owned();
        if let Some(reference_path) =
            git_output(root, &["rev-parse", "--git-path", reference.as_str()])
        {
            let reference_path = String::from_utf8_lossy(&reference_path).trim().to_owned();
            println!(
                "cargo:rerun-if-changed={}",
                root.join(reference_path).display()
            );
        }
    }

    let canonical_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let head = git_output(&canonical_root, &["rev-parse", "HEAD"])
        .map(|output| String::from_utf8_lossy(&output).trim().to_owned())
        .unwrap_or_default();
    let fingerprint = dirty_fingerprint(&canonical_root).unwrap_or_default();

    println!(
        "cargo:rustc-env=TERRANE_BUILD_REPOSITORY={}",
        canonical_root.display()
    );
    println!("cargo:rustc-env=TERRANE_BUILD_GIT_HEAD={head}");
    println!("cargo:rustc-env=TERRANE_BUILD_DIRTY_FINGERPRINT={fingerprint:016x}");
}
