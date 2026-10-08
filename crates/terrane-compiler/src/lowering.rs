// Stateful whole-package lowering. Emitter implementations are nested here so their
// shared control context remains private to the emitter subsystem.
mod emitter;

// Generated dependencies, runtime support, and backend-independent rendering helpers.
#[path = "rust_interop/lowering.rs"]
mod dependencies;
pub(crate) mod helpers;
mod required_init;
mod runtime_support;
mod storage_names;
pub(crate) use storage_names::StorageNames;

#[cfg(test)]
mod tests;

mod prelude {
    pub(super) use std::collections::{BTreeMap, BTreeSet};
    pub(super) use std::fmt::Write as _;

    pub(super) use indoc::indoc;
    pub(super) use num_bigint::BigInt;

    pub(super) use crate::{
        ScalarType, SourceFile, TypeCategory,
        rust_ir::{GeneratedModule, Item, Module, ModuleDestination, Program},
        semantics::{
            ArithmeticFamily, BuiltinDescriptor, CallableEffects, CallableParameterType,
            ClosureWrites, CoercionPolicy, ContextualConstant, DescriptorContract, ElementType,
            FloatMemberArgument, FloatMemberOperation, FunctionContract, GenericParameterContract,
            InvocationMode, MemberFamily, ObjectIdentity, ObjectKind, SemanticPackage,
            SemanticUnit, SourceEnumContract, StringFamily, SymbolKind, TaskTransferability,
            TypedBinding, ValueType, binding_read_value_is_reused,
            binding_requires_mutable_storage, binding_span_is_mutated, binding_store_value_is_read,
            bound_method, collection_member_call, contextual_constant,
            descriptor_binding_is_materialized, descriptor_contract_by_identity,
            effective_object_fields, float_member_contract, is_numeric, narrowed_optional_type,
            narrowed_value_type, object_member_type, promoted_integer_type, string_call_selection,
        },
        syntax::{SyntaxKind, SyntaxNode},
    };

    pub(super) use super::LoweringFailure;
    pub(super) use super::dependencies::*;
    pub(super) use super::emitter::*;
    pub(super) use super::helpers::*;
    pub(super) use super::runtime_support::*;
}

#[derive(Clone, Debug)]
pub(crate) struct LoweringFailure {
    pub(crate) span: crate::Span,
    pub(crate) message: String,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TestRunnerCase {
    pub(crate) span: crate::Span,
    pub(crate) is_async: bool,
    pub(crate) throws: bool,
}

pub(crate) use emitter::pipeline::{lower, lower_tests};

pub(crate) fn debug_rust_name(name: &str) -> String {
    helpers::rust_name(name)
}

pub(crate) fn binding_storage_rust_name<'a>(
    unit: &'a crate::semantics::SemanticUnit,
    binding: &crate::semantics::TypedBinding,
) -> &'a str {
    StorageNames::for_unit(unit).binding(binding.span)
}

pub(crate) fn parameter_source_rust_name(
    unit: &crate::semantics::SemanticUnit,
    name: &str,
    span: crate::Span,
) -> String {
    StorageNames::for_unit(unit)
        .parameter(span)
        .map_or_else(|| helpers::rust_name(name), str::to_owned)
}

pub(crate) fn debug_function_name(
    package: &crate::semantics::SemanticPackage,
    contract: &crate::semantics::FunctionContract,
) -> String {
    helpers::function_name(package, contract)
}
