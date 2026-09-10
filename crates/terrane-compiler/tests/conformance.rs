use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::io::{BufRead as _, BufReader, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance")
}

struct ConformanceBuild {
    root: PathBuf,
    target: PathBuf,
}

impl ConformanceBuild {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!("terrane-conformance-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        fs::create_dir_all(root.join("src")).unwrap();
        write_support_crates(&root);
        let target = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/conformance");
        Self { root, target }
    }

    fn write_manifest(
        &self,
        binary_names: &[&str],
        dependencies: &[terrane_compiler::RustDependency],
    ) {
        let mut manifest = r#"[package]
name = "terrane_conformance_harness"
version = "0.0.0"
edition = "2024"
autobins = false

[dependencies]
terrane-int-support = { path = "support/terrane-int-support" }
terrane-collection-support = { path = "support/terrane-collection-support" }
terrane-scalar-support = { path = "support/terrane-scalar-support" }
terrane-string-support = { path = "support/terrane-string-support" }
terrane-document-support = { path = "support/terrane-document-support" }
terrane-stream-abi = { path = "support/terrane-stream-abi" }
terrane-platform-support = { path = "support/terrane-platform-support" }
tokio = { version = "=1.53.0", features = ["rt", "rt-multi-thread", "time"] }
"#
        .to_owned();
        for dependency in dependencies
            .iter()
            .filter(|dependency| dependency.cargo_manifest_table() == "dependencies")
        {
            manifest.push_str(&dependency.cargo_dependency_spec());
        }
        let target_tables = dependencies
            .iter()
            .map(terrane_compiler::RustDependency::cargo_manifest_table)
            .filter(|table| table != "dependencies")
            .collect::<BTreeSet<_>>();
        for table in target_tables {
            writeln!(manifest, "\n[{table}]").unwrap();
            for dependency in dependencies
                .iter()
                .filter(|dependency| dependency.cargo_manifest_table() == table)
            {
                manifest.push_str(&dependency.cargo_dependency_spec());
            }
        }
        // Binary tables must follow every dependency table so they do not terminate a
        // target-specific dependency section.
        for binary_name in binary_names {
            write!(
                manifest,
                "\n[[bin]]\nname = \"{binary_name}\"\npath = \"src/{binary_name}.rs\"\n"
            )
            .unwrap();
        }
        manifest.push_str("\n[workspace]\n");
        fs::write(self.root.join("Cargo.toml"), manifest).unwrap();
    }
}

impl Drop for ConformanceBuild {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[derive(Clone, Copy)]
enum TimingStatus {
    Failed,
    Ignored,
    Passed,
}

impl TimingStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Ignored => "ignored",
            Self::Passed => "passed",
        }
    }
}

struct CaseTiming {
    output: Option<PathBuf>,
    name: String,
    elapsed: std::time::Duration,
    started: Option<std::time::Instant>,
    status: TimingStatus,
}

impl CaseTiming {
    fn new(case: &Path) -> Self {
        Self::with_output(
            case.strip_prefix(corpus())
                .unwrap_or(case)
                .to_string_lossy()
                .replace('\\', "/"),
            std::env::var_os("TERRANE_TEST_TIMING_FILE").map(PathBuf::from),
        )
    }

    fn named(name: &str) -> Self {
        Self::with_output(
            name.to_owned(),
            std::env::var_os("TERRANE_TEST_TIMING_FILE").map(PathBuf::from),
        )
    }

    fn with_output(name: String, output: Option<PathBuf>) -> Self {
        Self {
            output,
            name,
            elapsed: std::time::Duration::ZERO,
            started: Some(std::time::Instant::now()),
            status: TimingStatus::Failed,
        }
    }

    fn pause(&mut self) {
        if let Some(started) = self.started.take() {
            self.elapsed += started.elapsed();
        }
    }

    fn defer(&mut self) {
        self.pause();
        self.status = TimingStatus::Ignored;
    }

    fn begin_pending_work(&mut self) {
        let previous = self.started.replace(std::time::Instant::now());
        assert!(previous.is_none());
        self.status = TimingStatus::Failed;
    }

    fn fail(&mut self) {
        self.pause();
        self.status = TimingStatus::Failed;
    }

