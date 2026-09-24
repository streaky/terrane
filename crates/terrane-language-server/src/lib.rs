use std::collections::{BTreeSet, HashMap};
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};

use serde::{Deserialize, Serialize};
use terrane_compiler::highlight::{Highlight, HighlightKind, highlight};
#[cfg(test)]
use terrane_compiler::{Diagnostic as TerraneDiagnostic, Severity};
use terrane_compiler::{SourceFile, Span};
use tokio::sync::RwLock;
use tower_lsp_server::jsonrpc::{Error as JsonRpcError, Result};
use tower_lsp_server::ls_types::request::{GotoImplementationParams, GotoImplementationResponse};
use tower_lsp_server::ls_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionParams,
    CodeActionProviderCapability, CodeActionResponse, CompletionItem, CompletionItemKind,
    CompletionOptions, CompletionParams, CompletionResponse, Diagnostic, DiagnosticSeverity,
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DocumentChanges, DocumentFormattingParams, DocumentSymbol, DocumentSymbolParams,
    DocumentSymbolResponse, Documentation, GotoDefinitionParams, GotoDefinitionResponse, Hover,
    HoverContents, HoverParams, HoverProviderCapability, ImplementationProviderCapability,
    InitializeParams, InitializeResult, InitializedParams, Location as LspLocation, MarkedString,
    MessageType, NumberOrString, OneOf, OptionalVersionedTextDocumentIdentifier,
    ParameterInformation, ParameterLabel, Position, PositionEncodingKind, Range, ReferenceParams,
    RenameParams, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams,
    SemanticTokensResult, SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo,
    SignatureHelp, SignatureHelpOptions, SignatureHelpParams, SignatureInformation, SymbolKind,
    TextDocumentEdit, TextDocumentIdentifier, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextEdit, Uri, WorkspaceEdit,
};
use tower_lsp_server::{Client, LanguageServer};

const TOKEN_TYPES: [SemanticTokenType; 11] = [
    SemanticTokenType::COMMENT,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::NUMBER,
    SemanticTokenType::STRING,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::NAMESPACE,
    SemanticTokenType::TYPE,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::VARIABLE,
];

#[derive(Clone, Debug)]
struct Document {
    text: String,
    version: i32,
    snapshot_id: String,
}

#[derive(Debug)]
struct Analysis {
    snapshot_id: String,
    source_texts: HashMap<String, String>,
    diagnostics: Vec<terrane_compiler::tooling::DiagnosticProjection>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedRustParams {
    text_document: TextDocumentIdentifier,
    position: Position,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedRustResponse {
    availability:
        terrane_compiler::tooling::Availability<Vec<terrane_compiler::tooling::GeneratedLocation>>,
}
#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: Arc<RwLock<HashMap<Uri, Document>>>,
    tooling: Arc<Mutex<terrane_compiler::tooling::ToolingEngine>>,
    position_encoding: Arc<Mutex<PositionEncodingKind>>,
}

impl Backend {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            tooling: Arc::new(Mutex::new(terrane_compiler::tooling::ToolingEngine::new(
                usize::MAX,
                usize::MAX,
            ))),
            position_encoding: Arc::new(Mutex::new(PositionEncodingKind::UTF16)),
        }
    }

    async fn analyze(&self, uri: &Uri, text: &str) -> Analysis {
        let uri_text = uri.to_string();
        let mut overlays = self
            .documents
            .read()
            .await
            .iter()
            .map(|(uri, document)| (uri.to_string(), document.text.clone()))
            .collect::<HashMap<_, _>>();
        overlays.insert(uri_text.clone(), text.to_owned());
        let (sources, manifest, testing) = snapshot_inputs(uri, text, &overlays);
        let source_texts = sources
            .iter()
            .map(|source| (source.uri.clone(), source.text.clone()))
            .collect();
        let (metadata, syntax) = {
            let mut tooling = self.tooling.lock().expect("tooling engine lock");
            let metadata = tooling
                .open_snapshot(
                    sources,
                    manifest,
                    None,
                    terrane_compiler::tooling::SnapshotOptions {
                        semantic: true,
                        generated: false,
                        testing,
                        ..terrane_compiler::tooling::SnapshotOptions::default()
                    },
                )
                .expect("in-memory editor sources are valid snapshot inputs");
            let syntax = tooling
                .syntax(&metadata.snapshot_id, &uri_text, None)
                .expect("the opened source belongs to its snapshot");
            (metadata, syntax)
        };
        Analysis {
            snapshot_id: metadata.snapshot_id,
            source_texts,
            diagnostics: syntax.diagnostics,
        }
    }

    async fn install_analysis(&self, uri: Uri, text: String, version: i32, analysis: Analysis) {
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let generated_projection = uri.to_file_path().is_some_and(|path| {
            path.file_name()
                .is_some_and(|name| name == terrane_compiler::projection::GENERATED_PROJECTION_FILE)
        });
        let diagnostics = analysis
            .diagnostics
            .iter()
            .filter(|diagnostic| !(generated_projection && diagnostic.code == "S2002"))
            .map(|diagnostic| tooling_lsp_diagnostic(&text, diagnostic, &encoding))
            .collect();
        let mut documents = self.documents.write().await;
        if documents
            .get(&uri)
            .is_some_and(|document| document.version > version)
        {
            let snapshot_in_use = documents
                .values()
                .any(|document| document.snapshot_id == analysis.snapshot_id);
            drop(documents);
            if !snapshot_in_use {
                let _ = self
                    .tooling
                    .lock()
                    .expect("tooling engine lock")
                    .close_snapshot(&analysis.snapshot_id);
            }
            return;
        }
        let old_snapshots = documents
            .values()
            .map(|document| document.snapshot_id.clone())
            .collect::<std::collections::HashSet<_>>();
        for (open_uri, document) in &mut *documents {
            let open_uri = open_uri.to_string();
            if analysis
                .source_texts
                .get(&open_uri)
                .is_some_and(|analyzed| analyzed == &document.text)
            {
                document.snapshot_id.clone_from(&analysis.snapshot_id);
            }
        }
        documents.insert(
            uri.clone(),
            Document {
                text,
                version,
                snapshot_id: analysis.snapshot_id.clone(),
            },
        );
        let retained = documents
            .values()
            .map(|document| document.snapshot_id.as_str())
            .collect::<std::collections::HashSet<_>>();
        let expired = old_snapshots
            .into_iter()
            .filter(|snapshot| !retained.contains(snapshot.as_str()))
            .collect::<Vec<_>>();
        drop(documents);
        {
            let mut tooling = self.tooling.lock().expect("tooling engine lock");
            for snapshot in expired {
                let _ = tooling.close_snapshot(&snapshot);
            }
        }
        self.client
            .publish_diagnostics(uri, diagnostics, Some(version))
            .await;
    }
    /// Finds generated Rust associated with the Terrane syntax at a document position.
    ///
    /// # Errors
    ///
    /// Returns an LSP error when snapshot lookup fails.
    ///
    /// # Panics
    ///
    /// Panics if an internal language-server state lock is poisoned.
    pub async fn generated_rust(
        &self,
        params: GeneratedRustParams,
    ) -> Result<Option<GeneratedRustResponse>> {
        let uri = params.text_document.uri;
        let (document, overlays) = {
            let documents = self.documents.read().await;
            let Some(document) = documents.get(&uri).cloned() else {
                return Ok(None);
            };
            let overlays = documents
                .iter()
                .map(|(uri, document)| (uri.to_string(), document.text.clone()))
                .collect::<HashMap<_, _>>();
            (document, overlays)
        };
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let Some(offset) = byte_offset(&document.text, params.position, &encoding) else {
            return Ok(None);
        };
        let uri_text = uri.to_string();
        let (sources, manifest, testing) = snapshot_inputs(&uri, &document.text, &overlays);
        let mut tooling = self.tooling.lock().expect("tooling engine lock");
        let metadata = tooling
            .open_snapshot(
                sources,
                manifest,
                None,
                terrane_compiler::tooling::SnapshotOptions {
                    semantic: true,
                    generated: true,
                    testing,
                    ..terrane_compiler::tooling::SnapshotOptions::default()
                },
            )
            .expect("open documents are valid generated snapshot inputs");
        let response = tooling
            .locate(&metadata.snapshot_id, &uri_text, offset)
            .ok()
            .flatten()
            .map(|object| {
                let availability = if let terrane_compiler::tooling::Availability::Known(build_id) =
                    &metadata.build_id
                {
                    tooling
                        .generated_rust(
                            &metadata.snapshot_id,
                            &uri_text,
                            object.syntax_node_id,
                            build_id,
                        )
                        .unwrap_or(terrane_compiler::tooling::Availability::Unresolved)
                } else {
                    terrane_compiler::tooling::Availability::NotYetAnalyzed
                };
                GeneratedRustResponse { availability }
            });
        let _ = tooling.close_snapshot(&metadata.snapshot_id);
        Ok(response)
    }
}

