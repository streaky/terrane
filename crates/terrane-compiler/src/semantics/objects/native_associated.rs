use super::super::prelude::*;
use super::PendingProjectedBound;
use crate::rust_interop::projection::{
    AssociatedNormalizationError, ProjectedFunction, ProjectedType,
};

/// Some existing native graphs infer an owner through another input's trait
/// equality (for example `E: Executor<Database = DB>`). Preserve that joint
/// question until Rust selects `DB`; it is not an input-selected owner yet.
pub(super) fn requires_joint_selection(
    operation: &ProjectedFunction,
    bindings: &BTreeMap<String, ProjectedType>,
) -> bool {
    operation
        .generic_parameters
        .iter()
        .chain(&operation.operation_owner_generics)
        .any(|parameter| {
            !parameter.input_selected
                && !bindings.contains_key(&parameter.name)
                && result_uses_associated_owner(&operation.result, &parameter.name)
        })
}

fn result_uses_associated_owner(ty: &ProjectedType, name: &str) -> bool {
    match ty {
        ProjectedType::Associated(reference) => {
            reference.owner == name
                || reference
                    .owner_type
                    .as_ref()
                    .is_some_and(|owner| result_uses_associated_owner(owner, name))
                || reference
                    .owner_arguments
                    .iter()
                    .chain(&reference.trait_arguments)
                    .any(|argument| result_uses_associated_owner(argument, name))
        }
        ProjectedType::Generic(generic) => generic == name,
        ProjectedType::Sequence { item, .. }
        | ProjectedType::Set { item, .. }
        | ProjectedType::Optional(item)
        | ProjectedType::AsyncIterationStep(item)
        | ProjectedType::Reference { inner: item, .. }
        | ProjectedType::InvocationScoped { owned: item, .. } => {
            result_uses_associated_owner(item, name)
        }
        ProjectedType::Mapping { key, value, .. } => {
            result_uses_associated_owner(key, name) || result_uses_associated_owner(value, name)
        }
        ProjectedType::Tuple(items)
        | ProjectedType::Foreign {
            arguments: items, ..
        } => items
            .iter()
            .any(|item| result_uses_associated_owner(item, name)),
        ProjectedType::BoxedInterface {
            associated_type: Some(binding),
            ..
        } => result_uses_associated_owner(&binding.ty, name),
        ProjectedType::Callback {
            parameters, result, ..
        } => {
            parameters
                .iter()
                .any(|parameter| result_uses_associated_owner(parameter, name))
                || result_uses_associated_owner(result, name)
        }
        _ => false,
    }
}

pub(super) fn normalization_failure(
    unit: &SemanticUnit,
    span: Span,
    reason: AssociatedNormalizationError,
) -> SemanticFailure {
    let detail = match reason {
        AssociatedNormalizationError::Unknown => {
            "the selected input has no known associated implementation"
        }
        AssociatedNormalizationError::Ambiguous => {
            "the selected input has ambiguous associated implementations"
        }
        AssociatedNormalizationError::Unsupported => {
            "the associated projection has no supported exact contract"
        }
    };
    failure(
        &unit.source,
        "T0119",
        format!("cannot normalize native result: {detail}"),
        span,
    )
}

pub(super) fn equality_bounds(
    package: &SemanticPackage,
    unit: &SemanticUnit,
    span: Span,
    operation: &ProjectedFunction,
    bindings: &BTreeMap<String, ProjectedType>,
    existing: &[PendingProjectedBound],
) -> Result<Vec<PendingProjectedBound>, SemanticFailure> {
    let workspace = package.root.join(".trn/dependencies");
    let oracle = crate::rust_interop::ProjectionOracle::new(
        &workspace,
        &package.projection.cache_identity,
        package.projection.containment,
    );
    package
        .projection
        .associated_equality_obligations(operation, bindings, Some(&oracle))
        .map_err(|reason| normalization_failure(unit, span, reason))?
        .into_iter()
        .map(|obligation| {
            let generic = obligation.owner_parameter.filter(|name| {
                operation
                    .generic_parameters
                    .iter()
                    .any(|parameter| parameter.input_selected && parameter.name == *name)
            });
            let direct_rust_type = obligation.owner.rust_type();
            let abi = existing.iter().find(|bound| {
                bound.generic == generic
                    && super::rust_type_text_equal(
                        &package
                            .projection
                            .canonical_native_type(&bound.direct_rust_type),
                        &package.projection.canonical_native_type(&direct_rust_type),
                    )
            });
            let rust_bound = crate::rust_ir::rust_associated_type_bound(
                &obligation.trait_rust_path,
                obligation
                    .trait_arguments
                    .iter()
                    .map(ProjectedType::rust_type),
                &obligation.member,
                &obligation.result.rust_type(),
            )
            .ok_or_else(|| {
                normalization_failure(unit, span, AssociatedNormalizationError::Unsupported)
            })?;
            let owner_lifetimes =
                crate::rust_interop::projection::lifetime_occurrences(&direct_rust_type)
                    .ok_or_else(|| {
                        normalization_failure(unit, span, AssociatedNormalizationError::Unsupported)
                    })?;
            let bound_lifetimes =
                crate::rust_interop::projection::bound_lifetime_occurrences(&rust_bound)
                    .ok_or_else(|| {
                        normalization_failure(unit, span, AssociatedNormalizationError::Unsupported)
                    })?;
            let inferred_parameters = owner_lifetimes
                .into_iter()
                .chain(bound_lifetimes)
                .filter(|name| !matches!(name.as_str(), "'_" | "'static"))
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            Ok(PendingProjectedBound {
                receiver_call: None,
                generic,
                direct_rust_type,
                borrowed_rust_type: abi.and_then(|bound| bound.borrowed_rust_type.clone()),
                mutable_borrowed_rust_type: abi
                    .and_then(|bound| bound.mutable_borrowed_rust_type.clone()),
                borrowed_mutable: abi.is_some_and(|bound| bound.borrowed_mutable),
                rust_bound,
                inferred_parameters,
            })
        })
        .collect()
}
