use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

const FORMAT: u32 = 1;
#[derive(Deserialize, Serialize)]
struct CacheFile {
    format: u32,
    compiler: String,
    context: String,
    checksum: String,
    entries: BTreeMap<String, Option<super::ValueType>>,
}

#[expect(
    clippy::large_enum_variant,
    reason = "Inline cached types avoid an extra allocation for every selected answer"
)]
#[derive(Clone)]
pub(super) enum CachedAnswer {
    Selected(super::ValueType),
    Unavailable,
}

pub(super) struct NativeCache {
    path: Option<PathBuf>,
    context: String,
    entries: BTreeMap<String, CachedAnswer>,
    queried: BTreeSet<String>,
    hits: usize,
    dirty: bool,
    enabled: bool,
}

impl NativeCache {
    pub(super) fn load(root: &Path, context: String) -> Self {
        let enabled = crate::cache_scope::enabled();
        let path = enabled.then(|| root.join(".trn/cache/native-reconciliation.json"));
        let entries = if let Some(path) = &path {
            fs::read(path)
                .ok()
                .and_then(|bytes| serde_json::from_slice::<CacheFile>(&bytes).ok())
                .filter(|cache| {
                    cache.format == FORMAT
                        && cache.compiler == crate::cache_identity::COMPILER
                        && cache.context == context
                        && query_key(&cache.entries) == cache.checksum
                })
                .map_or_else(BTreeMap::new, |cache| {
                    cache
                        .entries
                        .into_iter()
                        .map(|(key, value)| (key, answer(value)))
                        .collect()
                })
        } else {
            BTreeMap::new()
        };
        Self {
            path,
            context,
            entries,
            queried: BTreeSet::new(),
            hits: 0,
            dirty: false,
            enabled,
        }
    }

    pub(super) fn get(&mut self, key: &str) -> Option<CachedAnswer> {
        if !self.enabled {
            return None;
        }
        self.entries.get(key).cloned().inspect(|_| {
            self.hits += 1;
            self.queried.insert(key.to_owned());
        })
    }

    pub(super) fn insert(&mut self, key: String, answer: CachedAnswer) {
        if self.enabled {
            self.queried.insert(key.clone());
            self.entries.insert(key, answer);
            self.dirty = true;
        }
    }

    pub(super) fn finish(self) {
        if self.hits > 0 {
            crate::compilation_progress::note(
                "reused native semantic queries",
                format_args!("{} cache hits", self.hits),
            );
        }
        if !self.enabled || (!self.dirty && self.entries.len() == self.queried.len()) {
            return;
        }
        let entries = self
            .entries
            .into_iter()
            .filter(|(key, _)| self.queried.contains(key))
            .map(|(key, answer)| (key, stored_answer(answer)))
            .collect::<BTreeMap<_, _>>();
        let checksum = query_key(&entries);
        let cache = CacheFile {
            format: FORMAT,
            compiler: crate::cache_identity::COMPILER.to_owned(),
            context: self.context,
            checksum,
            entries,
        };
        if let (Ok(bytes), Some(path)) = (serde_json::to_vec(&cache), self.path.as_deref()) {
            crate::cache_io::atomic_write(path, &bytes);
        }
    }
}

fn answer(value: Option<super::ValueType>) -> CachedAnswer {
    value.map_or(CachedAnswer::Unavailable, CachedAnswer::Selected)
}

fn stored_answer(answer: CachedAnswer) -> Option<super::ValueType> {
    match answer {
        CachedAnswer::Selected(value) => Some(value),
        CachedAnswer::Unavailable => None,
    }
}

pub(super) fn query_key<T: Serialize>(query: &T) -> String {
    let mut writer = crate::cache_io::HashWriter::new();
    serde_json::to_writer(&mut writer, query).expect("native semantic query is serializable");
    writer.finish_hex()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Root(PathBuf);
    impl Root {
        fn new() -> Self {
            Self(std::env::temp_dir().join(format!(
                "native-cache-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            )))
        }
    }
    impl Drop for Root {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn changed_projection_or_damaged_snapshot_cannot_supply_old_selections() {
        crate::with_compilation_cache(|| {
            let root = Root::new();
            let key = query_key(&("signature", "NativePayload"));
            let mut cache = NativeCache::load(&root.0, "graph-one".to_owned());
            cache.insert(
                key.clone(),
                CachedAnswer::Selected(super::super::ValueType::Scalar(crate::ScalarType::Int)),
            );
            cache.finish();
            let mut warm = NativeCache::load(&root.0, "graph-one".to_owned());
            assert!(matches!(
                warm.get(&key),
                Some(CachedAnswer::Selected(super::super::ValueType::Scalar(
                    crate::ScalarType::Int
                )))
            ));
            let mut different_graph = NativeCache::load(&root.0, "graph-two".to_owned());
            assert!(different_graph.get(&key).is_none());
            let path = root.0.join(".trn/cache/native-reconciliation.json");
            let mut record: serde_json::Value =
                serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            record["entries"][&key] = serde_json::Value::Null;
            fs::write(&path, serde_json::to_vec(&record).unwrap()).unwrap();
            let mut damaged = NativeCache::load(&root.0, "graph-one".to_owned());
            assert!(damaged.get(&key).is_none());
            fs::write(&path, b"malformed").unwrap();
            let mut malformed = NativeCache::load(&root.0, "graph-one".to_owned());
            assert!(malformed.get(&key).is_none());
        });
    }

    #[test]
    fn unavailable_answers_reuse_and_unused_queries_are_pruned() {
        crate::with_compilation_cache(|| {
            let root = Root::new();
            let mut cache = NativeCache::load(&root.0, "graph".to_owned());
            cache.insert("used".to_owned(), CachedAnswer::Unavailable);
            cache.insert(
                "obsolete".to_owned(),
                CachedAnswer::Selected(super::super::ValueType::Scalar(crate::ScalarType::Int)),
            );
            cache.finish();
            let mut warm = NativeCache::load(&root.0, "graph".to_owned());
            assert!(matches!(warm.get("used"), Some(CachedAnswer::Unavailable)));
            warm.finish();
            let mut pruned = NativeCache::load(&root.0, "graph".to_owned());
            assert!(matches!(
                pruned.get("used"),
                Some(CachedAnswer::Unavailable)
            ));
            assert!(pruned.get("obsolete").is_none());
        });
    }
}