    fn pass(&mut self) {
        self.pause();
        self.status = TimingStatus::Passed;
    }
}

impl Drop for CaseTiming {
    fn drop(&mut self) {
        self.pause();
        let Some(output) = &self.output else {
            return;
        };
        let Ok(mut output) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(output)
        else {
            return;
        };
        let status = self.status.as_str();
        let _ = writeln!(
            output,
            "terrane-test-timing-v1\tconformance\t{}\t{status}\t{:.6}",
            self.name,
            self.elapsed.as_secs_f64()
        );
    }
}

struct DeferredGeneratedCase {
    binary_name: String,
    case: PathBuf,
    should_run: bool,
    run_manifest: Option<String>,
    timing: CaseTiming,
}

impl DeferredGeneratedCase {
    fn new(
        binary_name: String,
        case: &Path,
        should_run: bool,
        manifest: String,
        mut timing: CaseTiming,
    ) -> Self {
        timing.defer();
        Self {
            binary_name,
            case: case.to_owned(),
            should_run,
            run_manifest: should_run.then_some(manifest),
            timing,
        }
    }
}

fn copy_package_fixture(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name();
        if name == ".trn" || name == "case.toml" || name == "lower.rs" {
            continue;
        }
        let source_path = entry.path();
        let destination_path = destination.join(name);
        if source_path.is_dir() {
            copy_package_fixture(&source_path, &destination_path);
        } else {
            fs::copy(source_path, destination_path).unwrap();
        }
    }
}

fn case_source_path(build: &ConformanceBuild, case: &Path, entrypoint: &str) -> PathBuf {
    if entrypoint != terrane_compiler::MANIFEST_FILE_NAME {
        return case.join(entrypoint);
    }
    let staged = build.root.join("package-input").join(
        case.file_name()
            .expect("conformance case directory must have a name"),
    );
    if staged.exists() {
        fs::remove_dir_all(&staged).unwrap();
    }
    copy_package_fixture(case, &staged);
    let fixture_registry = staged.join("fixture-registry");
    if fixture_registry.is_dir() {
        let dependency_workspace = staged.join(".trn/dependencies");
        copy_package_fixture(
            &fixture_registry,
            &dependency_workspace.join("fixture-registry"),
        );
        copy_package_fixture(&staged.join(".cargo"), &dependency_workspace.join(".cargo"));
    }
    staged.join(entrypoint)
}

fn reports(
    diagnostics: &[terrane_compiler::Diagnostic],
    code: &str,
    expected: Option<&str>,
) -> bool {
    diagnostics.iter().any(|diagnostic| {
        diagnostic.code == code && expected.is_none_or(|text| diagnostic.message.contains(text))
    })
}

fn assert_expected_warnings(
    case: &Path,
    manifest: &str,
    compilation: &terrane_compiler::Compilation,
) {
    let Some(warnings_file) = field(manifest, "warnings") else {
        return;
    };
    let expected = fs::read_to_string(case.join(warnings_file)).unwrap();
    let actual = compilation
        .warnings
        .iter()
        .map(|warning| {
            let source = warning
                .primary
                .and_then(|span| {
                    compilation
                        .sources
                        .iter()
                        .find(|source| source.id() == span.file)
                })
                .unwrap_or(&compilation.source);
            warning.render(source).replacen(
                &source.path().display().to_string(),
                &source.path().file_name().unwrap().to_string_lossy(),
                1,
            )
        })
        .collect::<String>();
    assert_eq!(actual, expected, "{} warnings", case.display());
}

fn with_compilation_dependencies(
    compilation: terrane_compiler::Compilation,
) -> (
    terrane_compiler::Compilation,
    Vec<terrane_compiler::RustDependency>,
) {
    let dependencies = compilation.rust_dependencies.clone();
    (compilation, dependencies)
}

