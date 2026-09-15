mod debug_command;
mod test_command;

use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::io::{BufRead as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CliCommand {
    Check,
    Rust,
    Build,
    Run,
    Debug,
    DebugAdapter,
    Test,
    Tooling,
    Query,
    Format,
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
            "run" => Some(Self::Run),
            "debug" => Some(Self::Debug),
            "debug-adapter" => Some(Self::DebugAdapter),
            "tooling" => Some(Self::Tooling),
            "query" => Some(Self::Query),
            "fmt" => Some(Self::Format),
            "toolchains" => Some(Self::Toolchains),
            "--help" | "-h" => Some(Self::Help),
            "--version" | "-V" => Some(Self::Version),
            _ => None,
        }
    }
}

fn main() -> ExitCode {
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
        CliCommand::Tooling => return run_tooling(arguments),
        CliCommand::Query => return run_query(arguments),
        CliCommand::Format => return run_format(arguments),
        CliCommand::Test => return test_command::run_tests(arguments),
        CliCommand::DebugAdapter => return debug_command::run_adapter(arguments),
        CliCommand::Check
        | CliCommand::Rust
        | CliCommand::Build
        | CliCommand::Run
        | CliCommand::Debug => {}
    }
    let (
        input_path,
        output_path,
        require_canonical_rust,
        lint_name_style,
        release,
        embed_debug_sources,
        embed_generated_sources,
    ) = parse_input(arguments, command)?;
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
    let compilation = match terrane_compiler::compile_package_with_options(
        &package,
        terrane_compiler::CompilerOptions {
            require_canonical_rust,
            lint_name_style,
            debug_build: if command == CliCommand::Debug {
                match (embed_debug_sources, embed_generated_sources) {
                    (false, false) => terrane_compiler::DebugBuild::ExternalSources,
                    (true, false) => terrane_compiler::DebugBuild::EmbeddedSources,
                    (false, true) => terrane_compiler::DebugBuild::EmbeddedGeneratedSources,
                    (true, true) => terrane_compiler::DebugBuild::EmbeddedAllSources,
                }
            } else {
                terrane_compiler::DebugBuild::Disabled
            },
        },
    ) {
        Ok(compilation) => compilation,
        Err(failure) => return Err(CliFailure::compilation(failure)),
    };
    emit_warnings(&compilation);
    if command == CliCommand::Rust && output_path.is_none() {
        print_rust(&compilation);
        return Ok(ExitCode::SUCCESS);
    }
    let rust_entrypoint = output_path
        .as_deref()
        .unwrap_or_else(|| Path::new("src/main.rs"));
    let rust_files = compilation
        .rust_files_for(rust_entrypoint)
        .map_err(CliFailure::rust_artifact)?;
    if command == CliCommand::Rust {
        write_rust(&rust_files)?;
        return Ok(ExitCode::SUCCESS);
    }
    ensure_rust_toolchain(package.build_toolchain)?;
    let uses_platform_support = compilation.requires_platform_support;
    let uses_async_runtime = compilation.requires_async_runtime;
    // Mutable async callable receiver transactions use `tokio::sync::Mutex` to
    // serialize overlapping invocations without coupling separated copies.
    let uses_tokio_sync = rust_files
        .iter()
        .any(|file| file.contents.contains("tokio::sync::"));
    let crate_dir = generated_crate_path(
        &package.root,
        &rust_files,
        uses_platform_support,
        uses_async_runtime,
        &compilation.rust_dependencies,
        package.build_toolchain,
    )?;
    let debug_profile = (command == CliCommand::Debug)
        .then_some(terrane_compiler::debugging::DEBUG_ARTIFACT_PROFILE);
    write_generated_crate(
        &crate_dir,
        &rust_files,
        &package.units,
        &compilation.rust_dependencies,
        GeneratedCrateOptions {
            panic: package.profile.panic,
            uses_platform_support,
            uses_async_runtime,
            uses_tokio_sync,
            build_toolchain: package.build_toolchain,
            debug_profile,
        },
    )?;
    record_and_prune_generated_crates(&crate_dir)?;
    let target_dir = package.root.join(".trn/cache/target");
    let executable = prepare_artifact(
        command,
        &crate_dir,
        &target_dir,
        &rust_files,
        &package.units,
        !compilation.rust_dependencies.is_empty(),
        compilation.dependency_containment,
        release,
    )?;
    if command == CliCommand::Check {
        return Ok(ExitCode::SUCCESS);
    }
    let executable = executable.expect("build, run, and debug prepare an executable");
    if command == CliCommand::Build {
        println!("{}", executable.display());
        return Ok(ExitCode::SUCCESS);
    }
    if command == CliCommand::Debug {
        let debug = compilation
            .debug_information(rust_entrypoint)
            .map_err(CliFailure::rust_artifact)?
            .expect("debug compilation produces debugger metadata");
        let build_identity = rust_debug_build_identity(&crate_dir)?;
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
fn rust_debug_build_identity(
    crate_dir: &Path,
) -> Result<terrane_compiler::debugging::DebugBuildIdentity, CliFailure> {
    let verbose = Command::new("rustc")
        .arg("-vV")
        .current_dir(crate_dir)
        .output()
        .map_err(|error| {
            CliFailure::backend(format!("failed to inspect debug Rust compiler: {error}"))
        })?;
    if !verbose.status.success() {
        return Err(CliFailure::backend(
            "debug Rust compiler did not report its target triple".to_owned(),
        ));
    }
    let rustc_release = String::from_utf8(verbose.stdout).map_err(|_| {
        CliFailure::backend("debug Rust compiler output was not valid UTF-8".to_owned())
    })?;
    let target = rustc_release
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .ok_or_else(|| {
            CliFailure::backend("debug Rust compiler output omitted its target triple".to_owned())
        })?
        .to_owned();
    let sysroot = Command::new("rustc")
        .args(["--print", "sysroot"])
        .current_dir(crate_dir)
        .output()
        .map_err(|error| {
            CliFailure::backend(format!("failed to inspect debug Rust sysroot: {error}"))
        })?;
    if !sysroot.status.success() {
        return Err(CliFailure::backend(
            "debug Rust compiler did not report its sysroot".to_owned(),
        ));
    }
    let sysroot = String::from_utf8(sysroot.stdout)
        .map_err(|_| CliFailure::backend("debug Rust sysroot was not valid UTF-8".to_owned()))?
        .trim()
        .to_owned();
    Ok(terrane_compiler::debugging::DebugBuildIdentity {
        target,
        rust_sysroot: sysroot,
        rustc_release,
        artifact_profile: terrane_compiler::debugging::DEBUG_ARTIFACT_PROFILE,
    })
}

type ParsedInput = (PathBuf, Option<PathBuf>, bool, bool, bool, bool, bool);

fn parse_input(arguments: &[OsString], command: CliCommand) -> Result<ParsedInput, CliFailure> {
    let mut input_index = 1;
    let mut output_path = None;
    let mut require_canonical_rust = false;
    let mut lint_name_style = false;
    let mut release = false;
    let mut embed_debug_sources = false;
    let mut embed_generated_sources = false;
    while let Some(argument) = arguments.get(input_index).and_then(|value| value.to_str()) {
        match argument {
            "--require-canonical-rust" => require_canonical_rust = true,
            "--lint-name-style" => lint_name_style = true,
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

#[expect(
    clippy::too_many_arguments,
    reason = "artifact preparation forwards one complete Cargo build context without hidden state"
)]
fn prepare_artifact(
    command: CliCommand,
    crate_dir: &Path,
    target_dir: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    units: &[terrane_compiler::SourceUnit],
    has_rust_dependencies: bool,
    containment: terrane_compiler::projection::Containment,
    release: bool,
) -> Result<Option<PathBuf>, CliFailure> {
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
                false,
            )?;
            fs::create_dir_all(stamp.parent().expect("artifact stamp has a parent")).map_err(
                |error| CliFailure::backend(format!("cannot create artifact cache: {error}")),
            )?;
            fs::write(&stamp, []).map_err(|error| {
                CliFailure::backend(format!("cannot record checked artifact: {error}"))
            })?;
        }
        return Ok(None);
    }
    let profile = if release { "release" } else { "debug" };
    let executable = executable_path(&crate_dir.join("artifacts").join(profile));
    if !executable.is_file() {
        run_cargo(
            "build",
            crate_dir,
            target_dir,
            rust_files,
            units,
            has_rust_dependencies,
            containment,
            release,
        )?;
        let built = executable_path(&target_dir.join(profile));
        fs::create_dir_all(executable.parent().expect("cached executable has a parent")).map_err(
            |error| CliFailure::backend(format!("cannot create artifact cache: {error}")),
        )?;
        fs::copy(&built, &executable)
            .map_err(|error| CliFailure::backend(format!("cannot cache built program: {error}")))?;
    }
    executable
        .canonicalize()
        .map(Some)
        .map_err(|error| CliFailure::backend(format!("cannot locate built program: {error}")))
}

