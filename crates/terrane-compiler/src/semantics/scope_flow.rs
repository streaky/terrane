// Reaching-path facts, loop fixed points, and function-owned storage identities.
use super::prelude::*;
fn canonical_binding_span(unit: &SemanticUnit, name: &str, position: usize) -> Option<Span> {
    let function_span = unit
        .enclosing_function_spans
        .get(&position)
        .copied()
        .flatten()?;
    let scope = unit
        .typed_bindings
        .iter()
        .filter(|binding| binding.name == name)
        .filter_map(|binding| binding.scope)
        .filter(|scope| {
            scope.file == function_span.file
                && scope.start <= position
                && position <= scope.end
                && scope.start <= function_span.start
                && function_span.end <= scope.end
        })
        .max_by_key(|scope| scope.start)?;
    unit.typed_bindings
        .iter()
        .filter(|binding| binding.name == name && binding.scope == Some(scope))
        .min_by_key(|binding| binding.span.start)
        .map(|binding| binding.span)
}

fn record_join_binding_ids(unit: &mut SemanticUnit, node: &SyntaxNode, name: &str, identity: Span) {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        return;
    }
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && let Some(target) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
        && node_text(&unit.source, target) == name
    {
        let key = (node.span.file, node.span.start, node.span.end);
        let previous = unit.flow_binding_ids.get(&key).copied().or_else(|| {
            unit.typed_bindings
                .iter()
                .find(|binding| binding.span == node.span)
                .map(|binding| binding.span)
        });
        let written_type = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == node.span)
            .map(|binding| &binding.value_type)
            .or_else(|| unit.flow_types.get(&key));
        let storage_type = unit.flow_binding_types.get(&identity).or_else(|| {
            unit.typed_bindings
                .iter()
                .find(|binding| binding.span == identity)
                .map(|binding| &binding.value_type)
        });
        if written_type
            .zip(storage_type)
            .is_none_or(|(written, storage)| {
                written == storage
                    || matches!(storage, ValueType::Union(arms) if arms.contains(written))
            })
        {
            unit.flow_binding_ids.insert(key, identity);
            if let Some(previous) = previous.filter(|previous| *previous != identity) {
                for binding in unit.flow_binding_ids.values_mut() {
                    if *binding == previous {
                        *binding = identity;
                    }
                }
                for replaced in unit.flow_replacements.values_mut() {
                    if *replaced == previous {
                        *replaced = identity;
                    }
                }
            }
        }
    }
    if node.kind == SyntaxKind::ForTarget {
        for target in node
            .children
            .iter()
            .filter(|target| node_text(&unit.source, target) == name)
        {
            unit.flow_binding_ids.insert(
                (target.span.file, target.span.start, target.span.end),
                identity,
            );
        }
    }
    for child in &node.children {
        record_join_binding_ids(unit, child, name, identity);
    }
}
fn record_join_for_env(unit: &mut SemanticUnit, node: &SyntaxNode, env: &FlowEnv) {
    for (name, value_type) in &env.types {
        if let Some(identity) = env.binding_ids.get(name).copied()
            && (matches!(value_type, ValueType::Union(_))
                || unit.flow_binding_types.contains_key(&identity))
        {
            record_join_binding_ids(unit, node, name, identity);
        }
    }
}

pub(crate) fn constant_boolean(unit: &SemanticUnit, node: &SyntaxNode) -> Option<bool> {
    match node_text(&unit.source, node).trim() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

pub(super) fn is_flow_value_child(node: &SyntaxNode, index: usize, child: &SyntaxNode) -> bool {
    !((node.kind == SyntaxKind::Argument
        && index == 0
        && child.kind == SyntaxKind::Name
        && node.children.len() > 1)
        || (node.kind == SyntaxKind::ConstructionExpression && index == 0)
        || matches!(
            node.kind.child_field(index, child.kind),
            "member" | "target" | "type" | "variant"
        ))
}

fn record_flow_reads(unit: &mut SemanticUnit, node: &SyntaxNode, active: &FlowEnv) {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        return;
    }
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        let key = (node.span.file, node.span.start, node.span.end);
        if let Some(value_type) = active.types.get(name) {
            unit.flow_types.insert(key, value_type.clone());
        }
        if let Some(identity) = active.binding_ids.get(name) {
            unit.flow_binding_ids.insert(key, *identity);
        }
        if active.types.contains_key(name) {
            let availability = if active.uncertain.contains(name) {
                FlowAvailability::MayBeUnassigned
            } else {
                FlowAvailability::DefinitelyAssigned
            };
            unit.flow_availability.insert(key, availability);
        }
        if let Some(origins) = active.origins.get(name) {
            unit.flow_read_bindings
                .entry(key)
                .or_default()
                .extend(origins);
        }
        return;
    }
    for (index, child) in node.children.iter().enumerate() {
        if !is_flow_value_child(node, index, child) {
            continue;
        }
        record_flow_reads(unit, child, active);
    }
}

