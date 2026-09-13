use super::prelude::*;

fn await_expression(case: &SyntaxNode) -> Option<(&SyntaxNode, Option<Span>, &SyntaxNode)> {
    let [header, body] = case.children.as_slice() else {
        return None;
    };
    let (awaited, binding) = if header.kind == SyntaxKind::Binding {
        (header.children.last()?, Some(header.span))
    } else {
        (header, None)
    };
    let operand = awaited.children.last()?;
    Some((operand, binding, body))
}

fn operation_kind(unit: &SemanticUnit, operand: &SyntaxNode) -> SelectionOperationKind {
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
        && unit
            .function_contracts_by_span
            .values()
            .any(|contract| contract.span == callee.span && contract.span.file != unit.source.id())
    {
        return SelectionOperationKind::Projected;
    }
    SelectionOperationKind::Task
}

fn throwable_types(unit: &SemanticUnit, operand: &SyntaxNode) -> BTreeSet<String> {
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
                let Some((operand, binding, body)) = await_expression(case) else {
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
                    await_span: operand.span,
                    binding,
                    body: body.span,
                    result_type: result.value_type(),
                    transferability,
                    throwable_types: throwable_types(unit, operand),
                    operation: operation_kind(unit, operand),
                });
            }
            selections.push(SemanticSelection {
                span: node.span,
                cases,
            });
        }
        for child in &node.children {
            collect(unit, child, selections)?;
        }
        Ok(())
    }

    for unit in &mut package.units {
        let mut selections = Vec::new();
        collect(unit, &unit.tree.root, &mut selections)?;
        unit.selections = selections;
    }
    Ok(())
}
