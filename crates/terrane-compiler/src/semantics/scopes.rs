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
                        add_lexical_scope(unit, context, &mut scopes, method, None, true)?;
                    }
                }
            }
            _ if is_function_node(node) => {
                add_lexical_scope(unit, context, &mut scopes, node, None, true)?;
            }
            SyntaxKind::Block => {
                add_lexical_scope(unit, context, &mut scopes, node, None, false)?;
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
    function_body: bool,
) -> Result<usize, SemanticFailure> {
    let namespaces = context.namespaces;
    let index = scopes.len();
    scopes.push(LexicalScope {
        span: node.span,
        parent,
        symbols: BTreeMap::new(),
        import_warnings: Vec::new(),
    });
    if parent.is_none() && function_body {
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
    if node.kind == SyntaxKind::Block {
        populate_scope(unit, context, scopes, index, node)?;
        return Ok(index);
    }
    if is_function_node(node) && object_name_containing(unit, node.span).is_some() {
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
    if is_function_node(node)
        && let Some(parameters) = node
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
            SyntaxKind::Block if is_function_node(node) => {
                populate_scope(unit, context, scopes, index, child)?;
            }
            SyntaxKind::Block => {
                add_lexical_scope(unit, context, scopes, child, Some(index), false)?;
            }
            _ if function_body => {
                populate_node(unit, context, scopes, index, child)?;
            }
            _ => {}
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
    for node in &block.children {
        populate_node(unit, context, scopes, index, node)?;
    }
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
            if let Some(name) = declaration_name(node, &unit.source) {
                insert_local(unit, scopes, index, name, node.span)?;
            }
            add_lexical_scope(unit, context, scopes, node, Some(index), true)?;
        }
        SyntaxKind::AnonymousFunction => {
            add_lexical_scope(unit, context, scopes, node, Some(index), true)?;
        }
        SyntaxKind::Block => {
            add_lexical_scope(unit, context, scopes, node, Some(index), false)?;
        }
        SyntaxKind::ForStatement => {
            let loop_index = scopes.len();
            scopes.push(LexicalScope {
                span: node.span,
                parent: Some(index),
                symbols: BTreeMap::new(),
                import_warnings: Vec::new(),
            });
            if let Some(first) = node.children.first() {
                if first.kind == SyntaxKind::ForTarget {
                    for name in &first.children {
                        insert_local(
                            unit,
                            scopes,
                            loop_index,
                            node_text(&unit.source, name).to_owned(),
                            name.span,
                        )?;
                    }
                } else {
                    populate_node(unit, context, scopes, loop_index, first)?;
                }
            }
            if let Some(block) = node.children.last()
                && block.kind == SyntaxKind::Block
            {
                add_lexical_scope(unit, context, scopes, block, Some(loop_index), false)?;
            }
        }
        SyntaxKind::CatchClause => {
            let catch_index = scopes.len();
            scopes.push(LexicalScope {
                span: node.span,
                parent: Some(index),
                symbols: BTreeMap::new(),
                import_warnings: Vec::new(),
            });
            if let Some(block) = node.children.last()
                && block.kind == SyntaxKind::Block
            {
                add_lexical_scope(unit, context, scopes, block, Some(catch_index), false)?;
            }
        }
        SyntaxKind::ElseClause => {
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    add_lexical_scope(unit, context, scopes, child, Some(index), false)?;
                }
            }
        }
        _ => {
            for child in &node.children {
                if child.kind == SyntaxKind::Block {
                    add_lexical_scope(unit, context, scopes, child, Some(index), false)?;
                } else if matches!(
                    child.kind,
                    SyntaxKind::AnonymousFunction
                        | SyntaxKind::ElseClause
                        | SyntaxKind::CatchClause
                        | SyntaxKind::FinallyClause
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
    if declaration.global {
        return Ok(());
    }
    let typed_replacement = node
        .children
        .iter()
        .any(|child| child.kind == SyntaxKind::TypeExpression)
        && scopes[index].symbols.contains_key(&declaration.name);
    if typed_replacement {
        insert_local_replacement(unit, scopes, index, declaration.name, node.span);
        Ok(())
    } else {
        insert_local(unit, scopes, index, declaration.name, node.span)
    }
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
    insert_local(unit, scopes, index, declaration.name, node.span)
}
pub(super) fn validate_definite_assignment(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        for function in unit
            .tree
            .root
            .children
            .iter()
            .filter(|node| node.kind == SyntaxKind::FunctionDeclaration)
        {
            let mut declared = BTreeSet::new();
            let mut assigned = BTreeSet::new();
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
                        assigned.insert(node_text(&unit.source, name).to_owned());
                    }
                }
            }
            if let Some(block) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block)
            {
                validate_assignment_block(unit, block, &mut declared, &mut assigned)?;
            }
        }
    }
    Ok(())
}

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
                let Some(name_node) = name_node else {
                    continue;
                };
                let name = node_text(&unit.source, name_node).to_owned();
                let initializer = statement.children.iter().rev().find(|child| {
                    child.span != name_node.span && child.kind != SyntaxKind::TypeExpression
                });
                if let Some(initializer) = initializer {
                    validate_assigned_reads(unit, initializer, declared, assigned)?;
                    assigned.insert(name.clone());
                }
                if statement
                    .children
                    .iter()
                    .any(|child| child.kind == SyntaxKind::TypeExpression)
                {
                    declared.insert(name);
                }
            }
            SyntaxKind::Assignment => {
                if let Some(value) = statement.children.get(1) {
                    validate_assigned_reads(unit, value, declared, assigned)?;
                }
                if let Some(target) = statement.children.first()
                    && target.kind == SyntaxKind::Name
                {
                    assigned.insert(node_text(&unit.source, target).to_owned());
                }
            }
            SyntaxKind::IfStatement => {
                if let Some(condition) = statement.children.first() {
                    validate_assigned_reads(unit, condition, declared, assigned)?;
                }
                let incoming = assigned.clone();
                let mut branch_results = Vec::new();
                for branch in statement.children.iter().skip(1) {
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
                        validate_assignment_block(
                            unit,
                            branch_block,
                            declared,
                            &mut branch_assigned,
                        )?;
                        branch_results.push(branch_assigned);
                    }
                }
                let has_else = statement
                    .children
                    .iter()
                    .any(|child| child.kind == SyntaxKind::ElseClause);
                if !has_else {
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
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        if declared.contains(name) && !assigned.contains(name) {
            return Err(failure(
                &unit.source,
                "T0007",
                format!("`{name}` may be read before it is assigned"),
                node.span,
            ));
        }
    }
    for child in &node.children {
        validate_assigned_reads(unit, child, declared, assigned)?;
    }
    Ok(())
}

