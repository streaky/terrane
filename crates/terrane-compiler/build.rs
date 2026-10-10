use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

fn collect(directory: &Path, files: &mut Vec<PathBuf>) {
    println!("cargo:rerun-if-changed={}", directory.display());
    let mut entries = fs::read_dir(directory)
        .expect("compiler input directory is readable")
        .map(|entry| entry.expect("compiler input entry is readable").path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect(&path, files);
        } else if path.is_file() {
            files.push(path);
        }
    }
}

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let workspace = manifest.parent().unwrap().parent().unwrap();
    let mut files = Vec::new();
    // Include local runtime/support crates too: generated programs depend on their ABI.
    let mut crates = fs::read_dir(workspace.join("crates"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    crates.sort();
    for directory in crates {
        let source = directory.join("src");
        if source.is_dir() {
            collect(&source, &mut files);
        }
        for name in ["Cargo.toml", "build.rs"] {
            let path = directory.join(name);
            if path.is_file() {
                println!("cargo:rerun-if-changed={}", path.display());
                files.push(path);
            }
        }
    }
    for name in ["Cargo.toml", "Cargo.lock", "rust-toolchain.toml"] {
        let path = workspace.join(name);
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.display());
            files.push(path);
        }
    }
    files.sort();
    let mut hash = Sha256::new();
    for path in files {
        let name = path.strip_prefix(workspace).unwrap().to_string_lossy();
        let content = fs::read(&path).expect("compiler input file is readable");
        hash.update((name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
        hash.update((content.len() as u64).to_le_bytes());
        hash.update(content);
    }
    for name in ["TARGET", "PROFILE", "CARGO_ENCODED_RUSTFLAGS"] {
        println!("cargo:rerun-if-env-changed={name}");
        hash.update(name.as_bytes());
        hash.update(env::var(name).unwrap_or_default().as_bytes());
    }
    println!(
        "cargo:rustc-env=TERRANE_COMPILER_CACHE_ID={:x}",
        hash.finalize()
    );
}
