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
#[serde(deny_unknown_fields)]
pub struct SourceInput {
    pub uri: String,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
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
    #[serde(default = "default_generated_entrypoint")]
    pub generated_entrypoint: String,
}

fn default_target() -> String {
    "host".to_owned()
}

fn default_profile() -> String {
    "development".to_owned()
}

fn default_generated_entrypoint() -> String {
    "src/main.rs".to_owned()
}

impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            semantic: false,
            generated: false,
            target: default_target(),
            profile: default_profile(),
            capabilities: Vec::new(),
            generated_entrypoint: default_generated_entrypoint(),
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
    pub generated_entrypoint: String,
    pub build_id: Availability<String>,
    pub dependency_projection: Availability<DependencyProjectionIdentity>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct DependencyProjectionIdentity {
    pub cache_identity: String,
    pub content_hash: String,
    pub dependencies: Vec<String>,
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
    ContainsRecovery,
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
    pub state: SyntaxState,
    pub name: Option<String>,
    pub symbol_identity: Availability<String>,
    pub descriptor_identity: Availability<String>,
    pub value_type: Availability<String>,
    pub ownership: Availability<String>,
    pub effects: Availability<Vec<String>>,
    pub capabilities: Availability<Vec<String>>,
    pub declaration: Availability<Location>,
    pub invocation_mode: Availability<String>,
    pub members: Availability<Vec<MemberFact>>,
    pub inheritance: Availability<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct MemberFact {
    pub name: String,
    pub identity: String,
    pub kind: String,
    pub value_type: Availability<String>,
    pub declaration: Availability<Location>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Location {
    pub uri: String,
    pub span: PublicSpan,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
    Implementations {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_report: Option<Box<ApplyReport>>,
}

impl ProtocolError {
    fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            message: message.into(),
            retry_fresh_query: false,
            apply_report: None,
        }
    }

    fn stale(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            message: message.into(),
            retry_fresh_query: true,
            apply_report: None,
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
    package: Package,
    semantic: Option<crate::SemanticPackage>,
    semantic_invalid: bool,
    generated: Option<GeneratedSnapshot>,
    bytes: usize,
}

#[derive(Clone, Debug)]
struct SemanticTarget {
    name: String,
    identity: String,
    declaration_span: Option<Span>,
    descriptor_identity: Option<String>,
    value_type: Option<crate::ValueType>,
    function: Option<crate::FunctionContract>,
    descriptor: Option<crate::semantics::DescriptorContract>,
}

#[derive(Clone, Debug)]
struct Continuation {
    snapshot_id: String,
    selector_hash: String,
    matches: Vec<SemanticObject>,
}

#[derive(Debug)]
pub struct ToolingEngine {
    snapshots: HashMap<String, Snapshot>,
    snapshot_order: VecDeque<String>,
    retained_bytes: usize,
    max_snapshots: usize,
    max_bytes: usize,
    continuation_order: VecDeque<String>,
    continuations: HashMap<String, Continuation>,
    proposals: HashMap<String, EditProposal>,
    canceled: BTreeSet<String>,
    canceled_order: VecDeque<String>,
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
            continuation_order: VecDeque::new(),
            proposals: HashMap::new(),
            canceled: BTreeSet::new(),
            canceled_order: VecDeque::new(),
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
            self.canceled_order
                .retain(|candidate| candidate != &request_id);
            return self.error_response(
                request_id,
                None,
                None,
                ProtocolError::new("canceled", "request was canceled"),
            );
        }
        let mut context = request_context(&envelope.request);
        let opened_source = match &envelope.request {
            Request::OpenSnapshot { sources, .. } => {
                sources.first().map(|source| source.uri.clone())
            }
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
            Request::Implementations {
                snapshot_id,
                uri,
                offset,
            } => serde_json::to_value(self.implementations(&snapshot_id, &uri, offset)?)
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
                let inserted = self.canceled.insert(request_id.clone());
                if inserted {
                    self.canceled_order.push_back(request_id);
                    if self.canceled_order.len() > 1_024
                        && let Some(expired) = self.canceled_order.pop_front()
                    {
                        self.canceled.remove(&expired);
                    }
                }
                Ok(serde_json::json!({ "canceled": true }))
            }
        }
    }

    /// Opens an immutable snapshot from explicit source and analysis inputs.
    ///
    /// # Errors
    ///
    /// Returns a protocol error for empty, duplicate, or oversized inputs.
    #[expect(
        clippy::too_many_lines,
        reason = "snapshot construction keeps hashing, parsing, semantic analysis, and retention ordered"
    )]
    pub fn open_snapshot(
        &mut self,
        mut sources: Vec<SourceInput>,
        manifest: Option<SourceInput>,
        lock: Option<SourceInput>,
        options: SnapshotOptions,
    ) -> Result<SnapshotMetadata, ProtocolError> {
        if options.target != "host" {
            return Err(ProtocolError::new(
                "unsupported-target",
                "source intelligence currently supports only the host target",
            ));
        }
        if let Some(lock) = &lock {
            let path = uri_path(&lock.uri).ok_or_else(|| {
                ProtocolError::new("invalid-lock", "lock input must use a file URI")
            })?;
            let disk = fs::read_to_string(&path).map_err(|error| {
                ProtocolError::new(
                    "invalid-lock",
                    format!(
                        "cannot read locked dependency input {}: {error}",
                        path.display()
                    ),
                )
            })?;
            if disk != lock.text {
                return Err(ProtocolError::new(
                    "lock-mismatch",
                    "provided lock input does not match the locked dependency file on disk",
                ));
            }
        }
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
                role: crate::SourceRole::Production,
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
        let package = snapshot_package(snapshot_id.clone(), units, &options, manifest.as_ref())?;
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
                        .rust_files_for(Path::new(&options.generated_entrypoint))
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
            manifest_hash: manifest.map_or(Availability::Unresolved, |input| {
                Availability::Known(hash_text(&input.text))
            }),
            lock_hash: lock.map_or(Availability::Unresolved, |input| {
                Availability::Known(hash_text(&input.text))
            }),
            target: options.target,
            profile: package.profile.name.clone(),
            capabilities: package
                .profile
                .capabilities
                .clone()
                .map_or_else(Vec::new, |capabilities| capabilities.into_iter().collect()),
            generated_entrypoint: options.generated_entrypoint.clone(),
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
            dependency_projection: semantic.as_ref().map_or_else(
                || {
                    if semantic_invalid {
                        Availability::Invalid
                    } else {
                        Availability::NotYetAnalyzed
                    }
                },
                |semantic| {
                    Availability::Known(DependencyProjectionIdentity {
                        cache_identity: semantic.projection.cache_identity.clone(),
                        content_hash: semantic.projection.content_hash.clone(),
                        dependencies: semantic
                            .projection
                            .dependencies
                            .iter()
                            .map(|dependency| {
                                format!(
                                    "{}@{} ({})",
                                    dependency.name, dependency.version, dependency.package
                                )
                            })
                            .collect(),
                    })
                },
            ),
        };
        self.retained_bytes += bytes;
        self.snapshot_order.push_back(snapshot_id.clone());
        self.snapshots.insert(
            snapshot_id,
            Snapshot {
                metadata: metadata.clone(),
                documents,
                package: package.clone(),
                semantic,
                semantic_invalid,
                generated,
                bytes,
            },
        );
        self.evict();
        Ok(metadata)
    }

    /// Returns immutable metadata for a retained snapshot.
    ///
    /// # Errors
    ///
    /// Returns a stale-snapshot error when the snapshot is no longer retained.
    pub fn metadata(&self, snapshot_id: &str) -> Result<SnapshotMetadata, ProtocolError> {
        Ok(self.snapshot(snapshot_id)?.metadata.clone())
    }

    /// Closes a snapshot and invalidates its continuations.
    ///
    /// # Errors
    ///
    /// Returns an expiry error when the snapshot is no longer retained.
    pub fn close_snapshot(&mut self, snapshot_id: &str) -> Result<(), ProtocolError> {
        let Some(snapshot) = self.snapshots.remove(snapshot_id) else {
            return Err(expired_snapshot());
        };
        self.retained_bytes = self.retained_bytes.saturating_sub(snapshot.bytes);
        self.snapshot_order
            .retain(|candidate| candidate != snapshot_id);
        self.continuations
            .retain(|_, continuation| continuation.snapshot_id != snapshot_id);
        self.continuation_order
            .retain(|token| self.continuations.contains_key(token));
        self.proposals
            .retain(|_, proposal| proposal.snapshot_id != snapshot_id);
        Ok(())
    }

    /// # Errors
    ///
    /// Returns an expiry, source, or node error for an invalid lookup.
    pub fn syntax(
        &self,
        snapshot_id: &str,
        uri: &str,
        node_id: Option<u64>,
    ) -> Result<SyntaxProjection, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let mut next_id = 0;
        let projected = project_tree(&document.tree.root, &document.diagnostics, &mut next_id);
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

    /// # Errors
    ///
    /// Returns a protocol error for stale snapshots, unknown sources, or invalid UTF-8 offsets.
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

    /// # Errors
    ///
    /// Returns a protocol error for stale snapshots or unknown sources.
    pub fn definition(
        &self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
    ) -> Result<Availability<Location>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        Ok(target_at(snapshot, document, offset).map_or_else(
            || semantic_fact_unavailable(snapshot),
            |target| {
                declaration_location(snapshot, &target)
                    .map_or(Availability::Unresolved, Availability::Known)
            },
        ))
    }

    /// # Errors
    ///
    /// Returns a protocol error for stale snapshots or unknown sources.
    pub fn references(
        &self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
    ) -> Result<Availability<Vec<Location>>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let Some(target) = target_at(snapshot, document, offset) else {
            return Ok(semantic_fact_unavailable(snapshot));
        };
        let Some(semantic) = &snapshot.semantic else {
            return Ok(semantic_fact_unavailable(snapshot));
        };
        let mut locations = Vec::new();
        for unit in semantic
            .units
            .iter()
            .filter(|unit| !unit.source_path.starts_with("<terrane>"))
        {
            let Some(document) = snapshot
                .documents
                .values()
                .find(|document| document.source.id() == unit.source.id())
            else {
                continue;
            };
            walk_nodes(&unit.tree.root, None, &mut |node, _field, _id| {
                if node.kind == SyntaxKind::Name
                    && resolve_semantic_target(snapshot, document, node)
                        .is_some_and(|candidate| candidate.identity == target.identity)
                    && let Some(location) = location_for_span(snapshot, node.span)
                {
                    locations.push(location);
                }
            });
        }
        if let Some(declaration) = declaration_location(snapshot, &target) {
            locations.push(declaration);
        }
        locations.sort();
        locations.dedup();
        Ok(Availability::Known(locations))
    }

    /// Returns concrete implementations or descendants of the semantic target.
    ///
    /// # Errors
    ///
    /// Returns a protocol error for stale snapshots or unknown sources.
    pub fn implementations(
        &self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
    ) -> Result<Availability<Vec<Location>>, ProtocolError> {
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let Some(target) = target_at(snapshot, document, offset) else {
            return Ok(semantic_fact_unavailable(snapshot));
        };
        let Some(semantic) = &snapshot.semantic else {
            return Ok(semantic_fact_unavailable(snapshot));
        };
        let owner = target
            .function
            .as_ref()
            .and_then(|function| function.owner_identity.as_ref())
            .map(crate::semantics::ObjectIdentity::qualified)
            .or_else(|| {
                target
                    .descriptor
                    .as_ref()
                    .map(|descriptor| descriptor.identity.qualified())
            });
        let Some(owner) = owner else {
            return Ok(Availability::Known(Vec::new()));
        };
        let mut locations = Vec::new();
        if target.function.is_some() {
            let identities = rename_identity_family(semantic, &target);
            if identities.len() > 1 {
                for candidate in all_functions(semantic).filter(|candidate| {
                    identities.contains(&function_target(semantic, candidate).identity)
                        && candidate.owner_identity.as_ref().is_some_and(|owner| {
                            all_descriptors(semantic)
                                .find(|descriptor| descriptor.identity == *owner)
                                .is_some_and(|descriptor| {
                                    descriptor.kind != crate::semantics::ObjectKind::Interface
                                })
                        })
                }) {
                    if let Some(location) =
                        declaration_location(snapshot, &function_target(semantic, candidate))
                    {
                        locations.push(location);
                    }
                }
            }
        } else {
            for descriptor in all_descriptors(semantic).filter(|descriptor| {
                descriptor.identity.qualified() != owner
                    && descriptor_lineage(semantic, descriptor).contains(&owner)
            }) {
                if let Some(location) =
                    declaration_location(snapshot, &descriptor_target(descriptor))
                {
                    locations.push(location);
                }
            }
        }
        locations.sort();
        locations.dedup();
        Ok(Availability::Known(locations))
    }

    /// # Errors
    ///
    /// Returns a protocol error for stale snapshots, selectors, or continuations.
    pub fn find(
        &mut self,
        snapshot_id: &str,
        selector: &Selector,
        page_size: Option<usize>,
        continuation_token: Option<&str>,
    ) -> Result<QueryPage, ProtocolError> {
        let selector_hash = hash_json(selector)?;
        let mut matches = if let Some(token) = continuation_token {
            let continuation = self.continuations.remove(token).ok_or_else(|| {
                ProtocolError::stale("expired-continuation", "continuation has expired")
            })?;
            self.continuation_order
                .retain(|candidate| candidate != token);
            if continuation.snapshot_id != snapshot_id
                || continuation.selector_hash != selector_hash
            {
                return Err(ProtocolError::stale(
                    "stale-continuation",
                    "continuation belongs to a different snapshot or query",
                ));
            }
            continuation.matches
        } else {
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
            matches
        };
        let page_size = page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(1, MAX_PAGE_SIZE);
        let remaining = (matches.len() > page_size).then(|| matches.split_off(page_size));
        let complete = remaining.is_none();
        let continuation = remaining.map(|matches| {
            self.nonce = self.nonce.wrapping_add(1);
            let token = hash_text(&format!(
                "{snapshot_id}:{selector_hash}:{}:{}",
                matches.len(),
                self.nonce
            ));
            self.continuations.insert(
                token.clone(),
                Continuation {
                    snapshot_id: snapshot_id.to_owned(),
                    selector_hash,
                    matches,
                },
            );
            self.continuation_order.push_back(token.clone());
            if self.continuation_order.len() > 128
                && let Some(expired) = self.continuation_order.pop_front()
            {
                self.continuations.remove(&expired);
            }
            token
        });
        Ok(QueryPage {
            matches,
            complete,
            continuation,
        })
    }

    /// # Errors
    ///
    /// Returns a protocol error for stale snapshots, unknown nodes, or mismatched builds.
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

    /// # Errors
    ///
    /// Returns a protocol error when edit spans or candidate source are invalid.
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
                .map(|uri| {
                    snapshot.documents.get(uri).map_or_else(
                        || {
                            Err(ProtocolError::new(
                                "unknown-source",
                                "replacement source left the snapshot",
                            ))
                        },
                        |document| {
                            Ok(AffectedFile {
                                uri: uri.to_owned(),
                                content_hash: document.identity.content_hash.clone(),
                            })
                        },
                    )
                })
                .collect::<Result<Vec<_>, ProtocolError>>()?;
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

    /// # Errors
    ///
    /// Returns a protocol error when the symbol, new name, or semantic rewrite is invalid.
    pub fn propose_rename(
        &mut self,
        snapshot_id: &str,
        uri: &str,
        offset: usize,
        new_name: &str,
    ) -> Result<EditProposal, ProtocolError> {
        if !valid_identifier(new_name) || crate::syntax::is_reserved_declaration_name(new_name) {
            return Err(ProtocolError::new(
                "invalid-name",
                "rename target must be a Terrane identifier",
            ));
        }
        let snapshot = self.snapshot(snapshot_id)?;
        let document = snapshot.document(uri)?;
        let Some(target) = target_at(snapshot, document, offset) else {
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
        let identities = rename_identity_family(semantic, &target);
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
            walk_nodes(&unit.tree.root, None, &mut |node, _field, _id| {
                if node.kind == SyntaxKind::Name
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
        if let Some(location) = declaration_location(snapshot, &target)
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
            let Some(unit) = semantic.units.iter().find(|unit| {
                snapshot.uri_for_file(unit.source.id()) == Some(replacement.uri.as_str())
            }) else {
                return Err(ProtocolError::new(
                    "unknown-source",
                    "rename source left the semantic snapshot",
                ));
            };
            if rename_would_capture(
                semantic,
                unit,
                &target,
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
        let proposal = self.propose_edits(snapshot_id, replacements)?;
        if !proposal.semantic_reanalysis || !proposal.preview_diagnostics.is_empty() {
            let message = proposal.preview_diagnostics.first().map_or_else(
                || "rename candidate does not preserve valid package semantics".to_owned(),
                |diagnostic| {
                    format!(
                        "rename candidate does not preserve valid package semantics; first diagnostic {}: {}",
                        diagnostic.code, diagnostic.message
                    )
                },
            );
            self.proposals.remove(&proposal.proposal_id);
            return Err(ProtocolError::new("invalid-rename", message));
        }
        Ok(proposal)
    }

    /// # Errors
    ///
    /// Returns a protocol error when preflight or a filesystem replacement fails.
    pub fn apply_edits(&mut self, proposal_id: &str) -> Result<ApplyReport, ProtocolError> {
        let proposal = self.proposals.get(proposal_id).cloned().ok_or_else(|| {
            ProtocolError::new("unknown-proposal", "edit proposal does not exist")
        })?;
        validate_applicable_proposal(&proposal)?;
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
                return Err(ProtocolError {
                    code: "partial-apply".to_owned(),
                    message: format!("failed to replace {}: {error}", path.display()),
                    retry_fresh_query: false,
                    apply_report: Some(Box::new(report)),
                });
            }
            report.committed.push(uri.clone());
        }
        self.proposals.remove(proposal_id);
        Ok(report)
    }

    /// # Errors
    ///
    /// Returns a protocol error for stale snapshots or unknown sources.
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
            self.continuation_order
                .retain(|token| self.continuations.contains_key(token));
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
    manifest: Option<&SourceInput>,
) -> Result<Package, ProtocolError> {
    if let Some(manifest) = manifest {
        let path = uri_path(&manifest.uri).ok_or_else(|| {
            ProtocolError::new("invalid-manifest", "manifest input must use a file URI")
        })?;
        return Package::from_tooling_sources(&path, &manifest.text, units).map_err(|errors| {
            ProtocolError::new(
                "invalid-manifest",
                errors
                    .into_iter()
                    .map(|error| error.diagnostic.message)
                    .collect::<Vec<_>>()
                    .join("; "),
            )
        });
    }
    let capabilities =
        (!options.capabilities.is_empty()).then(|| options.capabilities.iter().cloned().collect());
    let profile = CapabilityProfile {
        name: options.profile.clone(),
        capabilities,
        panic: PanicProfile::Unwind,
    };
    Ok(Package {
        identity,
        root: PathBuf::from("."),
        prelude: true,
        reflection: ReflectionProfile::Ordinary,
        executor: ExecutorProfile::Threaded,
        artifact: crate::ArtifactKind::Executable,
        profile: profile.clone(),
        purpose: crate::PackagePurpose::Production,
        testing: crate::testing::TestConfiguration::conventional(profile),
        build_toolchain: BuildToolchain::Pinned,
        units,
        rust_dependencies: Vec::new(),
        authored_rust_modules: Vec::new(),
        terrane_dependencies: Vec::new(),
        library_source_ids: BTreeSet::new(),
    })
}

fn project_tree(
    node: &SyntaxNode,
    diagnostics: &[Diagnostic],
    next_id: &mut u64,
) -> SyntaxNodeProjection {
    let id = *next_id;
    *next_id = next_id.saturating_add(1);
    let children = node
        .children
        .iter()
        .enumerate()
        .map(|(index, child)| SyntaxChild {
            field: node.kind.child_field(index, child.kind).to_owned(),
            node: project_tree(child, diagnostics, next_id),
        })
        .collect::<Vec<_>>();
    SyntaxNodeProjection {
        id,
        kind: format!("{:?}", node.kind),
        span: node.span.into(),
        state: syntax_state(node, diagnostics),
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

fn syntax_state(node: &SyntaxNode, diagnostics: &[Diagnostic]) -> SyntaxState {
    match node.kind {
        SyntaxKind::Error => SyntaxState::Error,
        SyntaxKind::Unsupported => SyntaxState::Unsupported,
        _ if diagnostics.iter().any(|diagnostic| {
            diagnostic.primary.is_some_and(|span| {
                node.span.start <= span.start
                    && span.end <= node.span.end
                    && !node
                        .children
                        .iter()
                        .any(|child| child.span.start <= span.start && span.end <= child.span.end)
            })
        }) =>
        {
            SyntaxState::Recovery
        }
        _ if node
            .children
            .iter()
            .any(|child| syntax_state(child, diagnostics) != SyntaxState::Complete) =>
        {
            SyntaxState::ContainsRecovery
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
            recurse(
                child,
                Some(node.kind.child_field(index, child.kind)),
                next,
                visit,
            );
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
    let (target, value_type) = semantic_target_and_type(snapshot, document, node);
    let known_or_unavailable = |value: Option<String>| {
        value.map_or_else(|| semantic_fact_unavailable(snapshot), Availability::Known)
    };
    let effects = target.as_ref().and_then(|target| {
        target.function.as_ref().map(|function| {
            let mut effects = Vec::new();
            if function.is_async {
                effects.push("async".to_owned());
            }
            effects.extend(
                function
                    .escaping_throwables
                    .iter()
                    .map(|throwable| format!("throws {throwable}")),
            );
            effects
        })
    });
    let invocation_mode = target.as_ref().and_then(|target| {
        target
            .function
            .as_ref()
            .map(|function| format!("{:?}", function.exact_invocation_mode).to_lowercase())
    });
    let members = target
        .as_ref()
        .and_then(|target| target.descriptor.as_ref())
        .map(|descriptor| member_facts(snapshot, descriptor));
    let inheritance = target
        .as_ref()
        .and_then(|target| target.descriptor.as_ref())
        .map(|descriptor| {
            descriptor
                .base
                .iter()
                .chain(&descriptor.interfaces)
                .chain(&descriptor.traits)
                .map(crate::semantics::ObjectIdentity::qualified)
                .collect()
        });
    let capabilities = target.as_ref().map(|target| {
        crate::semantics::namespace_capabilities(
            target.identity.split("::").next().unwrap_or_default(),
        )
        .iter()
        .map(ToString::to_string)
        .collect()
    });
    let declaration = if target
        .as_ref()
        .is_some_and(|target| target.identity == "/core/types::value.type")
    {
        Availability::Unsupported
    } else {
        target
            .as_ref()
            .and_then(|target| declaration_location(snapshot, target))
            .map_or_else(|| semantic_fact_unavailable(snapshot), Availability::Known)
    };
    SemanticObject {
        source_uri: uri.unwrap_or(&document.identity.uri).to_owned(),
        span: node.span.into(),
        syntax_node_id: id,
        kind: format!("{:?}", node.kind),
        state: syntax_state(node, &document.diagnostics),
        name,
        symbol_identity: known_or_unavailable(
            target.as_ref().map(|target| target.identity.clone()),
        ),
        descriptor_identity: known_or_unavailable(
            target
                .as_ref()
                .and_then(|target| target.descriptor_identity.clone()),
        ),
        value_type: known_or_unavailable(value_type.as_ref().map(ToString::to_string)),
        ownership: Availability::Unsupported,
        effects: semantic_optional_fact(snapshot, effects),
        capabilities: semantic_optional_fact(snapshot, capabilities),
        declaration,
        invocation_mode: semantic_optional_fact(snapshot, invocation_mode),
        members: semantic_optional_fact(snapshot, members),
        inheritance: semantic_optional_fact(snapshot, inheritance),
    }
}

fn semantic_target_and_type(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    node: &SyntaxNode,
) -> (Option<SemanticTarget>, Option<crate::ValueType>) {
    let target = (node.kind == SyntaxKind::Name)
        .then(|| resolve_semantic_target(snapshot, document, node))
        .flatten();
    let inferred = snapshot.semantic.as_ref().and_then(|semantic| {
        semantic
            .units
            .iter()
            .find(|unit| unit.source.id() == document.source.id())?
            .inferred_value_type(node)
    });
    let value_type = target
        .as_ref()
        .and_then(|target| target.value_type.clone())
        .or(inferred);
    (target, value_type)
}

fn resolve_semantic_target(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    name_node: &SyntaxNode,
) -> Option<SemanticTarget> {
    let semantic = snapshot.semantic.as_ref()?;
    let unit = semantic
        .units
        .iter()
        .find(|unit| unit.source.id() == document.source.id())?;
    let name = node_text(&document.source, name_node);
    for function in all_functions(semantic) {
        if function.name == name
            && declaration_name_span(snapshot, function.span, name) == Some(name_node.span)
        {
            return Some(function_target(semantic, function));
        }
    }
    for descriptor in all_descriptors(semantic) {
        if descriptor.name == name
            && declaration_name_span(snapshot, descriptor.span, name) == Some(name_node.span)
        {
            return Some(descriptor_target(descriptor));
        }
    }
    if let Some(symbol) = unit
        .scopes
        .iter()
        .filter_map(|scope| scope.symbols.get(name))
        .flatten()
        .find(|symbol| {
            symbol.declaration_span.is_some_and(|span| {
                span.file == name_node.span.file
                    && span.start <= name_node.span.start
                    && name_node.span.end <= span.end
            })
        })
    {
        return Some(symbol_target(semantic, unit, symbol));
    }
    if let Some(parent) = parent_node(&document.tree.root, name_node)
        && matches!(
            parent.kind,
            SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
        )
        && parent
            .children
            .last()
            .is_some_and(|child| child.span == name_node.span)
    {
        return member_target(semantic, unit, parent, name);
    }
    let symbol = semantic.resolve_name_at(unit, name_node.span.start, name)?;
    Some(symbol_target(semantic, unit, symbol))
}

fn symbol_target(
    semantic: &crate::SemanticPackage,
    unit: &crate::SemanticUnit,
    symbol: &crate::semantics::Symbol,
) -> SemanticTarget {
    let function = symbol
        .declaration_span
        .and_then(|span| {
            all_functions(semantic)
                .find(|function| function.span == span && function.name == symbol.name)
        })
        .cloned();
    let descriptor = symbol
        .descriptor_identity()
        .and_then(|identity| {
            all_descriptors(semantic).find(|descriptor| descriptor.identity.qualified() == identity)
        })
        .cloned();
    let binding_span = symbol.binding_span.or(symbol.declaration_span);
    let value_type = function.as_ref().and_then(function_value_type).or_else(|| {
        unit.typed_bindings
            .iter()
            .find(|binding| {
                binding.name == symbol.name && binding_span.is_some_and(|span| binding.span == span)
            })
            .map(|binding| binding.value_type.clone())
    });
    SemanticTarget {
        name: symbol.name.clone(),
        identity: symbol.identity.clone(),
        declaration_span: symbol.declaration_span,
        descriptor_identity: symbol.descriptor_identity().map(str::to_owned),
        value_type,
        function,
        descriptor,
    }
}

fn member_target(
    semantic: &crate::SemanticPackage,
    unit: &crate::SemanticUnit,
    member_expression: &SyntaxNode,
    name: &str,
) -> Option<SemanticTarget> {
    let receiver = member_expression.children.first()?;
    let receiver_type = unit.inferred_value_type(receiver)?;
    if name == "type" {
        let descriptor_identity = match unwrapped_value_type(&receiver_type) {
            crate::ValueType::Object(identity) => identity.qualified(),
            crate::ValueType::Descriptor(identity) => identity.clone(),
            value_type => value_type.to_string(),
        };
        return Some(SemanticTarget {
            name: name.to_owned(),
            identity: "/core/types::value.type".to_owned(),
            declaration_span: None,
            descriptor_identity: Some(descriptor_identity),
            value_type: unit.inferred_value_type(member_expression),
            function: None,
            descriptor: None,
        });
    }
    let descriptor_identity = match unwrapped_value_type(&receiver_type) {
        crate::ValueType::Object(identity) => identity.qualified(),
        crate::ValueType::Descriptor(identity) => identity.clone(),
        _ => return None,
    };
    let descriptor = all_descriptors(semantic)
        .find(|descriptor| descriptor.identity.qualified() == descriptor_identity)?;
    let is_static = member_expression.kind == SyntaxKind::StaticMemberExpression;
    if let Some(field_target) = resolved_member_field_target(semantic, descriptor, name, is_static)
    {
        return Some(field_target);
    }
    if let Some(function) = resolved_member_function(semantic, descriptor, name, is_static) {
        return Some(function_target(semantic, function));
    }
    let lineage = descriptor_lineage(semantic, descriptor);
    for owner in lineage {
        let descriptor = all_descriptors(semantic)
            .find(|descriptor| descriptor.identity.qualified() == owner)?;
        if descriptor.members.contains(name) || descriptor.static_members.contains(name) {
            return Some(SemanticTarget {
                name: name.to_owned(),
                identity: descriptor
                    .operations
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| format!("{}::{name}", descriptor.identity.qualified())),
                declaration_span: None,
                descriptor_identity: Some(descriptor.identity.qualified()),
                value_type: crate::semantics::object_member_type(
                    unit,
                    &descriptor.identity,
                    name,
                    is_static,
                ),
                function: None,
                descriptor: None,
            });
        }
    }
    None
}

fn resolved_member_field_target(
    semantic: &crate::SemanticPackage,
    descriptor: &crate::semantics::DescriptorContract,
    name: &str,
    is_static: bool,
) -> Option<SemanticTarget> {
    fn resolve(
        semantic: &crate::SemanticPackage,
        descriptor: &crate::semantics::DescriptorContract,
        name: &str,
        is_static: bool,
        visited: &mut BTreeSet<crate::semantics::ObjectIdentity>,
    ) -> Option<SemanticTarget> {
        if !visited.insert(descriptor.identity.clone()) {
            return None;
        }
        if let Some(field) = descriptor
            .fields
            .iter()
            .find(|field| field.name == name && field.is_static == is_static)
        {
            return Some(SemanticTarget {
                name: name.to_owned(),
                identity: format!("{}::{name}", descriptor.identity.qualified()),
                declaration_span: Some(field.span),
                descriptor_identity: Some(descriptor.identity.qualified()),
                value_type: Some(field.value_type.clone()),
                function: None,
                descriptor: None,
            });
        }
        descriptor
            .traits
            .iter()
            .filter_map(|identity| {
                all_descriptors(semantic).find(|candidate| candidate.identity == *identity)
            })
            .find_map(|parent| resolve(semantic, parent, name, is_static, visited))
            .or_else(|| {
                descriptor
                    .base
                    .as_ref()
                    .and_then(|identity| {
                        all_descriptors(semantic).find(|candidate| candidate.identity == *identity)
                    })
                    .and_then(|parent| resolve(semantic, parent, name, is_static, visited))
            })
    }
    resolve(semantic, descriptor, name, is_static, &mut BTreeSet::new())
}

fn resolved_member_function<'a>(
    semantic: &'a crate::SemanticPackage,
    descriptor: &crate::semantics::DescriptorContract,
    name: &str,
    is_static: bool,
) -> Option<&'a crate::FunctionContract> {
    fn resolve<'a>(
        semantic: &'a crate::SemanticPackage,
        descriptor: &crate::semantics::DescriptorContract,
        name: &str,
        is_static: bool,
        visited: &mut BTreeSet<crate::semantics::ObjectIdentity>,
    ) -> Option<&'a crate::FunctionContract> {
        if !visited.insert(descriptor.identity.clone()) {
            return None;
        }
        if let Some(method) = all_functions(semantic).find(|function| {
            function.owner_identity.as_ref() == Some(&descriptor.identity)
                && function.name == name
                && function.is_static == is_static
        }) {
            return Some(method);
        }
        descriptor
            .traits
            .iter()
            .filter_map(|identity| {
                all_descriptors(semantic).find(|candidate| candidate.identity == *identity)
            })
            .find_map(|parent| resolve(semantic, parent, name, is_static, visited))
            .or_else(|| {
                descriptor
                    .base
                    .as_ref()
                    .and_then(|identity| {
                        all_descriptors(semantic).find(|candidate| candidate.identity == *identity)
                    })
                    .and_then(|parent| resolve(semantic, parent, name, is_static, visited))
            })
            .or_else(|| {
                descriptor
                    .interfaces
                    .iter()
                    .filter_map(|identity| {
                        all_descriptors(semantic).find(|candidate| candidate.identity == *identity)
                    })
                    .find_map(|parent| resolve(semantic, parent, name, is_static, visited))
            })
    }
    resolve(semantic, descriptor, name, is_static, &mut BTreeSet::new())
}

