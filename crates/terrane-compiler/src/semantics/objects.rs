use super::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct EffectiveObjectField<'a> {
    pub(crate) unit: &'a SemanticUnit,
    pub(crate) owner: &'a DescriptorContract,
    pub(crate) field: &'a ObjectField,
}

impl std::ops::Deref for EffectiveObjectField<'_> {
    type Target = ObjectField;

    fn deref(&self) -> &Self::Target {
        self.field
    }
}

fn descriptor_declaration<'a>(
    package: &'a SemanticPackage,
    identity: &ObjectIdentity,
) -> Option<(&'a SemanticUnit, &'a DescriptorContract)> {
    package.units.iter().find_map(|unit| {
        unit.descriptors
            .iter()
            .find(|candidate| {
                candidate.identity == *identity && candidate.span.file == unit.source.id()
            })
            .map(|object| (unit, object))
    })
}

pub(crate) fn effective_object_fields<'a>(
    package: &'a SemanticPackage,
    object: &'a DescriptorContract,
) -> Vec<EffectiveObjectField<'a>> {
    fn collect<'a>(
        package: &'a SemanticPackage,
        unit: &'a SemanticUnit,
        object: &'a DescriptorContract,
        fields: &mut Vec<EffectiveObjectField<'a>>,
    ) {
        if let Some((base_unit, base)) = object
            .base
            .as_ref()
            .and_then(|identity| descriptor_declaration(package, identity))
        {
            collect(package, base_unit, base, fields);
        }
        for reused in &object.traits {
            if let Some((trait_unit, reused)) = descriptor_declaration(package, reused) {
                collect(package, trait_unit, reused, fields);
            }
        }
        for field in &object.fields {
            let effective = EffectiveObjectField {
                unit,
                owner: object,
                field,
            };
            if let Some(index) = fields.iter().position(|existing| {
                existing.field.name == field.name && existing.field.is_static == field.is_static
            }) {
                fields[index] = effective;
            } else {
                fields.push(effective);
            }
        }
    }

    let unit = package
        .units
        .iter()
        .find(|unit| unit.source.id() == object.span.file)
        .expect("object declaration source must belong to the semantic package");
    let mut fields = Vec::new();
    collect(package, unit, object, &mut fields);
    fields
}

