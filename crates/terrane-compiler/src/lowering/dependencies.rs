use super::prelude::*;
use std::collections::BTreeMap;

pub(super) type DependencyImportOwners = BTreeMap<String, (String, String, Vec<String>, bool)>;

fn visit_value_type_objects(value_type: &ValueType, visit: &mut impl FnMut(&ObjectIdentity)) {
    match value_type {
        ValueType::Object(identity) => visit(identity),
        ValueType::Optional(inner) => visit_value_type_objects(inner, visit),
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
            visit_value_type_objects(item.value_type_ref(), visit);
        }
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            visit_value_type_objects(key.value_type_ref(), visit);
            visit_value_type_objects(value.value_type_ref(), visit);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                visit_value_type_objects(parameter.value_type_ref(), visit);
            }
            visit_value_type_objects(result.value_type_ref(), visit);
        }
        _ => {}
    }
}

fn value_type_mentions_object(value_type: &ValueType, target: &ObjectIdentity) -> bool {
    let mut mentions = false;
    visit_value_type_objects(value_type, &mut |identity| {
        mentions |= identity == target;
    });
    mentions
}

fn collect_value_type_objects(value_type: &ValueType, objects: &mut BTreeSet<ObjectIdentity>) {
    visit_value_type_objects(value_type, &mut |identity| {
        objects.insert(identity.clone());
    });
}

