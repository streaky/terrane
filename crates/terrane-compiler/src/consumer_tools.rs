//! Resolution and host-native builds for explicitly exported declaration-consumer tools.

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::package::{ConsumerConfig, ConsumerExecutable};
use crate::{CompilationFailure, Diagnostic, Package, Span};

thread_local! {
    static BUILD_ACTION_STACK: RefCell<BTreeSet<PathBuf>> = const { RefCell::new(BTreeSet::new()) };
}

struct BuildActionGuard(PathBuf);

impl BuildActionGuard {
    fn enter(path: &Path, owner: &Package) -> Result<Self, CompilationFailure> {
        let canonical = path.canonicalize().map_err(|error| {
            failure(
                owner,
                format!(
                    "cannot resolve consumer tool manifest `{}`: {error}",
                    path.display()
                ),
            )
        })?;
        let cycle = BUILD_ACTION_STACK.with(|stack| !stack.borrow_mut().insert(canonical.clone()));
        if cycle {
            return Err(failure(
                owner,
                format!(
                    "consumer tool build-action cycle detected at `{}`",
                    canonical.display()
                ),
            ));
        }
        Ok(Self(canonical))
    }
}

impl Drop for BuildActionGuard {
    fn drop(&mut self) {
        BUILD_ACTION_STACK.with(|stack| {
            stack.borrow_mut().remove(&self.0);
        });
    }
}

