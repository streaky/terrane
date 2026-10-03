use super::prelude::*;
use crate::rust_interop::projection::{ProjectedParameter, ProjectedType};

pub(crate) fn macro_argument(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    value: &SyntaxNode,
    index: usize,
) -> Result<ProjectedParameter, SemanticFailure> {
    let mut value = value;
    while value.kind == SyntaxKind::GroupExpression && value.children.len() == 1 {
        value = &value.children[0];
    }
    if value.kind == SyntaxKind::CallExpression
        && value
            .children
            .first()
            .is_some_and(|callee| projected_macro_for_call(package, unit, callee).is_some())
    {
        return Ok(parameter(
            index,
            ProjectedType::Generic("__MacroTokens".to_owned()),
            false,
            false,
        ));
    }
    let actual = infer_value_type(unit, value, &unit.typed_bindings)?.ok_or_else(|| {
        failure(
            &unit.source,
            "T0119",
            "native macro argument requires a concrete projected type",
            value.span,
        )
    })?;
    let (actual, borrowed, mutable) = match actual {
        ValueType::Reference(item) => (item.value_type(), true, true),
        ValueType::SharedReference(item) => (item.value_type(), true, false),
        actual => (actual, false, false),
    };
    let ty = if value.kind == SyntaxKind::Literal && actual == ValueType::Scalar(ScalarType::String)
    {
        ProjectedType::BorrowedString
    } else {
        super::objects::destination_projected_type(package, &actual).map_err(|error| {
            failure(
                &unit.source,
                "T0119",
                format!("native macro argument cannot be projected: {error}"),
                value.span,
            )
        })?
    };
    if borrowed && !ty.has_identity_representation() {
        return Err(failure(
            &unit.source,
            "T0119",
            "native macro reference argument requires an identity-preserving native loan",
            value.span,
        ));
    }
    Ok(parameter(index, ty, borrowed, mutable))
}

fn parameter(
    index: usize,
    ty: ProjectedType,
    borrowed: bool,
    mutable_borrow: bool,
) -> ProjectedParameter {
    ProjectedParameter {
        name: format!("argument_{index}"),
        ty,
        borrowed,
        mutable_borrow,
        generic_parameter: None,
        generic_bounds: Vec::new(),
        generic_interface: None,
        associated_type: None,
    }
}

pub(super) fn macro_probe(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    result: &ProjectedType,
) -> Result<crate::rust_interop::CallQuestion, SemanticFailure> {
    fn expression(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        value: &SyntaxNode,
        parameters: &mut Vec<String>,
    ) -> Result<String, SemanticFailure> {
        if value.kind == SyntaxKind::GroupExpression
            && let [inner] = value.children.as_slice()
        {
            return expression(package, unit, inner, parameters);
        }
        if value.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = value.children.as_slice()
            && let Some(item) = projected_macro_for_call(package, unit, callee)
        {
            let arguments = arguments
                .children
                .iter()
                .map(|argument| {
                    expression(
                        package,
                        unit,
                        argument.children.last().unwrap_or(argument),
                        parameters,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(format!("{}!({})", item.rust_path, arguments.join(", ")));
        }
        let parameter = macro_argument(package, unit, value, parameters.len())?;
        if value.kind == SyntaxKind::Literal {
            let token =
                crate::lowering::helpers::native_macro_literal(node_text(&unit.source, value));
            return Ok(match parameter.ty {
                ProjectedType::Int => format!("{token}_i64"),
                ProjectedType::Float => format!("{token}_f64"),
                _ => token,
            });
        }
        let name = format!("argument_{}", parameters.len());
        parameters.push(format!(
            "{}{}: {}",
            if parameter.mutable_borrow { "mut " } else { "" },
            name,
            parameter.ty.rust_type()
        ));
        Ok(if parameter.borrowed {
            format!(
                "&{}{}",
                if parameter.mutable_borrow { "mut " } else { "" },
                name
            )
        } else {
            name
        })
    }
    let mut parameters = Vec::new();
    let invocation = expression(package, unit, node, &mut parameters)?;
    Ok(crate::rust_interop::CallQuestion {
        label: format!("native macro at {}:{}", unit.source_path, node.span.start),
        source: format!(
            "#[allow(dead_code, unused_variables, unused_mut)] fn __terrane_macro_probe({}) -> {} {{ core::convert::Into::into({}) }}\nfn main() {{}}\n",
            parameters.join(", "),
            result.rust_type(),
            invocation
        ),
    })
}
