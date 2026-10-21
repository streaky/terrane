use super::{
    ConsumerConfig, ConsumerExecutable, PackageLoadError, dependency_identity_parts, manifest_error,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
fn declarations_ok(declarations: &[String]) -> bool {
    !declarations.is_empty()
        && declarations
            .iter()
            .all(|identity| !identity.trim().is_empty())
}
pub(super) fn parse_consumer_tools(
    manifest_path: &Path,
    text: &str,
    table: &toml::Table,
    errors: &mut Vec<PackageLoadError>,
) -> BTreeMap<String, PathBuf> {
    let Some(value) = table.get("consumer-tools") else {
        return BTreeMap::new();
    };
    let Some(exports) = value.as_table() else {
        errors.push(manifest_error(
            manifest_path,
            text,
            "`consumer-tools` must be a table mapping tool names to package manifests",
            Some("consumer-tools"),
        ));
        return BTreeMap::new();
    };
    let mut tools = BTreeMap::new();
    for (name, value) in exports {
        let Some(path) = value.as_str().filter(|path| !path.trim().is_empty()) else {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("consumer tool `{name}` must name a non-empty manifest path"),
                Some(name),
            ));
            continue;
        };
        let path = PathBuf::from(path);
        if path.is_absolute()
            || path.components().any(|component| {
                matches!(
                    component,
                    std::path::Component::ParentDir
                        | std::path::Component::RootDir
                        | std::path::Component::Prefix(_)
                )
            })
        {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("consumer tool `{name}` manifest path must stay within its package"),
                Some(name),
            ));
            continue;
        }
        tools.insert(name.clone(), path);
    }
    tools
}

pub(super) fn parse_consumers(
    manifest_path: &Path,
    text: &str,
    table: &toml::Table,
    errors: &mut Vec<PackageLoadError>,
) -> Vec<ConsumerConfig> {
    let Some(toml::Value::Table(consumers)) = table.get("consumers") else {
        if table.contains_key("consumers") {
            errors.push(manifest_error(
                manifest_path,
                text,
                "`consumers` must be a table of explicitly configured consumers",
                Some("consumers"),
            ));
        }
        return Vec::new();
    };
    let mut configs = Vec::new();
    for (name, value) in consumers {
        let Some(fields) = value.as_table() else {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("consumer `{name}` must be a table"),
                Some(name),
            ));
            continue;
        };
        let arguments = consumer_string_array(
            manifest_path,
            text,
            name,
            "args",
            fields.get("args"),
            errors,
        );
        let declarations = consumer_string_array(
            manifest_path,
            text,
            name,
            "declarations",
            fields.get("declarations"),
            errors,
        );
        let interfaces = consumer_string_array(
            manifest_path,
            text,
            name,
            "interfaces",
            fields.get("interfaces"),
            errors,
        )
        .into_iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();
        let executable = parse_executable(fields, arguments).and_then(|executable| {
            if declarations_ok(&declarations) {
                Ok(executable)
            } else {
                Err("requires non-empty declaration identities")
            }
        });
        let executable = match executable {
            Ok(executable) => executable,
            Err(message) => {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("consumer `{name}` {message}"),
                    Some(name),
                ));
                continue;
            }
        };
        if fields.keys().any(|key| {
            !matches!(
                key.as_str(),
                "command" | "tool" | "args" | "declarations" | "interfaces"
            )
        }) {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("consumer `{name}` has unknown or incompatible configuration fields"),
                Some(name),
            ));
            continue;
        }
        configs.push(ConsumerConfig {
            name: name.clone(),
            executable,
            declarations,
            interfaces,
        });
    }
    configs.sort_by(|left, right| left.name.cmp(&right.name));
    configs
}

fn parse_executable(
    fields: &toml::Table,
    arguments: Vec<String>,
) -> Result<ConsumerExecutable, &'static str> {
    match (fields.get("command"), fields.get("tool")) {
        (Some(toml::Value::String(command)), None) if !command.trim().is_empty() => {
            Ok(ConsumerExecutable::Command {
                executable: command.into(),
                arguments,
            })
        }
        (None, Some(toml::Value::String(tool))) => {
            let (dependency, export) = tool
                .split_once("::")
                .ok_or("tool must be `owner/package::tool-name`")?;
            if dependency_identity_parts(dependency).is_none()
                || export.trim().is_empty()
                || export.contains("::")
            {
                return Err("has an invalid canonical tool identity");
            }
            Ok(ConsumerExecutable::Buildable {
                dependency: dependency.to_owned(),
                tool: export.to_owned(),
                arguments,
            })
        }
        _ => Err("requires exactly one non-empty command or tool"),
    }
}

fn consumer_string_array(
    manifest_path: &Path,
    text: &str,
    consumer: &str,
    field: &str,
    value: Option<&toml::Value>,
    errors: &mut Vec<PackageLoadError>,
) -> Vec<String> {
    let Some(value) = value else {
        return Vec::new();
    };
    let Some(values) = value.as_array() else {
        errors.push(manifest_error(
            manifest_path,
            text,
            format!("consumer `{consumer}` field `{field}` must be an array of strings"),
            Some(consumer),
        ));
        return Vec::new();
    };
    if values.iter().any(|value| value.as_str().is_none()) {
        errors.push(manifest_error(
            manifest_path,
            text,
            format!("consumer `{consumer}` field `{field}` must contain only strings"),
            Some(consumer),
        ));
        return Vec::new();
    }
    values
        .iter()
        .filter_map(toml::Value::as_str)
        .map(str::to_owned)
        .collect()
}
