use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_BUILD_ID: AtomicUsize = AtomicUsize::new(0);

/// Dependency resolution may publish a package lock, but corpus fixtures are inputs.
/// Keep resolved Cargo locks in `.trn` without changing the reviewed root lock.
pub(super) struct FixtureDependencyLock {
    path: PathBuf,
    original: Option<Vec<u8>>,
}

impl FixtureDependencyLock {
    pub(super) fn preserve(root: &Path) -> Self {
        let path = root.join("terrane-dependencies.lock");
        let original = match fs::read(&path) {
            Ok(bytes) => Some(bytes),
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            Err(error) => panic!("cannot read fixture dependency lock: {error}"),
        };
        Self { path, original }
    }
}

impl Drop for FixtureDependencyLock {
    fn drop(&mut self) {
        if let Some(original) = &self.original {
            write_if_changed(&self.path, original).expect("restore fixture dependency lock");
        } else if let Err(error) = fs::remove_file(&self.path)
            && error.kind() != io::ErrorKind::NotFound
        {
            panic!("cannot remove generated fixture dependency lock: {error}");
        }
    }
}

pub(super) struct ConformanceBuild {
    pub(super) root: PathBuf,
    pub(super) target: PathBuf,
    pub(super) support: PathBuf,
    lock: Option<fs::File>,
    retained: bool,
}

impl ConformanceBuild {
    pub(super) fn new() -> Self {
        let build_id = NEXT_BUILD_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "terrane-conformance-{}-{build_id}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove stale temporary conformance workspace");
        }
        fs::create_dir_all(root.join("src")).expect("create temporary conformance workspace");
        let support = root.join("support");
        write_support_crates(&support).expect("write temporary support crates");
        Self {
            target: target_directory(),
            root,
            support,
            lock: None,
            retained: false,
        }
    }

    pub(super) fn retained(case: &Path) -> Self {
        Self::retained_role(case, "conformance")
    }

    pub(super) fn retained_role(case: &Path, role: &str) -> Self {
        let state = case.join(".trn");
        let lock_path = state.join("locks").join(format!("{role}.lock"));
        fs::create_dir_all(lock_path.parent().expect("lock parent"))
            .expect("create retained workspace lock directory");
        let lock = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&lock_path)
            .expect("open retained workspace lock");
        lock.lock().expect("lock retained workspace");
        let root = state.join(role);
        let support = target_directory().join("support");
        fs::create_dir_all(root.join("src")).expect("create retained conformance workspace");
        fs::create_dir_all(&support).expect("create shared support directory");
        write_support_crates(&support).expect("write retained support crates");
        Self {
            target: target_directory(),
            root,
            support,
            lock: Some(lock),
            retained: true,
        }
    }

    pub(super) fn write_manifest(
        &self,
        binary_names: &[&str],
        dependencies: &[terrane_compiler::RustDependency],
        requires_unsafe_code: bool,
    ) {
        let mut manifest = String::from(
            "[package]\nname = \"terrane_conformance_harness\"\nversion = \"0.0.0\"\nedition = \"2024\"\nautobins = false\n\n[dependencies]\n",
        );
        for (name, path) in [
            (
                "terrane-int-support",
                self.support.join("terrane-int-support"),
            ),
            (
                "terrane-collection-support",
                self.support.join("terrane-collection-support"),
            ),
            (
                "terrane-scalar-support",
                self.support.join("terrane-scalar-support"),
            ),
            (
                "terrane-string-support",
                self.support.join("terrane-string-support"),
            ),
            (
                "terrane-document-support",
                self.support.join("terrane-document-support"),
            ),
            (
                "terrane-stream-abi",
                self.support.join("terrane-stream-abi"),
            ),
            (
                "terrane-platform-support",
                self.support.join("terrane-platform-support"),
            ),
        ] {
            let path = toml::Value::String(path.to_string_lossy().into_owned());
            writeln!(manifest, "{name} = {{ path = {path} }}").unwrap();
        }
        for dependency in terrane_compiler::with_tokio_runtime(
            dependencies,
            &["rt", "rt-multi-thread", "sync", "time"],
        )
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
        for binary_name in binary_names {
            write!(
                manifest,
                "\n[[bin]]\nname = \"{binary_name}\"\npath = \"src/{binary_name}.rs\"\n"
            )
            .unwrap();
        }
        writeln!(
            manifest,
            "\n[lints.rust]\nunsafe_code = \"{}\"",
            if requires_unsafe_code {
                "deny"
            } else {
                "forbid"
            }
        )
        .unwrap();
        manifest.push_str("\n[profile.dev]\ndebug = \"line-tables-only\"\nincremental = true\n\n[profile.test]\ndebug = \"line-tables-only\"\nincremental = true\n\n[workspace]\n");
        write_if_changed(self.root.join("Cargo.toml"), manifest)
            .expect("write conformance manifest");
    }
}