fn unit_references_object(
    unit: &SemanticUnit,
    identity: &ObjectIdentity,
    static_method_references: &StaticMethodReferences,
) -> bool {
    unit.functions.iter().any(|function| {
        if function.is_static {
            let Some(owner) = &function.owner_identity else {
                return false;
            };
            if !static_method_references.contains(&(
                owner.namespace.clone(),
                owner.name.clone(),
                function.name.clone(),
            )) {
                return false;
            }
            if owner == identity {
                return true;
            }
        } else if function.owner_identity.as_ref() == Some(identity) {
            return true;
        }
        function
            .return_type
            .as_ref()
            .is_some_and(|return_type| value_type_mentions_object(return_type, identity))
            || function.parameters.iter().any(|parameter| {
                parameter
                    .value_type
                    .as_ref()
                    .is_some_and(|value_type| value_type_mentions_object(value_type, identity))
            })
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "cross-unit import ownership is one deterministic indexing pass"
)]
pub(super) fn index_dependency_import_owners(
    package: &SemanticPackage,
    static_method_references: &StaticMethodReferences,
) -> DependencyImportOwners {
    let mut owners = BTreeMap::new();
    for unit in &package.units {
        if !unit.bundled || !unit.namespace.starts_with("/deps/") {
            continue;
        }
        for object in &unit.descriptors {
            if object.kind == ObjectKind::Interface {
                continue;
            }
            if object.identity.namespace != unit.namespace
                || !unit_references_object(unit, &object.identity, static_method_references)
                || package
                    .projection
                    .item(&object.identity.namespace, &object.identity.name)
                    .is_some_and(|item| {
                        matches!(item.kind, crate::projection::ProjectedKind::Interface(_))
                    })
            {
                continue;
            }
            let projected = package
                .projection
                .projected_type(&object.identity.namespace, &object.identity.name);
            let path = package
                .projection
                .foreign_rust_path(&object.identity.namespace, &object.identity.name)
                .map(str::to_owned)
                .or_else(|| {
                    projected
                        .as_ref()
                        .map(crate::projection::ProjectedType::rust_type)
                });
            let Some(path) = path else {
                continue;
            };
            let rust_name = rust_object_type_name(package, &object.identity);
            let generic_parameters = projected
                .as_ref()
                .map(projected_generic_names)
                .unwrap_or_default();
            owners
                .entry(rust_name)
                .and_modify(
                    |(owner, selected_path, selected_parameters, canonical): &mut (
                        String,
                        String,
                        Vec<String>,
                        bool,
                    )| {
                        if unit.namespace < *owner {
                            owner.clone_from(&unit.namespace);
                        }
                        if !*canonical {
                            selected_path.clone_from(&path);
                            selected_parameters.clone_from(&generic_parameters);
                            *canonical = true;
                        }
                    },
                )
                .or_insert_with(|| (unit.namespace.clone(), path, generic_parameters, true));
        }
        for (name, path) in package.projection.foreign_imports(&unit.namespace) {
            let rust_name = rust_object_name(&name);
            if !unit.descriptors.iter().any(|object| {
                object.kind != ObjectKind::Interface
                    && rust_object_type_name(package, &object.identity) == rust_name
                    && unit_references_object(unit, &object.identity, static_method_references)
            }) {
                continue;
            }
            let projected = package.projection.projected_type(&unit.namespace, &name);
            let generic_parameters = projected
                .as_ref()
                .map(projected_generic_names)
                .unwrap_or_default();
            owners
                .entry(rust_name)
                .and_modify(
                    |(owner, _, _, _): &mut (String, String, Vec<String>, bool)| {
                        if unit.namespace < *owner {
                            owner.clone_from(&unit.namespace);
                        }
                    },
                )
                .or_insert_with(|| (unit.namespace.clone(), path, generic_parameters, false));
        }
    }
    let Some(emission_namespace) = package
        .units
        .iter()
        .filter(|unit| unit.bundled && unit.namespace.starts_with("/deps/"))
        .map(|unit| unit.namespace.as_str())
        .min()
    else {
        return owners;
    };
    let mut source_objects = BTreeSet::new();
    for unit in &package.units {
        if unit.bundled && unit.namespace.starts_with("/deps/") {
            continue;
        }
        for binding in &unit.typed_bindings {
            collect_value_type_objects(&binding.value_type, &mut source_objects);
        }
        for function in &unit.functions {
            if let Some(return_type) = &function.return_type {
                collect_value_type_objects(return_type, &mut source_objects);
            }
            for parameter in &function.parameters {
                if let Some(value_type) = &parameter.value_type {
                    collect_value_type_objects(value_type, &mut source_objects);
                }
            }
        }
    }
    for identity in source_objects
        .into_iter()
        .filter(|identity| identity.namespace.starts_with("/deps/"))
    {
        if package.units.iter().any(|unit| {
            unit.descriptors
                .iter()
                .any(|object| object.identity == identity && object.kind == ObjectKind::Interface)
        }) {
            continue;
        }
        let projected = package
            .projection
            .projected_type(&identity.namespace, &identity.name);
        let path = package
            .projection
            .foreign_rust_path(&identity.namespace, &identity.name)
            .map(str::to_owned)
            .or_else(|| {
                projected
                    .as_ref()
                    .map(crate::projection::ProjectedType::rust_type)
            });
        let Some(path) = path else {
            continue;
        };
        let rust_name = rust_object_type_name(package, &identity);
        let generic_parameters = projected
            .as_ref()
            .map(projected_generic_names)
            .unwrap_or_default();
        owners.entry(rust_name).or_insert_with(|| {
            (
                emission_namespace.to_owned(),
                path,
                generic_parameters,
                true,
            )
        });
    }
    owners
}

pub(super) fn emit_dependency_imports(
    import_owners: &DependencyImportOwners,
    unit: &SemanticUnit,
    output: &mut String,
) {
    let first_namespace = import_owners.values().map(|(owner, _, _, _)| owner).min();
    if first_namespace.is_some_and(|namespace| namespace == &unit.namespace) {
        for (rust_name, (_, path, generic_parameters, _)) in import_owners {
            write_foreign_import(output, path, rust_name, generic_parameters);
        }
    }
}

fn projected_generic_names(ty: &crate::projection::ProjectedType) -> Vec<String> {
    fn collect(ty: &crate::projection::ProjectedType, names: &mut BTreeSet<String>) {
        use crate::projection::ProjectedType;
        match ty {
            ProjectedType::Generic(name) => {
                names.insert(name.clone());
            }
            ProjectedType::Sequence { item, .. }
            | ProjectedType::Set { item, .. }
            | ProjectedType::AsyncIterationStep(item)
            | ProjectedType::Optional(item) => collect(item, names),
            ProjectedType::Mapping { key, value, .. } => {
                collect(key, names);
                collect(value, names);
            }
            ProjectedType::Tuple(items) => {
                for item in items {
                    collect(item, names);
                }
            }
            ProjectedType::Foreign { arguments, .. } => {
                for argument in arguments {
                    collect(argument, names);
                }
            }
            ProjectedType::BoxedInterface {
                associated_type: Some(associated),
                ..
            } => collect(&associated.ty, names),
            ProjectedType::Callback {
                parameters, result, ..
            } => {
                for parameter in parameters {
                    collect(parameter, names);
                }
                collect(result, names);
            }
            _ => {}
        }
    }
    let mut names = BTreeSet::new();
    collect(ty, &mut names);
    names.into_iter().collect()
}

fn canonical_foreign_import_path(path: &str) -> std::borrow::Cow<'_, str> {
    if path == "std::io::error::Error" {
        "std::io::Error".into()
    } else if let Some(suffix) = path.strip_prefix("core::net::socket_addr::") {
        format!("std::net::{suffix}").into()
    } else {
        path.into()
    }
}

