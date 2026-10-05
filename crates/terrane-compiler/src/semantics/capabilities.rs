use super::model::{
    NativeTypeCapabilities, ObjectIdentity, SemanticFailure, SemanticPackage, ValueType,
};
use super::prelude::*;
use crate::rust_interop::{BoundQuestion, ProbeAnswer, ProjectionOracle};

impl SemanticPackage {
    pub(super) fn native_capability(
        &self,
        identity: &ObjectIdentity,
    ) -> Option<&NativeTypeCapabilities> {
        self.native_capabilities
            .get(identity.native_projection.as_deref()?)
    }
}

pub(super) fn populate_native_capabilities(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    let mut native_types = BTreeMap::<String, (usize, Span)>::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        visit_unit_types(unit, &mut |value_type, span| {
            collect_native_types(value_type, span, unit_index, &mut native_types);
        });
    }
    native_types.retain(|rust_type, _| {
        !package.native_capabilities.contains_key(rust_type)
            && !is_borrowed_native_type(&package.projection, rust_type)
    });
    prove_native_capabilities(package, &native_types)?;
    // Nominal facts still apply when there are no fresh concrete proofs to request.
    for unit in &mut package.units {
        let mut nonclone_foreign_objects = std::mem::take(&mut unit.nonclone_foreign_objects);
        let mut objects = Vec::new();
        visit_unit_types(unit, &mut |value_type, _| {
            collect_objects(value_type, &mut objects);
        });
        for identity in objects {
            // An exact concrete proof supersedes the nominal declaration's Clone fact.
            let cloneable = identity
                .native_projection
                .as_deref()
                .and_then(|rust_type| package.native_capabilities.get(rust_type))
                .map_or_else(
                    || {
                        package
                            .projection
                            .foreign_is_cloneable(&identity.namespace, &identity.name)
                    },
                    |capability| Some(capability.cloneable),
                );
            let borrowed = package
                .projection
                .item(&identity.namespace, &identity.name)
                .is_some_and(|item| {
                    matches!(
                        item.kind,
                        crate::rust_interop::projection::ProjectedKind::ForeignType {
                            borrowed_view: true,
                            ..
                        }
                    )
                })
                || identity
                    .native_projection
                    .as_deref()
                    .is_some_and(|rust_type| {
                        is_borrowed_native_type(&package.projection, rust_type)
                    });
            if cloneable == Some(false) && !borrowed {
                nonclone_foreign_objects.insert(identity.clone());
            } else {
                nonclone_foreign_objects.remove(identity);
            }
        }
        unit.nonclone_foreign_objects = nonclone_foreign_objects;
    }
    Ok(())
}

fn visit_unit_types<'a>(
    unit: &'a super::model::SemanticUnit,
    visit: &mut impl FnMut(&'a ValueType, Span),
) {
    for binding in &unit.typed_bindings {
        visit(&binding.value_type, binding.span);
    }
    for (&(file, start, end), value_type) in &unit.selected_expression_types {
        visit(value_type, Span::new(file, start, end));
    }
    for (&(file, start, end), value_type) in &unit.invocation_scoped_function_results {
        visit(value_type, Span::new(file, start, end));
    }
    for function in &unit.functions {
        for parameter in &function.parameters {
            if let Some(value_type) = &parameter.value_type {
                visit(value_type, parameter.span);
            }
        }
        if let Some(value_type) = &function.return_type {
            visit(value_type, function.span);
        }
        for value_type in &function.thrown_types {
            visit(value_type, function.span);
        }
    }
    for descriptor in &unit.descriptors {
        for field in &descriptor.fields {
            visit(&field.value_type, field.span);
        }
    }
    for (&(file, start, end), specialization) in &unit.projected_call_specializations {
        let span = Span::new(file, start, end);
        visit(&specialization.value_type, span);
        for value_type in specialization.value_parameters.iter().flatten() {
            visit(value_type, span);
        }
    }
}

