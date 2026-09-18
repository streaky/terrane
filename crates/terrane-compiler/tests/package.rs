use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use terrane_compiler::{
    ArtifactKind, BuildToolchain, CompilerOptions, IMPLICIT_PACKAGE_ID, Package, PanicProfile,
    RustDependency, analyze, compile_discovered_test_tier, compile_package, compile_test_package,
    discover_test_package, source_tree_hash,
    testing::{TestPackage, TestTier},
    with_tokio_runtime,
};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

struct TempPackage(std::path::PathBuf);

impl TempPackage {
    fn new() -> Self {
        let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("terrane-package-{}-{serial}", std::process::id()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn write(&self, path: &str, text: &str) {
        let path = self.0.join(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, text).unwrap();
    }
}

impl Drop for TempPackage {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn implicit_source_has_stable_package_contract() {
    let package = Package::implicit("examples/hello.trn", "namespace hello\n".to_owned());

    assert_eq!(package.identity, IMPLICIT_PACKAGE_ID);
    assert!(package.prelude);
    assert_eq!(package.root, Path::new("examples"));
    assert_eq!(package.units.len(), 1);
    assert_eq!(package.units[0].relative_path, Path::new("hello.trn"));
    assert_eq!(package.units[0].source.id(), 0);
}

#[test]
fn bare_implicit_source_uses_current_directory_as_root() {
    let package = Package::implicit("hello.trn", "namespace hello\n".to_owned());

    assert_eq!(package.root, Path::new("."));
    assert_eq!(package.units[0].relative_path, Path::new("hello.trn"));
}

#[test]
fn tokio_runtime_features_merge_by_package_identity() {
    let declared = RustDependency {
        name: "runtime".to_owned(),
        package: "tokio".to_owned(),
        version: "=1.53.0".to_owned(),
        features: vec!["net".to_owned()],
        default_features: false,
        target: None,
        effects: vec!["networking".to_owned()],
    };

    let merged = with_tokio_runtime(&[declared], &["rt", "time"]);

    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].name, "runtime");
    assert_eq!(merged[0].features, ["net", "rt", "time"]);
}

#[test]
fn manifest_discovers_sources_in_deterministic_path_order() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "# discovered source set\npackage = \"example.tools\"\nprelude = false\n[namespaces]\nzed = \"zed\"\nalpha = \"nested\"\n",
    );
    package.write("zed/zed.trn", "namespace zed\n");
    package.write("nested/alpha.trn", "namespace alpha\n");

    let loaded = Package::load(&package.0).unwrap();

    assert_eq!(loaded.identity, "example.tools");
    assert!(!loaded.prelude);
    assert_eq!(
        loaded
            .units
            .iter()
            .map(|unit| unit.relative_path.as_path())
            .collect::<Vec<_>>(),
        [Path::new("nested/alpha.trn"), Path::new("zed/zed.trn")]
    );
    assert_eq!(loaded.units[0].source.id(), 0);
    assert_eq!(loaded.units[1].source.id(), 1);
}

#[test]
fn manifest_can_explicitly_use_the_system_rust_toolchain() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"example.system-toolchain\"\nrust-toolchain = \"system\"\n[namespaces]\nexample = \"src\"\n",
    );
    package.write("src/main.trn", "namespace example\nfunction main;\n");

    let loaded = Package::load(&package.0).unwrap();

    assert_eq!(loaded.build_toolchain, BuildToolchain::System);
}

#[test]
fn manifest_can_select_a_dynamic_library_artifact() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"example.extension\"\nartifact = \"dynamic-library\"\n[namespaces]\nexample = \"src\"\n",
    );
    package.write("src/main.trn", "namespace example\nfunction main;\n");

    let loaded = Package::load(&package.0).unwrap();

    assert_eq!(loaded.artifact, ArtifactKind::DynamicLibrary);
}

#[test]
fn manifest_rejects_an_unknown_artifact_kind() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"example.extension\"\nartifact = \"shared\"\n[namespaces]\nexample = \"src\"\n",
    );
    package.write("src/main.trn", "namespace example\nfunction main;\n");

    let errors = Package::load(&package.0).unwrap_err();

    assert_eq!(errors.len(), 1);
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("`executable` or `dynamic-library`")
    );
}

#[test]
fn package_compilation_parses_every_discovered_unit() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"example.multi\"\n[namespaces]\nhello = \"hello\"\n\"hello/helpers\" = \"hello/helpers\"\n",
    );
    package.write(
        "hello/helpers/support.trn",
        "namespace hello/helpers\nconstant value = 1\n",
    );
    package.write(
        "hello/main.trn",
        "namespace hello\nfrom /core/output import print\nfunction main;\n  print; >package pipeline\n",
    );

    let loaded = Package::load(&package.0).unwrap();
    let compilation = compile_package(&loaded).unwrap();

    assert!(compilation.rust.contains("// Namespace: hello\n"));
    assert!(
        compilation
            .rust
            .contains("String::from(\"package pipeline\")")
    );
}

