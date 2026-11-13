//! Rustdoc method, chain-owner, and output-alias projection decisions.

use quote::ToTokens as _;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use rustdoc_types::{
    Function, GenericParamDef, GenericParamDefKind, Id, Impl, Item, ItemEnum, ItemSummary, Type,
    Visibility, WherePredicate,
};

use super::{
    ChainRole, ProjectedBoundaryCapabilities, ProjectedConstant, ProjectedFunction,
    ProjectedGenericParameter, ProjectedItem, ProjectedKind, ProjectedType, Receiver,
    RustDependency, SourceConstantCache, aliases, callable, canonicalize_rust_path, data,
    dependency_namespace, implementation_trait_path, project_function_with_generics,
    project_rust_constant_expression, project_type, projected_constant_name, render_resolved_path,
    render_rust_type, rewrite_rust_bound_root, rust_lifetimes, source_constant_expression,
    type_arguments,
};

pub(super) type ProjectedMethods = (
    Vec<ProjectedFunction>,
    Vec<(String, String, bool, Option<String>, ProjectedFunction)>,
    Vec<ProjectedConstant>,
    Vec<(String, String)>,
);

fn project_operation_owner_generics(
    implementation: &Impl,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_bounds: bool,
) -> Result<Vec<ProjectedGenericParameter>, String> {
    let declaration = &implementation.generics;
    let has_owner_lifetimes = declaration
        .params
        .iter()
        .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }));
    for predicate in &declaration.where_predicates {
        if !allow_lifetime_bounds
            && !matches!(
                predicate,
                WherePredicate::BoundPredicate {
                    type_: Type::Generic(name),
                    ..
                } if declaration.params.iter().any(|parameter| {
                    parameter.name == *name
                        && matches!(
                            parameter.kind,
                            GenericParamDefKind::Type { is_synthetic: false, .. }
                        )
                })
            )
        {
            return Err(
                "inherent impl predicate has no stable owner-generic projection".to_owned(),
            );
        }
    }
    let mut projected = BTreeMap::<String, ProjectedGenericParameter>::new();
    for parameter in &declaration.params {
        match &parameter.kind {
            GenericParamDefKind::Type {
                is_synthetic: false,
                ..
            } => {}
            GenericParamDefKind::Lifetime { outlives } if outlives.is_empty() => continue,
            _ => {
                return Err(
                    "inherent impl parameter has no stable owner-generic projection".to_owned(),
                );
            }
        }
        let rust_bounds = callable::render_generic_bounds_from_generics(
            parameter,
            declaration,
            &[],
            index,
            paths,
            generics,
        )?;
        if !allow_lifetime_bounds
            && rust_bounds.iter().any(|bound| {
                (bound.starts_with('\'') && bound != "'static")
                    || (has_owner_lifetimes
                        && rust_lifetimes(bound).iter().any(|lifetime| {
                            declaration.params.iter().any(|parameter| {
                                parameter.name == *lifetime
                                    && matches!(
                                        parameter.kind,
                                        GenericParamDefKind::Lifetime { .. }
                                    )
                            })
                        }))
            })
        {
            return Err(
                "inherent impl lifetime bound has no stable owner-generic projection".to_owned(),
            );
        }
        // The self-type substitution maps impl binders to nominal or alias slots.
        // Concrete alias arguments need no helper parameter; Rust checks their bounds.
        let name = match generics.get(&parameter.name) {
            Some(ProjectedType::Generic(name)) => name,
            Some(selected) if !selected.contains_open_generic() || rust_bounds.is_empty() => {
                continue;
            }
            None if allow_lifetime_bounds => &parameter.name,
            None if rust_bounds.is_empty() => continue,
            _ => {
                return Err(format!(
                    "inherent impl binder `{}` has no stable owner-generic projection",
                    parameter.name
                ));
            }
        };
        let entry = projected
            .entry(name.clone())
            .or_insert_with(|| ProjectedGenericParameter {
                input_selected: false,
                name: name.clone(),
                rust_bounds: Vec::new(),
                default: None,
            });
        entry.rust_bounds.extend(rust_bounds);
        entry.rust_bounds.sort();
        entry.rust_bounds.dedup();
    }
    Ok(projected.into_values().collect())
}

