use super::prelude::*;

pub(super) fn parse_unit(
    source: &SourceFile,
    source_path: String,
    expected_namespace: Option<&str>,
    prelude: bool,
    bundled: bool,
) -> Result<SemanticUnit, SemanticFailure> {
    let lexed = lexer::lex(source).map_err(|diagnostics| SemanticFailure {
        source: source.clone(),
        diagnostics,
    })?;
    let parsed = parser::parse(source, lexed);
    if !parsed.diagnostics.is_empty() {
        return Err(SemanticFailure {
            source: source.clone(),
            diagnostics: parsed.diagnostics,
        });
    }
    let namespace =
        declared_namespace(source, &parsed.tree).map_err(|diagnostic| SemanticFailure {
            source: source.clone(),
            diagnostics: vec![diagnostic],
        })?;
    if let Some(expected) = expected_namespace
        && namespace != expected
    {
        let span = parsed
            .tree
            .root
            .children
            .iter()
            .find(|node| node.kind == SyntaxKind::NamespaceDeclaration)
            .map_or(Span::new(source.id(), 0, source.text().len()), |node| {
                node.span
            });
        let diagnostic = Diagnostic::error(
            "S2020",
            format!(
                "declared namespace `{namespace}` does not match `{expected}` required by its source directory"
            ),
            span,
        )
        .with_help(format!("declare `namespace {}`", expected.trim_start_matches('/')));
        return Err(SemanticFailure {
            source: source.clone(),
            diagnostics: vec![diagnostic],
        });
    }
    let enclosing_function_spans = index_enclosing_function_spans(&parsed.tree.root);
    Ok(SemanticUnit {
        source: source.clone(),
        source_path,
        tree: parsed.tree,
        namespace,
        prelude,
        bundled,
        scopes: Vec::new(),
        typed_bindings: Vec::new(),
        functions: Vec::new(),
        objects: Vec::new(),
        comparable_foreign_objects: BTreeSet::new(),
        function_aliases: BTreeMap::new(),
        function_contracts_by_span: BTreeMap::new(),
        descriptor_aliases: BTreeMap::new(),
        enclosing_function_spans,
        unreachable_spans: Vec::new(),
        evaluation_steps: Vec::new(),
    })
}

