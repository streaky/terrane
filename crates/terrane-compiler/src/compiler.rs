use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::{
    Diagnostic, Package, RustDependency, ScalarType, SourceFile, Span,
    rust_ir::RenderedFile,
    semantics::{self, SymbolKind, ValueType},
    testing::{TestCase, TestPackage, TestTier, TestTierDiscovery},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompilerOptions {
    pub require_canonical_rust: bool,
    pub lint_name_style: bool,
    pub debug_information: bool,
}

#[derive(Clone, Debug)]
pub struct Compilation {
    pub source: SourceFile,
    pub sources: Vec<SourceFile>,
    pub rust: String,
    pub review_rust: String,
    rendered_rust: crate::rust_ir::RenderedProgram,
    require_canonical_rust: bool,
    entry_span: Span,
    pub requires_platform_support: bool,
    pub requires_async_runtime: bool,
    pub warnings: Vec<Diagnostic>,
    pub rust_dependencies: Vec<RustDependency>,
    pub dependency_containment: crate::projection::Containment,
    debug_symbols: Option<crate::debugging::DebugSymbols>,
}

impl Compilation {
    /// Render the generated program as an entrypoint and sibling support file.
    ///
    /// The entrypoint contains the authored lowering and one relative `include!`;
    /// compiler-owned runtime and error infrastructure is written to
    /// `<entrypoint-stem>.support.rs`.
    ///
    /// # Errors
    ///
    /// Returns [`RustArtifactError::InvalidOutputPath`] when `entrypoint` cannot derive a UTF-8
    /// support-file path, or [`RustArtifactError::Compilation`] when requested canonical Rust
    /// validation rejects either rendered file.
    pub fn rust_files_for(
        &self,
        entrypoint: &Path,
    ) -> Result<Vec<RenderedFile>, RustArtifactError> {
        let files = self
            .rendered_rust
            .files(entrypoint)
            .map_err(RustArtifactError::InvalidOutputPath)?;
        if self.require_canonical_rust {
            validate_canonical_rust(&files, &self.sources, &self.source, self.entry_span)
                .map_err(RustArtifactError::Compilation)?;
        }
        Ok(files)
    }

    /// Builds final-file debugger metadata for the exact generated Rust paths.
    ///
    /// Returns `None` unless compilation requested [`CompilerOptions::debug_information`].
    ///
    /// # Errors
    ///
    /// Returns [`RustArtifactError::InvalidOutputPath`] when `entrypoint` cannot identify the
    /// generated application and sibling support file.
    pub fn debug_information(
        &self,
        entrypoint: &Path,
    ) -> Result<Option<crate::debugging::DebugInformation>, RustArtifactError> {
        let Some(symbols) = &self.debug_symbols else {
            return Ok(None);
        };
        let files = self
            .rendered_rust
            .files(entrypoint)
            .map_err(RustArtifactError::InvalidOutputPath)?;
        Ok(Some(symbols.render(&files)))
    }
}

#[derive(Clone, Debug)]
pub struct CompilationFailure {
    pub source: SourceFile,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Debug)]
