use super::prelude::*;

pub(super) fn emit_dependency_imports(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    output: &mut String,
) {
    let mut imported = BTreeSet::new();
    for object in &unit.objects {
        if object.identity.namespace != unit.namespace {
            continue;
        }
        if let Some(path) = package
            .projection
            .foreign_rust_path(&object.identity.namespace, &object.identity.name)
        {
            let rust_name = rust_object_type_name(package, &object.identity);
            if !imported.insert(rust_name.clone()) {
                continue;
            }
            write_foreign_import(output, path, &rust_name);
        }
    }
    for (name, path) in package.projection.foreign_imports(&unit.namespace) {
        let rust_name = rust_object_name(&name);
        if imported.insert(rust_name.clone()) {
            write_foreign_import(output, &path, &rust_name);
        }
    }
}

pub(super) fn write_foreign_import(output: &mut String, path: &str, rust_name: &str) {
    if path.contains("<'_>") {
        return;
    }
    if path.contains('<') || path.starts_with('(') || path.starts_with('[') {
        writeln!(output, "pub type {rust_name} = {path};")
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

fn projected_callback_input_expression(
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

fn projected_callback_output_expression(
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
        .collect::<Vec<_>>()
        .join(", ");
    let converted_result = projected_callback_output_expression("callback_value", result);
    let invoke = if is_async {
        format!("({name})({terrane_arguments}).await")
    } else {
        format!("({name})({terrane_arguments})")
    };
    let fallible_body = if is_async {
        format!(
            "async {{ let callback_value = {invoke}; Ok::<_, crate::TerraneForeignError>({converted_result}) }}.await"
        )
    } else {
        format!(
            "(|| -> Result<_, crate::TerraneForeignError> {{ let callback_value = {invoke}.map_err(crate::TerraneForeignError)?; Ok({converted_result}) }})()"
        )
    };
    let body = format!(
        "match {fallible_body} {{ Ok(value) => value, Err(error) => std::panic::panic_any(error.0) }}"
    );
    if is_async {
        format!(
            "{{ let callback = {name}.clone(); move |{rust_parameters}| {{ let {name} = callback.clone(); Box::pin(async move {{ {body} }}) }} }}"
        )
    } else {
        format!("{{ let {name} = {name}.clone(); move |{rust_parameters}| {{ {body} }} }}")
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
        crate::projection::ProjectedType::Callback {
            parameters,
            result,
            is_async,
            ..
        } => projected_callback_argument(name, parameters, result, *is_async),
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

#[expect(
    clippy::too_many_lines,
    reason = "dependency shim emission keeps each generated branch beside the shared call contract"
)]
pub(super) fn emit_dependency_unit(package: &SemanticPackage, unit: &SemanticUnit) -> String {
    let mut output = String::new();
    emit_dependency_imports(package, unit, &mut output);
    for contract in unit
        .functions
        .iter()
        .filter(|contract| contract.owner.is_none())
    {
        let Some(item) = package.projection.item(&unit.namespace, &contract.name) else {
            continue;
        };
        let crate::projection::ProjectedKind::Function(projected) = &item.kind else {
            continue;
        };
        if projected.chain_role == Some(crate::projection::ChainRole::Root) {
            continue;
        }
        let dependency_name = package
            .projection
            .dependency_name(&unit.namespace, &contract.name)
            .unwrap_or("dependency");
        let parameters = contract
            .parameters
            .iter()
            .zip(&projected.parameters)
            .map(|(parameter, projected)| {
                let value_type = parameter.value_type.clone().map_or_else(
                    || "()".to_owned(),
                    |value_type| rust_value_type(package, value_type),
                );
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
        let value = contract.return_type.clone().map_or_else(
            || "()".to_owned(),
            |value_type| rust_value_type(package, value_type),
        );
        let result = format!("Result<{value}, crate::TerraneForeignError>");
        let converted_value = projected_result_expression("value", &projected.result);
        let unit_variant = package.projection.is_unit_variant(item);
        if unit_variant {
            writeln!(
                output,
                "/// Projected enum variant constructor for `{}`.",
                item.rust_path
            )
            .expect("writing to a string cannot fail");
        }
        writeln!(
            output,
            "pub {}fn {}({}) -> {result} {{",
            if projected.is_async { "async " } else { "" },
            rust_name(&contract.name),
            parameters.join(", ")
        )
        .expect("writing to a string cannot fail");
        for conversion in argument_conversions {
            writeln!(output, "{conversion}").expect("writing to a string cannot fail");
        }
        let value_path = rust_value_path(&item.rust_path);
        let call = if unit_variant {
            value_path
        } else {
            format!("{value_path}({arguments})")
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
            if projected.error.is_some() {
                writeln!(
                    output,
                    "    match {invocation} {{\n        Ok(value) => Ok({converted_value}),\n        Err(error) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{dependency_name}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n    }}",
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
        } else if projected.error.is_some() {
            writeln!(
                output,
                "    match {caught} {{\n        Ok(Ok(value)) => Ok({converted_value}),\n        Ok(Err(error)) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n        Err(payload) => Err(crate::__terrane_dependency_panic(payload, {:?}, {:?})),\n    }}",
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
        write_foreign_import(&mut output, "witness::Wrapper<u8>", "Wrapper_abcd");
        assert_eq!(output, "pub type Wrapper_abcd = witness::Wrapper<u8>;\n");
    }
}
