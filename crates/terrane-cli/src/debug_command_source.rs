use super::{
    BTreeMap, BreakpointResolution, DebugAssociation, Path, PathBuf, ProvenanceManifest,
    StopContext, Value, json,
};
use terrane_compiler::debugging::{GeneratedFileIdentity, SourceIdentity};

fn normalized_path(path: &Path) -> PathBuf {
    path.canonicalize()
        .unwrap_or_else(|_| lexical_normalize(path))
}

pub(super) fn find_source<'a>(
    provenance: &'a ProvenanceManifest,
    requested: &Path,
) -> Option<(&'a SourceIdentity, PathBuf)> {
    let physical_match = |path: &Path| {
        let requested = normalized_path(path);
        provenance.debug.sources.iter().find_map(|source| {
            let path = normalized_path(&provenance.source_path(source));
            (path == requested).then_some((source, path))
        })
    };
    if requested.is_absolute() {
        return physical_match(requested);
    }
    std::env::current_dir()
        .ok()
        .and_then(|cwd| physical_match(&cwd.join(requested)))
        .or_else(|| {
            let logical = lexical_normalize(requested);
            provenance
                .debug
                .sources
                .iter()
                .find(|source| Path::new(&source.uri) == logical)
                .map(|source| (source, normalized_path(&provenance.source_path(source))))
        })
        .or_else(|| physical_match(&Path::new(&provenance.relocation.source_root).join(requested)))
}

pub(super) fn lexical_normalize(path: &Path) -> PathBuf {
    use std::path::Component;

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() && !path.is_absolute() {
                    normalized.push(component);
                }
            }
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component);
            }
        }
    }
    normalized
}

pub(super) fn known_source_path(
    provenance: &ProvenanceManifest,
    requested: &Path,
) -> Option<PathBuf> {
    find_source(provenance, requested).map(|(_, path)| path)
}

pub(super) fn all_source_associations(
    provenance: &ProvenanceManifest,
) -> impl Iterator<Item = (&str, &DebugAssociation)> {
    provenance.debug.generated_files.iter().flat_map(|file| {
        file.associations
            .iter()
            .map(move |association| (file.path.as_str(), association))
    })
}

pub(super) fn resolve_breakpoint(
    provenance: &ProvenanceManifest,
    requested_path: &Path,
    line: usize,
) -> Vec<BreakpointResolution> {
    let Some((source, _)) = find_source(provenance, requested_path) else {
        return Vec::new();
    };
    let function = provenance
        .debug
        .functions
        .iter()
        .filter(|function| {
            function.source.source_id == source.id
                && function.source.line <= line
                && function.source.end_line >= line
        })
        .min_by_key(|function| function.source.end - function.source.start);
    let scope = provenance
        .debug
        .scopes
        .iter()
        .filter(|scope| {
            scope.source.source_id == source.id
                && scope.source.line <= line
                && scope.source.end_line >= line
                && function
                    .is_none_or(|function| scope.function_id.as_deref() == Some(&function.id))
        })
        .min_by_key(|scope| scope.source.end - scope.source.start);
    let mut candidates = all_source_associations(provenance)
        .filter(|(_, association)| {
            association.sequence_point
                && association
                    .causes
                    .iter()
                    .any(|cause| cause.source_id == source.id && cause.line == line)
        })
        .collect::<Vec<_>>();
    let adjusted = candidates.is_empty();
    if adjusted {
        let (Some(function), Some(scope)) = (function, scope) else {
            return Vec::new();
        };
        candidates = all_source_associations(provenance)
            .filter(|(_, association)| {
                association.sequence_point
                    && association.function_id.as_deref() == Some(&function.id)
                    && association.scope_ids.contains(&scope.id)
                    && association
                        .causes
                        .iter()
                        .any(|cause| cause.source_id == source.id && cause.line >= line)
            })
            .collect();
        let Some(distance) = candidates
            .iter()
            .flat_map(|(_, association)| {
                association
                    .causes
                    .iter()
                    .filter(|cause| cause.source_id == source.id && cause.line >= line)
                    .map(|cause| cause.line - line)
            })
            .min()
        else {
            return Vec::new();
        };
        candidates.retain(|(_, association)| {
            association.causes.iter().any(|cause| {
                cause.source_id == source.id && cause.line >= line && cause.line - line == distance
            })
        });
    }
    candidates
        .into_iter()
        .map(|(path, association)| {
            let source_line = association.causes.first().map_or(line, |cause| cause.line);
            BreakpointResolution {
                generated_path: path.to_owned(),
                generated_line: association.generated.line,
                source_line,
                message: if adjusted {
                    format!("adjusted to executable line {source_line}")
                } else {
                    "exact executable sequence point".to_owned()
                },
            }
        })
        .collect()
}

