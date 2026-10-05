use std::collections::BTreeMap;

use super::model::{ObjectIdentity, SemanticPackage, ValueType};
use super::objects::closed_projected_value_type;
use crate::Span;

fn projected_contract<'a>(
    projection: &'a crate::rust_interop::projection::Projection,
    namespace: &str,
    function: &super::model::FunctionContract,
) -> Option<&'a crate::rust_interop::projection::ProjectedFunction> {
    use crate::rust_interop::projection::ProjectedKind;
    if let Some(owner) = &function.owner_identity {
        let ProjectedKind::ForeignType {
            methods,
            static_methods,
            constructor,
            ..
        } = &projection.item(&owner.namespace, &owner.name)?.kind
        else {
            return None;
        };
        methods
            .iter()
            .chain(static_methods)
            .chain(constructor)
            .find(|projected| projected.name == function.name)
    } else {
        let ProjectedKind::Function(projected) = &projection.item(namespace, &function.name)?.kind
        else {
            return None;
        };
        Some(projected)
    }
}

pub(super) fn source_abi_nominal(package: &SemanticPackage, identity: &ObjectIdentity) -> bool {
    if !identity.type_arguments.is_empty()
        || identity.application.is_some()
        || !identity.native_arguments.is_empty()
    {
        return false;
    }
    let Some(item) = package.projection.item(&identity.namespace, &identity.name) else {
        return false;
    };
    let (crate::rust_interop::projection::ProjectedKind::Enum {
        generic_parameters, ..
    }
    | crate::rust_interop::projection::ProjectedKind::ForeignType {
        generic_parameters, ..
    }) = &item.kind
    else {
        return false;
    };
    if !generic_parameters.is_empty() {
        return false;
    }
    let projected = package
        .projection
        .projected_type(&identity.namespace, &identity.name);
    !projected.is_some_and(|projected| {
        package
            .projection
            .canonical_native_alias(&projected)
            .is_some()
    })
}

pub(super) fn canonical_source_nominal(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
) -> ObjectIdentity {
    let mut identity = identity.clone();
    if let Some(mut projected) = package
        .projection
        .projected_type(&identity.namespace, &identity.name)
        && let crate::rust_interop::projection::ProjectedType::Foreign {
            rust_path,
            base_rust_path,
            ..
        } = &mut projected
    {
        *rust_path = package
            .projection
            .canonical_native_type(rust_path)
            .into_owned();
        *base_rust_path = package
            .projection
            .canonical_native_type(base_rust_path)
            .into_owned();
        if let Some((namespace, name)) = package.projection.owner_for_projected_type(&projected) {
            identity.namespace = namespace;
            identity.name = name;
        }
    }
    identity.native_projection = None;
    identity
}

