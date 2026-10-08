// Stateful whole-package lowering. Emitter implementations are nested here so their
// shared control context remains private to the emitter subsystem.
mod emitter;

// Generated dependencies, runtime support, and backend-independent rendering helpers.
#[path = "rust_interop/lowering.rs"]
mod dependencies;
pub(crate) mod helpers;
mod required_init;
mod runtime_support;

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
            CanonicalDefault, ClosureWrites, CoercionPolicy, ContextualConstant,
            DescriptorContract, EffectiveObjectField, ElementType, FloatMemberArgument,
            FloatMemberOperation, FunctionContract, GenericParameterContract, InvocationMode,
            MemberFamily, ObjectIdentity, ObjectKind, SemanticPackage, SemanticUnit,
            SourceEnumContract, StringFamily, SymbolKind, TaskTransferability, TypedBinding,
            ValueType, binding_read_value_is_reused, binding_requires_mutable_storage,
            binding_span_is_mutated, binding_store_value_is_read, bound_method, canonical_default,
            collection_member_call, contextual_constant, descriptor_binding_is_materialized,
            descriptor_contract_by_identity, effective_object_fields, float_member_contract,
            is_numeric, narrowed_optional_type, narrowed_value_type, object_member_type,
            promoted_integer_type, string_call_selection,
        },
        syntax::{SyntaxKind, SyntaxNode},
    };

    pub(super) use super::LoweringFailure;
    pub(super) use super::dependencies::*;
    pub(super) use super::emitter::*;
    pub(super) use super::helpers::*;
    pub(super) use super::required_init::*;
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

pub(crate) fn debug_binding_rust_name(
    unit: &crate::semantics::SemanticUnit,
    binding: &crate::semantics::TypedBinding,
) -> String {
    let identity = unit
        .flow_binding_ids
        .get(&(binding.span.file, binding.span.start, binding.span.end))
        .copied()
        .unwrap_or(binding.span);
    let Some(storage) = unit
        .typed_bindings
        .iter()
        .find(|candidate| candidate.span == identity)
    else {
        return debug_rust_name(&binding.name);
    };
    if let Some(function) = unit
        .functions
        .iter()
        .find(|function| Some(function.span) == storage.scope)
    {
        if let Some(parameter) = function
            .parameters
            .iter()
            .find(|parameter| parameter.span == storage.span)
        {
            if matches!(
                unit.flow_binding_types.get(&storage.span),
                Some(crate::semantics::ValueType::Union(_))
            ) {
                helpers::rust_local_name(&parameter.name, parameter.span)
            } else {
                debug_rust_name(&storage.name)
            }
        } else {
            helpers::rust_binding_name(storage)
        }
    } else {
        debug_rust_name(&binding.name)
    }
}

pub(crate) fn debug_function_name(
    package: &crate::semantics::SemanticPackage,
    contract: &crate::semantics::FunctionContract,
) -> String {
    helpers::function_name(package, contract)
}
