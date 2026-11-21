//! Cache the checked assignment traversal, not the inference that feeds it.
use std::path::{Path, PathBuf};

use super::prelude::*;
use super::scope_validation::{AssignmentCheck, validate_assignment_block};
use crate::cache_io::HashWriter;
use crate::compiler::cache;

#[derive(Default)]
pub(super) struct AssignmentTickets {
    path: Option<(PathBuf, String)>,
    saved: BTreeMap<String, Vec<Read>>,
    learned: BTreeMap<String, Vec<Read>>,
}

// Absolute offsets in the exact source text, never a composition-local file ID.
#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
struct Read {
    start: usize,
    end: usize,
}

#[derive(Default)]
pub(super) struct TicketCounts {
    pub shared: usize,
    pub local: usize,
    pub checked: usize,
}

impl TicketCounts {
    pub(super) fn report(&self, started: std::time::Instant) {
        crate::compilation_progress::note(
            "assignment body tickets",
            format_args!(
                "{} persistent hits; {} in-pass hits; {} checked ({:.3}s including reaching flow)",
                self.shared,
                self.local,
                self.checked,
                started.elapsed().as_secs_f64(),
            ),
        );
    }
}

pub(super) fn directory() -> Option<PathBuf> {
    if !crate::cache_scope::enabled() {
        return None;
    }
    std::env::var_os("TERRANE_DEMAND_CACHE")
        .filter(|path| !path.is_empty())
        .map(|path| PathBuf::from(path).join("assignment"))
}

impl AssignmentTickets {
    pub(super) fn load(directory: Option<&Path>, source: &SourceFile) -> Self {
        let Some(directory) = directory else {
            return Self::default();
        };
        let mut hash = HashWriter::new();
        hash.part(crate::cache_identity::COMPILER.as_bytes());
        hash.part(source.text().as_bytes());
        let key = hash.finish_hex();
        let path = directory.join(&key[..2]).join(format!("{key}.json"));
        let saved = cache::load_file(&path, &key).unwrap_or_default();
        Self {
            path: Some((path, key)),
            saved,
            learned: BTreeMap::new(),
        }
    }

    pub(super) fn validate(
        &mut self,
        unit: &SemanticUnit,
        block: &SyntaxNode,
        mut declared: BTreeSet<String>,
        mut assigned: BTreeSet<String>,
        counts: &mut TicketCounts,
    ) -> Result<(), SemanticFailure> {
        let key = self.path.as_ref().map(|_| {
            let mut hash = HashWriter::new();
            hash.part(&block.span.start.to_le_bytes());
            hash.part(&block.span.end.to_le_bytes());
            for names in [&declared, &assigned] {
                hash.part(&names.len().to_le_bytes());
                for name in names {
                    hash.part(name.as_bytes());
                }
            }
            hash.finish_hex()
        });
        if let Some(key) = &key {
            // Only the exceptional reads depend on the live scope/flow facts.
            // Recheck them in traversal order before trusting a cached success.
            let valid = |reads: &[Read]| reads.iter().all(|read| read.is_allowed(unit));
            if self.learned.get(key).is_some_and(|reads| valid(reads)) {
                counts.local += 1;
                return Ok(());
            }
            if self.saved.get(key).is_some_and(|reads| valid(reads)) {
                counts.shared += 1;
                return Ok(());
            }
        }
        let mut reads = Vec::new();
        let mut check_read = |node: &SyntaxNode| {
            let read = Read {
                start: node.span.start,
                end: node.span.end,
            };
            if !read.is_allowed(unit) {
                let name = node_text(&unit.source, node);
                return Err(failure(
                    &unit.source,
                    "T0007",
                    format!("`{name}` may be read before it is assigned"),
                    node.span,
                ));
            }
            if key.is_some() {
                reads.push(read);
            }
            Ok(())
        };
        validate_assignment_block(
            &mut AssignmentCheck {
                unit,
                check_read: &mut check_read,
            },
            block,
            &mut declared,
            &mut assigned,
        )?;
        counts.checked += 1;
        if let Some(key) = key {
            self.learned.insert(key, reads);
        }
        Ok(())
    }

    pub(super) fn finish(mut self) {
        if !self.learned.is_empty()
            && let Some((path, key)) = self.path
        {
            self.saved.append(&mut self.learned);
            cache::store_file(&path, &key, &self.saved);
        }
    }
}

impl Read {
    fn is_allowed(self, unit: &SemanticUnit) -> bool {
        let Some(name) = unit.source.text().get(self.start..self.end) else {
            return false;
        };
        lexical_scope_chain(unit, self.start)
            .find_map(|scope| {
                scope.symbols.get(name)?.iter().rev().find(|symbol| {
                    symbol
                        .binding_span
                        .is_none_or(|span| span.end <= self.start)
                })
            })
            .is_some_and(|symbol| symbol.kind == SymbolKind::Function || symbol.global)
            || unit
                .flow_availability
                .contains_key(&(unit.source.id(), self.start, self.end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_read_is_rechecked_in_the_consumers_flow_context() {
        let text = "namespace example\nfunction probe;\n    return value\n";
        let unit = |id| {
            let source = SourceFile::new(id, PathBuf::from("body.trn"), text.to_owned());
            super::super::analysis::parse_unit(
                &source,
                "body.trn".to_owned(),
                None,
                false,
                false,
                crate::SourceRole::Production,
            )
            .unwrap()
        };
        let block =
            |unit: &SemanticUnit| unit.tree.root.children[1].children.last().unwrap().clone();
        let mut producer = unit(17);
        let start = text.find("value").unwrap();
        producer
            .flow_availability
            .insert((17, start, start + 5), FlowAvailability::DefinitelyAssigned);
        let mut tickets = AssignmentTickets {
            path: Some((PathBuf::new(), String::new())),
            ..AssignmentTickets::default()
        };
        let mut counts = TicketCounts::default();
        let declared = || BTreeSet::from(["value".to_owned()]);
        tickets
            .validate(
                &producer,
                &block(&producer),
                declared(),
                BTreeSet::new(),
                &mut counts,
            )
            .unwrap();

        tickets.saved = std::mem::take(&mut tickets.learned);
        let mut consumer = unit(29);
        let block = block(&consumer);
        consumer
            .flow_availability
            .insert((29, start, start + 5), FlowAvailability::DefinitelyAssigned);
        tickets
            .validate(&consumer, &block, declared(), BTreeSet::new(), &mut counts)
            .unwrap();
        assert_eq!(counts.shared, 1);

        consumer.flow_availability.clear();
        let failure = tickets
            .validate(&consumer, &block, declared(), BTreeSet::new(), &mut counts)
            .unwrap_err();
        assert_eq!(failure.diagnostics[0].code, "T0007");
        assert_eq!(
            failure.diagnostics[0].primary,
            Some(Span::new(29, start, start + 5))
        );
    }
}
