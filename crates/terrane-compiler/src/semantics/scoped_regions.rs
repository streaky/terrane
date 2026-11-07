use super::modifiers::ModifierOperation;
use super::prelude::*;

#[path = "scoped_regions/liveness.rs"]
mod liveness;
#[path = "scoped_regions/native_loans.rs"]
mod native_loans;
#[path = "scoped_regions/try_captures.rs"]
mod try_captures;

type NativeLenderPlace = (Span, Vec<ReferenceProjection>);

#[derive(Clone, Copy)]
struct NativePlaceLoan<'a> {
    places: &'a [NativeLenderPlace],
    live_after: Option<&'a BTreeSet<Span>>,
}

/// Validates the lexical and storage contract for compiler-resolved scoped-access regions.
pub(super) fn record_native_lender_owners(package: &mut SemanticPackage) {
    fn visit(unit: &SemanticUnit, node: &SyntaxNode, owners: &mut BTreeSet<Span>) {
        if node.kind == SyntaxKind::CallExpression {
            let key = (node.span.file, node.span.start, node.span.end);
            if let Some(specialization) = unit.projected_call_specializations.get(&key)
                && let crate::rust_interop::projection::ProjectedType::InvocationScoped {
                    lifetimes,
                    ..
                } = &specialization.projected_result
            {
                for (index, parameter) in specialization.projected_parameters.iter().enumerate() {
                    let Some(argument) = node.children.get(1).and_then(|arguments| {
                        super::bindings::projected_argument_for_parameter(
                            unit,
                            &specialization.projected_parameters,
                            arguments,
                            index,
                        )
                    }) else {
                        continue;
                    };
                    let mut source_lifetimes = Vec::new();
                    projected_lifetimes(&parameter.ty, &mut source_lifetimes);
                    if let Some(lifetime) = &parameter.borrow_lifetime {
                        source_lifetimes.push(lifetime.clone());
                    }
                    let matching = lifetimes
                        .iter()
                        .filter(|lifetime| {
                            lifetime.as_str() != "'static"
                                && source_lifetimes.contains(lifetime)
                                && !(lifetime.as_str() == "'_"
                                    && specialization.projected_receiver_lifetime.as_deref()
                                        == Some("'_"))
                        })
                        .cloned()
                        .collect::<Vec<_>>();
                    if !matching.is_empty() {
                        collect_flow_lenders(unit, argument.span, owners);
                    }
                }
                let receiver_borrow = specialization.projected_receiver_lifetime.as_ref();
                let receiver_type = node
                    .children
                    .first()
                    .and_then(|callee| callee.children.first())
                    .and_then(|receiver| {
                        infer_value_type(unit, receiver, &unit.typed_bindings).ok()
                    })
                    .flatten();
                let receiver_lifetimes = match receiver_type.as_ref() {
                    Some(ValueType::InvocationScopedNative { lifetimes, .. }) => {
                        lifetimes.as_slice()
                    }
                    _ => &[],
                };
                let receiver_matching = lifetimes.iter().any(|lifetime| {
                    lifetime != "'static"
                        && (receiver_borrow == Some(lifetime)
                            || receiver_lifetimes.contains(lifetime))
                });
                if receiver_matching
                    && let Some(receiver) = node
                        .children
                        .first()
                        .and_then(|callee| callee.children.first())
                {
                    collect_flow_lenders(unit, receiver.span, owners);
                }
            }
        }
        for child in &node.children {
            visit(unit, child, owners);
        }
    }

    for unit in &mut package.units {
        let mut owners = BTreeSet::new();
        visit(unit, &unit.tree.root, &mut owners);
        unit.reference_owner_spans
            .extend(owners.into_iter().map(|owner| (owner.start, owner.end)));
    }
}

pub(super) fn validate_scoped_regions(package: &SemanticPackage) -> Result<(), SemanticFailure> {
    for unit in &package.units {
        for object in unit
            .descriptors
            .iter()
            .filter(|object| object.span.file == unit.source.id())
        {
            for field in &object.fields {
                if field_has_native_loan(package, &field.value_type) {
                    return Err(failure(
                        &unit.source,
                        "T0101",
                        "lifetime-bearing native values cannot be stored in class fields",
                        field.span,
                    ));
                }
            }
        }
        validate_node(package, unit, &unit.tree.root)?;
        try_captures::validate(package, unit, &unit.tree.root)?;
        native_loans::validate_function_native_lenders(package, unit)?;
    }
    Ok(())
}

pub(super) fn record_completion_capture_slots(package: &mut SemanticPackage) {
    let slots = package
        .units
        .iter()
        .map(|unit| try_captures::completion_capture_slots(package, unit))
        .collect::<Vec<_>>();
    for (unit, slots) in package.units.iter_mut().zip(slots) {
        unit.completion_capture_slots = slots;
    }
}

pub(super) fn field_has_native_loan(package: &SemanticPackage, value_type: &ValueType) -> bool {
    let mut borrowed = false;
    super::capabilities::walk_value_type(value_type, &mut |value_type| match value_type {
        ValueType::InvocationScopedNative { .. } => borrowed = true,
        ValueType::Object(identity) => {
            let item = package.projection.item(&identity.namespace, &identity.name);
            if item.is_some_and(|item| {
                matches!(
                    item.kind,
                    crate::rust_interop::projection::ProjectedKind::ForeignType {
                        borrowed_view: true,
                        ..
                    }
                )
            }) {
                return;
            }
            if let Some(rust) = identity.native_projection.as_deref().or_else(|| {
                item.map(|item| match &item.kind {
                    crate::rust_interop::projection::ProjectedKind::ForeignType {
                        native_view_type: Some(native),
                        ..
                    } => native.as_str(),
                    _ => item.rust_path.as_str(),
                })
            }) {
                borrowed |= crate::rust_ir::rust_lifetimes(rust)
                    .iter()
                    .any(|lifetime| lifetime != "'static");
            }
        }
        _ => {}
    });
    borrowed
}

