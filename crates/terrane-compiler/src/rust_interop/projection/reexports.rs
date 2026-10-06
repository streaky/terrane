//! Owns supplemental rustdoc discovery and dependency reexport projection.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use rustdoc_types::{Crate as RustdocCrate, Id, ItemKind};
use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::{
    CargoExecution, CargoToolchain, Containment, DeclinedItem, ProjectedDependency, ProjectedItem,
    ProjectedKind, ProjectionError, RUSTDOC_TOOLCHAIN, RustDependency, canonicalize_rust_path,
    collect_foreign_function, dependency_namespace, run_cargo, rustdoc_public_paths,
    source_rendering, write_cache_atomically,
};
pub(super) struct ReexportRustdoc {
    pub(super) document: RustdocCrate,
    pub(super) providers: Vec<ReexportProvider>,
}

pub(super) struct ReexportProvider {
    pub(super) dependency_index: usize,
    pub(super) public_paths: BTreeMap<Id, String>,
    pub(super) canonical_public_paths: BTreeMap<String, String>,
    pub(super) rust_path_aliases: BTreeMap<String, String>,
}

pub(super) fn provider_fragment_public_paths(
    declared_public_paths: &BTreeMap<String, String>,
    provider: &ReexportProvider,
) -> BTreeMap<String, String> {
    let mut public_paths = declared_public_paths.clone();
    // Provider aliases override declared-owner paths only inside this facade fragment. Unmapped
    // owner paths remain owner-rooted and are declined by ordinary transitive reachability.
    public_paths.extend(provider.canonical_public_paths.clone());
    public_paths
}

struct ReexportRequest {
    package_spec: String,
    package_name: String,
    aliases: BTreeMap<usize, BTreeMap<String, String>>,
    named_aliases: BTreeMap<usize, BTreeSet<(String, String)>>,
    prefixes: BTreeMap<usize, Vec<(String, String)>>,
    signature_paths: BTreeMap<usize, BTreeSet<String>>,
}

pub(super) fn generate_rustdoc(
    workspace: &Path,
    package_spec: &str,
    crate_name: &str,
    package_name: &str,
    containment: Containment,
    retain_hidden_definitions: bool,
) -> Result<RustdocCrate, ProjectionError> {
    let mut rustdoc_args = vec![
        "rustdoc",
        "-p",
        package_spec,
        "--lib",
        "--target-dir",
        "target/rustdoc-57",
        "--offline",
        "--frozen",
        "--",
    ];
    rustdoc_args.extend_from_slice(terrane_rust_analysis::RUSTDOC_JSON_ARGS);
    if retain_hidden_definitions {
        rustdoc_args.push("--document-hidden-items");
    }
    run_cargo(
        workspace,
        &rustdoc_args,
        CargoToolchain::RustdocNightly,
        if containment == Containment::Enforced {
            CargoExecution::Contained
        } else {
            CargoExecution::Host
        },
    )?;
    let rustdoc_path = workspace
        .join("target/rustdoc-57/doc")
        .join(format!("{crate_name}.json"));
    let bytes = fs::read(&rustdoc_path).map_err(|error| ProjectionError {
        message: format!(
            "cannot read rustdoc projection `{}`: {error}",
            rustdoc_path.display()
        ),
    })?;
    terrane_rust_analysis::parse_rustdoc(package_name, &bytes, RUSTDOC_TOOLCHAIN).map_err(|error| {
        ProjectionError {
            message: error.message,
        }
    })
}

