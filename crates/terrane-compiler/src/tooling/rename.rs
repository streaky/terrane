use super::{
    ProtocolError, Replacement, SemanticTarget, Snapshot, declaration_location, import_rename,
    node_text, rename_identity_family, rename_would_capture, resolve_semantic_target, walk_nodes,
};
use crate::syntax::SyntaxKind;

pub(super) fn replacements(
    snapshot: &Snapshot,
    target: &SemanticTarget,
    new_name: &str,
) -> Result<Vec<Replacement>, ProtocolError> {
    let Some(semantic) = &snapshot.semantic else {
        return Err(ProtocolError::new(
            "semantic-unavailable",
            "rename requires a complete semantic snapshot",
        ));
    };
    let identities = rename_identity_family(semantic, target);
    let mut replacements = Vec::new();
    for unit in semantic
        .units
        .iter()
        .filter(|unit| !unit.source_path.starts_with("<terrane>"))
    {
        let Some(source_uri) = snapshot.uri_for_file(unit.source.id()) else {
            continue;
        };
        let Some(document) = snapshot
            .documents
            .values()
            .find(|document| document.source.id() == unit.source.id())
        else {
            continue;
        };
        import_rename::canonical_import_removals(
            document,
            unit,
            &identities,
            new_name,
            source_uri,
            &mut replacements,
        )?;
        walk_nodes(&unit.tree.root, None, &mut |node, _field, _id| {
            if node.kind == SyntaxKind::Name
                    // Explicit local aliases retain their own spelling when an export changes.
                    && node_text(&document.source, node) == target.name
                    && resolve_semantic_target(snapshot, document, node)
                        .is_some_and(|candidate| identities.contains(&candidate.identity))
            {
                replacements.push(Replacement {
                    uri: source_uri.to_owned(),
                    span: node.span.into(),
                    text: new_name.to_owned(),
                });
            }
        });
    }
    if let Some(location) = declaration_location(snapshot, target)
        && !replacements
            .iter()
            .any(|replacement| replacement.uri == location.uri && replacement.span == location.span)
    {
        replacements.push(Replacement {
            uri: location.uri,
            span: location.span,
            text: new_name.to_owned(),
        });
    }
    for replacement in &replacements {
        let Some(unit) = semantic
            .units
            .iter()
            .find(|unit| snapshot.uri_for_file(unit.source.id()) == Some(replacement.uri.as_str()))
        else {
            return Err(ProtocolError::new(
                "unknown-source",
                "rename source left the semantic snapshot",
            ));
        };
        if rename_would_capture(
            semantic,
            unit,
            target,
            &identities,
            new_name,
            replacement.span.start,
        ) {
            return Err(ProtocolError::new(
                "rename-capture",
                format!("`{new_name}` would capture an existing symbol"),
            ));
        }
    }
    Ok(replacements)
}