fn validate_native_return_escapes(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    function: Option<Span>,
) -> Result<(), SemanticFailure> {
    let function = if matches!(
        node.kind,
        SyntaxKind::FunctionDeclaration | SyntaxKind::AnonymousFunction
    ) {
        Some(node.span)
    } else {
        function
    };
    let exact_scoped_result = function.is_some_and(|function| {
        unit.functions.iter().any(|contract| {
            contract.span == function
                && matches!(
                    contract.return_type,
                    Some(ValueType::InvocationScopedNative { .. })
                )
        })
    });
    if node.kind == SyntaxKind::ReturnStatement
        && !exact_scoped_result
        && let Some(value) = node.children.last()
        && infer_value_type(unit, value, &unit.typed_bindings)
            .ok()
            .flatten()
            .is_some_and(|ty| field_has_native_loan(package, &ty))
    {
        let mut lenders = BTreeSet::new();
        let mut places = Vec::new();
        native_loans::collect_native_view_origins(package, unit, value, &mut lenders, &mut places);
        // Direct return expressions have no destination binding from which to
        // recover origins. Their reaching reads still identify lender parameters.
        collect_flow_lenders(unit, value.span, &mut lenders);
        for lender in lenders {
            let Some(binding) = unit
                .typed_bindings
                .iter()
                .find(|binding| binding.span == lender)
            else {
                continue;
            };
            // Passing through a view is not a new loan from parameter storage.
            if field_has_native_loan(package, &binding.value_type) {
                continue;
            }
            let external = matches!(
                binding.value_type,
                ValueType::Reference(_) | ValueType::MutableReference(_)
            ) || unit
                .reference_provenance
                .get(&(binding.span.start, binding.span.end))
                .is_some_and(|origin| origin.external_lender);
            if external
                && function.is_some_and(|function| {
                    unit.functions.iter().any(|contract| {
                        contract.span == function
                            && contract.return_type.as_ref().is_some_and(|ty| {
                                matches!(
                                    ty,
                                    ValueType::Object(_) | ValueType::InvocationScopedNative { .. }
                                ) && field_has_native_loan(package, ty)
                            })
                    })
                })
            {
                // Scalar borrowed helper results have a supported native ABI.
                // Collection-producing returns do not share that contract.
                return Ok(());
            }
            return Err(failure(
                &unit.source,
                if external { "T0119" } else { "T0240" },
                if external {
                    "returning a native view produced from a lender parameter is unsupported"
                } else {
                    "a native loan of local storage cannot escape through a return"
                },
                value.span,
            ));
        }
    }
    if node.kind == SyntaxKind::ReturnStatement
        && let Some(function) = function
        && !exact_scoped_result
        && let Some(value) = node.children.last()
        && infer_value_type(unit, value, &unit.typed_bindings)
            .ok()
            .flatten()
            .and_then(|value_type| scoped_region(&value_type))
            == Some((function.file, function.start, function.end))
    {
        return Err(failure(
            &unit.source,
            "T0240",
            "a local native loan cannot escape its function through a return",
            value.span,
        ));
    }
    for child in &node.children {
        validate_native_return_escapes(package, unit, child, function)?;
    }
    Ok(())
}

fn validate_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    if node.kind == SyntaxKind::WithRegion {
        validate_region(package, unit, node)?;
    }
    for child in &node.children {
        validate_node(package, unit, child)?;
    }
    Ok(())
}

