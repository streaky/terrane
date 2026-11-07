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
        "/app::add-tracked",
        "/app::forget-token",
        "/app::main",
    ];
    pub static SITES: [Site; 61] = [
        /* terrane-site-row: site 0: /app::add-tracked (src/main.trn:21:13-21:25) */
        { Site { function: 0, file: 0, line: 21, column: 13, end_line: 21, end_column: 25 } },
        /* terrane-site-row: site 1: /app::add-tracked (src/main.trn:23:55-23:69) */
        { Site { function: 0, file: 0, line: 23, column: 55, end_line: 23, end_column: 69 } },
        /* terrane-site-row: site 2: /app::add-tracked (src/main.trn:24:12-24:40) */
        { Site { function: 0, file: 0, line: 24, column: 12, end_line: 24, end_column: 40 } },
        /* terrane-site-row: site 3: /app::forget-token (src/main.trn:27:5-27:36) */
        { Site { function: 1, file: 0, line: 27, column: 5, end_line: 27, end_column: 36 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:33:17-33:31) */
        { Site { function: 2, file: 0, line: 33, column: 17, end_line: 33, end_column: 31 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:34:13-34:41) */
        { Site { function: 2, file: 0, line: 34, column: 13, end_line: 34, end_column: 41 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:35:20-35:48) */
        { Site { function: 2, file: 0, line: 35, column: 20, end_line: 35, end_column: 48 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:36:13-36:54) */
        { Site { function: 2, file: 0, line: 36, column: 13, end_line: 36, end_column: 54 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:37:23-37:37) */
        { Site { function: 2, file: 0, line: 37, column: 23, end_line: 37, end_column: 37 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:38:17-38:54) */
        { Site { function: 2, file: 0, line: 38, column: 17, end_line: 38, end_column: 54 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:39:13-39:40) */
        { Site { function: 2, file: 0, line: 39, column: 13, end_line: 39, end_column: 40 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:40:22-40:47) */
        { Site { function: 2, file: 0, line: 40, column: 22, end_line: 40, end_column: 47 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:40:5-40:48) */
        { Site { function: 2, file: 0, line: 40, column: 5, end_line: 40, end_column: 48 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:42:13-42:26) */
        { Site { function: 2, file: 0, line: 42, column: 13, end_line: 42, end_column: 26 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:43:13-43:43) */
        { Site { function: 2, file: 0, line: 43, column: 13, end_line: 43, end_column: 43 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:44:13-44:36) */
        { Site { function: 2, file: 0, line: 44, column: 13, end_line: 44, end_column: 36 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:45:13-45:41) */
        { Site { function: 2, file: 0, line: 45, column: 13, end_line: 45, end_column: 41 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:46:22-46:47) */
        { Site { function: 2, file: 0, line: 46, column: 22, end_line: 46, end_column: 47 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:46:5-46:48) */
        { Site { function: 2, file: 0, line: 46, column: 5, end_line: 46, end_column: 48 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:50:13-50:27) */
        { Site { function: 2, file: 0, line: 50, column: 13, end_line: 50, end_column: 27 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:51:67-51:92) */
        { Site { function: 2, file: 0, line: 51, column: 67, end_line: 51, end_column: 92 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:51:13-51:93) */
        { Site { function: 2, file: 0, line: 51, column: 13, end_line: 51, end_column: 93 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:53:13-53:27) */
        { Site { function: 2, file: 0, line: 53, column: 13, end_line: 53, end_column: 27 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:54:13-54:43) */
        { Site { function: 2, file: 0, line: 54, column: 13, end_line: 54, end_column: 43 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:57:13-57:29) */
        { Site { function: 2, file: 0, line: 57, column: 13, end_line: 57, end_column: 29 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:58:22-58:47) */
        { Site { function: 2, file: 0, line: 58, column: 22, end_line: 58, end_column: 47 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:58:5-58:48) */
        { Site { function: 2, file: 0, line: 58, column: 5, end_line: 58, end_column: 48 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:59:18-59:39) */
        { Site { function: 2, file: 0, line: 59, column: 18, end_line: 59, end_column: 39 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:61:9-61:26) */
        { Site { function: 2, file: 0, line: 61, column: 9, end_line: 61, end_column: 26 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:63:23-63:52) */
        { Site { function: 2, file: 0, line: 63, column: 23, end_line: 63, end_column: 52 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:64:22-64:47) */
        { Site { function: 2, file: 0, line: 64, column: 22, end_line: 64, end_column: 47 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:64:5-64:48) */
        { Site { function: 2, file: 0, line: 64, column: 5, end_line: 64, end_column: 48 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:65:13-65:29) */
        { Site { function: 2, file: 0, line: 65, column: 13, end_line: 65, end_column: 29 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:67:13-67:51) */
        { Site { function: 2, file: 0, line: 67, column: 13, end_line: 67, end_column: 51 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:70:13-70:27) */
        { Site { function: 2, file: 0, line: 70, column: 13, end_line: 70, end_column: 27 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:71:5-71:23) */
        { Site { function: 2, file: 0, line: 71, column: 5, end_line: 71, end_column: 23 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:72:13-72:45) */
        { Site { function: 2, file: 0, line: 72, column: 13, end_line: 72, end_column: 45 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:73:13-73:27) */
        { Site { function: 2, file: 0, line: 73, column: 13, end_line: 73, end_column: 27 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:81:22-81:60) */
        { Site { function: 2, file: 0, line: 81, column: 22, end_line: 81, end_column: 60 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:82:5-82:44) */
        { Site { function: 2, file: 0, line: 82, column: 5, end_line: 82, end_column: 44 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:85:5-85:51) */
        { Site { function: 2, file: 0, line: 85, column: 5, end_line: 85, end_column: 51 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:87:23-87:38) */
        { Site { function: 2, file: 0, line: 87, column: 23, end_line: 87, end_column: 38 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:89:28-89:49) */
        { Site { function: 2, file: 0, line: 89, column: 28, end_line: 89, end_column: 49 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:91:5-91:61) */
        { Site { function: 2, file: 0, line: 91, column: 5, end_line: 91, end_column: 61 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:95:32-95:58) */
        { Site { function: 2, file: 0, line: 95, column: 32, end_line: 95, end_column: 58 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:95:5-95:59) */
        { Site { function: 2, file: 0, line: 95, column: 5, end_line: 95, end_column: 59 } },
        /* terrane-site-row: site 46: /app::main (src/main.trn:96:34-96:60) */
        { Site { function: 2, file: 0, line: 96, column: 34, end_line: 96, end_column: 60 } },
        /* terrane-site-row: site 47: /app::main (src/main.trn:96:5-96:61) */
        { Site { function: 2, file: 0, line: 96, column: 5, end_line: 96, end_column: 61 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:97:52-97:77) */
        { Site { function: 2, file: 0, line: 97, column: 52, end_line: 97, end_column: 77 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:97:13-97:78) */
        { Site { function: 2, file: 0, line: 97, column: 13, end_line: 97, end_column: 78 } },
        /* terrane-site-row: site 50: /app::main (src/main.trn:99:35-99:60) */
        { Site { function: 2, file: 0, line: 99, column: 35, end_line: 99, end_column: 60 } },
        /* terrane-site-row: site 51: /app::main (src/main.trn:99:13-99:61) */
        { Site { function: 2, file: 0, line: 99, column: 13, end_line: 99, end_column: 61 } },
        /* terrane-site-row: site 52: /app::main (src/main.trn:102:50-102:76) */
        { Site { function: 2, file: 0, line: 102, column: 50, end_line: 102, end_column: 76 } },
        /* terrane-site-row: site 53: /app::main (src/main.trn:102:5-102:77) */
        { Site { function: 2, file: 0, line: 102, column: 5, end_line: 102, end_column: 77 } },
        /* terrane-site-row: site 54: /app::main (src/main.trn:103:40-103:66) */
        { Site { function: 2, file: 0, line: 103, column: 40, end_line: 103, end_column: 66 } },
        /* terrane-site-row: site 55: /app::main (src/main.trn:103:5-103:67) */
        { Site { function: 2, file: 0, line: 103, column: 5, end_line: 103, end_column: 67 } },
        /* terrane-site-row: site 56: /app::main (src/main.trn:106:5-106:36) */
        { Site { function: 2, file: 0, line: 106, column: 5, end_line: 106, end_column: 36 } },
        /* terrane-site-row: site 57: /app::main (src/main.trn:109:13-109:48) */
        { Site { function: 2, file: 0, line: 109, column: 13, end_line: 109, end_column: 48 } },
        /* terrane-site-row: site 58: /app::main (src/main.trn:110:24-110:50) */
        { Site { function: 2, file: 0, line: 110, column: 24, end_line: 110, end_column: 50 } },
        /* terrane-site-row: site 59: /app::main (src/main.trn:114:17-114:32) */
        { Site { function: 2, file: 0, line: 114, column: 17, end_line: 114, end_column: 32 } },
        /* terrane-site-row: site 60: /app::main (src/main.trn:117:22-117:48) */
        { Site { function: 2, file: 0, line: 117, column: 22, end_line: 117, end_column: 48 } },
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
pub struct TerraneOwnedConsumingInputsR<A0, Output> {
    body: Box<dyn FnOnce((&A0,)) -> Output + Send + 'static>,
}
impl<A0, Output> TerraneOwnedConsumingInputsR<A0, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnOnce((&A0,)) -> Output + Send + 'static,
    {
        Self { body: Box::new(function) }
    }
    fn call(self, arguments: (&A0,)) -> Output {
        (self.body)(arguments)
    }
}
trait TerraneOwnedMutableInputsRBody<A0, Output>: Send {
    fn call(&mut self, arguments: (&A0,)) -> Output;
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneOwnedMutableInputsRBody<A0, Output>>;
}
impl<A0, Output, Function> TerraneOwnedMutableInputsRBody<A0, Output> for Function
where
    Function: FnMut((&A0,)) -> Output + Clone + Send + 'static,
{
    fn call(&mut self, arguments: (&A0,)) -> Output {
        self(arguments)
    }
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneOwnedMutableInputsRBody<A0, Output>> {
        std::boxed::Box::new(self.clone())
    }
}
pub struct TerraneOwnedMutableInputsR<A0, Output> {
    body: std::sync::Mutex<
        std::boxed::Box<dyn TerraneOwnedMutableInputsRBody<A0, Output>>,
    >,
}
impl<A0, Output> TerraneOwnedMutableInputsR<A0, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnMut((&A0,)) -> Output + Clone + Send + 'static,
    {
        Self {
            body: std::sync::Mutex::new(std::boxed::Box::new(function)),
        }
    }
    fn call(&self, arguments: (&A0,)) -> Output {
        self.body.lock().expect("mutable callable lock poisoned").call(arguments)
    }
}
impl<A0, Output> Clone for TerraneOwnedMutableInputsR<A0, Output> {
    fn clone(&self) -> Self {
        let body = self.body.lock().expect("mutable callable lock poisoned").clone_box();
        Self {
            body: std::sync::Mutex::new(body),
        }
    }
}
trait TerraneOwnedMutableInputsMBody<A0, Output>: Send {
    fn call(&mut self, arguments: (&mut A0,)) -> Output;
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneOwnedMutableInputsMBody<A0, Output>>;
}
impl<A0, Output, Function> TerraneOwnedMutableInputsMBody<A0, Output> for Function
where
    Function: FnMut((&mut A0,)) -> Output + Clone + Send + 'static,
{
    fn call(&mut self, arguments: (&mut A0,)) -> Output {
        self(arguments)
    }
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneOwnedMutableInputsMBody<A0, Output>> {
        std::boxed::Box::new(self.clone())
    }
}
pub struct TerraneOwnedMutableInputsM<A0, Output> {
    body: std::sync::Mutex<
        std::boxed::Box<dyn TerraneOwnedMutableInputsMBody<A0, Output>>,
    >,
}
impl<A0, Output> TerraneOwnedMutableInputsM<A0, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnMut((&mut A0,)) -> Output + Clone + Send + 'static,
    {
        Self {
            body: std::sync::Mutex::new(std::boxed::Box::new(function)),
        }
    }
    fn call(&self, arguments: (&mut A0,)) -> Output {
        self.body.lock().expect("mutable callable lock poisoned").call(arguments)
    }
}
impl<A0, Output> Clone for TerraneOwnedMutableInputsM<A0, Output> {
    fn clone(&self) -> Self {
        let body = self.body.lock().expect("mutable callable lock poisoned").clone_box();
        Self {
            body: std::sync::Mutex::new(body),
        }
    }
}
trait TerraneConsumingCallableBody<Arguments, Output>: Send {
    fn call(self: std::boxed::Box<Self>, arguments: Arguments) -> Output;
}
impl<Arguments, Output, Function> TerraneConsumingCallableBody<Arguments, Output>
for Function
where
    Function: FnOnce(Arguments) -> Output + Send + 'static,
{
    fn call(self: std::boxed::Box<Self>, arguments: Arguments) -> Output {
        self(arguments)
    }
}
pub struct TerraneConsumingCallable<Arguments, Output> {
    body: std::boxed::Box<dyn TerraneConsumingCallableBody<Arguments, Output>>,
}
impl<Arguments, Output> TerraneConsumingCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnOnce(Arguments) -> Output + Send + 'static,
    {
        Self {
            body: std::boxed::Box::new(function),
        }
    }
    fn call(self, arguments: Arguments) -> Output {
        self.body.call(arguments)
    }
}
// Source: <terrane>/projected/deps/terrane-retained-callback-api.trn
// Namespace: deps/terrane-retained-callback-api
pub trait SinkProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn SinkProtocol>;
    fn separate_box(&self) -> Box<dyn SinkProtocol>;
    fn accept(&self, event: &Event) -> terrane_int_support::Int;
}
impl Clone for Box<dyn SinkProtocol> {
    fn clone(&self) -> Self {
        SinkProtocol::clone_box(&**self)
    }
}
pub struct Sink(Box<dyn SinkProtocol>);
impl Clone for Sink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Sink {
    pub fn accept(&self, event: &Event) -> terrane_int_support::Int {
        self.0.accept(event)
    }
}
impl terrane_retained_callback_api::Sink for Sink {
    fn accept(&self, event: &terrane_retained_callback_api::Event) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Sink>::accept(&*self, event);
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
pub use terrane_retained_callback_api::Event;
pub use terrane_retained_callback_api::MutableEvent;
pub use terrane_retained_callback_registry::Registry;
pub use terrane_retained_callback_api::SendOnly;
pub use terrane_retained_callback_api::Signal;
pub use std::sync::Weak;
pub use terrane_retained_callback_api::WeakListener;
pub use terrane_retained_callback_registry::WeakRegistry;
pub fn terrane_static_trn_53656e644f6e6c79_new() -> Result<
    SendOnly,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::SendOnly::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::SendOnly",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_5369676e616c_drops() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::Signal::drops()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::Signal",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_5369676e616c_new() -> Result<
    Signal,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::Signal::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::Signal",
                ),
            )
        }
    }
}
pub fn event_clones() -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::event_clones()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::event_clones",
                ),
            )
        }
    }
}
pub fn invoke_borrowed_once(
    callback: TerraneOwnedConsumingInputsR<Event, ()>,
    event: &Event,
) -> Result<(), crate::TerraneForeignError> {
    let callback = std::boxed::Box::new({
        let callback = callback;
        move |callback_argument_0: &terrane_retained_callback_api::Event| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback.call((&*callback_argument_0,));
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    });
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_borrowed_once(
            callback,
            event,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_borrowed_once",
                ),
            )
        }
    }
}
pub fn invoke_mut(
    callback: TerraneOwnedMutableInputsM<MutableEvent, ()>,
    event: &mut MutableEvent,
) -> Result<(), crate::TerraneForeignError> {
    let callback = std::boxed::Box::new({
        let callback = callback;
        move |callback_argument_0: &mut terrane_retained_callback_api::MutableEvent| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback.call((&mut *callback_argument_0,));
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    });
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_mut(
            callback,
            event,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_mut",
                ),
            )
        }
    }
}
pub fn invoke_number(
    callback: TerraneOwnedMutableInputsM<i64, ()>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let callback = std::boxed::Box::new({
        let callback = callback;
        move |callback_argument_0: &mut i64| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback.call((&mut *callback_argument_0,));
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    });
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_number(
            callback,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_number",
                ),
            )
        }
    }
}
pub fn invoke_object_alias(
    callback: std::sync::Arc<dyn Fn(&Event) -> () + Send + Sync>,
    event: &Event,
) -> Result<(), crate::TerraneForeignError> {
    let callback = std::sync::Arc::new({
        let callback = callback;
        move |callback_argument_0: &terrane_retained_callback_api::Event| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback(&*callback_argument_0);
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    });
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_object_alias(
            callback,
            event,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_object_alias",
                ),
            )
        }
    }
}
pub fn invoke_once(
    callback: TerraneConsumingCallable<(Event,), ()>,
    event: Event,
) -> Result<(), crate::TerraneForeignError> {
    let callback = std::boxed::Box::new({
        let callback = callback;
        move |callback_argument_0: terrane_retained_callback_api::Event| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback.call((callback_argument_0,));
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    });
    let event = event;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_once(
            callback,
            event,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_once",
                ),
            )
        }
    }
}
pub fn invoke_send_mut(
    callback: TerraneOwnedMutableInputsM<MutableEvent, ()>,
    event: &mut MutableEvent,
) -> Result<(), crate::TerraneForeignError> {
    let callback = std::boxed::Box::new({
        let callback = callback;
        move |callback_argument_0: &mut terrane_retained_callback_api::MutableEvent| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback.call((&mut *callback_argument_0,));
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    });
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_send_mut(
            callback,
            event,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_send_mut",
                ),
            )
        }
    }
}
pub fn invoke_shared(
    callback: std::sync::Arc<dyn Fn(&Event) -> () + Send + Sync>,
    event: &Event,
) -> Result<(), crate::TerraneForeignError> {
    let callback = std::sync::Arc::new({
        let callback = callback;
        move |callback_argument_0: &terrane_retained_callback_api::Event| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback(&*callback_argument_0);
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    });
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_shared(
            callback,
            event,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_shared",
                ),
            )
        }
    }
}
pub fn invoke_sink<
    TerraneBoxed0: terrane_retained_callback_api::Sink + core::marker::Send
        + core::marker::Sync + 'static,
>(
    sink: TerraneBoxed0,
    event: &Event,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let sink = std::sync::Arc::new(sink);
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_sink(
            sink,
            event,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_sink",
                ),
            )
        }
    }
}
pub fn invoke_text(
    callback: TerraneOwnedMutableInputsR<String, ()>,
) -> Result<(), crate::TerraneForeignError> {
    let callback = std::boxed::Box::new({
        let callback = callback;
        move |callback_argument_0: &str| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback.call((&callback_argument_0.to_owned(),));
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    });
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_api::invoke_text(
            callback,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-api",
                    "terrane_retained_callback_api::invoke_text",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/terrane-retained-callback-registry.trn
// Namespace: deps/terrane-retained-callback-registry
pub fn terrane_static_trn_5265676973747279_clone_token(
    token: &std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >,
) -> Result<
    terrane_retained_callback_api::WeakListener<terrane_retained_callback_api::Event>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_registry::Registry::clone_token(
            token,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-registry",
                    "terrane_retained_callback_registry::Registry",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_5265676973747279_new() -> Result<
    Registry,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_registry::Registry::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-registry",
                    "terrane_retained_callback_registry::Registry",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_5265676973747279_same_token(
    left: &std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >,
    right: &std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >,
) -> Result<bool, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_registry::Registry::same_token(
            left,
            right,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-registry",
                    "terrane_retained_callback_registry::Registry",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_5265676973747279_token_alive(
    token: &std::sync::Weak<
        dyn terrane_retained_callback_api::EventListener<
            terrane_retained_callback_api::Event,
        > + core::marker::Send + core::marker::Sync,
    >,
) -> Result<bool, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_retained_callback_registry::Registry::token_alive(
            token,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-retained-callback-registry",
                    "terrane_retained_callback_registry::Registry",
                ),
            )
        }
    }
}