/// Resolve and build a configured tool, returning the host-native executable path.
///
/// # Errors
///
/// Returns source-oriented diagnostics for missing, ambiguous, invalid, or unbuildable exports.
pub(crate) fn executable_for(
    owner: &Package,
    config: &ConsumerConfig,
) -> Result<PathBuf, CompilationFailure> {
    let ConsumerExecutable::Buildable {
        dependency, tool, ..
    } = &config.executable
    else {
        if let ConsumerExecutable::Command { executable, .. } = &config.executable {
            return resolve_command(owner, executable);
        }
        unreachable!();
    };
    let Some(root) = owner.dependency_roots.get(dependency) else {
        return Err(failure(
            owner,
            format!(
                "consumer `{}` names undeclared or unresolved tool dependency `{dependency}`",
                config.name
            ),
        ));
    };
    let package_manifest = root.join(crate::MANIFEST_FILE_NAME);
    let exports = crate::package::consumer_tool_exports(&package_manifest).map_err(|errors| {
        failure(
            owner,
            format!(
                "cannot load consumer tool dependency `{dependency}`: {}",
                errors
                    .into_iter()
                    .map(|error| error.diagnostic.message)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        )
    })?;
    let Some(relative_manifest) = exports.get(tool) else {
        return Err(failure(
            owner,
            format!("dependency `{dependency}` does not export consumer tool `{tool}`"),
        ));
    };
    let tool_manifest = root.join(relative_manifest);
    if !tool_manifest.is_file() {
        return Err(failure(
            owner,
            format!(
                "consumer tool `{dependency}::{tool}` manifest `{}` does not exist",
                tool_manifest.display()
            ),
        ));
    }
    let tool_manifest = tool_manifest.canonicalize().map_err(|error| {
        failure(
            owner,
            format!("cannot resolve consumer tool manifest: {error}"),
        )
    })?;
    let package = crate::Package::load(&tool_manifest).map_err(|errors| {
        failure(
            owner,
            format!(
                "cannot load consumer tool `{dependency}::{tool}`: {}",
                errors
                    .into_iter()
                    .map(|error| error.diagnostic.message)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        )
    })?;
    if package.artifact != crate::ArtifactKind::Executable {
        return Err(failure(
            owner,
            format!("consumer tool `{dependency}::{tool}` must be an executable package"),
        ));
    }
    let _guard = BuildActionGuard::enter(&tool_manifest, owner)?;
    build_host_executable(owner, &package)
}

fn resolve_command(owner: &Package, executable: &Path) -> Result<PathBuf, CompilationFailure> {
    if executable.is_absolute() || executable.components().count() > 1 {
        let path = if executable.is_absolute() {
            executable.to_path_buf()
        } else {
            owner.root.join(executable)
        };
        return path.is_file().then_some(path).ok_or_else(|| {
            failure(
                owner,
                format!("cannot locate consumer command `{}`", executable.display()),
            )
        });
    }
    let Some(paths) = std::env::var_os("PATH") else {
        return Err(failure(
            owner,
            format!(
                "cannot resolve consumer command `{}`: PATH is not set",
                executable.display()
            ),
        ));
    };
    #[cfg(windows)]
    let names = {
        let has_extension = executable.extension().is_some();
        let extensions = if has_extension {
            vec![std::ffi::OsString::new()]
        } else {
            std::env::var_os("PATHEXT")
                .unwrap_or_else(|| ".COM;.EXE;.BAT;.CMD".into())
                .to_string_lossy()
                .split(';')
                .filter(|extension| !extension.is_empty())
                .map(std::ffi::OsString::from)
                .collect()
        };
        extensions
            .into_iter()
            .map(|extension| {
                let mut name = executable.as_os_str().to_os_string();
                name.push(extension);
                name
            })
            .collect::<Vec<_>>()
    };
    #[cfg(not(windows))]
    let names = vec![executable.as_os_str().to_os_string()];
    for directory in std::env::split_paths(&paths) {
        let directory = if directory.is_absolute() {
            directory
        } else {
            owner.root.join(directory)
        };
        for name in &names {
            let candidate = directory.join(name);
            if is_executable_file(&candidate) {
                return Ok(candidate);
            }
        }
    }
    Err(failure(
        owner,
        format!(
            "consumer command `{}` was not found on PATH",
            executable.display()
        ),
    ))
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .is_ok_and(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_executable_file(path: &Path) -> bool {
    path.is_file()
}

fn build_host_executable(
    owner: &Package,
    package: &Package,
) -> Result<PathBuf, CompilationFailure> {
    let mut rustc = Command::new("rustc");
    if package.build_toolchain == crate::BuildToolchain::Pinned {
        rustc.env("RUSTUP_TOOLCHAIN", crate::BUILD_TOOLCHAIN);
    }
    let output = rustc
        .arg("-vV")
        .output()
        .map_err(|error| failure(owner, format!("cannot query rustc host target: {error}")))?;
    if !output.status.success() {
        return Err(failure(
            owner,
            format!(
                "rustc host target query failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    let version = String::from_utf8_lossy(&output.stdout);
    let host = version
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .ok_or_else(|| failure(owner, "rustc did not report its host target".to_owned()))?;
    terrane_rust_analysis::with_cargo_target(host, || {
        build_host_executable_for_target(owner, package, host)
    })
}

fn build_host_executable_for_target(
    diagnostic_owner: &Package,
    package: &Package,
    host: &str,
) -> Result<PathBuf, CompilationFailure> {
    use crate::native_build::prepare_cargo_command;
    let locks = package.root.join(".trn/locks");
    fs::create_dir_all(&locks).map_err(|error| {
        failure(
            diagnostic_owner,
            format!("cannot create consumer build lock directory: {error}"),
        )
    })?;
    let lock = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(locks.join("consumer-tool.lock"))
        .map_err(|error| {
            failure(
                diagnostic_owner,
                format!("cannot open consumer build lock: {error}"),
            )
        })?;
    lock.lock().map_err(|error| {
        failure(
            diagnostic_owner,
            format!("cannot lock consumer tool build: {error}"),
        )
    })?;
    let compilation = compile_tool(diagnostic_owner, package)?;
    let crate_root = write_host_workspace(diagnostic_owner, package, &compilation)?;
    let target = package.root.join(".trn/cache/target");
    let mut cargo = prepare_cargo_command(
        &crate_root,
        &target,
        !compilation.rust_dependencies.is_empty(),
        compilation.dependency_containment,
    )
    .map_err(|error| failure(diagnostic_owner, error.to_string()))?;
    cargo
        .args(["build", "--quiet", "--manifest-path"])
        .arg(crate_root.join("Cargo.toml"))
        .arg("--target")
        .arg(host);
    if !compilation.rust_dependencies.is_empty() {
        cargo.args(["--offline", "--frozen"]);
    }
    if let Some(identity) = compilation.native_dependency_identity() {
        use sha2::{Digest as _, Sha256};
        let metadata = format!(
            "-Cmetadata=terrane_consumer_{:x}",
            Sha256::digest(identity.as_bytes())
        );
        if let Some(mut flags) = std::env::var_os("CARGO_ENCODED_RUSTFLAGS") {
            if !flags.is_empty() {
                flags.push("\u{1f}");
            }
            flags.push(metadata);
            cargo.env("CARGO_ENCODED_RUSTFLAGS", flags);
        } else {
            let mut flags = std::env::var_os("RUSTFLAGS").unwrap_or_default();
            flags.push(" ");
            flags.push(metadata);
            cargo.env("RUSTFLAGS", flags);
        }
    }
    let output = cargo
        .env("CARGO_BUILD_TARGET", host)
        .output()
        .map_err(|error| {
            failure(
                diagnostic_owner,
                format!("cannot start native consumer tool build: {error}"),
            )
        })?;
    if !output.status.success() {
        return Err(failure(
            diagnostic_owner,
            format!(
                "native consumer tool build failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    let mut executable = target.join(host).join("debug/terrane_consumer_tool");
    executable.set_extension(std::env::consts::EXE_EXTENSION);
    if !executable.is_file() {
        return Err(failure(
            diagnostic_owner,
            format!(
                "Cargo did not produce host executable `{}`",
                executable.display()
            ),
        ));
    }
    executable.canonicalize().map_err(|error| {
        failure(
            diagnostic_owner,
            format!("cannot locate built consumer tool: {error}"),
        )
    })
}

fn compile_tool(
    owner: &Package,
    package: &Package,
) -> Result<crate::Compilation, CompilationFailure> {
    use std::fmt::Write as _;
    crate::with_compilation_cache(|| crate::compile_package(package)).map_err(|problem| {
        // Consumer graphs have independent file IDs. Report their compiler errors
        // at the owning build-action boundary, retaining their actual origin.
        let mut message = format!(
            "cannot compile consumer tool `{}` at `{}`:",
            package.identity,
            problem.source.path().display()
        );
        for diagnostic in problem.diagnostics {
            let _ = write!(message, "\n[{}] {}", diagnostic.code, diagnostic.message);
        }
        failure(owner, message)
    })
}

fn failure(package: &Package, message: String) -> CompilationFailure {
    let span = package
        .units
        .first()
        .map_or(Span::new(0, 0, 0), |unit| Span::new(unit.source.id(), 0, 0));
    let source = package.units.first().map_or_else(
        || {
            crate::SourceFile::new(
                0,
                package.root.join(crate::MANIFEST_FILE_NAME),
                String::new(),
            )
        },
        |unit| unit.source.clone(),
    );
    CompilationFailure {
        source,
        diagnostics: vec![Diagnostic::error("S2060", message, span)],
    }
}

fn write_host_workspace(
    diagnostic_owner: &Package,
    package: &Package,
    compilation: &crate::Compilation,
) -> Result<PathBuf, CompilationFailure> {
    use crate::native_build::{
        GeneratedCrateOptions, GeneratedCrateRole, UnsafeCodePolicy, write_generated_crate,
    };
    let files = compilation
        .rust_files_for(Path::new("src/main.rs"))
        .map_err(|error| match error {
            crate::RustArtifactError::Compilation(failure) => failure,
            crate::RustArtifactError::InvalidOutputPath(message) => {
                failure(diagnostic_owner, message)
            }
        })?;
    let crate_root = package.root.join(".trn/build/consumer-tool/host");
    write_generated_crate(
        &crate_root,
        &files,
        &package.units,
        &compilation.rust_dependencies,
        GeneratedCrateOptions {
            role: GeneratedCrateRole::ConsumerTool,
            panic: package.profile.panic,
            uses_platform_support: compilation.requires_platform_support,
            uses_async_runtime: compilation.requires_async_runtime,
            uses_tokio_blocking: compilation.requires_blocking_runtime,
            uses_tokio_sync: compilation.requires_runtime_sync,
            build_toolchain: package.build_toolchain,
            unsafe_code: if compilation.requires_unsafe_code
                || !package.authored_rust_modules.is_empty()
            {
                UnsafeCodePolicy::MaintainedModules
            } else {
                UnsafeCodePolicy::Forbid
            },
            artifact: crate::ArtifactKind::Executable,
        },
    )
    .map_err(|error| failure(diagnostic_owner, error.to_string()))?;
    let crate_root = crate_root.canonicalize().map_err(|error| {
        failure(
            diagnostic_owner,
            format!("cannot resolve consumer native workspace: {error}"),
        )
    })?;
    if let Ok(lock) = fs::read(package.root.join("terrane-dependencies.lock")) {
        crate::native_build::write_if_changed(&crate_root.join("Cargo.lock"), &lock).map_err(
            |error| {
                failure(
                    diagnostic_owner,
                    format!("cannot seed consumer native lock: {error}"),
                )
            },
        )?;
    }
    Ok(crate_root)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::{PermissionsExt, symlink};

    #[test]
    fn command_preserves_symlink_launcher_identity() {
        let root =
            std::env::temp_dir().join(format!("terrane-command-launcher-{}", std::process::id()));
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("bin")).unwrap();
        fs::write(
            root.join("package.toml"),
            "package = 'launcher'\n[namespaces]\nlauncher = 'src'\n",
        )
        .unwrap();
        fs::write(
            root.join("src/main.trn"),
            "namespace launcher\nfunction main;\n",
        )
        .unwrap();
        let script = root.join("bin/real");
        fs::write(&script, "#!/bin/sh\nprintf '%s' \"$0\"\n").unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        let launcher = root.join("bin/link");
        symlink(&script, &launcher).unwrap();
        let package = Package::load(root.join("package.toml")).unwrap();
        for configured in [Path::new("bin/link"), launcher.as_path()] {
            let resolved = resolve_command(&package, configured).unwrap();
            let output = Command::new(resolved)
                .current_dir(&package.root)
                .output()
                .unwrap();
            assert!(output.status.success());
            assert_eq!(
                String::from_utf8(output.stdout).unwrap(),
                launcher.to_str().unwrap()
            );
        }
        fs::set_permissions(&script, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(!is_executable_file(&script));
        fs::remove_dir_all(root).unwrap();
    }
}
