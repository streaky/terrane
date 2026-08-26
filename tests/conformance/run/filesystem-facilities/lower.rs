// Generated deterministically by Terrane <version>.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerraneErrorKind {
    ArithmeticOverflow,
    DivisionByZero,
    IntegerConversionOverflow,
    NegativeShiftCount,
    CoercionError,
    DecodeError,
    IndexError,
    MissingKey,
    ResourceError,
    SourceError,
}
impl TerraneErrorKind {
    fn from_source_name(name: &str) -> Self {
        match name {
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    message: String,
    cause: Option<Box<TerraneError>>,
    context: Vec<&'static str>,
}
impl TerraneError {
    fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            cause: None,
            context: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn at(mut self, frame: &'static str) -> Self {
        self.context.push(frame);
        self
    }
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
        if let Some(cause) = &self.cause {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        for frame in &self.context {
            rendered.push_str("\nat ");
            rendered.push_str(frame);
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
impl From<terrane_int_support::ArithmeticError> for TerraneError {
    fn from(error: terrane_int_support::ArithmeticError) -> Self {
        Self::new(
            TerraneErrorKind::from_source_name(error.source_name()),
            error.to_string(),
        )
    }
}
impl From<terrane_string_support::DecodeError> for TerraneError {
    fn from(error: terrane_string_support::DecodeError) -> Self {
        Self::new(
            TerraneErrorKind::DecodeError,
            error.to_string().trim_start_matches(".decode-error: "),
        )
    }
}
impl From<terrane_collection_support::IndexError> for TerraneError {
    fn from(error: terrane_collection_support::IndexError) -> Self {
        Self::new(TerraneErrorKind::IndexError, error.to_string())
    }
}
impl From<terrane_collection_support::MissingKey> for TerraneError {
    fn from(error: terrane_collection_support::MissingKey) -> Self {
        Self::new(TerraneErrorKind::MissingKey, error.to_string())
    }
}
impl From<terrane_collection_support::RangeStepError> for TerraneError {
    fn from(error: terrane_collection_support::RangeStepError) -> Self {
        Self::new(TerraneErrorKind::SourceError, error.to_string())
    }
}
fn __terrane_uncaught(error: TerraneError) -> ! {
    eprintln!("{}", error.render());
    std::process::exit(1);
}
fn __terrane_generated_defect(message: &str) -> ! {
    eprintln!(
        "internal compiler defect: generated program reached an impossible completion: {message}"
    );
    std::process::exit(5);
}
#[allow(dead_code)]
enum TerraneCompletion<T> {
    Normal,
    Return(T),
    Error(TerraneError),
    Break,
    Continue,
}
#[derive(Clone)]
pub struct TerranePlatformStreamHandle(std::sync::Arc<i64>);
impl Default for TerranePlatformStreamHandle {
    fn default() -> Self {
        Self(std::sync::Arc::new(0))
    }
}
impl TerranePlatformStreamHandle {
    fn new(handle: terrane_stream_abi::StreamHandle) -> Self {
        Self(std::sync::Arc::new(handle.id()))
    }
    fn abi_handle(&self) -> terrane_stream_abi::StreamHandle {
        terrane_stream_abi::StreamHandle::from_id(*self.0)
    }
}
#[allow(
    dead_code,
    reason = "file intrinsics are selected independently of standard stream intrinsics"
)]
#[derive(Clone)]
struct TerranePlatformOpenResult {
    handle: TerranePlatformStreamHandle,
    failed: bool,
    message: String,
}
#[derive(Clone)]
struct TerranePlatformReadResult {
    data: Vec<u8>,
    completed: terrane_int_support::Int,
    end: bool,
    failed: bool,
    message: String,
}
#[derive(Clone)]
struct TerranePlatformWriteResult {
    completed: terrane_int_support::Int,
    failed: bool,
    message: String,
}
#[derive(Clone)]
struct TerranePlatformUnitResult {
    failed: bool,
    message: String,
}
#[allow(
    dead_code,
    reason = "standard stream intrinsics are selected independently of file intrinsics"
)]
fn terrane_platform_acquire_stdin() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stdin())
}
#[allow(
    dead_code,
    reason = "standard stream intrinsics are selected independently of file intrinsics"
)]
fn terrane_platform_acquire_stdout() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stdout())
}
#[allow(
    dead_code,
    reason = "standard stream intrinsics are selected independently of file intrinsics"
)]
fn terrane_platform_acquire_stderr() -> TerranePlatformStreamHandle {
    TerranePlatformStreamHandle::new(terrane_stream_abi::acquire_stderr())
}
#[allow(
    dead_code,
    reason = "file intrinsics are selected independently of standard stream intrinsics"
)]
fn terrane_platform_open_file(
    path: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> TerranePlatformOpenResult {
    match terrane_stream_abi::open_file(&path, readable, writable, create, truncate) {
        Ok(handle) => {
            TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::new(handle),
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::default(),
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
fn terrane_platform_read(
    handle: &TerranePlatformStreamHandle,
    limit: terrane_int_support::Int,
) -> TerranePlatformReadResult {
    let Some(limit) = limit.as_usize() else {
        return TerranePlatformReadResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            end: false,
            failed: true,
            message: "stream read count is outside the supported size range".to_owned(),
        };
    };
    match terrane_stream_abi::read(handle.abi_handle(), limit) {
        Ok(outcome) => {
            TerranePlatformReadResult {
                completed: terrane_int_support::Int::from(outcome.data.len() as i128),
                data: outcome.data,
                end: outcome.end,
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformReadResult {
                data: Vec::new(),
                completed: terrane_int_support::Int::from(0_i64),
                end: false,
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
fn terrane_platform_write(
    handle: &TerranePlatformStreamHandle,
    data: &[u8],
    offset: terrane_int_support::Int,
) -> TerranePlatformWriteResult {
    let Some(offset) = offset.as_usize().filter(|offset| *offset <= data.len()) else {
        return TerranePlatformWriteResult {
            completed: terrane_int_support::Int::from(0_i64),
            failed: true,
            message: "stream write offset is outside the buffer".to_owned(),
        };
    };
    match terrane_stream_abi::write(handle.abi_handle(), &data[offset..]) {
        Ok(completed) => {
            TerranePlatformWriteResult {
                completed: terrane_int_support::Int::from(completed as i128),
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformWriteResult {
                completed: terrane_int_support::Int::from(0_i64),
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
fn terrane_platform_flush(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::flush(handle.abi_handle()))
}
fn terrane_platform_sync_data(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_data(handle.abi_handle()))
}
fn terrane_platform_sync_all(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_all(handle.abi_handle()))
}
fn terrane_platform_close(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::close(handle.abi_handle()))
}
fn terrane_platform_release(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    if std::sync::Arc::strong_count(&handle.0) == 1 {
        terrane_platform_unit(terrane_stream_abi::release(handle.abi_handle()))
    } else {
        TerranePlatformUnitResult {
            failed: false,
            message: String::new(),
        }
    }
}
fn terrane_platform_unit(result: std::io::Result<()>) -> TerranePlatformUnitResult {
    match result {
        Ok(()) => {
            TerranePlatformUnitResult {
                failed: false,
                message: String::new(),
            }
        }
        Err(error) => {
            TerranePlatformUnitResult {
                failed: true,
                message: error.to_string(),
            }
        }
    }
}
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
        .filter_map(|pair| Some(digit(pair[0])? << 4 | digit(pair[1])?))
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
    format!("unix-mode:{:04o}", metadata.permissions().mode() &0o7777)
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
        Err(error) => {
            terrane_pack(TerraneSystemResult {
                failed: true,
                message: error.to_string(),
                text: "other|unavailable".to_owned(),
                ..TerraneSystemResult::default()
            })
        }
    }
}
fn terrane_atomic_replace(path: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let name = path.file_name().and_then(std::ffi::OsStr::to_str).unwrap_or("file");
    let mut attempt = 0_u32;
    loop {
        let temporary = parent
            .join(format!(".{name}.terrane-{}-{attempt}", std::process::id()));
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
            Err(
                error,
            ) if error.kind() == std::io::ErrorKind::AlreadyExists && attempt < 32 => {
                attempt += 1;
            }
            Err(error) => return Err(error),
        }
    }
}
#[cfg(target_os = "linux")]
fn terrane_open_beneath(
    base: &std::path::Path,
    child: &std::path::Path,
    cross: bool,
) -> std::io::Result<String> {
    use std::os::fd::{AsRawFd as _, FromRawFd as _};
    use std::os::unix::ffi::OsStrExt as _;
    #[repr(C)]
    struct OpenHow {
        flags: u64,
        mode: u64,
        resolve: u64,
    }
    unsafe extern "C" {
        fn syscall(number: isize, ...) -> isize;
    }
    const SYS_OPENAT2: isize = 437;
    const O_PATH: u64 = 0o10000000;
    const O_CLOEXEC: u64 = 0o2000000;
    const RESOLVE_NO_XDEV: u64 = 0x01;
    const RESOLVE_NO_MAGICLINKS: u64 = 0x02;
    const RESOLVE_NO_SYMLINKS: u64 = 0x04;
    const RESOLVE_BENEATH: u64 = 0x08;
    let directory = std::fs::File::open(base)?;
    let child = std::ffi::CString::new(child.as_os_str().as_bytes())
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path contains NUL",
        ))?;
    let how = OpenHow {
        flags: O_PATH | O_CLOEXEC,
        mode: 0,
        resolve: RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS | RESOLVE_NO_SYMLINKS
            | if cross { 0 } else { RESOLVE_NO_XDEV },
    };
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
    let opened = unsafe { std::fs::File::from_raw_fd(descriptor as i32) };
    std::fs::read_link(format!("/proc/self/fd/{}", opened.as_raw_fd()))
        .map(|path| path.to_string_lossy().into_owned())
}
#[cfg(not(target_os = "linux"))]
fn terrane_open_beneath(
    _: &std::path::Path,
    _: &std::path::Path,
    _: bool,
) -> std::io::Result<String> {
    Err(
        std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "race-resistant beneath traversal is unavailable in this target profile",
        ),
    )
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
        "exists" => {
            terrane_pack(TerraneSystemResult {
                flag: path.try_exists().unwrap_or(false),
                ..TerraneSystemResult::default()
            })
        }
        "metadata" => terrane_metadata(path, follow),
        "canonical" => {
            match std::fs::canonicalize(path) {
                Ok(value) => {
                    terrane_pack(TerraneSystemResult {
                        text: value.to_string_lossy().into_owned(),
                        ..TerraneSystemResult::default()
                    })
                }
                Err(error) => terrane_io_error(error),
            }
        }
        "read-link" => {
            match std::fs::read_link(path) {
                Ok(value) => {
                    terrane_pack(TerraneSystemResult {
                        text: value.to_string_lossy().into_owned(),
                        ..TerraneSystemResult::default()
                    })
                }
                Err(error) => terrane_io_error(error),
            }
        }
        "read" => {
            let Some(limit) = limit.as_usize() else {
                return terrane_io_error(
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid read limit",
                    ),
                );
            };
            match std::fs::read(path) {
                Ok(value) if value.len() <= limit => {
                    terrane_pack(TerraneSystemResult {
                        number: value.len() as i128,
                        data: value,
                        ..TerraneSystemResult::default()
                    })
                }
                Ok(_) => {
                    terrane_io_error(
                        std::io::Error::new(
                            std::io::ErrorKind::FileTooLarge,
                            "file exceeds declared read limit",
                        ),
                    )
                }
                Err(error) => terrane_io_error(error),
            }
        }
        "atomic-write" => {
            match terrane_atomic_replace(path, &data) {
                Ok(()) => terrane_pack(TerraneSystemResult::default()),
                Err(error) => terrane_io_error(error),
            }
        }
        "remove" => {
            match std::fs::remove_file(path) {
                Ok(()) => terrane_pack(TerraneSystemResult::default()),
                Err(error) => terrane_io_error(error),
            }
        }
        "rename" => {
            match std::fs::rename(path, &other) {
                Ok(()) => terrane_pack(TerraneSystemResult::default()),
                Err(error) => terrane_io_error(error),
            }
        }
        "beneath" => {
            match terrane_open_beneath(path, std::path::Path::new(&other), cross) {
                Ok(value) => {
                    terrane_pack(TerraneSystemResult {
                        text: value,
                        ..TerraneSystemResult::default()
                    })
                }
                Err(error) => terrane_io_error(error),
            }
        }
        _ => {
            terrane_io_error(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "unknown filesystem operation",
                ),
            )
        }
    }
}
#[allow(
    dead_code,
    reason = "process intrinsics are selected independently of filesystem intrinsics"
)]
#[cfg(unix)]
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    use std::os::unix::ffi::OsStrExt as _;
    value
        .into_string()
        .map_or_else(
            |raw| format!("raw:{}", terrane_hex(raw.as_bytes())),
            |text| format!("text:{text}"),
        )
}
#[allow(
    dead_code,
    reason = "process intrinsics are selected independently of filesystem intrinsics"
)]
#[cfg(not(unix))]
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    value
        .into_string()
        .map_or_else(
            |raw| {
                let units = raw
                    .encode_wide()
                    .flat_map(u16::to_le_bytes)
                    .collect::<Vec<_>>();
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
    value.strip_prefix("raw:").map(terrane_unhex).unwrap_or_default()
}
#[allow(
    dead_code,
    reason = "process intrinsics are selected independently of filesystem intrinsics"
)]
fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}
#[allow(
    dead_code,
    reason = "process intrinsics are selected independently of filesystem intrinsics"
)]
fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [
            terrane_platform_value(name),
            terrane_platform_value(value),
        ])
        .collect()
}
#[allow(
    dead_code,
    reason = "process intrinsics are selected independently of filesystem intrinsics"
)]
fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}
// Source: case.trn
// Namespace: conformance/filesystem-facilities
fn main() {
    let fs: Filesystem = Filesystem::terrane_construct();
    let target: Path = Path::terrane_construct(
        String::from("terrane-filesystem-case.txt"),
    );
    let written: OperationResult = filesystem_write_atomic(
        fs.clone(),
        target.clone(),
        Vec::from([99, 111, 110, 116, 101, 110, 116]),
    );
    println!("{}", terrane_scalar_support::scalar_text(&written.failed));
    let exists: bool = filesystem_exists(fs.clone(), target.clone());
    println!("{}", terrane_scalar_support::scalar_text(&exists));
    let metadata: FileMetadata = filesystem_metadata(fs.clone(), target.clone());
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&metadata.failed),
        terrane_scalar_support::scalar_text(&metadata.kind),
        terrane_scalar_support::scalar_text(&metadata.size),
        terrane_scalar_support::scalar_text(&metadata.readonly)
    );
    let data: FileData = filesystem_read_bounded(
        fs.clone(),
        target.clone(),
        terrane_int_support::Int::from(7_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&data.failed),
        terrane_scalar_support::scalar_text(&data.completed)
    );
    let opened: std::sync::Arc<std::sync::Mutex<FileHandle>> = std::sync::Arc::new(
        std::sync::Mutex::new(open_file(target.clone(), true, false, false, false)),
    );
    let streamed: FileData = file_read(
        std::sync::Arc::downgrade(&opened),
        terrane_int_support::Int::from(16_i128),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&streamed.failed),
        terrane_scalar_support::scalar_text(&streamed.completed),
        terrane_scalar_support::scalar_text(&streamed.end)
    );
    let closing: FileHandle = open_file(target.clone(), true, false, false, false);
    let closed: OperationResult = file_close(closing);
    println!("{}", terrane_scalar_support::scalar_text(&closed.failed));
    let lexical_input: Path = Path::terrane_construct(
        String::from("missing/../terrane-filesystem-case.txt"),
    );
    let lexical: Path = normalise_path(lexical_input.clone());
    let canonical: PathResult = filesystem_canonical(fs.clone(), target.clone());
    println!("{}", terrane_scalar_support::scalar_text(&lexical.text));
    println!("{}", terrane_scalar_support::scalar_text(&canonical.failed));
    let base: Path = Path::terrane_construct(String::from("."));
    let escape: Path = Path::terrane_construct(String::from("../Cargo.toml"));
    let escaped: PathResult = filesystem_open_beneath(
        fs.clone(),
        base.clone(),
        escape.clone(),
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&escaped.failed));
    let removed: OperationResult = filesystem_remove(fs.clone(), target.clone());
    println!("{}", terrane_scalar_support::scalar_text(&removed.failed));
}
// Source: standard/filesystem.trn
// Namespace: standard/filesystem
#[derive(Clone)]
pub struct OperationResult {
    pub failed: bool,
    pub message: String,
}
impl OperationResult {
    pub fn terrane_construct(failure: bool, detail: String) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
        };
        value.construct(failure, detail);
        value
    }
    pub fn construct(&mut self, failure: bool, detail: String) {
        self.failed = failure;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct PathResult {
    pub resolved: Path,
    pub failed: bool,
    pub message: String,
}
impl PathResult {
    pub fn terrane_construct(target: Path, failure: bool, detail: String) -> Self {
        let mut value = Self {
            resolved: Path::terrane_construct(String::from("")),
            failed: false,
            message: String::from(""),
        };
        value.construct(target, failure, detail);
        value
    }
    pub fn construct(&mut self, target: Path, failure: bool, detail: String) {
        self.resolved = target.clone();
        self.failed = failure;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct FileMetadata {
    pub kind: String,
    pub size: terrane_int_support::Int,
    pub readonly: bool,
    pub permission_detail: String,
    pub failed: bool,
    pub message: String,
}
impl FileMetadata {
    pub fn terrane_construct(
        kind: String,
        size: terrane_int_support::Int,
        readonly: bool,
        permission_detail: String,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            kind: String::from("other"),
            size: terrane_int_support::Int::from(0_i128),
            readonly: false,
            permission_detail: String::from(""),
            failed: false,
            message: String::from(""),
        };
        value.construct(kind, size, readonly, permission_detail, failure, detail);
        value
    }
    pub fn construct(
        &mut self,
        kind: String,
        size: terrane_int_support::Int,
        readonly: bool,
        permission_detail: String,
        failure: bool,
        detail: String,
    ) {
        self.kind = kind;
        self.size = size.clone();
        self.readonly = readonly;
        self.permission_detail = permission_detail;
        self.failed = failure;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct FileData {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl FileData {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(data, completed, end, failure, detail);
        value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failure: bool,
        detail: String,
    ) {
        self.data = data;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failure;
        self.message = detail;
    }
}
pub struct FileHandle {
    pub handle: TerranePlatformStreamHandle,
    pub failed: bool,
    pub message: String,
}
impl FileHandle {
    pub fn terrane_construct(
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            handle: Default::default(),
            failed: false,
            message: String::from(""),
        };
        value.construct(raw, failure, detail);
        value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
    ) {
        self.handle = raw;
        self.failed = failure;
        self.message = detail;
    }
    pub fn destruct(&self) {
        terrane_platform_release(&self.handle);
    }
}
impl Drop for FileHandle {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub fn open_file(
    target: Path,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> FileHandle {
    let raw: TerranePlatformOpenResult = terrane_platform_open_file(
        target.text,
        readable,
        writable,
        create,
        truncate,
    );
    return FileHandle::terrane_construct(
        raw.handle.clone().clone(),
        raw.failed,
        raw.message.clone().clone(),
    );
}
pub fn file_read(
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
    limit: terrane_int_support::Int,
) -> FileData {
    let raw: TerranePlatformReadResult = terrane_platform_read(
        &{
            let __terrane_owner = file.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
        limit,
    );
    return FileData::terrane_construct(
        raw.data.clone().clone(),
        raw.completed.clone(),
        raw.end,
        raw.failed,
        raw.message.clone().clone(),
    );
}
pub fn file_write(
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
    data: Vec<u8>,
) -> FileData {
    let offset: i64 = 0;
    let raw: TerranePlatformWriteResult = terrane_platform_write(
        &{
            let __terrane_owner = file.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
        &data,
        terrane_int_support::Int::from(offset.clone()),
    );
    return FileData::terrane_construct(
        data,
        raw.completed.clone(),
        false,
        raw.failed,
        raw.message.clone().clone(),
    );
}
pub fn file_flush(
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
) -> OperationResult {
    let raw: TerranePlatformUnitResult = terrane_platform_flush(
        &{
            let __terrane_owner = file.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
    );
    return OperationResult::terrane_construct(raw.failed, raw.message.clone().clone());
}
pub fn file_sync_data(
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
) -> OperationResult {
    let raw: TerranePlatformUnitResult = terrane_platform_sync_data(
        &{
            let __terrane_owner = file.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
    );
    return OperationResult::terrane_construct(raw.failed, raw.message.clone().clone());
}
pub fn file_sync_all(
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
) -> OperationResult {
    let raw: TerranePlatformUnitResult = terrane_platform_sync_all(
        &{
            let __terrane_owner = file.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
    );
    return OperationResult::terrane_construct(raw.failed, raw.message.clone().clone());
}
pub fn file_close(file: FileHandle) -> OperationResult {
    let raw: TerranePlatformUnitResult = terrane_platform_close(&file.handle);
    return OperationResult::terrane_construct(raw.failed, raw.message.clone().clone());
}
#[derive(Clone)]
pub struct Filesystem {}
impl Filesystem {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
pub fn filesystem_exists(capability: Filesystem, target: Path) -> bool {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("exists"),
        target.text,
        String::from(""),
        Vec::from([]),
        terrane_int_support::Int::from(0_i128),
        false,
        false,
    );
    return terrane_system_result_bool(&record);
}
pub fn filesystem_metadata(capability: Filesystem, target: Path) -> FileMetadata {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("metadata"),
        target.text,
        String::from(""),
        Vec::from([]),
        terrane_int_support::Int::from(0_i128),
        true,
        false,
    );
    let details: Vec<String> = terrane_string_support::split(
        &terrane_system_result_text(&record),
        &String::from("|"),
    );
    return FileMetadata::terrane_construct(
        details
            .get(
                terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/filesystem::filesystem-metadata (filesystem.trn:112:27)",
                            ),
                    )),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/filesystem::filesystem-metadata (filesystem.trn:112:27)",
                            ),
                    )),
            })
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at(
                        "/standard/filesystem::filesystem-metadata (filesystem.trn:112:27)",
                    ),
            )),
        terrane_system_result_int(&record),
        terrane_system_result_bool(&record),
        details
            .get(
                terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(1_i128),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/filesystem::filesystem-metadata (filesystem.trn:112:84)",
                            ),
                    )),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(1_i128),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/filesystem::filesystem-metadata (filesystem.trn:112:84)",
                            ),
                    )),
            })
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at(
                        "/standard/filesystem::filesystem-metadata (filesystem.trn:112:84)",
                    ),
            )),
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
pub fn filesystem_symlink_metadata(
    capability: Filesystem,
    target: Path,
) -> FileMetadata {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("metadata"),
        target.text,
        String::from(""),
        Vec::from([]),
        terrane_int_support::Int::from(0_i128),
        false,
        false,
    );
    let details: Vec<String> = terrane_string_support::split(
        &terrane_system_result_text(&record),
        &String::from("|"),
    );
    return FileMetadata::terrane_construct(
        details
            .get(
                terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/filesystem::filesystem-symlink-metadata (filesystem.trn:117:27)",
                            ),
                    )),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/filesystem::filesystem-symlink-metadata (filesystem.trn:117:27)",
                            ),
                    )),
            })
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at(
                        "/standard/filesystem::filesystem-symlink-metadata (filesystem.trn:117:27)",
                    ),
            )),
        terrane_system_result_int(&record),
        terrane_system_result_bool(&record),
        details
            .get(
                terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(1_i128),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/filesystem::filesystem-symlink-metadata (filesystem.trn:117:84)",
                            ),
                    )),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(1_i128),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/filesystem::filesystem-symlink-metadata (filesystem.trn:117:84)",
                            ),
                    )),
            })
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at(
                        "/standard/filesystem::filesystem-symlink-metadata (filesystem.trn:117:84)",
                    ),
            )),
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
pub fn filesystem_canonical(capability: Filesystem, target: Path) -> PathResult {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("canonical"),
        target.text,
        String::from(""),
        Vec::from([]),
        terrane_int_support::Int::from(0_i128),
        false,
        false,
    );
    let canonical: Path = Path::terrane_construct(terrane_system_result_text(&record));
    return PathResult::terrane_construct(
        canonical.clone(),
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
pub fn filesystem_read_link(capability: Filesystem, target: Path) -> PathResult {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("read-link"),
        target.text,
        String::from(""),
        Vec::from([]),
        terrane_int_support::Int::from(0_i128),
        false,
        false,
    );
    let linked: Path = Path::terrane_construct(terrane_system_result_text(&record));
    return PathResult::terrane_construct(
        linked.clone(),
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
pub fn filesystem_open_beneath(
    capability: Filesystem,
    directory: Path,
    relative: Path,
    cross_filesystem: bool,
) -> PathResult {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("beneath"),
        directory.text,
        relative.text,
        Vec::from([]),
        terrane_int_support::Int::from(0_i128),
        false,
        cross_filesystem,
    );
    let resolved: Path = Path::terrane_construct(terrane_system_result_text(&record));
    return PathResult::terrane_construct(
        resolved.clone(),
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
pub fn filesystem_read_bounded(
    capability: Filesystem,
    target: Path,
    limit: terrane_int_support::Int,
) -> FileData {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("read"),
        target.text,
        String::from(""),
        Vec::from([]),
        limit.clone(),
        false,
        false,
    );
    return FileData::terrane_construct(
        terrane_system_result_bytes(&record),
        terrane_system_result_int(&record),
        true,
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
pub fn filesystem_write_atomic(
    capability: Filesystem,
    target: Path,
    data: Vec<u8>,
) -> OperationResult {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("atomic-write"),
        target.text,
        String::from(""),
        data,
        terrane_int_support::Int::from(0_i128),
        false,
        false,
    );
    return OperationResult::terrane_construct(
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
pub fn filesystem_rename(
    capability: Filesystem,
    source: Path,
    destination: Path,
) -> OperationResult {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("rename"),
        source.text,
        destination.text,
        Vec::from([]),
        terrane_int_support::Int::from(0_i128),
        false,
        false,
    );
    return OperationResult::terrane_construct(
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
pub fn filesystem_remove(capability: Filesystem, target: Path) -> OperationResult {
    let _ = &capability;
    let record: String = terrane_filesystem_call(
        String::from("remove"),
        target.text,
        String::from(""),
        Vec::from([]),
        terrane_int_support::Int::from(0_i128),
        false,
        false,
    );
    return OperationResult::terrane_construct(
        terrane_system_result_failed(&record),
        terrane_system_result_message(&record),
    );
}
// Source: standard/paths.trn
// Namespace: standard/paths
#[derive(Clone)]
pub struct Path {
    pub text: String,
}
impl Path {
    pub fn terrane_construct(input: String) -> Self {
        let mut value = Self { text: String::from("") };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: String) {
        self.text = input;
    }
}
pub fn path_components(subject: Path) -> terrane_collection_support::List<String> {
    let parts: Vec<String> = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    let mut result: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
        let part: String = parts
            .get(
                terrane_collection_support::index_from_int(&index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/paths::path-components (paths.trn:17:16)"),
                    )),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: terrane_collection_support::index_from_int(&index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/paths::path-components (paths.trn:17:16)"),
                    )),
            })
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/paths::path-components (paths.trn:17:16)"),
            ));
        if part != String::from("") {
            result.append(part);
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result.clone();
}
pub fn path_is_absolute(subject: Path) -> bool {
    return subject.text.starts_with(&String::from("/"));
}
pub fn normalise_path(subject: Path) -> Path {
    let parts: Vec<String> = terrane_string_support::split(
        &subject.text,
        &String::from("/"),
    );
    let absolute: bool = path_is_absolute(subject.clone());
    let mut kept: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut count: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    let mut part_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while part_index.clone() < terrane_int_support::Int::from(parts.len() as i128) {
        let part: String = parts
            .get(
                terrane_collection_support::index_from_int(&part_index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/paths::normalise-path (paths.trn:33:16)"),
                    )),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: terrane_collection_support::index_from_int(&part_index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/paths::normalise-path (paths.trn:33:16)"),
                    )),
            })
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/paths::normalise-path (paths.trn:33:16)"),
            ));
        if part != String::from("") && part != String::from(".") {
            if part == String::from("..") {
                if count.clone() > terrane_int_support::Int::from(0_i128)
                    && kept
                        .get_or_error(
                            terrane_collection_support::index_from_int(
                                    &(count.clone() - terrane_int_support::Int::from(1_i128)),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/paths::normalise-path (paths.trn:36:34)"),
                                )),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/paths::normalise-path (paths.trn:36:34)"),
                        )) != String::from("..")
                {
                    count = count.clone() - terrane_int_support::Int::from(1_i128);
                } else {
                    if !absolute {
                        if count.clone()
                            < terrane_int_support::Int::from(
                                terrane_int_support::Int::from(kept.length()),
                            )
                        {
                            kept.set(
                                    terrane_collection_support::index_from_int(&count.clone())
                                        .unwrap_or_else(|error| __terrane_uncaught(
                                            TerraneError::from(error)
                                                .at("/standard/paths::normalise-path (paths.trn:41:29)"),
                                        )),
                                    part,
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/paths::normalise-path (paths.trn:41:29)"),
                                ));
                        } else {
                            kept.append(part);
                        }
                        count = count.clone() + terrane_int_support::Int::from(1_i128);
                    }
                }
            } else {
                if count.clone()
                    < terrane_int_support::Int::from(
                        terrane_int_support::Int::from(kept.length()),
                    )
                {
                    kept.set(
                            terrane_collection_support::index_from_int(&count.clone())
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/paths::normalise-path (paths.trn:47:21)"),
                                )),
                            part,
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/paths::normalise-path (paths.trn:47:21)"),
                        ));
                } else {
                    kept.append(part);
                }
                count = count.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
        part_index = part_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < count.clone() {
        if result != String::from("") {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&kept
            .get_or_error(terrane_collection_support::index_from_int(&index.clone())
            .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::normalise-path (paths.trn:57:33)")))).unwrap_or_else(|
            error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::normalise-path (paths.trn:57:33)"))))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    if absolute {
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result)
        );
    }
    if result == String::from("") && absolute {
        result = String::from("/");
    }
    return Path::terrane_construct(result);
}
pub fn path_name(subject: Path) -> String {
    let normal: Path = normalise_path(subject.clone());
    let parts: terrane_collection_support::List<String> = path_components(
        normal.clone(),
    );
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return String::from("");
    }
    return parts
        .get_or_error(
            terrane_collection_support::index_from_int(
                    &(terrane_int_support::Int::from(
                        terrane_int_support::Int::from(parts.length()),
                    ) - terrane_int_support::Int::from(1_i128)),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/paths::path-name (paths.trn:70:12)"),
                )),
        )
        .unwrap_or_else(|error| __terrane_uncaught(
            TerraneError::from(error).at("/standard/paths::path-name (paths.trn:70:12)"),
        ));
}
pub fn path_parent(subject: Path) -> Path {
    let normal: Path = normalise_path(subject.clone());
    let parts: terrane_collection_support::List<String> = path_components(
        normal.clone(),
    );
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return normal.clone();
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
            - terrane_int_support::Int::from(1_i128)
    {
        if result != String::from("") {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("/"))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&parts
            .get_or_error(terrane_collection_support::index_from_int(&index.clone())
            .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-parent (paths.trn:82:33)")))).unwrap_or_else(|
            error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-parent (paths.trn:82:33)"))))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let absolute: bool = path_is_absolute(normal.clone());
    if absolute {
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("/")),
            terrane_scalar_support::scalar_text(&result)
        );
    }
    return Path::terrane_construct(result);
}
pub fn path_stem(subject: Path) -> String {
    let current: String = path_name(subject.clone());
    let pieces: Vec<String> = terrane_string_support::split(
        &current,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return current;
    }
    let mut result: String = String::from("");
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(pieces.len() as i128)
            - terrane_int_support::Int::from(1_i128)
    {
        if index.clone() > terrane_int_support::Int::from(0_i128) {
            result = format!(
                "{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&String::from("."))
            );
        }
        result = format!(
            "{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&pieces
            .get(terrane_collection_support::index_from_int(&index.clone())
            .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-stem (paths.trn:99:33)")))).cloned()
            .ok_or(terrane_collection_support::IndexError { index :
            terrane_collection_support::index_from_int(&index.clone()).unwrap_or_else(|
            error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-stem (paths.trn:99:33)"))) }).unwrap_or_else(|
            error | __terrane_uncaught(TerraneError::from(error)
            .at("/standard/paths::path-stem (paths.trn:99:33)"))))
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return result;
}
pub fn path_extension(subject: Path) -> String {
    let current: String = path_name(subject.clone());
    let pieces: Vec<String> = terrane_string_support::split(
        &current,
        &String::from("."),
    );
    if terrane_int_support::Int::from(pieces.len() as i128)
        <= terrane_int_support::Int::from(1_i128)
    {
        return String::from("");
    }
    return pieces
        .get(
            terrane_collection_support::index_from_int(
                    &(terrane_int_support::Int::from(pieces.len() as i128)
                        - terrane_int_support::Int::from(1_i128)),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/paths::path-extension (paths.trn:108:12)"),
                )),
        )
        .cloned()
        .ok_or(terrane_collection_support::IndexError {
            index: terrane_collection_support::index_from_int(
                    &(terrane_int_support::Int::from(pieces.len() as i128)
                        - terrane_int_support::Int::from(1_i128)),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/paths::path-extension (paths.trn:108:12)"),
                )),
        })
        .unwrap_or_else(|error| __terrane_uncaught(
            TerraneError::from(error)
                .at("/standard/paths::path-extension (paths.trn:108:12)"),
        ));
}
pub fn join_path(base: Path, child: Path) -> Path {
    let absolute: bool = path_is_absolute(child.clone());
    if absolute {
        return normalise_path(child.clone());
    }
    let mut joined: String = base.text.clone();
    if joined != String::from("") && !joined.ends_with(&String::from("/")) {
        joined = format!(
            "{}{}", terrane_scalar_support::scalar_text(&joined),
            terrane_scalar_support::scalar_text(&String::from("/"))
        );
    }
    joined = format!(
        "{}{}", terrane_scalar_support::scalar_text(&joined),
        terrane_scalar_support::scalar_text(&child.text)
    );
    let combined: Path = Path::terrane_construct(joined);
    return normalise_path(combined.clone());
}
