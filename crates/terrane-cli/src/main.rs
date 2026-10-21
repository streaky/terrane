mod build_progress;
mod census_command;
mod debug_command;
mod development_fingerprint;
mod development_provenance;
mod profile_command;
mod test_command;

use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::io::{BufRead as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use terrane_compiler::native_build::{
    GeneratedCrateOptions, GeneratedCrateRole, UnsafeCodePolicy, write_generated_crate,
};

impl From<terrane_compiler::native_build::NativeBuildError> for CliFailure {
    fn from(error: terrane_compiler::native_build::NativeBuildError) -> Self {
        Self::backend(error.to_string())
    }
}

#[derive(Debug)]
struct CliFailure {
    code: u8,
    message: String,
}

impl CliFailure {
    fn usage() -> Self {
        Self {
            code: 2,
            message: usage(),
        }
    }

    fn usage_with(message: impl AsRef<str>) -> Self {
        Self {
            code: 2,
            message: format!("error: {}\n\n{}", message.as_ref(), usage()),
        }
    }

    fn diagnostic(path: PathBuf, code: &'static str, message: String, exit_code: u8) -> Self {
        let source = terrane_compiler::SourceFile::new(0, path, String::new());
        let diagnostic = terrane_compiler::Diagnostic::unlocated_error(code, message);
        Self {
            code: exit_code,
            message: diagnostic.render(&source),
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "map_err transfers ownership of the compiler failure directly into this renderer"
    )]
    fn compilation(failure: terrane_compiler::CompilationFailure) -> Self {
        Self {
            code: 3,
            message: failure
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.render(&failure.source))
                .collect(),
        }
    }

    fn rust_artifact(error: terrane_compiler::RustArtifactError) -> Self {
        match error {
            terrane_compiler::RustArtifactError::InvalidOutputPath(message) => {
                Self::backend(message)
            }
            terrane_compiler::RustArtifactError::Compilation(failure) => Self::compilation(failure),
        }
    }

    fn backend(message: String) -> Self {
        Self::diagnostic(PathBuf::from("<generated Rust>"), "S9002", message, 5)
    }

    fn package(message: impl AsRef<str>) -> Self {
        Self {
            code: 3,
            message: format!("error: {}\n", message.as_ref()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CliCommand {
    Check,
    Rust,
    Build,
    Run,
    Debug,
    Profile,
    Test,
    DeclarationInterface,
    DebugAdapter,
    Tooling,
    Query,
    Format,
    Package,
    ProjectionCensus,
    Toolchains,
    Help,
    Version,
}

impl CliCommand {
    fn parse(argument: &OsString) -> Option<Self> {
        match argument.to_str()? {
            "check" => Some(Self::Check),
            "rust" => Some(Self::Rust),
            "build" => Some(Self::Build),
            "test" => Some(Self::Test),
            "declaration-interface" => Some(Self::DeclarationInterface),
            "run" => Some(Self::Run),
            "debug" => Some(Self::Debug),
            "profile" => Some(Self::Profile),
            "debug-adapter" => Some(Self::DebugAdapter),
            "tooling" => Some(Self::Tooling),
            "query" => Some(Self::Query),
            "fmt" => Some(Self::Format),
            "package" => Some(Self::Package),
            "projection-census" => Some(Self::ProjectionCensus),
            "toolchains" => Some(Self::Toolchains),
            "--help" | "-h" => Some(Self::Help),
            "--version" | "-V" => Some(Self::Version),
            _ => None,
        }
    }
}

fn main() -> ExitCode {
    if let Some(warning) = development_provenance::warning() {
        eprint!("{warning}");
    }
    match run(&std::env::args_os().skip(1).collect::<Vec<_>>()) {
        Ok(code) => code,
        Err(failure) => {
            eprint!("{}", failure.message);
            ExitCode::from(failure.code)
        }
    }
}

fn implicit_run_arguments(arguments: &[OsString]) -> Option<Vec<OsString>> {
    let first = arguments.first()?;
    if CliCommand::parse(first).is_some() {
        return None;
    }
    let path = Path::new(first);
    if !path.is_file() && path.extension().is_none_or(|extension| extension != "trn") {
        return None;
    }
    let mut normalized = Vec::with_capacity(arguments.len() + 1);
    normalized.push(OsString::from("run"));
    normalized.push(first.clone());
    if arguments.len() > 1 {
        normalized.push(OsString::from("--"));
        let remaining = if arguments.get(1).is_some_and(|argument| argument == "--") {
            &arguments[2..]
        } else {
            &arguments[1..]
        };
        normalized.extend_from_slice(remaining);
    }
    Some(normalized)
}

#[expect(
    clippy::too_many_lines,
    reason = "the shared CLI pipeline keeps command phase ordering explicit"
)]
fn run(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let normalized = implicit_run_arguments(arguments);
    let arguments = normalized.as_deref().unwrap_or(arguments);
    let Some(command) = arguments.first().and_then(CliCommand::parse) else {
        return Err(CliFailure::usage());
    };
    match command {
        CliCommand::Version => {
            println!(
                "terrane {} (build rust {}, projection rustdoc {})",
                terrane_compiler::VERSION,
                terrane_compiler::BUILD_TOOLCHAIN,
                terrane_compiler::RUSTDOC_TOOLCHAIN
            );
            return Ok(ExitCode::SUCCESS);
        }
        CliCommand::Toolchains => {
            report_toolchains();
            return Ok(ExitCode::SUCCESS);
        }
        CliCommand::Help => {
            println!("{}", usage());
            return Ok(ExitCode::SUCCESS);
        }
        CliCommand::DeclarationInterface => return run_declaration_interface(arguments),
        CliCommand::Tooling => return run_tooling(arguments),
        CliCommand::Query => return run_query(arguments),
        CliCommand::Format => return run_format(arguments),
        CliCommand::Package => return run_package(arguments),
        CliCommand::ProjectionCensus => return census_command::run(arguments),
        CliCommand::Test => return test_command::run_tests(arguments),
        CliCommand::DebugAdapter => return debug_command::adapter::run_adapter(arguments),
        CliCommand::Profile
            if arguments.get(1).and_then(|argument| argument.to_str()) == Some("show") =>
        {
            return profile_command::show(arguments);
        }
        CliCommand::Check
        | CliCommand::Rust
        | CliCommand::Build
        | CliCommand::Run
        | CliCommand::Debug
        | CliCommand::Profile => {}
    }

    let profile_options = (command == CliCommand::Profile)
        .then(|| profile_command::parse_record(arguments))
        .transpose()?;
    let (
        input_path,
        output_path,
        require_canonical_rust,
        lint_name_style,
        lint_unused_functions,
        release,
        embed_debug_sources,
        embed_generated_sources,
    ) = if let Some(options) = &profile_options {
        (
            options.input.clone(),
            None,
            false,
            false,
            false,
            false,
            options.embed_sources,
            false,
        )
    } else {
        parse_input(arguments, command)?
    };
    let mut progress =
        (command == CliCommand::Build).then(|| build_progress::BuildProgress::new(&input_path));
    let source_input = !input_path.is_dir()
        && input_path
            .extension()
            .is_none_or(|extension| extension != "toml");
    let package = if source_input {
        let source_text = fs::read_to_string(&input_path).map_err(|error| {
            CliFailure::diagnostic(input_path.clone(), "S0000", error.to_string(), 3)
        })?;
        terrane_compiler::Package::implicit(&input_path, source_text)
    } else {
        terrane_compiler::Package::load(&input_path).map_err(|errors| CliFailure {
            code: 3,
            message: errors
                .into_iter()
                .map(|error| error.diagnostic.render(&error.source))
                .collect(),
        })?
    };
    if let Some(progress) = &mut progress {
        progress.identify(&package.identity);
    }
    if package.artifact == terrane_compiler::ArtifactKind::DynamicLibrary
        && matches!(
            command,
            CliCommand::Run | CliCommand::Debug | CliCommand::Profile
        )
    {
        return Err(CliFailure::usage_with(
            "`run`, `debug`, and `profile record` require an executable package; use `build` for a dynamic library",
        ));
    }
    if package.artifact == terrane_compiler::ArtifactKind::Library
        && !matches!(command, CliCommand::Check | CliCommand::Rust)
    {
        return Err(CliFailure::usage_with(
            "Terrane library packages are lowered with an application; use `check` or `rust` directly",
        ));
    }
    if let Some(progress) = &mut progress {
        progress.advance("compiling Terrane sources");
    }
    let compile = || {
        terrane_compiler::compile_package_with_options(
            &package,
            terrane_compiler::CompilerOptions {
                require_canonical_rust,
                lint_name_style,
                lint_unused_functions,
                debug_build: match command {
                    CliCommand::Profile if embed_debug_sources => {
                        terrane_compiler::DebugBuild::EmbeddedAllSources
                    }
                    CliCommand::Profile => terrane_compiler::DebugBuild::EmbeddedGeneratedSources,
                    CliCommand::Debug => match (embed_debug_sources, embed_generated_sources) {
                        (false, false) => terrane_compiler::DebugBuild::ExternalSources,
                        (true, false) => terrane_compiler::DebugBuild::EmbeddedSources,
                        (false, true) => terrane_compiler::DebugBuild::EmbeddedGeneratedSources,
                        (true, true) => terrane_compiler::DebugBuild::EmbeddedAllSources,
                    },
                    _ => terrane_compiler::DebugBuild::Disabled,
                },
            },
        )
    };
    let compilation = match terrane_compiler::with_compilation_cache(|| {
        if command == CliCommand::Build {
            terrane_compiler::with_compilation_progress(compile)
        } else {
            compile()
        }
    }) {
        Ok(compilation) => compilation,
        Err(failure) => return Err(CliFailure::compilation(failure)),
    };
    emit_warnings(&compilation);
    if command == CliCommand::Rust && output_path.is_none() {
        print_rust(&compilation);
        return Ok(ExitCode::SUCCESS);
    }
    if let Some(progress) = &mut progress {
        progress.advance("preparing native build");
    }
    let rust_entrypoint = output_path.as_deref().unwrap_or_else(|| {
        Path::new(match package.artifact {
            terrane_compiler::ArtifactKind::Executable => "src/main.rs",
            terrane_compiler::ArtifactKind::DynamicLibrary
            | terrane_compiler::ArtifactKind::Library => "src/lib.rs",
        })
    });
    let rust_files = compilation
        .rust_files_for(rust_entrypoint)
        .map_err(CliFailure::rust_artifact)?;
    if command == CliCommand::Rust {
        write_rust(rust_entrypoint, &rust_files)?;
        return Ok(ExitCode::SUCCESS);
    }
    ensure_rust_toolchain(package.build_toolchain)?;
    let uses_platform_support = compilation.requires_platform_support;
    let uses_async_runtime = compilation.requires_async_runtime;
    let uses_tokio_blocking = compilation.requires_blocking_runtime;
    let uses_tokio_sync = compilation.requires_runtime_sync;
    let role = if matches!(command, CliCommand::Debug | CliCommand::Profile) {
        GeneratedCrateRole::Inspection
    } else {
        GeneratedCrateRole::Application
    };
    let crate_dir = generated_crate_path(&package.root, role)?;
    let artifact_profile = match command {
        CliCommand::Debug => Some(profile_with_panic(
            terrane_compiler::debugging::DEBUG_ARTIFACT_PROFILE,
            package.profile.panic,
        )),
        CliCommand::Profile => Some(profile_with_panic(
            terrane_compiler::profiling::CPU_ARTIFACT_PROFILE,
            package.profile.panic,
        )),
        _ => None,
    };
    write_generated_crate(
        &crate_dir,
        &rust_files,
        &package.units,
        &compilation.rust_dependencies,
        GeneratedCrateOptions {
            role,
            panic: package.profile.panic,
            uses_platform_support,
            uses_async_runtime,
            uses_tokio_blocking,
            uses_tokio_sync,
            build_toolchain: package.build_toolchain,
            unsafe_code: if compilation.requires_unsafe_code
                || !package.authored_rust_modules.is_empty()
            {
                UnsafeCodePolicy::MaintainedModules
            } else {
                UnsafeCodePolicy::Forbid
            },
            artifact: package.artifact,
        },
    )?;
    let target_dir = package.root.join(".trn/cache/target");
    if let Some(progress) = &mut progress {
        progress.advance("compiling native artifact");
    }
    let artifact = prepare_artifact(
        command,
        &crate_dir,
        role,
        &target_dir,
        &rust_files,
        &package.units,
        !compilation.rust_dependencies.is_empty(),
        compilation.dependency_containment,
        package.artifact,
        match command {
            CliCommand::Profile => CargoProfile::Profiling,
            CliCommand::Debug => CargoProfile::Debugger,
            _ if release => CargoProfile::Release,
            _ => CargoProfile::Debug,
        },
        compilation.native_dependency_identity(),
    )?;
    if command == CliCommand::Check {
        return Ok(ExitCode::SUCCESS);
    }
    let artifact = artifact.expect("build, run, debug, and profile prepare a native artifact");
    if command == CliCommand::Build {
        if let Some(progress) = progress {
            progress.finish();
        }
        println!("{}", artifact.display());
        return Ok(ExitCode::SUCCESS);
    }
    let executable = artifact;
    if command == CliCommand::Profile {
        let options = profile_options
            .as_ref()
            .expect("profile record options were parsed before compilation");
        let debug = compilation
            .debug_information(rust_entrypoint)
            .map_err(CliFailure::rust_artifact)?
            .expect("profile compilation produces source-attribution metadata");
        let build_identity = rust_build_identity(
            &crate_dir,
            artifact_profile.expect("profile build has an artifact profile"),
            "profiling",
        )?;
        return profile_command::record(
            options,
            &package,
            debug,
            &executable,
            &crate_dir,
            build_identity,
        );
    }
    if command == CliCommand::Debug {
        let debug = compilation
            .debug_information(rust_entrypoint)
            .map_err(CliFailure::rust_artifact)?
            .expect("debug compilation produces debugger metadata");
        let build_identity = rust_build_identity(
            &crate_dir,
            artifact_profile.expect("debug build has an artifact profile"),
            "debug",
        )?;
        let provenance = terrane_compiler::debugging::ProvenanceManifest::create(
            &package,
            debug,
            &executable,
            &crate_dir,
            build_identity,
        )
        .map_err(CliFailure::backend)?;
        let sidecar = debug_command::write_provenance(&crate_dir, &executable, &provenance)?;
        let separator = arguments.iter().position(|argument| argument == "--");
        let program_arguments = separator.map_or(&[][..], |index| &arguments[index + 1..]);
        return debug_command::run_cli(&executable, &sidecar, program_arguments);
    }
    let separator = arguments.iter().position(|argument| argument == "--");
    let program_arguments = separator.map_or(&[][..], |index| &arguments[index + 1..]);
    let status = Command::new(executable)
        .args(program_arguments)
        .status()
        .map_err(|error| {
            CliFailure::backend(format!("failed to run generated program: {error}"))
        })?;
    Ok(ExitCode::from(
        u8::try_from(status.code().unwrap_or(1)).unwrap_or(1),
    ))
}

fn run_declaration_interface(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    if arguments.len() != 4 || arguments[2] != "--output" {
        return Err(CliFailure::usage_with(
            "usage: terrane declaration-interface <package-or-source> --output <interface.json>",
        ));
    }
    let input = PathBuf::from(&arguments[1]);
    let package = if input.is_dir()
        || input
            .extension()
            .is_some_and(|extension| extension == "toml")
    {
        terrane_compiler::Package::load(&input).map_err(|errors| CliFailure {
            code: 3,
            message: errors
                .into_iter()
                .map(|error| error.diagnostic.render(&error.source))
                .collect(),
        })?
    } else {
        let source = fs::read_to_string(&input).map_err(|error| {
            CliFailure::diagnostic(input.clone(), "S0000", error.to_string(), 3)
        })?;
        terrane_compiler::Package::implicit(&input, source)
    };
    let interface = terrane_compiler::with_compilation_cache(|| {
        terrane_compiler::declaration_interface(&package)
    })
    .map_err(CliFailure::compilation)?;
    let json = interface.to_json().map_err(CliFailure::package)?;
    let output = PathBuf::from(&arguments[3]);
    fs::write(&output, json).map_err(|error| {
        CliFailure::package(format!(
            "cannot write declaration interface `{}`: {error}",
            output.display()
        ))
    })?;
    Ok(ExitCode::SUCCESS)
}

fn rust_build_identity(
    crate_dir: &Path,
    artifact_profile: terrane_compiler::provenance::ArtifactProfile,
    purpose: &str,
) -> Result<terrane_compiler::provenance::BuildIdentity, CliFailure> {
    let verbose = Command::new("rustc")
        .arg("-vV")
        .current_dir(crate_dir)
        .output()
        .map_err(|error| {
            CliFailure::backend(format!(
                "failed to inspect {purpose} Rust compiler: {error}"
            ))
        })?;
    if !verbose.status.success() {
        return Err(CliFailure::backend(format!(
            "{purpose} Rust compiler did not report its target triple"
        )));
    }
    let rustc_release = String::from_utf8(verbose.stdout).map_err(|_| {
        CliFailure::backend(format!(
            "{purpose} Rust compiler output was not valid UTF-8"
        ))
    })?;
    let target = rustc_release
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .ok_or_else(|| {
            CliFailure::backend(format!(
                "{purpose} Rust compiler output omitted its target triple"
            ))
        })?
        .to_owned();
    let sysroot = Command::new("rustc")
        .args(["--print", "sysroot"])
        .current_dir(crate_dir)
        .output()
        .map_err(|error| {
            CliFailure::backend(format!("failed to inspect {purpose} Rust sysroot: {error}"))
        })?;
    if !sysroot.status.success() {
        return Err(CliFailure::backend(format!(
            "{purpose} Rust compiler did not report its sysroot"
        )));
    }
    let sysroot = String::from_utf8(sysroot.stdout)
        .map_err(|_| CliFailure::backend(format!("{purpose} Rust sysroot was not valid UTF-8")))?
        .trim()
        .to_owned();
    Ok(terrane_compiler::provenance::BuildIdentity {
        target,
        rust_sysroot: sysroot,
        rustc_release,
        artifact_profile,
    })
}

type ParsedInput = (PathBuf, Option<PathBuf>, bool, bool, bool, bool, bool, bool);

fn parse_input(arguments: &[OsString], command: CliCommand) -> Result<ParsedInput, CliFailure> {
    let mut input_index = 1;
    let mut output_path = None;
    let mut require_canonical_rust = false;
    let mut lint_name_style = false;
    let mut lint_unused_functions = false;
    let mut release = false;
    let mut embed_debug_sources = false;
    let mut embed_generated_sources = false;
    while let Some(argument) = arguments.get(input_index).and_then(|value| value.to_str()) {
        match argument {
            "--require-canonical-rust" => require_canonical_rust = true,
            "--lint-name-style" => lint_name_style = true,
            "--lint-unused-functions" => lint_unused_functions = true,
            "--release" => release = true,
            "--embed-sources" if command == CliCommand::Debug => embed_debug_sources = true,
            "--embed-generated-sources" if command == CliCommand::Debug => {
                embed_generated_sources = true;
            }
            "-o" | "--output" if command == CliCommand::Rust && output_path.is_none() => {
                input_index += 1;
                output_path = Some(
                    arguments
                        .get(input_index)
                        .map(PathBuf::from)
                        .ok_or_else(CliFailure::usage)?,
                );
            }
            _ => break,
        }
        input_index += 1;
    }
    if release && command == CliCommand::Debug {
        return Err(CliFailure::usage_with(
            "`terrane debug --release` is unsupported: debugger source fidelity requires the compiler-owned unoptimized debug profile",
        ));
    }
    if release && !matches!(command, CliCommand::Build | CliCommand::Run) {
        return Err(CliFailure::usage());
    }
    if matches!(command, CliCommand::Run | CliCommand::Debug)
        && arguments.len() >= input_index + 2
        && arguments[input_index + 1] != "--"
    {
        return Err(CliFailure::usage_with(
            "program arguments must follow `--` (for example, `terrane run package.toml -- arg`)",
        ));
    }
    let has_valid_arity = if matches!(command, CliCommand::Run | CliCommand::Debug) {
        arguments.len() == input_index + 1
            || (arguments.len() >= input_index + 2 && arguments[input_index + 1] == "--")
    } else {
        arguments.len() == input_index + 1
    };
    if !has_valid_arity {
        return Err(CliFailure::usage());
    }
    let input_path = arguments
        .get(input_index)
        .map(PathBuf::from)
        .ok_or_else(CliFailure::usage)?;
    Ok((
        input_path,
        output_path,
        require_canonical_rust,
        lint_name_style,
        lint_unused_functions,
        release,
        embed_debug_sources,
        embed_generated_sources,
    ))
}

fn emit_warnings(compilation: &terrane_compiler::Compilation) {
    for warning in &compilation.warnings {
        let source = warning
            .primary
            .and_then(|span| {
                compilation
                    .sources
                    .iter()
                    .find(|source| source.id() == span.file)
            })
            .unwrap_or(&compilation.source);
        eprint!("{}", warning.render(source));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CargoProfile {
    Debug,
    Debugger,
    Release,
    Profiling,
}

impl CargoProfile {
    const fn directory(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Debugger => "terrane-debug",
            Self::Release => "release",
            Self::Profiling => "terrane-profile",
        }
    }

    fn configure(self, command: &mut Command) {
        match self {
            Self::Debug => {}
            Self::Debugger => {
                command.args(["--profile", "terrane-debug"]);
            }
            Self::Release => {
                command.arg("--release");
            }
            Self::Profiling => {
                command.args(["--profile", "terrane-profile"]);
            }
        }
    }
}

fn artifact_identity(
    crate_dir: &Path,
    containment: terrane_compiler::rust_interop::projection::Containment,
    artifact_kind: terrane_compiler::ArtifactKind,
    dependency_identity: Option<&str>,
) -> Result<String, CliFailure> {
    fn hash_path(path: &Path, hash: &mut Sha256) -> std::io::Result<()> {
        if !path.exists() {
            hash.update(b"absent\0");
        } else if path.is_dir() {
            let mut entries = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
            entries.sort_by_key(std::fs::DirEntry::file_name);
            for entry in entries {
                hash.update(entry.file_name().as_encoded_bytes());
                hash.update(b"\0");
                hash_path(&entry.path(), hash)?;
            }
        } else {
            let bytes = fs::read(path)?;
            hash.update(bytes.len().to_le_bytes());
            hash.update(&bytes);
        }
        Ok(())
    }
    let mut hash = Sha256::new();
    hash.update(b"terrane-native-artifact-v5\0");
    hash.update(terrane_compiler::VERSION.as_bytes());
    if let Some(identity) = dependency_identity {
        hash.update(b"native-dependency-inputs\0");
        hash.update(identity.as_bytes());
        hash.update(b"\0");
    }
    hash.update(format!("{containment:?}/{artifact_kind:?}"));
    if coverage_directory().is_some() {
        hash.update(b"rust-implementation-coverage-v4\0");
    }
    if artifact_kind == terrane_compiler::ArtifactKind::DynamicLibrary {
        hash.update(b"role-owned-library-basename-v1\0");
    }
    for variable in [
        "CARGO_BUILD_TARGET",
        "CARGO_ENCODED_RUSTFLAGS",
        "RUSTC",
        "RUSTFLAGS",
        "TERRANE_SCCACHE",
    ] {
        hash.update(variable.as_bytes());
        hash.update(b"\0");
        hash.update(
            std::env::var_os(variable)
                .unwrap_or_default()
                .as_encoded_bytes(),
        );
        hash.update(b"\0");
    }
    for relative in [
        "Cargo.toml",
        "Cargo.lock",
        "rust-toolchain.toml",
        "terrane-build.toml",
        "src",
        "support",
        ".cargo",
    ] {
        hash.update(relative.as_bytes());
        hash_path(&crate_dir.join(relative), &mut hash).map_err(|error| {
            CliFailure::backend(format!("cannot fingerprint generated crate: {error}"))
        })?;
    }
    let compiler = Command::new("rustc")
        .arg("-vV")
        .current_dir(crate_dir)
        .output()
        .map_err(|error| CliFailure::backend(format!("cannot inspect native compiler: {error}")))?;
    if !compiler.status.success() {
        return Err(CliFailure::backend(
            "cannot inspect native compiler version".to_owned(),
        ));
    }
    hash.update(&compiler.stdout);
    Ok(format!("{:x}", hash.finalize()))
}

#[expect(
    clippy::too_many_arguments,
    reason = "artifact preparation forwards one complete Cargo build context without hidden state"
)]
fn prepare_artifact(
    command: CliCommand,
    crate_dir: &Path,
    role: GeneratedCrateRole,
    target_dir: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    units: &[terrane_compiler::SourceUnit],
    has_rust_dependencies: bool,
    containment: terrane_compiler::rust_interop::projection::Containment,
    artifact_kind: terrane_compiler::ArtifactKind,
    profile: CargoProfile,
    dependency_identity: Option<&str>,
) -> Result<Option<PathBuf>, CliFailure> {
    let identity = artifact_identity(crate_dir, containment, artifact_kind, dependency_identity)?;
    if fs::read_to_string(crate_dir.join(".artifact-identity"))
        .ok()
        .as_deref()
        != Some(&identity)
    {
        let artifacts = crate_dir.join("artifacts");
        if artifacts.exists() {
            fs::remove_dir_all(&artifacts).map_err(|error| {
                CliFailure::backend(format!("cannot invalidate cached artifacts: {error}"))
            })?;
        }
        fs::write(crate_dir.join(".artifact-identity"), &identity).map_err(|error| {
            CliFailure::backend(format!("cannot record artifact identity: {error}"))
        })?;
    }
    if command == CliCommand::Check {
        let stamp = crate_dir.join("artifacts/check-success");
        if !stamp.is_file() {
            run_cargo(
                "check",
                crate_dir,
                target_dir,
                rust_files,
                units,
                has_rust_dependencies,
                containment,
                CargoProfile::Debug,
            )?;
            fs::write(
                crate_dir.join(".artifact-identity"),
                artifact_identity(crate_dir, containment, artifact_kind, dependency_identity)?,
            )
            .map_err(|error| {
                CliFailure::backend(format!("cannot record artifact identity: {error}"))
            })?;
            fs::create_dir_all(stamp.parent().expect("artifact stamp has a parent")).map_err(
                |error| CliFailure::backend(format!("cannot create artifact cache: {error}")),
            )?;
            fs::write(&stamp, []).map_err(|error| {
                CliFailure::backend(format!("cannot record checked artifact: {error}"))
            })?;
        }
        return Ok(None);
    }
    let profile_directory = profile.directory();
    let artifact = artifact_path(
        &crate_dir.join("artifacts").join(profile_directory),
        artifact_kind,
        if artifact_kind == terrane_compiler::ArtifactKind::DynamicLibrary {
            role.cargo_name()
        } else {
            "terrane_program"
        },
    );
    if !artifact.is_file() {
        run_cargo(
            "build",
            crate_dir,
            target_dir,
            rust_files,
            units,
            has_rust_dependencies,
            containment,
            profile,
        )?;
        let built = artifact_path(
            &target_dir.join(profile_directory),
            artifact_kind,
            role.cargo_name(),
        );
        fs::create_dir_all(artifact.parent().expect("cached artifact has a parent")).map_err(
            |error| CliFailure::backend(format!("cannot create artifact cache: {error}")),
        )?;
        fs::copy(&built, &artifact).map_err(|error| {
            CliFailure::backend(format!("cannot cache built artifact: {error}"))
        })?;
        #[cfg(all(target_os = "windows", target_env = "msvc"))]
        if artifact_kind == terrane_compiler::ArtifactKind::DynamicLibrary {
            cache_import_library(&built, &artifact)?;
        }
        fs::write(
            crate_dir.join(".artifact-identity"),
            artifact_identity(crate_dir, containment, artifact_kind, dependency_identity)?,
        )
        .map_err(|error| {
            CliFailure::backend(format!("cannot record artifact identity: {error}"))
        })?;
    }
    if artifact_kind == terrane_compiler::ArtifactKind::Executable
        && profile != CargoProfile::Debugger
        && let Some(directory) = coverage_directory()
    {
        preserve_coverage_object(&directory, &artifact)?;
    }
    artifact
        .canonicalize()
        .map(Some)
        .map_err(|error| CliFailure::backend(format!("cannot locate built artifact: {error}")))
}

fn executable_path(directory: &Path, name: &str) -> PathBuf {
    let mut path = directory.join(name);
    path.set_extension(std::env::consts::EXE_EXTENSION);
    path
}

fn artifact_path(
    directory: &Path,
    artifact_kind: terrane_compiler::ArtifactKind,
    name: &str,
) -> PathBuf {
    match artifact_kind {
        terrane_compiler::ArtifactKind::Executable => executable_path(directory, name),
        terrane_compiler::ArtifactKind::DynamicLibrary => directory.join(format!(
            "{}{name}{}",
            std::env::consts::DLL_PREFIX,
            std::env::consts::DLL_SUFFIX
        )),
        terrane_compiler::ArtifactKind::Library => {
            unreachable!("library packages are never prepared as standalone native artifacts")
        }
    }
}

fn coverage_directory() -> Option<PathBuf> {
    std::env::var_os("TERRANE_COVERAGE_DIR").map(PathBuf::from)
}

fn coverage_rustflags(
    mut flags: std::ffi::OsString,
    crate_dir: &Path,
    profile: CargoProfile,
) -> std::ffi::OsString {
    if coverage_directory().is_some() {
        // Debuggees retain the caller's exact compiler flags; the wrapper
        // instruments only its outer `cargo test --no-run` build.
        if profile == CargoProfile::Debugger {
            return flags;
        }
        flags.push(" -C instrument-coverage");
        // Distinguish independently built support graphs whose MIR/coverage
        // hashes can differ despite identical Cargo crate disambiguators.
        let identity = Sha256::digest(crate_dir.as_os_str().as_encoded_bytes());
        flags.push(format!(" -C metadata=terrane-coverage-{identity:x}"));
        flags.push(" --remap-path-prefix=");
        flags.push(crate_dir.join("support"));
        flags.push("=");
        flags.push(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap());
    }
    flags
}

fn preserve_coverage_object(directory: &Path, source: &Path) -> Result<(), CliFailure> {
    use std::io::Read as _;
    let mut input = fs::File::open(source).map_err(|error| {
        CliFailure::backend(format!("cannot open coverage executable: {error}"))
    })?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 8 * 1024];
    loop {
        let count = input.read(&mut buffer).map_err(|error| {
            CliFailure::backend(format!("cannot hash coverage executable: {error}"))
        })?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    let name = source
        .file_name()
        .expect("coverage executable has a filename");
    let destination =
        directory
            .join("objects")
            .join(format!("{:x}-{}", hash.finalize(), name.to_string_lossy()));
    fs::create_dir_all(destination.parent().expect("coverage object has a parent")).map_err(
        |error| CliFailure::backend(format!("cannot create coverage object directory: {error}")),
    )?;
    if !destination.exists() {
        fs::copy(source, destination).map_err(|error| {
            CliFailure::backend(format!("cannot preserve coverage executable: {error}"))
        })?;
    }
    Ok(())
}

#[cfg(any(test, all(target_os = "windows", target_env = "msvc")))]
fn cache_import_library(built: &Path, cached: &Path) -> Result<(), CliFailure> {
    let built = built.with_extension("dll.lib");
    let cached = cached.with_extension("dll.lib");
    fs::copy(&built, &cached).map_err(|error| {
        CliFailure::backend(format!(
            "cannot cache dynamic-library import library: {error}"
        ))
    })?;
    Ok(())
}

fn print_rust(compilation: &terrane_compiler::Compilation) {
    print!("{}", compilation.rust);
    println!("// Generated Rust form: standalone");
    println!(
        "// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi"
    );
}

fn write_rust(
    entrypoint: &Path,
    files: &[terrane_compiler::rust_ir::RenderedFile],
) -> Result<(), CliFailure> {
    for file in files {
        let path = Path::new(&file.path);
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(|error| {
                CliFailure::backend(format!(
                    "cannot create generated Rust output directory {}: {error}",
                    parent.display()
                ))
            })?;
        }
        fs::write(path, &file.contents).map_err(|error| {
            CliFailure::backend(format!(
                "cannot write generated Rust output {}: {error}",
                path.display()
            ))
        })?;
    }
    if !files.iter().any(|file| Path::new(&file.path) == entrypoint) {
        return Ok(());
    }
    let (Some(parent), Some(stem)) = (entrypoint.parent(), entrypoint.file_stem()) else {
        return Ok(());
    };
    let sidecar = parent.join(format!("{}.support.rs", stem.to_string_lossy()));
    if files.iter().any(|file| Path::new(&file.path) == sidecar) {
        return Ok(());
    }
    if sidecar.is_file() {
        fs::remove_file(&sidecar).map_err(|error| {
            CliFailure::backend(format!(
                "cannot remove obsolete generated Rust output {}: {error}",
                sidecar.display()
            ))
        })?;
    }
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    reason = "Cargo process policy, containment, diagnostics, and artifact caching form one operation"
)]
fn run_cargo(
    command: &str,
    crate_dir: &Path,
    target_dir: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    units: &[terrane_compiler::SourceUnit],
    has_rust_dependencies: bool,
    containment: terrane_compiler::rust_interop::projection::Containment,
    profile: CargoProfile,
) -> Result<(), CliFailure> {
    let rustflags = std::env::var_os("RUSTFLAGS").unwrap_or_default();
    let mut cargo = terrane_compiler::native_build::prepare_cargo_command(
        crate_dir,
        target_dir,
        has_rust_dependencies,
        containment,
    )?;
    cargo.args([
        command,
        "--quiet",
        "--message-format=json",
        "--manifest-path",
    ]);
    cargo.arg(crate_dir.join("Cargo.toml"));
    profile.configure(&mut cargo);
    if has_rust_dependencies {
        cargo.args(["--offline", "--frozen"]);
    }
    let flags = coverage_rustflags(
        rustflags,
        cargo
            .get_current_dir()
            .expect("prepared Cargo command owns a workspace"),
        profile,
    );
    let output = cargo
        .env("RUSTFLAGS", flags)
        .output()
        .map_err(|error| CliFailure::backend(format!("failed to start Cargo: {error}")))?;
    if output.status.success() {
        return Ok(());
    }
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if message["reason"] != "compiler-message" || message["message"]["level"] != "error" {
            continue;
        }
        let raw = message["message"]["rendered"]
            .as_str()
            .unwrap_or("rustc reported a generated-code error");
        for span in message["message"]["spans"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|span| span["is_primary"].as_bool().unwrap_or(false))
        {
            let Some(file_name) = span["file_name"].as_str() else {
                continue;
            };
            let Some(file) = rust_files
                .iter()
                .find(|file| Path::new(file_name).ends_with(&file.path))
            else {
                continue;
            };
            let Ok(byte_start) = usize::try_from(span["byte_start"].as_u64().unwrap_or(0)) else {
                continue;
            };
            let Some(association) = file.associations.iter().find(|association| {
                association.generated_start <= byte_start && byte_start < association.generated_end
            }) else {
                continue;
            };
            let Some(source) = units
                .iter()
                .find(|unit| unit.source.id() == association.source.file)
                .map(|unit| &unit.source)
            else {
                continue;
            };
            let diagnostic = terrane_compiler::Diagnostic::error(
                "S9003",
                "generated Rust failed backend validation",
                association.source,
            );
            return Err(CliFailure {
                code: 5,
                message: format!(
                    "{}note: raw rustc diagnostic:\n{raw}",
                    diagnostic.render(source)
                ),
            });
        }
        return Err(CliFailure::backend(format!(
            "Cargo {command} failed\nnote: raw rustc diagnostic:\n{raw}"
        )));
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(CliFailure::diagnostic(
        PathBuf::from("<toolchain>"),
        "S9001",
        format!("Cargo {command} failed: {}", stderr.trim()),
        4,
    ))
}