#[test]
fn every_manifest_drives_a_conformance_case() {
    let manifests = manifests_below(&corpus());
    let build = ConformanceBuild::new();
    let mut deferred_generated_cases = Vec::new();
    assert!(!manifests.is_empty());
    for (case_index, manifest_path) in manifests.into_iter().enumerate() {
        let binary_name = format!("terrane_conformance_case_{case_index}");
        let case = manifest_path.parent().unwrap();
        let mut timing = CaseTiming::new(case);
        let manifest = fs::read_to_string(&manifest_path).unwrap();
        let phase = field(&manifest, "phase").unwrap();
        let status = field(&manifest, "status").unwrap();
        let entrypoint = field(&manifest, "entrypoint").unwrap_or("case.trn");
        let package_case = entrypoint == terrane_compiler::MANIFEST_FILE_NAME;
        let source_path = case_source_path(&build, case, entrypoint);
        let options = terrane_compiler::CompilerOptions {
            require_canonical_rust: boolean_field(&manifest, "canonical-rust").unwrap_or(false),
            lint_name_style: false,
        };

        match (phase, status) {
            ("run" | "check", "accept") => {
                let expected = fs::read_to_string(case.join("lower.rs")).unwrap();
                let (compilation, dependencies) = if package_case {
                    let package = terrane_compiler::Package::load(&source_path).unwrap();
                    let compilation =
                        terrane_compiler::compile_package_with_options(&package, options).unwrap();
                    verify_reviewed_projection(case, &source_path);
                    with_compilation_dependencies(compilation)
                } else {
                    let source = fs::read_to_string(&source_path).unwrap();
                    let compilation =
                        terrane_compiler::compile_with_options(&source_path, source, options)
                            .unwrap();
                    (compilation, Vec::new())
                };
                assert_expected_warnings(case, &manifest, &compilation);
                let normalized = compilation
                    .rust
                    .replace(terrane_compiler::VERSION, "<version>");
                if std::env::var_os("TERRANE_UPDATE_GOLDENS").is_some() {
                    fs::write(case.join("lower.rs"), &normalized).unwrap();
                } else {
                    assert_eq!(normalized, expected, "{}", case.display());
                }
                // Only dependency-free binaries can safely share one Cargo manifest and build.
                if dependencies.is_empty() && field(&manifest, "dependency-panic-test").is_none() {
                    stage_generated_binary(
                        &binary_name,
                        case,
                        &manifest,
                        &compilation.rust,
                        &build,
                    );
                    let should_run = phase == "run";
                    deferred_generated_cases.push(DeferredGeneratedCase::new(
                        binary_name,
                        case,
                        should_run,
                        manifest,
                        timing,
                    ));
                    continue;
                }
                compile_and_maybe_run(
                    &binary_name,
                    case,
                    phase,
                    &manifest,
                    &compilation.rust,
                    &dependencies,
                    &build,
                );
            }
            ("check", "reject") => {
                let code = field(&manifest, "code").unwrap();
                let diagnostics = if package_case {
                    let package = terrane_compiler::Package::load(&source_path).unwrap();
                    let result = terrane_compiler::compile_package(&package);
                    verify_reviewed_projection(case, &source_path);
                    result.unwrap_err().diagnostics
                } else {
                    let source = fs::read_to_string(&source_path).unwrap();
                    terrane_compiler::compile(&source_path, source)
                        .unwrap_err()
                        .diagnostics
                };
                let expected = field(&manifest, "contains");
                let reported = reports(&diagnostics, code, expected);
                assert!(
                    reported,
                    "{} did not report {code} matching {expected:?}: {diagnostics:?}",
                    case.display()
                );
            }
            _ => panic!(
                "unsupported conformance manifest {}: phase={phase}, status={status}",
                manifest_path.display()
            ),
        }
        timing.pass();
    }
    compile_and_run_deferred_cases(&mut deferred_generated_cases, &build);
}

fn stage_generated_binary(
    binary_name: &str,
    case: &Path,
    manifest: &str,
    rust: &str,
    build: &ConformanceBuild,
) {
    let fixture_registry = case.join("fixture-registry");
    if fixture_registry.is_dir() {
        copy_package_fixture(&fixture_registry, &build.root.join("fixture-registry"));
        copy_package_fixture(&case.join(".cargo"), &build.root.join(".cargo"));
    }
    let rust = if let Some(test_path) = field(manifest, "dependency-panic-test") {
        format!(
            "{rust}\n{}",
            fs::read_to_string(case.join(test_path)).unwrap_or_else(|error| panic!(
                "cannot read {}: {error}",
                case.join(test_path).display()
            ))
        )
    } else {
        rust.to_owned()
    };
    fs::write(build.root.join(format!("src/{binary_name}.rs")), rust).unwrap();
}

