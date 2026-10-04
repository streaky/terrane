use super::prelude::*;

fn enclosing_parameters(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    declaration: &SyntaxNode,
    names: &mut BTreeSet<String>,
) {
    if node.span.start > declaration.span.start || node.span.end < declaration.span.end {
        return;
    }
    if (node.kind != declaration.kind || node.span != declaration.span)
        && matches!(
            node.kind,
            SyntaxKind::ClassDeclaration
                | SyntaxKind::InterfaceDeclaration
                | SyntaxKind::TraitDeclaration
                | SyntaxKind::EnumDeclaration
        )
        && let Some(list) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::TypeParameterList)
    {
        names.extend(
            list.children
                .iter()
                .filter_map(|parameter| {
                    parameter
                        .children
                        .iter()
                        .find(|part| part.kind == SyntaxKind::Name)
                })
                .map(|name| node_text(&unit.source, name).to_owned()),
        );
    }
    for child in &node.children {
        enclosing_parameters(unit, child, declaration, names);
    }
}
pub(super) fn generic_parameters(
    unit: &SemanticUnit,
    declaration: &SyntaxNode,
    aliases: &BTreeMap<String, ScalarType>,
) -> Result<Vec<GenericParameterContract>, SemanticFailure> {
    let Some(list) = declaration
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::TypeParameterList)
    else {
        return Ok(Vec::new());
    };
    let mut enclosing_names = BTreeSet::new();
    enclosing_parameters(unit, &unit.tree.root, declaration, &mut enclosing_names);
    let mut names = BTreeSet::new();
    list.children
        .iter()
        .map(|parameter| {
            let name_node = parameter
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::Name)
                .ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0001",
                        "type parameter requires a name",
                        parameter.span,
                    )
                })?;
            let name = node_text(&unit.source, name_node).to_owned();
            if enclosing_names.contains(&name) {
                return Err(failure(
                    &unit.source,
                    "T0001",
                    format!("type parameter `{name}` cannot shadow an enclosing type parameter"),
                    name_node.span,
                ));
            }
            if !names.insert(name.clone()) {
                return Err(failure(
                    &unit.source,
                    "T0001",
                    format!("type parameter `{name}` is declared more than once"),
                    name_node.span,
                ));
            }
            let bound = if let Some(node) = parameter
                .children
                .iter()
                .find(|child| child.kind == SyntaxKind::TypeExpression)
            {
                match super::types::declared_value_type(unit, node, aliases)? {
                    ValueType::Object(identity) => Some(identity),
                    _ => {
                        return Err(failure(
                            &unit.source,
                            "T0001",
                            "type parameter bound must name an interface",
                            node.span,
                        ));
                    }
                }
            } else {
                None
            };
            Ok(GenericParameterContract {
                name,
                span: name_node.span,
                bound,
            })
        })
        .collect()
}

pub(super) fn lexical_type_parameter(unit: &SemanticUnit, position: usize, name: &str) -> bool {
    fn visit(unit: &SemanticUnit, node: &SyntaxNode, position: usize, name: &str) -> bool {
        if node.span.start <= position
            && position <= node.span.end
            && matches!(
                node.kind,
                SyntaxKind::FunctionDeclaration
                    | SyntaxKind::ClassDeclaration
                    | SyntaxKind::InterfaceDeclaration
                    | SyntaxKind::TraitDeclaration
                    | SyntaxKind::EnumDeclaration
            )
            && node.children.iter().any(|child| {
                child.kind == SyntaxKind::TypeParameterList
                    && child.children.iter().any(|parameter| {
                        parameter
                            .children
                            .iter()
                            .find(|part| part.kind == SyntaxKind::Name)
                            .is_some_and(|part| node_text(&unit.source, part) == name)
                    })
            })
        {
            return true;
        }
        node.children.iter().any(|child| {
            child.span.start <= position
                && position <= child.span.end
                && visit(unit, child, position, name)
        })
    }
    if unit.type_parameter_at(position, name).is_some() {
        return true;
    }
    visit(unit, &unit.tree.root, position, name)
}

