use super::prelude::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Binder {
    file: u32,
    start: usize,
    name: String,
}
#[derive(Clone, Debug)]
struct Edge {
    from: Binder,
    to: Binder,
    expands: bool,
    span: Span,
}

fn nested_parameter(value: &ValueType, name: &str, nested: bool) -> Option<bool> {
    let child = |ty: &ValueType| nested_parameter(ty, name, true);
    match value {
        ValueType::TypeParameter(found) if found == name => Some(nested),
        ValueType::Object(id) => id.type_arguments.iter().find_map(child),
        ValueType::Optional(x) => child(x),
        ValueType::Reference(inner) | ValueType::SharedReference(inner) => {
            nested_parameter(inner.value_type_ref(), name, nested)
        }
        ValueType::List(x)
        | ValueType::Set(x)
        | ValueType::Iterator(x)
        | ValueType::Tuple(x, _)
        | ValueType::Task(x, _)
        | ValueType::ScopedTask(x, _)
        | ValueType::UnorderedSet(x)
        | ValueType::ChannelPair(x)
        | ValueType::ChannelSender(x)
        | ValueType::ChannelReceiver(x)
        | ValueType::ChannelSendOutcome(x)
        | ValueType::ChannelReceiveOutcome(x)
        | ValueType::DocumentDecodeOutcome(x)
        | ValueType::TaskOutcome(x)
        | ValueType::AsyncIterationStep(x) => child(x.value_type_ref()),
        ValueType::Map(k, v) | ValueType::Entry(k, v) | ValueType::UnorderedMap(k, v) => {
            child(k.value_type_ref()).or_else(|| child(v.value_type_ref()))
        }
        ValueType::Function(args, result, _) | ValueType::AsyncFunction(args, result, _, _) => args
            .iter()
            .find_map(|p| child(p.value_type_ref()))
            .or_else(|| child(result.value_type_ref())),
        _ => None,
    }
}

fn path_to(from: &Binder, goal: &Binder, edges: &[Edge], seen: &mut BTreeSet<Binder>) -> bool {
    if from == goal {
        return true;
    }
    if !seen.insert(from.clone()) {
        return false;
    }
    edges
        .iter()
        .filter(|e| &e.from == from)
        .any(|e| path_to(&e.to, goal, edges, seen))
}

fn function_parameters<'a>(
    unit: &'a SemanticUnit,
    function: &'a FunctionContract,
) -> impl Iterator<Item = &'a GenericParameterContract> {
    let owner = function.owner_identity.as_ref().and_then(|owner| {
        unit.descriptors
            .iter()
            .find(|descriptor| descriptor.identity.base() == owner.base())
    });
    function.generic_parameters.iter().chain(
        owner
            .into_iter()
            .flat_map(|descriptor| &descriptor.generic_parameters)
            .filter(|parameter| {
                !function
                    .generic_parameters
                    .iter()
                    .any(|local| local.name == parameter.name)
            }),
    )
}

fn owner_contract(
    unit: &SemanticUnit,
    call: &SyntaxNode,
    target: &FunctionContract,
) -> Option<(ValueType, ValueType)> {
    let mut callee = call.children.first()?;
    while matches!(
        callee.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression | SyntaxKind::AppliedType
    ) {
        callee = callee.children.first()?;
    }
    if callee.kind != SyntaxKind::MemberExpression {
        return None;
    }
    let owner = target.owner_identity.as_ref()?;
    let descriptor = unit
        .descriptors
        .iter()
        .find(|descriptor| descriptor.identity.base() == owner.base())?;
    let mut receiver =
        infer_value_type(unit, callee.children.first()?, &unit.typed_bindings).ok()??;
    while let ValueType::Reference(inner) | ValueType::SharedReference(inner) = receiver {
        receiver = inner.value_type();
    }
    let template = owner.clone().with_type_arguments(
        descriptor
            .generic_parameters
            .iter()
            .map(|parameter| ValueType::TypeParameter(parameter.name.clone()))
            .collect(),
    );
    Some((ValueType::Object(template), receiver))
}

fn collect(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    edges: &mut Vec<Edge>,
) {
    if node.kind == SyntaxKind::CallExpression
        && let Some(selected) = super::calls::selected_callable_contract(package, unit, node, false)
    {
        let caller = unit
            .functions
            .iter()
            .filter(|function| {
                function.span.start <= node.span.start
                    && node.span.end <= function.span.end
                    && function_parameters(unit, function).next().is_some()
            })
            .min_by_key(|function| function.span.end - function.span.start);
        let target = package
            .units
            .iter()
            .flat_map(|u| &u.functions)
            .find(|f| f.span == selected.span);
        if let (Some(caller), Some(target)) = (caller, target) {
            let owner = owner_contract(unit, node, target);
            let contracts = target
                .parameters
                .iter()
                .zip(&selected.parameters)
                .filter_map(|(original, selected)| {
                    Some((original.value_type.as_ref()?, selected.value_type.as_ref()?))
                })
                .chain(target.return_type.iter().zip(&selected.return_type))
                .chain(owner.as_ref().map(|(expected, actual)| (expected, actual)));
            for (expected, actual) in contracts {
                let mut substitutions = BTreeMap::new();
                if super::generics::bind_generic_type(expected, actual, &mut substitutions).is_err()
                {
                    continue;
                }
                for (callee_name, selected_type) in substitutions {
                    for caller_param in function_parameters(unit, caller) {
                        if let Some(expands) =
                            nested_parameter(&selected_type, &caller_param.name, false)
                        {
                            edges.push(Edge {
                                from: Binder {
                                    file: caller.span.file,
                                    start: caller.span.start,
                                    name: caller_param.name.clone(),
                                },
                                to: Binder {
                                    file: target.span.file,
                                    start: target.span.start,
                                    name: callee_name.clone(),
                                },
                                expands,
                                span: node.span,
                            });
                        }
                    }
                }
            }
        }
    }
    for child in &node.children {
        collect(package, unit, child, edges);
    }
}

pub(super) fn validate(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    let mut edges = Vec::new();
    for unit in &package.units {
        collect(package, unit, &unit.tree.root, &mut edges);
    }
    for edge in edges.iter().filter(|edge| edge.expands) {
        if path_to(&edge.to, &edge.from, &edges, &mut BTreeSet::new()) {
            let unit = package
                .units
                .iter()
                .find(|u| u.source.id() == edge.span.file)
                .expect("source unit for generic call");
            return Err(failure(
                &unit.source,
                "T0225",
                format!(
                    "generic recursion expands type parameter `{}` through a type constructor",
                    edge.from.name
                ),
                edge.span,
            ));
        }
    }
    Ok(())
}
