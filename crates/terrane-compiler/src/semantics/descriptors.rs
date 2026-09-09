use super::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct MaterializedDescriptor<'a> {
    pub contract: &'a DescriptorContract,
    pub identity: String,
    pub name: String,
}

fn identity(namespace: &str, name: &str) -> ObjectIdentity {
    ObjectIdentity::new(namespace, name)
}

fn members(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| (*name).to_owned()).collect()
}

fn scalar_members(scalar: ScalarType) -> BTreeSet<String> {
    let mut result = members(&["type"]);
    if scalar.is_integer() {
        result.extend(
            [
                "add",
                "subtract",
                "multiply",
                "divide",
                "remainder",
                "div-rem",
                "shift-left",
                "shift-right",
                "coerce",
                "coerce.checked",
                "coerce.wrap",
                "coerce.saturate",
                "radix",
            ]
            .into_iter()
            .map(str::to_owned),
        );
        if !scalar.conforms_to(TypeCategory::UnsignedFixedInteger) {
            result.insert("negate".to_owned());
        }
        if scalar == ScalarType::Int {
            result.extend(
                ["divide.checked", "remainder.checked", "div-rem.checked"]
                    .into_iter()
                    .map(str::to_owned),
            );
        } else {
            for family in [
                "add",
                "subtract",
                "multiply",
                "divide",
                "remainder",
                "negate",
                "shift-left",
                "shift-right",
            ] {
                if family != "negate" || !scalar.conforms_to(TypeCategory::UnsignedFixedInteger) {
                    result.insert(format!("{family}.checked"));
                    result.insert(format!("{family}.wrap"));
                }
                if !matches!(family, "shift-left" | "shift-right")
                    && (family != "negate"
                        || !scalar.conforms_to(TypeCategory::UnsignedFixedInteger))
                {
                    result.insert(format!("{family}.saturate"));
                    result.insert(format!("{family}.overflowing"));
                }
            }
            result.insert("div-rem.checked".to_owned());
        }
    }
    if matches!(scalar, ScalarType::Float32 | ScalarType::Float64) {
        result.extend(
            [
                "coerce",
                "coerce.checked",
                "coerce.wrap",
                "coerce.saturate",
                "finite",
                "infinite",
                "not-a-number",
                "square-root",
                "sine",
                "cosine",
                "sine-cosine",
                "natural-log",
                "exponential",
                "absolute",
                "round",
                "floor",
                "ceiling",
                "truncate",
                "minimum",
                "maximum",
                "multiply-add",
            ]
            .into_iter()
            .map(str::to_owned),
        );
    }
    match scalar {
        ScalarType::Bool => {
            result.insert("truth".to_owned());
        }
        ScalarType::String => result.extend(
            [
                "iterator",
                "length",
                "bytes",
                "scalars",
                "graphemes",
                "trim",
                "trim.start",
                "trim.end",
                "contains",
                "contains.start",
                "contains.end",
                "find",
                "find.all",
                "find.count",
                "upper",
                "upper.first",
                "upper.words",
                "lower",
                "lower.first",
                "normalise.nfc",
                "normalise.nfd",
                "normalise.nfkc",
                "normalise.nfkd",
                "case-fold",
                "split",
                "replace",
                "encode",
                "concat",
                "join",
                "parse",
                "parse.checked",
                "radix",
            ]
            .into_iter()
            .map(str::to_owned),
        ),
        ScalarType::Bytes => result.extend(
            ["iterator", "length", "decode", "concat"]
                .into_iter()
                .map(str::to_owned),
        ),
        _ => {}
    }
    result
}
fn collection_members(kind: BuiltinDescriptor) -> BTreeSet<String> {
    let names: &[&str] = match kind {
        BuiltinDescriptor::List => &[
            "type",
            "iterator",
            "length",
            "append",
            "set",
            "get",
            "get.checked",
            "remove",
            "clear",
        ],
        BuiltinDescriptor::Map | BuiltinDescriptor::UnorderedMap => &[
            "type",
            "iterator",
            "length",
            "set",
            "get",
            "get.checked",
            "keys",
            "values",
            "entries",
        ],
        BuiltinDescriptor::Set | BuiltinDescriptor::UnorderedSet => {
            &["type", "iterator", "length", "add", "contains", "remove"]
        }
        BuiltinDescriptor::Tuple => &["type", "iterator", "length", "get", "get.checked"],
        BuiltinDescriptor::Range => &["type", "iterator"],
        BuiltinDescriptor::Entry => &["type", "key", "value"],
        BuiltinDescriptor::Iterator => &["type", "iterator", "next"],
        BuiltinDescriptor::IterationStep => &["type", "end", "value"],
        _ => &["type"],
    };
    members(names)
}

