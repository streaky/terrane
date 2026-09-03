use super::prelude::*;

pub(super) fn validate_constant_reassignment(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn visit_declarations(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && node.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(&unit.source, child) == "constant"
            })
            && let Some(target) = first_write_to(package, unit, node.span, &unit.tree.root)
        {
            let name = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .map_or("constant", |child| node_text(&unit.source, child));
            return Err(failure(
                &unit.source,
                "S2022",
                format!("constant binding `{name}` cannot be reassigned"),
                target.span,
            ));
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && node.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(&unit.source, child) == "global"
            })
            && let Some(target) = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            && let Some(symbol) =
                package.resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            && symbol
                .declaration_span
                .is_some_and(|span| declaration_is_constant(package, span))
        {
            return Err(failure(
                &unit.source,
                "S2022",
                format!(
                    "constant binding `{}` cannot be reassigned",
                    node_text(&unit.source, target)
                ),
                target.span,
            ));
        }
        for child in &node.children {
            visit_declarations(package, unit, child)?;
        }
        Ok(())
    }

    for unit in &package.units {
        visit_declarations(package, unit, &unit.tree.root)?;
    }
    Ok(())
}

pub(super) fn declaration_is_constant_in_unit(unit: &SemanticUnit, span: Span) -> bool {
    fn find(node: &SyntaxNode, span: Span, source: &SourceFile) -> Option<bool> {
        if node.span == span {
            return Some(node.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(source, child) == "constant"
            }));
        }
        node.children
            .iter()
            .find_map(|child| find(child, span, source))
    }

    span.file == unit.source.id() && find(&unit.tree.root, span, &unit.source).unwrap_or(false)
}

pub(super) fn declaration_is_constant(package: &SemanticPackage, span: Span) -> bool {
    package
        .units
        .iter()
        .find(|unit| unit.source.id() == span.file)
        .is_some_and(|unit| declaration_is_constant_in_unit(unit, span))
}

