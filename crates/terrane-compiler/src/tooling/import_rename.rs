use super::{
    ParsedDocument, ProtocolError, PublicSpan, Replacement, Snapshot, node_text,
    resolve_semantic_target, smallest_node_at,
};
use crate::semantics::{SemanticUnit, Symbol};
use crate::syntax::{SyntaxKind, SyntaxNode};

#[cfg(test)]
use super::{SnapshotMetadata, SnapshotOptions, SourceInput, ToolingEngine};

struct AliasBinding<'a> {
    import: &'a SyntaxNode,
    export: &'a SyntaxNode,
    clause: &'a SyntaxNode,
    alias: &'a SyntaxNode,
    symbol: &'a Symbol,
}

impl AliasBinding<'_> {
    fn would_capture(&self, candidate: &Symbol, remove_alias: bool) -> bool {
        candidate.identity != self.symbol.identity
            || (!remove_alias && candidate.binding_span != Some(self.import.span))
    }
}

/// An existing selective-import alias has its own local rename boundary.
pub(super) fn propose(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    offset: usize,
    new_name: &str,
) -> Result<Option<Vec<Replacement>>, ProtocolError> {
    let Some(semantic) = &snapshot.semantic else {
        return Ok(None);
    };
    let Some(unit) = semantic
        .units
        .iter()
        .find(|unit| unit.source.id() == document.source.id())
    else {
        return Ok(None);
    };
    let Some(import) = selected_import(snapshot, document, unit, offset) else {
        return Ok(None);
    };
    let Some(binding) = alias_binding(document, unit, import) else {
        return Ok(None);
    };
    // Cursor boundaries follow smallest_node_at: the end of a token still selects it.
    if binding.export.span.start <= offset && offset <= binding.export.span.end {
        return Err(ProtocolError::new(
            "import-export-rename",
            "the exported name of an aliased import cannot be renamed here; select the alias to rename its local binding, or the exporting declaration to rename the export",
        ));
    }
    let remove_alias = new_name == node_text(&document.source, binding.export);
    if semantic
        .resolve_name_at(unit, binding.export.span.start, new_name)
        .is_some_and(|candidate| binding.would_capture(candidate, remove_alias))
    {
        return Err(capture_error(new_name));
    }
    let uri = snapshot.uri_for_file(unit.source.id()).ok_or_else(|| {
        ProtocolError::new("unknown-source", "rename source left the semantic snapshot")
    })?;
    let mut replacements = vec![clause_replacement(document, &binding, new_name, uri)?];
    collect_uses(
        snapshot,
        document,
        unit,
        &binding,
        new_name,
        uri,
        &mut replacements,
    )?;
    Ok(Some(replacements))
}

fn selected_import<'a>(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    unit: &'a SemanticUnit,
    offset: usize,
) -> Option<&'a SyntaxNode> {
    if let Some(import) = find_import(&unit.tree.root, &|node| {
        node.span.start <= offset && offset <= node.span.end
    }) {
        return Some(import);
    }
    let (node, _, _) = smallest_node_at(&document.tree.root, offset, 0)?;
    if node.kind != SyntaxKind::Name {
        return None;
    }
    let semantic = snapshot.semantic.as_ref()?;
    let symbol =
        semantic.resolve_name_at(unit, node.span.start, node_text(&document.source, node))?;
    if resolve_semantic_target(snapshot, document, node)?.identity != symbol.identity {
        return None;
    }
    let binding_span = symbol.binding_span?;
    find_import(&unit.tree.root, &|node| node.span == binding_span)
}

fn find_import<'a>(
    node: &'a SyntaxNode,
    matches: &impl Fn(&SyntaxNode) -> bool,
) -> Option<&'a SyntaxNode> {
    if node.kind == SyntaxKind::ObjectImport && matches(node) {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_import(child, matches))
}

fn alias_binding<'a>(
    document: &ParsedDocument,
    unit: &'a SemanticUnit,
    import: &'a SyntaxNode,
) -> Option<AliasBinding<'a>> {
    let export = import.children.first()?;
    let clause = import
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::ImportAlias)?;
    let alias = clause.children.first()?;
    let name = node_text(&document.source, alias);
    let symbol = unit
        .scopes
        .iter()
        .filter_map(|scope| scope.symbols.get(name))
        .flatten()
        .find(|symbol| symbol.binding_span == Some(import.span))?;
    Some(AliasBinding {
        import,
        export,
        clause,
        alias,
        symbol,
    })
}

