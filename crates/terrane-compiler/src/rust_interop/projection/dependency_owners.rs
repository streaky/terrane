//! Owns dependency reachability, owner binding, and Cargo-root rewrites.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use super::cargo_workspace::{is_crates_io_lock_source, resolved_package_versions};
use super::{
    DeclinedItem, ProjectedBoundDependency, ProjectedBoundaryCapabilities, ProjectedDependency,
    ProjectedEnumOperation, ProjectedFunction, ProjectedItem, ProjectedKind, ProjectedType,
    ProjectionError, RustDependency, io_error, rust_bound_roots,
};
pub(super) fn enforce_transitive_reachability(
    projected: &mut [ProjectedDependency],
    dependencies: &[RustDependency],
    private_dependencies: &[ProjectedBoundDependency],
    workspace: &Path,
    error_owners_only: bool,
) -> Result<(), ProjectionError> {
    let mut declared = dependencies
        .iter()
        .flat_map(|dependency| [&dependency.name, &dependency.package])
        .map(|name| name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    declared.extend(
        private_dependencies
            .iter()
            .map(|dependency| dependency.name.replace('-', "_")),
    );
    let versions = resolved_package_versions(workspace)?;
    for dependency in &mut *projected {
        let mut retained = Vec::new();
        for mut item in std::mem::take(&mut dependency.items) {
            if let Some(owner) = item_undeclared_owner(&item, &declared, error_owners_only) {
                let owner = owner.to_owned();
                dependency.declined.push(DeclinedItem {
                    rust_path: item.rust_path,
                    reason: undeclared_owner_reason(&owner, &versions),
                });
                continue;
            }
            if let ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } = &mut item.kind
            {
                let owner_path = item.rust_path.clone();
                for candidates in [methods, static_methods] {
                    let mut retained_methods = Vec::new();
                    for method in std::mem::take(candidates) {
                        if let Some(owner) =
                            function_undeclared_owner(&method, &declared, error_owners_only)
                        {
                            dependency.declined.push(DeclinedItem {
                                rust_path: format!("{owner_path}::{}", method.name),
                                reason: undeclared_owner_reason(owner, &versions),
                            });
                        } else {
                            retained_methods.push(method);
                        }
                    }
                    *candidates = retained_methods;
                }
            }
            retained.push(item);
        }
        dependency.items = retained;
        dependency
            .declined
            .sort_by(|left, right| left.rust_path.cmp(&right.rust_path));
    }

    for owner in declared {
        let Some(owner_versions) = versions.get(&owner) else {
            continue;
        };
        let referenced = projected.iter().any(|dependency| {
            dependency.package.replace('-', "_") != owner
                && dependency.items.iter().any(|item| {
                    item_foreign_owners(item)
                        .into_iter()
                        .any(|candidate| candidate == owner)
                })
        });
        if referenced && owner_versions.len() > 1 {
            return Err(ProjectionError {
                message: format!(
                    "declared Rust crate `{}` resolves to multiple versions ({}) while a dependency signature uses its types",
                    owner.replace('_', "-"),
                    owner_versions
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            });
        }
    }
    Ok(())
}
pub(super) fn resolve_cross_dependency_boundary_conversions(projected: &mut [ProjectedDependency]) {
    let capabilities = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match &item.kind {
            ProjectedKind::ForeignType { boundary, .. }
                if boundary != &ProjectedBoundaryCapabilities::default() =>
            {
                Some((item.rust_path.clone(), boundary.clone()))
            }
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();

    for item in projected
        .iter_mut()
        .flat_map(|dependency| &mut dependency.items)
    {
        let ProjectedKind::Enum {
            methods,
            static_methods,
            ..
        } = &mut item.kind
        else {
            continue;
        };
        for function in static_methods.iter_mut().chain(methods.iter_mut()) {
            let Some(operation) = &mut function.enum_operation else {
                continue;
            };
            match operation {
                ProjectedEnumOperation::Construct {
                    unit: false,
                    conversion,
                    payload_rust_type,
                    ..
                } => {
                    let Some(boundary) = capabilities.get(payload_rust_type) else {
                        continue;
                    };
                    let selected = if boundary.bytes_extraction.is_some() {
                        boundary
                            .bytes_constructor
                            .map(|conversion| (ProjectedType::Bytes, conversion))
                    } else if boundary.string_extraction.is_some() {
                        boundary
                            .string_constructor
                            .map(|conversion| (ProjectedType::String, conversion))
                    } else {
                        None
                    };
                    if let Some((ty, selected_conversion)) = selected
                        && let Some(parameter) = function.parameters.first_mut()
                    {
                        parameter.ty = ty;
                        *conversion = selected_conversion;
                    }
                }
                ProjectedEnumOperation::Extract {
                    conversion,
                    payload_rust_type,
                    ..
                } => {
                    let Some(boundary) = capabilities.get(payload_rust_type) else {
                        continue;
                    };
                    let selected = boundary
                        .bytes_extraction
                        .map(|conversion| (ProjectedType::Bytes, conversion))
                        .or_else(|| {
                            boundary
                                .string_extraction
                                .map(|conversion| (ProjectedType::String, conversion))
                        });
                    if let Some((ty, selected_conversion)) = selected {
                        function.result = ProjectedType::Optional(Box::new(ty));
                        *conversion = selected_conversion;
                    }
                }
                _ => {}
            }
        }
    }
}

pub(super) fn decline_unrepresentable_error_types(projected: &mut [ProjectedDependency]) {
    let projected_error_types = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match item.kind {
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => {
                Some(item.rust_path.clone())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let displayable = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter_map(|item| match &item.kind {
            ProjectedKind::ForeignType {
                displayable: true, ..
            }
            | ProjectedKind::Enum {
                displayable: true, ..
            } => Some(item.rust_path.clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    for dependency in projected {
        let mut retained = Vec::with_capacity(dependency.items.len());
        for mut item in std::mem::take(&mut dependency.items) {
            let item_path = item.rust_path.clone();
            match &mut item.kind {
                ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                    if let Some(error) =
                        unsupported_projected_error(function, &projected_error_types, &displayable)
                    {
                        dependency.declined.push(DeclinedItem {
                            rust_path: item_path,
                            reason: format!(
                                "projected error `{error}` has no canonical displayable type"
                            ),
                        });
                        continue;
                    }
                }
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => {
                    retain_displayable_error_methods(
                        methods,
                        &projected_error_types,
                        &displayable,
                        &item_path,
                        &mut dependency.declined,
                    );
                    retain_displayable_error_methods(
                        static_methods,
                        &projected_error_types,
                        &displayable,
                        &item_path,
                        &mut dependency.declined,
                    );
                }
                ProjectedKind::Interface(interface) => {
                    interface.methods.retain(|method| {
                        let Some(error) = unsupported_projected_error(
                            &method.function,
                            &projected_error_types,
                            &displayable,
                        ) else {
                            return true;
                        };
                        dependency.declined.push(DeclinedItem {
                            rust_path: format!("{item_path}::{}", method.function.name),
                            reason: format!(
                                "projected error `{error}` has no canonical displayable type"
                            ),
                        });
                        false
                    });
                }
            }
            retained.push(item);
        }
        dependency.items = retained;
    }
}

fn retain_displayable_error_methods(
    methods: &mut Vec<ProjectedFunction>,
    projected_error_types: &BTreeSet<String>,
    displayable: &BTreeSet<String>,
    owner_path: &str,
    declined: &mut Vec<DeclinedItem>,
) {
    methods.retain(|method| {
        let Some(error) = unsupported_projected_error(method, projected_error_types, displayable)
        else {
            return true;
        };
        declined.push(DeclinedItem {
            rust_path: format!("{owner_path}::{}", method.name),
            reason: format!("projected error `{error}` has no canonical displayable type"),
        });
        false
    });
}

fn unsupported_projected_error<'a>(
    function: &'a ProjectedFunction,
    projected_error_types: &BTreeSet<String>,
    displayable: &BTreeSet<String>,
) -> Option<&'a str> {
    function.error.as_deref().filter(|error| {
        let generic_fallback = error == &"Error"
            || rust_path_owner(error)
                .is_some_and(|owner| matches!(owner, "std" | "core" | "alloc"));
        !generic_fallback
            && (!projected_error_types.contains(*error) || !displayable.contains(*error))
    })
}
#[expect(
    clippy::too_many_lines,
    reason = "one recursive metadata pass keeps all native names and error identities canonical"
)]
pub(super) fn canonicalize_projected_type_names(projected: &mut [ProjectedDependency]) {
    let mut names = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| {
            matches!(
                item.kind,
                ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
            )
        })
        .map(|item| (item.rust_path.clone(), item.name.clone()))
        .collect::<BTreeMap<_, _>>();
    for item in projected.iter().flat_map(|dependency| &dependency.items) {
        let (ProjectedKind::ForeignType {
            generic_parameters: parameters,
            ..
        }
        | ProjectedKind::Enum {
            generic_parameters: parameters,
            ..
        }) = &item.kind
        else {
            continue;
        };
        if !parameters.is_empty()
            && let Some(root) = crate::rust_ir::rust_type_constructor(&item.rust_path)
        {
            names.entry(root).or_insert_with(|| item.name.clone());
        }
    }
    let error_paths = projected
        .iter()
        .flat_map(|dependency| {
            let dependency_root = format!("/deps/{}", dependency.name.replace('_', "-"));
            let rust_crate = dependency.name.replace('-', "_");
            dependency.items.iter().filter_map(move |item| {
                if !matches!(
                    item.kind,
                    ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
                ) {
                    return None;
                }
                let relative_namespace = item
                    .namespace
                    .strip_prefix(&dependency_root)?
                    .trim_start_matches('/')
                    .replace('/', "::");
                let alias = if relative_namespace.is_empty() {
                    format!("{rust_crate}::{}", item.name)
                } else {
                    format!("{rust_crate}::{relative_namespace}::{}", item.name)
                };
                Some((alias, item.rust_path.clone()))
            })
        })
        .collect::<BTreeMap<_, _>>();
    let mut unique_error_paths = BTreeMap::<String, Option<String>>::new();
    for item in projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .filter(|item| {
            matches!(
                item.kind,
                ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
            )
        })
    {
        unique_error_paths
            .entry(item.name.clone())
            .and_modify(|path| *path = None)
            .or_insert_with(|| Some(item.rust_path.clone()));
    }
    let canonicalize_function = |function: &mut ProjectedFunction| {
        for ty in function
            .parameters
            .iter_mut()
            .map(|parameter| &mut parameter.ty)
            .chain(std::iter::once(&mut function.result))
            .chain(
                function
                    .generic_parameters
                    .iter_mut()
                    .chain(function.operation_owner_generics.iter_mut())
                    .filter_map(|parameter| parameter.default.as_mut()),
            )
        {
            canonicalize_projected_type_name(ty, &names);
        }
        if let Some(error) = &mut function.error {
            let canonical = error_paths.get(error).cloned().or_else(|| {
                rust_path_owner(error)?;
                let name = error.rsplit("::").next()?;
                unique_error_paths.get(name)?.clone()
            });
            if let Some(canonical) = canonical {
                error.clone_from(&canonical);
            }
        }
    };
    for item in projected
        .iter_mut()
        .flat_map(|dependency| &mut dependency.items)
    {
        match &mut item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                canonicalize_function(function);
            }
            ProjectedKind::ForeignType {
                fields,
                methods,
                static_methods,
                constructor,
                generic_parameters,
                ..
            } => {
                for field in fields {
                    canonicalize_projected_type_name(&mut field.ty, &names);
                }
                for parameter in generic_parameters {
                    if let Some(default) = &mut parameter.default {
                        canonicalize_projected_type_name(default, &names);
                    }
                }
                for function in methods.iter_mut().chain(static_methods).chain(constructor) {
                    canonicalize_function(function);
                }
            }
            ProjectedKind::Enum {
                variants,
                methods,
                static_methods,
                generic_parameters,
                ..
            } => {
                for field in variants.iter_mut().flat_map(|variant| &mut variant.fields) {
                    canonicalize_projected_type_name(&mut field.ty, &names);
                }
                for parameter in generic_parameters {
                    if let Some(default) = &mut parameter.default {
                        canonicalize_projected_type_name(default, &names);
                    }
                }
                for function in methods.iter_mut().chain(static_methods) {
                    canonicalize_function(function);
                }
            }
            ProjectedKind::Interface(interface) => {
                for method in &mut interface.methods {
                    canonicalize_function(&mut method.function);
                }
            }
        }
    }
}

