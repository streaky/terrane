use super::{
    CaseTiming, ConformanceBuild, assert_expected_warnings, binary_path, boolean_field,
    build_generated_binaries, conformance_jobs, corpus, field, manifests_below,
    parallel_map_indexed, persist_lowering_artifacts, run_case_with_selector,
    stage_generated_binary, usize_field,
};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

fn validate_inventory<'a>(
    suite: &str,
    tests: impl IntoIterator<Item = &'a str>,
    manifests: &[PathBuf],
    requires_unsafe_code: bool,
) -> bool {
    let mut registrations = BTreeMap::<_, Option<&Path>>::new();
    for identity in tests {
        assert!(
            registrations.insert(identity, None).is_none(),
            "suite `{suite}` has duplicate compiled test `{identity}`"
        );
    }
    let mut safety = None;
    for path in manifests {
        let manifest = fs::read_to_string(path).unwrap();
        let identity = field(&manifest, "test")
            .unwrap_or_else(|| panic!("suite case lacks `test`: {}", path.display()));
        let slot = registrations.get_mut(identity).unwrap_or_else(|| {
            panic!(
                "suite `{suite}` has no test `{identity}` selected by {}",
                path.display()
            )
        });
        if let Some(previous) = slot {
            panic!(
                "suite `{suite}` test `{identity}` is selected by both {} and {}",
                previous.display(),
                path.display()
            );
        }
        *slot = Some(path);
        let unsafe_code = boolean_field(&manifest, "unsafe").unwrap_or_else(|| {
            assert!(
                !manifest.lines().any(|line| line.starts_with("unsafe = ")),
                "suite case `unsafe` must be true or false: {}",
                path.display()
            );
            false
        });
        if let Some(expected) = safety {
            assert_eq!(
                expected,
                unsafe_code,
                "suite `{suite}` mixes safe and unsafe scenarios at {}; unsafe scenarios require a separate suite with `unsafe = true` on every manifest",
                path.display()
            );
        } else {
            safety = Some(unsafe_code);
        }
    }
    if let Some((identity, _)) = registrations
        .iter()
        .find(|(_, registered)| registered.is_none())
    {
        panic!("suite `{suite}` scenario `{identity}` has no corpus manifest");
    }
    let unsafe_code = safety.unwrap_or(false);
    assert_eq!(
        requires_unsafe_code, unsafe_code,
        "suite `{suite}` unsafe category does not match its generated code: safe suites must not require unsafe code; explicitly unsafe suites require `unsafe = true` on every manifest"
    );
    unsafe_code
}

fn assert_registered_suites(root: &Path, registered: &BTreeMap<String, Vec<PathBuf>>) {
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() && path.join("package.toml").is_file() {
            let suite = path.file_name().unwrap().to_str().unwrap();
            assert!(
                registered.contains_key(suite),
                "shared suite `{suite}` has no registered corpus scenarios"
            );
        }
    }
}

fn shared_suite_groups(manifests: &[PathBuf]) -> BTreeMap<String, Vec<PathBuf>> {
    let mut suites = BTreeMap::<String, Vec<PathBuf>>::new();
    for manifest_path in manifests {
        let manifest = fs::read_to_string(manifest_path).unwrap();
        if let Some(suite) = field(&manifest, "suite") {
            suites
                .entry(suite.to_owned())
                .or_default()
                .push(manifest_path.clone());
        }
    }
    suites
}

fn prepare_shared_case(manifest_path: &Path, build_root: &Path) -> CaseTiming {
    let manifest = fs::read_to_string(manifest_path).unwrap();
    assert_eq!(
        field(&manifest, "phase"),
        Some("run"),
        "shared suites only support run cases: {}",
        manifest_path.display()
    );
    assert_eq!(
        field(&manifest, "status"),
        Some("accept"),
        "shared suites only support accepted cases: {}",
        manifest_path.display()
    );
    for option in [
        "entrypoint",
        "canonical-rust",
        "lint-unused-functions",
        "native-test",
        "warnings",
        "list-preallocation-limit-bytes",
        "projection-history",
        "native-tools",
        "dependency-panic-test",
    ] {
        assert!(
            field(&manifest, option).is_none()
                && boolean_field(&manifest, option).is_none()
                && usize_field(&manifest, option).is_none(),
            "shared suite case option `{option}` is unsupported: {}",
            manifest_path.display()
        );
    }
    let mut timing = CaseTiming::new(manifest_path.parent().unwrap(), build_root);
    timing.defer();
    timing
}

