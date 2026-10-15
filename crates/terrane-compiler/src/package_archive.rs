use std::fs::{self, OpenOptions};
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

const MAX_DOWNLOAD_BYTES: u64 = 128 * 1024 * 1024;
const MAX_EXPANDED_BYTES: u64 = 512 * 1024 * 1024;
const MAX_ENTRIES: usize = 10_000;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

/// Downloads and extracts a ZIP or gzip-compressed tar source archive into a new package root.
pub(crate) fn unpack_source_archive(url: &str, destination: &Path) -> Result<(), String> {
    if destination.exists() {
        return Err(format!(
            "archive destination `{}` already exists",
            destination.display()
        ));
    }
    let staging = tempfile_dir(destination)?;
    let mut owns_destination = false;
    let result: Result<(), String> = (|| {
        let contents = staging.join("contents");
        fs::create_dir(&contents)
            .map_err(|error| format!("cannot create archive staging root: {error}"))?;
        let bytes = download(url)?;
        extract(&bytes, &contents)?;
        let root = package_root(&contents)?;
        fs::create_dir(destination)
            .map_err(|error| format!("cannot create extracted package root: {error}"))?;
        owns_destination = true;
        for entry in
            fs::read_dir(&root).map_err(|error| format!("cannot inspect archive root: {error}"))?
        {
            let entry = entry.map_err(|error| format!("cannot inspect archive entry: {error}"))?;
            fs::rename(entry.path(), destination.join(entry.file_name()))
                .map_err(|error| format!("cannot normalize archive root: {error}"))?;
        }
        Ok(())
    })();
    let cleanup = fs::remove_dir_all(&staging);
    if result.is_err() && owns_destination {
        let _ = fs::remove_dir_all(destination);
    }
    result?;
    cleanup.map_err(|error| format!("cannot clean archive staging directory: {error}"))
}

pub(crate) fn with_source_archive<T>(
    url: &str,
    inspect: impl FnOnce(&Path) -> Result<T, String>,
) -> Result<T, String> {
    let base = std::env::temp_dir().join(format!(".terrane-inspect-{}-", std::process::id()));
    let temporary = unique_temp_dir(&base)?;
    let result = (|| {
        let bytes = download(url)?;
        extract(&bytes, &temporary)?;
        let root = package_root(&temporary)?;
        inspect(&root)
    })();
    let cleanup = fs::remove_dir_all(&temporary);
    match (result, cleanup) {
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(format!("cannot clean temporary archive: {error}")),
        (Ok(value), Ok(())) => Ok(value),
    }
}

fn download(url: &str) -> Result<Vec<u8>, String> {
    if !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err("archive URL must use HTTP or HTTPS".into());
    }
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(REQUEST_TIMEOUT))
        .build()
        .into();
    let response = agent
        .get(url)
        .call()
        .map_err(|error| format!("cannot download source archive: {error}"))?;
    let reader = response.into_body().into_reader();
    let mut bytes = Vec::new();
    reader
        .take(MAX_DOWNLOAD_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read source archive: {error}"))?;
    if bytes.len() as u64 > MAX_DOWNLOAD_BYTES {
        return Err("source archive exceeds 128 MiB download limit".into());
    }
    Ok(bytes)
}

fn tempfile_dir(destination: &Path) -> Result<PathBuf, String> {
    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent)
        .map_err(|error| format!("cannot create archive destination parent: {error}"))?;
    let base = parent.join(format!(".terrane-archive-{}-", std::process::id()));
    unique_temp_dir(&base)
}

fn unique_temp_dir(base: &Path) -> Result<PathBuf, String> {
    for attempt in 0..100 {
        let path = PathBuf::from(format!("{}{}", base.display(), attempt));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => {
                return Err(format!(
                    "cannot create temporary archive directory: {error}"
                ));
            }
        }
    }
    Err("cannot allocate unique temporary archive directory".into())
}

