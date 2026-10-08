use super::prelude::*;

pub(super) struct LexicalScopeContext<'a> {
    namespaces: &'a BTreeMap<String, Namespace>,
    globals: &'a BTreeMap<String, Symbol>,
    prelude_bindings: &'a BTreeMap<String, Symbol>,
}

pub(super) fn collect_lexical_scopes(
    unit: &SemanticUnit,
    namespaces: &BTreeMap<String, Namespace>,
    globals: &BTreeMap<String, Symbol>,
    prelude_bindings: &BTreeMap<String, Symbol>,
) -> Result<Vec<LexicalScope>, SemanticFailure> {
    let mut scopes = Vec::new();
    let context = &LexicalScopeContext {
        namespaces,
        globals,
        prelude_bindings,
    };
    for node in &unit.tree.root.children {
        match node.kind {
            SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration => {
                if let Some(block) = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)
                {
                    for method in block
                        .children
                        .iter()
                        .filter(|child| child.kind == SyntaxKind::FunctionDeclaration)
                    {
                        add_lexical_scope(unit, context, &mut scopes, method, None)?;
                    }
                }
            }
            _ if is_function_node(node) => {
                add_lexical_scope(unit, context, &mut scopes, node, None)?;
            }
            // Top-level source blocks are not independent lexical environments.
            SyntaxKind::Block => {
                for child in &node.children {
                    if is_function_node(child) {
                        add_lexical_scope(unit, context, &mut scopes, child, None)?;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(scopes)
}

pub(super) fn add_lexical_scope(
    unit: &SemanticUnit,
    context: &LexicalScopeContext<'_>,
    scopes: &mut Vec<LexicalScope>,
    node: &SyntaxNode,
    parent: Option<usize>,
) -> Result<usize, SemanticFailure> {
    let namespaces = context.namespaces;
    let index = scopes.len();
    scopes.push(LexicalScope {
        span: node.span,
        parent,
        symbols: BTreeMap::new(),
        import_warnings: Vec::new(),
    });
    if parent.is_none() {
        let mut namespace_paths = namespace_chain(&unit.namespace).collect::<Vec<_>>();
        namespace_paths.reverse();
        for path in namespace_paths {
            let Some(namespace) = namespaces.get(&path) else {
                continue;
            };
            for (name, symbol) in &namespace.symbols {
                if visible_from(symbol, &unit.namespace) && symbol.available_in_function_body() {
                    scopes[index]
                        .symbols
                        .entry(name.clone())
                        .or_default()
                        .push(symbol.clone());
                }
            }
        }
    }
    if parent.is_none() && object_name_containing(unit, node.span).is_some() {
        insert_local(
            unit,
            scopes,
            index,
            "self".to_owned(),
            implicit_receiver_span(node, "self"),
        )?;
        let is_static = node.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == "static"
        });
        if !is_static {
            insert_local(
                unit,
                scopes,
                index,
                "this".to_owned(),
                implicit_receiver_span(node, "this"),
            )?;
        }
    }
    if let Some(parameters) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::ParameterList)
    {
        for parameter in &parameters.children {
            if let Some(name) = declaration_name(parameter, &unit.source) {
                insert_local(unit, scopes, index, name, parameter.span)?;
            }
        }
    }
    for child in &node.children {
        match child.kind {
            SyntaxKind::ParameterList => {}
            SyntaxKind::Block => {
                populate_scope(unit, context, scopes, index, child)?;
            }
            _ => {
                populate_node(unit, context, scopes, index, child)?;
            }
        }
    }
    Ok(index)
}

pub(super) fn populate_scope(
    unit: &SemanticUnit,
    context: &LexicalScopeContext<'_>,
    scopes: &mut Vec<LexicalScope>,
    index: usize,
    block: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    // Local functions are block declarations, not statements with source-order
    // visibility. Install all headers before resolving any body so sibling
    // calls, including mutual recursion, see the same canonical symbols.
    for node in &block.children {
        if node.kind == SyntaxKind::FunctionDeclaration {
            populate_local_function(unit, scopes, index, node)?;
        }
    }
    for node in &block.children {
        if node.kind != SyntaxKind::FunctionDeclaration {
            populate_node(unit, context, scopes, index, node)?;
        }
    }
    for node in &block.children {
        if node.kind == SyntaxKind::FunctionDeclaration {
            add_lexical_scope(unit, context, scopes, node, Some(index))?;
        }
    }
    Ok(())
}

fn populate_local_function(
    unit: &SemanticUnit,
    scopes: &mut [LexicalScope],
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let Some(name) = declaration_name(node, &unit.source) else {
        return Ok(());
    };
    let scope = &scopes[index];
    if scope.symbols.get(&name).is_some_and(|symbols| {
        symbols.iter().any(|symbol| {
            symbol.declaration_span.is_some_and(|span| {
                span.file == scope.span.file
                    && scope.span.start <= span.start
                    && span.end <= scope.span.end
            })
        })
    }) {
        return Err(failure(
            &unit.source,
            "S2012",
            format!("duplicate binding `{name}` in the same lexical scope"),
            node.span,
        ));
    }
    insert_local_replacement(unit, scopes, index, name.clone(), node.span);
    let symbol = scopes[index]
        .symbols
        .get_mut(&name)
        .unwrap()
        .last_mut()
        .unwrap();
    symbol.kind = SymbolKind::Function;
    symbol.constant = true;
    // Retain declaration provenance; no binding-position gate applies to
    // block-visible named declarations.
    symbol.binding_span = None;
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "lexical scope construction handles each syntax-owned scope in one traversal"
)]
pub(super) fn populate_node(
    unit: &SemanticUnit,
    context: &LexicalScopeContext<'_>,
    scopes: &mut Vec<LexicalScope>,
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let namespaces = context.namespaces;
    let globals = context.globals;
    let prelude_bindings = context.prelude_bindings;
    match node.kind {
        SyntaxKind::Binding => {
            populate_binding(unit, scopes, index, node)?;
            for child in &node.children {
                if child.kind == SyntaxKind::AnonymousFunction {
                    populate_node(unit, context, scopes, index, child)?;
                }
            }
        }
        SyntaxKind::Assignment => {
            populate_assignment(unit, namespaces, globals, scopes, index, node)?;
            for child in &node.children {
                if child.kind == SyntaxKind::AnonymousFunction {
                    populate_node(unit, context, scopes, index, child)?;
                }
            }
        }

        SyntaxKind::ImportDeclaration => {
            populate_imports(
                unit,
                namespaces,
                globals,
                prelude_bindings,
                scopes,
                index,
                node,
            )?;
        }
        SyntaxKind::FunctionDeclaration => {
            // populate_scope predeclares every sibling in this block.
            add_lexical_scope(unit, context, scopes, node, Some(index))?;
        }
        SyntaxKind::AnonymousFunction => {
            add_lexical_scope(unit, context, scopes, node, Some(index))?;
        }
        SyntaxKind::Block => populate_scope(unit, context, scopes, index, node)?,
        SyntaxKind::ForStatement => {
            if let Some(target) = node
                .children
                .first()
                .filter(|child| child.kind == SyntaxKind::ForTarget)
            {
                for name_node in &target.children {
                    let name = node_text(&unit.source, name_node).to_owned();
                    if !scopes[index].symbols.contains_key(&name) {
                        insert_local(unit, scopes, index, name, name_node.span)?;
                    } else if visible_local_symbol(scopes, index, &name)
                        .is_some_and(|symbol| symbol.kind == SymbolKind::Function)
                    {
                        return Err(failure(
                            &unit.source,
                            "S2012",
                            format!("duplicate binding `{name}` in the same lexical scope"),
                            name_node.span,
                        ));
                    }
                }
            }
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    populate_scope(unit, context, scopes, index, child)?;
                } else if child.kind != SyntaxKind::ForTarget {
                    populate_node(unit, context, scopes, index, child)?;
                }
            }
        }
        SyntaxKind::SelectCase | SyntaxKind::MatchCase => {
            if node.kind == SyntaxKind::SelectCase
                && let Some(binding) = node
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::Binding)
                && let Some(name) = declaration_name(binding, &unit.source)
                && !scopes[index].symbols.contains_key(&name)
            {
                insert_local(unit, scopes, index, name, binding.span)?;
            }
            if node.kind == SyntaxKind::MatchCase
                && let Some(parameters) = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::ParameterList)
            {
                for parameter in &parameters.children {
                    let name = node_text(&unit.source, parameter).to_owned();
                    if name != "_" && !scopes[index].symbols.contains_key(&name) {
                        insert_local(unit, scopes, index, name, parameter.span)?;
                    }
                }
            }
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    populate_scope(unit, context, scopes, index, child)?;
                } else if child.kind != SyntaxKind::Binding {
                    populate_node(unit, context, scopes, index, child)?;
                }
            }
        }
        SyntaxKind::CatchClause => {
            if let Some(alias) = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::CatchBinding)
            {
                let name = node_text(&unit.source, alias).to_owned();
                if !scopes[index].symbols.contains_key(&name) {
                    insert_local(unit, scopes, index, name, alias.span)?;
                }
            }
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    populate_scope(unit, context, scopes, index, child)?;
                }
            }
        }
        SyntaxKind::ElseClause | SyntaxKind::FinallyClause => {
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    populate_scope(unit, context, scopes, index, child)?;
                }
            }
        }
        _ => {
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    populate_scope(unit, context, scopes, index, child)?;
                } else if matches!(
                    child.kind,
                    SyntaxKind::MatchCase
                        | SyntaxKind::AnonymousFunction
                        | SyntaxKind::ElseClause
                        | SyntaxKind::CatchClause
                        | SyntaxKind::FinallyClause
                        | SyntaxKind::SelectCase
                        | SyntaxKind::ForStatement
                ) {
                    populate_node(unit, context, scopes, index, child)?;
                }
            }
        }
    }
    Ok(())
}

