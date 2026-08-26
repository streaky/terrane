const TERRANE_RECORD_SEPARATOR: char = '\u{1e}';

#[derive(Default)]
struct TerraneSystemResult {
    failed: bool,
    message: String,
    text: String,
    data: Vec<u8>,
    number: i128,
    flag: bool,
}

fn terrane_hex(data: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(data.len() * 2);
    for byte in data {
        encoded.push(DIGITS[usize::from(byte >> 4)] as char);
        encoded.push(DIGITS[usize::from(byte & 0x0f)] as char);
    }
    encoded
}

fn terrane_unhex(text: &str) -> Vec<u8> {
    fn digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    text.as_bytes()
        .chunks_exact(2)
        .filter_map(|pair| Some((digit(pair[0])? << 4) | digit(pair[1])?))
        .collect()
}

fn terrane_pack(result: TerraneSystemResult) -> String {
    [
        if result.failed { "1" } else { "0" }.to_owned(),
        result.message.replace(TERRANE_RECORD_SEPARATOR, " "),
        result.text.replace(TERRANE_RECORD_SEPARATOR, " "),
        terrane_hex(&result.data),
        result.number.to_string(),
        if result.flag { "1" } else { "0" }.to_owned(),
    ]
    .join(&TERRANE_RECORD_SEPARATOR.to_string())
}

fn terrane_field(record: &str, index: usize) -> &str {
    record.split(TERRANE_RECORD_SEPARATOR).nth(index).unwrap_or("")
}

#[allow(dead_code, reason = "filesystem intrinsics are selected independently")]
fn terrane_system_result_failed(record: &str) -> bool {
    terrane_field(record, 0) == "1"
}
#[allow(dead_code, reason = "filesystem intrinsics are selected independently")]
fn terrane_system_result_message(record: &str) -> String {
    terrane_field(record, 1).to_owned()
}
#[allow(dead_code, reason = "filesystem intrinsics are selected independently")]
fn terrane_system_result_text(record: &str) -> String {
    terrane_field(record, 2).to_owned()
}
#[allow(dead_code, reason = "filesystem intrinsics are selected independently")]
fn terrane_system_result_bytes(record: &str) -> Vec<u8> {
    terrane_unhex(terrane_field(record, 3))
}
#[allow(dead_code, reason = "filesystem intrinsics are selected independently")]
fn terrane_system_result_int(record: &str) -> terrane_int_support::Int {
    terrane_field(record, 4)
        .parse::<i128>()
        .map(terrane_int_support::Int::from)
        .unwrap_or_else(|_| terrane_int_support::Int::from(0_i64))
}
#[allow(dead_code, reason = "filesystem intrinsics are selected independently")]
fn terrane_system_result_bool(record: &str) -> bool {
    terrane_field(record, 5) == "1"
}

fn terrane_io_error(error: std::io::Error) -> String {
    terrane_pack(TerraneSystemResult {
        failed: true,
        message: error.to_string(),
        ..TerraneSystemResult::default()
    })
}

#[cfg(unix)]
fn terrane_permission_detail(metadata: &std::fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt as _;
    format!("unix-mode:{:04o}", metadata.permissions().mode() & 0o7777)
}

#[cfg(not(unix))]
fn terrane_permission_detail(metadata: &std::fs::Metadata) -> String {
    format!("readonly:{}", metadata.permissions().readonly())
}

fn terrane_metadata(path: &std::path::Path, follow: bool) -> String {
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
            terrane_pack(TerraneSystemResult {
                text: format!("{kind}|{}", terrane_permission_detail(&metadata)),
                number: i128::from(metadata.len()),
                flag: metadata.permissions().readonly(),
                ..TerraneSystemResult::default()
            })
        }
        Err(error) => terrane_pack(TerraneSystemResult {
            failed: true,
            message: error.to_string(),
            text: "other|unavailable".to_owned(),
            ..TerraneSystemResult::default()
        }),
    }
}

fn terrane_atomic_replace(path: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let name = path.file_name().and_then(std::ffi::OsStr::to_str).unwrap_or("file");
    let mut attempt = 0_u32;
    loop {
        let temporary = parent.join(format!(".{name}.terrane-{}-{attempt}", std::process::id()));
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary) {
            Ok(mut file) => {
                let outcome = (|| {
                    file.write_all(data)?;
                    file.sync_all()?;
                    std::fs::rename(&temporary, path)?;
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

#[cfg(target_os = "linux")]
fn terrane_open_beneath(base: &std::path::Path, child: &std::path::Path, cross: bool) -> std::io::Result<String> {
    use std::os::fd::{AsRawFd as _, FromRawFd as _};
    use std::os::unix::ffi::OsStrExt as _;
    #[repr(C)]
    struct OpenHow { flags: u64, mode: u64, resolve: u64 }
    unsafe extern "C" { fn syscall(number: isize, ...) -> isize; }
    const SYS_OPENAT2: isize = 437;
    const O_PATH: u64 = 0o10000000;
    const O_CLOEXEC: u64 = 0o2000000;
    const RESOLVE_NO_XDEV: u64 = 0x01;
    const RESOLVE_NO_MAGICLINKS: u64 = 0x02;
    const RESOLVE_NO_SYMLINKS: u64 = 0x04;
    const RESOLVE_BENEATH: u64 = 0x08;
    let directory = std::fs::File::open(base)?;
    let child = std::ffi::CString::new(child.as_os_str().as_bytes())
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "path contains NUL"))?;
    let how = OpenHow {
        flags: O_PATH | O_CLOEXEC,
        mode: 0,
        resolve: RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS | RESOLVE_NO_SYMLINKS
            | if cross { 0 } else { RESOLVE_NO_XDEV },
    };
    // SAFETY: openat2 receives a live directory descriptor, NUL-terminated relative path, and a
    // correctly sized kernel ABI structure. The returned descriptor is immediately owned by File.
    let descriptor = unsafe {
        syscall(
            SYS_OPENAT2,
            directory.as_raw_fd(),
            child.as_ptr(),
            &how as *const OpenHow,
            std::mem::size_of::<OpenHow>(),
        )
    };
    if descriptor < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: a nonnegative openat2 return is a newly owned descriptor.
    let opened = unsafe { std::fs::File::from_raw_fd(descriptor as i32) };
    std::fs::read_link(format!("/proc/self/fd/{}", opened.as_raw_fd()))
        .map(|path| path.to_string_lossy().into_owned())
}

#[cfg(not(target_os = "linux"))]
fn terrane_open_beneath(_: &std::path::Path, _: &std::path::Path, _: bool) -> std::io::Result<String> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "race-resistant beneath traversal is unavailable in this target profile",
    ))
}