fn package_root(staging: &Path) -> Result<PathBuf, String> {
    if staging.join("package.toml").is_file() {
        return Ok(staging.to_path_buf());
    }
    let mut children = fs::read_dir(staging)
        .map_err(|error| format!("cannot inspect extracted archive: {error}"))?
        .map(|entry| {
            entry
                .map(|entry| entry.path())
                .map_err(|error| format!("cannot inspect archive entry: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    children.sort();
    if children.len() == 1 && children[0].is_dir() && children[0].join("package.toml").is_file() {
        return Ok(children.remove(0));
    }
    Err("archive must contain package.toml at its root or one wrapper directory".into())
}

fn safe_relative(path: &Path) -> Result<PathBuf, String> {
    let text = path.to_str().ok_or("archive path is not UTF-8")?;
    if text.contains('\\') || text.starts_with('/') || text.as_bytes().get(1) == Some(&b':') {
        return Err(format!("unsafe archive path `{text}`"));
    }
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            _ => return Err(format!("unsafe archive path `{text}`")),
        }
    }
    Ok(clean)
}

fn extract(bytes: &[u8], staging: &Path) -> Result<(), String> {
    if bytes.starts_with(b"PK\x03\x04") || bytes.starts_with(b"PK\x05\x06") {
        extract_zip(bytes, staging)
    } else if bytes.starts_with(&[0x1f, 0x8b]) {
        extract_tar_gz(bytes, staging)
    } else {
        Err("source archive is neither ZIP nor gzip-compressed tar".into())
    }
}

fn write_entry(
    root: &Path,
    path: PathBuf,
    is_dir: bool,
    reader: &mut dyn Read,
    total: &mut u64,
    seen: &mut std::collections::BTreeSet<PathBuf>,
) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return if is_dir {
            Ok(())
        } else {
            Err("empty archive file path".into())
        };
    }
    let target = root.join(&path);
    if !seen.insert(path) {
        return Err(format!("duplicate archive path `{}`", target.display()));
    }
    if is_dir {
        fs::create_dir_all(&target)
            .map_err(|error| format!("cannot create archive directory: {error}"))?;
        return Ok(());
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create archive parent: {error}"))?;
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&target)
        .map_err(|error| format!("conflicting archive path `{}`: {error}", target.display()))?;
    let copied = io::copy(
        &mut reader.take(MAX_EXPANDED_BYTES - *total + 1),
        &mut output,
    )
    .map_err(|error| format!("cannot extract archive file: {error}"))?;
    *total += copied;
    if *total > MAX_EXPANDED_BYTES {
        return Err("expanded source archive exceeds 512 MiB limit".into());
    }
    Ok(())
}

fn extract_zip(bytes: &[u8], root: &Path) -> Result<(), String> {
    let mut archive = zip::ZipArchive::new(io::Cursor::new(bytes))
        .map_err(|error| format!("invalid ZIP archive: {error}"))?;
    if archive.len() > MAX_ENTRIES {
        return Err("source archive exceeds 10,000-entry limit".into());
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut total = 0;
    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|error| format!("invalid ZIP entry: {error}"))?;
        let path = safe_relative(Path::new(entry.name()))?;
        let mode = entry.unix_mode().unwrap_or(0);
        let kind = mode & 0o170_000;
        if kind != 0 && kind != 0o100_000 && kind != 0o040_000 {
            return Err(format!("unsupported ZIP entry type `{}`", entry.name()));
        }
        write_entry(
            root,
            path,
            entry.is_dir() || kind == 0o040_000,
            &mut entry,
            &mut total,
            &mut seen,
        )?;
    }
    Ok(())
}