pub(super) fn cached_owner_rustdoc(
    workspace: &Path,
    package_spec: &str,
    crate_name: &str,
    package_name: &str,
    target: &str,
    containment: Containment,
) -> Result<RustdocCrate, ProjectionError> {
    #[derive(Deserialize)]
    struct CacheHeader<'a> {
        #[serde(borrow)]
        terrane_cache_identity: &'a str,
    }
    let rustdoc_format = rustdoc_types::FORMAT_VERSION.to_string();
    let containment_fingerprint = format!("{containment:?}");
    let mut fingerprint_hash = Sha256::new();
    for (label, value) in [
        ("package-spec", package_spec),
        ("crate-name", crate_name),
        ("package-name", package_name),
        ("target", target),
        ("hidden-items", "true"),
        ("rustdoc-toolchain", RUSTDOC_TOOLCHAIN),
        ("rustdoc-format", rustdoc_format.as_str()),
        ("containment", containment_fingerprint.as_str()),
    ] {
        fingerprint_hash.update(label.len().to_le_bytes());
        fingerprint_hash.update(label.as_bytes());
        fingerprint_hash.update(value.len().to_le_bytes());
        fingerprint_hash.update(value.as_bytes());
    }
    let fingerprint = format!("{:x}", fingerprint_hash.finalize());
    let cache_path = workspace
        .join("owner-rustdoc")
        .join(format!("{crate_name}.json"));
    if let Ok(bytes) = fs::read(&cache_path)
        && let Ok(header) = serde_json::from_slice::<CacheHeader<'_>>(&bytes)
        && header.terrane_cache_identity == fingerprint
        && let Ok(document) =
            terrane_rust_analysis::parse_rustdoc(package_name, &bytes, RUSTDOC_TOOLCHAIN)
    {
        return Ok(document);
    }
    let document = generate_rustdoc(
        workspace,
        package_spec,
        crate_name,
        package_name,
        containment,
        true,
    )?;
    let generated_path = workspace
        .join("target/rustdoc-57/doc")
        .join(format!("{crate_name}.json"));
    let bytes = fs::read(&generated_path).map_err(|error| ProjectionError {
        message: format!(
            "cannot cache owner rustdoc `{}`: {error}",
            generated_path.display()
        ),
    })?;
    let mut prefix =
        Vec::with_capacity(fingerprint.len() + package_spec.len() + crate_name.len() + 96);
    prefix.push(b'{');
    for (key, value) in [
        ("terrane_cache_identity", fingerprint.as_str()),
        ("terrane_cache_package", package_spec),
        ("terrane_cache_crate", crate_name),
    ] {
        serde_json::to_writer(&mut prefix, key).map_err(|error| ProjectionError {
            message: format!("cannot serialize owner rustdoc cache metadata: {error}"),
        })?;
        prefix.push(b':');
        serde_json::to_writer(&mut prefix, value).map_err(|error| ProjectionError {
            message: format!("cannot serialize owner rustdoc cache metadata: {error}"),
        })?;
        prefix.push(b',');
    }
    let Some(body) = bytes.trim_ascii_start().strip_prefix(b"{") else {
        return Err(ProjectionError {
            message: "generated owner rustdoc is not a JSON object".to_owned(),
        });
    };
    write_cache_atomically(&cache_path, &[&prefix, body])?;
    Ok(document)
}
pub(super) fn prefer_alias(current: &mut String, candidate: &str) {
    let mut paths = BTreeMap::from([(Id(0), current.clone())]);
    terrane_rust_analysis::prefer_public_path(&mut paths, Id(0), candidate.to_owned());
    current.clone_from(
        paths
            .get(&Id(0))
            .expect("the seeded public path remains present"),
    );
}
fn record_canonical_alias(
    paths: &mut BTreeMap<String, String>,
    canonical_path: &str,
    owner_path: &str,
    public_path: &str,
) {
    for source in [canonical_path, owner_path] {
        paths
            .entry(source.to_owned())
            .and_modify(|current| prefer_alias(current, public_path))
            .or_insert_with(|| public_path.to_owned());
    }
}

fn reexport_is_demanded(
    dependency: &RustDependency,
    public_path: &str,
    expands_descendants: bool,
    demands: Option<&BTreeSet<(String, String)>>,
) -> bool {
    let Some(demands) = demands else {
        return true;
    };
    let path = public_path
        .split("::")
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let Some(name) = path.last() else {
        return false;
    };
    if expands_descendants {
        let namespace = dependency_namespace(dependency, &path);
        return demands.iter().any(|(target, _)| {
            target == &namespace || target.starts_with(&format!("{namespace}/"))
        });
    }
    let namespace = dependency_namespace(dependency, &path[..path.len().saturating_sub(1)]);
    demands.contains(&(namespace.clone(), name.clone()))
        || demands.contains(&(format!("{namespace}/macros"), name.clone()))
}