fn canonicalize_projected_type_name(ty: &mut ProjectedType, names: &BTreeMap<String, String>) {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            name,
            arguments,
            ..
        } => {
            if let Some(canonical) = names.get(rust_path) {
                name.clone_from(canonical);
            } else if !arguments.is_empty()
                && let Some(root) = crate::rust_ir::rust_type_constructor(rust_path)
                && let Some(canonical) = names.get(&root)
            {
                name.clone_from(canonical);
            }
            for argument in arguments {
                canonicalize_projected_type_name(argument, names);
            }
        }
        ProjectedType::InvocationScoped { name, owned, .. } => {
            canonicalize_projected_type_name(owned, names);
            if !matches!(owned.as_ref(), ProjectedType::Optional(_)) {
                *name = owned.terrane_name();
            }
        }
        ProjectedType::Optional(inner)
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. }
        | ProjectedType::Reference { inner, .. } => {
            canonicalize_projected_type_name(inner, names);
        }
        ProjectedType::Mapping { key, value, .. } => {
            canonicalize_projected_type_name(key, names);
            canonicalize_projected_type_name(value, names);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                canonicalize_projected_type_name(item, names);
            }
        }
        _ => {}
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "bound-owner validation reports the complete resolved dependency conflict"
)]
pub(super) fn decline_unnameable_bound_owners(
    projected: &mut [ProjectedDependency],
    declared: &[RustDependency],
    bound_dependencies: &[ProjectedBoundDependency],
    workspace: &Path,
) -> Result<(), ProjectionError> {
    let declared = declared
        .iter()
        .map(|dependency| dependency.name.replace('-', "_"))
        .chain(
            bound_dependencies
                .iter()
                .map(|dependency| dependency.name.clone()),
        )
        .collect::<BTreeSet<_>>();
    let text = fs::read_to_string(workspace.join("Cargo.lock"))
        .map_err(io_error("read dependency projection lockfile"))?;
    let lock = text
        .parse::<toml::Value>()
        .map_err(|error| ProjectionError {
            message: format!("invalid dependency projection lockfile: {error}"),
        })?;
    let packages = lock
        .get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|package| {
            Some((
                package.get("name")?.as_str()?.to_owned(),
                package.get("version")?.as_str()?.to_owned(),
                package
                    .get("source")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
            ))
        })
        .collect::<Vec<_>>();
    let reason = |function: &ProjectedFunction| {
        let destination = function.destination_result.as_ref()?;
        destination.bound_roots.iter().find_map(|root| {
            if declared.contains(root)
                || matches!(root.as_str(), "std" | "core" | "alloc" | "self" | "crate")
            {
                return None;
            }
            let matches = packages
                .iter()
                .filter(|(package, _, _)| package.replace('-', "_") == *root)
                .collect::<Vec<_>>();
            match matches.as_slice() {
                [] => Some(format!(
                    "destination-result bound owner `{root}` is not a resolved package"
                )),
                [(_, _, Some(source))] if is_crates_io_lock_source(source) => None,
                [(_, version, _)] => Some(format!(
                    "destination-result bound owner `{root}` at `{version}` is not a nameable registry dependency"
                )),
                _ => Some(format!(
                    "destination-result bound owner `{root}` resolves to multiple package versions"
                )),
            }
        })
    };
    for dependency in projected {
        let mut retained = Vec::new();
        for mut item in std::mem::take(&mut dependency.items) {
            match &mut item.kind {
                ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                    if let Some(reason) = reason(function) {
                        dependency.declined.push(DeclinedItem {
                            rust_path: item.rust_path,
                            reason,
                        });
                        continue;
                    }
                }
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => {
                    for method_list in [methods, static_methods] {
                        let mut kept = Vec::new();
                        for method in std::mem::take(method_list) {
                            if let Some(reason) = reason(&method) {
                                dependency.declined.push(DeclinedItem {
                                    rust_path: format!("{}::{}", item.rust_path, method.name),
                                    reason,
                                });
                            } else {
                                kept.push(method);
                            }
                        }
                        *method_list = kept;
                    }
                }
                ProjectedKind::Interface(interface) => {
                    if let Some((method, reason)) = interface.methods.iter().find_map(|method| {
                        reason(&method.function).map(|reason| (&method.function, reason))
                    }) {
                        dependency.declined.push(DeclinedItem {
                            rust_path: format!("{}::{}", item.rust_path, method.name),
                            reason,
                        });
                        continue;
                    }
                }
            }
            retained.push(item);
        }
        dependency.items = retained;
        dependency
            .declined
            .sort_by(|left, right| left.rust_path.cmp(&right.rust_path));
    }
    Ok(())
}

