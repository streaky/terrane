use super::{
    BTreeMap, BTreeSet, ChainRole, DeclinedItem, InvocationMode, ProjectedBoundaryCapabilities,
    ProjectedDestinationParameter, ProjectedDestinationResult, ProjectedFunction,
    ProjectedGenericParameter, ProjectedItem, ProjectedKind, ProjectedParameter, ProjectedType,
    Receiver, source_rendering,
};

pub(super) fn add_owners(items: &mut Vec<ProjectedItem>) {
    add_slice_owners(items);
    add_optional_sum_owners(items);
    let namespaces = items
        .iter()
        .filter(|item| {
            source_rendering::projected_item_functions(item)
                .iter()
                .any(|function| {
                    matches!(&function.result, ProjectedType::InvocationScoped {
                owned, expression_scoped: true, ..
            } if matches!(owned.as_ref(), ProjectedType::Optional(_)))
                })
        })
        .map(|item| item.namespace.clone())
        .collect::<BTreeSet<_>>();
    for namespace in namespaces {
        if !items
            .iter()
            .any(|item| item.namespace == namespace && item.name == "borrowed-option")
        {
            items.push(optional_owner(namespace));
        }
    }
}

fn add_slice_owners(items: &mut Vec<ProjectedItem>) {
    let mut slices = BTreeMap::new();
    for owner in items.iter() {
        for function in source_rendering::projected_item_functions(owner) {
            let ProjectedType::InvocationScoped {
                owned,
                expression_scoped: true,
                ..
            } = &function.result
            else {
                continue;
            };
            let ProjectedType::Foreign {
                name,
                rust_path,
                arguments,
                ..
            } = owned.as_ref()
            else {
                continue;
            };
            let [element] = arguments.as_slice() else {
                continue;
            };
            if name.starts_with("borrowed-slice-of-") && rust_path.starts_with('[') {
                slices
                    .entry((&owner.namespace, name))
                    .or_insert((rust_path, element));
            }
        }
    }
    let additions = slices
        .into_iter()
        .filter_map(|((namespace, name), (rust_path, item))| {
            if items
                .iter()
                .any(|existing| existing.namespace == *namespace && existing.name == *name)
            {
                return None;
            }
            let output = ProjectedType::Generic("U".to_owned());
            let mut method = terminal_method(
                "map_collect",
                ProjectedType::Sequence {
                    rust_path: "std::vec::Vec<U>".to_owned(),
                    item: Box::new(output.clone()),
                },
                "U",
                Vec::new(),
            );
            method.native_owner = None;
            method.native_path =
                Some("terrane_collection_support::collect_borrowed_slice".to_owned());
            method
                .parameters
                .push(borrowed_visitor(item.clone(), output));
            Some(graph_owner(
                namespace.clone(),
                name.clone(),
                rust_path.clone(),
                "Native borrowed slice; usable only inside one operation graph.",
                vec![method],
            ))
        })
        .collect::<Vec<_>>();
    items.extend(additions);
}

/// Structural selectors name the enum in a Rust pattern. A signature can
/// mention an enum from a transitive crate without making it nameable.
pub(super) fn resolve_optional_selectors(
    projected: &mut [super::ProjectedDependency],
    rustdocs: &[(
        &super::RustDependency,
        super::RustdocCrate,
        BTreeMap<rustdoc_types::Id, String>,
    )],
) {
    let mut exports = BTreeMap::new();
    for (dependency, document, public_paths) in rustdocs {
        for (id, path) in public_paths {
            let Some(summary) = document.paths.get(id) else {
                continue;
            };
            if summary.kind != rustdoc_types::ItemKind::Enum {
                continue;
            }
            let emitted = super::extern_rust_path(dependency, path);
            exports.insert(emitted.clone(), emitted.clone());
            exports.insert(path.clone(), emitted.clone());
            exports
                .entry(summary.path.join("::"))
                .and_modify(|current| super::prefer_alias(current, &emitted))
                .or_insert(emitted);
        }
    }
    for dependency in projected {
        let (items, declined) = (&mut dependency.items, &mut dependency.declined);
        for item in items {
            let ProjectedKind::ForeignType { methods, .. } = &mut item.kind else {
                continue;
            };
            methods.retain_mut(|method| {
                let Some(super::ProjectedEnumOperation::SelectOptionalVariant { path }) =
                    &mut method.enum_operation
                else {
                    return true;
                };
                let Some((export, variant)) = path.rsplit_once("::").and_then(|(owner, variant)| {
                    Some((exports.get(owner)?, variant))
                }) else {
                    declined.push(DeclinedItem {
                        rust_path: format!("{}::{}", item.rust_path, method.name),
                        reason: "optional-sum variant owner is not nameable from a public dependency export".to_owned(),
                    });
                    return false;
                };
                *path = format!("{export}::{variant}");
                true
            });
        }
    }
}

