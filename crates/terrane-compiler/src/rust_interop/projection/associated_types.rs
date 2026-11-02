//! Input-selected associated equations and exact native equality obligations.

use super::instantiate_projected_container;
use std::collections::BTreeMap;

use super::{
    AssociatedNormalizationError, ProjectedAssociatedEqualityObligation,
    ProjectedAssociatedImplementation, ProjectedFunction, ProjectedType, Projection,
};

/// Resolves an operation result's associated outputs using retained implementation recipes.
///
/// # Errors
///
/// Returns [`AssociatedNormalizationError::Unknown`] when no implementation matches,
/// [`AssociatedNormalizationError::Ambiguous`] when multiple results match, or
/// [`AssociatedNormalizationError::Unsupported`] for unresolved substitutions, cycles,
/// or unsupported associated references.
pub fn normalize_associated_result<'a>(
    operation: &ProjectedFunction,
    substitutions: &BTreeMap<String, ProjectedType>,
    implementations: impl Iterator<Item = &'a ProjectedAssociatedImplementation> + Clone,
    oracle: Option<&crate::rust_interop::ProjectionOracle<'_>>,
) -> Result<ProjectedType, AssociatedNormalizationError> {
    let mut result = operation.result.clone();
    normalize(&mut result, substitutions, implementations, None, oracle)?;
    Ok(result)
}
fn unify(
    pattern: &ProjectedType,
    concrete: &ProjectedType,
    bindings: &mut BTreeMap<String, ProjectedType>,
) -> bool {
    match pattern {
        ProjectedType::Generic(name) => {
            bindings
                .entry(name.clone())
                .or_insert_with(|| concrete.clone())
                == concrete
        }
        ProjectedType::Foreign {
            base_rust_path: pattern_path,
            arguments: pattern_args,
            ..
        } => {
            matches!(concrete, ProjectedType::Foreign { base_rust_path, arguments, .. }
                    if pattern_path == base_rust_path
                        && pattern_args.len() == arguments.len()
                        && pattern_args.iter().zip(arguments).all(|(pattern, concrete)| unify(pattern, concrete, bindings)))
        }
        ProjectedType::Reference {
            inner: pattern_inner,
            mutable: pattern_mutable,
            lifetime: pattern_lifetime,
        } => {
            matches!(concrete, ProjectedType::Reference { inner, mutable, lifetime }
                    if pattern_mutable == mutable
                        && (pattern_lifetime.is_none() || pattern_lifetime == lifetime)
                        && unify(pattern_inner, inner, bindings))
        }
        ProjectedType::Optional(pattern_inner) => {
            matches!(concrete, ProjectedType::Optional(inner) if unify(pattern_inner, inner, bindings))
        }
        ProjectedType::AsyncIterationStep(pattern_inner) => {
            matches!(concrete, ProjectedType::AsyncIterationStep(inner) if unify(pattern_inner, inner, bindings))
        }
        ProjectedType::Sequence {
            rust_path: pattern_path,
            item: pattern_item,
        } => {
            matches!(concrete, ProjectedType::Sequence { rust_path, item } if same_constructor(pattern_path, rust_path) && unify(pattern_item, item, bindings))
        }
        ProjectedType::Set {
            rust_path: pattern_path,
            item: pattern_item,
            ordered: pattern_ordered,
        } => {
            matches!(concrete, ProjectedType::Set { rust_path, item, ordered } if same_constructor(pattern_path, rust_path) && pattern_ordered == ordered && unify(pattern_item, item, bindings))
        }
        ProjectedType::Mapping {
            rust_path: pattern_path,
            key: pattern_key,
            value: pattern_value,
            ordered: pattern_ordered,
        } => {
            matches!(concrete, ProjectedType::Mapping { rust_path, key, value, ordered }
                if same_constructor(pattern_path, rust_path)
                    && pattern_ordered == ordered
                    && unify(pattern_key, key, bindings)
                    && unify(pattern_value, value, bindings))
        }
        ProjectedType::Tuple(pattern_items) => {
            matches!(concrete, ProjectedType::Tuple(items) if pattern_items.len() == items.len() && pattern_items.iter().zip(items).all(|(pattern, concrete)| unify(pattern, concrete, bindings)))
        }
        _ => pattern == concrete,
    }
}
fn same_constructor(pattern: &str, concrete: &str) -> bool {
    fn identity(path: &str) -> &str {
        let path = path.split('<').next().unwrap_or(path);
        match path {
            "Vec" | "std::vec::Vec" | "alloc::vec::Vec" => "alloc::vec::Vec",
            "std::collections::BTreeSet"
            | "alloc::collections::btree::set::BTreeSet"
            | "alloc::collections::BTreeSet" => "alloc::collections::BTreeSet",
            "std::collections::HashSet" | "std::collections::hash::set::HashSet" => {
                "std::collections::HashSet"
            }
            "std::collections::BTreeMap"
            | "alloc::collections::btree::map::BTreeMap"
            | "alloc::collections::BTreeMap" => "alloc::collections::BTreeMap",
            "std::collections::HashMap" | "std::collections::hash::map::HashMap" => {
                "std::collections::HashMap"
            }
            _ => path,
        }
    }
    identity(pattern) == identity(concrete)
}