fn function_target(
    semantic: &crate::SemanticPackage,
    function: &crate::FunctionContract,
) -> SemanticTarget {
    let identity = function.owner_identity.as_ref().map_or_else(
        || {
            semantic
                .units
                .iter()
                .find(|unit| unit.source.id() == function.span.file)
                .and_then(|unit| semantic.namespaces.get(&unit.namespace))
                .and_then(|namespace| namespace.symbols.get(&function.name))
                .map_or_else(
                    || format!("<unknown>::{}", function.name),
                    |symbol| symbol.identity.clone(),
                )
        },
        |owner| format!("{}::{}", owner.qualified(), function.name),
    );
    SemanticTarget {
        name: function.name.clone(),
        identity,
        declaration_span: Some(function.span),
        descriptor_identity: function
            .owner_identity
            .as_ref()
            .map(crate::semantics::ObjectIdentity::qualified),
        value_type: function_value_type(function),
        function: Some(function.clone()),
        descriptor: None,
    }
}

fn descriptor_target(descriptor: &crate::semantics::DescriptorContract) -> SemanticTarget {
    let identity = descriptor.identity.qualified();
    SemanticTarget {
        name: descriptor.name.clone(),
        identity: identity.clone(),
        declaration_span: Some(descriptor.span),
        descriptor_identity: Some(identity),
        value_type: Some(crate::ValueType::Descriptor(
            descriptor.identity.qualified(),
        )),
        function: None,
        descriptor: Some(descriptor.clone()),
    }
}

