//! Owns generated Cargo workspaces, execution, metadata, and resolved dependency locks.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

use super::{
    Containment, ProjectedBoundDependency, ProjectionError, RUSTDOC_TOOLCHAIN, RustDependency,
    io_error, write_if_changed,
};
pub(super) const DEPENDENCY_LOCK_FILE: &str = "terrane-dependencies.lock";

pub(super) fn seed_dependency_lock(root: &Path, workspace: &Path) -> Result<(), ProjectionError> {
    let path = root.join(DEPENDENCY_LOCK_FILE);
    let Ok(bytes) = fs::read(&path) else {
        return Ok(());
    };
    fs::create_dir_all(workspace).map_err(io_error("create dependency projection workspace"))?;
    write_if_changed(&workspace.join("Cargo.lock"), &bytes)
}

pub(super) fn persist_dependency_lock(
    root: &Path,
    workspace: &Path,
) -> Result<(), ProjectionError> {
    let bytes = fs::read(workspace.join("Cargo.lock"))
        .map_err(io_error("read complete resolved dependency lock"))?;
    write_if_changed(&root.join(DEPENDENCY_LOCK_FILE), &bytes)
}

pub(super) fn write_workspace_with_bound_dependencies(
    workspace: &Path,
    dependencies: &[RustDependency],
    bound_dependencies: &[ProjectedBoundDependency],
) -> Result<(), ProjectionError> {
    let mut dependencies = dependencies.to_vec();
    dependencies.extend(bound_dependencies.iter().map(|dependency| RustDependency {
        name: dependency.name.clone(),
        package: dependency.package.clone(),
        // This private edge makes an already-resolved recursive signature or bound owner
        // nameable to generated Rust. Empty feature lists prevent widening; the original
        // locked transitive edges retain every active feature.
        version: dependency.version.clone(),
        features: Vec::new(),
        default_features: false,
        target: None,
        effects: Vec::new(),
    }));
    write_workspace(workspace, &dependencies)?;
    run_cargo(
        workspace,
        &["fetch", "--offline"],
        CargoToolchain::Default,
        CargoExecution::Host,
    )
}
#[derive(Clone, Copy)]
pub(super) enum CargoToolchain {
    Default,
    RustdocNightly,
}

#[derive(Clone, Copy)]
pub(super) enum CargoExecution {
    Host,
    Contained,
}
pub(super) fn write_workspace(
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

pub(super) fn run_cargo(
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
    crate::rust_interop::configure_projection_cargo_command(&mut command);
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

pub(super) fn resolved_dependency_metadata(
    workspace: &Path,
) -> Result<serde_json::Value, ProjectionError> {
    let mut command = Command::new("cargo");
    crate::rust_interop::configure_projection_cargo_command(&mut command);
    let output = command
        .args(["metadata", "--format-version", "1", "--offline", "--frozen"])
        .current_dir(workspace)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot read Cargo dependency metadata: {error}"),
        })?;
    if !output.status.success() {
        return Err(ProjectionError {
            message: format!(
                "Cargo dependency metadata failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        });
    }
    serde_json::from_slice::<serde_json::Value>(&output.stdout).map_err(|error| ProjectionError {
        message: format!("cannot decode Cargo dependency metadata: {error}"),
    })
}
pub(super) fn resolved_package_versions(
    workspace: &Path,
) -> Result<BTreeMap<String, BTreeSet<String>>, ProjectionError> {
    let text = fs::read_to_string(workspace.join("Cargo.lock"))
        .map_err(io_error("read dependency projection lockfile"))?;
    let lock = text
        .parse::<toml::Value>()
        .map_err(|error| ProjectionError {
            message: format!("invalid dependency projection lockfile: {error}"),
        })?;
    let mut versions = BTreeMap::<String, BTreeSet<String>>::new();
    for package in lock
        .get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
    {
        let (Some(name), Some(version)) = (
            package.get("name").and_then(toml::Value::as_str),
            package.get("version").and_then(toml::Value::as_str),
        ) else {
            continue;
        };
        versions
            .entry(name.replace('-', "_"))
            .or_default()
            .insert(version.to_owned());
    }
    Ok(versions)
}
pub(super) fn is_crates_io_lock_source(source: &str) -> bool {
    source == "registry+https://github.com/rust-lang/crates.io-index"
}
pub(super) fn containment() -> Containment {
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