fn add_optional_sum_owners(items: &mut Vec<ProjectedItem>) {
    let mut additions = BTreeMap::new();
    for item in items.iter() {
        for function in source_rendering::projected_item_functions(item) {
            let ProjectedType::InvocationScoped {
                name,
                rust_type,
                variants,
                ..
            } = &function.result
            else {
                continue;
            };
            if variants.is_empty() {
                continue;
            }
            let mut selectors = Vec::with_capacity(variants.len());
            for variant in variants {
                let ProjectedType::Foreign {
                    name,
                    rust_path,
                    arguments,
                    ..
                } = &variant.slice
                else {
                    unreachable!("projected slice owner")
                };
                let [element] = arguments.as_slice() else {
                    unreachable!("slice element type")
                };
                let selected_name = format!("optional-{name}");
                let selected_path = format!("std::option::Option<&{rust_path}>");
                let selected_owner = ProjectedType::Foreign {
                    name: selected_name.clone(),
                    rust_path: selected_path.clone(),
                    base_rust_path: selected_path.clone(),
                    arguments: Vec::new(),
                };
                let selected = ProjectedType::InvocationScoped {
                    name: selected_name.clone(),
                    rust_type: selected_path.clone(),
                    lifetimes: Vec::new(),
                    expression_scoped: true,
                    owned: Box::new(selected_owner),
                    variants: Vec::new(),
                };
                let mut select = concrete_terminal(&variant.name, selected);
                select.chain_role = Some(ChainRole::Continue);
                select.native_owner = None;
                select.enum_operation =
                    Some(super::ProjectedEnumOperation::SelectOptionalVariant {
                        path: variant.path.clone(),
                    });
                selectors.push(select);
                additions
                    .entry((item.namespace.clone(), selected_name.clone()))
                    .or_insert_with(|| {
                        let get = optional_slice_lookup(element);
                        graph_owner(
                            item.namespace.clone(),
                            selected_name,
                            selected_path,
                            "Optional borrowed slice; usable only within one operation graph.",
                            vec![get],
                        )
                    });
            }
            additions
                .entry((item.namespace.clone(), name.clone()))
                .or_insert_with(|| {
                    graph_owner(
                        item.namespace.clone(),
                        name.clone(),
                        rust_type.clone(),
                        "Optional native sum containing borrowed payloads.",
                        selectors,
                    )
                });
        }
    }
    additions.retain(|(namespace, name), _| {
        !items
            .iter()
            .any(|item| &item.namespace == namespace && &item.name == name)
    });
    items.extend(additions.into_values());
}

fn optional_slice_lookup(element: &ProjectedType) -> ProjectedFunction {
    let mut method = concrete_terminal(
        "get_cloned",
        ProjectedType::Optional(Box::new(element.clone())),
    );
    method.native_path = Some("terrane_collection_support::get_cloned_borrowed_slice".to_owned());
    method.parameters.push(ProjectedParameter {
        name: "index".to_owned(),
        ty: ProjectedType::FixedInt("u64".to_owned()),
        borrow_lifetime: None,
        borrowed: false,
        borrowed_str: false,
        mutable_borrow: false,
        generic_parameter: None,
        generic_bounds: Vec::new(),
        generic_interface: None,
        associated_type: None,
    });
    method
}

fn concrete_terminal(name: &str, result: ProjectedType) -> ProjectedFunction {
    let mut method = terminal_method(name, result, "U", Vec::new());
    method.destination_result = None;
    method.generic_parameters.clear();
    method.native_path = None;
    method
}