#[allow(dead_code, reason = "filesystem intrinsics are selected independently")]
fn terrane_filesystem_call(
    operation: String,
    path: String,
    other: String,
    data: Vec<u8>,
    limit: impl Into<terrane_int_support::Int>,
    follow: bool,
    cross: bool,
) -> String {
    let limit = limit.into();
    let path = std::path::Path::new(&path);
    match operation.as_str() {
        "exists" => terrane_pack(TerraneSystemResult {
            flag: path.try_exists().unwrap_or(false),
            ..TerraneSystemResult::default()
        }),
        "metadata" => terrane_metadata(path, follow),
        "canonical" => match std::fs::canonicalize(path) {
            Ok(value) => terrane_pack(TerraneSystemResult {
                text: value.to_string_lossy().into_owned(),
                ..TerraneSystemResult::default()
            }),
            Err(error) => terrane_io_error(error),
        },
        "read-link" => match std::fs::read_link(path) {
            Ok(value) => terrane_pack(TerraneSystemResult {
                text: value.to_string_lossy().into_owned(),
                ..TerraneSystemResult::default()
            }),
            Err(error) => terrane_io_error(error),
        },
        "read" => {
            let Some(limit) = limit.as_usize() else {
                return terrane_io_error(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid read limit"));
            };
            match std::fs::read(path) {
                Ok(value) if value.len() <= limit => terrane_pack(TerraneSystemResult {
                    number: value.len() as i128,
                    data: value,
                    ..TerraneSystemResult::default()
                }),
                Ok(_) => terrane_io_error(std::io::Error::new(std::io::ErrorKind::FileTooLarge, "file exceeds declared read limit")),
                Err(error) => terrane_io_error(error),
            }
        }
        "atomic-write" => match terrane_atomic_replace(path, &data) {
            Ok(()) => terrane_pack(TerraneSystemResult::default()),
            Err(error) => terrane_io_error(error),
        },
        "remove" => match std::fs::remove_file(path) {
            Ok(()) => terrane_pack(TerraneSystemResult::default()),
            Err(error) => terrane_io_error(error),
        },
        "rename" => match std::fs::rename(path, &other) {
            Ok(()) => terrane_pack(TerraneSystemResult::default()),
            Err(error) => terrane_io_error(error),
        },
        "beneath" => match terrane_open_beneath(path, std::path::Path::new(&other), cross) {
            Ok(value) => terrane_pack(TerraneSystemResult {
                text: value,
                ..TerraneSystemResult::default()
            }),
            Err(error) => terrane_io_error(error),
        },
        _ => terrane_io_error(std::io::Error::new(std::io::ErrorKind::InvalidInput, "unknown filesystem operation")),
    }
}

#[allow(dead_code, reason = "process intrinsics are selected independently of filesystem intrinsics")]
#[cfg(unix)]
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    use std::os::unix::ffi::OsStrExt as _;
    value.into_string().map_or_else(
        |raw| format!("raw:{}", terrane_hex(raw.as_bytes())),
        |text| format!("text:{text}"),
    )
}

#[allow(dead_code, reason = "process intrinsics are selected independently of filesystem intrinsics")]
#[cfg(not(unix))]
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    value.into_string().map_or_else(
        |raw| {
            let units = raw.encode_wide().flat_map(u16::to_le_bytes).collect::<Vec<_>>();
            format!("raw:{}", terrane_hex(&units))
        },
        |text| format!("text:{text}"),
    )
}

#[allow(dead_code, reason = "process intrinsics are selected independently")]
fn terrane_platform_value_is_text(value: &str) -> bool {
    value.starts_with("text:")
}

#[allow(dead_code, reason = "process intrinsics are selected independently")]
fn terrane_platform_value_text(value: &str) -> String {
    value.strip_prefix("text:").unwrap_or("").to_owned()
}

#[allow(dead_code, reason = "process intrinsics are selected independently")]
fn terrane_platform_value_bytes(value: &str) -> Vec<u8> {
    value
        .strip_prefix("raw:")
        .map(terrane_unhex)
        .unwrap_or_default()
}

#[allow(dead_code, reason = "process intrinsics are selected independently of filesystem intrinsics")]
fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}

#[allow(dead_code, reason = "process intrinsics are selected independently of filesystem intrinsics")]
fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [terrane_platform_value(name), terrane_platform_value(value)])
        .collect()
}

#[allow(dead_code, reason = "process intrinsics are selected independently of filesystem intrinsics")]
fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}