impl LanguageServer for Backend {
    #[expect(
        clippy::unused_async_trait_impl,
        reason = "the language-server trait declares lifecycle handlers as async"
    )]
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let encoding = params
            .capabilities
            .general
            .as_ref()
            .and_then(|general| general.position_encodings.as_ref())
            .and_then(|encodings| {
                encodings
                    .iter()
                    .find(|encoding| {
                        **encoding == PositionEncodingKind::UTF8
                            || **encoding == PositionEncodingKind::UTF16
                            || **encoding == PositionEncodingKind::UTF32
                    })
                    .cloned()
            })
            .unwrap_or(PositionEncodingKind::UTF16);
        *self
            .position_encoding
            .lock()
            .expect("position encoding lock") = encoding.clone();
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                position_encoding: Some(encoding),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            legend: SemanticTokensLegend {
                                token_types: TOKEN_TYPES.to_vec(),
                                token_modifiers: vec![SemanticTokenModifier::DECLARATION],
                            },
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                            ..Default::default()
                        },
                    ),
                ),
                completion_provider: Some(CompletionOptions::default()),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec![";".to_owned()]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Left(true)),
                implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "Terrane Language Server".to_owned(),
                version: Some(terrane_compiler::VERSION.to_owned()),
            }),
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Terrane language server initialized")
            .await;
    }

    #[expect(
        clippy::unused_async_trait_impl,
        reason = "the language-server trait declares lifecycle handlers as async"
    )]
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;
        let analysis = self.analyze(&uri, &text).await;
        self.install_analysis(uri, text, version, analysis).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let Some(change) = params.content_changes.into_iter().last() else {
            return;
        };
        let uri = params.text_document.uri;
        let text = change.text;
        let version = params.text_document.version;
        let analysis = self.analyze(&uri, &text).await;
        self.install_analysis(uri, text, version, analysis).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut documents = self.documents.write().await;
        let closed = documents.remove(&uri);
        let snapshot_in_use = closed.as_ref().is_some_and(|closed| {
            documents
                .values()
                .any(|document| document.snapshot_id == closed.snapshot_id)
        });
        drop(documents);
        if let Some(document) = closed
            && !snapshot_in_use
        {
            let _ = self
                .tooling
                .lock()
                .expect("tooling engine lock")
                .close_snapshot(&document.snapshot_id);
        }
        self.client.publish_diagnostics(uri, Vec::new(), None).await;
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let documents = self.documents.read().await;
        let Some(document) = documents.get(&uri) else {
            return Ok(None);
        };
        let source = source_file(&uri, &document.text);
        let output = highlight(&source);
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: Some(document.version.to_string()),
            data: encode_semantic_tokens(source.text(), &output.highlights),
        })))
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let Some(document) = self.documents.read().await.get(&uri).cloned() else {
            return Ok(None);
        };
        let Some(projection) = projection_for_uri(&uri, &document.text).await else {
            return Ok(None);
        };
        let namespace =
            dependency_import_namespace(&document.text, params.text_document_position.position);
        let Some(namespace) = namespace else {
            return Ok(None);
        };
        let mut items = projection
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| item.namespace == namespace)
            .map(|item| CompletionItem {
                label: item.name.clone(),
                kind: Some(match item.kind {
                    terrane_compiler::projection::ProjectedKind::Function(_) => {
                        CompletionItemKind::FUNCTION
                    }
                    _ => CompletionItemKind::CLASS,
                }),
                detail: Some(projected_item_detail(item)),
                documentation: item.docs.clone().map(Documentation::String),
                ..Default::default()
            })
            .collect::<Vec<_>>();
        for dependency in &projection.dependencies {
            items.extend(
                dependency
                    .declined
                    .iter()
                    .filter(|item| declined_namespace(dependency, &item.rust_path) == namespace)
                    .filter_map(|item| {
                        item.rust_path
                            .rsplit("::")
                            .next()
                            .map(|name| CompletionItem {
                                label: name.to_owned(),
                                kind: Some(CompletionItemKind::REFERENCE),
                                detail: Some(item.rust_path.clone()),
                                documentation: Some(Documentation::String(format!(
                                    "Not projected: {}",
                                    item.reason
                                ))),
                                ..Default::default()
                            })
                    }),
            );
        }
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let Some(document) = self.documents.read().await.get(&uri).cloned() else {
            return Ok(None);
        };
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let position = params.text_document_position_params.position;
        if let Some(name) = word_at(&document.text, position)
            && let Some(namespace) = imported_dependency_namespace(&document.text, name)
            && let Some(projection) = projection_for_uri(&uri, &document.text).await
            && let Some(content) =
                projected_hover_content(&projection, name, Some(namespace.as_str()))
        {
            return Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(content)),
                range: None,
            }));
        }
        if let Some(offset) = byte_offset(
            &document.text,
            params.text_document_position_params.position,
            &encoding,
        ) {
            let object = self
                .tooling
                .lock()
                .expect("tooling engine lock")
                .locate(&document.snapshot_id, &uri.to_string(), offset)
                .ok()
                .flatten();
            if let Some(object) = object
                && let Some(hover) = semantic_hover(object, &document.text, &encoding)
            {
                return Ok(Some(hover));
            }
        }
        let Some(name) = word_at(
            &document.text,
            params.text_document_position_params.position,
        ) else {
            return Ok(None);
        };
        let Some(projection) = projection_for_uri(&uri, &document.text).await else {
            return Ok(None);
        };
        let namespace = imported_dependency_namespace(&document.text, name);
        let content = projected_hover_content(&projection, name, namespace.as_deref());
        Ok(content.map(|content| Hover {
            contents: HoverContents::Scalar(MarkedString::String(content)),
            range: None,
        }))
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        let uri = params.text_document_position_params.text_document.uri;
        let documents = self.documents.read().await;
        let Some(document) = documents.get(&uri) else {
            return Ok(None);
        };
        let Some(name) = call_name_before(
            &document.text,
            params.text_document_position_params.position,
        ) else {
            return Ok(None);
        };
        let Some(projection) = projection_for_uri(&uri, &document.text).await else {
            return Ok(None);
        };
        let namespace = imported_dependency_namespace(&document.text, name);
        let function = projection
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find_map(|item| {
                if namespace
                    .as_deref()
                    .is_some_and(|value| item.namespace != value)
                {
                    return None;
                }
                match &item.kind {
                    terrane_compiler::projection::ProjectedKind::Function(function)
                        if function.name == name =>
                    {
                        Some(function)
                    }
                    _ => None,
                }
            });
        let Some(function) = function else {
            return Ok(None);
        };
        let parameters = function
            .parameters
            .iter()
            .map(|parameter| ParameterInformation {
                label: ParameterLabel::Simple(format!(
                    "{} {}",
                    parameter.name,
                    parameter.ty.terrane_name()
                )),
                documentation: None,
            })
            .collect::<Vec<_>>();
        let label = format!(
            "{}; {}",
            function.name,
            parameters
                .iter()
                .filter_map(|parameter| match &parameter.label {
                    ParameterLabel::Simple(label) => Some(label.as_str()),
                    ParameterLabel::LabelOffsets(_) => None,
                })
                .collect::<Vec<_>>()
                .join(", ")
        );
        Ok(Some(SignatureHelp {
            signatures: vec![SignatureInformation {
                label,
                documentation: None,
                parameters: Some(parameters),
                active_parameter: None,
            }],
            active_signature: Some(0),
            active_parameter: Some(0),
        }))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let Some(document) = self.documents.read().await.get(&uri).cloned() else {
            return Ok(None);
        };
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let Some(offset) = byte_offset(
            &document.text,
            params.text_document_position_params.position,
            &encoding,
        ) else {
            return Ok(None);
        };
        let definition = self
            .tooling
            .lock()
            .expect("tooling engine lock")
            .definition(&document.snapshot_id, &uri.to_string(), offset)
            .ok();
        let Some(terrane_compiler::tooling::Availability::Known(location)) = definition else {
            return Ok(None);
        };
        Ok(lsp_location(&location, &self.documents, &encoding)
            .await
            .map(GotoDefinitionResponse::Scalar))
    }

    async fn goto_implementation(
        &self,
        params: GotoImplementationParams,
    ) -> Result<Option<GotoImplementationResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let Some(document) = self.documents.read().await.get(&uri).cloned() else {
            return Ok(None);
        };
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let Some(offset) = byte_offset(
            &document.text,
            params.text_document_position_params.position,
            &encoding,
        ) else {
            return Ok(None);
        };
        let implementations = self
            .tooling
            .lock()
            .expect("tooling engine lock")
            .implementations(&document.snapshot_id, &uri.to_string(), offset)
            .ok();
        let Some(terrane_compiler::tooling::Availability::Known(implementations)) = implementations
        else {
            return Ok(None);
        };
        let mut locations = Vec::new();
        for implementation in implementations {
            if let Some(location) = lsp_location(&implementation, &self.documents, &encoding).await
            {
                locations.push(location);
            }
        }
        Ok(Some(GotoImplementationResponse::Array(locations)))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<LspLocation>>> {
        let uri = params.text_document_position.text_document.uri;
        let Some(document) = self.documents.read().await.get(&uri).cloned() else {
            return Ok(None);
        };
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let Some(offset) = byte_offset(
            &document.text,
            params.text_document_position.position,
            &encoding,
        ) else {
            return Ok(None);
        };
        let (references, definition) = {
            let tooling = self.tooling.lock().expect("tooling engine lock");
            (
                tooling
                    .references(&document.snapshot_id, &uri.to_string(), offset)
                    .ok(),
                tooling
                    .definition(&document.snapshot_id, &uri.to_string(), offset)
                    .ok(),
            )
        };
        let Some(terrane_compiler::tooling::Availability::Known(references)) = references else {
            return Ok(None);
        };
        let definition = match definition {
            Some(terrane_compiler::tooling::Availability::Known(location)) => Some(location),
            _ => None,
        };
        let mut locations = Vec::new();
        for location in references {
            if !params.context.include_declaration
                && definition
                    .as_ref()
                    .is_some_and(|definition| *definition == location)
            {
                continue;
            }
            if let Some(location) = lsp_location(&location, &self.documents, &encoding).await {
                locations.push(location);
            }
        }
        Ok(Some(locations))
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let documents = self.documents.read().await;
        let Some(document) = documents.get(&uri).cloned() else {
            return Ok(None);
        };
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let Some(offset) = byte_offset(
            &document.text,
            params.text_document_position.position,
            &encoding,
        ) else {
            return Ok(None);
        };
        let proposal = self
            .tooling
            .lock()
            .expect("tooling engine lock")
            .propose_rename(
                &document.snapshot_id,
                &uri.to_string(),
                offset,
                &params.new_name,
            )
            .map_err(|error| {
                JsonRpcError::invalid_params(format!("{}: {}", error.code, error.message))
            })?;
        let mut grouped = std::collections::BTreeMap::<String, Vec<_>>::new();
        for replacement in proposal.replacements {
            grouped
                .entry(replacement.uri.clone())
                .or_default()
                .push(replacement);
        }
        let mut edits = Vec::new();
        for (uri_text, replacements) in grouped {
            let Ok(edit_uri) = uri_text.parse::<Uri>() else {
                return Ok(None);
            };
            let (text, version) = if let Some(open) = documents.get(&edit_uri) {
                (open.text.clone(), Some(open.version))
            } else {
                let Some(path) = uri_text.strip_prefix("file://") else {
                    return Ok(None);
                };
                let Ok(text) = std::fs::read_to_string(path) else {
                    return Ok(None);
                };
                (text, None)
            };
            let text_edits = replacements
                .into_iter()
                .map(|replacement| {
                    OneOf::Left(TextEdit {
                        range: range_for_public_span(&text, &replacement.span, &encoding),
                        new_text: replacement.text,
                    })
                })
                .collect();
            edits.push(TextDocumentEdit {
                text_document: OptionalVersionedTextDocumentIdentifier {
                    uri: edit_uri,
                    version,
                },
                edits: text_edits,
            });
        }
        Ok(Some(WorkspaceEdit {
            document_changes: Some(DocumentChanges::Edits(edits)),
            ..Default::default()
        }))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri;
        let Some(document) = self.documents.read().await.get(&uri).cloned() else {
            return Ok(None);
        };
        let syntax = self
            .tooling
            .lock()
            .expect("tooling engine lock")
            .syntax(&document.snapshot_id, &uri.to_string(), None)
            .ok();
        let Some(syntax) = syntax else {
            return Ok(None);
        };
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let mut symbols = Vec::new();
        collect_document_symbols(&syntax.root, &document.text, &encoding, &mut symbols);
        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }
    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;
        let Some(document) = self.documents.read().await.get(&uri).cloned() else {
            return Ok(None);
        };
        let formatted = self
            .tooling
            .lock()
            .expect("tooling engine lock")
            .format(&document.snapshot_id, &uri.to_string())
            .ok();
        let Some(formatted) = formatted else {
            return Ok(None);
        };
        if !formatted.changed {
            return Ok(Some(Vec::new()));
        }
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        let edit = TextEdit {
            range: Range::new(
                Position::new(0, 0),
                position_for_offset(&document.text, document.text.len(), &encoding),
            ),
            new_text: formatted.text,
        };
        Ok(Some(vec![CodeActionOrCommand::CodeAction(CodeAction {
            title: "Format Terrane document".to_owned(),
            kind: Some(CodeActionKind::SOURCE),
            edit: Some(WorkspaceEdit {
                changes: Some(HashMap::from([(uri, vec![edit])])),
                ..WorkspaceEdit::default()
            }),
            ..CodeAction::default()
        })]))
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = params.text_document.uri;
        let Some(document) = self.documents.read().await.get(&uri).cloned() else {
            return Ok(None);
        };
        let formatted = self
            .tooling
            .lock()
            .expect("tooling engine lock")
            .format(&document.snapshot_id, &uri.to_string())
            .ok();
        let Some(formatted) = formatted else {
            return Ok(None);
        };
        if !formatted.changed {
            return Ok(Some(Vec::new()));
        }
        let encoding = self
            .position_encoding
            .lock()
            .expect("position encoding lock")
            .clone();
        Ok(Some(vec![TextEdit {
            range: Range::new(
                Position::new(0, 0),
                position_for_offset(&document.text, document.text.len(), &encoding),
            ),
            new_text: formatted.text,
        }]))
    }
}