fn field_metadata(
    unit: &SemanticUnit,
    field: &SyntaxNode,
    field_name: &str,
    value_type: &ValueType,
    defaulted: bool,
    is_static: bool,
) -> Result<ObjectFieldMetadata, SemanticFailure> {
    let Some(metadata) = field
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::FieldMetadata)
    else {
        return Ok(ObjectFieldMetadata {
            external_name: field_name.to_owned(),
            defaulted,
            optional: matches!(value_type, ValueType::Optional(_)),
            secret: false,
        });
    };
    if is_static {
        return Err(failure(
            &unit.source,
            "T0113",
            "field metadata is only valid on instance fields",
            metadata.span,
        ));
    }
    let mut external_name = field_name.to_owned();
    let mut secret = false;
    let mut seen = BTreeSet::new();
    for entry in &metadata.children {
        let text = node_text(&unit.source, entry);
        let Some((name, value)) = text.split_once('=') else {
            continue;
        };
        let name = name.trim();
        let value = value.trim();
        if !seen.insert(name) {
            return Err(failure(
                &unit.source,
                "T0113",
                format!("field metadata `{name}` is declared more than once"),
                entry.span,
            ));
        }
        match name {
            "external-name" => {
                if value.len() < 2
                    || !matches!(
                        (value.as_bytes().first(), value.as_bytes().last()),
                        (Some(b'\''), Some(b'\'')) | (Some(b'"'), Some(b'"'))
                    )
                {
                    return Err(failure(
                        &unit.source,
                        "T0113",
                        "`external-name` metadata requires a string",
                        entry.span,
                    ));
                }
                let decoded = crate::lexer::unescape_bytes(&value[1..value.len() - 1])
                    .expect("the lexer validated string escapes");
                external_name =
                    String::from_utf8(decoded).expect("Terrane source strings remain UTF-8");
            }
            "secret" => match value {
                "true" => secret = true,
                "false" => secret = false,
                _ => {
                    return Err(failure(
                        &unit.source,
                        "T0113",
                        "`secret` metadata requires a boolean",
                        entry.span,
                    ));
                }
            },
            _ => {
                return Err(failure(
                    &unit.source,
                    "T0113",
                    format!("unknown field metadata `{name}`"),
                    entry.span,
                ));
            }
        }
    }
    Ok(ObjectFieldMetadata {
        external_name,
        defaulted,
        optional: matches!(value_type, ValueType::Optional(_)),
        secret,
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "descriptor analysis assembles one complete source declaration contract"
)]
pub(super) fn analyze_descriptor_contracts(
    unit: &SemanticUnit,
    aliases: &BTreeMap<String, Vec<DescriptorAlias>>,
    visible_objects: &BTreeMap<String, ObjectIdentity>,
) -> Result<Vec<DescriptorContract>, SemanticFailure> {
    let visible = visible_descriptor_aliases(aliases, unit.source.id(), 0);
    let mut descriptors = Vec::new();
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
        let clause_identities = |clause_kind| -> Result<Vec<ObjectIdentity>, SemanticFailure> {
            let Some(clause) = node.children.iter().find(|child| child.kind == clause_kind) else {
                return Ok(Vec::new());
            };
            clause
                    .children
                    .iter()
                    .map(|type_node| {
                        if type_node.kind == SyntaxKind::AppliedType {
                            let [base, argument] = type_node.children.as_slice() else {
                                return Err(failure(
                                    &unit.source,
                                    "T0127",
                                    "a projected interface application requires exactly one closed type",
                                    type_node.span,
                                ));
                            };
                            let base_name = node_text(&unit.source, base);
                            let identity =
                                visible_objects.get(base_name).cloned().unwrap_or_else(|| {
                                    ObjectIdentity::new(&unit.namespace, base_name)
                                });
                            let application = declared_value_type_with_visible_objects(
                                unit,
                                argument,
                                &visible,
                                visible_objects,
                            )?;
                            Ok(identity.with_application(application))
                        } else {
                            let name = node_text(&unit.source, type_node);
                            Ok(visible_objects.get(name).cloned().unwrap_or_else(|| {
                                if name == "throwable" {
                                    ObjectIdentity::new("/core/errors", name)
                                } else {
                                    ObjectIdentity::new(&unit.namespace, name)
                                }
                            }))
                        }
                    })
                    .collect()
        };
        let base = clause_identities(SyntaxKind::ExtendsClause)?
            .into_iter()
            .next();
        let interfaces = clause_identities(SyntaxKind::ImplementsClause)?;
        let traits = clause_identities(SyntaxKind::UsesClause)?;
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
                let initializer = field.children.iter().rev().find(|child| {
                    child.span != field_name.span
                        && !matches!(
                            child.kind,
                            SyntaxKind::Visibility
                                | SyntaxKind::DeclarationQualifier
                                | SyntaxKind::TypeExpression
                                | SyntaxKind::FieldMetadata
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
                let uses_canonical_default = matches!(kind, ObjectKind::Class | ObjectKind::Trait)
                    && initializer.is_none()
                    && canonical_default(&value_type).is_some();
                let field_name = node_text(&unit.source, field_name).to_owned();
                let is_static = field.children.iter().any(|child| {
                    child.kind == SyntaxKind::DeclarationQualifier
                        && node_text(&unit.source, child) == "static"
                });
                let metadata = field_metadata(
                    unit,
                    field,
                    &field_name,
                    &value_type,
                    initializer.is_some() || uses_canonical_default,
                    is_static,
                )?;
                fields.push(ObjectField {
                    name: field_name,
                    span: field.span,
                    value_type,
                    initializer_span: initializer.map(|initializer| initializer.span),
                    is_static,
                    metadata,
                });
            }
        }
        let mut external_names = BTreeSet::new();
        for field in fields.iter().filter(|field| !field.is_static) {
            if !external_names.insert(field.metadata.external_name.clone()) {
                return Err(failure(
                    &unit.source,
                    "T0113",
                    format!(
                        "external field name `{}` is used by more than one field",
                        field.metadata.external_name
                    ),
                    field.span,
                ));
            }
        }
        let resource_owning = kind == ObjectKind::Class
            && fields.iter().any(|field| {
                matches!(
                    field.value_type,
                    ValueType::PlatformStreamHandle | ValueType::PlatformResourceHandle
                )
            });
        let methods = unit
            .functions
            .iter()
            .filter(|function| {
                function.owner.as_deref() == Some(name.as_str()) && !function.is_static
            })
            .map(|function| function.name.clone())
            .collect::<BTreeSet<_>>();
        let static_methods = unit
            .functions
            .iter()
            .filter(|function| {
                function.owner.as_deref() == Some(name.as_str()) && function.is_static
            })
            .map(|function| function.name.clone())
            .collect::<BTreeSet<_>>();
        let mut members = fields
            .iter()
            .filter(|field| !field.is_static)
            .map(|field| field.name.clone())
            .collect::<BTreeSet<_>>();
        members.extend(methods.iter().cloned());
        members.insert("type".to_owned());
        let mut static_members = fields
            .iter()
            .filter(|field| field.is_static)
            .map(|field| field.name.clone())
            .collect::<BTreeSet<_>>();
        static_members.extend(static_methods.iter().cloned());
        descriptors.push(DescriptorContract {
            identity: ObjectIdentity::new(&unit.namespace, &name),
            name,
            span: node.span,
            kind,
            resource_owning,
            builtin: None,
            categories: vec![TypeCategory::Value, TypeCategory::Object],
            operations: BTreeMap::new(),
            members,
            methods,
            invocation_only_methods: BTreeSet::new(),
            static_members,
            static_methods,
            base,
            interfaces,
            traits,
            fields,
        });
    }
    for object in &descriptors {
        let require_kind = |identity: &ObjectIdentity, expected: ObjectKind, role: &str| {
            let base_identity = identity.base();
            let local = descriptors
                .iter()
                .find(|candidate| candidate.identity == base_identity);
            let valid = (expected == ObjectKind::Interface
                && identity == &ObjectIdentity::new("/core/errors", "throwable"))
                || local.is_some_and(|candidate| candidate.kind == expected)
                || local.is_none()
                    && visible_objects
                        .values()
                        .any(|visible| visible == &base_identity);
            valid.then_some(()).ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0054",
                    format!(
                        "`{}` does not resolve to a {role}",
                        diagnostic_object_identity(&descriptors, identity)
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
    Ok(descriptors)
}

pub(super) fn value_type_owns_resource(
    value_type: &ValueType,
    resource_identities: &BTreeSet<String>,
) -> bool {
    match value_type {
        ValueType::PlatformStreamHandle
        | ValueType::PlatformResourceHandle
        | ValueType::ChannelPair(_)
        | ValueType::ChannelSender(_)
        | ValueType::ChannelReceiver(_) => true,
        ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects) => {
            effects.modes.written == InvocationMode::Consuming
        }
        ValueType::Object(identity) => resource_identities.contains(&identity.qualified()),
        ValueType::Optional(inner) => value_type_owns_resource(inner, resource_identities),
        ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::UnorderedSet(item)
        | ValueType::Task(item, _)
        | ValueType::ScopedTask(item, _)
        | ValueType::TaskOutcome(item)
        | ValueType::ChannelReceiveOutcome(item)
        | ValueType::ChannelSendOutcome(item)
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
pub(super) fn refresh_source_descriptor_members(units: &mut [SemanticUnit]) {
    for unit in units {
        for descriptor in unit
            .descriptors
            .iter_mut()
            .filter(|descriptor| descriptor.builtin.is_none())
        {
            descriptor.methods = unit
                .functions
                .iter()
                .filter(|function| {
                    function.owner_identity.as_ref() == Some(&descriptor.identity)
                        && !function.is_static
                })
                .map(|function| function.name.clone())
                .collect();
            descriptor.static_methods = unit
                .functions
                .iter()
                .filter(|function| {
                    function.owner_identity.as_ref() == Some(&descriptor.identity)
                        && function.is_static
                })
                .map(|function| function.name.clone())
                .collect();
            descriptor.members = descriptor
                .fields
                .iter()
                .filter(|field| !field.is_static)
                .map(|field| field.name.clone())
                .chain(descriptor.methods.iter().cloned())
                .chain(std::iter::once("type".to_owned()))
                .collect();
            descriptor.operations = descriptor
                .members
                .iter()
                .map(|member| {
                    (
                        member.clone(),
                        format!("source.{}.{}", descriptor.identity.qualified(), member),
                    )
                })
                .collect();
            descriptor.static_members = descriptor
                .fields
                .iter()
                .filter(|field| field.is_static)
                .map(|field| field.name.clone())
                .chain(descriptor.static_methods.iter().cloned())
                .collect();
        }
    }
}

fn value_type_contains_nonclone_foreign(
    projection: &crate::projection::Projection,
    value_type: &ValueType,
) -> bool {
    match value_type {
        ValueType::Object(identity) => projection
            .foreign_is_cloneable(&identity.namespace, &identity.name)
            .is_some_and(|cloneable| !cloneable),
        ValueType::Optional(inner) => value_type_contains_nonclone_foreign(projection, inner),
        ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::UnorderedSet(item)
        | ValueType::Task(item, _)
        | ValueType::ScopedTask(item, _)
        | ValueType::TaskOutcome(item)
        | ValueType::ChannelReceiveOutcome(item)
        | ValueType::ChannelSendOutcome(item)
        | ValueType::Reference(item)
        | ValueType::SharedReference(item) => {
            value_type_contains_nonclone_foreign(projection, &item.value_type())
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_contains_nonclone_foreign(projection, &key.value_type())
                || value_type_contains_nonclone_foreign(projection, &value.value_type())
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

fn resource_identities(package: &SemanticPackage) -> BTreeSet<String> {
    package
        .units
        .iter()
        .flat_map(|unit| {
            unit.descriptors
                .iter()
                .filter(|object| object.resource_owning)
                .filter_map(|object| {
                    package
                        .resolve_name_at(unit, object.span.start, &object.name)
                        .map(|symbol| symbol.identity.clone())
                })
        })
        .collect()
}

pub(super) fn propagate_resource_ownership(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    loop {
        let resource_identities = resource_identities(package);
        let mut newly_resource_owning = Vec::new();
        for (unit_index, unit) in package.units.iter().enumerate() {
            for (object_index, object) in unit.descriptors.iter().enumerate() {
                if object.kind != ObjectKind::Class || object.resource_owning {
                    continue;
                }
                let owns_field_resource = object.fields.iter().any(|field| {
                    value_type_owns_resource(&field.value_type, &resource_identities)
                        || value_type_contains_nonclone_foreign(
                            &package.projection,
                            &field.value_type,
                        )
                });
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
            package.units[unit_index].descriptors[object_index].resource_owning = true;
        }
    }

    let resource_identities = resource_identities(package);

    for unit in &package.units {
        for object in &unit.descriptors {
            let has_copyable_object_contract = object.base.is_some()
                || !object.traits.is_empty()
                || object.interfaces.iter().any(|interface| {
                    !package
                        .projection
                        .item(&interface.namespace, &interface.name)
                        .is_some_and(|item| {
                            matches!(item.kind, crate::projection::ProjectedKind::Interface(_))
                        })
                });
            if object.resource_owning && has_copyable_object_contract {
                return Err(failure(
                    &unit.source,
                    "T0098",
                    "a resource-owning class cannot extend or use a copyable object contract",
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
            unit.descriptors
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

pub(super) fn bind_projected_associated_type(
    value: &ValueType,
    application: &ValueType,
) -> ValueType {
    let element = |value: &ElementType| {
        ElementType::new(bind_projected_associated_type(
            value.value_type_ref(),
            application,
        ))
    };
    match value {
        ValueType::ProjectedAssociated => application.clone(),
        ValueType::Optional(inner) => {
            ValueType::Optional(Box::new(bind_projected_associated_type(inner, application)))
        }
        ValueType::Iterator(item) => ValueType::Iterator(element(item)),
        ValueType::IterationStep(item) => ValueType::IterationStep(element(item)),
        ValueType::AsyncIterationStep(item) => ValueType::AsyncIterationStep(element(item)),
        ValueType::ChannelPair(item) => ValueType::ChannelPair(element(item)),
        ValueType::ChannelSender(item) => ValueType::ChannelSender(element(item)),
        ValueType::ChannelReceiver(item) => ValueType::ChannelReceiver(element(item)),
        ValueType::ChannelSendOutcome(item) => ValueType::ChannelSendOutcome(element(item)),
        ValueType::ChannelReceiveOutcome(item) => ValueType::ChannelReceiveOutcome(element(item)),
        ValueType::DocumentDecodeOutcome(item) => ValueType::DocumentDecodeOutcome(element(item)),
        ValueType::List(item) => ValueType::List(element(item)),
        ValueType::Map(key, item) => ValueType::Map(element(key), element(item)),
        ValueType::Set(item) => ValueType::Set(element(item)),
        ValueType::Tuple(item, length) => ValueType::Tuple(element(item), *length),
        ValueType::Entry(key, item) => ValueType::Entry(element(key), element(item)),
        ValueType::UnorderedMap(key, item) => ValueType::UnorderedMap(element(key), element(item)),
        ValueType::UnorderedSet(item) => ValueType::UnorderedSet(element(item)),
        ValueType::Function(parameters, result, effects) => ValueType::Function(
            parameters.iter().map(element).collect(),
            element(result),
            effects.clone(),
        ),
        ValueType::AsyncFunction(parameters, result, transferability, effects) => {
            ValueType::AsyncFunction(
                parameters.iter().map(element).collect(),
                element(result),
                *transferability,
                effects.clone(),
            )
        }
        ValueType::Task(item, transferability) => ValueType::Task(element(item), *transferability),
        ValueType::ScopedTask(item, transferability) => {
            ValueType::ScopedTask(element(item), *transferability)
        }
        ValueType::TaskOutcome(item) => ValueType::TaskOutcome(element(item)),
        ValueType::Reference(item) => ValueType::Reference(element(item)),
        ValueType::SharedReference(item) => ValueType::SharedReference(element(item)),
        _ => value.clone(),
    }
}

pub(crate) fn bind_projected_requirement(
    requirement: &FunctionContract,
    application: Option<&ValueType>,
) -> FunctionContract {
    let Some(application) = application else {
        return requirement.clone();
    };
    let mut bound = requirement.clone();
    for parameter in &mut bound.parameters {
        if let Some(value_type) = &mut parameter.value_type {
            *value_type = bind_projected_associated_type(value_type, application);
        }
    }
    if let Some(value_type) = &mut bound.return_type {
        *value_type = bind_projected_associated_type(value_type, application);
    }
    for value_type in &mut bound.thrown_types {
        *value_type = bind_projected_associated_type(value_type, application);
    }
    bound
}

#[expect(
    clippy::too_many_lines,
    reason = "object conformance checks inheritance, interfaces, and trait conflicts together"
)]
pub(super) fn validate_object_conformance(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn implementation_satisfies_requirement(
        requirement: &FunctionContract,
        implementation: &FunctionContract,
    ) -> bool {
        requirement.parameters.len() == implementation.parameters.len()
            && requirement
                .parameters
                .iter()
                .zip(&implementation.parameters)
                .all(|(left, right)| left.value_type == right.value_type)
            && requirement.return_type == implementation.return_type
            && (!implementation.throws || requirement.throws)
            && requirement.is_async == implementation.is_async
            && requirement
                .written_invocation_mode
                .accepts(implementation.written_invocation_mode)
    }

    fn effective_method<'a>(
        unit: &'a SemanticUnit,
        object: &'a DescriptorContract,
        name: &str,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| {
                method.owner_identity.as_ref() == Some(&object.identity) && method.name == name
            })
            .or_else(|| {
                object
                    .base
                    .as_ref()
                    .and_then(|base| {
                        unit.descriptors
                            .iter()
                            .find(|candidate| candidate.identity == *base)
                    })
                    .and_then(|base| effective_method(unit, base, name))
            })
    }

    for unit in &package.units {
        for object in unit
            .descriptors
            .iter()
            .filter(|object| object.kind == ObjectKind::Class)
        {
            let declaration_unit = package
                .units
                .iter()
                .find(|candidate| candidate.source.id() == object.span.file)
                .expect("object declaration source must belong to the semantic package");
            let object = declaration_unit
                .descriptors
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
                                &declaration_unit.descriptors,
                                interface_identity
                            ),
                            object.name
                        ),
                        object.span,
                    ));
                };
                if let Some(crate::projection::ProjectedKind::Interface(projected)) = package
                    .projection
                    .item(&resolved_interface.namespace, &resolved_interface.name)
                    .map(|item| &item.kind)
                {
                    match (
                        projected.associated_type.as_ref(),
                        interface_identity.application.as_deref(),
                    ) {
                        (Some(_), None) => {
                            return Err(failure(
                                &declaration_unit.source,
                                "T0127",
                                format!(
                                    "projected interface `{}` requires one closed associated type",
                                    resolved_interface.name
                                ),
                                object.span,
                            ));
                        }
                        (None, Some(_)) => {
                            return Err(failure(
                                &declaration_unit.source,
                                "T0127",
                                format!(
                                    "interface `{}` does not declare a projectable associated type",
                                    resolved_interface.name
                                ),
                                object.span,
                            ));
                        }
                        (Some(associated), Some(application)) => {
                            destination_projected_type(package, application).map_err(|reason| {
                                failure(
                                    &declaration_unit.source,
                                    "T0127",
                                    format!(
                                        "associated type `{application}` for projected interface `{}` is not representable: {reason}",
                                        resolved_interface.name
                                    ),
                                    object.span,
                                )
                            })?;
                            for bound in &associated.bounds {
                                let satisfied = if bound.ends_with("::Send") {
                                    value_type_satisfies_auto_trait(
                                        package,
                                        application,
                                        AutoTraitObligation::Send,
                                    )
                                } else if bound.ends_with("::Sync") {
                                    value_type_satisfies_auto_trait(
                                        package,
                                        application,
                                        AutoTraitObligation::Sync,
                                    )
                                } else if bound.ends_with("::Clone") {
                                    let resources = package
                                        .units
                                        .iter()
                                        .flat_map(|unit| &unit.descriptors)
                                        .filter(|descriptor| descriptor.resource_owning)
                                        .map(|descriptor| descriptor.identity.qualified())
                                        .collect::<BTreeSet<_>>();
                                    !value_type_contains_nonclone_foreign(
                                        &package.projection,
                                        application,
                                    ) && !value_type_owns_resource(application, &resources)
                                } else if bound == "'static" {
                                    true
                                } else {
                                    let required = package.projection.dependencies.iter().find_map(
                                        |dependency| {
                                            dependency.items.iter().find(|item| {
                                                item.rust_path == *bound
                                                    && matches!(
                                                        &item.kind,
                                                        crate::projection::ProjectedKind::Interface(
                                                            interface
                                                        ) if interface.associated_type.is_none()
                                                    )
                                            })
                                        },
                                    );
                                    match (required, application) {
                                        (Some(required), ValueType::Object(actual)) => {
                                            let objects = package
                                                .units
                                                .iter()
                                                .flat_map(|unit| unit.descriptors.iter().cloned())
                                                .collect::<Vec<_>>();
                                            super::types::object_types_compatible(
                                                &objects,
                                                &ObjectIdentity::new(
                                                    &required.namespace,
                                                    &required.name,
                                                ),
                                                actual,
                                            )
                                        }
                                        _ => false,
                                    }
                                };
                                if !satisfied {
                                    return Err(failure(
                                        &declaration_unit.source,
                                        "T0127",
                                        format!(
                                            "associated type `{application}` does not satisfy projected bound `{bound}`"
                                        ),
                                        object.span,
                                    ));
                                }
                            }
                        }
                        (None, None) => {}
                    }
                    for (required, obligation, requirement) in [
                        (projected.send, AutoTraitObligation::Send, "`Send`"),
                        (projected.sync, AutoTraitObligation::Sync, "`Sync`"),
                    ] {
                        if required
                            && let Some(field) = effective_object_fields(package, object)
                                .into_iter()
                                .find(|field| {
                                    !field.is_static
                                        && !value_type_satisfies_auto_trait(
                                            package,
                                            &field.value_type,
                                            obligation,
                                        )
                                })
                        {
                            return Err(failure(
                                &declaration_unit.source,
                                "T0122",
                                format!(
                                    "class `{}` cannot implement `{}` because field `{}` does not satisfy the projected {requirement} obligation",
                                    object.name, resolved_interface.name, field.name
                                ),
                                field.span,
                            ));
                        }
                    }
                }
                if package
                    .projection
                    .item(&resolved_interface.namespace, &resolved_interface.name)
                    .and_then(|item| match &item.kind {
                        crate::projection::ProjectedKind::Interface(projected) => {
                            Some(projected.requires_drop)
                        }
                        _ => None,
                    })
                    == Some(true)
                    && effective_method(declaration_unit, object, "destruct").is_none()
                {
                    return Err(failure(
                        &declaration_unit.source,
                        "T0123",
                        format!(
                            "class `{}` must declare `consuming destruct` to implement projected interface `{}` because it requires canonical Rust `Drop`",
                            object.name, resolved_interface.name
                        ),
                        object.span,
                    ));
                }
                if object.resource_owning
                    && package
                        .projection
                        .item(&resolved_interface.namespace, &resolved_interface.name)
                        .and_then(|item| match &item.kind {
                            crate::projection::ProjectedKind::Interface(projected) => {
                                projected.methods.iter().find(|method| {
                                    method.function.is_async
                                        && method.function.receiver
                                            != Some(crate::projection::Receiver::Move)
                                })
                            }
                            _ => None,
                        })
                        .is_some()
                {
                    return Err(failure(
                        &declaration_unit.source,
                        "T0124",
                        format!(
                            "resource-owning class `{}` cannot implement projected asynchronous borrowed receiver `{}` because cancellation cleanup cannot be separated from the ended Rust borrow",
                            object.name, resolved_interface.name
                        ),
                        object.span,
                    ));
                }
                let projected_async = package
                    .projection
                    .item(&resolved_interface.namespace, &resolved_interface.name)
                    .is_some_and(|item| {
                        matches!(
                            &item.kind,
                            crate::projection::ProjectedKind::Interface(projected)
                                if projected.methods.iter().any(|method| method.function.is_async)
                        )
                    });
                let has_async_entry = package
                    .units
                    .iter()
                    .flat_map(|unit| &unit.functions)
                    .any(|function| function.name == "main" && function.is_async);
                if projected_async && !has_async_entry {
                    return Err(failure(
                        &declaration_unit.source,
                        "T0125",
                        format!(
                            "class `{}` cannot implement projected asynchronous interface `{}` without an asynchronous `main` runtime context",
                            object.name, resolved_interface.name
                        ),
                        object.span,
                    ));
                }
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
                        owner_identity: Some(ObjectIdentity::new("/core/errors", "throwable")),
                        is_anonymous: false,
                        projected_provided: false,
                        captures: Vec::new(),
                        parameters: Vec::new(),
                        is_static: false,
                        return_type: Some(ValueType::Scalar(ScalarType::String)),
                        exported: true,
                        thrown_types: Vec::new(),
                        escaping_throwables: BTreeSet::new(),
                        task_transferability: TaskTransferability::Transferable,
                        execution_requirements: crate::execution::ExecutionRequirements::default(),
                        throws: false,
                        is_async: false,
                        written_invocation_mode: InvocationMode::Shared,
                        exact_invocation_mode: InvocationMode::Shared,
                    };
                    if !implementation_satisfies_requirement(&required_render, render) {
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
                            && candidate.descriptors.iter().any(|candidate| {
                                candidate.name == resolved_interface.name
                                    && candidate.kind == ObjectKind::Interface
                            })
                    })
                    .expect("resolved interface must have a semantic declaration");
                let interface = interface_unit
                    .descriptors
                    .iter()
                    .find(|candidate| candidate.name == resolved_interface.name)
                    .expect("resolved interface must have an object contract");
                let requirements = interface_unit
                    .functions
                    .iter()
                    .filter(|method| method.owner_identity.as_ref() == Some(&interface.identity))
                    .map(|method| {
                        bind_projected_requirement(
                            method,
                            interface_identity.application.as_deref(),
                        )
                    })
                    .collect::<Vec<_>>();
                for required in &requirements {
                    let Some(actual) = effective_method(declaration_unit, object, &required.name)
                    else {
                        if required.projected_provided
                            || package
                                .projection
                                .interface_method(
                                    &interface.identity.namespace,
                                    &interface.identity.name,
                                    &required.name,
                                )
                                .is_some_and(|method| method.provided)
                        {
                            continue;
                        }
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
                    if let Some(reason) = package
                        .projection
                        .interface_method(
                            &interface.identity.namespace,
                            &interface.identity.name,
                            &required.name,
                        )
                        .filter(|method| method.provided)
                        .and_then(|method| {
                            if method
                                .function
                                .parameters
                                .iter()
                                .any(|parameter| parameter.borrowed)
                            {
                                Some("borrowed parameters are deferred")
                            } else if method.function.error.is_some() {
                                Some("Result-returning methods are deferred")
                            } else {
                                None
                            }
                        })
                    {
                        return Err(failure(
                            &declaration_unit.source,
                            "T0067",
                            format!(
                                "provided member `{}.{}` cannot be overridden: {reason}",
                                interface.name, required.name
                            ),
                            actual.span,
                        ));
                    }
                    if !implementation_satisfies_requirement(required, actual) {
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
                .filter(|method| method.owner_identity.as_ref() == Some(&object.identity))
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
                    .descriptors
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

pub(super) fn validate_class_field_initializers(
    package: &SemanticPackage,
) -> Result<(), SemanticFailure> {
    for object in package
        .units
        .iter()
        .flat_map(|unit| &unit.descriptors)
        .filter(|object| object.kind == ObjectKind::Class)
    {
        for effective in effective_object_fields(package, object) {
            let field = effective.field;
            if let Some(initializer_span) = field.initializer_span {
                if !matches!(
                    field.value_type,
                    ValueType::Function(..) | ValueType::AsyncFunction(..)
                ) {
                    continue;
                }
                let initializer = find_node_by_span(&effective.unit.tree.root, initializer_span)
                    .ok_or_else(|| {
                        failure(
                            &effective.unit.source,
                            "T0060",
                            "class field initializer is unavailable",
                            field.span,
                        )
                    })?;
                let actual =
                    infer_value_type(effective.unit, initializer, &effective.unit.typed_bindings)?
                        .ok_or_else(|| {
                            failure(
                                &effective.unit.source,
                                "T0060",
                                format!(
                                    "class field `{}` initializer type cannot be inferred",
                                    field.name
                                ),
                                initializer.span,
                            )
                        })?;
                if !value_types_compatible(&effective.unit.descriptors, &field.value_type, &actual)
                {
                    return Err(failure(
                        &effective.unit.source,
                        "T0060",
                        format!(
                            "class field `{}` requires `{}`, found `{}`",
                            field.name,
                            diagnostic_value_type(&effective.unit.descriptors, &field.value_type),
                            diagnostic_value_type(&effective.unit.descriptors, &actual)
                        ),
                        initializer.span,
                    ));
                }
                continue;
            }
            if canonical_default(&field.value_type).is_some()
                || matches!(
                    field.value_type,
                    ValueType::PlatformStreamHandle
                        | ValueType::PlatformResourceHandle
                        | ValueType::FilesystemAuthority
                )
            {
                continue;
            }
            return Err(failure(
                &effective.unit.source,
                "T0061",
                format!(
                    "class field `{}` contributed to `{}` has no canonical default and requires an initializer",
                    field.name, object.name
                ),
                field.span,
            ));
        }
    }
    Ok(())
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
                        .flat_map(|candidate| &candidate.descriptors)
                        .find(|object| object.span == span)
                })
                .is_some_and(|object| object.resource_owning),
            _ => false,
        }
    }
    fn effective_method<'a>(
        unit: &'a SemanticUnit,
        object_identity: &ObjectIdentity,
        method_name: &str,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| {
                method.owner_identity.as_ref() == Some(object_identity)
                    && method.name == method_name
            })
            .or_else(|| {
                unit.descriptors
                    .iter()
                    .find(|object| object.identity == *object_identity)
                    .and_then(|object| object.base.as_ref())
                    .and_then(|base| effective_method(unit, base, method_name))
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
                unit.descriptors
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
            unit.descriptors
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
                .flat_map(|candidate| &candidate.descriptors)
                .find(|object| object.span == declaration)?;
            return package
                .units
                .iter()
                .flat_map(|candidate| &candidate.functions)
                .find(|function| {
                    function.owner_identity.as_ref() == Some(&object.identity)
                        && function.name == "construct"
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
                            &object_name,
                            node_text(&unit.source, member)
                        )
                        .is_some_and(|method| {
                            method.written_invocation_mode == InvocationMode::Consuming
                        })
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
                if contract.exact_invocation_mode != InvocationMode::Consuming
                    && contract.owner.is_some()
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
                    contract.exact_invocation_mode = InvocationMode::Consuming;
                }
            }
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "invocation-mode inference and conformance validation form one fixed-point pass"
)]
pub(super) fn infer_and_validate_invocation_modes(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    fn root_name(node: &SyntaxNode) -> Option<&SyntaxNode> {
        match node.kind {
            SyntaxKind::Name => Some(node),
            SyntaxKind::MemberExpression
            | SyntaxKind::IndexExpression
            | SyntaxKind::GroupExpression => node.children.first().and_then(root_name),
            _ => None,
        }
    }

    fn tracked_receiver(
        unit: &SemanticUnit,
        contract: &FunctionContract,
        node: &SyntaxNode,
    ) -> bool {
        root_name(node).is_some_and(|root| {
            let name = node_text(&unit.source, root);
            name == "this" || contract.captures.iter().any(|capture| capture == name)
        })
    }

    fn call_mode(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        contract: &FunctionContract,
        call: &SyntaxNode,
    ) -> InvocationMode {
        let Some(callee) = call.children.first() else {
            return InvocationMode::Shared;
        };
        if callee.kind == SyntaxKind::Name
            && tracked_receiver(unit, contract, callee)
            && let Ok(Some(
                ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects),
            )) = infer_value_type(unit, callee, &unit.typed_bindings)
        {
            return effects.modes.written;
        }
        let [receiver, member] = callee.children.as_slice() else {
            return InvocationMode::Shared;
        };
        if !tracked_receiver(unit, contract, receiver) {
            return InvocationMode::Shared;
        }
        let Ok(Some(receiver_type)) =
            infer_receiver_value_type(unit, receiver, &unit.typed_bindings)
        else {
            return InvocationMode::Shared;
        };
        member_invocation_mode(
            package,
            unit,
            &receiver_type,
            node_text(&unit.source, member),
        )
    }

    fn required_mode(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        contract: &FunctionContract,
        node: &SyntaxNode,
    ) -> InvocationMode {
        if node.span != contract.span
            && matches!(
                node.kind,
                SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
            )
        {
            return InvocationMode::Shared;
        }
        let local = match node.kind {
            SyntaxKind::Assignment | SyntaxKind::PostfixExpression
                if node
                    .children
                    .first()
                    .is_some_and(|target| tracked_receiver(unit, contract, target)) =>
            {
                InvocationMode::Mutable
            }
            SyntaxKind::UnaryExpression
                if unary_operator_text(unit, node).as_deref() == Some("move")
                    && node
                        .children
                        .last()
                        .is_some_and(|target| tracked_receiver(unit, contract, target)) =>
            {
                InvocationMode::Consuming
            }
            SyntaxKind::CallExpression => call_mode(package, unit, contract, node),
            _ => InvocationMode::Shared,
        };
        node.children.iter().fold(local, |mode, child| {
            mode.max(required_mode(package, unit, contract, child))
        })
    }

    let inferred = package
        .units
        .iter()
        .flat_map(|unit| {
            unit.functions.iter().filter_map(|contract| {
                find_node_by_span(&unit.tree.root, contract.span).map(|node| {
                    (
                        span_key(contract.span),
                        contract
                            .exact_invocation_mode
                            .max(required_mode(package, unit, contract, node)),
                    )
                })
            })
        })
        .collect::<BTreeMap<_, _>>();

    for unit in &mut package.units {
        for contract in &mut unit.functions {
            if let Some(mode) = inferred.get(&span_key(contract.span)) {
                contract.exact_invocation_mode = *mode;
            }
            if !contract
                .written_invocation_mode
                .accepts(contract.exact_invocation_mode)
            {
                let required = contract.exact_invocation_mode.reflection_name();
                return Err(failure(
                    &unit.source,
                    "T0120",
                    format!(
                        "callable `{}` requires {required} invocation because its body uses {required} access; declare `{}function`",
                        contract.name,
                        contract.exact_invocation_mode.source_prefix()
                    ),
                    contract.span,
                ));
            }
        }
    }
    let binding_modes = package
        .units
        .iter()
        .enumerate()
        .flat_map(|(unit_index, unit)| {
            unit.typed_bindings
                .iter()
                .enumerate()
                .filter_map(move |(binding_index, binding)| {
                    let node = find_node_by_span(&unit.tree.root, binding.span)?;
                    let value = node.children.last()?;
                    let actual = infer_value_type(unit, value, &unit.typed_bindings).ok()??;
                    match actual {
                        ValueType::Function(_, _, effects)
                        | ValueType::AsyncFunction(_, _, _, effects) => {
                            Some((unit_index, binding_index, effects.modes.exact))
                        }
                        _ => None,
                    }
                })
        })
        .collect::<Vec<_>>();
    for (unit_index, binding_index, exact) in binding_modes {
        match &mut package.units[unit_index].typed_bindings[binding_index].value_type {
            ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects) => {
                effects.modes.exact = exact;
            }
            _ => {}
        }
    }
    Ok(())
}

fn open_projected_interface<'a>(
    projection: &'a crate::projection::Projection,
    value_type: &ValueType,
) -> Option<&'a crate::projection::ProjectedItem> {
    match value_type {
        ValueType::Object(identity) if identity.application.is_none() => projection
            .item(&identity.namespace, &identity.name)
            .filter(|item| {
                matches!(
                    &item.kind,
                    crate::projection::ProjectedKind::Interface(interface)
                        if interface.associated_type.is_some()
                )
            }),
        ValueType::Optional(inner) => open_projected_interface(projection, inner),
        ValueType::Iterator(item)
        | ValueType::IterationStep(item)
        | ValueType::AsyncIterationStep(item)
        | ValueType::ChannelPair(item)
        | ValueType::ChannelSender(item)
        | ValueType::ChannelReceiver(item)
        | ValueType::ChannelSendOutcome(item)
        | ValueType::ChannelReceiveOutcome(item)
        | ValueType::DocumentDecodeOutcome(item)
        | ValueType::List(item)
        | ValueType::Set(item)
        | ValueType::Tuple(item, _)
        | ValueType::UnorderedSet(item)
        | ValueType::Task(item, _)
        | ValueType::ScopedTask(item, _)
        | ValueType::TaskOutcome(item)
        | ValueType::Reference(item)
        | ValueType::SharedReference(item) => {
            open_projected_interface(projection, item.value_type_ref())
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            open_projected_interface(projection, key.value_type_ref())
                .or_else(|| open_projected_interface(projection, value.value_type_ref()))
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => parameters
            .iter()
            .find_map(|parameter| open_projected_interface(projection, parameter.value_type_ref()))
            .or_else(|| open_projected_interface(projection, result.value_type_ref())),
        _ => None,
    }
}

fn validate_closed_projected_types(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in package
        .units
        .iter()
        .filter(|unit| !unit.namespace.starts_with("/deps/"))
    {
        for field in unit.descriptors.iter().flat_map(|object| &object.fields) {
            if let Some(item) = open_projected_interface(&package.projection, &field.value_type) {
                return Err(failure(
                    &unit.source,
                    "T0127",
                    format!(
                        "projected interface `{}` requires one closed associated type",
                        item.name
                    ),
                    field.span,
                ));
            }
        }
        for function in &unit.functions {
            let open = function
                .parameters
                .iter()
                .filter_map(|parameter| parameter.value_type.as_ref())
                .chain(function.return_type.as_ref())
                .chain(&function.thrown_types)
                .find_map(|value_type| open_projected_interface(&package.projection, value_type));
            if let Some(item) = open {
                return Err(failure(
                    &unit.source,
                    "T0127",
                    format!(
                        "projected interface `{}` requires one closed associated type",
                        item.name
                    ),
                    function.span,
                ));
            }
        }
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "application materialization expands inherited identities and bound contracts atomically"
)]
fn materialize_projected_interface_applications(package: &mut SemanticPackage) {
    let projected_methods = package
        .units
        .iter()
        .flat_map(|unit| &unit.functions)
        .filter_map(|function| {
            function
                .owner_identity
                .as_ref()
                .filter(|owner| owner.namespace.starts_with("/deps/"))
                .map(|owner| (owner.clone(), function.clone()))
        })
        .fold(
            BTreeMap::<ObjectIdentity, Vec<FunctionContract>>::new(),
            |mut methods, (owner, function)| {
                methods.entry(owner).or_default().push(function);
                methods
            },
        );
    for unit in &mut package.units {
        for object in &mut unit.descriptors {
            let mut interface_index = 0;
            while interface_index < object.interfaces.len() {
                let interface_identity = object.interfaces[interface_index].clone();
                let supertraits = package
                    .projection
                    .item(&interface_identity.namespace, &interface_identity.name)
                    .and_then(|item| match &item.kind {
                        crate::projection::ProjectedKind::Interface(interface) => {
                            Some(interface.supertraits.clone())
                        }
                        _ => None,
                    })
                    .unwrap_or_default();
                for supertrait in supertraits {
                    let Some(item) = package
                        .projection
                        .dependencies
                        .iter()
                        .flat_map(|dependency| &dependency.items)
                        .find(|item| item.rust_path == supertrait.rust_path)
                    else {
                        continue;
                    };
                    let inherited = ObjectIdentity {
                        namespace: item.namespace.clone(),
                        name: item.name.clone(),
                        application: interface_identity.application.clone(),
                        application_key: interface_identity.application_key.clone(),
                    };
                    if !object.interfaces.contains(&inherited) {
                        object.interfaces.push(inherited);
                    }
                }
                interface_index += 1;
            }
        }
    }
    let applications = package
        .units
        .iter()
        .flat_map(|unit| &unit.descriptors)
        .flat_map(|object| &object.interfaces)
        .filter(|identity| identity.application.is_some())
        .cloned()
        .collect::<BTreeSet<_>>();
    for identity in applications {
        let Some(application) = identity.application.as_deref() else {
            continue;
        };
        let inherited_interfaces = package
            .projection
            .item(&identity.namespace, &identity.name)
            .and_then(|item| match &item.kind {
                crate::projection::ProjectedKind::Interface(interface) => Some(interface),
                _ => None,
            })
            .into_iter()
            .flat_map(|interface| &interface.supertraits)
            .map(|supertrait| ObjectIdentity {
                namespace: supertrait.namespace.clone(),
                name: supertrait.name.clone(),
                application: identity.application.clone(),
                application_key: identity.application_key.clone(),
            })
            .collect::<Vec<_>>();
        let projectable = package
            .projection
            .item(&identity.namespace, &identity.name)
            .is_some_and(|item| {
                matches!(
                    &item.kind,
                    crate::projection::ProjectedKind::Interface(interface)
                        if interface.associated_type.is_some()
                )
            });
        if !projectable {
            continue;
        }
        for unit in &mut package.units {
            if unit
                .descriptors
                .iter()
                .any(|candidate| candidate.identity == identity)
            {
                continue;
            }
            let base = identity.base();
            let Some(interface) = unit
                .descriptors
                .iter()
                .find(|candidate| {
                    candidate.identity == base && candidate.kind == ObjectKind::Interface
                })
                .cloned()
            else {
                continue;
            };
            let mut bound_interface = interface;
            bound_interface.identity = identity.clone();
            bound_interface.name = identity.to_string();
            for inherited in &inherited_interfaces {
                if !bound_interface.interfaces.contains(inherited) {
                    bound_interface.interfaces.push(inherited.clone());
                }
            }
            let methods = projected_methods
                .get(&base)
                .into_iter()
                .flatten()
                .map(|method| {
                    let mut method = bind_projected_requirement(method, Some(application));
                    method.owner = Some(identity.qualified());
                    method.owner_identity = Some(identity.clone());
                    method
                })
                .collect::<Vec<_>>();
            unit.descriptors.push(bound_interface);
            unit.functions.extend(methods);
        }
    }
}

pub(super) fn analyze_types(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    for index in 0..package.units.len() {
        let descriptors = {
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
            analyze_descriptor_contracts(unit, &alias_history, &visible_objects)?
        };
        package.units[index].descriptors = descriptors;
    }
    populate_object_aliases(package);
    for unit in &mut package.units {
        for object in &mut unit.descriptors {
            if package
                .projection
                .foreign_owns_resource(&object.identity.namespace, &object.identity.name)
            {
                object.resource_owning = true;
            }
        }
    }
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
    materialize_projected_interface_applications(package);
    populate_namespace_function_contracts(package);
    populate_function_aliases(package);
    populate_function_type_dependencies(package);
    refresh_source_descriptor_members(&mut package.units);
    validate_closed_projected_types(package)?;
    validate_descriptor_value_uses(package)?;

    collect_initial_typed_bindings(package)?;
    specialize_projected_results(package)?;
    for unit in &package.units {
        validate_invocation_only_members(unit)?;
    }
    validate_resource_collection_types(package)?;
    infer_receiver_consumption(package);
    populate_closure_captures(package);
    infer_and_validate_invocation_modes(package)?;
    validate_object_conformance(package)?;
    Ok(())
}
pub(super) fn collect_initial_typed_bindings(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    rebuild_typed_bindings(package)
}

pub(super) fn refresh_typed_bindings_after_effect_inference(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    rebuild_typed_bindings(package)
}

fn rebuild_typed_bindings(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
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
    Ok(())
}

#[derive(Clone)]
struct PendingProjectedSpecialization {
    unit: usize,
    span: Span,
    parameter: String,
    rust_type: String,
    projected_result: crate::projection::ProjectedType,
    value_type: ValueType,
    bounds: Vec<String>,
}

#[expect(
    clippy::too_many_lines,
    reason = "oracle proof and specialization installation remain one atomic semantic step"
)]
fn specialize_projected_results(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    let mut pending = Vec::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        collect_projected_destinations(
            package,
            unit,
            &unit.tree.root,
            None,
            None,
            unit_index,
            &mut pending,
        )?;
    }
    let questions = pending
        .iter()
        .flat_map(|specialization| {
            specialization
                .bounds
                .iter()
                .map(|rust_bound| crate::BoundQuestion {
                    rust_type: specialization.rust_type.clone(),
                    rust_bound: rust_bound.clone(),
                })
        })
        .collect::<Vec<_>>();
    let workspace = package.root.join(".trn/dependencies");
    let report = crate::ProjectionOracle::new(
        &workspace,
        &package.projection.cache_identity,
        package.projection.containment,
    )
    .prove_bounds(&questions)
    .map_err(|error| {
        let first = pending
            .first()
            .expect("an oracle failure requires at least one destination-bound question");
        failure(
            &package.units[first.unit].source,
            "T0119",
            format!(
                "projected result proof could not run: {}",
                oracle_diagnostic_summary(&error.message)
            ),
            first.span,
        )
    })?;
    let answers = report
        .evidence
        .iter()
        .map(|evidence| (evidence.question.clone(), evidence.answer.clone()))
        .collect::<BTreeMap<_, _>>();
    package.projection.probes.extend(report.evidence);
    package.projection.probe_wall_time_ms = package
        .projection
        .probe_wall_time_ms
        .saturating_add(report.wall_time_ms);
    for specialization in pending {
        for rust_bound in &specialization.bounds {
            let question = crate::BoundQuestion {
                rust_type: specialization.rust_type.clone(),
                rust_bound: rust_bound.clone(),
            };
            match answers.get(&question) {
                Some(crate::ProbeAnswer::Yes) => {}
                Some(crate::ProbeAnswer::No) => {
                    return Err(failure(
                        &package.units[specialization.unit].source,
                        "T0119",
                        format!(
                            "projected result destination `{}` does not satisfy `{rust_bound}`",
                            specialization.value_type
                        ),
                        specialization.span,
                    ));
                }
                Some(crate::ProbeAnswer::Unknown { reason }) => {
                    return Err(failure(
                        &package.units[specialization.unit].source,
                        "T0119",
                        format!(
                            "projected result destination `{}` could not be proven against `{rust_bound}`: {}",
                            specialization.value_type,
                            oracle_diagnostic_summary(reason)
                        ),
                        specialization.span,
                    ));
                }
                None => {
                    return Err(failure(
                        &package.units[specialization.unit].source,
                        "T0119",
                        format!(
                            "projection oracle returned no answer for destination `{}` against `{rust_bound}`",
                            specialization.value_type
                        ),
                        specialization.span,
                    ));
                }
            }
        }
        let key = (
            specialization.span.file,
            specialization.span.start,
            specialization.span.end,
        );
        package.units[specialization.unit]
            .projected_call_specializations
            .insert(
                key,
                ProjectedCallSpecialization {
                    parameter: specialization.parameter,
                    rust_type: specialization.rust_type,
                    projected_result: specialization.projected_result,
                    value_type: specialization.value_type,
                },
            );
    }
    Ok(())
}
fn oracle_diagnostic_summary(message: &str) -> String {
    const LIMIT: usize = 240;
    let summary = message
        .lines()
        .find(|line| line.trim_start().starts_with("error"))
        .or_else(|| message.lines().find(|line| !line.trim().is_empty()))
        .unwrap_or("projection proof failed")
        .trim();
    let mut characters = summary.chars();
    let concise = characters.by_ref().take(LIMIT).collect::<String>();
    if characters.next().is_some() {
        format!("{concise}…")
    } else {
        concise
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one syntax walk makes every permitted destination context explicit"
)]
fn collect_projected_destinations(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    expected: Option<&ValueType>,
    function_return: Option<&ValueType>,
    unit_index: usize,
    pending: &mut Vec<PendingProjectedSpecialization>,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && let Some(function) = projected_function_for_call(package, unit, callee)
        && let Some(destination_result) = &function.destination_result
    {
        let destination = expected.cloned().ok_or_else(|| {
            failure(
                &unit.source,
                "T0117",
                "projected generic result requires one explicit destination type",
                node.span,
            )
        })?;
        let expected_projected =
            destination_projected_type(package, &destination).map_err(|reason| {
                failure(
                    &unit.source,
                    "T0118",
                    format!(
                        "projected result destination `{destination}` is not supported: {reason}"
                    ),
                    node.span,
                )
            })?;
        let projected_destination = select_projected_generic_destination(
            &function.result,
            &destination_result.parameter,
            &expected_projected,
        )
        .map_err(|()| {
            failure(
                &unit.source,
                "T0117",
                "projected result template repeats its generic parameter with incompatible destination types",
                node.span,
            )
        })?
        .ok_or_else(|| {
            failure(
                &unit.source,
                "T0117",
                format!(
                    "projected result template `{}` cannot produce the required destination `{destination}`",
                    function.result.terrane_name()
                ),
                node.span,
            )
        })?;
        let projected_result = align_projected_result_representation(
            &substitute_projected_generic(
                &function.result,
                &destination_result.parameter,
                &projected_destination,
            ),
            &expected_projected,
        );
        pending.push(PendingProjectedSpecialization {
            unit: unit_index,
            span: node.span,
            parameter: destination_result.parameter.clone(),
            rust_type: projected_destination.rust_type(),
            projected_result,
            value_type: destination,
            bounds: destination_result.rust_bounds.clone(),
        });
    }

    if is_function_node(node) {
        let return_type = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
            .and_then(|contract| contract.return_type.clone());
        for child in &node.children {
            collect_projected_destinations(
                package,
                unit,
                child,
                None,
                return_type.as_ref(),
                unit_index,
                pending,
            )?;
        }
        return Ok(());
    }
    if node.kind == SyntaxKind::ReturnStatement {
        for child in &node.children {
            collect_projected_destinations(
                package,
                unit,
                child,
                function_return,
                function_return,
                unit_index,
                pending,
            )?;
        }
        return Ok(());
    }
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
        // The parser appends an initializer as the final binding/assignment child.
        // Declaration metadata is otherwise final, so exclude those non-value kinds explicitly.
        let initializer = node.children.last().filter(|child| {
            !matches!(
                child.kind,
                SyntaxKind::Name
                    | SyntaxKind::Visibility
                    | SyntaxKind::DeclarationQualifier
                    | SyntaxKind::TypeExpression
                    | SyntaxKind::FieldMetadata
            )
        });
        let written_destination = node
            .children
            .iter()
            .any(|child| child.kind == SyntaxKind::TypeExpression)
            .then(|| {
                unit.typed_bindings
                    .iter()
                    .rev()
                    .find(|binding| binding.span == node.span)
                    .map(|binding| binding.value_type.clone())
            })
            .flatten()
            .or_else(|| {
                (node.kind == SyntaxKind::Assignment)
                    .then(|| node.children.first())
                    .flatten()
                    .filter(|target| target.kind == SyntaxKind::MemberExpression)
                    .and_then(|target| {
                        infer_member_value_type(unit, target, &unit.typed_bindings)
                            .ok()
                            .flatten()
                    })
            });
        for child in &node.children {
            collect_projected_destinations(
                package,
                unit,
                child,
                if Some(child) == initializer {
                    written_destination.as_ref()
                } else {
                    None
                },
                function_return,
                unit_index,
                pending,
            )?;
        }
        return Ok(());
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
    {
        collect_projected_destinations(
            package,
            unit,
            callee,
            None,
            function_return,
            unit_index,
            pending,
        )?;
        let parameter_types = match infer_value_type(unit, callee, &unit.typed_bindings) {
            Ok(Some(
                ValueType::Function(parameters, _, _)
                | ValueType::AsyncFunction(parameters, _, _, _),
            )) => parameters,
            Ok(_) | Err(_) => Vec::new(),
        };
        for (index, argument) in arguments.children.iter().enumerate() {
            let value = argument.children.last().unwrap_or(argument);
            collect_projected_destinations(
                package,
                unit,
                value,
                parameter_types.get(index).map(ElementType::value_type_ref),
                function_return,
                unit_index,
                pending,
            )?;
        }
        return Ok(());
    }
    let transparent_destination = matches!(node.kind, SyntaxKind::GroupExpression)
        || (node.kind == SyntaxKind::UnaryExpression
            && node_text(&unit.source, node)
                .trim_start()
                .starts_with("await "));
    for child in &node.children {
        collect_projected_destinations(
            package,
            unit,
            child,
            transparent_destination.then_some(expected).flatten(),
            function_return,
            unit_index,
            pending,
        )?;
    }
    Ok(())
}

