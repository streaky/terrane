//! Shared generated-Cargo crate preparation for CLI artifacts and host consumer tools.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
pub struct NativeBuildError(String);

impl NativeBuildError {
    fn backend(message: String) -> Self {
        Self(message)
    }
}

impl std::fmt::Display for NativeBuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for NativeBuildError {}

pub(crate) fn write_if_changed(path: &Path, content: &[u8]) -> std::io::Result<()> {
    if fs::read(path).is_ok_and(|existing| existing == content) {
        return Ok(());
    }
    fs::write(path, content)
}
#[derive(Clone, Copy)]
pub enum UnsafeCodePolicy {
    Forbid,
    MaintainedModules,
}

impl UnsafeCodePolicy {
    const fn lint_level(self) -> &'static str {
        match self {
            Self::Forbid => "forbid",
            Self::MaintainedModules => "deny",
        }
    }
}

#[derive(Clone, Copy)]
pub enum GeneratedCrateRole {
    Application,
    Inspection,
    UnitTest,
    IntegrationTest,
    EndToEndTest,
    ConsumerTool,
}

impl GeneratedCrateRole {
    #[must_use]
    pub const fn directory(self) -> &'static str {
        match self {
            Self::Application => "application",
            Self::Inspection => "application-debug",
            Self::UnitTest => "test-unit",
            Self::IntegrationTest => "test-integration",
            Self::EndToEndTest => "test-end-to-end",
            Self::ConsumerTool => "consumer-tool",
        }
    }

    #[must_use]
    pub const fn cargo_name(self) -> &'static str {
        match self {
            Self::Application => "terrane_program_application",
            Self::Inspection => "terrane_program_application_debug",
            Self::UnitTest => "terrane_program_test_unit",
            Self::IntegrationTest => "terrane_program_test_integration",
            Self::EndToEndTest => "terrane_program_test_end_to_end",
            Self::ConsumerTool => "terrane_consumer_tool",
        }
    }
}

#[derive(Clone, Copy)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Generated Cargo features are independent additive requirements, not mutually exclusive runtime states"
)]
pub struct GeneratedCrateOptions {
    pub role: GeneratedCrateRole,
    pub panic: crate::PanicProfile,
    pub uses_platform_support: bool,
    pub uses_async_runtime: bool,
    pub uses_tokio_blocking: bool,
    pub uses_tokio_sync: bool,
    pub build_toolchain: crate::BuildToolchain,
    pub unsafe_code: UnsafeCodePolicy,
    pub artifact: crate::ArtifactKind,
}

fn base_generated_manifest(unsafe_code: UnsafeCodePolicy, role: GeneratedCrateRole) -> String {
    format!(
        "[package]\nname = {:?}\nversion = \"0.0.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n\
         [package.metadata.terrane]\nunicode-data-version = {:?}\n\n\
         [lints.rust]\nunsafe_code = {:?}\n\n\
         [dependencies]\nterrane-int-support = {{ path = \"support/terrane-int-support\" }}\n\
         terrane-collection-support = {{ path = \"support/terrane-collection-support\" }}\n\
         terrane-scalar-support = {{ path = \"support/terrane-scalar-support\" }}\n\
         terrane-string-support = {{ path = \"support/terrane-string-support\" }}\n\
         terrane-document-support = {{ path = \"support/terrane-document-support\" }}\n\
         terrane-stream-abi = {{ path = \"support/terrane-stream-abi\" }}\n",
        role.cargo_name(),
        crate::BUILD_TOOLCHAIN,
        crate::UNICODE_DATA_VERSION,
        unsafe_code.lint_level()
    )
}

