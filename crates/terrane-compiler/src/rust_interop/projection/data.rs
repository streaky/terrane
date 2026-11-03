use super::functions::project_function_inner;
use super::*;
#[path = "descriptor_fields.rs"]
mod descriptor_fields;
pub(super) fn normalize_projected_items(
    items: &mut Vec<ProjectedItem>,
    declined: &mut Vec<DeclinedItem>,
) {
    items.sort_by(|left, right| {
        (&left.namespace, &left.name, &left.rust_path).cmp(&(
            &right.namespace,
            &right.name,
            &right.rust_path,
        ))
    });
    items.dedup_by(|left, right| {
        left.namespace == right.namespace
            && left.name == right.name
            && left.rust_path == right.rust_path
    });
    declined.sort_by(|left, right| {
        (&left.rust_path, &left.reason).cmp(&(&right.rust_path, &right.reason))
    });
    declined.dedup();
}

fn exact_projected_field_type(ty: ProjectedType) -> ProjectedType {
    match ty {
        ProjectedType::RustInt(name) => ProjectedType::FixedInt(name),
        ProjectedType::Sequence { rust_path, item } => ProjectedType::Sequence {
            rust_path,
            item: Box::new(exact_projected_field_type(*item)),
        },
        ProjectedType::Optional(inner) => {
            ProjectedType::Optional(Box::new(exact_projected_field_type(*inner)))
        }
        ProjectedType::Tuple(items) => {
            ProjectedType::Tuple(items.into_iter().map(exact_projected_field_type).collect())
        }
        ProjectedType::Mapping {
            rust_path,
            key,
            value,
            ordered,
        } => ProjectedType::Mapping {
            rust_path,
            key: Box::new(exact_projected_field_type(*key)),
            value: Box::new(exact_projected_field_type(*value)),
            ordered,
        },
        other => other,
    }
}

fn owned_field_is_directly_constructible(ty: &ProjectedType) -> bool {
    match ty {
        ProjectedType::Bool
        | ProjectedType::Int
        | ProjectedType::FixedInt(_)
        | ProjectedType::RustInt(_)
        | ProjectedType::Float
        | ProjectedType::Float32
        | ProjectedType::Char
        | ProjectedType::String
        | ProjectedType::Bytes
        | ProjectedType::Foreign { .. }
        | ProjectedType::Generic(_) => true,
        ProjectedType::Sequence { item, .. } | ProjectedType::Optional(item) => {
            owned_field_is_directly_constructible(item)
        }
        ProjectedType::Tuple(items) => items.iter().all(owned_field_is_directly_constructible),
        ProjectedType::Mapping { key, value, .. } => {
            owned_field_is_directly_constructible(key)
                && owned_field_is_directly_constructible(value)
        }
        _ => false,
    }
}

fn projected_field_name(rust_name: &str) -> String {
    let unescaped = rust_name.trim_end_matches('_');
    let projected = projected_constant_name(unescaped);
    if crate::syntax::is_keyword(unescaped) {
        format!("native-{projected}")
    } else {
        projected
    }
}

