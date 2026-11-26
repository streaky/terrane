use crate::development_fingerprint::compiler_input_fingerprint;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const BUILD_REPOSITORY: &str = env!("TERRANE_BUILD_REPOSITORY");
const BUILD_INPUT_FINGERPRINT: &str = env!("TERRANE_BUILD_INPUT_FINGERPRINT");
const STALE_COMPILER_EXIT: u8 = 6;

pub fn check_before_dispatch() -> Option<ExitCode> {
    let executable = std::env::current_exe().ok()?.canonicalize().ok()?;
    let repository = Path::new(BUILD_REPOSITORY);
    if !is_repository_target_binary(&executable, repository) {
        return None;
    }

    let rebuild_command = rebuild_command(&executable);
    let build_fingerprint = u64::from_str_radix(BUILD_INPUT_FINGERPRINT, 16).ok();
    let current_fingerprint = compiler_input_fingerprint(repository);
    match (build_fingerprint, current_fingerprint) {
        (Some(build), Ok(current)) if build == current => return None,
        (Some(_), Ok(_)) => {
            let strict = requires_current_compiler();
            let severity = if strict { "error" } else { "warning" };
            eprintln!(
                "{severity}: this development Terrane compiler predates compiler input changes\n  binary: {}\n  help: rebuild this binary with `{rebuild_command}`",
                executable.display(),
            );
            return strict.then_some(ExitCode::from(STALE_COMPILER_EXIT));
        }
        _ => eprintln!(
            "warning: Terrane could not verify whether this development compiler is current\n  binary: {}\n  help: rebuild this binary with `{rebuild_command}`",
            executable.display(),
        ),
    }
    None
}

fn requires_current_compiler() -> bool {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| std::env::home_dir().map(|home| home.join(".config")))
        .is_some_and(|config| config.join("terrane/require-current-compiler").is_file())
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
    fn binary_detection_accepts_profiles_and_triples() {
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
