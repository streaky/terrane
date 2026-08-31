// Generated deterministically by Terrane <version>.
type TerraneSite = u32;
const TERRANE_NO_SITE: TerraneSite = u32::MAX;
#[allow(dead_code, reason = "custom descriptors are absent from some lowered programs")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DescriptorId(u16);
#[allow(
    dead_code,
    reason = "one canonical runtime enum covers every compiler-owned throwable kind"
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
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
    fn display_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => "arithmetic-overflow",
            Self::DivisionByZero => "division-by-zero",
            Self::IntegerConversionOverflow => "integer-conversion-overflow",
            Self::NegativeShiftCount => "negative-shift-count",
            Self::CoercionError => "coercion-error",
            Self::DecodeError => "decode-error",
            Self::IndexError => "index-error",
            Self::MissingKey => "missing-key",
            Self::ResourceError => "resource-error",
            Self::SourceError => "error",
        }
    }
    fn default_message(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => "fixed-width integer arithmetic overflow",
            Self::DivisionByZero => "integer division by zero",
            Self::IntegerConversionOverflow => "integer conversion overflow",
            Self::NegativeShiftCount => "negative integer shift count",
            Self::CoercionError => "coercion has no compatible result",
            Self::DecodeError => "invalid byte sequence for selected encoding",
            Self::IndexError => "collection index is out of range",
            Self::MissingKey => "collection key is absent",
            Self::ResourceError => {
                "integer shift count cannot be represented on this target"
            }
            Self::SourceError => "source error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct TerraneErrorDetail {
    message: Option<String>,
    cause: Option<Box<TerraneError>>,
    frames: Vec<TerraneSite>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    origin: TerraneSite,
    detail: Option<Box<TerraneErrorDetail>>,
}
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::size_of::< TerraneError > () == 16);
#[cfg(target_pointer_width = "64")]
const _: () = assert!(std::mem::size_of::< Result < i64, TerraneError >> () == 16);
#[allow(
    dead_code,
    reason = "one canonical runtime implementation serves every lowered error shape"
)]
impl TerraneError {
    #[cold]
    #[inline(never)]
    fn raised(kind: TerraneErrorKind, origin: TerraneSite) -> Self {
        Self { kind, origin, detail: None }
    }
    #[cold]
    #[inline(never)]
    fn raised_with_message(
        kind: TerraneErrorKind,
        message: impl Into<String>,
        origin: TerraneSite,
    ) -> Self {
        Self {
            kind,
            origin,
            detail: Some(
                Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    cause: None,
                    frames: Vec::new(),
                }),
            ),
        }
    }
    #[cold]
    #[inline(never)]
    fn with_cause(mut self, cause: TerraneError) -> Self {
        self
            .detail
            .get_or_insert_with(|| {
                Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                })
            })
            .cause = Some(Box::new(cause));
        self
    }
    #[cold]
    #[inline(never)]
    fn attributed(mut self, origin: TerraneSite) -> Self {
        debug_assert_eq!(self.origin, TERRANE_NO_SITE);
        self.origin = origin;
        self
    }
    #[cold]
    #[inline(never)]
    fn at(mut self, frame: TerraneSite) -> Self {
        self.detail
            .get_or_insert_with(|| {
                Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                })
            })
            .frames
            .push(frame);
        self
    }
    fn message(&self) -> &str {
        self.detail
            .as_ref()
            .and_then(|detail| detail.message.as_deref())
            .unwrap_or_else(|| self.kind.default_message())
    }
    #[cold]
    #[inline(never)]
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.display_name(), self.message());
        if let Some(cause) = self
            .detail
            .as_ref()
            .and_then(|detail| detail.cause.as_ref())
        {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        if self.origin != TERRANE_NO_SITE {
            rendered.push_str("\nat ");
            rendered.push_str(&__terrane_trace::render(self.origin));
        }
        if let Some(detail) = &self.detail {
            for frame in &detail.frames {
                rendered.push_str("\nat ");
                rendered.push_str(&__terrane_trace::render(*frame));
            }
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
#[allow(
    dead_code,
    reason = "fresh support failures are absent from some lowered programs"
)]
trait TerraneRaised {
    fn raised(self, origin: TerraneSite) -> TerraneError;
}
pub struct TerraneForeignError(TerraneError);
impl TerraneForeignError {
    pub fn render(&self) -> String {
        self.0.render()
    }
}
impl TerraneRaised for TerraneForeignError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        self.0.attributed(origin)
    }
}
impl TerraneRaised for terrane_int_support::ArithmeticError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        use terrane_int_support::ArithmeticError;
        match self {
            ArithmeticError::DivisionByZero => {
                TerraneError::raised(TerraneErrorKind::DivisionByZero, origin)
            }
            ArithmeticError::ArithmeticOverflow => {
                TerraneError::raised(TerraneErrorKind::ArithmeticOverflow, origin)
            }
            ArithmeticError::NegativeShiftCount => {
                TerraneError::raised(TerraneErrorKind::NegativeShiftCount, origin)
            }
            ArithmeticError::ShiftCountTooLarge => {
                TerraneError::raised(TerraneErrorKind::ResourceError, origin)
            }
            error @ (ArithmeticError::IntegerConversionOverflow
            | ArithmeticError::IntegerConversionOverflowDetail { .. }) => {
                TerraneError::raised_with_message(
                    TerraneErrorKind::IntegerConversionOverflow,
                    error.to_string(),
                    origin,
                )
            }
            error @ (ArithmeticError::InvalidRadix
            | ArithmeticError::InvalidRadixText) => {
                TerraneError::raised_with_message(
                    TerraneErrorKind::CoercionError,
                    error.to_string(),
                    origin,
                )
            }
        }
    }
}
impl TerraneRaised for terrane_string_support::DecodeError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::DecodeError,
            self.to_string(),
            origin,
        )
    }
}
impl TerraneRaised for terrane_collection_support::IndexError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::IndexError,
            self.to_string(),
            origin,
        )
    }
}
impl TerraneRaised for terrane_collection_support::MissingKey {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::MissingKey,
            self.to_string(),
            origin,
        )
    }
}
impl TerraneRaised for terrane_collection_support::RangeStepError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::SourceError,
            self.to_string(),
            origin,
        )
    }
}
#[allow(
    dead_code,
    reason = "terminating fresh failures are absent from some lowered programs"
)]
#[cold]
#[inline(never)]
fn __terrane_raise<E: TerraneRaised>(error: E, origin: TerraneSite) -> ! {
    __terrane_uncaught(error.raised(origin))
}
#[allow(
    dead_code,
    reason = "propagating failures are absent from some lowered programs"
)]
#[cold]
#[inline(never)]
fn __terrane_trace_error(error: TerraneError, frame: TerraneSite) -> TerraneError {
    error.at(frame)
}
#[allow(
    dead_code,
    reason = "terminating fresh failures are absent from some lowered programs"
)]
#[inline]
fn __terrane_raised<T, E: TerraneRaised>(
    result: Result<T, E>,
    origin: TerraneSite,
) -> T {
    result.unwrap_or_else(|error| __terrane_raise(error, origin))
}
#[allow(
    dead_code,
    reason = "fresh failure propagation is absent from some lowered programs"
)]
#[cold]
#[inline(never)]
fn __terrane_fresh_error<E: TerraneRaised>(
    error: E,
    origin: TerraneSite,
) -> TerraneError {
    error.raised(origin)
}
#[allow(
    dead_code,
    reason = "returning fresh failures are absent from some lowered programs"
)]
#[inline]
fn __terrane_raised_err<T, E: TerraneRaised>(
    result: Result<T, E>,
    origin: TerraneSite,
) -> Result<T, TerraneError> {
    result.map_err(|error| __terrane_fresh_error(error, origin))
}
macro_rules! __terrane_raised_completion {
    ($result:expr, $origin:expr) => {
        match $result { Ok(value) => value, Err(error) => { return
        TerraneCompletion::Error(__terrane_fresh_error(error, $origin)); } }
    };
}
#[allow(
    dead_code,
    reason = "terminating propagation is absent from some lowered programs"
)]
#[inline]
fn __terrane_traced<T>(result: Result<T, TerraneError>, frame: TerraneSite) -> T {
    result
        .unwrap_or_else(|error| __terrane_uncaught(__terrane_trace_error(error, frame)))
}
#[allow(
    dead_code,
    reason = "returning propagation is absent from some lowered programs"
)]
#[inline]
fn __terrane_traced_err<T>(
    result: Result<T, TerraneError>,
    frame: TerraneSite,
) -> Result<T, TerraneError> {
    result.map_err(|error| __terrane_trace_error(error, frame))
}
macro_rules! __terrane_traced_completion {
    ($result:expr, $frame:expr) => {
        match $result { Ok(value) => value, Err(error) => { return
        TerraneCompletion::Error(__terrane_trace_error(error, $frame)); } }
    };
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
mod __terrane_error_registry {
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 0] = [];
}
mod __terrane_trace {
    pub struct Site {
        pub function: u32,
        pub file: u32,
        pub line: u32,
        pub column: u32,
        pub end_line: u32,
        pub end_column: u32,
    }
    pub static FILES: [&str; 1] = ["standard/paths.trn"];
    pub static FUNCTIONS: [&str; 6] = [
        "/standard/paths::path-components",
        "/standard/paths::normalise-path",
        "/standard/paths::path-name",
        "/standard/paths::path-parent",
        "/standard/paths::path-stem",
        "/standard/paths::path-extension",
    ];
    pub static SITES: [Site; 12] = [
        {
            /* terrane-site-row: site 0: /standard/paths::path-components (standard/paths.trn:17:16-17:28) */
            Site {
                function: 0,
                file: 0,
                line: 17,
                column: 16,
                end_line: 17,
                end_column: 28,
            }
        },
        {
            /* terrane-site-row: site 1: /standard/paths::normalise-path (standard/paths.trn:33:16-33:33) */
            Site {
                function: 1,
                file: 0,
                line: 33,
                column: 16,
                end_line: 33,
                end_column: 33,
            }
        },
        {
            /* terrane-site-row: site 2: /standard/paths::normalise-path (standard/paths.trn:36:34-36:49) */
            Site {
                function: 1,
                file: 0,
                line: 36,
                column: 34,
                end_line: 36,
                end_column: 49,
            }
        },
        {
            /* terrane-site-row: site 3: /standard/paths::normalise-path (standard/paths.trn:41:29-41:50) */
            Site {
                function: 1,
                file: 0,
                line: 41,
                column: 29,
                end_line: 41,
                end_column: 50,
            }
        },
        {
            /* terrane-site-row: site 4: /standard/paths::normalise-path (standard/paths.trn:47:21-47:42) */
            Site {
                function: 1,
                file: 0,
                line: 47,
                column: 21,
                end_line: 47,
                end_column: 42,
            }
        },
        {
            /* terrane-site-row: site 5: /standard/paths::normalise-path (standard/paths.trn:57:33-57:44) */
            Site {
                function: 1,
                file: 0,
                line: 57,
                column: 33,
                end_line: 57,
                end_column: 44,
            }
        },
        {
            /* terrane-site-row: site 6: /standard/paths::path-name (standard/paths.trn:70:12-70:35) */
            Site {
                function: 2,
                file: 0,
                line: 70,
                column: 12,
                end_line: 70,
                end_column: 35,
            }
        },
        {
            /* terrane-site-row: site 7: /standard/paths::path-parent (standard/paths.trn:84:33-84:45) */
            Site {
                function: 3,
                file: 0,
                line: 84,
                column: 33,
                end_line: 84,
                end_column: 45,
            }
        },
        {
            /* terrane-site-row: site 8: /standard/paths::path-stem (standard/paths.trn:96:31-96:40) */
            Site {
                function: 4,
                file: 0,
                line: 96,
                column: 31,
                end_line: 96,
                end_column: 40,
            }
        },
        {
            /* terrane-site-row: site 9: /standard/paths::path-stem (standard/paths.trn:103:33-103:46) */
            Site {
                function: 4,
                file: 0,
                line: 103,
                column: 33,
                end_line: 103,
                end_column: 46,
            }
        },
        {
            /* terrane-site-row: site 10: /standard/paths::path-extension (standard/paths.trn:112:31-112:40) */
            Site {
                function: 5,
                file: 0,
                line: 112,
                column: 31,
                end_line: 112,
                end_column: 40,
            }
        },
        {
            /* terrane-site-row: site 11: /standard/paths::path-extension (standard/paths.trn:114:12-114:37) */
            Site {
                function: 5,
                file: 0,
                line: 114,
                column: 12,
                end_line: 114,
                end_column: 37,
            }
        },
    ];
    #[cold]
    #[inline(never)]
    pub fn render(site: u32) -> String {
        let site = &SITES[usize::try_from(site).expect("site id must fit usize")];
        format!(
            "{} ({}:{}:{}-{}:{})", FUNCTIONS[usize::try_from(site.function)
            .expect("function id must fit usize")], FILES[usize::try_from(site.file)
            .expect("file id must fit usize")], site.line, site.column, site.end_line,
            site.end_column,
        )
    }
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
#[derive(Clone)]
struct TerranePlatformOpenResult {
    handle: TerranePlatformStreamHandle,
    failed: bool,
    message: String,
}
fn terrane_file_open_options(
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> Result<terrane_stream_abi::FileOpenOptions, String> {
    let access = match (readable, writable) {
        (true, false) => terrane_stream_abi::FileAccess::Read,
        (false, true) => terrane_stream_abi::FileAccess::Write,
        (true, true) => terrane_stream_abi::FileAccess::ReadWrite,
        (false, false) => {
            return Err("a file must be opened for reading, writing, or both".to_owned());
        }
    };
    let creation = match (create, truncate) {
        (false, false) => terrane_stream_abi::FileCreation::Existing,
        (true, false) => terrane_stream_abi::FileCreation::Create,
        (false, true) => terrane_stream_abi::FileCreation::Truncate,
        (true, true) => terrane_stream_abi::FileCreation::CreateOrTruncate,
    };
    Ok(terrane_stream_abi::FileOpenOptions {
        access,
        creation,
    })
}
fn terrane_platform_open_result(
    result: std::io::Result<terrane_stream_abi::StreamHandle>,
) -> TerranePlatformOpenResult {
    match result {
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
fn terrane_platform_open_file(
    path: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> TerranePlatformOpenResult {
    let request = match terrane_file_open_options(readable, writable, create, truncate) {
        Ok(request) => request,
        Err(message) => {
            return TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::default(),
                failed: true,
                message,
            };
        }
    };
    terrane_platform_open_result(terrane_stream_abi::open_file(&path, request))
}
fn terrane_platform_open_directory_beneath(
    base: String,
    child: String,
    cross_filesystem: bool,
) -> TerranePlatformOpenResult {
    terrane_platform_open_result(
        terrane_stream_abi::open_directory_beneath(&base, &child, cross_filesystem),
    )
}
fn terrane_platform_open_file_beneath(
    directory: &TerranePlatformStreamHandle,
    child: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> TerranePlatformOpenResult {
    let request = match terrane_file_open_options(readable, writable, create, truncate) {
        Ok(request) => request,
        Err(message) => {
            return TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::default(),
                failed: true,
                message,
            };
        }
    };
    terrane_platform_open_result(
        terrane_stream_abi::open_file_beneath(directory.abi_handle(), &child, request),
    )
}
#[derive(Clone, Default)]
pub struct TerraneFilesystemAuthority {
    _private: (),
}
fn terrane_acquire_filesystem_authority() -> TerraneFilesystemAuthority {
    TerraneFilesystemAuthority {
        _private: (),
    }
}
#[derive(Clone, Default)]
pub struct TerraneFilesystemResult {
    failed: bool,
    message: String,
    text: String,
    detail: String,
    data: Vec<u8>,
    number: i128,
    flag: bool,
}
fn terrane_io_error(error: std::io::Error) -> TerraneFilesystemResult {
    TerraneFilesystemResult {
        failed: true,
        message: error.to_string(),
        ..TerraneFilesystemResult::default()
    }
}
fn terrane_filesystem_result_failed(result: &TerraneFilesystemResult) -> bool {
    result.failed
}
fn terrane_filesystem_result_message(result: &TerraneFilesystemResult) -> String {
    result.message.clone()
}
fn terrane_filesystem_result_text(result: &TerraneFilesystemResult) -> String {
    result.text.clone()
}
fn terrane_filesystem_result_detail(result: &TerraneFilesystemResult) -> String {
    result.detail.clone()
}
fn terrane_filesystem_result_bytes(result: &TerraneFilesystemResult) -> Vec<u8> {
    result.data.clone()
}
fn terrane_filesystem_result_int(
    result: &TerraneFilesystemResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
fn terrane_filesystem_result_bool(result: &TerraneFilesystemResult) -> bool {
    result.flag
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
fn terrane_metadata(path: &std::path::Path, follow: bool) -> TerraneFilesystemResult {
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
        Err(error) => {
            TerraneFilesystemResult {
                failed: true,
                message: error.to_string(),
                text: "other".to_owned(),
                detail: "unavailable".to_owned(),
                ..TerraneFilesystemResult::default()
            }
        }
    }
}
fn terrane_atomic_replace(path: &std::path::Path, data: &[u8]) -> std::io::Result<()> {
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
                    #[cfg(unix)] std::fs::File::open(parent)?.sync_all()?;
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
fn terrane_filesystem_exists(path: String) -> TerraneFilesystemResult {
    match std::path::Path::new(&path).try_exists() {
        Ok(exists) => {
            TerraneFilesystemResult {
                flag: exists,
                ..TerraneFilesystemResult::default()
            }
        }
        Err(error) => terrane_io_error(error),
    }
}
fn terrane_filesystem_metadata(path: String, follow: bool) -> TerraneFilesystemResult {
    terrane_metadata(std::path::Path::new(&path), follow)
}
fn terrane_filesystem_realpath(path: String) -> TerraneFilesystemResult {
    match std::fs::canonicalize(path).and_then(terrane_path_text) {
        Ok(value) => {
            TerraneFilesystemResult {
                text: value,
                ..TerraneFilesystemResult::default()
            }
        }
        Err(error) => terrane_io_error(error),
    }
}
fn terrane_filesystem_read_link(path: String) -> TerraneFilesystemResult {
    match std::fs::read_link(path).and_then(terrane_path_text) {
        Ok(value) => {
            TerraneFilesystemResult {
                text: value,
                ..TerraneFilesystemResult::default()
            }
        }
        Err(error) => terrane_io_error(error),
    }
}
fn terrane_filesystem_read_bounded(
    path: String,
    limit: impl Into<terrane_int_support::Int>,
) -> TerraneFilesystemResult {
    use std::io::Read as _;
    let Some(limit) = limit.into().as_usize() else {
        return terrane_io_error(
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid read limit"),
        );
    };
    let mut value = Vec::with_capacity(limit.min(8192));
    let outcome = std::fs::File::open(path)
        .and_then(|file| {
            file.take(limit.saturating_add(1) as u64).read_to_end(&mut value)
        });
    match outcome {
        Ok(_) if value.len() <= limit => {
            TerraneFilesystemResult {
                number: value.len() as i128,
                data: value,
                ..TerraneFilesystemResult::default()
            }
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
fn terrane_filesystem_write_atomic(
    path: String,
    data: Vec<u8>,
) -> TerraneFilesystemResult {
    match terrane_atomic_replace(std::path::Path::new(&path), &data) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
fn terrane_filesystem_remove(path: String) -> TerraneFilesystemResult {
    match std::fs::remove_file(path) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
fn terrane_filesystem_rename(
    source: String,
    destination: String,
) -> TerraneFilesystemResult {
    match std::fs::rename(source, destination) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
fn terrane_path_text(path: std::path::PathBuf) -> std::io::Result<String> {
    path.into_os_string()
        .into_string()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "filesystem path is not valid Unicode",
            )
        })
}
// Source: case.trn
// Namespace: conformance/filesystem-facilities
fn sync_file(
    capability: Filesystem,
    output: std::sync::Weak<std::sync::Mutex<FileHandle>>,
) -> bool {
    let flushed: OperationResult = file_flush(capability.clone(), output.clone());
    let data: OperationResult = file_sync_data(capability.clone(), output.clone());
    let all: OperationResult = file_sync_all(capability.clone(), output.clone());
    return flushed.failed || data.failed || all.failed;
}
fn main() {
    let fs: Filesystem = filesystem_capability();
    let target: Path = Path::terrane_construct(
        String::from("terrane-filesystem-case.txt"),
    );
    let written: OperationResult = filesystem_write_atomic(
        fs.clone(),
        target.clone(),
        Vec::from([99, 111, 110, 116, 101, 110, 116]),
    );
    println!("{}", terrane_scalar_support::scalar_text(&written.failed));
    let exists: ExistenceResult = filesystem_exists(fs.clone(), target.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&exists.exists),
        terrane_scalar_support::scalar_text(&exists.failed)
    );
    let metadata: FileMetadata = filesystem_metadata(fs.clone(), target.clone());
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&metadata.failed),
        terrane_scalar_support::scalar_text(&metadata.kind),
        terrane_scalar_support::scalar_text(&metadata.size),
        terrane_scalar_support::scalar_text(&metadata.readonly),
        terrane_scalar_support::scalar_text(&(metadata.permission_detail ==
        String::from("")))
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
        std::sync::Mutex::new(
            open_file(fs.clone(), target.clone(), true, false, false, false),
        ),
    );
    let streamed: FileData = file_read(
        fs.clone(),
        std::sync::Arc::downgrade(&opened),
        terrane_int_support::Int::from(16_i128),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&streamed.failed),
        terrane_scalar_support::scalar_text(&streamed.completed),
        terrane_scalar_support::scalar_text(&streamed.end)
    );
    let closing: FileHandle = open_file(
        fs.clone(),
        target.clone(),
        true,
        false,
        false,
        false,
    );
    let closed: OperationResult = file_close(fs.clone(), closing);
    println!("{}", terrane_scalar_support::scalar_text(&closed.failed));
    let lexical_input: Path = Path::terrane_construct(
        String::from("missing/../terrane-filesystem-case.txt"),
    );
    let lexical: Path = normalise_path(lexical_input);
    let canonical: PathResult = filesystem_canonical(fs.clone(), target.clone());
    let real: PathResult = filesystem_realpath(fs.clone(), target.clone());
    println!("{}", terrane_scalar_support::scalar_text(&lexical.text));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&canonical.failed),
        terrane_scalar_support::scalar_text(&(canonical.resolved.text == lexical.text)),
        terrane_scalar_support::scalar_text(&(real.resolved.text == canonical.resolved
        .text))
    );
    let link: Path = Path::terrane_construct(
        String::from("terrane-filesystem-case-link.txt"),
    );
    let link_metadata: FileMetadata = filesystem_symlink_metadata(
        fs.clone(),
        link.clone(),
    );
    let linked: PathResult = filesystem_read_link(fs.clone(), link.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&link_metadata.failed),
        terrane_scalar_support::scalar_text(&link_metadata.kind)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&linked.failed),
        terrane_scalar_support::scalar_text(&linked.resolved.text)
    );
    let renamed_target: Path = Path::terrane_construct(
        String::from("terrane-filesystem-case-renamed.txt"),
    );
    let renamed: OperationResult = filesystem_rename(
        fs.clone(),
        target.clone(),
        renamed_target.clone(),
    );
    let restored: OperationResult = filesystem_rename(
        fs.clone(),
        renamed_target.clone(),
        target.clone(),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&renamed.failed),
        terrane_scalar_support::scalar_text(&restored.failed)
    );
    let base: Path = Path::terrane_construct(String::from("."));
    let escape: Path = Path::terrane_construct(String::from("../Cargo.toml"));
    let escaped: DirectoryHandle = filesystem_open_beneath(
        fs.clone(),
        base.clone(),
        escape,
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&escaped.failed));
    let current: Path = Path::terrane_construct(String::from("."));
    let root: std::sync::Arc<std::sync::Mutex<DirectoryHandle>> = std::sync::Arc::new(
        std::sync::Mutex::new(
            filesystem_open_beneath(fs.clone(), base.clone(), current, false),
        ),
    );
    let manifest: Path = Path::terrane_construct(
        String::from("terrane-filesystem-case.txt"),
    );
    let relative_file: std::sync::Arc<std::sync::Mutex<FileHandle>> = std::sync::Arc::new(
        std::sync::Mutex::new(
            open_file_beneath(
                fs.clone(),
                std::sync::Arc::downgrade(&root),
                manifest,
                true,
                false,
                false,
                false,
            ),
        ),
    );
    let relative_data: FileData = file_read(
        fs.clone(),
        std::sync::Arc::downgrade(&relative_file),
        terrane_int_support::Int::from(8_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&relative_data.failed),
        terrane_scalar_support::scalar_text(&relative_data.completed)
    );
    let partial_target: std::sync::Arc<std::sync::Mutex<FileHandle>> = std::sync::Arc::new(
        std::sync::Mutex::new(
            open_file(fs.clone(), target.clone(), false, true, false, true),
        ),
    );
    let partial: FileData = file_write(
        fs.clone(),
        std::sync::Arc::downgrade(&partial_target),
        Vec::from([97, 98, 99]),
        terrane_int_support::Int::from(1_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&partial.failed),
        terrane_scalar_support::scalar_text(&partial.completed)
    );
    let partial_read: FileData = filesystem_read_bounded(
        fs.clone(),
        target.clone(),
        terrane_int_support::Int::from(3_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&partial_read.failed),
        terrane_scalar_support::scalar_text(&partial_read.completed)
    );
    let durable_target: std::sync::Arc<std::sync::Mutex<FileHandle>> = std::sync::Arc::new(
        std::sync::Mutex::new(
            open_file(fs.clone(), target.clone(), false, true, false, false),
        ),
    );
    let sync_failed: bool = sync_file(
        fs.clone(),
        std::sync::Arc::downgrade(&durable_target),
    );
    println!("{}", terrane_scalar_support::scalar_text(&sync_failed));
    let link_removed: OperationResult = filesystem_remove(fs.clone(), link.clone());
    println!("{}", terrane_scalar_support::scalar_text(&link_removed.failed));
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
pub struct ExistenceResult {
    pub exists: bool,
    pub failed: bool,
    pub message: String,
}
impl ExistenceResult {
    pub fn terrane_construct(exists: bool, failure: bool, detail: String) -> Self {
        let mut value = Self {
            exists: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(exists, failure, detail);
        value
    }
    pub fn construct(&mut self, exists: bool, failure: bool, detail: String) {
        self.exists = exists;
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
pub struct DirectoryHandle {
    pub handle: TerranePlatformStreamHandle,
    pub failed: bool,
    pub message: String,
}
impl DirectoryHandle {
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
impl Drop for DirectoryHandle {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub fn open_file(
    capability: Filesystem,
    target: Path,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> FileHandle {
    let _ = &capability;
    let raw: TerranePlatformOpenResult = terrane_platform_open_file(
        target.text,
        readable,
        writable,
        create,
        truncate,
    );
    let failure: bool = raw.failed;
    let detail: String = raw.message.clone().clone();
    let acquired: TerranePlatformStreamHandle = raw.handle.clone().clone();
    return FileHandle::terrane_construct(acquired, failure, detail);
}
pub fn file_read(
    capability: Filesystem,
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
    limit: terrane_int_support::Int,
) -> FileData {
    let _ = &capability;
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
    capability: Filesystem,
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
    data: Vec<u8>,
    offset: terrane_int_support::Int,
) -> FileData {
    let _ = &capability;
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
    capability: Filesystem,
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
) -> OperationResult {
    let _ = &capability;
    let raw: TerranePlatformUnitResult = terrane_platform_flush(
        &{
            let __terrane_owner = file.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
    );
    return OperationResult::terrane_construct(raw.failed, raw.message.clone().clone());
}
pub fn file_sync_data(
    capability: Filesystem,
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
) -> OperationResult {
    let _ = &capability;
    let raw: TerranePlatformUnitResult = terrane_platform_sync_data(
        &{
            let __terrane_owner = file.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
    );
    return OperationResult::terrane_construct(raw.failed, raw.message.clone().clone());
}
pub fn file_sync_all(
    capability: Filesystem,
    file: std::sync::Weak<std::sync::Mutex<FileHandle>>,
) -> OperationResult {
    let _ = &capability;
    let raw: TerranePlatformUnitResult = terrane_platform_sync_all(
        &{
            let __terrane_owner = file.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
    );
    return OperationResult::terrane_construct(raw.failed, raw.message.clone().clone());
}
pub fn file_close(capability: Filesystem, file: FileHandle) -> OperationResult {
    let _ = &capability;
    let raw: TerranePlatformUnitResult = terrane_platform_close(&file.handle);
    return OperationResult::terrane_construct(raw.failed, raw.message.clone().clone());
}
#[derive(Clone)]
pub struct Filesystem {
    pub authority: TerraneFilesystemAuthority,
}
impl Filesystem {
    pub fn terrane_construct(authority: TerraneFilesystemAuthority) -> Self {
        let mut value = Self {
            authority: Default::default(),
        };
        value.construct(authority);
        value
    }
    pub fn construct(&mut self, authority: TerraneFilesystemAuthority) {
        self.authority = authority;
    }
}
pub fn filesystem_capability() -> Filesystem {
    let authority: TerraneFilesystemAuthority = terrane_acquire_filesystem_authority();
    return Filesystem::terrane_construct(authority);
}
pub fn filesystem_exists(capability: Filesystem, target: Path) -> ExistenceResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_exists(target.text);
    return ExistenceResult::terrane_construct(
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_metadata(capability: Filesystem, target: Path) -> FileMetadata {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_metadata(target.text, true);
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record),
        terrane_filesystem_result_int(&record),
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_detail(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_symlink_metadata(
    capability: Filesystem,
    target: Path,
) -> FileMetadata {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_metadata(
        target.text,
        false,
    );
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record),
        terrane_filesystem_result_int(&record),
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_detail(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_canonical(capability: Filesystem, target: Path) -> PathResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_realpath(target.text);
    let resolved: Path = Path::terrane_construct(
        terrane_filesystem_result_text(&record),
    );
    return PathResult::terrane_construct(
        resolved,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_realpath(capability: Filesystem, target: Path) -> PathResult {
    return filesystem_canonical(capability.clone(), target.clone());
}
pub fn filesystem_read_link(capability: Filesystem, target: Path) -> PathResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_read_link(target.text);
    let linked: Path = Path::terrane_construct(terrane_filesystem_result_text(&record));
    return PathResult::terrane_construct(
        linked,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_open_beneath(
    capability: Filesystem,
    directory: Path,
    relative: Path,
    cross_filesystem: bool,
) -> DirectoryHandle {
    let _ = &capability;
    let raw: TerranePlatformOpenResult = terrane_platform_open_directory_beneath(
        directory.text,
        relative.text,
        cross_filesystem,
    );
    let failure: bool = raw.failed;
    let detail: String = raw.message.clone().clone();
    let acquired: TerranePlatformStreamHandle = raw.handle.clone().clone();
    return DirectoryHandle::terrane_construct(acquired, failure, detail);
}
pub fn open_file_beneath(
    capability: Filesystem,
    directory: std::sync::Weak<std::sync::Mutex<DirectoryHandle>>,
    relative: Path,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
) -> FileHandle {
    let _ = &capability;
    let raw: TerranePlatformOpenResult = terrane_platform_open_file_beneath(
        &{
            let __terrane_owner = directory.upgrade().expect("reference expired");
            __terrane_owner.lock().expect("reference lock poisoned").handle.clone()
        },
        relative.text,
        readable,
        writable,
        create,
        truncate,
    );
    let failure: bool = raw.failed;
    let detail: String = raw.message.clone().clone();
    let acquired: TerranePlatformStreamHandle = raw.handle.clone().clone();
    return FileHandle::terrane_construct(acquired, failure, detail);
}
pub fn filesystem_read_bounded(
    capability: Filesystem,
    target: Path,
    limit: terrane_int_support::Int,
) -> FileData {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_read_bounded(
        target.text,
        limit,
    );
    return FileData::terrane_construct(
        terrane_filesystem_result_bytes(&record),
        terrane_filesystem_result_int(&record),
        true,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_write_atomic(
    capability: Filesystem,
    target: Path,
    data: Vec<u8>,
) -> OperationResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_write_atomic(
        target.text,
        data,
    );
    return OperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_rename(
    capability: Filesystem,
    source: Path,
    destination: Path,
) -> OperationResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_rename(
        source.text,
        destination.text,
    );
    return OperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
    );
}
pub fn filesystem_remove(capability: Filesystem, target: Path) -> OperationResult {
    let _ = &capability;
    let record: TerraneFilesystemResult = terrane_filesystem_remove(target.text);
    return OperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
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
        let part: String = __terrane_raised(
            parts
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&index.clone()),
                        0 /* terrane-site: standard/paths.trn:17:16-17:28 */,
                    ),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: __terrane_raised(
                        terrane_collection_support::index_from_int(&index.clone()),
                        0 /* terrane-site: standard/paths.trn:17:16-17:28 */,
                    ),
                }),
            0 /* terrane-site: standard/paths.trn:17:16-17:28 */,
        );
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
        let part: String = __terrane_raised(
            parts
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&part_index.clone()),
                        1 /* terrane-site: standard/paths.trn:33:16-33:33 */,
                    ),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: __terrane_raised(
                        terrane_collection_support::index_from_int(&part_index.clone()),
                        1 /* terrane-site: standard/paths.trn:33:16-33:33 */,
                    ),
                }),
            1 /* terrane-site: standard/paths.trn:33:16-33:33 */,
        );
        if part != String::from("") && part != String::from(".") {
            if part == String::from("..") {
                if count.clone() > terrane_int_support::Int::from(0_i128)
                    && __terrane_raised(
                        kept
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &(count.clone() - terrane_int_support::Int::from(1_i128)),
                                    ),
                                    2 /* terrane-site: standard/paths.trn:36:34-36:49 */,
                                ),
                            ),
                        2 /* terrane-site: standard/paths.trn:36:34-36:49 */,
                    ) != String::from("..")
                {
                    count = count.clone() - terrane_int_support::Int::from(1_i128);
                } else {
                    if !absolute {
                        if count.clone()
                            < terrane_int_support::Int::from(
                                terrane_int_support::Int::from(kept.length()),
                            )
                        {
                            __terrane_raised(
                                kept
                                    .set(
                                        __terrane_raised(
                                            terrane_collection_support::index_from_int(&count.clone()),
                                            3 /* terrane-site: standard/paths.trn:41:29-41:50 */,
                                        ),
                                        part,
                                    ),
                                3 /* terrane-site: standard/paths.trn:41:29-41:50 */,
                            );
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
                    __terrane_raised(
                        kept
                            .set(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(&count.clone()),
                                    4 /* terrane-site: standard/paths.trn:47:21-47:42 */,
                                ),
                                part,
                            ),
                        4 /* terrane-site: standard/paths.trn:47:21-47:42 */,
                    );
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
            terrane_scalar_support::scalar_text(&__terrane_raised(kept
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 5 /* terrane-site: standard/paths.trn:57:33-57:44 */)),
            5 /* terrane-site: standard/paths.trn:57:33-57:44 */))
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
    let parts: terrane_collection_support::List<String> = path_components(normal);
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(0_i128)
    {
        return String::from("");
    }
    return __terrane_raised(
        parts
            .get_or_error(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(
                            terrane_int_support::Int::from(parts.length()),
                        ) - terrane_int_support::Int::from(1_i128)),
                    ),
                    6 /* terrane-site: standard/paths.trn:70:12-70:35 */,
                ),
            ),
        6 /* terrane-site: standard/paths.trn:70:12-70:35 */,
    );
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
    if terrane_int_support::Int::from(terrane_int_support::Int::from(parts.length()))
        == terrane_int_support::Int::from(1_i128) && !path_is_absolute(normal.clone())
    {
        return Path::terrane_construct(String::from("."));
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
            terrane_scalar_support::scalar_text(&__terrane_raised(parts
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 7 /* terrane-site: standard/paths.trn:84:33-84:45 */)),
            7 /* terrane-site: standard/paths.trn:84:33-84:45 */))
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
    if terrane_int_support::Int::from(pieces.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
            pieces
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        8 /* terrane-site: standard/paths.trn:96:31-96:40 */,
                    ),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        8 /* terrane-site: standard/paths.trn:96:31-96:40 */,
                    ),
                }),
            8 /* terrane-site: standard/paths.trn:96:31-96:40 */,
        ) == String::from("")
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
            terrane_scalar_support::scalar_text(&__terrane_raised(pieces
            .get(__terrane_raised(terrane_collection_support::index_from_int(&index
            .clone()), 9 /* terrane-site: standard/paths.trn:103:33-103:46 */))
            .cloned().ok_or(terrane_collection_support::IndexError { index :
            __terrane_raised(terrane_collection_support::index_from_int(&index.clone()),
            9 /* terrane-site: standard/paths.trn:103:33-103:46 */) }),
            9 /* terrane-site: standard/paths.trn:103:33-103:46 */))
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
    if terrane_int_support::Int::from(pieces.len() as i128)
        == terrane_int_support::Int::from(2_i128)
        && __terrane_raised(
            pieces
                .get(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        10 /* terrane-site: standard/paths.trn:112:31-112:40 */,
                    ),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        10 /* terrane-site: standard/paths.trn:112:31-112:40 */,
                    ),
                }),
            10 /* terrane-site: standard/paths.trn:112:31-112:40 */,
        ) == String::from("")
    {
        return String::from("");
    }
    return __terrane_raised(
        pieces
            .get(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(pieces.len() as i128)
                            - terrane_int_support::Int::from(1_i128)),
                    ),
                    11 /* terrane-site: standard/paths.trn:114:12-114:37 */,
                ),
            )
            .cloned()
            .ok_or(terrane_collection_support::IndexError {
                index: __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &(terrane_int_support::Int::from(pieces.len() as i128)
                            - terrane_int_support::Int::from(1_i128)),
                    ),
                    11 /* terrane-site: standard/paths.trn:114:12-114:37 */,
                ),
            }),
        11 /* terrane-site: standard/paths.trn:114:12-114:37 */,
    );
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
    return normalise_path(combined);
}