impl Drop for ConformanceBuild {
    fn drop(&mut self) {
        if self.retained {
            for runtime in [self.root.join("run"), self.root.join("suite-runtime")] {
                let _ = fs::remove_dir_all(runtime);
            }
        } else {
            let _ = fs::remove_dir_all(&self.root);
        }
        let _ = self.lock.take();
    }
}

fn target_directory() -> PathBuf {
    let target = if std::env::var_os("TERRANE_COVERAGE_DIR").is_some() {
        PathBuf::from(
            std::env::var_os("TERRANE_COVERAGE_TARGET_DIR").expect("coverage target directory"),
        )
        .join("conformance")
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/conformance")
    };
    fs::create_dir_all(&target).expect("create conformance target directory");
    fs::canonicalize(target).expect("resolve conformance target directory")
}

pub(super) fn write_if_changed(
    path: impl AsRef<Path>,
    contents: impl AsRef<[u8]>,
) -> io::Result<()> {
    let path = path.as_ref();
    let contents = contents.as_ref();
    if fs::read(path).is_ok_and(|existing| existing == contents) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
}

fn write_support_crates(directory: &Path) -> io::Result<()> {
    let int = directory.join("terrane-int-support");
    let collection = directory.join("terrane-collection-support");
    let scalar = directory.join("terrane-scalar-support");
    let string = directory.join("terrane-string-support");
    let document = directory.join("terrane-document-support");
    let stream = directory.join("terrane-stream-abi");
    for root in [&int, &collection, &scalar, &string, &document, &stream] {
        fs::create_dir_all(root.join("src"))?;
    }
    let collection_manifest = terrane_compiler::collection_support_manifest();
    let string_manifest = terrane_compiler::string_support_manifest();
    for (path, contents) in [
        (
            int.join("Cargo.toml"),
            "[package]\nname = \"terrane-int-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nnum-bigint = { version = \"0.4\", features = [\"std\"] }\nnum-integer = \"0.1\"\nnum-traits = \"0.2\"\n",
        ),
        (collection.join("Cargo.toml"), collection_manifest.as_str()),
        (
            scalar.join("Cargo.toml"),
            "[package]\nname = \"terrane-scalar-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nterrane-int-support = { path = \"../terrane-int-support\" }\n",
        ),
        (string.join("Cargo.toml"), string_manifest.as_str()),
        (
            document.join("Cargo.toml"),
            "[package]\nname = \"terrane-document-support\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nserde = \"1\"\nserde_json = { version = \"1\", features = [\"raw_value\"] }\njson-event-parser = \"=0.2.3\"\n# Keep this compatible with projected dependency graphs that select newer 2.5.x releases.\nurl = \"2.5\"\nyaml-rust2 = \"=0.10.4\"\n",
        ),
        (
            stream.join("Cargo.toml"),
            "[package]\nname = \"terrane-stream-abi\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nlibc = \"0.2\"\nrustix = { version = \"1\", features = [\"fs\"] }\n",
        ),
    ] {
        write_if_changed(path, contents)?;
    }
    for (path, contents) in [
        (
            int.join("src/lib.rs"),
            include_bytes!("../../../terrane-int-support/src/lib.rs").as_slice(),
        ),
        (
            collection.join("src/lib.rs"),
            include_bytes!("../../../terrane-collection-support/src/lib.rs").as_slice(),
        ),
        (
            scalar.join("src/lib.rs"),
            include_bytes!("../../../terrane-scalar-support/src/lib.rs").as_slice(),
        ),
        (
            string.join("src/lib.rs"),
            include_bytes!("../../../terrane-string-support/src/lib.rs").as_slice(),
        ),
        (
            document.join("src/lib.rs"),
            include_bytes!("../../../terrane-document-support/src/lib.rs").as_slice(),
        ),
        (
            document.join("src/native.rs"),
            include_bytes!("../../../terrane-document-support/src/native.rs").as_slice(),
        ),
        (
            document.join("src/native_serialize.rs"),
            include_bytes!("../../../terrane-document-support/src/native_serialize.rs").as_slice(),
        ),
        (
            stream.join("src/lib.rs"),
            include_bytes!("../../../terrane-stream-abi/src/lib.rs").as_slice(),
        ),
    ] {
        write_if_changed(path, contents)?;
    }
    terrane_compiler::write_platform_support(directory)?;
    Ok(())
}
