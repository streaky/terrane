use super::prelude::*;

#[expect(
    clippy::too_many_lines,
    reason = "object analysis assembles one complete declaration contract"
)]
pub(super) fn analyze_object_contracts(
    unit: &SemanticUnit,
    aliases: &BTreeMap<String, Vec<DescriptorAlias>>,
    visible_objects: &BTreeMap<String, ObjectIdentity>,
) -> Result<Vec<ObjectContract>, SemanticFailure> {
    let visible = visible_descriptor_aliases(aliases, unit.source.id(), 0);
    let mut objects = Vec::new();
    for node in &unit.tree.root.children {
        let kind = match node.kind {
            SyntaxKind::ClassDeclaration => ObjectKind::Class,
            SyntaxKind::InterfaceDeclaration => ObjectKind::Interface,
            SyntaxKind::TraitDeclaration => ObjectKind::Trait,
            _ => continue,
        };
        let name = declaration_name(node, &unit.source).ok_or_else(|| {
            failure(
                &unit.source,
                "T0053",
                "object declaration requires a name",
                node.span,
            )
        })?;
        let clause_identities = |clause_kind| {
            node.children
                .iter()
                .find(|child| child.kind == clause_kind)
                .map(|clause| {
                    clause
                        .children
                        .iter()
                        .map(|name| {
                            let name = node_text(&unit.source, name);
                            visible_objects.get(name).cloned().unwrap_or_else(|| {
                                if name == "throwable" {
                                    ObjectIdentity::new("/core/errors", name)
                                } else {
                                    ObjectIdentity::new(&unit.namespace, name)
                                }
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        let base = clause_identities(SyntaxKind::ExtendsClause)
            .into_iter()
            .next();
        let interfaces = clause_identities(SyntaxKind::ImplementsClause);
        let traits = clause_identities(SyntaxKind::UsesClause);
        let mut fields = Vec::new();
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            for field in block
                .children
                .iter()
                .filter(|child| child.kind == SyntaxKind::Binding)
            {
                let Some(field_name) = field
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)
                else {
                    continue;
                };
                let initializer = field.children.last().filter(|child| {
                    child.span != field_name.span
                        && !matches!(
                            child.kind,
                            SyntaxKind::Visibility
                                | SyntaxKind::DeclarationQualifier
                                | SyntaxKind::TypeExpression
                        )
                });
                let value_type = if let Some(type_node) = field
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::TypeExpression)
                {
                    declared_value_type_with_visible_objects(
                        unit,
                        type_node,
                        &visible,
                        visible_objects,
                    )?
                } else if let Some(initializer) = initializer {
                    infer_value_type(unit, initializer, &[])?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0065",
                            "object field type cannot be inferred",
                            field.span,
                        )
                    })?
                } else {
                    return Err(failure(
                        &unit.source,
                        "T0066",
                        "object fields require a type or initializer",
                        field.span,
                    ));
                };
                if kind == ObjectKind::Class
                    && initializer.is_none()
                    && !matches!(
                        value_type,
                        ValueType::PlatformStreamHandle
                            | ValueType::PlatformResourceHandle
                            | ValueType::FilesystemAuthority
                    )
                {
                    return Err(failure(
                        &unit.source,
                        "T0061",
                        format!(
                            "class field `{}` requires an initializer",
                            node_text(&unit.source, field_name)
                        ),
                        field.span,
                    ));
                }
                fields.push(ObjectField {
                    name: node_text(&unit.source, field_name).to_owned(),
                    span: field.span,
                    value_type,
                    is_static: field.children.iter().any(|child| {
                        child.kind == SyntaxKind::DeclarationQualifier
                            && node_text(&unit.source, child) == "static"
                    }),
                });
            }
        }
        let resource_owning = kind == ObjectKind::Class
            && fields.iter().any(|field| {
                matches!(
                    field.value_type,
                    ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle
                )
            });
        objects.push(ObjectContract {
            identity: ObjectIdentity::new(&unit.namespace, &name),
            name,
            span: node.span,
            kind,
            resource_owning,
            base,
            interfaces,
            traits,
            fields,
        });
    }
    for object in &objects {
        let require_kind = |identity: &ObjectIdentity, expected: ObjectKind, role: &str| {
            let local = objects
                .iter()
                .find(|candidate| candidate.identity == *identity);
            let valid = (expected == ObjectKind::Interface
                && identity == &ObjectIdentity::new("/core/errors", "throwable"))
                || local.is_some_and(|candidate| candidate.kind == expected)
                || local.is_none() && visible_objects.values().any(|visible| visible == identity);
            valid.then_some(()).ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0054",
                    format!(
                        "`{}` does not resolve to a {role}",
                        diagnostic_object_identity(&objects, identity)
                    ),
                    object.span,
                )
            })
        };
        if let Some(base) = &object.base {
            require_kind(base, ObjectKind::Class, "class")?;
        }
        for interface in &object.interfaces {
            require_kind(interface, ObjectKind::Interface, "interface")?;
        }
        for used_trait in &object.traits {
            require_kind(used_trait, ObjectKind::Trait, "trait")?;
        }
    }
    Ok(objects)
}

