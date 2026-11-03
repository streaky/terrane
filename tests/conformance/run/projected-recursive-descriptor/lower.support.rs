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
    pub static FUNCTIONS: [&str; 3] = [
        "/app::configuration",
        "/app::configuration-for",
        "/app::main",
    ];
    pub static SITES: [Site; 46] = [
        /* terrane-site-row: site 0: /app::configuration (src/main.trn:17:80-17:91) */
        { Site { function: 0, file: 0, line: 17, column: 80, end_line: 17, end_column: 91 } },
        /* terrane-site-row: site 1: /app::configuration (src/main.trn:17:105-17:119) */
        { Site { function: 0, file: 0, line: 17, column: 105, end_line: 17, end_column: 119 } },
        /* terrane-site-row: site 2: /app::configuration (src/main.trn:17:12-17:279) */
        { Site { function: 0, file: 0, line: 17, column: 12, end_line: 17, end_column: 279 } },
        /* terrane-site-row: site 3: /app::configuration-for (src/main.trn:20:78-20:89) */
        { Site { function: 1, file: 0, line: 20, column: 78, end_line: 20, end_column: 89 } },
        /* terrane-site-row: site 4: /app::configuration-for (src/main.trn:20:103-20:117) */
        { Site { function: 1, file: 0, line: 20, column: 103, end_line: 20, end_column: 117 } },
        /* terrane-site-row: site 5: /app::configuration-for (src/main.trn:20:12-20:220) */
        { Site { function: 1, file: 0, line: 20, column: 12, end_line: 20, end_column: 220 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:23:25-23:39) */
        { Site { function: 2, file: 0, line: 23, column: 25, end_line: 23, end_column: 39 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:24:13-24:28) */
        { Site { function: 2, file: 0, line: 24, column: 13, end_line: 24, end_column: 28 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:25:13-25:35) */
        { Site { function: 2, file: 0, line: 25, column: 13, end_line: 25, end_column: 35 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:26:13-26:35) */
        { Site { function: 2, file: 0, line: 26, column: 13, end_line: 26, end_column: 35 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:27:13-27:44) */
        { Site { function: 2, file: 0, line: 27, column: 13, end_line: 27, end_column: 44 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:29:9-29:27) */
        { Site { function: 2, file: 0, line: 29, column: 9, end_line: 29, end_column: 27 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:32:13-32:33) */
        { Site { function: 2, file: 0, line: 32, column: 13, end_line: 32, end_column: 33 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:33:26-33:40) */
        { Site { function: 2, file: 0, line: 33, column: 26, end_line: 33, end_column: 40 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:34:13-34:36) */
        { Site { function: 2, file: 0, line: 34, column: 13, end_line: 34, end_column: 36 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:35:26-35:40) */
        { Site { function: 2, file: 0, line: 35, column: 26, end_line: 35, end_column: 40 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:36:13-36:50) */
        { Site { function: 2, file: 0, line: 36, column: 13, end_line: 36, end_column: 50 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:37:51-37:58) */
        { Site { function: 2, file: 0, line: 37, column: 51, end_line: 37, end_column: 58 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:37:14-37:76) */
        { Site { function: 2, file: 0, line: 37, column: 14, end_line: 37, end_column: 76 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:38:14-38:38) */
        { Site { function: 2, file: 0, line: 38, column: 14, end_line: 38, end_column: 38 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:40:13-40:26) */
        { Site { function: 2, file: 0, line: 40, column: 13, end_line: 40, end_column: 26 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:41:52-41:59) */
        { Site { function: 2, file: 0, line: 41, column: 52, end_line: 41, end_column: 59 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:41:15-41:78) */
        { Site { function: 2, file: 0, line: 41, column: 15, end_line: 41, end_column: 78 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:43:9-43:34) */
        { Site { function: 2, file: 0, line: 43, column: 9, end_line: 43, end_column: 34 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:46:13-46:26) */
        { Site { function: 2, file: 0, line: 46, column: 13, end_line: 46, end_column: 26 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:53:13-53:44) */
        { Site { function: 2, file: 0, line: 53, column: 13, end_line: 53, end_column: 44 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:54:14-54:45) */
        { Site { function: 2, file: 0, line: 54, column: 14, end_line: 54, end_column: 45 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:55:13-55:116) */
        { Site { function: 2, file: 0, line: 55, column: 13, end_line: 55, end_column: 116 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:56:13-56:33) */
        { Site { function: 2, file: 0, line: 56, column: 13, end_line: 56, end_column: 33 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:58:5-58:18) */
        { Site { function: 2, file: 0, line: 58, column: 5, end_line: 58, end_column: 18 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:59:13-59:35) */
        { Site { function: 2, file: 0, line: 59, column: 13, end_line: 59, end_column: 35 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:61:25-61:50) */
        { Site { function: 2, file: 0, line: 61, column: 25, end_line: 61, end_column: 50 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:62:27-62:52) */
        { Site { function: 2, file: 0, line: 62, column: 27, end_line: 62, end_column: 52 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:63:13-63:28) */
        { Site { function: 2, file: 0, line: 63, column: 13, end_line: 63, end_column: 28 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:64:21-64:40) */
        { Site { function: 2, file: 0, line: 64, column: 21, end_line: 64, end_column: 40 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:65:13-65:33) */
        { Site { function: 2, file: 0, line: 65, column: 13, end_line: 65, end_column: 33 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:67:13-67:35) */
        { Site { function: 2, file: 0, line: 67, column: 13, end_line: 67, end_column: 35 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:69:13-69:30) */
        { Site { function: 2, file: 0, line: 69, column: 13, end_line: 69, end_column: 30 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:70:13-70:30) */
        { Site { function: 2, file: 0, line: 70, column: 13, end_line: 70, end_column: 30 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:71:13-71:32) */
        { Site { function: 2, file: 0, line: 71, column: 13, end_line: 71, end_column: 32 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:72:13-72:32) */
        { Site { function: 2, file: 0, line: 72, column: 13, end_line: 72, end_column: 32 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:75:9-75:37) */
        { Site { function: 2, file: 0, line: 75, column: 9, end_line: 75, end_column: 37 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:80:9-80:36) */
        { Site { function: 2, file: 0, line: 80, column: 9, end_line: 80, end_column: 36 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:83:23-83:30) */
        { Site { function: 2, file: 0, line: 83, column: 23, end_line: 83, end_column: 30 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:83:34-83:41) */
        { Site { function: 2, file: 0, line: 83, column: 34, end_line: 83, end_column: 41 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:84:13-84:36) */
        { Site { function: 2, file: 0, line: 84, column: 13, end_line: 84, end_column: 36 } },
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
// Source: <terrane>/projected/deps/terrane-recursive-descriptor-witness.trn
// Namespace: deps/terrane-recursive-descriptor-witness
pub struct Descriptor {
    path: std::path::PathBuf,
    timeout: core::time::Duration,
    args: Vec<String>,
    names: Vec<String>,
    strings: Vec<String>,
    window_size: Option<(u32, u32)>,
    nested: Option<(Option<u32>, Option<u32>)>,
    labels: Option<std::collections::HashMap<String, String>>,
    note: Option<String>,
}
impl Descriptor {
    pub fn terrane_construct(
        path: std::path::PathBuf,
        timeout: core::time::Duration,
        args: terrane_collection_support::List<String>,
        names: terrane_collection_support::List<String>,
        strings: terrane_collection_support::List<String>,
        window_size: impl Into<Option<terrane_collection_support::Tuple<u32>>>,
        nested: impl Into<Option<terrane_collection_support::Tuple<Option<u32>>>>,
        labels: impl Into<
            Option<terrane_collection_support::UnorderedMap<String, String>>,
        >,
        note: impl Into<Option<String>>,
    ) -> Result<Self, crate::TerraneForeignError> {
        Ok(Self {
            path: path,
            timeout: timeout,
            args: args
                .into_iter()
                .map(|item| -> Result<_, crate::TerraneForeignError> { Ok(item) })
                .collect::<Result<Vec<String>, _>>()?,
            names: names
                .into_iter()
                .map(|item| -> Result<_, crate::TerraneForeignError> { Ok(item) })
                .collect::<Result<Vec<String>, _>>()?,
            strings: strings
                .into_iter()
                .map(|item| -> Result<_, crate::TerraneForeignError> { Ok(item) })
                .collect::<Result<Vec<String>, _>>()?,
            window_size: window_size
                .into()
                .map(|value| -> Result<(u32, u32), crate::TerraneForeignError> {
                    Ok({
                        let tuple_value = value;
                        if tuple_value.length() != 2 {
                            return Err(
                                crate::TerraneForeignError(
                                    crate::TerraneError::custom_raised(
                                        crate::TERRANE_DEPENDENCY_ERROR,
                                        "projected tuple length did not match its checked type",
                                        crate::TERRANE_NO_SITE,
                                    ),
                                ),
                            );
                        }
                        let mut tuple_items = tuple_value.into_vec().into_iter();
                        (
                            tuple_items
                                .next()
                                .expect("validated projected tuple length"),
                            tuple_items.next().expect("validated projected tuple length"),
                        )
                    })
                })
                .transpose()?,
            nested: nested
                .into()
                .map(|
                    value,
                | -> Result<(Option<u32>, Option<u32>), crate::TerraneForeignError> {
                    Ok({
                        let tuple_value = value;
                        if tuple_value.length() != 2 {
                            return Err(
                                crate::TerraneForeignError(
                                    crate::TerraneError::custom_raised(
                                        crate::TERRANE_DEPENDENCY_ERROR,
                                        "projected tuple length did not match its checked type",
                                        crate::TERRANE_NO_SITE,
                                    ),
                                ),
                            );
                        }
                        let mut tuple_items = tuple_value.into_vec().into_iter();
                        (
                            tuple_items
                                .next()
                                .expect("validated projected tuple length"),
                            tuple_items.next().expect("validated projected tuple length"),
                        )
                    })
                })
                .transpose()?,
            labels: labels
                .into()
                .map(|
                    value,
                | -> Result<
                    std::collections::HashMap<String, String>,
                    crate::TerraneForeignError,
                > {
                    Ok(
                        value
                            .into_iter()
                            .map(|(key, value)| -> Result<_, crate::TerraneForeignError> {
                                Ok((key, value))
                            })
                            .collect::<
                                Result<std::collections::HashMap<String, String>, _>,
                            >()?,
                    )
                })
                .transpose()?,
            note: note.into(),
        })
    }
}
pub use core::time::Duration;
pub use terrane_recursive_descriptor_witness::Factory;
pub struct MixedDescriptor {
    rows: Vec<(String, String)>,
    items: std::collections::HashMap<
        String,
        terrane_recursive_descriptor_witness::Observed,
    >,
    pair: Option<(String, String)>,
}
impl MixedDescriptor {
    pub fn terrane_construct(
        rows: terrane_collection_support::List<
            terrane_collection_support::Tuple<String>,
        >,
        items: terrane_collection_support::UnorderedMap<
            String,
            terrane_recursive_descriptor_witness::Observed,
        >,
        pair: impl Into<Option<terrane_collection_support::Tuple<String>>>,
    ) -> Result<Self, crate::TerraneForeignError> {
        Ok(Self {
            rows: rows
                .into_iter()
                .map(|item| -> Result<_, crate::TerraneForeignError> {
                    Ok({
                        let tuple_value = item;
                        if tuple_value.length() != 2 {
                            return Err(
                                crate::TerraneForeignError(
                                    crate::TerraneError::custom_raised(
                                        crate::TERRANE_DEPENDENCY_ERROR,
                                        "projected tuple length did not match its checked type",
                                        crate::TERRANE_NO_SITE,
                                    ),
                                ),
                            );
                        }
                        let mut tuple_items = tuple_value.into_vec().into_iter();
                        (
                            tuple_items
                                .next()
                                .expect("validated projected tuple length"),
                            tuple_items.next().expect("validated projected tuple length"),
                        )
                    })
                })
                .collect::<Result<Vec<(String, String)>, _>>()?,
            items: items
                .into_iter()
                .map(|(key, value)| -> Result<_, crate::TerraneForeignError> {
                    Ok((key, value))
                })
                .collect::<
                    Result<
                        std::collections::HashMap<
                            String,
                            terrane_recursive_descriptor_witness::Observed,
                        >,
                        _,
                    >,
                >()?,
            pair: pair
                .into()
                .map(|value| -> Result<(String, String), crate::TerraneForeignError> {
                    Ok({
                        let tuple_value = value;
                        if tuple_value.length() != 2 {
                            return Err(
                                crate::TerraneForeignError(
                                    crate::TerraneError::custom_raised(
                                        crate::TERRANE_DEPENDENCY_ERROR,
                                        "projected tuple length did not match its checked type",
                                        crate::TERRANE_NO_SITE,
                                    ),
                                ),
                            );
                        }
                        let mut tuple_items = tuple_value.into_vec().into_iter();
                        (
                            tuple_items
                                .next()
                                .expect("validated projected tuple length"),
                            tuple_items.next().expect("validated projected tuple length"),
                        )
                    })
                })
                .transpose()?,
        })
    }
}
pub use terrane_recursive_descriptor_witness::Observed;
pub use std::path::PathBuf;
pub use terrane_recursive_descriptor_witness::Ticket;
pub struct UniqueDescriptor {
    ticket: terrane_recursive_descriptor_witness::Ticket,
    text: String,
}
impl UniqueDescriptor {
    pub fn terrane_construct(
        ticket: terrane_recursive_descriptor_witness::Ticket,
        text: String,
    ) -> Result<Self, crate::TerraneForeignError> {
        Ok(Self { ticket: ticket, text: text })
    }
}
pub fn terrane_static_trn_466163746f7279_consume_static(
    value: Descriptor,
) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_value = terrane_recursive_descriptor_witness::Descriptor {
        path: value.path,
        timeout: value.timeout,
        args: value.args.iter().map(|item| std::ffi::OsStr::new(&*item)).collect(),
        names: value.names.iter().map(|item| (*item).as_str()).collect(),
        strings: value.strings.as_slice(),
        window_size: value.window_size,
        nested: value.nested,
        labels: value.labels,
        note: value.note.as_ref().map(|value| (*value).as_str()),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::Factory::consume_static(
            __terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::Factory",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_466163746f7279_inspect_static(
    value: &Descriptor,
) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_value = terrane_recursive_descriptor_witness::Descriptor {
        path: value.path.clone(),
        timeout: value.timeout,
        args: value.args.iter().map(|item| std::ffi::OsStr::new(&*item)).collect(),
        names: value.names.iter().map(|item| (*item).as_str()).collect(),
        strings: value.strings.as_slice(),
        window_size: value.window_size,
        nested: value.nested,
        labels: value.labels.clone(),
        note: value.note.as_ref().map(|value| (*value).as_str()),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::Factory::inspect_static(
            &__terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::Factory",
                ),
            )
        }
    }
}
pub fn borrow_name(value: &Descriptor) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_value = terrane_recursive_descriptor_witness::Descriptor {
        path: value.path.clone(),
        timeout: value.timeout,
        args: value.args.iter().map(|item| std::ffi::OsStr::new(&*item)).collect(),
        names: value.names.iter().map(|item| (*item).as_str()).collect(),
        strings: value.strings.as_slice(),
        window_size: value.window_size,
        nested: value.nested,
        labels: value.labels.clone(),
        note: value.note.as_ref().map(|value| (*value).as_str()),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::borrow_name(
            &__terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value.to_owned()),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::borrow_name",
                ),
            )
        }
    }
}
pub fn consume(value: Descriptor) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_value = terrane_recursive_descriptor_witness::Descriptor {
        path: value.path,
        timeout: value.timeout,
        args: value.args.iter().map(|item| std::ffi::OsStr::new(&*item)).collect(),
        names: value.names.iter().map(|item| (*item).as_str()).collect(),
        strings: value.strings.as_slice(),
        window_size: value.window_size,
        nested: value.nested,
        labels: value.labels,
        note: value.note.as_ref().map(|value| (*value).as_str()),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::consume(
            __terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::consume",
                ),
            )
        }
    }
}
pub fn fail_early(__trn_5f: &Descriptor) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_trn_5f = terrane_recursive_descriptor_witness::Descriptor {
        path: __trn_5f.path.clone(),
        timeout: __trn_5f.timeout,
        args: __trn_5f.args.iter().map(|item| std::ffi::OsStr::new(&*item)).collect(),
        names: __trn_5f.names.iter().map(|item| (*item).as_str()).collect(),
        strings: __trn_5f.strings.as_slice(),
        window_size: __trn_5f.window_size,
        nested: __trn_5f.nested,
        labels: __trn_5f.labels.clone(),
        note: __trn_5f.note.as_ref().map(|value| (*value).as_str()),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::fail_early(
            &__terrane_view_trn_5f,
        )),
    ) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}",
                            "terrane-recursive-descriptor-witness",
                            "terrane_recursive_descriptor_witness::fail_early"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::fail_early",
                ),
            )
        }
    }
}
pub fn fail_unique(
    __trn_5f76616c7565: UniqueDescriptor,
) -> Result<(), crate::TerraneForeignError> {
    let __terrane_view_trn_5f76616c7565 = terrane_recursive_descriptor_witness::UniqueDescriptor {
        ticket: __trn_5f76616c7565.ticket,
        text: __trn_5f76616c7565.text.as_str(),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::fail_unique(
            __terrane_view_trn_5f76616c7565,
        )),
    ) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}",
                            "terrane-recursive-descriptor-witness",
                            "terrane_recursive_descriptor_witness::fail_unique"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::fail_unique",
                ),
            )
        }
    }
}
pub fn inspect(value: &Descriptor) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_value = terrane_recursive_descriptor_witness::Descriptor {
        path: value.path.clone(),
        timeout: value.timeout,
        args: value.args.iter().map(|item| std::ffi::OsStr::new(&*item)).collect(),
        names: value.names.iter().map(|item| (*item).as_str()).collect(),
        strings: value.strings.as_slice(),
        window_size: value.window_size,
        nested: value.nested,
        labels: value.labels.clone(),
        note: value.note.as_ref().map(|value| (*value).as_str()),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::inspect(
            &__terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::inspect",
                ),
            )
        }
    }
}
pub fn inspect_mixed(
    value: &MixedDescriptor,
) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_value = terrane_recursive_descriptor_witness::MixedDescriptor {
        rows: value
            .rows
            .iter()
            .map(|item| ((*item).0.as_str(), (*item).1.clone()))
            .collect(),
        items: value
            .items
            .iter()
            .map(|(key, value)| ((*key).as_str(), (*value).clone()))
            .collect(),
        pair: value.pair.as_ref().map(|value| ((*value).0.as_str(), (*value).1.clone())),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::inspect_mixed(
            &__terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::inspect_mixed",
                ),
            )
        }
    }
}
pub fn inspect_shared(value: &Descriptor) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_value = terrane_recursive_descriptor_witness::Descriptor {
        path: value.path.clone(),
        timeout: value.timeout,
        args: value.args.iter().map(|item| std::ffi::OsStr::new(&*item)).collect(),
        names: value.names.iter().map(|item| (*item).as_str()).collect(),
        strings: value.strings.as_slice(),
        window_size: value.window_size,
        nested: value.nested,
        labels: value.labels.clone(),
        note: value.note.as_ref().map(|value| (*value).as_str()),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::inspect_shared(
            &__terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::inspect_shared",
                ),
            )
        }
    }
}
pub fn pair_sum(
    value: terrane_collection_support::Tuple<terrane_int_support::Int>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = {
        let tuple_value = value;
        if tuple_value.length() != 2 {
            return Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        "projected tuple length did not match its checked type",
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            );
        }
        let mut tuple_items = tuple_value.into_vec().into_iter();
        (
            terrane_int_support::coerce::<
                i64,
            >(&tuple_items.next().expect("validated projected tuple length"))
                .map_err(|error| crate::TerraneForeignError(
                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                ))?,
            terrane_int_support::coerce::<
                i64,
            >(&tuple_items.next().expect("validated projected tuple length"))
                .map_err(|error| crate::TerraneForeignError(
                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                ))?,
        )
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::pair_sum(
            value,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::pair_sum",
                ),
            )
        }
    }
}
pub fn path_value() -> Result<PathBuf, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::path_value()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::path_value",
                ),
            )
        }
    }
}
pub fn reset_clones() -> Result<(), crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::reset_clones()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::reset_clones",
                ),
            )
        }
    }
}
pub fn take_mixed(value: MixedDescriptor) -> Result<String, crate::TerraneForeignError> {
    let mut __terrane_stage_value_0 = value
        .rows
        .into_iter()
        .map(|owned| {
            let (owned_0, owned_1) = owned;
            (owned_0, Some(owned_1))
        })
        .collect::<Vec<_>>();
    let mut __terrane_stage_value_1 = value
        .items
        .into_iter()
        .map(|(key, value)| (key, Some(value)))
        .collect::<Vec<_>>();
    let mut __terrane_stage_value_2 = value
        .pair
        .map(|owned| {
            let (owned_0, owned_1) = owned;
            (owned_0, Some(owned_1))
        });
    let __terrane_view_value = terrane_recursive_descriptor_witness::MixedDescriptor {
        rows: __terrane_stage_value_0
            .iter_mut()
            .map(|stage| (
                (*stage).0.as_str(),
                (*stage).1.take().expect("descriptor owned leaf is transferred once"),
            ))
            .collect(),
        items: __terrane_stage_value_1
            .iter_mut()
            .map(|(key, value)| (
                (*key).as_str(),
                (*value).take().expect("descriptor owned leaf is transferred once"),
            ))
            .collect(),
        pair: __terrane_stage_value_2
            .as_mut()
            .map(|stage| (
                (*stage).0.as_str(),
                (*stage).1.take().expect("descriptor owned leaf is transferred once"),
            )),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::take_mixed(
            __terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::take_mixed",
                ),
            )
        }
    }
}
pub fn take_pair(
    value: terrane_collection_support::Tuple<Ticket>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = {
        let tuple_value = value;
        if tuple_value.length() != 2 {
            return Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        "projected tuple length did not match its checked type",
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            );
        }
        let mut tuple_items = tuple_value.into_unique_vec().into_iter();
        (
            tuple_items.next().expect("validated projected tuple length"),
            tuple_items.next().expect("validated projected tuple length"),
        )
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::take_pair(
            value,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::take_pair",
                ),
            )
        }
    }
}
pub fn take_unique(
    value: UniqueDescriptor,
) -> Result<String, crate::TerraneForeignError> {
    let __terrane_view_value = terrane_recursive_descriptor_witness::UniqueDescriptor {
        ticket: value.ticket,
        text: value.text.as_str(),
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::take_unique(
            __terrane_view_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::take_unique",
                ),
            )
        }
    }
}
pub fn ticket() -> Result<Ticket, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::ticket()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::ticket",
                ),
            )
        }
    }
}
pub fn ticket_drops() -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::ticket_drops()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::ticket_drops",
                ),
            )
        }
    }
}
pub fn timeout_value() -> Result<Duration, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::timeout_value()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::timeout_value",
                ),
            )
        }
    }
}
pub fn window_sum(
    value: terrane_collection_support::Tuple<terrane_int_support::Int>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = {
        let tuple_value = value;
        if tuple_value.length() != 2 {
            return Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        "projected tuple length did not match its checked type",
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            );
        }
        let mut tuple_items = tuple_value.into_vec().into_iter();
        (
            terrane_int_support::coerce::<
                u32,
            >(&tuple_items.next().expect("validated projected tuple length"))
                .map_err(|error| crate::TerraneForeignError(
                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                ))?,
            terrane_int_support::coerce::<
                u32,
            >(&tuple_items.next().expect("validated projected tuple length"))
                .map_err(|error| crate::TerraneForeignError(
                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                ))?,
        )
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_recursive_descriptor_witness::window_sum(
            value,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-recursive-descriptor-witness",
                    "terrane_recursive_descriptor_witness::window_sum",
                ),
            )
        }
    }
}