fn union_flow_types(types: impl IntoIterator<Item = ValueType>) -> Option<ValueType> {
    fn flatten(value_type: ValueType, arms: &mut Vec<ValueType>) {
        match value_type {
            ValueType::Union(nested) => {
                for arm in nested {
                    flatten(arm, arms);
                }
            }
            value_type => arms.push(value_type),
        }
    }
    let mut arms = Vec::new();
    for value_type in types {
        flatten(value_type, &mut arms);
    }
    arms.sort_by_key(|value_type| format!("{value_type:?}"));
    arms.dedup();
    if let Some(optional) = arms.iter().position(|candidate| {
        let ValueType::Optional(inner) = candidate else {
            return false;
        };
        arms.iter().all(|arm| {
            arm == candidate || arm == inner.as_ref() || *arm == ValueType::Scalar(ScalarType::None)
        })
    }) {
        return Some(arms.swap_remove(optional));
    }
    match arms.len() {
        0 => None,
        1 => arms.pop(),
        _ => Some(ValueType::Union(arms)),
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum FlowExit {
    Normal,
    Return,
    Throw,
    Break,
    Continue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FlowEnv {
    types: BTreeMap<String, ValueType>,
    binding_ids: BTreeMap<String, Span>,
    carrier_types: BTreeMap<String, ValueType>,
    uncertain: BTreeSet<String>,
    origins: BTreeMap<String, BTreeSet<Span>>,
}

impl FlowEnv {
    fn new(types: BTreeMap<String, ValueType>) -> Self {
        Self {
            types,
            binding_ids: BTreeMap::new(),
            carrier_types: BTreeMap::new(),
            uncertain: BTreeSet::new(),
            origins: BTreeMap::new(),
        }
    }
}

type FlowStates = BTreeMap<FlowExit, FlowEnv>;

fn merge_union_carrier(
    unit: &mut SemanticUnit,
    span: Span,
    envs: &[FlowEnv],
    name: &str,
    prior_carriers: Vec<ValueType>,
    ids: &BTreeSet<Span>,
    merged: &mut FlowEnv,
) {
    let identity = envs
        .iter()
        .find_map(|env| {
            env.carrier_types
                .contains_key(name)
                .then(|| env.binding_ids.get(name).copied())
                .flatten()
        })
        .or_else(|| canonical_binding_span(unit, name, span.start))
        .or_else(|| ids.iter().next().copied());
    if let Some(identity) = identity {
        merged.binding_ids.insert(name.to_owned(), identity);
        let all_types = envs
            .iter()
            .filter_map(|env| env.types.get(name).cloned())
            .chain(prior_carriers);
        if let Some(carrier_type) = union_flow_types(all_types) {
            unit.flow_binding_types
                .insert(identity, carrier_type.clone());
            merged.carrier_types.insert(name.to_owned(), carrier_type);
        }
    }
}

fn merge_flow_states(
    unit: &mut SemanticUnit,
    span: Span,
    states: impl IntoIterator<Item = FlowStates>,
) -> FlowStates {
    let mut grouped = BTreeMap::<FlowExit, Vec<FlowEnv>>::new();
    for state in states {
        for (exit, env) in state {
            grouped.entry(exit).or_default().push(env);
        }
    }
    grouped
        .into_iter()
        .map(|(exit, mut envs)| {
            if envs.len() == 1 {
                return (exit, envs.pop().unwrap());
            }
            let names = envs
                .iter()
                .flat_map(|env| {
                    env.types
                        .keys()
                        .chain(env.binding_ids.keys())
                        .chain(env.uncertain.iter())
                        .cloned()
                })
                .collect::<BTreeSet<_>>();
            let mut merged = FlowEnv::new(BTreeMap::new());
            for name in names {
                let value_type =
                    union_flow_types(envs.iter().filter_map(|env| env.types.get(&name).cloned()));
                let prior_carriers = envs
                    .iter()
                    .filter_map(|env| env.carrier_types.get(&name).cloned())
                    .collect::<Vec<_>>();
                if let Some(value_type) = value_type {
                    let ids = envs
                        .iter()
                        .filter_map(|env| env.binding_ids.get(&name).copied())
                        .collect::<BTreeSet<_>>();
                    if matches!(value_type, ValueType::Union(_)) {
                        merge_union_carrier(
                            unit,
                            span,
                            &envs,
                            &name,
                            prior_carriers,
                            &ids,
                            &mut merged,
                        );
                    } else if ids.len() == 1 && prior_carriers.is_empty() {
                        merged
                            .binding_ids
                            .insert(name.clone(), *ids.iter().next().unwrap());
                    } else if let Some(carrier) = envs.iter().find_map(|env| {
                        env.binding_ids
                            .get(&name)
                            .copied()
                            .filter(|_| env.carrier_types.contains_key(&name))
                    }) {
                        merged.binding_ids.insert(name.clone(), carrier);
                        if let Some(carrier_type) = union_flow_types(
                            prior_carriers
                                .into_iter()
                                .chain(std::iter::once(value_type.clone())),
                        ) {
                            unit.flow_binding_types
                                .insert(carrier, carrier_type.clone());
                            merged.carrier_types.insert(name.clone(), carrier_type);
                        }
                    } else if let Some(identity) = ids.iter().next().copied() {
                        merged.binding_ids.insert(name.clone(), identity);
                        unit.flow_binding_types.insert(identity, value_type.clone());
                    }
                    merged.types.insert(name.clone(), value_type);
                } else if let Some(identity) = envs
                    .iter()
                    .find_map(|env| env.binding_ids.get(&name).copied())
                {
                    merged.binding_ids.insert(name.clone(), identity);
                }
                if merged.types.contains_key(&name) || merged.binding_ids.contains_key(&name) {
                    merged.origins.insert(
                        name.clone(),
                        envs.iter()
                            .filter_map(|env| env.origins.get(&name))
                            .flatten()
                            .copied()
                            .collect(),
                    );
                }
                if envs
                    .iter()
                    .any(|env| !env.types.contains_key(&name) || env.uncertain.contains(&name))
                {
                    merged.uncertain.insert(name);
                }
            }
            (exit, merged)
        })
        .collect()
}
fn merge_into_paths(unit: &mut SemanticUnit, span: Span, paths: &mut FlowStates, next: FlowStates) {
    let previous = std::mem::take(paths);
    *paths = merge_flow_states(unit, span, [previous, next]);
}

fn analyze_nested_function_flow(unit: &mut SemanticUnit, node: &SyntaxNode, captured: &FlowEnv) {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        let mut active = captured.clone();
        if let Some(parameters) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::ParameterList)
        {
            for parameter in &parameters.children {
                if let Some(name) = parameter
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)
                {
                    let name = node_text(&unit.source, name).to_owned();
                    if let Some(binding) = unit
                        .typed_bindings
                        .iter()
                        .find(|binding| binding.span == parameter.span)
                    {
                        active
                            .types
                            .insert(name.clone(), binding.value_type.clone());
                        active.binding_ids.insert(name.clone(), binding.span);
                        active
                            .origins
                            .insert(name.clone(), BTreeSet::from([binding.span]));
                        active.uncertain.remove(&name);
                    }
                }
            }
        }
        if let Some(body) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            let _ = flow_block(unit, body, active);
        }
        return;
    }
    for child in &node.children {
        analyze_nested_function_flow(unit, child, captured);
    }
}

