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
    payload: std::boxed::Box<dyn std::any::Any + Send>,
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
    pub static FUNCTIONS: [&str; 7] = [
        "/callbacks::mark-sensitive",
        "/callbacks::clear-sensitive",
        "/callbacks::read-sensitive",
        "/callbacks::clear-sensitive-optional",
        "/app::entry-value",
        "/app::request-view",
        "/app::main",
    ];
    pub static SITES: [Site; 55] = [
        /* terrane-site-row: site 0: /callbacks::mark-sensitive (callbacks/header.trn:6:5-6:31) */
        { Site { function: 0, file: 0, line: 6, column: 5, end_line: 6, end_column: 31 } },
        /* terrane-site-row: site 1: /callbacks::clear-sensitive (callbacks/header.trn:10:5-10:32) */
        { Site { function: 1, file: 0, line: 10, column: 5, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 2: /callbacks::read-sensitive (callbacks/header.trn:14:12-14:32) */
        { Site { function: 2, file: 0, line: 14, column: 12, end_line: 14, end_column: 32 } },
        /* terrane-site-row: site 3: /callbacks::clear-sensitive-optional (callbacks/header.trn:17:5-17:32) */
        { Site { function: 3, file: 0, line: 17, column: 5, end_line: 17, end_column: 32 } },
        /* terrane-site-row: site 4: /app::entry-value (src/main.trn:9:12-9:24) */
        { Site { function: 4, file: 1, line: 9, column: 12, end_line: 9, end_column: 24 } },
        /* terrane-site-row: site 5: /app::request-view (src/main.trn:12:12-12:53) */
        { Site { function: 5, file: 1, line: 12, column: 12, end_line: 12, end_column: 53 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:18:15-18:31) */
        { Site { function: 6, file: 1, line: 18, column: 15, end_line: 18, end_column: 31 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:20:13-20:50) */
        { Site { function: 6, file: 1, line: 20, column: 13, end_line: 20, end_column: 50 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:21:13-21:52) */
        { Site { function: 6, file: 1, line: 21, column: 13, end_line: 21, end_column: 52 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:22:13-22:39) */
        { Site { function: 6, file: 1, line: 22, column: 13, end_line: 22, end_column: 39 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:23:13-23:25) */
        { Site { function: 6, file: 1, line: 23, column: 13, end_line: 23, end_column: 25 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:24:27-24:63) */
        { Site { function: 6, file: 1, line: 24, column: 27, end_line: 24, end_column: 63 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:26:17-26:32) */
        { Site { function: 6, file: 1, line: 26, column: 17, end_line: 26, end_column: 32 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:27:13-27:25) */
        { Site { function: 6, file: 1, line: 27, column: 13, end_line: 27, end_column: 25 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:28:23-28:68) */
        { Site { function: 6, file: 1, line: 28, column: 23, end_line: 28, end_column: 68 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:31:13-31:25) */
        { Site { function: 6, file: 1, line: 31, column: 13, end_line: 31, end_column: 25 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:32:12-32:22) */
        { Site { function: 6, file: 1, line: 32, column: 12, end_line: 32, end_column: 22 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:33:5-33:36) */
        { Site { function: 6, file: 1, line: 33, column: 5, end_line: 33, end_column: 36 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:34:5-34:37) */
        { Site { function: 6, file: 1, line: 34, column: 5, end_line: 34, end_column: 37 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:35:13-35:24) */
        { Site { function: 6, file: 1, line: 35, column: 13, end_line: 35, end_column: 24 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:36:16-36:32) */
        { Site { function: 6, file: 1, line: 36, column: 16, end_line: 36, end_column: 32 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:36:13-36:61) */
        { Site { function: 6, file: 1, line: 36, column: 13, end_line: 36, end_column: 61 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:37:13-37:24) */
        { Site { function: 6, file: 1, line: 37, column: 13, end_line: 37, end_column: 24 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:38:13-38:42) */
        { Site { function: 6, file: 1, line: 38, column: 13, end_line: 38, end_column: 42 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:39:13-39:25) */
        { Site { function: 6, file: 1, line: 39, column: 13, end_line: 39, end_column: 25 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:41:15-41:30) */
        { Site { function: 6, file: 1, line: 41, column: 15, end_line: 41, end_column: 30 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:42:25-42:75) */
        { Site { function: 6, file: 1, line: 42, column: 25, end_line: 42, end_column: 75 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:44:12-44:45) */
        { Site { function: 6, file: 1, line: 44, column: 12, end_line: 44, end_column: 45 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:45:13-45:45) */
        { Site { function: 6, file: 1, line: 45, column: 13, end_line: 45, end_column: 45 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:46:5-46:32) */
        { Site { function: 6, file: 1, line: 46, column: 5, end_line: 46, end_column: 32 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:47:24-47:70) */
        { Site { function: 6, file: 1, line: 47, column: 24, end_line: 47, end_column: 70 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:50:24-50:74) */
        { Site { function: 6, file: 1, line: 50, column: 24, end_line: 50, end_column: 74 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:53:33-53:84) */
        { Site { function: 6, file: 1, line: 53, column: 33, end_line: 53, end_column: 84 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:57:9-57:35) */
        { Site { function: 6, file: 1, line: 57, column: 9, end_line: 57, end_column: 35 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:59:34-59:86) */
        { Site { function: 6, file: 1, line: 59, column: 34, end_line: 59, end_column: 86 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:62:36-62:82) */
        { Site { function: 6, file: 1, line: 62, column: 36, end_line: 62, end_column: 82 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:65:33-65:71) */
        { Site { function: 6, file: 1, line: 65, column: 33, end_line: 65, end_column: 71 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:66:25-66:76) */
        { Site { function: 6, file: 1, line: 66, column: 25, end_line: 66, end_column: 76 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:69:23-69:69) */
        { Site { function: 6, file: 1, line: 69, column: 23, end_line: 69, end_column: 69 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:71:16-71:36) */
        { Site { function: 6, file: 1, line: 71, column: 16, end_line: 71, end_column: 36 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:72:39-72:85) */
        { Site { function: 6, file: 1, line: 72, column: 39, end_line: 72, end_column: 85 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:75:30-75:80) */
        { Site { function: 6, file: 1, line: 75, column: 30, end_line: 75, end_column: 80 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:81:9-81:38) */
        { Site { function: 6, file: 1, line: 81, column: 9, end_line: 81, end_column: 38 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:82:17-82:39) */
        { Site { function: 6, file: 1, line: 82, column: 17, end_line: 82, end_column: 39 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:83:5-83:55) */
        { Site { function: 6, file: 1, line: 83, column: 5, end_line: 83, end_column: 55 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:84:23-84:88) */
        { Site { function: 6, file: 1, line: 84, column: 23, end_line: 84, end_column: 88 } },
        /* terrane-site-row: site 46: /app::main (src/main.trn:86:23-86:69) */
        { Site { function: 6, file: 1, line: 86, column: 23, end_line: 86, end_column: 69 } },
        /* terrane-site-row: site 47: /app::main (src/main.trn:90:14-90:45) */
        { Site { function: 6, file: 1, line: 90, column: 14, end_line: 90, end_column: 45 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:92:13-92:33) */
        { Site { function: 6, file: 1, line: 92, column: 13, end_line: 92, end_column: 33 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:96:15-96:47) */
        { Site { function: 6, file: 1, line: 96, column: 15, end_line: 96, end_column: 47 } },
        /* terrane-site-row: site 50: /app::main (src/main.trn:98:13-98:34) */
        { Site { function: 6, file: 1, line: 98, column: 13, end_line: 98, end_column: 34 } },
        /* terrane-site-row: site 51: /app::main (src/main.trn:100:20-100:50) */
        { Site { function: 6, file: 1, line: 100, column: 20, end_line: 100, end_column: 50 } },
        /* terrane-site-row: site 52: /app::main (src/main.trn:102:13-102:39) */
        { Site { function: 6, file: 1, line: 102, column: 13, end_line: 102, end_column: 39 } },
        /* terrane-site-row: site 53: /app::main (src/main.trn:107:5-107:37) */
        { Site { function: 6, file: 1, line: 107, column: 5, end_line: 107, end_column: 37 } },
        /* terrane-site-row: site 54: /app::main (src/main.trn:108:5-108:34) */
        { Site { function: 6, file: 1, line: 108, column: 5, end_line: 108, end_column: 34 } },
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
// Source: <terrane>/projected/deps/http.trn
// Namespace: deps/http
pub use std::option::Option as BorrowedOption;
pub use witness::Canvas;
pub use witness::Entry;
pub use witness::Extensions;
pub use witness::Header;
pub use http::HeaderMap;
pub use http::HeaderName;
pub use http::HeaderValue;
pub use witness::Host;
pub use witness::Request;
pub fn terrane_static_trn_4865616465724d6170_new() -> Result<
    http::HeaderMap<http::HeaderValue>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| http::HeaderMap::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "http", "http::HeaderMap<T>"))
        }
    }
}
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