fn compile_shared_suite(
    suite: &str,
    suite_root: &Path,
    selected: &[PathBuf],
    registered: &[PathBuf],
) -> terrane_compiler::testing::TestTierCompilation {
    let package =
        terrane_compiler::testing::TestPackage::load(suite_root).unwrap_or_else(|errors| {
            panic!(
                "cannot load shared suite {}: {errors:?}",
                suite_root.display()
            )
        });
    let options = terrane_compiler::CompilerOptions {
        require_canonical_rust: true,
        lint_name_style: false,
        lint_unused_functions: false,
        debug_build: terrane_compiler::DebugBuild::Disabled,
    };
    let mut tiers = terrane_compiler::compile_test_package(&package, options)
        .unwrap_or_else(|failure| {
            let cases = selected.iter().map(|path| path.display().to_string())
                .collect::<Vec<_>>().join("\n");
            // Shared compilation failed; deferred cases have not executed.
            panic!("shared suite `{suite}` compilation failed: {failure:?}\nselected cases not executed:\n{cases}");
        });
    assert_eq!(
        tiers.len(),
        1,
        "shared suite `{suite}` must have one test tier"
    );
    let tier = tiers.remove(0);
    validate_inventory(
        suite,
        tier.cases.iter().map(|case| case.identity.as_str()),
        registered,
        tier.compilation.requires_unsafe_code,
    );
    assert_expected_warnings(suite_root, "", &tier.compilation);
    tier
}

