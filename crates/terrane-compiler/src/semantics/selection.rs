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
    if callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && let Ok(Some(ValueType::Object(identity))) =
            infer_value_type(unit, receiver, &unit.typed_bindings)
        && package
            .projection
            .method(
                &identity.namespace,
                &identity.name,
                node_text(&unit.source, member),
                false,
            )
            .is_some()
    {
        return SelectionOperationKind::Projected;
    }
    SelectionOperationKind::Task
}
fn receiver_root_name(mut receiver: &SyntaxNode) -> Option<&SyntaxNode> {
    while matches!(
        receiver.kind,
        SyntaxKind::GroupExpression | SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
    ) {
        receiver = receiver.children.first()?;
    }
    (receiver.kind == SyntaxKind::Name).then_some(receiver)
}

struct SelectedReceiverBorrow {
    key: (u32, usize, usize),
    mode: InvocationMode,
    use_span: Span,
    name: String,
    declaration_span: Span,
}

fn selected_receiver_borrow(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    mut operand: &SyntaxNode,
) -> Option<SelectedReceiverBorrow> {
    while operand.kind == SyntaxKind::GroupExpression
        && let [grouped] = operand.children.as_slice()
    {
        operand = grouped;
    }
    let [callee, _] = operand.children.as_slice() else {
        return None;
    };
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    if callee.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let receiver_type = infer_value_type(unit, receiver, &unit.typed_bindings)
        .ok()
        .flatten()?;
    let mode = super::diagnostics::member_invocation_mode(
        package,
        unit,
        &receiver_type,
        node_text(&unit.source, member),
    );
    let root = receiver_root_name(receiver)?;
    let name = node_text(&unit.source, root);
    let binding = unit.typed_bindings.iter().rev().find(|binding| {
        binding.name == name && binding.is_visible_at(unit.source.id(), root.span.start)
    })?;
    Some(SelectedReceiverBorrow {
        key: (binding.span.file, binding.span.start, binding.span.end),
        mode,
        use_span: operand.span,
        name: name.to_owned(),
        declaration_span: binding.span,
    })
}

fn find_node(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    (node.span == span).then_some(node).or_else(|| {
        node.children
            .iter()
            .find_map(|child| find_node(child, span))
    })
}

fn throwable_types_inner(
    unit: &SemanticUnit,
    mut operand: &SyntaxNode,
    visited_bindings: &mut BTreeSet<(u32, usize, usize)>,
) -> BTreeSet<String> {
    while operand.kind == SyntaxKind::GroupExpression
        && let [grouped] = operand.children.as_slice()
    {
        operand = grouped;
    }
    if operand.kind == SyntaxKind::CallExpression
        && let Some(callee) = operand.children.first()
        && let Ok(Some(ValueType::AsyncFunction(_, _, _, effects))) =
            infer_value_type(unit, callee, &unit.typed_bindings)
    {
        return effects.possible_throwables();
    }
    if operand.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, operand);
        if let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
            binding.name == name
                && binding.is_visible_at(unit.source.id(), operand.span.start)
                && matches!(binding.value_type, ValueType::Task(_, _))
        }) {
            let key = (binding.span.file, binding.span.start, binding.span.end);
            if visited_bindings.insert(key)
                && let Some(declaration) = find_node(&unit.tree.root, binding.span)
                && let Some(initializer) = declaration.children.last()
            {
                return throwable_types_inner(unit, initializer, visited_bindings);
            }
        }
    }
    BTreeSet::new()
}

fn throwable_types(unit: &SemanticUnit, operand: &SyntaxNode) -> BTreeSet<String> {
    throwable_types_inner(unit, operand, &mut BTreeSet::new())
}

#[expect(
    clippy::too_many_lines,
    reason = "selection validation builds one ordered semantic node while checking cross-case state"
)]
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
            let mut receiver_borrows: BTreeMap<(u32, usize, usize), (InvocationMode, Span)> =
                BTreeMap::new();
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
                if let Some(borrow) = selected_receiver_borrow(package, unit, operand) {
                    let SelectedReceiverBorrow {
                        key,
                        mode,
                        use_span,
                        name,
                        declaration_span,
                    } = borrow;
                    if let Some((prior_mode, prior_use)) = receiver_borrows.get(&key)
                        && (*prior_mode != InvocationMode::Shared || mode != InvocationMode::Shared)
                    {
                        return Err(SemanticFailure {
                            source: unit.source.clone(),
                            diagnostics: vec![
                                Diagnostic::error(
                                    "T0132",
                                    format!(
                                        "select cases cannot hold overlapping incompatible borrows of `{name}`"
                                    ),
                                    use_span,
                                )
                                .with_help(format!(
                                    "the earlier case borrows it at byte {}; `{name}` is declared at byte {}",
                                    prior_use.start, declaration_span.start
                                )),
                            ],
                        });
                    }
                    receiver_borrows.insert(key, (mode, use_span));
                }
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
