use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::Package;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArtifactProfile {
    pub id: &'static str,
    pub optimization: &'static str,
    pub cargo_debug: &'static str,
    pub debug_information: &'static str,
    pub stripping: &'static str,
    pub inlining: &'static str,
    pub lto: &'static str,
    pub codegen_units: u32,
    pub panic: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildIdentity {
    pub target: String,
    pub rust_sysroot: String,
    pub rustc_release: String,
    pub artifact_profile: ArtifactProfile,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InputIdentity {
    pub path: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NativeModuleIdentity {
    pub file_name: String,
    pub content_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RelocationMapping {
    pub build_root: String,
    pub source_root: String,
}

/// Compiler-owned identity shared by debugger and profiler artifacts.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuildProvenance {
    pub compiler_version: String,
    pub rust_toolchain: String,
    pub target: String,
    pub rust_sysroot: String,
    pub rustc_release: String,
    pub abi_recipe: String,
    pub artifact_profile: String,
    pub optimization: String,
    pub debug_information: String,
    pub inlining: String,
    pub stripping: String,
    #[serde(default = "default_lto")]
    pub lto: String,
    #[serde(default = "default_codegen_units")]
    pub codegen_units: u32,
    #[serde(default = "default_panic")]
    pub panic: String,
    pub inputs: Vec<InputIdentity>,
    pub native_module: NativeModuleIdentity,
    pub relocation: RelocationMapping,
}

fn default_lto() -> String {
    "off".to_owned()
}

fn default_codegen_units() -> u32 {
    256
}

fn default_panic() -> String {
    "package-policy".to_owned()
}

impl BuildProvenance {
    /// Binds compiler metadata to one exact native module and its build/source roots.
    ///
    /// # Errors
    ///
    /// Returns an error when roots cannot be canonicalized or the executable cannot be read.
    pub fn create(
        package: &Package,
        executable: &Path,
        build_root: &Path,
        build: BuildIdentity,
    ) -> Result<Self, String> {
        let BuildIdentity {
            target,
            rust_sysroot,
            rustc_release,
            artifact_profile,
        } = build;
        let build_root = std::fs::canonicalize(build_root).map_err(|error| {
            format!(
                "cannot canonicalize build root {}: {error}",
                build_root.display()
            )
        })?;
        let source_root = std::fs::canonicalize(&package.root).map_err(|error| {
            format!(
                "cannot canonicalize source root {}: {error}",
                package.root.display()
            )
        })?;
        let executable_bytes = std::fs::read(executable)
            .map_err(|error| format!("cannot read executable {}: {error}", executable.display()))?;
        let mut inputs = Vec::new();
        for name in [
            crate::MANIFEST_FILE_NAME,
            "terrane-dependencies.lock",
            "terrane-projection.lock",
        ] {
            let path = package.root.join(name);
            if let Ok(bytes) = std::fs::read(path) {
                inputs.push(InputIdentity {
                    path: name.to_owned(),
                    content_hash: hash_bytes(&bytes),
                });
            }
        }
        Ok(Self {
            compiler_version: crate::VERSION.to_owned(),
            rust_toolchain: match package.build_toolchain {
                crate::BuildToolchain::Pinned => crate::BUILD_TOOLCHAIN.to_owned(),
                crate::BuildToolchain::System => "system".to_owned(),
            },
            abi_recipe: abi_recipe_for_toolchain(&target, &rustc_release),
            target,
            rust_sysroot,
            rustc_release,
            artifact_profile: artifact_profile.id.to_owned(),
            optimization: artifact_profile.optimization.to_owned(),
            debug_information: artifact_profile.debug_information.to_owned(),
            inlining: artifact_profile.inlining.to_owned(),
            stripping: artifact_profile.stripping.to_owned(),
            lto: artifact_profile.lto.to_owned(),
            codegen_units: artifact_profile.codegen_units,
            panic: artifact_profile.panic.to_owned(),
            inputs,
            native_module: NativeModuleIdentity {
                file_name: executable
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned(),
                content_hash: hash_bytes(&executable_bytes),
            },
            relocation: RelocationMapping {
                build_root: build_root.to_string_lossy().into_owned(),
                source_root: source_root.to_string_lossy().into_owned(),
            },
        })
    }

    /// Confirms that an executable is the exact module named by this provenance.
    ///
    /// # Errors
    ///
    /// Returns an error when the executable cannot be read or its identity differs.
    pub fn validate_executable(&self, executable: &Path) -> Result<(), String> {
        let bytes = std::fs::read(executable)
            .map_err(|error| format!("cannot read executable {}: {error}", executable.display()))?;
        let actual = hash_bytes(&bytes);
        if actual != self.native_module.content_hash {
            return Err(format!(
                "provenance does not match executable {} (expected {}, found {})",
                executable.display(),
                self.native_module.content_hash,
                actual
            ));
        }
        Ok(())
    }
}

#[must_use]
pub fn abi_recipe_for_toolchain(target: &str, rustc_release: &str) -> String {
    match target {
        "x86_64-unknown-linux-gnu" => format!(
            "terrane-rust-x86_64-linux-gnu-v2@{}",
            hash_bytes(rustc_release.as_bytes())
        ),
        _ => "unsupported".to_owned(),
    }
}

#[must_use]
pub fn hash_bytes(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
