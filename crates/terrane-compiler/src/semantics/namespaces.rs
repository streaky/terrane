use super::prelude::*;

pub(super) fn declared_namespace(
    source: &SourceFile,
    tree: &SyntaxTree,
) -> Result<String, Diagnostic> {
    let declarations = tree
        .root
        .children
        .iter()
        .filter(|node| node.kind == SyntaxKind::NamespaceDeclaration)
        .collect::<Vec<_>>();
    if declarations.is_empty() {
        return Err(Diagnostic::error(
            "S2002",
            "each source unit must declare exactly one namespace",
            Span::new(source.id(), 0, source.text().len()),
        ));
    }
    if declarations.len() > 1 {
        return Err(Diagnostic::error(
            "S0005",
            "duplicate namespace declaration",
            declarations[1].span,
        ));
    }
    let components = declarations[0]
        .children
        .iter()
        .filter(|child| child.kind == SyntaxKind::Name)
        .map(|child| {
            let component = node_text(source, child);
            validate_namespace_segment(component, child.span)?;
            Ok(component)
        })
        .collect::<Result<Vec<_>, Diagnostic>>()?;
    normalize_declared_path(&components).ok_or_else(|| {
        Diagnostic::error(
            "S2003",
            "namespace declaration requires an unanchored path",
            declarations[0].span,
        )
    })
}

pub(super) fn validate_namespace_segment(component: &str, span: Span) -> Result<(), Diagnostic> {
    fn valid(component: &str) -> bool {
        let mut bytes = component.bytes();
        let Some(first) = bytes.next() else {
            return false;
        };
        if !first.is_ascii_lowercase() {
            return false;
        }
        let mut previous_hyphen = false;
        for byte in bytes {
            if byte == b'-' {
                if previous_hyphen {
                    return false;
                }
                previous_hyphen = true;
            } else if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
                previous_hyphen = false;
            } else {
                return false;
            }
        }
        !previous_hyphen
    }

    if !valid(component) {
        let lowercase = component.to_ascii_lowercase();
        let mut diagnostic = Diagnostic::error(
            "S2018",
            format!(
                "invalid namespace segment `{component}`; segments must match `[a-z]([a-z0-9]|-[a-z0-9])*`"
            ),
            span,
        );
        if lowercase != component && valid(&lowercase) {
            diagnostic = diagnostic.with_help(format!("use `{lowercase}`"));
        }
        return Err(diagnostic);
    }
    if is_reserved_namespace_segment(component) {
        return Err(Diagnostic::error(
            "S2019",
            format!("namespace segment `{component}` is reserved"),
            span,
        )
        .with_help(format!(
            "choose a different name, such as `{component}-app`"
        )));
    }
    Ok(())
}

pub(super) fn is_reserved_namespace_segment(component: &str) -> bool {
    matches!(component, "con" | "prn" | "aux" | "nul")
        || component
            .strip_prefix("com")
            .or_else(|| component.strip_prefix("lpt"))
            .is_some_and(|suffix| suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9'))
}

pub(super) fn collect_unit(
    unit: &SemanticUnit,
    namespaces: &mut BTreeMap<String, Namespace>,
    globals: &mut BTreeMap<String, Symbol>,
    imports: &mut Vec<Import>,
) -> Result<(), SemanticFailure> {
    for node in &unit.tree.root.children {
        match node.kind {
            SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::FunctionDeclaration
            | SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration => {
                collect_declaration(unit, node, namespaces, globals)?;
            }
            SyntaxKind::ImportDeclaration => imports.extend(imports_from_syntax(unit, node)?),
            _ => {}
        }
        collect_nested_declarations(unit, node, namespaces, globals)?;
    }
    Ok(())
}
pub(super) fn collect_nested_declarations(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    namespaces: &mut BTreeMap<String, Namespace>,
    globals: &mut BTreeMap<String, Symbol>,
) -> Result<(), SemanticFailure> {
    for child in &node.children {
        if matches!(
            child.kind,
            SyntaxKind::Binding | SyntaxKind::Assignment | SyntaxKind::FunctionDeclaration
        ) && declaration_from_syntax(unit, child).is_some_and(|declaration| declaration.global)
        {
            collect_declaration(unit, child, namespaces, globals)?;
        }
        collect_nested_declarations(unit, child, namespaces, globals)?;
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub(super) struct Declaration {
    pub(super) name: String,
    pub(super) visibility: Visibility,
    pub(super) explicit_visibility: bool,
    pub(super) global: bool,
    pub(super) constant: bool,
    pub(super) kind: SymbolKind,
}

pub(super) fn declaration_from_syntax(
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Option<Declaration> {
    let name_node = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)?;
    let name = node_text(&unit.source, name_node).to_owned();
    let visibility_node = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Visibility);
    let visibility = visibility_node
        .map(|child| node_text(&unit.source, child))
        .map_or(Visibility::Public, |visibility| match visibility {
            "private" => Visibility::Private,
            "protected" => Visibility::Protected,
            _ => Visibility::Public,
        });
    let qualifier = |expected| {
        node.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == expected
        })
    };
    let kind = match node.kind {
        SyntaxKind::FunctionDeclaration => SymbolKind::Function,
        SyntaxKind::ClassDeclaration => SymbolKind::Class,
        SyntaxKind::InterfaceDeclaration => SymbolKind::Interface,
        SyntaxKind::TraitDeclaration => SymbolKind::Trait,
        _ => SymbolKind::Binding,
    };
    Some(Declaration {
        name,
        visibility,
        explicit_visibility: visibility_node.is_some(),
        global: qualifier("global"),
        constant: qualifier("constant"),
        kind,
    })
}