pub(super) fn value_type_owns_resource(
    value_type: &ValueType,
    resource_identities: &BTreeSet<String>,
) -> bool {
    match value_type {
        ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
        ValueType::Object(identity) => resource_identities.contains(&identity.qualified()),
        ValueType::Optional(inner) => value_type_owns_resource(inner, resource_identities),
        ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::UnorderedSet(item)
        | ValueType::Task(item)
        | ValueType::ScopedTask(item)
        | ValueType::TaskOutcome(item)
        | ValueType::Reference(item)
        | ValueType::SharedReference(item) => {
            value_type_owns_resource(&item.value_type(), resource_identities)
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_owns_resource(&key.value_type(), resource_identities)
                || value_type_owns_resource(&value.value_type(), resource_identities)
        }
        _ => false,
    }
}

pub(super) fn value_type_is_resource_container(
    value_type: &ValueType,
    resource_identities: &BTreeSet<String>,
) -> bool {
    matches!(
        value_type,
        ValueType::List(_)
            | ValueType::Map(_, _)
            | ValueType::Set(_)
            | ValueType::Tuple(_, _)
            | ValueType::UnorderedMap(_, _)
            | ValueType::UnorderedSet(_)
    ) && value_type_owns_resource(value_type, resource_identities)
}

pub(super) fn propagate_resource_ownership(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    loop {
        let resource_identities = package
            .units
            .iter()
            .flat_map(|unit| {
                unit.objects
                    .iter()
                    .filter(|object| object.resource_owning)
                    .filter_map(|object| {
                        package
                            .resolve_name_at(unit, object.span.start, &object.name)
                            .map(|symbol| symbol.identity.clone())
                    })
            })
            .collect::<BTreeSet<_>>();
        let mut newly_resource_owning = Vec::new();
        for (unit_index, unit) in package.units.iter().enumerate() {
            for (object_index, object) in unit.objects.iter().enumerate() {
                if object.kind != ObjectKind::Class || object.resource_owning {
                    continue;
                }
                let owns_field_resource = object
                    .fields
                    .iter()
                    .any(|field| value_type_owns_resource(&field.value_type, &resource_identities));
                let owns_base_resource = object
                    .base
                    .as_ref()
                    .is_some_and(|base| resource_identities.contains(&base.qualified()));
                if owns_field_resource || owns_base_resource {
                    newly_resource_owning.push((unit_index, object_index));
                }
            }
        }
        if newly_resource_owning.is_empty() {
            break;
        }
        for (unit_index, object_index) in newly_resource_owning {
            package.units[unit_index].objects[object_index].resource_owning = true;
        }
    }

    let resource_identities = package
        .units
        .iter()
        .flat_map(|unit| {
            unit.objects
                .iter()
                .filter(|object| object.resource_owning)
                .filter_map(|object| {
                    package
                        .resolve_name_at(unit, object.span.start, &object.name)
                        .map(|symbol| symbol.identity.clone())
                })
        })
        .collect::<BTreeSet<_>>();

    for unit in &package.units {
        for object in &unit.objects {
            if object.resource_owning
                && (object.base.is_some()
                    || !object.interfaces.is_empty()
                    || !object.traits.is_empty())
            {
                return Err(failure(
                    &unit.source,
                    "T0098",
                    "a resource-owning class cannot extend, implement, or use copyable object contracts",
                    object.span,
                ));
            }
            if let Some(field) = object.fields.iter().find(|field| {
                value_type_is_resource_container(&field.value_type, &resource_identities)
            }) {
                return Err(failure(
                    &unit.source,
                    "T0101",
                    "resource-owning values in collections are not supported yet",
                    field.span,
                ));
            }
        }
    }
    Ok(())
}
pub(super) fn validate_resource_collection_types(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    let resource_identities = package
        .units
        .iter()
        .flat_map(|unit| {
            unit.objects
                .iter()
                .filter(|object| object.resource_owning)
                .filter_map(|object| {
                    package
                        .resolve_name_at(unit, object.span.start, &object.name)
                        .map(|symbol| symbol.identity.clone())
                })
        })
        .collect::<BTreeSet<_>>();
    for unit in &package.units {
        if let Some(binding) = unit.typed_bindings.iter().find(|binding| {
            value_type_is_resource_container(&binding.value_type, &resource_identities)
        }) {
            return Err(failure(
                &unit.source,
                "T0101",
                "resource-owning values in collections are not supported yet",
                binding.span,
            ));
        }
        for function in &unit.functions {
            if let Some(parameter) = function.parameters.iter().find(|parameter| {
                parameter.value_type.as_ref().is_some_and(|value_type| {
                    value_type_is_resource_container(value_type, &resource_identities)
                })
            }) {
                return Err(failure(
                    &unit.source,
                    "T0101",
                    "resource-owning values in collections are not supported yet",
                    parameter.span,
                ));
            }
            if function.return_type.as_ref().is_some_and(|value_type| {
                value_type_is_resource_container(value_type, &resource_identities)
            }) {
                return Err(failure(
                    &unit.source,
                    "T0101",
                    "resource-owning values in collections are not supported yet",
                    function.span,
                ));
            }
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "object conformance checks inheritance, interfaces, and trait conflicts together"
)]
pub(super) fn validate_object_conformance(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn same_signature(left: &FunctionContract, right: &FunctionContract) -> bool {
        left.parameters.len() == right.parameters.len()
            && left
                .parameters
                .iter()
                .zip(&right.parameters)
                .all(|(left, right)| {
                    left.value_type == right.value_type
                        && left.optional == right.optional
                        && left.mutable == right.mutable
                })
            && left.return_type == right.return_type
            && (!right.throws || left.throws)
            && left.is_async == right.is_async
            && left.consumes_receiver == right.consumes_receiver
    }

    fn effective_method<'a>(
        unit: &'a SemanticUnit,
        object: &'a ObjectContract,
        name: &str,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| method.owner.as_deref() == Some(&object.name) && method.name == name)
            .or_else(|| {
                object
                    .base
                    .as_ref()
                    .and_then(|base| {
                        unit.objects
                            .iter()
                            .find(|candidate| candidate.identity == *base)
                    })
                    .and_then(|base| effective_method(unit, base, name))
            })
    }

    for unit in &package.units {
        for object in unit
            .objects
            .iter()
            .filter(|object| object.kind == ObjectKind::Class)
        {
            let declaration_unit = package
                .units
                .iter()
                .find(|candidate| candidate.source.id() == object.span.file)
                .expect("object declaration source must belong to the semantic package");
            let object = declaration_unit
                .objects
                .iter()
                .find(|candidate| candidate.identity == object.identity)
                .expect("object identity must resolve in its declaration unit");
            for interface_identity in &object.interfaces {
                let Some(resolved_interface) =
                    package.resolve_name(&interface_identity.namespace, &interface_identity.name)
                else {
                    return Err(failure(
                        &declaration_unit.source,
                        "T0001",
                        format!(
                            "interface `{}` implemented by `{}` does not resolve",
                            diagnostic_object_identity(
                                &declaration_unit.objects,
                                interface_identity
                            ),
                            object.name
                        ),
                        object.span,
                    ));
                };
                if resolved_interface.identity == "/core/errors::throwable" {
                    let has_message = object.fields.iter().any(|field| {
                        field.name == "message"
                            && field.value_type == ValueType::Scalar(ScalarType::String)
                    });
                    if !has_message {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0062",
                            format!(
                                "class `{}` must provide a `message string` field to implement `throwable`",
                                object.name
                            ),
                            object.span,
                        ));
                    }
                    let Some(render) = effective_method(declaration_unit, object, "render") else {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0062",
                            format!(
                                "class `{}` does not implement interface member `throwable.render`",
                                object.name
                            ),
                            object.span,
                        ));
                    };
                    let required_render = FunctionContract {
                        name: "render".to_owned(),
                        span: object.span,
                        owner: Some("/core/errors::throwable".to_owned()),
                        captures: Vec::new(),
                        parameters: Vec::new(),
                        is_static: false,
                        return_type: Some(ValueType::Scalar(ScalarType::String)),
                        exported: true,
                        thrown_types: Vec::new(),
                        escaping_throwables: BTreeSet::new(),
                        throws: false,
                        is_async: false,
                        mutates_receiver: false,
                        consumes_receiver: false,
                    };
                    if !same_signature(&required_render, render) {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0067",
                            format!(
                                "class `{}` implements `throwable.render` with an incompatible signature",
                                object.name
                            ),
                            render.span,
                        ));
                    }
                    continue;
                }
                let interface_unit = package
                    .units
                    .iter()
                    .find(|candidate| {
                        candidate.namespace == resolved_interface.namespace
                            && candidate.objects.iter().any(|candidate| {
                                candidate.name == resolved_interface.name
                                    && candidate.kind == ObjectKind::Interface
                            })
                    })
                    .expect("resolved interface must have a semantic declaration");
                let interface = interface_unit
                    .objects
                    .iter()
                    .find(|candidate| candidate.name == resolved_interface.name)
                    .expect("resolved interface must have an object contract");
                for required in interface_unit
                    .functions
                    .iter()
                    .filter(|method| method.owner.as_deref() == Some(&interface.name))
                {
                    let Some(actual) = effective_method(declaration_unit, object, &required.name)
                    else {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0062",
                            format!(
                                "class `{}` does not implement interface member `{}.{}`",
                                object.name, interface.name, required.name
                            ),
                            object.span,
                        ));
                    };
                    if !same_signature(required, actual) {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0067",
                            format!(
                                "class `{}` implements `{}.{}` with an incompatible signature",
                                object.name, interface.name, required.name
                            ),
                            actual.span,
                        ));
                    }
                }
            }

            let own_methods = declaration_unit
                .functions
                .iter()
                .filter(|method| method.owner.as_deref() == Some(&object.name))
                .map(|method| method.name.as_str())
                .collect::<BTreeSet<_>>();
            let own_fields = object
                .fields
                .iter()
                .map(|field| field.name.as_str())
                .collect::<BTreeSet<_>>();
            let mut providers = BTreeMap::<&str, Vec<&str>>::new();
            for trait_name in &object.traits {
                let used_trait = declaration_unit
                    .objects
                    .iter()
                    .find(|candidate| candidate.identity == *trait_name)
                    .expect("object-kind validation must resolve used traits");
                for method in declaration_unit
                    .functions
                    .iter()
                    .filter(|method| method.owner.as_deref() == Some(&used_trait.name))
                {
                    providers
                        .entry(method.name.as_str())
                        .or_default()
                        .push(used_trait.name.as_str());
                }
                for field in &used_trait.fields {
                    providers
                        .entry(field.name.as_str())
                        .or_default()
                        .push(used_trait.name.as_str());
                }
            }
            if let Some((member, traits)) = providers.iter().find(|(member, traits)| {
                traits.len() > 1
                    && !own_methods.contains(**member)
                    && !own_fields.contains(**member)
            }) {
                return Err(failure(
                    &declaration_unit.source,
                    "T0063",
                    format!(
                        "class `{}` inherits conflicting member `{member}` from traits {}",
                        object.name,
                        traits.join(", ")
                    ),
                    object.span,
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn propagate_interface_receiver_mutability(package: &mut SemanticPackage) {
    fn effective_method<'a>(
        unit: &'a SemanticUnit,
        object: &'a ObjectContract,
        name: &str,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| method.owner.as_deref() == Some(&object.name) && method.name == name)
            .or_else(|| {
                object
                    .base
                    .as_ref()
                    .and_then(|base| {
                        unit.objects
                            .iter()
                            .find(|candidate| candidate.identity == *base)
                    })
                    .and_then(|base| effective_method(unit, base, name))
            })
            .or_else(|| {
                object.traits.iter().find_map(|used_trait| {
                    unit.objects
                        .iter()
                        .find(|candidate| candidate.identity == *used_trait)
                        .and_then(|used_trait| effective_method(unit, used_trait, name))
                })
            })
    }

    let mut mutating = BTreeSet::<(u32, usize, usize, String)>::new();
    for unit in &package.units {
        for class in unit
            .objects
            .iter()
            .filter(|object| object.kind == ObjectKind::Class)
        {
            for interface_name in &class.interfaces {
                let Some(interface) = unit
                    .objects
                    .iter()
                    .find(|candidate| candidate.identity == *interface_name)
                else {
                    continue;
                };
                for required in unit
                    .functions
                    .iter()
                    .filter(|method| method.owner.as_deref() == Some(&interface.name))
                {
                    if effective_method(unit, class, &required.name)
                        .is_some_and(|actual| actual.mutates_receiver)
                    {
                        mutating.insert((
                            interface.span.file,
                            interface.span.start,
                            interface.span.end,
                            required.name.clone(),
                        ));
                    }
                }
            }
        }
    }

    for unit in &mut package.units {
        for method in &mut unit.functions {
            let Some(owner) = method.owner.as_deref() else {
                continue;
            };
            let Some(interface) = unit
                .objects
                .iter()
                .find(|object| object.kind == ObjectKind::Interface && object.name == owner)
            else {
                continue;
            };
            if mutating.contains(&(
                interface.span.file,
                interface.span.start,
                interface.span.end,
                method.name.clone(),
            )) {
                method.mutates_receiver = true;
            }
        }
    }
}
#[expect(
    clippy::too_many_lines,
    reason = "receiver-consumption inference keeps its source-ownership helpers scoped to one fixed-point pass"
)]
pub(super) fn infer_receiver_consumption(package: &mut SemanticPackage) {
    fn owns_resource(package: &SemanticPackage, value_type: &ValueType) -> bool {
        match value_type {
            ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle => true,
            ValueType::Object(name) => resolved_object_span(package, name)
                .and_then(|span| {
                    package
                        .units
                        .iter()
                        .flat_map(|candidate| &candidate.objects)
                        .find(|object| object.span == span)
                })
                .is_some_and(|object| object.resource_owning),
            _ => false,
        }
    }

    fn effective_method<'a>(
        unit: &'a SemanticUnit,
        object_name: &str,
        method_name: &str,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| {
                method.owner.as_deref() == Some(object_name) && method.name == method_name
            })
            .or_else(|| {
                unit.objects
                    .iter()
                    .find(|object| object.name == object_name)
                    .and_then(|object| object.base.as_ref())
                    .and_then(|base| unit.objects.iter().find(|object| object.identity == *base))
                    .and_then(|base| effective_method(unit, &base.name, method_name))
            })
    }

    fn receiver_resource_expression(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        contract: &FunctionContract,
        expression: &SyntaxNode,
    ) -> bool {
        if expression.kind == SyntaxKind::Name && node_text(&unit.source, expression) == "this" {
            return contract.owner.as_deref().is_some_and(|owner| {
                unit.objects
                    .iter()
                    .find(|object| object.name == owner)
                    .is_some_and(|object| object.resource_owning)
            });
        }
        let [receiver, member] = expression.children.as_slice() else {
            return expression
                .children
                .iter()
                .any(|child| receiver_resource_expression(package, unit, contract, child));
        };
        if expression.kind != SyntaxKind::MemberExpression
            || receiver.kind != SyntaxKind::Name
            || node_text(&unit.source, receiver) != "this"
        {
            return false;
        }
        contract.owner.as_deref().is_some_and(|owner| {
            unit.objects
                .iter()
                .find(|object| object.name == owner)
                .and_then(|object| {
                    object
                        .fields
                        .iter()
                        .find(|field| field.name == node_text(&unit.source, member))
                })
                .is_some_and(|field| owns_resource(package, &field.value_type))
        })
    }

    fn callable_parameters<'a>(
        package: &'a SemanticPackage,
        unit: &'a SemanticUnit,
        call: &SyntaxNode,
    ) -> Option<&'a [ParameterContract]> {
        let callee = call.children.first()?;
        if callee.kind != SyntaxKind::Name {
            return None;
        }
        let symbol =
            package.resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))?;
        let declaration = symbol.declaration_span?;
        if symbol.kind == SymbolKind::Class {
            let object = package
                .units
                .iter()
                .flat_map(|candidate| &candidate.objects)
                .find(|object| object.span == declaration)?;
            return package
                .units
                .iter()
                .flat_map(|candidate| &candidate.functions)
                .find(|function| {
                    function.owner.as_deref() == Some(&object.name) && function.name == "construct"
                })
                .map(|function| function.parameters.as_slice());
        }
        package
            .units
            .iter()
            .flat_map(|candidate| &candidate.functions)
            .find(|function| function.span == declaration)
            .map(|function| function.parameters.as_slice())
    }

    fn node_consumes_receiver(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        contract: &FunctionContract,
        node: &SyntaxNode,
    ) -> bool {
        if node.kind == SyntaxKind::CallExpression {
            let Some(callee) = node.children.first() else {
                return false;
            };
            let arguments = node
                .children
                .get(1)
                .map_or(&[][..], |arguments| arguments.children.as_slice());
            if callee.kind == SyntaxKind::Name {
                let identity = package
                    .resolve_name_at(unit, callee.span.start, node_text(&unit.source, callee))
                    .map(Symbol::compiler_identity);
                if matches!(
                    identity,
                    Some("intrinsic:streams::close" | "intrinsic:streams::release")
                ) && arguments
                    .first()
                    .and_then(|argument| argument.children.last())
                    .is_some_and(|argument| {
                        receiver_resource_expression(package, unit, contract, argument)
                    })
                {
                    return true;
                }
                if let Some(parameters) = callable_parameters(package, unit, node)
                    && arguments
                        .iter()
                        .zip(parameters)
                        .any(|(argument, parameter)| {
                            argument.children.last().is_some_and(|argument| {
                                parameter
                                    .value_type
                                    .as_ref()
                                    .is_some_and(|value_type| owns_resource(package, value_type))
                                    && receiver_resource_expression(
                                        package, unit, contract, argument,
                                    )
                            })
                        })
                {
                    return true;
                }
            } else if callee.kind == SyntaxKind::MemberExpression
                && let [receiver, member] = callee.children.as_slice()
                && matches!(
                    infer_value_type(unit, receiver, &unit.typed_bindings),
                    Ok(Some(ValueType::Object(object_name)))
                        if effective_method(
                            unit,
                            &object_name.name,
                            node_text(&unit.source, member)
                        )
                        .is_some_and(|method| method.consumes_receiver)
                )
                && receiver_resource_expression(package, unit, contract, receiver)
            {
                return true;
            }
        }
        node.children
            .iter()
            .any(|child| node_consumes_receiver(package, unit, contract, child))
    }

    loop {
        let mut newly_consuming = BTreeSet::new();
        for unit in &package.units {
            for contract in &unit.functions {
                if !contract.consumes_receiver
                    && contract.owner.is_some()
                    && contract.name != "destruct"
                    && find_node_by_span(&unit.tree.root, contract.span)
                        .is_some_and(|node| node_consumes_receiver(package, unit, contract, node))
                {
                    newly_consuming.insert((
                        contract.span.file,
                        contract.span.start,
                        contract.span.end,
                    ));
                }
            }
        }
        if newly_consuming.is_empty() {
            break;
        }
        for unit in &mut package.units {
            for contract in &mut unit.functions {
                if newly_consuming.contains(&(
                    contract.span.file,
                    contract.span.start,
                    contract.span.end,
                )) {
                    contract.consumes_receiver = true;
                }
            }
        }
    }

    let mut consuming_interfaces = BTreeSet::<((u32, usize, usize), String)>::new();
    for unit in &package.units {
        for class in unit
            .objects
            .iter()
            .filter(|object| object.kind == ObjectKind::Class)
        {
            for interface_name in &class.interfaces {
                let Some(interface) = unit.objects.iter().find(|object| {
                    object.kind == ObjectKind::Interface && object.identity == *interface_name
                }) else {
                    continue;
                };
                for required in unit
                    .functions
                    .iter()
                    .filter(|method| method.owner.as_deref() == Some(&interface.name))
                {
                    if effective_method(unit, &class.name, &required.name)
                        .is_some_and(|actual| actual.consumes_receiver)
                    {
                        consuming_interfaces.insert((
                            (
                                interface.span.file,
                                interface.span.start,
                                interface.span.end,
                            ),
                            required.name.clone(),
                        ));
                    }
                }
            }
        }
    }
    for unit in &mut package.units {
        for method in &mut unit.functions {
            if method.owner.as_deref().is_some_and(|owner| {
                unit.objects
                    .iter()
                    .find(|object| object.kind == ObjectKind::Interface && object.name == owner)
                    .is_some_and(|interface| {
                        consuming_interfaces.contains(&(
                            (
                                interface.span.file,
                                interface.span.start,
                                interface.span.end,
                            ),
                            method.name.clone(),
                        ))
                    })
            }) {
                method.consumes_receiver = true;
            }
        }
    }
}

