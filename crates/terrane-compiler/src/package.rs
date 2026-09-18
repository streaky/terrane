use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Diagnostic, SourceFile, Span};
use sha2::{Digest, Sha256};

pub const MANIFEST_FILE_NAME: &str = "package.toml";
pub const IMPLICIT_PACKAGE_ID: &str = "single-file";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceRole {
    Production,
    UnitTest,
    IntegrationTest,
    EndToEndTest,
    Bundled,
}
#[derive(Clone, Debug)]
pub struct SourceUnit {
    /// Normalized path relative to [`Package::root`].
    ///
    /// Package construction guarantees that this contains only ordinary path components.
    pub relative_path: PathBuf,
    pub source: SourceFile,
    pub expected_namespace: Option<String>,
    pub prelude: bool,
    pub role: SourceRole,
}

impl SourceUnit {
    pub(crate) fn relative_path_text(&self) -> String {
        self.relative_path.to_string_lossy().replace('\\', "/")
    }
}
#[derive(Clone, Debug)]
pub struct AuthoredRustModule {
    pub name: String,
    /// Normalized path relative to [`Package::root`].
    pub relative_path: PathBuf,
    pub source: SourceFile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReflectionProfile {
    Ordinary,
    Minimal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutorProfile {
    Cooperative,
    Threaded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanicProfile {
    Unwind,
    Abort,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildToolchain {
    Pinned,
    System,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactKind {
    Executable,
    DynamicLibrary,
    Library,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackagePurpose {
    Production,
    Testing,
}

const CAPABILITY_NAMES: [&str; 10] = [
    "build",
    "clocks",
    "entropy",
    "filesystem",
    "logging",
    "networking",
    "process",
    "process-signals",
    "threads",
    "tls",
];

fn is_capability_name(name: &str) -> bool {
    CAPABILITY_NAMES.contains(&name)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityProfile {
    pub name: String,
    pub capabilities: Option<BTreeSet<String>>,
    pub panic: PanicProfile,
}

impl CapabilityProfile {
    #[must_use]
    pub fn unrestricted() -> Self {
        Self {
            name: "default".to_owned(),
            capabilities: None,
            panic: PanicProfile::Unwind,
        }
    }

    #[must_use]
    pub fn allows(&self, capability: &str) -> bool {
        self.capabilities
            .as_ref()
            .is_none_or(|capabilities| capabilities.contains(capability))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RustDependency {
    pub name: String,
    pub package: String,
    pub version: String,
    pub features: Vec<String>,
    pub default_features: bool,
    pub target: Option<String>,
    pub effects: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerraneDependencySource {
    Path(PathBuf),
    Git { url: String, tag: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneDependency {
    pub name: String,
    pub source: TerraneDependencySource,
    pub hash: Option<String>,
}

/// Returns dependency declarations with the required Tokio runtime features merged into any
/// directly declared Tokio package, including dependencies declared under a Cargo alias.
#[must_use]
pub fn with_tokio_runtime(
    dependencies: &[RustDependency],
    required_features: &[&str],
) -> Vec<RustDependency> {
    let mut merged = dependencies.to_vec();
    if let Some(tokio) = merged.iter_mut().find(|dependency| {
        dependency.package == "tokio" && dependency.cargo_manifest_table() == "dependencies"
    }) {
        tokio.features.extend(
            required_features
                .iter()
                .map(|feature| (*feature).to_owned()),
        );
        tokio.features.sort();
        tokio.features.dedup();
    } else {
        let mut features = required_features
            .iter()
            .map(|feature| (*feature).to_owned())
            .collect::<Vec<_>>();
        features.sort();
        features.dedup();
        merged.push(RustDependency {
            name: "tokio".to_owned(),
            package: "tokio".to_owned(),
            version: "=1.53.0".to_owned(),
            features,
            default_features: true,
            target: None,
            effects: Vec::new(),
        });
    }
    merged
}

impl RustDependency {
    #[must_use]
    pub fn cargo_manifest_table(&self) -> String {
        self.target.as_ref().map_or_else(
            || "dependencies".to_owned(),
            |target| format!("target.{target:?}.dependencies"),
        )
    }

    #[must_use]
    pub fn cargo_dependency_spec(&self) -> String {
        use std::fmt::Write as _;

        let mut entry = format!(
            "{} = {{ package = {:?}, version = {:?}, default-features = {}",
            self.name, self.package, self.version, self.default_features
        );
        if !self.features.is_empty() {
            write!(entry, ", features = {:?}", self.features)
                .expect("writing to a string cannot fail");
        }
        entry.push_str(" }\n");
        entry
    }
}

#[derive(Clone, Debug)]
pub struct Package {
    pub identity: String,
    pub root: PathBuf,
    pub prelude: bool,
    pub reflection: ReflectionProfile,
    pub executor: ExecutorProfile,
    pub artifact: ArtifactKind,
    pub profile: CapabilityProfile,
    pub purpose: PackagePurpose,
    pub testing: crate::testing::TestConfiguration,
    pub build_toolchain: BuildToolchain,
    pub units: Vec<SourceUnit>,
    pub rust_dependencies: Vec<RustDependency>,
    pub authored_rust_modules: Vec<AuthoredRustModule>,
    pub terrane_dependencies: Vec<TerraneDependency>,
    pub dependency_manifests: Vec<PathBuf>,
    pub library_source_ids: BTreeSet<u32>,
}

#[derive(Clone, Debug)]
pub struct PackageLoadError {
    pub source: SourceFile,
    pub diagnostic: Diagnostic,
}

impl PackageLoadError {
    fn new(path: PathBuf, text: String, message: impl Into<String>, span: Option<Span>) -> Self {
        let source = SourceFile::new(0, path, text);
        let mut diagnostic = Diagnostic::unlocated_error("S2001", message);
        diagnostic.primary = span;
        Self { source, diagnostic }
    }

    fn unreadable(path: PathBuf, message: impl Into<String>) -> Self {
        Self::new(path, String::new(), message, None)
    }
}

impl Package {
    #[must_use]
    pub fn implicit(path: impl Into<PathBuf>, text: String) -> Self {
        let path = path.into();
        let root = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        let relative_path = path
            .strip_prefix(&root)
            .map_or_else(|_| path.clone(), Path::to_path_buf);
        Self {
            identity: IMPLICIT_PACKAGE_ID.to_owned(),
            root,
            prelude: true,
            reflection: ReflectionProfile::Ordinary,
            executor: ExecutorProfile::Threaded,
            artifact: ArtifactKind::Executable,
            profile: CapabilityProfile::unrestricted(),
            purpose: PackagePurpose::Production,
            testing: crate::testing::TestConfiguration::conventional(
                CapabilityProfile::unrestricted(),
            ),
            build_toolchain: BuildToolchain::Pinned,
            units: vec![SourceUnit {
                relative_path,
                source: SourceFile::new(0, path, text),
                expected_namespace: None,
                prelude: true,
                role: SourceRole::Production,
            }],
            rust_dependencies: Vec::new(),
            authored_rust_modules: Vec::new(),
            terrane_dependencies: Vec::new(),
            dependency_manifests: Vec::new(),
            library_source_ids: BTreeSet::new(),
        }
    }

    pub(crate) fn next_source_id(&self) -> u32 {
        self.units
            .iter()
            .map(|unit| unit.source.id())
            .chain(
                self.authored_rust_modules
                    .iter()
                    .map(|module| module.source.id()),
            )
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }

    /// The manifest is TOML with required `package` and `namespaces` fields plus
    /// optional package configuration and dependency tables. Sources are discovered in sorted
    /// path order, and Terrane library dependencies are composed recursively.
    ///
    /// # Errors
    ///
    /// Returns every manifest, dependency, or source file error.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Vec<PackageLoadError>> {
        let requested = path.as_ref();
        let manifest_path = if requested.is_dir() {
            requested.join(MANIFEST_FILE_NAME)
        } else {
            requested.to_path_buf()
        };
        load_package_graph(&manifest_path)
    }

    pub(crate) fn from_tooling_sources(
        manifest_path: &Path,
        manifest_text: &str,
        units: Vec<SourceUnit>,
    ) -> Result<Self, Vec<PackageLoadError>> {
        let manifest = parse_manifest(manifest_path, manifest_text)?;
        let root = manifest_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        let (mut units, external_units): (Vec<_>, Vec<_>) = units.into_iter().partition(|unit| {
            unit.source
                .path()
                .strip_prefix(&root)
                .ok()
                .and_then(Path::parent)
                .is_some_and(|parent| {
                    manifest
                        .namespace_roots
                        .iter()
                        .any(|mapping| parent.starts_with(&mapping.directory))
                })
        });
        let mut errors = Vec::new();
        for unit in &mut units {
            let relative_path = unit
                .source
                .path()
                .strip_prefix(&root)
                .expect("root package unit was partitioned by path");
            let parent = relative_path.parent().unwrap_or_else(|| Path::new(""));
            let mapping = manifest
                .namespace_roots
                .iter()
                .filter(|mapping| parent.starts_with(&mapping.directory))
                .max_by_key(|mapping| mapping.directory.components().count());
            let Some(mapping) = mapping else {
                errors.push(PackageLoadError::unreadable(
                    unit.source.path().to_path_buf(),
                    "snapshot source is outside every declared namespace root",
                ));
                continue;
            };
            let suffix = parent
                .strip_prefix(&mapping.directory)
                .expect("matched namespace directory");
            match expected_namespace(&mapping.namespace, suffix) {
                Ok(namespace) => unit.expected_namespace = Some(namespace),
                Err(message) => errors.push(PackageLoadError::unreadable(
                    unit.source.path().to_path_buf(),
                    message,
                )),
            }
            unit.prelude = manifest.prelude;
            unit.relative_path = relative_path.to_path_buf();
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        let authored_rust_modules =
            load_authored_rust_modules(&root, &manifest.authored_rust_modules, units.len())?;
        let mut package = Self {
            identity: manifest.identity,
            root,
            prelude: manifest.prelude,
            reflection: manifest.reflection,
            executor: manifest.executor,
            artifact: manifest.artifact,
            profile: manifest.profile,
            purpose: PackagePurpose::Production,
            testing: manifest.testing,
            build_toolchain: manifest.build_toolchain,
            units,
            rust_dependencies: manifest.rust_dependencies,
            authored_rust_modules,
            terrane_dependencies: manifest.terrane_dependencies,
            library_source_ids: BTreeSet::new(),
            dependency_manifests: vec![manifest_path.to_path_buf()],
        };
        compose_package_dependencies(manifest_path, &mut package)?;
        overlay_tooling_sources(&mut package, external_units)?;
        Ok(package)
    }
}

fn overlay_tooling_sources(
    package: &mut Package,
    overlays: Vec<SourceUnit>,
) -> Result<(), Vec<PackageLoadError>> {
    let mut errors = Vec::new();
    for overlay in overlays {
        let overlay_path = overlay
            .source
            .path()
            .canonicalize()
            .unwrap_or_else(|_| overlay.source.path().to_path_buf());
        let Some(unit) = package.units.iter_mut().find(|unit| {
            unit.source
                .path()
                .canonicalize()
                .unwrap_or_else(|_| unit.source.path().to_path_buf())
                == overlay_path
        }) else {
            errors.push(PackageLoadError::unreadable(
                overlay.source.path().to_path_buf(),
                "snapshot source is outside the composed package graph",
            ));
            continue;
        };
        unit.source = SourceFile::new(
            unit.source.id(),
            unit.source.path().to_path_buf(),
            overlay.source.text().to_owned(),
        );
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn load_package_graph(manifest_path: &Path) -> Result<Package, Vec<PackageLoadError>> {
    let mut package = load_package_unit(manifest_path)?;
    compose_package_dependencies(manifest_path, &mut package)?;
    Ok(package)
}

fn compose_package_dependencies(
    manifest_path: &Path,
    package: &mut Package,
) -> Result<(), Vec<PackageLoadError>> {
    let root_key = package
        .root
        .canonicalize()
        .unwrap_or_else(|_| package.root.clone());
    let mut loaded = BTreeSet::from([root_key.clone()]);
    let mut identities = BTreeMap::from([(package.identity.clone(), root_key.clone())]);
    let mut stack = vec![(root_key, package.identity.clone())];
    let cache_owner = package.root.clone();
    compose_dependencies(
        manifest_path,
        package,
        &cache_owner,
        &mut loaded,
        &mut identities,
        &mut stack,
    )
}

fn load_package_unit(manifest_path: &Path) -> Result<Package, Vec<PackageLoadError>> {
    let root = manifest_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let text = fs::read_to_string(manifest_path).map_err(|error| {
        vec![PackageLoadError::unreadable(
            manifest_path.to_path_buf(),
            format!("cannot read package manifest: {error}"),
        )]
    })?;
    let manifest = parse_manifest(manifest_path, &text)?;
    if manifest.artifact == ArtifactKind::Library {
        let namespace_identity = format!("/{}", manifest.identity.trim_start_matches('/'));
        let prefix = format!("{namespace_identity}/");
        let invalid = manifest.namespace_roots.iter().find(|mapping| {
            mapping.namespace != namespace_identity && !mapping.namespace.starts_with(&prefix)
        });
        if let Some(mapping) = invalid {
            return Err(vec![manifest_error(
                manifest_path,
                &text,
                format!(
                    "library namespace `{}` must equal or descend from package identity `{}`",
                    mapping.namespace, manifest.identity
                ),
                Some(&mapping.namespace),
            )]);
        }
    }
    let units = discover_source_units(&root, &manifest.namespace_roots, manifest.prelude)?;
    let authored_rust_modules =
        load_authored_rust_modules(&root, &manifest.authored_rust_modules, units.len())?;
    Ok(Package {
        identity: manifest.identity,
        root,
        prelude: manifest.prelude,
        reflection: manifest.reflection,
        executor: manifest.executor,
        artifact: manifest.artifact,
        profile: manifest.profile,
        purpose: PackagePurpose::Production,
        testing: manifest.testing,
        build_toolchain: manifest.build_toolchain,
        units,
        rust_dependencies: manifest.rust_dependencies,
        authored_rust_modules,
        terrane_dependencies: manifest.terrane_dependencies,
        library_source_ids: BTreeSet::new(),
        dependency_manifests: vec![manifest_path.to_path_buf()],
    })
}

fn compose_dependencies(
    manifest_path: &Path,
    package: &mut Package,
    cache_owner: &Path,
    loaded: &mut BTreeSet<PathBuf>,
    identities: &mut BTreeMap<String, PathBuf>,
    stack: &mut Vec<(PathBuf, String)>,
) -> Result<(), Vec<PackageLoadError>> {
    let dependencies = package.terrane_dependencies.clone();
    for dependency in dependencies {
        let dependency_root = resolve_terrane_dependency(&package.root, cache_owner, &dependency)
            .map_err(|message| {
            vec![dependency_error(manifest_path, &dependency.name, message)]
        })?;
        let dependency_key = dependency_root
            .canonicalize()
            .unwrap_or_else(|_| dependency_root.clone());
        if let Some((_, identity)) = stack.iter().find(|(path, _)| path == &dependency_key) {
            let chain = stack
                .iter()
                .map(|(_, identity)| identity.as_str())
                .chain(std::iter::once(identity.as_str()))
                .collect::<Vec<_>>()
                .join(" -> ");
            return Err(vec![dependency_error(
                manifest_path,
                &dependency.name,
                format!("Terrane dependency cycle detected: {chain}"),
            )]);
        }
        if loaded.contains(&dependency_key) {
            continue;
        }
        let dependency_manifest = dependency_root.join(MANIFEST_FILE_NAME);
        let mut library = load_package_unit(&dependency_manifest)?;
        if library.artifact != ArtifactKind::Library {
            return Err(vec![dependency_error(
                manifest_path,
                &dependency.name,
                format!(
                    "dependency package `{}` must declare `artifact = \"library\"`",
                    library.identity
                ),
            )]);
        }
        if let Some(existing) = identities.get(&library.identity)
            && existing != &dependency_key
        {
            return Err(vec![dependency_error(
                manifest_path,
                &dependency.name,
                format!(
                    "library identity `{}` is already loaded from `{}` and cannot also be loaded from `{}`",
                    library.identity,
                    existing.display(),
                    dependency_key.display()
                ),
            )]);
        }
        identities.insert(library.identity.clone(), dependency_key.clone());
        stack.push((dependency_key.clone(), library.identity.clone()));
        compose_dependencies(
            &dependency_manifest,
            &mut library,
            cache_owner,
            loaded,
            identities,
            stack,
        )?;
        stack.pop();
        loaded.insert(dependency_key);
        merge_library(package, library, manifest_path, &dependency.name)?;
    }
    Ok(())
}

fn dependency_error(
    manifest_path: &Path,
    dependency: &str,
    message: impl Into<String>,
) -> PackageLoadError {
    let text = fs::read_to_string(manifest_path).unwrap_or_default();
    let headers = [
        format!("[terrane-dependencies.{dependency}]"),
        format!("[terrane-dependencies.{dependency:?}]"),
    ];
    let span = headers.iter().find_map(|header| {
        text.find(header)
            .map(|start| Span::new(0, start, start + header.len()))
    });
    PackageLoadError::new(
        manifest_path.to_path_buf(),
        text,
        format!("Terrane dependency `{dependency}`: {}", message.into()),
        span,
    )
}

fn resolve_terrane_dependency(
    root: &Path,
    cache_owner: &Path,
    dependency: &TerraneDependency,
) -> Result<PathBuf, String> {
    let (resolved, already_verified) = match &dependency.source {
        TerraneDependencySource::Path(path) => (root.join(path), false),
        TerraneDependencySource::Git { url, tag } => {
            let hash = dependency
                .hash
                .as_deref()
                .expect("Git dependencies are parsed with a required hash");
            let digest = hash
                .strip_prefix("sha256:")
                .expect("validated dependency hash");
            let cache_root = cache_owner.join(".trn/packages");
            let destination = cache_root.join(digest);
            if destination.is_dir() && source_tree_hash(&destination)? != hash {
                fs::remove_dir_all(&destination).map_err(|error| {
                    format!("cannot remove invalid package cache entry: {error}")
                })?;
            }
            if !destination.is_dir() {
                fs::create_dir_all(&cache_root)
                    .map_err(|error| format!("cannot create package cache: {error}"))?;
                let temporary = cache_root.join(format!(".{digest}-{}", std::process::id()));
                if temporary.exists() {
                    fs::remove_dir_all(&temporary).map_err(|error| {
                        format!("cannot clear temporary package cache: {error}")
                    })?;
                }
                if let Err(error) = clone_git_tag(url, tag, &temporary) {
                    let _ = fs::remove_dir_all(&temporary);
                    return Err(error);
                }
                let actual = source_tree_hash(&temporary)?;
                if actual != hash {
                    let _ = fs::remove_dir_all(&temporary);
                    return Err(format!(
                        "source hash mismatch: expected `{hash}`, found `{actual}`"
                    ));
                }
                if let Err(error) = fs::rename(&temporary, &destination) {
                    if !destination.is_dir() || source_tree_hash(&destination)? != hash {
                        let _ = fs::remove_dir_all(&temporary);
                        return Err(format!("cannot publish package cache entry: {error}"));
                    }
                    let _ = fs::remove_dir_all(&temporary);
                }
            }
            (destination, true)
        }
    };
    if !resolved.is_dir() {
        return Err(format!(
            "source directory `{}` does not exist",
            resolved.display()
        ));
    }
    if !already_verified && let Some(expected) = dependency.hash.as_deref() {
        let actual = source_tree_hash(&resolved)?;
        if actual != expected {
            return Err(format!(
                "source hash mismatch: expected `{expected}`, found `{actual}`"
            ));
        }
    }
    Ok(resolved)
}

fn clone_git_tag(url: &str, tag: &str, destination: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--single-branch",
            "--branch",
            tag,
            "--",
        ])
        .arg(url)
        .arg(destination)
        .output()
        .map_err(|error| format!("cannot launch Git: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cannot clone tag `{tag}` from `{url}`: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let tagged = Command::new("git")
        .arg("-C")
        .arg(destination)
        .args([
            "rev-parse",
            "--verify",
            &format!("refs/tags/{tag}^{{commit}}"),
        ])
        .output()
        .map_err(|error| format!("cannot inspect cloned Git tag: {error}"))?;
    if !tagged.status.success() {
        return Err(format!("Git reference `{tag}` is not a tag"));
    }
    let head = Command::new("git")
        .arg("-C")
        .arg(destination)
        .args(["rev-parse", "--verify", "HEAD"])
        .output()
        .map_err(|error| format!("cannot inspect cloned Git revision: {error}"))?;
    if !head.status.success() || head.stdout != tagged.stdout {
        return Err(format!(
            "Git checkout does not resolve exactly to tag `{tag}`"
        ));
    }
    Ok(())
}

fn with_git_tag<T>(
    url: &str,
    tag: &str,
    inspect: impl FnOnce(&Path) -> Result<T, String>,
) -> Result<T, String> {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| format!("system clock is before the Unix epoch: {error}"))?
        .as_nanos();
    let temporary = std::env::temp_dir().join(format!(
        "terrane-package-hash-{}-{nonce}",
        std::process::id()
    ));
    let result = clone_git_tag(url, tag, &temporary).and_then(|()| inspect(&temporary));
    let cleanup = fs::remove_dir_all(&temporary);
    if let Err(error) = &result {
        return Err(error.clone());
    }
    if let Err(error) = cleanup
        && error.kind() != std::io::ErrorKind::NotFound
    {
        return Err(format!("cannot remove temporary Git checkout: {error}"));
    }
    result
}

/// Clones one exact Git tag and computes its deterministic Terrane package source hash.
///
/// # Errors
///
/// Returns an error when Git cannot clone or verify the tag, or when the checked-out source tree
/// cannot be hashed.
pub fn git_source_tree_hash(url: &str, tag: &str) -> Result<String, String> {
    with_git_tag(url, tag, source_tree_hash)
}

/// Reads the identity and deterministic source hash of a Terrane library at one exact Git tag.
///
/// # Errors
///
/// Returns an error when Git cannot clone or verify the tag, or when the checkout is not a valid
/// Terrane library package.
pub fn git_library_metadata(url: &str, tag: &str) -> Result<(String, String), String> {
    with_git_tag(url, tag, |root| {
        let hash = source_tree_hash(root)?;
        let package = Package::load(root).map_err(|errors| {
            errors
                .into_iter()
                .map(|error| error.diagnostic.message)
                .collect::<Vec<_>>()
                .join("; ")
        })?;
        if package.artifact != ArtifactKind::Library {
            return Err(format!(
                "package `{}` must declare `artifact = \"library\"`",
                package.identity
            ));
        }
        Ok((package.identity, hash))
    })
}

/// Computes the deterministic content hash used by Terrane package dependency declarations.
///
/// # Errors
///
/// Returns an error when the tree cannot be read, contains a symbolic link, or has a non-UTF-8
/// relative path.
pub fn source_tree_hash(root: &Path) -> Result<String, String> {
    fn collect(root: &Path, directory: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
        let mut entries = fs::read_dir(directory)
            .map_err(|error| format!("cannot read source tree: {error}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("cannot read source tree entry: {error}"))?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let path = entry.path();
            let name = entry.file_name();
            let file_type = entry
                .file_type()
                .map_err(|error| format!("cannot inspect source tree entry: {error}"))?;
            if file_type.is_symlink() {
                return Err(format!(
                    "source tree contains unsupported symbolic link `{}`",
                    path.display()
                ));
            }
            if file_type.is_dir() {
                if name != ".git" && name != ".trn" {
                    collect(root, &path, files)?;
                }
            } else if file_type.is_file() {
                files.push(
                    path.strip_prefix(root)
                        .expect("walked source path is below root")
                        .to_path_buf(),
                );
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    collect(root, root, &mut files)?;
    files.sort();
    let mut hasher = Sha256::new();
    for relative in files {
        let path = relative
            .to_str()
            .ok_or_else(|| "source tree contains a non-UTF-8 path".to_owned())?
            .replace('\\', "/");
        let contents = fs::read(root.join(&relative))
            .map_err(|error| format!("cannot read source tree file `{path}`: {error}"))?;
        hasher.update((path.len() as u64).to_le_bytes());
        hasher.update(path.as_bytes());
        hasher.update((contents.len() as u64).to_le_bytes());
        hasher.update(contents);
    }
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

fn merge_library(
    package: &mut Package,
    mut library: Package,
    manifest_path: &Path,
    dependency_name: &str,
) -> Result<(), Vec<PackageLoadError>> {
    for dependency in &library.rust_dependencies {
        if let Some(existing) = package
            .rust_dependencies
            .iter()
            .find(|existing| existing.name == dependency.name)
        {
            if existing != dependency {
                return Err(vec![dependency_error(
                    manifest_path,
                    dependency_name,
                    format!(
                        "Rust dependency alias `{}` conflicts with another package",
                        dependency.name
                    ),
                )]);
            }
        } else {
            package.rust_dependencies.push(dependency.clone());
        }
        for effect in std::iter::once("build").chain(dependency.effects.iter().map(String::as_str))
        {
            if !package.profile.allows(effect) {
                return Err(vec![dependency_error(
                    manifest_path,
                    dependency_name,
                    format!(
                        "profile `{}` forbids effect `{effect}` required by Rust dependency `{}`",
                        package.profile.name, dependency.name
                    ),
                )]);
            }
        }
    }
    for module in &library.authored_rust_modules {
        if package
            .authored_rust_modules
            .iter()
            .any(|existing| existing.name == module.name)
        {
            return Err(vec![dependency_error(
                manifest_path,
                dependency_name,
                format!(
                    "authored Rust module `{}` conflicts with another package",
                    module.name
                ),
            )]);
        }
        if !package.profile.allows("build") {
            return Err(vec![dependency_error(
                manifest_path,
                dependency_name,
                format!(
                    "profile `{}` forbids effect `build` required by authored Rust module `{}`",
                    package.profile.name, module.name
                ),
            )]);
        }
    }
    let prefix = Path::new("dependencies").join(&library.identity);
    for mut unit in library.units.drain(..) {
        let id = package.next_source_id();
        unit.relative_path = prefix.join(&unit.relative_path);
        unit.source = SourceFile::new(
            id,
            unit.source.path().to_path_buf(),
            unit.source.text().to_owned(),
        );
        package.library_source_ids.insert(id);
        package.units.push(unit);
    }
    for mut module in library.authored_rust_modules.drain(..) {
        let id = package.next_source_id();
        module.relative_path = prefix.join(&module.relative_path);
        module.source = SourceFile::new(
            id,
            module.source.path().to_path_buf(),
            module.source.text().to_owned(),
        );
        package.authored_rust_modules.push(module);
    }
    package
        .dependency_manifests
        .extend(library.dependency_manifests);
    package
        .rust_dependencies
        .sort_by(|left, right| left.name.cmp(&right.name));
    Ok(())
}

struct ParsedManifest {
    identity: String,
    prelude: bool,
    artifact: ArtifactKind,
    reflection: ReflectionProfile,
    build_toolchain: BuildToolchain,
    executor: ExecutorProfile,
    profile: CapabilityProfile,
    testing: crate::testing::TestConfiguration,
    namespace_roots: Vec<NamespaceRoot>,
    rust_dependencies: Vec<RustDependency>,
    authored_rust_modules: Vec<(String, PathBuf)>,
    terrane_dependencies: Vec<TerraneDependency>,
}

#[derive(Clone, Debug)]
struct NamespaceRoot {
    namespace: String,
    directory: PathBuf,
}

#[expect(
    clippy::too_many_lines,
    reason = "linear validation of one manifest table"
)]
fn parse_manifest(
    manifest_path: &Path,
    text: &str,
) -> Result<ParsedManifest, Vec<PackageLoadError>> {
    let table = text.parse::<toml::Table>().map_err(|error| {
        let span = error
            .span()
            .map(|range| Span::new(0, range.start, range.end));
        vec![PackageLoadError::new(
            manifest_path.to_path_buf(),
            text.to_owned(),
            format!("invalid TOML: {error}"),
            span,
        )]
    })?;
    let mut errors = Vec::new();
    for key in table.keys() {
        if !matches!(
            key.as_str(),
            "package"
                | "prelude"
                | "artifact"
                | "reflection"
                | "executor"
                | "rust-toolchain"
                | "profile"
                | "testing"
                | "namespaces"
                | "terrane-dependencies"
                | "rust-dependencies"
                | "rust-modules"
        ) {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("unknown manifest field `{key}`"),
                Some(key),
            ));
        }
    }
    let identity = match table.get("package") {
        Some(toml::Value::String(value)) if !value.is_empty() => Some(value.clone()),
        Some(_) => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "`package` must be a non-empty string",
                Some("package"),
            ));
            None
        }
        None => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "missing `package` identity",
                None,
            ));
            None
        }
    };
    let artifact = match table.get("artifact") {
        None => ArtifactKind::Executable,
        Some(toml::Value::String(value)) if value == "executable" => ArtifactKind::Executable,
        Some(toml::Value::String(value)) if value == "dynamic-library" => {
            ArtifactKind::DynamicLibrary
        }
        Some(toml::Value::String(value)) if value == "library" => ArtifactKind::Library,
        Some(_) => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "`artifact` must be either `executable` or `dynamic-library`, or `library`",
                Some("artifact"),
            ));
            ArtifactKind::Executable
        }
    };
    let prelude = match table.get("prelude") {
        Some(toml::Value::Boolean(value)) => *value,
        Some(_) => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "`prelude` must be a boolean",
                Some("prelude"),
            ));
            true
        }
        None => true,
    };
    let build_toolchain = match table.get("rust-toolchain") {
        None => BuildToolchain::Pinned,
        Some(toml::Value::String(value)) if value == "pinned" => BuildToolchain::Pinned,
        Some(toml::Value::String(value)) if value == "system" => BuildToolchain::System,
        Some(_) => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "`rust-toolchain` must be either `pinned` or `system`",
                Some("rust-toolchain"),
            ));
            BuildToolchain::Pinned
        }
    };
    let reflection = match table.get("reflection") {
        Some(toml::Value::String(value)) if value == "ordinary" => ReflectionProfile::Ordinary,
        Some(toml::Value::String(value)) if value == "minimal" => ReflectionProfile::Minimal,
        Some(_) => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "`reflection` must be either `ordinary` or `minimal`",
                Some("reflection"),
            ));
            ReflectionProfile::Ordinary
        }
        None => ReflectionProfile::Ordinary,
    };
    let executor = match table.get("executor") {
        Some(toml::Value::String(value)) if value == "cooperative" => ExecutorProfile::Cooperative,
        Some(toml::Value::String(value)) if value == "threaded" => ExecutorProfile::Threaded,
        Some(_) => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "`executor` must be either `cooperative` or `threaded`",
                Some("executor"),
            ));
            ExecutorProfile::Threaded
        }
        None => ExecutorProfile::Threaded,
    };
    let profile = parse_capability_profile(manifest_path, text, &table, &mut errors);
    let namespace_roots = parse_namespace_roots(manifest_path, text, &table, &mut errors);
    let rust_dependencies = parse_rust_dependencies(manifest_path, text, &table, &mut errors);
    let terrane_dependencies = parse_terrane_dependencies(manifest_path, text, &table, &mut errors);
    let authored_rust_modules =
        parse_authored_rust_modules(manifest_path, text, &table, &mut errors);
    if !authored_rust_modules.is_empty() && !profile.allows("build") {
        errors.push(manifest_error(
            manifest_path,
            text,
            format!(
                "profile `{}` forbids effect `build` required by authored Rust modules",
                profile.name
            ),
            Some("rust-modules"),
        ));
    }
    for dependency in &rust_dependencies {
        for effect in std::iter::once("build").chain(dependency.effects.iter().map(String::as_str))
        {
            if !profile.allows(effect) {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!(
                        "profile `{}` forbids effect `{effect}` required by Rust dependency `{}`",
                        profile.name, dependency.name
                    ),
                    Some(&dependency.name),
                ));
            }
        }
    }
    let testing = match crate::testing::parse_configuration(manifest_path, text, &table, &profile) {
        Ok(configuration) => Some(configuration),
        Err(mut testing_errors) => {
            errors.append(&mut testing_errors);
            None
        }
    };
    if errors.is_empty() {
        Ok(ParsedManifest {
            identity: identity.expect("validated package identity"),
            prelude,
            reflection,
            artifact,
            build_toolchain,
            executor,
            profile,
            namespace_roots,
            rust_dependencies,
            authored_rust_modules,
            terrane_dependencies,
            testing: testing.expect("validated testing configuration"),
        })
    } else {
        Err(errors)
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "linear validation of one dependency declaration table"
)]
fn parse_terrane_dependencies(
    manifest_path: &Path,
    text: &str,
    table: &toml::Table,
    errors: &mut Vec<PackageLoadError>,
) -> Vec<TerraneDependency> {
    let Some(value) = table.get("terrane-dependencies") else {
        return Vec::new();
    };
    let Some(dependencies) = value.as_table() else {
        errors.push(manifest_error(
            manifest_path,
            text,
            "`terrane-dependencies` must be a table",
            Some("terrane-dependencies"),
        ));
        return Vec::new();
    };
    dependencies
        .iter()
        .filter_map(|(name, value)| {
            let Some(fields) = value.as_table() else {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("Terrane dependency `{name}` must be a table"),
                    Some(name),
                ));
                return None;
            };
            for field in fields.keys() {
                if !matches!(field.as_str(), "path" | "git" | "tag" | "hash") {
                    errors.push(manifest_error(
                        manifest_path,
                        text,
                        format!("unknown field `{field}` in Terrane dependency `{name}`"),
                        Some(field),
                    ));
                }
            }
            let path = fields.get("path").and_then(toml::Value::as_str);
            let git = fields.get("git").and_then(toml::Value::as_str);
            let tag = fields.get("tag").and_then(toml::Value::as_str);
            let hash = fields.get("hash").and_then(toml::Value::as_str);
            if fields.get("path").is_some_and(|value| !value.is_str())
                || fields.get("git").is_some_and(|value| !value.is_str())
                || fields.get("tag").is_some_and(|value| !value.is_str())
                || fields.get("hash").is_some_and(|value| !value.is_str())
            {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("Terrane dependency `{name}` fields must be strings"),
                    Some(name),
                ));
                return None;
            }
            let source = match (path, git) {
                (Some(path), None) => {
                    let path = Path::new(path);
                    let Some(path) = (!path.as_os_str().is_empty() && !path.is_absolute())
                        .then(|| path.to_path_buf())
                    else {
                        errors.push(manifest_error(
                            manifest_path,
                            text,
                            format!("Terrane dependency `{name}` path must be relative"),
                            Some(name),
                        ));
                        return None;
                    };
                    if tag.is_some() {
                        errors.push(manifest_error(
                            manifest_path,
                            text,
                            format!("local Terrane dependency `{name}` cannot declare `tag`"),
                            Some("tag"),
                        ));
                        return None;
                    }
                    TerraneDependencySource::Path(path)
                }
                (None, Some(url)) if !url.is_empty() => {
                    let Some(tag) = tag.filter(|tag| !tag.is_empty()) else {
                        errors.push(manifest_error(
                            manifest_path,
                            text,
                            format!("Git Terrane dependency `{name}` requires a non-empty `tag`"),
                            Some(name),
                        ));
                        return None;
                    };
                    if hash.is_none() {
                        errors.push(manifest_error(
                            manifest_path,
                            text,
                            format!("Git Terrane dependency `{name}` requires `hash`"),
                            Some(name),
                        ));
                        return None;
                    }
                    TerraneDependencySource::Git {
                        url: url.to_owned(),
                        tag: tag.to_owned(),
                    }
                }
                _ => {
                    errors.push(manifest_error(
                        manifest_path,
                        text,
                        format!(
                            "Terrane dependency `{name}` must declare exactly one of `path` or `git`"
                        ),
                        Some(name),
                    ));
                    return None;
                }
            };
            let hash = match hash {
                Some(hash) if valid_tree_hash(hash) => Some(hash.to_owned()),
                Some(hash) => {
                    errors.push(manifest_error(
                        manifest_path,
                        text,
                        format!(
                            "Terrane dependency `{name}` hash must be `sha256:` followed by 64 lowercase hexadecimal digits"
                        ),
                        Some(hash),
                    ));
                    return None;
                }
                None => None,
            };
            Some(TerraneDependency {
                name: name.clone(),
                source,
                hash,
            })
        })
        .collect()
}

