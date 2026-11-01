//! Compile-time modifier contracts. Syntax, lexical identity, schema arguments and
//! operation selection are shared; each supported operation owns its semantic plan.
use super::prelude::*;

const DEFINITION: &str = "/core/modifiers::modifier";

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub(crate) enum ModifierOperation {
    ScopedAccess,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub(crate) struct ResolvedModifier {
    pub identity: String,
    pub span: MetadataSpan,
    pub operation: ModifierOperation,
    pub payload: CompileTimeValue,
}

fn error(unit: &SemanticUnit, span: Span, message: impl Into<String>) -> SemanticFailure {
    failure(&unit.source, "S2120", message, span)
}

fn field<'a>(value: &'a CompileTimeValue, name: &str) -> Option<&'a CompileTimeValue> {
    let CompileTimeValue::Map(fields) = value else {
        return None;
    };
    fields.iter().find_map(|(key, value)| {
        matches!(key, CompileTimeValue::String(key) if key == name).then_some(value)
    })
}

fn declaration_target(node: &SyntaxNode) -> Option<&'static str> {
    match node.kind {
        SyntaxKind::Binding | SyntaxKind::Assignment | SyntaxKind::WithRegion => Some("binding"),
        SyntaxKind::ClassDeclaration => Some("class"),
        SyntaxKind::InterfaceDeclaration => Some("interface"),
        SyntaxKind::TraitDeclaration => Some("trait"),
        SyntaxKind::EnumDeclaration => Some("enum"),
        SyntaxKind::FunctionDeclaration => Some("callable"),
        _ => None,
    }
}

fn resolve_application(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration: &SyntaxNode,
    application: &SyntaxNode,
    seen: &mut BTreeSet<String>,
    reads: &mut BTreeMap<(u32, usize, usize), Span>,
) -> Result<ResolvedModifier, SemanticFailure> {
    let name = application
        .children
        .first()
        .ok_or_else(|| error(unit, application.span, "malformed modifier application"))?;
    let symbol = package
        .resolve_name_at(unit, name.span.start, node_text(&unit.source, name))
        .ok_or_else(|| error(unit, name.span, "unknown declaration modifier"))?;
    if symbol.kind != SymbolKind::Class {
        return Err(error(
            unit,
            name.span,
            "a modifier must resolve to a concrete modifier schema class",
        ));
    }
    let definition = package
        .declarations()
        .find(|record| record.identity == symbol.identity)
        .and_then(|record| {
            record
                .annotations
                .iter()
                .find(|annotation| annotation.identity == DEFINITION)
        })
        .ok_or_else(|| {
            error(
                unit,
                name.span,
                "modifier class requires the /core/modifiers::modifier definition contract",
            )
        })?;
    let repeatable = matches!(
        field(&definition.payload, "repeatable"),
        Some(CompileTimeValue::Boolean(true))
    );
    if !seen.insert(symbol.identity.clone()) && !repeatable {
        return Err(error(
            unit,
            application.span,
            "declaration modifier is not repeatable",
        ));
    }
    let target = declaration_target(declaration).ok_or_else(|| {
        error(
            unit,
            declaration.span,
            "modifier requires a supported declaration target",
        )
    })?;
    if !matches!(field(&definition.payload, "targets"), Some(CompileTimeValue::List(targets))
        if targets.iter().any(|value| matches!(value, CompileTimeValue::String(value) if value == target)))
    {
        return Err(error(
            unit,
            application.span,
            format!(
                "modifier `{}` does not admit target `{target}`",
                symbol.identity
            ),
        ));
    }
    // Bind arguments without running a constructor, arbitrary package code or effects.
    // Unsupported operations still receive full schema validation before declining.
    let payload = super::annotations::application_payload(
        package,
        unit,
        application,
        &symbol.identity,
        reads,
    )?;
    let Some(CompileTimeValue::String(operation)) = field(&definition.payload, "operation") else {
        return Err(error(
            unit,
            name.span,
            "modifier definition requires an operation identity",
        ));
    };
    let structural = matches!(
        field(&definition.payload, "structural"),
        Some(CompileTimeValue::Boolean(true))
    );
    let operation = match operation.as_str() {
        "scoped-access" if structural => ModifierOperation::ScopedAccess,
        "scoped-access" => {
            return Err(error(
                unit,
                application.span,
                "scoped-access requires the compiler-defined structural contract",
            ));
        }
        _ => {
            return Err(error(
                unit,
                application.span,
                format!(
                    "modifier operation `{operation}` has no implemented lowering; no realization property is applied"
                ),
            ));
        }
    };
    match operation {
        ModifierOperation::ScopedAccess => {
            if declaration.kind != SyntaxKind::WithRegion {
                return Err(error(
                    unit,
                    declaration.span,
                    "scoped-access requires an explicitly delimited region and acquired local binding",
                ));
            }
            if !matches!(&payload, CompileTimeValue::Map(fields) if fields.is_empty()) {
                return Err(error(
                    unit,
                    application.span,
                    "the scoped-access operation does not accept configuration fields",
                ));
            }
        }
    }
    Ok(ResolvedModifier {
        identity: symbol.identity.clone(),
        span: MetadataSpan {
            file: application.span.file,
            path: unit.source_path.clone(),
            start: application.span.start,
            end: application.span.end,
        },
        operation,
        payload,
    })
}

fn collect(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    resolved: &mut BTreeMap<Span, Vec<ResolvedModifier>>,
    reads: &mut BTreeMap<(u32, usize, usize), Span>,
) -> Result<(), SemanticFailure> {
    if !node.modifier_applications.is_empty() {
        let mut applications = Vec::with_capacity(node.modifier_applications.len());
        let mut seen = BTreeSet::new();
        for application in &node.modifier_applications {
            let selected = resolve_application(package, unit, node, application, &mut seen, reads)?;
            if applications
                .iter()
                .any(|prior: &ResolvedModifier| prior.operation == selected.operation)
            {
                return Err(error(
                    unit,
                    application.span,
                    "conflicting structural modifier operations on the same declaration",
                ));
            }
            applications.push(selected);
        }
        resolved.insert(node.span, applications);
    } else if node.kind == SyntaxKind::WithRegion {
        return Err(error(
            unit,
            node.span,
            "region has no selected modifier contract",
        ));
    }
    for child in &node.children {
        collect(package, unit, child, resolved, reads)?;
    }
    Ok(())
}

pub(super) fn resolve_declaration_modifiers(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    let mut resolved = BTreeMap::new();
    let mut reads = BTreeMap::new();
    for unit in &package.units {
        collect(package, unit, &unit.tree.root, &mut resolved, &mut reads)?;
    }
    package.declaration_modifiers = resolved;
    package.metadata_constant_reads.extend(reads);
    Ok(())
}