fn signature_foreign_aliases(
    dependency: &RustDependency,
    items: &[ProjectedItem],
) -> BTreeMap<String, String> {
    let source_prefix = format!("/deps/{}", dependency.name);
    let crate_name = dependency.package.replace('-', "_");
    let mut aliases = BTreeMap::new();
    for item in items {
        let mut foreign = BTreeMap::new();
        match &item.kind {
            ProjectedKind::Function(function) | ProjectedKind::Macro(function) => {
                collect_foreign_function(function, &mut foreign);
            }
            ProjectedKind::ForeignType {
                fields,
                methods,
                static_methods,
                constructor,
                generic_parameters,
                ..
            } => {
                for field in fields {
                    source_rendering::collect_foreign_type(&field.ty, &mut foreign);
                }
                for function in methods.iter().chain(static_methods).chain(constructor) {
                    collect_foreign_function(function, &mut foreign);
                }
                for parameter in generic_parameters {
                    if let Some(default) = &parameter.default {
                        source_rendering::collect_foreign_type(default, &mut foreign);
                    }
                }
            }
            ProjectedKind::Enum {
                variants,
                methods,
                static_methods,
                ..
            } => {
                for field in variants.iter().flat_map(|variant| &variant.fields) {
                    source_rendering::collect_foreign_type(&field.ty, &mut foreign);
                }
                for function in methods.iter().chain(static_methods) {
                    collect_foreign_function(function, &mut foreign);
                }
            }
            ProjectedKind::Interface(interface) => {
                for method in &interface.methods {
                    collect_foreign_function(&method.function, &mut foreign);
                }
            }
        }
        let Some(namespace) = item.namespace.strip_prefix(&source_prefix) else {
            continue;
        };
        for (rust_path, name) in foreign {
            let Some(constructor) = crate::rust_ir::rust_type_constructor(&rust_path) else {
                continue;
            };
            let public_path = format!("{crate_name}{}::{name}", namespace.replace('/', "::"));
            aliases
                .entry(constructor)
                .and_modify(|current| prefer_alias(current, &public_path))
                .or_insert(public_path);
        }
    }
    aliases
}