fn valid_tree_hash(hash: &str) -> bool {
    hash.strip_prefix("sha256:").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    })
}

fn parse_authored_rust_modules(
    manifest_path: &Path,
    text: &str,
    table: &toml::Table,
    errors: &mut Vec<PackageLoadError>,
) -> Vec<(String, PathBuf)> {
    let Some(value) = table.get("rust-modules") else {
        return Vec::new();
    };
    let Some(modules) = value.as_table() else {
        errors.push(manifest_error(
            manifest_path,
            text,
            "`rust-modules` must be a table mapping module names to relative `.rs` paths",
            Some("rust-modules"),
        ));
        return Vec::new();
    };
    modules
        .iter()
        .filter_map(|(name, value)| {
            if syn::parse_str::<syn::Ident>(name).is_err() {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("authored Rust module name `{name}` is not a Rust identifier"),
                    Some(name),
                ));
                return None;
            }
            let Some(path) = value.as_str().and_then(normalized_relative_directory) else {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("authored Rust module `{name}` must name a relative `.rs` path"),
                    Some(name),
                ));
                return None;
            };
            if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("authored Rust module `{name}` path must end in `.rs`"),
                    Some(name),
                ));
                return None;
            }
            Some((name.clone(), path))
        })
        .collect()
}

fn load_authored_rust_modules(
    root: &Path,
    modules: &[(String, PathBuf)],
    source_id_start: usize,
) -> Result<Vec<AuthoredRustModule>, Vec<PackageLoadError>> {
    let mut loaded = Vec::with_capacity(modules.len());
    let mut errors = Vec::new();
    for (offset, (name, relative_path)) in modules.iter().enumerate() {
        let path = root.join(relative_path);
        let Ok(source_id) = u32::try_from(source_id_start + offset) else {
            errors.push(PackageLoadError::unreadable(
                path,
                "package has too many source units",
            ));
            continue;
        };
        match fs::read_to_string(&path) {
            Ok(text) => loaded.push(AuthoredRustModule {
                name: name.clone(),
                relative_path: relative_path.clone(),
                source: SourceFile::new(source_id, path, text),
            }),
            Err(error) => errors.push(PackageLoadError::unreadable(
                path,
                format!("cannot read authored Rust module: {error}"),
            )),
        }
    }
    if errors.is_empty() {
        Ok(loaded)
    } else {
        Err(errors)
    }
}