pub(super) fn parse_units(
    package: &Package,
    projection: &crate::projection::Projection,
) -> Result<Vec<SemanticUnit>, SemanticFailure> {
    let mut units = package
        .units
        .iter()
        .map(|unit| {
            parse_unit(
                &unit.source,
                unit.relative_path_text(),
                unit.expected_namespace.as_deref(),
                package.prelude,
                false,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut loaded = units
        .iter()
        .map(|unit| unit.namespace.clone())
        .collect::<BTreeSet<_>>();
    let mut next_source_id = units
        .iter()
        .map(|unit| unit.source.id())
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    let mut dependency_imports = BTreeMap::<String, BTreeSet<String>>::new();
    for unit in &units {
        for import in imports_in_tree(unit)?
            .into_iter()
            .filter(|import| import.target.starts_with("/deps/"))
        {
            dependency_imports
                .entry(import.target)
                .or_default()
                .insert(import.object);
        }
    }
    for (namespace, text) in projection.source_for_imports(&dependency_imports) {
        if !loaded.insert(namespace.clone()) {
            continue;
        }
        let path = format!(
            "<terrane>/projected/{}.trn",
            namespace.trim_start_matches('/')
        );
        let source = SourceFile::new(next_source_id, path.clone().into(), text);
        next_source_id = next_source_id.saturating_add(1);
        units.push(parse_unit(
            &source,
            path,
            Some(&namespace),
            package.prelude,
            true,
        )?);
    }
    let mut index = 0;
    while index < units.len() {
        let targets = imports_in_tree(&units[index])?
            .into_iter()
            .map(|import| import.target)
            .collect::<BTreeSet<_>>();
        for target in targets {
            let Some(bundled) = crate::bundled::source(&target) else {
                continue;
            };
            if !loaded.insert(target) {
                continue;
            }
            let source = SourceFile::new(
                next_source_id,
                std::path::PathBuf::from(format!("<terrane>/{}", bundled.path)),
                bundled.text.to_owned(),
            );
            next_source_id = next_source_id.saturating_add(1);
            units.push(parse_unit(
                &source,
                bundled.path.to_owned(),
                Some(bundled.namespace),
                package.prelude,
                true,
            )?);
        }
        index += 1;
    }
    apply_projected_method_contracts(&mut units, projection);
    Ok(units)
}
pub(super) fn apply_projected_method_contracts(
    units: &mut [SemanticUnit],
    projection: &crate::projection::Projection,
) {
    for unit in units {
        if !unit.namespace.starts_with("/deps/") {
            continue;
        }
        for contract in &mut unit.functions {
            let Some(owner) = contract.owner.as_deref() else {
                continue;
            };
            let Some(method) = projection.method(&unit.namespace, owner, &contract.name) else {
                continue;
            };
            contract.throws = true;
            contract.mutates_receiver = matches!(
                method.receiver,
                Some(crate::projection::Receiver::MutableBorrow)
            );
            contract.consumes_receiver =
                matches!(method.receiver, Some(crate::projection::Receiver::Move));
        }
    }
}

pub(super) fn dependency_projection(
    package: &Package,
) -> Result<crate::projection::Projection, SemanticFailure> {
    crate::projection::resolve(&package.root, &package.rust_dependencies).map_err(|error| {
        failure(
            &package.units[0].source,
            "S2028",
            error.message,
            Span::new(package.units[0].source.id(), 0, 0),
        )
    })
}

/// Builds the complete namespace tree, then resolves declarations and imports.
///
/// Semantic phases fail at the first diagnostic in deterministic package and source
/// order. Unlike independently discoverable manifest errors, later semantic errors can
/// depend on declarations or imports that an earlier error prevented from assembling.
///
/// # Errors
/// Returns the first source-oriented lexer, parser, namespace, scope, or import failure.
#[expect(
    clippy::too_many_lines,
    reason = "semantic phase orchestration remains linear and order-sensitive"
)]
pub fn analyze(package: &Package) -> Result<SemanticPackage, SemanticFailure> {
    let projection = dependency_projection(package)?;
    let mut units = parse_units(package, &projection)?;
    for unit in &mut units {
        unit.comparable_foreign_objects = projection
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .filter(|item| {
                matches!(
                    item.kind,
                    crate::projection::ProjectedKind::Enum {
                        data_carrying: false,
                        comparable: true,
                    }
                )
            })
            .map(|item| ObjectIdentity::new(&item.namespace, &item.name))
            .collect();
    }
    validate_compiler_owned_names(&units)?;

    let mut namespaces = bootstrap_namespaces();
    for unit in &units {
        let bundled = unit.bundled;
        if !bundled
            && (unit.namespace == "/core"
                || unit.namespace.starts_with("/core/")
                || unit.namespace == "/deps"
                || unit.namespace.starts_with("/deps/")
                || crate::bundled::source(&unit.namespace).is_some())
        {
            let span = unit
                .tree
                .root
                .children
                .iter()
                .find(|node| node.kind == SyntaxKind::NamespaceDeclaration)
                .map_or(Span::new(unit.source.id(), 0, 0), |node| node.span);
            return Err(failure(
                &unit.source,
                "S2017",
                format!(
                    "cannot declare into compiler-owned namespace `{}`",
                    unit.namespace
                ),
                span,
            ));
        }
        namespaces.entry(unit.namespace.clone()).or_default();
    }

    let mut imports = Vec::new();
    let mut globals = BTreeMap::<String, Symbol>::new();
    for unit in &units {
        collect_unit(unit, &mut namespaces, &mut globals, &mut imports)?;
    }
    let discovered_imports = units
        .iter()
        .map(imports_in_tree)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    for import in discovered_imports.iter().filter(|import| !import.bundled) {
        for capability in namespace_capabilities(&import.target) {
            if !package.profile.allows(capability) {
                return Err(failure(
                    &import.source,
                    "S2032",
                    format!(
                        "profile `{}` forbids capability `{capability}` required by `{}` imported by `{}`",
                        package.profile.name, import.target, import.namespace
                    ),
                    import.span,
                ));
            }
        }
    }
    for import in &discovered_imports {
        let Some(dependency) = import
            .target
            .strip_prefix("/deps/")
            .and_then(|path| path.split('/').next())
        else {
            continue;
        };
        if !package
            .rust_dependencies
            .iter()
            .any(|declared| declared.name.replace('_', "-") == dependency)
        {
            return Err(failure(
                &import.source,
                "S2027",
                format!("Rust dependency `{dependency}` is not declared in `package.toml`"),
                import.span,
            ));
        }
        if projection.item(&import.target, &import.object).is_none() {
            if let Some(removed) = projection
                .removed
                .iter()
                .find(|removed| removed.namespace == import.target && removed.name == import.object)
            {
                return Err(failure(
                    &import.source,
                    "S2031",
                    format!(
                        "Rust dependency member `{}` in `{}` was removed when the projected dependency changed from version `{}` to `{}`",
                        import.object,
                        import.target,
                        removed.previous_version,
                        removed.current_version
                    ),
                    import.span,
                ));
            }
            let reason = projection.dependencies.iter().find_map(|dependency| {
                dependency.declined.iter().find_map(|declined| {
                    (crate::projection::namespace_for_rust_path(dependency, &declined.rust_path)
                        == import.target
                        && declined.rust_path.rsplit("::").next() == Some(import.object.as_str()))
                    .then_some(declined.reason.as_str())
                })
            });
            let message = reason.map_or_else(
                || {
                    format!(
                        "Rust dependency projection has no member `{}` in `{}`",
                        import.object, import.target
                    )
                },
                |reason| {
                    format!(
                        "Rust dependency member `{}` in `{}` is not projected: {reason}",
                        import.object, import.target
                    )
                },
            );
            return Err(failure(&import.source, "S2029", message, import.span));
        }
    }
    let prelude_bindings = if package.prelude {
        bootstrap_prelude()
    } else {
        BTreeMap::new()
    };
    let mut import_warnings =
        resolve_imports(imports, &mut namespaces, &globals, &prelude_bindings)?;
    for unit in &mut units {
        unit.scopes = collect_lexical_scopes(unit, &namespaces, &globals, &prelude_bindings)?;
        import_warnings.extend(
            unit.scopes
                .iter()
                .flat_map(|scope| scope.import_warnings.iter().cloned()),
        );
    }
    let descriptor_constructs = bootstrap_descriptor_constructs();

    let mut semantic = SemanticPackage {
        identity: package.identity.clone(),
        prelude: package.prelude,
        reflection: package.reflection,
        executor: package.executor,
        profile: package.profile.clone(),
        namespaces,
        globals,
        prelude_bindings,
        descriptor_constructs,
        units,
        projection,
        binding_events: BTreeMap::new(),
        import_warnings,
        bootstrap_version: BOOTSTRAP_VERSION,
    };
    validate_initializer_dependencies(&semantic)?;
    validate_references(&semantic)?;
    analyze_types(&mut semantic)?;
    validate_error_clauses(&semantic)?;
    validate_moves(&semantic)?;
    validate_reference_origins(&semantic)?;
    validate_referenced_replacements(&semantic)?;
    infer_throwing_effects(&mut semantic)?;
    validate_constant_reassignment(&semantic)?;
    validate_global_definite_assignment(&semantic)?;
    record_binding_mutability(&mut semantic);
    validate_calls(&semantic)?;
    validate_definite_assignment(&semantic)?;
    record_binding_events(&mut semantic);
    validate_suspension_ownership(&semantic)?;
    validate_task_consumption(&semantic)?;
    let unreachable_units = validate_control_flow(&semantic)?;
    for (unit, unreachable_spans) in semantic.units.iter_mut().zip(unreachable_units) {
        unit.unreachable_spans = unreachable_spans;
        unit.evaluation_steps = collect_evaluation_steps(&unit.source, &unit.tree.root);
    }
    Ok(semantic)
}

pub(super) fn object_implements_identity(object: &ObjectContract, target: &str) -> bool {
    object
        .interfaces
        .iter()
        .any(|interface| interface.qualified() == target)
}

pub(super) fn identity_implements(package: &SemanticPackage, identity: &str, target: &str) -> bool {
    package.units.iter().any(|unit| {
        unit.objects.iter().any(|object| {
            package
                .namespaces
                .values()
                .flat_map(|namespace| namespace.symbols.values())
                .any(|symbol| {
                    symbol.identity == identity && symbol.declaration_span == Some(object.span)
                })
                && object_implements_identity(object, target)
        })
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "error validation keeps throw, catch, and finally rules in one ordered traversal"
)]
pub(super) fn validate_error_clauses(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    #[expect(
        clippy::too_many_lines,
        reason = "the recursive visitor validates the complete structured-error boundary"
    )]
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        in_catch: bool,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::ThrowStatement {
            if node.children.is_empty() {
                if !in_catch {
                    return Err(failure(
                        &unit.source,
                        "T0020",
                        "bare `throw` is only valid inside a catch clause",
                        node.span,
                    ));
                }
            } else {
                let thrown = &node.children[0];
                let symbol = package.resolve_name_at(
                    unit,
                    thrown.span.start,
                    node_text(&unit.source, thrown.children.first().unwrap_or(thrown)),
                );
                let standard = symbol.is_some_and(|symbol| symbol.kind == SymbolKind::ErrorObject);
                let value_type = infer_value_type(unit, thrown, &unit.typed_bindings)?;
                let object_name = match &value_type {
                    Some(ValueType::Descriptor(name)) => Some(name.as_str()),
                    Some(ValueType::Object(identity)) => Some(identity.name.as_str()),
                    _ if thrown.kind == SyntaxKind::CallExpression => thrown
                        .children
                        .first()
                        .filter(|callee| callee.kind == SyntaxKind::Name)
                        .map(|callee| node_text(&unit.source, callee)),
                    _ => None,
                };
                let user_throwable = object_name
                    .and_then(|name| {
                        package.resolve_name_at(
                            unit,
                            thrown.span.start,
                            name.rsplit_once("::").map_or(name, |(_, local)| local),
                        )
                    })
                    .is_some_and(|symbol| {
                        identity_implements(package, &symbol.identity, "/core/errors::throwable")
                    });
                if !standard && !user_throwable {
                    return Err(failure(
                        &unit.source,
                        "T0021",
                        "thrown values must implement `throwable`",
                        thrown.span,
                    ));
                }
            }
        }
        if node.kind == SyntaxKind::TryStatement {
            let mut caught = BTreeSet::new();
            let mut catches_all = false;
            for clause in node
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::CatchClause)
            {
                if let Some(alias) = clause
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::CatchBinding)
                {
                    return Err(failure(
                        &unit.source,
                        "T0027",
                        "catch aliases are unavailable until error values expose source-level members",
                        alias.span,
                    ));
                }
                let Some(descriptor) = clause
                    .children
                    .first()
                    .filter(|child| child.kind == SyntaxKind::Name)
                else {
                    if catches_all {
                        return Err(failure(
                            &unit.source,
                            "T0022",
                            "catch-all clause is unreachable",
                            clause.span,
                        ));
                    }
                    catches_all = true;
                    continue;
                };
                let name = node_text(&unit.source, descriptor);
                let symbol = package.resolve_name_at(unit, descriptor.span.start, name);
                let valid = symbol.is_some_and(|symbol| {
                    symbol.kind == SymbolKind::ErrorObject
                        || (symbol.kind == SymbolKind::Interface
                            && symbol.identity == "/core/errors::throwable")
                        || (matches!(symbol.kind, SymbolKind::Class | SymbolKind::TypeDescriptor)
                            && identity_implements(
                                package,
                                &symbol.identity,
                                "/core/errors::throwable",
                            ))
                });
                if !valid {
                    return Err(failure(
                        &unit.source,
                        "T0021",
                        format!("`{name}` is not a throwable descriptor"),
                        descriptor.span,
                    ));
                }
                let identity = &symbol.expect("validated error symbol").identity;
                if catches_all || !caught.insert(identity.clone()) {
                    return Err(failure(
                        &unit.source,
                        "T0022",
                        format!("catch clause for `{name}` is unreachable"),
                        clause.span,
                    ));
                }
                catches_all = identity == "/core/errors::throwable";
            }
        }
        for child in &node.children {
            let child_in_catch = in_catch || node.kind == SyntaxKind::CatchClause;
            visit(package, unit, child, child_in_catch)?;
        }
        Ok(())
    }

    for unit in &package.units {
        visit(package, unit, &unit.tree.root, false)?;
    }
    Ok(())
}

