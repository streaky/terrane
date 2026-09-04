use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
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

fn main() -> ExitCode {
    match run(&std::env::args_os().skip(1).collect::<Vec<_>>()) {
        Ok(code) => code,
        Err(failure) => {
            eprint!("{}", failure.message);
            ExitCode::from(failure.code)
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "the shared CLI pipeline keeps command phase ordering explicit"
)]
fn run(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let Some(command) = arguments.first().and_then(|value| value.to_str()) else {
        return Err(CliFailure::usage());
    };
    if command == "--version" || command == "-V" {
        println!("terrane {}", terrane_compiler::VERSION);
        return Ok(ExitCode::SUCCESS);
    }
    if command == "--help" || command == "-h" {
        println!("{}", usage());
        return Ok(ExitCode::SUCCESS);
    }
    if !matches!(command, "check" | "rust" | "build" | "run") {
        return Err(CliFailure::usage());
    }
    let (input_path, output_path, require_canonical_rust, lint_name_style, release) =
        parse_input(arguments, command)?;
    let package = if input_path
        .extension()
        .is_some_and(|extension| extension == "trn")
    {
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
        },
    ) {
        Ok(compilation) => compilation,
        Err(failure) => return Err(CliFailure::compilation(failure)),
    };
    emit_warnings(&compilation);
    if command == "rust" && output_path.is_none() {
        print_rust(&compilation);
        return Ok(ExitCode::SUCCESS);
    }
    let rust_entrypoint = output_path
        .as_deref()
        .unwrap_or_else(|| Path::new("src/main.rs"));
    let rust_files = compilation
        .rust_files_for(rust_entrypoint)
        .map_err(CliFailure::rust_artifact)?;
    if command == "rust" {
        write_rust(&rust_files)?;
        return Ok(ExitCode::SUCCESS);
    }
    ensure_rust_toolchain()?;
    let uses_platform_support = compilation.requires_platform_support;
    let crate_dir = generated_crate_path(
        &package.root,
        &rust_files,
        uses_platform_support,
        &compilation.rust_dependencies,
    )?;
    write_generated_crate(
        &crate_dir,
        &rust_files,
        &package.units,
        &compilation.rust_dependencies,
        package.profile.panic,
        uses_platform_support,
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
    if command == "check" {
        return Ok(ExitCode::SUCCESS);
    }
    let executable = executable.expect("build and run prepare an executable");
    if command == "build" {
        println!("{}", executable.display());
        return Ok(ExitCode::SUCCESS);
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

fn parse_input(
    arguments: &[OsString],
    command: &str,
) -> Result<(PathBuf, Option<PathBuf>, bool, bool, bool), CliFailure> {
    let mut input_index = 1;
    let mut output_path = None;
    let mut require_canonical_rust = false;
    let mut lint_name_style = false;
    let mut release = false;
    while let Some(argument) = arguments.get(input_index).and_then(|value| value.to_str()) {
        match argument {
            "--require-canonical-rust" => require_canonical_rust = true,
            "--lint-name-style" => lint_name_style = true,
            "--release" => release = true,
            "-o" | "--output" if command == "rust" && output_path.is_none() => {
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
    if release && !matches!(command, "build" | "run") {
        return Err(CliFailure::usage());
    }
    let has_valid_arity = if command == "run" {
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
    command: &str,
    crate_dir: &Path,
    target_dir: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    units: &[terrane_compiler::SourceUnit],
    has_rust_dependencies: bool,
    containment: terrane_compiler::projection::Containment,
    release: bool,
) -> Result<Option<PathBuf>, CliFailure> {
    if command == "check" {
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
    let mut rustflags = std::env::var_os("RUSTFLAGS").unwrap_or_default();
    if !rustflags.is_empty() {
        rustflags.push(" ");
    }
    rustflags.push("-Dwarnings");
    let contained =
        has_rust_dependencies && containment == terrane_compiler::projection::Containment::Enforced;
    if has_rust_dependencies {
        let mut fetch = Command::new("cargo");
        terrane_compiler::cargo_toolchain::configure_cargo_command(&mut fetch);
        let fetch = fetch
            .args(["fetch", "--manifest-path"])
            .arg(crate_dir.join("Cargo.toml"))
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

fn generated_crate_path(
    package_root: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    uses_platform_support: bool,
    rust_dependencies: &[terrane_compiler::RustDependency],
) -> Result<PathBuf, CliFailure> {
    let root = package_root.canonicalize().map_err(|error| {
        CliFailure::backend(format!(
            "cannot locate package root {}: {error}",
            package_root.display()
        ))
    })?;
    let mut hash = Sha256::new();
    hash.update(b"terrane-generated-crate-v2\0");
    hash.update(terrane_compiler::VERSION.as_bytes());
    for variable in [
        "CARGO_BUILD_TARGET",
        "CARGO_ENCODED_RUSTFLAGS",
        "RUSTC",
        "RUSTFLAGS",
    ] {
        hash.update(variable.as_bytes());
        hash.update(b"=");
        hash.update(std::env::var(variable).unwrap_or_default());
        hash.update(b"\0");
    }
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
    inactive.sort_by(|left, right| right.0.cmp(&left.0));
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

fn write_generated_crate(
    directory: &Path,
    rust_files: &[terrane_compiler::rust_ir::RenderedFile],
    units: &[terrane_compiler::SourceUnit],
    rust_dependencies: &[terrane_compiler::RustDependency],
    panic: terrane_compiler::PanicProfile,
    uses_platform_support: bool,
) -> Result<(), CliFailure> {
    fs::create_dir_all(directory.join("src"))
        .map_err(|error| CliFailure::backend(format!("cannot create generated crate: {error}")))?;
    let mut manifest = String::from(
        "[package]\nname = \"terrane_program\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n\
         [dependencies]\nterrane-int-support = { path = \"support/terrane-int-support\" }\n\
         terrane-collection-support = { path = \"support/terrane-collection-support\" }\n\
         terrane-scalar-support = { path = \"support/terrane-scalar-support\" }\n\
         terrane-string-support = { path = \"support/terrane-string-support\" }\n\
         terrane-document-support = { path = \"support/terrane-document-support\" }\n\
         terrane-stream-abi = { path = \"support/terrane-stream-abi\" }\n",
    );
    if uses_platform_support {
        manifest.push_str(
            "terrane-platform-support = { path = \"support/terrane-platform-support\" }\n",
        );
    }
    for dependency in rust_dependencies
        .iter()
        .filter(|dependency| dependency.cargo_manifest_table() == "dependencies")
    {
        write_rust_dependency(&mut manifest, dependency);
    }
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
    if panic == terrane_compiler::PanicProfile::Abort {
        manifest.push_str("\n[profile.dev]\npanic = \"abort\"\n");
    }
    manifest.push_str("\n[profile.release]\nopt-level = 3\nlto = \"fat\"\ncodegen-units = 1\n");
    manifest.push_str("\n[workspace]\n");
    write_if_changed(&directory.join("Cargo.toml"), manifest.as_bytes()).map_err(|error| {
        CliFailure::backend(format!("cannot write generated manifest: {error}"))
    })?;
    write_generated_support(directory, uses_platform_support).map_err(|error| {
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
    write_if_changed(&directory.join("terrane-build.toml"), sources.as_bytes())
        .map_err(|error| CliFailure::backend(format!("cannot write build metadata: {error}")))?;
    Ok(())
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
    fs::create_dir_all(int.join("src"))?;
    fs::create_dir_all(collection.join("src"))?;
    fs::create_dir_all(scalar.join("src"))?;
    fs::create_dir_all(string.join("src"))?;
    fs::create_dir_all(document.join("src"))?;
    fs::create_dir_all(stream.join("src"))?;
    if let Some(platform) = &platform {
        fs::create_dir_all(platform.join("src"))?;
    }
    write_if_changed(
        &int.join("Cargo.toml"),
        b"[package]\nname = \"terrane-int-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nnum-bigint = { version = \"0.4\", features = [\"std\"] }\nnum-integer = \"0.1\"\nnum-traits = \"0.2\"\n",
    )?;
    write_if_changed(
        &collection.join("Cargo.toml"),
        b"[package]\nname = \"terrane-collection-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nindexmap = \"2\"\nterrane-int-support = { path = \"../terrane-int-support\" }\nunicode-segmentation = \"1\"\n",
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
        b"[package]\nname = \"terrane-scalar-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nterrane-int-support = { path = \"../terrane-int-support\" }\n",
    )?;
    write_if_changed(
        &scalar.join("src/lib.rs"),
        include_bytes!("../../terrane-scalar-support/src/lib.rs"),
    )?;
    write_if_changed(
        &string.join("Cargo.toml"),
        b"[package]\nname = \"terrane-string-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n# unicode-normalization permits tinyvec 1.13, whose alloc-only build fails on Rust 1.93.\ntinyvec = { version = \"=1.12.0\", features = [\"std\"] }\nunicode-casefold = \"0.2\"\nunicode-normalization = \"0.1\"\nunicode-segmentation = \"1\"\n",
    )?;
    write_if_changed(
        &string.join("src/lib.rs"),
        include_bytes!("../../terrane-string-support/src/lib.rs"),
    )?;
    write_if_changed(
        &document.join("Cargo.toml"),
        b"[package]\nname = \"terrane-document-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nserde = \"=1.0.219\"\nserde_json = { version = \"=1.0.143\", features = [\"arbitrary_precision\", \"unbounded_depth\"] }\nurl = \"=2.5.7\"\nyaml-rust2 = \"=0.10.4\"\n",
    )?;
    write_if_changed(
        &document.join("src/lib.rs"),
        include_bytes!("../../terrane-document-support/src/lib.rs"),
    )?;
    write_if_changed(
        &stream.join("Cargo.toml"),
        b"[package]\nname = \"terrane-stream-abi\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nlibc = \"0.2\"\nrustix = { version = \"1\", features = [\"fs\"] }\n",
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
    }
    Ok(())
}

fn write_if_changed(path: &Path, content: &[u8]) -> std::io::Result<()> {
    if fs::read(path).is_ok_and(|existing| existing == content) {
        return Ok(());
    }
    fs::write(path, content)
}
fn ensure_rust_toolchain() -> Result<(), CliFailure> {
    let status = Command::new("cargo")
        .arg("--version")
        .output()
        .map_err(|error| {
            CliFailure::diagnostic(
                PathBuf::from("<toolchain>"),
                "S9001",
                format!("Cargo is required to compile generated Rust: {error}"),
                4,
            )
        })?;
    if status.status.success() {
        Ok(())
    } else {
        Err(CliFailure::diagnostic(
            PathBuf::from("<toolchain>"),
            "S9001",
            "Cargo prerequisite check failed".to_owned(),
            4,
        ))
    }
}

fn usage() -> String {
    "usage: terrane <check|rust|build|run> [--require-canonical-rust] [--lint-name-style] \
     [--release] [--output <file>] <source.trn> [-- program arguments]\n\
     options:\n  --require-canonical-rust  fail unless lowering emits bundled-formatter output\n  \
     --lint-name-style  warn when authored declarations are not kebab-case\n  \
     --release  use Cargo's optimized release profile for build or run\n  \
     -o, --output <file>  write rust output and its support sidecar (rust only)\n\
     commands:\n  check  validate and compile generated Rust\n  rust   print generated Rust or write split files\n  \
     build  compile a native executable\n  run    compile and execute the program"
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
    fn release_is_available_only_for_native_build_and_run() {
        let build = [
            OsString::from("build"),
            OsString::from("--release"),
            OsString::from("package.toml"),
        ];
        assert_eq!(
            parse_input(&build, "build").unwrap_or_else(|_| panic!("release build should parse")),
            (PathBuf::from("package.toml"), None, false, false, true)
        );

        let check = [
            OsString::from("check"),
            OsString::from("--release"),
            OsString::from("package.toml"),
        ];
        assert!(parse_input(&check, "check").is_err());
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
    fn generated_cargo_manifest_configures_build_profiles() {
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
                terrane_compiler::PanicProfile::Abort,
                false,
            )
            .is_ok()
        );

        let manifest = fs::read_to_string(directory.join("Cargo.toml")).unwrap();
        assert!(manifest.contains("[profile.dev]\npanic = \"abort\"\n"));
        assert!(
            manifest
                .contains("[profile.release]\nopt-level = 3\nlto = \"fat\"\ncodegen-units = 1\n")
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
