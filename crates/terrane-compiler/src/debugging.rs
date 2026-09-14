use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::rust_ir::RenderedFile;
use crate::semantics::{SemanticPackage, SemanticUnit, ValueType};
use crate::{Package, SourceFile, Span};

pub const SCHEMA_VERSION: &str = "1.2";
const DEBUG_MARKER: &str = "/* terrane-debug-point:";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DebugArtifactProfile {
    pub id: &'static str,
    pub optimization: &'static str,
    pub cargo_debug: &'static str,
    pub debug_information: &'static str,
    pub stripping: &'static str,
    pub inlining: &'static str,
}

pub const DEBUG_ARTIFACT_PROFILE: DebugArtifactProfile = DebugArtifactProfile {
    id: "terrane-debug-v1",
    optimization: "0",
    cargo_debug: "2",
    debug_information: "full",
    stripping: "none",
    inlining: "compiler-default-at-opt-level-0",
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProvenanceRole {
    User,
    Generated,
    Runtime,
    Cleanup,
    Hidden,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceSpan {
    pub source_id: u32,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GeneratedRange {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SourceIdentity {
    pub id: u32,
    pub uri: String,
    pub content_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_source: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugAssociation {
    pub generated: GeneratedRange,
    pub causes: Vec<SourceSpan>,
    pub role: ProvenanceRole,
    pub sequence_point: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    pub scope_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GeneratedFileIdentity {
    pub path: String,
    pub content_hash: String,
    pub associations: Vec<DebugAssociation>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugFunction {
    pub id: String,
    pub name: String,
    pub namespace: String,
    pub source: SourceSpan,
    pub rust_name: String,
    pub is_async: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugScope {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub source: SourceSpan,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugBinding {
    pub id: String,
    pub name: String,
    pub rust_name: String,
    pub source: SourceSpan,
    pub visible_from: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    pub visible_until: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    pub type_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    pub mutable: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugObjectField {
    pub name: String,
    pub rust_name: String,
    pub type_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    pub secret: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugObject {
    pub id: String,
    pub fields: Vec<DebugObjectField>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DebugInformation {
    pub schema_version: String,
    pub compiler_version: String,
    pub sources: Vec<SourceIdentity>,
    pub generated_files: Vec<GeneratedFileIdentity>,
    pub functions: Vec<DebugFunction>,
    pub scopes: Vec<DebugScope>,
    pub bindings: Vec<DebugBinding>,
    pub objects: Vec<DebugObject>,
}

#[derive(Clone, Debug)]
pub(crate) struct DebugSymbols {
    sources: Vec<SourceIdentity>,
    functions: Vec<DebugFunction>,
    scopes: Vec<DebugScope>,
    bindings: Vec<DebugBinding>,
    source_roles: BTreeMap<u32, crate::SourceRole>,
    objects: Vec<DebugObject>,
    source_files: BTreeMap<u32, SourceFile>,
}

impl DebugSymbols {
    pub(crate) fn from_semantic(semantic: &SemanticPackage, embed_sources: bool) -> Self {
        let mut sources = Vec::new();
        let mut functions = Vec::new();
        let mut scopes = Vec::new();
        let mut bindings = Vec::new();
        let mut objects = Vec::new();
        let mut source_roles = BTreeMap::new();
        let mut source_files = BTreeMap::new();
        for unit in &semantic.units {
            source_roles.insert(unit.source.id(), unit.role);
            source_files.insert(unit.source.id(), unit.source.clone());
            sources.push(SourceIdentity {
                id: unit.source.id(),
                uri: unit.source_path.clone(),
                content_hash: hash_bytes(unit.source.text().as_bytes()),
                embedded_source: embed_sources.then(|| unit.source.text().to_owned()),
            });
            append_unit_symbols(
                semantic,
                unit,
                &mut functions,
                &mut scopes,
                &mut bindings,
                &mut objects,
            );
        }
        Self {
            sources,
            functions,
            scopes,
            bindings,
            source_roles,
            objects,
            source_files,
        }
    }

    pub(crate) fn render(&self, files: &[RenderedFile]) -> DebugInformation {
        let generated_files = files
            .iter()
            .map(|file| {
                let mut associations = marker_associations(file, self);
                associations.extend(file.associations.iter().filter_map(|association| {
                    let source = self.source_files.get(&association.source.file)?;
                    let role = match self.source_roles.get(&association.source.file) {
                        Some(
                            crate::SourceRole::Production
                            | crate::SourceRole::UnitTest
                            | crate::SourceRole::IntegrationTest
                            | crate::SourceRole::EndToEndTest,
                        ) => ProvenanceRole::User,
                        Some(crate::SourceRole::Bundled) => ProvenanceRole::Runtime,
                        None => ProvenanceRole::Generated,
                    };
                    Some(DebugAssociation {
                        generated: generated_range(
                            &file.contents,
                            association.generated_start,
                            association.generated_end,
                        ),
                        causes: vec![source_span(source, association.source)],
                        role,
                        sequence_point: false,
                        function_id: None,
                        scope_ids: Vec::new(),
                    })
                }));
                GeneratedFileIdentity {
                    path: file.path.clone(),
                    content_hash: hash_bytes(file.contents.as_bytes()),
                    associations,
                }
            })
            .collect();
        DebugInformation {
            schema_version: SCHEMA_VERSION.to_owned(),
            compiler_version: crate::VERSION.to_owned(),
            sources: self.sources.clone(),
            generated_files,
            functions: self.functions.clone(),
            scopes: self.scopes.clone(),
            bindings: self.bindings.clone(),
            objects: self.objects.clone(),
        }
    }
}

fn append_unit_symbols(
    semantic: &SemanticPackage,
    unit: &SemanticUnit,
    functions: &mut Vec<DebugFunction>,
    scopes: &mut Vec<DebugScope>,
    bindings: &mut Vec<DebugBinding>,
    objects: &mut Vec<DebugObject>,
) {
    for descriptor in &unit.descriptors {
        objects.push(DebugObject {
            id: object_id(&descriptor.identity),
            fields: descriptor
                .fields
                .iter()
                .filter(|field| !field.is_static)
                .map(|field| DebugObjectField {
                    name: field.name.clone(),
                    rust_name: crate::lowering::debug_rust_name(&field.name),
                    type_name: format!("{:?}", field.value_type),
                    object_id: value_object_id(&field.value_type),
                    secret: field.metadata.secret,
                })
                .collect(),
        });
    }
    for function in &unit.functions {
        let id = function_id(
            unit,
            function.span,
            &function.name,
            function.owner.as_deref(),
        );
        functions.push(DebugFunction {
            id,
            name: function.owner.as_ref().map_or_else(
                || format!("{}::{}", unit.namespace, function.name),
                |owner| format!("{}::{owner}.{}", unit.namespace, function.name),
            ),
            namespace: unit.namespace.clone(),
            source: source_span(&unit.source, function.span),
            rust_name: crate::lowering::debug_function_name(semantic, function),
            is_async: function.is_async,
        });
    }
    for (index, scope) in unit.scopes.iter().enumerate() {
        let id = scope_id(scope.span);
        scopes.push(DebugScope {
            id: id.clone(),
            parent_id: scope
                .parent
                .map(|parent| scope_id(unit.scopes[parent].span)),
            source: source_span(&unit.source, scope.span),
            function_id: containing_function_id(unit, scope.span.start),
        });
        debug_assert_eq!(id, scope_id(unit.scopes[index].span));
    }
    for binding in &unit.typed_bindings {
        let scope_id = binding.scope.and_then(|span| {
            unit.scopes
                .iter()
                .filter(|scope| scope.span.start <= span.start && scope.span.end >= span.end)
                .min_by_key(|scope| scope.span.end - scope.span.start)
                .map(|scope| scope_id(scope.span))
        });
        bindings.push(DebugBinding {
            id: format!(
                "binding:{}:{}:{}",
                binding.span.file, binding.span.start, binding.span.end
            ),
            name: binding.name.clone(),
            rust_name: crate::lowering::debug_rust_name(&binding.name),
            source: source_span(&unit.source, binding.span),
            visible_from: binding.visible_from,
            scope_id,
            type_name: format!("{:?}", binding.value_type),
            object_id: value_object_id(&binding.value_type),
            visible_until: binding
                .scope
                .map_or(unit.source.text().len(), |scope| scope.end),
            function_id: containing_function_id(unit, binding.span.start),
            mutable: binding.mutable,
        });
    }
}

fn value_object_id(value_type: &ValueType) -> Option<String> {
    match value_type {
        ValueType::Object(identity) => Some(object_id(identity)),
        ValueType::Reference(element) | ValueType::SharedReference(element) => {
            value_object_id(&element.value_type())
        }
        _ => None,
    }
}

fn object_id(identity: &crate::semantics::ObjectIdentity) -> String {
    format!("{}::{}", identity.namespace, identity.name)
}

fn marker_associations(file: &RenderedFile, symbols: &DebugSymbols) -> Vec<DebugAssociation> {
    let mut associations = Vec::new();
    let mut offset = 0;
    let mut pending = Vec::new();
    for line in file.contents.split_inclusive('\n') {
        let trimmed = line.trim();
        if let Some(marker) = trimmed
            .strip_prefix(DEBUG_MARKER)
            .and_then(|value| value.strip_suffix(" */"))
        {
            if let Some((span, role)) = parse_marker(marker) {
                pending.clear();
                pending.push((span, role));
            }
        } else if !pending.is_empty() && !trimmed.is_empty() {
            let generated = generated_range(
                &file.contents,
                offset,
                offset + line.trim_end_matches('\n').len(),
            );
            for (span, role) in pending.drain(..) {
                let Some(source) = symbols.source_files.get(&span.file) else {
                    continue;
                };
                let function = symbols
                    .functions
                    .iter()
                    .filter(|function| {
                        function.source.source_id == span.file
                            && function.source.start <= span.start
                            && function.source.end >= span.end
                    })
                    .min_by_key(|function| function.source.end - function.source.start);
                let scope_ids = symbols
                    .scopes
                    .iter()
                    .filter(|scope| {
                        scope.source.source_id == span.file
                            && scope.source.start <= span.start
                            && scope.source.end >= span.end
                    })
                    .map(|scope| scope.id.clone())
                    .collect();
                associations.push(DebugAssociation {
                    generated: generated.clone(),
                    causes: vec![source_span(source, span)],
                    role,
                    sequence_point: true,
                    function_id: function.map(|function| function.id.clone()),
                    scope_ids,
                });
            }
        }
        offset += line.len();
    }
    associations
}

fn parse_marker(value: &str) -> Option<(Span, ProvenanceRole)> {
    let mut fields = value.split(':');
    let file = fields.next()?.parse().ok()?;
    let start = fields.next()?.parse().ok()?;
    let end = fields.next()?.parse().ok()?;
    let role = match fields.next()? {
        "user" => ProvenanceRole::User,
        "generated" => ProvenanceRole::Generated,
        "runtime" => ProvenanceRole::Runtime,
        "cleanup" => ProvenanceRole::Cleanup,
        "hidden" => ProvenanceRole::Hidden,
        _ => return None,
    };
    Some((Span::new(file, start, end), role))
}

fn containing_function_id(unit: &SemanticUnit, position: usize) -> Option<String> {
    unit.functions
        .iter()
        .filter(|function| function.span.start <= position && function.span.end >= position)
        .min_by_key(|function| function.span.end - function.span.start)
        .map(|function| {
            function_id(
                unit,
                function.span,
                &function.name,
                function.owner.as_deref(),
            )
        })
}

fn function_id(unit: &SemanticUnit, span: Span, name: &str, owner: Option<&str>) -> String {
    format!(
        "function:{}:{}:{}:{}:{}",
        unit.namespace,
        owner.unwrap_or(""),
        name,
        span.start,
        span.end
    )
}

fn scope_id(span: Span) -> String {
    format!("scope:{}:{}:{}", span.file, span.start, span.end)
}

fn source_span(source: &SourceFile, span: Span) -> SourceSpan {
    let (line, column) = source.line_column(span.start);
    let (end_line, end_column) = source.line_column(span.end);
    SourceSpan {
        source_id: span.file,
        start: span.start,
        end: span.end,
        line,
        column,
        end_line,
        end_column,
    }
}

fn generated_range(text: &str, start: usize, end: usize) -> GeneratedRange {
    let (line, column) = text_line_column(text, start);
    let (end_line, end_column) = text_line_column(text, end);
    GeneratedRange {
        start,
        end,
        line,
        column,
        end_line,
        end_column,
    }
}

fn text_line_column(text: &str, offset: usize) -> (usize, usize) {
    let prefix = &text[..offset];
    let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let column = prefix
        .rsplit_once('\n')
        .map_or(prefix, |(_, tail)| tail)
        .chars()
        .count()
        + 1;
    (line, column)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InputIdentity {
    pub path: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NativeModuleIdentity {
    pub file_name: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RelocationMapping {
    pub build_root: String,
    pub source_root: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProvenanceManifest {
    pub schema_version: String,
    pub compiler_version: String,
    pub rust_toolchain: String,
    pub target: String,
    pub rust_sysroot: String,
    pub rustc_release: String,
    pub abi_recipe: String,
    pub artifact_profile: String,
    pub optimization: String,
    pub debug_information: String,
    pub inlining: String,
    pub stripping: String,
    pub inputs: Vec<InputIdentity>,
    pub debug: DebugInformation,
    pub native_module: NativeModuleIdentity,
    pub relocation: RelocationMapping,
}

impl ProvenanceManifest {
    /// Binds compiler metadata to one native module and its build/source relocation roots.
    ///
    /// # Errors
    ///
    /// Returns an error when the executable cannot be read for identity hashing.
    pub fn create(
        package: &Package,
        debug: DebugInformation,
        executable: &Path,
        build_root: &Path,
        target: String,
        rust_sysroot: String,
        rustc_release: String,
        artifact_profile: DebugArtifactProfile,
    ) -> Result<Self, String> {
        let build_root = std::fs::canonicalize(build_root).map_err(|error| {
            format!(
                "cannot canonicalize debug build root {}: {error}",
                build_root.display()
            )
        })?;
        let source_root = std::fs::canonicalize(&package.root).map_err(|error| {
            format!(
                "cannot canonicalize debug source root {}: {error}",
                package.root.display()
            )
        })?;
        let executable_bytes = std::fs::read(executable).map_err(|error| {
            format!(
                "cannot read debug executable {}: {error}",
                executable.display()
            )
        })?;
        let mut inputs = Vec::new();
        for name in [crate::MANIFEST_FILE_NAME, "terrane-projection.lock"] {
            let path = package.root.join(name);
            if let Ok(bytes) = std::fs::read(&path) {
                inputs.push(InputIdentity {
                    path: name.to_owned(),
                    content_hash: hash_bytes(&bytes),
                });
            }
        }
        Ok(Self {
            schema_version: SCHEMA_VERSION.to_owned(),
            compiler_version: crate::VERSION.to_owned(),
            rust_toolchain: match package.build_toolchain {
                crate::BuildToolchain::Pinned => crate::BUILD_TOOLCHAIN.to_owned(),
                crate::BuildToolchain::System => "system".to_owned(),
            },
            abi_recipe: abi_recipe_for_toolchain(&target, &rustc_release),
            target,
            rust_sysroot,
            rustc_release,
            artifact_profile: artifact_profile.id.to_owned(),
            optimization: artifact_profile.optimization.to_owned(),
            debug_information: artifact_profile.debug_information.to_owned(),
            inlining: artifact_profile.inlining.to_owned(),
            stripping: artifact_profile.stripping.to_owned(),
            inputs,
            debug,
            native_module: NativeModuleIdentity {
                file_name: executable
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                content_hash: hash_bytes(&executable_bytes),
            },
            relocation: RelocationMapping {
                build_root: build_root.to_string_lossy().into_owned(),
                source_root: source_root.to_string_lossy().into_owned(),
            },
        })
    }

    /// Confirms that an executable is the exact module named by this provenance.
    ///
    /// # Errors
    ///
    /// Returns an error when the executable cannot be read or its identity differs.
    pub fn validate_executable(&self, executable: &Path) -> Result<(), String> {
        let bytes = std::fs::read(executable)
            .map_err(|error| format!("cannot read executable {}: {error}", executable.display()))?;
        let actual = hash_bytes(&bytes);
        if actual != self.native_module.content_hash {
            return Err(format!(
                "debug provenance does not match executable {} (expected {}, found {})",
                executable.display(),
                self.native_module.content_hash,
                actual
            ));
        }
        Ok(())
    }

    /// Returns logical source paths that are missing or differ from their build-time identity.
    #[must_use]
    pub fn validate_sources(&self, root: &Path) -> Vec<String> {
        self.debug
            .sources
            .iter()
            .filter_map(|source| {
                let path = root.join(PathBuf::from(&source.uri));
                let actual = std::fs::read(&path).ok().map(|bytes| hash_bytes(&bytes));
                (actual.as_deref() != Some(source.content_hash.as_str()))
                    .then(|| source.uri.clone())
            })
            .collect()
    }
}

#[must_use]
pub fn abi_recipe_for_toolchain(target: &str, rustc_release: &str) -> String {
    match target {
        "x86_64-unknown-linux-gnu" => {
            format!(
                "terrane-rust-x86_64-linux-gnu-v2@{}",
                hash_bytes(rustc_release.as_bytes())
            )
        }
        _ => "unsupported".to_owned(),
    }
}

#[must_use]
pub fn hash_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{CompilerOptions, compile_with_options};

    #[test]
    fn debug_information_tracks_final_sequence_points_and_bindings() {
        let source = "namespace debug\nfunction main;\n  value = 1\n  value = value + 1\n";
        let compilation = compile_with_options(
            "debug.trn",
            source.to_owned(),
            CompilerOptions {
                debug_build: crate::DebugBuild::ExternalSources,
                ..CompilerOptions::default()
            },
        )
        .unwrap();
        let debug = compilation
            .debug_information(Path::new("src/main.rs"))
            .unwrap()
            .unwrap();

        assert_eq!(debug.schema_version, super::SCHEMA_VERSION);
        assert!(debug.sources.iter().any(|source| source.uri == "debug.trn"));
        assert!(
            debug
                .sources
                .iter()
                .all(|source| source.embedded_source.is_none())
        );
        assert!(debug.generated_files.iter().any(|file| {
            file.path == "src/main.rs"
                && file
                    .associations
                    .iter()
                    .filter(|association| {
                        association.sequence_point
                            && association.role == super::ProvenanceRole::User
                    })
                    .count()
                    >= 2
        }));
        assert!(
            debug
                .bindings
                .iter()
                .any(|binding| binding.name == "value" && binding.rust_name == "value")
        );
        let embedded = compile_with_options(
            "embedded.trn",
            source.to_owned(),
            CompilerOptions {
                debug_build: crate::DebugBuild::EmbeddedSources,
                ..CompilerOptions::default()
            },
        )
        .unwrap()
        .debug_information(Path::new("src/main.rs"))
        .unwrap()
        .unwrap();
        assert_eq!(embedded.sources[0].embedded_source.as_deref(), Some(source));
    }

    #[test]
    fn sequence_markers_do_not_drift_from_non_emitting_statements() {
        let compilation = compile_with_options(
            "markers.trn",
            concat!(
                "namespace markers\n",
                "global = 1\n",
                "function main;\n",
                "  local = 2\n",
            )
            .to_owned(),
            CompilerOptions {
                debug_build: crate::DebugBuild::ExternalSources,
                ..CompilerOptions::default()
            },
        )
        .unwrap();
        let debug = compilation
            .debug_information(Path::new("src/main.rs"))
            .unwrap()
            .unwrap();
        assert!(debug.generated_files.iter().all(|file| {
            file.associations.iter().all(|association| {
                !association.sequence_point
                    || association.causes.iter().all(|cause| cause.line != 2)
            })
        }));
    }

    #[test]
    fn ordinary_compilation_does_not_emit_debug_markers() {
        let compilation = crate::compile(
            "ordinary.trn",
            "namespace ordinary\nfunction main;\n  value = 1\n".to_owned(),
        )
        .unwrap();
        assert!(!compilation.rust.contains("terrane-debug-point"));
        assert!(
            compilation
                .debug_information(Path::new("src/main.rs"))
                .unwrap()
                .is_none()
        );
    }
    #[test]
    fn debug_information_carries_secret_field_policy() {
        let compilation = compile_with_options(
            "secret.trn",
            concat!(
                "namespace secret\n",
                "class credentials\n",
                "  username string = ''\n",
                "  token string = '' metadata (secret = true)\n",
                "function main;\n",
            )
            .to_owned(),
            CompilerOptions {
                debug_build: crate::DebugBuild::ExternalSources,
                ..CompilerOptions::default()
            },
        )
        .unwrap();
        let debug = compilation
            .debug_information(Path::new("src/main.rs"))
            .unwrap()
            .unwrap();
        let object = debug
            .objects
            .iter()
            .find(|object| object.id == "/secret::credentials")
            .unwrap();

        assert!(
            object
                .fields
                .iter()
                .any(|field| field.name == "token" && field.secret)
        );
        assert!(
            object
                .fields
                .iter()
                .any(|field| field.name == "username" && !field.secret)
        );
    }
}
