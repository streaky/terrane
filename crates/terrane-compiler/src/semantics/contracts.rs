use super::prelude::*;

pub(crate) fn descriptor_expression_type(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Option<ScalarType> {
    let name = node_text(&unit.source, node).trim();
    match node.kind {
        SyntaxKind::Name | SyntaxKind::TypeExpression => unit
            .descriptor_alias_at(name, node.span.start)
            .or_else(|| package.descriptor_constructs.get(name)?.descriptor_type())
            .or_else(|| {
                node.children
                    .first()
                    .and_then(|child| descriptor_expression_type(package, unit, child))
            }),
        _ => None,
    }
}

pub(crate) fn descriptor_expression_category(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Option<TypeCategory> {
    let name = node_text(&unit.source, node).trim();
    match node.kind {
        SyntaxKind::Name | SyntaxKind::TypeExpression => package
            .resolve_name_at(unit, node.span.start, name)
            .and_then(Symbol::descriptor_category)
            .or_else(|| {
                node.children
                    .first()
                    .and_then(|child| descriptor_expression_category(package, unit, child))
            }),
        _ => None,
    }
}

pub(super) fn collect_type_declarations(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    aliases: &mut BTreeMap<String, Vec<DescriptorAlias>>,
    functions: &mut Vec<FunctionContract>,
    scope: Option<Span>,
) -> Result<(), SemanticFailure> {
    if let Some((name, alias)) = descriptor_alias(unit, node, aliases, scope) {
        aliases.entry(name).or_default().push(alias);
    }
    if is_function_node(node) {
        let visible = visible_descriptor_aliases(aliases, unit.source.id(), node.span.start);
        functions.push(analyze_function_contract(unit, node, &visible)?);
    }
    let child_scope = is_function_node(node).then_some(node.span).or(scope);
    for child in &node.children {
        collect_type_declarations(unit, child, aliases, functions, child_scope)?;
    }
    Ok(())
}

pub(super) fn descriptor_alias(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    aliases: &BTreeMap<String, Vec<DescriptorAlias>>,
    scope: Option<Span>,
) -> Option<(String, DescriptorAlias)> {
    if !matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        || node
            .children
            .iter()
            .any(|child| child.kind == SyntaxKind::TypeExpression)
    {
        return None;
    }
    let name = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)?;
    let initializer = node.children.last()?;
    let descriptor_name = node_text(&unit.source, initializer).trim();
    if descriptor_name == "none" {
        return None;
    }
    let value_type = match initializer.kind {
        SyntaxKind::Name => {
            visible_descriptor_aliases(aliases, unit.source.id(), initializer.span.start)
                .get(descriptor_name)
                .copied()
        }
        _ => None,
    }?;
    Some((
        node_text(&unit.source, name).to_owned(),
        DescriptorAlias {
            visible_from: node.span.end,
            scope,
            value_type,
        },
    ))
}

#[expect(
    clippy::too_many_lines,
    reason = "binding collection preserves declaration and scope ordering in one traversal"
)]
pub(super) fn collect_typed_bindings(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    visible_bindings: &mut Vec<TypedBinding>,
    bindings: &mut Vec<TypedBinding>,
    scope: Option<Span>,
) -> Result<(), SemanticFailure> {
    if matches!(
        node.kind,
        SyntaxKind::ClassDeclaration
            | SyntaxKind::InterfaceDeclaration
            | SyntaxKind::TraitDeclaration
    ) {
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
                collect_typed_bindings(
                    unit,
                    method,
                    visible_bindings,
                    bindings,
                    Some(method.span),
                )?;
            }
        }
        return Ok(());
    }
    if is_function_node(node) {
        let contract = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed function declaration must have a semantic contract");
        let mut parameter_bindings = contract
            .parameters
            .iter()
            .filter_map(|parameter| {
                parameter.value_type.clone().map(|value_type| TypedBinding {
                    name: parameter.name.clone(),
                    span: parameter.span,
                    visible_from: parameter.span.start,
                    scope: Some(node.span),
                    value_type,
                    destination_arms: Vec::new(),
                    storage_type: None,
                    mutable: false,
                })
            })
            .collect::<Vec<_>>();
        if let Some(owner) = &contract.owner {
            parameter_bindings.push(TypedBinding {
                name: "self".to_owned(),
                span: implicit_receiver_span(node, "self"),
                visible_from: node.span.start,
                scope: Some(node.span),
                value_type: ValueType::Descriptor(owner.clone()),
                destination_arms: Vec::new(),
                storage_type: None,
                mutable: false,
            });
            if !contract.is_static {
                parameter_bindings.push(TypedBinding {
                    name: "this".to_owned(),
                    span: implicit_receiver_span(node, "this"),
                    visible_from: node.span.start,
                    scope: Some(node.span),
                    value_type: ValueType::Object(ObjectIdentity::new(&unit.namespace, owner)),
                    destination_arms: Vec::new(),
                    storage_type: None,
                    mutable: true,
                });
            }
        }
        let mut function_bindings = visible_bindings.clone();
        function_bindings.extend(parameter_bindings.iter().cloned());
        bindings.extend(parameter_bindings);
        for child in &node.children {
            collect_typed_bindings(
                unit,
                child,
                &mut function_bindings,
                bindings,
                Some(node.span),
            )?;
        }
        return Ok(());
    }
    if let [target, collection, block] = node.children.as_slice()
        && node.kind == SyntaxKind::ForStatement
        && target.kind == SyntaxKind::ForTarget
    {
        collect_typed_bindings(unit, collection, visible_bindings, bindings, scope)?;
        let item_type = infer_value_type(unit, collection, visible_bindings)?
            .and_then(iterable_item_type)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0016",
                    "collection iteration requires an iterable value",
                    collection.span,
                )
            })?;
        let loop_bindings =
            iteration_target_bindings(unit, target, collection.span.end, block.span, item_type)?;
        bindings.extend(loop_bindings.iter().cloned());
        let mut visible_loop_bindings = visible_bindings.clone();
        visible_loop_bindings.extend(loop_bindings);
        collect_typed_bindings(
            unit,
            block,
            &mut visible_loop_bindings,
            bindings,
            Some(block.span),
        )?;
        return Ok(());
    }
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
        let prior_len = visible_bindings.len();
        analyze_binding_node(unit, node, visible_bindings, scope)?;
        bindings.extend_from_slice(&visible_bindings[prior_len..]);
    }
    for child in &node.children {
        let child_scope = (child.kind == SyntaxKind::Block)
            .then_some(child.span)
            .or(scope);
        collect_typed_bindings(unit, child, visible_bindings, bindings, child_scope)?;
    }
    Ok(())
}

