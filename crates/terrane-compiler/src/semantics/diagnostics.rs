use super::prelude::*;

pub(super) fn collect_loop_target_spans(
    node: &SyntaxNode,
    loop_targets: &mut BTreeSet<(u32, usize, usize)>,
) {
    if node.kind == SyntaxKind::ForTarget {
        loop_targets.extend(node.children.iter().map(|name| span_key(name.span)));
    }
    for child in &node.children {
        collect_loop_target_spans(child, loop_targets);
    }
}

pub(super) fn invalid_name_style_declarations(unit: &SemanticUnit) -> Vec<(&str, Span)> {
    let mut declarations = unit
        .typed_bindings
        .iter()
        .map(|binding| (binding.name.as_str(), binding.span))
        .chain(
            unit.functions
                .iter()
                .map(|function| (function.name.as_str(), function.span)),
        )
        .chain(
            unit.objects
                .iter()
                .map(|object| (object.name.as_str(), object.span)),
        )
        .collect::<Vec<_>>();
    declarations.sort_by_key(|(_, span)| (span.start, span.end));
    declarations.dedup_by_key(|(_, span)| (span.start, span.end));
    declarations.retain(|(text, _)| {
        !text.bytes().enumerate().all(|(index, byte)| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || (index > 0 && byte == b'-')
        })
    });
    declarations
}

pub(super) fn validate_compiler_owned_names(units: &[SemanticUnit]) -> Result<(), SemanticFailure> {
    for unit in units
        .iter()
        .filter(|unit| unit.bundled && !unit.namespace.starts_with("/deps/"))
    {
        if let Some((text, span)) = invalid_name_style_declarations(unit).into_iter().next() {
            return Err(failure(
                &unit.source,
                "S2018",
                format!("compiler-owned declaration `{text}` is not kebab-case"),
                span,
            ));
        }
    }
    Ok(())
}

pub(super) fn collect_name_style_warnings(unit: &SemanticUnit, warnings: &mut Vec<Diagnostic>) {
    for (text, span) in invalid_name_style_declarations(unit) {
        warnings.push(
            Diagnostic::warning(
                "S2018",
                format!("declared name `{text}` is not kebab-case"),
                span,
            )
            .with_help(
                "use kebab-case for Terrane-owned declarations; projected dependency names remain verbatim",
            ),
        );
    }
}

pub(super) fn union_arm_identity(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    text: &str,
    position: usize,
) -> String {
    if let Some(scalar) = unit.descriptor_alias_at(text, position) {
        return format!("scalar:{}", scalar.source_name());
    }
    package
        .resolve_name_at(unit, position, text)
        .filter(|symbol| {
            matches!(
                symbol.kind,
                SymbolKind::Class
                    | SymbolKind::Interface
                    | SymbolKind::Trait
                    | SymbolKind::ErrorObject
            )
        })
        .map_or_else(
            || format!("unresolved:{text}"),
            |symbol| format!("object:{}", symbol.identity),
        )
}

pub(super) fn collect_duplicate_union_arm_warnings(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    warnings: &mut Vec<Diagnostic>,
) {
    fn collect(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        warnings: &mut Vec<Diagnostic>,
    ) {
        if node.kind == SyntaxKind::UnionType {
            let mut seen = BTreeSet::new();
            for arm in &node.children {
                let text = node_text(&unit.source, arm).trim();
                let identity = union_arm_identity(package, unit, text, arm.span.start);
                if !seen.insert(identity) {
                    warnings.push(
                        Diagnostic::warning(
                            "W4003",
                            format!("union arm `{text}` duplicates an earlier arm"),
                            arm.span,
                        )
                        .with_help(
                            "remove the repeated arm; union arms are normalized by semantic identity",
                        ),
                    );
                }
            }
        }
        for child in &node.children {
            collect(package, unit, child, warnings);
        }
    }

    collect(package, unit, &unit.tree.root, warnings);
}

