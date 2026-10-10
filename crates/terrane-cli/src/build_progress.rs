use std::fmt;
use std::path::{Path, PathBuf};
use std::time::Instant;

/// Build-only reporting keeps machine-readable stdout and other commands unchanged.
pub(crate) struct BuildProgress<'a> {
    started: Instant,
    stage_started: Instant,
    stage: &'static str,
    input: PathBuf,
    package: Option<&'a str>,
}

impl<'a> BuildProgress<'a> {
    pub(crate) fn new(input: &Path) -> Self {
        let input = std::path::absolute(input).unwrap_or_else(|_| input.to_path_buf());
        eprintln!("Building [{}]: loading package...", input.display());
        let started = Instant::now();
        Self {
            started,
            stage_started: started,
            stage: "loading package",
            input,
            package: None,
        }
    }

    pub(crate) fn identify(&mut self, package: &'a str) {
        self.package = Some(package);
    }

    pub(crate) fn advance(&mut self, stage: &'static str) {
        self.finish_stage();
        eprintln!("Building [{self}]: {stage}...");
        self.stage = stage;
        self.stage_started = Instant::now();
    }

    fn finish_stage(&self) {
        eprintln!(
            "Completed [{self}]: {} ({:.3}s)",
            self.stage,
            self.stage_started.elapsed().as_secs_f64()
        );
    }

    pub(crate) fn finish(self) {
        self.finish_stage();
        eprintln!(
            "Build finished [{self}] ({:.3}s total)",
            self.started.elapsed().as_secs_f64()
        );
    }
}

impl fmt::Display for BuildProgress<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(package) = self.package {
            write!(formatter, "{package} at ")?;
        }
        write!(formatter, "{}", self.input.display())
    }
}