fn projected_hover_content(
    projection: &terrane_compiler::projection::Projection,
    name: &str,
    namespace: Option<&str>,
) -> Option<String> {
    projection
        .dependencies
        .iter()
        .flat_map(|dependency| &dependency.items)
        .find(|item| item.name == name && namespace.is_none_or(|value| item.namespace == value))
        .map(|item| {
            let mut text = format!("`{}`", item.rust_path);
            if let Some(requirements) = projected_execution_requirements(item) {
                text.push_str("\n\n");
                text.push_str(&requirements);
            }
            if let Some(docs) = &item.docs {
                text.push_str("\n\n");
                text.push_str(docs);
            }
            text
        })
        .or_else(|| {
            projection
                .dependencies
                .iter()
                .flat_map(|dependency| {
                    dependency
                        .declined
                        .iter()
                        .map(move |item| (dependency, item))
                })
                .find(|(dependency, item)| {
                    item.rust_path.rsplit("::").next() == Some(name)
                        && namespace.is_none_or(|value| {
                            declined_namespace(dependency, &item.rust_path) == value
                        })
                })
                .map(|(_, item)| format!("`{}`\n\nNot projected: {}", item.rust_path, item.reason))
        })
}

fn semantic_hover(
    object: terrane_compiler::tooling::SemanticObject,
    text: &str,
    encoding: &PositionEncodingKind,
) -> Option<Hover> {
    let name = object.name?;
    let mut content = format!("`{name}`");
    if let terrane_compiler::tooling::Availability::Known(value_type) = object.value_type {
        let _ = write!(content, "\n\nType: `{value_type}`");
    }
    if let terrane_compiler::tooling::Availability::Known(ownership) = object.ownership {
        let _ = write!(content, "\n\nOwnership: {ownership}");
    }
    if let terrane_compiler::tooling::Availability::Known(identity) = object.symbol_identity {
        let _ = write!(content, "\n\nSymbol: `{identity}`");
    }
    Some(Hover {
        contents: HoverContents::Scalar(MarkedString::String(content)),
        range: Some(range_for_public_span(text, &object.span, encoding)),
    })
}
type SnapshotInputs = (
    Vec<terrane_compiler::tooling::SourceInput>,
    Option<terrane_compiler::tooling::SourceInput>,
    bool,
);

