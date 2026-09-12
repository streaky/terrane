use super::prelude::*;

fn open_projected_associated_interface(unit: &SemanticUnit, value_type: &ValueType) -> bool {
    match value_type {
        ValueType::Object(identity) => {
            identity.application.is_none()
                && unit
                    .projected_interfaces_requiring_application
                    .contains(&identity.base())
        }
        ValueType::Optional(inner) => open_projected_associated_interface(unit, inner),
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
            open_projected_associated_interface(unit, item.value_type_ref())
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            open_projected_associated_interface(unit, key.value_type_ref())
                || open_projected_associated_interface(unit, value.value_type_ref())
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            parameters.iter().any(|parameter| {
                open_projected_associated_interface(unit, parameter.value_type_ref())
            }) || open_projected_associated_interface(unit, result.value_type_ref())
        }
        _ => false,
    }
}
#[expect(
    clippy::too_many_lines,
    reason = "binding analysis keeps destination selection and initialization validation together"
)]
pub(super) fn analyze_binding_node(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &mut Vec<TypedBinding>,
    scope: Option<Span>,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::Assignment
        && let [target, value] = node.children.as_slice()
        && target.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = target.children.as_slice()
    {
        infer_member_value_type(unit, target, bindings)?;
        let expected = match infer_receiver_value_type(unit, receiver, bindings)? {
            Some(ValueType::Object(identity)) => {
                object_field_type(unit, &identity, node_text(&unit.source, member), false)
            }
            _ => None,
        };
        let Some(expected) = expected else {
            return Err(failure(
                &unit.source,
                "T0072",
                format!(
                    "member `{}` is read-only and cannot be assigned",
                    node_text(&unit.source, member)
                ),
                member.span,
            ));
        };
        if let Some(actual) = infer_value_type(unit, value, bindings)? {
            validate_value_destination(
                &unit.source,
                &unit.descriptors,
                node_text(&unit.source, member),
                expected,
                actual,
                value,
                "T0077",
            )?;
        }
        return Ok(());
    }
    if node.kind == SyntaxKind::Assignment
        && let [target, value] = node.children.as_slice()
        && target.kind == SyntaxKind::IndexExpression
        && let [receiver, _] = target.children.as_slice()
        && let Some(receiver_type) = infer_receiver_value_type(unit, receiver, bindings)?
    {
        let expected = match receiver_type {
            ValueType::List(item) => Some(item.value_type()),
            ValueType::Map(_, value) | ValueType::UnorderedMap(_, value) => {
                Some(value.value_type())
            }
            ValueType::Tuple(_, _) => {
                return Err(failure(
                    &unit.source,
                    "T0048",
                    "tuple items are fixed at construction and cannot be replaced",
                    target.span,
                ));
            }
            other => {
                return Err(failure(
                    &unit.source,
                    "T0048",
                    format!("indexed assignment is not supported for `{other}`"),
                    target.span,
                ));
            }
        };
        let actual = infer_value_type(unit, value, bindings)?;
        if let (Some(expected), Some(actual)) = (expected, actual)
            && expected != actual
        {
            return Err(failure(
                &unit.source,
                "T0046",
                format!("indexed assignment requires `{expected}`, found `{actual}`"),
                value.span,
            ));
        }
        return Ok(());
    }
    let Some(name_node) = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::Name)
    else {
        return Ok(());
    };
    let name = node_text(&unit.source, name_node).to_owned();
    let declared = node
        .children
        .iter()
        .find(|child| child.kind == SyntaxKind::TypeExpression);
    let initializer = node.children.iter().rev().find(|child| {
        child.span != name_node.span
            && !matches!(
                child.kind,
                SyntaxKind::Visibility
                    | SyntaxKind::DeclarationQualifier
                    | SyntaxKind::TypeExpression
            )
    });
    if let Some(initializer) = initializer {
        validate_invocation_only_member_expression(unit, initializer, bindings)?;
    }

    if node.kind == SyntaxKind::Assignment
        && declared.is_none()
        && let Some(previous) = bindings.iter().rev().find(|binding| binding.name == name)
        && let Some(initializer) = initializer
        && let Some(actual) = infer_value_type(unit, initializer, bindings)?
    {
        if previous.destination_arms.is_empty() {
            if let ValueType::Scalar(expected) = previous.value_type.clone() {
                validate_numeric_destination(
                    &unit.source,
                    &name,
                    expected,
                    actual,
                    initializer,
                    "T0002",
                )?;
            }
        } else {
            select_union_candidates(
                &unit.source,
                initializer,
                actual,
                previous.destination_arms.clone(),
            )?;
        }
        return Ok(());
    }
    let declared_value = declared
        .map(|type_node| {
            let aliases = visible_descriptor_aliases(
                &unit.descriptor_aliases,
                unit.source.id(),
                type_node.span.start,
            );
            declared_value_type(unit, type_node, &aliases).or_else(|failure| {
                union_destination_candidates(unit, type_node)
                    .ok()
                    .and_then(|candidates| candidates.into_iter().next())
                    .map(ValueType::Scalar)
                    .ok_or(failure)
            })
        })
        .transpose()?;
    if declared_value
        .as_ref()
        .is_some_and(|value_type| open_projected_associated_interface(unit, value_type))
    {
        return Err(failure(
            &unit.source,
            "T0127",
            "a projected interface annotation requires one closed associated type",
            declared
                .expect("declared value type has a syntax node")
                .span,
        ));
    }
    if declared_value.is_none()
        && let Some(initializer) = initializer
        && let Some(identity) = empty_collection_identity(unit, initializer, bindings)
    {
        let collection = identity
            .strip_prefix("/core/collections::")
            .unwrap_or(identity);
        if collection == "entry" {
            return Err(failure(
                &unit.source,
                "T0045",
                "`entry` requires exactly a key and value",
                initializer.span,
            ));
        }
        let message = if matches!(collection, "map" | "unordered-map") {
            format!("an empty `{collection}` requires explicit key and value types")
        } else {
            format!("an empty `{collection}` requires an explicit item type")
        };
        return Err(failure(&unit.source, "T0043", message, initializer.span));
    }
    if let Some(initializer) = initializer
        && initializer.kind == SyntaxKind::Name
        && collection_constructor_identity(unit, initializer, bindings).is_some_and(|identity| {
            identity
                .strip_prefix("/core/collections::")
                .unwrap_or(identity)
                == "entry"
        })
    {
        return Err(failure(
            &unit.source,
            "T0045",
            "`entry` requires exactly a key and value",
            initializer.span,
        ));
    }
    let inferred = initializer
        .map(|value| {
            if let Some(declared_value) = declared_value.clone()
                && collection_constructor_matches(unit, value, &declared_value, bindings)
            {
                validate_collection_constructor_items(
                    unit,
                    value,
                    &declared_value,
                    &name,
                    bindings,
                )?;
                Ok(Some(declared_value))
            } else {
                infer_value_type(unit, value, bindings)
            }
        })
        .transpose()?
        .flatten();
    let value_type =
        if let (Some(type_node), Some(declared_type)) = (declared, declared_value.clone()) {
            let value_type = if matches!(declared_type, ValueType::Optional(_)) {
                declared_type
            } else if let (Some(inferred), Some(initializer), Ok(_)) = (
                inferred.clone(),
                initializer,
                union_destination_candidates(unit, type_node),
            ) {
                ValueType::Scalar(select_union_destination(
                    unit,
                    type_node,
                    initializer,
                    inferred,
                )?)
            } else {
                declared_type
            };
            if let (Some(inferred), Some(initializer)) = (inferred.clone(), initializer) {
                validate_value_destination(
                    &unit.source,
                    &unit.descriptors,
                    &name,
                    value_type.clone(),
                    inferred,
                    initializer,
                    "T0002",
                )?;
            }
            value_type
        } else if let Some(inferred) = inferred.clone() {
            inferred
        } else {
            return Ok(());
        };
    let value_type = inferred.as_ref().map_or(value_type.clone(), |actual| {
        merge_callable_destination_effects(value_type, actual)
    });
    let destination_arms = if matches!(value_type, ValueType::Optional(_)) {
        Vec::new()
    } else {
        declared
            .and_then(|type_node| union_destination_candidates(unit, type_node).ok())
            .filter(|arms| arms.len() > 1)
            .unwrap_or_default()
    };
    let storage_type = (value_type == ValueType::Scalar(ScalarType::Int))
        .then(|| initializer.and_then(|value| small_int_storage(unit, value, inferred.clone())))
        .flatten();

    bindings.push(TypedBinding {
        name,
        span: node.span,
        visible_from: node.span.end,
        scope,
        value_type,
        destination_arms,
        storage_type,
        mutable: false,
    });
    Ok(())
}

