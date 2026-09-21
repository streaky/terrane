use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;
use std::time::Duration;

use rustdoc_types::{
    AssocItemConstraintKind, Attribute, Crate as RustdocCrate, Function, GenericArg, GenericArgs,
    GenericBound, GenericParamDef, GenericParamDefKind, Id, Impl, Item, ItemEnum, ItemSummary,
    Path as RustdocPath, Struct, Term, Type, VariantKind, Visibility, WherePredicate,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{InvocationMode, RustDependency};

pub use crate::RUSTDOC_TOOLCHAIN;
const PROJECTION_SCHEMA: &str = "63";
pub type ProjectedMemberDemands = BTreeMap<(String, String), BTreeSet<String>>;
const MAX_PROJECTION_CACHE_RECORDS: usize = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Projection {
    pub cache_identity: String,
    #[serde(default)]
    pub content_hash: String,
    pub dependencies: Vec<ProjectedDependency>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bound_dependencies: Vec<ProjectedBoundDependency>,
    pub containment: Containment,
    #[serde(default)]
    pub source: ProjectionSource,
    #[serde(default)]
    pub probes: Vec<crate::ProbeEvidence>,
    #[serde(default)]
    pub probe_wall_time_ms: u128,
    #[serde(default)]
    pub resolution: ProjectionResolution,
    #[serde(default)]
    pub removed: Vec<RemovedItem>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedBoundDependency {
    pub name: String,
    pub package: String,
    pub version: String,
}

pub use terrane_rust_analysis::Containment;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectionSource {
    Remote,
    #[default]
    Local,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum ResolutionOutcome {
    NoDependencies,
    ExactCache,
    PublishedArtifact,
    #[default]
    LocalRustdoc,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ResolutionSource {
    ExactCache,
    BundledArtifact,
    PublishedArtifact,
    LocalRustdoc,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ResolutionStatus {
    Hit,
    Miss,
    Rejected,
    Skipped,
    Generated,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResolutionEvent {
    pub source: ResolutionSource,
    pub status: ResolutionStatus,
    pub reason: String,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectionResolution {
    pub outcome: ResolutionOutcome,
    pub events: Vec<ResolutionEvent>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProjectionArtifact {
    format: u32,
    cache_identity: String,
    content_hash: String,
    target: String,
    build_toolchain: String,
    rustdoc_toolchain: String,
    rustdoc_format: u32,
    projection_schema: String,
    dependencies: Vec<ArtifactDependency>,
    projection: Projection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ArtifactDependency {
    name: String,
    package: String,
    version: String,
    features: Vec<String>,
    default_features: bool,
    target: Option<String>,
    effects: Vec<String>,
}

impl From<&RustDependency> for ArtifactDependency {
    fn from(dependency: &RustDependency) -> Self {
        Self {
            name: dependency.name.clone(),
            package: dependency.package.clone(),
            version: dependency.version.clone(),
            features: dependency.features.clone(),
            default_features: dependency.default_features,
            target: dependency.target.clone(),
            effects: dependency.effects.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct NamespaceOverlayMetadata {
    module: String,
    target_package: String,
    feature: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NamespaceOverlay {
    provider_name: String,
    provider_package: String,
    source_namespace: String,
    target_namespace: String,
}

enum PublishedProjection {
    Hit(Projection),
    Event(ResolutionEvent),
}
#[derive(Clone, Copy)]
enum CargoToolchain {
    Default,
    RustdocNightly,
}

#[derive(Clone, Copy)]
enum CargoExecution {
    Host,
    Contained,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedDependency {
    pub name: String,
    pub package: String,
    pub version: String,
    pub items: Vec<ProjectedItem>,
    pub declined: Vec<DeclinedItem>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RemovedItem {
    pub namespace: String,
    pub name: String,
    pub previous_version: String,
    pub current_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProjectionHistory {
    #[serde(default = "projection_history_format")]
    format: u32,
    dependencies: Vec<ProjectionHistoryDependency>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    bound_dependencies: Vec<ProjectedBoundDependency>,
    #[serde(default)]
    removed: Vec<RemovedItem>,
    #[serde(default)]
    cache_identity: Option<String>,
    #[serde(default)]
    source: Option<ProjectionSource>,
    #[serde(default)]
    rustdoc_format: Option<u32>,
    #[serde(default)]
    projection_schema: Option<String>,
    #[serde(default)]
    content_hash: Option<String>,
    #[serde(default)]
    resolution: Option<ProjectionResolution>,
}

fn projection_history_format() -> u32 {
    1
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProjectionHistoryDependency {
    name: String,
    version: String,
    members: BTreeSet<(String, String)>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedItem {
    pub namespace: String,
    pub name: String,
    pub rust_path: String,
    pub docs: Option<String>,
    pub kind: ProjectedKind,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedKind {
    Function(ProjectedFunction),
    ForeignType {
        methods: Vec<ProjectedFunction>,
        #[serde(default)]
        static_methods: Vec<ProjectedFunction>,
        #[serde(default)]
        boundary: ProjectedBoundaryCapabilities,
        #[serde(default)]
        displayable: bool,
        #[serde(default)]
        cloneable: bool,
        #[serde(default)]
        send: bool,
        #[serde(default)]
        sync: bool,
    },
    Interface(ProjectedInterface),
    Enum {
        #[serde(default)]
        methods: Vec<ProjectedFunction>,
        #[serde(default)]
        static_methods: Vec<ProjectedFunction>,
        #[serde(default)]
        displayable: bool,
        #[serde(default)]
        send: bool,
        #[serde(default)]
        sync: bool,
        data_carrying: bool,
        comparable: bool,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedAssociatedType {
    pub name: String,
    pub rust_path: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bounds: Vec<String>,
    pub docs: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedSupertrait {
    pub namespace: String,
    pub name: String,
    pub rust_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedInterface {
    pub methods: Vec<ProjectedInterfaceMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated_type: Option<ProjectedAssociatedType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supertraits: Vec<ProjectedSupertrait>,
    pub send: bool,
    pub sync: bool,
    #[serde(default)]
    pub requires_drop: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declined_methods: Vec<DeclinedItem>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedInterfaceMethod {
    pub function: ProjectedFunction,
    pub provided: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_rust_path: Option<String>,
    pub docs: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChainRole {
    Root,
    Continue,
    Terminal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedEnumOperation {
    Construct {
        variant: String,
        unit: bool,
        conversion: ProjectedEnumPayloadConversion,
        payload_rust_type: String,
    },
    VariantName {
        variants: Vec<String>,
        exhaustive: bool,
    },
    Extract {
        variant: String,
        conversion: ProjectedEnumPayloadConversion,
        payload_rust_type: String,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedEnumPayloadConversion {
    Identity,
    Into,
    AsRefString,
    AsRefBytes,
    DerefString,
    DerefBytes,
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedBoundaryCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    string_constructor: Option<ProjectedEnumPayloadConversion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bytes_constructor: Option<ProjectedEnumPayloadConversion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    string_extraction: Option<ProjectedEnumPayloadConversion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    bytes_extraction: Option<ProjectedEnumPayloadConversion>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedFunction {
    pub name: String,
    pub parameters: Vec<ProjectedParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub generic_parameters: Vec<ProjectedGenericParameter>,
    pub result: ProjectedType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_result: Option<ProjectedDestinationResult>,
    pub error: Option<String>,
    pub is_async: bool,
    #[serde(default)]
    pub into_future: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_requirements: Option<ProjectedExecutionRequirements>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_operation: Option<ProjectedEnumOperation>,
    #[serde(default)]
    pub error_optional_depth: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chain_role: Option<ChainRole>,
    pub receiver: Option<Receiver>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedGenericParameter {
    #[serde(default)]
    pub input_selected: bool,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rust_bounds: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedDestinationResult {
    pub parameter: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bound_roots: Vec<String>,
    pub rust_bounds: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedExecutionRequirements {
    pub runtime_context: RequirementKnowledge,
    pub wake_support: RequirementKnowledge,
    pub transfer: RequirementKnowledge,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RequirementKnowledge {
    Required,
    NotRequired,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedParameter {
    pub name: String,
    pub ty: ProjectedType,
    pub borrowed: bool,
    pub mutable_borrow: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generic_parameter: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub generic_bounds: Vec<String>,
    #[serde(skip)]
    pub generic_interface: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated_type: Option<ProjectedAssociatedBinding>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Receiver {
    Borrow,
    MutableBorrow,
    Move,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedAssociatedBinding {
    pub name: String,
    pub ty: Box<ProjectedType>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedType {
    None,
    Associated(String),
    Generic(String),
    Bool,
    Int,
    FixedInt(String),
    RustInt(String),
    Float,
    Float32,
    Char,
    String,
    Bytes,
    Sequence {
        rust_path: String,
        item: Box<ProjectedType>,
    },
    Mapping {
        rust_path: String,
        key: Box<ProjectedType>,
        value: Box<ProjectedType>,
        ordered: bool,
    },
    Set {
        rust_path: String,
        item: Box<ProjectedType>,
        ordered: bool,
    },
    Tuple(Vec<ProjectedType>),
    AsyncIterationStep(Box<ProjectedType>),
    AsyncSinkOutcome,
    Foreign {
        rust_path: String,
        name: String,
        #[serde(default)]
        base_rust_path: String,
        #[serde(default)]
        arguments: Vec<ProjectedType>,
    },
    BoxedInterface {
        rust_path: String,
        trait_path: String,
        name: String,
        #[serde(default)]
        auto_traits: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        associated_type: Option<ProjectedAssociatedBinding>,
    },
    Callback {
        rust_name: String,
        parameters: Vec<ProjectedType>,
        result: Box<ProjectedType>,
        invocation_mode: InvocationMode,
        is_async: bool,
        retained: bool,
        send: bool,
        sync: bool,
    },
    Optional(Box<ProjectedType>),
}

impl ProjectedType {
    fn is_terrane_scalar(&self) -> bool {
        matches!(
            self,
            Self::None
                | Self::Generic(_)
                | Self::Bool
                | Self::Int
                | Self::FixedInt(_)
                | Self::RustInt(_)
                | Self::Float
                | Self::Float32
                | Self::Char
                | Self::String
                | Self::Bytes
        )
    }

    pub(crate) fn rust_type(&self) -> String {
        match self {
            Self::None => "()".to_owned(),
            Self::Bool | Self::AsyncSinkOutcome => "bool".to_owned(),
            Self::Int => "i64".to_owned(),
            Self::Generic(name)
            | Self::Associated(name)
            | Self::FixedInt(name)
            | Self::RustInt(name) => name.clone(),
            Self::Float => "f64".to_owned(),
            Self::Float32 => "f32".to_owned(),
            Self::Char => "char".to_owned(),
            Self::String => "String".to_owned(),
            Self::Bytes => "Vec<u8>".to_owned(),
            Self::Sequence { rust_path, .. }
            | Self::Mapping { rust_path, .. }
            | Self::Set { rust_path, .. }
            | Self::Foreign { rust_path, .. }
            | Self::BoxedInterface { rust_path, .. } => rust_path.clone(),
            Self::AsyncIterationStep(item) => {
                format!("Option<{}>", item.rust_type())
            }
            Self::Callback { rust_name, .. } => rust_name.clone(),
            Self::Tuple(items) => format!(
                "({},)",
                items
                    .iter()
                    .map(Self::rust_type)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::Optional(inner) => format!("Option<{}>", inner.rust_type()),
        }
    }
    pub(crate) fn bind_associated(&mut self, replacement: &Self) {
        match self {
            Self::Associated(_) => *self = replacement.clone(),
            Self::Sequence { item, .. }
            | Self::Set { item, .. }
            | Self::AsyncIterationStep(item)
            | Self::Optional(item) => item.bind_associated(replacement),
            Self::Mapping { key, value, .. } => {
                key.bind_associated(replacement);
                value.bind_associated(replacement);
            }
            Self::Tuple(items) => {
                for item in items {
                    item.bind_associated(replacement);
                }
            }
            Self::Foreign { arguments, .. } => {
                for argument in arguments {
                    argument.bind_associated(replacement);
                }
            }
            Self::BoxedInterface {
                associated_type: Some(associated),
                ..
            } => associated.ty.bind_associated(replacement),
            _ => {}
        }
    }
}

impl ProjectedType {
    #[must_use]
    pub fn terrane_name(&self) -> String {
        match self {
            Self::Associated(_) => "host-projected-associated".to_owned(),
            Self::Generic(name) => format!("host-projected-generic-{name}"),
            Self::None => "none".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Int | Self::RustInt(_) => "int".to_owned(),
            Self::FixedInt(name) => match name.as_str() {
                "i8" => "int8",
                "i16" => "int16",
                "i32" => "int32",
                "i64" => "int64",
                "i128" => "int128",
                "u8" => "uint8",
                "u16" => "uint16",
                "u32" => "uint32",
                "u64" => "uint64",
                "u128" => "uint128",
                _ => name,
            }
            .to_owned(),
            Self::Float => "float64".to_owned(),
            Self::Float32 => "float32".to_owned(),
            Self::Char | Self::String => "string".to_owned(),
            Self::Bytes => "bytes".to_owned(),
            Self::BoxedInterface { name, .. } | Self::Foreign { name, .. } => name.clone(),
            Self::Sequence { item, .. } => format!("list of {}", item.terrane_name()),
            Self::Mapping {
                key,
                value,
                ordered,
                ..
            } => format!(
                "{}map of {}, {}",
                if *ordered { "" } else { "unordered-" },
                key.terrane_name(),
                value.terrane_name()
            ),
            Self::Set { item, ordered, .. } => format!(
                "{}set of {}",
                if *ordered { "" } else { "unordered-" },
                item.terrane_name()
            ),
            Self::Tuple(items) if items.windows(2).all(|pair| pair[0] == pair[1]) => {
                format!("tuple of {}", items[0].terrane_name())
            }
            Self::Tuple(_) => "heterogeneous tuple".to_owned(),
            Self::Callback {
                parameters,
                result,
                is_async,
                ..
            } => {
                let mut name = if *is_async {
                    "async function".to_owned()
                } else {
                    "function".to_owned()
                };
                if !parameters.is_empty() {
                    write!(
                        name,
                        " from {}",
                        parameters
                            .iter()
                            .map(Self::terrane_name)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                    .expect("writing to a string cannot fail");
                }
                write!(name, " to {}", result.terrane_name())
                    .expect("writing to a string cannot fail");
                name
            }
            Self::AsyncIterationStep(inner) => {
                format!("async-iteration-step of {}", inner.terrane_name())
            }
            Self::AsyncSinkOutcome => "async-sink-outcome".to_owned(),
            Self::Optional(inner) => format!("{}|none", inner.terrane_name()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeclinedItem {
    pub rust_path: String,
    pub reason: String,
}

fn collect_nested_projected_types(
    ty: &ProjectedType,
    name: &str,
    candidates: &mut Vec<ProjectedType>,
) {
    if matches!(
        ty,
        ProjectedType::Foreign {
            name: candidate,
            ..
        } | ProjectedType::BoxedInterface {
            name: candidate,
            ..
        } if candidate == name
    ) {
        candidates.push(ty.clone());
    }
    match ty {
        ProjectedType::Sequence { item, .. }
        | ProjectedType::Set { item, .. }
        | ProjectedType::AsyncIterationStep(item)
        | ProjectedType::Optional(item) => collect_nested_projected_types(item, name, candidates),
        ProjectedType::Mapping { key, value, .. } => {
            collect_nested_projected_types(key, name, candidates);
            collect_nested_projected_types(value, name, candidates);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                collect_nested_projected_types(item, name, candidates);
            }
        }
        ProjectedType::Foreign { arguments, .. } => {
            for item in arguments {
                collect_nested_projected_types(item, name, candidates);
            }
        }
        ProjectedType::BoxedInterface {
            associated_type: Some(associated),
            ..
        } => collect_nested_projected_types(&associated.ty, name, candidates),
        ProjectedType::Callback {
            parameters, result, ..
        } => {
            for parameter in parameters {
                collect_nested_projected_types(parameter, name, candidates);
            }
            collect_nested_projected_types(result, name, candidates);
        }
        _ => {}
    }
}

fn collect_function_projected_types(
    function: &ProjectedFunction,
    name: &str,
    candidates: &mut Vec<ProjectedType>,
) {
    for parameter in &function.parameters {
        collect_nested_projected_types(&parameter.ty, name, candidates);
    }
    collect_nested_projected_types(&function.result, name, candidates);
}

fn projected_type_owner(ty: &ProjectedType) -> Option<&str> {
    let path = match ty {
        ProjectedType::Foreign { base_rust_path, .. } => base_rust_path,
        ProjectedType::BoxedInterface { trait_path, .. } => trait_path,
        _ => return None,
    };
    Some(
        path.split_once('<')
            .map_or(path, |(constructor, _)| constructor),
    )
}

fn projected_owner_path_matches(candidate: &str, owner: &str) -> bool {
    candidate == owner
        || candidate
            .strip_prefix(owner)
            .is_some_and(|suffix| suffix.starts_with('<'))
}

impl Projection {
    /// Renders the projected dependency sources required by `imports`, limiting foreign member
    /// declarations and their type graph to member names that occur in the importing package.
    ///
    /// # Errors
    ///
    /// Returns an error when an imported projected name is ambiguous or demanded projected sources
    /// form a cycle.
    pub fn source_for_imports_with_members(
        &self,
        imports: &BTreeMap<String, BTreeSet<String>>,
        demanded_members: &ProjectedMemberDemands,
    ) -> Result<Vec<(String, String)>, String> {
        let all_items = self
            .dependencies
            .iter()
            .flat_map(|dependency| dependency.items.iter())
            .collect::<Vec<_>>();
        let imports = expanded_source_imports(&all_items, imports, demanded_members);
        let mut sources = Vec::new();
        for (namespace, names) in &imports {
            for name in names {
                if let Some(details) = self.item_ambiguity(namespace, name) {
                    return Err(format!(
                        "projected import `{namespace}::{name}` is ambiguous: {details}"
                    ));
                }
            }
            let selected = all_items
                .iter()
                .copied()
                .filter(|item| item.namespace == *namespace && names.contains(&item.name))
                .collect::<Vec<_>>();
            if selected.is_empty() {
                continue;
            }
            let foreign = collect_source_foreign(&all_items, &selected, demanded_members);
            let mut aliases = foreign_aliases(&foreign);
            let distinct_aliases = aliases.clone();
            for (rust_path, alias) in &mut aliases {
                let Some(item) =
                    projected_item_for_foreign(&all_items, rust_path, &foreign[rust_path])
                else {
                    continue;
                };
                if item.namespace == *namespace {
                    alias.clone_from(&foreign[rust_path]);
                } else if let Some(distinct_alias) = distinct_aliases.get(&item.rust_path) {
                    alias.clone_from(distinct_alias);
                }
            }
            let mut ordered_foreign = foreign.iter().collect::<Vec<_>>();
            ordered_foreign.sort_by_key(|(rust_path, name)| {
                let projected_item = projected_item_for_foreign(&all_items, rust_path, name);
                let cross_namespace =
                    projected_item.is_some_and(|item| item.namespace != *namespace);
                let dependency_count = all_items
                    .iter()
                    .copied()
                    .find(|item| item.rust_path == **rust_path)
                    .and_then(|item| match &item.kind {
                        ProjectedKind::ForeignType {
                            methods,
                            static_methods,
                            ..
                        } => Some(
                            methods
                                .iter()
                                .chain(static_methods)
                                .map(|method| foreign_function_dependency_count(method, name))
                                .sum::<usize>(),
                        ),
                        _ => None,
                    })
                    .unwrap_or_default();
                (!cross_namespace, dependency_count, aliases.get(*rust_path))
            });
            let source_dependencies = ordered_foreign
                .iter()
                .filter_map(|(rust_path, name)| {
                    projected_item_for_foreign(&all_items, rust_path, name)
                        .filter(|item| item.namespace != *namespace)
                        .map(|item| item.namespace.clone())
                })
                .collect::<BTreeSet<_>>();
            let mut text = format!("namespace {}\n\n", namespace.trim_start_matches('/'));
            let mut rendered_foreign = BTreeSet::new();
            for (rust_path, _) in ordered_foreign {
                let name = &aliases[rust_path];
                let projected_item =
                    projected_item_for_foreign(&all_items, rust_path, &foreign[rust_path]);
                let declaration_path =
                    projected_item.map_or(rust_path.as_str(), |item| item.rust_path.as_str());
                if !rendered_foreign.insert(declaration_path) {
                    continue;
                }
                render_foreign_declaration(
                    &mut text,
                    namespace,
                    name,
                    projected_item,
                    &aliases,
                    demanded_members,
                );
            }
            for item in selected {
                if let ProjectedKind::Function(function) = &item.kind {
                    render_function(&mut text, function, true, 0, &aliases, None);
                }
            }
            sources.push((namespace.clone(), text, source_dependencies));
        }
        Self::order_projected_sources(sources)
    }

    /// Renders complete projected dependency sources for callers without importing-package syntax.
    ///
    /// # Errors
    ///
    /// Returns an error when an imported projected name is ambiguous or projected sources form a
    /// cycle.
    pub fn source_for_imports(
        &self,
        imports: &BTreeMap<String, BTreeSet<String>>,
    ) -> Result<Vec<(String, String)>, String> {
        let demanded_members = self
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter_map(|item| match &item.kind {
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => Some((
                    (item.namespace.clone(), item.name.clone()),
                    methods
                        .iter()
                        .chain(static_methods)
                        .map(|function| function.name.clone())
                        .collect(),
                )),
                _ => None,
            })
            .collect();
        self.source_for_imports_with_members(imports, &demanded_members)
    }

    fn order_projected_sources(
        mut sources: Vec<(String, String, BTreeSet<String>)>,
    ) -> Result<Vec<(String, String)>, String> {
        let mut ordered = Vec::with_capacity(sources.len());
        while !sources.is_empty() {
            let remaining_names = sources
                .iter()
                .map(|(namespace, _, _)| namespace)
                .collect::<BTreeSet<_>>();
            let Some(index) = sources.iter().position(|(_, _, dependencies)| {
                dependencies
                    .iter()
                    .all(|dependency| !remaining_names.contains(dependency))
            }) else {
                let cycle = sources
                    .iter()
                    .map(|(namespace, _, dependencies)| {
                        let unresolved = dependencies
                            .iter()
                            .filter(|dependency| remaining_names.contains(dependency))
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{namespace} -> [{unresolved}]")
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                return Err(format!(
                    "projected dependency source namespaces contain an import cycle: {cycle}; \
                     this is the recorded `projection/mutually-referential-namespace-sources` \
                     limitation"
                ));
            };
            let (namespace, text, _) = sources.remove(index);
            ordered.push((namespace, text));
        }
        Ok(ordered)
    }

    #[must_use]
    pub fn foreign_imports(&self, namespace: &str) -> BTreeMap<String, String> {
        let all_items = self
            .dependencies
            .iter()
            .flat_map(|dependency| dependency.items.iter())
            .collect::<Vec<_>>();
        let mut foreign = BTreeMap::new();
        for item in all_items
            .iter()
            .copied()
            .filter(|item| item.namespace == namespace)
        {
            if let ProjectedKind::Function(function) = &item.kind {
                collect_foreign_function(function, &mut foreign);
            }
        }
        foreign.retain(|rust_path, name| {
            projected_item_for_foreign(&all_items, rust_path, name).is_none()
        });
        foreign_aliases(&foreign)
            .into_iter()
            .map(|(rust_path, name)| (name, rust_path))
            .collect()
    }

    /// Returns the uniquely named projected item in `namespace`.
    ///
    /// A duplicate name is ambiguous even when dependency iteration would otherwise provide a
    /// stable first match, so both missing and ambiguous lookups return `None`.
    #[must_use]
    pub fn item(&self, namespace: &str, name: &str) -> Option<&ProjectedItem> {
        let mut matching = self
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| item.namespace == namespace && item.name == name);
        let item = matching.next()?;
        matching.next().is_none().then_some(item)
    }

    #[must_use]
    pub(crate) fn projected_owner_for_import(
        &self,
        namespace: &str,
        name: &str,
    ) -> Option<(String, String)> {
        let item = self.item(namespace, name)?;
        let owner_path = match &item.kind {
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => {
                return Some((item.namespace.clone(), item.name.clone()));
            }
            ProjectedKind::Function(function) => match &function.result {
                ProjectedType::Foreign {
                    rust_path,
                    base_rust_path,
                    ..
                } => {
                    if base_rust_path.is_empty() {
                        rust_path
                    } else {
                        base_rust_path
                    }
                }
                _ => return None,
            },
            ProjectedKind::Interface(_) => return None,
        };
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|candidate| projected_owner_path_matches(&candidate.rust_path, owner_path))
            .map(|candidate| (candidate.namespace.clone(), candidate.name.clone()))
    }

    pub(crate) fn projected_member_result_owner(
        &self,
        namespace: &str,
        owner: &str,
        member: &str,
    ) -> Option<(String, String)> {
        let item = self.item(namespace, owner)?;
        let function = match &item.kind {
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => methods
                .iter()
                .chain(static_methods)
                .find(|function| function.name == member)?,
            _ => return None,
        };
        let ProjectedType::Foreign {
            rust_path,
            base_rust_path,
            ..
        } = &function.result
        else {
            return None;
        };
        let owner_path = if base_rust_path.is_empty() {
            rust_path
        } else {
            base_rust_path
        };
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|candidate| projected_owner_path_matches(&candidate.rust_path, owner_path))
            .map(|candidate| (candidate.namespace.clone(), candidate.name.clone()))
    }

    pub(crate) fn item_ambiguity(&self, namespace: &str, name: &str) -> Option<String> {
        let mut paths = self
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| item.namespace == namespace && item.name == name)
            .map(|item| item.rust_path.as_str())
            .collect::<Vec<_>>();
        if paths.len() < 2 {
            return None;
        }
        paths.sort_unstable();
        paths.dedup();
        Some(if paths.len() == 1 {
            format!("multiple projected items for Rust type `{}`", paths[0])
        } else {
            format!("projected Rust types `{}`", paths.join("`, `"))
        })
    }

    #[must_use]
    pub(crate) fn projected_type(&self, namespace: &str, name: &str) -> Option<ProjectedType> {
        let mut candidates = Vec::new();
        for item in self
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| item.namespace == namespace)
        {
            match &item.kind {
                ProjectedKind::Function(projected) => {
                    collect_function_projected_types(projected, name, &mut candidates);
                }
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => {
                    if item.name == name {
                        candidates.push(ProjectedType::Foreign {
                            rust_path: item.rust_path.clone(),
                            name: item.name.clone(),
                            base_rust_path: item.rust_path.clone(),
                            arguments: Vec::new(),
                        });
                    }
                    for method in methods.iter().chain(static_methods) {
                        collect_function_projected_types(method, name, &mut candidates);
                    }
                }
                ProjectedKind::Interface(interface) => {
                    for method in &interface.methods {
                        collect_function_projected_types(&method.function, name, &mut candidates);
                    }
                }
            }
        }
        let first = candidates.first()?;
        let first_owner = projected_type_owner(first)?;
        candidates
            .iter()
            .all(|candidate| projected_type_owner(candidate) == Some(first_owner))
            .then(|| candidates.remove(0))
    }

    #[must_use]
    pub(crate) fn projected_type_is_send(&self, namespace: &str, name: &str) -> bool {
        let direct = self.item(namespace, name);
        let instantiated = self.projected_type(namespace, name).and_then(|projected| {
            let base = match projected {
                ProjectedType::Foreign { base_rust_path, .. } => base_rust_path,
                ProjectedType::BoxedInterface { trait_path, .. } => trait_path,
                _ => return None,
            };
            self.dependencies
                .iter()
                .flat_map(|dependency| &dependency.items)
                .find(|item| item.rust_path == base)
        });
        match direct.or(instantiated).map(|item| &item.kind) {
            Some(ProjectedKind::ForeignType { send, .. } | ProjectedKind::Enum { send, .. }) => {
                *send
            }
            Some(ProjectedKind::Interface(interface)) => interface.send,
            _ => false,
        }
    }
    #[must_use]
    pub fn foreign_rust_path(&self, namespace: &str, name: &str) -> Option<&str> {
        self.item(namespace, name)
            .filter(|item| {
                matches!(
                    item.kind,
                    ProjectedKind::ForeignType { .. }
                        | ProjectedKind::Interface(_)
                        | ProjectedKind::Enum { .. }
                )
            })
            .map(|item| item.rust_path.as_str())
    }

    #[must_use]
    pub fn method(
        &self,
        namespace: &str,
        type_name: &str,
        method_name: &str,
        is_static: bool,
    ) -> Option<&ProjectedFunction> {
        let rust_path = self.foreign_rust_path(namespace, type_name)?;
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| item.rust_path == rust_path)
            .and_then(|item| match &item.kind {
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => {
                    let candidates = if is_static { static_methods } else { methods };
                    candidates.iter().find(|method| method.name == method_name)
                }
                ProjectedKind::Interface(interface) if !is_static => interface
                    .methods
                    .iter()
                    .find(|method| method.function.name == method_name)
                    .map(|method| &method.function),
                _ => None,
            })
    }

    #[must_use]
    pub fn projected_identity_for_rust_path(&self, rust_path: &str) -> Option<(&str, &str)> {
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| item.rust_path == rust_path)
            .map(|item| (item.namespace.as_str(), item.name.as_str()))
    }

    pub fn is_projected_error_type(&self, namespace: &str, name: &str) -> bool {
        let Some(error_item) = self
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| item.namespace == namespace && item.name == name)
        else {
            return false;
        };
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .flat_map(projected_item_functions)
            .any(|function| function.error.as_deref() == Some(error_item.rust_path.as_str()))
    }
    #[must_use]
    pub(crate) fn interface_method(
        &self,
        namespace: &str,
        type_name: &str,
        method_name: &str,
    ) -> Option<&ProjectedInterfaceMethod> {
        let item = self.item(namespace, type_name)?;
        let ProjectedKind::Interface(interface) = &item.kind else {
            return None;
        };
        interface
            .methods
            .iter()
            .find(|method| method.function.name == method_name)
    }

    #[must_use]
    pub(crate) fn foreign_owns_resource(&self, namespace: &str, name: &str) -> bool {
        self.item(namespace, name).is_some_and(|item| {
            matches!(
                &item.kind,
                ProjectedKind::ForeignType { methods, .. }
                    if methods.iter().any(|method| {
                        method.is_async || matches!(method.receiver, Some(Receiver::Move))
                    })
            )
        })
    }

    #[must_use]
    pub(crate) fn foreign_is_cloneable(&self, namespace: &str, name: &str) -> Option<bool> {
        self.item(namespace, name)
            .and_then(|item| match &item.kind {
                ProjectedKind::ForeignType { cloneable, .. } => Some(*cloneable),
                _ => None,
            })
    }

    #[must_use]
    pub(crate) fn foreign_auto_traits(&self, namespace: &str, name: &str) -> Option<(bool, bool)> {
        self.item(namespace, name)
            .and_then(|item| match &item.kind {
                ProjectedKind::ForeignType { send, sync, .. } => Some((*send, *sync)),
                ProjectedKind::Interface(interface) => Some((interface.send, interface.sync)),
                ProjectedKind::Enum { .. } => Some((true, true)),
                ProjectedKind::Function(_) => None,
            })
    }

    #[must_use]
    pub(crate) fn declined_method_reason(
        &self,
        namespace: &str,
        type_name: &str,

        method_name: &str,
    ) -> Option<&str> {
        let declined_path = self
            .foreign_rust_path(namespace, type_name)
            .map(|path| format!("{path}::{method_name}"));
        let suffix = format!("::{method_name}");
        Self::unique_declined_reason(&self.dependencies, declined_path.as_deref(), &suffix)
    }

    fn unique_declined_reason<'a>(
        dependencies: &'a [ProjectedDependency],
        exact_path: Option<&str>,
        suffix: &str,
    ) -> Option<&'a str> {
        let qualified_prefix = exact_path
            .and_then(|path| path.strip_suffix(suffix))
            .map(|owner| format!("<{owner} as "));
        let mut matches = dependencies
            .iter()
            .flat_map(|dependency| &dependency.declined)
            .filter(|item| match exact_path {
                Some(path) => {
                    item.rust_path == path
                        || qualified_prefix.as_ref().is_some_and(|prefix| {
                            item.rust_path.starts_with(prefix) && item.rust_path.ends_with(suffix)
                        })
                }
                None => item.rust_path.ends_with(suffix),
            })
            .map(|item| item.reason.as_str());
        let reason = matches.next()?;
        matches
            .all(|candidate| candidate == reason)
            .then_some(reason)
    }

    #[must_use]
    pub(crate) fn has_borrowed_async_method_named(&self, name: &str) -> bool {
        self.dependencies.iter().any(|dependency| {
            dependency.items.iter().any(|item| {
                matches!(&item.kind, ProjectedKind::ForeignType { methods, .. } if methods.iter().any(|method| {
                    method.name == name
                        && method.is_async
                        && matches!(
                            method.receiver,
                            Some(Receiver::Borrow | Receiver::MutableBorrow)
                        )
                }))
            })
        })
    }

    #[must_use]
    pub(crate) fn is_unit_variant(&self, item: &ProjectedItem) -> bool {
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .any(|candidate| {
                matches!(
                    candidate.kind,
                    ProjectedKind::Enum {
                        data_carrying: false,
                        ..
                    }
                ) && item
                    .rust_path
                    .starts_with(&format!("{}::", candidate.rust_path))
            })
    }
}

fn projected_item_functions(item: &ProjectedItem) -> Vec<&ProjectedFunction> {
    match &item.kind {
        ProjectedKind::Function(function) => vec![function],
        ProjectedKind::ForeignType {
            methods,
            static_methods,
            ..
        }
        | ProjectedKind::Enum {
            methods,
            static_methods,
            ..
        } => methods.iter().chain(static_methods).collect(),
        ProjectedKind::Interface(interface) => interface
            .methods
            .iter()
            .map(|method| &method.function)
            .collect(),
    }
}

fn projected_member_is_demanded(
    namespace: &str,
    owner: &str,
    member: &str,
    demanded: &ProjectedMemberDemands,
) -> bool {
    demanded
        .get(&(namespace.to_owned(), owner.to_owned()))
        .is_some_and(|members| members.contains(member))
}

fn expanded_source_imports(
    all_items: &[&ProjectedItem],
    imports: &BTreeMap<String, BTreeSet<String>>,
    demanded_members: &ProjectedMemberDemands,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut expanded = imports.clone();
    loop {
        let previous_count = expanded.values().map(BTreeSet::len).sum::<usize>();
        let selected = all_items
            .iter()
            .copied()
            .filter(|item| {
                expanded
                    .get(&item.namespace)
                    .is_some_and(|names| names.contains(&item.name))
            })
            .collect::<Vec<_>>();
        for (rust_path, name) in collect_source_foreign(all_items, &selected, demanded_members) {
            if let Some(item) = projected_item_for_foreign(all_items, &rust_path, &name) {
                expanded
                    .entry(item.namespace.clone())
                    .or_default()
                    .insert(item.name.clone());
            }
        }
        if expanded.values().map(BTreeSet::len).sum::<usize>() == previous_count {
            return expanded;
        }
    }
}

fn projected_item_for_foreign<'a>(
    all_items: &'a [&ProjectedItem],
    rust_path: &str,
    name: &str,
) -> Option<&'a ProjectedItem> {
    if let Some(exact) = all_items
        .iter()
        .copied()
        .find(|item| item.rust_path == rust_path)
    {
        return Some(exact);
    }
    let mut candidates = all_items
        .iter()
        .copied()
        .filter(|item| item.name == name)
        .filter_map(|item| {
            let parent = item.rust_path.rsplit_once("::")?.0;
            rust_path
                .starts_with(&format!("{parent}::"))
                .then_some((parent.len(), item))
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|(prefix_len, _)| std::cmp::Reverse(*prefix_len));
    let (best_len, best) = candidates.first().copied()?;
    (candidates
        .get(1)
        .is_none_or(|(next_len, _)| *next_len < best_len))
    .then_some(best)
}

fn collect_source_foreign(
    all_items: &[&ProjectedItem],
    selected: &[&ProjectedItem],
    demanded_members: &ProjectedMemberDemands,
) -> BTreeMap<String, String> {
    let mut foreign = BTreeMap::<String, String>::new();
    for item in selected {
        match &item.kind {
            ProjectedKind::Function(function) => {
                collect_foreign_function(function, &mut foreign);
            }
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
                for method in methods.iter().chain(static_methods).filter(|method| {
                    projected_member_is_demanded(
                        &item.namespace,
                        &item.name,
                        &method.name,
                        demanded_members,
                    )
                }) {
                    collect_foreign_function(method, &mut foreign);
                }
            }
            ProjectedKind::Interface(interface) => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
                for supertrait in &interface.supertraits {
                    foreign.insert(supertrait.rust_path.clone(), supertrait.name.clone());
                }
                for method in &interface.methods {
                    collect_foreign_function(&method.function, &mut foreign);
                }
            }
        }
    }
    loop {
        let previous_len = foreign.len();
        let referenced = foreign.keys().cloned().collect::<Vec<_>>();
        for rust_path in referenced {
            let Some(item) = all_items
                .iter()
                .copied()
                .find(|item| item.rust_path == rust_path)
            else {
                continue;
            };
            let methods = match &item.kind {
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => Some((methods, static_methods)),
                _ => None,
            };
            let Some((methods, static_methods)) = methods else {
                continue;
            };
            for method in methods.iter().chain(static_methods).filter(|method| {
                projected_member_is_demanded(
                    &item.namespace,
                    &item.name,
                    &method.name,
                    demanded_members,
                )
            }) {
                collect_foreign_function(method, &mut foreign);
            }
        }
        if foreign.len() == previous_len {
            return foreign;
        }
    }
}

fn foreign_aliases(foreign: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let counts = foreign.values().fold(BTreeMap::new(), |mut counts, name| {
        *counts.entry(name.as_str()).or_insert(0_usize) += 1;
        counts
    });
    foreign
        .iter()
        .map(|(rust_path, name)| {
            let alias = if counts[name.as_str()] == 1 {
                name.clone()
            } else {
                rust_path.replace("::", "-").replace('_', "-")
            };
            (rust_path.clone(), alias)
        })
        .collect()
}

fn collect_foreign_function(function: &ProjectedFunction, foreign: &mut BTreeMap<String, String>) {
    for ty in function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
    {
        collect_foreign_type(ty, foreign);
    }
}

fn collect_foreign_type(ty: &ProjectedType, foreign: &mut BTreeMap<String, String>) {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            name,
            arguments,
            ..
        } => {
            foreign.insert(rust_path.clone(), name.clone());
            for argument in arguments {
                collect_foreign_type(argument, foreign);
            }
        }
        ProjectedType::BoxedInterface {
            trait_path,
            name,
            associated_type,
            ..
        } => {
            foreign.insert(trait_path.clone(), name.clone());
            if let Some(associated) = associated_type {
                collect_foreign_type(&associated.ty, foreign);
            }
        }

        ProjectedType::Optional(inner)
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. } => collect_foreign_type(inner, foreign),
        ProjectedType::Mapping { key, value, .. } => {
            collect_foreign_type(key, foreign);
            collect_foreign_type(value, foreign);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                collect_foreign_type(item, foreign);
            }
        }
        ProjectedType::Callback {
            parameters, result, ..
        } => {
            for parameter in parameters {
                collect_foreign_type(parameter, foreign);
            }
            collect_foreign_type(result, foreign);
        }
        _ => {}
    }
}
fn render_foreign_declaration(
    output: &mut String,
    namespace: &str,
    name: &str,
    projected_item: Option<&ProjectedItem>,
    aliases: &BTreeMap<String, String>,
    demanded_members: &ProjectedMemberDemands,
) {
    if let Some(item) = projected_item
        && item.namespace != namespace
    {
        write!(output, "from {} import {}", item.namespace, item.name)
            .expect("writing to a string cannot fail");
        if name != item.name {
            write!(output, " as {name}").expect("writing to a string cannot fail");
        }
        output.push('\n');
        return;
    }
    match projected_item.map(|item| &item.kind) {
        Some(ProjectedKind::Interface(interface)) => {
            writeln!(output, "interface {name}").expect("writing to a string cannot fail");
            for method in &interface.methods {
                render_interface_method(output, method, aliases);
            }
        }
        projected_kind => {
            writeln!(output, "class {name}").expect("writing to a string cannot fail");
            if let Some(
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                },
            ) = projected_kind
            {
                for method in methods.iter().filter(|method| {
                    projected_member_is_demanded(namespace, name, &method.name, demanded_members)
                }) {
                    render_function(output, method, false, 4, aliases, None);
                }
                for method in static_methods.iter().filter(|method| {
                    projected_member_is_demanded(namespace, name, &method.name, demanded_members)
                }) {
                    render_function(output, method, false, 4, aliases, Some(name));
                }
            }
        }
    }
    output.push('\n');
}

fn render_interface_method(
    output: &mut String,
    method: &ProjectedInterfaceMethod,
    foreign_aliases: &BTreeMap<String, String>,
) {
    if let Some(docs) = &method.docs {
        for line in docs.lines() {
            writeln!(output, "  ## {}", line.trim()).expect("writing to a string cannot fail");
        }
    }
    let function = &method.function;
    let mode = match function.receiver {
        Some(Receiver::MutableBorrow) => "mutable ",
        Some(Receiver::Move) => "consuming ",
        _ => "",
    };
    let asynchronous = if function.is_async { "async " } else { "" };
    write!(output, "  {mode}{asynchronous}function {}", function.name)
        .expect("writing to a string cannot fail");
    if function.result != ProjectedType::None {
        write!(
            output,
            " {}",
            projected_type_name(&function.result, foreign_aliases)
        )
        .expect("writing to a string cannot fail");
    }
    if function.error.is_some() {
        output.push_str(" throws dependency-error");
    }
    output.push(';');
    if !function.parameters.is_empty() {
        output.push(' ');
        for (index, parameter) in function.parameters.iter().enumerate() {
            if index > 0 {
                output.push_str(", ");
            }
            write!(
                output,
                "{} {}",
                parameter.name,
                projected_type_name(&parameter.ty, foreign_aliases)
            )
            .expect("writing to a string cannot fail");
        }
    }
    output.push('\n');
}

fn foreign_function_dependency_count(function: &ProjectedFunction, owner: &str) -> usize {
    function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
        .filter(|ty| foreign_type_name(ty).is_some_and(|name| name != owner))
        .count()
}

fn foreign_type_name(ty: &ProjectedType) -> Option<&str> {
    match ty {
        ProjectedType::Foreign { name, .. } => Some(name),
        ProjectedType::Optional(inner)
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. } => foreign_type_name(inner),
        ProjectedType::Mapping { key, value, .. } => {
            foreign_type_name(key).or_else(|| foreign_type_name(value))
        }
        ProjectedType::Tuple(items) => items.iter().find_map(foreign_type_name),
        _ => None,
    }
}

fn render_function(
    output: &mut String,
    function: &ProjectedFunction,
    public: bool,
    indent: usize,
    foreign_aliases: &BTreeMap<String, String>,
    static_owner: Option<&str>,
) {
    let prefix = " ".repeat(indent);
    let visibility = if public { "public " } else { "" };
    let static_ = if static_owner.is_some() {
        "static "
    } else {
        ""
    };
    let asynchronous = if function.is_async { "async " } else { "" };
    write!(
        output,
        "{prefix}{visibility}{static_}{asynchronous}function {}",
        function.name
    )
    .expect("writing to a string cannot fail");
    if function.result != ProjectedType::None && function.destination_result.is_none() {
        write!(
            output,
            " {}",
            projected_type_name(&function.result, foreign_aliases)
        )
        .expect("writing to a string cannot fail");
    }
    output.push_str(if function.error.is_some() {
        " throws dependency-error"
    } else {
        " throws dependency-panic"
    });
    output.push(';');
    if !function.parameters.is_empty() {
        output.push(' ');
        for (index, parameter) in function.parameters.iter().enumerate() {
            if index != 0 {
                output.push_str(", ");
            }
            write!(
                output,
                "{} {}",
                parameter.name,
                projected_type_name(&parameter.ty, foreign_aliases)
            )
            .expect("writing to a string cannot fail");
        }
    }
    output.push('\n');
}

fn projected_type_name(ty: &ProjectedType, foreign_aliases: &BTreeMap<String, String>) -> String {
    match ty {
        ProjectedType::Foreign {
            rust_path, name, ..
        } => foreign_aliases
            .get(rust_path)
            .cloned()
            .unwrap_or_else(|| name.clone()),
        ProjectedType::BoxedInterface {
            trait_path,
            name,
            associated_type,
            ..
        } => {
            let base = foreign_aliases
                .get(trait_path)
                .cloned()
                .unwrap_or_else(|| name.clone());
            associated_type.as_ref().map_or(base.clone(), |associated| {
                format!(
                    "{base} of {}",
                    projected_type_name(&associated.ty, foreign_aliases)
                )
            })
        }
        ProjectedType::Optional(inner) => {
            format!("{}|none", projected_type_name(inner, foreign_aliases))
        }
        ProjectedType::AsyncIterationStep(inner) => {
            format!(
                "async-iteration-step of {}",
                projected_type_name(inner, foreign_aliases)
            )
        }
        ProjectedType::Sequence { item, .. } => {
            format!("list of {}", projected_type_name(item, foreign_aliases))
        }
        ProjectedType::Mapping {
            key,
            value,
            ordered,
            ..
        } => format!(
            "{}map of {}, {}",
            if *ordered { "" } else { "unordered-" },
            projected_type_name(key, foreign_aliases),
            projected_type_name(value, foreign_aliases)
        ),
        ProjectedType::Set { item, ordered, .. } => format!(
            "{}set of {}",
            if *ordered { "" } else { "unordered-" },
            projected_type_name(item, foreign_aliases)
        ),
        ProjectedType::Tuple(items) => format!(
            "tuple of {}",
            projected_type_name(&items[0], foreign_aliases)
        ),
        ProjectedType::Callback {
            parameters,
            result,
            invocation_mode,
            is_async,
            ..
        } => {
            let mut rendered = format!(
                "{}{}function",
                invocation_mode.source_prefix(),
                if *is_async { "async " } else { "" }
            );
            if !parameters.is_empty() {
                write!(
                    rendered,
                    " from {}",
                    parameters
                        .iter()
                        .map(|parameter| projected_type_name(parameter, foreign_aliases))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .expect("writing to a string cannot fail");
            }
            write!(
                rendered,
                " to {}",
                projected_type_name(result, foreign_aliases)
            )
            .expect("writing to a string cannot fail");
            rendered
        }
        _ => ty.terrane_name(),
    }
}
#[derive(Debug)]
pub struct ProjectionError {
    pub message: String,
}

impl std::fmt::Display for ProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ProjectionError {}

impl From<terrane_rust_analysis::AnalysisError> for ProjectionError {
    fn from(error: terrane_rust_analysis::AnalysisError) -> Self {
        Self {
            message: error.message,
        }
    }
}

fn decline_unproven_projected_interfaces(
    projected: &mut [ProjectedDependency],
    evidence: &[crate::projection_oracle::ImplProbeEvidence],
) {
    for evidence in evidence
        .iter()
        .filter(|evidence| evidence.answer != crate::ProbeAnswer::Yes)
    {
        for dependency in &mut *projected {
            let Some(index) = dependency
                .items
                .iter()
                .position(|item| item.rust_path == evidence.question.label)
            else {
                continue;
            };
            let item = dependency.items.remove(index);
            dependency.declined.push(DeclinedItem {
                rust_path: item.rust_path,
                reason:
                    "trait implementation signature is not representable against the resolved dependency"
                        .to_owned(),
            });
            break;
        }
    }
}
fn decline_functions_with_missing_generic_interfaces(projected: &mut [ProjectedDependency]) {
    let projected_interfaces = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| matches!(item.kind, ProjectedKind::Interface(_)))
        .map(|item| item.rust_path.clone())
        .collect::<BTreeSet<_>>();
    for dependency in projected {
        let mut retained = Vec::with_capacity(dependency.items.len());
        for item in std::mem::take(&mut dependency.items) {
            let declined_bound = match &item.kind {
                ProjectedKind::Function(function) => function
                    .parameters
                    .iter()
                    .filter_map(|parameter| parameter.generic_interface.as_ref())
                    .find(|bound| !projected_interfaces.contains(*bound))
                    .cloned(),
                _ => None,
            };
            if let Some(bound) = declined_bound {
                dependency.declined.push(DeclinedItem {
                    rust_path: item.rust_path,
                    reason: format!("generic input references declined interface `{bound}`"),
                });
            } else {
                retained.push(item);
            }
        }
        dependency.items = retained;
    }
}
/// Resolves every declared Rust package and derives the shared Terrane projection.
///
/// # Errors
/// Returns a projection error when Cargo resolution, rustdoc generation, cache input reading, or
/// projection of the resolved metadata fails.
#[expect(
    clippy::too_many_lines,
    reason = "one transactional resolution path owns fetch, exact cache, artifact, and local fallback"
)]
pub fn resolve(
    root: &Path,
    dependencies: &[RustDependency],
) -> Result<Projection, ProjectionError> {
    let sandbox = containment();
    if dependencies.is_empty() {
        let mut projection = Projection {
            cache_identity: String::from("no-rust-dependencies"),
            content_hash: String::new(),
            source: ProjectionSource::Local,
            dependencies: Vec::new(),
            probes: Vec::new(),
            bound_dependencies: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution {
                outcome: ResolutionOutcome::NoDependencies,
                events: Vec::new(),
            },
            removed: Vec::new(),
            containment: sandbox,
        };
        projection.content_hash = projection_content_hash(&projection)?;
        return Ok(projection);
    }
    // The declared manifest and resolved lock are hashed into the projection identity before any
    // review-visible bound-owner edges are injected. Once such edges exist, Cargo cannot accept
    // that deliberate manifest rewrite under `--locked`; offline resolution plus exact pins and
    // the projection content hash preserve the already-resolved graph without network drift.
    // Path-dependency source and package-metadata contents are deliberately outside this identity:
    // changing either without changing the consumer manifest or lock requires clearing the
    // projection cache. Namespace-overlay metadata follows that existing invalidation boundary so
    // warm cache hits remain metadata-free.
    let workspace = root.join(".trn/dependencies");
    write_workspace(&workspace, dependencies)?;
    if workspace.join("Cargo.lock").exists() {
        run_cargo(
            &workspace,
            &["fetch", "--offline"],
            CargoToolchain::Default,
            CargoExecution::Host,
        )?;
    } else {
        run_cargo(
            &workspace,
            &["fetch"],
            CargoToolchain::Default,
            CargoExecution::Host,
        )?;
    }
    let (identity, target) = cache_identity(root, &workspace, dependencies, sandbox)?;
    let cache_path = workspace.join(format!("projection-{identity}.json"));
    if let Ok(bytes) = fs::read(&cache_path) {
        let mut cached =
            serde_json::from_slice::<Projection>(&bytes).map_err(|error| ProjectionError {
                message: format!(
                    "invalid cached dependency projection `{}`: {error}",
                    cache_path.display()
                ),
            })?;
        let actual_hash = projection_content_hash(&cached)?;
        if cached.content_hash != actual_hash {
            return Err(ProjectionError {
                message: format!(
                    "cached dependency projection `{}` failed its content hash: expected `{}`, computed `{actual_hash}`",
                    cache_path.display(),
                    cached.content_hash
                ),
            });
        }
        cached.containment = sandbox;
        cached.resolution = ProjectionResolution {
            outcome: ResolutionOutcome::ExactCache,
            events: vec![ResolutionEvent {
                source: ResolutionSource::ExactCache,
                status: ResolutionStatus::Hit,
                reason: format!("matched projection identity `{identity}`"),
            }],
        };
        write_workspace_with_bound_dependencies(
            &workspace,
            dependencies,
            &cached.bound_dependencies,
        )?;
        apply_projection_history(root, &mut cached)?;
        prune_projection_cache(&workspace, &cache_path)?;
        return Ok(cached);
    }

    let mut resolution_events = vec![ResolutionEvent {
        source: ResolutionSource::BundledArtifact,
        status: ResolutionStatus::Skipped,
        reason: "bundled projection distribution is deliberately deferred until Terrane has a release artifact channel".to_owned(),
    }];
    match fetch_remote_projection(&identity, &target, dependencies, sandbox)? {
        PublishedProjection::Hit(mut projection) => {
            resolution_events.push(ResolutionEvent {
                source: ResolutionSource::PublishedArtifact,
                status: ResolutionStatus::Hit,
                reason: format!(
                    "verified identity and content hash `{}`",
                    projection.content_hash
                ),
            });
            projection.resolution = ProjectionResolution {
                outcome: ResolutionOutcome::PublishedArtifact,
                events: resolution_events,
            };
            write_workspace_with_bound_dependencies(
                &workspace,
                dependencies,
                &projection.bound_dependencies,
            )?;
            let bytes =
                serde_json::to_vec_pretty(&projection).map_err(|error| ProjectionError {
                    message: format!("cannot serialize published dependency projection: {error}"),
                })?;
            write_if_changed(&cache_path, &bytes)?;
            apply_projection_history(root, &mut projection)?;
            prune_projection_cache(&workspace, &cache_path)?;
            projection
                .probes
                .sort_by(|left, right| left.question.cmp(&right.question));
            return Ok(projection);
        }
        PublishedProjection::Event(event) => resolution_events.push(event),
    }

    let metadata = resolved_dependency_metadata(&workspace)?;
    let overlays = namespace_overlays_from_metadata(&metadata, dependencies)?;

    let mut rustdocs = Vec::new();
    for dependency in dependencies {
        let package_spec = dependency.version.strip_prefix('=').map_or_else(
            || dependency.package.clone(),
            |version| format!("{}@{version}", dependency.package),
        );
        let crate_name = dependency.package.replace('-', "_");
        let document = generate_rustdoc(
            &workspace,
            &package_spec,
            &crate_name,
            &dependency.package,
            sandbox,
            false,
        )?;
        let public_paths = rustdoc_public_paths(&document);
        rustdocs.push((dependency, document, public_paths));
    }
    let reexport_rustdocs =
        external_reexport_rustdocs(&workspace, &rustdocs, &metadata, sandbox)?;
    let mut canonical_public_paths = BTreeMap::new();
    for (_, document, public_paths) in &rustdocs {
        for (id, public_path) in public_paths {
            if let Some(summary) = document.paths.get(id) {
                canonical_public_paths
                    .entry(summary.path.join("::"))
                    .and_modify(|current| prefer_alias(current, public_path))
                    .or_insert_with(|| public_path.clone());
            }
        }
    }
    let mut projected = rustdocs
        .iter()
        .map(|(dependency, document, public_paths)| {
            project_rustdoc(
                dependency,
                document,
                public_paths,
                &canonical_public_paths,
                true,
            )
        })
        .collect::<Vec<_>>();
    for reexport in &reexport_rustdocs {
        for (dependency_index, public_paths) in &reexport.providers {
            let mut fragment = project_rustdoc(
                dependencies
                    .get(*dependency_index)
                    .expect("reexport provider index came from declared dependencies"),
                &reexport.document,
                public_paths,
                &canonical_public_paths,
                false,
            );
            let dependency = projected
                .get_mut(*dependency_index)
                .expect("reexport provider index came from projected dependencies");
            dependency.items.append(&mut fragment.items);
            dependency.declined.append(&mut fragment.declined);
            dependency.items.sort_by(|left, right| {
                (&left.namespace, &left.name, &left.rust_path).cmp(&(
                    &right.namespace,
                    &right.name,
                    &right.rust_path,
                ))
            });
            dependency.items.dedup_by(|left, right| {
                left.namespace == right.namespace
                    && left.name == right.name
                    && left.rust_path == right.rust_path
            });
            dependency.declined.sort_by(|left, right| {
                (&left.rust_path, &left.reason).cmp(&(&right.rust_path, &right.reason))
            });
            dependency.declined.dedup();
        }
    }
    project_external_provided_trait_methods(&mut projected, &rustdocs, &canonical_public_paths);
    apply_namespace_overlays(&mut projected, &overlays)?;
    resolve_cross_dependency_boundary_conversions(&mut projected);
    enforce_transitive_reachability(&mut projected, dependencies, &workspace, false)?;
    canonicalize_projected_type_names(&mut projected);
    enforce_transitive_reachability(&mut projected, dependencies, &workspace, true)?;
    decline_unrepresentable_error_types(&mut projected);
    let auto_trait_questions = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| {
            matches!(
                item.kind,
                ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
            )
        })
        .flat_map(|item| {
            ["Send", "Sync"]
                .into_iter()
                .map(|rust_bound| crate::projection_oracle::BoundQuestion {
                    rust_type: item.rust_path.clone(),
                    rust_bound: rust_bound.to_owned(),
                    inferred_parameters: Vec::new(),
                })
        })
        .collect::<Vec<_>>();
    if !auto_trait_questions.is_empty() {
        let report =
            crate::projection_oracle::ProjectionOracle::new(&workspace, &identity, sandbox)
                .prove_bounds(&auto_trait_questions)?;
        for evidence in report.evidence {
            let satisfied = evidence.answer == crate::projection_oracle::ProbeAnswer::Yes;
            for item in projected
                .iter_mut()
                .flat_map(|dependency| &mut dependency.items)
                .filter(|item| item.rust_path == evidence.question.rust_type)
            {
                match &mut item.kind {
                    ProjectedKind::ForeignType { send, sync, .. }
                    | ProjectedKind::Enum { send, sync, .. } => {
                        match evidence.question.rust_bound.as_str() {
                            "Send" => *send = satisfied,
                            "Sync" => *sync = satisfied,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    let impl_questions = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match &item.kind {
            ProjectedKind::Interface(interface) => {
                Some(projected_interface_impl_question(item, interface))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    if !impl_questions.is_empty() {
        let report =
            crate::projection_oracle::ProjectionOracle::new(&workspace, &identity, sandbox)
                .prove_impls(&impl_questions)?;
        decline_unproven_projected_interfaces(&mut projected, &report.evidence);
    }
    resolution_events.push(ResolutionEvent {
        source: ResolutionSource::LocalRustdoc,
        status: ResolutionStatus::Generated,
        reason: "no reusable exact artifact was available; generated with the pinned local rustdoc toolchain".to_owned(),
    });
    decline_unnameable_bound_owners(&mut projected, dependencies, &workspace)?;
    decline_functions_with_missing_generic_interfaces(&mut projected);
    let bound_dependencies = projected_bound_dependencies(&projected, dependencies, &workspace)?;
    let mut projection = Projection {
        cache_identity: identity,
        content_hash: String::new(),
        source: ProjectionSource::Local,
        dependencies: projected,
        bound_dependencies,
        containment: sandbox,
        probes: Vec::new(),
        probe_wall_time_ms: 0,
        resolution: ProjectionResolution {
            outcome: ResolutionOutcome::LocalRustdoc,
            events: resolution_events,
        },
        removed: Vec::new(),
    };
    write_workspace_with_bound_dependencies(
        &workspace,
        dependencies,
        &projection.bound_dependencies,
    )?;
    projection.content_hash = projection_content_hash(&projection)?;
    let bytes = serde_json::to_vec_pretty(&projection).map_err(|error| ProjectionError {
        message: format!("cannot serialize dependency projection: {error}"),
    })?;
    write_if_changed(&cache_path, &bytes)?;
    apply_projection_history(root, &mut projection)?;
    prune_projection_cache(&workspace, &cache_path)?;
    Ok(projection)
}
fn fetch_remote_projection(
    identity: &str,
    target: &str,
    dependencies: &[RustDependency],
    containment: Containment,
) -> Result<PublishedProjection, ProjectionError> {
    let Some(base_url) = std::env::var_os("TERRANE_PROJECTION_ARTIFACT_URL") else {
        return Ok(PublishedProjection::Event(ResolutionEvent {
            source: ResolutionSource::PublishedArtifact,
            status: ResolutionStatus::Skipped,
            reason: "`TERRANE_PROJECTION_ARTIFACT_URL` is not configured".to_owned(),
        }));
    };
    let base_url = base_url.to_string_lossy();
    if !base_url.starts_with("https://") {
        return Err(ProjectionError {
            message:
                "`TERRANE_PROJECTION_ARTIFACT_URL` must name a trusted HTTPS artifact repository"
                    .to_owned(),
        });
    }
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let url = format!("{}/{}.json", base_url.trim_end_matches('/'), identity);
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .https_only(true)
        .timeout_global(Some(Duration::from_secs(10)))
        .build()
        .into();
    let mut response = match agent.get(&url).call() {
        Ok(response) => response,
        Err(error) => {
            return Ok(PublishedProjection::Event(ResolutionEvent {
                source: ResolutionSource::PublishedArtifact,
                status: ResolutionStatus::Miss,
                reason: format!("artifact request `{url}` was unavailable: {error}"),
            }));
        }
    };
    let bytes = match response
        .body_mut()
        .with_config()
        .limit(64 * 1024 * 1024)
        .read_to_vec()
    {
        Ok(bytes) => bytes,
        Err(error) => {
            return Ok(PublishedProjection::Event(ResolutionEvent {
                source: ResolutionSource::PublishedArtifact,
                status: ResolutionStatus::Rejected,
                reason: format!("artifact body could not be read: {error}"),
            }));
        }
    };
    let artifact = match serde_json::from_slice::<ProjectionArtifact>(&bytes) {
        Ok(artifact) => artifact,
        Err(error) => {
            return Ok(PublishedProjection::Event(ResolutionEvent {
                source: ResolutionSource::PublishedArtifact,
                status: ResolutionStatus::Rejected,
                reason: format!("artifact JSON did not match the projection envelope: {error}"),
            }));
        }
    };
    match validate_projection_artifact(artifact, identity, target, dependencies, containment) {
        Ok(projection) => Ok(PublishedProjection::Hit(projection)),
        Err(reason) => Ok(PublishedProjection::Event(ResolutionEvent {
            source: ResolutionSource::PublishedArtifact,
            status: ResolutionStatus::Rejected,
            reason,
        })),
    }
}

fn artifact_dependency_mismatch(
    actual: &[ArtifactDependency],
    expected: &[ArtifactDependency],
) -> Option<String> {
    if actual.len() != expected.len() {
        return Some(format!(
            "dependency metadata mismatch: expected {} entries, found {}",
            expected.len(),
            actual.len()
        ));
    }
    for (actual, expected) in actual.iter().zip(expected) {
        if actual.name != expected.name
            || actual.package != expected.package
            || actual.version != expected.version
            || actual.effects != expected.effects
        {
            return Some(format!(
                "dependency metadata mismatch for `{}`",
                expected.name
            ));
        }
        if actual.features != expected.features
            || actual.default_features != expected.default_features
        {
            return Some(format!("feature metadata mismatch for `{}`", expected.name));
        }
        if actual.target != expected.target {
            return Some(format!(
                "dependency target mismatch for `{}`: expected `{}`, found `{}`",
                expected.name,
                expected.target.as_deref().unwrap_or("all targets"),
                actual.target.as_deref().unwrap_or("all targets")
            ));
        }
    }
    None
}

fn validate_projection_artifact(
    artifact: ProjectionArtifact,
    identity: &str,
    target: &str,
    dependencies: &[RustDependency],
    containment: Containment,
) -> Result<Projection, String> {
    let expected_dependencies = dependencies
        .iter()
        .map(ArtifactDependency::from)
        .collect::<Vec<_>>();
    let mismatch = if artifact.format != 1 {
        Some(format!(
            "envelope format {} is not supported",
            artifact.format
        ))
    } else if artifact.cache_identity != identity {
        Some("cache identity mismatch".to_owned())
    } else if artifact.target != target {
        Some(format!(
            "target mismatch: expected `{target}`, found `{}`",
            artifact.target
        ))
    } else if artifact.build_toolchain != crate::BUILD_TOOLCHAIN {
        Some("stable build toolchain mismatch".to_owned())
    } else if artifact.rustdoc_toolchain != RUSTDOC_TOOLCHAIN {
        Some("rustdoc toolchain mismatch".to_owned())
    } else if artifact.rustdoc_format != rustdoc_types::FORMAT_VERSION {
        Some("rustdoc format mismatch".to_owned())
    } else if artifact.projection_schema != PROJECTION_SCHEMA {
        Some("projection schema mismatch".to_owned())
    } else if let Some(reason) =
        artifact_dependency_mismatch(&artifact.dependencies, &expected_dependencies)
    {
        Some(reason)
    } else if artifact.projection.cache_identity != identity {
        Some("projection payload identity mismatch".to_owned())
    } else {
        None
    };
    if let Some(reason) = mismatch {
        return Err(reason);
    }
    let computed = projection_content_hash(&artifact.projection).map_err(|error| error.message)?;
    if artifact.content_hash != computed {
        return Err(format!(
            "content hash mismatch: envelope recorded `{}`, computed `{computed}`",
            artifact.content_hash
        ));
    }
    if artifact.projection.content_hash != artifact.content_hash {
        return Err("projection payload content hash does not match its envelope".to_owned());
    }
    let mut projection = artifact.projection;
    projection.source = ProjectionSource::Remote;
    projection.containment = containment;
    Ok(projection)
}

fn projection_content_hash(projection: &Projection) -> Result<String, ProjectionError> {
    let payload = serde_json::to_vec(&(
        &projection.cache_identity,
        &projection.dependencies,
        &projection.probes,
        projection.probe_wall_time_ms,
    ))
    .map_err(|error| ProjectionError {
        message: format!("cannot encode projection content hash: {error}"),
    })?;
    Ok(format!("{:x}", Sha256::digest(payload)))
}

fn projection_history_members(dependency: &ProjectedDependency) -> BTreeSet<(String, String)> {
    let mut members = BTreeSet::new();
    for item in &dependency.items {
        members.insert((item.namespace.clone(), item.name.clone()));
        if let ProjectedKind::ForeignType {
            methods,
            static_methods,
            ..
        } = &item.kind
        {
            members.extend(methods.iter().map(|method| {
                (
                    item.namespace.clone(),
                    format!("{}.{}", item.name, method.name),
                )
            }));
            members.extend(static_methods.iter().map(|method| {
                (
                    item.namespace.clone(),
                    format!("{}::{}", item.name, method.name),
                )
            }));
        }
    }
    members
}

fn read_projection_history(path: &Path) -> Result<Option<ProjectionHistory>, ProjectionError> {
    match fs::read(path) {
        Ok(bytes) => {
            let history = serde_json::from_slice::<ProjectionHistory>(&bytes).map_err(|error| {
                ProjectionError {
                    message: format!("invalid projection history `{}`: {error}", path.display()),
                }
            })?;
            if !matches!(history.format, 1..=3) {
                return Err(ProjectionError {
                    message: format!(
                        "unsupported projection history format {} in `{}`",
                        history.format,
                        path.display()
                    ),
                });
            }
            Ok(Some(history))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(ProjectionError {
            message: format!(
                "cannot read projection history `{}`: {error}",
                path.display()
            ),
        }),
    }
}

fn apply_projection_history(
    root: &Path,
    projection: &mut Projection,
) -> Result<(), ProjectionError> {
    let path = root.join("terrane-projection.lock");
    let previous = read_projection_history(&path)?;
    let dependencies = projection
        .dependencies
        .iter()
        .map(|dependency| ProjectionHistoryDependency {
            name: dependency.name.clone(),
            version: dependency.version.clone(),
            members: projection_history_members(dependency),
        })
        .collect::<Vec<_>>();
    if let Some(previous) = &previous
        && matches!(previous.format, 2 | 3)
        && previous.dependencies == dependencies
        && previous.bound_dependencies == projection.bound_dependencies
        && previous.rustdoc_format == Some(rustdoc_types::FORMAT_VERSION)
        && previous.projection_schema.as_deref() == Some(PROJECTION_SCHEMA)
        && previous.cache_identity.as_deref() == Some(&projection.cache_identity)
        && previous.content_hash.as_deref() != Some(&projection.content_hash)
    {
        return Err(ProjectionError {
            message: format!(
                "projection replay mismatch in `{}`: the same cache identity previously produced `{}`, now produced `{}`",
                path.display(),
                previous.content_hash.as_deref().unwrap_or("missing"),
                projection.content_hash
            ),
        });
    }
    let mut removed = previous
        .as_ref()
        .map_or_else(Vec::new, |history| history.removed.clone());
    removed.retain(|removed| {
        !dependencies.iter().any(|dependency| {
            dependency
                .members
                .contains(&(removed.namespace.clone(), removed.name.clone()))
        })
    });
    if let Some(previous) = &previous {
        for old in &previous.dependencies {
            let Some(current) = dependencies
                .iter()
                .find(|dependency| dependency.name == old.name)
            else {
                continue;
            };
            if old.version == current.version {
                continue;
            }
            for (namespace, name) in old.members.difference(&current.members) {
                let removed_item = RemovedItem {
                    namespace: namespace.clone(),
                    name: name.clone(),
                    previous_version: old.version.clone(),
                    current_version: current.version.clone(),
                };
                if !removed.iter().any(|existing| {
                    existing.namespace == removed_item.namespace
                        && existing.name == removed_item.name
                }) {
                    removed.push(removed_item);
                }
            }
        }
    }
    removed
        .sort_by(|left, right| (&left.namespace, &left.name).cmp(&(&right.namespace, &right.name)));
    projection.removed.clone_from(&removed);
    let persisted_resolution = previous
        .as_ref()
        .filter(|history| {
            matches!(history.format, 2 | 3)
                && history.bound_dependencies == projection.bound_dependencies
                && history.cache_identity.as_deref() == Some(&projection.cache_identity)
                && history.content_hash.as_deref() == Some(&projection.content_hash)
                && history.source == Some(projection.source)
        })
        .and_then(|history| history.resolution.clone())
        .unwrap_or_else(|| projection.resolution.clone());
    let history = ProjectionHistory {
        format: 3,
        dependencies,
        bound_dependencies: projection.bound_dependencies.clone(),
        removed,
        cache_identity: Some(projection.cache_identity.clone()),
        source: Some(projection.source),
        rustdoc_format: Some(rustdoc_types::FORMAT_VERSION),
        projection_schema: Some(PROJECTION_SCHEMA.to_owned()),
        content_hash: Some(projection.content_hash.clone()),
        resolution: Some(persisted_resolution),
    };
    let mut bytes = serde_json::to_vec_pretty(&history).map_err(|error| ProjectionError {
        message: format!("cannot serialize projection history: {error}"),
    })?;
    bytes.push(b'\n');
    write_if_changed(&path, &bytes)
}

fn prune_projection_cache(directory: &Path, retained: &Path) -> Result<(), ProjectionError> {
    let entries = fs::read_dir(directory).map_err(io_error("read dependency projection cache"))?;
    let mut previous = Vec::new();
    for entry in entries {
        let entry = entry.map_err(io_error("read dependency projection cache entry"))?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if path == retained
            || !name.starts_with("projection-")
            || path.extension() != Some(std::ffi::OsStr::new("json"))
        {
            continue;
        }
        let modified = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .map_err(io_error("read dependency projection cache metadata"))?;
        previous.push((modified, path));
    }
    previous.sort_by(|left, right| right.cmp(left));
    for (_, path) in previous
        .into_iter()
        .skip(MAX_PROJECTION_CACHE_RECORDS.saturating_sub(1))
    {
        fs::remove_file(&path).map_err(io_error("remove stale dependency projection"))?;
    }
    Ok(())
}

fn write_workspace(
    directory: &Path,
    dependencies: &[RustDependency],
) -> Result<(), ProjectionError> {
    fs::create_dir_all(directory.join("src")).map_err(io_error("create dependency workspace"))?;
    let mut manifest = String::from(
        "[package]\nname = \"terrane_dependency_projection\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\n",
    );
    for dependency in dependencies
        .iter()
        .filter(|dependency| dependency.cargo_manifest_table() == "dependencies")
    {
        manifest.push_str(&dependency.cargo_dependency_spec());
    }
    let target_tables = dependencies
        .iter()
        .map(RustDependency::cargo_manifest_table)
        .filter(|table| table != "dependencies")
        .collect::<BTreeSet<_>>();
    for table in target_tables {
        writeln!(manifest, "\n[{table}]").expect("writing to a string cannot fail");
        for dependency in dependencies
            .iter()
            .filter(|dependency| dependency.cargo_manifest_table() == table)
        {
            manifest.push_str(&dependency.cargo_dependency_spec());
        }
    }
    manifest.push_str("\n[workspace]\n");
    write_if_changed(&directory.join("Cargo.toml"), manifest.as_bytes())?;
    write_if_changed(&directory.join("src/lib.rs"), b"")?;
    Ok(())
}

fn run_cargo(
    directory: &Path,
    arguments: &[&str],
    toolchain: CargoToolchain,
    execution: CargoExecution,
) -> Result<(), ProjectionError> {
    let sandboxed = matches!(execution, CargoExecution::Contained);
    let canonical_directory = if sandboxed {
        Some(
            directory
                .canonicalize()
                .map_err(io_error("canonicalize dependency projection workspace"))?,
        )
    } else {
        None
    };
    let working_directory = canonical_directory.as_deref().unwrap_or(directory);
    let mut command = if sandboxed {
        let mut command = Command::new("bwrap");
        command.args([
            "--die-with-parent",
            "--unshare-all",
            "--ro-bind",
            "/",
            "/",
            "--dev",
            "/dev",
            "--proc",
            "/proc",
            "--tmpfs",
            "/tmp",
            "--bind",
        ]);
        command
            .arg(working_directory)
            .arg(working_directory)
            .arg("--")
            .arg("cargo");
        command
    } else {
        Command::new("cargo")
    };
    crate::cargo_toolchain::configure_projection_cargo_command(&mut command);
    if matches!(toolchain, CargoToolchain::RustdocNightly) {
        command.arg(format!("+{RUSTDOC_TOOLCHAIN}"));
    }
    let output = command
        .args(arguments)
        .current_dir(working_directory)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot run Cargo dependency projection: {error}"),
        })?;
    if output.status.success() {
        return Ok(());
    }
    Err(ProjectionError {
        message: format!(
            "Cargo dependency projection failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ),
    })
}

fn resolved_dependency_metadata(
    workspace: &Path,
) -> Result<serde_json::Value, ProjectionError> {
    let mut command = Command::new("cargo");
    crate::cargo_toolchain::configure_projection_cargo_command(&mut command);
    let output = command
        .args(["metadata", "--format-version", "1", "--offline", "--frozen"])
        .current_dir(workspace)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot read Cargo dependency metadata: {error}"),
        })?;
    if !output.status.success() {
        return Err(ProjectionError {
            message: format!(
                "Cargo dependency metadata failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        });
    }
    serde_json::from_slice::<serde_json::Value>(&output.stdout).map_err(|error| ProjectionError {
        message: format!("cannot decode Cargo dependency metadata: {error}"),
    })
}

fn namespace_overlays_from_metadata(
    metadata: &serde_json::Value,
    dependencies: &[RustDependency],
) -> Result<Vec<NamespaceOverlay>, ProjectionError> {
    let packages = metadata
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| ProjectionError {
            message: "Cargo dependency metadata has no package list".to_owned(),
        })?;
    let root = metadata
        .pointer("/resolve/root")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| ProjectionError {
            message: "Cargo dependency metadata has no resolved root package".to_owned(),
        })?;
    let nodes = metadata
        .pointer("/resolve/nodes")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| ProjectionError {
            message: "Cargo dependency metadata has no resolved dependency graph".to_owned(),
        })?;
    let direct = nodes
        .iter()
        .find(|node| node.get("id").and_then(serde_json::Value::as_str) == Some(root))
        .and_then(|node| node.get("deps"))
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| ProjectionError {
            message: "Cargo dependency metadata has no direct dependency graph".to_owned(),
        })?;
    let mut overlays = Vec::new();
    for provider in dependencies {
        let (package, node) = direct_metadata_package(packages, nodes, direct, provider)?;
        let enabled_features = node
            .get("features")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_str)
            .collect::<BTreeSet<_>>();
        overlays.extend(package_namespace_overlays(
            package,
            provider,
            dependencies,
            &enabled_features,
        )?);
    }
    overlays.sort_by(|left, right| {
        (&left.provider_name, &left.source_namespace)
            .cmp(&(&right.provider_name, &right.source_namespace))
    });
    for pair in overlays.windows(2) {
        if pair[0].provider_name == pair[1].provider_name
            && (pair[0].source_namespace == pair[1].source_namespace
                || pair[1]
                    .source_namespace
                    .starts_with(&format!("{}/", pair[0].source_namespace)))
        {
            return Err(ProjectionError {
                message: format!(
                    "Rust dependency `{}` has overlapping namespace overlays `{}` and `{}`",
                    pair[0].provider_name, pair[0].source_namespace, pair[1].source_namespace
                ),
            });
        }
    }
    Ok(overlays)
}

fn direct_metadata_package<'a>(
    packages: &'a [serde_json::Value],
    nodes: &'a [serde_json::Value],
    direct: &[serde_json::Value],
    dependency: &RustDependency,
) -> Result<(&'a serde_json::Value, &'a serde_json::Value), ProjectionError> {
    let cargo_name = dependency.name.replace('-', "_");
    let by_alias = direct.iter().find(|candidate| {
        candidate
            .get("name")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|name| name.replace('-', "_") == cargo_name)
    });
    let by_package = direct
        .iter()
        .filter(|candidate| {
            let package_id = candidate.get("pkg").and_then(serde_json::Value::as_str);
            packages.iter().any(|package| {
                package.get("id").and_then(serde_json::Value::as_str) == package_id
                    && package.get("name").and_then(serde_json::Value::as_str)
                        == Some(dependency.package.as_str())
            })
        })
        .collect::<Vec<_>>();
    let resolved = by_alias.or_else(|| {
        let [candidate] = by_package.as_slice() else {
            return None;
        };
        Some(*candidate)
    });
    let package_id = resolved
        .and_then(|candidate| candidate.get("pkg"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| ProjectionError {
            message: format!(
                "Cargo dependency metadata has no unambiguous resolved package for direct dependency `{}`",
                dependency.name
            ),
        })?;
    let package = packages
        .iter()
        .find(|package| package.get("id").and_then(serde_json::Value::as_str) == Some(package_id))
        .ok_or_else(|| ProjectionError {
            message: format!(
                "Cargo dependency metadata is missing package `{package_id}` for direct dependency `{}`",
                dependency.name
            ),
        })?;
    if package.get("name").and_then(serde_json::Value::as_str) != Some(dependency.package.as_str())
    {
        return Err(ProjectionError {
            message: format!(
                "Cargo dependency metadata resolved `{}` to unexpected package `{}`",
                dependency.name,
                package
                    .get("name")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("<unknown>")
            ),
        });
    }
    let node = nodes
        .iter()
        .find(|node| node.get("id").and_then(serde_json::Value::as_str) == Some(package_id))
        .ok_or_else(|| ProjectionError {
            message: format!(
                "Cargo dependency metadata has no resolved feature set for direct dependency `{}`",
                dependency.name
            ),
        })?;
    Ok((package, node))
}

fn package_namespace_overlays(
    package: &serde_json::Value,
    provider: &RustDependency,
    dependencies: &[RustDependency],
    enabled_features: &BTreeSet<&str>,
) -> Result<Vec<NamespaceOverlay>, ProjectionError> {
    let Some(declarations) = package
        .pointer("/metadata/terrane/namespace-overlays")
        .filter(|value| !value.is_null())
    else {
        return Ok(Vec::new());
    };
    let declarations =
        serde_json::from_value::<Vec<NamespaceOverlayMetadata>>(declarations.clone()).map_err(
            |error| ProjectionError {
                message: format!(
                    "Rust dependency `{}` has invalid `package.metadata.terrane.namespace-overlays`: {error}",
                    provider.name
                ),
            },
        )?;
    let mut overlays = Vec::new();
    for declaration in declarations {
        if !enabled_features.contains(declaration.feature.as_str()) {
            continue;
        }
        let source_module =
            overlay_module_namespace(&declaration.module).ok_or_else(|| ProjectionError {
                message: format!(
                    "Rust dependency `{}` namespace overlay module `{}` is not a Rust module path",
                    provider.name, declaration.module
                ),
            })?;
        let targets = dependencies
            .iter()
            .filter(|dependency| dependency.package == declaration.target_package)
            .collect::<Vec<_>>();
        let [target] = targets.as_slice() else {
            return Err(ProjectionError {
                message: if targets.is_empty() {
                    format!(
                        "Rust dependency `{}` namespace overlay targets undeclared package `{}`",
                        provider.name, declaration.target_package
                    )
                } else {
                    format!(
                        "Rust dependency `{}` namespace overlay target package `{}` has multiple direct aliases",
                        provider.name, declaration.target_package
                    )
                },
            });
        };
        if provider.name == target.name {
            return Err(ProjectionError {
                message: format!(
                    "Rust dependency `{}` namespace overlay cannot target itself",
                    provider.name
                ),
            });
        }
        overlays.push(NamespaceOverlay {
            provider_name: provider.name.clone(),
            provider_package: provider.package.clone(),
            source_namespace: format!(
                "/deps/{}/{}",
                provider.name.replace('_', "-"),
                source_module
            ),
            target_namespace: format!("/deps/{}", target.name.replace('_', "-")),
        });
    }
    Ok(overlays)
}

fn overlay_module_namespace(module: &str) -> Option<String> {
    let segments = module.split("::").collect::<Vec<_>>();
    if segments.is_empty()
        || segments.iter().any(|segment| {
            segment.is_empty()
                || !segment.bytes().enumerate().all(|(index, byte)| {
                    byte == b'_'
                        || byte.is_ascii_alphanumeric() && (index > 0 || !byte.is_ascii_digit())
                })
        })
    {
        return None;
    }
    Some(
        segments
            .into_iter()
            .map(|segment| segment.replace('_', "-"))
            .collect::<Vec<_>>()
            .join("/"),
    )
}

fn apply_namespace_overlays(
    projected: &mut [ProjectedDependency],
    overlays: &[NamespaceOverlay],
) -> Result<(), ProjectionError> {
    let mut matched = vec![0_usize; overlays.len()];
    let mut moved = BTreeSet::new();
    for (dependency_index, dependency) in projected.iter_mut().enumerate() {
        for (item_index, item) in dependency.items.iter_mut().enumerate() {
            for (index, overlay) in overlays.iter().enumerate().filter(|(_, overlay)| {
                dependency.name == overlay.provider_name
                    && dependency.package == overlay.provider_package
            }) {
                let suffix = item
                    .namespace
                    .strip_prefix(&overlay.source_namespace)
                    .filter(|suffix| suffix.is_empty() || suffix.starts_with('/'));
                if let Some(suffix) = suffix {
                    item.namespace = format!("{}{suffix}", overlay.target_namespace);
                    matched[index] += 1;
                    moved.insert((dependency_index, item_index));
                    break;
                }
            }
        }
    }
    if let Some((_, overlay)) = matched.iter().zip(overlays).find(|(count, _)| **count == 0) {
        return Err(ProjectionError {
            message: format!(
                "Rust dependency `{}` namespace overlay source `{}` contains no projectable items",
                overlay.provider_name, overlay.source_namespace
            ),
        });
    }
    let mut names = BTreeMap::<(&str, &str), (&str, bool)>::new();
    for (dependency_index, dependency) in projected.iter().enumerate() {
        for (item_index, item) in dependency.items.iter().enumerate() {
            let current_moved = moved.contains(&(dependency_index, item_index));
            if let Some((previous, previous_moved)) = names.insert(
                (item.namespace.as_str(), item.name.as_str()),
                (item.rust_path.as_str(), current_moved),
            ) && (previous_moved || current_moved)
            {
                return Err(ProjectionError {
                    message: format!(
                        "dependency namespace overlay collides on `{}::{}` between `{previous}` and `{}`",
                        item.namespace, item.name, item.rust_path
                    ),
                });
            }
        }
    }
    Ok(())
}

fn enforce_transitive_reachability(
    projected: &mut [ProjectedDependency],
    dependencies: &[RustDependency],
    workspace: &Path,
    error_owners_only: bool,
) -> Result<(), ProjectionError> {
    let declared = dependencies
        .iter()
        .flat_map(|dependency| [&dependency.name, &dependency.package])
        .map(|name| name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    let versions = resolved_package_versions(workspace)?;
    for dependency in &mut *projected {
        let mut retained = Vec::new();
        for mut item in std::mem::take(&mut dependency.items) {
            if let Some(owner) = item_undeclared_owner(&item, &declared, error_owners_only) {
                let owner = owner.to_owned();
                dependency.declined.push(DeclinedItem {
                    rust_path: item.rust_path,
                    reason: undeclared_owner_reason(&owner, &versions),
                });
                continue;
            }
            if let ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } = &mut item.kind
            {
                let owner_path = item.rust_path.clone();
                for candidates in [methods, static_methods] {
                    let mut retained_methods = Vec::new();
                    for method in std::mem::take(candidates) {
                        if let Some(owner) =
                            function_undeclared_owner(&method, &declared, error_owners_only)
                        {
                            dependency.declined.push(DeclinedItem {
                                rust_path: format!("{owner_path}::{}", method.name),
                                reason: undeclared_owner_reason(owner, &versions),
                            });
                        } else {
                            retained_methods.push(method);
                        }
                    }
                    *candidates = retained_methods;
                }
            }
            retained.push(item);
        }
        dependency.items = retained;
        dependency
            .declined
            .sort_by(|left, right| left.rust_path.cmp(&right.rust_path));
    }

    for owner in declared {
        let Some(owner_versions) = versions.get(&owner) else {
            continue;
        };
        let referenced = projected.iter().any(|dependency| {
            dependency.package.replace('-', "_") != owner
                && dependency.items.iter().any(|item| {
                    item_foreign_owners(item)
                        .into_iter()
                        .any(|candidate| candidate == owner)
                })
        });
        if referenced && owner_versions.len() > 1 {
            return Err(ProjectionError {
                message: format!(
                    "declared Rust crate `{}` resolves to multiple versions ({}) while a dependency signature uses its types",
                    owner.replace('_', "-"),
                    owner_versions
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            });
        }
    }
    Ok(())
}
fn resolve_cross_dependency_boundary_conversions(projected: &mut [ProjectedDependency]) {
    let capabilities = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match &item.kind {
            ProjectedKind::ForeignType { boundary, .. }
                if boundary != &ProjectedBoundaryCapabilities::default() =>
            {
                Some((item.rust_path.clone(), boundary.clone()))
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();

    for item in projected
        .iter_mut()
        .flat_map(|dependency| &mut dependency.items)
    {
        let ProjectedKind::Enum {
            methods,
            static_methods,
            ..
        } = &mut item.kind
        else {
            continue;
        };
        for function in static_methods.iter_mut().chain(methods.iter_mut()) {
            let Some(operation) = &mut function.enum_operation else {
                continue;
            };
            match operation {
                ProjectedEnumOperation::Construct {
                    unit: false,
                    conversion,
                    payload_rust_type,
                    ..
                } => {
                    let Some(boundary) = capabilities.get(payload_rust_type) else {
                        continue;
                    };
                    let selected = if boundary.bytes_extraction.is_some() {
                        boundary
                            .bytes_constructor
                            .map(|conversion| (ProjectedType::Bytes, conversion))
                    } else if boundary.string_extraction.is_some() {
                        boundary
                            .string_constructor
                            .map(|conversion| (ProjectedType::String, conversion))
                    } else {
                        None
                    };
                    if let Some((ty, selected_conversion)) = selected
                        && let Some(parameter) = function.parameters.first_mut()
                    {
                        parameter.ty = ty;
                        *conversion = selected_conversion;
                    }
                }
                ProjectedEnumOperation::Extract {
                    conversion,
                    payload_rust_type,
                    ..
                } => {
                    let Some(boundary) = capabilities.get(payload_rust_type) else {
                        continue;
                    };
                    let selected = boundary
                        .bytes_extraction
                        .map(|conversion| (ProjectedType::Bytes, conversion))
                        .or_else(|| {
                            boundary
                                .string_extraction
                                .map(|conversion| (ProjectedType::String, conversion))
                        });
                    if let Some((ty, selected_conversion)) = selected {
                        function.result = ProjectedType::Optional(Box::new(ty));
                        *conversion = selected_conversion;
                    }
                }
                _ => {}
            }
        }
    }
}

fn decline_unrepresentable_error_types(projected: &mut [ProjectedDependency]) {
    let projected_error_types = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match item.kind {
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => {
                Some(item.rust_path.clone())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let displayable = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match &item.kind {
            ProjectedKind::ForeignType {
                displayable: true, ..
            }
            | ProjectedKind::Enum {
                displayable: true, ..
            } => Some(item.rust_path.clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    for dependency in projected {
        let mut retained = Vec::with_capacity(dependency.items.len());
        for mut item in std::mem::take(&mut dependency.items) {
            let item_path = item.rust_path.clone();
            match &mut item.kind {
                ProjectedKind::Function(function) => {
                    if let Some(error) =
                        unsupported_projected_error(function, &projected_error_types, &displayable)
                    {
                        dependency.declined.push(DeclinedItem {
                            rust_path: item_path,
                            reason: format!(
                                "projected error `{error}` has no canonical displayable type"
                            ),
                        });
                        continue;
                    }
                }
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => {
                    retain_displayable_error_methods(
                        methods,
                        &projected_error_types,
                        &displayable,
                        &item_path,
                        &mut dependency.declined,
                    );
                    retain_displayable_error_methods(
                        static_methods,
                        &projected_error_types,
                        &displayable,
                        &item_path,
                        &mut dependency.declined,
                    );
                }
                ProjectedKind::Interface(interface) => {
                    interface.methods.retain(|method| {
                        let Some(error) = unsupported_projected_error(
                            &method.function,
                            &projected_error_types,
                            &displayable,
                        ) else {
                            return true;
                        };
                        dependency.declined.push(DeclinedItem {
                            rust_path: format!("{item_path}::{}", method.function.name),
                            reason: format!(
                                "projected error `{error}` has no canonical displayable type"
                            ),
                        });
                        false
                    });
                }
            }
            retained.push(item);
        }
        dependency.items = retained;
    }
}

fn retain_displayable_error_methods(
    methods: &mut Vec<ProjectedFunction>,
    projected_error_types: &BTreeSet<String>,
    displayable: &BTreeSet<String>,
    owner_path: &str,
    declined: &mut Vec<DeclinedItem>,
) {
    methods.retain(|method| {
        let Some(error) = unsupported_projected_error(method, projected_error_types, displayable)
        else {
            return true;
        };
        declined.push(DeclinedItem {
            rust_path: format!("{owner_path}::{}", method.name),
            reason: format!("projected error `{error}` has no canonical displayable type"),
        });
        false
    });
}

fn unsupported_projected_error<'a>(
    function: &'a ProjectedFunction,
    projected_error_types: &BTreeSet<String>,
    displayable: &BTreeSet<String>,
) -> Option<&'a str> {
    function.error.as_deref().filter(|error| {
        let generic_fallback = error == &"Error"
            || rust_path_owner(error)
                .is_some_and(|owner| matches!(owner, "std" | "core" | "alloc"));
        !generic_fallback
            && (!projected_error_types.contains(*error) || !displayable.contains(*error))
    })
}
fn canonicalize_projected_type_names(projected: &mut [ProjectedDependency]) {
    let names = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| {
            matches!(
                item.kind,
                ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
            )
        })
        .map(|item| (item.rust_path.clone(), item.name.clone()))
        .collect::<BTreeMap<_, _>>();
    let error_paths = projected
        .iter()
        .flat_map(|dependency| {
            let dependency_root = format!("/deps/{}", dependency.name.replace('_', "-"));
            let rust_crate = dependency.name.replace('-', "_");
            dependency.items.iter().filter_map(move |item| {
                if !matches!(
                    item.kind,
                    ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
                ) {
                    return None;
                }
                let relative_namespace = item
                    .namespace
                    .strip_prefix(&dependency_root)?
                    .trim_start_matches('/')
                    .replace('/', "::");
                let alias = if relative_namespace.is_empty() {
                    format!("{rust_crate}::{}", item.name)
                } else {
                    format!("{rust_crate}::{relative_namespace}::{}", item.name)
                };
                Some((alias, item.rust_path.clone()))
            })
        })
        .collect::<BTreeMap<_, _>>();
    let mut unique_error_paths = BTreeMap::<String, Option<String>>::new();
    for item in projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| {
            matches!(
                item.kind,
                ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
            )
        })
    {
        unique_error_paths
            .entry(item.name.clone())
            .and_modify(|path| *path = None)
            .or_insert_with(|| Some(item.rust_path.clone()));
    }
    for item in projected
        .iter_mut()
        .flat_map(|dependency| &mut dependency.items)
    {
        let functions: Vec<&mut ProjectedFunction> = match &mut item.kind {
            ProjectedKind::Function(function) => vec![function],
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => methods.iter_mut().chain(static_methods).collect(),
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter_mut()
                .map(|method| &mut method.function)
                .collect(),
        };
        for function in functions {
            for ty in function
                .parameters
                .iter_mut()
                .map(|parameter| &mut parameter.ty)
                .chain(std::iter::once(&mut function.result))
            {
                canonicalize_projected_type_name(ty, &names);
            }
            if let Some(error) = &mut function.error {
                let canonical = error_paths.get(error).cloned().or_else(|| {
                    rust_path_owner(error)?;
                    let name = error.rsplit("::").next()?;
                    unique_error_paths.get(name)?.clone()
                });
                if let Some(canonical) = canonical {
                    error.clone_from(&canonical);
                }
            }
        }
    }
}

fn canonicalize_projected_type_name(ty: &mut ProjectedType, names: &BTreeMap<String, String>) {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            name,
            arguments,
            ..
        } => {
            if let Some(canonical) = names.get(rust_path) {
                name.clone_from(canonical);
            }
            for argument in arguments {
                canonicalize_projected_type_name(argument, names);
            }
        }
        ProjectedType::Optional(inner)
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. } => {
            canonicalize_projected_type_name(inner, names);
        }
        ProjectedType::Mapping { key, value, .. } => {
            canonicalize_projected_type_name(key, names);
            canonicalize_projected_type_name(value, names);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                canonicalize_projected_type_name(item, names);
            }
        }
        _ => {}
    }
}

fn resolved_package_versions(
    workspace: &Path,
) -> Result<BTreeMap<String, BTreeSet<String>>, ProjectionError> {
    let text = fs::read_to_string(workspace.join("Cargo.lock"))
        .map_err(io_error("read dependency projection lockfile"))?;
    let lock = text
        .parse::<toml::Value>()
        .map_err(|error| ProjectionError {
            message: format!("invalid dependency projection lockfile: {error}"),
        })?;
    let mut versions = BTreeMap::<String, BTreeSet<String>>::new();
    for package in lock
        .get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
    {
        let (Some(name), Some(version)) = (
            package.get("name").and_then(toml::Value::as_str),
            package.get("version").and_then(toml::Value::as_str),
        ) else {
            continue;
        };
        versions
            .entry(name.replace('-', "_"))
            .or_default()
            .insert(version.to_owned());
    }
    Ok(versions)
}
fn is_crates_io_lock_source(source: &str) -> bool {
    source == "registry+https://github.com/rust-lang/crates.io-index"
}

#[expect(
    clippy::too_many_lines,
    reason = "bound-owner validation reports the complete resolved dependency conflict"
)]
fn decline_unnameable_bound_owners(
    projected: &mut [ProjectedDependency],
    declared: &[RustDependency],
    workspace: &Path,
) -> Result<(), ProjectionError> {
    let declared = declared
        .iter()
        .map(|dependency| dependency.name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    let text = fs::read_to_string(workspace.join("Cargo.lock"))
        .map_err(io_error("read dependency projection lockfile"))?;
    let lock = text
        .parse::<toml::Value>()
        .map_err(|error| ProjectionError {
            message: format!("invalid dependency projection lockfile: {error}"),
        })?;
    let packages = lock
        .get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|package| {
            Some((
                package.get("name")?.as_str()?.to_owned(),
                package.get("version")?.as_str()?.to_owned(),
                package
                    .get("source")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
            ))
        })
        .collect::<Vec<_>>();
    let reason = |function: &ProjectedFunction| {
        let destination = function.destination_result.as_ref()?;
        destination.bound_roots.iter().find_map(|root| {
            if declared.contains(root)
                || matches!(root.as_str(), "std" | "core" | "alloc" | "self" | "crate")
            {
                return None;
            }
            let matches = packages
                .iter()
                .filter(|(package, _, _)| package.replace('-', "_") == *root)
                .collect::<Vec<_>>();
            match matches.as_slice() {
                [] => Some(format!(
                    "destination-result bound owner `{root}` is not a resolved package"
                )),
                [(_, _, Some(source))] if is_crates_io_lock_source(source) => None,
                [(_, version, _)] => Some(format!(
                    "destination-result bound owner `{root}` at `{version}` is not a nameable registry dependency"
                )),
                _ => Some(format!(
                    "destination-result bound owner `{root}` resolves to multiple package versions"
                )),
            }
        })
    };
    for dependency in projected {
        let mut retained = Vec::new();
        for mut item in std::mem::take(&mut dependency.items) {
            match &mut item.kind {
                ProjectedKind::Function(function) => {
                    if let Some(reason) = reason(function) {
                        dependency.declined.push(DeclinedItem {
                            rust_path: item.rust_path,
                            reason,
                        });
                        continue;
                    }
                }
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => {
                    for method_list in [methods, static_methods] {
                        let mut kept = Vec::new();
                        for method in std::mem::take(method_list) {
                            if let Some(reason) = reason(&method) {
                                dependency.declined.push(DeclinedItem {
                                    rust_path: format!("{}::{}", item.rust_path, method.name),
                                    reason,
                                });
                            } else {
                                kept.push(method);
                            }
                        }
                        *method_list = kept;
                    }
                }
                ProjectedKind::Interface(interface) => {
                    if let Some((method, reason)) = interface.methods.iter().find_map(|method| {
                        reason(&method.function).map(|reason| (&method.function, reason))
                    }) {
                        dependency.declined.push(DeclinedItem {
                            rust_path: format!("{}::{}", item.rust_path, method.name),
                            reason,
                        });
                        continue;
                    }
                }
            }
            retained.push(item);
        }
        dependency.items = retained;
        dependency
            .declined
            .sort_by(|left, right| left.rust_path.cmp(&right.rust_path));
    }
    Ok(())
}

fn projected_bound_dependencies(
    projected: &[ProjectedDependency],
    declared: &[RustDependency],
    workspace: &Path,
) -> Result<Vec<ProjectedBoundDependency>, ProjectionError> {
    let declared = declared
        .iter()
        .map(|dependency| dependency.name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    let required = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .flat_map(|item| match &item.kind {
            ProjectedKind::Function(function) => vec![function],
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => methods.iter().chain(static_methods).collect(),
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter()
                .map(|method| &method.function)
                .collect(),
        })
        .filter_map(|function| function.destination_result.as_ref())
        .flat_map(|destination| &destination.bound_roots)
        .filter(|root| {
            !declared.contains(*root)
                && !matches!(root.as_str(), "std" | "core" | "alloc" | "self" | "crate")
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    if required.is_empty() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(workspace.join("Cargo.lock"))
        .map_err(io_error("read dependency projection lockfile"))?;
    let lock = text
        .parse::<toml::Value>()
        .map_err(|error| ProjectionError {
            message: format!("invalid dependency projection lockfile: {error}"),
        })?;
    let packages = lock
        .get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|package| {
            Some((
                package.get("name")?.as_str()?.to_owned(),
                package.get("version")?.as_str()?.to_owned(),
                package
                    .get("source")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
            ))
        })
        .collect::<Vec<_>>();
    required
        .into_iter()
        .map(|root| {
            let matches = packages
                .iter()
                .filter(|(package, _, _)| package.replace('-', "_") == root)
                .collect::<Vec<_>>();
            let [(package, version, source)] = matches.as_slice() else {
                return Err(ProjectionError {
                    message: format!(
                        "projected result bound root `{root}` is not a unique resolved package"
                    ),
                });
            };
            if !source.as_deref().is_some_and(is_crates_io_lock_source) {
                return Err(ProjectionError {
                    message: format!(
                        "projected result bound root `{root}` is not a nameable registry dependency"
                    ),
                });
            }
            Ok(ProjectedBoundDependency {
                name: root,
                package: package.clone(),
                version: format!("={version}"),
            })
        })
        .collect()
}
fn rewrite_rust_bound_root(bound: &str, package_root: &str, dependency_root: &str) -> String {
    let mut rendered = String::with_capacity(bound.len());
    let mut cursor = 0;
    while let Some(relative_start) = bound[cursor..].find(package_root) {
        let start = cursor + relative_start;
        let end = start + package_root.len();
        let boundary_before = bound[..start].chars().next_back().is_none_or(|character| {
            !(character.is_alphanumeric() || matches!(character, '_' | ':'))
        });
        let path_root = bound[end..].starts_with("::") || (start == 0 && end == bound.len());
        rendered.push_str(&bound[cursor..start]);
        if boundary_before && path_root {
            rendered.push_str(dependency_root);
        } else {
            rendered.push_str(package_root);
        }
        cursor = end;
    }
    rendered.push_str(&bound[cursor..]);
    rendered
}

fn write_workspace_with_bound_dependencies(
    workspace: &Path,
    dependencies: &[RustDependency],
    bound_dependencies: &[ProjectedBoundDependency],
) -> Result<(), ProjectionError> {
    let mut dependencies = dependencies.to_vec();
    dependencies.extend(bound_dependencies.iter().map(|dependency| RustDependency {
        name: dependency.name.clone(),
        package: dependency.package.clone(),
        // This edge exists only to make an already-resolved bound owner nameable. Leaving both
        // feature lists empty prevents the injected direct edge from widening the feature set;
        // the original transitive edges retain every feature active in the resolved lock.
        version: dependency.version.clone(),
        features: Vec::new(),
        default_features: false,
        target: None,
        effects: Vec::new(),
    }));
    write_workspace(workspace, &dependencies)?;
    run_cargo(
        workspace,
        &["fetch", "--offline"],
        CargoToolchain::Default,
        CargoExecution::Host,
    )
}

fn item_undeclared_owner<'a>(
    item: &'a ProjectedItem,
    declared: &BTreeSet<String>,
    error_owners_only: bool,
) -> Option<&'a str> {
    if error_owners_only {
        return match &item.kind {
            ProjectedKind::Function(function) => {
                function_error_undeclared_owner(function, declared)
            }
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter()
                .find_map(|method| function_error_undeclared_owner(&method.function, declared)),
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => None,
        };
    }
    rust_path_owner(&item.rust_path)
        .filter(|owner| !owner_is_reachable(owner, declared))
        .or_else(|| match &item.kind {
            ProjectedKind::Function(function) => {
                function_undeclared_owner(function, declared, false)
            }
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter()
                .find_map(|method| function_undeclared_owner(&method.function, declared, false)),
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => None,
        })
}

fn function_undeclared_owner<'a>(
    function: &'a ProjectedFunction,
    declared: &BTreeSet<String>,
    error_owners_only: bool,
) -> Option<&'a str> {
    if error_owners_only {
        return function_error_undeclared_owner(function, declared);
    }
    function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
        .find_map(|ty| type_undeclared_owner(ty, declared))
        .or_else(|| function_error_undeclared_owner(function, declared))
}
fn function_error_undeclared_owner<'a>(
    function: &'a ProjectedFunction,
    declared: &BTreeSet<String>,
) -> Option<&'a str> {
    function
        .error
        .as_deref()
        .filter(|error| *error != "Error")
        .and_then(rust_path_owner)
        .filter(|owner| !owner_is_reachable(owner, declared))
}

fn type_undeclared_owner<'a>(
    ty: &'a ProjectedType,
    declared: &BTreeSet<String>,
) -> Option<&'a str> {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            base_rust_path,
            arguments,
            ..
        } => arguments
            .iter()
            .find_map(|argument| type_undeclared_owner(argument, declared))
            .or_else(|| {
                rust_path_owner(if base_rust_path.is_empty() {
                    rust_path
                } else {
                    base_rust_path
                })
                .filter(|owner| !owner_is_reachable(owner, declared))
            }),
        ProjectedType::Optional(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. } => type_undeclared_owner(inner, declared),
        ProjectedType::Mapping { key, value, .. } => {
            type_undeclared_owner(key, declared).or_else(|| type_undeclared_owner(value, declared))
        }
        ProjectedType::Tuple(items) => items
            .iter()
            .find_map(|item| type_undeclared_owner(item, declared)),
        _ => None,
    }
}

fn item_foreign_owners(item: &ProjectedItem) -> BTreeSet<String> {
    let mut owners = BTreeSet::new();
    if let Some(owner) = rust_path_owner(&item.rust_path) {
        owners.insert(owner.to_owned());
    }
    let functions = match &item.kind {
        ProjectedKind::Function(function) => vec![function],
        ProjectedKind::ForeignType {
            methods,
            static_methods,
            ..
        }
        | ProjectedKind::Enum {
            methods,
            static_methods,
            ..
        } => methods.iter().chain(static_methods).collect(),
        ProjectedKind::Interface(interface) => interface
            .methods
            .iter()
            .map(|method| &method.function)
            .collect(),
    };
    for function in functions {
        for ty in function
            .parameters
            .iter()
            .map(|parameter| &parameter.ty)
            .chain(std::iter::once(&function.result))
        {
            collect_type_owners(ty, &mut owners);
        }
    }
    owners
}

fn collect_type_owners(ty: &ProjectedType, owners: &mut BTreeSet<String>) {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            arguments,
            ..
        } => {
            if let Some(owner) = rust_path_owner(rust_path) {
                owners.insert(owner.to_owned());
            }
            for argument in arguments {
                collect_type_owners(argument, owners);
            }
        }
        ProjectedType::Optional(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. } => collect_type_owners(inner, owners),
        ProjectedType::Mapping { key, value, .. } => {
            collect_type_owners(key, owners);
            collect_type_owners(value, owners);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                collect_type_owners(item, owners);
            }
        }
        _ => {}
    }
}

fn rust_path_owner(path: &str) -> Option<&str> {
    path.trim_start_matches('<')
        .split("::")
        .next()
        .filter(|owner| {
            !owner.is_empty()
                && owner
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        })
}

fn owner_is_reachable(owner: &str, declared: &BTreeSet<String>) -> bool {
    matches!(owner, "std" | "core" | "alloc") || declared.contains(owner)
}

fn undeclared_owner_reason(owner: &str, versions: &BTreeMap<String, BTreeSet<String>>) -> String {
    let version = versions.get(owner).map_or_else(
        || "unknown".to_owned(),
        |versions| versions.iter().cloned().collect::<Vec<_>>().join(", "),
    );
    format!(
        "type is owned by undeclared crate `{}` at resolved version `{version}`; declare that crate with a unifying version to project its members",
        owner.replace('_', "-")
    )
}

struct ReexportRustdoc {
    document: RustdocCrate,
    providers: BTreeMap<usize, BTreeMap<Id, String>>,
}

struct ReexportRequest {
    package_spec: String,
    package_name: String,
    aliases: BTreeMap<usize, BTreeMap<String, String>>,
    prefixes: BTreeMap<usize, Vec<(String, String)>>,
}

fn generate_rustdoc(
    workspace: &Path,
    package_spec: &str,
    crate_name: &str,
    package_name: &str,
    containment: Containment,
    retain_hidden_definitions: bool,
) -> Result<RustdocCrate, ProjectionError> {
    let mut rustdoc_args = vec![
        "rustdoc",
        "-p",
        package_spec,
        "--lib",
        "--target-dir",
        "target/rustdoc-57",
        "--offline",
        "--frozen",
        "--",
    ];
    rustdoc_args.extend_from_slice(terrane_rust_analysis::RUSTDOC_JSON_ARGS);
    if retain_hidden_definitions {
        rustdoc_args.push("--document-hidden-items");
    }
    run_cargo(
        workspace,
        &rustdoc_args,
        CargoToolchain::RustdocNightly,
        if containment == Containment::Enforced {
            CargoExecution::Contained
        } else {
            CargoExecution::Host
        },
    )?;
    let rustdoc_path = workspace
        .join("target/rustdoc-57/doc")
        .join(format!("{crate_name}.json"));
    let bytes = fs::read(&rustdoc_path).map_err(|error| ProjectionError {
        message: format!(
            "cannot read rustdoc projection `{}`: {error}",
            rustdoc_path.display()
        ),
    })?;
    terrane_rust_analysis::parse_rustdoc(package_name, &bytes, RUSTDOC_TOOLCHAIN).map_err(
        |error| ProjectionError {
            message: error.message,
        },
    )
}

fn resolved_library_package(
    metadata: &serde_json::Value,
    crate_name: &str,
) -> Option<(String, String)> {
    let packages = metadata.get("packages")?.as_array()?;
    let matches = packages
        .iter()
        .filter(|package| {
            package
                .get("targets")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|targets| {
                    targets.iter().any(|target| {
                        target.get("name").and_then(serde_json::Value::as_str)
                            == Some(crate_name)
                            && target
                                .get("kind")
                                .and_then(serde_json::Value::as_array)
                                .is_some_and(|kinds| {
                                    kinds.iter().any(|kind| {
                                        kind.as_str().is_some_and(|kind| {
                                            matches!(
                                                kind,
                                                "lib"
                                                    | "rlib"
                                                    | "dylib"
                                                    | "cdylib"
                                                    | "staticlib"
                                                    | "proc-macro"
                                            )
                                        })
                                    })
                                })
                    })
                })
        })
        .collect::<Vec<_>>();
    let [package] = matches.as_slice() else {
        return None;
    };
    Some((
        package.get("name")?.as_str()?.to_owned(),
        package.get("version")?.as_str()?.to_owned(),
    ))
}

fn prefer_alias(current: &mut String, candidate: &str) {
    let mut paths = BTreeMap::from([(Id(0), current.clone())]);
    terrane_rust_analysis::prefer_public_path(&mut paths, Id(0), candidate.to_owned());
    current.clone_from(
        paths
            .get(&Id(0))
            .expect("the seeded public path remains present"),
    );
}

fn external_reexport_rustdocs(
    workspace: &Path,
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    metadata: &serde_json::Value,
    containment: Containment,
) -> Result<Vec<ReexportRustdoc>, ProjectionError> {
    let mut requests = BTreeMap::<String, ReexportRequest>::new();
    for (dependency_index, (_, document, public_paths)) in rustdocs.iter().enumerate() {
        for (id, public_path) in public_paths {
            let Some(summary) = document.paths.get(id).filter(|summary| summary.crate_id != 0)
            else {
                continue;
            };
            let Some(external) = document.external_crates.get(&summary.crate_id) else {
                continue;
            };
            let Some((package_name, version)) =
                resolved_library_package(metadata, &external.name)
            else {
                continue;
            };
            let request = requests
                .entry(external.name.clone())
                .or_insert_with(|| ReexportRequest {
                    package_spec: format!("{package_name}@{version}"),
                    package_name,
                    aliases: BTreeMap::new(),
                    prefixes: BTreeMap::new(),
                });
            let aliases = request.aliases.entry(dependency_index).or_default();
            aliases
                .entry(summary.path.join("::"))
                .and_modify(|current| prefer_alias(current, public_path))
                .or_insert_with(|| public_path.clone());
        }
        for reexport in terrane_rust_analysis::external_reexports(document) {
            if !reexport.expands_descendants {
                continue;
            }
            let Some((package_name, version)) =
                resolved_library_package(metadata, &reexport.crate_name)
            else {
                continue;
            };
            let request = requests
                .entry(reexport.crate_name)
                .or_insert_with(|| ReexportRequest {
                    package_spec: format!("{package_name}@{version}"),
                    package_name,
                    aliases: BTreeMap::new(),
                    prefixes: BTreeMap::new(),
                });
            request
                .prefixes
                .entry(dependency_index)
                .or_default()
                .push((reexport.canonical_path, reexport.public_path));
        }
    }

    requests
        .into_iter()
        .map(|(crate_name, request)| {
            let document = generate_rustdoc(
                workspace,
                &request.package_spec,
                &crate_name,
                &request.package_name,
                containment,
                true,
            )?;
            let owner_public_paths = rustdoc_public_paths(&document);
            let provider_indices = request
                .aliases
                .keys()
                .chain(request.prefixes.keys())
                .copied()
                .collect::<BTreeSet<_>>();
            let providers = provider_indices
                .into_iter()
                .map(|dependency_index| {
                    let aliases = request.aliases.get(&dependency_index);
                    let prefixes = request.prefixes.get(&dependency_index);
                    let mut public_paths = BTreeMap::new();
                    for (id, summary) in document
                        .paths
                        .iter()
                        .filter(|(_, summary)| summary.crate_id == 0)
                    {
                        let canonical_path = summary.path.join("::");
                        let owner_path = owner_public_paths.get(id).unwrap_or(&canonical_path);
                        if let Some(public_path) = aliases.and_then(|aliases| {
                            aliases
                                .get(&canonical_path)
                                .or_else(|| aliases.get(owner_path))
                        }) {
                            terrane_rust_analysis::prefer_public_path(
                                &mut public_paths,
                                *id,
                                public_path.clone(),
                            );
                        }
                        for (canonical_prefix, public_prefix) in
                            prefixes.into_iter().flatten()
                        {
                            let suffix = if owner_path == canonical_prefix {
                                Some("")
                            } else {
                                owner_path.strip_prefix(&format!("{canonical_prefix}::"))
                            };
                            if let Some(suffix) = suffix {
                                let public_path = if suffix.is_empty() {
                                    public_prefix.clone()
                                } else {
                                    format!("{public_prefix}::{suffix}")
                                };
                                terrane_rust_analysis::prefer_public_path(
                                    &mut public_paths,
                                    *id,
                                    public_path,
                                );
                            }
                        }
                    }
                    (dependency_index, public_paths)
                })
                .filter(|(_, public_paths)| !public_paths.is_empty())
                .collect();
            Ok(ReexportRustdoc {
                document,
                providers,
            })
        })
        .collect()
}

#[cfg(test)]
fn parse_rustdoc(
    dependency: &RustDependency,
    bytes: &[u8],
) -> Result<RustdocCrate, ProjectionError> {
    terrane_rust_analysis::parse_rustdoc(&dependency.package, bytes, RUSTDOC_TOOLCHAIN).map_err(
        |error| ProjectionError {
            message: error.message,
        },
    )
}

fn rustdoc_public_paths(document: &RustdocCrate) -> BTreeMap<Id, String> {
    terrane_rust_analysis::public_paths(document)
}

#[expect(
    clippy::too_many_lines,
    reason = "projected Rust root rewriting exhaustively traverses the closed type model"
)]
fn rewrite_projected_rust_root(ty: &mut ProjectedType, package_root: &str, dependency_root: &str) {
    let rewrite = |path: &str| {
        if path == package_root {
            dependency_root.to_owned()
        } else if let Some(suffix) = path.strip_prefix(&format!("{package_root}::")) {
            format!("{dependency_root}::{suffix}")
        } else {
            path.to_owned()
        }
    };
    match ty {
        ProjectedType::Sequence { rust_path, item } => {
            rewrite_projected_rust_root(item, package_root, dependency_root);
            let constructor = rust_path
                .split_once('<')
                .map_or(rust_path.as_str(), |(constructor, _)| constructor);
            *rust_path = format!("{}<{}>", rewrite(constructor), item.rust_type());
        }
        ProjectedType::Mapping {
            rust_path,
            key,
            value,
            ..
        } => {
            rewrite_projected_rust_root(key, package_root, dependency_root);
            rewrite_projected_rust_root(value, package_root, dependency_root);
            let constructor = rust_path
                .split_once('<')
                .map_or(rust_path.as_str(), |(constructor, _)| constructor);
            *rust_path = format!(
                "{}<{}, {}>",
                rewrite(constructor),
                key.rust_type(),
                value.rust_type()
            );
        }
        ProjectedType::Set {
            rust_path, item, ..
        } => {
            rewrite_projected_rust_root(item, package_root, dependency_root);
            let constructor = rust_path
                .split_once('<')
                .map_or(rust_path.as_str(), |(constructor, _)| constructor);
            *rust_path = format!("{}<{}>", rewrite(constructor), item.rust_type());
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                rewrite_projected_rust_root(item, package_root, dependency_root);
            }
        }
        ProjectedType::AsyncIterationStep(item) | ProjectedType::Optional(item) => {
            rewrite_projected_rust_root(item, package_root, dependency_root);
        }
        ProjectedType::Foreign {
            rust_path,
            base_rust_path,
            arguments,
            ..
        } => {
            for argument in arguments.iter_mut() {
                rewrite_projected_rust_root(argument, package_root, dependency_root);
            }
            *base_rust_path = rewrite(base_rust_path);
            *rust_path = if arguments.is_empty() {
                base_rust_path.clone()
            } else {
                format!(
                    "{base_rust_path}<{}>",
                    arguments
                        .iter()
                        .map(ProjectedType::rust_type)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
        }
        ProjectedType::BoxedInterface {
            rust_path,
            trait_path,
            auto_traits,
            associated_type,
            ..
        } => {
            *trait_path = rewrite(trait_path);
            for auto_trait in auto_traits.iter_mut() {
                *auto_trait = rewrite(auto_trait);
            }
            if let Some(associated) = associated_type.as_mut() {
                rewrite_projected_rust_root(&mut associated.ty, package_root, dependency_root);
            }
            let principal = associated_type.as_ref().map_or_else(
                || trait_path.clone(),
                |associated| {
                    format!(
                        "{trait_path}<{} = {}>",
                        associated.name,
                        associated.ty.rust_type()
                    )
                },
            );
            *rust_path = format!(
                "Box<dyn {}>",
                std::iter::once(principal.as_str())
                    .chain(auto_traits.iter().map(String::as_str))
                    .collect::<Vec<_>>()
                    .join(" + ")
            );
        }
        ProjectedType::Callback {
            parameters, result, ..
        } => {
            for parameter in parameters {
                rewrite_projected_rust_root(parameter, package_root, dependency_root);
            }
            rewrite_projected_rust_root(result, package_root, dependency_root);
        }
        ProjectedType::None
        | ProjectedType::Generic(_)
        | ProjectedType::Bool
        | ProjectedType::Int
        | ProjectedType::FixedInt(_)
        | ProjectedType::RustInt(_)
        | ProjectedType::Float
        | ProjectedType::Float32
        | ProjectedType::Char
        | ProjectedType::String
        | ProjectedType::Bytes
        | ProjectedType::AsyncSinkOutcome
        | ProjectedType::Associated(_) => {}
    }
}

fn project_interface(
    declaration: &rustdoc_types::Trait,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    rust_path: &str,
) -> Result<ProjectedInterface, String> {
    project_interface_inner(declaration, index, paths, rust_path)
}

#[expect(
    clippy::too_many_lines,
    reason = "interface admission keeps inherited and member-level evidence together"
)]
fn project_interface_inner(
    declaration: &rustdoc_types::Trait,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    rust_path: &str,
) -> Result<ProjectedInterface, String> {
    if declaration.is_unsafe {
        return Err("unsafe trait".to_owned());
    }
    if !declaration.generics.params.is_empty() {
        return Err("trait has generic or lifetime parameters".to_owned());
    }
    if !declaration.generics.where_predicates.is_empty() {
        return Err("trait has unsupported where predicates".to_owned());
    }
    let mut send = false;
    let mut sync = false;
    let mut requires_drop = false;
    let mut methods = Vec::new();
    let mut declined_methods = Vec::new();
    let mut associated_type = None;
    let mut supertraits = Vec::new();
    for bound in &declaration.bounds {
        let GenericBound::TraitBound { trait_, .. } = bound else {
            return Err("trait has a non-trait supertrait".to_owned());
        };
        let path = paths
            .get(&trait_.id)
            .map(|summary| summary.path.join("::"))
            .ok_or_else(|| "trait has an unresolved supertrait".to_owned())?;
        match path.as_str() {
            "core::marker::Send" | "std::marker::Send" => send = true,
            "core::marker::Sync" | "std::marker::Sync" => sync = true,
            "core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop" => {
                requires_drop = true;
            }
            _ => {
                let Some(Item {
                    inner: ItemEnum::Trait(supertrait),
                    ..
                }) = index.get(&trait_.id)
                else {
                    return Err(format!("supertrait `{path}` does not resolve to a trait"));
                };
                let supertrait_rust_path =
                    render_resolved_path(trait_, index, paths, &BTreeMap::new())?;
                let projected =
                    project_interface_inner(supertrait, index, paths, &supertrait_rust_path)?;
                send |= projected.send;
                sync |= projected.sync;
                requires_drop |= projected.requires_drop;
                if let Some(inherited_type) = projected.associated_type {
                    match &associated_type {
                        None => associated_type = Some(inherited_type),
                        Some(existing) if existing == &inherited_type => {}
                        Some(_) => {
                            return Err(
                                "trait closure contains more than one associated type".to_owned()
                            );
                        }
                    }
                }
                for method in projected.methods {
                    if !methods.contains(&method) {
                        methods.push(method);
                    }
                }
                for declined in projected.declined_methods {
                    if !declined_methods.contains(&declined) {
                        declined_methods.push(declined);
                    }
                }
                let direct = ProjectedSupertrait {
                    namespace: String::new(),
                    name: trait_
                        .path
                        .rsplit("::")
                        .next()
                        .unwrap_or(&trait_.path)
                        .to_owned(),
                    rust_path: supertrait_rust_path,
                };
                if !supertraits.contains(&direct) {
                    supertraits.push(direct);
                }
                for inherited in projected.supertraits {
                    if !supertraits.contains(&inherited) {
                        supertraits.push(inherited);
                    }
                }
            }
        }
    }
    for id in &declaration.items {
        let Some(item) = index.get(id) else {
            return Err("trait member is missing from rustdoc".to_owned());
        };
        let name = item
            .name
            .as_deref()
            .ok_or_else(|| "trait has an unnamed member".to_owned())?;
        let function = match &item.inner {
            ItemEnum::Function(function) => function,
            ItemEnum::AssocType {
                generics,
                bounds,
                type_,
            } => {
                if associated_type.is_some() {
                    return Err("trait closure contains more than one associated type".to_owned());
                }
                if !generics.params.is_empty() || !generics.where_predicates.is_empty() {
                    return Err(format!("associated type `{name}` is generic"));
                }
                if type_.is_some() {
                    return Err(format!("associated type `{name}` has a default"));
                }
                let bounds = bounds
                    .iter()
                    .map(|bound| render_generic_bound(bound, &[], index, paths, &BTreeMap::new()))
                    .collect::<Result<Vec<_>, _>>()?;
                associated_type = Some(ProjectedAssociatedType {
                    name: name.to_owned(),
                    rust_path: format!("{rust_path}::{name}"),
                    bounds,
                    docs: item.docs.clone(),
                });
                continue;
            }
            _ => return Err(format!("trait member `{name}` is not a receiver method")),
        };
        let provided = function.has_body;
        let member_path = format!("trait::{name}");
        if provided && !function.generics.where_predicates.is_empty() {
            declined_methods.push(DeclinedItem {
                rust_path: member_path,
                reason: "provided method has unsupported where predicates".to_owned(),
            });
            continue;
        }
        if function
            .sig
            .inputs
            .first()
            .is_none_or(|(name, _)| name != "self")
        {
            if provided {
                declined_methods.push(DeclinedItem {
                    rust_path: member_path,
                    reason: "provided associated function is not a receiver method".to_owned(),
                });
                continue;
            }
            return Err(format!("trait member `{name}` is an associated function"));
        }
        let supplied = associated_type
            .as_ref()
            .map(|associated| {
                BTreeMap::from([(
                    format!("Self::{}", associated.name),
                    ProjectedType::Associated(format!("Self::{}", associated.name)),
                )])
            })
            .unwrap_or_default();
        if let Some(parameter) = function.generics.params.first() {
            return Err(format!(
                "trait member `{name}`: open generic `{}`",
                parameter.name
            ));
        }
        let projected = match project_function_with_generics(
            function,
            index,
            paths,
            Some(name),
            &supplied,
            false,
        ) {
            Ok(projected) => projected,
            Err(reason) if provided => {
                declined_methods.push(DeclinedItem {
                    rust_path: member_path,
                    reason,
                });
                continue;
            }
            Err(reason) => return Err(format!("trait member `{name}`: {reason}")),
        };
        if !provided && projected.error.is_some() {
            return Err(format!(
                "trait member `{name}`: required Result-returning methods are deferred"
            ));
        }
        if !provided
            && projected
                .parameters
                .iter()
                .any(|parameter| parameter.borrowed)
        {
            return Err(format!(
                "trait member `{name}`: required borrowed parameters are deferred"
            ));
        }
        debug_assert!(projected.receiver.is_some());
        methods.push(ProjectedInterfaceMethod {
            function: projected,
            provided,
            owner_rust_path: Some(rust_path.to_owned()),
            docs: item.docs.clone(),
        });
    }
    if methods.iter().any(|method| method.function.is_async) && !send {
        return Err(
            "asynchronous projected methods require a `Send` interface so cancellation cleanup can own and detach receiver state"
                .to_owned(),
        );
    }
    if methods.is_empty() {
        return Err("trait has no projectable receiver methods".to_owned());
    }
    Ok(ProjectedInterface {
        methods,
        associated_type,
        supertraits,
        send,
        sync,
        requires_drop,
        declined_methods,
    })
}
fn projected_interface_impl_question(
    item: &ProjectedItem,
    interface: &ProjectedInterface,
) -> crate::projection_oracle::ImplQuestion {
    let associated_bounds = interface.associated_type.as_ref().map(|associated| {
        let bounds = associated.bounds.join(" + ");
        if bounds.is_empty() {
            "'static".to_owned()
        } else {
            format!("'static + {bounds}")
        }
    });
    let generic = associated_bounds
        .as_ref()
        .map_or_else(String::new, |bounds| {
            format!("<TerraneAssociated: {bounds}>")
        });
    let implementation = if associated_bounds.is_some() {
        "TerraneProjectionImpl<TerraneAssociated>"
    } else {
        "TerraneProjectionImpl"
    };
    let mut source = if associated_bounds.is_some() {
        "struct TerraneProjectionImpl<T>(std::marker::PhantomData<fn() -> T>);\n".to_owned()
    } else {
        "struct TerraneProjectionImpl;\n".to_owned()
    };
    if interface.requires_drop {
        writeln!(
            source,
            "impl{generic} Drop for {implementation} {{ fn drop(&mut self) {{}} }}"
        )
        .expect("writing to a string cannot fail");
    }
    let mut traits = interface
        .supertraits
        .iter()
        .map(|supertrait| supertrait.rust_path.as_str())
        .collect::<Vec<_>>();
    traits.push(&item.rust_path);
    for trait_path in traits {
        writeln!(source, "impl{generic} {trait_path} for {implementation} {{")
            .expect("writing to a string cannot fail");
        if let Some(associated) = &interface.associated_type
            && associated
                .rust_path
                .strip_suffix(&format!("::{}", associated.name))
                == Some(trait_path)
        {
            writeln!(source, "type {} = TerraneAssociated;", associated.name)
                .expect("writing to a string cannot fail");
        }
        for method in interface.methods.iter().filter(|method| {
            !method.provided && method.owner_rust_path.as_deref() == Some(trait_path)
        }) {
            let function = &method.function;
            if function.is_async {
                source.push_str("async ");
            }
            write!(source, "fn {}(", function.name).expect("writing to a string cannot fail");
            source.push_str(match function.receiver {
                Some(Receiver::Borrow) => "&self",
                Some(Receiver::MutableBorrow) => "&mut self",
                Some(Receiver::Move) => "self",
                None => "",
            });
            for parameter in &function.parameters {
                let mut ty = parameter.ty.rust_type();
                if parameter.borrowed {
                    ty = format!(
                        "&{}{}",
                        if parameter.mutable_borrow { "mut " } else { "" },
                        ty
                    );
                }
                write!(source, ", {}: {ty}", parameter.name)
                    .expect("writing to a string cannot fail");
            }
            source.push(')');
            let result = function.error.as_ref().map_or_else(
                || function.result.rust_type(),
                |error| format!("Result<{}, {error}>", function.result.rust_type()),
            );
            if result != "()" {
                write!(source, " -> {result}").expect("writing to a string cannot fail");
            }
            source.push_str(" { todo!() }\n");
        }
        source.push_str("}\n");
    }
    source.push_str("fn main() {}\n");
    crate::projection_oracle::ImplQuestion {
        label: item.rust_path.clone(),
        source,
    }
}

struct ProjectedTraitOperation {
    item: ProjectedItem,
    fallback_namespace: String,
}

fn owner_trait_namespace(owner_namespace: &str, owner_name: &str) -> String {
    format!(
        "{owner_namespace}/{}",
        owner_name.to_lowercase().replace('_', "-")
    )
}

fn trait_fallback_namespace(owner_namespace: &str, trait_path: &str) -> String {
    let segments = trait_path
        .split("::")
        .map(|segment| {
            let mut normalized = String::new();
            let mut separator = false;
            for character in segment.chars() {
                if character.is_ascii_alphanumeric() {
                    normalized.push(character.to_ascii_lowercase());
                    separator = false;
                } else if !normalized.is_empty() && !separator {
                    normalized.push('-');
                    separator = true;
                }
            }
            normalized.trim_end_matches('-').to_owned()
        })
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("/");
    format!("{owner_namespace}/trait/{segments}")
}

fn trait_operation_docs(rust_path: &str, docs: Option<&str>) -> String {
    let provenance =
        format!("Projected Rust trait operation `{rust_path}` for this concrete owner.");
    docs.map_or(provenance.clone(), |docs| format!("{provenance}\n\n{docs}"))
}

fn merge_projected_trait_operations(
    items: &mut Vec<ProjectedItem>,
    declined: &mut Vec<DeclinedItem>,
    operations: Vec<ProjectedTraitOperation>,
) {
    let mut occupied = items
        .iter()
        .map(|item| (item.namespace.clone(), item.name.clone()))
        .collect::<BTreeSet<_>>();
    let mut primary_counts = BTreeMap::new();
    for operation in &operations {
        *primary_counts
            .entry((
                operation.item.namespace.clone(),
                operation.item.name.clone(),
            ))
            .or_insert(0usize) += 1;
    }
    let (primary, mut fallback): (Vec<_>, Vec<_>) = operations.into_iter().partition(|operation| {
        let key = (
            operation.item.namespace.clone(),
            operation.item.name.clone(),
        );
        primary_counts.get(&key).copied().unwrap_or_default() == 1 && !occupied.contains(&key)
    });
    for operation in primary {
        occupied.insert((
            operation.item.namespace.clone(),
            operation.item.name.clone(),
        ));
        items.push(operation.item);
    }
    for operation in &mut fallback {
        operation
            .item
            .namespace
            .clone_from(&operation.fallback_namespace);
    }
    let mut fallback_counts = BTreeMap::new();
    for operation in &fallback {
        *fallback_counts
            .entry((
                operation.item.namespace.clone(),
                operation.item.name.clone(),
            ))
            .or_insert(0usize) += 1;
    }
    for operation in fallback {
        let key = (
            operation.item.namespace.clone(),
            operation.item.name.clone(),
        );
        if fallback_counts.get(&key).copied().unwrap_or_default() == 1 && !occupied.contains(&key) {
            occupied.insert(key);
            items.push(operation.item);
        } else {
            declined.push(DeclinedItem {
                rust_path: operation.item.rust_path,
                reason:
                    "trait method conflicts within its concrete owner and trait-qualified namespace"
                        .to_owned(),
            });
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one pass must classify every concrete provided-method candidate before resolving collisions"
)]
fn project_external_provided_trait_methods(
    projected: &mut [ProjectedDependency],
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    canonical_public_paths: &BTreeMap<String, String>,
) {
    for (dependency_index, (_, document, _)) in rustdocs.iter().enumerate() {
        let mut paths = document.paths.clone();
        for summary in paths.values_mut() {
            if let Some(public_path) = canonical_public_paths.get(&summary.path.join("::")) {
                summary.path = public_path.split("::").map(str::to_owned).collect();
            }
        }
        let mut additions = Vec::new();
        for item in document.index.values() {
            let ItemEnum::Impl(implementation) = &item.inner else {
                continue;
            };
            let Some(trait_reference) = implementation.trait_.as_ref() else {
                continue;
            };
            let Some(trait_summary) = document.paths.get(&trait_reference.id) else {
                continue;
            };
            if trait_summary.crate_id == 0 || implementation.provided_trait_methods.is_empty() {
                continue;
            }
            let trait_path = trait_summary.path.join("::");
            let Some((trait_id, trait_document, trait_public_paths, trait_declaration)) = rustdocs
                .iter()
                .find_map(|(_, trait_document, trait_public_paths)| {
                    trait_document.index.iter().find_map(|(id, item)| {
                        let ItemEnum::Trait(declaration) = &item.inner else {
                            return None;
                        };
                        let source_path = trait_document
                            .paths
                            .get(id)
                            .filter(|summary| summary.crate_id == 0)
                            .map(|summary| summary.path.join("::"))?;
                        let public_path = trait_public_paths
                            .get(id)
                            .cloned()
                            .unwrap_or_else(|| source_path.clone());
                        (public_path == trait_path || source_path == trait_path).then_some((
                            id,
                            trait_document,
                            trait_public_paths,
                            declaration,
                        ))
                    })
                })
            else {
                continue;
            };
            if !trait_declaration
                .bounds
                .iter()
                .any(|bound| matches!(bound, GenericBound::Outlives(_)))
            {
                continue;
            }
            if project_interface_inner(
                trait_declaration,
                &trait_document.index,
                &trait_document.paths,
                &trait_path,
            )
            .is_ok()
            {
                continue;
            }
            let Ok(mut owner) = project_type(
                &implementation.for_,
                &document.index,
                &paths,
                &BTreeMap::new(),
            ) else {
                continue;
            };
            let ProjectedType::Foreign {
                rust_path: owner_rust_path,
                ..
            } = &owner
            else {
                continue;
            };
            let owner_rust_path = owner_rust_path.clone();
            let Some(owner_item) = projected[dependency_index]
                .items
                .iter()
                .find(|item| item.rust_path == owner_rust_path)
            else {
                continue;
            };
            if let ProjectedType::Foreign {
                name,
                base_rust_path,
                ..
            } = &mut owner
            {
                name.clone_from(&owner_item.name);
                *base_rust_path = owner_item
                    .rust_path
                    .split_once('<')
                    .map_or_else(|| owner_item.rust_path.clone(), |(base, _)| base.to_owned());
            }
            let owner_namespace = owner_trait_namespace(&owner_item.namespace, &owner_item.name);
            let mut supplied = BTreeMap::from([("Self".to_owned(), owner.clone())]);
            for associated_id in &implementation.items {
                let Some(Item {
                    name: Some(name),
                    inner:
                        ItemEnum::AssocType {
                            type_: Some(type_), ..
                        },
                    ..
                }) = document.index.get(associated_id)
                else {
                    continue;
                };
                if let Ok(projected_type) = project_type(type_, &document.index, &paths, &supplied)
                {
                    supplied.insert(format!("Self::{name}"), projected_type);
                }
            }
            supplied.insert(
                "__terrane_external_associated_bounds".to_owned(),
                ProjectedType::None,
            );
            let namespace = owner_namespace;
            let public_trait_path = trait_public_paths
                .get(trait_id)
                .cloned()
                .unwrap_or_else(|| trait_path.clone())
                .replace('-', "_");
            let mut trait_paths = trait_document.paths.clone();
            for summary in trait_paths.values_mut() {
                if let Some(public_path) = canonical_public_paths.get(&summary.path.join("::")) {
                    summary.path = public_path.split("::").map(str::to_owned).collect();
                }
            }
            for method_name in &implementation.provided_trait_methods {
                let method_rust_path =
                    format!("<{owner_rust_path} as {public_trait_path}>::{method_name}");

                let Some(Item {
                    inner: ItemEnum::Function(function),
                    docs,
                    ..
                }) = trait_declaration.items.iter().find_map(|method_id| {
                    trait_document
                        .index
                        .get(method_id)
                        .filter(|method| method.name.as_deref() == Some(method_name))
                })
                else {
                    projected[dependency_index].declined.push(DeclinedItem {
                        rust_path: method_rust_path,
                        reason: "provided trait method declaration is unavailable".to_owned(),
                    });
                    continue;
                };
                let mut generic_type_parameters =
                    function.generics.params.iter().filter_map(|generic| {
                        matches!(generic.kind, GenericParamDefKind::Type { .. })
                            .then_some(generic.name.as_str())
                    });
                let has_input_selected = generic_type_parameters.clone().any(|generic| {
                    function
                        .sig
                        .inputs
                        .iter()
                        .any(|(_, ty)| type_mentions_generic(ty, generic))
                });
                let has_destination_selected = generic_type_parameters.any(|generic| {
                    function
                        .sig
                        .output
                        .as_ref()
                        .is_some_and(|output| type_mentions_generic(output, generic))
                        && !function
                            .sig
                            .inputs
                            .iter()
                            .any(|(_, ty)| type_mentions_generic(ty, generic))
                });
                if !has_input_selected || !has_destination_selected {
                    projected[dependency_index].declined.push(DeclinedItem {
                        rust_path: method_rust_path,
                        reason: "provided external trait method requires both input- and destination-selected generic parameters".to_owned(),
                    });
                    continue;
                }
                let mut method = match project_function_with_generics(
                    function,
                    &trait_document.index,
                    &trait_paths,
                    Some(method_name),
                    &supplied,
                    false,
                ) {
                    Ok(method) => method,
                    Err(reason) => {
                        projected[dependency_index].declined.push(DeclinedItem {
                            rust_path: method_rust_path,
                            reason,
                        });
                        continue;
                    }
                };
                if method.destination_result.is_none()
                    || !method
                        .generic_parameters
                        .iter()
                        .any(|generic| generic.input_selected)
                {
                    projected[dependency_index].declined.push(DeclinedItem {
                        rust_path: method_rust_path,
                        reason: "provided external trait method did not retain both input- and destination-selected generic parameters".to_owned(),
                    });
                    continue;
                }
                if let Some(receiver) = method.receiver.take() {
                    method.parameters.insert(
                        0,
                        ProjectedParameter {
                            name: "receiver".to_owned(),
                            ty: owner.clone(),
                            generic_parameter: None,
                            generic_interface: None,
                            generic_bounds: Vec::new(),
                            associated_type: None,
                            borrowed: receiver != Receiver::Move,
                            mutable_borrow: receiver == Receiver::MutableBorrow,
                        },
                    );
                }
                additions.push(ProjectedTraitOperation {
                    fallback_namespace: trait_fallback_namespace(&namespace, &trait_path),
                    item: ProjectedItem {
                        namespace: namespace.clone(),
                        name: method_name.clone(),
                        rust_path: method_rust_path.clone(),
                        docs: Some(trait_operation_docs(&method_rust_path, docs.as_deref())),
                        kind: ProjectedKind::Function(method),
                    },
                });
            }
        }
        merge_projected_trait_operations(
            &mut projected[dependency_index].items,
            &mut projected[dependency_index].declined,
            additions,
        );
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one rustdoc item pass records admitted and declined public items together"
)]
fn project_rustdoc(
    dependency: &RustDependency,
    document: &RustdocCrate,
    public_paths: &BTreeMap<Id, String>,
    canonical_public_paths: &BTreeMap<String, String>,
    include_canonical_items: bool,
) -> ProjectedDependency {
    let index = &document.index;
    let original_paths = &document.paths;
    let mut canonical_paths = original_paths.clone();
    for summary in canonical_paths.values_mut() {
        if let Some(public_path) = canonical_public_paths.get(&summary.path.join("::")) {
            summary.path = public_path.split("::").map(str::to_owned).collect();
        }
    }
    let paths = &canonical_paths;
    let mut candidates = BTreeMap::<Id, Vec<String>>::new();
    if include_canonical_items {
        for (id, summary) in paths.iter().filter(|(_, summary)| summary.crate_id == 0) {
            candidates.insert(*id, summary.path.clone());
        }
    }
    for (id, public_path) in public_paths {
        candidates.insert(*id, public_path.split("::").map(str::to_owned).collect());
    }
    let mut items = Vec::new();
    let mut declined = Vec::new();
    let mut projected_trait_items = Vec::new();
    let mut projected_associated_items = Vec::new();
    for (id, path) in candidates {
        let Some(item) = index.get(&id) else {
            if original_paths
                .get(&id)
                .map(|summary| summary.path.join("::"))
                .as_deref()
                .is_some_and(|path| {
                    matches!(
                        path,
                        "core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop"
                    )
                })
            {
                declined.push(DeclinedItem {
                    rust_path: extern_rust_path(dependency, &path.join("::")),
                    reason: "canonical Rust `Drop` is declared with Terrane `consuming destruct`"
                        .to_owned(),
                });
            }
            continue;
        };
        if item.visibility != Visibility::Public {
            continue;
        }
        let Some(name) = path.last().cloned() else {
            continue;
        };
        let namespace = dependency_namespace(dependency, &path[..path.len().saturating_sub(1)]);
        let public_rust_path = public_paths
            .get(&id)
            .cloned()
            .unwrap_or_else(|| path.join("::"));
        let mut rust_path = extern_rust_path(dependency, &public_rust_path);
        let docs = item.docs.clone();
        let projected = match &item.inner {
            ItemEnum::Function(function) => {
                project_function(function, index, paths, Some(&name), true).and_then(
                    |mut projected_function| {
                        let chain_owner = project_chain_owner(
                            dependency,
                            function,
                            &projected_function.result,
                            index,
                            paths,
                            public_paths,
                        );
                        if function
                            .sig
                            .output
                            .as_ref()
                            .is_some_and(type_contains_lifetime_argument)
                            && chain_owner.is_none()
                        {
                            return Err(
                                "lifetime-bearing foreign type cannot cross a projected boundary"
                                    .to_owned(),
                            );
                        }
                        if let Some(chain_owner) = chain_owner {
                            projected_function.chain_role = Some(ChainRole::Root);
                            projected_associated_items.push(chain_owner);
                        }
                        Ok(ProjectedKind::Function(projected_function))
                    },
                )
            }
            ItemEnum::TypeAlias(alias) => (|| {
                let mut alias_generics = BTreeMap::new();
                for parameter in &alias.generics.params {
                    let projected = match &parameter.kind {
                        GenericParamDefKind::Type {
                            default: Some(default),
                            ..
                        } => project_type(default, index, paths, &alias_generics)?,
                        GenericParamDefKind::Type { default: None, .. } => {
                            return Err(format!(
                                "generic type parameter `{}` has no default instantiation",
                                parameter.name
                            ));
                        }
                        GenericParamDefKind::Lifetime { .. } => {
                            return Err(format!(
                                "lifetime parameter `{}` requires non-escaping chain projection",
                                parameter.name
                            ));
                        }
                        GenericParamDefKind::Const { .. } => {
                            return Err(format!(
                                "const parameter `{}` has no projected value identity",
                                parameter.name
                            ));
                        }
                    };
                    alias_generics.insert(parameter.name.clone(), projected);
                }
                if !matches!(
                    project_type(&alias.type_, index, paths, &alias_generics)?,
                    ProjectedType::Foreign { .. }
                ) {
                    return Err("type alias target has no projectable foreign identity".to_owned());
                }
                Ok(ProjectedKind::ForeignType {
                    methods: Vec::new(),
                    static_methods: Vec::new(),
                    boundary: project_boundary_capabilities(&alias.type_, index, paths),
                    displayable: false,
                    cloneable: false,
                    send: false,
                    sync: false,
                })
            })(),
            ItemEnum::Struct(structure) => {
                let mut owner_generics =
                    match default_generic_instantiation(structure, index, paths) {
                        Ok(generics) => generics,
                        Err(reason) => {
                            declined.push(DeclinedItem {
                                rust_path: rust_path.clone(),
                                reason,
                            });
                            continue;
                        }
                    };
                let base_rust_path = rust_path.clone();
                let arguments = structure
                    .generics
                    .params
                    .iter()
                    .filter_map(|parameter| owner_generics.get(&parameter.name).cloned())
                    .collect::<Vec<_>>();
                if !arguments.is_empty() {
                    rust_path = format!(
                        "{base_rust_path}<{}>",
                        arguments
                            .iter()
                            .map(ProjectedType::rust_type)
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                owner_generics.insert(
                    "Self".to_owned(),
                    ProjectedType::Foreign {
                        rust_path: rust_path.clone(),
                        name: name.clone(),
                        base_rust_path,
                        arguments,
                    },
                );
                {
                    let (projected_methods, trait_methods, method_declines) = project_methods(
                        &structure.impls,
                        index,
                        paths,
                        public_paths,
                        &rust_path,
                        &owner_generics,
                        false,
                    );
                    let (mut methods, mut static_methods): (Vec<_>, Vec<_>) = projected_methods
                        .into_iter()
                        .partition(|method| method.receiver.is_some());
                    promote_async_endpoint_methods(&mut methods);
                    promote_async_endpoint_methods(&mut static_methods);
                    let owner_namespace = owner_trait_namespace(&namespace, &name);
                    for (trait_implementation_path, public_trait_path, local_trait, docs, method) in
                        trait_methods
                    {
                        let trait_rust_path = if local_trait {
                            extern_rust_path(dependency, &public_trait_path)
                        } else {
                            public_trait_path.replace('-', "_")
                        };
                        let method_rust_path =
                            format!("<{rust_path} as {trait_rust_path}>::{}", method.name);
                        projected_trait_items.push(ProjectedTraitOperation {
                            fallback_namespace: trait_fallback_namespace(
                                &owner_namespace,
                                &trait_implementation_path,
                            ),
                            item: ProjectedItem {
                                namespace: owner_namespace.clone(),
                                name: method.name.clone(),
                                rust_path: method_rust_path.clone(),
                                docs: Some(trait_operation_docs(
                                    &method_rust_path,
                                    docs.as_deref(),
                                )),
                                kind: ProjectedKind::Function(method),
                            },
                        });
                    }
                    declined.extend(method_declines.into_iter().map(|(name, reason)| {
                        DeclinedItem {
                            rust_path: format!("{rust_path}::{name}"),
                            reason,
                        }
                    }));
                    Ok(ProjectedKind::ForeignType {
                        methods,
                        static_methods,
                        boundary: project_boundary_capabilities(
                            &Type::ResolvedPath(RustdocPath {
                                path: rust_path.clone(),
                                id,
                                args: None,
                            }),
                            index,
                            paths,
                        ),
                        displayable: implements_trait(
                            &structure.impls,
                            index,
                            paths,
                            "core::fmt::Display",
                        ),
                        cloneable: implements_trait(
                            &structure.impls,
                            index,
                            paths,
                            "core::clone::Clone",
                        ),
                        send: false,
                        sync: false,
                    })
                }
            }
            ItemEnum::Enum(enumeration) => {
                if has_type_parameters(&enumeration.generics.params) {
                    Err("type has generic or lifetime parameters".to_owned())
                } else {
                    let enum_type = ProjectedType::Foreign {
                        rust_path: rust_path.clone(),
                        name: name.clone(),
                        base_rust_path: rust_path.clone(),
                        arguments: Vec::new(),
                    };
                    let mut methods = Vec::new();
                    let mut static_methods = Vec::new();
                    let mut variant_names = Vec::new();
                    let mut data_carrying = false;
                    for variant_id in &enumeration.variants {
                        let Some(variant_item) = index.get(variant_id) else {
                            continue;
                        };
                        let Some(variant_name) = variant_item.name.as_deref() else {
                            continue;
                        };
                        let ItemEnum::Variant(variant) = &variant_item.inner else {
                            continue;
                        };
                        variant_names.push(variant_name.to_owned());
                        let projected_payload = match &variant.kind {
                            VariantKind::Plain => None,
                            VariantKind::Tuple(fields) if fields.len() == 1 => {
                                data_carrying = true;
                                let Some(field_id) = fields[0].as_ref() else {
                                    declined.push(DeclinedItem {
                                        rust_path: format!("{rust_path}::{variant_name}"),
                                        reason: "payload enum variant field is stripped".to_owned(),
                                    });
                                    continue;
                                };
                                let Some(Item {
                                    inner: ItemEnum::StructField(field_type),
                                    ..
                                }) = index.get(field_id)
                                else {
                                    declined.push(DeclinedItem {
                                        rust_path: format!("{rust_path}::{variant_name}"),
                                        reason:
                                            "payload enum variant field metadata is unavailable"
                                                .to_owned(),
                                    });
                                    continue;
                                };
                                match project_enum_payload(field_type, index, paths) {
                                    Ok(payload) => Some(payload),
                                    Err(reason) => {
                                        declined.push(DeclinedItem {
                                            rust_path: format!("{rust_path}::{variant_name}"),
                                            reason,
                                        });
                                        continue;
                                    }
                                }
                            }
                            VariantKind::Tuple(_) => {
                                data_carrying = true;
                                declined.push(DeclinedItem {
                                    rust_path: format!("{rust_path}::{variant_name}"),
                                    reason: "payload enum variant has multiple tuple fields"
                                        .to_owned(),
                                });
                                continue;
                            }
                            VariantKind::Struct { .. } => {
                                data_carrying = true;
                                declined.push(DeclinedItem {
                                    rust_path: format!("{rust_path}::{variant_name}"),
                                    reason: "payload enum variant has named fields".to_owned(),
                                });
                                continue;
                            }
                        };
                        let (
                            constructor_type,
                            constructor_conversion,
                            extraction_type,
                            extraction_conversion,
                            payload_rust_type,
                        ) = projected_payload.unwrap_or_else(|| {
                            (
                                ProjectedType::None,
                                ProjectedEnumPayloadConversion::Identity,
                                ProjectedType::None,
                                ProjectedEnumPayloadConversion::Identity,
                                String::new(),
                            )
                        });
                        static_methods.push(ProjectedFunction {
                            name: variant_name.to_owned(),
                            parameters: (constructor_type != ProjectedType::None)
                                .then(|| ProjectedParameter {
                                    name: "value".to_owned(),
                                    ty: constructor_type,
                                    borrowed: false,
                                    mutable_borrow: false,
                                    generic_parameter: None,
                                    generic_bounds: Vec::new(),
                                    generic_interface: None,
                                    associated_type: None,
                                })
                                .into_iter()
                                .collect(),
                            generic_parameters: Vec::new(),
                            result: enum_type.clone(),
                            destination_result: None,
                            error: None,
                            is_async: false,
                            into_future: false,
                            execution_requirements: None,
                            enum_operation: Some(ProjectedEnumOperation::Construct {
                                variant: variant_name.to_owned(),
                                conversion: constructor_conversion,
                                unit: matches!(variant.kind, VariantKind::Plain),
                                payload_rust_type: payload_rust_type.clone(),
                            }),
                            error_optional_depth: 0,
                            chain_role: None,
                            receiver: None,
                        });
                        if extraction_type != ProjectedType::None
                            && !matches!(extraction_type, ProjectedType::Optional(_))
                        {
                            methods.push(ProjectedFunction {
                                name: format!("into-{variant_name}"),
                                parameters: Vec::new(),
                                generic_parameters: Vec::new(),
                                result: ProjectedType::Optional(Box::new(extraction_type)),
                                destination_result: None,
                                error: None,
                                is_async: false,
                                into_future: false,
                                execution_requirements: None,
                                enum_operation: Some(ProjectedEnumOperation::Extract {
                                    variant: variant_name.to_owned(),
                                    conversion: extraction_conversion,
                                    payload_rust_type,
                                }),
                                error_optional_depth: 0,
                                chain_role: None,
                                receiver: Some(Receiver::Move),
                            });
                        }
                    }
                    if data_carrying {
                        methods.push(ProjectedFunction {
                            name: "variant-name".to_owned(),
                            parameters: Vec::new(),
                            generic_parameters: Vec::new(),
                            result: ProjectedType::String,
                            destination_result: None,
                            error: None,
                            is_async: false,
                            into_future: false,
                            execution_requirements: None,
                            enum_operation: Some(ProjectedEnumOperation::VariantName {
                                variants: variant_names,
                                exhaustive: !enumeration.has_stripped_variants
                                    && !item.attrs.iter().any(|attribute| {
                                        matches!(attribute, Attribute::NonExhaustive)
                                    }),
                            }),
                            error_optional_depth: 0,
                            chain_role: None,
                            receiver: Some(Receiver::Borrow),
                        });
                    }
                    Ok(ProjectedKind::Enum {
                        methods,
                        static_methods,
                        send: false,
                        sync: false,
                        displayable: implements_trait(
                            &enumeration.impls,
                            index,
                            paths,
                            "core::fmt::Display",
                        ),
                        data_carrying,
                        comparable: implements_trait(
                            &enumeration.impls,
                            index,
                            paths,
                            "core::cmp::PartialEq",
                        ),
                    })
                }
            }
            ItemEnum::Trait(declaration) => {
                if paths
                    .get(&id)
                    .map(|summary| summary.path.join("::"))
                    .as_deref()
                    .is_some_and(|path| {
                        matches!(
                            path,
                            "core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop"
                        )
                    })
                {
                    Err(
                        "canonical Rust `Drop` is declared with Terrane `consuming destruct`"
                            .to_owned(),
                    )
                } else {
                    project_interface(declaration, index, paths, &rust_path)
                        .map(ProjectedKind::Interface)
                }
            }
            _ => Err("item kind has no Terrane projection".to_owned()),
        };
        match projected {
            Ok(kind) => {
                let docs = if matches!(
                    &kind,
                    ProjectedKind::Function(function) if function.chain_role == Some(ChainRole::Root)
                ) {
                    Some(match docs {
                        Some(docs) => format!(
                            "{docs}\n\nChain-only: this value must terminate within one expression."
                        ),
                        None => "Chain-only: this value must terminate within one expression."
                            .to_owned(),
                    })
                } else {
                    docs
                };
                items.push(ProjectedItem {
                    namespace,
                    name,
                    rust_path,
                    docs,
                    kind,
                });
            }
            Err(reason) => declined.push(DeclinedItem { rust_path, reason }),
        }
    }
    projected_associated_items.sort_by(|left, right| {
        (&left.namespace, &left.name, &left.rust_path).cmp(&(
            &right.namespace,
            &right.name,
            &right.rust_path,
        ))
    });
    let mut associated_index = 0;
    while associated_index < projected_associated_items.len() {
        let first = associated_index;
        let key = (
            projected_associated_items[first].namespace.clone(),
            projected_associated_items[first].name.clone(),
        );
        while associated_index < projected_associated_items.len()
            && projected_associated_items[associated_index].namespace == key.0
            && projected_associated_items[associated_index].name == key.1
        {
            associated_index += 1;
        }
        if associated_index - first == 1
            && !items
                .iter()
                .any(|item| item.namespace == key.0 && item.name == key.1)
        {
            items.push(projected_associated_items[first].clone());
        } else {
            declined.extend(
                projected_associated_items[first..associated_index]
                    .iter()
                    .map(|item| DeclinedItem {
                        rust_path: item.rust_path.clone(),
                        reason: "multiple receiver-free associated functions with the same projected name"
                            .to_owned(),
                    }),
            );
        }
    }
    merge_projected_trait_operations(&mut items, &mut declined, projected_trait_items);
    let projected_interfaces = items
        .iter()
        .filter(|item| matches!(item.kind, ProjectedKind::Interface(_)))
        .map(|item| item.rust_path.clone())
        .collect::<BTreeSet<_>>();
    let mut retained_items = Vec::with_capacity(items.len());
    for item in items {
        let declined_bound = match &item.kind {
            ProjectedKind::Function(function) => function
                .parameters
                .iter()
                .filter_map(|parameter| parameter.generic_interface.as_ref())
                .find(|bound| !projected_interfaces.contains(*bound))
                .cloned(),
            _ => None,
        };
        if let Some(bound) = declined_bound {
            declined.push(DeclinedItem {
                rust_path: item.rust_path,
                reason: format!("generic input references declined interface `{bound}`"),
            });
        } else {
            retained_items.push(item);
        }
    }
    let mut items = retained_items;
    let package_root = dependency.package.replace('-', "_");
    let dependency_root = dependency.name.replace('-', "_");
    let normalize = |function: &mut ProjectedFunction| {
        for parameter in &mut function.parameters {
            rewrite_projected_rust_root(&mut parameter.ty, &package_root, &dependency_root);
            if let Some(associated) = &mut parameter.associated_type {
                rewrite_projected_rust_root(&mut associated.ty, &package_root, &dependency_root);
            }
            if let Some(interface) = &mut parameter.generic_interface {
                *interface = rewrite_rust_bound_root(interface, &package_root, &dependency_root);
            }
        }
        rewrite_projected_rust_root(&mut function.result, &package_root, &dependency_root);
        if let Some(error) = &mut function.error {
            *error = rewrite_rust_bound_root(error, &package_root, &dependency_root);
        }
        if let Some(destination) = &mut function.destination_result {
            for bound in &mut destination.rust_bounds {
                *bound = rewrite_rust_bound_root(bound, &package_root, &dependency_root);
            }
            for root in &mut destination.bound_roots {
                if root == &package_root {
                    root.clone_from(&dependency_root);
                }
            }
        }
    };
    let interface_identities = items
        .iter()
        .filter(|item| matches!(item.kind, ProjectedKind::Interface(_)))
        .map(|item| {
            (
                item.rust_path.clone(),
                (item.namespace.clone(), item.name.clone()),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for item in &mut items {
        match &mut item.kind {
            ProjectedKind::Function(function) => normalize(function),
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => {
                for method in methods.iter_mut().chain(static_methods) {
                    normalize(method);
                }
            }
            ProjectedKind::Interface(interface) => {
                for method in &mut interface.methods {
                    normalize(&mut method.function);
                    if let Some(owner) = &mut method.owner_rust_path {
                        *owner = rewrite_rust_bound_root(owner, &package_root, &dependency_root);
                    }
                }
                if let Some(associated) = &mut interface.associated_type {
                    associated.rust_path = rewrite_rust_bound_root(
                        &associated.rust_path,
                        &package_root,
                        &dependency_root,
                    );
                    for bound in &mut associated.bounds {
                        *bound = rewrite_rust_bound_root(bound, &package_root, &dependency_root);
                    }
                }
                for supertrait in &mut interface.supertraits {
                    if let Some((namespace, name)) = interface_identities.get(&supertrait.rust_path)
                    {
                        supertrait.namespace.clone_from(namespace);
                        supertrait.name.clone_from(name);
                    }
                    supertrait.rust_path = rewrite_rust_bound_root(
                        &supertrait.rust_path,
                        &package_root,
                        &dependency_root,
                    );
                }
            }
        }
    }
    items.sort_by(|left, right| {
        (&left.namespace, &left.name, &left.rust_path).cmp(&(
            &right.namespace,
            &right.name,
            &right.rust_path,
        ))
    });
    declined.sort_by(|left, right| {
        (&left.rust_path, &left.reason).cmp(&(&right.rust_path, &right.reason))
    });
    ProjectedDependency {
        name: dependency.name.clone(),
        package: dependency.package.clone(),
        version: document
            .crate_version
            .clone()
            .unwrap_or_else(|| dependency.version.clone()),
        items,
        declined,
    }
}

fn default_generic_instantiation(
    structure: &Struct,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<BTreeMap<String, ProjectedType>, String> {
    let mut substitutions = BTreeMap::new();
    for parameter in &structure.generics.params {
        match &parameter.kind {
            GenericParamDefKind::Type {
                default: Some(default),
                ..
            } => {
                let projected = project_type(default, index, paths, &substitutions)?;
                substitutions.insert(parameter.name.clone(), projected);
            }
            GenericParamDefKind::Type { default: None, .. } => {
                return Err(format!(
                    "generic type parameter `{}` has no default instantiation",
                    parameter.name
                ));
            }
            GenericParamDefKind::Lifetime { .. } => {
                return Err(format!(
                    "lifetime parameter `{}` requires non-escaping chain projection",
                    parameter.name
                ));
            }
            GenericParamDefKind::Const { .. } => {
                return Err(format!(
                    "const parameter `{}` has no projected value identity",
                    parameter.name
                ));
            }
        }
    }
    Ok(substitutions)
}

fn has_type_parameters(parameters: &[rustdoc_types::GenericParamDef]) -> bool {
    !parameters.is_empty()
}

fn extern_rust_path(dependency: &RustDependency, path: &str) -> String {
    let mut segments = path.split("::");
    let _package_root = segments.next();
    std::iter::once(dependency.name.replace('-', "_"))
        .chain(segments.map(str::to_owned))
        .collect::<Vec<_>>()
        .join("::")
}

type ProjectedMethods = (
    Vec<ProjectedFunction>,
    Vec<(String, String, bool, Option<String>, ProjectedFunction)>,
    Vec<(String, String)>,
);

#[expect(
    clippy::too_many_lines,
    reason = "rustdoc impl traversal keeps trait and inherent decisions in one deterministic pass"
)]
fn project_methods(
    impl_ids: &[Id],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    owner_rust_path: &str,
    owner_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
) -> ProjectedMethods {
    let mut candidates = Vec::new();
    let mut trait_methods = Vec::new();
    let mut declined = Vec::new();
    for impl_id in impl_ids {
        let Some(Item {
            inner: ItemEnum::Impl(implementation),
            ..
        }) = index.get(impl_id)
        else {
            continue;
        };
        if implementation.is_negative
            || implementation.is_synthetic
            || implementation.blanket_impl.is_some()
        {
            continue;
        }
        let inherent = implementation.trait_.is_none();
        let mut implementation_generics = owner_generics.clone();
        for item_id in &implementation.items {
            let Some(Item {
                name: Some(name),
                inner:
                    ItemEnum::AssocType {
                        type_: Some(type_), ..
                    },
                ..
            }) = index.get(item_id)
            else {
                continue;
            };
            if let Ok(projected) = project_type(type_, index, paths, owner_generics) {
                implementation_generics.insert(format!("Self::{name}"), projected);
            }
        }
        for method_id in &implementation.items {
            let Some(item) = index.get(method_id) else {
                continue;
            };
            if inherent && item.visibility != Visibility::Public {
                continue;
            }
            let Some(name) = item.name.as_deref() else {
                continue;
            };
            let ItemEnum::Function(function) = &item.inner else {
                declined.push((
                    name.to_owned(),
                    "item kind has no Terrane method projection".to_owned(),
                ));
                continue;
            };
            if !inherent {
                let Some(trait_) = implementation.trait_.as_ref() else {
                    continue;
                };
                let Some((trait_path, local_trait)) = implementation_trait_path(trait_, paths)
                else {
                    declined.push((
                        name.to_owned(),
                        "trait method has no canonical Rust trait path".to_owned(),
                    ));
                    continue;
                };
                let public_trait_path = public_paths
                    .get(&trait_.id)
                    .cloned()
                    .unwrap_or_else(|| trait_path.clone());
                let rendered_trait =
                    render_resolved_path(trait_, index, paths, &implementation_generics)
                        .unwrap_or_else(|_| trait_path.clone());
                let trait_implementation_path = rendered_trait.find('<').map_or_else(
                    || trait_path.clone(),
                    |start| format!("{trait_path}{}", &rendered_trait[start..]),
                );
                match project_function_with_generics(
                    function,
                    index,
                    paths,
                    Some(name),
                    &implementation_generics,
                    allow_lifetime_output,
                ) {
                    Ok(mut method) => {
                        if let Some(receiver) = method.receiver.take() {
                            method.parameters.insert(
                                0,
                                ProjectedParameter {
                                    name: "receiver".to_owned(),
                                    ty: implementation_generics
                                        .get("Self")
                                        .cloned()
                                        .unwrap_or_else(|| {
                                            let base_rust_path = owner_rust_path
                                                .split_once('<')
                                                .map_or(owner_rust_path, |(base, _)| base);
                                            ProjectedType::Foreign {
                                                rust_path: owner_rust_path.to_owned(),
                                                name: base_rust_path
                                                    .rsplit("::")
                                                    .next()
                                                    .unwrap_or(base_rust_path)
                                                    .to_owned(),
                                                base_rust_path: base_rust_path.to_owned(),
                                                arguments: Vec::new(),
                                            }
                                        }),
                                    generic_parameter: None,
                                    generic_interface: None,
                                    generic_bounds: Vec::new(),
                                    associated_type: None,
                                    borrowed: receiver != Receiver::Move,
                                    mutable_borrow: receiver == Receiver::MutableBorrow,
                                },
                            );
                        }
                        trait_methods.push((
                            trait_implementation_path,
                            public_trait_path,
                            local_trait,
                            item.docs.clone(),
                            method,
                        ));
                    }
                    Err(reason) => declined.push((name.to_owned(), reason)),
                }
                continue;
            }
            match project_function_with_generics(
                function,
                index,
                paths,
                Some(name),
                &implementation_generics,
                allow_lifetime_output,
            ) {
                Ok(method) => candidates.push(method),
                Err(reason) => declined.push((name.to_owned(), reason)),
            }
        }
    }
    let mut methods = Vec::new();
    while let Some(method) = candidates.pop() {
        let name = method.name.clone();
        let mut matching = vec![method];
        let mut candidate_index = 0;
        while candidate_index < candidates.len() {
            if candidates[candidate_index].name == name {
                matching.push(candidates.swap_remove(candidate_index));
            } else {
                candidate_index += 1;
            }
        }
        if matching.len() == 1 {
            methods.push(matching.pop().expect("one matching method"));
            continue;
        }
        declined.extend(matching.into_iter().map(|method| {
            (
                method.name,
                "multiple inherent methods with the same name are not projectable".to_owned(),
            )
        }));
    }
    methods.sort_by(|left, right| left.name.cmp(&right.name));
    (methods, trait_methods, declined)
}
fn project_chain_owner(
    dependency: &RustDependency,
    function: &Function,
    result: &ProjectedType,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
) -> Option<ProjectedItem> {
    let output = function.sig.output.as_ref()?;
    let Type::ResolvedPath(path) = output else {
        return None;
    };
    let Item {
        inner: ItemEnum::Struct(structure),
        ..
    } = index.get(&path.id)?
    else {
        return None;
    };
    if !structure
        .generics
        .params
        .iter()
        .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }))
    {
        return None;
    }
    let ProjectedType::Foreign {
        rust_path,
        name,
        base_rust_path: _,
        arguments,
    } = result
    else {
        return None;
    };
    let mut owner_generics = BTreeMap::new();
    owner_generics.insert("Self".to_owned(), result.clone());
    for (parameter, argument) in structure
        .generics
        .params
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .zip(arguments)
    {
        owner_generics.insert(parameter.name.clone(), argument.clone());
    }
    let (mut methods, _, _) = project_methods(
        &structure.impls,
        index,
        paths,
        public_paths,
        rust_path,
        &owner_generics,
        true,
    );
    methods.retain_mut(|method| {
        if method.receiver.is_none() {
            return false;
        }
        method.chain_role = Some(if method.result == *result {
            ChainRole::Continue
        } else {
            ChainRole::Terminal
        });
        true
    });
    if methods
        .iter()
        .all(|method| method.chain_role != Some(ChainRole::Terminal))
    {
        return None;
    }
    let source_path = paths.get(&path.id)?.path.clone();
    let namespace = dependency_namespace(
        dependency,
        &source_path[..source_path.len().saturating_sub(1)],
    );
    Some(ProjectedItem {
        namespace,
        name: name.clone(),
        rust_path: rust_path.clone(),
        docs: Some("chain-only; value must terminate within one expression".to_owned()),
        kind: ProjectedKind::ForeignType {
            methods,
            static_methods: Vec::new(),
            boundary: ProjectedBoundaryCapabilities::default(),
            displayable: false,
            cloneable: false,
            send: false,
            sync: false,
        },
    })
}

fn project_function_with_generics(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    method_name: Option<&str>,
    supplied_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    project_function_inner(
        function,
        index,
        paths,
        method_name,
        supplied_generics,
        allow_lifetime_output,
    )
}

fn project_function(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    method_name: Option<&str>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    project_function_inner(
        function,
        index,
        paths,
        method_name,
        &BTreeMap::new(),
        allow_lifetime_output,
    )
}

fn open_chain_generics(
    function: &Function,
    index: &HashMap<Id, Item>,
) -> Option<BTreeMap<String, ProjectedType>> {
    let output = function.sig.output.as_ref()?;
    if function
        .sig
        .inputs
        .first()
        .is_some_and(|(name, _)| name == "self")
    {
        return None;
    }
    if !matches!(output, Type::ResolvedPath(_)) || !type_contains_lifetime_argument(output) {
        return None;
    }
    if !type_arguments(output).into_iter().any(|argument| {
        matches!(
            argument,
            Type::QualifiedPath { self_type, .. }
                if matches!(self_type.as_ref(), Type::Generic(_))
        )
    }) {
        return None;
    }
    let has_terminal_associated_shape = function.generics.params.iter().any(|parameter| {
        generic_bounds(parameter, function).iter().any(|bound| {
            let GenericBound::TraitBound { trait_, .. } = bound else {
                return false;
            };
            let Some(Item {
                inner: ItemEnum::Trait(declaration),
                ..
            }) = index.get(&trait_.id)
            else {
                return false;
            };
            let associated = declaration
                .items
                .iter()
                .filter_map(|id| index.get(id).and_then(|item| item.name.as_deref()))
                .collect::<BTreeSet<_>>();
            ["Arguments", "QueryResult", "Row"]
                .into_iter()
                .all(|name| associated.contains(name))
        })
    });
    if !has_terminal_associated_shape {
        return None;
    }
    let generics = function
        .generics
        .params
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .map(|parameter| {
            (
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            )
        })
        .collect::<BTreeMap<_, _>>();
    (!generics.is_empty()).then_some(generics)
}

fn open_chain_result(
    output: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Option<ProjectedType> {
    let Type::ResolvedPath(path) = output else {
        return None;
    };
    let Item {
        inner: ItemEnum::Struct(structure),
        ..
    } = index.get(&path.id)?
    else {
        return None;
    };
    if !structure
        .generics
        .params
        .iter()
        .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }))
    {
        return None;
    }
    let base_rust_path = resolved_path_name(path, paths);
    let arguments = structure
        .generics
        .params
        .iter()
        .filter_map(|parameter| match parameter.kind {
            GenericParamDefKind::Type { .. } => generics.get(&parameter.name).cloned(),
            _ => None,
        })
        .collect::<Vec<_>>();
    let rust_identity = format!(
        "{}<{}>",
        base_rust_path,
        arguments
            .iter()
            .map(ProjectedType::rust_type)
            .collect::<Vec<_>>()
            .join(", ")
    );
    Some(ProjectedType::Foreign {
        rust_path: base_rust_path.clone(),
        name: instantiated_type_name(
            base_rust_path.rsplit("::").next().unwrap_or("chain"),
            &rust_identity,
        ),
        base_rust_path,
        arguments,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "function projection keeps generic selection and exact parameter contracts together"
)]
fn project_function_inner(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    method_name: Option<&str>,
    supplied_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    if function.header.is_unsafe {
        return Err("unsafe function".to_owned());
    }
    let open_chain = open_chain_generics(function, index);
    let (generic_types, destination_result) = if let Some(types) = open_chain.clone() {
        (types, None)
    } else {
        generic_monomorphisations(function, index, paths, supplied_generics)
            .map_err(|reason| format!("generic selection: {reason}"))?
    };
    if destination_result.is_some()
        && function.sig.output.as_ref().is_some_and(|output| {
            render_rust_type(output, index, paths, &generic_types)
                .is_ok_and(|rendered| rendered.contains('&'))
        })
    {
        return Err("borrowed result values cannot cross a projected boundary".to_owned());
    }
    let mut parameters = Vec::new();
    let mut receiver = None;
    for (parameter_index, (name, ty)) in function.sig.inputs.iter().enumerate() {
        if name == "self" {
            receiver = Some(receiver_kind(ty)?);
            continue;
        }
        if !matches!(ty, Type::BorrowedRef { .. }) && type_contains_borrowed_ref(ty) {
            return Err("nested borrowed parameter cannot cross a projected boundary".to_owned());
        }
        if type_contains_lifetime_argument(ty) {
            return Err(
                "lifetime-bearing foreign type cannot cross a projected boundary".to_owned(),
            );
        }
        let (projected_type, impl_trait_parameter) = if let Some(bounds) = impl_trait_bounds(ty) {
            if let Some(projected) = structural_impl_trait_input(bounds, paths) {
                (projected, Some(format!("TerraneImpl{parameter_index}")))
            } else {
                let projectable = projectable_interface_bound(bounds, index, paths)?;
                let Some(Item {
                    inner: ItemEnum::Trait(declaration),
                    ..
                }) = index.get(&projectable.id)
                else {
                    return Err("`impl Trait` input has an unresolved trait bound".to_owned());
                };
                if project_interface(declaration, index, paths, &projectable.path).is_err() {
                    return Err(
                        "`impl Trait` input bound is not a projectable interface".to_owned()
                    );
                }
                let rust_path = render_resolved_path(projectable, index, paths, &generic_types)?;
                (
                    ProjectedType::Foreign {
                        name: projectable
                            .path
                            .rsplit("::")
                            .next()
                            .unwrap_or(&projectable.path)
                            .to_owned(),
                        base_rust_path: rust_path.clone(),
                        rust_path,
                        arguments: Vec::new(),
                    },
                    Some(format!("TerraneImpl{parameter_index}")),
                )
            }
        } else {
            (project_type(ty, index, paths, &generic_types)?, None)
        };
        let (borrowed, mutable_borrow) = match ty {
            Type::BorrowedRef { is_mutable, .. } => (true, *is_mutable),
            _ => (false, false),
        };
        if mutable_borrow && !matches!(projected_type, ProjectedType::Foreign { .. }) {
            return Err("mutable borrowed primitive parameters are not representable".to_owned());
        }
        let boxed_adapter = match &projected_type {
            ProjectedType::BoxedInterface {
                trait_path,
                auto_traits,
                associated_type,
                ..
            } => {
                let principal = associated_type.as_ref().map_or_else(
                    || trait_path.clone(),
                    |associated| {
                        format!(
                            "{trait_path}<{} = {}>",
                            associated.name,
                            associated.ty.rust_type()
                        )
                    },
                );
                Some((
                    format!("TerraneBoxed{parameter_index}"),
                    std::iter::once(principal)
                        .chain(auto_traits.iter().cloned())
                        .chain(std::iter::once("'static".to_owned()))
                        .collect::<Vec<_>>(),
                ))
            }
            _ => None,
        };
        let generic_parameter = impl_trait_parameter
            .or_else(|| boxed_adapter.as_ref().map(|(name, _)| name.clone()))
            .or_else(|| {
                function.generics.params.iter().find_map(|parameter| {
                    generic_types.get(&parameter.name).and_then(|projected| {
                        ((matches!(projected, ProjectedType::Foreign { .. })
                            || matches!(projected, ProjectedType::Generic(_))
                                && matches!(projected_type, ProjectedType::Generic(_)))
                            && type_mentions_generic(ty, &parameter.name))
                        .then(|| parameter.name.clone())
                    })
                })
            });
        let rendered_generic_bounds = if let Some((_, bounds)) = &boxed_adapter {
            bounds.clone()
        } else if let Some(name) = &generic_parameter {
            if name.starts_with("TerraneImpl") {
                impl_trait_bounds(ty)
                    .map(|bounds| {
                        bounds
                            .iter()
                            .map(|bound| {
                                render_generic_bound(
                                    bound,
                                    &function.generics.params,
                                    index,
                                    paths,
                                    &generic_types,
                                )
                            })
                            .collect()
                    })
                    .transpose()?
                    .unwrap_or_default()
            } else {
                let parameter = function
                    .generics
                    .params
                    .iter()
                    .find(|parameter| parameter.name == *name)
                    .expect("selected generic parameter must be declared");
                render_generic_bounds(parameter, function, index, paths, &generic_types)?
            }
        } else {
            Vec::new()
        };
        let associated_type = if let ProjectedType::BoxedInterface {
            associated_type, ..
        } = &projected_type
        {
            associated_type.clone()
        } else if let Some(name) = &generic_parameter {
            let bounds = if name.starts_with("TerraneImpl") {
                impl_trait_bounds(ty).map(<[_]>::to_vec)
            } else {
                function
                    .generics
                    .params
                    .iter()
                    .find(|parameter| parameter.name == *name)
                    .map(|parameter| generic_bounds(parameter, function))
            };
            bounds
                .as_deref()
                .and_then(|bounds| projectable_interface_bound(bounds, index, paths).ok())
                .map(|trait_| project_associated_binding(trait_, index, paths, &generic_types))
                .transpose()?
                .flatten()
        } else {
            None
        };
        let generic_interface = if let Some(name) = &generic_parameter {
            let bounds = if name.starts_with("TerraneImpl") {
                impl_trait_bounds(ty).map(<[_]>::to_vec)
            } else {
                function
                    .generics
                    .params
                    .iter()
                    .find(|parameter| parameter.name == *name)
                    .map(|parameter| generic_bounds(parameter, function))
            };
            bounds
                .as_deref()
                .and_then(|bounds| projectable_interface_bound(bounds, index, paths).ok())
                .and_then(|trait_| {
                    let Item {
                        inner: ItemEnum::Trait(declaration),
                        ..
                    } = index.get(&trait_.id)?
                    else {
                        return None;
                    };
                    project_interface(declaration, index, paths, &trait_.path)
                        .is_ok()
                        .then(|| paths.get(&trait_.id).map(|summary| summary.path.join("::")))
                        .flatten()
                })
        } else {
            None
        };
        let borrowed = borrowed
            || (generic_parameter
                .as_ref()
                .is_some_and(|name| name.starts_with("TerraneImpl"))
                && generic_interface.is_none()
                && matches!(projected_type, ProjectedType::String | ProjectedType::Bytes));
        parameters.push(ProjectedParameter {
            name: safe_parameter_name(name),
            ty: projected_type,
            borrowed,
            mutable_borrow,
            generic_parameter,
            generic_interface,
            generic_bounds: rendered_generic_bounds,
            associated_type,
        });
    }
    let (effective_output, returns_future, into_future, output_generic_types) =
        match function.sig.output.as_ref() {
            Some(output)
                if resolved_name(output, paths).as_deref()
                    == Some("futures_core::future::BoxFuture") =>
            {
                (
                    type_arguments(output).into_iter().next_back().cloned(),
                    true,
                    false,
                    generic_types.clone(),
                )
            }
            Some(output) => match concrete_into_future_output(output, index, paths, &generic_types)
            {
                Some((into_future_output, into_future_generics)) => {
                    (Some(into_future_output), true, true, into_future_generics)
                }
                None => (Some(output.clone()), false, false, generic_types.clone()),
            },
            None => (None, false, false, generic_types.clone()),
        };
    if (function.header.is_async || returns_future)
        && effective_output
            .as_ref()
            .is_some_and(type_contains_borrowed_ref)
    {
        return Err("borrowed result values cannot cross a projected boundary".to_owned());
    }
    if !allow_lifetime_output
        && open_chain.is_none()
        && effective_output
            .as_ref()
            .is_some_and(type_contains_lifetime_argument)
    {
        return Err("lifetime-bearing foreign type cannot cross a projected boundary".to_owned());
    }
    let mut error = None;
    let mut error_optional_depth = 0;
    let result = if let Some(output) = effective_output.as_ref() {
        if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Result") || name == "Result")
        {
            let arguments = type_arguments(output);
            let value = arguments
                .first()
                .ok_or_else(|| "Result has no value type".to_owned())?;
            let projected = project_type(value, index, paths, &output_generic_types)
                .map_err(|reason| format!("projected result value: {reason}"))?;
            if projected.rust_type().contains('&') {
                return Err("borrowed result values cannot cross a projected boundary".to_owned());
            }
            error = arguments
                .get(1)
                .and_then(|ty| projected_error_name(ty, output, paths))
                .or_else(|| Some("Error".to_owned()));
            projected
        } else if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Option") || name == "Option")
        {
            let arguments = type_arguments(output);
            let value = arguments
                .first()
                .ok_or_else(|| "Option has no value type".to_owned())?;
            if resolved_name(value, paths)
                .is_some_and(|name| name.ends_with("::Result") || name == "Result")
            {
                let arguments = type_arguments(value);
                let success = arguments
                    .first()
                    .ok_or_else(|| "nested Result has no value type".to_owned())?;
                let projected = project_type(success, index, paths, &output_generic_types)
                    .map_err(|reason| format!("projected nested result value: {reason}"))?;
                if projected.rust_type().contains('&') {
                    return Err(
                        "borrowed nested result values cannot cross a projected boundary"
                            .to_owned(),
                    );
                }
                error = arguments
                    .get(1)
                    .and_then(|ty| projected_error_name(ty, value, paths))
                    .or_else(|| Some("Error".to_owned()));
                error_optional_depth = 1;
                ProjectedType::Optional(Box::new(projected))
            } else {
                ProjectedType::Optional(Box::new(
                    project_type(value, index, paths, &output_generic_types)
                        .map_err(|reason| format!("projected optional value: {reason}"))?,
                ))
            }
        } else {
            project_type(output, index, paths, &output_generic_types).or_else(|reason| {
                open_chain_result(output, index, paths, &output_generic_types)
                    .ok_or_else(|| format!("projected output: {reason}"))
            })?
        }
    } else {
        ProjectedType::None
    };
    if open_chain.is_none()
        && allow_lifetime_output
        && let Some(Type::ResolvedPath(path)) = effective_output.as_ref()
        && let Some(Item {
            inner: ItemEnum::Struct(structure),
            ..
        }) = index.get(&path.id)
        && structure
            .generics
            .params
            .iter()
            .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }))
        && let ProjectedType::Foreign { arguments, .. } = &result
        && arguments.len()
            != structure
                .generics
                .params
                .iter()
                .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
                .count()
    {
        return Err("lifetime-bearing chain result has unresolved generic state".to_owned());
    }
    if matches!(result, ProjectedType::BoxedInterface { .. }) {
        return Err("boxed trait-object results cannot cross a projected boundary".to_owned());
    }
    let generic_parameters = function
        .generics
        .params
        .iter()
        .filter(|parameter| {
            matches!(
                generic_types.get(&parameter.name),
                Some(ProjectedType::Generic(_))
            )
        })
        .map(|parameter| {
            Ok(ProjectedGenericParameter {
                name: parameter.name.clone(),
                input_selected: function
                    .sig
                    .inputs
                    .iter()
                    .any(|(_, ty)| type_mentions_generic(ty, &parameter.name)),
                rust_bounds: render_generic_bounds(
                    parameter,
                    function,
                    index,
                    paths,
                    &generic_types,
                )?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(ProjectedFunction {
        name: method_name.unwrap_or_default().to_owned(),
        parameters,
        result,
        generic_parameters,
        destination_result,
        error,
        is_async: function.header.is_async || returns_future,
        into_future,
        execution_requirements: (function.header.is_async || returns_future).then_some(
            ProjectedExecutionRequirements {
                runtime_context: RequirementKnowledge::Unknown,
                wake_support: RequirementKnowledge::Required,
                transfer: RequirementKnowledge::Unknown,
            },
        ),
        enum_operation: None,
        error_optional_depth,
        chain_role: None,
        receiver,
    })
}

fn promote_async_endpoint_methods(methods: &mut [ProjectedFunction]) {
    let has_consuming_close = methods
        .iter()
        .any(|method| method.name == "close" && method.receiver == Some(Receiver::Move));
    if !has_consuming_close {
        return;
    }
    for method in methods.iter_mut() {
        if method.name == "next"
            && method.is_async
            && matches!(
                method.receiver,
                Some(Receiver::Borrow | Receiver::MutableBorrow)
            )
            && method.error.is_some()
            && let ProjectedType::Optional(item) = &method.result
        {
            method.result = ProjectedType::AsyncIterationStep(item.clone());
        }
    }
    for method in methods {
        if method.name == "send"
            && method.is_async
            && method.parameters.len() == 1
            && matches!(
                method.receiver,
                Some(Receiver::Borrow | Receiver::MutableBorrow)
            )
            && method.error.is_some()
            && method.result == ProjectedType::Bool
        {
            method.result = ProjectedType::AsyncSinkOutcome;
        }
    }
}

fn projectable_interface_bound<'a>(
    bounds: &'a [GenericBound],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<&'a RustdocPath, String> {
    let candidates = bounds
        .iter()
        .filter_map(|bound| {
            let (trait_, generic_params) = trait_bound_name(bound)?;
            if !generic_params.is_empty() {
                return Some(Err(
                    "higher-ranked interface bound is not projectable".to_owned()
                ));
            }
            let auto = paths
                .get(&trait_.id)
                .map(|summary| summary.path.join("::"))
                .is_some_and(|path| {
                    matches!(path.as_str(), "core::marker::Send" | "core::marker::Sync")
                });
            if auto { None } else { Some(Ok(trait_)) }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let [trait_] = candidates.as_slice() else {
        return Err("generic input requires one projectable interface bound".to_owned());
    };
    let Some(Item {
        inner: ItemEnum::Trait(declaration),
        ..
    }) = index.get(&trait_.id)
    else {
        return Err("generic input has an unresolved interface bound".to_owned());
    };
    project_interface(declaration, index, paths, &trait_.path)
        .map_err(|_| "generic input bound is not a projectable interface".to_owned())?;

    Ok(trait_)
}
fn structural_impl_trait_input(
    bounds: &[GenericBound],
    paths: &HashMap<Id, ItemSummary>,
) -> Option<ProjectedType> {
    let mut candidates = bounds.iter().filter_map(|bound| {
        let (trait_, generic_params) = trait_bound_name(bound)?;
        if !generic_params.is_empty() {
            return None;
        }
        let trait_path = paths
            .get(&trait_.id)
            .map_or_else(|| trait_.path.clone(), |summary| summary.path.join("::"));
        (!matches!(
            trait_path.as_str(),
            "core::marker::Send" | "core::marker::Sync"
        ))
        .then_some((trait_path, trait_))
    });
    let (trait_path, trait_) = candidates.next()?;
    if candidates.next().is_some()
        || !matches!(
            trait_path.as_str(),
            "core::convert::AsRef" | "std::convert::AsRef"
        )
    {
        return None;
    }
    let GenericArgs::AngleBracketed { args, constraints } = trait_.args.as_deref()? else {
        return None;
    };
    if !constraints.is_empty() {
        return None;
    }
    let [GenericArg::Type(target)] = args.as_slice() else {
        return None;
    };
    match target {
        Type::Primitive(name) if name == "str" => Some(ProjectedType::String),
        Type::ResolvedPath(path)
            if paths
                .get(&path.id)
                .map_or_else(|| path.path.clone(), |summary| summary.path.join("::"))
                == "std::path::Path" =>
        {
            Some(ProjectedType::String)
        }
        _ => None,
    }
}

fn trait_bound_name(bound: &GenericBound) -> Option<(&RustdocPath, &[GenericParamDef])> {
    let GenericBound::TraitBound {
        trait_,
        generic_params,
        ..
    } = bound
    else {
        return None;
    };
    Some((trait_, generic_params))
}

fn generic_bounds(parameter: &GenericParamDef, function: &Function) -> Vec<GenericBound> {
    let mut bounds = match &parameter.kind {
        GenericParamDefKind::Type { bounds, .. } => bounds.clone(),
        _ => Vec::new(),
    };
    for predicate in &function.generics.where_predicates {
        let WherePredicate::BoundPredicate {
            type_: Type::Generic(name),
            bounds: predicate_bounds,
            generic_params,
        } = predicate
        else {
            continue;
        };
        if name == &parameter.name {
            if !generic_params.is_empty() {
                bounds.push(GenericBound::Outlives("__higher_ranked__".to_owned()));
            }
            bounds.extend(predicate_bounds.iter().cloned());
        }
    }
    bounds
}

fn concrete_into_future_output(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    outer_generics: &BTreeMap<String, ProjectedType>,
) -> Option<(Type, BTreeMap<String, ProjectedType>)> {
    let Type::ResolvedPath(path) = ty else {
        return None;
    };
    let Item {
        inner: ItemEnum::Struct(structure),
        ..
    } = index.get(&path.id)?
    else {
        return None;
    };
    let mut concrete_generics = outer_generics.clone();
    for (parameter, argument) in structure
        .generics
        .params
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .zip(type_arguments(ty))
    {
        let projected = match argument {
            Type::Generic(name) => outer_generics
                .get(name)
                .cloned()
                .unwrap_or_else(|| ProjectedType::Generic(name.clone())),
            _ => project_type(argument, index, paths, outer_generics).ok()?,
        };
        concrete_generics.insert(parameter.name.clone(), projected);
    }
    for implementation_id in &structure.impls {
        let Some(Item {
            inner: ItemEnum::Impl(implementation),
            ..
        }) = index.get(implementation_id)
        else {
            continue;
        };
        let trait_path = implementation
            .trait_
            .as_ref()
            .and_then(|trait_| paths.get(&trait_.id))
            .map(|summary| summary.path.join("::"));
        if trait_path.as_deref() != Some("core::future::into_future::IntoFuture") {
            continue;
        }
        for item_id in &implementation.items {
            let Some(Item {
                name: Some(name),
                inner:
                    ItemEnum::AssocType {
                        type_: Some(output),
                        ..
                    },
                ..
            }) = index.get(item_id)
            else {
                continue;
            };
            if name == "Output" {
                return Some((output.clone(), concrete_generics));
            }
        }
    }
    None
}

fn future_output(
    name: &str,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedType>, String> {
    let Some(parameter) = function
        .generics
        .params
        .iter()
        .find(|parameter| parameter.name == name)
    else {
        return Ok(None);
    };
    let bounds = generic_bounds(parameter, function);
    for bound in bounds {
        let Some((trait_, generic_params)) = trait_bound_name(&bound) else {
            continue;
        };
        if !trait_.path.ends_with("Future") {
            continue;
        }
        if !generic_params.is_empty() {
            return Err("higher-ranked callback future is not projectable".to_owned());
        }
        let Some(GenericArgs::AngleBracketed { constraints, .. }) = trait_.args.as_deref() else {
            return Err("callback future has no concrete Output type".to_owned());
        };
        let output = constraints.iter().find_map(|constraint| {
            if constraint.name != "Output" {
                return None;
            }
            let AssocItemConstraintKind::Equality(Term::Type(output)) = &constraint.binding else {
                return None;
            };
            Some(output)
        });
        return output
            .map(|output| project_type(output, index, paths, generics))
            .transpose();
    }
    Ok(None)
}

fn project_callback_generic(
    parameter: &GenericParamDef,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    known: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedType>, String> {
    let bounds = generic_bounds(parameter, function);
    let callback = bounds.iter().find_map(|bound| {
        let (trait_, generic_params) = trait_bound_name(bound)?;
        let invocation_mode = if trait_.path.ends_with("FnOnce") {
            InvocationMode::Consuming
        } else if trait_.path.ends_with("FnMut") {
            InvocationMode::Mutable
        } else if trait_.path.ends_with("Fn") {
            InvocationMode::Shared
        } else {
            return None;
        };
        Some((trait_, generic_params, invocation_mode))
    });
    let Some((trait_, generic_params, kind)) = callback else {
        return Ok(None);
    };
    if !generic_params.is_empty() {
        return Err(format!(
            "callback generic `{}` uses a higher-ranked lifetime",
            parameter.name
        ));
    }
    let Some(GenericArgs::Parenthesized { inputs, output }) = trait_.args.as_deref() else {
        return Err(format!(
            "callback generic `{}` has no concrete call signature",
            parameter.name
        ));
    };
    let parameters = inputs
        .iter()
        .map(|input| project_type(input, index, paths, known))
        .collect::<Result<Vec<_>, _>>()?;
    let direct_output = output.clone().unwrap_or(Type::Tuple(Vec::new()));
    let (result, is_async) = if let Type::Generic(future) = &direct_output {
        if let Some(output) = future_output(future, function, index, paths, known)? {
            (output, true)
        } else {
            (project_type(&direct_output, index, paths, known)?, false)
        }
    } else {
        (project_type(&direct_output, index, paths, known)?, false)
    };
    let has_trait = |suffix: &str| {
        bounds.iter().any(|bound| {
            trait_bound_name(bound).is_some_and(|(trait_, _)| trait_.path.ends_with(suffix))
        })
    };
    let retained = bounds
        .iter()
        .any(|bound| matches!(bound, GenericBound::Outlives(name) if name == "'static" || name == "static"));
    Ok(Some(ProjectedType::Callback {
        rust_name: parameter.name.clone(),
        parameters,
        result: Box::new(result),
        invocation_mode: kind,
        is_async,
        retained,
        send: has_trait("Send"),
        sync: has_trait("Sync"),
    }))
}
fn is_callback_parameter(parameter: &GenericParamDef, function: &Function) -> bool {
    generic_bounds(parameter, function).iter().any(|bound| {
        trait_bound_name(bound).is_some_and(|(trait_, _)| {
            matches!(
                trait_.path.rsplit("::").next(),
                Some("Fn" | "FnMut" | "FnOnce")
            )
        })
    })
}

fn is_callback_future_parameter(name: &str, function: &Function) -> bool {
    let future_bound = function
        .generics
        .params
        .iter()
        .find(|parameter| parameter.name == name)
        .is_some_and(|parameter| {
            generic_bounds(parameter, function).iter().any(|bound| {
                trait_bound_name(bound)
                    .is_some_and(|(trait_, _)| trait_.path.rsplit("::").next() == Some("Future"))
            })
        });
    if !future_bound {
        return false;
    }
    function.generics.params.iter().any(|parameter| {
        let bounds = generic_bounds(parameter, function);
        bounds.iter().any(|bound| {
            let Some((trait_, _)) = trait_bound_name(bound) else {
                return false;
            };
            if !matches!(
                trait_.path.rsplit("::").next(),
                Some("Fn" | "FnMut" | "FnOnce")
            ) {
                return false;
            }
            matches!(
                trait_.args.as_deref(),
                Some(GenericArgs::Parenthesized {
                    output: Some(Type::Generic(output)),
                    ..
                }) if output == name
            )
        })
    })
}

fn type_mentions_generic(ty: &Type, generic: &str) -> bool {
    match ty {
        Type::Generic(name) => name == generic,
        Type::ResolvedPath(path) => path
            .args
            .as_deref()
            .is_some_and(|arguments| generic_args_mention(arguments, generic)),
        Type::BorrowedRef { type_, .. }
        | Type::RawPointer { type_, .. }
        | Type::Slice(type_)
        | Type::Array { type_, .. }
        | Type::Pat { type_, .. } => type_mentions_generic(type_, generic),
        Type::Tuple(types) => types
            .iter()
            .any(|item| type_mentions_generic(item, generic)),
        _ => false,
    }
}

fn type_contains_lifetime_argument(ty: &Type) -> bool {
    match ty {
        Type::ResolvedPath(path) => path
            .args
            .as_deref()
            .is_some_and(generic_args_contain_lifetime),
        Type::QualifiedPath {
            args, self_type, ..
        } => {
            args.as_deref().is_some_and(generic_args_contain_lifetime)
                || type_contains_lifetime_argument(self_type)
        }
        Type::BorrowedRef { type_, .. }
        | Type::RawPointer { type_, .. }
        | Type::Slice(type_)
        | Type::Array { type_, .. }
        | Type::Pat { type_, .. } => type_contains_lifetime_argument(type_),
        Type::Tuple(types) => types.iter().any(type_contains_lifetime_argument),
        _ => false,
    }
}

fn generic_args_contain_lifetime(arguments: &GenericArgs) -> bool {
    match arguments {
        GenericArgs::AngleBracketed { args, constraints } => {
            args.iter().any(|argument| match argument {
                GenericArg::Lifetime(_) => true,
                GenericArg::Type(ty) => type_contains_lifetime_argument(ty),
                GenericArg::Const(_) | GenericArg::Infer => false,
            }) || constraints
                .iter()
                .any(|constraint| match &constraint.binding {
                    AssocItemConstraintKind::Equality(Term::Type(ty)) => {
                        type_contains_lifetime_argument(ty)
                    }
                    AssocItemConstraintKind::Constraint(bounds) => bounds.iter().any(|bound| {
                        matches!(bound, GenericBound::Outlives(_))
                            || matches!(
                                bound,
                                GenericBound::TraitBound { generic_params, trait_, .. }
                                    if !generic_params.is_empty()
                                        || trait_.args.as_deref().is_some_and(
                                            generic_args_contain_lifetime
                                        )
                            )
                    }),
                    AssocItemConstraintKind::Equality(Term::Constant(_)) => false,
                })
        }
        GenericArgs::Parenthesized { inputs, output } => {
            inputs.iter().any(type_contains_lifetime_argument)
                || output.as_ref().is_some_and(type_contains_lifetime_argument)
        }
        GenericArgs::ReturnTypeNotation => false,
    }
}

fn generic_args_mention(arguments: &GenericArgs, generic: &str) -> bool {
    match arguments {
        GenericArgs::AngleBracketed { args, constraints } => {
            args.iter().any(|argument| {
                matches!(argument, GenericArg::Type(ty) if type_mentions_generic(ty, generic))
            }) || constraints.iter().any(|constraint| match &constraint.binding {
                AssocItemConstraintKind::Equality(Term::Type(ty)) => {
                    type_mentions_generic(ty, generic)
                }
                AssocItemConstraintKind::Constraint(bounds) => bounds
                    .iter()
                    .any(|bound| generic_bound_mentions(bound, generic)),
                AssocItemConstraintKind::Equality(Term::Constant(_)) => false,
            })
        }
        GenericArgs::Parenthesized { inputs, output } => {
            inputs
                .iter()
                .any(|input| type_mentions_generic(input, generic))
                || output
                    .as_ref()
                    .is_some_and(|output| type_mentions_generic(output, generic))
        }
        GenericArgs::ReturnTypeNotation => false,
    }
}

fn generic_bound_mentions(bound: &GenericBound, generic: &str) -> bool {
    matches!(
        bound,
        GenericBound::TraitBound { trait_, .. }
            if trait_
                .args
                .as_deref()
                .is_some_and(|arguments| generic_args_mention(arguments, generic))
    )
}

fn render_generic_bounds(
    parameter: &GenericParamDef,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Vec<String>, String> {
    let mut rendered = Vec::new();
    if let GenericParamDefKind::Type { bounds, .. } = &parameter.kind {
        for bound in bounds {
            rendered.push(render_generic_bound(
                bound,
                &function.generics.params,
                index,
                paths,
                generics,
            )?);
        }
    }
    for predicate in &function.generics.where_predicates {
        let WherePredicate::BoundPredicate {
            type_: Type::Generic(name),
            bounds,
            generic_params,
        } = predicate
        else {
            continue;
        };
        if name == &parameter.name {
            let outer_generic_params = function
                .generics
                .params
                .iter()
                .chain(generic_params)
                .cloned()
                .collect::<Vec<_>>();
            for bound in bounds {
                rendered.push(render_generic_bound(
                    bound,
                    &outer_generic_params,
                    index,
                    paths,
                    generics,
                )?);
            }
        }
    }
    rendered.sort();
    rendered.dedup();
    Ok(rendered)
}

fn render_generic_bound(
    bound: &GenericBound,
    outer_generic_params: &[GenericParamDef],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    match bound {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            modifier,
        } => {
            let mut quantified = outer_generic_params
                .iter()
                .chain(generic_params)
                .filter_map(|parameter| {
                    matches!(parameter.kind, GenericParamDefKind::Lifetime { .. })
                        .then_some(parameter.name.as_str())
                })
                .collect::<Vec<_>>();
            quantified.sort_unstable();
            quantified.dedup();
            let higher_ranked = if quantified.is_empty() {
                String::new()
            } else {
                format!("for<{}> ", quantified.join(", "))
            };
            let modifier = match modifier {
                rustdoc_types::TraitBoundModifier::None => "",
                rustdoc_types::TraitBoundModifier::Maybe => "?",
                rustdoc_types::TraitBoundModifier::MaybeConst => "~const ",
            };
            Ok(format!(
                "{higher_ranked}{modifier}{}",
                render_resolved_path(trait_, index, paths, generics)?
            ))
        }
        GenericBound::Outlives(lifetime) => Ok(lifetime.clone()),
        GenericBound::Use(_) => {
            Err("precise-capturing generic bound has no stable projection".to_owned())
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "generic selection handles callbacks, destination results, and closed impls in order"
)]
fn generic_monomorphisations(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    supplied: &BTreeMap<String, ProjectedType>,
) -> Result<
    (
        BTreeMap<String, ProjectedType>,
        Option<ProjectedDestinationResult>,
    ),
    String,
> {
    let mut result = supplied.clone();
    let mut destination_result = None;
    // Keep input-selected value generics open until a Terrane call site supplies concrete
    // argument and callback types. Immediate projectable interface inputs retain their
    // established source-class adapter contract.
    for parameter in &function.generics.params {
        if result.contains_key(&parameter.name)
            || is_callback_future_parameter(&parameter.name, function)
            || is_callback_parameter(parameter, function)
        {
            continue;
        }
        let mentioned_inputs = function
            .sig
            .inputs
            .iter()
            .filter(|(_, ty)| type_mentions_generic(ty, &parameter.name))
            .collect::<Vec<_>>();
        if mentioned_inputs.is_empty() {
            let output_selected = function
                .sig
                .output
                .as_ref()
                .is_some_and(|output| type_mentions_generic(output, &parameter.name));
            if !output_selected {
                result.insert(
                    parameter.name.clone(),
                    ProjectedType::Generic(parameter.name.clone()),
                );
            }
            continue;
        }
        let immediate_projectable = mentioned_inputs.len() == 1
            && immediate_generic_input(&mentioned_inputs[0].1, &parameter.name)
            && projectable_interface_bound(&generic_bounds(parameter, function), index, paths)
                .is_ok();
        if !immediate_projectable {
            result.insert(
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            );
        }
    }
    // Resolve callable parameters before unrelated generic parameters. A callback whose
    // signature mentions an open `T` must decline; it must not inherit a guessed closed
    // implementation selected while monomorphising `T`.
    for parameter in &function.generics.params {
        if let Some(callback) =
            project_callback_generic(parameter, function, index, paths, &result)?
        {
            result.insert(parameter.name.clone(), callback);
        }
    }
    for parameter in &function.generics.params {
        if result.contains_key(&parameter.name) {
            continue;
        }
        if project_callback_generic(parameter, function, index, paths, &result)?.is_some() {
            unreachable!("callback parameters were resolved in the first pass");
        }
        if is_callback_future_parameter(&parameter.name, function) {
            result.insert(parameter.name.clone(), ProjectedType::None);
            continue;
        }
        let GenericParamDefKind::Type {
            bounds,
            is_synthetic,
            ..
        } = &parameter.kind
        else {
            continue;
        };
        if *is_synthetic {
            continue;
        }
        let caller_chosen_result = function
            .sig
            .output
            .as_ref()
            .is_some_and(|output| type_mentions_generic(output, &parameter.name))
            && !function
                .sig
                .inputs
                .iter()
                .any(|(_, ty)| type_mentions_generic(ty, &parameter.name));
        if caller_chosen_result {
            if destination_result.is_some() {
                return Err("projected result depends on multiple caller-chosen types".to_owned());
            }
            let mut rendering_generics = result.clone();
            rendering_generics.insert(
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            );
            let rust_bounds =
                render_generic_bounds(parameter, function, index, paths, &rendering_generics)?;
            result.insert(
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            );
            destination_result = Some(ProjectedDestinationResult {
                parameter: parameter.name.clone(),
                bound_roots: rust_bounds
                    .iter()
                    .flat_map(|bound| rust_bound_roots(bound))
                    .collect(),
                rust_bounds,
            });
            continue;
        }
        let caller_chosen_inputs = function
            .sig
            .inputs
            .iter()
            .filter(|(_, ty)| type_mentions_generic(ty, &parameter.name))
            .collect::<Vec<_>>();
        let caller_chosen_input = !caller_chosen_inputs.is_empty()
            && !function
                .sig
                .output
                .as_ref()
                .is_some_and(|output| type_mentions_generic(output, &parameter.name));
        if caller_chosen_input
            && (caller_chosen_inputs.len() != 1
                || !caller_chosen_inputs
                    .iter()
                    .all(|(_, ty)| immediate_generic_input(ty, &parameter.name)))
        {
            return Err(format!(
                "generic bound `{}` must appear in exactly one immediate input",
                parameter.name
            ));
        }
        if caller_chosen_input {
            let all_bounds = generic_bounds(parameter, function);
            if let Ok(trait_) = projectable_interface_bound(&all_bounds, index, paths) {
                let rust_path = render_resolved_path(trait_, index, paths, &result)?;
                result.insert(
                    parameter.name.clone(),
                    ProjectedType::Foreign {
                        name: trait_
                            .path
                            .rsplit("::")
                            .next()
                            .unwrap_or(&trait_.path)
                            .to_owned(),
                        base_rust_path: rust_path.clone(),
                        rust_path,
                        arguments: Vec::new(),
                    },
                );
                continue;
            }
        }
        let Some(GenericBound::TraitBound { trait_, .. }) = bounds.first() else {
            return Err(format!("open generic `{}`", parameter.name));
        };
        let Some(Item {
            inner: ItemEnum::Trait(trait_definition),
            ..
        }) = index.get(&trait_.id)
        else {
            return Err(format!(
                "generic bound for `{}` has no closed impl set",
                parameter.name
            ));
        };
        let mut candidates = trait_definition
            .implementations
            .iter()
            .filter_map(|id| index.get(id))
            .filter_map(|item| match &item.inner {
                ItemEnum::Impl(implementation) => Some(&implementation.for_),
                _ => None,
            })
            .filter_map(|ty| project_type(ty, index, paths, &BTreeMap::new()).ok())
            .collect::<Vec<_>>();
        let mut unique = Vec::new();
        for candidate in candidates.drain(..) {
            if !unique.contains(&candidate) {
                unique.push(candidate);
            }
        }
        let direct = unique
            .iter()
            .filter(|ty| {
                matches!(
                    ty,
                    ProjectedType::String
                        | ProjectedType::Bytes
                        | ProjectedType::Bool
                        | ProjectedType::Int
                        | ProjectedType::Float
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        let selected = if direct.is_empty() { &unique } else { &direct };
        let [chosen] = selected.as_slice() else {
            return Err(if selected.is_empty() {
                format!(
                    "generic bound for `{}` has no Terrane-representable impl",
                    parameter.name
                )
            } else {
                format!(
                    "generic bound for `{}` has {} viable Terrane representations and requires a caller-chosen type",
                    parameter.name,
                    selected.len()
                )
            });
        };
        result.insert(parameter.name.clone(), chosen.clone());
    }
    Ok((result, destination_result))
}
fn rust_bound_roots(bound: &str) -> BTreeSet<String> {
    bound
        .split(['<', '>', ',', '=', '+', '(', ')'])
        .filter_map(|fragment| {
            let fragment = fragment
                .trim()
                .trim_start_matches('?')
                .trim_start_matches("~const ");
            let (root, _) = fragment.split_once("::")?;
            let root = root.split_whitespace().last().unwrap_or(root);
            (!root.starts_with('\'') && !root.is_empty()).then(|| root.to_owned())
        })
        .collect()
}

fn project_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    match ty {
        Type::BorrowedRef { type_, .. } => project_type(type_, index, paths, generics),
        Type::Generic(generic) => generics.get(generic).cloned().ok_or_else(|| {
            if generic == "Self" {
                "receiver type used outside receiver position".to_owned()
            } else {
                format!("unbounded generic `{generic}`")
            }
        }),
        Type::Primitive(primitive) => match primitive.as_str() {
            "bool" => Ok(ProjectedType::Bool),
            "str" => Ok(ProjectedType::String),
            "f32" => Ok(ProjectedType::Float32),
            "f64" => Ok(ProjectedType::Float),
            "char" => Ok(ProjectedType::Char),
            "i64" => Ok(ProjectedType::Int),
            "i8" | "i16" | "i32" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
            | "usize" => Ok(ProjectedType::RustInt(primitive.clone())),
            "unit" => Ok(ProjectedType::None),
            other => Err(format!("unsupported primitive `{other}`")),
        },
        Type::Tuple(types) if types.is_empty() => Ok(ProjectedType::None),
        Type::Tuple(types) => {
            let items = types
                .iter()
                .map(|item| project_type(item, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            if items.windows(2).any(|pair| pair[0] != pair[1]) {
                return Err("heterogeneous tuple has no Terrane tuple representation".to_owned());
            }
            Ok(ProjectedType::Tuple(items))
        }
        Type::FunctionPointer(function) if !function.generic_params.is_empty() => {
            Err("higher-ranked function type is not projectable".to_owned())
        }
        Type::ResolvedPath(path) => project_resolved_type(ty, path, index, paths, generics),
        Type::QualifiedPath {
            name,
            args,
            self_type,
            ..
        } if args.is_none()
            && matches!(self_type.as_ref(), Type::Generic(self_) if self_ == "Self") =>
        {
            generics
                .get(&format!("Self::{name}"))
                .cloned()
                .ok_or_else(|| format!("unresolved associated type `Self::{name}`"))
        }
        Type::QualifiedPath {
            name,
            args,
            self_type,
            ..
        } if args.is_none()
            && matches!(self_type.as_ref(), Type::Generic(owner) if generics.contains_key(owner)) =>
        {
            let Type::Generic(owner) = self_type.as_ref() else {
                unreachable!("qualified generic owner was matched above");
            };
            Ok(ProjectedType::Associated(format!("{owner}::{name}")))
        }
        Type::DynTrait(_) => {
            Err("trait objects require an owning `Box<dyn Trait>` parameter".to_owned())
        }
        _ => Err("type has no stable Rust path".to_owned()),
    }
}
fn project_associated_binding(
    trait_: &RustdocPath,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedAssociatedBinding>, String> {
    let Some(GenericArgs::AngleBracketed { constraints, .. }) = trait_.args.as_deref() else {
        return Ok(None);
    };
    let mut bindings = constraints.iter().map(|constraint| {
        if constraint.args.is_some() {
            return Err("generic associated type bindings are not supported".to_owned());
        }
        match &constraint.binding {
            AssocItemConstraintKind::Equality(Term::Type(ty)) => {
                project_type(ty, index, paths, generics).map(|ty| ProjectedAssociatedBinding {
                    name: constraint.name.clone(),
                    ty: Box::new(ty),
                })
            }
            _ => Err("associated types require an exact type binding".to_owned()),
        }
    });
    let binding = bindings.next().transpose()?;
    if bindings.next().is_some() {
        return Err("more than one associated binding is not supported".to_owned());
    }
    Ok(binding)
}

fn project_dyn_interface(
    dynamic: &rustdoc_types::DynTrait,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    if dynamic
        .lifetime
        .as_deref()
        .is_some_and(|lifetime| lifetime != "'static" && lifetime != "static")
    {
        return Err("borrowed trait objects cannot cross an owning projected boundary".to_owned());
    }
    let bounds = dynamic
        .traits
        .iter()
        .map(|poly| GenericBound::TraitBound {
            trait_: poly.trait_.clone(),
            generic_params: poly.generic_params.clone(),
            modifier: rustdoc_types::TraitBoundModifier::None,
        })
        .collect::<Vec<_>>();
    let trait_ = projectable_interface_bound(&bounds, index, paths)?;
    let mut base_trait = trait_.clone();
    let principal_rust_path = render_resolved_path(trait_, index, paths, generics)?;
    base_trait.args = None;
    let trait_path = render_resolved_path(&base_trait, index, paths, generics)?;
    let associated_type = project_associated_binding(trait_, index, paths, generics)?;
    let Some(Item {
        inner: ItemEnum::Trait(declaration),
        ..
    }) = index.get(&trait_.id)
    else {
        return Err("trait object principal does not resolve to a trait".to_owned());
    };
    let projected_interface = project_interface(declaration, index, paths, &trait_path)?;
    match (
        projected_interface.associated_type.as_ref(),
        associated_type.as_ref(),
    ) {
        (Some(expected), Some(binding)) if expected.name == binding.name => {}
        (Some(expected), Some(binding)) => {
            return Err(format!(
                "boxed projected interface binds `{}` but requires `{}`",
                binding.name, expected.name
            ));
        }
        (Some(expected), None) => {
            return Err(format!(
                "boxed projected interface requires an exact `{}` binding",
                expected.name
            ));
        }
        (None, Some(binding)) => {
            return Err(format!(
                "boxed projected interface has an unexpected `{}` binding",
                binding.name
            ));
        }
        (None, None) => {}
    }
    let mut auto_traits = dynamic
        .traits
        .iter()
        .filter_map(|poly| {
            let canonical = paths.get(&poly.trait_.id)?.path.join("::");
            matches!(
                canonical.as_str(),
                "core::marker::Send"
                    | "std::marker::Send"
                    | "core::marker::Sync"
                    | "std::marker::Sync"
            )
            .then(|| canonical)
        })
        .collect::<Vec<_>>();
    auto_traits.sort();
    auto_traits.dedup();
    let dynamic_bounds = std::iter::once(principal_rust_path.as_str())
        .chain(auto_traits.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" + ");
    Ok(ProjectedType::BoxedInterface {
        rust_path: format!("dyn {dynamic_bounds}"),
        trait_path,
        name: trait_
            .path
            .rsplit("::")
            .next()
            .unwrap_or(&trait_.path)
            .to_owned(),
        auto_traits,
        associated_type,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "resolved Rust type classification keeps canonical paths and recursive shape checks together"
)]
fn project_resolved_type(
    ty: &Type,
    path: &RustdocPath,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    if let Some(Item {
        inner: ItemEnum::TypeAlias(alias),
        ..
    }) = index.get(&path.id)
    {
        let mut substitutions = generics.clone();
        let arguments = type_arguments(ty);
        let parameters = alias
            .generics
            .params
            .iter()
            .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }));
        for (parameter, argument) in parameters.zip(arguments) {
            substitutions.insert(
                parameter.name.clone(),
                project_type(argument, index, paths, generics)?,
            );
        }
        return project_type(&alias.type_, index, paths, &substitutions);
    }
    let resolved = resolved_path_name(path, paths);
    if matches!(
        resolved.as_str(),
        "alloc::string::String" | "std::string::String"
    ) {
        return Ok(ProjectedType::String);
    }
    let arguments = type_arguments(ty);
    if matches!(
        resolved.as_str(),
        "core::option::Option" | "std::option::Option"
    ) {
        let inner = arguments
            .first()
            .ok_or_else(|| "Option has no value type".to_owned())?;
        return Ok(ProjectedType::Optional(Box::new(project_type(
            inner, index, paths, generics,
        )?)));
    }
    if matches!(resolved.as_str(), "alloc::boxed::Box" | "std::boxed::Box") {
        let inner = arguments
            .first()
            .ok_or_else(|| "Box has no value type".to_owned())?;
        let Type::DynTrait(dynamic) = inner else {
            return Err("only owning projected interface boxes are supported".to_owned());
        };
        let ProjectedType::BoxedInterface {
            rust_path: dynamic,
            trait_path,
            name,
            auto_traits,
            associated_type,
        } = project_dyn_interface(dynamic, index, paths, generics)?
        else {
            unreachable!("dynamic interface projection returns its boxed-interface shape");
        };
        return Ok(ProjectedType::BoxedInterface {
            rust_path: format!("Box<{dynamic}>"),
            trait_path,
            name,
            auto_traits,
            associated_type,
        });
    }
    if matches!(resolved.as_str(), "alloc::vec::Vec" | "std::vec::Vec") {
        let item = arguments
            .first()
            .ok_or_else(|| "Vec has no item type".to_owned())?;
        let item = project_type(item, index, paths, generics)?;
        if item == ProjectedType::RustInt("u8".to_owned()) {
            return Ok(ProjectedType::Bytes);
        }
        let rust_path = render_resolved_path(path, index, paths, generics)?;
        return Ok(ProjectedType::Sequence {
            rust_path,
            item: Box::new(item),
        });
    }
    let rust_path = render_resolved_path(path, index, paths, generics)?;
    if matches!(
        resolved.as_str(),
        "std::collections::HashMap"
            | "std::collections::hash::map::HashMap"
            | "alloc::collections::BTreeMap"
            | "alloc::collections::btree::map::BTreeMap"
    ) {
        let [key, value] = arguments.as_slice() else {
            return Err("map type does not have exactly two type arguments".to_owned());
        };
        let key = project_type(key, index, paths, generics)?;
        if !key.is_terrane_scalar() {
            return Err("map key is not a Terrane scalar".to_owned());
        }
        return Ok(ProjectedType::Mapping {
            rust_path,
            key: Box::new(key),
            value: Box::new(project_type(value, index, paths, generics)?),
            ordered: resolved.contains("BTree"),
        });
    }
    if matches!(
        resolved.as_str(),
        "std::collections::HashSet"
            | "std::collections::hash::set::HashSet"
            | "alloc::collections::BTreeSet"
            | "alloc::collections::btree::set::BTreeSet"
    ) {
        let item = arguments
            .first()
            .ok_or_else(|| "set has no item type".to_owned())?;
        let item = project_type(item, index, paths, generics)?;
        if !item.is_terrane_scalar() {
            return Err("set item is not a Terrane scalar".to_owned());
        }
        return Ok(ProjectedType::Set {
            rust_path,
            item: Box::new(item),
            ordered: resolved.contains("BTree"),
        });
    }
    let short = resolved.rsplit("::").next().unwrap_or(&resolved).to_owned();
    let arguments = arguments
        .into_iter()
        .map(|argument| project_type(argument, index, paths, generics))
        .collect::<Result<Vec<_>, _>>()?;
    let name = instantiated_type_name(&short, &rust_path);
    let base_rust_path = rust_path
        .split_once('<')
        .map_or_else(|| rust_path.clone(), |(base, _)| base.to_owned());
    Ok(ProjectedType::Foreign {
        rust_path,
        name,
        base_rust_path,
        arguments,
    })
}
fn render_generic_arguments(
    arguments: &GenericArgs,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    match arguments {
        GenericArgs::AngleBracketed { args, constraints } => {
            if !constraints.is_empty() {
                return Err(
                    "nested associated generic constraints have no stable projection".to_owned(),
                );
            }
            let rendered = args
                .iter()
                .map(|argument| match argument {
                    GenericArg::Lifetime(lifetime) => Ok(lifetime.clone()),
                    GenericArg::Type(ty) => render_rust_type(ty, index, paths, generics),
                    GenericArg::Const(constant) => Ok(constant.expr.clone()),
                    GenericArg::Infer => {
                        Err("inferred generic argument has no stable projection".to_owned())
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("<{}>", rendered.join(", ")))
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let inputs = inputs
                .iter()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            let output = output
                .as_ref()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .transpose()?
                .map_or_else(String::new, |ty| format!(" -> {ty}"));
            Ok(format!("({}){output}", inputs.join(", ")))
        }
        GenericArgs::ReturnTypeNotation => {
            Err("return-type notation has no stable projection".to_owned())
        }
    }
}

fn render_resolved_path(
    path: &RustdocPath,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    let base = match resolved_path_name(path, paths).as_str() {
        "alloc::collections::btree::map::BTreeMap" => "std::collections::BTreeMap".to_owned(),
        "core::task::wake::Context" => "core::task::Context".to_owned(),
        "alloc::collections::btree::set::BTreeSet" => "std::collections::BTreeSet".to_owned(),
        path => path
            .strip_prefix("alloc::")
            .map_or_else(|| path.to_owned(), |path| format!("std::{path}")),
    };
    let Some(arguments) = path.args.as_deref() else {
        return Ok(base);
    };
    match arguments {
        GenericArgs::AngleBracketed { args, constraints } => {
            let mut rendered = args
                .iter()
                .map(|argument| match argument {
                    GenericArg::Lifetime(lifetime) => Ok(lifetime.clone()),
                    GenericArg::Type(ty) => render_rust_type(ty, index, paths, generics),
                    GenericArg::Const(constant) => Ok(constant.expr.clone()),
                    GenericArg::Infer => {
                        Err("inferred generic argument has no stable projection".to_owned())
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            for constraint in constraints {
                let arguments = constraint
                    .args
                    .as_deref()
                    .map(|arguments| render_generic_arguments(arguments, index, paths, generics))
                    .transpose()?
                    .unwrap_or_default();
                let binding = match &constraint.binding {
                    AssocItemConstraintKind::Equality(Term::Type(ty)) => {
                        format!(" = {}", render_rust_type(ty, index, paths, generics)?)
                    }
                    AssocItemConstraintKind::Equality(Term::Constant(constant)) => {
                        format!(" = {}", constant.expr)
                    }
                    AssocItemConstraintKind::Constraint(bounds) => {
                        let bounds = bounds
                            .iter()
                            .map(|bound| render_generic_bound(bound, &[], index, paths, generics))
                            .collect::<Result<Vec<_>, _>>()?;
                        format!(": {}", bounds.join(" + "))
                    }
                };
                rendered.push(format!("{}{arguments}{binding}", constraint.name));
            }
            Ok(format!("{base}<{}>", rendered.join(", ")))
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let inputs = inputs
                .iter()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            let output = output
                .as_ref()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .transpose()?
                .map_or_else(String::new, |ty| format!(" -> {ty}"));
            Ok(format!("{base}({}){output}", inputs.join(", ")))
        }
        GenericArgs::ReturnTypeNotation => {
            Err("return-type notation has no stable projection".to_owned())
        }
    }
}

fn render_rust_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    match ty {
        Type::ResolvedPath(path) => render_resolved_path(path, index, paths, generics),
        Type::Generic(name) => generics
            .get(name)
            .map(ProjectedType::rust_type)
            .ok_or_else(|| format!("unbounded generic `{name}`")),
        Type::Primitive(name) => Ok(if name == "unit" {
            "()".to_owned()
        } else {
            name.clone()
        }),
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => Ok(format!(
            "&{}{}{}",
            lifetime
                .as_deref()
                .map(|lifetime| format!("{lifetime} "))
                .unwrap_or_default(),
            if *is_mutable { "mut " } else { "" },
            render_rust_type(type_, index, paths, generics)?
        )),
        Type::Tuple(types) => Ok(format!(
            "({})",
            types
                .iter()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        )),
        Type::Slice(type_) => Ok(format!(
            "[{}]",
            render_rust_type(type_, index, paths, generics)?
        )),
        Type::Array { type_, len } => Ok(format!(
            "[{}; {len}]",
            render_rust_type(type_, index, paths, generics)?
        )),
        Type::DynTrait(dynamic) => {
            let projected = project_dyn_interface(dynamic, index, paths, generics)?;
            Ok(projected.rust_type())
        }
        Type::QualifiedPath {
            name,
            args,
            self_type,
            ..
        } if args.is_none()
            && generics.contains_key("__terrane_external_associated_bounds")
            && matches!(self_type.as_ref(), Type::Generic(self_) if self_ == "Self") =>
        {
            generics
                .get(&format!("Self::{name}"))
                .map(ProjectedType::rust_type)
                .ok_or_else(|| format!("unresolved associated type `Self::{name}`"))
        }
        Type::QualifiedPath {
            name,
            args,
            self_type,
            ..
        } if args.is_none()
            && matches!(self_type.as_ref(), Type::Generic(owner) if generics.contains_key(owner)) =>
        {
            let Type::Generic(owner) = self_type.as_ref() else {
                unreachable!("qualified generic owner was matched above");
            };
            Ok(format!("{owner}::{name}"))
        }
        _ => Err("generic argument has no stable Rust type spelling".to_owned()),
    }
}

fn instantiated_type_name(short: &str, rust_path: &str) -> String {
    if rust_path.rsplit("::").next() == Some(short) {
        return short.to_owned();
    }
    format!("{short}-{:x}", Sha256::digest(rust_path.as_bytes()))
}

fn receiver_kind(ty: &Type) -> Result<Receiver, String> {
    match ty {
        Type::Generic(name) if name == "Self" => Ok(Receiver::Move),
        Type::BorrowedRef {
            is_mutable, type_, ..
        } if matches!(type_.as_ref(), Type::Generic(name) if name == "Self") => {
            Ok(if *is_mutable {
                Receiver::MutableBorrow
            } else {
                Receiver::Borrow
            })
        }
        _ => Err("receiver is not plain `self`, `&self`, or `&mut self`".to_owned()),
    }
}

fn implementation_trait_path(
    trait_path: &RustdocPath,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<(String, bool)> {
    let summary = paths.get(&trait_path.id)?;
    let path = summary.path.join("::");
    (!path.is_empty()).then_some((path, summary.crate_id == 0))
}

fn implements_trait(
    impls: &[Id],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    expected: &str,
) -> bool {
    impls.iter().any(|id| {
        let Some(Item {
            inner:
                ItemEnum::Impl(Impl {
                    trait_: Some(trait_),
                    ..
                }),
            ..
        }) = index.get(id)
        else {
            return false;
        };
        paths
            .get(&trait_.id)
            .is_some_and(|summary| summary.path.join("::") == expected)
    })
}

fn resolved_path_name(path: &RustdocPath, paths: &HashMap<Id, ItemSummary>) -> String {
    paths
        .get(&path.id)
        .map(|summary| summary.path.join("::"))
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| path.path.replace("crate::", ""))
}
fn impl_trait_bounds(ty: &Type) -> Option<&[GenericBound]> {
    match ty {
        Type::ImplTrait(bounds) => Some(bounds),
        Type::BorrowedRef { type_, .. } => match type_.as_ref() {
            Type::ImplTrait(bounds) => Some(bounds),

            _ => None,
        },
        _ => None,
    }
}
fn project_enum_payload(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<
    (
        ProjectedType,
        ProjectedEnumPayloadConversion,
        ProjectedType,
        ProjectedEnumPayloadConversion,
        String,
    ),
    String,
> {
    let projected = project_type(ty, index, paths, &BTreeMap::new())?;
    let rust_type = render_rust_type(ty, index, paths, &BTreeMap::new())?;
    let mut constructor_type = projected.clone();
    let mut constructor_conversion = ProjectedEnumPayloadConversion::Identity;
    let mut extraction_type = projected.clone();
    let mut extraction_conversion = ProjectedEnumPayloadConversion::Identity;
    if matches!(projected, ProjectedType::Foreign { .. }) {
        if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::From",
            is_rust_string_type,
        ) {
            constructor_type = ProjectedType::String;
            constructor_conversion = ProjectedEnumPayloadConversion::Into;
        } else if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::From",
            is_rust_byte_vector_type,
        ) {
            constructor_type = ProjectedType::Bytes;
            constructor_conversion = ProjectedEnumPayloadConversion::Into;
        }
        if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::AsRef",
            |argument, _| matches!(argument, Type::Primitive(name) if name == "str"),
        ) {
            extraction_type = ProjectedType::String;
            extraction_conversion = ProjectedEnumPayloadConversion::AsRefString;
        } else if type_implements_deref_target(
            ty,
            index,
            paths,
            |target| matches!(target, Type::Primitive(name) if name == "str"),
        ) {
            extraction_type = ProjectedType::String;
            extraction_conversion = ProjectedEnumPayloadConversion::DerefString;
        } else if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::AsRef",
            |argument, _| {
                matches!(
                    argument,
                    Type::Slice(item) if matches!(item.as_ref(), Type::Primitive(name) if name == "u8")
                )
            },
        ) {
            extraction_type = ProjectedType::Bytes;
            extraction_conversion = ProjectedEnumPayloadConversion::AsRefBytes;
        } else if type_implements_deref_target(ty, index, paths, |target| {
            matches!(
                target,
                Type::Slice(item) if matches!(item.as_ref(), Type::Primitive(name) if name == "u8")
            )
        }) {
            extraction_type = ProjectedType::Bytes;
            extraction_conversion = ProjectedEnumPayloadConversion::DerefBytes;
        }
    }
    Ok((
        constructor_type,
        constructor_conversion,
        extraction_type,
        extraction_conversion,
        rust_type,
    ))
}

fn type_implements_deref_target(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    target_matches: impl Fn(&Type) -> bool,
) -> bool {
    let Type::ResolvedPath(path) = ty else {
        return false;
    };
    let Some(item) = index.get(&path.id) else {
        return false;
    };
    let implementations = match &item.inner {
        ItemEnum::Struct(structure) => &structure.impls,
        ItemEnum::Enum(enumeration) => &enumeration.impls,
        _ => return false,
    };
    implementations.iter().any(|implementation| {
        let Some(Item {
            inner:
                ItemEnum::Impl(Impl {
                    trait_: Some(trait_),
                    items,
                    ..
                }),
            ..
        }) = index.get(implementation)
        else {
            return false;
        };
        if paths
            .get(&trait_.id)
            .is_none_or(|summary| summary.path.join("::") != "core::ops::deref::Deref")
        {
            return false;
        }
        items.iter().any(|item| {
            matches!(
                index.get(item),
                Some(Item {
                    name: Some(name),
                    inner: ItemEnum::AssocType { type_: Some(target), .. },
                    ..
                }) if name == "Target" && target_matches(target)
            )
        })
    })
}

fn type_implements_generic_trait(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    expected_trait: &str,
    argument_matches: impl Fn(&Type, &HashMap<Id, ItemSummary>) -> bool,
) -> bool {
    let Type::ResolvedPath(path) = ty else {
        return false;
    };
    let Some(item) = index.get(&path.id) else {
        return false;
    };
    let implementations = match &item.inner {
        ItemEnum::Struct(structure) => &structure.impls,
        ItemEnum::Enum(enumeration) => &enumeration.impls,
        _ => return false,
    };
    implementations.iter().any(|implementation| {
        let Some(Item {
            inner:
                ItemEnum::Impl(Impl {
                    trait_: Some(trait_),
                    ..
                }),
            ..
        }) = index.get(implementation)
        else {
            return false;
        };
        if paths
            .get(&trait_.id)
            .is_none_or(|summary| summary.path.join("::") != expected_trait)
        {
            return false;
        }
        let Some(arguments) = trait_.args.as_deref() else {
            return false;
        };
        let GenericArgs::AngleBracketed { args, .. } = arguments else {
            return false;
        };
        matches!(args.as_slice(), [GenericArg::Type(argument)] if argument_matches(argument, paths))
    })
}

fn project_boundary_capabilities(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> ProjectedBoundaryCapabilities {
    ProjectedBoundaryCapabilities {
        string_constructor: type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::From",
            is_rust_string_type,
        )
        .then_some(ProjectedEnumPayloadConversion::Into),
        bytes_constructor: type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::From",
            is_rust_byte_vector_type,
        )
        .then_some(ProjectedEnumPayloadConversion::Into),
        string_extraction: if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::AsRef",
            |argument, _| matches!(argument, Type::Primitive(name) if name == "str"),
        ) {
            Some(ProjectedEnumPayloadConversion::AsRefString)
        } else {
            type_implements_deref_target(
                ty,
                index,
                paths,
                |target| matches!(target, Type::Primitive(name) if name == "str"),
            )
            .then_some(ProjectedEnumPayloadConversion::DerefString)
        },
        bytes_extraction: if type_implements_generic_trait(
            ty,
            index,
            paths,
            "core::convert::AsRef",
            |argument, _| {
                matches!(
                    argument,
                    Type::Slice(item) if matches!(item.as_ref(), Type::Primitive(name) if name == "u8")
                )
            },
        ) {
            Some(ProjectedEnumPayloadConversion::AsRefBytes)
        } else {
            type_implements_deref_target(ty, index, paths, |target| {
                matches!(
                    target,
                    Type::Slice(item)
                        if matches!(item.as_ref(), Type::Primitive(name) if name == "u8")
                )
            })
            .then_some(ProjectedEnumPayloadConversion::DerefBytes)
        },
    }
}

fn is_rust_string_type(ty: &Type, paths: &HashMap<Id, ItemSummary>) -> bool {
    matches!(
        ty,
        Type::ResolvedPath(path)
            if matches!(
                resolved_path_name(path, paths).as_str(),
                "alloc::string::String" | "std::string::String" | "std::string::string::String"
            )
    )
}

fn is_rust_byte_vector_type(ty: &Type, paths: &HashMap<Id, ItemSummary>) -> bool {
    let Type::ResolvedPath(path) = ty else {
        return false;
    };
    if !matches!(
        resolved_path_name(path, paths).as_str(),
        "alloc::vec::Vec" | "std::vec::Vec" | "std::vec::vec::Vec"
    ) {
        return false;
    }
    matches!(
        path.args.as_deref(),
        Some(GenericArgs::AngleBracketed { args, .. })
            if matches!(args.as_slice(), [GenericArg::Type(Type::Primitive(name))] if name == "u8")
    )
}

fn resolved_name(ty: &Type, paths: &HashMap<Id, ItemSummary>) -> Option<String> {
    match ty {
        Type::ResolvedPath(path) => Some(resolved_path_name(path, paths)),
        _ => None,
    }
}

fn immediate_generic_input(ty: &Type, generic: &str) -> bool {
    match ty {
        Type::Generic(name) => name == generic,
        Type::BorrowedRef { type_, .. } => {
            matches!(type_.as_ref(), Type::Generic(name) if name == generic)
        }
        _ => false,
    }
}

fn resolved_error_name(ty: &Type, paths: &HashMap<Id, ItemSummary>) -> Option<String> {
    let Type::ResolvedPath(path) = ty else {
        return None;
    };
    let resolved = resolved_path_name(path, paths);
    let written = path.path.replace("crate::", "");
    Some(
        [resolved, written]
            .into_iter()
            .max_by_key(|candidate| candidate.matches("::").count())
            .expect("two error type spellings are available"),
    )
}

fn projected_error_name(
    error: &Type,
    result: &Type,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<String> {
    let error = resolved_error_name(error, paths)?;
    if rust_path_owner(&error).is_some() {
        return Some(error);
    }
    let Type::ResolvedPath(result) = result else {
        return Some(error);
    };
    let resolved = resolved_path_name(result, paths);
    let written = result.path.replace("crate::", "");
    let alias = [resolved, written]
        .into_iter()
        .max_by_key(|candidate| candidate.matches("::").count())
        .expect("two result type spellings are available");
    let Some(owner) = alias.strip_suffix("::Result") else {
        return Some(error);
    };
    if matches!(owner, "std::result" | "core::result") {
        Some(error)
    } else {
        Some(format!("{owner}::Error"))
    }
}

fn type_contains_borrowed_ref(ty: &Type) -> bool {
    match ty {
        Type::BorrowedRef { .. } => true,
        Type::ResolvedPath(_) => type_arguments(ty)
            .into_iter()
            .any(type_contains_borrowed_ref),
        Type::Tuple(items) => items.iter().any(type_contains_borrowed_ref),
        Type::Slice(item)
        | Type::Array { type_: item, .. }
        | Type::Pat { type_: item, .. }
        | Type::RawPointer { type_: item, .. } => type_contains_borrowed_ref(item),
        // Borrowed values inside callable signatures are governed by the
        // callable boundary rather than escaping through the outer parameter.
        Type::FunctionPointer(_)
        | Type::DynTrait(_)
        | Type::Generic(_)
        | Type::Primitive(_)
        | Type::ImplTrait(_)
        | Type::Infer => false,
        Type::QualifiedPath { self_type, .. } => type_contains_borrowed_ref(self_type),
    }
}

fn type_arguments(ty: &Type) -> Vec<&Type> {
    let Type::ResolvedPath(path) = ty else {
        return Vec::new();
    };
    let Some(GenericArgs::AngleBracketed { args, .. }) = path.args.as_deref() else {
        return Vec::new();
    };
    args.iter()
        .filter_map(|argument| match argument {
            GenericArg::Type(ty) => Some(ty),
            _ => None,
        })
        .collect()
}

fn dependency_namespace(dependency: &RustDependency, path: &[String]) -> String {
    let modules = path
        .iter()
        .skip(1)
        .map(|segment| segment.to_lowercase().replace('_', "-"))
        .collect::<Vec<_>>();
    if modules.is_empty() {
        format!("/deps/{}", dependency.name.replace('_', "-"))
    } else {
        format!(
            "/deps/{}/{}",
            dependency.name.replace('_', "-"),
            modules.join("/")
        )
    }
}

#[must_use]
pub fn namespace_for_rust_path(dependency: &ProjectedDependency, rust_path: &str) -> String {
    let modules = rust_path.split("::").skip(1).collect::<Vec<_>>();
    let modules = &modules[..modules.len().saturating_sub(1)];
    if modules.is_empty() {
        format!("/deps/{}", dependency.name.replace('_', "-"))
    } else {
        format!(
            "/deps/{}/{}",
            dependency.name.replace('_', "-"),
            modules
                .iter()
                .map(|segment| segment.to_lowercase().replace('_', "-"))
                .collect::<Vec<_>>()
                .join("/")
        )
    }
}

fn safe_parameter_name(name: &str) -> String {
    if matches!(
        name,
        "as" | "await"
            | "case"
            | "catch"
            | "class"
            | "else"
            | "finally"
            | "for"
            | "function"
            | "goto"
            | "if"
            | "import"
            | "is"
            | "label"
            | "linear"
            | "match"
            | "move"
            | "namespace"
            | "ref"
            | "return"
            | "rust"
            | "throw"
            | "unsafe"
            | "use"
            | "when"
            | "select"
            | "yield"
    ) {
        format!("{name}_")
    } else {
        name.to_owned()
    }
}

fn projection_lock_identity(lock: &[u8]) -> Result<Vec<u8>, ProjectionError> {
    let Ok(text) = std::str::from_utf8(lock) else {
        return Ok(lock.to_vec());
    };
    let mut parsed = text
        .parse::<toml::Value>()
        .map_err(|error| ProjectionError {
            message: format!("invalid dependency projection lockfile: {error}"),
        })?;
    if let Some(packages) = parsed
        .get_mut("package")
        .and_then(toml::Value::as_array_mut)
    {
        packages.retain(|package| {
            package.get("name").and_then(toml::Value::as_str)
                != Some("terrane_dependency_projection")
        });
    }
    Ok(parsed.to_string().into_bytes())
}

fn cache_identity(
    root: &Path,
    workspace: &Path,
    dependencies: &[RustDependency],
    containment: Containment,
) -> Result<(String, String), ProjectionError> {
    let manifest = fs::read(root.join(crate::MANIFEST_FILE_NAME)).unwrap_or_default();
    let lock = fs::read(workspace.join("Cargo.lock"))
        .or_else(|_| fs::read(root.join("Cargo.lock")))
        .unwrap_or_default();
    let lock = projection_lock_identity(&lock)?;
    let build_selector = format!("+{}", crate::BUILD_TOOLCHAIN);
    let rustc_verbose = tool_version("rustc", &[&build_selector, "-vV"])?;
    let target = selected_target(workspace, &rustc_verbose)?;
    let rustdoc_format = rustdoc_types::FORMAT_VERSION.to_string();
    let mut hash = Sha256::new();
    for (label, bytes) in [
        ("manifest", manifest.as_slice()),
        ("lock", lock.as_slice()),
        ("inputs", format!("{dependencies:?}").as_bytes()),
        ("target", target.as_bytes()),
        ("build-toolchain", crate::BUILD_TOOLCHAIN.as_bytes()),
        ("rustdoc-toolchain", RUSTDOC_TOOLCHAIN.as_bytes()),
        ("rustdoc-format", rustdoc_format.as_bytes()),
        ("schema", PROJECTION_SCHEMA.as_bytes()),
        ("containment", format!("{containment:?}").as_bytes()),
    ] {
        hash.update(label.len().to_le_bytes());
        hash.update(label.as_bytes());
        hash.update(bytes.len().to_le_bytes());
        hash.update(bytes);
    }
    Ok((format!("{:x}", hash.finalize()), target))
}

fn selected_target(
    workspace: &Path,
    rustc_verbose_version: &str,
) -> Result<String, ProjectionError> {
    if let Some(target) = std::env::var("CARGO_BUILD_TARGET")
        .ok()
        .filter(|target| !target.is_empty())
    {
        return Ok(target);
    }
    let output = Command::new("cargo")
        .arg(format!("+{RUSTDOC_TOOLCHAIN}"))
        .args([
            "-Z",
            "unstable-options",
            "config",
            "get",
            "build.target",
            "--format",
            "json",
        ])
        .current_dir(workspace)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot inspect Cargo build target configuration: {error}"),
        })?;
    if output.status.success() {
        let config =
            serde_json::from_slice::<serde_json::Value>(&output.stdout).map_err(|error| {
                ProjectionError {
                    message: format!("cannot decode Cargo build target configuration: {error}"),
                }
            })?;
        let target = config
            .get("build.target")
            .or_else(|| config.get("build").and_then(|build| build.get("target")));
        return match target {
            Some(serde_json::Value::String(target)) => Ok(target.clone()),
            Some(serde_json::Value::Array(targets)) if targets.len() == 1 => targets[0]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| ProjectionError {
                    message: "Cargo build target configuration is not a string".to_owned(),
                }),
            Some(serde_json::Value::Array(_)) => Err(ProjectionError {
                message: "dependency projection requires exactly one Cargo build target".to_owned(),
            }),
            _ => Err(ProjectionError {
                message: "Cargo returned no usable build target configuration".to_owned(),
            }),
        };
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.contains("config value `build.target` is not set") {
        return Err(ProjectionError {
            message: format!(
                "cannot inspect Cargo build target configuration: {}",
                stderr.trim()
            ),
        });
    }
    Ok(rustc_verbose_version
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(str::to_owned))
        .unwrap_or_else(|| "unknown-target".to_owned()))
}

fn tool_version(program: &str, arguments: &[&str]) -> Result<String, ProjectionError> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot inspect dependency projection toolchain: {error}"),
        })?;
    if !output.status.success() {
        return Err(ProjectionError {
            message: format!(
                "cannot inspect dependency projection toolchain: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn containment() -> Containment {
    static CONTAINMENT: LazyLock<Containment> = LazyLock::new(|| {
        let available = Command::new("bwrap")
            .args([
                "--die-with-parent",
                "--unshare-all",
                "--ro-bind",
                "/",
                "/",
                "--dev",
                "/dev",
                "--proc",
                "/proc",
                "--",
                "/bin/true",
            ])
            .output()
            .is_ok_and(|output| output.status.success());
        if available {
            Containment::Enforced
        } else {
            Containment::Unavailable
        }
    });
    *CONTAINMENT
}

fn write_if_changed(path: &Path, content: &[u8]) -> Result<(), ProjectionError> {
    if fs::read(path).is_ok_and(|existing| existing == content) {
        return Ok(());
    }
    fs::write(path, content).map_err(io_error("write dependency projection input"))
}

fn io_error(context: &'static str) -> impl FnOnce(std::io::Error) -> ProjectionError {
    move |error| ProjectionError {
        message: format!("{context}: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet, HashMap};
    use std::fs;

    use rustdoc_types::{
        GenericArg, GenericArgs, GenericBound, GenericParamDef, GenericParamDefKind, Id, ItemKind,
        ItemSummary, Path as RustdocPath, TraitBoundModifier, Type,
    };
    use serde_json::json;

    use super::{
        ArtifactDependency, Containment, DeclinedItem, InvocationMode, NamespaceOverlay,
        ProjectedBoundDependency, ProjectedBoundaryCapabilities, ProjectedDependency,
        ProjectedFunction, ProjectedInterface, ProjectedItem, ProjectedKind, ProjectedParameter,
        ProjectedType, Projection, ProjectionArtifact, ProjectionHistory, ProjectionResolution,
        ProjectionSource, Receiver, ResolutionOutcome, apply_namespace_overlays,
        apply_projection_history, decline_functions_with_missing_generic_interfaces,
        decline_unproven_projected_interfaces, enforce_transitive_reachability,
        has_type_parameters, namespace_overlays_from_metadata, parse_rustdoc, project_type,
        projectable_interface_bound, projection_content_hash, prune_projection_cache,
        receiver_kind, resolve, rewrite_rust_bound_root, selected_target,
        validate_projection_artifact,
    };
    use crate::RustDependency;
    use terrane_rust_analysis::prefer_public_path;
    fn dependency(name: &str, package: &str, features: &[&str]) -> RustDependency {
        RustDependency {
            name: name.to_owned(),
            package: package.to_owned(),
            version: "=1.0.0".to_owned(),
            features: features
                .iter()
                .map(|feature| (*feature).to_owned())
                .collect(),
            default_features: true,
            target: None,
            effects: Vec::new(),
        }
    }

    fn projected_function_item(namespace: &str, name: &str, rust_path: &str) -> ProjectedItem {
        ProjectedItem {
            namespace: namespace.to_owned(),
            name: name.to_owned(),
            rust_path: rust_path.to_owned(),
            docs: None,
            kind: ProjectedKind::Function(ProjectedFunction {
                name: name.to_owned(),
                generic_parameters: Vec::new(),
                parameters: Vec::new(),
                result: ProjectedType::None,
                destination_result: None,
                error: None,
                is_async: false,
                into_future: false,
                execution_requirements: None,
                enum_operation: None,
                error_optional_depth: 0,
                chain_role: None,
                receiver: None,
            }),
        }
    }

    #[test]
    fn projected_type_lookup_is_scoped_to_canonical_namespace() {
        let item = |namespace: &str, rust_path: &str| {
            let mut item = projected_function_item(namespace, "make", rust_path);
            let ProjectedKind::Function(function) = &mut item.kind else {
                unreachable!();
            };
            function.result = ProjectedType::Foreign {
                rust_path: rust_path.to_owned(),
                name: "Message".to_owned(),
                base_rust_path: rust_path.to_owned(),
                arguments: Vec::new(),
            };
            item
        };
        let projection = Projection {
            cache_identity: "canonical-identities".to_owned(),
            content_hash: String::new(),
            dependencies: vec![
                ProjectedDependency {
                    name: "one".to_owned(),
                    package: "one".to_owned(),
                    version: "1.0.0".to_owned(),
                    items: vec![item("/deps/one", "one::Message")],
                    declined: Vec::new(),
                },
                ProjectedDependency {
                    name: "two".to_owned(),
                    package: "two".to_owned(),
                    version: "1.0.0".to_owned(),
                    items: vec![item("/deps/two", "two::Message")],
                    declined: Vec::new(),
                },
            ],
            bound_dependencies: Vec::new(),
            containment: Containment::Enforced,
            source: ProjectionSource::default(),
            probes: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution::default(),
            removed: Vec::new(),
        };

        assert_eq!(
            projection
                .projected_type("/deps/two", "Message")
                .map(|ty| ty.rust_type()),
            Some("two::Message".to_owned())
        );
        assert!(projection.projected_type("/app", "Message").is_none());
    }

    #[test]
    fn dependency_metadata_uses_resolved_default_features_and_renamed_packages_for_overlays() {
        let dependencies = [
            dependency("db", "sqlx-sqlite", &[]),
            dependency("bridges", "terrane-integration-adapters", &[]),
        ];
        let metadata = serde_json::json!({
            "packages": [
                {
                    "id": "registry+sqlx-sqlite@1.0.0",
                    "name": "sqlx-sqlite",
                    "version": "1.0.0",
                    "metadata": {}
                },
                {
                    "id": "path+terrane-integration-adapters@1.0.0",
                    "name": "terrane-integration-adapters",
                    "version": "1.0.0",
                    "metadata": {
                        "terrane": {
                            "namespace-overlays": [{
                                "module": "sqlx_sqlite",
                                "target-package": "sqlx-sqlite",
                                "feature": "sqlx-sqlite"
                            }]
                        }
                    }
                }
            ],
            "resolve": {
                "root": "path+root@0.1.0",
                "nodes": [
                    {
                        "id": "path+root@0.1.0",
                        "deps": [
                            {"name": "db", "pkg": "registry+sqlx-sqlite@1.0.0"},
                            {"name": "bridges", "pkg": "path+terrane-integration-adapters@1.0.0"}
                        ]
                    },
                    {
                        "id": "registry+sqlx-sqlite@1.0.0",
                        "features": []
                    },
                    {
                        "id": "path+terrane-integration-adapters@1.0.0",
                        "features": ["default", "sqlx-sqlite"]
                    }
                ]
            }
        });
        assert_eq!(
            namespace_overlays_from_metadata(&metadata, &dependencies).unwrap(),
            [NamespaceOverlay {
                provider_name: "bridges".to_owned(),
                provider_package: "terrane-integration-adapters".to_owned(),
                source_namespace: "/deps/bridges/sqlx-sqlite".to_owned(),
                target_namespace: "/deps/db".to_owned(),
            }]
        );
    }

    #[test]
    fn namespace_overlays_require_a_direct_target_and_reject_collisions() {
        let metadata = serde_json::json!({
            "packages": [{
                "id": "path+adapter@1.0.0",
                "name": "adapter",
                "version": "1.0.0",
                "metadata": {
                    "terrane": {
                        "namespace-overlays": [{
                            "module": "upstream",
                            "target-package": "upstream",
                            "feature": "bridge"
                        }]
                    }
                }
            }],
            "resolve": {
                "root": "path+root@0.1.0",
                "nodes": [
                    {
                        "id": "path+root@0.1.0",
                        "deps": [{"name": "adapter", "pkg": "path+adapter@1.0.0"}]
                    },
                    {
                        "id": "path+adapter@1.0.0",
                        "features": ["bridge"]
                    }
                ]
            }
        });
        let undeclared = namespace_overlays_from_metadata(
            &metadata,
            &[dependency("adapter", "adapter", &["bridge"])],
        )
        .unwrap_err();
        assert!(
            undeclared
                .message
                .contains("targets undeclared package `upstream`")
        );

        let overlay = NamespaceOverlay {
            provider_name: "adapter".to_owned(),
            provider_package: "adapter".to_owned(),
            source_namespace: "/deps/adapter/upstream".to_owned(),
            target_namespace: "/deps/upstream".to_owned(),
        };
        let mut projected = [
            ProjectedDependency {
                name: "upstream".to_owned(),
                package: "upstream".to_owned(),
                version: "1.0.0".to_owned(),
                items: vec![projected_function_item(
                    "/deps/upstream",
                    "open",
                    "upstream::open",
                )],
                declined: Vec::new(),
            },
            ProjectedDependency {
                name: "adapter".to_owned(),
                package: "adapter".to_owned(),
                version: "1.0.0".to_owned(),
                items: vec![projected_function_item(
                    "/deps/adapter/upstream",
                    "open",
                    "adapter::upstream::open",
                )],
                declined: Vec::new(),
            },
        ];
        let collision = apply_namespace_overlays(&mut projected, &[overlay]).unwrap_err();
        assert!(
            collision
                .message
                .contains("collides on `/deps/upstream::open`")
        );
    }

    #[test]
    fn target_identity_reads_effective_cargo_configuration() {
        let directory =
            std::env::temp_dir().join(format!("terrane-projection-target-{}", std::process::id()));
        fs::create_dir_all(directory.join(".cargo")).unwrap();
        fs::write(
            directory.join(".cargo/config.toml"),
            "[build]\ntarget = \"wasm32-unknown-unknown\"\n",
        )
        .unwrap();
        assert_eq!(
            selected_target(&directory, "host: x86_64-unknown-linux-gnu").unwrap(),
            "wasm32-unknown-unknown"
        );
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn bound_alias_rewrite_only_changes_path_roots() {
        assert_eq!(
            rewrite_rust_bound_root(
                "for<'value> factory::Decode<'value> + outer::factory::Marker",
                "factory",
                "renamed",
            ),
            "for<'value> renamed::Decode<'value> + outer::factory::Marker"
        );
    }

    #[test]
    fn equally_short_public_paths_use_lexical_tie_breaking() {
        let id = Id(1);
        let mut paths = BTreeMap::new();
        prefer_public_path(&mut paths, id, "crate::zeta::Item".to_owned());
        prefer_public_path(&mut paths, id, "crate::alpha::Item".to_owned());
        assert_eq!(paths[&id], "crate::alpha::Item");
    }

    #[test]
    fn substantive_public_paths_beat_shorter_prelude_paths() {
        let id = Id(1);
        let mut paths = BTreeMap::new();
        prefer_public_path(&mut paths, id, "crate::prelude::Item".to_owned());
        prefer_public_path(&mut paths, id, "crate::algorithm::special::Item".to_owned());
        assert_eq!(paths[&id], "crate::algorithm::special::Item");
    }

    #[test]
    fn projected_map_and_set_keys_require_scalars() {
        assert!(ProjectedType::String.is_terrane_scalar());
        assert!(ProjectedType::Bytes.is_terrane_scalar());
        assert!(!ProjectedType::Optional(Box::new(ProjectedType::String)).is_terrane_scalar());
        assert!(
            !ProjectedType::Sequence {
                rust_path: "alloc::vec::Vec<String>".to_owned(),
                item: Box::new(ProjectedType::String),
            }
            .is_terrane_scalar()
        );
        assert!(!ProjectedType::Tuple(vec![ProjectedType::String]).is_terrane_scalar());
    }

    #[test]
    fn projected_callback_name_is_the_language_server_signature() {
        let callback = ProjectedType::Callback {
            rust_name: "F".to_owned(),
            parameters: vec![ProjectedType::String, ProjectedType::Bool],
            result: Box::new(ProjectedType::Int),
            invocation_mode: InvocationMode::Shared,
            is_async: true,
            retained: true,
            send: true,
            sync: true,
        };
        assert_eq!(
            callback.terrane_name(),
            "async function from string, bool to int"
        );
    }

    #[test]
    fn dependency_free_resolution_records_its_outcome() {
        let projection = resolve(std::path::Path::new("."), &[]).unwrap();
        assert_eq!(
            projection.resolution.outcome,
            ResolutionOutcome::NoDependencies
        );
        assert!(projection.resolution.events.is_empty());
        assert_eq!(
            projection.content_hash,
            projection_content_hash(&projection).unwrap()
        );
    }

    #[test]
    fn exact_decline_path_wins_over_conflicting_suffixes() {
        let dependency = |name: &str, rust_path: &str, reason: &str| ProjectedDependency {
            name: name.to_owned(),
            package: name.to_owned(),
            version: "1.0.0".to_owned(),
            items: Vec::new(),
            declined: vec![DeclinedItem {
                rust_path: rust_path.to_owned(),
                reason: reason.to_owned(),
            }],
        };
        let dependencies = vec![
            dependency("one", "one::Type::build", "first reason"),
            dependency("two", "two::Type::build", "second reason"),
        ];
        assert_eq!(
            Projection::unique_declined_reason(&dependencies, Some("one::Type::build"), "::build"),
            Some("first reason")
        );
        assert_eq!(
            Projection::unique_declined_reason(&dependencies, None, "::build"),
            None
        );
    }

    #[test]
    fn ambiguous_projected_type_identities_do_not_resolve() {
        let dependency = |name: &str, rust_path: &str| ProjectedDependency {
            name: name.to_owned(),
            package: name.to_owned(),
            version: "1.0.0".to_owned(),
            items: vec![ProjectedItem {
                namespace: "/deps/shared".to_owned(),
                name: "Generic".to_owned(),
                rust_path: rust_path.to_owned(),
                docs: None,
                kind: ProjectedKind::ForeignType {
                    methods: Vec::new(),
                    static_methods: Vec::new(),
                    boundary: ProjectedBoundaryCapabilities::default(),
                    displayable: false,
                    cloneable: false,
                    send: false,
                    sync: false,
                },
            }],
            declined: Vec::new(),
        };
        let nested_dependency = |dependency_name: &str, rust_path: &str| {
            let mut dependency = dependency(dependency_name, rust_path);
            let mut item = projected_function_item("/deps/shared", dependency_name, "shared::make");
            let ProjectedKind::Function(function) = &mut item.kind else {
                unreachable!();
            };
            function.result = ProjectedType::Foreign {
                rust_path: rust_path.to_owned(),
                name: "Message".to_owned(),
                base_rust_path: rust_path.to_owned(),
                arguments: Vec::new(),
            };
            dependency.items = vec![item];
            dependency
        };
        let projection = |dependencies| Projection {
            cache_identity: "ambiguous-identities".to_owned(),
            content_hash: String::new(),
            dependencies,
            bound_dependencies: Vec::new(),
            containment: Containment::Enforced,
            source: ProjectionSource::default(),
            probes: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution::default(),
            removed: Vec::new(),
        };

        let mut first = dependency("one", "one::Generic<A>");
        let ProjectedKind::ForeignType { send, .. } = &mut first.items[0].kind else {
            unreachable!();
        };
        *send = true;
        let direct = projection(vec![first, dependency("two", "two::Generic<B>")]);
        assert!(direct.item("/deps/shared", "Generic").is_none());
        assert_eq!(
            direct.item_ambiguity("/deps/shared", "Generic").as_deref(),
            Some("projected Rust types `one::Generic<A>`, `two::Generic<B>`")
        );
        assert!(direct.projected_type("/deps/shared", "Generic").is_none());
        assert!(!direct.projected_type_is_send("/deps/shared", "Generic"));
        let source_error = direct
            .source_for_imports(&BTreeMap::from([(
                "/deps/shared".to_owned(),
                BTreeSet::from(["Generic".to_owned()]),
            )]))
            .unwrap_err();
        assert_eq!(
            source_error,
            "projected import `/deps/shared::Generic` is ambiguous: projected Rust types `one::Generic<A>`, `two::Generic<B>`"
        );
        assert!(
            projection(vec![
                nested_dependency("one", "one::Message"),
                nested_dependency("two", "two::Message"),
            ])
            .projected_type("/deps/shared", "Message")
            .is_none()
        );
    }

    #[test]
    fn type_parameter_guard_rejects_declared_generics() {
        let parameters = vec![GenericParamDef {
            name: "T".to_owned(),
            kind: GenericParamDefKind::Type {
                bounds: Vec::new(),
                default: None,
                is_synthetic: false,
            },
        }];
        assert!(has_type_parameters(&parameters));
        assert!(!has_type_parameters(&[]));
    }

    #[test]
    fn receiver_kind_preserves_only_plain_self_receivers() {
        let borrowed = |is_mutable| Type::BorrowedRef {
            lifetime: None,
            is_mutable,
            type_: Box::new(Type::Generic("Self".to_owned())),
        };
        assert_eq!(
            receiver_kind(&borrowed(true)).unwrap(),
            Receiver::MutableBorrow
        );
        assert_eq!(receiver_kind(&borrowed(false)).unwrap(), Receiver::Borrow);
        assert_eq!(
            receiver_kind(&Type::Generic("Self".to_owned())).unwrap(),
            Receiver::Move
        );
        assert!(receiver_kind(&Type::Primitive("str".to_owned())).is_err());
    }

    #[test]
    fn failed_impl_witness_declines_bound_functions_with_the_unproven_interface() {
        let mut dependencies = vec![ProjectedDependency {
            name: "witness".to_owned(),
            package: "witness".to_owned(),
            version: "1.0.0".to_owned(),
            items: vec![
                ProjectedItem {
                    namespace: "/deps/witness".to_owned(),
                    name: "Rejected".to_owned(),
                    rust_path: "witness::Rejected".to_owned(),
                    docs: None,
                    kind: ProjectedKind::Interface(ProjectedInterface {
                        methods: Vec::new(),
                        associated_type: None,
                        supertraits: Vec::new(),
                        send: false,
                        sync: false,
                        requires_drop: false,
                        declined_methods: Vec::new(),
                    }),
                },
                ProjectedItem {
                    namespace: "/deps/witness".to_owned(),
                    name: "Proven".to_owned(),
                    rust_path: "witness::Proven".to_owned(),
                    docs: None,
                    kind: ProjectedKind::Interface(ProjectedInterface {
                        methods: Vec::new(),
                        associated_type: None,
                        supertraits: Vec::new(),
                        send: false,
                        sync: false,
                        requires_drop: false,
                        declined_methods: Vec::new(),
                    }),
                },
                ProjectedItem {
                    namespace: "/deps/witness".to_owned(),
                    name: "rejected_total".to_owned(),
                    rust_path: "witness::rejected_total".to_owned(),
                    docs: None,
                    kind: ProjectedKind::Function(ProjectedFunction {
                        name: "rejected_total".to_owned(),
                        generic_parameters: Vec::new(),
                        parameters: vec![ProjectedParameter {
                            name: "value".to_owned(),
                            ty: ProjectedType::Generic("T".to_owned()),
                            borrowed: false,
                            mutable_borrow: false,
                            generic_parameter: Some("T".to_owned()),
                            generic_bounds: vec!["witness::Rejected".to_owned()],
                            generic_interface: Some("witness::Rejected".to_owned()),
                            associated_type: None,
                        }],
                        result: ProjectedType::Int,
                        destination_result: None,
                        error: None,
                        is_async: false,
                        into_future: false,
                        execution_requirements: None,
                        enum_operation: None,
                        error_optional_depth: 0,
                        chain_role: None,
                        receiver: None,
                    }),
                },
            ],
            declined: Vec::new(),
        }];
        let evidence = vec![crate::projection_oracle::ImplProbeEvidence {
            question: crate::projection_oracle::ImplQuestion {
                label: "witness::Rejected".to_owned(),
                source: "compile_error!(\"witness failure\");".to_owned(),
            },
            answer: crate::ProbeAnswer::No,
        }];

        decline_unproven_projected_interfaces(&mut dependencies, &evidence);
        decline_functions_with_missing_generic_interfaces(&mut dependencies);

        assert_eq!(dependencies[0].items[0].rust_path, "witness::Proven");
        assert_eq!(dependencies[0].declined[0].rust_path, "witness::Rejected");
        assert_eq!(
            dependencies[0].declined[0].reason,
            "trait implementation signature is not representable against the resolved dependency"
        );
        assert_eq!(
            dependencies[0].declined[1],
            DeclinedItem {
                rust_path: "witness::rejected_total".to_owned(),
                reason: "generic input references declined interface `witness::Rejected`"
                    .to_owned(),
            }
        );
    }

    #[test]
    fn instantiated_generic_paths_have_distinct_foreign_identity() {
        let id = Id(1);
        let paths = HashMap::from([(
            id,
            ItemSummary {
                crate_id: 0,
                path: vec!["witness".to_owned(), "Wrapper".to_owned()],
                kind: ItemKind::Struct,
            },
        )]);
        let instantiated = |primitive: &str| {
            Type::ResolvedPath(RustdocPath {
                path: "witness::Wrapper".to_owned(),
                id,
                args: Some(Box::new(GenericArgs::AngleBracketed {
                    args: vec![GenericArg::Type(Type::Primitive(primitive.to_owned()))],
                    constraints: Vec::new(),
                })),
            })
        };
        let index = HashMap::new();
        let generics = BTreeMap::new();

        let left = project_type(&instantiated("u8"), &index, &paths, &generics).unwrap();
        let right = project_type(&instantiated("u16"), &index, &paths, &generics).unwrap();

        assert_ne!(left, right);
        assert!(matches!(
            left,
            ProjectedType::Foreign {
                rust_path, name, ..
            }
                if rust_path == "witness::Wrapper<u8>" && name.starts_with("Wrapper-")
        ));
        assert!(matches!(
            right,
            ProjectedType::Foreign {
                rust_path, name, ..
            }
                if rust_path == "witness::Wrapper<u16>" && name.starts_with("Wrapper-")
        ));
    }
    #[test]
    fn standard_aggregates_project_recursively() {
        let vec_id = Id(1);
        let string_id = Id(2);
        let map_id = Id(3);
        let option_id = Id(4);
        let paths = HashMap::from([
            (
                vec_id,
                ItemSummary {
                    crate_id: 0,
                    path: vec!["alloc".to_owned(), "vec".to_owned(), "Vec".to_owned()],
                    kind: ItemKind::Struct,
                },
            ),
            (
                string_id,
                ItemSummary {
                    crate_id: 0,
                    path: vec!["alloc".to_owned(), "string".to_owned(), "String".to_owned()],
                    kind: ItemKind::Struct,
                },
            ),
            (
                map_id,
                ItemSummary {
                    crate_id: 0,
                    path: vec![
                        "std".to_owned(),
                        "collections".to_owned(),
                        "HashMap".to_owned(),
                    ],
                    kind: ItemKind::Struct,
                },
            ),
            (
                option_id,
                ItemSummary {
                    crate_id: 0,
                    path: vec!["core".to_owned(), "option".to_owned(), "Option".to_owned()],
                    kind: ItemKind::Enum,
                },
            ),
        ]);
        let resolved = |id, path: &str, arguments: Vec<Type>| {
            Type::ResolvedPath(RustdocPath {
                path: path.to_owned(),
                id,
                args: Some(Box::new(GenericArgs::AngleBracketed {
                    args: arguments.into_iter().map(GenericArg::Type).collect(),
                    constraints: Vec::new(),
                })),
            })
        };
        let string = resolved(string_id, "alloc::string::String", Vec::new());
        let vector = resolved(vec_id, "alloc::vec::Vec", vec![string.clone()]);
        let map = resolved(
            map_id,
            "std::collections::HashMap",
            vec![string, Type::Primitive("u16".to_owned())],
        );
        let optional = resolved(
            option_id,
            "core::option::Option",
            vec![Type::Primitive("u16".to_owned())],
        );
        let index = HashMap::new();
        let generics = BTreeMap::new();

        assert!(matches!(
            project_type(&vector, &index, &paths, &generics).unwrap(),
            ProjectedType::Sequence { item, .. } if *item == ProjectedType::String
        ));
        assert!(matches!(
            project_type(&map, &index, &paths, &generics).unwrap(),
            ProjectedType::Mapping { key, value, ordered: false, .. }
                if *key == ProjectedType::String
                    && *value == ProjectedType::RustInt("u16".to_owned())
        ));
        assert_eq!(
            project_type(&optional, &index, &paths, &generics).unwrap(),
            ProjectedType::Optional(Box::new(ProjectedType::RustInt("u16".to_owned())))
        );
        assert_eq!(
            project_type(
                &Type::Tuple(vec![
                    Type::Primitive("u16".to_owned()),
                    Type::Primitive("u16".to_owned()),
                ]),
                &index,
                &paths,
                &generics,
            )
            .unwrap(),
            ProjectedType::Tuple(vec![
                ProjectedType::RustInt("u16".to_owned()),
                ProjectedType::RustInt("u16".to_owned()),
            ])
        );
    }

    #[test]
    fn method_lookup_keeps_colliding_foreign_types_in_their_namespaces() {
        let projection: Projection = serde_json::from_value(json!({
            "cache_identity": "test",
            "containment": "Unavailable",
            "dependencies": [{
                "name": "witness",
                "package": "witness",
                "version": "1.0.0",
                "declined": [],

                "items": [
                    {
                        "namespace": "/deps/witness/async",
                        "name": "Response",
                        "rust_path": "witness::async::Response",
                        "docs": null,
                        "kind": {"ForeignType": {"methods": [{
                            "name": "touch",
                            "parameters": [],
                            "result": "None",
                            "error": null,
                            "is_async": false,
                            "receiver": "Borrow"
                        }]}}
                    },
                    {
                        "namespace": "/deps/witness/blocking",
                        "name": "Response",
                        "rust_path": "witness::blocking::Response",
                        "docs": null,
                        "kind": {"ForeignType": {"methods": [{
                            "name": "touch",
                            "parameters": [],
                            "result": "None",
                            "error": null,
                            "is_async": false,
                            "receiver": "MutableBorrow"
                        }]}}
                    }
                ]
            }]
        }))
        .unwrap();

        assert_eq!(
            projection
                .method("/deps/witness/async", "Response", "touch", false)
                .and_then(|method| method.receiver),
            Some(Receiver::Borrow)
        );
        assert_eq!(
            projection
                .method("/deps/witness/blocking", "Response", "touch", false)
                .and_then(|method| method.receiver),
            Some(Receiver::MutableBorrow)
        );
    }
    #[test]
    fn transitive_type_owner_must_be_declared_at_one_version() {
        let directory =
            std::env::temp_dir().join(format!("terrane-transitive-owner-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        fs::write(
            directory.join("Cargo.lock"),
            "version = 4\n\n[[package]]\nname = \"reqwest\"\nversion = \"0.12.28\"\n\n[[package]]\nname = \"http\"\nversion = \"1.3.1\"\n",
        )
        .unwrap();
        let dependency = |name: &str| RustDependency {
            name: name.to_owned(),
            package: name.to_owned(),
            version: "=1.0.0".to_owned(),
            features: Vec::new(),
            default_features: true,
            target: None,
            effects: Vec::new(),
        };
        let response_status = || ProjectedDependency {
            name: "reqwest".to_owned(),
            package: "reqwest".to_owned(),
            version: "0.12.28".to_owned(),
            items: vec![ProjectedItem {
                namespace: "/deps/reqwest".to_owned(),
                name: "status".to_owned(),
                rust_path: "reqwest::Response::status".to_owned(),
                docs: None,
                kind: ProjectedKind::Function(ProjectedFunction {
                    name: "status".to_owned(),
                    generic_parameters: Vec::new(),
                    parameters: Vec::new(),
                    result: ProjectedType::Foreign {
                        rust_path: "http::StatusCode".to_owned(),
                        name: "StatusCode".to_owned(),
                        base_rust_path: "http::StatusCode".to_owned(),
                        arguments: Vec::new(),
                    },
                    destination_result: None,
                    error: None,
                    is_async: false,
                    into_future: false,
                    execution_requirements: None,
                    enum_operation: None,
                    error_optional_depth: 0,
                    chain_role: None,
                    receiver: None,
                }),
            }],
            declined: Vec::new(),
        };

        let mut undeclared = vec![response_status()];
        enforce_transitive_reachability(
            &mut undeclared,
            &[dependency("reqwest")],
            &directory,
            false,
        )
        .unwrap();
        assert!(undeclared[0].items.is_empty());
        assert!(
            undeclared[0].declined[0]
                .reason
                .contains("undeclared crate `http` at resolved version `1.3.1`")
        );

        let mut declared = vec![response_status()];
        enforce_transitive_reachability(
            &mut declared,
            &[dependency("reqwest"), dependency("http")],
            &directory,
            false,
        )
        .unwrap();
        assert_eq!(declared[0].items.len(), 1);
        fs::write(
            directory.join("Cargo.lock"),
            "version = 4\n\n[[package]]\nname = \"reqwest\"\nversion = \"0.12.28\"\n\n[[package]]\nname = \"http\"\nversion = \"1.3.1\"\n\n[[package]]\nname = \"http\"\nversion = \"0.2.12\"\n",
        )
        .unwrap();
        let mut conflicting = [response_status()];
        let error = enforce_transitive_reachability(
            &mut conflicting,
            &[dependency("reqwest"), dependency("http")],
            &directory,
            false,
        )
        .unwrap_err();
        assert!(error.message.contains("http"));
        assert!(error.message.contains("0.2.12"));
        assert!(error.message.contains("1.3.1"));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn function_signatures_keep_same_named_foreign_types_distinct() {
        let projection: Projection = serde_json::from_value(json!({
            "cache_identity": "test",
            "containment": "Unavailable",
            "dependencies": [{
                "name": "witness",
                "package": "witness",
                "version": "1.0.0",
                "declined": [],
                "items": [{
                    "namespace": "/deps/witness",
                    "name": "cross",
                    "rust_path": "witness::cross",
                    "docs": null,
                    "kind": {"Function": {
                        "name": "cross",
                        "parameters": [
                            {
                                "name": "left",
                                "ty": {"Foreign": {
                                    "rust_path": "witness::left::Response",
                                    "name": "Response"
                                }},
                                "borrowed": false,
                                "mutable_borrow": false
                            },
                            {
                                "name": "right",
                                "ty": {"Foreign": {
                                    "rust_path": "witness::right::Response",
                                    "name": "Response"
                                }},
                                "borrowed": false,
                                "mutable_borrow": false
                            }
                        ],
                        "result": "None",
                        "error": null,
                        "is_async": false,
                        "receiver": null
                    }}
                }]
            }]
        }))
        .unwrap();

        assert_eq!(
            projection.foreign_imports("/deps/witness"),
            BTreeMap::from([
                (
                    "witness-left-Response".to_owned(),
                    "witness::left::Response".to_owned()
                ),
                (
                    "witness-right-Response".to_owned(),
                    "witness::right::Response".to_owned()
                )
            ])
        );
        let sources = projection
            .source_for_imports(&BTreeMap::from([(
                "/deps/witness".to_owned(),
                BTreeSet::from(["cross".to_owned()]),
            )]))
            .unwrap();
        assert!(sources[0].1.contains(
            "function cross throws dependency-panic; left witness-left-Response, right witness-right-Response"
        ));
    }

    #[test]
    fn projected_source_cycles_are_diagnostic() {
        let cycle = Projection::order_projected_sources(vec![
            (
                "/deps/one".to_owned(),
                String::new(),
                BTreeSet::from(["/deps/two".to_owned()]),
            ),
            (
                "/deps/two".to_owned(),
                String::new(),
                BTreeSet::from(["/deps/one".to_owned()]),
            ),
        ])
        .unwrap_err();

        assert_eq!(
            cycle,
            "projected dependency source namespaces contain an import cycle: \
             /deps/one -> [/deps/two]; /deps/two -> [/deps/one]; this is the recorded \
             `projection/mutually-referential-namespace-sources` limitation"
        );
    }
    #[test]
    fn wider_primitives_project_without_narrowing() {
        let paths = HashMap::new();
        let index = HashMap::new();
        let generics = BTreeMap::new();

        assert_eq!(
            project_type(&Type::Primitive("u8".to_owned()), &index, &paths, &generics).unwrap(),
            ProjectedType::RustInt("u8".to_owned())
        );
        assert_eq!(
            project_type(
                &Type::Primitive("f32".to_owned()),
                &index,
                &paths,
                &generics
            )
            .unwrap(),
            ProjectedType::Float32
        );
        assert_eq!(
            project_type(
                &Type::Primitive("char".to_owned()),
                &index,
                &paths,
                &generics
            )
            .unwrap(),
            ProjectedType::Char
        );
    }

    #[test]
    fn rustdoc_schema_mismatch_is_explicit() {
        let document = json!({
            "root": 0,
            "crate_version": "1.0.0",
            "includes_private": false,
            "index": {},
            "paths": {},
            "external_crates": {},
            "target": {"triple": "x86_64-unknown-linux-gnu", "target_features": []},
            "format_version": rustdoc_types::FORMAT_VERSION + 1
        });
        let dependency = RustDependency {
            name: "witness".to_owned(),
            package: "witness".to_owned(),
            version: "=1.0.0".to_owned(),
            features: Vec::new(),
            default_features: true,
            target: None,
            effects: Vec::new(),
        };

        let error = parse_rustdoc(&dependency, &serde_json::to_vec(&document).unwrap())
            .expect_err("an unsupported rustdoc schema must fail");

        assert!(error.message.contains("schema mismatch"));
        assert!(
            error
                .message
                .contains(&format!("format {}", rustdoc_types::FORMAT_VERSION + 1))
        );
        assert!(error.message.contains(&format!(
            "expected format {}",
            rustdoc_types::FORMAT_VERSION
        )));
    }

    #[test]
    fn malformed_rustdoc_json_is_not_silently_declined() {
        let dependency = RustDependency {
            name: "witness".to_owned(),
            package: "witness".to_owned(),
            version: "=1.0.0".to_owned(),
            features: Vec::new(),
            default_features: true,
            target: None,
            effects: Vec::new(),
        };
        let malformed = format!(r#"{{"format_version":{}}}"#, rustdoc_types::FORMAT_VERSION);

        let error = parse_rustdoc(&dependency, malformed.as_bytes())
            .expect_err("malformed rustdoc JSON must fail");

        assert!(error.message.contains("schema mismatch"));
        assert!(error.message.contains(&format!(
            "expected rustdoc format {}",
            rustdoc_types::FORMAT_VERSION
        )));
    }

    #[test]
    fn remote_artifact_requires_exact_projection_inputs() {
        let dependency = RustDependency {
            name: "witness".to_owned(),
            package: "witness-package".to_owned(),
            version: "=1.2.3".to_owned(),
            features: vec!["derive".to_owned()],
            default_features: false,
            target: Some("cfg(unix)".to_owned()),
            effects: vec!["filesystem".to_owned()],
        };
        let mut payload = Projection {
            cache_identity: "exact".to_owned(),
            content_hash: String::new(),
            dependencies: Vec::new(),
            bound_dependencies: Vec::new(),
            containment: Containment::Enforced,
            source: ProjectionSource::Local,
            probes: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution::default(),
            removed: Vec::new(),
        };
        payload.content_hash = projection_content_hash(&payload).unwrap();
        let artifact = ProjectionArtifact {
            format: 1,
            cache_identity: "exact".to_owned(),
            content_hash: payload.content_hash.clone(),
            target: "x86_64-unknown-linux-gnu".to_owned(),
            build_toolchain: crate::BUILD_TOOLCHAIN.to_owned(),
            rustdoc_toolchain: crate::RUSTDOC_TOOLCHAIN.to_owned(),
            rustdoc_format: rustdoc_types::FORMAT_VERSION,
            projection_schema: super::PROJECTION_SCHEMA.to_owned(),
            dependencies: vec![ArtifactDependency::from(&dependency)],
            projection: payload,
        };

        let projection = validate_projection_artifact(
            artifact.clone(),
            "exact",
            "x86_64-unknown-linux-gnu",
            std::slice::from_ref(&dependency),
            Containment::Unavailable,
        )
        .expect("exact artifact metadata must be admitted");
        assert_eq!(projection.source, ProjectionSource::Remote);
        assert_eq!(projection.containment, Containment::Unavailable);

        let reject = |candidate| {
            validate_projection_artifact(
                candidate,
                "exact",
                "x86_64-unknown-linux-gnu",
                std::slice::from_ref(&dependency),
                Containment::Unavailable,
            )
            .unwrap_err()
        };

        let mut mismatched = artifact.clone();
        mismatched.dependencies[0].version = "=9.9.9".to_owned();
        assert_eq!(
            reject(mismatched),
            "dependency metadata mismatch for `witness`"
        );

        let mut mismatched = artifact.clone();
        mismatched.dependencies[0].effects = vec!["network".to_owned()];
        assert_eq!(
            reject(mismatched),
            "dependency metadata mismatch for `witness`"
        );

        let mut mismatched = artifact.clone();
        mismatched.dependencies[0].features = vec!["different".to_owned()];
        assert_eq!(
            reject(mismatched),
            "feature metadata mismatch for `witness`"
        );

        let mut mismatched = artifact.clone();
        mismatched.dependencies[0].default_features = true;
        assert_eq!(
            reject(mismatched),
            "feature metadata mismatch for `witness`"
        );

        let mut mismatched = artifact.clone();
        mismatched.dependencies[0].target = Some("cfg(windows)".to_owned());
        assert_eq!(
            reject(mismatched),
            "dependency target mismatch for `witness`: expected `cfg(unix)`, found `cfg(windows)`"
        );

        let mut corrupt = artifact;
        corrupt.content_hash = "not-the-payload-hash".to_owned();
        let reason = validate_projection_artifact(
            corrupt,
            "exact",
            "x86_64-unknown-linux-gnu",
            &[dependency],
            Containment::Unavailable,
        )
        .unwrap_err();
        assert!(reason.contains("content hash mismatch"));
    }

    #[test]
    fn projection_history_retains_removed_members_across_checks() {
        let directory =
            std::env::temp_dir().join(format!("terrane-projection-history-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let dependency = |version: &str, items: Vec<ProjectedItem>| ProjectedDependency {
            name: "fixture".to_owned(),
            package: "fixture".to_owned(),
            version: version.to_owned(),
            items,
            declined: Vec::new(),
        };
        let item = ProjectedItem {
            namespace: "/deps/fixture".to_owned(),
            name: "removed".to_owned(),
            rust_path: "fixture::removed".to_owned(),
            docs: None,
            kind: ProjectedKind::ForeignType {
                boundary: ProjectedBoundaryCapabilities::default(),
                displayable: false,
                methods: vec![ProjectedFunction {
                    name: "read".to_owned(),
                    generic_parameters: Vec::new(),
                    parameters: Vec::new(),
                    result: ProjectedType::None,
                    destination_result: None,
                    error: None,
                    is_async: false,
                    into_future: false,
                    execution_requirements: None,
                    enum_operation: None,
                    error_optional_depth: 0,
                    chain_role: None,
                    receiver: Some(Receiver::Borrow),
                }],
                static_methods: vec![ProjectedFunction {
                    name: "create".to_owned(),
                    generic_parameters: Vec::new(),
                    parameters: Vec::new(),
                    result: ProjectedType::None,
                    destination_result: None,
                    error: None,
                    is_async: false,
                    into_future: false,
                    execution_requirements: None,
                    enum_operation: None,
                    error_optional_depth: 0,
                    chain_role: None,
                    receiver: None,
                }],
                cloneable: false,
                send: false,
                sync: false,
            },
        };
        let mut old = Projection {
            cache_identity: "old".to_owned(),
            content_hash: String::new(),
            dependencies: vec![dependency("1.0.0", vec![item])],
            bound_dependencies: Vec::new(),
            containment: Containment::Unavailable,
            source: ProjectionSource::Local,
            probes: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution::default(),
            removed: Vec::new(),
        };
        old.content_hash = projection_content_hash(&old).unwrap();
        apply_projection_history(&directory, &mut old).unwrap();
        let mut current = Projection {
            cache_identity: "current".to_owned(),
            content_hash: String::new(),
            dependencies: vec![dependency("2.0.0", Vec::new())],
            bound_dependencies: Vec::new(),
            containment: Containment::Unavailable,
            source: ProjectionSource::Local,
            probes: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution::default(),
            removed: Vec::new(),
        };
        current.content_hash = projection_content_hash(&current).unwrap();
        apply_projection_history(&directory, &mut current).unwrap();
        assert_eq!(
            current
                .removed
                .iter()
                .map(|item| item.name.as_str())
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["removed", "removed.read", "removed::create"])
        );
        current.removed.clear();
        apply_projection_history(&directory, &mut current).unwrap();
        assert_eq!(current.removed.len(), 3);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn projection_history_keeps_content_origin_across_cache_hits() {
        let directory =
            std::env::temp_dir().join(format!("terrane-projection-origin-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let generated = ProjectionResolution {
            outcome: ResolutionOutcome::LocalRustdoc,
            events: Vec::new(),
        };
        let mut projection = Projection {
            cache_identity: "stable-identity".to_owned(),
            content_hash: String::new(),
            dependencies: Vec::new(),
            bound_dependencies: vec![ProjectedBoundDependency {
                name: "serde_core".to_owned(),
                package: "serde_core".to_owned(),
                version: "1.0.229".to_owned(),
            }],
            containment: Containment::Unavailable,
            source: ProjectionSource::Local,
            probes: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: generated.clone(),
            removed: Vec::new(),
        };
        projection.content_hash = projection_content_hash(&projection).unwrap();
        apply_projection_history(&directory, &mut projection).unwrap();
        projection.resolution = ProjectionResolution {
            outcome: ResolutionOutcome::ExactCache,
            events: Vec::new(),
        };
        apply_projection_history(&directory, &mut projection).unwrap();
        let history: ProjectionHistory =
            serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
                .unwrap();
        assert_eq!(history.resolution, Some(generated));
        assert_eq!(history.format, 3);
        assert_eq!(history.bound_dependencies, projection.bound_dependencies);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn projection_history_migrates_provenance_and_detects_replay_drift() {
        let directory = std::env::temp_dir().join(format!(
            "terrane-projection-provenance-{}",
            std::process::id()
        ));
        fs::create_dir_all(&directory).unwrap();
        fs::write(
            directory.join("terrane-projection.lock"),
            b"{\"format\":1,\"dependencies\":[],\"removed\":[]}\n",
        )
        .unwrap();
        let mut projection = Projection {
            cache_identity: "stable-identity".to_owned(),
            content_hash: String::new(),
            dependencies: Vec::new(),
            bound_dependencies: Vec::new(),
            containment: Containment::Unavailable,
            source: ProjectionSource::Local,
            probes: Vec::new(),
            probe_wall_time_ms: 0,
            resolution: ProjectionResolution::default(),
            removed: Vec::new(),
        };
        projection.content_hash = projection_content_hash(&projection).unwrap();
        apply_projection_history(&directory, &mut projection).unwrap();

        let migrated: ProjectionHistory =
            serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
                .unwrap();
        assert_eq!(migrated.format, 3);
        assert_eq!(migrated.source, Some(ProjectionSource::Local));
        assert_eq!(migrated.rustdoc_format, Some(rustdoc_types::FORMAT_VERSION));
        assert_eq!(
            migrated.content_hash.as_deref(),
            Some(projection.content_hash.as_str())
        );

        projection.source = ProjectionSource::Remote;
        apply_projection_history(&directory, &mut projection).unwrap();
        let changed_source: ProjectionHistory =
            serde_json::from_slice(&fs::read(directory.join("terrane-projection.lock")).unwrap())
                .unwrap();
        assert_eq!(changed_source.source, Some(ProjectionSource::Remote));

        projection.probe_wall_time_ms = 1;
        projection.content_hash = projection_content_hash(&projection).unwrap();
        let error = apply_projection_history(&directory, &mut projection).unwrap_err();
        assert!(error.message.contains("projection replay mismatch"));
        assert!(error.message.contains("previously produced"));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn projection_cache_retains_a_bounded_history_and_unrelated_files() {
        let directory =
            std::env::temp_dir().join(format!("terrane-projection-prune-{}", std::process::id()));
        let retained = directory.join("projection-current.json");
        let unrelated = directory.join("Cargo.lock");
        fs::create_dir_all(&directory).unwrap();
        fs::write(&retained, b"current").unwrap();
        for index in 0..6 {
            fs::write(
                directory.join(format!("projection-previous-{index}.json")),
                b"previous",
            )
            .unwrap();
        }
        fs::write(&unrelated, b"lock").unwrap();

        prune_projection_cache(&directory, &retained).unwrap();

        let projection_count = fs::read_dir(&directory)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("projection-"))
            })
            .count();
        assert_eq!(projection_count, super::MAX_PROJECTION_CACHE_RECORDS);
        assert!(retained.exists());
        assert!(unrelated.exists());
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn dynamic_trait_shape_rejects_multiple_non_auto_traits() {
        let bound = |id, path: &str| GenericBound::TraitBound {
            trait_: RustdocPath {
                path: path.to_owned(),
                id: Id(id),
                args: None,
            },
            generic_params: Vec::new(),
            modifier: TraitBoundModifier::None,
        };
        let bounds = [bound(1, "witness::One"), bound(2, "witness::Two")];
        let error =
            projectable_interface_bound(&bounds, &HashMap::new(), &HashMap::new()).unwrap_err();
        assert_eq!(
            error,
            "generic input requires one projectable interface bound"
        );
    }
}
