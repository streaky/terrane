mod bundled;
pub mod cargo_toolchain;
mod compiler;
pub mod debugging;
pub mod diagnostic;
mod execution;
pub mod highlight;
mod invocation;
pub mod lexer;
mod lowering;
pub mod package;
pub mod parser;
pub mod profiling;
pub mod projection;
pub mod provenance;
pub mod rust_ir;
pub mod semantics;
pub mod source;
pub mod syntax;
pub mod testing;
pub mod tokens;
pub mod tooling;
pub mod types;

mod projection_oracle;
pub use compiler::{
    Compilation, CompilationFailure, CompilerOptions, DebugBuild, RustArtifactError, compile,
    compile_discovered_test_tier, compile_package, compile_package_with_options,
    compile_test_package, compile_test_package_tiers, compile_with_options, discover_test_package,
};
pub use diagnostic::{Diagnostic, Severity};
pub use invocation::InvocationMode;
pub use package::{
    ArtifactKind, AuthoredRustModule, BuildToolchain, CapabilityProfile, ExecutorProfile,
    IMPLICIT_PACKAGE_ID, MANIFEST_FILE_NAME, Package, PackageLoadError, PackagePurpose,
    PanicProfile, ReflectionProfile, RustDependency, SourceRole, SourceUnit, TerraneDependency,
    TerraneDependencySource, git_library_metadata, git_source_tree_hash, source_tree_hash,
    with_tokio_runtime,
};
pub use projection_oracle::{
    BoundQuestion, CallProbeEvidence, CallProbeReport, CallQuestion, ProbeAnswer, ProbeEvidence,
    ProbeReport, ProjectionOracle,
};
pub use semantics::{
    BOOTSTRAP_VERSION, BoundMethod, CallableParameterType, EvaluationKind, EvaluationStep,
    FunctionContract, MemberFamily, Namespace, ParameterContract, SemanticFailure, SemanticPackage,
    SemanticUnit, Symbol, TypedBinding, ValueType, Visibility, analyze,
};
pub use source::{SourceFile, Span};
pub use terrane_rust_analysis::RUSTDOC_TOOLCHAIN;
pub use types::{ScalarType, TypeCategory};
/// Unicode Character Database version selected by the compiler toolchain profile.
pub const UNICODE_DATA_VERSION: &str = "16.0.0";
#[cfg(test)]
const UNICODE_DATA_VERSION_COMPONENTS: (u64, u64, u64) = (16, 0, 0);

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TOOLCHAIN: &str = "1.98.1";

#[must_use]
pub fn platform_support_manifest() -> String {
    platform_support_manifest_from(include_str!("../../terrane-platform-support/Cargo.toml"))
}

fn platform_support_manifest_from(source: &str) -> String {
    let (_, rest) = source
        .split_once("[dependencies]")
        .expect("platform support manifest must declare dependencies");
    let dependencies = rest
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .take_while(|line| !line.trim_start().starts_with('['))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "[package]\nname = \"terrane-platform-support\"\nversion = \"{VERSION}\"\nedition = \"2024\"\nrust-version = \"{BUILD_TOOLCHAIN}\"\n\n[lints.rust]\nunsafe_code = \"forbid\"\n\n[dependencies]\n{dependencies}\n\n[target.'cfg(unix)'.dependencies]\nterrane-signal-support = {{ path = \"../terrane-signal-support\" }}\n"
    )
}

#[must_use]
pub fn signal_support_manifest() -> String {
    format!(
        "[package]\nname = \"terrane-signal-support\"\nversion = \"{VERSION}\"\nedition = \"2024\"\nrust-version = \"{BUILD_TOOLCHAIN}\"\n\n[target.'cfg(unix)'.dependencies]\nlibc = \"0.2\"\n"
    )
}

#[cfg(test)]
mod manifest_tests {
    use super::platform_support_manifest_from;

    #[test]
    fn workspace_and_generated_build_toolchains_match() {
        assert_eq!(super::BUILD_TOOLCHAIN, env!("CARGO_PKG_RUST_VERSION"));
    }
    #[test]
    fn dependency_extraction_stops_at_any_following_section() {
        let with_features = "[dependencies]\nbase64 = \"0.22\"\n\n[features]\ndefault = []\n\n[dev-dependencies]\nrcgen = \"0.14\"\n";
        let without_following_section = "[dependencies]\nbase64 = \"0.22\"\n";

        let with_features = platform_support_manifest_from(with_features);
        let without_following_section = platform_support_manifest_from(without_following_section);

        assert!(with_features.contains("base64 = \"0.22\""));
        assert!(!with_features.contains("[features]"));
        assert!(!with_features.contains("rcgen"));
        assert!(without_following_section.contains("base64 = \"0.22\""));
    }

    #[test]
    fn compiler_and_runtime_unicode_profiles_match() {
        assert_eq!(
            super::UNICODE_DATA_VERSION_COMPONENTS,
            terrane_string_support::UNICODE_DATA_VERSION
        );
        assert_eq!(
            super::UNICODE_DATA_VERSION_COMPONENTS,
            terrane_collection_support::UNICODE_DATA_VERSION
        );
        assert_eq!(
            super::UNICODE_DATA_VERSION,
            format!(
                "{}.{}.{}",
                super::UNICODE_DATA_VERSION_COMPONENTS.0,
                super::UNICODE_DATA_VERSION_COMPONENTS.1,
                super::UNICODE_DATA_VERSION_COMPONENTS.2
            )
        );
    }
}