pub(super) fn declared_value_type(
    unit: &SemanticUnit,
    type_node: &SyntaxNode,
    aliases: &BTreeMap<String, ScalarType>,
) -> Result<ValueType, SemanticFailure> {
    declared_value_type_with_visible_objects(unit, type_node, aliases, &BTreeMap::new())
}

#[expect(
    clippy::too_many_lines,
    reason = "type-shape validation keeps all supported composite forms in one ordered match"
)]
pub(super) fn declared_value_type_with_visible_objects(
    unit: &SemanticUnit,
    type_node: &SyntaxNode,
    aliases: &BTreeMap<String, ScalarType>,
    visible_objects: &BTreeMap<String, ObjectIdentity>,
) -> Result<ValueType, SemanticFailure> {
    let shape = if type_node.kind == SyntaxKind::TypeExpression {
        type_node.children.first().unwrap_or(type_node)
    } else {
        type_node
    };
    if shape.kind == SyntaxKind::PrefixType
        && let Some(inner) = shape.children.first()
    {
        let inner = ElementType::new(declared_value_type_with_visible_objects(
            unit,
            inner,
            aliases,
            visible_objects,
        )?);
        return Ok(
            if node_text(&unit.source, shape)
                .split_whitespace()
                .take(2)
                .eq(["shared", "ref"])
            {
                ValueType::SharedReference(inner)
            } else {
                ValueType::Reference(inner)
            },
        );
    }
    if shape.kind == SyntaxKind::FunctionType {
        let function = shape;
        let mut signature = function
            .children
            .iter()
            .filter(|child| child.kind == SyntaxKind::TypeExpression)
            .collect::<Vec<_>>();
        let Some(result) = signature.pop() else {
            return Err(failure(
                &unit.source,
                "T0001",
                "function type requires a result type",
                type_node.span,
            ));
        };
        let parameters = signature
            .into_iter()
            .map(|parameter| {
                declared_value_type_with_visible_objects(unit, parameter, aliases, visible_objects)
                    .map(ElementType::new)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let result = ElementType::new(declared_value_type_with_visible_objects(
            unit,
            result,
            aliases,
            visible_objects,
        )?);
        let upper_bound = function
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::EffectClause)
            .and_then(|effect| effect.children.first())
            .map(|bound| {
                declared_value_type_with_visible_objects(unit, bound, aliases, visible_objects)
            })
            .transpose()?;
        let has_qualifier = |qualifier| {
            function.children.iter().any(|child| {
                child.kind == SyntaxKind::DeclarationQualifier
                    && node_text(&unit.source, child) == qualifier
            })
        };
        let written = if has_qualifier("consuming") {
            InvocationMode::Consuming
        } else if has_qualifier("mutable") {
            InvocationMode::Mutable
        } else {
            InvocationMode::Shared
        };
        let mut effects = CallableEffects {
            modes: CallableModes {
                written,
                exact: written,
            },
            upper_bound: upper_bound.clone().map(Box::new),
            escaping: BTreeSet::new(),
        };
        if let Some(bound) = &upper_bound {
            let throwable = ValueType::Object(ObjectIdentity::new("/core/errors", "throwable"));
            if !value_types_compatible(&unit.descriptors, &throwable, bound) {
                return Err(failure(
                    &unit.source,
                    "T0027",
                    "callable `throws` upper bound must be a throwable object type",
                    type_node.span,
                ));
            }
            let ValueType::Object(identity) = bound else {
                unreachable!("throwable-compatible callable bound must be an object");
            };
            effects.escaping.insert(identity.qualified());
        }
        return Ok(if has_qualifier("async") {
            ValueType::AsyncFunction(
                parameters,
                result,
                TaskTransferability::Transferable,
                effects,
            )
        } else {
            ValueType::Function(parameters, result, effects)
        });
    }
    if let Some(union) = type_node
        .children
        .first()
        .filter(|child| child.kind == SyntaxKind::UnionType)
    {
        let arms = union
            .children
            .iter()
            .filter(|arm| node_text(&unit.source, arm).trim() != "none")
            .collect::<Vec<_>>();
        if union.children.len() == 2 && arms.len() == 1 {
            let Some(arm) = arms.first().copied() else {
                return Err(failure(
                    &unit.source,
                    "T0001",
                    "an optional type requires one non-`none` arm",
                    union.span,
                ));
            };
            let inner =
                declared_value_type_with_visible_objects(unit, arm, aliases, visible_objects)?;
            if matches!(
                inner,
                ValueType::Scalar(ScalarType::None) | ValueType::Optional(_)
            ) {
                return Err(failure(
                    &unit.source,
                    "T0001",
                    "an optional type cannot contain `none` or another optional type",
                    union.span,
                ));
            }
            return Ok(ValueType::Optional(Box::new(inner)));
        }
    }
    if shape.kind == SyntaxKind::GroupExpression
        && let Some(inner) = shape.children.first()
    {
        return declared_value_type_with_visible_objects(unit, inner, aliases, visible_objects);
    }
    if shape.kind == SyntaxKind::AppliedType
        && let Some((base, arguments)) = shape.children.split_first()
    {
        let base_name = node_text(&unit.source, base).trim();
        let resolve_argument = |argument: &SyntaxNode| {
            declared_value_type_with_visible_objects(unit, argument, aliases, visible_objects)
        };
        if let [argument] = arguments {
            let construct = match base_name {
                "list" => Some(ValueType::List as fn(ElementType) -> ValueType),
                "tuple" => {
                    Some((|item| ValueType::Tuple(item, None)) as fn(ElementType) -> ValueType)
                }
                "set" => Some(ValueType::Set as fn(ElementType) -> ValueType),
                "unordered-set" => Some(ValueType::UnorderedSet as fn(ElementType) -> ValueType),
                "channel-sender" => Some(ValueType::ChannelSender as fn(ElementType) -> ValueType),
                "channel-receiver" => {
                    Some(ValueType::ChannelReceiver as fn(ElementType) -> ValueType)
                }
                "iterator" => Some(ValueType::Iterator as fn(ElementType) -> ValueType),
                "iteration-step" => Some(ValueType::IterationStep as fn(ElementType) -> ValueType),
                "async-iteration-step" => {
                    Some(ValueType::AsyncIterationStep as fn(ElementType) -> ValueType)
                }
                _ => None,
            };
            if let Some(construct) = construct {
                let item = ElementType::new(resolve_argument(argument)?);
                if matches!(base_name, "set" | "unordered-set") && item.scalar().is_none() {
                    return Err(failure(
                        &unit.source,
                        "T0001",
                        format!("`{base_name}` requires a scalar item type"),
                        argument.span,
                    ));
                }
                return Ok(construct(item));
            }
        }
        if let [key_node, value_node] = arguments
            && let Some(construct) = match base_name {
                "map" => Some(ValueType::Map as fn(ElementType, ElementType) -> ValueType),
                "unordered-map" => {
                    Some(ValueType::UnorderedMap as fn(ElementType, ElementType) -> ValueType)
                }
                "entry" => Some(ValueType::Entry as fn(ElementType, ElementType) -> ValueType),
                _ => None,
            }
        {
            let key = ElementType::new(resolve_argument(key_node)?);
            if key.scalar().is_none() {
                return Err(failure(
                    &unit.source,
                    "T0001",
                    format!("`{base_name}` requires a scalar key type"),
                    key_node.span,
                ));
            }
            return Ok(construct(
                key,
                ElementType::new(resolve_argument(value_node)?),
            ));
        }
        if let [argument] = arguments {
            let lexical_identity = lexical_scope_chain(unit, base.span.start).find_map(|scope| {
                scope.symbols.get(base_name).and_then(|symbols| {
                    symbols.iter().rev().find_map(|symbol| {
                        matches!(
                            symbol.kind,
                            SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                        )
                        .then(|| ObjectIdentity::new(&symbol.namespace, &symbol.name))
                    })
                })
            });
            let object_identity = lexical_identity
                .or_else(|| visible_objects.get(base_name).cloned())
                .or_else(|| {
                    unit.descriptors
                        .iter()
                        .find(|object| object.builtin.is_none() && object.name == base_name)
                        .map(|object| object.identity.clone())
                });
            if let Some(identity) = object_identity {
                return Ok(ValueType::Object(
                    identity.with_application(resolve_argument(argument)?),
                ));
            }
        }
    }
    let type_name = node_text(&unit.source, type_node).trim();
    let type_name = type_name
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .unwrap_or(type_name)
        .trim();
    let lexical_identity = lexical_scope_chain(unit, type_node.span.start).find_map(|scope| {
        scope.symbols.get(type_name).and_then(|symbols| {
            symbols.iter().rev().find_map(|symbol| {
                matches!(
                    symbol.kind,
                    SymbolKind::Class | SymbolKind::Interface | SymbolKind::Trait
                )
                .then(|| ObjectIdentity::new(&symbol.namespace, &symbol.name))
            })
        })
    });
    let object_identity = lexical_identity
        .or_else(|| visible_objects.get(type_name).cloned())
        .or_else(|| {
            unit.descriptors
                .iter()
                .find(|object| object.builtin.is_none() && object.name == type_name)
                .map(|object| object.identity.clone())
        });
    if let Some(identity) = object_identity {
        return Ok(ValueType::Object(identity));
    }
    match type_name {
        "host-projected-associated" => return Ok(ValueType::ProjectedAssociated),
        "host-resource-handle" => return Ok(ValueType::PlatformStreamHandle),
        "host-filesystem-authority" => return Ok(ValueType::FilesystemAuthority),
        "host-platform-data-result" => return Ok(ValueType::PlatformDataResult),
        "host-platform-url-result" => return Ok(ValueType::PlatformUrlResult),
        "host-platform-capability" => return Ok(ValueType::PlatformCapability),
        "host-platform-resource-handle" => return Ok(ValueType::PlatformResourceHandle),
        "host-platform-result" => return Ok(ValueType::PlatformResult),
        _ => {}
    }
    if type_name == "encoding" {
        return Ok(ValueType::Encoding);
    }
    parse_declared_value_type(type_name, aliases).ok_or_else(|| {
        failure(
            &unit.source,
            "T0001",
            format!("`{type_name}` does not resolve to a type descriptor"),
            type_node.span,
        )
    })
}

fn is_builtin_error_type(type_name: &str) -> bool {
    type_name == "throwable" || BUILTIN_ERROR_NAMES.contains(&type_name)
}

fn parse_single_argument_value_type(
    type_name: &str,
    aliases: &BTreeMap<String, ScalarType>,
) -> Option<ValueType> {
    for (constructor, construct) in [
        ("list of ", ValueType::List as fn(ElementType) -> ValueType),
        (
            "tuple of ",
            (|item| ValueType::Tuple(item, None)) as fn(ElementType) -> ValueType,
        ),
        ("set of ", ValueType::Set as fn(ElementType) -> ValueType),
        (
            "unordered-set of ",
            ValueType::UnorderedSet as fn(ElementType) -> ValueType,
        ),
        (
            "channel-sender of ",
            ValueType::ChannelSender as fn(ElementType) -> ValueType,
        ),
        (
            "channel-receiver of ",
            ValueType::ChannelReceiver as fn(ElementType) -> ValueType,
        ),
        (
            "iterator of ",
            ValueType::Iterator as fn(ElementType) -> ValueType,
        ),
        (
            "iteration-step of ",
            ValueType::IterationStep as fn(ElementType) -> ValueType,
        ),
    ] {
        if let Some(argument) = type_name.strip_prefix(constructor) {
            let item = ElementType::new(parse_declared_value_type(argument, aliases)?);
            if matches!(constructor, "set of " | "unordered-set of ") && item.scalar().is_none() {
                return None;
            }
            return Some(construct(item));
        }
    }
    None
}

pub(super) fn parse_declared_value_type(
    type_name: &str,
    aliases: &BTreeMap<String, ScalarType>,
) -> Option<ValueType> {
    let type_name = type_name.trim();
    if let Some(inner) = type_name
        .strip_prefix('(')
        .and_then(|type_name| type_name.strip_suffix(')'))
    {
        return parse_declared_value_type(inner, aliases);
    }
    if is_builtin_error_type(type_name) {
        return Some(ValueType::Object(ObjectIdentity::new(
            "/core/errors",
            type_name,
        )));
    }
    if let Some(scalar) = aliases
        .get(type_name)
        .copied()
        .or_else(|| ScalarType::from_source_name(type_name))
    {
        return Some(ValueType::Scalar(scalar));
    }
    for (constructor, construct) in [
        (
            "overflow-result of ",
            ValueType::OverflowResult as fn(ScalarType) -> ValueType,
        ),
        (
            "div-rem-result of ",
            ValueType::DivRemResult as fn(ScalarType) -> ValueType,
        ),
    ] {
        if let Some(argument) = type_name.strip_prefix(constructor)
            && let Some(scalar) = aliases
                .get(argument.trim())
                .copied()
                .or_else(|| ScalarType::from_source_name(argument.trim()))
        {
            return Some(construct(scalar));
        }
    }
    if type_name == "async-sink-outcome" {
        return Some(ValueType::AsyncSinkOutcome);
    }
    if let Some(argument) = type_name.strip_prefix("async-iteration-step of ") {
        return Some(ValueType::AsyncIterationStep(ElementType::new(
            parse_declared_value_type(argument, aliases)?,
        )));
    }
    if let Some(value_type) = parse_single_argument_value_type(type_name, aliases) {
        return Some(value_type);
    }
    for (constructor, construct) in [
        (
            "map of ",
            ValueType::Map as fn(ElementType, ElementType) -> ValueType,
        ),
        (
            "unordered-map of ",
            ValueType::UnorderedMap as fn(ElementType, ElementType) -> ValueType,
        ),
        (
            "entry of ",
            ValueType::Entry as fn(ElementType, ElementType) -> ValueType,
        ),
    ] {
        if let Some(arguments) = type_name.strip_prefix(constructor)
            && let Some((key, value)) = arguments.split_once(',')
        {
            let key = ElementType::new(parse_declared_value_type(key, aliases)?);
            key.scalar()?;
            let value = ElementType::new(parse_declared_value_type(value, aliases)?);
            return Some(construct(key, value));
        }
    }
    None
}

pub(super) fn union_destination_candidates(
    unit: &SemanticUnit,
    type_node: &SyntaxNode,
) -> Result<Vec<ScalarType>, SemanticFailure> {
    let Some(union) = type_node
        .children
        .first()
        .filter(|child| child.kind == SyntaxKind::UnionType)
    else {
        return Err(failure(
            &unit.source,
            "T0001",
            format!(
                "`{}` does not resolve to a scalar type descriptor",
                node_text(&unit.source, type_node).trim()
            ),
            type_node.span,
        ));
    };
    let mut candidates = Vec::new();
    for arm in &union.children {
        let name = node_text(&unit.source, arm).trim();
        let candidate = unit
            .descriptor_alias_at(name, arm.span.start)
            .ok_or_else(|| {
                failure(
                    &unit.source,
                    "T0001",
                    format!("`{name}` does not resolve to a scalar type descriptor"),
                    arm.span,
                )
            })?;
        if !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    }
    Ok(candidates)
}

pub(super) fn select_union_destination(
    unit: &SemanticUnit,
    type_node: &SyntaxNode,
    value: &SyntaxNode,
    actual: ValueType,
) -> Result<ScalarType, SemanticFailure> {
    select_union_candidates(
        &unit.source,
        value,
        actual,
        union_destination_candidates(unit, type_node)?,
    )
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "union selection owns the inferred recursive value type for complete matching"
)]
pub(super) fn select_union_candidates(
    source: &SourceFile,
    value: &SyntaxNode,
    actual: ValueType,
    candidates: Vec<ScalarType>,
) -> Result<ScalarType, SemanticFailure> {
    let is_constant = candidates
        .iter()
        .any(|candidate| contextual_constant(source, value, *candidate).is_some());
    if !is_constant
        && let ValueType::Scalar(actual) = actual
        && candidates.contains(&actual)
    {
        return Ok(actual);
    }
    let admitted = candidates
        .into_iter()
        .filter(|candidate| {
            if let Some(result) = contextual_constant(source, value, *candidate) {
                return result.is_ok();
            }
            matches!(actual, ValueType::Scalar(actual) if is_numeric(actual) && is_numeric(*candidate))
        })
        .collect::<Vec<_>>();
    match admitted.as_slice() {
        [candidate] => Ok(*candidate),
        [] => Err(failure(
            source,
            "T0002",
            "value is not admitted by any union destination arm",
            value.span,
        )),
        candidates => Err(failure(
            source,
            "T0002",
            format!(
                "numeric destination is ambiguous between {}",
                candidates
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            value.span,
        )),
    }
}

pub(super) fn validate_numeric_destination(
    source: &SourceFile,
    name: &str,
    expected: ScalarType,
    actual: ValueType,
    value: &SyntaxNode,
    mismatch_code: &'static str,
) -> Result<(), SemanticFailure> {
    let ValueType::Scalar(actual) = actual else {
        return Err(failure(
            source,
            mismatch_code,
            destination_mismatch_message(mismatch_code, name, expected, actual),
            value.span,
        ));
    };
    if is_numeric(expected)
        && let Some(constant) = contextual_constant(source, value, expected)
    {
        constant?;
        return Ok(());
    }
    if actual == expected {
        return Ok(());
    }
    if is_numeric(actual) && is_numeric(expected) {
        return Ok(());
    }
    Err(failure(
        source,
        mismatch_code,
        destination_mismatch_message(mismatch_code, name, expected, ValueType::Scalar(actual)),
        value.span,
    ))
}

pub(super) fn diagnostic_object_identity(
    objects: &[DescriptorContract],
    identity: &ObjectIdentity,
) -> String {
    let identities = objects
        .iter()
        .filter(|object| object.identity.name == identity.name)
        .map(|object| &object.identity)
        .collect::<BTreeSet<_>>();
    if identities.len() > 1 {
        identity.qualified()
    } else {
        identity.name.clone()
    }
}

pub(super) fn diagnostic_value_type(
    objects: &[DescriptorContract],
    value_type: &ValueType,
) -> String {
    let nested = |item: &ElementType| diagnostic_value_type(objects, &item.value_type());
    match value_type {
        ValueType::Optional(inner) => {
            format!("{}|none", diagnostic_value_type(objects, inner))
        }
        ValueType::Object(identity) => diagnostic_object_identity(objects, identity),
        ValueType::Iterator(item) => format!("iterator of {}", nested(item)),
        ValueType::IterationStep(item) => format!("iteration-step of {}", nested(item)),
        ValueType::IterationEnd => "iteration-step.end".to_owned(),
        ValueType::AsyncIterationStep(item) => {
            format!("async-iteration-step of {}", nested(item))
        }
        ValueType::List(item) => format!("list of {}", nested(item)),
        ValueType::Map(key, value) => format!("map of {}, {}", nested(key), nested(value)),
        ValueType::Set(item) => format!("set of {}", nested(item)),
        ValueType::Tuple(item, _) => format!("tuple of {}", nested(item)),
        ValueType::Entry(key, value) => format!("entry of {}, {}", nested(key), nested(value)),
        ValueType::UnorderedMap(key, value) => {
            format!("unordered-map of {}, {}", nested(key), nested(value))
        }
        ValueType::UnorderedSet(item) => format!("unordered-set of {}", nested(item)),
        ValueType::Function(parameters, result, effects)
        | ValueType::AsyncFunction(parameters, result, _, effects) => {
            let asynchronous = if matches!(value_type, ValueType::AsyncFunction(..)) {
                "async "
            } else {
                ""
            };
            let prefix = format!(
                "{}{asynchronous}function",
                effects.modes.written.source_prefix()
            );
            let parameters = parameters.iter().map(nested).collect::<Vec<_>>().join(", ");
            let from = if parameters.is_empty() {
                String::new()
            } else {
                format!(" from {parameters}")
            };
            let throws = effects
                .upper_bound
                .as_deref()
                .map_or_else(String::new, |bound| {
                    format!(" throws {}", diagnostic_value_type(objects, bound))
                });
            format!("{prefix}{from} to {}{throws}", nested(result))
        }
        ValueType::Task(result, _) => format!("task of {}", nested(result)),
        ValueType::ScopedTask(result, _) => format!("scoped task of {}", nested(result)),
        ValueType::TaskOutcome(result) => format!("task-outcome of {}", nested(result)),
        ValueType::Reference(item) => format!("ref {}", nested(item)),
        ValueType::SharedReference(item) => format!("shared ref {}", nested(item)),
        _ => value_type.to_string(),
    }
}

fn merge_callable_destination_effects(expected: ValueType, actual: &ValueType) -> ValueType {
    match (expected, actual) {
        (
            ValueType::Function(parameters, result, mut effects),
            ValueType::Function(_, _, actual_effects),
        ) => {
            effects.escaping.clone_from(&actual_effects.escaping);
            effects.modes.exact = actual_effects.modes.exact;
            ValueType::Function(parameters, result, effects)
        }
        (
            ValueType::AsyncFunction(parameters, result, transferability, mut effects),
            ValueType::AsyncFunction(_, _, _, actual_effects),
        ) => {
            effects.escaping.clone_from(&actual_effects.escaping);
            effects.modes.exact = actual_effects.modes.exact;
            ValueType::AsyncFunction(parameters, result, transferability, effects)
        }
        (expected, _) => expected,
    }
}

fn throwable_identity_type(identity: &str) -> Option<ValueType> {
    let (namespace, name) = identity.rsplit_once("::")?;
    Some(ValueType::Object(ObjectIdentity::new(namespace, name)))
}

fn callable_effects_compatible(
    objects: &[DescriptorContract],
    expected: &CallableEffects,
    actual: &CallableEffects,
) -> bool {
    let accepts = |actual: &ValueType| {
        expected
            .upper_bound
            .as_deref()
            .is_some_and(|bound| value_types_compatible(objects, bound, actual))
            || expected.escaping.iter().any(|bound| {
                throwable_identity_type(bound)
                    .is_some_and(|bound| value_types_compatible(objects, &bound, actual))
            })
    };
    if let Some(actual) = actual.upper_bound.as_deref() {
        return accepts(actual);
    }
    actual
        .escaping
        .iter()
        .all(|identity| throwable_identity_type(identity).is_some_and(|actual| accepts(&actual)))
}

pub(super) fn validate_value_destination(
    source: &SourceFile,
    objects: &[DescriptorContract],
    name: &str,
    expected: ValueType,
    actual: ValueType,
    value: &SyntaxNode,
    mismatch_code: &'static str,
) -> Result<(), SemanticFailure> {
    if let ValueType::Scalar(expected) = expected {
        return validate_numeric_destination(source, name, expected, actual, value, mismatch_code);
    }
    if let ValueType::Optional(expected_inner) = expected {
        if actual == ValueType::Scalar(ScalarType::None) {
            return Ok(());
        }
        if let ValueType::Optional(actual_inner) = actual {
            return validate_value_destination(
                source,
                objects,
                name,
                *expected_inner,
                *actual_inner,
                value,
                mismatch_code,
            );
        }
        return validate_value_destination(
            source,
            objects,
            name,
            *expected_inner,
            actual,
            value,
            mismatch_code,
        );
    }
    if value_types_compatible(objects, &expected, &actual) {
        return Ok(());
    }
    Err(failure(
        source,
        mismatch_code,
        format!(
            "`{name}` requires `{}`, found `{}`",
            diagnostic_value_type(objects, &expected),
            diagnostic_value_type(objects, &actual)
        ),
        value.span,
    ))
}

fn callable_types_compatible(
    objects: &[DescriptorContract],
    expected_parameters: &[ElementType],
    expected_result: &ElementType,
    expected_effects: &CallableEffects,
    actual_parameters: &[ElementType],
    actual_result: &ElementType,
    actual_effects: &CallableEffects,
) -> bool {
    expected_effects
        .modes
        .written
        .accepts(actual_effects.modes.written)
        && expected_parameters == actual_parameters
        && expected_result == actual_result
        && callable_effects_compatible(objects, expected_effects, actual_effects)
}

pub(super) fn object_types_compatible(
    objects: &[DescriptorContract],
    expected: &ObjectIdentity,
    actual: &ObjectIdentity,
) -> bool {
    if expected == actual
        || (expected == &ObjectIdentity::new("/core/errors", "throwable")
            && actual.namespace == "/core/errors"
            && is_builtin_error_type(&actual.name))
    {
        return true;
    }
    objects
        .iter()
        .find(|object| object.identity == *actual)
        .is_some_and(|object| {
            if object
                .interfaces
                .iter()
                .any(|interface| interface == expected || interface.base() == *expected)
            {
                return true;
            }
            let mut base = object.base.as_ref();
            while let Some(identity) = base {
                let Some(base_object) = objects.iter().find(|object| object.identity == *identity)
                else {
                    break;
                };
                if base_object.identity == *expected
                    || base_object
                        .interfaces
                        .iter()
                        .any(|interface| interface == expected || interface.base() == *expected)
                {
                    return true;
                }
                base = base_object.base.as_ref();
            }
            false
        })
}

pub(super) fn value_types_compatible(
    objects: &[DescriptorContract],
    expected: &ValueType,
    actual: &ValueType,
) -> bool {
    match (expected, actual) {
        (ValueType::Optional(expected), ValueType::Optional(actual)) => {
            value_types_compatible(objects, expected, actual)
        }
        (ValueType::Tuple(expected_item, None), ValueType::Tuple(actual_item, _)) => {
            value_types_compatible(
                objects,
                &expected_item.value_type(),
                &actual_item.value_type(),
            )
        }
        (
            ValueType::Tuple(expected_item, Some(expected_length)),
            ValueType::Tuple(actual_item, Some(actual_length)),
        ) => {
            expected_length == actual_length
                && value_types_compatible(
                    objects,
                    &expected_item.value_type(),
                    &actual_item.value_type(),
                )
        }
        (ValueType::IterationStep(_), ValueType::IterationEnd) => true,
        (ValueType::List(expected), ValueType::List(actual))
        | (ValueType::Set(expected), ValueType::Set(actual))
        | (ValueType::Iterator(expected), ValueType::Iterator(actual))
        | (ValueType::IterationStep(expected), ValueType::IterationStep(actual))
        | (ValueType::AsyncIterationStep(expected), ValueType::AsyncIterationStep(actual)) => {
            value_types_compatible(objects, &expected.value_type(), &actual.value_type())
        }
        (
            ValueType::Map(expected_key, expected_value),
            ValueType::Map(actual_key, actual_value),
        )
        | (
            ValueType::UnorderedMap(expected_key, expected_value),
            ValueType::UnorderedMap(actual_key, actual_value),
        )
        | (
            ValueType::Entry(expected_key, expected_value),
            ValueType::Entry(actual_key, actual_value),
        ) => {
            value_types_compatible(
                objects,
                &expected_key.value_type(),
                &actual_key.value_type(),
            ) && value_types_compatible(
                objects,
                &expected_value.value_type(),
                &actual_value.value_type(),
            )
        }
        (
            ValueType::Function(expected_parameters, expected_result, expected_effects),
            ValueType::Function(actual_parameters, actual_result, actual_effects),
        ) => callable_types_compatible(
            objects,
            expected_parameters,
            expected_result,
            expected_effects,
            actual_parameters,
            actual_result,
            actual_effects,
        ),
        (
            ValueType::AsyncFunction(
                expected_parameters,
                expected_result,
                expected_transferability,
                expected_effects,
            ),
            ValueType::AsyncFunction(
                actual_parameters,
                actual_result,
                actual_transferability,
                actual_effects,
            ),
        ) => {
            expected_transferability == actual_transferability
                && callable_types_compatible(
                    objects,
                    expected_parameters,
                    expected_result,
                    expected_effects,
                    actual_parameters,
                    actual_result,
                    actual_effects,
                )
        }
        (ValueType::Object(expected), ValueType::Object(actual)) => {
            object_types_compatible(objects, expected, actual)
        }
        _ => expected == actual,
    }
}

pub(super) fn erase_tuple_lengths(value_type: ValueType) -> ValueType {
    match value_type {
        ValueType::Tuple(item, _) => ValueType::Tuple(
            ElementType::new(erase_tuple_lengths(item.value_type())),
            None,
        ),
        ValueType::List(item) => {
            ValueType::List(ElementType::new(erase_tuple_lengths(item.value_type())))
        }
        ValueType::Set(item) => {
            ValueType::Set(ElementType::new(erase_tuple_lengths(item.value_type())))
        }
        ValueType::UnorderedSet(item) => {
            ValueType::UnorderedSet(ElementType::new(erase_tuple_lengths(item.value_type())))
        }
        ValueType::Map(key, value) => ValueType::Map(
            ElementType::new(erase_tuple_lengths(key.value_type())),
            ElementType::new(erase_tuple_lengths(value.value_type())),
        ),
        ValueType::UnorderedMap(key, value) => ValueType::UnorderedMap(
            ElementType::new(erase_tuple_lengths(key.value_type())),
            ElementType::new(erase_tuple_lengths(value.value_type())),
        ),
        ValueType::Entry(key, value) => ValueType::Entry(
            ElementType::new(erase_tuple_lengths(key.value_type())),
            ElementType::new(erase_tuple_lengths(value.value_type())),
        ),
        other => other,
    }
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "diagnostic rendering owns the recursive actual type description"
)]
pub(super) fn destination_mismatch_message(
    code: &str,
    name: &str,
    expected: ScalarType,
    actual: ValueType,
) -> String {
    match code {
        "T0012" => {
            format!("argument for parameter `{name}` has type `{actual}`, expected `{expected}`")
        }
        "T0015" => format!("function `{name}` must return `{expected}`"),
        _ => format!("cannot assign `{actual}` to `{name}` of type `{expected}`"),
    }
}