fn flow_block(unit: &mut SemanticUnit, block: &SyntaxNode, initial: FlowEnv) -> FlowStates {
    let mut paths = FlowStates::from([(FlowExit::Normal, initial)]);
    for statement in &block.children {
        let Some(state) = paths.remove(&FlowExit::Normal) else {
            break;
        };
        match statement.kind {
            SyntaxKind::TryStatement => {
                flow_try(unit, statement, state, &mut paths);
            }
            SyntaxKind::IfStatement => {
                flow_if(unit, statement, &state, &mut paths);
            }
            SyntaxKind::SelectStatement | SyntaxKind::MatchStatement => {
                flow_cases(unit, statement, &state, &mut paths);
            }
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement => {
                flow_loop(unit, statement, state, &mut paths);
            }
            SyntaxKind::Binding | SyntaxKind::Assignment => {
                flow_assignment(unit, statement, state, &mut paths);
            }
            SyntaxKind::ReturnStatement
            | SyntaxKind::ThrowStatement
            | SyntaxKind::BreakStatement
            | SyntaxKind::ContinueStatement => {
                record_flow_reads(unit, statement, &state);
                let exit = match statement.kind {
                    SyntaxKind::ReturnStatement => FlowExit::Return,
                    SyntaxKind::ThrowStatement => FlowExit::Throw,
                    SyntaxKind::BreakStatement => FlowExit::Break,
                    _ => FlowExit::Continue,
                };
                merge_into_paths(
                    unit,
                    statement.span,
                    &mut paths,
                    FlowStates::from([(exit, state)]),
                );
            }
            _ => {
                analyze_nested_function_flow(unit, statement, &state);
                record_flow_reads(unit, statement, &state);
                paths.insert(FlowExit::Normal, state);
            }
        }
    }
    paths
}
fn refine_condition_env(
    unit: &SemanticUnit,
    condition: &SyntaxNode,
    env: &mut FlowEnv,
    truth: bool,
) -> bool {
    let condition = ungrouped_expression(condition);
    if condition.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, condition).as_deref() == Some("not")
        && let Some(operand) = condition.children.last()
    {
        return refine_condition_env(unit, operand, env, !truth);
    }
    if truth {
        narrow_present_env(unit, condition, env);
    }
    if condition.kind != SyntaxKind::TypeMembershipExpression {
        return true;
    }
    let [subject, type_node] = condition.children.as_slice() else {
        return true;
    };
    let subject = ungrouped_expression(subject);
    if subject.kind != SyntaxKind::Name {
        return true;
    }
    let name = node_text(&unit.source, subject);
    let Some(value_type) = env.types.get(name) else {
        return true;
    };
    let aliases = visible_descriptor_aliases(
        &unit.descriptor_aliases,
        unit.source.id(),
        type_node.span.start,
    );
    let Ok(expected) = declared_value_type(unit, type_node, &aliases) else {
        return true;
    };
    let arms = match value_type {
        ValueType::Union(arms) => arms.clone(),
        ValueType::Optional(inner) => {
            vec![inner.as_ref().clone(), ValueType::Scalar(ScalarType::None)]
        }
        other => vec![other.clone()],
    };
    // A nominal/interface test may match through conformance rather than exact identity.
    // Only eliminate alternatives when the tested type is one of this finite inventory.
    if !arms.contains(&expected) {
        return true;
    }
    let Some(narrowed) =
        union_flow_types(arms.into_iter().filter(|arm| (*arm == expected) == truth))
    else {
        return false;
    };
    env.types.insert(name.to_owned(), narrowed);
    true
}

