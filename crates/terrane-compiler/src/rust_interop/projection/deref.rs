//! Discover receiver views from canonical type arguments/Rustdoc associated
//! targets, then admit them only after rustc proves the exact Deref contract.
//! This keeps owner identity and native access mode independent of pointer names.
use super::*;
use quote::ToTokens;

#[derive(Clone, Copy)]
#[repr(u8)]
enum Access {
    Shared = 1 << 0,
    Mutable = 1 << 1,
    MoveTarget = 1 << 2,
    BoxedTarget = 1 << 3,
    CloneOwner = 1 << 4,
}

#[derive(Clone, Copy, Default)]
struct AccessCapabilities(u8);

impl AccessCapabilities {
    fn insert_if(&mut self, condition: bool, capability: Access) {
        if condition {
            self.0 |= capability as u8;
        }
    }

    fn contains(self, capability: Access) -> bool {
        self.0 & capability as u8 != 0
    }
}

fn key(rust: &str) -> Option<String> {
    syn::parse_str::<syn::Type>(rust)
        .ok()
        .map(|ty| ty.to_token_stream().to_string())
}

fn collect(
    ty: &ProjectedType,
    types: &mut BTreeMap<String, ProjectedType>,
    views: &mut BTreeMap<String, String>,
) {
    match ty {
        ProjectedType::Foreign { arguments, .. } => {
            if let Some(identity) = key(&ty.rust_type()) {
                types.entry(identity).or_insert_with(|| ty.clone());
            }
            for argument in arguments {
                collect(argument, types, views);
            }
        }
        ProjectedType::Optional(inner)
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. }
        | ProjectedType::Reference { inner, .. } => collect(inner, types, views),
        ProjectedType::InvocationScoped {
            owned, rust_type, ..
        } => {
            if let Some(identity) = key(&owned.rust_type()) {
                views.entry(identity).or_insert_with(|| rust_type.clone());
            }
            collect(owned, types, views);
        }
        ProjectedType::Mapping { key, value, .. } => {
            collect(key, types, views);
            collect(value, types, views);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                collect(item, types, views);
            }
        }
        ProjectedType::Callback {
            parameters, result, ..
        } => {
            for parameter in parameters {
                collect(parameter, types, views);
            }
            collect(result, types, views);
        }
        _ => {}
    }
}

fn target_from_document(
    owner: &ProjectedType,
    dependency: &RustDependency,
    document: &RustdocCrate,
    public_paths: &BTreeMap<Id, String>,
) -> Vec<ProjectedType> {
    let ProjectedType::Foreign {
        base_rust_path,
        arguments,
        ..
    } = owner
    else {
        return Vec::new();
    };
    let mut targets = Vec::new();
    for (id, path) in public_paths {
        if key(&extern_rust_path(dependency, path)) != key(base_rust_path) {
            continue;
        }
        let Some(item) = document.index.get(id) else {
            continue;
        };
        let (implementations, parameters) = match &item.inner {
            ItemEnum::Struct(value) => (&value.impls, &value.generics.params),
            ItemEnum::Enum(value) => (&value.impls, &value.generics.params),
            _ => continue,
        };
        let mut generics = BTreeMap::from([("Self".to_owned(), owner.clone())]);
        generics.extend(
            parameters
                .iter()
                .filter(|parameter| {
                    matches!(
                        parameter.kind,
                        rustdoc_types::GenericParamDefKind::Type { .. }
                    )
                })
                .map(|parameter| parameter.name.clone())
                .zip(arguments.iter().cloned()),
        );
        for implementation in implementations {
            let Some(Item {
                inner: ItemEnum::Impl(implementation),
                ..
            }) = document.index.get(implementation)
            else {
                continue;
            };
            let Some(trait_) = &implementation.trait_ else {
                continue;
            };
            let canonical_trait = document
                .paths
                .get(&trait_.id)
                .map(|summary| summary.path.join("::"));
            if !matches!(
                canonical_trait.as_deref(),
                Some("core::ops::deref::Deref" | "core::ops::Deref" | "std::ops::Deref")
            ) {
                continue;
            }
            for member in &implementation.items {
                let Some(Item {
                    name: Some(name),
                    inner:
                        ItemEnum::AssocType {
                            type_: Some(target),
                            ..
                        },
                    ..
                }) = document.index.get(member)
                else {
                    continue;
                };
                if name == "Target"
                    && let Ok(target) =
                        project_type(target, &document.index, &document.paths, &generics)
                {
                    targets.push(target);
                }
            }
        }
    }
    targets
}