pub(crate) const fn is_numeric(ty: ScalarType) -> bool {
    ty.is_integer() || matches!(ty, ScalarType::Float32 | ScalarType::Float64)
}

pub(super) fn optional_inner(value_type: ValueType) -> Option<ValueType> {
    match value_type {
        ValueType::Optional(inner) => Some(*inner),
        _ => None,
    }
}

pub(super) fn ungrouped_expression(mut node: &SyntaxNode) -> &SyntaxNode {
    while node.kind == SyntaxKind::GroupExpression {
        let Some(child) = node.children.first() else {
            break;
        };
        node = child;
    }
    node
}

pub(super) fn membership_names<'a>(
    source: &'a SourceFile,
    node: &'a SyntaxNode,
) -> Option<(&'a str, &'a str)> {
    let [left, right] = node.children.as_slice() else {
        return None;
    };
    let left = ungrouped_expression(left);
    let right = ungrouped_expression(right);
    Some((node_text(source, left), node_text(source, right)))
}

pub(super) fn condition_proves_present(source: &SourceFile, node: &SyntaxNode, name: &str) -> bool {
    if node.kind == SyntaxKind::GroupExpression {
        return node
            .children
            .first()
            .is_some_and(|child| condition_proves_present(source, child, name));
    }
    if node.kind == SyntaxKind::BinaryExpression {
        let [left, right] = node.children.as_slice() else {
            return false;
        };
        let operator = source.text()[left.span.end..right.span.start].trim();
        let left = ungrouped_expression(left);
        let right = ungrouped_expression(right);
        let names = (node_text(source, left), node_text(source, right));
        return operator == "!="
            && matches!(names, (left, "none") | ("none", left) if left == name);
    }
    if node.kind == SyntaxKind::UnaryExpression
        && node.children.iter().any(|child| {
            child.kind == SyntaxKind::UnaryOperator && node_text(source, child) == "not"
        })
        && let Some(child) = node
            .children
            .iter()
            .find(|child| child.kind != SyntaxKind::UnaryOperator)
    {
        let child = ungrouped_expression(child);
        return child.kind == SyntaxKind::TypeMembershipExpression
            && membership_names(source, child).is_some_and(|names| names == (name, "none"));
    }
    false
}