fn validate_region(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    region: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let invalid = |span, message: &'static str| failure(&unit.source, "T0240", message, span);
    if unit.containing_function(region.span).is_none() {
        return Err(invalid(
            region.span,
            "scoped-access regions are only valid inside a function",
        ));
    }
    let [acquisition, body] = region.children.as_slice() else {
        return Err(invalid(
            region.span,
            "scoped-access requires one acquisition and one body block",
        ));
    };
    if !matches!(
        acquisition.kind,
        SyntaxKind::Binding | SyntaxKind::Assignment
    ) || body.kind != SyntaxKind::Block
    {
        return Err(invalid(
            region.span,
            "scoped-access requires a binding or assignment followed by a body block",
        ));
    }
    let modifiers = package.declaration_modifiers.get(&region.span);
    if !modifiers.is_some_and(|applications| {
        applications.len() == 1 && applications[0].operation == ModifierOperation::ScopedAccess
    }) {
        return Err(invalid(
            region.span,
            "region requires exactly one resolved scoped-access modifier",
        ));
    }

    let binding = declared_bindings_at_node(unit, acquisition)
        .find(|binding| binding.name != "_" && binding.scope == Some(region.span))
        .ok_or_else(|| {
            invalid(
                acquisition.span,
                "scoped-access acquisition must introduce a region-local binding",
            )
        })?;
    let mut places = Vec::new();
    let mut owner_lenders =
        scoped_native_lender_spans(package, unit, acquisition, binding, &mut places, true)
            .ok_or_else(|| {
                invalid(
                    acquisition.span,
                    "cannot establish the scoped-access guard's source lifetime",
                )
            })?;
    owner_lenders.sort_unstable();
    owner_lenders.dedup();
    let mut lenders = vec![binding.span];
    let mut cursor = 0;
    while cursor < lenders.len() {
        let lender = lenders[cursor];
        cursor += 1;
        for candidate in &unit.typed_bindings {
            let Some(provenance) = unit
                .reference_provenance
                .get(&(candidate.span.start, candidate.span.end))
            else {
                continue;
            };
            if provenance.owner != lender || lenders.contains(&candidate.span) {
                continue;
            }
            if candidate.span.file != region.span.file
                || candidate.span.start < region.span.start
                || candidate.span.end > region.span.end
            {
                return Err(invalid(
                    candidate.span,
                    "a reference depending on scoped-access cannot outlive its region",
                ));
            }
            lenders.push(candidate.span);
        }
    }
    let body = &region.children[1];
    validate_lender_stability(
        package,
        unit,
        region.span,
        body,
        Some(binding.span),
        &owner_lenders,
        NativePlaceLoan {
            places: &places,
            live_after: None,
        },
    )?;
    validate_local_lender_destinations(unit, region.span, body, &lenders)?;
    validate_region_escape_node(package, unit, region.span, body, &mut lenders)?;
    Ok(())
}
fn validate_local_lender_destinations(
    unit: &SemanticUnit,
    region: Span,
    node: &SyntaxNode,
    lenders: &[Span],
) -> Result<(), SemanticFailure> {
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment) {
        let declared = declared_bindings_at_node(unit, node)
            .map(|binding| binding.span)
            .collect::<Vec<_>>();
        let destination_spans = if declared.is_empty() {
            node.children
                .first()
                .and_then(|target| binding_identity_at(unit, target))
                .into_iter()
                .collect()
        } else {
            declared
        };
        if let Some(value) = node.children.last()
            && (infer_value_type(unit, value, &unit.typed_bindings)
                .ok()
                .flatten()
                .and_then(|value_type| scoped_region(&value_type))
                == Some((region.file, region.start, region.end))
                || (references_lender(unit, value, lenders)
                    && expression_has_lender_provenance(unit, value, lenders)))
        {
            for destination_span in destination_spans {
                let destination = unit
                    .typed_bindings
                    .iter()
                    .find(|binding| binding.span == destination_span);
                let Some(destination_scope) = destination.and_then(|binding| binding.scope) else {
                    return Err(failure(
                        &unit.source,
                        "T0240",
                        "cannot establish a native value's destination scope",
                        value.span,
                    ));
                };
                for lender in lenders {
                    let lender_scope = unit
                        .typed_bindings
                        .iter()
                        .find(|binding| binding.span == *lender)
                        .and_then(|binding| binding.scope);
                    let Some(lender_scope) = lender_scope else {
                        return Err(failure(
                            &unit.source,
                            "T0119",
                            "cannot establish a native lender's lexical scope",
                            value.span,
                        ));
                    };
                    if destination_scope.file != lender_scope.file
                        || destination_scope.start < lender_scope.start
                        || destination_scope.end > lender_scope.end
                    {
                        return Err(failure(
                            &unit.source,
                            "T0240",
                            "a native value cannot outlive the lexical scope of its lender",
                            value.span,
                        ));
                    }
                }
            }
        }
    }
    for child in &node.children {
        validate_local_lender_destinations(unit, region, child, lenders)?;
    }
    Ok(())
}

fn binding_identity_at(unit: &SemanticUnit, name: &SyntaxNode) -> Option<Span> {
    unit.flow_binding_ids
        .get(&(name.span.file, name.span.start, name.span.end))
        .copied()
        .or_else(|| {
            let text = node_text(&unit.source, name);
            unit.typed_bindings
                .iter()
                .filter(|binding| {
                    binding.name == text && binding.is_visible_at(unit.source.id(), name.span.start)
                })
                .max_by_key(|binding| binding.visible_from)
                .map(|binding| binding.span)
        })
}

fn directly_uses_owner(unit: &SemanticUnit, node: &SyntaxNode, owners: &[Span]) -> bool {
    match (node.kind, node.children.as_slice()) {
        (SyntaxKind::GroupExpression, [inner]) | (SyntaxKind::UnaryExpression, [_, inner])
            if node.kind == SyntaxKind::GroupExpression
                || unary_operator_text(unit, node).as_deref() == Some("move") =>
        {
            directly_uses_owner(unit, inner, owners)
        }
        (SyntaxKind::Name, _) => {
            binding_identity_at(unit, node).is_some_and(|span| owners.contains(&span))
        }
        _ => false,
    }
}