fn owner_clone(owner: &ProjectedType) -> ProjectedFunction {
    ProjectedFunction {
        native_owner: Some(owner.rust_type()),
        native_path: Some(format!(
            "<{} as core::clone::Clone>::clone",
            owner.rust_type()
        )),
        name: "clone".to_owned(),
        parameters: Vec::new(),
        generic_parameters: Vec::new(),
        receiver_bounds: Vec::new(),
        operation_owner_generics: Vec::new(),
        rust_generic_arguments: Vec::new(),
        result: owner.clone(),
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
        receiver: Some(Receiver::Borrow),
    }
}

pub(super) fn project_deref_receivers(
    dependencies: &mut [ProjectedDependency],
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    workspace: &Path,
    identity: &str,
    sandbox: Containment,
) -> Result<(), ProjectionError> {
    let (types, namespaces, native_views) = collect_projected_types(dependencies);
    let pairs = discover_receiver_pairs(&types, rustdocs);
    if pairs.is_empty() {
        return Ok(());
    }
    let access = prove_receiver_access(&pairs, workspace, identity, sandbox)?;
    apply_receiver_methods(dependencies, &pairs, &access, &namespaces, &native_views);
    Ok(())
}

fn collect_projected_types(
    dependencies: &[ProjectedDependency],
) -> (
    BTreeMap<String, ProjectedType>,
    BTreeMap<String, String>,
    BTreeMap<String, String>,
) {
    let mut types = BTreeMap::new();
    let mut namespaces = BTreeMap::new();
    let mut native_views = BTreeMap::new();
    for dependency in dependencies {
        let namespace = format!("/deps/{}", dependency.name);
        for item in &dependency.items {
            for function in source_rendering::projected_item_functions(item) {
                for parameter in &function.parameters {
                    collect(&parameter.ty, &mut types, &mut native_views);
                }
                collect(&function.result, &mut types, &mut native_views);
            }
            if let ProjectedKind::ForeignType {
                fields,
                generic_parameters,
                ..
            } = &item.kind
            {
                for field in fields {
                    collect(&field.ty, &mut types, &mut native_views);
                }
                if generic_parameters.is_empty()
                    && let Some(identity) = key(&item.rust_path)
                {
                    types
                        .entry(identity)
                        .or_insert_with(|| ProjectedType::Foreign {
                            name: item.name.clone(),
                            rust_path: item.rust_path.clone(),
                            base_rust_path: crate::rust_ir::rust_type_constructor(&item.rust_path)
                                .unwrap_or_else(|| item.rust_path.clone()),
                            arguments: Vec::new(),
                        });
                }
            }
        }
        for identity in types.keys() {
            namespaces
                .entry(identity.clone())
                .or_insert_with(|| namespace.clone());
        }
    }
    (types, namespaces, native_views)
}

fn discover_receiver_pairs(
    types: &BTreeMap<String, ProjectedType>,
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
) -> BTreeMap<(String, String), (ProjectedType, ProjectedType)> {
    let mut pairs = BTreeMap::new();
    for (owner_key, owner) in types {
        let ProjectedType::Foreign { arguments, .. } = owner else {
            continue;
        };
        let mut targets = arguments.clone();
        for (dependency, document, paths) in rustdocs {
            targets.extend(target_from_document(owner, dependency, document, paths));
        }
        for target in targets {
            let Some(target_key) = key(&target.rust_type()) else {
                continue;
            };
            if target_key != *owner_key && matches!(target, ProjectedType::Foreign { .. }) {
                pairs
                    .entry((owner_key.clone(), target_key))
                    .or_insert((owner.clone(), target));
            }
        }
    }
    pairs
}

