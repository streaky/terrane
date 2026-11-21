//! Persistent results for a single typed-binding collection round.
use std::collections::BTreeMap;
use std::io::{BufWriter, Write as _};
use std::path::PathBuf;

use serde::{Serialize, Serializer};

use super::enums::{MatchContext, ResolvedEnum};
use super::model::{SemanticFailure, SemanticUnit, TypedBinding};
use crate::{cache_io::HashWriter, compiler::cache};

pub(super) struct BindingTickets {
    directory: PathBuf,
    enum_key: String,
}

impl BindingTickets {
    pub(super) fn open(enums: &[ResolvedEnum]) -> Option<Self> {
        if !crate::cache_scope::enabled() {
            return None;
        }
        let directory = std::env::var_os("TERRANE_INFERENCE_CACHE")
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)?;
        Some(Self {
            directory,
            enum_key: digest(enums, crate::cache_identity::COMPILER)?,
        })
    }

    pub(super) fn collect(
        &self,
        unit: &SemanticUnit,
        matches: &MatchContext<'_>,
        infer: impl FnOnce() -> Result<Vec<TypedBinding>, SemanticFailure>,
    ) -> Result<(Vec<TypedBinding>, bool), SemanticFailure> {
        // All context the collector can observe is confined to these two
        // inputs. Preserve source IDs rather than partially rebinding them:
        // both the input and inferred types can carry file-qualified spans.
        let Some(key) = digest(&(unit, matches.names()), &self.enum_key) else {
            return infer().map(|bindings| (bindings, false));
        };
        let path = self.directory.join(&key[..2]).join(format!("{key}.json"));
        // The caller may be a scoped compiler worker; cache scope is local to
        // each thread. `open` has already checked the parent thread's opt-in.
        crate::with_compilation_cache(|| {
            if let Some(bindings) = cache::load_file(&path, &key) {
                return Ok((bindings, true));
            }
            let bindings = infer()?;
            cache::store_file(&path, &key, &bindings);
            Ok((bindings, false))
        })
    }
}

fn digest(value: &(impl Serialize + ?Sized), context: &str) -> Option<String> {
    let mut hash = HashWriter::new();
    hash.part(context.as_bytes());
    let mut stream = BufWriter::new(&mut hash);
    serde_json::to_writer(&mut stream, value).ok()?;
    stream.flush().ok()?;
    drop(stream);
    Some(hash.finish_hex())
}

// JSON object keys cannot represent span tuples. Hash the sorted entries,
// preserving both the complete keys and any duplicates in nested values.
pub(super) fn ordered_map<K: Serialize, V: Serialize, S: Serializer>(
    value: &BTreeMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(value)
}

pub(super) fn nested_ordered_map<K: Serialize, V: Serialize, S: Serializer>(
    value: &BTreeMap<K, BTreeMap<K, V>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    struct Entries<'a, K, V>(&'a BTreeMap<K, V>);
    impl<K: Serialize, V: Serialize> Serialize for Entries<'_, K, V> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            ordered_map(self.0, serializer)
        }
    }
    serializer.collect_seq(value.iter().map(|(key, values)| (key, Entries(values))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Package, ScalarType, ValueType, syntax::SyntaxKind};
    use std::sync::atomic::{AtomicU64, Ordering};

    #[test]
    fn changed_type_selection_reinfers_the_same_source_body() {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        struct Scratch(PathBuf);
        impl Drop for Scratch {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
        let root = Scratch(std::env::temp_dir().join(format!(
            "terrane-binding-tickets-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        )));
        let source = Package::implicit(
            root.0.join("body.trn"),
            "namespace example\nfunction main;\n    value int = 1\n    print; value\n".to_owned(),
        );
        let package = super::super::analysis::analyze(&source).unwrap();
        let enums = super::super::enums::resolve_enums(&package);
        let tickets = BindingTickets {
            directory: root.0.join("tickets"),
            enum_key: digest(&enums, crate::cache_identity::COMPILER).unwrap(),
        };
        let mut unit = package.units[0].clone();
        let infer = |unit: &SemanticUnit| {
            let matches = MatchContext::new(&package, unit, &enums);
            tickets
                .collect(unit, &matches, || {
                    let mut bindings = Vec::new();
                    super::super::contracts::collect_typed_bindings(
                        unit,
                        &matches,
                        &unit.tree.root,
                        &mut Vec::new(),
                        &mut bindings,
                        None,
                    )?;
                    Ok(bindings)
                })
                .unwrap()
        };
        let annotation = unit.tree.root.children[1]
            .children
            .iter()
            .find(|node| node.kind == SyntaxKind::Block)
            .unwrap()
            .children[0]
            .children
            .iter()
            .find(|node| node.kind == SyntaxKind::TypeExpression)
            .unwrap()
            .span;
        let answer = |bindings: &[TypedBinding]| {
            bindings
                .iter()
                .find(|binding| binding.name == "value")
                .unwrap()
                .value_type
                .clone()
        };
        crate::with_compilation_cache(|| {
            let (original, _) = infer(&unit);
            assert_eq!(answer(&original), ValueType::Scalar(ScalarType::Int));
            let (loaded, hit) = infer(&unit);
            assert!(hit);
            assert_eq!(original, loaded);
            unit.selected_expression_types.insert(
                (annotation.file, annotation.start, annotation.end),
                ValueType::Scalar(ScalarType::Uint8),
            );
            let (changed, _) = infer(&unit);
            assert_eq!(answer(&changed), ValueType::Scalar(ScalarType::Uint8));
        });
    }
}