/// Concrete signature aliases retain the ordinary projected class identity and its payload contract.
#[expect(
    clippy::too_many_lines,
    reason = "Contract normalization is one ordered semantic pass; splitting it would obscure its shared selection and async metadata state."
)]
pub(super) fn normalize_contracts(
    package: &mut SemanticPackage,
) -> Result<(), super::model::SemanticFailure> {
    fn annotations<'a>(
        node: &'a crate::syntax::SyntaxNode,
        initializer: Option<&'a crate::syntax::SyntaxNode>,
        found: &mut Vec<(
            &'a crate::syntax::SyntaxNode,
            Option<&'a crate::syntax::SyntaxNode>,
        )>,
    ) {
        use crate::syntax::SyntaxKind;
        if node.kind == SyntaxKind::TypeExpression {
            found.push((node, initializer));
        } else {
            let initializer = matches!(node.kind, SyntaxKind::Binding | SyntaxKind::Assignment)
                .then(|| super::ownership::binding_initializer(node))
                .flatten();
            // The right operand is a descriptor query, not a value-type annotation.
            for (index, child) in node.children.iter().enumerate() {
                if node.kind == SyntaxKind::TypeMembershipExpression && index == 1 {
                    continue;
                }
                annotations(child, initializer, found);
            }
        }
    }
    let mut selections = package
        .units
        .iter()
        .flat_map(|unit| &unit.descriptors)
        .filter_map(|descriptor| {
            let identity = &descriptor.identity;
            if source_abi_nominal(package, identity) {
                return None;
            }
            let projected = package
                .projection
                .projected_type(&identity.namespace, &identity.name)?;
            let (namespace, name) = package.projection.owner_for_projected_type(&projected)?;
            if package
                .projection
                .projected_constructor(&namespace, &name)
                .is_none()
                && package
                    .projection
                    .canonical_native_alias(&projected)
                    .is_none()
            {
                return None;
            }
            let ValueType::Object(selected) = closed_projected_value_type(package, &projected)?
            else {
                return None;
            };
            Some((identity.clone(), selected))
        })
        .collect::<BTreeMap<_, _>>();
    let mut sites = Vec::new();
    let mut written_types = Vec::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        // Generated native signatures are closed from projection metadata below,
        // not from their intentionally erased source declaration spelling.
        if unit.role == crate::SourceRole::Bundled {
            continue;
        }
        let mut nodes = Vec::new();
        annotations(&unit.tree.root, None, &mut nodes);
        for (node, initializer) in nodes {
            let aliases = super::prelude::visible_descriptor_aliases(
                &unit.descriptor_aliases,
                unit.source.id(),
                node.span.start,
            );
            let value_type = super::types::declared_value_type(unit, node, &aliases)?;
            // A bare constructor-family destination leaves representation selection
            // to the payload; explicitly applied destinations are validated here.
            if let ValueType::Object(identity) = &value_type
                && identity.type_arguments.is_empty()
                && identity.application.is_none()
                && initializer.is_some_and(|initializer| {
                    initializer.kind == crate::syntax::SyntaxKind::CallExpression
                        && initializer.children.first().is_some_and(|callee| {
                            callee.kind == crate::syntax::SyntaxKind::ConstructionExpression
                        })
                })
            {
                continue;
            }
            collect_written_native_selections(
                package,
                &value_type,
                unit_index,
                node.span,
                &mut selections,
                &mut sites,
            )?;
            written_types.push((unit_index, node.span, value_type));
        }
        for descriptor in unit
            .descriptors
            .iter()
            .filter(|descriptor| descriptor.span.file == unit.source.id())
        {
            for field in &descriptor.fields {
                collect_written_native_selections(
                    package,
                    &field.value_type,
                    unit_index,
                    field.span,
                    &mut selections,
                    &mut sites,
                )?;
            }
        }
        for function in unit
            .functions
            .iter()
            .filter(|function| function.span.file == unit.source.id())
        {
            for parameter in &function.parameters {
                if let Some(value_type) = &parameter.value_type {
                    collect_written_native_selections(
                        package,
                        value_type,
                        unit_index,
                        parameter.span,
                        &mut selections,
                        &mut sites,
                    )?;
                }
            }
            if let Some(value_type) = &function.return_type {
                collect_written_native_selections(
                    package,
                    value_type,
                    unit_index,
                    function.span,
                    &mut selections,
                    &mut sites,
                )?;
            }
        }
    }
    for unit in &package.units {
        for descriptor in &unit.descriptors {
            collect_inferred_native_selections(
                package,
                &ValueType::Object(descriptor.identity.clone()),
                &mut selections,
            );
        }
    }
    for unit in &package.units {
        for value_type in unit.selected_expression_types.values() {
            collect_inferred_native_selections(package, value_type, &mut selections);
        }
    }

    validate_native_nominal_bounds(package, &selections, &sites)?;
    for (unit_index, span, mut value_type) in written_types {
        normalize_type(&mut value_type, &selections);
        package.units[unit_index]
            .selected_expression_types
            .insert((span.file, span.start, span.end), value_type);
    }
    let mut signatures = Vec::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        for (function_index, function) in unit.functions.iter().enumerate() {
            let projected = projected_contract(&package.projection, &unit.namespace, function);
            let Some(projected) = projected else {
                continue;
            };
            for (parameter_index, (parameter, native)) in function
                .parameters
                .iter()
                .zip(&projected.parameters)
                .enumerate()
            {
                if let Some(current) = &parameter.value_type
                    && let Some(mut selected) = selected_signature_type(
                        package,
                        current,
                        &native.ty,
                        native
                            .associated_type
                            .as_ref()
                            .map(|associated| associated.ty.as_ref()),
                    )
                {
                    normalize_type(&mut selected, &selections);
                    signatures.push((unit_index, function_index, Some(parameter_index), selected));
                }
            }
            if let Some(current) = &function.return_type
                && let Some(mut selected) =
                    selected_signature_type(package, current, &projected.result, None)
            {
                normalize_type(&mut selected, &selections);
                signatures.push((unit_index, function_index, None, selected));
            }
        }
    }
    for unit in &mut package.units {
        for descriptor in &mut unit.descriptors {
            if let Some(selected) = selections.get(&descriptor.identity) {
                descriptor.identity.clone_from(selected);
            }
            for field in &mut descriptor.fields {
                normalize_type(&mut field.value_type, &selections);
            }
        }
        for function in &mut unit.functions {
            if let Some(owner_identity) = &mut function.owner_identity
                && let Some(selected) = selections.get(owner_identity)
            {
                owner_identity.clone_from(selected);
            }
            for parameter in &mut function.parameters {
                if let Some(value_type) = &mut parameter.value_type {
                    normalize_type(value_type, &selections);
                }
            }
            if let Some(value_type) = &mut function.return_type {
                normalize_type(value_type, &selections);
            }
        }
        for value_type in unit.selected_expression_types.values_mut() {
            normalize_type(value_type, &selections);
        }
        for binding in &mut unit.typed_bindings {
            normalize_type(&mut binding.value_type, &selections);
        }
    }
    for (unit_index, function_index, parameter_index, selected) in signatures {
        let function = &mut package.units[unit_index].functions[function_index];
        if let Some(parameter_index) = parameter_index {
            function.parameters[parameter_index].value_type = Some(selected);
        } else {
            function.return_type = Some(selected);
        }
    }
    super::analysis::populate_function_aliases(package);
    super::analysis::populate_object_aliases(package);
    populate_applied_native_signatures(package)?;
    super::capabilities::populate_native_capabilities(package)?;
    Ok(())
}