pub(super) fn type_parameter_bound_at<'a>(
    unit: &'a SemanticUnit,
    position: usize,
    name: &str,
) -> Option<&'a ObjectIdentity> {
    unit.type_parameter_at(position, name)?.bound.as_ref()
}

pub(crate) fn select_unit_callable_contract(
    package: Option<&SemanticPackage>,
    unit: &SemanticUnit,
    call: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
) -> Option<(FunctionContract, BTreeMap<String, ValueType>)> {
    if contract.generic_parameters.is_empty() {
        return Some((contract.clone(), BTreeMap::new()));
    }
    let arguments = call.children.get(1)?;
    let callee = call.children.first()?;
    let mut substitutions = explicit_callable_arguments(unit, callee, contract)?;
    let mut positional = 0;
    let mut named_seen = false;
    let mut bound_parameters = BTreeSet::new();
    for argument in &arguments.children {
        let named = argument
            .children
            .first()
            .filter(|name| name.kind == SyntaxKind::Name && argument.children.len() > 1);
        let parameter = super::calls::resolve_call_parameter(
            unit,
            argument,
            named,
            contract,
            &mut positional,
            &mut named_seen,
        )
        .ok()?;
        if !parameter.variadic && !bound_parameters.insert(parameter.name.as_str()) {
            return None;
        }
        let Some(expected) = parameter.element_value_type() else {
            continue;
        };
        let value = argument.children.last().unwrap_or(argument);
        if let Some(mut actual) = infer_value_type(unit, value, bindings).ok().flatten() {
            if value.kind == SyntaxKind::Name {
                let callback = package
                    .and_then(|package| function_contract_for_call(package, unit, value))
                    .or_else(|| {
                        resolved_function_contract(
                            unit,
                            node_text(&unit.source, value),
                            value.span.start,
                        )
                    });
                if let Some(callback) = callback
                    && !callback.generic_parameters.is_empty()
                    && let Some(closed) = close_generic_callable_value(
                        unit,
                        callback,
                        &actual,
                        &substitute_value_type(&expected, &substitutions),
                    )
                {
                    actual = closed;
                }
            }
            let selected_expected = substitute_value_type(&expected, &substitutions);
            let explicitly_selected = callee.kind == SyntaxKind::AppliedType
                && matches!(selected_expected, ValueType::Object(_))
                && selected_expected != expected;
            if explicitly_selected
                && let ValueType::Object(expected_identity) = &selected_expected
                && let ValueType::Object(actual_identity) = &actual
                && super::types::object_types_compatible(
                    &unit.descriptors,
                    expected_identity,
                    actual_identity,
                )
            {
                continue;
            }
            bind_generic_type(&expected, &actual, &mut substitutions).ok()?;
        }
    }
    let expected = package
        .and_then(|package| expected_type_at(package, unit, &unit.tree.root, call.span))
        .or_else(|| expected_destination_in_unit(unit, &unit.tree.root, call.span));
    if contract
        .generic_parameters
        .iter()
        .any(|parameter| !substitutions.contains_key(&parameter.name))
        && let (Some(expected), Some(result)) = (expected, contract.return_type.as_ref())
    {
        bind_generic_type(result, &expected, &mut substitutions).ok()?;
    }
    if contract
        .generic_parameters
        .iter()
        .any(|parameter| !substitutions.contains_key(&parameter.name))
    {
        return None;
    }
    if !generic_bounds_satisfied(unit, &contract.generic_parameters, &substitutions) {
        return None;
    }
    Some((
        substitute_function_contract(contract, &substitutions),
        substitutions,
    ))
}