fn prove_native_capabilities(
    package: &mut SemanticPackage,
    native_types: &BTreeMap<String, (usize, Span)>,
) -> Result<(), SemanticFailure> {
    if native_types.is_empty() {
        return Ok(());
    }
    let mut questions = BTreeSet::new();
    for rust_type in native_types.keys() {
        for rust_bound in [
            "core::clone::Clone",
            "core::marker::Send",
            "core::marker::Sync",
        ] {
            questions.insert(BoundQuestion {
                rust_type: rust_type.clone(),
                rust_bound: rust_bound.to_owned(),
                inferred_parameters: Vec::new(),
            });
        }
    }
    let projected_names = questions
        .iter()
        .flat_map(|question| [&question.rust_type, &question.rust_bound])
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
    let mut qualified_types = BTreeMap::new();
    let questions = questions
        .into_iter()
        .map(|mut question| {
            let original = question.rust_type.clone();
            question.rust_type =
                super::objects::qualify_projected_rust_names(&question.rust_type, &projected_names);
            qualified_types.insert(question.rust_type.clone(), original);
            question.rust_bound = super::objects::qualify_projected_rust_names(
                &question.rust_bound,
                &projected_names,
            );
            question
        })
        .collect::<Vec<_>>();
    let workspace = package.root.join(".trn/dependencies");
    let report = ProjectionOracle::new(
        &workspace,
        &package.projection.cache_identity,
        package.projection.containment,
    )
    .prove_bounds(&questions)
    .map_err(|error| {
        let &(unit_index, span) = native_types
            .values()
            .min_by_key(|(unit_index, span)| (*unit_index, span.start))
            .expect("nonempty capability questions retain their source sites");
        super::diagnostics::failure(
            &package.units[unit_index].source,
            "T0119",
            format!(
                "cannot prove selected native type capabilities: {}",
                error.message
            ),
            span,
        )
    })?;
    for evidence in report.evidence {
        let Some(rust_type) = qualified_types.get(&evidence.question.rust_type) else {
            continue;
        };
        let capability = package
            .native_capabilities
            .entry(rust_type.clone())
            .or_default();
        let satisfied = evidence.answer == ProbeAnswer::Yes;
        match evidence.question.rust_bound.as_str() {
            "core::clone::Clone" => capability.cloneable = satisfied,
            "core::marker::Send" => capability.send = satisfied,
            "core::marker::Sync" => capability.sync = satisfied,
            _ => {}
        }
    }
    Ok(())
}

fn collect_native_types(
    value_type: &ValueType,
    span: Span,
    unit_index: usize,
    types: &mut BTreeMap<String, (usize, Span)>,
) {
    walk_value_type(value_type, &mut |value_type| {
        if let ValueType::Object(identity) = value_type
            && let Some(native) = &identity.native_projection
            && identity.application.as_deref().is_none_or(is_closed)
            && identity.type_arguments.iter().all(is_closed)
            && identity.native_arguments.values().all(is_closed)
        {
            types.entry(native.clone()).or_insert((unit_index, span));
        }
    });
}

fn collect_objects<'a>(value_type: &'a ValueType, objects: &mut Vec<&'a ObjectIdentity>) {
    walk_value_type(value_type, &mut |value_type| {
        if let ValueType::Object(identity) = value_type {
            objects.push(identity);
        }
    });
}

fn walk_value_type<'a>(value_type: &'a ValueType, visit: &mut impl FnMut(&'a ValueType)) {
    visit(value_type);
    match value_type {
        ValueType::Object(identity) => {
            if let Some(application) = &identity.application {
                walk_value_type(application, visit);
            }
            for argument in &identity.type_arguments {
                walk_value_type(argument, visit);
            }
            for argument in identity.native_arguments.values() {
                walk_value_type(argument, visit);
            }
        }
        ValueType::Optional(inner) => walk_value_type(inner, visit),
        ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::UnorderedSet(inner)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::Reference(inner)
        | ValueType::SharedReference(inner) => walk_value_type(inner.value_type_ref(), visit),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            walk_value_type(key.value_type_ref(), visit);
            walk_value_type(value.value_type_ref(), visit);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                walk_value_type(parameter.value_type_ref(), visit);
            }
            walk_value_type(result.value_type_ref(), visit);
        }
        _ => {}
    }
}
fn is_borrowed_native_type(
    projection: &crate::rust_interop::projection::Projection,
    rust_type: &str,
) -> bool {
    let base = rust_type
        .split_once('<')
        .map_or(rust_type, |(base, _)| base);
    projection
        .dependencies
        .iter()
        .flat_map(|dependency| &dependency.items)
        .any(|item| {
            item.rust_path
                .split_once('<')
                .map_or(item.rust_path.as_str(), |(path, _)| path)
                == base
                && matches!(
                    item.kind,
                    crate::rust_interop::projection::ProjectedKind::ForeignType {
                        borrowed_view: true,
                        ..
                    }
                )
        })
}

fn is_closed(value_type: &ValueType) -> bool {
    let mut closed = true;
    walk_value_type(value_type, &mut |value_type| {
        if matches!(
            value_type,
            ValueType::ProjectedGeneric(_) | ValueType::TypeParameter(_)
        ) {
            closed = false;
        }
    });
    closed
}
