#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt as _;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use terrane_compiler::{BUILD_TOOLCHAIN, RUSTDOC_TOOLCHAIN};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

// Only these read and write operations are available. In offline mode any
// attempted mutation is recorded and rejected even if Terrane ignores the error.
const RUSTUP: &str = r#"#!/bin/sh
set -eu
cd "$FAKE_RUSTUP_STATE"
printf '%s\n' "$*" >> calls

unexpected() {
    printf '%s\n' "$*" >> unexpected
    printf 'unexpected rustup invocation: %s\n' "$*" >&2
    exit 97
}

[ "${RUSTUP_AUTO_INSTALL-}" = 0 ] || unexpected "missing RUSTUP_AUTO_INSTALL=0" "$@"

mutate() {
    printf '%s\n' "$1" >> mutations
    if [ "$FAKE_RUSTUP_MODE" = offline ]; then
        printf 'rustup mutation attempted offline\n' >&2
        exit 98
    fi
    if [ "$FAKE_RUSTUP_MODE" = fail-write ]; then
        printf 'fixture: toolchain download failed\n' >&2
        exit 43
    fi
}

slot() {
    case "$1" in
        "$FAKE_STABLE") printf 'stable' ;;
        "$FAKE_NIGHTLY") printf 'nightly' ;;
        *) unexpected "$@" ;;
    esac
}

case "${1-} ${2-}" in
    'show ')
        printf 'Default host: %s\nrustup home: %s\n\n' "$FAKE_HOST" "$RUSTUP_HOME"
        ;;
    'toolchain list')
        [ "$#" -eq 2 ] || unexpected "$@"
        if [ "$FAKE_RUSTUP_MODE" = fail-read ]; then
            printf 'fixture: cannot read toolchain inventory\n' >&2
            exit 42
        fi
        printf 'stable-%s (default)\nnightly-%s\n%s-other-host\n%s-other-host\n' \
            "$FAKE_HOST" "$FAKE_HOST" "$FAKE_STABLE" "$FAKE_NIGHTLY"
        if [ -f stable ]; then printf '%s-%s\n' "$FAKE_STABLE" "$FAKE_HOST"; fi
        if [ -f nightly ]; then printf '%s-%s\n' "$FAKE_NIGHTLY" "$FAKE_HOST"; fi
        ;;
    'component list')
        [ "$#" -eq 5 ] && [ "$3 $4" = '--installed --toolchain' ] || unexpected "$@"
        toolchain=$(slot "$5")
        [ -f "$toolchain" ] || unexpected "$@"
        for component in cargo rustc rust-std; do
            if [ ! -f "$toolchain-missing-$component" ] || [ -f "$toolchain-added-$component" ]; then
                printf '%s-%s (installed)\n' "$component" "$FAKE_HOST"
            elif [ "$component" = rust-std ]; then
                printf 'rust-std-other-host (installed)\n'
            fi
        done
        if [ "$toolchain" = nightly ] && [ -f rust-src ]; then printf 'rust-src (installed)\n'; fi
        ;;
    'run --install')
        [ "$#" -eq 5 ] && [ "$4 $5" = 'rustc --version' ] || unexpected "$@"
        toolchain=$(slot "$3")
        mutate "install $3"
        : > "$toolchain"
        printf 'rustc %s\n' "$3"
        ;;
    'component add')
        [ "$#" -eq 5 ] && [ "$4" = --toolchain ] || unexpected "$@"
        toolchain=$(slot "$5")
        [ -f "$toolchain" ] || unexpected "$@"
        case "$3" in
            cargo|rustc|rust-std)
                [ -f "$toolchain-missing-$3" ] || unexpected "$@"
                mutate "add $3 to $5"
                : > "$toolchain-added-$3"
                ;;
            rust-src)
                [ "$toolchain" = nightly ] || unexpected "$@"
                mutate "add rust-src to $5"
                : > rust-src
                ;;
            *) unexpected "$@" ;;
        esac
        ;;
    *) unexpected "$@" ;;