fn append_build_profiles(manifest: &mut String, panic: crate::PanicProfile) {
    if panic == crate::PanicProfile::Abort {
        manifest.push_str("\n[profile.dev]\npanic = \"abort\"\n");
    }
    for (name, parent, profile) in [
        (
            "terrane-debug",
            "dev",
            crate::debugging::DEBUG_ARTIFACT_PROFILE,
        ),
        (
            "terrane-profile",
            "release",
            crate::profiling::CPU_ARTIFACT_PROFILE,
        ),
    ] {
        writeln!(manifest, "\n[profile.{name}]\ninherits = {parent:?}")
            .expect("writing to a string cannot fail");
        writeln!(manifest, "opt-level = {}", profile.optimization)
            .expect("writing to a string cannot fail");
        writeln!(manifest, "debug = {}", profile.cargo_debug)
            .expect("writing to a string cannot fail");
        writeln!(manifest, "strip = {:?}", profile.stripping)
            .expect("writing to a string cannot fail");
        writeln!(manifest, "lto = {:?}", profile.lto).expect("writing to a string cannot fail");
        writeln!(manifest, "codegen-units = {}", profile.codegen_units)
            .expect("writing to a string cannot fail");
        if panic == crate::PanicProfile::Abort {
            manifest.push_str("panic = \"abort\"\n");
        }
    }
    manifest.push_str("\n[profile.release]\nopt-level = 3\nlto = \"fat\"\ncodegen-units = 1\n");
}

