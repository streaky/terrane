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
    if path.rsplit("::").next() == Some(rust_name) {
        writeln!(output, "pub use {path};").expect("writing to a string cannot fail");
    } else {
        writeln!(output, "pub use {path} as {rust_name};")
            .expect("writing to a string cannot fail");
    }
}

pub(super) fn projected_argument_expression(
    name: &str,
    ty: &crate::projection::ProjectedType,
) -> String {
    match ty {
        crate::projection::ProjectedType::RustInt(rust_type) => format!(
            "terrane_int_support::coerce::<{rust_type}>(&{name}).map_err(|error| crate::TerraneForeignError(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)))?"
        ),
        crate::projection::ProjectedType::Char => format!(
            "{name}.parse::<char>().map_err(|_| crate::TerraneForeignError(crate::TerraneError::raised_with_message(crate::TerraneErrorKind::CoercionError, \"projected `char` requires exactly one Unicode scalar\", crate::TERRANE_NO_SITE)))?"
        ),
        crate::projection::ProjectedType::Optional(inner) => match inner.as_ref() {
            crate::projection::ProjectedType::RustInt(rust_type) => format!(
                "{name}.map(|value| terrane_int_support::coerce::<{rust_type}>(&value)).transpose().map_err(|error| crate::TerraneForeignError(crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE)))?"
            ),
            crate::projection::ProjectedType::Char => format!(
                "{name}.map(|value| value.parse::<char>().map_err(|_| crate::TerraneForeignError(crate::TerraneError::raised_with_message(crate::TerraneErrorKind::CoercionError, \"projected `char` requires exactly one Unicode scalar\", crate::TERRANE_NO_SITE)))).transpose()?"
            ),
            _ => name.to_owned(),
        },
        _ => name.to_owned(),
    }
}

pub(super) fn projected_result_expression(
    value: &str,
    ty: &crate::projection::ProjectedType,
) -> String {
    match ty {
        crate::projection::ProjectedType::RustInt(rust_type) if rust_type.starts_with('u') => {
            format!("terrane_int_support::Int::from_u128({value} as u128)")
        }
        crate::projection::ProjectedType::RustInt(_) => {
            format!("terrane_int_support::Int::from({value} as i128)")
        }
        crate::projection::ProjectedType::Char => format!("{value}.to_string()"),
        crate::projection::ProjectedType::Optional(inner) => {
            let converted = projected_result_expression("value", inner);
            if converted == "value" {
                value.to_owned()
            } else {
                format!("{value}.map(|value| {converted})")
            }
        }
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
            "pub fn {}({}) -> {result} {{",
            rust_name(&contract.name),
            parameters.join(", ")
        )
        .expect("writing to a string cannot fail");
        for conversion in argument_conversions {
            writeln!(output, "{conversion}").expect("writing to a string cannot fail");
        }
        let call = if unit_variant {
            item.rust_path.clone()
        } else {
            format!("{}({arguments})", item.rust_path)
        };
        let caught = if projected
            .parameters
            .iter()
            .any(|parameter| parameter.mutable_borrow)
        {
            format!("std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {call}))")
        } else {
            format!("std::panic::catch_unwind(|| {call})")
        };
        if package.profile.panic == crate::package::PanicProfile::Abort {
            if projected.error.is_some() {
                writeln!(
                    output,
                    "    match {call} {{\n        Ok(value) => Ok({converted_value}),\n        Err(error) => Err(crate::TerraneForeignError(crate::TerraneError::custom_raised(crate::TERRANE_DEPENDENCY_ERROR, format!(\"Rust dependency `{dependency_name}` member `{}` failed: {{error}}\"), crate::TERRANE_NO_SITE))),\n    }}",
                    item.rust_path,
                )
                .expect("writing to a string cannot fail");
            } else {
                writeln!(output, "    let value = {call};\n    Ok({converted_value})",)
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