fn snapshot_inputs(
    current_uri: &Uri,
    current_text: &str,
    overlays: &HashMap<String, String>,
) -> SnapshotInputs {
    package_snapshot_inputs(current_uri, current_text, overlays).unwrap_or_else(|| {
        (
            vec![terrane_compiler::tooling::SourceInput {
                uri: current_uri.to_string(),
                text: current_text.to_owned(),
            }],
            None,
            false,
        )
    })
}

fn package_snapshot_inputs(
    current_uri: &Uri,
    current_text: &str,
    overlays: &HashMap<String, String>,
) -> Option<SnapshotInputs> {
    let current_path = std::fs::canonicalize(PathBuf::from(
        current_uri.to_string().strip_prefix("file://")?,
    ))
    .ok()?;
    let manifest_path = current_path
        .parent()?
        .ancestors()
        .map(|directory| directory.join(terrane_compiler::package::MANIFEST_FILE_NAME))
        .find(|candidate| candidate.is_file())?;
    let package = terrane_compiler::Package::load(&manifest_path).ok()?;
    let mut sources = Vec::with_capacity(package.units.len());
    for unit in package.units {
        let path = std::fs::canonicalize(unit.source.path()).ok()?;
        let uri = format!("file://{}", path.display());
        let text = if path == current_path {
            current_text.to_owned()
        } else {
            overlays
                .get(&uri)
                .cloned()
                .unwrap_or_else(|| unit.source.text().to_owned())
        };
        sources.push(terrane_compiler::tooling::SourceInput { uri, text });
    }
    if sources
        .iter()
        .any(|source| source.uri == current_uri.to_string())
    {
        let manifest_path = std::fs::canonicalize(manifest_path).ok()?;
        let manifest = terrane_compiler::tooling::SourceInput {
            uri: format!("file://{}", manifest_path.display()),
            text: std::fs::read_to_string(manifest_path).ok()?,
        };
        return Some((sources, Some(manifest), false));
    }
    let test_package = terrane_compiler::testing::TestPackage::load(&manifest_path).ok()?;
    let package = test_package.tier_packages.into_values().find(|package| {
        package.units.iter().any(|unit| {
            std::fs::canonicalize(unit.source.path()).is_ok_and(|path| path == current_path)
        })
    })?;
    let mut sources = Vec::with_capacity(package.units.len());
    for unit in package.units {
        let path = std::fs::canonicalize(unit.source.path()).ok()?;
        let uri = format!("file://{}", path.display());
        let text = if path == current_path {
            current_text.to_owned()
        } else {
            overlays
                .get(&uri)
                .cloned()
                .unwrap_or_else(|| unit.source.text().to_owned())
        };
        sources.push(terrane_compiler::tooling::SourceInput { uri, text });
    }
    Some((sources, None, true))
}