fn parse_capability_profile(
    manifest_path: &Path,
    text: &str,
    table: &toml::Table,
    errors: &mut Vec<PackageLoadError>,
) -> CapabilityProfile {
    let Some(value) = table.get("profile") else {
        return CapabilityProfile::unrestricted();
    };
    let Some(fields) = value.as_table() else {
        errors.push(manifest_error(
            manifest_path,
            text,
            "`profile` must be a table",
            Some("profile"),
        ));
        return CapabilityProfile::unrestricted();
    };
    for key in fields.keys() {
        if !matches!(key.as_str(), "name" | "capabilities" | "panic") {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("unknown profile field `{key}`"),
                Some(key),
            ));
        }
    }
    let name = fields
        .get("name")
        .and_then(toml::Value::as_str)
        .unwrap_or("default")
        .to_owned();
    let capabilities = match fields.get("capabilities") {
        Some(toml::Value::Array(values)) if values.iter().all(|value| value.as_str().is_some()) => {
            let values = values
                .iter()
                .filter_map(toml::Value::as_str)
                .collect::<Vec<_>>();
            for capability in &values {
                if !is_capability_name(capability) {
                    errors.push(manifest_error(
                        manifest_path,
                        text,
                        format!("unknown profile capability `{capability}`"),
                        Some(capability),
                    ));
                }
            }
            Some(values.into_iter().map(str::to_owned).collect())
        }
        Some(_) => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "profile `capabilities` must be an array of strings",
                Some("capabilities"),
            ));
            Some(BTreeSet::new())
        }
        None => None,
    };
    let panic = match fields.get("panic") {
        Some(toml::Value::String(value)) if value == "unwind" => PanicProfile::Unwind,
        Some(toml::Value::String(value)) if value == "abort" => PanicProfile::Abort,
        Some(_) => {
            errors.push(manifest_error(
                manifest_path,
                text,
                "profile `panic` must be either `unwind` or `abort`",
                Some("panic"),
            ));
            PanicProfile::Unwind
        }
        None => PanicProfile::Unwind,
    };
    CapabilityProfile {
        name,
        capabilities,
        panic,
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "linear validation keeps every dependency field diagnostic together"
)]
fn parse_rust_dependencies(
    manifest_path: &Path,
    text: &str,
    table: &toml::Table,
    errors: &mut Vec<PackageLoadError>,
) -> Vec<RustDependency> {
    let Some(value) = table.get("rust-dependencies") else {
        return Vec::new();
    };
    let Some(dependencies) = value.as_table() else {
        errors.push(manifest_error(
            manifest_path,
            text,
            "`rust-dependencies` must be a table",
            Some("rust-dependencies"),
        ));
        return Vec::new();
    };
    let mut parsed = Vec::new();
    for (name, value) in dependencies {
        let Some(fields) = value.as_table() else {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("Rust dependency `{name}` must be a table"),
                Some(name),
            ));
            continue;
        };
        for key in fields.keys() {
            if !matches!(
                key.as_str(),
                "package" | "version" | "features" | "default-features" | "target" | "effects"
            ) {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("unknown field `{key}` in Rust dependency `{name}`"),
                    Some(key),
                ));
            }
        }
        let package = fields
            .get("package")
            .and_then(toml::Value::as_str)
            .unwrap_or(name);
        let Some(version) = fields.get("version").and_then(toml::Value::as_str) else {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("Rust dependency `{name}` requires a string `version`"),
                Some(name),
            ));
            continue;
        };
        let features = match fields.get("features") {
            None => Vec::new(),
            Some(toml::Value::Array(values))
                if values.iter().all(|value| value.as_str().is_some()) =>
            {
                values
                    .iter()
                    .filter_map(toml::Value::as_str)
                    .map(str::to_owned)
                    .collect()
            }
            Some(_) => {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("Rust dependency `{name}` has a non-string `features` entry"),
                    Some(name),
                ));
                continue;
            }
        };
        let effects = match fields.get("effects") {
            None => Vec::new(),
            Some(toml::Value::Array(values))
                if values.iter().all(|value| value.as_str().is_some()) =>
            {
                let values = values
                    .iter()
                    .filter_map(toml::Value::as_str)
                    .collect::<Vec<_>>();
                for effect in &values {
                    if !is_capability_name(effect) {
                        errors.push(manifest_error(
                            manifest_path,
                            text,
                            format!("Rust dependency `{name}` declares unknown effect `{effect}`"),
                            Some(effect),
                        ));
                    }
                }
                values.into_iter().map(str::to_owned).collect()
            }
            Some(_) => {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("Rust dependency `{name}` has a non-string `effects` entry"),
                    Some(name),
                ));
                continue;
            }
        };
        let default_features = match fields.get("default-features") {
            None => true,
            Some(toml::Value::Boolean(value)) => *value,
            Some(_) => {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("Rust dependency `{name}` has a non-boolean `default-features`"),
                    Some(name),
                ));
                continue;
            }
        };
        let target = match fields.get("target") {
            None => None,
            Some(toml::Value::String(value)) => Some(value.clone()),
            Some(_) => {
                errors.push(manifest_error(
                    manifest_path,
                    text,
                    format!("Rust dependency `{name}` has a non-string `target`"),
                    Some(name),
                ));
                continue;
            }
        };
        parsed.push(RustDependency {
            name: name.clone(),
            package: package.to_owned(),
            version: version.to_owned(),
            features,
            default_features,
            target,
            effects,
        });
    }
    parsed.sort_by(|left, right| left.name.cmp(&right.name));
    parsed
}