pub(super) fn projected_bound_dependencies(
    projected: &[ProjectedDependency],
    declared: &[RustDependency],
    private_dependencies: &[ProjectedBoundDependency],
    workspace: &Path,
) -> Result<Vec<ProjectedBoundDependency>, ProjectionError> {
    let mut declared = declared
        .iter()
        .map(|dependency| dependency.name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    declared.extend(
        private_dependencies
            .iter()
            .map(|dependency| dependency.name.clone()),
    );
    let required = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .flat_map(|item| match &item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => vec![function],
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => methods.iter().chain(static_methods).collect(),
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter()
                .map(|method| &method.function)
                .collect(),
        })
        .flat_map(|function| {
            let mut roots = function
                .destination_result
                .iter()
                .flat_map(|destination| destination.bound_roots.iter().cloned())
                .collect::<BTreeSet<_>>();
            roots.extend(
                function
                    .generic_parameters
                    .iter()
                    .chain(&function.operation_owner_generics)
                    .flat_map(|parameter| &parameter.rust_bounds)
                    .flat_map(|bound| rust_bound_roots(bound)),
            );
            for parameter in function
                .generic_parameters
                .iter()
                .chain(&function.operation_owner_generics)
            {
                roots.remove(&parameter.name);
            }
            roots
        })
        .filter(|root| {
            !declared.contains(root)
                && !matches!(
                    root.as_str(),
                    "std" | "core" | "alloc" | "self" | "crate" | "Self"
                )
        })
        .collect::<BTreeSet<_>>();
    resolved_projected_dependencies(required, workspace, false)
}