/// The source token of a selective import names the export, not its local alias.
pub(super) fn export_target(
    semantic: &crate::SemanticPackage,
    unit: &SemanticUnit,
    document: &ParsedDocument,
    import: &SyntaxNode,
) -> Option<super::SemanticTarget> {
    let export = import.children.first()?;
    let local = import
        .children
        .iter()
        .find(|node| node.kind == SyntaxKind::ImportAlias)
        .and_then(|clause| clause.children.first())
        .unwrap_or(export);
    let name = node_text(&document.source, local);
    let symbol = unit
        .scopes
        .iter()
        .filter_map(|scope| scope.symbols.get(name))
        .flatten()
        .find(|symbol| symbol.binding_span == Some(import.span))
        .or_else(|| semantic.resolve_name_at(unit, export.span.start, name))?;
    Some(super::symbol_target(semantic, unit, symbol))
}

pub(super) fn canonical_import_removals(
    document: &ParsedDocument,
    unit: &SemanticUnit,
    identities: &std::collections::BTreeSet<String>,
    new_name: &str,
    uri: &str,
    replacements: &mut Vec<Replacement>,
) -> Result<(), ProtocolError> {
    let mut result = Ok(());
    super::walk_nodes(&unit.tree.root, None, &mut |node, _, _| {
        if node.kind != SyntaxKind::ObjectImport || result.is_err() {
            return;
        }
        let Some(binding) = alias_binding(document, unit, node) else {
            return;
        };
        if identities.contains(&binding.symbol.identity)
            && node_text(&document.source, binding.alias) == new_name
        {
            match remove_alias_clause(document, &binding, uri) {
                Ok(replacement) => replacements.push(replacement),
                Err(error) => result = Err(error),
            }
        }
    });
    result
}

fn clause_replacement(
    document: &ParsedDocument,
    binding: &AliasBinding<'_>,
    new_name: &str,
    uri: &str,
) -> Result<Replacement, ProtocolError> {
    if new_name != node_text(&document.source, binding.export) {
        return Ok(Replacement {
            uri: uri.to_owned(),
            span: binding.alias.span.into(),
            text: new_name.to_owned(),
        });
    }
    remove_alias_clause(document, binding, uri)
}

fn remove_alias_clause(
    document: &ParsedDocument,
    binding: &AliasBinding<'_>,
    uri: &str,
) -> Result<Replacement, ProtocolError> {
    let keyword = document
        .tree
        .lexed
        .tokens
        .get(binding.clause.token_range.clone())
        .and_then(|tokens| {
            tokens
                .iter()
                .find(|token| token.text == "as" && token.span.end <= binding.alias.span.start)
        })
        .ok_or_else(|| {
            ProtocolError::new(
                "invalid-import-alias",
                "import alias has no `as` keyword token",
            )
        })?;
    // Keep trivia around the actual keyword rather than assuming a clause byte layout.
    let before_as = &document.source.text()[binding.export.span.end..keyword.span.start];
    let after_as = &document.source.text()[keyword.span.end..binding.alias.span.start];
    Ok(Replacement {
        uri: uri.to_owned(),
        span: PublicSpan {
            start: binding.export.span.end,
            end: binding.alias.span.end,
        },
        text: format!("{}{}", before_as.trim_end(), after_as.trim_end()),
    })
}

fn collect_uses(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    unit: &SemanticUnit,
    binding: &AliasBinding<'_>,
    new_name: &str,
    uri: &str,
    replacements: &mut Vec<Replacement>,
) -> Result<(), ProtocolError> {
    let semantic = snapshot.semantic.as_ref().ok_or_else(|| {
        ProtocolError::new(
            "semantic-unavailable",
            "rename requires a complete semantic snapshot",
        )
    })?;
    let local_name = node_text(&document.source, binding.alias);
    let remove_alias = new_name == node_text(&document.source, binding.export);
    let mut capture = false;
    walk_uses(&unit.tree.root, &mut |node| {
        if node.kind != SyntaxKind::Name {
            return;
        }
        let name = node_text(&document.source, node);
        if name != local_name && name != new_name {
            return;
        }
        let resolves_binding = semantic
            .resolve_name_at(unit, node.span.start, local_name)
            .is_some_and(|candidate| {
                candidate.identity == binding.symbol.identity
                    && candidate.binding_span == Some(binding.import.span)
            });
        if !resolves_binding {
            return;
        }
        let target = resolve_semantic_target(snapshot, document, node);
        let new_symbol = semantic.resolve_name_at(unit, node.span.start, new_name);
        if name == local_name
            && target
                .as_ref()
                .is_some_and(|target| target.identity == binding.symbol.identity)
        {
            capture |=
                new_symbol.is_some_and(|candidate| binding.would_capture(candidate, remove_alias));
            replacements.push(Replacement {
                uri: uri.to_owned(),
                span: node.span.into(),
                text: new_name.to_owned(),
            });
        } else if name == new_name
            && let Some(candidate) = new_symbol
            && (!remove_alias || candidate.identity != binding.symbol.identity)
            && target.is_some_and(|target| target.identity == candidate.identity)
        {
            capture = true;
        }
    });
    if capture {
        Err(capture_error(new_name))
    } else {
        Ok(())
    }
}

