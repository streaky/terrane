use super::prelude::*;

fn await_expression(case: &SyntaxNode) -> Option<(&SyntaxNode, Span, Option<Span>, &SyntaxNode)> {
    let [header, body] = case.children.as_slice() else {
        return None;
    };
    let (awaited, binding) = if header.kind == SyntaxKind::Binding {
        (header.children.last()?, Some(header.span))
    } else {
        (header, None)
    };
    let operand = awaited.children.last()?;
    Some((operand, awaited.span, binding, body))
}

fn operation_kind(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    mut operand: &SyntaxNode,
) -> SelectionOperationKind {
    while operand.kind == SyntaxKind::GroupExpression
        && let [grouped] = operand.children.as_slice()
    {
        operand = grouped;
    }
    let Some(callee) = (operand.kind == SyntaxKind::CallExpression)
        .then(|| operand.children.first())
        .flatten()
    else {
        return SelectionOperationKind::Task;
    };
    if callee.kind == SyntaxKind::MemberExpression
        && let Some(receiver) = callee.children.first()
        && matches!(
            infer_receiver_value_type(unit, receiver, &unit.typed_bindings),
            Ok(Some(
                ValueType::ChannelSender(_) | ValueType::ChannelReceiver(_)
            ))
        )
    {
        return SelectionOperationKind::Channel;
    }
    if callee.kind == SyntaxKind::Name
        && package
            .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
            .and_then(|symbol| symbol.identity.rsplit_once("::"))
            .and_then(|(namespace, name)| package.projection.item(namespace, name))
            .is_some_and(|item| matches!(&item.kind, crate::projection::ProjectedKind::Function(_)))
    {
        return SelectionOperationKind::Projected;
    }
    SelectionOperationKind::Task
}
fn throwable_types(unit: &SemanticUnit, mut operand: &SyntaxNode) -> BTreeSet<String> {
    while operand.kind == SyntaxKind::GroupExpression
        && let [grouped] = operand.children.as_slice()
    {
        operand = grouped;
    }
    let Some(callee) = (operand.kind == SyntaxKind::CallExpression)
        .then(|| operand.children.first())
        .flatten()
    else {
        return BTreeSet::new();
    };
    if let Ok(Some(ValueType::AsyncFunction(_, _, _, effects))) =
        infer_value_type(unit, callee, &unit.typed_bindings)
    {
        return effects.possible_throwables();
    }
    BTreeSet::new()
}

pub(super) fn analyze_selections(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    fn collect(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        selections: &mut Vec<SemanticSelection>,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::SelectStatement {
            let async_function = unit.functions.iter().any(|function| {
                function.is_async
                    && function.span.start <= node.span.start
                    && node.span.end <= function.span.end
            });
            if !async_function {
                return Err(failure(
                    &unit.source,
                    "T0130",
                    "`select` is valid only inside an async callable",
                    node.span,
                ));
            }
            let mut cases = Vec::with_capacity(node.children.len());
            for case in &node.children {
                let Some((operand, await_span, binding, body)) = await_expression(case) else {
                    continue;
                };
                let Some(result_type) = infer_value_type(unit, operand, &unit.typed_bindings)?
                else {
                    return Err(failure(
                        &unit.source,
                        "T0131",
                        "select case expression must produce a task",
                        operand.span,
                    ));
                };
                let ValueType::Task(result, transferability) = result_type else {
                    return Err(failure(
                        &unit.source,
                        "T0131",
                        format!(
                            "select case expression must produce a task, found `{result_type}`"
                        ),
                        operand.span,
                    ));
                };
                cases.push(SemanticSelectionCase {
                    span: case.span,
                    await_span,
                    binding,
                    body: body.span,
                    result_type: result.value_type(),
                    transferability,
                    throwable_types: throwable_types(unit, operand),
                    operation: operation_kind(package, unit, operand),
                });
            }
            selections.push(SemanticSelection {
                span: node.span,
                cases,
            });
        }
        for child in &node.children {
            collect(package, unit, child, selections)?;
        }
        Ok(())
    }

    for index in 0..package.units.len() {
        let mut selections = Vec::new();
        {
            let unit = &package.units[index];
            collect(package, unit, &unit.tree.root, &mut selections)?;
        }
        package.units[index].selections = selections;
    }
    Ok(())
}
