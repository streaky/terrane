use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

use terrane_rust_analysis::Containment;

#[derive(Debug, Eq, PartialEq)]
struct Arguments {
    manifest: PathBuf,
    package: Option<String>,
    target: Option<String>,
    probes: Option<PathBuf>,
    containment: Containment,
    complete_surface: bool,
}

fn main() {
    let arguments = parse_args().unwrap_or_else(|message| usage(&message));
    let probes = arguments
        .probes
        .as_deref()
        .map(read_probes)
        .transpose()
        .unwrap_or_else(|message| usage(&message))
        .unwrap_or_default();
    let report = match arguments.target {
        Some(target) if arguments.complete_surface && probes == terrane_rust_analysis::SurveyProbeRequest::default() => {
            terrane_rust_analysis::survey_complete_package_for_target(
                &arguments.manifest,
                arguments.package.as_deref(),
                &target,
            )
        }
        Some(target) => terrane_rust_analysis::survey_package_with_policy(
            &arguments.manifest,
            arguments.package.as_deref(),
            &target,
            &probes,
            arguments.containment,
        ),
        None if probes == terrane_rust_analysis::SurveyProbeRequest::default()
            && arguments.containment == Containment::Unavailable
            && !arguments.complete_surface =>
        {
            terrane_rust_analysis::survey_package(
                &arguments.manifest,
                arguments.package.as_deref(),
            )
        }
        None => Err(terrane_rust_analysis::AnalysisError {
            message: "--probes, --complete-surface, or enforced containment requires an explicit --target".to_owned(),
        }),
    }
    .unwrap_or_else(|error| fail(&error.message));
    let mut output = io::BufWriter::new(io::stdout().lock());
    let result = serde_json::to_writer_pretty(&mut output, &report)
        .and_then(|()| writeln!(output).map_err(serde_json::Error::io));
    match result {
        Ok(()) => {}
        Err(error) if error.io_error_kind() == Some(io::ErrorKind::BrokenPipe) => {}
        Err(error) => fail(&format!("cannot write report: {error}")),
    }
}

fn parse_args() -> Result<Arguments, String> {
    parse_values(std::env::args().skip(1))
}

fn parse_values(values: impl IntoIterator<Item = String>) -> Result<Arguments, String> {
    let mut values = values.into_iter();
    let manifest = values
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "missing Cargo.toml".to_owned())?;
    let mut package = None;
    let mut target = None;
    let mut probes = None;
    let mut containment = None;
    let mut complete_surface = false;
    while let Some(flag) = values.next() {
        let value = values
            .next()
            .ok_or_else(|| format!("missing value for `{flag}`"))?;
        match flag.as_str() {
            "--package" if package.is_none() => package = Some(value),
            "--target" if target.is_none() => target = Some(value),
            "--probes" if probes.is_none() => probes = Some(PathBuf::from(value)),
            "--containment" if containment.is_none() => {
                containment = Some(match value.as_str() {
                    "enforced" => Containment::Enforced,
                    "unavailable" => Containment::Unavailable,
                    _ => return Err("--containment must be `enforced` or `unavailable`".to_owned()),
                });
            }
            "--complete-surface" if !complete_surface && value == "true" => {
                complete_surface = true;
            }
            _ => return Err(format!("unknown or repeated option `{flag}`")),
        }
    }
    Ok(Arguments {
        manifest,
        package,
        target,
        probes,
        containment: containment.unwrap_or(Containment::Unavailable),
        complete_surface,
    })
}

fn read_probes(path: &Path) -> Result<terrane_rust_analysis::SurveyProbeRequest, String> {
    let bytes = std::fs::read(path)
        .map_err(|error| format!("cannot read probe request `{}`: {error}", path.display()))?;
    serde_json::from_slice(&bytes)
        .map_err(|error| format!("invalid probe request `{}`: {error}", path.display()))
}

fn usage(message: &str) -> ! {
    eprintln!("terrane-rust-survey: {message}");
    eprintln!(
        "usage: terrane-rust-survey <Cargo.toml> [--package <name[@version]>] \
         [--target <triple>] [--probes <json>] [--complete-surface <true>] \
         [--containment <enforced|unavailable>]"
    );
    std::process::exit(2);
}

fn fail(message: &str) -> ! {
    eprintln!("terrane-rust-survey: {message}");
    std::process::exit(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_options_in_any_order() {
        let arguments = parse_values(
            [
                "Cargo.toml",
                "--containment",
                "enforced",
                "--target",
                "x86_64-unknown-linux-gnu",
                "--package",
                "sample@1.0.0",
                "--complete-surface",
                "true",
            ]
            .map(str::to_owned),
        )
        .unwrap();
        assert_eq!(arguments.containment, Containment::Enforced);
        assert_eq!(arguments.package.as_deref(), Some("sample@1.0.0"));
        assert_eq!(
            arguments.target.as_deref(),
            Some("x86_64-unknown-linux-gnu")
        );
        assert!(arguments.complete_surface);
    }

    #[test]
    fn rejects_repeated_and_invalid_options() {
        assert!(
            parse_values(["Cargo.toml", "--target", "one", "--target", "two"].map(str::to_owned))
                .is_err()
        );
        assert!(parse_values(["Cargo.toml", "--containment", "maybe"].map(str::to_owned)).is_err());
    }
}
