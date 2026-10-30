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
    pub static FUNCTIONS: [&str; 2] = ["/app::read-holder", "/app::main"];
    pub static SITES: [Site; 76] = [
        /* terrane-site-row: site 0: /app::read-holder (src/main.trn:19:13-19:44) */
        { Site { function: 0, file: 0, line: 19, column: 13, end_line: 19, end_column: 44 } },
        /* terrane-site-row: site 1: /app::read-holder (src/main.trn:20:13-20:46) */
        { Site { function: 0, file: 0, line: 20, column: 13, end_line: 20, end_column: 46 } },
        /* terrane-site-row: site 2: /app::read-holder (src/main.trn:21:13-21:45) */
        { Site { function: 0, file: 0, line: 21, column: 13, end_line: 21, end_column: 45 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:26:13-26:34) */
        { Site { function: 1, file: 0, line: 26, column: 13, end_line: 26, end_column: 34 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:27:13-27:36) */
        { Site { function: 1, file: 0, line: 27, column: 13, end_line: 27, end_column: 36 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:28:13-28:35) */
        { Site { function: 1, file: 0, line: 28, column: 13, end_line: 28, end_column: 35 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:30:13-30:44) */
        { Site { function: 1, file: 0, line: 30, column: 13, end_line: 30, end_column: 44 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:31:13-31:45) */
        { Site { function: 1, file: 0, line: 31, column: 13, end_line: 31, end_column: 45 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:34:24-34:33) */
        { Site { function: 1, file: 0, line: 34, column: 24, end_line: 34, end_column: 33 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:34:35-34:44) */
        { Site { function: 1, file: 0, line: 34, column: 35, end_line: 34, end_column: 44 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:34:13-34:44) */
        { Site { function: 1, file: 0, line: 34, column: 13, end_line: 34, end_column: 44 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:35:24-35:33) */
        { Site { function: 1, file: 0, line: 35, column: 24, end_line: 35, end_column: 33 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:35:35-35:44) */
        { Site { function: 1, file: 0, line: 35, column: 35, end_line: 35, end_column: 44 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:35:13-35:44) */
        { Site { function: 1, file: 0, line: 35, column: 13, end_line: 35, end_column: 44 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:37:24-37:33) */
        { Site { function: 1, file: 0, line: 37, column: 24, end_line: 37, end_column: 33 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:37:24-37:36) */
        { Site { function: 1, file: 0, line: 37, column: 24, end_line: 37, end_column: 36 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:37:38-37:47) */
        { Site { function: 1, file: 0, line: 37, column: 38, end_line: 37, end_column: 47 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:37:38-37:50) */
        { Site { function: 1, file: 0, line: 37, column: 38, end_line: 37, end_column: 50 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:37:13-37:50) */
        { Site { function: 1, file: 0, line: 37, column: 13, end_line: 37, end_column: 50 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:39:24-39:42) */
        { Site { function: 1, file: 0, line: 39, column: 24, end_line: 39, end_column: 42 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:39:24-39:50) */
        { Site { function: 1, file: 0, line: 39, column: 24, end_line: 39, end_column: 50 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:39:52-39:69) */
        { Site { function: 1, file: 0, line: 39, column: 52, end_line: 39, end_column: 69 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:39:52-39:78) */
        { Site { function: 1, file: 0, line: 39, column: 52, end_line: 39, end_column: 78 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:39:13-39:78) */
        { Site { function: 1, file: 0, line: 39, column: 13, end_line: 39, end_column: 78 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:40:13-40:42) */
        { Site { function: 1, file: 0, line: 40, column: 13, end_line: 40, end_column: 42 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:42:13-42:47) */
        { Site { function: 1, file: 0, line: 42, column: 13, end_line: 42, end_column: 47 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:43:5-43:31) */
        { Site { function: 1, file: 0, line: 43, column: 5, end_line: 43, end_column: 31 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:44:13-44:47) */
        { Site { function: 1, file: 0, line: 44, column: 13, end_line: 44, end_column: 47 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:45:12-45:21) */
        { Site { function: 1, file: 0, line: 45, column: 12, end_line: 45, end_column: 21 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:46:12-46:26) */
        { Site { function: 1, file: 0, line: 46, column: 12, end_line: 46, end_column: 26 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:47:13-47:37) */
        { Site { function: 1, file: 0, line: 47, column: 13, end_line: 47, end_column: 37 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:48:14-48:26) */
        { Site { function: 1, file: 0, line: 48, column: 14, end_line: 48, end_column: 26 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:49:13-49:48) */
        { Site { function: 1, file: 0, line: 49, column: 13, end_line: 49, end_column: 48 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:50:13-50:37) */
        { Site { function: 1, file: 0, line: 50, column: 13, end_line: 50, end_column: 37 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:51:13-51:50) */
        { Site { function: 1, file: 0, line: 51, column: 13, end_line: 51, end_column: 50 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:52:18-52:56) */
        { Site { function: 1, file: 0, line: 52, column: 18, end_line: 52, end_column: 56 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:54:5-54:19) */
        { Site { function: 1, file: 0, line: 54, column: 5, end_line: 54, end_column: 19 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:57:13-57:29) */
        { Site { function: 1, file: 0, line: 57, column: 13, end_line: 57, end_column: 29 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:61:13-61:44) */
        { Site { function: 1, file: 0, line: 61, column: 13, end_line: 61, end_column: 44 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:62:5-62:26) */
        { Site { function: 1, file: 0, line: 62, column: 5, end_line: 62, end_column: 26 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:65:14-65:23) */
        { Site { function: 1, file: 0, line: 65, column: 14, end_line: 65, end_column: 23 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:68:13-68:48) */
        { Site { function: 1, file: 0, line: 68, column: 13, end_line: 68, end_column: 48 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:69:13-69:49) */
        { Site { function: 1, file: 0, line: 69, column: 13, end_line: 69, end_column: 49 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:70:15-70:44) */
        { Site { function: 1, file: 0, line: 70, column: 15, end_line: 70, end_column: 44 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:72:18-72:25) */
        { Site { function: 1, file: 0, line: 72, column: 18, end_line: 72, end_column: 25 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:74:20-74:36) */
        { Site { function: 1, file: 0, line: 74, column: 20, end_line: 74, end_column: 36 } },
        /* terrane-site-row: site 46: /app::main (src/main.trn:83:38-83:55) */
        { Site { function: 1, file: 0, line: 83, column: 38, end_line: 83, end_column: 55 } },
        /* terrane-site-row: site 47: /app::main (src/main.trn:83:13-83:56) */
        { Site { function: 1, file: 0, line: 83, column: 13, end_line: 83, end_column: 56 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:85:36-85:53) */
        { Site { function: 1, file: 0, line: 85, column: 36, end_line: 85, end_column: 53 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:87:13-87:54) */
        { Site { function: 1, file: 0, line: 87, column: 13, end_line: 87, end_column: 54 } },
        /* terrane-site-row: site 50: /app::main (src/main.trn:90:19-90:32) */
        { Site { function: 1, file: 0, line: 90, column: 19, end_line: 90, end_column: 32 } },
        /* terrane-site-row: site 51: /app::main (src/main.trn:95:21-95:45) */
        { Site { function: 1, file: 0, line: 95, column: 21, end_line: 95, end_column: 45 } },
        /* terrane-site-row: site 52: /app::main (src/main.trn:97:18-97:31) */
        { Site { function: 1, file: 0, line: 97, column: 18, end_line: 97, end_column: 31 } },
        /* terrane-site-row: site 53: /app::main (src/main.trn:99:15-99:39) */
        { Site { function: 1, file: 0, line: 99, column: 15, end_line: 99, end_column: 39 } },
        /* terrane-site-row: site 54: /app::main (src/main.trn:100:20-100:53) */
        { Site { function: 1, file: 0, line: 100, column: 20, end_line: 100, end_column: 53 } },
        /* terrane-site-row: site 55: /app::main (src/main.trn:101:5-101:38) */
        { Site { function: 1, file: 0, line: 101, column: 5, end_line: 101, end_column: 38 } },
        /* terrane-site-row: site 56: /app::main (src/main.trn:102:22-102:34) */
        { Site { function: 1, file: 0, line: 102, column: 22, end_line: 102, end_column: 34 } },
        /* terrane-site-row: site 57: /app::main (src/main.trn:105:12-105:21) */
        { Site { function: 1, file: 0, line: 105, column: 12, end_line: 105, end_column: 21 } },
        /* terrane-site-row: site 58: /app::main (src/main.trn:107:18-107:55) */
        { Site { function: 1, file: 0, line: 107, column: 18, end_line: 107, end_column: 55 } },
        /* terrane-site-row: site 59: /app::main (src/main.trn:111:18-111:54) */
        { Site { function: 1, file: 0, line: 111, column: 18, end_line: 111, end_column: 54 } },
        /* terrane-site-row: site 60: /app::main (src/main.trn:113:24-113:46) */
        { Site { function: 1, file: 0, line: 113, column: 24, end_line: 113, end_column: 46 } },
        /* terrane-site-row: site 61: /app::main (src/main.trn:113:13-113:54) */
        { Site { function: 1, file: 0, line: 113, column: 13, end_line: 113, end_column: 54 } },
        /* terrane-site-row: site 62: /app::main (src/main.trn:115:24-115:45) */
        { Site { function: 1, file: 0, line: 115, column: 24, end_line: 115, end_column: 45 } },
        /* terrane-site-row: site 63: /app::main (src/main.trn:115:47-115:61) */
        { Site { function: 1, file: 0, line: 115, column: 47, end_line: 115, end_column: 61 } },
        /* terrane-site-row: site 64: /app::main (src/main.trn:115:13-115:61) */
        { Site { function: 1, file: 0, line: 115, column: 13, end_line: 115, end_column: 61 } },
        /* terrane-site-row: site 65: /app::main (src/main.trn:116:24-116:49) */
        { Site { function: 1, file: 0, line: 116, column: 24, end_line: 116, end_column: 49 } },
        /* terrane-site-row: site 66: /app::main (src/main.trn:116:51-116:73) */
        { Site { function: 1, file: 0, line: 116, column: 51, end_line: 116, end_column: 73 } },
        /* terrane-site-row: site 67: /app::main (src/main.trn:116:13-116:73) */
        { Site { function: 1, file: 0, line: 116, column: 13, end_line: 116, end_column: 73 } },
        /* terrane-site-row: site 68: /app::main (src/main.trn:117:24-117:39) */
        { Site { function: 1, file: 0, line: 117, column: 24, end_line: 117, end_column: 39 } },
        /* terrane-site-row: site 69: /app::main (src/main.trn:117:41-117:56) */
        { Site { function: 1, file: 0, line: 117, column: 41, end_line: 117, end_column: 56 } },
        /* terrane-site-row: site 70: /app::main (src/main.trn:117:58-117:72) */
        { Site { function: 1, file: 0, line: 117, column: 58, end_line: 117, end_column: 72 } },
        /* terrane-site-row: site 71: /app::main (src/main.trn:117:74-117:88) */
        { Site { function: 1, file: 0, line: 117, column: 74, end_line: 117, end_column: 88 } },
        /* terrane-site-row: site 72: /app::main (src/main.trn:117:13-117:88) */
        { Site { function: 1, file: 0, line: 117, column: 13, end_line: 117, end_column: 88 } },
        /* terrane-site-row: site 73: /app::main (src/main.trn:118:24-118:39) */
        { Site { function: 1, file: 0, line: 118, column: 24, end_line: 118, end_column: 39 } },
        /* terrane-site-row: site 74: /app::main (src/main.trn:118:41-118:57) */
        { Site { function: 1, file: 0, line: 118, column: 41, end_line: 118, end_column: 57 } },
        /* terrane-site-row: site 75: /app::main (src/main.trn:118:13-118:57) */
        { Site { function: 1, file: 0, line: 118, column: 13, end_line: 118, end_column: 57 } },
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
static TERRANE_SHARED_NEXT_THREAD_TOKEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(
    1,
);
std::thread_local! {
    static TERRANE_SHARED_THREAD_TOKEN : u64 = TERRANE_SHARED_NEXT_THREAD_TOKEN
    .try_update(std::sync::atomic::Ordering::Relaxed,
    std::sync::atomic::Ordering::Relaxed, | next | next.checked_add(1),)
    .expect("shared storage thread token space exhausted");
}
#[derive(Debug)]
pub struct TerraneSharedCell<T> {
    owner: std::sync::atomic::AtomicU64,
    value: std::sync::Mutex<T>,
}
impl<T> TerraneSharedCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            owner: std::sync::atomic::AtomicU64::new(0),
            value: std::sync::Mutex::new(value),
        }
    }
    /// Acquires exclusive shared-storage access, waiting for other threads.
    ///
    /// # Panics
    /// Panics on same-thread reentry, before attempting to wait on itself.
    ///
    /// # Errors
    /// Returns a poisoned guard if a previous holder unwound while borrowing.
    pub fn lock(&self) -> std::sync::LockResult<TerraneSharedGuard<'_, T>> {
        let token = TERRANE_SHARED_THREAD_TOKEN.with(|token| *token);
        assert!(
            self.owner.load(std::sync::atomic::Ordering::Acquire) != token,
            "shared reference borrow conflict: storage is already borrowed by this thread"
        );
        let (guard, poisoned) = match self.value.lock() {
            Ok(guard) => (guard, false),
            Err(error) => (error.into_inner(), true),
        };
        self.owner.store(token, std::sync::atomic::Ordering::Release);
        let wrapped = TerraneSharedGuard {
            owner: &self.owner,
            guard,
        };
        if poisoned { Err(std::sync::PoisonError::new(wrapped)) } else { Ok(wrapped) }
    }
}
#[derive(Debug)]
pub struct TerraneSharedGuard<'a, T> {
    owner: &'a std::sync::atomic::AtomicU64,
    guard: std::sync::MutexGuard<'a, T>,
}
impl<T> std::ops::Deref for TerraneSharedGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.guard
    }
}
impl<T> std::ops::DerefMut for TerraneSharedGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}
impl<T> Drop for TerraneSharedGuard<'_, T> {
    fn drop(&mut self) {
        self.owner.store(0, std::sync::atomic::Ordering::Release);
    }
}
pub struct TerraneStaticObject<T> {
    current: TerraneSharedCell<std::sync::Arc<TerraneSharedCell<T>>>,
}
impl<T> TerraneStaticObject<T> {
    pub fn new(value: T) -> Self {
        Self {
            current: TerraneSharedCell::new(
                std::sync::Arc::new(TerraneSharedCell::new(value)),
            ),
        }
    }
    /// Captures the current object without retaining the replaceable slot guard.
    ///
    /// # Panics
    /// Panics if the slot is poisoned or is recursively borrowed by this thread.
    pub fn handle(&self) -> std::sync::Arc<TerraneSharedCell<T>> {
        self.current.lock().expect("static object slot lock poisoned").clone()
    }
    /// Installs a new object, releasing the slot before dropping its predecessor.
    ///
    /// # Panics
    /// Panics if the slot is poisoned, recursively borrowed, or its old object panics on drop.
    pub fn replace(&self, value: T) {
        let replacement = std::sync::Arc::new(TerraneSharedCell::new(value));
        let previous = {
            let mut current = self
                .current
                .lock()
                .expect("static object slot lock poisoned");
            std::mem::replace(&mut *current, replacement)
        };
        drop(previous);
    }
}
pub trait TerraneSharedHandle<T> {
    /// Acquires a bounded exclusive borrow of the receiver's live storage.
    ///
    /// # Errors
    /// Returns the original cell's poison state with a guard to the selected value.
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>>;
}
impl<T> TerraneSharedHandle<T> for std::sync::Arc<TerraneSharedCell<T>> {
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>> {
        (**self).lock()
    }
}
pub struct TerraneSharedProjection<H, U, T> {
    handle: H,
    read: fn(&U) -> &T,
    write: fn(&mut U) -> &mut T,
}
impl<H, U, T> TerraneSharedProjection<H, U, T> {
    pub fn new(handle: H, read: fn(&U) -> &T, write: fn(&mut U) -> &mut T) -> Self {
        Self { handle, read, write }
    }
}
impl<H: Clone, U, T> Clone for TerraneSharedProjection<H, U, T> {
    fn clone(&self) -> Self {
        Self::new(self.handle.clone(), self.read, self.write)
    }
}
struct TerraneSharedProjectedGuard<G, U, T> {
    guard: G,
    read: fn(&U) -> &T,
    write: fn(&mut U) -> &mut T,
}
impl<G: std::ops::Deref<Target = U>, U, T> std::ops::Deref
for TerraneSharedProjectedGuard<G, U, T> {
    type Target = T;
    fn deref(&self) -> &T {
        (self.read)(&self.guard)
    }
}
impl<G: std::ops::DerefMut<Target = U>, U, T> std::ops::DerefMut
for TerraneSharedProjectedGuard<G, U, T> {
    fn deref_mut(&mut self) -> &mut T {
        (self.write)(&mut self.guard)
    }
}
impl<H: TerraneSharedHandle<U>, U, T> TerraneSharedHandle<T>
for TerraneSharedProjection<H, U, T> {
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>> {
        let (guard, poisoned) = match self.handle.lock() {
            Ok(guard) => (guard, false),
            Err(error) => (error.into_inner(), true),
        };
        let guard = TerraneSharedProjectedGuard {
            guard,
            read: self.read,
            write: self.write,
        };
        if poisoned { Err(std::sync::PoisonError::new(guard)) } else { Ok(guard) }
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
async fn __terrane_dependency_await_unwind<F: Future>(
    future: F,
) -> Result<F::Output, Box<dyn std::any::Any + Send>> {
    let mut future = std::pin::pin!(future);
    std::future::poll_fn(|context| {
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| { future.as_mut().poll(context) }),
            ) {
                Ok(std::task::Poll::Ready(value)) => std::task::Poll::Ready(Ok(value)),
                Ok(std::task::Poll::Pending) => std::task::Poll::Pending,
                Err(payload) => std::task::Poll::Ready(Err(payload)),
            }
        })
        .await
}
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub trait InspectProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn InspectProtocol>;
    fn separate_box(&self) -> Box<dyn InspectProtocol>;
    fn marker(&self) -> bool;
    fn inspect_text(&self, left: &str, right: &str) -> bool;
    fn inspect_strings(
        &self,
        left: &std::vec::Vec<String>,
        right: &std::vec::Vec<String>,
    ) -> bool;
}
impl Clone for Box<dyn InspectProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Inspect(Box<dyn InspectProtocol>);
impl Clone for Inspect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Inspect {
    pub fn marker(&self) -> bool {
        self.0.marker()
    }
    pub fn inspect_text(&self, left: &str, right: &str) -> bool {
        self.0.inspect_text(left, right)
    }
    pub fn inspect_strings(
        &self,
        left: &std::vec::Vec<String>,
        right: &std::vec::Vec<String>,
    ) -> bool {
        self.0.inspect_strings(left, right)
    }
}
impl witness::Inspect for Inspect {
    fn marker(&self) -> bool {
        let __terrane_boundary: Result<bool, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Inspect>::marker(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub use witness::ReadObservation;
pub use witness::Record;
pub fn terrane_static_trn_5265636f7264_inspect(
    text: &str,
    other: &str,
) -> Result<bool, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::Record::inspect(text, other)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::Record"))
        }
    }
}
pub fn terrane_static_trn_5265636f7264_new() -> Result<
    Record,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::Record::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::Record"))
        }
    }
}
pub async fn echo_after_yield(text: &str) -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(witness::echo_after_yield(text)).await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::echo_after_yield",
                ),
            )
        }
    }
}
pub fn join_four(
    first: &str,
    second: &str,
    third: &str,
    fourth: &str,
) -> Result<String, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::join_four(first, second, third, fourth)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::join_four",
                ),
            )
        }
    }
}
pub fn join_text(left: &str, right: &str) -> Result<String, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::join_text(left, right)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::join_text",
                ),
            )
        }
    }
}
pub fn owned_text(text: String) -> Result<String, crate::TerraneForeignError> {
    let text = text;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::owned_text(text)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::owned_text",
                ),
            )
        }
    }
}
pub async fn read_sequence_after_yield(
    items: &std::vec::Vec<String>,
) -> Result<ReadObservation, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            witness::read_sequence_after_yield(items),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::read_sequence_after_yield",
                ),
            )
        }
    }
}
pub async fn same_after_yield(
    left: &str,
    right: &str,
) -> Result<bool, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            witness::same_after_yield(left, right),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::same_after_yield",
                ),
            )
        }
    }
}
pub fn same_bytes(
    left: &Vec<u8>,
    right: &Vec<u8>,
) -> Result<bool, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::same_bytes(left, right)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::same_bytes",
                ),
            )
        }
    }
}
pub fn same_sequence(
    left: &std::vec::Vec<String>,
    right: &std::vec::Vec<String>,
) -> Result<bool, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::same_sequence(left, right)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::same_sequence",
                ),
            )
        }
    }
}
pub fn same_string(
    left: &String,
    right: &String,
) -> Result<bool, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::same_string(left, right)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::same_string",
                ),
            )
        }
    }
}
pub fn same_text(left: &str, right: &str) -> Result<bool, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::same_text(left, right)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::same_text",
                ),
            )
        }
    }
}
pub fn sequence_address(
    items: &std::vec::Vec<String>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::sequence_address(items)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::sequence_address",
                ),
            )
        }
    }
}
pub fn take_text(record: &mut Record) -> Result<String, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::take_text(record)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::take_text",
                ),
            )
        }
    }
}
pub fn update_record(record: &mut Record) -> Result<(), crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::update_record(record)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::update_record",
                ),
            )
        }
    }
}
