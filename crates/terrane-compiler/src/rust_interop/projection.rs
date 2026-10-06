//! Shared projection metadata and public query APIs.
//!
//! Extend the owning module rather than adding admission or resolution policy here.
//! Implementation modules live under `projection/`:
//! - `aliases.rs`: same-owner callable aliases and Rust-path aliases.
//! - `artifacts.rs`: cached and remote projection artifact handling.
//! - `borrowed_graph.rs`: borrowed-operation owners and optional boundary conversions.
//! - `cache_identity.rs`: projection identity, effective target, and cache paths.
//! - `callable.rs`: generic callable selection, bound matching, and blanket trait recipes.
//! - `cargo_workspace.rs`: Cargo workspace execution and dependency lock metadata.
//! - `data.rs`: fields, constants, and projected-item normalization.
//! - `dependency_owners.rs`: dependency reachability, owner binding, and Rust-root rewrites.
//! - `enum_payload.rs`: enum payload projection and boundary conversion.
//! - `foreign_impls.rs`: implementation discovery and probes across dependency crates.
//! - `functions.rs`: complete function-signature admission.
//! - `history.rs`: projection-lock provenance and replay history.
//! - `interfaces.rs`: projected interfaces, associated types, and trait operations.
//! - `items.rs`: Rustdoc item admission and nominal generic parameters.
//! - `macros.rs`: macro-generated public reexports.
//! - `methods.rs`: impl methods, chain owners, and output aliases.
//! - `namespace_overlays.rs`: Cargo metadata namespace overlays.
//! - `partial.rs`: unavailable-item partial contracts.
//! - `reexports.rs`: supplemental Rustdoc discovery for reexports.
//! - `resolution.rs`: the resolution transaction and impl-witness/bound admission policy.
//! - `rustdoc_support.rs`: Rustdoc path resolution, type inspection, and boundary capabilities.
//! - `source_rendering.rs`: Terrane declaration rendering and import closure.
//! - `type_rendering.rs`: Rust type spelling and instantiated nominal names.
//! - `types.rs`: value-type admission.
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use quote::ToTokens as _;
use rustdoc_types::{
    AssocItemConstraintKind, Crate as RustdocCrate, Function, GenericArg, GenericArgs,
    GenericBound, GenericParamDef, GenericParamDefKind, Generics, Id, Impl, Item, ItemEnum,
    ItemSummary, Path as RustdocPath, Struct, Term, Type, Visibility, WherePredicate,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{InvocationMode, RustDependency};

mod aliases;
mod artifacts;
mod borrowed_graph;
mod cache_identity;
mod callable;
mod cargo_workspace;
mod data;
mod dependency_owners;
mod enum_payload;
mod foreign_impls;
mod functions;
mod history;
mod interfaces;
mod items;
mod macros;
mod methods;
mod namespace_overlays;
mod partial;
mod reexports;
mod resolution;
mod rustdoc_support;
mod source_rendering;
mod type_rendering;
mod types;

pub use super::generated_projection::{GeneratedProjectionUnit, generated_projection_units};
pub use crate::RUSTDOC_TOOLCHAIN;
pub use resolution::resolve;

use artifacts::{
    ProjectionCacheIdentity, PublishedProjection, fetch_remote_projection, projection_content_hash,
    remove_legacy_projection_cache, write_cache_atomically, write_if_changed,
};
use cache_identity::cache_identity;
use callable::{
    GenericMonomorphisations, concrete_into_future_output, generic_bounds,
    generic_monomorphisations, project_callable_adapter_bounds, render_generic_bound,
    render_generic_bounds, resolved_path_type_arguments, rust_bound_roots, rust_lifetimes,
    trait_bound_name, type_contains_lifetime_argument, type_mentions_generic,
};
use cargo_workspace::{
    CargoExecution, CargoToolchain, containment, persist_dependency_lock,
    resolved_dependency_metadata, run_cargo, seed_dependency_lock, write_workspace,
    write_workspace_with_bound_dependencies,
};
use data::{
    SourceConstantCache, default_generic_instantiation, extern_rust_path,
    normalize_projected_items, project_rust_constant_expression, project_struct_fields,
    projected_constant_name, source_constant_expression,
};
use dependency_owners::{
    canonicalize_projected_type_names, decline_unnameable_bound_owners,
    decline_unrepresentable_error_types, enforce_transitive_reachability,
    projected_bound_dependencies, recursive_owner_dependencies,
    resolve_cross_dependency_boundary_conversions, rewrite_projected_function_root,
    rewrite_projected_owner_root, rewrite_projected_rust_root, rewrite_rust_bound_root,
    rust_path_owner,
};
use enum_payload::{project_enum_payload, project_multi_enum_payload};
use functions::{project_function, project_function_with_generics, promote_async_endpoint_methods};
use interfaces::{
    ProjectedTraitOperation, merge_projected_trait_operations, owner_trait_namespace,
    project_external_provided_trait_methods, project_interface, projected_interface_impl_question,
    trait_fallback_namespace, trait_operation_docs,
};
use items::{project_rustdoc, projected_nominal_generic_parameters};
use macros::project_macro;
use methods::{
    alias_type_substitutions, expand_output_alias, project_chain_owner, project_methods,
    resolved_nominal_id,
};
use namespace_overlays::{apply_namespace_overlays, namespace_overlays_from_metadata};
use partial::{partial_projection, partial_projection_references};
use reexports::{
    ReexportRustdoc, external_reexport_rustdocs, generate_rustdoc, prefer_alias,
    provider_fragment_public_paths,
};
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
    render_unavailable_projection, source_foreign_aliases, unavailable_member_map,
};
use type_rendering::{
    canonicalize_rust_path, instantiated_nominal_name, instantiated_type_name, nominal_generics,
    render_resolved_path, render_rust_type,
};
use types::{
    immediate_generic_input, project_associated_binding, project_borrowed_graph_type,
    project_dyn_interface, project_invocation_scoped_type, project_type,
    projectable_interface_bound, projected_error_name, type_arguments, type_contains_borrowed_ref,
};

