use super::prelude::*;

#[expect(
    clippy::too_many_lines,
    reason = "move provenance and its control-flow join remain one auditable analysis"
)]
pub(super) fn validate_moves(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn binding_at(unit: &SemanticUnit, name: &str, position: usize) -> Option<usize> {
        unit.typed_bindings
            .iter()
            .enumerate()
            .rev()
            .find(|(_, binding)| {
                binding.name == name && binding.is_visible_at(unit.source.id(), position)
            })
            .map(|(index, _)| index)
    }

    fn resource_binding(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        binding: usize,
        resource_objects: &BTreeSet<(u32, usize, usize)>,
    ) -> bool {
        match &unit.typed_bindings[binding].value_type {
            ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
            ValueType::Object(name) => resolved_object_span(package, name)
                .is_some_and(|span| resource_objects.contains(&span_key(span))),
            _ => false,
        }
    }

    fn method_consumes_receiver(
        package: &SemanticPackage,
        object_identity: &ObjectIdentity,
        method_name: &str,
    ) -> bool {
        method_contract(package, object_identity, method_name, false)
            .is_some_and(|method| method.consumes_receiver)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "move traversal keeps scope transitions and diagnostics in one ordered dispatch"
    )]
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        moved: &mut BTreeSet<usize>,
        declaration_name: bool,
        resource_objects: &BTreeSet<(u32, usize, usize)>,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last()
            && unary_operator_text(unit, node).as_deref() == Some("move")
            && operand.kind == SyntaxKind::Name
        {
            let name = node_text(&unit.source, operand);
            if let Some(binding) = binding_at(unit, name, operand.span.start) {
                if !moved.insert(binding) {
                    return Err(failure(
                        &unit.source,
                        "T0058",
                        format!("`{name}` was already moved and is unavailable until rebound"),
                        operand.span,
                    ));
                }
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member, ..] = callee.children.as_slice()
            && receiver.kind != SyntaxKind::Name
            && let Ok(Some(ValueType::Object(object_name))) =
                infer_value_type(unit, receiver, &unit.typed_bindings)
            && method_consumes_receiver(package, &object_name, node_text(&unit.source, member))
            && resolved_object_span(package, &object_name)
                .is_some_and(|span| resource_objects.contains(&span_key(span)))
        {
            return Err(failure(
                &unit.source,
                "T0101",
                "a resource-consuming call requires a named binding; move the member into a binding first",
                receiver.span,
            ));
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member, ..] = callee.children.as_slice()
            && receiver.kind == SyntaxKind::Name
            && matches!(
                infer_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(ValueType::Object(object_name)))
                    if method_consumes_receiver(
                        package,
                        &object_name,
                        node_text(&unit.source, member),
                    )
            )
            && let Some(binding) =
                binding_at(unit, node_text(&unit.source, receiver), receiver.span.start)
            && resource_binding(package, unit, binding, resource_objects)
        {
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.insert(binding);
            return Ok(());
        }
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
            && let Some(parameters) = function_parameters(package, unit, callee)
        {
            for (argument, parameter) in arguments.children.iter().zip(parameters) {
                let Some(expected) = parameter.value_type.as_ref() else {
                    continue;
                };
                let expects_resource = match expected {
                    ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
                    ValueType::Object(name) => resolved_object_span(package, name)
                        .is_some_and(|span| resource_objects.contains(&span_key(span))),
                    _ => false,
                };
                let value = argument.children.last().unwrap_or(argument);
                if expects_resource
                    && matches!(
                        value.kind,
                        SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
                    )
                    && !value.children.first().is_some_and(|receiver| {
                        receiver.kind == SyntaxKind::Name
                            && node_text(&unit.source, receiver) == "this"
                    })
                {
                    return Err(failure(
                        &unit.source,
                        "T0101",
                        "resource transfer requires a named binding",
                        value.span,
                    ));
                }
            }
            let transferred = arguments
                .children
                .iter()
                .zip(parameters)
                .filter_map(|(argument, parameter)| {
                    parameter
                        .value_type
                        .as_ref()
                        .filter(|value_type| {
                            matches!(
                                value_type,
                                ValueType::PlatformStreamHandle
                                    | ValueType::PlatformResourceHandle
                                    | ValueType::Object(_)
                            )
                        })
                        .and_then(|_| argument.children.last())
                        .filter(|value| value.kind == SyntaxKind::Name)
                        .and_then(|value| {
                            binding_at(unit, node_text(&unit.source, value), value.span.start)
                        })
                        .filter(|binding| {
                            resource_binding(package, unit, *binding, resource_objects)
                        })
                })
                .collect::<Vec<_>>();
            for child in &node.children {
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            moved.extend(transferred);
            return Ok(());
        }
        if node.kind == SyntaxKind::Name && !declaration_name {
            let name = node_text(&unit.source, node);
            if let Some(binding) = binding_at(unit, name, node.span.start)
                && moved.contains(&binding)
            {
                return Err(failure(
                    &unit.source,
                    "T0058",
                    format!("`{name}` was moved and is unavailable until rebound"),
                    node.span,
                ));
            }
            return Ok(());
        }
        if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
            let transferred = node
                .children
                .last()
                .filter(|initializer| initializer.kind == SyntaxKind::Name)
                .and_then(|initializer| {
                    binding_at(
                        unit,
                        node_text(&unit.source, initializer),
                        initializer.span.start,
                    )
                })
                .filter(|binding| resource_binding(package, unit, *binding, resource_objects));
            let mut skipped_name = false;
            for child in &node.children {
                if !skipped_name && child.kind == SyntaxKind::Name {
                    skipped_name = true;
                    continue;
                }
                visit(package, unit, child, moved, false, resource_objects)?;
            }
            if let Some(binding) = transferred {
                moved.insert(binding);
            }
            if node.kind == SyntaxKind::Assignment
                && let Some(name) = node
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)
                    .map(|name| node_text(&unit.source, name))
                && let Some(binding) = binding_at(unit, name, node.span.start)
            {
                moved.remove(&binding);
            }
            return Ok(());
        }
        if node.kind == SyntaxKind::IfStatement {
            let mut entry = moved.clone();
            for child in &node.children {
                if !matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause) {
                    visit(package, unit, child, &mut entry, false, resource_objects)?;
                }
            }
            let mut branches = Vec::new();
            let mut has_else = false;
            for child in &node.children {
                if matches!(child.kind, SyntaxKind::Block | SyntaxKind::ElseClause) {
                    has_else |= child.kind == SyntaxKind::ElseClause;
                    let mut branch = entry.clone();
                    visit(package, unit, child, &mut branch, false, resource_objects)?;
                    branches.push(branch);
                }
            }
            if !has_else {
                branches.push(entry);
            }
            moved.clear();
            moved.extend(branches.into_iter().flatten());
            return Ok(());
        }
        if matches!(
            node.kind,
            SyntaxKind::WhileStatement | SyntaxKind::ForStatement
        ) {
            let mut entry = moved.clone();
            let body = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Block);
            for child in &node.children {
                if Some(child) != body {
                    visit(package, unit, child, &mut entry, false, resource_objects)?;
                }
            }
            if let Some(body) = body {
                let mut after_iteration = entry.clone();
                visit(
                    package,
                    unit,
                    body,
                    &mut after_iteration,
                    false,
                    resource_objects,
                )?;
                // Validate the back edge: the next iteration starts with the first iteration's
                // move state, even though only the may-execute-once state leaves the loop.
                let mut next_iteration = after_iteration.clone();
                visit(
                    package,
                    unit,
                    body,
                    &mut next_iteration,
                    false,
                    resource_objects,
                )?;
                entry.extend(after_iteration);
            }
            *moved = entry;
            return Ok(());
        }
        for child in &node.children {
            visit(package, unit, child, moved, false, resource_objects)?;
        }
        Ok(())
    }

    let resource_objects = package
        .units
        .iter()
        .flat_map(|unit| unit.objects.iter())
        .filter(|object| object.resource_owning)
        .map(|object| span_key(object.span))
        .collect();
    for unit in &package.units {
        visit(
            package,
            unit,
            &unit.tree.root,
            &mut BTreeSet::new(),
            false,
            &resource_objects,
        )?;
    }
    Ok(())
}