pub(super) fn write_foreign_import(
    output: &mut String,
    path: &str,
    rust_name: &str,
    generic_parameters: &[String],
) {
    let path = canonical_foreign_import_path(path);
    if path.contains("<'_>") {
        return;
    }
    if path.contains('<') || path.starts_with('(') || path.starts_with('[') {
        let parameters = if generic_parameters.is_empty() {
            String::new()
        } else {
            format!("<{}>", generic_parameters.join(", "))
        };
        writeln!(output, "pub type {rust_name}{parameters} = {path};")
            .expect("writing to a string cannot fail");
    } else if path.rsplit("::").next() == Some(rust_name) {
        writeln!(output, "pub use {path};").expect("writing to a string cannot fail");
    } else {
        writeln!(output, "pub use {path} as {rust_name};")
            .expect("writing to a string cannot fail");
    }
}

fn projected_type_is_identity(ty: &crate::projection::ProjectedType) -> bool {
    match ty {
        crate::projection::ProjectedType::Optional(inner) => projected_type_is_identity(inner),
        crate::projection::ProjectedType::None
        | crate::projection::ProjectedType::Bool
        | crate::projection::ProjectedType::FixedInt(_)
        | crate::projection::ProjectedType::Float
        | crate::projection::ProjectedType::Float32
        | crate::projection::ProjectedType::String
        | crate::projection::ProjectedType::Bytes
        | crate::projection::ProjectedType::Foreign { .. } => true,
        _ => false,
    }
}

fn projected_sequence_is_vec(path: &str) -> bool {
    path.starts_with("alloc::vec::Vec<") || path.starts_with("std::vec::Vec<")
}

pub(super) fn projected_callback_input_expression(
    value: &str,
    ty: &crate::projection::ProjectedType,
) -> String {
    match ty {
        crate::projection::ProjectedType::Int => {
            format!("terrane_int_support::Int::from(i128::from({value}))")
        }
        _ => projected_result_expression(value, ty),
    }
}

pub(super) fn projected_callback_output_expression(
    value: &str,
    ty: &crate::projection::ProjectedType,
) -> String {
    match ty {
        crate::projection::ProjectedType::Int => format!(
            "terrane_int_support::coerce::<i64>(&{value}).map_err(|error| crate::TerraneForeignError(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)))?"
        ),
        _ => projected_argument_expression(value, ty),
    }
}