fn populate_applied_native_signatures(
    package: &mut SemanticPackage,
) -> Result<(), super::model::SemanticFailure> {
    use crate::syntax::{SyntaxKind, SyntaxNode};

    fn applications<'a>(node: &'a SyntaxNode, found: &mut Vec<&'a SyntaxNode>) {
        if node.kind == SyntaxKind::AppliedType {
            found.push(node);
        }
        for child in &node.children {
            applications(child, found);
        }
    }

    let mut candidates = BTreeMap::<&str, Vec<_>>::new();
    for unit in &package.units {
        for contract in &unit.functions {
            if let Some(native) = projected_contract(&package.projection, &unit.namespace, contract)
            {
                candidates
                    .entry(&contract.name)
                    .or_default()
                    .push((contract, native));
            }
        }
    }
    let mut selections = Vec::new();
    for (unit_index, unit) in package.units.iter().enumerate() {
        if unit.role == crate::SourceRole::Bundled {
            continue;
        }
        let mut nodes = Vec::new();
        applications(&unit.tree.root, &mut nodes);
        for node in nodes {
            let Some(mut base) = node.children.first() else {
                continue;
            };
            while matches!(
                base.kind,
                SyntaxKind::GroupExpression | SyntaxKind::TypeExpression
            ) && let Some(inner) = base.children.first()
            {
                base = inner;
            }
            let name_node = match base.kind {
                SyntaxKind::Name => base,
                SyntaxKind::MemberExpression | SyntaxKind::StaticMemberExpression => {
                    let Some(member) = base.children.last() else {
                        continue;
                    };
                    member
                }
                _ => continue,
            };
            let name = &unit.source.text()[name_node.span.start..name_node.span.end];
            let Some(contracts) = candidates.get(name) else {
                continue;
            };
            let resolved = super::namespaces::function_contract_for_call_with_safety(
                package,
                unit,
                node,
                crate::syntax::call_is_unsafe(node),
            );
            let owner = super::objects::projected_call_owner_substitutions(package, unit, node)
                .map_err(|reason| {
                    super::diagnostics::failure(&unit.source, "T0118", reason, node.span)
                })?;
            for &(contract, native) in contracts {
                if resolved.is_some_and(|resolved| resolved.span != contract.span) {
                    continue;
                }
                if let Some(selected) =
                    applied_native_signature(package, unit, node, contract, native, &owner)?
                {
                    selections.push((unit_index, node.span, contract.span, selected));
                }
            }
        }
    }
    for (unit, application, contract, selected) in selections {
        package.units[unit]
            .projected_callable_applications
            .entry((application.file, application.start, application.end))
            .or_default()
            .insert((contract.file, contract.start, contract.end), selected);
    }
    Ok(())
}

