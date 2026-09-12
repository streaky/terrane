use super::prelude::*;

pub(super) fn validate_calls(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    let contracts = package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .map(|contract| {
            (
                (contract.span.file, contract.span.start, contract.span.end),
                contract,
            )
        })
        .collect();
    for unit in &package.units {
        let bindings = call_site_bindings(unit, None);
        validate_call_nodes(package, unit, &unit.tree.root, &contracts, None, &bindings)?;
    }
    Ok(())
}

pub(super) fn validate_string_member_expression(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let member = (node.kind == SyntaxKind::MemberExpression)
        .then(|| node.children.get(1))
        .flatten()
        .map(|member| node_text(&unit.source, member));
    let call_member = (node.kind == SyntaxKind::CallExpression)
        .then(|| node.children.first())
        .flatten()
        .filter(|callee| callee.kind == SyntaxKind::MemberExpression)
        .and_then(|callee| callee.children.get(1))
        .map(|member| node_text(&unit.source, member));
    if member == Some("length") || matches!(call_member, Some("concat" | "join")) {
        infer_value_type(unit, node, bindings)?;
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "call validation remains one traversal so every call form shares lexical scope and contracts"
)]
pub(super) fn validate_call_nodes<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    node: &SyntaxNode,
    contracts: &BTreeMap<(u32, usize, usize), &FunctionContract>,
    active_function: Option<&'a FunctionContract>,
    scoped_bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let entered_function = is_function_node(node)
        .then(|| {
            unit.functions
                .iter()
                .find(|contract| contract.span == node.span)
        })
        .flatten();
    let active_function = entered_function.or(active_function);
    let function_bindings =
        entered_function.map(|contract| call_site_bindings(unit, Some(contract)));
    let scoped_bindings = function_bindings.as_deref().unwrap_or(scoped_bindings);
    if node.kind == SyntaxKind::UnaryExpression
        && unary_operator_text(unit, node).as_deref() == Some("await")
        && !active_function.is_some_and(|function| function.is_async)
    {
        return Err(failure(
            &unit.source,
            "T0028",
            "`await` is valid only inside an async callable",
            node.span,
        ));
    }

    validate_resolved_assignment(package, unit, node, contracts)?;
    validate_numeric_coercion_call(unit, node, scoped_bindings)?;
    if node.kind == SyntaxKind::CallExpression
        && let Some(arguments) = node.children.get(1)
    {
        for argument in &arguments.children {
            let value = argument.children.last().unwrap_or(argument);
            infer_value_type(unit, value, scoped_bindings)?;
        }
    }
    if node.kind == SyntaxKind::CallExpression {
        validate_projected_generic_arguments(package, unit, node, scoped_bindings)?;
    }
    if node.kind == SyntaxKind::CallExpression {
        let inferred = infer_value_type(unit, node, scoped_bindings)?;
        if inferred.is_none()
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
        {
            infer_member_value_type(unit, callee, scoped_bindings)?;
        }
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::Name
        && package
            .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
            .is_some_and(|symbol| symbol.identity == "/core/output::print")
    {
        for argument in &arguments.children {
            let value = argument.children.last().unwrap_or(argument);
            validate_call_nodes(
                package,
                unit,
                value,
                contracts,
                active_function,
                scoped_bindings,
            )?;
            let value_type =
                transparent_value_type(infer_value_type(unit, value, scoped_bindings)?);
            if !matches!(
                value_type,
                Some(
                    ValueType::Scalar(
                        ScalarType::Bool
                            | ScalarType::Int
                            | ScalarType::Int8
                            | ScalarType::Int16
                            | ScalarType::Int32
                            | ScalarType::Int64
                            | ScalarType::Int128
                            | ScalarType::Uint8
                            | ScalarType::Uint16
                            | ScalarType::Uint32
                            | ScalarType::Uint64
                            | ScalarType::Uint128
                            | ScalarType::Float32
                            | ScalarType::Float64
                            | ScalarType::String
                            | ScalarType::None
                    ) | ValueType::Descriptor(_)
                )
            ) {
                return Err(failure(
                    &unit.source,
                    "T0035",
                    format!(
                        "`print` requires a text-displayable scalar value, found {}",
                        value_type.map_or_else(|| "unknown".to_owned(), |ty| ty.to_string())
                    ),
                    value.span,
                ));
            }
        }
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && callee.kind == SyntaxKind::Name
        && let Some(binding) = scoped_bindings.iter().rev().find(|binding| {
            binding.name == node_text(&unit.source, callee)
                && binding.is_visible_at(unit.source.id(), callee.span.start)
        })
        && let ValueType::Function(parameters, _, _) = &binding.value_type
    {
        if arguments.children.len() != parameters.len() {
            return Err(failure(
                &unit.source,
                "T0012",
                format!(
                    "callable expects {} arguments, found {}",
                    parameters.len(),
                    arguments.children.len()
                ),
                arguments.span,
            ));
        }
        for (argument, expected) in arguments.children.iter().zip(parameters) {
            if argument.children.len() > 1 {
                return Err(failure(
                    &unit.source,
                    "T0012",
                    "calls through function values use positional arguments",
                    argument.span,
                ));
            }
            let value = argument.children.last().unwrap_or(argument);
            if let Some(actual) = infer_value_type(unit, value, scoped_bindings)? {
                validate_value_destination(
                    &unit.source,
                    &unit.descriptors,
                    "callable argument",
                    expected.value_type(),
                    actual,
                    value,
                    "T0012",
                )?;
            }
        }
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
    {
        let contract = match callee.kind {
            SyntaxKind::Name => package
                .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
                .filter(|symbol| symbol.kind == SymbolKind::Function)
                .and_then(|symbol| symbol.declaration_span)
                .and_then(|declaration_span| {
                    contracts
                        .get(&(
                            declaration_span.file,
                            declaration_span.start,
                            declaration_span.end,
                        ))
                        .copied()
                }),
            SyntaxKind::MemberExpression => match callee.children.as_slice() {
                [receiver, member] => infer_value_type(unit, receiver, scoped_bindings)
                    .ok()
                    .flatten()
                    .and_then(|value_type| {
                        let ValueType::Object(identity) = value_type else {
                            return None;
                        };
                        method_contract(package, &identity, node_text(&unit.source, member), false)
                    }),
                _ => None,
            },
            SyntaxKind::StaticMemberExpression => match callee.children.as_slice() {
                [receiver, member] => {
                    class_designator_identity(unit, receiver).and_then(|identity| {
                        method_contract(package, &identity, node_text(&unit.source, member), true)
                    })
                }
                _ => None,
            },
            SyntaxKind::ConstructionExpression => construction_contract(package, unit, callee),
            _ => None,
        };
        if let Some(contract) = contract {
            validate_call_arguments(unit, arguments, contract, scoped_bindings)?;
        }
    }
    if let [target, collection, block] = node.children.as_slice()
        && node.kind == SyntaxKind::ForStatement
        && target.kind == SyntaxKind::ForTarget
    {
        validate_call_nodes(
            package,
            unit,
            collection,
            contracts,
            active_function,
            scoped_bindings,
        )?;
        let collection_type =
            infer_value_type(unit, collection, scoped_bindings)?.ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0016",
                    "collection iteration requires an iterable value",
                    collection.span,
                )
            })?;
        let item_type = iterable_item_type(unit, collection_type).map_err(|(message, span)| {
            failure(
                &unit.source,
                "T0016",
                message,
                span.unwrap_or(collection.span),
            )
        })?;
        let mut loop_bindings = scoped_bindings.to_vec();
        loop_bindings.extend(iteration_target_bindings(
            unit,
            target,
            collection.span.end,
            block.span,
            item_type,
        )?);
        validate_call_nodes(
            package,
            unit,
            block,
            contracts,
            active_function,
            &loop_bindings,
        )?;
        return Ok(());
    }
    validate_string_member_expression(unit, node, scoped_bindings)?;
    validate_coercion_family_expression(unit, node)?;
    for (index, child) in node.children.iter().enumerate() {
        if node.kind == SyntaxKind::CallExpression
            && index == 0
            && let Some((source, _)) = numeric_coercion_call(&unit.source, child)
        {
            validate_call_nodes(
                package,
                unit,
                source,
                contracts,
                active_function,
                scoped_bindings,
            )?;
            continue;
        }
        validate_call_nodes(
            package,
            unit,
            child,
            contracts,
            active_function,
            scoped_bindings,
        )?;
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "projected source-generic and boxed-interface obligations share one argument pass"
)]
fn validate_projected_generic_arguments(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let [callee, arguments] = node.children.as_slice() else {
        return Ok(());
    };
    let Some(projected) = projected_function_for_call(package, unit, callee) else {
        return Ok(());
    };
    for (argument, parameter) in arguments.children.iter().zip(&projected.parameters) {
        if parameter.generic_parameter.is_none() {
            continue;
        }
        let value = argument.children.last().unwrap_or(argument);
        let Some(ValueType::Object(identity)) = infer_value_type(unit, value, bindings)? else {
            return Err(failure(
                &unit.source,
                "T0121",
                "immediate projected generic bounds require a concrete source class argument",
                value.span,
            ));
        };
        let Some((_, implementor)) = package.units.iter().find_map(|owner| {
            owner
                .descriptors
                .iter()
                .find(|descriptor| descriptor.identity == identity)
                .map(|descriptor| (owner, descriptor))
        }) else {
            return Err(failure(
                &unit.source,
                "T0121",
                "projected interface bounds require a source object argument",
                value.span,
            ));
        };
        let expected_rust_path = match &parameter.ty {
            crate::projection::ProjectedType::BoxedInterface { trait_path, .. } => {
                trait_path.clone()
            }
            _ => parameter.ty.rust_type(),
        };
        let expected_base = expected_rust_path
            .split_once('<')
            .map_or(expected_rust_path.as_str(), |(base, _)| base);
        let required_item = package
            .projection
            .dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| {
                item.rust_path == expected_base
                    && matches!(item.kind, crate::projection::ProjectedKind::Interface(_))
            });
        let required = required_item.map(|item| ObjectIdentity::new(&item.namespace, &item.name));
        let boxed_interface = matches!(
            parameter.ty,
            crate::projection::ProjectedType::BoxedInterface { .. }
        );
        let implemented = required.as_ref().and_then(|required| {
            implementor.interfaces.iter().find(|implemented| {
                implemented.namespace == required.namespace && implemented.name == required.name
            })
        });
        let implements_required = implemented.is_some()
            || required.as_ref().is_some_and(|required| {
                boxed_interface
                    && implementor.kind == ObjectKind::Interface
                    && implementor.identity.base() == *required
            });
        if !implements_required {
            return Err(failure(
                &unit.source,
                "T0121",
                format!(
                    "object `{}` does not implement the projected bound `{expected_rust_path}`",
                    implementor.identity.name
                ),
                value.span,
            ));
        }
        if let Some(crate::projection::ProjectedKind::Interface(interface)) =
            required_item.map(|item| &item.kind)
            && let Some(associated) = &interface.associated_type
            && let Some(expected) = parameter.associated_type.as_ref()
        {
            let application = implemented
                .and_then(|implemented| implemented.application.as_deref())
                .or_else(|| {
                    (boxed_interface
                        && implementor.kind == ObjectKind::Interface
                        && implementor.identity.base()
                            == required
                                .as_ref()
                                .expect("resolved projected interface")
                                .base())
                    .then(|| implementor.identity.application.as_deref())
                    .flatten()
                });
            let actual = application
                .map(|application| destination_projected_type(package, application))
                .transpose()
                .map_err(|_| {
                    failure(
                        &unit.source,
                        "T0121",
                        "projected associated type is not representable at the generic call",
                        value.span,
                    )
                })?;
            if actual.as_ref() != Some(expected.ty.as_ref()) {
                return Err(failure(
                    &unit.source,
                    "T0121",
                    format!(
                        "object `{}` binds projected associated type `{}` incompatibly with `{}`",
                        implementor.identity.name,
                        associated.name,
                        expected.ty.rust_type()
                    ),
                    value.span,
                ));
            }
        }
        if implementor.kind != ObjectKind::Class && !boxed_interface {
            return Err(failure(
                &unit.source,
                "T0121",
                "interface-typed values cannot select a projected source generic bound",
                value.span,
            ));
        }
        if implementor.kind == ObjectKind::Interface
            && let crate::projection::ProjectedType::BoxedInterface { auto_traits, .. } =
                &parameter.ty
            && let Some((send, sync)) = package
                .projection
                .foreign_auto_traits(&implementor.identity.namespace, &implementor.identity.name)
            && let Some(requirement) = auto_traits.iter().find(|requirement| {
                requirement.ends_with("::Send") && !send || requirement.ends_with("::Sync") && !sync
            })
        {
            return Err(failure(
                &unit.source,
                "T0126",
                format!(
                    "interface `{}` cannot cross the boxed projected boundary because its erased wrapper does not satisfy `{requirement}`",
                    implementor.identity.name
                ),
                value.span,
            ));
        }
        let requires_static = parameter
            .generic_bounds
            .iter()
            .any(|bound| matches!(bound.as_str(), "'static" | "static"));
        let requires_send = parameter
            .generic_bounds
            .iter()
            .any(|bound| bound.ends_with("::Send") || bound == "Send");
        let requires_sync = parameter
            .generic_bounds
            .iter()
            .any(|bound| bound.ends_with("::Sync") || bound == "Sync");
        if implementor.kind == ObjectKind::Class {
            for (required, requirement, obligation) in [
                (requires_send, "`Send`", Some(AutoTraitObligation::Send)),
                (requires_sync, "`Sync`", Some(AutoTraitObligation::Sync)),
                (requires_static, "`'static`", None),
            ] {
                if required
                    && let Some(field) = effective_object_fields(package, implementor)
                        .into_iter()
                        .find(|field| {
                            !field.is_static
                                && obligation.map_or_else(
                                    || !value_type_is_owned_static(&field.value_type),
                                    |obligation| {
                                        !value_type_satisfies_auto_trait(
                                            package,
                                            &field.value_type,
                                            obligation,
                                        )
                                    },
                                )
                        })
                {
                    return Err(failure(
                        &unit.source,
                        "T0126",
                        format!(
                            "class `{}` cannot cross the retained projected boundary because field `{}` does not satisfy its {requirement} obligation",
                            implementor.identity.name, field.name
                        ),
                        value.span,
                    ));
                }
            }
        }
    }
    Ok(())
}