pub(super) fn populate_namespace_function_contracts(package: &mut SemanticPackage) {
    let namespaces = package
        .units
        .iter()
        .map(|unit| unit.namespace.clone())
        .collect::<Vec<_>>();
    let functions = package
        .units
        .iter()
        .map(|unit| unit.functions.clone())
        .collect::<Vec<_>>();
    for (unit, namespace) in package.units.iter_mut().zip(&namespaces) {
        unit.functions = namespaces
            .iter()
            .zip(&functions)
            .filter(|(candidate, _)| *candidate == namespace)
            .flat_map(|(_, functions)| functions.iter().cloned())
            .collect();
    }
}

pub(super) fn populate_object_aliases(package: &mut SemanticPackage) {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| unit.objects.iter())
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for unit in &mut package.units {
        let mut aliases = package
            .namespaces
            .get(&unit.namespace)
            .into_iter()
            .flat_map(|namespace| &namespace.symbols)
            .chain(
                unit.scopes
                    .iter()
                    .flat_map(|scope| &scope.symbols)
                    .flat_map(|(name, symbols)| symbols.iter().map(move |symbol| (name, symbol))),
            )
            .filter_map(|(visible_name, symbol)| {
                let span = symbol.declaration_span?;
                matches!(
                    symbol.kind,
                    SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                )
                .then(|| contracts.get(&(span.file, span.start, span.end)))
                .flatten()
                .cloned()
                .map(|mut contract| {
                    contract.name.clone_from(visible_name);
                    contract
                })
            })
            .collect::<Vec<_>>();
        aliases.retain(|alias| {
            !unit
                .objects
                .iter()
                .any(|contract| contract.name == alias.name)
        });
        unit.objects.extend(aliases);
    }
}

