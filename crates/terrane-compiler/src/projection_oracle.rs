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
        let mut answers = vec![ProbeAnswer::Yes; questions.len()];
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let Ok(message) = serde_json::from_str::<serde_json::Value>(line) else {
                continue;
            };
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
        if !output.status.success() && answers.iter().all(|answer| answer == &ProbeAnswer::Yes) {
            let reason = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            answers.fill(ProbeAnswer::Unknown {
                reason: if reason.is_empty() {
                    "Cargo probe failed without a compiler diagnostic".to_owned()
                } else {
                    reason
                },
            });
        }
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
    use std::time::{SystemTime, UNIX_EPOCH};

    use rustdoc_types::{Crate as RustdocCrate, ItemEnum};

    use super::{BoundQuestion, ProbeAnswer, ProjectionOracle};
    use crate::projection::Containment;

    fn workspace(name: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("terrane-oracle-{name}-{nonce}"));
        fs::create_dir_all(path.join("src")).unwrap();
        fs::write(
            path.join("Cargo.toml"),
            "[package]\nname = \"terrane_dependency_projection\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n",
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

    use std::process::Command;

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