fn fixed_call_uses_lender_incompatibly(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    guard: Option<Span>,
    owners: &[Span],
    places: &[NativeLenderPlace],
) -> bool {
    let is_lender = |expression: &SyntaxNode| {
        direct_name_identity(unit, expression)
            .is_some_and(|identity| Some(identity) == guard || owners.contains(&identity))
            || place_conflicts_with_lender(package, unit, expression, places)
    };
    let is_owner = |expression: &SyntaxNode| {
        direct_name_identity(unit, expression).is_some_and(|identity| owners.contains(&identity))
            || place_conflicts_with_lender(package, unit, expression, places)
    };
    if node
        .children
        .first()
        .and_then(|callee| super::diagnostics::typed_member_call(unit, callee))
        .is_some_and(|(receiver, ty, member)| {
            let mode = super::diagnostics::member_invocation_mode_at(
                package, unit, receiver, &ty, &member,
            );
            (mode == InvocationMode::Mutable && is_owner(receiver))
                || (mode == InvocationMode::Consuming && is_lender(receiver))
        })
    {
        return true;
    }
    let Some(contract) = super::calls::selected_callable_contract(
        package,
        unit,
        node,
        crate::syntax::call_is_unsafe(node),
    ) else {
        return false;
    };
    let mut next_positional = 0;
    node.children.get(1).is_some_and(|arguments| {
        arguments.children.iter().any(|argument| {
            let (name, expression) = if argument.kind == SyntaxKind::Argument
                && argument.children.len() > 1
                && argument.children[0].kind == SyntaxKind::Name
            {
                (
                    Some(node_text(&unit.source, &argument.children[0]).to_owned()),
                    argument.children.last().unwrap(),
                )
            } else {
                let expression = if argument.kind == SyntaxKind::Argument {
                    argument.children.last().unwrap_or(argument)
                } else {
                    argument
                };
                let index = next_positional;
                next_positional += 1;
                (
                    contract.parameters.get(index).map(|p| p.name.clone()),
                    expression,
                )
            };
            let parameter = name
                .as_deref()
                .and_then(|name| contract.parameters.iter().find(|p| p.name == name));
            parameter.is_some_and(|parameter| {
                let borrowed = parameter.value_type.as_ref().is_some_and(|value_type| {
                    matches!(
                        value_type,
                        ValueType::Reference(_) | ValueType::SharedReference(_)
                    )
                });
                let mutable = parameter
                    .value_type
                    .as_ref()
                    .is_some_and(|value_type| matches!(value_type, ValueType::MutableReference(_)));
                (mutable && is_owner(expression))
                    || (!borrowed && !mutable && is_lender(expression))
            })
        })
    })
}

fn call_uses_lender_incompatibly(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    node: &SyntaxNode,
    guard: Option<Span>,
    owners: &[Span],
    places: &[NativeLenderPlace],
) -> bool {
    use crate::rust_interop::projection::Receiver;
    if node.kind != SyntaxKind::CallExpression {
        return false;
    }
    let is_lender = |expression: &SyntaxNode| {
        direct_name_identity(unit, expression)
            .is_some_and(|identity| Some(identity) == guard || owners.contains(&identity))
            || place_conflicts_with_lender(package, unit, expression, places)
    };
    // The acquired value may be reborrowed according to its contract. Its
    // backing owners, in contrast, cannot be mutably accessed under the loan.
    let is_owner = |expression: &SyntaxNode| {
        direct_name_identity(unit, expression).is_some_and(|identity| owners.contains(&identity))
            || place_conflicts_with_lender(package, unit, expression, places)
    };
    let key = (node.span.file, node.span.start, node.span.end);
    let Some(specialization) = unit.projected_call_specializations.get(&key) else {
        return fixed_call_uses_lender_incompatibly(package, unit, node, guard, owners, places);
    };
    let callee = node.children.first();
    let receiver = callee.and_then(|callee| callee.children.first());
    let receiver_conflict = receiver.is_some_and(|receiver| {
        specialization.projected_receiver.is_some_and(|mode| {
            (matches!(mode, Receiver::MutableBorrow) && is_owner(receiver))
                || (mode.consumes() && is_lender(receiver))
        })
    });
    let mut next_positional = 0;
    let argument_conflict = node.children.get(1).is_some_and(|arguments| {
        arguments.children.iter().any(|argument| {
            let (name, expression) = if argument.kind == SyntaxKind::Argument
                && argument.children.len() > 1
                && argument.children[0].kind == SyntaxKind::Name
            {
                (
                    Some(node_text(&unit.source, &argument.children[0]).to_owned()),
                    argument.children.last().unwrap(),
                )
            } else {
                let expression = if argument.kind == SyntaxKind::Argument {
                    argument.children.last().unwrap_or(argument)
                } else {
                    argument
                };
                let index = next_positional;
                next_positional += 1;
                (
                    specialization
                        .projected_parameters
                        .get(index)
                        .map(|p| p.name.clone()),
                    expression,
                )
            };
            let parameter = name.as_deref().and_then(|name| {
                specialization
                    .projected_parameters
                    .iter()
                    .find(|p| p.name == name)
            });
            parameter.is_some_and(|parameter| {
                (parameter.mutable_borrow && is_owner(expression))
                    || (!parameter.borrows_argument() && is_lender(expression))
            })
        })
    });
    receiver_conflict || argument_conflict
}

fn validate_lender_stability(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    region: Span,
    node: &SyntaxNode,
    guard: Option<Span>,
    owners: &[Span],
    loan: NativePlaceLoan<'_>,
) -> Result<(), SemanticFailure> {
    if node.span.end <= region.start || node.span.start >= region.end {
        return Ok(());
    }
    let live = loan
        .live_after
        .is_none_or(|live_after| live_after.contains(&node.span));
    let active_places = if live { loan.places } else { &[] };
    let active_owners = if live { owners } else { &[] };
    let active_guard = if live { guard } else { None };
    let owner_transfer = matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && node.children.last().is_some_and(|value| {
            direct_name_identity(unit, value).is_some_and(|identity| {
                Some(identity) == active_guard || active_owners.contains(&identity)
            })
        });
    let destructive_move = owner_transfer
        || (node.kind == SyntaxKind::UnaryExpression
            && unary_operator_text(unit, node).as_deref() == Some("move")
            && node.children.last().is_some_and(|operand| {
                directly_uses_owner(unit, operand, active_owners)
                    || direct_name_identity(unit, operand)
                        .is_some_and(|identity| Some(identity) == active_guard)
                    || place_conflicts_with_lender(package, unit, operand, active_places)
            }))
        || call_uses_lender_incompatibly(
            package,
            unit,
            node,
            active_guard,
            active_owners,
            active_places,
        );
    let owner_replacement = node.kind == SyntaxKind::Assignment
        && node.children.first().is_some_and(|target| {
            target.kind == SyntaxKind::Name
                && !matches!(
                    unit.inferred_value_type(target),
                    Some(ValueType::MutableReference(_))
                )
        })
        && node
            .children
            .first()
            .and_then(|target| binding_identity_at(unit, target))
            .is_some_and(|span| Some(span) == active_guard || active_owners.contains(&span));
    let place_replacement = node.kind == SyntaxKind::Assignment
        && node.children.first().is_some_and(|target| {
            matches!(
                target.kind,
                SyntaxKind::MemberExpression | SyntaxKind::IndexExpression
            ) && place_conflicts_with_lender(package, unit, target, active_places)
        });
    if destructive_move || owner_replacement || place_replacement {
        return Err(failure(
            &unit.source,
            "T0240",
            "a scoped-access guard or lender cannot be moved, replaced, or dropped while the region is active",
            node.span,
        ));
    }
    for child in &node.children {
        validate_lender_stability(package, unit, region, child, guard, owners, loan)?;
    }
    Ok(())
}

