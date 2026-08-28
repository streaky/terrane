use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::Value;

use crate::RustDependency;

pub const RUSTDOC_TOOLCHAIN: &str = "nightly-2026-04-29";
const PROJECTION_SCHEMA: &str = "2";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Projection {
    pub cache_identity: String,
    pub dependencies: Vec<ProjectedDependency>,
    pub containment: Containment,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Containment {
    Enforced,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedDependency {
    pub name: String,
    pub package: String,
    pub version: String,
    pub items: Vec<ProjectedItem>,
    pub declined: Vec<DeclinedItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedItem {
    pub namespace: String,
    pub name: String,
    pub rust_path: String,
    pub docs: Option<String>,
    pub kind: ProjectedKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectedKind {
    Function(ProjectedFunction),
    ForeignType { methods: Vec<ProjectedFunction> },
    Enum { data_carrying: bool },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedFunction {
    pub name: String,
    pub parameters: Vec<ProjectedParameter>,
    pub result: ProjectedType,
    pub error: Option<String>,
    pub is_async: bool,
    pub receiver: Option<Receiver>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedParameter {
    pub name: String,
    pub ty: ProjectedType,
    pub borrowed: bool,
    pub mutable_borrow: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Receiver {
    Borrow,
    MutableBorrow,
    Move,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProjectedType {
    None,
    Bool,
    Int,
    Float,
    String,
    Bytes,
    Foreign { rust_path: String, name: String },
    Optional(Box<ProjectedType>),
}

impl ProjectedType {
    #[must_use]
    pub fn terrane_name(&self) -> String {
        match self {
            Self::None => "none".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Int => "int".to_owned(),
            Self::Float => "float64".to_owned(),
            Self::String => "string".to_owned(),
            Self::Bytes => "bytes".to_owned(),
            Self::Foreign { name, .. } => name.clone(),
            Self::Optional(inner) => format!("{}|none", inner.terrane_name()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclinedItem {
    pub rust_path: String,
    pub reason: String,
}

impl Projection {
    #[must_use]
    pub fn source_for_imports(
        &self,
        imports: &BTreeMap<String, BTreeSet<String>>,
    ) -> Vec<(String, String)> {
        let all_items = self
            .dependencies
            .iter()
            .flat_map(|dependency| dependency.items.iter())
            .collect::<Vec<_>>();
        let mut sources = Vec::new();
        for (namespace, names) in imports {
            let selected = all_items
                .iter()
                .copied()
                .filter(|item| item.namespace == *namespace && names.contains(&item.name))
                .collect::<Vec<_>>();
            if selected.is_empty() {
                continue;
            }
            let mut foreign = BTreeMap::<String, String>::new();
            for item in &selected {
                match &item.kind {
                    ProjectedKind::Function(function) => {
                        collect_foreign_function(function, &mut foreign);
                    }
                    ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. } => {
                        foreign.insert(item.name.clone(), item.rust_path.clone());
                    }
                }
            }
            let mut text = format!("namespace {}\n\n", namespace.trim_start_matches('/'));
            for (name, rust_path) in &foreign {
                writeln!(text, "class {name}").expect("writing to a string cannot fail");
                if let Some(ProjectedItem {
                    kind: ProjectedKind::ForeignType { methods },
                    ..
                }) = all_items.iter().copied().find(|item| {
                    item.rust_path == *rust_path
                        || (item.name == *name
                            && rust_prefix(namespace)
                                .is_some_and(|prefix| item.rust_path.starts_with(&prefix)))
                }) {
                    for method in methods.iter().filter(|method| {
                        method
                            .parameters
                            .iter()
                            .all(|parameter| !contains_foreign(&parameter.ty))
                            && !contains_foreign(&method.result)
                    }) {
                        render_function(&mut text, method, false, 4);
                    }
                }
                text.push('\n');
            }
            for item in selected {
                if let ProjectedKind::Function(function) = &item.kind {
                    render_function(&mut text, function, true, 0);
                }
            }
            sources.push((namespace.clone(), text));
        }
        sources
    }

    #[must_use]
    pub fn item(&self, namespace: &str, name: &str) -> Option<&ProjectedItem> {
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| item.namespace == namespace && item.name == name)
    }

    #[must_use]
    pub fn foreign_rust_path(&self, namespace: &str, name: &str) -> Option<&str> {
        self.item(namespace, name)
            .and_then(|item| {
                matches!(
                    item.kind,
                    ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
                )
                .then_some(item.rust_path.as_str())
            })
            .or_else(|| {
                let prefix = rust_prefix(namespace);
                self.dependencies
                    .iter()
                    .flat_map(|dependency| &dependency.items)
                    .filter(|item| {
                        matches!(
                            item.kind,
                            ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
                        ) && item.name == name
                            && prefix
                                .as_ref()
                                .is_none_or(|prefix| item.rust_path.starts_with(prefix))
                    })
                    .map(|item| item.rust_path.as_str())
                    .min_by_key(|path| (path.matches("::").count(), *path))
            })
            .or_else(|| {
                self.dependencies
                    .iter()
                    .flat_map(|dependency| &dependency.items)
                    .filter(|item| item.namespace == namespace)
                    .find_map(|item| match &item.kind {
                        ProjectedKind::Function(function) => function
                            .parameters
                            .iter()
                            .map(|parameter| &parameter.ty)
                            .chain(std::iter::once(&function.result))
                            .find_map(|ty| foreign_path_named(ty, name)),
                        _ => None,
                    })
            })
    }

    #[must_use]
    pub fn method(
        &self,
        namespace: &str,
        type_name: &str,
        method_name: &str,
    ) -> Option<&ProjectedFunction> {
        let rust_path = self.foreign_rust_path(namespace, type_name)?;
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| item.rust_path == rust_path)
            .and_then(|item| match &item.kind {
                ProjectedKind::ForeignType { methods } => {
                    methods.iter().find(|method| method.name == method_name)
                }
                _ => None,
            })
    }
}

fn rust_prefix(namespace: &str) -> Option<String> {
    let mut segments = namespace.strip_prefix("/dependencies/")?.split('/');
    let dependency = segments.next()?.replace('-', "_");
    let modules = segments.map(|segment| segment.replace('-', "_"));
    Some(
        std::iter::once(dependency)
            .chain(modules)
            .collect::<Vec<_>>()
            .join("::"),
    )
}

fn foreign_path_named<'a>(ty: &'a ProjectedType, name: &str) -> Option<&'a str> {
    match ty {
        ProjectedType::Foreign {
            rust_path,
            name: foreign_name,
        } if foreign_name == name => Some(rust_path),
        ProjectedType::Optional(inner) => foreign_path_named(inner, name),
        _ => None,
    }
}

fn collect_foreign_function(function: &ProjectedFunction, foreign: &mut BTreeMap<String, String>) {
    for ty in function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
    {
        collect_foreign_type(ty, foreign);
    }
}

fn collect_foreign_type(ty: &ProjectedType, foreign: &mut BTreeMap<String, String>) {
    match ty {
        ProjectedType::Foreign { rust_path, name } => {
            foreign.insert(name.clone(), rust_path.clone());
        }
        ProjectedType::Optional(inner) => collect_foreign_type(inner, foreign),
        _ => {}
    }
}

fn contains_foreign(ty: &ProjectedType) -> bool {
    match ty {
        ProjectedType::Foreign { .. } => true,
        ProjectedType::Optional(inner) => contains_foreign(inner),
        _ => false,
    }
}

fn render_function(output: &mut String, function: &ProjectedFunction, public: bool, indent: usize) {
    let prefix = " ".repeat(indent);
    let visibility = if public { "public " } else { "" };
    let asynchronous = if function.is_async { "async " } else { "" };
    write!(
        output,
        "{prefix}{visibility}{asynchronous}function {}",
        function.name
    )
    .expect("writing to a string cannot fail");
    if function.result != ProjectedType::None {
        write!(output, " {}", function.result.terrane_name())
            .expect("writing to a string cannot fail");
    }
    output.push(';');
    if !function.parameters.is_empty() {
        output.push(' ');
        for (index, parameter) in function.parameters.iter().enumerate() {
            if index != 0 {
                output.push_str(", ");
            }
            write!(output, "{} {}", parameter.name, parameter.ty.terrane_name())
                .expect("writing to a string cannot fail");
        }
    }
    output.push('\n');
}
#[derive(Debug)]
pub struct ProjectionError {
    pub message: String,
}

impl std::fmt::Display for ProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ProjectionError {}

/// Resolves every declared Rust package and derives the shared Terrane projection.
///
/// # Errors
/// Returns a projection error when Cargo resolution, rustdoc generation, cache input reading, or
/// projection of the resolved metadata fails.
pub fn resolve(
    root: &Path,
    dependencies: &[RustDependency],
) -> Result<Projection, ProjectionError> {
    let identity = cache_identity(root, dependencies);
    if dependencies.is_empty() {
        return Ok(Projection {
            cache_identity: identity,
            dependencies: Vec::new(),
            containment: containment(),
        });
    }
    let workspace = root.join(".trn/dependencies");
    write_workspace(&workspace, dependencies)?;
    if workspace.join("Cargo.lock").exists() {
        run_cargo(&workspace, &["fetch", "--locked"], false)?;
    } else {
        run_cargo(&workspace, &["fetch"], false)?;
    }
    let mut projected = Vec::new();
    for dependency in dependencies {
        run_cargo(
            &workspace,
            &[
                "rustdoc",
                "-p",
                &dependency.package,
                "--lib",
                "--offline",
                "--frozen",
                "--",
                "-Z",
                "unstable-options",
                "--output-format",
                "json",
            ],
            true,
        )?;
        let crate_name = dependency.package.replace('-', "_");
        let rustdoc_path = workspace
            .join("target/doc")
            .join(format!("{crate_name}.json"));
        let bytes = fs::read(&rustdoc_path).map_err(|error| ProjectionError {
            message: format!(
                "cannot read rustdoc projection `{}`: {error}",
                rustdoc_path.display()
            ),
        })?;
        projected.push(project_rustdoc(dependency, &bytes)?);
    }
    Ok(Projection {
        cache_identity: identity,
        dependencies: projected,
        containment: containment(),
    })
}

fn write_workspace(
    directory: &Path,
    dependencies: &[RustDependency],
) -> Result<(), ProjectionError> {
    fs::create_dir_all(directory.join("src")).map_err(io_error("create dependency workspace"))?;
    let mut manifest = String::from(
        "[package]\nname = \"terrane_dependency_projection\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\n",
    );
    let mut source = String::new();
    for dependency in dependencies {
        let features = dependency
            .features
            .iter()
            .map(|feature| format!("{feature:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            manifest,
            "{} = {{ package = {:?}, version = {:?}, default-features = {}, features = [{}] }}",
            dependency.name.replace('-', "_"),
            dependency.package,
            dependency.version,
            dependency.default_features,
            features
        )
        .expect("writing to a string cannot fail");
        writeln!(
            source,
            "pub use {} as {};",
            dependency.name.replace('-', "_"),
            dependency.name.replace('-', "_")
        )
        .expect("writing to a string cannot fail");
    }
    manifest.push_str("\n[workspace]\n");
    write_if_changed(&directory.join("Cargo.toml"), manifest.as_bytes())?;
    write_if_changed(&directory.join("src/lib.rs"), source.as_bytes())?;
    Ok(())
}

fn run_cargo(directory: &Path, arguments: &[&str], nightly: bool) -> Result<(), ProjectionError> {
    let mut command = Command::new("cargo");
    if nightly {
        command.arg(format!("+{RUSTDOC_TOOLCHAIN}"));
    }
    let output = command
        .args(arguments)
        .current_dir(directory)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot run Cargo dependency projection: {error}"),
        })?;
    if output.status.success() {
        return Ok(());
    }
    Err(ProjectionError {
        message: format!(
            "Cargo dependency projection failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ),
    })
}

#[expect(
    clippy::too_many_lines,
    reason = "one rustdoc item pass records admitted and declined public items together"
)]
fn project_rustdoc(
    dependency: &RustDependency,
    bytes: &[u8],
) -> Result<ProjectedDependency, ProjectionError> {
    let document: Value = serde_json::from_slice(bytes).map_err(|error| ProjectionError {
        message: format!("invalid rustdoc JSON for `{}`: {error}", dependency.package),
    })?;
    let index = document["index"]
        .as_object()
        .ok_or_else(|| ProjectionError {
            message: format!(
                "rustdoc JSON for `{}` has no item index",
                dependency.package
            ),
        })?;
    let paths = document["paths"]
        .as_object()
        .ok_or_else(|| ProjectionError {
            message: format!(
                "rustdoc JSON for `{}` has no path index",
                dependency.package
            ),
        })?;
    let mut public_paths = BTreeMap::<String, String>::new();
    for (module_id, summary) in paths {
        if summary["crate_id"].as_u64() != Some(0) {
            continue;
        }
        let Some(module) = index
            .get(module_id)
            .and_then(|item| item["inner"]["module"].as_object())
        else {
            continue;
        };
        let module_path = string_array(&summary["path"]);
        for child_id in string_or_number_array(&module["items"]) {
            let Some(import) = index
                .get(&child_id)
                .filter(|item| item["visibility"].as_str() == Some("public"))
                .and_then(|item| item["inner"]["use"].as_object())
            else {
                continue;
            };
            let Some(target_id) = value_id(&import["id"]) else {
                continue;
            };
            let Some(name) = import["name"].as_str() else {
                continue;
            };
            let candidate = module_path
                .iter()
                .map(String::as_str)
                .chain(std::iter::once(name))
                .collect::<Vec<_>>()
                .join("::");
            public_paths
                .entry(target_id)
                .and_modify(|existing| {
                    if candidate.matches("::").count() < existing.matches("::").count() {
                        existing.clone_from(&candidate);
                    }
                })
                .or_insert(candidate);
        }
    }
    let mut items = Vec::new();
    let mut declined = Vec::new();
    for (id, summary) in paths {
        if summary["crate_id"].as_u64() != Some(0) {
            continue;
        }
        let Some(item) = index.get(id) else { continue };
        if item["visibility"].as_str() != Some("public") {
            continue;
        }
        let path = string_array(&summary["path"]);
        let Some(name) = path.last().cloned() else {
            continue;
        };
        let namespace = dependency_namespace(dependency, &path[..path.len().saturating_sub(1)]);
        let rust_path = public_paths
            .get(id)
            .cloned()
            .unwrap_or_else(|| path.join("::"));
        let docs = item["docs"].as_str().map(str::to_owned);
        let inner = &item["inner"];
        let projected = if let Some(function) = inner.get("function") {
            project_function(function, index, paths, Some(&name)).map(ProjectedKind::Function)
        } else if let Some(structure) = inner.get("struct") {
            let methods = project_methods(structure, index, paths);
            Ok(ProjectedKind::ForeignType { methods })
        } else if let Some(enumeration) = inner.get("enum") {
            let data_carrying = enumeration["variants"].as_array().is_some_and(|variants| {
                variants.iter().any(|id| {
                    index
                        .get(&id.to_string())
                        .and_then(|variant| variant["inner"]["variant"]["kind"].as_str())
                        .is_none()
                })
            });
            Ok(ProjectedKind::Enum { data_carrying })
        } else {
            Err("item kind has no Terrane projection".to_owned())
        };
        match projected {
            Ok(kind) => items.push(ProjectedItem {
                namespace,
                name,
                rust_path,
                docs,
                kind,
            }),
            Err(reason) => declined.push(DeclinedItem { rust_path, reason }),
        }
    }
    items
        .sort_by(|left, right| (&left.namespace, &left.name).cmp(&(&right.namespace, &right.name)));
    declined.sort_by(|left, right| left.rust_path.cmp(&right.rust_path));
    Ok(ProjectedDependency {
        name: dependency.name.clone(),
        package: dependency.package.clone(),
        version: document["crate_version"]
            .as_str()
            .unwrap_or(&dependency.version)
            .to_owned(),
        items,
        declined,
    })
}

fn project_methods(
    structure: &Value,
    index: &serde_json::Map<String, Value>,
    paths: &serde_json::Map<String, Value>,
) -> Vec<ProjectedFunction> {
    let mut methods = Vec::new();
    let Some(impls) = structure["impls"].as_array() else {
        return methods;
    };
    for impl_id in impls {
        let Some(implementation) = index
            .get(&impl_id.to_string())
            .and_then(|item| item["inner"]["impl"].as_object())
        else {
            continue;
        };
        if !implementation["trait"].is_null()
            || implementation["is_negative"].as_bool() == Some(true)
        {
            continue;
        }
        let Some(method_ids) = implementation["items"].as_array() else {
            continue;
        };
        for method_id in method_ids {
            let Some(item) = index.get(&method_id.to_string()) else {
                continue;
            };
            if item["visibility"].as_str() != Some("public") {
                continue;
            }
            let Some(function) = item["inner"].get("function") else {
                continue;
            };
            if let Ok(method) = project_function(function, index, paths, item["name"].as_str()) {
                methods.push(method);
            }
        }
    }
    methods
}

fn project_function(
    function: &Value,
    index: &serde_json::Map<String, Value>,
    paths: &serde_json::Map<String, Value>,
    method_name: Option<&str>,
) -> Result<ProjectedFunction, String> {
    if function["header"]["is_unsafe"].as_bool() == Some(true) {
        return Err("unsafe function".to_owned());
    }
    let generic_types = generic_monomorphisations(function, index, paths)?;
    let inputs = function["sig"]["inputs"]
        .as_array()
        .ok_or_else(|| "function signature has no inputs".to_owned())?;
    let mut parameters = Vec::new();
    let mut receiver = None;
    for input in inputs {
        let pair = input
            .as_array()
            .ok_or_else(|| "malformed function input".to_owned())?;
        let name = pair.first().and_then(Value::as_str).unwrap_or("value");
        let ty = pair
            .get(1)
            .ok_or_else(|| "malformed function input type".to_owned())?;
        if name == "self" {
            receiver = Some(receiver_kind(ty));
            continue;
        }
        parameters.push(ProjectedParameter {
            name: safe_parameter_name(name),
            ty: project_type(ty, paths, &generic_types)?,
            borrowed: ty.get("borrowed_ref").is_some(),
            mutable_borrow: ty["borrowed_ref"]["is_mutable"].as_bool() == Some(true),
        });
    }
    let mut error = None;
    let result = if function["sig"]["output"].is_null() {
        ProjectedType::None
    } else {
        let output = &function["sig"]["output"];
        if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Result") || name == "Result")
        {
            let arguments = type_arguments(output);
            let value = arguments
                .first()
                .ok_or_else(|| "Result has no value type".to_owned())?;
            error = arguments
                .get(1)
                .and_then(|ty| resolved_name(ty, paths))
                .or_else(|| Some("Error".to_owned()));
            project_type(value, paths, &generic_types)?
        } else if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Option") || name == "Option")
        {
            let value = type_arguments(output)
                .into_iter()
                .next()
                .ok_or_else(|| "Option has no value type".to_owned())?;
            ProjectedType::Optional(Box::new(project_type(value, paths, &generic_types)?))
        } else {
            project_type(output, paths, &generic_types)?
        }
    };
    Ok(ProjectedFunction {
        name: method_name.unwrap_or_default().to_owned(),
        parameters,
        result,
        error,
        is_async: function["header"]["is_async"].as_bool() == Some(true),
        receiver,
    })
}

fn generic_monomorphisations(
    function: &Value,
    index: &serde_json::Map<String, Value>,
    paths: &serde_json::Map<String, Value>,
) -> Result<BTreeMap<String, ProjectedType>, String> {
    let mut result = BTreeMap::new();
    let Some(params) = function["generics"]["params"].as_array() else {
        return Ok(result);
    };
    for parameter in params {
        let Some(name) = parameter["name"].as_str() else {
            continue;
        };
        let Some(bounds) = parameter["kind"]["type"]["bounds"].as_array() else {
            return Err(format!("unbounded generic `{name}`"));
        };
        let Some(trait_id) = bounds
            .first()
            .and_then(|bound| bound["trait_bound"]["trait"]["id"].as_u64())
        else {
            return Err(format!("open generic `{name}`"));
        };
        let Some(implementations) = index
            .get(&trait_id.to_string())
            .and_then(|item| item["inner"]["trait"]["implementations"].as_array())
        else {
            return Err(format!("generic bound for `{name}` has no closed impl set"));
        };
        let candidates = implementations
            .iter()
            .filter_map(|id| index.get(&id.to_string()))
            .filter_map(|item| item["inner"]["impl"].get("for"))
            .filter_map(|ty| project_type(ty, paths, &BTreeMap::new()).ok())
            .collect::<Vec<_>>();
        let chosen = candidates
            .iter()
            .find(|ty| {
                matches!(
                    ty,
                    ProjectedType::String
                        | ProjectedType::Bytes
                        | ProjectedType::Bool
                        | ProjectedType::Int
                        | ProjectedType::Float
                )
            })
            .or_else(|| candidates.first())
            .cloned()
            .ok_or_else(|| {
                format!("generic bound for `{name}` has no Terrane-representable impl")
            })?;
        result.insert(name.to_owned(), chosen);
    }
    Ok(result)
}

fn project_type(
    ty: &Value,
    paths: &serde_json::Map<String, Value>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    if let Some(reference) = ty.get("borrowed_ref") {
        return project_type(&reference["type"], paths, generics);
    }
    if let Some(generic) = ty.get("generic").and_then(Value::as_str) {
        if generic == "Self" {
            return Err("receiver type used outside receiver position".to_owned());
        }
        return generics
            .get(generic)
            .cloned()
            .ok_or_else(|| format!("unbounded generic `{generic}`"));
    }
    if let Some(primitive) = ty.get("primitive").and_then(Value::as_str) {
        return match primitive {
            "bool" => Ok(ProjectedType::Bool),
            "str" | "char" => Ok(ProjectedType::String),
            "f32" | "f64" => Ok(ProjectedType::Float),
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64"
            | "u128" | "usize" => Ok(ProjectedType::Int),
            "unit" => Ok(ProjectedType::None),
            other => Err(format!("unsupported primitive `{other}`")),
        };
    }
    let Some(path) = resolved_name(ty, paths) else {
        return Err("type has no stable Rust path".to_owned());
    };
    let short = path.rsplit("::").next().unwrap_or(&path).to_owned();
    match short.as_str() {
        "String" => Ok(ProjectedType::String),
        "Vec"
            if type_arguments(ty)
                .first()
                .is_some_and(|inner| inner["primitive"].as_str() == Some("u8")) =>
        {
            Ok(ProjectedType::Bytes)
        }
        _ => Ok(ProjectedType::Foreign {
            rust_path: path,
            name: short,
        }),
    }
}

fn receiver_kind(ty: &Value) -> Receiver {
    ty.get("borrowed_ref").map_or(Receiver::Move, |reference| {
        if reference["is_mutable"].as_bool() == Some(true) {
            Receiver::MutableBorrow
        } else {
            Receiver::Borrow
        }
    })
}

fn resolved_name(ty: &Value, paths: &serde_json::Map<String, Value>) -> Option<String> {
    let resolved = ty.get("resolved_path")?;
    let id = resolved["id"].as_u64()?.to_string();
    Some(
        paths
            .get(&id)
            .map(|summary| string_array(&summary["path"]).join("::"))
            .filter(|path| !path.is_empty())
            .unwrap_or_else(|| {
                resolved["path"]
                    .as_str()
                    .unwrap_or_default()
                    .replace("crate::", "")
            }),
    )
}

fn type_arguments(ty: &Value) -> Vec<&Value> {
    ty["resolved_path"]["args"]["angle_bracketed"]["args"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|argument| argument.get("type"))
        .collect()
}

fn dependency_namespace(dependency: &RustDependency, path: &[String]) -> String {
    let modules = path
        .iter()
        .skip(1)
        .map(|segment| segment.replace('_', "-"))
        .collect::<Vec<_>>();
    if modules.is_empty() {
        format!("/dependencies/{}", dependency.name.replace('_', "-"))
    } else {
        format!(
            "/dependencies/{}/{}",
            dependency.name.replace('_', "-"),
            modules.join("/")
        )
    }
}

fn safe_parameter_name(name: &str) -> String {
    match name {
        "ref" | "move" | "function" | "class" | "return" | "throw" => format!("{name}_"),
        _ => name.to_owned(),
    }
}

fn string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}

fn string_or_number_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(value_id)
        .collect()
}

fn value_id(value: &Value) -> Option<String> {
    value
        .as_str()
        .map(str::to_owned)
        .or_else(|| value.as_u64().map(|number| number.to_string()))
}

fn cache_identity(root: &Path, dependencies: &[RustDependency]) -> String {
    let manifest = fs::read(root.join(crate::MANIFEST_FILE_NAME)).unwrap_or_default();
    let lock = fs::read(root.join("Cargo.lock")).unwrap_or_default();
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in manifest.iter().chain(lock.iter()).chain(
        format!(
            "{dependencies:?}|{}|{RUSTDOC_TOOLCHAIN}|{PROJECTION_SCHEMA}",
            std::env::consts::ARCH
        )
        .as_bytes(),
    ) {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

fn containment() -> Containment {
    if cfg!(target_os = "linux") {
        Containment::Enforced
    } else {
        Containment::Unavailable
    }
}

fn write_if_changed(path: &Path, content: &[u8]) -> Result<(), ProjectionError> {
    if fs::read(path).is_ok_and(|existing| existing == content) {
        return Ok(());
    }
    fs::write(path, content).map_err(io_error("write dependency projection input"))
}

fn io_error(context: &'static str) -> impl FnOnce(std::io::Error) -> ProjectionError {
    move |error| ProjectionError {
        message: format!("{context}: {error}"),
    }
}
