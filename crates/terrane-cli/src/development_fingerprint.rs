use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

pub fn compiler_input_paths(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut paths = vec![root.join("Cargo.toml"), root.join("Cargo.lock")];
    collect_compiler_inputs(&root.join("crates"), &mut paths)?;
    paths.sort_unstable();
    Ok(paths)
}

pub fn compiler_input_fingerprint(root: &Path) -> io::Result<u64> {
    let paths = compiler_input_paths(root)?;
    let mut hash = FNV_OFFSET;
    for path in paths {
        let relative = path.strip_prefix(root).unwrap_or(&path);
        hash_bytes(&mut hash, relative.as_os_str().as_encoded_bytes());
        hash_bytes(&mut hash, &[0]);
        hash_bytes(&mut hash, &fs::read(&path)?);
        hash_bytes(&mut hash, &[0]);
    }
    Ok(hash)
}

fn collect_compiler_inputs(directory: &Path, paths: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_compiler_inputs(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "rs")
            || path.file_name().is_some_and(|name| name == "Cargo.toml")
        {
            paths.push(path);
        }
    }
    Ok(())
}

fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(FNV_PRIME);
    }
}

#[cfg(test)]
mod tests {
    use super::compiler_input_fingerprint;
    use std::fs;

    #[test]
    fn fingerprint_tracks_compiler_inputs_and_ignores_documentation() {
        let root = std::env::temp_dir().join(format!(
            "terrane-development-fingerprint-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("crates/example/src")).unwrap();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
        fs::write(root.join("Cargo.lock"), "").unwrap();
        fs::write(
            root.join("crates/example/Cargo.toml"),
            "[package]\nname = \"example\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::write(
            root.join("crates/example/src/lib.rs"),
            "pub fn value() {}\n",
        )
        .unwrap();
        fs::write(root.join("docs/guide.md"), "first\n").unwrap();

        let original = compiler_input_fingerprint(&root).unwrap();
        fs::write(root.join("docs/guide.md"), "second\n").unwrap();
        assert_eq!(compiler_input_fingerprint(&root).unwrap(), original);
        fs::write(
            root.join("crates/example/src/lib.rs"),
            "pub fn changed() {}\n",
        )
        .unwrap();
        assert_ne!(compiler_input_fingerprint(&root).unwrap(), original);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn fingerprint_reports_unreadable_input_roots() {
        let root = std::env::temp_dir().join(format!(
            "terrane-missing-development-fingerprint-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        assert!(compiler_input_fingerprint(&root).is_err());
    }
}
