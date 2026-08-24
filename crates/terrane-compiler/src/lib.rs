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
    ReflectionProfile, SourceUnit,
};
pub use semantics::{
    BOOTSTRAP_VERSION, BoundMethod, EvaluationKind, EvaluationStep, FunctionContract, MemberFamily,
    Namespace, ParameterContract, SemanticFailure, SemanticPackage, SemanticUnit, Symbol,
    TypedBinding, ValueType, Visibility, analyze,
};
pub use source::{SourceFile, Span};
pub use types::{DescriptorSchema, ScalarType, TypeCategory};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
