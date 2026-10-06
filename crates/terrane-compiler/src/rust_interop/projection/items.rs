//! Rustdoc items projection decisions.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use rustdoc_types::{
    Attribute, Crate as RustdocCrate, GenericParamDefKind, Generics, Id, Item, ItemEnum,
    ItemSummary, Type, VariantKind, Visibility,
};

use super::{
    ChainRole, DeclinedItem, PartialProjectionRecord, ProjectedDependency, ProjectedEnumOperation,
    ProjectedEnumPayloadConversion, ProjectedEnumPayloadStyle, ProjectedFunction,
    ProjectedGenericParameter, ProjectedItem, ProjectedKind, ProjectedParameter,
    ProjectedTraitOperation, ProjectedType, Receiver, RustDependency, RustdocPath,
    SourceConstantCache, borrowed_graph, callable, canonicalize_rust_path, data,
    dependency_namespace, enum_payload, extern_rust_path, implements_trait, macros,
    merge_projected_trait_operations, normalize_projected_items, owner_trait_namespace,
    partial_projection, partial_projection_references, project_boundary_capabilities,
    project_chain_owner, project_enum_payload, project_function, project_interface, project_macro,
    project_methods, project_multi_enum_payload, project_struct_fields, project_type,
    promote_async_endpoint_methods, render_generic_bound, render_rust_type, resolved_nominal_id,
    resolved_path_name, resolved_path_type_arguments, rewrite_projected_function_root,
    rewrite_projected_rust_root, rewrite_rust_bound_root, rust_lifetimes, trait_fallback_namespace,
    trait_operation_docs, type_contains_lifetime_argument,
};