pub(super) fn mutates_object_receiver(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
    if node.kind == SyntaxKind::Assignment
        && let Some(target) = node.children.first()
        && target.kind == SyntaxKind::MemberExpression
        && target.children.first().is_some_and(|receiver| {
            receiver.kind == SyntaxKind::Name && node_text(&unit.source, receiver) == "this"
        })
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| mutates_object_receiver(unit, child))
}

#[expect(
    clippy::too_many_lines,
    reason = "callable signature analysis keeps parameter and result contracts in source order"
)]
pub(super) fn analyze_function_contract(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    aliases: &BTreeMap<String, ScalarType>,
) -> Result<FunctionContract, SemanticFailure> {
    let name_node = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name);
    if node.kind == SyntaxKind::FunctionDeclaration && name_node.is_none() {
        return Err(failure(
            &unit.source,
            "T0004",
            "function requires a name",
            node.span,
        ));
    }
    let return_type = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::TypeExpression)
        .map(|type_node| declared_value_type(unit, type_node, aliases))
        .transpose()?;
    let mut parameters = Vec::new();
    let mut optional_seen = false;
    if let Some(parameter_list) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::ParameterList)
    {
        for parameter in &parameter_list.children {
            let Some(parameter_name) = parameter
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
            else {
                continue;
            };
            let type_node = parameter
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::TypeExpression);
            let value_type = type_node
                .map(|type_node| declared_value_type(unit, type_node, aliases))
                .transpose()?;
            let default = parameter.children.iter().find(|child| {
                child.span != parameter_name.span && child.kind != SyntaxKind::TypeExpression
            });
            let optional = default.is_some();
            if optional {
                optional_seen = true;
            } else if optional_seen {
                return Err(failure(
                    &unit.source,
                    "T0005",
                    "required parameters must precede optional parameters",
                    parameter.span,
                ));
            }
            if let (Some(expected), Some(default)) = (value_type.clone(), default) {
                let actual =
                    infer_value_type(unit, default, &unit.typed_bindings)?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0006",
                            "parameter default has no value",
                            default.span,
                        )
                    })?;
                validate_value_destination(
                    &unit.source,
                    &unit.objects,
                    node_text(&unit.source, parameter_name),
                    expected,
                    actual,
                    default,
                    "T0006",
                )?;
            }
            parameters.push(ParameterContract {
                name: node_text(&unit.source, parameter_name).to_owned(),
                span: parameter.span,
                value_type,
                optional,
                mutable: false,
            });
        }
    }
    if name_node.is_some_and(|name| node_text(&unit.source, name) == "main")
        && object_name_containing(unit, node.span).is_none()
        && !parameters.is_empty()
    {
        return Err(failure(
            &unit.source,
            "T0078",
            "program entrypoint `main` cannot declare parameters",
            node.span,
        ));
    }
    let mut thrown_types = Vec::new();
    for child in &node.children {
        if child.kind == SyntaxKind::EffectClause {
            if let Some(type_node) = child
                .children
                .iter()
                .find(|part| part.kind == SyntaxKind::TypeExpression)
            {
                thrown_types.push(declared_value_type(unit, type_node, aliases)?);
            }
        }
    }
    let is_async = node.children.iter().any(|child| {
        child.kind == SyntaxKind::DeclarationQualifier && node_text(&unit.source, child) == "async"
    });
    let is_static = node.children.iter().any(|child| {
        child.kind == SyntaxKind::DeclarationQualifier && node_text(&unit.source, child) == "static"
    });
    let throws = !thrown_types.is_empty();
    let exported = node.children.iter().any(|child| {
        child.kind == SyntaxKind::Visibility && node_text(&unit.source, child) == "public"
    });
    Ok(FunctionContract {
        name: name_node.map_or_else(
            || format!("closure@{}", node.span.start),
            |name| node_text(&unit.source, name).to_owned(),
        ),
        span: node.span,
        owner: (node.kind == SyntaxKind::FunctionDeclaration)
            .then(|| object_name_containing(unit, node.span))
            .flatten(),
        parameters,
        captures: Vec::new(),
        return_type,
        thrown_types,
        escaping_throwables: BTreeSet::new(),
        throws,
        is_async,
        is_static,
        mutates_receiver: mutates_object_receiver(unit, node),
        consumes_receiver: false,
        exported,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "the fixed-point call graph and its local traversals form one auditable effect analysis"
)]
pub(super) fn infer_throwing_effects(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    type FunctionKey = (u32, usize, usize);

    fn key(span: Span) -> FunctionKey {
        (span.file, span.start, span.end)
    }

    fn direct_errors(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> BTreeSet<String> {
        if node.kind == SyntaxKind::FunctionDeclaration {
            return BTreeSet::new();
        }
        if node.kind == SyntaxKind::ThrowStatement {
            return node
                .children
                .first()
                .and_then(|error| {
                    let descriptor = if error.kind == SyntaxKind::CallExpression {
                        error.children.first().unwrap_or(error)
                    } else {
                        error
                    };
                    let descriptor = if descriptor.kind == SyntaxKind::ConstructionExpression {
                        descriptor.children.first().unwrap_or(descriptor)
                    } else {
                        descriptor
                    };
                    package.resolve_name_at(
                        unit,
                        descriptor.span.start,
                        node_text(&unit.source, descriptor),
                    )
                })
                .map(|symbol| symbol.identity.clone())
                .into_iter()
                .collect();
        }
        if node.kind == SyntaxKind::TryStatement {
            let mut errors = node
                .children
                .first()
                .map_or_else(BTreeSet::new, |block| direct_errors(package, unit, block));
            let mut clauses_finished = false;
            for child in node.children.iter().skip(1) {
                if child.kind == SyntaxKind::CatchClause {
                    let descriptor = child
                        .children
                        .first()
                        .filter(|candidate| candidate.kind == SyntaxKind::Name);
                    if let Some(descriptor) = descriptor
                        && let Some(symbol) = package.resolve_name_at(
                            unit,
                            descriptor.span.start,
                            node_text(&unit.source, descriptor),
                        )
                    {
                        if symbol.identity == "/core/errors::throwable" {
                            errors.clear();
                        } else {
                            errors.remove(&symbol.identity);
                        }
                    } else {
                        errors.clear();
                    }
                    if let Some(block) = child.children.last() {
                        errors.extend(direct_errors(package, unit, block));
                    }
                    clauses_finished = true;
                } else if child.kind == SyntaxKind::FinallyClause {
                    if let Some(block) = child.children.last() {
                        errors.extend(direct_errors(package, unit, block));
                    }
                } else if !clauses_finished {
                    errors.extend(direct_errors(package, unit, child));
                }
            }
            return errors;
        }
        node.children
            .iter()
            .flat_map(|child| direct_errors(package, unit, child))
            .collect()
    }

    fn integer_coercion_can_fail(unit: &SemanticUnit, node: &SyntaxNode) -> bool {
        let Some(callee) = node.children.first() else {
            return false;
        };
        let Some((source_node, CoercionPolicy::Default)) =
            integer_coercion_call(&unit.source, callee)
        else {
            return false;
        };
        let Ok(Some(ValueType::Scalar(source))) =
            infer_receiver_value_type(unit, source_node, &unit.typed_bindings)
        else {
            return false;
        };
        let Some(destination_node) = node
            .children
            .get(1)
            .and_then(|arguments| arguments.children.first())
            .and_then(|argument| argument.children.last())
        else {
            return false;
        };
        let Some(destination) = unit.descriptor_alias_at(
            node_text(&unit.source, destination_node),
            destination_node.span.start,
        ) else {
            return false;
        };
        if destination == ScalarType::Int {
            return false;
        }
        let Some(destination_bounds) = scalar_integer_bounds(destination) else {
            return false;
        };
        let Some(source_bounds) = scalar_integer_bounds(source) else {
            return source == ScalarType::Int;
        };
        source_bounds.0 < destination_bounds.0 || source_bounds.1 > destination_bounds.1
    }

    fn fixed_integer_bits(ty: ScalarType) -> Option<u16> {
        match ty {
            ScalarType::Int8 | ScalarType::Uint8 => Some(8),
            ScalarType::Int16 | ScalarType::Uint16 => Some(16),
            ScalarType::Int32 | ScalarType::Uint32 => Some(32),
            ScalarType::Int64 | ScalarType::Uint64 => Some(64),
            ScalarType::Int128 | ScalarType::Uint128 => Some(128),
            _ => None,
        }
    }

    fn numeric_conversion_can_fail(source: ScalarType, destination: ScalarType) -> bool {
        if source == destination {
            return false;
        }
        if destination == ScalarType::Int {
            return matches!(source, ScalarType::Float32 | ScalarType::Float64);
        }
        if source == ScalarType::Int {
            return destination.is_integer()
                || matches!(destination, ScalarType::Float32 | ScalarType::Float64);
        }
        if source.is_integer() && destination.is_integer() {
            let Some(source_bounds) = scalar_integer_bounds(source) else {
                return false;
            };
            let Some(destination_bounds) = scalar_integer_bounds(destination) else {
                return false;
            };
            return source_bounds.0 < destination_bounds.0
                || source_bounds.1 > destination_bounds.1;
        }
        if source == ScalarType::Float32 && destination == ScalarType::Float64 {
            return false;
        }
        if source.is_integer() && matches!(destination, ScalarType::Float32 | ScalarType::Float64) {
            let exact_bits = if destination == ScalarType::Float32 {
                16
            } else {
                32
            };
            return fixed_integer_bits(source).is_some_and(|bits| bits > exact_bits);
        }
        matches!(source, ScalarType::Float32 | ScalarType::Float64)
            && (destination.is_integer() || destination == ScalarType::Float32)
    }

    fn destination_conversion_can_fail(
        unit: &SemanticUnit,
        expected: &ValueType,
        value: &SyntaxNode,
    ) -> bool {
        if value.kind == SyntaxKind::GroupExpression
            && let [grouped] = value.children.as_slice()
        {
            return destination_conversion_can_fail(unit, expected, grouped);
        }
        if let ValueType::Optional(inner) = expected {
            return destination_conversion_can_fail(unit, inner, value);
        }
        if let ValueType::Scalar(destination) = expected {
            if contextual_constant(&unit.source, value, *destination).is_some() {
                return false;
            }
            let Ok(Some(ValueType::Scalar(source))) =
                infer_value_type(unit, value, &unit.typed_bindings)
            else {
                return false;
            };
            return numeric_conversion_can_fail(source, *destination);
        }
        let [callee, arguments] = value.children.as_slice() else {
            return false;
        };
        if value.kind != SyntaxKind::CallExpression {
            return false;
        }
        let Some(identity) = collection_constructor_identity(unit, callee, &unit.typed_bindings)
        else {
            return false;
        };
        let name = identity
            .strip_prefix("/core/collections::")
            .unwrap_or(identity);
        match (name, expected) {
            (
                "list" | "tuple" | "set" | "unordered-set",
                ValueType::List(item)
                | ValueType::Tuple(item, _)
                | ValueType::Set(item)
                | ValueType::UnorderedSet(item),
            ) => arguments.children.iter().any(|argument| {
                destination_conversion_can_fail(
                    unit,
                    &item.value_type(),
                    argument.children.last().unwrap_or(argument),
                )
            }),
            ("entry", ValueType::Entry(key, entry_value)) => {
                let [key_argument, value_argument] = arguments.children.as_slice() else {
                    return false;
                };
                destination_conversion_can_fail(
                    unit,
                    &key.value_type(),
                    key_argument.children.last().unwrap_or(key_argument),
                ) || destination_conversion_can_fail(
                    unit,
                    &entry_value.value_type(),
                    value_argument.children.last().unwrap_or(value_argument),
                )
            }
            (
                "map" | "unordered-map",
                ValueType::Map(key, map_value) | ValueType::UnorderedMap(key, map_value),
            ) => arguments.children.iter().any(|argument| {
                let value_node = argument.children.last().unwrap_or(argument);
                if argument.children.len() < 2
                    && matches!(
                        infer_value_type(unit, value_node, &unit.typed_bindings),
                        Ok(Some(ValueType::Entry(_, _)))
                    )
                {
                    destination_conversion_can_fail(
                        unit,
                        &ValueType::Entry(key.clone(), map_value.clone()),
                        value_node,
                    )
                } else {
                    destination_conversion_can_fail(unit, &map_value.value_type(), value_node)
                }
            }),
            _ => false,
        }
    }

    fn call_argument_errors(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> BTreeSet<String> {
        let mut errors = BTreeSet::new();
        let [callee, arguments] = node.children.as_slice() else {
            return errors;
        };
        if node.kind != SyntaxKind::CallExpression {
            return errors;
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && let Ok(Some(receiver_type)) =
                infer_receiver_value_type(unit, receiver, &unit.typed_bindings)
        {
            let argument_can_fail = |index: usize, expected: &ValueType| {
                arguments.children.get(index).is_some_and(|argument| {
                    destination_conversion_can_fail(
                        unit,
                        expected,
                        argument.children.last().unwrap_or(argument),
                    )
                })
            };
            let conversion_can_fail = match (receiver_type, node_text(&unit.source, member)) {
                (ValueType::List(item), "append")
                | (
                    ValueType::Set(item) | ValueType::UnorderedSet(item),
                    "add" | "contains" | "remove",
                ) => argument_can_fail(0, &item.value_type()),
                (ValueType::List(item), "set") => argument_can_fail(1, &item.value_type()),
                (ValueType::Map(key, value) | ValueType::UnorderedMap(key, value), "set") => {
                    argument_can_fail(0, &key.value_type())
                        || argument_can_fail(1, &value.value_type())
                }
                _ => false,
            };
            if conversion_can_fail {
                errors.insert("/core/errors::integer-conversion-overflow".to_owned());
            }
        }
        let Some(parameters) = function_parameters(package, unit, callee) else {
            return errors;
        };
        let mut positional = 0;
        for argument in &arguments.children {
            let name = argument
                .children
                .first()
                .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1)
                .map(|name| node_text(&unit.source, name));
            let parameter = if let Some(name) = name {
                parameters.iter().find(|parameter| parameter.name == name)
            } else {
                let parameter = parameters.get(positional);
                positional += 1;
                parameter
            };
            let value = argument.children.last().unwrap_or(argument);
            if parameter
                .and_then(|parameter| parameter.value_type.as_ref())
                .is_some_and(|expected| destination_conversion_can_fail(unit, expected, value))
            {
                errors.insert("/core/errors::integer-conversion-overflow".to_owned());
            }
        }
        errors
    }

    fn local_builtin_errors(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> BTreeSet<String> {
        let mut errors = call_argument_errors(package, unit, node);
        if node.kind == SyntaxKind::BinaryExpression
            && let [left, right] = node.children.as_slice()
            && matches!(
                unit.inferred_value_type(node),
                Some(ValueType::Scalar(ScalarType::Int))
            )
            && matches!(
                unit.source.text()[left.span.end..right.span.start].trim(),
                "/" | "%"
            )
        {
            errors.insert("/core/errors::division-by-zero".to_owned());
        }
        if node.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = node.children.as_slice()
            && matches!(
                node_text(&unit.source, member),
                "round" | "floor" | "ceiling" | "truncate"
            )
            && matches!(
                infer_receiver_value_type(unit, receiver, &unit.typed_bindings),
                Ok(Some(ValueType::Scalar(
                    ScalarType::Float32 | ScalarType::Float64
                )))
            )
        {
            errors.insert("/core/errors::integer-conversion-overflow".to_owned());
        }
        let destination = if node.kind == SyntaxKind::Binding {
            node.children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .and_then(|name| {
                    unit.typed_bindings
                        .iter()
                        .find(|binding| binding.span == name.span)
                        .map(|binding| binding.value_type.clone())
                })
                .zip(node.children.iter().find(|child| {
                    !matches!(
                        child.kind,
                        SyntaxKind::Name
                            | SyntaxKind::Visibility
                            | SyntaxKind::DeclarationQualifier
                            | SyntaxKind::TypeExpression
                    )
                }))
        } else if node.kind == SyntaxKind::Assignment {
            let [target, value] = node.children.as_slice() else {
                return errors;
            };
            infer_value_type(unit, target, &unit.typed_bindings)
                .ok()
                .flatten()
                .map(|expected| (expected, value))
        } else {
            None
        };
        if destination.is_some_and(|(expected, value)| {
            destination_conversion_can_fail(unit, &expected, value)
        }) {
            errors.insert("/core/errors::integer-conversion-overflow".to_owned());
        }
        if node.kind == SyntaxKind::ReturnStatement
            && let Some(value) = node.children.first()
            && let Some(function_span) = unit
                .enclosing_function_spans
                .get(&node.span.start)
                .copied()
                .flatten()
            && let Some(expected) = unit
                .functions
                .iter()
                .find(|contract| contract.span == function_span)
                .and_then(|contract| contract.return_type.as_ref())
            && destination_conversion_can_fail(unit, expected, value)
        {
            errors.insert("/core/errors::integer-conversion-overflow".to_owned());
        }
        errors
    }
    fn escaping_errors(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        inferred: &BTreeMap<FunctionKey, BTreeSet<String>>,
    ) -> BTreeSet<String> {
        if node.kind == SyntaxKind::FunctionDeclaration {
            return BTreeSet::new();
        }
        if node.kind == SyntaxKind::ThrowStatement {
            return direct_errors(package, unit, node);
        }
        let local_errors = local_builtin_errors(package, unit, node);
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
        {
            let member_name = node_text(&unit.source, member);
            let receiver_type = infer_receiver_value_type(unit, receiver, &unit.typed_bindings)
                .ok()
                .flatten();
            let mut errors = if integer_coercion_can_fail(unit, node) {
                BTreeSet::from(["/core/errors::integer-conversion-overflow".to_owned()])
            } else if let Some(ValueType::Object(object)) = receiver_type {
                unit.functions
                    .iter()
                    .find(|contract| {
                        contract.owner.as_deref() == Some(object.name.as_str())
                            && contract.name == member_name
                    })
                    .and_then(|contract| inferred.get(&key(contract.span)))
                    .cloned()
                    .unwrap_or_default()
            } else if member_name == "decode" {
                BTreeSet::from(["/core/errors::decode-error".to_owned()])
            } else {
                BTreeSet::new()
            };
            errors.extend(local_errors);
            errors.extend(
                node.children
                    .iter()
                    .flat_map(|child| escaping_errors(package, unit, child, inferred)),
            );
            return errors;
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
            && let Some(symbol) =
                package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
            && symbol.kind == SymbolKind::Function
            && let Some(span) = symbol.declaration_span
        {
            let mut errors = inferred.get(&key(span)).cloned().unwrap_or_default();
            errors.extend(local_errors);
            errors.extend(
                node.children
                    .iter()
                    .skip(1)
                    .flat_map(|argument| escaping_errors(package, unit, argument, inferred)),
            );
            return errors;
        }
        if node.kind == SyntaxKind::CallExpression
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
            && matches!(
                infer_value_type(unit, callee, &unit.typed_bindings),
                Ok(Some(ValueType::Function(_, _)))
            )
        {
            let mut errors = BTreeSet::from(["/core/errors::throwable".to_owned()]);
            errors.extend(local_errors);
            errors.extend(
                node.children
                    .iter()
                    .skip(1)
                    .flat_map(|argument| escaping_errors(package, unit, argument, inferred)),
            );
            return errors;
        }
        if node.kind == SyntaxKind::TryStatement {
            let mut errors = node.children.first().map_or_else(BTreeSet::new, |block| {
                escaping_errors(package, unit, block, inferred)
            });
            let mut clauses_finished = false;
            for child in node.children.iter().skip(1) {
                if child.kind == SyntaxKind::CatchClause {
                    let descriptor = child
                        .children
                        .first()
                        .filter(|candidate| candidate.kind == SyntaxKind::Name);
                    if let Some(descriptor) = descriptor
                        && let Some(symbol) = package.resolve_name_at(
                            unit,
                            descriptor.span.start,
                            node_text(&unit.source, descriptor),
                        )
                    {
                        if symbol.identity == "/core/errors::throwable" {
                            errors.clear();
                        } else {
                            errors.remove(&symbol.identity);
                        }
                    } else {
                        errors.clear();
                    }
                    if let Some(block) = child.children.last() {
                        errors.extend(escaping_errors(package, unit, block, inferred));
                    }
                    clauses_finished = true;
                } else if child.kind == SyntaxKind::FinallyClause {
                    if let Some(block) = child.children.last() {
                        errors.extend(escaping_errors(package, unit, block, inferred));
                    }
                } else if !clauses_finished {
                    errors.extend(escaping_errors(package, unit, child, inferred));
                }
            }
            return errors;
        }
        let mut errors = local_errors;
        errors.extend(
            node.children
                .iter()
                .flat_map(|child| escaping_errors(package, unit, child, inferred)),
        );
        errors
    }

    let mut inferred_throwables = BTreeMap::<FunctionKey, BTreeSet<String>>::new();
    let mut bodies = BTreeMap::<FunctionKey, (usize, SyntaxNode)>::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        for contract in unit
            .functions
            .iter()
            .filter(|contract| contract.span.file == unit.source.id())
        {
            let Some(function) = find_node_by_span(&unit.tree.root, contract.span) else {
                continue;
            };
            let function_key = key(function.span);
            bodies.insert(function_key, (unit_index, function.clone()));
            let function_throwables = function
                .children
                .iter()
                .flat_map(|child| direct_errors(package, unit, child))
                .collect();
            inferred_throwables.insert(function_key, function_throwables);
        }
    }

    loop {
        let mut changed = false;
        for function in bodies.keys() {
            let (unit_index, body) = &bodies[function];
            let unit = &package.units[*unit_index];
            let combined_throwables = body
                .children
                .iter()
                .flat_map(|child| escaping_errors(package, unit, child, &inferred_throwables))
                .collect::<BTreeSet<_>>();
            if combined_throwables != inferred_throwables[function] {
                inferred_throwables.insert(*function, combined_throwables);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    for unit in &package.units {
        for contract in unit
            .functions
            .iter()
            .filter(|contract| contract.span.file == unit.source.id())
        {
            let Some(ValueType::Object(bound)) = contract.thrown_types.first() else {
                continue;
            };
            for identity in inferred_throwables
                .get(&key(contract.span))
                .into_iter()
                .flatten()
            {
                let actual = identity
                    .rsplit_once("::")
                    .map_or(identity.as_str(), |(_, name)| name);
                let bound_identity = Some(bound.qualified());
                let compatible = bound_identity.as_deref().is_some_and(|bound_identity| {
                    bound_identity == "/core/errors::throwable"
                        || bound_identity == identity
                        || identity_implements(package, identity, bound_identity)
                });
                if !compatible {
                    return Err(failure(
                        &unit.source,
                        "T0027",
                        format!(
                            "`{actual}` may escape `{}` but does not satisfy its `throws {bound}` contract",
                            contract.name
                        ),
                        contract.span,
                    ));
                }
            }
        }
    }

    for unit in &mut package.units {
        for contract in &mut unit.functions {
            contract.escaping_throwables = inferred_throwables
                .get(&key(contract.span))
                .cloned()
                .unwrap_or_default();
            contract.throws = !contract.escaping_throwables.is_empty();
        }
    }
    Ok(())
}