fn disambiguate_static_method_owner_generics(
    implementation: &Impl,
    function: &Function,
    mut generics: BTreeMap<String, ProjectedType>,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> BTreeMap<String, ProjectedType> {
    // Keep owner and method binders distinct before name-keyed signature projection.
    let method_names = function
        .generics
        .params
        .iter()
        .map(|parameter| parameter.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut used_names = method_names
        .iter()
        .map(|name| (*name).to_owned())
        .collect::<BTreeSet<_>>();
    used_names.extend(
        implementation
            .generics
            .params
            .iter()
            .map(|parameter| parameter.name.clone()),
    );
    used_names.extend(generics.values().filter_map(|generic| match generic {
        ProjectedType::Generic(name) => Some(name.clone()),
        _ => None,
    }));
    for parameter in &implementation.generics.params {
        let GenericParamDefKind::Type {
            is_synthetic: false,
            ..
        } = &parameter.kind
        else {
            continue;
        };
        let Some(ProjectedType::Generic(owner_slot)) = generics.get(&parameter.name) else {
            continue;
        };
        if !method_names.contains(owner_slot.as_str()) {
            continue;
        }
        let owner_slot = owner_slot.clone();
        let mut suffix = 0;
        let mut renamed = format!("__TerraneOwner_{owner_slot}__{suffix}");
        while used_names.contains(&renamed) {
            suffix += 1;
            renamed = format!("__TerraneOwner_{owner_slot}__{suffix}");
        }
        used_names.insert(renamed.clone());
        generics.remove(&owner_slot);
        generics.insert(parameter.name.clone(), ProjectedType::Generic(renamed));
    }
    if let Ok(owner) = project_type(&implementation.for_, index, paths, &generics) {
        generics.insert("Self".to_owned(), owner);
    }
    generics
}

/// Lifetime-bearing native results retain their source-derived lender graph so
/// ordinary local bindings can use the lender until the enclosing function exits.
fn project_method_function(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    name: Option<&str>,
    generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
) -> Result<ProjectedFunction, String> {
    match project_function_with_generics(
        function,
        index,
        paths,
        public_paths,
        name,
        generics,
        allow_lifetime_output,
    ) {
        Ok(function) => Ok(function),
        Err(reason) => {
            let lifetime_bearing_output = function.sig.output.as_ref().is_some_and(|output| {
                expand_output_alias(output.clone(), index, paths, generics.clone()).is_ok_and(
                    |(effective_output, _)| {
                        callable::type_contains_lifetime_argument(&effective_output)
                    },
                )
            });
            if allow_lifetime_output || !lifetime_bearing_output {
                return Err(reason);
            }
            let scoped = project_function_with_generics(
                function,
                index,
                paths,
                public_paths,
                name,
                generics,
                true,
            )?;
            if !matches!(scoped.result, ProjectedType::InvocationScoped { .. }) {
                return Err(reason);
            }
            Ok(scoped)
        }
    }
}

fn scoped_self_matches_owner(scoped: &ProjectedType, owner: &ProjectedType) -> bool {
    let ProjectedType::InvocationScoped { owned, .. } = scoped else {
        return false;
    };
    matches!(
        (owned.as_ref(), owner),
        (
            ProjectedType::Foreign {
                rust_path: left_path,
                arguments: left_arguments,
                ..
            },
            ProjectedType::Foreign {
                rust_path: right_path,
                arguments: right_arguments,
                ..
            }
        ) if left_path == right_path && left_arguments == right_arguments
    )
}

fn implementation_lifetime_substitutions(
    implementation: &Impl,
    self_type: &Type,
    scoped_self: Option<&ProjectedType>,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> BTreeMap<String, String> {
    let binders = implementation
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Lifetime { .. } => Some(normalize_lifetime_name(&parameter.name)),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let replacement = scoped_self
        .filter(|ty| matches!(ty, ProjectedType::InvocationScoped { .. }))
        .and_then(|scoped| {
            let source = render_rust_type(self_type, index, paths, &BTreeMap::new()).ok()?;
            let source_lifetimes = lifetime_occurrences(&source)?;
            let target_lifetimes = lifetime_occurrences(&scoped.rust_type())?;
            (source_lifetimes.len() == target_lifetimes.len()).then_some(
                source_lifetimes
                    .into_iter()
                    .zip(target_lifetimes)
                    .collect::<Vec<_>>(),
            )
        });
    let mut substitutions = BTreeMap::new();
    if let Some(replacements) = replacement {
        for (source, target) in replacements {
            if binders.contains(&source) {
                if substitutions
                    .get(&source)
                    .is_some_and(|existing| existing != &target)
                {
                    return BTreeMap::new();
                }
                substitutions.insert(source, target);
            }
        }
    }
    // Rustdoc may spell `Self` with its nominal declaration's lifetimes
    // even for `impl Owner<'_>`, whose impl has no named lifetime binders.
    // Map those declaration slots to the same receiver view as the impl slots.
    if let (Type::ResolvedPath(path), Some(scoped)) = (self_type, scoped_self)
        && let Some(item) = index.get(&path.id)
    {
        let parameters = match &item.inner {
            ItemEnum::Struct(declaration) => Some(&declaration.generics.params),
            ItemEnum::Enum(declaration) => Some(&declaration.generics.params),
            _ => None,
        };
        if let (Some(parameters), Some(targets)) =
            (parameters, lifetime_occurrences(&scoped.rust_type()))
        {
            let declarations = parameters
                .iter()
                .filter_map(|parameter| match parameter.kind {
                    GenericParamDefKind::Lifetime { .. } => {
                        Some(normalize_lifetime_name(&parameter.name))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();
            if declarations.len() == targets.len() {
                for (declaration, target) in declarations.into_iter().zip(targets) {
                    substitutions.entry(declaration).or_insert(target);
                }
            }
        }
    }
    substitutions
}

fn normalize_lifetime_name(name: &str) -> String {
    if name.starts_with('\'') {
        name.to_owned()
    } else {
        format!("'{name}")
    }
}

/// A higher-ranked binder scopes its entire node, not just its declaration.
/// Keep collection and replacement on the same hygienic traversal.
struct FreeLifetimeFold<F> {
    action: F,
    bound: Vec<BTreeSet<String>>,
}

impl<F> FreeLifetimeFold<F> {
    fn enter(&mut self, binders: Option<&syn::BoundLifetimes>) -> bool {
        let Some(binders) = binders else {
            return false;
        };
        self.bound.push(
            binders
                .lifetimes
                .iter()
                .filter_map(|parameter| match parameter {
                    syn::GenericParam::Lifetime(parameter) => Some(parameter.lifetime.to_string()),
                    _ => None,
                })
                .collect(),
        );
        true
    }

    fn leave(&mut self, entered: bool) {
        if entered {
            self.bound.pop();
        }
    }
}

impl<F: FnMut(syn::Lifetime, String) -> syn::Lifetime> syn::fold::Fold for FreeLifetimeFold<F> {
    fn fold_lifetime(&mut self, lifetime: syn::Lifetime) -> syn::Lifetime {
        let name = lifetime.to_string();
        if self.bound.iter().rev().any(|scope| scope.contains(&name)) {
            lifetime
        } else {
            (self.action)(lifetime, name)
        }
    }

    fn fold_trait_bound(&mut self, node: syn::TraitBound) -> syn::TraitBound {
        let entered = self.enter(node.lifetimes.as_ref());
        let node = syn::fold::fold_trait_bound(self, node);
        self.leave(entered);
        node
    }

    fn fold_type_bare_fn(&mut self, node: syn::TypeBareFn) -> syn::TypeBareFn {
        let entered = self.enter(node.lifetimes.as_ref());
        let node = syn::fold::fold_type_bare_fn(self, node);
        self.leave(entered);
        node
    }

    fn fold_predicate_type(&mut self, node: syn::PredicateType) -> syn::PredicateType {
        let entered = self.enter(node.lifetimes.as_ref());
        let node = syn::fold::fold_predicate_type(self, node);
        self.leave(entered);
        node
    }

    fn fold_expr_closure(&mut self, node: syn::ExprClosure) -> syn::ExprClosure {
        let entered = self.enter(node.lifetimes.as_ref());
        let node = syn::fold::fold_expr_closure(self, node);
        self.leave(entered);
        node
    }
}

pub(crate) fn lifetime_occurrences(rust_type: &str) -> Option<Vec<String>> {
    let ty = syn::parse_str::<syn::Type>(rust_type).ok()?;
    let mut occurrences = Vec::new();
    syn::fold::Fold::fold_type(
        &mut FreeLifetimeFold {
            action: |lifetime: syn::Lifetime, name| {
                occurrences.push(name);
                lifetime
            },
            bound: Vec::new(),
        },
        ty,
    );
    Some(occurrences)
}

pub(crate) fn bound_lifetime_occurrences(rust_bound: &str) -> Option<Vec<String>> {
    let parameter = syn::parse_str::<syn::TypeParam>(&format!("__Native: {rust_bound}")).ok()?;
    let mut occurrences = Vec::new();
    syn::fold::Fold::fold_type_param(
        &mut FreeLifetimeFold {
            action: |lifetime: syn::Lifetime, name| {
                occurrences.push(name);
                lifetime
            },
            bound: Vec::new(),
        },
        parameter,
    );
    Some(occurrences)
}

/// Native binder names remain in semantic lender contracts, but source storage
/// has no Rust lifetime declarations. Infer its free lifetimes at the use site;
/// preserve static lifetimes and higher-ranked binders in nested native types.
pub(crate) fn source_storage_rust_type(rust_type: String) -> String {
    let ty = syn::parse_str::<syn::Type>(&rust_type)
        .expect("concrete projected storage has a valid native Rust type");
    let mut changed = false;
    let ty = syn::fold::Fold::fold_type(
        &mut FreeLifetimeFold {
            action: |lifetime: syn::Lifetime, name: String| {
                if matches!(name.as_str(), "'static" | "'_") {
                    lifetime
                } else {
                    changed = true;
                    syn::Lifetime::new("'_", lifetime.apostrophe)
                }
            },
            bound: Vec::new(),
        },
        ty,
    );
    if changed {
        quote::ToTokens::to_token_stream(&ty).to_string()
    } else {
        rust_type
    }
}

fn substitute_rust_lifetimes(rust_type: &str, substitutions: &BTreeMap<String, String>) -> String {
    if substitutions.is_empty() {
        return rust_type.to_owned();
    }
    let mut substitution = FreeLifetimeFold {
        action: |lifetime: syn::Lifetime, name| {
            substitutions
                .get(&name)
                .and_then(|replacement| syn::parse_str(replacement).ok())
                .unwrap_or(lifetime)
        },
        bound: Vec::new(),
    };

    if let Ok(ty) = syn::parse_str::<syn::Type>(rust_type) {
        syn::fold::Fold::fold_type(&mut substitution, ty)
            .to_token_stream()
            .to_string()
    } else if let Ok(expression) = syn::parse_str::<syn::Expr>(rust_type) {
        syn::fold::Fold::fold_expr(&mut substitution, expression)
            .to_token_stream()
            .to_string()
    } else if let Ok(bound) = syn::parse_str::<syn::TypeParamBound>(rust_type) {
        syn::fold::Fold::fold_type_param_bound(&mut substitution, bound)
            .to_token_stream()
            .to_string()
    } else if let Ok(predicate) = syn::parse_str::<syn::WherePredicate>(rust_type) {
        syn::fold::Fold::fold_where_predicate(&mut substitution, predicate)
            .to_token_stream()
            .to_string()
    } else {
        rust_type.to_owned()
    }
}

fn bound_lifetimes(rust_type: &str) -> BTreeSet<String> {
    struct Lifetimes(BTreeSet<String>);
    impl syn::fold::Fold for Lifetimes {
        fn fold_bound_lifetimes(&mut self, bound: syn::BoundLifetimes) -> syn::BoundLifetimes {
            self.0.extend(
                bound
                    .lifetimes
                    .iter()
                    .filter_map(|parameter| match parameter {
                        syn::GenericParam::Lifetime(parameter) => {
                            Some(parameter.lifetime.to_string())
                        }
                        _ => None,
                    }),
            );
            syn::fold::fold_bound_lifetimes(self, bound)
        }
    }
    let Ok(ty) = syn::parse_str::<syn::Type>(rust_type) else {
        return BTreeSet::new();
    };
    let mut lifetimes = Lifetimes(BTreeSet::new());
    syn::fold::Fold::fold_type(&mut lifetimes, ty);
    lifetimes.0
}

fn without_lifetimes(
    substitutions: &BTreeMap<String, String>,
    excluded: &BTreeSet<String>,
) -> BTreeMap<String, String> {
    substitutions
        .iter()
        .filter(|(name, _)| !excluded.contains(*name))
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect()
}

fn substitute_projected_lifetimes(
    ty: &mut ProjectedType,
    substitutions: &BTreeMap<String, String>,
) {
    match ty {
        ProjectedType::InvocationScoped {
            rust_type,
            lifetimes,
            owned,
            ..
        } => {
            *rust_type = substitute_rust_lifetimes(rust_type, substitutions);
            let mut projected_lifetimes = Vec::new();
            for lifetime in lifetime_occurrences(rust_type)
                .unwrap_or_else(|| callable::rust_lifetimes(rust_type))
            {
                if lifetime != "'static" && !projected_lifetimes.contains(&lifetime) {
                    projected_lifetimes.push(lifetime);
                }
            }
            *lifetimes = projected_lifetimes;
            substitute_projected_lifetimes(owned, substitutions);
        }
        ProjectedType::Reference {
            inner, lifetime, ..
        } => {
            if let Some(lifetime) = lifetime
                && let Some(replacement) = substitutions.get(lifetime)
            {
                *lifetime = replacement.clone();
            }
            substitute_projected_lifetimes(inner, substitutions);
        }
        ProjectedType::Optional(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. }
        | ProjectedType::AsyncIterationStep(inner) => {
            substitute_projected_lifetimes(inner, substitutions);
        }
        ProjectedType::Mapping { key, value, .. } => {
            substitute_projected_lifetimes(key, substitutions);
            substitute_projected_lifetimes(value, substitutions);
        }
        ProjectedType::Tuple(items) => {
            for item in items {
                substitute_projected_lifetimes(item, substitutions);
            }
        }
        ProjectedType::Foreign { arguments, .. } => {
            for argument in arguments {
                substitute_projected_lifetimes(argument, substitutions);
            }
        }
        ProjectedType::Callback {
            rust_name,
            parameters,
            result,
            parameter_rust_types,
            native_bound,
            native_method,
            native_result,
            native_substitutions,
            ..
        } => {
            let callback_substitutions =
                without_lifetimes(substitutions, &bound_lifetimes(rust_name));
            *rust_name = substitute_rust_lifetimes(rust_name, substitutions);
            for parameter_type in parameter_rust_types {
                *parameter_type =
                    substitute_rust_lifetimes(parameter_type, &callback_substitutions);
            }
            for native in [native_bound, native_method, native_result]
                .into_iter()
                .flatten()
            {
                *native = substitute_rust_lifetimes(native, &callback_substitutions);
            }
            for value in native_substitutions.values_mut() {
                substitute_projected_lifetimes(value, &callback_substitutions);
            }
            for parameter in parameters {
                substitute_projected_lifetimes(parameter, &callback_substitutions);
            }
            substitute_projected_lifetimes(result, &callback_substitutions);
        }
        _ => {}
    }
}

fn substitute_projected_function_lifetimes(
    function: &mut ProjectedFunction,
    source: &Function,
    implementation: &Impl,
    substitutions: &BTreeMap<String, String>,
) {
    let implementation_binders = implementation
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Lifetime { .. } => Some(normalize_lifetime_name(&parameter.name)),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let method_binders = source
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Lifetime { .. } => Some(normalize_lifetime_name(&parameter.name)),
            _ => None,
        })
        .filter(|name| !implementation_binders.contains(name))
        .collect::<BTreeSet<_>>();
    let substitutions = substitutions
        .iter()
        .filter(|(name, _)| !method_binders.contains(*name))
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect::<BTreeMap<_, _>>();
    apply_function_lifetime_substitutions(function, &substitutions);
}

fn apply_function_lifetime_substitutions(
    function: &mut ProjectedFunction,
    substitutions: &BTreeMap<String, String>,
) {
    substitute_projected_lifetimes(&mut function.result, substitutions);
    for parameter in &mut function.generic_parameters {
        for bound in &mut parameter.rust_bounds {
            *bound = substitute_rust_lifetimes(bound, substitutions);
        }
        if let Some(default) = &mut parameter.default {
            substitute_projected_lifetimes(default, substitutions);
        }
    }
    for parameter in &mut function.operation_owner_generics {
        for bound in &mut parameter.rust_bounds {
            *bound = substitute_rust_lifetimes(bound, substitutions);
        }
        if let Some(default) = &mut parameter.default {
            substitute_projected_lifetimes(default, substitutions);
        }
    }
    for bound in &mut function.receiver_bounds {
        *bound = substitute_rust_lifetimes(bound, substitutions);
    }
    for argument in &mut function.rust_generic_arguments {
        substitute_projected_lifetimes(argument, substitutions);
    }
    if let Some(destination) = &mut function.destination_result {
        for parameter in &mut destination.parameters {
            for bound in &mut parameter.rust_bounds {
                *bound = substitute_rust_lifetimes(bound, substitutions);
            }
        }
    }
    for parameter in &mut function.parameters {
        if let Some(lifetime) = &mut parameter.borrow_lifetime
            && let Some(replacement) = substitutions.get(lifetime)
        {
            *lifetime = replacement.clone();
        }
        substitute_projected_lifetimes(&mut parameter.ty, substitutions);
    }
    if let Some(lifetime) = &mut function.receiver_lifetime
        && let Some(replacement) = substitutions.get(lifetime)
    {
        *lifetime = replacement.clone();
    }
}

fn receiver_lifetime_substitutions(
    declared_owner: &str,
    receiver: &str,
    owner_arguments: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>, String> {
    let declared_owner = crate::rust_ir::instantiate_rust_generics(declared_owner, owner_arguments);
    let (Ok(declared), Ok(actual)) = (
        syn::parse_str::<syn::Type>(&declared_owner),
        syn::parse_str::<syn::Type>(receiver),
    ) else {
        return Err("cannot parse the projected receiver's lifetime contract".to_owned());
    };
    let erased = |ty| {
        syn::fold::Fold::fold_type(
            &mut FreeLifetimeFold {
                action: |lifetime: syn::Lifetime, _| syn::Lifetime::new("'_", lifetime.apostrophe),
                bound: Vec::new(),
            },
            ty,
        )
    };
    let declared_shape = erased(declared);
    let actual_shape = erased(actual);
    if quote::ToTokens::to_token_stream(&declared_shape).to_string()
        != quote::ToTokens::to_token_stream(&actual_shape).to_string()
    {
        return Ok(BTreeMap::new());
    }
    let (Some(source), Some(target)) = (
        lifetime_occurrences(&declared_owner),
        lifetime_occurrences(receiver),
    ) else {
        return Err("cannot inspect the projected receiver's lifetime contract".to_owned());
    };
    let mut substitutions = BTreeMap::new();
    for (source, target) in source.into_iter().zip(target) {
        if source == "'static" && target != "'static" {
            return Err("the receiver does not satisfy its owner's static lifetime".to_owned());
        }
        if source == "'_" || source == "'static" {
            continue;
        }
        if substitutions
            .get(&source)
            .is_some_and(|previous| previous != &target)
        {
            return Err(
                "one receiver owner lifetime would bind to distinct lender lifetimes".to_owned(),
            );
        }
        substitutions.insert(source, target);
    }
    substitutions.retain(|source, target| source != target);
    Ok(substitutions)
}

pub(crate) fn instantiate_receiver_lifetimes<'a>(
    function: &'a ProjectedFunction,
    receiver: &str,
    owner_arguments: &BTreeMap<String, String>,
) -> Result<std::borrow::Cow<'a, ProjectedFunction>, String> {
    let Some(owner) = &function.native_owner else {
        return Ok(std::borrow::Cow::Borrowed(function));
    };
    let substitutions = receiver_lifetime_substitutions(owner, receiver, owner_arguments)?;
    if substitutions.is_empty() {
        return Ok(std::borrow::Cow::Borrowed(function));
    }
    let mut instantiated = function.clone();
    apply_function_lifetime_substitutions(&mut instantiated, &substitutions);
    if let Some(owner) = &mut instantiated.native_owner {
        *owner = substitute_rust_lifetimes(owner, &substitutions);
    }
    if let Some(path) = &mut instantiated.native_path {
        *path = substitute_rust_lifetimes(path, &substitutions);
    }
    Ok(std::borrow::Cow::Owned(instantiated))
}

pub(crate) fn instantiate_receiver_result_lifetimes(
    result: &mut ProjectedType,
    declared_owner: &str,
    receiver: &str,
    owner_arguments: &BTreeMap<String, String>,
) -> Result<(), String> {
    let substitutions = receiver_lifetime_substitutions(declared_owner, receiver, owner_arguments)?;
    substitute_projected_lifetimes(result, &substitutions);
    Ok(())
}

fn impl_has_nominal_owner<'a>(mut ty: &'a Type, index: &'a HashMap<Id, Item>) -> bool {
    loop {
        match ty {
            Type::BorrowedRef { type_, .. } => ty = type_,
            Type::ResolvedPath(path) => match index.get(&path.id).map(|item| &item.inner) {
                Some(ItemEnum::TypeAlias(alias)) => ty = &alias.type_,
                _ => return true,
            },
            Type::Generic(_) | Type::QualifiedPath { .. } => return true,
            _ => return false,
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "rustdoc impl traversal keeps one shared context for trait, inherent, and source-backed member decisions"
)]
pub(super) fn project_methods(
    dependency: &RustDependency,
    impl_ids: &[Id],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    owner_rust_path: &str,
    owner_generics: &BTreeMap<String, ProjectedType>,
    allow_lifetime_output: bool,
    source_constants: &mut SourceConstantCache,
    scoped_owner_keys: &mut BTreeSet<String>,
    scoped_owners: &mut Vec<ProjectedItem>,
) -> ProjectedMethods {
    let mut candidates = Vec::new();
    let mut trait_methods = Vec::new();
    let mut constants = Vec::new();
    let mut declined = Vec::new();
    for impl_id in impl_ids {
        let Some(Item {
            inner: ItemEnum::Impl(implementation),
            ..
        }) = index.get(impl_id)
        else {
            continue;
        };
        if implementation.is_negative || implementation.is_synthetic {
            continue;
        }
        // Rustdoc can list an impl because the nominal type is a trait argument,
        // not Self (for example ColumnIndex<Row> for &str). Such an impl cannot
        // contribute methods to this nominal owner's source class.
        if matches!(
            owner_generics.get("Self"),
            Some(ProjectedType::Foreign { .. })
        ) && !impl_has_nominal_owner(&implementation.for_, index)
        {
            let reason = format!(
                "implementation's non-nominal Self cannot supply members of projected owner `{owner_rust_path}`"
            );
            declined.extend(implementation.items.iter().filter_map(|id| {
                let item = index.get(id)?;
                let name = item.name.as_ref()?;
                matches!(item.inner, ItemEnum::Function(_)).then(|| (name.clone(), reason.clone()))
            }));
            continue;
        }
        let inherent = implementation.trait_.is_none();
        let mut implementation_generics =
            aliases::implementation_owner_generics(implementation, index, owner_generics);
        let native_owner = if matches!(implementation.for_, Type::Generic(_))
            && implementation.blanket_impl.is_some()
        {
            owner_rust_path.to_owned()
        } else {
            render_rust_type(&implementation.for_, index, paths, &implementation_generics)
                .unwrap_or_else(|_| owner_rust_path.to_owned())
        };
        let self_type = match &implementation.for_ {
            Type::BorrowedRef {
                is_mutable: true,
                type_,
                ..
            } if !inherent => type_.as_ref(),
            _ => &implementation.for_,
        };
        let mut scoped_lifetime_substitutions = BTreeMap::new();
        if let Ok(owner) = project_type(self_type, index, paths, &implementation_generics) {
            let scoped_self = owner_generics
                .get("Self")
                .filter(|scoped| scoped_self_matches_owner(scoped, &owner));
            scoped_lifetime_substitutions = implementation_lifetime_substitutions(
                implementation,
                self_type,
                scoped_self,
                index,
                paths,
            );
            implementation_generics
                .insert("Self".to_owned(), scoped_self.cloned().unwrap_or(owner));
        }
        if let Some(Type::Generic(generic)) = &implementation.blanket_impl
            && let Some(owner) = owner_generics.get("Self")
        {
            implementation_generics.insert(generic.clone(), owner.clone());
        }
        let mut method_ids = implementation.items.clone();
        if !inherent
            && let (Some(trait_reference), Some(public_trait_path)) = (
                implementation.trait_.as_ref(),
                implementation
                    .trait_
                    .as_ref()
                    .and_then(|trait_| public_paths.get(&trait_.id)),
            )
            && paths.contains_key(&trait_reference.id)
            && !public_trait_path.is_empty()
            && let Some(Item {
                inner: ItemEnum::Trait(trait_declaration),
                ..
            }) = index.get(&trait_reference.id)
        {
            for method_name in &implementation.provided_trait_methods {
                if implementation.items.iter().any(|id| {
                    index
                        .get(id)
                        .is_some_and(|item| item.name.as_deref() == Some(method_name))
                }) {
                    continue;
                }
                let Some((method_id, _)) = trait_declaration.items.iter().find_map(|method_id| {
                    let item = index.get(method_id)?;
                    (item.name.as_deref() == Some(method_name)
                        && matches!(&item.inner, ItemEnum::Function(function) if function.has_body))
                    .then_some((*method_id, item))
                }) else {
                    continue;
                };
                if !implementation.items.contains(&method_id) && !method_ids.contains(&method_id) {
                    method_ids.push(method_id);
                }
            }
        }
        for item_id in &implementation.items {
            let Some(Item {
                name: Some(name),
                inner:
                    ItemEnum::AssocType {
                        type_: Some(type_), ..
                    },
                ..
            }) = index.get(item_id)
            else {
                continue;
            };
            if let Ok(projected) = project_type(type_, index, paths, &implementation_generics) {
                implementation_generics.insert(format!("Self::{name}"), projected);
            }
        }
        for method_id in &method_ids {
            let Some(item) = index.get(method_id) else {
                continue;
            };
            if inherent && item.visibility != Visibility::Public {
                continue;
            }
            let Some(name) = item.name.as_deref() else {
                continue;
            };
            if let ItemEnum::AssocConst { type_, value, .. } = &item.inner {
                if !inherent {
                    continue;
                }
                match project_type(type_, index, paths, &implementation_generics) {
                    Ok(ty) => {
                        let source_expression = value
                            .as_deref()
                            .filter(|value| *value != "_")
                            .map(str::to_owned)
                            .or_else(|| {
                                source_constant_expression(
                                    item,
                                    &native_owner,
                                    name,
                                    source_constants,
                                )
                            });
                        let terrane_value = source_expression
                            .as_deref()
                            .and_then(project_rust_constant_expression);
                        constants.push(ProjectedConstant {
                            name: projected_constant_name(name),
                            rust_name: name.to_owned(),
                            rust_path: if native_owner.contains('<') {
                                format!("<{native_owner}>::{name}")
                            } else {
                                format!("{native_owner}::{name}")
                            },
                            ty,
                            source_expression,
                            terrane_value,
                        });
                    }
                    Err(reason) => declined.push((name.to_owned(), reason)),
                }
                continue;
            }
            let ItemEnum::Function(function) = &item.inner else {
                declined.push((
                    name.to_owned(),
                    "item kind has no Terrane member projection".to_owned(),
                ));
                continue;
            };
            let mut enriched_function = function.clone();
            if implementation.blanket_impl.is_some() {
                for parameter in &implementation.generics.params {
                    if !enriched_function
                        .generics
                        .params
                        .iter()
                        .any(|candidate| candidate.name == parameter.name)
                    {
                        enriched_function.generics.params.push(parameter.clone());
                    }
                }
                enriched_function
                    .generics
                    .where_predicates
                    .extend(implementation.generics.where_predicates.iter().cloned());
            }
            let function = &enriched_function;
            if !inherent {
                let Some(trait_) = implementation.trait_.as_ref() else {
                    continue;
                };
                let Some((trait_path, local_trait)) = implementation_trait_path(trait_, paths)
                else {
                    declined.push((
                        name.to_owned(),
                        "trait method has no canonical Rust trait path".to_owned(),
                    ));
                    continue;
                };
                let public_trait_path = public_paths
                    .get(&trait_.id)
                    .cloned()
                    .unwrap_or_else(|| trait_path.clone());
                let mut trait_render_generics = implementation_generics.clone();
                for parameter in &implementation.generics.params {
                    if matches!(parameter.kind, GenericParamDefKind::Type { .. }) {
                        trait_render_generics
                            .entry(parameter.name.clone())
                            .or_insert_with(|| ProjectedType::Generic(parameter.name.clone()));
                    }
                }
                let rendered_trait =
                    render_resolved_path(trait_, index, paths, &trait_render_generics)
                        .unwrap_or_else(|_| trait_path.clone());
                let trait_implementation_path = rendered_trait.find('<').map_or_else(
                    || trait_path.clone(),
                    |start| format!("{trait_path}{}", &rendered_trait[start..]),
                );
                match project_method_function(
                    function,
                    index,
                    paths,
                    public_paths,
                    Some(name),
                    &implementation_generics,
                    allow_lifetime_output,
                ) {
                    Ok(mut method) => {
                        substitute_projected_function_lifetimes(
                            &mut method,
                            function,
                            implementation,
                            &scoped_lifetime_substitutions,
                        );
                        data::rebind_method_owner(
                            &mut method,
                            implementation,
                            index,
                            paths,
                            &implementation_generics,
                        );
                        method.native_owner = Some(substitute_rust_lifetimes(
                            &native_owner,
                            &scoped_lifetime_substitutions,
                        ));
                        let trait_rust_path = if local_trait {
                            let package_root = trait_implementation_path
                                .split("::")
                                .next()
                                .unwrap_or_default();
                            let dependency_root =
                                owner_rust_path.split("::").next().unwrap_or_default();
                            rewrite_rust_bound_root(
                                &trait_implementation_path,
                                package_root,
                                dependency_root,
                            )
                        } else {
                            trait_implementation_path.replace('-', "_")
                        };
                        let mut call_lifetimes = scoped_lifetime_substitutions.clone();
                        if call_lifetimes.is_empty() {
                            for parameter in &implementation.generics.params {
                                if matches!(&parameter.kind, GenericParamDefKind::Lifetime { .. }) {
                                    call_lifetimes.insert(
                                        normalize_lifetime_name(&parameter.name),
                                        "'_".to_owned(),
                                    );
                                }
                            }
                        }
                        let native_owner =
                            substitute_rust_lifetimes(&native_owner, &call_lifetimes);
                        let trait_rust_path = substitute_rust_lifetimes(
                            &canonicalize_rust_path(&trait_rust_path),
                            &call_lifetimes,
                        );
                        let invocation = format!("<{native_owner} as {trait_rust_path}>::{name}");
                        method.native_path =
                            Some(substitute_rust_lifetimes(&invocation, &call_lifetimes));
                        if implementation.blanket_impl.is_some()
                            && !(method.destination_result.is_some()
                                && method.receiver.is_some_and(Receiver::consumes)
                                && method.parameters.is_empty())
                        {
                            continue;
                        }
                        collect_method_scoped_owner(
                            dependency,
                            function,
                            &method.result,
                            index,
                            paths,
                            public_paths,
                            source_constants,
                            scoped_owner_keys,
                            scoped_owners,
                        );
                        trait_methods.push((
                            trait_implementation_path,
                            public_trait_path,
                            local_trait,
                            item.docs.clone(),
                            method,
                        ));
                    }
                    Err(reason) => {
                        if implementation.blanket_impl.is_none()
                            && !is_internal_rust_protocol_method(&trait_path, name)
                        {
                            declined.push((name.to_owned(), reason));
                        }
                    }
                }
                continue;
            }
            let static_owner_generics = inherent
                && implementation.blanket_impl.is_none()
                && !function.sig.inputs.iter().any(|(name, _)| name == "self");
            let method_generics = if static_owner_generics {
                disambiguate_static_method_owner_generics(
                    implementation,
                    function,
                    implementation_generics.clone(),
                    index,
                    paths,
                )
            } else {
                implementation_generics.clone()
            };
            let method_native_owner = if static_owner_generics {
                render_rust_type(&implementation.for_, index, paths, &method_generics)
                    .unwrap_or_else(|_| native_owner.clone())
            } else if let Some(scoped @ ProjectedType::InvocationScoped { .. }) =
                method_generics.get("Self")
            {
                // An `impl Owner<'_>` can return `Self` spelled with the
                // nominal owner's binder by Rustdoc. Keep one receiver binder
                // vocabulary so call-site instantiation also closes that result.
                scoped.rust_type()
            } else {
                native_owner.clone()
            };
            match project_method_function(
                function,
                index,
                paths,
                public_paths,
                Some(name),
                &method_generics,
                allow_lifetime_output,
            ) {
                Ok(mut method) => {
                    substitute_projected_function_lifetimes(
                        &mut method,
                        function,
                        implementation,
                        &scoped_lifetime_substitutions,
                    );
                    data::rebind_method_owner(
                        &mut method,
                        implementation,
                        index,
                        paths,
                        &method_generics,
                    );
                    method.native_owner = Some(substitute_rust_lifetimes(
                        &method_native_owner,
                        &scoped_lifetime_substitutions,
                    ));
                    collect_method_scoped_owner(
                        dependency,
                        function,
                        &method.result,
                        index,
                        paths,
                        public_paths,
                        source_constants,
                        scoped_owner_keys,
                        scoped_owners,
                    );
                    match project_operation_owner_generics(
                        implementation,
                        index,
                        paths,
                        &method_generics,
                        !static_owner_generics,
                    ) {
                        Ok(owner_generics) => method.operation_owner_generics = owner_generics,
                        Err(reason) if static_owner_generics => {
                            declined.push((name.to_owned(), reason));
                            continue;
                        }
                        // Instance calls retain their native receiver and full
                        // graph obligations. Keep any representable owner binders
                        // above even when other impl predicates cannot detach;
                        // only an unrepresentable binder itself defers this plan.
                        Err(_) => {}
                    }
                    candidates.push(method);
                }
                Err(reason) => declined.push((name.to_owned(), reason)),
            }
        }
    }
    let mut methods = Vec::new();
    while let Some(method) = candidates.pop() {
        let name = method.name.clone();
        let mut matching = vec![method];
        let mut candidate_index = 0;
        while candidate_index < candidates.len() {
            if candidates[candidate_index].name == name {
                matching.push(candidates.swap_remove(candidate_index));
            } else {
                candidate_index += 1;
            }
        }
        if matching.len() == 1 {
            methods.push(matching.pop().expect("one matching method"));
            continue;
        }
        declined.extend(matching.into_iter().map(|method| {
            (
                method.name,
                "multiple inherent methods with the same name are not projectable".to_owned(),
            )
        }));
    }
    methods.sort_by(|left, right| left.name.cmp(&right.name));
    constants.sort_by(|left, right| left.name.cmp(&right.name));
    (methods, trait_methods, constants, declined)
}
fn is_internal_rust_protocol_method(trait_path: &str, method: &str) -> bool {
    (method == "fmt"
        && (trait_path.ends_with("::fmt::Debug") || trait_path.ends_with("::fmt::Display")))
        || (method == "hash" && trait_path.ends_with("::hash::Hash"))
}
#[expect(
    clippy::too_many_arguments,
    reason = "shared Rustdoc projection context"
)]
pub(super) fn project_chain_owner(
    dependency: &RustDependency,
    function: &Function,
    result: &ProjectedType,
    invocation_scoped: bool,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    source_constants: &mut SourceConstantCache,
    scoped_owner_keys: &mut BTreeSet<String>,
    scoped_owners: &mut Vec<ProjectedItem>,
) -> Option<ProjectedItem> {
    let output = function.sig.output.as_ref()?;
    let owned_result = match result {
        ProjectedType::InvocationScoped { owned, .. } => owned.as_ref(),
        result => result,
    };
    let owner = project_chain_owner_item(
        dependency,
        output,
        owned_result,
        result,
        invocation_scoped,
        index,
        paths,
        public_paths,
        source_constants,
        scoped_owner_keys,
        scoped_owners,
    )?;
    Some(owner)
}

#[expect(
    clippy::too_many_arguments,
    reason = "method-result owner collection reuses the shared projection context"
)]
fn collect_method_scoped_owner(
    dependency: &RustDependency,
    function: &Function,
    result: &ProjectedType,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    source_constants: &mut SourceConstantCache,
    scoped_owner_keys: &mut BTreeSet<String>,
    scoped_owners: &mut Vec<ProjectedItem>,
) {
    let invocation_scoped = matches!(result, ProjectedType::InvocationScoped { .. });
    let owner = project_chain_owner(
        dependency,
        function,
        result,
        invocation_scoped,
        index,
        paths,
        public_paths,
        source_constants,
        scoped_owner_keys,
        scoped_owners,
    );
    if let Some(owner) = owner {
        scoped_owners.push(owner);
    }
}

