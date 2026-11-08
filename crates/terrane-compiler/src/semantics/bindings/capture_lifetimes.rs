use std::collections::{BTreeMap, BTreeSet};

use crate::{
    InvocationMode, Span,
    syntax::{SyntaxKind, SyntaxNode},
};

use super::{
    async_tasks::{
        AutoTraitObligation, value_type_is_owned_static, value_type_satisfies_capture_auto_trait,
    },
    callbacks::{callback_contract, captured_binding},
};
use crate::semantics::{
    capabilities::walk_value_type,
    collections::collection_member_call,
    diagnostics::failure,
    expressions::infer_value_type,
    model::{SemanticFailure, SemanticPackage, SemanticUnit, ValueType},
    namespaces::function_contract_for_call,
    ownership::argument_for_parameter,
};

#[derive(Default)]
struct CaptureGraph<'a> {
    dependencies: BTreeMap<Span, BTreeSet<Span>>,
    static_roots: BTreeSet<Span>,
    unproven: BTreeMap<Span, &'a str>,
}

fn contains_callable(value: &ValueType) -> bool {
    let mut found = false;
    walk_value_type(value, &mut |value| {
        found |= matches!(
            value,
            ValueType::Function(..) | ValueType::AsyncFunction(..)
        );
    });
    found
}

fn carries_callable(unit: &SemanticUnit, value: &SyntaxNode) -> Result<bool, SemanticFailure> {
    Ok(infer_value_type(unit, value, &unit.typed_bindings)?
        .as_ref()
        .is_some_and(contains_callable))
}

