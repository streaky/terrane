use std::fs::{self, OpenOptions};
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct LimitedReader<R> {
    inner: R,
    remaining: u64,
    overflowed: bool,
}

impl<R: Read> Read for LimitedReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        if self.remaining == 0 {
            let mut probe = [0_u8; 1];
            if self.inner.read(&mut probe)? == 0 {
                return Ok(0);
            }
            self.overflowed = true;
            return Err(io::Error::other("expanded source archive limit exceeded"));
        }
        let max = buffer
            .len()
            .min(usize::try_from(self.remaining).unwrap_or(usize::MAX));
        let read = self.inner.read(&mut buffer[..max])?;
        self.remaining -= read as u64;
        Ok(read)
    }
}
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
    let staging = TempDir::new(tempfile_dir(destination)?);
    let mut owns_destination = false;
    let result: Result<(), String> = (|| {
        let contents = staging.0.join("contents");
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
    let cleanup = staging.cleanup();
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
    let temporary = TempDir::new(unique_temp_dir(&base)?);
    let result = (|| {
        let bytes = download(url)?;
        extract(&bytes, &temporary.0)?;
        let root = package_root(&temporary.0)?;
        inspect(&root)
    })();
    let cleanup = temporary.cleanup();
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
    for _ in 0..100 {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let name = base.file_name().unwrap_or_default().to_string_lossy();
        let path = base.with_file_name(format!("{name}{sequence}-{nonce}"));
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

struct TempDir(PathBuf);

impl TempDir {
    fn new(path: PathBuf) -> Self {
        Self(path)
    }

    fn cleanup(mut self) -> io::Result<()> {
        fs::remove_dir_all(&self.0)?;
        self.0.clear();
        Ok(())
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if !self.0.as_os_str().is_empty() {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
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
    limit: u64,
    seen: &mut std::collections::BTreeSet<PathBuf>,
) -> Result<(), String> {
    if path.as_os_str().is_empty() {
        return if is_dir {
            Ok(())
        } else {
            Err("empty archive file path".into())
        };
    }
    if excluded_path(&path, is_dir) {
        if !is_dir {
            let copied = io::copy(
                &mut reader.take(limit.saturating_sub(*total) + 1),
                &mut io::sink(),
            )
            .map_err(|error| format!("cannot inspect excluded archive file: {error}"))?;
            *total += copied;
            if *total > limit {
                return Err(expansion_limit_error(limit));
            }
        }
        return Ok(());
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
    let remaining = limit.saturating_sub(*total);
    let copied = io::copy(&mut reader.take(remaining + 1), &mut output)
        .map_err(|error| format!("cannot extract archive file: {error}"))?;
    *total += copied;
    if copied > remaining {
        return Err(expansion_limit_error(limit));
    }
    Ok(())
}

fn excluded_path(path: &Path, is_dir: bool) -> bool {
    let directories = if is_dir {
        path
    } else {
        path.parent().unwrap_or(Path::new(""))
    };
    directories.components().any(|component| {
        matches!(component, Component::Normal(name) if name == ".git" || name == ".trn")
    })
}

fn expansion_limit_error(limit: u64) -> String {
    format!(
        "expanded source archive exceeds {} MiB limit",
        limit / (1024 * 1024)
    )
}

fn extract_zip(bytes: &[u8], root: &Path) -> Result<(), String> {
    extract_zip_with_limit(bytes, root, MAX_EXPANDED_BYTES)
}

fn extract_zip_with_limit(bytes: &[u8], root: &Path, limit: u64) -> Result<(), String> {
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
            limit,
            &mut seen,
        )?;
    }
    Ok(())
}

fn extract_tar_gz(bytes: &[u8], root: &Path) -> Result<(), String> {
    extract_tar_gz_with_limit(bytes, root, MAX_EXPANDED_BYTES)
}

fn extract_tar_gz_with_limit(bytes: &[u8], root: &Path, limit: u64) -> Result<(), String> {
    let decoder = flate2::read::GzDecoder::new(io::Cursor::new(bytes));
    let limited = LimitedReader {
        inner: decoder,
        remaining: limit,
        overflowed: false,
    };
    let mut archive = tar::Archive::new(limited);
    let mut seen = std::collections::BTreeSet::new();
    let mut total: u64 = 0;
    let mut count = 0;
    let result = (|| {
        let entries = archive
            .entries()
            .map_err(|error| format!("invalid tar.gz archive: {error}"))?;
        for item in entries {
            count += 1;
            if count > MAX_ENTRIES {
                return Err("source archive exceeds 10,000-entry limit".into());
            }
            let mut entry = item.map_err(|error| format!("invalid tar entry: {error}"))?;
            let kind = entry.header().entry_type();
            if kind == tar::EntryType::XGlobalHeader {
                io::copy(&mut entry, &mut io::sink())
                    .map_err(|error| format!("cannot read tar global metadata: {error}"))?;
                continue;
            }
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
                limit,
                &mut seen,
            )?;
        }
        Ok(())
    })();
    let mut limited = archive.into_inner();
    // Include end markers, padding and any trailing decompressed data in the stream budget.
    let result = result.and_then(|()| {
        io::copy(&mut limited, &mut io::sink())
            .map(|_| ())
            .map_err(|error| format!("invalid tar.gz stream: {error}"))
    });
    if limited.overflowed {
        Err(expansion_limit_error(limit))
    } else {
        result
    }
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
    fn tar_gz_with_global_header() -> Vec<u8> {
        let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        let mut builder = tar::Builder::new(encoder);
        let mut global = tar::Header::new_gnu();
        global.set_entry_type(tar::EntryType::XGlobalHeader);
        let metadata = b"52 comment=0123456789012345678901234567890123456789\n";
        global.set_size(metadata.len() as u64);
        global.set_mode(0o644);
        global.set_cksum();
        builder.append(&global, &metadata[..]).unwrap();
        let mut file = tar::Header::new_gnu();
        file.set_entry_type(tar::EntryType::Regular);
        file.set_size(1);
        file.set_mode(0o644);
        file.set_cksum();
        builder
            .append_data(&mut file, "repo/package.toml", &b"x"[..])
            .unwrap();
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
    fn handles_global_pax_and_excludes_unhashed_metadata_paths() {
        let root = temp();
        extract(&tar_gz_with_global_header(), &root).unwrap();
        let package = package_root(&root).unwrap();
        assert_eq!(fs::read(package.join("package.toml")).unwrap(), b"x");
        fs::remove_dir_all(root).unwrap();

        for bytes in [
            zip(&[
                (".git/config", b"ignored", 0o100_644),
                ("nested/.trn/generated", b"ignored", 0o100_644),
                ("package.toml", b"kept", 0o100_644),
            ]),
            tar_gz(&[
                (".git/config", b"ignored", tar::EntryType::Regular),
                ("nested/.trn/generated", b"ignored", tar::EntryType::Regular),
                ("package.toml", b"kept", tar::EntryType::Regular),
            ]),
        ] {
            let root = temp();
            extract(&bytes, &root).unwrap();
            assert!(!root.join(".git").exists());
            assert!(!root.join("nested").exists());
            assert_eq!(fs::read(root.join("package.toml")).unwrap(), b"kept");
            fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn global_pax_entries_count_toward_entry_limit() {
        let bytes = tar_gz(&vec![
            (
                "pax_global_header",
                &b""[..],
                tar::EntryType::XGlobalHeader
            );
            MAX_ENTRIES + 1
        ]);
        let root = temp();
        assert!(extract(&bytes, &root).unwrap_err().contains("entry limit"));
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn excluded_zip_data_still_counts_toward_expansion_limit() {
        let root = temp();
        let bytes = zip(&[(".trn/cache", b"oversize", 0o100_644)]);
        assert!(
            extract_zip_with_limit(&bytes, &root, 2)
                .unwrap_err()
                .contains("expanded source archive exceeds")
        );
        assert!(!root.join(".trn").exists());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn tar_expansion_limit_includes_tar_headers_and_padding() {
        let bytes = tar_gz_with_global_header();
        let mut decoded = Vec::new();
        flate2::read::GzDecoder::new(&bytes[..])
            .read_to_end(&mut decoded)
            .unwrap();
        let length = decoded.len() as u64;
        for limit in [1, 512, 1024, length - 1] {
            let root = temp();
            assert!(
                extract_tar_gz_with_limit(&bytes, &root, limit)
                    .unwrap_err()
                    .contains("expanded source archive exceeds")
            );
            fs::remove_dir_all(root).unwrap();
        }
        let root = temp();
        extract_tar_gz_with_limit(&bytes, &root, length).unwrap();
        assert_eq!(fs::read(root.join("repo/package.toml")).unwrap(), b"x");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_size_overflow_without_large_test_allocations() {
        let root = temp();
        let mut total = 0;
        let mut seen = std::collections::BTreeSet::new();
        assert!(
            write_entry(
                &root,
                PathBuf::from("x"),
                false,
                &mut io::Cursor::new(b"xx"),
                &mut total,
                1,
                &mut seen
            )
            .unwrap_err()
            .contains("expanded source archive exceeds")
        );
        fs::remove_dir_all(root).unwrap();
    }
}
