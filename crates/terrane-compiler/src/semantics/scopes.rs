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
            {
                insert_local_replacement(unit, scopes, index, name, binding.span);
            }
            if node.kind == SyntaxKind::MatchCase
                && let Some(parameters) = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::ParameterList)
            {
                for parameter in &parameters.children {
                    let name = node_text(&unit.source, parameter).to_owned();
                    if name != "_" {
                        insert_local_replacement(unit, scopes, index, name, parameter.span);
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
                insert_local_replacement(unit, scopes, index, name, alias.span);
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
    if local_binding_exists(scopes, index, &declaration.name)
        && visible_local_symbol(scopes, index, &declaration.name).is_some_and(|symbol| {
            symbol.kind == SymbolKind::Binding || is_lexically_owned_function(symbol, scopes, index)
        })
    {
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
    if scopes[index].symbols.contains_key(&declaration.name) {
        insert_local_replacement(unit, scopes, index, declaration.name, node.span);
    } else {
        insert_local(unit, scopes, index, declaration.name, node.span)?;
    }
    Ok(())
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
