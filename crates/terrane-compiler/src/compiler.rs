use std::path::PathBuf;

use crate::{
    Diagnostic, Package, RustDependency, SourceFile, Span,
    rust_ir::RenderedFile,
    semantics::{self, SymbolKind},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompilerOptions {
    pub require_canonical_rust: bool,
    pub lint_name_style: bool,
}

#[derive(Clone, Debug)]
pub struct Compilation {
    pub source: SourceFile,
    pub sources: Vec<SourceFile>,
    pub rust: String,
    pub rust_files: Vec<RenderedFile>,
    pub warnings: Vec<Diagnostic>,
    pub rust_dependencies: Vec<RustDependency>,
}

#[derive(Clone, Debug)]
pub struct CompilationFailure {
    pub source: SourceFile,
    pub diagnostics: Vec<Diagnostic>,
}

impl std::ops::Deref for CompilationFailure {
    type Target = [Diagnostic];

    fn deref(&self) -> &Self::Target {
        &self.diagnostics
    }
}

/// Compiles one Terrane source file as an implicit, stable-identity package.
///
/// # Errors
///
/// Returns every source-oriented diagnostic produced by the shared frontend.
pub fn compile(path: impl Into<PathBuf>, text: String) -> Result<Compilation, CompilationFailure> {
    compile_with_options(path, text, CompilerOptions::default())
}

/// Compiles one Terrane source file with explicit compiler-development options.
///
/// # Errors
///
/// Returns every source-oriented diagnostic produced by the shared frontend,
/// including generated-Rust invariant failures requested by `options`.
pub fn compile_with_options(
    path: impl Into<PathBuf>,
    text: String,
    options: CompilerOptions,
) -> Result<Compilation, CompilationFailure> {
    compile_package_with_options(&Package::implicit(path, text), options)
}

/// Compiles every manifest-discovered source unit through the shared frontend.
///
/// # Errors
///
/// Returns diagnostics from the first source unit that fails. All units are
/// parsed before semantic projection, in deterministic package order.
pub fn compile_package(package: &Package) -> Result<Compilation, CompilationFailure> {
    compile_package_with_options(package, CompilerOptions::default())
}

/// Compiles every manifest-discovered source unit with explicit
/// compiler-development options.
///
/// # Errors
///
/// Returns diagnostics from the first source unit that fails, including
/// generated-Rust invariant failures requested by `options`.
pub fn compile_package_with_options(
    package: &Package,
    options: CompilerOptions,
) -> Result<Compilation, CompilationFailure> {
    let semantic = semantics::analyze(package).map_err(|failure| CompilationFailure {
        source: failure.source,
        diagnostics: failure.diagnostics,
    })?;
    let entry_points = semantic
        .namespaces
        .values()
        .filter_map(|namespace| namespace.symbols.get("main"))
        .filter(|symbol| symbol.kind == SymbolKind::Function)
        .collect::<Vec<_>>();
    let entry = match entry_points.as_slice() {
        [] => {
            let source = &semantic.units[0].source;
            return Err(CompilationFailure {
                source: source.clone(),
                diagnostics: vec![Diagnostic::error(
                    "S2015",
                    "package has no `main` function",
                    Span::new(source.id(), 0, 0),
                )],
            });
        }
        [entry] => *entry,
        [_, ambiguous, ..] => {
            let span = ambiguous
                .declaration_span
                .unwrap_or_else(|| Span::new(semantic.units[0].source.id(), 0, 0));
            let source = semantic
                .units
                .iter()
                .find(|unit| unit.source.id() == span.file)
                .map_or(&semantic.units[0].source, |unit| &unit.source);
            return Err(CompilationFailure {
                source: source.clone(),
                diagnostics: vec![Diagnostic::error(
                    "S2016",
                    "package has more than one `main` function",
                    span,
                )],
            });
        }
    };
    let entry_span = entry
        .declaration_span
        .unwrap_or_else(|| Span::new(semantic.units[0].source.id(), 0, 0));
    let unit = semantic
        .units
        .iter()
        .find(|unit| unit.source.id() == entry_span.file)
        .unwrap_or(&semantic.units[0]);
    let source = &unit.source;
    let sources = semantic
        .units
        .iter()
        .map(|unit| unit.source.clone())
        .collect();
    let warnings = semantics::warnings(&semantic, options.lint_name_style);
    let rust_ir = crate::lowering::lower(&semantic);
    let rust = rust_ir.render();
    let rust_files = rust_ir.render_files();
    let rust_dependencies = package
        .rust_dependencies
        .iter()
        .map(|dependency| {
            let mut dependency = dependency.clone();
            if let Some(projected) = semantic
                .projection
                .dependencies
                .iter()
                .find(|projected| projected.name == dependency.name)
            {
                dependency.version = format!("={}", projected.version);
            }
            dependency
        })
        .collect();
    if options.require_canonical_rust {
        validate_canonical_rust(&rust_files, &semantic.units, source, entry_span)?;
    }
    Ok(Compilation {
        source: (*source).clone(),
        sources,
        rust,
        rust_files,
        warnings,
        rust_dependencies,
    })
}

fn validate_canonical_rust(
    files: &[RenderedFile],
    units: &[crate::SemanticUnit],
    fallback_source: &SourceFile,
    fallback_span: Span,
) -> Result<(), CompilationFailure> {
    for file in files {
        let canonical = canonical_rust(&file.contents).map_err(|error| CompilationFailure {
            source: fallback_source.clone(),
            diagnostics: vec![
                Diagnostic::error(
                    "S9004",
                    format!(
                        "generated Rust `{}` cannot be checked for canonical formatting: {error}",
                        file.path
                    ),
                    fallback_span,
                )
                .with_help("generated Rust must parse before its formatting can be validated"),
            ],
        })?;
        if canonical != file.contents {
            let difference = first_difference(&file.contents, &canonical);
            let span = file
                .associations
                .iter()
                .find(|association| {
                    association.generated_start <= difference
                        && difference < association.generated_end
                })
                .map_or(fallback_span, |association| association.source);
            let source = units
                .iter()
                .find(|unit| unit.source.id() == span.file)
                .map_or(fallback_source, |unit| &unit.source);
            return Err(CompilationFailure {
                source: source.clone(),
                diagnostics: vec![
                    Diagnostic::error(
                        "S9004",
                        format!("generated Rust `{}` is not canonical", file.path),
                        span,
                    )
                    .with_help(format!(
                        "lowering first differs from the bundled formatter at generated byte {difference}"
                    )),
                ],
            });
        }
    }
    Ok(())
}

fn canonical_rust(rust: &str) -> Result<String, syn::Error> {
    let metadata_end = rust
        .split_inclusive('\n')
        .take_while(|line| line.starts_with("//"))
        .map(str::len)
        .sum();
    let (metadata, body) = rust.split_at(metadata_end);
    Ok(format!(
        "{metadata}{}",
        crate::rust_ir::canonicalize_rust(body)?
    ))
}

fn first_difference(left: &str, right: &str) -> usize {
    left.bytes()
        .zip(right.bytes())
        .position(|(left, right)| left != right)
        .unwrap_or_else(|| left.len().min(right.len()))
}

#[cfg(test)]
mod tests {
    use super::{canonical_rust, first_difference};

    #[test]
    fn canonical_formatter_preserves_generated_metadata() {
        let rust = "// Generated deterministically.\nfn main() {\n    println!(\"hello\");\n}\n";
        assert_eq!(canonical_rust(rust).unwrap(), rust);
    }

    #[test]
    fn first_difference_handles_changed_and_appended_text() {
        assert_eq!(first_difference("abc", "axc"), 1);
        assert_eq!(first_difference("abc", "abcd"), 3);
    }
}