#[test]
fn package_compilation_emits_functions_and_bindings_from_every_unit() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"example.multi\"\n[namespaces]\nhello = \"src\"\n",
    );
    package.write(
        "src/main.trn",
        "namespace hello\nfrom /core/output import print\nfunction main;\n  print; (helper;)\n",
    );
    package.write(
        "src/support.trn",
        "namespace hello\nconstant value int = 41\nfunction helper int;\n  return value + 1\n",
    );

    let compilation = compile_package(&Package::load(&package.0).unwrap()).unwrap();

    assert!(
        compilation
            .rust
            .contains("static __TERRANE_F1_VALUE: std::sync::LazyLock<terrane_int_support::Int>")
    );
    assert!(
        compilation
            .rust
            .contains("fn helper() -> terrane_int_support::Int")
    );
    assert!(compilation.rust.contains(
        "return (*__TERRANE_F1_VALUE).clone() + terrane_int_support::Int::from(1_i128);"
    ));
}

#[test]
fn package_entry_point_comes_from_resolved_function_declarations() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"example.entry\"\n[namespaces]\ndecoy = \"decoy\"\nactual = \"actual\"\n",
    );
    package.write(
        "decoy/decoy.trn",
        "namespace decoy\nconstant text = >>\n  function main;\n",
    );
    package.write(
        "actual/main.trn",
        "namespace actual\nfrom /core/output import print\nfunction main;\n  print; >real entry\n",
    );

    let compilation = compile_package(&Package::load(&package.0).unwrap()).unwrap();

    assert!(compilation.rust.contains("// Namespace: actual\n"));
    assert!(compilation.rust.contains("String::from(\"real entry\")"));
}

#[test]
fn package_requires_one_unambiguous_main_function() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"example.entry\"\n[namespaces]\nfirst = \"first\"\nsecond = \"second\"\n",
    );
    package.write("first/first.trn", "namespace first\nconstant value = 1\n");
    package.write(
        "second/second.trn",
        "namespace second\nconstant value = 2\n",
    );

    let missing = compile_package(&Package::load(&package.0).unwrap()).unwrap_err();
    assert_eq!(missing.diagnostics[0].code, "S2015");

    package.write("first/first.trn", "namespace first\nfunction main;\n");
    package.write("second/second.trn", "namespace second\nfunction main;\n");
    let ambiguous = compile_package(&Package::load(&package.0).unwrap()).unwrap_err();
    assert_eq!(ambiguous.diagnostics[0].code, "S2016");
}

#[test]
fn syntax_failure_in_non_main_unit_stops_package_compilation() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"example.invalid\"\n[namespaces]\nhello = \"src\"\n",
    );
    package.write(
        "src/main.trn",
        "namespace hello\nfrom /core/output import print\nfunction main;\n  print; >unreachable\n",
    );
    package.write("src/support.trn", "namespace hello\nvalue =\n");

    let failure = compile_package(&Package::load(&package.0).unwrap()).unwrap_err();

    assert!(failure.source.path().ends_with("support.trn"));
    assert_eq!(failure.diagnostics[0].code, "S1019");
}

#[test]
fn malformed_manifests_report_all_manifest_errors() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "prelude = \"perhaps\"\nmystery = \"field\"\n[namespaces]\n\"Bad Root\" = \"../escape\"\nvalid = \"../escape\"\n",
    );

    let errors = Package::load(&package.0).unwrap_err();
    let messages = errors
        .iter()
        .map(|error| error.diagnostic.message.as_str())
        .collect::<Vec<_>>();

    assert!(messages.iter().any(|message| message.contains("prelude")));
    assert!(
        messages
            .iter()
            .any(|message| message.contains("must match `[a-z]"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("relative path"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("unknown manifest field"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("missing `package`"))
    );
}

#[test]
fn manifest_package_drives_complete_namespace_and_scope_resolution() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        concat!(
            "package = \"namespace-contract\"\n",
            "prelude = false\n",
            "[namespaces]\n",
            "shared = \"shared\"\n",
            "\"app/support\" = \"app/support\"\n",
            "\"app/child\" = \"app/child\"\n",
        ),
    );
    package.write(
        "shared/exports.trn",
        "namespace shared\npublic constant item = 1\n",
    );
    package.write(
        "app/support/parent.trn",
        "namespace app/support\npublic constant parent = 1\n",
    );
    package.write(
        "app/child/consumer.trn",
        concat!(
            "namespace app/child\n",
            "from /core/types import int\n",
            "from /shared import item\n",
            "from ../support import parent\n",
            "function run; argument int\n",
            "  from /core/output import print as local-print\n",
            "  value = argument\n",
        ),
    );

    let loaded = Package::load(&package.0).unwrap();
    let analyzed = analyze(&loaded).unwrap();
    let consumer = analyzed
        .units
        .iter()
        .find(|unit| unit.namespace == "/app/child")
        .unwrap();
    let body_offset = consumer.source.text().find("value =").unwrap();

    assert_eq!(analyzed.identity, "namespace-contract");
    assert!(!analyzed.prelude);
    assert!(analyzed.symbol("/app/child", "item").is_some());
    assert!(analyzed.symbol("/app/child", "parent").is_some());
    assert!(
        analyzed
            .resolve_name_at(consumer, body_offset, "argument")
            .is_some()
    );
    assert!(
        analyzed
            .resolve_name_at(consumer, body_offset, "local-print")
            .is_some()
    );
}