fn generated_crate_path(
    package_root: &Path,
    role: GeneratedCrateRole,
) -> Result<PathBuf, CliFailure> {
    let root = package_root.canonicalize().map_err(|error| {
        CliFailure::backend(format!(
            "cannot locate package root {}: {error}",
            package_root.display()
        ))
    })?;
    let build = root.join(".trn/build");
    if build.is_dir() {
        for entry in fs::read_dir(&build).map_err(|error| {
            CliFailure::backend(format!("cannot inspect generated builds: {error}"))
        })? {
            let entry = entry.map_err(|error| {
                CliFailure::backend(format!("cannot inspect generated build: {error}"))
            })?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.len() == 64
                && name.bytes().all(|byte| byte.is_ascii_hexdigit())
                && entry.file_type().is_ok_and(|kind| kind.is_dir())
                && entry.path().join("terrane-build.toml").is_file()
            {
                fs::remove_dir_all(entry.path()).map_err(|error| {
                    CliFailure::backend(format!("cannot remove obsolete generated build: {error}"))
                })?;
            }
        }
    }
    Ok(build.join(role.directory()))
}

fn profile_with_panic(
    mut profile: terrane_compiler::provenance::ArtifactProfile,
    panic: terrane_compiler::PanicProfile,
) -> terrane_compiler::provenance::ArtifactProfile {
    profile.panic = match panic {
        terrane_compiler::PanicProfile::Unwind => "unwind",
        terrane_compiler::PanicProfile::Abort => "abort",
    };
    profile
}

