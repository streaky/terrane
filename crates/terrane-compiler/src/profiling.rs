use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::debugging::{
    DebugAssociation, DebugInformation, GeneratedFileIdentity, ProvenanceRole, SourceSpan,
};
use crate::provenance::{ArtifactProfile, BuildProvenance};

pub const SCHEMA_VERSION: &str = "1.0";
pub const ATTRIBUTION_SCHEMA_VERSION: &str = "1.0";
pub const MAX_CAPTURED_SAMPLES: usize = 1_000_000;
pub const MAX_STACK_DEPTH: usize = 4_096;

pub const CPU_ARTIFACT_PROFILE: ArtifactProfile = ArtifactProfile {
    id: "terrane-profile-cpu-v1",
    optimization: "3",
    cargo_debug: "1",
    debug_information: "line-tables-only",
    stripping: "none",
    inlining: "compiler-default-at-opt-level-3",
    lto: "thin",
    codegen_units: 1,
    panic: "package-policy",
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceKind {
    CpuSamples,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceUnit {
    SampleCount,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArgumentPolicy {
    Omitted,
    Retained,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrivacyDeclaration {
    pub source_paths: bool,
    pub symbol_names: bool,
    pub arguments: ArgumentPolicy,
    pub timing: bool,
    pub embedded_sources: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CollectorIdentity {
    pub name: String,
    pub version: String,
    pub raw_configuration: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CollectionConditions {
    pub host: String,
    pub target: String,
    pub build_profile: String,
    pub workload: String,
    pub arguments: Vec<String>,
    pub argument_policy: ArgumentPolicy,
    pub included_processes: String,
    pub included_threads: String,
    pub sample_frequency_hz: u32,
    pub sample_period: String,
    pub elapsed_nanoseconds: u64,
    pub active_nanoseconds: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warmup_nanoseconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_exit_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminating_signal: Option<i32>,
    pub interrupted: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CapturedModule {
    pub path: String,
    pub build_id: String,
    pub content_hash: String,
    pub is_profiled_executable: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NativeSourceLocation {
    pub path: String,
    pub line: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NativeFrame {
    pub module: usize,
    pub module_offset: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_location: Option<NativeSourceLocation>,
    pub inline: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CpuSample {
    pub process_id: u32,
    pub thread_id: u32,
    pub monotonic_nanoseconds: u64,
    pub stack: Vec<NativeFrame>,
    pub unreadable: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CollectionLoss {
    pub lost_events: u64,
    pub captured_events: u64,
    pub truncated_events: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CpuEvidence {
    pub modules: Vec<CapturedModule>,
    pub samples: Vec<CpuSample>,
    pub loss: CollectionLoss,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProfileArtifact {
    pub schema_version: String,
    pub evidence_kind: EvidenceKind,
    pub evidence_unit: EvidenceUnit,
    pub attribution_schema_version: String,
    pub provenance: BuildProvenance,
    pub source_attribution: DebugInformation,
    pub collector: CollectorIdentity,
    pub conditions: CollectionConditions,
    pub privacy: PrivacyDeclaration,
    pub evidence: CpuEvidence,
}

impl ProfileArtifact {
    /// Validates bounds, exact-module identity, units, and whole-capture accounting.
    ///
    /// # Errors
    ///
    /// Returns a stable explanation when the artifact cannot be safely consumed.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(format!(
                "unsupported profile schema `{}`; expected `{SCHEMA_VERSION}`",
                self.schema_version
            ));
        }
        if self.attribution_schema_version != ATTRIBUTION_SCHEMA_VERSION {
            return Err(format!(
                "unsupported attribution schema `{}`; expected `{ATTRIBUTION_SCHEMA_VERSION}`",
                self.attribution_schema_version
            ));
        }
        if self.evidence_kind != EvidenceKind::CpuSamples
            || self.evidence_unit != EvidenceUnit::SampleCount
        {
            return Err("CPU profile evidence must use sample-count units".to_owned());
        }
        if self.evidence.samples.len() > MAX_CAPTURED_SAMPLES {
            return Err(format!(
                "profile contains {} samples; limit is {MAX_CAPTURED_SAMPLES}",
                self.evidence.samples.len()
            ));
        }
        if let Some(depth) = self
            .evidence
            .samples
            .iter()
            .map(|sample| sample.stack.len())
            .find(|depth| *depth > MAX_STACK_DEPTH)
        {
            return Err(format!(
                "profile contains a stack with {depth} frames; limit is {MAX_STACK_DEPTH}"
            ));
        }
        if self.evidence.loss.captured_events != self.evidence.samples.len() as u64 {
            return Err(format!(
                "capture accounting reports {} captured events but stores {} samples",
                self.evidence.loss.captured_events,
                self.evidence.samples.len()
            ));
        }
        if self
            .evidence
            .samples
            .iter()
            .flat_map(|sample| &sample.stack)
            .any(|frame| frame.module >= self.evidence.modules.len())
        {
            return Err("profile frame references an unknown native module".to_owned());
        }
        let Some(executable) = self
            .evidence
            .modules
            .iter()
            .find(|module| module.is_profiled_executable)
        else {
            return Err("profile does not identify its exact executable module".to_owned());
        };
        if executable.content_hash != self.provenance.native_module.content_hash {
            return Err("captured executable module differs from build provenance".to_owned());
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AttributionQuality {
    ExactAuthored,
    SharedOrAmbiguous,
    RuntimeAssociated,
    GeneratedOnly,
    NativeOnly,
    Unavailable,
}

impl AttributionQuality {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactAuthored => "exact authored",
            Self::SharedOrAmbiguous => "shared or ambiguous",
            Self::RuntimeAssociated => "runtime-associated",
            Self::GeneratedOnly => "generated-only",
            Self::NativeOnly => "native-only",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SourceCostIdentity {
    pub source_id: u32,
    pub source_uri: String,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub end_line: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    pub association_role: ProvenanceRole,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GeneratedConstituent {
    pub path: String,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NativeConstituent {
    pub module: String,
    pub module_offset: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AttributionRow {
    pub quality: AttributionQuality,
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceCostIdentity>,
    pub related_causes: Vec<SourceSpan>,
    pub exclusive_samples: u64,
    pub inclusive_samples: u64,
    pub generated: Vec<GeneratedConstituent>,
    pub native: Vec<NativeConstituent>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WeightedStack {
    pub frames: Vec<String>,
    pub samples: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AttributionReport {
    pub captured_samples: u64,
    pub lost_samples: u64,
    pub truncated_samples: u64,
    pub buckets: BTreeMap<AttributionQuality, u64>,
    pub rows: Vec<AttributionRow>,
    pub call_tree: Vec<WeightedStack>,
    pub flame_graph: Vec<WeightedStack>,
    pub fidelity: String,
    pub fidelity_reasons: Vec<String>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RowKey {
    quality: AttributionQuality,
    label: String,
    source: Option<SourceCostIdentity>,
}

#[derive(Clone, Debug)]
struct FrameAttribution {
    key: RowKey,
    related_causes: Vec<SourceSpan>,
    generated: Vec<GeneratedConstituent>,
    native: NativeConstituent,
}

#[derive(Default)]
struct RowAccumulator {
    exclusive: u64,
    inclusive: u64,
    related_causes: Vec<SourceSpan>,
    generated: BTreeSet<(String, usize, usize, usize)>,
    native: BTreeSet<(String, u64, Option<String>)>,
}

/// Re-runs semantic attribution over normalized native evidence.
/// Every captured sample enters exactly one exclusive quality bucket. Source and generated
/// identities are validated against the selected relocation roots before authored attribution.
#[must_use]
pub fn attribute(
    artifact: &ProfileArtifact,
    source_root: &Path,
    build_root: &Path,
) -> AttributionReport {
    let (valid_sources, valid_generated, mut fidelity_reasons) =
        validate_attribution_inputs(artifact, source_root, build_root);
    let mut buckets = BTreeMap::from([
        (AttributionQuality::ExactAuthored, 0),
        (AttributionQuality::SharedOrAmbiguous, 0),
        (AttributionQuality::RuntimeAssociated, 0),
        (AttributionQuality::GeneratedOnly, 0),
        (AttributionQuality::NativeOnly, 0),
        (AttributionQuality::Unavailable, 0),
    ]);
    let mut rows = BTreeMap::<RowKey, RowAccumulator>::new();
    let mut stacks = BTreeMap::<Vec<String>, u64>::new();
    for sample in &artifact.evidence.samples {
        let frames = sample
            .stack
            .iter()
            .map(|frame| classify_frame(artifact, frame, &valid_sources, &valid_generated))
            .collect::<Vec<_>>();
        let exclusive = if sample.unreadable || frames.is_empty() {
            unavailable_attribution("<unreadable sample>")
        } else {
            frames[0].clone()
        };
        *buckets.entry(exclusive.key.quality).or_default() += 1;
        accumulate_row(&mut rows, &exclusive, true);

        let mut stack = Vec::new();
        let mut inclusive_keys = BTreeSet::new();
        for frame in frames.iter().rev() {
            if stack.last() != Some(&frame.key.label) {
                stack.push(frame.key.label.clone());
            }
            if inclusive_keys.insert(frame.key.clone()) {
                accumulate_row(&mut rows, frame, false);
            }
        }
        if stack.is_empty() {
            stack.push("<unreadable sample>".to_owned());
        }
        *stacks.entry(stack).or_default() += 1;
    }
    let accounted: u64 = buckets.values().sum();
    if accounted != artifact.evidence.loss.captured_events {
        fidelity_reasons.push(format!(
            "exclusive accounting produced {accounted} events for {} captured samples",
            artifact.evidence.loss.captured_events
        ));
    }
    let mut rows = rows
        .into_iter()
        .map(|(key, accumulator)| AttributionRow {
            quality: key.quality,
            label: key.label,
            source: key.source,
            related_causes: accumulator.related_causes,
            exclusive_samples: accumulator.exclusive,
            inclusive_samples: accumulator.inclusive,
            generated: accumulator
                .generated
                .into_iter()
                .map(|(path, line, start, end)| GeneratedConstituent {
                    path,
                    line,
                    start,
                    end,
                })
                .collect(),
            native: accumulator
                .native
                .into_iter()
                .map(|(module, module_offset, symbol)| NativeConstituent {
                    module,
                    module_offset,
                    symbol,
                })
                .collect(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        right
            .exclusive_samples
            .cmp(&left.exclusive_samples)
            .then_with(|| right.inclusive_samples.cmp(&left.inclusive_samples))
            .then_with(|| left.label.cmp(&right.label))
    });
    let weighted_stacks = stacks
        .into_iter()
        .map(|(frames, samples)| WeightedStack { frames, samples })
        .collect::<Vec<_>>();
    AttributionReport {
        captured_samples: artifact.evidence.loss.captured_events,
        lost_samples: artifact.evidence.loss.lost_events,
        truncated_samples: artifact.evidence.loss.truncated_events,
        buckets,
        rows,
        call_tree: weighted_stacks.clone(),
        flame_graph: weighted_stacks,
        fidelity: if fidelity_reasons.is_empty() {
            "exact-build-source".to_owned()
        } else {
            "reduced-native".to_owned()
        },
        fidelity_reasons,
    }
}

fn validate_attribution_inputs(
    artifact: &ProfileArtifact,
    source_root: &Path,
    build_root: &Path,
) -> (BTreeSet<u32>, BTreeSet<String>, Vec<String>) {
    let mut sources = BTreeSet::new();
    let mut generated = BTreeSet::new();
    let mut reasons = Vec::new();
    for source in &artifact.source_attribution.sources {
        let actual = source.embedded_source.as_ref().map_or_else(
            || std::fs::read(source_root.join(&source.uri)).ok(),
            |text| Some(text.as_bytes().to_vec()),
        );
        if actual
            .as_deref()
            .is_some_and(|bytes| crate::provenance::hash_bytes(bytes) == source.content_hash)
        {
            sources.insert(source.id);
        } else {
            reasons.push(format!(
                "authored source `{}` is missing or changed",
                source.uri
            ));
        }
    }
    for file in &artifact.source_attribution.generated_files {
        let actual = file.embedded_source.as_ref().map_or_else(
            || std::fs::read(build_root.join(&file.path)).ok(),
            |text| Some(text.as_bytes().to_vec()),
        );
        if actual
            .as_deref()
            .is_some_and(|bytes| crate::provenance::hash_bytes(bytes) == file.content_hash)
        {
            generated.insert(file.path.clone());
        } else {
            reasons.push(format!(
                "generated source `{}` is missing or changed",
                file.path
            ));
        }
    }
    reasons.sort();
    reasons.dedup();
    (sources, generated, reasons)
}

fn classify_frame(
    artifact: &ProfileArtifact,
    frame: &NativeFrame,
    valid_sources: &BTreeSet<u32>,
    valid_generated: &BTreeSet<String>,
) -> FrameAttribution {
    let Some(module) = artifact.evidence.modules.get(frame.module) else {
        return unavailable_attribution("<unknown module>");
    };
    let native = NativeConstituent {
        module: module.path.clone(),
        module_offset: frame.module_offset,
        symbol: frame.symbol.clone(),
    };
    if !module.is_profiled_executable {
        return FrameAttribution {
            key: RowKey {
                quality: AttributionQuality::NativeOnly,
                label: frame.symbol.clone().unwrap_or_else(|| module.path.clone()),
                source: None,
            },
            related_causes: Vec::new(),
            generated: Vec::new(),
            native,
        };
    }
    let Some(location) = &frame.generated_location else {
        return FrameAttribution {
            key: RowKey {
                quality: AttributionQuality::NativeOnly,
                label: frame
                    .symbol
                    .clone()
                    .unwrap_or_else(|| "<native instruction>".to_owned()),
                source: None,
            },
            related_causes: Vec::new(),
            generated: Vec::new(),
            native,
        };
    };
    let Some(file) = artifact
        .source_attribution
        .generated_files
        .iter()
        .find(|file| Path::new(&location.path).ends_with(&file.path))
    else {
        return FrameAttribution {
            key: RowKey {
                quality: AttributionQuality::NativeOnly,
                label: frame
                    .symbol
                    .clone()
                    .unwrap_or_else(|| location.path.clone()),
                source: None,
            },
            related_causes: Vec::new(),
            generated: Vec::new(),
            native,
        };
    };
    if !valid_generated.contains(&file.path) {
        return unavailable_attribution_with_native(
            format!("{}:{}", file.path, location.line),
            native,
        );
    }
    association_attribution(artifact, file, location.line, native, valid_sources)
}

fn association_attribution(
    artifact: &ProfileArtifact,
    file: &GeneratedFileIdentity,
    line: usize,
    native: NativeConstituent,
    valid_sources: &BTreeSet<u32>,
) -> FrameAttribution {
    let candidates = file
        .associations
        .iter()
        .filter(|association| {
            association.generated.line <= line && line <= association.generated.end_line
        })
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return FrameAttribution {
            key: RowKey {
                quality: AttributionQuality::GeneratedOnly,
                label: format!("{}:{line}", file.path),
                source: None,
            },
            related_causes: Vec::new(),
            generated: Vec::new(),
            native,
        };
    }
    let smallest = candidates
        .iter()
        .map(|association| association.generated.end - association.generated.start)
        .min()
        .unwrap_or(0);
    let candidates = candidates
        .into_iter()
        .filter(|association| association.generated.end - association.generated.start == smallest)
        .collect::<Vec<_>>();
    if candidates
        .iter()
        .flat_map(|association| &association.causes)
        .any(|cause| !valid_sources.contains(&cause.source_id))
    {
        return unavailable_attribution_with_native(format!("{}:{line}", file.path), native);
    }
    let owners = candidates
        .iter()
        .filter_map(|association| source_identity(artifact, association))
        .collect::<BTreeSet<_>>();
    let roles = candidates
        .iter()
        .map(|association| association.role.clone())
        .collect::<BTreeSet<_>>();
    let quality = if roles.contains(&ProvenanceRole::User) {
        if owners.len() == 1 {
            AttributionQuality::ExactAuthored
        } else {
            AttributionQuality::SharedOrAmbiguous
        }
    } else if roles
        .iter()
        .any(|role| matches!(role, ProvenanceRole::Runtime | ProvenanceRole::Cleanup))
        && !owners.is_empty()
    {
        AttributionQuality::RuntimeAssociated
    } else {
        AttributionQuality::GeneratedOnly
    };
    let source = (owners.len() == 1)
        .then(|| owners.iter().next().cloned())
        .flatten();
    let label = source.as_ref().map_or_else(
        || format!("{}:{line}", file.path),
        |source| source_label(artifact, source),
    );
    FrameAttribution {
        key: RowKey {
            quality,
            label,
            source,
        },
        related_causes: candidates
            .iter()
            .flat_map(|association| association.causes.iter().cloned())
            .collect(),
        generated: candidates
            .iter()
            .map(|association| GeneratedConstituent {
                path: file.path.clone(),
                line,
                start: association.generated.start,
                end: association.generated.end,
            })
            .collect(),
        native,
    }
}

fn source_identity(
    artifact: &ProfileArtifact,
    association: &DebugAssociation,
) -> Option<SourceCostIdentity> {
    let cause = association
        .causes
        .iter()
        .min_by_key(|cause| cause.end.saturating_sub(cause.start))?;
    let source = artifact
        .source_attribution
        .sources
        .iter()
        .find(|source| source.id == cause.source_id)?;
    Some(SourceCostIdentity {
        source_id: cause.source_id,
        source_uri: source.uri.clone(),
        start: cause.start,
        end: cause.end,
        line: cause.line,
        end_line: cause.end_line,
        function_id: association.function_id.clone(),
        association_role: association.role.clone(),
    })
}

fn source_label(artifact: &ProfileArtifact, source: &SourceCostIdentity) -> String {
    source
        .function_id
        .as_ref()
        .and_then(|identity| {
            artifact
                .source_attribution
                .functions
                .iter()
                .find(|function| &function.id == identity)
                .map(|function| format!("{} {}", function.name, source.source_uri))
        })
        .unwrap_or_else(|| {
            if source.line == source.end_line {
                format!("{}:{}", source.source_uri, source.line)
            } else {
                format!("{}:{}-{}", source.source_uri, source.line, source.end_line)
            }
        })
}

fn unavailable_attribution(label: &str) -> FrameAttribution {
    unavailable_attribution_with_native(
        label.to_owned(),
        NativeConstituent {
            module: "<unavailable>".to_owned(),
            module_offset: 0,
            symbol: None,
        },
    )
}

fn unavailable_attribution_with_native(
    label: String,
    native: NativeConstituent,
) -> FrameAttribution {
    FrameAttribution {
        key: RowKey {
            quality: AttributionQuality::Unavailable,
            label,
            source: None,
        },
        related_causes: Vec::new(),
        generated: Vec::new(),
        native,
    }
}

fn accumulate_row(
    rows: &mut BTreeMap<RowKey, RowAccumulator>,
    attribution: &FrameAttribution,
    exclusive: bool,
) {
    let row = rows.entry(attribution.key.clone()).or_default();
    if exclusive {
        row.exclusive += 1;
    } else {
        row.inclusive += 1;
    }
    for cause in &attribution.related_causes {
        if !row.related_causes.contains(cause) {
            row.related_causes.push(cause.clone());
        }
    }
    row.generated
        .extend(attribution.generated.iter().map(|generated| {
            (
                generated.path.clone(),
                generated.line,
                generated.start,
                generated.end,
            )
        }));
    row.native.insert((
        attribution.native.module.clone(),
        attribution.native.module_offset,
        attribution.native.symbol.clone(),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debugging::{
        DebugAssociation, DebugInformation, GeneratedFileIdentity, GeneratedRange, SourceIdentity,
        SourceSpan,
    };
    use crate::provenance::{NativeModuleIdentity, RelocationMapping};

    fn artifact() -> ProfileArtifact {
        ProfileArtifact {
            schema_version: SCHEMA_VERSION.to_owned(),
            evidence_kind: EvidenceKind::CpuSamples,
            evidence_unit: EvidenceUnit::SampleCount,
            attribution_schema_version: ATTRIBUTION_SCHEMA_VERSION.to_owned(),
            provenance: BuildProvenance {
                compiler_version: "compiler".to_owned(),
                rust_toolchain: "toolchain".to_owned(),
                target: "x86_64-unknown-linux-gnu".to_owned(),
                rust_sysroot: "/rust".to_owned(),
                rustc_release: "rustc".to_owned(),
                abi_recipe: "abi".to_owned(),
                artifact_profile: CPU_ARTIFACT_PROFILE.id.to_owned(),
                optimization: CPU_ARTIFACT_PROFILE.optimization.to_owned(),
                debug_information: CPU_ARTIFACT_PROFILE.debug_information.to_owned(),
                inlining: CPU_ARTIFACT_PROFILE.inlining.to_owned(),
                stripping: CPU_ARTIFACT_PROFILE.stripping.to_owned(),
                lto: CPU_ARTIFACT_PROFILE.lto.to_owned(),
                codegen_units: CPU_ARTIFACT_PROFILE.codegen_units,
                panic: CPU_ARTIFACT_PROFILE.panic.to_owned(),
                inputs: Vec::new(),
                native_module: NativeModuleIdentity {
                    file_name: "program".to_owned(),
                    content_hash: "sha256:program".to_owned(),
                },
                relocation: RelocationMapping {
                    build_root: "/build".to_owned(),
                    source_root: "/source".to_owned(),
                },
            },
            source_attribution: DebugInformation {
                schema_version: crate::debugging::SCHEMA_VERSION.to_owned(),
                compiler_version: crate::VERSION.to_owned(),
                sources: Vec::new(),
                functions: Vec::new(),
                scopes: Vec::new(),
                bindings: Vec::new(),
                objects: Vec::new(),
                generated_files: Vec::new(),
            },
            collector: CollectorIdentity {
                name: "perf".to_owned(),
                version: "perf version fixture".to_owned(),
                raw_configuration: Vec::new(),
            },
            conditions: CollectionConditions {
                host: "linux-x86_64".to_owned(),
                target: "x86_64-unknown-linux-gnu".to_owned(),
                build_profile: CPU_ARTIFACT_PROFILE.id.to_owned(),
                workload: "program".to_owned(),
                arguments: Vec::new(),
                argument_policy: ArgumentPolicy::Omitted,
                included_processes: "launched-process-tree".to_owned(),
                included_threads: "all".to_owned(),
                sample_frequency_hz: 999,
                sample_period: "frequency-derived".to_owned(),
                elapsed_nanoseconds: 1,
                active_nanoseconds: 1,
                warmup_nanoseconds: None,
                process_exit_code: Some(0),
                terminating_signal: None,
                interrupted: false,
            },
            privacy: PrivacyDeclaration {
                source_paths: true,
                symbol_names: true,
                arguments: ArgumentPolicy::Omitted,
                timing: true,
                embedded_sources: false,
            },
            evidence: CpuEvidence {
                modules: vec![CapturedModule {
                    path: "/build/program".to_owned(),
                    build_id: "build-id".to_owned(),
                    content_hash: "sha256:program".to_owned(),
                    is_profiled_executable: true,
                }],
                samples: vec![CpuSample {
                    process_id: 1,
                    thread_id: 1,
                    monotonic_nanoseconds: 1,
                    stack: vec![NativeFrame {
                        module: 0,
                        module_offset: 1,
                        symbol: Some("main".to_owned()),
                        symbol_offset: Some(0),
                        generated_location: None,
                        inline: false,
                    }],
                    unreadable: false,
                }],
                loss: CollectionLoss {
                    lost_events: 0,
                    captured_events: 1,
                    truncated_events: 0,
                },
            },
        }
    }

    #[test]
    fn validates_typed_cpu_evidence_and_exact_executable() {
        artifact().validate().unwrap();

        let mut mismatched = artifact();
        mismatched.evidence.modules[0].content_hash = "sha256:other".to_owned();
        assert_eq!(
            mismatched.validate().unwrap_err(),
            "captured executable module differs from build provenance"
        );
    }

    #[test]
    fn rejects_unknown_modules_and_inconsistent_capture_accounting() {
        let mut unknown = artifact();
        unknown.evidence.samples[0].stack[0].module = 1;
        assert_eq!(
            unknown.validate().unwrap_err(),
            "profile frame references an unknown native module"
        );

        let mut inconsistent = artifact();
        inconsistent.evidence.loss.captured_events = 2;
        assert!(
            inconsistent
                .validate()
                .unwrap_err()
                .contains("reports 2 captured events but stores 1 samples")
        );
    }
    #[test]
    fn attributes_each_sample_once_and_preserves_ambiguous_causes() {
        let mut artifact = artifact();
        let source_text = "function main;\n";
        let generated_text = "fn main() {}\n";
        let source = |id, uri: &str| SourceIdentity {
            id,
            uri: uri.to_owned(),
            content_hash: crate::provenance::hash_bytes(source_text.as_bytes()),
            embedded_source: Some(source_text.to_owned()),
        };
        let cause = |source_id| SourceSpan {
            source_id,
            start: 0,
            end: 14,
            line: 1,
            column: 1,
            end_line: 1,
            end_column: 15,
        };
        let association = |source_id| DebugAssociation {
            generated: GeneratedRange {
                start: 0,
                end: generated_text.len(),
                line: 1,
                column: 1,
                end_line: 1,
                end_column: generated_text.len() + 1,
            },
            causes: vec![cause(source_id)],
            role: ProvenanceRole::User,
            sequence_point: true,
            function_id: None,
            scope_ids: Vec::new(),
        };
        artifact.source_attribution.sources = vec![source(1, "src/α.trn"), source(2, "src/β.trn")];
        artifact.source_attribution.generated_files = vec![GeneratedFileIdentity {
            path: "src/main.rs".to_owned(),
            content_hash: crate::provenance::hash_bytes(generated_text.as_bytes()),
            embedded_source: Some(generated_text.to_owned()),
            associations: vec![association(1), association(2)],
        }];
        artifact.evidence.samples[0].stack[0].generated_location = Some(NativeSourceLocation {
            path: "/relocated/build/src/main.rs".to_owned(),
            line: 1,
            column: None,
        });

        let report = attribute(&artifact, Path::new("/missing"), Path::new("/missing"));

        assert_eq!(report.captured_samples, 1);
        assert_eq!(report.buckets[&AttributionQuality::SharedOrAmbiguous], 1);
        assert_eq!(report.buckets.values().sum::<u64>(), 1);
        assert_eq!(report.rows[0].exclusive_samples, 1);
        assert_eq!(report.rows[0].related_causes.len(), 2);
        assert_eq!(report.fidelity, "exact-build-source");

        artifact.source_attribution.generated_files[0]
            .associations
            .truncate(1);
        let inline_frame = artifact.evidence.samples[0].stack[0].clone();
        artifact.evidence.samples[0].stack.push(inline_frame);
        let exact = attribute(&artifact, Path::new("/relocated"), Path::new("/relocated"));
        assert_eq!(exact.buckets[&AttributionQuality::ExactAuthored], 1);
        let authored = exact
            .rows
            .iter()
            .find(|row| row.quality == AttributionQuality::ExactAuthored)
            .unwrap();
        assert_eq!(authored.exclusive_samples, 1);
        assert_eq!(authored.inclusive_samples, 1);

        artifact.source_attribution.sources[0].embedded_source = None;
        artifact.source_attribution.generated_files[0].embedded_source = None;
        let stale = attribute(&artifact, Path::new("/missing"), Path::new("/missing"));
        assert_eq!(stale.buckets[&AttributionQuality::Unavailable], 1);
        assert_eq!(stale.fidelity, "reduced-native");
    }
}