pub(super) fn populate_binding(
    unit: &SemanticUnit,
    scopes: &mut [LexicalScope],
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let Some(declaration) = declaration_from_syntax(unit, node) else {
        return Ok(());
    };
    if declaration.name == "_" {
        return Ok(());
    }
    if declaration.global {
        return Ok(());
    }
    let typed_replacement = node
        .children
        .iter()
        .any(|child| child.kind == SyntaxKind::TypeExpression)
        && scopes[index].symbols.contains_key(&declaration.name);
    if typed_replacement
        && visible_local_symbol(scopes, index, &declaration.name).is_some_and(|symbol| {
            symbol.kind == SymbolKind::Function
                && symbol.declaration_span.is_some_and(|span| {
                    let scope = &scopes[index];
                    span.file == scope.span.file
                        && scope.span.start < span.start
                        && span.end <= scope.span.end
                })
        })
    {
        return Err(failure(
            &unit.source,
            "S2012",
            format!(
                "duplicate binding `{}` in the same lexical scope",
                declaration.name
            ),
            node.span,
        ));
    }
    if typed_replacement {
        insert_local_replacement(unit, scopes, index, declaration.name, node.span);
        Ok(())
    } else {
        insert_local(unit, scopes, index, declaration.name, node.span)
    }
}

fn is_lexically_owned_function(symbol: &Symbol, scopes: &[LexicalScope], index: usize) -> bool {
    if symbol.kind != SymbolKind::Function {
        return false;
    }
    let Some(origin) = symbol.declaration_span else {
        return false;
    };
    let mut current = Some(index);
    while let Some(owner) = current {
        let scope = &scopes[owner];
        if origin.file == scope.span.file
            && scope.span.start < origin.start
            && origin.end <= scope.span.end
        {
            return true;
        }
        current = scope.parent;
    }
    false
}

