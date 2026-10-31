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
    pub static DESCRIPTORS: [&str; 2] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
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
    pub static FILES: [&str; 1] = ["src/main.trn"];
    pub static FUNCTIONS: [&str; 12] = [
        "/app::execute-local",
        "/app::execute-reference",
        "/app::execute-formal-reference",
        "/app::execute-optional",
        "/app::execute-deferred",
        "/app::execute-enum",
        "/app::observe-native",
        "/app::observe-captured",
        "/app::execute-outlives",
        "/app::observe-outlives",
        "/app::execute-owner",
        "/app::main",
    ];
    pub static SITES: [Site; 52] = [
        /* terrane-site-row: site 0: /app::execute-local (src/main.trn:16:18-16:34) */
        { Site { function: 0, file: 0, line: 16, column: 18, end_line: 16, end_column: 34 } },
        /* terrane-site-row: site 1: /app::execute-local (src/main.trn:17:13-17:33) */
        { Site { function: 0, file: 0, line: 17, column: 13, end_line: 17, end_column: 33 } },
        /* terrane-site-row: site 2: /app::execute-local (src/main.trn:18:12-18:35) */
        { Site { function: 0, file: 0, line: 18, column: 12, end_line: 18, end_column: 35 } },
        /* terrane-site-row: site 3: /app::execute-reference (src/main.trn:21:13-21:37) */
        { Site { function: 1, file: 0, line: 21, column: 13, end_line: 21, end_column: 37 } },
        /* terrane-site-row: site 4: /app::execute-reference (src/main.trn:22:12-22:35) */
        { Site { function: 1, file: 0, line: 22, column: 12, end_line: 22, end_column: 35 } },
        /* terrane-site-row: site 5: /app::execute-formal-reference (src/main.trn:25:12-25:31) */
        { Site { function: 2, file: 0, line: 25, column: 12, end_line: 25, end_column: 31 } },
        /* terrane-site-row: site 6: /app::execute-optional (src/main.trn:30:17-30:33) */
        { Site { function: 3, file: 0, line: 30, column: 17, end_line: 30, end_column: 33 } },
        /* terrane-site-row: site 7: /app::execute-optional (src/main.trn:32:17-32:34) */
        { Site { function: 3, file: 0, line: 32, column: 17, end_line: 32, end_column: 34 } },
        /* terrane-site-row: site 8: /app::execute-optional (src/main.trn:33:16-33:34) */
        { Site { function: 3, file: 0, line: 33, column: 16, end_line: 33, end_column: 34 } },
        /* terrane-site-row: site 9: /app::execute-deferred (src/main.trn:38:27-38:43) */
        { Site { function: 4, file: 0, line: 38, column: 27, end_line: 38, end_column: 43 } },
        /* terrane-site-row: site 10: /app::execute-deferred (src/main.trn:39:13-39:30) */
        { Site { function: 4, file: 0, line: 39, column: 13, end_line: 39, end_column: 30 } },
        /* terrane-site-row: site 11: /app::execute-deferred (src/main.trn:40:12-40:29) */
        { Site { function: 4, file: 0, line: 40, column: 12, end_line: 40, end_column: 29 } },
        /* terrane-site-row: site 12: /app::execute-enum (src/main.trn:43:12-43:26) */
        { Site { function: 5, file: 0, line: 43, column: 12, end_line: 43, end_column: 26 } },
        /* terrane-site-row: site 13: /app::observe-native (src/main.trn:46:12-46:32) */
        { Site { function: 6, file: 0, line: 46, column: 12, end_line: 46, end_column: 32 } },
        /* terrane-site-row: site 14: /app::observe-captured (src/main.trn:49:18-49:34) */
        { Site { function: 7, file: 0, line: 49, column: 18, end_line: 49, end_column: 34 } },
        /* terrane-site-row: site 15: /app::observe-captured (src/main.trn:51:16-51:36) */
        { Site { function: 7, file: 0, line: 51, column: 16, end_line: 51, end_column: 36 } },
        /* terrane-site-row: site 16: /app::observe-captured (src/main.trn:52:13-52:20) */
        { Site { function: 7, file: 0, line: 52, column: 13, end_line: 52, end_column: 20 } },
        /* terrane-site-row: site 17: /app::observe-captured (src/main.trn:53:21-53:28) */
        { Site { function: 7, file: 0, line: 53, column: 21, end_line: 53, end_column: 28 } },
        /* terrane-site-row: site 18: /app::observe-captured (src/main.trn:53:12-53:29) */
        { Site { function: 7, file: 0, line: 53, column: 12, end_line: 53, end_column: 29 } },
        /* terrane-site-row: site 19: /app::execute-outlives (src/main.trn:56:18-56:34) */
        { Site { function: 8, file: 0, line: 56, column: 18, end_line: 56, end_column: 34 } },
        /* terrane-site-row: site 20: /app::execute-outlives (src/main.trn:57:13-57:34) */
        { Site { function: 8, file: 0, line: 57, column: 13, end_line: 57, end_column: 34 } },
        /* terrane-site-row: site 21: /app::execute-outlives (src/main.trn:58:13-58:45) */
        { Site { function: 8, file: 0, line: 58, column: 13, end_line: 58, end_column: 45 } },
        /* terrane-site-row: site 22: /app::execute-outlives (src/main.trn:59:15-59:37) */
        { Site { function: 8, file: 0, line: 59, column: 15, end_line: 59, end_column: 37 } },
        /* terrane-site-row: site 23: /app::execute-outlives (src/main.trn:59:14-59:65) */
        { Site { function: 8, file: 0, line: 59, column: 14, end_line: 59, end_column: 65 } },
        /* terrane-site-row: site 24: /app::execute-outlives (src/main.trn:60:12-60:26) */
        { Site { function: 8, file: 0, line: 60, column: 12, end_line: 60, end_column: 26 } },
        /* terrane-site-row: site 25: /app::observe-outlives (src/main.trn:63:18-63:34) */
        { Site { function: 9, file: 0, line: 63, column: 18, end_line: 63, end_column: 34 } },
        /* terrane-site-row: site 26: /app::observe-outlives (src/main.trn:66:16-66:34) */
        { Site { function: 9, file: 0, line: 66, column: 16, end_line: 66, end_column: 34 } },
        /* terrane-site-row: site 27: /app::observe-outlives (src/main.trn:67:13-67:20) */
        { Site { function: 9, file: 0, line: 67, column: 13, end_line: 67, end_column: 20 } },
        /* terrane-site-row: site 28: /app::observe-outlives (src/main.trn:68:21-68:28) */
        { Site { function: 9, file: 0, line: 68, column: 21, end_line: 68, end_column: 28 } },
        /* terrane-site-row: site 29: /app::observe-outlives (src/main.trn:68:12-68:29) */
        { Site { function: 9, file: 0, line: 68, column: 12, end_line: 68, end_column: 29 } },
        /* terrane-site-row: site 30: /app::execute-owner (src/main.trn:71:13-71:41) */
        { Site { function: 10, file: 0, line: 71, column: 13, end_line: 71, end_column: 41 } },
        /* terrane-site-row: site 31: /app::execute-owner (src/main.trn:71:12-71:60) */
        { Site { function: 10, file: 0, line: 71, column: 12, end_line: 71, end_column: 60 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:74:13-74:27) */
        { Site { function: 11, file: 0, line: 74, column: 13, end_line: 74, end_column: 27 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:75:18-75:34) */
        { Site { function: 11, file: 0, line: 75, column: 18, end_line: 75, end_column: 34 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:76:13-76:42) */
        { Site { function: 11, file: 0, line: 76, column: 13, end_line: 76, end_column: 42 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:77:14-77:29) */
        { Site { function: 11, file: 0, line: 77, column: 14, end_line: 77, end_column: 29 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:77:13-77:48) */
        { Site { function: 11, file: 0, line: 77, column: 13, end_line: 77, end_column: 48 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:78:13-78:49) */
        { Site { function: 11, file: 0, line: 78, column: 13, end_line: 78, end_column: 49 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:79:13-79:35) */
        { Site { function: 11, file: 0, line: 79, column: 13, end_line: 79, end_column: 35 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:80:13-80:35) */
        { Site { function: 11, file: 0, line: 80, column: 13, end_line: 80, end_column: 35 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:81:13-81:36) */
        { Site { function: 11, file: 0, line: 81, column: 13, end_line: 81, end_column: 36 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:82:13-82:35) */
        { Site { function: 11, file: 0, line: 82, column: 13, end_line: 82, end_column: 35 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:84:13-84:28) */
        { Site { function: 11, file: 0, line: 84, column: 13, end_line: 84, end_column: 28 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:87:13-87:32) */
        { Site { function: 11, file: 0, line: 87, column: 13, end_line: 87, end_column: 32 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:93:13-93:39) */
        { Site { function: 11, file: 0, line: 93, column: 13, end_line: 93, end_column: 39 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:95:13-95:34) */
        { Site { function: 11, file: 0, line: 95, column: 13, end_line: 95, end_column: 34 } },
        /* terrane-site-row: site 46: /app::main (src/main.trn:101:13-101:30) */
        { Site { function: 11, file: 0, line: 101, column: 13, end_line: 101, end_column: 30 } },
        /* terrane-site-row: site 47: /app::main (src/main.trn:102:13-102:30) */
        { Site { function: 11, file: 0, line: 102, column: 13, end_line: 102, end_column: 30 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:103:13-103:30) */
        { Site { function: 11, file: 0, line: 103, column: 13, end_line: 103, end_column: 30 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:104:14-104:41) */
        { Site { function: 11, file: 0, line: 104, column: 14, end_line: 104, end_column: 41 } },
        /* terrane-site-row: site 50: /app::main (src/main.trn:104:13-104:60) */
        { Site { function: 11, file: 0, line: 104, column: 13, end_line: 104, end_column: 60 } },
        /* terrane-site-row: site 51: /app::main (src/main.trn:106:13-106:36) */
        { Site { function: 11, file: 0, line: 106, column: 13, end_line: 106, end_column: 36 } },
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
async fn __terrane_await<F: Future>(future: F) -> F::Output {
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();
        fn poll(
            mut self: std::pin::Pin<&mut Self>,
            context: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            if self.0 {
                std::task::Poll::Ready(())
            } else {
                self.0 = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
    YieldOnce(false).await;
    let output = future.await;
    YieldOnce(false).await;
    output
}
#[allow(
    dead_code,
    clippy::unused_async,
    reason = "executor shutdown uses one hook for both simple and cancellation-aware runtimes"
)]
async fn __terrane_wait_projected_cleanups() {}
#[allow(
    dead_code,
    reason = "async entry support is emitted for libraries without an entrypoint"
)]
async fn __terrane_async_entry<F: Future>(future: F) -> F::Output {
    future.await
}
#[allow(
    dead_code,
    reason = "shared async support also serves packages without awaited destructors"
)]
const TERRANE_RUNTIME_CLOSING: usize = 1 << usize::BITS - 1;
#[allow(
    dead_code,
    reason = "libraries can use async support without owning an executor"
)]
struct TerraneRuntimeLiveness {
    active: std::sync::atomic::AtomicUsize,
    drain: std::sync::Mutex<()>,
    changed: std::sync::Condvar,
}
thread_local! {
    static TERRANE_RUNTIME_LIVE : std::cell::RefCell < Option < (tokio::runtime::Id,
    std::sync::Arc < TerraneRuntimeLiveness >) >> = const { std::cell::RefCell::new(None)
    };
}
#[allow(
    dead_code,
    reason = "libraries can use awaited destruction without owning an executor"
)]
struct TerraneRuntimeLivenessGuard {
    live: std::sync::Arc<TerraneRuntimeLiveness>,
    previous: Option<(tokio::runtime::Id, std::sync::Arc<TerraneRuntimeLiveness>)>,
}
impl Drop for TerraneRuntimeLivenessGuard {
    fn drop(&mut self) {
        if self
            .live
            .active
            .fetch_or(TERRANE_RUNTIME_CLOSING, std::sync::atomic::Ordering::AcqRel) != 0
        {
            let drained = self
                .live
                .drain
                .lock()
                .expect("destructor drain lock poisoned");
            let _drained = self
                .live
                .changed
                .wait_while(
                    drained,
                    |()| {
                        self.live.active.load(std::sync::atomic::Ordering::Acquire)
                            != TERRANE_RUNTIME_CLOSING
                    },
                )
                .expect("destructor drain lock poisoned");
        }
        TERRANE_RUNTIME_LIVE.with(|live| *live.borrow_mut() = self.previous.take());
    }
}
#[allow(
    dead_code,
    reason = "libraries can use awaited destruction without owning an executor"
)]
fn __terrane_runtime_liveness_enter(
    runtime: &tokio::runtime::Runtime,
    live: std::sync::Arc<TerraneRuntimeLiveness>,
) -> TerraneRuntimeLivenessGuard {
    let previous = TERRANE_RUNTIME_LIVE
        .with(|current| {
            current.borrow_mut().replace((runtime.handle().id(), live.clone()))
        });
    TerraneRuntimeLivenessGuard {
        live,
        previous,
    }
}
#[allow(
    dead_code,
    reason = "shared async support also serves packages without awaited destructors"
)]
struct TerraneDestructorPermit(std::sync::Arc<TerraneRuntimeLiveness>);
impl Drop for TerraneDestructorPermit {
    fn drop(&mut self) {
        if self.0.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel)
            == TERRANE_RUNTIME_CLOSING + 1
        {
            let _drained = self.0.drain.lock().expect("destructor drain lock poisoned");
            self.0.changed.notify_one();
        }
    }
}
fn __terrane_await_destructor<F>(future: F)
where
    F: Future<Output = ()> + Send,
{
    let permit = tokio::runtime::Handle::try_current()
        .ok()
        .and_then(|handle| {
            if handle.runtime_flavor() != tokio::runtime::RuntimeFlavor::MultiThread {
                return None;
            }
            TERRANE_RUNTIME_LIVE
                .with(|current| {
                    let current = current.borrow();
                    let (id, live) = current.as_ref()?;
                    if *id != handle.id() {
                        return None;
                    }
                    live.active
                        .try_update(
                            std::sync::atomic::Ordering::AcqRel,
                            std::sync::atomic::Ordering::Acquire,
                            |active| {
                                (active < TERRANE_RUNTIME_CLOSING - 1).then(|| active + 1)
                            },
                        )
                        .ok()?;
                    Some((handle, TerraneDestructorPermit(live.clone())))
                })
        });
    if let Some((handle, _permit)) = permit {
        tokio::task::block_in_place(|| handle.block_on(future));
        return;
    }
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Terrane destructor runtime must initialize")
                    .block_on(future);
            })
            .join()
            .expect("Terrane awaited destructor must not panic");
    });
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    let live = std::sync::Arc::new(TerraneRuntimeLiveness {
        active: std::sync::atomic::AtomicUsize::new(0),
        drain: std::sync::Mutex::new(()),
        changed: std::sync::Condvar::new(),
    });
    let worker_live = live.clone();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .on_thread_start(move || {
            TERRANE_RUNTIME_LIVE
                .with(|current| {
                    *current.borrow_mut() = Some((
                        tokio::runtime::Handle::current().id(),
                        worker_live.clone(),
                    ));
                });
        })
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize");
    let _liveness = __terrane_runtime_liveness_enter(&runtime, live);
    let future = Box::pin(future);
    runtime
        .block_on(async move {
            let output = __terrane_async_entry(future).await;
            __terrane_wait_projected_cleanups().await;
            output
        })
}
fn __terrane_uninitialized_binding(
    name: &str,
    path: &str,
    line: usize,
    column: usize,
) -> ! {
    eprintln!(
        "{path}:{line}:{column}: error[T0007]: `{name}` may be read before it is assigned"
    );
    std::process::exit(1);
}
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub trait AdvanceProtocol {
    fn clone_box(&self) -> Box<dyn AdvanceProtocol>;
    fn separate_box(&self) -> Box<dyn AdvanceProtocol>;
    fn advance(&mut self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn AdvanceProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Advance(Box<dyn AdvanceProtocol>);
impl Clone for Advance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Advance {
    pub fn advance(&mut self) -> terrane_int_support::Int {
        self.0.advance()
    }
}
impl witness::Advance for Advance {
    fn advance(&mut self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Advance>::advance(&mut *self);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub use witness::Connection;
pub use witness::Database;
pub use witness::OwnerQuery;
pub use witness::Query;
pub use witness::Stage;
pub fn terrane_static_trn_436f6e6e656374696f6e_new() -> Result<
    Connection,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::Connection::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::Connection",
                ),
            )
        }
    }
}
pub fn owner_query(
    __trn_5f73716c: &str,
) -> Result<witness::OwnerQuery<witness::Database>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::owner_query(__trn_5f73716c)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::owner_query",
                ),
            )
        }
    }
}
pub fn query(__trn_5f73716c: &str) -> Result<Query, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::query(__trn_5f73716c)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::query"))
        }
    }
}