fn byte_offset(text: &str, position: Position, encoding: &PositionEncodingKind) -> Option<usize> {
    let line = text
        .split_inclusive('\n')
        .nth(usize::try_from(position.line).ok()?)?;
    let line_start = text
        .split_inclusive('\n')
        .take(usize::try_from(position.line).ok()?)
        .map(str::len)
        .sum::<usize>();
    let line = line.trim_end_matches(['\r', '\n']);
    let target = usize::try_from(position.character).ok()?;
    if target == 0 {
        return Some(line_start);
    }
    let mut units = 0;
    for (byte, character) in line.char_indices() {
        if units == target {
            return Some(line_start + byte);
        }
        units += position_units(character, encoding);
        if units > target {
            return None;
        }
    }
    (units == target).then_some(line_start + line.len())
}

fn position_for_offset(text: &str, offset: usize, encoding: &PositionEncodingKind) -> Position {
    let line = text[..offset].bytes().filter(|byte| *byte == b'\n').count();
    let line_start = text[..offset].rfind('\n').map_or(0, |index| index + 1);
    let character = text[line_start..offset]
        .chars()
        .map(|character| position_units(character, encoding))
        .sum::<usize>();
    Position::new(
        u32::try_from(line).expect("document line fits in LSP position"),
        u32::try_from(character).expect("document column fits in LSP position"),
    )
}

fn position_units(character: char, encoding: &PositionEncodingKind) -> usize {
    if *encoding == PositionEncodingKind::UTF8 {
        character.len_utf8()
    } else if *encoding == PositionEncodingKind::UTF32 {
        1
    } else {
        character.len_utf16()
    }
}

fn range_for_public_span(
    text: &str,
    span: &terrane_compiler::tooling::PublicSpan,
    encoding: &PositionEncodingKind,
) -> Range {
    Range::new(
        position_for_offset(text, span.start, encoding),
        position_for_offset(text, span.end, encoding),
    )
}

async fn lsp_location(
    location: &terrane_compiler::tooling::Location,
    documents: &RwLock<HashMap<Uri, Document>>,
    encoding: &PositionEncodingKind,
) -> Option<LspLocation> {
    let uri = location.uri.parse::<Uri>().ok()?;
    let text = if let Some(document) = documents.read().await.get(&uri) {
        document.text.clone()
    } else {
        std::fs::read_to_string(location.uri.strip_prefix("file://")?).ok()?
    };
    Some(LspLocation {
        uri,
        range: range_for_public_span(&text, &location.span, encoding),
    })
}

fn tooling_lsp_diagnostic(
    text: &str,
    diagnostic: &terrane_compiler::tooling::DiagnosticProjection,
    encoding: &PositionEncodingKind,
) -> Diagnostic {
    let range = diagnostic.span.as_ref().map_or_else(
        || Range::new(Position::new(0, 0), Position::new(0, 0)),
        |span| range_for_public_span(text, span, encoding),
    );
    let message = diagnostic.help.as_ref().map_or_else(
        || diagnostic.message.clone(),
        |help| format!("{}\n\nhelp: {help}", diagnostic.message),
    );
    Diagnostic {
        range,
        severity: Some(if diagnostic.severity == "warning" {
            DiagnosticSeverity::WARNING
        } else {
            DiagnosticSeverity::ERROR
        }),
        code: Some(NumberOrString::String(diagnostic.code.clone())),
        source: Some("terrane".to_owned()),
        message,
        ..Default::default()
    }
}