// Keep publication local to the CLI; sharing this tiny helper would expand the
// compiler's public API for an otherwise independent filesystem boundary.
fn write_if_changed(path: &Path, content: &[u8]) -> std::io::Result<()> {
    if fs::read(path).is_ok_and(|existing| existing == content) {
        return Ok(());
    }
    fs::write(path, content)
}
fn ensure_rust_toolchain(
    build_toolchain: terrane_compiler::BuildToolchain,
) -> Result<(), CliFailure> {
    let mut command = Command::new("cargo");
    if build_toolchain == terrane_compiler::BuildToolchain::Pinned {
        record_toolchain_pin(terrane_compiler::BUILD_TOOLCHAIN)?;
        command.arg(format!("+{}", terrane_compiler::BUILD_TOOLCHAIN));
    }
    let status = command.arg("--version").output().map_err(|error| {
        CliFailure::diagnostic(
            PathBuf::from("<toolchain>"),
            "S9001",
            format!(
                "Cargo with Rust {} is required to compile generated Rust: {error}",
                if build_toolchain == terrane_compiler::BuildToolchain::Pinned {
                    terrane_compiler::BUILD_TOOLCHAIN.to_owned()
                } else {
                    format!("{} or newer", terrane_compiler::BUILD_TOOLCHAIN)
                }
            ),
            4,
        )
    })?;
    if !status.status.success() {
        return Err(CliFailure::diagnostic(
            PathBuf::from("<toolchain>"),
            "S9001",
            format!(
                "Cargo prerequisite check failed; generated Rust requires Rust {}",
                terrane_compiler::BUILD_TOOLCHAIN
            ),
            4,
        ));
    }
    if build_toolchain == terrane_compiler::BuildToolchain::System {
        ensure_system_rust_version()?;
    }
    Ok(())
}