fn capture_error(new_name: &str) -> ProtocolError {
    ProtocolError::new(
        "rename-capture",
        format!("`{new_name}` would capture an existing symbol"),
    )
}

fn walk_uses(node: &SyntaxNode, visit: &mut impl FnMut(&SyntaxNode)) {
    if node.kind == SyntaxKind::ImportDeclaration {
        return;
    }
    visit(node);
    for child in &node.children {
        walk_uses(child, visit);
    }
}

#[cfg(test)]
mod projected_tests;

#[cfg(test)]
mod tests {
    use super::*;

    fn open(
        engine: &mut ToolingEngine,
        uri: &str,
        text: &str,
        options: SnapshotOptions,
    ) -> SnapshotMetadata {
        engine
            .open_snapshot(
                vec![SourceInput {
                    uri: uri.to_owned(),
                    text: text.to_owned(),
                }],
                None,
                None,
                options,
            )
            .expect("snapshot")
    }
    #[test]
    fn import_alias_rename_preserves_exports_other_aliases_and_shadowed_uses() {
        let exporter =
            "namespace library\npublic function Answer int;\n    return 42\npublic class Model\n";
        let uri = "file:///workspace/consumer.trn";
        for (export, use_site) in [("Answer", "_ = NAME;"), ("Model", "value NAME|none = none")] {
            let name = "local-name";
            let import = format!("{export} as {name}");
            for nested in [false, true] {
                let prefix = if nested { "function main;\n    " } else { "" };
                let body = if nested { "" } else { "function main;\n" };
                let shadow = if nested {
                    format!("function shadow; {name} int\n    _ = {name}\n")
                } else {
                    String::new()
                };
                let source = format!(
                    "namespace consumer\n{prefix}from /library import {import}, {export} as other-name\n{body}    {}\n    {}\n{shadow}",
                    use_site.replace("NAME", name),
                    use_site
                        .replace("NAME", "other-name")
                        .replace("value ", "other "),
                );
                for new_name in ["renamed", export] {
                    let renamed_import = if new_name == export {
                        export.to_owned()
                    } else {
                        format!("{export} as {new_name}")
                    };
                    let expected = format!(
                        "namespace consumer\n{prefix}from /library import {renamed_import}, {export} as other-name\n{body}    {}\n    {}\n{shadow}",
                        use_site.replace("NAME", new_name),
                        use_site
                            .replace("NAME", "other-name")
                            .replace("value ", "other "),
                    );
                    let other = format!(
                        "namespace other\nfrom /library import {export}\nfunction main;\n    {}\n",
                        use_site.replace("NAME", export),
                    );
                    for offset in [
                        source.find(name).unwrap(),
                        source.find(name).unwrap() + name.len(),
                        source.find(&use_site.replace("NAME", name)).unwrap()
                            + use_site.find("NAME").unwrap(),
                        source.find(&use_site.replace("NAME", name)).unwrap()
                            + use_site.find("NAME").unwrap()
                            + name.len(),
                    ] {
                        let mut engine = ToolingEngine::default();
                        let snapshot = engine
                            .open_snapshot(
                                vec![
                                    SourceInput {
                                        uri: uri.to_owned(),
                                        text: source.clone(),
                                    },
                                    SourceInput {
                                        uri: "file:///workspace/library.trn".to_owned(),
                                        text: exporter.to_owned(),
                                    },
                                    SourceInput {
                                        uri: "file:///workspace/other.trn".to_owned(),
                                        text: other.clone(),
                                    },
                                ],
                                None,
                                None,
                                SnapshotOptions {
                                    semantic: true,
                                    ..SnapshotOptions::default()
                                },
                            )
                            .unwrap();
                        let proposal = engine
                            .propose_rename(&snapshot.snapshot_id, uri, offset, new_name)
                            .expect("rename local import binding");
                        let mut actual = source.clone();
                        for replacement in proposal.replacements.iter().rev() {
                            assert_eq!(replacement.uri, uri);
                            actual.replace_range(
                                replacement.span.start..replacement.span.end,
                                &replacement.text,
                            );
                        }
                        assert_eq!(actual, expected);
                    }
                }
            }
        }
    }