fn contract(
    file: u32,
    namespace: &str,
    name: &str,
    builtin: BuiltinDescriptor,
    categories: Vec<TypeCategory>,
    members: BTreeSet<String>,
) -> DescriptorContract {
    let methods: BTreeSet<String> = members
        .iter()
        .map(|member| {
            member
                .split_once('.')
                .map_or(member.as_str(), |(family, _)| family)
        })
        .filter(|member| {
            !matches!(
                *member,
                "type" | "length" | "bytes" | "scalars" | "graphemes" | "key" | "value"
            )
        })
        .map(str::to_owned)
        .collect();
    let invocation_only_methods = match builtin {
        BuiltinDescriptor::Scalar(scalar) if scalar.conforms_to(TypeCategory::Number) => methods
            .iter()
            .filter(|member| {
                matches!(
                    member.as_str(),
                    "add"
                        | "subtract"
                        | "multiply"
                        | "divide"
                        | "remainder"
                        | "div-rem"
                        | "negate"
                        | "shift-left"
                        | "shift-right"
                        | "coerce"
                        | "radix"
                )
            })
            .cloned()
            .collect(),
        BuiltinDescriptor::Scalar(ScalarType::String | ScalarType::Bytes)
        | BuiltinDescriptor::Iterator
        | BuiltinDescriptor::IterationStep
        | BuiltinDescriptor::List
        | BuiltinDescriptor::Map
        | BuiltinDescriptor::Set
        | BuiltinDescriptor::Tuple
        | BuiltinDescriptor::Range
        | BuiltinDescriptor::Entry
        | BuiltinDescriptor::UnorderedMap
        | BuiltinDescriptor::UnorderedSet => methods.clone(),
        _ => BTreeSet::new(),
    };
    let operation_prefix = match builtin {
        BuiltinDescriptor::Scalar(ScalarType::String) => "string",
        BuiltinDescriptor::Scalar(ScalarType::Bytes) => "bytes",
        BuiltinDescriptor::Scalar(scalar) if scalar.conforms_to(TypeCategory::Number) => "numeric",
        BuiltinDescriptor::List
        | BuiltinDescriptor::Map
        | BuiltinDescriptor::Set
        | BuiltinDescriptor::Tuple
        | BuiltinDescriptor::Range
        | BuiltinDescriptor::Entry
        | BuiltinDescriptor::UnorderedMap
        | BuiltinDescriptor::UnorderedSet
        | BuiltinDescriptor::Iterator
        | BuiltinDescriptor::IterationStep => "collection",
        _ => "value",
    };
    let operations = members
        .iter()
        .map(|member| (member.clone(), format!("{operation_prefix}.{member}")))
        .collect();
    DescriptorContract {
        name: name.to_owned(),
        identity: identity(namespace, name),
        span: Span::new(file, 0, 0),
        kind: ObjectKind::Type,
        resource_owning: false,
        builtin: Some(builtin),
        categories,
        members,
        methods,
        operations,
        invocation_only_methods,
        static_members: BTreeSet::new(),
        static_methods: BTreeSet::new(),
        base: None,
        interfaces: Vec::new(),
        traits: Vec::new(),
        fields: Vec::new(),
    }
}