pub(super) fn validate_numeric_coercion_call(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::CallExpression {
        infer_numeric_coercion_type(unit, node, bindings)?;
    }
    Ok(())
}

pub(super) fn validate_coercion_family_expression(
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::MemberExpression && coercion_family_receiver(unit, node) {
        return Err(failure(
            &unit.source,
            "T0018",
            "`.coerce` and its policy members are not storable values before bound methods exist",
            node.span,
        ));
    }
    Ok(())
}

pub(super) fn validate_resolved_assignment(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contracts: &BTreeMap<(u32, usize, usize), &FunctionContract>,
) -> Result<(), SemanticFailure> {
    if !matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
        return Ok(());
    }
    let Some(name_node) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
    else {
        return Ok(());
    };
    let Some(initializer) = node.children.iter().rev().find(|child| {
        child.span != name_node.span
            && !matches!(
                child.kind,
                SyntaxKind::Visibility
                    | SyntaxKind::DeclarationQualifier
                    | SyntaxKind::TypeExpression
            )
    }) else {
        return Ok(());
    };
    let actual = if let Some(actual) = resolved_call_type(package, unit, initializer, contracts) {
        actual
    } else if let Some(actual) =
        infer_collection_call_type(unit, initializer, &unit.typed_bindings)?
    {
        actual
    } else if initializer.kind != SyntaxKind::CallExpression {
        let Some(actual) = infer_value_type(unit, initializer, &unit.typed_bindings)? else {
            return Ok(());
        };
        actual
    } else {
        return Ok(());
    };
    let name = node_text(&unit.source, name_node);
    let Some(expected) = unit
        .typed_bindings
        .iter()
        .rev()
        .find(|binding| {
            binding.name == name
                && if node.kind == SyntaxKind::Binding {
                    binding.span == node.span
                } else {
                    binding.is_visible_at(unit.source.id(), node.span.start)
                }
        })
        .map(|binding| binding.value_type.clone())
    else {
        return Ok(());
    };
    validate_value_destination(
        &unit.source,
        &unit.descriptors,
        name,
        expected,
        actual,
        initializer,
        "T0002",
    )
}

