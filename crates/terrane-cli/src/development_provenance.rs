use crate::development_fingerprint::compiler_input_fingerprint;
use std::path::Path;

const BUILD_REPOSITORY: &str = env!("TERRANE_BUILD_REPOSITORY");
const BUILD_INPUT_FINGERPRINT: &str = env!("TERRANE_BUILD_INPUT_FINGERPRINT");

pub fn warning() -> Option<String> {
    let executable = std::env::current_exe().ok()?.canonicalize().ok()?;
    let repository = Path::new(BUILD_REPOSITORY);
    if !is_repository_target_binary(&executable, repository) {
        return None;
    }

    let rebuild_command = rebuild_command(&executable);
    let build_fingerprint = u64::from_str_radix(BUILD_INPUT_FINGERPRINT, 16).ok();
    let current_fingerprint = compiler_input_fingerprint(repository);
    match (build_fingerprint, current_fingerprint) {
        (Some(build), Ok(current)) if build == current => None,
        (Some(_), Ok(_)) => Some(format!(
            "warning: this development Terrane compiler predates compiler input changes\n  binary: {}\n  help: rebuild this binary with `{rebuild_command}`\n",
            executable.display(),
        )),
        _ => Some(format!(
            "warning: Terrane could not verify whether this development compiler is current\n  binary: {}\n  help: rebuild this binary with `{rebuild_command}`\n",
            executable.display(),
        )),
    }
}

fn is_repository_target_binary(executable: &Path, repository: &Path) -> bool {
    let Ok(relative) = executable.strip_prefix(repository.join("target")) else {
        return false;
    };
    relative.components().count() >= 2
        && executable.file_stem().is_some_and(|name| name == "terrane")
}

fn rebuild_command(executable: &Path) -> &'static str {
    if executable
        .components()
        .any(|component| component.as_os_str() == "release")
    {
        "cargo build --release --bin terrane"
    } else {
        "cargo build --bin terrane"
    }
}

#[cfg(test)]
mod tests {
    use super::{is_repository_target_binary, rebuild_command};
    use std::path::Path;

    #[test]
    fn development_binary_detection_accepts_profiles_and_target_triples() {
        let root = Path::new("/workspace/terrane");
        assert!(is_repository_target_binary(
            Path::new("/workspace/terrane/target/release/terrane"),
            root,
        ));
        assert!(is_repository_target_binary(
            Path::new("/workspace/terrane/target/x86_64-unknown-linux-gnu/custom/terrane"),
            root,
        ));
        assert!(!is_repository_target_binary(
            Path::new("/usr/local/bin/terrane"),
            root,
        ));
    }

    #[test]
    fn rebuild_advice_matches_the_binary_profile() {
        assert_eq!(
            rebuild_command(Path::new("/workspace/terrane/target/debug/terrane")),
            "cargo build --bin terrane"
        );
        assert_eq!(
            rebuild_command(Path::new("/workspace/terrane/target/release/terrane")),
            "cargo build --release --bin terrane"
        );
    }
}