pub(super) fn populate_assignment(
    unit: &SemanticUnit,
    namespaces: &BTreeMap<String, Namespace>,
    globals: &BTreeMap<String, Symbol>,
    scopes: &mut [LexicalScope],
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let Some(declaration) = declaration_from_syntax(unit, node) else {
        return Ok(());
    };
    if declaration.name == "_" {
        return Ok(());
    }
    let typed_declaration = node
        .children
        .iter()
        .any(|child| child.kind == SyntaxKind::TypeExpression);
    if declaration.global {
        return Ok(());
    }
    if typed_declaration {
        insert_local_replacement(unit, scopes, index, declaration.name, node.span);
        return Ok(());
    }
    if local_binding_exists(scopes, index, &declaration.name) {
        if node
            .children
            .first()
            .is_some_and(|child| child.kind == SyntaxKind::Name)
            && visible_local_symbol(scopes, index, &declaration.name)
                .is_some_and(|symbol| is_lexically_owned_function(symbol, scopes, index))
        {
            let name = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .expect("ordinary assignment has a name");
            return Err(failure(
                &unit.source,
                "S2022",
                format!("function `{}` cannot be reassigned", declaration.name),
                name.span,
            ));
        }
        return Ok(());
    }
    let namespace_binding = globals
        .get(&declaration.name)
        .filter(|symbol| symbol.kind == SymbolKind::Binding)
        .or_else(|| {
            namespace_chain(&unit.namespace).find_map(|path| {
                namespaces
                    .get(&path)
                    .and_then(|scope| scope.symbols.get(&declaration.name))
                    .filter(|symbol| symbol.kind == SymbolKind::Binding)
            })
        });
    if let Some(symbol) = namespace_binding {
        let name = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .expect("ordinary assignment has a name");
        if symbol
            .declaration_span
            .is_some_and(|span| declaration_is_constant_in_unit(unit, span))
        {
            return Err(failure(
                &unit.source,
                "S2022",
                format!(
                    "constant binding `{}` cannot be reassigned",
                    declaration.name
                ),
                name.span,
            ));
        }
        return Err(SemanticFailure {
            source: unit.source.clone(),
            diagnostics: vec![
                Diagnostic::error(
                    "S2021",
                    format!(
                        "plain assignment cannot replace namespace binding `{}`",
                        declaration.name
                    ),
                    name.span,
                )
                .with_help(format!(
                    "pass `{}` as a parameter and return changes, or declare it `constant` if it never varies",
                    declaration.name
                )),
            ],
        });
    }
    insert_local(unit, scopes, index, declaration.name, node.span)?;
    Ok(())
}

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

fn is_flow_value_child(node: &SyntaxNode, index: usize, child: &SyntaxNode) -> bool {
    !((node.kind == SyntaxKind::Argument
        && index == 0
        && child.kind == SyntaxKind::Name
        && node.children.len() > 1)
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
        if active.uncertain.contains(name) {
            unit.flow_availability
                .insert(key, FlowAvailability::MayBeUnassigned);
        } else if active.types.contains_key(name) {
            unit.flow_availability
                .insert(key, FlowAvailability::DefinitelyAssigned);
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
        .map(|(exit, envs)| {
            let names = envs
                .iter()
                .flat_map(|env| env.types.keys().chain(env.uncertain.iter()).cloned())
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
                        let identity = envs
                            .iter()
                            .find_map(|env| {
                                env.carrier_types
                                    .contains_key(&name)
                                    .then(|| env.binding_ids.get(&name).copied())
                                    .flatten()
                            })
                            .or_else(|| canonical_binding_span(unit, &name, span.start))
                            .or_else(|| ids.iter().next().copied());
                        if let Some(identity) = identity {
                            merged.binding_ids.insert(name.clone(), identity);
                            let all_types = envs
                                .iter()
                                .filter_map(|env| env.types.get(&name).cloned())
                                .chain(prior_carriers)
                                .collect::<Vec<_>>();
                            if let Some(carrier_type) = union_flow_types(all_types) {
                                unit.flow_binding_types
                                    .insert(identity, carrier_type.clone());
                                merged.carrier_types.insert(name.clone(), carrier_type);
                            }
                        }
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
                let _flow_exits = flow_block(unit, block, active);
                validate_assignment_block(unit, block, &mut declared, &mut assigned)?;
            }
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "definite assignment keeps statement-specific branch joins in one ordered traversal"
)]
pub(super) fn validate_assignment_block(
    unit: &SemanticUnit,
    block: &SyntaxNode,
    declared: &mut BTreeSet<String>,
    assigned: &mut BTreeSet<String>,
) -> Result<(), SemanticFailure> {
    for statement in &block.children {
        match statement.kind {
            SyntaxKind::Binding => {
                let name_node = statement
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name);
                let Some(name_node) = name_node else { continue };
                let name = node_text(&unit.source, name_node).to_owned();
                let initializer = statement.children.iter().rev().find(|child| {
                    child.span != name_node.span && child.kind != SyntaxKind::TypeExpression
                });
                declared.insert(name.clone());
                if let Some(value) = initializer {
                    validate_assigned_reads(unit, value, declared, assigned)?;
                    assigned.insert(name);
                }
            }
            SyntaxKind::Assignment => {
                if let Some(value) = statement.children.get(1) {
                    validate_assigned_reads(unit, value, declared, assigned)?;
                }
                if let Some(target) = statement
                    .children
                    .first()
                    .filter(|target| target.kind == SyntaxKind::Name)
                {
                    let name = node_text(&unit.source, target).to_owned();
                    declared.insert(name.clone());
                    assigned.insert(name);
                }
            }
            SyntaxKind::TryStatement => {
                let incoming = assigned.clone();
                let mut results = Vec::new();
                if let Some(block) = statement.children.first() {
                    let mut try_declared = declared.clone();
                    let mut try_assigned = incoming.clone();
                    validate_assignment_block(unit, block, &mut try_declared, &mut try_assigned)?;
                    declared.extend(try_declared);
                    results.push(try_assigned);
                }
                for clause in statement
                    .children
                    .iter()
                    .filter(|child| child.kind == SyntaxKind::CatchClause)
                {
                    let Some(block) = clause
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block)
                    else {
                        continue;
                    };
                    let mut catch_declared = declared.clone();
                    let mut catch_assigned = incoming.clone();
                    if let Some(alias) = clause
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::CatchBinding)
                    {
                        let name = node_text(&unit.source, alias).to_owned();
                        catch_declared.insert(name.clone());
                        catch_assigned.insert(name);
                    }
                    validate_assignment_block(
                        unit,
                        block,
                        &mut catch_declared,
                        &mut catch_assigned,
                    )?;
                    declared.extend(catch_declared);
                    results.push(catch_assigned);
                }
                if let Some(first) = results.first() {
                    *assigned = results
                        .iter()
                        .skip(1)
                        .fold(first.clone(), |common, branch| {
                            common.intersection(branch).cloned().collect()
                        });
                }
                if let Some(finally) = statement
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::FinallyClause)
                    .and_then(|clause| clause.children.first())
                {
                    validate_assignment_block(unit, finally, declared, assigned)?;
                }
            }
            SyntaxKind::IfStatement => {
                if let Some(condition) = statement.children.first() {
                    validate_assigned_reads(unit, condition, declared, assigned)?;
                }
                let incoming = assigned.clone();
                let mut branch_results = Vec::new();
                let mut falls_through = true;
                for branch in statement.children.iter().skip(1) {
                    if !falls_through {
                        break;
                    }
                    let condition = if branch.kind == SyntaxKind::Block {
                        statement.children.first()
                    } else if branch.kind == SyntaxKind::ElseClause {
                        branch
                            .children
                            .iter()
                            .find(|child| child.kind != SyntaxKind::Block)
                    } else {
                        branch.children.first()
                    };
                    if let Some(condition) = condition {
                        validate_assigned_reads(unit, condition, declared, &incoming)?;
                        if constant_boolean(unit, condition) == Some(false) {
                            continue;
                        }
                    }
                    let branch_block = if branch.kind == SyntaxKind::Block {
                        Some(branch)
                    } else {
                        branch
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::Block)
                    };
                    if let Some(branch_block) = branch_block {
                        let mut branch_declared = declared.clone();
                        let mut branch_assigned = incoming.clone();
                        validate_assignment_block(
                            unit,
                            branch_block,
                            &mut branch_declared,
                            &mut branch_assigned,
                        )?;
                        declared.extend(branch_declared);
                        branch_results.push(branch_assigned);
                    }
                    if condition.is_none()
                        || constant_boolean(unit, condition.unwrap()) == Some(true)
                    {
                        falls_through = false;
                    }
                }
                if falls_through {
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
            }
            SyntaxKind::SelectStatement | SyntaxKind::MatchStatement => {
                let incoming = assigned.clone();
                let mut results = Vec::new();
                let mut exhaustive = statement.kind == SyntaxKind::SelectStatement;
                for case in &statement.children {
                    if !matches!(
                        case.kind,
                        SyntaxKind::SelectCase | SyntaxKind::MatchCase | SyntaxKind::ElseClause
                    ) {
                        continue;
                    }
                    let header = case.children.iter().find(|child| {
                        child.kind != SyntaxKind::Block && child.kind != SyntaxKind::ParameterList
                    });
                    let body = case
                        .children
                        .iter()
                        .find(|child| child.kind == SyntaxKind::Block);
                    let Some(body) = body else { continue };
                    let mut case_declared = declared.clone();
                    let mut case_assigned = incoming.clone();
                    if let Some(header) = header {
                        if case.kind == SyntaxKind::SelectCase && header.kind == SyntaxKind::Binding
                        {
                            if let Some(value) = header.children.iter().rev().find(|child| {
                                child.kind != SyntaxKind::Name
                                    && child.kind != SyntaxKind::TypeExpression
                            }) {
                                validate_assigned_reads(unit, value, declared, &incoming)?;
                            }
                            if let Some(name) = declaration_name(header, &unit.source) {
                                case_declared.insert(name.clone());
                                case_assigned.insert(name);
                            }
                        } else {
                            validate_assigned_reads(unit, header, declared, &incoming)?;
                        }
                    }
                    if case.kind == SyntaxKind::MatchCase {
                        if case
                            .children
                            .first()
                            .is_some_and(|child| child.kind == SyntaxKind::MatchCatchAll)
                        {
                            exhaustive = true;
                        }
                        if let Some(parameters) = case
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::ParameterList)
                        {
                            for parameter in &parameters.children {
                                let name = node_text(&unit.source, parameter).to_owned();
                                if name != "_" {
                                    case_declared.insert(name.clone());
                                    case_assigned.insert(name);
                                }
                            }
                        }
                    }
                    validate_assignment_block(unit, body, &mut case_declared, &mut case_assigned)?;
                    declared.extend(case_declared);
                    results.push(case_assigned);
                }
                if !exhaustive {
                    results.push(incoming);
                }
                if let Some(first) = results.first() {
                    *assigned = results
                        .iter()
                        .skip(1)
                        .fold(first.clone(), |common, branch| {
                            common.intersection(branch).cloned().collect()
                        });
                }
            }
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement => {
                let incoming = assigned.clone();
                for expression in statement.children.iter().filter(|child| {
                    child.kind != SyntaxKind::Block && child.kind != SyntaxKind::ForTarget
                }) {
                    validate_assigned_reads(unit, expression, declared, assigned)?;
                }
                let body = statement
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block);
                if let Some(body) = body {
                    let mut body_declared = declared.clone();
                    let mut body_assigned = incoming.clone();
                    if statement.kind == SyntaxKind::ForStatement
                        && let Some(target) = statement
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::ForTarget)
                    {
                        for name in target
                            .children
                            .iter()
                            .filter(|child| child.kind == SyntaxKind::Name)
                        {
                            let name = node_text(&unit.source, name).to_owned();
                            body_declared.insert(name.clone());
                            body_assigned.insert(name);
                        }
                    }
                    validate_assignment_block(unit, body, &mut body_declared, &mut body_assigned)?;
                    declared.extend(body_declared);
                }
                *assigned = incoming;
            }

            _ => validate_assigned_reads(unit, statement, declared, assigned)?,
        }
    }
    Ok(())
}