fn resolved_projected_dependencies(
    required: BTreeSet<String>,
    workspace: &Path,
    strict: bool,
) -> Result<Vec<ProjectedBoundDependency>, ProjectionError> {
    if required.is_empty() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(workspace.join("Cargo.lock"))
        .map_err(io_error("read dependency projection lockfile"))?;
    let lock = text
        .parse::<toml::Value>()
        .map_err(|error| ProjectionError {
            message: format!("invalid dependency projection lockfile: {error}"),
        })?;
    let packages = lock
        .get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|package| {
            Some((
                package.get("name")?.as_str()?.to_owned(),
                package.get("version")?.as_str()?.to_owned(),
                package
                    .get("source")
                    .and_then(toml::Value::as_str)
                    .map(str::to_owned),
            ))
        })
        .collect::<Vec<_>>();
    let mut dependencies = Vec::new();
    for root in required {
        let matches = packages
            .iter()
            .filter(|(package, _, _)| package.replace('-', "_") == root)
            .collect::<Vec<_>>();
        let [(package, version, source)] = matches.as_slice() else {
            if strict {
                return Err(ProjectionError {
                    message: format!(
                        "projected result bound root `{root}` is not a unique resolved package"
                    ),
                });
            }
            continue;
        };
        if !source.as_deref().is_some_and(is_crates_io_lock_source) {
            if strict {
                return Err(ProjectionError {
                    message: format!(
                        "projected result bound root `{root}` is not a nameable registry dependency"
                    ),
                });
            }
            continue;
        }
        dependencies.push(ProjectedBoundDependency {
            name: root,
            package: package.clone(),
            version: format!("={version}"),
        });
    }
    Ok(dependencies)
}

