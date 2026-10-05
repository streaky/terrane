use super::{
    BTreeSet, Id, Item, ItemEnum, ProjectedDestinationParameter, ProjectedDestinationResult,
    ProjectedFunction, ProjectedItem, ProjectedKind, ProjectedParameter, ProjectedType,
    RustDependency, RustdocCrate, Visibility, dependency_namespace, extern_rust_path,
};

pub(super) fn project_macro(name: &str) -> ProjectedFunction {
    ProjectedFunction {
        native_owner: None,
        native_path: None,
        name: name.to_owned(),
        parameters: vec![ProjectedParameter {
            name: "arguments".to_owned(),
            ty: ProjectedType::Generic("__MacroArgument".to_owned()),
            borrowed: false,
            mutable_borrow: false,
            generic_parameter: None,
            generic_bounds: Vec::new(),
            generic_interface: None,
            associated_type: None,
        }],
        generic_parameters: Vec::new(),
        operation_owner_generics: Vec::new(),
        rust_generic_arguments: Vec::new(),
        result: ProjectedType::Generic("__MacroResult".to_owned()),
        destination_result: Some(ProjectedDestinationResult {
            parameters: vec![ProjectedDestinationParameter {
                name: "__MacroResult".to_owned(),
                rust_bounds: Vec::new(),
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
        chain_role: None,
        receiver: None,
    }
}

pub(super) fn macro_reexports(
    dependency: &RustDependency,
    document: &RustdocCrate,
) -> Vec<ProjectedItem> {
    fn visit(
        dependency: &RustDependency,
        document: &RustdocCrate,
        id: Id,
        path: &mut Vec<String>,
        visited: &mut BTreeSet<Id>,
        items: &mut Vec<ProjectedItem>,
    ) {
        if !visited.insert(id) {
            return;
        }
        let Some(Item {
            inner: ItemEnum::Module(module),
            ..
        }) = document.index.get(&id)
        else {
            return;
        };
        for id in &module.items {
            let Some(item) = document
                .index
                .get(id)
                .filter(|item| item.visibility == Visibility::Public)
            else {
                continue;
            };
            match &item.inner {
                ItemEnum::Module(_) => {
                    if let Some(name) = &item.name {
                        path.push(name.clone());
                        visit(dependency, document, *id, path, visited, items);
                        path.pop();
                    }
                }
                ItemEnum::Use(import)
                    if !import.is_glob
                        && import.id.is_some_and(|target| {
                            document
                                .index
                                .get(&target)
                                .is_some_and(|item| matches!(item.inner, ItemEnum::Macro(_)))
                                || document.paths.get(&target).is_some_and(|summary| {
                                    summary.kind == rustdoc_types::ItemKind::Macro
                                })
                        }) =>
                {
                    let mut export = path.clone();
                    export.push(import.name.clone());
                    items.push(ProjectedItem {
                        namespace: format!("{}/macros", dependency_namespace(dependency, path)),
                        name: import.name.clone(),
                        rust_path: extern_rust_path(dependency, &export.join("::")),
                        docs: item.docs.clone(),
                        kind: ProjectedKind::Macro(project_macro(&import.name)),
                    });
                }
                _ => {}
            }
        }
    }
    let mut items = Vec::new();
    let root_name = document
        .index
        .get(&document.root)
        .and_then(|item| item.name.clone())
        .unwrap_or_else(|| dependency.package.replace('-', "_"));
    visit(
        dependency,
        document,
        document.root,
        &mut vec![root_name],
        &mut BTreeSet::new(),
        &mut items,
    );
    items
}