#[test]
fn missing_namespace_directories_are_package_errors() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"missing-source\"\n[namespaces]\nabsent = \"absent\"\n",
    );

    let errors = Package::load(package.0.join("package.toml")).unwrap_err();

    assert!(!errors.is_empty());
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("cannot read namespace directory")
    );
}

#[test]
fn empty_namespace_roots_are_package_errors() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"empty-root\"\n[namespaces]\napp = \"app\"\ntools = \"tools\"\n",
    );
    package.write("app/main.trn", "namespace app\n");
    fs::create_dir_all(package.0.join("tools")).unwrap();

    let errors = Package::load(package.0.join("package.toml")).unwrap_err();

    assert_eq!(errors.len(), 1);
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("namespace root `/tools` contains no `.trn` source files")
    );
}

#[test]
fn namespace_mappings_discover_sources_in_sorted_order() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"mapped\"\nprelude = false\n[namespaces]\napp = \"src\"\n\"app/private\" = \"src/http/private\"\n",
    );
    package.write("src/zeta.trn", "namespace app\n");
    package.write("src/http/beta.trn", "namespace app/http\n");
    package.write("src/http/alpha.trn", "namespace app/http\n");
    package.write("src/http/private/internal.trn", "namespace app/private\n");
    package.write("outside.trn", "namespace ignored\n");

    let loaded = Package::load(&package.0).unwrap();
    assert_eq!(
        loaded
            .units
            .iter()
            .map(|unit| unit.relative_path.as_path())
            .collect::<Vec<_>>(),
        [
            Path::new("src/http/alpha.trn"),
            Path::new("src/http/beta.trn"),
            Path::new("src/http/private/internal.trn"),
            Path::new("src/zeta.trn"),
        ]
    );
    assert_eq!(
        loaded
            .units
            .iter()
            .map(|unit| unit.expected_namespace.as_deref().unwrap())
            .collect::<Vec<_>>(),
        ["/app/http", "/app/http", "/app/private", "/app"]
    );
}

#[test]
fn namespace_mappings_reject_duplicate_directories() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"mapped\"\n[namespaces]\napp = \"src\"\ntools = \"src\"\n",
    );

    let errors = Package::load(&package.0).unwrap_err();
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("map to the same directory")
    );
}

#[test]
fn semantic_analysis_checks_mapped_directory_correspondence() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"mapped\"\n[namespaces]\napp = \"src\"\n",
    );
    package.write("src/http/main.trn", "namespace app/wrong\n");

    let failure = analyze(&Package::load(&package.0).unwrap()).unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2020");
    assert!(
        failure.diagnostics[0]
            .message
            .contains("does not match `/app/http`")
    );
}

#[test]
fn namespace_mapping_directories_are_normalized_before_discovery_and_deduplication() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"mapped\"\n[namespaces]\napp = \"./src\"\ntools = \"src\"\n",
    );
    package.write("src/http/main.trn", "namespace app/http\n");

    let errors = Package::load(&package.0).unwrap_err();
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("map to the same directory")
    );

    package.write(
        "package.toml",
        "package = \"mapped\"\n[namespaces]\napp = \"./src\"\n",
    );
    let loaded = Package::load(&package.0).unwrap();
    assert_eq!(
        loaded.units[0].expected_namespace.as_deref(),
        Some("/app/http")
    );
}

#[test]
fn namespace_mapping_rejects_invalid_roots_and_directory_segments() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"mapped\"\n[namespaces]\n\"/\" = \"root\"\n\"app/con\" = \"reserved\"\n\"app/bad-\" = \"bad\"\nvalid = \"src\"\n",
    );
    package.write("src/Http/main.trn", "namespace valid/http\n");

    let errors = Package::load(&package.0).unwrap_err();
    let messages = errors
        .iter()
        .map(|error| error.diagnostic.message.as_str())
        .collect::<Vec<_>>();
    assert!(
        messages
            .iter()
            .any(|message| message.contains("cannot be declared"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("`con` is reserved"))
    );
    assert!(
        messages
            .iter()
            .any(|message| message.contains("`bad-` must match"))
    );
    package.write(
        "package.toml",
        "package = \"mapped\"\n[namespaces]\nvalid = \"src\"\n",
    );
    let errors = Package::load(&package.0).unwrap_err();
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("directory segment `Http` must match")
    );
}

#[cfg(unix)]
#[test]
fn namespace_discovery_follows_symlinked_source_files() {
    use std::os::unix::fs::symlink;

    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"mapped\"\n[namespaces]\napp = \"src\"\n",
    );
    package.write("elsewhere/helper.trn", "namespace app\n");
    fs::create_dir_all(package.0.join("src")).unwrap();
    symlink("../elsewhere/helper.trn", package.0.join("src/helper.trn")).unwrap();

    let loaded = Package::load(&package.0).unwrap();
    assert_eq!(loaded.units.len(), 1);
    assert_eq!(loaded.units[0].relative_path, Path::new("src/helper.trn"));
}

