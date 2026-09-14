use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::{
    CapabilityProfile, Diagnostic, Package, PackageLoadError, PanicProfile, SourceFile, SourceUnit,
    Span,
};

/// Conventional isolation tier for a Terrane test source.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TestTier {
    Unit,
    Integration,
    EndToEnd,
}

impl TestTier {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Unit => "unit",
            Self::Integration => "integration",
            Self::EndToEnd => "end-to-end",
        }
    }
}

/// Manifest-selected roots and capability profile used by `terrane test`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestConfiguration {
    pub roots: BTreeMap<TestTier, PathBuf>,
    pub profile: CapabilityProfile,
}

/// A package augmented with its test-only source set.
#[derive(Clone, Debug)]
pub struct TestPackage {
    pub package: Package,
    pub configuration: TestConfiguration,
    pub source_tiers: BTreeMap<u32, TestTier>,
}

/// One compiler-discovered ordinary Terrane test function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestCase {
    pub identity: String,
    pub tier: TestTier,
    pub source_path: String,
    pub source_span: Span,
    pub is_async: bool,
    pub throws: bool,
}

impl TestPackage {
    /// Loads production sources plus the conventional or manifest-overridden test roots.
    ///
    /// Test roots are optional. Every discovered test source remains an ordinary package input and
    /// therefore participates in parsing, semantic analysis, lowering, and cache identity.
    ///
    /// # Errors
    ///
    /// Returns package, manifest, containment, or source-read diagnostics.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Vec<PackageLoadError>> {
        let requested = path.as_ref();
        let manifest_path = if requested.is_dir() {
            requested.join(crate::MANIFEST_FILE_NAME)
        } else {
            requested.to_path_buf()
        };
        let root = manifest_path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();
        let text = fs::read_to_string(&manifest_path).map_err(|error| {
            vec![test_load_error(
                manifest_path.clone(),
                format!("cannot read package manifest: {error}"),
            )]
        })?;
        let table = text.parse::<toml::Table>().map_err(|error| {
            vec![test_load_error(
                manifest_path.clone(),
                format!("invalid TOML: {error}"),
            )]
        })?;
        let mut package = Package::load(&manifest_path)?;
        let configuration = parse_configuration(&manifest_path, &text, &table, &package.profile)?;
        package.profile = configuration.profile.clone();

        let mut source_tiers = BTreeMap::new();
        let mut paths = BTreeMap::<PathBuf, TestTier>::new();
        let mut errors = Vec::new();
        for (tier, relative_root) in &configuration.roots {
            let absolute_root = root.join(relative_root);
            if !absolute_root.exists() {
                continue;
            }
            discover_test_files(&absolute_root, &root, *tier, &mut paths, &mut errors);
        }
        for unit in &package.units {
            if let Some((_, tier)) = paths.get_key_value(&unit.relative_path) {
                errors.push(test_load_error(
                    unit.source.path().to_path_buf(),
                    format!(
                        "{} test source is also included in the production namespace roots",
                        tier.name()
                    ),
                ));
            }
        }
        let first_test_id = u32::try_from(package.units.len()).map_err(|_| {
            vec![test_load_error(
                manifest_path.clone(),
                "package has too many source units".to_owned(),
            )]
        })?;
        for (offset, (relative_path, tier)) in paths.into_iter().enumerate() {
            let source_path = root.join(&relative_path);
            let Some(source_id) = u32::try_from(offset)
                .ok()
                .and_then(|offset| first_test_id.checked_add(offset))
            else {
                errors.push(test_load_error(
                    source_path,
                    "package has too many test sources".to_owned(),
                ));
                continue;
            };
            match fs::read_to_string(&source_path) {
                Ok(source) => {
                    package.units.push(SourceUnit {
                        relative_path,
                        source: SourceFile::new(source_id, source_path, source),
                        expected_namespace: None,
                    });
                    source_tiers.insert(source_id, tier);
                }
                Err(error) => errors.push(test_load_error(
                    source_path,
                    format!("cannot read test source: {error}"),
                )),
            }
        }
        if errors.is_empty() {
            Ok(Self {
                package,
                configuration,
                source_tiers,
            })
        } else {
            Err(errors)
        }
    }
}

