#[derive(Clone, Default)]
pub struct TerraneFilesystemAuthority {
    _private: (),
}

pub fn terrane_acquire_filesystem_authority() -> TerraneFilesystemAuthority {
    TerraneFilesystemAuthority { _private: () }
}


#[derive(Clone, Default)]
pub struct TerraneFilesystemResult {
    pub failed: bool,
    pub message: String,
    pub text: String,
    pub detail: String,
    pub data: Vec<u8>,
    pub number: i128,
    pub flag: bool,
    pub entries: Option<terrane_collection_support::List<String>>,
    pub error_kind: &'static str,
}

pub fn terrane_io_error(error: std::io::Error) -> TerraneFilesystemResult {
    let kind = terrane_stream_abi::io_error_kind(error.kind());
    TerraneFilesystemResult {
        failed: true,
        message: error.to_string(),
        error_kind: kind,
        ..TerraneFilesystemResult::default()
    }
}
pub fn terrane_filesystem_result_failed(result: &TerraneFilesystemResult) -> bool {
    result.failed
}
pub fn terrane_filesystem_result_message(result: &TerraneFilesystemResult) -> String {
    result.message.clone()
}
pub fn terrane_filesystem_result_text(result: &TerraneFilesystemResult) -> String {
    result.text.clone()
}
pub fn terrane_filesystem_result_detail(result: &TerraneFilesystemResult) -> String {
    result.detail.clone()
}
pub fn terrane_filesystem_result_bytes(result: &TerraneFilesystemResult) -> Vec<u8> {
    result.data.clone()
}
pub fn terrane_filesystem_result_int(result: &TerraneFilesystemResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
pub fn terrane_filesystem_result_bool(result: &TerraneFilesystemResult) -> bool {
    result.flag
}
pub fn terrane_filesystem_result_error_kind(result: &TerraneFilesystemResult) -> String {
    if result.failed {
        result.error_kind.to_owned()
    } else {
        "none".to_owned()
    }
}
pub fn terrane_filesystem_result_entries(
    result: &TerraneFilesystemResult,
) -> terrane_collection_support::List<String> {
    result.entries.clone().unwrap_or_else(|| terrane_collection_support::List::new(Vec::new()))
}

#[cfg(unix)]
pub fn terrane_permission_detail(metadata: &std::fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt as _;
    format!("unix-mode:{:04o}", metadata.permissions().mode() & 0o7777)
}

#[cfg(not(unix))]
pub fn terrane_permission_detail(metadata: &std::fs::Metadata) -> String {
    format!("readonly:{}", metadata.permissions().readonly())
}

pub fn terrane_metadata(path: &std::path::Path, follow: bool) -> TerraneFilesystemResult {
    let metadata = if follow {
        std::fs::metadata(path)
    } else {
        std::fs::symlink_metadata(path)
    };
    match metadata {
        Ok(metadata) => {
            let file_type = metadata.file_type();
            let kind = if file_type.is_file() {
                "regular-file"
            } else if file_type.is_dir() {
                "directory"
            } else if file_type.is_symlink() {
                "symlink"
            } else {
                "other"
            };
            TerraneFilesystemResult {
                text: kind.to_owned(),
                detail: terrane_permission_detail(&metadata),
                number: i128::from(metadata.len()),
                flag: metadata.permissions().readonly(),
                ..TerraneFilesystemResult::default()
            }
        }
        Err(error) => TerraneFilesystemResult {
            failed: true,
            message: error.to_string(),
            error_kind: terrane_stream_abi::io_error_kind(error.kind()),
            text: "other".to_owned(),
            detail: "unavailable".to_owned(),
            ..TerraneFilesystemResult::default()
        },
    }
}

pub fn terrane_atomic_replace(path: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    let name = path.file_name().unwrap_or_else(|| std::ffi::OsStr::new("file"));
    let existing_permissions = std::fs::symlink_metadata(path)
        .ok()
        .filter(|metadata| !metadata.file_type().is_symlink())
        .map(|metadata| metadata.permissions());
    let mut attempt = 0_u32;
    loop {
        let mut temporary_name = std::ffi::OsString::from(".");
        temporary_name.push(name);
        temporary_name.push(format!(".terrane-{}-{attempt}", std::process::id()));
        let temporary = parent.join(temporary_name);
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary) {
            Ok(mut file) => {
                let outcome = (|| {
                    if let Some(permissions) = existing_permissions.clone() {
                        file.set_permissions(permissions)?;
                    }
                    file.write_all(data)?;
                    file.sync_all()?;
                    std::fs::rename(&temporary, path)?;
                    #[cfg(unix)]
                    std::fs::File::open(parent)?.sync_all()?;
                    Ok(())
                })();
                if outcome.is_err() {
                    let _ = std::fs::remove_file(&temporary);
                }
                return outcome;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists && attempt < 32 => {
                attempt += 1;
            }
            Err(error) => return Err(error),
        }
    }
}


pub fn terrane_filesystem_exists(path: String) -> TerraneFilesystemResult {
    match std::path::Path::new(&path).try_exists() {
        Ok(exists) => TerraneFilesystemResult {
            flag: exists,
            ..TerraneFilesystemResult::default()
        },
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_metadata(path: String, follow: bool) -> TerraneFilesystemResult {
    terrane_metadata(std::path::Path::new(&path), follow)
}

pub fn terrane_filesystem_realpath(path: String) -> TerraneFilesystemResult {
    match std::fs::canonicalize(path).and_then(terrane_path_text) {
        Ok(value) => TerraneFilesystemResult {
            text: value,
            ..TerraneFilesystemResult::default()
        },
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_read_link(path: String) -> TerraneFilesystemResult {
    match std::fs::read_link(path).and_then(terrane_path_text) {
        Ok(value) => TerraneFilesystemResult {
            text: value,
            ..TerraneFilesystemResult::default()
        },
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_read_bounded(
    path: String,
    limit: impl Into<terrane_int_support::Int>,
) -> TerraneFilesystemResult {
    use std::io::Read as _;
    let Some(limit) = limit.into().as_usize() else {
        return terrane_io_error(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid read limit",
        ));
    };
    let mut value = Vec::with_capacity(limit.min(8192));
    let outcome = std::fs::File::open(path)
        .and_then(|file| file.take(limit.saturating_add(1) as u64).read_to_end(&mut value));
    match outcome {
        Ok(_) if value.len() <= limit => TerraneFilesystemResult {
            number: value.len() as i128,
            data: value,
            ..TerraneFilesystemResult::default()
        },
        Ok(_) => terrane_io_error(std::io::Error::new(
            std::io::ErrorKind::FileTooLarge,
            "file exceeds declared read limit",
        )),
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_write_atomic(path: String, data: Vec<u8>) -> TerraneFilesystemResult {
    match terrane_atomic_replace(std::path::Path::new(&path), &data) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_remove(path: String) -> TerraneFilesystemResult {
    match std::fs::remove_file(path) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_create_directory(path: String) -> TerraneFilesystemResult {
    match std::fs::create_dir(path) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_create_directories(path: String) -> TerraneFilesystemResult {
    match std::fs::create_dir_all(path) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_list_directory(
    path: String,
    limit: terrane_int_support::Int,
) -> TerraneFilesystemResult {
    let Some(limit) = limit.as_usize() else {
        return terrane_io_error(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid directory entry limit",
        ));
    };
    let mut entries = Vec::new();
    let directory = match std::fs::read_dir(path) {
        Ok(directory) => directory,
        Err(error) => return terrane_io_error(error),
    };
    for entry in directory {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => return terrane_io_error(error),
        };
        if entries.len() == limit {
            return terrane_io_error(std::io::Error::new(
                std::io::ErrorKind::FileTooLarge,
                "directory exceeds declared entry limit",
            ));
        }
        match entry.file_name().into_string() {
            Ok(name) => entries.push(name),
            Err(_) => {
                return terrane_io_error(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "directory entry name is not valid UTF-8",
                ));
            }
        }
    }
    TerraneFilesystemResult {
        entries: Some(terrane_collection_support::List::new(entries)),
        ..TerraneFilesystemResult::default()
    }
}

pub fn terrane_filesystem_remove_directory(path: String) -> TerraneFilesystemResult {
    match std::fs::remove_dir(path) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}

//# SECURITY-BEGIN [filesystem.directory-tree-removal]
pub fn terrane_filesystem_remove_directory_tree(path: String) -> TerraneFilesystemResult {
    match std::fs::remove_dir_all(path) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
//# SECURITY-END [filesystem.directory-tree-removal]

pub fn terrane_filesystem_create_symlink(
    target: String,
    link: String,
    directory: bool,
) -> TerraneFilesystemResult {
    #[cfg(unix)]
    let result = {
        let _ = directory;
        std::os::unix::fs::symlink(target, link)
    };
    #[cfg(windows)]
    let result = if directory {
        std::os::windows::fs::symlink_dir(target, link)
    } else {
        std::os::windows::fs::symlink_file(target, link)
    };
    #[cfg(not(any(unix, windows)))]
    let result: std::io::Result<()> = {
        let _ = (target, link, directory);
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "symbolic links are unsupported on this platform",
        ))
    };
    match result {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_create_hard_link(
    target: String,
    link: String,
) -> TerraneFilesystemResult {
    match std::fs::hard_link(target, link) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_filesystem_rename(source: String, destination: String) -> TerraneFilesystemResult {
    match std::fs::rename(source, destination) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}

pub fn terrane_path_text(path: std::path::PathBuf) -> std::io::Result<String> {
    path.into_os_string().into_string().map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "filesystem path is not valid Unicode",
        )
    })
}

