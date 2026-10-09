use super::prelude::*;

pub(super) fn call_base_designator(mut callee: &SyntaxNode) -> Option<&SyntaxNode> {
    while matches!(
        callee.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression | SyntaxKind::AppliedType
    ) {
        callee = callee.children.first()?;
    }
    Some(callee)
}

pub(super) fn call_base_contract<'a>(
    unit: &'a SemanticUnit,
    call: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<&'a FunctionContract>, SemanticFailure> {
    let Some(callee) = call.children.first().and_then(call_base_designator) else {
        return Ok(None);
    };
    let is_unsafe = crate::syntax::call_is_unsafe(call);
    if callee.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, callee);
        return Ok(if is_unsafe {
            resolved_function_contract(unit, &format!("unsafe::{name}"), callee.span.start)
        } else {
            resolved_function_contract(unit, name, callee.span.start)
        });
    }
    if matches!(
        callee.kind,
        SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression
    ) && let [receiver, member] = callee.children.as_slice()
    {
        let is_static = callee.kind == SyntaxKind::StaticMemberExpression;
        let identity = if is_static {
            class_designator_identity(unit, receiver)
        } else {
            match infer_receiver_value_type(unit, receiver, bindings)? {
                Some(
                    ValueType::Object(identity)
                    | ValueType::InvocationScopedNative {
                        family: identity, ..
                    },
                ) => Some(identity),
                _ => None,
            }
        };
        return Ok(identity.as_ref().and_then(|identity| {
            object_method_contract_with_safety(
                unit,
                identity,
                node_text(&unit.source, member),
                is_static,
                is_unsafe,
            )
        }));
    }
    Ok(None)
}

fn channel_item_descriptor_type(unit: &SemanticUnit, name: &str) -> Option<ValueType> {
    if let Some(scalar) = ScalarType::from_source_name(name) {
        return Some(ValueType::Scalar(scalar));
    }
    if let Some(item) = name.strip_prefix("list of ") {
        return channel_item_descriptor_type(unit, item)
            .map(|item| ValueType::List(ElementType::new(item)));
    }
    unit.descriptors
        .iter()
        .find(|object| object.name == name || object.identity.qualified() == name)
        .map(|object| ValueType::Object(object.identity.clone()))
}

fn constant_duration_factory_value(
    unit: &SemanticUnit,
    mut node: &SyntaxNode,
    bindings: &[TypedBinding],
    visited: &mut BTreeSet<(u32, usize, usize)>,
) -> Option<num_bigint::BigInt> {
    while node.kind == SyntaxKind::GroupExpression {
        node = node.children.first()?;
    }
    if node.kind == SyntaxKind::Name {
        let binding = bindings
            .iter()
            .filter(|binding| {
                binding.name == node_text(&unit.source, node)
                    && binding.is_visible_at(unit.source.id(), node.span.start)
            })
            .max_by_key(|binding| binding.visible_from)?;
        if !visited.insert((binding.span.file, binding.span.start, binding.span.end)) {
            return None;
        }
        let initializer = find_binding_initializer(&unit.tree.root, binding.span)?;
        return constant_duration_factory_value(unit, initializer, bindings, visited);
    }
    let [callee, arguments] = node.children.as_slice() else {
        return None;
    };
    if node.kind != SyntaxKind::CallExpression || callee.kind != SyntaxKind::StaticMemberExpression
    {
        return None;
    }
    let [receiver, member] = callee.children.as_slice() else {
        return None;
    };
    let identity = class_designator_identity(unit, receiver)?;
    if identity.namespace != "/core/time"
        || identity.name != "duration"
        || !matches!(
            node_text(&unit.source, member),
            "seconds" | "milliseconds" | "microseconds" | "nanoseconds"
        )
    {
        return None;
    }
    let argument = arguments.children.first()?.children.last()?;
    match contextual_constant(&unit.source, argument, ScalarType::Int)? {
        Ok(ContextualConstant::Integer(value)) => Some(value),
        Ok(ContextualConstant::Float32(_) | ContextualConstant::Float64(_)) | Err(_) => None,
    }
}

fn is_destination_directed_projected_call(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<bool, SemanticFailure> {
    let [callee, _] = node.children.as_slice() else {
        return Ok(false);
    };
    let identity = if callee.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, callee);
        lexical_scope_chain(unit, callee.span.start)
            .find_map(|scope| {
                scope.symbols.get(name)?.iter().rev().find(|symbol| {
                    symbol.declaration_span.is_none_or(|span| {
                        span.file != unit.source.id() || span.end <= callee.span.start
                    })
                })
            })
            .map(|symbol| symbol.identity.clone())
    } else if callee.kind == SyntaxKind::MemberExpression {
        let [receiver, member] = callee.children.as_slice() else {
            return Ok(false);
        };
        match infer_value_type(unit, receiver, bindings)? {
            Some(ValueType::Object(identity)) => Some(format!(
                "{}.{}",
                identity.qualified(),
                node_text(&unit.source, member)
            )),
            _ => None,
        }
    } else if callee.kind == SyntaxKind::StaticMemberExpression {
        let [receiver, member] = callee.children.as_slice() else {
            return Ok(false);
        };
        class_designator_identity(unit, receiver).map(|identity| {
            format!(
                "{}.{}",
                identity.qualified(),
                node_text(&unit.source, member)
            )
        })
    } else {
        None
    };
    Ok(identity.is_some_and(|identity| unit.projected_destination_functions.contains(&identity)))
}