fn function_value_type(function: &crate::FunctionContract) -> Option<crate::ValueType> {
    let parameters = function
        .parameters
        .iter()
        .map(crate::ParameterContract::callable_type)
        .collect::<Option<Vec<_>>>()?;
    let result = crate::semantics::ElementType::new(
        function
            .return_type
            .clone()
            .unwrap_or(crate::ValueType::Scalar(crate::ScalarType::None)),
    );
    let effects = crate::semantics::CallableEffects::from_contract(function);
    Some(if function.is_async {
        crate::ValueType::AsyncFunction(parameters, result, function.task_transferability, effects)
    } else {
        crate::ValueType::Function(parameters, result, effects)
    })
}

fn all_functions(
    semantic: &crate::SemanticPackage,
) -> impl Iterator<Item = &crate::FunctionContract> {
    semantic.units.iter().flat_map(|unit| &unit.functions)
}

fn all_descriptors(
    semantic: &crate::SemanticPackage,
) -> impl Iterator<Item = &crate::semantics::DescriptorContract> {
    semantic.units.iter().flat_map(|unit| &unit.descriptors)
}

fn descriptor_lineage(
    semantic: &crate::SemanticPackage,
    descriptor: &crate::semantics::DescriptorContract,
) -> BTreeSet<String> {
    fn visit(
        semantic: &crate::SemanticPackage,
        identity: &crate::semantics::ObjectIdentity,
        output: &mut BTreeSet<String>,
    ) {
        if !output.insert(identity.qualified()) {
            return;
        }
        if let Some(descriptor) =
            all_descriptors(semantic).find(|descriptor| descriptor.identity == *identity)
        {
            for parent in descriptor
                .base
                .iter()
                .chain(&descriptor.interfaces)
                .chain(&descriptor.traits)
            {
                visit(semantic, parent, output);
            }
        }
    }
    let mut output = BTreeSet::new();
    visit(semantic, &descriptor.identity, &mut output);
    output
}

