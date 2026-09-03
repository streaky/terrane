// Compiler-owned semantic data and the ordered package analysis pipeline.
mod analysis;
mod model;

// Namespace, import, bootstrap-surface, and lexical name resolution.
mod bootstrap;
mod namespaces;
mod scopes;

// Type contracts and expression-family inference.
mod calls;
mod collections;
mod contracts;
mod expressions;
mod member_inference;
mod numeric;
mod types;

// Object, ownership, binding-lifetime, and diagnostic validation.
mod bindings;
mod diagnostics;
mod objects;
mod ownership;

#[cfg(test)]
mod tests;

mod prelude {
    pub(super) use std::collections::{BTreeMap, BTreeSet};

    pub(super) use num_bigint::BigInt;
    pub(super) use num_traits::{FromPrimitive, ToPrimitive};

    pub(super) use crate::syntax::{SyntaxKind, SyntaxNode, SyntaxTree};
    pub(super) use crate::{
        Diagnostic, Package, ScalarType, SourceFile, Span, TypeCategory, lexer, parser,
    };

    pub(super) use super::analysis::*;
    pub(super) use super::bindings::*;
    pub(super) use super::bootstrap::*;
    pub(super) use super::calls::*;
    pub(super) use super::collections::*;
    pub(super) use super::contracts::*;
    pub(super) use super::diagnostics::*;
    pub(super) use super::expressions::*;
    pub(super) use super::member_inference::*;
    pub(super) use super::model::*;
    pub(super) use super::namespaces::*;
    pub(super) use super::numeric::*;
    pub(super) use super::objects::*;
    pub(super) use super::ownership::*;
    pub(super) use super::scopes::*;
    pub(super) use super::types::*;
}

pub use analysis::analyze;
pub(crate) use bindings::{binding_store_value_is_read, descriptor_binding_is_materialized};
pub(crate) use contracts::{descriptor_expression_category, descriptor_expression_type};
pub(crate) use diagnostics::{binding_span_is_mutated, warnings};
pub(crate) use member_inference::string_call_selection;
pub use model::{
    ArithmeticFamily, BOOTSTRAP_VERSION, BoundMethod, ElementType, EvaluationKind, EvaluationStep,
    FunctionContract, MemberFamily, Namespace, ObjectContract, ObjectField, ObjectIdentity,
    ObjectKind, ParameterContract, SemanticFailure, SemanticPackage, SemanticUnit, Symbol,
    SymbolKind, TextUnit, TypedBinding, ValueType, Visibility,
};
pub(crate) use model::{
    CoercionPolicy, ContextualConstant, FloatMemberOperation, StringFamily, float_member_contract,
};
pub(crate) use numeric::{bound_method, contextual_constant, promoted_integer_type};
pub(crate) use types::{narrowed_optional_type, narrowed_value_type};