fn resolved_owner_structure<'a>(
    output: &'a Type,
    index: &'a HashMap<Id, Item>,
) -> Option<(&'a rustdoc_types::Path, &'a rustdoc_types::Struct)> {
    let Type::ResolvedPath(path) = output else {
        return None;
    };
    let ItemEnum::Struct(structure) = &index.get(&path.id)?.inner else {
        return None;
    };
    Some((path, structure))
}

fn requires_chain_owner(
    invocation_scoped: bool,
    result: &ProjectedType,
    structure: &rustdoc_types::Struct,
) -> bool {
    invocation_scoped
        || result.contains_opaque()
        || structure
            .generics
            .params
            .iter()
            .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }))
}

#[expect(
    clippy::too_many_arguments,
    reason = "chain-owner projection needs the resolved dependency and all Rustdoc path contexts"
)]
fn project_chain_owner_item(
    dependency: &RustDependency,
    output: &Type,
    result: &ProjectedType,
    scoped_self: &ProjectedType,
    invocation_scoped: bool,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    source_constants: &mut SourceConstantCache,
    scoped_owner_keys: &mut BTreeSet<String>,
    scoped_owners: &mut Vec<ProjectedItem>,
) -> Option<ProjectedItem> {
    let (path, structure) = resolved_owner_structure(output, index)?;
    let ProjectedType::Foreign {
        rust_path,
        name,
        arguments,
        ..
    } = result
    else {
        return None;
    };
    if !requires_chain_owner(invocation_scoped, result, structure) {
        return None;
    }
    let mut owner_generics = structure
        .generics
        .params
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .zip(arguments)
        .map(|(parameter, argument)| (parameter.name.clone(), argument.clone()))
        .collect::<BTreeMap<_, _>>();
    let canonical_self;
    let scoped_self = if invocation_scoped {
        canonical_self = canonical_scoped_owner(
            output,
            scoped_self,
            &structure.generics,
            &owner_generics,
            index,
            paths,
        )?;
        &canonical_self
    } else {
        scoped_self
    };
    let owner_key = canonicalize_rust_path(&scoped_self.rust_type());
    if !scoped_owner_keys.insert(owner_key) {
        return None;
    }
    owner_generics.insert("Self".to_owned(), scoped_self.clone());
    let generic_parameters = super::items::projected_nominal_generic_parameters(
        &structure.generics,
        &owner_generics,
        index,
        paths,
    )
    .ok()?;
    let (methods, cloneable, expression_only) = project_chain_owner_methods(
        dependency,
        structure,
        rust_path,
        &owner_generics,
        scoped_self,
        invocation_scoped,
        index,
        paths,
        public_paths,
        source_constants,
        scoped_owner_keys,
        scoped_owners,
    )?;
    let source_path = paths.get(&path.id)?.path.clone();
    let namespace = dependency_namespace(
        dependency,
        &source_path[..source_path.len().saturating_sub(1)],
    );
    Some(ProjectedItem {
        namespace,
        name: name.clone(),
        rust_path: rust_path.clone(),
        docs: Some(
            if expression_only {
                "chain-only; value must terminate within one expression"
            } else {
                "native lifetime-bearing owner; each call retains its actual lenders"
            }
            .to_owned(),
        ),
        kind: ProjectedKind::ForeignType {
            constructor: None,
            methods,
            static_methods: Vec::new(),
            constants: Vec::new(),
            boundary: ProjectedBoundaryCapabilities::default(),
            fields: Vec::new(),
            borrowed_view: false,
            native_view_type: invocation_scoped.then(|| scoped_self.rust_type()),
            enum_payload: None,
            generic_parameters,
            displayable: false,
            cloneable,
            copyable: false,
            send: false,
            sync: false,
        },
    })
}