pub(super) fn project_struct_fields(
    structure: &Struct,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<(Vec<ProjectedField>, bool), String> {
    let tuple_fields;
    let fields = match &structure.kind {
        rustdoc_types::StructKind::Tuple(fields) => {
            tuple_fields = fields
                .iter()
                .copied()
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| "tuple struct has private or hidden fields".to_owned())?;
            &tuple_fields
        }
        rustdoc_types::StructKind::Plain {
            fields,
            has_stripped_fields: false,
        } => fields,
        rustdoc_types::StructKind::Plain {
            has_stripped_fields: true,
            ..
        } => return Err("struct has private or hidden fields".to_owned()),
        rustdoc_types::StructKind::Unit => return Ok((Vec::new(), false)),
    };
    let borrowed_view = structure
        .generics
        .params
        .iter()
        .any(|parameter| matches!(parameter.kind, GenericParamDefKind::Lifetime { .. }));
    let mut projected = Vec::with_capacity(fields.len());
    for field_id in fields {
        let field = index
            .get(field_id)
            .ok_or_else(|| "struct field is missing from rustdoc".to_owned())?;
        let rust_name = field
            .name
            .clone()
            .ok_or_else(|| "struct field has no name".to_owned())?;
        let ItemEnum::StructField(field_type) = &field.inner else {
            return Err(format!("`{rust_name}` is not a struct field"));
        };
        let (ty, conversion) = descriptor_fields::project_field(field_type, index, paths, generics)
            .map_err(|reason| format!("field `{rust_name}`: {reason}"))?;
        let ty = exact_projected_field_type(ty);
        if !borrowed_view && !owned_field_is_directly_constructible(&ty) {
            return Err(format!(
                "field `{rust_name}` requires a non-identity owned conversion"
            ));
        }
        projected.push(ProjectedField {
            name: projected_field_name(&rust_name),
            rust_name,
            ty,
            rust_type: render_rust_type(field_type, index, paths, generics)?,
            conversion,
        });
    }
    projected.sort_by_key(|field| matches!(field.ty, ProjectedType::Optional(_)));
    Ok((projected, borrowed_view))
}

