// Compiler-generated glue materializes statically known Terrane destinations. Parser policy
// remains in document support; field/default/validation policy remains in emitted class decoders.
#[derive(Clone)]
pub struct TerraneDocumentDiagnostic {
    pub path: String,
    pub expected: String,
    pub actual_kind: String,
    pub reason: &'static str,
    pub message: String,
    pub source: String,
    pub field_source: String,
}

#[derive(Clone)]
pub struct TerraneDocumentDecodeOutcome<T: Clone> {
    pub value: T,
    pub diagnostics: terrane_collection_support::List<TerraneDocumentDiagnostic>,
}

fn __terrane_document_diagnostic(
    path: &impl std::fmt::Display,
    expected: impl Into<String>,
    actual_kind: impl Into<String>,
    reason: &'static str,
    message: impl Into<String>,
    source: &str,
    field_source: &str,
) -> TerraneDocumentDiagnostic {
    TerraneDocumentDiagnostic {
        path: path.to_string(),
        expected: expected.into(),
        actual_kind: actual_kind.into(),
        reason,
        message: message.into(),
        source: source.to_owned(),
        field_source: field_source.to_owned(),
    }
}