fn explicit_callable_arguments(
    unit: &SemanticUnit,
    callee: &SyntaxNode,
    contract: &FunctionContract,
) -> Option<BTreeMap<String, ValueType>> {
    if matches!(
        callee.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression
    ) {
        return explicit_callable_arguments(unit, callee.children.first()?, contract);
    }
    let mut substitutions = BTreeMap::new();
    if callee.kind != SyntaxKind::AppliedType {
        return Some(substitutions);
    }
    let (_, arguments) = callee.children.split_first()?;
    if arguments.len() != contract.generic_parameters.len() {
        return None;
    }
    let aliases = visible_descriptor_aliases(
        &unit.descriptor_aliases,
        callee.span.file,
        callee.span.start,
    );
    for (parameter, node) in contract.generic_parameters.iter().zip(arguments) {
        substitutions.insert(
            parameter.name.clone(),
            super::types::declared_value_type(unit, node, &aliases).ok()?,
        );
    }
    Some(substitutions)
}

fn substitute_function_contract(
    contract: &FunctionContract,
    substitutions: &BTreeMap<String, ValueType>,
) -> FunctionContract {
    let mut selected = contract.clone();
    selected.generic_parameters.clear();
    for parameter in &mut selected.parameters {
        parameter.value_type = parameter
            .value_type
            .as_ref()
            .map(|ty| substitute_value_type(ty, substitutions));
    }
    selected.return_type = selected
        .return_type
        .as_ref()
        .map(|ty| substitute_value_type(ty, substitutions));
    selected.thrown_types = selected
        .thrown_types
        .iter()
        .map(|ty| substitute_value_type(ty, substitutions))
        .collect();
    selected
}

pub(super) fn expected_destination_in_unit(
    unit: &SemanticUnit,
    root: &SyntaxNode,
    span: Span,
) -> Option<ValueType> {
    if root.kind == SyntaxKind::Binding
        && root
            .children
            .iter()
            .any(|child| child.span.start <= span.start && child.span.end >= span.end)
    {
        return root
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::TypeExpression)
            .and_then(|ty| {
                super::types::declared_value_type(
                    unit,
                    ty,
                    &visible_descriptor_aliases(
                        &unit.descriptor_aliases,
                        ty.span.file,
                        ty.span.start,
                    ),
                )
                .ok()
            });
    }
    if root.kind == SyntaxKind::ReturnStatement
        && root.span.start <= span.start
        && root.span.end >= span.end
    {
        return unit
            .functions
            .iter()
            .filter(|function| {
                function.span.start <= root.span.start && function.span.end >= root.span.end
            })
            .min_by_key(|function| function.span.end - function.span.start)
            .and_then(|function| function.return_type.clone());
    }
    root.children
        .iter()
        .find_map(|child| expected_destination_in_unit(unit, child, span))
}

pub(crate) fn expected_type_at(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    root: &SyntaxNode,
    span: Span,
) -> Option<ValueType> {
    if root.kind == SyntaxKind::Binding
        && root.children.iter().any(|initializer| {
            initializer.span.start <= span.start && initializer.span.end >= span.end
        })
    {
        return root
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::TypeExpression)
            .and_then(|node| {
                super::types::declared_value_type(
                    unit,
                    node,
                    &visible_descriptor_aliases(
                        &unit.descriptor_aliases,
                        node.span.file,
                        node.span.start,
                    ),
                )
                .ok()
            });
    }
    if root.kind == SyntaxKind::ReturnStatement
        && root.span.start <= span.start
        && root.span.end >= span.end
    {
        return unit
            .functions
            .iter()
            .filter(|contract| {
                contract.span.start <= root.span.start && contract.span.end >= root.span.end
            })
            .min_by_key(|contract| contract.span.end - contract.span.start)
            .and_then(|contract| contract.return_type.clone());
    }
    if root.kind == SyntaxKind::CallExpression
        && let Some(arguments) = root.children.get(1)
        && let Some((index, argument)) =
            arguments.children.iter().enumerate().find(|(_, argument)| {
                argument.span.start <= span.start && argument.span.end >= span.end
            })
        && let Some(callee) = root.children.first()
        && let Some(contract) = super::namespaces::function_contract_for_call(package, unit, callee)
    {
        let named = argument
            .children
            .first()
            .filter(|_| argument.children.len() > 1)
            .map(|name| node_text(&unit.source, name));
        return named
            .and_then(|name| {
                contract
                    .parameters
                    .iter()
                    .find(|parameter| parameter.name == name)
            })
            .or_else(|| contract.parameters.get(index))
            .and_then(ParameterContract::element_value_type);
    }
    root.children
        .iter()
        .find_map(|child| expected_type_at(package, unit, child, span))
}