#[expect(
    clippy::too_many_lines,
    reason = "the global assignment transfer rules remain visible as one analysis"
)]
pub(super) fn validate_global_definite_assignment(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn has_qualifier(unit: &SemanticUnit, node: &SyntaxNode, qualifier: &str) -> bool {
        node.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == qualifier
        })
    }

    fn has_initializer(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
        unit.source.text()[node.span.start..node.span.end].contains('=')
    }

    fn global_name<'a>(unit: &'a SemanticUnit, node: &'a SyntaxNode) -> Option<&'a str> {
        node.children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .map(|child| node_text(&unit.source, child))
    }

    fn collect_writes(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        writes: &mut BTreeSet<String>,
    ) {
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && has_qualifier(unit, node, "global")
            && has_initializer(unit, node)
            && let Some(name) = global_name(unit, node)
        {
            writes.insert(name.to_owned());
        } else if node.kind == SyntaxKind::PostfixExpression
            && let Some(target) = node.children.first()
            && package
                .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
                .is_some_and(|symbol| symbol.global)
        {
            writes.insert(node_text(&unit.source, target).to_owned());
        }
        for child in &node.children {
            collect_writes(package, unit, child, writes);
        }
    }

    fn validate_node(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        relevant: &BTreeSet<String>,
        assigned: &mut BTreeSet<String>,
    ) -> Result<(), SemanticFailure> {
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
            && has_qualifier(unit, node, "global")
        {
            let name_node = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name);
            for child in &node.children {
                if Some(child.span) != name_node.map(|name| name.span) {
                    validate_node(package, unit, child, relevant, assigned)?;
                }
            }
            if has_initializer(unit, node)
                && let Some(name) = name_node.map(|name| node_text(&unit.source, name))
            {
                assigned.insert(name.to_owned());
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::PostfixExpression
            && let Some(target) = node.children.first()
            && let Some(symbol) =
                package.resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            && symbol.global
        {
            let name = node_text(&unit.source, target);
            if relevant.contains(name) && !assigned.contains(name) {
                return Err(failure(
                    &unit.source,
                    "T0007",
                    format!("`{name}` may be read before it is assigned"),
                    target.span,
                ));
            }
            assigned.insert(name.to_owned());
            return Ok(());
        }
        if node.kind == SyntaxKind::Name
            && let Some(symbol) =
                package.resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
            && symbol.global
        {
            let name = node_text(&unit.source, node);
            if relevant.contains(name) && !assigned.contains(name) {
                return Err(failure(
                    &unit.source,
                    "T0007",
                    format!("`{name}` may be read before it is assigned"),
                    node.span,
                ));
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::IfStatement {
            if let Some(condition) = node.children.first() {
                validate_node(package, unit, condition, relevant, assigned)?;
            }
            let incoming = assigned.clone();
            let mut branch_results = Vec::new();
            for branch in node.children.iter().skip(1) {
                let branch_block = if branch.kind == SyntaxKind::Block {
                    Some(branch)
                } else {
                    branch
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block)
                };
                if let Some(branch_block) = branch_block {
                    let mut branch_assigned = incoming.clone();
                    validate_node(package, unit, branch_block, relevant, &mut branch_assigned)?;
                    branch_results.push(branch_assigned);
                }
            }
            if !node
                .children
                .iter()
                .any(|child| child.kind == SyntaxKind::ElseClause)
            {
                branch_results.push(incoming);
            }
            if let Some(first) = branch_results.first() {
                *assigned = branch_results
                    .iter()
                    .skip(1)
                    .fold(first.clone(), |common, branch| {
                        common.intersection(branch).cloned().collect()
                    });
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::WhileStatement {
            let before = assigned.clone();
            for child in &node.children {
                let mut branch = before.clone();
                validate_node(package, unit, child, relevant, &mut branch)?;
            }
            return Ok(());
        }
        for child in &node.children {
            validate_node(package, unit, child, relevant, assigned)?;
        }
        Ok(())
    }

    let mut uninitialized = package
        .globals
        .values()
        .filter(|symbol| symbol.kind == SymbolKind::Binding)
        .map(|symbol| symbol.name.clone())
        .collect::<BTreeSet<_>>();
    for unit in &package.units {
        for node in &unit.tree.root.children {
            if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
                && has_qualifier(unit, node, "global")
                && has_initializer(unit, node)
                && let Some(name) = global_name(unit, node)
            {
                uninitialized.remove(name);
            }
        }
    }
    if uninitialized.is_empty() {
        return Ok(());
    }

    let mut writes = BTreeSet::new();
    for unit in &package.units {
        collect_writes(package, unit, &unit.tree.root, &mut writes);
    }
    for unit in &package.units {
        for function in unit
            .tree
            .root
            .children
            .iter()
            .filter(|node| node.kind == SyntaxKind::FunctionDeclaration)
        {
            let mut function_writes = BTreeSet::new();
            collect_writes(package, unit, function, &mut function_writes);
            let relevant = uninitialized
                .iter()
                .filter(|name| function_writes.contains(*name) || !writes.contains(*name))
                .cloned()
                .collect();
            validate_node(package, unit, function, &relevant, &mut BTreeSet::new())?;
        }
    }
    Ok(())
}

pub(super) fn first_write_to<'a>(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
    node: &'a SyntaxNode,
) -> Option<&'a SyntaxNode> {
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::Name
        && let Some(symbol) =
            package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
        && let Some(crate::projection::ProjectedKind::Function(function)) = package
            .projection
            .item(&symbol.namespace, &symbol.name)
            .map(|item| &item.kind)
    {
        let mut positional = 0;
        for argument in &arguments.children {
            let named = argument
                .children
                .first()
                .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
            let index = named.map_or_else(
                || {
                    let index = positional;
                    positional += 1;
                    index
                },
                |name| {
                    function
                        .parameters
                        .iter()
                        .position(|parameter| parameter.name == node_text(&unit.source, name))
                        .unwrap_or(usize::MAX)
                },
            );
            let value = argument.children.last().unwrap_or(argument);
            if function
                .parameters
                .get(index)
                .is_some_and(|parameter| parameter.mutable_borrow)
                && value.kind == SyntaxKind::Name
                && package
                    .resolve_name_at(unit, value.span.start, node_text(&unit.source, value))
                    .is_some_and(|symbol| symbol.declaration_span == Some(declaration_span))
            {
                return Some(value);
            }
        }
    }
    if matches!(
        node.kind,
        SyntaxKind::Assignment | SyntaxKind::PostfixExpression
    ) && node.span != declaration_span
        && let Some(target) = node.children.first()
        && target.kind == SyntaxKind::Name
        && package
            .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            .is_some_and(|symbol| symbol.declaration_span == Some(declaration_span))
    {
        return Some(target);
    }
    node.children
        .iter()
        .find_map(|child| first_write_to(package, unit, declaration_span, child))
}

pub(super) fn record_binding_mutability(package: &mut SemanticPackage) {
    let mutable_bindings = package
        .units
        .iter()
        .map(|unit| {
            unit.typed_bindings
                .iter()
                .map(|binding| {
                    let initially_assigned =
                        unit.source.text()[binding.span.start..binding.span.end].contains('=');
                    binding_span_is_mutated(package, unit, binding.span, initially_assigned)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mutable_parameters = package
        .units
        .iter()
        .map(|unit| {
            unit.functions
                .iter()
                .map(|function| {
                    function
                        .parameters
                        .iter()
                        .map(|parameter| {
                            binding_span_is_mutated(package, unit, parameter.span, true)
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for ((unit, binding_mutability), parameter_mutability) in package
        .units
        .iter_mut()
        .zip(mutable_bindings)
        .zip(mutable_parameters)
    {
        for (binding, mutable) in unit.typed_bindings.iter_mut().zip(binding_mutability) {
            binding.mutable = mutable;
        }
        for (function, mutability) in unit.functions.iter_mut().zip(parameter_mutability) {
            for (parameter, mutable) in function.parameters.iter_mut().zip(mutability) {
                parameter.mutable = mutable;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ControlRegion {
    pub(super) statement: Span,
    pub(super) arm: Option<usize>,
}

#[derive(Clone, Debug)]
pub(super) enum BindingEvent {
    Read {
        span: Span,
        loops: Vec<Span>,
        regions: Vec<ControlRegion>,
    },
    Write {
        span: Span,
        loops: Vec<Span>,
        regions: Vec<ControlRegion>,
    },
}

pub(super) fn span_key(span: Span) -> (u32, usize, usize) {
    (span.file, span.start, span.end)
}

pub(super) fn binding_event_child_repeats(node: &SyntaxNode, index: usize) -> bool {
    if node.kind == SyntaxKind::ForStatement
        && node
            .children
            .get(index)
            .is_some_and(|child| child.kind == SyntaxKind::ForTarget)
    {
        return true;
    }
    match node.kind {
        SyntaxKind::WhileStatement => true,
        SyntaxKind::ForStatement if node.children.len() == 3 => index == 2,
        SyntaxKind::ForStatement if node.children.len() == 4 => index != 0,
        _ => false,
    }
}

pub(super) fn binding_event_child_region(
    node: &SyntaxNode,
    child: &SyntaxNode,
    index: usize,
) -> Option<ControlRegion> {
    if node.kind == SyntaxKind::ForStatement && child.kind == SyntaxKind::ForTarget {
        return Some(ControlRegion {
            statement: node.span,
            arm: None,
        });
    }
    if node.kind == SyntaxKind::IfStatement
        && matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause)
    {
        return Some(ControlRegion {
            statement: node.span,
            arm: Some(index),
        });
    }
    if child.kind != SyntaxKind::Block {
        return None;
    }
    let statement = match node.kind {
        SyntaxKind::WhileStatement | SyntaxKind::ForStatement => node.span,
        SyntaxKind::TryStatement | SyntaxKind::CatchClause | SyntaxKind::FinallyClause => {
            child.span
        }
        _ => return None,
    };
    Some(ControlRegion {
        statement,
        arm: None,
    })
}

pub(super) fn node_may_declare_typed_binding(node: &SyntaxNode) -> bool {
    matches!(
        node.kind,
        SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::Parameter
            | SyntaxKind::ForTarget
    )
}

pub(super) fn declared_bindings_at_node<'a>(
    unit: &'a SemanticUnit,
    node: &SyntaxNode,
) -> impl Iterator<Item = &'a TypedBinding> {
    unit.typed_bindings.iter().filter(move |binding| {
        if node.kind == SyntaxKind::ForTarget {
            node.children.iter().any(|name| binding.span == name.span)
        } else {
            binding.span == node.span
        }
    })
}

pub(super) fn initial_store_span(node: &SyntaxNode, binding: &TypedBinding) -> Span {
    if node.kind == SyntaxKind::ForTarget {
        binding.span
    } else {
        node.span
    }
}

pub(super) fn record_declared_binding_writes(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    declares_binding: bool,
    events: &mut BTreeMap<(u32, usize, usize), Vec<BindingEvent>>,
    loops: &[Span],
    regions: &[ControlRegion],
) -> bool {
    if !declares_binding {
        return false;
    }
    let initial_store = node.kind == SyntaxKind::ForTarget
        || node.kind == SyntaxKind::Parameter
        || unit.source.text()[node.span.start..node.span.end].contains('=');
    if !initial_store {
        return false;
    }
    let mut recorded = false;
    for binding in declared_bindings_at_node(unit, node) {
        recorded = true;
        events
            .entry(span_key(binding.span))
            .or_default()
            .push(BindingEvent::Write {
                span: initial_store_span(node, binding),
                loops: loops.to_vec(),
                regions: regions.to_vec(),
            });
    }
    recorded
}

pub(super) fn collect_binding_events(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    events: &mut BTreeMap<(u32, usize, usize), Vec<BindingEvent>>,
    declaration_name: bool,
    loops: &mut Vec<Span>,
    regions: &mut Vec<ControlRegion>,
) {
    if node.kind == SyntaxKind::Name {
        let function_span = unit
            .enclosing_function_spans
            .get(&node.span.start)
            .copied()
            .flatten();
        let typed_declaration = unit.typed_bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, node)
                && binding.is_visible_at(unit.source.id(), node.span.start)
                && unit
                    .enclosing_function_spans
                    .get(&binding.span.start)
                    .copied()
                    .flatten()
                    == function_span
        });
        let declaration_span = typed_declaration.map(|binding| binding.span).or_else(|| {
            package
                .resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
                .and_then(|symbol| symbol.declaration_span)
        });
        if !declaration_name && let Some(declaration_span) = declaration_span {
            events
                .entry(span_key(declaration_span))
                .or_default()
                .push(BindingEvent::Read {
                    span: node.span,
                    loops: loops.clone(),
                    regions: regions.clone(),
                });
        }
        return;
    }

    let declares_binding = node_may_declare_typed_binding(node)
        && declared_bindings_at_node(unit, node).next().is_some();
    let assignment_target = if matches!(
        node.kind,
        SyntaxKind::Assignment | SyntaxKind::PostfixExpression
    ) && !declares_binding
    {
        node.children
            .first()
            .filter(|target| target.kind == SyntaxKind::Name)
    } else {
        None
    };

    for (index, child) in node.children.iter().enumerate() {
        let declares_child = child.kind == SyntaxKind::Name
            && if node.kind == SyntaxKind::ForTarget {
                true
            } else {
                (declares_binding || node.kind == SyntaxKind::Parameter)
                    && !node.children[..index]
                        .iter()
                        .any(|prior| prior.kind == SyntaxKind::Name)
            };
        let plain_assignment_target =
            assignment_target.is_some() && node.kind == SyntaxKind::Assignment && index == 0;
        if !plain_assignment_target {
            let repeats = binding_event_child_repeats(node, index);
            let region = binding_event_child_region(node, child, index);
            if repeats {
                loops.push(node.span);
            }
            if let Some(region) = region {
                regions.push(region);
            }
            collect_binding_events(package, unit, child, events, declares_child, loops, regions);
            if region.is_some() {
                regions.pop();
            }
            if repeats {
                loops.pop();
            }
        }
    }
    if !record_declared_binding_writes(unit, node, declares_binding, events, loops, regions)
        && let Some(target) = assignment_target
        && let Some(declaration_span) = package
            .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            .and_then(|symbol| symbol.declaration_span)
    {
        events
            .entry(span_key(declaration_span))
            .or_default()
            .push(BindingEvent::Write {
                span: node.span,
                loops: loops.clone(),
                regions: regions.clone(),
            });
    }
}

pub(super) fn record_binding_events(package: &mut SemanticPackage) {
    let mut events = BTreeMap::new();
    for unit in &package.units {
        collect_binding_events(
            package,
            unit,
            &unit.tree.root,
            &mut events,
            false,
            &mut Vec::new(),
            &mut Vec::new(),
        );
    }
    package.binding_events = events;
}

pub(super) fn regions_conflict(left: &[ControlRegion], right: &[ControlRegion]) -> bool {
    left.iter().any(|left| {
        right.iter().any(|right| {
            left.statement == right.statement
                && left.arm.is_some()
                && right.arm.is_some()
                && left.arm != right.arm
        })
    })
}

pub(super) fn later_store_replaces(earlier: &[ControlRegion], later: &[ControlRegion]) -> bool {
    later.iter().all(|region| earlier.contains(region))
}

pub(super) fn collect_suspension_points(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    spans: &mut Vec<Span>,
) {
    if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("await")
    {
        spans.push(node.span);
    }
    for child in &node.children {
        collect_suspension_points(unit, child, spans);
    }
}

pub(super) fn moves_binding_between(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    owner_span: Span,
    after: usize,
    before: usize,
) -> bool {
    if node.span.start >= after
        && node.span.end <= before
        && node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("move")
        && node.children.last().is_some_and(|operand| {
            operand.kind == SyntaxKind::Name
                && package
                    .resolve_name_at(unit, operand.span.start, node_text(&unit.source, operand))
                    .and_then(|symbol| symbol.declaration_span)
                    == Some(owner_span)
        })
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| moves_binding_between(package, unit, child, owner_span, after, before))
}

pub(super) fn reference_has_stable_local_owner(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    contract: &FunctionContract,
    reference: &TypedBinding,
) -> bool {
    let Some(declaration) = find_node_by_span(&unit.tree.root, reference.span) else {
        return false;
    };
    let Some(initializer) = declaration.children.last() else {
        return false;
    };
    if initializer.kind != SyntaxKind::UnaryExpression
        || unary_operator_text(unit, initializer).as_deref() != Some("ref")
    {
        return false;
    }
    let Some(source) = initializer
        .children
        .last()
        .filter(|source| source.kind == SyntaxKind::Name)
    else {
        return false;
    };
    let Some(owner_span) = package
        .resolve_name_at(unit, source.span.start, node_text(&unit.source, source))
        .and_then(|symbol| symbol.declaration_span)
    else {
        return false;
    };
    if !unit.typed_bindings.iter().any(|owner| {
        owner.span == owner_span
            && owner.span.start >= contract.span.start
            && owner.span.end <= contract.span.end
    }) {
        return false;
    }
    !moves_binding_between(
        package,
        unit,
        &unit.tree.root,
        owner_span,
        reference.visible_from,
        contract.span.end,
    ) && package
        .binding_events
        .get(&span_key(owner_span))
        .is_none_or(|events| {
            !events.iter().any(|event| {
                matches!(
                    event,
                    BindingEvent::Write { span, .. } if span.start >= reference.visible_from
                )
            })
        })
}

pub(super) fn validate_suspension_ownership(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        let mut awaits = Vec::new();
        collect_suspension_points(unit, &unit.tree.root, &mut awaits);
        for contract in unit.functions.iter().filter(|contract| contract.is_async) {
            for binding in unit.typed_bindings.iter().filter(|binding| {
                matches!(binding.value_type, ValueType::Reference(_))
                    && binding.span.start >= contract.span.start
                    && binding.span.end <= contract.span.end
            }) {
                let Some(events) = package.binding_events.get(&span_key(binding.span)) else {
                    continue;
                };
                if let Some(suspension) = awaits.iter().find(|suspension| {
                    suspension.start >= binding.visible_from
                        && suspension.end <= contract.span.end
                        && events.iter().any(|event| {
                            matches!(
                                event,
                                BindingEvent::Read { span, .. } if span.start > suspension.end
                            )
                        })
                        && !reference_has_stable_local_owner(package, unit, contract, binding)
                }) {
                    return Err(failure(
                        &unit.source,
                        "T0073",
                        format!(
                            "non-owning reference `{}` remains live across `await`; end its use before suspension or transfer owned state",
                            binding.name
                        ),
                        *suspension,
                    ));
                }
            }
        }
    }
    Ok(())
}

pub(super) fn validate_task_consumption(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn consumed(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        binding: &TypedBinding,
        join_argument: bool,
    ) -> bool {
        let await_operand = node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("await");
        let joined = node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                callee.kind == SyntaxKind::MemberExpression
                    && callee
                        .children
                        .get(1)
                        .is_some_and(|member| node_text(&unit.source, member) == "join")
            });
        if join_argument
            && node.kind == SyntaxKind::Name
            && node_text(&unit.source, node) == binding.name
            && unit
                .typed_bindings
                .iter()
                .rev()
                .find(|candidate| {
                    candidate.name == binding.name
                        && candidate.is_visible_at(unit.source.id(), node.span.start)
                })
                .is_some_and(|candidate| candidate.span == binding.span)
        {
            return true;
        }
        node.children.iter().enumerate().any(|(index, child)| {
            consumed(
                unit,
                child,
                binding,
                join_argument || await_operand || (joined && index == 1),
            )
        })
    }

    for unit in &package.units {
        for binding in unit.typed_bindings.iter().filter(|binding| {
            matches!(
                binding.value_type,
                ValueType::Task(_) | ValueType::ScopedTask(_)
            )
        }) {
            if !consumed(unit, &unit.tree.root, binding, false) {
                return Err(failure(
                    &unit.source,
                    "T0076",
                    format!(
                        "task `{}` must be awaited or joined before its scope ends",
                        binding.name
                    ),
                    binding.span,
                ));
            }
        }
    }
    Ok(())
}

pub(crate) fn descriptor_binding_is_materialized(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
) -> bool {
    fn read_materializes(node: &SyntaxNode, read_span: Span, is_designator: bool) -> Option<bool> {
        if node.kind == SyntaxKind::Name && node.span == read_span {
            return Some(!is_designator);
        }
        node.children.iter().enumerate().find_map(|(index, child)| {
            let child_is_designator = index == 0
                && matches!(
                    node.kind,
                    SyntaxKind::ConstructionExpression | SyntaxKind::StaticMemberExpression
                );
            read_materializes(child, read_span, child_is_designator)
        })
    }

    package
        .binding_events
        .get(&span_key(declaration_span))
        .is_some_and(|events| {
            events.iter().any(|event| {
                let BindingEvent::Read { span, .. } = event else {
                    return false;
                };
                read_materializes(&unit.tree.root, *span, false).unwrap_or(false)
            })
        })
}

pub(crate) fn binding_store_value_is_read(
    package: &SemanticPackage,
    declaration_span: Span,
    store_span: Span,
) -> bool {
    let Some(events) = package.binding_events.get(&span_key(declaration_span)) else {
        return false;
    };
    let Some((store, store_loops, store_regions)) =
        events.iter().enumerate().find_map(|(index, event)| {
            let BindingEvent::Write {
                span,
                loops,
                regions,
            } = event
            else {
                return None;
            };
            (*span == store_span).then_some((index, loops, regions))
        })
    else {
        return false;
    };
    let mut intervening_stores: Vec<&[ControlRegion]> = Vec::new();
    for event in &events[store + 1..] {
        match event {
            BindingEvent::Read { regions, .. }
                if !regions_conflict(store_regions, regions)
                    && !intervening_stores
                        .iter()
                        .any(|intervening| later_store_replaces(regions, intervening)) =>
            {
                return true;
            }
            BindingEvent::Write { regions, .. } => {
                if later_store_replaces(store_regions, regions) {
                    return false;
                }
                intervening_stores.push(regions.as_slice());
            }
            BindingEvent::Read { .. } => {}
        }
    }
    !store_loops.is_empty()
        && events.iter().any(|event| {
            let BindingEvent::Read {
                loops: read_loops, ..
            } = event
            else {
                return false;
            };
            store_loops
                .iter()
                .any(|store_loop| read_loops.contains(store_loop))
        })
}
