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
pub(crate) fn effective_object_interfaces<'a>(
    package: &'a SemanticPackage,
    object: &'a DescriptorContract,
) -> Vec<&'a ObjectIdentity> {
    let mut interfaces = object
        .base
        .as_ref()
        .and_then(|identity| descriptor_declaration(package, identity))
        .map_or_else(Vec::new, |(_, base)| {
            effective_object_interfaces(package, base)
        });
    for interface in &object.interfaces {
        if !interfaces.contains(&interface) {
            interfaces.push(interface);
        }
    }
    interfaces
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
        let generic_parameters = super::generics::generic_parameters(unit, node, &visible)?;
        let clause_identities = |clause_kind| -> Result<Vec<ObjectIdentity>, SemanticFailure> {
            let Some(clause) = node.children.iter().find(|child| child.kind == clause_kind) else {
                return Ok(Vec::new());
            };
            let is_unsafe = matches!(
                clause_kind,
                SyntaxKind::ImplementsClause | SyntaxKind::ExtendsClause
            ) && clause.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(&unit.source, child) == "unsafe"
            });
            clause
                .children
                .iter()
                .filter(|type_node| type_node.kind != SyntaxKind::DeclarationQualifier)
                .map(|type_node| {
                    let mut type_node = type_node;
                    while matches!(
                        type_node.kind,
                        SyntaxKind::TypeExpression | SyntaxKind::GroupExpression
                    ) && type_node.children.len() == 1
                    {
                        type_node = &type_node.children[0];
                    }
                    if type_node.kind == SyntaxKind::AppliedType {
                        let ValueType::Object(identity) = declared_value_type_with_visible_objects(
                            unit,
                            type_node,
                            &visible,
                            visible_objects,
                        )?
                        else {
                            return Err(failure(
                                &unit.source,
                                "T0054",
                                "relationship requires a nominal object type",
                                type_node.span,
                            ));
                        };
                        return Ok(identity.with_safety(is_unsafe));
                    }
                    let name = node_text(&unit.source, type_node);
                    let lookup_name = if is_unsafe {
                        format!("unsafe::{name}")
                    } else {
                        name.to_owned()
                    };
                    Ok(visible_objects
                        .get(&lookup_name)
                        .cloned()
                        .unwrap_or_else(|| {
                            if name == "throwable" {
                                ObjectIdentity::new("/core/errors", name)
                            } else {
                                ObjectIdentity::new(&unit.namespace, name).with_safety(is_unsafe)
                            }
                        }))
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
                        && matches!(node_text(&unit.source, child), "static" | "constant")
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
                    required: !is_static && initializer.is_none() && !uses_canonical_default,
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
        let is_unsafe = kind == ObjectKind::Interface
            && node.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(&unit.source, child) == "unsafe"
            });
        descriptors.push(DescriptorContract {
            identity: ObjectIdentity::new(&unit.namespace, &name).with_safety(is_unsafe),
            name,
            span: node.span,
            generic_parameters,
            kind,
            is_unsafe,
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
            if valid {
                return Ok(());
            }
            let opposite_exists = expected == ObjectKind::Interface
                && (descriptors.iter().any(|candidate| {
                    candidate.identity.namespace == base_identity.namespace
                        && candidate.identity.name == base_identity.name
                        && candidate.identity.is_unsafe != base_identity.is_unsafe
                        && candidate.kind == ObjectKind::Interface
                }) || visible_objects.values().any(|visible| {
                    visible.namespace == base_identity.namespace
                        && visible.name == base_identity.name
                        && visible.is_unsafe != base_identity.is_unsafe
                }));
            Err(failure(
                &unit.source,
                if opposite_exists { "T0131" } else { "T0054" },
                if opposite_exists {
                    format!(
                        "interface `{}` has the opposite safety contract",
                        base_identity.name
                    )
                } else {
                    format!(
                        "`{}` does not resolve to a {role}",
                        diagnostic_object_identity(&descriptors, identity)
                    )
                },
                object.span,
            ))
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
        ValueType::Union(arms) => arms
            .iter()
            .any(|arm| value_type_owns_resource(arm, resource_identities)),
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

fn value_type_contains_nonclone_foreign(package: &SemanticPackage, value_type: &ValueType) -> bool {
    match value_type {
        ValueType::Object(identity) => package.native_capability(identity).map_or_else(
            || {
                package
                    .projection
                    .foreign_is_cloneable(&identity.namespace, &identity.name)
                    .is_some_and(|cloneable| !cloneable)
            },
            |capability| !capability.cloneable,
        ),
        ValueType::Optional(inner) => value_type_contains_nonclone_foreign(package, inner),
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
            value_type_contains_nonclone_foreign(package, item.value_type_ref())
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            value_type_contains_nonclone_foreign(package, key.value_type_ref())
                || value_type_contains_nonclone_foreign(package, value.value_type_ref())
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

#[expect(
    clippy::too_many_lines,
    reason = "Recursive resource classification mirrors semantic value shapes and preserves generic-parameter ownership profiles"
)]
pub(crate) fn application_is_resource_owning(
    package: &SemanticPackage,
    value_type: &ValueType,
) -> bool {
    fn owns(
        package: &SemanticPackage,
        value_type: &ValueType,
        resources: &BTreeSet<String>,
        parameters: &BTreeMap<String, bool>,
        visiting: &mut BTreeSet<(ObjectIdentity, Vec<bool>)>,
    ) -> bool {
        match value_type {
            ValueType::Reference(_) | ValueType::SharedReference(_) => return false,
            ValueType::TypeParameter(name) => return parameters.get(name).copied().unwrap_or(true),
            _ => {}
        }
        match value_type {
            ValueType::Object(identity) => {
                // A closed native proof takes precedence over its open nominal declaration.
                if let Some(capability) = package.native_capability(identity) {
                    return !capability.cloneable;
                }
                if resources.contains(&identity.qualified())
                    || value_type_contains_nonclone_foreign(package, value_type)
                {
                    return true;
                }
                let Some(descriptor) = package
                    .units
                    .iter()
                    .flat_map(|unit| &unit.descriptors)
                    .find(|descriptor| descriptor.identity.base() == identity.base())
                else {
                    return false;
                };
                let arguments = identity
                    .type_arguments
                    .iter()
                    .map(|argument| owns(package, argument, resources, parameters, visiting))
                    .collect::<Vec<_>>();
                let key = (identity.base(), arguments.clone());
                if !visiting.insert(key.clone()) {
                    return false;
                }
                let parameters = descriptor
                    .generic_parameters
                    .iter()
                    .enumerate()
                    .map(|(index, parameter)| {
                        (
                            parameter.name.clone(),
                            arguments.get(index).copied().unwrap_or(true),
                        )
                    })
                    .collect::<BTreeMap<_, _>>();
                let result = if descriptor.kind == ObjectKind::Enum {
                    if let Some(enumeration) = package
                        .units
                        .iter()
                        .flat_map(|unit| &unit.source_enums)
                        .find(|enumeration| enumeration.identity.base() == identity.base())
                    {
                        enumeration
                            .variants
                            .iter()
                            .flat_map(|variant| &variant.payload)
                            .any(|field| {
                                owns(package, &field.value_type, resources, &parameters, visiting)
                            })
                    } else {
                        package
                            .projection
                            .item(&identity.namespace, &identity.name)
                            .and_then(|item| {
                                let crate::rust_interop::projection::ProjectedKind::Enum {
                                    variants,
                                    ..
                                } = &item.kind
                                else {
                                    return None;
                                };
                                Some(variants.iter().flat_map(|variant| &variant.fields).any(
                                    |field| {
                                        let Some(field_type) =
                                            super::enums::projected_enum_payload_type(
                                                package, &field.ty,
                                            )
                                        else {
                                            return true;
                                        };
                                        let field_type = super::generics::substitute_value_type(
                                            &field_type,
                                            &identity.native_arguments,
                                        );
                                        owns(package, &field_type, resources, &parameters, visiting)
                                    },
                                ))
                            })
                            .unwrap_or(false)
                    }
                } else {
                    effective_object_fields(package, descriptor)
                        .iter()
                        .filter(|field| !field.is_static)
                        .any(|field| {
                            owns(package, &field.value_type, resources, &parameters, visiting)
                        })
                };
                visiting.remove(&key);
                result
            }
            ValueType::Union(arms) => arms
                .iter()
                .any(|arm| owns(package, arm, resources, parameters, visiting)),
            ValueType::Optional(inner) => owns(package, inner, resources, parameters, visiting),
            ValueType::List(inner)
            | ValueType::Set(inner)
            | ValueType::Tuple(inner, _)
            | ValueType::Iterator(inner)
            | ValueType::IterationStep(inner)
            | ValueType::UnorderedSet(inner)
            | ValueType::Task(inner, _)
            | ValueType::ScopedTask(inner, _)
            | ValueType::ChannelReceiveOutcome(inner)
            | ValueType::ChannelSendOutcome(inner)
            | ValueType::TaskOutcome(inner) => owns(
                package,
                inner.value_type_ref(),
                resources,
                parameters,
                visiting,
            ),
            ValueType::Map(key, value)
            | ValueType::Entry(key, value)
            | ValueType::UnorderedMap(key, value) => {
                owns(
                    package,
                    key.value_type_ref(),
                    resources,
                    parameters,
                    visiting,
                ) || owns(
                    package,
                    value.value_type_ref(),
                    resources,
                    parameters,
                    visiting,
                )
            }
            ValueType::PlatformStreamHandle
            | ValueType::PlatformResourceHandle
            | ValueType::ChannelPair(_)
            | ValueType::ChannelSender(_)
            | ValueType::ChannelReceiver(_) => true,
            ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects) => {
                effects.modes.written == InvocationMode::Consuming
            }
            _ => false,
        }
    }
    if let ValueType::Object(identity) = value_type
        && let Some(capability) = package.native_capability(identity)
    {
        return !capability.cloneable;
    }
    owns(
        package,
        value_type,
        &resource_identities(package),
        &BTreeMap::new(),
        &mut BTreeSet::new(),
    )
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
                        || value_type_contains_nonclone_foreign(package, &field.value_type)
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
                            matches!(
                                item.kind,
                                crate::rust_interop::projection::ProjectedKind::Interface(_)
                            )
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
            parameters
                .iter()
                .map(|parameter| parameter.with_element_type(element(&parameter.element_type())))
                .collect(),
            element(result),
            effects.clone(),
        ),
        ValueType::AsyncFunction(parameters, result, transferability, effects) => {
            ValueType::AsyncFunction(
                parameters
                    .iter()
                    .map(|parameter| {
                        parameter.with_element_type(element(&parameter.element_type()))
                    })
                    .collect(),
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
                .all(|(left, right)| {
                    left.value_type == right.value_type && left.variadic == right.variadic
                })
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
        is_unsafe: bool,
    ) -> Option<&'a FunctionContract> {
        unit.functions
            .iter()
            .find(|method| {
                method.owner_identity.as_ref() == Some(&object.identity)
                    && method.name == name
                    && method.is_unsafe == is_unsafe
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
                    .and_then(|base| effective_method(unit, base, name, is_unsafe))
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
            for interface_identity in effective_object_interfaces(package, object) {
                let lookup_name = if interface_identity.is_unsafe {
                    format!("unsafe::{}", interface_identity.name)
                } else {
                    interface_identity.name.clone()
                };
                let Some(resolved_interface) =
                    package.resolve_name(&interface_identity.namespace, &lookup_name)
                else {
                    let opposite_name = if interface_identity.is_unsafe {
                        interface_identity.name.clone()
                    } else {
                        format!("unsafe::{}", interface_identity.name)
                    };
                    if package
                        .resolve_name(&interface_identity.namespace, &opposite_name)
                        .is_some()
                    {
                        let relation = if interface_identity.is_unsafe {
                            format!(
                                "remove `unsafe` after `implements` before `{}`",
                                interface_identity.name
                            )
                        } else {
                            format!("write `implements unsafe {}`", interface_identity.name)
                        };
                        return Err(SemanticFailure {
                            source: declaration_unit.source.clone(),
                            diagnostics: vec![
                                Diagnostic::error(
                                    "T0131",
                                    format!(
                                        "interface `{}` has the opposite safety contract",
                                        interface_identity.name
                                    ),
                                    object.span,
                                )
                                .with_help(relation),
                            ],
                        });
                    }
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
                if package
                    .projection
                    .item(&interface_identity.namespace, &interface_identity.name)
                    .is_none()
                {
                    for (obligation, requirement) in [
                        (AutoTraitObligation::Send, "`Send`"),
                        (AutoTraitObligation::Sync, "`Sync`"),
                    ] {
                        if let Some(field) = effective_object_fields(package, object)
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
                                &field.unit.source,
                                "T0122",
                                format!(
                                    "class `{}` cannot implement authored interface `{}` because field `{}` does not satisfy the {requirement} transfer obligation",
                                    object.name, resolved_interface.name, field.name
                                ),
                                field.span,
                            ));
                        }
                    }
                }
                if let Some(crate::rust_interop::projection::ProjectedKind::Interface(projected)) =
                    package
                        .projection
                        .item(&interface_identity.namespace, &interface_identity.name)
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
                                    !value_type_contains_nonclone_foreign(package, application)
                                        && !value_type_owns_resource(application, &resources)
                                } else if bound == "'static" {
                                    true
                                } else {
                                    let required = package.projection.dependencies.iter().find_map(
                                        |dependency| {
                                            dependency.items.iter().find(|item| {
                                                item.rust_path == *bound
                                                    && matches!(
                                                        &item.kind,
                                                        crate::rust_interop::projection::ProjectedKind::Interface(
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
                                &field.unit.source,
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
                        crate::rust_interop::projection::ProjectedKind::Interface(projected) => {
                            Some(projected.requires_drop)
                        }
                        _ => None,
                    })
                    == Some(true)
                    && effective_method(declaration_unit, object, "destruct", false).is_none()
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
                            crate::rust_interop::projection::ProjectedKind::Interface(
                                projected,
                            ) => projected.methods.iter().find(|method| {
                                method.function.is_async
                                    && method.function.receiver
                                        != Some(crate::rust_interop::projection::Receiver::Move)
                            }),
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
                            crate::rust_interop::projection::ProjectedKind::Interface(projected)
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
                    let Some(render) = effective_method(declaration_unit, object, "render", false)
                    else {
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
                        generic_parameters: Vec::new(),
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
                        is_unsafe: false,
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
                                candidate.identity.base() == interface_identity.base()
                                    && candidate.kind == ObjectKind::Interface
                            })
                    })
                    .expect("resolved interface must have a semantic declaration");
                let interface = interface_unit
                    .descriptors
                    .iter()
                    .find(|candidate| candidate.identity.base() == interface_identity.base())
                    .expect("resolved interface must have an object contract");
                let substitutions = interface
                    .generic_parameters
                    .iter()
                    .zip(&interface_identity.type_arguments)
                    .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
                    .collect();
                let requirements = interface_unit
                    .functions
                    .iter()
                    .filter(|method| method.owner_identity.as_ref() == Some(&interface.identity))
                    .map(|method| {
                        let mut requirement = bind_projected_requirement(
                            method,
                            interface_identity.application.as_deref(),
                        );
                        for parameter in &mut requirement.parameters {
                            parameter.value_type =
                                parameter.value_type.as_ref().map(|value_type| {
                                    substitute_value_type(value_type, &substitutions)
                                });
                        }
                        requirement.return_type = requirement
                            .return_type
                            .as_ref()
                            .map(|value_type| substitute_value_type(value_type, &substitutions));
                        requirement
                    })
                    .collect::<Vec<_>>();
                for required in &requirements {
                    let Some(actual) = effective_method(
                        declaration_unit,
                        object,
                        &required.name,
                        required.is_unsafe,
                    ) else {
                        if effective_method(
                            declaration_unit,
                            object,
                            &required.name,
                            !required.is_unsafe,
                        )
                        .is_some()
                        {
                            return Err(failure(
                                &declaration_unit.source,
                                "T0131",
                                format!(
                                    "class `{}` implements `{}.{}` with the opposite safety contract",
                                    object.name, interface.name, required.name
                                ),
                                object.span,
                            ));
                        }
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
                            } else if method.function.result.contains_borrowed_result() {
                                Some("borrowed results require native default implementation")
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
        if object.identity.native_projection.is_some()
            || package
                .projection
                .projected_struct(&object.identity.namespace, &object.identity.name)
                .is_some()
        {
            continue;
        }
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
                validate_value_destination(
                    &effective.unit.source,
                    &effective.unit.descriptors,
                    &format!("class field {}", field.name),
                    field.value_type.clone(),
                    actual,
                    initializer,
                    "T0060",
                )?;
                continue;
            }
            if super::bindings::declaration_is_constant(package, field.span)
                && package
                    .projection
                    .constant_for_native(
                        &object.identity.namespace,
                        &object.identity.name,
                        object.identity.native_projection.as_deref(),
                        &field.name,
                    )
                    .is_some()
            {
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
            if field.required {
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
                // Moving an optional receiver field is a `mem::take` operation:
                // it leaves `none` behind, so an ordinary mutable close/reset
                // method need not consume the enclosing object.
                if node
                    .children
                    .last()
                    .and_then(|target| infer_value_type(unit, target, &unit.typed_bindings).ok())
                    .is_some_and(|value_type| matches!(value_type, Some(ValueType::Optional(_))))
                {
                    InvocationMode::Mutable
                } else {
                    InvocationMode::Consuming
                }
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
    projection: &'a crate::rust_interop::projection::Projection,
    value_type: &ValueType,
) -> Option<&'a crate::rust_interop::projection::ProjectedItem> {
    match value_type {
        ValueType::Object(identity) if identity.application.is_none() => projection
            .item(&identity.namespace, &identity.name)
            .filter(|item| {
                matches!(
                    &item.kind,
                    crate::rust_interop::projection::ProjectedKind::Interface(interface)
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
                        crate::rust_interop::projection::ProjectedKind::Interface(interface) => {
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
                        is_unsafe: matches!(
                            &item.kind,
                            crate::rust_interop::projection::ProjectedKind::Interface(interface) if interface.is_unsafe
                        ),
                        type_arguments: interface_identity.type_arguments.clone(),
                        type_arguments_key: interface_identity.type_arguments_key.clone(),
                        application: interface_identity.application.clone(),
                        application_key: interface_identity.application_key.clone(),
                        native_projection: interface_identity.native_projection.clone(),
                        native_arguments: interface_identity.native_arguments.clone(),
                        native_parameters: interface_identity.native_parameters.clone(),
                        native_arguments_key: interface_identity.native_arguments_key.clone(),
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
                crate::rust_interop::projection::ProjectedKind::Interface(interface) => {
                    Some(interface)
                }
                _ => None,
            })
            .into_iter()
            .flat_map(|interface| &interface.supertraits)
            .map(|supertrait| ObjectIdentity {
                namespace: supertrait.namespace.clone(),
                name: supertrait.name.clone(),
                is_unsafe: package
                    .projection
                    .item(&supertrait.namespace, &supertrait.name)
                    .is_some_and(|item| {
                        matches!(
                            &item.kind,
                            crate::rust_interop::projection::ProjectedKind::Interface(interface) if interface.is_unsafe
                        )
                    }),
                application: identity.application.clone(),
                application_key: identity.application_key.clone(),
                native_projection: identity.native_projection.clone(),
                native_arguments: identity.native_arguments.clone(),
                native_parameters: identity.native_parameters.clone(),
                native_arguments_key: identity.native_arguments_key.clone(),
                type_arguments: Vec::new(),
                type_arguments_key: None,
            })
            .collect::<Vec<_>>();
        let projectable = package
            .projection
            .item(&identity.namespace, &identity.name)
            .is_some_and(|item| {
                matches!(
                    &item.kind,
                    crate::rust_interop::projection::ProjectedKind::Interface(interface)
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

#[expect(
    clippy::too_many_lines,
    reason = "Enum declarations publish their variants and nominal descriptor atomically"
)]
fn analyze_source_enums(
    unit: &SemanticUnit,
    aliases: &BTreeMap<String, ScalarType>,
) -> Result<(Vec<SourceEnumContract>, Vec<DescriptorContract>), SemanticFailure> {
    let mut result = Vec::new();
    let mut descriptors = Vec::new();
    for node in unit
        .tree
        .root
        .children
        .iter()
        .filter(|node| node.kind == SyntaxKind::EnumDeclaration)
    {
        let name_node = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Name)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0053",
                    "enum declaration requires a name",
                    node.span,
                )
            })?;
        let name = node_text(&unit.source, name_node).to_owned();
        let generic_parameters = super::generics::generic_parameters(unit, node, aliases)?;
        let mut variants = Vec::new();
        if let Some(block) = node
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block)
        {
            for variant in &block.children {
                let Some(variant_name) = variant
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::Name)
                else {
                    continue;
                };
                let mut payload = Vec::new();
                if let Some(list) = variant
                    .children
                    .iter()
                    .find(|child| child.kind == SyntaxKind::ParameterList)
                {
                    for field in &list.children {
                        let Some(field_name) = field
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::Name)
                        else {
                            continue;
                        };
                        let field_type = field
                            .children
                            .iter()
                            .find(|child| child.kind == SyntaxKind::TypeExpression)
                            .ok_or_else(|| {
                                failure(
                                    &unit.source,
                                    "T0001",
                                    "enum payload fields require a type",
                                    field.span,
                                )
                            })?;
                        payload.push(SourceEnumField {
                            name: node_text(&unit.source, field_name).to_owned(),
                            span: field.span,
                            value_type: declared_value_type(unit, field_type, aliases)?,
                        });
                    }
                }
                variants.push(SourceEnumVariant {
                    name: node_text(&unit.source, variant_name).to_owned(),
                    span: variant.span,
                    payload,
                });
            }
        }
        descriptors.push(DescriptorContract {
            name: name.clone(),
            identity: ObjectIdentity::new(&unit.namespace, &name),
            span: node.span,
            kind: ObjectKind::Enum,
            generic_parameters,
            is_unsafe: false,
            resource_owning: false,
            builtin: None,
            categories: vec![TypeCategory::Value, TypeCategory::Object],
            operations: BTreeMap::new(),
            members: BTreeSet::new(),
            methods: BTreeSet::new(),
            invocation_only_methods: BTreeSet::new(),
            static_members: BTreeSet::new(),
            static_methods: BTreeSet::new(),
            base: None,
            interfaces: Vec::new(),
            traits: Vec::new(),
            fields: Vec::new(),
        });
        result.push(SourceEnumContract {
            identity: ObjectIdentity::new(&unit.namespace, &name),
            span: node.span,
            variants,
        });
    }
    Ok((result, descriptors))
}

fn prepare_source_enums(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    for index in 0..package.units.len() {
        let (source_enums, enum_descriptors) = {
            let unit = &package.units[index];
            let alias_history = descriptor_construct_alias_history(package, unit);
            let aliases = visible_descriptor_aliases(&alias_history, unit.source.id(), 0);
            analyze_source_enums(unit, &aliases)?
        };
        package.units[index].descriptors.extend(enum_descriptors);
        package.units[index].source_enums =
            source_enums.into_iter().map(std::sync::Arc::new).collect();
    }
    let source_enum_registry = std::sync::Arc::<[std::sync::Arc<SourceEnumContract>]>::from(
        package
            .units
            .iter()
            .flat_map(|unit| unit.source_enums.iter().cloned())
            .collect::<Vec<_>>(),
    );
    for unit in &mut package.units {
        unit.source_enum_registry = source_enum_registry.clone();
    }
    Ok(())
}

pub(super) fn prepare_type_declarations(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
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
                        SymbolKind::Class
                            | SymbolKind::Enum
                            | SymbolKind::Interface
                            | SymbolKind::Trait
                    )
                })
                .map(|(visible_name, symbol)| {
                    (
                        visible_name.clone(),
                        ObjectIdentity::new(&symbol.namespace, &symbol.name)
                            .with_safety(visible_name.starts_with("unsafe::")),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            analyze_descriptor_contracts(unit, &alias_history, &visible_objects)?
        };
        package.units[index].descriptors = descriptors;
    }
    prepare_source_enums(package)?;
    populate_object_aliases(package);
    for unit in &mut package.units {
        for object in &mut unit.descriptors {
            if package
                .projection
                .item(&object.identity.namespace, &object.identity.name)
                .is_some_and(|item| {
                    matches!(
                        item.kind,
                        crate::rust_interop::projection::ProjectedKind::Enum { .. },
                    )
                })
            {
                object.kind = ObjectKind::Enum;
            }
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
        for function in &mut functions {
            if let Some(identity) = unit
                .descriptors
                .iter()
                .filter(|descriptor| {
                    descriptor.span.file == function.span.file
                        && descriptor.span.start <= function.span.start
                        && descriptor.span.end >= function.span.end
                })
                .min_by_key(|descriptor| descriptor.span.end - descriptor.span.start)
                .map(|descriptor| descriptor.identity.clone())
            {
                function.owner_identity = Some(identity);
            }
        }
        package.units[index].descriptor_aliases = alias_history;
        package.units[index].functions = functions;
    }
    super::native_constructors::normalize_contracts(package)?;
    materialize_projected_interface_applications(package);
    populate_namespace_function_contracts(package);
    populate_function_aliases(package);
    populate_function_type_dependencies(package);
    refresh_source_descriptor_members(&mut package.units);
    Ok(())
}

pub(super) fn analyze_types(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    let target = crate::compilation_progress::owned_target(package.root.display());
    let stage = crate::compilation_progress::start("preparing type declarations", &target);
    prepare_type_declarations(package)?;
    validate_closed_projected_types(package)?;
    validate_descriptor_value_uses(package)?;
    stage.finish();

    let stage = crate::compilation_progress::start("inferring source types", &target);
    let bindings = crate::compilation_progress::start("inferring initial bindings", &target);
    collect_initial_typed_bindings(package)?;
    bindings.finish();
    let projected = crate::compilation_progress::start("inferring projected call results", &target);
    super::enums::validate_enum_constructions(package)?;
    populate_projected_call_result_types(package)?;
    projected.finish();
    let bindings = crate::compilation_progress::start("refreshing inferred bindings", &target);
    collect_initial_typed_bindings(package)?;
    bindings.finish();
    let native = crate::compilation_progress::start("normalizing native type contracts", &target);
    super::native_constructors::normalize_contracts(package)?;
    native.finish();
    let specialization =
        crate::compilation_progress::start("specializing projected results", &target);
    specialize_projected_results(package)?;
    specialization.finish();
    let validation =
        crate::compilation_progress::start("validating source type contracts", &target);
    for unit in &package.units {
        validate_invocation_only_members(unit)?;
    }
    validate_resource_collection_types(package)?;
    infer_receiver_consumption(package);
    populate_closure_captures(package);
    infer_and_validate_invocation_modes(package)?;
    validate_object_conformance(package)?;
    validation.finish();
    stage.finish();
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
    let enums = super::enums::resolve_enums(package);
    // Discard provisional projected results once. Subsequent passes consume only
    // the newly computed canonical reaching facts, not the previous analysis.
    for unit in &mut package.units {
        unit.flow_types.clear();
        unit.rust_storage_names.take();
    }
    let mut first_pass = true;
    loop {
        let mut changed = false;
        for index in 0..package.units.len() {
            let unit = &package.units[index];
            let matches = super::enums::MatchContext::new(package, unit, &enums);
            let mut bindings = Vec::new();
            let mut visible_bindings = Vec::new();
            collect_typed_bindings(
                unit,
                &matches,
                &unit.tree.root,
                &mut visible_bindings,
                &mut bindings,
                None,
            )?;
            if bindings != unit.typed_bindings {
                package.units[index].rust_storage_names.take();
                package.units[index].typed_bindings = bindings;
                changed = true;
            }
        }
        if !changed && !first_pass {
            return Ok(());
        }
        // A loop-carried RHS may discover an earlier inferred declaration.
        // Declaration identities are source-owned; their finite reaching-type
        // alternatives grow through the same joins as the loop fixed point.
        super::scope_flow::validate_definite_assignment(package)?;
        first_pass = false;
    }
}
fn populate_projected_call_result_types(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    loop {
        let mut additions = Vec::new();
        for (unit_index, unit) in package.units.iter().enumerate() {
            collect_projected_call_result_types(
                package,
                unit,
                &unit.tree.root,
                unit_index,
                &mut additions,
            )?;
        }
        let mut changed = false;
        for (unit_index, key, value_type) in additions {
            changed |= package.units[unit_index]
                .selected_expression_types
                .insert(key, value_type)
                .is_none();
        }
        if !changed {
            return Ok(());
        }
        rebuild_typed_bindings(package)?;
    }
}

fn projected_constructor_result(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    constructor: &crate::rust_interop::projection::ProjectedFunction,
) -> Result<Option<ValueType>, SemanticFailure> {
    let Some(arguments) = node.children.get(1) else {
        return Ok(None);
    };
    let mut substitutions = BTreeMap::new();
    let mut positional = 0;
    for argument in &arguments.children {
        let named = argument
            .children
            .first()
            .filter(|child| child.kind == SyntaxKind::Name && argument.children.len() > 1);
        let parameter = if let Some(name) = named {
            constructor
                .parameters
                .iter()
                .find(|parameter| parameter.name == node_text(&unit.source, name))
        } else {
            let parameter = constructor.parameters.get(positional);
            positional += 1;
            parameter
        };
        let Some(parameter) = parameter else {
            return Ok(None);
        };
        let value = argument.children.last().unwrap_or(argument);
        let Some(actual) = infer_value_type(unit, value, &unit.typed_bindings)? else {
            return Ok(None);
        };
        if actual == ValueType::Scalar(ScalarType::None) {
            continue;
        }
        let actual = destination_projected_type(package, &actual).map_err(|error| {
            failure(
                &unit.source,
                "T0129",
                format!("constructor payload cannot select a native representation: {error}"),
                value.span,
            )
        })?;
        bind_projected_native_generics(&parameter.ty, &actual, &mut substitutions).map_err(
            |reason| {
                failure(
                    &unit.source,
                    "T0129",
                    format!(
                        "constructor payloads select incompatible native representations: {reason}"
                    ),
                    value.span,
                )
            },
        )?;
    }
    if constructor
        .generic_parameters
        .iter()
        .any(|parameter| !substitutions.contains_key(&parameter.name))
    {
        return Ok(None);
    }
    let result = substitutions
        .iter()
        .fold(constructor.result.clone(), |result, (name, selected)| {
            substitute_projected_generic(&result, name, selected)
        });
    Ok(closed_projected_value_type(package, &result))
}

#[expect(
    clippy::too_many_lines,
    reason = "one recursive refinement pass keeps projected call-result correlation atomic"
)]
fn collect_projected_call_result_types(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    unit_index: usize,
    additions: &mut Vec<(usize, (u32, usize, usize), ValueType)>,
) -> Result<(), SemanticFailure> {
    for child in &node.children {
        collect_projected_call_result_types(package, unit, child, unit_index, additions)?;
    }
    if node.kind != SyntaxKind::CallExpression
        || unit.selected_expression_types.contains_key(&(
            node.span.file,
            node.span.start,
            node.span.end,
        ))
    {
        return Ok(());
    }
    let Some(callee) = node.children.first() else {
        return Ok(());
    };
    let designator = if callee.kind == SyntaxKind::AppliedType {
        callee.children.first().unwrap_or(callee)
    } else {
        callee
    };
    let projected_function =
        projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node));
    if super::namespaces::function_contract_for_call_with_safety(
        package,
        unit,
        designator,
        crate::syntax::call_is_unsafe(node),
    )
    .is_some_and(|contract| !contract.generic_parameters.is_empty())
        && !projected_function.as_ref().is_some_and(|function| {
            function.chain_role.is_some()
                || matches!(
                    function.result,
                    crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                )
        })
        && let Some(contract) = super::calls::selected_callable_contract(
            package,
            unit,
            node,
            crate::syntax::call_is_unsafe(node),
        )
        && let Some(value_type) = contract.return_type
    {
        let mut value_type = match value_type {
            ValueType::Object(identity) => ValueType::Object(
                close_written_native_nominal(package, &identity).unwrap_or(identity),
            ),
            value_type => value_type,
        };
        canonicalize_native_value_type(package, &mut value_type);
        additions.push((
            unit_index,
            (node.span.file, node.span.start, node.span.end),
            value_type,
        ));
        return Ok(());
    }
    let Some(function) = projected_function else {
        return Ok(());
    };
    if callee.kind == SyntaxKind::ConstructionExpression {
        if let Some(value_type) = projected_constructor_result(package, unit, node, function)? {
            additions.push((
                unit_index,
                (node.span.file, node.span.start, node.span.end),
                value_type,
            ));
        }
        return Ok(());
    }
    if function.chain_role == Some(crate::rust_interop::projection::ChainRole::Continue)
        && let Some(receiver) = callee.children.first()
        && let Some(ValueType::InvocationScopedNative {
            rust_type,
            concrete: true,
            family,
            lifetimes,
            expression_scoped,
            region,
        }) = infer_value_type(unit, receiver, &unit.typed_bindings)
            .ok()
            .flatten()
    {
        let replacements = node
            .children
            .get(1)
            .map(|arguments| {
                function
                    .parameters
                    .iter()
                    .zip(&arguments.children)
                    .filter_map(|(parameter, argument)| {
                        let crate::rust_interop::projection::ProjectedType::Generic(name) =
                            &parameter.ty
                        else {
                            return None;
                        };
                        let value = argument.children.last().unwrap_or(argument);
                        let value_type = infer_value_type(unit, value, &unit.typed_bindings)
                            .ok()
                            .flatten()?;
                        let projected = destination_projected_type(package, &value_type).ok()?;
                        Some((name.clone(), projected.rust_type()))
                    })
                    .collect::<BTreeMap<_, _>>()
            })
            .unwrap_or_default();
        additions.push((
            unit_index,
            (node.span.file, node.span.start, node.span.end),
            ValueType::InvocationScopedNative {
                rust_type: crate::rust_ir::instantiate_rust_generics(&rust_type, &replacements),
                concrete: true,
                family,
                lifetimes,
                expression_scoped,
                region,
            },
        ));
        return Ok(());
    }

    let owner_substitutions = projected_call_owner_substitutions(package, unit, designator)
        .map_err(|reason| failure(&unit.source, "T0118", reason, node.span))?;
    let projected_result = owner_substitutions
        .iter()
        .fold(function.result.clone(), |result, (name, projected)| {
            substitute_projected_generic(&result, name, projected)
        });
    match &projected_result {
        crate::rust_interop::projection::ProjectedType::InvocationScoped {
            rust_type,
            name,
            lifetimes,
            expression_scoped,
            owned,
        } => {
            let region = package.units[unit_index]
                .enclosing_function_spans
                .get(&node.span.start)
                .copied()
                .flatten()
                .filter(|span| {
                    package.units[unit_index].functions.iter().any(|function| {
                        function.span == *span
                            && matches!(
                                function.return_type,
                                Some(ValueType::InvocationScopedNative { .. })
                            )
                    })
                })
                .map(|span| (span.file, span.start, span.end));
            let region = (!expression_scoped).then_some(region).flatten();
            let (family_namespace, family_name) = package
                .projection
                .borrowed_scope_owner(owned)
                .unwrap_or_else(|| {
                    let namespace = callee
                        .children
                        .first()
                        .and_then(|receiver| {
                            infer_value_type(unit, receiver, &unit.typed_bindings)
                                .ok()
                                .flatten()
                        })
                        .and_then(|value| match value {
                            ValueType::Object(identity)
                            | ValueType::InvocationScopedNative {
                                family: identity, ..
                            } => Some(identity.namespace),
                            _ => None,
                        })
                        .unwrap_or_else(|| package.units[unit_index].namespace.clone());
                    (namespace, name.clone())
                });
            let mut family = closed_projected_value_type(package, owned)
                .and_then(|value_type| match value_type {
                    ValueType::Object(identity)
                        if identity.namespace == family_namespace
                            && identity.name == family_name =>
                    {
                        Some(identity)
                    }
                    _ => None,
                })
                .unwrap_or_else(|| ObjectIdentity::new(family_namespace, family_name));
            if family.native_projection.is_none() {
                family.native_projection = Some(
                    if matches!(
                        owned.as_ref(),
                        crate::rust_interop::projection::ProjectedType::Optional(_)
                    ) {
                        "std::option::Option".to_owned()
                    } else {
                        owned
                            .rust_type()
                            .split_once('<')
                            .map_or(owned.rust_type(), |(base, _)| base.to_owned())
                    },
                );
            }
            additions.push((
                unit_index,
                (node.span.file, node.span.start, node.span.end),
                ValueType::InvocationScopedNative {
                    rust_type: rust_type.clone(),
                    concrete: true,
                    family,
                    lifetimes: lifetimes.clone(),
                    expression_scoped: *expression_scoped,
                    region,
                },
            ));
        }
        _ => {
            if !projected_result.contains_open_generic()
                && let Some(value_type) = closed_projected_value_type(package, &projected_result)
            {
                additions.push((
                    unit_index,
                    (node.span.file, node.span.start, node.span.end),
                    value_type,
                ));
            }
        }
    }
    Ok(())
}