pub(super) fn validate_assigned_reads(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    declared: &BTreeSet<String>,
    assigned: &BTreeSet<String>,
) -> Result<(), SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        return Ok(());
    }
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        if declared.contains(name)
            && !assigned.contains(name)
            && !lexical_scope_chain(unit, node.span.start)
                .find_map(|scope| {
                    scope.symbols.get(name)?.iter().rev().find(|symbol| {
                        symbol
                            .binding_span
                            .is_none_or(|span| span.end <= node.span.start)
                    })
                })
                .is_some_and(|symbol| symbol.kind == SymbolKind::Function)
        {
            let key = (node.span.file, node.span.start, node.span.end);
            if !unit.flow_availability.contains_key(&key) {
                return Err(failure(
                    &unit.source,
                    "T0007",
                    format!("`{name}` may be read before it is assigned"),
                    node.span,
                ));
            }
        }
    }
    for (index, child) in node.children.iter().enumerate() {
        if is_flow_value_child(node, index, child) {
            validate_assigned_reads(unit, child, declared, assigned)?;
        }
    }
    Ok(())
}

pub(super) fn validate_control_flow(
    package: &SemanticPackage,
) -> Result<Vec<Vec<Span>>, SemanticFailure> {
    fn function_declarations<'a>(node: &'a SyntaxNode, declarations: &mut Vec<&'a SyntaxNode>) {
        if node.kind == SyntaxKind::FunctionDeclaration {
            declarations.push(node);
            return;
        }
        for child in &node.children {
            function_declarations(child, declarations);
        }
    }
    let mut unreachable_units = Vec::with_capacity(package.units.len());
    for unit in &package.units {
        let mut unreachable = Vec::new();
        let mut declarations = Vec::new();
        function_declarations(&unit.tree.root, &mut declarations);
        for function in declarations {
            let Some(contract) = unit
                .functions
                .iter()
                .find(|contract| contract.span == function.span)
            else {
                continue;
            };
            let Some(block) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            else {
                continue;
            };
            if block.children.is_empty() {
                continue;
            }
            let bindings = call_site_bindings(unit, Some(contract));
            let falls_through =
                validate_flow_block(unit, block, contract, &bindings, 0, &mut unreachable)?;
            if contract.return_type.clone().is_some() && falls_through {
                return Err(failure(
                    &unit.source,
                    "T0015",
                    format!(
                        "function `{}` may finish without returning a value",
                        contract.name
                    ),
                    function.span,
                ));
            }
        }
        unreachable_units.push(unreachable);
    }
    Ok(unreachable_units)
}