pub(super) fn analyze_types(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    for index in 0..package.units.len() {
        let objects = {
            let unit = &package.units[index];
            let alias_history = descriptor_construct_alias_history(package, unit);
            let visible_objects = package
                .namespaces
                .get(&unit.namespace)
                .into_iter()
                .flat_map(|namespace| &namespace.symbols)
                .filter(|(_, symbol)| {
                    matches!(
                        symbol.kind,
                        SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                    )
                })
                .map(|(visible_name, symbol)| {
                    (
                        visible_name.clone(),
                        ObjectIdentity::new(&symbol.namespace, &symbol.name),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            analyze_object_contracts(unit, &alias_history, &visible_objects)?
        };
        package.units[index].objects = objects;
    }
    populate_object_aliases(package);
    propagate_resource_ownership(package)?;
    for index in 0..package.units.len() {
        let unit = &package.units[index];
        let mut alias_history = descriptor_construct_alias_history(package, unit);
        let mut functions = Vec::new();
        collect_type_declarations(
            unit,
            &unit.tree.root,
            &mut alias_history,
            &mut functions,
            None,
        )?;
        package.units[index].descriptor_aliases = alias_history;
        package.units[index].functions = functions;
    }
    populate_namespace_function_contracts(package);
    populate_function_aliases(package);
    populate_function_type_dependencies(package);
    propagate_interface_receiver_mutability(package);
    validate_descriptor_value_uses(package)?;

    for index in 0..package.units.len() {
        let unit = &package.units[index];
        let mut visible_bindings = Vec::new();
        let mut bindings = Vec::new();
        collect_typed_bindings(
            unit,
            &unit.tree.root,
            &mut visible_bindings,
            &mut bindings,
            None,
        )?;
        package.units[index].typed_bindings = bindings;
    }
    validate_resource_collection_types(package)?;
    infer_receiver_consumption(package);
    validate_object_conformance(package)?;
    populate_closure_captures(package);
    Ok(())
}
pub(super) fn populate_closure_captures(package: &mut SemanticPackage) {
    fn collect(
        unit: &SemanticUnit,
        closure: Span,
        node: &SyntaxNode,
        captures: &mut BTreeSet<String>,
        declaration_name: bool,
    ) {
        if node.kind == SyntaxKind::Name && !declaration_name {
            let name = node_text(&unit.source, node);
            if unit
                .typed_bindings
                .iter()
                .rev()
                .find(|binding| {
                    binding.name == name && binding.is_visible_at(unit.source.id(), node.span.start)
                })
                .is_some_and(|binding| {
                    !(closure.start <= binding.span.start && binding.span.end <= closure.end)
                })
            {
                captures.insert(name.to_owned());
            }
            return;
        }
        match node.kind {
            SyntaxKind::Binding
            | SyntaxKind::Assignment
            | SyntaxKind::FunctionDeclaration
            | SyntaxKind::AnonymousFunction => {
                let mut skipped_name = false;
                for child in &node.children {
                    if !skipped_name && child.kind == SyntaxKind::Name {
                        skipped_name = true;
                        continue;
                    }
                    collect(unit, closure, child, captures, false);
                }
            }
            SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression => {
                if let Some(receiver) = node.children.first() {
                    collect(unit, closure, receiver, captures, false);
                }
            }
            SyntaxKind::ConstructionExpression => {}
            SyntaxKind::Argument if node.children.len() > 1 => {
                for child in node.children.iter().skip(1) {
                    collect(unit, closure, child, captures, false);
                }
            }
            _ => {
                for child in &node.children {
                    collect(unit, closure, child, captures, false);
                }
            }
        }
    }
    fn closure_node(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
        if node.kind == SyntaxKind::AnonymousFunction && node.span == span {
            return Some(node);
        }
        node.children
            .iter()
            .find_map(|child| closure_node(child, span))
    }

    for unit in &mut package.units {
        let captures = unit
            .functions
            .iter()
            .filter(|contract| contract.name.starts_with("closure@"))
            .map(|contract| {
                let mut captures = BTreeSet::new();
                if let Some(node) = closure_node(&unit.tree.root, contract.span) {
                    collect(unit, contract.span, node, &mut captures, false);
                }
                (contract.span, captures.into_iter().collect::<Vec<_>>())
            })
            .collect::<Vec<_>>();
        for contract in &mut unit.functions {
            if let Some((_, captures)) = captures.iter().find(|(span, _)| *span == contract.span) {
                contract.captures.clone_from(captures);
            }
        }
    }
}

pub(super) fn validate_descriptor_value_uses(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        validate_descriptor_value_node(package, unit, &unit.tree.root, false)?;
    }
    Ok(())
}

pub(super) fn validate_descriptor_value_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    descriptor_context: bool,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::TypeMembershipExpression
        && let Some(descriptor) = node.children.get(1)
        && descriptor_expression_type(package, unit, descriptor).is_none()
        && descriptor_expression_category(package, unit, descriptor).is_none()
    {
        return Err(failure(
            &unit.source,
            "T0001",
            format!(
                "`{}` does not resolve to a type descriptor",
                node_text(&unit.source, descriptor).trim()
            ),
            descriptor.span,
        ));
    }
    if !descriptor_context
        && node.kind == SyntaxKind::MemberExpression
        && node.children.first().is_some_and(|receiver| {
            descriptor_expression_type(package, unit, receiver).is_some()
                || descriptor_expression_category(package, unit, receiver).is_some()
        })
        && package.reflection == crate::package::ReflectionProfile::Minimal
    {
        return Err(failure(
            &unit.source,
            "T0070",
            "the selected minimal profile does not retain reflection metadata",
            node.span,
        ));
    }

    for (index, child) in node.children.iter().enumerate() {
        let child_is_descriptor_context = descriptor_context
            || node.kind == SyntaxKind::TypeExpression
            || node.kind == SyntaxKind::ImportDeclaration
            || (node.kind == SyntaxKind::TypeMembershipExpression && index == 1)
            || (node.kind == SyntaxKind::MemberExpression && index == 1)
            || (matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) && index == 0)
            || (node.kind == SyntaxKind::BinaryExpression
                && node.children.len() == 2
                && node_text(&unit.source, node)[node.children[0].span.end - node.span.start
                    ..node.children[1].span.start - node.span.start]
                    .trim()
                    == "is")
            || (node.kind == SyntaxKind::BinaryExpression
                && node.children.len() == 2
                && matches!(
                    node_text(&unit.source, node)[node.children[0].span.end - node.span.start
                        ..node.children[1].span.start - node.span.start]
                        .trim(),
                    "==" | "!="
                )
                && node_text(&unit.source, child).trim() == "none")
            || (node.kind == SyntaxKind::CallExpression
                && index == 1
                && node.children.first().is_some_and(|callee| {
                    coercion_family_receiver(unit, callee)
                        || obsolete_integer_coercion_member(unit, callee).is_some()
                }));
        validate_descriptor_value_node(package, unit, child, child_is_descriptor_context)?;
    }
    Ok(())
}