pub(crate) fn close_generic_callable_value(
    unit: &SemanticUnit,
    contract: &FunctionContract,
    actual: &ValueType,
    expected: &ValueType,
) -> Option<ValueType> {
    if contract.generic_parameters.is_empty() {
        return None;
    }
    let mut substitutions = BTreeMap::new();
    let (ValueType::Function(actual_parameters, actual_result, _)
    | ValueType::AsyncFunction(actual_parameters, actual_result, _, _)) = actual
    else {
        return None;
    };
    let (ValueType::Function(expected_parameters, expected_result, _)
    | ValueType::AsyncFunction(expected_parameters, expected_result, _, _)) = expected
    else {
        return None;
    };
    if actual_parameters.len() != expected_parameters.len() {
        return None;
    }
    for (actual, expected) in actual_parameters.iter().zip(expected_parameters) {
        bind_generic_type(
            actual.value_type_ref(),
            expected.value_type_ref(),
            &mut substitutions,
        )
        .ok()?;
    }
    bind_generic_type(
        actual_result.value_type_ref(),
        expected_result.value_type_ref(),
        &mut substitutions,
    )
    .ok()?;
    if contract
        .generic_parameters
        .iter()
        .any(|parameter| !substitutions.contains_key(&parameter.name))
    {
        return None;
    }
    if !generic_bounds_satisfied(unit, &contract.generic_parameters, &substitutions) {
        return None;
    }
    Some(substitute_value_type(actual, &substitutions))
}

pub(crate) fn generic_bounds_satisfied(
    unit: &SemanticUnit,
    parameters: &[GenericParameterContract],
    substitutions: &BTreeMap<String, ValueType>,
) -> bool {
    parameters.iter().all(|parameter| {
        let Some(bound) = &parameter.bound else {
            return true;
        };
        let Some(ValueType::Object(actual)) = substitutions.get(&parameter.name) else {
            return false;
        };
        if !unit
            .descriptors
            .iter()
            .any(|descriptor| descriptor.identity.base() == actual.base())
        {
            return false;
        }
        super::types::object_types_compatible(&unit.descriptors, bound, actual)
    })
}

