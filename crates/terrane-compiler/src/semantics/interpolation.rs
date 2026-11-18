//! Interpolation selects ordinary calls before ownership/effect validation. Keys
//! identify explicitly registered callable storage, never ordinary function names.
use std::collections::{BTreeMap, BTreeSet};

use super::{
    diagnostics::{failure, node_text},
    expressions::infer_value_type,
    model::{CompileTimeValue, SemanticFailure, SemanticPackage, SemanticUnit, ValueType},
    modifiers::{ModifierOperation, ResolvedModifier},
};
use crate::{
    ScalarType, SourceFile, Span,
    syntax::{SyntaxKind, SyntaxNode},
};

struct Registration {
    key: String,
    name: String,
    owner: Span,
    declaration: Span,
}

fn registrations(
    source: &SourceFile,
    node: &SyntaxNode,
    owner: Option<Span>,
    modifiers: &BTreeMap<Span, Vec<ResolvedModifier>>,
    result: &mut Vec<Registration>,
) -> Result<(), SemanticFailure> {
    let owner = if super::model::is_function_node(node) {
        Some(node.span)
    } else {
        owner
    };
    if let Some(applications) = modifiers.get(&node.span) {
        for application in applications
            .iter()
            .filter(|application| application.operation == ModifierOperation::InterpolationRenderer)
        {
            let Some(owner) = owner else {
                return Err(failure(
                    source,
                    "T0250",
                    "an interpolation renderer requires a function-local callable binding",
                    node.span,
                ));
            };
            let Some(target) = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            else {
                return Err(failure(
                    source,
                    "T0250",
                    "an interpolation renderer requires a bare binding target",
                    node.span,
                ));
            };
            if !matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
                || target != node.children.first().unwrap_or(target)
                    && node.kind == SyntaxKind::Assignment
            {
                return Err(failure(
                    source,
                    "T0250",
                    "an interpolation renderer requires a bare binding target",
                    node.span,
                ));
            }
            let CompileTimeValue::Map(fields) = &application.payload else {
                unreachable!("validated modifier payload")
            };
            let key = fields
                .iter()
                .find_map(|(key, value)| match (key, value) {
                    (CompileTimeValue::String(field), CompileTimeValue::String(value))
                        if field == "key" =>
                    {
                        Some(value)
                    }
                    _ => None,
                })
                .expect("validated renderer key");
            let name = node_text(source, target);
            if name == "_"
                || result
                    .iter()
                    .any(|prior| prior.owner == owner && (prior.key == *key || prior.name == name))
            {
                return Err(failure(
                    source,
                    "T0250",
                    "register each interpolation key and callable binding only once per function",
                    target.span,
                ));
            }
            result.push(Registration {
                key: key.clone(),
                name: name.to_owned(),
                owner,
                declaration: node.span,
            });
        }
    }
    for child in &node.children {
        registrations(source, child, owner, modifiers, result)?;
    }
    Ok(())
}

fn node(kind: SyntaxKind, at: Span, children: Vec<SyntaxNode>) -> SyntaxNode {
    SyntaxNode::new(kind, at, 0..0, children)
}

fn name(at: Span, text: String) -> SyntaxNode {
    let mut result = node(SyntaxKind::Name, at, Vec::new());
    result.name_override = Some(text);
    result
}

fn call(at: Span, callee: SyntaxNode, values: Vec<SyntaxNode>) -> SyntaxNode {
    let arguments = values
        .into_iter()
        .map(|value| {
            let span = value.span;
            node(SyntaxKind::Argument, span, vec![value])
        })
        .collect();
    node(
        SyntaxKind::CallExpression,
        at,
        vec![callee, node(SyntaxKind::ArgumentList, at, arguments)],
    )
}

fn method_call(
    at: Span,
    value: SyntaxNode,
    key: Option<&SyntaxNode>,
    source: &SourceFile,
) -> SyntaxNode {
    let member_at = key.map_or(Span::new(at.file, at.start, at.start), |key| key.span);
    let callee = node(
        SyntaxKind::MemberExpression,
        at,
        vec![value, name(member_at, "render".to_owned())],
    );
    let arguments = key
        .map(|key| {
            let mut literal = node(SyntaxKind::Literal, key.span, Vec::new());
            literal.name_override = Some(format!("'{}'", node_text(source, key)));
            literal
        })
        .into_iter()
        .collect();
    call(at, callee, arguments)
}

fn rewrite_keys(source: &SourceFile, current: &mut SyntaxNode, registrations: &[Registration]) {
    for child in &mut current.children {
        rewrite_keys(source, child, registrations);
    }
    if current.kind != SyntaxKind::Interpolation {
        return;
    }
    let Some(key) = current.children.get(1) else {
        return;
    };
    let selected = registrations
        .iter()
        .filter(|registration| {
            registration.key == node_text(source, key)
                && registration.owner.start <= current.span.start
                && current.span.end <= registration.owner.end
        })
        .min_by_key(|registration| registration.owner.end - registration.owner.start);
    let placeholder = node(SyntaxKind::Error, current.span, Vec::new());
    let value = std::mem::replace(&mut current.children[0], placeholder);
    let key = &current.children[1];
    current.children[0] = if let Some(selected) = selected {
        call(
            current.span,
            name(key.span, selected.name.clone()),
            vec![value],
        )
    } else {
        method_call(current.span, value, Some(key), source)
    };
    current.interpolation_renderer = true;
}