#[test]
fn parse_callbacks_resolve_through_import_aliases() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"callback-alias\"\n[namespaces]\ncallbacks = \"callbacks\"\napp = \"app\"\n",
    );
    package.write(
        "callbacks/parse.trn",
        "namespace callbacks\npublic function decode int; text string\n  return text.radix; 10\n",
    );
    package.write(
        "app/main.trn",
        "namespace app\nfrom /callbacks import decode as parse-decimal\nfunction main;\n  text string = >42\n  value int = text.parse; parse-decimal\n",
    );

    let compilation = compile_package(&Package::load(&package.0).unwrap()).unwrap();

    assert!(compilation.rust.contains("parse_radix"));
    assert!(compilation.rust.contains("decode"));
}

#[test]
fn rust_dependency_manifest_preserves_resolution_inputs() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"dependencies\"\n[namespaces]\napp = \"src\"\n[rust-dependencies.http]\npackage = \"reqwest\"\nversion = \"=0.12.23\"\nfeatures = [\"blocking\", \"rustls-tls-webpki-roots\"]\ndefault-features = false\ntarget = \"x86_64-unknown-linux-gnu\"\n",
    );
    package.write("src/main.trn", "namespace app\nfunction main;\n");

    let loaded = Package::load(&package.0).unwrap();

    assert_eq!(loaded.rust_dependencies.len(), 1);
    let dependency = &loaded.rust_dependencies[0];
    assert_eq!(dependency.name, "http");
    assert_eq!(dependency.package, "reqwest");
    assert_eq!(dependency.version, "=0.12.23");
    assert_eq!(dependency.features, ["blocking", "rustls-tls-webpki-roots"]);
    assert!(!dependency.default_features);
    assert_eq!(
        dependency.target.as_deref(),
        Some("x86_64-unknown-linux-gnu")
    );
}

#[test]
fn capability_profile_and_dependency_effects_are_loaded() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"profiled\"\n[profile]\nname = \"service\"\ncapabilities = [\"build\", \"networking\", \"tls\"]\npanic = \"abort\"\n[namespaces]\napp = \"src\"\n[rust-dependencies.http]\npackage = \"httpdate\"\nversion = \"=1.0.3\"\neffects = [\"networking\"]\n",
    );
    package.write("src/main.trn", "namespace app\nfunction main;\n");

    let loaded = Package::load(&package.0).unwrap();

    assert_eq!(loaded.profile.name, "service");
    assert_eq!(loaded.profile.panic, PanicProfile::Abort);
    assert!(loaded.profile.allows("networking"));
    assert!(!loaded.profile.allows("filesystem"));
    assert_eq!(loaded.rust_dependencies[0].effects, ["networking"]);
}

#[test]
fn dependency_effect_outside_selected_profile_is_rejected() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"profiled\"\n[profile]\nname = \"restricted\"\ncapabilities = [\"build\", \"filesystem\"]\n[namespaces]\napp = \"src\"\n[rust-dependencies.http]\npackage = \"httpdate\"\nversion = \"=1.0.3\"\neffects = [\"networking\"]\n",
    );
    package.write("src/main.trn", "namespace app\nfunction main;\n");

    let errors = Package::load(&package.0).unwrap_err();

    assert!(errors.iter().any(|error| {
        error
            .diagnostic
            .message
            .contains("forbids effect `networking`")
    }));
}

#[test]
fn unknown_capability_and_effect_names_are_rejected() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"profiled\"\n[profile]\ncapabilities = [\"networking\", \"telepathy\"]\n[namespaces]\napp = \"src\"\n[rust-dependencies.http]\npackage = \"httpdate\"\nversion = \"=1.0.3\"\neffects = [\"networking\", \"prophecy\"]\n",
    );
    package.write("src/main.trn", "namespace app\nfunction main;\n");

    let errors = Package::load(&package.0).unwrap_err();
    let messages = errors
        .iter()
        .map(|error| error.diagnostic.message.as_str())
        .collect::<Vec<_>>();

    assert!(messages.contains(&"unknown profile capability `telepathy`"));
    assert!(messages.contains(&"Rust dependency `http` declares unknown effect `prophecy`"));
}

#[test]
fn test_packages_discover_and_lower_tiered_ordinary_functions() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"native-tests\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    );
    package.write(
        "src/library.trn",
        "namespace app\npublic constant value = 1\n",
    );
    package.write(
        "tests/unit/z-last.trn",
        "namespace app\nasync function test-later;\n",
    );
    package.write(
        "tests/unit/a-first.trn",
        "namespace app\nfunction test-first;\n",
    );
    package.write(
        "tests/integration/public.trn",
        "namespace app-tests\nfrom /app import value\nfunction test-public;\n  value\n",
    );

    let test_package = TestPackage::load(&package.0).unwrap();
    let compiled = compile_test_package(
        &test_package,
        CompilerOptions {
            require_canonical_rust: true,
            ..CompilerOptions::default()
        },
    )
    .unwrap();
    let cases = compiled
        .iter()
        .flat_map(|tier| tier.cases.iter())
        .collect::<Vec<_>>();

    assert_eq!(
        cases
            .iter()
            .map(|case| (case.tier, case.identity.as_str()))
            .collect::<Vec<_>>(),
        [
            (TestTier::Unit, "/app::test-first"),
            (TestTier::Unit, "/app::test-later"),
            (TestTier::Integration, "/app-tests::test-public"),
        ]
    );
    assert_eq!(compiled.len(), 2);
    assert!(
        compiled
            .iter()
            .all(|tier| tier.compilation.rust.contains("match selected.as_str()"))
    );
    assert!(
        compiled
            .iter()
            .all(|tier| tier.compilation.rust.contains("\"0\" =>"))
    );
    assert!(
        compiled
            .iter()
            .find(|tier| tier.tier == TestTier::Unit)
            .unwrap()
            .compilation
            .rust
            .contains("__terrane_run(async move")
    );
}