fn place_conflicts_with_lender(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    expression: &SyntaxNode,
    places: &[NativeLenderPlace],
) -> bool {
    if places.is_empty() {
        return false;
    }
    super::calls::call_reference_origin(package, unit, expression).is_some_and(|(owner, path)| {
        places.iter().any(|(lender, borrowed)| {
            owner == *lender && super::calls::reference_paths_overlap(&path, borrowed)
        })
    })
}

fn direct_name_identity(unit: &SemanticUnit, node: &SyntaxNode) -> Option<Span> {
    match node.kind {
        SyntaxKind::Name => binding_identity_at(unit, node),
        SyntaxKind::GroupExpression => node
            .children
            .first()
            .and_then(|inner| direct_name_identity(unit, inner)),
        _ => None,
    }
}
fn scoped_region(value_type: &ValueType) -> Option<(u32, usize, usize)> {
    match value_type {
        ValueType::InvocationScopedNative { region, .. } => *region,
        ValueType::Optional(inner) => scoped_region(inner),
        ValueType::Union(items) => items.iter().find_map(scoped_region),
        ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner)
        | ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Task(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskGroup(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner) => scoped_region(inner.value_type_ref()),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            scoped_region(key.value_type_ref()).or_else(|| scoped_region(value.value_type_ref()))
        }
        ValueType::Tuple(item, _) => scoped_region(item.value_type_ref()),
        _ => None,
    }
}

fn directly_uses_lender(unit: &SemanticUnit, node: &SyntaxNode, lenders: &[Span]) -> bool {
    match (node.kind, node.children.as_slice()) {
        (SyntaxKind::GroupExpression, [inner]) | (SyntaxKind::UnaryExpression, [_, inner])
            if node.kind == SyntaxKind::GroupExpression
                || unary_operator_text(unit, node).as_deref() == Some("move") =>
        {
            directly_uses_lender(unit, inner, lenders)
        }
        (SyntaxKind::Name, _) => references_lender(unit, node, lenders),
        _ => false,
    }
}

fn references_lender(unit: &SemanticUnit, node: &SyntaxNode, lenders: &[Span]) -> bool {
    (node.kind == SyntaxKind::Name
        && unit
            .flow_binding_ids
            .get(&(node.span.file, node.span.start, node.span.end))
            .is_some_and(|identity| lenders.contains(identity)))
        || node
            .children
            .iter()
            .any(|child| references_lender(unit, child, lenders))
}

fn expression_has_lender_provenance(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    lenders: &[Span],
) -> bool {
    unit.reference_provenance
        .get(&(node.span.start, node.span.end))
        .is_some_and(|provenance| lenders.contains(&provenance.owner))
        || node
            .children
            .iter()
            .any(|child| expression_has_lender_provenance(unit, child, lenders))
}

fn contains_nested_type(container: &ValueType, lender: &ValueType) -> bool {
    match container {
        ValueType::Optional(inner) => {
            inner.as_ref() == lender || contains_nested_type(inner, lender)
        }
        ValueType::Union(items) => items
            .iter()
            .any(|item| item == lender || contains_nested_type(item, lender)),
        ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner)
        | ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Task(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskGroup(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner) => {
            inner.value_type_ref() == lender || contains_nested_type(inner.value_type_ref(), lender)
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            key.value_type_ref() == lender
                || value.value_type_ref() == lender
                || contains_nested_type(key.value_type_ref(), lender)
                || contains_nested_type(value.value_type_ref(), lender)
        }
        ValueType::Tuple(item, _) => {
            item.value_type_ref() == lender || contains_nested_type(item.value_type_ref(), lender)
        }
        _ => false,
    }
}

fn projected_lifetimes(
    projected: &crate::rust_interop::projection::ProjectedType,
    lifetimes: &mut Vec<String>,
) {
    use crate::rust_interop::projection::ProjectedType;
    match projected {
        ProjectedType::Reference {
            inner, lifetime, ..
        } => {
            if let Some(lifetime) = lifetime {
                lifetimes.push(lifetime.clone());
            }
            projected_lifetimes(inner, lifetimes);
        }
        ProjectedType::Sequence { item, .. }
        | ProjectedType::Set { item, .. }
        | ProjectedType::Optional(item)
        | ProjectedType::AsyncIterationStep(item) => projected_lifetimes(item, lifetimes),
        ProjectedType::Mapping { key, value, .. } => {
            projected_lifetimes(key, lifetimes);
            projected_lifetimes(value, lifetimes);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                projected_lifetimes(item, lifetimes);
            }
        }
        ProjectedType::InvocationScoped {
            lifetimes: nested, ..
        } => lifetimes.extend(nested.iter().cloned()),
        _ => {}
    }
}