fn narrow_present_env(unit: &SemanticUnit, condition: &SyntaxNode, env: &mut FlowEnv) {
    for (name, value_type) in env.types.clone() {
        if !condition_proves_present(&unit.source, condition, &name) {
            continue;
        }
        let narrowed = match value_type {
            ValueType::Optional(inner) => Some(*inner),
            ValueType::Union(arms) => {
                union_flow_types(arms.into_iter().filter_map(|arm| match arm {
                    ValueType::Optional(inner) => Some(*inner),
                    ValueType::Scalar(ScalarType::None) => None,
                    other => Some(other),
                }))
            }
            _ => None,
        };
        if let Some(narrowed) = narrowed {
            env.types.insert(name, narrowed);
        }
    }
}

fn merge_envs(unit: &mut SemanticUnit, span: Span, envs: Vec<FlowEnv>) -> FlowEnv {
    merge_flow_states(
        unit,
        span,
        envs.into_iter()
            .map(|env| FlowStates::from([(FlowExit::Normal, env)])),
    )
    .remove(&FlowExit::Normal)
    .unwrap_or_else(|| FlowEnv::new(BTreeMap::new()))
}

pub(super) fn validate_definite_assignment(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn collect_functions<'a>(node: &'a SyntaxNode, functions: &mut Vec<&'a SyntaxNode>) {
        if matches!(
            node.kind,
            SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
        ) {
            functions.push(node);
            return;
        }
        for child in &node.children {
            collect_functions(child, functions);
        }
    }
    for unit in &mut package.units {
        unit.rust_storage_names.take();
        unit.flow_types.clear();
        unit.flow_binding_ids.clear();
        unit.flow_binding_types.clear();
        unit.flow_availability.clear();
        unit.flow_replacements.clear();
        unit.flow_read_bindings.clear();
        let mut functions = Vec::new();
        let tree = unit.tree.clone();
        collect_functions(&tree.root, &mut functions);
        for function in functions {
            let mut declared = unit
                .typed_bindings
                .iter()
                .filter(|binding| binding.scope == Some(function.span))
                .filter(|binding| !matches!(binding.name.as_str(), "self" | "this"))
                .map(|binding| binding.name.clone())
                .collect();
            let mut assigned = BTreeSet::new();
            let mut active = FlowEnv::new(BTreeMap::new());
            if let Some(parameters) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::ParameterList)
            {
                for parameter in &parameters.children {
                    if let Some(name) = parameter
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Name)
                    {
                        let name = node_text(&unit.source, name).to_owned();
                        assigned.insert(name.clone());
                        if let Some(binding) = unit
                            .typed_bindings
                            .iter()
                            .find(|binding| binding.span == parameter.span)
                        {
                            active
                                .types
                                .insert(name.clone(), binding.value_type.clone());
                            active.binding_ids.insert(name.clone(), binding.span);
                            active
                                .origins
                                .insert(name.clone(), BTreeSet::from([binding.span]));
                        }
                    }
                }
            }
            if let Some(block) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            {
                flow_block(unit, block, active);
                validate_assignment_block(unit, block, &mut declared, &mut assigned)?;
            }
        }
    }
    Ok(())
}
fn require_completion_storage(unit: &mut SemanticUnit, statement: &SyntaxNode, state: &FlowEnv) {
    // Completion closures capture function-owned slots before their body runs.
    // Slots created inside that boundary need an empty state even when every
    // source read is proven available; ordinary Rust locals retain drop flags.
    for binding in &unit.typed_bindings {
        if binding.name == "_"
            || binding.span.file != statement.span.file
            || binding.span.start < statement.span.start
            || binding.span.end > statement.span.end
            || !binding.scope.is_some_and(|owner| {
                owner.start <= statement.span.start && statement.span.end <= owner.end
            })
        {
            continue;
        }
        let key = span_key(binding.span);
        let identity = unit
            .flow_binding_ids
            .get(&key)
            .copied()
            .unwrap_or(binding.span);
        let incoming = state.binding_ids.get(&binding.name).map(|origin| {
            unit.flow_binding_ids
                .get(&span_key(*origin))
                .copied()
                .unwrap_or(*origin)
        });
        if incoming != Some(identity) {
            unit.flow_binding_ids.entry(key).or_insert(identity);
            unit.flow_availability
                .insert(key, FlowAvailability::MayBeUnassigned);
        }
    }
}

