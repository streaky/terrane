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
            .get(native_rust_type(&self.projection, identity)?)
    }

    pub(crate) fn native_value_is_copyable(&self, value_type: &ValueType) -> bool {
        match value_type {
            ValueType::Object(identity) => self.native_capability(identity).map_or_else(
                || {
                    self.projection
                        .native_type_copyable(&identity.namespace, &identity.name)
                },
                |capability| capability.copyable,
            ),
            ValueType::InvocationScopedNative {
                rust_type, family, ..
            } => self.native_capabilities.get(rust_type).map_or_else(
                || {
                    self.projection
                        .native_type_copyable(&family.namespace, &family.name)
                },
                |capability| capability.copyable,
            ),
            _ => false,
        }
    }
}

fn native_rust_type<'a>(
    projection: &'a crate::rust_interop::projection::Projection,
    identity: &'a ObjectIdentity,
) -> Option<&'a str> {
    identity.native_projection.as_deref().or_else(|| {
        let item = projection.item(&identity.namespace, &identity.name)?;
        matches!(&item.kind,
            crate::rust_interop::projection::ProjectedKind::Enum { generic_parameters, .. }
                if generic_parameters.is_empty()
        )
        .then_some(item.rust_path.as_str())
    })
}

pub(super) fn populate_native_capabilities(
    package: &mut SemanticPackage,
) -> Result<(), SemanticFailure> {
    let mut native_types = BTreeMap::<String, (usize, Span)>::new();
    let mut scoped_native_types = BTreeSet::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        visit_unit_types(unit, &mut |value_type, span| {
            if let ValueType::Object(identity) = value_type
                && identity.native_projection.is_none()
                && let Some(item) = package.projection.item(&identity.namespace, &identity.name)
                && matches!(&item.kind,
                    crate::rust_interop::projection::ProjectedKind::Enum { generic_parameters, .. }
                        if generic_parameters.is_empty()
                )
                && !native_types.contains_key(&item.rust_path)
            {
                native_types.insert(item.rust_path.clone(), (unit_index, span));
            }
            if let ValueType::InvocationScopedNative {
                rust_type,
                concrete: true,
                ..
            } = value_type
            {
                scoped_native_types.insert(rust_type.clone());
            }
            collect_native_types(value_type, span, unit_index, &mut native_types);
        });
    }
    native_types.retain(|rust_type, _| {
        !package.native_capabilities.contains_key(rust_type)
            && (scoped_native_types.contains(rust_type)
                || !is_borrowed_native_type(&package.projection, rust_type))
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
            let cloneable = native_rust_type(&package.projection, identity)
                .and_then(|rust| package.native_capabilities.get(rust))
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

fn quantified_native_type(rust_type: &str) -> (String, Vec<String>) {
    let bytes = rust_type.as_bytes();
    let mut replacements = BTreeMap::<String, String>::new();
    let mut offset = 0;
    while offset < bytes.len() {
        if bytes[offset] != b'\'' || offset + 1 == bytes.len() {
            let character = rust_type[offset..]
                .chars()
                .next()
                .expect("offset remains within native type");
            offset += character.len_utf8();
            continue;
        }
        let first = bytes[offset + 1];
        if first != b'_' && !first.is_ascii_alphabetic() {
            offset += 1;
            continue;
        }
        let mut end = offset + 2;
        while end < bytes.len() && (bytes[end] == b'_' || bytes[end].is_ascii_alphanumeric()) {
            end += 1;
        }
        let lifetime = &rust_type[offset..end];
        if lifetime != "'static" {
            let index = replacements.len();
            replacements
                .entry(lifetime.to_owned())
                .or_insert_with(|| format!("'__terrane_scoped_{index}"));
        }
        offset = end;
    }
    if replacements.is_empty() {
        return (rust_type.to_owned(), Vec::new());
    }
    let mut quantified = String::with_capacity(rust_type.len());
    offset = 0;
    while offset < bytes.len() {
        if bytes[offset] == b'\'' {
            let end = (offset + 1..bytes.len())
                .find(|&index| !(bytes[index] == b'_' || bytes[index].is_ascii_alphanumeric()))
                .unwrap_or(bytes.len());
            if end > offset + 1 {
                let lifetime = &rust_type[offset..end];
                if let Some(replacement) = replacements.get(lifetime) {
                    quantified.push_str(replacement);
                } else {
                    quantified.push_str(lifetime);
                }
                offset = end;
                continue;
            }
        }
        let character = rust_type[offset..]
            .chars()
            .next()
            .expect("offset remains within native type");
        quantified.push(character);
        offset += character.len_utf8();
    }
    (quantified, replacements.into_values().collect::<Vec<_>>())
}

fn prove_native_capabilities(
    package: &mut SemanticPackage,
    native_types: &BTreeMap<String, (usize, Span)>,
) -> Result<(), SemanticFailure> {
    if native_types.is_empty() {
        return Ok(());
    }
    let mut questions = BTreeMap::<BoundQuestion, BTreeSet<String>>::new();
    for rust_type in native_types.keys() {
        let (proof_type, lifetime_parameters) = quantified_native_type(rust_type);
        for rust_bound in [
            "core::clone::Clone",
            "core::marker::Copy",
            "core::marker::Send",
            "core::marker::Sync",
        ] {
            questions
                .entry(BoundQuestion {
                    rust_type: proof_type.clone(),
                    rust_bound: rust_bound.to_owned(),
                    inferred_parameters: lifetime_parameters.clone(),
                })
                .or_default()
                .insert(rust_type.clone());
        }
    }
    let projected_names = questions
        .keys()
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
    let mut question_types = BTreeMap::<BoundQuestion, BTreeSet<String>>::new();
    let questions = questions
        .into_iter()
        .map(|(mut question, original_types)| {
            question.rust_type =
                super::objects::qualify_projected_rust_names(&question.rust_type, &projected_names);
            question.rust_bound = super::objects::qualify_projected_rust_names(
                &question.rust_bound,
                &projected_names,
            );
            question_types
                .entry(question.clone())
                .or_default()
                .extend(original_types);
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
        let Some(rust_types) = question_types.get(&evidence.question) else {
            continue;
        };
        let satisfied = evidence.answer == ProbeAnswer::Yes;
        for rust_type in rust_types {
            let capability = package
                .native_capabilities
                .entry(rust_type.clone())
                .or_default();
            match evidence.question.rust_bound.as_str() {
                "core::clone::Clone" => capability.cloneable = satisfied,
                "core::marker::Copy" => capability.copyable = satisfied,
                "core::marker::Send" => capability.send = satisfied,
                "core::marker::Sync" => capability.sync = satisfied,
                _ => {}
            }
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
    walk_value_type(value_type, &mut |value_type| match value_type {
        ValueType::Object(identity)
            if identity.application.as_deref().is_none_or(is_closed)
                && identity.type_arguments.iter().all(is_closed)
                && identity.native_arguments.values().all(is_closed) =>
        {
            if let Some(native) = &identity.native_projection {
                types.entry(native.clone()).or_insert((unit_index, span));
            }
        }
        ValueType::InvocationScopedNative {
            rust_type,
            concrete: true,
            ..
        } => {
            types.entry(rust_type.clone()).or_insert((unit_index, span));
        }
        _ => {}
    });
}

fn collect_objects<'a>(value_type: &'a ValueType, objects: &mut Vec<&'a ObjectIdentity>) {
    walk_value_type(value_type, &mut |value_type| {
        if let ValueType::Object(identity) = value_type {
            objects.push(identity);
        }
    });
}

pub(super) fn walk_value_type<'a>(
    value_type: &'a ValueType,
    visit: &mut impl FnMut(&'a ValueType),
) {
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
        ValueType::Union(items) => {
            for item in items {
                walk_value_type(item, visit);
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
        | ValueType::TaskGroup(inner)
        | ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
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