fn call_returning_scoped_native<'a>(
    unit: &SemanticUnit,
    node: &'a SyntaxNode,
    rust_type: &str,
) -> Option<&'a SyntaxNode> {
    if unit
        .projected_call_specializations
        .get(&(node.span.file, node.span.start, node.span.end))
        .is_some_and(|specialization| {
            matches!(
                &specialization.projected_result,
                crate::rust_interop::projection::ProjectedType::InvocationScoped {
                    rust_type: result,
                    ..
                } if result == rust_type || matches!(
                    (syn::parse_str::<syn::Type>(result), syn::parse_str::<syn::Type>(rust_type)),
                    (Ok(result), Ok(expected)) if result == expected
                )
            )
        })
    {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| call_returning_scoped_native(unit, child, rust_type))
}

fn ordinary_acquisition_lenders(
    unit: &SemanticUnit,
    binding: &TypedBinding,
    places: &mut Vec<NativeLenderPlace>,
) -> Option<Vec<Span>> {
    match &binding.value_type {
        ValueType::Reference(_) | ValueType::MutableReference(_) => unit
            .reference_provenance
            .get(&(binding.span.start, binding.span.end))
            .map(|provenance| {
                places.push((provenance.owner, provenance.path.clone()));
                vec![provenance.owner]
            }),
        // Owned source values use ordinary destruction at the region
        // boundary. They do not acquire native lifetime parameters merely
        // by appearing in a scoped-access acquisition.
        _ => Some(Vec::new()),
    }
}

struct LenderOrigins {
    lenders: BTreeSet<Span>,
    matched: BTreeSet<String>,
    forwarded: bool,
}

fn argument_lender_origins(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    call: &SyntaxNode,
    specialization: &super::model::ProjectedCallSpecialization,
    lifetimes: &[String],
    places: &mut Vec<NativeLenderPlace>,
    expand_initializers: bool,
) -> LenderOrigins {
    let mut lenders = BTreeSet::new();
    let mut matched = BTreeSet::new();
    let mut forwarded = false;
    if let Some(arguments) = call.children.get(1) {
        for (index, parameter) in specialization.projected_parameters.iter().enumerate() {
            let Some(argument) = super::bindings::projected_argument_for_parameter(
                unit,
                &specialization.projected_parameters,
                arguments,
                index,
            ) else {
                continue;
            };
            let mut parameter_lifetimes = Vec::new();
            projected_lifetimes(&parameter.ty, &mut parameter_lifetimes);
            if let Some(lifetime) = &parameter.borrow_lifetime {
                parameter_lifetimes.push(lifetime.clone());
            }
            for lifetime in lifetimes.iter().filter(|lifetime| {
                lifetime.as_str() != "'static"
                    && parameter_lifetimes.contains(lifetime)
                    && !(lifetime.as_str() == "'_"
                        && specialization.projected_receiver_lifetime.as_deref() == Some("'_"))
            }) {
                matched.insert(lifetime.clone());
                if !expand_initializers
                    && parameter.borrow_lifetime.as_ref() != Some(lifetime)
                    && matches!(
                        infer_value_type(unit, argument, &unit.typed_bindings)
                            .ok()
                            .flatten(),
                        Some(ValueType::InvocationScopedNative { .. })
                    )
                {
                    forwarded = true;
                } else {
                    collect_native_lender_place(
                        package,
                        unit,
                        argument,
                        &mut lenders,
                        places,
                        expand_initializers,
                    );
                }
            }
        }
    }
    LenderOrigins {
        lenders,
        matched,
        forwarded,
    }
}

fn scoped_native_lender_spans(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    acquisition: &SyntaxNode,
    binding: &TypedBinding,
    places: &mut Vec<NativeLenderPlace>,
    expand_initializers: bool,
) -> Option<Vec<Span>> {
    let ValueType::InvocationScopedNative {
        rust_type,
        concrete: true,
        family,
        lifetimes: output_lifetimes,
        expression_scoped: false,
        ..
    } = &binding.value_type
    else {
        return ordinary_acquisition_lenders(unit, binding, places);
    };
    family.native_projection.as_ref()?;
    if !output_lifetimes
        .iter()
        .any(|lifetime| lifetime != "'static")
    {
        return Some(Vec::new());
    }
    let call = call_returning_scoped_native(unit, acquisition, rust_type)?;
    let specialization = unit.projected_call_specializations.get(&(
        call.span.file,
        call.span.start,
        call.span.end,
    ))?;
    let crate::rust_interop::projection::ProjectedType::InvocationScoped { lifetimes, .. } =
        &specialization.projected_result
    else {
        return None;
    };
    let callee = call.children.first()?;
    let LenderOrigins {
        mut lenders,
        mut matched,
        mut forwarded,
    } = argument_lender_origins(
        package,
        unit,
        call,
        specialization,
        lifetimes,
        places,
        expand_initializers,
    );
    if let Some(receiver_lifetime) = &specialization.projected_receiver_lifetime {
        for lifetime in lifetimes
            .iter()
            .filter(|lifetime| lifetime.as_str() != "'static" && *lifetime == receiver_lifetime)
        {
            matched.insert(lifetime.clone());
            if let Some(receiver) = callee.children.first() {
                collect_native_lender_place(
                    package,
                    unit,
                    receiver,
                    &mut lenders,
                    places,
                    expand_initializers,
                );
            }
        }
    }
    if let Some(receiver) = callee.children.first()
        && let Some(ValueType::InvocationScopedNative {
            lifetimes: receiver_lifetimes,
            ..
        }) = infer_value_type(unit, receiver, &unit.typed_bindings)
            .ok()
            .flatten()
    {
        for lifetime in lifetimes.iter().filter(|lifetime| {
            lifetime.as_str() != "'static" && receiver_lifetimes.contains(lifetime)
        }) {
            matched.insert(lifetime.clone());
            if expand_initializers {
                collect_native_lender_place(package, unit, receiver, &mut lenders, places, true);
            } else {
                // Reaching values retain the current producer, including after
                // reassignment. The binding's first initializer is not its loan.
                forwarded = true;
            }
        }
    }
    if lifetimes
        .iter()
        .any(|lifetime| lifetime != "'static" && !matched.contains(lifetime))
        || (lenders.is_empty() && !forwarded)
    {
        None
    } else {
        Some(lenders.into_iter().collect())
    }
}