fn ensure_system_rust_version() -> Result<(), CliFailure> {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .map_err(|error| {
            CliFailure::diagnostic(
                PathBuf::from("<toolchain>"),
                "S9001",
                format!(
                    "the system-toolchain escape hatch requires Rust {} or newer: {error}",
                    terrane_compiler::BUILD_TOOLCHAIN
                ),
                4,
            )
        })?;
    let found = String::from_utf8_lossy(&output.stdout);
    let version = found.split_whitespace().nth(1).unwrap_or_default();
    let numeric = |text: &str| {
        text.split('.')
            .take(3)
            .map(|part| part.parse::<u32>().unwrap_or_default())
            .collect::<Vec<_>>()
    };
    if !output.status.success() || numeric(version) < numeric(terrane_compiler::BUILD_TOOLCHAIN) {
        return Err(CliFailure::diagnostic(
            PathBuf::from("<toolchain>"),
            "S9001",
            format!(
                "system Rust `{version}` is too old; generated Rust requires {} or newer",
                terrane_compiler::BUILD_TOOLCHAIN
            ),
            4,
        ));
    }
    Ok(())
}

fn toolchain_state_path() -> PathBuf {
    std::env::var_os("XDG_STATE_HOME")
        .map_or_else(
            || {
                std::env::var_os("HOME").map_or_else(
                    || PathBuf::from(".trn-state"),
                    |home| PathBuf::from(home).join(".local/state"),
                )
            },
            PathBuf::from,
        )
        .join("terrane/toolchains")
}

