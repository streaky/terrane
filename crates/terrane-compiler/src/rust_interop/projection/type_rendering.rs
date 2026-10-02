use super::{
    AssocItemConstraintKind, BTreeMap, Digest, GenericArg, GenericArgs, Generics, HashMap, Id,
    Item, ItemEnum, ItemSummary, ProjectedType, RustdocPath, Sha256, Term, Type,
    descriptive_rust_identity, project_dyn_interface, render_generic_bound, resolved_path_name,
};
pub(super) fn nominal_generics(item: Option<&Item>) -> Option<&Generics> {
    match &item?.inner {
        ItemEnum::Struct(item) => Some(&item.generics),
        ItemEnum::Enum(item) => Some(&item.generics),
        ItemEnum::Union(item) => Some(&item.generics),
        _ => None,
    }
}

fn render_generic_arguments(
    arguments: &GenericArgs,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    match arguments {
        GenericArgs::AngleBracketed { args, constraints } => {
            if !constraints.is_empty() {
                return Err(
                    "nested associated generic constraints have no stable projection".to_owned(),
                );
            }
            let rendered = args
                .iter()
                .map(|argument| match argument {
                    GenericArg::Lifetime(lifetime) => Ok(lifetime.clone()),
                    GenericArg::Type(ty) => render_rust_type(ty, index, paths, generics),
                    GenericArg::Const(constant) => Ok(constant.expr.clone()),
                    GenericArg::Infer => {
                        Err("inferred generic argument has no stable projection".to_owned())
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(format!("<{}>", rendered.join(", ")))
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let inputs = inputs
                .iter()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            let output = output
                .as_ref()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .transpose()?
                .map_or_else(String::new, |ty| format!(" -> {ty}"));
            Ok(format!("({}){output}", inputs.join(", ")))
        }
        GenericArgs::ReturnTypeNotation => {
            Err("return-type notation has no stable projection".to_owned())
        }
    }
}

pub(super) fn canonicalize_rust_path(path: &str) -> String {
    match path {
        "alloc::collections::btree::map::BTreeMap" => "std::collections::BTreeMap".to_owned(),
        "core::task::wake::Context" => "core::task::Context".to_owned(),
        "alloc::collections::btree::set::BTreeSet" => "std::collections::BTreeSet".to_owned(),
        "core::ops::function::Fn" => "std::ops::Fn".to_owned(),
        "core::ops::function::FnMut" => "std::ops::FnMut".to_owned(),
        "core::ops::function::FnOnce" => "std::ops::FnOnce".to_owned(),
        "core::str::traits::FromStr" | "std::str::traits::FromStr" => {
            "std::str::FromStr".to_owned()
        }
        "core::net::socket_addr::SocketAddr" | "core::net::SocketAddr" => {
            "std::net::SocketAddr".to_owned()
        }
        "core::net::ip_addr::IpAddr" | "core::net::IpAddr" => "std::net::IpAddr".to_owned(),
        "core::net::ip_addr::Ipv4Addr" | "core::net::Ipv4Addr" => "std::net::Ipv4Addr".to_owned(),
        "core::net::ip_addr::Ipv6Addr" | "core::net::Ipv6Addr" => "std::net::Ipv6Addr".to_owned(),
        path => path
            .strip_prefix("alloc::")
            .map_or_else(|| path.to_owned(), |path| format!("std::{path}")),
    }
}

pub(super) fn render_resolved_path(
    path: &RustdocPath,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    let base = canonicalize_rust_path(&resolved_path_name(path, paths));
    let Some(arguments) = path.args.as_deref() else {
        return Ok(base);
    };
    match arguments {
        GenericArgs::AngleBracketed { args, constraints } => {
            let mut rendered = args
                .iter()
                .map(|argument| match argument {
                    GenericArg::Lifetime(lifetime) => Ok(lifetime.clone()),
                    GenericArg::Type(ty) => render_rust_type(ty, index, paths, generics),
                    GenericArg::Const(constant) => Ok(constant.expr.clone()),
                    GenericArg::Infer => {
                        Err("inferred generic argument has no stable projection".to_owned())
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
            for constraint in constraints {
                let arguments = constraint
                    .args
                    .as_deref()
                    .map(|arguments| render_generic_arguments(arguments, index, paths, generics))
                    .transpose()?
                    .unwrap_or_default();
                let binding = match &constraint.binding {
                    AssocItemConstraintKind::Equality(Term::Type(ty)) => {
                        format!(" = {}", render_rust_type(ty, index, paths, generics)?)
                    }
                    AssocItemConstraintKind::Equality(Term::Constant(constant)) => {
                        format!(" = {}", constant.expr)
                    }
                    AssocItemConstraintKind::Constraint(bounds) => {
                        let bounds = bounds
                            .iter()
                            .map(|bound| render_generic_bound(bound, &[], index, paths, generics))
                            .collect::<Result<Vec<_>, _>>()?;
                        format!(": {}", bounds.join(" + "))
                    }
                };
                rendered.push(format!("{}{arguments}{binding}", constraint.name));
            }
            Ok(format!("{base}<{}>", rendered.join(", ")))
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let inputs = inputs
                .iter()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?;
            let output = output
                .as_ref()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .transpose()?
                .map_or_else(String::new, |ty| format!(" -> {ty}"));
            Ok(format!("{base}({}){output}", inputs.join(", ")))
        }
        GenericArgs::ReturnTypeNotation => {
            Err("return-type notation has no stable projection".to_owned())
        }
    }
}

pub(super) fn render_rust_type(
    ty: &Type,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<String, String> {
    match ty {
        Type::ResolvedPath(path) => render_resolved_path(path, index, paths, generics),
        Type::Generic(name) => generics
            .get(name)
            .map(ProjectedType::rust_type)
            .ok_or_else(|| format!("unbounded generic `{name}`")),
        Type::Primitive(name) => Ok(if name == "unit" {
            "()".to_owned()
        } else {
            name.clone()
        }),
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => Ok(format!(
            "&{}{}{}",
            lifetime
                .as_deref()
                .map(|lifetime| format!("{lifetime} "))
                .unwrap_or_default(),
            if *is_mutable { "mut " } else { "" },
            render_rust_type(type_, index, paths, generics)?
        )),
        Type::Tuple(types) => Ok(format!(
            "({})",
            types
                .iter()
                .map(|ty| render_rust_type(ty, index, paths, generics))
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        )),
        Type::Slice(type_) => Ok(format!(
            "[{}]",
            render_rust_type(type_, index, paths, generics)?
        )),
        Type::Array { type_, len } => Ok(format!(
            "[{}; {len}]",
            render_rust_type(type_, index, paths, generics)?
        )),
        Type::DynTrait(dynamic) => {
            let projected = project_dyn_interface(dynamic, index, paths, generics)?;
            Ok(projected.rust_type())
        }
        Type::ImplTrait(_) => Ok("_".to_owned()),
        Type::QualifiedPath {
            name,
            args,
            self_type,
            ..
        } if args.is_none()
            && generics.contains_key("__terrane_external_associated_bounds")
            && matches!(self_type.as_ref(), Type::Generic(self_) if self_ == "Self") =>
        {
            generics
                .get(&format!("Self::{name}"))
                .map(ProjectedType::rust_type)
                .ok_or_else(|| format!("unresolved associated type `Self::{name}`"))
        }
        Type::QualifiedPath {
            name,
            args,
            self_type,
            ..
        } if args.is_none()
            && matches!(self_type.as_ref(), Type::Generic(owner) if generics.contains_key(owner)) =>
        {
            let Type::Generic(owner) = self_type.as_ref() else {
                unreachable!("qualified generic owner was matched above");
            };
            Ok(format!("{owner}::{name}"))
        }
        _ => Err("generic argument has no stable Rust type spelling".to_owned()),
    }
}

pub(super) fn instantiated_type_name(short: &str, rust_path: &str) -> String {
    if rust_path.rsplit("::").next() == Some(short) {
        return short.to_owned();
    }
    if let Some(arguments) = outer_generic_arguments(rust_path) {
        let arguments = split_top_level_arguments(arguments)
            .into_iter()
            .map(descriptive_rust_identity)
            .collect::<Vec<_>>();
        if !arguments.is_empty() && arguments.iter().all(|argument| !argument.is_empty()) {
            return format!("{short}-of-{}", arguments.join("-and-"));
        }
    }
    let descriptive = descriptive_rust_identity(rust_path);
    if descriptive.is_empty() {
        format!("{short}-hash-h{:x}", Sha256::digest(rust_path.as_bytes()))
    } else {
        format!("{short}-from-{descriptive}")
    }
}

pub(super) fn instantiated_nominal_name(
    short: &str,
    rust_path: &str,
    arguments: &[ProjectedType],
) -> String {
    if !arguments.is_empty()
        && arguments
            .iter()
            .all(ProjectedType::is_concrete_terrane_numeric)
    {
        short.to_owned()
    } else if !arguments.is_empty() {
        let descriptive_arguments = arguments
            .iter()
            .map(descriptive_projected_argument)
            .collect::<Option<Vec<_>>>();
        if let Some(descriptive_arguments) = descriptive_arguments {
            format!("{short}-of-{}", descriptive_arguments.join("-and-"))
        } else {
            let identity = format!(
                "{rust_path}|{}",
                serde_json::to_string(arguments)
                    .expect("projected type arguments always serialize")
            );
            format!("{short}-hash-h{:x}", Sha256::digest(identity.as_bytes()))
        }
    } else {
        instantiated_type_name(short, rust_path)
    }
}

fn descriptive_projected_argument(argument: &ProjectedType) -> Option<String> {
    match argument {
        ProjectedType::Generic(_) | ProjectedType::Associated(_) => None,
        ProjectedType::Opaque { bounds, .. } => descriptive_opaque_bounds(bounds),
        _ if argument.contains_open_generic() => None,
        _ => {
            let name = descriptive_rust_identity(&argument.rust_type());
            (!name.is_empty()).then_some(name)
        }
    }
}

fn descriptive_opaque_bounds(bounds: &[String]) -> Option<String> {
    let mut names = Vec::new();
    for bound in bounds {
        let (trait_path, arguments) = bound
            .split_once('<')
            .map_or((bound.as_str(), None), |(path, arguments)| {
                (path, arguments.strip_suffix('>'))
            });
        let trait_name = trait_path.rsplit("::").next().unwrap_or(trait_path).trim();
        if !trait_name.is_empty() {
            names.push(trait_name.to_owned());
        }
        for argument in arguments.into_iter().flat_map(split_top_level_arguments) {
            let Some((_, value)) = argument.split_once('=') else {
                continue;
            };
            let value = descriptive_rust_identity(value);
            if let Some(short) = value.rsplit('-').next().filter(|short| !short.is_empty()) {
                names.push(short.to_owned());
            }
        }
    }
    names.dedup();
    (!names.is_empty()).then(|| names.join("-"))
}

fn outer_generic_arguments(path: &str) -> Option<&str> {
    let start = path.find('<')?;
    path.ends_with('>')
        .then(|| &path[start + 1..path.len() - 1])
}

fn split_top_level_arguments(arguments: &str) -> Vec<&str> {
    let mut depth = 0usize;
    let mut start = 0usize;
    let mut split = Vec::new();
    for (index, character) in arguments.char_indices() {
        match character {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                split.push(arguments[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    split.push(arguments[start..].trim());
    split
}