fn projected_callback_argument(
    name: &str,
    parameters: &[crate::projection::ProjectedType],
    result: &crate::projection::ProjectedType,
    invocation_mode: InvocationMode,
    is_async: bool,
) -> String {
    let rust_parameters = parameters
        .iter()
        .enumerate()
        .map(|(index, parameter)| format!("callback_argument_{index}: {}", parameter.rust_type()))
        .collect::<Vec<_>>()
        .join(", ");
    let terrane_arguments = parameters
        .iter()
        .enumerate()
        .map(|(index, parameter)| {
            projected_callback_input_expression(&format!("callback_argument_{index}"), parameter)
        })
        .collect::<Vec<_>>();
    let direct_arguments = terrane_arguments.join(", ");
    let tuple_arguments = match terrane_arguments.as_slice() {
        [] => "()".to_owned(),
        [argument] => format!("({argument},)"),
        _ => format!("({direct_arguments})"),
    };
    let converted_result = projected_callback_output_expression("callback_value", result);
    let invoke = if invocation_mode == InvocationMode::Shared {
        format!("(callback)({direct_arguments})")
    } else {
        format!("callback.call({tuple_arguments})")
    };
    let fallible_body = if is_async {
        format!(
            "async {{ let callback_value = callback_future.await; Ok::<_, crate::TerraneForeignError>({converted_result}) }}.await"
        )
    } else {
        format!(
            "(|| -> Result<_, crate::TerraneForeignError> {{ let callback_value = {invoke}; Ok({converted_result}) }})()"
        )
    };
    let body = format!(
        "match {fallible_body} {{ Ok(value) => value, Err(error) => std::panic::panic_any(error.0) }}"
    );
    let capture = if invocation_mode == InvocationMode::Consuming {
        name.to_owned()
    } else {
        format!("{name}.clone()")
    };
    if is_async {
        if invocation_mode == InvocationMode::Shared {
            format!(
                "{{ let callback = {capture}; move |{rust_parameters}| {{ let callback = callback.clone(); let callback_future = callback({direct_arguments}); Box::pin(async move {{ {body} }}) }} }}"
            )
        } else {
            format!(
                "{{ let callback = {capture}; move |{rust_parameters}| {{ let callback_future = {invoke}; Box::pin(async move {{ {body} }}) }} }}"
            )
        }
    } else {
        format!("{{ let callback = {capture}; move |{rust_parameters}| {{ {body} }} }}")
    }
}