fn collect_projected_generic_names(
    projected: &crate::rust_interop::projection::ProjectedType,
    names: &mut BTreeSet<String>,
) {
    use crate::rust_interop::projection::ProjectedType;
    match projected {
        ProjectedType::Generic(name) => {
            names.insert(name.clone());
        }
        ProjectedType::Sequence { item, .. }
        | ProjectedType::Set { item, .. }
        | ProjectedType::AsyncIterationStep(item)
        | ProjectedType::Optional(item)
        | ProjectedType::Reference { inner: item, .. } => {
            collect_projected_generic_names(item, names);
        }
        ProjectedType::Mapping { key, value, .. } => {
            collect_projected_generic_names(key, names);
            collect_projected_generic_names(value, names);
        }
        ProjectedType::Tuple(items)
        | ProjectedType::Foreign {
            arguments: items, ..
        } => {
            for item in items {
                collect_projected_generic_names(item, names);
            }
        }
        ProjectedType::InvocationScoped { owned, .. } => {
            collect_projected_generic_names(owned, names);
        }
        ProjectedType::BoxedInterface {
            associated_type: Some(binding),
            ..
        } => collect_projected_generic_names(&binding.ty, names),
        ProjectedType::Callback {
            parameters, result, ..
        } => {
            for parameter in parameters {
                collect_projected_generic_names(parameter, names);
            }
            collect_projected_generic_names(result, names);
        }
        ProjectedType::None
        | ProjectedType::Associated(_)
        | ProjectedType::Opaque { .. }
        | ProjectedType::Bool
        | ProjectedType::Int
        | ProjectedType::FixedInt(_)
        | ProjectedType::RustInt(_)
        | ProjectedType::Float
        | ProjectedType::Float32
        | ProjectedType::Char
        | ProjectedType::String
        | ProjectedType::BorrowedString
        | ProjectedType::Bytes
        | ProjectedType::AsyncSinkOutcome
        | ProjectedType::BoxedInterface {
            associated_type: None,
            ..
        } => {}
    }
}