pub(super) fn block_may_fall_through(block: &SyntaxNode) -> bool {
    let Some(statement) = block.children.last() else {
        return true;
    };
    match statement.kind {
        SyntaxKind::ReturnStatement
        | SyntaxKind::ThrowStatement
        | SyntaxKind::BreakStatement
        | SyntaxKind::ContinueStatement => false,
        SyntaxKind::IfStatement => {
            let branches = statement
                .children
                .iter()
                .filter(|child| matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause))
                .collect::<Vec<_>>();
            let has_else = branches
                .iter()
                .any(|branch| branch.kind == SyntaxKind::ElseClause);
            !has_else || branches.iter().any(|branch| block_may_fall_through(branch))
        }
        SyntaxKind::MatchCase => statement.children.last().is_none_or(block_may_fall_through),
        SyntaxKind::Block | SyntaxKind::ElseClause | SyntaxKind::SelectCase => {
            block_may_fall_through(statement)
        }
        SyntaxKind::SelectStatement => statement.children.iter().any(block_may_fall_through),
        SyntaxKind::MatchStatement => statement
            .children
            .iter()
            .filter(|child| matches!(child.kind, SyntaxKind::MatchCase | SyntaxKind::ElseClause))
            .any(block_may_fall_through),
        _ => true,
    }
}

pub(super) fn validate_flow_block(
    unit: &SemanticUnit,
    block: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
    loop_depth: usize,
    unreachable: &mut Vec<Span>,
) -> Result<bool, SemanticFailure> {
    let mut falls_through = true;
    for statement in &block.children {
        if !falls_through {
            unreachable.push(statement.span);
            continue;
        }
        falls_through =
            validate_flow_statement(unit, statement, contract, bindings, loop_depth, unreachable)?;
    }
    Ok(falls_through)
}

#[expect(
    clippy::too_many_lines,
    reason = "flow validation keeps every statement transition in one exhaustive dispatch"
)]
pub(super) fn validate_flow_statement(
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
    loop_depth: usize,
    unreachable: &mut Vec<Span>,
) -> Result<bool, SemanticFailure> {
    match statement.kind {
        SyntaxKind::ReturnStatement => {
            validate_return(unit, statement, contract, bindings)?;
            Ok(false)
        }
        SyntaxKind::ThrowStatement => Ok(false),
        SyntaxKind::BreakStatement | SyntaxKind::ContinueStatement => {
            if loop_depth == 0 {
                let keyword = node_text(&unit.source, statement);
                return Err(failure(
                    &unit.source,
                    "T0014",
                    format!("`{keyword}` is only valid inside a loop"),
                    statement.span,
                ));
            }
            Ok(false)
        }
        SyntaxKind::IfStatement => {
            validate_if_flow(unit, statement, contract, bindings, loop_depth, unreachable)
        }
        SyntaxKind::MatchStatement => {
            let mut any_falls_through = false;
            for case in statement.children.iter().filter(|child| {
                matches!(child.kind, SyntaxKind::MatchCase | SyntaxKind::ElseClause)
            }) {
                if let Some(block) = case.children.last() {
                    any_falls_through |= validate_flow_block(
                        unit,
                        block,
                        contract,
                        bindings,
                        loop_depth,
                        unreachable,
                    )?;
                }
            }
            Ok(any_falls_through)
        }
        SyntaxKind::SelectStatement => {
            let mut any_falls_through = false;
            for case in &statement.children {
                if let Some(block) = case.children.last() {
                    any_falls_through |= validate_flow_block(
                        unit,
                        block,
                        contract,
                        bindings,
                        loop_depth,
                        unreachable,
                    )?;
                }
            }
            Ok(any_falls_through)
        }

        SyntaxKind::TryStatement => {
            let try_falls_through = if let Some(block) = statement.children.first() {
                validate_flow_block(unit, block, contract, bindings, loop_depth, unreachable)?
            } else {
                true
            };
            let mut catch_falls_through = false;
            for clause in statement
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::CatchClause)
            {
                if let Some(block) = clause
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Block)
                {
                    catch_falls_through |= validate_flow_block(
                        unit,
                        block,
                        contract,
                        bindings,
                        loop_depth,
                        unreachable,
                    )?;
                }
            }
            if let Some(finally) = statement
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::FinallyClause)
                .and_then(|clause| clause.children.first())
                && !validate_flow_block(unit, finally, contract, bindings, loop_depth, unreachable)?
            {
                return Ok(false);
            }
            Ok(try_falls_through || catch_falls_through)
        }
        SyntaxKind::WhileStatement => {
            if let Some(condition) = statement.children.first() {
                validate_bool_condition(unit, condition, bindings)?;
            }
            if let Some(block) = statement
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            {
                validate_flow_block(unit, block, contract, bindings, loop_depth + 1, unreachable)?;
            }
            Ok(true)
        }
        SyntaxKind::ForStatement => {
            let mut loop_bindings = bindings.to_vec();
            if statement.children.len() == 4 {
                validate_bool_condition(unit, &statement.children[1], bindings)?;
            } else if let [target, collection, block] = statement.children.as_slice() {
                let collection_type =
                    infer_value_type(unit, collection, bindings)?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0016",
                            "collection iteration requires an iterable value",
                            collection.span,
                        )
                    })?;
                let item_type = iterable_item_type(unit, collection_type).map_err(
                    |(code, message, span)| {
                        failure(&unit.source, code, message, span.unwrap_or(collection.span))
                    },
                )?;
                loop_bindings.extend(iteration_target_bindings(
                    unit,
                    target,
                    collection.span.end,
                    block.span,
                    item_type,
                )?);
            }
            if let Some(block) = statement
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            {
                validate_flow_block(
                    unit,
                    block,
                    contract,
                    &loop_bindings,
                    loop_depth + 1,
                    unreachable,
                )?;
            }
            Ok(true)
        }
        SyntaxKind::PostfixExpression => {
            let Some(operand) = statement.children.first() else {
                return Ok(true);
            };
            if operand.kind != SyntaxKind::Name
                || !matches!(
                    infer_value_type(unit, operand, bindings)?,
                    Some(ValueType::Scalar(ty)) if ty.is_integer()
                )
            {
                return Err(failure(
                    &unit.source,
                    "T0014",
                    "postfix update requires an assignable integer binding",
                    statement.span,
                ));
            }
            Ok(true)
        }
        _ => Ok(true),
    }
}