#[allow(deprecated)]
fn collect_document_symbols(
    node: &terrane_compiler::tooling::SyntaxNodeProjection,
    text: &str,
    encoding: &PositionEncodingKind,
    output: &mut Vec<DocumentSymbol>,
) {
    let kind = match node.kind.as_str() {
        "FunctionDeclaration" => Some(SymbolKind::FUNCTION),
        "ClassDeclaration" => Some(SymbolKind::CLASS),
        "InterfaceDeclaration" => Some(SymbolKind::INTERFACE),
        "TraitDeclaration" => Some(SymbolKind::STRUCT),
        "Binding" => Some(SymbolKind::VARIABLE),
        _ => None,
    };
    if let Some(kind) = kind
        && let Some(name) = node.children.iter().find(|child| child.field == "name")
        && let Some(name_text) = text.get(name.node.span.start..name.node.span.end)
    {
        output.push(DocumentSymbol {
            name: name_text.to_owned(),
            detail: None,
            kind,
            tags: None,
            deprecated: None,
            range: range_for_public_span(text, &node.span, encoding),
            selection_range: range_for_public_span(text, &name.node.span, encoding),
            children: None,
        });
    }
    for child in &node.children {
        collect_document_symbols(&child.node, text, encoding, output);
    }
}

fn projected_item_detail(item: &terrane_compiler::projection::ProjectedItem) -> String {
    let mut details = vec![item.rust_path.clone()];
    if matches!(
        &item.kind,
        terrane_compiler::projection::ProjectedKind::Function(function)
            if function.chain_role.is_some()
    ) || matches!(
        &item.kind,
        terrane_compiler::projection::ProjectedKind::ForeignType { methods, .. }
            if methods.iter().any(|method| method.chain_role.is_some())
    ) {
        details.push("chain-only; must terminate within one expression".to_owned());
    }
    if let Some(requirements) = projected_execution_requirements(item) {
        details.push(requirements);
    }
    details.join(" — ")
}

fn projected_execution_requirements(
    item: &terrane_compiler::projection::ProjectedItem,
) -> Option<String> {
    let terrane_compiler::projection::ProjectedKind::Function(function) = &item.kind else {
        return None;
    };
    let requirements = function.execution_requirements?;
    Some(format!(
        "async Terrane task; runtime context {}; wake support {}; transfer {}",
        requirement_knowledge(requirements.runtime_context),
        requirement_knowledge(requirements.wake_support),
        requirement_knowledge(requirements.transfer)
    ))
}

fn requirement_knowledge(
    knowledge: terrane_compiler::projection::RequirementKnowledge,
) -> &'static str {
    match knowledge {
        terrane_compiler::projection::RequirementKnowledge::Required => "required",
        terrane_compiler::projection::RequirementKnowledge::NotRequired => "not required",
        terrane_compiler::projection::RequirementKnowledge::Unknown => "unknown",
    }
}

#[must_use]
pub fn encode_semantic_tokens(text: &str, highlights: &[Highlight]) -> Vec<SemanticToken> {
    let mut encoded = Vec::with_capacity(highlights.len());
    let mut previous_line = 0_u32;
    let mut previous_start = 0_u32;
    for highlight in highlights {
        for span in split_lines(text, highlight.span) {
            let (line, start, length) = utf16_range(text, span);
            let delta_line = line - previous_line;
            let delta_start = if delta_line == 0 {
                start - previous_start
            } else {
                start
            };
            encoded.push(SemanticToken {
                delta_line,
                delta_start,
                length,
                token_type: token_type(highlight.kind),
                token_modifiers_bitset: u32::from(highlight.declaration),
            });
            previous_line = line;
            previous_start = start;
        }
    }
    encoded
}

fn split_lines(text: &str, span: Span) -> impl Iterator<Item = Span> + '_ {
    let file = span.file;
    text[span.start..span.end]
        .split_inclusive('\n')
        .scan(span.start, move |start, part| {
            let content_end = *start + part.trim_end_matches(['\n', '\r']).len();
            let result = (*start < content_end).then_some(Span::new(file, *start, content_end));
            *start += part.len();
            Some(result)
        })
        .flatten()
}

fn utf16_range(text: &str, span: Span) -> (u32, u32, u32) {
    let start = utf16_position(text, span.start);
    let length = text[span.start..span.end].encode_utf16().count();
    (
        start.line,
        start.character,
        u32::try_from(length).expect("token length fits in LSP position"),
    )
}

fn utf16_position(text: &str, offset: usize) -> Position {
    let line = text[..offset].bytes().filter(|byte| *byte == b'\n').count();
    let line_start = text[..offset].rfind('\n').map_or(0, |index| index + 1);
    let character = text[line_start..offset].encode_utf16().count();
    Position::new(
        u32::try_from(line).expect("document line fits in LSP position"),
        u32::try_from(character).expect("document column fits in LSP position"),
    )
}

#[cfg(test)]
fn lsp_diagnostic(source: &SourceFile, diagnostic: &TerraneDiagnostic) -> Diagnostic {
    let range = diagnostic.primary.map_or_else(
        || Range::new(Position::new(0, 0), Position::new(0, 0)),
        |span| {
            Range::new(
                utf16_position(source.text(), span.start),
                utf16_position(source.text(), span.end),
            )
        },
    );
    let message = diagnostic.help.as_ref().map_or_else(
        || diagnostic.message.clone(),
        |help| format!("{}\n\nhelp: {help}", diagnostic.message),
    );
    Diagnostic {
        range,
        severity: Some(match diagnostic.severity {
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
        }),
        code: Some(NumberOrString::String(diagnostic.code.to_owned())),
        source: Some("terrane".to_owned()),
        message,
        ..Default::default()
    }
}

type ProjectionStamp = Vec<(PathBuf, std::time::SystemTime)>;
type ProjectionDemands = BTreeSet<(String, String)>;
type CachedProjection = (
    ProjectionStamp,
    ProjectionDemands,
    terrane_compiler::projection::Projection,
);

static PROJECTIONS: LazyLock<Mutex<HashMap<PathBuf, CachedProjection>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn projection_stamp(paths: &[PathBuf]) -> Option<ProjectionStamp> {
    paths
        .iter()
        .map(|path| Some((path.clone(), path.metadata().ok()?.modified().ok()?)))
        .collect()
}

