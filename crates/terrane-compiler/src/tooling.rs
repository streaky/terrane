use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::package::{
    BuildToolchain, CapabilityProfile, ExecutorProfile, Package, PanicProfile, ReflectionProfile,
    SourceUnit,
};
use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxTree};
use crate::tokens::{TokenKind, TriviaKind};
use crate::{Diagnostic, SourceFile, Span};

pub const SCHEMA_VERSION: &str = "1.0";
const DEFAULT_MAX_SNAPSHOTS: usize = 32;
const DEFAULT_MAX_BYTES: usize = 64 * 1024 * 1024;
const DEFAULT_PAGE_SIZE: usize = 100;
const MAX_PAGE_SIZE: usize = 1_000;

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Availability<T> {
    Known(T),
    Unresolved,
    Invalid,
    NotYetAnalyzed,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct PublicSpan {
    pub start: usize,
    pub end: usize,
}

impl From<Span> for PublicSpan {
    fn from(span: Span) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SourceInput {
    pub uri: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SnapshotOptions {
    #[serde(default)]
    pub semantic: bool,
    #[serde(default)]
    pub generated: bool,
    #[serde(default = "default_target")]
    pub target: String,
    #[serde(default = "default_profile")]
    pub profile: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

fn default_target() -> String {
    "host".to_owned()
}

fn default_profile() -> String {
    "development".to_owned()
}

impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            semantic: false,
            generated: false,
            target: default_target(),
            profile: default_profile(),
            capabilities: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SnapshotMetadata {
    pub compiler_version: String,
    pub schema_version: String,
    pub snapshot_id: String,
    pub sources: Vec<SourceIdentity>,
    pub manifest_hash: Availability<String>,
    pub lock_hash: Availability<String>,
    pub target: String,
    pub profile: String,
    pub capabilities: Vec<String>,
    pub build_id: Availability<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SourceIdentity {
    pub uri: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct DiagnosticProjection {
    pub severity: String,
    pub code: String,
    pub message: String,
    pub span: Option<PublicSpan>,
    pub help: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct TokenProjection {
    pub kind: String,
    pub text: String,
    pub span: PublicSpan,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct TriviaProjection {
    pub kind: String,
    pub text: String,
    pub span: PublicSpan,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SyntaxState {
    Complete,
    Error,
    Recovery,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SyntaxChild {
    pub field: String,
    pub node: SyntaxNodeProjection,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SyntaxNodeProjection {
    pub id: u64,
    pub kind: String,
    pub span: PublicSpan,
    pub state: SyntaxState,
    pub children: Vec<SyntaxChild>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SyntaxProjection {
    pub source: SourceIdentity,
    pub diagnostics: Vec<DiagnosticProjection>,
    pub root: SyntaxNodeProjection,
    pub tokens: Vec<TokenProjection>,
    pub trivia: Vec<TriviaProjection>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct SemanticObject {
    pub source_uri: String,
    pub span: PublicSpan,
    pub syntax_node_id: u64,
    pub kind: String,
    pub name: Option<String>,
    pub symbol_identity: Availability<String>,
    pub descriptor_identity: Availability<String>,
    pub value_type: Availability<String>,
    pub ownership: Availability<String>,
    pub effects: Availability<Vec<String>>,
    pub capabilities: Availability<Vec<String>>,
    pub declaration: Availability<Location>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    pub uri: String,
    pub span: PublicSpan,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Selector {
    pub kind: Option<String>,
    pub child_field: Option<String>,
    pub containing: Option<PublicSpan>,
    pub text: Option<String>,
    pub token_kind: Option<String>,
    pub symbol_identity: Option<String>,
    pub descriptor_identity: Option<String>,
    #[serde(default)]
    pub include_recovery: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct QueryPage {
    pub matches: Vec<SemanticObject>,
    pub complete: bool,
    pub continuation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct GeneratedLocation {
    pub build_id: String,
    pub path: String,
    pub span: PublicSpan,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Replacement {
    pub uri: String,
    pub span: PublicSpan,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct AffectedFile {
    pub uri: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct EditProposal {
    pub proposal_id: String,
    pub snapshot_id: String,
    pub affected_files: Vec<AffectedFile>,
    pub replacements: Vec<Replacement>,
    pub preview_diagnostics: Vec<DiagnosticProjection>,
    pub semantic_reanalysis: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ApplyReport {
    pub committed: Vec<String>,
    pub uncommitted: Vec<String>,
    pub recovery_files: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct FormatResult {
    pub uri: String,
    pub changed: bool,
    pub text: String,
    pub edits: Vec<Replacement>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestEnvelope {
    pub schema_version: String,
    pub request_id: String,
    #[serde(flatten)]
    pub request: Request,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "operation", rename_all = "kebab-case")]
pub enum Request {
    OpenSnapshot {
        sources: Vec<SourceInput>,
        #[serde(default)]
        manifest: Option<SourceInput>,
        #[serde(default)]
        lock: Option<SourceInput>,
        #[serde(default)]
        options: SnapshotOptions,
    },
    CloseSnapshot {
        snapshot_id: String,
    },
    Syntax {
        snapshot_id: String,
        uri: String,
        node_id: Option<u64>,
    },
    Locate {
        snapshot_id: String,
        uri: String,
        offset: usize,
    },
    Definition {
        snapshot_id: String,
        uri: String,
        offset: usize,
    },
    References {
        snapshot_id: String,
        uri: String,
        offset: usize,
    },
    Find {
        snapshot_id: String,
        selector: Selector,
        page_size: Option<usize>,
        continuation: Option<String>,
    },
    GeneratedRust {
        snapshot_id: String,
        uri: String,
        node_id: u64,
        build_id: String,
    },
    ProposeEdits {
        snapshot_id: String,
        replacements: Vec<Replacement>,
    },
    ProposeRename {
        snapshot_id: String,
        uri: String,
        offset: usize,
        new_name: String,
    },
    ApplyEdits {
        proposal_id: String,
    },
    Format {
        snapshot_id: String,
        uri: String,
    },
    Cancel {
        request_id: String,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseEnvelope {
    pub compiler_version: String,
    pub schema_version: String,
    pub request_id: String,
    pub snapshot_id: Option<String>,
    pub source_uri: Option<String>,
    pub source_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ProtocolError>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct ProtocolError {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub retry_fresh_query: bool,
}

impl ProtocolError {
    fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            message: message.into(),
            retry_fresh_query: false,
        }
    }

    fn stale(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            message: message.into(),
            retry_fresh_query: true,
        }
    }
}

#[derive(Clone, Debug)]
struct ParsedDocument {
    source: SourceFile,
    identity: SourceIdentity,
    tree: SyntaxTree,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Debug)]
struct GeneratedSnapshot {
    build_id: String,
    files: Vec<crate::rust_ir::RenderedFile>,
}

#[derive(Clone, Debug)]
struct Snapshot {
    metadata: SnapshotMetadata,
    documents: BTreeMap<String, ParsedDocument>,
    semantic: Option<crate::SemanticPackage>,
    semantic_invalid: bool,
    generated: Option<GeneratedSnapshot>,
    bytes: usize,
}

#[derive(Clone, Debug)]
struct Continuation {
    snapshot_id: String,
    selector_hash: String,
    offset: usize,
}

#[derive(Debug)]
pub struct ToolingEngine {
    snapshots: HashMap<String, Snapshot>,
    snapshot_order: VecDeque<String>,
    retained_bytes: usize,
    max_snapshots: usize,
    max_bytes: usize,
    continuations: HashMap<String, Continuation>,
    proposals: HashMap<String, EditProposal>,
    canceled: BTreeSet<String>,
    nonce: u64,
}

impl Default for ToolingEngine {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_SNAPSHOTS, DEFAULT_MAX_BYTES)
    }
}

impl ToolingEngine {
    #[must_use]
    pub fn new(max_snapshots: usize, max_bytes: usize) -> Self {
        Self {
            snapshots: HashMap::new(),
            snapshot_order: VecDeque::new(),
            retained_bytes: 0,
            max_snapshots: max_snapshots.max(1),
            max_bytes: max_bytes.max(1),
            continuations: HashMap::new(),
            proposals: HashMap::new(),
            canceled: BTreeSet::new(),
            nonce: 0,
        }
    }

    pub fn handle(&mut self, envelope: RequestEnvelope) -> ResponseEnvelope {
        let request_id = envelope.request_id.clone();
        if envelope.schema_version != SCHEMA_VERSION {
            return self.error_response(
                request_id,
                None,
                None,
                ProtocolError::new(
                    "unsupported-schema",
                    format!(
                        "schema `{}` is not supported; use `{SCHEMA_VERSION}`",
                        envelope.schema_version
                    ),
                ),
            );
        }
        if self.canceled.remove(&request_id) {
            return self.error_response(
                request_id,
                None,
                None,
                ProtocolError::new("canceled", "request was canceled"),
            );
        }
        let mut context = request_context(&envelope.request);
        let opened_source = match &envelope.request {
            Request::OpenSnapshot { sources, .. } => sources.first().map(|source| source.uri.clone()),
            _ => None,
        };
        let result = self.dispatch(envelope.request);
        match result {
            Ok(value) => {
                if context.0.is_none() {
                    context.0 = value
                        .get("snapshot_id")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned);
                    context.1 = opened_source;
                }
                self.success_response(request_id, context, value)
            }
            Err(error) => self.error_response(request_id, context.0, context.1, error),
        }
    }

    fn dispatch(&mut self, request: Request) -> Result<serde_json::Value, ProtocolError> {
        match request {
            Request::OpenSnapshot {
                sources,
                manifest,
                lock,
                options,
            } => serde_json::to_value(self.open_snapshot(sources, manifest, lock, options)?)
                .map_err(serialization_error),
            Request::CloseSnapshot { snapshot_id } => {
                self.close_snapshot(&snapshot_id)?;
                Ok(serde_json::json!({ "closed": true }))
            }
            Request::Syntax {
                snapshot_id,
                uri,
                node_id,
            } => serde_json::to_value(self.syntax(&snapshot_id, &uri, node_id)?)
                .map_err(serialization_error),
            Request::Locate {
                snapshot_id,
                uri,
                offset,
            } => serde_json::to_value(self.locate(&snapshot_id, &uri, offset)?)
                .map_err(serialization_error),
            Request::Definition {
                snapshot_id,
                uri,
                offset,
            } => serde_json::to_value(self.definition(&snapshot_id, &uri, offset)?)
                .map_err(serialization_error),
            Request::References {
                snapshot_id,
                uri,
                offset,
            } => serde_json::to_value(self.references(&snapshot_id, &uri, offset)?)
                .map_err(serialization_error),
            Request::Find {
                snapshot_id,
                selector,
                page_size,
                continuation,
            } => serde_json::to_value(self.find(
                &snapshot_id,
                &selector,
                page_size,
                continuation.as_deref(),
            )?)
            .map_err(serialization_error),
            Request::GeneratedRust {
                snapshot_id,
                uri,
                node_id,
                build_id,
            } => {
                serde_json::to_value(self.generated_rust(&snapshot_id, &uri, node_id, &build_id)?)
                    .map_err(serialization_error)
            }
            Request::ProposeEdits {
                snapshot_id,
                replacements,
            } => serde_json::to_value(self.propose_edits(&snapshot_id, replacements)?)
                .map_err(serialization_error),
            Request::ProposeRename {
                snapshot_id,
                uri,
                offset,
                new_name,
            } => {
                serde_json::to_value(self.propose_rename(&snapshot_id, &uri, offset, &new_name)?)
                    .map_err(serialization_error)
            }
            Request::ApplyEdits { proposal_id } => {
                serde_json::to_value(self.apply_edits(&proposal_id)?).map_err(serialization_error)
            }
            Request::Format { snapshot_id, uri } => {
                serde_json::to_value(self.format(&snapshot_id, &uri)?).map_err(serialization_error)
            }
            Request::Cancel { request_id } => {
                self.canceled.insert(request_id);
                Ok(serde_json::json!({ "canceled": true }))
            }
        }
    }

    pub fn open_snapshot(
        &mut self,
        mut sources: Vec<SourceInput>,
        manifest: Option<SourceInput>,
        lock: Option<SourceInput>,
        options: SnapshotOptions,
    ) -> Result<SnapshotMetadata, ProtocolError> {
        if sources.is_empty() {
            return Err(ProtocolError::new(
                "invalid-request",
                "a snapshot requires at least one source",
            ));
        }
        sources.sort_by(|left, right| left.uri.cmp(&right.uri));
        if sources.windows(2).any(|pair| pair[0].uri == pair[1].uri) {
            return Err(ProtocolError::new(
                "invalid-request",
                "snapshot source URIs must be unique",
            ));
        }
        let snapshot_id = snapshot_hash(&sources, manifest.as_ref(), lock.as_ref(), &options);
        if let Some(snapshot) = self.snapshots.get(&snapshot_id) {
            return Ok(snapshot.metadata.clone());
        }
        let mut documents = BTreeMap::new();
        let mut units = Vec::with_capacity(sources.len());
        let mut identities = Vec::with_capacity(sources.len());
        let mut syntax_invalid = false;
        let mut bytes = 0;
        for (index, input) in sources.into_iter().enumerate() {
            bytes += input.text.len();
            let file_id = u32::try_from(index)
                .map_err(|_| ProtocolError::new("snapshot-too-large", "too many source files"))?;
            let path = uri_path(&input.uri).unwrap_or_else(|| PathBuf::from(&input.uri));
            let source = SourceFile::new(file_id, path, input.text);
            let lexed = crate::lexer::lex_recovering(&source);
            let mut diagnostics = lexed.diagnostics;
            let parsed = crate::parser::parse(&source, lexed.lexed);
            diagnostics.extend(parsed.diagnostics);
            syntax_invalid |= !diagnostics.is_empty();
            let identity = SourceIdentity {
                uri: input.uri.clone(),
                content_hash: hash_text(source.text()),
            };
            identities.push(identity.clone());
            units.push(SourceUnit {
                relative_path: PathBuf::from(format!("snapshot/{file_id}.trn")),
                source: source.clone(),
                expected_namespace: None,
            });
            documents.insert(
                input.uri,
                ParsedDocument {
                    source,
                    identity,
                    tree: parsed.tree,
                    diagnostics,
                },
            );
        }
        let package = snapshot_package(snapshot_id.clone(), units, &options);
        let (semantic, semantic_invalid) = if options.semantic && !syntax_invalid {
            match crate::semantics::analyze(&package) {
                Ok(semantic) => (Some(semantic), false),
                Err(failure) => {
                    if let Some(document) = documents
                        .values_mut()
                        .find(|document| document.source.id() == failure.source.id())
                    {
                        document.diagnostics.extend(failure.diagnostics);
                    }
                    (None, true)
                }
            }
        } else {
            (None, options.semantic && syntax_invalid)
        };
        let generated = if options.generated && semantic.is_some() {
            crate::compile_package(&package)
                .ok()
                .and_then(|compilation| {
                    compilation
                        .rust_files_for(Path::new("src/main.rs"))
                        .ok()
                        .map(|files| {
                            let build_id = build_hash(&snapshot_id, &files);
                            GeneratedSnapshot { build_id, files }
                        })
                })
        } else {
            None
        };
        let metadata = SnapshotMetadata {
            compiler_version: crate::VERSION.to_owned(),
            schema_version: SCHEMA_VERSION.to_owned(),
            snapshot_id: snapshot_id.clone(),
            sources: identities,
            manifest_hash: manifest.map_or(Availability::NotYetAnalyzed, |input| {
                Availability::Known(hash_text(&input.text))
            }),
            lock_hash: lock.map_or(Availability::NotYetAnalyzed, |input| {
                Availability::Known(hash_text(&input.text))
            }),
            target: options.target,
            profile: options.profile,
            capabilities: options.capabilities,
            build_id: generated.as_ref().map_or_else(
                || {
                    if options.generated {
                        Availability::Invalid
                    } else {
                        Availability::NotYetAnalyzed
                    }
                },
                |generated| Availability::Known(generated.build_id.clone()),
            ),
        };
        self.retained_bytes += bytes;
        self.snapshot_order.push_back(snapshot_id.clone());
        self.snapshots.insert(
            snapshot_id,
            Snapshot {
                metadata: metadata.clone(),
                documents,
                semantic,
                semantic_invalid,
                generated,
                bytes,
            },
        );
        self.evict();
        Ok(metadata)
    }

    pub fn close_snapshot(&mut self, snapshot_id: &str) -> Result<(), ProtocolError> {
        let Some(snapshot) = self.snapshots.remove(snapshot_id) else {
            return Err(expired_snapshot());
        };
        self.retained_bytes = self.retained_bytes.saturating_sub(snapshot.bytes);
        self.snapshot_order
            .retain(|candidate| candidate != snapshot_id);
        self.continuations
            .retain(|_, continuation| continuation.snapshot_id != snapshot_id);
        self.proposals
            .retain(|_, proposal| proposal.snapshot_id != snapshot_id);
        Ok(())
    }

    pub fn syntax(
        &self,
        snapshot_id: &str,
        uri: &str,
        node_id: Option<u64>,
    ) -> Result<SyntaxProjection, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let mut next_id = 0;
        let projected = project_tree(&document.tree.root, &mut next_id);
        let root = node_id
            .map_or_else(
                || Some(projected.clone()),
                |wanted| find_projected_node(&projected, wanted).cloned(),
            )
            .ok_or_else(|| ProtocolError::new("unknown-node", "syntax node does not exist"))?;
        Ok(SyntaxProjection {
            source: document.identity.clone(),
            diagnostics: document
                .diagnostics
                .iter()
                .map(project_diagnostic)
                .collect(),
            root,
            tokens: document
                .tree
                .lexed
                .tokens
                .iter()
                .map(|token| TokenProjection {
                    kind: format!("{:?}", token.kind),
                    text: token.text.clone(),
                    span: token.span.into(),
                })
                .collect(),
            trivia: document
                .tree
                .lexed
                .trivia
                .iter()
                .map(|trivia| TriviaProjection {
                    kind: format!("{:?}", trivia.kind),
                    text: trivia.text.clone(),
                    span: trivia.span.into(),
                })
                .collect(),
        })
    }

    pub fn locate(
        &self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
    ) -> Result<Option<SemanticObject>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        if offset > document.source.text().len() || !document.source.text().is_char_boundary(offset)
        {
            return Err(ProtocolError::new(
                "invalid-position",
                "position must be a UTF-8 byte boundary within the source",
            ));
        }
        let Some((node, id, _)) = smallest_node_at(&document.tree.root, offset, 0) else {
            return Ok(None);
        };
        Ok(Some(semantic_object(snapshot, document, node, id, None)))
    }

    pub fn definition(
        &self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
    ) -> Result<Availability<Location>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        Ok(symbol_at(snapshot, document, offset).map_or_else(
            || semantic_unavailable(snapshot),
            |symbol| {
                declaration_location(snapshot, symbol)
                    .map_or(Availability::Unresolved, Availability::Known)
            },
        ))
    }

    pub fn references(
        &self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
    ) -> Result<Availability<Vec<Location>>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let Some(target) = symbol_at(snapshot, document, offset) else {
            return Ok(semantic_unavailable(snapshot));
        };
        let Some(semantic) = &snapshot.semantic else {
            return Ok(semantic_unavailable(snapshot));
        };
        let mut locations = Vec::new();
        for unit in semantic
            .units
            .iter()
            .filter(|unit| !unit.source_path.starts_with("<terrane>"))
        {
            walk_nodes(&unit.tree.root, None, &mut |node, _field, _id| {
                if node.kind != SyntaxKind::Name {
                    return;
                }
                let name = node_text(&unit.source, node);
                if semantic
                    .resolve_name_at(unit, node.span.start, name)
                    .is_some_and(|symbol| symbol.identity == target.identity)
                    && let Some(location) = location_for_span(snapshot, node.span)
                {
                    locations.push(location);
                }
            });
        }
        if let Some(declaration) = declaration_location(snapshot, target) {
            locations.push(declaration);
        }
        locations.sort();
        locations.dedup();
        Ok(Availability::Known(locations))
    }

    pub fn find(
        &mut self,
        snapshot_id: &str,
        selector: &Selector,
        page_size: Option<usize>,
        continuation_token: Option<&str>,
    ) -> Result<QueryPage, ProtocolError> {
        let selector_hash = hash_json(selector)?;
        let start = if let Some(token) = continuation_token {
            let continuation = self.continuations.remove(token).ok_or_else(|| {
                ProtocolError::stale("expired-continuation", "continuation has expired")
            })?;
            if continuation.snapshot_id != snapshot_id
                || continuation.selector_hash != selector_hash
            {
                return Err(ProtocolError::stale(
                    "stale-continuation",
                    "continuation belongs to a different snapshot or query",
                ));
            }
            continuation.offset
        } else {
            0
        };
        let snapshot = self.snapshot(snapshot_id)?;
        let mut matches = Vec::new();
        for (uri, document) in &snapshot.documents {
            walk_nodes(&document.tree.root, None, &mut |node, field, id| {
                if selector_matches(snapshot, document, node, field, selector) {
                    matches.push(semantic_object(snapshot, document, node, id, Some(uri)));
                }
            });
        }
        matches.sort_by(|left, right| {
            left.source_uri
                .cmp(&right.source_uri)
                .then(left.span.start.cmp(&right.span.start))
                .then(left.span.end.cmp(&right.span.end))
        });
        let page_size = page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(1, MAX_PAGE_SIZE);
        let end = start.saturating_add(page_size).min(matches.len());
        let page = matches.get(start..end).unwrap_or(&[]).to_vec();
        let complete = end == matches.len();
        let continuation = if complete {
            None
        } else {
            self.nonce = self.nonce.wrapping_add(1);
            let token = hash_text(&format!(
                "{snapshot_id}:{selector_hash}:{end}:{}",
                self.nonce
            ));
            self.continuations.insert(
                token.clone(),
                Continuation {
                    snapshot_id: snapshot_id.to_owned(),
                    selector_hash,
                    offset: end,
                },
            );
            Some(token)
        };
        Ok(QueryPage {
            matches: page,
            complete,
            continuation,
        })
    }

    pub fn generated_rust(
        &self,
        snapshot_id: &str,
        uri: &str,
        node_id: u64,
        build_id: &str,
    ) -> Result<Availability<Vec<GeneratedLocation>>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let Some(generated) = &snapshot.generated else {
            return Ok(Availability::NotYetAnalyzed);
        };
        if generated.build_id != build_id {
            return Err(ProtocolError::new(
                "mismatched-build",
                "generated Rust belongs to a different build",
            ));
        }
        let mut current = 0;
        let Some(node) = node_by_id(&document.tree.root, node_id, &mut current) else {
            return Err(ProtocolError::new(
                "unknown-node",
                "syntax node does not exist",
            ));
        };
        let mut locations = Vec::new();
        for file in &generated.files {
            for association in &file.associations {
                if association.source.file == document.source.id()
                    && spans_overlap(association.source, node.span)
                {
                    locations.push(GeneratedLocation {
                        build_id: generated.build_id.clone(),
                        path: file.path.clone(),
                        span: PublicSpan {
                            start: association.generated_start,
                            end: association.generated_end,
                        },
                    });
                }
            }
        }
        Ok(Availability::Known(locations))
    }

    pub fn propose_edits(
        &mut self,
        snapshot_id: &str,
        mut replacements: Vec<Replacement>,
    ) -> Result<EditProposal, ProtocolError> {
        let (diagnostics, semantic_reanalysis, affected_files) = {
            let snapshot = self.snapshot(snapshot_id)?;
            validate_replacements(snapshot, &mut replacements)?;
            let (_, diagnostics, semantic_reanalysis) = candidate_sources(snapshot, &replacements)?;
            let affected_files = replacements
                .iter()
                .map(|replacement| replacement.uri.as_str())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|uri| AffectedFile {
                    uri: uri.to_owned(),
                    content_hash: snapshot
                        .documents
                        .get(uri)
                        .expect("validated replacement URI")
                        .identity
                        .content_hash
                        .clone(),
                })
                .collect();
            (diagnostics, semantic_reanalysis, affected_files)
        };
        self.nonce = self.nonce.wrapping_add(1);
        let proposal_id = hash_text(&format!(
            "{snapshot_id}:{}:{}",
            hash_json(&replacements)?,
            self.nonce
        ));
        let proposal = EditProposal {
            proposal_id: proposal_id.clone(),
            snapshot_id: snapshot_id.to_owned(),
            affected_files,
            replacements,
            preview_diagnostics: diagnostics,
            semantic_reanalysis,
        };
        self.proposals.insert(proposal_id, proposal.clone());
        Ok(proposal)
    }

    pub fn propose_rename(
        &mut self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
        new_name: &str,
    ) -> Result<EditProposal, ProtocolError> {
        if !valid_identifier(new_name) {
            return Err(ProtocolError::new(
                "invalid-name",
                "rename target must be a Terrane identifier",
            ));
        }
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let Some(target) = symbol_at(snapshot, document, offset) else {
            return Err(ProtocolError::new(
                "unresolved-symbol",
                "position does not resolve to a semantic symbol",
            ));
        };
        let Some(semantic) = &snapshot.semantic else {
            return Err(ProtocolError::new(
                "semantic-unavailable",
                "rename requires a complete semantic snapshot",
            ));
        };
        let mut replacements = Vec::new();
        for unit in semantic
            .units
            .iter()
            .filter(|unit| !unit.source_path.starts_with("<terrane>"))
        {
            let Some(source_uri) = snapshot.uri_for_file(unit.source.id()) else {
                continue;
            };
            walk_nodes(&unit.tree.root, None, &mut |node, _field, _id| {
                if node.kind != SyntaxKind::Name {
                    return;
                }
                let name = node_text(&unit.source, node);
                if semantic
                    .resolve_name_at(unit, node.span.start, name)
                    .is_some_and(|symbol| symbol.identity == target.identity)
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
            && !replacements.iter().any(|replacement| {
                replacement.uri == location.uri && replacement.span == location.span
            })
        {
            replacements.push(Replacement {
                uri: location.uri,
                span: location.span,
                text: new_name.to_owned(),
            });
        }
        for replacement in &replacements {
            let unit = semantic
                .units
                .iter()
                .find(|unit| {
                    snapshot.uri_for_file(unit.source.id()) == Some(replacement.uri.as_str())
                })
                .expect("semantic source must belong to snapshot");
            if semantic
                .resolve_name_at(unit, replacement.span.start, new_name)
                .is_some_and(|candidate| candidate.identity != target.identity)
            {
                return Err(ProtocolError::new(
                    "rename-capture",
                    format!("`{new_name}` would capture an existing symbol"),
                ));
            }
        }
        self.propose_edits(snapshot_id, replacements)
    }

    pub fn apply_edits(&mut self, proposal_id: &str) -> Result<ApplyReport, ProtocolError> {
        let proposal = self.proposals.get(proposal_id).cloned().ok_or_else(|| {
            ProtocolError::new("unknown-proposal", "edit proposal does not exist")
        })?;
        let snapshot = self.snapshot(&proposal.snapshot_id)?;
        let mut new_contents = BTreeMap::new();
        let mut paths = BTreeMap::new();
        for affected in &proposal.affected_files {
            let path = uri_path(&affected.uri).ok_or_else(|| {
                ProtocolError::new("read-only-source", "disk apply requires a file URI")
            })?;
            let current = fs::read_to_string(&path).map_err(|error| {
                ProtocolError::new(
                    "apply-io",
                    format!("cannot read {}: {error}", path.display()),
                )
            })?;
            if hash_text(&current) != affected.content_hash {
                return Err(ProtocolError::new(
                    "stale-write",
                    format!("{} changed after the snapshot was opened", path.display()),
                ));
            }
            let replacements = proposal
                .replacements
                .iter()
                .filter(|replacement| replacement.uri == affected.uri)
                .cloned()
                .collect::<Vec<_>>();
            let updated = apply_replacements(&current, &replacements)?;
            new_contents.insert(affected.uri.clone(), updated);
            paths.insert(affected.uri.clone(), path);
        }
        let _ = snapshot;
        let mut temporary = BTreeMap::new();
        for (uri, contents) in &new_contents {
            let path = &paths[uri];
            let parent = path.parent().unwrap_or_else(|| Path::new("."));
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| ProtocolError::new("apply-io", "file name must be valid UTF-8"))?;
            let temp_path = parent.join(format!(".{file_name}.terrane-{proposal_id}.tmp"));
            let mut file = fs::File::create(&temp_path).map_err(|error| {
                ProtocolError::new(
                    "apply-io",
                    format!("cannot create {}: {error}", temp_path.display()),
                )
            })?;
            file.write_all(contents.as_bytes()).map_err(|error| {
                ProtocolError::new(
                    "apply-io",
                    format!("cannot write {}: {error}", temp_path.display()),
                )
            })?;
            file.sync_all().map_err(|error| {
                ProtocolError::new(
                    "apply-io",
                    format!("cannot sync {}: {error}", temp_path.display()),
                )
            })?;
            temporary.insert(uri.clone(), temp_path);
        }
        let mut report = ApplyReport {
            committed: Vec::new(),
            uncommitted: Vec::new(),
            recovery_files: Vec::new(),
        };
        for affected in &proposal.affected_files {
            let uri = &affected.uri;
            let path = &paths[uri];
            let temp_path = &temporary[uri];
            if let Err(error) = fs::rename(temp_path, path) {
                report.uncommitted.extend(
                    proposal
                        .affected_files
                        .iter()
                        .map(|file| file.uri.clone())
                        .filter(|candidate| !report.committed.contains(candidate)),
                );
                report.recovery_files.extend(
                    temporary
                        .iter()
                        .filter(|(candidate, _)| !report.committed.contains(candidate))
                        .map(|(_, path)| path.display().to_string()),
                );
                return Err(ProtocolError::new(
                    "partial-apply",
                    format!(
                        "failed after committing {:?}; recovery files {:?}: {error}",
                        report.committed, report.recovery_files
                    ),
                ));
            }
            report.committed.push(uri.clone());
        }
        self.proposals.remove(proposal_id);
        Ok(report)
    }

    pub fn format(&self, snapshot_id: &str, uri: &str) -> Result<FormatResult, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let formatted = format_source(document);
        let changed = formatted != document.source.text();
        let edits = changed
            .then(|| Replacement {
                uri: uri.to_owned(),
                span: PublicSpan {
                    start: 0,
                    end: document.source.text().len(),
                },
                text: formatted.clone(),
            })
            .into_iter()
            .collect();
        Ok(FormatResult {
            uri: uri.to_owned(),
            changed,
            text: formatted,
            edits,
        })
    }

    fn snapshot(&self, snapshot_id: &str) -> Result<&Snapshot, ProtocolError> {
        self.snapshots.get(snapshot_id).ok_or_else(expired_snapshot)
    }

    fn evict(&mut self) {
        while self.snapshots.len() > self.max_snapshots || self.retained_bytes > self.max_bytes {
            let Some(id) = self.snapshot_order.pop_front() else {
                break;
            };
            if let Some(snapshot) = self.snapshots.remove(&id) {
                self.retained_bytes = self.retained_bytes.saturating_sub(snapshot.bytes);
            }
            self.continuations
                .retain(|_, continuation| continuation.snapshot_id != id);
            self.proposals
                .retain(|_, proposal| proposal.snapshot_id != id);
        }
    }

    fn success_response(
        &self,
        request_id: String,
        context: (Option<String>, Option<String>),
        result: serde_json::Value,
    ) -> ResponseEnvelope {
        let (snapshot_id, source_uri) = context;
        let source_hash = snapshot_id
            .as_deref()
            .and_then(|id| self.snapshots.get(id))
            .and_then(|snapshot| {
                source_uri
                    .as_deref()
                    .and_then(|uri| snapshot.documents.get(uri))
            })
            .map(|document| document.identity.content_hash.clone());
        ResponseEnvelope {
            compiler_version: crate::VERSION.to_owned(),
            schema_version: SCHEMA_VERSION.to_owned(),
            request_id,
            snapshot_id,
            source_uri,
            source_hash,
            result: Some(result),
            error: None,
        }
    }

    fn error_response(
        &self,
        request_id: String,
        snapshot_id: Option<String>,
        source_uri: Option<String>,
        error: ProtocolError,
    ) -> ResponseEnvelope {
        let source_hash = snapshot_id
            .as_deref()
            .and_then(|id| self.snapshots.get(id))
            .and_then(|snapshot| {
                source_uri
                    .as_deref()
                    .and_then(|uri| snapshot.documents.get(uri))
            })
            .map(|document| document.identity.content_hash.clone());
        ResponseEnvelope {
            compiler_version: crate::VERSION.to_owned(),
            schema_version: SCHEMA_VERSION.to_owned(),
            request_id,
            snapshot_id,
            source_uri,
            source_hash,
            result: None,
            error: Some(error),
        }
    }
}

impl Snapshot {
    fn document(&self, uri: &str) -> Result<&ParsedDocument, ProtocolError> {
        self.documents.get(uri).ok_or_else(|| {
            ProtocolError::new("unknown-source", "source URI is not in the snapshot")
        })
    }

    fn uri_for_file(&self, file: u32) -> Option<&str> {
        self.documents
            .iter()
            .find(|(_, document)| document.source.id() == file)
            .map(|(uri, _)| uri.as_str())
    }
}

fn snapshot_package(
    identity: String,
    units: Vec<SourceUnit>,
    options: &SnapshotOptions,
) -> Package {
    let capabilities =
        (!options.capabilities.is_empty()).then(|| options.capabilities.iter().cloned().collect());
    Package {
        identity,
        root: PathBuf::from("."),
        prelude: true,
        reflection: ReflectionProfile::Ordinary,
        executor: ExecutorProfile::Threaded,
        profile: CapabilityProfile {
            name: options.profile.clone(),
            capabilities,
            panic: PanicProfile::Unwind,
        },
        build_toolchain: BuildToolchain::Pinned,
        units,
        rust_dependencies: Vec::new(),
    }
}

fn project_tree(node: &SyntaxNode, next_id: &mut u64) -> SyntaxNodeProjection {
    let id = *next_id;
    *next_id = next_id.saturating_add(1);
    let children = node
        .children
        .iter()
        .enumerate()
        .map(|(index, child)| SyntaxChild {
            field: child_field(node.kind, index).to_owned(),
            node: project_tree(child, next_id),
        })
        .collect();
    SyntaxNodeProjection {
        id,
        kind: format!("{:?}", node.kind),
        span: node.span.into(),
        state: syntax_state(node),
        children,
    }
}

fn find_projected_node(node: &SyntaxNodeProjection, wanted: u64) -> Option<&SyntaxNodeProjection> {
    if node.id == wanted {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_projected_node(&child.node, wanted))
}

fn child_field(parent: SyntaxKind, index: usize) -> &'static str {
    match (parent, index) {
        (SyntaxKind::Binding, 0) => "name",
        (SyntaxKind::Binding, 1) => "type",
        (SyntaxKind::Binding, 2) => "value",
        (SyntaxKind::FunctionDeclaration, 0) => "name",
        (SyntaxKind::FunctionDeclaration, 1) => "parameters",
        (SyntaxKind::FunctionDeclaration, _) => "body",
        (
            SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration,
            0,
        ) => "name",
        (SyntaxKind::Assignment, 0) => "target",
        (SyntaxKind::Assignment, 1) => "value",
        (SyntaxKind::CallExpression, 0) => "callee",
        (SyntaxKind::CallExpression, 1) => "arguments",
        (SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression, 0) => "receiver",
        (SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression, 1) => "member",
        (SyntaxKind::IfStatement | SyntaxKind::WhileStatement, 0) => "condition",
        (SyntaxKind::IfStatement | SyntaxKind::WhileStatement, _) => "body",
        (SyntaxKind::ReturnStatement | SyntaxKind::ThrowStatement, 0) => "value",
        (SyntaxKind::CompilationUnit | SyntaxKind::Block, _) => "item",
        _ => "child",
    }
}

fn syntax_state(node: &SyntaxNode) -> SyntaxState {
    match node.kind {
        SyntaxKind::Error => SyntaxState::Error,
        SyntaxKind::Unsupported => SyntaxState::Unsupported,
        _ if node.children.iter().any(|child| {
            matches!(
                syntax_state(child),
                SyntaxState::Error | SyntaxState::Recovery | SyntaxState::Unsupported
            )
        }) =>
        {
            SyntaxState::Recovery
        }
        _ => SyntaxState::Complete,
    }
}

fn walk_nodes(
    node: &SyntaxNode,
    field: Option<&str>,
    visit: &mut impl FnMut(&SyntaxNode, Option<&str>, u64),
) {
    fn recurse(
        node: &SyntaxNode,
        field: Option<&str>,
        next: &mut u64,
        visit: &mut impl FnMut(&SyntaxNode, Option<&str>, u64),
    ) {
        let id = *next;
        *next = next.saturating_add(1);
        visit(node, field, id);
        for (index, child) in node.children.iter().enumerate() {
            recurse(child, Some(child_field(node.kind, index)), next, visit);
        }
    }
    let mut next = 0;
    recurse(node, field, &mut next, visit);
}

fn smallest_node_at(node: &SyntaxNode, offset: usize, id: u64) -> Option<(&SyntaxNode, u64, u64)> {
    if offset < node.span.start || offset > node.span.end {
        return None;
    }
    let mut next = id.saturating_add(1);
    for child in &node.children {
        if let Some(found) = smallest_node_at(child, offset, next) {
            return Some(found);
        }
        next = next.saturating_add(node_count(child));
    }
    Some((node, id, next))
}

fn node_count(node: &SyntaxNode) -> u64 {
    1 + node.children.iter().map(node_count).sum::<u64>()
}

fn node_by_id<'a>(node: &'a SyntaxNode, wanted: u64, next: &mut u64) -> Option<&'a SyntaxNode> {
    let id = *next;
    *next = next.saturating_add(1);
    if id == wanted {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| node_by_id(child, wanted, next))
}

fn semantic_object(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    node: &SyntaxNode,
    id: u64,
    uri: Option<&str>,
) -> SemanticObject {
    let name =
        (node.kind == SyntaxKind::Name).then(|| node_text(&document.source, node).to_owned());
    let symbol = snapshot.semantic.as_ref().and_then(|semantic| {
        let unit = semantic
            .units
            .iter()
            .find(|unit| unit.source.id() == document.source.id())?;
        name.as_deref()
            .and_then(|name| semantic.resolve_name_at(unit, node.span.start, name))
    });
    let value_type = snapshot.semantic.as_ref().and_then(|semantic| {
        let unit = semantic
            .units
            .iter()
            .find(|unit| unit.source.id() == document.source.id())?;
        unit.inferred_value_type(node)
            .map(|value| value.to_string())
    });
    SemanticObject {
        source_uri: uri.unwrap_or(&document.identity.uri).to_owned(),
        span: node.span.into(),
        syntax_node_id: id,
        kind: format!("{:?}", node.kind),
        name,
        symbol_identity: symbol.map_or_else(
            || semantic_unavailable(snapshot),
            |symbol| Availability::Known(symbol.identity.clone()),
        ),
        descriptor_identity: symbol
            .and_then(|symbol| symbol.descriptor_identity())
            .map_or_else(
                || semantic_unavailable(snapshot),
                |identity| Availability::Known(identity.to_owned()),
            ),
        value_type: value_type.as_ref().map_or_else(
            || semantic_unavailable(snapshot),
            |value| Availability::Known(value.clone()),
        ),
        ownership: snapshot.semantic.as_ref().map_or_else(
            || semantic_unavailable(snapshot),
            |_| {
                Availability::Known(if value_type_is_reference(value_type.as_deref()) {
                    "borrowed".to_owned()
                } else {
                    "owned".to_owned()
                })
            },
        ),
        effects: Availability::Unsupported,
        capabilities: Availability::Known(snapshot.metadata.capabilities.clone()),
        declaration: symbol
            .and_then(|symbol| declaration_location(snapshot, symbol))
            .map_or_else(|| semantic_unavailable(snapshot), Availability::Known),
    }
}

fn value_type_is_reference(value_type: Option<&str>) -> bool {
    value_type.is_some_and(|value| value.starts_with('&'))
}

fn symbol_at<'a>(
    snapshot: &'a Snapshot,
    document: &ParsedDocument,
    offset: usize,
) -> Option<&'a crate::Symbol> {
    let semantic = snapshot.semantic.as_ref()?;
    let unit = semantic
        .units
        .iter()
        .find(|unit| unit.source.id() == document.source.id())?;
    let (node, _, _) = smallest_node_at(&document.tree.root, offset, 0)?;
    let name_node = if node.kind == SyntaxKind::Name {
        node
    } else {
        nearest_name(node, offset)?
    };
    semantic.resolve_name_at(
        unit,
        name_node.span.start,
        node_text(&document.source, name_node),
    )
}

fn nearest_name(node: &SyntaxNode, offset: usize) -> Option<&SyntaxNode> {
    node.children
        .iter()
        .filter(|child| child.span.start <= offset && offset <= child.span.end)
        .find_map(|child| {
            (child.kind == SyntaxKind::Name)
                .then_some(child)
                .or_else(|| nearest_name(child, offset))
        })
}

fn semantic_unavailable<T>(snapshot: &Snapshot) -> Availability<T> {
    if snapshot.semantic_invalid {
        Availability::Invalid
    } else {
        Availability::NotYetAnalyzed
    }
}

fn declaration_location(snapshot: &Snapshot, symbol: &crate::Symbol) -> Option<Location> {
    let declaration = symbol.declaration_span?;
    let document = snapshot
        .documents
        .values()
        .find(|document| document.source.id() == declaration.file)?;
    let mut name_span = None;
    walk_nodes(&document.tree.root, None, &mut |node, _field, _id| {
        if name_span.is_none()
            && node.kind == SyntaxKind::Name
            && declaration.start <= node.span.start
            && node.span.end <= declaration.end
            && node_text(&document.source, node) == symbol.name
        {
            name_span = Some(node.span);
        }
    });
    location_for_span(snapshot, name_span.unwrap_or(declaration))
}

fn location_for_span(snapshot: &Snapshot, span: Span) -> Option<Location> {
    Some(Location {
        uri: snapshot.uri_for_file(span.file)?.to_owned(),
        span: span.into(),
    })
}

fn selector_matches(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    node: &SyntaxNode,
    field: Option<&str>,
    selector: &Selector,
) -> bool {
    if !selector.include_recovery && syntax_state(node) != SyntaxState::Complete {
        return false;
    }
    if selector
        .kind
        .as_deref()
        .is_some_and(|kind| kind != format!("{:?}", node.kind))
    {
        return false;
    }
    if selector
        .child_field
        .as_deref()
        .is_some_and(|wanted| field != Some(wanted))
    {
        return false;
    }
    if selector
        .containing
        .as_ref()
        .is_some_and(|span| !(node.span.start <= span.start && span.end <= node.span.end))
    {
        return false;
    }
    let text = node_text(&document.source, node);
    if selector
        .text
        .as_deref()
        .is_some_and(|wanted| wanted != text)
    {
        return false;
    }
    if selector.token_kind.as_deref().is_some_and(|wanted| {
        !document.tree.lexed.tokens[node.token_range.clone()]
            .iter()
            .any(|token| format!("{:?}", token.kind) == wanted)
    }) {
        return false;
    }
    if selector.symbol_identity.is_some() || selector.descriptor_identity.is_some() {
        if node.kind != SyntaxKind::Name {
            return false;
        }
        let Some(semantic) = &snapshot.semantic else {
            return false;
        };
        let Some(unit) = semantic
            .units
            .iter()
            .find(|unit| unit.source.id() == document.source.id())
        else {
            return false;
        };
        let Some(symbol) = semantic.resolve_name_at(unit, node.span.start, text) else {
            return false;
        };
        if selector
            .symbol_identity
            .as_deref()
            .is_some_and(|identity| identity != symbol.identity)
        {
            return false;
        }
        if selector
            .descriptor_identity
            .as_deref()
            .is_some_and(|identity| symbol.descriptor_identity() != Some(identity))
        {
            return false;
        }
    }
    true
}

fn validate_replacements(
    snapshot: &Snapshot,
    replacements: &mut [Replacement],
) -> Result<(), ProtocolError> {
    replacements.sort_by(|left, right| {
        left.uri
            .cmp(&right.uri)
            .then(left.span.start.cmp(&right.span.start))
            .then(left.span.end.cmp(&right.span.end))
    });
    let mut previous: Option<&Replacement> = None;
    for replacement in replacements.iter() {
        let document = snapshot.document(&replacement.uri)?;
        let text = document.source.text();
        if replacement.span.start > replacement.span.end
            || replacement.span.end > text.len()
            || !text.is_char_boundary(replacement.span.start)
            || !text.is_char_boundary(replacement.span.end)
        {
            return Err(ProtocolError::new(
                "invalid-edit",
                "replacement span must use valid UTF-8 byte boundaries",
            ));
        }
        if previous.is_some_and(|prior| {
            prior.uri == replacement.uri && replacement.span.start < prior.span.end
        }) {
            return Err(ProtocolError::new(
                "overlapping-edits",
                "replacement spans must not overlap",
            ));
        }
        previous = Some(replacement);
    }
    Ok(())
}

fn candidate_sources(
    snapshot: &Snapshot,
    replacements: &[Replacement],
) -> Result<(BTreeMap<String, String>, Vec<DiagnosticProjection>, bool), ProtocolError> {
    let mut candidate = BTreeMap::new();
    let mut diagnostics = Vec::new();
    let mut units = Vec::new();
    for (index, (uri, document)) in snapshot.documents.iter().enumerate() {
        let edits = replacements
            .iter()
            .filter(|replacement| replacement.uri == *uri)
            .cloned()
            .collect::<Vec<_>>();
        let text = apply_replacements(document.source.text(), &edits)?;
        let file_id = u32::try_from(index)
            .map_err(|_| ProtocolError::new("snapshot-too-large", "too many source files"))?;
        let source = SourceFile::new(file_id, document.source.path().to_path_buf(), text.clone());
        let lexed = crate::lexer::lex_recovering(&source);
        diagnostics.extend(lexed.diagnostics.iter().map(project_diagnostic));
        let parsed = crate::parser::parse(&source, lexed.lexed);
        diagnostics.extend(parsed.diagnostics.iter().map(project_diagnostic));
        units.push(SourceUnit {
            relative_path: PathBuf::from(format!("snapshot/{file_id}.trn")),
            source,
            expected_namespace: None,
        });
        candidate.insert(uri.clone(), text);
    }
    let semantic_reanalysis = if diagnostics.is_empty() && snapshot.semantic.is_some() {
        let package = snapshot_package(
            snapshot.metadata.snapshot_id.clone(),
            units,
            &SnapshotOptions {
                semantic: true,
                generated: false,
                target: snapshot.metadata.target.clone(),
                profile: snapshot.metadata.profile.clone(),
                capabilities: snapshot.metadata.capabilities.clone(),
            },
        );
        match crate::semantics::analyze(&package) {
            Ok(_) => true,
            Err(failure) => {
                diagnostics.extend(failure.diagnostics.iter().map(project_diagnostic));
                false
            }
        }
    } else {
        false
    };
    Ok((candidate, diagnostics, semantic_reanalysis))
}

fn apply_replacements(source: &str, replacements: &[Replacement]) -> Result<String, ProtocolError> {
    let mut output = source.to_owned();
    for replacement in replacements.iter().rev() {
        if replacement.span.end > output.len()
            || !output.is_char_boundary(replacement.span.start)
            || !output.is_char_boundary(replacement.span.end)
        {
            return Err(ProtocolError::new(
                "invalid-edit",
                "replacement span is invalid",
            ));
        }
        output.replace_range(
            replacement.span.start..replacement.span.end,
            &replacement.text,
        );
    }
    Ok(output)
}

fn format_source(document: &ParsedDocument) -> String {
    if !document.diagnostics.is_empty() {
        return document.source.text().to_owned();
    }
    let protected = document
        .tree
        .lexed
        .tokens
        .iter()
        .filter(|token| token.kind == TokenKind::BlockString)
        .map(|token| token.span)
        .chain(
            document
                .tree
                .lexed
                .trivia
                .iter()
                .filter(|trivia| {
                    trivia.kind == TriviaKind::BlockComment && trivia.text.contains('\n')
                })
                .map(|trivia| trivia.span),
        )
        .collect::<Vec<_>>();
    let source = document.source.text();
    let newline = if source.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let had_final_newline = source.ends_with('\n');
    let mut output = String::with_capacity(source.len());
    let mut offset = 0;
    for raw in source.split_inclusive('\n') {
        let content = raw.trim_end_matches(['\r', '\n']);
        let end = offset + content.len();
        let is_protected = protected
            .iter()
            .any(|span| span.start < end && offset < span.end);
        if is_protected {
            output.push_str(content);
        } else {
            output.push_str(content.trim_end_matches([' ', '\t']));
        }
        if raw.ends_with('\n') {
            output.push_str(newline);
        }
        offset += raw.len();
    }
    if !had_final_newline {
        output.truncate(output.trim_end_matches(['\r', '\n']).len());
    }
    output
}

fn project_diagnostic(diagnostic: &Diagnostic) -> DiagnosticProjection {
    DiagnosticProjection {
        severity: format!("{:?}", diagnostic.severity).to_lowercase(),
        code: diagnostic.code.to_owned(),
        message: diagnostic.message.clone(),
        span: diagnostic.primary.map(Into::into),
        help: diagnostic.help.clone(),
    }
}

fn node_text<'a>(source: &'a SourceFile, node: &SyntaxNode) -> &'a str {
    source
        .text()
        .get(node.span.start..node.span.end)
        .unwrap_or_default()
}

fn valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    chars
        .next()
        .is_some_and(|first| first == '_' || first.is_alphabetic())
        && chars
            .all(|character| character == '_' || character == '-' || character.is_alphanumeric())
}

fn spans_overlap(left: Span, right: Span) -> bool {
    left.file == right.file && left.start < right.end && right.start < left.end
}

fn request_context(request: &Request) -> (Option<String>, Option<String>) {
    match request {
        Request::CloseSnapshot { snapshot_id }
        | Request::Find { snapshot_id, .. }
        | Request::ProposeEdits { snapshot_id, .. } => (Some(snapshot_id.clone()), None),
        Request::Syntax {
            snapshot_id, uri, ..
        }
        | Request::Locate {
            snapshot_id, uri, ..
        }
        | Request::Definition {
            snapshot_id, uri, ..
        }
        | Request::References {
            snapshot_id, uri, ..
        }
        | Request::GeneratedRust {
            snapshot_id, uri, ..
        }
        | Request::ProposeRename {
            snapshot_id, uri, ..
        }
        | Request::Format {
            snapshot_id, uri, ..
        } => (Some(snapshot_id.clone()), Some(uri.clone())),
        Request::OpenSnapshot { .. } | Request::ApplyEdits { .. } | Request::Cancel { .. } => {
            (None, None)
        }
    }
}

fn snapshot_hash(
    sources: &[SourceInput],
    manifest: Option<&SourceInput>,
    lock: Option<&SourceInput>,
    options: &SnapshotOptions,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(crate::VERSION.as_bytes());
    hasher.update(SCHEMA_VERSION.as_bytes());
    for source in sources {
        hasher.update(source.uri.as_bytes());
        hasher.update([0]);
        hasher.update(source.text.as_bytes());
        hasher.update([0xff]);
    }
    for input in [manifest, lock].into_iter().flatten() {
        hasher.update(input.uri.as_bytes());
        hasher.update([0]);
        hasher.update(input.text.as_bytes());
        hasher.update([0xfe]);
    }
    if let Ok(encoded) = serde_json::to_vec(options) {
        hasher.update(encoded);
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn build_hash(snapshot_id: &str, files: &[crate::rust_ir::RenderedFile]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(snapshot_id.as_bytes());
    for file in files {
        hasher.update(file.path.as_bytes());
        hasher.update([0]);
        hasher.update(file.contents.as_bytes());
    }
    format!("sha256:{:x}", hasher.finalize())
}

fn hash_text(text: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(text.as_bytes()))
}

fn hash_json(value: &impl Serialize) -> Result<String, ProtocolError> {
    serde_json::to_vec(value)
        .map(|bytes| format!("sha256:{:x}", Sha256::digest(bytes)))
        .map_err(serialization_error)
}

fn serialization_error(error: serde_json::Error) -> ProtocolError {
    ProtocolError::new("internal-serialization", error.to_string())
}

fn uri_path(uri: &str) -> Option<PathBuf> {
    uri.strip_prefix("file://").map(PathBuf::from)
}

fn expired_snapshot() -> ProtocolError {
    ProtocolError::stale(
        "expired-snapshot",
        "snapshot is closed, evicted, or unknown; open a fresh snapshot",
    )
}

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
            .expect("snapshot should open")
    }

    #[test]
    fn lossless_syntax_preserves_unicode_crlf_trivia_and_recovery() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/incomplete.trn";
        let text =
            "namespace café\r\n\r\nfunction main;\r\n    value string = 'é'  # note\r\n    (\r\n";
        let metadata = open(&mut engine, uri, text, SnapshotOptions::default());
        let syntax = engine
            .syntax(&metadata.snapshot_id, uri, None)
            .expect("syntax should remain available");

        assert_eq!(syntax.source.content_hash, hash_text(text));
        assert!(syntax.tokens.iter().any(|token| token.text == "'é'"));
        assert!(syntax.trivia.iter().any(|trivia| trivia.text == "# note"));
        assert!(!syntax.diagnostics.is_empty());
        assert_ne!(syntax.root.state, SyntaxState::Complete);
        assert_eq!(syntax.root.span.end, text.len());
    }

    #[test]
    fn semantic_identity_drives_definition_references_and_paged_matching() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/query.trn";
        let text = "namespace query\n\nfunction answer int;\n    return 42\n\nfunction main;\n    value int = answer;\n";
        let metadata = open(
            &mut engine,
            uri,
            text,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        let call_offset = text.rfind("answer").expect("call name");
        let definition = engine
            .definition(&metadata.snapshot_id, uri, call_offset)
            .expect("definition query");
        let Availability::Known(definition) = definition else {
            panic!("definition should be known");
        };
        assert_eq!(&text[definition.span.start..definition.span.end], "answer");
        let Availability::Known(references) = engine
            .references(&metadata.snapshot_id, uri, call_offset)
            .expect("reference query")
        else {
            panic!("references should be known");
        };
        assert!(references.len() >= 2);

        let selector = Selector {
            kind: Some("Name".to_owned()),
            child_field: None,
            containing: None,
            text: None,
            token_kind: Some("Identifier".to_owned()),
            symbol_identity: None,
            descriptor_identity: None,
            include_recovery: false,
        };
        let first = engine
            .find(&metadata.snapshot_id, &selector, Some(1), None)
            .expect("first page");
        assert_eq!(first.matches.len(), 1);
        assert!(!first.complete);
        let second = engine
            .find(
                &metadata.snapshot_id,
                &selector,
                Some(100),
                first.continuation.as_deref(),
            )
            .expect("second page");
        assert!(second.complete);
        assert!(!second.matches.is_empty());
    }

    #[test]
    fn continuations_expire_with_their_bounded_snapshot() {
        let mut engine = ToolingEngine::new(1, usize::MAX);
        let first = open(
            &mut engine,
            "file:///workspace/first.trn",
            "function main;\n",
            SnapshotOptions::default(),
        );
        let selector = Selector {
            kind: None,
            child_field: None,
            containing: None,
            text: None,
            token_kind: None,
            symbol_identity: None,
            descriptor_identity: None,
            include_recovery: true,
        };
        let page = engine
            .find(&first.snapshot_id, &selector, Some(1), None)
            .expect("first query");
        let continuation = page.continuation.expect("query should be paged");
        let _second = open(
            &mut engine,
            "file:///workspace/second.trn",
            "function main;\n    value int = 2\n",
            SnapshotOptions::default(),
        );

        let error = engine
            .find(&first.snapshot_id, &selector, Some(1), Some(&continuation))
            .expect_err("evicted continuation must fail");
        assert!(error.retry_fresh_query);
    }

    #[test]
    fn formatter_is_idempotent_and_leaves_recovered_source_untouched() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/format.trn";
        let source = "function main;   \r\n    value int = 1   \r\n";
        let snapshot = open(&mut engine, uri, source, SnapshotOptions::default());
        let first = engine
            .format(&snapshot.snapshot_id, uri)
            .expect("format valid source");
        assert!(first.changed);
        assert_eq!(first.text, "function main;\r\n    value int = 1\r\n");
        let second_snapshot = open(&mut engine, uri, &first.text, SnapshotOptions::default());
        let second = engine
            .format(&second_snapshot.snapshot_id, uri)
            .expect("format second pass");
        assert!(!second.changed);

        let invalid = "function main;\n    (\n";
        let invalid_snapshot = open(
            &mut engine,
            "file:///workspace/recovered.trn",
            invalid,
            SnapshotOptions::default(),
        );
        assert_eq!(
            engine
                .format(
                    &invalid_snapshot.snapshot_id,
                    "file:///workspace/recovered.trn"
                )
                .expect("format recovered source")
                .text,
            invalid
        );
    }

    #[test]
    fn edit_apply_preflights_every_hash_and_refuses_stale_writes() {
        let directory = std::env::temp_dir().join(format!(
            "terrane-tooling-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock after epoch")
                .as_nanos()
        ));
        fs::create_dir_all(&directory).expect("create temporary directory");
        let path = directory.join("case.trn");
        let original = "function main;\n    value int = 1\n";
        fs::write(&path, original).expect("write fixture");
        let uri = format!("file://{}", path.display());
        let mut engine = ToolingEngine::default();
        let snapshot = open(&mut engine, &uri, original, SnapshotOptions::default());
        let start = original.find('1').expect("literal");
        let proposal = engine
            .propose_edits(
                &snapshot.snapshot_id,
                vec![Replacement {
                    uri: uri.clone(),
                    span: PublicSpan {
                        start,
                        end: start + 1,
                    },
                    text: "2".to_owned(),
                }],
            )
            .expect("propose edit");
        fs::write(&path, original.replace('1', "3")).expect("make source stale");
        let error = engine
            .apply_edits(&proposal.proposal_id)
            .expect_err("stale edit must fail");
        assert_eq!(error.code, "stale-write");
        assert_eq!(
            fs::read_to_string(&path).expect("read unchanged source"),
            original.replace('1', "3")
        );
        fs::remove_dir_all(directory).expect("remove temporary directory");
    }

    #[test]
    fn generated_navigation_is_bound_to_the_exact_build() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/generated.trn";
        let text = "namespace generated\n\nfunction main;\n    value int = 1\n";
        let metadata = open(
            &mut engine,
            uri,
            text,
            SnapshotOptions {
                semantic: true,
                generated: true,
                ..SnapshotOptions::default()
            },
        );
        let Availability::Known(build_id) = metadata.build_id.clone() else {
            panic!("build should be available");
        };
        let syntax = engine
            .syntax(&metadata.snapshot_id, uri, None)
            .expect("syntax tree");
        let main_node = syntax
            .root
            .children
            .iter()
            .flat_map(|child| &child.node.children)
            .find(|child| child.node.kind == "Name")
            .map(|child| child.node.id)
            .unwrap_or(syntax.root.id);
        let locations = engine
            .generated_rust(&metadata.snapshot_id, uri, main_node, &build_id)
            .expect("matching build");
        assert!(matches!(locations, Availability::Known(_)));
        let error = engine
            .generated_rust(&metadata.snapshot_id, uri, main_node, "sha256:other")
            .expect_err("mismatched build must fail");
        assert_eq!(error.code, "mismatched-build");
    }
}