fn applied_native_signature(
    package: &SemanticPackage,
    unit: &super::model::SemanticUnit,
    application: &crate::syntax::SyntaxNode,
    contract: &super::model::FunctionContract,
    native: &crate::rust_interop::projection::ProjectedFunction,
    owner: &BTreeMap<String, crate::rust_interop::projection::ProjectedType>,
) -> Result<Option<ValueType>, super::model::SemanticFailure> {
    use super::model::{CallableEffects, ElementType};
    use super::objects::{destination_projected_type, substitute_projected_generic};

    let binders = native
        .generic_parameters
        .iter()
        .filter(|parameter| {
            parameter.input_selected
                || native.destination_result.as_ref().is_some_and(|result| {
                    result
                        .parameters
                        .iter()
                        .any(|binder| binder.name == parameter.name)
                })
        })
        .collect::<Vec<_>>();
    let arguments = &application.children[1..];
    if binders.len() != arguments.len() {
        return Ok(None);
    }
    let mut substitutions = owner.clone();
    for (binder, argument) in binders.into_iter().zip(arguments) {
        let value = super::types::declared_value_type(
            unit,
            argument,
            &super::prelude::visible_descriptor_aliases(
                &unit.descriptor_aliases,
                argument.span.file,
                argument.span.start,
            ),
        )?;
        let projected = destination_projected_type(package, &value).map_err(|reason| {
            super::diagnostics::failure(&unit.source, "T0129", reason, argument.span)
        })?;
        substitutions.insert(binder.name.clone(), projected);
    }
    let specialize = |template: &crate::rust_interop::projection::ProjectedType| {
        substitutions
            .iter()
            .fold(template.clone(), |ty, (name, selected)| {
                substitute_projected_generic(&ty, name, selected)
            })
    };
    let result = specialize(&native.result);
    if result.contains_open_generic() {
        return Ok(None);
    }
    let Some(result) = closed_projected_value_type(package, &result) else {
        return Ok(None);
    };
    let parameters = contract
        .parameters
        .iter()
        .zip(&native.parameters)
        .map(|(parameter, native)| {
            let current = parameter.element_value_type()?;
            let associated = native
                .associated_type
                .as_ref()
                .map(|associated| specialize(&associated.ty));
            let selected = selected_signature_type(
                package,
                &current,
                &specialize(&native.ty),
                associated.as_ref(),
            )?;
            parameter
                .callable_type()
                .map(|parameter| parameter.with_element_type(ElementType::new(selected)))
        })
        .collect::<Option<Vec<_>>>();
    let Some(parameters) = parameters else {
        return Ok(None);
    };
    let result = ElementType::new(result);
    let effects = CallableEffects::from_contract(contract);
    Ok(Some(if native.is_async {
        ValueType::AsyncFunction(parameters, result, contract.task_transferability, effects)
    } else {
        ValueType::Function(parameters, result, effects)
    }))
}