fn build_generated_binaries(
    binary_names: &[&str],
    dependencies: &[terrane_compiler::RustDependency],
    build: &ConformanceBuild,
) -> std::process::Output {
    build.write_manifest(binary_names, dependencies);
    Command::new("cargo")
        .arg(format!("+{}", terrane_compiler::BUILD_TOOLCHAIN))
        .args(["build", "--quiet", "--manifest-path"])
        .arg(build.root.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", &build.target)
        .env("RUSTFLAGS", "-Dwarnings")
        .current_dir(&build.root)
        .output()
        .unwrap()
}

fn binary_path(binary_name: &str, build: &ConformanceBuild) -> PathBuf {
    let mut path = build.target.join("debug").join(binary_name);
    path.set_extension(std::env::consts::EXE_EXTENSION);
    path
}

fn stderr_mentions_binary(stderr: &str, binary_name: &str) -> bool {
    stderr.contains(&format!("{binary_name}.rs"))
        || stderr.contains(&format!("(bin \"{binary_name}\")"))
}

fn compile_and_run_deferred_cases(cases: &mut [DeferredGeneratedCase], build: &ConformanceBuild) {
    if cases.is_empty() {
        return;
    }
    let cargo_configuration = build.root.join(".cargo");
    if cargo_configuration.exists() {
        fs::remove_dir_all(cargo_configuration).unwrap();
    }
    let binary_names = cases
        .iter()
        .map(|case| case.binary_name.as_str())
        .collect::<Vec<_>>();
    let mut build_timing = CaseTiming::named("generated-rust/batched-dependency-free-build");
    let output = build_generated_binaries(&binary_names, &[], build);
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut mapping = Vec::new();
        for case in &mut *cases {
            if stderr_mentions_binary(&stderr, &case.binary_name) {
                case.timing.fail();
                mapping.push(format!("{}: {}", case.binary_name, case.case.display()));
            }
        }
        let mapping = if mapping.is_empty() {
            "Cargo did not identify a case-specific binary".to_owned()
        } else {
            mapping.join("\n")
        };
        panic!(
            "batched generated Rust failed to compile:\n{stderr}\n\nimplicated cases:\n{mapping}"
        );
    }
    build_timing.pass();
    for case in cases {
        if case.should_run {
            case.timing.begin_pending_work();
            run_case(
                &binary_path(&case.binary_name, build),
                &build.root,
                &case.case,
                case.run_manifest
                    .as_deref()
                    .expect("run cases retain their manifest"),
            );
        }
        case.timing.pass();
    }
}

