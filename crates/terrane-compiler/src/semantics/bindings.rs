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
        if node.kind == SyntaxKind::SelectStatement {
            let incoming = assigned.clone();
            let mut branch_results = Vec::with_capacity(node.children.len());
            for case in &node.children {
                let Some([header, block]) = case.children.get(..2) else {
                    continue;
                };
                validate_node(package, unit, header, relevant, assigned)?;
                let mut branch_assigned = incoming.clone();
                validate_node(package, unit, block, relevant, &mut branch_assigned)?;
                branch_results.push(branch_assigned);
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
        && let Some(function) = projected_function_for_call(package, unit, callee)
    {
        let projected_parameters = unit
            .projected_call_specializations
            .get(&(node.span.file, node.span.start, node.span.end))
            .map_or(function.parameters.as_slice(), |specialization| {
                specialization.projected_parameters.as_slice()
            });
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
                    projected_parameters
                        .iter()
                        .position(|parameter| parameter.name == node_text(&unit.source, name))
                        .unwrap_or(usize::MAX)
                },
            );
            let value = argument.children.last().unwrap_or(argument);
            let value = if value.kind == SyntaxKind::UnaryExpression
                && unary_operator_text(unit, value).as_deref() == Some("ref")
            {
                value.children.last().unwrap_or(value)
            } else {
                value
            };
            if projected_parameters
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
    let mut pending = Vec::new();
    loop {
        for (unit_index, unit) in package.units.iter().enumerate() {
            for (function_index, function) in unit.functions.iter().enumerate() {
                for (parameter_index, parameter) in function.parameters.iter().enumerate() {
                    if !parameter.mutable
                        && binding_span_is_mutated(
                            package,
                            unit,
                            parameter.span,
                            true,
                            ClosureWrites::Include,
                        )
                    {
                        pending.push((unit_index, function_index, parameter_index));
                    }
                }
            }
        }
        if pending.is_empty() {
            break;
        }
        for (unit, function, parameter) in pending.drain(..) {
            package.units[unit].functions[function].parameters[parameter].mutable = true;
        }
    }
    let mutable_bindings = package
        .units
        .iter()
        .map(|unit| {
            unit.typed_bindings
                .iter()
                .map(|binding| {
                    let initially_assigned =
                        unit.source.text()[binding.span.start..binding.span.end].contains('=');
                    binding_span_is_mutated(
                        package,
                        unit,
                        binding.span,
                        initially_assigned,
                        ClosureWrites::Include,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for (unit, binding_mutability) in package.units.iter_mut().zip(mutable_bindings) {
        for (binding, mutable) in unit.typed_bindings.iter_mut().zip(binding_mutability) {
            binding.mutable = mutable;
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
    if node.kind == SyntaxKind::SelectStatement && child.kind == SyntaxKind::SelectCase {
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
            | SyntaxKind::CatchBinding
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
    let initial_store = matches!(
        node.kind,
        SyntaxKind::ForTarget | SyntaxKind::Parameter | SyntaxKind::CatchBinding
    ) || unit.source.text()[node.span.start..node.span.end].contains('=');
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
pub(crate) fn binding_requires_mutable_storage(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
    initially_assigned: bool,
    closure_writes: ClosureWrites,
) -> bool {
    if initially_assigned {
        return binding_span_is_mutated(package, unit, declaration_span, true, closure_writes);
    }
    let declaration_function = unit
        .enclosing_function_spans
        .get(&declaration_span.start)
        .copied()
        .flatten();
    let writes = package
        .binding_events
        .get(&span_key(declaration_span))
        .into_iter()
        .flatten()
        .filter_map(|event| match event {
            BindingEvent::Write {
                span,
                loops,
                regions,
            } if closure_writes == ClosureWrites::Include
                || unit
                    .enclosing_function_spans
                    .get(&span.start)
                    .copied()
                    .flatten()
                    == declaration_function =>
            {
                Some((loops, regions))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    writes.iter().any(|(loops, _)| !loops.is_empty())
        || writes.iter().enumerate().any(|(index, (_, left))| {
            writes
                .iter()
                .skip(index + 1)
                .any(|(_, right)| !regions_conflict(left, right))
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

#[derive(Clone, Copy)]
pub(super) enum AutoTraitObligation {
    Send,
    Sync,
}

#[expect(
    clippy::too_many_lines,
    reason = "Recursive auto-trait proof cases stay aligned with the complete semantic value shape"
)]
pub(super) fn value_type_satisfies_auto_trait(
    package: &SemanticPackage,
    value_type: &ValueType,
    obligation: AutoTraitObligation,
) -> bool {
    fn satisfies(
        package: &SemanticPackage,
        value_type: &ValueType,
        obligation: AutoTraitObligation,
        visiting: &mut BTreeSet<String>,
    ) -> bool {
        match value_type {
            ValueType::Reference(_) => false,
            ValueType::Object(identity) => {
                if let Some((send, sync)) = package
                    .projection
                    .foreign_auto_traits(&identity.namespace, &identity.name)
                {
                    return match obligation {
                        AutoTraitObligation::Send => send,
                        AutoTraitObligation::Sync => sync,
                    };
                }
                let key = format!("{}::{}", identity.namespace, identity.name);
                if !visiting.insert(key.clone()) {
                    return true;
                }
                let result = package
                    .units
                    .iter()
                    .flat_map(|unit| &unit.descriptors)
                    .find(|object| object.identity.base() == identity.base())
                    .is_some_and(|object| match object.kind {
                        ObjectKind::Interface => true,
                        ObjectKind::Class => {
                            let substitutions = object
                                .generic_parameters
                                .iter()
                                .zip(&identity.type_arguments)
                                .map(|(parameter, argument)| {
                                    (parameter.name.clone(), argument.clone())
                                })
                                .collect();
                            effective_object_fields(package, object)
                                .into_iter()
                                .all(|field| {
                                    field.is_static
                                        || satisfies(
                                            package,
                                            &substitute_value_type(
                                                &field.value_type,
                                                &substitutions,
                                            ),
                                            obligation,
                                            visiting,
                                        )
                                })
                        }
                        ObjectKind::Enum => package
                            .units
                            .iter()
                            .flat_map(|unit| &unit.source_enums)
                            .find(|enumeration| enumeration.identity.base() == identity.base())
                            .is_some_and(|enumeration| {
                                let substitutions = object
                                    .generic_parameters
                                    .iter()
                                    .zip(&identity.type_arguments)
                                    .map(|(parameter, argument)| {
                                        (parameter.name.clone(), argument.clone())
                                    })
                                    .collect();
                                enumeration.variants.iter().all(|variant| {
                                    variant.payload.iter().all(|field| {
                                        satisfies(
                                            package,
                                            &substitute_value_type(
                                                &field.value_type,
                                                &substitutions,
                                            ),
                                            obligation,
                                            visiting,
                                        )
                                    })
                                })
                            }),
                        ObjectKind::Trait | ObjectKind::Type => false,
                    });
                visiting.remove(&key);
                result
            }
            ValueType::Optional(inner) => satisfies(package, inner, obligation, visiting),
            ValueType::Iterator(inner)
            | ValueType::IterationStep(inner)
            | ValueType::List(inner)
            | ValueType::Set(inner)
            | ValueType::Tuple(inner, _)
            | ValueType::UnorderedSet(inner)
            | ValueType::TaskOutcome(inner) => {
                satisfies(package, inner.value_type_ref(), obligation, visiting)
            }
            ValueType::SharedReference(inner) => match obligation {
                AutoTraitObligation::Send => {
                    satisfies(
                        package,
                        inner.value_type_ref(),
                        AutoTraitObligation::Send,
                        visiting,
                    ) && satisfies(
                        package,
                        inner.value_type_ref(),
                        AutoTraitObligation::Sync,
                        visiting,
                    )
                }
                AutoTraitObligation::Sync => {
                    satisfies(package, inner.value_type_ref(), obligation, visiting)
                }
            },
            ValueType::Map(key, value)
            | ValueType::Entry(key, value)
            | ValueType::UnorderedMap(key, value) => {
                satisfies(package, key.value_type_ref(), obligation, visiting)
                    && satisfies(package, value.value_type_ref(), obligation, visiting)
            }
            ValueType::AsyncFunction(_, _, transferability, _)
            | ValueType::Task(_, transferability)
            | ValueType::ScopedTask(_, transferability) => match obligation {
                AutoTraitObligation::Send => *transferability != TaskTransferability::Local,
                AutoTraitObligation::Sync => false,
            },
            _ => true,
        }
    }

    satisfies(package, value_type, obligation, &mut BTreeSet::new())
}

pub(super) fn value_type_is_task_transferable(
    package: &SemanticPackage,
    value_type: &ValueType,
) -> bool {
    match value_type {
        ValueType::Reference(_) => false,
        ValueType::Object(identity) => {
            !identity.namespace.starts_with("/deps/")
                || package
                    .projection
                    .projected_type_is_send(&identity.namespace, &identity.name)
        }
        ValueType::Optional(inner) => value_type_is_task_transferable(package, inner),
        ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::UnorderedSet(inner)
        | ValueType::TaskOutcome(inner)
        | ValueType::SharedReference(inner) => {
            value_type_is_task_transferable(package, inner.value_type_ref())
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_is_task_transferable(package, key.value_type_ref())
                && value_type_is_task_transferable(package, value.value_type_ref())
        }
        ValueType::AsyncFunction(_, _, transferability, _)
        | ValueType::Task(_, transferability)
        | ValueType::ScopedTask(_, transferability) => {
            *transferability != TaskTransferability::Local
        }
        _ => true,
    }
}

pub(super) fn value_type_is_owned_static(value_type: &ValueType) -> bool {
    match value_type {
        ValueType::Reference(_) => false,
        ValueType::Optional(inner) => value_type_is_owned_static(inner),
        ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::UnorderedSet(inner)
        | ValueType::TaskOutcome(inner)
        | ValueType::SharedReference(inner) => value_type_is_owned_static(inner.value_type_ref()),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_is_owned_static(key.value_type_ref())
                && value_type_is_owned_static(value.value_type_ref())
        }
        _ => true,
    }
}

fn async_boundary_transferability(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    function_span: Span,
) -> TaskTransferability {
    if node.span.start < function_span.start || node.span.end > function_span.end {
        return TaskTransferability::Transferable;
    }
    if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && resolved_function_contract(unit, node_text(&unit.source, callee), callee.span.start)
            .is_some_and(|contract| {
                contract.is_async && contract.task_transferability == TaskTransferability::Local
            })
    {
        return if projected_function_for_call(package, unit, callee).is_some() {
            TaskTransferability::RustProven
        } else {
            TaskTransferability::Local
        };
    }
    node.children
        .iter()
        .map(|child| async_boundary_transferability(package, unit, child, function_span))
        .min_by_key(|transferability| match transferability {
            TaskTransferability::Local => 0,
            TaskTransferability::RustProven => 1,
            TaskTransferability::Transferable => 2,
        })
        .unwrap_or(TaskTransferability::Transferable)
}

pub(super) fn infer_task_transferability(package: &mut SemanticPackage) {
    for unit_index in 0..package.units.len() {
        let updates = {
            let unit = &package.units[unit_index];
            let mut suspensions = Vec::new();
            collect_suspension_points(unit, &unit.tree.root, &mut suspensions);
            unit.functions
                .iter()
                .filter(|contract| contract.is_async)
                .map(|contract| {
                    let boundary = async_boundary_transferability(
                        package,
                        unit,
                        &unit.tree.root,
                        contract.span,
                    );
                    let values_transfer = contract.parameters.iter().all(|parameter| {
                        parameter.value_type.as_ref().is_none_or(|value_type| {
                            value_type_is_task_transferable(package, value_type)
                        })
                    }) && unit.typed_bindings.iter().all(|binding| {
                        if binding.span.start < contract.span.start
                            || binding.span.end > contract.span.end
                            || value_type_is_task_transferable(package, &binding.value_type)
                        {
                            return true;
                        }
                        let Some(events) = package.binding_events.get(&span_key(binding.span))
                        else {
                            return true;
                        };
                        !suspensions.iter().any(|suspension| {
                            suspension.start >= binding.visible_from
                                && suspension.end <= contract.span.end
                                && events.iter().any(|event| {
                                    matches!(
                                        event,
                                        BindingEvent::Read { span, .. }
                                            if span.start > suspension.end
                                    )
                                })
                        })
                    });
                    let transferability = if unit.namespace.starts_with("/deps/")
                        || boundary == TaskTransferability::Local
                        || !values_transfer
                    {
                        TaskTransferability::Local
                    } else {
                        boundary
                    };
                    (contract.span, transferability)
                })
                .collect::<Vec<_>>()
        };
        for contract in &mut package.units[unit_index].functions {
            if let Some((_, transferability)) =
                updates.iter().find(|(span, _)| *span == contract.span)
            {
                contract.task_transferability = *transferability;
            }
        }
    }
    for unit in &mut package.units {
        for contract in &mut unit.functions {
            if !contract.is_async {
                continue;
            }
            contract.execution_requirements.tasks.local =
                contract.task_transferability == TaskTransferability::Local;
            contract.execution_requirements.tasks.transferable =
                contract.task_transferability != TaskTransferability::Local;
        }
    }
    synchronize_execution_requirements(package);
}

pub(super) fn synchronize_execution_requirements(package: &mut SemanticPackage) {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .map(|contract| {
            (
                span_key(contract.span),
                (
                    contract.task_transferability,
                    contract.execution_requirements,
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for unit in &mut package.units {
        for contract in unit
            .function_aliases
            .values_mut()
            .chain(unit.function_contracts_by_span.values_mut())
        {
            if let Some((transferability, requirements)) = contracts.get(&span_key(contract.span)) {
                contract.task_transferability = *transferability;
                contract.execution_requirements = *requirements;
            }
        }
    }
    let mut requirements = crate::execution::ExecutionRequirements::default();
    for contract in package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .filter(|contract| {
            contract.is_async
                && (contract.name == "main" || package.function_is_referenced(contract.span))
        })
    {
        requirements.merge(contract.execution_requirements);
    }
    package.execution_requirements = requirements;
    if package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .any(|contract| contract.name == "main" && contract.is_async)
    {
        package.execution_requirements.runtime.context = true;
        package.execution_requirements.runtime.wake_support = true;
    }
    if package.units.iter().any(|unit| {
        unit.typed_bindings
            .iter()
            .any(|binding| binding.value_type == ValueType::TaskScope)
    }) {
        package.execution_requirements.runtime.wake_support = true;
    }
    if package.units.iter().any(|unit| {
        [
            "host-read-async",
            "host-tcp-connect-async",
            "host-tcp-connect-host-async",
            "host-tcp-accept-async",
            "host-tcp-read-async",
            "host-tcp-write-async",
            "host-udp-send-to-async",
            "host-udp-receive-from-async",
            "host-dns-lookup-async",
            "host-tls-client-async",
            "host-tls-read-async",
            "host-tls-write-async",
            "host-tls-shutdown-async",
            "host-time-sleep-until",
            "host-process-signal-next",
        ]
        .iter()
        .any(|name| unit.source.text().contains(name))
    }) {
        package.execution_requirements.runtime.context = true;
        package.execution_requirements.runtime.wake_support = true;
        package.execution_requirements.runtime.blocking_delegation = true;
    }
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
    fn discarded_task(
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<Option<Span>, SemanticFailure> {
        for child in &node.children {
            if node.kind == SyntaxKind::Block
                && child.kind == SyntaxKind::CallExpression
                && matches!(
                    infer_value_type(unit, child, &unit.typed_bindings)?,
                    Some(ValueType::Task(_, _) | ValueType::ScopedTask(_, _))
                )
            {
                return Ok(Some(child.span));
            }
            if let Some(span) = discarded_task(unit, child)? {
                return Ok(Some(span));
            }
        }
        Ok(None)
    }

    fn consumed(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        binding: &TypedBinding,
        consuming: bool,
    ) -> bool {
        let move_operand = node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("move");
        let await_operand = node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("await");
        let assignment_value = node.kind == SyntaxKind::Assignment;
        let task_consumer = node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                let [receiver, member] = callee.children.as_slice() else {
                    return false;
                };
                callee.kind == SyntaxKind::MemberExpression
                    && matches!(
                        infer_value_type(unit, receiver, &unit.typed_bindings),
                        Ok(Some(ValueType::TaskScope))
                    )
                    && matches!(node_text(&unit.source, member), "join" | "spawn")
            });
        if consuming
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
                consuming
                    || move_operand
                    || await_operand
                    || (assignment_value && index == 1)
                    || (task_consumer && index == 1),
            )
        })
    }

    for unit in &package.units {
        if let Some(span) = discarded_task(unit, &unit.tree.root)? {
            return Err(failure(
                &unit.source,
                "T0076",
                "task must be awaited, joined, or bound for later consumption",
                span,
            ));
        }
        for binding in unit.typed_bindings.iter().filter(|binding| {
            matches!(
                binding.value_type,
                ValueType::Task(_, _) | ValueType::ScopedTask(_, _)
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
pub(super) fn validate_task_transferability(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn visit(unit: &SemanticUnit, node: &SyntaxNode) -> Result<Option<Span>, SemanticFailure> {
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && node_text(&unit.source, member) == "spawn"
            && infer_value_type(unit, receiver, &unit.typed_bindings)? == Some(ValueType::TaskScope)
            && let Some(arguments) = node.children.get(1)
            && let Some(argument) = arguments.children.first()
        {
            let callable = argument.children.last().unwrap_or(argument);
            if matches!(
                infer_value_type(unit, callable, &unit.typed_bindings)?,
                Some(
                    ValueType::AsyncFunction(_, _, TaskTransferability::Local, _)
                        | ValueType::Task(_, TaskTransferability::Local)
                )
            ) {
                return Ok(Some(callable.span));
            }
        }
        for child in &node.children {
            if let Some(span) = visit(unit, child)? {
                return Ok(Some(span));
            }
        }
        Ok(None)
    }

    if package.execution_strategy != crate::execution::ExecutionStrategy::Parallel {
        return Ok(());
    }
    for unit in &package.units {
        if let Some(span) = visit(unit, &unit.tree.root)? {
            return Err(failure(
                &unit.source,
                "T0104",
                "executor-local async callable cannot be spawned by the threaded executor",
                span,
            ));
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

pub(crate) fn binding_read_value_is_reused(
    package: &SemanticPackage,
    declaration_span: Span,
    read_span: Span,
) -> bool {
    let Some(events) = package.binding_events.get(&span_key(declaration_span)) else {
        return false;
    };
    let Some((read, read_loops, read_regions)) =
        events.iter().enumerate().find_map(|(index, event)| {
            let BindingEvent::Read {
                span,
                loops,
                regions,
            } = event
            else {
                return None;
            };
            (*span == read_span).then_some((index, loops, regions))
        })
    else {
        return false;
    };
    let mut intervening_stores: Vec<&[ControlRegion]> = Vec::new();
    for event in &events[read + 1..] {
        match event {
            BindingEvent::Read { regions, .. }
                if !regions_conflict(read_regions, regions)
                    && !intervening_stores
                        .iter()
                        .any(|intervening| later_store_replaces(regions, intervening)) =>
            {
                return true;
            }
            BindingEvent::Write { regions, .. } => {
                if later_store_replaces(read_regions, regions) {
                    return false;
                }
                intervening_stores.push(regions.as_slice());
            }
            BindingEvent::Read { .. } => {}
        }
    }
    read_loops.last().is_some_and(|loop_span| {
        declaration_span.file != loop_span.file
            || declaration_span.start < loop_span.start
            || loop_span.end < declaration_span.end
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

fn node_with_span(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    if node.span == span {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| node_with_span(child, span))
}

fn declaration_node_with_name_span(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && node.children.iter().any(|child| child.span == span)
    {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| declaration_node_with_name_span(child, span))
}

pub(crate) fn callback_contract<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    value: &SyntaxNode,
) -> Option<&'a FunctionContract> {
    if value.kind == SyntaxKind::Name {
        if let Some(contract) =
            resolved_function_contract(unit, node_text(&unit.source, value), value.span.start)
        {
            return Some(contract);
        }
        let declaration = package
            .resolve_name_at(unit, value.span.start, node_text(&unit.source, value))?
            .declaration_span?;
        let declaration_node = declaration_node_with_name_span(&unit.tree.root, declaration)
            .or_else(|| node_with_span(&unit.tree.root, declaration))?;
        let initializer = super::ownership::binding_initializer(declaration_node)?;
        if initializer.span.start >= value.span.start {
            return None;
        }
        return callback_contract(package, unit, initializer);
    }
    if value.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = value.children.as_slice()
        && let Ok(Some(ValueType::Object(owner))) =
            infer_value_type(unit, receiver, &unit.typed_bindings)
    {
        return unit.functions.iter().find(|contract| {
            contract.owner.as_deref() == Some(owner.name.as_str())
                && contract.name == node_text(&unit.source, member)
        });
    }
    unit.functions
        .iter()
        .find(|contract| contract.span == value.span)
}

fn captured_binding<'a>(
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    contract: &FunctionContract,
    name: &str,
) -> Option<&'a TypedBinding> {
    let declaration = package
        .resolve_name_at(unit, contract.span.start, name)?
        .declaration_span?;
    unit.typed_bindings
        .iter()
        .find(|binding| binding.span == declaration)
}
fn validate_projected_callback_contract(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    value: &SyntaxNode,
    contract: &FunctionContract,
    callback: &crate::rust_interop::projection::ProjectedType,
    consumed_once: &mut BTreeSet<(u32, usize, usize)>,
) -> Result<(), SemanticFailure> {
    let crate::rust_interop::projection::ProjectedType::Callback {
        invocation_mode,
        retained,
        send,
        parameter_borrows,
        parameter_rust_types,
        ..
    } = callback
    else {
        unreachable!("callback validation requires callback metadata");
    };
    let reject = |code, message| Err(failure(&unit.source, code, message, value.span));
    for (index, parameter) in contract.parameters.iter().enumerate() {
        if parameter_borrows.get(index) == Some(&true)
            && parameter_rust_types.get(index).is_some_and(|rust_type| {
                matches!(
                    syn::parse_str::<syn::Type>(rust_type),
                    Ok(syn::Type::Reference(reference)) if reference.mutability.is_none()
                )
            })
            && package
                .units
                .iter()
                .find(|source| source.source.id() == contract.span.file)
                .and_then(|source| {
                    source
                        .functions
                        .iter()
                        .find(|actual| actual.span == contract.span)
                })
                .and_then(|source| {
                    source
                        .parameters
                        .iter()
                        .find(|actual| actual.span == parameter.span)
                })
                .is_some_and(|source| source.mutable)
        {
            return Err(failure(
                &unit.source,
                "T0139",
                format!(
                    "mutating callback parameter `{}` requires a mutable native reference; the selected callback supplies a shared reference",
                    parameter.name
                ),
                value.span,
            ));
        }
    }
    if *send && contract.task_transferability == TaskTransferability::Local {
        return reject(
            "T0081",
            "projected callback requires a transferable callable",
        );
    }
    if *retained && contract.owner.is_some() && !contract.is_static {
        return reject(
            "T0082",
            "retained callback cannot borrow an object receiver",
        );
    }
    if *retained
        && contract.captures.iter().any(|capture| {
            captured_binding(package, unit, contract, capture)
                .is_some_and(|binding| matches!(binding.value_type, ValueType::Reference(_)))
        })
    {
        return reject("T0083", "retained callback cannot capture a borrowed value");
    }
    if *send
        && contract.captures.iter().any(|capture| {
            captured_binding(package, unit, contract, capture).is_some_and(|binding| {
                !value_type_is_task_transferable(package, &binding.value_type)
            })
        })
    {
        return reject(
            "T0084",
            "projected callback requires transferable captured values",
        );
    }
    if *invocation_mode == InvocationMode::Consuming
        && value.kind == SyntaxKind::Name
        && let Some(declaration) = package
            .resolve_name_at(unit, value.span.start, node_text(&unit.source, value))
            .and_then(|symbol| symbol.declaration_span)
        && unit
            .typed_bindings
            .iter()
            .any(|binding| binding.span == declaration)
        && !consumed_once.insert(span_key(declaration))
    {
        return reject("T0086", "one-shot projected callback was already consumed");
    }
    Ok(())
}

fn validate_channel_endpoint_extraction(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    extracted: &mut BTreeSet<(u32, usize, usize, String)>,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = node.children.as_slice()
        && receiver.kind == SyntaxKind::Name
        && matches!(node_text(&unit.source, member), "sender" | "receiver")
        && let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, receiver)
                && binding.is_visible_at(unit.source.id(), receiver.span.start)
        })
        && matches!(binding.value_type, ValueType::ChannelPair(_))
        && !extracted.insert((
            binding.span.file,
            binding.span.start,
            binding.span.end,
            node_text(&unit.source, member).to_owned(),
        ))
    {
        return Err(failure(
            &unit.source,
            "T0110",
            format!(
                "channel pair `{}` endpoint `{}` was already moved",
                binding.name,
                node_text(&unit.source, member)
            ),
            node.span,
        ));
    }
    Ok(())
}

fn validate_projected_borrowed_async_call(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    immediately_awaited: bool,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::CallExpression
        && !immediately_awaited
        && let [callee, _] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && receiver.kind == SyntaxKind::Name
        && package
            .projection
            .has_borrowed_async_method_named(node_text(&unit.source, member))
        && let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, receiver)
                && binding.is_visible_at(unit.source.id(), receiver.span.start)
        })
        && let ValueType::Object(identity) = &binding.value_type
        && let Some(method) = package.projection.method(
            &identity.namespace,
            &identity.name,
            node_text(&unit.source, member),
            false,
        )
        && method.is_async
        && matches!(
            method.receiver,
            Some(
                crate::rust_interop::projection::Receiver::Borrow
                    | crate::rust_interop::projection::Receiver::MutableBorrow
            )
        )
    {
        return Err(failure(
            &unit.source,
            "T0106",
            "borrowed projected async operation must be awaited immediately",
            node.span,
        ));
    }
    Ok(())
}

fn projected_chain_role(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Option<crate::rust_interop::projection::ChainRole> {
    if node.kind != SyntaxKind::CallExpression {
        return None;
    }
    let [callee, _] = node.children.as_slice() else {
        return None;
    };
    let function = projected_function_for_call(package, unit, callee)?;
    function.chain_role.or_else(|| {
        let receiver = callee.children.first()?;
        matches!(
            infer_value_type(unit, receiver, &unit.typed_bindings)
                .ok()
                .flatten(),
            Some(ValueType::InvocationScopedNative {
                expression_scoped: true,
                ..
            })
        )
        .then_some(crate::rust_interop::projection::ChainRole::Terminal)
    })
}

fn collect_chain_receivers(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    receivers: &mut BTreeSet<(u32, usize, usize)>,
) {
    if node.kind == SyntaxKind::Block {
        for statement in &node.children {
            let mut value = statement;
            while value.kind == SyntaxKind::GroupExpression
                && let [inner] = value.children.as_slice()
            {
                value = inner;
            }
            if value.kind == SyntaxKind::CallExpression
                && matches!(
                    infer_value_type(unit, value, &unit.typed_bindings)
                        .ok()
                        .flatten(),
                    Some(ValueType::InvocationScopedNative {
                        expression_scoped: true,
                        ..
                    })
                )
            {
                receivers.insert(span_key(value.span));
            }
        }
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, _] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, _] = callee.children.as_slice()
        && projected_chain_role(package, unit, node).is_some()
    {
        let mut receiver = receiver;
        while receiver.kind == SyntaxKind::GroupExpression
            && let [inner] = receiver.children.as_slice()
        {
            receiver = inner;
        }
        receivers.insert(span_key(receiver.span));
    }
    if node.kind == SyntaxKind::CallExpression
        && projected_function_for_call(
            package,
            unit,
            node.children.first().expect("call expression has a callee"),
        )
        .is_some()
        && let Some(arguments) = node.children.get(1)
    {
        for argument in &arguments.children {
            let mut value = argument.children.last().unwrap_or(argument);
            while value.kind == SyntaxKind::GroupExpression
                && let Some(inner) = value.children.first()
            {
                value = inner;
            }
            if matches!(
                projected_chain_role(package, unit, value),
                Some(
                    crate::rust_interop::projection::ChainRole::Root
                        | crate::rust_interop::projection::ChainRole::Continue
                )
            ) {
                receivers.insert(span_key(value.span));
            }
        }
    }
    for child in &node.children {
        collect_chain_receivers(package, unit, child, receivers);
    }
}

pub(crate) fn projected_macro_for_call<'a>(
    package: &'a SemanticPackage,
    unit: &SemanticUnit,
    callee: &SyntaxNode,
) -> Option<&'a crate::rust_interop::projection::ProjectedItem> {
    if callee.kind != SyntaxKind::Name {
        return None;
    }
    let symbol =
        package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
    package
        .projection
        .item(&symbol.namespace, &symbol.name)
        .filter(|item| {
            matches!(
                item.kind,
                crate::rust_interop::projection::ProjectedKind::Macro(_)
            )
        })
}

pub(super) fn projected_function_for_call<'a>(
    package: &'a SemanticPackage,
    unit: &SemanticUnit,
    callee: &SyntaxNode,
) -> Option<&'a crate::rust_interop::projection::ProjectedFunction> {
    let is_unsafe = crate::syntax::call_is_unsafe(callee);
    if callee.kind == SyntaxKind::ConstructionExpression {
        let identity = class_designator_identity(unit, callee.children.first()?)?;
        return package
            .projection
            .projected_constructor(&identity.namespace, &identity.name);
    }
    if callee.kind == SyntaxKind::Name {
        let lookup_name = if is_unsafe {
            format!("unsafe::{}", node_text(&unit.source, callee))
        } else {
            node_text(&unit.source, callee).to_owned()
        };
        let symbol = package.resolve_name_at(unit, callee.span.start, &lookup_name)?;
        return package
            .projection
            .item(&symbol.namespace, &symbol.name)
            .and_then(|item| match &item.kind {
                crate::rust_interop::projection::ProjectedKind::Function(function)
                | crate::rust_interop::projection::ProjectedKind::Macro(function) => Some(function),
                _ => None,
            });
    }
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    let (identity, is_static) = if callee.kind == SyntaxKind::StaticMemberExpression {
        (class_designator_identity(unit, receiver)?, true)
    } else {
        let receiver_type = infer_value_type(unit, receiver, &unit.typed_bindings)
            .ok()
            .flatten()?;
        let identity = match receiver_type {
            ValueType::Object(identity)
            | ValueType::InvocationScopedNative {
                family: identity, ..
            } => identity,
            ValueType::Reference(inner) | ValueType::SharedReference(inner) => {
                let ValueType::Object(identity) = inner.value_type() else {
                    return None;
                };
                identity
            }
            _ => return None,
        };
        (identity, false)
    };
    package.projection.method_for_native(
        &identity.namespace,
        &identity.name,
        identity.native_projection.as_deref(),
        node_text(&unit.source, member),
        is_static,
        is_unsafe,
    )
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum InvocationScopedRegion {
    Unbound,
    Expression,
    Bound((u32, usize, usize)),
}

fn invocation_scoped_region(value_type: &ValueType) -> Option<InvocationScopedRegion> {
    match value_type {
        ValueType::InvocationScopedNative {
            expression_scoped: true,
            ..
        } => Some(InvocationScopedRegion::Expression),
        ValueType::InvocationScopedNative { region, .. } => Some(region.map_or(
            InvocationScopedRegion::Unbound,
            InvocationScopedRegion::Bound,
        )),
        ValueType::Optional(inner) => invocation_scoped_region(inner),
        ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::Reference(inner)
        | ValueType::SharedReference(inner) => invocation_scoped_region(inner.value_type_ref()),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => invocation_scoped_region(key.value_type_ref())
            .or_else(|| invocation_scoped_region(value.value_type_ref())),
        ValueType::Tuple(item, _) => invocation_scoped_region(item.value_type_ref()),
        _ => None,
    }
}

fn projected_native_name(projected: &crate::rust_interop::projection::ProjectedType) -> String {
    match projected {
        crate::rust_interop::projection::ProjectedType::InvocationScoped { name, .. } => {
            name.clone()
        }
        _ => projected.rust_type(),
    }
}

fn visible_binding<'a>(
    unit: &'a SemanticUnit,
    name_node: &SyntaxNode,
) -> Option<&'a crate::semantics::model::TypedBinding> {
    let name = node_text(&unit.source, name_node);
    unit.typed_bindings
        .iter()
        .filter(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), name_node.span.start)
        })
        .max_by_key(|binding| binding.visible_from)
}

fn local_lender_name(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contract: &FunctionContract,
    function_span: Span,
) -> Option<(String, Span)> {
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        let binding = visible_binding(unit, node)?;
        if !contract
            .parameters
            .iter()
            .any(|parameter| parameter.name == name)
            && binding.scope == Some(function_span)
            && invocation_scoped_region(&binding.value_type).is_none()
        {
            return Some((name.to_owned(), binding.span));
        }
    }
    node.children
        .iter()
        .find_map(|child| local_lender_name(unit, child, contract, function_span))
}

fn local_lender_in_scoped_expression(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contract: &FunctionContract,
    function_span: Span,
) -> Option<(String, Span)> {
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && (projected_function_for_call(package, unit, callee).is_some_and(|function| {
            matches!(
                function.result,
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
            )
        }) || (callee.kind == SyntaxKind::Name
            && resolved_function_contract(
                unit,
                node_text(&unit.source, callee),
                callee.span.start,
            )
            .is_some_and(|function| {
                matches!(
                    function.return_type,
                    Some(ValueType::InvocationScopedNative { .. })
                )
            })))
    {
        let lender = arguments
            .children
            .iter()
            .find_map(|argument| local_lender_name(unit, argument, contract, function_span));
        if lender.is_some() {
            return lender;
        }
    }
    node.children.iter().find_map(|child| {
        local_lender_in_scoped_expression(package, unit, child, contract, function_span)
    })
}

fn collect_scoped_append_types(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    receiver_name: &str,
    results: &mut Vec<crate::rust_interop::projection::ProjectedType>,
) {
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && node_text(&unit.source, receiver) == receiver_name
        && node_text(&unit.source, member) == "append"
        && let Some(argument) = arguments.children.first()
    {
        let value = argument.children.last().unwrap_or(argument);
        if let Ok(Some(value_type)) = infer_value_type(unit, value, &unit.typed_bindings)
            && let Ok(projected) = super::objects::destination_projected_type(package, &value_type)
            && !results
                .iter()
                .any(|result| super::objects::same_projected_native_family(result, &projected))
        {
            results.push(projected);
        }
    }
    for child in &node.children {
        collect_scoped_append_types(package, unit, child, receiver_name, results);
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one syntax walk keeps invocation-region rejection rules in source order"
)]
fn validate_invocation_scoped_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::FunctionDeclaration
        && let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
        && !unit
            .source
            .path()
            .to_string_lossy()
            .starts_with("<terrane>/projected/")
        && let Some(parameter) = contract.parameters.iter().find(|parameter| {
            matches!(
                &parameter.value_type,
                Some(ValueType::InvocationScopedNative {
                    concrete: false,
                    ..
                })
            )
        })
    {
        return Err(failure(
            &unit.source,
            "T0119",
            format!(
                "parameter `{}` cannot use unresolved `host-invocation-scoped-native` as a source type",
                parameter.name
            ),
            parameter.span,
        ));
    }
    if node.kind == SyntaxKind::CallExpression
        && let Some(function_span) = unit
            .enclosing_function_spans
            .get(&node.span.start)
            .copied()
            .flatten()
        && let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.span == function_span)
        && let Some((name, span)) =
            local_lender_in_scoped_expression(package, unit, node, contract, function_span)
    {
        return Err(failure(
            &unit.source,
            "T0119",
            format!("invocation-scoped native result cannot borrow local value `{name}`"),
            span,
        ));
    }

    if node.kind == SyntaxKind::CallExpression
        && let [callee, _] = node.children.as_slice()
        && callee.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = callee.children.as_slice()
        && receiver.kind == SyntaxKind::Name
        && node_text(&unit.source, member) == "append"
    {
        let receiver_name = node_text(&unit.source, receiver);
        let scoped_wildcard_list = unit
            .typed_bindings
            .iter()
            .filter(|binding| {
                binding.name == receiver_name
                    && binding.is_visible_at(unit.source.id(), receiver.span.start)
            })
            .max_by_key(|binding| binding.visible_from)
            .is_some_and(|binding| {
                matches!(
                    &binding.value_type,
                    ValueType::List(item)
                        if matches!(
                            item.value_type_ref(),
                            ValueType::InvocationScopedNative {
                                concrete: false, ..
                            }
                        )
                )
            });
        if scoped_wildcard_list
            && let Some(function_span) = unit
                .enclosing_function_spans
                .get(&node.span.start)
                .copied()
                .flatten()
            && let Some(function) = find_node_by_span(&unit.tree.root, function_span)
        {
            let mut item_types = Vec::new();
            collect_scoped_append_types(package, unit, function, receiver_name, &mut item_types);
            if item_types.len() > 1 {
                return Err(failure(
                    &unit.source,
                    "T0119",
                    format!(
                        "invocation-scoped list `{receiver_name}` cannot mix native element types"
                    ),
                    node.span,
                ));
            }
        }
    }

    if node.kind == SyntaxKind::Assignment
        && let [target, value] = node.children.as_slice()
        && target.kind == SyntaxKind::Name
        && let Some(binding) = visible_binding(unit, target)
        && let Some(declaration) = find_node_by_span(&unit.tree.root, binding.span)
        && let Some(initializer) = declaration.children.last()
        && let Some(initial_type) = infer_value_type(unit, initializer, &unit.typed_bindings)?
        && let Some(reassigned_type) = infer_value_type(unit, value, &unit.typed_bindings)?
        && let Ok(initial_projected) =
            super::objects::destination_projected_type(package, &initial_type)
        && let Ok(reassigned_projected) =
            super::objects::destination_projected_type(package, &reassigned_type)
        && matches!(
            (&initial_projected, &reassigned_projected),
            (
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. },
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
            )
        )
        && !super::objects::same_projected_native_family(&initial_projected, &reassigned_projected)
    {
        return Err(failure(
            &unit.source,
            "T0019",
            format!(
                "cannot reassign invocation-scoped `{}` from `{}` to `{}`",
                binding.name,
                projected_native_name(&initial_projected),
                projected_native_name(&reassigned_projected)
            ),
            value.span,
        ));
    }

    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && let Some(value) = node.children.last()
        && let Some(value_type) = infer_value_type(unit, value, &unit.typed_bindings)?
        && let Some(region) = invocation_scoped_region(&value_type)
    {
        let local_span = unit
            .enclosing_function_spans
            .get(&node.span.start)
            .copied()
            .flatten();
        let local_region = local_span.map(|span| (span.file, span.start, span.end));
        let region_local_function = local_span.is_some_and(|span| {
            unit.functions.iter().any(|function| {
                function.span == span
                    && matches!(
                        function.return_type,
                        Some(ValueType::InvocationScopedNative { .. })
                    )
            })
        });
        if matches!(region, InvocationScopedRegion::Bound(region) if local_region != Some(region))
            || (region == InvocationScopedRegion::Unbound && !region_local_function)
            || region == InvocationScopedRegion::Expression
        {
            return Err(failure(
                &unit.source,
                "T0119",
                "invocation-scoped native value cannot escape into ordinary storage",
                value.span,
            ));
        }
    }
    if node.kind == SyntaxKind::AnonymousFunction
        && let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
        && let Some(capture) = contract.captures.iter().find(|capture| {
            unit.typed_bindings.iter().any(|binding| {
                binding.name == **capture
                    && binding.is_visible_at(unit.source.id(), node.span.start)
                    && invocation_scoped_region(&binding.value_type).is_some()
            })
        })
    {
        return Err(failure(
            &unit.source,
            "T0119",
            format!(
                "invocation-scoped native value `{capture}` cannot be captured outside its invocation region"
            ),
            node.span,
        ));
    }
    if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("await")
        && let Some(binding) = unit.typed_bindings.iter().find(|binding| {
            binding.is_visible_at(unit.source.id(), node.span.start)
                && invocation_scoped_region(&binding.value_type).is_some()
        })
    {
        return Err(failure(
            &unit.source,
            "T0119",
            format!(
                "invocation-scoped native value `{}` cannot remain live across suspension",
                binding.name
            ),
            node.span,
        ));
    }
    Ok(())
}

fn projected_argument_for_parameter<'a>(
    unit: &SemanticUnit,
    function: &crate::rust_interop::projection::ProjectedFunction,
    arguments: &'a SyntaxNode,
    parameter_index: usize,
) -> Option<&'a SyntaxNode> {
    let mut positional = 0;
    arguments.children.iter().find_map(|argument| {
        let named = argument
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
        let argument_index = named.map_or_else(
            || {
                let current = positional;
                positional += 1;
                current
            },
            |name| {
                function
                    .parameters
                    .iter()
                    .position(|candidate| candidate.name == node_text(&unit.source, name))
                    .unwrap_or(usize::MAX)
            },
        );
        (argument_index == parameter_index).then(|| argument.children.last().unwrap_or(argument))
    })
}