fn selected_signature_type(
    package: &SemanticPackage,
    current: &ValueType,
    projected: &crate::rust_interop::projection::ProjectedType,
    associated: Option<&crate::rust_interop::projection::ProjectedType>,
) -> Option<ValueType> {
    use crate::rust_interop::projection::{ProjectedKind, ProjectedType};
    if let ValueType::Reference(inner) | ValueType::SharedReference(inner) = current {
        let selected =
            selected_signature_type(package, inner.value_type_ref(), projected, associated)?;
        let element = super::model::ElementType::new(selected);
        return Some(if matches!(current, ValueType::Reference(_)) {
            ValueType::Reference(element)
        } else {
            ValueType::SharedReference(element)
        });
    }
    if let (ValueType::Object(identity), Some(associated)) = (current, associated)
        && package
            .projection
            .item(&identity.namespace, &identity.name)
            .is_some_and(|item| matches!(item.kind, ProjectedKind::Interface(_)))
    {
        return Some(ValueType::Object(identity.clone().with_application(
            closed_projected_value_type(package, associated)?,
        )));
    }
    if matches!(current, ValueType::InvocationScopedNative { .. }) {
        // Scoped families retain their lifetime graph rather than becoming owned nominal values.
        return None;
    }
    if let ProjectedType::Foreign { .. } = projected
        && let Some(item) = package
            .projection
            .owner_for_projected_type(projected)
            .and_then(|(namespace, name)| package.projection.item(&namespace, &name))
        && matches!(
            item.kind,
            ProjectedKind::ForeignType {
                borrowed_view: true,
                ..
            }
        )
    {
        return None;
    }
    if matches!(
        projected,
        ProjectedType::BoxedInterface {
            associated_type: None,
            ..
        }
    ) && associated.is_none()
        && matches!(current, ValueType::Object(identity) if identity.application.is_some())
    {
        // Missing native slot metadata does not erase a written associated application.
        return Some(current.clone());
    }
    if let ProjectedType::BoxedInterface { .. } = projected
        && let Some(normalized) = closed_projected_value_type(package, projected)
    {
        return match (normalized, associated) {
            (ValueType::Object(identity), Some(associated)) => Some(ValueType::Object(
                identity.with_application(closed_projected_value_type(package, associated)?),
            )),
            (normalized, _) => Some(normalized),
        };
    }
    if let ValueType::Object(identity) = current
        && !contains_open_value_type(current)
        && source_abi_nominal(package, identity)
    {
        return Some(ValueType::Object(canonical_source_nominal(
            package, identity,
        )));
    }
    if let ValueType::Object(identity) = current
        && !contains_open_value_type(current)
        && package
            .projection
            .item(&identity.namespace, &identity.name)
            .is_some_and(|item| matches!(item.kind, ProjectedKind::Interface(_)))
    {
        // Preserve authored nominal contracts; projected interface applications are normalized above.
        return Some(current.clone());
    }
    closed_projected_value_type(package, projected)
}