pub(super) fn resolved_call_type(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    contracts: &BTreeMap<(u32, usize, usize), &FunctionContract>,
) -> Option<ValueType> {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .and_then(|child| resolved_call_type(package, unit, child, contracts));
    }
    if let Some(specialization) =
        unit.projected_call_specializations
            .get(&(node.span.file, node.span.start, node.span.end))
    {
        return Some(specialization.value_type.clone());
    }
    let [callee, _arguments] = node.children.as_slice() else {
        return None;
    };
    if node.kind != SyntaxKind::CallExpression || callee.kind != SyntaxKind::Name {
        return None;
    }
    let symbol =
        package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
    let declaration = symbol.declaration_span?;
    let contract = contracts.get(&(declaration.file, declaration.start, declaration.end))?;
    let result = ElementType::new(
        contract
            .return_type
            .clone()
            .unwrap_or(ValueType::Scalar(ScalarType::None)),
    );
    Some(if contract.is_async {
        ValueType::Task(result, contract.task_transferability)
    } else {
        result.value_type()
    })
}

pub(super) fn validate_call_arguments(
    unit: &SemanticUnit,
    arguments: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let mut bound = BTreeSet::new();
    let mut positional = 0;
    let mut named_seen = false;
    for argument in &arguments.children {
        let name = argument
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
        let parameter = if let Some(name) = name {
            named_seen = true;
            let name_text = node_text(&unit.source, name);
            contract
                .parameters
                .iter()
                .find(|parameter| parameter.name == name_text)
                .ok_or_else(|| {
                    failure(
                        &unit.source,
                        "T0012",
                        format!(
                            "function `{}` has no parameter named `{name_text}`",
                            contract.name
                        ),
                        name.span,
                    )
                })?
        } else {
            if named_seen {
                return Err(failure(
                    &unit.source,
                    "T0012",
                    "positional arguments must precede named arguments",
                    argument.span,
                ));
            }
            let parameter = contract.parameters.get(positional).ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0012",
                    format!("too many arguments for function `{}`", contract.name),
                    argument.span,
                )
            })?;
            positional += 1;
            parameter
        };
        if !bound.insert(parameter.name.as_str()) {
            return Err(failure(
                &unit.source,
                "T0012",
                format!("parameter `{}` is bound more than once", parameter.name),
                argument.span,
            ));
        }
        let value = argument.children.last().unwrap_or(argument);
        if let Some(expected) = parameter.value_type.clone() {
            if contextual_collection_constructor_matches(unit, value, &expected, bindings) {
                validate_collection_constructor_value(
                    unit,
                    value,
                    &expected,
                    &parameter.name,
                    bindings,
                )?;
            } else if let Some(actual) = infer_value_type(unit, value, bindings)? {
                validate_value_destination(
                    &unit.source,
                    &unit.descriptors,
                    &parameter.name,
                    expected,
                    actual,
                    value,
                    "T0012",
                )?;
            }
        }
    }
    if let Some(missing) = contract
        .parameters
        .iter()
        .find(|parameter| !parameter.optional && !bound.contains(parameter.name.as_str()))
    {
        return Err(failure(
            &unit.source,
            "T0012",
            format!("missing required argument `{}`", missing.name),
            arguments.span,
        ));
    }
    Ok(())
}