pub(super) fn projected_argument_expression(
    name: &str,
    ty: &crate::projection::ProjectedType,
) -> String {
    match ty {
        crate::projection::ProjectedType::Int => format!(
            "terrane_int_support::coerce::<i64>(&{name}).map_err(|error| crate::TerraneForeignError(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)))?"
        ),
        crate::projection::ProjectedType::RustInt(rust_type) => format!(
            "terrane_int_support::coerce::<{rust_type}>(&{name}).map_err(|error| crate::TerraneForeignError(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)))?"
        ),
        crate::projection::ProjectedType::Char => format!(
            "{name}.parse::<char>().map_err(|_| crate::TerraneForeignError(crate::TerraneError::raised_with_message(crate::TerraneErrorKind::CoercionError, \"projected `char` requires exactly one Unicode scalar\", crate::TERRANE_NO_SITE)))?"
        ),
        crate::projection::ProjectedType::BoxedInterface { .. } => {
            format!("Box::new({name})")
        }
        crate::projection::ProjectedType::Callback {
            parameters,
            result,
            invocation_mode,
            is_async,
            ..
        } => projected_callback_argument(name, parameters, result, *invocation_mode, *is_async),
        crate::projection::ProjectedType::Optional(inner) => {
            if projected_type_is_identity(inner) {
                name.to_owned()
            } else {
                let converted = projected_argument_expression("value", inner);
                format!(
                    "{name}.map(|value| -> Result<_, crate::TerraneForeignError> {{ Ok({converted}) }}).transpose()?"
                )
            }
        }
        crate::projection::ProjectedType::Sequence { rust_path, item } => {
            if projected_sequence_is_vec(rust_path) && projected_type_is_identity(item) {
                format!("{name}.into_vec()")
            } else {
                let converted = projected_argument_expression("item", item);
                format!(
                    "{name}.into_iter().map(|item| -> Result<_, crate::TerraneForeignError> {{ Ok({converted}) }}).collect::<Result<{rust_path}, _>>()?"
                )
            }
        }
        crate::projection::ProjectedType::Set {
            rust_path, item, ..
        } => {
            let converted = projected_argument_expression("item", item);
            format!(
                "{name}.into_iter().map(|item| -> Result<_, crate::TerraneForeignError> {{ Ok({converted}) }}).collect::<Result<{rust_path}, _>>()?"
            )
        }
        crate::projection::ProjectedType::Mapping {
            rust_path,
            key,
            value,
            ..
        } => {
            let key = projected_argument_expression("entry.key", key);
            let value = projected_argument_expression("entry.value", value);
            format!(
                "terrane_collection_support::Iterable::terrane_iterator(&{name}).map(|entry| -> Result<_, crate::TerraneForeignError> {{ Ok(({key}, {value})) }}).collect::<Result<{rust_path}, _>>()?"
            )
        }
        crate::projection::ProjectedType::Tuple(items) => {
            let converted = items
                .iter()
                .map(|item| {
                    projected_argument_expression(
                        "tuple_items.next().ok_or_else(|| crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, \"projected tuple length did not match its checked type\", crate::TERRANE_NO_SITE)))?",
                        item,
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{{ let mut tuple_items = {name}.try_into_iter().map_err(|_| crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, \"projected tuple cannot move shared elements\", crate::TERRANE_NO_SITE)))?; ({converted},) }}"
            )
        }
        _ => name.to_owned(),
    }
}

pub(super) fn projected_chain_argument_expression(
    name: &str,
    ty: &crate::projection::ProjectedType,
) -> String {
    if projected_type_is_identity(ty) {
        return name.to_owned();
    }
    let converted = projected_argument_expression(name, ty);
    format!(
        "match (|| -> Result<_, crate::TerraneForeignError> {{ Ok({converted}) }})() {{ Ok(value) => value, Err(error) => std::panic::panic_any(error) }}"
    )
}

pub(super) fn projected_result_expression(
    value: &str,
    ty: &crate::projection::ProjectedType,
) -> String {
    match ty {
        crate::projection::ProjectedType::Int => {
            format!("terrane_int_support::Int::from(i128::from({value}))")
        }
        crate::projection::ProjectedType::RustInt(rust_type) if rust_type.starts_with('u') => {
            format!("terrane_int_support::Int::from_u128({value} as u128)")
        }
        crate::projection::ProjectedType::RustInt(_) => {
            format!("terrane_int_support::Int::from({value} as i128)")
        }
        crate::projection::ProjectedType::Char => format!("{value}.to_string()"),
        crate::projection::ProjectedType::Optional(inner) => {
            if projected_type_is_identity(inner) {
                value.to_owned()
            } else {
                let converted = projected_result_expression("value", inner);
                format!("{value}.map(|value| {converted})")
            }
        }
        crate::projection::ProjectedType::AsyncIterationStep(item) => {
            let converted = projected_result_expression("item", item);
            format!(
                "match {value} {{ Some(item) => terrane_collection_support::AsyncIterationStep::item({converted}), None => terrane_collection_support::AsyncIterationStep::end() }}"
            )
        }
        crate::projection::ProjectedType::AsyncSinkOutcome => {
            format!("terrane_collection_support::AsyncSinkOutcome::from_accepted({value})")
        }
        crate::projection::ProjectedType::Sequence { item, .. } => {
            if projected_type_is_identity(item) {
                format!("terrane_collection_support::List::new({value})")
            } else {
                let converted = projected_result_expression("item", item);
                format!(
                    "terrane_collection_support::List::new({value}.into_iter().map(|item| {converted}).collect())"
                )
            }
        }
        crate::projection::ProjectedType::Mapping {
            key,
            value: item,
            ordered,
            ..
        } => {
            let key = projected_result_expression("key", key);
            let item = projected_result_expression("item", item);
            let collection = if *ordered { "Map" } else { "UnorderedMap" };
            format!(
                "terrane_collection_support::{collection}::new({value}.into_iter().map(|(key, item)| terrane_collection_support::Entry::new({key}, {item})).collect())"
            )
        }
        crate::projection::ProjectedType::Set { item, ordered, .. } => {
            let converted = projected_result_expression("item", item);
            let collection = if *ordered { "Set" } else { "UnorderedSet" };
            format!(
                "terrane_collection_support::{collection}::new({value}.into_iter().map(|item| {converted}).collect())"
            )
        }
        crate::projection::ProjectedType::Tuple(items) => format!(
            "terrane_collection_support::Tuple::new(vec![{}])",
            items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    projected_result_expression(&format!("{value}.{index}"), item)
                })
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => value.to_owned(),
    }
}

pub(super) type StaticMethodReferences = BTreeSet<(String, String, String)>;

pub(super) fn index_projected_static_method_references(
    package: &SemanticPackage,
) -> StaticMethodReferences {
    fn collect(
        package: &SemanticPackage,
        unit: &SemanticUnit,
        node: &SyntaxNode,
        references: &mut StaticMethodReferences,
    ) {
        if node.kind == SyntaxKind::StaticMemberExpression
            && let [receiver, member] = node.children.as_slice()
            && let Some(symbol) = package.resolve_name_at(
                unit,
                receiver.span.start,
                &unit.source.text()[receiver.span.start..receiver.span.end],
            )
        {
            references.insert((
                symbol.namespace.clone(),
                symbol.name.clone(),
                unit.source.text()[member.span.start..member.span.end].to_owned(),
            ));
        }
        for child in &node.children {
            collect(package, unit, child, references);
        }
    }

    let mut references = BTreeSet::new();
    for unit in &package.units {
        if unit.bundled && unit.namespace.starts_with("/deps/") {
            continue;
        }
        collect(package, unit, &unit.tree.root, &mut references);
    }
    references
}

#[expect(
    clippy::too_many_lines,
    reason = "dependency shim emission keeps each generated branch beside the shared call contract"
)]
pub(super) fn emit_dependency_unit(
    package: &SemanticPackage,
    registry: &LoweringRegistry,
    unit: &SemanticUnit,
    static_method_references: &StaticMethodReferences,
    import_owners: &DependencyImportOwners,
) -> String {
    let mut output = String::new();
    emit_dependency_imports(import_owners, unit, &mut output);
    for contract in &unit.functions {
        let (item, projected, static_owner) = if let Some(owner) =
            contract.owner.as_deref().filter(|_| contract.is_static)
        {
            let Some(identity) = contract.owner_identity.as_ref() else {
                continue;
            };
            if !static_method_references.contains(&(
                identity.namespace.clone(),
                identity.name.clone(),
                contract.name.clone(),
            )) {
                continue;
            }
            let type_name = unit
                .descriptors
                .iter()
                .find(|object| object.identity.name == owner)
                .map_or(owner, |object| object.name.as_str());
            let Some(item) = package.projection.item(&unit.namespace, type_name) else {
                continue;
            };
            let crate::projection::ProjectedKind::ForeignType { static_methods, .. } = &item.kind
            else {
                continue;
            };
            let Some(projected) = static_methods
                .iter()
                .find(|method| method.name == contract.name)
            else {
                continue;
            };
            (item, projected, Some(type_name))
        } else {
            let Some(item) = package.projection.item(&unit.namespace, &contract.name) else {
                continue;
            };
            let crate::projection::ProjectedKind::Function(projected) = &item.kind else {
                continue;
            };
            (item, projected, None)
        };
        if projected
            .generic_parameters
            .iter()
            .any(|generic| generic.input_selected)
        {
            continue;
        }
        if projected.chain_role == Some(crate::projection::ChainRole::Root) {
            continue;
        }
        let dependency_name = unit
            .namespace
            .strip_prefix("/deps/")
            .and_then(|namespace| namespace.split('/').next())
            .unwrap_or("dependency");
        let parameters = contract
            .parameters
            .iter()
            .zip(&projected.parameters)
            .map(|(parameter, projected)| {
                let value_type = projected.generic_parameter.clone().unwrap_or_else(|| {
                    parameter.value_type.clone().map_or_else(
                        || "()".to_owned(),
                        |value_type| rust_value_type(package, value_type),
                    )
                });
                let preserves_identity = projected.borrowed
                    && matches!(
                        projected.ty,
                        crate::projection::ProjectedType::Foreign { .. }
                    );
                format!(
                    "{}: {}{value_type}",
                    rust_name(&parameter.name),
                    if preserves_identity {
                        if projected.mutable_borrow {
                            "&mut "
                        } else {
                            "&"
                        }
                    } else {
                        ""
                    },
                )
            })
            .collect::<Vec<_>>();
        let mut argument_conversions = Vec::new();
        let mut arguments = Vec::new();
        for (parameter, projected) in contract.parameters.iter().zip(&projected.parameters) {
            let name = rust_name(&parameter.name);
            if projected.borrowed
                && matches!(
                    projected.ty,
                    crate::projection::ProjectedType::Foreign { .. }
                )
            {
                arguments.push(name);
                continue;
            }
            let value = projected_argument_expression(&name, &projected.ty);
            argument_conversions.push(format!(
                "    let {}{name} = {value};",
                if projected.mutable_borrow { "mut " } else { "" }
            ));
            arguments.push(if projected.mutable_borrow {
                format!("&mut {name}")
            } else if projected.borrowed {
                format!("&{name}")
            } else {
                name
            });
        }
        let arguments = arguments.join(", ");
        let value = projected.destination_result.as_ref().map_or_else(
            || {
                contract.return_type.clone().map_or_else(
                    || "()".to_owned(),
                    |value_type| rust_value_type(package, value_type),
                )
            },
            |_| projected.result.rust_type(),
        );
        let result = format!("Result<{value}, crate::TerraneForeignError>");
        let error_kind = projected
            .error
            .as_deref()
            .and_then(|rust_path| {
                package
                    .projection
                    .projected_identity_for_rust_path(rust_path)
                    .map(|(namespace, name)| (format!("{namespace}::{name}"), name.to_owned()))
            })
            .map_or_else(
                || "TERRANE_DEPENDENCY_ERROR".to_owned(),
                |(identity, name)| {
                    let descriptor = registry.register_descriptor(&identity, &name);
                    format!("DescriptorId({descriptor})")
                },
            );
        let converted_value = if projected.destination_result.is_some() {
            "value".to_owned()
        } else {
            projected_result_expression("value", &projected.result)
        };
        let nested_converted_value = match &projected.result {
            crate::projection::ProjectedType::Optional(inner)
                if projected.error_optional_depth == 1 =>
            {
                projected_result_expression("value", inner)
            }
            _ => converted_value.clone(),
        };
        let unit_variant = package.projection.is_unit_variant(item);
        if unit_variant {
            writeln!(
                output,
                "/// Projected enum variant constructor for `{}`.",
                item.rust_path
            )
            .expect("writing to a string cannot fail");
        }
        let mut generic_parameters = projected
            .parameters
            .iter()
            .filter_map(|parameter| {
                parameter.generic_parameter.as_ref().map(|name| {
                    if parameter.generic_bounds.is_empty() {
                        name.clone()
                    } else {
                        format!("{name}: {}", parameter.generic_bounds.join(" + "))
                    }
                })
            })
            .collect::<BTreeSet<_>>();
        if let Some(destination) = &projected.destination_result {
            generic_parameters.insert(if destination.rust_bounds.is_empty() {
                destination.parameter.clone()
            } else {
                format!(
                    "{}: {}",
                    destination.parameter,
                    destination.rust_bounds.join(" + ")
                )
            });
        }
        let generic_declaration = if generic_parameters.is_empty() {
            String::new()
        } else {
            format!(
                "<{}>",
                generic_parameters
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        writeln!(
            output,
            "pub {}fn {}{generic_declaration}({}) -> {result} {{",
            if projected.is_async { "async " } else { "" },
            static_owner.map_or_else(
                || function_name(package, contract),
                |owner| projected_static_shim_name(owner, &contract.name),
            ),
            parameters.join(", ")
        )
        .expect("writing to a string cannot fail");
        for conversion in argument_conversions {
            writeln!(output, "{conversion}").expect("writing to a string cannot fail");
        }
        let value_path = static_owner.map_or_else(
            || rust_value_path(&item.rust_path),
            |_| {
                format!(
                    "{}::{}",
                    rust_value_path(&item.rust_path),
                    rust_name(&projected.name)
                )
            },
        );
        let call = if unit_variant {
            value_path
        } else {
            let generic_arguments = projected
                .destination_result
                .as_ref()
                .map_or_else(String::new, |destination| {
                    format!("::<{}>", destination.parameter)
                });
            format!("{value_path}{generic_arguments}({arguments})")
        };
        let invocation = if projected.is_async {
            format!("{call}.await")
        } else {
            call.clone()
        };
        let caught = if projected.is_async {
            format!("crate::__terrane_dependency_await_unwind({call}).await")
        } else {
            format!("std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {call}))")
        };
        if package.profile.panic == crate::package::PanicProfile::Abort {
            if projected.error_optional_depth == 1 {
                writeln!(
                    output,
                    "    match {invocation} {{\n        None => Ok(None),\n        Some(Ok(value)) => Ok(Some({nested_converted_value})),\n        Some(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, format!(\"Rust dependency `{dependency_name}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n    }}",
                    item.rust_path,
                )
                .expect("writing to a string cannot fail");
            } else if projected.error.is_some() {
                writeln!(
                    output,
                    "    match {invocation} {{\n        Ok(value) => Ok({converted_value}),\n        Err(error) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, format!(\"Rust dependency `{dependency_name}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n    }}",
                    item.rust_path,
                )
                .expect("writing to a string cannot fail");
            } else {
                writeln!(
                    output,
                    "    let value = {invocation};\n    Ok({converted_value})",
                )
                .expect("writing to a string cannot fail");
            }
        } else if projected.error_optional_depth == 1 {
            writeln!(
                output,
                "    match {caught} {{\n        Ok(None) => Ok(None),\n        Ok(Some(Ok(value))) => Ok(Some({nested_converted_value})),\n        Ok(Some(Err(error))) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, format!(\"Rust dependency `{}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n        Err(payload) => Err(crate::__terrane_dependency_panic(payload, {:?}, {:?})),\n    }}",
                dependency_name,
                item.rust_path,
                dependency_name,
                item.rust_path,
            )
            .expect("writing to a string cannot fail");
        } else if projected.error.is_some() {
            writeln!(
                output,
                "    match {caught} {{\n        Ok(Ok(value)) => Ok({converted_value}),\n        Ok(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::{error_kind}, format!(\"Rust dependency `{}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n        Err(payload) => Err(crate::__terrane_dependency_panic(payload, {:?}, {:?})),\n    }}",
                dependency_name,
                item.rust_path,
                dependency_name,
                item.rust_path,
            )
            .expect("writing to a string cannot fail");
        } else {
            writeln!(
                output,
                "    match {caught} {{\n        Ok(value) => Ok({converted_value}),\n        Err(payload) => Err(crate::__terrane_dependency_panic(payload, {:?}, {:?})),\n    }}",
                dependency_name,
                item.rust_path,
            )
            .expect("writing to a string cannot fail");
        }

        output.push_str("}\n");
    }
    output
}
pub(super) fn projected_static_shim_name(owner: &str, method: &str) -> String {
    format!(
        "terrane_static_{}_{}",
        rust_name(owner).trim_start_matches('_'),
        rust_name(method)
    )
}
fn rust_value_path(path: &str) -> String {
    if path.starts_with('<') {
        return path.to_owned();
    }
    path.find('<').map_or_else(
        || path.to_owned(),
        |arguments| format!("{}::{}", &path[..arguments], &path[arguments..]),
    )
}

#[cfg(test)]
mod tests {
    use super::write_foreign_import;

    #[test]
    fn instantiated_foreign_import_is_a_type_alias() {
        let mut output = String::new();
        write_foreign_import(&mut output, "witness::Wrapper<u8>", "Wrapper_abcd", &[]);
        assert_eq!(output, "pub type Wrapper_abcd = witness::Wrapper<u8>;\n");
    }

    #[test]
    fn open_foreign_import_declares_its_type_parameters() {
        let mut output = String::new();
        write_foreign_import(
            &mut output,
            "witness::Wrapper<T>",
            "Wrapper_open",
            &["T".to_owned()],
        );
        assert_eq!(output, "pub type Wrapper_open<T> = witness::Wrapper<T>;\n");
    }

    #[test]
    fn private_standard_library_paths_use_public_reexports() {
        let mut output = String::new();
        write_foreign_import(&mut output, "std::io::error::Error", "Error", &[]);
        write_foreign_import(
            &mut output,
            "core::net::socket_addr::SocketAddr",
            "SocketAddr",
            &[],
        );
        assert_eq!(
            output,
            "pub use std::io::Error;\npub use std::net::SocketAddr;\n"
        );
    }
}