fn collect_native_lender_place(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    value: &SyntaxNode,
    lenders: &mut BTreeSet<Span>,
    places: &mut Vec<NativeLenderPlace>,
    expand_initializers: bool,
) {
    // A declaration identity alone is not a local lender. Implicit receivers
    // have synthesized zero-width identities, not ordinary storage provenance.
    if let Some(place) =
        super::calls::call_reference_origin(package, unit, value).filter(|(owner, _)| {
            owner.start < owner.end
                && unit
                    .typed_bindings
                    .iter()
                    .any(|binding| binding.span == *owner)
        })
    {
        lenders.insert(place.0);
        if expand_initializers && place.1.is_empty()
            && let Some(binding) = unit.typed_bindings.iter().find(|binding| {
                binding.span == place.0
                    && matches!(
                        binding.value_type,
                        ValueType::InvocationScopedNative { .. }
                    )
            })
            && let Some(initializer) =
                find_binding_initializer(&unit.tree.root, binding.span)
            // Only follow an earlier producer, never a self-reference or backedge.
            && initializer.span.end <= value.span.start
            && let Some(origins) =
                scoped_native_lender_spans(package, unit, initializer, binding, places, true)
        {
            lenders.extend(origins);
        }
        if !places.contains(&place) {
            places.push(place);
        }
    } else {
        collect_flow_lenders(unit, value.span, lenders);
    }
}

fn collect_flow_lenders(unit: &SemanticUnit, source: Span, lenders: &mut BTreeSet<Span>) {
    for (&(file, start, end), origins) in &unit.flow_read_bindings {
        if file == source.file && start >= source.start && end <= source.end {
            lenders.extend(origins);
        }
    }
}

fn root_name(node: &SyntaxNode) -> Option<&SyntaxNode> {
    match node.kind {
        SyntaxKind::Name => Some(node),
        SyntaxKind::MemberExpression
        | SyntaxKind::IndexExpression
        | SyntaxKind::GroupExpression => node.children.first().and_then(root_name),
        _ => None,
    }
}

fn infer_lender_related_value_type(
    unit: &SemanticUnit,
    value: &SyntaxNode,
    lenders: &[Span],
) -> Result<Option<ValueType>, SemanticFailure> {
    let expression_key = (value.span.file, value.span.start, value.span.end);
    if let Some(value_type) = unit
        .invocation_scoped_function_results
        .get(&expression_key)
        .or_else(|| unit.selected_expression_types.get(&expression_key))
    {
        return Ok(Some(value_type.clone()));
    }
    if value.kind == SyntaxKind::Name
        && let Some(identity) = binding_identity_at(unit, value)
        && let Some(binding) = unit
            .typed_bindings
            .iter()
            .find(|binding| binding.span == identity)
    {
        return Ok(Some(binding.value_type.clone()));
    }
    if value.kind == SyntaxKind::GroupExpression
        && let Some(inner) = value.children.first()
    {
        return infer_lender_related_value_type(unit, inner, lenders);
    }
    if directly_uses_lender(unit, value, lenders) || references_lender(unit, value, lenders) {
        infer_value_type(unit, value, &unit.typed_bindings)
    } else {
        Ok(None)
    }
}