pub(super) fn populate_function_aliases(package: &mut SemanticPackage) {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| unit.functions.iter())
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for unit in &mut package.units {
        let mut aliases = BTreeMap::new();
        let mut contracts_by_span = BTreeMap::new();
        for namespace_name in namespace_chain(&unit.namespace) {
            let Some(namespace) = package.namespaces.get(&namespace_name) else {
                continue;
            };
            for (visible_name, symbol) in &namespace.symbols {
                let Some(span) = symbol.declaration_span else {
                    continue;
                };
                if symbol.kind != SymbolKind::Function || !visible_from(symbol, &unit.namespace) {
                    continue;
                }
                let key = (span.file, span.start, span.end);
                if let Some(contract) = contracts.get(&key) {
                    aliases
                        .entry(visible_name.clone())
                        .or_insert_with(|| contract.clone());
                    contracts_by_span
                        .entry(key)
                        .or_insert_with(|| contract.clone());
                }
            }
        }
        for symbol in unit
            .scopes
            .iter()
            .flat_map(|scope| scope.symbols.values())
            .flatten()
            .filter(|symbol| symbol.kind == SymbolKind::Function)
        {
            let Some(span) = symbol.declaration_span else {
                continue;
            };
            let key = (span.file, span.start, span.end);
            if let Some(contract) = contracts.get(&key) {
                contracts_by_span
                    .entry(key)
                    .or_insert_with(|| contract.clone());
            }
        }
        unit.function_aliases = aliases;
        unit.function_contracts_by_span = contracts_by_span;
    }
}