#[test]
fn discovered_test_tiers_lower_without_rediscovery_warnings() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"discovered-native-tests\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    );
    package.write(
        "src/library.trn",
        "namespace app\npublic constant value = 1\n",
    );
    package.write(
        "tests/unit/case.trn",
        "namespace app\nfunction test-discovered;\n  value\n",
    );
    let test_package = TestPackage::load(&package.0).unwrap();
    let mut discovered = discover_test_package(&test_package, CompilerOptions::default()).unwrap();

    assert_eq!(discovered.len(), 1);
    assert!(
        discovered[0]
            .warnings
            .iter()
            .all(|warning| warning.code != "W4005")
    );
    let compiled =
        compile_discovered_test_tier(discovered.pop().unwrap(), CompilerOptions::default())
            .unwrap();
    assert_eq!(compiled.cases[0].identity, "/app::test-discovered");
    assert!(
        compiled
            .compilation
            .rust
            .contains("match selected.as_str()")
    );
}

#[test]
fn invalid_test_signatures_are_source_diagnostics() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"invalid-native-tests\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    );
    package.write("src/library.trn", "namespace app\nconstant value = 1\n");
    package.write(
        "tests/unit/case.trn",
        "namespace app\nfunction test-invalid int; supplied int\n  return supplied\n",
    );

    let failure = compile_test_package(
        &TestPackage::load(&package.0).unwrap(),
        CompilerOptions::default(),
    )
    .unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2051");
    assert!(failure.source.path().ends_with("tests/unit/case.trn"));
}

#[test]
fn test_manifest_roots_are_bounded_and_profiled() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        concat!(
            "package = \"configured-native-tests\"\n",
            "prelude = false\n",
            "[namespaces]\napp = \"src\"\n",
            "[testing]\nunit = \"spec/unit\"\n",
            "[testing.profile]\nname = \"test\"\ncapabilities = []\npanic = \"abort\"\n",
        ),
    );
    package.write("src/library.trn", "namespace app\nconstant value = 1\n");
    package.write(
        "spec/unit/case.trn",
        "namespace app\nfunction test-configured;\n",
    );

    let loaded = TestPackage::load(&package.0).unwrap();
    assert_eq!(loaded.configuration.profile.name, "test");
    assert!(
        loaded
            .configuration
            .profile
            .capabilities
            .as_ref()
            .unwrap()
            .is_empty()
    );
    assert_eq!(loaded.configuration.profile.panic, PanicProfile::Abort);
    assert_eq!(
        loaded.tier_packages.keys().copied().collect::<Vec<_>>(),
        [TestTier::Unit]
    );
}

#[test]
fn integration_tests_cannot_import_private_package_bindings() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"private-integration-tests\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    );
    package.write(
        "src/library.trn",
        "namespace app\nprivate constant secret int = 42\n",
    );
    package.write(
        "tests/integration/private.trn",
        "namespace app-tests\nfrom /app import secret\nfunction test-private;\n    secret\n",
    );

    let failure = compile_test_package(
        &TestPackage::load(&package.0).unwrap(),
        CompilerOptions::default(),
    )
    .unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2010");
    assert!(
        failure
            .source
            .path()
            .ends_with("tests/integration/private.trn")
    );
}

#[test]
fn duplicate_test_identities_use_a_test_specific_diagnostic() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"duplicate-native-tests\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    );
    package.write("src/library.trn", "namespace app\nconstant value int = 1\n");
    package.write(
        "tests/unit/first.trn",
        "namespace app\nfunction test-same;\n",
    );
    package.write(
        "tests/unit/second.trn",
        "namespace app\nfunction test-same;\n",
    );

    let failure = compile_test_package(
        &TestPackage::load(&package.0).unwrap(),
        CompilerOptions::default(),
    )
    .unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2052");
    assert!(
        failure.diagnostics[0]
            .message
            .contains("duplicate test identity")
    );
}

#[test]
fn production_sources_cannot_import_test_only_namespaces() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"test-boundary\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    );
    package.write(
        "src/main.trn",
        "namespace app\nfrom /test-helpers import helper\nfunction main;\n  helper\n",
    );
    package.write(
        "tests/unit/helper.trn",
        "namespace test-helpers\nconstant helper = 1\nfunction test-helper;\n",
    );

    let failure = compile_test_package(
        &TestPackage::load(&package.0).unwrap(),
        CompilerOptions::default(),
    )
    .unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2055");
    assert!(
        failure.diagnostics[0]
            .message
            .contains("production source cannot import test-only namespace")
    );
}