pub(super) fn validate_control_flow(
    package: &SemanticPackage,
) -> Result<Vec<Vec<Span>>, SemanticFailure> {
    let mut unreachable_units = Vec::with_capacity(package.units.len());
    for unit in &package.units {
        let mut unreachable = Vec::new();
        for function in unit
            .tree
            .root
            .children
            .iter()
            .filter(|node| node.kind == SyntaxKind::FunctionDeclaration)
        {
            let Some(name_node) = function
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            else {
                continue;
            };
            let Some(contract) = unit
                .functions
                .iter()
                .find(|contract| contract.name == node_text(&unit.source, name_node))
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
                let collection_type = infer_value_type(unit, collection, bindings)?;
                let Some(item_type) = collection_type.and_then(iterable_item_type) else {
                    return Err(failure(
                        &unit.source,
                        "T0016",
                        "collection iteration requires an iterable value",
                        collection.span,
                    ));
                };
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
    if matches!(
        infer_value_type(unit, condition, bindings)?,
        Some(ValueType::Scalar(ScalarType::Bool))
    ) {
        return Ok(());
    }
    Err(failure(
        &unit.source,
        "T0014",
        "control-flow condition must have type `bool`",
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
                diagnostic_value_type(&unit.objects, &expected)
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
                        diagnostic_value_type(&unit.objects, &expected)
                    ),
                    value.span,
                ));
            };
            validate_value_destination(
                &unit.source,
                &unit.objects,
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
    visible_fallback_symbol(&unit.namespace, name, namespaces, globals, prelude_bindings)
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

pub(super) fn constant_deadline_ms(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
    visited: &mut BTreeSet<(u32, usize, usize)>,
) -> Option<u64> {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .and_then(|child| constant_deadline_ms(unit, child, bindings, visited));
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
            .and_then(|value| constant_deadline_ms(unit, value, bindings, visited));
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

pub(super) fn task_scope_deadline_ms(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
    visited: &mut BTreeSet<(u32, usize, usize)>,
) -> Option<u64> {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .and_then(|child| task_scope_deadline_ms(unit, child, bindings, visited));
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
            .and_then(|value| task_scope_deadline_ms(unit, value, bindings, visited));
    }
    if node.kind != SyntaxKind::CallExpression {
        return None;
    }
    let [callee, arguments] = node.children.as_slice() else {
        return None;
    };
    if callee.kind == SyntaxKind::Name
        && resolved_compiler_identity(unit, callee)
            .is_some_and(|identity| identity == "/core/async::task-scope")
    {
        return arguments
            .children
            .first()
            .and_then(|argument| argument.children.last().or(Some(argument)))
            .and_then(|value| constant_deadline_ms(unit, value, bindings, visited));
    }
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    if callee.kind != SyntaxKind::MemberExpression
        || node_text(&unit.source, member) != "child-scope"
    {
        return None;
    }
    arguments
        .children
        .first()
        .and_then(|argument| argument.children.last().or(Some(argument)))
        .and_then(|value| constant_deadline_ms(unit, value, bindings, visited))
        .or_else(|| task_scope_deadline_ms(unit, receiver, bindings, visited))
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