fn rename_identity_family(
    semantic: &crate::SemanticPackage,
    target: &SemanticTarget,
) -> BTreeSet<String> {
    let mut identities = BTreeSet::from([target.identity.clone()]);
    let Some(function) = &target.function else {
        return identities;
    };
    let Some(owner) = function.owner_identity.as_ref().and_then(|owner| {
        all_descriptors(semantic).find(|descriptor| descriptor.identity == *owner)
    }) else {
        return identities;
    };
    let owner_lineage = descriptor_lineage(semantic, owner);
    let contracts = all_descriptors(semantic)
        .filter(|descriptor| {
            descriptor.kind == crate::semantics::ObjectKind::Interface
                && owner_lineage.contains(&descriptor.identity.qualified())
                && all_functions(semantic).any(|candidate| {
                    candidate.owner_identity.as_ref() == Some(&descriptor.identity)
                        && candidate.name == function.name
                        && candidate.is_static == function.is_static
                })
        })
        .map(|descriptor| descriptor.identity.qualified())
        .collect::<BTreeSet<_>>();
    if contracts.is_empty() {
        return identities;
    }
    for candidate in all_functions(semantic).filter(|candidate| {
        candidate.name == function.name
            && candidate.is_static == function.is_static
            && candidate
                .owner_identity
                .as_ref()
                .and_then(|owner| {
                    all_descriptors(semantic).find(|descriptor| descriptor.identity == *owner)
                })
                .is_some_and(|owner| {
                    let lineage = descriptor_lineage(semantic, owner);
                    contracts.iter().any(|contract| lineage.contains(contract))
                })
    }) {
        identities.insert(function_target(semantic, candidate).identity);
    }
    identities
}

fn unwrapped_value_type(value_type: &crate::ValueType) -> &crate::ValueType {
    match value_type {
        crate::ValueType::Reference(inner) | crate::ValueType::SharedReference(inner) => {
            unwrapped_value_type(inner.value_type_ref())
        }
        _ => value_type,
    }
}