// Option is already a compiler-owned projection protocol. Its borrowed form stays native
// until one of these native operations produces an owned terminal or another scoped edge.
pub(super) fn optional_owner(namespace: String) -> ProjectedItem {
    let item = ProjectedType::Generic("T".to_owned());
    let optional = ProjectedType::Optional(Box::new(item.clone()));
    let mut cloned = terminal_method(
        "cloned",
        optional.clone(),
        "T",
        vec!["std::clone::Clone".to_owned()],
    );
    cloned.native_owner = Some("std::option::Option<&T>".to_owned());
    cloned.native_path = None;
    let mut map = terminal_method(
        "map",
        ProjectedType::Optional(Box::new(ProjectedType::Generic("U".to_owned()))),
        "U",
        Vec::new(),
    );
    map.native_path = None;
    map.generic_parameters.push(ProjectedGenericParameter {
        name: "T".to_owned(),
        input_selected: true,
        rust_bounds: Vec::new(),
        default: None,
    });
    map.parameters.push(ProjectedParameter {
        borrowed_str: false,
        name: "operation".to_owned(),
        ty: ProjectedType::Callback {
            carrier: None,
            rust_name: "F".to_owned(),
            parameters: vec![item],
            native_bound: Some("FnOnce(&T) -> U".to_owned()),
            native_method: None,
            native_result: None,
            native_substitutions: BTreeMap::new(),
            parameter_rust_types: vec!["&T".to_owned()],
            parameter_borrows: vec![true],
            admitted_shapes: Vec::new(),
            parameters_destination_selected: false,
            result: Box::new(ProjectedType::Generic("U".to_owned())),
            invocation_mode: InvocationMode::Consuming,
            is_async: false,
            retained: false,
            send: false,
            sync: false,
        },
        borrow_lifetime: None,
        borrowed: false,
        mutable_borrow: false,
        generic_parameter: Some("F".to_owned()),
        generic_bounds: Vec::new(),
        generic_interface: None,
        associated_type: None,
    });
    let mut and_then = map.clone();
    "and_then".clone_into(&mut and_then.name);
    if let ProjectedType::Callback {
        result,
        native_bound,
        ..
    } = &mut and_then.parameters[0].ty
    {
        **result = ProjectedType::Optional(Box::new(ProjectedType::Generic("U".to_owned())));
        *native_bound = Some("FnOnce(&T) -> Option<U>".to_owned());
    }
    graph_owner(
        namespace,
        "borrowed-option".to_owned(),
        "std::option::Option".to_owned(),
        "Native optional borrow; usable only inside one operation graph.",
        vec![cloned, map, and_then],
    )
}

fn graph_owner(
    namespace: String,
    name: String,
    rust_path: String,
    docs: &str,
    methods: Vec<ProjectedFunction>,
) -> ProjectedItem {
    ProjectedItem {
        namespace,
        name,
        rust_path,
        docs: Some(docs.to_owned()),
        kind: ProjectedKind::ForeignType {
            constructor: None,
            methods,
            static_methods: Vec::new(),
            constants: Vec::new(),
            cloneable: false,
            copyable: false,
            send: false,
            sync: false,
            displayable: false,
            fields: Vec::new(),
            borrowed_view: false,
            native_view_type: None,
            enum_payload: None,
            generic_parameters: Vec::new(),
            boundary: ProjectedBoundaryCapabilities::default(),
        },
    }
}

/// A scoped iterator with a shared borrowed item lends directly to a synchronous
/// visitor; neither the item nor the iterator owner is cloned.
pub(super) fn iterator_find_map(item: ProjectedType, native_owner: &str) -> ProjectedFunction {
    let result = ProjectedType::Optional(Box::new(ProjectedType::Generic("U".to_owned())));
    let mut method = terminal_method("find_map", result.clone(), "U", Vec::new());
    method.native_owner = Some(native_owner.to_owned());
    method.native_path = Some("std::iter::Iterator::find_map".to_owned());
    method.receiver = Some(Receiver::MutableBorrow);
    method.parameters.push(borrowed_visitor(item, result));
    method
}

pub(super) fn iterator_collect(
    item: ProjectedType,
    native_owner: &str,
    filter: bool,
) -> ProjectedFunction {
    let (name, native_path) = if filter {
        (
            "filter_map_collect",
            "terrane_collection_support::filter_map_collect_borrowed",
        )
    } else {
        (
            "map_collect",
            "terrane_collection_support::map_collect_borrowed",
        )
    };
    let output = ProjectedType::Generic("U".to_owned());
    let mut method = terminal_method(
        name,
        ProjectedType::Sequence {
            rust_path: "std::vec::Vec<U>".to_owned(),
            item: Box::new(output.clone()),
        },
        "U",
        Vec::new(),
    );
    method.native_owner = Some(native_owner.to_owned());
    method.native_path = Some(native_path.to_owned());
    method.receiver = Some(Receiver::MutableBorrow);
    let result = if filter {
        ProjectedType::Optional(Box::new(output))
    } else {
        output
    };
    method.parameters.push(borrowed_visitor(item, result));
    method
}