impl CaptureGraph<'_> {
    fn edge(&mut self, from: Span, to: Span) {
        self.dependencies.entry(from).or_default().insert(to);
    }

    fn store(&mut self, unit: &SemanticUnit, target: &SyntaxNode, value: Span) {
        // Source object fields use their declared owned callable representation,
        // even when the containing object itself is local.
        if target.kind == SyntaxKind::MemberExpression {
            self.static_roots.insert(value);
        }
        if let Some(origins) =
            unit.flow_read_bindings
                .get(&(target.span.file, target.span.start, target.span.end))
        {
            for origin in origins {
                self.edge(*origin, value);
                // Reaching reference writes preserve caller-owned storage through
                // aliases and branch joins; an owned local copy has no such lender.
                // Parameter and field declarations also fix their callable ABI
                // to 'static, even for a parameter passed by value.
                let external = unit.reference_assignment_provenance.iter().any(|version| {
                    version.assignment == *origin
                        && (version.provenance.external_lender
                            || version.provenance.lender_parameter.is_some())
                }) || unit.functions.iter().any(|function| {
                    function.parameters.iter().any(|parameter| {
                        parameter.span == *origin
                            && parameter.value_type.as_ref().is_some_and(contains_callable)
                    })
                });
                if external {
                    self.static_roots.insert(value);
                }
            }
        }
        if target.kind == SyntaxKind::Name {
            self.edge(target.span, value);
        } else if let Some(root) = target.children.first() {
            self.store(unit, root, value);
        }
    }

    fn closure(
        &mut self,
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
        else {
            return Ok(());
        };
        let owned = infer_value_type(unit, node, &unit.typed_bindings)?
            .as_ref()
            .is_none_or(value_type_is_owned_static);
        if !owned {
            self.unproven.remove(&node.span);
            return Ok(());
        }
        if contract.written_invocation_mode != InvocationMode::Shared || contract.is_async {
            self.static_roots.insert(node.span);
        }
        for capture in &contract.captures {
            if let Some(binding) = captured_binding(package, unit, contract, capture) {
                self.edge(node.span, binding.span);
            }
        }
        Ok(())
    }

    fn call(
        &mut self,
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<bool, SemanticFailure> {
        let [callee, arguments] = node.children.as_slice() else {
            return Ok(false);
        };
        if let Some((receiver, _, operation)) =
            collection_member_call(unit, callee, &unit.typed_bindings)?
        {
            if matches!(operation.as_str(), "append" | "set") {
                for argument in &arguments.children {
                    let value = argument.children.last().unwrap_or(argument);
                    self.store(unit, receiver, value.span);
                }
            }
            return Ok(false);
        }
        let contract = function_contract_for_call(package, unit, callee)
            .or_else(|| callback_contract(package, unit, callee));
        let Some(contract) = contract else {
            let mut value = callee;
            while value.kind == SyntaxKind::GroupExpression
                && let [inner] = value.children.as_slice()
            {
                value = inner;
            }
            let identity =
                unit.flow_binding_ids
                    .get(&(value.span.file, value.span.start, value.span.end));
            let callable = unit
                .typed_bindings
                .iter()
                .find(|binding| Some(&binding.span) == identity);
            if let Some(
                ValueType::Function(parameters, _, _)
                | ValueType::AsyncFunction(parameters, _, _, _),
            ) = callable.map(|binding| &binding.value_type)
            {
                for (index, argument) in arguments.children.iter().enumerate() {
                    if parameters
                        .get(index)
                        .or_else(|| parameters.last().filter(|p| p.is_variadic()))
                        .is_some_and(|parameter| contains_callable(parameter.value_type_ref()))
                    {
                        self.static_roots
                            .insert(argument.children.last().unwrap_or(argument).span);
                    }
                }
                return Ok(true);
            }
            return Ok(false);
        };
        let projected = super::projected_function_for_call(
            package,
            unit,
            callee,
            crate::syntax::call_is_unsafe(node),
        );
        for (index, parameter) in contract.parameters.iter().enumerate() {
            if parameter.value_type.as_ref().is_some_and(contains_callable)
                && projected.as_ref().is_none_or(|function| {
                    function.parameters.get(index).is_some_and(|parameter| {
                        let mut ty = &parameter.ty;
                        while let crate::rust_interop::projection::ProjectedType::Optional(inner) =
                            ty
                        {
                            ty = inner;
                        }
                        matches!(
                            ty,
                            crate::rust_interop::projection::ProjectedType::Callback {
                                retained: true,
                                ..
                            }
                        )
                    })
                })
                && let Some(value) = argument_for_parameter(unit, arguments, contract, index)
            {
                self.static_roots.insert(value.span);
            }
        }
        if let Some(ValueType::TypeParameter(name)) = &contract.return_type {
            for (index, parameter) in contract.parameters.iter().enumerate() {
                if parameter.value_type.as_ref() == Some(&ValueType::TypeParameter(name.clone()))
                    && let Some(value) = argument_for_parameter(unit, arguments, contract, index)
                {
                    self.edge(node.span, value.span);
                }
            }
        }
        // A declared concrete callable result already has the callee's static ABI.
        // Only a generic pass-through carries a caller-local environment out.
        Ok(true)
    }

    fn visit(
        &mut self,
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::TypeExpression {
            return Ok(());
        }
        if matches!(
            node.kind,
            SyntaxKind::AnonymousFunction | SyntaxKind::FunctionDeclaration
        ) {
            self.closure(package, unit, node)?;
        } else if node.kind == SyntaxKind::Name {
            if let Some(origins) =
                unit.flow_read_bindings
                    .get(&(node.span.file, node.span.start, node.span.end))
            {
                for origin in origins {
                    self.edge(node.span, *origin);
                }
            }
        } else {
            let closed_call =
                node.kind == SyntaxKind::CallExpression && self.call(package, unit, node)?;
            if !closed_call {
                for child in &node.children {
                    self.edge(node.span, child.span);
                }
            }
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && let [target, .., value] = node.children.as_slice()
        {
            self.edge(target.span, value.span);
            if node.kind == SyntaxKind::Assignment {
                self.store(unit, target, value.span);
            }
            if carries_callable(unit, value)?
                && (unit.containing_function(node.span).is_none()
                    || node.kind == SyntaxKind::Assignment
                        && matches!(
                            target.kind,
                            SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
                        ))
            {
                self.static_roots.insert(value.span);
            }
        }
        if node.kind == SyntaxKind::ForStatement
            && let [target, collection, ..] = node.children.as_slice()
            && target.kind == SyntaxKind::ForTarget
        {
            for name in &target.children {
                self.edge(name.span, collection.span);
            }
        }
        if node.kind == SyntaxKind::ReturnStatement
            && let Some(value) = node.children.first()
            && carries_callable(unit, value)?
        {
            self.static_roots.insert(value.span);
        }
        for (index, child) in node.children.iter().enumerate() {
            if node.kind == SyntaxKind::MemberExpression && index != 0 {
                continue;
            }
            self.visit(package, unit, child)?;
        }
        Ok(())
    }

    fn validate(self, unit: &SemanticUnit) -> Result<(), SemanticFailure> {
        let mut pending: Vec<_> = self.static_roots.iter().copied().collect();
        let mut visited = BTreeSet::new();
        while let Some(span) = pending.pop() {
            if !visited.insert(span) {
                continue;
            }
            if let Some(capture) = self.unproven.get(&span) {
                return Err(failure(
                    &unit.source,
                    "T0084",
                    format!(
                        "escaping callable requires a 'static captured environment; cannot prove the lifetime of `{capture}`"
                    ),
                    span,
                ));
            }
            if let Some(dependencies) = self.dependencies.get(&span) {
                pending.extend(dependencies.iter().copied());
            }
        }
        Ok(())
    }
}

pub(super) fn validate_owned_capture_lifetimes(
    package: &SemanticPackage,
    unit: &SemanticUnit,
) -> Result<(), SemanticFailure> {
    let mut unproven = BTreeMap::new();
    for contract in &unit.functions {
        for capture in &contract.captures {
            if let Some(binding) = captured_binding(package, unit, contract, capture)
                && !crate::semantics::scoped_regions::field_has_native_loan(
                    package,
                    &binding.value_type,
                )
                && !value_type_satisfies_capture_auto_trait(
                    package,
                    unit,
                    contract.span.start,
                    &binding.value_type,
                    AutoTraitObligation::Static,
                )
            {
                unproven.insert(contract.span, capture.as_str());
            }
        }
    }
    if unproven.is_empty() {
        return Ok(());
    }
    let mut graph = CaptureGraph {
        unproven,
        ..CaptureGraph::default()
    };
    graph.visit(package, unit, &unit.tree.root)?;
    graph.validate(unit)
}