#[test]
fn integration_tests_cannot_join_production_namespaces() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"integration-boundary\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    );
    package.write(
        "src/library.trn",
        "namespace app\npublic constant value = 1\n",
    );
    package.write(
        "tests/integration/private.trn",
        "namespace app\nfunction test-private;\n  value\n",
    );

    let failure = compile_test_package(
        &TestPackage::load(&package.0).unwrap(),
        CompilerOptions::default(),
    )
    .unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2054");
}

#[test]
fn testing_process_fixtures_require_process_capability() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"testing-capabilities\"\nprelude = false\n[profile]\nname = \"confined\"\ncapabilities = []\n[namespaces]\napp = \"src\"\n",
    );
    package.write(
        "src/library.trn",
        "namespace app\npublic constant value = 1\n",
    );
    package.write(
        "tests/unit/case.trn",
        "namespace app\nfrom /core/testing/process import process-fixture\nfunction test-process;\n  fixture = instance process-fixture; ''\n",
    );

    let failure = compile_test_package(
        &TestPackage::load(&package.0).unwrap(),
        CompilerOptions::default(),
    )
    .unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2032");
    assert!(failure.diagnostics[0].message.contains("process"));
}

#[test]
fn production_compilation_rejects_test_only_namespaces() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"production-testing-import\"\nprelude = false\n[namespaces]\napp = \"src\"\n",
    );
    package.write(
        "src/main.trn",
        "namespace app\nfrom /core/testing import assert\nfunction main;\n  assert; true\n",
    );

    let failure = compile_package(&Package::load(&package.0).unwrap()).unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2053");
}

#[test]
fn authored_and_generated_sources_have_distinct_ids() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"source-identities\"\n[namespaces]\napp = \"src\"\n[rust-modules]\nadapters = \"rust/adapters.rs\"\n",
    );
    package.write(
        "src/main.trn",
        "namespace app\nfunction main;\n  answer int = rust\n    crate::adapters::answer()\n  print; answer\n",
    );
    package.write(
        "rust/adapters.rs",
        "pub fn answer() -> terrane_int_support::Int {\n    terrane_int_support::Int::from(42)\n}\n",
    );

    let compilation = compile_package(&Package::load(&package.0).unwrap()).unwrap();
    let source_ids = compilation
        .sources
        .iter()
        .map(terrane_compiler::SourceFile::id)
        .collect::<std::collections::BTreeSet<_>>();

    assert_eq!(source_ids.len(), compilation.sources.len());
}

#[test]
fn local_terrane_library_lowers_with_the_application() {
    let workspace = TempPackage::new();
    workspace.write(
        "library/package.toml",
        "package = \"acme/lib\"\nartifact = \"library\"\nprelude = false\n[namespaces]\n\"acme/lib\" = \"src\"\n",
    );
    workspace.write(
        "library/src/library.trn",
        "namespace acme/lib\nconstant answer = 42\n",
    );
    workspace.write(
        "app/package.toml",
        "package = \"example.app\"\nprelude = false\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\npath = \"../library\"\n",
    );
    workspace.write(
        "app/src/main.trn",
        "namespace app\nfrom /acme/lib import answer\nfunction main;\n  answer\n",
    );

    let package = Package::load(workspace.0.join("app")).unwrap();
    assert_eq!(package.units.len(), 2);
    assert_eq!(package.library_source_ids.len(), 1);
    let compilation = compile_package(&package).unwrap();
    assert!(compilation.rust.contains("// Namespace: acme/lib"));
}

#[test]
fn library_packages_allow_no_main_and_reject_main() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"acme/lib\"\nartifact = \"library\"\nprelude = false\n[namespaces]\n\"acme/lib\" = \"src\"\n",
    );
    package.write(
        "src/library.trn",
        "namespace acme/lib\nconstant answer = 42\n",
    );
    compile_package(&Package::load(&package.0).unwrap()).unwrap();

    package.write(
        "src/library.trn",
        "namespace acme/lib\nfunction main;\n  none\n",
    );
    let failure = compile_package(&Package::load(&package.0).unwrap()).unwrap_err();
    assert_eq!(failure.diagnostics[0].code, "S2017");
}

#[test]
fn library_rust_dependencies_and_modules_merge_into_the_application() {
    let workspace = TempPackage::new();
    workspace.write(
        "library/package.toml",
        "package = \"acme/lib\"\nartifact = \"library\"\n[namespaces]\n\"acme/lib\" = \"src\"\n[rust-dependencies.codec]\npackage = \"base64\"\nversion = \"=0.22.1\"\n[rust-modules]\ncodec_adapter = \"rust/codec.rs\"\n",
    );
    workspace.write(
        "library/src/library.trn",
        "namespace acme/lib\nconstant answer = 42\n",
    );
    workspace.write("library/rust/codec.rs", "pub fn marker() {}\n");
    workspace.write(
        "app/package.toml",
        "package = \"example.app\"\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\npath = \"../library\"\n",
    );
    workspace.write(
        "app/src/main.trn",
        "namespace app\nfunction main;\n  none\n",
    );

    let package = Package::load(workspace.0.join("app")).unwrap();
    assert_eq!(package.rust_dependencies[0].name, "codec");
    assert_eq!(package.authored_rust_modules[0].name, "codec_adapter");
    assert!(
        package.authored_rust_modules[0]
            .relative_path
            .starts_with("dependencies/acme/lib")
    );
}

