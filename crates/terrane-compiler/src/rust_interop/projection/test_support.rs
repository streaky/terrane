//! Shared public-metadata fixtures for projection tests; never compiled into production.

use super::{ProjectedFunction, ProjectedItem, ProjectedKind, ProjectedType};
use crate::RustDependency;

pub(super) fn dependency(name: &str, package: &str, features: &[&str]) -> RustDependency {
    RustDependency {
        name: name.to_owned(),
        package: package.to_owned(),
        version: "=1.0.0".to_owned(),
        features: features
            .iter()
            .map(|feature| (*feature).to_owned())
            .collect(),
        default_features: true,
        target: None,
        effects: Vec::new(),
    }
}

pub(super) fn projected_function_item(
    namespace: &str,
    name: &str,
    rust_path: &str,
) -> ProjectedItem {
    ProjectedItem {
        namespace: namespace.to_owned(),
        name: name.to_owned(),
        rust_path: rust_path.to_owned(),
        docs: None,
        kind: ProjectedKind::Function(ProjectedFunction {
            native_owner: None,
            native_path: None,
            name: name.to_owned(),
            generic_parameters: Vec::new(),
            operation_owner_generics: Vec::new(),
            rust_generic_arguments: Vec::new(),
            parameters: Vec::new(),
            result: ProjectedType::None,
            destination_result: None,
            error: None,
            is_async: false,
            is_unsafe: false,
            into_future: false,
            execution_requirements: None,
            enum_operation: None,
            error_optional_depth: 0,
            chain_role: None,
            receiver: None,
        }),
    }
}