fn compile_and_maybe_run(
    binary_name: &str,
    case: &Path,
    phase: &str,
    manifest: &str,
    rust: &str,
    dependencies: &[terrane_compiler::RustDependency],
    build: &ConformanceBuild,
) {
    stage_generated_binary(binary_name, case, manifest, rust, build);
    let output = build_generated_binaries(&[binary_name], dependencies, build);
    assert!(
        output.status.success(),
        "{} generated Rust failed to compile:\n{}",
        case.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    if field(manifest, "dependency-panic-test").is_some() {
        let output = Command::new("cargo")
            .arg(format!("+{}", terrane_compiler::BUILD_TOOLCHAIN))
            .args(["test", "--quiet", "--manifest-path"])
            .arg(build.root.join("Cargo.toml"))
            .env("CARGO_TARGET_DIR", &build.target)
            .env("RUSTFLAGS", "-Dwarnings")
            .current_dir(&build.root)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{} generated dependency panic test failed:\n{}",
            case.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    if phase == "run" {
        run_case(
            &binary_path(binary_name, build),
            &build.root,
            case,
            manifest,
        );
    }
}
fn verify_reviewed_projection(case: &Path, source_path: &Path) {
    let reviewed_path = case.join("terrane-projection.lock");
    if !reviewed_path.is_file() {
        return;
    }
    let staged_path = source_path
        .parent()
        .expect("package entrypoint must have a parent")
        .join("terrane-projection.lock");
    let staged = stable_projection_history(&staged_path);
    if std::env::var_os("TERRANE_UPDATE_GOLDENS").is_some() {
        let mut reviewed = serde_json::to_string_pretty(&staged).unwrap();
        reviewed.push('\n');
        fs::write(reviewed_path, reviewed).unwrap();
        return;
    }
    assert_eq!(
        staged,
        stable_projection_history(&reviewed_path),
        "{} changed its reviewed projection semantics",
        case.display()
    );
}

fn stable_projection_history(path: &Path) -> serde_json::Value {
    let bytes = fs::read(path).unwrap_or_else(|error| {
        panic!(
            "cannot read reviewed projection {}: {error}",
            path.display()
        )
    });
    let mut history = serde_json::from_slice::<serde_json::Value>(&bytes)
        .unwrap_or_else(|error| panic!("cannot decode projection {}: {error}", path.display()));
    let object = history
        .as_object_mut()
        .unwrap_or_else(|| panic!("projection {} must contain a JSON object", path.display()));
    object.remove("cache_identity");
    object.remove("content_hash");
    history
}

fn run_case(binary_path: &Path, build_dir: &Path, case: &Path, manifest: &str) {
    let mut command = Command::new(binary_path);
    if let Some(arguments) = optional_text(case.join("arguments.txt")) {
        command.args(arguments.lines());
    }
    command.args(platform_arguments(case.join("arguments-raw.hex")));
    if let Some(worker_threads) = field(manifest, "worker-threads") {
        command.env("TOKIO_WORKER_THREADS", worker_threads);
    }
    if boolean_field(manifest, "isolated-working-directory") == Some(true) {
        let working_directory = build_dir.join("run");
        if working_directory.exists() {
            fs::remove_dir_all(&working_directory).unwrap();
        }
        fs::create_dir(&working_directory).unwrap();
        if let (Some(link), Some(target)) = (
            field(manifest, "symlink-fixture"),
            field(manifest, "symlink-target"),
        ) {
            create_file_symlink(target, working_directory.join(link));
        }
        command.current_dir(working_directory);
    }
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = Some(child.stdin.take().unwrap());
    if let Err(error) = stdin
        .as_mut()
        .unwrap()
        .write_all(&optional_bytes(case.join("stdin.txt")))
    {
        assert_eq!(
            error.kind(),
            std::io::ErrorKind::BrokenPipe,
            "{} could not receive conformance stdin",
            case.display()
        );
    }
    let hold_stdin = boolean_field(manifest, "hold-stdin-until-stdout") == Some(true);
    if !hold_stdin {
        drop(stdin.take());
    }
    let (status, stdout, stderr) = if hold_stdin {
        let mut stdout = BufReader::new(child.stdout.take().unwrap());
        let mut stdout_bytes = Vec::new();
        stdout.read_until(b'\n', &mut stdout_bytes).unwrap();
        drop(stdin.take());
        stdout.read_to_end(&mut stdout_bytes).unwrap();
        let mut stderr = child.stderr.take().unwrap();
        let mut stderr_bytes = Vec::new();
        stderr.read_to_end(&mut stderr_bytes).unwrap();
        (child.wait().unwrap(), stdout_bytes, stderr_bytes)
    } else {
        let output = child.wait_with_output().unwrap();
        (output.status, output.stdout, output.stderr)
    };
    let expected_stdout = fs::read(case.join("stdout.txt")).unwrap();
    let expected_stderr = optional_bytes(case.join("stderr.txt"));
    let expected_code =
        optional_text(case.join("exit-code.txt")).map_or(0, |text| text.trim().parse().unwrap());
    assert_eq!(stdout, expected_stdout, "{} stdout", case.display());
    assert_eq!(stderr, expected_stderr, "{} stderr", case.display());
    assert_eq!(
        status.code(),
        Some(expected_code),
        "{} exit code",
        case.display()
    );
}

#[cfg(unix)]
fn create_file_symlink(target: &str, link: PathBuf) {
    std::os::unix::fs::symlink(target, link).unwrap();
}

#[cfg(windows)]
fn create_file_symlink(target: &str, link: PathBuf) {
    std::os::windows::fs::symlink_file(target, link).unwrap();
}

fn write_support_crates(directory: &Path) {
    let int = directory.join("support/terrane-int-support");
    let collection = directory.join("support/terrane-collection-support");
    let scalar = directory.join("support/terrane-scalar-support");
    let string = directory.join("support/terrane-string-support");
    let document = directory.join("support/terrane-document-support");
    let stream = directory.join("support/terrane-stream-abi");
    let platform = directory.join("support/terrane-platform-support");
    fs::create_dir_all(int.join("src")).unwrap();
    fs::create_dir_all(collection.join("src")).unwrap();
    fs::create_dir_all(scalar.join("src")).unwrap();
    fs::create_dir_all(string.join("src")).unwrap();
    fs::create_dir_all(document.join("src")).unwrap();
    fs::create_dir_all(stream.join("src")).unwrap();
    fs::create_dir_all(platform.join("src")).unwrap();
    fs::write(
        int.join("Cargo.toml"),
        "[package]\nname = \"terrane-int-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nnum-bigint = { version = \"0.4\", features = [\"std\"] }\nnum-integer = \"0.1\"\nnum-traits = \"0.2\"\n",
    )
    .unwrap();
    fs::write(
        collection.join("Cargo.toml"),
        "[package]\nname = \"terrane-collection-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nindexmap = \"2\"\nterrane-int-support = { path = \"../terrane-int-support\" }\nunicode-segmentation = \"=1.12.0\"\n",
    )
    .unwrap();
    fs::write(
        collection.join("src/lib.rs"),
        include_bytes!("../../terrane-collection-support/src/lib.rs"),
    )
    .unwrap();
    fs::write(
        int.join("src/lib.rs"),
        include_bytes!("../../terrane-int-support/src/lib.rs"),
    )
    .unwrap();
    fs::write(
        scalar.join("Cargo.toml"),
        "[package]\nname = \"terrane-scalar-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nterrane-int-support = { path = \"../terrane-int-support\" }\n",
    )
    .unwrap();
    fs::write(
        scalar.join("src/lib.rs"),
        include_bytes!("../../terrane-scalar-support/src/lib.rs"),
    )
    .unwrap();
    fs::write(
        string.join("Cargo.toml"),
        "[package]\nname = \"terrane-string-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n# Unicode 16.0 profile: case folding, normalization, and segmentation move together.\ncaseless = \"=0.2.2\"\n# unicode-normalization permits tinyvec 1.13, whose alloc-only build fails on Rust 1.93.\ntinyvec = { version = \"=1.12.0\", features = [\"std\"] }\nunicode-normalization = \"=0.1.24\"\nunicode-segmentation = \"=1.12.0\"\n",
    )
    .unwrap();
    fs::write(
        string.join("src/lib.rs"),
        include_bytes!("../../terrane-string-support/src/lib.rs"),
    )
    .unwrap();
    fs::write(
        document.join("Cargo.toml"),
        "[package]\nname = \"terrane-document-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nserde = \"1\"\nserde_json = { version = \"1\", features = [\"arbitrary_precision\", \"unbounded_depth\"] }\nurl = \"=2.5.7\"\nyaml-rust2 = \"=0.10.4\"\n",
    )
    .unwrap();
    fs::write(
        document.join("src/lib.rs"),
        include_bytes!("../../terrane-document-support/src/lib.rs"),
    )
    .unwrap();
    fs::write(
        stream.join("Cargo.toml"),
        "[package]\nname = \"terrane-stream-abi\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nlibc = \"0.2\"\nrustix = { version = \"1\", features = [\"fs\"] }\n",
    )
    .unwrap();
    fs::write(
        stream.join("src/lib.rs"),
        include_bytes!("../../terrane-stream-abi/src/lib.rs"),
    )
    .unwrap();
    fs::write(
        platform.join("Cargo.toml"),
        terrane_compiler::platform_support_manifest(),
    )
    .unwrap();
    fs::write(
        platform.join("src/lib.rs"),
        include_bytes!("../../terrane-platform-support/src/lib.rs"),
    )
    .unwrap();
    fs::write(
        platform.join("src/observability.rs"),
        include_bytes!("../../terrane-platform-support/src/observability.rs"),
    )
    .unwrap();
}

fn optional_bytes(path: PathBuf) -> Vec<u8> {
    fs::read(path).unwrap_or_default()
}

fn optional_text(path: PathBuf) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn platform_arguments(path: PathBuf) -> Vec<std::ffi::OsString> {
    let Some(encoded) = optional_text(path) else {
        return Vec::new();
    };
    encoded
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (pairs, remainder) = line.as_bytes().as_chunks::<2>();
            assert!(remainder.is_empty(), "argument hex has an odd digit");
            let bytes = pairs
                .iter()
                .map(|pair| {
                    let text = std::str::from_utf8(pair).expect("argument hex is ASCII");
                    u8::from_str_radix(text, 16).expect("argument bytes use hexadecimal")
                })
                .collect::<Vec<_>>();
            #[cfg(unix)]
            {
                use std::os::unix::ffi::OsStringExt as _;
                std::ffi::OsString::from_vec(bytes)
            }
            #[cfg(windows)]
            {
                use std::os::windows::ffi::OsStringExt as _;
                assert_eq!(bytes.len() % 2, 0, "Windows arguments use UTF-16LE units");
                let units = bytes
                    .chunks_exact(2)
                    .map(|pair| u16::from_le_bytes([pair[0], pair[1]]));
                std::ffi::OsString::from_wide(&units.collect::<Vec<_>>())
            }
        })
        .collect()
}

