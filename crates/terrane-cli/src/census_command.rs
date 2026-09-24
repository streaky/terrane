use std::ffi::OsString;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::CliFailure;

pub(crate) fn run(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let mut package = None;
    let mut version = None;
    let mut alias = None;
    let mut target = None;
    let mut target_condition = None;
    let mut features = Vec::new();
    let mut default_features = true;
    let mut root = None;
    let mut index = 1;
    while index < arguments.len() {
        let value = arguments[index]
            .to_str()
            .ok_or_else(|| CliFailure::usage_with("projection-census arguments must be UTF-8"))?;
        match value {
            "--alias" => alias = Some(value_after(arguments, &mut index, value)?),
            "--target" => target = Some(value_after(arguments, &mut index, value)?),
            "--target-condition" => {
                target_condition = Some(value_after(arguments, &mut index, value)?);
            }
            "--features" => {
                features.extend(
                    value_after(arguments, &mut index, value)?
                        .split(',')
                        .filter(|feature| !feature.is_empty())
                        .map(str::to_owned),
                );
            }
            "--no-default-features" => default_features = false,
            "--root" => root = Some(PathBuf::from(value_after(arguments, &mut index, value)?)),
            option if option.starts_with('-') => {
                return Err(CliFailure::usage_with(format!(
                    "unknown projection-census option `{option}`"
                )));
            }
            positional if package.is_none() => package = Some(positional.to_owned()),
            positional if version.is_none() => version = Some(positional.to_owned()),
            positional => {
                return Err(CliFailure::usage_with(format!(
                    "unexpected projection-census argument `{positional}`"
                )));
            }
        }
        index += 1;
    }
    let package = package.ok_or_else(|| CliFailure::usage_with("missing package name"))?;
    let version = version.ok_or_else(|| CliFailure::usage_with("missing package version"))?;
    let target = target.ok_or_else(|| CliFailure::usage_with("missing required --target"))?;
    let root = root.ok_or_else(|| CliFailure::usage_with("missing required --root"))?;
    std::fs::create_dir_all(&root)
        .map_err(|error| CliFailure::package(format!("cannot create census root: {error}")))?;
    features.sort();
    features.dedup();
    let dependency = terrane_compiler::RustDependency {
        name: alias.unwrap_or_else(|| package.replace('-', "_")),
        package,
        version: format!("={}", version.trim_start_matches('=')),
        features,
        default_features,
        target: target_condition,
        effects: Vec::new(),
    };
    let report =
        terrane_compiler::projection_census::assess_projection_gap(&root, &dependency, &target)
            .map_err(CliFailure::package)?;
    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| CliFailure::package(format!("cannot encode census report: {error}")))?;
    println!("{json}");
    Ok(ExitCode::SUCCESS)
}

fn value_after(
    arguments: &[OsString],
    index: &mut usize,
    option: &str,
) -> Result<String, CliFailure> {
    *index += 1;
    arguments
        .get(*index)
        .and_then(|value| value.to_str())
        .map(str::to_owned)
        .ok_or_else(|| CliFailure::usage_with(format!("missing value after `{option}`")))
}
