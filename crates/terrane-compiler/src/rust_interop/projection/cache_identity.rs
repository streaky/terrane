use super::{
    BTreeSet, Command, Containment, Digest, PROJECTION_SCHEMA, Path, ProjectionError,
    RUSTDOC_TOOLCHAIN, RustDependency, Sha256, fs,
};
fn projection_lock_identity(lock: &[u8]) -> Result<Vec<u8>, ProjectionError> {
    let Ok(text) = std::str::from_utf8(lock) else {
        return Ok(lock.to_vec());
    };
    let mut parsed = text
        .parse::<toml::Value>()
        .map_err(|error| ProjectionError {
            message: format!("invalid dependency projection lockfile: {error}"),
        })?;
    if let Some(packages) = parsed
        .get_mut("package")
        .and_then(toml::Value::as_array_mut)
    {
        packages.retain(|package| {
            package.get("name").and_then(toml::Value::as_str)
                != Some("terrane_dependency_projection")
        });
    }
    Ok(parsed.to_string().into_bytes())
}

pub(super) fn cache_identity(
    root: &Path,
    workspace: &Path,
    dependencies: &[RustDependency],
    demands: Option<&BTreeSet<(String, String)>>,
    containment: Containment,
) -> Result<(String, String), ProjectionError> {
    let manifest = fs::read(root.join(crate::MANIFEST_FILE_NAME)).unwrap_or_default();
    let lock = fs::read(workspace.join("Cargo.lock"))
        .or_else(|_| fs::read(root.join("Cargo.lock")))
        .unwrap_or_default();
    let lock = projection_lock_identity(&lock)?;
    let build_selector = format!("+{}", crate::BUILD_TOOLCHAIN);
    let rustc_verbose = tool_version("rustc", &[&build_selector, "-vV"])?;
    let target = selected_target(workspace, &rustc_verbose)?;
    let rustdoc_format = rustdoc_types::FORMAT_VERSION.to_string();
    let mut hash = Sha256::new();
    for (label, bytes) in [
        ("manifest", manifest.as_slice()),
        ("lock", lock.as_slice()),
        ("inputs", format!("{dependencies:?}").as_bytes()),
        ("target", target.as_bytes()),
        ("source-demands", format!("{demands:?}").as_bytes()),
        ("build-toolchain", crate::BUILD_TOOLCHAIN.as_bytes()),
        ("rustdoc-toolchain", RUSTDOC_TOOLCHAIN.as_bytes()),
        ("rustdoc-format", rustdoc_format.as_bytes()),
        ("schema", PROJECTION_SCHEMA.as_bytes()),
        ("containment", format!("{containment:?}").as_bytes()),
    ] {
        hash.update(label.len().to_le_bytes());
        hash.update(label.as_bytes());
        hash.update(bytes.len().to_le_bytes());
        hash.update(bytes);
    }
    Ok((format!("{:x}", hash.finalize()), target))
}

pub(super) fn selected_target(
    workspace: &Path,
    rustc_verbose_version: &str,
) -> Result<String, ProjectionError> {
    if let Some(target) = std::env::var("CARGO_BUILD_TARGET")
        .ok()
        .filter(|target| !target.is_empty())
    {
        return Ok(target);
    }
    let output = Command::new("cargo")
        .arg(format!("+{RUSTDOC_TOOLCHAIN}"))
        .args([
            "-Z",
            "unstable-options",
            "config",
            "get",
            "build.target",
            "--format",
            "json",
        ])
        .current_dir(workspace)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot inspect Cargo build target configuration: {error}"),
        })?;
    if output.status.success() {
        let config =
            serde_json::from_slice::<serde_json::Value>(&output.stdout).map_err(|error| {
                ProjectionError {
                    message: format!("cannot decode Cargo build target configuration: {error}"),
                }
            })?;
        let target = config
            .get("build.target")
            .or_else(|| config.get("build").and_then(|build| build.get("target")));
        return match target {
            Some(serde_json::Value::String(target)) => Ok(target.clone()),
            Some(serde_json::Value::Array(targets)) if targets.len() == 1 => targets[0]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| ProjectionError {
                    message: "Cargo build target configuration is not a string".to_owned(),
                }),
            Some(serde_json::Value::Array(_)) => Err(ProjectionError {
                message: "dependency projection requires exactly one Cargo build target".to_owned(),
            }),
            _ => Err(ProjectionError {
                message: "Cargo returned no usable build target configuration".to_owned(),
            }),
        };
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.contains("config value `build.target` is not set") {
        return Err(ProjectionError {
            message: format!(
                "cannot inspect Cargo build target configuration: {}",
                stderr.trim()
            ),
        });
    }
    Ok(rustc_verbose_version
        .lines()
        .find_map(|line| line.strip_prefix("host: ").map(str::to_owned))
        .unwrap_or_else(|| "unknown-target".to_owned()))
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
