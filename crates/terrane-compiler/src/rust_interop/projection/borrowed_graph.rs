use super::{
    BTreeMap, BTreeSet, ChainRole, InvocationMode, ProjectedBoundaryCapabilities,
    ProjectedDestinationParameter, ProjectedDestinationResult, ProjectedFunction,
    ProjectedGenericParameter, ProjectedItem, ProjectedKind, ProjectedParameter, ProjectedType,
    Receiver, source_rendering,
};

pub(super) fn add_optional_owners(items: &mut Vec<ProjectedItem>) {
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

// Option is already a compiler-owned projection protocol. Its borrowed form stays native
// until one of these native operations produces an owned terminal or another scoped edge.
pub(super) fn optional_owner(namespace: String) -> ProjectedItem {
    let item = ProjectedType::Generic("T".to_owned());
    let optional = ProjectedType::Optional(Box::new(item.clone()));
    let mut cloned = optional_method(
        "cloned",
        optional.clone(),
        "T",
        vec!["std::clone::Clone".to_owned()],
    );
    cloned.native_owner = Some("std::option::Option<&T>".to_owned());
    cloned.native_path = None;
    let mut map = optional_method(
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
    });
    map.parameters.push(ProjectedParameter {
        name: "operation".to_owned(),
        ty: ProjectedType::Callback {
            rust_name: "F".to_owned(),
            parameters: vec![item],
            native_bound: Some("FnOnce(&T) -> U".to_owned()),
            native_method: None,
            native_result: None,
            native_substitutions: BTreeMap::new(),
            parameter_rust_types: vec!["&T".to_owned()],
            parameter_borrows: vec![true],
            parameters_destination_selected: false,
            result: Box::new(ProjectedType::Generic("U".to_owned())),
            invocation_mode: InvocationMode::Consuming,
            is_async: false,
            retained: false,
            send: false,
            sync: false,
        },
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
    ProjectedItem {
        namespace,
        name: "borrowed-option".to_owned(),
        rust_path: "std::option::Option".to_owned(),
        docs: Some("Native optional borrow; usable only inside one operation graph.".to_owned()),
        kind: ProjectedKind::ForeignType {
            methods: vec![cloned, map, and_then],
            static_methods: Vec::new(),
            constants: Vec::new(),
            cloneable: false,
            send: false,
            sync: false,
            displayable: false,
            fields: Vec::new(),
            borrowed_view: false,
            native_view_type: None,
            enum_payload: None,
            boundary: ProjectedBoundaryCapabilities::default(),
        },
    }
}

fn optional_method(
    name: &str,
    result: ProjectedType,
    selected: &str,
    bounds: Vec<String>,
) -> ProjectedFunction {
    ProjectedFunction {
        name: name.to_owned(),
        native_owner: Some("std::option::Option".to_owned()),
        native_path: Some(format!("std::option::Option::<&T>::{name}")),
        parameters: Vec::new(),
        generic_parameters: vec![ProjectedGenericParameter {
            input_selected: false,
            name: selected.to_owned(),
            rust_bounds: bounds.clone(),
        }],
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
        receiver: Some(Receiver::Move),
    }
}