pub(crate) fn destination_projected_type(
    package: &SemanticPackage,
    value_type: &ValueType,
) -> Result<crate::projection::ProjectedType, &'static str> {
    use crate::projection::ProjectedType;
    Ok(match value_type {
        ValueType::Scalar(ScalarType::None) => ProjectedType::None,
        ValueType::Scalar(ScalarType::Bool) => ProjectedType::Bool,
        ValueType::Scalar(ScalarType::Int) => ProjectedType::Int,
        ValueType::Scalar(
            scalar @ (ScalarType::Int8
            | ScalarType::Int16
            | ScalarType::Int32
            | ScalarType::Int64
            | ScalarType::Int128
            | ScalarType::Uint8
            | ScalarType::Uint16
            | ScalarType::Uint32
            | ScalarType::Uint64
            | ScalarType::Uint128),
        ) => ProjectedType::FixedInt(
            scalar
                .rust_type()
                .expect("fixed-width integer has a native Rust representation")
                .to_owned(),
        ),
        ValueType::Scalar(ScalarType::Float32) => ProjectedType::Float32,
        ValueType::Scalar(ScalarType::Float64) => ProjectedType::Float,
        ValueType::Scalar(ScalarType::String) => ProjectedType::String,
        ValueType::Scalar(ScalarType::Bytes) => ProjectedType::Bytes,
        ValueType::Optional(inner) => {
            ProjectedType::Optional(Box::new(destination_projected_type(package, inner)?))
        }
        ValueType::List(item) => {
            let item = destination_projected_type(package, item.value_type_ref())?;
            ProjectedType::Sequence {
                rust_path: format!("Vec<{}>", item.rust_type()),
                item: Box::new(item),
            }
        }
        ValueType::Map(key, value) | ValueType::UnorderedMap(key, value) => {
            let ordered = matches!(value_type, ValueType::Map(_, _));
            let key = destination_projected_type(package, key.value_type_ref())?;
            let value = destination_projected_type(package, value.value_type_ref())?;
            ProjectedType::Mapping {
                rust_path: format!(
                    "std::collections::{}<{}, {}>",
                    if ordered { "BTreeMap" } else { "HashMap" },
                    key.rust_type(),
                    value.rust_type()
                ),
                key: Box::new(key),
                value: Box::new(value),
                ordered,
            }
        }
        ValueType::Set(item) | ValueType::UnorderedSet(item) => {
            let ordered = matches!(value_type, ValueType::Set(_));
            let item = destination_projected_type(package, item.value_type_ref())?;
            ProjectedType::Set {
                rust_path: format!(
                    "std::collections::{}<{}>",
                    if ordered { "BTreeSet" } else { "HashSet" },
                    item.rust_type()
                ),
                item: Box::new(item),
                ordered,
            }
        }
        ValueType::Tuple(item, Some(length)) => {
            let item = destination_projected_type(package, item.value_type_ref())?;
            ProjectedType::Tuple(vec![item; *length])
        }
        ValueType::Object(identity) => {
            let item = package
                .projection
                .item(&identity.namespace, &identity.name)
                .filter(|item| {
                    matches!(
                        item.kind,
                        crate::projection::ProjectedKind::ForeignType { .. }
                            | crate::projection::ProjectedKind::Enum { .. }
                    )
                })
                .ok_or(
                    "source-declared object destinations have no dependency conversion contract",
                )?;
            ProjectedType::Foreign {
                rust_path: item.rust_path.clone(),
                name: item.name.clone(),
                base_rust_path: item.rust_path.clone(),
                arguments: Vec::new(),
            }
        }
        ValueType::Reference(_) | ValueType::SharedReference(_) => {
            return Err("borrowed results cannot escape a projected call");
        }
        _ => return Err("the destination is outside the closed projected result set"),
    })
}