#[expect(
    clippy::too_many_lines,
    reason = "This recursive traversal handles every nested value-type shape while preserving one diagnostic site and selection context."
)]
fn collect_written_native_selections(
    package: &SemanticPackage,
    value_type: &ValueType,
    unit_index: usize,
    span: Span,
    selections: &mut BTreeMap<ObjectIdentity, ObjectIdentity>,
    sites: &mut Vec<(ObjectIdentity, usize, Span)>,
) -> Result<(), super::model::SemanticFailure> {
    match value_type {
        ValueType::Object(identity) => {
            for argument in identity
                .type_arguments
                .iter()
                .chain(identity.application.iter().map(std::convert::AsRef::as_ref))
                .chain(identity.native_arguments.values())
            {
                collect_written_native_selections(
                    package, argument, unit_index, span, selections, sites,
                )?;
            }
            if package
                .projection
                .item(&identity.namespace, &identity.name)
                .is_none()
            {
                return Ok(());
            }
            if let Some(problem) = native_application_problem(package, identity) {
                return Err(super::diagnostics::failure(
                    &package.units[unit_index].source,
                    "T0117",
                    problem,
                    span,
                ));
            }
            sites.push((identity.clone(), unit_index, span));
            if let Some(selected) = super::objects::close_written_native_nominal(package, identity)
            {
                selections.insert(identity.clone(), selected);
            }
        }
        ValueType::Optional(inner) => {
            collect_written_native_selections(package, inner, unit_index, span, selections, sites)?;
        }
        ValueType::Reference(inner)
        | ValueType::SharedReference(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner) => {
            collect_written_native_selections(
                package,
                inner.value_type_ref(),
                unit_index,
                span,
                selections,
                sites,
            )?;
        }
        ValueType::Map(key, value)
        | ValueType::UnorderedMap(key, value)
        | ValueType::Entry(key, value) => {
            collect_written_native_selections(
                package,
                key.value_type_ref(),
                unit_index,
                span,
                selections,
                sites,
            )?;
            collect_written_native_selections(
                package,
                value.value_type_ref(),
                unit_index,
                span,
                selections,
                sites,
            )?;
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                collect_written_native_selections(
                    package,
                    parameter.value_type_ref(),
                    unit_index,
                    span,
                    selections,
                    sites,
                )?;
            }
            collect_written_native_selections(
                package,
                result.value_type_ref(),
                unit_index,
                span,
                selections,
                sites,
            )?;
        }
        _ => {}
    }
    Ok(())
}
fn collect_inferred_native_selections(
    package: &SemanticPackage,
    value_type: &ValueType,
    selections: &mut BTreeMap<ObjectIdentity, ObjectIdentity>,
) {
    match value_type {
        ValueType::Object(identity) => {
            for argument in identity
                .type_arguments
                .iter()
                .chain(identity.application.iter().map(std::convert::AsRef::as_ref))
                .chain(identity.native_arguments.values())
            {
                collect_inferred_native_selections(package, argument, selections);
            }
            if identity.native_projection.is_none()
                && !source_abi_nominal(package, identity)
                && !selections.contains_key(identity)
                && let Some(selected) =
                    super::objects::close_written_native_nominal(package, identity)
            {
                selections.insert(identity.clone(), selected);
            }
        }
        ValueType::Optional(inner) => {
            collect_inferred_native_selections(package, inner, selections);
        }
        ValueType::Reference(inner)
        | ValueType::SharedReference(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner) => {
            collect_inferred_native_selections(package, inner.value_type_ref(), selections);
        }
        ValueType::Map(key, value)
        | ValueType::UnorderedMap(key, value)
        | ValueType::Entry(key, value) => {
            collect_inferred_native_selections(package, key.value_type_ref(), selections);
            collect_inferred_native_selections(package, value.value_type_ref(), selections);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                collect_inferred_native_selections(package, parameter.value_type_ref(), selections);
            }
            collect_inferred_native_selections(package, result.value_type_ref(), selections);
        }
        _ => {}
    }
}

