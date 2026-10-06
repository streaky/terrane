//! Rustdoc partial projection decisions.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use rustdoc_types::{
    AssocItemConstraintKind, Function, GenericArg, GenericArgs, GenericBound, GenericParamDefKind,
    Id, Item, ItemEnum, ItemKind, ItemSummary, Term, Type, WherePredicate,
};

use super::{
    PartialCallbackShape, PartialProjection, ProjectedType, RustDependency, extern_rust_path,
    render_generic_bound, render_rust_type, resolved_path_name,
};

pub(super) fn partial_projection(
    item: &Item,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Option<PartialProjection> {
    partial_projection_inner(item, index, paths, &mut HashSet::new())
}

fn partial_projection_inner(
    item: &Item,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    visited: &mut HashSet<Id>,
) -> Option<PartialProjection> {
    if !visited.insert(item.id) {
        return None;
    }
    let name = item.name.as_deref().unwrap_or("<anonymous>");
    match &item.inner {
        ItemEnum::Function(function) => {
            partial_function_projection(name, function, index, paths).ok()
        }
        ItemEnum::Struct(struct_) => Some(partial_nominal_type(name, "struct", &struct_.generics)),
        ItemEnum::Enum(enum_) => Some(partial_nominal_type(name, "enum", &enum_.generics)),
        ItemEnum::TypeAlias(alias) => {
            Some(partial_nominal_type(name, "type alias", &alias.generics))
        }
        ItemEnum::Trait(trait_) => Some(PartialProjection::NominalType {
            declaration: format!("interface {}", name.replace('_', "-")),
            native_kind: "trait".to_owned(),
            generic_parameters: trait_
                .generics
                .params
                .iter()
                .map(|parameter| parameter.name.clone())
                .collect(),
        }),
        ItemEnum::Use(import) => import
            .id
            .as_ref()
            .and_then(|id| index.get(id))
            .and_then(|target| partial_projection_inner(target, index, paths, visited))
            .or_else(|| partial_nominal_from_summary(name, paths.get(&item.id)?))
            .map(|mut projection| {
                if let PartialProjection::NominalType { declaration, .. } = &mut projection {
                    let kind = declaration
                        .split_once(' ')
                        .map_or("class", |(kind, _)| kind);
                    *declaration = format!("{kind} {}", name.replace('_', "-"));
                }
                projection
            }),
        ItemEnum::Module(_) => Some(PartialProjection::Namespace),
        _ => None,
    }
}

fn partial_nominal_type(
    name: &str,
    native_kind: &str,
    generics: &rustdoc_types::Generics,
) -> PartialProjection {
    PartialProjection::NominalType {
        declaration: format!("class {}", name.replace('_', "-")),
        native_kind: native_kind.to_owned(),
        generic_parameters: generics
            .params
            .iter()
            .map(|parameter| parameter.name.clone())
            .collect(),
    }
}

fn partial_nominal_from_summary(name: &str, summary: &ItemSummary) -> Option<PartialProjection> {
    let (declaration, native_kind) = match summary.kind {
        ItemKind::Struct => (format!("class {}", name.replace('_', "-")), "struct"),
        ItemKind::Enum => (format!("class {}", name.replace('_', "-")), "enum"),
        ItemKind::TypeAlias => (format!("class {}", name.replace('_', "-")), "type alias"),
        ItemKind::Trait => (format!("interface {}", name.replace('_', "-")), "trait"),
        _ => return None,
    };
    Some(PartialProjection::NominalType {
        declaration,
        native_kind: native_kind.to_owned(),
        generic_parameters: Vec::new(),
    })
}

pub(super) fn partial_projection_references(
    dependency: &RustDependency,
    projection: &PartialProjection,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
) -> BTreeSet<String> {
    let PartialProjection::Function {
        signature,
        generic_constraints,
        callback_shapes,
    } = projection
    else {
        return BTreeSet::new();
    };
    let mut contract = signature.clone();
    for constraint in generic_constraints {
        contract.push('\n');
        contract.push_str(constraint);
    }
    for callback in callback_shapes {
        contract.push('\n');
        contract.push_str(&callback.contract);
        for method in &callback.methods {
            contract.push('\n');
            contract.push_str(method);
        }
    }
    paths
        .iter()
        .filter(|(_, summary)| native_path_occurs_in(&contract, &summary.path.join("::")))
        .map(|(id, summary)| {
            let public_path = public_paths
                .get(id)
                .cloned()
                .unwrap_or_else(|| summary.path.join("::"));
            extern_rust_path(dependency, &public_path)
        })
        .collect()
}

fn native_path_occurs_in(contract: &str, path: &str) -> bool {
    contract.match_indices(path).any(|(start, _)| {
        let before = contract[..start].chars().next_back();
        let after = contract[start + path.len()..].chars().next();
        !before.is_some_and(|character| character.is_alphanumeric() || character == '_')
            && !after.is_some_and(|character| {
                character.is_alphanumeric() || matches!(character, '_' | ':')
            })
    })
}

fn partial_function_projection(
    name: &str,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<PartialProjection, String> {
    let generics = function
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Type { .. } => Some((
                parameter.name.clone(),
                ProjectedType::Generic(parameter.name.clone()),
            )),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    let signature = render_partial_function_signature(name, function, index, paths, &generics)?;
    let generic_constraints =
        render_partial_generic_constraints(function, index, paths, &generics)?;
    let mut callback_shapes = Vec::new();
    for (parameter, ty) in &function.sig.inputs {
        let Type::ImplTrait(bounds) = ty else {
            continue;
        };
        for bound in bounds {
            let GenericBound::TraitBound { trait_, .. } = bound else {
                continue;
            };
            let contract = render_generic_bound(bound, &[], index, paths, &generics)?;
            let Some(Item {
                inner: ItemEnum::Trait(declaration),
                ..
            }) = index.get(&trait_.id)
            else {
                continue;
            };
            let mut trait_generics = declaration
                .generics
                .params
                .iter()
                .filter_map(|parameter| match &parameter.kind {
                    GenericParamDefKind::Type { .. } => Some((
                        parameter.name.clone(),
                        ProjectedType::Generic(parameter.name.clone()),
                    )),
                    _ => None,
                })
                .collect::<BTreeMap<_, _>>();
            trait_generics.insert("Self".to_owned(), ProjectedType::Generic("Self".to_owned()));
            let methods = declaration
                .items
                .iter()
                .filter_map(|id| {
                    let method = index.get(id)?;
                    let ItemEnum::Function(function) = &method.inner else {
                        return None;
                    };
                    render_partial_function_signature(
                        method.name.as_deref().unwrap_or("<anonymous>"),
                        function,
                        index,
                        paths,
                        &trait_generics,
                    )
                    .ok()
                })
                .collect();
            callback_shapes.push(PartialCallbackShape {
                parameter: parameter.clone(),
                contract,
                methods,
            });
        }
    }
    Ok(PartialProjection::Function {
        signature,
        generic_constraints,
        callback_shapes,
    })
}

fn render_partial_function_signature(
    name: &str,
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    let parameters = function
        .generics
        .params
        .iter()
        .filter_map(|parameter| match &parameter.kind {
            GenericParamDefKind::Type {
                is_synthetic: false,
                ..
            }
            | GenericParamDefKind::Lifetime { .. }
            | GenericParamDefKind::Const { .. } => Some(parameter.name.clone()),
            GenericParamDefKind::Type {
                is_synthetic: true, ..
            } => None,
        })
        .collect::<Vec<_>>();
    let parameters = if parameters.is_empty() {
        String::new()
    } else {
        format!("<{}>", parameters.join(", "))
    };
    let inputs = function
        .sig
        .inputs
        .iter()
        .map(|(name, ty)| {
            render_partial_type(ty, index, paths, generics).map(|ty| format!("{name}: {ty}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let output = function
        .sig
        .output
        .as_ref()
        .map(|ty| render_partial_type(ty, index, paths, generics))
        .transpose()?
        .map_or_else(String::new, |ty| format!(" -> {ty}"));
    Ok(format!(
        "fn {name}{parameters}({}){output}",
        inputs.join(", ")
    ))
}

fn render_partial_generic_constraints(
    function: &Function,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<Vec<String>, String> {
    let mut constraints = Vec::new();
    for parameter in &function.generics.params {
        let GenericParamDefKind::Type {
            bounds,
            is_synthetic: false,
            ..
        } = &parameter.kind
        else {
            continue;
        };
        if !bounds.is_empty() {
            let bounds = bounds
                .iter()
                .map(|bound| render_generic_bound(bound, &[], index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            constraints.push(format!("{}: {}", parameter.name, bounds.join(" + ")));
        }
    }
    for predicate in &function.generics.where_predicates {
        let WherePredicate::BoundPredicate {
            type_,
            bounds,
            generic_params,
        } = predicate
        else {
            continue;
        };
        let ty = render_partial_type(type_, index, paths, generics)?;
        let bounds = bounds
            .iter()
            .map(|bound| render_generic_bound(bound, generic_params, index, paths, generics))
            .collect::<Result<Vec<_>, _>>()?;
        if !bounds.is_empty() {
            constraints.push(format!("{ty}: {}", bounds.join(" + ")));
        }
    }
    Ok(constraints)
}

fn render_partial_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    match ty {
        Type::ImplTrait(bounds) => {
            let bounds = bounds
                .iter()
                .map(|bound| render_generic_bound(bound, &[], index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("impl {}", bounds.join(" + ")))
        }
        Type::ResolvedPath(path) => {
            let base = resolved_path_name(path, paths);
            let Some(arguments) = path.args.as_deref() else {
                return Ok(base);
            };
            match arguments {
                GenericArgs::AngleBracketed { args, constraints } => {
                    let mut rendered = args
                        .iter()
                        .map(|argument| match argument {
                            GenericArg::Lifetime(lifetime) => Ok(lifetime.clone()),
                            GenericArg::Type(ty) => render_partial_type(ty, index, paths, generics),
                            GenericArg::Const(constant) => Ok(constant.expr.clone()),
                            GenericArg::Infer => Ok("_".to_owned()),
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    for constraint in constraints {
                        let binding = match &constraint.binding {
                            AssocItemConstraintKind::Equality(Term::Type(ty)) => format!(
                                "{} = {}",
                                constraint.name,
                                render_partial_type(ty, index, paths, generics)?
                            ),
                            AssocItemConstraintKind::Equality(Term::Constant(constant)) => {
                                format!("{} = {}", constraint.name, constant.expr)
                            }
                            AssocItemConstraintKind::Constraint(bounds) => {
                                let bounds = bounds
                                    .iter()
                                    .map(|bound| {
                                        render_generic_bound(bound, &[], index, paths, generics)
                                    })
                                    .collect::<Result<Vec<_>, _>>()?;
                                format!("{}: {}", constraint.name, bounds.join(" + "))
                            }
                        };
                        rendered.push(binding);
                    }
                    Ok(format!("{base}<{}>", rendered.join(", ")))
                }
                GenericArgs::Parenthesized { inputs, output } => {
                    let inputs = inputs
                        .iter()
                        .map(|ty| render_partial_type(ty, index, paths, generics))
                        .collect::<Result<Vec<_>, _>>()?;
                    let output = output
                        .as_ref()
                        .map(|ty| render_partial_type(ty, index, paths, generics))
                        .transpose()?
                        .map_or_else(String::new, |ty| format!(" -> {ty}"));
                    Ok(format!("{base}({}){output}", inputs.join(", ")))
                }
                GenericArgs::ReturnTypeNotation => Ok(format!("{base}(..)")),
            }
        }
        _ => render_rust_type(ty, index, paths, generics),
    }
}
