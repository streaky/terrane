// Stateful whole-package lowering. Emitter implementations are nested here so their
// shared control context remains private to the emitter subsystem.
mod emitter;

// Generated dependencies, runtime support, and backend-independent rendering helpers.
mod dependencies;
mod helpers;
mod runtime_support;

#[cfg(test)]
mod tests;

mod prelude {
    pub(super) use std::collections::BTreeSet;
    pub(super) use std::fmt::Write as _;

    pub(super) use indoc::indoc;
    pub(super) use num_bigint::BigInt;

    pub(super) use crate::{
        ScalarType, SourceFile, TypeCategory,
        rust_ir::{GeneratedModule, Item, Module, ModuleDestination, Program},
        semantics::{
            ArithmeticFamily, CoercionPolicy, ContextualConstant, ElementType,
            FloatMemberOperation, FunctionContract, MemberFamily, ObjectContract, ObjectField,
            ObjectIdentity, ObjectKind, SemanticPackage, SemanticUnit, StringFamily, SymbolKind,
            TypedBinding, ValueType, binding_read_value_is_reused, binding_span_is_mutated,
            binding_store_value_is_read, bound_method, contextual_constant,
            descriptor_binding_is_materialized, float_member_contract, is_numeric,
            narrowed_optional_type, narrowed_value_type, promoted_integer_type,
            string_call_selection,
        },
        syntax::{SyntaxKind, SyntaxNode},
    };

    pub(super) use super::dependencies::*;
    pub(super) use super::emitter::*;
    pub(super) use super::helpers::*;
    pub(super) use super::runtime_support::*;
}

pub(crate) use emitter::pipeline::lower;