pub(super) fn run(manifests: &[PathBuf], build: &ConformanceBuild) {
    let registered = shared_suite_groups(&manifests_below(&corpus()));
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/conformance-suites");
    assert_registered_suites(&root, &registered);
    let suites = shared_suite_groups(manifests)
        .into_iter()
        .collect::<Vec<_>>();
    let prepared = parallel_map_indexed(
        &suites,
        conformance_jobs(suites.len()),
        |_, (suite, case_manifests)| {
            let suite_root = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../tests/fixtures/conformance-suites")
                .join(suite);
            let case_timings = case_manifests
                .iter()
                .map(|path| prepare_shared_case(path, &build.root))
                .collect::<Vec<_>>();
            let suite_timing = CaseTiming::named(&format!("shared-suite/{suite}/compile"));
            let tier = compile_shared_suite(suite, &suite_root, case_manifests, &registered[suite]);
            (suite_root, case_timings, suite_timing, tier)
        },
    );
    for ((suite, case_manifests), (suite_root, mut case_timings, mut suite_timing, tier)) in
        suites.into_iter().zip(prepared)
    {
        let mut selected = Vec::new();
        for manifest_path in &case_manifests {
            let manifest = fs::read_to_string(manifest_path).unwrap();
            let identity = field(&manifest, "test")
                .unwrap_or_else(|| panic!("suite case lacks `test`: {}", manifest_path.display()));
            let test = tier
                .cases
                .iter()
                .find(|test| test.identity == identity)
                .unwrap_or_else(|| {
                    panic!(
                        "suite `{suite}` has no test `{identity}` selected by {}",
                        manifest_path.display()
                    )
                });
            selected.push((
                manifest_path.clone(),
                test.selector,
                test.identity.clone(),
                test.tier.name().to_owned(),
            ));
        }
        let rust_files = tier
            .compilation
            .rust_files_for(Path::new("lower.rs"))
            .unwrap();
        persist_lowering_artifacts(&suite_root, &rust_files);
        let binary_name = format!("terrane_conformance_suite_{suite}");
        stage_generated_binary(
            &binary_name,
            &suite_root,
            "",
            &rust_files,
            tier.compilation.requires_unsafe_code,
            build,
        );
        let output = build_generated_binaries(
            &[&binary_name],
            &tier.compilation.rust_dependencies,
            tier.compilation.requires_unsafe_code,
            build,
        );
        if !output.status.success() {
            suite_timing.fail();
            let stderr = String::from_utf8_lossy(&output.stderr);
            let cases = selected
                .iter()
                .map(|(path, _, identity, _)| format!("{} selects {identity}", path.display()))
                .collect::<Vec<_>>()
                .join("\n");
            panic!(
                "shared suite `{suite}` generated Rust failed to compile; selected cases not executed:\n{cases}\n{stderr}"
            );
        }
        suite_timing.pass();
        for (index, (manifest_path, selector, identity, tier_name)) in
            selected.into_iter().enumerate()
        {
            let case = manifest_path.parent().unwrap();
            let manifest = fs::read_to_string(&manifest_path).unwrap();
            case_timings[index].begin_pending_work();
            run_case_with_selector(
                &binary_path(&binary_name, build),
                &build.root,
                case,
                &manifest,
                Some(selector),
                Some(&identity),
                Some(&tier_name),
            );
            case_timings[index].pass();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

    struct Fixture {
        root: PathBuf,
        manifests: Vec<PathBuf>,
    }

    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "terrane-suite-inventory-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&root).unwrap();
            Self {
                root,
                manifests: Vec::new(),
            }
        }

        fn register(&mut self, name: &str, identity: &str, unsafe_code: bool) {
            let path = self.root.join(format!("{name}.toml"));
            fs::write(&path, format!(
                "phase = \"run\"\nstatus = \"accept\"\nsuite = \"sample\"\ntest = \"{identity}\"\nunsafe = {unsafe_code}\n"
            )).unwrap();
            self.manifests.push(path);
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.root).unwrap();
        }
    }

    fn rejects(action: impl FnOnce() -> bool + std::panic::UnwindSafe, expected: &str) {
        let error = std::panic::catch_unwind(action).expect_err("invalid suite must fail");
        let message = error
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| error.downcast_ref::<&str>().copied())
            .unwrap();
        assert!(message.contains(expected), "{message}");
    }

    #[test]
    fn every_scenario_requires_one_manifest() {
        let mut fixture = Fixture::new();
        fixture.register("first", "/sample::first", false);
        rejects(
            || {
                validate_inventory(
                    "sample",
                    ["/sample::first", "/sample::second"],
                    &fixture.manifests,
                    false,
                )
            },
            "scenario `/sample::second` has no corpus manifest",
        );
        fixture.register("second", "/sample::second", false);
        assert!(!validate_inventory(
            "sample",
            ["/sample::first", "/sample::second"],
            &fixture.manifests,
            false
        ));
        fixture.register("duplicate", "/sample::first", false);
        rejects(
            || {
                validate_inventory(
                    "sample",
                    ["/sample::first", "/sample::second"],
                    &fixture.manifests,
                    false,
                )
            },
            "is selected by both",
        );
    }

    #[test]
    fn manifests_cannot_select_missing_scenarios() {
        let mut fixture = Fixture::new();
        fixture.register("missing", "/sample::missing", false);
        rejects(
            || validate_inventory("sample", ["/sample::first"], &fixture.manifests, false),
            "has no test `/sample::missing`",
        );
    }

    #[test]
    fn unsafe_requires_separate_explicit_category() {
        let mut safe = Fixture::new();
        safe.register("safe", "/sample::safe", false);
        rejects(
            || validate_inventory("sample", ["/sample::safe"], &safe.manifests, true),
            "safe suites must not require unsafe code",
        );
        let mut unsafe_suite = Fixture::new();
        unsafe_suite.register("unsafe", "/sample::unsafe", true);
        assert!(validate_inventory(
            "sample",
            ["/sample::unsafe"],
            &unsafe_suite.manifests,
            true
        ));
        unsafe_suite.register("safe", "/sample::safe", false);
        rejects(
            || {
                validate_inventory(
                    "sample",
                    ["/sample::unsafe", "/sample::safe"],
                    &unsafe_suite.manifests,
                    true,
                )
            },
            "mixes safe and unsafe scenarios",
        );
    }
}
