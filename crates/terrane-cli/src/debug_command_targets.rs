use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use terrane_compiler::{MANIFEST_FILE_NAME, Package};

use super::{CliFailure, debugger_failure};

fn load(path: &Path) -> Result<Package, CliFailure> {
    Package::load(path).map_err(|errors| {
        debugger_failure(
            errors
                .into_iter()
                .map(|e| e.diagnostic.render(&e.source))
                .collect::<String>(),
        )
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
    let canonical =
        |path: &OsString| fs::canonicalize(path).map_err(|err| debugger_failure(err.to_string()));
    let source = canonical(source)?;
    let boundary = match flags {
        [] => None,
        [flag, folder] if flag == "--workspace" => Some(canonical(folder)?),
        _ => return Err(CliFailure::usage()),
    };
    let limit = if boundary.is_some() { usize::MAX } else { 8 };
    let mut load_failure = None;
    for directory in source
        .ancestors()
        .skip(1)
        .take(limit)
        .take_while(|dir| boundary.as_ref().is_none_or(|root| dir.starts_with(root)))
    {
        let manifest = directory.join(MANIFEST_FILE_NAME);
        if !manifest.is_file() {
            continue;
        }
        let package = match load(&manifest) {
            Ok(package) => package,
            Err(error) => {
                // Prefer a verified owner farther out, but don't silently treat
                // a possibly broken package as a standalone script.
                load_failure.get_or_insert(error);
                continue;
            }
        };
        if contains(&package, &source) {
            return Ok((manifest, Some(package)));
        }
    }
    if let Some(error) = load_failure {
        return Err(error);
    }
    Ok((source, None))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_prefers_verified_owner() {
        let root =
            std::env::temp_dir().join(format!("terrane-view-target-owner-{}", std::process::id()));
        let nested = root.join("nested");
        fs::create_dir_all(&nested).unwrap();
        let root = root.canonicalize().unwrap();
        let nested = root.join("nested");
        let outer_manifest = root.join(MANIFEST_FILE_NAME);
        let inner_manifest = nested.join(MANIFEST_FILE_NAME);
        let source = nested.join("sample.trn");
        fs::write(
            &outer_manifest,
            "package = 'sample'\nartifact = 'library'\n[namespaces]\nsample = 'nested'\n",
        )
        .unwrap();
        fs::write(&inner_manifest, "not valid toml").unwrap();
        fs::write(&source, "namespace sample\n").unwrap();
        let source = source.into_os_string();
        let owned = resolve(std::slice::from_ref(&source));
        let no_owner = resolve(&[source, "--workspace".into(), nested.into_os_string()]);
        fs::remove_dir_all(&root).unwrap();

        let (manifest, package) = owned.unwrap();
        assert_eq!(manifest, outer_manifest);
        assert!(package.is_some());
        assert!(no_owner.err().unwrap().message.contains(MANIFEST_FILE_NAME));
    }
}
