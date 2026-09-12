use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::projection::{Containment, ProjectionError};

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BoundQuestion {
    pub rust_type: String,
    pub rust_bound: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProbeAnswer {
    Yes,
    No,
    Unknown { reason: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProbeEvidence {
    pub question: BoundQuestion,
    pub answer: ProbeAnswer,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProbeReport {
    pub evidence: Vec<ProbeEvidence>,
    pub compiled_probe_count: usize,
    pub wall_time_ms: u128,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CallQuestion {
    pub label: String,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CallProbeEvidence {
    pub question: CallQuestion,
    pub answer: ProbeAnswer,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CallProbeReport {
    pub evidence: Vec<CallProbeEvidence>,
    pub compiled_probe_count: usize,
    pub wall_time_ms: u128,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ImplQuestion {
    pub label: String,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImplProbeEvidence {
    pub question: ImplQuestion,
    pub answer: ProbeAnswer,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ImplProbeReport {
    pub evidence: Vec<ImplProbeEvidence>,
    pub compiled_probe_count: usize,
    pub wall_time_ms: u128,
}

#[derive(Debug)]
pub struct ProjectionOracle<'a> {
    workspace: &'a Path,
    cache_identity: &'a str,
    containment: Containment,
}

impl<'a> ProjectionOracle<'a> {
    #[must_use]
    pub fn new(workspace: &'a Path, cache_identity: &'a str, containment: Containment) -> Self {
        Self {
            workspace,
            cache_identity,
            containment,
        }
    }

    /// Proves a batch of concrete Rust trait-bound questions.
    ///
    /// # Errors
    ///
    /// Returns an error when the deterministic probe workspace or its cache cannot be prepared.
    pub fn prove_bounds(
        &self,
        questions: &[BoundQuestion],
    ) -> Result<ProbeReport, ProjectionError> {
        let mut questions = questions.to_vec();
        questions.sort();
        questions.dedup();
        if questions.is_empty() {
            return Ok(ProbeReport::default());
        }
        let batch_identity = batch_identity(self.cache_identity, &questions)?;
        let cache_path = self.workspace.join(format!("oracle-{batch_identity}.json"));
        if let Ok(bytes) = fs::read(&cache_path) {
            return serde_json::from_slice(&bytes).map_err(|error| ProjectionError {
                message: format!("invalid cached projection probe: {error}"),
            });
        }

        let started = Instant::now();
        let bin_directory = self.workspace.join("src/bin");
        if bin_directory.exists() {
            fs::remove_dir_all(&bin_directory)
                .map_err(io_error("clear projection probe directory"))?;
        }
        fs::create_dir_all(&bin_directory)
            .map_err(io_error("create projection probe directory"))?;
        let mut names = BTreeMap::new();
        for (index, question) in questions.iter().enumerate() {
            let name = format!("terrane_probe_{index}");
            names.insert(name.clone(), index);
            let source = format!(
                "fn assert_bound<T: {}>() {{}}\nfn main() {{ assert_bound::<{}>(); }}\n",
                question.rust_bound, question.rust_type
            );
            write_if_changed(&bin_directory.join(format!("{name}.rs")), source.as_bytes())?;
        }

        let output = cargo_output(
            self.workspace,
            &[
                "check",
                "--bins",
                "--keep-going",
                "--offline",
                "--frozen",
                "--message-format=json",
            ],
            self.containment,
        )?;
        let answers = classify_probe_answers(&output, &names, questions.len());
        let report = ProbeReport {
            evidence: questions
                .into_iter()
                .zip(answers)
                .map(|(question, answer)| ProbeEvidence { question, answer })
                .collect(),
            compiled_probe_count: names.len(),
            wall_time_ms: started.elapsed().as_millis(),
        };
        let mut bytes = serde_json::to_vec_pretty(&report).map_err(|error| ProjectionError {
            message: format!("cannot serialize projection probe cache: {error}"),
        })?;
        bytes.push(b'\n');
        write_if_changed(&cache_path, &bytes)?;
        Ok(report)
    }

    /// Compiles exact Rust call shapes against the resolved dependency workspace.
    ///
    /// A successful answer requires Cargo to emit a compiler artifact for that probe target.
    /// Probe-local compiler failures are `No` only for trait-bound failures; every other failure
    /// remains `Unknown`.
    ///
    /// # Errors
    ///
    /// Returns an error when the deterministic probe workspace or its cache cannot be prepared.
    pub fn prove_calls(
        &self,
        questions: &[CallQuestion],
    ) -> Result<CallProbeReport, ProjectionError> {
        let mut questions = questions.to_vec();
        questions.sort();
        questions.dedup();
        if questions.is_empty() {
            return Ok(CallProbeReport::default());
        }
        let batch_identity = call_batch_identity(self.cache_identity, &questions)?;
        let cache_path = self
            .workspace
            .join(format!("oracle-calls-{batch_identity}.json"));
        if let Ok(bytes) = fs::read(&cache_path) {
            return serde_json::from_slice(&bytes).map_err(|error| ProjectionError {
                message: format!("invalid cached projection call probe: {error}"),
            });
        }

        let started = Instant::now();
        let bin_directory = self.workspace.join("src/bin");
        if bin_directory.exists() {
            fs::remove_dir_all(&bin_directory)
                .map_err(io_error("clear projection probe directory"))?;
        }
        fs::create_dir_all(&bin_directory)
            .map_err(io_error("create projection probe directory"))?;
        let mut names = BTreeMap::new();
        for (index, question) in questions.iter().enumerate() {
            let name = format!("terrane_call_probe_{index}");
            names.insert(name.clone(), index);
            write_if_changed(
                &bin_directory.join(format!("{name}.rs")),
                question.source.as_bytes(),
            )?;
        }
        let output = cargo_output(
            self.workspace,
            &[
                "check",
                "--bins",
                "--keep-going",
                "--offline",
                "--frozen",
                "--message-format=json",
            ],
            self.containment,
        )?;

        let report = CallProbeReport {
            evidence: questions
                .into_iter()
                .zip(classify_probe_answers(&output, &names, names.len()))
                .map(|(question, answer)| CallProbeEvidence { question, answer })
                .collect(),
            compiled_probe_count: names.len(),
            wall_time_ms: started.elapsed().as_millis(),
        };
        let mut bytes = serde_json::to_vec_pretty(&report).map_err(|error| ProjectionError {
            message: format!("cannot serialize projection call probe cache: {error}"),
        })?;
        bytes.push(b'\n');
        write_if_changed(&cache_path, &bytes)?;
        Ok(report)
    }

    /// Proves complete impl-shaped witnesses against the resolved dependency graph.
    ///
    /// # Errors
    /// Returns a projection error when the complete witness probe cannot be compiled or cached.
    pub fn prove_impls(
        &self,
        questions: &[ImplQuestion],
    ) -> Result<ImplProbeReport, ProjectionError> {
        let calls = questions
            .iter()
            .map(|question| CallQuestion {
                label: format!("impl:{}", question.label),
                source: question.source.clone(),
            })
            .collect::<Vec<_>>();
        let report = self.prove_calls(&calls)?;
        Ok(ImplProbeReport {
            evidence: questions
                .iter()
                .cloned()
                .zip(report.evidence.into_iter().map(|evidence| evidence.answer))
                .map(|(question, answer)| ImplProbeEvidence { question, answer })
                .collect(),
            compiled_probe_count: report.compiled_probe_count,
            wall_time_ms: report.wall_time_ms,
        })
    }

    /// Expands one macro-bearing library through rustdoc and returns its typed JSON input.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid probe name or when generation, rustdoc, or reading fails.
    pub fn expand_macro(&self, name: &str, source: &str) -> Result<Vec<u8>, ProjectionError> {
        if !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(ProjectionError {
                message: "projection macro probe name must be a Rust identifier".to_owned(),
            });
        }
        let probe = self.workspace.join("src").join(format!("{name}.rs"));
        write_if_changed(&probe, source.as_bytes())?;
        let output = cargo_output(
            self.workspace,
            &[
                "rustdoc",
                "--lib",
                "--offline",
                "--frozen",
                "--",
                "-Z",
                "unstable-options",
                "--output-format",
                "json",
            ],
            self.containment,
        )?;
        if !output.status.success() {
            return Err(ProjectionError {
                message: format!(
                    "projection macro probe failed: {}",
                    String::from_utf8_lossy(&output.stderr).trim()
                ),
            });
        }
        fs::read(
            self.workspace
                .join("target/doc/terrane_dependency_projection.json"),
        )
        .map_err(io_error("read macro expansion rustdoc JSON"))
    }
}

fn classify_probe_answers(
    output: &std::process::Output,
    names: &BTreeMap<String, usize>,
    count: usize,
) -> Vec<ProbeAnswer> {
    let unclassified =
        "compiler emitted neither a successful artifact nor a probe-local diagnostic";
    let mut answers = vec![
        ProbeAnswer::Unknown {
            reason: unclassified.to_owned(),
        };
        count
    ];
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        if message["reason"] == "compiler-artifact" {
            if let Some(index) = message["target"]["name"]
                .as_str()
                .and_then(|name| names.get(name))
                .copied()
            {
                answers[index] = ProbeAnswer::Yes;
            }
            continue;
        }
        if message["reason"] != "compiler-message" || message["message"]["level"] != "error" {
            continue;
        }
        let code = message["message"]["code"]["code"].as_str();
        let rendered = message["message"]["rendered"]
            .as_str()
            .unwrap_or("probe compilation failed")
            .trim()
            .to_owned();
        for span in message["message"]["spans"].as_array().into_iter().flatten() {
            let Some(file_name) = span["file_name"].as_str() else {
                continue;
            };
            let Some(stem) = Path::new(file_name)
                .file_stem()
                .and_then(|stem| stem.to_str())
            else {
                continue;
            };
            let Some(index) = names.get(stem).copied() else {
                continue;
            };
            answers[index] = if code == Some("E0277") {
                ProbeAnswer::No
            } else {
                ProbeAnswer::Unknown {
                    reason: rendered.clone(),
                }
            };
        }
    }
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        if !stderr.is_empty() {
            for answer in &mut answers {
                if matches!(answer, ProbeAnswer::Unknown { reason } if reason == unclassified) {
                    *answer = ProbeAnswer::Unknown {
                        reason: stderr.clone(),
                    };
                }
            }
        }
    }
    answers
}

fn batch_identity(identity: &str, questions: &[BoundQuestion]) -> Result<String, ProjectionError> {
    let encoded = serde_json::to_vec(questions).map_err(|error| ProjectionError {
        message: format!("cannot encode projection probe identity: {error}"),
    })?;
    let mut hash = Sha256::new();
    hash.update(identity.as_bytes());
    hash.update([0]);
    hash.update(encoded);
    Ok(format!("{:x}", hash.finalize()))
}

fn call_batch_identity(
    identity: &str,
    questions: &[CallQuestion],
) -> Result<String, ProjectionError> {
    let encoded = serde_json::to_vec(questions).map_err(|error| ProjectionError {
        message: format!("cannot encode projection call probe identity: {error}"),
    })?;
    let mut hash = Sha256::new();
    hash.update(identity.as_bytes());
    hash.update(b"calls");
    hash.update(encoded);
    Ok(format!("{:x}", hash.finalize()))
}

fn cargo_output(
    directory: &Path,
    arguments: &[&str],
    containment: Containment,
) -> Result<std::process::Output, ProjectionError> {
    let sandboxed = containment == Containment::Enforced;
    let canonical = sandboxed
        .then(|| directory.canonicalize())
        .transpose()
        .map_err(io_error("canonicalize projection probe workspace"))?;
    let working_directory = canonical.as_deref().unwrap_or(directory);
    let mut command = if sandboxed {
        let mut command = Command::new("bwrap");
        command.args([
            "--die-with-parent",
            "--unshare-all",
            "--ro-bind",
            "/",
            "/",
            "--dev",
            "/dev",
            "--proc",
            "/proc",
            "--tmpfs",
            "/tmp",
            "--bind",
        ]);
        command
            .arg(working_directory)
            .arg(working_directory)
            .arg("--")
            .arg("cargo");
        command
    } else {
        Command::new("cargo")
    };
    crate::cargo_toolchain::configure_cargo_command(&mut command);
    command
        .arg(format!("+{}", crate::RUSTDOC_TOOLCHAIN))
        .args(arguments)
        .current_dir(working_directory)
        .output()
        .map_err(|error| ProjectionError {
            message: format!("cannot run projection probe: {error}"),
        })
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<(), ProjectionError> {
    if fs::read(path).ok().as_deref() == Some(bytes) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error("create projection probe path"))?;
    }
    fs::write(path, bytes).map_err(io_error("write projection probe"))
}

fn io_error(context: &'static str) -> impl Fn(std::io::Error) -> ProjectionError {
    move |error| ProjectionError {
        message: format!("{context}: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    use rustdoc_types::{Crate as RustdocCrate, ItemEnum};

    use super::{BoundQuestion, CallQuestion, ImplQuestion, ProbeAnswer, ProjectionOracle};
    use crate::projection::Containment;

    fn workspace(name: &str) -> std::path::PathBuf {
        workspace_with_dependencies(name, "")
    }

    fn workspace_with_dependencies(name: &str, dependencies: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("terrane-oracle-{name}-{nonce}"));
        fs::create_dir_all(path.join("src")).unwrap();
        fs::write(
            path.join("Cargo.toml"),
            format!(
                "[package]\nname = \"terrane_dependency_projection\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[dependencies]\n{dependencies}\n[workspace]\n"
            ),
        )
        .unwrap();
        fs::write(path.join("src/lib.rs"), "").unwrap();
        Command::new("cargo")
            .arg(format!("+{}", crate::RUSTDOC_TOOLCHAIN))
            .args(["generate-lockfile", "--offline"])
            .current_dir(&path)
            .status()
            .unwrap();
        path
    }

    #[test]
    fn bound_probe_distinguishes_yes_no_and_unknown_and_caches() {
        let workspace = workspace("bounds");
        let oracle = ProjectionOracle::new(&workspace, "identity", Containment::Unavailable);
        let questions = vec![
            BoundQuestion {
                rust_type: "Vec<u8>".to_owned(),
                rust_bound: "IntoIterator<Item = u8>".to_owned(),
            },
            BoundQuestion {
                rust_type: "String".to_owned(),
                rust_bound: "Copy".to_owned(),
            },
            BoundQuestion {
                rust_type: "missing::Type".to_owned(),
                rust_bound: "Send".to_owned(),
            },
        ];

        let first = oracle.prove_bounds(&questions).unwrap();
        assert_eq!(first.compiled_probe_count, 3);
        assert!(
            first
                .evidence
                .iter()
                .any(|item| item.answer == ProbeAnswer::Yes)
        );
        assert!(
            first
                .evidence
                .iter()
                .any(|item| item.answer == ProbeAnswer::No)
        );
        assert!(
            first
                .evidence
                .iter()
                .any(|item| matches!(item.answer, ProbeAnswer::Unknown { .. }))
        );

        let second = oracle.prove_bounds(&questions).unwrap();
        assert_eq!(second, first);
        fs::remove_dir_all(workspace).unwrap();
    }

    #[test]
    fn later_batch_uses_positive_artifacts_and_clears_stale_bins() {
        let workspace = workspace("success-artifacts");
        let oracle = ProjectionOracle::new(&workspace, "identity", Containment::Unavailable);
        let first = oracle
            .prove_bounds(&[
                BoundQuestion {
                    rust_type: "Vec<u8>".to_owned(),
                    rust_bound: "IntoIterator<Item = u8>".to_owned(),
                },
                BoundQuestion {
                    rust_type: "String".to_owned(),
                    rust_bound: "Copy".to_owned(),
                },
            ])
            .unwrap();
        assert!(
            first
                .evidence
                .iter()
                .any(|evidence| evidence.answer == ProbeAnswer::No)
        );

        let second = oracle
            .prove_bounds(&[BoundQuestion {
                rust_type: "Vec<u16>".to_owned(),
                rust_bound: "IntoIterator<Item = u16>".to_owned(),
            }])
            .unwrap();
        assert_eq!(second.evidence[0].answer, ProbeAnswer::Yes);
        assert_eq!(fs::read_dir(workspace.join("src/bin")).unwrap().count(), 1);
        fs::remove_dir_all(workspace).unwrap();
    }

    #[test]
    fn impl_probe_compiles_complete_trait_implementation_shape() {
        let workspace = workspace("impl-shape");
        let oracle = ProjectionOracle::new(&workspace, "identity", Containment::Unavailable);
        let report = oracle
            .prove_impls(&[ImplQuestion {
                label: "Display for Local".to_owned(),
                source: "struct Local;\nimpl std::fmt::Display for Local {\n    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        formatter.write_str(\"local\")\n    }\n}\nfn main() { let _ = Local.to_string(); }\n".to_owned(),
            }])
            .unwrap();

        assert_eq!(report.compiled_probe_count, 1);
        assert_eq!(report.evidence[0].answer, ProbeAnswer::Yes);
        fs::remove_dir_all(workspace).unwrap();
    }

    #[test]
    fn exact_sqlx_row_accessor_call_compiles_against_resolved_dependency() {
        let workspace = workspace_with_dependencies(
            "sqlx-row-accessor",
            "sqlx = { version = \"=0.8.6\", default-features = false, features = [\"postgres\", \"runtime-tokio-rustls\"] }\n",
        );
        let oracle = ProjectionOracle::new(&workspace, "sqlx-0.8.6", Containment::Unavailable);
        let report = oracle
            .prove_calls(&[CallQuestion {
                label: "sqlx::Row::try_get::<String, _>".to_owned(),
                source: "use sqlx::Row as _;\nfn read_name(row: &sqlx::postgres::PgRow) -> Result<String, sqlx::Error> {\n    row.try_get::<String, _>(\"name\")\n}\nfn main() { let _ = read_name; }\n".to_owned(),
            }])
            .unwrap();

        assert_eq!(report.compiled_probe_count, 1);
        assert_eq!(report.evidence[0].answer, ProbeAnswer::Yes);
        fs::remove_dir_all(workspace).unwrap();
    }

    #[test]
    fn macro_probe_recovers_expanded_public_api() {
        let workspace = workspace("macro");
        let oracle = ProjectionOracle::new(&workspace, "identity", Containment::Unavailable);
        let bytes = oracle
            .expand_macro(
                "lib",
                "macro_rules! make_public { () => { pub struct Generated; } }\nmake_public!();\n",
            )
            .unwrap();
        let document: RustdocCrate = serde_json::from_slice(&bytes).unwrap();
        assert!(document.index.values().any(|item| {
            item.name.as_deref() == Some("Generated") && matches!(item.inner, ItemEnum::Struct(_))
        }));
        fs::remove_dir_all(workspace).unwrap();
    }
}