pub(super) fn builtin_descriptor_contracts(file: u32) -> Vec<DescriptorContract> {
    let mut contracts = ScalarType::ALL
        .into_iter()
        .map(|scalar| {
            contract(
                file,
                "/core/types",
                scalar.source_name(),
                BuiltinDescriptor::Scalar(scalar),
                scalar.builtin_categories().to_vec(),
                scalar_members(scalar),
            )
        })
        .collect::<Vec<_>>();
    for (name, category) in TypeCategory::ABSTRACT_SOURCE_NAMES {
        contracts.push(contract(
            file,
            "/core/types",
            name,
            BuiltinDescriptor::Category(category),
            Vec::new(),
            members(&["type"]),
        ));
    }
    contracts.push(contract(
        file,
        "/core/types",
        "value",
        BuiltinDescriptor::Value,
        vec![TypeCategory::Value, TypeCategory::Object],
        members(&["type"]),
    ));
    for (name, builtin) in [
        ("encoding", BuiltinDescriptor::Encoding),
        ("overflow-result", BuiltinDescriptor::OverflowResult),
        ("div-rem-result", BuiltinDescriptor::DivRemResult),
    ] {
        contracts.push(contract(
            file,
            "/core/types",
            name,
            builtin,
            vec![TypeCategory::Value, TypeCategory::Object],
            members(&["type"]),
        ));
    }
    for (name, builtin) in [
        ("iterator", BuiltinDescriptor::Iterator),
        ("iteration-step", BuiltinDescriptor::IterationStep),
        ("list", BuiltinDescriptor::List),
        ("map", BuiltinDescriptor::Map),
        ("set", BuiltinDescriptor::Set),
        ("tuple", BuiltinDescriptor::Tuple),
        ("range", BuiltinDescriptor::Range),
        ("entry", BuiltinDescriptor::Entry),
        ("unordered-map", BuiltinDescriptor::UnorderedMap),
        ("unordered-set", BuiltinDescriptor::UnorderedSet),
    ] {
        contracts.push(contract(
            file,
            "/core/collections",
            name,
            builtin,
            vec![TypeCategory::Value, TypeCategory::Object],
            collection_members(builtin),
        ));
    }
    contracts
}

fn builtin_for_value_type(value_type: &ValueType) -> Option<BuiltinDescriptor> {
    match value_type {
        ValueType::Scalar(scalar) => Some(BuiltinDescriptor::Scalar(*scalar)),
        ValueType::Encoding => Some(BuiltinDescriptor::Encoding),
        ValueType::OverflowResult(_) => Some(BuiltinDescriptor::OverflowResult),
        ValueType::DivRemResult(_) => Some(BuiltinDescriptor::DivRemResult),
        ValueType::Iterator(_) => Some(BuiltinDescriptor::Iterator),
        ValueType::IterationStep(_) | ValueType::IterationEnd => {
            Some(BuiltinDescriptor::IterationStep)
        }
        ValueType::StringList | ValueType::TextRangeList | ValueType::List(_) => {
            Some(BuiltinDescriptor::List)
        }
        ValueType::Map(_, _) => Some(BuiltinDescriptor::Map),
        ValueType::Set(_) => Some(BuiltinDescriptor::Set),
        ValueType::Tuple(_, _) => Some(BuiltinDescriptor::Tuple),
        ValueType::Range => Some(BuiltinDescriptor::Range),
        ValueType::Entry(_, _) => Some(BuiltinDescriptor::Entry),
        ValueType::UnorderedMap(_, _) => Some(BuiltinDescriptor::UnorderedMap),
        ValueType::UnorderedSet(_) => Some(BuiltinDescriptor::UnorderedSet),
        _ => Some(BuiltinDescriptor::Value),
    }
}

pub(crate) fn descriptor_contract_for_value<'a>(
    unit: &'a SemanticUnit,
    value_type: &ValueType,
) -> Option<&'a DescriptorContract> {
    if let ValueType::Object(identity) = value_type {
        return unit
            .descriptors
            .iter()
            .find(|contract| contract.identity == *identity);
    }
    let builtin = builtin_for_value_type(value_type)?;
    unit.descriptors
        .iter()
        .find(|contract| contract.builtin == Some(builtin))
}

pub(crate) fn materialized_descriptor<'a>(
    unit: &'a SemanticUnit,
    value_type: &ValueType,
) -> Option<MaterializedDescriptor<'a>> {
    if let ValueType::Descriptor(identity) = value_type {
        let contract = descriptor_contract_by_identity(unit, identity)?;
        return Some(MaterializedDescriptor {
            contract,
            identity: identity.clone(),
            name: identity
                .rsplit_once("::")
                .map_or(identity.as_str(), |(_, name)| name)
                .to_owned(),
        });
    }
    let contract = descriptor_contract_for_value(unit, value_type)?;
    if matches!(value_type, ValueType::Object(_))
        || matches!(
            contract.builtin,
            Some(BuiltinDescriptor::Scalar(_) | BuiltinDescriptor::Encoding)
        )
    {
        return Some(MaterializedDescriptor {
            contract,
            identity: contract.identity.qualified(),
            name: contract.name.clone(),
        });
    }
    let name = value_type.to_string();
    Some(MaterializedDescriptor {
        contract,
        identity: format!("{}::{name}", contract.identity.namespace),
        name,
    })
}