pub(super) fn call_site_bindings(
    unit: &SemanticUnit,
    active_function: Option<&FunctionContract>,
) -> Vec<TypedBinding> {
    let mut bindings = unit
        .typed_bindings
        .iter()
        .filter(|binding| {
            let owner = unit
                .functions
                .iter()
                .filter(|function| {
                    function.span.file == binding.span.file
                        && function.span.start <= binding.span.start
                        && binding.span.end <= function.span.end
                })
                .min_by_key(|function| function.span.end - function.span.start);
            owner
                .is_none_or(|owner| active_function.is_some_and(|active| active.span == owner.span))
        })
        .cloned()
        .collect::<Vec<_>>();
    if let Some(function) = active_function {
        bindings.extend(function.parameters.iter().filter_map(|parameter| {
            parameter.value_type.clone().map(|value_type| TypedBinding {
                name: parameter.name.clone(),
                span: parameter.span,
                visible_from: parameter.span.start,
                scope: Some(function.span),
                value_type,
                destination_arms: Vec::new(),
                storage_type: None,
                mutable: false,
            })
        }));
    }
    bindings
}

pub(super) fn descriptor_construct_alias_history(
    package: &SemanticPackage,
    unit: &SemanticUnit,
) -> BTreeMap<String, Vec<DescriptorAlias>> {
    let mut aliases = package
        .descriptor_constructs
        .iter()
        .filter_map(|(name, symbol)| Some((name.clone(), symbol.descriptor_identity()?.to_owned())))
        .collect::<BTreeMap<_, _>>();
    if let Some(namespace) = package.namespaces.get(&unit.namespace) {
        aliases.extend(namespace.symbols.iter().filter_map(|(name, symbol)| {
            Some((name.clone(), symbol.descriptor_identity()?.to_owned()))
        }));
    }
    aliases
        .into_iter()
        .map(|(name, identity)| {
            (
                name,
                vec![DescriptorAlias {
                    visible_from: 0,
                    scope: None,
                    identity,
                }],
            )
        })
        .collect()
}
