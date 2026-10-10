use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const FORMAT: u32 = 1;
static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

#[derive(Deserialize, Serialize)]
struct CacheFile {
    format: u32,
    compiler: String,
    context: String,
    checksum: String,
    entries: BTreeMap<String, Option<super::ValueType>>,
}

pub(super) struct NativeCache {
    path: PathBuf,
    context: String,
    entries: BTreeMap<String, Option<super::ValueType>>,
    hits: usize,
    dirty: bool,
}

impl NativeCache {
    pub(super) fn load(root: &Path, context: String) -> Self {
        let path = root.join(".trn/cache/native-reconciliation.json");
        let entries = fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<CacheFile>(&bytes).ok())
            .filter(|cache| {
                cache.format == FORMAT
                    && cache.compiler == crate::cache_identity::COMPILER
                    && cache.context == context
                    && query_key(&cache.entries) == cache.checksum
            })
            .map_or_else(BTreeMap::new, |cache| cache.entries);
        Self {
            path,
            context,
            entries,
            hits: 0,
            dirty: false,
        }
    }
    pub(super) fn get(&mut self, key: &str) -> Option<Option<super::ValueType>> {
        self.entries.get(key).map(|value| {
            self.hits += 1;
            value.clone()
        })
    }
    pub(super) fn insert(&mut self, key: String, value: Option<super::ValueType>) {
        self.entries.insert(key, value);
        self.dirty = true;
    }
    pub(super) fn finish(self) {
        if self.hits > 0 {
            crate::compilation_progress::note(
                "reused native semantic queries",
                format_args!("{} cache hits", self.hits),
            );
        }
        if !self.dirty {
            return;
        }
        let checksum = query_key(&self.entries);
        let cache = CacheFile {
            format: FORMAT,
            compiler: crate::cache_identity::COMPILER.to_owned(),
            context: self.context,
            checksum,
            entries: self.entries,
        };
        let Ok(bytes) = serde_json::to_vec(&cache) else {
            return;
        };
        let _ = persist(&self.path, &bytes);
    }
}

struct HashWriter(Sha256);
impl Write for HashWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.update(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(super) fn query_key<T: Serialize>(query: &T) -> String {
    let mut writer = HashWriter(Sha256::new());
    // These queries consist only of compiler-owned strings, integers, maps and enums.
    serde_json::to_writer(&mut writer, query).expect("native semantic query is serializable");
    format!("{:x}", writer.0.finalize())
}

fn persist(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("native cache has no parent"))?;
    fs::create_dir_all(parent)?;
    let temporary = path.with_extension(format!(
        "{}-{}.tmp",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    let result = (|| {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(bytes)?;
        drop(file);
        fs::rename(&temporary, path)
    })();
    let _ = fs::remove_file(temporary);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn changed_projection_or_damaged_snapshot_cannot_supply_old_selections() {
        let root = std::env::temp_dir().join(format!(
            "native-cache-{}-{}",
            std::process::id(),
            NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        let key = query_key(&("signature", "NativePayload"));
        let mut cache = NativeCache::load(&root, "graph-one".to_owned());
        cache.insert(
            key.clone(),
            Some(super::super::ValueType::Scalar(crate::ScalarType::Int)),
        );
        cache.finish();
        let mut different_graph = NativeCache::load(&root, "graph-two".to_owned());
        assert_eq!(different_graph.get(&key), None);
        let path = root.join(".trn/cache/native-reconciliation.json");
        let original = fs::read(&path).unwrap();
        let mut record: serde_json::Value = serde_json::from_slice(&original).unwrap();
        record["entries"][&key] = serde_json::Value::Null;
        fs::write(&path, serde_json::to_vec(&record).unwrap()).unwrap();
        let mut damaged = NativeCache::load(&root, "graph-one".to_owned());
        assert_eq!(damaged.get(&key), None);
        fs::write(&path, b"malformed").unwrap();
        let mut malformed = NativeCache::load(&root, "graph-one".to_owned());
        assert_eq!(malformed.get(&key), None);
        fs::remove_dir_all(root).unwrap();
    }
}
