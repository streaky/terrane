use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

struct Arguments {
    manifest: PathBuf,
    package: Option<String>,
    target: Option<String>,
    probes: Option<PathBuf>,
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
        Some(target) => terrane_rust_analysis::survey_package_for_target_with_probes(
            &arguments.manifest,
            arguments.package.as_deref(),
            &target,
            &probes,
        ),
        None if probes == terrane_rust_analysis::SurveyProbeRequest::default() => {
            terrane_rust_analysis::survey_package(&arguments.manifest, arguments.package.as_deref())
        }
        None => Err(terrane_rust_analysis::AnalysisError {
            message: "--probes requires an explicit --target".to_owned(),
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
    let mut values = std::env::args().skip(1);
    let manifest = values
        .next()
        .map(PathBuf::from)
        .ok_or_else(|| "missing Cargo.toml".to_owned())?;
    let mut package = None;
    let mut target = None;
    let mut probes = None;
    while let Some(flag) = values.next() {
        let value = values
            .next()
            .ok_or_else(|| format!("missing value for `{flag}`"))?;
        match flag.as_str() {
            "--package" if package.is_none() => package = Some(value),
            "--target" if target.is_none() => target = Some(value),
            "--probes" if probes.is_none() => probes = Some(PathBuf::from(value)),
            _ => return Err(format!("unknown or repeated option `{flag}`")),
        }
    }
    Ok(Arguments {
        manifest,
        package,
        target,
        probes,
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
         [--target <triple>] [--probes <json>]"
    );
    std::process::exit(2);
}

fn fail(message: &str) -> ! {
    eprintln!("terrane-rust-survey: {message}");
    std::process::exit(1)
}
