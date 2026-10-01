#[path = "src/development_fingerprint.rs"]
mod development_fingerprint;

use development_fingerprint::{compiler_input_fingerprint, compiler_input_paths};
use std::path::PathBuf;

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
}
