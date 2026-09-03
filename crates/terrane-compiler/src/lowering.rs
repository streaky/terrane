// Shared lowering state and whole-package orchestration.
mod pipeline;
mod state;

// Generated dependency and runtime support.
mod dependencies;
mod runtime_support;

// Source construct emission, ordered from items down to expressions.
mod call_support;
mod calls;
mod expressions;
mod items;
mod members;
mod statements;

// Emitter context queries and backend-independent rendering helpers.
mod context;
mod helpers;

#[cfg(test)]
mod tests;

mod prelude {
    pub(super) use std::cell::RefCell;
    pub(super) use std::collections::{BTreeMap, BTreeSet};
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
            TypedBinding, ValueType, binding_span_is_mutated, binding_store_value_is_read,
            bound_method, contextual_constant, descriptor_binding_is_materialized,
            float_member_contract, narrowed_optional_type, narrowed_value_type,
            promoted_integer_type, string_call_selection,
        },
        syntax::{SyntaxKind, SyntaxNode},
    };

    pub(super) use super::dependencies::*;
    pub(super) use super::helpers::*;
    pub(super) use super::runtime_support::*;
    pub(super) use super::state::*;
}

pub(crate) use pipeline::lower;