fn flow_try(
    unit: &mut SemanticUnit,
    statement: &SyntaxNode,
    state: FlowEnv,
    paths: &mut FlowStates,
) {
    let Some(try_block) = statement.children.first() else {
        merge_into_paths(
            unit,
            statement.span,
            paths,
            FlowStates::from([(FlowExit::Normal, state)]),
        );
        return;
    };
    let mut try_paths = flow_block(unit, try_block, state.clone());
    require_completion_storage(unit, statement, &state);
    let throw_state = try_paths.remove(&FlowExit::Throw);
    let mut outcomes = try_paths
        .into_iter()
        .map(|(exit, env)| FlowStates::from([(exit, env)]))
        .collect::<Vec<_>>();
    for clause in statement
        .children
        .iter()
        .filter(|child| child.kind == SyntaxKind::CatchClause)
    {
        let Some(body) = clause
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        else {
            continue;
        };
        let mut caught = throw_state.clone().unwrap_or_else(|| state.clone());
        if let Some(alias) = clause
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::CatchBinding)
            && let Some(binding) = unit
                .typed_bindings
                .iter()
                .find(|binding| binding.span == alias.span)
        {
            // Catch storage exists only on the error path, even when all reads
            // occur within that path and are proven available there.
            unit.flow_binding_ids
                .insert(span_key(alias.span), binding.span);
            unit.flow_availability
                .insert(span_key(alias.span), FlowAvailability::MayBeUnassigned);
            caught
                .types
                .insert(binding.name.clone(), binding.value_type.clone());
            caught
                .binding_ids
                .insert(binding.name.clone(), binding.span);
            caught
                .origins
                .insert(binding.name.clone(), BTreeSet::from([binding.span]));
            caught.uncertain.remove(&binding.name);
        }
        outcomes.push(flow_block(unit, body, caught));
    }
    if let Some(finally) = statement
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::FinallyClause)
        .and_then(|clause| clause.children.first())
    {
        let mut finalized = Vec::new();
        for outcome in outcomes {
            for (exit, env) in outcome {
                for (final_exit, final_env) in flow_block(unit, finally, env) {
                    let actual_exit = if final_exit == FlowExit::Normal {
                        exit
                    } else {
                        final_exit
                    };
                    finalized.push(FlowStates::from([(actual_exit, final_env)]));
                }
            }
        }
        outcomes = finalized;
    }
    let joined = merge_flow_states(unit, statement.span, outcomes);
    if let Some(env) = joined.get(&FlowExit::Normal) {
        record_join_for_env(unit, statement, env);
    }
    merge_into_paths(unit, statement.span, paths, joined);
}

fn flow_if(
    unit: &mut SemanticUnit,
    statement: &SyntaxNode,
    state: &FlowEnv,
    paths: &mut FlowStates,
) {
    let mut outcomes = Vec::new();
    let mut clauses = Vec::new();
    for clause in statement.children.iter().skip(1) {
        if clause.kind == SyntaxKind::Block {
            clauses.push((statement.children.first(), clause));
        } else if clause.kind == SyntaxKind::ElseClause {
            let condition = clause
                .children
                .iter()
                .find(|child| child.kind != SyntaxKind::Block);
            let body = clause
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block);
            if let Some(body) = body {
                clauses.push((condition, body));
            }
        } else if let Some(body) = clause
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            clauses.push((clause.children.first(), body));
        }
    }
    let mut falls_through = true;
    let mut remaining = state.clone();
    for (condition, body) in clauses {
        if !falls_through {
            break;
        }
        if let Some(condition) = condition {
            record_flow_reads(unit, condition, &remaining);
            match constant_boolean(unit, condition) {
                Some(false) => {}
                Some(true) => {
                    let mut branch = remaining.clone();
                    if !refine_condition_env(unit, condition, &mut branch, true) {
                        continue;
                    }
                    outcomes.push(flow_block(unit, body, branch));
                    falls_through = false;
                }
                None => {
                    let mut branch = remaining.clone();
                    if refine_condition_env(unit, condition, &mut branch, true) {
                        outcomes.push(flow_block(unit, body, branch));
                    }
                    if !refine_condition_env(unit, condition, &mut remaining, false) {
                        falls_through = false;
                    }
                }
            }
        } else {
            outcomes.push(flow_block(unit, body, remaining.clone()));
            falls_through = false;
        }
    }
    if falls_through {
        outcomes.push(FlowStates::from([(FlowExit::Normal, remaining)]));
    }
    let joined = merge_flow_states(unit, statement.span, outcomes);
    if let Some(env) = joined.get(&FlowExit::Normal) {
        record_join_for_env(unit, statement, env);
    }
    merge_into_paths(unit, statement.span, paths, joined);
}

