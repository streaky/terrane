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
const PROJECTION_SCHEMA: &str = "7";
const MAX_PROJECTION_CACHE_RECORDS: usize = 4;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Projection {
    pub cache_identity: String,
    pub dependencies: Vec<ProjectedDependency>,
    pub containment: Containment,
    #[serde(default)]
    pub removed: Vec<RemovedItem>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Containment {
    Enforced,
    Unavailable,
}
#[derive(Clone, Copy)]
enum CargoToolchain {
    Default,
    RustdocNightly,
}

#[derive(Clone, Copy)]
enum CargoExecution {
    Host,
    Contained,
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
pub struct RemovedItem {
    pub namespace: String,
    pub name: String,
    pub previous_version: String,
    pub current_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProjectionHistory {
    #[serde(default = "projection_history_format")]
    format: u32,
    dependencies: Vec<ProjectionHistoryDependency>,
    #[serde(default)]
    removed: Vec<RemovedItem>,
}

fn projection_history_format() -> u32 {
    1
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProjectionHistoryDependency {
    name: String,
    version: String,
    members: BTreeSet<(String, String)>,
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
    ForeignType {
        methods: Vec<ProjectedFunction>,
    },
    Enum {
        data_carrying: bool,
        comparable: bool,
    },
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
    RustInt(String),
    Float,
    Float32,
    Char,
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
            Self::Int | Self::RustInt(_) => "int".to_owned(),
            Self::Float => "float64".to_owned(),
            Self::Float32 => "float32".to_owned(),
            Self::Char | Self::String => "string".to_owned(),
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
        let imports = expanded_source_imports(&all_items, imports);
        let mut sources = Vec::new();
        for (namespace, names) in &imports {
            let selected = all_items
                .iter()
                .copied()
                .filter(|item| item.namespace == *namespace && names.contains(&item.name))
                .collect::<Vec<_>>();
            if selected.is_empty() {
                continue;
            }
            let foreign = collect_source_foreign(&all_items, &selected);
            let mut aliases = foreign_aliases(&foreign);
            for (rust_path, alias) in &mut aliases {
                if projected_item_for_foreign(&all_items, rust_path, &foreign[rust_path])
                    .is_some_and(|item| item.namespace == *namespace)
                {
                    alias.clone_from(&foreign[rust_path]);
                }
            }
            let mut ordered_foreign = foreign.iter().collect::<Vec<_>>();
            ordered_foreign.sort_by_key(|(rust_path, name)| {
                let dependency_count = all_items
                    .iter()
                    .copied()
                    .find(|item| item.rust_path == **rust_path)
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
                (dependency_count, aliases.get(*rust_path))
            });
            let mut text = format!("namespace {}\n\n", namespace.trim_start_matches('/'));
            for (rust_path, _) in ordered_foreign {
                let name = &aliases[rust_path];
                if let Some(item) =
                    projected_item_for_foreign(&all_items, rust_path, &foreign[rust_path])
                {
                    if item.namespace != *namespace {
                        write!(text, "from {} import {}", item.namespace, item.name)
                            .expect("writing to a string cannot fail");
                        if name != &item.name {
                            write!(text, " as {name}").expect("writing to a string cannot fail");
                        }
                        text.push('\n');
                        continue;
                    }
                }
                writeln!(text, "class {name}").expect("writing to a string cannot fail");
                if let Some(ProjectedItem {
                    kind: ProjectedKind::ForeignType { methods },
                    ..
                }) = projected_item_for_foreign(&all_items, rust_path, &foreign[rust_path])
                {
                    for method in methods {
                        render_function(&mut text, method, false, 4, &aliases);
                    }
                }
                text.push('\n');
            }
            for item in selected {
                if let ProjectedKind::Function(function) = &item.kind {
                    render_function(&mut text, function, true, 0, &aliases);
                }
            }
            sources.push((namespace.clone(), text));
        }
        sources
    }

    #[must_use]
    pub fn foreign_imports(&self, namespace: &str) -> BTreeMap<String, String> {
        let all_items = self
            .dependencies
            .iter()
            .flat_map(|dependency| dependency.items.iter())
            .collect::<Vec<_>>();
        let mut foreign = BTreeMap::new();
        for item in all_items
            .iter()
            .copied()
            .filter(|item| item.namespace == namespace)
        {
            if let ProjectedKind::Function(function) = &item.kind {
                collect_foreign_function(function, &mut foreign);
            }
        }
        foreign.retain(|rust_path, name| {
            projected_item_for_foreign(&all_items, rust_path, name).is_none()
        });
        foreign_aliases(&foreign)
            .into_iter()
            .map(|(rust_path, name)| (name, rust_path))
            .collect()
    }

    #[must_use]
    pub fn item(&self, namespace: &str, name: &str) -> Option<&ProjectedItem> {
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .find(|item| item.namespace == namespace && item.name == name)
    }
    #[must_use]
    pub(crate) fn dependency_name(&self, namespace: &str, name: &str) -> Option<&str> {
        self.dependencies
            .iter()
            .find(|dependency| {
                dependency
                    .items
                    .iter()
                    .any(|item| item.namespace == namespace && item.name == name)
            })
            .map(|dependency| dependency.name.as_str())
    }

    #[must_use]
    pub fn foreign_rust_path(&self, namespace: &str, name: &str) -> Option<&str> {
        self.item(namespace, name)
            .filter(|item| {
                matches!(
                    item.kind,
                    ProjectedKind::ForeignType { .. } | ProjectedKind::Enum { .. }
                )
            })
            .map(|item| item.rust_path.as_str())
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

    #[must_use]
    pub(crate) fn is_unit_variant(&self, item: &ProjectedItem) -> bool {
        self.dependencies
            .iter()
            .flat_map(|dependency| &dependency.items)
            .any(|candidate| {
                matches!(
                    candidate.kind,
                    ProjectedKind::Enum {
                        data_carrying: false,
                        ..
                    }
                ) && item
                    .rust_path
                    .starts_with(&format!("{}::", candidate.rust_path))
            })
    }
}

fn expanded_source_imports(
    all_items: &[&ProjectedItem],
    imports: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut expanded = imports.clone();
    loop {
        let previous_count = expanded.values().map(BTreeSet::len).sum::<usize>();
        let selected = all_items
            .iter()
            .copied()
            .filter(|item| {
                expanded
                    .get(&item.namespace)
                    .is_some_and(|names| names.contains(&item.name))
            })
            .collect::<Vec<_>>();
        for (rust_path, name) in collect_source_foreign(all_items, &selected) {
            if let Some(item) = projected_item_for_foreign(all_items, &rust_path, &name) {
                expanded
                    .entry(item.namespace.clone())
                    .or_default()
                    .insert(item.name.clone());
            }
        }
        if expanded.values().map(BTreeSet::len).sum::<usize>() == previous_count {
            return expanded;
        }
    }
}

fn projected_item_for_foreign<'a>(
    all_items: &'a [&ProjectedItem],
    rust_path: &str,
    name: &str,
) -> Option<&'a ProjectedItem> {
    if let Some(exact) = all_items
        .iter()
        .copied()
        .find(|item| item.rust_path == rust_path)
    {
        return Some(exact);
    }
    let mut candidates = all_items
        .iter()
        .copied()
        .filter(|item| item.name == name)
        .filter_map(|item| {
            let parent = item.rust_path.rsplit_once("::")?.0;
            rust_path
                .starts_with(&format!("{parent}::"))
                .then_some((parent.len(), item))
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|(prefix_len, _)| std::cmp::Reverse(*prefix_len));
    let (best_len, best) = candidates.first().copied()?;
    (candidates
        .get(1)
        .is_none_or(|(next_len, _)| *next_len < best_len))
    .then_some(best)
}

fn collect_source_foreign(
    all_items: &[&ProjectedItem],
    selected: &[&ProjectedItem],
) -> BTreeMap<String, String> {
    let mut foreign = BTreeMap::<String, String>::new();
    for item in selected {
        match &item.kind {
            ProjectedKind::Function(function) => {
                collect_foreign_function(function, &mut foreign);
            }
            ProjectedKind::ForeignType { methods } => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
                for method in methods {
                    collect_foreign_function(method, &mut foreign);
                }
            }
            ProjectedKind::Enum { .. } => {
                foreign.insert(item.rust_path.clone(), item.name.clone());
            }
        }
    }
    loop {
        let previous_len = foreign.len();
        let referenced = foreign.keys().cloned().collect::<Vec<_>>();
        for rust_path in referenced {
            let Some(ProjectedItem {
                kind: ProjectedKind::ForeignType { methods },
                ..
            }) = all_items
                .iter()
                .copied()
                .find(|item| item.rust_path == rust_path)
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

fn foreign_aliases(foreign: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let counts = foreign.values().fold(BTreeMap::new(), |mut counts, name| {
        *counts.entry(name.as_str()).or_insert(0_usize) += 1;
        counts
    });
    foreign
        .iter()
        .map(|(rust_path, name)| {
            let alias = if counts[name.as_str()] == 1 {
                name.clone()
            } else {
                rust_path.replace("::", "-").replace('_', "-")
            };
            (rust_path.clone(), alias)
        })
        .collect()
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
            foreign.insert(rust_path.clone(), name.clone());
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

fn render_function(
    output: &mut String,
    function: &ProjectedFunction,
    public: bool,
    indent: usize,
    foreign_aliases: &BTreeMap<String, String>,
) {
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
        write!(
            output,
            " {}",
            projected_type_name(&function.result, foreign_aliases)
        )
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
            write!(
                output,
                "{} {}",
                parameter.name,
                projected_type_name(&parameter.ty, foreign_aliases)
            )
            .expect("writing to a string cannot fail");
        }
    }
    output.push('\n');
}

fn projected_type_name(ty: &ProjectedType, foreign_aliases: &BTreeMap<String, String>) -> String {
    match ty {
        ProjectedType::Foreign { rust_path, name } => foreign_aliases
            .get(rust_path)
            .cloned()
            .unwrap_or_else(|| name.clone()),
        ProjectedType::Optional(inner) => {
            format!("{}|none", projected_type_name(inner, foreign_aliases))
        }
        _ => ty.terrane_name(),
    }
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
            removed: Vec::new(),
            containment: sandbox,
        });
    }
    let workspace = root.join(".trn/dependencies");
    write_workspace(&workspace, dependencies)?;
    if workspace.join("Cargo.lock").exists() {
        run_cargo(
            &workspace,
            &["fetch", "--locked"],
            CargoToolchain::Default,
            CargoExecution::Host,
        )?;
    } else {
        run_cargo(
            &workspace,
            &["fetch"],
            CargoToolchain::Default,
            CargoExecution::Host,
        )?;
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
        apply_projection_history(root, &mut cached)?;
        prune_projection_cache(&workspace, &cache_path)?;
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
            CargoToolchain::RustdocNightly,
            if sandbox == Containment::Enforced {
                CargoExecution::Contained
            } else {
                CargoExecution::Host
            },
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
        removed: Vec::new(),
    };
    let bytes = serde_json::to_vec_pretty(&projection).map_err(|error| ProjectionError {
        message: format!("cannot serialize dependency projection: {error}"),
    })?;
    write_if_changed(&cache_path, &bytes)?;
    let mut projection = projection;
    apply_projection_history(root, &mut projection)?;
    prune_projection_cache(&workspace, &cache_path)?;
    Ok(projection)
}
fn apply_projection_history(
    root: &Path,
    projection: &mut Projection,
) -> Result<(), ProjectionError> {
    let path = root.join("terrane-projection.lock");
    let previous = match fs::read(&path) {
        Ok(bytes) => {
            let history = serde_json::from_slice::<ProjectionHistory>(&bytes).map_err(|error| {
                ProjectionError {
                    message: format!("invalid projection history `{}`: {error}", path.display()),
                }
            })?;
            if history.format != 1 {
                return Err(ProjectionError {
                    message: format!(
                        "unsupported projection history format {} in `{}`",
                        history.format,
                        path.display()
                    ),
                });
            }
            Some(history)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => {
            return Err(ProjectionError {
                message: format!(
                    "cannot read projection history `{}`: {error}",
                    path.display()
                ),
            });
        }
    };
    let dependencies = projection
        .dependencies
        .iter()
        .map(|dependency| ProjectionHistoryDependency {
            name: dependency.name.clone(),
            version: dependency.version.clone(),
            members: dependency
                .items
                .iter()
                .map(|item| (item.namespace.clone(), item.name.clone()))
                .collect(),
        })
        .collect::<Vec<_>>();
    let mut removed = previous
        .as_ref()
        .map_or_else(Vec::new, |history| history.removed.clone());
    removed.retain(|removed| {
        !dependencies.iter().any(|dependency| {
            dependency
                .members
                .contains(&(removed.namespace.clone(), removed.name.clone()))
        })
    });
    if let Some(previous) = &previous {
        for old in &previous.dependencies {
            let Some(current) = dependencies
                .iter()
                .find(|dependency| dependency.name == old.name)
            else {
                continue;
            };
            if old.version == current.version {
                continue;
            }
            for (namespace, name) in old.members.difference(&current.members) {
                let removed_item = RemovedItem {
                    namespace: namespace.clone(),
                    name: name.clone(),
                    previous_version: old.version.clone(),
                    current_version: current.version.clone(),
                };
                if !removed.iter().any(|existing| {
                    existing.namespace == removed_item.namespace
                        && existing.name == removed_item.name
                }) {
                    removed.push(removed_item);
                }
            }
        }
    }
    removed
        .sort_by(|left, right| (&left.namespace, &left.name).cmp(&(&right.namespace, &right.name)));
    projection.removed.clone_from(&removed);
    let history = ProjectionHistory {
        format: 1,
        dependencies,
        removed,
    };
    let mut bytes = serde_json::to_vec_pretty(&history).map_err(|error| ProjectionError {
        message: format!("cannot serialize projection history: {error}"),
    })?;
    bytes.push(b'\n');
    write_if_changed(&path, &bytes)
}

fn prune_projection_cache(directory: &Path, retained: &Path) -> Result<(), ProjectionError> {
    let entries = fs::read_dir(directory).map_err(io_error("read dependency projection cache"))?;
    let mut previous = Vec::new();
    for entry in entries {
        let entry = entry.map_err(io_error("read dependency projection cache entry"))?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if path == retained
            || !name.starts_with("projection-")
            || path.extension() != Some(std::ffi::OsStr::new("json"))
        {
            continue;
        }
        let modified = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .map_err(io_error("read dependency projection cache metadata"))?;
        previous.push((modified, path));
    }
    previous.sort_by(|left, right| right.cmp(left));
    for (_, path) in previous
        .into_iter()
        .skip(MAX_PROJECTION_CACHE_RECORDS.saturating_sub(1))
    {
        fs::remove_file(&path).map_err(io_error("remove stale dependency projection"))?;
    }
    Ok(())
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
        .filter(|dependency| dependency.cargo_manifest_table() == "dependencies")
    {
        manifest.push_str(&dependency.cargo_dependency_spec());
    }
    let target_tables = dependencies
        .iter()
        .map(RustDependency::cargo_manifest_table)
        .filter(|table| table != "dependencies")
        .collect::<BTreeSet<_>>();
    for table in target_tables {
        writeln!(manifest, "\n[{table}]").expect("writing to a string cannot fail");
        for dependency in dependencies
            .iter()
            .filter(|dependency| dependency.cargo_manifest_table() == table)
        {
            manifest.push_str(&dependency.cargo_dependency_spec());
        }
    }
    manifest.push_str("\n[workspace]\n");
    write_if_changed(&directory.join("Cargo.toml"), manifest.as_bytes())?;
    write_if_changed(&directory.join("src/lib.rs"), b"")?;
    Ok(())
}

fn run_cargo(
    directory: &Path,
    arguments: &[&str],
    toolchain: CargoToolchain,
    execution: CargoExecution,
) -> Result<(), ProjectionError> {
    let sandboxed = matches!(execution, CargoExecution::Contained);
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
    crate::cargo_toolchain::configure_cargo_command(&mut command);
    if matches!(toolchain, CargoToolchain::RustdocNightly) {
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
    let mut resolved_paths = paths.clone();
    for (id, public_path) in &public_paths {
        if let Some(summary) = resolved_paths.get_mut(id) {
            summary["path"] = Value::Array(
                public_path
                    .split("::")
                    .map(|segment| Value::String(segment.to_owned()))
                    .collect(),
            );
        }
    }
    let mut items = Vec::new();
    let mut projected_enum_items = Vec::new();
    let mut declined = Vec::new();
    let mut projected_trait_items = Vec::new();
    let mut projected_associated_items = Vec::new();
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
            project_function(function, index, &resolved_paths, Some(&name))
                .map(ProjectedKind::Function)
        } else if let Some(structure) = inner.get("struct") {
            if has_type_parameters(structure) {
                Err("type has generic or lifetime parameters".to_owned())
            } else {
                let (methods, trait_methods, method_declines) =
                    project_methods(structure, index, &resolved_paths, &rust_path);
                for method in methods.iter().filter(|method| method.receiver.is_none()) {
                    projected_associated_items.push(ProjectedItem {
                        namespace: namespace.clone(),
                        name: method.name.clone(),
                        rust_path: format!("{rust_path}::{}", method.name),
                        docs: None,
                        kind: ProjectedKind::Function(method.clone()),
                    });
                }
                for (trait_path, method) in trait_methods {
                    let trait_segments = trait_path
                        .split("::")
                        .map(str::to_owned)
                        .collect::<Vec<_>>();
                    let trait_namespace = dependency_namespace(dependency, &trait_segments);
                    let trait_rust_path = extern_rust_path(dependency, &trait_path);
                    projected_trait_items.push(ProjectedItem {
                        namespace: trait_namespace,
                        name: method.name.clone(),
                        rust_path: format!("<{rust_path} as {trait_rust_path}>::{}", method.name),
                        docs: None,
                        kind: ProjectedKind::Function(method),
                    });
                }
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
                            != Some("plain")
                    })
                });
                if !data_carrying {
                    let variant_namespace = dependency_namespace(dependency, &path);
                    for variant_id in string_or_number_array(&enumeration["variants"]) {
                        let Some(variant) = index.get(&variant_id) else {
                            continue;
                        };
                        let Some(variant_name) = variant["name"].as_str() else {
                            continue;
                        };
                        projected_enum_items.push(ProjectedItem {
                            namespace: variant_namespace.clone(),
                            name: variant_name.to_owned(),
                            rust_path: format!("{rust_path}::{variant_name}"),
                            docs: variant["docs"].as_str().map(str::to_owned),
                            kind: ProjectedKind::Function(ProjectedFunction {
                                name: variant_name.to_owned(),
                                parameters: Vec::new(),
                                result: ProjectedType::Foreign {
                                    rust_path: rust_path.clone(),
                                    name: name.clone(),
                                },
                                error: None,
                                is_async: false,
                                receiver: None,
                            }),
                        });
                    }
                }
                Ok(ProjectedKind::Enum {
                    data_carrying,
                    comparable: implements_trait(
                        enumeration,
                        index,
                        &resolved_paths,
                        "core::cmp::PartialEq",
                    ),
                })
            }
        } else if inner.get("trait").is_some() {
            continue;
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
    items.extend(projected_enum_items);
    projected_associated_items.sort_by(|left, right| {
        (&left.namespace, &left.name, &left.rust_path).cmp(&(
            &right.namespace,
            &right.name,
            &right.rust_path,
        ))
    });
    let mut associated_index = 0;
    while associated_index < projected_associated_items.len() {
        let first = associated_index;
        let key = (
            projected_associated_items[first].namespace.clone(),
            projected_associated_items[first].name.clone(),
        );
        while associated_index < projected_associated_items.len()
            && projected_associated_items[associated_index].namespace == key.0
            && projected_associated_items[associated_index].name == key.1
        {
            associated_index += 1;
        }
        if associated_index - first == 1
            && !items
                .iter()
                .any(|item| item.namespace == key.0 && item.name == key.1)
        {
            items.push(projected_associated_items[first].clone());
        } else {
            declined.extend(
                projected_associated_items[first..associated_index]
                    .iter()
                    .map(|item| DeclinedItem {
                        rust_path: item.rust_path.clone(),
                        reason: "multiple receiver-free associated functions with the same projected name"
                            .to_owned(),
                    }),
            );
        }
    }
    projected_trait_items.sort_by(|left, right| {
        (&left.namespace, &left.name, &left.rust_path).cmp(&(
            &right.namespace,
            &right.name,
            &right.rust_path,
        ))
    });
    let mut index = 0;
    while index < projected_trait_items.len() {
        let first = index;
        let key = (
            projected_trait_items[index].namespace.clone(),
            projected_trait_items[index].name.clone(),
        );
        while index < projected_trait_items.len()
            && projected_trait_items[index].namespace == key.0
            && projected_trait_items[index].name == key.1
        {
            index += 1;
        }
        if index - first == 1
            && !items
                .iter()
                .any(|item| item.namespace == key.0 && item.name == key.1)
        {
            items.push(projected_trait_items[first].clone());
        } else {
            declined.extend(
                projected_trait_items[first..index]
                    .iter()
                    .map(|item| DeclinedItem {
                        rust_path: item.rust_path.clone(),
                        reason: "trait method has multiple concrete receiver implementations"
                            .to_owned(),
                    }),
            );
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

type ProjectedMethods = (
    Vec<ProjectedFunction>,
    Vec<(String, ProjectedFunction)>,
    Vec<(String, String)>,
);

#[expect(
    clippy::too_many_lines,
    reason = "rustdoc impl traversal keeps trait and inherent decisions in one deterministic pass"
)]
fn project_methods(
    structure: &Value,
    index: &serde_json::Map<String, Value>,
    paths: &serde_json::Map<String, Value>,
    owner_rust_path: &str,
) -> ProjectedMethods {
    let mut candidates = Vec::new();
    let mut trait_methods = Vec::new();
    let mut declined = Vec::new();
    let Some(impls) = structure["impls"].as_array() else {
        return (Vec::new(), trait_methods, declined);
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
            if inherent && item["visibility"].as_str() != Some("public") {
                continue;
            }
            let Some(name) = item["name"].as_str() else {
                continue;
            };
            let Some(function) = item["inner"].get("function") else {
                declined.push((
                    name.to_owned(),
                    "item kind has no Terrane method projection".to_owned(),
                ));
                continue;
            };
            if !inherent {
                let Some(trait_path) = implementation_trait_path(&implementation["trait"], paths)
                else {
                    declined.push((
                        name.to_owned(),
                        "trait method is not declared by this dependency crate".to_owned(),
                    ));
                    continue;
                };
                match project_function(function, index, paths, Some(name)) {
                    Ok(mut method) => {
                        let Some(receiver) = method.receiver.take() else {
                            declined
                                .push((name.to_owned(), "trait method has no receiver".to_owned()));
                            continue;
                        };
                        method.parameters.insert(
                            0,
                            ProjectedParameter {
                                name: "receiver".to_owned(),
                                ty: ProjectedType::Foreign {
                                    rust_path: owner_rust_path.to_owned(),
                                    name: owner_rust_path
                                        .rsplit("::")
                                        .next()
                                        .unwrap_or(owner_rust_path)
                                        .to_owned(),
                                },
                                borrowed: receiver != Receiver::Move,
                                mutable_borrow: receiver == Receiver::MutableBorrow,
                            },
                        );
                        trait_methods.push((trait_path, method));
                    }
                    Err(reason) => declined.push((name.to_owned(), reason)),
                }
                continue;
            }
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
    (methods, trait_methods, declined)
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
        let projected_type = project_type(ty, index, paths, &generic_types)?;
        let borrowed = ty.get("borrowed_ref").is_some();
        let mutable_borrow = ty["borrowed_ref"]["is_mutable"].as_bool() == Some(true);
        if mutable_borrow && !matches!(projected_type, ProjectedType::Foreign { .. }) {
            return Err("mutable borrowed primitive parameters are not representable".to_owned());
        }
        parameters.push(ProjectedParameter {
            name: safe_parameter_name(name),
            ty: projected_type,
            borrowed,
            mutable_borrow,
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
            project_type(value, index, paths, &generic_types)?
        } else if resolved_name(output, paths)
            .is_some_and(|name| name.ends_with("::Option") || name == "Option")
        {
            let value = type_arguments(output)
                .into_iter()
                .next()
                .ok_or_else(|| "Option has no value type".to_owned())?;
            ProjectedType::Optional(Box::new(project_type(value, index, paths, &generic_types)?))
        } else {
            project_type(output, index, paths, &generic_types)?
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
            .filter_map(|ty| project_type(ty, index, paths, &BTreeMap::new()).ok())
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
    index: &serde_json::Map<String, Value>,
    paths: &serde_json::Map<String, Value>,
    generics: &BTreeMap<String, ProjectedType>,
) -> Result<ProjectedType, String> {
    if let Some(reference) = ty.get("borrowed_ref") {
        return project_type(&reference["type"], index, paths, generics);
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
            "f32" => Ok(ProjectedType::Float32),
            "f64" => Ok(ProjectedType::Float),
            "char" => Ok(ProjectedType::Char),
            "i64" => Ok(ProjectedType::Int),
            "i8" | "i16" | "i32" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128"
            | "usize" => Ok(ProjectedType::RustInt(primitive.to_owned())),
            "unit" => Ok(ProjectedType::None),
            other => Err(format!("unsupported primitive `{other}`")),
        };
    }
    if let Some(id) = ty
        .get("resolved_path")
        .and_then(|resolved| value_id(&resolved["id"]))
        && let Some(alias) = index
            .get(&id)
            .and_then(|item| item["inner"].get("type_alias"))
    {
        return project_type(&alias["type"], index, paths, generics);
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
fn implementation_trait_path(
    trait_path: &Value,
    paths: &serde_json::Map<String, Value>,
) -> Option<String> {
    let id = trait_path["id"].as_u64()?.to_string();
    let summary = paths.get(&id)?;
    (summary["crate_id"].as_u64() == Some(0))
        .then(|| string_array(&summary["path"]).join("::"))
        .filter(|path| !path.is_empty())
}

fn implements_trait(
    item: &Value,
    index: &serde_json::Map<String, Value>,
    paths: &serde_json::Map<String, Value>,
    expected: &str,
) -> bool {
    string_or_number_array(&item["impls"])
        .into_iter()
        .any(|id| {
            index
                .get(&id)
                .and_then(|implementation| implementation["inner"]["impl"]["trait"].as_object())
                .and_then(|trait_path| trait_path.get("id"))
                .and_then(Value::as_u64)
                .map(|id| id.to_string())
                .and_then(|id| paths.get(&id))
                .is_some_and(|summary| string_array(&summary["path"]).join("::") == expected)
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
        .map(|segment| segment.to_lowercase().replace('_', "-"))
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
                .map(|segment| segment.to_lowercase().replace('_', "-"))
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
    let target = selected_target(&rustc);
    let nightly = format!("+{RUSTDOC_TOOLCHAIN}");
    let rustdoc = tool_version("rustdoc", &[&nightly, "--version"])?;
    let mut hash = Sha256::new();
    for (label, bytes) in [
        ("manifest", manifest.as_slice()),
        ("lock", lock.as_slice()),
        ("inputs", format!("{dependencies:?}").as_bytes()),
        ("target", target.as_bytes()),
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

fn selected_target(rustc_verbose_version: &str) -> String {
    std::env::var("CARGO_BUILD_TARGET")
        .ok()
        .filter(|target| !target.is_empty())
        .or_else(|| {
            rustc_verbose_version
                .lines()
                .find_map(|line| line.strip_prefix("host: ").map(str::to_owned))
        })
        .unwrap_or_else(|| "unknown-target".to_owned())
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
    use std::{
        collections::{BTreeMap, BTreeSet},
        fs,
    };

    use serde_json::{Value, json};

    use super::{
        Containment, ProjectedDependency, ProjectedFunction, ProjectedItem, ProjectedKind,
        ProjectedType, Projection, Receiver, apply_projection_history, has_type_parameters,
        project_rustdoc, project_type, prune_projection_cache, receiver_kind,
    };
    use crate::RustDependency;

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

    #[test]
    fn method_lookup_keeps_colliding_foreign_types_in_their_namespaces() {
        let projection: Projection = serde_json::from_value(json!({
            "cache_identity": "test",
            "containment": "Unavailable",
            "dependencies": [{
                "name": "witness",
                "package": "witness",
                "version": "1.0.0",
                "declined": [],
                "items": [
                    {
                        "namespace": "/deps/witness/async",
                        "name": "Response",
                        "rust_path": "witness::async::Response",
                        "docs": null,
                        "kind": {"ForeignType": {"methods": [{
                            "name": "touch",
                            "parameters": [],
                            "result": "None",
                            "error": null,
                            "is_async": false,
                            "receiver": "Borrow"
                        }]}}
                    },
                    {
                        "namespace": "/deps/witness/blocking",
                        "name": "Response",
                        "rust_path": "witness::blocking::Response",
                        "docs": null,
                        "kind": {"ForeignType": {"methods": [{
                            "name": "touch",
                            "parameters": [],
                            "result": "None",
                            "error": null,
                            "is_async": false,
                            "receiver": "MutableBorrow"
                        }]}}
                    }
                ]
            }]
        }))
        .unwrap();

        assert_eq!(
            projection
                .method("/deps/witness/async", "Response", "touch")
                .and_then(|method| method.receiver),
            Some(Receiver::Borrow)
        );
        assert_eq!(
            projection
                .method("/deps/witness/blocking", "Response", "touch")
                .and_then(|method| method.receiver),
            Some(Receiver::MutableBorrow)
        );
    }

    #[test]
    fn function_signatures_keep_same_named_foreign_types_distinct() {
        let projection: Projection = serde_json::from_value(json!({
            "cache_identity": "test",
            "containment": "Unavailable",
            "dependencies": [{
                "name": "witness",
                "package": "witness",
                "version": "1.0.0",
                "declined": [],
                "items": [{
                    "namespace": "/deps/witness",
                    "name": "cross",
                    "rust_path": "witness::cross",
                    "docs": null,
                    "kind": {"Function": {
                        "name": "cross",
                        "parameters": [
                            {
                                "name": "left",
                                "ty": {"Foreign": {
                                    "rust_path": "witness::left::Response",
                                    "name": "Response"
                                }},
                                "borrowed": false,
                                "mutable_borrow": false
                            },
                            {
                                "name": "right",
                                "ty": {"Foreign": {
                                    "rust_path": "witness::right::Response",
                                    "name": "Response"
                                }},
                                "borrowed": false,
                                "mutable_borrow": false
                            }
                        ],
                        "result": "None",
                        "error": null,
                        "is_async": false,
                        "receiver": null
                    }}
                }]
            }]
        }))
        .unwrap();

        assert_eq!(
            projection.foreign_imports("/deps/witness"),
            BTreeMap::from([
                (
                    "witness-left-Response".to_owned(),
                    "witness::left::Response".to_owned()
                ),
                (
                    "witness-right-Response".to_owned(),
                    "witness::right::Response".to_owned()
                )
            ])
        );
        let sources = projection.source_for_imports(&BTreeMap::from([(
            "/deps/witness".to_owned(),
            BTreeSet::from(["cross".to_owned()]),
        )]));
        assert!(sources[0].1.contains(
            "function cross throws dependency-panic; left witness-left-Response, right witness-right-Response"
        ));
    }

    #[test]
    fn wider_primitives_project_without_narrowing() {
        let paths = serde_json::Map::new();
        let generics = BTreeMap::new();

        assert_eq!(
            project_type(
                &json!({"primitive": "u8"}),
                &serde_json::Map::new(),
                &paths,
                &generics
            )
            .unwrap(),
            ProjectedType::RustInt("u8".to_owned())
        );
        assert_eq!(
            project_type(
                &json!({"primitive": "f32"}),
                &serde_json::Map::new(),
                &paths,
                &generics
            )
            .unwrap(),
            ProjectedType::Float32
        );
        assert_eq!(
            project_type(
                &json!({"primitive": "char"}),
                &serde_json::Map::new(),
                &paths,
                &generics
            )
            .unwrap(),
            ProjectedType::Char
        );
    }

    #[test]
    fn representable_type_aliases_are_transparent() {
        let paths =
            serde_json::Map::from_iter([("7".to_owned(), json!({"path": ["fixture", "Count"]}))]);
        let index = serde_json::Map::from_iter([(
            "7".to_owned(),
            json!({"inner": {"type_alias": {"type": {"primitive": "u32"}}}}),
        )]);

        assert_eq!(
            project_type(
                &json!({"resolved_path": {"id": 7, "name": "Count", "args": {"angle_bracketed": {"args": []}}}}),
                &index,
                &paths,
                &BTreeMap::new()
            )
            .unwrap(),
            ProjectedType::RustInt("u32".to_owned())
        );
    }

    fn trait_and_enum_rustdoc() -> Value {
        json!({
            "crate_version": "1.0.0",
            "paths": {
                "0": {"crate_id": 0, "path": ["witness"]},
                "1": {"crate_id": 0, "path": ["witness", "Reader"]},
                "2": {"crate_id": 0, "path": ["witness", "Readable"]},
                "5": {"crate_id": 0, "path": ["witness", "Mood"]},
                "11": {"crate_id": 1, "path": ["core", "cmp", "PartialEq"]}
            },
            "index": {
                "0": {
                    "visibility": "public",
                    "inner": {"module": {"items": [1, 2, 5]}}
                },
                "1": {
                    "name": "Reader",
                    "visibility": "public",
                    "docs": null,
                    "inner": {"struct": {"generics": {"params": []}, "impls": [3]}}
                },
                "2": {
                    "name": "Readable",
                    "visibility": "public",
                    "inner": {"trait": {}}
                },
                "3": {
                    "inner": {"impl": {
                        "is_negative": false,
                        "trait": {"id": 2},
                        "items": [4]
                    }}
                },
                "4": {
                    "name": "remaining",
                    "visibility": "default",
                    "inner": {"function": {
                        "header": {"is_unsafe": false, "is_async": false},
                        "generics": {"params": []},
                        "sig": {
                            "inputs": [["self", {"borrowed_ref": {
                                "is_mutable": false,
                                "type": {"generic": "Self"}
                            }}]],
                            "output": {"primitive": "usize"}
                        }
                    }}
                },
                "5": {
                    "name": "Mood",
                    "visibility": "public",
                    "docs": null,
                    "inner": {"enum": {
                        "generics": {"params": []},
                        "variants": [6, 7],
                        "impls": [10]
                    }}
                },
                "6": {
                    "name": "Calm",
                    "docs": null,
                    "inner": {"variant": {"kind": "plain"}}
                },
                "7": {
                    "name": "Busy",
                    "docs": null,
                    "inner": {"variant": {"kind": "plain"}}
                },
                "10": {
                    "inner": {"impl": {"trait": {"id": 11}}}
                }
            }
        })
    }

    #[test]
    fn rustdoc_projects_receiver_first_traits_and_comparable_enum_variants() {
        let document = trait_and_enum_rustdoc();
        let dependency = RustDependency {
            name: "witness".to_owned(),
            package: "witness".to_owned(),
            version: "=1.0.0".to_owned(),
            features: Vec::new(),
            default_features: true,
            target: None,
            effects: Vec::new(),
        };

        let projected =
            project_rustdoc(&dependency, &serde_json::to_vec(&document).unwrap()).unwrap();
        let trait_method = projected
            .items
            .iter()
            .find(|item| item.namespace == "/deps/witness/readable" && item.name == "remaining")
            .expect("receiver-first trait method");
        let ProjectedKind::Function(function) = &trait_method.kind else {
            panic!("trait method must project as a function");
        };
        assert_eq!(function.result, ProjectedType::RustInt("usize".to_owned()));
        assert_eq!(function.parameters.len(), 1);
        assert!(function.parameters[0].borrowed);
        assert_eq!(
            function.parameters[0].ty,
            ProjectedType::Foreign {
                rust_path: "witness::Reader".to_owned(),
                name: "Reader".to_owned(),
            }
        );
        assert!(
            projected
                .items
                .iter()
                .any(|item| { item.namespace == "/deps/witness/mood" && item.name == "Calm" })
        );
        assert!(
            projected
                .items
                .iter()
                .any(|item| { item.namespace == "/deps/witness/mood" && item.name == "Busy" })
        );
        assert!(matches!(
            projected
                .items
                .iter()
                .find(|item| item.name == "Mood")
                .map(|item| &item.kind),
            Some(ProjectedKind::Enum {
                data_carrying: false,
                comparable: true,
            })
        ));
        let variant = projected
            .items
            .iter()
            .find(|item| item.name == "Calm")
            .unwrap()
            .clone();
        let projection = Projection {
            cache_identity: "fixture".to_owned(),
            dependencies: vec![projected],
            containment: Containment::Unavailable,
            removed: Vec::new(),
        };
        assert!(projection.is_unit_variant(&variant));
    }

    #[test]
    fn colliding_receiver_free_associated_functions_are_declined() {
        let mut document = trait_and_enum_rustdoc();
        document["index"]["0"]["inner"]["module"]["items"]
            .as_array_mut()
            .unwrap()
            .push(json!(12));
        document["index"]["1"]["inner"]["struct"]["impls"]
            .as_array_mut()
            .unwrap()
            .push(json!(13));
        document["paths"]["12"] = json!({"crate_id": 0, "path": ["witness", "Writer"]});
        document["index"]["12"] = json!({
            "name": "Writer",
            "visibility": "public",
            "docs": null,
            "inner": {"struct": {"generics": {"params": []}, "impls": [15]}}
        });
        document["index"]["13"] = json!({
            "inner": {"impl": {
                "is_negative": false,
                "trait": null,
                "items": [14]
            }}
        });
        document["index"]["14"] = json!({
            "name": "new",
            "visibility": "public",
            "inner": {"function": {
                "header": {"is_unsafe": false, "is_async": false},
                "generics": {"params": []},
                "sig": {"inputs": [], "output": null}
            }}
        });
        document["index"]["15"] = json!({
            "inner": {"impl": {
                "is_negative": false,
                "trait": null,
                "items": [16]
            }}
        });
        document["index"]["16"] = document["index"]["14"].clone();
        let dependency = RustDependency {
            name: "witness".to_owned(),
            package: "witness".to_owned(),
            version: "=1.0.0".to_owned(),
            features: Vec::new(),
            default_features: true,
            target: None,
            effects: Vec::new(),
        };

        let projected =
            project_rustdoc(&dependency, &serde_json::to_vec(&document).unwrap()).unwrap();

        assert!(
            !projected
                .items
                .iter()
                .any(|item| item.namespace == "/deps/witness" && item.name == "new")
        );
        let declines = projected
            .declined
            .iter()
            .filter(|item| {
                item.reason
                    == "multiple receiver-free associated functions with the same projected name"
            })
            .map(|item| item.rust_path.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            declines,
            BTreeSet::from(["witness::Reader::new", "witness::Writer::new"])
        );
    }

    #[test]
    fn projection_history_retains_removed_members_across_checks() {
        let directory =
            std::env::temp_dir().join(format!("terrane-projection-history-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let dependency = |version: &str, items: Vec<ProjectedItem>| ProjectedDependency {
            name: "fixture".to_owned(),
            package: "fixture".to_owned(),
            version: version.to_owned(),
            items,
            declined: Vec::new(),
        };
        let item = ProjectedItem {
            namespace: "/deps/fixture".to_owned(),
            name: "removed".to_owned(),
            rust_path: "fixture::removed".to_owned(),
            docs: None,
            kind: ProjectedKind::Function(ProjectedFunction {
                name: "removed".to_owned(),
                parameters: Vec::new(),
                result: ProjectedType::None,
                error: None,
                is_async: false,
                receiver: None,
            }),
        };
        let mut old = Projection {
            cache_identity: "old".to_owned(),
            dependencies: vec![dependency("1.0.0", vec![item])],
            containment: Containment::Unavailable,
            removed: Vec::new(),
        };
        apply_projection_history(&directory, &mut old).unwrap();
        let mut current = Projection {
            cache_identity: "current".to_owned(),
            dependencies: vec![dependency("2.0.0", Vec::new())],
            containment: Containment::Unavailable,
            removed: Vec::new(),
        };
        apply_projection_history(&directory, &mut current).unwrap();
        assert_eq!(current.removed.len(), 1);
        current.removed.clear();
        apply_projection_history(&directory, &mut current).unwrap();
        assert_eq!(current.removed.len(), 1);
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn projection_cache_retains_a_bounded_history_and_unrelated_files() {
        let directory =
            std::env::temp_dir().join(format!("terrane-projection-prune-{}", std::process::id()));
        let retained = directory.join("projection-current.json");
        let unrelated = directory.join("Cargo.lock");
        fs::create_dir_all(&directory).unwrap();
        fs::write(&retained, b"current").unwrap();
        for index in 0..6 {
            fs::write(
                directory.join(format!("projection-previous-{index}.json")),
                b"previous",
            )
            .unwrap();
        }
        fs::write(&unrelated, b"lock").unwrap();

        prune_projection_cache(&directory, &retained).unwrap();

        let projection_count = fs::read_dir(&directory)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("projection-"))
            })
            .count();
        assert_eq!(projection_count, super::MAX_PROJECTION_CACHE_RECORDS);
        assert!(retained.exists());
        assert!(unrelated.exists());
        fs::remove_dir_all(directory).unwrap();
    }
}