fn projected_types_share_concrete_rust_representation(
    left: &crate::projection::ProjectedType,
    right: &crate::projection::ProjectedType,
) -> bool {
    use crate::projection::ProjectedType;

    fn integer_rust_type(projected: &ProjectedType) -> Option<&str> {
        match projected {
            ProjectedType::Int => Some("i64"),
            ProjectedType::FixedInt(name) | ProjectedType::RustInt(name) => Some(name),
            _ => None,
        }
    }

    left == right
        || integer_rust_type(left)
            .zip(integer_rust_type(right))
            .is_some_and(|(left, right)| left == right)
}

fn select_projected_generic_destination(
    template: &crate::projection::ProjectedType,
    parameter: &str,
    expected: &crate::projection::ProjectedType,
) -> Result<Option<crate::projection::ProjectedType>, ()> {
    fn collect(
        template: &crate::projection::ProjectedType,
        parameter: &str,
        expected: &crate::projection::ProjectedType,
        destinations: &mut Vec<crate::projection::ProjectedType>,
    ) -> bool {
        use crate::projection::ProjectedType;
        match (template, expected) {
            (ProjectedType::Generic(name), expected) if name == parameter => {
                destinations.push(expected.clone());
                true
            }
            (ProjectedType::Optional(template), ProjectedType::Optional(expected))
            | (
                ProjectedType::Sequence { item: template, .. },
                ProjectedType::Sequence { item: expected, .. },
            )
            | (
                ProjectedType::Set { item: template, .. },
                ProjectedType::Set { item: expected, .. },
            ) => collect(template, parameter, expected, destinations),
            (
                ProjectedType::Mapping {
                    key: template_key,
                    value: template_value,
                    ..
                },
                ProjectedType::Mapping {
                    key: expected_key,
                    value: expected_value,
                    ..
                },
            ) => {
                collect(template_key, parameter, expected_key, destinations)
                    && collect(template_value, parameter, expected_value, destinations)
            }
            (ProjectedType::Tuple(template), ProjectedType::Tuple(expected))
                if template.len() == expected.len() =>
            {
                template.iter().zip(expected).all(|(template, expected)| {
                    collect(template, parameter, expected, destinations)
                })
            }
            (template, expected)
                if projected_types_share_concrete_rust_representation(template, expected) =>
            {
                true
            }
            _ => false,
        }
    }
    let mut destinations = Vec::new();
    if !collect(template, parameter, expected, &mut destinations) {
        return Ok(None);
    }
    let Some(first) = destinations.first().cloned() else {
        return Ok(None);
    };
    if destinations.iter().all(|destination| destination == &first) {
        Ok(Some(first))
    } else {
        Err(())
    }
}