fn flow_cases(
    unit: &mut SemanticUnit,
    statement: &SyntaxNode,
    state: &FlowEnv,
    paths: &mut FlowStates,
) {
    record_flow_reads(unit, statement.children.first().unwrap_or(statement), state);
    // Valid enum matches cover every variant; enum validation rejects
    // missing coverage (including guarded-only arms).
    let mut cases = Vec::new();
    for case in &statement.children {
        if !matches!(
            case.kind,
            SyntaxKind::SelectCase | SyntaxKind::MatchCase | SyntaxKind::ElseClause
        ) {
            continue;
        }
        let header = case
            .children
            .iter()
            .find(|child| child.kind != SyntaxKind::Block);
        let body = case
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
            .or_else(|| {
                case.children
                    .last()
                    .filter(|child| child.kind == SyntaxKind::Block)
            });
        let Some(body) = body else { continue };
        let mut arm = state.clone();
        if let Some(header) = header {
            if header.kind == SyntaxKind::Binding {
                if let Some(value) = header.children.iter().rev().find(|child| {
                    child.kind != SyntaxKind::Name && child.kind != SyntaxKind::TypeExpression
                }) {
                    record_flow_reads(unit, value, state);
                }
                if case.kind == SyntaxKind::SelectCase
                    && let Some(binding) = unit
                        .typed_bindings
                        .iter()
                        .find(|binding| binding.span == header.span)
                {
                    let identity = state
                        .binding_ids
                        .get(&binding.name)
                        .copied()
                        .unwrap_or(binding.span);
                    if let Some(previous) = state.binding_ids.get(&binding.name).copied() {
                        unit.flow_replacements.insert(header.span, previous);
                    }
                    unit.flow_binding_ids.insert(
                        (header.span.file, header.span.start, header.span.end),
                        identity,
                    );
                    arm.types
                        .insert(binding.name.clone(), binding.value_type.clone());
                    arm.uncertain.remove(&binding.name);
                    arm.binding_ids.insert(binding.name.clone(), identity);
                    arm.origins
                        .insert(binding.name.clone(), BTreeSet::from([identity]));
                }
            } else {
                record_flow_reads(unit, header, state);
            }
        }
        if case.kind == SyntaxKind::MatchCase
            && let Some(parameters) = case
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::ParameterList)
        {
            for parameter in &parameters.children {
                if let Some(binding) = unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == parameter.span)
                {
                    arm.types
                        .insert(binding.name.clone(), binding.value_type.clone());
                    arm.binding_ids.insert(binding.name.clone(), binding.span);
                    arm.origins
                        .insert(binding.name.clone(), BTreeSet::from([binding.span]));
                    arm.uncertain.remove(&binding.name);
                }
            }
        }
        cases.push(flow_block(unit, body, arm));
    }
    let joined = merge_flow_states(unit, statement.span, cases);
    if let Some(env) = joined.get(&FlowExit::Normal) {
        record_join_for_env(unit, statement, env);
    }
    merge_into_paths(unit, statement.span, paths, joined);
}

fn record_loop_header_reads(unit: &mut SemanticUnit, statement: &SyntaxNode, state: &FlowEnv) {
    for expression in statement.children.iter().filter(|child| {
        !matches!(
            child.kind,
            SyntaxKind::Block
                | SyntaxKind::ForTarget
                | SyntaxKind::Binding
                | SyntaxKind::Assignment
                | SyntaxKind::PostfixExpression
        )
    }) {
        record_flow_reads(unit, expression, state);
    }
}

fn loop_entry(
    unit: &mut SemanticUnit,
    statement: &SyntaxNode,
    mut state: FlowEnv,
    paths: &mut FlowStates,
) -> Option<(FlowEnv, bool)> {
    let initializer = statement
        .children
        .iter()
        .find(|child| matches!(child.kind, SyntaxKind::Binding | SyntaxKind::Assignment));
    let has_initializer = initializer.is_some();
    if statement.kind == SyntaxKind::ForStatement
        && let Some(initializer) = initializer
    {
        let mut entry_paths = FlowStates::new();
        flow_assignment(unit, initializer, state, &mut entry_paths);
        let Some(normal) = entry_paths.remove(&FlowExit::Normal) else {
            merge_into_paths(unit, statement.span, paths, entry_paths);
            return None;
        };
        state = normal;
        merge_into_paths(unit, statement.span, paths, entry_paths);
    }
    Some((state, has_initializer))
}

