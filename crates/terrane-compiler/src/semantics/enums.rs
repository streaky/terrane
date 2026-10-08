use super::matching::MatchCoverage;
use super::prelude::*;

#[derive(Clone)]
pub(super) struct ResolvedEnum {
    identity: ObjectIdentity,
    variants: Vec<(String, Vec<ValueType>)>,
    generic_parameters: Vec<GenericParameterContract>,
    exhaustive: bool,
    projected: bool,
    unavailable_variants: BTreeSet<String>,
}

fn match_value_type(
    unit: &SemanticUnit,
    scrutinee: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<(ValueType, bool, bool)>, SemanticFailure> {
    let operator = (scrutinee.kind == SyntaxKind::UnaryExpression)
        .then(|| unary_operator_text(unit, scrutinee))
        .flatten();
    let explicit_borrow = operator.as_deref() == Some("ref");
    let consuming = operator.as_deref() == Some("move");
    let operand = if explicit_borrow || consuming {
        scrutinee.children.last()
    } else {
        Some(scrutinee)
    };
    let Some(operand) = operand else {
        return Ok(None);
    };
    let Some(mut value_type) = infer_value_type(unit, operand, bindings)? else {
        return Ok(None);
    };
    let borrowed = explicit_borrow
        || matches!(
            &value_type,
            ValueType::Reference(_) | ValueType::SharedReference(_)
        );
    let value_type = loop {
        match value_type {
            ValueType::Reference(item) | ValueType::SharedReference(item) => {
                value_type = item.value_type();
            }
            other => break other,
        }
    };
    Ok(Some((value_type, borrowed, consuming)))
}

fn enum_identity(value_type: &ValueType) -> Option<(ObjectIdentity, bool)> {
    match value_type {
        ValueType::Object(identity) => Some((identity.clone(), false)),
        ValueType::Optional(inner) => match inner.as_ref() {
            ValueType::Object(identity) => Some((identity.clone(), true)),
            _ => None,
        },
        ValueType::Reference(inner) | ValueType::SharedReference(inner) => {
            enum_identity(&inner.value_type())
        }
        _ => None,
    }
}

fn resolve_enum_name(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    name: &SyntaxNode,
) -> Option<String> {
    fn resolve(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
    ) -> Option<String> {
        if node.kind == SyntaxKind::Name
            && let Some(symbol) =
                package.resolve_name_at(unit, node.span.start, node_text(&unit.source, node))
        {
            if matches!(symbol.kind, SymbolKind::Enum | SymbolKind::Class) {
                return Some(ObjectIdentity::new(&symbol.namespace, &symbol.name).qualified());
            }
            return symbol.descriptor_identity().map(str::to_owned);
        }
        node.children
            .iter()
            .find_map(|child| resolve(package, unit, child))
    }
    resolve(package, unit, name)
}

fn first_name(node: &SyntaxNode) -> Option<&SyntaxNode> {
    (node.kind == SyntaxKind::Name)
        .then_some(node)
        .or_else(|| node.children.iter().find_map(first_name))
}

#[expect(
    clippy::too_many_lines,
    reason = "The ordered pattern proof keeps coverage, payload authority, and branch-local binding diagnostics together"
)]
fn validate_match(
    enum_names: &BTreeMap<usize, String>,
    enums: &[ResolvedEnum],
    unit: &SemanticUnit,
    visible_bindings: &[TypedBinding],
    bindings: &mut Vec<TypedBinding>,
    provenance: &mut BTreeMap<(usize, usize), ReferenceProvenance>,
    node: &SyntaxNode,
) -> Result<(), SemanticFailure> {
    let Some(scrutinee) = node.children.first() else {
        return Ok(());
    };
    let Some((value_type, borrowed, consuming)) =
        match_value_type(unit, scrutinee, visible_bindings)?
    else {
        return Ok(());
    };
    if borrowed && consuming {
        return Err(failure(
            &unit.source,
            "T0224",
            "cannot move an enum value through a borrowed scrutinee",
            scrutinee.span,
        ));
    }
    let Some((identity, optional)) = enum_identity(&value_type) else {
        return Ok(());
    };
    let Some(contract) = enums.iter().find(|item| {
        item.identity.namespace == identity.namespace && item.identity.name == identity.name
    }) else {
        return Ok(());
    };
    if contract.generic_parameters.len() != identity.type_arguments.len() {
        return Err(failure(
            &unit.source,
            "T0210",
            format!(
                "enum `{}` requires a fully selected application with {} type argument(s)",
                contract.identity.name,
                contract.generic_parameters.len()
            ),
            scrutinee.span,
        ));
    }
    let mut substitutions = contract
        .generic_parameters
        .iter()
        .zip(&identity.type_arguments)
        .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
        .collect::<BTreeMap<_, _>>();
    substitutions.extend(identity.native_arguments.clone());

    let borrowed_owner = if borrowed {
        let operand = if unary_operator_text(unit, scrutinee).as_deref() == Some("ref") {
            scrutinee.children.last().unwrap_or(scrutinee)
        } else {
            scrutinee
        };
        first_name(operand)
            .and_then(|name| {
                visible_bindings.iter().rev().find(|binding| {
                    binding.name == node_text(&unit.source, name)
                        && binding.is_visible_at(unit.source.id(), name.span.start)
                })
            })
            .map_or(operand.span, |binding| binding.span)
    } else {
        scrutinee.span
    };
    let mut coverage = MatchCoverage::default();
    let mut saw_else = false;
    for case in node.children.iter().skip(1) {
        let Some(selector) = case.children.first() else {
            continue;
        };
        let text = node_text(&unit.source, selector);
        let payload_bindings = case
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::ParameterList);
        let body = case
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::Block);
        if selector.kind == SyntaxKind::MatchCatchAll {
            if payload_bindings.is_some() {
                return Err(failure(
                    &unit.source,
                    "T0211",
                    "`else` cannot bind a variant payload",
                    case.span,
                ));
            }
            if !coverage.insert_catchall() {
                return Err(failure(
                    &unit.source,
                    "T0212",
                    "duplicate or unreachable match arm",
                    case.span,
                ));
            }
            saw_else = true;
            continue;
        }
        if selector.kind == SyntaxKind::Name && text == "none" {
            if !optional {
                return Err(failure(
                    &unit.source,
                    "T0213",
                    "`none` can only match an optional enum",
                    selector.span,
                ));
            }
            if payload_bindings.is_some() {
                return Err(failure(
                    &unit.source,
                    "T0216",
                    "`none` cannot bind payloads",
                    case.span,
                ));
            }
            if !coverage.insert("none") {
                return Err(failure(
                    &unit.source,
                    "T0212",
                    "duplicate or unreachable match arm",
                    case.span,
                ));
            }
            continue;
        }
        let [enum_name, variant_node] = selector.children.as_slice() else {
            return Err(failure(
                &unit.source,
                "T0214",
                "expected a qualified enum variant",
                selector.span,
            ));
        };
        if enum_names.get(&enum_name.span.start).map(String::as_str)
            != Some(contract.identity.qualified().as_str())
        {
            return Err(failure(
                &unit.source,
                "T0214",
                format!(
                    "match case enum `{:?}` does not match scrutinee enum `{}`",
                    enum_names.get(&enum_name.span.start),
                    contract.identity.qualified()
                ),
                selector.span,
            ));
        }
        let variant_name = node_text(&unit.source, variant_node);
        let Some((_, payload)) = contract
            .variants
            .iter()
            .find(|(name, _)| name == variant_name)
        else {
            return Err(failure(
                &unit.source,
                "T0215",
                format!(
                    "enum `{}` has no variant `{variant_name}`",
                    contract.identity.name
                ),
                variant_node.span,
            ));
        };
        if contract.unavailable_variants.contains(variant_name) {
            return Err(failure(
                &unit.source,
                "T0219",
                format!(
                    "native enum variant `{variant_name}` has payload metadata unavailable for semantic matching"
                ),
                variant_node.span,
            ));
        }
        if !coverage.insert(variant_name.to_owned()) {
            return Err(failure(
                &unit.source,
                "T0212",
                "duplicate or unreachable match arm",
                case.span,
            ));
        }
        let names = payload_bindings.map_or(&[][..], |list| list.children.as_slice());
        if names.len() != payload.len() {
            return Err(failure(
                &unit.source,
                "T0216",
                format!(
                    "variant `{variant_name}` requires {} payload binding(s), found {}",
                    payload.len(),
                    names.len()
                ),
                case.span,
            ));
        }
        let function_scope = unit
            .enclosing_function_spans
            .get(&case.span.start)
            .copied()
            .flatten();
        for (name_node, ty) in names.iter().zip(payload) {
            let name = node_text(&unit.source, name_node);
            if name == "_" {
                continue;
            }
            let field_type = substitute_value_type(ty, &substitutions);
            // Native unbounded integers are converted to Terrane's integer
            // representation, so this payload is a value, not a borrowed view.
            let payload_borrowed = borrowed
                && !(contract.projected && field_type == ValueType::Scalar(ScalarType::Int));
            let binding_type = if payload_borrowed {
                ValueType::Reference(ElementType::new(field_type))
            } else {
                field_type
            };
            bindings.push(TypedBinding {
                name: name.to_owned(),
                span: name_node.span,
                visible_from: body.map_or(case.span.end, |body| body.span.start),
                scope: function_scope,
                value_type: binding_type,
                destination_arms: Vec::new(),
                storage_type: None,
                mutable: false,
            });
            if payload_borrowed {
                provenance.insert(
                    (name_node.span.start, name_node.span.end),
                    ReferenceProvenance {
                        owner: borrowed_owner,
                        external_lender: false,
                        lender_parameter: None,
                        path: Vec::new(),
                        lifetime_end: function_scope,
                    },
                );
            }
        }
    }
    if !contract.exhaustive && !saw_else {
        return Err(failure(
            &unit.source,
            "T0217",
            "non-exhaustive native enum match requires an `else` arm",
            node.span,
        ));
    }
    let missing = coverage.missing(
        contract
            .variants
            .iter()
            .map(|(name, _)| name.as_str())
            .chain(optional.then_some("none")),
    );
    if !missing.is_empty() {
        return Err(failure(
            &unit.source,
            "T0217",
            format!(
                "non-exhaustive match; missing {}",
                missing
                    .iter()
                    .map(|name| format!("`{name}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            node.span,
        ));
    }
    Ok(())
}

pub(crate) fn projected_enum_payload_type(
    package: &SemanticPackage,
    projected: &crate::rust_interop::projection::ProjectedType,
) -> Option<ValueType> {
    use crate::rust_interop::projection::ProjectedType;
    match projected {
        ProjectedType::Generic(name) => Some(ValueType::TypeParameter(name.clone())),
        ProjectedType::Optional(inner) => Some(ValueType::Optional(Box::new(
            projected_enum_payload_type(package, inner)?,
        ))),
        ProjectedType::Sequence { item, .. } => Some(ValueType::List(ElementType::new(
            projected_enum_payload_type(package, item)?,
        ))),
        ProjectedType::Set {
            item,
            ordered: true,
            ..
        } => Some(ValueType::Set(ElementType::new(
            projected_enum_payload_type(package, item)?,
        ))),
        ProjectedType::Set {
            item,
            ordered: false,
            ..
        } => Some(ValueType::UnorderedSet(ElementType::new(
            projected_enum_payload_type(package, item)?,
        ))),
        ProjectedType::Mapping {
            key,
            value,
            ordered: true,
            ..
        } => Some(ValueType::Map(
            ElementType::new(projected_enum_payload_type(package, key)?),
            ElementType::new(projected_enum_payload_type(package, value)?),
        )),
        ProjectedType::Mapping {
            key,
            value,
            ordered: false,
            ..
        } => Some(ValueType::UnorderedMap(
            ElementType::new(projected_enum_payload_type(package, key)?),
            ElementType::new(projected_enum_payload_type(package, value)?),
        )),
        _ => super::objects::closed_projected_value_type(package, projected),
    }
}

pub(super) fn resolve_enums(package: &SemanticPackage) -> Vec<ResolvedEnum> {
    let mut enums = package
        .units
        .iter()
        .flat_map(|unit| {
            unit.source_enums.iter().map(|source_enum| {
                let generic_parameters = unit
                    .descriptors
                    .iter()
                    .find(|descriptor| descriptor.identity == source_enum.identity)
                    .map_or_else(Vec::new, |descriptor| descriptor.generic_parameters.clone());
                ResolvedEnum {
                    identity: source_enum.identity.clone(),
                    variants: source_enum
                        .variants
                        .iter()
                        .map(|variant| {
                            (
                                variant.name.clone(),
                                variant
                                    .payload
                                    .iter()
                                    .map(|field| field.value_type.clone())
                                    .collect(),
                            )
                        })
                        .collect(),
                    generic_parameters,
                    exhaustive: true,
                    projected: false,
                    unavailable_variants: BTreeSet::new(),
                }
            })
        })
        .collect::<Vec<_>>();
    for dependency in &package.projection.dependencies {
        for item in &dependency.items {
            let crate::rust_interop::projection::ProjectedKind::Enum {
                variants,
                exhaustive,
                ..
            } = &item.kind
            else {
                continue;
            };
            let mut resolved = Vec::with_capacity(variants.len());
            let mut unavailable_variants = BTreeSet::new();
            for variant in variants {
                let fields = variant
                    .fields
                    .iter()
                    .map(|field| projected_enum_payload_type(package, &field.ty))
                    .collect::<Option<Vec<_>>>();
                if variant.unavailable_reason.is_some() || fields.is_none() {
                    unavailable_variants.insert(variant.name.clone());
                }
                resolved.push((variant.name.clone(), fields.unwrap_or_default()));
            }
            let generic_parameters = package
                .units
                .iter()
                .flat_map(|unit| &unit.descriptors)
                .find(|descriptor| {
                    descriptor.identity.namespace == item.namespace
                        && descriptor.identity.name == item.name
                })
                .map_or_else(Vec::new, |descriptor| descriptor.generic_parameters.clone());
            enums.push(ResolvedEnum {
                identity: ObjectIdentity::new(&item.namespace, &item.name),
                variants: resolved,
                generic_parameters,
                exhaustive: *exhaustive,
                projected: true,
                unavailable_variants,
            });
        }
    }
    enums
}

pub(super) struct MatchContext<'a> {
    enums: &'a [ResolvedEnum],
    names: BTreeMap<usize, String>,
}

impl<'a> MatchContext<'a> {
    pub(super) fn new(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        enums: &'a [ResolvedEnum],
    ) -> Self {
        let names = collect_matches(&unit.tree.root)
            .into_iter()
            .flat_map(|node| node.children.iter().skip(1))
            .filter_map(|case| case.children.first())
            .filter(|selector| selector.kind == SyntaxKind::StaticMemberExpression)
            .filter_map(|selector| selector.children.first())
            .filter_map(|name| {
                resolve_enum_name(package, unit, name).map(|identity| (name.span.start, identity))
            })
            .collect();
        Self { enums, names }
    }

    pub(super) fn bindings(
        &self,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        visible: &[TypedBinding],
    ) -> Result<Vec<TypedBinding>, SemanticFailure> {
        let mut bindings = Vec::new();
        validate_match(
            &self.names,
            self.enums,
            unit,
            visible,
            &mut bindings,
            &mut BTreeMap::new(),
            node,
        )?;
        Ok(bindings)
    }
}

pub(crate) fn validate_enum_matches(package: &mut SemanticPackage) -> Result<(), SemanticFailure> {
    let enums = resolve_enums(package);
    for index in 0..package.units.len() {
        let unit = &package.units[index];
        let context = MatchContext::new(package, unit, &enums);
        let mut bindings = Vec::new();
        let mut provenance = BTreeMap::new();
        for node in collect_matches(&unit.tree.root) {
            validate_match(
                &context.names,
                context.enums,
                unit,
                &unit.typed_bindings,
                &mut bindings,
                &mut provenance,
                node,
            )?;
        }
        let unit = &mut package.units[index];
        unit.rust_storage_names.take();
        for binding in bindings {
            if let Some(existing) = unit
                .typed_bindings
                .iter_mut()
                .find(|existing| existing.span == binding.span)
            {
                *existing = binding;
            } else {
                unit.typed_bindings.push(binding);
            }
        }
        unit.reference_provenance.extend(provenance);
    }
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "Constructor selection preserves source/native payload ordering and diagnostic precedence in one traversal"
)]
pub(crate) fn validate_enum_constructions(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    let source_enums = package
        .units
        .iter()
        .flat_map(|unit| unit.source_enums.clone())
        .collect::<Vec<_>>();
    let native_enums = package
        .projection
        .dependencies
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| {
            let crate::rust_interop::projection::ProjectedKind::Enum { variants, .. } = &item.kind
            else {
                return None;
            };
            let variants = variants
                .iter()
                .map(|variant| {
                    let fields = variant
                        .fields
                        .iter()
                        .map(|field| projected_enum_payload_type(package, &field.ty))
                        .collect::<Option<Vec<_>>>();
                    (variant.clone(), fields)
                })
                .collect::<Vec<_>>();
            Some((item.namespace.clone(), item.name.clone(), variants))
        })
        .collect::<Vec<_>>();
    for index in 0..package.units.len() {
        let constructions = {
            fn visit<'a>(node: &'a SyntaxNode, found: &mut Vec<&'a SyntaxNode>) {
                if node.kind == SyntaxKind::ConstructionExpression {
                    found.push(node);
                }
                for child in &node.children {
                    visit(child, found);
                }
            }
            let mut found = Vec::new();
            visit(&package.units[index].tree.root, &mut found);
            found.into_iter().cloned().collect::<Vec<_>>()
        };
        let (names, expected_types) = {
            let unit = &package.units[index];
            let names = constructions
                .iter()
                .filter_map(|construction| construction.children.first())
                .filter(|designator| designator.kind == SyntaxKind::StaticMemberExpression)
                .filter_map(|designator| designator.children.first())
                .filter_map(|name| {
                    resolve_enum_name(package, unit, name)
                        .map(|identity| (name.span.start, identity))
                })
                .collect::<BTreeMap<_, _>>();
            let expected = constructions
                .iter()
                .filter_map(|construction| {
                    constructor_expected_type(package, unit, &unit.tree.root, construction.span)
                        .map(|value_type| (construction.span.start, value_type))
                })
                .collect::<BTreeMap<_, _>>();
            (names, expected)
        };
        let unit = &package.units[index];
        let mut selections = Vec::new();
        for construction in &constructions {
            let Some(designator) = construction
                .children
                .first()
                .filter(|node| node.kind == SyntaxKind::StaticMemberExpression)
            else {
                continue;
            };
            let [enum_name, variant_node] = designator.children.as_slice() else {
                continue;
            };
            let Some(identity) = names.get(&enum_name.span.start) else {
                continue;
            };
            let variant = node_text(&unit.source, variant_node);
            if let Some(source_enum) = source_enums.iter().find(|item| {
                item.identity.qualified() == *identity
                    && package
                        .projection
                        .item(&item.identity.namespace, &item.identity.name)
                        .is_none()
            }) {
                let Some(contract) = source_enum
                    .variants
                    .iter()
                    .find(|item| item.name == variant)
                else {
                    return Err(failure(
                        &unit.source,
                        "T0215",
                        format!("enum variant `{variant}` does not exist"),
                        variant_node.span,
                    ));
                };
                let call = find_call_containing(&unit.tree.root, construction.span);
                let arguments = call
                    .and_then(|call| call.children.get(1))
                    .map_or(&[][..], |list| list.children.as_slice());
                if arguments.len() != contract.payload.len() {
                    return Err(failure(
                        &unit.source,
                        "T0216",
                        format!(
                            "variant `{variant}` requires {} named payload value(s), found {}",
                            contract.payload.len(),
                            arguments.len()
                        ),
                        construction.span,
                    ));
                }
                let mut by_name = BTreeMap::new();
                for argument in arguments {
                    let Some(name) = argument
                        .children
                        .first()
                        .filter(|_| argument.children.len() > 1)
                    else {
                        return Err(failure(
                            &unit.source,
                            "T0216",
                            "enum payload values must be named",
                            argument.span,
                        ));
                    };
                    let name = node_text(&unit.source, name);
                    if !contract.payload.iter().any(|field| field.name == name) {
                        return Err(failure(
                            &unit.source,
                            "T0216",
                            format!("variant `{variant}` has no payload field `{name}`"),
                            argument.span,
                        ));
                    }
                    if by_name.insert(name.to_owned(), argument).is_some() {
                        return Err(failure(
                            &unit.source,
                            "T0216",
                            format!(
                                "variant `{variant}` payload `{name}` is provided more than once"
                            ),
                            argument.span,
                        ));
                    }
                }
                let ordered_arguments = contract
                    .payload
                    .iter()
                    .map(|field| {
                        by_name.get(&field.name).copied().ok_or_else(|| {
                            failure(
                                &unit.source,
                                "T0216",
                                format!("variant `{variant}` is missing payload `{}`", field.name),
                                construction.span,
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let generic_parameters = unit
                    .descriptors
                    .iter()
                    .find(|item| item.identity == source_enum.identity)
                    .map_or(&[][..], |item| item.generic_parameters.as_slice());
                let mut substitutions = BTreeMap::new();
                if let Some(ValueType::Object(applied)) =
                    expected_types.get(&construction.span.start).cloned()
                    && applied.namespace == source_enum.identity.namespace
                    && applied.name == source_enum.identity.name
                {
                    for (parameter, argument) in
                        generic_parameters.iter().zip(&applied.type_arguments)
                    {
                        substitutions.insert(parameter.name.clone(), argument.clone());
                    }
                }
                if let ValueType::Object(applied) =
                    super::types::declared_value_type(unit, enum_name, &BTreeMap::new())?
                    && applied.namespace == source_enum.identity.namespace
                    && applied.name == source_enum.identity.name
                {
                    for (parameter, argument) in
                        generic_parameters.iter().zip(&applied.type_arguments)
                    {
                        substitutions.insert(parameter.name.clone(), argument.clone());
                    }
                }
                for (field, argument) in contract.payload.iter().zip(&ordered_arguments) {
                    let value = argument.children.last().expect("argument AST has a value");
                    let actual_type = infer_value_type(unit, value, &unit.typed_bindings)?
                        .ok_or_else(|| {
                            failure(
                                &unit.source,
                                "T0218",
                                "cannot determine enum payload argument type",
                                value.span,
                            )
                        })?;
                    super::generics::bind_generic_type(
                        &field.value_type,
                        &actual_type,
                        &mut substitutions,
                    )
                    .map_err(|message| failure(&unit.source, "T0218", message, value.span))?;
                }
                if let Some(parameter) = generic_parameters
                    .iter()
                    .find(|parameter| !substitutions.contains_key(&parameter.name))
                {
                    return Err(failure(
                        &unit.source,
                        "T0210",
                        format!(
                            "enum type parameter `{}` is not selected by this variant; write an applied enum type or provide constraining payloads",
                            parameter.name
                        ),
                        construction.span,
                    ));
                }
                for (field, argument) in contract.payload.iter().zip(&ordered_arguments) {
                    let value = argument.children.last().expect("argument AST has a value");
                    let actual_type = infer_value_type(unit, value, &unit.typed_bindings)?
                        .ok_or_else(|| {
                            failure(
                                &unit.source,
                                "T0218",
                                "cannot determine enum payload argument type",
                                value.span,
                            )
                        })?;
                    let expected = substitute_value_type(&field.value_type, &substitutions);
                    if !super::types::value_types_compatible(
                        &unit.descriptors,
                        &expected,
                        &actual_type,
                    ) {
                        return Err(failure(
                            &unit.source,
                            "T0218",
                            format!(
                                "enum payload `{}` expects `{expected}`, found `{actual_type}`",
                                field.name
                            ),
                            value.span,
                        ));
                    }
                }
                if let Some(call) = find_call_containing(&unit.tree.root, construction.span) {
                    let arguments = generic_parameters
                        .iter()
                        .map(|parameter| substitutions[&parameter.name].clone())
                        .collect();
                    let identity = source_enum.identity.clone().with_type_arguments(arguments);
                    selections.push((
                        (call.span.file, call.span.start, call.span.end),
                        ValueType::Object(identity),
                    ));
                }
                continue;
            }
            let Some((namespace, name)) = identity.rsplit_once("::") else {
                continue;
            };
            let Some((_, _, variants)) =
                native_enums.iter().find(|(item_namespace, item_name, _)| {
                    item_namespace == namespace && item_name == name
                })
            else {
                continue;
            };
            let Some((contract, field_types)) =
                variants.iter().find(|(item, _)| item.name == variant)
            else {
                return Err(failure(
                    &unit.source,
                    "T0215",
                    format!("enum variant `{variant}` does not exist"),
                    variant_node.span,
                ));
            };
            if !contract.constructible {
                return Err(failure(
                    &unit.source,
                    "T0219",
                    contract
                        .unavailable_reason
                        .as_deref()
                        .unwrap_or("native enum variant cannot be constructed from Terrane"),
                    variant_node.span,
                ));
            }
            let arguments = find_call_containing(&unit.tree.root, construction.span)
                .and_then(|call| call.children.get(1))
                .map_or(&[][..], |list| list.children.as_slice());
            let payload_carrier = package
                .projection
                .item(namespace, name)
                .and_then(|item| native_enum_payload_carrier(package, item, contract));
            let expected_arguments = if payload_carrier.is_some() {
                1
            } else {
                contract.fields.len()
            };
            if arguments.len() != expected_arguments {
                return Err(failure(
                    &unit.source,
                    "T0216",
                    format!(
                        "variant `{variant}` requires {expected_arguments} payload value(s), found {}",
                        arguments.len()
                    ),
                    construction.span,
                ));
            }
            if let Some(carrier) = &payload_carrier {
                let value = arguments[0]
                    .children
                    .last()
                    .expect("argument AST has a value");
                let actual =
                    infer_value_type(unit, value, &unit.typed_bindings)?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0218",
                            "cannot determine enum payload carrier type",
                            value.span,
                        )
                    })?;
                if !super::types::value_types_compatible(&unit.descriptors, carrier, &actual) {
                    return Err(failure(
                        &unit.source,
                        "T0218",
                        format!("native enum payload expects `{carrier}`, found `{actual}`"),
                        value.span,
                    ));
                }
            }
            let ordered_arguments = if payload_carrier.is_some() {
                Vec::new()
            } else if contract.style
                == crate::rust_interop::projection::ProjectedEnumVariantStyle::Struct
            {
                let mut by_name = BTreeMap::new();
                for argument in arguments {
                    let Some(name) = argument
                        .children
                        .first()
                        .filter(|_| argument.children.len() > 1)
                    else {
                        return Err(failure(
                            &unit.source,
                            "T0216",
                            "native struct-variant payload values must be named",
                            argument.span,
                        ));
                    };
                    let name = node_text(&unit.source, name);
                    if !contract.fields.iter().any(|field| field.name == name)
                        || by_name.insert(name.to_owned(), argument).is_some()
                    {
                        return Err(failure(
                            &unit.source,
                            "T0216",
                            format!("invalid or duplicate native payload field `{name}`"),
                            argument.span,
                        ));
                    }
                }
                contract
                    .fields
                    .iter()
                    .map(|field| {
                        by_name.get(&field.name).copied().ok_or_else(|| {
                            failure(
                                &unit.source,
                                "T0216",
                                format!("variant `{variant}` is missing payload `{}`", field.name),
                                construction.span,
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                if arguments.iter().any(|argument| argument.children.len() > 1) {
                    return Err(failure(
                        &unit.source,
                        "T0216",
                        "native tuple-variant payload values are positional",
                        construction.span,
                    ));
                }
                arguments.iter().collect()
            };
            let Some(field_types) = field_types else {
                return Err(failure(
                    &unit.source,
                    "T0219",
                    "native enum payload type is not available for semantic construction checking",
                    construction.span,
                ));
            };
            let declared = super::types::declared_value_type(unit, enum_name, &BTreeMap::new())?;
            let mut substitutions = BTreeMap::new();
            let mut native_arguments = BTreeMap::new();
            for applied in expected_types
                .get(&construction.span.start)
                .cloned()
                .into_iter()
                .chain(Some(declared))
                .filter_map(|value_type| match value_type {
                    ValueType::Object(identity)
                        if identity.namespace == namespace && identity.name == name =>
                    {
                        Some(identity)
                    }
                    _ => None,
                })
            {
                if let Some(descriptor) = unit.descriptors.iter().find(|descriptor| {
                    descriptor.identity.namespace == namespace && descriptor.identity.name == name
                }) {
                    for (parameter, argument) in descriptor
                        .generic_parameters
                        .iter()
                        .zip(&applied.type_arguments)
                    {
                        substitutions.insert(parameter.name.clone(), argument.clone());
                    }
                }
                native_arguments.extend(applied.native_arguments.clone());
            }
            for (field, argument) in field_types.iter().zip(&ordered_arguments) {
                let value = argument.children.last().expect("argument AST has a value");
                let actual_type =
                    infer_value_type(unit, value, &unit.typed_bindings)?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0218",
                            "cannot determine enum payload argument type",
                            value.span,
                        )
                    })?;
                super::generics::bind_generic_type(field, &actual_type, &mut substitutions)
                    .map_err(|message| failure(&unit.source, "T0218", message, value.span))?;
            }
            for (field, argument) in field_types.iter().zip(&ordered_arguments) {
                let value = argument.children.last().expect("argument AST has a value");
                let actual_type =
                    infer_value_type(unit, value, &unit.typed_bindings)?.ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0218",
                            "cannot determine enum payload argument type",
                            value.span,
                        )
                    })?;
                let expected = substitute_value_type(field, &substitutions);
                if matches!(expected, ValueType::TypeParameter(_)) {
                    return Err(failure(
                        &unit.source,
                        "T0210",
                        format!(
                            "native enum type parameter in variant `{variant}` is not selected"
                        ),
                        construction.span,
                    ));
                }
                if !super::types::value_types_compatible(&unit.descriptors, &expected, &actual_type)
                {
                    return Err(failure(
                        &unit.source,
                        "T0218",
                        format!("native enum payload expects `{expected}`, found `{actual_type}`"),
                        value.span,
                    ));
                }
            }
            if let Some(call) = find_call_containing(&unit.tree.root, construction.span) {
                let descriptor = unit.descriptors.iter().find(|descriptor| {
                    descriptor.identity.namespace == namespace && descriptor.identity.name == name
                });
                let arguments = descriptor
                    .into_iter()
                    .flat_map(|descriptor| &descriptor.generic_parameters)
                    .map(|parameter| substitutions.get(&parameter.name).cloned())
                    .collect::<Option<Vec<_>>>()
                    .ok_or_else(|| {
                        failure(
                            &unit.source,
                            "T0210",
                            "native enum type parameters are not fully selected",
                            construction.span,
                        )
                    })?;
                for (parameter, argument) in descriptor
                    .into_iter()
                    .flat_map(|descriptor| &descriptor.generic_parameters)
                    .zip(&arguments)
                {
                    native_arguments
                        .entry(parameter.name.clone())
                        .or_insert_with(|| argument.clone());
                }
                let item = package
                    .projection
                    .item(namespace, name)
                    .expect("validated native enum owner");
                let carriers = descriptor
                    .into_iter()
                    .flat_map(|descriptor| &descriptor.generic_parameters)
                    .map(|parameter| {
                        super::objects::destination_projected_type(
                            package,
                            &native_arguments[&parameter.name],
                        )
                        .map(|projected| projected.rust_type())
                        .map_err(|message| {
                            failure(
                                &unit.source,
                                "T0218",
                                message.into_owned(),
                                construction.span,
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let base_path = package
                    .projection
                    .canonical_native_type(
                        item.rust_path
                            .split_once('<')
                            .map_or(item.rust_path.as_str(), |(base, _)| base),
                    )
                    .into_owned();
                let native_path = if carriers.is_empty() {
                    base_path
                } else {
                    format!("{base_path}<{}>", carriers.join(", "))
                };
                let mut identity = ObjectIdentity::new(namespace, name)
                    .with_type_arguments(arguments)
                    .with_native_parameters(
                        descriptor
                            .into_iter()
                            .flat_map(|descriptor| &descriptor.generic_parameters)
                            .map(|parameter| parameter.name.clone())
                            .collect(),
                    )
                    .with_native_arguments(native_arguments)
                    .with_native_projection(native_path);
                if super::native_constructors::source_abi_nominal(package, &identity) {
                    identity =
                        super::native_constructors::canonical_source_nominal(package, &identity);
                }
                selections.push((
                    (call.span.file, call.span.start, call.span.end),
                    ValueType::Object(identity),
                ));
            }
        }
        package.units[index]
            .selected_expression_types
            .extend(selections);
    }
    Ok(())
}

fn native_enum_payload_carrier(
    package: &SemanticPackage,
    owner: &crate::rust_interop::projection::ProjectedItem,
    variant: &crate::rust_interop::projection::ProjectedEnumVariant,
) -> Option<ValueType> {
    if matches!(
        &owner.kind,
        crate::rust_interop::projection::ProjectedKind::Enum { generic_parameters, .. }
            if !generic_parameters.is_empty()
    ) {
        // Authored generic enum contracts expose their payload fields directly.
        return None;
    }
    package
        .projection
        .dependencies
        .iter()
        .flat_map(|dependency| &dependency.items)
        .find_map(|item| {
            let crate::rust_interop::projection::ProjectedKind::ForeignType {
                enum_payload: Some(payload),
                ..
            } = &item.kind
            else {
                return None;
            };
            (payload.owner_rust_path == owner.rust_path && payload.variant == variant.name)
                .then(|| ValueType::Object(ObjectIdentity::new(&item.namespace, &item.name)))
        })
}

#[expect(
    clippy::too_many_lines,
    reason = "Complete source and native constructor callable metadata is assembled at the same selection boundary"
)]
pub(super) fn selected_enum_constructor_contract(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    call: &SyntaxNode,
    _is_unsafe: bool,
) -> Option<FunctionContract> {
    let [callee, _] = call.children.as_slice() else {
        return None;
    };
    let callee = if callee.kind == SyntaxKind::ConstructionExpression {
        callee.children.first()?
    } else {
        callee
    };
    if callee.kind != SyntaxKind::StaticMemberExpression {
        return None;
    }
    let [enum_name, variant_node] = callee.children.as_slice() else {
        return None;
    };
    let enum_identity = resolve_enum_name(package, unit, enum_name)?;
    let variant_name = node_text(&unit.source, variant_node);
    let result = match unit.selected_expression_types.get(&(
        call.span.file,
        call.span.start,
        call.span.end,
    ))? {
        ValueType::Object(identity) => identity.clone(),
        _ => return None,
    };
    if let Some(source_enum) = unit
        .source_enums
        .iter()
        .find(|item| item.identity.qualified() == enum_identity)
    {
        let variant = source_enum
            .variants
            .iter()
            .find(|item| item.name == variant_name)?;
        let substitutions = unit
            .descriptors
            .iter()
            .find(|item| item.identity == source_enum.identity)
            .map_or(&[][..], |item| item.generic_parameters.as_slice())
            .iter()
            .zip(&result.type_arguments)
            .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
            .collect::<BTreeMap<_, _>>();
        let parameters = variant
            .payload
            .iter()
            .map(|field| ParameterContract {
                name: field.name.clone(),
                span: field.span,
                value_type: Some(substitute_value_type(&field.value_type, &substitutions)),
                optional: false,
                mutable: false,
                variadic: false,
            })
            .collect();
        return Some(enum_constructor_contract(
            &variant.name,
            variant.span,
            source_enum.identity.name.clone(),
            result,
            parameters,
        ));
    }
    let (namespace, name) = enum_identity.rsplit_once("::")?;
    let item = package.projection.item(namespace, name)?;
    let crate::rust_interop::projection::ProjectedKind::Enum { variants, .. } = &item.kind else {
        return None;
    };
    let variant = variants
        .iter()
        .find(|variant| variant.name == variant_name)?;
    let descriptor = unit.descriptors.iter().find(|descriptor| {
        descriptor.identity.namespace == namespace && descriptor.identity.name == name
    });
    let substitutions = descriptor
        .into_iter()
        .flat_map(|descriptor| &descriptor.generic_parameters)
        .zip(&result.type_arguments)
        .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
        .collect::<BTreeMap<_, _>>();
    let parameters = if let Some(ValueType::Object(carrier)) =
        native_enum_payload_carrier(package, item, variant)
    {
        vec![ParameterContract {
            name: "payload".to_owned(),
            span: call.span,
            value_type: Some(ValueType::Object(carrier)),
            optional: false,
            mutable: false,
            variadic: false,
        }]
    } else {
        variant
            .fields
            .iter()
            .map(|field| {
                Some(ParameterContract {
                    name: field.name.clone(),
                    span: call.span,
                    value_type: Some(substitute_value_type(
                        &projected_enum_payload_type(package, &field.ty)?,
                        &substitutions,
                    )),
                    optional: false,
                    mutable: false,
                    variadic: false,
                })
            })
            .collect::<Option<Vec<_>>>()?
    };
    Some(enum_constructor_contract(
        &variant.name,
        call.span,
        name.to_owned(),
        result,
        parameters,
    ))
}

fn enum_constructor_contract(
    name: &str,
    span: Span,
    owner: String,
    result: ObjectIdentity,
    parameters: Vec<ParameterContract>,
) -> FunctionContract {
    FunctionContract {
        name: name.to_owned(),
        span,
        owner: Some(owner),
        owner_identity: Some(result.clone()),
        is_anonymous: false,
        projected_provided: false,
        captures: Vec::new(),
        generic_parameters: Vec::new(),
        parameters,
        return_type: Some(ValueType::Object(result)),
        exported: false,
        thrown_types: Vec::new(),
        escaping_throwables: BTreeSet::new(),
        throws: false,
        is_async: false,
        is_unsafe: false,
        task_transferability: TaskTransferability::Transferable,
        execution_requirements: crate::execution::ExecutionRequirements::default(),
        is_static: true,
        written_invocation_mode: InvocationMode::Shared,
        exact_invocation_mode: InvocationMode::Shared,
    }
}

fn find_call_containing(node: &SyntaxNode, span: Span) -> Option<&SyntaxNode> {
    if node.kind == SyntaxKind::CallExpression
        && node
            .children
            .first()
            .is_some_and(|callee| callee.span == span)
    {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_call_containing(child, span))
}
fn constructor_expected_type(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    root: &SyntaxNode,
    span: Span,
) -> Option<ValueType> {
    if root.kind == SyntaxKind::Binding
        && root.children.iter().any(|initializer| {
            initializer.span.start <= span.start
                && initializer.span.end >= span.end
                && (initializer.kind == SyntaxKind::ConstructionExpression
                    || initializer.kind == SyntaxKind::CallExpression
                        && initializer
                            .children
                            .first()
                            .is_some_and(|callee| callee.span == span))
        })
        && let Some(type_node) = root
            .children
            .iter()
            .find(|child| child.kind == SyntaxKind::TypeExpression)
        && let Ok(value_type) = super::types::declared_value_type(unit, type_node, &BTreeMap::new())
    {
        return Some(value_type);
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
        && let Some(contract) = super::calls::selected_callable_contract(package, unit, root, false)
            .or_else(|| super::calls::selected_callable_contract(package, unit, root, true))
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
        .find_map(|child| constructor_expected_type(package, unit, child, span))
}

fn collect_matches(node: &SyntaxNode) -> Vec<&SyntaxNode> {
    fn visit<'a>(node: &'a SyntaxNode, matches: &mut Vec<&'a SyntaxNode>) {
        if node.kind == SyntaxKind::MatchStatement {
            matches.push(node);
        }
        for child in &node.children {
            visit(child, matches);
        }
    }
    let mut matches = Vec::new();
    visit(node, &mut matches);
    matches
}
