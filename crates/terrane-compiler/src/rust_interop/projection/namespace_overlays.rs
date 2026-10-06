//! Owns namespace overlay metadata parsing and projected namespace relocation.
use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;

use super::{ProjectedDependency, ProjectionError, RustDependency};
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
struct NamespaceOverlayMetadata {
    module: String,
    target_package: String,
    feature: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct NamespaceOverlay {
    pub(super) provider_name: String,
    pub(super) provider_package: String,
    pub(super) source_namespace: String,
    pub(super) target_namespace: String,
}
pub(super) fn namespace_overlays_from_metadata(
    metadata: &serde_json::Value,
    dependencies: &[RustDependency],
) -> Result<Vec<NamespaceOverlay>, ProjectionError> {
    let packages = metadata
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| ProjectionError {
            message: "Cargo dependency metadata has no package list".to_owned(),
        })?;
    let root = metadata
        .pointer("/resolve/root")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| ProjectionError {
            message: "Cargo dependency metadata has no resolved root package".to_owned(),
        })?;
    let nodes = metadata
        .pointer("/resolve/nodes")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| ProjectionError {
            message: "Cargo dependency metadata has no resolved dependency graph".to_owned(),
        })?;
    let direct = nodes
        .iter()
        .find(|node| node.get("id").and_then(serde_json::Value::as_str) == Some(root))
        .and_then(|node| node.get("deps"))
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| ProjectionError {
            message: "Cargo dependency metadata has no direct dependency graph".to_owned(),
        })?;
    let mut overlays = Vec::new();
    for provider in dependencies {
        let (package, node) = direct_metadata_package(packages, nodes, direct, provider)?;
        let enabled_features = node
            .get("features")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(serde_json::Value::as_str)
            .collect::<BTreeSet<_>>();
        overlays.extend(package_namespace_overlays(
            package,
            provider,
            dependencies,
            &enabled_features,
        )?);
    }
    overlays.sort_by(|left, right| {
        (&left.provider_name, &left.source_namespace)
            .cmp(&(&right.provider_name, &right.source_namespace))
    });
    for pair in overlays.windows(2) {
        if pair[0].provider_name == pair[1].provider_name
            && (pair[0].source_namespace == pair[1].source_namespace
                || pair[1]
                    .source_namespace
                    .starts_with(&format!("{}/", pair[0].source_namespace)))
        {
            return Err(ProjectionError {
                message: format!(
                    "Rust dependency `{}` has overlapping namespace overlays `{}` and `{}`",
                    pair[0].provider_name, pair[0].source_namespace, pair[1].source_namespace
                ),
            });
        }
    }
    Ok(overlays)
}

fn direct_metadata_package<'a>(
    packages: &'a [serde_json::Value],
    nodes: &'a [serde_json::Value],
    direct: &[serde_json::Value],
    dependency: &RustDependency,
) -> Result<(&'a serde_json::Value, &'a serde_json::Value), ProjectionError> {
    let cargo_name = dependency.name.replace('-', "_");
    let by_alias = direct.iter().find(|candidate| {
        candidate
            .get("name")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|name| name.replace('-', "_") == cargo_name)
    });
    let by_package = direct
        .iter()
        .filter(|candidate| {
            let package_id = candidate.get("pkg").and_then(serde_json::Value::as_str);
            packages.iter().any(|package| {
                package.get("id").and_then(serde_json::Value::as_str) == package_id
                    && package.get("name").and_then(serde_json::Value::as_str)
                        == Some(dependency.package.as_str())
            })
        })
        .collect::<Vec<_>>();
    let resolved = by_alias.or_else(|| {
        let [candidate] = by_package.as_slice() else {
            return None;
        };
        Some(*candidate)
    });
    let package_id = resolved
        .and_then(|candidate| candidate.get("pkg"))
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| ProjectionError {
            message: format!(
                "Cargo dependency metadata has no unambiguous resolved package for direct dependency `{}`",
                dependency.name
            ),
        })?;
    let package = packages
        .iter()
        .find(|package| package.get("id").and_then(serde_json::Value::as_str) == Some(package_id))
        .ok_or_else(|| ProjectionError {
            message: format!(
                "Cargo dependency metadata is missing package `{package_id}` for direct dependency `{}`",
                dependency.name
            ),
        })?;
    if package.get("name").and_then(serde_json::Value::as_str) != Some(dependency.package.as_str())
    {
        return Err(ProjectionError {
            message: format!(
                "Cargo dependency metadata resolved `{}` to unexpected package `{}`",
                dependency.name,
                package
                    .get("name")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or("<unknown>")
            ),
        });
    }
    let node = nodes
        .iter()
        .find(|node| node.get("id").and_then(serde_json::Value::as_str) == Some(package_id))
        .ok_or_else(|| ProjectionError {
            message: format!(
                "Cargo dependency metadata has no resolved feature set for direct dependency `{}`",
                dependency.name
            ),
        })?;
    Ok((package, node))
}

