// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi, terrane-platform-support
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
    ReplaceError,
    IndexError,
    MissingKey,
    ResourceError,
    SourceError,
    Custom(DescriptorId),
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
            Self::ReplaceError => "replace-error",
            Self::IndexError => "index-error",
            Self::MissingKey => "missing-key",
            Self::ResourceError => "resource-error",
            Self::SourceError => "error",
            Self::Custom(descriptor) => {
                __terrane_error_registry::DESCRIPTORS[usize::from(descriptor.0)]
            }
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
            Self::ReplaceError => {
                "replacement search and replacement lists have different lengths"
            }
            Self::IndexError => "collection index is out of range",
            Self::MissingKey => "collection key is absent",
            Self::ResourceError => {
                "integer shift count cannot be represented on this target"
            }
            Self::SourceError => "source error",
            Self::Custom(_) => "source error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct TerraneErrorDetail {
    message: Option<String>,
    rendered_message: Option<String>,
    cause: Option<std::boxed::Box<TerraneError>>,
    frames: Vec<TerraneSite>,
    structured: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    origin: TerraneSite,
    detail: Option<std::boxed::Box<TerraneErrorDetail>>,
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                }),
            ),
        }
    }
    #[cold]
    #[inline(never)]
    fn raised_with_rendered_message(
        kind: TerraneErrorKind,
        message: impl Into<String>,
        rendered_message: Option<String>,
        origin: TerraneSite,
    ) -> Self {
        Self {
            kind,
            origin,
            detail: Some(
                std::boxed::Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    rendered_message,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                }),
            ),
        }
    }
    #[cold]
    #[inline(never)]
    fn custom_raised(
        descriptor: DescriptorId,
        message: impl Into<String>,
        origin: TerraneSite,
    ) -> Self {
        Self::raised_with_message(TerraneErrorKind::Custom(descriptor), message, origin)
    }
    #[cold]
    #[inline(never)]
    fn with_cause(mut self, cause: TerraneError) -> Self {
        self
            .detail
            .get_or_insert_with(|| {
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                })
            })
            .cause = Some(std::boxed::Box::new(cause));
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
    fn rendered_message(&self) -> &str {
        self.detail
            .as_ref()
            .and_then(|detail| detail.rendered_message.as_deref())
            .unwrap_or_else(|| self.message())
    }
    fn descriptor_name(&self) -> &str {
        self.kind.display_name()
    }
    fn source_frames(&self) -> Vec<String> {
        let mut frames = Vec::new();
        if self.origin != TERRANE_NO_SITE {
            frames.push(__terrane_trace::render(self.origin));
        }
        if let Some(detail) = &self.detail {
            frames
                .extend(
                    detail.frames.iter().map(|frame| __terrane_trace::render(*frame)),
                );
        }
        frames
    }
    fn with_structured_details(mut self, structured: Vec<String>) -> Self {
        self
            .detail
            .get_or_insert_with(|| {
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                })
            })
            .structured = structured;
        self
    }
    fn structured_details(&self) -> &[String] {
        self.detail.as_deref().map_or(&[], |detail| detail.structured.as_slice())
    }
    #[cold]
    #[inline(never)]
    fn render(&self) -> String {
        let mut rendered = format!(
            "{}: {}", self.kind.display_name(), self.rendered_message()
        );
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
impl TerraneRaised for terrane_string_support::ReplaceError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::ReplaceError,
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
    pub static DESCRIPTORS: [&str; 1] = ["/core/filesystem::temporary-path-error"];
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
    pub static FILES: [&str; 3] = ["case.trn", "core/filesystem.trn", "core/paths.trn"];
    pub static FUNCTIONS: [&str; 13] = [
        "/conformance/filesystem-facilities::exercise-directory-operations",
        "/conformance/filesystem-facilities::dropped-file",
        "/conformance/filesystem-facilities::exercise-temporary-paths",
        "/conformance/filesystem-facilities::main",
        "/core/filesystem::create-temporary-path",
        "/core/filesystem::filesystem-create-temporary-directory",
        "/core/filesystem::filesystem-create-temporary-file",
        "/core/filesystem/paths::path-components",
        "/core/filesystem/paths::normalise-path",
        "/core/filesystem/paths::path-name",
        "/core/filesystem/paths::path-parent",
        "/core/filesystem/paths::path-stem",
        "/core/filesystem/paths::path-extension",
    ];
    pub static SITES: [Site; 25] = [
        /* terrane-site-row: site 0: /conformance/filesystem-facilities::exercise-directory-operations (case.trn:30:16-30:34) */
        { Site { function: 0, file: 0, line: 30, column: 16, end_line: 30, end_column: 34 } },
        /* terrane-site-row: site 1: /conformance/filesystem-facilities::dropped-file (case.trn:82:12-82:56) */
        { Site { function: 1, file: 0, line: 82, column: 12, end_line: 82, end_column: 56 } },
        /* terrane-site-row: site 2: /conformance/filesystem-facilities::exercise-temporary-paths (case.trn:86:17-86:80) */
        { Site { function: 2, file: 0, line: 86, column: 17, end_line: 86, end_column: 80 } },
        /* terrane-site-row: site 3: /conformance/filesystem-facilities::exercise-temporary-paths (case.trn:90:12-90:59) */
        { Site { function: 2, file: 0, line: 90, column: 12, end_line: 90, end_column: 59 } },
        /* terrane-site-row: site 4: /conformance/filesystem-facilities::exercise-temporary-paths (case.trn:91:15-91:62) */
        { Site { function: 2, file: 0, line: 91, column: 15, end_line: 91, end_column: 62 } },
        /* terrane-site-row: site 5: /conformance/filesystem-facilities::exercise-temporary-paths (case.trn:99:15-99:42) */
        { Site { function: 2, file: 0, line: 99, column: 15, end_line: 99, end_column: 42 } },
        /* terrane-site-row: site 6: /conformance/filesystem-facilities::exercise-temporary-paths (case.trn:105:9-105:56) */
        { Site { function: 2, file: 0, line: 105, column: 9, end_line: 105, end_column: 56 } },
        /* terrane-site-row: site 7: /conformance/filesystem-facilities::exercise-temporary-paths (case.trn:109:15-109:51) */
        { Site { function: 2, file: 0, line: 109, column: 15, end_line: 109, end_column: 51 } },
        /* terrane-site-row: site 8: /conformance/filesystem-facilities::exercise-temporary-paths (case.trn:114:16-114:52) */
        { Site { function: 2, file: 0, line: 114, column: 16, end_line: 114, end_column: 52 } },
        /* terrane-site-row: site 9: /conformance/filesystem-facilities::main (case.trn:158:5-158:53) */
        { Site { function: 3, file: 0, line: 158, column: 5, end_line: 158, end_column: 53 } },
        /* terrane-site-row: site 10: /core/filesystem::create-temporary-path (core/filesystem.trn:80:9-80:53) */
        { Site { function: 4, file: 1, line: 80, column: 9, end_line: 80, end_column: 53 } },
        /* terrane-site-row: site 11: /core/filesystem::filesystem-create-temporary-directory (core/filesystem.trn:84:12-84:47) */
        { Site { function: 5, file: 1, line: 84, column: 12, end_line: 84, end_column: 47 } },
        /* terrane-site-row: site 12: /core/filesystem::filesystem-create-temporary-file (core/filesystem.trn:87:12-87:48) */
        { Site { function: 6, file: 1, line: 87, column: 12, end_line: 87, end_column: 48 } },
        /* terrane-site-row: site 13: /core/filesystem/paths::path-components (core/paths.trn:16:16-16:28) */
        { Site { function: 7, file: 2, line: 16, column: 16, end_line: 16, end_column: 28 } },
        /* terrane-site-row: site 14: /core/filesystem/paths::normalise-path (core/paths.trn:32:16-32:33) */
        { Site { function: 8, file: 2, line: 32, column: 16, end_line: 32, end_column: 33 } },
        /* terrane-site-row: site 15: /core/filesystem/paths::normalise-path (core/paths.trn:35:34-35:49) */
        { Site { function: 8, file: 2, line: 35, column: 34, end_line: 35, end_column: 49 } },
        /* terrane-site-row: site 16: /core/filesystem/paths::normalise-path (core/paths.trn:40:29-40:50) */
        { Site { function: 8, file: 2, line: 40, column: 29, end_line: 40, end_column: 50 } },
        /* terrane-site-row: site 17: /core/filesystem/paths::normalise-path (core/paths.trn:46:21-46:42) */
        { Site { function: 8, file: 2, line: 46, column: 21, end_line: 46, end_column: 42 } },
        /* terrane-site-row: site 18: /core/filesystem/paths::normalise-path (core/paths.trn:56:33-56:44) */
        { Site { function: 8, file: 2, line: 56, column: 33, end_line: 56, end_column: 44 } },
        /* terrane-site-row: site 19: /core/filesystem/paths::path-name (core/paths.trn:69:12-69:35) */
        { Site { function: 9, file: 2, line: 69, column: 12, end_line: 69, end_column: 35 } },
        /* terrane-site-row: site 20: /core/filesystem/paths::path-parent (core/paths.trn:83:33-83:45) */
        { Site { function: 10, file: 2, line: 83, column: 33, end_line: 83, end_column: 45 } },
        /* terrane-site-row: site 21: /core/filesystem/paths::path-stem (core/paths.trn:95:31-95:40) */
        { Site { function: 11, file: 2, line: 95, column: 31, end_line: 95, end_column: 40 } },
        /* terrane-site-row: site 22: /core/filesystem/paths::path-stem (core/paths.trn:102:33-102:46) */
        { Site { function: 11, file: 2, line: 102, column: 33, end_line: 102, end_column: 46 } },
        /* terrane-site-row: site 23: /core/filesystem/paths::path-extension (core/paths.trn:111:31-111:40) */
        { Site { function: 12, file: 2, line: 111, column: 31, end_line: 111, end_column: 40 } },
        /* terrane-site-row: site 24: /core/filesystem/paths::path-extension (core/paths.trn:113:12-113:37) */
        { Site { function: 12, file: 2, line: 113, column: 12, end_line: 113, end_column: 37 } },
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
impl From<terrane_stream_abi::StreamHandle> for TerranePlatformStreamHandle {
    fn from(handle: terrane_stream_abi::StreamHandle) -> Self {
        Self(std::sync::Arc::new(handle.id()))
    }
}
impl TerranePlatformStreamHandle {
    fn abi_handle(&self) -> terrane_stream_abi::StreamHandle {
        terrane_stream_abi::StreamHandle::from_id(*self.0)
    }
}
#[derive(Clone)]
pub struct TerranePlatformReadResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
    pub error_kind: &'static str,
    pub deadline_exceeded: bool,
}
#[derive(Clone)]
pub struct TerranePlatformWriteResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
    pub error_kind: &'static str,
    pub deadline_exceeded: bool,
}
#[derive(Clone)]
pub struct TerranePlatformUnitResult {
    pub failed: bool,
    pub message: String,
    pub error_kind: &'static str,
}
pub fn terrane_platform_read(
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
            error_kind: "invalid-input",
            deadline_exceeded: false,
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
                error_kind: "none",
                deadline_exceeded: false,
            }
        }
        Err(error) => {
            TerranePlatformReadResult {
                data: Vec::new(),
                completed: terrane_int_support::Int::from(0_i64),
                end: false,
                failed: true,
                message: error.to_string(),
                error_kind: terrane_stream_abi::io_error_kind(error.kind()),
                deadline_exceeded: false,
            }
        }
    }
}
pub fn terrane_platform_write(
    handle: &TerranePlatformStreamHandle,
    data: &[u8],
    offset: terrane_int_support::Int,
) -> TerranePlatformWriteResult {
    let Some(offset) = offset.as_usize().filter(|offset| *offset <= data.len()) else {
        return TerranePlatformWriteResult {
            data: Vec::new(),
            completed: terrane_int_support::Int::from(0_i64),
            failed: true,
            message: "stream write offset is outside the buffer".to_owned(),
            error_kind: "invalid-input",
            deadline_exceeded: false,
        };
    };
    match terrane_stream_abi::write(handle.abi_handle(), &data[offset..]) {
        Ok(completed) => {
            TerranePlatformWriteResult {
                data: Vec::new(),
                completed: terrane_int_support::Int::from(completed as i128),
                failed: false,
                message: String::new(),
                error_kind: "none",
                deadline_exceeded: false,
            }
        }
        Err(error) => {
            TerranePlatformWriteResult {
                data: Vec::new(),
                completed: terrane_int_support::Int::from(0_i64),
                failed: true,
                message: error.to_string(),
                error_kind: terrane_stream_abi::io_error_kind(error.kind()),
                deadline_exceeded: false,
            }
        }
    }
}
pub fn terrane_platform_flush(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::flush(handle.abi_handle()))
}
pub fn terrane_platform_sync_data(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_data(handle.abi_handle()))
}
pub fn terrane_platform_sync_all(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::sync_all(handle.abi_handle()))
}
pub fn terrane_platform_close(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    terrane_platform_unit(terrane_stream_abi::close(handle.abi_handle()))
}
pub fn terrane_platform_release(
    handle: &TerranePlatformStreamHandle,
) -> TerranePlatformUnitResult {
    if std::sync::Arc::strong_count(&handle.0) == 1 {
        terrane_platform_unit(terrane_stream_abi::release(handle.abi_handle()))
    } else {
        TerranePlatformUnitResult {
            failed: false,
            message: String::new(),
            error_kind: "none",
        }
    }
}
pub fn terrane_platform_unit(result: std::io::Result<()>) -> TerranePlatformUnitResult {
    match result {
        Ok(()) => {
            TerranePlatformUnitResult {
                failed: false,
                message: String::new(),
                error_kind: "none",
            }
        }
        Err(error) => {
            TerranePlatformUnitResult {
                failed: true,
                message: error.to_string(),
                error_kind: terrane_stream_abi::io_error_kind(error.kind()),
            }
        }
    }
}
#[derive(Clone)]
pub struct TerranePlatformOpenResult {
    pub handle: TerranePlatformStreamHandle,
    pub failed: bool,
    pub message: String,
    pub error_kind: &'static str,
}
pub fn terrane_file_open_options(
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
    exclusive: bool,
    append: bool,
) -> std::io::Result<terrane_stream_abi::FileOpenOptions> {
    let access = match (readable, writable) {
        (true, false) => terrane_stream_abi::FileAccess::Read,
        (false, true) => terrane_stream_abi::FileAccess::Write,
        (true, true) => terrane_stream_abi::FileAccess::ReadWrite,
        (false, false) => {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "a file must be opened for reading, writing, or both",
                ),
            );
        }
    };
    use terrane_stream_abi::FileCreation;
    let creation = match (create, truncate, exclusive) {
        (false, false, true) => FileCreation::Exclusive,
        (_, _, true) => {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "exclusive conflicts with create or truncate",
                ),
            );
        }
        (false, false, false) => FileCreation::Existing,
        (true, false, false) => FileCreation::Create,
        (false, true, false) => FileCreation::Truncate,
        (true, true, false) => FileCreation::CreateOrTruncate,
    };
    Ok(terrane_stream_abi::FileOpenOptions {
        access,
        creation,
        append,
    })
}
pub fn terrane_platform_open_result(
    result: std::io::Result<terrane_stream_abi::StreamHandle>,
) -> TerranePlatformOpenResult {
    match result {
        Ok(handle) => {
            TerranePlatformOpenResult {
                handle: handle.into(),
                failed: false,
                message: String::new(),
                error_kind: "none",
            }
        }
        Err(error) => {
            TerranePlatformOpenResult {
                handle: TerranePlatformStreamHandle::default(),
                failed: true,
                message: error.to_string(),
                error_kind: terrane_stream_abi::io_error_kind(error.kind()),
            }
        }
    }
}
pub fn terrane_platform_open_file(
    path: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
    exclusive: bool,
    append: bool,
) -> TerranePlatformOpenResult {
    terrane_platform_open_result(
        terrane_file_open_options(
                readable,
                writable,
                create,
                truncate,
                exclusive,
                append,
            )
            .and_then(|request| terrane_stream_abi::open_file(&path, request)),
    )
}
pub fn terrane_platform_open_directory_beneath(
    base: String,
    child: String,
    cross_filesystem: bool,
) -> TerranePlatformOpenResult {
    terrane_platform_open_result(
        terrane_stream_abi::open_directory_beneath(&base, &child, cross_filesystem),
    )
}
pub fn terrane_platform_open_file_beneath(
    directory: &TerranePlatformStreamHandle,
    child: String,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
    exclusive: bool,
    append: bool,
) -> TerranePlatformOpenResult {
    terrane_platform_open_result(
        terrane_file_open_options(
                readable,
                writable,
                create,
                truncate,
                exclusive,
                append,
            )
            .and_then(|request| {
                terrane_stream_abi::open_file_beneath(
                    directory.abi_handle(),
                    &child,
                    request,
                )
            }),
    )
}
#[derive(Clone, Default)]
pub struct TerraneFilesystemAuthority {
    _private: (),
}
pub fn terrane_acquire_filesystem_authority() -> TerraneFilesystemAuthority {
    TerraneFilesystemAuthority {
        _private: (),
    }
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
pub fn terrane_filesystem_result_int(
    result: &TerraneFilesystemResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
pub fn terrane_filesystem_result_bool(result: &TerraneFilesystemResult) -> bool {
    result.flag
}
pub fn terrane_filesystem_result_error_kind(result: &TerraneFilesystemResult) -> String {
    if result.failed { result.error_kind.to_owned() } else { "none".to_owned() }
}
pub fn terrane_filesystem_result_entries(
    result: &TerraneFilesystemResult,
) -> terrane_collection_support::List<String> {
    result
        .entries
        .clone()
        .unwrap_or_else(|| terrane_collection_support::List::new(Vec::new()))
}
#[cfg(unix)]
pub fn terrane_permission_detail(metadata: &std::fs::Metadata) -> String {
    use std::os::unix::fs::PermissionsExt as _;
    format!("unix-mode:{:04o}", metadata.permissions().mode() &0o7777)
}
#[cfg(not(unix))]
pub fn terrane_permission_detail(metadata: &std::fs::Metadata) -> String {
    format!("readonly:{}", metadata.permissions().readonly())
}
pub fn terrane_metadata(
    path: &std::path::Path,
    follow: bool,
) -> TerraneFilesystemResult {
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
                error_kind: terrane_stream_abi::io_error_kind(error.kind()),
                text: "other".to_owned(),
                detail: "unavailable".to_owned(),
                ..TerraneFilesystemResult::default()
            }
        }
    }
}
pub fn terrane_atomic_replace(
    path: &std::path::Path,
    data: &[u8],
) -> std::io::Result<()> {
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
pub fn terrane_filesystem_exists(path: String) -> TerraneFilesystemResult {
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
pub fn terrane_filesystem_metadata(
    path: String,
    follow: bool,
) -> TerraneFilesystemResult {
    terrane_metadata(std::path::Path::new(&path), follow)
}
pub fn terrane_filesystem_realpath(path: String) -> TerraneFilesystemResult {
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
pub fn terrane_filesystem_read_link(path: String) -> TerraneFilesystemResult {
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
pub fn terrane_filesystem_read_bounded(
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
pub fn terrane_filesystem_write_atomic(
    path: String,
    data: Vec<u8>,
) -> TerraneFilesystemResult {
    match terrane_atomic_replace(std::path::Path::new(&path), &data) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
pub type TerraneTemporaryPath = std::io::Result<
    terrane_platform_support::temporary::TemporaryPath,
>;
pub fn terrane_temporary_create(
    parent: Option<String>,
    directory: bool,
) -> TerraneTemporaryPath {
    use terrane_platform_support::temporary::TemporaryPath;
    let parent = parent.as_deref().map(std::path::Path::new);
    let owner = if directory {
        TemporaryPath::directory(parent)?
    } else {
        TemporaryPath::file(parent)?.0
    };
    if owner.path().to_str().is_none() {
        return Err(
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "temporary path is not valid UTF-8",
            ),
        );
    }
    Ok(owner)
}
pub fn terrane_temporary_status(entry: &TerraneTemporaryPath) -> Option<String> {
    entry.as_ref().err().map(ToString::to_string)
}
pub fn terrane_temporary_location(entry: &TerraneTemporaryPath) -> String {
    entry
        .as_ref()
        .expect("checked temporary creation")
        .path()
        .to_str()
        .expect("checked temporary path encoding")
        .to_owned()
}
pub fn terrane_temporary_delete(entry: TerraneTemporaryPath) -> TerraneFilesystemResult {
    match entry.and_then(terrane_platform_support::temporary::TemporaryPath::delete) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_temporary_keep(entry: TerraneTemporaryPath) -> String {
    entry
        .expect("checked temporary creation")
        .keep()
        .into_os_string()
        .into_string()
        .expect("checked temporary path encoding")
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
        return terrane_io_error(
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid directory entry limit",
            ),
        );
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
            return TerraneFilesystemResult {
                failed: true,
                message: "directory exceeds declared entry limit".to_owned(),
                error_kind: "too-many-entries",
                ..TerraneFilesystemResult::default()
            };
        }
        match entry.file_name().into_string() {
            Ok(name) => entries.push(name),
            Err(_) => {
                return terrane_io_error(
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "directory entry name is not valid UTF-8",
                    ),
                );
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
pub fn terrane_filesystem_remove_directory_tree(
    path: String,
) -> TerraneFilesystemResult {
    match std::fs::remove_dir_all(path) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_filesystem_create_symlink(
    target: String,
    link: String,
) -> TerraneFilesystemResult {
    #[cfg(target_os = "linux")]
    let result = std::os::unix::fs::symlink(target, link);
    #[cfg(not(target_os = "linux"))]
    let result = {
        let _ = (target, link);
        Err(
            std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "symbolic link creation is supported only on Linux",
            ),
        )
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
pub fn terrane_filesystem_rename(
    source: String,
    destination: String,
) -> TerraneFilesystemResult {
    match std::fs::rename(source, destination) {
        Ok(()) => TerraneFilesystemResult::default(),
        Err(error) => terrane_io_error(error),
    }
}
pub fn terrane_path_text(path: std::path::PathBuf) -> std::io::Result<String> {
    path.into_os_string()
        .into_string()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "filesystem path is not valid Unicode",
            )
        })
}
// Source: core/filesystem.trn
// Namespace: core/filesystem
pub fn filesystem_error_kind(failure: bool, category: Option<String>) -> String {
    if (&category).is_some() {
        return match category {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    }
    if failure {
        return String::from("other");
    }
    return String::from("none");
}
#[derive(Clone)]
pub struct FilesystemOperationResult {
    pub failed: bool,
    pub message: String,
    pub error_kind: String,
}
impl FilesystemOperationResult {
    pub fn terrane_construct(
        failure: bool,
        detail: String,
        category: Option<String>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            error_kind: String::from(""),
        };
        __terrane_constructed_value.construct(failure, detail, category);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) {
        {
            let __terrane_value: bool = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value: String = filesystem_error_kind(failure, category);
            self.error_kind = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct ExistenceResult {
    pub exists: bool,
    pub failed: bool,
    pub message: String,
    pub error_kind: String,
}
impl ExistenceResult {
    pub fn terrane_construct(
        exists: bool,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            exists: false,
            failed: false,
            message: String::from(""),
            error_kind: String::from(""),
        };
        __terrane_constructed_value.construct(exists, failure, detail, category);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        exists: bool,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) {
        {
            let __terrane_value: bool = exists;
            self.exists = __terrane_value;
        }
        {
            let __terrane_value: bool = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value: String = filesystem_error_kind(failure, category);
            self.error_kind = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct PathResult {
    pub resolved: Path,
    pub failed: bool,
    pub message: String,
    pub error_kind: String,
}
impl PathResult {
    pub fn terrane_construct(
        target: Path,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            resolved: Path::terrane_construct(String::from("")),
            failed: false,
            message: String::from(""),
            error_kind: String::from(""),
        };
        __terrane_constructed_value.construct(target, failure, detail, category);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        target: Path,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) {
        {
            let __terrane_value: Path = target;
            self.resolved = __terrane_value;
        }
        {
            let __terrane_value: bool = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value: String = filesystem_error_kind(failure, category);
            self.error_kind = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct TemporaryPathError {
    pub message: String,
}
impl TemporaryPathError {
    pub fn terrane_construct(detail: String) -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
        __terrane_constructed_value.construct(detail);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, detail: String) {
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
pub struct TemporaryPath {
    pub entry: Option<TerraneTemporaryPath>,
}
impl TemporaryPath {
    pub fn terrane_construct(entry: TerraneTemporaryPath) -> Self {
        let mut __terrane_constructed_value = Self { entry: None };
        __terrane_constructed_value.construct(entry);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, entry: TerraneTemporaryPath) {
        {
            let __terrane_value: TerraneTemporaryPath = entry;
            self.entry = Some(__terrane_value);
        }
    }
    pub fn location(&self) -> Path {
        return Path::terrane_construct(
            terrane_temporary_location(
                &self.entry.as_ref().expect("required field initialized"),
            ),
        );
    }
    pub fn delete(mut self) -> FilesystemOperationResult {
        let raw: TerraneFilesystemResult;
        raw = terrane_temporary_delete(
            self.entry.take().expect("required field initialized"),
        );
        return FilesystemOperationResult::terrane_construct(
            terrane_filesystem_result_failed(&raw),
            terrane_filesystem_result_message(&raw),
            Some(terrane_filesystem_result_error_kind(&raw)),
        );
    }
    pub fn keep(mut self) -> Path {
        return Path::terrane_construct(
            terrane_temporary_keep(
                self.entry.take().expect("required field initialized"),
            ),
        );
    }
}
pub fn create_temporary_path(
    parent: Option<Path>,
    directory: bool,
) -> Result<TemporaryPath, TerraneError> {
    let mut parent_text: Option<String>;
    let entry: TerraneTemporaryPath;
    let problem: Option<String>;
    parent_text = None;
    if (&parent).is_some() {
        parent_text = Some(
            match &parent {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .text
                .clone(),
        );
    }
    entry = terrane_temporary_create(parent_text, directory);
    problem = terrane_temporary_status(&entry);
    if (&problem).is_some() {
        return Err({
            let value = TemporaryPathError::terrane_construct(
                match problem {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                },
            );
            let rendered = value.render();
            let original = &value.message;
            let (message, rendered) = if &rendered == original {
                (rendered, None)
            } else {
                (original.clone(), Some(rendered))
            };
            TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                10 /* terrane-site: core/filesystem.trn:80:9-80:53 */,
            )
        });
    }
    return Ok(TemporaryPath::terrane_construct(entry));
}
pub fn filesystem_create_temporary_directory(
    capability: Filesystem,
    parent: Option<Path>,
) -> Result<TemporaryPath, TerraneError> {
    let _ = &capability;
    return Ok(
        __terrane_traced_err(
            create_temporary_path(parent, true),
            11 /* terrane-site: core/filesystem.trn:84:12-84:47 */,
        )?,
    );
}
pub fn filesystem_create_temporary_file(
    capability: Filesystem,
    parent: Option<Path>,
) -> Result<TemporaryPath, TerraneError> {
    let _ = &capability;
    return Ok(
        __terrane_traced_err(
            create_temporary_path(parent, false),
            12 /* terrane-site: core/filesystem.trn:87:12-87:48 */,
        )?,
    );
}
#[derive(Clone)]
pub struct FileMetadata {
    pub kind: String,
    pub size: terrane_int_support::Int,
    pub readonly: bool,
    pub permission_detail: String,
    pub failed: bool,
    pub message: String,
    pub error_kind: String,
}
impl FileMetadata {
    pub fn terrane_construct(
        kind: String,
        size: terrane_int_support::Int,
        readonly: bool,
        permission_detail: String,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            kind: String::from("other"),
            size: terrane_int_support::Int::from(0_i128),
            readonly: false,
            permission_detail: String::from(""),
            failed: false,
            message: String::from(""),
            error_kind: String::from(""),
        };
        __terrane_constructed_value
            .construct(
                kind,
                size,
                readonly,
                permission_detail,
                failure,
                detail,
                category,
            );
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        kind: String,
        size: terrane_int_support::Int,
        readonly: bool,
        permission_detail: String,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) {
        {
            let __terrane_value: String = kind;
            self.kind = __terrane_value;
        }
        {
            let __terrane_value: terrane_int_support::Int = size.clone();
            self.size = __terrane_value;
        }
        {
            let __terrane_value: bool = readonly;
            self.readonly = __terrane_value;
        }
        {
            let __terrane_value: String = permission_detail;
            self.permission_detail = __terrane_value;
        }
        {
            let __terrane_value: bool = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value: String = filesystem_error_kind(failure, category);
            self.error_kind = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct FileData {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
    pub error_kind: String,
}
impl FileData {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            data: Vec::<u8>::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
            error_kind: String::from(""),
        };
        __terrane_constructed_value
            .construct(data, completed, end, failure, detail, category);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failure: bool,
        detail: String,
        category: Option<String>,
    ) {
        {
            let __terrane_value: Vec<u8> = data;
            self.data = __terrane_value;
        }
        {
            let __terrane_value: terrane_int_support::Int = completed.clone();
            self.completed = __terrane_value;
        }
        {
            let __terrane_value: bool = end;
            self.end = __terrane_value;
        }
        {
            let __terrane_value: bool = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value: String = filesystem_error_kind(failure, category);
            self.error_kind = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct DirectoryListing {
    pub entries: terrane_collection_support::List<String>,
    pub failed: bool,
    pub message: String,
    pub error_kind: String,
}
impl DirectoryListing {
    pub fn terrane_construct(
        entries: terrane_collection_support::List<String>,
        failure: bool,
        detail: String,
        category: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
            failed: false,
            message: String::from(""),
            error_kind: String::from(""),
        };
        __terrane_constructed_value.construct(entries, failure, detail, category);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        entries: terrane_collection_support::List<String>,
        failure: bool,
        detail: String,
        category: String,
    ) {
        {
            let __terrane_value: terrane_collection_support::List<String> = entries;
            self.entries = __terrane_value;
        }
        {
            let __terrane_value: bool = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value: String = category;
            self.error_kind = __terrane_value;
        }
    }
}
pub struct FileHandle {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
    pub failed: bool,
    pub message: String,
    pub error_kind: String,
}
impl FileHandle {
    pub fn terrane_construct(
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
        category: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            failed: false,
            message: String::from(""),
            error_kind: String::from(""),
        };
        __terrane_constructed_value.construct(raw, failure, detail, category);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
        category: String,
    ) {
        {
            let __terrane_value: TerranePlatformStreamHandle = raw;
            self.handle = Some(__terrane_value);
        }
        {
            let __terrane_value: bool = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value: String = category;
            self.error_kind = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for FileHandle {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct DirectoryHandle {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
    pub failed: bool,
    pub message: String,
    pub error_kind: String,
}
impl DirectoryHandle {
    pub fn terrane_construct(
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
        category: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            failed: false,
            message: String::from(""),
            error_kind: String::from(""),
        };
        __terrane_constructed_value.construct(raw, failure, detail, category);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformStreamHandle,
        failure: bool,
        detail: String,
        category: String,
    ) {
        {
            let __terrane_value: TerranePlatformStreamHandle = raw;
            self.handle = Some(__terrane_value);
        }
        {
            let __terrane_value: bool = failure;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value: String = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value: String = category;
            self.error_kind = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for DirectoryHandle {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
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
    exclusive: bool,
    append: bool,
) -> FileHandle {
    let raw: TerranePlatformOpenResult;
    let failure: bool;
    let detail: String;
    let category: String;
    let acquired: TerranePlatformStreamHandle;
    let _ = &capability;
    raw = terrane_platform_open_file(
        target.text,
        readable,
        writable,
        create,
        truncate,
        exclusive,
        append,
    );
    failure = raw.failed;
    detail = raw.message.clone();
    category = raw.error_kind.to_owned();
    acquired = raw.handle.clone();
    return FileHandle::terrane_construct(acquired, failure, detail, category);
}
pub fn file_read(
    capability: Filesystem,
    file: &FileHandle,
    limit: terrane_int_support::Int,
) -> FileData {
    let raw: TerranePlatformReadResult;
    let _ = &capability;
    raw = terrane_platform_read(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
        limit,
    );
    return FileData::terrane_construct(
        raw.data.clone(),
        raw.completed.clone(),
        raw.end,
        raw.failed,
        raw.message.clone(),
        Some(raw.error_kind.to_owned()),
    );
}
pub fn file_write(
    capability: Filesystem,
    file: &FileHandle,
    data: Vec<u8>,
    offset: terrane_int_support::Int,
) -> FileData {
    let raw: TerranePlatformWriteResult;
    let _ = &capability;
    raw = terrane_platform_write(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
        &data,
        terrane_int_support::Int::from(offset.clone()),
    );
    return FileData::terrane_construct(
        data,
        raw.completed.clone(),
        false,
        raw.failed,
        raw.message.clone(),
        Some(raw.error_kind.to_owned()),
    );
}
pub fn file_flush(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw: TerranePlatformUnitResult;
    let _ = &capability;
    raw = terrane_platform_flush(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
    );
    return FilesystemOperationResult::terrane_construct(
        raw.failed,
        raw.message.clone(),
        Some(raw.error_kind.to_owned()),
    );
}
pub fn file_sync_data(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw: TerranePlatformUnitResult;
    let _ = &capability;
    raw = terrane_platform_sync_data(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
    );
    return FilesystemOperationResult::terrane_construct(
        raw.failed,
        raw.message.clone(),
        Some(raw.error_kind.to_owned()),
    );
}
pub fn file_sync_all(
    capability: Filesystem,
    file: &FileHandle,
) -> FilesystemOperationResult {
    let raw: TerranePlatformUnitResult;
    let _ = &capability;
    raw = terrane_platform_sync_all(
        &{
            let __terrane_snapshot = file
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
    );
    return FilesystemOperationResult::terrane_construct(
        raw.failed,
        raw.message.clone(),
        Some(raw.error_kind.to_owned()),
    );
}
pub fn file_close(
    capability: Filesystem,
    file: FileHandle,
) -> FilesystemOperationResult {
    let raw: TerranePlatformUnitResult;
    let _ = &capability;
    raw = terrane_platform_close(
        &file.handle.as_ref().expect("required field initialized"),
    );
    return FilesystemOperationResult::terrane_construct(
        raw.failed,
        raw.message.clone(),
        Some(raw.error_kind.to_owned()),
    );
}
#[derive(Clone)]
pub struct Filesystem {
    pub authority: Option<TerraneFilesystemAuthority>,
}
impl Filesystem {
    pub fn terrane_construct(authority: TerraneFilesystemAuthority) -> Self {
        let mut __terrane_constructed_value = Self { authority: None };
        __terrane_constructed_value.construct(authority);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, authority: TerraneFilesystemAuthority) {
        {
            let __terrane_value: TerraneFilesystemAuthority = authority;
            self.authority = Some(__terrane_value);
        }
    }
}
pub fn filesystem_capability() -> Filesystem {
    let authority: TerraneFilesystemAuthority;
    authority = terrane_acquire_filesystem_authority();
    return Filesystem::terrane_construct(authority);
}
pub fn filesystem_exists(capability: Filesystem, target: Path) -> ExistenceResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_exists(target.text);
    return ExistenceResult::terrane_construct(
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_metadata(capability: Filesystem, target: Path) -> FileMetadata {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_metadata(target.text, true);
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record),
        terrane_filesystem_result_int(&record),
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_detail(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_symlink_metadata(
    capability: Filesystem,
    target: Path,
) -> FileMetadata {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_metadata(target.text, false);
    return FileMetadata::terrane_construct(
        terrane_filesystem_result_text(&record),
        terrane_filesystem_result_int(&record),
        terrane_filesystem_result_bool(&record),
        terrane_filesystem_result_detail(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_canonical(capability: Filesystem, target: Path) -> PathResult {
    let record: TerraneFilesystemResult;
    let resolved: Path;
    let _ = &capability;
    record = terrane_filesystem_realpath(target.text);
    resolved = Path::terrane_construct(terrane_filesystem_result_text(&record));
    return PathResult::terrane_construct(
        resolved,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_realpath(capability: Filesystem, target: Path) -> PathResult {
    return filesystem_canonical(capability, target);
}
pub fn filesystem_read_link(capability: Filesystem, target: Path) -> PathResult {
    let record: TerraneFilesystemResult;
    let linked: Path;
    let _ = &capability;
    record = terrane_filesystem_read_link(target.text);
    linked = Path::terrane_construct(terrane_filesystem_result_text(&record));
    return PathResult::terrane_construct(
        linked,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_open_beneath(
    capability: Filesystem,
    directory: Path,
    relative: Path,
    cross_filesystem: bool,
) -> DirectoryHandle {
    let raw: TerranePlatformOpenResult;
    let failure: bool;
    let detail: String;
    let category: String;
    let acquired: TerranePlatformStreamHandle;
    let _ = &capability;
    raw = terrane_platform_open_directory_beneath(
        directory.text,
        relative.text,
        cross_filesystem,
    );
    failure = raw.failed;
    detail = raw.message.clone();
    category = raw.error_kind.to_owned();
    acquired = raw.handle.clone();
    return DirectoryHandle::terrane_construct(acquired, failure, detail, category);
}
pub fn open_file_beneath(
    capability: Filesystem,
    directory: &DirectoryHandle,
    relative: Path,
    readable: bool,
    writable: bool,
    create: bool,
    truncate: bool,
    exclusive: bool,
    append: bool,
) -> FileHandle {
    let raw: TerranePlatformOpenResult;
    let failure: bool;
    let detail: String;
    let category: String;
    let acquired: TerranePlatformStreamHandle;
    let _ = &capability;
    raw = terrane_platform_open_file_beneath(
        &{
            let __terrane_snapshot = directory
                .handle
                .as_ref()
                .expect("required field initialized")
                .clone();
            __terrane_snapshot
        },
        relative.text,
        readable,
        writable,
        create,
        truncate,
        exclusive,
        append,
    );
    failure = raw.failed;
    detail = raw.message.clone();
    category = raw.error_kind.to_owned();
    acquired = raw.handle.clone();
    return FileHandle::terrane_construct(acquired, failure, detail, category);
}
pub fn filesystem_read_bounded(
    capability: Filesystem,
    target: Path,
    limit: terrane_int_support::Int,
) -> FileData {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_read_bounded(target.text, limit);
    return FileData::terrane_construct(
        terrane_filesystem_result_bytes(&record),
        terrane_filesystem_result_int(&record),
        true,
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_write_atomic(
    capability: Filesystem,
    target: Path,
    data: Vec<u8>,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_write_atomic(target.text, data);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_rename(
    capability: Filesystem,
    source: Path,
    destination: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_rename(source.text, destination.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_remove(
    capability: Filesystem,
    target: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_remove(target.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_create_directory(
    capability: Filesystem,
    target: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_create_directory(target.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_create_directories(
    capability: Filesystem,
    target: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_create_directories(target.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_list_directory(
    capability: Filesystem,
    target: Path,
    limit: terrane_int_support::Int,
) -> DirectoryListing {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_list_directory(target.text, limit.clone());
    return DirectoryListing::terrane_construct(
        terrane_filesystem_result_entries(&record),
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        terrane_filesystem_result_error_kind(&record),
    );
}
pub fn filesystem_remove_directory(
    capability: Filesystem,
    target: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_remove_directory(target.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_remove_directory_tree(
    capability: Filesystem,
    target: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_remove_directory_tree(target.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_create_symlink(
    capability: Filesystem,
    target: Path,
    link: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_create_symlink(target.text, link.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
pub fn filesystem_create_hard_link(
    capability: Filesystem,
    target: Path,
    link: Path,
) -> FilesystemOperationResult {
    let record: TerraneFilesystemResult;
    let _ = &capability;
    record = terrane_filesystem_create_hard_link(target.text, link.text);
    return FilesystemOperationResult::terrane_construct(
        terrane_filesystem_result_failed(&record),
        terrane_filesystem_result_message(&record),
        Some(terrane_filesystem_result_error_kind(&record)),
    );
}
// Source: core/paths.trn
// Namespace: core/filesystem/paths
#[derive(Clone)]
pub struct Path {
    pub text: String,
}
impl Path {
    pub fn terrane_construct(input: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("") };
        __terrane_constructed_value.construct(input);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, input: String) {
        {
            let __terrane_value: String = input;
            self.text = __terrane_value;
        }
    }
}
pub fn path_components(subject: Path) -> terrane_collection_support::List<String> {
    let parts: Vec<String>;
    let mut result: terrane_collection_support::List<String>;
    let mut index: terrane_int_support::Int;
    let mut part: String;
    parts = {
        let __terrane_string_argument_0 = "/";
        terrane_string_support::split(
            &subject.text,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    result = terrane_collection_support::List::<String>::new(vec![]);
    index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = result.make_unique();
        while index < terrane_int_support::Int::from(parts.len() as i128) {
            part = __terrane_raised(
                {
                    let __terrane_receiver = &parts;
                    let __terrane_index = __terrane_raised(
                        terrane_collection_support::index_from_int(&index.clone()),
                        13 /* terrane-site: core/paths.trn:16:16-16:28 */,
                    );
                    __terrane_receiver
                        .get(__terrane_index)
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_index,
                        ))
                },
                13 /* terrane-site: core/paths.trn:16:16-16:28 */,
            );
            if part.as_str() != "" {
                {
                    let __terrane_collection_argument_0 = part;
                    __terrane_list_append_0.push(__terrane_collection_argument_0);
                };
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return result;
}
pub fn path_is_absolute(subject: Path) -> bool {
    return {
        let __terrane_string_argument_0 = "/";
        subject
            .text
            .starts_with(
                std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
            )
    };
}
pub fn normalise_path(subject: Path) -> Path {
    let parts: Vec<String>;
    let absolute: bool;
    let mut kept: terrane_collection_support::List<String>;
    let mut count: terrane_int_support::Int;
    let mut part_index: terrane_int_support::Int;
    let mut part: String;
    let mut result: String;
    let mut index: terrane_int_support::Int;
    parts = {
        let __terrane_string_argument_0 = "/";
        terrane_string_support::split(
            &subject.text,
            &std::convert::AsRef::<str>::as_ref(&__terrane_string_argument_0),
        )
    };
    absolute = path_is_absolute(subject);
    kept = terrane_collection_support::List::<String>::new(vec![]);
    count = terrane_int_support::Int::from(0_i128);
    part_index = terrane_int_support::Int::from(0_i128);
    while part_index < terrane_int_support::Int::from(parts.len() as i128) {
        part = __terrane_raised(
            {
                let __terrane_receiver = &parts;
                let __terrane_index = __terrane_raised(
                    terrane_collection_support::index_from_int(&part_index.clone()),
                    14 /* terrane-site: core/paths.trn:32:16-32:33 */,
                );
                __terrane_receiver
                    .get(__terrane_index)
                    .cloned()
                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                        __terrane_index,
                    ))
            },
            14 /* terrane-site: core/paths.trn:32:16-32:33 */,
        );
        if part.as_str() != "" && part.as_str() != "." {
            if part.as_str() == ".." {
                if count > terrane_int_support::Int::from(0_i128)
                    && __terrane_raised(
                            kept
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &(count.clone() - terrane_int_support::Int::from(1_i128)),
                                        ),
                                        15 /* terrane-site: core/paths.trn:35:34-35:49 */,
                                    ),
                                ),
                            15 /* terrane-site: core/paths.trn:35:34-35:49 */,
                        )
                        .as_str() != ".."
                {
                    count = count.clone() - terrane_int_support::Int::from(1_i128);
                } else {
                    if !absolute {
                        if count < terrane_int_support::Int::from(kept.length()) {
                            {
                                let __terrane_collection_argument_0 = __terrane_raised(
                                    terrane_collection_support::index_from_int(&count.clone()),
                                    16 /* terrane-site: core/paths.trn:40:29-40:50 */,
                                );
                                let __terrane_collection_argument_1 = part;
                                __terrane_raised(
                                    kept
                                        .set(
                                            __terrane_collection_argument_0,
                                            __terrane_collection_argument_1,
                                        ),
                                    16 /* terrane-site: core/paths.trn:40:29-40:50 */,
                                );
                            };
                        } else {
                            {
                                let __terrane_collection_argument_0 = part;
                                kept.append(__terrane_collection_argument_0);
                            };
                        }
                        count = count.clone() + terrane_int_support::Int::from(1_i128);
                    }
                }
            } else {
                if count < terrane_int_support::Int::from(kept.length()) {
                    {
                        let __terrane_collection_argument_0 = __terrane_raised(
                            terrane_collection_support::index_from_int(&count.clone()),
                            17 /* terrane-site: core/paths.trn:46:21-46:42 */,
                        );
                        let __terrane_collection_argument_1 = part;
                        __terrane_raised(
                            kept
                                .set(
                                    __terrane_collection_argument_0,
                                    __terrane_collection_argument_1,
                                ),
                            17 /* terrane-site: core/paths.trn:46:21-46:42 */,
                        );
                    };
                } else {
                    {
                        let __terrane_collection_argument_0 = part;
                        kept.append(__terrane_collection_argument_0);
                    };
                }
                count = count.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
        part_index = part_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    result = String::from("");
    index = terrane_int_support::Int::from(0_i128);
    while index < count {
        if result.as_str() != "" {
            result = {
                let mut __terrane_text = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &result,
                    &mut __terrane_text,
                );
                __terrane_text.push_str("/");
                __terrane_text
            };
        }
        result = {
            let mut __terrane_text = String::with_capacity(0);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &__terrane_raised(
                    kept
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                18 /* terrane-site: core/paths.trn:56:33-56:44 */,
                            ),
                        ),
                    18 /* terrane-site: core/paths.trn:56:33-56:44 */,
                ),
                &mut __terrane_text,
            );
            __terrane_text
        };
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    if absolute {
        result = {
            let mut __terrane_text = String::with_capacity(1);
            __terrane_text.push_str("/");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
    if result.as_str() == "" && absolute {
        result = String::from("/");
    }
    return Path::terrane_construct(result);
}
pub fn path_parent(subject: Path) -> Path {
    let normal: Path;
    let parts: terrane_collection_support::List<String>;
    let mut result: String;
    let mut index: terrane_int_support::Int;
    let absolute: bool;
    normal = normalise_path(subject);
    parts = path_components(normal.clone());
    if terrane_int_support::Int::from(parts.length())
        == terrane_int_support::Int::from(0_i128)
    {
        return normal;
    }
    if terrane_int_support::Int::from(parts.length())
        == terrane_int_support::Int::from(1_i128) && !path_is_absolute(normal.clone())
    {
        return Path::terrane_construct(String::from("."));
    }
    result = String::from("");
    index = terrane_int_support::Int::from(0_i128);
    while index
        < terrane_int_support::Int::from(parts.length())
            - terrane_int_support::Int::from(1_i128)
    {
        if result.as_str() != "" {
            result = {
                let mut __terrane_text = String::with_capacity(1);
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &result,
                    &mut __terrane_text,
                );
                __terrane_text.push_str("/");
                __terrane_text
            };
        }
        result = {
            let mut __terrane_text = String::with_capacity(0);
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &__terrane_raised(
                    parts
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                20 /* terrane-site: core/paths.trn:83:33-83:45 */,
                            ),
                        ),
                    20 /* terrane-site: core/paths.trn:83:33-83:45 */,
                ),
                &mut __terrane_text,
            );
            __terrane_text
        };
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    absolute = path_is_absolute(normal);
    if absolute {
        result = {
            let mut __terrane_text = String::with_capacity(1);
            __terrane_text.push_str("/");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &result,
                &mut __terrane_text,
            );
            __terrane_text
        };
    }
    return Path::terrane_construct(result);
}