#[derive(Clone)]
struct PendingProjectedBound {
    generic: Option<String>,
    direct_rust_type: String,
    borrowed_rust_type: Option<String>,
    rust_bound: String,
    inferred_parameters: Vec<String>,
}

#[derive(Clone)]
struct PendingProjectedSpecialization {
    unit: usize,
    span: Span,
    operation_name: String,
    substitutions: BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
    generic_arguments: Vec<crate::rust_interop::projection::ProjectedType>,
    projected_result: crate::rust_interop::projection::ProjectedType,
    projected_parameters: Vec<crate::rust_interop::projection::ProjectedParameter>,
    direct_projected_call: bool,
    value_parameters: Vec<Option<ValueType>>,
    value_type: ValueType,
    bounds: Vec<PendingProjectedBound>,
}

pub(super) fn qualify_projected_rust_names(
    rust: &str,
    projected_names: &BTreeMap<String, String>,
) -> String {
    projected_names
        .iter()
        .fold(rust.to_owned(), |rust, (name, native)| {
            let mut qualified = String::with_capacity(rust.len());
            let mut consumed = 0;
            for (start, matched) in rust.match_indices(name) {
                let end = start + matched.len();
                let left = rust[..start].trim_end();
                let right = rust[end..].trim_start();
                if left.ends_with("::")
                    || right.starts_with("::")
                    || left
                        .chars()
                        .next_back()
                        .is_some_and(|character| character.is_alphanumeric() || character == '_')
                    || right
                        .chars()
                        .next()
                        .is_some_and(|character| character.is_alphanumeric() || character == '_')
                {
                    continue;
                }
                qualified.push_str(&rust[consumed..start]);
                qualified.push_str(native);
                consumed = end;
            }
            qualified.push_str(&rust[consumed..]);
            qualified
        })
}

fn specialize_projected_results(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    loop {
        let before = projected_specialization_count(package);
        let Some((unit_index, span)) = specialize_projected_result_batch(package)? else {
            return Ok(());
        };
        if projected_specialization_count(package) == before {
            return Err(failure(
                &package.units[unit_index].source,
                "T0119",
                "internal compiler error: projected result specialization made no progress",
                span,
            ));
        }
    }
}

fn projected_specialization_count(package: &SemanticPackage) -> usize {
    package
        .units
        .iter()
        .map(|unit| unit.projected_call_specializations.len())
        .sum()
}

#[expect(
    clippy::too_many_lines,
    reason = "oracle proof and specialization installation remain one atomic semantic step"
)]
fn specialize_projected_result_batch(
    package: &mut SemanticPackage,
) -> Result<Option<(usize, Span)>, SemanticFailure> {
    for unit_index in 0..package.units.len() {
        let plans = {
            let unit = &package.units[unit_index];
            resolve_callback_function_results(package, unit)
                .into_iter()
                .map(|(span, result)| ((span.file, span.start, span.end), result.value_type))
                .collect::<Vec<_>>()
        };
        package.units[unit_index]
            .invocation_scoped_function_results
            .extend(plans);
    }
    let mut pending = Vec::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        if let Err(error) = collect_projected_destinations(
            package,
            unit,
            &unit.tree.root,
            None,
            None,
            unit_index,
            &mut pending,
        ) {
            if !pending.is_empty()
                && error
                    .diagnostics
                    .iter()
                    .all(|diagnostic| diagnostic.code == "T0129")
            {
                // Prove producers first, then retry their consumers with the
                // closed binding identities. With no progress, retain the error.
                break;
            }
            return Err(error);
        }
    }
    let Some(first_pending) = pending.first() else {
        return Ok(None);
    };
    let first_pending_site = (first_pending.unit, first_pending.span);
    let projected_rust_names = pending
        .iter()
        .flat_map(|specialization| &specialization.bounds)
        .flat_map(|bound| {
            std::iter::once(&bound.direct_rust_type)
                .chain(bound.borrowed_rust_type.as_ref())
                .chain(std::iter::once(&bound.rust_bound))
        })
        .flat_map(|rust| {
            rust.split(|character: char| !character.is_alphanumeric() && character != '_')
        })
        .filter(|name| name.chars().next().is_some_and(char::is_uppercase))
        .filter_map(|name| {
            package
                .projection
                .item_named(name)
                .map(|item| (name.to_owned(), item.rust_path.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    let normalize_projected_rust_names =
        |rust: &str| qualify_projected_rust_names(rust, &projected_rust_names);
    let questions = pending
        .iter()
        .flat_map(|specialization| &specialization.bounds)
        .flat_map(|bound| {
            std::iter::once(&bound.direct_rust_type)
                .chain(bound.borrowed_rust_type.as_ref())
                .map(|rust_type| crate::rust_interop::BoundQuestion {
                    rust_type: normalize_projected_rust_names(rust_type),
                    rust_bound: normalize_projected_rust_names(&bound.rust_bound),
                    inferred_parameters: bound.inferred_parameters.clone(),
                })
        })
        .collect::<Vec<_>>();
    let workspace = package.root.join(".trn/dependencies");
    let report = crate::rust_interop::ProjectionOracle::new(
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
    let mut macro_questions = Vec::new();
    let mut macro_expansions = Vec::new();
    let mut macro_sites = Vec::new();
    for specialization in &pending {
        let unit = &package.units[specialization.unit];
        if let Some(node) = find_node_by_span(&unit.tree.root, specialization.span)
            && let Some(callee) = node.children.first()
            && projected_macro_for_call(package, unit, callee).is_some()
            && !matches!(
                specialization.value_type,
                ValueType::InvocationScopedNative { .. }
            )
        {
            let (expansion, result) =
                super::macros::macro_probe(package, unit, node, &specialization.projected_result)?;
            macro_expansions.push(expansion);
            macro_questions.push(result);
            macro_sites.push((
                specialization.unit,
                specialization.span,
                node_text(&unit.source, node).to_owned(),
            ));
        }
    }
    if !macro_questions.is_empty() {
        let oracle = crate::rust_interop::ProjectionOracle::new(
            &workspace,
            &package.projection.cache_identity,
            package.projection.containment,
        );
        let report = oracle.prove_calls(&macro_questions).map_err(|error| {
            failure(
                &package.units[macro_sites[0].0].source,
                "T0119",
                format!("native macro proof could not run: {}", error.message),
                macro_sites[0].1,
            )
        })?;
        package.projection.probe_wall_time_ms = package
            .projection
            .probe_wall_time_ms
            .saturating_add(report.wall_time_ms);
        let result_answers = report
            .evidence
            .iter()
            .map(|e| (&e.question, &e.answer))
            .collect::<BTreeMap<_, _>>();
        let rejected = macro_questions
            .iter()
            .enumerate()
            .filter_map(|(i, question)| {
                (result_answers.get(question) != Some(&&crate::rust_interop::ProbeAnswer::Yes))
                    .then_some(i)
            })
            .collect::<Vec<_>>();
        let expansion_questions = rejected
            .iter()
            .map(|&i| macro_expansions[i].clone())
            .collect::<Vec<_>>();
        let expansion_answers = if expansion_questions.is_empty() {
            BTreeMap::new()
        } else {
            let report = oracle.prove_calls(&expansion_questions).map_err(|error| {
                failure(
                    &package.units[macro_sites[rejected[0]].0].source,
                    "T0119",
                    format!("native macro proof could not run: {}", error.message),
                    macro_sites[rejected[0]].1,
                )
            })?;
            package.projection.probe_wall_time_ms = package
                .projection
                .probe_wall_time_ms
                .saturating_add(report.wall_time_ms);
            report
                .evidence
                .iter()
                .map(|e| (e.question.clone(), e.answer.clone()))
                .collect::<BTreeMap<_, _>>()
        };
        if let Some(i) = rejected.into_iter().next() {
            let (unit_index, span, invocation) = &macro_sites[i];
            let result = result_answers.get(&macro_questions[i]);
            let expansion = expansion_answers.get(&macro_expansions[i]);
            let reason = match (expansion, result) {
                (Some(crate::rust_interop::ProbeAnswer::No), _) =>
                    format!("the native macro expansion rejects the supplied argument(s) in `{invocation}`; check their types and the macro's accepted arguments"),
                (Some(crate::rust_interop::ProbeAnswer::Yes), Some(crate::rust_interop::ProbeAnswer::No)) =>
                    "the macro expansion does not produce a value compatible with the destination type".to_owned(),
                (Some(crate::rust_interop::ProbeAnswer::Unknown { reason }), _)
                    if reason.contains("no rules expected") || reason.contains("unexpected end of macro invocation") =>
                    format!("supplied argument(s) in `{invocation}` do not match the native macro's accepted token pattern"),
                (Some(crate::rust_interop::ProbeAnswer::Unknown { .. }), _) =>
                    format!("the supplied argument(s) in `{invocation}` could not be checked against the native macro"),
                (_, Some(crate::rust_interop::ProbeAnswer::Unknown { .. })) =>
                    "the macro expansion could not be checked against the destination type".to_owned(),
                _ => "native macro proof returned no answer".to_owned(),
            };
            return Err(failure(
                &package.units[*unit_index].source,
                "T0119",
                format!(
                    "native macro invocation does not satisfy its concrete expression contract: {reason}"
                ),
                *span,
            ));
        }
    }
    for mut specialization in pending {
        let mut generic_groups = Vec::new();
        for bound in &specialization.bounds {
            if !generic_groups.contains(&bound.generic) {
                generic_groups.push(bound.generic.clone());
            }
        }
        for generic in generic_groups {
            let bounds = specialization
                .bounds
                .iter()
                .filter(|bound| bound.generic == generic)
                .collect::<Vec<_>>();
            let candidate_works = |borrowed: bool| {
                bounds.iter().all(|bound| {
                    let rust_type = if borrowed {
                        let Some(rust_type) = &bound.borrowed_rust_type else {
                            return false;
                        };
                        rust_type
                    } else {
                        &bound.direct_rust_type
                    };
                    answers.get(&crate::rust_interop::BoundQuestion {
                        rust_type: normalize_projected_rust_names(rust_type),
                        rust_bound: normalize_projected_rust_names(&bound.rust_bound),
                        inferred_parameters: bound.inferred_parameters.clone(),
                    }) == Some(&crate::rust_interop::ProbeAnswer::Yes)
                })
            };
            if candidate_works(false) {
                continue;
            }
            if generic.is_some() && candidate_works(true) {
                for parameter in &mut specialization.projected_parameters {
                    if parameter.generic_parameter.as_ref() == generic.as_ref() {
                        parameter.borrowed = true;
                        parameter.mutable_borrow = bounds.iter().any(|bound| {
                            bound
                                .borrowed_rust_type
                                .as_deref()
                                .is_some_and(|rust_type| rust_type.starts_with("&mut "))
                        });
                    }
                }
                continue;
            }
            let bound = bounds
                .iter()
                .find(|bound| {
                    answers.get(&crate::rust_interop::BoundQuestion {
                        rust_type: normalize_projected_rust_names(&bound.direct_rust_type),
                        rust_bound: normalize_projected_rust_names(&bound.rust_bound),
                        inferred_parameters: bound.inferred_parameters.clone(),
                    }) != Some(&crate::rust_interop::ProbeAnswer::Yes)
                })
                .copied()
                .expect("a failed direct candidate has a failed bound");
            let question = crate::rust_interop::BoundQuestion {
                rust_type: normalize_projected_rust_names(&bound.direct_rust_type),
                rust_bound: normalize_projected_rust_names(&bound.rust_bound),
                inferred_parameters: bound.inferred_parameters.clone(),
            };
            let unit = &package.units[specialization.unit];
            let call_name = &specialization.operation_name;
            let native_name = crate::rust_ir::rust_type_constructor(&bound.direct_rust_type)
                .and_then(|path| path.rsplit("::").next().map(str::to_owned))
                .unwrap_or_else(|| bound.direct_rust_type.clone());
            let subject = generic.as_ref().map_or_else(
                || format!("projected result destination `{}`", specialization.value_type),
                |_| {
                    format!(
                        "input to projected operation `{call_name}` with native type `{native_name}`"
                    )
                },
            );
            let displayed_bound = crate::rust_ir::format_rust_bound(&bound.rust_bound);
            match answers.get(&question) {
                Some(crate::rust_interop::ProbeAnswer::Unknown { reason }) => {
                    return Err(failure(
                        &unit.source,
                        "T0119",
                        format!(
                            "{subject} could not be proven against `{displayed_bound}`: {}",
                            oracle_diagnostic_summary(reason)
                        ),
                        specialization.span,
                    ));
                }
                Some(
                    crate::rust_interop::ProbeAnswer::No | crate::rust_interop::ProbeAnswer::Yes,
                ) => {
                    return Err(failure(
                        &unit.source,
                        "T0119",
                        format!("{subject} does not satisfy `{displayed_bound}`"),
                        specialization.span,
                    ));
                }
                None => {
                    return Err(failure(
                        &unit.source,
                        "T0119",
                        format!(
                            "projection oracle returned no answer for {subject} against \
                             `{displayed_bound}`"
                        ),
                        specialization.span,
                    ));
                }
            }
        }
        for parameter in &mut specialization.projected_parameters {
            if !parameter.borrowed {
                parameter.generic_parameter = None;
            }
        }
        canonicalize_native_value_type(package, &mut specialization.value_type);
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
                    substitutions: specialization.substitutions,
                    generic_arguments: specialization.generic_arguments,
                    projected_result: specialization.projected_result,
                    value_type: specialization.value_type,
                    projected_parameters: specialization.projected_parameters,
                    direct_projected_call: specialization.direct_projected_call,
                    value_parameters: specialization.value_parameters,
                },
            );
    }
    super::capabilities::populate_native_capabilities(package)?;
    rebuild_typed_bindings(package)?;
    Ok(Some(first_pending_site))
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

fn merge_projected_callback_shape(
    template: &crate::rust_interop::projection::ProjectedType,
    actual: &crate::rust_interop::projection::ProjectedType,
) -> crate::rust_interop::projection::ProjectedType {
    use crate::rust_interop::projection::ProjectedType;
    let (
        ProjectedType::Callback {
            rust_name,
            native_bound,
            native_method,
            native_result,
            native_substitutions,
            parameters: template_parameters,
            result: template_result,
            parameters_destination_selected,
            parameter_rust_types,
            retained,
            send,
            sync,
            ..
        },
        ProjectedType::Callback {
            parameters,
            parameter_borrows,
            result,
            is_async,
            invocation_mode,
            ..
        },
    ) = (template, actual)
    else {
        return actual.clone();
    };
    let mut parameter_replacements = template_parameters
        .iter()
        .zip(parameters)
        .filter_map(|(template, actual)| {
            let ProjectedType::Generic(name) = template else {
                return None;
            };
            Some((name.clone(), actual.rust_type()))
        })
        .collect::<BTreeMap<_, _>>();
    for (native, actual) in parameter_rust_types.iter().zip(parameters) {
        let mut generic = native.trim();
        if let Some(rest) = generic.strip_prefix('&') {
            generic = rest.trim_start();
            if generic.starts_with('\'') {
                generic = generic
                    .split_once(char::is_whitespace)
                    .map_or(generic, |(_, rest)| rest.trim_start());
            }
            generic = generic.strip_prefix("mut ").unwrap_or(generic);
        }
        if generic
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
            && generic.chars().next().is_some_and(char::is_uppercase)
        {
            parameter_replacements
                .entry(generic.to_owned())
                .or_insert_with(|| actual.rust_type());
        }
    }
    ProjectedType::Callback {
        rust_name: rust_name.clone(),
        native_bound: native_bound
            .as_ref()
            .map(|bound| crate::rust_ir::instantiate_rust_generics(bound, &parameter_replacements)),
        native_method: native_method.clone(),
        native_result: native_result.as_ref().map(|result| {
            crate::rust_ir::instantiate_rust_generics(result, &parameter_replacements)
        }),
        native_substitutions: native_substitutions.clone(),
        parameters_destination_selected: *parameters_destination_selected,
        parameters: parameters.clone(),
        parameter_borrows: parameter_borrows.clone(),
        parameter_rust_types: parameter_rust_types
            .iter()
            .map(|rust_type| {
                crate::rust_ir::instantiate_rust_generics(rust_type, &parameter_replacements)
            })
            .collect(),
        result: if matches!(
            template_result.as_ref(),
            ProjectedType::InvocationScoped { .. }
        ) {
            template_result.clone()
        } else {
            result.clone()
        },
        invocation_mode: *invocation_mode,
        is_async: *is_async,
        retained: *retained,
        send: *send,
        sync: *sync,
    }
}

fn merge_projected_callback_value_shape(
    expected: &ValueType,
    actual: &ValueType,
    projected_result: Option<&crate::rust_interop::projection::ProjectedType>,
    namespace: &str,
    region: Option<(u32, usize, usize)>,
) -> ValueType {
    let result = match projected_result {
        Some(crate::rust_interop::projection::ProjectedType::InvocationScoped {
            rust_type,
            name,
            lifetimes,
            expression_scoped,
            ..
        }) => Some(ElementType::new(ValueType::InvocationScopedNative {
            rust_type: rust_type.clone(),
            concrete: true,
            family: ObjectIdentity::new(namespace.to_owned(), name.clone()),
            lifetimes: lifetimes.clone(),
            expression_scoped: *expression_scoped,
            region,
        })),
        _ => None,
    };
    match (expected, actual) {
        (
            ValueType::Function(_, _, expected_effects),
            ValueType::Function(actual_parameters, actual_result, _),
        ) => ValueType::Function(
            actual_parameters.clone(),
            result.unwrap_or_else(|| actual_result.clone()),
            expected_effects.clone(),
        ),
        (
            ValueType::AsyncFunction(_, _, expected_transferability, expected_effects),
            ValueType::AsyncFunction(actual_parameters, actual_result, _, _),
        ) => ValueType::AsyncFunction(
            actual_parameters.clone(),
            result.unwrap_or_else(|| actual_result.clone()),
            *expected_transferability,
            expected_effects.clone(),
        ),
        _ => actual.clone(),
    }
}

fn bind_projected_callback_generics(
    package: &SemanticPackage,
    expected: &ValueType,
    actual: &ValueType,
    actual_projected_result: Option<&crate::rust_interop::projection::ProjectedType>,
    bindings: &mut BTreeMap<String, ValueType>,
    projected_bindings: &mut BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
    allow_destination_selected_parameters: bool,
) -> Result<(), String> {
    let (
        ValueType::Function(expected_parameters, expected_result, _)
        | ValueType::AsyncFunction(expected_parameters, expected_result, _, _),
        ValueType::Function(actual_parameters, actual_result, _)
        | ValueType::AsyncFunction(actual_parameters, actual_result, _, _),
    ) = (expected, actual)
    else {
        return super::calls::bind_projected_generics(expected, actual, bindings);
    };
    if expected_parameters.len() == actual_parameters.len() {
        for (expected, actual) in expected_parameters.iter().zip(actual_parameters) {
            let expected = expected.value_type_ref();
            let actual = actual.value_type_ref();
            let actual = match (expected, actual) {
                (
                    ValueType::ProjectedGeneric(_),
                    ValueType::Reference(inner) | ValueType::SharedReference(inner),
                ) => inner.value_type_ref(),
                _ => actual,
            };
            super::calls::bind_projected_generics(expected, actual, bindings)?;
        }
    } else if !allow_destination_selected_parameters {
        return super::calls::bind_projected_generics(expected, actual, bindings);
    }
    if let Some(actual_projected_result) = actual_projected_result {
        let expected_projected_result =
            destination_projected_type(package, expected_result.value_type_ref())?;
        return bind_projected_native_generics(
            &expected_projected_result,
            actual_projected_result,
            projected_bindings,
        );
    }
    super::calls::bind_projected_generics(
        expected_result.value_type_ref(),
        actual_result.value_type_ref(),
        bindings,
    )
}

fn contextual_scoped_replacements(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    template: &str,
) -> BTreeMap<String, String> {
    fn collect(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        template_arguments: &[String],
        replacements: &mut BTreeMap<String, String>,
    ) {
        if let Some(ValueType::InvocationScopedNative { rust_type, .. }) =
            infer_value_type(unit, node, &unit.typed_bindings)
                .ok()
                .flatten()
        {
            let actual_arguments = crate::rust_ir::rust_type_arguments(&rust_type);
            if actual_arguments.len() == template_arguments.len() {
                for (template, actual) in template_arguments.iter().zip(actual_arguments) {
                    if template
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
                        && template != &actual
                    {
                        replacements.entry(template.clone()).or_insert(actual);
                    }
                }
            }
        }
        for child in &node.children {
            collect(unit, child, template_arguments, replacements);
        }
    }
    let template_arguments = crate::rust_ir::rust_type_arguments(template);
    let mut replacements = BTreeMap::new();
    for child in &node.children {
        collect(unit, child, &template_arguments, &mut replacements);
    }
    replacements
}
fn projected_call_result<'a>(
    package: &SemanticPackage,
    unit: &'a SemanticUnit,
    mut node: &'a SyntaxNode,
) -> Option<crate::rust_interop::projection::ProjectedType> {
    let mut visited = BTreeSet::new();
    loop {
        if node.kind == SyntaxKind::GroupExpression {
            node = node.children.first()?;
        } else if node.kind == SyntaxKind::Name {
            if let Some(value_type) = infer_value_type(unit, node, &unit.typed_bindings)
                .ok()
                .flatten()
                && let Ok(projected) = destination_projected_type(package, &value_type)
            {
                return Some(projected);
            }
            let binding = unit.typed_bindings.iter().rev().find(|binding| {
                binding.name == node_text(&unit.source, node)
                    && binding.is_visible_at(unit.source.id(), node.span.start)
            })?;
            if !visited.insert((binding.span.file, binding.span.start, binding.span.end)) {
                return None;
            }
            node = find_binding_initializer(&unit.tree.root, binding.span)?;
        } else {
            break;
        }
    }
    if let Some(value_type @ ValueType::InvocationScopedNative { .. }) = unit
        .selected_expression_types
        .get(&(node.span.file, node.span.start, node.span.end))
        && let Ok(mut projected) = destination_projected_type(package, value_type)
    {
        if let crate::rust_interop::projection::ProjectedType::InvocationScoped {
            rust_type, ..
        } = &mut projected
        {
            let replacements = contextual_scoped_replacements(unit, node, rust_type);
            *rust_type = crate::rust_ir::instantiate_rust_generics(rust_type, &replacements);
        }
        return Some(projected);
    }
    if let Some(specialization) =
        unit.projected_call_specializations
            .get(&(node.span.file, node.span.start, node.span.end))
    {
        return Some(specialization.projected_result.clone());
    }
    let [callee, _] = node.children.as_slice() else {
        return None;
    };
    projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
        .map(|function| function.result.clone())
}

#[derive(Clone)]
struct CallbackFunctionResult {
    projected: crate::rust_interop::projection::ProjectedType,
    value_type: ValueType,
}

fn callback_expression_result(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Option<CallbackFunctionResult> {
    let mut node = node;
    while node.kind == SyntaxKind::GroupExpression {
        node = node.children.first()?;
    }
    if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && callee.kind == SyntaxKind::Name
        && unit.functions.iter().any(|contract| {
            contract.name == node_text(&unit.source, callee)
                && matches!(
                    contract.return_type,
                    Some(ValueType::InvocationScopedNative {
                        concrete: false,
                        ..
                    })
                )
        })
    {
        // A call within the same unresolved scoped-function graph contributes no new concrete
        // producer. Concrete return sites in that graph establish the converged result.
        return None;
    }
    let value_type = unit
        .selected_expression_types
        .get(&(node.span.file, node.span.start, node.span.end))
        .cloned()
        .or_else(|| {
            infer_value_type(unit, node, &unit.typed_bindings)
                .ok()
                .flatten()
        })?;
    let projected = projected_call_result(package, unit, node)
        .or_else(|| destination_projected_type(package, &value_type).ok())?;
    Some(CallbackFunctionResult {
        projected,
        value_type,
    })
}

fn forwarded_callback_function_span(
    unit: &SemanticUnit,
    value: &SyntaxNode,
    visited_bindings: &mut BTreeSet<(u32, usize, usize)>,
) -> Option<Span> {
    let mut value = value;
    while value.kind == SyntaxKind::GroupExpression {
        value = value.children.first()?;
    }
    if value.kind == SyntaxKind::CallExpression
        && let Some(callee) = value.children.first()
        && callee.kind == SyntaxKind::Name
        && let Some(contract) =
            resolved_function_contract(unit, node_text(&unit.source, callee), callee.span.start)
        && matches!(
            contract.return_type,
            Some(ValueType::InvocationScopedNative {
                concrete: false,
                ..
            })
        )
        && find_node_by_span(&unit.tree.root, contract.span)
            .is_some_and(|node| node.kind == SyntaxKind::FunctionDeclaration)
    {
        return Some(contract.span);
    }
    if value.kind != SyntaxKind::Name {
        return None;
    }
    let binding = unit
        .typed_bindings
        .iter()
        .filter(|binding| {
            binding.name == node_text(&unit.source, value)
                && binding.is_visible_at(unit.source.id(), value.span.start)
        })
        .max_by_key(|binding| binding.visible_from)?;
    let key = (binding.span.file, binding.span.start, binding.span.end);
    if !visited_bindings.insert(key) {
        return None;
    }
    let declaration = find_node_by_span(&unit.tree.root, binding.span)?;
    let initializer = declaration.children.last()?;
    forwarded_callback_function_span(unit, initializer, visited_bindings)
}

#[derive(Clone)]
struct CallbackFunctionFacts {
    span: Span,
    direct_results: Vec<CallbackFunctionResult>,
    forwarded_returns: Vec<Span>,
}

fn collect_callback_function_facts(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    facts: &mut CallbackFunctionFacts,
) {
    if node.kind == SyntaxKind::ReturnStatement
        && let Some(value) = node.children.first()
    {
        if let Some(span) = forwarded_callback_function_span(unit, value, &mut BTreeSet::new()) {
            facts.forwarded_returns.push(span);
        } else if let Some(result) = callback_expression_result(package, unit, value) {
            facts.direct_results.push(result);
        }
        return;
    }
    for child in &node.children {
        collect_callback_function_facts(package, unit, child, facts);
    }
}

fn anonymous_callback_node<'a>(
    unit: &'a SemanticUnit,
    value: &'a SyntaxNode,
) -> Option<&'a SyntaxNode> {
    if value.kind == SyntaxKind::AnonymousFunction {
        return Some(value);
    }
    if value.kind != SyntaxKind::Name {
        return None;
    }
    let name = node_text(&unit.source, value);
    let binding = unit
        .typed_bindings
        .iter()
        .filter(|binding| {
            binding.name == name && binding.is_visible_at(unit.source.id(), value.span.start)
        })
        .max_by_key(|binding| binding.visible_from)?;
    let binding = find_node_by_span(&unit.tree.root, binding.span)?;
    binding
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::AnonymousFunction)
}