fn flow_loop(
    unit: &mut SemanticUnit,
    statement: &SyntaxNode,
    state: FlowEnv,
    paths: &mut FlowStates,
) {
    let Some((state, has_initializer)) = loop_entry(unit, statement, state, paths) else {
        return;
    };
    let mut head = state.clone();
    let mut break_states = Vec::new();
    let body = statement
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Block);
    let target = statement
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::ForTarget);
    let mut exits = Vec::new();
    if statement.kind == SyntaxKind::ForStatement {
        record_loop_header_reads(unit, statement, &state);
    }
    let condition = if statement.kind == SyntaxKind::WhileStatement {
        statement.children.first()
    } else if has_initializer {
        statement.children.get(1)
    } else {
        None
    };
    if condition.is_some_and(|condition| constant_boolean(unit, condition) == Some(false)) {
        record_loop_header_reads(unit, statement, &state);
        merge_into_paths(
            unit,
            statement.span,
            paths,
            FlowStates::from([(FlowExit::Normal, state)]),
        );
        return;
    }
    loop {
        if statement.kind == SyntaxKind::WhileStatement || has_initializer {
            record_loop_header_reads(unit, statement, &head);
        }
        let mut body_entry = head.clone();
        if let Some(target) = target {
            flow_iteration_entry(unit, target, &mut body_entry);
        }
        let result = if let Some(body) = body {
            flow_block(unit, body, body_entry)
        } else {
            FlowStates::from([(FlowExit::Normal, body_entry)])
        };
        if let Some(break_state) = result.get(&FlowExit::Break) {
            break_states.push(break_state.clone());
        }
        exits.extend(
            result
                .iter()
                .filter(|(exit, _)| matches!(exit, FlowExit::Return | FlowExit::Throw))
                .map(|(exit, env)| FlowStates::from([(*exit, env.clone())])),
        );
        let backedge = result
            .iter()
            .filter(|(exit, _)| matches!(exit, FlowExit::Normal | FlowExit::Continue))
            .map(|(_, env)| env.clone())
            .collect::<Vec<_>>();
        for env in &backedge {
            for update in statement
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::PostfixExpression)
            {
                record_flow_reads(unit, update, env);
            }
        }
        if backedge.is_empty() {
            break;
        }
        // The ordinary reaching join promotes every assigned back-edge name,
        // preserving its canonical identity and entry-path uncertainty. Break,
        // return, and nested-function assignments cannot enter this loop head.
        let mut next = vec![state.clone()];
        next.extend(backedge);
        let joined = merge_envs(unit, statement.span, next);
        if joined == head {
            break;
        }
        head = joined;
    }
    let mut after = vec![state];
    after.push(head);
    after.extend(break_states);
    exits.push(FlowStates::from([(
        FlowExit::Normal,
        merge_envs(unit, statement.span, after),
    )]));
    let joined = merge_flow_states(unit, statement.span, exits);
    if let Some(env) = joined.get(&FlowExit::Normal) {
        record_join_for_env(unit, statement, env);
    }
    merge_into_paths(unit, statement.span, paths, joined);
}

fn assignment_destination_type(
    unit: &SemanticUnit,
    state: &FlowEnv,
    name: &str,
) -> Option<ValueType> {
    let identity = state.binding_ids.get(name)?;
    let declared = unit.flow_binding_types.get(identity).or_else(|| {
        unit.typed_bindings
            .iter()
            .find(|binding| binding.span == *identity)
            .map(|binding| &binding.value_type)
    })?;
    if matches!(declared, ValueType::Optional(_)) {
        Some(declared.clone())
    } else {
        state
            .types
            .get(name)
            .cloned()
            .or_else(|| Some(declared.clone()))
    }
}

fn assignment_flow_value(
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    target: &SyntaxNode,
    value: Option<&SyntaxNode>,
    state: &FlowEnv,
) -> (String, bool, Option<ValueType>) {
    let name = node_text(&unit.source, target).to_owned();
    let plain_assignment = statement.kind == SyntaxKind::Assignment
        || (state.binding_ids.contains_key(&name)
            && !statement
                .children
                .iter()
                .any(|child| child.kind == SyntaxKind::TypeExpression));
    let value_type = value.and_then(|value| {
        plain_assignment
            .then(|| assignment_destination_type(unit, state, &name))
            .flatten()
            .or_else(|| {
                unit.typed_bindings
                    .iter()
                    .find(|binding| binding.span == statement.span)
                    .map(|binding| binding.value_type.clone())
            })
            .or_else(|| {
                infer_value_type(unit, value, &unit.typed_bindings)
                    .ok()
                    .flatten()
            })
    });
    (name, plain_assignment, value_type)
}