#[test]
fn local_library_hash_is_verified() {
    let workspace = TempPackage::new();
    workspace.write(
        "library/package.toml",
        "package = \"acme/lib\"\nartifact = \"library\"\n[namespaces]\n\"acme/lib\" = \"src\"\n",
    );
    workspace.write(
        "library/src/library.trn",
        "namespace acme/lib\nconstant answer = 42\n",
    );
    workspace.write(
        "app/package.toml",
        "package = \"example.app\"\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\npath = \"../library\"\nhash = \"sha256:0000000000000000000000000000000000000000000000000000000000000000\"\n",
    );
    workspace.write(
        "app/src/main.trn",
        "namespace app\nfunction main;\n  none\n",
    );

    let errors = Package::load(workspace.0.join("app")).unwrap_err();
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("source hash mismatch")
    );
}

#[test]
fn tagged_git_library_is_cached_and_loaded_by_hash() {
    let workspace = TempPackage::new();
    workspace.write(
        "repository/package.toml",
        "package = \"acme/lib\"\nartifact = \"library\"\n[namespaces]\n\"acme/lib\" = \"src\"\n",
    );
    workspace.write(
        "repository/src/library.trn",
        "namespace acme/lib\nconstant answer = 42\n",
    );
    let repository = workspace.0.join("repository");
    for args in [
        vec!["init"],
        vec!["config", "user.email", "terrane@example.invalid"],
        vec!["config", "user.name", "Terrane Test"],
        vec!["add", "."],
        vec!["commit", "-m", "library"],
        vec!["tag", "v1.0.0"],
    ] {
        let status = std::process::Command::new("git")
            .args(args)
            .current_dir(&repository)
            .status()
            .unwrap();
        assert!(status.success());
    }
    let hash = source_tree_hash(&repository).unwrap();
    workspace.write(
        "app/package.toml",
        &format!(
            "package = \"example.app\"\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\ngit = {:?}\ntag = \"v1.0.0\"\nhash = \"{hash}\"\n",
            repository.to_string_lossy()
        ),
    );
    workspace.write(
        "app/src/main.trn",
        "namespace app\nfunction main;\n  none\n",
    );
    let bogus = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    workspace.write(
        "app/package.toml",
        &format!(
            "package = \"example.app\"\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\ngit = {:?}\ntag = \"v1.0.0\"\nhash = \"{bogus}\"\n",
            repository.to_string_lossy()
        ),
    );
    assert!(Package::load(workspace.0.join("app")).is_err());
    assert!(
        !workspace
            .0
            .join("app/.trn/packages")
            .join(bogus.trim_start_matches("sha256:"))
            .exists()
    );
    workspace.write(
        "app/package.toml",
        &format!(
            "package = \"example.app\"\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\ngit = {:?}\ntag = \"v1.0.0\"\nhash = \"{hash}\"\n",
            repository.to_string_lossy()
        ),
    );

    let package = Package::load(workspace.0.join("app")).unwrap();
    assert_eq!(package.units.len(), 2);
    assert!(workspace.0.join("app/.trn/packages").is_dir());
}

#[test]
fn composed_library_keeps_its_own_prelude_setting() {
    let workspace = TempPackage::new();
    workspace.write(
        "library/package.toml",
        "package = \"acme/lib\"\nartifact = \"library\"\nprelude = true\n[namespaces]\n\"acme/lib\" = \"src\"\n",
    );
    workspace.write(
        "library/src/library.trn",
        "namespace acme/lib\nfunction announce;\n  print; >hello\n",
    );
    workspace.write(
        "app/package.toml",
        "package = \"example.app\"\nprelude = false\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\npath = \"../library\"\n",
    );
    workspace.write(
        "app/src/main.trn",
        "namespace app\nfunction main;\n  none\n",
    );

    let package = Package::load(workspace.0.join("app")).unwrap();
    assert!(
        package
            .units
            .iter()
            .any(|unit| { package.library_source_ids.contains(&unit.source.id()) && unit.prelude })
    );
    compile_package(&package).unwrap();
}

#[test]
fn application_and_library_preludes_are_independent() {
    for (application_prelude, library_prelude) in
        [(false, false), (false, true), (true, false), (true, true)]
    {
        let workspace = TempPackage::new();
        workspace.write(
            "library/package.toml",
            &format!(
                "package = \"acme/lib\"\nartifact = \"library\"\nprelude = {library_prelude}\n[namespaces]\n\"acme/lib\" = \"src\"\n"
            ),
        );
        workspace.write(
            "library/src/library.trn",
            "namespace acme/lib\nfunction announce;\n  print; >library\n",
        );
        workspace.write(
            "app/package.toml",
            &format!(
                "package = \"example.app\"\nprelude = {application_prelude}\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\npath = \"../library\"\n"
            ),
        );
        workspace.write(
            "app/src/main.trn",
            "namespace app\nfunction main;\n  print; >application\n",
        );

        let package = Package::load(workspace.0.join("app")).unwrap();
        assert_eq!(
            compile_package(&package).is_ok(),
            application_prelude && library_prelude,
            "application prelude {application_prelude}, library prelude {library_prelude}"
        );
    }
}