pub(super) fn validate_bool_condition(
    unit: &SemanticUnit,
    condition: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    match infer_value_type(unit, condition, bindings)? {
        Some(value_type)
            if descriptor_operation(unit, &value_type, "truth") == Some("value.truth") =>
        {
            return Ok(());
        }
        Some(ValueType::Object(identity)) => {
            let Some(truth) = descriptor_protocol_method(unit, &identity, "truth") else {
                return Err(failure(
                    &unit.source,
                    "T0014",
                    "control-flow object must define a non-static `truth` method",
                    condition.span,
                ));
            };
            if truth.is_async
                || truth.throws
                || truth.written_invocation_mode != InvocationMode::Shared
                || !truth.parameters.is_empty()
                || truth.return_type != Some(ValueType::Scalar(ScalarType::Bool))
            {
                return Err(failure(
                    &unit.source,
                    "T0014",
                    "truth protocol requires a synchronous, non-throwing, non-mutating, parameterless method returning `bool`",
                    truth.span,
                ));
            }
            return Ok(());
        }
        _ => {}
    }
    Err(failure(
        &unit.source,
        "T0014",
        "control-flow condition must have type `bool` or satisfy the truth protocol",
        condition.span,
    ))
}

pub(super) fn validate_if_flow(
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
    loop_depth: usize,
    unreachable: &mut Vec<Span>,
) -> Result<bool, SemanticFailure> {
    let condition = statement.children.first().ok_or_else(|| {
        failure(
            &unit.source,
            "T0014",
            "an `if` statement requires a condition",
            statement.span,
        )
    })?;
    validate_bool_condition(unit, condition, bindings)?;
    let mut branch_falls_through = Vec::new();
    let mut has_else = false;
    for branch in statement.children.iter().skip(1) {
        let block = if branch.kind == SyntaxKind::Block {
            Some(branch)
        } else if branch.kind == SyntaxKind::ElseClause {
            let mut children = branch.children.iter();
            let first = children.next();
            if first.is_some_and(|child| child.kind == SyntaxKind::Block) {
                has_else = true;
                first
            } else {
                if let Some(condition) = first {
                    validate_bool_condition(unit, condition, bindings)?;
                }
                children.find(|child| child.kind == SyntaxKind::Block)
            }
        } else {
            None
        };
        if let Some(block) = block {
            branch_falls_through.push(validate_flow_block(
                unit,
                block,
                contract,
                bindings,
                loop_depth,
                unreachable,
            )?);
        }
    }
    Ok(!has_else || branch_falls_through.into_iter().any(|branch| branch))
}

fn condition_proves_absent_member(
    source: &SourceFile,
    condition: &SyntaxNode,
    target_name: &str,
) -> bool {
    let condition = super::types::ungrouped_expression(condition);
    if condition.kind != SyntaxKind::BinaryExpression {
        return false;
    }
    let [left, right] = condition.children.as_slice() else {
        return false;
    };
    let operator = source.text()[left.span.end..right.span.start].trim();
    let left = super::types::ungrouped_expression(left);
    let right = super::types::ungrouped_expression(right);
    operator == "=="
        && matches!(
            (node_text(source, left), node_text(source, right)),
            (target, "none") | ("none", target) if target == target_name
        )
}

fn writes_exact_target(source: &SourceFile, node: &SyntaxNode, target_name: &str) -> bool {
    if matches!(
        node.kind,
        SyntaxKind::Assignment | SyntaxKind::PostfixExpression
    ) && node
        .children
        .first()
        .is_some_and(|target| node_text(source, target) == target_name)
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| writes_exact_target(source, child, target_name))
}

fn enclosing_block(node: &SyntaxNode, position: usize) -> Option<&SyntaxNode> {
    if !(node.span.start <= position && position <= node.span.end) {
        return None;
    }
    node.children
        .iter()
        .find_map(|child| enclosing_block(child, position))
        .or_else(|| (node.kind == SyntaxKind::Block).then_some(node))
}

fn post_if_initialized_member_type(
    unit: &SemanticUnit,
    returned: &SyntaxNode,
    actual: &ValueType,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    let ValueType::Optional(inner) = actual else {
        return Ok(None);
    };
    let returned = super::types::ungrouped_expression(returned);
    if returned.kind != SyntaxKind::StaticMemberExpression {
        return Ok(None);
    }
    let target_name = node_text(&unit.source, returned);
    let Some(block) = enclosing_block(&unit.tree.root, returned.span.start) else {
        return Ok(None);
    };
    let Some(return_index) = block.children.iter().position(|statement| {
        statement.span.file == returned.span.file
            && statement.span.start <= returned.span.start
            && returned.span.end <= statement.span.end
    }) else {
        return Ok(None);
    };
    for (candidate_index, candidate) in block.children[..return_index].iter().enumerate().rev() {
        if candidate.kind != SyntaxKind::IfStatement
            || candidate
                .children
                .iter()
                .any(|child| child.kind == SyntaxKind::ElseClause)
        {
            continue;
        }
        let Some(condition) = candidate.children.first() else {
            continue;
        };
        if !condition_proves_absent_member(&unit.source, condition, target_name)
            || block.children[candidate_index + 1..return_index]
                .iter()
                .any(|statement| writes_exact_target(&unit.source, statement, target_name))
        {
            continue;
        }
        let Some(body) = candidate
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        else {
            continue;
        };
        let mut assignments = body.children.iter().filter(|statement| {
            statement.kind == SyntaxKind::Assignment
                && statement
                    .children
                    .first()
                    .is_some_and(|target| node_text(&unit.source, target) == target_name)
        });
        let Some(assignment) = assignments.next() else {
            continue;
        };
        if assignments.next().is_some() {
            continue;
        }
        if body.children.iter().any(|statement| {
            statement.span != assignment.span
                && writes_exact_target(&unit.source, statement, target_name)
        }) {
            continue;
        }
        let Some(value) = assignment.children.get(1) else {
            continue;
        };
        let Some(assigned_type) = infer_value_type(unit, value, bindings)? else {
            continue;
        };
        if super::types::value_types_compatible(&unit.descriptors, inner, &assigned_type) {
            return Ok(Some(inner.as_ref().clone()));
        }
    }
    Ok(None)
}