fn parse_namespace_roots(
    manifest_path: &Path,
    text: &str,
    table: &toml::Table,
    errors: &mut Vec<PackageLoadError>,
) -> Vec<NamespaceRoot> {
    let Some(toml::Value::Table(mappings)) = table.get("namespaces") else {
        errors.push(manifest_error(
            manifest_path,
            text,
            "package must declare a non-empty `namespaces` mapping table",
            Some("namespaces"),
        ));
        return Vec::new();
    };
    if mappings.is_empty() {
        errors.push(manifest_error(
            manifest_path,
            text,
            "`namespaces` must be a non-empty mapping table",
            Some("namespaces"),
        ));
    }
    let mut directories = BTreeMap::<PathBuf, String>::new();
    let mut roots = Vec::new();
    for (namespace, value) in mappings {
        if namespace == "/" {
            errors.push(manifest_error(
                manifest_path,
                text,
                "namespace root `/` cannot be declared by a source file",
                Some(namespace),
            ));
            continue;
        }
        let path = namespace.trim_start_matches('/');
        if let Some(segment) = path
            .split('/')
            .find(|segment| !valid_namespace_segment(segment))
        {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("namespace segment `{segment}` must match `[a-z]([a-z0-9]|-[a-z0-9])*`"),
                Some(namespace),
            ));
            continue;
        }
        if let Some(segment) = path
            .split('/')
            .find(|segment| reserved_namespace_segment(segment))
        {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("namespace segment `{segment}` is reserved"),
                Some(namespace),
            ));
            continue;
        }
        let canonical = format!("/{path}");
        let Some(directory) = value.as_str().and_then(normalized_relative_directory) else {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!("directory for namespace root `{namespace}` must be a relative path"),
                Some(namespace),
            ));
            continue;
        };
        if let Some(existing) = directories.insert(directory.clone(), namespace.clone()) {
            errors.push(manifest_error(
                manifest_path,
                text,
                format!(
                    "namespace roots `{existing}` and `{namespace}` map to the same directory `{}`",
                    directory.display()
                ),
                Some(namespace),
            ));
            continue;
        }
        roots.push(NamespaceRoot {
            namespace: canonical,
            directory,
        });
    }
    roots.sort_by(|left, right| left.namespace.cmp(&right.namespace));
    roots
}