fn canonical_scoped_owner(
    output: &Type,
    scoped_self: &ProjectedType,
    generics: &rustdoc_types::Generics,
    owner_generics: &BTreeMap<String, ProjectedType>,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<ProjectedType> {
    if !matches!(scoped_self, ProjectedType::InvocationScoped { .. }) {
        return None;
    }
    let mut canonical_output = output.clone();
    if let Type::ResolvedPath(path) = &mut canonical_output
        && let Some(rustdoc_types::GenericArgs::AngleBracketed { args, .. }) =
            path.args.as_deref_mut()
    {
        let declared = generics.params.iter().filter_map(|parameter| {
            matches!(parameter.kind, GenericParamDefKind::Lifetime { .. })
                .then_some(&parameter.name)
        });
        for (argument, declared) in args
            .iter_mut()
            .filter_map(|argument| match argument {
                rustdoc_types::GenericArg::Lifetime(lifetime) => Some(lifetime),
                _ => None,
            })
            .zip(declared)
        {
            argument.clone_from(declared);
        }
    }
    let mut owner = scoped_self.clone();
    if let ProjectedType::InvocationScoped {
        rust_type,
        lifetimes,
        ..
    } = &mut owner
    {
        *rust_type = render_rust_type(&canonical_output, index, paths, owner_generics).ok()?;
        *lifetimes = lifetime_occurrences(rust_type)?
            .into_iter()
            .filter(|lifetime| lifetime != "'static")
            .collect();
    }
    Some(owner)
}

#[expect(
    clippy::too_many_arguments,
    reason = "chain-owner method projection reuses shared Rustdoc contexts"
)]
fn project_chain_owner_methods(
    dependency: &RustDependency,
    structure: &rustdoc_types::Struct,
    rust_path: &str,
    owner_generics: &BTreeMap<String, ProjectedType>,
    scoped_self: &ProjectedType,
    invocation_scoped: bool,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    source_constants: &mut SourceConstantCache,
    scoped_owner_keys: &mut BTreeSet<String>,
    scoped_owners: &mut Vec<ProjectedItem>,
) -> Option<(Vec<ProjectedFunction>, bool, bool)> {
    let has_type_parameters = structure
        .generics
        .params
        .iter()
        .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }));
    let (mut methods, scoped_methods, _, _) = project_methods(
        dependency,
        &structure.impls,
        index,
        paths,
        public_paths,
        rust_path,
        owner_generics,
        true,
        source_constants,
        scoped_owner_keys,
        scoped_owners,
    );
    let projected_clone_method = scoped_methods.iter().any(
        |(trait_implementation_path, public_trait_path, _, _, method)| {
            let trait_path = canonicalize_rust_path(trait_implementation_path);
            method.name == "clone"
                && (trait_path == "Clone"
                    || trait_path.ends_with("::clone::Clone")
                    || public_trait_path == "Clone"
                    || public_trait_path.ends_with("::clone::Clone"))
        },
    );
    let cloneable = !has_type_parameters
        && projected_clone_method
        && super::rustdoc_support::implements_trait(
            &structure.impls,
            index,
            paths,
            "core::clone::Clone",
        );
    methods.extend(
        scoped_methods
            .into_iter()
            .map(|(_, _, _, _, method)| method),
    );
    let expression_only = !invocation_scoped
        || matches!(
            scoped_self,
            ProjectedType::InvocationScoped {
                expression_scoped: true,
                ..
            }
        );
    methods.retain_mut(|method| {
        if method.receiver.is_none() {
            return false;
        }
        if expression_only {
            method.chain_role = Some(if method.result == *scoped_self {
                ChainRole::Continue
            } else {
                ChainRole::Terminal
            });
        }
        true
    });
    if !invocation_scoped
        && methods
            .iter()
            .all(|method| method.chain_role != Some(ChainRole::Terminal))
    {
        return None;
    }
    Some((methods, cloneable, expression_only))
}

