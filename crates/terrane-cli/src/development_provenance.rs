use crate::development_fingerprint::{dirty_fingerprint, git_output};
use std::path::Path;

const BUILD_REPOSITORY: &str = env!("TERRANE_BUILD_REPOSITORY");
const BUILD_GIT_HEAD: &str = env!("TERRANE_BUILD_GIT_HEAD");
const BUILD_DIRTY_FINGERPRINT: &str = env!("TERRANE_BUILD_DIRTY_FINGERPRINT");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Staleness {
    Commit,
    WorkingTree,
}

pub fn warning() -> Option<String> {
    let executable = std::env::current_exe().ok()?.canonicalize().ok()?;
    let repository = Path::new(BUILD_REPOSITORY);
    if !is_repository_target_binary(&executable, repository) || BUILD_GIT_HEAD.is_empty() {
        return None;
    }

    let current_head = git_output(repository, &["rev-parse", "HEAD"])?;
    let current_head = String::from_utf8(current_head).ok()?;
    let current_head = current_head.trim();
    let current_fingerprint = dirty_fingerprint(repository)?;
    let build_fingerprint = u64::from_str_radix(BUILD_DIRTY_FINGERPRINT, 16).ok()?;
    let rebuild_command = rebuild_command(&executable);
    match staleness(
        BUILD_GIT_HEAD,
        build_fingerprint,
        current_head,
        current_fingerprint,
    )? {
        Staleness::Commit => Some(format!(
            "warning: this development Terrane compiler is out of date\n  binary: {}\n  built from: {}\n  checkout:   {}\n  help: rebuild this binary with `{rebuild_command}`\n",
            executable.display(),
            short_commit(BUILD_GIT_HEAD),
            short_commit(current_head),
        )),
        Staleness::WorkingTree => Some(format!(
            "warning: this development Terrane compiler predates compiler input changes in the working tree\n  binary: {}\n  help: rebuild this binary with `{rebuild_command}`\n",
            executable.display(),
        )),
    }
}

fn staleness(
    build_head: &str,
    build_fingerprint: u64,
    current_head: &str,
    current_fingerprint: u64,
) -> Option<Staleness> {
    if build_head != current_head {
        Some(Staleness::Commit)
    } else if build_fingerprint != current_fingerprint {
        Some(Staleness::WorkingTree)
    } else {
        None
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

fn short_commit(commit: &str) -> &str {
    commit.get(..8).unwrap_or(commit)
}

#[cfg(test)]
mod tests {
    use super::{Staleness, is_repository_target_binary, rebuild_command, staleness};
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

    #[test]
    fn provenance_distinguishes_commit_and_working_tree_staleness() {
        assert_eq!(staleness("aaa", 1, "bbb", 1), Some(Staleness::Commit));
        assert_eq!(staleness("aaa", 1, "aaa", 2), Some(Staleness::WorkingTree));
        assert_eq!(staleness("aaa", 1, "aaa", 1), None);
    }
}
