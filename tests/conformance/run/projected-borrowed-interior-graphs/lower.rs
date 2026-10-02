// Generated deterministically by Terrane <version>.
// Runtime support:
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
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
    cause: Option<Box<TerraneError>>,
    frames: Vec<TerraneSite>,
    structured: Vec<String>,
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
                Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
                Box::new(TerraneErrorDetail {
                    message: None,
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
#[allow(dead_code, reason = "a projected dependency may expose no Result members")]
const TERRANE_DEPENDENCY_ERROR: DescriptorId = DescriptorId(0);
#[allow(dead_code, reason = "panic catching may be disabled or not crossed")]
const TERRANE_DEPENDENCY_PANIC: DescriptorId = DescriptorId(1);
#[allow(
    dead_code,
    reason = "projected type methods may be imported without being crossed"
)]
fn __terrane_dependency_panic(
    payload: Box<dyn std::any::Any + Send>,
    crate_name: &'static str,
    member: &'static str,
) -> TerraneForeignError {
    let detail = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("non-string panic payload");
    TerraneForeignError(
        TerraneError::custom_raised(
            TERRANE_DEPENDENCY_PANIC,
            format!(
                "Rust dependency `{crate_name}` member `{member}` panicked: {detail}"
            ),
            TERRANE_NO_SITE,
        ),
    )
}
mod __terrane_error_registry {
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 4] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
        "/deps/http/header::InvalidHeaderName",
        "/deps/http/header::InvalidHeaderValue",
    ];
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
    pub static FILES: [&str; 2] = ["callbacks/header.trn", "src/main.trn"];
    pub static FUNCTIONS: [&str; 6] = [
        "/callbacks::mark-sensitive",
        "/callbacks::clear-sensitive",
        "/callbacks::read-sensitive",
        "/app::entry-value",
        "/app::request-view",
        "/app::main",
    ];
    pub static SITES: [Site; 35] = [
        /* terrane-site-row: site 0: /callbacks::mark-sensitive (callbacks/header.trn:6:5-6:31) */
        { Site { function: 0, file: 0, line: 6, column: 5, end_line: 6, end_column: 31 } },
        /* terrane-site-row: site 1: /callbacks::clear-sensitive (callbacks/header.trn:10:5-10:32) */
        { Site { function: 1, file: 0, line: 10, column: 5, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 2: /callbacks::read-sensitive (callbacks/header.trn:14:12-14:32) */
        { Site { function: 2, file: 0, line: 14, column: 12, end_line: 14, end_column: 32 } },
        /* terrane-site-row: site 3: /app::entry-value (src/main.trn:9:12-9:24) */
        { Site { function: 3, file: 1, line: 9, column: 12, end_line: 9, end_column: 24 } },
        /* terrane-site-row: site 4: /app::request-view (src/main.trn:12:12-12:53) */
        { Site { function: 4, file: 1, line: 12, column: 12, end_line: 12, end_column: 53 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:15:15-15:31) */
        { Site { function: 5, file: 1, line: 15, column: 15, end_line: 15, end_column: 31 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:17:13-17:50) */
        { Site { function: 5, file: 1, line: 17, column: 13, end_line: 17, end_column: 50 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:18:13-18:52) */
        { Site { function: 5, file: 1, line: 18, column: 13, end_line: 18, end_column: 52 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:19:13-19:39) */
        { Site { function: 5, file: 1, line: 19, column: 13, end_line: 19, end_column: 39 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:20:13-20:25) */
        { Site { function: 5, file: 1, line: 20, column: 13, end_line: 20, end_column: 25 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:21:27-21:63) */
        { Site { function: 5, file: 1, line: 21, column: 27, end_line: 21, end_column: 63 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:23:17-23:32) */
        { Site { function: 5, file: 1, line: 23, column: 17, end_line: 23, end_column: 32 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:24:13-24:25) */
        { Site { function: 5, file: 1, line: 24, column: 13, end_line: 24, end_column: 25 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:25:23-25:68) */
        { Site { function: 5, file: 1, line: 25, column: 23, end_line: 25, end_column: 68 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:28:13-28:25) */
        { Site { function: 5, file: 1, line: 28, column: 13, end_line: 28, end_column: 25 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:29:12-29:22) */
        { Site { function: 5, file: 1, line: 29, column: 12, end_line: 29, end_column: 22 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:30:5-30:36) */
        { Site { function: 5, file: 1, line: 30, column: 5, end_line: 30, end_column: 36 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:31:5-31:37) */
        { Site { function: 5, file: 1, line: 31, column: 5, end_line: 31, end_column: 37 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:32:13-32:24) */
        { Site { function: 5, file: 1, line: 32, column: 13, end_line: 32, end_column: 24 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:33:16-33:32) */
        { Site { function: 5, file: 1, line: 33, column: 16, end_line: 33, end_column: 32 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:33:13-33:61) */
        { Site { function: 5, file: 1, line: 33, column: 13, end_line: 33, end_column: 61 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:34:13-34:24) */
        { Site { function: 5, file: 1, line: 34, column: 13, end_line: 34, end_column: 24 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:35:13-35:42) */
        { Site { function: 5, file: 1, line: 35, column: 13, end_line: 35, end_column: 42 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:36:13-36:25) */
        { Site { function: 5, file: 1, line: 36, column: 13, end_line: 36, end_column: 25 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:38:15-38:30) */
        { Site { function: 5, file: 1, line: 38, column: 15, end_line: 38, end_column: 30 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:39:25-39:75) */
        { Site { function: 5, file: 1, line: 39, column: 25, end_line: 39, end_column: 75 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:41:12-41:45) */
        { Site { function: 5, file: 1, line: 41, column: 12, end_line: 41, end_column: 45 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:42:13-42:45) */
        { Site { function: 5, file: 1, line: 42, column: 13, end_line: 42, end_column: 45 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:43:5-43:32) */
        { Site { function: 5, file: 1, line: 43, column: 5, end_line: 43, end_column: 32 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:44:24-44:74) */
        { Site { function: 5, file: 1, line: 44, column: 24, end_line: 44, end_column: 74 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:47:24-47:74) */
        { Site { function: 5, file: 1, line: 47, column: 24, end_line: 47, end_column: 74 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:50:33-50:71) */
        { Site { function: 5, file: 1, line: 50, column: 33, end_line: 50, end_column: 71 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:51:25-51:76) */
        { Site { function: 5, file: 1, line: 51, column: 25, end_line: 51, end_column: 76 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:54:23-54:73) */
        { Site { function: 5, file: 1, line: 54, column: 23, end_line: 54, end_column: 73 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:58:17-58:39) */
        { Site { function: 5, file: 1, line: 58, column: 17, end_line: 58, end_column: 39 } },
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
// Source: callbacks/header.trn
// Namespace: callbacks
fn mark_sensitive(header: &mut HeaderValue) -> bool {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = header.set_sensitive(true);
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::set_sensitive",
                    ),
                )
            }
        },
        0 /* terrane-site: callbacks/header.trn:6:5-6:31 */,
    );
    return true;
}
fn clear_sensitive(header: &mut HeaderValue) -> bool {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = header.set_sensitive(false);
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::set_sensitive",
                    ),
                )
            }
        },
        1 /* terrane-site: callbacks/header.trn:10:5-10:32 */,
    );
    return true;
}
fn read_sensitive(header: &HeaderValue) -> bool {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| header.is_sensitive()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::is_sensitive",
                    ),
                )
            }
        },
        2 /* terrane-site: callbacks/header.trn:14:12-14:32 */,
    );
}
// Source: src/main.trn
// Namespace: app
fn entry_value(entry: &Entry) -> terrane_int_support::Int {
    return __terrane_raised(
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| entry.value())) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Entry::value",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:9:12-9:24 */,
    );
}
fn request_view<'view>(request: &'view Request) -> witness::View<'view> {
    return __terrane_raised(
        make_view(&request.extensions().entry()),
        4 /* terrane-site: src/main.trn:12:12-12:53 */,
    );
}
fn main() {
    let request: Request = __terrane_raised(
        terrane_static_trn_52657175657374_new(terrane_int_support::Int::from(41_i128)),
        5 /* terrane-site: src/main.trn:15:15-15:31 */,
    );
    request.extensions();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | request.extensions()
        .entry().value())) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::Entry::value")) },
        6 /* terrane-site: src/main.trn:17:13-17:50 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(consume(&request
        .extensions().entry()), 7 /* terrane-site: src/main.trn:18:13-18:52 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | request.headers()
        .length())) { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Header::length")) }, 8 /* terrane-site: src/main.trn:19:13-19:39 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        9 /* terrane-site: src/main.trn:20:13-20:25 */))
    );
    let selected: Option<Entry> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                request.extensions().get::<witness::Entry>().cloned()
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "std",
                        "std::option::Option::cloned",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:21:27-21:63 */,
    );
    if selected.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | selected.as_ref()
            .expect("semantic optional narrowing").value())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Entry::value")) }, 11 /* terrane-site: src/main.trn:23:17-23:32 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        12 /* terrane-site: src/main.trn:24:13-24:25 */))
    );
    let mapped: Option<terrane_int_support::Int> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                request
                    .extensions()
                    .get::<witness::Entry>()
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = entry_value(borrowed_argument_0);
                            Ok(
                                terrane_int_support::coerce::<i64>(&callback_value)
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => {
                Ok(value.map(|value| terrane_int_support::Int::from(i128::from(value))))
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "(((request).extensions()).get::<witness::Entry>()).map",
                    ),
                )
            }
        },
        13 /* terrane-site: src/main.trn:25:23-25:68 */,
    );
    if mapped.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&mapped.as_ref()
            .expect("semantic optional narrowing").clone())
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        14 /* terrane-site: src/main.trn:28:13-28:25 */))
    );
    let mut host: Host = __terrane_raised(
        terrane_static_trn_486f7374_new(),
        15 /* terrane-site: src/main.trn:29:12-29:22 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = host
                    .base_mut()
                    .draw_circle(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<
                                    i64,
                                >(&terrane_int_support::Int::from(7_i128))
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    );
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Canvas::draw_circle",
                    ),
                )
            }
        },
        16 /* terrane-site: src/main.trn:30:5-30:36 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = host
                    .base_mut()
                    .draw_circle(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<
                                    i64,
                                >(&terrane_int_support::Int::from(11_i128))
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    );
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Canvas::draw_circle",
                    ),
                )
            }
        },
        17 /* terrane-site: src/main.trn:31:5-31:37 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | host.total())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Host::total")) }, 18 /* terrane-site: src/main.trn:32:13-32:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        __terrane_raised(terrane_static_trn_52657175657374_new(terrane_int_support::Int::from(43_i128)),
        19 /* terrane-site: src/main.trn:33:16-33:32 */).extensions().entry()
        .value())) { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Entry::value")) }, 20 /* terrane-site: src/main.trn:33:13-33:61 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(root_count(),
        21 /* terrane-site: src/main.trn:34:13-34:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(render(&request,
        request_view), 22 /* terrane-site: src/main.trn:35:13-35:42 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        23 /* terrane-site: src/main.trn:36:13-36:25 */))
    );
    let mut headers: HeaderMap = __terrane_raised(
        terrane_static_trn_4865616465724d6170_new(),
        24 /* terrane-site: src/main.trn:38:15-38:30 */,
    );
    let missing: Option<bool> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = mark_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers).get_mut(String::from(\"forwarded\"))).map",
                    ),
                )
            }
        },
        25 /* terrane-site: src/main.trn:39:25-39:75 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&missing.is_none()));
    let name: HeaderName = __terrane_raised(
        terrane_static_trn_4865616465724e616d65_from_str(String::from("forwarded")),
        26 /* terrane-site: src/main.trn:41:12-41:45 */,
    );
    let value: HeaderValue = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str(String::from("initial")),
        27 /* terrane-site: src/main.trn:42:13-42:45 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = headers.insert(name, value);
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "(headers).insert",
                    ),
                )
            }
        },
        28 /* terrane-site: src/main.trn:43:5-43:32 */,
    );
    let before: Option<bool> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = read_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers).get_mut(String::from(\"forwarded\"))).map",
                    ),
                )
            }
        },
        29 /* terrane-site: src/main.trn:44:24-44:74 */,
    );
    if before.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* before.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let marked: Option<bool> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = mark_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers).get_mut(String::from(\"forwarded\"))).map",
                    ),
                )
            }
        },
        30 /* terrane-site: src/main.trn:47:24-47:74 */,
    );
    if marked.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* marked.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let snapshot: Option<HeaderValue> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers.get_mut(String::from("forwarded")).cloned()
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "std",
                        "std::option::Option::cloned",
                    ),
                )
            }
        },
        31 /* terrane-site: src/main.trn:50:33-50:71 */,
    );
    let cleared: Option<bool> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = clear_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers).get_mut(String::from(\"forwarded\"))).map",
                    ),
                )
            }
        },
        32 /* terrane-site: src/main.trn:51:25-51:76 */,
    );
    if cleared.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* cleared.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let after: Option<bool> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = read_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers).get_mut(String::from(\"forwarded\"))).map",
                    ),
                )
            }
        },
        33 /* terrane-site: src/main.trn:54:23-54:73 */,
    );
    if after.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* after.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    if snapshot.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | snapshot.as_ref()
            .expect("semantic optional narrowing").is_sensitive())) { Ok(value) =>
            Ok(value), Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "http", "http::HeaderValue::is_sensitive")) }, 34 /* terrane-site: src/main.trn:58:17-58:39 */))
        );
    }
}
// Source: <terrane>/projected/deps/http.trn
// Namespace: deps/http
pub use std::option::Option as BorrowedOption;
pub use witness::Canvas;
pub use witness::Entry;
pub use witness::Extensions;
pub use witness::Header;
pub type HeaderMap = http::HeaderMap<http::HeaderValue>;
pub use http::HeaderName;
pub use http::HeaderValue;
pub use witness::Host;
pub use witness::Request;
pub fn terrane_static_trn_4865616465724e616d65_from_str(
    s: String,
) -> Result<HeaderName, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| <http::HeaderName as std::str::FromStr>::from_str(
            &s,
        )),
    ) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::DescriptorId(2),
                        error.to_string(),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "http", "http::HeaderName"))
        }
    }
}
pub fn terrane_static_trn_48656164657256616c7565_from_str(
    src: String,
) -> Result<HeaderValue, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| http::HeaderValue::from_str(&src)),
    ) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::DescriptorId(3),
                        error.to_string(),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "http", "http::HeaderValue"))
        }
    }
}
pub fn terrane_static_trn_4865616465724d6170_new() -> Result<
    http::HeaderMap<http::HeaderValue>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| http::HeaderMap::<http::HeaderValue>::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "http",
                    "http::HeaderMap<http::HeaderValue>",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub fn terrane_static_trn_486f7374_new() -> Result<Host, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::Host::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::Host"))
        }
    }
}
pub fn terrane_static_trn_52657175657374_new(
    value: terrane_int_support::Int,
) -> Result<Request, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::Request::new(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(payload, "witness", "witness::Request"),
            )
        }
    }
}
pub fn clone_count() -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::clone_count()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::clone_count",
                ),
            )
        }
    }
}
pub fn consume(
    entry: &Entry,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::consume(entry)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(payload, "witness", "witness::consume"),
            )
        }
    }
}
pub fn make_view<'view>(
    entry: &&'view Entry,
) -> Result<witness::View<'view>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::make_view(entry)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::make_view",
                ),
            )
        }
    }
}
pub fn render<
    F: for<'view> std::ops::Fn(&'view witness::Request) -> witness::View<'view>,
>(
    request: &Request,
    view: F,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::render(request, view)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::render"))
        }
    }
}
pub fn root_count() -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::root_count()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::root_count",
                ),
            )
        }
    }
}