fn package_namespace_overlays(
    package: &serde_json::Value,
    provider: &RustDependency,
    dependencies: &[RustDependency],
    enabled_features: &BTreeSet<&str>,
) -> Result<Vec<NamespaceOverlay>, ProjectionError> {
    let Some(declarations) = package
        .pointer("/metadata/terrane/namespace-overlays")
        .filter(|value| !value.is_null())
    else {
        return Ok(Vec::new());
    };
    let declarations =
        serde_json::from_value::<Vec<NamespaceOverlayMetadata>>(declarations.clone()).map_err(
            |error| ProjectionError {
                message: format!(
                    "Rust dependency `{}` has invalid `package.metadata.terrane.namespace-overlays`: {error}",
                    provider.name
                ),
            },
        )?;
    let mut overlays = Vec::new();
    for declaration in declarations {
        if !enabled_features.contains(declaration.feature.as_str()) {
            continue;
        }
        let source_module =
            overlay_module_namespace(&declaration.module).ok_or_else(|| ProjectionError {
                message: format!(
                    "Rust dependency `{}` namespace overlay module `{}` is not a Rust module path",
                    provider.name, declaration.module
                ),
            })?;
        let targets = dependencies
            .iter()
            .filter(|dependency| dependency.package == declaration.target_package)
            .collect::<Vec<_>>();
        let [target] = targets.as_slice() else {
            return Err(ProjectionError {
                message: if targets.is_empty() {
                    format!(
                        "Rust dependency `{}` namespace overlay targets undeclared package `{}`",
                        provider.name, declaration.target_package
                    )
                } else {
                    format!(
                        "Rust dependency `{}` namespace overlay target package `{}` has multiple direct aliases",
                        provider.name, declaration.target_package
                    )
                },
            });
        };
        if provider.name == target.name {
            return Err(ProjectionError {
                message: format!(
                    "Rust dependency `{}` namespace overlay cannot target itself",
                    provider.name
                ),
            });
        }
        overlays.push(NamespaceOverlay {
            provider_name: provider.name.clone(),
            provider_package: provider.package.clone(),
            source_namespace: format!(
                "/deps/{}/{}",
                provider.name.replace('_', "-"),
                source_module
            ),
            target_namespace: format!("/deps/{}", target.name.replace('_', "-")),
        });
    }
    Ok(overlays)
}

fn overlay_module_namespace(module: &str) -> Option<String> {
    let segments = module.split("::").collect::<Vec<_>>();
    if segments.is_empty()
        || segments.iter().any(|segment| {
            segment.is_empty()
                || !segment.bytes().enumerate().all(|(index, byte)| {
                    byte == b'_'
                        || byte.is_ascii_alphanumeric() && (index > 0 || !byte.is_ascii_digit())
                })
        })
    {
        return None;
    }
    Some(
        segments
            .into_iter()
            .map(|segment| segment.replace('_', "-"))
            .collect::<Vec<_>>()
            .join("/"),
    )
}

pub(super) fn apply_namespace_overlays(
    projected: &mut [ProjectedDependency],
    overlays: &[NamespaceOverlay],
) -> Result<(), ProjectionError> {
    let mut matched = vec![0_usize; overlays.len()];
    let mut moved = BTreeSet::new();
    for (dependency_index, dependency) in projected.iter_mut().enumerate() {
        for (item_index, item) in dependency.items.iter_mut().enumerate() {
            for (index, overlay) in overlays.iter().enumerate().filter(|(_, overlay)| {
                dependency.name == overlay.provider_name
                    && dependency.package == overlay.provider_package
            }) {
                let suffix = item
                    .namespace
                    .strip_prefix(&overlay.source_namespace)
                    .filter(|suffix| suffix.is_empty() || suffix.starts_with('/'));
                if let Some(suffix) = suffix {
                    item.namespace = format!("{}{suffix}", overlay.target_namespace);
                    matched[index] += 1;
                    moved.insert((dependency_index, item_index));
                    break;
                }
            }
        }
    }
    if let Some((_, overlay)) = matched.iter().zip(overlays).find(|(count, _)| **count == 0) {
        return Err(ProjectionError {
            message: format!(
                "Rust dependency `{}` namespace overlay source `{}` contains no projectable items",
                overlay.provider_name, overlay.source_namespace
            ),
        });
    }
    let mut names = BTreeMap::<(&str, &str), (&str, bool)>::new();
    for (dependency_index, dependency) in projected.iter().enumerate() {
        for (item_index, item) in dependency.items.iter().enumerate() {
            let current_moved = moved.contains(&(dependency_index, item_index));
            if let Some((previous, previous_moved)) = names.insert(
                (item.namespace.as_str(), item.name.as_str()),
                (item.rust_path.as_str(), current_moved),
            ) && (previous_moved || current_moved)
            {
                return Err(ProjectionError {
                    message: format!(
                        "dependency namespace overlay collides on `{}::{}` between `{previous}` and `{}`",
                        item.namespace, item.name, item.rust_path
                    ),
                });
            }
        }
    }
    Ok(())
}