pub(super) fn validate_return(
    unit: &SemanticUnit,
    statement: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let value = statement.children.first();
    match (contract.return_type.clone(), value) {
        (None, None) => Ok(()),
        (None, Some(value)) => Err(failure(
            &unit.source,
            "T0015",
            format!("function `{}` does not return a value", contract.name),
            value.span,
        )),
        (Some(expected), None) => Err(failure(
            &unit.source,
            "T0015",
            format!(
                "function `{}` must return `{}`",
                contract.name,
                diagnostic_value_type(&unit.descriptors, &expected)
            ),
            statement.span,
        )),
        (Some(expected), Some(value)) => {
            if contextual_collection_constructor_matches(unit, value, &expected, bindings) {
                return validate_collection_constructor_value(
                    unit,
                    value,
                    &expected,
                    &contract.name,
                    bindings,
                );
            }
            let Some(actual) = infer_value_type(unit, value, bindings)? else {
                return Err(failure(
                    &unit.source,
                    "T0015",
                    format!(
                        "function `{}` must return `{}`",
                        contract.name,
                        diagnostic_value_type(&unit.descriptors, &expected)
                    ),
                    value.span,
                ));
            };
            let actual =
                post_if_initialized_member_type(unit, value, &actual, bindings)?.unwrap_or(actual);
            validate_value_destination(
                &unit.source,
                &unit.descriptors,
                &contract.name,
                expected,
                actual,
                value,
                "T0015",
            )
        }
    }
}

pub(super) fn visible_symbol_for_lexical_import<'a>(
    unit: &SemanticUnit,
    namespaces: &'a BTreeMap<String, Namespace>,
    globals: &'a BTreeMap<String, Symbol>,
    prelude_bindings: &'a BTreeMap<String, Symbol>,
    scopes: &'a [LexicalScope],
    mut index: usize,
    name: &str,
) -> Option<&'a Symbol> {
    loop {
        if let Some(symbol) = scopes[index]
            .symbols
            .get(name)
            .and_then(|symbols| symbols.last())
        {
            return Some(symbol);
        }
        let Some(parent) = scopes[index].parent else {
            break;
        };
        index = parent;
    }
    visible_fallback_symbol(
        &unit.namespace,
        name,
        namespaces,
        globals,
        prelude_bindings,
        unit.prelude,
    )
}

pub(super) fn populate_imports(
    unit: &SemanticUnit,
    namespaces: &BTreeMap<String, Namespace>,
    globals: &BTreeMap<String, Symbol>,
    prelude_bindings: &BTreeMap<String, Symbol>,
    scopes: &mut [LexicalScope],
    index: usize,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    for import in imports_from_syntax(unit, node)? {
        for (name, mut export) in imported_objects(&import, namespaces)? {
            let existing = if import.namespace_wide {
                visible_symbol_for_lexical_import(
                    unit,
                    namespaces,
                    globals,
                    prelude_bindings,
                    scopes,
                    index,
                    &name,
                )
                .cloned()
            } else {
                scopes[index]
                    .symbols
                    .get(&name)
                    .and_then(|symbols| symbols.last())
                    .cloned()
            };
            if let Some(existing) = existing {
                if existing.identity == export.identity {
                    continue;
                }
                if !import.namespace_wide {
                    return Err(import_collision_failure(&import, &name));
                }
                scopes[index].import_warnings.push(import_overwrite_warning(
                    &name,
                    &existing,
                    &export,
                    import.span,
                ));
            }
            export.binding_span = Some(import.span);
            scopes[index].symbols.insert(name, vec![export]);
        }
    }
    Ok(())
}

pub(super) fn local_binding_exists(scopes: &[LexicalScope], mut index: usize, name: &str) -> bool {
    loop {
        let scope = &scopes[index];
        if scope.symbols.contains_key(name) {
            return true;
        }
        let Some(parent) = scope.parent else {
            return false;
        };
        index = parent;
    }
}

pub(super) fn insert_local(
    unit: &SemanticUnit,
    scopes: &mut [LexicalScope],
    index: usize,
    name: String,
    span: Span,
) -> Result<(), SemanticFailure> {
    let scope = &mut scopes[index];
    if scope.symbols.contains_key(&name) {
        return Err(failure(
            &unit.source,
            "S2012",
            format!("duplicate binding `{name}` in the same lexical scope"),
            span,
        ));
    }
    insert_local_replacement(unit, scopes, index, name, span);
    Ok(())
}

pub(super) fn insert_local_replacement(
    unit: &SemanticUnit,
    scopes: &mut [LexicalScope],
    index: usize,
    name: String,
    span: Span,
) {
    scopes[index]
        .symbols
        .entry(name.clone())
        .or_default()
        .push(Symbol {
            identity: format!("{}::scope{index}::{name}@{}", unit.namespace, span.start),
            lowering_identity: None,
            name,
            namespace: unit.namespace.clone(),
            visibility: Visibility::Private,
            global: false,
            constant: false,
            kind: SymbolKind::Binding,
            declaration_span: Some(span),
            binding_span: Some(span),
        });
}

pub(super) fn lexical_scope_index_at(unit: &SemanticUnit, offset: usize) -> Option<usize> {
    unit.scopes
        .iter()
        .enumerate()
        .filter(|(_, scope)| scope.span.start <= offset && offset < scope.span.end)
        .min_by_key(|(_, scope)| scope.span.end - scope.span.start)
        .map(|(index, _)| index)
}

pub(super) fn lexical_scope_chain(
    unit: &SemanticUnit,
    offset: usize,
) -> impl Iterator<Item = &LexicalScope> {
    let mut current = lexical_scope_index_at(unit, offset);
    std::iter::from_fn(move || {
        let index = current?;
        let scope = &unit.scopes[index];
        current = scope.parent;
        Some(scope)
    })
}

pub(super) fn namespace_chain(namespace: &str) -> impl Iterator<Item = String> {
    let mut current = namespace.trim_end_matches('/').to_owned();
    std::iter::from_fn(move || {
        if current.is_empty() {
            return None;
        }
        let result = current.clone();
        if current == "/" {
            current.clear();
        } else if let Some(separator) = current.rfind('/') {
            current.truncate(separator.max(1));
        } else {
            current.clear();
        }
        Some(result)
    })
}