fn prove_receiver_access(
    pairs: &BTreeMap<(String, String), (ProjectedType, ProjectedType)>,
    workspace: &Path,
    identity: &str,
    sandbox: Containment,
) -> Result<BTreeMap<(String, String), AccessCapabilities>, ProjectionError> {
    let mut questions = Vec::new();
    let mut calls = Vec::new();
    for ((owner_key, target_key), (owner, target)) in pairs {
        for bound in [
            format!("core::ops::Deref<Target = {}>", target.rust_type()),
            "core::ops::DerefMut".to_owned(),
            "core::clone::Clone".to_owned(),
        ] {
            questions.push(crate::rust_interop::BoundQuestion {
                rust_type: owner.rust_type(),
                rust_bound: bound,
                inferred_parameters: Vec::new(),
            });
        }
        for (mode, result, expression) in [
            ("move", target.rust_type(), "*value"),
            (
                "boxed",
                format!("std::boxed::Box<{}>", target.rust_type()),
                "value",
            ),
        ] {
            calls.push(crate::rust_interop::CallQuestion {
                label: format!("receiver-{mode}:{owner_key}:{target_key}"),
                source: format!("fn __terrane_receiver(value: {}) -> {result} {{ {expression} }}\nfn main() {{}}\n", owner.rust_type()),
            });
        }
    }
    let oracle = crate::rust_interop::ProjectionOracle::new(workspace, identity, sandbox);
    let bounds = oracle.prove_bounds(&questions)?;
    let moves = oracle.prove_calls(&calls)?;
    let yes = bounds
        .evidence
        .iter()
        .filter(|entry| entry.answer == crate::rust_interop::ProbeAnswer::Yes)
        .map(|entry| {
            (
                entry.question.rust_type.as_str(),
                entry.question.rust_bound.as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    let move_yes = moves
        .evidence
        .iter()
        .filter(|entry| entry.answer == crate::rust_interop::ProbeAnswer::Yes)
        .map(|entry| entry.question.label.as_str())
        .collect::<BTreeSet<_>>();
    let mut access = BTreeMap::new();
    for ((owner_key, target_key), (owner, target)) in pairs {
        let owner_rust = owner.rust_type();
        let shared = yes.contains(&(
            owner_rust.as_str(),
            format!("core::ops::Deref<Target = {}>", target.rust_type()).as_str(),
        ));
        if !shared {
            continue;
        }
        let mut capabilities = AccessCapabilities::default();
        capabilities.insert_if(shared, Access::Shared);
        capabilities.insert_if(
            yes.contains(&(owner_rust.as_str(), "core::ops::DerefMut")),
            Access::Mutable,
        );
        capabilities.insert_if(
            yes.contains(&(owner_rust.as_str(), "core::clone::Clone")),
            Access::CloneOwner,
        );
        capabilities.insert_if(
            move_yes.contains(format!("receiver-move:{owner_key}:{target_key}").as_str()),
            Access::MoveTarget,
        );
        capabilities.insert_if(
            move_yes.contains(format!("receiver-boxed:{owner_key}:{target_key}").as_str()),
            Access::BoxedTarget,
        );
        access.insert((owner_key.clone(), target_key.clone()), capabilities);
    }
    Ok(access)
}

fn forwarded_receiver_methods(
    owner: &ProjectedType,
    target: &ProjectedType,
    target_methods: &[ProjectedFunction],
    capabilities: AccessCapabilities,
) -> Vec<ProjectedFunction> {
    let mut methods = if capabilities.contains(Access::CloneOwner) {
        vec![owner_clone(owner)]
    } else {
        Vec::new()
    };
    for method in target_methods {
        if method.name == "clone" && capabilities.contains(Access::CloneOwner) {
            continue;
        }
        let receiver = match method.receiver {
            Some(Receiver::Borrow) if capabilities.contains(Access::Shared) => Receiver::Borrow,
            Some(Receiver::MutableBorrow) if capabilities.contains(Access::Mutable) => {
                Receiver::MutableBorrow
            }
            Some(Receiver::Move) if capabilities.contains(Access::MoveTarget) => {
                Receiver::DerefMove(1)
            }
            Some(Receiver::DerefMove(depth)) if capabilities.contains(Access::MoveTarget) => {
                Receiver::DerefMove(depth + 1)
            }
            Some(Receiver::BoxedMove) if capabilities.contains(Access::BoxedTarget) => {
                Receiver::Move
            }
            _ => continue,
        };
        let mut method = method.clone();
        method.receiver = Some(receiver);
        if method.native_path.is_none() {
            method.native_path = Some(format!("<{}>::{}", target.rust_type(), method.name));
        }
        methods.push(method);
    }
    methods
}

fn apply_receiver_methods(
    dependencies: &mut [ProjectedDependency],
    pairs: &BTreeMap<(String, String), (ProjectedType, ProjectedType)>,
    access: &BTreeMap<(String, String), AccessCapabilities>,
    namespaces: &BTreeMap<String, String>,
    native_views: &BTreeMap<String, String>,
) {
    // A fixed point preserves direct-owner precedence and supports nested pointer
    // chains. No method is copied until its exact native receiver path is proven.
    loop {
        let snapshot = dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .cloned()
            .collect::<Vec<_>>();
        let mut changed = false;
        for ((owner_key, target_key), capabilities) in access {
            let (owner, target) = &pairs[&(owner_key.clone(), target_key.clone())];
            let Some(target_item) = snapshot
                .iter()
                .find(|item| key(&item.rust_path).as_ref() == Some(target_key))
            else {
                continue;
            };
            let (ProjectedKind::ForeignType {
                methods: target_methods,
                ..
            }
            | ProjectedKind::Enum {
                methods: target_methods,
                ..
            }) = &target_item.kind
            else {
                continue;
            };
            let methods = forwarded_receiver_methods(owner, target, target_methods, *capabilities);
            if methods.is_empty() {
                continue;
            }
            if let Some(item) = dependencies
                .iter_mut()
                .flat_map(|dependency| &mut dependency.items)
                .find(|item| key(&item.rust_path).as_ref() == Some(owner_key))
            {
                if let ProjectedKind::ForeignType {
                    methods: existing, ..
                }
                | ProjectedKind::Enum {
                    methods: existing, ..
                } = &mut item.kind
                {
                    for method in methods {
                        if !existing.iter().any(|entry| entry.name == method.name) {
                            existing.push(method);
                            changed = true;
                        }
                    }
                }
            } else {
                let ProjectedType::Foreign { name, .. } = owner else {
                    continue;
                };
                let namespace = namespaces[owner_key].clone();
                let dependency = dependencies
                    .iter_mut()
                    .find(|dependency| namespace == format!("/deps/{}", dependency.name))
                    .expect("recorded native owner dependency");
                dependency.items.push(ProjectedItem {
                    namespace,
                    name: name.clone(),
                    rust_path: owner.rust_type(),
                    docs: None,
                    kind: ProjectedKind::ForeignType {
                        methods,
                        static_methods: Vec::new(),
                        constants: Vec::new(),
                        boundary: ProjectedBoundaryCapabilities::default(),
                        fields: Vec::new(),
                        constructor: None,
                        borrowed_view: false,
                        native_view_type: native_views.get(owner_key).cloned(),
                        enum_payload: None,
                        displayable: false,
                        cloneable: capabilities.contains(Access::CloneOwner),
                        send: false,
                        sync: false,
                        generic_parameters: Vec::new(),
                    },
                });
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}