async fn projection_for_uri(
    uri: &Uri,
    document_text: &str,
) -> Option<terrane_compiler::projection::Projection> {
    let path = uri.to_file_path()?.into_owned();
    let document_text = document_text.to_owned();
    tokio::task::spawn_blocking(move || {
        let manifest = path
            .ancestors()
            .map(|directory| directory.join(terrane_compiler::MANIFEST_FILE_NAME))
            .find(|candidate| candidate.is_file())?;
        let mut package = terrane_compiler::Package::load(&manifest).ok()?;
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.clone());
        let unit = package.units.iter_mut().find(|unit| {
            unit.source
                .path()
                .canonicalize()
                .unwrap_or_else(|_| unit.source.path().to_path_buf())
                == canonical_path
        })?;
        unit.source = SourceFile::new(
            unit.source.id(),
            unit.source.path().to_path_buf(),
            document_text,
        );
        let demands = terrane_compiler::semantics::dependency_projection_demands(&package).ok()?;
        let mut stamp_paths = package.dependency_manifests.clone();
        stamp_paths.extend(
            package
                .units
                .iter()
                .map(|unit| unit.source.path().to_path_buf()),
        );
        let stamp = projection_stamp(&stamp_paths)?;
        if let Some((cached_stamp, cached_demands, projection)) = PROJECTIONS
            .lock()
            .expect("projection cache lock is not poisoned")
            .get(&manifest)
            .cloned()
            && cached_stamp == stamp
            && cached_demands == demands
        {
            return Some(projection);
        }
        let projection = terrane_compiler::projection::resolve(
            &package.root,
            &package.rust_dependencies,
            Some(&demands),
        )
        .ok()?;
        PROJECTIONS
            .lock()
            .expect("projection cache lock is not poisoned")
            .insert(manifest, (stamp, demands, projection.clone()));
        Some(projection)
    })
    .await
    .ok()
    .flatten()
}

fn declined_namespace(
    dependency: &terrane_compiler::projection::ProjectedDependency,
    rust_path: &str,
) -> String {
    terrane_compiler::projection::namespace_for_rust_path(dependency, rust_path)
}

fn dependency_import_namespace(text: &str, position: Position) -> Option<String> {
    let line = line_prefix(text, position)?;
    let path = line.strip_prefix("from ")?.split(" import").next()?;
    path.starts_with("/deps/").then(|| path.to_owned())
}

fn imported_dependency_namespace(text: &str, name: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (path, imported) = line.strip_prefix("from ")?.split_once(" import ")?;
        if !path.starts_with("/deps/") {
            return None;
        }
        imported
            .split(',')
            .map(str::trim)
            .any(|item| {
                item == name
                    || item
                        .rsplit_once(" as ")
                        .is_some_and(|(_, alias)| alias == name)
            })
            .then(|| path.to_owned())
    })
}

fn word_at(text: &str, position: Position) -> Option<&str> {
    let line = text.lines().nth(usize::try_from(position.line).ok()?)?;
    let byte = line_prefix(text, position)?.len();
    let is_name =
        |character: char| character.is_ascii_alphanumeric() || matches!(character, '_' | '-');
    let start = line[..byte]
        .char_indices()
        .rev()
        .find(|(_, character)| !is_name(*character))
        .map_or(0, |(index, character)| index + character.len_utf8());
    let end = line[byte..]
        .char_indices()
        .find(|(_, character)| !is_name(*character))
        .map_or(line.len(), |(index, _)| byte + index);
    (start < end).then_some(&line[start..end])
}