fn executable_path(directory: &Path) -> PathBuf {
    let mut path = directory.join("terrane_program");
    path.set_extension(std::env::consts::EXE_EXTENSION);
    path
}

fn print_rust(compilation: &terrane_compiler::Compilation) {
    print!("{}", compilation.rust);
    println!("// Generated Rust form: standalone");
    println!(
        "// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi"
    );
}

fn write_rust(files: &[terrane_compiler::rust_ir::RenderedFile]) -> Result<(), CliFailure> {
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
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    reason = "Cargo process policy, containment, diagnostics, and artifact caching form one operation"
)]
fn run_cargo(
    command: &str,
    crate_dir: &Path,
    target_dir: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    units: &[terrane_compiler::SourceUnit],
    has_rust_dependencies: bool,
    containment: terrane_compiler::projection::Containment,
    release: bool,
) -> Result<(), CliFailure> {
    let rustflags = std::env::var_os("RUSTFLAGS").unwrap_or_default();
    let contained =
        has_rust_dependencies && containment == terrane_compiler::projection::Containment::Enforced;
    if has_rust_dependencies {
        let mut fetch = Command::new("cargo");
        terrane_compiler::cargo_toolchain::configure_cargo_command(&mut fetch);
        configure_generated_toolchain(&mut fetch, crate_dir);
        let fetch = fetch
            .args(["fetch", "--manifest-path"])
            .arg(crate_dir.join("Cargo.toml"))
            .current_dir(crate_dir)
            .output()
            .map_err(|error| {
                CliFailure::backend(format!("failed to fetch generated dependencies: {error}"))
            })?;
        if !fetch.status.success() {
            return Err(CliFailure::backend(format!(
                "Cargo dependency fetch failed: {}",
                String::from_utf8_lossy(&fetch.stderr).trim()
            )));
        }
        if !contained {
            eprintln!(
                "warning: generated dependency build containment is unavailable; using declared host tier"
            );
        }
    }
    fs::create_dir_all(target_dir).map_err(|error| {
        CliFailure::backend(format!("cannot create Cargo target directory: {error}"))
    })?;
    let canonical_crate = crate_dir.canonicalize().map_err(|error| {
        CliFailure::backend(format!("cannot canonicalize generated crate: {error}"))
    })?;
    let canonical_target = target_dir.canonicalize().map_err(|error| {
        CliFailure::backend(format!(
            "cannot canonicalize Cargo target directory: {error}"
        ))
    })?;
    let mut cargo = if contained {
        let mut cargo = Command::new("bwrap");
        cargo.args([
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
        cargo
            .arg(&canonical_crate)
            .arg(&canonical_crate)
            .args(["--bind"])
            .arg(&canonical_target)
            .arg(&canonical_target)
            .args(["--", "cargo"]);
        cargo
    } else {
        Command::new("cargo")
    };
    terrane_compiler::cargo_toolchain::configure_cargo_command(&mut cargo);
    configure_generated_toolchain(&mut cargo, crate_dir);
    cargo.args([
        command,
        "--quiet",
        "--message-format=json",
        "--manifest-path",
    ]);
    cargo.arg(crate_dir.join("Cargo.toml"));
    if release {
        cargo.arg("--release");
    }
    if has_rust_dependencies {
        cargo.args(["--offline", "--frozen"]);
    }
    let output = cargo
        .current_dir(&canonical_crate)
        .env("CARGO_TARGET_DIR", &canonical_target)
        .env("RUSTFLAGS", rustflags)
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

fn generated_crate_path(
    package_root: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    uses_platform_support: bool,
    uses_async_runtime: bool,
    rust_dependencies: &[terrane_compiler::RustDependency],
    build_toolchain: terrane_compiler::BuildToolchain,
) -> Result<PathBuf, CliFailure> {
    let root = package_root.canonicalize().map_err(|error| {
        CliFailure::backend(format!(
            "cannot locate package root {}: {error}",
            package_root.display()
        ))
    })?;
    let mut hash = Sha256::new();
    hash.update(b"terrane-generated-crate-v3\0");
    hash.update(terrane_compiler::VERSION.as_bytes());
    for variable in [
        "CARGO_BUILD_TARGET",
        "CARGO_ENCODED_RUSTFLAGS",
        "RUSTC",
        "RUSTFLAGS",
        "TERRANE_SCCACHE",
    ] {
        hash.update(variable.as_bytes());
        hash.update(b"=");
        hash.update(std::env::var(variable).unwrap_or_default());
        hash.update(b"\0");
    }
    hash.update(format!("build-toolchain={build_toolchain:?}\0").as_bytes());
    hash.update([u8::from(uses_async_runtime)]);
    hash.update(b"profile=debug\0");
    for file in rust_files {
        hash.update(file.path.as_bytes());
        hash.update(b"\0");
        hash.update(file.contents.as_bytes());
        hash.update(b"\0");
    }
    for dependency in rust_dependencies {
        hash.update(dependency.name.as_bytes());
        hash.update(b"\0");
        hash.update(dependency.package.as_bytes());
        hash.update(b"\0");
        hash.update(dependency.version.as_bytes());
        hash.update(b"\0");
        hash.update([u8::from(dependency.default_features)]);
        hash.update(b"\0target=");
        hash.update(dependency.target.as_deref().unwrap_or_default().as_bytes());
        for feature in &dependency.features {
            hash.update(feature.as_bytes());
            hash.update(b"\0");
        }
    }
    for support in [
        include_bytes!("../../terrane-int-support/src/lib.rs").as_slice(),
        include_bytes!("../../terrane-scalar-support/src/lib.rs").as_slice(),
        include_bytes!("../../terrane-string-support/src/lib.rs").as_slice(),
        include_bytes!("../../terrane-stream-abi/src/lib.rs").as_slice(),
    ] {
        hash.update(support);
        hash.update(b"\0");
    }
    if uses_platform_support {
        hash.update(include_bytes!("../../terrane-platform-support/src/lib.rs"));
        hash.update(b"\0");
        hash.update(include_bytes!(
            "../../terrane-platform-support/src/observability.rs"
        ));
        hash.update(b"\0");
        hash.update(include_bytes!(
            "../../terrane-platform-support/src/signals.rs"
        ));
        hash.update(b"\0");
        hash.update(include_bytes!("../../terrane-signal-support/src/lib.rs"));
        hash.update(b"\0");
    }
    Ok(root
        .join(".trn/build")
        .join(format!("{:x}", hash.finalize())))
}

fn record_and_prune_generated_crates(active: &Path) -> Result<(), CliFailure> {
    const MAX_GENERATED_CRATES: usize = 8;

    fs::write(active.join(".last-used"), []).map_err(|error| {
        CliFailure::backend(format!("cannot record generated crate use: {error}"))
    })?;
    let root = active
        .parent()
        .expect("generated crate identity always has a build directory");
    let mut inactive = fs::read_dir(root)
        .map_err(|error| CliFailure::backend(format!("cannot inspect generated crates: {error}")))?
        .filter_map(Result::ok)
        .filter(|entry| entry.path() != active && entry.path().is_dir())
        .filter_map(|entry| {
            let used = entry
                .path()
                .join(".last-used")
                .metadata()
                .or_else(|_| entry.metadata())
                .and_then(|metadata| metadata.modified())
                .ok()?;
            Some((used, entry.path()))
        })
        .collect::<Vec<_>>();
    inactive.sort_by_key(|entry| std::cmp::Reverse(entry.0));
    for (_, path) in inactive.into_iter().skip(MAX_GENERATED_CRATES - 1) {
        fs::remove_dir_all(&path).map_err(|error| {
            CliFailure::backend(format!(
                "cannot evict stale generated crate {}: {error}",
                path.display()
            ))
        })?;
    }
    Ok(())
}

type DebugProfile = Option<terrane_compiler::debugging::DebugArtifactProfile>;

#[derive(Clone, Copy)]
struct GeneratedCrateOptions {
    panic: terrane_compiler::PanicProfile,
    uses_platform_support: bool,
    uses_async_runtime: bool,
    uses_tokio_sync: bool,
    build_toolchain: terrane_compiler::BuildToolchain,
    debug_profile: DebugProfile,
}

fn base_generated_manifest() -> String {
    format!(
        "[package]\nname = \"terrane_program\"\nversion = \"0.0.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n\
         [package.metadata.terrane]\nunicode-data-version = {:?}\n\n\
         [lints.rust]\nunsafe_code = \"forbid\"\n\n\
         [dependencies]\nterrane-int-support = {{ path = \"support/terrane-int-support\" }}\n\
         terrane-collection-support = {{ path = \"support/terrane-collection-support\" }}\n\
         terrane-scalar-support = {{ path = \"support/terrane-scalar-support\" }}\n\
         terrane-string-support = {{ path = \"support/terrane-string-support\" }}\n\
         terrane-document-support = {{ path = \"support/terrane-document-support\" }}\n\
         terrane-stream-abi = {{ path = \"support/terrane-stream-abi\" }}\n",
        terrane_compiler::BUILD_TOOLCHAIN,
        terrane_compiler::UNICODE_DATA_VERSION
    )
}

fn append_build_profiles(
    manifest: &mut String,
    panic: terrane_compiler::PanicProfile,
    debug_profile: DebugProfile,
) {
    if panic == terrane_compiler::PanicProfile::Abort || debug_profile.is_some() {
        manifest.push_str("\n[profile.dev]\n");
        if panic == terrane_compiler::PanicProfile::Abort {
            manifest.push_str("panic = \"abort\"\n");
        }
        if let Some(profile) = debug_profile {
            writeln!(manifest, "opt-level = {}", profile.optimization)
                .expect("writing to a string cannot fail");
            writeln!(manifest, "debug = {}", profile.cargo_debug)
                .expect("writing to a string cannot fail");
            writeln!(manifest, "strip = {:?}", profile.stripping)
                .expect("writing to a string cannot fail");
        }
    }
    manifest.push_str("\n[profile.release]\nopt-level = 3\nlto = \"fat\"\ncodegen-units = 1\n");
}

fn write_generated_crate(
    directory: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    units: &[terrane_compiler::SourceUnit],
    rust_dependencies: &[terrane_compiler::RustDependency],
    options: GeneratedCrateOptions,
) -> Result<(), CliFailure> {
    fs::create_dir_all(directory.join("src"))
        .map_err(|error| CliFailure::backend(format!("cannot create generated crate: {error}")))?;
    let mut manifest = base_generated_manifest();
    if options.uses_platform_support {
        manifest.push_str(
            "terrane-platform-support = { path = \"support/terrane-platform-support\" }\n",
        );
    }
    write_runtime_dependencies(&mut manifest, rust_dependencies, &options);
    let target_tables = rust_dependencies
        .iter()
        .map(terrane_compiler::RustDependency::cargo_manifest_table)
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
    append_build_profiles(&mut manifest, options.panic, options.debug_profile);
    manifest.push_str("\n[workspace]\n");
    write_if_changed(&directory.join("Cargo.toml"), manifest.as_bytes()).map_err(|error| {
        CliFailure::backend(format!("cannot write generated manifest: {error}"))
    })?;
    let toolchain_path = directory.join("rust-toolchain.toml");
    if options.build_toolchain == terrane_compiler::BuildToolchain::Pinned {
        let toolchain = format!(
            "[toolchain]\nchannel = {:?}\nprofile = \"minimal\"\n",
            terrane_compiler::BUILD_TOOLCHAIN
        );
        write_if_changed(&toolchain_path, toolchain.as_bytes()).map_err(|error| {
            CliFailure::backend(format!("cannot write generated toolchain pin: {error}"))
        })?;
    } else if toolchain_path.exists() {
        fs::remove_file(&toolchain_path).map_err(|error| {
            CliFailure::backend(format!("cannot remove generated toolchain pin: {error}"))
        })?;
    }
    write_generated_support(directory, options.uses_platform_support).map_err(|error| {
        CliFailure::backend(format!("cannot write generated runtime support: {error}"))
    })?;
    for rust_file in rust_files {
        let path = directory.join(&rust_file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                CliFailure::backend(format!("cannot create generated source directory: {error}"))
            })?;
        }
        write_if_changed(&path, rust_file.contents.as_bytes()).map_err(|error| {
            CliFailure::backend(format!("cannot write generated Rust: {error}"))
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
            terrane_compiler::BuildToolchain::Pinned => terrane_compiler::BUILD_TOOLCHAIN,
            terrane_compiler::BuildToolchain::System => "system",
        }
    )
    .expect("writing to a String cannot fail");
    write_if_changed(&directory.join("terrane-build.toml"), sources.as_bytes())
        .map_err(|error| CliFailure::backend(format!("cannot write build metadata: {error}")))?;
    Ok(())
}

fn write_runtime_dependencies(
    manifest: &mut String,
    rust_dependencies: &[terrane_compiler::RustDependency],
    options: &GeneratedCrateOptions,
) {
    let declared_tokio = rust_dependencies.iter().find(|dependency| {
        dependency.name == "tokio" && dependency.cargo_manifest_table() == "dependencies"
    });
    if options.uses_async_runtime {
        if let Some(dependency) = declared_tokio {
            let mut dependency = dependency.clone();
            dependency.features.extend(
                ["macros", "rt", "rt-multi-thread", "time"]
                    .into_iter()
                    .map(str::to_owned),
            );
            if options.uses_tokio_sync {
                dependency.features.push("sync".to_owned());
            }
            dependency.features.sort();
            dependency.features.dedup();
            write_rust_dependency(manifest, &dependency);
        } else {
            let sync = if options.uses_tokio_sync {
                ", \"sync\""
            } else {
                ""
            };
            writeln!(
                manifest,
                "tokio = {{ version = \"=1.53.0\", features = [\"macros\", \"rt\", \"rt-multi-thread\"{sync}, \"time\"] }}"
            )
            .expect("writing to a string cannot fail");
        }
    }
    for dependency in rust_dependencies
        .iter()
        .filter(|dependency| dependency.cargo_manifest_table() == "dependencies")
        .filter(|dependency| !options.uses_async_runtime || dependency.name != "tokio")
    {
        write_rust_dependency(manifest, dependency);
    }
}

fn write_rust_dependency(manifest: &mut String, dependency: &terrane_compiler::RustDependency) {
    manifest.push_str(&dependency.cargo_dependency_spec());
}

fn write_generated_support(directory: &Path, uses_platform_support: bool) -> std::io::Result<()> {
    let int = directory.join("support/terrane-int-support");
    let collection = directory.join("support/terrane-collection-support");
    let scalar = directory.join("support/terrane-scalar-support");
    let string = directory.join("support/terrane-string-support");
    let document = directory.join("support/terrane-document-support");
    let stream = directory.join("support/terrane-stream-abi");
    let platform =
        uses_platform_support.then(|| directory.join("support/terrane-platform-support"));
    let signal = uses_platform_support.then(|| directory.join("support/terrane-signal-support"));
    fs::create_dir_all(int.join("src"))?;
    fs::create_dir_all(collection.join("src"))?;
    fs::create_dir_all(scalar.join("src"))?;
    fs::create_dir_all(string.join("src"))?;
    fs::create_dir_all(document.join("src"))?;
    fs::create_dir_all(stream.join("src"))?;
    if let Some(platform) = &platform {
        fs::create_dir_all(platform.join("src"))?;
    }
    if let Some(signal) = &signal {
        fs::create_dir_all(signal.join("src"))?;
    }
    write_if_changed(
        &int.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-int-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nnum-bigint = {{ version = \"0.4\", features = [\"std\"] }}\nnum-integer = \"0.1\"\nnum-traits = \"0.2\"\n", terrane_compiler::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &collection.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-collection-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nindexmap = \"2\"\nterrane-int-support = {{ path = \"../terrane-int-support\" }}\nunicode-segmentation = \"=1.12.0\"\n", terrane_compiler::BUILD_TOOLCHAIN).as_bytes(),
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
        format!("[package]\nname = \"terrane-scalar-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nterrane-int-support = {{ path = \"../terrane-int-support\" }}\n", terrane_compiler::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &scalar.join("src/lib.rs"),
        include_bytes!("../../terrane-scalar-support/src/lib.rs"),
    )?;
    write_if_changed(
        &string.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-string-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\n# Unicode 16.0 profile: case folding, normalization, and segmentation move together.\ncaseless = \"=0.2.2\"\n# unicode-normalization permits tinyvec 1.13, whose alloc-only build fails on Rust 1.93.\ntinyvec = {{ version = \"=1.12.0\", features = [\"std\"] }}\nunicode-normalization = \"=0.1.24\"\nunicode-segmentation = \"=1.12.0\"\n", terrane_compiler::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &string.join("src/lib.rs"),
        include_bytes!("../../terrane-string-support/src/lib.rs"),
    )?;
    write_if_changed(
        &document.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-document-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nserde = \"1\"\nserde_json = {{ version = \"1\", features = [\"arbitrary_precision\", \"unbounded_depth\"] }}\nurl = \"=2.5.7\"\nyaml-rust2 = \"=0.10.4\"\n", terrane_compiler::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &document.join("src/lib.rs"),
        include_bytes!("../../terrane-document-support/src/lib.rs"),
    )?;
    write_if_changed(
        &stream.join("Cargo.toml"),
        format!("[package]\nname = \"terrane-stream-abi\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = {:?}\n\n[dependencies]\nlibc = \"0.2\"\nrustix = {{ version = \"1\", features = [\"fs\"] }}\n", terrane_compiler::BUILD_TOOLCHAIN).as_bytes(),
    )?;
    write_if_changed(
        &stream.join("src/lib.rs"),
        include_bytes!("../../terrane-stream-abi/src/lib.rs"),
    )?;
    if let Some(platform) = platform {
        write_if_changed(
            &platform.join("Cargo.toml"),
            terrane_compiler::platform_support_manifest().as_bytes(),
        )?;
        write_if_changed(
            &platform.join("src/lib.rs"),
            include_bytes!("../../terrane-platform-support/src/lib.rs"),
        )?;
        write_if_changed(
            &platform.join("src/observability.rs"),
            include_bytes!("../../terrane-platform-support/src/observability.rs"),
        )?;
        write_if_changed(
            &platform.join("src/signals.rs"),
            include_bytes!("../../terrane-platform-support/src/signals.rs"),
        )?;
    }
    if let Some(signal) = signal {
        write_if_changed(
            &signal.join("Cargo.toml"),
            terrane_compiler::signal_support_manifest().as_bytes(),
        )?;
        write_if_changed(
            &signal.join("src/lib.rs"),
            include_bytes!("../../terrane-signal-support/src/lib.rs"),
        )?;
    }
    Ok(())
}

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

fn usage() -> String {
    "usage: terrane <check|rust|build|run> [--require-canonical-rust] [--lint-name-style] \
     [--release] <file-or-manifest> [-- program arguments]\n\
     terrane debug [--embed-sources] [--embed-generated-sources] <file-or-manifest> \
     [-- program arguments]\n\
     terrane debug-adapter --stdio\n\
     terrane test [--list] [--filter <text>|--exact <identity>|--glob <pattern>|--regex <pattern>] \
     [--tier <tier>] [--jobs <count>] [--timeout <duration>] [--argument <value>] [--fail-fast] \
     [--show-output] [--report <json-file>] <package-or-manifest>\n\
     terrane <file-or-manifest> [program arguments]\n\
     terrane tooling --stdio\n\
     terrane query --request <json-file>\n\
     terrane fmt [--check] <file-or-manifest>\n\
     terrane toolchains\n\
     options:\n  --require-canonical-rust  fail unless lowering emits bundled-formatter output\n  \
     --lint-name-style  warn when authored declarations are not kebab-case\n  \
     --release  use Cargo's optimized release profile for build or run\n  \
     --embed-sources  include authored source snapshots in debug provenance (debug only)\n  \
     -o, --output <file>  write rust output and its support sidecar (rust only)\n\
     commands:\n  check  validate and compile generated Rust\n  rust   print generated Rust or write split files\n  \
     build  compile a native executable\n  run    compile and execute the program\n  \
     debug  build and launch the LLDB-backed Terrane source debugger\n  \
     debug-adapter  serve the Terrane DAP translation layer over standard input/output\n  \
     test   discover, compile, and isolate Terrane test functions\n  \
     tooling  serve versioned JSON-lines source-intelligence requests\n  \
     query  execute one source-intelligence request\n  fmt    format Terrane source (`--check` does not write)\n  \
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
                true,
                false,
                false
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
            role: terrane_compiler::SourceRole::Production,
        }];

        let failure = run_cargo(
            "check",
            &directory,
            &directory.join("target"),
            &rust_files,
            &units,
            false,
            terrane_compiler::projection::Containment::Unavailable,
            false,
        )
        .unwrap_err();
        assert_eq!(failure.code, 5);
        assert!(failure.message.contains("case.trn:1:1: error[S9003]"));
        assert!(failure.message.contains("raw rustc diagnostic"));
        assert!(failure.message.contains("missing_backend_name"));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn generated_crate_cache_evicts_stale_identities() {
        let directory =
            std::env::temp_dir().join(format!("terrane-cache-eviction-{}", std::process::id()));
        if directory.exists() {
            fs::remove_dir_all(&directory).unwrap();
        }
        fs::create_dir_all(&directory).unwrap();
        for index in 0..10 {
            let identity = directory.join(format!("{index:02}"));
            fs::create_dir(&identity).unwrap();
            assert!(record_and_prune_generated_crates(&identity).is_ok());
        }
        let identities = fs::read_dir(&directory).unwrap().count();
        assert_eq!(identities, 8);
        assert!(directory.join("09").is_dir());
        fs::remove_dir_all(directory).unwrap();
    }
    #[test]
    fn generated_cargo_manifest_configures_build_profiles_and_runtime() {
        let directory =
            std::env::temp_dir().join(format!("terrane-build-profiles-{}", std::process::id()));
        if directory.exists() {
            fs::remove_dir_all(&directory).unwrap();
        }

        assert!(
            write_generated_crate(
                &directory,
                &[],
                &[],
                &[],
                GeneratedCrateOptions {
                    panic: terrane_compiler::PanicProfile::Abort,
                    uses_platform_support: false,
                    uses_async_runtime: true,
                    uses_tokio_sync: true,
                    build_toolchain: terrane_compiler::BuildToolchain::Pinned,
                    debug_profile: Some(terrane_compiler::debugging::DEBUG_ARTIFACT_PROFILE,),
                },
            )
            .is_ok()
        );

        let manifest = fs::read_to_string(directory.join("Cargo.toml")).unwrap();
        assert!(manifest.contains("[profile.dev]\npanic = \"abort\"\n"));
        assert!(manifest.contains("opt-level = 0\ndebug = 2\nstrip = \"none\"\n"));
        assert!(
            manifest
                .contains("[profile.release]\nopt-level = 3\nlto = \"fat\"\ncodegen-units = 1\n")
        );
        assert!(manifest.contains(&format!(
            "rust-version = \"{}\"",
            terrane_compiler::BUILD_TOOLCHAIN
        )));
        assert!(manifest.contains("unicode-data-version = \"16.0.0\""));
        assert!(manifest.contains(
            "tokio = { version = \"=1.53.0\", features = [\"macros\", \"rt\", \"rt-multi-thread\", \"sync\", \"time\"] }"
        ));
        assert!(manifest.contains("[lints.rust]\nunsafe_code = \"forbid\""));
        let string_support =
            fs::read_to_string(directory.join("support/terrane-string-support/Cargo.toml"))
                .unwrap();
        assert!(string_support.contains("caseless = \"=0.2.2\""));
        assert!(string_support.contains("unicode-normalization = \"=0.1.24\""));
        assert!(string_support.contains("unicode-segmentation = \"=1.12.0\""));
        assert!(directory.join("rust-toolchain.toml").is_file());
        let metadata = fs::read_to_string(directory.join("terrane-build.toml")).unwrap();
        assert!(metadata.contains(&format!(
            "rust-toolchain = \"{}\"",
            terrane_compiler::BUILD_TOOLCHAIN
        )));

        assert!(
            write_generated_crate(
                &directory,
                &[],
                &[],
                &[],
                GeneratedCrateOptions {
                    panic: terrane_compiler::PanicProfile::Abort,
                    uses_platform_support: false,
                    uses_async_runtime: false,
                    uses_tokio_sync: false,
                    build_toolchain: terrane_compiler::BuildToolchain::Pinned,
                    debug_profile: None,
                },
            )
            .is_ok()
        );
        let synchronous_manifest = fs::read_to_string(directory.join("Cargo.toml")).unwrap();
        assert!(!synchronous_manifest.contains("\ntokio = "));
        fs::remove_dir_all(directory).unwrap();
    }
}