pub enum RustArtifactError {
    InvalidOutputPath(String),
    Compilation(CompilationFailure),
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

fn lowering_failure(
    semantic: &semantics::SemanticPackage,
    failure: crate::lowering::LoweringFailure,
) -> CompilationFailure {
    let source = semantic
        .units
        .iter()
        .find(|unit| unit.source.id() == failure.span.file)
        .map_or_else(
            || semantic.units[0].source.clone(),
            |unit| unit.source.clone(),
        );
    CompilationFailure {
        source,
        diagnostics: vec![
            Diagnostic::error("S9005", failure.message, failure.span).with_help(
                "this is an internal compiler defect; report the source program and compiler version",
            ),
        ],
    }
}

fn compilation_rust_dependencies(
    package: &Package,
    projection: &crate::projection::Projection,
) -> Vec<RustDependency> {
    let mut dependencies = package
        .rust_dependencies
        .iter()
        .map(|dependency| {
            let mut dependency = dependency.clone();
            if let Some(projected) = projection
                .dependencies
                .iter()
                .find(|projected| projected.name == dependency.name)
            {
                dependency.version = format!("={}", projected.version);
            }
            dependency
        })
        .collect::<Vec<_>>();
    dependencies.extend(
        projection
            .bound_dependencies
            .iter()
            .map(|dependency| RustDependency {
                name: dependency.name.clone(),
                package: dependency.package.clone(),
                version: dependency.version.clone(),
                features: Vec::new(),
                default_features: false,
                target: None,
                effects: Vec::new(),
            }),
    );
    dependencies
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
    let sources: Vec<SourceFile> = semantic
        .units
        .iter()
        .map(|unit| unit.source.clone())
        .collect();
    let warnings = semantics::warnings(&semantic, options.lint_name_style);
    let rust_ir = crate::lowering::lower(&semantic, options.debug_information)
        .map_err(|failure| lowering_failure(&semantic, failure))?;
    let rendered_rust = rust_ir.rendered();
    let standalone_file = rendered_rust.standalone_file("<stdout>");
    let rust = standalone_file.contents.clone();
    let review_rust = rendered_rust.review_file();
    let rust_dependencies = compilation_rust_dependencies(package, &semantic.projection);
    if options.require_canonical_rust {
        validate_canonical_rust(&[standalone_file], &sources, source, entry_span)?;
    }
    Ok(Compilation {
        source: (*source).clone(),
        sources,
        rust,
        review_rust,
        rendered_rust,
        debug_symbols: options
            .debug_information
            .then(|| crate::debugging::DebugSymbols::from_semantic(&semantic)),
        require_canonical_rust: options.require_canonical_rust,
        entry_span,
        requires_platform_support: rust_ir.requires_platform_support,
        requires_async_runtime: rust_ir.requires_async_runtime,
        warnings,
        rust_dependencies,
        dependency_containment: semantic.projection.containment,
    })
}

/// Compiles every populated test tier as an independent native dispatch runner.
///
/// Each tier is analyzed and lowered once. Test functions are ordinary top-level functions whose
/// names begin with `test-`; they may be asynchronous and throwing, but must take no parameters and
/// return `none`.
///
/// # Errors
///
/// Returns ordinary frontend diagnostics or source-oriented test-boundary diagnostics.
pub fn compile_test_package(
    test_package: &TestPackage,
    options: CompilerOptions,
) -> Result<Vec<crate::testing::TestTierCompilation>, CompilationFailure> {
    let tiers = test_package.tier_packages.keys().copied().collect();
    compile_test_package_tiers(test_package, options, &tiers)
}

/// Compiles only the selected populated test tiers as independent native dispatch runners.
///
/// # Errors
///
/// Returns ordinary frontend diagnostics or source-oriented test-boundary diagnostics.
pub fn compile_test_package_tiers(
    test_package: &TestPackage,
    options: CompilerOptions,
    tiers: &BTreeSet<TestTier>,
) -> Result<Vec<crate::testing::TestTierCompilation>, CompilationFailure> {
    let mut compiled = Vec::new();
    let mut identities = BTreeMap::<String, Span>::new();
    for (&tier, package) in test_package
        .tier_packages
        .iter()
        .filter(|(tier, _)| tiers.contains(tier))
    {
        let discovery = discover_test_tier(package, tier, options)?;
        for case in &discovery.cases {
            if let Some(previous) = identities.insert(case.identity.clone(), case.source_span) {
                return Err(duplicate_test_identity_failure(
                    &discovery.semantic.units[0].source,
                    case,
                    previous,
                ));
            }
        }
        compiled.push(compile_discovered_test_tier(discovery, options)?);
    }
    Ok(compiled)
}

/// Discovers test cases through semantic analysis without lowering native dispatch runners.
///
/// # Errors
///
/// Returns ordinary frontend diagnostics or source-oriented test-boundary diagnostics.
pub fn discover_test_package(
    test_package: &TestPackage,
    options: CompilerOptions,
) -> Result<Vec<TestTierDiscovery>, CompilationFailure> {
    let mut discovered = Vec::new();
    let mut identities = BTreeMap::<String, Span>::new();
    for (&tier, package) in &test_package.tier_packages {
        let discovery = discover_test_tier(package, tier, options)?;
        for case in &discovery.cases {
            if let Some(previous) = identities.insert(case.identity.clone(), case.source_span) {
                return Err(duplicate_test_identity_failure(
                    &discovery.semantic.units[0].source,
                    case,
                    previous,
                ));
            }
        }
        discovered.push(discovery);
    }
    Ok(discovered)
}

fn duplicate_test_identity_failure(
    source: &SourceFile,
    case: &TestCase,
    previous: Span,
) -> CompilationFailure {
    CompilationFailure {
        source: source.clone(),
        diagnostics: vec![
            Diagnostic::error(
                "S2052",
                format!("duplicate test identity `{}`", case.identity),
                case.source_span,
            )
            .with_help(format!(
                "the first test with this identity starts at byte {}",
                previous.start
            )),
        ],
    }
}

fn discover_test_tier(
    package: &Package,
    tier: TestTier,
    options: CompilerOptions,
) -> Result<TestTierDiscovery, CompilationFailure> {
    let mut semantic = semantics::analyze(package).map_err(|failure| CompilationFailure {
        source: failure.source,
        diagnostics: failure.diagnostics,
    })?;
    let role = match tier {
        TestTier::Unit => crate::SourceRole::UnitTest,
        TestTier::Integration => crate::SourceRole::IntegrationTest,
        TestTier::EndToEnd => crate::SourceRole::EndToEndTest,
    };
    let mut cases = Vec::new();
    let mut diagnostics = Vec::new();
    for unit in &semantic.units {
        if unit.role != role {
            continue;
        }
        for contract in unit.functions.iter().filter(|contract| {
            contract.span.file == unit.source.id()
                && contract.owner.is_none()
                && contract.name.starts_with("test-")
        }) {
            let returns_none = contract
                .return_type
                .as_ref()
                .is_none_or(|value| *value == ValueType::Scalar(ScalarType::None));
            if !contract.parameters.is_empty() || !returns_none {
                diagnostics.push(
                    Diagnostic::error(
                        "S2051",
                        format!(
                            "test function `{}` must be parameterless and return `none`",
                            contract.name
                        ),
                        contract.span,
                    )
                    .with_help(
                        "use ordinary local functions to supply table rows or parameterized setup",
                    ),
                );
                continue;
            }
            cases.push(TestCase {
                identity: format!(
                    "{}::{}",
                    unit.namespace.trim_end_matches('/'),
                    contract.name
                ),
                tier,
                source_path: unit.source_path.clone(),
                source_span: contract.span,
                is_async: contract.is_async,
                throws: contract.throws,
                selector: 0,
            });
        }
    }
    cases.sort_by(|left, right| {
        (&left.source_path, left.source_span.start, &left.identity).cmp(&(
            &right.source_path,
            right.source_span.start,
            &right.identity,
        ))
    });
    for (selector, case) in cases.iter_mut().enumerate() {
        case.selector = selector;
    }
    semantic.mark_functions_referenced(cases.iter().map(|case| case.source_span));
    if !diagnostics.is_empty() {
        let span = diagnostics[0]
            .primary
            .unwrap_or_else(|| Span::new(semantic.units[0].source.id(), 0, 0));
        let source = semantic
            .units
            .iter()
            .find(|unit| unit.source.id() == span.file)
            .map_or_else(
                || semantic.units[0].source.clone(),
                |unit| unit.source.clone(),
            );
        return Err(CompilationFailure {
            source,
            diagnostics,
        });
    }
    let sources = semantic
        .units
        .iter()
        .map(|unit| unit.source.clone())
        .collect();
    let warnings = semantics::warnings(&semantic, options.lint_name_style);
    Ok(TestTierDiscovery {
        tier,
        cases,
        warnings,
        sources,
        package: package.clone(),
        semantic,
    })
}

/// Lowers one previously discovered test tier without repeating semantic analysis.
///
/// # Errors
///
/// Returns source-oriented lowering or generated-Rust validation diagnostics.
pub fn compile_discovered_test_tier(
    discovery: TestTierDiscovery,
    options: CompilerOptions,
) -> Result<crate::testing::TestTierCompilation, CompilationFailure> {
    let TestTierDiscovery {
        tier,
        cases,
        warnings: _,
        sources,
        package,
        semantic,
    } = discovery;
    let runner_cases = cases
        .iter()
        .map(|case| crate::lowering::TestRunnerCase {
            span: case.source_span,
            is_async: case.is_async,
            throws: case.throws,
        })
        .collect::<Vec<_>>();
    let entry_span = cases.first().map_or_else(
        || Span::new(semantic.units[0].source.id(), 0, 0),
        |case| case.source_span,
    );
    let source = semantic
        .units
        .iter()
        .find(|unit| unit.source.id() == entry_span.file)
        .map_or_else(
            || semantic.units[0].source.clone(),
            |unit| unit.source.clone(),
        );
    let warnings = semantics::warnings(&semantic, options.lint_name_style);
    let rust_ir = crate::lowering::lower_tests(&semantic, &runner_cases, options.debug_information)
        .map_err(|failure| lowering_failure(&semantic, failure))?;
    let rendered_rust = rust_ir.rendered();
    let standalone_file = rendered_rust.standalone_file("<stdout>");
    if options.require_canonical_rust {
        validate_canonical_rust(
            std::slice::from_ref(&standalone_file),
            &sources,
            &source,
            entry_span,
        )?;
    }
    let compilation = Compilation {
        source,
        sources,
        rust: standalone_file.contents,
        review_rust: rendered_rust.review_file(),
        rendered_rust,
        debug_symbols: options
            .debug_information
            .then(|| crate::debugging::DebugSymbols::from_semantic(&semantic)),
        require_canonical_rust: options.require_canonical_rust,
        entry_span,
        requires_platform_support: rust_ir.requires_platform_support,
        requires_async_runtime: rust_ir.requires_async_runtime,
        warnings,
        rust_dependencies: compilation_rust_dependencies(&package, &semantic.projection),
        dependency_containment: semantic.projection.containment,
    };
    Ok(crate::testing::TestTierCompilation {
        tier,
        package: package.clone(),
        compilation,
        cases,
    })
}

fn validate_canonical_rust(
    files: &[RenderedFile],
    sources: &[SourceFile],
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
            let source = sources
                .iter()
                .find(|source| source.id() == span.file)
                .unwrap_or(fallback_source);
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
    use std::path::PathBuf;

    use super::{canonical_rust, first_difference, validate_canonical_rust};
    use crate::{
        SourceFile, Span,
        rust_ir::{RenderedFile, SourceAssociation},
    };

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

    #[test]
    fn canonical_failure_uses_the_associated_authored_source() {
        let fallback =
            SourceFile::new(0, PathBuf::from("entry.trn"), "function main;\n".to_owned());
        let authored = SourceFile::new(
            1,
            PathBuf::from("authored.trn"),
            "function affected;\n".to_owned(),
        );
        let source_span = Span::new(1, 0, 17);
        let rust = "fn affected(){ }\n";
        let failure = validate_canonical_rust(
            &[RenderedFile {
                path: "src/main.rs".to_owned(),
                contents: rust.to_owned(),
                associations: vec![SourceAssociation {
                    generated_start: 0,
                    generated_end: rust.len(),
                    source: source_span,
                }],
            }],
            &[fallback.clone(), authored],
            &fallback,
            Span::new(0, 0, 13),
        )
        .unwrap_err();

        assert_eq!(failure.source.id(), 1);
        assert_eq!(failure.diagnostics[0].code, "S9004");
        assert_eq!(failure.diagnostics[0].primary, Some(source_span));
    }
}