fn call_name_before(text: &str, position: Position) -> Option<&str> {
    let prefix = line_prefix(text, position)?;
    let callee = prefix.rsplit_once(';')?.0.trim_end();
    callee
        .rsplit(|character: char| {
            !(character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.'))
        })
        .next()?
        .rsplit('.')
        .next()
}

fn line_prefix(text: &str, position: Position) -> Option<&str> {
    let line = text.lines().nth(usize::try_from(position.line).ok()?)?;
    let mut utf16 = 0_u32;
    let mut end = 0;
    for (index, character) in line.char_indices() {
        if utf16 >= position.character {
            break;
        }
        utf16 += u32::try_from(character.len_utf16()).ok()?;
        end = index + character.len_utf8();
    }
    Some(&line[..end])
}

fn source_file(uri: &Uri, text: &str) -> SourceFile {
    let path = uri
        .to_file_path()
        .map_or_else(|| PathBuf::from(uri.as_str()), std::borrow::Cow::into_owned);
    SourceFile::new(0, path, text.to_owned())
}

const fn token_type(kind: HighlightKind) -> u32 {
    match kind {
        HighlightKind::Comment => 0,
        HighlightKind::Keyword => 1,
        HighlightKind::Number => 2,
        HighlightKind::String => 3,
        HighlightKind::Operator => 4,
        HighlightKind::Namespace => 5,
        HighlightKind::Type => 6,
        HighlightKind::Function => 7,
        HighlightKind::Parameter => 8,
        HighlightKind::Property => 9,
        HighlightKind::Variable => 10,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negotiated_position_encodings_round_trip_unicode_offsets() {
        let text = "a🙂é\nnext";
        let offset = text.find('é').expect("unicode character");
        for (encoding, character) in [
            (PositionEncodingKind::UTF8, 5),
            (PositionEncodingKind::UTF16, 3),
            (PositionEncodingKind::UTF32, 2),
        ] {
            let position = position_for_offset(text, offset, &encoding);
            assert_eq!(position, Position::new(0, character));
            assert_eq!(byte_offset(text, position, &encoding), Some(offset));
        }
    }

    #[test]
    fn document_symbols_come_from_compiler_snapshot_fields() {
        let text = "namespace symbols\n\nasync function main;\n    value int = 1\n";
        let uri = "file:///workspace/symbols.trn";
        let mut tooling = terrane_compiler::tooling::ToolingEngine::default();
        let snapshot = tooling
            .open_snapshot(
                vec![terrane_compiler::tooling::SourceInput {
                    uri: uri.to_owned(),
                    text: text.to_owned(),
                }],
                None,
                None,
                terrane_compiler::tooling::SnapshotOptions::default(),
            )
            .expect("snapshot");
        let syntax = tooling
            .syntax(&snapshot.snapshot_id, uri, None)
            .expect("syntax");
        let mut symbols = Vec::new();
        collect_document_symbols(
            &syntax.root,
            text,
            &PositionEncodingKind::UTF16,
            &mut symbols,
        );

        assert!(symbols.iter().any(|symbol| symbol.name == "main"));
        assert!(symbols.iter().any(|symbol| symbol.name == "value"));
        assert!(!symbols.iter().any(|symbol| symbol.name == "async"));
    }

    #[test]
    fn diagnostics_use_utf16_positions_at_both_ends_of_multiline_spans() {
        let source = SourceFile::new(0, "editor.trn".into(), "🙂 start\nend".to_owned());
        let diagnostic =
            TerraneDiagnostic::error("S0000", "multiline", Span::new(0, 0, source.text().len()));

        let converted = lsp_diagnostic(&source, &diagnostic);

        assert_eq!(converted.range.start, Position::new(0, 0));
        assert_eq!(converted.range.end, Position::new(1, 3));
    }

    #[test]
    fn diagnostics_retain_compiler_help() {
        let source = SourceFile::new(0, "editor.trn".into(), "bad".to_owned());
        let diagnostic = TerraneDiagnostic::error("S0000", "invalid source", Span::new(0, 0, 3))
            .with_help("replace it");

        let converted = lsp_diagnostic(&source, &diagnostic);

        assert_eq!(converted.message, "invalid source\n\nhelp: replace it");
    }

    #[test]
    fn projected_async_completion_describes_execution_requirements() {
        use terrane_compiler::projection::{
            ProjectedExecutionRequirements, ProjectedFunction, ProjectedItem, ProjectedKind,
            ProjectedType, RequirementKnowledge,
        };

        let item = ProjectedItem {
            namespace: "/deps/witness".to_owned(),
            name: "wait".to_owned(),
            rust_path: "witness::wait".to_owned(),
            docs: None,
            kind: ProjectedKind::Function(ProjectedFunction {
                name: "wait".to_owned(),
                parameters: Vec::new(),
                generic_parameters: Vec::new(),
                result: ProjectedType::None,
                destination_result: None,
                error: None,
                is_async: true,
                into_future: false,
                execution_requirements: Some(ProjectedExecutionRequirements {
                    runtime_context: RequirementKnowledge::Unknown,
                    wake_support: RequirementKnowledge::Required,
                    transfer: RequirementKnowledge::Unknown,
                }),
                enum_operation: None,
                error_optional_depth: 0,
                chain_role: None,
                receiver: None,
            }),
        };

        assert_eq!(
            projected_item_detail(&item),
            "witness::wait — async Terrane task; runtime context unknown; wake support required; transfer unknown"
        );
    }

    #[test]
    fn projected_chain_completion_exposes_non_escaping_constraint() {
        use terrane_compiler::projection::{
            ChainRole, ProjectedFunction, ProjectedItem, ProjectedKind, ProjectedType,
        };

        let item = ProjectedItem {
            namespace: "/deps/witness".to_owned(),
            name: "builder".to_owned(),
            rust_path: "witness::builder".to_owned(),
            docs: None,
            kind: ProjectedKind::Function(ProjectedFunction {
                name: "builder".to_owned(),
                parameters: Vec::new(),
                generic_parameters: Vec::new(),
                result: ProjectedType::None,
                destination_result: None,
                error: None,
                is_async: false,
                into_future: false,
                execution_requirements: None,
                enum_operation: None,
                error_optional_depth: 0,
                chain_role: Some(ChainRole::Root),
                receiver: None,
            }),
        };

        assert_eq!(
            projected_item_detail(&item),
            "witness::builder — chain-only; must terminate within one expression"
        );
    }
    #[test]
    fn package_snapshots_resolve_definitions_across_source_units() {
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repository root");
        let child_path = repository
            .join("tests/conformance/check/parent-namespace-function/app/child/child.trn")
            .canonicalize()
            .expect("child fixture");
        let disk_child = std::fs::read_to_string(&child_path).expect("child source");
        let child_text = format!("{disk_child}\n# unsaved editor overlay\n");
        let child_uri = format!("file://{}", child_path.display());
        let uri = child_uri.parse::<Uri>().expect("file URI");
        let (sources, manifest, _) = package_snapshot_inputs(&uri, &child_text, &HashMap::new())
            .expect("package snapshot inputs");
        assert_eq!(sources.len(), 2);
        assert_eq!(
            sources
                .iter()
                .find(|source| source.uri == child_uri)
                .map(|source| source.text.as_str()),
            Some(child_text.as_str())
        );

        let mut tooling = terrane_compiler::tooling::ToolingEngine::default();
        let snapshot = tooling
            .open_snapshot(
                sources,
                manifest,
                None,
                terrane_compiler::tooling::SnapshotOptions {
                    semantic: true,
                    ..terrane_compiler::tooling::SnapshotOptions::default()
                },
            )
            .expect("semantic package snapshot");
        assert_eq!(snapshot.profile, "default");
        assert!(matches!(
            snapshot.build_id,
            terrane_compiler::tooling::Availability::NotYetAnalyzed
        ));
        assert!(matches!(
            snapshot.dependency_projection,
            terrane_compiler::tooling::Availability::Known(_)
        ));
        let use_offset = child_text.find("double").expect("parent function use");
        let terrane_compiler::tooling::Availability::Known(definition) = tooling
            .definition(&snapshot.snapshot_id, &child_uri, use_offset)
            .expect("cross-file definition")
        else {
            panic!("cross-file definition should be known");
        };
        assert_ne!(definition.uri, child_uri);
        assert!(definition.uri.ends_with("/app/main.trn"));
    }

    #[test]
    fn package_snapshots_include_and_analyze_terrane_library_sources() {
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repository root");
        let app_path = repository
            .join("tests/conformance/check/terrane-library-package/src/main.trn")
            .canonicalize()
            .expect("library consumer fixture");
        let app_text = std::fs::read_to_string(&app_path).expect("application source");
        let app_uri = format!("file://{}", app_path.display());
        let uri = app_uri.parse::<Uri>().expect("file URI");
        let (sources, manifest, _) =
            package_snapshot_inputs(&uri, &app_text, &HashMap::new()).expect("package inputs");
        assert!(
            sources
                .iter()
                .any(|source| source.uri.ends_with("/library/src/library.trn"))
        );
        assert_eq!(sources.len(), 2);

        let mut tooling = terrane_compiler::tooling::ToolingEngine::default();
        tooling
            .open_snapshot(
                sources,
                manifest,
                None,
                terrane_compiler::tooling::SnapshotOptions {
                    semantic: true,
                    ..terrane_compiler::tooling::SnapshotOptions::default()
                },
            )
            .expect("semantic library snapshot");
    }
}
