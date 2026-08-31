use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Makes Cargo use `sccache` whenever an executable is available on `PATH`.
///
/// An absolute wrapper path makes the decision independent of inherited Cargo
/// configuration and remains valid for contained Cargo processes.
pub fn configure_cargo_command(command: &mut Command) {
    configure_cargo_command_from_path(command, std::env::var_os("PATH").as_deref());
}

fn configure_cargo_command_from_path(command: &mut Command, path: Option<&OsStr>) {
    if let Some(sccache) = sccache_on_path(path) {
        command.env("RUSTC_WRAPPER", sccache);
    }
}

fn sccache_on_path(path: Option<&OsStr>) -> Option<PathBuf> {
    let executable = format!("sccache{}", std::env::consts::EXE_SUFFIX);
    std::env::split_paths(path?).find_map(|directory| {
        let candidate = directory.join(&executable);
        is_executable(&candidate)
            .then(|| candidate.canonicalize().ok())
            .flatten()
    })
}

fn is_executable(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_executable_sccache_and_ignores_non_executable_files() {
        let root =
            std::env::temp_dir().join(format!("terrane-sccache-path-{}", std::process::id()));
        let unavailable = root.join("unavailable");
        let available = root.join("available");
        fs::create_dir_all(&unavailable).unwrap();
        fs::create_dir_all(&available).unwrap();
        let executable_name = format!("sccache{}", std::env::consts::EXE_SUFFIX);
        fs::write(unavailable.join(&executable_name), []).unwrap();
        let expected = available.join(&executable_name);
        fs::write(&expected, []).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            fs::set_permissions(&expected, fs::Permissions::from_mode(0o755)).unwrap();
        }

        let path = std::env::join_paths([&unavailable, &available]).unwrap();
        assert_eq!(sccache_on_path(Some(&path)), Some(expected));
        assert_eq!(sccache_on_path(None), None);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn configures_an_absolute_rustc_wrapper_only_when_available() {
        let root =
            std::env::temp_dir().join(format!("terrane-sccache-command-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let expected = root.join(format!("sccache{}", std::env::consts::EXE_SUFFIX));
        fs::write(&expected, []).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            fs::set_permissions(&expected, fs::Permissions::from_mode(0o755)).unwrap();
        }

        let mut command = Command::new("cargo");
        let path = std::env::join_paths([&root]).unwrap();
        configure_cargo_command_from_path(&mut command, Some(&path));
        assert!(command.get_envs().any(|(key, value)| {
            key == "RUSTC_WRAPPER" && value.is_some_and(|value| value == expected.as_os_str())
        }));

        let mut command = Command::new("cargo");
        configure_cargo_command_from_path(&mut command, None);
        assert!(!command.get_envs().any(|(key, _)| key == "RUSTC_WRAPPER"));

        fs::remove_dir_all(root).unwrap();
    }
}