pub(super) fn projected_nominal_generic_parameters(
    generics: &Generics,
    substitutions: &BTreeMap<String, ProjectedType>,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<Vec<ProjectedGenericParameter>, String> {
    generics
        .params
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .map(|parameter| {
            let rust_bounds = callable::generic_bounds_from_generics(parameter, generics)
                .iter()
                .map(|bound| render_generic_bound(bound, &[], index, paths, substitutions))
                .collect::<Result<Vec<_>, _>>()?;
            let default = match &parameter.kind {
                GenericParamDefKind::Type {
                    default: Some(default),
                    ..
                } => Some(project_type(default, index, paths, substitutions)?),
                GenericParamDefKind::Type { default: None, .. } => None,
                _ => unreachable!("only type generic parameters are retained"),
            };
            Ok(ProjectedGenericParameter {
                name: parameter.name.clone(),
                input_selected: true,
                rust_bounds,
                default,
            })
        })
        .collect()
}
fn nominal_generic_instantiation(
    generics: &Generics,
    _index: &HashMap<Id, Item>,
    _paths: &HashMap<Id, ItemSummary>,
) -> Result<BTreeMap<String, ProjectedType>, String> {
    let lifetime_only = !generics.params.is_empty()
        && generics
            .params
            .iter()
            .all(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }));
    let mut substitutions = BTreeMap::new();
    for parameter in &generics.params {
        let projected = match &parameter.kind {
            GenericParamDefKind::Type { .. } => ProjectedType::Generic(parameter.name.clone()),
            GenericParamDefKind::Lifetime { .. } if lifetime_only => continue,
            GenericParamDefKind::Lifetime { .. } => {
                return Err(format!(
                    "lifetime parameter `{}` cannot be mixed with projected type parameters",
                    parameter.name
                ));
            }
            GenericParamDefKind::Const { .. } => {
                return Err(format!(
                    "const parameter `{}` has no projected value identity",
                    parameter.name
                ));
            }
        };
        substitutions.insert(parameter.name.clone(), projected);
    }
    Ok(substitutions)
}
#[expect(
    clippy::too_many_lines,
    reason = "one rustdoc item pass records admitted and declined public items together"
)]
pub(super) fn project_rustdoc(
    dependency: &RustDependency,
    document: &RustdocCrate,
    public_paths: &BTreeMap<Id, String>,
    canonical_public_paths: &BTreeMap<String, String>,
    include_canonical_items: bool,
) -> ProjectedDependency {
    let index = &document.index;
    let original_paths = &document.paths;
    let mut canonical_paths = original_paths.clone();
    for summary in canonical_paths.values_mut() {
        if let Some(public_path) = canonical_public_paths.get(&summary.path.join("::")) {
            summary.path = public_path.split("::").map(str::to_owned).collect();
        }
    }
    let paths = &canonical_paths;
    let mut items = Vec::new();
    let mut declined = Vec::new();
    let mut candidates = BTreeMap::<Id, Vec<String>>::new();
    if include_canonical_items {
        for (id, summary) in paths.iter().filter(|(_, summary)| summary.crate_id == 0) {
            let path = summary.path.join("::");
            // `Drop` has Terrane's dedicated consuming-destruct protocol. Its public
            // reexports must remain declined just like a directly named import.
            if matches!(
                path.as_str(),
                "core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop"
            ) {
                declined.push(DeclinedItem {
                    rust_path: extern_rust_path(dependency, &path),
                    reason: "canonical Rust `Drop` is declared with Terrane `consuming destruct`"
                        .to_owned(),
                });
                continue;
            }
            candidates.insert(*id, summary.path.clone());
        }
    }
    for (id, public_path) in public_paths {
        let canonical_path = original_paths
            .get(id)
            .map(|summary| summary.path.join("::"));
        if canonical_path.as_deref().is_some_and(|path| {
            matches!(
                path,
                "core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop"
            )
        }) {
            let rust_path = extern_rust_path(dependency, public_path);
            if !declined
                .iter()
                .any(|declined| declined.rust_path == rust_path)
            {
                declined.push(DeclinedItem {
                    rust_path,
                    reason: "canonical Rust `Drop` is declared with Terrane `consuming destruct`"
                        .to_owned(),
                });
            }
            continue;
        }
        candidates.insert(*id, public_path.split("::").map(str::to_owned).collect());
    }
    let mut partial_declines = Vec::new();
    let mut projected_trait_items = Vec::new();
    let mut projected_associated_items = Vec::new();
    let mut source_constants = SourceConstantCache::new();
    let mut enum_payload_items = Vec::new();
    for (id, path) in candidates {
        let Some(item) = index.get(&id) else {
            if original_paths
                .get(&id)
                .map(|summary| summary.path.join("::"))
                .as_deref()
                .is_some_and(|path| {
                    matches!(
                        path,
                        "core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop"
                    )
                })
            {
                declined.push(DeclinedItem {
                    rust_path: extern_rust_path(dependency, &path.join("::")),
                    reason: "canonical Rust `Drop` is declared with Terrane `consuming destruct`"
                        .to_owned(),
                });
            }
            continue;
        };
        if item.visibility != Visibility::Public {
            continue;
        }
        let Some(name) = path.last().cloned() else {
            continue;
        };
        let mut namespace = dependency_namespace(dependency, &path[..path.len().saturating_sub(1)]);
        let public_rust_path = public_paths
            .get(&id)
            .cloned()
            .unwrap_or_else(|| path.join("::"));
        let canonical_rust_path = original_paths
            .get(&id)
            .map_or_else(|| path.join("::"), |summary| summary.path.join("::"));
        let mut rust_path = if canonical_rust_path.starts_with("core::")
            || canonical_rust_path.starts_with("alloc::")
        {
            canonicalize_rust_path(&canonical_rust_path)
        } else {
            extern_rust_path(dependency, &public_rust_path)
        };
        let docs = item.docs.clone();
        if matches!(item.inner, ItemEnum::Module(_) | ItemEnum::Use(_)) {
            continue;
        }
        let projected = match &item.inner {
            ItemEnum::Function(function) => project_function(
                function,
                index,
                paths,
                public_paths,
                Some(&name),
                true,
            )
            .and_then(|mut projected_function| {
                let candidate_result = match &projected_function.result {
                    ProjectedType::InvocationScoped { owned, .. } => owned.as_ref().clone(),
                    result => result.clone(),
                };
                let mut chain_owner = project_chain_owner(
                    dependency,
                    function,
                    &candidate_result,
                    false,
                    index,
                    paths,
                    public_paths,
                    &mut source_constants,
                );
                let has_external_terminal_conversion =
                    function.sig.output.as_ref().is_some_and(|output| {
                        let Some(output_id) = resolved_nominal_id(output, index) else {
                            return false;
                        };
                        let Some(output_item) = index.get(&output_id) else {
                            return false;
                        };
                        let implementations = match &output_item.inner {
                            ItemEnum::Struct(output) => &output.impls,
                            ItemEnum::Enum(output) => &output.impls,
                            ItemEnum::Union(output) => &output.impls,
                            _ => return false,
                        };
                        let explicit_into = implementations
                            .iter()
                            .filter_map(|implementation| index.get(implementation))
                            .any(|implementation| {
                                let ItemEnum::Impl(implementation) = &implementation.inner else {
                                    return false;
                                };
                                implementation.blanket_impl.is_none()
                                    && implementation.trait_.as_ref().is_some_and(|interface| {
                                        let name = resolved_path_name(interface, paths);
                                        (name.ends_with("::Into") || name == "Into")
                                            && resolved_path_type_arguments(interface)
                                                .first()
                                                .is_some_and(|destination| {
                                                    !matches!(destination, Type::Generic(_))
                                                })
                                    })
                            });
                        explicit_into
                            || index.values().any(|item| {
                                let ItemEnum::Impl(implementation) = &item.inner else {
                                    return false;
                                };
                                implementation.blanket_impl.is_none()
                                    && implementation.trait_.as_ref().is_some_and(|interface| {
                                        let name = resolved_path_name(interface, paths);
                                        (name.ends_with("::From") || name == "From")
                                            && resolved_path_type_arguments(interface)
                                                .first()
                                                .is_some_and(|source| {
                                                    matches!(
                                                        source,
                                                        Type::ResolvedPath(source)
                                                            if source.id == output_id
                                                    )
                                                })
                                    })
                            })
                    });
                if has_external_terminal_conversion {
                    chain_owner = None;
                }
                let ordinary_chain_owner = chain_owner.is_some();
                let receiver_tied =
                    projected_function
                        .parameters
                        .first()
                        .is_some_and(|parameter| {
                            parameter.borrowed
                                && matches!(parameter.ty, ProjectedType::Foreign { .. })
                        });
                let lifetime_bearing = function
                    .sig
                    .output
                    .as_ref()
                    .is_some_and(type_contains_lifetime_argument);
                if lifetime_bearing
                    && has_external_terminal_conversion
                    && !receiver_tied
                    && chain_owner.is_none()
                {
                    let output_generics = function
                        .generics
                        .params
                        .iter()
                        .filter(|parameter| {
                            matches!(parameter.kind, GenericParamDefKind::Type { .. })
                        })
                        .map(|parameter| {
                            (
                                parameter.name.clone(),
                                ProjectedType::Generic(parameter.name.clone()),
                            )
                        })
                        .collect();
                    let rust_type = function
                        .sig
                        .output
                        .as_ref()
                        .and_then(|output| {
                            render_rust_type(output, index, paths, &output_generics).ok()
                        })
                        .unwrap_or_else(|| candidate_result.rust_type());
                    let lifetimes = rust_lifetimes(&rust_type);
                    if lifetimes.is_empty() {
                        return Err(
                            "lifetime-bearing foreign type cannot cross a projected boundary"
                                .to_owned(),
                        );
                    }
                    projected_function.result = ProjectedType::InvocationScoped {
                        rust_type,
                        name: candidate_result.terrane_name(),
                        lifetimes,
                        expression_scoped: false,
                        owned: Box::new(candidate_result.clone()),
                    };
                }
                let invocation_scoped_chain = matches!(
                    projected_function.result,
                    ProjectedType::InvocationScoped { .. }
                );
                if chain_owner.is_none() && invocation_scoped_chain {
                    chain_owner = project_chain_owner(
                        dependency,
                        function,
                        &candidate_result,
                        true,
                        index,
                        paths,
                        public_paths,
                        &mut source_constants,
                    );
                }
                if let Some(chain_owner) = chain_owner {
                    if ordinary_chain_owner {
                        projected_function.result = candidate_result;
                        projected_function.chain_role = Some(ChainRole::Root);
                    }
                    projected_associated_items.push(chain_owner);
                } else if matches!(
                    projected_function.result,
                    ProjectedType::Opaque {
                        anonymous_chain: true,
                        ..
                    }
                ) {
                    projected_function.chain_role = Some(ChainRole::Root);
                }
                Ok(ProjectedKind::Function(projected_function))
            }),
            ItemEnum::TypeAlias(alias) => (|| {
                let mut alias_generics = BTreeMap::new();
                for parameter in &alias.generics.params {
                    let projected = match &parameter.kind {
                        GenericParamDefKind::Type { .. } => {
                            ProjectedType::Generic(parameter.name.clone())
                        }
                        GenericParamDefKind::Lifetime { .. } => {
                            return Err(format!(
                                "lifetime parameter `{}` requires non-escaping chain projection",
                                parameter.name
                            ));
                        }
                        GenericParamDefKind::Const { .. } => {
                            return Err(format!(
                                "const parameter `{}` has no projected value identity",
                                parameter.name
                            ));
                        }
                    };
                    alias_generics.insert(parameter.name.clone(), projected);
                }
                Ok(ProjectedKind::ForeignType {
                    constructor: None,
                    methods: Vec::new(),
                    static_methods: Vec::new(),
                    constants: Vec::new(),
                    boundary: project_boundary_capabilities(&alias.type_, index, paths),
                    fields: Vec::new(),
                    borrowed_view: false,
                    native_view_type: None,
                    enum_payload: None,
                    generic_parameters: projected_nominal_generic_parameters(
                        &alias.generics,
                        &alias_generics,
                        index,
                        paths,
                    )?,
                    displayable: false,
                    cloneable: false,
                    send: false,
                    sync: false,
                })
            })(),
            ItemEnum::Struct(structure) => {
                let mut owner_generics =
                    match nominal_generic_instantiation(&structure.generics, index, paths) {
                        Ok(generics) => generics,
                        Err(reason) => {
                            declined.push(DeclinedItem {
                                rust_path: rust_path.clone(),
                                reason,
                            });
                            continue;
                        }
                    };
                let has_lifetime = structure.generics.params.iter().any(|parameter| {
                    matches!(parameter.kind, GenericParamDefKind::Lifetime { .. })
                });
                let field_projection = if item
                    .attrs
                    .iter()
                    .any(|attribute| matches!(attribute, rustdoc_types::Attribute::NonExhaustive))
                {
                    Err("non-exhaustive struct cannot be constructed outside its crate".to_owned())
                } else {
                    project_struct_fields(structure, index, paths, &owner_generics)
                };
                let (fields, borrowed_view) = match field_projection {
                    Ok(projected) => projected,
                    Err(reason) if has_lifetime => {
                        declined.push(DeclinedItem {
                            rust_path: rust_path.clone(),
                            reason,
                        });
                        continue;
                    }
                    Err(_) => (Vec::new(), false),
                };
                let native_view_type = borrowed_view.then(|| {
                    let native_arguments = structure
                        .generics
                        .params
                        .iter()
                        .filter_map(|parameter| match parameter.kind {
                            GenericParamDefKind::Lifetime { .. } => Some("'_".to_owned()),
                            GenericParamDefKind::Type { .. } => owner_generics
                                .get(&parameter.name)
                                .map(ProjectedType::rust_type),
                            GenericParamDefKind::Const { .. } => None,
                        })
                        .collect::<Vec<_>>();
                    format!("{rust_path}<{}>", native_arguments.join(", "))
                });
                let base_rust_path = rust_path.clone();
                let arguments = structure
                    .generics
                    .params
                    .iter()
                    .filter_map(|parameter| owner_generics.get(&parameter.name).cloned())
                    .collect::<Vec<_>>();
                if !arguments.is_empty() {
                    rust_path = format!(
                        "{base_rust_path}<{}>",
                        arguments
                            .iter()
                            .map(ProjectedType::rust_type)
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
                owner_generics.insert(
                    "Self".to_owned(),
                    ProjectedType::Foreign {
                        rust_path: rust_path.clone(),
                        name: name.clone(),
                        base_rust_path,
                        arguments,
                    },
                );
                let constructor = if owner_generics
                    .values()
                    .any(|ty| matches!(ty, ProjectedType::Generic(_)))
                {
                    if item
                        .attrs
                        .contains(&rustdoc_types::Attribute::NonExhaustive)
                    {
                        declined.push(DeclinedItem {
                            rust_path: rust_path.clone(),
                            reason: "non-exhaustive native structs cannot be constructed outside their defining crate".to_owned(),
                        });
                        None
                    } else {
                        match data::project_struct_constructor(
                            structure,
                            &fields,
                            index,
                            paths,
                            public_paths,
                            &owner_generics,
                        ) {
                            Ok(constructor) => Some(constructor),
                            Err(reason) => {
                                declined.push(DeclinedItem {
                                    rust_path: rust_path.clone(),
                                    reason,
                                });
                                None
                            }
                        }
                    }
                } else {
                    None
                };
                let generic_parameters = match projected_nominal_generic_parameters(
                    &structure.generics,
                    &owner_generics,
                    index,
                    paths,
                ) {
                    Ok(parameters) => parameters,
                    Err(reason) => {
                        declined.push(DeclinedItem {
                            rust_path: rust_path.clone(),
                            reason,
                        });
                        continue;
                    }
                };
                {
                    let projected_impls = if borrowed_view {
                        &[][..]
                    } else {
                        structure.impls.as_slice()
                    };
                    let (projected_methods, trait_methods, projected_constants, method_declines) =
                        project_methods(
                            projected_impls,
                            index,
                            paths,
                            public_paths,
                            &rust_path,
                            &owner_generics,
                            false,
                            &mut source_constants,
                        );
                    let (mut methods, mut static_methods): (Vec<_>, Vec<_>) = projected_methods
                        .into_iter()
                        .partition(|method| method.receiver.is_some());
                    promote_async_endpoint_methods(&mut methods);
                    promote_async_endpoint_methods(&mut static_methods);
                    let owner_namespace = owner_trait_namespace(&namespace, &name);
                    for (trait_implementation_path, public_trait_path, local_trait, docs, method) in
                        trait_methods
                    {
                        let trait_rust_path = if local_trait {
                            extern_rust_path(dependency, &public_trait_path)
                        } else {
                            public_trait_path.replace('-', "_")
                        };
                        let method_rust_path =
                            format!("<{rust_path} as {trait_rust_path}>::{}", method.name);
                        projected_trait_items.push(ProjectedTraitOperation {
                            fallback_namespace: trait_fallback_namespace(
                                &owner_namespace,
                                &trait_implementation_path,
                            ),
                            item: ProjectedItem {
                                namespace: owner_namespace.clone(),
                                name: method.name.clone(),
                                rust_path: method_rust_path.clone(),
                                docs: Some(trait_operation_docs(
                                    &method_rust_path,
                                    docs.as_deref(),
                                )),
                                kind: ProjectedKind::Function(method),
                            },
                        });
                    }
                    declined.extend(method_declines.into_iter().map(|(name, reason)| {
                        DeclinedItem {
                            rust_path: format!("{rust_path}::{name}"),
                            reason,
                        }
                    }));
                    Ok(ProjectedKind::ForeignType {
                        constructor,
                        methods,
                        fields,
                        borrowed_view,
                        native_view_type,
                        enum_payload: None,
                        static_methods,
                        constants: projected_constants,
                        boundary: project_boundary_capabilities(
                            &Type::ResolvedPath(RustdocPath {
                                path: rust_path.clone(),
                                id,
                                args: None,
                            }),
                            index,
                            paths,
                        ),
                        displayable: implements_trait(
                            &structure.impls,
                            index,
                            paths,
                            "core::fmt::Display",
                        ),
                        cloneable: !structure.generics.params.iter().any(|parameter| {
                            matches!(parameter.kind, GenericParamDefKind::Type { .. })
                        }) && implements_trait(
                            &structure.impls,
                            index,
                            paths,
                            "core::clone::Clone",
                        ),
                        send: false,
                        sync: false,
                        generic_parameters,
                    })
                }
            }
            ItemEnum::Enum(enumeration) => {
                let unsupported_parameter =
                    enumeration.generics.params.iter().find(|parameter| {
                        !matches!(parameter.kind, GenericParamDefKind::Type { .. })
                    });
                if let Some(parameter) = unsupported_parameter {
                    Err(format!(
                        "enum generic parameter `{}` is not a type parameter",
                        parameter.name
                    ))
                } else {
                    let nominal_generic_types =
                        match nominal_generic_instantiation(&enumeration.generics, index, paths) {
                            Ok(substitutions) => substitutions,
                            Err(reason) => {
                                declined.push(DeclinedItem {
                                    rust_path: rust_path.clone(),
                                    reason,
                                });
                                continue;
                            }
                        };
                    let generic_parameters = match projected_nominal_generic_parameters(
                        &enumeration.generics,
                        &nominal_generic_types,
                        index,
                        paths,
                    ) {
                        Ok(parameters) => parameters,
                        Err(reason) => {
                            declined.push(DeclinedItem {
                                rust_path: rust_path.clone(),
                                reason,
                            });
                            continue;
                        }
                    };
                    let generic_arguments = enumeration
                        .generics
                        .params
                        .iter()
                        .filter_map(|parameter| nominal_generic_types.get(&parameter.name).cloned())
                        .collect::<Vec<_>>();
                    let generic_rust_path = if generic_arguments.is_empty() {
                        rust_path.clone()
                    } else {
                        format!(
                            "{rust_path}<{}>",
                            generic_arguments
                                .iter()
                                .map(ProjectedType::rust_type)
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    };
                    let mut owner_generics = BTreeMap::from([(
                        "Self".to_owned(),
                        ProjectedType::Foreign {
                            rust_path: generic_rust_path,
                            name: name.clone(),
                            base_rust_path: rust_path.clone(),
                            arguments: generic_arguments,
                        },
                    )]);
                    owner_generics.extend(nominal_generic_types);
                    let enum_type = owner_generics["Self"].clone();
                    let (projected_methods, trait_methods, projected_constants, method_declines) =
                        project_methods(
                            &enumeration.impls,
                            index,
                            paths,
                            public_paths,
                            &rust_path,
                            &owner_generics,
                            false,
                            &mut source_constants,
                        );
                    let (mut methods, mut static_methods): (Vec<_>, Vec<_>) = projected_methods
                        .into_iter()
                        .partition(|method| method.receiver.is_some());
                    promote_async_endpoint_methods(&mut methods);
                    promote_async_endpoint_methods(&mut static_methods);
                    let owner_namespace = owner_trait_namespace(&namespace, &name);
                    for (trait_implementation_path, public_trait_path, local_trait, docs, method) in
                        trait_methods
                    {
                        let trait_rust_path = if local_trait {
                            extern_rust_path(dependency, &public_trait_path)
                        } else {
                            public_trait_path.replace('-', "_")
                        };
                        let method_rust_path =
                            format!("<{rust_path} as {trait_rust_path}>::{}", method.name);
                        projected_trait_items.push(ProjectedTraitOperation {
                            fallback_namespace: trait_fallback_namespace(
                                &owner_namespace,
                                &trait_implementation_path,
                            ),
                            item: ProjectedItem {
                                namespace: owner_namespace.clone(),
                                name: method.name.clone(),
                                rust_path: method_rust_path.clone(),
                                docs: Some(trait_operation_docs(
                                    &method_rust_path,
                                    docs.as_deref(),
                                )),
                                kind: ProjectedKind::Function(method),
                            },
                        });
                    }
                    declined.extend(method_declines.into_iter().map(|(name, reason)| {
                        DeclinedItem {
                            rust_path: format!("{rust_path}::{name}"),
                            reason,
                        }
                    }));
                    let mut variant_names = Vec::new();
                    let mut variants = Vec::new();
                    let mut data_carrying = false;
                    for variant_id in &enumeration.variants {
                        let Some(variant_item) = index.get(variant_id) else {
                            continue;
                        };
                        let Some(variant_name) = variant_item.name.as_deref() else {
                            continue;
                        };
                        let ItemEnum::Variant(variant) = &variant_item.inner else {
                            continue;
                        };
                        variant_names.push(variant_name.to_owned());
                        variants.push(enum_payload::project_variant(
                            variant_name,
                            variant,
                            index,
                            paths,
                            &owner_generics,
                        ));
                        let projected_payload = match &variant.kind {
                            VariantKind::Plain => None,
                            VariantKind::Tuple(fields) if fields.len() == 1 => {
                                data_carrying = true;
                                let Some(field_id) = fields[0].as_ref() else {
                                    declined.push(DeclinedItem {
                                        rust_path: format!("{rust_path}::{variant_name}"),
                                        reason: "payload enum variant field is stripped".to_owned(),
                                    });
                                    continue;
                                };
                                let Some(Item {
                                    inner: ItemEnum::StructField(field_type),
                                    ..
                                }) = index.get(field_id)
                                else {
                                    declined.push(DeclinedItem {
                                        rust_path: format!("{rust_path}::{variant_name}"),
                                        reason:
                                            "payload enum variant field metadata is unavailable"
                                                .to_owned(),
                                    });
                                    continue;
                                };
                                match project_enum_payload(
                                    field_type,
                                    index,
                                    paths,
                                    &owner_generics,
                                ) {
                                    Ok((
                                        constructor,
                                        constructor_conversion,
                                        extraction,
                                        extraction_conversion,
                                        rust_type,
                                    )) => Some((
                                        constructor,
                                        constructor_conversion,
                                        extraction,
                                        extraction_conversion,
                                        rust_type,
                                        None,
                                    )),
                                    Err(reason) => {
                                        declined.push(DeclinedItem {
                                            rust_path: format!("{rust_path}::{variant_name}"),
                                            reason,
                                        });
                                        continue;
                                    }
                                }
                            }
                            VariantKind::Tuple(fields) => {
                                data_carrying = true;
                                match project_multi_enum_payload(
                                    &namespace,
                                    &name,
                                    &rust_path,
                                    variant_name,
                                    variant_item.docs.clone(),
                                    ProjectedEnumPayloadStyle::Tuple,
                                    fields.iter().enumerate().map(|(index, field)| {
                                        (format!("item-n{index}"), index.to_string(), *field)
                                    }),
                                    index,
                                    paths,
                                    &owner_generics,
                                ) {
                                    Ok((item, payload)) => {
                                        enum_payload_items.push(item);
                                        Some(payload)
                                    }
                                    Err(reason) => {
                                        declined.push(DeclinedItem {
                                            rust_path: format!("{rust_path}::{variant_name}"),
                                            reason,
                                        });
                                        continue;
                                    }
                                }
                            }
                            VariantKind::Struct {
                                fields,
                                has_stripped_fields,
                            } => {
                                data_carrying = true;
                                if *has_stripped_fields {
                                    declined.push(DeclinedItem {
                                        rust_path: format!("{rust_path}::{variant_name}"),
                                        reason: "payload enum variant has stripped named fields"
                                            .to_owned(),
                                    });
                                    continue;
                                }
                                match project_multi_enum_payload(
                                    &namespace,
                                    &name,
                                    &rust_path,
                                    variant_name,
                                    variant_item.docs.clone(),
                                    ProjectedEnumPayloadStyle::Struct,
                                    fields.iter().map(|field| {
                                        let field_name = index
                                            .get(field)
                                            .and_then(|item| item.name.clone())
                                            .unwrap_or_default();
                                        (field_name.clone(), field_name, Some(*field))
                                    }),
                                    index,
                                    paths,
                                    &owner_generics,
                                ) {
                                    Ok((item, payload)) => {
                                        enum_payload_items.push(item);
                                        Some(payload)
                                    }
                                    Err(reason) => {
                                        declined.push(DeclinedItem {
                                            rust_path: format!("{rust_path}::{variant_name}"),
                                            reason,
                                        });
                                        continue;
                                    }
                                }
                            }
                        };
                        let (
                            constructor_type,
                            constructor_conversion,
                            extraction_type,
                            extraction_conversion,
                            payload_rust_type,
                            payload,
                        ) = projected_payload.unwrap_or_else(|| {
                            (
                                ProjectedType::None,
                                ProjectedEnumPayloadConversion::Identity,
                                ProjectedType::None,
                                ProjectedEnumPayloadConversion::Identity,
                                String::new(),
                                None,
                            )
                        });
                        static_methods.push(ProjectedFunction {
                            native_path: None,
                            native_owner: None,
                            operation_owner_generics: Vec::new(),
                            name: variant_name.to_owned(),
                            parameters: (constructor_type != ProjectedType::None)
                                .then(|| ProjectedParameter {
                                    name: "value".to_owned(),
                                    ty: constructor_type,
                                    borrowed: false,
                                    mutable_borrow: false,
                                    generic_parameter: None,
                                    generic_bounds: Vec::new(),
                                    generic_interface: None,
                                    associated_type: None,
                                })
                                .into_iter()
                                .collect(),
                            generic_parameters: Vec::new(),
                            rust_generic_arguments: Vec::new(),
                            result: enum_type.clone(),
                            destination_result: None,
                            error: None,
                            is_async: false,
                            is_unsafe: false,
                            into_future: false,
                            execution_requirements: None,
                            enum_operation: Some(ProjectedEnumOperation::Construct {
                                variant: variant_name.to_owned(),
                                unit: matches!(variant.kind, VariantKind::Plain),
                                conversion: constructor_conversion,
                                payload_rust_type: payload_rust_type.clone(),
                                payload: payload.clone(),
                            }),
                            error_optional_depth: 0,
                            chain_role: None,
                            receiver: None,
                        });
                        if extraction_type != ProjectedType::None
                            && !matches!(extraction_type, ProjectedType::Optional(_))
                        {
                            methods.push(ProjectedFunction {
                                generic_parameters: Vec::new(),
                                native_owner: None,
                                native_path: None,
                                operation_owner_generics: Vec::new(),
                                name: format!("into-{variant_name}"),
                                parameters: Vec::new(),
                                rust_generic_arguments: Vec::new(),
                                result: ProjectedType::Optional(Box::new(extraction_type)),
                                destination_result: None,
                                error: None,
                                is_async: false,
                                is_unsafe: false,
                                into_future: false,
                                execution_requirements: None,
                                enum_operation: Some(ProjectedEnumOperation::Extract {
                                    variant: variant_name.to_owned(),
                                    conversion: extraction_conversion,
                                    payload_rust_type,
                                    payload,
                                }),
                                error_optional_depth: 0,
                                chain_role: None,
                                receiver: Some(Receiver::Move),
                            });
                        }
                    }
                    if data_carrying {
                        methods.push(ProjectedFunction {
                            generic_parameters: Vec::new(),
                            native_owner: None,
                            native_path: None,
                            operation_owner_generics: Vec::new(),
                            name: "variant-name".to_owned(),
                            parameters: Vec::new(),
                            rust_generic_arguments: Vec::new(),
                            result: ProjectedType::String,
                            destination_result: None,
                            error: None,
                            is_async: false,
                            is_unsafe: false,
                            into_future: false,
                            execution_requirements: None,
                            enum_operation: Some(ProjectedEnumOperation::VariantName {
                                variants: variant_names,
                                exhaustive: !enumeration.has_stripped_variants
                                    && !item.attrs.iter().any(|attribute| {
                                        matches!(attribute, Attribute::NonExhaustive)
                                    }),
                            }),
                            error_optional_depth: 0,
                            chain_role: None,
                            receiver: Some(Receiver::Borrow),
                        });
                    }
                    Ok(ProjectedKind::Enum {
                        variants,
                        exhaustive: !enumeration.has_stripped_variants
                            && !item
                                .attrs
                                .iter()
                                .any(|attribute| matches!(attribute, Attribute::NonExhaustive)),
                        methods,
                        static_methods,
                        constants: projected_constants,
                        send: false,
                        sync: false,
                        displayable: implements_trait(
                            &enumeration.impls,
                            index,
                            paths,
                            "core::fmt::Display",
                        ),
                        data_carrying,
                        comparable: implements_trait(
                            &enumeration.impls,
                            index,
                            paths,
                            "core::cmp::PartialEq",
                        ),
                        generic_parameters,
                    })
                }
            }
            ItemEnum::Trait(declaration) => {
                if paths
                    .get(&id)
                    .map(|summary| summary.path.join("::"))
                    .as_deref()
                    .is_some_and(|path| {
                        matches!(
                            path,
                            "core::ops::Drop" | "core::ops::drop::Drop" | "std::ops::Drop"
                        )
                    })
                {
                    Err(
                        "canonical Rust `Drop` is declared with Terrane `consuming destruct`"
                            .to_owned(),
                    )
                } else {
                    project_interface(declaration, index, paths, &rust_path)
                        .map(ProjectedKind::Interface)
                }
            }
            ItemEnum::Macro(_) => {
                namespace.push_str("/macros");
                Ok(ProjectedKind::Macro(project_macro(&name)))
            }
            _ => Err("item kind has no Terrane projection".to_owned()),
        };
        match projected {
            Ok(kind) => {
                let docs = if matches!(
                    &kind,
                    ProjectedKind::Function(function) if function.chain_role == Some(ChainRole::Root)
                ) {
                    Some(match docs {
                        Some(docs) => format!(
                            "{docs}\n\nChain-only: this value must terminate within one expression."
                        ),
                        None => "Chain-only: this value must terminate within one expression."
                            .to_owned(),
                    })
                } else {
                    docs
                };
                items.push(ProjectedItem {
                    namespace,
                    name,
                    rust_path,
                    docs,
                    kind,
                });
            }
            Err(reason) => {
                if let Some(projection) = partial_projection(item, index, paths) {
                    let references =
                        partial_projection_references(dependency, &projection, paths, public_paths);
                    partial_declines.push(PartialProjectionRecord {
                        rust_path: rust_path.clone(),
                        reason: reason.clone(),
                        references,
                        projection,
                    });
                }
                declined.push(DeclinedItem { rust_path, reason });
            }
        }
    }
    items.extend(enum_payload_items);
    if include_canonical_items {
        items.extend(macros::macro_reexports(dependency, document));
    }
    borrowed_graph::add_optional_owners(&mut items);
    projected_associated_items.sort_by(|left, right| {
        (&left.namespace, &left.name, &left.rust_path).cmp(&(
            &right.namespace,
            &right.name,
            &right.rust_path,
        ))
    });
    let mut associated_index = 0;
    while associated_index < projected_associated_items.len() {
        let first = associated_index;
        let key = (
            projected_associated_items[first].namespace.clone(),
            projected_associated_items[first].name.clone(),
        );
        while associated_index < projected_associated_items.len()
            && projected_associated_items[associated_index].namespace == key.0
            && projected_associated_items[associated_index].name == key.1
        {
            associated_index += 1;
        }
        if associated_index - first == 1
            && !items
                .iter()
                .any(|item| item.namespace == key.0 && item.name == key.1)
        {
            items.push(projected_associated_items[first].clone());
        } else {
            declined.extend(
                projected_associated_items[first..associated_index]
                    .iter()
                    .map(|item| DeclinedItem {
                        rust_path: item.rust_path.clone(),
                        reason: "multiple receiver-free associated functions with the same projected name"
                            .to_owned(),
                    }),
            );
        }
    }
    merge_projected_trait_operations(&mut items, &mut declined, projected_trait_items);
    let projected_interfaces = items
        .iter()
        .filter(|item| matches!(item.kind, ProjectedKind::Interface(_)))
        .map(|item| item.rust_path.clone())
        .collect::<BTreeSet<_>>();
    let mut retained_items = Vec::with_capacity(items.len());
    for item in items {
        let declined_bound = match &item.kind {
            ProjectedKind::Function(function) => function
                .parameters
                .iter()
                .filter_map(|parameter| parameter.generic_interface.as_ref())
                .find(|bound| !projected_interfaces.contains(*bound))
                .cloned(),
            _ => None,
        };
        if let Some(bound) = declined_bound {
            declined.push(DeclinedItem {
                rust_path: item.rust_path,
                reason: format!("generic input references declined interface `{bound}`"),
            });
        } else {
            retained_items.push(item);
        }
    }
    let mut items = retained_items;
    let package_root = dependency.package.replace('-', "_");
    let dependency_root = dependency.name.replace('-', "_");
    let normalize = |function: &mut ProjectedFunction| {
        rewrite_projected_function_root(function, &package_root, &dependency_root);
    };
    let interface_identities = items
        .iter()
        .filter(|item| matches!(item.kind, ProjectedKind::Interface(_)))
        .map(|item| {
            (
                item.rust_path.clone(),
                (item.namespace.clone(), item.name.clone()),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for item in &mut items {
        match &mut item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                normalize(function);
            }
            ProjectedKind::ForeignType {
                fields,
                methods,
                static_methods,
                native_view_type,
                ..
            } => {
                if let Some(native_view_type) = native_view_type {
                    *native_view_type =
                        rewrite_rust_bound_root(native_view_type, &package_root, &dependency_root);
                }
                for field in fields {
                    rewrite_projected_rust_root(&mut field.ty, &package_root, &dependency_root);
                    field.rust_type =
                        rewrite_rust_bound_root(&field.rust_type, &package_root, &dependency_root);
                }
                for method in methods.iter_mut().chain(static_methods) {
                    normalize(method);
                }
            }
            ProjectedKind::Enum {
                methods,
                static_methods,
                ..
            } => {
                for method in methods.iter_mut().chain(static_methods) {
                    normalize(method);
                }
            }
            ProjectedKind::Interface(interface) => {
                for method in &mut interface.methods {
                    normalize(&mut method.function);
                    if let Some(owner) = &mut method.owner_rust_path {
                        *owner = rewrite_rust_bound_root(owner, &package_root, &dependency_root);
                    }
                }
                if let Some(associated) = &mut interface.associated_type {
                    associated.rust_path = rewrite_rust_bound_root(
                        &associated.rust_path,
                        &package_root,
                        &dependency_root,
                    );
                    for bound in &mut associated.bounds {
                        *bound = rewrite_rust_bound_root(bound, &package_root, &dependency_root);
                    }
                }
                for supertrait in &mut interface.supertraits {
                    if let Some((namespace, name)) = interface_identities.get(&supertrait.rust_path)
                    {
                        supertrait.namespace.clone_from(namespace);
                        supertrait.name.clone_from(name);
                    }
                    supertrait.rust_path = rewrite_rust_bound_root(
                        &supertrait.rust_path,
                        &package_root,
                        &dependency_root,
                    );
                }
            }
        }
    }
    normalize_projected_items(&mut items, &mut declined);
    ProjectedDependency {
        name: dependency.name.clone(),
        partial_declines,
        native_alias_identities: BTreeMap::new(),
        package: dependency.package.clone(),
        version: document
            .crate_version
            .clone()
            .unwrap_or_else(|| dependency.version.clone()),
        items,
        declined,
    }
}