fn manifest_error(
    path: &Path,
    text: &str,
    message: impl Into<String>,
    needle: Option<&str>,
) -> PackageLoadError {
    let span = needle.and_then(|needle| {
        text.find(needle)
            .map(|start| Span::new(0, start, start + needle.len()))
    });
    PackageLoadError::new(path.to_path_buf(), text.to_owned(), message, span)
}

fn discover_source_units(
    root: &Path,
    namespace_roots: &[NamespaceRoot],
    prelude: bool,
) -> Result<Vec<SourceUnit>, Vec<PackageLoadError>> {
    let mut discovered = BTreeMap::<PathBuf, (usize, String)>::new();
    let mut errors = Vec::new();
    for mapping in namespace_roots {
        let directory = root.join(&mapping.directory);
        let mut paths = BTreeSet::new();
        let errors_before_discovery = errors.len();
        discover_trn_files(&directory, root, &mut paths, &mut errors);
        if paths.is_empty() && errors.len() == errors_before_discovery {
            errors.push(PackageLoadError::unreadable(
                directory,
                format!(
                    "namespace root `{}` contains no `.trn` source files",
                    mapping.namespace
                ),
            ));
        }
        let depth = mapping.directory.components().count();
        for relative_path in paths {
            let suffix = relative_path
                .parent()
                .expect("discovered source has a parent")
                .strip_prefix(&mapping.directory)
                .expect("discovered source is beneath its normalized namespace root");
            let expected = match expected_namespace(&mapping.namespace, suffix) {
                Ok(expected) => expected,
                Err(message) => {
                    errors.push(PackageLoadError::unreadable(
                        root.join(&relative_path),
                        message,
                    ));
                    continue;
                }
            };
            match discovered.get(&relative_path) {
                Some((existing_depth, _)) if *existing_depth >= depth => {}
                _ => {
                    discovered.insert(relative_path, (depth, expected));
                }
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let mut units = Vec::with_capacity(discovered.len());
    for (id, (relative_path, (_, expected_namespace))) in discovered.into_iter().enumerate() {
        let source_path = root.join(&relative_path);
        let Ok(source_id) = u32::try_from(id) else {
            errors.push(PackageLoadError::unreadable(
                source_path,
                "package has too many source units",
            ));
            continue;
        };
        match fs::read_to_string(&source_path) {
            Ok(source_text) => units.push(SourceUnit {
                relative_path,
                source: SourceFile::new(source_id, source_path, source_text),
                expected_namespace: Some(expected_namespace),
                prelude,
                role: SourceRole::Production,
            }),
            Err(error) => errors.push(PackageLoadError::unreadable(
                source_path,
                format!("cannot read package source: {error}"),
            )),
        }
    }
    if errors.is_empty() {
        Ok(units)
    } else {
        Err(errors)
    }
}

fn discover_trn_files(
    directory: &Path,
    root: &Path,
    paths: &mut BTreeSet<PathBuf>,
    errors: &mut Vec<PackageLoadError>,
) {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(PackageLoadError::unreadable(
                directory.to_path_buf(),
                format!("cannot read namespace directory: {error}"),
            ));
            return;
        }
    };
    let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) if file_type.is_symlink() => match fs::metadata(&path) {
                Ok(metadata) => metadata.file_type(),
                Err(error) => {
                    errors.push(PackageLoadError::unreadable(
                        path,
                        format!("cannot inspect symlinked namespace source entry: {error}"),
                    ));
                    continue;
                }
            },
            Ok(file_type) => file_type,
            Err(_) => {
                errors.push(PackageLoadError::unreadable(
                    path,
                    "cannot inspect namespace source entry",
                ));
                continue;
            }
        };
        if file_type.is_dir() {
            discover_trn_files(&path, root, paths, errors);
        } else if file_type.is_file()
            && path.extension().is_some_and(|extension| extension == "trn")
        {
            match path.strip_prefix(root) {
                Ok(relative) => {
                    paths.insert(relative.to_path_buf());
                }
                Err(_) => errors.push(PackageLoadError::unreadable(
                    path,
                    "discovered source escapes the package root",
                )),
            }
        }
    }
}