fn validate_projected_callback_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    consumed_once: &mut BTreeSet<(u32, usize, usize)>,
    extracted_channel_endpoints: &mut BTreeSet<(u32, usize, usize, String)>,
    immediately_awaited: bool,
    chain_receivers: &BTreeSet<(u32, usize, usize)>,
) -> Result<(), SemanticFailure> {
    validate_invocation_scoped_node(package, unit, node)?;
    if matches!(
        projected_chain_role(package, unit, node),
        Some(
            crate::rust_interop::projection::ChainRole::Root
                | crate::rust_interop::projection::ChainRole::Continue
        )
    ) && !chain_receivers.contains(&span_key(node.span))
    {
        return Err(failure(
            &unit.source,
            "T0112",
            "chain-only dependency value must terminate within one expression",
            node.span,
        ));
    }
    validate_channel_endpoint_extraction(unit, node, extracted_channel_endpoints)?;
    validate_projected_borrowed_async_call(package, unit, node, immediately_awaited)?;
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && let Some(function) = projected_function_for_call(package, unit, callee)
    {
        for (index, parameter) in function.parameters.iter().enumerate() {
            let Some(value) = projected_argument_for_parameter(unit, function, arguments, index)
            else {
                continue;
            };
            if matches!(
                parameter.ty,
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
            ) && let Some(actual) = infer_value_type(unit, value, &unit.typed_bindings)?
                && let Ok(actual_projected) =
                    super::objects::destination_projected_type(package, &actual)
                && matches!(
                    actual_projected,
                    crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                )
                && !super::objects::same_projected_native_family(&parameter.ty, &actual_projected)
            {
                return Err(failure(
                    &unit.source,
                    "T0019",
                    format!(
                        "incompatible argument types: expected `{}`, found `{}`",
                        projected_native_name(&parameter.ty),
                        projected_native_name(&actual_projected)
                    ),
                    value.span,
                ));
            }
            if !matches!(
                parameter.ty,
                crate::rust_interop::projection::ProjectedType::Callback { .. }
            ) {
                continue;
            }
            let Some(contract) = callback_contract(package, unit, value) else {
                continue;
            };
            let callback_parameter = unit
                .projected_call_specializations
                .get(&span_key(node.span))
                .and_then(|selection| selection.projected_parameters.get(index))
                .unwrap_or(parameter);
            validate_projected_callback_contract(
                package,
                unit,
                value,
                contract,
                &callback_parameter.ty,
                consumed_once,
            )?;
        }
    }
    for child in &node.children {
        validate_projected_callback_node(
            package,
            unit,
            child,
            consumed_once,
            extracted_channel_endpoints,
            node.kind == SyntaxKind::UnaryExpression
                && unary_operator_text(unit, node).as_deref() == Some("await")
                || node.kind == SyntaxKind::GroupExpression && immediately_awaited,
            chain_receivers,
        )?;
    }
    Ok(())
}

pub(super) fn validate_projected_callback_arguments(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        let mut chain_receivers = BTreeSet::new();
        collect_chain_receivers(package, unit, &unit.tree.root, &mut chain_receivers);
        validate_projected_callback_node(
            package,
            unit,
            &unit.tree.root,
            &mut BTreeSet::new(),
            &mut BTreeSet::new(),
            false,
            &chain_receivers,
        )?;
    }
    Ok(())
}
