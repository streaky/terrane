use serde::{Deserialize, Serialize};

use crate::debugging::DebugInformation;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debugging::DebugInformation;
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
}
