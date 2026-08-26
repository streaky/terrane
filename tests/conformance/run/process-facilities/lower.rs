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
// Namespace: conformance/process-facilities
fn main() {
    let actual: terrane_collection_support::List<PlatformString> = arguments();
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(actual
        .length())), terrane_scalar_support::scalar_text(&actual
        .get_or_error(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:10:27)")))).unwrap_or_else(|
        error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:10:27)"))).text),
        terrane_scalar_support::scalar_text(&actual
        .get_or_error(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:10:43)")))).unwrap_or_else(|
        error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:10:43)"))).text)
    );
    let ambient: terrane_collection_support::List<EnvironmentEntry> = environment();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(terrane_int_support::Int::from(ambient
        .length())) > terrane_int_support::Int::from(0_i128)))
    );
    let schema_entries: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("flag:--verbose"), String::from("value:--output")]);
    let schema: CliSchema = CliSchema::terrane_construct(schema_entries);
    let supplied: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(
        vec![
            PlatformString::terrane_construct(String::from("text:--verbose")),
            PlatformString::terrane_construct(String::from("text:--output")),
            PlatformString::terrane_construct(String::from("text:result.txt")),
            PlatformString::terrane_construct(String::from("text:input.trn")),
            PlatformString::terrane_construct(String::from("text:--unknown")),
            PlatformString::terrane_construct(String::from("text:--output"))
        ],
    );
    let parsed: CommandLine = parse_command_line(schema.clone(), supplied.clone());
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(parsed.flags
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(parsed
        .option_names.length())), terrane_scalar_support::scalar_text(&parsed
        .option_values
        .get_or_error(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:17:61)")))).unwrap_or_else(|
        error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:17:61)"))).text)
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(parsed
        .positionals.length())), terrane_scalar_support::scalar_text(&parsed.positionals
        .get_or_error(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:18:39)")))).unwrap_or_else(|
        error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:18:39)"))).text)
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(parsed
        .diagnostic_arguments.length())), terrane_scalar_support::scalar_text(&parsed
        .diagnostic_arguments
        .get_or_error(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:19:48)")))).unwrap_or_else(|
        error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:19:48)")))),
        terrane_scalar_support::scalar_text(&parsed.diagnostic_messages
        .get_or_error(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:19:80)")))).unwrap_or_else(|
        error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:19:80)"))))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&parsed.diagnostic_arguments
        .get_or_error(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:20:12)")))).unwrap_or_else(|
        error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:20:12)")))),
        terrane_scalar_support::scalar_text(&parsed.diagnostic_messages
        .get_or_error(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128))
        .unwrap_or_else(| error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:20:44)")))).unwrap_or_else(|
        error | __terrane_uncaught(TerraneError::from(error)
        .at("/conformance/process-facilities::main (case.trn:20:44)"))))
    );
    let success_code: i64 = 0;
    let invalid_code: i64 = 256;
    let success: ExitStatus = make_exit_status(
        terrane_int_support::Int::from(success_code as i128),
    );
    let invalid: ExitStatus = make_exit_status(
        terrane_int_support::Int::from(invalid_code as i128),
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&success.valid),
        terrane_scalar_support::scalar_text(&success.code),
        terrane_scalar_support::scalar_text(&invalid.valid),
        terrane_scalar_support::scalar_text(&invalid.code)
    );
}
// Source: standard/process.trn
// Namespace: standard/process
#[derive(Clone)]
pub struct PlatformString {
    pub is_text: bool,
    pub text: String,
    pub raw: Vec<u8>,
}
impl PlatformString {
    pub fn terrane_construct(encoded: String) -> Self {
        let mut value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::from([]),
        };
        value.construct(encoded);
        value
    }
    pub fn construct(&mut self, encoded: String) {
        self.is_text = terrane_platform_value_is_text(&encoded);
        self.text = terrane_platform_value_text(&encoded);
        self.raw = terrane_platform_value_bytes(&encoded);
    }
}
#[derive(Clone)]
pub struct EnvironmentEntry {
    pub name: PlatformString,
    pub value: PlatformString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: PlatformString, entry_value: PlatformString) -> Self {
        let mut value = Self {
            name: PlatformString::terrane_construct(String::from("text:")),
            value: PlatformString::terrane_construct(String::from("text:")),
        };
        value.construct(name, entry_value);
        value
    }
    pub fn construct(&mut self, name: PlatformString, entry_value: PlatformString) {
        self.name = name.clone();
        self.value = entry_value.clone();
    }
}
pub fn arguments() -> terrane_collection_support::List<PlatformString> {
    let encoded: Vec<String> = terrane_process_arguments();
    let mut values: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
        values
            .append(
                PlatformString::terrane_construct(
                    encoded
                        .get(
                            terrane_collection_support::index_from_int(&index.clone())
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/process::arguments (process.trn:33:42)"),
                                )),
                        )
                        .cloned()
                        .ok_or(terrane_collection_support::IndexError {
                            index: terrane_collection_support::index_from_int(
                                    &index.clone(),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/process::arguments (process.trn:33:42)"),
                                )),
                        })
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::arguments (process.trn:33:42)"),
                        )),
                ),
            );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return values.clone();
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String> = terrane_environment_entries();
    let mut values: terrane_collection_support::List<EnvironmentEntry> = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
        let name: PlatformString = PlatformString::terrane_construct(
            encoded
                .get(
                    terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:42:33)"),
                        )),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:42:33)"),
                        )),
                })
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/process::environment (process.trn:42:33)"),
                )),
        );
        let value: PlatformString = PlatformString::terrane_construct(
            encoded
                .get(
                    terrane_collection_support::index_from_int(
                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:43:34)"),
                        )),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: terrane_collection_support::index_from_int(
                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:43:34)"),
                        )),
                })
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/process::environment (process.trn:43:34)"),
                )),
        );
        values.append(EnvironmentEntry::terrane_construct(name.clone(), value.clone()));
        index = index.clone() + terrane_int_support::Int::from(2_i128);
    }
    return values.clone();
}
#[derive(Clone)]
pub struct CliSchema {
    pub entries: terrane_collection_support::List<String>,
}
impl CliSchema {
    pub fn terrane_construct(
        declared: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(declared);
        value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        self.entries = declared.clone();
    }
}
#[derive(Clone)]
pub struct CommandLine {
    pub flags: terrane_collection_support::List<String>,
    pub option_names: terrane_collection_support::List<String>,
    pub option_values: terrane_collection_support::List<PlatformString>,
    pub positionals: terrane_collection_support::List<PlatformString>,
    pub diagnostic_arguments: terrane_collection_support::List<terrane_int_support::Int>,
    pub diagnostic_messages: terrane_collection_support::List<String>,
}
impl CommandLine {
    pub fn terrane_construct() -> Self {
        Self {
            flags: terrane_collection_support::List::<String>::new(Vec::new()),
            option_names: terrane_collection_support::List::<String>::new(Vec::new()),
            option_values: terrane_collection_support::List::<
                PlatformString,
            >::new(Vec::new()),
            positionals: terrane_collection_support::List::<
                PlatformString,
            >::new(Vec::new()),
            diagnostic_arguments: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            diagnostic_messages: terrane_collection_support::List::<
                String,
            >::new(Vec::new()),
        }
    }
}
pub fn schema_has(schema: CliSchema, sought: String) -> bool {
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &schema.entries,
    );
    loop {
        let entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry == sought {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<PlatformString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_names: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_values: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut positionals: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut diagnostic_messages: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(supplied.length()),
        )
    {
        let argument: PlatformString = supplied
            .get_or_error(
                terrane_collection_support::index_from_int(&index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/process::parse-command-line (process.trn:78:20)",
                            ),
                    )),
            )
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/process::parse-command-line (process.trn:78:20)"),
            ));
        if !argument.is_text {
            diagnostic_arguments.append(index.clone());
            diagnostic_messages
                .append(String::from("command-line option is not Unicode text"));
        } else {
            let flag_entry: String = format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("flag:")),
                terrane_scalar_support::scalar_text(&argument.text)
            );
            let value_entry: String = format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("value:")),
                terrane_scalar_support::scalar_text(&argument.text)
            );
            if schema_has(schema.clone(), flag_entry) {
                flags.append(argument.text.clone());
            } else if schema_has(schema.clone(), value_entry) {
                if index.clone() + terrane_int_support::Int::from(1_i128)
                    >= terrane_int_support::Int::from(
                        terrane_int_support::Int::from(supplied.length()),
                    )
                {
                    diagnostic_arguments.append(index.clone());
                    diagnostic_messages.append(String::from("option requires a value"));
                } else {
                    option_names.append(argument.text.clone());
                    option_values
                        .append(
                            supplied
                                .get_or_error(
                                    terrane_collection_support::index_from_int(
                                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                        )
                                        .unwrap_or_else(|error| __terrane_uncaught(
                                            TerraneError::from(error)
                                                .at(
                                                    "/standard/process::parse-command-line (process.trn:93:43)",
                                                ),
                                        )),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at(
                                            "/standard/process::parse-command-line (process.trn:93:43)",
                                        ),
                                )),
                        );
                    index = index.clone() + terrane_int_support::Int::from(1_i128);
                }
            } else if argument.text.starts_with(&String::from("--")) {
                diagnostic_arguments.append(index.clone());
                diagnostic_messages.append(String::from("unknown option"));
            } else {
                positionals.append(argument.clone());
            }
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let mut result: CommandLine = CommandLine::terrane_construct();
    result.flags = flags.clone();
    result.option_names = option_names.clone();
    result.option_values = option_values.clone();
    result.positionals = positionals.clone();
    result.diagnostic_arguments = diagnostic_arguments.clone();
    result.diagnostic_messages = diagnostic_messages.clone();
    return result.clone();
}
#[derive(Clone)]
pub struct ExitStatus {
    pub code: terrane_int_support::Int,
    pub valid: bool,
}
impl ExitStatus {
    pub fn terrane_construct() -> Self {
        Self {
            code: terrane_int_support::Int::from(0_i128),
            valid: true,
        }
    }
}
pub fn make_exit_status(requested: terrane_int_support::Int) -> ExitStatus {
    let mut result: ExitStatus = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result.code = terrane_int_support::Int::from(255_i128);
        result.valid = false;
    } else {
        result.code = requested.clone();
    }
    return result.clone();
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