pub(crate) fn descriptor_contract_by_identity<'a>(
    unit: &'a SemanticUnit,
    descriptor_identity: &str,
) -> Option<&'a DescriptorContract> {
    if let Some(contract) = unit.descriptors.iter().find(|contract| {
        contract.identity.qualified() == descriptor_identity || contract.name == descriptor_identity
    }) {
        return Some(contract);
    }
    let name = descriptor_identity
        .rsplit_once("::")
        .map_or(descriptor_identity, |(_, name)| name);
    let family = name.split_once(" of ").map_or(name, |(family, _)| family);
    unit.descriptors
        .iter()
        .find(|contract| contract.builtin.is_some() && contract.name == family)
        .or_else(|| {
            unit.descriptors
                .iter()
                .find(|contract| contract.builtin == Some(BuiltinDescriptor::Value))
        })
}

pub(crate) fn descriptor_operation<'a>(
    unit: &'a SemanticUnit,
    value_type: &ValueType,
    member: &str,
) -> Option<&'a str> {
    descriptor_contract_for_value(unit, value_type)?
        .operations
        .get(member)
        .map(String::as_str)
}

pub(crate) fn descriptor_has_member(
    unit: &SemanticUnit,
    value_type: &ValueType,
    member: &str,
) -> bool {
    descriptor_operation(unit, value_type, member).is_some()
}

pub(crate) fn descriptor_has_method(
    unit: &SemanticUnit,
    value_type: &ValueType,
    member: &str,
) -> bool {
    descriptor_contract_for_value(unit, value_type)
        .is_some_and(|contract| contract.methods.contains(member))
}

pub(crate) fn descriptor_conforms_to(
    unit: &SemanticUnit,
    value_type: &ValueType,
    category: TypeCategory,
) -> bool {
    descriptor_contract_for_value(unit, value_type)
        .is_some_and(|contract| contract.categories.contains(&category))
}

pub(crate) fn descriptor_method_requires_invocation(
    unit: &SemanticUnit,
    value_type: &ValueType,
    member: &str,
) -> bool {
    descriptor_contract_for_value(unit, value_type)
        .is_some_and(|contract| contract.invocation_only_methods.contains(member))
}

pub(crate) fn canonical_descriptor_identity(unit: &SemanticUnit, value_type: &ValueType) -> String {
    materialized_descriptor(unit, value_type)
        .map_or_else(|| value_type.to_string(), |descriptor| descriptor.identity)
}
pub(crate) fn escaped_invocation_only_member(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<Option<(ValueType, String, Span)>, SemanticFailure> {
    if node.kind == SyntaxKind::CallExpression {
        for argument in node.children.iter().skip(1) {
            if let Some(found) = escaped_invocation_only_member(unit, argument, bindings)? {
                return Ok(Some(found));
            }
        }
        return Ok(None);
    }
    if node.kind == SyntaxKind::MemberExpression
        && let [receiver, member] = node.children.as_slice()
    {
        let member_name = node_text(&unit.source, member);
        if let Some(receiver_type) = infer_receiver_value_type(unit, receiver, bindings)?
            && descriptor_method_requires_invocation(unit, &receiver_type, member_name)
        {
            return Ok(Some((receiver_type, member_name.to_owned(), node.span)));
        }
    }
    for child in &node.children {
        if let Some(found) = escaped_invocation_only_member(unit, child, bindings)? {
            return Ok(Some(found));
        }
    }
    Ok(None)
}

pub(crate) fn validate_invocation_only_member_expression(
    unit: &SemanticUnit,
    node: &SyntaxNode,
    bindings: &[TypedBinding],
) -> Result<(), SemanticFailure> {
    let Some((receiver_type, member, span)) = escaped_invocation_only_member(unit, node, bindings)?
    else {
        return Ok(());
    };
    Err(failure(
        &unit.source,
        "T0018",
        format!(
            "{receiver_type} methods are not storable values before bound methods exist; \
             method `.{member}` must be invoked with `;`"
        ),
        span,
    ))
}

pub(crate) fn validate_invocation_only_members(unit: &SemanticUnit) -> Result<(), SemanticFailure> {
    validate_invocation_only_member_expression(unit, &unit.tree.root, &unit.typed_bindings)
}