esac
"#;

struct FakeRustup(PathBuf);

impl FakeRustup {
    fn new(installed: &[&str]) -> Self {
        let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "terrane-cli-toolchain-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir(&root).unwrap();
        let fixture = Self(root);
        fs::create_dir(fixture.0.join("bin")).unwrap();
        fs::create_dir(fixture.0.join("home")).unwrap();
        fs::create_dir(fixture.rustup_home()).unwrap();
        let executable = fixture.0.join("bin/rustup");
        fs::write(&executable, RUSTUP).unwrap();
        fs::set_permissions(executable, fs::Permissions::from_mode(0o755)).unwrap();
        for member in installed {
            fs::write(fixture.0.join(member), "").unwrap();
        }
        fixture
    }

    fn run(&self, arguments: &[&str], mode: &str) -> Output {
        let output = Command::new(env!("CARGO_BIN_EXE_terrane"))
            .arg("toolchain")
            .args(arguments)
            .current_dir(&self.0)
            .env_clear()
            .env("PATH", self.0.join("bin"))
            .env("HOME", self.0.join("home"))
            .env("CARGO_HOME", self.0.join("home/.cargo"))
            .env("RUSTUP_HOME", self.rustup_home())
            .env("FAKE_RUSTUP_STATE", &self.0)
            .env("FAKE_STABLE", BUILD_TOOLCHAIN)
            .env("FAKE_NIGHTLY", RUSTDOC_TOOLCHAIN)
            .env("FAKE_HOST", "x86_64-unknown-linux-gnu")
            .env("FAKE_RUSTUP_MODE", mode)
            .output()
            .unwrap();
        assert!(
            !self.0.join("unexpected").exists(),
            "unexpected calls: {}\n{output:?}",
            self.log("unexpected")
        );
        output
    }

    fn log(&self, name: &str) -> String {
        match fs::read_to_string(self.0.join(name)) {
            Ok(log) => log,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(error) => panic!("could not read {name}: {error}"),
        }
    }

    fn rustup_home(&self) -> PathBuf {
        self.0.join("home/.rustup")
    }
}