pub(super) fn is_presence_test_occurrence(
    source: &SourceFile,
    current: &SyntaxNode,
    position: usize,
    name: &str,
) -> bool {
    if current.kind == SyntaxKind::BinaryExpression
        && condition_proves_present(source, current, name)
    {
        return true;
    }
    current
        .children
        .iter()
        .filter(|child| child.span.start <= position && position <= child.span.end)
        .any(|child| is_presence_test_occurrence(source, child, position, name))
}

pub(super) fn assigns_name_before(
    source: &SourceFile,
    node: &SyntaxNode,
    position: usize,
    name: &str,
) -> bool {
    if node.span.start >= position {
        return false;
    }
    if node.kind == SyntaxKind::Assignment
        && node
            .children
            .first()
            .is_some_and(|target| node_text(source, target) == name)
    {
        return true;
    }
    node.children
        .iter()
        .any(|child| assigns_name_before(source, child, position, name))
}

pub(super) fn enclosed_by_present_guard(
    source: &SourceFile,
    current: &SyntaxNode,
    position: usize,
    name: &str,
) -> bool {
    if current.kind == SyntaxKind::IfStatement
        && let Some(condition) = current.children.first()
        && let Some(block) = current.children.iter().find(|child| {
            child.kind == SyntaxKind::Block
                && child.span.start <= position
                && position <= child.span.end
        })
    {
        if condition_proves_present(source, condition, name)
            && !assigns_name_before(source, block, position, name)
        {
            return true;
        }
        return enclosed_by_present_guard(source, block, position, name);
    }
    current
        .children
        .iter()
        .filter(|child| child.span.start <= position && position <= child.span.end)
        .any(|child| enclosed_by_present_guard(source, child, position, name))
}

pub(crate) fn narrowed_optional_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    value_type: ValueType,
) -> Option<ValueType> {
    let name = node_text(&unit.source, node);
    if is_presence_test_occurrence(&unit.source, &unit.tree.root, node.span.start, name) {
        return None;
    }
    let inner = optional_inner(value_type)?;
    enclosed_by_present_guard(&unit.source, &unit.tree.root, node.span.start, name).then_some(inner)
}

pub(crate) fn narrowed_value_type(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Option<ValueType> {
    let name = node_text(&unit.source, node);
    let function_span = unit
        .enclosing_function_spans
        .get(&node.span.start)
        .copied()
        .flatten();
    let binding = bindings.iter().rev().find(|binding| {
        binding.name == name
            && binding.is_visible_at(unit.source.id(), node.span.start)
            && unit
                .enclosing_function_spans
                .get(&binding.span.start)
                .copied()
                .flatten()
                == function_span
    })?;
    narrowed_optional_type(unit, node, binding.value_type.clone())
}