fn expected_namespace(root: &str, suffix: &Path) -> Result<String, String> {
    let mut segments = Vec::new();
    for component in suffix.components() {
        let std::path::Component::Normal(value) = component else {
            return Err("source directory is not a normalized relative path".to_owned());
        };
        let Some(segment) = value.to_str() else {
            return Err("source directory contains a non-UTF-8 namespace segment".to_owned());
        };
        if !valid_namespace_segment(segment) {
            return Err(format!(
                "source directory segment `{segment}` must match `[a-z]([a-z0-9]|-[a-z0-9])*`"
            ));
        }
        if reserved_namespace_segment(segment) {
            return Err(format!("source directory segment `{segment}` is reserved"));
        }
        segments.push(segment);
    }
    if segments.is_empty() {
        Ok(root.to_owned())
    } else {
        Ok(format!("{root}/{}", segments.join("/")))
    }
}

fn valid_namespace_segment(segment: &str) -> bool {
    let mut bytes = segment.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() {
        return false;
    }
    let mut previous_hyphen = false;
    for byte in bytes {
        if byte == b'-' {
            if previous_hyphen {
                return false;
            }
            previous_hyphen = true;
        } else if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
            previous_hyphen = false;
        } else {
            return false;
        }
    }
    !previous_hyphen
}