/// Registration discovery precedes type inference so callback reads participate
/// in the ordinary capture, availability and effect machinery.
pub(super) fn normalize_keys(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &mut package.units {
        let mut registered = Vec::new();
        registrations(
            &unit.source,
            &unit.tree.root,
            None,
            &package.declaration_modifiers,
            &mut registered,
        )?;
        rewrite_keys(&unit.source, &mut unit.tree.root, &registered);
    }
    Ok(())
}

fn scalar_display(value: &ValueType) -> bool {
    match value {
        ValueType::Scalar(scalar) => *scalar != ScalarType::Bytes,
        ValueType::Union(arms) => arms.iter().all(scalar_display),
        ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner) => scalar_display(inner.value_type_ref()),
        ValueType::Optional(inner) => scalar_display(inner),
        _ => false,
    }
}

fn collect_default_calls(
    unit: &SemanticUnit,
    current: &SyntaxNode,
    calls: &mut BTreeSet<Span>,
) -> Result<(), SemanticFailure> {
    for child in &current.children {
        collect_default_calls(unit, child, calls)?;
    }
    if current.kind == SyntaxKind::Interpolation && current.children.len() == 1 {
        let value = &current.children[0];
        let value_type = infer_value_type(unit, value, &unit.typed_bindings)?.ok_or_else(|| {
            failure(
                &unit.source,
                "T0251",
                "cannot infer interpolated value type",
                value.span,
            )
        })?;
        if !scalar_display(&value_type) {
            calls.insert(current.span);
        }
    }
    Ok(())
}

fn replace_default_calls(
    source: &SourceFile,
    current: &mut SyntaxNode,
    calls: &mut BTreeSet<Span>,
) {
    for child in &mut current.children {
        replace_default_calls(source, child, calls);
    }
    if calls.remove(&current.span) {
        let value = current.children.remove(0);
        current
            .children
            .push(method_call(current.span, value, None, source));
        current.interpolation_renderer = true;
    }
}

/// Initial expression types distinguish scalar display from object rendering.
/// Materialize method calls before capture/receiver/effect analysis, and refresh
/// only the bindings affected by introducing those calls through the normal pass.
pub(super) fn normalize_objects(package: &mut SemanticPackage) -> Result<bool, SemanticFailure> {
    let mut changed = false;
    for unit in &mut package.units {
        let mut calls = BTreeSet::new();
        collect_default_calls(unit, &unit.tree.root, &mut calls)?;
        changed |= !calls.is_empty();
        replace_default_calls(&unit.source, &mut unit.tree.root, &mut calls);
    }
    Ok(changed)
}

fn validate_node(unit: &SemanticUnit, current: &SyntaxNode) -> Result<(), SemanticFailure> {
    if current.kind == SyntaxKind::Interpolation {
        let value = &current.children[0];
        let actual = infer_value_type(unit, value, &unit.typed_bindings)?;
        if actual.as_ref().is_none_or(|value| !scalar_display(value))
            || current.interpolation_renderer
                && actual != Some(ValueType::Scalar(ScalarType::String))
        {
            return Err(failure(
                &unit.source,
                "T0251",
                "interpolation requires scalar text display or a synchronous renderer returning string",
                current.span,
            ));
        }
        if current.children.len() > 1
            && value.kind == SyntaxKind::CallExpression
            && value.children[0].kind == SyntaxKind::Name
            && unit.flow_availability.get(&(
                value.children[0].span.file,
                value.children[0].span.start,
                value.children[0].span.end,
            )) == Some(&super::model::FlowAvailability::MayBeUnassigned)
        {
            return Err(failure(
                &unit.source,
                "T0252",
                "an interpolation renderer must be assigned on every reachable path before use",
                value.children[0].span,
            ));
        }
    }
    for child in &current.children {
        validate_node(unit, child)?;
    }
    Ok(())
}

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        let mut registered = Vec::new();
        registrations(
            &unit.source,
            &unit.tree.root,
            None,
            &package.declaration_modifiers,
            &mut registered,
        )?;
        for registration in registered {
            let value = unit
                .typed_bindings
                .iter()
                .find(|binding| binding.span == registration.declaration);
            if !value.is_some_and(|binding| {
                matches!(&binding.value_type,
                ValueType::Function(parameters, result, _) if parameters.len() == 1
                    && !parameters[0].is_variadic()
                    && result.value_type_ref() == &ValueType::Scalar(ScalarType::String))
            }) {
                return Err(failure(
                    &unit.source,
                    "T0251",
                    "a registered interpolation renderer must be a synchronous one-argument callable returning string",
                    registration.declaration,
                ));
            }
        }
        validate_node(unit, &unit.tree.root)?;
    }
    Ok(())
}