pub(super) fn collect_declaration(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    namespaces: &mut BTreeMap<String, Namespace>,
    globals: &mut BTreeMap<String, Symbol>,
) -> Result<(), SemanticFailure> {
    let declaration = declaration_from_syntax(unit, node).ok_or_else(|| {
        failure(
            &unit.source,
            "S2004",
            "declaration has no resolvable name",
            node.span,
        )
    })?;
    if node.kind == SyntaxKind::Assignment && globals.contains_key(&declaration.name) {
        return Ok(());
    }
    if declaration.kind == SymbolKind::Binding
        && !declaration.constant
        && !declaration.global
        && declaration.explicit_visibility
        && declaration.visibility == Visibility::Public
    {
        return Err(failure(
            &unit.source,
            "S2025",
            format!("namespace variable `{}` cannot be public", declaration.name),
            node.span,
        ));
    }
    let identity = if declaration.global {
        format!("global::{}", declaration.name)
    } else {
        format!("{}::{}", unit.namespace, declaration.name)
    };
    let symbol = Symbol {
        identity,
        lowering_identity: None,
        name: declaration.name.clone(),
        namespace: unit.namespace.clone(),
        visibility: declaration.visibility,
        global: declaration.global,
        constant: declaration.constant,
        kind: declaration.kind,
        declaration_span: Some(node.span),
        binding_span: Some(node.span),
    };
    if declaration.global {
        globals.insert(declaration.name, symbol);
        return Ok(());
    }
    let table = &mut namespaces
        .get_mut(&unit.namespace)
        .expect("every source-unit namespace is assembled before declarations")
        .symbols;
    if node.kind == SyntaxKind::Assignment
        && table.get(&declaration.name).is_some_and(|existing| {
            existing
                .declaration_span
                .is_some_and(|span| span.file == node.span.file)
        })
    {
        return Ok(());
    }
    if table.contains_key(&declaration.name) {
        return Err(failure(
            &unit.source,
            "S2005",
            format!("duplicate declaration `{}`", declaration.name),
            node.span,
        ));
    }
    table.insert(declaration.name, symbol);
    Ok(())
}

pub(super) fn namespace_capabilities(namespace: &str) -> &'static [&'static str] {
    match namespace {
        "/core/streams" | "/core/process" => &["process"],
        "/core/filesystem" => &["filesystem"],
        "/core/random" | "/core/random/uuid" => &["entropy"],
        "/core/networking" => &["networking"],
        "/core/networking/tls" => &["networking", "tls"],
        "/core/concurrency" => &["threads"],
        _ => &[],
    }
}