pub(super) fn same_projected_native_family(
    left: &crate::rust_interop::projection::ProjectedType,
    right: &crate::rust_interop::projection::ProjectedType,
) -> bool {
    match (left, right) {
        (
            crate::rust_interop::projection::ProjectedType::InvocationScoped {
                name: left_name,
                rust_type: left_rust,
                ..
            },
            crate::rust_interop::projection::ProjectedType::InvocationScoped {
                name: right_name,
                rust_type: right_rust,
                ..
            },
        ) => {
            left_name == right_name
                && crate::rust_ir::rust_type_constructors_match(left_rust, right_rust)
        }
        _ => left == right,
    }
}

fn contextual_results(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    results: &mut BTreeMap<(u32, usize, usize), Vec<CallbackFunctionResult>>,
) {
    use crate::rust_interop::projection::ProjectedType;
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && let Some(function) =
            projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
    {
        for (parameter, argument) in function.parameters.iter().zip(&arguments.children) {
            let ProjectedType::Callback {
                result,
                native_result,
                ..
            } = &parameter.ty
            else {
                continue;
            };
            let ProjectedType::InvocationScoped {
                name,
                owned,
                lifetimes,
                rust_type,
                ..
            } = result.as_ref()
            else {
                continue;
            };
            if native_result.as_ref().is_some_and(|native| {
                !crate::rust_ir::rust_type_constructors_match(native, rust_type)
            }) {
                // A trait adapter's terminal result is not necessarily its
                // callback producer contract; retain that producer's identity.
                continue;
            }
            let native = native_result.as_ref().unwrap_or(rust_type);
            let value = argument.children.last().unwrap_or(argument);
            if value.kind != SyntaxKind::Name {
                continue;
            }
            let Some(contract) =
                resolved_function_contract(unit, node_text(&unit.source, value), value.span.start)
            else {
                continue;
            };
            let base = owned.rust_type();
            let value_type = ValueType::InvocationScopedNative {
                rust_type: native.clone(),
                concrete: true,
                family: ObjectIdentity::new(unit.namespace.clone(), name.clone())
                    .with_native_projection(
                        base.split_once('<')
                            .map_or(base.clone(), |(base, _)| base.to_owned()),
                    ),
                lifetimes: lifetimes.clone(),
                expression_scoped: false,
                region: None,
            };
            let mut projected = result.as_ref().clone();
            if let ProjectedType::InvocationScoped { rust_type, .. } = &mut projected {
                rust_type.clone_from(native);
            }
            results
                .entry((contract.span.file, contract.span.start, contract.span.end))
                .or_default()
                .push(CallbackFunctionResult {
                    projected,
                    value_type,
                });
        }
    }
    for child in &node.children {
        contextual_results(package, unit, child, results);
    }
}
fn resolve_callback_function_results(
    package: &SemanticPackage,
    unit: &SemanticUnit,
) -> Vec<(Span, CallbackFunctionResult)> {
    let mut contexts = BTreeMap::new();
    contextual_results(package, unit, &unit.tree.root, &mut contexts);
    let mut facts = unit
        .functions
        .iter()
        .filter(|contract| {
            matches!(
                contract.return_type,
                Some(ValueType::InvocationScopedNative {
                    concrete: false,
                    ..
                })
            )
        })
        .filter_map(|contract| {
            let function = find_node_by_span(&unit.tree.root, contract.span)?;
            let mut facts = CallbackFunctionFacts {
                span: contract.span,
                direct_results: Vec::new(),
                forwarded_returns: Vec::new(),
            };
            collect_callback_function_facts(package, unit, function, &mut facts);
            // A scoped function's consumer selects its result, even when the
            // producer is a different native builder closed through Into.
            if let Some(context) =
                contexts.get(&(contract.span.file, contract.span.start, contract.span.end))
            {
                facts.direct_results.clone_from(context);
            }
            for result in &mut facts.direct_results {
                contextualize_callback_value_type(unit, &mut result.value_type, &[contract.span]);
            }
            Some(facts)
        })
        .collect::<Vec<_>>();
    facts.sort_by_key(|facts| (facts.span.file, facts.span.start, facts.span.end));
    let mut candidates = facts
        .iter()
        .map(|facts| {
            facts.direct_results.iter().fold(
                Vec::<CallbackFunctionResult>::new(),
                |mut results, result| {
                    if !results.iter().any(|candidate| {
                        same_projected_native_family(&candidate.projected, &result.projected)
                    }) {
                        results.push(result.clone());
                    }
                    results
                },
            )
        })
        .collect::<Vec<_>>();
    loop {
        let previous = candidates.clone();
        for (index, function) in facts.iter().enumerate() {
            for forwarded in &function.forwarded_returns {
                let Some(callee) = facts.iter().position(|facts| facts.span == *forwarded) else {
                    continue;
                };
                for result in &previous[callee] {
                    if !candidates[index].iter().any(|candidate| {
                        same_projected_native_family(&candidate.projected, &result.projected)
                    }) {
                        candidates[index].push(result.clone());
                    }
                }
            }
        }
        if candidates
            .iter()
            .zip(&previous)
            .all(|(current, previous)| current.len() == previous.len())
        {
            break;
        }
    }
    facts
        .into_iter()
        .zip(candidates)
        .filter_map(|(facts, mut candidates)| {
            (candidates.len() == 1).then(|| (facts.span, candidates.remove(0)))
        })
        .collect()
}

fn projected_callback_function_result(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    value: &SyntaxNode,
) -> Option<CallbackFunctionResult> {
    let name = node_text(&unit.source, value);
    let span = resolved_function_contract(unit, name, value.span.start)?.span;
    let value_type = unit
        .invocation_scoped_function_results
        .get(&(span.file, span.start, span.end))?
        .clone();
    let projected = destination_projected_type(package, &value_type).ok()?;
    Some(CallbackFunctionResult {
        projected,
        value_type,
    })
}

fn contextualize_callback_value_type(
    unit: &SemanticUnit,
    value_type: &mut ValueType,
    callback_spans: &[Span],
) {
    fn collect(
        unit: &SemanticUnit,
        node: &SyntaxNode,
        template_arguments: &[String],
        replacements: &mut BTreeMap<String, String>,
    ) {
        if let Ok(Some(ValueType::InvocationScopedNative {
            rust_type: actual, ..
        })) = infer_value_type(unit, node, &unit.typed_bindings)
        {
            let actual_arguments = crate::rust_ir::rust_type_arguments(&actual);
            if actual_arguments.len() == template_arguments.len() {
                for (template, actual) in template_arguments.iter().zip(actual_arguments) {
                    if template
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
                        && template != &actual
                    {
                        replacements.entry(template.clone()).or_insert(actual);
                    }
                }
            }
        }
        for child in &node.children {
            collect(unit, child, template_arguments, replacements);
        }
    }

    let ValueType::InvocationScopedNative { rust_type, .. } = value_type else {
        return;
    };
    let template_arguments = crate::rust_ir::rust_type_arguments(rust_type);
    let mut replacements = BTreeMap::new();
    for span in callback_spans {
        if let Some(function) = find_node_by_span(&unit.tree.root, *span) {
            collect(unit, function, &template_arguments, &mut replacements);
        }
    }
    *rust_type = crate::rust_ir::instantiate_rust_generics(rust_type, &replacements);
}

fn substitute_native_type_parameters(
    value_type: &mut ValueType,
    bindings: &BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
) {
    let ValueType::InvocationScopedNative { rust_type, .. } = value_type else {
        return;
    };
    let replacements = bindings
        .iter()
        .map(|(name, projected)| (name.clone(), projected.rust_type()))
        .collect();
    *rust_type = crate::rust_ir::instantiate_rust_generics(rust_type, &replacements);
}