fn parent_node<'a>(root: &'a SyntaxNode, target: &SyntaxNode) -> Option<&'a SyntaxNode> {
    for child in &root.children {
        if child.span == target.span && child.kind == target.kind {
            return Some(root);
        }
        if child.span.start <= target.span.start
            && target.span.end <= child.span.end
            && let Some(parent) = parent_node(child, target)
        {
            return Some(parent);
        }
    }
    None
}

fn semantic_fact_unavailable<T>(snapshot: &Snapshot) -> Availability<T> {
    if snapshot.semantic_invalid {
        Availability::Invalid
    } else if snapshot.semantic.is_some() {
        Availability::Unresolved
    } else {
        Availability::NotYetAnalyzed
    }
}

fn semantic_optional_fact<T>(snapshot: &Snapshot, value: Option<T>) -> Availability<T> {
    value.map_or_else(
        || {
            if snapshot.semantic_invalid {
                Availability::Invalid
            } else if snapshot.semantic.is_some() {
                Availability::Unsupported
            } else {
                Availability::NotYetAnalyzed
            }
        },
        Availability::Known,
    )
}

fn target_at(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    offset: usize,
) -> Option<SemanticTarget> {
    let (node, _, _) = smallest_node_at(&document.tree.root, offset, 0)?;
    let name_node = if node.kind == SyntaxKind::Name {
        node
    } else {
        nearest_name(node, offset)?
    };
    resolve_semantic_target(snapshot, document, name_node)
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
fn rename_would_capture(
    semantic: &crate::SemanticPackage,
    unit: &crate::SemanticUnit,
    target: &SemanticTarget,
    identities: &BTreeSet<String>,
    new_name: &str,
    offset: usize,
) -> bool {
    if target
        .function
        .as_ref()
        .and_then(|function| function.owner_identity.as_ref())
        .is_some()
    {
        for function in all_functions(semantic)
            .filter(|function| identities.contains(&function_target(semantic, function).identity))
        {
            let Some(owner) = &function.owner_identity else {
                continue;
            };
            if all_descriptors(semantic).any(|descriptor| {
                descriptor.identity == *owner
                    && descriptor.fields.iter().any(|field| field.name == new_name)
            }) {
                return true;
            }
            if all_functions(semantic).any(|candidate| {
                candidate.name == new_name
                    && candidate.owner_identity.as_ref() == Some(owner)
                    && !identities.contains(&function_target(semantic, candidate).identity)
            }) {
                return true;
            }
        }
        return false;
    }
    semantic
        .resolve_name_at(unit, offset, new_name)
        .is_some_and(|candidate| !identities.contains(&candidate.identity))
}

fn declaration_location(snapshot: &Snapshot, target: &SemanticTarget) -> Option<Location> {
    let declaration = target.declaration_span?;
    location_for_span(
        snapshot,
        declaration_name_span(snapshot, declaration, &target.name).unwrap_or(declaration),
    )
}

fn declaration_name_span(snapshot: &Snapshot, declaration: Span, name: &str) -> Option<Span> {
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
            && node_text(&document.source, node) == name
        {
            name_span = Some(node.span);
        }
    });
    name_span
}

fn location_for_span(snapshot: &Snapshot, span: Span) -> Option<Location> {
    Some(Location {
        uri: snapshot.uri_for_file(span.file)?.to_owned(),
        span: span.into(),
    })
}

fn member_facts(
    snapshot: &Snapshot,
    descriptor: &crate::semantics::DescriptorContract,
) -> Vec<MemberFact> {
    let mut members = BTreeMap::new();
    for field in &descriptor.fields {
        members.insert(
            field.name.clone(),
            MemberFact {
                name: field.name.clone(),
                identity: format!("{}::{}", descriptor.identity.qualified(), field.name),
                kind: "field".to_owned(),
                value_type: Availability::Known(field.value_type.to_string()),
                declaration: location_for_span(snapshot, field.span)
                    .map_or(Availability::Unresolved, Availability::Known),
            },
        );
    }
    if let Some(semantic) = &snapshot.semantic {
        let lineage = descriptor_lineage(semantic, descriptor);
        for inherited in all_descriptors(semantic).filter(|candidate| {
            candidate.identity != descriptor.identity
                && lineage.contains(&candidate.identity.qualified())
        }) {
            for field in &inherited.fields {
                members
                    .entry(field.name.clone())
                    .or_insert_with(|| MemberFact {
                        name: field.name.clone(),
                        identity: format!("{}::{}", inherited.identity.qualified(), field.name),
                        kind: "field".to_owned(),
                        value_type: Availability::Known(field.value_type.to_string()),
                        declaration: location_for_span(snapshot, field.span)
                            .map_or(Availability::Unresolved, Availability::Known),
                    });
            }
        }
        for function in all_functions(semantic).filter(|function| {
            function
                .owner_identity
                .as_ref()
                .is_some_and(|owner| lineage.contains(&owner.qualified()))
        }) {
            let target = function_target(semantic, function);
            members.entry(function.name.clone()).or_insert(MemberFact {
                name: function.name.clone(),
                identity: target.identity.clone(),
                kind: "method".to_owned(),
                value_type: target
                    .value_type
                    .as_ref()
                    .map_or(Availability::Unresolved, |value| {
                        Availability::Known(value.to_string())
                    }),
                declaration: declaration_location(snapshot, &target)
                    .map_or(Availability::Unresolved, Availability::Known),
            });
        }
    }
    if descriptor.members.contains("type") {
        members.insert(
            "type".to_owned(),
            MemberFact {
                name: "type".to_owned(),
                identity: "/core/types::value.type".to_owned(),
                kind: "property".to_owned(),
                value_type: Availability::Known(
                    crate::ValueType::Descriptor(descriptor.identity.qualified()).to_string(),
                ),
                declaration: Availability::Unsupported,
            },
        );
    }
    for name in descriptor
        .members
        .iter()
        .chain(&descriptor.static_members)
        .filter(|name| name.as_str() != "type")
    {
        members.entry(name.clone()).or_insert_with(|| MemberFact {
            name: name.clone(),
            identity: descriptor
                .operations
                .get(name)
                .cloned()
                .unwrap_or_else(|| format!("{}::{name}", descriptor.identity.qualified())),
            kind: if descriptor.static_members.contains(name) {
                "static".to_owned()
            } else {
                "member".to_owned()
            },
            value_type: Availability::Unresolved,
            declaration: Availability::Unresolved,
        });
    }
    members.into_values().collect()
}

