mod bundled;
mod compiler;
pub mod diagnostic;
pub mod highlight;
pub mod lexer;
mod lowering;
pub mod package;
pub mod parser;
pub mod rust_ir;
pub mod semantics;
pub mod source;
pub mod syntax;
pub mod tokens;
pub mod types;

pub use compiler::{
    Compilation, CompilationFailure, CompilerOptions, compile, compile_package,
    compile_package_with_options, compile_with_options,
};
pub use diagnostic::{Diagnostic, Severity};
pub use package::{
    ExecutorProfile, IMPLICIT_PACKAGE_ID, MANIFEST_FILE_NAME, Package, PackageLoadError,
    ReflectionProfile, RustDependency, SourceUnit,
};
pub use semantics::{
    BOOTSTRAP_VERSION, BoundMethod, EvaluationKind, EvaluationStep, FunctionContract, MemberFamily,
    Namespace, ParameterContract, SemanticFailure, SemanticPackage, SemanticUnit, Symbol,
    TypedBinding, ValueType, Visibility, analyze,
};
pub use source::{SourceFile, Span};
pub use types::{DescriptorSchema, ScalarType, TypeCategory};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

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
        "[package]\nname = \"terrane-platform-support\"\nversion = \"{VERSION}\"\nedition = \"2024\"\n\n[dependencies]\n{dependencies}\n"
    )
}

#[cfg(test)]
mod manifest_tests {
    use super::platform_support_manifest_from;

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
}
