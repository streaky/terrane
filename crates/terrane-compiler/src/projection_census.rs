use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Serialize;

use crate::package::RustDependency;
use crate::projection::{DeclinedItem, ProjectedKind, Projection};

/// Compiler and native-surface evidence for one exactly pinned Rust package.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectionCensusReport {
    pub schema: u32,
    pub dependency: RustDependencyEvidence,
    pub native: terrane_rust_analysis::SurveyReport,
    pub native_closure: Vec<terrane_rust_analysis::SurveyReport>,
    pub native_closure_failures: Vec<ClosureSurveyFailure>,
    pub native_sysroot: Vec<terrane_rust_analysis::SurveyDeclaration>,
    pub native_sysroot_failures: Vec<ClosureSurveyFailure>,
    pub projection: Projection,
    pub assessment: ProjectionAssessment,
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClosureSurveyFailure {
    pub external_crate: String,
    pub candidate_packages: Vec<String>,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RustDependencyEvidence {
    pub name: String,
    pub package: String,
    pub version: String,
    pub features: Vec<String>,
    pub default_features: bool,
    pub target_condition: Option<String>,
    pub survey_target: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectionAssessment {
    pub discovered_declarations: usize,
    pub discovered_public_paths: usize,
    pub projected_operations: usize,
    pub declined_operations: usize,
    pub discovery_failures: usize,
    pub classifications: BTreeMap<String, usize>,
    pub structural_causes: Vec<StructuralCause>,
    pub declaration_results: Vec<DeclarationResult>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct StructuralCause {
    pub classification: String,
    pub reason: String,
    pub affected_operations: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeclarationResult {
    pub public_path: String,
    pub canonical_path: Option<String>,
    pub native_owner: String,
    pub source: Option<serde_json::Value>,
    pub native_kind: String,
    pub metadata_evidence: String,
    pub classification: String,
    pub explanation: String,
}

/// Runs native discovery and the compiler's real projection engine against the same locked graph.
///
/// The caller owns `root`; projection cache and Cargo lock evidence remain below its `.trn`
/// directory so repeated census runs are reproducible and inspectable.
///
/// # Errors
/// Returns a stable explanation when projection, native discovery, or graph selection fails.
pub fn assess_projection_gap(
    root: &Path,
    dependency: &RustDependency,
    survey_target: &str,
) -> Result<ProjectionCensusReport, String> {
    let projection = crate::projection::resolve(root, std::slice::from_ref(dependency), None)
        .map_err(|error| error.message)?;
    let manifest = root.join(".trn/dependencies/Cargo.toml");
    let selector = format!(
        "{}@{}",
        dependency.package,
        dependency.version.trim_start_matches('=')
    );
    let native =
        terrane_rust_analysis::survey_package_for_target(&manifest, Some(&selector), survey_target)
            .map_err(|error| error.message)?;
    let (native_closure, mut native_closure_failures) =
        survey_closure_owners(&manifest, &native, survey_target);
    let core_paths = native
        .discovery_failures
        .iter()
        .filter(|failure| failure.external_crate.as_deref() == Some("core"))
        .filter_map(|failure| failure.canonical_path.as_ref().map(|path| path.join("::")))
        .collect::<BTreeSet<_>>();
    let (native_sysroot, native_sysroot_failures) =
        match terrane_rust_analysis::survey_core_declarations(
            &root.join(".trn/dependencies/rust-survey/sysroot"),
            survey_target,
            &core_paths,
        ) {
            Ok(declarations) => (declarations, Vec::new()),
            Err(error) => (
                Vec::new(),
                vec![ClosureSurveyFailure {
                    external_crate: "core".to_owned(),
                    candidate_packages: vec![format!("core@{}", crate::RUSTDOC_TOOLCHAIN)],
                    reason: error.message,
                }],
            ),
        };
    native_closure_failures.retain(|failure| failure.external_crate != "core");
    let assessment = assess(&native, &native_closure, &native_sysroot, &projection);
    Ok(ProjectionCensusReport {
        schema: 2,
        dependency: RustDependencyEvidence {
            name: dependency.name.clone(),
            package: dependency.package.clone(),
            version: dependency.version.clone(),
            features: dependency.features.clone(),
            default_features: dependency.default_features,
            target_condition: dependency.target.clone(),
            survey_target: survey_target.to_owned(),
        },
        native,
        native_closure,
        native_closure_failures,
        native_sysroot,
        native_sysroot_failures,
        projection,
        assessment,
    })
}

fn survey_closure_owners(
    manifest: &Path,
    native: &terrane_rust_analysis::SurveyReport,
    target: &str,
) -> (
    Vec<terrane_rust_analysis::SurveyReport>,
    Vec<ClosureSurveyFailure>,
) {
    let mut pending = native
        .discovery_failures
        .iter()
        .filter_map(|failure| failure.external_crate.clone())
        .collect::<BTreeSet<_>>();
    let mut attempted = BTreeSet::new();
    let mut reports = Vec::new();
    let mut failures = Vec::new();
    while let Some(owner) = pending.pop_first() {
        if !attempted.insert(owner.clone()) {
            continue;
        }
        let candidates = native
            .resolved_packages
            .iter()
            .filter(|package| package.name.replace('-', "_") == owner)
            .collect::<Vec<_>>();
        if candidates.len() != 1 {
            failures.push(ClosureSurveyFailure {
                external_crate: owner,
                candidate_packages: candidates
                    .iter()
                    .map(|package| package.identity.clone())
                    .collect(),
                reason: if candidates.is_empty() {
                    "external declaration owner is absent from the resolved target graph".to_owned()
                } else {
                    "external declaration owner is ambiguous in the resolved target graph"
                        .to_owned()
                },
            });
            continue;
        }
        let candidate = candidates[0];
        match terrane_rust_analysis::survey_package_for_target(
            manifest,
            Some(&candidate.identity),
            target,
        ) {
            Ok(report) => {
                pending.extend(
                    report
                        .discovery_failures
                        .iter()
                        .filter_map(|failure| failure.external_crate.clone()),
                );
                reports.push(report);
            }
            Err(error) => failures.push(ClosureSurveyFailure {
                external_crate: owner,
                candidate_packages: vec![candidate.identity.clone()],
                reason: error.message,
            }),
        }
    }
    reports.sort_by(|left, right| {
        left.selected_package
            .identity
            .cmp(&right.selected_package.identity)
    });
    failures.sort_by(|left, right| left.external_crate.cmp(&right.external_crate));
    (reports, failures)
}

struct RecoveredDeclaration {
    owner: String,
    source: Option<serde_json::Value>,
    kind: String,
    metadata_evidence: String,
}

fn classify_paths(
    public_path: &str,
    canonical_path: Option<&str>,
    projected: &BTreeSet<String>,
    declined: &BTreeMap<String, String>,
) -> (&'static str, String) {
    let candidates = [Some(public_path), canonical_path];
    if candidates
        .iter()
        .flatten()
        .any(|path| projected.contains(*path))
    {
        return (
            "projectable",
            "admitted by the compiler projection engine".to_owned(),
        );
    }
    if let Some((_, reason)) = candidates
        .iter()
        .flatten()
        .find_map(|path| declined.get_key_value(*path))
    {
        return ("unsupported-capability", reason.clone());
    }
    (
        "deferred-contextual",
        "discovered native declaration has no independently projected operation; concrete source use may be required".to_owned(),
    )
}

#[expect(
    clippy::too_many_lines,
    reason = "one assessment pass keeps projection indexing, declaration classification, and denominators aligned"
)]
fn assess(
    native: &terrane_rust_analysis::SurveyReport,
    native_closure: &[terrane_rust_analysis::SurveyReport],
    native_sysroot: &[terrane_rust_analysis::SurveyDeclaration],
    projection: &Projection,
) -> ProjectionAssessment {
    let mut projected = BTreeSet::new();
    let mut declined = BTreeMap::new();
    for dependency in &projection.dependencies {
        for item in &dependency.items {
            projected.insert(item.rust_path.clone());
            match &item.kind {
                ProjectedKind::ForeignType {
                    methods,
                    static_methods,
                    ..
                }
                | ProjectedKind::Enum {
                    methods,
                    static_methods,
                    ..
                } => {
                    projected.extend(
                        methods
                            .iter()
                            .chain(static_methods)
                            .map(|method| format!("{}::{}", item.rust_path, method.name)),
                    );
                }
                ProjectedKind::Interface(interface) => {
                    projected.extend(
                        interface
                            .methods
                            .iter()
                            .map(|method| format!("{}::{}", item.rust_path, method.function.name)),
                    );
                    for item in &interface.declined_methods {
                        declined.insert(item.rust_path.clone(), item.reason.clone());
                    }
                }
                ProjectedKind::Function(_) => {}
            }
        }
        for item in &dependency.declined {
            declined.insert(item.rust_path.clone(), item.reason.clone());
        }
    }
    let generated_aliases = projection
        .bound_dependencies
        .iter()
        .filter_map(|dependency| {
            let package = dependency.package.replace('-', "_");
            (dependency.name != package).then_some((dependency.name.as_str(), package))
        })
        .collect::<Vec<_>>();
    for path in projected.clone() {
        for (alias, package) in &generated_aliases {
            if path == *alias {
                projected.insert(package.clone());
            } else if let Some(suffix) = path.strip_prefix(&format!("{alias}::")) {
                projected.insert(format!("{package}::{suffix}"));
            }
        }
    }

    let mut recovered = BTreeMap::new();
    for report in native_closure {
        for declaration in &report.declarations {
            let owner = report.selected_package.identity.clone();
            let recovered_declaration = RecoveredDeclaration {
                owner: owner.clone(),
                source: declaration.source.clone(),
                kind: declaration.kind.clone(),
                metadata_evidence: "closure-definition".to_owned(),
            };
            recovered.insert(
                declaration.public_path.clone(),
                RecoveredDeclaration {
                    owner: owner.clone(),
                    source: declaration.source.clone(),
                    kind: declaration.kind.clone(),
                    metadata_evidence: "closure-definition".to_owned(),
                },
            );
            let canonical = declaration
                .canonical_path
                .as_ref()
                .map(|path| path.join("::"));
            if let Some(canonical) = &canonical {
                recovered.insert(canonical.clone(), recovered_declaration);
            }
            for member in &declaration.members {
                let recovered_member = || RecoveredDeclaration {
                    owner: owner.clone(),
                    source: member.source.clone(),
                    kind: member.kind.clone(),
                    metadata_evidence: "closure-definition".to_owned(),
                };
                recovered.insert(
                    format!("{}::{}", declaration.public_path, member.name),
                    recovered_member(),
                );
                if let Some(canonical) = &canonical {
                    recovered.insert(format!("{canonical}::{}", member.name), recovered_member());
                }
            }
        }
    }
    for declaration in native_sysroot {
        let owner = format!("core@{}", crate::RUSTDOC_TOOLCHAIN);
        let recovered_declaration = RecoveredDeclaration {
            owner: owner.clone(),
            source: declaration.source.clone(),
            kind: declaration.kind.clone(),
            metadata_evidence: "sysroot-definition".to_owned(),
        };
        recovered.insert(
            declaration.public_path.clone(),
            RecoveredDeclaration {
                owner: owner.clone(),
                source: declaration.source.clone(),
                kind: declaration.kind.clone(),
                metadata_evidence: "sysroot-definition".to_owned(),
            },
        );
        if let Some(canonical) = &declaration.canonical_path {
            recovered.insert(canonical.join("::"), recovered_declaration);
        }
    }

    let mut classifications = BTreeMap::new();
    let mut declaration_results =
        Vec::with_capacity(native.declarations.len() + native.discovery_failures.len());
    for declaration in &native.declarations {
        let canonical = declaration
            .canonical_path
            .as_ref()
            .map(|path| path.join("::"));
        let (classification, explanation) = classify_paths(
            &declaration.public_path,
            canonical.as_deref(),
            &projected,
            &declined,
        );
        *classifications
            .entry(classification.to_owned())
            .or_insert(0) += 1;
        declaration_results.push(DeclarationResult {
            public_path: declaration.public_path.clone(),
            canonical_path: canonical,
            native_owner: native.selected_package.identity.clone(),
            source: declaration.source.clone(),
            native_kind: declaration.kind.clone(),
            metadata_evidence: "package-definition".to_owned(),
            classification: classification.to_owned(),
            explanation,
        });
    }
    let mut unresolved = Vec::new();
    for failure in &native.discovery_failures {
        let canonical = failure.canonical_path.as_ref().map(|path| path.join("::"));
        let recovered_declaration = canonical
            .as_ref()
            .and_then(|path| recovered.get(path))
            .or_else(|| recovered.get(&failure.public_path));
        let Some(declaration) = recovered_declaration else {
            let namespace_only = failure.native_kind.as_deref() == Some("module")
                || failure.external_crate.as_ref().is_some_and(|owner| {
                    failure.canonical_path.as_deref() == Some(std::slice::from_ref(owner))
                        && native_closure
                            .iter()
                            .any(|report| report.selected_package.name.replace('-', "_") == *owner)
                });
            if namespace_only {
                *classifications
                    .entry("namespace-path".to_owned())
                    .or_insert(0) += 1;
                declaration_results.push(DeclarationResult {
                    public_path: failure.public_path.clone(),
                    canonical_path: canonical,
                    native_owner: failure.external_crate.clone().unwrap_or_default(),
                    source: None,
                    native_kind: "module".to_owned(),
                    metadata_evidence: "rustdoc-item-summary".to_owned(),
                    classification: "namespace-path".to_owned(),
                    explanation:
                        "public namespace reexport has no operation or type signature to assess"
                            .to_owned(),
                });
                continue;
            }
            if failure.external_crate.as_deref() == Some("std") && failure.native_kind.is_some() {
                let (classification, explanation) = classify_paths(
                    &failure.public_path,
                    canonical.as_deref(),
                    &projected,
                    &declined,
                );
                *classifications
                    .entry(classification.to_owned())
                    .or_insert(0) += 1;
                declaration_results.push(DeclarationResult {
                    public_path: failure.public_path.clone(),
                    canonical_path: canonical,
                    native_owner: format!("std@{}", crate::RUSTDOC_TOOLCHAIN),
                    source: None,
                    native_kind: failure.native_kind.clone().unwrap_or_default(),
                    metadata_evidence: "pinned-sysroot-item-summary".to_owned(),
                    classification: classification.to_owned(),
                    explanation: format!(
                        "{explanation}; exact kind and identity recovered from the pinned toolchain's Rustdoc summary"
                    ),
                });
                continue;
            }
            *classifications
                .entry("missing-metadata".to_owned())
                .or_insert(0) += 1;
            unresolved.push(failure);
            declaration_results.push(DeclarationResult {
                public_path: failure.public_path.clone(),
                canonical_path: canonical,
                native_owner: failure
                    .external_crate
                    .clone()
                    .unwrap_or_else(|| "unknown".to_owned()),
                source: None,
                native_kind: "unknown".to_owned(),
                metadata_evidence: "unavailable".to_owned(),
                classification: "missing-metadata".to_owned(),
                explanation: failure.reason.clone(),
            });
            continue;
        };
        let (classification, explanation) = classify_paths(
            &failure.public_path,
            canonical.as_deref(),
            &projected,
            &declined,
        );
        *classifications
            .entry(classification.to_owned())
            .or_insert(0) += 1;
        declaration_results.push(DeclarationResult {
            public_path: failure.public_path.clone(),
            canonical_path: canonical,
            native_owner: declaration.owner.clone(),
            source: declaration.source.clone(),
            native_kind: declaration.kind.clone(),
            metadata_evidence: declaration.metadata_evidence.clone(),
            classification: classification.to_owned(),
            explanation,
        });
    }

    let mut causes = BTreeMap::<(String, String), usize>::new();
    for DeclinedItem { reason, .. } in projection
        .dependencies
        .iter()
        .flat_map(|dependency| &dependency.declined)
    {
        *causes
            .entry(("unsupported-capability".to_owned(), reason.clone()))
            .or_insert(0) += 1;
    }
    for failure in &unresolved {
        *causes
            .entry(("missing-metadata".to_owned(), failure.reason.clone()))
            .or_insert(0) += 1;
    }
    let mut structural_causes = causes
        .into_iter()
        .map(
            |((classification, reason), affected_operations)| StructuralCause {
                classification,
                reason,
                affected_operations,
            },
        )
        .collect::<Vec<_>>();
    structural_causes.sort_by(|left, right| {
        right
            .affected_operations
            .cmp(&left.affected_operations)
            .then_with(|| left.reason.cmp(&right.reason))
    });

    ProjectionAssessment {
        discovered_declarations: native.declarations.len() + native.discovery_failures.len()
            - unresolved.len(),
        discovered_public_paths: native.public_paths.len(),
        projected_operations: projected.len(),
        declined_operations: declined.len(),
        discovery_failures: unresolved.len(),
        classifications,
        structural_causes,
        declaration_results,
    }
}