fn manifests_below(root: &Path) -> Vec<PathBuf> {
    let mut manifests = Vec::new();
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            manifests.extend(manifests_below(&path));
        } else if path.file_name().is_some_and(|name| name == "case.toml") {
            manifests.push(path);
        }
    }
    manifests.sort();
    manifests
}

fn field<'manifest>(manifest: &'manifest str, name: &str) -> Option<&'manifest str> {
    manifest.lines().find_map(|line| {
        line.strip_prefix(name)?
            .strip_prefix(" = \"")?
            .strip_suffix('"')
    })
}

fn boolean_field(manifest: &str, name: &str) -> Option<bool> {
    manifest.lines().find_map(|line| {
        let value = line.strip_prefix(name)?.strip_prefix(" = ")?;
        match value {
            "true" => Some(true),
            "false" => Some(false),
            _ => None,
        }
    })
}

#[cfg(unix)]
#[test]
fn raw_argument_fixture_preserves_non_utf8_bytes() {
    use std::os::unix::ffi::OsStrExt as _;
    let root = std::env::temp_dir().join(format!("terrane-raw-arguments-{}", std::process::id()));
    fs::write(&root, "ff0061\n").unwrap();
    let values = platform_arguments(root.clone());
    assert_eq!(values[0].as_os_str().as_bytes(), &[0xff, 0x00, b'a']);
    fs::remove_file(root).unwrap();
}

