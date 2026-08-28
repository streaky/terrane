use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::RustDependency;

pub const RUSTDOC_TOOLCHAIN: &str = "nightly-2026-04-29";
const PROJECTION_SCHEMA: &str = "4";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Projection {
    pub cache_identity: String,
    pub dependencies: Vec<ProjectedDependency>,
    pub containment: Containment,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Containment {
    Enforced,
    Unavailable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedDependency {
    pub name: String,
    pub package: String,
    pub version: String,
    pub items: Vec<ProjectedItem>,
    pub declined: Vec<DeclinedItem>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedItem {
    pub namespace: String,
    pub name: String,
    pub rust_path: String,
    pub docs: Option<String>,
    pub kind: ProjectedKind,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectedKind {
    Function(ProjectedFunction),
    ForeignType { methods: Vec<ProjectedFunction> },
    Enum { data_carrying: bool },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedFunction {
    pub name: String,
    pub parameters: Vec<ProjectedParameter>,
    pub result: ProjectedType,
    pub error: Option<String>,
    pub is_async: bool,
    pub receiver: Option<Receiver>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectedParameter {
    pub name: String,
    pub ty: ProjectedType,
    pub borrowed: bool,
    pub mutable_borrow: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Receiver {
    Borrow,
    MutableBorrow,
    Move,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
            let foreign = collect_source_foreign(&all_items, &selected, namespace);
            let mut ordered_foreign = foreign.iter().collect::<Vec<_>>();
            ordered_foreign.sort_by_key(|(name, rust_path)| {
                let dependency_count = all_items
                    .iter()
                    .copied()
                    .find(|item| {
                        item.rust_path == ***rust_path
                            || (item.name == ***name
                                && rust_prefix(namespace)
                                    .is_some_and(|prefix| item.rust_path.starts_with(&prefix)))
                    })
                    .and_then(|item| match &item.kind {
                        ProjectedKind::ForeignType { methods } => Some(
                            methods
                                .iter()
                                .map(|method| foreign_function_dependency_count(method, name))
                                .sum::<usize>(),
                        ),
                        _ => None,
                    })
                    .unwrap_or_default();
                (dependency_count, *name)
            });
            let mut text = format!("namespace {}\n\n", namespace.trim_start_matches('/'));
            for (name, rust_path) in ordered_foreign {
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
                    for method in methods {
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
    let mut segments = namespace.strip_prefix("/deps/")?.split('/');
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
fn collect_source_foreign(
    all_items: &[&ProjectedItem],
    selected: &[&ProjectedItem],
    namespace: &str,
) -> BTreeMap<String, String> {
    let mut foreign = BTreeMap::<String, String>::new();
    for item in selected {
        match &item.kind {
            ProjectedKind::Function(function) => {
                collect_foreign_function(function, &mut foreign);
            }
            ProjectedKind::ForeignType { methods } => {
                foreign.insert(item.name.clone(), item.rust_path.clone());
                for method in methods {
                    collect_foreign_function(method, &mut foreign);
                }
            }
            ProjectedKind::Enum { .. } => {
                foreign.insert(item.name.clone(), item.rust_path.clone());
            }
        }
    }
    loop {
        let previous_len = foreign.len();
        let referenced = foreign
            .iter()
            .map(|(name, rust_path)| (name.clone(), rust_path.clone()))
            .collect::<Vec<_>>();
        for (name, rust_path) in referenced {
            let Some(ProjectedItem {
                kind: ProjectedKind::ForeignType { methods },
                ..
            }) = all_items.iter().copied().find(|item| {
                item.rust_path == rust_path
                    || (item.name == name
                        && rust_prefix(namespace)
                            .is_some_and(|prefix| item.rust_path.starts_with(&prefix)))
            })
            else {
                continue;
            };
            for method in methods {
                collect_foreign_function(method, &mut foreign);
            }
        }
        if foreign.len() == previous_len {
            return foreign;
        }
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

fn foreign_function_dependency_count(function: &ProjectedFunction, owner: &str) -> usize {
    function
        .parameters
        .iter()
        .map(|parameter| &parameter.ty)
        .chain(std::iter::once(&function.result))
        .filter(|ty| foreign_type_name(ty).is_some_and(|name| name != owner))
        .count()
}

fn foreign_type_name(ty: &ProjectedType) -> Option<&str> {
    match ty {
        ProjectedType::Foreign { name, .. } => Some(name),
        ProjectedType::Optional(inner) => foreign_type_name(inner),
        _ => None,
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
    output.push_str(if function.error.is_some() {
        " throws dependency-error"
    } else {
        " throws dependency-panic"
    });
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
    let sandbox = containment();
    if dependencies.is_empty() {
        return Ok(Projection {
            cache_identity: String::from("no-rust-dependencies"),
            dependencies: Vec::new(),
            containment: sandbox,
        });
    }
    if sandbox == Containment::Unavailable {
        return Err(ProjectionError {
            message: "Rust dependency projection requires bubblewrap (`bwrap`); install bubblewrap and ensure `bwrap` is available on PATH"
                .to_owned(),
        });
    }
    let workspace = root.join(".trn/dependencies");
    write_workspace(&workspace, dependencies)?;
    if workspace.join("Cargo.lock").exists() {
        run_cargo(&workspace, &["fetch", "--locked"], false)?;
    } else {
        run_cargo(&workspace, &["fetch"], false)?;
    }
    let identity = cache_identity(root, &workspace, dependencies, sandbox)?;
    let cache_path = workspace.join(format!("projection-{identity}.json"));
    if let Ok(bytes) = fs::read(&cache_path) {
        let mut cached =
            serde_json::from_slice::<Projection>(&bytes).map_err(|error| ProjectionError {
                message: format!(
                    "invalid cached dependency projection `{}`: {error}",
                    cache_path.display()
                ),
            })?;
        cached.containment = sandbox;
        return Ok(cached);
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
    let projection = Projection {
        cache_identity: identity,
        dependencies: projected,
        containment: sandbox,
    };
    let bytes = serde_json::to_vec_pretty(&projection).map_err(|error| ProjectionError {
        message: format!("cannot serialize dependency projection: {error}"),
    })?;
    write_if_changed(&cache_path, &bytes)?;
    Ok(projection)
}

fn write_workspace(
    directory: &Path,
    dependencies: &[RustDependency],
) -> Result<(), ProjectionError> {
    fs::create_dir_all(directory.join("src")).map_err(io_error("create dependency workspace"))?;
    let mut manifest = String::from(
        "[package]\nname = \"terrane_dependency_projection\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\n",
    );
    for dependency in dependencies
        .iter()
        .filter(|dependency| dependency.target.is_none())
    {
        write_dependency_spec(&mut manifest, dependency);
    }
    let targets = dependencies
        .iter()
        .filter_map(|dependency| dependency.target.as_deref())
        .collect::<BTreeSet<_>>();
    for target in targets {
        writeln!(manifest, "\n[target.{target:?}.dependencies]")
            .expect("writing to a string cannot fail");
        for dependency in dependencies
            .iter()
            .filter(|dependency| dependency.target.as_deref() == Some(target))
        {
            write_dependency_spec(&mut manifest, dependency);
        }
    }
    manifest.push_str("\n[workspace]\n");
    write_if_changed(&directory.join("Cargo.toml"), manifest.as_bytes())?;
    write_if_changed(&directory.join("src/lib.rs"), b"")?;
    Ok(())
}

fn write_dependency_spec(manifest: &mut String, dependency: &RustDependency) {
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
}

fn run_cargo(directory: &Path, arguments: &[&str], nightly: bool) -> Result<(), ProjectionError> {
    let sandboxed = nightly && containment() == Containment::Enforced;
    let canonical_directory = if sandboxed {
        Some(
            directory
                .canonicalize()
                .map_err(io_error("canonicalize dependency projection workspace"))?,
        )
    } else {
        None
    };
    let working_directory = canonical_directory.as_deref().unwrap_or(directory);
    let mut command = if sandboxed {
        let mut command = Command::new("bwrap");
        command.args([
            "--die-with-parent",
            "--unshare-all",
            "--ro-bind",
            "/",
            "/",
            "--dev",
            "/dev",
            "--proc",
            "/proc",
            "--tmpfs",
            "/tmp",
            "--bind",
        ]);
        command
            .arg(working_directory)
            .arg(working_directory)
            .arg("--")
            .arg("cargo");
        command
    } else {
        Command::new("cargo")
    };
    if nightly {
        command.arg(format!("+{RUSTDOC_TOOLCHAIN}"));
    }
    let output = command
        .args(arguments)
        .current_dir(working_directory)
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
        let rust_path = extern_rust_path(
            dependency,
            &public_paths
                .get(id)
                .cloned()
                .unwrap_or_else(|| path.join("::")),
        );
        let docs = item["docs"].as_str().map(str::to_owned);
        let inner = &item["inner"];
        let projected = if let Some(function) = inner.get("function") {
            project_function(function, index, paths, Some(&name)).map(ProjectedKind::Function)
        } else if let Some(structure) = inner.get("struct") {
            if has_type_parameters(structure) {
                Err("type has generic or lifetime parameters".to_owned())
            } else {
                let (methods, method_declines) = project_methods(structure, index, paths);
                declined.extend(
                    method_declines
                        .into_iter()
                        .map(|(name, reason)| DeclinedItem {
                            rust_path: format!("{rust_path}::{name}"),
                            reason,
                        }),
                );
                Ok(ProjectedKind::ForeignType { methods })
            }
        } else if let Some(enumeration) = inner.get("enum") {
            if has_type_parameters(enumeration) {
                Err("type has generic or lifetime parameters".to_owned())
            } else {
                let data_carrying = enumeration["variants"].as_array().is_some_and(|variants| {
                    variants.iter().any(|id| {
                        value_id(id)
                            .and_then(|id| index.get(&id))
                            .and_then(|variant| variant["inner"]["variant"]["kind"].as_str())
                            .is_none()
                    })
                });
                Ok(ProjectedKind::Enum { data_carrying })
            }
        } else if inner.get("trait").is_some() {
            Err(
                "trait projection is deferred until receiver-first trait namespaces are implemented"
                    .to_owned(),
            )
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

fn has_type_parameters(item: &Value) -> bool {
    item["generics"]["params"]
        .as_array()
        .is_some_and(|parameters| !parameters.is_empty())
}

fn extern_rust_path(dependency: &RustDependency, path: &str) -> String {
    let mut segments = path.split("::");
    let _package_root = segments.next();
    std::iter::once(dependency.name.replace('-', "_"))
        .chain(segments.map(str::to_owned))
        .collect::<Vec<_>>()
        .join("::")
}

fn project_methods(
    structure: &Value,
    index: &serde_json::Map<String, Value>,
    paths: &serde_json::Map<String, Value>,
) -> (Vec<ProjectedFunction>, Vec<(String, String)>) {
    let mut candidates = Vec::new();
    let mut declined = Vec::new();
    let Some(impls) = structure["impls"].as_array() else {
        return (Vec::new(), declined);
    };
    for impl_id in impls {
        let Some(implementation) = value_id(impl_id)
            .and_then(|id| index.get(&id))
            .and_then(|item| item["inner"]["impl"].as_object())
        else {
            continue;
        };
        if implementation["is_negative"].as_bool() == Some(true) {
            continue;
        }
        let inherent = implementation["trait"].is_null();
        let Some(method_ids) = implementation["items"].as_array() else {
            continue;
        };
        for method_id in method_ids {
            let Some(item) = value_id(method_id).and_then(|id| index.get(&id)) else {
                continue;
            };
            if item["visibility"].as_str() != Some("public") {
                continue;
            }
            let Some(name) = item["name"].as_str() else {
                continue;
            };
            if !inherent {
                declined.push((
                    name.to_owned(),
                    "trait method projection is deferred until receiver-first trait namespaces are implemented"
                        .to_owned(),
                ));
                continue;
            }
            let Some(function) = item["inner"].get("function") else {
                declined.push((
                    name.to_owned(),
                    "item kind has no Terrane method projection".to_owned(),
                ));
                continue;
            };
            match project_function(function, index, paths, Some(name)) {
                Ok(method) => candidates.push(method),
                Err(reason) => declined.push((name.to_owned(), reason)),
            }
        }
    }
    let mut methods = Vec::new();
    while let Some(method) = candidates.pop() {
        let name = method.name.clone();
        let mut matching = vec![method];
        let mut index = 0;
        while index < candidates.len() {
            if candidates[index].name == name {
                matching.push(candidates.swap_remove(index));
            } else {
                index += 1;
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
    (methods, declined)
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
    if parameters
        .iter()
        .any(|parameter| matches!(parameter.ty, ProjectedType::Optional(_)))
        || matches!(result, ProjectedType::Optional(_))
    {
        return Err(
            "Option projection is deferred until general `T|none` semantic types are implemented"
                .to_owned(),
        );
    }
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
            .and_then(|bound| value_id(&bound["trait_bound"]["trait"]["id"]))
        else {
            return Err(format!("open generic `{name}`"));
        };
        let Some(implementations) = index
            .get(&trait_id)
            .and_then(|item| item["inner"]["trait"]["implementations"].as_array())
        else {
            return Err(format!("generic bound for `{name}` has no closed impl set"));
        };
        let mut candidates = implementations
            .iter()
            .filter_map(value_id)
            .filter_map(|id| index.get(&id))
            .filter_map(|item| item["inner"]["impl"].get("for"))
            .filter_map(|ty| project_type(ty, paths, &BTreeMap::new()).ok())
            .collect::<Vec<_>>();
        let mut unique = Vec::new();
        for candidate in candidates.drain(..) {
            if !unique.contains(&candidate) {
                unique.push(candidate);
            }
        }
        let direct = unique
            .iter()
            .filter(|ty| {
                matches!(
                    ty,
                    ProjectedType::String
                        | ProjectedType::Bytes
                        | ProjectedType::Bool
                        | ProjectedType::Int
                        | ProjectedType::Float
                )
            })
            .cloned()
            .collect::<Vec<_>>();
        let selected = if direct.is_empty() { &unique } else { &direct };
        let [chosen] = selected.as_slice() else {
            return Err(if selected.is_empty() {
                format!("generic bound for `{name}` has no Terrane-representable impl")
            } else {
                format!(
                    "generic bound for `{name}` has {} viable Terrane representations and requires a caller-chosen type",
                    selected.len()
                )
            });
        };
        result.insert(name.to_owned(), chosen.clone());
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
            "str" => Ok(ProjectedType::String),
            "f64" => Ok(ProjectedType::Float),
            "i64" => Ok(ProjectedType::Int),
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
        format!("/deps/{}", dependency.name.replace('_', "-"))
    } else {
        format!(
            "/deps/{}/{}",
            dependency.name.replace('_', "-"),
            modules.join("/")
        )
    }
}

#[must_use]
pub fn namespace_for_rust_path(dependency: &ProjectedDependency, rust_path: &str) -> String {
    let modules = rust_path.split("::").skip(1).collect::<Vec<_>>();
    let modules = &modules[..modules.len().saturating_sub(1)];
    if modules.is_empty() {
        format!("/deps/{}", dependency.name.replace('_', "-"))
    } else {
        format!(
            "/deps/{}/{}",
            dependency.name.replace('_', "-"),
            modules
                .iter()
                .map(|segment| segment.replace('_', "-"))
                .collect::<Vec<_>>()
                .join("/")
        )
    }
}

fn safe_parameter_name(name: &str) -> String {
    if matches!(
        name,
        "as" | "await"
            | "case"
            | "catch"
            | "class"
            | "else"
            | "finally"
            | "for"
            | "function"
            | "goto"
            | "if"
            | "import"
            | "is"
            | "label"
            | "linear"
            | "match"
            | "move"
            | "namespace"
            | "ref"
            | "return"
            | "rust"
            | "throw"
            | "unsafe"
            | "use"
            | "when"
            | "yield"
    ) {
        format!("{name}_")
    } else {
        name.to_owned()
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

fn cache_identity(
    root: &Path,
    workspace: &Path,
    dependencies: &[RustDependency],
    containment: Containment,
) -> Result<String, ProjectionError> {
    let manifest = fs::read(root.join(crate::MANIFEST_FILE_NAME)).unwrap_or_default();
    let lock = fs::read(workspace.join("Cargo.lock"))
        .or_else(|_| fs::read(root.join("Cargo.lock")))
        .unwrap_or_default();
    let rustc = tool_version("rustc", &["-vV"])?;
    let nightly = format!("+{RUSTDOC_TOOLCHAIN}");
    let rustdoc = tool_version("rustdoc", &[&nightly, "--version"])?;
    let mut hash = Sha256::new();
    for (label, bytes) in [
        ("manifest", manifest.as_slice()),
        ("lock", lock.as_slice()),
        ("inputs", format!("{dependencies:?}").as_bytes()),
        ("rustc", rustc.as_bytes()),
        ("rustdoc", rustdoc.as_bytes()),
        ("schema", PROJECTION_SCHEMA.as_bytes()),
        ("containment", format!("{containment:?}").as_bytes()),
    ] {
        hash.update(label.len().to_le_bytes());
        hash.update(label.as_bytes());
        hash.update(bytes.len().to_le_bytes());
        hash.update(bytes);
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn tool_version(program: &str, arguments: &[&str]) -> Result<String, ProjectionError> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot inspect dependency projection toolchain: {error}"),
        })?;
    if !output.status.success() {
        return Err(ProjectionError {
            message: format!(
                "cannot inspect dependency projection toolchain: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn containment() -> Containment {
    static CONTAINMENT: LazyLock<Containment> = LazyLock::new(|| {
        let available = Command::new("bwrap")
            .args([
                "--die-with-parent",
                "--unshare-all",
                "--ro-bind",
                "/",
                "/",
                "--dev",
                "/dev",
                "--proc",
                "/proc",
                "--",
                "/bin/true",
            ])
            .output()
            .is_ok_and(|output| output.status.success());
        if available {
            Containment::Enforced
        } else {
            Containment::Unavailable
        }
    });
    *CONTAINMENT
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Receiver, has_type_parameters, receiver_kind};

    #[test]
    fn type_parameter_guard_reads_destructured_type_descriptors() {
        for descriptor in [
            json!({"generics": {"params": [{"name": "T"}]}}),
            json!({"generics": {"params": [{"lifetime": "'a"}]}}),
        ] {
            assert!(has_type_parameters(&descriptor));
        }
        assert!(!has_type_parameters(&json!({"generics": {"params": []}})));
    }

    #[test]
    fn receiver_kind_preserves_mutable_borrows() {
        assert_eq!(
            receiver_kind(&json!({"borrowed_ref": {"is_mutable": true}})),
            Receiver::MutableBorrow
        );
        assert_eq!(
            receiver_kind(&json!({"borrowed_ref": {"is_mutable": false}})),
            Receiver::Borrow
        );
        assert_eq!(receiver_kind(&json!({})), Receiver::Move);
    }
}