pub(super) fn validate_reference_origins(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit(unit: &SemanticUnit, node: &SyntaxNode) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::UnaryExpression
            && let Some(operand) = node.children.last()
            && unary_operator_text(unit, node).as_deref() == Some("ref")
        {
            let valid_origin = operand.kind == SyntaxKind::Name
                && unit.typed_bindings.iter().rev().any(|binding| {
                    binding.name == node_text(&unit.source, operand)
                        && binding.is_visible_at(unit.source.id(), operand.span.start)
                        && binding.scope.is_some()
                        && find_node_by_span(&unit.tree.root, binding.span)
                            .is_some_and(|origin| origin.kind == SyntaxKind::Binding)
                });
            if !valid_origin {
                return Err(failure(
                    &unit.source,
                    "T0064",
                    "`ref` requires a named binding with reference-backed storage",
                    operand.span,
                ));
            }
        }
        if node.kind == SyntaxKind::ReturnStatement
            && let Some(value) = node.children.first()
            && matches!(
                infer_value_type(unit, value, &unit.typed_bindings)?,
                Some(ValueType::Reference(_))
            )
        {
            return Err(failure(
                &unit.source,
                "T0068",
                "a non-owning reference cannot escape its proven source lifetime",
                value.span,
            ));
        }
        for child in &node.children {
            visit(unit, child)?;
        }
        Ok(())
    }

    for unit in &package.units {
        visit(unit, &unit.tree.root)?;
    }
    Ok(())
}

