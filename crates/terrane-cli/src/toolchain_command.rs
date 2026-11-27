use std::env;
use std::ffi::OsString;
use std::io;
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};

use terrane_compiler::{BUILD_TOOLCHAIN, RUSTDOC_TOOLCHAIN};

use crate::CliFailure;

const USAGE: &str = concat!(
    "usage: terrane toolchain <check|install>\n\n",
    "  check    report whether the pinned Rust toolchains and components are installed (offline)\n",
    "  install  install only missing pinned Rust toolchains and components\n",
);
const INSTALL_HINT: &str =
    "run `terrane toolchain install` to install the pinned Rust prerequisites";
const BUILD_COMPONENTS: &[&str] = &["cargo", "rustc", "rust-std"];
const PROJECTION_COMPONENTS: &[&str] = &["cargo", "rustc", "rust-std", "rust-src"];

pub(super) fn run(arguments: &[OsString]) -> Result<ExitCode, CliFailure> {
    let is_help = |arg: &OsString| arg == "--help" || arg == "-h";
    let install = match arguments {
        [arg] if is_help(arg) => {
            print!("{USAGE}");
            return Ok(ExitCode::SUCCESS);
        }
        [action, help] if (action == "check" || action == "install") && is_help(help) => {
            print!("{USAGE}");
            return Ok(ExitCode::SUCCESS);
        }
        [action] if action == "check" => false,
        [action] if action == "install" => true,
        _ => {
            return Err(CliFailure {
                code: 2,
                message: USAGE.to_owned(),
            });
        }
    };

    let Some(inventory) = Inventory::read(install)? else {
        return Err(failure(format!(
            "Rustup home does not exist; neither {BUILD_TOOLCHAIN} nor {RUSTDOC_TOOLCHAIN} is installed; {INSTALL_HINT}"
        )));
    };
    let host = &inventory.host;
    let mut incomplete = false;
    for (pin, required) in [
        (BUILD_TOOLCHAIN, BUILD_COMPONENTS),
        (RUSTDOC_TOOLCHAIN, PROJECTION_COMPONENTS),
    ] {
        if !inventory.contains(pin) {
            if !install {
                println!("{pin}: not installed ({host})");
                incomplete = true;
                continue;
            }
            // `rustup toolchain install` may set the default if there isn't one.
            // Explicit `run --install` uses the configured profile without changing the default.
            rustup_install(&["run", "--install", pin, "rustc", "--version"])?;
        }

        let mut missing = missing_components(pin, host, required)?;
        if install && !missing.is_empty() {
            for component in &missing {
                rustup_install(&["component", "add", component, "--toolchain", pin])?;
            }
            missing = missing_components(pin, host, required)?;
        }
        if missing.is_empty() {
            println!("{pin}: ready ({host})");
        } else {
            println!("{pin}: missing components: {}", missing.join(", "));
            incomplete = true;
        }
    }

    if incomplete {
        return Err(if install {
            failure(
                "Rust prerequisites are still incomplete after installation; see the statuses above",
            )
        } else {
            failure(format!("missing Rust prerequisites; {INSTALL_HINT}"))
        });
    }
    Ok(ExitCode::SUCCESS)
}

pub(super) fn ensure_projection_toolchain() -> Result<(), CliFailure> {
    let not_installed = || {
        failure(format!(
            "pinned projection Rust toolchain {RUSTDOC_TOOLCHAIN} is not installed; {INSTALL_HINT}"
        ))
    };
    let inventory = Inventory::read(false)?.ok_or_else(not_installed)?;
    if !inventory.contains(RUSTDOC_TOOLCHAIN) {
        return Err(not_installed());
    }
    let missing = missing_components(RUSTDOC_TOOLCHAIN, &inventory.host, PROJECTION_COMPONENTS)?;
    if !missing.is_empty() {
        return Err(failure(format!(
            "pinned projection Rust toolchain {RUSTDOC_TOOLCHAIN} is missing components: {}; {INSTALL_HINT}",
            missing.join(", ")
        )));
    }
    Ok(())
}