fn flow_assignment(
    unit: &mut SemanticUnit,
    statement: &SyntaxNode,
    state: FlowEnv,
    paths: &mut FlowStates,
) {
    let mut outcomes = Vec::new();
    analyze_nested_function_flow(unit, statement, &state);
    let target = if statement.kind == SyntaxKind::Binding {
        statement
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
    } else {
        statement
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name)
    };
    let value = if statement.kind == SyntaxKind::Binding {
        super::ownership::binding_initializer(statement)
    } else {
        statement.children.get(1)
    };
    if let Some(value) = value {
        record_flow_reads(unit, value, &state);
    }
    if let Some(target) = target.filter(|target| node_text(&unit.source, target) != "_") {
        let (name, plain_assignment, value_type) =
            assignment_flow_value(unit, statement, target, value, &state);
        if let Some(previous) = state.binding_ids.get(&name).copied() {
            unit.flow_replacements.insert(statement.span, previous);
        }
        let mut next = state;
        if let Some(value_type) = value_type {
            if let Some(carrier) = next
                .binding_ids
                .get(&name)
                .copied()
                .filter(|_| next.carrier_types.contains_key(&name))
            {
                let accumulated =
                    union_flow_types([next.carrier_types[&name].clone(), value_type.clone()])
                        .unwrap_or(value_type.clone());
                next.carrier_types.insert(name.clone(), accumulated.clone());
                unit.flow_binding_types.insert(carrier, accumulated);
            } else if (!plain_assignment
                && next.binding_ids.get(&name).is_none_or(|identity| {
                    next.types
                        .get(&name)
                        .or_else(|| unit.flow_binding_types.get(identity))
                        != Some(&value_type)
                })
                || !next.binding_ids.contains_key(&name))
                && let Some(binding) = unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == statement.span)
            {
                next.binding_ids.insert(name.clone(), binding.span);
                next.origins
                    .insert(name.clone(), BTreeSet::from([binding.span]));
                unit.flow_binding_types
                    .insert(binding.span, value_type.clone());
            }
            if !next.binding_ids.contains_key(&name)
                && let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
                    binding.name == name
                        && binding.value_type == value_type
                        && binding.is_visible_at(unit.source.id(), target.span.start)
                })
            {
                next.binding_ids.insert(name.clone(), binding.span);
                next.origins
                    .insert(name.clone(), BTreeSet::from([binding.span]));
            }
            record_assignment_origin(unit, statement, target, &name, &mut next);
            unit.flow_types.insert(
                (
                    statement.span.file,
                    statement.span.start,
                    statement.span.end,
                ),
                value_type.clone(),
            );
            next.types.insert(name.clone(), value_type);
            next.uncertain.remove(&name);
        } else if value.is_none()
            && let Some(binding) = unit
                .typed_bindings
                .iter()
                .find(|binding| binding.span == statement.span)
        {
            next.binding_ids.insert(name.clone(), binding.span);
            next.origins
                .insert(name.clone(), BTreeSet::from([binding.span]));
            unit.flow_binding_types
                .insert(binding.span, binding.value_type.clone());
            record_assignment_origin(unit, statement, target, &name, &mut next);
        }
        outcomes.push(FlowStates::from([(FlowExit::Normal, next)]));
    } else {
        outcomes.push(FlowStates::from([(FlowExit::Normal, state)]));
    }
    let joined = merge_flow_states(unit, statement.span, outcomes);
    merge_into_paths(unit, statement.span, paths, joined);
}

fn flow_iteration_entry(unit: &mut SemanticUnit, target: &SyntaxNode, body_entry: &mut FlowEnv) {
    for name in &target.children {
        let name_text = node_text(&unit.source, name).to_owned();
        if name_text == "_" {
            continue;
        }
        if let Some(binding) = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == name.span)
        {
            body_entry
                .types
                .insert(name_text.clone(), binding.value_type.clone());
            if let Some(carrier) = body_entry
                .binding_ids
                .get(&name_text)
                .copied()
                .filter(|_| body_entry.carrier_types.contains_key(&name_text))
            {
                let accumulated = union_flow_types([
                    body_entry.carrier_types[&name_text].clone(),
                    binding.value_type.clone(),
                ])
                .unwrap_or(binding.value_type.clone());
                body_entry
                    .carrier_types
                    .insert(name_text.clone(), accumulated.clone());
                unit.flow_binding_types.insert(carrier, accumulated);
            } else {
                body_entry
                    .binding_ids
                    .insert(name_text.clone(), binding.span);
                body_entry
                    .origins
                    .insert(name_text.clone(), BTreeSet::from([binding.span]));
            }
        }
        if let Some(identity) = body_entry.binding_ids.get(&name_text).copied() {
            unit.flow_binding_ids
                .insert((name.span.file, name.span.start, name.span.end), identity);
            body_entry.uncertain.remove(&name_text);
        }
    }
}

fn record_assignment_origin(
    unit: &mut SemanticUnit,
    statement: &SyntaxNode,
    target: &SyntaxNode,
    name: &str,
    next: &mut FlowEnv,
) {
    if let Some(identity) = next.binding_ids.get(name).copied() {
        if next.uncertain.contains(name)
            && let Some(previous) = unit.flow_replacements.get(&statement.span).copied()
            && previous != identity
        {
            unit.flow_availability.insert(
                (previous.file, previous.start, previous.end),
                FlowAvailability::MayBeUnassigned,
            );
        }
        unit.flow_binding_ids.insert(
            (
                statement.span.file,
                statement.span.start,
                statement.span.end,
            ),
            identity,
        );
        unit.flow_binding_ids.insert(
            (target.span.file, target.span.start, target.span.end),
            identity,
        );
    }
    let origins = unit
        .typed_bindings
        .iter()
        .find(|binding| binding.span == statement.span)
        .map(|binding| BTreeSet::from([binding.span]))
        .or_else(|| next.origins.get(name).cloned());
    if let Some(origins) = origins {
        unit.flow_read_bindings
            .entry((target.span.file, target.span.start, target.span.end))
            .or_default()
            .extend(&origins);
        next.origins.insert(name.to_owned(), origins);
    }
}