fn align_projected_result_representation(
    projected: &crate::projection::ProjectedType,
    expected: &crate::projection::ProjectedType,
) -> crate::projection::ProjectedType {
    use crate::projection::ProjectedType;

    match (projected, expected) {
        (
            ProjectedType::Sequence { rust_path, item },
            ProjectedType::Sequence {
                item: expected_item,
                ..
            },
        ) => {
            let item = align_projected_result_representation(item, expected_item);
            ProjectedType::Sequence {
                rust_path: instantiate_projected_container(rust_path, &[item.rust_type()]),
                item: Box::new(item),
            }
        }
        (
            ProjectedType::Mapping {
                rust_path,
                key,
                value,
                ordered,
            },
            ProjectedType::Mapping {
                key: expected_key,
                value: expected_value,
                ..
            },
        ) => {
            let key = align_projected_result_representation(key, expected_key);
            let value = align_projected_result_representation(value, expected_value);
            ProjectedType::Mapping {
                rust_path: instantiate_projected_container(
                    rust_path,
                    &[key.rust_type(), value.rust_type()],
                ),
                key: Box::new(key),
                value: Box::new(value),
                ordered: *ordered,
            }
        }
        (
            ProjectedType::Set {
                rust_path,
                item,
                ordered,
            },
            ProjectedType::Set {
                item: expected_item,
                ..
            },
        ) => {
            let item = align_projected_result_representation(item, expected_item);
            ProjectedType::Set {
                rust_path: instantiate_projected_container(rust_path, &[item.rust_type()]),
                item: Box::new(item),
                ordered: *ordered,
            }
        }
        (ProjectedType::Tuple(items), ProjectedType::Tuple(expected_items))
            if items.len() == expected_items.len() =>
        {
            ProjectedType::Tuple(
                items
                    .iter()
                    .zip(expected_items)
                    .map(|(item, expected)| align_projected_result_representation(item, expected))
                    .collect(),
            )
        }
        (ProjectedType::Optional(inner), ProjectedType::Optional(expected_inner)) => {
            ProjectedType::Optional(Box::new(align_projected_result_representation(
                inner,
                expected_inner,
            )))
        }
        (projected, expected)
            if projected_types_share_concrete_rust_representation(projected, expected) =>
        {
            expected.clone()
        }
        (projected, _) => projected.clone(),
    }
}