struct Frame<'a> {
    owner: &'a ProjectedType,
    trait_path: &'a str,
    member: &'a str,
    arguments: &'a [ProjectedType],
    parent: Option<&'a Frame<'a>>,
}

fn associated_owner(
    reference: &super::ProjectedAssociatedTypeRef,
    substitutions: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, AssociatedNormalizationError> {
    reference
        .owner_type
        .as_deref()
        .cloned()
        .or_else(|| {
            substitutions.get(&reference.owner).cloned().or_else(|| {
                reference
                    .owner
                    .contains("::")
                    .then(|| ProjectedType::Foreign {
                        rust_path: reference.owner.clone(),
                        name: reference
                            .owner
                            .rsplit("::")
                            .next()
                            .unwrap_or(&reference.owner)
                            .to_owned(),
                        base_rust_path: reference.owner.clone(),
                        arguments: reference.owner_arguments.clone(),
                    })
            })
        })
        .ok_or(AssociatedNormalizationError::Unknown)
}

fn resolve_associated<'a>(
    reference: &super::ProjectedAssociatedTypeRef,
    substitutions: &BTreeMap<String, ProjectedType>,
    implementations: &(impl Iterator<Item = &'a ProjectedAssociatedImplementation> + Clone),
    active: Option<&Frame<'_>>,
    oracle: Option<&crate::rust_interop::ProjectionOracle<'_>>,
) -> Result<ProjectedType, AssociatedNormalizationError> {
    let mut owner = associated_owner(reference, substitutions)?;
    substitute_type(&mut owner, substitutions)?;
    let trait_path = reference
        .trait_rust_path
        .as_deref()
        .ok_or(AssociatedNormalizationError::Unsupported)?;
    let query = Frame {
        owner: &owner,
        trait_path,
        member: &reference.member,
        arguments: &reference.trait_arguments,
        parent: active,
    };
    let mut ancestor = active;
    while let Some(previous) = ancestor {
        if previous.owner == query.owner
            && previous.trait_path == query.trait_path
            && previous.member == query.member
            && previous.arguments == query.arguments
        {
            return Err(AssociatedNormalizationError::Unsupported);
        }
        ancestor = previous.parent;
    }
    let mut normalized = None;
    let mut candidate_error = None;
    for implementation in implementations.clone() {
        if implementation.trait_rust_path != trait_path
            || implementation.member != reference.member
            || implementation.trait_arguments.len() != reference.trait_arguments.len()
        {
            continue;
        }
        let mut bindings = BTreeMap::new();
        if !unify(&implementation.owner_pattern, &owner, &mut bindings) {
            continue;
        }
        let mut matches = true;
        for (pattern, concrete) in implementation
            .trait_arguments
            .iter()
            .zip(&reference.trait_arguments)
        {
            let mut concrete = concrete.clone();
            if let Err(error) = substitute_type(&mut concrete, substitutions) {
                candidate_error.get_or_insert(error);
                matches = false;
                break;
            }
            if !unify(pattern, &concrete, &mut bindings) {
                matches = false;
                break;
            }
        }
        if !matches {
            continue;
        }
        let mut result_bindings = substitutions.clone();
        result_bindings.extend(bindings);
        result_bindings.insert("Self".to_owned(), owner.clone());
        let mut result = implementation.ty.clone();
        if let Err(error) = substitute_type(&mut result, &result_bindings) {
            candidate_error.get_or_insert(error);
            continue;
        }
        if implementation.requires_native_selection
            && !candidate_is_selected(oracle, &owner, reference, &result, &result_bindings)?
        {
            continue;
        }
        if let Err(error) = normalize(
            &mut result,
            &result_bindings,
            implementations.clone(),
            Some(&query),
            oracle,
        ) {
            candidate_error.get_or_insert(error);
            continue;
        }
        match &normalized {
            Some(previous) if previous != &result => {
                return Err(AssociatedNormalizationError::Ambiguous);
            }
            Some(_) => {}
            None => normalized = Some(result),
        }
    }
    match (normalized, candidate_error) {
        (Some(result), None) => Ok(result),
        (_, Some(error)) => Err(error),
        (None, None) => Err(AssociatedNormalizationError::Unknown),
    }
}
fn normalize<'b>(
    ty: &mut ProjectedType,
    substitutions: &BTreeMap<String, ProjectedType>,
    implementations: impl Iterator<Item = &'b ProjectedAssociatedImplementation> + Clone,
    active: Option<&Frame<'_>>,
    oracle: Option<&crate::rust_interop::ProjectionOracle<'_>>,
) -> Result<(), AssociatedNormalizationError> {
    substitute_type(ty, substitutions)?;
    match ty {
        ProjectedType::Associated(reference) => {
            *ty = resolve_associated(reference, substitutions, &implementations, active, oracle)?;
        }
        ProjectedType::Sequence { item, .. }
        | ProjectedType::Set { item, .. }
        | ProjectedType::Optional(item)
        | ProjectedType::AsyncIterationStep(item)
        | ProjectedType::Reference { inner: item, .. } => {
            normalize(item, substitutions, implementations, active, oracle)?;
        }
        ProjectedType::Mapping { key, value, .. } => {
            normalize(key, substitutions, implementations.clone(), active, oracle)?;
            normalize(value, substitutions, implementations, active, oracle)?;
        }
        ProjectedType::Tuple(items)
        | ProjectedType::Foreign {
            arguments: items, ..
        } => {
            for item in items {
                normalize(item, substitutions, implementations.clone(), active, oracle)?;
            }
        }
        ProjectedType::InvocationScoped { owned, .. } => {
            normalize(owned, substitutions, implementations, active, oracle)?;
        }
        ProjectedType::BoxedInterface {
            associated_type: Some(binding),
            ..
        } => normalize(
            &mut binding.ty,
            substitutions,
            implementations,
            active,
            oracle,
        )?,
        ProjectedType::Callback {
            parameters,
            native_substitutions,
            result,
            ..
        } => {
            for parameter in parameters {
                normalize(
                    parameter,
                    substitutions,
                    implementations.clone(),
                    active,
                    oracle,
                )?;
            }
            for value in native_substitutions.values_mut() {
                normalize(
                    value,
                    substitutions,
                    implementations.clone(),
                    active,
                    oracle,
                )?;
            }
            normalize(result, substitutions, implementations, active, oracle)?;
        }
        _ => {}
    }
    refresh_native_spelling(ty);
    Ok(())
}