pub(super) fn imports_from_syntax(
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<Vec<Import>, SemanticFailure> {
    let path = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::NamespacePath)
        .ok_or_else(|| failure(&unit.source, "S2006", "malformed import", node.span))?;
    let anchored = path.children.first().is_some_and(|child| {
        child.kind == SyntaxKind::NamespaceAnchor && node_text(&unit.source, child) == "/"
    });
    let components = path
        .children
        .iter()
        .map(|child| node_text(&unit.source, child))
        .collect::<Vec<_>>();
    let target = resolve_path(&unit.namespace, anchored, &components).ok_or_else(|| {
        failure(
            &unit.source,
            "S2007",
            "namespace path escapes above root",
            path.span,
        )
    })?;
    let imports = node
        .children
        .iter()
        .filter(|child| child.kind == SyntaxKind::ObjectImport);
    let mut result = Vec::new();
    for import_node in imports {
        let imported_node = import_node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "S2008",
                    "import has no name",
                    import_node.span,
                )
            })?;
        let imported = node_text(&unit.source, imported_node);
        let alias = import_node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::ImportAlias)
            .and_then(|alias| {
                alias
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)
            })
            .map_or(imported, |alias| node_text(&unit.source, alias));
        result.push(Import {
            source: unit.source.clone(),
            bundled: unit.bundled,
            namespace: unit.namespace.clone(),
            target: target.clone(),
            namespace_wide: false,
            object: imported.to_owned(),
            alias: alias.to_owned(),
            span: import_node.span,
        });
    }
    if result.is_empty() {
        if target == "/deps" || target.starts_with("/deps/") {
            return Err(failure(
                &unit.source,
                "S2033",
                "namespace-wide dependency imports are not implemented; import dependency objects explicitly",
                node.span,
            ));
        }
        result.push(Import {
            source: unit.source.clone(),
            bundled: unit.bundled,
            namespace: unit.namespace.clone(),
            target,
            namespace_wide: true,
            object: String::new(),
            alias: String::new(),
            span: node.span,
        });
    }
    Ok(result)
}
pub(super) fn imports_in_tree(unit: &SemanticUnit) -> Result<Vec<Import>, SemanticFailure> {
    fn collect(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        imports: &mut Vec<Import>,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::ImportDeclaration {
            imports.extend(imports_from_syntax(unit, node)?);
        }
        for child in &node.children {
            collect(unit, child, imports)?;
        }
        Ok(())
    }

    let mut imports = Vec::new();
    collect(unit, &unit.tree.root, &mut imports)?;
    Ok(imports)
}
pub(super) fn imported_object(
    import: &Import,
    namespaces: &BTreeMap<String, Namespace>,
) -> Result<Symbol, SemanticFailure> {
    let export = namespaces
        .get(&import.target)
        .and_then(|namespace| namespace.symbols.get(&import.object))
        .ok_or_else(|| {
            failure(
                &import.source,
                "S2009",
                format!("unresolved name `{}` in `{}`", import.object, import.target),
                import.span,
            )
        })?;
    if !visible_from(export, &import.namespace) {
        return Err(failure(
            &import.source,
            "S2010",
            format!("name `{}` is inaccessible", import.object),
            import.span,
        ));
    }
    if !export.available_in_function_body() {
        return Err(namespace_variable_import_failure(
            &import.source,
            &import.object,
            import.span,
        ));
    }
    Ok(export.clone())
}
pub(super) fn imported_objects(
    import: &Import,
    namespaces: &BTreeMap<String, Namespace>,
) -> Result<Vec<(String, Symbol)>, SemanticFailure> {
    if !import.namespace_wide {
        return Ok(vec![(
            import.alias.clone(),
            imported_object(import, namespaces)?,
        )]);
    }
    let namespace = namespaces.get(&import.target).ok_or_else(|| {
        failure(
            &import.source,
            "S2009",
            format!("unknown namespace `{}`", import.target),
            import.span,
        )
    })?;
    if let Some(symbol) = namespace.symbols.values().find(|symbol| {
        symbol.visibility == Visibility::Public && !symbol.available_in_function_body()
    }) {
        return Err(namespace_variable_import_failure(
            &import.source,
            &symbol.name,
            import.span,
        ));
    }
    Ok(namespace
        .symbols
        .values()
        .filter(|symbol| {
            symbol.visibility == Visibility::Public && symbol.available_in_function_body()
        })
        .map(|symbol| (symbol.name.clone(), symbol.clone()))
        .collect())
}

pub(super) fn import_collision_failure(import: &Import, name: &str) -> SemanticFailure {
    failure(
        &import.source,
        "S2011",
        format!("object import collides on `{name}`; use an `as` alias"),
        import.span,
    )
}

