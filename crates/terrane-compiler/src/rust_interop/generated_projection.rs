const GENERATED_SOURCE_UNIT_MARKER: &str = "# Generated source unit: ";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedProjectionUnit {
    pub namespace: String,
    pub source: String,
    pub start: usize,
}

/// Splits the compiler-owned projection artifact into ordinary one-namespace source units.
///
/// Text before the first generated-unit marker is report metadata and residual-obligation
/// documentation. It is intentionally not a source unit.
///
/// # Errors
///
/// Returns an error when a generated-unit marker has no body or its declared namespace does not
/// match the first non-comment declaration in that body.
pub fn generated_projection_units(document: &str) -> Result<Vec<GeneratedProjectionUnit>, String> {
    let mut units = Vec::new();
    let mut cursor = 0;
    while let Some(relative_start) = document[cursor..].find(GENERATED_SOURCE_UNIT_MARKER) {
        let marker_start = cursor + relative_start;
        let namespace_start = marker_start + GENERATED_SOURCE_UNIT_MARKER.len();
        let namespace_end = document[namespace_start..]
            .find('\n')
            .map(|offset| namespace_start + offset)
            .ok_or_else(|| "generated source-unit marker has no source body".to_owned())?;
        let namespace = document[namespace_start..namespace_end].trim().to_owned();
        let source_start = namespace_end + 1;
        let source_end = document[source_start..]
            .find(GENERATED_SOURCE_UNIT_MARKER)
            .map_or(document.len(), |offset| source_start + offset);
        let source = document[source_start..source_end].trim_end().to_owned();
        let expected = format!("namespace {}", namespace.trim_start_matches('/'));
        let first_declaration = source
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty() && !line.starts_with('#'));
        if first_declaration != Some(expected.as_str()) {
            return Err(format!(
                "generated source unit `{namespace}` does not begin with `{expected}`",
            ));
        }
        units.push(GeneratedProjectionUnit {
            namespace,
            source,
            start: source_start,
        });
        cursor = source_end;
    }
    Ok(units)
}

pub(crate) fn write_source_unit(output: &mut String, namespace: &str, source: &str) {
    use std::fmt::Write as _;
    writeln!(
        output,
        "{GENERATED_SOURCE_UNIT_MARKER}{namespace}\n{source}"
    )
    .expect("writing to a string cannot fail");
}