pub(super) fn associated_equality_obligations(
    projection: &Projection,
    operation: &ProjectedFunction,
    substitutions: &BTreeMap<String, ProjectedType>,
    oracle: Option<&crate::rust_interop::ProjectionOracle<'_>>,
) -> Result<Vec<ProjectedAssociatedEqualityObligation>, AssociatedNormalizationError> {
    let mut obligations = Vec::new();
    visit(
        projection,
        operation,
        &operation.result,
        substitutions,
        &mut obligations,
        oracle,
    )?;
    Ok(obligations)
}
fn visit(
    projection: &Projection,
    operation: &ProjectedFunction,
    ty: &ProjectedType,
    substitutions: &BTreeMap<String, ProjectedType>,
    obligations: &mut Vec<ProjectedAssociatedEqualityObligation>,
    oracle: Option<&crate::rust_interop::ProjectionOracle<'_>>,
) -> Result<(), AssociatedNormalizationError> {
    if let ProjectedType::Associated(reference) = ty {
        let mut owner = associated_owner(reference, substitutions)?;
        substitute_type(&mut owner, substitutions)?;
        let trait_rust_path = reference
            .trait_rust_path
            .clone()
            .ok_or(AssociatedNormalizationError::Unsupported)?;
        let mut trait_arguments = reference.trait_arguments.clone();
        for argument in &mut trait_arguments {
            substitute_type(argument, substitutions)?;
        }
        let mut selected = operation.clone();
        selected.result = ProjectedType::Associated(reference.clone());
        let result = projection.normalize_associated_result(&selected, substitutions, oracle)?;
        let obligation = ProjectedAssociatedEqualityObligation {
            owner,
            owner_parameter: substitutions
                .contains_key(&reference.owner)
                .then(|| reference.owner.clone()),
            trait_rust_path,
            trait_arguments,
            member: reference.member.clone(),
            result,
        };
        if !obligations.contains(&obligation) {
            obligations.push(obligation);
        }
    }
    let mut child = |ty: &ProjectedType| {
        visit(
            projection,
            operation,
            ty,
            substitutions,
            obligations,
            oracle,
        )
    };
    match ty {
        ProjectedType::Associated(reference) => {
            if let Some(owner) = &reference.owner_type {
                child(owner)?;
            }
            for argument in reference
                .owner_arguments
                .iter()
                .chain(&reference.trait_arguments)
            {
                child(argument)?;
            }
        }
        ProjectedType::Sequence { item, .. }
        | ProjectedType::Set { item, .. }
        | ProjectedType::Optional(item)
        | ProjectedType::AsyncIterationStep(item)
        | ProjectedType::Reference { inner: item, .. } => child(item)?,
        ProjectedType::Mapping { key, value, .. } => {
            child(key)?;
            child(value)?;
        }
        ProjectedType::Tuple(items)
        | ProjectedType::Foreign {
            arguments: items, ..
        } => {
            for item in items {
                child(item)?;
            }
        }
        ProjectedType::InvocationScoped { owned, .. } => child(owned)?,
        ProjectedType::BoxedInterface {
            associated_type: Some(binding),
            ..
        } => child(&binding.ty)?,
        ProjectedType::Callback {
            parameters,
            native_substitutions,
            result,
            ..
        } => {
            for parameter in parameters {
                child(parameter)?;
            }
            for value in native_substitutions.values() {
                child(value)?;
            }
            child(result)?;
        }
        _ => {}
    }
    Ok(())
}

