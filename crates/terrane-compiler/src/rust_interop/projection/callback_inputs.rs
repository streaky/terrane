//! One native-input plan for source callback signatures, checking and lowering.
use super::ProjectedType;

/// Mutable loans use the native scalar width, never an adaptive snapshot.
/// Alias substitution can close a generic payload, so normalization also runs
/// when the provider's callback plan is installed in a consumer signature.
pub(super) fn normalize(callback: &mut ProjectedType) {
    let ProjectedType::Callback {
        parameters,
        parameter_rust_types,
        parameter_borrows,
        ..
    } = callback
    else {
        return;
    };
    for (index, (parameter, native)) in parameters.iter_mut().zip(parameter_rust_types).enumerate()
    {
        if parameter_borrows.get(index) == Some(&true)
            && matches!(syn::parse_str::<syn::Type>(native), Ok(syn::Type::Reference(reference)) if reference.mutability.is_some())
        {
            match parameter {
                ProjectedType::Int => *parameter = ProjectedType::FixedInt("i64".to_owned()),
                ProjectedType::RustInt(name)
                    if crate::ScalarType::ALL
                        .iter()
                        .any(|scalar| scalar.rust_type() == Some(name.as_str())) =>
                {
                    *parameter = ProjectedType::FixedInt(std::mem::take(name));
                }
                _ => {}
            }
        }
    }
}

pub(super) fn validate_native_inputs(parameters: &[ProjectedType]) -> Result<(), String> {
    if parameters
        .iter()
        .any(super::functions::contains_owned_callback_output)
    {
        return Err("native callback inputs cannot contain owning callback carriers".to_owned());
    }
    Ok(())
}