pub(super) fn import_overwrite_warning(
    name: &str,
    existing: &Symbol,
    replacement: &Symbol,
    span: Span,
) -> Diagnostic {
    Diagnostic::warning(
        "W4004",
        format!(
            "namespace-wide import overwrites `{name}` from `{}` with `{}`",
            existing.identity, replacement.identity
        ),
        span,
    )
    .with_help("use selective `from ... import ... as ...` imports to retain both objects")
}

pub(super) fn import_declaration_precedence_warning(
    import: &Import,
    name: &str,
    declaration: &Symbol,
    rejected: &Symbol,
) -> Diagnostic {
    Diagnostic::warning(
        "W4004",
        format!(
            "namespace-wide import leaves declared `{name}` from `{}` in place instead of `{}`",
            declaration.identity, rejected.identity
        ),
        import.span,
    )
    .with_help("use a selective `from ... import ... as ...` import to bind the imported object")
}

pub(super) fn visible_fallback_symbol<'a>(
    namespace: &str,
    name: &str,
    namespaces: &'a BTreeMap<String, Namespace>,
    globals: &'a BTreeMap<String, Symbol>,
    prelude_bindings: &'a BTreeMap<String, Symbol>,
) -> Option<&'a Symbol> {
    namespace_chain(namespace)
        .skip(1)
        .find_map(|path| {
            namespaces.get(&path)?.symbols.get(name).filter(|symbol| {
                visible_from(symbol, namespace)
                    && (symbol.kind != SymbolKind::Binding
                        || symbol.constant
                        || symbol.global
                        || symbol.namespace == namespace)
            })
        })
        .or_else(|| {
            globals
                .get(name)
                .filter(|symbol| visible_from(symbol, namespace))
        })
        .or_else(|| namespaces.get("/core/types")?.symbols.get(name))
        .or_else(|| prelude_bindings.get(name))
}

pub(super) fn resolve_imports(
    imports: Vec<Import>,
    namespaces: &mut BTreeMap<String, Namespace>,
    globals: &BTreeMap<String, Symbol>,
    prelude_bindings: &BTreeMap<String, Symbol>,
) -> Result<Vec<Diagnostic>, SemanticFailure> {
    let mut warnings = Vec::new();
    for import in imports {
        let exports = imported_objects(&import, namespaces)?;
        for (name, mut export) in exports {
            let existing = namespaces
                .get(&import.namespace)
                .and_then(|destination| destination.symbols.get(&name))
                .cloned();
            if let Some(existing) = existing {
                if existing.identity == export.identity {
                    continue;
                }
                if !import.namespace_wide {
                    return Err(import_collision_failure(&import, &name));
                }
                if existing.namespace == import.namespace {
                    warnings.push(import_declaration_precedence_warning(
                        &import, &name, &existing, &export,
                    ));
                    continue;
                }
                warnings.push(import_overwrite_warning(
                    &name,
                    &existing,
                    &export,
                    existing.binding_span.unwrap_or(import.span),
                ));
            } else if import.namespace_wide
                && let Some(existing) = visible_fallback_symbol(
                    &import.namespace,
                    &name,
                    namespaces,
                    globals,
                    prelude_bindings,
                )
                && existing.identity != export.identity
            {
                warnings.push(import_overwrite_warning(
                    &name,
                    existing,
                    &export,
                    import.span,
                ));
            }
            export.binding_span = Some(import.span);
            namespaces
                .get_mut(&import.namespace)
                .expect("every import destination is a preassembled source-unit namespace")
                .symbols
                .insert(name, export);
        }
    }
    Ok(warnings)
}

pub(super) fn resolved_object_span(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
) -> Option<Span> {
    package
        .units
        .iter()
        .flat_map(|unit| &unit.objects)
        .find(|object| object.identity == *identity)
        .map(|object| object.span)
}

pub(super) fn enclosing_function_contract(
    unit: &SemanticUnit,
    offset: usize,
) -> Option<&FunctionContract> {
    let span = unit
        .enclosing_function_spans
        .get(&offset)
        .copied()
        .flatten()?;
    unit.functions.iter().find(|contract| contract.span == span)
}