pub(super) fn recursive_owner_dependencies(
    projected: &[ProjectedDependency],
    declared: &[RustDependency],
    workspace: &Path,
    _error_owners_only: bool,
) -> Result<Vec<ProjectedBoundDependency>, ProjectionError> {
    let reachable = declared
        .iter()
        .flat_map(|dependency| [&dependency.name, &dependency.package])
        .map(|name| name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    let required = projected
        .iter()
        .flat_map(|dependency| &dependency.items)
        .flat_map(item_foreign_owners)
        .filter(|owner| !owner_is_reachable(owner, &reachable))
        .collect::<BTreeSet<_>>();
    let mut dependencies = resolved_projected_dependencies(required, workspace, false)?;
    for dependency in &mut dependencies {
        dependency.name = format!("__terrane_recursive_{}", dependency.name);
    }
    Ok(dependencies)
}
pub(super) fn rewrite_rust_bound_root(
    bound: &str,
    package_root: &str,
    dependency_root: &str,
) -> String {
    let mut rendered = String::with_capacity(bound.len());
    let mut cursor = 0;
    while let Some(relative_start) = bound[cursor..].find(package_root) {
        let start = cursor + relative_start;
        let end = start + package_root.len();
        let boundary_before = bound[..start].chars().next_back().is_none_or(|character| {
            !(character.is_alphanumeric() || matches!(character, '_' | ':'))
        });
        let path_root = bound[end..].starts_with("::") || (start == 0 && end == bound.len());
        rendered.push_str(&bound[cursor..start]);
        if boundary_before && path_root {
            rendered.push_str(dependency_root);
        } else {
            rendered.push_str(package_root);
        }
        cursor = end;
    }
    rendered.push_str(&bound[cursor..]);
    rendered
}

fn item_undeclared_owner<'a>(
    item: &'a ProjectedItem,
    declared: &BTreeSet<String>,
    error_owners_only: bool,
) -> Option<&'a str> {
    if error_owners_only {
        return match &item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                function_error_undeclared_owner(function, declared)
            }
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter()
                .find_map(|method| function_error_undeclared_owner(&method.function, declared)),
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => None,
        };
    }
    rust_path_owner(&item.rust_path)
        .filter(|owner| !owner_is_reachable(owner, declared))
        .or_else(|| match &item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                function_undeclared_owner(function, declared, false)
            }
            ProjectedKind::Interface(interface) => interface
                .methods
                .iter()
                .find_map(|method| function_undeclared_owner(&method.function, declared, false)),
            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => None,
        })
}

