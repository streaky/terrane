use super::{
    DiagnosticProjection, ParsedDocument, ProtocolError, SourceIdentity, hash_text,
    project_diagnostic,
};
use crate::{CompilationFailure, Diagnostic, Package, SourceFile, Span};
use std::collections::BTreeMap;
use std::path::Path;

/// Prepare against overlay declarations, with an isolated, content-addressed
/// source location that ordinary editors can open without a virtual-file plugin.
pub(super) fn prepare(package: &Package, trusted: bool) -> Result<Package, CompilationFailure> {
    let Some(mut expanded) = crate::compiler::prepare_package_for_tooling(package, trusted)? else {
        return Ok(package.clone());
    };
    for unit in &mut expanded.units {
        if !unit.relative_path.starts_with(".trn/generated") {
            continue;
        }
        let suffix = unit
            .relative_path
            .strip_prefix(".trn/generated")
            .expect("generated prefix");
        let digest = hash_text(unit.source.text());
        let relative = Path::new(".trn/editor-generated")
            .join(&digest[7..])
            .join(suffix);
        let absolute = package.root.join(&relative);
        publish(package, &absolute, unit.source.text())?;
        let absolute = absolute.canonicalize().map_err(|error| {
            failure(
                package,
                format!("cannot resolve editor-generated source: {error}"),
            )
        })?;
        unit.source = SourceFile::new(unit.source.id(), absolute, unit.source.text().to_owned());
        unit.relative_path = relative;
    }
    Ok(expanded)
}

fn publish(package: &Package, target: &Path, text: &str) -> Result<(), CompilationFailure> {
    let relative = target
        .strip_prefix(&package.root)
        .expect("editor output belongs to package");
    let mut directory = package.root.clone();
    for component in relative
        .parent()
        .expect("editor source parent")
        .components()
    {
        directory.push(component);
        match std::fs::symlink_metadata(&directory) {
            Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
            Ok(_) => {
                return Err(failure(
                    package,
                    format!(
                        "editor-generated directory `{}` is not an owned directory",
                        directory.display()
                    ),
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir(&directory).map_err(|error| {
                    failure(
                        package,
                        format!("cannot create editor-generated directory: {error}"),
                    )
                })?;
            }
            Err(error) => {
                return Err(failure(
                    package,
                    format!("cannot inspect editor-generated directory: {error}"),
                ));
            }
        }
    }
    if std::fs::symlink_metadata(target).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err(failure(
            package,
            "editor-generated source is a symbolic link".to_owned(),
        ));
    }
    crate::native_build::write_if_changed(target, text.as_bytes()).map_err(|error| {
        failure(
            package,
            format!("cannot publish editor-generated source: {error}"),
        )
    })
}

fn failure(package: &Package, message: String) -> CompilationFailure {
    let source = package.units.first().map_or_else(
        || {
            SourceFile::new(
                0,
                package.root.join(crate::MANIFEST_FILE_NAME),
                String::new(),
            )
        },
        |unit| unit.source.clone(),
    );
    CompilationFailure {
        diagnostics: vec![Diagnostic::error(
            "S2060",
            message,
            Span::new(source.id(), 0, 0),
        )],
        source,
    }
}

/// Package composition owns file IDs. Rebind overlay trees when dependency
/// composition assigned a different ID, and expose dependency/generated units
/// omitted by an explicit-source caller so navigation has real documents.
pub(super) fn synchronize_documents(
    package: &mut Package,
    documents: &mut BTreeMap<String, ParsedDocument>,
) {
    for unit in &mut package.units {
        let canonical = unit
            .source
            .path()
            .canonicalize()
            .unwrap_or_else(|_| unit.source.path().to_path_buf());
        if canonical != unit.source.path() {
            unit.source =
                SourceFile::new(unit.source.id(), canonical, unit.source.text().to_owned());
        }
        let uri = format!("file://{}", unit.source.path().display());
        if documents.get(&uri).is_some_and(|document| {
            document.source.id() == unit.source.id() && document.source.text() == unit.source.text()
        }) {
            continue;
        }
        let source = unit.source.clone();
        let lexed = crate::lexer::lex_recovering(&source);
        let mut diagnostics = lexed.diagnostics;
        let parsed = crate::parser::parse(&source, lexed.lexed);
        diagnostics.extend(parsed.diagnostics);
        documents.insert(
            uri.clone(),
            ParsedDocument {
                identity: SourceIdentity {
                    uri,
                    content_hash: hash_text(source.text()),
                },
                source,
                tree: parsed.tree,
                diagnostics,
            },
        );
    }
}

pub(super) fn record_failure(
    documents: &mut BTreeMap<String, ParsedDocument>,
    package: &Package,
    failure: CompilationFailure,
) {
    // File IDs are local to one composed graph. A separately compiled consumer
    // may have file 0 too; its diagnostics must not masquerade as caller spans.
    let origin = failure
        .source
        .path()
        .canonicalize()
        .unwrap_or_else(|_| failure.source.path().to_path_buf());
    let source_uri = documents
        .iter()
        .find_map(|(uri, document)| (document.source.path() == origin).then(|| uri.clone()));
    let fallback = package
        .units
        .first()
        .map(|unit| format!("file://{}", unit.source.path().display()))
        .filter(|uri| documents.contains_key(uri))
        .or_else(|| documents.keys().next().cloned());
    for mut diagnostic in failure.diagnostics {
        let uri = source_uri
            .as_ref()
            .and_then(|source_uri| {
                diagnostic
                    .primary
                    .and_then(|span| {
                        documents.iter().find_map(|(uri, document)| {
                            (document.source.id() == span.file).then(|| uri.clone())
                        })
                    })
                    .or_else(|| Some(source_uri.clone()))
            })
            .or_else(|| fallback.clone());
        let Some(uri) = uri else { continue };
        let document = documents.get_mut(&uri).expect("selected snapshot document");
        if source_uri.is_none() {
            diagnostic.message = format!("in `{}`: {}", origin.display(), diagnostic.message);
            diagnostic.primary = Some(Span::new(document.source.id(), 0, 0));
        } else if diagnostic
            .primary
            .is_some_and(|span| span.file != document.source.id())
        {
            diagnostic.primary = Some(Span::new(document.source.id(), 0, 0));
        }
        document.diagnostics.push(diagnostic);
    }
}

pub(super) fn snapshot_identity(
    base: &str,
    documents: &BTreeMap<String, ParsedDocument>,
) -> Result<String, ProtocolError> {
    let sources: Vec<_> = documents
        .values()
        .map(|document| &document.identity)
        .collect();
    let diagnostics: Vec<DiagnosticProjection> = documents
        .values()
        .flat_map(|document| document.diagnostics.iter().map(project_diagnostic))
        .collect();
    super::hash_json(&(base, sources, diagnostics))
}