#[test]
fn canonical_rust_manifest_expectation_is_opt_in() {
    assert_eq!(
        boolean_field("phase = \"run\"\ncanonical-rust = true\n", "canonical-rust"),
        Some(true)
    );
    assert_eq!(boolean_field("phase = \"run\"\n", "canonical-rust"), None);
}

#[test]
fn deferred_timing_does_not_report_an_unrelated_failure() {
    let output = std::env::temp_dir().join(format!(
        "terrane-deferred-timing-{}.txt",
        std::process::id()
    ));
    {
        let _ = fs::remove_file(&output);
        let mut timing = CaseTiming::with_output("deferred-case".to_owned(), Some(output.clone()));
        timing.defer();
    }
    let record = fs::read_to_string(&output).unwrap();
    assert!(
        record.starts_with("terrane-test-timing-v1\tconformance\tdeferred-case\tignored\t"),
        "{record:?}"
    );
    fs::remove_file(output).unwrap();
}

#[test]
fn compile_failure_attribution_distinguishes_binary_name_prefixes() {
    let rustc_stderr = " --> src/terrane_conformance_case_12.rs:1:26\n\
                        error: could not compile `probe` (bin \"terrane_conformance_case_12\")";
    let linker_stderr = "error: linking with `cc` failed: exit status: 1\n\
                         error: could not compile `probe` (bin \"terrane_conformance_case_12\")";
    for stderr in [rustc_stderr, linker_stderr] {
        assert!(!stderr_mentions_binary(
            stderr,
            "terrane_conformance_case_1"
        ));
        assert!(stderr_mentions_binary(
            stderr,
            "terrane_conformance_case_12"
        ));
    }
}