pub(crate) fn bind_generic_type(
    expected: &ValueType,
    actual: &ValueType,
    bindings: &mut BTreeMap<String, ValueType>,
) -> Result<(), String> {
    if let ValueType::TypeParameter(name) | ValueType::ProjectedGeneric(name) = expected {
        return match bindings.get(name) {
            Some(previous) if previous != actual => Err(format!(
                "type parameter `{name}` must have one invariant type, found `{previous}` and `{actual}`"
            )),
            Some(_) => Ok(()),
            None => {
                bindings.insert(name.clone(), actual.clone());
                Ok(())
            }
        };
    }
    match (expected, actual) {
        (ValueType::Object(left), ValueType::Object(right))
            if left.namespace == right.namespace
                && left.name == right.name
                && left.is_unsafe == right.is_unsafe
                && left.native_projection == right.native_projection
                && left.application.is_some() == right.application.is_some()
                && left.type_arguments.len() == right.type_arguments.len()
                && left
                    .native_arguments
                    .keys()
                    .eq(right.native_arguments.keys()) =>
        {
            if let (Some(left), Some(right)) = (&left.application, &right.application) {
                bind_generic_type(left, right, bindings)?;
                if substitute_value_type(left, bindings) != **right {
                    return Err(format!(
                        "generic arguments `{left}` and `{right}` are invariant"
                    ));
                }
            }
            for (left, right) in left.type_arguments.iter().zip(&right.type_arguments) {
                bind_generic_type(left, right, bindings)?;
                if substitute_value_type(left, bindings) != *right {
                    return Err(format!(
                        "generic arguments `{left}` and `{right}` are invariant"
                    ));
                }
            }
            for (name, left) in &left.native_arguments {
                let right = &right.native_arguments[name];
                bind_generic_type(left, right, bindings)?;
                if substitute_value_type(left, bindings) != *right {
                    return Err(format!("generic argument `{name}` is invariant"));
                }
            }
            Ok(())
        }
        (ValueType::Optional(l), ValueType::Optional(r)) => bind_generic_type(l, r, bindings),
        (ValueType::Optional(inner), actual) => bind_generic_type(inner, actual, bindings),
        (ValueType::List(l), ValueType::List(r))
        | (ValueType::Set(l), ValueType::Set(r))
        | (ValueType::UnorderedSet(l), ValueType::UnorderedSet(r))
        | (ValueType::Reference(l), ValueType::Reference(r))
        | (ValueType::SharedReference(l), ValueType::SharedReference(r))
        | (ValueType::Iterator(l), ValueType::Iterator(r))
        | (ValueType::IterationStep(l), ValueType::IterationStep(r))
        | (ValueType::AsyncIterationStep(l), ValueType::AsyncIterationStep(r))
        | (ValueType::ChannelPair(l), ValueType::ChannelPair(r))
        | (ValueType::ChannelSender(l), ValueType::ChannelSender(r))
        | (ValueType::ChannelReceiver(l), ValueType::ChannelReceiver(r))
        | (ValueType::ChannelSendOutcome(l), ValueType::ChannelSendOutcome(r))
        | (ValueType::ChannelReceiveOutcome(l), ValueType::ChannelReceiveOutcome(r))
        | (ValueType::DocumentDecodeOutcome(l), ValueType::DocumentDecodeOutcome(r))
        | (ValueType::Task(l, _), ValueType::Task(r, _))
        | (ValueType::ScopedTask(l, _), ValueType::ScopedTask(r, _))
        | (ValueType::TaskOutcome(l), ValueType::TaskOutcome(r)) => {
            bind_generic_type(l.value_type_ref(), r.value_type_ref(), bindings)
        }
        (ValueType::Tuple(l, left_count), ValueType::Tuple(r, right_count))
            if left_count == right_count =>
        {
            bind_generic_type(l.value_type_ref(), r.value_type_ref(), bindings)
        }
        (ValueType::Map(lk, lv), ValueType::Map(rk, rv))
        | (ValueType::Entry(lk, lv), ValueType::Entry(rk, rv))
        | (ValueType::UnorderedMap(lk, lv), ValueType::UnorderedMap(rk, rv)) => {
            bind_generic_type(lk.value_type_ref(), rk.value_type_ref(), bindings)?;
            bind_generic_type(lv.value_type_ref(), rv.value_type_ref(), bindings)
        }
        (ValueType::Function(lp, lr, _), ValueType::Function(rp, rr, _))
            if lp.len() == rp.len() =>
        {
            for (l, r) in lp.iter().zip(rp) {
                bind_generic_type(l.value_type_ref(), r.value_type_ref(), bindings)?;
            }
            bind_generic_type(lr.value_type_ref(), rr.value_type_ref(), bindings)
        }
        _ if expected == actual => Ok(()),
        _ => Err(format!("type `{actual}` does not match `{expected}`")),
    }
}

fn substitute_object_identity(
    identity: &ObjectIdentity,
    substitutions: &BTreeMap<String, ValueType>,
) -> ObjectIdentity {
    let mut result = identity.clone();
    result.application = identity
        .application
        .as_ref()
        .map(|argument| Box::new(substitute_value_type(argument, substitutions)));
    result.application_key = result
        .application
        .as_deref()
        .map(|argument| format!("{argument:?}"));
    result.type_arguments = identity
        .type_arguments
        .iter()
        .map(|argument| substitute_value_type(argument, substitutions))
        .collect();
    result.type_arguments_key =
        (!result.type_arguments.is_empty()).then(|| format!("{:?}", result.type_arguments));
    result.native_arguments = identity
        .native_arguments
        .iter()
        .map(|(name, argument)| (name.clone(), substitute_value_type(argument, substitutions)))
        .collect();
    result.native_arguments_key =
        (!result.native_arguments.is_empty()).then(|| format!("{:?}", result.native_arguments));
    result
}