pub(super) fn generated_path(provenance: &ProvenanceManifest, relative: &str) -> PathBuf {
    Path::new(&provenance.relocation.build_root).join(relative)
}

pub(super) fn association_for_frame<'a>(
    provenance: &'a ProvenanceManifest,
    frame: &Value,
) -> Option<&'a DebugAssociation> {
    mapped_frame(provenance, frame).map(|(_, _, association)| association)
}

fn mapped_frame<'a>(
    provenance: &'a ProvenanceManifest,
    frame: &Value,
) -> Option<(&'a GeneratedFileIdentity, usize, &'a DebugAssociation)> {
    let path = Path::new(frame["source"]["path"].as_str()?);
    let line = usize::try_from(frame["line"].as_u64()?).ok()?;
    let file = provenance
        .debug
        .generated_files
        .iter()
        .find(|file| path.ends_with(&file.path))?;
    let association = file
        .associations
        .iter()
        .find(|association| association.sequence_point && association.generated.line == line)?;
    Some((file, line, association))
}

pub(super) fn generated_association(
    file: &GeneratedFileIdentity,
    line: usize,
) -> Option<&DebugAssociation> {
    file.associations
        .iter()
        .filter(|association| {
            let range = &association.generated;
            association.sequence_point
                && range.line <= line
                && (line < range.end_line
                    || (line == range.end_line && (line == range.line || range.end_column > 1)))
        })
        .min_by_key(|association| {
            let range = &association.generated;
            (range.line != line, range.end.saturating_sub(range.start))
        })
}

pub(super) fn translate_stack_frames(
    body: &mut Value,
    provenance: &ProvenanceManifest,
    include_native: bool,
) {
    let Some(frames) = body["stackFrames"].as_array_mut() else {
        return;
    };
    for frame in frames.iter_mut() {
        let Some((file, line, association)) = mapped_frame(provenance, frame) else {
            if !include_native {
                frame["presentationHint"] = "subtle".into();
            }
            continue;
        };
        let Some(cause) = association.causes.first() else {
            continue;
        };
        let Some(source) = provenance
            .debug
            .sources
            .iter()
            .find(|source| source.id == cause.source_id)
        else {
            continue;
        };
        frame["terraneGenerated"] = json!({
            "path": generated_path(provenance, &file.path),
            "line": line,
        });
        let path = provenance.source_path(source);
        frame["source"] = json!({"name": path.file_name().unwrap_or_default(), "path": path});
        frame["line"] = cause.line.into();
        frame["column"] = cause.column.into();
        if let Some(function) = association.function_id.as_deref().and_then(|id| {
            provenance
                .debug
                .functions
                .iter()
                .find(|function| function.id == id)
        }) {
            frame["name"] = function.name.clone().into();
        }
    }
}

pub(super) fn frame_stop_context(
    frame: &Value,
    provenance: &ProvenanceManifest,
) -> Option<StopContext> {
    let association = association_for_frame(provenance, frame)?;
    let cause = association.causes.first()?;
    Some(StopContext {
        source_id: cause.source_id,
        position: cause.start,
        function_id: association.function_id.clone(),
        scope_ids: association.scope_ids.clone(),
    })
}

pub(super) fn frame_stop_contexts(
    body: &Value,
    provenance: &ProvenanceManifest,
) -> BTreeMap<i64, StopContext> {
    body["stackFrames"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|frame| {
            Some((
                frame["id"].as_i64()?,
                frame_stop_context(frame, provenance)?,
            ))
        })
        .collect()
}