pub(super) fn validate_referenced_replacements(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn collect_origins(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        observer: Option<Span>,
        origins: &mut Vec<(Span, Span)>,
    ) {
        let observer = if node.kind == SyntaxKind::Binding {
            unit.typed_bindings
                .iter()
                .find(|binding| binding.span == node.span)
                .map(|binding| binding.span)
                .or(observer)
        } else {
            observer
        };
        if node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("ref")
            && let Some(observer) = observer
            && let Some(operand) = node.children.last()
            && operand.kind == SyntaxKind::Name
            && let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == node_text(&unit.source, operand)
                    && binding.is_visible_at(unit.source.id(), operand.span.start)
            })
        {
            origins.push((binding.span, observer));
        }
        for child in &node.children {
            collect_origins(unit, child, observer, origins);
        }
    }

    fn first_use_after(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        declaration: Span,
        position: usize,
    ) -> Option<Span> {
        if node.kind == SyntaxKind::Name
            && node.span.start > position
            && package
                .resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
                .and_then(|symbol| symbol.declaration_span)
                == Some(declaration)
        {
            return Some(node.span);
        }
        node.children
            .iter()
            .find_map(|child| first_use_after(package, unit, child, declaration, position))
    }

    for unit in &package.units {
        let mut origins = Vec::new();
        collect_origins(unit, &unit.tree.root, None, &mut origins);
        for replacement in &unit.typed_bindings {
            let previous = unit
                .typed_bindings
                .iter()
                .filter(|binding| {
                    binding.name == replacement.name
                        && binding.scope == replacement.scope
                        && binding.visible_from < replacement.visible_from
                })
                .max_by_key(|binding| binding.visible_from);
            if let Some(previous) = previous
                && previous.value_type != replacement.value_type
                && let Some(use_span) = origins
                    .iter()
                    .filter(|(origin, _)| *origin == previous.span)
                    .find_map(|(_, observer)| {
                        first_use_after(
                            package,
                            unit,
                            &unit.tree.root,
                            *observer,
                            replacement.span.end,
                        )
                    })
            {
                return Err(failure(
                    &unit.source,
                    "T0059",
                    format!(
                        "a reference to the previous `{}` value is unavailable after replacement",
                        replacement.name
                    ),
                    use_span,
                ));
            }
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "reference validation keeps ownership forms in one ordered syntax traversal"
)]
pub(super) fn validate_references(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    fn visit(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        _callable_position: bool,
    ) -> Result<(), SemanticFailure> {
        match node.kind {
            SyntaxKind::Name => {
                let name = node_text(&unit.source, node);
                let resolved = package.resolve_name_at(unit, node.span.start, name);
                let implicit_receiver = is_implicit_object_receiver(unit, node.span.start, name);
                if resolved.is_none()
                    && !package.descriptor_constructs.contains_key(name)
                    && !implicit_receiver
                {
                    if namespace_chain(&unit.namespace)
                        .filter_map(|path| package.namespaces.get(&path))
                        .filter_map(|namespace| namespace.symbols.get(name))
                        .chain(package.globals.get(name))
                        .any(|symbol| {
                            symbol.kind == SymbolKind::Binding
                                && !symbol.available_in_function_body()
                        })
                    {
                        return Err(namespace_variable_reference_failure(
                            &unit.source,
                            name,
                            node.span,
                        ));
                    }
                    return Err(failure(
                        &unit.source,
                        "S2013",
                        format!("unresolved name `{name}`"),
                        node.span,
                    ));
                }
            }
            SyntaxKind::NamespaceDeclaration
            | SyntaxKind::ImportDeclaration
            | SyntaxKind::ParameterList
            | SyntaxKind::Parameter
            | SyntaxKind::ForTarget
            | SyntaxKind::TypeExpression
            | SyntaxKind::UnionType
            | SyntaxKind::PrefixType
            | SyntaxKind::AppliedType
            | SyntaxKind::FunctionType => {}
            SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::FunctionDeclaration
            | SyntaxKind::AnonymousFunction
            | SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration => {
                let mut declaration_name_skipped = false;
                for child in &node.children {
                    if !declaration_name_skipped && child.kind == SyntaxKind::Name {
                        declaration_name_skipped = true;
                        continue;
                    }
                    visit(package, unit, child, false)?;
                }
            }
            SyntaxKind::CatchClause => {
                if let Some(descriptor) = node.children.first() {
                    visit(package, unit, descriptor, false)?;
                }
                if let Some(block) = node.children.last()
                    && block.kind == SyntaxKind::Block
                {
                    visit(package, unit, block, false)?;
                }
            }
            SyntaxKind::Argument => {
                for (index, child) in node.children.iter().enumerate() {
                    if index == 0 && node.children.len() > 1 && child.kind == SyntaxKind::Name {
                        continue;
                    }
                    visit(package, unit, child, false)?;
                }
            }
            SyntaxKind::MemberExpression
            | SyntaxKind::StaticMemberExpression
            | SyntaxKind::ConstructionExpression => {
                if let Some(receiver) = node.children.first() {
                    visit(package, unit, receiver, false)?;
                }
            }
            SyntaxKind::CallExpression => {
                for (index, child) in node.children.iter().enumerate() {
                    visit(package, unit, child, index == 0)?;
                }
            }
            _ => {
                for child in &node.children {
                    visit(package, unit, child, false)?;
                }
            }
        }
        Ok(())
    }

    for unit in &package.units {
        for node in &unit.tree.root.children {
            visit(package, unit, node, false)?;
        }
    }
    Ok(())
}
pub(super) fn namespace_variable_reference_failure(
    source: &SourceFile,
    name: &str,
    span: Span,
) -> SemanticFailure {
    SemanticFailure {
        source: source.clone(),
        diagnostics: vec![
            Diagnostic::error(
                "S2026",
                format!("namespace variable `{name}` cannot cross a function boundary"),
                span,
            )
            .with_help(format!(
                "pass `{name}` as a parameter or return it from a function"
            )),
        ],
    }
}