pub(super) fn project_struct_constructor(
    structure: &Struct,
    fields: &[ProjectedField],
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    public_paths: &BTreeMap<Id, String>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedFunction, String> {
    if fields.is_empty() {
        return Err("native generic constructor requires public payload fields".to_owned());
    }
    let ids = match &structure.kind {
        rustdoc_types::StructKind::Plain { fields, .. } => fields.clone(),
        rustdoc_types::StructKind::Tuple(fields) => fields.iter().flatten().copied().collect(),
        rustdoc_types::StructKind::Unit => Vec::new(),
    };
    let inputs = fields
        .iter()
        .map(|field| {
            ids.iter()
                .filter_map(|id| index.get(id))
                .find_map(|item| {
                    if item.name.as_deref() != Some(&field.rust_name) {
                        return None;
                    }
                    let ItemEnum::StructField(ty) = &item.inner else {
                        return None;
                    };
                    Some((field.name.clone(), ty.clone()))
                })
                .ok_or_else(|| {
                    format!(
                        "native constructor field `{}` has no declaration",
                        field.name
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let function = Function {
        sig: rustdoc_types::FunctionSignature {
            inputs,
            output: Some(Type::Generic("Self".to_owned())),
            is_c_variadic: false,
        },
        generics: structure.generics.clone(),
        header: rustdoc_types::FunctionHeader {
            is_const: false,
            is_unsafe: false,
            is_async: false,
            abi: rustdoc_types::Abi::Rust,
        },
        has_body: false,
        default_unstable: None,
    };
    project_function_inner(
        &function,
        index,
        paths,
        public_paths,
        Some("construct"),
        generics,
        false,
    )
}

pub(super) fn rebind_method_owner(
    method: &mut ProjectedFunction,
    implementation: &rustdoc_types::Impl,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
    generics: &BTreeMap<String, ProjectedType>,
) {
    if method.receiver == Some(super::Receiver::Move)
        && let Type::BorrowedRef { is_mutable, .. } = &implementation.for_
    {
        method.receiver = Some(if *is_mutable {
            super::Receiver::MutableBorrow
        } else {
            super::Receiver::Borrow
        });
    }
    let Some(facade) = generics.get("Self") else {
        return;
    };
    let Ok(native) = project_type(&implementation.for_, index, paths, generics) else {
        return;
    };
    if &native == facade {
        return;
    }
    rebind_owner_type(&mut method.result, &native, facade);
    for parameter in &mut method.parameters {
        rebind_owner_type(&mut parameter.ty, &native, facade);
    }
}

fn rebind_owner_type(ty: &mut ProjectedType, native: &ProjectedType, facade: &ProjectedType) {
    let same_owner = same_projected_owner(ty, native);
    if same_owner {
        ty.clone_from(facade);
        return;
    }
    match ty {
        ProjectedType::InvocationScoped {
            rust_type,
            lifetimes,
            owned,
            ..
        } => {
            if same_projected_owner(owned, native) {
                if let ProjectedType::InvocationScoped {
                    rust_type: facade_rust_type,
                    lifetimes: facade_lifetimes,
                    owned: facade_owned,
                    ..
                } = facade
                    && lifetimes
                        .iter()
                        .any(|lifetime| facade_lifetimes.contains(lifetime))
                {
                    rust_type.clone_from(facade_rust_type);
                    lifetimes.clone_from(facade_lifetimes);
                    owned.clone_from(facade_owned);
                }
            } else {
                rebind_owner_type(owned, native, facade);
            }
        }
        ProjectedType::Optional(inner)
        | ProjectedType::Sequence { item: inner, .. }
        | ProjectedType::Set { item: inner, .. }
        | ProjectedType::AsyncIterationStep(inner)
        | ProjectedType::Reference { inner, .. } => {
            rebind_owner_type(inner, native, facade);
        }
        ProjectedType::Tuple(items)
        | ProjectedType::Foreign {
            arguments: items, ..
        } => {
            for item in items {
                rebind_owner_type(item, native, facade);
            }
        }
        ProjectedType::Mapping { key, value, .. } => {
            rebind_owner_type(key, native, facade);
            rebind_owner_type(value, native, facade);
        }
        ProjectedType::Callback {
            parameters,
            result,
            native_substitutions,
            ..
        } => {
            for parameter in parameters {
                rebind_owner_type(parameter, native, facade);
            }
            rebind_owner_type(result, native, facade);
            for substitution in native_substitutions.values_mut() {
                rebind_owner_type(substitution, native, facade);
            }
        }
        ProjectedType::OwnedInterface {
            associated_type: Some(associated),
            ..
        } => rebind_owner_type(&mut associated.ty, native, facade),
        _ => {}
    }
}

fn same_projected_owner(left: &ProjectedType, right: &ProjectedType) -> bool {
    match (left, right) {
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
            },
        ) => left_path == right_path && left_arguments == right_arguments,
        _ => left == right,
    }
}

pub(super) fn default_generic_instantiation(
    structure: &Struct,
    index: &HashMap<Id, Item>,
    paths: &HashMap<Id, ItemSummary>,
) -> Result<BTreeMap<String, ProjectedType>, String> {
    let mut substitutions = BTreeMap::new();
    for parameter in &structure.generics.params {
        match &parameter.kind {
            GenericParamDefKind::Type {
                default: Some(default),
                ..
            } => {
                let projected = project_type(default, index, paths, &substitutions)?;
                substitutions.insert(parameter.name.clone(), projected);
            }
            GenericParamDefKind::Type { default: None, .. } => {
                return Err(format!(
                    "generic type parameter `{}` has no default instantiation",
                    parameter.name
                ));
            }
            GenericParamDefKind::Lifetime { .. } => {}
            GenericParamDefKind::Const { .. } => {
                return Err(format!(
                    "const parameter `{}` has no projected value identity",
                    parameter.name
                ));
            }
        }
    }
    Ok(substitutions)
}

pub(super) fn extern_rust_path(dependency: &RustDependency, path: &str) -> String {
    let mut segments = path.split("::");
    let _package_root = segments.next();
    std::iter::once(dependency.name.replace('-', "_"))
        .chain(segments.map(str::to_owned))
        .collect::<Vec<_>>()
        .join("::")
}

pub(super) fn projected_constant_name(name: &str) -> String {
    name.split('_')
        .map(|segment| {
            if segment.chars().all(|character| character.is_ascii_digit()) {
                format!("n{segment}")
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("-")
}
pub(super) fn project_rust_constant_expression(expression: &str) -> Option<String> {
    fn translate(expression: &syn::Expr) -> Option<String> {
        match expression {
            syn::Expr::Lit(expression) => match &expression.lit {
                syn::Lit::Bool(value) => Some(value.value.to_string()),
                syn::Lit::Int(value) => Some(value.base10_digits().to_owned()),
                syn::Lit::Float(value) => Some(value.base10_digits().to_owned()),
                _ => None,
            },
            syn::Expr::Group(group) => translate(&group.expr),
            syn::Expr::Paren(paren) => translate(&paren.expr).map(|value| format!("({value})")),
            syn::Expr::Unary(unary) => {
                let operator = match unary.op {
                    syn::UnOp::Neg(_) => "-",
                    _ => return None,
                };
                translate(&unary.expr).map(|value| format!("{operator}{value}"))
            }
            syn::Expr::Binary(binary) => {
                let operator = match binary.op {
                    syn::BinOp::Add(_) => "+",
                    syn::BinOp::Sub(_) => "-",
                    syn::BinOp::Mul(_) => "*",
                    syn::BinOp::Div(_) => "/",
                    syn::BinOp::Rem(_) => "%",
                    _ => return None,
                };
                Some(format!(
                    "({} {operator} {})",
                    translate(&binary.left)?,
                    translate(&binary.right)?
                ))
            }

            syn::Expr::Tuple(tuple) => tuple
                .elems
                .iter()
                .map(translate)
                .collect::<Option<Vec<_>>>()
                .map(|items| format!("({})", items.join(", "))),
            syn::Expr::Array(array) => array
                .elems
                .iter()
                .map(translate)
                .collect::<Option<Vec<_>>>()
                .map(|items| format!("[{}]", items.join(", "))),
            _ => None,
        }
    }

    syn::parse_str::<syn::Expr>(expression)
        .ok()
        .and_then(|expression| translate(&expression))
}

pub(super) type SourceConstantCache = BTreeMap<PathBuf, BTreeMap<(String, String), Option<String>>>;

pub(super) fn source_constant_expression(
    item: &Item,
    owner: &str,
    name: &str,
    cache: &mut SourceConstantCache,
) -> Option<String> {
    fn collect(items: &[syn::Item], constants: &mut BTreeMap<(String, String), Option<String>>) {
        for item in items {
            match item {
                syn::Item::Impl(implementation) => {
                    let owner = implementation
                        .self_ty
                        .to_token_stream()
                        .to_string()
                        .split('<')
                        .next()
                        .and_then(|path| path.split("::").last())
                        .map(str::trim)
                        .unwrap_or_default()
                        .to_owned();
                    for member in &implementation.items {
                        let syn::ImplItem::Const(constant) = member else {
                            continue;
                        };
                        let key = (owner.clone(), constant.ident.to_string());
                        let expression = constant.expr.to_token_stream().to_string();
                        constants
                            .entry(key)
                            .and_modify(|existing| *existing = None)
                            .or_insert(Some(expression));
                    }
                }
                syn::Item::Mod(module) => {
                    if let Some((_, items)) = &module.content {
                        collect(items, constants);
                    }
                }
                _ => {}
            }
        }
    }

    let filename = PathBuf::from(&item.span.as_ref()?.filename);
    let constants = cache.entry(filename.clone()).or_insert_with(|| {
        let mut constants = BTreeMap::new();
        if let Ok(source) = fs::read_to_string(filename)
            && let Ok(file) = syn::parse_file(&source)
        {
            collect(&file.items, &mut constants);
        }
        constants
    });
    let owner = owner
        .split('<')
        .next()
        .unwrap_or(owner)
        .rsplit("::")
        .next()
        .unwrap_or(owner);
    constants
        .get(&(owner.to_owned(), name.to_owned()))
        .cloned()
        .flatten()
}

#[cfg(test)]
mod constant_expression_tests {
    use super::project_rust_constant_expression;

    #[test]
    fn rust_str_not_const() {
        assert_eq!(project_rust_constant_expression(r#""hello world""#), None);
        assert_eq!(project_rust_constant_expression(r#""don't""#), None);
        assert_eq!(project_rust_constant_expression(r#""line\nbreak""#), None);
    }
}