fn remove_obsolete_generated_sources(
    root: &Path,
    files: &[crate::rust_ir::RenderedFile],
) -> std::io::Result<()> {
    for entry in fs::read_dir(root.join("src"))? {
        let entry = entry?;
        let path = entry.path();
        let relative = path
            .strip_prefix(root)
            .expect("generated source entry belongs to its crate");
        if path.extension().is_some_and(|extension| extension == "rs")
            && entry.file_type()?.is_file()
            && !files.iter().any(|file| Path::new(&file.path) == relative)
        {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

/// Write a role-owned generated crate, preserving unchanged inputs and retiring
/// obsolete source/support files.
///
/// # Errors
///
/// Returns filesystem failures while preparing the native workspace.
pub fn write_generated_crate(
    directory: &Path,
    rust_files: &[crate::rust_ir::RenderedFile],
    units: &[crate::SourceUnit],
    rust_dependencies: &[crate::RustDependency],
    options: GeneratedCrateOptions,
) -> Result<(), NativeBuildError> {
    fs::create_dir_all(directory.join("src")).map_err(|error| {
        NativeBuildError::backend(format!("cannot create generated crate: {error}"))
    })?;
    remove_obsolete_generated_sources(directory, rust_files).map_err(|error| {
        NativeBuildError::backend(format!("cannot remove obsolete source: {error}"))
    })?;
    if !options.uses_platform_support {
        for name in ["terrane-platform-support", "terrane-signal-support"] {
            let obsolete = directory.join("support").join(name);
            if obsolete.is_dir() {
                fs::remove_dir_all(obsolete).map_err(|error| {
                    NativeBuildError::backend(format!(
                        "cannot remove obsolete generated support: {error}"
                    ))
                })?;
            }
        }
    }
    let mut manifest = base_generated_manifest(options.unsafe_code, options.role);
    if options.uses_platform_support {
        manifest.push_str(
            "terrane-platform-support = { path = \"support/terrane-platform-support\" }\n",
        );
    }
    write_runtime_dependencies(&mut manifest, rust_dependencies, options);
    let target_tables = rust_dependencies
        .iter()
        .map(crate::RustDependency::cargo_manifest_table)
        .filter(|table| table != "dependencies")
        .collect::<BTreeSet<_>>();
    for table in target_tables {
        writeln!(manifest, "\n[{table}]").expect("writing to a string cannot fail");
        for dependency in rust_dependencies
            .iter()
            .filter(|dependency| dependency.cargo_manifest_table() == table)
        {
            write_rust_dependency(&mut manifest, dependency);
        }
    }
    if options.artifact == crate::ArtifactKind::DynamicLibrary {
        manifest.push_str("\n[lib]\ncrate-type = [\"cdylib\"]\n");
    }
    append_build_profiles(&mut manifest, options.panic);
    manifest.push_str("\n[workspace]\n");
    write_if_changed(&directory.join("Cargo.toml"), manifest.as_bytes()).map_err(|error| {
        NativeBuildError::backend(format!("cannot write generated manifest: {error}"))
    })?;
    let toolchain_path = directory.join("rust-toolchain.toml");
    if options.build_toolchain == crate::BuildToolchain::Pinned {
        let toolchain = format!(
            "[toolchain]\nchannel = {:?}\nprofile = \"minimal\"\n",
            crate::BUILD_TOOLCHAIN
        );
        write_if_changed(&toolchain_path, toolchain.as_bytes()).map_err(|error| {
            NativeBuildError::backend(format!("cannot write generated toolchain pin: {error}"))
        })?;
    } else if toolchain_path.exists() {
        fs::remove_file(&toolchain_path).map_err(|error| {
            NativeBuildError::backend(format!("cannot remove generated toolchain pin: {error}"))
        })?;
    }
    write_generated_support(directory, options.uses_platform_support).map_err(|error| {
        NativeBuildError::backend(format!("cannot write generated runtime support: {error}"))
    })?;
    for rust_file in rust_files {
        let path = directory.join(&rust_file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                NativeBuildError::backend(format!(
                    "cannot create generated source directory: {error}"
                ))
            })?;
        }
        write_if_changed(&path, rust_file.contents.as_bytes()).map_err(|error| {
            NativeBuildError::backend(format!("cannot write generated Rust: {error}"))
        })?;
    }
    let mut sources = String::from("version = 1\n\n");
    for unit in units {
        write!(
            sources,
            "[[sources]]\npath = {:?}\n",
            unit.relative_path.to_string_lossy()
        )
        .expect("writing to a String cannot fail");
    }
    writeln!(
        sources,
        "rust-toolchain = {:?}",
        match options.build_toolchain {
            crate::BuildToolchain::Pinned => crate::BUILD_TOOLCHAIN,
            crate::BuildToolchain::System => "system",
        }
    )
    .expect("writing to a String cannot fail");
    write_if_changed(&directory.join("terrane-build.toml"), sources.as_bytes()).map_err(
        |error| NativeBuildError::backend(format!("cannot write build metadata: {error}")),
    )?;
    Ok(())
}

fn write_runtime_dependencies(
    manifest: &mut String,
    rust_dependencies: &[crate::RustDependency],
    options: GeneratedCrateOptions,
) {
    let dependencies =
        if options.uses_async_runtime || options.uses_tokio_blocking || options.uses_tokio_sync {
            let mut required_features = vec!["rt"];
            if options.uses_async_runtime {
                required_features.extend(["macros", "rt-multi-thread", "time"]);
            }
            if options.uses_tokio_sync {
                required_features.push("sync");
            }
            crate::with_tokio_runtime(rust_dependencies, &required_features)
        } else {
            rust_dependencies.to_vec()
        };
    for dependency in dependencies
        .iter()
        .filter(|dependency| dependency.cargo_manifest_table() == "dependencies")
    {
        write_rust_dependency(manifest, dependency);
    }
}

fn write_rust_dependency(manifest: &mut String, dependency: &crate::RustDependency) {
    manifest.push_str(&dependency.cargo_dependency_spec());
}

fn write_generated_support(directory: &Path, uses_platform_support: bool) -> std::io::Result<()> {
    let int = directory.join("support/terrane-int-support");
    let collection = directory.join("support/terrane-collection-support");
    let scalar = directory.join("support/terrane-scalar-support");
    let string = directory.join("support/terrane-string-support");
    let document = directory.join("support/terrane-document-support");
    let stream = directory.join("support/terrane-stream-abi");
    fs::create_dir_all(int.join("src"))?;
    fs::create_dir_all(collection.join("src"))?;
    fs::create_dir_all(scalar.join("src"))?;
    fs::create_dir_all(string.join("src"))?;
    fs::create_dir_all(document.join("src"))?;
    fs::create_dir_all(stream.join("src"))?;
    write_if_changed(
        &int.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-int-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nnum-bigint = {{ version = \"0.4\", features = [\"std\"] }}\nnum-integer = \"0.1\"\nnum-traits = \"0.2\"\n", crate::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &collection.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-collection-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nindexmap = \"2\"\nterrane-int-support = {{ path = \"../terrane-int-support\" }}\nunicode-segmentation = \"=1.12.0\"\n", crate::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &collection.join("src/lib.rs"),
        include_bytes!("../../terrane-collection-support/src/lib.rs"),
    )?;
    write_if_changed(
        &int.join("src/lib.rs"),
        include_bytes!("../../terrane-int-support/src/lib.rs"),
    )?;
    write_if_changed(
        &scalar.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-scalar-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nterrane-int-support = {{ path = \"../terrane-int-support\" }}\n", crate::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &scalar.join("src/lib.rs"),
        include_bytes!("../../terrane-scalar-support/src/lib.rs"),
    )?;
    write_if_changed(
        &string.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-string-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\n# Unicode 16.0 profile: case folding, normalization, and segmentation move together.\ncaseless = \"=0.2.2\"\n# unicode-normalization permits tinyvec 1.13, whose alloc-only build fails on Rust 1.93.\ntinyvec = {{ version = \"=1.12.0\", features = [\"std\"] }}\nunicode-normalization = \"=0.1.24\"\nunicode-segmentation = \"=1.12.0\"\n", crate::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &string.join("src/lib.rs"),
        include_bytes!("../../terrane-string-support/src/lib.rs"),
    )?;
    write_if_changed(
        &document.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-document-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nserde = \"1\"\nserde_json = {{ version = \"1\", features = [\"arbitrary_precision\", \"unbounded_depth\"] }}\nurl = \"2.5\"\nyaml-rust2 = \"=0.10.4\"\n", crate::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &document.join("src/lib.rs"),
        include_bytes!("../../terrane-document-support/src/lib.rs"),
    )?;
    write_if_changed(
        &stream.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-stream-abi\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nlibc = \"0.2\"\nrustix = {{ version = \"1\", features = [\"fs\"] }}\n", crate::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &stream.join("src/lib.rs"),
        include_bytes!("../../terrane-stream-abi/src/lib.rs"),
    )?;
    if uses_platform_support {
        crate::write_platform_support(&directory.join("support"))?;
    }
    Ok(())
}

/// Prepare a native Cargo process using the same fetch, containment and toolchain
/// policy for ordinary artifacts and host-side consumer executables.
///
/// # Errors
///
/// Returns dependency-fetch or generated workspace preparation failures.
pub fn prepare_cargo_command(
    crate_dir: &Path,
    target_dir: &Path,
    has_rust_dependencies: bool,
    containment: crate::rust_interop::projection::Containment,
) -> Result<Command, NativeBuildError> {
    let contained = has_rust_dependencies
        && containment == crate::rust_interop::projection::Containment::Enforced;
    if has_rust_dependencies {
        let mut fetch = Command::new("cargo");
        crate::rust_interop::configure_cargo_command(&mut fetch);
        configure_generated_toolchain(&mut fetch, crate_dir);
        let fetched = fetch
            .args(["fetch", "--manifest-path"])
            .arg(crate_dir.join("Cargo.toml"))
            .current_dir(crate_dir)
            .output()
            .map_err(|error| {
                NativeBuildError::backend(format!(
                    "failed to fetch generated dependencies: {error}"
                ))
            })?;
        if !fetched.status.success() {
            return Err(NativeBuildError::backend(format!(
                "Cargo dependency fetch failed: {}",
                String::from_utf8_lossy(&fetched.stderr).trim()
            )));
        }
        if !contained {
            eprintln!(
                "warning: generated dependency build containment is unavailable; using declared host tier"
            );
        }
    }
    fs::create_dir_all(target_dir).map_err(|error| {
        NativeBuildError::backend(format!("cannot create Cargo target directory: {error}"))
    })?;
    let root = crate_dir.canonicalize().map_err(|error| {
        NativeBuildError::backend(format!("cannot canonicalize generated crate: {error}"))
    })?;
    let target = target_dir.canonicalize().map_err(|error| {
        NativeBuildError::backend(format!(
            "cannot canonicalize Cargo target directory: {error}"
        ))
    })?;
    let mut cargo = if contained {
        let mut command = Command::new("bwrap");
        command
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
                "--tmpfs",
                "/tmp",
                "--bind",
            ])
            .arg(&root)
            .arg(&root)
            .arg("--bind")
            .arg(&target)
            .arg(&target)
            .args(["--", "cargo"]);
        command
    } else {
        Command::new("cargo")
    };
    crate::rust_interop::configure_cargo_command(&mut cargo);
    configure_generated_toolchain(&mut cargo, crate_dir);
    cargo.current_dir(&root).env("CARGO_TARGET_DIR", target);
    Ok(cargo)
}

fn configure_generated_toolchain(command: &mut Command, crate_dir: &Path) {
    let Ok(metadata) = fs::read_to_string(crate_dir.join("terrane-build.toml")) else {
        return;
    };
    let Some(toolchain) = metadata.lines().find_map(|line| {
        line.strip_prefix("rust-toolchain = ")
            .map(|value| value.trim_matches('"'))
    }) else {
        return;
    };
    if toolchain != "system" {
        command.arg(format!("+{toolchain}"));
    }
}