fn extract_tar_gz(bytes: &[u8], root: &Path) -> Result<(), String> {
    let decoder = flate2::read::GzDecoder::new(io::Cursor::new(bytes));
    let mut archive = tar::Archive::new(decoder.take(MAX_EXPANDED_BYTES + 1));
    let entries = archive
        .entries()
        .map_err(|error| format!("invalid tar.gz archive: {error}"))?;
    let mut seen = std::collections::BTreeSet::new();
    let mut total = 0;
    let mut count = 0;
    for item in entries {
        count += 1;
        if count > MAX_ENTRIES {
            return Err("source archive exceeds 10,000-entry limit".into());
        }
        let mut entry = item.map_err(|error| format!("invalid tar entry: {error}"))?;
        let kind = entry.header().entry_type();
        if !kind.is_file() && !kind.is_dir() {
            return Err("tar archive contains unsupported link or special entry".into());
        }
        let path = entry
            .path()
            .map_err(|error| format!("invalid tar path: {error}"))?
            .into_owned();
        write_entry(
            root,
            safe_relative(&path)?,
            kind.is_dir(),
            &mut entry,
            &mut total,
            &mut seen,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp() -> PathBuf {
        let base = std::env::temp_dir().join(format!(".terrane-test-{}-", std::process::id()));
        unique_temp_dir(&base).unwrap()
    }

    fn zip(entries: &[(&str, &[u8], u32)]) -> Vec<u8> {
        let cursor = io::Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(cursor);
        for (name, data, mode) in entries {
            let options = zip::write::SimpleFileOptions::default().unix_permissions(*mode);
            writer.start_file(*name, options).unwrap();
            writer.write_all(data).unwrap();
        }
        writer.finish().unwrap().into_inner()
    }

    fn tar_gz(entries: &[(&str, &[u8], tar::EntryType)]) -> Vec<u8> {
        let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        let mut builder = tar::Builder::new(encoder);
        for (name, data, kind) in entries {
            let mut header = tar::Header::new_gnu();
            header.set_entry_type(*kind);
            header.set_size(data.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            builder.append_data(&mut header, *name, *data).unwrap();
        }
        builder.into_inner().unwrap().finish().unwrap()
    }

    #[test]
    fn roots_package_at_archive_root_or_single_wrapper() {
        for (bytes, expected) in [
            (
                zip(&[
                    ("package.toml", b"[package]", 0o100_644),
                    ("src/main.trn", b"main", 0o100_644),
                ]),
                "src/main.trn",
            ),
            (
                zip(&[
                    ("repo/package.toml", b"[package]", 0o100_644),
                    ("repo/src/main.trn", b"main", 0o100_644),
                ]),
                "src/main.trn",
            ),
        ] {
            let root = temp();
            extract(&bytes, &root).unwrap();
            let package = package_root(&root).unwrap();
            assert_eq!(fs::read(package.join(expected)).unwrap(), b"main");
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn zip_and_tar_gz_extract_equivalent_trees() {
        let zip_root = temp();
        let tar_root = temp();
        let zip_bytes = zip(&[
            ("package.toml", b"manifest", 0o100_644),
            ("src/lib.trn", b"source", 0o100_644),
        ]);
        let tar_bytes = tar_gz(&[
            (".", b"", tar::EntryType::Directory),
            ("./package.toml", b"manifest", tar::EntryType::Regular),
            ("./src/lib.trn", b"source", tar::EntryType::Regular),
        ]);
        extract(&zip_bytes, &zip_root).unwrap();
        extract(&tar_bytes, &tar_root).unwrap();
        assert_eq!(
            fs::read(zip_root.join("src/lib.trn")).unwrap(),
            fs::read(tar_root.join("src/lib.trn")).unwrap()
        );
        assert_eq!(
            fs::read(zip_root.join("package.toml")).unwrap(),
            fs::read(tar_root.join("package.toml")).unwrap()
        );
        fs::remove_dir_all(zip_root).unwrap();
        fs::remove_dir_all(tar_root).unwrap();
    }

    #[test]
    fn rejects_traversal_links_duplicates_and_malformed_data() {
        let sandbox = temp();
        let root = sandbox.join("root");
        fs::create_dir(&root).unwrap();
        assert!(extract(&zip(&[("../escape", b"bad", 0o100_644)]), &root).is_err());
        assert!(!root.parent().unwrap().join("escape").exists());
        let link = tar_gz(&[("link", b"target", tar::EntryType::Symlink)]);
        assert!(extract(&link, &root).is_err());
        let duplicate = tar_gz(&[
            ("same", b"a", tar::EntryType::Regular),
            ("same", b"b", tar::EntryType::Regular),
        ]);
        assert!(extract(&duplicate, &root).is_err());
        assert!(extract(b"PK\x03\x04malformed", &root).is_err());
        fs::remove_dir_all(sandbox).unwrap();
    }

    #[test]
    fn rejects_size_overflow_without_large_test_allocations() {
        let root = temp();
        let mut total = MAX_EXPANDED_BYTES;
        let mut seen = std::collections::BTreeSet::new();
        assert!(
            write_entry(
                &root,
                PathBuf::from("x"),
                false,
                &mut io::Cursor::new(b"x"),
                &mut total,
                &mut seen
            )
            .is_err()
        );
        fs::remove_dir_all(root).unwrap();
    }
}
