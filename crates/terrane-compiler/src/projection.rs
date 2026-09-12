use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;
use std::time::Duration;

use rustdoc_types::{
    AssocItemConstraintKind, Crate as RustdocCrate, Function, GenericArg, GenericArgs,
    GenericBound, GenericParamDef, GenericParamDefKind, Id, Impl, Item, ItemEnum, ItemSummary,
    Path as RustdocPath, Struct, Term, Type, VariantKind, Visibility, WherePredicate,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{InvocationMode, RustDependency};

pub use crate::RUSTDOC_TOOLCHAIN;
const PROJECTION_SCHEMA: &str = "40";
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Containment {
    Enforced,
    Unavailable,
}

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
        cloneable: bool,
        #[serde(default)]
        send: bool,
        #[serde(default)]
        sync: bool,
    },
    Interface(ProjectedInterface),
    Enum {
        data_carrying: bool,
        comparable: bool,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedInterface {
    pub methods: Vec<ProjectedInterfaceMethod>,
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
    pub docs: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChainRole {
    Root,
    Continue,
    Terminal,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedFunction {
    pub name: String,
    pub parameters: Vec<ProjectedParameter>,
    pub result: ProjectedType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_result: Option<ProjectedDestinationResult>,
    pub error: Option<String>,
    pub is_async: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_requirements: Option<ProjectedExecutionRequirements>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chain_role: Option<ChainRole>,
    pub receiver: Option<Receiver>,
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
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Receiver {
    Borrow,
    MutableBorrow,
    Move,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedType {
    None,
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
            Self::Generic(name) | Self::FixedInt(name) | Self::RustInt(name) => name.clone(),
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
}

impl ProjectedType {
    #[must_use]
    pub fn terrane_name(&self) -> String {
        match self {
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
            Self::BoxedInterface { name, .. }
            | Self::Generic(name)
            | Self::Foreign { name, .. } => name.clone(),
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

impl Projection {
    #[must_use]
    pub fn source_for_imports(
        &self,
        imports: &BTreeMap<String, BTreeSet<String>>,
    ) -> Vec<(String, String)> {
        let all_items = self
            .dependencies
            .iter()
            .flat_map(|dependency| dependency.items.iter())
            .collect::<Vec<_>>();
        let imports = expanded_source_imports(&all_items, imports);
        let mut sources = Vec::new();
        for (namespace, names) in &imports {
            let selected = all_items
                .iter()
                .copied()
                .filter(|item| item.namespace == *namespace && names.contains(&item.name))
                .collect::<Vec<_>>();
            if selected.is_empty() {
                continue;
            }
            let foreign = collect_source_foreign(&all_items, &selected);
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
                (dependency_count, aliases.get(*rust_path))
            });
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
                render_foreign_declaration(&mut text, namespace, name, projected_item, &aliases);
            }
            for item in selected {
                if let ProjectedKind::Function(function) = &item.kind {
                    render_function(&mut text, function, true, 0, &aliases, None);
                }
            }
            sources.push((namespace.clone(), text));
        }
        sources
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

    #[must_use]
    pub fn item(&self, namespace: &str, name: &str) -> Option<&ProjectedItem> {
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| item.namespace == namespace && item.name == name)
    }
    #[must_use]
    pub(crate) fn dependency_name(&self, namespace: &str, name: &str) -> Option<&str> {
        self.dependencies
            .iter()
            .find(|dependency| {
                dependency
                    .items
                    .iter()
                    .any(|item| item.namespace == namespace && item.name == name)
            })
            .map(|dependency| dependency.name.as_str())
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
        let mut matches = dependencies
            .iter()
            .flat_map(|dependency| &dependency.declined)
            .filter(|item| match exact_path {
                Some(path) => item.rust_path == path,
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

fn expanded_source_imports(
    all_items: &[&ProjectedItem],
    imports: &BTreeMap<String, BTreeSet<String>>,
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
        for (rust_path, name) in collect_source_foreign(all_items, &selected) {
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
            } => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
                for method in methods.iter().chain(static_methods) {
                    collect_foreign_function(method, &mut foreign);
                }
            }
            ProjectedKind::Interface(interface) => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
                for method in &interface.methods {
                    collect_foreign_function(&method.function, &mut foreign);
                }
            }
            ProjectedKind::Enum { .. } => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
            }
        }
    }
    loop {
        let previous_len = foreign.len();
        let referenced = foreign.keys().cloned().collect::<Vec<_>>();
        for rust_path in referenced {
            let Some(ProjectedItem {
                kind:
                    ProjectedKind::ForeignType {
                        methods,
                        static_methods,
                        ..
                    },
                ..
            }) = all_items
                .iter()
                .copied()
                .find(|item| item.rust_path == rust_path)
            else {
                continue;
            };
            for method in methods.iter().chain(static_methods) {
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
            rust_path, name, ..
        } => {
            foreign.insert(rust_path.clone(), name.clone());
        }
        ProjectedType::BoxedInterface {
            trait_path, name, ..
        } => {
            foreign.insert(trait_path.clone(), name.clone());
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
        _ => {}
    }
}
fn render_foreign_declaration(
    output: &mut String,
    namespace: &str,
    name: &str,
    projected_item: Option<&ProjectedItem>,
    aliases: &BTreeMap<String, String>,
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
            if let Some(ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }) = projected_kind
            {
                for method in methods {
                    render_function(output, method, false, 4, aliases, None);
                }
                for method in static_methods {
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
        }
        | ProjectedType::BoxedInterface {
            trait_path: rust_path,
            name,
            ..
        } => foreign_aliases
            .get(rust_path)
            .cloned()
            .unwrap_or_else(|| name.clone()),
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

    let mut rustdocs = Vec::new();
    for dependency in dependencies {
        let package_spec = dependency.version.strip_prefix('=').map_or_else(
            || dependency.package.clone(),
            |version| format!("{}@{version}", dependency.package),
        );
        run_cargo(
            &workspace,
            &[
                "rustdoc",
                "-p",
                &package_spec,
                "--target-dir",
                "target/rustdoc-57",
                "--offline",
                "--frozen",
                "--",
                "-Z",
                "unstable-options",
                "--output-format",
                "json",
            ],
            CargoToolchain::RustdocNightly,
            if sandbox == Containment::Enforced {
                CargoExecution::Contained
            } else {
                CargoExecution::Host
            },
        )?;
        let crate_name = dependency.package.replace('-', "_");
        let rustdoc_path = workspace
            .join("target/rustdoc-57/doc")
            .join(format!("{crate_name}.json"));
        let bytes = fs::read(&rustdoc_path).map_err(|error| ProjectionError {
            message: format!(
                "cannot read rustdoc projection `{}`: {error}",
                rustdoc_path.display()
            ),
        })?;
        let document = parse_rustdoc(dependency, &bytes)?;
        let public_paths = rustdoc_public_paths(&document);
        rustdocs.push((dependency, document, public_paths));
    }
    let mut canonical_public_paths = BTreeMap::new();
    for (_, document, public_paths) in &rustdocs {
        for (id, public_path) in public_paths {
            if let Some(summary) = document
                .paths
                .get(id)
                .filter(|summary| summary.crate_id == 0)
            {
                canonical_public_paths.insert(summary.path.join("::"), public_path.clone());
            }
        }
    }
    let mut projected = rustdocs
        .iter()
        .map(|(dependency, document, public_paths)| {
            project_rustdoc(dependency, document, public_paths, &canonical_public_paths)
        })
        .collect::<Vec<_>>();
    enforce_transitive_reachability(&mut projected, dependencies, &workspace)?;
    canonicalize_projected_type_names(&mut projected);
    validate_unique_projected_type_identities(&projected)?;
    let auto_trait_questions = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| matches!(item.kind, ProjectedKind::ForeignType { .. }))
        .flat_map(|item| {
            ["Send", "Sync"]
                .into_iter()
                .map(|rust_bound| crate::projection_oracle::BoundQuestion {
                    rust_type: item.rust_path.clone(),
                    rust_bound: rust_bound.to_owned(),
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
                if let ProjectedKind::ForeignType { send, sync, .. } = &mut item.kind {
                    match evidence.question.rust_bound.as_str() {
                        "Send" => *send = satisfied,
                        "Sync" => *sync = satisfied,
                        _ => {}
                    }
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
    crate::cargo_toolchain::configure_cargo_command(&mut command);
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

fn prefer_public_path(public_paths: &mut BTreeMap<Id, String>, id: Id, candidate: String) {
    public_paths
        .entry(id)
        .and_modify(|existing| {
            let candidate_is_prelude = candidate.split("::").any(|segment| segment == "prelude");
            let existing_is_prelude = existing.split("::").any(|segment| segment == "prelude");
            let candidate_depth = candidate.matches("::").count();
            let existing_depth = existing.matches("::").count();
            if (existing_is_prelude && !candidate_is_prelude)
                || (candidate_is_prelude == existing_is_prelude
                    && (candidate_depth < existing_depth
                        || (candidate_depth == existing_depth && candidate < *existing)))
            {
                existing.clone_from(&candidate);
            }
        })
        .or_insert(candidate);
}

fn record_public_module_items(
    index: &HashMap<Id, Item>,
    module_id: Id,
    module_path: &[String],
    public_paths: &mut BTreeMap<Id, String>,
    visiting: &mut BTreeSet<Id>,
) {
    if !visiting.insert(module_id) {
        return;
    }
    let Some(Item {
        inner: ItemEnum::Module(module),
        ..
    }) = index.get(&module_id)
    else {
        visiting.remove(&module_id);
        return;
    };
    for child_id in &module.items {
        let Some(item) = index
            .get(child_id)
            .filter(|item| item.visibility == Visibility::Public)
        else {
            continue;
        };
        if let ItemEnum::Use(import) = &item.inner {
            let Some(target_id) = import.id else {
                continue;
            };
            if import.is_glob {
                record_public_module_items(index, target_id, module_path, public_paths, visiting);
            } else {
                let candidate = module_path
                    .iter()
                    .map(String::as_str)
                    .chain(std::iter::once(import.name.as_str()))
                    .collect::<Vec<_>>()
                    .join("::");
                prefer_public_path(public_paths, target_id, candidate);
            }
            continue;
        }
        let Some(name) = item.name.as_deref() else {
            continue;
        };
        let candidate = module_path
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(name))
            .collect::<Vec<_>>()
            .join("::");
        prefer_public_path(public_paths, *child_id, candidate);
    }
    visiting.remove(&module_id);
}

fn enforce_transitive_reachability(
    projected: &mut [ProjectedDependency],
    dependencies: &[RustDependency],
    workspace: &Path,
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
            if let Some(owner) = item_undeclared_owner(&item, &declared) {
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
            } = &mut item.kind
            {
                let owner_path = item.rust_path.clone();
                for candidates in [methods, static_methods] {
                    let mut retained_methods = Vec::new();
                    for method in std::mem::take(candidates) {
                        if let Some(owner) = function_undeclared_owner(&method, &declared) {
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
fn canonicalize_projected_type_names(projected: &mut [ProjectedDependency]) {
    let names = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| matches!(item.kind, ProjectedKind::ForeignType { .. }))
        .map(|item| (item.rust_path.clone(), item.name.clone()))
        .collect::<BTreeMap<_, _>>();
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
            } => methods.iter_mut().chain(static_methods).collect(),
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter_mut()
                .map(|method| &mut method.function)
                .collect(),
            ProjectedKind::Enum { .. } => Vec::new(),
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

fn validate_unique_projected_type_identities(
    projected: &[ProjectedDependency],
) -> Result<(), ProjectionError> {
    let mut identities = BTreeMap::new();
    for item in projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| matches!(item.kind, ProjectedKind::ForeignType { .. }))
    {
        let key = (item.namespace.as_str(), item.name.as_str());
        if let Some(previous) = identities.insert(key, item.rust_path.as_str())
            && previous != item.rust_path
        {
            return Err(ProjectionError {
                message: format!(
                    "projected foreign types `{previous}` and `{}` collide at `{}::{}`; distinct concrete instantiations require distinct projected names",
                    item.rust_path, item.namespace, item.name
                ),
            });
        }
    }
    Ok(())
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
                ProjectedKind::Enum { .. } => {}
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
            } => methods.iter().chain(static_methods).collect(),
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter()
                .map(|method| &method.function)
                .collect(),
            ProjectedKind::Enum { .. } => Vec::new(),
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
    if bound_dependencies.is_empty() {
        return Ok(());
    }
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
) -> Option<&'a str> {
    rust_path_owner(&item.rust_path)
        .filter(|owner| !owner_is_reachable(owner, declared))
        .or_else(|| match &item.kind {
            ProjectedKind::Function(function) => function_undeclared_owner(function, declared),
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter()
                .find_map(|method| function_undeclared_owner(&method.function, declared)),
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => None,
        })
}

fn function_undeclared_owner<'a>(
    function: &'a ProjectedFunction,
    declared: &BTreeSet<String>,
) -> Option<&'a str> {
    function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
        .find_map(|ty| type_undeclared_owner(ty, declared))
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
        } => methods.iter().chain(static_methods).collect(),
        ProjectedKind::Interface(interface) => interface
            .methods
            .iter()
            .map(|method| &method.function)
            .collect(),
        ProjectedKind::Enum { .. } => Vec::new(),
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

fn parse_rustdoc(
    dependency: &RustDependency,
    bytes: &[u8],
) -> Result<RustdocCrate, ProjectionError> {
    let document: RustdocCrate =
        serde_json::from_slice(bytes).map_err(|error| ProjectionError {
            message: format!(
                "rustdoc JSON schema mismatch for `{}`: {error}; expected rustdoc format {} from `{RUSTDOC_TOOLCHAIN}`",
                dependency.package,
                rustdoc_types::FORMAT_VERSION
            ),
        })?;
    if document.format_version != rustdoc_types::FORMAT_VERSION {
        return Err(ProjectionError {
            message: format!(
                "rustdoc JSON schema mismatch for `{}`: format {} is unsupported; expected format {} from `{RUSTDOC_TOOLCHAIN}`",
                dependency.package,
                document.format_version,
                rustdoc_types::FORMAT_VERSION
            ),
        });
    }
    Ok(document)
}

fn rustdoc_public_paths(document: &RustdocCrate) -> BTreeMap<Id, String> {
    let mut public_paths = BTreeMap::new();
    for (module_id, summary) in &document.paths {
        if summary.crate_id != 0
            || !matches!(
                document.index.get(module_id).map(|item| &item.inner),
                Some(ItemEnum::Module(_))
            )
        {
            continue;
        }
        record_public_module_items(
            &document.index,
            *module_id,
            &summary.path,
            &mut public_paths,
            &mut BTreeSet::new(),
        );
    }
    public_paths
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
            ..
        } => {
            *trait_path = rewrite(trait_path);
            for auto_trait in auto_traits.iter_mut() {
                *auto_trait = rewrite(auto_trait);
            }
            *rust_path = format!(
                "Box<dyn {}>",
                std::iter::once(trait_path.as_str())
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
        | ProjectedType::AsyncSinkOutcome => {}
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "interface admission keeps trait-level and member-level evidence together"
)]
fn project_interface(
    declaration: &rustdoc_types::Trait,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
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
    for bound in &declaration.bounds {
        let GenericBound::TraitBound { trait_, .. } = bound else {
            return Err("trait has an unsupported lifetime supertrait".to_owned());
        };
        match paths
            .get(&trait_.id)
            .map(|summary| summary.path.join("::"))
            .as_deref()
        {
            Some("core::marker::Send" | "std::marker::Send") => send = true,
            Some("core::marker::Sync" | "std::marker::Sync") => sync = true,
            Some("core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop") => {
                requires_drop = true;
            }
            Some(path) => return Err(format!("non-marker supertrait `{path}` is deferred")),
            None => return Err("trait has an unresolved supertrait".to_owned()),
        }
    }
    let mut methods = Vec::new();
    let mut declined_methods = Vec::new();
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
            ItemEnum::AssocType { .. } => {
                return Err(format!("trait has unresolved associated type `{name}`"));
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
        let projected = match project_function(function, index, paths, Some(name)) {
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
    let mut source = "struct TerraneProjectionImpl;\n".to_owned();
    if interface.requires_drop {
        source.push_str("impl Drop for TerraneProjectionImpl { fn drop(&mut self) {} }\n");
    }
    writeln!(
        source,
        "impl {} for TerraneProjectionImpl {{",
        item.rust_path
    )
    .expect("writing to a string cannot fail");
    for method in interface.methods.iter().filter(|method| !method.provided) {
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
            write!(source, ", {}: {ty}", parameter.name).expect("writing to a string cannot fail");
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
    source.push_str("}\nfn main() {}\n");
    crate::projection_oracle::ImplQuestion {
        label: item.rust_path.clone(),
        source,
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
    for (id, summary) in paths.iter().filter(|(_, summary)| summary.crate_id == 0) {
        candidates.insert(*id, summary.path.clone());
    }
    for (id, public_path) in public_paths {
        candidates.insert(*id, public_path.split("::").map(str::to_owned).collect());
    }
    let mut items = Vec::new();
    let mut projected_enum_items = Vec::new();
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
            ItemEnum::Function(function) => project_function(function, index, paths, Some(&name))
                .map(|mut projected_function| {
                    if let Some(chain_owner) = project_chain_owner(
                        dependency,
                        function,
                        &projected_function.result,
                        index,
                        paths,
                        public_paths,
                    ) {
                        projected_function.chain_role = Some(ChainRole::Root);
                        projected_associated_items.push(chain_owner);
                    }
                    ProjectedKind::Function(projected_function)
                }),
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
                if !owner_generics.is_empty() {
                    let base_rust_path = rust_path.clone();
                    let arguments = structure
                        .generics
                        .params
                        .iter()
                        .filter_map(|parameter| owner_generics.get(&parameter.name).cloned())
                        .collect::<Vec<_>>();
                    rust_path = format!(
                        "{base_rust_path}<{}>",
                        arguments
                            .iter()
                            .map(ProjectedType::rust_type)
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    owner_generics.insert(
                        "Self".to_owned(),
                        ProjectedType::Foreign {
                            rust_path: rust_path.clone(),
                            name: name.clone(),
                            base_rust_path,
                            arguments,
                        },
                    );
                }
                {
                    let (projected_methods, trait_methods, method_declines) = project_methods(
                        &structure.impls,
                        index,
                        paths,
                        public_paths,
                        &rust_path,
                        &owner_generics,
                    );
                    let (mut methods, mut static_methods): (Vec<_>, Vec<_>) = projected_methods
                        .into_iter()
                        .partition(|method| method.receiver.is_some());
                    promote_async_endpoint_methods(&mut methods);
                    promote_async_endpoint_methods(&mut static_methods);
                    for (trait_path, public_trait_path, method) in trait_methods {
                        let trait_segments = trait_path
                            .split("::")
                            .map(str::to_owned)
                            .collect::<Vec<_>>();
                        let trait_namespace = dependency_namespace(dependency, &trait_segments);
                        let trait_rust_path = extern_rust_path(dependency, &public_trait_path);
                        projected_trait_items.push(ProjectedItem {
                            namespace: trait_namespace,
                            name: method.name.clone(),
                            rust_path: format!(
                                "<{rust_path} as {trait_rust_path}>::{}",
                                method.name
                            ),
                            docs: None,
                            kind: ProjectedKind::Function(method),
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
                    let data_carrying = enumeration.variants.iter().any(|id| {
                        !matches!(
                            index.get(id).map(|variant| &variant.inner),
                            Some(ItemEnum::Variant(variant))
                                if matches!(variant.kind, VariantKind::Plain)
                        )
                    });
                    if !data_carrying {
                        let variant_namespace = dependency_namespace(dependency, &path);
                        for variant_id in &enumeration.variants {
                            let Some(variant) = index.get(variant_id) else {
                                continue;
                            };
                            let Some(variant_name) = variant.name.as_deref() else {
                                continue;
                            };
                            projected_enum_items.push(ProjectedItem {
                                namespace: variant_namespace.clone(),
                                name: variant_name.to_owned(),
                                rust_path: format!("{rust_path}::{variant_name}"),
                                docs: variant.docs.clone(),
                                kind: ProjectedKind::Function(ProjectedFunction {
                                    name: variant_name.to_owned(),
                                    parameters: Vec::new(),
                                    result: ProjectedType::Foreign {
                                        rust_path: rust_path.clone(),
                                        name: name.clone(),
                                        base_rust_path: rust_path.clone(),
                                        arguments: Vec::new(),
                                    },
                                    destination_result: None,
                                    error: None,
                                    is_async: false,
                                    execution_requirements: None,
                                    chain_role: None,
                                    receiver: None,
                                }),
                            });
                        }
                    }
                    Ok(ProjectedKind::Enum {
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
                    project_interface(declaration, index, paths).map(ProjectedKind::Interface)
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
    items.extend(projected_enum_items);
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
    projected_trait_items.sort_by(|left, right| {
        (&left.namespace, &left.name, &left.rust_path).cmp(&(
            &right.namespace,
            &right.name,
            &right.rust_path,
        ))
    });
    let mut projected_index = 0;
    while projected_index < projected_trait_items.len() {
        let first = projected_index;
        let key = (
            projected_trait_items[projected_index].namespace.clone(),
            projected_trait_items[projected_index].name.clone(),
        );
        while projected_index < projected_trait_items.len()
            && projected_trait_items[projected_index].namespace == key.0
            && projected_trait_items[projected_index].name == key.1
        {
            projected_index += 1;
        }
        if projected_index - first == 1
            && !items
                .iter()
                .any(|item| item.namespace == key.0 && item.name == key.1)
        {
            items.push(projected_trait_items[first].clone());
        } else {
            declined.extend(
                projected_trait_items[first..projected_index]
                    .iter()
                    .map(|item| DeclinedItem {
                        rust_path: item.rust_path.clone(),
                        reason: "trait method has multiple concrete receiver implementations"
                            .to_owned(),
                    }),
            );
        }
    }
    let package_root = dependency.package.replace('-', "_");
    let dependency_root = dependency.name.replace('-', "_");
    let normalize = |function: &mut ProjectedFunction| {
        for parameter in &mut function.parameters {
            rewrite_projected_rust_root(&mut parameter.ty, &package_root, &dependency_root);
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
    for item in &mut items {
        match &mut item.kind {
            ProjectedKind::Function(function) => normalize(function),
            ProjectedKind::ForeignType {
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
                }
            }
            ProjectedKind::Enum { .. } => {}
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
    Vec<(String, String, ProjectedFunction)>,
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
        if implementation.is_negative {
            continue;
        }
        let inherent = implementation.trait_.is_none();
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
                let Some(trait_path) = implementation_trait_path(trait_, paths) else {
                    declined.push((
                        name.to_owned(),
                        "trait method is not declared by this dependency crate".to_owned(),
                    ));
                    continue;
                };
                let public_trait_path = public_paths
                    .get(&trait_.id)
                    .cloned()
                    .unwrap_or_else(|| trait_path.clone());
                match project_function_with_generics(
                    function,
                    index,
                    paths,
                    Some(name),
                    owner_generics,
                ) {
                    Ok(mut method) => {
                        let Some(receiver) = method.receiver.take() else {
                            declined
                                .push((name.to_owned(), "trait method has no receiver".to_owned()));
                            continue;
                        };
                        method.parameters.insert(
                            0,
                            ProjectedParameter {
                                name: "receiver".to_owned(),
                                ty: ProjectedType::Foreign {
                                    rust_path: owner_rust_path.to_owned(),
                                    name: owner_rust_path
                                        .rsplit("::")
                                        .next()
                                        .unwrap_or(owner_rust_path)
                                        .to_owned(),
                                    base_rust_path: owner_rust_path
                                        .split_once('<')
                                        .map_or(owner_rust_path, |(base, _)| base)
                                        .to_owned(),
                                    arguments: Vec::new(),
                                },
                                generic_parameter: None,
                                generic_bounds: Vec::new(),
                                borrowed: receiver != Receiver::Move,
                                mutable_borrow: receiver == Receiver::MutableBorrow,
                            },
                        );
                        trait_methods.push((trait_path, public_trait_path, method));
                    }
                    Err(reason) => declined.push((name.to_owned(), reason)),
                }
                continue;
            }
            match project_function_with_generics(function, index, paths, Some(name), owner_generics)
            {
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
) -> Result<ProjectedFunction, String> {
    project_function_inner(function, index, paths, method_name, supplied_generics)
}

fn project_function(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    method_name: Option<&str>,
) -> Result<ProjectedFunction, String> {
    project_function_inner(function, index, paths, method_name, &BTreeMap::new())
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
) -> Result<ProjectedFunction, String> {
    if function.header.is_unsafe {
        return Err("unsafe function".to_owned());
    }
    let (generic_types, destination_result) =
        generic_monomorphisations(function, index, paths, supplied_generics)?;
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
        let (projected_type, impl_trait_parameter) = if let Some(bounds) = impl_trait_bounds(ty) {
            let projectable = projectable_interface_bound(bounds, index, paths)?;
            let Some(Item {
                inner: ItemEnum::Trait(declaration),
                ..
            }) = index.get(&projectable.id)
            else {
                return Err("`impl Trait` input has an unresolved trait bound".to_owned());
            };
            if project_interface(declaration, index, paths).is_err() {
                return Err("`impl Trait` input bound is not a projectable interface".to_owned());
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
                ..
            } => Some((
                format!("TerraneBoxed{parameter_index}"),
                std::iter::once(trait_path.clone())
                    .chain(auto_traits.iter().cloned())
                    .chain(std::iter::once("'static".to_owned()))
                    .collect::<Vec<_>>(),
            )),
            _ => None,
        };
        let generic_parameter = impl_trait_parameter
            .or_else(|| boxed_adapter.as_ref().map(|(name, _)| name.clone()))
            .or_else(|| {
                function.generics.params.iter().find_map(|parameter| {
                    generic_types.get(&parameter.name).and_then(|projected| {
                        (matches!(projected, ProjectedType::Foreign { .. })
                            && type_mentions_generic(ty, &parameter.name))
                        .then(|| parameter.name.clone())
                    })
                })
            });
        let generic_bounds = if let Some((_, bounds)) = &boxed_adapter {
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
        parameters.push(ProjectedParameter {
            name: safe_parameter_name(name),
            ty: projected_type,
            borrowed,
            mutable_borrow,
            generic_parameter,
            generic_bounds,
        });
    }
    let mut error = None;
    let result = if let Some(output) = &function.sig.output {
        if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Result") || name == "Result")
        {
            let arguments = type_arguments(output);
            let value = arguments
                .first()
                .ok_or_else(|| "Result has no value type".to_owned())?;
            if render_rust_type(value, index, paths, &generic_types)?.contains('&') {
                return Err("borrowed result values cannot cross a projected boundary".to_owned());
            }
            error = arguments
                .get(1)
                .and_then(|ty| resolved_name(ty, paths))
                .or_else(|| Some("Error".to_owned()));
            project_type(value, index, paths, &generic_types)?
        } else if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Option") || name == "Option")
        {
            let value = type_arguments(output)
                .into_iter()
                .next()
                .ok_or_else(|| "Option has no value type".to_owned())?;
            ProjectedType::Optional(Box::new(project_type(value, index, paths, &generic_types)?))
        } else {
            project_type(output, index, paths, &generic_types)?
        }
    } else {
        ProjectedType::None
    };
    if matches!(result, ProjectedType::BoxedInterface { .. }) {
        return Err("boxed trait-object results cannot cross a projected boundary".to_owned());
    }
    Ok(ProjectedFunction {
        name: method_name.unwrap_or_default().to_owned(),
        parameters,
        result,
        destination_result,
        error,
        is_async: function.header.is_async,
        execution_requirements: function.header.is_async.then_some(
            ProjectedExecutionRequirements {
                runtime_context: RequirementKnowledge::Unknown,
                wake_support: RequirementKnowledge::Required,
                transfer: RequirementKnowledge::Unknown,
            },
        ),
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
    project_interface(declaration, index, paths)
        .map_err(|_| "generic input bound is not a projectable interface".to_owned())?;
    Ok(trait_)
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

fn is_callback_future_parameter(name: &str, function: &Function) -> bool {
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
        Type::DynTrait(_) => {
            Err("trait objects require an owning `Box<dyn Trait>` parameter".to_owned())
        }
        _ => Err("type has no stable Rust path".to_owned()),
    }
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
    let trait_path = render_resolved_path(trait_, index, paths, generics)?;
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
    let dynamic_bounds = std::iter::once(trait_path.as_str())
        .chain(auto_traits.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" + ");
    Ok(ProjectedType::BoxedInterface {
        rust_path: format!("dyn {dynamic_bounds}"),
        trait_path: trait_path.clone(),
        name: trait_
            .path
            .rsplit("::")
            .next()
            .unwrap_or(&trait_.path)
            .to_owned(),
        auto_traits,
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
        } = project_dyn_interface(dynamic, index, paths, generics)?
        else {
            unreachable!("dynamic interface projection returns its boxed-interface shape");
        };
        return Ok(ProjectedType::BoxedInterface {
            rust_path: format!("Box<{dynamic}>"),
            trait_path,
            name,
            auto_traits,
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
    Ok(ProjectedType::Foreign {
        rust_path,
        name,
        base_rust_path: resolved,
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
) -> Option<String> {
    let summary = paths.get(&trait_path.id)?;
    (summary.crate_id == 0)
        .then(|| summary.path.join("::"))
        .filter(|path| !path.is_empty())
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
        ArtifactDependency, Containment, DeclinedItem, InvocationMode, ProjectedBoundDependency,
        ProjectedDependency, ProjectedFunction, ProjectedInterface, ProjectedItem, ProjectedKind,
        ProjectedType, Projection, ProjectionArtifact, ProjectionHistory, ProjectionResolution,
        ProjectionSource, Receiver, ResolutionOutcome, apply_projection_history,
        decline_unproven_projected_interfaces, enforce_transitive_reachability,
        has_type_parameters, parse_rustdoc, prefer_public_path, project_type,
        projectable_interface_bound, projection_content_hash, prune_projection_cache,
        receiver_kind, resolve, rewrite_rust_bound_root, selected_target,
        validate_projection_artifact, validate_unique_projected_type_identities,
    };
    use crate::RustDependency;

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
    fn colliding_projected_type_identities_fail_explicitly() {
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
                    cloneable: false,
                    send: false,
                    sync: false,
                },
            }],
            declined: Vec::new(),
        };
        let error = validate_unique_projected_type_identities(&[
            dependency("one", "one::Generic<A>"),
            dependency("two", "two::Generic<B>"),
        ])
        .expect_err("distinct concrete identities cannot share one projected name");
        assert!(error.message.contains("distinct concrete instantiations"));
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
    fn failed_impl_witness_declines_only_the_unproven_interface() {
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
                        send: false,
                        sync: false,
                        requires_drop: false,
                        declined_methods: Vec::new(),
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

        assert_eq!(dependencies[0].items[0].rust_path, "witness::Proven");
        assert_eq!(dependencies[0].declined[0].rust_path, "witness::Rejected");
        assert_eq!(
            dependencies[0].declined[0].reason,
            "trait implementation signature is not representable against the resolved dependency"
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
                    execution_requirements: None,
                    chain_role: None,
                    receiver: None,
                }),
            }],
            declined: Vec::new(),
        };

        let mut undeclared = vec![response_status()];
        enforce_transitive_reachability(&mut undeclared, &[dependency("reqwest")], &directory)
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
        let sources = projection.source_for_imports(&BTreeMap::from([(
            "/deps/witness".to_owned(),
            BTreeSet::from(["cross".to_owned()]),
        )]));
        assert!(sources[0].1.contains(
            "function cross throws dependency-panic; left witness-left-Response, right witness-right-Response"
        ));
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
                methods: vec![ProjectedFunction {
                    name: "read".to_owned(),
                    parameters: Vec::new(),
                    result: ProjectedType::None,
                    destination_result: None,
                    error: None,
                    is_async: false,
                    execution_requirements: None,
                    chain_role: None,
                    receiver: Some(Receiver::Borrow),
                }],
                static_methods: vec![ProjectedFunction {
                    name: "create".to_owned(),
                    parameters: Vec::new(),
                    result: ProjectedType::None,
                    destination_result: None,
                    error: None,
                    is_async: false,
                    execution_requirements: None,
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