fn record_toolchain_pin(toolchain: &str) -> Result<(), CliFailure> {
    let path = toolchain_state_path();
    let mut pins = fs::read_to_string(&path)
        .unwrap_or_default()
        .lines()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    if !pins.insert(toolchain.to_owned()) {
        return Ok(());
    }
    fs::create_dir_all(path.parent().expect("toolchain state file has a parent"))
        .and_then(|()| {
            fs::write(
                &path,
                pins.into_iter().collect::<Vec<_>>().join("\n") + "\n",
            )
        })
        .map_err(|error| {
            CliFailure::backend(format!("cannot record requested Rust toolchain: {error}"))
        })
}

fn report_toolchains() {
    let pins = fs::read_to_string(toolchain_state_path()).unwrap_or_default();
    if pins.lines().next().is_none() {
        println!("Terrane has not requested any Rust toolchains.");
        return;
    }
    println!("Rust toolchains requested by Terrane:");
    for pin in pins.lines() {
        let use_note = if pin == terrane_compiler::BUILD_TOOLCHAIN {
            "used by this Terrane version"
        } else {
            "not used by this Terrane version"
        };
        println!("  {pin} ({use_note})");
    }
}

fn run_tooling(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    if arguments != [OsString::from("tooling"), OsString::from("--stdio")] {
        return Err(CliFailure::usage());
    }
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut output = stdout.lock();
    let mut session = ToolingSession::default();
    for line in stdin.lock().lines() {
        let line = line.map_err(|error| {
            CliFailure::diagnostic(
                PathBuf::from("<stdin>"),
                "S3001",
                format!("cannot read tooling request: {error}"),
                4,
            )
        })?;
        if line.trim().is_empty() {
            continue;
        }
        let response = match serde_json::from_str(&line) {
            Ok(value) => session.handle_value(value),
            Err(error) => protocol_parse_error(&error, None),
        };
        serde_json::to_writer(&mut output, &response).map_err(|error| {
            CliFailure::diagnostic(
                PathBuf::from("<stdout>"),
                "S3002",
                format!("cannot encode tooling response: {error}"),
                4,
            )
        })?;
        writeln!(output).map_err(|error| {
            CliFailure::diagnostic(
                PathBuf::from("<stdout>"),
                "S3003",
                format!("cannot write tooling response: {error}"),
                4,
            )
        })?;
        output.flush().map_err(|error| {
            CliFailure::diagnostic(
                PathBuf::from("<stdout>"),
                "S3003",
                format!("cannot flush tooling response: {error}"),
                4,
            )
        })?;
    }
    Ok(ExitCode::SUCCESS)
}

fn run_query(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let [_, flag, request_path] = arguments else {
        return Err(CliFailure::usage());
    };
    if flag != "--request" {
        return Err(CliFailure::usage());
    }
    let request_path = PathBuf::from(request_path);
    let request_text = fs::read_to_string(&request_path).map_err(|error| {
        CliFailure::diagnostic(request_path.clone(), "S3004", error.to_string(), 4)
    })?;
    let values = match serde_json::from_str::<serde_json::Value>(&request_text) {
        Ok(serde_json::Value::Array(values)) => values,
        Ok(value) => vec![value],
        Err(error) => {
            println!(
                "{}",
                serde_json::to_string(&protocol_parse_error(&error, None))
                    .expect("tooling responses are serializable")
            );
            return Ok(ExitCode::from(4));
        }
    };
    let mut session = ToolingSession::default();
    let mut failed = false;
    for value in values {
        let response = session.handle_value(value);
        failed |= response.error.is_some();
        println!(
            "{}",
            serde_json::to_string(&response).expect("tooling responses are serializable")
        );
    }
    Ok(if failed {
        ExitCode::from(4)
    } else {
        ExitCode::SUCCESS
    })
}