fn substitute_type(
    ty: &mut ProjectedType,
    substitutions: &BTreeMap<String, ProjectedType>,
) -> Result<(), AssociatedNormalizationError> {
    struct Frame<'a> {
        parameter: &'a str,
        parent: Option<&'a Frame<'a>>,
    }
    fn apply(
        ty: &mut ProjectedType,
        substitutions: &BTreeMap<String, ProjectedType>,
        active: Option<&Frame<'_>>,
    ) -> Result<(), AssociatedNormalizationError> {
        if let ProjectedType::Generic(name) = ty
            && let Some((parameter, replacement)) = substitutions.get_key_value(name)
        {
            if replacement == ty {
                return Ok(());
            }
            let mut ancestor = active;
            while let Some(previous) = ancestor {
                if previous.parameter == parameter {
                    return Err(AssociatedNormalizationError::Unsupported);
                }
                ancestor = previous.parent;
            }
            let frame = Frame {
                parameter,
                parent: active,
            };
            *ty = replacement.clone();
            return apply(ty, substitutions, Some(&frame));
        }
        match ty {
            ProjectedType::Associated(r) => {
                if r.owner_type.is_none() {
                    r.owner_type = substitutions.get(&r.owner).cloned().map(Box::new);
                }
                if let Some(owner) = &mut r.owner_type {
                    apply(owner, substitutions, active)?;
                }
                for x in r.owner_arguments.iter_mut().chain(&mut r.trait_arguments) {
                    apply(x, substitutions, active)?;
                }
            }
            ProjectedType::Sequence { item, .. }
            | ProjectedType::Set { item, .. }
            | ProjectedType::Optional(item)
            | ProjectedType::AsyncIterationStep(item)
            | ProjectedType::Reference { inner: item, .. }
            | ProjectedType::InvocationScoped { owned: item, .. } => {
                apply(item, substitutions, active)?;
            }
            ProjectedType::Mapping { key, value, .. } => {
                apply(key, substitutions, active)?;
                apply(value, substitutions, active)?;
            }
            ProjectedType::Tuple(items)
            | ProjectedType::Foreign {
                arguments: items, ..
            } => {
                for item in items {
                    apply(item, substitutions, active)?;
                }
            }
            ProjectedType::BoxedInterface {
                associated_type: Some(binding),
                ..
            } => apply(&mut binding.ty, substitutions, active)?,
            ProjectedType::Callback {
                parameters,
                native_substitutions,
                result,
                ..
            } => {
                for p in parameters {
                    apply(p, substitutions, active)?;
                }
                for v in native_substitutions.values_mut() {
                    apply(v, substitutions, active)?;
                }
                apply(result, substitutions, active)?;
            }
            _ => {}
        }
        refresh_native_spelling(ty);
        Ok(())
    }
    apply(ty, substitutions, None)
}