#[expect(
    clippy::too_many_lines,
    reason = "one pass groups demanded facade aliases and prefixes, then materializes each owner"
)]
#[expect(
    clippy::too_many_arguments,
    reason = "owner discovery uses the exact resolved graph, target, containment, demand, and decline sink"
)]
pub(super) fn external_reexport_rustdocs(
    workspace: &Path,
    rustdocs: &[(&RustDependency, RustdocCrate, BTreeMap<Id, String>)],
    projected: &[ProjectedDependency],
    metadata: &serde_json::Value,
    target: &str,
    containment: Containment,
    demands: Option<&BTreeSet<(String, String)>>,
    declines: &mut [Vec<DeclinedItem>],
) -> Result<Vec<ReexportRustdoc>, ProjectionError> {
    // Aggregate every demanded facade binding by concrete owner so one projection run emits
    // and parses at most one supplemental rustdoc document per resolved owner package.
    let mut requests = BTreeMap::<String, ReexportRequest>::new();
    for (dependency_index, (dependency, document, public_paths)) in rustdocs.iter().enumerate() {
        for (id, public_path) in public_paths {
            let Some(summary) = document.paths.get(id) else {
                continue;
            };
            let owner_crate_name = if summary.crate_id == 0 {
                if document.index.contains_key(id) {
                    continue;
                }
                document
                    .paths
                    .get(&document.root)
                    .and_then(|root| root.path.first())
                    .cloned()
                    .unwrap_or_else(|| dependency.package.replace('-', "_"))
            } else {
                let Some(external) = document.external_crates.get(&summary.crate_id) else {
                    continue;
                };
                if matches!(external.name.as_str(), "std" | "alloc") {
                    continue;
                }
                external.name.clone()
            };
            let demanded = reexport_is_demanded(
                dependency,
                public_path,
                summary.kind == ItemKind::Module,
                demands,
            );
            if !demanded {
                continue;
            }
            if owner_crate_name == "core"
                && !matches!(summary.kind, ItemKind::Struct | ItemKind::Enum)
            {
                declines[dependency_index].push(DeclinedItem {
                    rust_path: public_path.clone(),
                    reason: format!(
                        "demanded core item kind {:?} has no Rust-to-Terrane projection",
                        summary.kind
                    ),
                });
                continue;
            }
            if owner_crate_name == "core" {
                let request = requests
                    .entry(owner_crate_name)
                    .or_insert_with(|| ReexportRequest {
                        package_spec: String::new(),
                        package_name: "core".to_owned(),
                        aliases: BTreeMap::new(),
                        named_aliases: BTreeMap::new(),
                        prefixes: BTreeMap::new(),
                        signature_paths: BTreeMap::new(),
                    });
                let aliases = request.aliases.entry(dependency_index).or_default();
                aliases
                    .entry(summary.path.join("::"))
                    .and_modify(|current| prefer_alias(current, public_path))
                    .or_insert_with(|| public_path.clone());
                continue;
            }
            let (package_name, version) =
                match resolved_library_package(metadata, &owner_crate_name) {
                    Ok(package) => package,
                    Err(reason) => {
                        declines[dependency_index].push(DeclinedItem {
                            rust_path: public_path.clone(),
                            reason,
                        });
                        continue;
                    }
                };
            let request = requests
                .entry(owner_crate_name)
                .or_insert_with(|| ReexportRequest {
                    package_spec: format!("{package_name}@{version}"),
                    package_name,
                    aliases: BTreeMap::new(),
                    named_aliases: BTreeMap::new(),
                    prefixes: BTreeMap::new(),
                    signature_paths: BTreeMap::new(),
                });
            let aliases = request.aliases.entry(dependency_index).or_default();
            aliases
                .entry(summary.path.join("::"))
                .and_modify(|current| prefer_alias(current, public_path))
                .or_insert_with(|| public_path.clone());
        }
        for reexport in terrane_rust_analysis::external_reexports(document) {
            if matches!(reexport.crate_name.as_str(), "std" | "alloc")
                || reexport.crate_name == "core" && reexport.expands_descendants
                || !reexport_is_demanded(
                    dependency,
                    &reexport.public_path,
                    reexport.expands_descendants,
                    demands,
                )
            {
                continue;
            }
            let (package_spec, package_name) = if reexport.crate_name == "core" {
                (String::new(), "core".to_owned())
            } else {
                match resolved_library_package(metadata, &reexport.crate_name) {
                    Ok((name, version)) => (format!("{name}@{version}"), name),
                    Err(reason) => {
                        declines[dependency_index].push(DeclinedItem {
                            rust_path: reexport.public_path,
                            reason,
                        });
                        continue;
                    }
                }
            };
            let request = requests
                .entry(reexport.crate_name)
                .or_insert_with(|| ReexportRequest {
                    package_spec,
                    package_name,
                    aliases: BTreeMap::new(),
                    named_aliases: BTreeMap::new(),
                    prefixes: BTreeMap::new(),
                    signature_paths: BTreeMap::new(),
                });
            if reexport.expands_descendants {
                request
                    .prefixes
                    .entry(dependency_index)
                    .or_default()
                    .push((reexport.canonical_path, reexport.public_path));
            } else {
                request
                    .aliases
                    .entry(dependency_index)
                    .or_default()
                    .entry(reexport.canonical_path.clone())
                    .and_modify(|current| prefer_alias(current, &reexport.public_path))
                    .or_insert_with(|| reexport.public_path.clone());
                request
                    .named_aliases
                    .entry(dependency_index)
                    .or_default()
                    .insert((reexport.canonical_path, reexport.public_path));
            }
        }
    }

    for (dependency_index, ((dependency, document, _), projection)) in
        rustdocs.iter().zip(projected).enumerate()
    {
        let summaries = document
            .paths
            .values()
            .filter(|summary| {
                summary.crate_id != 0 && matches!(summary.kind, ItemKind::Struct | ItemKind::Enum)
            })
            .map(|summary| (canonicalize_rust_path(&summary.path.join("::")), summary))
            .collect::<BTreeMap<_, _>>();
        for (rust_path, public_path) in signature_foreign_aliases(dependency, &projection.items) {
            let Some(summary) = summaries.get(&rust_path) else {
                continue;
            };
            let Some(external) = document.external_crates.get(&summary.crate_id) else {
                continue;
            };
            if matches!(external.name.as_str(), "std" | "alloc") {
                continue;
            }
            let (package_spec, package_name) = if external.name == "core" {
                (String::new(), "core".to_owned())
            } else {
                let (name, version) = match resolved_library_package(metadata, &external.name) {
                    Ok(package) => package,
                    Err(reason) => {
                        declines[dependency_index].push(DeclinedItem {
                            rust_path: public_path,
                            reason,
                        });
                        continue;
                    }
                };
                (format!("{name}@{version}"), name)
            };
            let request =
                requests
                    .entry(external.name.clone())
                    .or_insert_with(|| ReexportRequest {
                        package_spec,
                        package_name,
                        aliases: BTreeMap::new(),
                        named_aliases: BTreeMap::new(),
                        prefixes: BTreeMap::new(),
                        signature_paths: BTreeMap::new(),
                    });
            // Real demanded Rust reexports outrank names inferred from signatures.
            // The latter may not even be exported by the facade.
            let public_path = request
                .aliases
                .entry(dependency_index)
                .or_default()
                .entry(summary.path.join("::"))
                .or_insert(public_path);
            request
                .signature_paths
                .entry(dependency_index)
                .or_default()
                .insert(public_path.clone());
        }
    }

    requests
        .into_iter()
        .map(|(crate_name, request)| {
            let document = if crate_name == "core" {
                terrane_rust_analysis::core_rustdoc(&workspace.join("rust-survey/sysroot"), target)
                    .map_err(|error| ProjectionError {
                        message: error.message,
                    })?
            } else {
                cached_owner_rustdoc(
                    workspace,
                    &request.package_spec,
                    &crate_name,
                    &request.package_name,
                    target,
                    containment,
                )?
            };
            let owner_public_paths = rustdoc_public_paths(&document);
            let mut provider_indices = request
                .aliases
                .keys()
                .chain(request.prefixes.keys())
                .copied()
                .collect::<BTreeSet<_>>();
            for (dependency_index, (_, direct_document, _)) in rustdocs.iter().enumerate() {
                if direct_document
                    .paths
                    .get(&direct_document.root)
                    .and_then(|root| root.path.first())
                    == Some(&crate_name)
                {
                    provider_indices.insert(dependency_index);
                }
            }
            let mut providers: Vec<ReexportProvider> = provider_indices
                .into_iter()
                .filter_map(|dependency_index| {
                    let dependency = rustdocs[dependency_index].0;
                    let direct_owner = rustdocs[dependency_index]
                        .1
                        .paths
                        .get(&rustdocs[dependency_index].1.root)
                        .and_then(|root| root.path.first())
                        == Some(&crate_name);
                    let aliases = request.aliases.get(&dependency_index);
                    let prefixes = request.prefixes.get(&dependency_index);
                    let mut public_paths = BTreeMap::new();
                    let mut canonical_public_paths = BTreeMap::new();
                    let mut rust_path_aliases = BTreeMap::new();
                    for (id, summary) in document
                        .paths
                        .iter()
                        .filter(|(_, summary)| summary.crate_id == 0)
                    {
                        let canonical_path = summary.path.join("::");
                        let owner_path = owner_public_paths.get(id).unwrap_or(&canonical_path);
                        if direct_owner && let Some(public_path) = owner_public_paths.get(id) {
                            record_canonical_alias(
                                &mut canonical_public_paths,
                                &canonical_path,
                                owner_path,
                                public_path,
                            );
                            if reexport_is_demanded(dependency, public_path, false, demands) {
                                terrane_rust_analysis::prefer_public_path(
                                    &mut public_paths,
                                    *id,
                                    public_path.clone(),
                                );
                            }
                        }
                        if let Some(public_path) = aliases.and_then(|aliases| {
                            aliases
                                .get(&canonical_path)
                                .or_else(|| aliases.get(owner_path))
                        }) {
                            record_canonical_alias(
                                &mut canonical_public_paths,
                                &canonical_path,
                                owner_path,
                                public_path,
                            );
                            if crate_name == "core"
                                || request
                                    .signature_paths
                                    .get(&dependency_index)
                                    .is_some_and(|paths| paths.contains(public_path))
                            {
                                let native_path = canonicalize_rust_path(owner_path);
                                rust_path_aliases.insert(public_path.clone(), native_path.clone());
                                rust_path_aliases.insert(canonical_path.clone(), native_path);
                            }
                            if reexport_is_demanded(dependency, public_path, false, demands)
                                || request
                                    .signature_paths
                                    .get(&dependency_index)
                                    .is_some_and(|paths| paths.contains(public_path))
                            {
                                terrane_rust_analysis::prefer_public_path(
                                    &mut public_paths,
                                    *id,
                                    public_path.clone(),
                                );
                            }
                        }
                        for (canonical_prefix, public_prefix) in prefixes.into_iter().flatten() {
                            let suffix = if owner_path == canonical_prefix {
                                Some("")
                            } else {
                                owner_path.strip_prefix(&format!("{canonical_prefix}::"))
                            };
                            if let Some(suffix) = suffix {
                                let public_path = if suffix.is_empty() {
                                    public_prefix.clone()
                                } else {
                                    format!("{public_prefix}::{suffix}")
                                };
                                record_canonical_alias(
                                    &mut canonical_public_paths,
                                    &canonical_path,
                                    owner_path,
                                    &public_path,
                                );
                                if reexport_is_demanded(dependency, &public_path, false, demands) {
                                    terrane_rust_analysis::prefer_public_path(
                                        &mut public_paths,
                                        *id,
                                        public_path,
                                    );
                                }
                            }
                        }
                    }
                    if public_paths.is_empty() {
                        return None;
                    }
                    Some(ReexportProvider {
                        dependency_index,
                        public_paths,
                        canonical_public_paths,
                        rust_path_aliases,
                    })
                })
                .collect();
            // One preferred path per Rustdoc ID supplies the normal fragment.
            // Additional demanded public bindings need their own source views of
            // that same canonical declaration, with the same generic/member graph.
            let owner_ids = document
                .paths
                .iter()
                .filter(|(_, summary)| summary.crate_id == 0)
                .map(|(id, summary)| (summary.path.join("::"), *id))
                .chain(
                    owner_public_paths
                        .iter()
                        .map(|(id, path)| (path.clone(), *id)),
                )
                .collect::<BTreeMap<_, _>>();
            for (dependency_index, bindings) in &request.named_aliases {
                for (canonical_path, public_path) in bindings {
                    let Some(id) = owner_ids.get(canonical_path) else {
                        continue;
                    };
                    if providers.iter().any(|provider| {
                        provider.dependency_index == *dependency_index
                            && provider.public_paths.get(id) == Some(public_path)
                    }) {
                        continue;
                    }
                    let Some(primary) = providers
                        .iter()
                        .find(|provider| provider.dependency_index == *dependency_index)
                    else {
                        continue;
                    };
                    let owner_path = owner_public_paths.get(id).unwrap_or(canonical_path);
                    let mut canonical_public_paths = primary.canonical_public_paths.clone();
                    canonical_public_paths.insert(canonical_path.clone(), public_path.clone());
                    canonical_public_paths.insert(owner_path.clone(), public_path.clone());
                    let mut rust_path_aliases = primary.rust_path_aliases.clone();
                    rust_path_aliases
                        .insert(public_path.clone(), canonicalize_rust_path(owner_path));
                    providers.push(ReexportProvider {
                        dependency_index: *dependency_index,
                        public_paths: BTreeMap::from([(*id, public_path.clone())]),
                        canonical_public_paths,
                        rust_path_aliases,
                    });
                }
            }
            Ok(ReexportRustdoc {
                document,
                providers,
            })
        })
        .collect()
}
pub(super) fn resolved_library_package(
    metadata: &serde_json::Value,
    crate_name: &str,
) -> Result<(String, String), String> {
    let packages = metadata
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "resolved Cargo metadata has no package list".to_owned())?;
    let matches = packages
        .iter()
        .filter(|package| {
            package
                .get("targets")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|targets| {
                    targets.iter().any(|target| {
                        target.get("name").and_then(serde_json::Value::as_str) == Some(crate_name)
                            && target
                                .get("kind")
                                .and_then(serde_json::Value::as_array)
                                .is_some_and(|kinds| {
                                    kinds.iter().any(|kind| {
                                        kind.as_str().is_some_and(|kind| {
                                            matches!(
                                                kind,
                                                "lib"
                                                    | "rlib"
                                                    | "dylib"
                                                    | "cdylib"
                                                    | "staticlib"
                                                    | "proc-macro"
                                            )
                                        })
                                    })
                                })
                    })
                })
        })
        .filter_map(|package| {
            Some((
                package.get("name")?.as_str()?.to_owned(),
                package.get("version")?.as_str()?.to_owned(),
            ))
        })
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [(package, version)] => Ok((package.clone(), version.clone())),
        [] => Err(format!(
            "external reexport owner `{crate_name}` has no resolved library package"
        )),
        _ => Err(format!(
            "external reexport owner `{crate_name}` is ambiguous across resolved packages: {}",
            matches
                .iter()
                .map(|(package, version)| format!("{package}@{version}"))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}