pub(super) fn infer_value_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    // Keep operator recursion outside the large non-operator inference frame.
    match node.kind {
        SyntaxKind::BinaryExpression => infer_binary_type(unit, node, bindings),
        SyntaxKind::GroupExpression => match node.children.first() {
            Some(child) => infer_value_type(unit, child, bindings),
            None => Ok(None),
        },
        _ => infer_nonbinary_value_type(unit, node, bindings),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "value inference centralizes the precedence among syntax forms and typed member families"
)]
fn infer_nonbinary_value_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<ValueType>, SemanticFailure> {
    if node.kind == SyntaxKind::Literal {
        return Ok(infer_literal_type(unit, node).map(ValueType::Scalar));
    }
    if matches!(
        node.kind,
        SyntaxKind::RustBlock | SyntaxKind::UnsafeRustBlock
    ) {
        return Ok(Some(ValueType::InlineRust));
    }
    if node.kind == SyntaxKind::AnonymousFunction {
        let contract = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .expect("analyzed closure must have a semantic contract");
        let parameters = contract
            .parameters
            .iter()
            .map(|parameter| {
                parameter.callable_type().ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0052",
                        "stored function parameters require explicit types",
                        parameter.span,
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let result = ElementType::new(
            contract
                .return_type
                .clone()
                .unwrap_or(ValueType::Scalar(ScalarType::None)),
        );
        let effects = CallableEffects::from_contract(contract);
        return Ok(Some(if contract.is_async {
            ValueType::AsyncFunction(parameters, result, contract.task_transferability, effects)
        } else {
            ValueType::Function(parameters, result, effects)
        }));
    }
    if node.kind == SyntaxKind::UnaryExpression {
        return infer_unary_type(unit, node, bindings).map(Some);
    }
    if node.kind == SyntaxKind::TypeMembershipExpression {
        return Ok(Some(ValueType::Scalar(ScalarType::Bool)));
    }
    if node.kind == SyntaxKind::AppliedType
        && let Some(contract) = call_base_contract(unit, node, bindings)?
        && let Some(selected) = unit
            .projected_callable_applications
            .get(&(node.span.file, node.span.start, node.span.end))
            .and_then(|contracts| {
                contracts.get(&(contract.span.file, contract.span.start, contract.span.end))
            })
    {
        return Ok(Some(selected.clone()));
    }
    if node.kind == SyntaxKind::AppliedType
        && let Some(name_node) = node.children.first()
        && name_node.kind == SyntaxKind::Name
    {
        let name = node_text(&unit.source, name_node);
        if let Some(contract) = resolved_function_contract(unit, name, name_node.span.start)
            && !contract.generic_parameters.is_empty()
        {
            let type_nodes = &node.children[1..];
            if type_nodes.len() != contract.generic_parameters.len() {
                return Err(failure(
                    &unit.source,
                    "T0012",
                    format!(
                        "function `{name}` requires {} type arguments, found {}",
                        contract.generic_parameters.len(),
                        type_nodes.len()
                    ),
                    node.span,
                ));
            }
            let aliases = visible_descriptor_aliases(
                &unit.descriptor_aliases,
                node.span.file,
                node.span.start,
            );
            let substitutions = contract
                .generic_parameters
                .iter()
                .zip(type_nodes)
                .map(|(parameter, type_node)| {
                    super::types::declared_value_type(unit, type_node, &aliases)
                        .map(|value_type| (parameter.name.clone(), value_type))
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?;
            if !super::generics::generic_bounds_satisfied(
                unit,
                &contract.generic_parameters,
                &substitutions,
            ) {
                return Err(failure(
                    &unit.source,
                    "T0012",
                    format!(
                        "explicit application of `{name}` does not satisfy its type parameter bounds"
                    ),
                    node.span,
                ));
            }
            let parameters = contract
                .parameters
                .iter()
                .map(ParameterContract::callable_type)
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0052",
                        "stored function parameters require explicit types",
                        node.span,
                    )
                })?
                .into_iter()
                .map(|parameter| {
                    let value_type = super::generics::substitute_value_type(
                        parameter.value_type_ref(),
                        &substitutions,
                    );
                    parameter.with_element_type(ElementType::new(value_type))
                })
                .collect();
            let result = ElementType::new(super::generics::substitute_value_type(
                contract
                    .return_type
                    .as_ref()
                    .unwrap_or(&ValueType::Scalar(ScalarType::None)),
                &substitutions,
            ));
            let effects = CallableEffects::from_contract(contract);
            return Ok(Some(if contract.is_async {
                ValueType::AsyncFunction(parameters, result, contract.task_transferability, effects)
            } else {
                ValueType::Function(parameters, result, effects)
            }));
        }
    }
    if node.kind == SyntaxKind::Name {
        let name = node_text(&unit.source, node);
        if name == "none" {
            return Ok(Some(ValueType::Scalar(ScalarType::None)));
        }
        if resolved_compiler_identity(unit, node).is_some_and(|identity| {
            matches!(
                identity,
                "/core/concurrency::channel-block"
                    | "/core/concurrency::channel-fail-send"
                    | "/core/concurrency::channel-drop-newest"
                    | "/core/concurrency::channel-drop-oldest"
            )
        }) {
            return Ok(Some(ValueType::ChannelOverflowPolicy));
        }
        if let Some(scalar) = ScalarType::from_source_name(name).or_else(|| {
            visible_descriptor_aliases(&unit.descriptor_aliases, unit.source.id(), node.span.start)
                .get(name)
                .copied()
        }) {
            return Ok(Some(ValueType::Descriptor(scalar.source_name().to_owned())));
        }
        if let Some(value_type) =
            unit.flow_types
                .get(&(node.span.file, node.span.start, node.span.end))
        {
            return Ok(Some(
                narrowed_value_type(unit, node, bindings).unwrap_or_else(|| value_type.clone()),
            ));
        }
        if let Some(binding) = bindings
            .iter()
            .filter(|binding| {
                binding.name == name && binding.is_visible_at(unit.source.id(), node.span.start)
            })
            .max_by_key(|binding| binding.visible_from)
        {
            return Ok(Some(
                narrowed_value_type(unit, node, bindings).unwrap_or(binding.value_type.clone()),
            ));
        }
        if let Some(object) = unit
            .descriptors
            .iter()
            .find(|object| object.builtin.is_none() && object.name == name)
        {
            return Ok(Some(ValueType::Descriptor(object.identity.qualified())));
        }
        if let Some(contract) = unit.functions.iter().find(|contract| {
            contract.owner.is_none() && contract.name == name && !contract.is_unsafe
        }) {
            let parameters = contract
                .parameters
                .iter()
                .map(ParameterContract::callable_type)
                .collect::<Option<Vec<_>>>();
            if let Some(parameters) = parameters {
                let result = ElementType::new(
                    contract
                        .return_type
                        .clone()
                        .unwrap_or(ValueType::Scalar(ScalarType::None)),
                );
                let effects = CallableEffects::from_contract(contract);
                return Ok(Some(if contract.is_async {
                    ValueType::AsyncFunction(
                        parameters,
                        result,
                        contract.task_transferability,
                        effects,
                    )
                } else {
                    ValueType::Function(parameters, result, effects)
                }));
            }
        }
        if let Some(contract) = resolved_function_contract(unit, name, node.span.start) {
            let parameters = contract
                .parameters
                .iter()
                .map(ParameterContract::callable_type)
                .collect::<Option<Vec<_>>>();
            if let Some(parameters) = parameters {
                let result = ElementType::new(
                    contract
                        .return_type
                        .clone()
                        .unwrap_or(ValueType::Scalar(ScalarType::None)),
                );
                let effects = CallableEffects::from_contract(contract);
                return Ok(Some(if contract.is_async {
                    ValueType::AsyncFunction(
                        parameters,
                        result,
                        contract.task_transferability,
                        effects,
                    )
                } else {
                    ValueType::Function(parameters, result, effects)
                }));
            }
        }
        if resolved_function_contract(unit, &format!("unsafe::{name}"), node.span.start).is_some() {
            return Err(failure(
                &unit.source,
                "T0130",
                format!(
                    "unsafe function `{name}` cannot be used as a function value; invoke it directly with `unsafe`"
                ),
                node.span,
            ));
        }
        let resolved_symbol = lexical_scope_chain(unit, node.span.start).find_map(|scope| {
            scope.symbols.get(name)?.iter().rev().find(|symbol| {
                symbol
                    .declaration_span
                    .is_none_or(|span| span.end <= node.span.start)
            })
        });
        let resolved_encoding = resolved_symbol
            .map(|symbol| symbol.identity.as_str())
            .or_else(|| {
                unit.prelude.then_some(name).and_then(|name| {
                    matches!(
                        name,
                        "utf8" | "utf16-le" | "utf16-be" | "utf32-le" | "utf32-be"
                    )
                    .then_some(name)
                })
            })
            .is_some_and(|identity| {
                identity.starts_with("/core/encodings::")
                    || matches!(
                        identity,
                        "utf8" | "utf16-le" | "utf16-be" | "utf32-le" | "utf32-be"
                    )
            });
        return Ok(resolved_encoding.then_some(ValueType::Encoding));
    }
    if node.kind == SyntaxKind::IndexExpression {
        let [receiver, index] = node.children.as_slice() else {
            return Ok(None);
        };
        let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
        let index_type = infer_value_type(unit, index, bindings)?;
        return match receiver_type {
            Some(ValueType::List(item))
                if value_type_contains_nonclone_foreign(unit, item.value_type_ref()) =>
            {
                Err(failure(
                    &unit.source,
                    "T0135",
                    "persistent list indexing cannot copy an item containing a non-Clone projected foreign value (`collections/consume-non-clone-foreign-items`)",
                    receiver.span,
                ))
            }
            Some(ValueType::List(item) | ValueType::Tuple(item, _)) => Ok(Some(item.value_type())),
            Some(ValueType::StringList) => Ok(Some(ValueType::Scalar(ScalarType::String))),
            Some(ValueType::Map(_, value) | ValueType::UnorderedMap(_, value)) => {
                Ok(Some(value.value_type()))
            }
            Some(ValueType::Scalar(ScalarType::Bytes)) => match index_type {
                Some(ValueType::Scalar(index)) if index.is_integer() => {
                    Ok(Some(ValueType::Scalar(ScalarType::Uint8)))
                }
                Some(ValueType::Range) => Ok(Some(ValueType::Scalar(ScalarType::Bytes))),
                Some(other) => Err(failure(
                    &unit.source,
                    "T0050",
                    format!("bytes require an integer index or range, found `{other}`"),
                    index.span,
                )),
                None => Err(failure(
                    &unit.source,
                    "T0050",
                    "byte indexing requires a statically known integer index or range",
                    index.span,
                )),
            },
            Some(ValueType::Scalar(ScalarType::String)) => Err(failure(
                &unit.source,
                "T0050",
                "string indexing is not implemented yet",
                receiver.span,
            )),
            Some(other) => Err(failure(
                &unit.source,
                "T0050",
                format!("indexing is not supported for `{other}`"),
                receiver.span,
            )),
            None => Err(failure(
                &unit.source,
                "T0050",
                "indexing requires a receiver with a statically known collection type",
                receiver.span,
            )),
        };
    }
    if node.kind == SyntaxKind::MemberExpression {
        return infer_member_value_type(unit, node, bindings);
    }
    if node.kind == SyntaxKind::StaticMemberExpression {
        let [receiver, member] = node.children.as_slice() else {
            return Ok(None);
        };
        let aliases = visible_descriptor_aliases(
            &unit.descriptor_aliases,
            receiver.span.file,
            receiver.span.start,
        );
        let declared = declared_value_type(unit, receiver, &aliases).ok();
        if let Some(ValueType::Object(identity)) = declared
            && unit.descriptors.iter().any(|descriptor| {
                descriptor.identity.base() == identity.base() && descriptor.kind == ObjectKind::Enum
            })
        {
            return Ok(Some(ValueType::Object(identity)));
        }
        let identity = class_designator_identity(unit, receiver).ok_or_else(|| {
            failure(
                &unit.source,
                "T0104",
                "the left side of `::` must resolve to a class",
                receiver.span,
            )
        })?;
        return object_member_type(unit, &identity, node_text(&unit.source, member), true)
            .map(Some)
            .ok_or_else(|| missing_static_member_failure(unit, &identity, member));
    }
    if node.kind == SyntaxKind::CallExpression {
        let cached_value_type = unit
            .projected_call_specializations
            .get(&(node.span.file, node.span.start, node.span.end))
            .map(|specialization| &specialization.value_type)
            .or_else(|| {
                unit.selected_expression_types.get(&(
                    node.span.file,
                    node.span.start,
                    node.span.end,
                ))
            });
        if let Some(value_type) = cached_value_type {
            let transferability = if let Some(contract) = call_base_contract(unit, node, bindings)?
            {
                contract.is_async.then_some(contract.task_transferability)
            } else if let Some(callee) = node.children.first().and_then(call_base_designator)
                && let Some(ValueType::AsyncFunction(_, _, transferability, _)) =
                    infer_value_type(unit, callee, bindings)?
            {
                Some(transferability)
            } else {
                None
            };
            if let Some(transferability) = transferability {
                return Ok(Some(ValueType::Task(
                    ElementType::new(value_type.clone()),
                    transferability,
                )));
            }
            return Ok(Some(value_type.clone()));
        }
        if is_destination_directed_projected_call(unit, node, bindings)? {
            return Ok(None);
        }
        if let Some(value_type) = infer_typed_document_decode(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_structured_log_emit(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let [callee, arguments] = node.children.as_slice() {
            if callee.kind == SyntaxKind::MemberExpression
                && let [receiver, member] = callee.children.as_slice()
                && node_text(&unit.source, member) == "multiply"
                && infer_value_type(unit, receiver, bindings)?
                    == Some(ValueType::Object(ObjectIdentity::new(
                        "/core/time",
                        "duration",
                    )))
                && let Some(argument) = arguments.children.first()
                && let Some(Ok(ContextualConstant::Integer(value))) = contextual_constant(
                    &unit.source,
                    argument.children.last().unwrap_or(argument),
                    ScalarType::Int,
                )
                && value < num_bigint::BigInt::from(0)
            {
                return Err(failure(
                    &unit.source,
                    "T0133",
                    "duration multipliers must be non-negative",
                    argument.span,
                ));
            }
            if callee.kind == SyntaxKind::ConstructionExpression {
                let class = callee.children.first().ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0103",
                        "construction requires a class",
                        callee.span,
                    )
                })?;
                if class.kind == SyntaxKind::StaticMemberExpression {
                    let [owner, variant_name] = class.children.as_slice() else {
                        return Ok(None);
                    };
                    let aliases = visible_descriptor_aliases(
                        &unit.descriptor_aliases,
                        owner.span.file,
                        owner.span.start,
                    );
                    let ValueType::Object(mut identity) =
                        declared_value_type(unit, owner, &aliases)?
                    else {
                        return Ok(None);
                    };
                    if let Some(enumeration) = unit
                        .source_enum_registry
                        .iter()
                        .find(|item| item.identity.base() == identity.base())
                    {
                        let Some(variant) = enumeration
                            .variants
                            .iter()
                            .find(|item| item.name == node_text(&unit.source, variant_name))
                        else {
                            return Ok(None);
                        };
                        let Some(descriptor) = unit
                            .descriptors
                            .iter()
                            .find(|item| item.identity.base() == identity.base())
                        else {
                            return Ok(None);
                        };
                        let mut substitutions = descriptor
                            .generic_parameters
                            .iter()
                            .zip(&identity.type_arguments)
                            .map(|(parameter, actual)| (parameter.name.clone(), actual.clone()))
                            .collect::<BTreeMap<_, _>>();
                        if let Some(ValueType::Object(expected)) =
                            super::generics::expected_destination_in_unit(
                                unit,
                                &unit.tree.root,
                                node.span,
                            )
                            && expected.base() == identity.base()
                        {
                            for (parameter, argument) in descriptor
                                .generic_parameters
                                .iter()
                                .zip(&expected.type_arguments)
                            {
                                if let Some(previous) =
                                    substitutions.insert(parameter.name.clone(), argument.clone())
                                    && previous != *argument
                                {
                                    return Err(failure(
                                        &unit.source,
                                        "T0218",
                                        "enum application disagrees with its destination",
                                        node.span,
                                    ));
                                }
                            }
                        }
                        for argument in &arguments.children {
                            let Some(name) = argument
                                .children
                                .first()
                                .filter(|_| argument.children.len() > 1)
                            else {
                                continue;
                            };
                            let Some(field) = variant
                                .payload
                                .iter()
                                .find(|field| field.name == node_text(&unit.source, name))
                            else {
                                continue;
                            };
                            let Some(value) = argument.children.last() else {
                                continue;
                            };
                            if let Some(actual) = infer_value_type(unit, value, bindings)? {
                                bind_generic_type(&field.value_type, &actual, &mut substitutions)
                                    .map_err(|message| {
                                    failure(&unit.source, "T0218", message, argument.span)
                                })?;
                            }
                        }
                        if descriptor
                            .generic_parameters
                            .iter()
                            .all(|parameter| substitutions.contains_key(&parameter.name))
                        {
                            identity = identity.with_type_arguments(
                                descriptor
                                    .generic_parameters
                                    .iter()
                                    .map(|parameter| substitutions[&parameter.name].clone())
                                    .collect(),
                            );
                        }
                    }
                    if identity.type_arguments.is_empty()
                        && let Some(ValueType::Object(expected)) =
                            super::generics::expected_destination_in_unit(
                                unit,
                                &unit.tree.root,
                                node.span,
                            )
                        && expected.base() == identity.base()
                    {
                        identity = expected;
                    }
                    return Ok(Some(ValueType::Object(identity)));
                }
                let identity = class_designator_identity(unit, class).ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0103",
                        format!(
                            "`{}` does not resolve to a constructible class",
                            node_text(&unit.source, class)
                        ),
                        class.span,
                    )
                })?;
                let Some(descriptor) = unit
                    .descriptors
                    .iter()
                    .find(|item| item.identity.base() == identity.base())
                else {
                    return Ok(Some(ValueType::Object(identity)));
                };
                // Native generic construction is selected and admitted from projection
                // metadata, not the authored-class generic constructor contract.
                if identity.namespace.starts_with("/deps/") {
                    if let Some(ValueType::Object(expected)) =
                        super::generics::expected_destination_in_unit(
                            unit,
                            &unit.tree.root,
                            node.span,
                        )
                        && expected.base() == identity.base()
                    {
                        return Ok(Some(ValueType::Object(expected)));
                    }
                    return Ok(Some(ValueType::Object(identity)));
                }
                if descriptor.generic_parameters.is_empty() || !identity.type_arguments.is_empty() {
                    return Ok(Some(ValueType::Object(identity)));
                }
                let mut substitutions = BTreeMap::new();
                if let Some(ValueType::Object(expected)) =
                    super::generics::expected_destination_in_unit(unit, &unit.tree.root, node.span)
                    && expected.base() == identity.base()
                {
                    substitutions.extend(
                        descriptor
                            .generic_parameters
                            .iter()
                            .zip(&expected.type_arguments)
                            .map(|(parameter, argument)| {
                                (parameter.name.clone(), argument.clone())
                            }),
                    );
                }
                let constructor = unit.functions.iter().find(|function| {
                    function
                        .owner_identity
                        .as_ref()
                        .is_some_and(|owner| owner.base() == identity.base())
                        && function.name == "construct"
                });
                if let (Some(constructor), Some(arguments)) = (constructor, node.children.get(1)) {
                    for (index, argument) in arguments.children.iter().enumerate() {
                        let parameter = if argument.children.len() > 1 {
                            constructor.parameters.iter().find(|parameter| {
                                parameter.name == node_text(&unit.source, &argument.children[0])
                            })
                        } else {
                            constructor.parameters.get(index)
                        };
                        let Some(parameter) = parameter else { continue };
                        let Some(expected) = parameter.element_value_type() else {
                            continue;
                        };
                        let value = argument.children.last().unwrap_or(argument);
                        let Some(actual) = infer_value_type(unit, value, bindings)? else {
                            continue;
                        };
                        bind_generic_type(&expected, &actual, &mut substitutions).map_err(
                            |message| failure(&unit.source, "T0218", message, argument.span),
                        )?;
                    }
                }
                let unselected = descriptor
                    .generic_parameters
                    .iter()
                    .filter(|parameter| !substitutions.contains_key(&parameter.name))
                    .map(|parameter| parameter.name.as_str())
                    .collect::<Vec<_>>();
                if !unselected.is_empty() {
                    return Err(failure(
                        &unit.source,
                        "T0210",
                        format!(
                            "class `{}` construction leaves type parameter(s) {} unselected; write an applied class type or provide constraining constructor arguments or a destination",
                            descriptor.name,
                            unselected
                                .iter()
                                .map(|name| format!("`{name}`"))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                        class.span,
                    ));
                }
                let arguments = descriptor
                    .generic_parameters
                    .iter()
                    .map(|parameter| substitutions[&parameter.name].clone())
                    .collect();
                return Ok(Some(ValueType::Object(
                    identity.with_type_arguments(arguments),
                )));
            }
            if callee.kind == SyntaxKind::StaticMemberExpression {
                let [receiver, member] = callee.children.as_slice() else {
                    return Ok(None);
                };
                let identity = class_designator_identity(unit, receiver).ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0104",
                        "the left side of `::` must resolve to a class",
                        receiver.span,
                    )
                })?;
                let member_name = node_text(&unit.source, member);
                if identity.namespace == "/core/time"
                    && identity.name == "duration"
                    && matches!(
                        member_name,
                        "seconds" | "milliseconds" | "microseconds" | "nanoseconds"
                    )
                    && let Some(argument) = arguments.children.first()
                    && let Some(Ok(ContextualConstant::Integer(value))) = contextual_constant(
                        &unit.source,
                        argument.children.last().unwrap_or(argument),
                        ScalarType::Int,
                    )
                    && value < num_bigint::BigInt::from(0)
                {
                    return Err(failure(
                        &unit.source,
                        "T0133",
                        "duration values must be non-negative",
                        argument.span,
                    ));
                }
                if identity.namespace == "/core/time"
                    && identity.name == "clock"
                    && member_name == "interval"
                    && let Some(argument) = arguments.children.first()
                    && constant_duration_factory_value(
                        unit,
                        argument.children.last().unwrap_or(argument),
                        bindings,
                        &mut BTreeSet::new(),
                    )
                    .is_some_and(|value| value == num_bigint::BigInt::from(0))
                {
                    return Err(failure(
                        &unit.source,
                        "T0133",
                        "ticker intervals must be greater than zero",
                        argument.span,
                    ));
                }
                let member_type =
                    object_member_type(unit, &identity, node_text(&unit.source, member), true)
                        .ok_or_else(|| missing_static_member_failure(unit, &identity, member))?;
                return match member_type {
                    ValueType::Function(_, result, _) => {
                        let result = result.value_type();
                        let result = object_method_contract(
                            unit,
                            &identity,
                            node_text(&unit.source, member),
                            true,
                        )
                        .filter(|method| {
                            matches!(
                                &result,
                                ValueType::Object(returned)
                                    if method.owner.as_deref() == Some(returned.name.as_str())
                            )
                        })
                        .map_or(result, |_| {
                            ValueType::Object(
                                super::member_inference::selected_descriptor_identity(
                                    unit, &identity,
                                ),
                            )
                        });
                        Ok(Some(result))
                    }
                    ValueType::AsyncFunction(_, result, transferability, _) => {
                        Ok(Some(ValueType::Task(result, transferability)))
                    }
                    _ => Err(failure(
                        &unit.source,
                        "T0039",
                        format!(
                            "`{}::{}` is a property and cannot be invoked",
                            identity.name,
                            node_text(&unit.source, member)
                        ),
                        callee.span,
                    )),
                };
            }
            if callee.kind == SyntaxKind::Name
                && resolved_compiler_identity(unit, callee).is_some_and(|identity| {
                    matches!(
                        identity,
                        "/core/concurrency::mutex"
                            | "/core/concurrency::read-write-lock"
                            | "/core/concurrency::shared-cell"
                    )
                })
            {
                return Err(failure(
                    &unit.source,
                    "T0111",
                    "typed shared cells are unavailable; keep mutable state in one owner task and communicate through bounded typed channels",
                    node.span,
                ));
            }
            if callee.kind == SyntaxKind::Name
                && resolved_compiler_identity(unit, callee)
                    .is_some_and(|identity| identity == "/core/concurrency::channel")
            {
                let values = arguments
                    .children
                    .iter()
                    .map(|argument| argument.children.last().unwrap_or(argument))
                    .collect::<Vec<_>>();
                let [descriptor, capacity, overflow] = values.as_slice() else {
                    return Err(failure(
                        &unit.source,
                        "T0107",
                        "`channel` requires an item descriptor, constant capacity, and overflow policy",
                        node.span,
                    ));
                };
                let Some(ValueType::Descriptor(item_name)) =
                    infer_value_type(unit, descriptor, bindings)?
                else {
                    return Err(failure(
                        &unit.source,
                        "T0107",
                        "`channel` item must be a concrete type descriptor",
                        descriptor.span,
                    ));
                };
                let item_type =
                    channel_item_descriptor_type(unit, &item_name).ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0107",
                            "`channel` item descriptor must name a concrete transferable type",
                            descriptor.span,
                        )
                    })?;
                let Some(capacity_value) =
                    constant_nonnegative_u64(unit, capacity, bindings, &mut BTreeSet::new())
                else {
                    return Err(failure(
                        &unit.source,
                        "T0107",
                        "`channel` capacity must be a nonnegative constant integer",
                        capacity.span,
                    ));
                };
                if infer_value_type(unit, overflow, bindings)?
                    != Some(ValueType::ChannelOverflowPolicy)
                {
                    return Err(failure(
                        &unit.source,
                        "T0107",
                        "`channel` overflow argument must be a channel overflow policy",
                        overflow.span,
                    ));
                }
                if capacity_value == 0
                    && resolved_compiler_identity(unit, overflow)
                        .is_none_or(|identity| identity != "/core/concurrency::channel-block")
                {
                    return Err(failure(
                        &unit.source,
                        "T0107",
                        "zero-capacity channels require the `channel-block` rendezvous policy",
                        overflow.span,
                    ));
                }
                return Ok(Some(ValueType::ChannelPair(ElementType::new(item_type))));
            }
            if callee.kind == SyntaxKind::Name
                && resolved_compiler_identity(unit, callee)
                    .is_some_and(|identity| identity == "/core/async::task-scope")
            {
                if let Some(argument) = arguments.children.first() {
                    let deadline = argument.children.last().unwrap_or(argument);
                    if infer_value_type(unit, deadline, bindings)?
                        != Some(ValueType::Object(ObjectIdentity::new(
                            "/core/time",
                            "deadline",
                        )))
                    {
                        return Err(failure(
                            &unit.source,
                            "T0074",
                            "`task-scope` requires a `/core/time` deadline",
                            deadline.span,
                        ));
                    }
                }
                return Ok(Some(ValueType::TaskScope));
            }
            if callee.kind == SyntaxKind::Name
                && let Some(identity) = resolved_compiler_identity(unit, callee)
            {
                let platform_result = match identity {
                    "intrinsic:streams::acquire-stdin"
                    | "intrinsic:streams::acquire-stdout"
                    | "intrinsic:streams::acquire-stderr" => Some(ValueType::PlatformStreamHandle),
                    "intrinsic:system::acquire-filesystem-authority" => {
                        Some(ValueType::FilesystemAuthority)
                    }
                    "intrinsic:streams::open-file"
                    | "intrinsic:streams::open-directory-beneath"
                    | "intrinsic:streams::open-file-beneath" => Some(ValueType::PlatformOpenResult),
                    "intrinsic:streams::read" => Some(ValueType::PlatformReadResult),
                    "intrinsic:streams::read-async" => {
                        return Ok(Some(ValueType::Task(
                            ElementType::new(ValueType::PlatformReadResult),
                            TaskTransferability::Local,
                        )));
                    }
                    "intrinsic:streams::write" => Some(ValueType::PlatformWriteResult),
                    "intrinsic:streams::flush"
                    | "intrinsic:streams::sync-data"
                    | "intrinsic:streams::sync-all"
                    | "intrinsic:streams::close"
                    | "intrinsic:streams::release" => Some(ValueType::PlatformUnitResult),
                    "intrinsic:logging::log-empty-fields" => {
                        Some(ValueType::List(ElementType::new(ValueType::Object(
                            ObjectIdentity::new("/core/logging", "log-field"),
                        ))))
                    }
                    "intrinsic:logging::log-empty-spans"
                    | "intrinsic:logging::log-result-entries" => Some(ValueType::List(
                        ElementType::new(ValueType::Scalar(ScalarType::String)),
                    )),
                    "intrinsic:logging::log-memory-sink"
                    | "intrinsic:logging::log-console-sink"
                    | "intrinsic:logging::log-failing-sink"
                    | "intrinsic:logging::log-write"
                    | "intrinsic:logging::log-drain"
                    | "intrinsic:logging::log-drain-fallback"
                    | "intrinsic:logging::log-install-dependency-bridge"
                    | "intrinsic:testing::test-spawn"
                    | "intrinsic:time::time-wall" => Some(ValueType::PlatformResult),
                    "intrinsic:data::empty-document"
                    | "intrinsic:data::make-document-none"
                    | "intrinsic:data::make-document-bool"
                    | "intrinsic:data::make-document-string"
                    | "intrinsic:data::make-document-integer"
                    | "intrinsic:data::make-document-decimal"
                    | "intrinsic:data::make-document-list"
                    | "intrinsic:data::make-document-map"
                    | "intrinsic:data::document-list-append"
                    | "intrinsic:data::document-map-insert"
                    | "intrinsic:data::json-parse"
                    | "intrinsic:data::json-canonical"
                    | "intrinsic:data::yaml-parse"
                    | "intrinsic:data::document-item"
                    | "intrinsic:data::document-field"
                    | "intrinsic:data::validate-mapping" => Some(ValueType::PlatformDataResult),
                    "intrinsic:data::url-parse" => Some(ValueType::PlatformUrlResult),
                    "intrinsic:logging::log-no-sink"
                    | "intrinsic:logging::log-result-capability"
                    | "intrinsic:capabilities::secure-random"
                    | "intrinsic:capabilities::cancellation-token"
                    | "intrinsic:capabilities::pseudo-random"
                    | "intrinsic:capabilities::secret-buffer"
                    | "intrinsic:capabilities::result-capability"
                    | "intrinsic:concurrency::platform-capability"
                    | "intrinsic:concurrency::no-capability"
                    | "intrinsic:process-signals::process-signal-result-capability"
                    | "intrinsic:process-signals::process-signal-no-capability" => {
                        Some(ValueType::PlatformCapability)
                    }
                    "intrinsic:capabilities::result-resource"
                    | "intrinsic:capabilities::no-resource" => {
                        Some(ValueType::PlatformResourceHandle)
                    }
                    "intrinsic:capabilities::tcp-connect-async"
                    | "intrinsic:capabilities::tcp-connect-host-async"
                    | "intrinsic:capabilities::tcp-accept-async"
                    | "intrinsic:capabilities::tcp-read-async"
                    | "intrinsic:capabilities::tcp-write-async"
                    | "intrinsic:capabilities::udp-send-to-async"
                    | "intrinsic:capabilities::udp-receive-from-async"
                    | "intrinsic:capabilities::dns-lookup-async"
                    | "intrinsic:capabilities::tls-client-async"
                    | "intrinsic:capabilities::tls-read-async"
                    | "intrinsic:capabilities::tls-write-async"
                    | "intrinsic:capabilities::tls-shutdown-async"
                    | "intrinsic:process-signals::process-signal-next" => {
                        return Ok(Some(ValueType::Task(
                            ElementType::new(ValueType::PlatformResult),
                            TaskTransferability::Local,
                        )));
                    }
                    "intrinsic:capabilities::failed-result"
                    | "intrinsic:capabilities::random-bytes"
                    | "intrinsic:capabilities::random-bounded"
                    | "intrinsic:capabilities::random-split"
                    | "intrinsic:capabilities::digest"
                    | "intrinsic:capabilities::hmac"
                    | "intrinsic:capabilities::destroy-secret"
                    | "intrinsic:capabilities::hex-decode"
                    | "intrinsic:capabilities::base64-decode"
                    | "intrinsic:capabilities::uuid-parse"
                    | "intrinsic:capabilities::uuid-v4"
                    | "intrinsic:capabilities::uuid-v7"
                    | "intrinsic:capabilities::compress"
                    | "intrinsic:capabilities::decompress"
                    | "intrinsic:capabilities::parse-ip"
                    | "intrinsic:capabilities::parse-host-name"
                    | "intrinsic:capabilities::parse-socket"
                    | "intrinsic:capabilities::parse-socket-text"
                    | "intrinsic:capabilities::tcp-bind"
                    | "intrinsic:capabilities::tcp-connect"
                    | "intrinsic:capabilities::tcp-connect-host"
                    | "intrinsic:capabilities::tcp-accept"
                    | "intrinsic:capabilities::tcp-read"
                    | "intrinsic:capabilities::tcp-write"
                    | "intrinsic:capabilities::tcp-shutdown"
                    | "intrinsic:capabilities::tcp-configure"
                    | "intrinsic:capabilities::udp-bind"
                    | "intrinsic:capabilities::udp-configure"
                    | "intrinsic:capabilities::udp-send-to"
                    | "intrinsic:capabilities::udp-receive-from"
                    | "intrinsic:capabilities::dns-lookup"
                    | "intrinsic:capabilities::tls-client"
                    | "intrinsic:capabilities::tls-read"
                    | "intrinsic:capabilities::tls-write"
                    | "intrinsic:capabilities::tls-shutdown"
                    | "intrinsic:capabilities::cancel"
                    | "intrinsic:capabilities::close"
                    | "intrinsic:concurrency::platform-result"
                    | "intrinsic:concurrency::int-mutex"
                    | "intrinsic:concurrency::int-read-write-lock"
                    | "intrinsic:concurrency::atomic-int64"
                    | "intrinsic:concurrency::thread-local-int"
                    | "intrinsic:concurrency::int-mutex-load"
                    | "intrinsic:concurrency::int-mutex-store"
                    | "intrinsic:concurrency::int-mutex-add"
                    | "intrinsic:concurrency::int-read-write-lock-read"
                    | "intrinsic:concurrency::int-read-write-lock-write"
                    | "intrinsic:concurrency::atomic-int64-load"
                    | "intrinsic:concurrency::atomic-int64-store"
                    | "intrinsic:concurrency::atomic-int64-add"
                    | "intrinsic:concurrency::thread-local-int-get"
                    | "intrinsic:concurrency::thread-local-int-set"
                    | "intrinsic:adapters::platform-result"
                    | "intrinsic:adapters::system-host-name"
                    | "intrinsic:process-signals::process-signal-subscribe"
                    | "intrinsic:process-signals::process-signal-close" => {
                        Some(ValueType::PlatformResult)
                    }
                    "intrinsic:time::time-sleep-until" => Some(ValueType::Task(
                        ElementType::new(ValueType::Scalar(ScalarType::None)),
                        TaskTransferability::Local,
                    )),
                    "intrinsic:process-signals::process-signal-result-observed" => Some(
                        ValueType::Object(ObjectIdentity::new("/core/time", "monotonic-instant")),
                    ),
                    "intrinsic:system::filesystem-exists"
                    | "intrinsic:system::filesystem-metadata"
                    | "intrinsic:system::filesystem-realpath"
                    | "intrinsic:system::filesystem-read-link"
                    | "intrinsic:system::filesystem-read-bounded"
                    | "intrinsic:system::filesystem-write-atomic"
                    | "intrinsic:system::filesystem-rename"
                    | "intrinsic:system::filesystem-remove" => {
                        Some(ValueType::PlatformFilesystemResult)
                    }
                    "intrinsic:time::time-wall-seconds"
                    | "intrinsic:time::time-wall-nanoseconds"
                    | "intrinsic:time::time-domain"
                    | "intrinsic:time::time-monotonic"
                    | "intrinsic:time::time-div"
                    | "intrinsic:time::time-mod" => Some(ValueType::Scalar(ScalarType::Int)),
                    "intrinsic:logging::log-result-failed"
                    | "intrinsic:system::result-failed"
                    | "intrinsic:system::result-bool"
                    | "intrinsic:system::platform-value-is-text"
                    | "intrinsic:data::data-failed"
                    | "intrinsic:data::url-failed"
                    | "intrinsic:capabilities::constant-time-equal"
                    | "intrinsic:capabilities::result-failed"
                    | "intrinsic:capabilities::result-resource-limit"
                    | "intrinsic:capabilities::result-truncated"
                    | "intrinsic:capabilities::result-deadline-exceeded"
                    | "intrinsic:capabilities::result-bool"
                    | "intrinsic:concurrency::result-failed"
                    | "intrinsic:concurrency::result-bool"
                    | "intrinsic:adapters::result-failed"
                    | "intrinsic:adapters::result-bool"
                    | "intrinsic:process-signals::process-signal-result-failed"
                    | "intrinsic:process-signals::process-signal-result-bool"
                    | "intrinsic:testing::test-result-failed"
                    | "intrinsic:testing::test-result-deadline-exceeded"
                    | "intrinsic:testing::test-result-crashed"
                    | "intrinsic:testing::test-result-stdout-truncated"
                    | "intrinsic:testing::test-result-stderr-truncated"
                    | "intrinsic:testing::test-time-advance" => {
                        Some(ValueType::Scalar(ScalarType::Bool))
                    }
                    "intrinsic:logging::log-result-message"
                    | "intrinsic:system::result-message"
                    | "intrinsic:system::result-text"
                    | "intrinsic:system::result-detail"
                    | "intrinsic:system::platform-value-text"
                    | "intrinsic:data::data-message"
                    | "intrinsic:data::data-path"
                    | "intrinsic:data::data-expected"
                    | "intrinsic:data::data-encoded"
                    | "intrinsic:data::document-kind"
                    | "intrinsic:data::document-text"
                    | "intrinsic:data::document-coefficient"
                    | "intrinsic:data::document-key"
                    | "intrinsic:data::url-message"
                    | "intrinsic:data::url-serialized"
                    | "intrinsic:data::url-display"
                    | "intrinsic:data::url-scheme"
                    | "intrinsic:data::url-username"
                    | "intrinsic:data::url-password"
                    | "intrinsic:data::url-host"
                    | "intrinsic:data::url-port"
                    | "intrinsic:data::url-path"
                    | "intrinsic:data::url-query-key"
                    | "intrinsic:data::url-query-value"
                    | "intrinsic:data::url-fragment"
                    | "intrinsic:data::url-origin"
                    | "intrinsic:capabilities::hex-encode"
                    | "intrinsic:capabilities::base64-encode"
                    | "intrinsic:capabilities::result-message"
                    | "intrinsic:capabilities::result-text"
                    | "intrinsic:capabilities::result-detail"
                    | "intrinsic:concurrency::result-message"
                    | "intrinsic:adapters::result-message"
                    | "intrinsic:adapters::result-text"
                    | "intrinsic:process-signals::process-signal-result-message"
                    | "intrinsic:process-signals::process-signal-result-detail"
                    | "intrinsic:system::platform-value-from-bytes"
                    | "intrinsic:system::platform-value-from-text"
                    | "intrinsic:testing::test-result-message"
                    | "intrinsic:testing::test-render-int"
                    | "intrinsic:testing::test-render-float64"
                    | "intrinsic:testing::test-render-bytes"
                    | "intrinsic:testing::test-render-bool" => {
                        Some(ValueType::Scalar(ScalarType::String))
                    }
                    "intrinsic:system::result-bytes"
                    | "intrinsic:system::platform-value-bytes"
                    | "intrinsic:capabilities::bytes-from-octets"
                    | "intrinsic:testing::test-result-stdout"
                    | "intrinsic:testing::test-result-stderr" => {
                        Some(ValueType::Scalar(ScalarType::Bytes))
                    }
                    "intrinsic:logging::log-discarded-count"
                    | "intrinsic:system::result-int"
                    | "intrinsic:data::document-exponent"
                    | "intrinsic:data::document-length"
                    | "intrinsic:data::url-query-length"
                    | "intrinsic:capabilities::result-int"
                    | "intrinsic:concurrency::result-int"
                    | "intrinsic:process-signals::process-signal-result-int"
                    | "intrinsic:process-signals::process-signal-result-exact-int"
                    | "intrinsic:testing::test-result-exit-code"
                    | "intrinsic:testing::test-deadline-nanoseconds" => {
                        Some(ValueType::Scalar(ScalarType::Int))
                    }
                    "intrinsic:system::process-arguments"
                    | "intrinsic:system::environment-entries"
                    | "intrinsic:capabilities::result-entries" => Some(ValueType::StringList),
                    "intrinsic:system::process-exit" => Some(ValueType::Scalar(ScalarType::None)),
                    _ => None,
                };
                if platform_result.is_some() {
                    return Ok(platform_result);
                }
            }
            if callee.kind == SyntaxKind::MemberExpression
                && let [receiver, member] = callee.children.as_slice()
                && receiver.kind == SyntaxKind::Name
                && let Some(receiver_type) = bindings.iter().rev().find_map(|binding| {
                    (binding.name == node_text(&unit.source, receiver)
                        && binding.is_visible_at(unit.source.id(), receiver.span.start))
                    .then(|| binding.value_type.clone())
                })
                && matches!(
                    receiver_type,
                    ValueType::ChannelSender(_) | ValueType::ChannelReceiver(_)
                )
            {
                let values = arguments
                    .children
                    .iter()
                    .map(|argument| argument.children.last().unwrap_or(argument))
                    .collect::<Vec<_>>();
                return match (receiver_type, node_text(&unit.source, member)) {
                    (ValueType::ChannelSender(item), "send") => {
                        let [value] = values.as_slice() else {
                            return Err(failure(
                                &unit.source,
                                "T0109",
                                "`channel-sender.send` requires one item",
                                node.span,
                            ));
                        };
                        let actual = infer_value_type(unit, value, bindings)?.ok_or_else(|| {
                            failure(
                                &unit.source,
                                "T0109",
                                "channel item type cannot be inferred",
                                value.span,
                            )
                        })?;
                        validate_value_destination(
                            &unit.source,
                            &unit.descriptors,
                            "channel-sender.send",
                            item.value_type(),
                            actual,
                            value,
                            "T0109",
                        )?;
                        Ok(Some(ValueType::Task(
                            ElementType::new(ValueType::ChannelSendOutcome(item)),
                            TaskTransferability::Local,
                        )))
                    }
                    (ValueType::ChannelReceiver(item), "receive") if values.is_empty() => {
                        Ok(Some(ValueType::Task(
                            ElementType::new(ValueType::ChannelReceiveOutcome(item)),
                            TaskTransferability::Local,
                        )))
                    }
                    (ValueType::ChannelSender(_), "close") if values.is_empty() => {
                        Ok(Some(ValueType::Scalar(ScalarType::None)))
                    }
                    (ValueType::ChannelReceiver(item), "close") if values.is_empty() => {
                        Ok(Some(ValueType::List(item)))
                    }
                    (_, operation) => Err(failure(
                        &unit.source,
                        "T0109",
                        format!("invalid channel `{operation}` invocation"),
                        node.span,
                    )),
                };
            }
            if callee.kind == SyntaxKind::MemberExpression
                && let [receiver, member] = callee.children.as_slice()
                && matches!(
                    node_text(&unit.source, member),
                    "spawn" | "join" | "cancel" | "child-scope"
                )
                && infer_value_type(unit, receiver, bindings)? == Some(ValueType::TaskScope)
            {
                return match node_text(&unit.source, member) {
                    "spawn" => {
                        let Some(callable) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.spawn` requires one async callable",
                                node.span,
                            ));
                        };
                        let callable = callable.children.last().unwrap_or(callable);
                        match infer_value_type(unit, callable, bindings)? {
                            Some(
                                ValueType::AsyncFunction(_, result, transferability, _)
                                | ValueType::Task(result, transferability),
                            ) => Ok(Some(ValueType::ScopedTask(result, transferability))),
                            _ => Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.spawn` requires an async callable or task value",
                                callable.span,
                            )),
                        }
                    }
                    "join" => {
                        let Some(task) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.join` requires one scoped task",
                                node.span,
                            ));
                        };
                        let task = task.children.last().unwrap_or(task);
                        match infer_value_type(unit, task, bindings)? {
                            Some(ValueType::ScopedTask(result, _)) => Ok(Some(ValueType::Task(
                                ElementType::new(ValueType::TaskOutcome(result)),
                                TaskTransferability::Local,
                            ))),
                            _ => Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.join` requires a scoped task",
                                task.span,
                            )),
                        }
                    }
                    "cancel" => Ok(Some(ValueType::Scalar(ScalarType::None))),
                    "child-scope" => {
                        let Some(argument) = arguments.children.first() else {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.child-scope` requires one deadline",
                                node.span,
                            ));
                        };
                        let child = argument.children.last().unwrap_or(argument);
                        if infer_value_type(unit, child, bindings)?
                            != Some(ValueType::Object(ObjectIdentity::new(
                                "/core/time",
                                "deadline",
                            )))
                        {
                            return Err(failure(
                                &unit.source,
                                "T0074",
                                "`task-scope.child-scope` requires a `/core/time` deadline",
                                child.span,
                            ));
                        }
                        Ok(Some(ValueType::TaskScope))
                    }
                    _ => Ok(None),
                };
            }
        }
        if let Some(value_type) = infer_collection_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_iterator_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_string_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if node
            .children
            .first()
            .is_some_and(|callee| super::numeric::coercion_family_receiver(unit, callee))
            && let Some(value_type) = infer_numeric_coercion_type(unit, node, bindings)?
        {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_float_call_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_arithmetic_family_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_parse_or_radix_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(value_type) = infer_numeric_coercion_type(unit, node, bindings)? {
            return Ok(Some(value_type));
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let [receiver, member] = callee.children.as_slice()
            && matches!(node_text(&unit.source, member), "concat" | "join")
        {
            let receiver_type = infer_receiver_value_type(unit, receiver, bindings)?;
            if receiver_type == Some(ValueType::Scalar(ScalarType::String)) {
                return Ok(Some(ValueType::Scalar(ScalarType::String)));
            }
            if receiver_type == Some(ValueType::Scalar(ScalarType::Bytes))
                && node_text(&unit.source, member) == "concat"
            {
                if let Some(arguments) = node.children.get(1) {
                    for argument in &arguments.children {
                        let value = argument.children.last().unwrap_or(argument);
                        let actual = infer_value_type(unit, value, bindings)?;
                        if actual != Some(ValueType::Scalar(ScalarType::Bytes)) {
                            return Err(failure(
                                &unit.source,
                                "T0013",
                                format!(
                                    "`.concat` on `bytes` requires `bytes` arguments, found `{}`",
                                    actual.map_or_else(
                                        || "unknown".to_owned(),
                                        |value_type| value_type.to_string()
                                    )
                                ),
                                value.span,
                            ));
                        }
                    }
                }
                return Ok(Some(ValueType::Scalar(ScalarType::Bytes)));
            }
            return Err(failure(
                &unit.source,
                "T0013",
                format!(
                    "`.{}` requires a `string` receiver{}; found `{}`",
                    node_text(&unit.source, member),
                    if node_text(&unit.source, member) == "concat" {
                        " or `bytes` receiver"
                    } else {
                        ""
                    },
                    receiver_type
                        .map_or_else(|| "unknown".to_owned(), |value_type| value_type.to_string())
                ),
                receiver.span,
            ));
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
            && let Some(member_type) = infer_member_call_type(unit, callee, bindings)?
        {
            return match member_type {
                ValueType::Function(_, result, _) => Ok(Some(result.value_type())),
                ValueType::AsyncFunction(_, result, transferability, _) => {
                    Ok(Some(ValueType::Task(result, transferability)))
                }
                _ => Err(failure(
                    &unit.source,
                    "T0039",
                    format!(
                        "`.{}` is a property and cannot be invoked",
                        node_text(
                            &unit.source,
                            callee.children.get(1).expect("member expression")
                        )
                    ),
                    callee.span,
                )),
            };
        }
        if let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::Name
        {
            let name = node_text(&unit.source, callee);
            if unit
                .descriptors
                .iter()
                .any(|object| object.name == name && object.kind == ObjectKind::Class)
            {
                return Err(failure(
                    &unit.source,
                    "T0102",
                    format!("class `{name}` is not callable; construct it with `instance {name};`"),
                    callee.span,
                ));
            }
            let lookup_name = if crate::syntax::call_is_unsafe(node) {
                format!("unsafe::{name}")
            } else {
                name.to_owned()
            };
            if let Some(contract) =
                resolved_function_contract(unit, &lookup_name, callee.span.start)
            {
                if !contract.generic_parameters.is_empty()
                    && let Some((selected, _)) = super::generics::select_unit_callable_contract(
                        None, unit, node, contract, bindings,
                    )
                {
                    let result_type = selected
                        .return_type
                        .clone()
                        .unwrap_or(ValueType::Scalar(ScalarType::None));
                    let result = ElementType::new(result_type);
                    return Ok(Some(if selected.is_async {
                        ValueType::Task(result, selected.task_transferability)
                    } else {
                        result.value_type()
                    }));
                }
                if !contract.generic_parameters.is_empty() {
                    return Ok(None);
                }
                let mut result_type = unit
                    .invocation_scoped_function_results
                    .get(&(contract.span.file, contract.span.start, contract.span.end))
                    .cloned()
                    .or_else(|| contract.return_type.clone())
                    .unwrap_or(ValueType::Scalar(ScalarType::None));
                if let ValueType::InvocationScopedNative {
                    region,
                    expression_scoped: false,
                    ..
                } = &mut result_type
                    && let Some(Some(function_span)) =
                        unit.enclosing_function_spans.get(&node.span.start)
                {
                    *region = Some((function_span.file, function_span.start, function_span.end));
                }
                let result = ElementType::new(result_type);
                return Ok(Some(if contract.is_async {
                    ValueType::Task(result, contract.task_transferability)
                } else {
                    result.value_type()
                }));
            }
            if let Some(binding) = bindings
                .iter()
                .filter(|binding| {
                    binding.name == name
                        && binding.is_visible_at(unit.source.id(), callee.span.start)
                })
                .max_by_key(|binding| binding.visible_from)
            {
                return match &binding.value_type {
                    ValueType::Function(_, result, _) => Ok(Some(result.value_type())),
                    ValueType::AsyncFunction(_, result, transferability, _) => {
                        Ok(Some(ValueType::Task(result.clone(), *transferability)))
                    }
                    _ => Err(failure(
                        &unit.source,
                        "T0039",
                        format!("`{name}` is a value and cannot be called"),
                        callee.span,
                    )),
                };
            }
            return Ok(None);
        }
        if let Some(callee) = node.children.first()
            && let Some(callee_type) = infer_value_type(unit, callee, bindings)?
        {
            return match callee_type {
                ValueType::Function(_, result, _) => Ok(Some(result.value_type())),
                ValueType::AsyncFunction(_, result, transferability, _) => {
                    Ok(Some(ValueType::Task(result, transferability)))
                }
                _ => Err(failure(
                    &unit.source,
                    "T0039",
                    format!("`{callee_type}` value cannot be called"),
                    callee.span,
                )),
            };
        }
        return Ok(None);
    }
    Ok(None)
}
fn missing_static_member_failure(
    unit: &SemanticUnit,
    identity: &ObjectIdentity,
    member: &SyntaxNode,
) -> SemanticFailure {
    let member_name = node_text(&unit.source, member);
    if let Some(removed) = unit.removed_projected_member(identity, member_name, true) {
        return failure(
            &unit.source,
            "S2031",
            format!(
                "Rust dependency member `{}::{member_name}` was projected by version {} but is absent from version {}",
                identity.name, removed.previous_version, removed.current_version
            ),
            member.span,
        );
    }
    failure(
        &unit.source,
        "T0105",
        format!(
            "class `{}` has no static member `{member_name}`",
            identity.name
        ),
        member.span,
    )
}
