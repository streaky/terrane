use super::prelude::*;
use crate::rust_interop::projection::{ProjectedParameter, ProjectedType};

pub(crate) fn macro_argument(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    value: &SyntaxNode,
    index: usize,
    joint_native_graph: bool,
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
    if joint_native_graph
        && value.kind == SyntaxKind::CallExpression
        && let Some(callee) = value.children.first()
        && let Some(function) =
            projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(value))
        && (matches!(function.result, ProjectedType::InvocationScoped { .. })
            || matches!(
                function.chain_role,
                Some(
                    crate::rust_interop::projection::ChainRole::Root
                        | crate::rust_interop::projection::ChainRole::Continue
                )
            ))
    {
        // The enclosing callback proves this complete producer/macro expression.
        // Do not demand a detached concrete type for an intermediate whose
        // lifetime and generics are selected jointly by that native graph.
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
        ValueType::Reference(item) | ValueType::SharedReference(item) => {
            (item.value_type(), true, false)
        }
        ValueType::MutableReference(item) => (item.value_type(), true, true),
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
        borrowed_str: false,
        name: format!("argument_{index}"),
        ty,
        borrowed,
        borrow_lifetime: None,
        mutable_borrow,
        generic_parameter: None,
        generic_bounds: Vec::new(),
        generic_interface: None,
        associated_type: None,
    }
}

pub(super) fn input_conversion_errors(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    errors: &mut std::collections::BTreeSet<String>,
) {
    let [callee, arguments] = node.children.as_slice() else {
        return;
    };
    if node.kind != SyntaxKind::CallExpression
        || projected_macro_for_call(package, unit, callee).is_none()
    {
        return;
    }
    let joint_native_graph = unit
        .projected_call_specializations
        .get(&(node.span.file, node.span.start, node.span.end))
        .is_some_and(|selection| {
            matches!(
                selection.value_type,
                ValueType::InvocationScopedNative { concrete: true, .. }
            )
        })
        || unit.invocation_scoped_result_at(node.span).is_some();
    for (index, argument) in arguments.children.iter().enumerate() {
        let value = argument.children.last().unwrap_or(argument);
        let Ok(parameter) = macro_argument(package, unit, value, index, joint_native_graph) else {
            continue;
        };
        let actual = infer_value_type(unit, value, &unit.typed_bindings)
            .ok()
            .flatten();
        super::native_constructors::native_value_conversion_errors(
            unit,
            value,
            &parameter.ty,
            actual.as_ref(),
            errors,
        );
    }
}

pub(super) fn macro_probe(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    result: &ProjectedType,
) -> Result<
    (
        crate::rust_interop::CallQuestion,
        crate::rust_interop::CallQuestion,
    ),
    SemanticFailure,
> {
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
        let parameter = macro_argument(package, unit, value, parameters.len(), false)?;
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
    let parameters = parameters.join(", ");
    let expansion_source = format!(
        "#[allow(dead_code, unused_variables, unused_mut)] fn __terrane_macro_probe({parameters}) {{ let _ = {invocation}; }}\nfn main() {{}}\n"
    );
    let result_source = format!(
        "#[allow(dead_code, unused_variables, unused_mut)] fn __terrane_macro_probe({parameters}) -> {} {{ core::convert::Into::into({invocation}) }}\nfn main() {{}}\n",
        result.rust_type()
    );
    let label = format!("native macro at {}:{}", unit.source_path, node.span.start);
    Ok((
        crate::rust_interop::CallQuestion {
            label: format!("{label}: expansion"),
            source: expansion_source,
        },
        crate::rust_interop::CallQuestion {
            label: format!("{label}: result"),
            source: result_source,
        },
    ))
}