#[test]
fn library_warnings_do_not_leak_into_consumers_or_flag_exports() {
    let workspace = TempPackage::new();
    workspace.write(
        "library/package.toml",
        "package = \"acme/lib\"\nartifact = \"library\"\nprelude = false\n[namespaces]\n\"acme/lib\" = \"src\"\n",
    );
    workspace.write(
        "library/src/library.trn",
        "namespace acme/lib\nfunction first;\n  none\nfunction second;\n  none\n",
    );
    let library = Package::load(workspace.0.join("library")).unwrap();
    let standalone = compile_package(&library).unwrap();
    assert!(
        standalone
            .warnings
            .iter()
            .all(|warning| !matches!(warning.code, "W4001" | "W4005"))
    );

    workspace.write(
        "app/package.toml",
        "package = \"example.app\"\nprelude = false\n[namespaces]\napp = \"src\"\n[terrane-dependencies.library]\npath = \"../library\"\n",
    );
    workspace.write(
        "app/src/main.trn",
        "namespace app\nfunction main;\n  none\n",
    );
    let consumer = Package::load(workspace.0.join("app")).unwrap();
    let compilation = compile_package(&consumer).unwrap();
    assert!(compilation.warnings.iter().all(|warning| {
        warning
            .primary
            .is_none_or(|span| !consumer.library_source_ids.contains(&span.file))
    }));
}

#[test]
fn duplicate_library_identities_are_manifest_errors() {
    let workspace = TempPackage::new();
    for directory in ["first", "second"] {
        workspace.write(
            &format!("{directory}/package.toml"),
            "package = \"acme/lib\"\nartifact = \"library\"\nprelude = false\n[namespaces]\n\"acme/lib\" = \"src\"\n",
        );
        workspace.write(
            &format!("{directory}/src/library.trn"),
            "namespace acme/lib\nconstant answer = 42\n",
        );
    }
    workspace.write(
        "app/package.toml",
        "package = \"example.app\"\n[namespaces]\napp = \"src\"\n[terrane-dependencies.first]\npath = \"../first\"\n[terrane-dependencies.second]\npath = \"../second\"\n",
    );
    workspace.write(
        "app/src/main.trn",
        "namespace app\nfunction main;\n  none\n",
    );

    let errors = Package::load(workspace.0.join("app")).unwrap_err();
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("library identity `acme/lib` is already loaded")
    );
}

#[test]
fn dependency_errors_highlight_the_dependency_header() {
    let workspace = TempPackage::new();
    workspace.write(
        "package.toml",
        "package = \"acme/app\"\n[namespaces]\napp = \"src\"\n[terrane-dependencies.me]\npath = \"missing\"\n",
    );
    workspace.write("src/main.trn", "namespace app\nfunction main;\n  none\n");

    let error = Package::load(&workspace.0).unwrap_err().remove(0);
    let span = error.diagnostic.primary.unwrap();
    assert_eq!(
        &error.source.text()[span.start..span.end],
        "[terrane-dependencies.me]"
    );
}

#[test]
fn source_tree_hash_includes_non_package_files() {
    let package = TempPackage::new();
    package.write(
        "package.toml",
        "package = \"hash-test\"\n[namespaces]\napp = \"src\"\n",
    );
    package.write("src/main.trn", "namespace app\nfunction main;\n  none\n");
    let before = source_tree_hash(&package.0).unwrap();
    package.write("README.md", "local notes\n");
    let after = source_tree_hash(&package.0).unwrap();
    assert_ne!(before, after);
}

#[test]
fn dependency_cycles_report_the_complete_identity_chain() {
    let workspace = TempPackage::new();
    workspace.write(
        "app/package.toml",
        "package = \"example/app\"\n[namespaces]\napp = \"src\"\n[terrane-dependencies.b]\npath = \"../b\"\n",
    );
    workspace.write(
        "app/src/main.trn",
        "namespace app\nfunction main;\n  none\n",
    );
    workspace.write(
        "b/package.toml",
        "package = \"example/b\"\nartifact = \"library\"\n[namespaces]\n\"example/b\" = \"src\"\n[terrane-dependencies.c]\npath = \"../c\"\n",
    );
    workspace.write("b/src/library.trn", "namespace example/b\nconstant b = 1\n");
    workspace.write(
        "c/package.toml",
        "package = \"example/c\"\nartifact = \"library\"\n[namespaces]\n\"example/c\" = \"src\"\n[terrane-dependencies.app]\npath = \"../app\"\n",
    );
    workspace.write("c/src/library.trn", "namespace example/c\nconstant c = 1\n");

    let errors = Package::load(workspace.0.join("app")).unwrap_err();
    assert!(
        errors[0]
            .diagnostic
            .message
            .contains("example/app -> example/b -> example/c -> example/app")
    );
}