fn run_format(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let mut index = 1;
    let check = arguments
        .get(index)
        .is_some_and(|argument| argument == "--check");
    if check {
        index += 1;
    }
    if arguments.len() != index + 1 {
        return Err(CliFailure::usage());
    }
    let requested = PathBuf::from(&arguments[index]);
    let package = if requested.is_dir()
        || requested
            .extension()
            .is_some_and(|extension| extension == "toml")
    {
        terrane_compiler::Package::load(&requested).map_err(|errors| CliFailure {
            code: 3,
            message: errors
                .into_iter()
                .map(|error| error.diagnostic.render(&error.source))
                .collect(),
        })?
    } else {
        let text = fs::read_to_string(&requested).map_err(|error| {
            CliFailure::diagnostic(requested.clone(), "S0000", error.to_string(), 3)
        })?;
        terrane_compiler::Package::implicit(&requested, text)
    };
    let sources = package
        .units
        .iter()
        .map(|unit| {
            let path = fs::canonicalize(unit.source.path()).map_err(|error| {
                CliFailure::diagnostic(
                    unit.source.path().to_path_buf(),
                    "S3006",
                    format!("cannot identify formatting source: {error}"),
                    3,
                )
            })?;
            Ok(terrane_compiler::tooling::SourceInput {
                uri: format!("file://{}", path.display()),
                text: unit.source.text().to_owned(),
            })
        })
        .collect::<Result<Vec<_>, CliFailure>>()?;
    let mut engine = terrane_compiler::tooling::ToolingEngine::default();
    let snapshot = engine
        .open_snapshot(
            sources,
            None,
            None,
            terrane_compiler::tooling::SnapshotOptions::default(),
        )
        .map_err(tooling_failure)?;
    let mut replacements = Vec::new();
    for source in &snapshot.sources {
        let formatted = engine
            .format(&snapshot.snapshot_id, &source.uri)
            .map_err(tooling_failure)?;
        if formatted.changed {
            println!("{}", source.uri.trim_start_matches("file://"));
            replacements.extend(formatted.edits);
        }
    }
    if replacements.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    if check {
        return Ok(ExitCode::from(1));
    }
    let proposal = engine
        .propose_edits(&snapshot.snapshot_id, replacements)
        .map_err(tooling_failure)?;
    engine
        .apply_edits(&proposal.proposal_id)
        .map_err(tooling_failure)?;
    Ok(ExitCode::SUCCESS)
}

#[expect(
    clippy::needless_pass_by_value,
    reason = "map_err transfers the owned protocol error into CLI rendering"
)]
fn tooling_failure(error: terrane_compiler::tooling::ProtocolError) -> CliFailure {
    CliFailure::diagnostic(
        PathBuf::from("<tooling>"),
        "S3000",
        format!("{}: {}", error.code, error.message),
        4,
    )
}

#[derive(Default)]
struct ToolingSession {
    engine: terrane_compiler::tooling::ToolingEngine,
    last_snapshot: Option<String>,
    last_build: Option<String>,
    last_proposal: Option<String>,
}

impl ToolingSession {
    fn handle_value(
        &mut self,
        mut value: serde_json::Value,
    ) -> terrane_compiler::tooling::ResponseEnvelope {
        let request_id = request_id_from_value(&value);
        if let Some(schema_version) = value
            .get("schema_version")
            .and_then(serde_json::Value::as_str)
            && schema_version != terrane_compiler::tooling::SCHEMA_VERSION
        {
            return protocol_error_response(
                "unsupported-schema",
                format!(
                    "unsupported schema version `{schema_version}`; expected `{}`",
                    terrane_compiler::tooling::SCHEMA_VERSION
                ),
                request_id,
            );
        }
        if value.get("snapshot_id").and_then(serde_json::Value::as_str) == Some("$last")
            && let Some(snapshot_id) = &self.last_snapshot
        {
            value["snapshot_id"] = serde_json::Value::String(snapshot_id.clone());
        }
        if value.get("build_id").and_then(serde_json::Value::as_str) == Some("$last-build")
            && let Some(build_id) = &self.last_build
        {
            value["build_id"] = serde_json::Value::String(build_id.clone());
        }
        if value.get("proposal_id").and_then(serde_json::Value::as_str) == Some("$last-proposal")
            && let Some(proposal_id) = &self.last_proposal
        {
            value["proposal_id"] = serde_json::Value::String(proposal_id.clone());
        }
        let response = match serde_json::from_value(value) {
            Ok(request) => self.engine.handle(request),
            Err(error) => protocol_parse_error(&error, request_id),
        };
        if response.error.is_none() && response.snapshot_id.is_some() {
            self.last_snapshot.clone_from(&response.snapshot_id);
        }
        if let Some(build_id) = response
            .result
            .as_ref()
            .and_then(|result| result.get("build_id"))
            .and_then(|build_id| build_id.get("known"))
            .and_then(serde_json::Value::as_str)
        {
            self.last_build = Some(build_id.to_owned());
        }
        if let Some(proposal_id) = response
            .result
            .as_ref()
            .and_then(|result| result.get("proposal_id"))
            .and_then(serde_json::Value::as_str)
        {
            self.last_proposal = Some(proposal_id.to_owned());
        }
        response
    }
}

fn request_id_from_value(value: &serde_json::Value) -> Option<String> {
    value.get("request_id").map(|request_id| {
        request_id
            .as_str()
            .map_or_else(|| request_id.to_string(), str::to_owned)
    })
}

fn protocol_error_response(
    code: &str,
    message: String,
    request_id: Option<String>,
) -> terrane_compiler::tooling::ResponseEnvelope {
    terrane_compiler::tooling::ResponseEnvelope {
        compiler_version: terrane_compiler::VERSION.to_owned(),
        schema_version: terrane_compiler::tooling::SCHEMA_VERSION.to_owned(),
        request_id: request_id.unwrap_or_default(),
        snapshot_id: None,
        source_uri: None,
        source_hash: None,
        result: None,
        error: Some(terrane_compiler::tooling::ProtocolError {
            code: code.to_owned(),
            message,
            retry_fresh_query: false,
            apply_report: None,
        }),
    }
}

fn protocol_parse_error(
    error: &serde_json::Error,
    request_id: Option<String>,
) -> terrane_compiler::tooling::ResponseEnvelope {
    protocol_error_response("invalid-json", error.to_string(), request_id)
}

#[derive(Default)]
struct PackageSourceArguments {
    path: Option<PathBuf>,
    git: Option<String>,
    archive: Option<String>,
    tag: Option<String>,
}

fn parse_package_source(arguments: &[OsString]) -> Result<PackageSourceArguments, CliFailure> {
    let mut parsed = PackageSourceArguments::default();
    let mut index = 2;
    while index < arguments.len() {
        let argument = &arguments[index];
        let option = argument.to_str();
        if matches!(option, Some("--git" | "--archive" | "--tag")) {
            let value = arguments
                .get(index + 1)
                .and_then(|value| value.to_str())
                .filter(|value| !value.is_empty())
                .ok_or_else(CliFailure::usage)?;
            let destination = match option {
                Some("--git") => &mut parsed.git,
                Some("--archive") => &mut parsed.archive,
                Some("--tag") => &mut parsed.tag,
                _ => unreachable!("matched package option"),
            };
            if destination.replace(value.to_owned()).is_some() {
                return Err(CliFailure::usage_with(format!(
                    "package option `{}` was provided more than once",
                    option.expect("matched package option")
                )));
            }
            index += 2;
            continue;
        }
        if option.is_some_and(|argument| argument.starts_with('-')) || parsed.path.is_some() {
            return Err(CliFailure::usage());
        }
        parsed.path = Some(PathBuf::from(argument));
        index += 1;
    }
    match (&parsed.path, &parsed.git, &parsed.archive, &parsed.tag) {
        (Some(_), None, None, None)
        | (None, Some(_), None, Some(_))
        | (None, None, Some(_), None) => Ok(parsed),
        _ => Err(CliFailure::usage_with(
            "package source must be one local directory, `--git <url> --tag <tag>`, or `--archive <url>`",
        )),
    }
}

fn local_library_identity(path: &Path) -> Result<String, CliFailure> {
    let package = terrane_compiler::Package::load(path).map_err(|errors| {
        CliFailure::package(
            errors
                .into_iter()
                .map(|error| error.diagnostic.message)
                .collect::<Vec<_>>()
                .join("; "),
        )
    })?;
    if package.artifact != terrane_compiler::ArtifactKind::Library {
        return Err(CliFailure::package(format!(
            "package `{}` must declare `artifact = \"library\"`",
            package.identity
        )));
    }
    Ok(package.identity)
}

fn has_terrane_dependency(
    root: &toml_edit::Table,
    identity: &str,
    (owner, package): (&str, &str),
) -> bool {
    root.contains_key(identity)
        || root.get(owner).is_some_and(|owner_item| {
            owner_item
                .as_table()
                .is_some_and(|owner_table| owner_table.contains_key(package))
                || owner_item
                    .as_inline_table()
                    .is_some_and(|owner_table| owner_table.contains_key(package))
        })
}

fn is_dependency_owner_table(item: &toml_edit::Item) -> bool {
    if let Some(table) = item.as_table() {
        table
            .iter()
            .all(|(_, item)| item.is_table() || item.is_inline_table())
    } else if let Some(table) = item.as_inline_table() {
        table.iter().all(|(_, value)| value.is_inline_table())
    } else {
        false
    }
}

fn installation_manifest_path() -> Result<PathBuf, CliFailure> {
    let path = std::env::current_dir()
        .map_err(|error| CliFailure::package(format!("cannot read current directory: {error}")))?
        .join(terrane_compiler::MANIFEST_FILE_NAME);
    if !path.is_file() {
        return Err(CliFailure::package(
            "`package install` requires package.toml in the current directory",
        ));
    }
    Ok(path)
}

