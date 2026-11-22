//! Interpolation selects ordinary calls before ownership/effect validation. Keys
//! identify explicitly registered callable storage, never ordinary function names.
use std::collections::BTreeMap;

use super::super::{
    diagnostics::{failure, node_text},
    model::{CompileTimeValue, SemanticFailure, SemanticPackage, SemanticUnit, ValueType},
    modifiers::{ModifierOperation, ResolvedModifier},
};
use crate::{
    ScalarType, SourceFile, Span,
    syntax::{SyntaxKind, SyntaxNode},
    tokens::TokenKind,
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
    let owner = if super::super::model::is_function_node(node) {
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
            let key_source = SourceFile::new(0, "interpolation-key.trn".into(), key.clone());
            let valid_key = crate::lexer::lex(&key_source).is_ok_and(|lexed| {
                let mut tokens = lexed
                    .tokens
                    .iter()
                    .filter(|token| !matches!(token.kind, TokenKind::Newline | TokenKind::Eof));
                tokens.next().is_some_and(|token| {
                    token.kind == TokenKind::Identifier
                        && token.span.start == 0
                        && token.span.end == key.len()
                }) && tokens.next().is_none()
            });
            if !valid_key {
                return Err(failure(
                    source,
                    "T0250",
                    "interpolation renderer key must be a selector identifier",
                    target.span,
                ));
            }
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

fn rewrite_keys(
    source: &SourceFile,
    current: &mut SyntaxNode,
    registrations: &[Registration],
) -> Result<(), SemanticFailure> {
    for child in &mut current.children {
        rewrite_keys(source, child, registrations)?;
    }
    if current.kind != SyntaxKind::Interpolation {
        return Ok(());
    }
    let Some(key) = current.children.get(1) else {
        return Ok(());
    };
    let Some(selected) = registrations
        .iter()
        .filter(|registration| {
            registration.key == node_text(source, key)
                && registration.owner.start <= current.span.start
                && current.span.end <= registration.owner.end
        })
        .min_by_key(|registration| registration.owner.end - registration.owner.start)
    else {
        return Ok(());
    };
    if current.span.start < selected.declaration.start {
        return Err(failure(
            source,
            "T0252",
            format!(
                "interpolation renderer for key `{}` must be assigned before use",
                selected.key
            ),
            key.span,
        ));
    }
    let callee_at = Span::new(key.span.file, key.span.start, key.span.start);
    let value = current.children.remove(0);
    current.children.insert(
        0,
        call(
            current.span,
            name(callee_at, selected.name.clone()),
            vec![value],
        ),
    );
    current.interpolation_renderer = true;
    Ok(())
}

fn catch_shadow_span(
    source: &SourceFile,
    current: &SyntaxNode,
    registration: &Registration,
) -> Option<Span> {
    if current.span.end <= registration.owner.start || current.span.start >= registration.owner.end
    {
        return None;
    }
    if current.kind == SyntaxKind::CatchBinding && node_text(source, current) == registration.name {
        return Some(current.span);
    }
    current
        .children
        .iter()
        .find_map(|child| catch_shadow_span(source, child, registration))
}

fn reject_shadowing(
    unit: &SemanticUnit,
    registered: &[Registration],
) -> Result<(), SemanticFailure> {
    for registration in registered {
        for scope in &unit.scopes {
            if scope.span.start <= registration.owner.start
                || scope.span.end > registration.owner.end
            {
                continue;
            }
            if let Some(symbol) = scope
                .symbols
                .get(&registration.name)
                .and_then(|symbols| symbols.first())
            {
                return Err(failure(
                    &unit.source,
                    "T0250",
                    format!(
                        "registered interpolation renderer binding `{}` cannot be shadowed within its registration scope",
                        registration.name
                    ),
                    symbol.declaration_span.unwrap_or(scope.span),
                ));
            }
        }
        if let Some(alias) = catch_shadow_span(&unit.source, &unit.tree.root, registration) {
            return Err(failure(
                &unit.source,
                "T0250",
                format!(
                    "registered interpolation renderer binding `{}` cannot be shadowed within its registration scope",
                    registration.name
                ),
                alias,
            ));
        }
    }
    Ok(())
}

pub(super) fn normalize(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &mut package.units {
        let mut registered = Vec::new();
        registrations(
            &unit.source,
            &unit.tree.root,
            None,
            &package.declaration_modifiers,
            &mut registered,
        )?;
        reject_shadowing(unit, &registered)?;
        unit.mutations.normalized |= !registered.is_empty();
        rewrite_keys(&unit.source, &mut unit.tree.root, &registered)?;
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
    }
    Ok(())
}
