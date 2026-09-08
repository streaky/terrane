use super::prelude::*;
fn logging_object(unit: &SemanticUnit, name: &str) -> ObjectIdentity {
    unit.descriptors
        .iter()
        .find(|object| object.name == name)
        .map_or_else(
            || ObjectIdentity::new("/core/logging", name),
            |object| object.identity.clone(),
        )
}

pub(super) fn infer_structured_log_emit(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let [callee, arguments] = node.children.as_slice() else {
        return Ok(None);
    };
    if resolved_compiler_identity(unit, callee) != Some("/core/logging::emit") {
        return Ok(None);
    }
    let values = arguments
        .children
        .iter()
        .map(|argument| argument.children.last().unwrap_or(argument))
        .collect::<Vec<_>>();
    let [logger, level, message, fields] = values.as_slice() else {
        return Err(failure(
            &unit.source,
            "T0116",
            "structured log emission requires logger, level, message, and fields",
            node.span,
        ));
    };
    let expected = [
        ValueType::Object(logging_object(unit, "logger")),
        ValueType::Object(logging_object(unit, "log-level")),
        ValueType::Scalar(ScalarType::String),
        ValueType::List(ElementType::new(ValueType::Object(logging_object(
            unit,
            "log-field",
        )))),
    ];
    for (argument, expected) in [logger, level, message, fields].into_iter().zip(expected) {
        let actual = infer_value_type(unit, argument, bindings)?;
        if actual != Some(expected.clone()) {
            return Err(failure(
                &unit.source,
                "T0116",
                format!(
                    "structured log argument must have type `{expected}`, found `{}`",
                    actual
                        .as_ref()
                        .map_or("unknown".to_owned(), ToString::to_string)
                ),
                argument.span,
            ));
        }
    }
    Ok(Some(ValueType::Object(logging_object(unit, "log-outcome"))))
}