    #[test]
    fn removing_an_import_alias_preserves_comments_and_checks_capture() {
        let uri = "file:///workspace/remove-alias.trn";
        for (gap, expected_gap) in [
            (" as local-int", ""),
            (" /* retained */ as local-int", " /* retained */"),
            (" as /* retained */ local-int", " /* retained */"),
        ] {
            let source = format!(
                "namespace app\r\nfrom /core/types import int{gap}, string # retained suffix\r\nfunction main;\r\n    value local-int = 42\r\n"
            );
            let mut engine = ToolingEngine::default();
            let snapshot = open(
                &mut engine,
                uri,
                &source,
                SnapshotOptions {
                    semantic: true,
                    ..SnapshotOptions::default()
                },
            );
            let proposal = engine
                .propose_rename(
                    &snapshot.snapshot_id,
                    uri,
                    source.find("local-int").unwrap(),
                    "int",
                )
                .expect("removing an alias preserves the imported identity");
            let mut actual = source.clone();
            for replacement in proposal.replacements.iter().rev() {
                actual.replace_range(
                    replacement.span.start..replacement.span.end,
                    &replacement.text,
                );
            }
            assert_eq!(
                actual,
                format!(
                    "namespace app\r\nfrom /core/types import int{expected_gap}, string # retained suffix\r\nfunction main;\r\n    value int = 42\r\n"
                )
            );
        }

        let mut engine = ToolingEngine::default();
        let source = "namespace app\nfrom /core/types import int as local-int\nfunction inspect; int string\n    value local-int = 42\n";
        let snapshot = open(
            &mut engine,
            uri,
            source,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        for document in engine
            .snapshot(&snapshot.snapshot_id)
            .unwrap()
            .documents
            .values()
        {
            assert!(
                document.diagnostics.is_empty(),
                "{:?}",
                document.diagnostics
            );
        }
        let error = engine
            .propose_rename(
                &snapshot.snapshot_id,
                uri,
                source.find("local-int").unwrap(),
                "int",
            )
            .expect_err("alias removal must not capture an unrelated parameter");
        assert_eq!(error.code, "rename-capture");
    }

    #[test]
    fn rename_does_not_introduce_an_import_alias() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/no-alias.trn";
        let source =
            "namespace no-alias\nfrom /core/types import int\nfunction main;\n    value int = 42\n";
        let snapshot = open(
            &mut engine,
            uri,
            source,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        for offset in [source.find("int").unwrap(), source.rfind("int").unwrap()] {
            let error = engine
                .propose_rename(&snapshot.snapshot_id, uri, offset, "local-int")
                .expect_err("an external export cannot be renamed by creating an alias");
            assert_eq!(error.code, "invalid-rename");
        }
    }

    #[test]
    fn aliased_export_token_boundaries_reject_without_a_candidate_diagnostic() {
        let uri = "file:///workspace/export-token.trn";
        let source = "namespace app\nfrom /core/types import int as local-int\nfunction main;\n    value local-int = 42\n";
        let mut engine = ToolingEngine::default();
        let snapshot = open(
            &mut engine,
            uri,
            source,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        let export_start = source.find("int as").unwrap();
        for offset in [export_start, export_start + 2, export_start + 3] {
            let error = engine
                .propose_rename(&snapshot.snapshot_id, uri, offset, "amount")
                .expect_err("export-token selection must not rename the alias");
            assert_eq!(error.code, "import-export-rename");
        }
    }

    #[test]
    fn keyword_and_internal_whitespace_select_the_alias_clause() {
        let uri = "file:///workspace/alias-clause.trn";
        let source = "namespace app\nfrom /core/types import int  as  local-int\nfunction main;\n    value local-int = 42\n";
        let expected = source.replace("local-int", "amount");
        let mut engine = ToolingEngine::default();
        let snapshot = open(
            &mut engine,
            uri,
            source,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        for offset in [
            source.find("int  as").unwrap() + 4,
            source.find("as  local-int").unwrap(),
            source.find("as  local-int").unwrap() + 3,
        ] {
            let proposal = engine
                .propose_rename(&snapshot.snapshot_id, uri, offset, "amount")
                .expect("clause keyword and internal gaps select the local alias");
            let mut actual = source.to_owned();
            for replacement in proposal.replacements.iter().rev() {
                actual.replace_range(
                    replacement.span.start..replacement.span.end,
                    &replacement.text,
                );
            }
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn import_binding_rename_rejects_local_capture() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/capture.trn";
        let source = "namespace capture\nfrom /core/types import int as number\nfunction main;\n    taken string = 'value'\n    value number = 1\n";
        let snapshot = open(
            &mut engine,
            uri,
            source,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        let error = engine
            .propose_rename(
                &snapshot.snapshot_id,
                uri,
                source.find("number").unwrap(),
                "taken",
            )
            .expect_err("new alias must not capture a local");
        assert_eq!(error.code, "rename-capture");
    }
}