fn refresh_native_spelling(ty: &mut ProjectedType) {
    match ty {
        ProjectedType::Sequence { rust_path, item }
        | ProjectedType::Set {
            rust_path, item, ..
        } => {
            *rust_path = instantiate_projected_container(rust_path, &[item.rust_type()]);
        }
        ProjectedType::Mapping {
            rust_path,
            key,
            value,
            ..
        } => {
            *rust_path =
                instantiate_projected_container(rust_path, &[key.rust_type(), value.rust_type()]);
        }
        ProjectedType::Foreign {
            rust_path,
            base_rust_path,
            arguments,
            ..
        } => {
            *rust_path = if arguments.is_empty() {
                base_rust_path.clone()
            } else {
                instantiate_projected_container(
                    base_rust_path,
                    &arguments
                        .iter()
                        .map(ProjectedType::rust_type)
                        .collect::<Vec<_>>(),
                )
            };
        }
        _ => {}
    }
}

fn candidate_is_selected(
    oracle: Option<&crate::rust_interop::ProjectionOracle<'_>>,
    owner: &ProjectedType,
    reference: &super::ProjectedAssociatedTypeRef,
    result: &ProjectedType,
    substitutions: &BTreeMap<String, ProjectedType>,
) -> Result<bool, AssociatedNormalizationError> {
    // Ask rustc to select the complete equation, so conditional impl predicates
    // (including private ones) retain their native meaning. Do not invent a
    // second trait solver or treat every structural blanket match as applicable.
    let oracle = oracle.ok_or(AssociatedNormalizationError::Unsupported)?;
    let mut arguments = reference.trait_arguments.clone();
    for argument in &mut arguments {
        substitute_type(argument, substitutions)?;
    }
    let bound = crate::rust_ir::rust_associated_type_bound(
        reference
            .trait_rust_path
            .as_deref()
            .ok_or(AssociatedNormalizationError::Unsupported)?,
        arguments.iter().map(ProjectedType::rust_type),
        &reference.member,
        &result.rust_type(),
    )
    .ok_or(AssociatedNormalizationError::Unsupported)?;
    let rust_type = owner.rust_type();
    let inferred_parameters = super::lifetime_occurrences(&rust_type)
        .ok_or(AssociatedNormalizationError::Unsupported)?
        .into_iter()
        .chain(
            super::bound_lifetime_occurrences(&bound)
                .ok_or(AssociatedNormalizationError::Unsupported)?,
        )
        .filter(|name| !matches!(name.as_str(), "'_" | "'static"))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    let report = oracle
        .prove_bounds(&[crate::rust_interop::BoundQuestion {
            rust_type,
            rust_bound: bound,
            inferred_parameters,
        }])
        .map_err(|_| AssociatedNormalizationError::Unsupported)?;
    match report.evidence.first().map(|evidence| &evidence.answer) {
        Some(crate::rust_interop::ProbeAnswer::Yes) => Ok(true),
        Some(crate::rust_interop::ProbeAnswer::No) => Ok(false),
        _ => Err(AssociatedNormalizationError::Unsupported),
    }
}
