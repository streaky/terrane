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
    if matches!(
        node.kind,
        SyntaxKind::ImportDeclaration
            | SyntaxKind::ObjectImport
            | SyntaxKind::ImportSelection
            | SyntaxKind::ImportAlias
            | SyntaxKind::NamespaceDeclaration
    ) {
        return Ok(());
    }
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
    if node.kind == SyntaxKind::Name && projected_macro_for_call(package, unit, node).is_some() {
        return Err(failure(
            &unit.source,
            "T0119",
            "native macros are invocation-only operations, not callable values",
            node.span,
        ));
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && projected_macro_for_call(package, unit, callee).is_some()
    {
        if crate::syntax::call_is_unsafe(node) {
            return Err(failure(
                &unit.source,
                "T0119",
                "native macro invocations must satisfy the safe Rust expression contract",
                node.span,
            ));
        }
        for argument in &arguments.children {
            if argument.children.len() > 1 {
                return Err(failure(
                    &unit.source,
                    "T0012",
                    "native macro invocations use positional expression arguments",
                    argument.span,
                ));
            }
            validate_call_nodes(
                package,
                unit,
                argument.children.last().unwrap_or(argument),
                contracts,
                active_function,
                scoped_bindings,
            )?;
        }
        return Ok(());
    }
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
    if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
    {
        let requested_unsafe = crate::syntax::call_is_unsafe(node);
        let selected =
            function_contract_for_call_with_safety(package, unit, callee, requested_unsafe);
        let opposite =
            function_contract_for_call_with_safety(package, unit, callee, !requested_unsafe);
        if selected.is_none() && (requested_unsafe || opposite.is_some()) {
            let invocation = node_text(&unit.source, callee);
            let name = callee
                .children
                .last()
                .map_or(invocation, |member| node_text(&unit.source, member));
            let (message, help) = if requested_unsafe && opposite.is_some() {
                (
                    format!("`{name}` has no unsafe function declaration"),
                    format!("remove `unsafe` to call the safe `{name}` declaration"),
                )
            } else if requested_unsafe {
                (
                    format!("unsafe call `{name}` has no matching unsafe function declaration"),
                    format!("declare `unsafe function {name}` before calling it as unsafe"),
                )
            } else {
                (
                    format!("`{name}` is declared only as an unsafe function"),
                    format!("write `unsafe {invocation}; ...` to select the unsafe declaration"),
                )
            };
            return Err(SemanticFailure {
                source: unit.source.clone(),
                diagnostics: vec![Diagnostic::error("T0130", message, node.span).with_help(help)],
            });
        }
    }
    if node.kind == SyntaxKind::CallExpression {
        let inferred = infer_value_type(unit, node, scoped_bindings)?;
        if inferred.is_none()
            && let Some(callee) = node.children.first()
            && callee.kind == SyntaxKind::MemberExpression
        {
            infer_member_call_type(unit, callee, scoped_bindings)?;
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
        && let ValueType::Function(parameters, _, _) | ValueType::AsyncFunction(parameters, _, _, _) =
            &binding.value_type
    {
        let variadic = parameters
            .last()
            .is_some_and(CallableParameterType::is_variadic);
        let fixed = parameters.len() - usize::from(variadic);
        if arguments.children.len() < fixed
            || (!variadic && arguments.children.len() != parameters.len())
        {
            let expected = if variadic {
                format!("at least {fixed}")
            } else {
                parameters.len().to_string()
            };
            return Err(failure(
                &unit.source,
                "T0012",
                format!(
                    "callable expects {expected} arguments, found {}",
                    arguments.children.len()
                ),
                arguments.span,
            ));
        }
        for (index, argument) in arguments.children.iter().enumerate() {
            if argument.children.len() > 1 {
                return Err(failure(
                    &unit.source,
                    "T0012",
                    "calls through function values use positional arguments",
                    argument.span,
                ));
            }
            let expected = parameters
                .get(index)
                .or_else(|| parameters.last())
                .expect("validated callable argument has a parameter");
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
        let mut callee = callee;
        while matches!(
            callee.kind,
            SyntaxKind::GroupExpression | SyntaxKind::TypeExpression
        ) {
            let Some(inner) = callee.children.first() else {
                break;
            };
            callee = inner;
        }
        if callee.kind == SyntaxKind::ConstructionExpression
            && let Some(class) = callee.children.first()
            && let Some(identity) = class_designator_identity(unit, class)
            && let Some(item) = package.projection.item(&identity.namespace, &identity.name)
            && matches!(
                item.kind,
                crate::rust_interop::projection::ProjectedKind::ForeignType { .. }
                    | crate::rust_interop::projection::ProjectedKind::Enum { .. }
            )
            && package
                .projection
                .projected_constructor(&identity.namespace, &identity.name)
                .is_none()
            && package
                .projection
                .projected_struct(&identity.namespace, &identity.name)
                .is_none_or(|(_, fields, _)| fields.is_empty())
        {
            return Err(failure(
                &unit.source,
                "T0117",
                format!(
                    "native class `{}` has no projected source constructor; use an admitted factory or enum variant",
                    identity.name
                ),
                callee.span,
            ));
        }
        let designator = if callee.kind == SyntaxKind::AppliedType {
            callee.children.first().unwrap_or(callee)
        } else {
            callee
        };
        let contract = function_contract_for_call_with_safety(
            package,
            unit,
            designator,
            crate::syntax::call_is_unsafe(node),
        );
        let selected_contract = super::calls::selected_callable_contract(
            package,
            unit,
            node,
            crate::syntax::call_is_unsafe(node),
        );
        if selected_contract.is_none()
            && let Some(contract) = contract
            && !contract.generic_parameters.is_empty()
        {
            use super::generics::GenericSelectionFailure;
            let selection = super::generics::select_unit_callable_contract_result(
                Some(package),
                unit,
                node,
                contract,
                scoped_bindings,
            );
            let message = match selection {
                Err(GenericSelectionFailure::Unselected(parameters)) => format!(
                    "generic type parameter {} is unselected; write an explicit type argument or a destination selecting the result",
                    parameters
                        .iter()
                        .map(|name| format!("`{name}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Err(GenericSelectionFailure::Bound {
                    parameter,
                    actual,
                    bound,
                }) => format!(
                    "type argument `{actual}` for `{parameter}` does not satisfy interface bound `{bound}`"
                ),
                Err(GenericSelectionFailure::Argument(message)) => message,
                Err(GenericSelectionFailure::InvalidArguments) | Ok(_) => {
                    "generic callable arguments cannot select a valid application".to_owned()
                }
            };
            return Err(failure(&unit.source, "T0012", message, callee.span));
        }
        let base_contract = selected_contract.as_ref().or(contract);
        if let Some(contract) = base_contract {
            let specialized = unit
                .projected_call_specializations
                .get(&(node.span.file, node.span.start, node.span.end))
                .map(|specialization| {
                    let mut selected = contract.clone();
                    for (parameter, value_type) in selected
                        .parameters
                        .iter_mut()
                        .zip(&specialization.value_parameters)
                    {
                        if let Some(value_type) = value_type {
                            parameter.value_type = Some(value_type.clone());
                        }
                    }
                    selected
                });
            validate_call_arguments(
                unit,
                arguments,
                specialized.as_ref().unwrap_or(contract),
                scoped_bindings,
            )?;
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
        let item_type =
            iterable_item_type(unit, collection_type).map_err(|(code, message, span)| {
                failure(&unit.source, code, message, span.unwrap_or(collection.span))
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
        if node.kind.child_field(index, child.kind) == "name" {
            continue;
        }
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
    let Some(projected) =
        projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
    else {
        return Ok(());
    };
    for (argument, parameter) in arguments.children.iter().zip(&projected.parameters) {
        if parameter.generic_parameter.is_none()
            || (parameter.generic_interface.is_none()
                && !matches!(
                    parameter.ty,
                    crate::rust_interop::projection::ProjectedType::BoxedInterface { .. }
                ))
        {
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
            crate::rust_interop::projection::ProjectedType::BoxedInterface {
                trait_path, ..
            } => trait_path.clone(),
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
                    && matches!(
                        item.kind,
                        crate::rust_interop::projection::ProjectedKind::Interface(_)
                    )
            });
        let required = required_item.map(|item| ObjectIdentity::new(&item.namespace, &item.name));
        let boxed_interface = matches!(
            parameter.ty,
            crate::rust_interop::projection::ProjectedType::BoxedInterface { .. }
        );
        let interface_matches_required = required.as_ref().is_some_and(|required| {
            boxed_interface
                && implementor.kind == ObjectKind::Interface
                && implementor.identity.base() == *required
        });
        let effective_interfaces =
            crate::semantics::effective_object_interfaces(package, implementor);
        let implemented = required.as_ref().and_then(|required| {
            effective_interfaces.into_iter().find(|implemented| {
                implemented.namespace == required.namespace && implemented.name == required.name
            })
        });
        let implements_required = implemented.is_some() || interface_matches_required;
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
        if let Some(crate::rust_interop::projection::ProjectedKind::Interface(interface)) =
            required_item.map(|item| &item.kind)
            && let Some(associated) = &interface.associated_type
            && let Some(expected) = parameter.associated_type.as_ref()
        {
            let application = implemented
                .and_then(|implemented| implemented.application.as_deref())
                .or_else(|| {
                    interface_matches_required
                        .then_some(implementor.identity.application.as_deref())
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
            && let crate::rust_interop::projection::ProjectedType::BoxedInterface {
                auto_traits,
                ..
            } = &parameter.ty
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
    let name_node = if node.kind == SyntaxKind::Assignment {
        node.children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name)
    } else {
        node.children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
    };
    let Some(name_node) = name_node else {
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
    let mut actual = if let Some(actual) = resolved_call_type(package, unit, initializer, contracts)
    {
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
    if initializer.kind == SyntaxKind::Name
        && let Some(contract) = function_contract_for_call(package, unit, initializer)
        && !contract.generic_parameters.is_empty()
        && let Some(closed) =
            super::generics::close_generic_callable_value(unit, contract, &actual, &expected)
    {
        actual = closed;
    }
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

pub(super) fn bind_projected_generics(
    expected: &ValueType,
    actual: &ValueType,
    bindings: &mut BTreeMap<String, ValueType>,
) -> Result<(), String> {
    if let ValueType::ProjectedGeneric(name) = expected {
        if let Some(previous) = bindings.get(name) {
            return (previous == actual)
                .then_some(())
                .ok_or_else(|| name.clone());
        }
        bindings.insert(name.clone(), actual.clone());
        return Ok(());
    }
    match (expected, actual) {
        (
            ValueType::Reference(expected) | ValueType::SharedReference(expected),
            ValueType::Reference(actual) | ValueType::SharedReference(actual),
        )
        | (ValueType::List(expected), ValueType::List(actual))
        | (ValueType::Set(expected), ValueType::Set(actual))
        | (ValueType::UnorderedSet(expected), ValueType::UnorderedSet(actual))
        | (ValueType::Iterator(expected), ValueType::Iterator(actual)) => {
            bind_projected_generics(expected.value_type_ref(), actual.value_type_ref(), bindings)
        }
        (ValueType::Reference(expected) | ValueType::SharedReference(expected), actual) => {
            bind_projected_generics(expected.value_type_ref(), actual, bindings)
        }
        (ValueType::Optional(expected), ValueType::Optional(actual)) => {
            bind_projected_generics(expected, actual, bindings)
        }
        (ValueType::Optional(expected), actual)
            if actual != &ValueType::Scalar(ScalarType::None) =>
        {
            bind_projected_generics(expected, actual, bindings)
        }
        (
            ValueType::Map(expected_key, expected_value),
            ValueType::Map(actual_key, actual_value),
        )
        | (
            ValueType::UnorderedMap(expected_key, expected_value),
            ValueType::UnorderedMap(actual_key, actual_value),
        ) => {
            bind_projected_generics(
                expected_key.value_type_ref(),
                actual_key.value_type_ref(),
                bindings,
            )?;
            bind_projected_generics(
                expected_value.value_type_ref(),
                actual_value.value_type_ref(),
                bindings,
            )
        }
        (
            ValueType::Tuple(expected_item, expected_length),
            ValueType::Tuple(actual_item, actual_length),
        ) if expected_length == actual_length => bind_projected_generics(
            expected_item.value_type_ref(),
            actual_item.value_type_ref(),
            bindings,
        ),
        (ValueType::Object(expected), ValueType::Object(actual))
            if expected.namespace == actual.namespace
                && expected.name == actual.name
                && expected.is_unsafe == actual.is_unsafe
                && expected.type_arguments.len() == actual.type_arguments.len() =>
        {
            for (expected, actual) in expected.type_arguments.iter().zip(&actual.type_arguments) {
                bind_projected_generics(expected, actual, bindings)?;
            }
            for (name, expected) in &expected.native_arguments {
                let actual = actual
                    .native_arguments
                    .get(name)
                    .ok_or_else(|| name.clone())?;
                bind_projected_generics(expected, actual, bindings)?;
            }
            Ok(())
        }
        (
            ValueType::Function(expected_parameters, expected_result, _),
            ValueType::Function(actual_parameters, actual_result, _),
        )
        | (
            ValueType::AsyncFunction(expected_parameters, expected_result, _, _),
            ValueType::AsyncFunction(actual_parameters, actual_result, _, _),
        ) if expected_parameters.len() == actual_parameters.len() => {
            for (expected, actual) in expected_parameters.iter().zip(actual_parameters) {
                bind_projected_generics(
                    expected.value_type_ref(),
                    actual.value_type_ref(),
                    bindings,
                )?;
            }
            bind_projected_generics(
                expected_result.value_type_ref(),
                actual_result.value_type_ref(),
                bindings,
            )
        }
        _ => projected_generic_name(expected).map_or(Ok(()), Err),
    }
}

fn projected_generic_name(value_type: &ValueType) -> Option<String> {
    match value_type {
        ValueType::ProjectedGeneric(name) => Some(name.clone()),
        ValueType::Optional(inner) => projected_generic_name(inner),
        ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::UnorderedSet(item)
        | ValueType::Tuple(item, _)
        | ValueType::Iterator(item) => projected_generic_name(item.value_type_ref()),
        ValueType::Map(key, value) | ValueType::UnorderedMap(key, value) => {
            projected_generic_name(key.value_type_ref())
                .or_else(|| projected_generic_name(value.value_type_ref()))
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => parameters
            .iter()
            .find_map(|parameter| projected_generic_name(parameter.value_type_ref()))
            .or_else(|| projected_generic_name(result.value_type_ref())),
        _ => None,
    }
}

fn substitute_callable_parameter_generics(
    parameter: &CallableParameterType,
    bindings: &BTreeMap<String, ValueType>,
) -> CallableParameterType {
    let value_type = substitute_projected_value_generics(parameter.value_type_ref(), bindings);
    if parameter.is_variadic() {
        CallableParameterType::variadic(ElementType::new(value_type))
    } else {
        CallableParameterType::fixed(ElementType::new(value_type))
    }
}

pub(super) fn substitute_projected_value_generics(
    value_type: &ValueType,
    bindings: &BTreeMap<String, ValueType>,
) -> ValueType {
    match value_type {
        ValueType::ProjectedGeneric(name) => bindings
            .get(name)
            .cloned()
            .unwrap_or_else(|| value_type.clone()),
        ValueType::Reference(inner) => ValueType::Reference(ElementType::new(
            substitute_projected_value_generics(inner.value_type_ref(), bindings),
        )),
        ValueType::SharedReference(inner) => ValueType::SharedReference(ElementType::new(
            substitute_projected_value_generics(inner.value_type_ref(), bindings),
        )),
        ValueType::Optional(inner) => ValueType::Optional(Box::new(
            substitute_projected_value_generics(inner, bindings),
        )),
        ValueType::List(item) => ValueType::List(ElementType::new(
            substitute_projected_value_generics(item.value_type_ref(), bindings),
        )),
        ValueType::Set(item) => ValueType::Set(ElementType::new(
            substitute_projected_value_generics(item.value_type_ref(), bindings),
        )),
        ValueType::UnorderedSet(item) => ValueType::UnorderedSet(ElementType::new(
            substitute_projected_value_generics(item.value_type_ref(), bindings),
        )),
        ValueType::Iterator(item) => ValueType::Iterator(ElementType::new(
            substitute_projected_value_generics(item.value_type_ref(), bindings),
        )),
        ValueType::Map(key, value) => ValueType::Map(
            ElementType::new(substitute_projected_value_generics(
                key.value_type_ref(),
                bindings,
            )),
            ElementType::new(substitute_projected_value_generics(
                value.value_type_ref(),
                bindings,
            )),
        ),
        ValueType::UnorderedMap(key, value) => ValueType::UnorderedMap(
            ElementType::new(substitute_projected_value_generics(
                key.value_type_ref(),
                bindings,
            )),
            ElementType::new(substitute_projected_value_generics(
                value.value_type_ref(),
                bindings,
            )),
        ),
        ValueType::Tuple(item, length) => ValueType::Tuple(
            ElementType::new(substitute_projected_value_generics(
                item.value_type_ref(),
                bindings,
            )),
            *length,
        ),
        ValueType::Function(parameters, result, effects) => ValueType::Function(
            parameters
                .iter()
                .map(|parameter| substitute_callable_parameter_generics(parameter, bindings))
                .collect(),
            ElementType::new(substitute_projected_value_generics(
                result.value_type_ref(),
                bindings,
            )),
            effects.clone(),
        ),
        ValueType::AsyncFunction(parameters, result, transferability, effects) => {
            ValueType::AsyncFunction(
                parameters
                    .iter()
                    .map(|parameter| substitute_callable_parameter_generics(parameter, bindings))
                    .collect(),
                ElementType::new(substitute_projected_value_generics(
                    result.value_type_ref(),
                    bindings,
                )),
                *transferability,
                effects.clone(),
            )
        }
        _ => value_type.clone(),
    }
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
    if unit.projected_call_specializations.contains_key(&(
        node.span.file,
        node.span.start,
        node.span.end,
    )) {
        return infer_value_type(unit, node, &unit.typed_bindings)
            .ok()
            .flatten();
    }
    let [callee, arguments] = node.children.as_slice() else {
        return None;
    };
    if node.kind != SyntaxKind::CallExpression || callee.kind != SyntaxKind::Name {
        return None;
    }
    let symbol =
        package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
    let declaration = symbol.declaration_span?;
    let contract = contracts.get(&(declaration.file, declaration.start, declaration.end))?;
    if !contract.generic_parameters.is_empty() {
        let selected =
            selected_callable_contract(package, unit, node, crate::syntax::call_is_unsafe(node))?;
        let result = ElementType::new(
            selected
                .return_type
                .unwrap_or(ValueType::Scalar(ScalarType::None)),
        );
        return Some(if selected.is_async {
            ValueType::Task(result, selected.task_transferability)
        } else {
            result.value_type()
        });
    }
    let mut generic_bindings = BTreeMap::new();
    for (argument, parameter) in arguments.children.iter().zip(&contract.parameters) {
        let value = argument.children.last().unwrap_or(argument);
        let Some(expected) = parameter.element_value_type() else {
            continue;
        };
        let Some(actual) = infer_value_type(unit, value, &unit.typed_bindings)
            .ok()
            .flatten()
        else {
            continue;
        };
        if bind_projected_generics(&expected, &actual, &mut generic_bindings).is_err() {
            return None;
        }
    }
    let result = ElementType::new(substitute_projected_value_generics(
        contract
            .return_type
            .as_ref()
            .unwrap_or(&ValueType::Scalar(ScalarType::None)),
        &generic_bindings,
    ));
    Some(if contract.is_async {
        ValueType::Task(result, contract.task_transferability)
    } else {
        result.value_type()
    })
}

pub(super) fn resolve_call_parameter<'a>(
    unit: &SemanticUnit,
    argument: &SyntaxNode,
    name: Option<&SyntaxNode>,
    contract: &'a FunctionContract,
    positional: &mut usize,
    named_seen: &mut bool,
) -> Result<&'a ParameterContract, SemanticFailure> {
    if let Some(name) = name {
        *named_seen = true;
        let name_text = node_text(&unit.source, name);
        let parameter = contract
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
            })?;
        if parameter.variadic {
            return Err(failure(
                &unit.source,
                "T0012",
                format!(
                    "variadic parameter `{}` accepts positional arguments only",
                    parameter.name
                ),
                name.span,
            ));
        }
        return Ok(parameter);
    }
    if *named_seen {
        return Err(failure(
            &unit.source,
            "T0012",
            "positional arguments must precede named arguments",
            argument.span,
        ));
    }
    let parameter = contract.parameters.get(*positional).or_else(|| {
        contract
            .parameters
            .last()
            .filter(|parameter| parameter.variadic)
    });
    let parameter = parameter.ok_or_else(|| {
        failure(
            &unit.source,
            "T0012",
            format!("too many arguments for function `{}`", contract.name),
            argument.span,
        )
    })?;
    *positional += 1;
    Ok(parameter)
}

fn bind_destination_selected_callback(
    expected: &ValueType,
    actual: &ValueType,
    bindings: &mut BTreeMap<String, ValueType>,
) -> Result<bool, String> {
    let (
        ValueType::Function(_, expected_result, _)
        | ValueType::AsyncFunction(_, expected_result, _, _),
        ValueType::Function(_, actual_result, _) | ValueType::AsyncFunction(_, actual_result, _, _),
    ) = (expected, actual)
    else {
        return Ok(false);
    };
    if projected_generic_name(expected_result.value_type_ref()).is_none() {
        return Ok(false);
    }
    bind_projected_generics(
        expected_result.value_type_ref(),
        actual_result.value_type_ref(),
        bindings,
    )?;
    Ok(true)
}

pub(super) fn validate_call_arguments(
    unit: &SemanticUnit,
    arguments: &SyntaxNode,
    contract: &FunctionContract,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let mut bound = BTreeSet::new();
    let mut generic_bindings = BTreeMap::new();
    let mut positional = 0;
    let mut named_seen = false;
    for argument in &arguments.children {
        let name = argument
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
        let parameter = resolve_call_parameter(
            unit,
            argument,
            name,
            contract,
            &mut positional,
            &mut named_seen,
        )?;
        if !parameter.variadic && !bound.insert(parameter.name.as_str()) {
            return Err(failure(
                &unit.source,
                "T0012",
                format!("parameter `{}` is bound more than once", parameter.name),
                argument.span,
            ));
        }
        let value = argument.children.last().unwrap_or(argument);
        if let Some(expected) = parameter.element_value_type() {
            if contextual_collection_constructor_matches(unit, value, &expected, bindings) {
                validate_collection_constructor_value(
                    unit,
                    value,
                    &expected,
                    &parameter.name,
                    bindings,
                )?;
            } else if let Some(actual) = infer_value_type(unit, value, bindings)? {
                if bind_destination_selected_callback(
                    &expected,
                    &actual,
                    &mut generic_bindings,
                )
                .map_err(|generic| {
                    failure(
                        &unit.source,
                        "T0012",
                        format!(
                            "projected generic `{generic}` is inferred as incompatible argument types"
                        ),
                        value.span,
                    )
                })? {
                    continue;
                }
                if let Err(generic) =
                    bind_projected_generics(&expected, &actual, &mut generic_bindings)
                {
                    return Err(failure(
                        &unit.source,
                        "T0012",
                        format!(
                            "projected generic `{generic}` is inferred as incompatible argument types"
                        ),
                        value.span,
                    ));
                }
                let validation_expected = match (&expected, &actual) {
                    (
                        ValueType::Reference(_) | ValueType::SharedReference(_),
                        ValueType::Reference(_) | ValueType::SharedReference(_),
                    ) => expected.clone(),
                    (ValueType::Reference(inner) | ValueType::SharedReference(inner), _) => {
                        inner.value_type()
                    }
                    _ => expected.clone(),
                };
                validate_value_destination(
                    &unit.source,
                    &unit.descriptors,
                    &parameter.name,
                    validation_expected,
                    actual,
                    value,
                    "T0012",
                )?;
            }
        }
    }
    if let Some(missing) = contract.parameters.iter().find(|parameter| {
        !parameter.optional && !parameter.variadic && !bound.contains(parameter.name.as_str())
    }) {
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
        for capture in &function.captures {
            if let Some(binding) = unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == *capture
                    && binding.is_visible_at(unit.source.id(), function.span.start)
            }) && !bindings
                .iter()
                .any(|existing| existing.span == binding.span)
            {
                bindings.push(binding.clone());
            }
        }
        bindings.extend(function.parameters.iter().filter_map(|parameter| {
            parameter
                .binding_value_type()
                .map(|value_type| TypedBinding {
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

pub(crate) fn selected_callable_contract(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    call: &SyntaxNode,
    is_unsafe: bool,
) -> Option<FunctionContract> {
    if let Some(contract) =
        super::enums::selected_enum_constructor_contract(package, unit, call, is_unsafe)
    {
        return Some(contract);
    }
    let mut callee = call.children.first()?;
    while matches!(
        callee.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression
    ) {
        callee = callee.children.first()?;
    }
    let designator = if callee.kind == SyntaxKind::AppliedType {
        callee.children.first()?
    } else {
        callee
    };
    let contract = function_contract_for_call_with_safety(package, unit, designator, is_unsafe)?;
    if contract.generic_parameters.is_empty() {
        if callee.kind == SyntaxKind::AppliedType {
            return None;
        }
        let mut selected = contract.clone();
        if callee.kind == SyntaxKind::ConstructionExpression
            && let Some(ValueType::Object(identity)) =
                infer_value_type(unit, call, &unit.typed_bindings)
                    .ok()
                    .flatten()
            && let Some(descriptor) = unit
                .descriptors
                .iter()
                .find(|item| item.identity.base() == identity.base())
        {
            let substitutions = descriptor
                .generic_parameters
                .iter()
                .zip(&identity.type_arguments)
                .map(|(parameter, actual)| (parameter.name.clone(), actual.clone()))
                .collect();
            for parameter in &mut selected.parameters {
                parameter.value_type = parameter
                    .value_type
                    .as_ref()
                    .map(|value_type| substitute_value_type(value_type, &substitutions));
            }
            selected.return_type = Some(ValueType::Object(identity));
        }
        if callee.kind == SyntaxKind::MemberExpression
            && let Some(
                ValueType::Function(parameters, result, _)
                | ValueType::AsyncFunction(parameters, result, _, _),
            ) = infer_member_value_type(unit, callee, &unit.typed_bindings)
                .ok()
                .flatten()
        {
            for (parameter, callable) in selected.parameters.iter_mut().zip(parameters) {
                parameter.value_type = Some(callable.value_type());
            }
            selected.return_type = Some(result.value_type());
        }
        canonicalize_selected_contract(package, &mut selected);
        return Some(selected);
    }
    let (mut selected, substitutions) = super::generics::select_unit_callable_contract(
        Some(package),
        unit,
        call,
        contract,
        &unit.typed_bindings,
    )?;
    for parameter in &contract.generic_parameters {
        let Some(bound) = &parameter.bound else {
            continue;
        };
        let Some(ValueType::Object(actual)) = substitutions.get(&parameter.name) else {
            return None;
        };
        let implementor = package
            .units
            .iter()
            .flat_map(|owner| &owner.descriptors)
            .find(|descriptor| descriptor.identity.base() == actual.base())?;
        let implements_bound = |interface: &ObjectIdentity| {
            interface.base() == bound.base()
                && interface.type_arguments == bound.type_arguments
                && interface.application == bound.application
                && interface.native_arguments == bound.native_arguments
        };
        if !implements_bound(actual)
            && !effective_object_interfaces(package, implementor)
                .iter()
                .any(|interface| implements_bound(interface))
        {
            return None;
        }
    }
    canonicalize_selected_contract(package, &mut selected);
    Some(selected)
}

fn canonicalize_selected_contract(package: &SemanticPackage, contract: &mut FunctionContract) {
    for value_type in contract
        .parameters
        .iter_mut()
        .filter_map(|parameter| parameter.value_type.as_mut())
        .chain(contract.return_type.iter_mut())
        .chain(contract.thrown_types.iter_mut())
    {
        super::objects::canonicalize_native_value_type(package, value_type);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{bind_projected_generics, substitute_projected_value_generics};
    use crate::ScalarType;
    use crate::semantics::{ElementType, ValueType};

    #[test]
    fn projected_generics_bind_through_maps_and_tuples() {
        let generic = || ElementType::new(ValueType::ProjectedGeneric("T".to_owned()));
        let integer = || ElementType::new(ValueType::Scalar(ScalarType::Int));
        let string = || ElementType::new(ValueType::Scalar(ScalarType::String));
        let mut bindings = BTreeMap::new();

        bind_projected_generics(
            &ValueType::Map(string(), generic()),
            &ValueType::Map(string(), integer()),
            &mut bindings,
        )
        .unwrap();
        bind_projected_generics(
            &ValueType::Tuple(generic(), Some(2)),
            &ValueType::Tuple(integer(), Some(2)),
            &mut bindings,
        )
        .unwrap();

        assert_eq!(
            substitute_projected_value_generics(&ValueType::Map(string(), generic()), &bindings),
            ValueType::Map(string(), integer())
        );
        assert!(
            bind_projected_generics(
                &ValueType::Tuple(generic(), Some(2)),
                &ValueType::List(integer()),
                &mut BTreeMap::new(),
            )
            .is_err()
        );
    }
}