pub(crate) fn warnings(package: &SemanticPackage, lint_name_style: bool) -> Vec<Diagnostic> {
    let mut warnings = Vec::new();
    warnings.extend(package.import_warnings.iter().cloned());
    for unit in &package.units {
        if lint_name_style && !unit.bundled && !unit.namespace.starts_with("/deps/") {
            collect_name_style_warnings(unit, &mut warnings);
        }
        collect_duplicate_union_arm_warnings(package, unit, &mut warnings);
        let mut loop_targets = BTreeSet::new();
        collect_loop_target_spans(&unit.tree.root, &mut loop_targets);
        for binding in &unit.typed_bindings {
            if package
                .globals
                .values()
                .any(|symbol| symbol.declaration_span == Some(binding.span))
            {
                continue;
            }
            let parameter = unit.functions.iter().any(|contract| {
                contract
                    .parameters
                    .iter()
                    .any(|parameter| parameter.span == binding.span)
            });
            let loop_target = loop_targets.contains(&span_key(binding.span));
            let Some(events) = package.binding_events.get(&span_key(binding.span)) else {
                continue;
            };
            for (index, event) in events.iter().enumerate() {
                let BindingEvent::Write {
                    span: store_span, ..
                } = event
                else {
                    continue;
                };
                if binding_store_value_is_read(package, binding.span, *store_span) {
                    continue;
                }
                let later_store = events[index + 1..]
                    .iter()
                    .any(|event| matches!(event, BindingEvent::Write { .. }));
                let initial_store = *store_span == binding.span;
                let (code, message) = if initial_store && !later_store {
                    if parameter || loop_target {
                        continue;
                    }
                    ("W4001", format!("binding `{}` is never read", binding.name))
                } else if initial_store {
                    (
                        "W4002",
                        format!("initial value assigned to `{}` is never read", binding.name),
                    )
                } else {
                    (
                        "W4002",
                        format!("value assigned to `{}` is never read", binding.name),
                    )
                };
                warnings.push(Diagnostic::warning(code, message, *store_span));
            }
        }
    }
    warnings.sort_by_key(|diagnostic| {
        diagnostic
            .primary
            .map_or((u32::MAX, usize::MAX), |span| (span.file, span.start))
    });
    warnings
}

pub(super) fn object_method_mutates(
    package: &SemanticPackage,
    object_identity: &ObjectIdentity,
    method_name: &str,
) -> bool {
    fn contract_mutates(unit: &SemanticUnit, object_name: &str, method_name: &str) -> bool {
        if let Some(method) = unit.functions.iter().find(|method| {
            method.owner.as_deref() == Some(object_name) && method.name == method_name
        }) {
            return method.mutates_receiver;
        }
        unit.objects
            .iter()
            .find(|object| object.name == object_name)
            .and_then(|object| object.base.as_ref())
            .and_then(|base| unit.objects.iter().find(|object| object.identity == *base))
            .is_some_and(|base| contract_mutates(unit, &base.name, method_name))
    }

    if package.units.iter().any(|candidate| {
        candidate
            .objects
            .iter()
            .find(|object| object.identity == *object_identity)
            .is_some_and(|object| contract_mutates(candidate, &object.name, method_name))
    }) {
        return true;
    }

    package
        .projection
        .method(
            &object_identity.namespace,
            &object_identity.name,
            method_name,
        )
        .is_some_and(|method| {
            matches!(
                method.receiver,
                Some(crate::projection::Receiver::MutableBorrow)
            )
        })
}

pub(crate) fn binding_span_is_mutated(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    declaration_span: Span,
    initially_assigned: bool,
) -> bool {
    fn writes(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        declaration_span: Span,
        iterator_binding: bool,
        node: &SyntaxNode,
    ) -> usize {
        let resolves_to_binding = |target: &SyntaxNode| {
            target.kind == SyntaxKind::Name
                && !package.is_lexical_replacement(unit, node.span, node_text(&unit.source, target))
                && package
                    .resolve_name_at(unit, target.span.start, node_text(&unit.source, target))
                    .is_some_and(|symbol| symbol.declaration_span == Some(declaration_span))
        };
        let direct_write = matches!(
            node.kind,
            SyntaxKind::Assignment | SyntaxKind::PostfixExpression
        ) && node.span != declaration_span
            && node.children.first().is_some_and(|target| {
                resolves_to_binding(target)
                    || (matches!(
                        target.kind,
                        SyntaxKind::IndexExpression | SyntaxKind::MemberExpression
                    ) && target.children.first().is_some_and(resolves_to_binding))
            });
        let mutator_call = node.kind == SyntaxKind::CallExpression
            && node.children.first().is_some_and(|callee| {
                let [receiver, member] = callee.children.as_slice() else {
                    return false;
                };
                callee.kind == SyntaxKind::MemberExpression
                    && (matches!(
                        node_text(&unit.source, member),
                        "append" | "set" | "add" | "remove"
                    ) || matches!(
                        infer_value_type(unit, receiver, &unit.typed_bindings),
                        Ok(Some(ValueType::Object(object)))
                            if object_method_mutates(
                                package,
                                &object,
                                node_text(&unit.source, member)
                            )
                    ))
                    && resolves_to_binding(receiver)
            });
        let iterator_advance = iterator_binding
            && node.kind == SyntaxKind::ForStatement
            && node.children.get(1).is_some_and(resolves_to_binding);
        let writes_here = usize::from(direct_write || mutator_call || iterator_advance);
        writes_here
            + node
                .children
                .iter()
                .map(|child| writes(package, unit, declaration_span, iterator_binding, child))
                .sum::<usize>()
    }

    let iterator_binding = unit.typed_bindings.iter().any(|binding| {
        binding.span == declaration_span && matches!(binding.value_type, ValueType::Iterator(_))
    });
    writes(
        package,
        unit,
        declaration_span,
        iterator_binding,
        &unit.tree.root,
    ) > usize::from(!initially_assigned)
}

