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

#[expect(
    clippy::too_many_lines,
    reason = "Capability collection, proof caching, and fail-closed application are one ordered closed-world analysis."
)]
pub(super) fn populate_native_capabilities(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    let mut native_types = BTreeSet::<String>::new();
    for unit in &package.units {
        for binding in &unit.typed_bindings {
            collect_native_types(&binding.value_type, &mut native_types);
        }
        for value_type in unit.selected_expression_types.values() {
            collect_native_types(value_type, &mut native_types);
        }
        for value_type in unit.invocation_scoped_function_results.values() {
            collect_native_types(value_type, &mut native_types);
        }
        for function in &unit.functions {
            for parameter in &function.parameters {
                if let Some(value_type) = &parameter.value_type {
                    collect_native_types(value_type, &mut native_types);
                }
            }
            if let Some(value_type) = &function.return_type {
                collect_native_types(value_type, &mut native_types);
            }
            for value_type in &function.thrown_types {
                collect_native_types(value_type, &mut native_types);
            }
        }
        for descriptor in &unit.descriptors {
            for field in &descriptor.fields {
                collect_native_types(&field.value_type, &mut native_types);
            }
        }
        for specialization in unit.projected_call_specializations.values() {
            collect_native_types(&specialization.value_type, &mut native_types);
            for value_type in specialization.value_parameters.iter().flatten() {
                collect_native_types(value_type, &mut native_types);
            }
        }
    }
    native_types.retain(|rust_type| {
        !package.native_capabilities.contains_key(rust_type)
            && !is_borrowed_native_type(&package.projection, rust_type)
    });
    prove_native_capabilities(package, &native_types)?;
    // Nominal facts still apply when there are no fresh concrete proofs to request.
    for unit in &mut package.units {
        let mut objects = Vec::new();
        for binding in &unit.typed_bindings {
            collect_objects(&binding.value_type, &mut objects);
        }
        for value_type in unit.selected_expression_types.values() {
            collect_objects(value_type, &mut objects);
        }
        for value_type in unit.invocation_scoped_function_results.values() {
            collect_objects(value_type, &mut objects);
        }
        for function in &unit.functions {
            for parameter in &function.parameters {
                if let Some(value_type) = &parameter.value_type {
                    collect_objects(value_type, &mut objects);
                }
            }
            if let Some(value_type) = &function.return_type {
                collect_objects(value_type, &mut objects);
            }
            for value_type in &function.thrown_types {
                collect_objects(value_type, &mut objects);
            }
        }
        for descriptor in &unit.descriptors {
            for field in &descriptor.fields {
                collect_objects(&field.value_type, &mut objects);
            }
        }
        for specialization in unit.projected_call_specializations.values() {
            collect_objects(&specialization.value_type, &mut objects);
            for value_type in specialization.value_parameters.iter().flatten() {
                collect_objects(value_type, &mut objects);
            }
        }
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
                unit.nonclone_foreign_objects.insert(identity.clone());
            } else {
                unit.nonclone_foreign_objects.remove(identity);
            }
        }
    }
    Ok(())
}

fn prove_native_capabilities(
    package: &mut SemanticPackage,
    native_types: &BTreeSet<String>,
) -> Result<(), SemanticFailure> {
    if native_types.is_empty() {
        return Ok(());
    }
    let mut questions = BTreeSet::new();
    for rust_type in native_types {
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
        let unit = package
            .units
            .first()
            .expect("native types were collected from semantic units");
        super::diagnostics::failure(
            &unit.source,
            "T0119",
            format!(
                "cannot prove selected native type capabilities: {}",
                error.message
            ),
            Span::new(unit.source.id(), 0, 0),
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

fn collect_native_types(value_type: &ValueType, types: &mut BTreeSet<String>) {
    match value_type {
        ValueType::Object(identity) => {
            if let Some(native) = &identity.native_projection
                && identity.application.as_deref().is_none_or(is_closed)
                && identity.type_arguments.iter().all(is_closed)
                && identity.native_arguments.values().all(is_closed)
            {
                types.insert(native.clone());
            }
            if let Some(application) = &identity.application {
                collect_native_types(application, types);
            }
            for argument in &identity.type_arguments {
                collect_native_types(argument, types);
            }
            for argument in identity.native_arguments.values() {
                collect_native_types(argument, types);
            }
        }
        ValueType::Optional(inner) => collect_native_types(inner, types),
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
        | ValueType::SharedReference(inner) => collect_native_types(inner.value_type_ref(), types),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            collect_native_types(key.value_type_ref(), types);
            collect_native_types(value.value_type_ref(), types);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                collect_native_types(parameter.value_type_ref(), types);
            }
            collect_native_types(result.value_type_ref(), types);
        }
        _ => {}
    }
}

fn collect_objects<'a>(value_type: &'a ValueType, objects: &mut Vec<&'a ObjectIdentity>) {
    match value_type {
        ValueType::Object(identity) => {
            objects.push(identity);
            if let Some(application) = &identity.application {
                collect_objects(application, objects);
            }
            for argument in &identity.type_arguments {
                collect_objects(argument, objects);
            }
            for argument in identity.native_arguments.values() {
                collect_objects(argument, objects);
            }
        }
        ValueType::Optional(inner) => collect_objects(inner, objects),
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
        | ValueType::SharedReference(inner) => collect_objects(inner.value_type_ref(), objects),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            collect_objects(key.value_type_ref(), objects);
            collect_objects(value.value_type_ref(), objects);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                collect_objects(parameter.value_type_ref(), objects);
            }
            collect_objects(result.value_type_ref(), objects);
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
    match value_type {
        ValueType::ProjectedGeneric(_) | ValueType::TypeParameter(_) => false,
        ValueType::Object(identity) => {
            identity.application.as_deref().is_none_or(is_closed)
                && identity.type_arguments.iter().all(is_closed)
                && identity.native_arguments.values().all(is_closed)
        }
        ValueType::Optional(inner) => is_closed(inner),
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
        | ValueType::SharedReference(inner) => is_closed(inner.value_type_ref()),
        ValueType::Map(key, value)
        | ValueType::Entry(key, value)
        | ValueType::UnorderedMap(key, value) => {
            is_closed(key.value_type_ref()) && is_closed(value.value_type_ref())
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            parameters
                .iter()
                .all(|parameter| is_closed(parameter.value_type_ref()))
                && is_closed(result.value_type_ref())
        }
        _ => true,
    }
}