pub(super) fn namespace_variable_import_failure(
    source: &SourceFile,
    name: &str,
    span: Span,
) -> SemanticFailure {
    SemanticFailure {
        source: source.clone(),
        diagnostics: vec![
            Diagnostic::error(
                "S2026",
                format!("namespace variable `{name}` cannot be imported outside its namespace"),
                span,
            )
            .with_help(format!(
                "import a function that reads `{name}` and returns its value instead"
            )),
        ],
    }
}

pub(super) fn binding_initializer(node: &SyntaxNode) -> Option<&SyntaxNode> {
    let name_index = node
        .children
        .iter()
        .position(|child| child.kind == SyntaxKind::Name)?;
    node.children
        .iter()
        .enumerate()
        .rev()
        .find(|(index, child)| {
            *index != name_index
                && !matches!(
                    child.kind,
                    SyntaxKind::TypeExpression
                        | SyntaxKind::Visibility
                        | SyntaxKind::DeclarationQualifier
                )
        })
        .map(|(_, child)| child)
}

#[expect(
    clippy::too_many_lines,
    reason = "the dependency graph construction and its diagnostics are one ordered validation pass"
)]
pub(super) fn validate_initializer_dependencies(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    type Key = (u32, usize, usize);

    fn key(span: Span) -> Key {
        (span.file, span.start, span.end)
    }

    fn collect_reads(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        reads: &mut Vec<(Key, Span)>,
        functions: &mut BTreeSet<Key>,
    ) {
        if node.kind == SyntaxKind::Name {
            if let Some(symbol) =
                package.resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
                && let Some(span) = symbol.declaration_span
            {
                if symbol.kind == SymbolKind::Binding && !symbol.global {
                    reads.push((key(span), node.span));
                } else if symbol.kind == SymbolKind::Function && functions.insert(key(span)) {
                    for owner in &package.units {
                        if let Some(function) = find_node_by_span(&owner.tree.root, span) {
                            collect_reads(package, owner, function, reads, functions);
                            break;
                        }
                    }
                }
            }
            return;
        }
        if matches!(
            node.kind,
            SyntaxKind::NamespaceDeclaration
                | SyntaxKind::ImportDeclaration
                | SyntaxKind::Parameter
                | SyntaxKind::ForTarget
                | SyntaxKind::TypeExpression
                | SyntaxKind::UnionType
                | SyntaxKind::PrefixType
                | SyntaxKind::AppliedType
                | SyntaxKind::FunctionType
        ) {
            return;
        }
        for child in &node.children {
            collect_reads(package, unit, child, reads, functions);
        }
    }
    fn unresolved_name_span(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        name: &str,
    ) -> Option<Span> {
        if node.kind == SyntaxKind::Name
            && node_text(&unit.source, node) == name
            && package
                .resolve_name_at(unit, node.span.start, name)
                .is_none()
        {
            return Some(node.span);
        }
        node.children
            .iter()
            .find_map(|child| unresolved_name_span(package, unit, child, name))
    }
    fn validate_self_references(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Result<(), SemanticFailure> {
        if node.kind == SyntaxKind::Binding
            && let Some(initializer) = binding_initializer(node)
        {
            let declaration =
                declaration_from_syntax(unit, node).expect("ordinary binding has a name");
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            let direct_unresolved_self =
                unresolved_name_span(package, unit, initializer, &declaration.name);
            if let Some(span) = reads
                .iter()
                .find(|(dependency, _)| *dependency == key(node.span))
                .map(|(_, span)| *span)
                .or(direct_unresolved_self)
            {
                return Err(failure(
                    &unit.source,
                    "S2023",
                    format!(
                        "binding `{}` cannot reference itself in its initializer",
                        declaration.name
                    ),
                    span,
                ));
            }
        }
        for child in &node.children {
            validate_self_references(package, unit, child)?;
        }
        Ok(())
    }

    fn find_cycle(
        current: Key,
        edges: &BTreeMap<Key, Vec<(Key, Span)>>,
        path: &mut BTreeSet<Key>,
    ) -> Option<Span> {
        if !path.insert(current) {
            return None;
        }
        for &(dependency, span) in edges.get(&current).into_iter().flatten() {
            if path.contains(&dependency) {
                return Some(span);
            }
            if let Some(span) = find_cycle(dependency, edges, path) {
                return Some(span);
            }
        }
        path.remove(&current);
        None
    }
    for unit in &package.units {
        validate_self_references(package, unit, &unit.tree.root)?;
    }

    let mut edges = BTreeMap::<Key, Vec<(Key, Span)>>::new();
    for unit in &package.units {
        for node in &unit.tree.root.children {
            if node.kind != SyntaxKind::Binding {
                continue;
            }
            let Some(declaration) = declaration_from_syntax(unit, node) else {
                continue;
            };
            let Some(initializer) = binding_initializer(node) else {
                continue;
            };
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            if reads
                .iter()
                .any(|(dependency, _)| *dependency == key(node.span))
            {
                let span = reads
                    .iter()
                    .find(|(dependency, _)| *dependency == key(node.span))
                    .expect("checked self-reference")
                    .1;
                return Err(failure(
                    &unit.source,
                    "S2023",
                    format!(
                        "binding `{}` cannot reference itself in its initializer",
                        declaration.name
                    ),
                    span,
                ));
            }
            if !declaration.global {
                edges.entry(key(node.span)).or_default().extend(reads);
            }
        }
    }
    for unit in &package.units {
        for node in &unit.tree.root.children {
            if node.kind != SyntaxKind::Assignment {
                continue;
            }
            let Some(declaration) = declaration_from_syntax(unit, node) else {
                continue;
            };
            let name = node
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .expect("ordinary assignment has a name");
            if package
                .globals
                .get(&declaration.name)
                .is_some_and(|global| global.namespace == unit.namespace)
                && !declaration.global
            {
                return Err(SemanticFailure {
                    source: unit.source.clone(),
                    diagnostics: vec![
                        Diagnostic::error(
                            "S2021",
                            format!(
                                "plain namespace assignment cannot replace program-global binding `{}`",
                                declaration.name
                            ),
                            name.span,
                        )
                        .with_help(
                            "pass changing values through parameters and returns instead",
                        ),
                    ],
                });
            }
            let Some(target) = package.resolve_name_at(unit, name.span.start, &declaration.name)
            else {
                continue;
            };
            let Some(initializer) = binding_initializer(node) else {
                continue;
            };
            let Some(owner) = target.declaration_span else {
                continue;
            };
            let mut reads = Vec::new();
            collect_reads(package, unit, initializer, &mut reads, &mut BTreeSet::new());
            reads.retain(|(dependency, _)| *dependency != key(owner));
            edges.entry(key(owner)).or_default().extend(reads);
        }
    }
    for &start in edges.keys() {
        if let Some(span) = find_cycle(start, &edges, &mut BTreeSet::new()) {
            let source = package
                .units
                .iter()
                .find(|unit| unit.source.id() == span.file)
                .expect("dependency span belongs to a semantic unit");
            return Err(failure(
                &source.source,
                "S2024",
                "namespace binding initialization has a dependency cycle",
                span,
            ));
        }
    }
    Ok(())
}

pub(super) fn find_node_by_span(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    (node.span == span).then_some(node).or_else(|| {
        node.children
            .iter()
            .find_map(|child| find_node_by_span(child, span))
    })
}

pub(super) fn collect_evaluation_steps(
    source: &SourceFile,
    root: &SyntaxNode,
) -> Vec<EvaluationStep> {
    fn visit(
        source: &SourceFile,
        node: &SyntaxNode,
        conditional: bool,
        steps: &mut Vec<EvaluationStep>,
    ) {
        if node.kind == SyntaxKind::BinaryExpression
            && let [left, right] = node.children.as_slice()
        {
            visit(source, left, conditional, steps);
            let operator = source.text()[left.span.end..right.span.start].trim();
            let short_circuit = matches!(operator, "and" | "or");
            if short_circuit {
                steps.push(EvaluationStep {
                    kind: EvaluationKind::ShortCircuitRhs,
                    span: right.span,
                    conditional: true,
                });
            }
            visit(source, right, conditional || short_circuit, steps);
        } else {
            for child in &node.children {
                visit(source, child, conditional, steps);
            }
        }
        let kind = match node.kind {
            SyntaxKind::CallExpression => Some(EvaluationKind::Call),
            SyntaxKind::PostfixExpression => Some(EvaluationKind::PostfixUpdate),
            _ => None,
        };
        if let Some(kind) = kind {
            steps.push(EvaluationStep {
                kind,
                span: node.span,
                conditional,
            });
        }
    }

    let mut steps = Vec::new();
    visit(source, root, false, &mut steps);
    steps
}
