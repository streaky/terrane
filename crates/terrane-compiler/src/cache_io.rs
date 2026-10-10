//! Shared streaming hashes and durable cache publication helpers.
use std::fs;
use std::io::{self, Write as _};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

/// SHA-256 stream with an explicit length-prefix operation for structured components.
pub(crate) struct HashWriter(Sha256);

impl HashWriter {
    pub(crate) fn new() -> Self {
        Self(Sha256::new())
    }

    pub(crate) fn part(&mut self, bytes: &[u8]) {
        self.0.update((bytes.len() as u64).to_le_bytes());
        self.0.update(bytes);
    }

    pub(crate) fn finish_hex(self) -> String {
        format!("{:x}", self.0.finalize())
    }
}

impl io::Write for HashWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.update(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Atomically publish a complete file through a unique sibling temporary file.
pub(crate) fn atomic_write(path: &Path, bytes: &[u8]) {
    let Some(parent) = path.parent() else {
        return;
    };
    if fs::create_dir_all(parent).is_err() {
        return;
    }
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let temp = parent.join(format!(
        ".{name}-{}-{}.tmp",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    if let Ok(mut file) = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
    {
        let written = file.write_all(bytes).is_ok();
        drop(file);
        if written {
            let _ = fs::rename(&temp, path);
        }
        let _ = fs::remove_file(&temp);
    }
}
