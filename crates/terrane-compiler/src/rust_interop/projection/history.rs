use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{
    PROJECTION_SCHEMA, ProjectedBoundDependency, ProjectedDependency, ProjectedKind, Projection,
    ProjectionError, ProjectionResolution, ProjectionSource, RemovedItem, write_if_changed,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(super) struct ProjectionHistory {
    #[serde(default = "projection_history_format")]
    pub(super) format: u32,
    dependencies: Vec<ProjectionHistoryDependency>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) bound_dependencies: Vec<ProjectedBoundDependency>,
    #[serde(default)]
    removed: Vec<RemovedItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cache_identity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) source: Option<ProjectionSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) rustdoc_format: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    projection_schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) content_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) resolution: Option<ProjectionResolution>,
}

fn projection_history_format() -> u32 {
    1
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ProjectionHistoryDependency {
    name: String,
    version: String,
    #[serde(deserialize_with = "deserialize_projection_members")]
    members: BTreeMap<String, BTreeSet<String>>,
}

fn deserialize_projection_members<'de, D>(
    deserializer: D,
) -> Result<BTreeMap<String, BTreeSet<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Members {
        Grouped(BTreeMap<String, BTreeSet<String>>),
        Flat(BTreeSet<(String, String)>),
    }

    Ok(match Members::deserialize(deserializer)? {
        Members::Grouped(members) => members,
        Members::Flat(members) => {
            let mut grouped = BTreeMap::<String, BTreeSet<String>>::new();
            for (namespace, name) in members {
                grouped.entry(namespace).or_default().insert(name);
            }
            grouped
        }
    })
}

fn projection_history_members(
    dependency: &ProjectedDependency,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut members = BTreeMap::<String, BTreeSet<String>>::new();
    for item in &dependency.items {
        let names = members.entry(item.namespace.clone()).or_default();
        names.insert(item.name.clone());
        if let ProjectedKind::ForeignType {
            methods,
            static_methods,
            ..
        } = &item.kind
        {
            names.extend(
                methods
                    .iter()
                    .map(|method| format!("{}.{}", item.name, method.name)),
            );
            names.extend(
                static_methods
                    .iter()
                    .map(|method| format!("{}::{}", item.name, method.name)),
            );
        }
    }
    members
}

fn read_projection_history(path: &Path) -> Result<Option<ProjectionHistory>, ProjectionError> {
    match fs::read(path) {
        Ok(bytes) => {
            let history = serde_json::from_slice::<ProjectionHistory>(&bytes).map_err(|error| {
                ProjectionError {
                    message: format!("invalid projection history `{}`: {error}", path.display()),
                }
            })?;
            if !matches!(history.format, 1..=4) {
                return Err(ProjectionError {
                    message: format!(
                        "unsupported projection history format {} in `{}`",
                        history.format,
                        path.display()
                    ),
                });
            }
            Ok(Some(history))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(ProjectionError {
            message: format!(
                "cannot read projection history `{}`: {error}",
                path.display()
            ),
        }),
    }
}

fn retain_removed_history_members(
    previous: &[ProjectionHistoryDependency],
    current: &[ProjectionHistoryDependency],
    removed: &mut Vec<RemovedItem>,
) {
    for old in previous {
        let Some(current) = current
            .iter()
            .find(|dependency| dependency.name == old.name)
        else {
            continue;
        };
        if old.version == current.version {
            continue;
        }
        for (namespace, old_names) in &old.members {
            let current_names = current.members.get(namespace);
            for name in old_names
                .iter()
                .filter(|name| current_names.is_none_or(|names| !names.contains(*name)))
            {
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
}

pub(super) fn apply_projection_history(
    root: &Path,
    projection: &mut Projection,
) -> Result<(), ProjectionError> {
    let path = root.join("terrane-projection.lock");
    let previous = read_projection_history(&path)?;
    let dependencies = projection
        .dependencies
        .iter()
        .map(|dependency| ProjectionHistoryDependency {
            name: dependency.name.clone(),
            version: dependency.version.clone(),
            members: projection_history_members(dependency),
        })
        .collect::<Vec<_>>();
    if let Some(previous) = &previous
        && matches!(previous.format, 2..=4)
        && previous.dependencies == dependencies
        && previous.bound_dependencies == projection.bound_dependencies
        && previous.rustdoc_format == Some(rustdoc_types::FORMAT_VERSION)
        && previous.projection_schema.as_deref() == Some(PROJECTION_SCHEMA)
        && previous.cache_identity.as_deref() == Some(&projection.cache_identity)
        && previous.content_hash.as_deref() != Some(&projection.content_hash)
    {
        return Err(ProjectionError {
            message: format!(
                "projection replay mismatch in `{}`: the same cache identity previously produced `{}`, now produced `{}`",
                path.display(),
                previous.content_hash.as_deref().unwrap_or("missing"),
                projection.content_hash
            ),
        });
    }
    let mut removed = previous
        .as_ref()
        .map_or_else(Vec::new, |history| history.removed.clone());
    removed.retain(|removed| {
        !dependencies.iter().any(|dependency| {
            dependency
                .members
                .get(&removed.namespace)
                .is_some_and(|names| names.contains(&removed.name))
        })
    });
    if let Some(previous) = &previous {
        retain_removed_history_members(&previous.dependencies, &dependencies, &mut removed);
    }
    removed
        .sort_by(|left, right| (&left.namespace, &left.name).cmp(&(&right.namespace, &right.name)));
    projection.removed.clone_from(&removed);
    let persist_provenance = previous.as_ref().is_none_or(|history| {
        history.format < 4
            || history.cache_identity.is_some()
            || history.projection_schema.is_some()
            || history.content_hash.is_some()
            || history.resolution.is_some()
    });
    let persist_source = persist_provenance
        || previous
            .as_ref()
            .is_some_and(|history| history.source.is_some());
    let persist_rustdoc_format = persist_provenance
        || previous
            .as_ref()
            .is_some_and(|history| history.rustdoc_format.is_some());
    let persisted_resolution = persist_provenance.then(|| {
        previous
            .as_ref()
            .filter(|history| {
                matches!(history.format, 2..=4)
                    && history.bound_dependencies == projection.bound_dependencies
                    && history.cache_identity.as_deref() == Some(&projection.cache_identity)
                    && history.content_hash.as_deref() == Some(&projection.content_hash)
                    && history.source == Some(projection.source)
            })
            .and_then(|history| history.resolution.clone())
            .unwrap_or_else(|| projection.resolution.clone())
    });
    let history = ProjectionHistory {
        format: 4,
        dependencies,
        bound_dependencies: projection.bound_dependencies.clone(),
        removed,
        cache_identity: persist_provenance.then(|| projection.cache_identity.clone()),
        source: persist_source.then_some(projection.source),
        rustdoc_format: persist_rustdoc_format.then_some(rustdoc_types::FORMAT_VERSION),
        projection_schema: persist_provenance.then(|| PROJECTION_SCHEMA.to_owned()),
        content_hash: persist_provenance.then(|| projection.content_hash.clone()),
        resolution: persisted_resolution,
    };
    // Preserve existing formatting when the history contract has not changed.
    if previous.as_ref() == Some(&history) {
        return Ok(());
    }
    let mut bytes = serde_json::to_vec_pretty(&history).map_err(|error| ProjectionError {
        message: format!("cannot serialize projection history: {error}"),
    })?;
    bytes.push(b'\n');
    write_if_changed(&path, &bytes)
}
#[cfg(test)]
mod tests;
