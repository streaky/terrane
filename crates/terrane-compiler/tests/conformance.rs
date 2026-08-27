use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/conformance")
}

struct ConformanceBuild {
    root: PathBuf,
}

impl ConformanceBuild {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!("terrane-conformance-{}", std::process::id()));
        if root.exists() {
            fs::remove_dir_all(&root).unwrap();
        }
        fs::create_dir_all(root.join("src")).unwrap();
        write_support_crates(&root);
        Self { root }
    }

    fn write_manifest(&self) {
        fs::write(
            self.root.join("Cargo.toml"),
            r#"[package]
name = "terrane_conformance_program"
version = "0.0.0"
edition = "2024"

[dependencies]
terrane-int-support = { path = "support/terrane-int-support" }
terrane-collection-support = { path = "support/terrane-collection-support" }
terrane-scalar-support = { path = "support/terrane-scalar-support" }
terrane-string-support = { path = "support/terrane-string-support" }
terrane-document-support = { path = "support/terrane-document-support" }
terrane-stream-abi = { path = "support/terrane-stream-abi" }
terrane-platform-support = { path = "support/terrane-platform-support" }

[workspace]
"#,
        )
        .unwrap();
    }
}

impl Drop for ConformanceBuild {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn every_manifest_drives_a_conformance_case() {
    let manifests = manifests_below(&corpus());
    let build = ConformanceBuild::new();
    assert!(!manifests.is_empty());
    for manifest_path in manifests {
        let case = manifest_path.parent().unwrap();
        let manifest = fs::read_to_string(&manifest_path).unwrap();
        let phase = field(&manifest, "phase").unwrap();
        let status = field(&manifest, "status").unwrap();
        let entrypoint = field(&manifest, "entrypoint").unwrap_or("case.trn");
        let source_path = case.join(entrypoint);
        let package_case = entrypoint == terrane_compiler::MANIFEST_FILE_NAME;
        let options = terrane_compiler::CompilerOptions {
            require_canonical_rust: boolean_field(&manifest, "canonical-rust").unwrap_or(false),
        };

        match (phase, status) {
            ("run" | "check", "accept") => {
                let expected = fs::read_to_string(case.join("lower.rs")).unwrap();
                let (compilation, sources) = if package_case {
                    let package = terrane_compiler::Package::load(&source_path).unwrap();
                    let compilation =
                        terrane_compiler::compile_package_with_options(&package, options).unwrap();
                    let sources = compilation.sources.clone();
                    (compilation, sources)
                } else {
                    let source = fs::read_to_string(&source_path).unwrap();
                    let compilation =
                        terrane_compiler::compile_with_options(&source_path, source, options)
                            .unwrap();
                    let sources = compilation.sources.clone();
                    (compilation, sources)
                };
                if let Some(warnings_file) = field(&manifest, "warnings") {
                    let expected_warnings = fs::read_to_string(case.join(warnings_file)).unwrap();
                    let actual_warnings = compilation
                        .warnings
                        .iter()
                        .map(|warning| {
                            let source = warning
                                .primary
                                .and_then(|span| {
                                    sources.iter().find(|source| source.id() == span.file)
                                })
                                .unwrap_or(&compilation.source);
                            warning.render(source).replacen(
                                &source.path().display().to_string(),
                                &source.path().file_name().unwrap().to_string_lossy(),
                                1,
                            )
                        })
                        .collect::<String>();
                    assert_eq!(
                        actual_warnings,
                        expected_warnings,
                        "{} warnings",
                        case.display()
                    );
                }
                let normalized = compilation
                    .rust
                    .replace(terrane_compiler::VERSION, "<version>");
                assert_eq!(normalized, expected, "{}", case.display());
                compile_and_maybe_run(case, phase, &manifest, &compilation.rust, &build);
            }
            ("check", "reject") => {
                let code = field(&manifest, "code").unwrap();
                let diagnostics = if package_case {
                    let package = terrane_compiler::Package::load(&source_path).unwrap();
                    terrane_compiler::compile_package(&package)
                        .unwrap_err()
                        .diagnostics
                } else {
                    let source = fs::read_to_string(&source_path).unwrap();
                    terrane_compiler::compile(&source_path, source)
                        .unwrap_err()
                        .diagnostics
                };
                assert!(
                    diagnostics.iter().any(|diagnostic| diagnostic.code == code),
                    "{} did not report {code}: {diagnostics:?}",
                    case.display()
                );
            }
            _ => panic!(
                "unsupported conformance manifest {}: phase={phase}, status={status}",
                manifest_path.display()
            ),
        }
    }
}

fn compile_and_maybe_run(
    case: &Path,
    phase: &str,
    manifest: &str,
    rust: &str,
    build: &ConformanceBuild,
) {
    build.write_manifest();
    let build_dir = &build.root;
    fs::write(build_dir.join("src/main.rs"), rust).unwrap();
    let output = Command::new("cargo")
        .args(["build", "--quiet", "--manifest-path"])
        .arg(build_dir.join("Cargo.toml"))
        .env("RUSTFLAGS", "-Dwarnings")
        .output()
        .unwrap();
    let mut binary_path = build_dir.join("target/debug/terrane_conformance_program");
    binary_path.set_extension(std::env::consts::EXE_EXTENSION);
    assert!(
        output.status.success(),
        "{} generated Rust failed to compile:\n{}",
        case.display(),
        String::from_utf8_lossy(&output.stderr)
    );

    if phase == "run" {
        let mut command = Command::new(&binary_path);
        if let Some(arguments) = optional_text(case.join("arguments.txt")) {
            command.args(arguments.lines());
        }
        command.args(platform_arguments(case.join("arguments-raw.hex")));
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
        if let Err(error) = child
            .stdin
            .take()
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
        let output = child.wait_with_output().unwrap();
        let expected_stdout = fs::read(case.join("stdout.txt")).unwrap();
        let expected_stderr = optional_bytes(case.join("stderr.txt"));
        let expected_code = optional_text(case.join("exit-code.txt"))
            .map_or(0, |text| text.trim().parse().unwrap());
        assert_eq!(output.stdout, expected_stdout, "{} stdout", case.display());
        assert_eq!(output.stderr, expected_stderr, "{} stderr", case.display());
        assert_eq!(
            output.status.code(),
            Some(expected_code),
            "{} exit code",
            case.display()
        );
    }
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
        "[package]\nname = \"terrane-collection-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nindexmap = \"2\"\nterrane-int-support = { path = \"../terrane-int-support\" }\nunicode-segmentation = \"1\"\n",
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
        "[package]\nname = \"terrane-string-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nunicode-casefold = \"0.2\"\nunicode-normalization = \"0.1\"\nunicode-segmentation = \"1\"\n",
    )
    .unwrap();
    fs::write(
        string.join("src/lib.rs"),
        include_bytes!("../../terrane-string-support/src/lib.rs"),
    )
    .unwrap();
    fs::write(
        document.join("Cargo.toml"),
        "[package]\nname = \"terrane-document-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nserde = \"=1.0.219\"\nserde_json = { version = \"=1.0.143\", features = [\"arbitrary_precision\", \"unbounded_depth\"] }\nurl = \"=2.5.7\"\nyaml-rust2 = \"=0.10.4\"\n",
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
        "[package]\nname = \"terrane-platform-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nbase64 = \"0.22\"\nflate2 = \"1\"\ngetrandom = \"0.3\"\nhmac = \"0.12\"\nrand_chacha = \"0.3\"\nrand_core = \"0.6\"\nrustls = { version = \"0.23\", default-features = false, features = [\"aws_lc_rs\", \"std\", \"tls12\"] }\nsha2 = \"0.10\"\nsubtle = \"2\"\nuuid = { version = \"1\", features = [\"v4\", \"v7\"] }\nwebpki-roots = \"1\"\nzeroize = \"1\"\nzstd = \"0.13\"\n",
    )
    .unwrap();
    fs::write(
        platform.join("src/lib.rs"),
        include_bytes!("../../terrane-platform-support/src/lib.rs"),
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
            let bytes = line
                .as_bytes()
                .chunks_exact(2)
                .map(|pair| {
                    let text = std::str::from_utf8(pair).expect("argument hex is ASCII");
                    u8::from_str_radix(text, 16).expect("argument bytes use hexadecimal")
                })
                .collect::<Vec<_>>();
            assert_eq!(line.len(), bytes.len() * 2, "argument hex has an odd digit");
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