fn bind_projected_native_generics(
    expected: &crate::rust_interop::projection::ProjectedType,
    actual: &crate::rust_interop::projection::ProjectedType,
    bindings: &mut BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
) -> Result<(), String> {
    use crate::rust_interop::projection::ProjectedType;
    if let ProjectedType::Generic(name) = expected {
        if let Some(previous) = bindings.get(name) {
            return projected_types_share_concrete_rust_representation(previous, actual)
                .then_some(())
                .ok_or_else(|| name.clone());
        }
        bindings.insert(name.clone(), actual.clone());
        return Ok(());
    }
    match (expected, actual) {
        (
            ProjectedType::Reference {
                inner: expected,
                mutable: expected_mutable,
                ..
            },
            ProjectedType::Reference {
                inner: actual,
                mutable: actual_mutable,
                ..
            },
        ) if expected_mutable == actual_mutable => {
            bind_projected_native_generics(expected, actual, bindings)
        }
        (
            ProjectedType::Foreign {
                base_rust_path: expected_path,
                arguments: expected_arguments,
                ..
            },
            ProjectedType::Foreign {
                base_rust_path: actual_path,
                arguments: actual_arguments,
                ..
            },
        ) if expected_path == actual_path && expected_arguments.len() == actual_arguments.len() => {
            for (expected, actual) in expected_arguments.iter().zip(actual_arguments) {
                bind_projected_native_generics(expected, actual, bindings)?;
            }
            Ok(())
        }
        (
            ProjectedType::InvocationScoped {
                name: expected_name,
                owned: expected_owned,
                ..
            },
            ProjectedType::InvocationScoped {
                name: actual_name,
                owned: actual_owned,
                ..
            },
        ) if expected_name == actual_name => {
            bind_projected_native_generics(expected_owned, actual_owned, bindings)
        }
        (ProjectedType::Optional(expected), ProjectedType::Optional(actual))
        | (
            ProjectedType::Sequence { item: expected, .. },
            ProjectedType::Sequence { item: actual, .. },
        )
        | (ProjectedType::Set { item: expected, .. }, ProjectedType::Set { item: actual, .. })
        | (
            ProjectedType::AsyncIterationStep(expected),
            ProjectedType::AsyncIterationStep(actual),
        ) => bind_projected_native_generics(expected, actual, bindings),
        (ProjectedType::Optional(expected), actual) if actual != &ProjectedType::None => {
            bind_projected_native_generics(expected, actual, bindings)
        }
        (ProjectedType::Tuple(expected), ProjectedType::Tuple(actual))
            if expected.len() == actual.len() =>
        {
            for (expected, actual) in expected.iter().zip(actual) {
                bind_projected_native_generics(expected, actual, bindings)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

fn rust_type_text_equal(left: &str, right: &str) -> bool {
    left.chars()
        .filter(|character| !character.is_whitespace())
        .eq(right.chars().filter(|character| !character.is_whitespace()))
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
    fn collect_inputs(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        value: &SyntaxNode,
        function_result: Option<&ValueType>,
        unit_index: usize,
        pending: &mut Vec<PendingProjectedSpecialization>,
    ) -> Result<(), SemanticFailure> {
        if value.kind == SyntaxKind::CallExpression
            && let Some(callee) = value.children.first()
            && super::bindings::projected_macro_for_call(package, unit, callee).is_some()
        {
            for argument in value
                .children
                .get(1)
                .into_iter()
                .flat_map(|arguments| &arguments.children)
            {
                collect_inputs(
                    package,
                    unit,
                    argument.children.last().unwrap_or(argument),
                    function_result,
                    unit_index,
                    pending,
                )?;
            }
            return Ok(());
        }
        if value.kind == SyntaxKind::GroupExpression {
            for child in &value.children {
                collect_inputs(package, unit, child, function_result, unit_index, pending)?;
            }
            return Ok(());
        }
        collect_projected_destinations(
            package,
            unit,
            value,
            None,
            function_result,
            unit_index,
            pending,
        )
    }
    if let Some(specialization) =
        unit.projected_call_specializations
            .get(&(node.span.file, node.span.start, node.span.end))
    {
        if node.kind == SyntaxKind::CallExpression
            && let [callee, arguments] = node.children.as_slice()
        {
            if projected_macro_for_call(package, unit, callee).is_some() {
                for argument in &arguments.children {
                    collect_inputs(
                        package,
                        unit,
                        argument.children.last().unwrap_or(argument),
                        function_return,
                        unit_index,
                        pending,
                    )?;
                }
            } else {
                for (index, argument) in arguments.children.iter().enumerate() {
                    collect_projected_destinations(
                        package,
                        unit,
                        argument.children.last().unwrap_or(argument),
                        specialization
                            .value_parameters
                            .get(index)
                            .and_then(Option::as_ref),
                        function_return,
                        unit_index,
                        pending,
                    )?;
                }
            }
            return Ok(());
        }
        for child in &node.children {
            collect_projected_destinations(
                package,
                unit,
                child,
                None,
                function_return,
                unit_index,
                pending,
            )?;
        }
        return Ok(());
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && let Some(item) = projected_macro_for_call(package, unit, callee)
    {
        let destination = expected.ok_or_else(|| {
            failure(
                &unit.source,
                "T0119",
                "native macro invocation requires a concrete result destination",
                node.span,
            )
        })?;
        let projected_result =
            destination_projected_type(package, destination).map_err(|error| {
                failure(
                    &unit.source,
                    "T0119",
                    format!("native macro result cannot be projected: {error}"),
                    node.span,
                )
            })?;
        let projected_parameters = arguments
            .children
            .iter()
            .enumerate()
            .map(|(index, argument)| {
                super::macros::macro_argument(
                    package,
                    unit,
                    argument.children.last().unwrap_or(argument),
                    index,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        pending.push(PendingProjectedSpecialization {
            unit: unit_index,
            span: node.span,
            operation_name: item.name.clone(),
            substitutions: BTreeMap::new(),
            generic_arguments: Vec::new(),
            projected_result,
            value_type: destination.clone(),
            value_parameters: vec![None; projected_parameters.len()],
            projected_parameters,
            direct_projected_call: true,
            bounds: Vec::new(),
        });
        let input_start = pending.len();
        for argument in &arguments.children {
            collect_inputs(
                package,
                unit,
                argument.children.last().unwrap_or(argument),
                function_return,
                unit_index,
                pending,
            )?;
        }
        if matches!(
            destination,
            ValueType::InvocationScopedNative { concrete: true, .. }
        ) {
            // The native callback closes producer lifetimes jointly with the macro.
            // A detached higher-ranked assertion would demand an unrelated lifetime.
            for producer in &mut pending[input_start..] {
                producer.bounds.clear();
                if producer.generic_arguments.iter().any(|argument| {
                    matches!(
                        argument,
                        crate::rust_interop::projection::ProjectedType::Generic(_)
                    )
                }) {
                    producer.generic_arguments.clear();
                }
            }
        }
        return Ok(());
    }
    if node.kind == SyntaxKind::CallExpression
        && let [callee, arguments] = node.children.as_slice()
        && let Some(function) =
            projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
        && (function
            .generic_parameters
            .iter()
            .any(|generic| generic.input_selected)
            || function.parameters.iter().any(|parameter| {
                matches!(
                    parameter.ty,
                    crate::rust_interop::projection::ProjectedType::Callback {
                        native_result: Some(_),
                        ..
                    }
                )
            }))
        && (function.chain_role.is_none()
            || function.parameters.iter().any(|parameter| {
                parameter.generic_parameter.is_some()
                    && matches!(
                        parameter.ty,
                        crate::rust_interop::projection::ProjectedType::Callback { .. }
                    )
            }))
        && let Some(contract) = super::namespaces::function_contract_for_call_with_safety(
            package,
            unit,
            callee,
            crate::syntax::call_is_unsafe(node),
        )
    {
        let mut function = (*function).clone();
        let owner_substitutions = projected_call_owner_substitutions(package, unit, callee)
            .map_err(|reason| failure(&unit.source, "T0129", reason, callee.span))?;
        let specialize_owner = |template: &crate::rust_interop::projection::ProjectedType| {
            owner_substitutions.iter().fold(
                template.clone(),
                |specialized, (generic, projected)| {
                    substitute_projected_generic(&specialized, generic, projected)
                },
            )
        };
        let owner_rust_replacements = owner_substitutions
            .iter()
            .map(|(name, projected)| (name.clone(), projected.rust_type()))
            .collect::<BTreeMap<_, _>>();
        function.result = specialize_owner(&function.result);
        function.rust_generic_arguments = function
            .rust_generic_arguments
            .iter()
            .map(&specialize_owner)
            .collect();
        for parameter in &mut function.parameters {
            parameter.ty = specialize_owner(&parameter.ty);
            parameter.generic_bounds = parameter
                .generic_bounds
                .iter()
                .map(|bound| {
                    crate::rust_ir::instantiate_rust_generics(bound, &owner_rust_replacements)
                })
                .collect();
            if let Some(associated_type) = &mut parameter.associated_type {
                *associated_type.ty = specialize_owner(&associated_type.ty);
            }
        }
        for generic in &mut function.generic_parameters {
            generic.rust_bounds = generic
                .rust_bounds
                .iter()
                .map(|bound| {
                    crate::rust_ir::instantiate_rust_generics(bound, &owner_rust_replacements)
                })
                .collect();
            generic.default = generic.default.as_ref().map(&specialize_owner);
        }
        if let Some(destination_result) = &mut function.destination_result {
            destination_result.bound_roots = destination_result
                .bound_roots
                .iter()
                .map(|bound| {
                    crate::rust_ir::instantiate_rust_generics(bound, &owner_rust_replacements)
                })
                .collect();
            for parameter in &mut destination_result.parameters {
                parameter.rust_bounds = parameter
                    .rust_bounds
                    .iter()
                    .map(|bound| {
                        crate::rust_ir::instantiate_rust_generics(bound, &owner_rust_replacements)
                    })
                    .collect();
            }
        }
        let mut value_bindings = BTreeMap::new();
        let mut callback_bindings = BTreeMap::new();
        let mut callback_value_bindings = BTreeMap::new();
        let mut native_bindings = BTreeMap::new();
        let mut native_value_bindings = BTreeMap::new();
        let mut native_projected_parameters = BTreeMap::new();
        for (parameter_index, (parameter, projected_parameter)) in contract
            .parameters
            .iter()
            .zip(&function.parameters)
            .enumerate()
        {
            let argument = arguments
                .children
                .iter()
                .find(|argument| {
                    argument.children.first().is_some_and(|name| {
                        name.kind == SyntaxKind::Name
                            && argument.children.len() > 1
                            && node_text(&unit.source, name) == parameter.name
                    })
                })
                .or_else(|| {
                    arguments
                        .children
                        .iter()
                        .filter(|argument| {
                            !argument.children.first().is_some_and(|name| {
                                name.kind == SyntaxKind::Name && argument.children.len() > 1
                            })
                        })
                        .nth(parameter_index)
                });
            let Some(argument) = argument else {
                continue;
            };
            let value = argument.children.last().unwrap_or(argument);
            let actual_projected = projected_call_result(package, unit, value);
            if let Some(actual_projected) = actual_projected.as_ref() {
                native_projected_parameters.insert(parameter_index, actual_projected.clone());
                bind_projected_native_generics(
                    &projected_parameter.ty,
                    actual_projected,
                    &mut native_bindings,
                )
                .map_err(|generic| {
                    failure(
                        &unit.source,
                        "T0129",
                        format!(
                            "projected generic `{generic}` is inferred as incompatible native argument types"
                        ),
                        value.span,
                    )
                })?;
            }
            let Some(actual) = infer_value_type(unit, value, &unit.typed_bindings)? else {
                continue;
            };
            if actual_projected.is_some() {
                native_value_bindings.insert(parameter_index, actual.clone());
            }
            let Some(expected) = parameter.element_value_type() else {
                continue;
            };
            let actual_invocation_scoped_callback = match &actual {
                ValueType::Function(_, result, _) | ValueType::AsyncFunction(_, result, _, _) => {
                    matches!(
                        result.value_type_ref(),
                        ValueType::InvocationScopedNative { .. }
                    )
                }
                _ => false,
            };
            if let (
                crate::rust_interop::projection::ProjectedType::Callback {
                    parameters_destination_selected,
                    native_result: Some(_),
                    native_substitutions: callback_native_substitutions,
                    result: projected_callback_result,
                    ..
                },
                _,
                ValueType::Function(..) | ValueType::AsyncFunction(..),
            ) = (
                &projected_parameter.ty,
                projected_parameter.generic_parameter.as_ref(),
                &actual,
            ) && actual_invocation_scoped_callback
            {
                if anonymous_callback_node(unit, value).is_some() {
                    return Err(failure(
                        &unit.source,
                        "T0129",
                        "anonymous callbacks are not yet supported for exact invocation-scoped results",
                        value.span,
                    ));
                }
                let destination_selected_parameters = *parameters_destination_selected;
                let Some(actual_callback_result) =
                    projected_callback_function_result(package, unit, value)
                else {
                    return Err(failure(
                        &unit.source,
                        "T0129",
                        "projected callback returns do not converge on one native type",
                        value.span,
                    ));
                };
                let actual_projected_result = Some(actual_callback_result.projected.clone());
                let mut callback_value_type = actual_callback_result.value_type;
                substitute_native_type_parameters(
                    &mut callback_value_type,
                    callback_native_substitutions,
                );
                if let (
                    crate::rust_interop::projection::ProjectedType::InvocationScoped {
                        name: expected_name,
                        ..
                    },
                    Some(crate::rust_interop::projection::ProjectedType::InvocationScoped {
                        name: actual_name,
                        ..
                    }),
                ) = (
                    projected_callback_result.as_ref(),
                    actual_projected_result.as_ref(),
                ) && expected_name != actual_name
                {
                    return Err(failure(
                        &unit.source,
                        "T0129",
                        format!(
                            "projected callback has an incompatible native producer type: expected `{expected_name}`, actual `{actual_name}`"
                        ),
                        value.span,
                    ));
                }
                bind_projected_native_generics(
                    projected_callback_result,
                    actual_projected_result
                        .as_ref()
                        .expect("callback convergence was checked above"),
                    &mut native_bindings,
                )
                .map_err(|reason| {
                    failure(
                        &unit.source,
                        "T0129",
                        format!(
                            "projected callback has an incompatible native producer type: {reason}"
                        ),
                        value.span,
                    )
                })?;
                bind_projected_callback_generics(
                    package,
                    &expected,
                    &actual,
                    None,
                    &mut value_bindings,
                    &mut native_bindings,
                    destination_selected_parameters,
                )
                .map_err(|generic| {
                    failure(
                        &unit.source,
                        "T0129",
                        format!(
                            "projected generic `{generic}` is inferred as incompatible callback types: expected `{expected}`, actual `{actual}`"
                        ),
                        value.span,
                    )
                })?;
                let mut callback = match &actual {
                    ValueType::Function(parameters, result, _) => destination_projected_callback(
                        package,
                        parameters,
                        result,
                        false,
                        true,
                        actual_projected_result.clone(),
                    ),
                    ValueType::AsyncFunction(parameters, result, transferability, _) => {
                        destination_projected_callback(
                            package,
                            parameters,
                            result,
                            true,
                            *transferability != TaskTransferability::Local,
                            actual_projected_result.clone(),
                        )
                    }
                    _ => unreachable!("callback pattern checked above"),
                }
                .map_err(|reason| {
                    failure(
                        &unit.source,
                        "T0129",
                        format!("projected callback cannot use `{actual}`: {reason}"),
                        value.span,
                    )
                })?;
                if let (
                    crate::rust_interop::projection::ProjectedType::Callback {
                        native_bound: template_bound,
                        native_method: template_method,
                        native_result: template_result,
                        native_substitutions: template_substitutions,
                        ..
                    },
                    crate::rust_interop::projection::ProjectedType::Callback {
                        native_bound,
                        native_method,
                        native_result,
                        native_substitutions,
                        ..
                    },
                ) = (&projected_parameter.ty, &mut callback)
                {
                    native_bound.clone_from(template_bound);
                    native_method.clone_from(template_method);
                    native_result.clone_from(template_result);
                    native_substitutions.clone_from(template_substitutions);
                }
                callback_bindings.insert(parameter_index, callback);
                let callback_region = unit
                    .functions
                    .iter()
                    .find(|contract| contract.name == node_text(&unit.source, value))
                    .map(|contract| (contract.span.file, contract.span.start, contract.span.end));
                callback_value_bindings.insert(
                    parameter_index,
                    merge_projected_callback_value_shape(
                        &expected,
                        &actual,
                        actual_projected_result.as_ref(),
                        &unit.namespace,
                        callback_region,
                    ),
                );
                continue;
            }
            super::calls::bind_projected_generics(
                &expected,
                &actual,
                &mut value_bindings,
            )
            .map_err(|generic| {
                failure(
                    &unit.source,
                    "T0129",
                    format!(
                        "projected generic `{generic}` is inferred as incompatible argument types: expected `{expected}`, actual `{actual}`"
                    ),
                    value.span,
                )
            })?;
        }
        let explicit_projected = explicit_projected_callable_arguments(callee)
            .map(|types| {
                let result_names = function
                    .destination_result
                    .as_ref()
                    .into_iter()
                    .flat_map(|result| &result.parameters)
                    .map(|parameter| parameter.name.as_str())
                    .collect::<BTreeSet<_>>();
                let binders = function
                    .generic_parameters
                    .iter()
                    .filter(|generic| generic.input_selected || result_names.contains(generic.name.as_str()))
                    .collect::<Vec<_>>();
                if types.len() != binders.len() {
                    return Err(failure(
                        &unit.source,
                        "T0129",
                        "explicit native type arguments must select every input- or result-selected generic parameter",
                        callee.span,
                    ));
                }
                binders
                    .into_iter()
                    .zip(types)
                    .map(|(generic, ty)| {
                        let value_type = super::types::declared_value_type(
                            unit,
                            ty,
                            &visible_descriptor_aliases(
                                &unit.descriptor_aliases,
                                ty.span.file,
                                ty.span.start,
                            ),
                        )
                        .map_err(|_| {
                            failure(
                                &unit.source,
                                "T0129",
                                format!("explicit type for projected generic `{}` is not available", generic.name),
                                ty.span,
                            )
                        })?;
                        let projected = destination_projected_type(package, &value_type).map_err(|reason| {
                            failure(
                                &unit.source,
                                "T0129",
                                format!("explicit type for projected generic `{}` cannot be projected: {reason}", generic.name),
                                ty.span,
                            )
                        })?;
                        Ok((generic.name.clone(), projected))
                    })
                    .collect::<Result<BTreeMap<_, _>, SemanticFailure>>()
            })
            .transpose()?
            .unwrap_or_default();
        let mut projected_bindings = BTreeMap::new();
        for generic in function
            .generic_parameters
            .iter()
            .filter(|generic| generic.input_selected)
        {
            let inferred = native_bindings.get(&generic.name).cloned().or_else(|| {
                value_bindings.get(&generic.name).and_then(|actual| {
                    let (actual_projection, reference) = match actual {
                        ValueType::Reference(inner) => (inner.value_type_ref(), Some(true)),
                        ValueType::SharedReference(inner) => (inner.value_type_ref(), Some(false)),
                        actual => (actual, None),
                    };
                    destination_projected_type(package, actual_projection)
                        .ok()
                        .map(|projected| {
                            reference.map_or(projected.clone(), |mutable| {
                                projected_reference_type(projected, mutable)
                            })
                        })
                })
            });
            let projected = if let Some(explicit) = explicit_projected.get(&generic.name) {
                if let Some(inferred) = inferred
                    && !projected_types_share_concrete_rust_representation(&inferred, explicit)
                {
                    return Err(failure(
                        &unit.source,
                        "T0129",
                        format!(
                            "explicit native type `{}` for `{}` conflicts with the selected input type `{}`",
                            explicit.terrane_name(),
                            generic.name,
                            inferred.terrane_name()
                        ),
                        node.span,
                    ));
                }
                explicit.clone()
            } else if let Some(inferred) = inferred {
                inferred
            } else if matches!(
                &function.result,
                crate::rust_interop::projection::ProjectedType::InvocationScoped { rust_type, .. }
                    if rust_type.split(|character: char| !character.is_ascii_alphanumeric() && character != '_').any(|token| token == generic.name)
            ) && function.parameters.iter().any(|parameter| matches!(
                &parameter.ty,
                crate::rust_interop::projection::ProjectedType::Sequence { item, .. }
                    if matches!(item.as_ref(), crate::rust_interop::projection::ProjectedType::InvocationScoped { .. })
            )) {
                continue;
            } else {
                return Err(failure(
                    &unit.source,
                    "T0129",
                    format!(
                        "projected generic `{}` cannot be inferred from this call",
                        generic.name
                    ),
                    node.span,
                ));
            };
            projected_bindings.insert(generic.name.clone(), projected);
        }
        for (name, projected) in &explicit_projected {
            projected_bindings
                .entry(name.clone())
                .or_insert_with(|| projected.clone());
        }
        let specialize = |template: &crate::rust_interop::projection::ProjectedType| {
            projected_bindings
                .iter()
                .fold(template.clone(), |specialized, (generic, projected)| {
                    substitute_projected_generic(&specialized, generic, projected)
                })
        };
        let mut projected_parameters: Vec<crate::rust_interop::projection::ProjectedParameter> =
            function
                .parameters
                .iter()
                .cloned()
                .enumerate()
                .map(|(index, mut parameter)| {
                    if let crate::rust_interop::projection::ProjectedType::Generic(name) =
                        &parameter.ty
                    {
                        parameter.generic_parameter = Some(name.clone());
                    }
                    parameter.ty = if let Some(actual) = callback_bindings.get(&index) {
                        specialize(&merge_projected_callback_shape(&parameter.ty, actual))
                    } else if let Some(actual) = native_projected_parameters.get(&index) {
                        if matches!(
                            parameter.ty,
                            crate::rust_interop::projection::ProjectedType::Sequence { .. }
                        ) {
                            specialize(&parameter.ty)
                        } else if matches!(
                            parameter.ty,
                            crate::rust_interop::projection::ProjectedType::Callback { .. }
                        ) {
                            specialize(&merge_projected_callback_shape(&parameter.ty, actual))
                        } else {
                            actual.clone()
                        }
                    } else {
                        specialize(&parameter.ty)
                    };
                    parameter.generic_bounds.clear();
                    parameter
                })
                .collect();
        if function.native_owner.as_deref() == Some("std::option::Option")
            && let Some(receiver) = callee.children.first()
            && let Some(rust_type) = {
                let mut receiver = receiver;
                while receiver.kind == SyntaxKind::GroupExpression
                    && let [inner] = receiver.children.as_slice()
                {
                    receiver = inner;
                }
                projected_function_for_call(
                    package,
                    unit,
                    receiver,
                    crate::syntax::call_is_unsafe(receiver),
                )
                .map(|function| function.result.rust_type())
                .or_else(|| {
                    match infer_value_type(unit, receiver, &unit.typed_bindings)
                        .ok()
                        .flatten()
                    {
                        Some(ValueType::InvocationScopedNative { rust_type, .. }) => {
                            Some(rust_type)
                        }
                        _ => None,
                    }
                })
            }
            && let Ok(syn::Type::Path(receiver_type)) = syn::parse_str::<syn::Type>(&rust_type)
            && let Some(segment) = receiver_type.path.segments.last()
            && let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments
            && let Some(syn::GenericArgument::Type(syn::Type::Reference(referent))) =
                arguments.args.first()
            && referent.mutability.is_some()
        {
            for parameter in &mut projected_parameters {
                if let crate::rust_interop::projection::ProjectedType::Callback {
                    parameters,
                    parameter_rust_types,
                    native_bound,
                    result,
                    ..
                } = &mut parameter.ty
                {
                    *parameter_rust_types = parameters
                        .iter()
                        .map(|parameter| {
                            projected_reference_type(parameter.clone(), true).rust_type()
                        })
                        .collect();
                    *native_bound = Some(format!(
                        "FnOnce({}) -> {}",
                        parameter_rust_types.join(", "),
                        result.rust_type(),
                    ));
                }
            }
        }
        let value_parameters = contract
            .parameters
            .iter()
            .enumerate()
            .map(|(index, parameter)| {
                callback_value_bindings
                    .get(&index)
                    .cloned()
                    .or_else(|| native_value_bindings.get(&index).cloned())
                    .or_else(|| {
                        let crate::rust_interop::projection::ProjectedType::Sequence {
                            item, ..
                        } = &projected_parameters[index].ty
                        else {
                            return None;
                        };
                        let crate::rust_interop::projection::ProjectedType::InvocationScoped {
                            name,
                            lifetimes,
                            ..
                        } = item.as_ref()
                        else {
                            return None;
                        };
                        Some(ValueType::List(ElementType::new(
                            ValueType::InvocationScopedNative {
                                rust_type: "host-invocation-scoped-native".to_owned(),
                                concrete: false,
                                family: ObjectIdentity::new("/deps", name.clone()),
                                lifetimes: lifetimes.clone(),
                                expression_scoped: false,
                                region: None,
                            },
                        )))
                    })
                    .or_else(|| {
                        parameter.element_value_type().map(|value_type| {
                            super::calls::substitute_projected_value_generics(
                                &value_type,
                                &value_bindings,
                            )
                        })
                    })
            })
            .collect();
        let mut projected_result = specialize(&function.result);
        let result = if callee.kind == SyntaxKind::ConstructionExpression {
            closed_projected_value_type(package, &projected_result).ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0129",
                    "constructor payloads do not determine a concrete native representation",
                    node.span,
                )
            })?
        } else if let crate::rust_interop::projection::ProjectedType::Generic(name) =
            &function.result
            && let Some(bound) = value_bindings.get(name)
        {
            bound.clone()
        } else if let Some(result) = closed_projected_value_type(package, &projected_result) {
            result
        } else {
            super::calls::substitute_projected_value_generics(
                contract
                    .return_type
                    .as_ref()
                    .unwrap_or(&ValueType::Scalar(ScalarType::None)),
                &value_bindings,
            )
        };
        // Specializations cache the completed payload; expression inference adds
        // the async task wrapper using the selected callable's transferability.
        let mut value_type = result;
        let mut rust_replacements = projected_bindings
            .iter()
            .map(|(name, projected)| (name.clone(), projected.rust_type()))
            .collect::<BTreeMap<_, _>>();
        let mut specialization_substitutions = projected_bindings.clone();
        if let (Some(destination_result), Some(destination)) =
            (&function.destination_result, expected)
            && let Ok(expected_projected) = destination_projected_type(package, destination)
        {
            let all_parameters = destination_result
                .parameters
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect::<BTreeSet<_>>();
            if let Ok(Some(destination_choices)) = select_projected_generic_destinations(
                &projected_result,
                &all_parameters,
                &expected_projected,
            ) && destination_choices.iter().any(|(name, selected)| {
                projected_bindings.get(name).is_some_and(|explicit| {
                    !projected_types_share_concrete_rust_representation(explicit, selected)
                })
            }) {
                return Err(failure(
                    &unit.source,
                    "T0129",
                    "explicit native type arguments conflict with the written result destination",
                    node.span,
                ));
            }
            let parameters = destination_result
                .parameters
                .iter()
                .filter(|parameter| !projected_bindings.contains_key(&parameter.name))
                .map(|parameter| parameter.name.clone())
                .collect::<BTreeSet<_>>();
            if !parameters.is_empty()
                && let Ok(Some(substitutions)) = select_projected_generic_destinations(
                    &projected_result,
                    &parameters,
                    &expected_projected,
                )
            {
                for (name, projected) in &substitutions {
                    rust_replacements.insert(name.clone(), projected.rust_type());
                    for parameter in &mut projected_parameters {
                        parameter.ty = substitute_projected_generic(&parameter.ty, name, projected);
                    }
                }
                projected_result = align_projected_result_representation(
                    &substitutions
                        .iter()
                        .fold(projected_result, |result, (name, projected)| {
                            substitute_projected_generic(&result, name, projected)
                        }),
                    &expected_projected,
                );
                specialization_substitutions.extend(substitutions);
                value_type = destination.clone();
            }
        }
        let deferred_native_generics = function
            .parameters
            .iter()
            .enumerate()
            .filter_map(|(index, parameter)| {
                let generic = parameter.generic_parameter.as_ref()?;
                let projected = native_projected_parameters.get(&index)?;
                let mut open = BTreeSet::new();
                collect_projected_generic_names(projected, &mut open);
                (!open.is_empty()).then(|| generic.clone())
            })
            .collect::<BTreeSet<_>>();
        let mut bounds = function
            .generic_parameters
            .iter()
            .filter(|generic| generic.input_selected)
            .filter(|generic| !deferred_native_generics.contains(&generic.name))
            .filter(|generic| projected_bindings.contains_key(&generic.name))
            .flat_map(|generic| {
                let projected = &projected_bindings[&generic.name];
                if matches!(
                    projected,
                    crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                ) {
                    return Vec::new();
                }
                let rust_type = projected.rust_type();
                let borrowed_rust_type = match projected {
                    crate::rust_interop::projection::ProjectedType::String => {
                        Some("&str".to_owned())
                    }
                    crate::rust_interop::projection::ProjectedType::Bytes => {
                        Some("&[u8]".to_owned())
                    }
                    crate::rust_interop::projection::ProjectedType::Foreign { .. } => {
                        Some(format!("&{rust_type}"))
                    }
                    _ => None,
                };
                generic
                    .rust_bounds
                    .iter()
                    .filter_map(|bound| {
                        let rust_bound =
                            crate::rust_ir::instantiate_rust_generics(bound, &rust_replacements);
                        if let crate::rust_interop::projection::ProjectedType::Opaque {
                            bounds, ..
                        } = projected
                            && bounds
                                .iter()
                                .any(|bound| rust_type_text_equal(bound, &rust_bound))
                        {
                            return None;
                        }
                        let mut inferred_parameters = function
                            .generic_parameters
                            .iter()
                            .filter(|candidate| !rust_replacements.contains_key(&candidate.name))
                            .filter(|candidate| {
                                rust_bound
                                    .split(|character: char| {
                                        !character.is_ascii_alphanumeric() && character != '_'
                                    })
                                    .any(|token| token == candidate.name)
                            })
                            .map(|candidate| candidate.name.clone())
                            .collect::<BTreeSet<_>>();
                        collect_projected_generic_names(projected, &mut inferred_parameters);
                        let inferred_declarations = inferred_parameters
                            .iter()
                            .map(|name| {
                                let Some(parameter) = function
                                    .generic_parameters
                                    .iter()
                                    .find(|parameter| parameter.name == *name)
                                else {
                                    return name.clone();
                                };
                                if parameter.rust_bounds.is_empty() {
                                    name.clone()
                                } else {
                                    format!(
                                        "{name}: {}",
                                        parameter
                                            .rust_bounds
                                            .iter()
                                            .map(|bound| crate::rust_ir::instantiate_rust_generics(
                                                bound,
                                                &rust_replacements,
                                            ))
                                            .collect::<Vec<_>>()
                                            .join(" + ")
                                    )
                                }
                            })
                            .collect();
                        Some(PendingProjectedBound {
                            generic: Some(generic.name.clone()),
                            direct_rust_type: rust_type.clone(),
                            borrowed_rust_type: borrowed_rust_type.clone(),
                            rust_bound,
                            inferred_parameters: inferred_declarations,
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        if let Some(destination_result) = &function.destination_result {
            for parameter in &destination_result.parameters {
                let Some(projected) = specialization_substitutions.get(&parameter.name) else {
                    continue;
                };
                bounds.extend(
                    parameter
                        .rust_bounds
                        .iter()
                        .map(|bound| PendingProjectedBound {
                            generic: None,
                            direct_rust_type: projected.rust_type(),
                            borrowed_rust_type: None,
                            rust_bound: crate::rust_ir::instantiate_rust_generics(
                                bound,
                                &rust_replacements,
                            ),
                            inferred_parameters: Vec::new(),
                        }),
                );
            }
        }
        let emit_generic_arguments = !function.rust_generic_arguments.is_empty()
            && (matches!(
                function.result,
                crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
            ) || function.parameters.iter().any(|parameter| {
                matches!(
                    &parameter.ty,
                    crate::rust_interop::projection::ProjectedType::Callback {
                        result,
                        native_result,
                        ..
                    } if matches!(
                        result.as_ref(),
                        crate::rust_interop::projection::ProjectedType::InvocationScoped { .. }
                    ) || native_result.is_some()
                )
            }));
        let generic_arguments = if emit_generic_arguments {
            function
                .rust_generic_arguments
                .iter()
                .map(specialize)
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        for (argument, selected) in function
            .rust_generic_arguments
            .iter()
            .zip(&generic_arguments)
        {
            if let crate::rust_interop::projection::ProjectedType::Generic(name) = argument {
                specialization_substitutions.insert(name.clone(), selected.clone());
            }
        }
        pending.push(PendingProjectedSpecialization {
            unit: unit_index,
            span: node.span,
            operation_name: function.name.clone(),
            generic_arguments,
            substitutions: specialization_substitutions,
            projected_result,
            projected_parameters,
            direct_projected_call: true,
            value_parameters,
            value_type,
            bounds,
        });
    }
    if node.kind == SyntaxKind::CallExpression
        && let Some(callee) = node.children.first()
        && let Some(function) =
            projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(node))
        && let Some(destination_result) = &function.destination_result
        && !function
            .generic_parameters
            .iter()
            .any(|generic| generic.input_selected)
    {
        let destination = expected.cloned();
        let expected_projected = destination
            .as_ref()
            .map(|destination| {
                destination_projected_type(package, destination).map_err(|reason| {
                    failure(
                        &unit.source,
                        "T0118",
                        format!("projected result destination `{destination}` is not supported: {reason}"),
                        node.span,
                    )
                })
            })
            .transpose()?;
        let explicit = explicit_projected_callable_arguments(callee)
            .map(|types| {
                let selected_names = destination_result
                    .parameters
                    .iter()
                    .map(|parameter| parameter.name.as_str())
                    .collect::<BTreeSet<_>>();
                let binders = function
                    .generic_parameters
                    .iter()
                    .filter(|generic| selected_names.contains(generic.name.as_str()))
                    .collect::<Vec<_>>();
                if types.len() != binders.len() {
                    return Err(failure(
                        &unit.source,
                        "T0129",
                        "explicit native type arguments must select every projected result parameter",
                        callee.span,
                    ));
                }
                binders
                    .into_iter()
                    .zip(types)
                    .map(|(generic, ty)| {
                        let value = super::types::declared_value_type(
                            unit,
                            ty,
                            &visible_descriptor_aliases(
                                &unit.descriptor_aliases,
                                ty.span.file,
                                ty.span.start,
                            ),
                        )
                        .map_err(|_| failure(&unit.source, "T0129", "explicit native type argument is not available", ty.span))?;
                        destination_projected_type(package, &value)
                            .map(|projected| (generic.name.clone(), projected))
                            .map_err(|reason| failure(&unit.source, "T0129", format!("explicit native type argument cannot be projected: {reason}"), ty.span))
                    })
                    .collect::<Result<BTreeMap<_, _>, SemanticFailure>>()
            })
            .transpose()?
            .unwrap_or_default();
        let parameters = destination_result
            .parameters
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect::<BTreeSet<_>>();
        let explicit_result = if explicit.is_empty() {
            None
        } else {
            Some(
                destination_result
                    .parameters
                    .iter()
                    .map(|parameter| {
                        explicit.get(&parameter.name).cloned().map(|selected| (parameter.name.clone(), selected))
                    })
                    .collect::<Option<BTreeMap<_, _>>>()
                    .ok_or_else(|| failure(
                        &unit.source,
                        "T0129",
                        "explicit native type arguments do not select every projected result parameter",
                        callee.span,
                    ))?,
            )
        };
        let owner_substitutions = projected_call_owner_substitutions(package, unit, callee)
            .map_err(|reason| failure(&unit.source, "T0118", reason, callee.span))?;
        let result_template = owner_substitutions
            .iter()
            .fold(function.result.clone(), |result, (name, projected)| {
                substitute_projected_generic(&result, name, projected)
            });
        let destination_selection = expected_projected
            .as_ref()
            .map(|expected_projected| {
                select_projected_generic_destinations(
                    &result_template,
                    &parameters,
                    expected_projected,
                )
                .map_err(|parameter| failure(&unit.source, "T0117", format!("projected result template repeats its generic parameter with incompatible destination types (`{parameter}`)"), node.span))?
                .ok_or_else(|| failure(&unit.source, "T0117", format!("projected result template `{}` cannot produce the required destination `{}`", result_template.terrane_name(), destination.as_ref().expect("expected destination")), node.span))
            })
            .transpose()?;
        if let (Some(explicit), Some(destination_selection)) =
            (&explicit_result, &destination_selection)
            && (explicit.len() != destination_selection.len()
                || explicit.iter().any(|(name, selected)| {
                    destination_selection.get(name).is_none_or(|destination| {
                        !projected_types_share_concrete_rust_representation(selected, destination)
                    })
                }))
        {
            return Err(failure(
                &unit.source,
                "T0129",
                "explicit native type arguments conflict with the written result destination",
                node.span,
            ));
        }
        let selected_result = explicit_result.or(destination_selection).ok_or_else(|| {
            failure(
                &unit.source,
                "T0117",
                "projected generic result requires an explicit type argument or one explicit destination type",
                node.span,
            )
        })?;
        let mut substitutions = owner_substitutions;
        substitutions.extend(selected_result);
        let rust_replacements = substitutions
            .iter()
            .map(|(name, projected)| (name.clone(), projected.rust_type()))
            .collect::<BTreeMap<_, _>>();
        let projected_result = substitutions
            .iter()
            .fold(result_template.clone(), |result, (name, projected)| {
                substitute_projected_generic(&result, name, projected)
            });
        let projected_result = expected_projected.as_ref().map_or_else(
            || projected_result.clone(),
            |expected| align_projected_result_representation(&projected_result, expected),
        );
        let destination_arguments = destination_result
            .parameters
            .iter()
            .map(|parameter| substitutions.get(&parameter.name).cloned())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0119",
                    "projected result specialization did not resolve every correlated parameter",
                    node.span,
                )
            })?;
        let projected_parameters = function
            .parameters
            .iter()
            .cloned()
            .map(|mut parameter| {
                parameter.ty = substitutions
                    .iter()
                    .fold(parameter.ty, |ty, (name, projected)| {
                        substitute_projected_generic(&ty, name, projected)
                    });
                parameter
            })
            .collect();
        let bounds = destination_result
            .parameters
            .iter()
            .zip(&destination_arguments)
            .flat_map(|(parameter, projected)| {
                parameter
                    .rust_bounds
                    .iter()
                    .map(|bound| PendingProjectedBound {
                        generic: None,
                        direct_rust_type: projected.rust_type(),
                        borrowed_rust_type: None,
                        rust_bound: crate::rust_ir::instantiate_rust_generics(
                            bound,
                            &rust_replacements,
                        ),
                        inferred_parameters: Vec::new(),
                    })
            })
            .collect();
        let generic_arguments = if function.rust_generic_arguments.is_empty() {
            destination_arguments
        } else {
            function
                .rust_generic_arguments
                .iter()
                .map(|argument| {
                    substitutions
                        .iter()
                        .fold(argument.clone(), |ty, (name, projected)| {
                            substitute_projected_generic(&ty, name, projected)
                        })
                })
                .collect()
        };
        let value_type = match &projected_result {
            crate::rust_interop::projection::ProjectedType::InvocationScoped {
                rust_type,
                name,
                lifetimes,
                expression_scoped: true,
                owned,
            } => {
                let namespace = package
                    .projection
                    .borrowed_scope_owner(owned)
                    .map(|(namespace, _)| namespace)
                    .or_else(|| {
                        callee
                            .children
                            .first()
                            .and_then(|receiver| {
                                infer_value_type(unit, receiver, &unit.typed_bindings)
                                    .ok()
                                    .flatten()
                            })
                            .and_then(|value| match value {
                                ValueType::Object(identity)
                                | ValueType::InvocationScopedNative {
                                    family: identity, ..
                                } => Some(identity.namespace),
                                _ => None,
                            })
                    })
                    .unwrap_or_else(|| unit.namespace.clone());
                ValueType::InvocationScopedNative {
                    rust_type: rust_type.clone(),
                    concrete: true,
                    family: ObjectIdentity::new(namespace, name.clone()),
                    lifetimes: lifetimes.clone(),
                    expression_scoped: true,
                    region: None,
                }
            }
            _ => destination.clone().or_else(|| closed_projected_value_type(package, &projected_result)).ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0129",
                    "explicit native type arguments do not close the projected result representation",
                    node.span,
                )
            })?,
        };
        pending.push(PendingProjectedSpecialization {
            unit: unit_index,
            span: node.span,
            operation_name: function.name.clone(),
            generic_arguments,
            substitutions,
            projected_result,
            value_type,
            value_parameters: function.parameters.iter().map(|_| None).collect(),
            bounds,
            projected_parameters,
            direct_projected_call: true,
        });
    }

    if is_function_node(node) {
        let return_type = unit
            .invocation_scoped_function_results
            .get(&(node.span.file, node.span.start, node.span.end))
            .cloned()
            .or_else(|| {
                unit.functions
                    .iter()
                    .find(|contract| contract.span == node.span)
                    .and_then(|contract| contract.return_type.clone())
            });
        let graph_start = pending.len();
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
        if matches!(
            return_type,
            Some(ValueType::InvocationScopedNative { concrete: true, .. })
        ) {
            // The complete callback, including ordinary builders around macros,
            // closes its native loans in the final backend check.
            for producer in &mut pending[graph_start..] {
                producer.bounds.clear();
            }
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
        if let Some(receiver) = callee.children.first()
            && let Some(selected) = pending
                .iter()
                .rev()
                .find(|selected| selected.span == node.span)
            && let Some(destination) =
                borrowed_receiver_destination(package, unit, receiver, &selected.substitutions)
        {
            collect_projected_destinations(
                package,
                unit,
                receiver,
                Some(&destination),
                function_return,
                unit_index,
                pending,
            )?;
        } else {
            collect_projected_destinations(
                package,
                unit,
                callee,
                None,
                function_return,
                unit_index,
                pending,
            )?;
        }
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
                parameter_types
                    .get(index)
                    .or_else(|| {
                        parameter_types
                            .last()
                            .filter(|parameter| parameter.is_variadic())
                    })
                    .map(CallableParameterType::value_type_ref),
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

type DestinationProjectionError = std::borrow::Cow<'static, str>;

fn borrowed_receiver_destination(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    mut receiver: &SyntaxNode,
    substitutions: &BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
) -> Option<ValueType> {
    while receiver.kind == SyntaxKind::GroupExpression {
        receiver = receiver.children.first()?;
    }
    let producer = receiver;
    let function = projected_function_for_call(
        package,
        unit,
        producer.children.first()?,
        crate::syntax::call_is_unsafe(producer),
    )?;
    let crate::rust_interop::projection::ProjectedType::InvocationScoped { owned, .. } =
        &function.result
    else {
        return None;
    };
    let projected = substitutions
        .iter()
        .fold(owned.as_ref().clone(), |result, (name, selected)| {
            substitute_projected_generic(&result, name, selected)
        });
    closed_projected_value_type(package, &projected)
}

fn explicit_projected_callable_arguments(callee: &SyntaxNode) -> Option<Vec<&SyntaxNode>> {
    let mut callee = callee;
    while matches!(
        callee.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression
    ) {
        callee = callee.children.first()?;
    }
    if callee.kind != SyntaxKind::AppliedType {
        return None;
    }
    let (_, arguments) = callee.children.split_first()?;
    Some(arguments.iter().collect())
}

fn close_projected_foreign_generic_arguments(
    package: &SemanticPackage,
    generic_parameters: &[crate::rust_interop::projection::ProjectedGenericParameter],
    arguments: &[crate::rust_interop::projection::ProjectedType],
) -> Option<(Vec<ValueType>, BTreeMap<String, ValueType>)> {
    use crate::rust_interop::projection::ProjectedType;
    if arguments.len() > generic_parameters.len() {
        return None;
    }
    let mut selections = BTreeMap::new();
    let mut substitutions = BTreeMap::<String, ProjectedType>::new();
    for (index, parameter) in generic_parameters.iter().enumerate() {
        let argument = arguments
            .get(index)
            .or(parameter.default.as_ref())
            .map(|argument| {
                substitutions
                    .iter()
                    .fold(argument.clone(), |ty, (name, value)| {
                        substitute_projected_generic(&ty, name, value)
                    })
            })?;
        let selected = closed_projected_native_argument(package, &argument)?;
        substitutions.insert(parameter.name.clone(), argument);
        selections.insert(parameter.name.clone(), selected);
    }
    let language_arguments = generic_parameters
        .iter()
        .map(|parameter| selections[&parameter.name].clone())
        .collect();
    Some((language_arguments, selections))
}
fn closed_projected_foreign_value_type(
    package: &SemanticPackage,
    projected: &crate::rust_interop::projection::ProjectedType,
) -> Option<ValueType> {
    use crate::rust_interop::projection::{ProjectedKind, ProjectedType};
    let ProjectedType::Foreign {
        rust_path,
        arguments,
        ..
    } = projected
    else {
        return None;
    };
    let (namespace, name) = package.projection.owner_for_projected_type(projected)?;
    let item = package.projection.item(&namespace, &name)?;
    let mut identity = ObjectIdentity::new(namespace, name);
    let source_abi = super::native_constructors::source_abi_nominal(package, &identity);
    if source_abi {
        identity = super::native_constructors::canonical_source_nominal(package, &identity);
    }
    if let ProjectedKind::Interface(interface) = &item.kind
        && interface.associated_type.is_some()
    {
        if arguments.len() != 1 {
            return None;
        }
        let argument = closed_projected_value_type_with_context(package, &arguments[0], false)?;
        identity = identity.with_application(argument);
    } else if let Some(generic_parameters) = match &item.kind {
        ProjectedKind::Enum {
            generic_parameters, ..
        }
        | ProjectedKind::ForeignType {
            generic_parameters, ..
        } => Some(generic_parameters),
        _ => None,
    } && !generic_parameters.is_empty()
    {
        if arguments.len() > generic_parameters.len() {
            return None;
        }
        let (language_arguments, selections) =
            close_projected_foreign_generic_arguments(package, generic_parameters, arguments)?;
        identity = identity
            .with_type_arguments(language_arguments)
            .with_native_parameters(
                generic_parameters
                    .iter()
                    .map(|parameter| parameter.name.clone())
                    .collect(),
            )
            .with_native_arguments(selections);
    } else if let Some(constructor) = package
        .projection
        .projected_constructor(&identity.namespace, &identity.name)
        && let ProjectedType::Foreign {
            arguments: templates,
            ..
        } = &constructor.result
    {
        let selections = templates
            .iter()
            .zip(arguments)
            .filter_map(|(template, actual)| {
                let ProjectedType::Generic(name) = template else {
                    return None;
                };
                Some(Some((
                    name.clone(),
                    closed_projected_native_argument(package, actual)?,
                )))
            })
            .collect::<Option<BTreeMap<_, _>>>()?;
        identity = identity.with_native_arguments(selections);
    }
    if !identity.native_arguments.is_empty() {
        let native = destination_projected_object(package, &identity)
            .ok()?
            .rust_type();
        identity = identity.with_native_projection(native);
    } else if !source_abi && !arguments.is_empty() {
        identity = identity.with_native_projection(
            package
                .projection
                .canonical_native_type(rust_path)
                .into_owned(),
        );
    }
    if !source_abi
        && identity.native_projection.is_none()
        && let Some(selected) = close_written_native_nominal(package, &identity)
    {
        identity = selected;
    }
    if !source_abi
        && let Some(native_alias) = package.projection.canonical_native_alias(projected)
        && identity.native_projection.is_none()
    {
        identity = identity.with_native_projection(native_alias.rust_type());
    }
    Some(ValueType::Object(identity))
}

pub(super) fn closed_projected_value_type(
    package: &SemanticPackage,
    projected: &crate::rust_interop::projection::ProjectedType,
) -> Option<ValueType> {
    closed_projected_value_type_with_context(package, projected, false)
}

fn closed_projected_native_argument(
    package: &SemanticPackage,
    projected: &crate::rust_interop::projection::ProjectedType,
) -> Option<ValueType> {
    closed_projected_value_type_with_context(package, projected, true)
}

fn closed_projected_value_type_with_context(
    package: &SemanticPackage,
    projected: &crate::rust_interop::projection::ProjectedType,
    native_argument: bool,
) -> Option<ValueType> {
    use crate::rust_interop::projection::ProjectedType;
    Some(match projected {
        ProjectedType::Generic(name) => ValueType::ProjectedGeneric(name.clone()),
        ProjectedType::Optional(inner) => ValueType::Optional(Box::new(
            closed_projected_value_type_with_context(package, inner, native_argument)?,
        )),
        ProjectedType::Foreign { .. } => closed_projected_foreign_value_type(package, projected)?,
        ProjectedType::BoxedInterface {
            associated_type, ..
        } => {
            let (namespace, name) = package.projection.owner_for_projected_type(projected)?;
            let identity = ObjectIdentity::new(namespace, name);
            ValueType::Object(if let Some(associated) = associated_type {
                identity.with_application(closed_projected_value_type_with_context(
                    package,
                    &associated.ty,
                    native_argument,
                )?)
            } else {
                identity
            })
        }
        ProjectedType::None => ValueType::Scalar(ScalarType::None),
        ProjectedType::String | ProjectedType::BorrowedString => {
            ValueType::Scalar(ScalarType::String)
        }
        ProjectedType::Bool => ValueType::Scalar(ScalarType::Bool),
        ProjectedType::Int => ValueType::Scalar(ScalarType::Int),
        ProjectedType::RustInt(name) if native_argument => ValueType::Scalar(
            ScalarType::ALL
                .into_iter()
                .find(|scalar| scalar.rust_type() == Some(name.as_str()))?,
        ),
        ProjectedType::Char if native_argument => return None,
        ProjectedType::RustInt(_)
        | ProjectedType::FixedInt(_)
        | ProjectedType::Float
        | ProjectedType::Float32
        | ProjectedType::Char
        | ProjectedType::Bytes => {
            ValueType::Scalar(ScalarType::from_source_name(&projected.terrane_name())?)
        }
        ProjectedType::Sequence { item, .. } => ValueType::List(ElementType::new(
            closed_projected_value_type_with_context(package, item, native_argument)?,
        )),
        ProjectedType::Tuple(items) => {
            let [first, rest @ ..] = items.as_slice() else {
                return None;
            };
            if rest.iter().any(|item| item != first) {
                return None;
            }
            ValueType::Tuple(
                ElementType::new(closed_projected_value_type_with_context(
                    package,
                    first,
                    native_argument,
                )?),
                Some(items.len()),
            )
        }
        ProjectedType::Mapping {
            key,
            value,
            ordered,
            ..
        } => {
            let key = ElementType::new(closed_projected_value_type_with_context(
                package,
                key,
                native_argument,
            )?);
            let value = ElementType::new(closed_projected_value_type_with_context(
                package,
                value,
                native_argument,
            )?);
            key.scalar()?;
            if *ordered {
                ValueType::Map(key, value)
            } else {
                ValueType::UnorderedMap(key, value)
            }
        }
        ProjectedType::Set { item, ordered, .. } => {
            let item = ElementType::new(closed_projected_value_type_with_context(
                package,
                item,
                native_argument,
            )?);
            item.scalar()?;
            if *ordered {
                ValueType::Set(item)
            } else {
                ValueType::UnorderedSet(item)
            }
        }
        _ => return None,
    })
}

/// Rebuild native representations after source-generic substitution.
pub(super) fn canonicalize_native_value_type(
    package: &SemanticPackage,
    value_type: &mut ValueType,
) {
    match value_type {
        ValueType::Object(identity)
            if super::native_constructors::source_abi_nominal(package, identity) =>
        {
            *identity = super::native_constructors::canonical_source_nominal(package, identity);
        }
        ValueType::Object(identity)
            if identity.native_projection.is_some() || !identity.native_arguments.is_empty() =>
        {
            if let Ok(projected) = destination_projected_object(package, identity) {
                let native = package
                    .projection
                    .canonical_native_alias(&projected)
                    .unwrap_or(projected)
                    .rust_type();
                identity.native_projection = Some(
                    package
                        .projection
                        .canonical_native_type(&native)
                        .into_owned(),
                );
            }
        }
        ValueType::Optional(inner) => canonicalize_native_value_type(package, inner),
        ValueType::Reference(inner)
        | ValueType::SharedReference(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner) => {
            canonicalize_native_value_type(package, inner.value_type_mut());
        }
        ValueType::Map(key, value)
        | ValueType::UnorderedMap(key, value)
        | ValueType::Entry(key, value) => {
            canonicalize_native_value_type(package, key.value_type_mut());
            canonicalize_native_value_type(package, value.value_type_mut());
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                canonicalize_native_value_type(package, parameter.value_type_mut());
            }
            canonicalize_native_value_type(package, result.value_type_mut());
        }
        _ => {}
    }
}

fn closed_written_projected_type(
    package: &SemanticPackage,
    value_type: &ValueType,
) -> Option<crate::rust_interop::projection::ProjectedType> {
    match value_type {
        ValueType::Object(identity) => {
            let selected =
                close_written_native_nominal(package, identity).unwrap_or_else(|| identity.clone());
            destination_projected_type(package, &ValueType::Object(selected)).ok()
        }
        _ => destination_projected_type(package, value_type).ok(),
    }
}

fn close_written_native_arguments(
    package: &SemanticPackage,
    generic_parameters: &[crate::rust_interop::projection::ProjectedGenericParameter],
    supplied_arguments: &[ValueType],
) -> Option<(
    BTreeMap<String, ValueType>,
    BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
)> {
    let mut selections = BTreeMap::new();
    let mut substitutions =
        BTreeMap::<String, crate::rust_interop::projection::ProjectedType>::new();
    for (index, parameter) in generic_parameters.iter().enumerate() {
        let projected_argument = if let Some(argument) = supplied_arguments.get(index) {
            closed_written_projected_type(package, argument)?
        } else {
            let default = parameter.default.as_ref()?;
            substitutions
                .iter()
                .fold(default.clone(), |ty, (name, value)| {
                    substitute_projected_generic(&ty, name, value)
                })
        };
        let selected = closed_projected_native_argument(package, &projected_argument)?;
        let canonical_projected_argument = destination_projected_type(package, &selected).ok()?;
        substitutions.insert(parameter.name.clone(), canonical_projected_argument);
        selections.insert(parameter.name.clone(), selected);
    }
    Some((selections, substitutions))
}
fn close_nongeneric_native_nominal(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
    rust_path: &str,
) -> Option<ObjectIdentity> {
    let constructor = crate::rust_ir::rust_type_constructor(rust_path)?;
    if identity.native_projection.is_none()
        && !package
            .projection
            .native_owner_aliases
            .contains_key(&constructor)
    {
        // Compiler-owned protocol families have no Rustdoc nominal declaration.
        return None;
    }
    let native = identity.native_projection.as_deref().unwrap_or(rust_path);
    Some(
        identity.clone().with_native_projection(
            package
                .projection
                .canonical_native_type(native)
                .into_owned(),
        ),
    )
}

pub(super) fn close_written_native_nominal(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
) -> Option<ObjectIdentity> {
    if super::native_constructors::source_abi_nominal(package, identity) {
        return Some(super::native_constructors::canonical_source_nominal(
            package, identity,
        ));
    }
    let item = package
        .projection
        .item(&identity.namespace, &identity.name)?;
    let (crate::rust_interop::projection::ProjectedKind::ForeignType {
        generic_parameters, ..
    }
    | crate::rust_interop::projection::ProjectedKind::Enum {
        generic_parameters, ..
    }) = &item.kind
    else {
        return None;
    };
    let supplied_arguments = if !identity.type_arguments.is_empty() {
        identity.type_arguments.as_slice()
    } else if let Some(application) = identity.application.as_deref() {
        std::slice::from_ref(application)
    } else {
        &[]
    };
    if supplied_arguments.len() > generic_parameters.len() {
        return None;
    }
    if generic_parameters.is_empty() {
        return close_nongeneric_native_nominal(package, identity, &item.rust_path);
    }
    let (selections, substitutions) =
        close_written_native_arguments(package, generic_parameters, supplied_arguments)?;
    let rust_path = if package.projection.dependencies.iter().any(|dependency| {
        dependency
            .native_alias_identities
            .contains_key(&item.rust_path)
    }) {
        let arguments = generic_parameters
            .iter()
            .map(|parameter| substitutions[&parameter.name].clone())
            .collect::<Vec<_>>();
        let alias = crate::rust_interop::projection::ProjectedType::Foreign {
            rust_path: item.rust_path.clone(),
            base_rust_path: item.rust_path.clone(),
            name: item.name.clone(),
            arguments,
        };
        package
            .projection
            .canonical_native_alias(&alias)?
            .rust_type()
    } else {
        let base_rust_path = crate::rust_ir::rust_type_constructor(
            &package.projection.canonical_native_type(&item.rust_path),
        )?;
        format!(
            "{}<{}>",
            base_rust_path,
            generic_parameters
                .iter()
                .map(|parameter| substitutions[&parameter.name].rust_type())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let mut selected = identity.clone();
    // Clear both halves of the legacy application identity before installing its
    // normalized ordered native arguments.
    selected.application = None;
    selected.application_key = None;
    Some(
        selected
            .with_type_arguments(
                generic_parameters
                    .iter()
                    .map(|parameter| selections[&parameter.name].clone())
                    .collect(),
            )
            .with_native_parameters(
                generic_parameters
                    .iter()
                    .map(|parameter| parameter.name.clone())
                    .collect(),
            )
            .with_native_arguments(selections)
            .with_native_projection(rust_path),
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "one closed conversion keeps every semantic destination category exhaustive"
)]
pub(crate) fn destination_projected_type(
    package: &SemanticPackage,
    value_type: &ValueType,
) -> Result<crate::rust_interop::projection::ProjectedType, DestinationProjectionError> {
    use crate::rust_interop::projection::ProjectedType;
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
        ValueType::ProjectedGeneric(name) => ProjectedType::Generic(name.clone()),
        ValueType::Optional(inner) => {
            ProjectedType::Optional(Box::new(destination_projected_type(package, inner)?))
        }
        ValueType::List(item) => {
            let item = destination_projected_type(package, item.value_type_ref())?;
            ProjectedType::Sequence {
                rust_path: format!("std::vec::Vec<{}>", item.rust_type()),
                item: Box::new(item),
            }
        }
        ValueType::Map(key, value) => destination_projected_mapping(package, key, value, true)?,
        ValueType::UnorderedMap(key, value) => {
            destination_projected_mapping(package, key, value, false)?
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
        ValueType::Function(parameters, result, _) => {
            destination_projected_callback(package, parameters, result, false, true, None)?
        }
        ValueType::AsyncFunction(parameters, result, transferability, _) => {
            destination_projected_callback(
                package,
                parameters,
                result,
                true,
                *transferability != TaskTransferability::Local,
                None,
            )?
        }
        ValueType::InvocationScopedNative {
            rust_type,
            concrete: true,
            family,
            lifetimes,
            expression_scoped,
            ..
        } => ProjectedType::InvocationScoped {
            rust_type: rust_type.clone(),
            name: family.name.clone(),
            lifetimes: lifetimes.clone(),
            expression_scoped: *expression_scoped,
            owned: Box::new(ProjectedType::Foreign {
                rust_path: rust_type.clone(),
                name: family.name.clone(),
                base_rust_path: rust_type
                    .split_once('<')
                    .map_or_else(|| rust_type.clone(), |(base, _)| base.to_owned()),
                arguments: Vec::new(),
            }),
        },
        ValueType::InvocationScopedNative {
            rust_type,
            concrete: false,
            family,
            lifetimes,
            expression_scoped,
            ..
        } => ProjectedType::InvocationScoped {
            rust_type: rust_type.clone(),
            name: family.name.clone(),
            lifetimes: lifetimes.clone(),
            expression_scoped: *expression_scoped,
            owned: Box::new(ProjectedType::Foreign {
                rust_path: rust_type.clone(),
                name: family.name.clone(),
                base_rust_path: rust_type.clone(),
                arguments: Vec::new(),
            }),
        },
        ValueType::Object(identity) => destination_projected_object(package, identity)?,
        ValueType::Reference(_) | ValueType::SharedReference(_) => {
            return Err("borrowed results cannot escape a projected call".into());
        }
        _ => return Err("the destination is outside the closed projected result set".into()),
    })
}

/// Apply the already-selected owner arguments to its native field contract.
pub(crate) fn projected_owned_field_type(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
    name: &str,
) -> Option<crate::rust_interop::projection::ProjectedType> {
    let (_, fields, borrowed_view) = package
        .projection
        .projected_struct(&identity.namespace, &identity.name)?;
    if borrowed_view {
        return None;
    }
    let mut projected = fields.iter().find(|field| field.name == name)?.ty.clone();
    for (parameter, value_type) in &identity.native_arguments {
        let selected = destination_projected_type(package, value_type).ok()?;
        projected = substitute_projected_generic(&projected, parameter, &selected);
    }
    Some(projected)
}

fn projected_reference_type(
    projected: crate::rust_interop::projection::ProjectedType,
    mutable: bool,
) -> crate::rust_interop::projection::ProjectedType {
    crate::rust_interop::projection::ProjectedType::Reference {
        inner: Box::new(projected),
        mutable,
        lifetime: None,
    }
}

fn destination_projected_native_nominal(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
    item: &crate::rust_interop::projection::ProjectedItem,
    generic_parameters: &[crate::rust_interop::projection::ProjectedGenericParameter],
) -> Result<crate::rust_interop::projection::ProjectedType, DestinationProjectionError> {
    let arguments = generic_parameters
        .iter()
        .map(|parameter| {
            identity
                .native_arguments
                .get(&parameter.name)
                .ok_or_else(|| DestinationProjectionError::from("missing native generic selection"))
                .and_then(|value| destination_projected_type(package, value))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if package.projection.dependencies.iter().any(|dependency| {
        dependency
            .native_alias_identities
            .contains_key(&item.rust_path)
    }) {
        let alias = crate::rust_interop::projection::ProjectedType::Foreign {
            rust_path: item.rust_path.clone(),
            base_rust_path: item.rust_path.clone(),
            name: item.name.clone(),
            arguments,
        };
        return package
            .projection
            .canonical_native_alias(&alias)
            .ok_or_else(|| {
                DestinationProjectionError::from("native alias selection is not closed")
            });
    }
    let base_rust_path = crate::rust_ir::rust_type_constructor(
        &package.projection.canonical_native_type(&item.rust_path),
    )
    .ok_or("selected native owner has no nominal Rust constructor")?;
    Ok(crate::rust_interop::projection::ProjectedType::Foreign {
        rust_path: format!(
            "{}<{}>",
            base_rust_path,
            arguments
                .iter()
                .map(crate::rust_interop::projection::ProjectedType::rust_type)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        name: item.name.clone(),
        base_rust_path,
        arguments,
    })
}
fn destination_projected_object(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
) -> Result<crate::rust_interop::projection::ProjectedType, DestinationProjectionError> {
    if identity.native_arguments.is_empty()
        && let Some(selected) = close_written_native_nominal(package, identity)
        && !selected.native_arguments.is_empty()
    {
        return destination_projected_object(package, &selected);
    }
    if !identity.native_arguments.is_empty()
        && let Some(item) = package.projection.item(&identity.namespace, &identity.name)
        && let crate::rust_interop::projection::ProjectedKind::ForeignType {
            generic_parameters,
            ..
        }
        | crate::rust_interop::projection::ProjectedKind::Enum {
            generic_parameters, ..
        } = &item.kind
    {
        return destination_projected_native_nominal(package, identity, item, generic_parameters);
    }
    if !identity.native_arguments.is_empty()
        && let Some(constructor) = package
            .projection
            .projected_constructor(&identity.namespace, &identity.name)
    {
        let mut projected = constructor.result.clone();
        for (name, value_type) in &identity.native_arguments {
            let selected = destination_projected_type(package, value_type)?;
            projected = substitute_projected_generic(&projected, name, &selected);
        }
        return Ok(projected);
    }
    if let Some(projected) = package
        .projection
        .projected_type(&identity.namespace, &identity.name)
    {
        return Ok(projected);
    }
    if let Some(details) = package
        .projection
        .item_ambiguity(&identity.namespace, &identity.name)
    {
        return Err(format!(
            "projected object `{}::{}` is ambiguous: {details}",
            identity.namespace, identity.name
        )
        .into());
    }
    let item = package
        .projection
        .item(&identity.namespace, &identity.name)
        .filter(|item| {
            matches!(
                item.kind,
                crate::rust_interop::projection::ProjectedKind::ForeignType { .. }
                    | crate::rust_interop::projection::ProjectedKind::Interface(_)
                    | crate::rust_interop::projection::ProjectedKind::Enum { .. }
            )
        })
        .ok_or("source-declared object destinations have no dependency conversion contract")?;
    Ok(crate::rust_interop::projection::ProjectedType::Foreign {
        rust_path: item.rust_path.clone(),
        name: item.name.clone(),
        base_rust_path: item.rust_path.clone(),
        arguments: Vec::new(),
    })
}

fn destination_projected_mapping(
    package: &SemanticPackage,
    key: &ElementType,
    value: &ElementType,
    ordered: bool,
) -> Result<crate::rust_interop::projection::ProjectedType, DestinationProjectionError> {
    let key = destination_projected_type(package, key.value_type_ref())?;
    let value = destination_projected_type(package, value.value_type_ref())?;
    Ok(crate::rust_interop::projection::ProjectedType::Mapping {
        rust_path: format!(
            "std::collections::{}<{}, {}>",
            if ordered { "BTreeMap" } else { "HashMap" },
            key.rust_type(),
            value.rust_type()
        ),
        key: Box::new(key),
        value: Box::new(value),
        ordered,
    })
}

fn destination_projected_callback(
    package: &SemanticPackage,
    parameters: &[CallableParameterType],
    result: &ElementType,
    is_async: bool,
    send: bool,
    projected_result: Option<crate::rust_interop::projection::ProjectedType>,
) -> Result<crate::rust_interop::projection::ProjectedType, DestinationProjectionError> {
    if parameters.iter().any(CallableParameterType::is_variadic) {
        return Err("variadic source callables have no fixed Rust callback representation".into());
    }
    let parameter_borrows = parameters
        .iter()
        .map(|parameter| {
            matches!(
                parameter.element_type().value_type_ref(),
                ValueType::Reference(_) | ValueType::SharedReference(_)
            )
        })
        .collect::<Vec<_>>();
    let parameters = parameters
        .iter()
        .map(|parameter| {
            let element_type = parameter.element_type();
            let value_type = element_type.value_type_ref();
            let value_type = match value_type {
                ValueType::Reference(inner) | ValueType::SharedReference(inner) => {
                    inner.value_type_ref()
                }
                _ => value_type,
            };
            destination_projected_type(package, value_type)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let result = if let Some(projected_result) = projected_result {
        projected_result
    } else {
        destination_projected_type(package, result.value_type_ref())?
    };
    let parameters_rust = parameters
        .iter()
        .map(crate::rust_interop::projection::ProjectedType::rust_type)
        .collect::<Vec<_>>()
        .join(", ");
    let rust_name = if is_async {
        format!(
            "fn({parameters_rust}) -> std::pin::Pin<std::boxed::Box<dyn Future<Output = {}>{}>>",
            result.rust_type(),
            if send { " + Send" } else { "" }
        )
    } else {
        format!("fn({parameters_rust}) -> {}", result.rust_type())
    };
    let parameter_rust_types = parameters
        .iter()
        .map(crate::rust_interop::projection::ProjectedType::rust_type)
        .collect();
    Ok(crate::rust_interop::projection::ProjectedType::Callback {
        rust_name,
        native_bound: None,
        native_method: None,
        native_result: None,
        native_substitutions: BTreeMap::new(),
        parameters,
        parameter_rust_types,
        parameters_destination_selected: false,
        result: Box::new(result),
        invocation_mode: InvocationMode::Shared,
        is_async,
        retained: false,
        parameter_borrows,
        send,
        sync: true,
    })
}

fn projected_types_share_concrete_rust_representation(
    left: &crate::rust_interop::projection::ProjectedType,
    right: &crate::rust_interop::projection::ProjectedType,
) -> bool {
    use crate::rust_interop::projection::ProjectedType;

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

fn projected_call_owner_identity(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    callee: &SyntaxNode,
) -> Option<ObjectIdentity> {
    let mut callee = callee;
    while matches!(
        callee.kind,
        SyntaxKind::GroupExpression | SyntaxKind::TypeExpression | SyntaxKind::AppliedType
    ) {
        callee = callee.children.first()?;
    }
    let receiver = callee.children.first()?;
    let owner = if callee.kind == SyntaxKind::StaticMemberExpression {
        class_designator_identity(unit, receiver)?
    } else if callee.kind == SyntaxKind::MemberExpression {
        match infer_receiver_value_type(unit, receiver, &unit.typed_bindings)
            .ok()
            .flatten()?
        {
            ValueType::Object(identity)
            | ValueType::InvocationScopedNative {
                family: identity, ..
            } => identity,
            _ => return None,
        }
    } else {
        return None;
    };
    Some(close_written_native_nominal(package, &owner).unwrap_or(owner))
}

pub(super) fn projected_call_owner_substitutions(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    callee: &SyntaxNode,
) -> Result<BTreeMap<String, crate::rust_interop::projection::ProjectedType>, String> {
    let Some(owner) = projected_call_owner_identity(package, unit, callee) else {
        return Ok(BTreeMap::new());
    };
    let function =
        projected_function_for_call(package, unit, callee, crate::syntax::call_is_unsafe(callee));
    let Some(function) = function.filter(|function| !function.operation_owner_generics.is_empty())
    else {
        return owner
            .native_arguments
            .into_iter()
            .map(|(name, value_type)| {
                destination_projected_type(package, &value_type)
                    .map(|projected| (name.clone(), projected))
                    .map_err(|reason| {
                        format!(
                            "projected method owner argument `{name}` is not supported: {reason}"
                        )
                    })
            })
            .collect();
    };
    let mut substitutions = BTreeMap::new();
    for owner_slot in &function.operation_owner_generics {
        let native_argument = owner
            .native_arguments
            .get_key_value(&owner_slot.name)
            .or_else(|| {
                let (name, suffix) = owner_slot
                    .name
                    .strip_prefix("__TerraneOwner_")?
                    .rsplit_once("__")?;
                suffix.parse::<usize>().ok()?;
                owner.native_arguments.get_key_value(name)
            });
        let Some((nominal_name, value_type)) = native_argument else {
            continue;
        };
        let projected = destination_projected_type(package, value_type).map_err(|reason| {
            format!("projected method owner argument `{nominal_name}` is not supported: {reason}")
        })?;
        substitutions.insert(owner_slot.name.clone(), projected);
    }
    Ok(substitutions)
}

#[expect(
    clippy::too_many_lines,
    reason = "recursive projected type matching is clearest as one exhaustive traversal"
)]
fn select_projected_generic_destinations(
    template: &crate::rust_interop::projection::ProjectedType,
    parameters: &BTreeSet<String>,
    expected: &crate::rust_interop::projection::ProjectedType,
) -> Result<Option<BTreeMap<String, crate::rust_interop::projection::ProjectedType>>, String> {
    fn collect(
        template: &crate::rust_interop::projection::ProjectedType,
        parameters: &BTreeSet<String>,
        expected: &crate::rust_interop::projection::ProjectedType,
        destinations: &mut BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
    ) -> Result<bool, String> {
        use crate::rust_interop::projection::ProjectedType;
        match (template, expected) {
            (
                ProjectedType::InvocationScoped {
                    owned,
                    expression_scoped: true,
                    ..
                },
                expected,
            ) => collect(owned, parameters, expected, destinations),
            (ProjectedType::Generic(name), expected) if parameters.contains(name) => {
                if let Some(previous) = destinations.get(name) {
                    return if previous == expected {
                        Ok(true)
                    } else {
                        Err(name.clone())
                    };
                }
                destinations.insert(name.clone(), expected.clone());
                Ok(true)
            }
            (ProjectedType::Optional(template), ProjectedType::Optional(expected))
            | (
                ProjectedType::Sequence { item: template, .. },
                ProjectedType::Sequence { item: expected, .. },
            )
            | (
                ProjectedType::Set { item: template, .. },
                ProjectedType::Set { item: expected, .. },
            ) => collect(template, parameters, expected, destinations),
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
            ) => Ok(
                collect(template_key, parameters, expected_key, destinations)?
                    && collect(template_value, parameters, expected_value, destinations)?,
            ),
            (ProjectedType::Tuple(template), ProjectedType::Tuple(expected))
                if template.len() == expected.len() =>
            {
                for (template, expected) in template.iter().zip(expected) {
                    if !collect(template, parameters, expected, destinations)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            (
                ProjectedType::Callback {
                    parameters: template_parameters,
                    result: template_result,
                    ..
                },
                ProjectedType::Callback {
                    parameters: expected_parameters,
                    result: expected_result,
                    ..
                },
            ) if template_parameters.len() == expected_parameters.len() => {
                for (template, expected) in template_parameters.iter().zip(expected_parameters) {
                    if !collect(template, parameters, expected, destinations)? {
                        return Ok(false);
                    }
                }
                collect(template_result, parameters, expected_result, destinations)
            }
            (
                ProjectedType::Foreign {
                    base_rust_path: template_base,
                    arguments: template_arguments,
                    ..
                },
                ProjectedType::Foreign {
                    base_rust_path: expected_base,
                    arguments: expected_arguments,
                    ..
                },
            ) if template_base == expected_base
                && template_arguments.len() >= expected_arguments.len() =>
            {
                for (template, expected) in template_arguments.iter().zip(expected_arguments) {
                    if !collect(template, parameters, expected, destinations)? {
                        return Ok(false);
                    }
                }
                Ok(template_arguments[expected_arguments.len()..]
                    .iter()
                    .all(|template| {
                        !matches!(
                            template,
                            ProjectedType::Generic(name) if parameters.contains(name)
                        )
                    }))
            }
            (template, expected)
                if projected_types_share_concrete_rust_representation(template, expected) =>
            {
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    let mut destinations = BTreeMap::new();
    if !collect(template, parameters, expected, &mut destinations)?
        || destinations.len() != parameters.len()
    {
        return Ok(None);
    }
    Ok(Some(destinations))
}

fn align_projected_result_representation(
    projected: &crate::rust_interop::projection::ProjectedType,
    expected: &crate::rust_interop::projection::ProjectedType,
) -> crate::rust_interop::projection::ProjectedType {
    use crate::rust_interop::projection::ProjectedType;

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

#[expect(
    clippy::too_many_lines,
    reason = "recursive projected type substitution is clearest as one exhaustive match"
)]
pub(super) fn substitute_projected_generic(
    template: &crate::rust_interop::projection::ProjectedType,
    parameter: &str,
    destination: &crate::rust_interop::projection::ProjectedType,
) -> crate::rust_interop::projection::ProjectedType {
    use crate::rust_interop::projection::ProjectedType;
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
        ProjectedType::AsyncIterationStep(item) => ProjectedType::AsyncIterationStep(Box::new(
            substitute_projected_generic(item, parameter, destination),
        )),
        ProjectedType::Callback {
            rust_name,
            native_bound,
            native_method,
            native_result,
            native_substitutions,
            parameters,
            parameter_rust_types,
            parameter_borrows,
            parameters_destination_selected,
            result,
            invocation_mode,
            is_async,
            retained,
            send,
            sync,
        } => ProjectedType::Callback {
            rust_name: rust_name.clone(),
            native_bound: native_bound.as_ref().map(|bound| {
                crate::rust_ir::instantiate_rust_generics(
                    bound,
                    &BTreeMap::from([(parameter.to_owned(), destination.rust_type())]),
                )
            }),
            native_method: native_method.clone(),
            native_result: native_result.as_ref().map(|result| {
                crate::rust_ir::instantiate_rust_generics(
                    result,
                    &BTreeMap::from([(parameter.to_owned(), destination.rust_type())]),
                )
            }),
            native_substitutions: native_substitutions
                .iter()
                .map(|(name, ty)| {
                    (
                        name.clone(),
                        substitute_projected_generic(ty, parameter, destination),
                    )
                })
                .collect(),
            parameters_destination_selected: *parameters_destination_selected,
            parameter_rust_types: parameter_rust_types
                .iter()
                .map(|rust_type| {
                    crate::rust_ir::instantiate_rust_generics(
                        rust_type,
                        &BTreeMap::from([(parameter.to_owned(), destination.rust_type())]),
                    )
                })
                .collect(),
            parameter_borrows: parameter_borrows.clone(),
            parameters: parameters
                .iter()
                .map(|item| substitute_projected_generic(item, parameter, destination))
                .collect(),
            result: Box::new(substitute_projected_generic(result, parameter, destination)),
            invocation_mode: *invocation_mode,
            is_async: *is_async,
            retained: *retained,
            send: *send,
            sync: *sync,
        },
        ProjectedType::Reference {
            inner,
            mutable,
            lifetime,
        } => ProjectedType::Reference {
            inner: Box::new(substitute_projected_generic(inner, parameter, destination)),
            mutable: *mutable,
            lifetime: lifetime.clone(),
        },
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