pub(crate) fn substitute_value_type(
    value: &ValueType,
    substitutions: &BTreeMap<String, ValueType>,
) -> ValueType {
    fn element(value: &ElementType, substitutions: &BTreeMap<String, ValueType>) -> ElementType {
        ElementType::new(substitute_value_type(value.value_type_ref(), substitutions))
    }
    match value {
        ValueType::TypeParameter(name) | ValueType::ProjectedGeneric(name) => substitutions
            .get(name)
            .cloned()
            .unwrap_or_else(|| value.clone()),
        ValueType::Optional(inner) => {
            ValueType::Optional(Box::new(substitute_value_type(inner, substitutions)))
        }
        ValueType::Object(identity) => {
            ValueType::Object(substitute_object_identity(identity, substitutions))
        }
        ValueType::List(item) => ValueType::List(element(item, substitutions)),
        ValueType::Set(item) => ValueType::Set(element(item, substitutions)),
        ValueType::UnorderedSet(item) => ValueType::UnorderedSet(element(item, substitutions)),
        ValueType::Map(key, item) => {
            ValueType::Map(element(key, substitutions), element(item, substitutions))
        }
        ValueType::Entry(key, item) => {
            ValueType::Entry(element(key, substitutions), element(item, substitutions))
        }
        ValueType::UnorderedMap(key, item) => {
            ValueType::UnorderedMap(element(key, substitutions), element(item, substitutions))
        }
        ValueType::Tuple(item, count) => ValueType::Tuple(element(item, substitutions), *count),
        ValueType::Reference(item) => ValueType::Reference(element(item, substitutions)),
        ValueType::SharedReference(item) => {
            ValueType::SharedReference(element(item, substitutions))
        }
        ValueType::Iterator(item) => ValueType::Iterator(element(item, substitutions)),
        ValueType::IterationStep(item) => ValueType::IterationStep(element(item, substitutions)),
        ValueType::AsyncIterationStep(item) => {
            ValueType::AsyncIterationStep(element(item, substitutions))
        }
        ValueType::ChannelPair(item) => ValueType::ChannelPair(element(item, substitutions)),
        ValueType::ChannelSender(item) => ValueType::ChannelSender(element(item, substitutions)),
        ValueType::ChannelReceiver(item) => {
            ValueType::ChannelReceiver(element(item, substitutions))
        }
        ValueType::ChannelSendOutcome(item) => {
            ValueType::ChannelSendOutcome(element(item, substitutions))
        }
        ValueType::ChannelReceiveOutcome(item) => {
            ValueType::ChannelReceiveOutcome(element(item, substitutions))
        }
        ValueType::DocumentDecodeOutcome(item) => {
            ValueType::DocumentDecodeOutcome(element(item, substitutions))
        }
        ValueType::Task(item, transferable) => {
            ValueType::Task(element(item, substitutions), *transferable)
        }
        ValueType::ScopedTask(item, transferable) => {
            ValueType::ScopedTask(element(item, substitutions), *transferable)
        }
        ValueType::TaskOutcome(item) => ValueType::TaskOutcome(element(item, substitutions)),
        ValueType::Function(parameters, result, effects) => ValueType::Function(
            parameters
                .iter()
                .map(|parameter| {
                    parameter.with_element_type(element(&parameter.element_type(), substitutions))
                })
                .collect(),
            element(result, substitutions),
            effects.clone(),
        ),
        ValueType::AsyncFunction(parameters, result, transferability, effects) => {
            ValueType::AsyncFunction(
                parameters
                    .iter()
                    .map(|parameter| {
                        parameter
                            .with_element_type(element(&parameter.element_type(), substitutions))
                    })
                    .collect(),
                element(result, substitutions),
                *transferability,
                effects.clone(),
            )
        }
        _ => value.clone(),
    }
}
