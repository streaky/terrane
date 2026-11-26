use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use terrane_compiler::{MANIFEST_FILE_NAME, Package};

use super::{CliFailure, debugger_failure};

fn load(path: &Path) -> Result<Package, CliFailure> {
    Package::load(path).map_err(|errors| {
        debugger_failure(errors.into_iter().map(|e| e.diagnostic.render(&e.source)).collect::<String>())
    })
}

// Compare physical files: a dependency may have been reached via a symlink.
fn contains(package: &Package, source: &Path) -> bool {
    package.units.iter().any(|unit| {
        let path = unit.source.path();
        path == source || path.canonicalize().is_ok_and(|p| p == source)
    })
}

pub(crate) fn resolve(args: &[OsString]) -> Result<(PathBuf, Option<Package>), CliFailure> {
    let (source, flags) = args.split_first().ok_or_else(CliFailure::usage)?;
    let canonical = |path: &OsString| fs::canonicalize(path).map_err(|err| debugger_failure(err.to_string()));
    let source = canonical(source)?;
    let boundary = match flags {
        [] => None,
        [flag, folder] if flag == "--workspace" => Some(canonical(folder)?),
        _ => return Err(CliFailure::usage()),
    };
    let limit = if boundary.is_some() { usize::MAX } else { 8 };
    for directory in source.ancestors().skip(1).take(limit)
        .take_while(|dir| boundary.as_ref().is_none_or(|root| dir.starts_with(root))) {
        let manifest = directory.join(MANIFEST_FILE_NAME);
        if !manifest.is_file() { continue; }
        let package = load(&manifest)?;
        if contains(&package, &source) {
            return Ok((manifest, Some(package)));
        }
    }
    Ok((source, None))
}