pub(super) fn resolved_nominal_id(ty: &Type, index: &HashMap<Id, Item>) -> Option<Id> {
    let Type::ResolvedPath(path) = ty else {
        return None;
    };
    match index.get(&path.id).map(|item| &item.inner) {
        Some(ItemEnum::TypeAlias(alias)) => resolved_nominal_id(&alias.type_, index),
        Some(_) => Some(path.id),
        None => None,
    }
}

pub(super) fn alias_type_substitutions(
    ty: &Type,
    parameters: &[GenericParamDef],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<BTreeMap<String, ProjectedType>, String> {
    let arguments = type_arguments(ty);
    let mut substitutions = generics.clone();
    for (position, parameter) in parameters
        .iter()
        .filter(|parameter| matches!(parameter.kind, GenericParamDefKind::Type { .. }))
        .enumerate()
    {
        let projected = if let Some(argument) = arguments.get(position) {
            project_type(argument, index, paths, generics)?
        } else if let GenericParamDefKind::Type {
            default: Some(default),
            ..
        } = &parameter.kind
        {
            project_type(default, index, paths, &substitutions)?
        } else {
            return Err(format!(
                "type alias parameter `{}` has no argument or default",
                parameter.name
            ));
        };
        substitutions.insert(parameter.name.clone(), projected);
    }
    Ok(substitutions)
}

pub(super) fn expand_output_alias(
    mut output: Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    mut generics: BTreeMap<String, ProjectedType>,
) -> Result<(Type, BTreeMap<String, ProjectedType>), String> {
    let mut visited = BTreeSet::new();
    loop {
        let Type::ResolvedPath(path) = &output else {
            return Ok((output, generics));
        };
        if !visited.insert(path.id) {
            return Err("recursive Rust type alias in projected output".to_owned());
        }
        let Some(Item {
            inner: ItemEnum::TypeAlias(alias),
            ..
        }) = index.get(&path.id)
        else {
            if let Some(summary) = paths.get(&path.id)
                && summary.kind == rustdoc_types::ItemKind::TypeAlias
                && super::functions::native_output_container(
                    summary.path.iter().map(String::as_str),
                )
                .is_none()
            {
                return Err(format!(
                    "output type alias `{}` has no available unambiguous definition in its defining Rustdoc document",
                    summary.path.join("::")
                ));
            }
            return Ok((output, generics));
        };
        generics =
            alias_type_substitutions(&output, &alias.generics.params, index, paths, &generics)?;
        output = alias.type_.clone();
    }
}
#[cfg(test)]
mod tests;
