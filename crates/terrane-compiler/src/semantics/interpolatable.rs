//! Nominal interpolation conformance and the compiler-owned scalar implementations.
use super::{
    member_inference::descriptor_protocol_method,
    model::{DescriptorContract, FunctionContract, ObjectIdentity, SemanticUnit, ValueType},
    types::object_types_compatible,
};
use crate::{InvocationMode, ScalarType};

pub(crate) fn identity() -> &'static ObjectIdentity {
    static IDENTITY: std::sync::LazyLock<ObjectIdentity> =
        std::sync::LazyLock::new(|| ObjectIdentity::new("/core/display", "interpolatable"));
    &IDENTITY
}

pub(crate) fn is_protocol(value: &ObjectIdentity) -> bool {
    value == identity()
}

pub(crate) fn conforms(unit: &SemanticUnit, value: &ValueType, at: usize) -> bool {
    match value {
        ValueType::Union(arms) => arms.iter().all(|arm| conforms(unit, arm, at)),
        ValueType::Optional(inner) => conforms(unit, inner, at),
        ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner) => conforms(unit, inner.value_type_ref(), at),
        ValueType::TypeParameter(name) => unit
            .type_parameter_at(at, name)
            .and_then(|parameter| parameter.bound.as_ref())
            .is_some_and(|bound| object_types_compatible(&unit.descriptors, identity(), bound)),
        _ => concrete_conforms(&unit.descriptors, value),
    }
}

pub(crate) fn concrete_conforms(objects: &[DescriptorContract], value: &ValueType) -> bool {
    match value {
        ValueType::Scalar(scalar) => *scalar != ScalarType::Bytes,
        ValueType::Object(actual) => object_types_compatible(objects, identity(), actual),
        ValueType::Union(arms) => arms.iter().all(|arm| concrete_conforms(objects, arm)),
        ValueType::Optional(inner) => concrete_conforms(objects, inner),
        ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner) => concrete_conforms(objects, inner.value_type_ref()),
        _ => false,
    }
}

/// Bounds describe the generic value representation, not an implicit read facade.
/// Borrowing a conforming value uses a `ref T` parameter, as for other interfaces.
pub(crate) fn generic_conforms(unit: &SemanticUnit, value: &ValueType, at: usize) -> bool {
    match value {
        ValueType::Reference(_)
        | ValueType::MutableReference(_)
        | ValueType::SharedReference(_) => false,
        ValueType::Optional(inner) => generic_conforms(unit, inner, at),
        _ => conforms(unit, value, at),
    }
}

pub(crate) fn method<'a>(
    unit: &'a SemanticUnit,
    value: &ValueType,
    at: usize,
) -> Option<&'a FunctionContract> {
    let owner = match value {
        ValueType::Reference(inner)
        | ValueType::MutableReference(inner)
        | ValueType::SharedReference(inner) => return method(unit, inner.value_type_ref(), at),
        ValueType::Object(owner) => owner,
        ValueType::TypeParameter(name) => unit.type_parameter_at(at, name)?.bound.as_ref()?,
        _ => return None,
    };
    if !object_types_compatible(&unit.descriptors, identity(), owner) {
        return None;
    }
    descriptor_protocol_method(unit, owner, "interpolate").filter(|method| {
        !method.is_async
            && !method.is_unsafe
            && method.written_invocation_mode == InvocationMode::Shared
    })
}

/// Source signatures retain `string`; the intrinsic read-only key ABI lends text.
pub(crate) fn borrows_key(unit: &SemanticUnit, method: &FunctionContract) -> bool {
    method.name == "interpolate"
        && method
            .owner_identity
            .as_ref()
            .is_some_and(|owner| object_types_compatible(&unit.descriptors, identity(), owner))
        && method.parameters.len() == 1
        && !method.parameters[0].mutable
}