impl Drop for FakeRustup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn diagnostic(output: &Output) -> String {
    format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn check_does_not_initialize_an_absent_rustup_home() {
    let rustup = FakeRustup::new(&[]);
    fs::remove_dir(rustup.rustup_home()).unwrap();
    let missing = rustup.run(&["check"], "offline");
    let report = diagnostic(&missing);
    assert!(!missing.status.success(), "{report}");
    assert!(report.contains(BUILD_TOOLCHAIN), "{report}");
    assert!(report.contains(RUSTDOC_TOOLCHAIN), "{report}");
    assert!(rustup.log("calls").is_empty());
    assert!(!rustup.rustup_home().exists());
}

#[test]
fn fresh_check_is_read_only_and_install_provisions_the_exact_pins() {
    let rustup = FakeRustup::new(&[]);
    let missing = rustup.run(&["check"], "offline");
    let report = diagnostic(&missing);
    assert!(!missing.status.success(), "{report}");
    assert!(report.contains(BUILD_TOOLCHAIN), "{report}");
    assert!(report.contains(RUSTDOC_TOOLCHAIN), "{report}");
    assert!(rustup.log("mutations").is_empty());

    let installed = rustup.run(&["install"], "online");
    assert!(installed.status.success(), "{}", diagnostic(&installed));
    let mutations = rustup.log("mutations");
    let mut mutations: Vec<_> = mutations.lines().collect();
    mutations.sort_unstable();
    assert_eq!(
        mutations,
        [
            format!("add rust-src to {RUSTDOC_TOOLCHAIN}"),
            format!("install {BUILD_TOOLCHAIN}"),
            format!("install {RUSTDOC_TOOLCHAIN}"),
        ]
    );
    let checked = rustup.run(&["check"], "offline");
    assert!(checked.status.success(), "{}", diagnostic(&checked));
}

#[test]
fn partial_install_adds_only_the_missing_component_and_toolchain() {
    let rustup = FakeRustup::new(&["nightly"]);
    let missing = rustup.run(&["check"], "offline");
    let report = diagnostic(&missing);
    assert!(!missing.status.success(), "{report}");
    assert!(report.contains(BUILD_TOOLCHAIN), "{report}");
    assert!(report.contains("rust-src"), "{report}");
    assert!(rustup.log("mutations").is_empty());

    let installed = rustup.run(&["install"], "online");
    assert!(installed.status.success(), "{}", diagnostic(&installed));
    let mutations = rustup.log("mutations");
    let mut mutations: Vec<_> = mutations.lines().collect();
    mutations.sort_unstable();
    assert_eq!(
        mutations,
        [
            format!("add rust-src to {RUSTDOC_TOOLCHAIN}"),
            format!("install {BUILD_TOOLCHAIN}"),
        ]
    );
}

#[test]
fn partial_install_repairs_each_missing_host_component_without_reinstalling() {
    for (slot, pin) in [("stable", BUILD_TOOLCHAIN), ("nightly", RUSTDOC_TOOLCHAIN)] {
        for component in ["cargo", "rustc", "rust-std"] {
            let rustup = FakeRustup::new(&[
                "stable",
                "nightly",
                "rust-src",
                &format!("{slot}-missing-{component}"),
            ]);
            let checked = rustup.run(&["check"], "offline");
            let report = diagnostic(&checked);
            assert!(!checked.status.success(), "{report}");
            assert!(
                report.contains(pin) && report.contains(component),
                "{report}"
            );
            assert!(rustup.log("mutations").is_empty());

            let installed = rustup.run(&["install"], "online");
            assert!(installed.status.success(), "{}", diagnostic(&installed));
            assert_eq!(
                rustup.log("mutations"),
                format!("add {component} to {pin}\n")
            );
            let checked = rustup.run(&["check"], "offline");
            assert!(checked.status.success(), "{}", diagnostic(&checked));
        }
    }
}

#[test]
fn complete_check_and_repeated_installs_need_no_mutating_rustup_commands() {
    let rustup = FakeRustup::new(&["stable", "nightly", "rust-src"]);
    for action in ["check", "install", "install"] {
        let output = rustup.run(&[action], "offline");
        assert!(output.status.success(), "{action}: {}", diagnostic(&output));
    }
    assert!(rustup.log("mutations").is_empty());
}

#[test]
fn rustup_inventory_and_install_errors_surface_to_the_caller() {
    let rustup = FakeRustup::new(&[]);
    for action in ["check", "install"] {
        let output = rustup.run(&[action], "fail-read");
        let report = diagnostic(&output);
        assert!(!output.status.success(), "{action}: {report}");
        assert!(
            report.contains("cannot read toolchain inventory"),
            "{report}"
        );
    }
    assert!(rustup.log("mutations").is_empty());

    let output = rustup.run(&["install"], "fail-write");
    let report = diagnostic(&output);
    assert!(!output.status.success(), "{report}");
    assert!(report.contains("toolchain download failed"), "{report}");
    assert_eq!(rustup.log("mutations").lines().count(), 1);
}

#[test]
fn invalid_toolchain_usage_never_invokes_rustup() {
    let rustup = FakeRustup::new(&[]);
    for arguments in [
        &[][..],
        &["update"],
        &["check", "extra"],
        &["install", "--force"],
    ] {
        let output = rustup.run(arguments, "offline");
        let report = diagnostic(&output);
        assert_eq!(output.status.code(), Some(2), "{arguments:?}: {report}");
        assert!(
            report.contains("usage:") && report.contains("toolchain"),
            "{report}"
        );
    }
    assert!(rustup.log("calls").is_empty());
}