const PROJECTION_SCHEMA: &str = "259";
pub type ProjectedMemberDemands = BTreeMap<(String, String), BTreeSet<String>>;
pub type ProjectionDemandSites = BTreeMap<(String, String, Option<String>), BTreeSet<String>>;
pub const GENERATED_PROJECTION_FILE: &str = "terrane-projection.generated.trn";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Projection {
    pub cache_identity: String,
    #[serde(default)]
    pub content_hash: String,
    pub dependencies: Vec<ProjectedDependency>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub native_owner_aliases: BTreeMap<String, String>,
    pub containment: Containment,
    #[serde(default)]
    pub bound_dependencies: Vec<ProjectedBoundDependency>,
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
pub struct ProjectedNativeAlias {
    pub native_type: ProjectedType,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub generic_parameters: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub generic_defaults: Vec<Option<ProjectedType>>,
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
pub struct ProjectedDependency {
    pub name: String,
    pub package: String,
    pub version: String,
    pub items: Vec<ProjectedItem>,
    pub declined: Vec<DeclinedItem>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub native_alias_identities: BTreeMap<String, ProjectedNativeAlias>,
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
    /// Inherent impl bounds, keyed by the selected nominal owner's generic slots.
    /// These do not introduce method-owned generic arguments.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operation_owner_generics: Vec<ProjectedGenericParameter>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<ProjectedType>,
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
    Reference {
        inner: Box<ProjectedType>,
        mutable: bool,
        lifetime: Option<String>,
    },
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
            Self::Reference {
                inner,
                mutable,
                lifetime,
            } => format!(
                "&{}{}{}",
                lifetime
                    .as_ref()
                    .map_or_else(String::new, |name| format!("{name} ")),
                if *mutable { "mut " } else { "" },
                inner.rust_type()
            ),
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
            | Self::Optional(item)
            | Self::Reference { inner: item, .. } => item.bind_associated(replacement),
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
            | Self::Optional(item)
            | Self::Reference { inner: item, .. } => item.contains_opaque(),
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
            | Self::Optional(item)
            | Self::Reference { inner: item, .. } => item.contains_generic(generic),
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
            | Self::Optional(item)
            | Self::Reference { inner: item, .. } => item.contains_open_generic(),
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
            Self::Reference { inner, .. } => inner.terrane_name(),
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
            | Self::Reference { .. }
            | Self::Foreign { .. } => true,
            _ => false,
        }
    }

    pub(crate) fn contains_borrowed_result(&self) -> bool {
        match self {
            Self::BorrowedString
            | Self::Reference { .. }
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
        | ProjectedType::Optional(item)
        | ProjectedType::Reference { inner: item, .. } => {
            collect_nested_projected_types(item, name, candidates);
        }
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
        ProjectedType::Reference { inner, .. } => projected_type_owner(inner),
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

fn projected_item_for_native_owner<'a>(
    items: impl Iterator<Item = &'a ProjectedItem>,
    name: &str,
    owner_path: &str,
) -> Option<&'a ProjectedItem> {
    let owner_path = owner_path
        .split_once('<')
        .map_or(owner_path, |(base, _)| base);
    let mut matching = items.filter(|item| {
        item.name == name && projected_owner_path_matches(&item.rust_path, owner_path)
    });
    let item = matching.next()?;
    matching.next().is_none().then_some(item)
}

impl Projection {
    /// Renders imported projected source units with requested members.
    ///
    /// # Errors
    ///
    /// Returns an error when an imported projected name is ambiguous.
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
        let canonical_members =
            source_rendering::source_member_demands(self, &all_items, demanded_members);
        let demanded_members = &canonical_members;
        let imports = expanded_source_imports(&all_items, imports, demanded_members);
        let mut sources = imports
            .iter()
            .map(|(namespace, names)| {
                self.render_imported_namespace(
                    namespace,
                    names,
                    &all_items,
                    demanded_members,
                    unavailable_members,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        sources.retain(|(_, source, _)| !source.is_empty());
        Ok(Self::finalize_projected_sources(
            sources,
            allow_namespace_cycles,
        ))
    }

    fn render_imported_namespace(
        &self,
        namespace: &str,
        names: &BTreeSet<String>,
        all_items: &[&ProjectedItem],
        demanded_members: &ProjectedMemberDemands,
        unavailable_members: Option<&UnavailableMemberMap<'_>>,
    ) -> Result<(String, String, BTreeSet<String>), String> {
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
            .filter(|item| item.namespace == namespace && names.contains(&item.name))
            .collect::<Vec<_>>();
        if selected.is_empty() {
            return Ok((namespace.to_owned(), String::new(), BTreeSet::new()));
        }
        let foreign = collect_source_foreign(all_items, &selected, demanded_members);
        let aliases = source_foreign_aliases(&foreign, all_items, namespace);
        let mut ordered_foreign = foreign.iter().collect::<Vec<_>>();
        ordered_foreign.sort_by_key(|(rust_path, name)| {
            let projected_item = projected_item_for_foreign(all_items, rust_path, name);
            let cross_namespace = projected_item.is_some_and(|item| item.namespace != namespace);
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
            projected_source_dependencies(&ordered_foreign, all_items, namespace);
        let mut text = format!("namespace {}\n\n", namespace.trim_start_matches('/'));
        let mut rendered_foreign = BTreeSet::new();
        for (rust_path, _) in ordered_foreign {
            let name = &aliases[rust_path];
            let projected_item =
                projected_item_for_foreign(all_items, rust_path, &foreign[rust_path]);
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
            if matches!(
                item.kind,
                ProjectedKind::ForeignType { .. }
                    | ProjectedKind::Enum { .. }
                    | ProjectedKind::Interface(_)
            ) && let Some(canonical_name) = aliases.get(&item.rust_path)
                && canonical_name != &item.name
                && let Some(canonical) = projected_item_for_foreign(
                    all_items,
                    &item.rust_path,
                    &foreign[&item.rust_path],
                )
            {
                source_rendering::render_foreign_import(
                    &mut text,
                    &canonical.namespace,
                    &canonical.name,
                    &item.name,
                );
            }
            source_rendering::render_callable_declaration(&mut text, item, &aliases);
        }
        Ok((namespace.to_owned(), text, source_dependencies))
    }

    fn finalize_projected_sources(
        sources: Vec<(String, String, BTreeSet<String>)>,
        allow_namespace_cycles: bool,
    ) -> Vec<(String, String)> {
        if allow_namespace_cycles {
            sources
                .into_iter()
                .map(|(namespace, source, _)| (namespace, source))
                .collect()
        } else {
            Self::order_projected_sources(sources)
        }
    }
    /// Renders complete projected dependency sources for callers without importing-package syntax.
    ///
    /// # Errors
    ///
    /// Returns an error when an imported projected name is ambiguous.
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
    ) -> Vec<(String, String)> {
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
                // Generated declarations are staged together, including mutually
                // referring namespaces. Authored source cycles are checked separately.
                ordered.extend(
                    sources
                        .into_iter()
                        .map(|(namespace, text, _)| (namespace, text)),
                );
                return ordered;
            };
            let (namespace, text, _) = sources.remove(index);
            ordered.push((namespace, text));
        }
        ordered
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
        projected_item_for_native_owner(
            self.dependencies
                .iter()
                .flat_map(|dependency| &dependency.items),
            owner_name,
            owner_path,
        )
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
            name,
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
        let items = self
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items);
        if item.name == *name && projected_owner_path_matches(&item.rust_path, owner_path) {
            return Some((item.namespace.clone(), item.name.clone()));
        }
        projected_item_for_native_owner(items, name, owner_path)
            .map(|candidate| (candidate.namespace.clone(), candidate.name.clone()))
    }

    pub(crate) fn projected_member_scoped_family_owner(
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
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => {
                &methods
                    .iter()
                    .chain(static_methods)
                    .find(|function| function.name == member)?
                    .result
            }
            _ => return None,
        };
        let ProjectedType::InvocationScoped {
            name, rust_type, ..
        } = result
        else {
            return None;
        };
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|candidate| candidate.name == *name)
            .find(|candidate| {
                projected_owner_path_matches(
                    &self.canonical_native_type(&candidate.rust_path),
                    &self.canonical_native_type(rust_type),
                )
            })
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
            .min_by_key(|item| {
                (
                    // Canonical native equivalence must not replace an advertised source slot.
                    !projected_owner_path_matches(&item.rust_path, owner),
                    preferred_name.is_some_and(|name| item.name != name),
                )
            })
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

fn rustdoc_public_paths(document: &RustdocCrate) -> BTreeMap<Id, String> {
    terrane_rust_analysis::public_paths(document)
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

fn io_error(context: &'static str) -> impl FnOnce(std::io::Error) -> ProjectionError {
    move |error| ProjectionError {
        message: format!("{context}: {error}"),
    }
}

#[cfg(test)]
#[path = "projection_tests.rs"]
mod tests;