fn substitute_projected_generic(
    template: &crate::projection::ProjectedType,
    parameter: &str,
    destination: &crate::projection::ProjectedType,
) -> crate::projection::ProjectedType {
    use crate::projection::ProjectedType;
    match template {
        ProjectedType::Generic(name) if name == parameter => destination.clone(),
        ProjectedType::Sequence { rust_path, item } => {
            let item = substitute_projected_generic(item, parameter, destination);
            ProjectedType::Sequence {
                rust_path: instantiate_projected_container(rust_path, &[item.rust_type()]),
                item: Box::new(item),
            }
        }
        ProjectedType::Mapping {
            rust_path,
            key,
            value,
            ordered,
        } => {
            let key = substitute_projected_generic(key, parameter, destination);
            let value = substitute_projected_generic(value, parameter, destination);
            ProjectedType::Mapping {
                rust_path: instantiate_projected_container(
                    rust_path,
                    &[key.rust_type(), value.rust_type()],
                ),
                key: Box::new(key),
                value: Box::new(value),
                ordered: *ordered,
            }
        }
        ProjectedType::Set {
            rust_path,
            item,
            ordered,
        } => {
            let item = substitute_projected_generic(item, parameter, destination);
            ProjectedType::Set {
                rust_path: instantiate_projected_container(rust_path, &[item.rust_type()]),
                item: Box::new(item),
                ordered: *ordered,
            }
        }
        ProjectedType::Tuple(items) => ProjectedType::Tuple(
            items
                .iter()
                .map(|item| substitute_projected_generic(item, parameter, destination))
                .collect(),
        ),
        ProjectedType::Optional(inner) => ProjectedType::Optional(Box::new(
            substitute_projected_generic(inner, parameter, destination),
        )),
        ProjectedType::Foreign {
            name,
            base_rust_path,
            arguments,
            ..
        } => {
            let arguments = arguments
                .iter()
                .map(|argument| substitute_projected_generic(argument, parameter, destination))
                .collect::<Vec<_>>();
            ProjectedType::Foreign {
                rust_path: if arguments.is_empty() {
                    base_rust_path.clone()
                } else {
                    format!(
                        "{base_rust_path}<{}>",
                        arguments
                            .iter()
                            .map(ProjectedType::rust_type)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                },
                name: name.clone(),
                base_rust_path: base_rust_path.clone(),
                arguments,
            }
        }
        other => other.clone(),
    }
}

fn instantiate_projected_container(template: &str, arguments: &[String]) -> String {
    let constructor = template
        .split_once('<')
        .map_or(template, |(constructor, _)| constructor);
    format!("{constructor}<{}>", arguments.join(", "))
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
            .filter(|contract| contract.is_anonymous)
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
