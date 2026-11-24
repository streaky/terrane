//! Declares checked native conversion candidates; a call must prove its concrete ABI.

use super::{
    ProjectedFunction, ProjectedGenericParameter, ProjectedItem, ProjectedKind, ProjectedParameter,
    ProjectedType,
};

const SOURCE: &str = "__TerraneFromSource";

pub(crate) fn from_destination(function: &ProjectedFunction) -> Option<&str> {
    (function.name == "from"
        && function
            .generic_parameters
            .first()
            .is_some_and(|parameter| parameter.name == SOURCE)
        && function.native_path.as_deref().is_some_and(|path| {
            path.ends_with(" as ::core::convert::From<__TerraneFromSource>>::from")
        }))
    .then_some(function.native_owner.as_deref())
    .flatten()
}

pub(crate) fn from_bound(input: &str) -> String {
    if let Some(referent) = input.strip_prefix('&') {
        format!("for<'terrane_from> ::core::convert::From<&'terrane_from {referent}>")
    } else {
        format!("::core::convert::From<{input}>")
    }
}

pub(crate) fn from_target(destination: &str, input: &str) -> String {
    format!("<{destination} as ::core::convert::From<{input}>>::from")
}

pub(crate) fn input_abi(parameter: &mut ProjectedParameter) -> Option<String> {
    if parameter.ty.contains_open_generic() {
        return None;
    }
    if let ProjectedType::Reference { inner, mutable, .. } = &parameter.ty {
        if !mutable && matches!(inner.as_ref(), ProjectedType::String) {
            parameter.ty = ProjectedType::String;
            parameter.borrowed = true;
            parameter.borrowed_str = true;
            return Some("&str".to_owned());
        }
        parameter.mutable_borrow = *mutable;
        let qualifier = if *mutable { "mut " } else { "" };
        return Some(format!("&{qualifier}{}", inner.rust_type()));
    }
    Some(parameter.ty.rust_type())
}

pub(super) fn add_from(item: &mut ProjectedItem) {
    let (ProjectedKind::ForeignType {
        methods,
        static_methods,
        generic_parameters,
        borrowed_view: false,
        native_view_type: None,
        enum_payload: None,
        ..
    }
    | ProjectedKind::Enum {
        methods,
        static_methods,
        generic_parameters,
        ..
    }) = &mut item.kind else {
        return;
    };
    if !generic_parameters.is_empty()
        || methods
            .iter()
            .chain(static_methods.iter())
            .any(|method| method.name == "from")
        || syn::parse_str::<syn::TypePath>(&item.rust_path).is_err()
    {
        return;
    }
    let owner = super::canonicalize_rust_path(&item.rust_path);
    static_methods.push(ProjectedFunction {
        native_path: Some(from_target(&owner, SOURCE)),
        native_owner: Some(owner),
        name: "from".to_owned(),
        parameters: vec![ProjectedParameter {
            name: "value".to_owned(),
            ty: ProjectedType::Generic(SOURCE.to_owned()),
            borrow_lifetime: None,
            borrowed: false,
            mutable_borrow: false,
            borrowed_str: false,
            generic_parameter: Some(SOURCE.to_owned()),
            generic_bounds: Vec::new(),
            generic_interface: None,
            associated_type: None,
        }],
        generic_parameters: vec![ProjectedGenericParameter {
            input_selected: true,
            name: SOURCE.to_owned(),
            rust_bounds: Vec::new(),
            default: None,
        }],
        receiver_bounds: Vec::new(),
        operation_owner_generics: Vec::new(),
        rust_generic_arguments: Vec::new(),
        result: ProjectedType::Foreign {
            rust_path: item.rust_path.clone(),
            name: item.name.clone(),
            base_rust_path: item.rust_path.clone(),
            arguments: Vec::new(),
        },
        destination_result: None,
        error: None,
        is_async: false,
        is_unsafe: false,
        into_future: false,
        execution_requirements: None,
        enum_operation: None,
        error_optional_depth: 0,
        chain_role: None,
        receiver_lifetime: None,
        receiver: None,
    });
}
