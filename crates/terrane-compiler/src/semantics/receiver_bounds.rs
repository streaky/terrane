use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
};

use crate::rust_interop::{
    CallQuestion,
    projection::{ProjectedFunction, ProjectedType},
};

/// Check predicates through the public operation: their Rust traits can live
/// in private modules and need not be nameable by a detached bound probe.
pub(super) fn native_receiver_call(
    function: &ProjectedFunction,
    receiver: &ProjectedType,
    replacements: &BTreeMap<String, String>,
    lifetimes: &BTreeSet<String>,
    loan_lifetime: Option<&str>,
) -> CallQuestion {
    let mutable_receiver = matches!(
        function.receiver,
        Some(crate::rust_interop::projection::Receiver::MutableBorrow)
    );
    let mut source = String::from(
        "#[allow(dead_code, unused_variables, unused_mut)] fn __terrane_receiver_probe",
    );
    let lifetimes = lifetimes
        .iter()
        .map(String::as_str)
        .chain(loan_lifetime)
        .collect::<Vec<_>>();
    if !lifetimes.is_empty() {
        source.push('<');
        source.push_str(&lifetimes.join(", "));
        source.push('>');
    }
    write!(
        source,
        "({}receiver: {}",
        if mutable_receiver { "mut " } else { "" },
        receiver.rust_type()
    )
    .unwrap();
    for (index, parameter) in function.parameters.iter().enumerate() {
        let ty = crate::rust_ir::instantiate_rust_generics(&parameter.ty.rust_type(), replacements);
        write!(
            source,
            ", {}argument_{index}: {ty}",
            if parameter.mutable_borrow { "mut " } else { "" }
        )
        .unwrap();
    }
    let native_path = function
        .native_path
        .as_ref()
        .map(|path| crate::rust_ir::instantiate_rust_generics(path, replacements));
    source.push_str(") { let _ = ");
    if let Some(path) = &native_path {
        source.push_str(path);
    } else {
        write!(
            source,
            "receiver.r#{}",
            function.name.trim_start_matches("r#")
        )
        .unwrap();
    }
    if !function.rust_generic_arguments.is_empty() {
        source.push_str("::<");
        for (index, argument) in function.rust_generic_arguments.iter().enumerate() {
            if index != 0 {
                source.push_str(", ");
            }
            source.push_str(&crate::rust_ir::instantiate_rust_generics(
                &argument.rust_type(),
                replacements,
            ));
        }
        source.push('>');
    }
    source.push('(');
    if native_path.is_some() {
        let receiver_argument = match function.receiver {
            Some(crate::rust_interop::projection::Receiver::Move) => "receiver",
            Some(crate::rust_interop::projection::Receiver::MutableBorrow) => "&mut receiver",
            _ => "&receiver",
        };
        source.push_str(receiver_argument);
    }
    for (index, parameter) in function.parameters.iter().enumerate() {
        let borrow = if parameter.borrowed {
            if parameter.mutable_borrow {
                "&mut "
            } else {
                "&"
            }
        } else {
            ""
        };
        if index != 0 || native_path.is_some() {
            source.push_str(", ");
        }
        write!(source, "{borrow}argument_{index}").unwrap();
    }
    source.push_str("); }\nfn main() {}\n");
    CallQuestion {
        label: format!(
            "native receiver: {}",
            native_path.as_deref().unwrap_or(&function.name)
        ),
        source,
    }
}