fn native_application_problem(
    package: &SemanticPackage,
    identity: &ObjectIdentity,
) -> Option<String> {
    let item = package
        .projection
        .item(&identity.namespace, &identity.name)?;
    let (crate::rust_interop::projection::ProjectedKind::ForeignType {
        generic_parameters: parameters,
        ..
    }
    | crate::rust_interop::projection::ProjectedKind::Enum {
        generic_parameters: parameters,
        ..
    }) = &item.kind
    else {
        return None;
    };
    let actual = if identity.type_arguments.is_empty() {
        usize::from(identity.application.is_some())
    } else {
        identity.type_arguments.len()
    };
    if actual > parameters.len() {
        return Some(format!(
            "native type `{}` accepts at most {} type arguments, found {actual}",
            identity.name,
            parameters.len()
        ));
    }
    if let Some(missing) = parameters
        .iter()
        .skip(actual)
        .find(|parameter| parameter.default.is_none())
    {
        return Some(format!(
            "native type `{}` requires type argument `{}`",
            identity.name, missing.name
        ));
    }
    None
}

#[expect(
    clippy::too_many_lines,
    reason = "Nominal-bound validation builds and discharges one proof set so evidence and source sites remain aligned."
)]
fn validate_native_nominal_bounds(
    package: &mut SemanticPackage,
    selections: &BTreeMap<ObjectIdentity, ObjectIdentity>,
    sites: &[(ObjectIdentity, usize, Span)],
) -> Result<(), super::model::SemanticFailure> {
    let mut questions = BTreeMap::new();
    for (identity, unit_index, span) in sites {
        if package
            .projection
            .item(&identity.namespace, &identity.name)
            .is_some_and(|item| {
                !matches!(
                    item.kind,
                    crate::rust_interop::projection::ProjectedKind::ForeignType { .. }
                        | crate::rust_interop::projection::ProjectedKind::Enum { .. }
                )
            })
        {
            continue;
        }
        let Some(selected) = selections.get(identity) else {
            if (!identity.type_arguments.is_empty() || identity.application.is_some())
                && !contains_open_type_argument(identity)
            {
                return Err(super::diagnostics::failure(
                    &package.units[*unit_index].source,
                    "T0117",
                    format!(
                        "native type application `{}` has no closed conversion",
                        identity.name
                    ),
                    *span,
                ));
            }
            continue;
        };
        let Some(item) = package.projection.item(&selected.namespace, &selected.name) else {
            continue;
        };
        let (crate::rust_interop::projection::ProjectedKind::ForeignType {
            generic_parameters,
            ..
        }
        | crate::rust_interop::projection::ProjectedKind::Enum {
            generic_parameters, ..
        }) = &item.kind
        else {
            continue;
        };
        let rust_arguments = generic_parameters
            .iter()
            .filter_map(|parameter| {
                let argument = selected.native_arguments.get(&parameter.name)?;
                Some((
                    parameter.name.clone(),
                    super::objects::destination_projected_type(package, argument)
                        .ok()?
                        .rust_type(),
                ))
            })
            .collect::<BTreeMap<_, _>>();
        for parameter in generic_parameters {
            let Some(rust_type) = rust_arguments.get(&parameter.name) else {
                continue;
            };
            for rust_bound in &parameter.rust_bounds {
                let question = crate::rust_interop::BoundQuestion {
                    rust_type: rust_type.clone(),
                    rust_bound: crate::rust_ir::instantiate_rust_generics(
                        rust_bound,
                        &rust_arguments,
                    ),
                    inferred_parameters: Vec::new(),
                };
                questions.entry(question).or_insert((*unit_index, *span));
            }
        }
    }
    if questions.is_empty() {
        return Ok(());
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
    let questions = questions
        .into_iter()
        .map(|(mut question, site)| {
            question.rust_type =
                super::objects::qualify_projected_rust_names(&question.rust_type, &projected_names);
            question.rust_bound = super::objects::qualify_projected_rust_names(
                &question.rust_bound,
                &projected_names,
            );
            (question, site)
        })
        .collect::<BTreeMap<_, _>>();
    let workspace = package.root.join(".trn/dependencies");
    let report = crate::rust_interop::ProjectionOracle::new(
        &workspace,
        &package.projection.cache_identity,
        package.projection.containment,
    )
    .prove_bounds(&questions.keys().cloned().collect::<Vec<_>>())
    .map_err(|error| {
        let (unit_index, span) = *questions.values().next().expect("questions are nonempty");
        super::diagnostics::failure(
            &package.units[unit_index].source,
            "T0119",
            format!(
                "native nominal bounds could not be validated: {}",
                error.message
            ),
            span,
        )
    })?;
    for evidence in &report.evidence {
        if evidence.answer != crate::rust_interop::ProbeAnswer::Yes {
            let (unit_index, span) = questions[&evidence.question];
            return Err(super::diagnostics::failure(
                &package.units[unit_index].source,
                "T0119",
                format!(
                    "native type argument `{}` does not satisfy `{}`",
                    evidence.question.rust_type, evidence.question.rust_bound
                ),
                span,
            ));
        }
    }
    package.projection.probes.extend(report.evidence);
    package.projection.probe_wall_time_ms = package
        .projection
        .probe_wall_time_ms
        .saturating_add(report.wall_time_ms);
    Ok(())
}

fn contains_open_type_argument(identity: &ObjectIdentity) -> bool {
    identity.type_arguments.iter().any(contains_open_value_type)
        || identity
            .application
            .as_deref()
            .is_some_and(contains_open_value_type)
}

fn contains_open_value_type(value_type: &ValueType) -> bool {
    match value_type {
        ValueType::TypeParameter(_) | ValueType::ProjectedGeneric(_) => true,
        ValueType::Object(identity) => contains_open_type_argument(identity),
        ValueType::Optional(inner) => contains_open_value_type(inner),
        ValueType::Reference(inner)
        | ValueType::SharedReference(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner) => {
            contains_open_value_type(inner.value_type_ref())
        }
        ValueType::Map(key, value)
        | ValueType::UnorderedMap(key, value)
        | ValueType::Entry(key, value) => {
            contains_open_value_type(key.value_type_ref())
                || contains_open_value_type(value.value_type_ref())
        }
        _ => false,
    }
}

fn normalize_type(
    value_type: &mut ValueType,
    selections: &BTreeMap<ObjectIdentity, ObjectIdentity>,
) {
    match value_type {
        ValueType::Object(identity) => {
            if let Some(selected) = selections.get(identity) {
                identity.clone_from(selected);
            }
        }
        ValueType::Optional(inner) => normalize_type(inner, selections),
        ValueType::Reference(inner)
        | ValueType::SharedReference(inner)
        | ValueType::List(inner)
        | ValueType::Set(inner)
        | ValueType::UnorderedSet(inner)
        | ValueType::Iterator(inner)
        | ValueType::IterationStep(inner)
        | ValueType::AsyncIterationStep(inner)
        | ValueType::Tuple(inner, _)
        | ValueType::Task(inner, _)
        | ValueType::ScopedTask(inner, _)
        | ValueType::TaskOutcome(inner)
        | ValueType::ChannelPair(inner)
        | ValueType::ChannelSender(inner)
        | ValueType::ChannelReceiver(inner)
        | ValueType::ChannelReceiveOutcome(inner)
        | ValueType::ChannelSendOutcome(inner)
        | ValueType::DocumentDecodeOutcome(inner) => {
            normalize_type(inner.value_type_mut(), selections);
        }
        ValueType::Map(key, value)
        | ValueType::UnorderedMap(key, value)
        | ValueType::Entry(key, value) => {
            normalize_type(key.value_type_mut(), selections);
            normalize_type(value.value_type_mut(), selections);
        }
        ValueType::Function(parameters, result, _)
        | ValueType::AsyncFunction(parameters, result, _, _) => {
            for parameter in parameters {
                normalize_type(parameter.value_type_mut(), selections);
            }
            normalize_type(result.value_type_mut(), selections);
        }
        _ => {}
    }
}