pub(super) fn add_private_host_bindings<'a>(
    namespaces: &mut BTreeMap<String, Namespace>,
    path: &str,
    group: &str,
    bindings: impl IntoIterator<Item = &'a str>,
) {
    let namespace = namespaces.entry(path.to_owned()).or_default();
    for intrinsic in bindings {
        let local_name = format!("host-{intrinsic}");
        let previous = namespace.symbols.insert(
            local_name.clone(),
            Symbol {
                identity: format!("{path}::{local_name}"),
                lowering_identity: Some(format!("intrinsic:{group}::{intrinsic}")),
                name: local_name.clone(),
                namespace: path.to_owned(),
                visibility: Visibility::Private,
                global: false,
                constant: false,
                kind: SymbolKind::Function,
                declaration_span: None,

                binding_span: None,
            },
        );
        assert!(
            previous.is_none(),
            "duplicate private host binding `{path}::{local_name}`"
        );
    }
}

pub(super) fn namespace_with_objects<'a>(
    path: &str,
    names: impl IntoIterator<Item = &'a str>,
    kind: SymbolKind,
) -> Namespace {
    let symbols = names
        .into_iter()
        .map(|name| (name.to_owned(), compiler_owned_object(path, name, kind)))
        .collect();
    Namespace { symbols }
}

pub(super) fn compiler_owned_object(path: &str, name: &str, kind: SymbolKind) -> Symbol {
    Symbol {
        identity: format!("{path}::{name}"),
        lowering_identity: None,
        name: name.to_owned(),
        namespace: path.to_owned(),
        visibility: Visibility::Public,
        global: false,
        constant: false,
        kind,
        declaration_span: None,

        binding_span: None,
    }
}

pub(super) fn normalize_declared_path(components: &[&str]) -> Option<String> {
    if components.is_empty()
        || components
            .iter()
            .any(|component| matches!(*component, "/" | "..") || component.is_empty())
    {
        return None;
    }
    Some(format!("/{}", components.join("/")))
}

pub(super) fn resolve_path(current: &str, anchored: bool, path: &[&str]) -> Option<String> {
    let mut components = if anchored {
        Vec::new()
    } else {
        current
            .trim_start_matches('/')
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
    };
    for component in path {
        if *component == "/" {
            continue;
        }
        if *component == ".." {
            components.pop()?;
        } else {
            components.push(component);
        }
    }
    Some(format!("/{}", components.join("/")))
}

pub(super) fn declaration_name(node: &SyntaxNode, source: &SourceFile) -> Option<String> {
    node.children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
        .map(|child| node_text(source, child).to_owned())
}

pub(super) fn unary_operator_text(unit: &SemanticUnit, node: &SyntaxNode) -> Option<String> {
    node.children
        .iter()
        .find(|child| child.kind == SyntaxKind::UnaryOperator)
        .map(|operator| {
            node_text(&unit.source, operator)
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
}

pub(super) fn transparent_value_type(value_type: Option<ValueType>) -> Option<ValueType> {
    value_type.map(|value_type| match value_type {
        ValueType::Reference(item) | ValueType::SharedReference(item) => item.value_type(),
        value_type => value_type,
    })
}

pub(super) fn node_text<'a>(source: &'a SourceFile, node: &SyntaxNode) -> &'a str {
    &source.text()[node.span.start..node.span.end]
}

pub(super) fn failure(
    source: &SourceFile,
    code: &'static str,
    message: impl Into<String>,
    span: Span,
) -> SemanticFailure {
    SemanticFailure {
        source: source.clone(),
        diagnostics: vec![Diagnostic::error(code, message, span)],
    }
}