fn visible_local_symbol<'a>(
    scopes: &'a [LexicalScope],
    mut index: usize,
    name: &str,
) -> Option<&'a Symbol> {
    loop {
        if let Some(symbol) = scopes[index]
            .symbols
            .get(name)
            .and_then(|symbols| symbols.last())
        {
            return Some(symbol);
        }
        index = scopes[index].parent?;
    }
}

pub(super) fn visible_from(symbol: &Symbol, namespace: &str) -> bool {
    match symbol.visibility {
        Visibility::Public => true,
        Visibility::Private => symbol.namespace == namespace,
        Visibility::Protected => {
            symbol.namespace == namespace
                || namespace
                    .strip_prefix(&symbol.namespace)
                    .is_some_and(|suffix| suffix.starts_with('/'))
        }
    }
}
pub(super) fn resolved_compiler_identity<'a>(
    unit: &'a SemanticUnit,
    node: &SyntaxNode,
) -> Option<&'a str> {
    let name = node_text(&unit.source, node);
    lexical_scope_chain(unit, node.span.start)
        .find_map(|scope| {
            scope.symbols.get(name)?.iter().rev().find(|symbol| {
                symbol
                    .declaration_span
                    .is_none_or(|span| span.end <= node.span.start)
            })
        })
        .map(Symbol::compiler_identity)
        .or_else(|| (unit.prelude && name == "task-scope").then_some("/core/async::task-scope"))
}

pub(super) fn constant_nonnegative_u64(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
    visited: &mut BTreeSet<(u32, usize, usize)>,
) -> Option<u64> {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .and_then(|child| constant_nonnegative_u64(unit, child, bindings, visited));
    }
    if node.kind == SyntaxKind::Name {
        let binding = bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, node)
                && binding.is_visible_at(unit.source.id(), node.span.start)
        })?;
        if !visited.insert((binding.span.file, binding.span.start, binding.span.end)) {
            return None;
        }
        return find_binding_initializer(&unit.tree.root, binding.span)
            .and_then(|value| constant_nonnegative_u64(unit, value, bindings, visited));
    }
    match contextual_constant(&unit.source, node, ScalarType::Int)? {
        Ok(ContextualConstant::Integer(value)) => value.to_u64(),
        Ok(ContextualConstant::Float32(_) | ContextualConstant::Float64(_)) | Err(_) => None,
    }
}

pub(super) fn find_binding_initializer(node: &SyntaxNode, name_span: Span) -> Option<&SyntaxNode> {
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) && node.span == name_span {
        return node.children.last();
    }
    node.children
        .iter()
        .find_map(|child| find_binding_initializer(child, name_span))
}

pub(super) fn bootstrap_prelude() -> BTreeMap<String, Symbol> {
    const PRELUDE: [(&str, &str, &str); 7] = [
        ("print", "/core/output::print", "/core/output"),
        ("task-scope", "/core/async::task-scope", "/core/async"),
        ("utf8", "/core/encodings::utf8", "/core/encodings"),
        ("utf16-le", "/core/encodings::utf16-le", "/core/encodings"),
        ("utf16-be", "/core/encodings::utf16-be", "/core/encodings"),
        ("utf32-le", "/core/encodings::utf32-le", "/core/encodings"),
        ("utf32-be", "/core/encodings::utf32-be", "/core/encodings"),
    ];
    PRELUDE
        .into_iter()
        .map(|(name, identity, namespace)| {
            (
                name.to_owned(),
                Symbol {
                    identity: identity.to_owned(),
                    lowering_identity: None,
                    name: name.to_owned(),
                    namespace: namespace.to_owned(),
                    visibility: Visibility::Public,
                    global: false,
                    constant: !matches!(name, "print" | "task-scope"),
                    kind: if matches!(name, "print" | "task-scope") {
                        SymbolKind::Function
                    } else {
                        SymbolKind::Binding
                    },
                    declaration_span: None,

                    binding_span: None,
                },
            )
        })
        .collect()
}

pub(super) fn bootstrap_descriptor_constructs() -> BTreeMap<String, Symbol> {
    ScalarType::SOURCE_NAMES
        .into_iter()
        .map(|(source_name, ty)| {
            let name = source_name.to_owned();
            (
                name.clone(),
                Symbol {
                    identity: format!("/core/types::{}", ty.source_name()),
                    lowering_identity: None,
                    name,
                    namespace: "/core/types".to_owned(),
                    visibility: Visibility::Public,
                    global: false,
                    constant: false,
                    kind: SymbolKind::TypeDescriptor,
                    declaration_span: None,

                    binding_span: None,
                },
            )
        })
        .collect()
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
                if let Some(binding) = unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == header.span)
                {
                    arm.types
                        .insert(binding.name.clone(), binding.value_type.clone());
                    arm.uncertain.remove(&binding.name);
                    arm.binding_ids.insert(binding.name.clone(), binding.span);
                    arm.origins
                        .insert(binding.name.clone(), BTreeSet::from([binding.span]));
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

fn flow_loop(
    unit: &mut SemanticUnit,
    statement: &SyntaxNode,
    state: FlowEnv,
    paths: &mut FlowStates,
) {
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
        for expression in statement
            .children
            .iter()
            .filter(|child| child.kind != SyntaxKind::Block && child.kind != SyntaxKind::ForTarget)
        {
            record_flow_reads(unit, expression, &state);
        }
    }
    loop {
        for expression in statement.children.iter().filter(|child| {
            statement.kind == SyntaxKind::WhileStatement
                && child.kind != SyntaxKind::Block
                && child.kind != SyntaxKind::ForTarget
        }) {
            record_flow_reads(unit, expression, &head);
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
        if backedge.is_empty() {
            break;
        }
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
    if let Some(target) = target {
        let name = node_text(&unit.source, target).to_owned();
        let value_type = value.and_then(|value| {
            unit.typed_bindings
                .iter()
                .find(|binding| binding.span == statement.span)
                .map(|binding| binding.value_type.clone())
                .or_else(|| {
                    infer_value_type(unit, value, &unit.typed_bindings)
                        .ok()
                        .flatten()
                })
        });
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
            } else if let Some(binding) = unit
                .typed_bindings
                .iter()
                .find(|binding| binding.span == statement.span)
            {
                next.binding_ids.insert(name.clone(), binding.span);
                next.origins
                    .insert(name.clone(), BTreeSet::from([binding.span]));
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