fn reserved_namespace_segment(segment: &str) -> bool {
    matches!(segment, "con" | "prn" | "aux" | "nul")
        || segment
            .strip_prefix("com")
            .or_else(|| segment.strip_prefix("lpt"))
            .is_some_and(|suffix| suffix.len() == 1 && matches!(suffix.as_bytes()[0], b'1'..=b'9'))
}

fn normalized_relative_directory(value: &str) -> Option<PathBuf> {
    let path = Path::new(value);
    if value.is_empty() || path.is_absolute() {
        return None;
    }
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::CurDir => {}
            std::path::Component::Normal(component) => normalized.push(component),
            _ => return None,
        }
    }
    Some(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implicit_source_paths_are_normalized_relative_to_the_package_root() {
        let package = Package::implicit("/workspace/example/app/main.trn", String::new());
        assert_eq!(package.root, PathBuf::from("/workspace/example/app"));
        assert_eq!(package.units[0].relative_path, PathBuf::from("main.trn"));
        assert_eq!(package.units[0].relative_path_text(), "main.trn");
    }

    #[test]
    fn package_relative_paths_have_platform_independent_text() {
        let unit = SourceUnit {
            relative_path: ["app", "support", "values.trn"].iter().collect(),
            source: SourceFile::new(0, PathBuf::from("values.trn"), String::new()),
            expected_namespace: None,
            prelude: true,
            role: SourceRole::Production,
        };
        assert_eq!(unit.relative_path_text(), "app/support/values.trn");
    }

    #[test]
    fn arbitrary_source_paths_render_without_panicking() {
        for (path, expected) in [
            ("../shared/main.trn", "../shared/main.trn"),
            ("/workspace/main.trn", "/workspace/main.trn"),
            (r"C:\workspace\main.trn", "C:/workspace/main.trn"),
        ] {
            let unit = SourceUnit {
                relative_path: PathBuf::from(path),
                source: SourceFile::new(0, PathBuf::from(path), String::new()),
                expected_namespace: None,
                prelude: true,
                role: SourceRole::Production,
            };
            assert_eq!(unit.relative_path_text(), expected);
        }
    }

    #[test]
    fn rust_dependency_manifest_parts_preserve_target_alias_and_features() {
        let dependency = RustDependency {
            name: "date-codec".to_owned(),
            package: "httpdate".to_owned(),
            version: "=1.0.3".to_owned(),
            features: vec!["clock".to_owned(), "serde".to_owned()],
            default_features: false,
            target: Some("cfg(unix)".to_owned()),
            effects: Vec::new(),
        };
        assert_eq!(
            dependency.cargo_manifest_table(),
            "target.\"cfg(unix)\".dependencies"
        );
        assert_eq!(
            dependency.cargo_dependency_spec(),
            "date-codec = { package = \"httpdate\", version = \"=1.0.3\", default-features = false, features = [\"clock\", \"serde\"] }\n"
        );
    }
    #[test]
    fn authored_rust_module_paths_cannot_escape_the_package() {
        let manifest = r#"
package = "example"

[namespaces]
app = "src"

[rust-modules]
adapters = "../escape.rs"
"#;
        let Err(errors) = parse_manifest(Path::new("package.toml"), manifest) else {
            panic!("parent traversal must be rejected");
        };
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].diagnostic.message,
            "authored Rust module `adapters` must name a relative `.rs` path"
        );
    }
}