pub(super) fn resolved_function_contract<'a>(
    unit: &'a SemanticUnit,
    name: &str,
    offset: usize,
) -> Option<&'a FunctionContract> {
    lexical_scope_chain(unit, offset)
        .find_map(|scope| {
            let symbol = scope.symbols.get(name)?.iter().rev().find(|symbol| {
                symbol.kind == SymbolKind::Function
                    && symbol.binding_span.is_none_or(|span| span.end <= offset)
            })?;
            let span = symbol.declaration_span?;
            unit.function_contracts_by_span
                .get(&(span.file, span.start, span.end))
        })
        .or_else(|| unit.function_aliases.get(name))
}

pub(super) fn populate_function_type_dependencies(package: &mut SemanticPackage) {
    let objects = package
        .units
        .iter()
        .flat_map(|unit| unit.objects.iter())
        .map(|object| (object.identity.clone(), object.clone()))
        .collect::<BTreeMap<_, _>>();
    let methods = package
        .units
        .iter()
        .flat_map(|unit| unit.functions.iter())
        .filter_map(|method| {
            method
                .owner
                .as_ref()
                .map(|owner| ((method.span.file, owner.clone()), method.clone()))
        })
        .fold(
            BTreeMap::<(u32, String), Vec<FunctionContract>>::new(),
            |mut methods, (key, method)| {
                methods.entry(key).or_default().push(method);
                methods
            },
        );
    for unit in &mut package.units {
        let mut queue = unit
            .function_aliases
            .values()
            .chain(unit.function_contracts_by_span.values())
            .filter_map(|contract| match &contract.return_type {
                Some(ValueType::Object(identity)) => Some(identity.clone()),
                _ => None,
            })
            .chain(
                unit.objects
                    .iter()
                    .filter(|object| {
                        object.name != object.identity.name
                            || object.identity.namespace != unit.namespace
                    })
                    .map(|object| object.identity.clone()),
            )
            .collect::<Vec<_>>();
        let mut visited = BTreeSet::new();
        while let Some(key) = queue.pop() {
            if !visited.insert(key.clone()) {
                continue;
            }
            let Some(object) = objects.get(&key) else {
                continue;
            };
            for field in &object.fields {
                if let ValueType::Object(name) = &field.value_type {
                    queue.push(name.clone());
                }
            }
            let object_methods = methods
                .get(&(object.span.file, object.name.clone()))
                .cloned()
                .unwrap_or_default();
            for method in &object_methods {
                if let Some(ValueType::Object(name)) = &method.return_type {
                    queue.push(name.clone());
                }
                for parameter in &method.parameters {
                    if let Some(ValueType::Object(name)) = &parameter.value_type {
                        queue.push(name.clone());
                    }
                }
            }
            if !unit
                .objects
                .iter()
                .any(|candidate| candidate.name == object.name)
            {
                unit.objects.push(object.clone());
            }
            for method in object_methods {
                if !unit
                    .functions
                    .iter()
                    .any(|candidate| candidate.span == method.span && candidate.name == method.name)
                {
                    unit.functions.push(method);
                }
            }
        }
    }
}