fn install_package(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let source = parse_package_source(arguments)?;
    let manifest_path = installation_manifest_path()?;
    let (identity, hash, source_fields) = if let Some(path) = source.path {
        if path.is_absolute() {
            return Err(CliFailure::usage_with(
                "local package installation requires a relative directory path",
            ));
        }
        let identity = local_library_identity(&path)?;
        let path = path
            .to_str()
            .ok_or_else(|| CliFailure::package("local package path is not valid UTF-8"))?
            .replace('\\', "/");
        (identity, None, vec![("path", path)])
    } else if let Some(url) = source.git {
        let tag = source.tag.expect("validated Git package tag");
        let (identity, hash) =
            terrane_compiler::git_library_metadata(&url, &tag).map_err(CliFailure::package)?;
        (
            identity,
            Some(hash.clone()),
            vec![("git", url), ("tag", tag), ("hash", hash)],
        )
    } else {
        let url = source.archive.expect("validated archive package source");
        let (identity, hash) =
            terrane_compiler::archive_library_metadata(&url).map_err(CliFailure::package)?;
        (
            identity,
            Some(hash.clone()),
            vec![("archive", url), ("hash", hash)],
        )
    };
    let (owner, package) =
        terrane_compiler::dependency_identity_parts(&identity).ok_or_else(|| {
            CliFailure::package(format!(
                "library package identity `{identity}` must be canonical owner/name"
            ))
        })?;
    let manifest_text = fs::read_to_string(&manifest_path)
        .map_err(|error| CliFailure::package(format!("cannot read package.toml: {error}")))?;
    let mut document = manifest_text
        .parse::<toml_edit::DocumentMut>()
        .map_err(|error| CliFailure::package(format!("invalid package.toml: {error}")))?;
    if document.get("terrane-dependencies").is_none() {
        let mut dependencies = toml_edit::Table::new();
        dependencies.set_implicit(true);
        document.insert("terrane-dependencies", toml_edit::Item::Table(dependencies));
    }
    let dependencies = document["terrane-dependencies"]
        .as_table_mut()
        .ok_or_else(|| CliFailure::package("`terrane-dependencies` must be a table"))?;
    if has_terrane_dependency(dependencies, &identity, (owner, package)) {
        return Err(CliFailure::package(format!(
            "Terrane dependency `{identity}` is already installed"
        )));
    }
    if dependencies
        .get(owner)
        .is_some_and(|item| !is_dependency_owner_table(item))
    {
        return Err(CliFailure::package(format!(
            "Terrane dependency owner `{owner}` has a conflicting `{package}` entry"
        )));
    }
    let mut dependency = toml_edit::Table::new();
    for (field, value) in source_fields {
        dependency.insert(field, toml_edit::value(value));
    }
    if let Some(owner_table) = dependencies
        .get_mut(owner)
        .and_then(toml_edit::Item::as_inline_table_mut)
    {
        owner_table.insert(
            package,
            toml_edit::Value::InlineTable(dependency.into_inline_table()),
        );
    } else {
        if !dependencies.contains_key(owner) {
            let mut owner_table = toml_edit::Table::new();
            owner_table.set_implicit(true);
            dependencies.insert(owner, toml_edit::Item::Table(owner_table));
        }
        dependencies[owner]
            .as_table_mut()
            .expect("validated owner table")
            .insert(package, toml_edit::Item::Table(dependency));
    }
    fs::write(&manifest_path, document.to_string())
        .map_err(|error| CliFailure::package(format!("cannot update package.toml: {error}")))?;
    if let Some(hash) = hash {
        println!("installed {identity} ({hash})");
    } else {
        println!("installed {identity}");
    }
    Ok(ExitCode::SUCCESS)
}

fn run_package(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    match arguments.get(1).and_then(|argument| argument.to_str()) {
        Some("hash") => {
            let source = parse_package_source(arguments)?;
            let hash = if let Some(path) = source.path {
                terrane_compiler::source_tree_hash(&path).map_err(CliFailure::package)?
            } else if let Some(url) = source.archive {
                terrane_compiler::archive_source_tree_hash(&url).map_err(CliFailure::package)?
            } else {
                terrane_compiler::git_source_tree_hash(
                    &source.git.expect("validated Git package source"),
                    &source.tag.expect("validated Git package tag"),
                )
                .map_err(CliFailure::package)?
            };
            println!("{hash}");
            Ok(ExitCode::SUCCESS)
        }
        Some("install") => install_package(arguments),
        _ => Err(CliFailure::usage()),
    }
}

fn usage() -> String {
    "usage: terrane <check|rust|build|run> [--require-canonical-rust] [--lint-name-style] \
     [--lint-unused-functions] [--release] <file-or-manifest> [-- program arguments]\n\
     terrane debug [--embed-sources] [--embed-generated-sources] <file-or-manifest> \
     [-- program arguments]\n\
     terrane profile record ((--cpu|--allocations) [--memory-timeline]|--memory-timeline) \
     [--embed-sources] [--retain-arguments] [-o|--output <file>] <file-or-manifest> [-- program arguments]\n\
     terrane profile show <file.trnprof> [--focus <source>:<line>] [--generated] [--native] \
     [--format <text|json>] [--limit <count>] [--compare <baseline.trnprof>] \
     [--max-allocated-bytes <bytes>] [--max-retained-bytes <bytes>] \
     [--source-root <path>] [--build-root <path>]\n\
     terrane debug-adapter --stdio\n\
     terrane test [--list] [--filter <text>|--exact <identity>|--glob <pattern>|--regex <pattern>] \
     [--tier <tier>] [--jobs <count>] [--timeout <duration>] [--argument <value>] [--fail-fast] \
     [--show-output] [--report <json-file>] <package-or-manifest>\n\
     terrane declaration-interface <package-or-source> --output <interface.json>\n\
     terrane <file-or-manifest> [program arguments]\n\
     terrane tooling --stdio\n\
     terrane query --request <json-file>\n\
     terrane fmt [--check] <file-or-manifest>\n\
     terrane package install <relative-directory>\n\
     terrane package install --git <url> --tag <tag>\n\
     terrane package install --archive <url>\n\
     terrane package hash <directory>\n\
     terrane package hash --git <url> --tag <tag>\n\
     terrane package hash --archive <url>\n\
     terrane projection-census <package> <version> --target <triple> --root <directory> \
     [--alias <name>] [--features <comma-list>] [--no-default-features] \
     [--target-condition <cargo-cfg>]\n\
     terrane toolchains\n\
     options:\n  --require-canonical-rust  fail unless lowering emits bundled-formatter output\n  \
     --lint-name-style  warn when authored declarations are not kebab-case\n  \
     --lint-unused-functions  report authored top-level functions with no resolved references\n  \
     --release  use Cargo's optimized release profile for build or run\n  \
     --embed-sources  include authored source snapshots in debug provenance or profile artifacts\n  \
     --retain-arguments  include profile workload arguments in the artifact (profile record only)\n  \
     -o, --output <file>  write rust output and its support sidecar (rust only)\n\
     commands:\n  check  validate and compile generated Rust\n  rust   print generated Rust or write split files\n  \
     build  compile the package's native artifact\n  run    compile and execute the program\n  \
     debug  build and launch the LLDB-backed Terrane source debugger\n  \
     profile  record optimized CPU, allocation, and process-memory evidence, or show source-attributed reports\n\
     debug-adapter  serve the Terrane DAP translation layer over standard input/output\n  \
     test   discover, compile, and isolate Terrane test functions\n  \
     tooling  serve versioned JSON-lines source-intelligence requests\n  \
     query  execute one source-intelligence request\n  fmt    format Terrane source (`--check` does not write)\n  \
     package  hash or install directory, tagged-Git, and source-archive Terrane libraries\n  \
     projection-census  compare native public surface with real compiler projection admission;\n\
       versions are normalized to exact requirements and aliases default to package names with hyphens replaced by underscores\n  \
     toolchains  report Rust toolchains previously requested by Terrane"
        .to_owned()
}
#[cfg(test)]
mod tests {
    use super::*;
    use terrane_compiler::{
        SourceFile, SourceUnit, Span,
        rust_ir::{RenderedFile, SourceAssociation},
    };
    use toml::Value as TomlValue;