fn collection_stores_lender(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    region: Span,
    call: &SyntaxNode,
    lenders: &[Span],
    acquired_type: Option<&ValueType>,
) -> Result<bool, SemanticFailure> {
    let [callee, arguments] = call.children.as_slice() else {
        return Ok(false);
    };
    let Some((receiver, receiver_type, operation)) =
        super::collections::collection_member_call(unit, callee, &unit.typed_bindings)?
    else {
        return Ok(false);
    };
    let storing = matches!(
        (receiver_type, operation.as_str()),
        (ValueType::List(_), "append" | "set")
            | (ValueType::Map(_, _) | ValueType::UnorderedMap(_, _), "set")
            | (ValueType::Set(_) | ValueType::UnorderedSet(_), "add")
    );
    if !storing {
        return Ok(false);
    }
    let receiver_scope = root_name(receiver)
        .and_then(|name| binding_identity_at(unit, name))
        .and_then(|identity| {
            unit.typed_bindings
                .iter()
                .find(|binding| binding.span == identity)
        })
        .and_then(|binding| binding.scope);
    let destination_within_region = receiver_scope.is_some_and(|scope| {
        scope.file == region.file && scope.start >= region.start && scope.end <= region.end
    });
    if destination_within_region {
        return Ok(false);
    }
    for argument in &arguments.children {
        let value = argument.children.last().unwrap_or(argument);
        let Some(value_type) = infer_lender_related_value_type(unit, value, lenders)? else {
            continue;
        };
        if scoped_region(&value_type) == Some((region.file, region.start, region.end))
            || directly_uses_lender(unit, value, lenders)
            || (references_lender(unit, value, lenders)
                && (field_has_native_loan(package, &value_type)
                    || scoped_region(&value_type).is_some()
                    || expression_has_lender_provenance(unit, value, lenders)
                    || acquired_type
                        .is_some_and(|lender| contains_nested_type(&value_type, lender))))
        {
            return Ok(true);
        }
    }
    Ok(false)
}
fn validate_lender_binding_destination(
    unit: &SemanticUnit,
    region: Span,
    node: &SyntaxNode,
    value: &SyntaxNode,
    value_type: &ValueType,
    lenders: &mut Vec<Span>,
    acquired_type: Option<&ValueType>,
) -> Result<(), SemanticFailure> {
    let region_id = (region.file, region.start, region.end);
    let carries_lender = scoped_region(value_type) == Some(region_id)
        || directly_uses_lender(unit, value, lenders)
        || (references_lender(unit, value, lenders)
            && (scoped_region(value_type).is_some()
                || expression_has_lender_provenance(unit, value, lenders)
                || acquired_type.is_some_and(|lender| contains_nested_type(value_type, lender))));
    if !carries_lender {
        return Ok(());
    }
    let destination = declared_bindings_at_node(unit, node).next().or_else(|| {
        node.children
            .first()
            .filter(|target| target.kind == SyntaxKind::Name)
            .and_then(|target| {
                let name = node_text(&unit.source, target);
                unit.typed_bindings
                    .iter()
                    .filter(|binding| {
                        binding.name == name
                            && binding.is_visible_at(unit.source.id(), target.span.start)
                    })
                    .max_by_key(|binding| binding.visible_from)
            })
    });
    let Some(destination) = destination else {
        return Err(failure(
            &unit.source,
            "T0240",
            "scoped-access guard or dependent value cannot be stored through an outer place",
            node.span,
        ));
    };
    let destination_within_lender = destination.scope.is_some_and(|scope| {
        scope.file == region.file && scope.start >= region.start && scope.end <= region.end
    });
    if !destination_within_lender {
        return Err(failure(
            &unit.source,
            "T0240",
            "scoped-access guard or dependent value cannot escape its region",
            value.span,
        ));
    }
    if !lenders.contains(&destination.span) {
        lenders.push(destination.span);
    }
    Ok(())
}

fn validate_region_escape_node(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    region: Span,
    node: &SyntaxNode,
    lenders: &mut Vec<Span>,
) -> Result<(), SemanticFailure> {
    let region_id = (region.file, region.start, region.end);
    let acquired_type = lenders.first().and_then(|span| {
        unit.typed_bindings
            .iter()
            .find(|binding| binding.span == *span)
            .map(|binding| &binding.value_type)
    });
    if node.kind == SyntaxKind::CallExpression
        && collection_stores_lender(package, unit, region, node, lenders, acquired_type)?
    {
        return Err(failure(
            &unit.source,
            "T0240",
            "a scoped-access guard or dependent value cannot be stored in an outer collection",
            node.span,
        ));
    }
    if node.kind == SyntaxKind::ReturnStatement
        && let Some(value) = node.children.first()
        && let Some(value_type) = infer_lender_related_value_type(unit, value, lenders)?
        && (scoped_region(&value_type) == Some(region_id)
            || directly_uses_lender(unit, value, lenders)
            || (references_lender(unit, value, lenders)
                && (field_has_native_loan(package, &value_type)
                    || scoped_region(&value_type).is_some()
                    || expression_has_lender_provenance(unit, value, lenders)
                    || acquired_type
                        .is_some_and(|lender| contains_nested_type(&value_type, lender)))))
    {
        return Err(failure(
            &unit.source,
            "T0240",
            "scoped-access guard or dependent value cannot be returned",
            node.span,
        ));
    }
    if node.kind == SyntaxKind::AnonymousFunction
        && let Some(contract) = unit
            .functions
            .iter()
            .find(|contract| contract.span == node.span)
        && contract.captures.iter().any(|capture| {
            unit.typed_bindings
                .iter()
                .any(|binding| binding.name == *capture && lenders.contains(&binding.span))
        })
    {
        let carrier = unit.typed_bindings.iter().find(|binding| {
            binding.span.file == node.span.file
                && binding.span.start <= node.span.start
                && node.span.end <= binding.span.end
                && binding.scope.is_some_and(|scope| {
                    scope.file == region.file
                        && region.start <= scope.start
                        && scope.end <= region.end
                })
                && matches!(
                    &binding.value_type,
                    ValueType::Function(_, _, effects) | ValueType::AsyncFunction(_, _, _, effects)
                        if effects.borrowed_environment
                )
        });
        let Some(carrier) = carrier else {
            return Err(failure(
                &unit.source,
                "T0240",
                "a scoped-access capture requires a callable owned within the same region",
                node.span,
            ));
        };
        if !lenders.contains(&carrier.span) {
            lenders.push(carrier.span);
        }
    }
    if matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
        && let Some(value) = node.children.last()
        && let Some(value_type) = infer_lender_related_value_type(unit, value, lenders)?
    {
        validate_lender_binding_destination(
            unit,
            region,
            node,
            value,
            &value_type,
            lenders,
            acquired_type,
        )?;
    }
    for child in &node.children {
        validate_region_escape_node(package, unit, region, child, lenders)?;
    }
    Ok(())
}