pub(super) fn is_implicit_object_receiver(unit: &SemanticUnit, offset: usize, name: &str) -> bool {
    let Some(function_span) = unit
        .enclosing_function_spans
        .get(&offset)
        .copied()
        .flatten()
    else {
        return false;
    };
    if object_name_containing(unit, function_span).is_none() {
        return false;
    }
    if name == "self" {
        return true;
    }
    if name != "this" {
        return false;
    }
    find_node_by_span(&unit.tree.root, function_span).is_some_and(|function| {
        !function.children.iter().any(|child| {
            child.kind == SyntaxKind::DeclarationQualifier
                && node_text(&unit.source, child) == "static"
        })
    })
}

pub(super) fn class_designator_identity(
    unit: &SemanticUnit,
    designator: &SyntaxNode,
) -> Option<ObjectIdentity> {
    if designator.kind != SyntaxKind::Name {
        return None;
    }
    let name = node_text(&unit.source, designator);
    if name != "self"
        && unit.typed_bindings.iter().rev().any(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), designator.span.start)
        })
    {
        return None;
    }
    if name == "self" {
        let owner = enclosing_function_contract(unit, designator.span.start)?
            .owner
            .as_deref()?;
        return unit
            .objects
            .iter()
            .find(|object| object.name == owner && object.kind == ObjectKind::Class)
            .map(|object| object.identity.clone());
    }
    unit.objects
        .iter()
        .find(|object| object.name == name && object.kind == ObjectKind::Class)
        .map(|object| object.identity.clone())
}

pub(super) fn method_contract<'a>(
    package: &'a SemanticPackage,
    object_identity: &ObjectIdentity,
    method_name: &str,
    is_static: bool,
) -> Option<&'a FunctionContract> {
    fn contract<'a>(
        unit: &'a SemanticUnit,
        object_name: &str,
        method_name: &str,
        is_static: bool,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| {
                method.owner.as_deref() == Some(object_name)
                    && method.name == method_name
                    && method.is_static == is_static
            })
            .or_else(|| {
                unit.objects
                    .iter()
                    .find(|object| object.name == object_name)
                    .and_then(|object| object.base.as_ref())
                    .and_then(|base| unit.objects.iter().find(|object| object.identity == *base))
                    .and_then(|base| contract(unit, &base.name, method_name, is_static))
            })
    }
    let object = package
        .units
        .iter()
        .flat_map(|candidate| &candidate.objects)
        .find(|object| object.identity == *object_identity)?;
    package
        .units
        .iter()
        .find(|candidate| candidate.source.id() == object.span.file)
        .and_then(|candidate| contract(candidate, &object.name, method_name, is_static))
}

pub(super) fn construction_contract<'a>(
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    callee: &SyntaxNode,
) -> Option<&'a FunctionContract> {
    let class = callee
        .children
        .first()
        .filter(|_| callee.kind == SyntaxKind::ConstructionExpression)?;
    let identity = class_designator_identity(unit, class)?;
    method_contract(package, &identity, "construct", false)
}

pub(super) fn function_parameters<'a>(
    package: &'a SemanticPackage,
    unit: &'a SemanticUnit,
    callee: &SyntaxNode,
) -> Option<&'a [ParameterContract]> {
    if matches!(
        callee.kind,
        SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
    ) {
        let [receiver, member] = callee.children.as_slice() else {
            return None;
        };
        let object_identity = if callee.kind == SyntaxKind::StaticMemberExpression {
            class_designator_identity(unit, receiver)?
        } else {
            let ValueType::Object(object_identity) = unit.inferred_value_type(receiver)? else {
                return None;
            };
            object_identity
        };
        return method_contract(
            package,
            &object_identity,
            node_text(&unit.source, member),
            callee.kind == SyntaxKind::StaticMemberExpression,
        )
        .map(|method| method.parameters.as_slice());
    }
    if callee.kind == SyntaxKind::ConstructionExpression {
        return construction_contract(package, unit, callee)
            .map(|function| function.parameters.as_slice());
    }
    if callee.kind != SyntaxKind::Name {
        return None;
    }
    let symbol =
        package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
    let declaration = symbol.declaration_span?;
    package
        .units
        .iter()
        .flat_map(|candidate| &candidate.functions)
        .find(|function| function.span == declaration)
        .map(|function| function.parameters.as_slice())
}
