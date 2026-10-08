mod async_tasks;
mod callbacks;

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    SourceFile, Span,
    syntax::{SyntaxKind, SyntaxNode},
};

use super::{
    diagnostics::{
        ClosureWrites, binding_span_has_interior_mutation, binding_span_is_mutated, failure,
        node_text, unary_operator_text,
    },
    model::{SemanticFailure, SemanticPackage, SemanticUnit, SymbolKind, TypedBinding, ValueType},
    scopes::lexical_scope_chain,
};

pub(super) use async_tasks::{
    AutoTraitObligation, infer_task_transferability, synchronize_execution_requirements,
    validate_suspension_ownership, validate_task_consumption, validate_task_transferability,
    value_type_is_owned_static, value_type_satisfies_auto_trait,
};
pub(crate) use callbacks::{callback_contract, projected_macro_for_call};
pub(super) use callbacks::{projected_function_for_call, validate_projected_callback_arguments};

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
        && let Some(function) =
            projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
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
        declaration_store: bool,
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
    if (node.kind == SyntaxKind::SelectStatement && child.kind == SyntaxKind::SelectCase)
        || (node.kind == SyntaxKind::MatchStatement
            && matches!(child.kind, SyntaxKind::MatchCase | SyntaxKind::ElseClause))
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
    if node.kind == SyntaxKind::ForTarget {
        let mut recorded = false;
        for name in &node.children {
            let identity = unit
                .flow_binding_ids
                .get(&span_key(name.span))
                .copied()
                .or_else(|| {
                    unit.typed_bindings
                        .iter()
                        .find(|binding| binding.span == name.span)
                        .map(|binding| binding.span)
                })
                .or_else(|| {
                    unit.typed_bindings
                        .iter()
                        .rev()
                        .find(|binding| {
                            binding.name == node_text(&unit.source, name)
                                && binding.scope
                                    == lexical_scope_chain(unit, name.span.start)
                                        .next()
                                        .map(|scope| scope.span)
                                && binding.is_visible_at(unit.source.id(), name.span.start)
                        })
                        .map(|binding| binding.span)
                });
            if let Some(identity) = identity {
                events
                    .entry(span_key(identity))
                    .or_default()
                    .push(BindingEvent::Write {
                        span: name.span,
                        declaration_store: true,
                        loops: loops.to_vec(),
                        regions: regions.to_vec(),
                    });
                recorded = true;
            }
        }
        return recorded;
    }
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
                declaration_store: true,
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
        record_binding_reads(
            package,
            unit,
            node,
            events,
            declaration_name,
            loops,
            regions,
        );
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
    {
        let mut record = |declaration_span| {
            events
                .entry(span_key(declaration_span))
                .or_default()
                .push(BindingEvent::Write {
                    span: node.span,
                    declaration_store: false,
                    loops: loops.clone(),
                    regions: regions.clone(),
                });
        };
        if let Some(origins) = unit.flow_read_bindings.get(&span_key(target.span)) {
            for origin in origins {
                record(*origin);
            }
        } else if let Some(declaration_span) = package
            .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
            .and_then(|symbol| symbol.declaration_span)
        {
            record(declaration_span);
        }
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
    // Interior mutation needs a mutable place even when reaching paths only
    // initialize the carrier once.
    if unit.typed_bindings.iter().any(|binding| {
        unit.flow_binding_ids
            .get(&span_key(binding.span))
            .copied()
            .unwrap_or(binding.span)
            == declaration_span
            && !matches!(
                binding.value_type,
                ValueType::Reference(_) | ValueType::SharedReference(_)
            )
            && binding_span_has_interior_mutation(package, unit, binding.span, closure_writes)
    }) {
        return true;
    }
    binding_storage_is_replaced(package, unit, declaration_span, closure_writes, false)
}

pub(crate) fn binding_storage_is_replaced(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
    closure_writes: ClosureWrites,
    declarations_only: bool,
) -> bool {
    let declaration_function = unit
        .enclosing_function_spans
        .get(&declaration_span.start)
        .copied()
        .flatten();
    let writes = unit
        .typed_bindings
        .iter()
        .filter(|binding| {
            unit.flow_binding_ids
                .get(&span_key(binding.span))
                .copied()
                .unwrap_or(binding.span)
                == declaration_span
        })
        .filter_map(|binding| package.binding_events.get(&span_key(binding.span)))
        .flatten()
        .filter_map(|event| match event {
            BindingEvent::Write {
                span,
                declaration_store,
                loops,
                regions,
            } if (!declarations_only || *declaration_store)
                && (closure_writes == ClosureWrites::Include
                    || unit
                        .enclosing_function_spans
                        .get(&span.start)
                        .copied()
                        .flatten()
                        == declaration_function) =>
            {
                Some((loops, regions))
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    writes.iter().any(|(loops, _)| !loops.is_empty())
        || (unit.flow_replacements.iter().any(|(statement, previous)| {
            unit.flow_binding_ids
                .get(&span_key(*statement))
                .copied()
                .unwrap_or(*statement)
                == declaration_span
                && unit
                    .flow_binding_ids
                    .get(&span_key(*previous))
                    .copied()
                    .unwrap_or(*previous)
                    == declaration_span
        }) && writes.iter().enumerate().any(|(index, (_, left))| {
            writes
                .iter()
                .skip(index + 1)
                .any(|(_, right)| !regions_conflict(left, right))
        }))
}

pub(super) fn later_store_replaces(earlier: &[ControlRegion], later: &[ControlRegion]) -> bool {
    later.iter().all(|region| earlier.contains(region))
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
                ..
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

fn record_binding_reads(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    events: &mut BTreeMap<(u32, usize, usize), Vec<BindingEvent>>,
    declaration_name: bool,
    loops: &[Span],
    regions: &[ControlRegion],
) {
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
    if !declaration_name {
        let mut record = |declaration_span| {
            events
                .entry(span_key(declaration_span))
                .or_default()
                .push(BindingEvent::Read {
                    span: node.span,
                    loops: loops.to_vec(),
                    regions: regions.to_vec(),
                });
        };
        if let Some(origins) = unit.flow_read_bindings.get(&span_key(node.span)) {
            for origin in origins {
                record(*origin);
            }
        } else if let Some(declaration_span) = declaration_span {
            record(declaration_span);
        }
    }
}
