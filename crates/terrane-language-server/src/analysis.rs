use super::{Analysis, GeneratedRustResponse, snapshot_inputs};
use std::collections::HashMap;
use std::sync::Mutex;
use terrane_compiler::tooling::{
    Availability, DiagnosticProjection, SnapshotOptions, ToolingEngine,
};
use tower_lsp_server::ls_types::Uri;

pub(super) fn snapshot(
    uri: &Uri,
    text: &str,
    overlays: &HashMap<String, String>,
    tooling: &Mutex<ToolingEngine>,
    trusted: bool,
) -> Analysis {
    let uri_text = uri.to_string();
    {
        let tooling = tooling.lock().expect("tooling engine lock");
        if let Some(metadata) = tooling.generated_source_snapshot(&uri_text, text) {
            let diagnostics = tooling
                .editor_diagnostics(&metadata.snapshot_id, &uri_text)
                .expect("retained generated source");
            return Analysis {
                snapshot_id: metadata.snapshot_id,
                source_texts: HashMap::from([(uri_text, text.to_owned())]),
                diagnostics,
            };
        }
    }
    let (sources, manifest, testing) = snapshot_inputs(uri, text, overlays);
    let source_texts = sources
        .iter()
        .map(|source| (source.uri.clone(), source.text.clone()))
        .collect();
    let mut tooling = tooling.lock().expect("tooling engine lock");
    let metadata = match tooling.open_snapshot(
        sources,
        manifest,
        None,
        SnapshotOptions {
            semantic: true,
            allow_consumer_execution: trusted,
            testing,
            ..SnapshotOptions::default()
        },
    ) {
        Ok(metadata) => metadata,
        Err(error) => {
            return Analysis {
                snapshot_id: String::new(),
                source_texts,
                diagnostics: vec![DiagnosticProjection {
                    severity: "error".to_owned(),
                    code: "S2060".to_owned(),
                    message: format!("{}: {}", error.code, error.message),
                    span: None,
                    help: None,
                }],
            };
        }
    };
    let diagnostics = tooling
        .editor_diagnostics(&metadata.snapshot_id, &uri_text)
        .expect("opened source belongs to its snapshot");
    Analysis {
        snapshot_id: metadata.snapshot_id,
        source_texts,
        diagnostics,
    }
}

pub(super) fn generated_rust(
    uri: &Uri,
    text: &str,
    overlays: &HashMap<String, String>,
    tooling: &Mutex<ToolingEngine>,
    trusted: bool,
    offset: usize,
) -> Option<GeneratedRustResponse> {
    let uri_text = uri.to_string();
    let (sources, manifest, testing) = snapshot_inputs(uri, text, overlays);
    let mut tooling = tooling.lock().expect("tooling engine lock");
    let metadata = tooling
        .open_snapshot(
            sources,
            manifest,
            None,
            SnapshotOptions {
                semantic: true,
                generated: true,
                allow_consumer_execution: trusted,
                testing,
                ..SnapshotOptions::default()
            },
        )
        .ok()?;
    let response = tooling
        .locate(&metadata.snapshot_id, &uri_text, offset)
        .ok()
        .flatten()
        .map(|object| {
            let availability = if let Availability::Known(build_id) = &metadata.build_id {
                tooling
                    .generated_rust(
                        &metadata.snapshot_id,
                        &uri_text,
                        object.syntax_node_id,
                        build_id,
                    )
                    .unwrap_or(Availability::Unresolved)
            } else {
                Availability::NotYetAnalyzed
            };
            GeneratedRustResponse { availability }
        });
    let _ = tooling.close_snapshot(&metadata.snapshot_id);
    response
}