fn selector_matches(
    snapshot: &Snapshot,
    document: &ParsedDocument,
    node: &SyntaxNode,
    field: Option<&str>,
    selector: &Selector,
) -> bool {
    if !selector.include_recovery
        && matches!(
            syntax_state(node, &document.diagnostics),
            SyntaxState::Error | SyntaxState::Recovery | SyntaxState::Unsupported
        )
    {
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

fn validate_applicable_proposal(proposal: &EditProposal) -> Result<(), ProtocolError> {
    if !proposal.preview_diagnostics.is_empty() {
        return Err(ProtocolError::new(
            "invalid-proposal",
            "edit proposal has preview diagnostics and cannot be applied",
        ));
    }
    Ok(())
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

type CandidateSources = (BTreeMap<String, String>, Vec<DiagnosticProjection>, bool);

fn candidate_sources(
    snapshot: &Snapshot,
    replacements: &[Replacement],
) -> Result<CandidateSources, ProtocolError> {
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
        let original = snapshot
            .package
            .units
            .iter()
            .find(|unit| unit.source.path() == document.source.path());
        units.push(SourceUnit {
            relative_path: original.map_or_else(
                || PathBuf::from(format!("snapshot/{file_id}.trn")),
                |unit| unit.relative_path.clone(),
            ),
            source,
            expected_namespace: original.and_then(|unit| unit.expected_namespace.clone()),
            role: original.map_or(crate::SourceRole::Production, |unit| unit.role),
        });
        candidate.insert(uri.clone(), text);
    }
    let semantic_reanalysis = if diagnostics.is_empty() && snapshot.semantic.is_some() {
        let mut package = snapshot.package.clone();
        package.units = units;
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
    let mut protected = document
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
    protected.extend(document.diagnostics.iter().filter_map(|diagnostic| {
        diagnostic
            .primary
            .map(|span| diagnostic_line_span(document.source.text(), span))
    }));
    let source = document.source.text();
    let mut edits = Vec::<(usize, usize, &'static str)>::new();
    let mut offset = 0;
    for raw in source.split_inclusive('\n') {
        let content = raw.trim_end_matches(['\r', '\n']);
        let trimmed = content.trim_end_matches([' ', '\t']);
        let start = offset + trimmed.len();
        let end = offset + content.len();
        if start < end && !overlaps_protected(start, end, &protected) {
            edits.push((start, end, ""));
        }
        offset += raw.len();
    }
    collect_operator_spacing(
        &document.tree.root,
        &document.tree.lexed.tokens,
        source,
        &protected,
        &mut edits,
    );
    for (index, token) in document.tree.lexed.tokens.iter().enumerate() {
        if token.kind == TokenKind::Assign {
            add_token_spacing(
                index,
                &document.tree.lexed.tokens,
                source,
                &protected,
                &mut edits,
            );
        }
    }
    edits.sort_unstable_by_key(|(start, end, _)| (*start, *end));
    edits.dedup();
    let mut output = source.to_owned();
    for (start, end, replacement) in edits.into_iter().rev() {
        output.replace_range(start..end, replacement);
    }
    output
}

fn collect_operator_spacing(
    node: &SyntaxNode,
    tokens: &[crate::tokens::Token],
    source: &str,
    protected: &[Span],
    edits: &mut Vec<(usize, usize, &'static str)>,
) {
    if node.kind == SyntaxKind::BinaryExpression
        && let [left, right, ..] = node.children.as_slice()
        && let Some(index) = node.token_range.clone().find(|index| {
            tokens[*index].kind == TokenKind::Operator
                && left.span.end <= tokens[*index].span.start
                && tokens[*index].span.end <= right.span.start
        })
    {
        add_token_spacing(index, tokens, source, protected, edits);
    }
    for child in &node.children {
        collect_operator_spacing(child, tokens, source, protected, edits);
    }
}

fn add_token_spacing(
    index: usize,
    tokens: &[crate::tokens::Token],
    source: &str,
    protected: &[Span],
    edits: &mut Vec<(usize, usize, &'static str)>,
) {
    let previous = tokens[..index]
        .iter()
        .rev()
        .find(|token| !is_layout_token(token.kind));
    let next = tokens[index + 1..]
        .iter()
        .find(|token| !is_layout_token(token.kind));
    if let Some(previous) = previous {
        add_safe_space(
            previous.span.end,
            tokens[index].span.start,
            source,
            protected,
            edits,
        );
    }
    if let Some(next) = next {
        add_safe_space(
            tokens[index].span.end,
            next.span.start,
            source,
            protected,
            edits,
        );
    }
}

fn add_safe_space(
    start: usize,
    end: usize,
    source: &str,
    protected: &[Span],
    edits: &mut Vec<(usize, usize, &'static str)>,
) {
    let Some(gap) = source.get(start..end) else {
        return;
    };
    if gap != " "
        && gap.chars().all(|character| matches!(character, ' ' | '\t'))
        && !overlaps_protected(start, end, protected)
    {
        edits.push((start, end, " "));
    }
}

fn diagnostic_line_span(source: &str, span: Span) -> Span {
    let mut anchor = span.start.min(source.len());
    if anchor == source.len() || source.as_bytes().get(anchor) == Some(&b'\n') {
        anchor = anchor.saturating_sub(1);
        if source.as_bytes().get(anchor) == Some(&b'\r') {
            anchor = anchor.saturating_sub(1);
        }
    }
    let start = source[..anchor].rfind('\n').map_or(0, |index| index + 1);
    let end = source[anchor..]
        .find('\n')
        .map_or(source.len(), |index| anchor + index + 1);
    Span::new(span.file, start, end)
}

fn overlaps_protected(start: usize, end: usize, protected: &[Span]) -> bool {
    protected
        .iter()
        .any(|span| span.start < end && start < span.end)
}

fn is_layout_token(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Newline | TokenKind::Indent | TokenKind::Dedent | TokenKind::Eof
    )
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
        | Request::Implementations {
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

#[expect(
    clippy::needless_pass_by_value,
    reason = "serde map_err callbacks transfer their owned error"
)]
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
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_TEMPORARY_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

    struct TemporaryDirectory(PathBuf);

    impl TemporaryDirectory {
        fn new(label: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "terrane-{label}-{}-{}",
                std::process::id(),
                NEXT_TEMPORARY_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            if path.exists() {
                fs::remove_dir_all(&path).expect("remove stale temporary directory");
            }
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TemporaryDirectory {
        fn drop(&mut self) {
            if self.0.exists() {
                fs::remove_dir_all(&self.0).expect("remove temporary directory");
            }
        }
    }

    fn has_direct_recovery(node: &SyntaxNodeProjection) -> bool {
        matches!(
            node.state,
            SyntaxState::Recovery | SyntaxState::Error | SyntaxState::Unsupported
        ) || node
            .children
            .iter()
            .any(|child| has_direct_recovery(&child.node))
    }

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
        let without_recovery = engine
            .find(
                &metadata.snapshot_id,
                &Selector {
                    kind: None,
                    child_field: None,
                    containing: None,
                    text: None,
                    token_kind: None,
                    symbol_identity: None,
                    descriptor_identity: None,
                    include_recovery: false,
                },
                Some(MAX_PAGE_SIZE),
                None,
            )
            .expect("find valid and recovery-containing nodes");
        assert!(
            without_recovery
                .matches
                .iter()
                .any(|object| object.state == SyntaxState::ContainsRecovery)
        );
        assert!(without_recovery.matches.iter().all(|object| !matches!(
            object.state,
            SyntaxState::Error | SyntaxState::Recovery | SyntaxState::Unsupported
        )));
        let with_recovery = engine
            .find(
                &metadata.snapshot_id,
                &Selector {
                    kind: None,
                    child_field: None,
                    containing: None,
                    text: None,
                    token_kind: None,
                    symbol_identity: None,
                    descriptor_identity: None,
                    include_recovery: true,
                },
                Some(MAX_PAGE_SIZE),
                None,
            )
            .expect("find directly recovered nodes");
        assert!(with_recovery.matches.iter().any(|object| matches!(
            object.state,
            SyntaxState::Error | SyntaxState::Recovery | SyntaxState::Unsupported
        )));
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
        let source = "function main;   \r\n    value int=1 +2   \r\n";
        let snapshot = open(&mut engine, uri, source, SnapshotOptions::default());
        let first = engine
            .format(&snapshot.snapshot_id, uri)
            .expect("format valid source");
        assert!(first.changed);
        assert_eq!(first.text, "function main;\r\n    value int = 1 + 2\r\n");
        let second_snapshot = open(&mut engine, uri, &first.text, SnapshotOptions::default());
        let second = engine
            .format(&second_snapshot.snapshot_id, uri)
            .expect("format second pass");
        assert!(!second.changed);
        let unary = "function main;\n    value int = -7 / -2\n";
        let unary_snapshot = open(
            &mut engine,
            "file:///workspace/unary.trn",
            unary,
            SnapshotOptions::default(),
        );
        assert_eq!(
            engine
                .format(&unary_snapshot.snapshot_id, "file:///workspace/unary.trn")
                .expect("format unary operators")
                .text,
            "function main;\n    value int = -7 / -2\n"
        );

        let partly_recovered = "function main;   \n    value int=1   \n    (   \n";
        let partial_snapshot = open(
            &mut engine,
            "file:///workspace/partial.trn",
            partly_recovered,
            SnapshotOptions::default(),
        );
        let recovery_tree = engine
            .syntax(
                &partial_snapshot.snapshot_id,
                "file:///workspace/partial.trn",
                None,
            )
            .expect("recovery projection");
        assert_eq!(recovery_tree.root.state, SyntaxState::ContainsRecovery);
        assert!(has_direct_recovery(&recovery_tree.root));
        assert_eq!(
            engine
                .format(
                    &partial_snapshot.snapshot_id,
                    "file:///workspace/partial.trn"
                )
                .expect("format safe lines around recovery")
                .text,
            "function main;\n    value int = 1\n    (   \n"
        );

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
        let directory = TemporaryDirectory::new("tooling");
        fs::create_dir_all(directory.path()).expect("create temporary directory");
        let path = directory.path().join("case.trn");
        let original = "function main;\n    value int = 1\n";
        fs::write(&path, original).expect("write fixture");
        let uri = format!("file://{}", path.display());
        let mut engine = ToolingEngine::default();
        let snapshot = open(&mut engine, &uri, original, SnapshotOptions::default());
        let start = original.find('1').expect("literal");
        let invalid = engine
            .propose_edits(
                &snapshot.snapshot_id,
                vec![Replacement {
                    uri: uri.clone(),
                    span: PublicSpan {
                        start,
                        end: start + 1,
                    },
                    text: ")".to_owned(),
                }],
            )
            .expect("diagnostic proposal");
        assert!(!invalid.preview_diagnostics.is_empty());
        let error = engine
            .apply_edits(&invalid.proposal_id)
            .expect_err("diagnostic proposal must not apply");
        assert_eq!(error.code, "invalid-proposal");
        assert_eq!(
            fs::read_to_string(&path).expect("unchanged source"),
            original
        );
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
                generated_entrypoint: "generated/inspect.rs".to_owned(),
                ..SnapshotOptions::default()
            },
        );
        let Availability::Known(build_id) = metadata.build_id.clone() else {
            panic!("build should be available");
        };
        let syntax = engine
            .syntax(&metadata.snapshot_id, uri, None)
            .expect("syntax tree");
        let main_node = syntax.root.id;
        let locations = engine
            .generated_rust(&metadata.snapshot_id, uri, main_node, &build_id)
            .expect("matching build");
        let Availability::Known(locations) = locations else {
            panic!("generated locations should be known");
        };
        assert!(
            locations
                .iter()
                .any(|location| location.path.ends_with("generated/inspect.rs")),
            "{locations:?}"
        );
        let error = engine
            .generated_rust(&metadata.snapshot_id, uri, main_node, "sha256:other")
            .expect_err("mismatched build must fail");
        assert_eq!(error.code, "mismatched-build");
    }
    #[test]
    fn member_navigation_uses_receiver_type_instead_of_bare_name_resolution() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/members.trn";
        let text = "namespace members\n\ninterface responder\n    function answer int;\n\nclass first implements responder\n    count int = 0\n\n    function answer int;\n        return 1\n\nclass second\n    function answer int;\n        return 2\n\nfunction main;\n    left = instance first;\n    right = instance second;\n    one int = left.answer;\n    two int = right.answer;\n";
        let metadata = open(
            &mut engine,
            uri,
            text,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        let left_use = text.find("left.answer").expect("left member") + "left.".len();
        let right_use = text.find("right.answer").expect("right member") + "right.".len();
        let Availability::Known(left_definition) = engine
            .definition(&metadata.snapshot_id, uri, left_use)
            .expect("left definition")
        else {
            panic!("left definition should be known");
        };
        let Availability::Known(right_definition) = engine
            .definition(&metadata.snapshot_id, uri, right_use)
            .expect("right definition")
        else {
            panic!("right definition should be known");
        };
        assert_ne!(left_definition.span, right_definition.span);
        let answer_declarations = text
            .match_indices("function answer")
            .map(|(offset, _)| offset + "function ".len())
            .collect::<Vec<_>>();
        assert_eq!(answer_declarations.len(), 3);
        assert_eq!(left_definition.span.start, answer_declarations[1]);
        assert_eq!(right_definition.span.start, answer_declarations[2]);
        let interface_object = engine
            .locate(&metadata.snapshot_id, uri, answer_declarations[0])
            .expect("interface query")
            .expect("interface method object");
        let concrete_object = engine
            .locate(&metadata.snapshot_id, uri, left_use)
            .expect("concrete query")
            .expect("concrete method object");
        let plain_object = engine
            .locate(&metadata.snapshot_id, uri, right_use)
            .expect("plain query")
            .expect("plain method object");
        assert_eq!(
            interface_object.symbol_identity,
            Availability::Known("/members::responder::answer".to_owned())
        );
        assert_eq!(
            concrete_object.symbol_identity,
            Availability::Known("/members::first::answer".to_owned())
        );
        assert_eq!(
            plain_object.symbol_identity,
            Availability::Known("/members::second::answer".to_owned())
        );
        let Availability::Known(left_references) = engine
            .references(&metadata.snapshot_id, uri, left_use)
            .expect("left references")
        else {
            panic!("left references should be known");
        };
        assert_eq!(left_references.len(), 2);
        assert!(left_references.iter().all(|location| {
            location.span != right_definition.span && location.span.start != right_use
        }));
        let capture = engine
            .propose_rename(&metadata.snapshot_id, uri, left_use, "count")
            .expect_err("method rename must not capture a field");
        assert_eq!(capture.code, "rename-capture");
        for keyword in crate::syntax::RESERVED_DECLARATION_NAMES {
            let error = engine
                .propose_rename(&metadata.snapshot_id, uri, left_use, keyword)
                .expect_err("language keyword is not a declaration name");
            assert_eq!(error.code, "invalid-name", "{keyword}");
        }
    }

    #[test]
    fn implementation_navigation_finds_descriptor_and_method_implementors() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/implementations.trn";
        let text = "namespace implementations\n\ninterface worker\n    function run int;\n\nclass first implements worker\n    function run int;\n        return 1\n\nclass second implements worker\n    function run int;\n        return 2\n\nfunction main;\n    item = instance first;\n    result int = item.run;\n";
        let metadata = open(
            &mut engine,
            uri,
            text,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        let descriptor_offset = text.find("worker").expect("interface name");
        let Availability::Known(descriptors) = engine
            .implementations(&metadata.snapshot_id, uri, descriptor_offset)
            .expect("descriptor implementations")
        else {
            panic!("descriptor implementations should be known");
        };
        assert_eq!(descriptors.len(), 2);

        let method_offset = text.find("run int").expect("interface method");
        let Availability::Known(methods) = engine
            .implementations(&metadata.snapshot_id, uri, method_offset)
            .expect("method implementations")
        else {
            panic!("method implementations should be known");
        };
        assert_eq!(methods.len(), 2);
        let concrete_method_offset =
            text.find("item.run").expect("concrete method use") + "item.".len();
        let Availability::Known(concrete_methods) = engine
            .implementations(&metadata.snapshot_id, uri, concrete_method_offset)
            .expect("concrete method implementations")
        else {
            panic!("concrete method implementations should be known");
        };
        assert_eq!(concrete_methods, methods);
        let interface_rename = engine
            .propose_rename(&metadata.snapshot_id, uri, method_offset, "execute")
            .expect("interface contract rename");
        assert!(interface_rename.preview_diagnostics.is_empty());
        assert!(interface_rename.semantic_reanalysis);
        assert_eq!(interface_rename.replacements.len(), 4);
        let concrete_rename = engine
            .propose_rename(
                &metadata.snapshot_id,
                uri,
                concrete_method_offset,
                "execute",
            )
            .expect("concrete contract rename");
        assert_eq!(concrete_rename.replacements, interface_rename.replacements);
        let broken = engine
            .propose_edits(
                &metadata.snapshot_id,
                vec![Replacement {
                    uri: uri.to_owned(),
                    span: PublicSpan {
                        start: method_offset,
                        end: method_offset + "run".len(),
                    },
                    text: "broken".to_owned(),
                }],
            )
            .expect("diagnostic interface proposal");
        assert!(!broken.semantic_reanalysis);
        assert!(!broken.preview_diagnostics.is_empty());
        let error = engine
            .apply_edits(&broken.proposal_id)
            .expect_err("diagnostic interface proposal must not be applicable");
        assert_eq!(error.code, "invalid-proposal");
    }

    #[test]
    fn invalid_contract_rename_reports_the_first_preview_diagnostic() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/shared-method.trn";
        let text = "namespace shared-method\n\ninterface shape\n    function area int;\n\ninterface sized\n    function area int;\n\nclass both implements shape, sized\n    function area int;\n        return 1\n\nfunction main;\n    item = instance both;\n    result int = item.area;\n";
        let metadata = open(
            &mut engine,
            uri,
            text,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        let shape_method = text.find("function area").expect("shape method") + "function ".len();
        let error = engine
            .propose_rename(&metadata.snapshot_id, uri, shape_method, "surface")
            .expect_err("unrelated interface contract must make the rename invalid");
        assert_eq!(error.code, "invalid-rename");
        assert!(
            error.message.contains("first diagnostic"),
            "{}",
            error.message
        );
        assert!(error.message.contains("T0062"), "{}", error.message);
        assert!(error.message.contains("sized.area"), "{}", error.message);
    }

    #[test]
    fn semantic_objects_report_callable_and_descriptor_facts() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/facts.trn";
        let text = "namespace facts\n\nfrom /core/errors import coercion-error\nfrom /core/types import int as number\n\nclass base\n    value number = 1\n\nclass child extends base\n    async function compute number throws coercion-error;\n        throw coercion-error\n\nasync function main;\n    item = instance child;\n    descriptor = item.type\n    result number = await item.compute;\n";
        let metadata = open(
            &mut engine,
            uri,
            text,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        let compute_offset = text.rfind("compute").expect("method use");
        let object = engine
            .locate(&metadata.snapshot_id, uri, compute_offset)
            .expect("method semantic object")
            .expect("method node");
        assert_eq!(
            object.invocation_mode,
            Availability::Known("shared".to_owned())
        );
        let Availability::Known(effects) = object.effects else {
            panic!("function effects should be known");
        };
        assert_eq!(effects.len(), 2);
        assert_eq!(effects[0], "async");
        assert!(effects[1].starts_with("throws "));
        assert_eq!(object.ownership, Availability::Unsupported);
        assert_eq!(object.capabilities, Availability::Known(Vec::new()));
        assert!(matches!(object.value_type, Availability::Known(_)));

        let child_offset = text.find("child extends").expect("child declaration");
        let alias_offset = text.rfind("number").expect("aliased type use");
        let alias = engine
            .locate(&metadata.snapshot_id, uri, alias_offset)
            .expect("alias query")
            .expect("alias syntax object");
        assert_eq!(
            alias.symbol_identity,
            Availability::Known("/core/types::int".to_owned())
        );

        let descriptor = engine
            .locate(&metadata.snapshot_id, uri, child_offset)
            .expect("descriptor semantic object")
            .expect("descriptor node");
        let Availability::Known(inheritance) = descriptor.inheritance else {
            panic!("inheritance should be known");
        };
        assert!(
            inheritance
                .iter()
                .any(|identity| identity.ends_with("::base"))
        );
        let Availability::Known(members) = descriptor.members else {
            panic!("members should be known");
        };
        assert!(members.iter().any(|member| member.name == "compute"));
        assert!(members.iter().any(|member| member.name == "value"));
        let type_offset = text.find("item.type").expect("universal type use") + "item.".len();
        let type_member = engine
            .locate(&metadata.snapshot_id, uri, type_offset)
            .expect("type member query")
            .expect("type member object");
        assert_eq!(
            type_member.symbol_identity,
            Availability::Known("/core/types::value.type".to_owned())
        );
        assert_eq!(type_member.declaration, Availability::Unsupported);
        assert!(matches!(type_member.value_type, Availability::Known(_)));
    }

    #[test]
    fn snapshot_rejects_unsupported_targets_and_drifted_lock_inputs() {
        let mut engine = ToolingEngine::default();
        let source = SourceInput {
            uri: "file:///workspace/main.trn".to_owned(),
            text: "function main;\n".to_owned(),
        };
        let target_error = engine
            .open_snapshot(
                vec![source.clone()],
                None,
                None,
                SnapshotOptions {
                    target: "wasm32-unknown-unknown".to_owned(),
                    ..SnapshotOptions::default()
                },
            )
            .expect_err("unsupported target");
        assert_eq!(target_error.code, "unsupported-target");

        let lock_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../Cargo.lock")
            .canonicalize()
            .expect("workspace lock");
        let lock_error = engine
            .open_snapshot(
                vec![source],
                None,
                Some(SourceInput {
                    uri: format!("file://{}", lock_path.display()),
                    text: "drifted lock".to_owned(),
                }),
                SnapshotOptions::default(),
            )
            .expect_err("drifted lock");
        assert_eq!(lock_error.code, "lock-mismatch");
    }
    #[test]
    fn semantic_navigation_preserves_lexical_shadowing() {
        let mut engine = ToolingEngine::default();
        let uri = "file:///workspace/shadowing.trn";
        let text = "namespace shadowing\n\nfunction answer int;\n    return 1\n\nfunction main;\n    answer int = 2\n    local int = answer\n";
        let snapshot = open(
            &mut engine,
            uri,
            text,
            SnapshotOptions {
                semantic: true,
                ..SnapshotOptions::default()
            },
        );
        let use_offset = text.rfind("answer").expect("shadowed use");
        let declaration_offset = text.find("answer int = 2").expect("local declaration");
        let Availability::Known(definition) = engine
            .definition(&snapshot.snapshot_id, uri, use_offset)
            .expect("shadowed definition")
        else {
            panic!("local definition should be known");
        };
        assert_eq!(definition.span.start, declaration_offset);
        let Availability::Known(references) = engine
            .references(&snapshot.snapshot_id, uri, use_offset)
            .expect("shadowed references")
        else {
            panic!("local references should be known");
        };
        assert_eq!(references.len(), 2);
        let declaration = engine
            .locate(&snapshot.snapshot_id, uri, declaration_offset)
            .expect("local declaration query")
            .expect("local declaration object");
        assert!(matches!(
            &declaration.symbol_identity,
            Availability::Known(identity) if identity.contains("::scope")
        ));
        let Availability::Known(declaration_references) = engine
            .references(&snapshot.snapshot_id, uri, declaration_offset)
            .expect("references from declaration")
        else {
            panic!("declaration references should be known");
        };
        assert_eq!(declaration_references, references);
        let proposal = engine
            .propose_rename(&snapshot.snapshot_id, uri, declaration_offset, "computed")
            .expect("rename from declaration");
        assert_eq!(proposal.replacements.len(), 2);
    }

    #[test]
    fn package_rename_is_identity_safe_preflighted_and_comment_preserving() {
        let directory = TemporaryDirectory::new("package-rename");
        let root = directory.path();
        let child_directory = root.join("app/child");
        fs::create_dir_all(&child_directory).expect("create package sources");
        let manifest_path = root.join("package.toml");
        let main_path = root.join("app/main.trn");
        let child_path = child_directory.join("child.trn");
        let manifest = "package = \"tooling-rename\"\n[namespaces]\napp = \"app\"\n";
        let main = "namespace app\n\n# answer remains in this comment\npublic function answer int;\n    return 1\n\nfunction occupied int;\n    return 2\n";
        let child = "namespace app/child\n\nfunction use-answer int;\n    # answer remains here too\n    return answer;\n";
        fs::write(&manifest_path, manifest).expect("write manifest");
        fs::write(&main_path, main).expect("write main source");
        fs::write(&child_path, child).expect("write child source");
        let main_uri = format!("file://{}", main_path.display());
        let child_uri = format!("file://{}", child_path.display());
        let mut engine = ToolingEngine::default();
        let snapshot = engine
            .open_snapshot(
                vec![
                    SourceInput {
                        uri: main_uri.clone(),
                        text: main.to_owned(),
                    },
                    SourceInput {
                        uri: child_uri.clone(),
                        text: child.to_owned(),
                    },
                ],
                Some(SourceInput {
                    uri: format!("file://{}", manifest_path.display()),
                    text: manifest.to_owned(),
                }),
                None,
                SnapshotOptions {
                    semantic: true,
                    ..SnapshotOptions::default()
                },
            )
            .expect("package snapshot");
        let use_offset = child.rfind("answer").expect("answer use");
        let capture = engine
            .propose_rename(&snapshot.snapshot_id, &child_uri, use_offset, "occupied")
            .expect_err("same-namespace declaration captures rename");
        assert_eq!(capture.code, "rename-capture");

        let stale = engine
            .propose_rename(
                &snapshot.snapshot_id,
                &child_uri,
                use_offset,
                "computed-answer",
            )
            .expect("multi-file rename proposal");
        assert_eq!(stale.affected_files.len(), 2);
        fs::write(&child_path, format!("{child}\n")).expect("drift child source");
        let stale_error = engine
            .apply_edits(&stale.proposal_id)
            .expect_err("preflight rejects every file before committing");
        assert_eq!(stale_error.code, "stale-write");
        assert_eq!(fs::read_to_string(&main_path).unwrap(), main);

        fs::write(&child_path, child).expect("restore child source");
        let proposal = engine
            .propose_rename(
                &snapshot.snapshot_id,
                &child_uri,
                use_offset,
                "computed-answer",
            )
            .expect("fresh rename proposal");
        let report = engine
            .apply_edits(&proposal.proposal_id)
            .expect("apply rename");
        assert_eq!(report.committed.len(), 2);
        let updated_main = fs::read_to_string(&main_path).expect("updated main");
        let updated_child = fs::read_to_string(&child_path).expect("updated child");
        assert!(updated_main.contains("function computed-answer int"));
        assert!(updated_child.contains("return computed-answer;"));
        assert!(updated_main.contains("# answer remains in this comment"));
        assert!(updated_child.contains("# answer remains here too"));
    }

    #[test]
    fn protocol_rejects_unknown_fields_and_bounds_pending_cancellations() {
        let unknown = serde_json::json!({
            "schema_version": SCHEMA_VERSION,
            "request_id": "unknown",
            "operation": "syntax",
            "snapshot_id": "sha256:missing",
            "uri": "file:///workspace/main.trn",
            "node_id": null,
            "surprise": true
        });
        assert!(serde_json::from_value::<RequestEnvelope>(unknown).is_err());

        let mut engine = ToolingEngine::default();
        for index in 0..1_100 {
            let response = engine.handle(RequestEnvelope {
                schema_version: SCHEMA_VERSION.to_owned(),
                request_id: format!("cancel-command-{index}"),
                request: Request::Cancel {
                    request_id: format!("future-{index}"),
                },
            });
            assert!(response.error.is_none());
        }
        assert_eq!(engine.canceled.len(), 1_024);
        let canceled = engine.handle(RequestEnvelope {
            schema_version: SCHEMA_VERSION.to_owned(),
            request_id: "future-1099".to_owned(),
            request: Request::Find {
                snapshot_id: "sha256:missing".to_owned(),
                selector: Selector {
                    kind: None,
                    child_field: None,
                    containing: None,
                    text: None,
                    token_kind: None,
                    symbol_identity: None,
                    descriptor_identity: None,
                    include_recovery: false,
                },
                page_size: None,
                continuation: None,
            },
        });
        assert_eq!(
            canceled.error.as_ref().map(|error| error.code.as_str()),
            Some("canceled")
        );
        let expired = engine.handle(RequestEnvelope {
            schema_version: SCHEMA_VERSION.to_owned(),
            request_id: "future-0".to_owned(),
            request: Request::CloseSnapshot {
                snapshot_id: "sha256:missing".to_owned(),
            },
        });
        assert_eq!(
            expired.error.as_ref().map(|error| error.code.as_str()),
            Some("expired-snapshot")
        );
    }
    #[test]
    fn real_dependency_inputs_drive_snapshot_and_projection_invalidation() {
        let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/conformance/check/rust-dependency-reqwest")
            .canonicalize()
            .expect("dependency fixture");
        let directory = TemporaryDirectory::new("dependency-invalidation");
        let source_directory = directory.path().join("src");
        fs::create_dir_all(&source_directory).expect("temporary package source directory");
        let manifest_path = directory.path().join("package.toml");
        let source_path = source_directory.join("main.trn");
        fs::copy(fixture.join("package.toml"), &manifest_path).expect("copy manifest");
        fs::copy(fixture.join("src/main.trn"), &source_path).expect("copy source");
        fs::copy(
            fixture.join("terrane-projection.lock"),
            directory.path().join("terrane-projection.lock"),
        )
        .expect("copy projection lock");
        let manifest_text = fs::read_to_string(&manifest_path).expect("manifest");
        let source_text = fs::read_to_string(&source_path).expect("source");
        let manifest_uri = format!("file://{}", manifest_path.display());
        let source_uri = format!("file://{}", source_path.display());
        let mut engine = ToolingEngine::default();
        let open_dependency = |engine: &mut ToolingEngine, manifest: &str, source: &str| {
            engine
                .open_snapshot(
                    vec![SourceInput {
                        uri: source_uri.clone(),
                        text: source.to_owned(),
                    }],
                    Some(SourceInput {
                        uri: manifest_uri.clone(),
                        text: manifest.to_owned(),
                    }),
                    None,
                    SnapshotOptions {
                        semantic: true,
                        ..SnapshotOptions::default()
                    },
                )
                .expect("dependency snapshot")
        };
        let first = open_dependency(&mut engine, &manifest_text, &source_text);
        let Availability::Known(first_projection) = &first.dependency_projection else {
            panic!("real dependency projection should be known");
        };
        assert!(
            first_projection
                .dependencies
                .iter()
                .any(|dependency| dependency.starts_with("reqwest@"))
        );

        let edited_source = format!("{source_text}\n# editor overlay\n");
        let source_changed = open_dependency(&mut engine, &manifest_text, &edited_source);
        let Availability::Known(source_projection) = &source_changed.dependency_projection else {
            panic!("overlay dependency projection should be known");
        };
        assert_ne!(first.snapshot_id, source_changed.snapshot_id);
        assert_eq!(
            first_projection.cache_identity,
            source_projection.cache_identity
        );

        let changed_manifest = manifest_text.replace(
            "default-features = false",
            "default-features = false\neffects = [\"networking\"]",
        );
        let dependency_changed = open_dependency(&mut engine, &changed_manifest, &source_text);
        let Availability::Known(changed_projection) = &dependency_changed.dependency_projection
        else {
            panic!("changed dependency projection should be known");
        };
        assert_ne!(first.snapshot_id, dependency_changed.snapshot_id);
        assert_ne!(
            first_projection.cache_identity,
            changed_projection.cache_identity
        );
    }
}