fn function_undeclared_owner<'a>(
    function: &'a ProjectedFunction,
    declared: &BTreeSet<String>,
    error_owners_only: bool,
) -> Option<&'a str> {
    if error_owners_only {
        return function_error_undeclared_owner(function, declared);
    }
    function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
        .find_map(|ty| type_undeclared_owner(ty, declared))
        .or_else(|| function_error_undeclared_owner(function, declared))
}
fn function_error_undeclared_owner<'a>(
    function: &'a ProjectedFunction,
    declared: &BTreeSet<String>,
) -> Option<&'a str> {
    function
        .error
        .as_deref()
        .filter(|error| *error != "Error")
        .and_then(rust_path_owner)
        .filter(|owner| !owner_is_reachable(owner, declared))
}

fn type_undeclared_owner<'a>(
    ty: &'a ProjectedType,
    declared: &BTreeSet<String>,
) -> Option<&'a str> {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            base_rust_path,
            arguments,
            ..
        } => arguments
            .iter()
            .find_map(|argument| type_undeclared_owner(argument, declared))
            .or_else(|| {
                rust_path_owner(if base_rust_path.is_empty() {
                    rust_path
                } else {
                    base_rust_path
                })
                .filter(|owner| !owner_is_reachable(owner, declared))
            }),
        ProjectedType::Optional(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. }
        | ProjectedType::Reference { inner, .. } => type_undeclared_owner(inner, declared),
        ProjectedType::Mapping { key, value, .. } => {
            type_undeclared_owner(key, declared).or_else(|| type_undeclared_owner(value, declared))
        }
        ProjectedType::Tuple(items) => items
            .iter()
            .find_map(|item| type_undeclared_owner(item, declared)),
        _ => None,
    }
}

fn item_foreign_owners(item: &ProjectedItem) -> BTreeSet<String> {
    let mut owners = BTreeSet::new();
    if let Some(owner) = rust_path_owner(&item.rust_path) {
        owners.insert(owner.to_owned());
    }
    let functions = match &item.kind {
        ProjectedKind::Function(function) | ProjectedKind::Macro(function) => vec![function],
        ProjectedKind::ForeignType {
            methods,
            static_methods,
            ..
        }
        | ProjectedKind::Enum {
            methods,
            static_methods,
            ..
        } => methods.iter().chain(static_methods).collect(),
        ProjectedKind::Interface(interface) => interface
            .methods
            .iter()
            .map(|method| &method.function)
            .collect(),
    };
    for function in functions {
        for ty in function
            .parameters
            .iter()
            .map(|parameter| &parameter.ty)
            .chain(std::iter::once(&function.result))
        {
            collect_type_owners(ty, &mut owners);
        }
        if let Some(error) = function.error.as_deref().and_then(rust_path_owner) {
            owners.insert(error.to_owned());
        }
    }
    owners
}

fn collect_type_owners(ty: &ProjectedType, owners: &mut BTreeSet<String>) {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            arguments,
            ..
        } => {
            if let Some(owner) = rust_path_owner(rust_path) {
                owners.insert(owner.to_owned());
            }
            for argument in arguments {
                collect_type_owners(argument, owners);
            }
        }
        ProjectedType::Optional(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. }
        | ProjectedType::Reference { inner, .. } => collect_type_owners(inner, owners),
        ProjectedType::Mapping { key, value, .. } => {
            collect_type_owners(key, owners);
            collect_type_owners(value, owners);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                collect_type_owners(item, owners);
            }
        }
        _ => {}
    }
}

pub(super) fn rust_path_owner(path: &str) -> Option<&str> {
    path.trim_start_matches('<')
        .split("::")
        .next()
        .filter(|owner| {
            !owner.is_empty()
                && owner
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        })
}

fn owner_is_reachable(owner: &str, declared: &BTreeSet<String>) -> bool {
    matches!(owner, "std" | "core" | "alloc") || declared.contains(owner)
}