struct Inventory {
    host: String,
    toolchains: String,
}

impl Inventory {
    fn read(install: bool) -> Result<Option<Self>, CliFailure> {
        if !install {
            // Even rustup's query commands create a missing home on startup.
            let home = env::var_os("RUSTUP_HOME")
                .filter(|home| !home.is_empty())
                .map(PathBuf::from)
                .or_else(|| env::home_dir().map(|home| home.join(".rustup")))
                .ok_or_else(|| {
                    failure("could not determine rustup home; set RUSTUP_HOME and retry")
                })?;
            let exists = home.try_exists().map_err(|error| {
                failure(format!(
                    "could not inspect rustup home {}: {error}",
                    home.display()
                ))
            })?;
            if !exists {
                return Ok(None);
            }
        }

        let show = rustup_output(&["show"])?;
        let host = show
            .lines()
            .find_map(|line| line.strip_prefix("Default host:").map(str::trim))
            .filter(|host| !host.is_empty())
            .ok_or_else(|| {
                failure("could not determine the default Rust host from `rustup show`; check your rustup setup")
            })?;
        let toolchains = rustup_output(&["toolchain", "list"])?;
        Ok(Some(Self {
            host: host.to_owned(),
            toolchains,
        }))
    }

    fn contains(&self, pin: &str) -> bool {
        let qualified = format!("{pin}-{}", self.host);
        self.toolchains
            .lines()
            .any(|line| line.split_whitespace().next() == Some(qualified.as_str()))
    }
}

fn missing_components(
    pin: &str,
    host: &str,
    required: &[&'static str],
) -> Result<Vec<&'static str>, CliFailure> {
    let installed = rustup_output(&["component", "list", "--installed", "--toolchain", pin])?;
    Ok(required
        .iter()
        .copied()
        .filter(|&component| {
            !installed.lines().any(|line| {
                let Some(name) = line.split_whitespace().next() else {
                    return false;
                };
                name == component
                    || name
                        .strip_prefix(component)
                        .and_then(|suffix| suffix.strip_prefix('-'))
                        == Some(host)
            })
        })
        .collect())
}

fn rustup(args: &[&str]) -> Command {
    let mut command = Command::new("rustup");
    command.args(args).env("RUSTUP_AUTO_INSTALL", "0");
    if args == ["show"] {
        // Older rustup versions ignore RUSTUP_AUTO_INSTALL for `show`. A custom
        // override bypasses the checkout's toolchain file and cannot be downloaded.
        command.env("RUSTUP_TOOLCHAIN", "terrane-inventory-no-active-toolchain");
    }
    command
}

fn rustup_output(args: &[&str]) -> Result<String, CliFailure> {
    let output = rustup(args)
        .output()
        .map_err(|error| spawn_failure(args, &error))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(failure(format!(
            "`rustup {}` failed ({}): {}",
            args.join(" "),
            output.status,
            stderr.trim()
        )));
    }
    String::from_utf8(output.stdout).map_err(|error| {
        failure(format!(
            "could not read output from `rustup {}`: {error}",
            args.join(" ")
        ))
    })
}

fn rustup_install(args: &[&str]) -> Result<(), CliFailure> {
    let status = rustup(args)
        .stdout(Stdio::null())
        .status()
        .map_err(|error| spawn_failure(args, &error))?;
    if status.success() {
        Ok(())
    } else {
        Err(failure(format!(
            "`rustup {}` failed ({status}); see rustup's error above",
            args.join(" ")
        )))
    }
}

fn spawn_failure(args: &[&str], error: &io::Error) -> CliFailure {
    if error.kind() == io::ErrorKind::NotFound {
        failure("rustup was not found on PATH; install it from https://rustup.rs/ and retry")
    } else {
        failure(format!(
            "could not run `rustup {}`: {error}",
            args.join(" ")
        ))
    }
}

fn failure(message: impl std::fmt::Display) -> CliFailure {
    CliFailure {
        code: 4,
        message: format!("error: {message}\n"),
    }
}
