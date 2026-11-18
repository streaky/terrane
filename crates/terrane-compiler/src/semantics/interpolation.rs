//! Resolve registered renderer slots and validate explicit interpolation protocols.
use super::{
    diagnostics::{failure, node_text},
    expressions::infer_value_type,
    interpolatable,
    model::{FlowAvailability, SemanticFailure, SemanticPackage, SemanticUnit, ValueType},
};
use crate::{
    ScalarType,
    syntax::{SyntaxKind, SyntaxNode},
};

mod registrations;

pub(super) fn normalize_keys(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    registrations::normalize(package)
}

fn validate_node(unit: &SemanticUnit, node: &SyntaxNode) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::Interpolation {
        let value = &node.children[0];
        let actual = infer_value_type(unit, value, &unit.typed_bindings)?;
        if node.interpolation_renderer {
            if actual != Some(ValueType::Scalar(ScalarType::String)) {
                return Err(failure(
                    &unit.source,
                    "T0251",
                    "registered interpolation renderer must synchronously return string",
                    node.span,
                ));
            }
            let callee = &value.children[0];
            if unit
                .flow_availability
                .get(&(callee.span.file, callee.span.start, callee.span.end))
                == Some(&FlowAvailability::MayBeUnassigned)
            {
                return Err(failure(
                    &unit.source,
                    "T0252",
                    "an interpolation renderer must be assigned on every reachable path before use",
                    node.children[1].span,
                ));
            }
        } else {
            let actual = actual.ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0253",
                    "cannot infer interpolated value type",
                    value.span,
                )
            })?;
            if !interpolatable::conforms(unit, &actual, value.span.start) {
                return Err(failure(
                    &unit.source,
                    "T0253",
                    format!("`{actual}` does not implement `/core/display::interpolatable`"),
                    value.span,
                ));
            }
            if let Some(key) = node.children.get(1)
                && let Some(builtin) = builtin_arm(&actual)
            {
                let diagnostic = if matches!(actual, ValueType::Scalar(_)) {
                    format!(
                        "no interpolation renderer is registered for key `{}`, and `{actual}` defines no interpolation keys",
                        node_text(&unit.source, key)
                    )
                } else {
                    format!(
                        "no interpolation renderer is registered for key `{}`, and built-in arm `{builtin}` defines no interpolation keys",
                        node_text(&unit.source, key)
                    )
                };
                return Err(failure(&unit.source, "T0253", diagnostic, key.span));
            }
        }
    }
    for child in &node.children {
        validate_node(unit, child)?;
    }
    Ok(())
}

fn builtin_arm(value: &ValueType) -> Option<ScalarType> {
    match value {
        ValueType::Scalar(scalar) => Some(*scalar),
        ValueType::Optional(_) => Some(ScalarType::None),
        ValueType::Union(arms) => arms.iter().find_map(builtin_arm),
        ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner) => builtin_arm(inner.value_type_ref()),
        _ => None,
    }
}

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    registrations::validate(package)?;
    for unit in &package.units {
        validate_node(unit, &unit.tree.root)?;
    }
    Ok(())
}