/// Reduce a borrowed iterator without materializing borrowed items or cloning its owner.
pub(super) fn iterator_fold(item: ProjectedType, native_owner: &str) -> ProjectedFunction {
    let accumulator = ProjectedType::Generic("U".to_owned());
    let (parameters, pair) = match item {
        ProjectedType::Tuple(items) => (items, true),
        item => (vec![item], false),
    };
    let native_parameters = parameters
        .iter()
        .map(|item| format!("&{}", item.rust_type()))
        .collect::<Vec<_>>();
    let bound = format!("FnMut(U, {}) -> U", native_parameters.join(", "));
    let mut method = terminal_method("fold", accumulator.clone(), "U", Vec::new());
    method.generic_parameters[0].input_selected = true;
    method
        .generic_parameters
        .push(super::ProjectedGenericParameter {
            name: "F".to_owned(),
            input_selected: false,
            rust_bounds: vec![bound.clone()],
            default: None,
        });
    method.native_owner = Some(native_owner.to_owned());
    method.native_path = Some(if pair {
        "terrane_collection_support::fold_borrowed_pair".to_owned()
    } else {
        "terrane_collection_support::fold_borrowed".to_owned()
    });
    method.receiver = Some(Receiver::MutableBorrow);
    method.parameters.push(ProjectedParameter {
        name: "initial".to_owned(),
        ty: accumulator,
        borrow_lifetime: None,
        borrowed: false,
        borrowed_str: false,
        mutable_borrow: false,
        generic_parameter: Some("U".to_owned()),
        generic_bounds: Vec::new(),
        generic_interface: None,
        associated_type: None,
    });
    method.parameters.push(ProjectedParameter {
        name: "operation".to_owned(),
        ty: ProjectedType::Callback {
            carrier: None,
            rust_name: "F".to_owned(),
            parameters: std::iter::once(ProjectedType::Generic("U".to_owned()))
                .chain(parameters)
                .collect(),
            native_bound: Some(bound),
            native_method: None,
            native_result: None,
            native_substitutions: BTreeMap::new(),
            parameter_rust_types: std::iter::once("U".to_owned())
                .chain(native_parameters)
                .collect(),
            parameter_borrows: std::iter::once(false)
                .chain(std::iter::repeat_n(true, if pair { 2 } else { 1 }))
                .collect(),
            admitted_shapes: Vec::new(),
            parameters_destination_selected: false,
            result: Box::new(ProjectedType::Generic("U".to_owned())),
            invocation_mode: InvocationMode::Shared,
            is_async: false,
            retained: false,
            send: false,
            sync: false,
        },
        borrow_lifetime: None,
        borrowed: false,
        borrowed_str: false,
        mutable_borrow: false,
        generic_parameter: Some("F".to_owned()),
        generic_bounds: Vec::new(),
        generic_interface: None,
        associated_type: None,
    });
    method
}

fn borrowed_visitor(item: ProjectedType, result: ProjectedType) -> ProjectedParameter {
    let native_item = format!("&{}", item.rust_type());
    ProjectedParameter {
        name: "operation".to_owned(),
        ty: ProjectedType::Callback {
            carrier: None,
            rust_name: "F".to_owned(),
            parameters: vec![item],
            native_bound: Some(format!("FnMut({native_item}) -> {}", result.rust_type())),
            native_method: None,
            native_result: None,
            native_substitutions: BTreeMap::new(),
            parameter_rust_types: vec![native_item],
            parameter_borrows: vec![true],
            admitted_shapes: Vec::new(),
            parameters_destination_selected: false,
            result: Box::new(result),
            invocation_mode: InvocationMode::Shared,
            is_async: false,
            retained: false,
            send: false,
            sync: false,
        },
        borrow_lifetime: None,
        borrowed: false,
        mutable_borrow: false,
        borrowed_str: false,
        generic_parameter: Some("F".to_owned()),
        generic_bounds: Vec::new(),
        generic_interface: None,
        associated_type: None,
    }
}

fn terminal_method(
    name: &str,
    result: ProjectedType,
    selected: &str,
    bounds: Vec<String>,
) -> ProjectedFunction {
    ProjectedFunction {
        receiver_bounds: Vec::new(),
        name: name.to_owned(),
        native_owner: Some("std::option::Option".to_owned()),
        native_path: Some(format!("std::option::Option::<&T>::{name}")),
        parameters: Vec::new(),
        generic_parameters: vec![ProjectedGenericParameter {
            input_selected: false,
            name: selected.to_owned(),
            rust_bounds: bounds.clone(),
            default: None,
        }],
        operation_owner_generics: Vec::new(),
        rust_generic_arguments: Vec::new(),
        result,
        destination_result: Some(ProjectedDestinationResult {
            parameters: vec![ProjectedDestinationParameter {
                name: selected.to_owned(),
                rust_bounds: bounds,
            }],
            bound_roots: Vec::new(),
        }),
        error: None,
        is_async: false,
        is_unsafe: false,
        into_future: false,
        execution_requirements: None,
        enum_operation: None,
        error_optional_depth: 0,
        chain_role: Some(ChainRole::Terminal),
        receiver_lifetime: None,
        receiver: Some(Receiver::Move),
    }
}