fn parse_configuration(
    manifest_path: &Path,
    text: &str,
    table: &toml::Table,
    ordinary_profile: &CapabilityProfile,
) -> Result<TestConfiguration, Vec<PackageLoadError>> {
    let mut roots = BTreeMap::from([
        (TestTier::Unit, PathBuf::from("tests/unit")),
        (TestTier::Integration, PathBuf::from("tests/integration")),
        (TestTier::EndToEnd, PathBuf::from("tests/end-to-end")),
    ]);
    let mut profile = ordinary_profile.clone();
    let Some(testing) = table.get("testing") else {
        return Ok(TestConfiguration { roots, profile });
    };
    let Some(testing) = testing.as_table() else {
        return Err(vec![manifest_test_error(
            manifest_path,
            text,
            "`testing` must be a table",
            "testing",
        )]);
    };
    let mut errors = Vec::new();
    for key in testing.keys() {
        if !matches!(
            key.as_str(),
            "unit" | "integration" | "end-to-end" | "profile"
        ) {
            errors.push(manifest_test_error(
                manifest_path,
                text,
                format!("unknown testing field `{key}`"),
                key,
            ));
        }
    }
    for (key, tier) in [
        ("unit", TestTier::Unit),
        ("integration", TestTier::Integration),
        ("end-to-end", TestTier::EndToEnd),
    ] {
        if let Some(value) = testing.get(key) {
            match value.as_str().and_then(normalized_relative_directory) {
                Some(path) => {
                    roots.insert(tier, path);
                }
                None => errors.push(manifest_test_error(
                    manifest_path,
                    text,
                    format!("testing root `{key}` must be a bounded relative path"),
                    key,
                )),
            }
        }
    }
    if let Some(value) = testing.get("profile") {
        let Some(fields) = value.as_table() else {
            errors.push(manifest_test_error(
                manifest_path,
                text,
                "testing `profile` must be a table",
                "profile",
            ));
            return if errors.is_empty() {
                Ok(TestConfiguration { roots, profile })
            } else {
                Err(errors)
            };
        };
        for key in fields.keys() {
            if !matches!(key.as_str(), "name" | "capabilities" | "panic") {
                errors.push(manifest_test_error(
                    manifest_path,
                    text,
                    format!("unknown testing profile field `{key}`"),
                    key,
                ));
            }
        }
        if let Some(name) = fields.get("name").and_then(toml::Value::as_str) {
            if name.is_empty() {
                errors.push(manifest_test_error(
                    manifest_path,
                    text,
                    "testing profile `name` must not be empty",
                    "name",
                ));
            } else {
                profile.name = name.to_owned();
            }
        }
        if let Some(capabilities) = fields.get("capabilities") {
            match capabilities.as_array() {
                Some(values) if values.iter().all(|value| value.as_str().is_some()) => {
                    profile.capabilities = Some(
                        values
                            .iter()
                            .filter_map(toml::Value::as_str)
                            .map(str::to_owned)
                            .collect::<BTreeSet<_>>(),
                    );
                }
                _ => errors.push(manifest_test_error(
                    manifest_path,
                    text,
                    "testing profile `capabilities` must be an array of strings",
                    "capabilities",
                )),
            }
        }
        if let Some(panic) = fields.get("panic") {
            profile.panic = match panic.as_str() {
                Some("unwind") => PanicProfile::Unwind,
                Some("abort") => PanicProfile::Abort,
                _ => {
                    errors.push(manifest_test_error(
                        manifest_path,
                        text,
                        "testing profile `panic` must be `unwind` or `abort`",
                        "panic",
                    ));
                    profile.panic
                }
            };
        }
    }
    if errors.is_empty() {
        Ok(TestConfiguration { roots, profile })
    } else {
        Err(errors)
    }
}

fn normalized_relative_directory(value: &str) -> Option<PathBuf> {
    let path = Path::new(value);
    if value.is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            !matches!(component, Component::Normal(_)) || component.as_os_str().to_str().is_none()
        })
    {
        None
    } else {
        Some(path.to_path_buf())
    }
}

fn discover_test_files(
    directory: &Path,
    package_root: &Path,
    tier: TestTier,
    paths: &mut BTreeMap<PathBuf, TestTier>,
    errors: &mut Vec<PackageLoadError>,
) {
    let entries = match fs::read_dir(directory) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(test_load_error(
                directory.to_path_buf(),
                format!("cannot read test root: {error}"),
            ));
            return;
        }
    };
    let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
    entries.sort_by_key(std::fs::DirEntry::path);
    for entry in entries {
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                errors.push(test_load_error(
                    path,
                    format!("cannot inspect test source: {error}"),
                ));
                continue;
            }
        };
        if file_type.is_symlink() {
            errors.push(test_load_error(
                path,
                "test roots may not contain symbolic links".to_owned(),
            ));
        } else if file_type.is_dir() {
            discover_test_files(&path, package_root, tier, paths, errors);
        } else if file_type.is_file()
            && path.extension().is_some_and(|extension| extension == "trn")
        {
            let relative = path
                .strip_prefix(package_root)
                .expect("test discovery remains beneath the package root")
                .to_path_buf();
            if let Some(previous) = paths.insert(relative.clone(), tier) {
                errors.push(test_load_error(
                    path,
                    format!(
                        "test source belongs to both {} and {} roots",
                        previous.name(),
                        tier.name()
                    ),
                ));
            }
        }
    }
}

fn manifest_test_error(
    path: &Path,
    text: &str,
    message: impl Into<String>,
    needle: &str,
) -> PackageLoadError {
    let start = text.find(needle).unwrap_or(0);
    PackageLoadError {
        source: SourceFile::new(0, path.to_path_buf(), text.to_owned()),
        diagnostic: Diagnostic::error(
            "S2050",
            message,
            Span::new(0, start, start.saturating_add(needle.len())),
        ),
    }
}

fn test_load_error(path: PathBuf, message: String) -> PackageLoadError {
    PackageLoadError {
        source: SourceFile::new(0, path, String::new()),
        diagnostic: Diagnostic::unlocated_error("S2050", message),
    }
}