    fn toml_table<'a>(value: &'a TomlValue, path: &[&str]) -> &'a toml::Table {
        path.iter()
            .fold(value, |value, key| &value[*key])
            .as_table()
            .expect("expected TOML table")
    }

    fn assert_generated_runtime_manifest(manifest: &str, directory: &Path) {
        let application: TomlValue = manifest.parse().unwrap();
        let package = toml_table(&application, &["package"]);
        assert_eq!(
            package["rust-version"].as_str(),
            Some(terrane_compiler::BUILD_TOOLCHAIN)
        );
        assert_eq!(
            toml_table(&application, &["package", "metadata", "terrane"])["unicode-data-version"]
                .as_str(),
            Some(terrane_compiler::UNICODE_DATA_VERSION)
        );
        assert_eq!(
            toml_table(&application, &["lints", "rust"])["unsafe_code"].as_str(),
            Some("forbid")
        );
        let dependencies = toml_table(&application, &["dependencies"]);
        let tokio = dependencies["tokio"].as_table().expect("Tokio dependency");
        assert_eq!(tokio["version"].as_str(), Some("=1.53.0"));
        let features: Vec<_> = tokio["features"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        for feature in ["macros", "rt", "rt-multi-thread", "sync", "time"] {
            assert!(
                features.contains(&feature),
                "missing Tokio feature {feature}"
            );
        }
        assert_eq!(
            toml_table(&application, &["profile", "dev"])["panic"].as_str(),
            Some("abort")
        );
        let release = toml_table(&application, &["profile", "release"]);
        assert_eq!(release["opt-level"].as_integer(), Some(3));
        assert_eq!(release["lto"].as_str(), Some("fat"));
        assert_eq!(release["codegen-units"].as_integer(), Some(1));
        assert_eq!(
            toml_table(&application, &["profile", "terrane-debug"])["inherits"].as_str(),
            Some("dev")
        );
        assert_eq!(
            toml_table(&application, &["profile", "terrane-profile"])["inherits"].as_str(),
            Some("release")
        );
        assert_eq!(
            toml_table(&application, &["profile", "terrane-debug"])["opt-level"].as_integer(),
            Some(0)
        );
        assert_eq!(
            toml_table(&application, &["profile", "terrane-profile"])["opt-level"].as_integer(),
            Some(3)
        );

        let support: TomlValue =
            fs::read_to_string(directory.join("support/terrane-string-support/Cargo.toml"))
                .unwrap()
                .parse()
                .unwrap();
        let string_dependencies = toml_table(&support, &["dependencies"]);
        assert_eq!(string_dependencies["caseless"].as_str(), Some("=0.2.2"));
        assert_eq!(
            string_dependencies["unicode-normalization"].as_str(),
            Some("=0.1.24")
        );
        assert_eq!(
            string_dependencies["unicode-segmentation"].as_str(),
            Some("=1.12.0")
        );
        let toolchain: TomlValue = fs::read_to_string(directory.join("rust-toolchain.toml"))
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            toml_table(&toolchain, &["toolchain"])["channel"].as_str(),
            Some(terrane_compiler::BUILD_TOOLCHAIN)
        );
        let metadata: TomlValue = fs::read_to_string(directory.join("terrane-build.toml"))
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            metadata["rust-toolchain"].as_str(),
            Some(terrane_compiler::BUILD_TOOLCHAIN)
        );
    }

    #[test]
    fn generated_cargo_manifest_configures_runtime_profiles_and_toolchain() {
        let directory =
            std::env::temp_dir().join(format!("terrane-generated-manifest-{}", std::process::id()));
        if directory.exists() {
            fs::remove_dir_all(&directory).unwrap();
        }
        let write = |role, panic, runtime, artifact, unsafe_code, toolchain| {
            write_generated_crate(
                &directory,
                &[],
                &[],
                &[],
                GeneratedCrateOptions {
                    role,
                    panic,
                    uses_platform_support: false,
                    uses_async_runtime: runtime,
                    uses_tokio_blocking: false,
                    uses_tokio_sync: runtime,
                    build_toolchain: toolchain,
                    unsafe_code,
                    artifact,
                },
            )
            .unwrap();
            fs::read_to_string(directory.join("Cargo.toml")).unwrap()
        };

        let application = write(
            GeneratedCrateRole::Application,
            terrane_compiler::PanicProfile::Abort,
            true,
            terrane_compiler::ArtifactKind::Executable,
            UnsafeCodePolicy::Forbid,
            terrane_compiler::BuildToolchain::Pinned,
        );
        assert_generated_runtime_manifest(&application, &directory);

        let library = write(
            GeneratedCrateRole::UnitTest,
            terrane_compiler::PanicProfile::Unwind,
            false,
            terrane_compiler::ArtifactKind::DynamicLibrary,
            UnsafeCodePolicy::MaintainedModules,
            terrane_compiler::BuildToolchain::System,
        )
        .parse::<TomlValue>()
        .unwrap();
        assert_eq!(
            toml_table(&library, &["lib"])["crate-type"]
                .as_array()
                .unwrap()[0]
                .as_str(),
            Some("cdylib")
        );
        assert_eq!(
            toml_table(&library, &["lints", "rust"])["unsafe_code"].as_str(),
            Some("deny")
        );
        assert!(!toml_table(&library, &["dependencies"]).contains_key("tokio"));
        assert!(!toml_table(&library, &["profile"]).contains_key("dev"));
        assert!(toml_table(&library, &["profile", "terrane-debug"]).contains_key("inherits"));
        assert_eq!(
            fs::read_to_string(directory.join("terrane-build.toml"))
                .unwrap()
                .parse::<TomlValue>()
                .unwrap()["rust-toolchain"]
                .as_str(),
            Some("system")
        );
        assert!(!directory.join("rust-toolchain.toml").exists());
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn source_path_dispatches_to_run_and_forwards_arguments() {
        let arguments = [
            OsString::from("thing.trn"),
            OsString::from("first"),
            OsString::from("--flag"),
        ];
        assert_eq!(
            implicit_run_arguments(&arguments),
            Some(vec![
                OsString::from("run"),
                OsString::from("thing.trn"),
                OsString::from("--"),
                OsString::from("first"),
                OsString::from("--flag"),
            ])
        );
        for command in [
            "check",
            "rust",
            "build",
            "run",
            "tooling",
            "query",
            "fmt",
            "toolchains",
            "--help",
            "-h",
            "--version",
            "-V",
        ] {
            assert!(
                implicit_run_arguments(&[OsString::from(command)]).is_none(),
                "{command}"
            );
        }
        assert_eq!(
            implicit_run_arguments(&[
                OsString::from("thing.trn"),
                OsString::from("--"),
                OsString::from("first"),
            ]),
            Some(vec![
                OsString::from("run"),
                OsString::from("thing.trn"),
                OsString::from("--"),
                OsString::from("first"),
            ])
        );
    }

    #[test]
    fn release_is_available_only_for_native_build_and_run() {
        let build = [
            OsString::from("build"),
            OsString::from("--release"),
            OsString::from("package.toml"),
        ];
        assert_eq!(
            parse_input(&build, CliCommand::Build)
                .unwrap_or_else(|_| panic!("release build should parse")),
            (
                PathBuf::from("package.toml"),
                None,
                false,
                false,
                false,
                true,
                false,
                false,
            )
        );

        let check = [
            OsString::from("check"),
            OsString::from("--release"),
            OsString::from("package.toml"),
        ];
        assert!(parse_input(&check, CliCommand::Check).is_err());
    }

    #[test]
    fn run_and_debug_require_program_argument_separator() {
        for (name, command) in [("run", CliCommand::Run), ("debug", CliCommand::Debug)] {
            let arguments = [
                OsString::from(name),
                OsString::from("package.toml"),
                OsString::from("argument"),
            ];
            let failure = parse_input(&arguments, command).expect_err("separator must be required");
            assert_eq!(failure.code, 2);
            assert!(failure.message.contains(
                "program arguments must follow `--` (for example, `terrane run package.toml -- arg`)"
            ));
        }
    }

    #[test]
    fn backend_error_in_support_sidecar_projects_to_terrane_source() {
        let directory =
            std::env::temp_dir().join(format!("terrane-backend-diagnostic-{}", std::process::id()));
        if directory.exists() {
            fs::remove_dir_all(&directory).unwrap();
        }
        fs::create_dir_all(directory.join("src")).unwrap();
        fs::write(
            directory.join("Cargo.toml"),
            "[package]\nname = \"broken\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n",
        )
        .unwrap();
        let entrypoint = "include!(\"main.support.rs\");\nfn main() {}\n";
        let support = "fn broken() { missing_backend_name(); }\n";
        fs::write(directory.join("src/main.rs"), entrypoint).unwrap();
        fs::write(directory.join("src/main.support.rs"), support).unwrap();
        let rust_files = vec![
            RenderedFile {
                path: "src/main.support.rs".to_owned(),
                contents: support.to_owned(),
                associations: vec![SourceAssociation {
                    generated_start: 0,
                    generated_end: support.len(),
                    source: Span::new(0, 0, 14),
                }],
            },
            RenderedFile {
                path: "src/main.rs".to_owned(),
                contents: entrypoint.to_owned(),
                associations: Vec::new(),
            },
        ];
        let units = vec![SourceUnit {
            relative_path: PathBuf::from("case.trn"),
            source: SourceFile::new(0, PathBuf::from("case.trn"), "function main;\n".to_owned()),
            expected_namespace: None,
            prelude: true,
            role: terrane_compiler::SourceRole::Production,
        }];

        let failure = run_cargo(
            "check",
            &directory,
            &directory.join("target"),
            &rust_files,
            &units,
            false,
            terrane_compiler::rust_interop::projection::Containment::Unavailable,
            CargoProfile::Debug,
        )
        .unwrap_err();
        assert_eq!(failure.code, 5);
        assert!(failure.message.contains("case.trn:1:1: error[S9003]"));
        assert!(failure.message.contains("raw rustc diagnostic"));
        assert!(failure.message.contains("missing_backend_name"));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn windows_import_library_cache_copies_the_dll_companion() {
        let directory =
            std::env::temp_dir().join(format!("terrane-import-library-{}", std::process::id()));
        if directory.exists() {
            fs::remove_dir_all(&directory).unwrap();
        }
        let built = directory.join("target/debug/terrane_program.dll");
        let cached = directory.join("artifacts/debug/terrane_program.dll");
        fs::create_dir_all(built.parent().unwrap()).unwrap();
        fs::create_dir_all(cached.parent().unwrap()).unwrap();
        fs::write(built.with_extension("dll.lib"), b"import-library").unwrap();

        assert!(cache_import_library(&built, &cached).is_ok());

        assert_eq!(
            fs::read(cached.with_extension("dll.lib")).unwrap(),
            b"import-library"
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