impl SemanticPackage {
    #[must_use]
    pub fn symbol(&self, namespace: &str, name: &str) -> Option<&Symbol> {
        self.namespaces.get(namespace)?.symbols.get(name)
    }

    #[must_use]
    pub fn resolve_name(&self, namespace: &str, name: &str) -> Option<&Symbol> {
        namespace_chain(namespace)
            .find_map(|path| {
                self.symbol(&path, name).filter(|symbol| {
                    visible_from(symbol, namespace)
                        && (symbol.kind != SymbolKind::Binding
                            || symbol.constant
                            || symbol.global
                            || symbol.namespace == namespace)
                })
            })
            .or_else(|| {
                self.globals
                    .get(name)
                    .filter(|symbol| visible_from(symbol, namespace))
            })
            .or_else(|| self.symbol("/core/types", name))
            .or_else(|| self.prelude_bindings.get(name))
    }

    #[must_use]
    pub fn resolve_name_at<'a>(
        &'a self,
        unit: &'a SemanticUnit,
        offset: usize,
        name: &str,
    ) -> Option<&'a Symbol> {
        let mut scopes = lexical_scope_chain(unit, offset).peekable();
        let inside_lexical_scope = scopes.peek().is_some();
        scopes
            .find_map(|scope| {
                scope
                    .symbols
                    .get(name)?
                    .iter()
                    .rev()
                    .find(|symbol| symbol.binding_span.is_none_or(|span| span.end <= offset))
            })
            .or_else(|| {
                self.resolve_name(&unit.namespace, name)
                    .filter(|symbol| !inside_lexical_scope || symbol.available_in_function_body())
            })
    }

    #[must_use]
    pub fn is_lexical_replacement(&self, unit: &SemanticUnit, span: Span, name: &str) -> bool {
        let Some(current) = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.name == name && binding.span == span)
        else {
            return false;
        };
        let current_scope = lexical_scope_index_at(unit, current.span.start);
        lexical_scope_chain(unit, span.start).any(|scope| {
            scope.symbols.get(name).is_some_and(|symbols| {
                symbols
                    .iter()
                    .any(|symbol| symbol.declaration_span == Some(span))
                    && symbols.iter().any(|symbol| {
                        symbol.declaration_span.is_some_and(|prior| {
                            prior.start < span.start
                                && lexical_scope_index_at(unit, prior.start) == current_scope
                        })
                    })
            })
        })
    }
}
