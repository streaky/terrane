use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;
use std::time::Duration;

use quote::ToTokens as _;
use rustdoc_types::{
    AssocItemConstraintKind, Attribute, Crate as RustdocCrate, Function, GenericArg, GenericArgs,
    GenericBound, GenericParamDef, GenericParamDefKind, Generics, Id, Impl, Item, ItemEnum,
    ItemKind, ItemSummary, Path as RustdocPath, Struct, Term, Type, VariantKind, Visibility,
    WherePredicate,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{InvocationMode, RustDependency};

mod aliases;
mod borrowed_graph;
mod cache_identity;
mod callable;
use cache_identity::cache_identity;
#[cfg(test)]
use cache_identity::selected_target;
mod data;
mod enum_payload;
mod foreign_impls;
mod macros;
use macros::project_macro;
mod rustdoc_support;
mod source_rendering;
mod type_rendering;
use callable::{
    GenericMonomorphisations, concrete_into_future_output, generic_bounds,
    generic_monomorphisations, project_callable_adapter_bounds, render_generic_bound,
    render_generic_bounds, resolved_path_type_arguments, rust_bound_roots, rust_lifetimes,
    trait_bound_name, type_contains_lifetime_argument, type_mentions_generic,
};
#[cfg(test)]
use callable::{
    builtin_callable_mode, has_supported_callable_trait_shape, is_builtin_clone,
    is_builtin_marker_trait,
};
use data::{
    SourceConstantCache, default_generic_instantiation, extern_rust_path,
    normalize_projected_items, project_rust_constant_expression, project_struct_fields,
    projected_constant_name, source_constant_expression,
};
use enum_payload::{project_enum_payload, project_multi_enum_payload};
use rustdoc_support::{
    descriptive_rust_identity, impl_trait_bounds, implementation_trait_path, implements_trait,
    is_rust_byte_vector_type, is_rust_string_type, project_boundary_capabilities, receiver_kind,
    resolved_name, resolved_path_name, type_implements_deref_target, type_implements_generic_trait,
};
use source_rendering::{
    append_required_nominal_units, collect_foreign_function, collect_source_foreign,
    expanded_source_imports, foreign_aliases, foreign_function_dependency_count,
    inventory_member_syntax_gap, projected_item_for_foreign, projected_item_functions,
    projected_source_dependencies, propagate_partial_contract_requirements, render_demand_sites,
    render_foreign_declaration, render_required_projected_declarations,
    render_unavailable_projection, unavailable_member_map,
};
use type_rendering::{
    canonicalize_rust_path, instantiated_nominal_name, instantiated_type_name, nominal_generics,
    render_resolved_path, render_rust_type,
};
mod history;
#[cfg(test)]
use history::{ProjectionHistory, apply_projection_history};

pub use super::generated_projection::{GeneratedProjectionUnit, generated_projection_units};
pub use crate::RUSTDOC_TOOLCHAIN;
const PROJECTION_SCHEMA: &str = "243";
pub type ProjectedMemberDemands = BTreeMap<(String, String), BTreeSet<String>>;
pub type ProjectionDemandSites = BTreeMap<(String, String, Option<String>), BTreeSet<String>>;
pub const GENERATED_PROJECTION_FILE: &str = "terrane-projection.generated.trn";

const MAX_PROJECTION_CACHE_RECORDS: usize = 4;
const MAX_OWNER_RUSTDOC_CACHE_RECORDS: usize = 16;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Projection {
    pub cache_identity: String,
    #[serde(default)]
    pub content_hash: String,
    pub dependencies: Vec<ProjectedDependency>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bound_dependencies: Vec<ProjectedBoundDependency>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub native_owner_aliases: BTreeMap<String, String>,
    pub containment: Containment,
    #[serde(default)]
    pub source: ProjectionSource,
    #[serde(default)]
    pub probes: Vec<crate::rust_interop::ProbeEvidence>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) partial_declines: Vec<PartialProjectionRecord>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct PartialProjectionRecord {
    rust_path: String,
    reason: String,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    references: BTreeSet<String>,
    projection: PartialProjection,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum PartialProjection {
    Function {
        signature: String,
        generic_constraints: Vec<String>,
        callback_shapes: Vec<PartialCallbackShape>,
    },
    NominalType {
        declaration: String,
        native_kind: String,
        generic_parameters: Vec<String>,
    },
    Namespace,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct PartialCallbackShape {
    parameter: String,
    contract: String,
    methods: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RemovedItem {
    pub namespace: String,
    pub name: String,
    pub previous_version: String,
    pub current_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedItem {
    pub namespace: String,
    pub name: String,
    pub rust_path: String,
    pub docs: Option<String>,
    pub kind: ProjectedKind,
}

#[expect(
    clippy::large_enum_variant,
    reason = "Inline projection metadata avoids allocating every foreign owner or callable solely to equalize enum variant sizes"
)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedKind {
    Function(ProjectedFunction),
    Macro(ProjectedFunction),
    ForeignType {
        methods: Vec<ProjectedFunction>,
        #[serde(default)]
        static_methods: Vec<ProjectedFunction>,
        #[serde(default)]
        constants: Vec<ProjectedConstant>,
        #[serde(default)]
        boundary: ProjectedBoundaryCapabilities,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        fields: Vec<ProjectedField>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constructor: Option<ProjectedFunction>,
        #[serde(default)]
        borrowed_view: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        native_view_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enum_payload: Option<ProjectedEnumPayload>,
        #[serde(default)]
        displayable: bool,
        #[serde(default)]
        cloneable: bool,
        #[serde(default)]
        send: bool,
        #[serde(default)]
        sync: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        generic_parameters: Vec<ProjectedGenericParameter>,
    },
    Interface(ProjectedInterface),
    Enum {
        #[serde(default)]
        methods: Vec<ProjectedFunction>,
        #[serde(default)]
        static_methods: Vec<ProjectedFunction>,
        #[serde(default)]
        constants: Vec<ProjectedConstant>,
        #[serde(default)]
        displayable: bool,
        #[serde(default)]
        send: bool,
        #[serde(default)]
        sync: bool,
        data_carrying: bool,
        comparable: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        variants: Vec<ProjectedEnumVariant>,
        #[serde(default)]
        exhaustive: bool,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        generic_parameters: Vec<ProjectedGenericParameter>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedConstant {
    pub name: String,
    pub rust_name: String,
    pub rust_path: String,
    pub ty: ProjectedType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_expression: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terrane_value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedField {
    pub name: String,
    pub rust_name: String,
    pub ty: ProjectedType,
    pub rust_type: String,
    pub conversion: ProjectedFieldConversion,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectedFieldConversion {
    Identity,
    OptionalOwned,
    StringBorrow,
    OptionalStringBorrow,
    SliceBorrow,
    OptionalSliceBorrow,
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

#[expect(
    clippy::struct_excessive_bools,
    reason = "interface safety, auto traits, and drop behavior are independent Rust facts"
)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedInterface {
    #[serde(default)]
    pub is_unsafe: bool,
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
pub struct ProjectedEnumPayload {
    pub owner_rust_path: String,
    pub variant: String,
    pub style: ProjectedEnumPayloadStyle,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedEnumPayloadStyle {
    Tuple,
    Struct,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedEnumVariantStyle {
    Unit,
    Tuple,
    Struct,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedEnumVariant {
    pub name: String,
    pub style: ProjectedEnumVariantStyle,
    pub fields: Vec<ProjectedField>,
    pub constructible: bool,
    pub unavailable_reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedEnumPayloadOperation {
    pub style: ProjectedEnumPayloadStyle,
    pub fields: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedEnumOperation {
    Construct {
        variant: String,
        unit: bool,
        conversion: ProjectedEnumPayloadConversion,
        payload_rust_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        payload: Option<ProjectedEnumPayloadOperation>,
    },
    VariantName {
        variants: Vec<String>,
        exhaustive: bool,
    },
    Extract {
        variant: String,
        conversion: ProjectedEnumPayloadConversion,
        payload_rust_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        payload: Option<ProjectedEnumPayloadOperation>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_path: Option<String>,
    pub name: String,
    pub parameters: Vec<ProjectedParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub generic_parameters: Vec<ProjectedGenericParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rust_generic_arguments: Vec<ProjectedType>,
    pub result: ProjectedType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_result: Option<ProjectedDestinationResult>,
    pub error: Option<String>,
    pub is_async: bool,
    #[serde(default)]
    pub is_unsafe: bool,
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
    pub parameters: Vec<ProjectedDestinationParameter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bound_roots: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedDestinationParameter {
    pub name: String,
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
    Opaque {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        bounds: Vec<String>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        anonymous_chain: bool,
    },
    Bool,
    Int,
    FixedInt(String),
    RustInt(String),
    Float,
    Float32,
    Char,
    String,
    BorrowedString,
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
    InvocationScoped {
        rust_type: String,
        name: String,
        lifetimes: Vec<String>,
        #[serde(default)]
        expression_scoped: bool,
        owned: Box<ProjectedType>,
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        native_bound: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        native_method: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        native_result: Option<String>,
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        native_substitutions: BTreeMap<String, ProjectedType>,
        #[serde(default)]
        parameter_rust_types: Vec<String>,
        #[serde(default)]
        parameter_borrows: Vec<bool>,
        #[serde(default)]
        parameters_destination_selected: bool,
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

    pub(crate) fn is_concrete_terrane_numeric(&self) -> bool {
        matches!(
            self,
            Self::Int | Self::FixedInt(_) | Self::RustInt(_) | Self::Float | Self::Float32
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
            Self::Opaque { .. } => "_".to_owned(),
            Self::Float => "f64".to_owned(),
            Self::Float32 => "f32".to_owned(),
            Self::Char => "char".to_owned(),
            Self::String => "String".to_owned(),
            Self::BorrowedString => "&str".to_owned(),
            Self::Bytes => "Vec<u8>".to_owned(),
            Self::Sequence { rust_path, .. }
            | Self::Mapping { rust_path, .. }
            | Self::Set { rust_path, .. }
            | Self::Foreign { rust_path, .. }
            | Self::BoxedInterface { rust_path, .. } => rust_path.clone(),
            Self::InvocationScoped { rust_type, .. } => rust_type.clone(),
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
            Self::InvocationScoped { owned, .. } => owned.bind_associated(replacement),
            Self::BoxedInterface {
                associated_type: Some(associated),
                ..
            } => associated.ty.bind_associated(replacement),
            _ => {}
        }
    }
    pub(crate) fn contains_opaque(&self) -> bool {
        match self {
            Self::Opaque { .. } => true,
            Self::Sequence { item, .. }
            | Self::Set { item, .. }
            | Self::AsyncIterationStep(item)
            | Self::Optional(item) => item.contains_opaque(),
            Self::Mapping { key, value, .. } => key.contains_opaque() || value.contains_opaque(),
            Self::Tuple(items) => items.iter().any(Self::contains_opaque),
            Self::Foreign { arguments, .. } => arguments.iter().any(Self::contains_opaque),
            Self::InvocationScoped { owned, .. } => owned.contains_opaque(),
            Self::BoxedInterface {
                associated_type, ..
            } => associated_type
                .as_ref()
                .is_some_and(|binding| binding.ty.contains_opaque()),
            Self::Callback {
                parameters, result, ..
            } => parameters.iter().any(Self::contains_opaque) || result.contains_opaque(),
            _ => false,
        }
    }
    fn contains_generic(&self, generic: &str) -> bool {
        match self {
            Self::Generic(name) => name == generic,
            Self::Sequence { item, .. }
            | Self::Set { item, .. }
            | Self::AsyncIterationStep(item)
            | Self::Optional(item) => item.contains_generic(generic),
            Self::Mapping { key, value, .. } => {
                key.contains_generic(generic) || value.contains_generic(generic)
            }
            Self::Tuple(items) => items.iter().any(|item| item.contains_generic(generic)),
            Self::Foreign { arguments, .. } => arguments
                .iter()
                .any(|argument| argument.contains_generic(generic)),
            Self::InvocationScoped { owned, .. } => owned.contains_generic(generic),
            Self::BoxedInterface {
                associated_type, ..
            } => associated_type
                .as_ref()
                .is_some_and(|binding| binding.ty.contains_generic(generic)),
            Self::Callback {
                parameters, result, ..
            } => {
                parameters
                    .iter()
                    .any(|parameter| parameter.contains_generic(generic))
                    || result.contains_generic(generic)
            }
            _ => false,
        }
    }
    pub(crate) fn contains_open_generic(&self) -> bool {
        match self {
            Self::Generic(_) | Self::Associated(_) => true,
            Self::Sequence { item, .. }
            | Self::Set { item, .. }
            | Self::AsyncIterationStep(item)
            | Self::Optional(item) => item.contains_open_generic(),
            Self::Mapping { key, value, .. } => {
                key.contains_open_generic() || value.contains_open_generic()
            }
            Self::Tuple(items) => items.iter().any(Self::contains_open_generic),
            Self::Foreign { arguments, .. } => arguments.iter().any(Self::contains_open_generic),
            Self::InvocationScoped { owned, .. } => owned.contains_open_generic(),
            Self::BoxedInterface {
                associated_type, ..
            } => associated_type
                .as_ref()
                .is_some_and(|binding| binding.ty.contains_open_generic()),
            Self::Callback {
                parameters, result, ..
            } => {
                parameters.iter().any(Self::contains_open_generic) || result.contains_open_generic()
            }
            _ => false,
        }
    }
}

impl ProjectedType {
    #[must_use]
    pub fn terrane_name(&self) -> String {
        match self {
            Self::Associated(_) => "host-projected-associated".to_owned(),
            Self::Generic(name) => format!("host-projected-generic-{name}"),
            Self::Opaque { .. } => "host-projected-opaque".to_owned(),
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
            Self::Char | Self::String | Self::BorrowedString => "string".to_owned(),
            Self::Bytes => "bytes".to_owned(),
            Self::BoxedInterface { name, .. }
            | Self::Foreign { name, .. }
            | Self::InvocationScoped { name, .. } => name.clone(),
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

    pub(crate) fn has_identity_representation(&self) -> bool {
        match self {
            Self::Optional(inner) => inner.has_identity_representation(),
            Self::None
            | Self::Bool
            | Self::FixedInt(_)
            | Self::Float
            | Self::Float32
            | Self::String
            | Self::Bytes
            | Self::InvocationScoped { .. }
            | Self::Foreign { .. } => true,
            _ => false,
        }
    }

    pub(crate) fn contains_borrowed_result(&self) -> bool {
        match self {
            Self::BorrowedString
            | Self::InvocationScoped {
                expression_scoped: true,
                ..
            } => true,
            Self::Optional(inner)
            | Self::Sequence { item: inner, .. }
            | Self::Set { item: inner, .. }
            | Self::AsyncIterationStep(inner) => inner.contains_borrowed_result(),
            Self::Tuple(items)
            | Self::Foreign {
                arguments: items, ..
            } => items.iter().any(Self::contains_borrowed_result),
            Self::Mapping { key, value, .. } => {
                key.contains_borrowed_result() || value.contains_borrowed_result()
            }
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeclinedItem {
    pub rust_path: String,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnavailableProjection {
    pub rust_path: String,
    pub namespace: String,
    pub name: String,
    pub member: Option<String>,
    pub reason: String,
    pub required_by: BTreeSet<String>,
    required_by_contracts: BTreeSet<String>,
    references: BTreeSet<String>,
    partial: Option<PartialProjection>,
}

type UnavailableMemberMap<'a> = BTreeMap<(&'a str, &'a str), Vec<&'a UnavailableProjection>>;
impl UnavailableProjection {
    fn is_required(&self) -> bool {
        !self.required_by.is_empty() || !self.required_by_contracts.is_empty()
    }
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
        ProjectedType::InvocationScoped { owned, .. } => {
            collect_nested_projected_types(owned, name, candidates);
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
    match ty {
        ProjectedType::InvocationScoped { owned, .. } => projected_type_owner(owned),
        ProjectedType::Foreign { base_rust_path, .. } => Some(
            base_rust_path
                .split_once('<')
                .map_or(base_rust_path, |(constructor, _)| constructor),
        ),
        ProjectedType::BoxedInterface { trait_path, .. } => Some(
            trait_path
                .split_once('<')
                .map_or(trait_path, |(constructor, _)| constructor),
        ),
        _ => None,
    }
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
        self.source_for_imports_with_members_impl(imports, demanded_members, false, None)
    }

    fn source_for_imports_with_members_impl(
        &self,
        imports: &BTreeMap<String, BTreeSet<String>>,
        demanded_members: &ProjectedMemberDemands,
        allow_namespace_cycles: bool,
        unavailable_members: Option<&UnavailableMemberMap<'_>>,
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
            let source_dependencies =
                projected_source_dependencies(&ordered_foreign, &all_items, namespace);
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
                    unavailable_members,
                );
            }
            for item in selected {
                source_rendering::render_callable_declaration(&mut text, item, &aliases);
            }
            sources.push((namespace.clone(), text, source_dependencies));
        }
        Self::finalize_projected_sources(sources, allow_namespace_cycles)
    }

    fn finalize_projected_sources(
        sources: Vec<(String, String, BTreeSet<String>)>,
        allow_namespace_cycles: bool,
    ) -> Result<Vec<(String, String)>, String> {
        if allow_namespace_cycles {
            Ok(sources
                .into_iter()
                .map(|(namespace, source, _)| (namespace, source))
                .collect())
        } else {
            Self::order_projected_sources(sources)
        }
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
        let demanded_members = self.all_projected_member_demands();
        self.source_for_imports_with_members(imports, &demanded_members)
    }

    fn all_projected_member_demands(&self) -> ProjectedMemberDemands {
        self.dependencies
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
            .collect()
    }

    fn simple_projected_member_demands(&self) -> ProjectedMemberDemands {
        self.dependencies
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
                        .filter(|function| inventory_member_syntax_gap(function).is_none())
                        .map(|function| function.name.clone())
                        .collect(),
                )),
                _ => None,
            })
            .collect()
    }

    fn documented_sources(
        &self,
        imports: &BTreeMap<String, BTreeSet<String>>,
        unavailable: &[UnavailableProjection],
    ) -> (Vec<(String, String)>, Option<String>) {
        let unavailable_members = unavailable_member_map(unavailable);
        let demanded_members = self.simple_projected_member_demands();
        let (mut sources, gap) = self
            .source_for_imports_with_members_impl(
                imports,
                &demanded_members,
                true,
                Some(&unavailable_members),
            )
            .map_or_else(|error| (Vec::new(), Some(error)), |sources| (sources, None));
        append_required_nominal_units(&mut sources, self, unavailable);
        (sources, gap)
    }

    #[must_use]
    pub fn unavailable_inventory(
        &self,
        demand_sites: &ProjectionDemandSites,
    ) -> Vec<UnavailableProjection> {
        let mut declines = self
            .dependencies
            .iter()
            .flat_map(|dependency| {
                dependency
                    .declined
                    .iter()
                    .map(move |declined| (dependency, None, declined))
                    .chain(dependency.items.iter().flat_map(move |item| {
                        let ProjectedKind::Interface(interface) = &item.kind else {
                            return Vec::new().into_iter();
                        };
                        interface
                            .declined_methods
                            .iter()
                            .map(|declined| (dependency, Some(item), declined))
                            .collect::<Vec<_>>()
                            .into_iter()
                    }))
            })
            .collect::<Vec<_>>();
        declines.sort_by(|left, right| {
            left.2
                .rust_path
                .cmp(&right.2.rust_path)
                .then_with(|| left.2.reason.cmp(&right.2.reason))
        });
        let mut unavailable = declines
            .into_iter()
            .map(|(dependency, interface_owner, declined)| {
                let (namespace, name, member) = interface_owner.map_or_else(
                    || {
                        let member_owner = dependency.items.iter().find(|item| {
                            declined
                                .rust_path
                                .strip_prefix(&item.rust_path)
                                .is_some_and(|suffix| suffix.starts_with("::"))
                        });
                        member_owner.map_or_else(
                            || {
                                (
                                    namespace_for_rust_path(dependency, &declined.rust_path),
                                    declined
                                        .rust_path
                                        .rsplit("::")
                                        .next()
                                        .unwrap_or(&declined.rust_path)
                                        .replace('_', "-"),
                                    None,
                                )
                            },
                            |owner| {
                                (
                                    owner.namespace.clone(),
                                    owner.name.clone(),
                                    declined
                                        .rust_path
                                        .rsplit("::")
                                        .next()
                                        .map(|member| member.replace('_', "-")),
                                )
                            },
                        )
                    },
                    |owner| {
                        (
                            owner.namespace.clone(),
                            owner.name.clone(),
                            declined
                                .rust_path
                                .rsplit("::")
                                .next()
                                .map(|member| member.replace('_', "-")),
                        )
                    },
                );
                let required_by = demand_sites
                    .get(&(namespace.clone(), name.clone(), member.clone()))
                    .cloned()
                    .unwrap_or_default();
                let partial = dependency.partial_declines.iter().find(|partial| {
                    partial.rust_path == declined.rust_path && partial.reason == declined.reason
                });
                UnavailableProjection {
                    rust_path: declined.rust_path.clone(),
                    namespace,
                    name,
                    member,
                    reason: declined.reason.clone(),
                    required_by,
                    required_by_contracts: BTreeSet::new(),
                    references: partial
                        .map(|partial| partial.references.clone())
                        .unwrap_or_default(),
                    partial: partial.map(|partial| partial.projection.clone()),
                }
            })
            .collect::<Vec<_>>();
        propagate_partial_contract_requirements(&mut unavailable);
        unavailable
    }

    /// Renders a complete human-readable inventory plus compiler-owned Terrane source units.
    ///
    /// Report metadata and residual obligations remain comments. Every `Generated source unit`
    /// marker introduces an ordinary one-namespace Terrane unit; package analysis splits and
    /// syntax-validates those units before replacing the artifact. Admitted nominal declarations
    /// are active source. A demanded unavailable struct, enum, alias, or trait also receives an
    /// active class/interface skeleton when its nominal shape is recoverable, but remains
    /// unregistered for lowering until its documented residual obligations are satisfied.
    ///
    /// Namespace cycles are valid between these separate generated units and therefore do not
    /// suppress unrelated nominal declarations.
    #[must_use]
    pub fn documented_inventory(&self, demand_sites: &ProjectionDemandSites) -> String {
        let mut output = String::from(
            "# Generated by Terrane. Do not edit.\n\
             # Complete native dependency projection inventory.\n",
        );
        writeln!(output, "# Projection schema: {PROJECTION_SCHEMA}")
            .expect("writing to a string cannot fail");
        writeln!(output, "# Cache identity: {}", self.cache_identity)
            .expect("writing to a string cannot fail");
        writeln!(output, "# Content hash: {}\n", self.content_hash)
            .expect("writing to a string cannot fail");

        let mut counts = BTreeMap::<(String, String), usize>::new();
        for item in self
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| {
                !matches!(
                    item.kind,
                    ProjectedKind::Function(_) | ProjectedKind::Macro(_)
                )
            })
        {
            *counts
                .entry((item.namespace.clone(), item.name.clone()))
                .or_default() += 1;
        }
        let imports = counts.iter().filter(|(_, count)| **count == 1).fold(
            BTreeMap::<String, BTreeSet<String>>::new(),
            |mut imports, ((namespace, name), _)| {
                imports
                    .entry(namespace.clone())
                    .or_default()
                    .insert(name.clone());
                imports
            },
        );
        let unavailable = self.unavailable_inventory(demand_sites);
        let (sources, source_rendering_gap) = self.documented_sources(&imports, &unavailable);
        let (required, unused): (Vec<_>, Vec<_>) = unavailable
            .iter()
            .partition(|unavailable| unavailable.is_required());
        output.push_str("# Required unavailable projected declarations\n");
        if required.is_empty() {
            output.push_str("# None\n");
        } else {
            for unavailable in required {
                render_unavailable_projection(&mut output, unavailable);
            }
        }
        render_required_projected_declarations(&mut output, self, &unavailable);
        output.push_str(
            "#\n# -----------------------------------------------------------------------------\n\
             # Projected Terrane declarations\n",
        );
        if let Some(error) = source_rendering_gap {
            writeln!(output, "# Projected source rendering gap: {error}")
                .expect("writing to a string cannot fail");
        }

        for (namespace, source) in sources {
            super::generated_projection::write_source_unit(&mut output, &namespace, &source);
        }

        output.push_str(
            "#\n# -----------------------------------------------------------------------------\n\
             # Unused unavailable projected declarations\n",
        );
        if unused.is_empty() {
            output.push_str("# None\n");
        } else {
            for unavailable in unused {
                render_unavailable_projection(&mut output, unavailable);
            }
        }

        let ambiguous = counts
            .iter()
            .filter(|(_, count)| **count > 1)
            .collect::<Vec<_>>();
        if !ambiguous.is_empty() {
            output.push_str("#\n# Ambiguous projected declarations\n");
            for ((namespace, name), count) in ambiguous {
                writeln!(
                    output,
                    "# - {namespace} {name}: {count} native declarations project to this name"
                )
                .expect("writing to a string cannot fail");
                render_demand_sites(
                    &mut output,
                    demand_sites.get(&(namespace.clone(), name.clone(), None)),
                );
            }
        }

        output
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
            if let ProjectedKind::Function(function) | ProjectedKind::Macro(function) = &item.kind {
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
    pub(crate) fn item_named(&self, name: &str) -> Option<&ProjectedItem> {
        let mut matching = self
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| item.name == name);
        let item = matching.next()?;
        matching.next().is_none().then_some(item)
    }

    #[must_use]
    pub(crate) fn borrowed_struct_view(
        &self,
        rust_path: &str,
    ) -> Option<(&str, &[ProjectedField])> {
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find_map(|item| match &item.kind {
                ProjectedKind::ForeignType {
                    fields,
                    borrowed_view: true,
                    native_view_type: Some(native_view_type),
                    ..
                } if projected_owner_path_matches(&item.rust_path, rust_path) => {
                    Some((native_view_type.as_str(), fields.as_slice()))
                }
                _ => None,
            })
    }
    #[must_use]
    pub(crate) fn enum_payload(
        &self,
        rust_path: &str,
    ) -> Option<(&ProjectedEnumPayload, &[ProjectedField])> {
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find_map(|item| match &item.kind {
                ProjectedKind::ForeignType {
                    fields,
                    enum_payload: Some(payload),
                    ..
                } if projected_owner_path_matches(&item.rust_path, rust_path) => {
                    Some((payload, fields.as_slice()))
                }
                _ => None,
            })
    }

    #[must_use]
    pub(crate) fn projected_constructor(
        &self,
        namespace: &str,
        name: &str,
    ) -> Option<&ProjectedFunction> {
        match &self.item(namespace, name)?.kind {
            ProjectedKind::ForeignType { constructor, .. } => constructor.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn projected_payload_selector<'a>(
        &'a self,
        namespace: &str,
        owner: &str,
        member: &str,
    ) -> Option<&'a str> {
        let ProjectedKind::ForeignType {
            fields,
            methods,
            static_methods,
            ..
        } = &self.item(namespace, owner)?.kind
        else {
            return None;
        };
        let ty = fields
            .iter()
            .find(|field| field.name == member)
            .map(|field| &field.ty)
            .or_else(|| {
                methods
                    .iter()
                    .chain(static_methods)
                    .find(|method| method.name == member)
                    .map(|method| &method.result)
            })?;
        Self::payload_selector(ty)
    }

    pub(crate) fn payload_selector(ty: &ProjectedType) -> Option<&str> {
        match ty {
            ProjectedType::Generic(name) => Some(name),
            ProjectedType::Optional(inner)
            | ProjectedType::InvocationScoped { owned: inner, .. } => Self::payload_selector(inner),
            _ => None,
        }
    }

    #[must_use]
    pub(crate) fn projected_struct(
        &self,
        namespace: &str,
        name: &str,
    ) -> Option<(&str, &[ProjectedField], bool)> {
        let item = self.item(namespace, name)?;
        match &item.kind {
            ProjectedKind::ForeignType {
                fields,
                borrowed_view,
                enum_payload,
                ..
            } if !fields.is_empty() => Some((
                item.rust_path.as_str(),
                fields.as_slice(),
                *borrowed_view || enum_payload.is_some(),
            )),
            _ => None,
        }
    }

    #[must_use]
    pub(crate) fn projected_owner_for_import(
        &self,
        namespace: &str,
        name: &str,
    ) -> Option<(String, String)> {
        let item = self.item(namespace, name)?;
        let (owner_path, owner_name) = match &item.kind {
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => {
                return Some((item.namespace.clone(), item.name.clone()));
            }
            ProjectedKind::Function(function) => match &function.result {
                ProjectedType::Foreign {
                    rust_path,
                    base_rust_path,
                    name,
                    ..
                } => (
                    if base_rust_path.is_empty() {
                        rust_path
                    } else {
                        base_rust_path
                    },
                    name,
                ),
                _ => return None,
            },
            ProjectedKind::Interface(_) | ProjectedKind::Macro(_) => return None,
        };
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|candidate| {
                candidate.name == *owner_name
                    && projected_owner_path_matches(&candidate.rust_path, owner_path)
            })
            .map(|candidate| (candidate.namespace.clone(), candidate.name.clone()))
    }

    pub(crate) fn projected_member_result_owner(
        &self,
        namespace: &str,
        owner: &str,
        member: &str,
    ) -> Option<(String, String)> {
        let item = self.item(namespace, owner)?;
        let result = match &item.kind {
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                constants,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                constants,
                ..
            } => {
                if let Some(constant) = constants.iter().find(|constant| constant.name == member) {
                    &constant.ty
                } else {
                    &methods
                        .iter()
                        .chain(static_methods)
                        .find(|function| function.name == member)?
                        .result
                }
            }
            _ => return None,
        };
        let result = match result {
            ProjectedType::InvocationScoped {
                owned,
                expression_scoped: true,
                ..
            } => {
                if matches!(owned.as_ref(), ProjectedType::Optional(_)) {
                    return self.borrowed_scope_owner(owned);
                }
                owned.as_ref()
            }
            result => result,
        };
        let ProjectedType::Foreign {
            rust_path,
            base_rust_path,
            ..
        } = result
        else {
            return None;
        };
        let owner_path = if base_rust_path.is_empty() {
            rust_path
        } else {
            base_rust_path
        };
        if projected_owner_path_matches(&item.rust_path, owner_path) {
            return Some((item.namespace.clone(), item.name.clone()));
        }
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
                ProjectedKind::Function(projected) | ProjectedKind::Macro(projected) => {
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
                        if let ProjectedKind::ForeignType {
                            constructor: Some(constructor),
                            ..
                        } = &item.kind
                        {
                            candidates.push(constructor.result.clone());
                        } else {
                            candidates.push(ProjectedType::Foreign {
                                rust_path: item.rust_path.clone(),
                                name: item.name.clone(),
                                base_rust_path: item.rust_path.clone(),
                                arguments: Vec::new(),
                            });
                        }
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
        let instantiated = candidates
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate,
                    ProjectedType::Foreign { arguments, .. } if !arguments.is_empty()
                )
            })
            .collect::<Vec<_>>();
        if let Some(first) = instantiated.first()
            && instantiated.iter().all(|candidate| *candidate == *first)
        {
            return Some((*first).clone());
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
    pub(crate) fn canonical_native_type<'a>(&'a self, rust: &'a str) -> std::borrow::Cow<'a, str> {
        crate::rust_ir::rewrite_type_paths(rust, &self.native_owner_aliases)
    }

    pub(crate) fn owner_for_projected_type(
        &self,
        projected: &ProjectedType,
    ) -> Option<(String, String)> {
        let owner = projected_type_owner(projected)?;
        let preferred_name = match projected {
            ProjectedType::Foreign { name, .. } => Some(name.as_str()),
            _ => None,
        };
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| {
                projected_owner_path_matches(
                    &self.canonical_native_type(&item.rust_path),
                    &self.canonical_native_type(owner),
                )
            })
            .min_by_key(|item| preferred_name.is_some_and(|name| item.name != name))
            .map(|item| (item.namespace.clone(), item.name.clone()))
    }

    pub(crate) fn borrowed_scope_owner(&self, owned: &ProjectedType) -> Option<(String, String)> {
        if !matches!(owned, ProjectedType::Optional(_)) {
            return self.owner_for_projected_type(owned);
        }
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| item.name == "borrowed-option" && item.rust_path == "std::option::Option")
            .map(|item| (item.namespace.clone(), item.name.clone()))
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
        self.method_for_native(namespace, type_name, None, method_name, is_static, false)
    }

    pub(crate) fn method_for_native(
        &self,
        namespace: &str,
        type_name: &str,
        native_projection: Option<&str>,
        method_name: &str,
        is_static: bool,
        is_unsafe: bool,
    ) -> Option<&ProjectedFunction> {
        let rust_path =
            native_projection.or_else(|| self.foreign_rust_path(namespace, type_name))?;
        let base_rust_path = rust_path
            .split_once('<')
            .map_or(rust_path, |(base, _)| base);
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| {
                item.namespace == namespace
                    && item.name == type_name
                    && (item
                        .rust_path
                        .split_once('<')
                        .map_or(item.rust_path.as_str(), |(base, _)| base)
                        == base_rust_path
                        || Self::unqualified_rust_type(&item.rust_path)
                            == Self::unqualified_rust_type(rust_path)
                        || projected_owner_path_matches(
                            &self.canonical_native_type(&item.rust_path),
                            &self.canonical_native_type(base_rust_path),
                        ))
            })
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
                    candidates.iter().find(|method| {
                        method.name == method_name
                            && method.is_unsafe == is_unsafe
                            && native_projection.is_none_or(|owner| {
                                method.native_owner.as_deref().is_none_or(|native| {
                                    let native =
                                        native.split_once('<').map_or(native, |(base, _)| base);
                                    let owner =
                                        owner.split_once('<').map_or(owner, |(base, _)| base);
                                    Self::unqualified_rust_type(native)
                                        == Self::unqualified_rust_type(owner)
                                        || self.canonical_native_type(native)
                                            == self.canonical_native_type(owner)
                                })
                            })
                    })
                }
                ProjectedKind::Interface(interface) if !is_static => interface
                    .methods
                    .iter()
                    .find(|method| {
                        method.function.name == method_name
                            && method.function.is_unsafe == is_unsafe
                    })
                    .map(|method| &method.function),
                _ => None,
            })
    }

    pub(crate) fn constant_for_native(
        &self,
        namespace: &str,
        type_name: &str,
        native_projection: Option<&str>,
        constant_name: &str,
    ) -> Option<&ProjectedConstant> {
        let rust_path =
            native_projection.or_else(|| self.foreign_rust_path(namespace, type_name))?;
        let base_rust_path = rust_path
            .split_once('<')
            .map_or(rust_path, |(base, _)| base);
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| {
                item.namespace == namespace
                    && item.name == type_name
                    && (Self::unqualified_rust_type(&item.rust_path)
                        == Self::unqualified_rust_type(rust_path)
                        || projected_owner_path_matches(&item.rust_path, base_rust_path))
            })
            .and_then(|item| match &item.kind {
                ProjectedKind::ForeignType { constants, .. }
                | ProjectedKind::Enum { constants, .. } => constants
                    .iter()
                    .find(|constant| constant.name == constant_name),
                _ => None,
            })
    }

    fn unqualified_rust_type(rust_type: &str) -> String {
        let mut normalized = String::with_capacity(rust_type.len());
        let mut token_start = 0;
        for (index, character) in rust_type.char_indices() {
            if character.is_alphanumeric() || matches!(character, '_' | ':') {
                continue;
            }
            if token_start < index {
                normalized.push_str(rust_type[token_start..index].rsplit("::").next().unwrap());
            }
            normalized.push(character);
            token_start = index + character.len_utf8();
        }
        if token_start < rust_type.len() {
            normalized.push_str(rust_type[token_start..].rsplit("::").next().unwrap());
        }
        normalized
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
                ProjectedKind::ForeignType {
                    methods, cloneable, ..
                } if !cloneable
                    && methods.iter().any(|method| {
                        method.is_async
                            || (matches!(method.receiver, Some(Receiver::Move))
                                && method.native_path.is_none())
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
                ProjectedKind::Function(_) | ProjectedKind::Macro(_) => None,
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
    evidence: &[crate::rust_interop::ImplProbeEvidence],
) {
    for evidence in evidence
        .iter()
        .filter(|evidence| evidence.answer != crate::rust_interop::ProbeAnswer::Yes)
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
///
/// # Panics
/// Panics only when internally derived reexport-provider indices no longer address the declared
/// and projected dependency vectors built from the same resolved graph.
#[expect(
    clippy::too_many_lines,
    reason = "one transactional resolution path owns fetch, exact cache, artifact, and local fallback"
)]
pub fn resolve(
    root: &Path,
    dependencies: &[RustDependency],
    demands: Option<&BTreeSet<(String, String)>>,
) -> Result<Projection, ProjectionError> {
    let sandbox = containment();
    if dependencies.is_empty() {
        let mut projection = Projection {
            native_owner_aliases: BTreeMap::default(),
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
    seed_dependency_lock(root, &workspace)?;
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
    persist_dependency_lock(root, &workspace)?;
    let (identity, target) = cache_identity(root, &workspace, dependencies, demands, sandbox)?;
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
        persist_dependency_lock(root, &workspace)?;
        history::apply_projection_history(root, &mut cached)?;
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
            persist_dependency_lock(root, &workspace)?;
            let bytes =
                serde_json::to_vec_pretty(&projection).map_err(|error| ProjectionError {
                    message: format!("cannot serialize published dependency projection: {error}"),
                })?;
            write_if_changed(&cache_path, &bytes)?;
            history::apply_projection_history(root, &mut projection)?;
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
    let mut reexport_declines = (0..dependencies.len())
        .map(|_| Vec::new())
        .collect::<Vec<_>>();
    let reexport_rustdocs = external_reexport_rustdocs(
        &workspace,
        &rustdocs,
        &metadata,
        &target,
        sandbox,
        demands,
        &mut reexport_declines,
    )?;
    let mut declared_public_paths = BTreeMap::new();
    for (_, document, public_paths) in &rustdocs {
        for (id, public_path) in public_paths {
            let Some(summary) = document
                .paths
                .get(id)
                .filter(|summary| summary.crate_id == 0)
            else {
                continue;
            };
            declared_public_paths
                .entry(summary.path.join("::"))
                .and_modify(|current| prefer_alias(current, public_path))
                .or_insert_with(|| public_path.clone());
        }
    }
    let canonical_public_paths = vec![declared_public_paths.clone(); dependencies.len()];
    let mut projected = rustdocs
        .iter()
        .enumerate()
        .map(|(dependency_index, (dependency, document, public_paths))| {
            project_rustdoc(
                dependency,
                document,
                public_paths,
                &canonical_public_paths[dependency_index],
                true,
            )
        })
        .collect::<Vec<_>>();
    for (dependency, mut declines) in projected.iter_mut().zip(reexport_declines) {
        dependency.declined.append(&mut declines);
        normalize_projected_items(&mut dependency.items, &mut dependency.declined);
    }
    for reexport in &reexport_rustdocs {
        for provider in &reexport.providers {
            for (facade_path, canonical_path) in &provider.rust_path_aliases {
                rewrite_projected_owner_root(
                    std::slice::from_mut(
                        projected
                            .get_mut(provider.dependency_index)
                            .expect("reexport provider index came from projected dependencies"),
                    ),
                    facade_path,
                    canonical_path,
                );
            }
            let fragment_public_paths =
                provider_fragment_public_paths(&declared_public_paths, provider);
            let mut fragment = project_rustdoc(
                dependencies
                    .get(provider.dependency_index)
                    .expect("reexport provider index came from declared dependencies"),
                &reexport.document,
                &provider.public_paths,
                &fragment_public_paths,
                false,
            );
            for (facade_path, canonical_path) in &provider.rust_path_aliases {
                rewrite_projected_owner_root(
                    std::slice::from_mut(&mut fragment),
                    facade_path,
                    canonical_path,
                );
            }
            let dependency = projected
                .get_mut(provider.dependency_index)
                .expect("reexport provider index came from projected dependencies");
            dependency.items.append(&mut fragment.items);
            dependency.declined.append(&mut fragment.declined);
            normalize_projected_items(&mut dependency.items, &mut dependency.declined);
        }
    }
    project_external_provided_trait_methods(&mut projected, &rustdocs, &canonical_public_paths);
    let native_owner_aliases = foreign_impls::project_foreign_owner_impls(
        &mut projected,
        &rustdocs,
        &canonical_public_paths,
        &reexport_rustdocs,
    );
    aliases::project_closed_alias_members(
        &mut projected,
        &rustdocs,
        &reexport_rustdocs,
        &declared_public_paths,
    );
    for dependency in &mut projected {
        borrowed_graph::add_optional_owners(&mut dependency.items);
    }
    apply_namespace_overlays(&mut projected, &overlays)?;
    resolve_cross_dependency_boundary_conversions(&mut projected);
    for dependency in dependencies {
        rewrite_projected_owner_root(
            &mut projected,
            &dependency.package.replace('-', "_"),
            &dependency.name.replace('-', "_"),
        );
    }
    let mut bound_dependencies =
        recursive_owner_dependencies(&projected, dependencies, &workspace, false)?;
    for dependency in &bound_dependencies {
        rewrite_projected_owner_root(
            &mut projected,
            &dependency.package.replace('-', "_"),
            &dependency.name,
        );
    }
    enforce_transitive_reachability(
        &mut projected,
        dependencies,
        &bound_dependencies,
        &workspace,
        false,
    )?;
    canonicalize_projected_type_names(&mut projected);
    bound_dependencies.sort_by(|left, right| left.name.cmp(&right.name));
    enforce_transitive_reachability(
        &mut projected,
        dependencies,
        &bound_dependencies,
        &workspace,
        true,
    )?;
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
            ["core::marker::Send", "core::marker::Sync"]
                .into_iter()
                .map(|rust_bound| crate::rust_interop::BoundQuestion {
                    rust_type: item.rust_path.clone(),
                    rust_bound: rust_bound.to_owned(),
                    inferred_parameters: Vec::new(),
                })
        })
        .collect::<Vec<_>>();
    if !auto_trait_questions.is_empty() {
        let report = crate::rust_interop::ProjectionOracle::new(&workspace, &identity, sandbox)
            .prove_bounds(&auto_trait_questions)?;
        for evidence in report.evidence {
            let satisfied = evidence.answer == crate::rust_interop::ProbeAnswer::Yes;
            for item in projected
                .iter_mut()
                .flat_map(|dependency| &mut dependency.items)
                .filter(|item| item.rust_path == evidence.question.rust_type)
            {
                match &mut item.kind {
                    ProjectedKind::ForeignType { send, sync, .. }
                    | ProjectedKind::Enum { send, sync, .. } => {
                        match evidence.question.rust_bound.as_str() {
                            "core::marker::Send" => *send = satisfied,
                            "core::marker::Sync" => *sync = satisfied,
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
        let report = crate::rust_interop::ProjectionOracle::new(&workspace, &identity, sandbox)
            .prove_impls(&impl_questions)?;
        decline_unproven_projected_interfaces(&mut projected, &report.evidence);
    }
    resolution_events.push(ResolutionEvent {
        source: ResolutionSource::LocalRustdoc,
        status: ResolutionStatus::Generated,
        reason: "no reusable exact artifact was available; generated with the pinned local rustdoc toolchain".to_owned(),
    });
    decline_unnameable_bound_owners(
        &mut projected,
        dependencies,
        &bound_dependencies,
        &workspace,
    )?;
    decline_functions_with_missing_generic_interfaces(&mut projected);
    for dependency in
        projected_bound_dependencies(&projected, dependencies, &bound_dependencies, &workspace)?
    {
        if !bound_dependencies
            .iter()
            .any(|existing| existing.name == dependency.name)
        {
            bound_dependencies.push(dependency);
        }
    }
    bound_dependencies.sort_by(|left, right| left.name.cmp(&right.name));
    let mut projection = Projection {
        native_owner_aliases,
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
    persist_dependency_lock(root, &workspace)?;
    projection.content_hash = projection_content_hash(&projection)?;
    let bytes = serde_json::to_vec_pretty(&projection).map_err(|error| ProjectionError {
        message: format!("cannot serialize dependency projection: {error}"),
    })?;
    write_if_changed(&cache_path, &bytes)?;
    history::apply_projection_history(root, &mut projection)?;
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
        &projection.native_owner_aliases,
        &projection.probes,
        projection.probe_wall_time_ms,
    ))
    .map_err(|error| ProjectionError {
        message: format!("cannot encode projection content hash: {error}"),
    })?;
    Ok(format!("{:x}", Sha256::digest(payload)))
}

fn prune_projection_cache(directory: &Path, retained: &Path) -> Result<(), ProjectionError> {
    prune_cache_family(
        directory,
        "projection-",
        Some(retained),
        MAX_PROJECTION_CACHE_RECORDS,
        "remove stale dependency projection",
    )?;
    prune_cache_family(
        directory,
        "owner-rustdoc-",
        None,
        MAX_OWNER_RUSTDOC_CACHE_RECORDS,
        "remove stale owner rustdoc",
    )
}

fn prune_cache_family(
    directory: &Path,
    prefix: &str,
    retained: Option<&Path>,
    max_records: usize,
    removal_context: &'static str,
) -> Result<(), ProjectionError> {
    let entries = fs::read_dir(directory).map_err(io_error("read dependency projection cache"))?;
    let mut previous = Vec::new();
    for entry in entries {
        let entry = entry.map_err(io_error("read dependency projection cache entry"))?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if retained.is_some_and(|retained| path == retained)
            || !name.starts_with(prefix)
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
    let retained_count = usize::from(retained.is_some());
    for (_, path) in previous
        .into_iter()
        .skip(max_records.saturating_sub(retained_count))
    {
        fs::remove_file(&path).map_err(io_error(removal_context))?;
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
    crate::rust_interop::configure_projection_cargo_command(&mut command);
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

fn resolved_dependency_metadata(workspace: &Path) -> Result<serde_json::Value, ProjectionError> {
    let mut command = Command::new("cargo");
    crate::rust_interop::configure_projection_cargo_command(&mut command);
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
    private_dependencies: &[ProjectedBoundDependency],
    workspace: &Path,
    error_owners_only: bool,
) -> Result<(), ProjectionError> {
    let mut declared = dependencies
        .iter()
        .flat_map(|dependency| [&dependency.name, &dependency.package])
        .map(|name| name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    declared.extend(
        private_dependencies
            .iter()
            .map(|dependency| dependency.name.replace('-', "_")),
    );
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
                ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
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
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => vec![function],
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
        ProjectedType::InvocationScoped { name, owned, .. } => {
            canonicalize_projected_type_name(owned, names);
            if !matches!(owned.as_ref(), ProjectedType::Optional(_)) {
                *name = owned.terrane_name();
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
    bound_dependencies: &[ProjectedBoundDependency],
    workspace: &Path,
) -> Result<(), ProjectionError> {
    let declared = declared
        .iter()
        .map(|dependency| dependency.name.replace('-', "_"))
        .chain(
            bound_dependencies
                .iter()
                .map(|dependency| dependency.name.clone()),
        )
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
                ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
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
    private_dependencies: &[ProjectedBoundDependency],
    workspace: &Path,
) -> Result<Vec<ProjectedBoundDependency>, ProjectionError> {
    let mut declared = declared
        .iter()
        .map(|dependency| dependency.name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    declared.extend(
        private_dependencies
            .iter()
            .map(|dependency| dependency.name.clone()),
    );
    let required = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .flat_map(|item| match &item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => vec![function],
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
        .flat_map(|function| {
            let mut roots = function
                .destination_result
                .iter()
                .flat_map(|destination| destination.bound_roots.iter().cloned())
                .collect::<BTreeSet<_>>();
            roots.extend(
                function
                    .generic_parameters
                    .iter()
                    .flat_map(|parameter| &parameter.rust_bounds)
                    .flat_map(|bound| rust_bound_roots(bound)),
            );
            for parameter in &function.generic_parameters {
                roots.remove(&parameter.name);
            }
            roots
        })
        .filter(|root| {
            !declared.contains(root)
                && !matches!(
                    root.as_str(),
                    "std" | "core" | "alloc" | "self" | "crate" | "Self"
                )
        })
        .collect::<BTreeSet<_>>();
    resolved_projected_dependencies(required, workspace, false)
}

fn resolved_projected_dependencies(
    required: BTreeSet<String>,
    workspace: &Path,
    strict: bool,
) -> Result<Vec<ProjectedBoundDependency>, ProjectionError> {
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
    let mut dependencies = Vec::new();
    for root in required {
        let matches = packages
            .iter()
            .filter(|(package, _, _)| package.replace('-', "_") == root)
            .collect::<Vec<_>>();
        let [(package, version, source)] = matches.as_slice() else {
            if strict {
                return Err(ProjectionError {
                    message: format!(
                        "projected result bound root `{root}` is not a unique resolved package"
                    ),
                });
            }
            continue;
        };
        if !source.as_deref().is_some_and(is_crates_io_lock_source) {
            if strict {
                return Err(ProjectionError {
                    message: format!(
                        "projected result bound root `{root}` is not a nameable registry dependency"
                    ),
                });
            }
            continue;
        }
        dependencies.push(ProjectedBoundDependency {
            name: root,
            package: package.clone(),
            version: format!("={version}"),
        });
    }
    Ok(dependencies)
}

fn recursive_owner_dependencies(
    projected: &[ProjectedDependency],
    declared: &[RustDependency],
    workspace: &Path,
    _error_owners_only: bool,
) -> Result<Vec<ProjectedBoundDependency>, ProjectionError> {
    let reachable = declared
        .iter()
        .flat_map(|dependency| [&dependency.name, &dependency.package])
        .map(|name| name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    let required = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .flat_map(item_foreign_owners)
        .filter(|owner| !owner_is_reachable(owner, &reachable))
        .collect::<BTreeSet<_>>();
    let mut dependencies = resolved_projected_dependencies(required, workspace, false)?;
    for dependency in &mut dependencies {
        dependency.name = format!("__terrane_recursive_{}", dependency.name);
    }
    Ok(dependencies)
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

const DEPENDENCY_LOCK_FILE: &str = "terrane-dependencies.lock";

fn seed_dependency_lock(root: &Path, workspace: &Path) -> Result<(), ProjectionError> {
    let path = root.join(DEPENDENCY_LOCK_FILE);
    let Ok(bytes) = fs::read(&path) else {
        return Ok(());
    };
    fs::create_dir_all(workspace).map_err(io_error("create dependency projection workspace"))?;
    write_if_changed(&workspace.join("Cargo.lock"), &bytes)
}

fn persist_dependency_lock(root: &Path, workspace: &Path) -> Result<(), ProjectionError> {
    let bytes = fs::read(workspace.join("Cargo.lock"))
        .map_err(io_error("read complete resolved dependency lock"))?;
    write_if_changed(&root.join(DEPENDENCY_LOCK_FILE), &bytes)
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
        // This private edge makes an already-resolved recursive signature or bound owner
        // nameable to generated Rust. Empty feature lists prevent widening; the original
        // locked transitive edges retain every active feature.
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
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
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
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
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
        ProjectedKind::Function(function) | ProjectedKind::Macro(function) => vec![function],
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
        if let Some(error) = function.error.as_deref().and_then(rust_path_owner) {
            owners.insert(error.to_owned());
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
        "projected signature references unreachable crate `{}` at resolved version `{version}`; the defining trait or helper is not public through a declared dependency",
        owner.replace('_', "-")
    )
}

struct ReexportRustdoc {
    document: RustdocCrate,
    providers: Vec<ReexportProvider>,
}

struct ReexportProvider {
    dependency_index: usize,
    public_paths: BTreeMap<Id, String>,
    canonical_public_paths: BTreeMap<String, String>,
    rust_path_aliases: BTreeMap<String, String>,
}

fn provider_fragment_public_paths(
    declared_public_paths: &BTreeMap<String, String>,
    provider: &ReexportProvider,
) -> BTreeMap<String, String> {
    let mut public_paths = declared_public_paths.clone();
    // Provider aliases override declared-owner paths only inside this facade fragment. Unmapped
    // owner paths remain owner-rooted and are declined by ordinary transitive reachability.
    public_paths.extend(provider.canonical_public_paths.clone());
    public_paths
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
    terrane_rust_analysis::parse_rustdoc(package_name, &bytes, RUSTDOC_TOOLCHAIN).map_err(|error| {
        ProjectionError {
            message: error.message,
        }
    })
}

fn cached_owner_rustdoc(
    workspace: &Path,
    package_spec: &str,
    crate_name: &str,
    package_name: &str,
    target: &str,
    containment: Containment,
) -> Result<RustdocCrate, ProjectionError> {
    let mut hash = Sha256::new();
    let rustdoc_format = rustdoc_types::FORMAT_VERSION.to_string();
    for (label, value) in [
        ("package-spec", package_spec),
        ("crate-name", crate_name),
        ("package-name", package_name),
        ("hidden-items", "true"),
        ("target", target),
        ("rustdoc-toolchain", RUSTDOC_TOOLCHAIN),
        ("rustdoc-format", rustdoc_format.as_str()),
    ] {
        hash.update(label.len().to_le_bytes());
        hash.update(label.as_bytes());
        hash.update(value.len().to_le_bytes());
        hash.update(value.as_bytes());
    }
    hash.update(format!("{containment:?}"));
    let cache_path = workspace.join(format!("owner-rustdoc-{:x}.json", hash.finalize()));
    if let Ok(bytes) = fs::read(&cache_path)
        && let Ok(document) =
            terrane_rust_analysis::parse_rustdoc(package_name, &bytes, RUSTDOC_TOOLCHAIN)
    {
        mark_cache_record_used(&cache_path);
        return Ok(document);
    }
    let document = generate_rustdoc(
        workspace,
        package_spec,
        crate_name,
        package_name,
        containment,
        true,
    )?;
    let generated_path = workspace
        .join("target/rustdoc-57/doc")
        .join(format!("{crate_name}.json"));
    let bytes = fs::read(&generated_path).map_err(|error| ProjectionError {
        message: format!(
            "cannot cache owner rustdoc `{}`: {error}",
            generated_path.display()
        ),
    })?;
    write_if_changed(&cache_path, &bytes)?;
    Ok(document)
}

fn mark_cache_record_used(path: &Path) {
    // Cache recency is best-effort: a usable owner document must not become a projection failure
    // merely because its timestamp cannot be updated.
    let _ = fs::OpenOptions::new()
        .write(true)
        .open(path)
        .and_then(|file| {
            file.set_times(fs::FileTimes::new().set_modified(std::time::SystemTime::now()))
        });
}

fn resolved_library_package(
    metadata: &serde_json::Value,
    crate_name: &str,
) -> Result<(String, String), String> {
    let packages = metadata
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "resolved Cargo metadata has no package list".to_owned())?;
    let matches = packages
        .iter()
        .filter(|package| {
            package
                .get("targets")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|targets| {
                    targets.iter().any(|target| {
                        target.get("name").and_then(serde_json::Value::as_str) == Some(crate_name)
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
        .filter_map(|package| {
            Some((
                package.get("name")?.as_str()?.to_owned(),
                package.get("version")?.as_str()?.to_owned(),
            ))
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [(package, version)] => Ok((package.clone(), version.clone())),
        [] => Err(format!(
            "external reexport owner `{crate_name}` has no resolved library package"
        )),
        _ => Err(format!(
            "external reexport owner `{crate_name}` is ambiguous across resolved packages: {}",
            matches
                .iter()
                .map(|(package, version)| format!("{package}@{version}"))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
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
fn record_canonical_alias(
    paths: &mut BTreeMap<String, String>,
    canonical_path: &str,
    owner_path: &str,
    public_path: &str,
) {
    for source in [canonical_path, owner_path] {
        paths
            .entry(source.to_owned())
            .and_modify(|current| prefer_alias(current, public_path))
            .or_insert_with(|| public_path.to_owned());
    }
}

fn reexport_is_demanded(
    dependency: &RustDependency,
    public_path: &str,
    expands_descendants: bool,
    demands: Option<&BTreeSet<(String, String)>>,
) -> bool {
    let Some(demands) = demands else {
        return true;
    };
    let path = public_path
        .split("::")
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let Some(name) = path.last() else {
        return false;
    };
    if expands_descendants {
        let namespace = dependency_namespace(dependency, &path);
        return demands.iter().any(|(target, _)| {
            target == &namespace || target.starts_with(&format!("{namespace}/"))
        });
    }
    let namespace = dependency_namespace(dependency, &path[..path.len().saturating_sub(1)]);
    demands.contains(&(namespace.clone(), name.clone()))
        || demands.contains(&(format!("{namespace}/macros"), name.clone()))
}

#[expect(
    clippy::too_many_lines,
    reason = "one pass groups demanded facade aliases and prefixes, then materializes each owner"
)]
fn external_reexport_rustdocs(
    workspace: &Path,
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    metadata: &serde_json::Value,
    target: &str,
    containment: Containment,
    demands: Option<&BTreeSet<(String, String)>>,
    declines: &mut [Vec<DeclinedItem>],
) -> Result<Vec<ReexportRustdoc>, ProjectionError> {
    // Aggregate every demanded facade binding by concrete owner so one projection run emits
    // and parses at most one supplemental rustdoc document per resolved owner package.
    let mut requests = BTreeMap::<String, ReexportRequest>::new();
    for (dependency_index, (dependency, document, public_paths)) in rustdocs.iter().enumerate() {
        for (id, public_path) in public_paths {
            let Some(summary) = document.paths.get(id) else {
                continue;
            };
            let owner_crate_name = if summary.crate_id == 0 {
                if document.index.contains_key(id) {
                    continue;
                }
                document
                    .paths
                    .get(&document.root)
                    .and_then(|root| root.path.first())
                    .cloned()
                    .unwrap_or_else(|| dependency.package.replace('-', "_"))
            } else {
                let Some(external) = document.external_crates.get(&summary.crate_id) else {
                    continue;
                };
                if matches!(external.name.as_str(), "std" | "alloc") {
                    continue;
                }
                external.name.clone()
            };
            if owner_crate_name == "core"
                && !matches!(summary.kind, ItemKind::Struct | ItemKind::Enum)
            {
                continue;
            }
            if !reexport_is_demanded(
                dependency,
                public_path,
                summary.kind == ItemKind::Module,
                demands,
            ) {
                continue;
            }
            if owner_crate_name == "core" {
                let request = requests
                    .entry(owner_crate_name)
                    .or_insert_with(|| ReexportRequest {
                        package_spec: String::new(),
                        package_name: "core".to_owned(),
                        aliases: BTreeMap::new(),
                        prefixes: BTreeMap::new(),
                    });
                let aliases = request.aliases.entry(dependency_index).or_default();
                aliases
                    .entry(summary.path.join("::"))
                    .and_modify(|current| prefer_alias(current, public_path))
                    .or_insert_with(|| public_path.clone());
                continue;
            }
            let (package_name, version) =
                match resolved_library_package(metadata, &owner_crate_name) {
                    Ok(package) => package,
                    Err(reason) => {
                        declines[dependency_index].push(DeclinedItem {
                            rust_path: public_path.clone(),
                            reason,
                        });
                        continue;
                    }
                };
            let request = requests
                .entry(owner_crate_name)
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
            if !reexport.expands_descendants
                || matches!(reexport.crate_name.as_str(), "std" | "core" | "alloc")
                || !reexport_is_demanded(dependency, &reexport.public_path, true, demands)
            {
                continue;
            }
            let (package_name, version) =
                match resolved_library_package(metadata, &reexport.crate_name) {
                    Ok(package) => package,
                    Err(reason) => {
                        declines[dependency_index].push(DeclinedItem {
                            rust_path: reexport.public_path,
                            reason,
                        });
                        continue;
                    }
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
            let document = if crate_name == "core" {
                terrane_rust_analysis::core_rustdoc(&workspace.join("rust-survey/sysroot"), target)
                    .map_err(|error| ProjectionError {
                        message: error.message,
                    })?
            } else {
                cached_owner_rustdoc(
                    workspace,
                    &request.package_spec,
                    &crate_name,
                    &request.package_name,
                    target,
                    containment,
                )?
            };
            let owner_public_paths = rustdoc_public_paths(&document);
            let mut provider_indices = request
                .aliases
                .keys()
                .chain(request.prefixes.keys())
                .copied()
                .collect::<BTreeSet<_>>();
            for (dependency_index, (_, direct_document, _)) in rustdocs.iter().enumerate() {
                if direct_document
                    .paths
                    .get(&direct_document.root)
                    .and_then(|root| root.path.first())
                    == Some(&crate_name)
                {
                    provider_indices.insert(dependency_index);
                }
            }
            let providers = provider_indices
                .into_iter()
                .filter_map(|dependency_index| {
                    let dependency = rustdocs[dependency_index].0;
                    let direct_owner = rustdocs[dependency_index]
                        .1
                        .paths
                        .get(&rustdocs[dependency_index].1.root)
                        .and_then(|root| root.path.first())
                        == Some(&crate_name);
                    let aliases = request.aliases.get(&dependency_index);
                    let prefixes = request.prefixes.get(&dependency_index);
                    let mut public_paths = BTreeMap::new();
                    let mut canonical_public_paths = BTreeMap::new();
                    let mut rust_path_aliases = BTreeMap::new();
                    for (id, summary) in document
                        .paths
                        .iter()
                        .filter(|(_, summary)| summary.crate_id == 0)
                    {
                        let canonical_path = summary.path.join("::");
                        let owner_path = owner_public_paths.get(id).unwrap_or(&canonical_path);
                        if direct_owner && let Some(public_path) = owner_public_paths.get(id) {
                            record_canonical_alias(
                                &mut canonical_public_paths,
                                &canonical_path,
                                owner_path,
                                public_path,
                            );
                            if reexport_is_demanded(dependency, public_path, false, demands) {
                                terrane_rust_analysis::prefer_public_path(
                                    &mut public_paths,
                                    *id,
                                    public_path.clone(),
                                );
                            }
                        }
                        if let Some(public_path) = aliases.and_then(|aliases| {
                            aliases
                                .get(&canonical_path)
                                .or_else(|| aliases.get(owner_path))
                        }) {
                            record_canonical_alias(
                                &mut canonical_public_paths,
                                &canonical_path,
                                owner_path,
                                public_path,
                            );
                            if crate_name == "core" {
                                rust_path_aliases.insert(
                                    public_path.clone(),
                                    canonicalize_rust_path(&canonical_path),
                                );
                            }
                            if reexport_is_demanded(dependency, public_path, false, demands) {
                                terrane_rust_analysis::prefer_public_path(
                                    &mut public_paths,
                                    *id,
                                    public_path.clone(),
                                );
                            }
                        }
                        for (canonical_prefix, public_prefix) in prefixes.into_iter().flatten() {
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
                                record_canonical_alias(
                                    &mut canonical_public_paths,
                                    &canonical_path,
                                    owner_path,
                                    &public_path,
                                );
                                if reexport_is_demanded(dependency, &public_path, false, demands) {
                                    terrane_rust_analysis::prefer_public_path(
                                        &mut public_paths,
                                        *id,
                                        public_path,
                                    );
                                }
                            }
                        }
                    }
                    if public_paths.is_empty() {
                        return None;
                    }
                    Some(ReexportProvider {
                        dependency_index,
                        public_paths,
                        canonical_public_paths,
                        rust_path_aliases,
                    })
                })
                .collect();
            Ok(ReexportRustdoc {
                document,
                providers,
            })
        })
        .collect()
}

// Production rustdoc parsing is centralized in `generate_rustdoc`; this byte-oriented helper
// remains only for focused decoder and projection unit tests.
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
            name: _,
            base_rust_path,
            arguments,
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
        ProjectedType::InvocationScoped {
            rust_type, owned, ..
        } => {
            rewrite_projected_rust_root(owned, package_root, dependency_root);
            *rust_type = rewrite_rust_bound_root(rust_type, package_root, dependency_root);
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
            parameters,
            result,
            native_bound,
            native_result,
            native_substitutions,
            ..
        } => {
            for parameter in parameters {
                rewrite_projected_rust_root(parameter, package_root, dependency_root);
            }
            rewrite_projected_rust_root(result, package_root, dependency_root);
            for substitution in native_substitutions.values_mut() {
                rewrite_projected_rust_root(substitution, package_root, dependency_root);
            }
            if let Some(native_bound) = native_bound {
                *native_bound =
                    rewrite_rust_bound_root(native_bound, package_root, dependency_root);
            }
            if let Some(native_result) = native_result {
                *native_result =
                    rewrite_rust_bound_root(native_result, package_root, dependency_root);
            }
        }
        ProjectedType::Opaque { bounds, .. } => {
            for bound in bounds {
                *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
            }
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
        | ProjectedType::BorrowedString
        | ProjectedType::Bytes
        | ProjectedType::AsyncSinkOutcome
        | ProjectedType::Associated(_) => {}
    }
}

fn rewrite_projected_function_root(
    function: &mut ProjectedFunction,
    package_root: &str,
    dependency_root: &str,
) {
    for parameter in &mut function.parameters {
        rewrite_projected_rust_root(&mut parameter.ty, package_root, dependency_root);
        if let Some(associated) = &mut parameter.associated_type {
            rewrite_projected_rust_root(&mut associated.ty, package_root, dependency_root);
        }
        if let Some(interface) = &mut parameter.generic_interface {
            *interface = rewrite_rust_bound_root(interface, package_root, dependency_root);
        }
        for bound in &mut parameter.generic_bounds {
            *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
        }
    }
    for generic in &mut function.generic_parameters {
        for bound in &mut generic.rust_bounds {
            *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
        }
    }
    for argument in &mut function.rust_generic_arguments {
        rewrite_projected_rust_root(argument, package_root, dependency_root);
    }
    rewrite_projected_rust_root(&mut function.result, package_root, dependency_root);
    if let Some(error) = &mut function.error {
        *error = rewrite_rust_bound_root(error, package_root, dependency_root);
    }
    if let Some(destination) = &mut function.destination_result {
        for parameter in &mut destination.parameters {
            for bound in &mut parameter.rust_bounds {
                *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
            }
        }
        for root in &mut destination.bound_roots {
            if root == package_root {
                root.clone_from(&dependency_root.to_owned());
            }
        }
    }
    if let Some(
        ProjectedEnumOperation::Construct {
            payload_rust_type, ..
        }
        | ProjectedEnumOperation::Extract {
            payload_rust_type, ..
        },
    ) = &mut function.enum_operation
    {
        *payload_rust_type =
            rewrite_rust_bound_root(payload_rust_type, package_root, dependency_root);
    }
}

fn rewrite_projected_owner_root(
    projected: &mut [ProjectedDependency],
    package_root: &str,
    dependency_root: &str,
) {
    for item in projected
        .iter_mut()
        .flat_map(|dependency| &mut dependency.items)
    {
        item.rust_path = rewrite_rust_bound_root(&item.rust_path, package_root, dependency_root);
        match &mut item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                rewrite_projected_function_root(function, package_root, dependency_root);
            }
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                constants,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                constants,
                ..
            } => {
                for function in methods.iter_mut().chain(static_methods) {
                    rewrite_projected_function_root(function, package_root, dependency_root);
                }
                for constant in constants {
                    constant.rust_path =
                        rewrite_rust_bound_root(&constant.rust_path, package_root, dependency_root);
                    rewrite_projected_rust_root(&mut constant.ty, package_root, dependency_root);
                }
            }
            ProjectedKind::Interface(interface) => {
                for method in &mut interface.methods {
                    rewrite_projected_function_root(
                        &mut method.function,
                        package_root,
                        dependency_root,
                    );
                    if let Some(owner) = &mut method.owner_rust_path {
                        *owner = rewrite_rust_bound_root(owner, package_root, dependency_root);
                    }
                }
                if let Some(associated) = &mut interface.associated_type {
                    associated.rust_path = rewrite_rust_bound_root(
                        &associated.rust_path,
                        package_root,
                        dependency_root,
                    );
                    for bound in &mut associated.bounds {
                        *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
                    }
                }
                for supertrait in &mut interface.supertraits {
                    supertrait.rust_path = rewrite_rust_bound_root(
                        &supertrait.rust_path,
                        package_root,
                        dependency_root,
                    );
                }
            }
        }
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
            &BTreeMap::new(),
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
        if !provided
            && function
                .sig
                .output
                .as_ref()
                .is_some_and(type_contains_borrowed_ref)
        {
            return Err(format!(
                "trait member `{name}`: required borrowed results cannot be implemented by an owned source result"
            ));
        }
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
        is_unsafe: declaration.is_unsafe,
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
) -> crate::rust_interop::ImplQuestion {
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
    crate::rust_interop::ImplQuestion {
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

fn attach_unique_trait_operation(
    items: &mut [ProjectedItem],
    operation: &ProjectedTraitOperation,
    primary_counts: &BTreeMap<(String, String), usize>,
) -> bool {
    let key = (
        operation.item.namespace.clone(),
        operation.item.name.clone(),
    );
    if primary_counts.get(&key).copied().unwrap_or_default() != 1 {
        return false;
    }
    let ProjectedKind::Function(function) = &operation.item.kind else {
        return false;
    };
    let Some((methods, static_methods)) = items.iter_mut().find_map(|item| {
        if owner_trait_namespace(&item.namespace, &item.name) != operation.item.namespace {
            return None;
        }
        match &mut item.kind {
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
        }
    }) else {
        return false;
    };
    let candidates = if function.receiver.is_some() {
        methods
    } else {
        static_methods
    };
    if candidates
        .iter()
        .any(|candidate| candidate.name == function.name)
    {
        return false;
    }
    let mut function = function.clone();
    if function.native_path.is_none() {
        function.native_path = Some(operation.item.rust_path.clone());
    }
    candidates.push(function);
    true
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
    let mut remaining = Vec::new();
    for operation in operations {
        if attach_unique_trait_operation(items, &operation, &primary_counts) {
            continue;
        }
        remaining.push(operation);
    }
    let operations = remaining;
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
        if let ProjectedKind::Function(function) = &mut operation.item.kind
            && let Some(receiver) = function.receiver.take()
            && let Some(owner) = function.native_owner.as_deref()
        {
            let base_rust_path = owner.split_once('<').map_or(owner, |(base, _)| base);
            function.parameters.insert(
                0,
                ProjectedParameter {
                    name: "receiver".to_owned(),
                    ty: ProjectedType::Foreign {
                        rust_path: owner.to_owned(),
                        name: base_rust_path
                            .rsplit("::")
                            .next()
                            .unwrap_or(base_rust_path)
                            .to_owned(),
                        base_rust_path: base_rust_path.to_owned(),
                        arguments: Vec::new(),
                    },
                    generic_parameter: None,
                    generic_interface: None,
                    generic_bounds: Vec::new(),
                    associated_type: None,
                    borrowed: receiver != Receiver::Move,
                    mutable_borrow: receiver == Receiver::MutableBorrow,
                },
            );
        }
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
    canonical_public_paths: &[BTreeMap<String, String>],
) {
    for (dependency_index, (_, document, _)) in rustdocs.iter().enumerate() {
        let mut paths = document.paths.clone();
        for summary in paths.values_mut() {
            if let Some(public_path) =
                canonical_public_paths[dependency_index].get(&summary.path.join("::"))
            {
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
                if let Some(public_path) =
                    canonical_public_paths[dependency_index].get(&summary.path.join("::"))
                {
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
                    &BTreeMap::new(),
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

fn partial_projection(
    item: &Item,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<PartialProjection> {
    partial_projection_inner(item, index, paths, &mut HashSet::new())
}

fn partial_projection_inner(
    item: &Item,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    visited: &mut HashSet<Id>,
) -> Option<PartialProjection> {
    if !visited.insert(item.id) {
        return None;
    }
    let name = item.name.as_deref().unwrap_or("<anonymous>");
    match &item.inner {
        ItemEnum::Function(function) => {
            partial_function_projection(name, function, index, paths).ok()
        }
        ItemEnum::Struct(struct_) => Some(partial_nominal_type(name, "struct", &struct_.generics)),
        ItemEnum::Enum(enum_) => Some(partial_nominal_type(name, "enum", &enum_.generics)),
        ItemEnum::TypeAlias(alias) => {
            Some(partial_nominal_type(name, "type alias", &alias.generics))
        }
        ItemEnum::Trait(trait_) => Some(PartialProjection::NominalType {
            declaration: format!("interface {}", name.replace('_', "-")),
            native_kind: "trait".to_owned(),
            generic_parameters: trait_
                .generics
                .params
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect(),
        }),
        ItemEnum::Use(import) => import
            .id
            .as_ref()
            .and_then(|id| index.get(id))
            .and_then(|target| partial_projection_inner(target, index, paths, visited))
            .or_else(|| partial_nominal_from_summary(name, paths.get(&item.id)?))
            .map(|mut projection| {
                if let PartialProjection::NominalType { declaration, .. } = &mut projection {
                    let kind = declaration
                        .split_once(' ')
                        .map_or("class", |(kind, _)| kind);
                    *declaration = format!("{kind} {}", name.replace('_', "-"));
                }
                projection
            }),
        ItemEnum::Module(_) => Some(PartialProjection::Namespace),
        _ => None,
    }
}

fn partial_nominal_type(
    name: &str,
    native_kind: &str,
    generics: &rustdoc_types::Generics,
) -> PartialProjection {
    PartialProjection::NominalType {
        declaration: format!("class {}", name.replace('_', "-")),
        native_kind: native_kind.to_owned(),
        generic_parameters: generics
            .params
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect(),
    }
}

fn partial_nominal_from_summary(name: &str, summary: &ItemSummary) -> Option<PartialProjection> {
    let (declaration, native_kind) = match summary.kind {
        ItemKind::Struct => (format!("class {}", name.replace('_', "-")), "struct"),
        ItemKind::Enum => (format!("class {}", name.replace('_', "-")), "enum"),
        ItemKind::TypeAlias => (format!("class {}", name.replace('_', "-")), "type alias"),
        ItemKind::Trait => (format!("interface {}", name.replace('_', "-")), "trait"),
        _ => return None,
    };
    Some(PartialProjection::NominalType {
        declaration,
        native_kind: native_kind.to_owned(),
        generic_parameters: Vec::new(),
    })
}

fn partial_projection_references(
    dependency: &RustDependency,
    projection: &PartialProjection,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
) -> BTreeSet<String> {
    let PartialProjection::Function {
        signature,
        generic_constraints,
        callback_shapes,
    } = projection
    else {
        return BTreeSet::new();
    };
    let mut contract = signature.clone();
    for constraint in generic_constraints {
        contract.push('\n');
        contract.push_str(constraint);
    }
    for callback in callback_shapes {
        contract.push('\n');
        contract.push_str(&callback.contract);
        for method in &callback.methods {
            contract.push('\n');
            contract.push_str(method);
        }
    }
    paths
        .iter()
        .filter(|(_, summary)| native_path_occurs_in(&contract, &summary.path.join("::")))
        .map(|(id, summary)| {
            let public_path = public_paths
                .get(id)
                .cloned()
                .unwrap_or_else(|| summary.path.join("::"));
            extern_rust_path(dependency, &public_path)
        })
        .collect()
}

fn native_path_occurs_in(contract: &str, path: &str) -> bool {
    contract.match_indices(path).any(|(start, _)| {
        let before = contract[..start].chars().next_back();
        let after = contract[start + path.len()..].chars().next();
        !before.is_some_and(|character| character.is_alphanumeric() || character == '_')
            && !after.is_some_and(|character| {
                character.is_alphanumeric() || matches!(character, '_' | ':')
            })
    })
}

fn partial_function_projection(
    name: &str,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<PartialProjection, String> {
    let generics = function
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Type { .. } => Some((
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            )),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let signature = render_partial_function_signature(name, function, index, paths, &generics)?;
    let generic_constraints =
        render_partial_generic_constraints(function, index, paths, &generics)?;
    let mut callback_shapes = Vec::new();
    for (parameter, ty) in &function.sig.inputs {
        let Type::ImplTrait(bounds) = ty else {
            continue;
        };
        for bound in bounds {
            let GenericBound::TraitBound { trait_, .. } = bound else {
                continue;
            };
            let contract = render_generic_bound(bound, &[], index, paths, &generics)?;
            let Some(Item {
                inner: ItemEnum::Trait(declaration),
                ..
            }) = index.get(&trait_.id)
            else {
                continue;
            };
            let mut trait_generics = declaration
                .generics
                .params
                .iter()
                .filter_map(|parameter| match &parameter.kind {
                    GenericParamDefKind::Type { .. } => Some((
                        parameter.name.clone(),
                        ProjectedType::Generic(parameter.name.clone()),
                    )),
                    _ => None,
                })
                .collect::<BTreeMap<_, _>>();
            trait_generics.insert("Self".to_owned(), ProjectedType::Generic("Self".to_owned()));
            let methods = declaration
                .items
                .iter()
                .filter_map(|id| {
                    let method = index.get(id)?;
                    let ItemEnum::Function(function) = &method.inner else {
                        return None;
                    };
                    render_partial_function_signature(
                        method.name.as_deref().unwrap_or("<anonymous>"),
                        function,
                        index,
                        paths,
                        &trait_generics,
                    )
                    .ok()
                })
                .collect();
            callback_shapes.push(PartialCallbackShape {
                parameter: parameter.clone(),
                contract,
                methods,
            });
        }
    }
    Ok(PartialProjection::Function {
        signature,
        generic_constraints,
        callback_shapes,
    })
}

fn render_partial_function_signature(
    name: &str,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    let parameters = function
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Type {
                is_synthetic: false,
                ..
            }
            | GenericParamDefKind::Lifetime { .. }
            | GenericParamDefKind::Const { .. } => Some(parameter.name.clone()),
            GenericParamDefKind::Type {
                is_synthetic: true, ..
            } => None,
        })
        .collect::<Vec<_>>();
    let parameters = if parameters.is_empty() {
        String::new()
    } else {
        format!("<{}>", parameters.join(", "))
    };
    let inputs = function
        .sig
        .inputs
        .iter()
        .map(|(name, ty)| {
            render_partial_type(ty, index, paths, generics).map(|ty| format!("{name}: {ty}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let output = function
        .sig
        .output
        .as_ref()
        .map(|ty| render_partial_type(ty, index, paths, generics))
        .transpose()?
        .map_or_else(String::new, |ty| format!(" -> {ty}"));
    Ok(format!(
        "fn {name}{parameters}({}){output}",
        inputs.join(", ")
    ))
}

fn render_partial_generic_constraints(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Vec<String>, String> {
    let mut constraints = Vec::new();
    for parameter in &function.generics.params {
        let GenericParamDefKind::Type {
            bounds,
            is_synthetic: false,
            ..
        } = &parameter.kind
        else {
            continue;
        };
        if !bounds.is_empty() {
            let bounds = bounds
                .iter()
                .map(|bound| render_generic_bound(bound, &[], index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            constraints.push(format!("{}: {}", parameter.name, bounds.join(" + ")));
        }
    }
    for predicate in &function.generics.where_predicates {
        let WherePredicate::BoundPredicate {
            type_,
            bounds,
            generic_params,
        } = predicate
        else {
            continue;
        };
        let ty = render_partial_type(type_, index, paths, generics)?;
        let bounds = bounds
            .iter()
            .map(|bound| render_generic_bound(bound, generic_params, index, paths, generics))
            .collect::<Result<Vec<_>, _>>()?;
        if !bounds.is_empty() {
            constraints.push(format!("{ty}: {}", bounds.join(" + ")));
        }
    }
    Ok(constraints)
}

fn render_partial_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    match ty {
        Type::ImplTrait(bounds) => {
            let bounds = bounds
                .iter()
                .map(|bound| render_generic_bound(bound, &[], index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("impl {}", bounds.join(" + ")))
        }
        Type::ResolvedPath(path) => {
            let base = resolved_path_name(path, paths);
            let Some(arguments) = path.args.as_deref() else {
                return Ok(base);
            };
            match arguments {
                GenericArgs::AngleBracketed { args, constraints } => {
                    let mut rendered = args
                        .iter()
                        .map(|argument| match argument {
                            GenericArg::Lifetime(lifetime) => Ok(lifetime.clone()),
                            GenericArg::Type(ty) => render_partial_type(ty, index, paths, generics),
                            GenericArg::Const(constant) => Ok(constant.expr.clone()),
                            GenericArg::Infer => Ok("_".to_owned()),
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    for constraint in constraints {
                        let binding = match &constraint.binding {
                            AssocItemConstraintKind::Equality(Term::Type(ty)) => format!(
                                "{} = {}",
                                constraint.name,
                                render_partial_type(ty, index, paths, generics)?
                            ),
                            AssocItemConstraintKind::Equality(Term::Constant(constant)) => {
                                format!("{} = {}", constraint.name, constant.expr)
                            }
                            AssocItemConstraintKind::Constraint(bounds) => {
                                let bounds = bounds
                                    .iter()
                                    .map(|bound| {
                                        render_generic_bound(bound, &[], index, paths, generics)
                                    })
                                    .collect::<Result<Vec<_>, _>>()?;
                                format!("{}: {}", constraint.name, bounds.join(" + "))
                            }
                        };
                        rendered.push(binding);
                    }
                    Ok(format!("{base}<{}>", rendered.join(", ")))
                }
                GenericArgs::Parenthesized { inputs, output } => {
                    let inputs = inputs
                        .iter()
                        .map(|ty| render_partial_type(ty, index, paths, generics))
                        .collect::<Result<Vec<_>, _>>()?;
                    let output = output
                        .as_ref()
                        .map(|ty| render_partial_type(ty, index, paths, generics))
                        .transpose()?
                        .map_or_else(String::new, |ty| format!(" -> {ty}"));
                    Ok(format!("{base}({}){output}", inputs.join(", ")))
                }
                GenericArgs::ReturnTypeNotation => Ok(format!("{base}(..)")),
            }
        }
        _ => render_rust_type(ty, index, paths, generics),
    }
}
fn projected_nominal_generic_parameters(
    generics: &Generics,
    substitutions: &BTreeMap<String, ProjectedType>,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<Vec<ProjectedGenericParameter>, String> {
    generics
        .params
        .iter()
        .filter(|parameter| {
            matches!(parameter.kind, GenericParamDefKind::Type { .. })
                && matches!(
                    substitutions.get(&parameter.name),
                    Some(ProjectedType::Generic(_))
                )
        })
        .map(|parameter| {
            let rust_bounds = callable::generic_bounds_from_generics(parameter, generics)
                .iter()
                .map(|bound| render_generic_bound(bound, &[], index, paths, substitutions))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(ProjectedGenericParameter {
                name: parameter.name.clone(),
                input_selected: false,
                rust_bounds,
            })
        })
        .collect()
}
fn nominal_generic_instantiation(
    generics: &Generics,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<BTreeMap<String, ProjectedType>, String> {
    let mut substitutions = BTreeMap::new();
    for parameter in &generics.params {
        let projected = match &parameter.kind {
            GenericParamDefKind::Type {
                default: Some(default),
                ..
            } => project_type(default, index, paths, &substitutions)?,
            GenericParamDefKind::Type { default: None, .. } => {
                ProjectedType::Generic(parameter.name.clone())
            }
            GenericParamDefKind::Lifetime { .. } => {
                return Err(format!(
                    "lifetime parameter `{}` requires non-escaping projection",
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
        substitutions.insert(parameter.name.clone(), projected);
    }
    Ok(substitutions)
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
    let mut partial_declines = Vec::new();
    let mut projected_trait_items = Vec::new();
    let mut projected_associated_items = Vec::new();
    let mut source_constants = SourceConstantCache::new();
    let mut enum_payload_items = Vec::new();
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
        let mut namespace = dependency_namespace(dependency, &path[..path.len().saturating_sub(1)]);
        let public_rust_path = public_paths
            .get(&id)
            .cloned()
            .unwrap_or_else(|| path.join("::"));
        let canonical_rust_path = original_paths
            .get(&id)
            .map_or_else(|| path.join("::"), |summary| summary.path.join("::"));
        let mut rust_path = if canonical_rust_path.starts_with("core::")
            || canonical_rust_path.starts_with("alloc::")
        {
            canonicalize_rust_path(&canonical_rust_path)
        } else {
            extern_rust_path(dependency, &public_rust_path)
        };
        let docs = item.docs.clone();
        if matches!(item.inner, ItemEnum::Module(_) | ItemEnum::Use(_)) {
            continue;
        }
        let projected = match &item.inner {
            ItemEnum::Function(function) => project_function(
                function,
                index,
                paths,
                public_paths,
                Some(&name),
                true,
            )
            .and_then(|mut projected_function| {
                let candidate_result = match &projected_function.result {
                    ProjectedType::InvocationScoped { owned, .. } => owned.as_ref().clone(),
                    result => result.clone(),
                };
                let mut chain_owner = project_chain_owner(
                    dependency,
                    function,
                    &candidate_result,
                    false,
                    index,
                    paths,
                    public_paths,
                    &mut source_constants,
                );
                let has_external_terminal_conversion =
                    function.sig.output.as_ref().is_some_and(|output| {
                        let Some(output_id) = resolved_nominal_id(output, index) else {
                            return false;
                        };
                        let Some(output_item) = index.get(&output_id) else {
                            return false;
                        };
                        let implementations = match &output_item.inner {
                            ItemEnum::Struct(output) => &output.impls,
                            ItemEnum::Enum(output) => &output.impls,
                            ItemEnum::Union(output) => &output.impls,
                            _ => return false,
                        };
                        let explicit_into = implementations
                            .iter()
                            .filter_map(|implementation| index.get(implementation))
                            .any(|implementation| {
                                let ItemEnum::Impl(implementation) = &implementation.inner else {
                                    return false;
                                };
                                implementation.blanket_impl.is_none()
                                    && implementation.trait_.as_ref().is_some_and(|interface| {
                                        let name = resolved_path_name(interface, paths);
                                        (name.ends_with("::Into") || name == "Into")
                                            && resolved_path_type_arguments(interface)
                                                .first()
                                                .is_some_and(|destination| {
                                                    !matches!(destination, Type::Generic(_))
                                                })
                                    })
                            });
                        explicit_into
                            || index.values().any(|item| {
                                let ItemEnum::Impl(implementation) = &item.inner else {
                                    return false;
                                };
                                implementation.blanket_impl.is_none()
                                    && implementation.trait_.as_ref().is_some_and(|interface| {
                                        let name = resolved_path_name(interface, paths);
                                        (name.ends_with("::From") || name == "From")
                                            && resolved_path_type_arguments(interface)
                                                .first()
                                                .is_some_and(|source| {
                                                    matches!(
                                                        source,
                                                        Type::ResolvedPath(source)
                                                            if source.id == output_id
                                                    )
                                                })
                                    })
                            })
                    });
                if has_external_terminal_conversion {
                    chain_owner = None;
                }
                let ordinary_chain_owner = chain_owner.is_some();
                let receiver_tied =
                    projected_function
                        .parameters
                        .first()
                        .is_some_and(|parameter| {
                            parameter.borrowed
                                && matches!(parameter.ty, ProjectedType::Foreign { .. })
                        });
                let lifetime_bearing = function
                    .sig
                    .output
                    .as_ref()
                    .is_some_and(type_contains_lifetime_argument);
                if lifetime_bearing
                    && has_external_terminal_conversion
                    && !receiver_tied
                    && chain_owner.is_none()
                {
                    let output_generics = function
                        .generics
                        .params
                        .iter()
                        .filter(|parameter| {
                            matches!(parameter.kind, GenericParamDefKind::Type { .. })
                        })
                        .map(|parameter| {
                            (
                                parameter.name.clone(),
                                ProjectedType::Generic(parameter.name.clone()),
                            )
                        })
                        .collect();
                    let rust_type = function
                        .sig
                        .output
                        .as_ref()
                        .and_then(|output| {
                            render_rust_type(output, index, paths, &output_generics).ok()
                        })
                        .unwrap_or_else(|| candidate_result.rust_type());
                    let lifetimes = rust_lifetimes(&rust_type);
                    if lifetimes.is_empty() {
                        return Err(
                            "lifetime-bearing foreign type cannot cross a projected boundary"
                                .to_owned(),
                        );
                    }
                    projected_function.result = ProjectedType::InvocationScoped {
                        rust_type,
                        name: candidate_result.terrane_name(),
                        lifetimes,
                        expression_scoped: false,
                        owned: Box::new(candidate_result.clone()),
                    };
                }
                let invocation_scoped_chain = matches!(
                    projected_function.result,
                    ProjectedType::InvocationScoped { .. }
                );
                if chain_owner.is_none() && invocation_scoped_chain {
                    chain_owner = project_chain_owner(
                        dependency,
                        function,
                        &candidate_result,
                        true,
                        index,
                        paths,
                        public_paths,
                        &mut source_constants,
                    );
                }
                if let Some(chain_owner) = chain_owner {
                    if ordinary_chain_owner {
                        projected_function.result = candidate_result;
                        projected_function.chain_role = Some(ChainRole::Root);
                    }
                    projected_associated_items.push(chain_owner);
                } else if matches!(
                    projected_function.result,
                    ProjectedType::Opaque {
                        anonymous_chain: true,
                        ..
                    }
                ) {
                    projected_function.chain_role = Some(ChainRole::Root);
                }
                Ok(ProjectedKind::Function(projected_function))
            }),
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
                    constructor: None,
                    methods: Vec::new(),
                    static_methods: Vec::new(),
                    constants: Vec::new(),
                    boundary: project_boundary_capabilities(&alias.type_, index, paths),
                    fields: Vec::new(),
                    borrowed_view: false,
                    native_view_type: None,
                    enum_payload: None,
                    generic_parameters: Vec::new(),
                    displayable: false,
                    cloneable: false,
                    send: false,
                    sync: false,
                })
            })(),
            ItemEnum::Struct(structure) => {
                let mut owner_generics =
                    match default_generic_instantiation(structure, index, paths).or_else(|_| {
                        data::constructor_generic_instantiation(structure, index, paths)
                    }) {
                        Ok(generics) => generics,
                        Err(reason) => {
                            declined.push(DeclinedItem {
                                rust_path: rust_path.clone(),
                                reason,
                            });
                            continue;
                        }
                    };
                let has_lifetime = structure.generics.params.iter().any(|parameter| {
                    matches!(parameter.kind, GenericParamDefKind::Lifetime { .. })
                });
                let field_projection = if item
                    .attrs
                    .iter()
                    .any(|attribute| matches!(attribute, rustdoc_types::Attribute::NonExhaustive))
                {
                    Err("non-exhaustive struct cannot be constructed outside its crate".to_owned())
                } else {
                    project_struct_fields(structure, index, paths, &owner_generics)
                };
                let (fields, borrowed_view) = match field_projection {
                    Ok(projected) => projected,
                    Err(reason) if has_lifetime => {
                        declined.push(DeclinedItem {
                            rust_path: rust_path.clone(),
                            reason,
                        });
                        continue;
                    }
                    Err(_) => (Vec::new(), false),
                };
                let native_view_type = borrowed_view.then(|| {
                    let native_arguments = structure
                        .generics
                        .params
                        .iter()
                        .filter_map(|parameter| match parameter.kind {
                            GenericParamDefKind::Lifetime { .. } => Some("'_".to_owned()),
                            GenericParamDefKind::Type { .. } => owner_generics
                                .get(&parameter.name)
                                .map(ProjectedType::rust_type),
                            GenericParamDefKind::Const { .. } => None,
                        })
                        .collect::<Vec<_>>();
                    format!("{rust_path}<{}>", native_arguments.join(", "))
                });
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
                let constructor = if owner_generics
                    .values()
                    .any(|ty| matches!(ty, ProjectedType::Generic(_)))
                {
                    if item
                        .attrs
                        .contains(&rustdoc_types::Attribute::NonExhaustive)
                    {
                        declined.push(DeclinedItem {
                            rust_path: rust_path.clone(),
                            reason: "non-exhaustive native structs cannot be constructed outside their defining crate".to_owned(),
                        });
                        continue;
                    }
                    match data::project_struct_constructor(
                        structure,
                        &fields,
                        index,
                        paths,
                        public_paths,
                        &owner_generics,
                    ) {
                        Ok(constructor) => Some(constructor),
                        Err(reason) => {
                            declined.push(DeclinedItem {
                                rust_path: rust_path.clone(),
                                reason,
                            });
                            continue;
                        }
                    }
                } else {
                    None
                };
                let generic_parameters = match projected_nominal_generic_parameters(
                    &structure.generics,
                    &owner_generics,
                    index,
                    paths,
                ) {
                    Ok(parameters) => parameters,
                    Err(reason) => {
                        declined.push(DeclinedItem {
                            rust_path: rust_path.clone(),
                            reason,
                        });
                        continue;
                    }
                };
                {
                    let projected_impls = if borrowed_view {
                        &[][..]
                    } else {
                        structure.impls.as_slice()
                    };
                    let (projected_methods, trait_methods, projected_constants, method_declines) =
                        project_methods(
                            projected_impls,
                            index,
                            paths,
                            public_paths,
                            &rust_path,
                            &owner_generics,
                            false,
                            &mut source_constants,
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
                        constructor,
                        methods,
                        fields,
                        borrowed_view,
                        native_view_type,
                        enum_payload: None,
                        static_methods,
                        constants: projected_constants,
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
                        generic_parameters,
                    })
                }
            }
            ItemEnum::Enum(enumeration) => {
                let unsupported_parameter =
                    enumeration.generics.params.iter().find(|parameter| {
                        !matches!(parameter.kind, GenericParamDefKind::Type { .. })
                    });
                if let Some(parameter) = unsupported_parameter {
                    Err(format!(
                        "enum generic parameter `{}` is not a type parameter",
                        parameter.name
                    ))
                } else {
                    let nominal_generic_types =
                        match nominal_generic_instantiation(&enumeration.generics, index, paths) {
                            Ok(substitutions) => substitutions,
                            Err(reason) => {
                                declined.push(DeclinedItem {
                                    rust_path: rust_path.clone(),
                                    reason,
                                });
                                continue;
                            }
                        };
                    let generic_parameters = match projected_nominal_generic_parameters(
                        &enumeration.generics,
                        &nominal_generic_types,
                        index,
                        paths,
                    ) {
                        Ok(parameters) => parameters,
                        Err(reason) => {
                            declined.push(DeclinedItem {
                                rust_path: rust_path.clone(),
                                reason,
                            });
                            continue;
                        }
                    };
                    let generic_arguments = enumeration
                        .generics
                        .params
                        .iter()
                        .filter_map(|parameter| nominal_generic_types.get(&parameter.name).cloned())
                        .collect::<Vec<_>>();
                    let generic_rust_path = if generic_arguments.is_empty() {
                        rust_path.clone()
                    } else {
                        format!(
                            "{rust_path}<{}>",
                            generic_arguments
                                .iter()
                                .map(ProjectedType::rust_type)
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    };
                    let mut owner_generics = BTreeMap::from([(
                        "Self".to_owned(),
                        ProjectedType::Foreign {
                            rust_path: generic_rust_path,
                            name: name.clone(),
                            base_rust_path: rust_path.clone(),
                            arguments: generic_arguments,
                        },
                    )]);
                    owner_generics.extend(nominal_generic_types);
                    let enum_type = owner_generics["Self"].clone();
                    let (projected_methods, trait_methods, projected_constants, method_declines) =
                        project_methods(
                            &enumeration.impls,
                            index,
                            paths,
                            public_paths,
                            &rust_path,
                            &owner_generics,
                            false,
                            &mut source_constants,
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
                    let mut variant_names = Vec::new();
                    let mut variants = Vec::new();
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
                        variants.push(enum_payload::project_variant(
                            variant_name,
                            variant,
                            index,
                            paths,
                            &owner_generics,
                        ));
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
                                match project_enum_payload(
                                    field_type,
                                    index,
                                    paths,
                                    &owner_generics,
                                ) {
                                    Ok((
                                        constructor,
                                        constructor_conversion,
                                        extraction,
                                        extraction_conversion,
                                        rust_type,
                                    )) => Some((
                                        constructor,
                                        constructor_conversion,
                                        extraction,
                                        extraction_conversion,
                                        rust_type,
                                        None,
                                    )),
                                    Err(reason) => {
                                        declined.push(DeclinedItem {
                                            rust_path: format!("{rust_path}::{variant_name}"),
                                            reason,
                                        });
                                        continue;
                                    }
                                }
                            }
                            VariantKind::Tuple(fields) => {
                                data_carrying = true;
                                match project_multi_enum_payload(
                                    &namespace,
                                    &name,
                                    &rust_path,
                                    variant_name,
                                    variant_item.docs.clone(),
                                    ProjectedEnumPayloadStyle::Tuple,
                                    fields.iter().enumerate().map(|(index, field)| {
                                        (format!("item-n{index}"), index.to_string(), *field)
                                    }),
                                    index,
                                    paths,
                                    &owner_generics,
                                ) {
                                    Ok((item, payload)) => {
                                        enum_payload_items.push(item);
                                        Some(payload)
                                    }
                                    Err(reason) => {
                                        declined.push(DeclinedItem {
                                            rust_path: format!("{rust_path}::{variant_name}"),
                                            reason,
                                        });
                                        continue;
                                    }
                                }
                            }
                            VariantKind::Struct {
                                fields,
                                has_stripped_fields,
                            } => {
                                data_carrying = true;
                                if *has_stripped_fields {
                                    declined.push(DeclinedItem {
                                        rust_path: format!("{rust_path}::{variant_name}"),
                                        reason: "payload enum variant has stripped named fields"
                                            .to_owned(),
                                    });
                                    continue;
                                }
                                match project_multi_enum_payload(
                                    &namespace,
                                    &name,
                                    &rust_path,
                                    variant_name,
                                    variant_item.docs.clone(),
                                    ProjectedEnumPayloadStyle::Struct,
                                    fields.iter().map(|field| {
                                        let field_name = index
                                            .get(field)
                                            .and_then(|item| item.name.clone())
                                            .unwrap_or_default();
                                        (field_name.clone(), field_name, Some(*field))
                                    }),
                                    index,
                                    paths,
                                    &owner_generics,
                                ) {
                                    Ok((item, payload)) => {
                                        enum_payload_items.push(item);
                                        Some(payload)
                                    }
                                    Err(reason) => {
                                        declined.push(DeclinedItem {
                                            rust_path: format!("{rust_path}::{variant_name}"),
                                            reason,
                                        });
                                        continue;
                                    }
                                }
                            }
                        };
                        let (
                            constructor_type,
                            constructor_conversion,
                            extraction_type,
                            extraction_conversion,
                            payload_rust_type,
                            payload,
                        ) = projected_payload.unwrap_or_else(|| {
                            (
                                ProjectedType::None,
                                ProjectedEnumPayloadConversion::Identity,
                                ProjectedType::None,
                                ProjectedEnumPayloadConversion::Identity,
                                String::new(),
                                None,
                            )
                        });
                        static_methods.push(ProjectedFunction {
                            native_owner: None,
                            native_path: None,
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
                            rust_generic_arguments: Vec::new(),
                            result: enum_type.clone(),
                            destination_result: None,
                            error: None,
                            is_async: false,
                            is_unsafe: false,
                            into_future: false,
                            execution_requirements: None,
                            enum_operation: Some(ProjectedEnumOperation::Construct {
                                variant: variant_name.to_owned(),
                                unit: matches!(variant.kind, VariantKind::Plain),
                                conversion: constructor_conversion,
                                payload_rust_type: payload_rust_type.clone(),
                                payload: payload.clone(),
                            }),
                            error_optional_depth: 0,
                            chain_role: None,
                            receiver: None,
                        });
                        if extraction_type != ProjectedType::None
                            && !matches!(extraction_type, ProjectedType::Optional(_))
                        {
                            methods.push(ProjectedFunction {
                                native_owner: None,
                                native_path: None,
                                name: format!("into-{variant_name}"),
                                parameters: Vec::new(),
                                generic_parameters: Vec::new(),
                                rust_generic_arguments: Vec::new(),
                                result: ProjectedType::Optional(Box::new(extraction_type)),
                                destination_result: None,
                                error: None,
                                is_async: false,
                                is_unsafe: false,
                                into_future: false,
                                execution_requirements: None,
                                enum_operation: Some(ProjectedEnumOperation::Extract {
                                    variant: variant_name.to_owned(),
                                    conversion: extraction_conversion,
                                    payload_rust_type,
                                    payload,
                                }),
                                error_optional_depth: 0,
                                chain_role: None,
                                receiver: Some(Receiver::Move),
                            });
                        }
                    }
                    if data_carrying {
                        methods.push(ProjectedFunction {
                            native_owner: None,
                            native_path: None,
                            name: "variant-name".to_owned(),
                            parameters: Vec::new(),
                            generic_parameters: Vec::new(),
                            rust_generic_arguments: Vec::new(),
                            result: ProjectedType::String,
                            destination_result: None,
                            error: None,
                            is_async: false,
                            is_unsafe: false,
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
                        variants,
                        exhaustive: !enumeration.has_stripped_variants
                            && !item
                                .attrs
                                .iter()
                                .any(|attribute| matches!(attribute, Attribute::NonExhaustive)),
                        methods,
                        static_methods,
                        constants: projected_constants,
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
                        generic_parameters,
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
            ItemEnum::Macro(_) => {
                namespace.push_str("/macros");
                Ok(ProjectedKind::Macro(project_macro(&name)))
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
            Err(reason) => {
                if let Some(projection) = partial_projection(item, index, paths) {
                    let references =
                        partial_projection_references(dependency, &projection, paths, public_paths);
                    partial_declines.push(PartialProjectionRecord {
                        rust_path: rust_path.clone(),
                        reason: reason.clone(),
                        references,
                        projection,
                    });
                }
                declined.push(DeclinedItem { rust_path, reason });
            }
        }
    }
    items.extend(enum_payload_items);
    if include_canonical_items {
        items.extend(macros::macro_reexports(dependency, document));
    }
    borrowed_graph::add_optional_owners(&mut items);
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
        for argument in &mut function.rust_generic_arguments {
            rewrite_projected_rust_root(argument, &package_root, &dependency_root);
        }
        rewrite_projected_rust_root(&mut function.result, &package_root, &dependency_root);
        if let Some(error) = &mut function.error {
            *error = rewrite_rust_bound_root(error, &package_root, &dependency_root);
        }
        if let Some(destination) = &mut function.destination_result {
            for parameter in &mut destination.parameters {
                for bound in &mut parameter.rust_bounds {
                    *bound = rewrite_rust_bound_root(bound, &package_root, &dependency_root);
                }
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
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                normalize(function);
            }
            ProjectedKind::ForeignType {
                fields,
                methods,
                static_methods,
                native_view_type,
                ..
            } => {
                if let Some(native_view_type) = native_view_type {
                    *native_view_type =
                        rewrite_rust_bound_root(native_view_type, &package_root, &dependency_root);
                }
                for field in fields {
                    rewrite_projected_rust_root(&mut field.ty, &package_root, &dependency_root);
                    field.rust_type =
                        rewrite_rust_bound_root(&field.rust_type, &package_root, &dependency_root);
                }
                for method in methods.iter_mut().chain(static_methods) {
                    normalize(method);
                }
            }
            ProjectedKind::Enum {
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
    normalize_projected_items(&mut items, &mut declined);
    ProjectedDependency {
        name: dependency.name.clone(),
        partial_declines,
        package: dependency.package.clone(),
        version: document
            .crate_version
            .clone()
            .unwrap_or_else(|| dependency.version.clone()),
        items,
        declined,
    }
}

type ProjectedMethods = (
    Vec<ProjectedFunction>,
    Vec<(String, String, bool, Option<String>, ProjectedFunction)>,
    Vec<ProjectedConstant>,
    Vec<(String, String)>,
);

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "rustdoc impl traversal keeps one shared context for trait, inherent, and source-backed member decisions"
)]
fn project_methods(
    impl_ids: &[Id],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    owner_rust_path: &str,
    owner_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
    source_constants: &mut SourceConstantCache,
) -> ProjectedMethods {
    let mut candidates = Vec::new();
    let mut trait_methods = Vec::new();
    let mut constants = Vec::new();
    let mut declined = Vec::new();
    for impl_id in impl_ids {
        let Some(Item {
            inner: ItemEnum::Impl(implementation),
            ..
        }) = index.get(impl_id)
        else {
            continue;
        };
        if implementation.is_negative || implementation.is_synthetic {
            continue;
        }
        let inherent = implementation.trait_.is_none();
        let native_owner = if matches!(implementation.for_, Type::Generic(_))
            && implementation.blanket_impl.is_some()
        {
            owner_rust_path.to_owned()
        } else {
            render_rust_type(&implementation.for_, index, paths, owner_generics)
                .unwrap_or_else(|_| owner_rust_path.to_owned())
        };
        let mut implementation_generics = owner_generics.clone();
        if let Some(Type::Generic(generic)) = &implementation.blanket_impl
            && let Some(owner) = owner_generics.get("Self")
        {
            implementation_generics.insert(generic.clone(), owner.clone());
        }
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
            if let ItemEnum::AssocConst { type_, value } = &item.inner {
                if !inherent {
                    continue;
                }
                match project_type(type_, index, paths, &implementation_generics) {
                    Ok(ty) => {
                        let source_expression = value
                            .as_deref()
                            .filter(|value| *value != "_")
                            .map(str::to_owned)
                            .or_else(|| {
                                source_constant_expression(
                                    item,
                                    &native_owner,
                                    name,
                                    source_constants,
                                )
                            });
                        let terrane_value = source_expression
                            .as_deref()
                            .and_then(project_rust_constant_expression);
                        constants.push(ProjectedConstant {
                            name: projected_constant_name(name),
                            rust_name: name.to_owned(),
                            rust_path: if native_owner.contains('<') {
                                format!("<{native_owner}>::{name}")
                            } else {
                                format!("{native_owner}::{name}")
                            },
                            ty,
                            source_expression,
                            terrane_value,
                        });
                    }
                    Err(reason) => declined.push((name.to_owned(), reason)),
                }
                continue;
            }
            let ItemEnum::Function(function) = &item.inner else {
                declined.push((
                    name.to_owned(),
                    "item kind has no Terrane member projection".to_owned(),
                ));
                continue;
            };
            let mut enriched_function = function.clone();
            if implementation.blanket_impl.is_some() {
                for parameter in &implementation.generics.params {
                    if !enriched_function
                        .generics
                        .params
                        .iter()
                        .any(|candidate| candidate.name == parameter.name)
                    {
                        enriched_function.generics.params.push(parameter.clone());
                    }
                }
                enriched_function
                    .generics
                    .where_predicates
                    .extend(implementation.generics.where_predicates.iter().cloned());
            }
            let function = &enriched_function;
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
                let mut trait_render_generics = implementation_generics.clone();
                for parameter in &implementation.generics.params {
                    if matches!(parameter.kind, GenericParamDefKind::Type { .. }) {
                        trait_render_generics
                            .entry(parameter.name.clone())
                            .or_insert_with(|| ProjectedType::Generic(parameter.name.clone()));
                    }
                }
                let rendered_trait =
                    render_resolved_path(trait_, index, paths, &trait_render_generics)
                        .unwrap_or_else(|_| trait_path.clone());
                let trait_implementation_path = rendered_trait.find('<').map_or_else(
                    || trait_path.clone(),
                    |start| format!("{trait_path}{}", &rendered_trait[start..]),
                );
                match project_function_with_generics(
                    function,
                    index,
                    paths,
                    public_paths,
                    Some(name),
                    &implementation_generics,
                    allow_lifetime_output,
                ) {
                    Ok(mut method) => {
                        data::rebind_method_owner(
                            &mut method,
                            implementation,
                            index,
                            paths,
                            &implementation_generics,
                        );
                        method.native_owner = Some(native_owner.clone());
                        let trait_rust_path = if local_trait {
                            let package_root = trait_implementation_path
                                .split("::")
                                .next()
                                .unwrap_or_default();
                            let dependency_root =
                                owner_rust_path.split("::").next().unwrap_or_default();
                            rewrite_rust_bound_root(
                                &trait_implementation_path,
                                package_root,
                                dependency_root,
                            )
                        } else {
                            trait_implementation_path.replace('-', "_")
                        };
                        let trait_rust_path = canonicalize_rust_path(&trait_rust_path);
                        method.native_path =
                            Some(format!("<{owner_rust_path} as {trait_rust_path}>::{name}"));
                        if implementation.blanket_impl.is_some()
                            && !(method.destination_result.is_some()
                                && method.receiver == Some(Receiver::Move)
                                && method.parameters.is_empty())
                        {
                            continue;
                        }
                        trait_methods.push((
                            trait_implementation_path,
                            public_trait_path,
                            local_trait,
                            item.docs.clone(),
                            method,
                        ));
                    }
                    Err(reason) => {
                        if implementation.blanket_impl.is_none()
                            && !is_internal_rust_protocol_method(&trait_path, name)
                        {
                            declined.push((name.to_owned(), reason));
                        }
                    }
                }
                continue;
            }
            match project_function_with_generics(
                function,
                index,
                paths,
                public_paths,
                Some(name),
                &implementation_generics,
                allow_lifetime_output,
            ) {
                Ok(mut method) => {
                    data::rebind_method_owner(
                        &mut method,
                        implementation,
                        index,
                        paths,
                        &implementation_generics,
                    );
                    method.native_owner = Some(native_owner.clone());
                    candidates.push(method);
                }
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
    constants.sort_by(|left, right| left.name.cmp(&right.name));
    (methods, trait_methods, constants, declined)
}
fn is_internal_rust_protocol_method(trait_path: &str, method: &str) -> bool {
    (method == "fmt"
        && (trait_path.ends_with("::fmt::Debug") || trait_path.ends_with("::fmt::Display")))
        || (method == "hash" && trait_path.ends_with("::hash::Hash"))
}
#[expect(
    clippy::too_many_arguments,
    reason = "chain-owner projection needs the resolved dependency and all Rustdoc path contexts"
)]
fn project_chain_owner(
    dependency: &RustDependency,
    function: &Function,
    result: &ProjectedType,
    invocation_scoped: bool,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    source_constants: &mut SourceConstantCache,
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
    let ProjectedType::Foreign {
        rust_path,
        name,
        base_rust_path: _,
        arguments,
    } = result
    else {
        return None;
    };
    if !result.contains_opaque()
        && !structure
            .generics
            .params
            .iter()
            .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }))
    {
        return None;
    }
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
    let (mut methods, _, _, _) = project_methods(
        &structure.impls,
        index,
        paths,
        public_paths,
        rust_path,
        &owner_generics,
        true,
        source_constants,
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
    if !invocation_scoped
        && methods
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
            constructor: None,
            methods,
            static_methods: Vec::new(),
            constants: Vec::new(),
            boundary: ProjectedBoundaryCapabilities::default(),
            fields: Vec::new(),
            borrowed_view: false,
            native_view_type: None,
            enum_payload: None,
            generic_parameters: Vec::new(),
            displayable: false,
            cloneable: false,
            send: false,
            sync: false,
        },
    })
}

fn resolved_nominal_id(ty: &Type, index: &HashMap<Id, Item>) -> Option<Id> {
    let Type::ResolvedPath(path) = ty else {
        return None;
    };
    match index.get(&path.id).map(|item| &item.inner) {
        Some(ItemEnum::TypeAlias(alias)) => resolved_nominal_id(&alias.type_, index),
        Some(_) => Some(path.id),
        None => None,
    }
}

fn alias_type_substitutions(
    ty: &Type,
    parameters: &[GenericParamDef],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<BTreeMap<String, ProjectedType>, String> {
    let arguments = type_arguments(ty);
    let mut substitutions = generics.clone();
    for (position, parameter) in parameters
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .enumerate()
    {
        let projected = if let Some(argument) = arguments.get(position) {
            project_type(argument, index, paths, generics)?
        } else if let GenericParamDefKind::Type {
            default: Some(default),
            ..
        } = &parameter.kind
        {
            project_type(default, index, paths, &substitutions)?
        } else {
            return Err(format!(
                "type alias parameter `{}` has no argument or default",
                parameter.name
            ));
        };
        substitutions.insert(parameter.name.clone(), projected);
    }
    Ok(substitutions)
}

fn expand_output_alias(
    mut output: Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    mut generics: BTreeMap<String, ProjectedType>,
) -> Result<(Type, BTreeMap<String, ProjectedType>), String> {
    let mut visited = BTreeSet::new();
    loop {
        let Type::ResolvedPath(path) = &output else {
            return Ok((output, generics));
        };
        if !visited.insert(path.id) {
            return Err("recursive Rust type alias in projected output".to_owned());
        }
        let Some(Item {
            inner: ItemEnum::TypeAlias(alias),
            ..
        }) = index.get(&path.id)
        else {
            return Ok((output, generics));
        };
        generics =
            alias_type_substitutions(&output, &alias.generics.params, index, paths, &generics)?;
        output = alias.type_.clone();
    }
}

fn project_function_with_generics(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    method_name: Option<&str>,
    supplied_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    project_function_inner(
        function,
        index,
        paths,
        public_paths,
        method_name,
        supplied_generics,
        allow_lifetime_output,
    )
}

fn project_function(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    method_name: Option<&str>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    project_function_inner(
        function,
        index,
        paths,
        public_paths,
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
        name: instantiated_nominal_name(
            base_rust_path.rsplit("::").next().unwrap_or("chain"),
            &rust_identity,
            &arguments,
        ),
        base_rust_path,
        arguments,
    })
}

fn input_has_owned_borrowed_view(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> bool {
    let resolved = match ty {
        Type::BorrowedRef {
            is_mutable: false,
            type_,
            ..
        } => type_.as_ref(),
        other => other,
    };
    let Type::ResolvedPath(path) = resolved else {
        return false;
    };
    let Some(Item {
        inner: ItemEnum::Struct(structure),
        attrs,
        ..
    }) = index.get(&path.id)
    else {
        return false;
    };
    !attrs
        .iter()
        .any(|attribute| matches!(attribute, rustdoc_types::Attribute::NonExhaustive))
        && default_generic_instantiation(structure, index, paths)
            .and_then(|generics| project_struct_fields(structure, index, paths, &generics))
            .is_ok_and(|(_, borrowed_view)| borrowed_view)
}

#[expect(
    clippy::too_many_lines,
    reason = "function projection keeps generic selection and exact parameter contracts together"
)]
fn project_function_inner(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    method_name: Option<&str>,
    supplied_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    let open_chain = open_chain_generics(function, index);
    let GenericMonomorphisations {
        types: generic_types,
        destination_result,
        adapter_result_bounds,
    } = if let Some(types) = open_chain.clone() {
        GenericMonomorphisations {
            types,
            destination_result: None,
            adapter_result_bounds: BTreeMap::new(),
        }
    } else {
        generic_monomorphisations(function, index, paths, supplied_generics)
            .map_err(|reason| format!("generic selection: {reason}"))?
    };
    let borrows_input = function
        .sig
        .inputs
        .iter()
        .any(|(_, ty)| matches!(ty, Type::BorrowedRef { .. }));
    if function
        .sig
        .output
        .as_ref()
        .is_some_and(type_contains_borrowed_ref)
        && !borrows_input
    {
        return Err("borrowed result values require an invocation-scoped lender".to_owned());
    }
    let mut parameters: Vec<ProjectedParameter> = Vec::new();
    let mut receiver = None;
    for (parameter_index, (name, ty)) in function.sig.inputs.iter().enumerate() {
        if name == "self" {
            receiver = Some(receiver_kind(ty)?);
            continue;
        }
        if !matches!(ty, Type::BorrowedRef { .. }) && type_contains_borrowed_ref(ty) {
            return Err("nested borrowed parameter cannot cross a projected boundary".to_owned());
        }
        let invocation_scoped_input = allow_lifetime_output
            && type_contains_lifetime_argument(ty)
            && function
                .sig
                .output
                .as_ref()
                .is_some_and(type_contains_lifetime_argument);
        if type_contains_lifetime_argument(ty)
            && !input_has_owned_borrowed_view(ty, index, paths)
            && !invocation_scoped_input
        {
            return Err(
                "input contains a lifetime-bearing foreign value with no non-escaping Terrane call representation"
                    .to_owned(),
            );
        }
        let (projected_type, impl_trait_parameter, inline_adapter_bounds) = if let Some(bounds) =
            impl_trait_bounds(ty)
        {
            let generic = format!("TerraneImpl{parameter_index}");
            if let Some((callback, result_bounds, _)) =
                project_callable_adapter_bounds(&generic, bounds, index, paths, &generic_types)?
            {
                (callback, Some(generic), Some(result_bounds))
            } else if let Some(projected) = invocation_scoped_sequence_impl_trait_input(
                bounds,
                index,
                paths,
                public_paths,
                &generic_types,
            )? {
                (projected, None, None)
            } else if let Some(projected) = structural_impl_trait_input(bounds, paths) {
                (projected, Some(generic), None)
            } else if let Ok(projectable) = projectable_interface_bound(bounds, index, paths) {
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
                    Some(generic),
                    None,
                )
            } else {
                (ProjectedType::Generic(generic.clone()), Some(generic), None)
            }
        } else if invocation_scoped_input {
            (
                project_invocation_scoped_type(ty, index, paths, &generic_types)?,
                None,
                None,
            )
        } else {
            (
                project_type(ty, index, paths, &generic_types)
                    .map_err(|reason| format!("parameter `{name}`: {reason}"))?,
                None,
                None,
            )
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
        let callable_adapter = inline_adapter_bounds
            .as_ref()
            .and_then(|bounds| match &projected_type {
                ProjectedType::Callback { result, .. } => match result.as_ref() {
                    ProjectedType::Generic(result) => Some((result.clone(), bounds.clone())),
                    _ => None,
                },
                _ => None,
            })
            .or_else(|| match (ty, &projected_type) {
                (Type::Generic(parameter), ProjectedType::Callback { result, .. }) => {
                    adapter_result_bounds
                        .get(parameter)
                        .and_then(|bounds| match result.as_ref() {
                            ProjectedType::Generic(result) => {
                                Some((result.clone(), bounds.clone()))
                            }
                            _ => None,
                        })
                }
                _ => None,
            });
        let generic_parameter = callable_adapter
            .as_ref()
            .map(|(name, _)| name.clone())
            .or(impl_trait_parameter)
            .or_else(|| boxed_adapter.as_ref().map(|(name, _)| name.clone()))
            .or_else(|| {
                function.generics.params.iter().find_map(|parameter| {
                    generic_types.get(&parameter.name).and_then(|projected| {
                        let exact_scoped_callback = matches!(
                            projected,
                            ProjectedType::Callback { result, .. }
                                if matches!(
                                    result.as_ref(),
                                    ProjectedType::InvocationScoped { .. }
                                )
                        );
                        ((matches!(projected, ProjectedType::Foreign { .. })
                            || exact_scoped_callback
                            || matches!(projected, ProjectedType::Generic(_))
                                && matches!(projected_type, ProjectedType::Generic(_)))
                            && type_mentions_generic(ty, &parameter.name))
                        .then(|| parameter.name.clone())
                    })
                })
            });
        let rendered_generic_bounds = if let Some((_, bounds)) = &callable_adapter {
            bounds.clone()
        } else if let Some((_, bounds)) = &boxed_adapter {
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
        let mut parameter_name = safe_parameter_name(name);
        if method_name.is_some_and(|function_name| function_name == parameter_name)
            || parameters
                .iter()
                .any(|parameter| parameter.name == parameter_name)
        {
            parameter_name = format!("{parameter_name}-value");
        }
        parameters.push(ProjectedParameter {
            name: parameter_name,
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
    let (effective_output, output_generic_types) = match effective_output {
        Some(output) => {
            let (output, generics) =
                expand_output_alias(output, index, paths, output_generic_types)?;
            (Some(output), generics)
        }
        None => (None, output_generic_types),
    };
    if (function.header.is_async || returns_future)
        && effective_output
            .as_ref()
            .is_some_and(type_contains_borrowed_ref)
    {
        return Err("borrowed result values cannot cross a projected boundary".to_owned());
    }
    let mut error = None;
    let mut error_optional_depth = 0;
    let project_output = |ty: &Type| {
        if type_contains_borrowed_ref(ty) {
            return project_borrowed_graph_type(ty, index, paths, &output_generic_types);
        }
        if allow_lifetime_output && type_contains_lifetime_argument(ty) {
            project_invocation_scoped_type(ty, index, paths, &output_generic_types)
        } else {
            project_type(ty, index, paths, &output_generic_types)
        }
    };
    let result = if let Some(output) = effective_output.as_ref() {
        if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Result") || name == "Result")
        {
            let arguments = type_arguments(output);
            let value = arguments
                .first()
                .ok_or_else(|| "Result has no value type".to_owned())?;
            let projected = project_output(value)
                .map_err(|reason| format!("projected result value: {reason}"))?;
            if projected.rust_type().contains('&')
                && !matches!(
                    projected,
                    ProjectedType::InvocationScoped {
                        expression_scoped: true,
                        ..
                    } | ProjectedType::BorrowedString
                )
            {
                return Err("borrowed result values cannot cross a projected boundary".to_owned());
            }
            error = arguments
                .get(1)
                .and_then(|ty| projected_error_name(ty, output, paths))
                .or_else(|| Some("Error".to_owned()));
            projected
        } else if type_contains_borrowed_ref(output) {
            project_output(output)?
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
                let projected = project_output(success)
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
                    project_output(value)
                        .map_err(|reason| format!("projected optional value: {reason}"))?,
                ))
            }
        } else {
            project_output(output).or_else(|reason| {
                open_chain_result(output, index, paths, &output_generic_types)
                    .ok_or_else(|| format!("projected output: {reason}"))
            })?
        }
    } else {
        ProjectedType::None
    };
    if result.contains_opaque()
        && !matches!(result, ProjectedType::Foreign { .. })
        && !matches!(
            result,
            ProjectedType::Opaque {
                anonymous_chain: true,
                ..
            }
        )
    {
        return Err("producer-selected opaque result requires a named foreign owner".to_owned());
    }
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
    let mut generic_parameters = function
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
                input_selected: parameters
                    .iter()
                    .any(|input| input.ty.contains_generic(&parameter.name)),
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
    generic_parameters.extend(parameters.iter().filter_map(|parameter| {
        let name = match &parameter.ty {
            ProjectedType::Generic(name) if name.starts_with("TerraneImpl") => name,
            ProjectedType::Callback { result, .. }
                if parameter.generic_parameter.as_ref().is_some_and(|name| {
                    result.as_ref() == &ProjectedType::Generic(name.clone())
                }) =>
            {
                parameter.generic_parameter.as_ref().expect("checked above")
            }
            _ => return None,
        };
        Some(ProjectedGenericParameter {
            name: name.clone(),
            input_selected: true,
            rust_bounds: parameter.generic_bounds.clone(),
        })
    }));
    generic_parameters.sort_by(|left, right| left.name.cmp(&right.name));
    let mut merged_generic_parameters: Vec<ProjectedGenericParameter> = Vec::new();
    for generic in generic_parameters {
        if let Some(existing) = merged_generic_parameters
            .iter_mut()
            .find(|existing| existing.name == generic.name)
        {
            existing.input_selected |= generic.input_selected;
            existing.rust_bounds.extend(generic.rust_bounds);
            existing.rust_bounds.sort();
            existing.rust_bounds.dedup();
        } else {
            merged_generic_parameters.push(generic);
        }
    }
    let generic_parameters = merged_generic_parameters;
    let rust_generic_arguments = function
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Type {
                is_synthetic: false,
                ..
            } => Some(
                generic_types
                    .get(&parameter.name)
                    .cloned()
                    .unwrap_or_else(|| ProjectedType::Generic(parameter.name.clone())),
            ),
            _ => None,
        })
        .collect();
    let chain_role = matches!(
        &result,
        ProjectedType::InvocationScoped {
            expression_scoped: true,
            ..
        }
    )
    .then_some(ChainRole::Root);
    Ok(ProjectedFunction {
        name: method_name.unwrap_or_default().to_owned(),
        native_owner: None,
        native_path: None,
        parameters,
        result,
        generic_parameters,
        rust_generic_arguments,
        destination_result,
        error,
        is_async: function.header.is_async || returns_future,
        is_unsafe: function.header.is_unsafe,
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
        chain_role,
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
fn invocation_scoped_sequence_impl_trait_input(
    bounds: &[GenericBound],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Option<ProjectedType>, String> {
    let Some(trait_) = bounds.iter().find_map(|bound| {
        let (trait_, _) = trait_bound_name(bound)?;
        let path = paths
            .get(&trait_.id)
            .map_or_else(|| trait_.path.clone(), |summary| summary.path.join("::"));
        matches!(
            path.as_str(),
            "core::iter::traits::collect::IntoIterator"
                | "std::iter::IntoIterator"
                | "core::iter::IntoIterator"
        )
        .then_some(trait_)
    }) else {
        return Ok(None);
    };
    let Some(GenericArgs::AngleBracketed { constraints, .. }) = trait_.args.as_deref() else {
        return Ok(None);
    };
    let Some(item_type) = constraints.iter().find_map(|constraint| {
        (constraint.name == "Item")
            .then_some(&constraint.binding)
            .and_then(|binding| match binding {
                AssocItemConstraintKind::Equality(Term::Type(ty)) => Some(ty),
                _ => None,
            })
    }) else {
        return Ok(None);
    };
    if !type_contains_lifetime_argument(item_type) {
        return Ok(None);
    }
    let mut rendering_paths = paths.clone();
    for (id, public_path) in public_paths {
        if let Some(summary) = rendering_paths.get_mut(id) {
            summary.path = public_path.split("::").map(str::to_owned).collect();
        }
    }
    if let Type::ResolvedPath(item_path) = item_type
        && let Some(name) = item_path.path.rsplit("::").next()
        && let Some(public_path) = public_paths
            .values()
            .filter(|path| path.rsplit("::").next() == Some(name))
            .min_by_key(|path| path.matches("::").count())
        && let Some(summary) = rendering_paths.get_mut(&item_path.id)
    {
        summary.path = public_path.split("::").map(str::to_owned).collect();
    }
    let item = project_invocation_scoped_type(item_type, index, &rendering_paths, generics)?;
    Ok(Some(ProjectedType::Sequence {
        rust_path: format!("std::vec::Vec<{}>", item.rust_type()),
        item: Box::new(item),
    }))
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

fn project_borrowed_graph_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    let projected = project_type(ty, index, paths, generics)?;
    if matches!(projected, ProjectedType::String) {
        return Ok(ProjectedType::BorrowedString);
    }
    let rust_type = if let Type::BorrowedRef {
        type_,
        lifetime,
        is_mutable,
    } = ty
        && matches!(type_.as_ref(), Type::QualifiedPath { .. })
    {
        format!(
            "&{}{}{}",
            lifetime
                .as_ref()
                .map_or(String::new(), |lifetime| format!("{lifetime} ")),
            if *is_mutable { "mut " } else { "" },
            projected.rust_type()
        )
    } else {
        render_rust_type(ty, index, paths, generics)?
    };
    Ok(ProjectedType::InvocationScoped {
        name: if matches!(projected, ProjectedType::Optional(_)) {
            "borrowed-option".to_owned()
        } else {
            projected.terrane_name()
        },
        lifetimes: rust_lifetimes(&rust_type),
        rust_type,
        expression_scoped: true,
        owned: Box::new(projected),
    })
}

fn project_invocation_scoped_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    let projected = project_type(ty, index, paths, generics)?;
    if let ProjectedType::Sequence { rust_path, .. } = &projected
        && let Some(item) = type_arguments(ty).into_iter().next()
    {
        return Ok(ProjectedType::Sequence {
            rust_path: rust_path.clone(),
            item: Box::new(project_invocation_scoped_type(
                item, index, paths, generics,
            )?),
        });
    }
    let rust_type = render_rust_type(ty, index, paths, generics)?;
    let lifetimes = rust_lifetimes(&rust_type);
    if lifetimes.is_empty() {
        return Ok(projected);
    }
    Ok(ProjectedType::InvocationScoped {
        name: projected.terrane_name(),
        rust_type,
        lifetimes,
        expression_scoped: false,
        owned: Box::new(projected),
    })
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
        Type::ImplTrait(bounds) => Ok(ProjectedType::Opaque {
            anonymous_chain: bounds.iter().any(|bound| {
                trait_bound_name(bound).is_some_and(|(path, _)| {
                    resolved_path_name(path, paths)
                        .rsplit("::")
                        .next()
                        .is_some_and(|name| matches!(name, "Fn" | "FnMut" | "FnOnce"))
                })
            }),
            bounds: bounds
                .iter()
                .filter_map(|bound| render_generic_bound(bound, &[], index, paths, generics).ok())
                .collect(),
        }),
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
        let substitutions =
            alias_type_substitutions(ty, &alias.generics.params, index, paths, generics)?;
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
    let short = resolved
        .rsplit("::")
        .next()
        .unwrap_or(&resolved)
        .split_once('<')
        .map_or_else(
            || resolved.rsplit("::").next().unwrap_or(&resolved),
            |(name, _)| name,
        )
        .to_owned();
    let mut projected_arguments = arguments
        .into_iter()
        .map(|argument| project_type(argument, index, paths, generics))
        .collect::<Result<Vec<_>, _>>()?;
    let mut substitutions = generics.clone();
    if let Some(declaration_generics) = nominal_generics(index.get(&path.id)) {
        let parameters = declaration_generics
            .params
            .iter()
            .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
            .collect::<Vec<_>>();
        for (parameter, argument) in parameters.iter().zip(&projected_arguments) {
            substitutions.insert(parameter.name.clone(), argument.clone());
        }
        for parameter in parameters.iter().skip(projected_arguments.len()) {
            let GenericParamDefKind::Type {
                default: Some(default),
                ..
            } = &parameter.kind
            else {
                break;
            };
            let argument = project_type(default, index, paths, &substitutions)?;
            substitutions.insert(parameter.name.clone(), argument.clone());
            projected_arguments.push(argument);
        }
    }
    let base_rust_path = rust_path
        .split_once('<')
        .map_or_else(|| rust_path.clone(), |(base, _)| base.to_owned());
    let rust_path = if projected_arguments.is_empty() {
        base_rust_path.clone()
    } else {
        format!(
            "{base_rust_path}<{}>",
            projected_arguments
                .iter()
                .map(ProjectedType::rust_type)
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let name = instantiated_nominal_name(&short, &rust_path, &projected_arguments);
    Ok(ProjectedType::Foreign {
        rust_path,
        name,
        base_rust_path,
        arguments: projected_arguments,
    })
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
#[path = "projection_tests.rs"]
mod tests;
