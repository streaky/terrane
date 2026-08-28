use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};

use terrane_compiler::highlight::{Highlight, HighlightKind, highlight};
use terrane_compiler::{Diagnostic as TerraneDiagnostic, Severity, SourceFile, Span};
use tokio::sync::RwLock;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse,
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, Documentation, Hover, HoverContents, HoverParams,
    HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams, MarkedString,
    MessageType, NumberOrString, ParameterInformation, ParameterLabel, Position, Range,
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams,
    SemanticTokensResult, SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo,
    SignatureHelp, SignatureHelpOptions, SignatureHelpParams, SignatureInformation,
    TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
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
}

#[derive(Debug)]
pub struct Backend {
    client: Client,
    documents: Arc<RwLock<HashMap<Uri, Document>>>,
}

impl Backend {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn analyze(&self, uri: &Uri, text: &str, version: i32) {
        let source = source_file(uri, text);
        let output = highlight(&source);
        let diagnostics = output
            .diagnostics
            .iter()
            .map(|diagnostic| lsp_diagnostic(&source, diagnostic))
            .collect();
        self.client
            .publish_diagnostics(uri.clone(), diagnostics, Some(version))
            .await;
    }
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
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

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        let version = params.text_document.version;
        self.documents.write().await.insert(
            uri.clone(),
            Document {
                text: text.clone(),
                version,
            },
        );
        self.analyze(&uri, &text, version).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let Some(change) = params.content_changes.into_iter().last() else {
            return;
        };
        let uri = params.text_document.uri;
        let text = change.text;
        let version = params.text_document.version;
        self.documents.write().await.insert(
            uri.clone(),
            Document {
                text: text.clone(),
                version,
            },
        );
        self.analyze(&uri, &text, version).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        self.documents.write().await.remove(&uri);
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
        let Some(projection) = projection_for_uri(&uri).await else {
            return Ok(None);
        };
        let namespace = self.documents.read().await.get(&uri).and_then(|document| {
            dependency_import_namespace(&document.text, params.text_document_position.position)
        });
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
                detail: Some(item.rust_path.clone()),
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
        let documents = self.documents.read().await;
        let Some(document) = documents.get(&uri) else {
            return Ok(None);
        };
        let Some(name) = word_at(
            &document.text,
            params.text_document_position_params.position,
        ) else {
            return Ok(None);
        };
        let Some(projection) = projection_for_uri(&uri).await else {
            return Ok(None);
        };
        let namespace = imported_dependency_namespace(&document.text, name);
        let content = projection
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| {
                item.name == name
                    && namespace
                        .as_deref()
                        .is_none_or(|value| item.namespace == value)
            })
            .map(|item| {
                let mut text = format!("`{}`", item.rust_path);
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
                            && namespace.as_deref().is_none_or(|value| {
                                declined_namespace(dependency, &item.rust_path) == value
                            })
                    })
                    .map(|(_, item)| {
                        format!("`{}`\n\nNot projected: {}", item.rust_path, item.reason)
                    })
            });
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
        let Some(projection) = projection_for_uri(&uri).await else {
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

type CachedProjection = (
    std::time::SystemTime,
    terrane_compiler::projection::Projection,
);

static PROJECTIONS: LazyLock<Mutex<HashMap<PathBuf, CachedProjection>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

async fn projection_for_uri(uri: &Uri) -> Option<terrane_compiler::projection::Projection> {
    let path = uri.to_file_path()?.into_owned();
    tokio::task::spawn_blocking(move || {
        let manifest = path
            .ancestors()
            .map(|directory| directory.join(terrane_compiler::MANIFEST_FILE_NAME))
            .find(|candidate| candidate.is_file())?;
        let modified = manifest.metadata().ok()?.modified().ok()?;
        if let Some((cached_at, projection)) = PROJECTIONS
            .lock()
            .expect("projection cache lock is not poisoned")
            .get(&manifest)
        {
            if *cached_at == modified {
                return Some(projection.clone());
            }
        }
        let package = terrane_compiler::Package::load(&manifest).ok()?;
        let projection =
            terrane_compiler::projection::resolve(&package.root, &package.rust_dependencies)
                .ok()?;
        PROJECTIONS
            .lock()
            .expect("projection cache lock is not poisoned")
            .insert(manifest, (modified, projection.clone()));
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
}