fn undeclared_owner_reason(owner: &str, versions: &BTreeMap<String, BTreeSet<String>>) -> String {
    let version = versions.get(owner).map_or_else(
        || "unknown".to_owned(),
        |versions| versions.iter().cloned().collect::<Vec<_>>().join(", "),
    );
    format!(
        "projected signature references unreachable crate `{}` at resolved version `{version}`; the defining trait or helper is not public through a declared dependency",
        owner.replace('_', "-")
    )
}
#[expect(
    clippy::too_many_lines,
    reason = "projected Rust root rewriting exhaustively traverses the closed type model"
)]
pub(super) fn rewrite_projected_rust_root(
    ty: &mut ProjectedType,
    package_root: &str,
    dependency_root: &str,
) {
    let rewrite = |path: &str| {
        if path == package_root {
            dependency_root.to_owned()
        } else if let Some(suffix) = path.strip_prefix(&format!("{package_root}::")) {
            format!("{dependency_root}::{suffix}")
        } else {
            path.to_owned()
        }
    };
    match ty {
        ProjectedType::Sequence { rust_path, item } => {
            rewrite_projected_rust_root(item, package_root, dependency_root);
            let constructor = rust_path
                .split_once('<')
                .map_or(rust_path.as_str(), |(constructor, _)| constructor);
            *rust_path = format!("{}<{}>", rewrite(constructor), item.rust_type());
        }
        ProjectedType::Mapping {
            rust_path,
            key,
            value,
            ..
        } => {
            rewrite_projected_rust_root(key, package_root, dependency_root);
            rewrite_projected_rust_root(value, package_root, dependency_root);
            let constructor = rust_path
                .split_once('<')
                .map_or(rust_path.as_str(), |(constructor, _)| constructor);
            *rust_path = format!(
                "{}<{}, {}>",
                rewrite(constructor),
                key.rust_type(),
                value.rust_type()
            );
        }
        ProjectedType::Set {
            rust_path, item, ..
        } => {
            rewrite_projected_rust_root(item, package_root, dependency_root);
            let constructor = rust_path
                .split_once('<')
                .map_or(rust_path.as_str(), |(constructor, _)| constructor);
            *rust_path = format!("{}<{}>", rewrite(constructor), item.rust_type());
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                rewrite_projected_rust_root(item, package_root, dependency_root);
            }
        }
        ProjectedType::AsyncIterationStep(item)
        | ProjectedType::Optional(item)
        | ProjectedType::Reference { inner: item, .. } => {
            rewrite_projected_rust_root(item, package_root, dependency_root);
        }
        ProjectedType::Foreign {
            rust_path,
            name: _,
            base_rust_path,
            arguments,
        } => {
            for argument in arguments.iter_mut() {
                rewrite_projected_rust_root(argument, package_root, dependency_root);
            }
            *base_rust_path = rewrite(base_rust_path);
            *rust_path = if arguments.is_empty() {
                base_rust_path.clone()
            } else {
                format!(
                    "{base_rust_path}<{}>",
                    arguments
                        .iter()
                        .map(ProjectedType::rust_type)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
        }
        ProjectedType::InvocationScoped {
            rust_type, owned, ..
        } => {
            rewrite_projected_rust_root(owned, package_root, dependency_root);
            *rust_type = rewrite_rust_bound_root(rust_type, package_root, dependency_root);
        }
        ProjectedType::BoxedInterface {
            rust_path,
            trait_path,
            auto_traits,
            associated_type,
            ..
        } => {
            *trait_path = rewrite(trait_path);
            for auto_trait in auto_traits.iter_mut() {
                *auto_trait = rewrite(auto_trait);
            }
            if let Some(associated) = associated_type.as_mut() {
                rewrite_projected_rust_root(&mut associated.ty, package_root, dependency_root);
            }
            let principal = associated_type.as_ref().map_or_else(
                || trait_path.clone(),
                |associated| {
                    format!(
                        "{trait_path}<{} = {}>",
                        associated.name,
                        associated.ty.rust_type()
                    )
                },
            );
            *rust_path = format!(
                "Box<dyn {}>",
                std::iter::once(principal.as_str())
                    .chain(auto_traits.iter().map(String::as_str))
                    .collect::<Vec<_>>()
                    .join(" + ")
            );
        }
        ProjectedType::Callback {
            parameters,
            result,
            native_bound,
            native_result,
            native_substitutions,
            ..
        } => {
            for parameter in parameters {
                rewrite_projected_rust_root(parameter, package_root, dependency_root);
            }
            rewrite_projected_rust_root(result, package_root, dependency_root);
            for substitution in native_substitutions.values_mut() {
                rewrite_projected_rust_root(substitution, package_root, dependency_root);
            }
            if let Some(native_bound) = native_bound {
                *native_bound =
                    rewrite_rust_bound_root(native_bound, package_root, dependency_root);
            }
            if let Some(native_result) = native_result {
                *native_result =
                    rewrite_rust_bound_root(native_result, package_root, dependency_root);
            }
        }
        ProjectedType::Opaque { bounds, .. } => {
            for bound in bounds {
                *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
            }
        }
        ProjectedType::None
        | ProjectedType::Generic(_)
        | ProjectedType::Bool
        | ProjectedType::Int
        | ProjectedType::FixedInt(_)
        | ProjectedType::RustInt(_)
        | ProjectedType::Float
        | ProjectedType::Float32
        | ProjectedType::Char
        | ProjectedType::String
        | ProjectedType::BorrowedString
        | ProjectedType::Bytes
        | ProjectedType::AsyncSinkOutcome
        | ProjectedType::Associated(_) => {}
    }
}

pub(super) fn rewrite_projected_function_root(
    function: &mut ProjectedFunction,
    package_root: &str,
    dependency_root: &str,
) {
    if let Some(owner) = &mut function.native_owner {
        *owner = rewrite_rust_bound_root(owner, package_root, dependency_root);
    }
    if let Some(path) = &mut function.native_path {
        *path = rewrite_rust_bound_root(path, package_root, dependency_root);
    }
    for parameter in &mut function.parameters {
        rewrite_projected_rust_root(&mut parameter.ty, package_root, dependency_root);
        if let Some(associated) = &mut parameter.associated_type {
            rewrite_projected_rust_root(&mut associated.ty, package_root, dependency_root);
        }
        if let Some(interface) = &mut parameter.generic_interface {
            *interface = rewrite_rust_bound_root(interface, package_root, dependency_root);
        }
        for bound in &mut parameter.generic_bounds {
            *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
        }
    }
    for generic in function
        .generic_parameters
        .iter_mut()
        .chain(&mut function.operation_owner_generics)
    {
        for bound in &mut generic.rust_bounds {
            *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
        }
    }
    for argument in &mut function.rust_generic_arguments {
        rewrite_projected_rust_root(argument, package_root, dependency_root);
    }
    rewrite_projected_rust_root(&mut function.result, package_root, dependency_root);
    if let Some(error) = &mut function.error {
        *error = rewrite_rust_bound_root(error, package_root, dependency_root);
    }
    if let Some(destination) = &mut function.destination_result {
        for parameter in &mut destination.parameters {
            for bound in &mut parameter.rust_bounds {
                *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
            }
        }
        for root in &mut destination.bound_roots {
            if root == package_root {
                root.clone_from(&dependency_root.to_owned());
            }
        }
    }
    if let Some(
        ProjectedEnumOperation::Construct {
            payload_rust_type, ..
        }
        | ProjectedEnumOperation::Extract {
            payload_rust_type, ..
        },
    ) = &mut function.enum_operation
    {
        *payload_rust_type =
            rewrite_rust_bound_root(payload_rust_type, package_root, dependency_root);
    }
}

pub(super) fn rewrite_projected_owner_root(
    projected: &mut [ProjectedDependency],
    package_root: &str,
    dependency_root: &str,
) {
    for item in projected
        .iter_mut()
        .flat_map(|dependency| &mut dependency.items)
    {
        item.rust_path = rewrite_rust_bound_root(&item.rust_path, package_root, dependency_root);
        match &mut item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                rewrite_projected_function_root(function, package_root, dependency_root);
            }
            ProjectedKind::ForeignType {
                methods,
                static_methods,
                constants,
                ..
            }
            | ProjectedKind::Enum {
                methods,
                static_methods,
                constants,
                ..
            } => {
                for function in methods.iter_mut().chain(static_methods) {
                    rewrite_projected_function_root(function, package_root, dependency_root);
                }
                for constant in constants {
                    constant.rust_path =
                        rewrite_rust_bound_root(&constant.rust_path, package_root, dependency_root);
                    rewrite_projected_rust_root(&mut constant.ty, package_root, dependency_root);
                }
            }
            ProjectedKind::Interface(interface) => {
                for method in &mut interface.methods {
                    rewrite_projected_function_root(
                        &mut method.function,
                        package_root,
                        dependency_root,
                    );
                    if let Some(owner) = &mut method.owner_rust_path {
                        *owner = rewrite_rust_bound_root(owner, package_root, dependency_root);
                    }
                }
                if let Some(associated) = &mut interface.associated_type {
                    associated.rust_path = rewrite_rust_bound_root(
                        &associated.rust_path,
                        package_root,
                        dependency_root,
                    );
                    for bound in &mut associated.bounds {
                        *bound = rewrite_rust_bound_root(bound, package_root, dependency_root);
                    }
                }
                for supertrait in &mut interface.supertraits {
                    supertrait.rust_path = rewrite_rust_bound_root(
                        &supertrait.rust_path,
                        package_root,
                        dependency_root,
                    );
                }
            }
        }
    }
}
#[cfg(test)]
mod tests;
