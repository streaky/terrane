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
    pub static DESCRIPTORS: [&str; 6] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
        "/deps/facade::Failure",
        "/deps/facade::NewFailure",
        "/deps/facade::TransitiveFailure",
        "/deps/facade::OldHiddenFailure",
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
    pub static FUNCTIONS: [&str; 7] = [
        "/app::call-opaque",
        "/app::call-opaque-async",
        "/app::call-public",
        "/app::checked-inferred",
        "/app::checked-inferred-method",
        "/app::checked-inferred-async",
        "/app::main",
    ];
    pub static SITES: [Site; 65] = [
        /* terrane-site-row: site 0: /app::call-opaque (src/main.trn:16:12-16:36) */
        { Site { function: 0, file: 0, line: 16, column: 12, end_line: 16, end_column: 36 } },
        /* terrane-site-row: site 1: /app::call-opaque-async (src/main.trn:19:14-19:28) */
        { Site { function: 1, file: 0, line: 19, column: 14, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 2: /app::call-opaque-async (src/main.trn:20:18-20:44) */
        { Site { function: 1, file: 0, line: 20, column: 18, end_line: 20, end_column: 44 } },
        /* terrane-site-row: site 3: /app::call-public (src/main.trn:23:12-23:36) */
        { Site { function: 2, file: 0, line: 23, column: 12, end_line: 23, end_column: 36 } },
        /* terrane-site-row: site 4: /app::checked-inferred (src/main.trn:26:12-26:32) */
        { Site { function: 3, file: 0, line: 26, column: 12, end_line: 26, end_column: 32 } },
        /* terrane-site-row: site 5: /app::checked-inferred-method (src/main.trn:29:14-29:30) */
        { Site { function: 4, file: 0, line: 29, column: 14, end_line: 29, end_column: 30 } },
        /* terrane-site-row: site 6: /app::checked-inferred-method (src/main.trn:30:12-30:30) */
        { Site { function: 4, file: 0, line: 30, column: 12, end_line: 30, end_column: 30 } },
        /* terrane-site-row: site 7: /app::checked-inferred-async (src/main.trn:33:14-33:30) */
        { Site { function: 5, file: 0, line: 33, column: 14, end_line: 33, end_column: 30 } },
        /* terrane-site-row: site 8: /app::checked-inferred-async (src/main.trn:34:18-34:44) */
        { Site { function: 5, file: 0, line: 34, column: 18, end_line: 34, end_column: 44 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:37:16-37:28) */
        { Site { function: 6, file: 0, line: 37, column: 16, end_line: 37, end_column: 28 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:38:13-38:38) */
        { Site { function: 6, file: 0, line: 38, column: 13, end_line: 38, end_column: 38 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:39:33-39:52) */
        { Site { function: 6, file: 0, line: 39, column: 33, end_line: 39, end_column: 52 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:39:13-39:53) */
        { Site { function: 6, file: 0, line: 39, column: 13, end_line: 39, end_column: 53 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:40:16-40:27) */
        { Site { function: 6, file: 0, line: 40, column: 16, end_line: 40, end_column: 27 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:43:17-43:29) */
        { Site { function: 6, file: 0, line: 43, column: 17, end_line: 43, end_column: 29 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:46:24-46:35) */
        { Site { function: 6, file: 0, line: 46, column: 24, end_line: 46, end_column: 35 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:48:23-48:35) */
        { Site { function: 6, file: 0, line: 48, column: 23, end_line: 48, end_column: 35 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:50:14-50:24) */
        { Site { function: 6, file: 0, line: 50, column: 14, end_line: 50, end_column: 24 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:51:13-51:29) */
        { Site { function: 6, file: 0, line: 51, column: 13, end_line: 51, end_column: 29 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:52:46-52:57) */
        { Site { function: 6, file: 0, line: 52, column: 46, end_line: 52, end_column: 57 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:53:13-53:30) */
        { Site { function: 6, file: 0, line: 53, column: 13, end_line: 53, end_column: 30 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:54:26-54:39) */
        { Site { function: 6, file: 0, line: 54, column: 26, end_line: 54, end_column: 39 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:56:27-56:45) */
        { Site { function: 6, file: 0, line: 56, column: 27, end_line: 56, end_column: 45 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:58:13-58:44) */
        { Site { function: 6, file: 0, line: 58, column: 13, end_line: 58, end_column: 44 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:60:17-60:34) */
        { Site { function: 6, file: 0, line: 60, column: 17, end_line: 60, end_column: 34 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:63:13-63:27) */
        { Site { function: 6, file: 0, line: 63, column: 13, end_line: 63, end_column: 27 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:63:36-63:49) */
        { Site { function: 6, file: 0, line: 63, column: 36, end_line: 63, end_column: 49 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:65:17-65:32) */
        { Site { function: 6, file: 0, line: 65, column: 17, end_line: 65, end_column: 32 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:68:13-68:33) */
        { Site { function: 6, file: 0, line: 68, column: 13, end_line: 68, end_column: 33 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:69:13-69:34) */
        { Site { function: 6, file: 0, line: 69, column: 13, end_line: 69, end_column: 34 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:70:13-70:33) */
        { Site { function: 6, file: 0, line: 70, column: 13, end_line: 70, end_column: 33 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:71:13-71:31) */
        { Site { function: 6, file: 0, line: 71, column: 13, end_line: 71, end_column: 31 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:73:17-73:36) */
        { Site { function: 6, file: 0, line: 73, column: 17, end_line: 73, end_column: 36 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:76:13-76:20) */
        { Site { function: 6, file: 0, line: 76, column: 13, end_line: 76, end_column: 20 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:77:13-77:31) */
        { Site { function: 6, file: 0, line: 77, column: 13, end_line: 77, end_column: 31 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:77:40-77:56) */
        { Site { function: 6, file: 0, line: 77, column: 40, end_line: 77, end_column: 56 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:77:71-77:95) */
        { Site { function: 6, file: 0, line: 77, column: 71, end_line: 77, end_column: 95 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:79:17-79:34) */
        { Site { function: 6, file: 0, line: 79, column: 17, end_line: 79, end_column: 34 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:83:23-83:48) */
        { Site { function: 6, file: 0, line: 83, column: 23, end_line: 83, end_column: 48 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:86:13-86:25) */
        { Site { function: 6, file: 0, line: 86, column: 13, end_line: 86, end_column: 25 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:88:17-88:30) */
        { Site { function: 6, file: 0, line: 88, column: 17, end_line: 88, end_column: 30 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:91:15-91:24) */
        { Site { function: 6, file: 0, line: 91, column: 15, end_line: 91, end_column: 24 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:92:13-92:29) */
        { Site { function: 6, file: 0, line: 92, column: 13, end_line: 92, end_column: 29 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:93:19-93:41) */
        { Site { function: 6, file: 0, line: 93, column: 19, end_line: 93, end_column: 41 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:94:13-94:33) */
        { Site { function: 6, file: 0, line: 94, column: 13, end_line: 94, end_column: 33 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:95:23-95:32) */
        { Site { function: 6, file: 0, line: 95, column: 23, end_line: 95, end_column: 32 } },
        /* terrane-site-row: site 46: /app::main (src/main.trn:97:13-97:30) */
        { Site { function: 6, file: 0, line: 97, column: 13, end_line: 97, end_column: 30 } },
        /* terrane-site-row: site 47: /app::main (src/main.trn:99:17-99:35) */
        { Site { function: 6, file: 0, line: 99, column: 17, end_line: 99, end_column: 35 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:102:13-102:36) */
        { Site { function: 6, file: 0, line: 102, column: 13, end_line: 102, end_column: 36 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:104:17-104:29) */
        { Site { function: 6, file: 0, line: 104, column: 17, end_line: 104, end_column: 29 } },
        /* terrane-site-row: site 50: /app::main (src/main.trn:107:13-107:36) */
        { Site { function: 6, file: 0, line: 107, column: 13, end_line: 107, end_column: 36 } },
        /* terrane-site-row: site 51: /app::main (src/main.trn:109:17-109:29) */
        { Site { function: 6, file: 0, line: 109, column: 17, end_line: 109, end_column: 29 } },
        /* terrane-site-row: site 52: /app::main (src/main.trn:112:13-112:33) */
        { Site { function: 6, file: 0, line: 112, column: 13, end_line: 112, end_column: 33 } },
        /* terrane-site-row: site 53: /app::main (src/main.trn:114:17-114:38) */
        { Site { function: 6, file: 0, line: 114, column: 17, end_line: 114, end_column: 38 } },
        /* terrane-site-row: site 54: /app::main (src/main.trn:117:14-117:28) */
        { Site { function: 6, file: 0, line: 117, column: 14, end_line: 117, end_column: 28 } },
        /* terrane-site-row: site 55: /app::main (src/main.trn:118:13-118:46) */
        { Site { function: 6, file: 0, line: 118, column: 13, end_line: 118, end_column: 46 } },
        /* terrane-site-row: site 56: /app::main (src/main.trn:120:17-120:51) */
        { Site { function: 6, file: 0, line: 120, column: 17, end_line: 120, end_column: 51 } },
        /* terrane-site-row: site 57: /app::main (src/main.trn:123:13-123:31) */
        { Site { function: 6, file: 0, line: 123, column: 13, end_line: 123, end_column: 31 } },
        /* terrane-site-row: site 58: /app::main (src/main.trn:125:17-125:36) */
        { Site { function: 6, file: 0, line: 125, column: 17, end_line: 125, end_column: 36 } },
        /* terrane-site-row: site 59: /app::main (src/main.trn:128:19-128:44) */
        { Site { function: 6, file: 0, line: 128, column: 19, end_line: 128, end_column: 44 } },
        /* terrane-site-row: site 60: /app::main (src/main.trn:130:23-130:41) */
        { Site { function: 6, file: 0, line: 130, column: 23, end_line: 130, end_column: 41 } },
        /* terrane-site-row: site 61: /app::main (src/main.trn:133:13-133:32) */
        { Site { function: 6, file: 0, line: 133, column: 13, end_line: 133, end_column: 32 } },
        /* terrane-site-row: site 62: /app::main (src/main.trn:135:17-135:34) */
        { Site { function: 6, file: 0, line: 135, column: 17, end_line: 135, end_column: 34 } },
        /* terrane-site-row: site 63: /app::main (src/main.trn:139:17-139:41) */
        { Site { function: 6, file: 0, line: 139, column: 17, end_line: 139, end_column: 41 } },
        /* terrane-site-row: site 64: /app::main (src/main.trn:143:23-143:46) */
        { Site { function: 6, file: 0, line: 143, column: 23, end_line: 143, end_column: 46 } },
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
// Source: <terrane>/projected/deps/facade.trn
// Namespace: deps/facade
pub use facade::BridgeReader;
pub use core::time::Duration;
pub use facade::Either;
pub use __terrane_recursive_anyhow::Error;
pub use facade::InferredClient;
pub use facade::Reader;
pub use facade::Response;
pub use facade::TransitiveFailure;
pub fn terrane_static_trn_427269646765526561646572_opaque_static(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::BridgeReader::opaque_static(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}", "facade",
                            "facade::BridgeReader"
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
                    "facade",
                    "facade::BridgeReader",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_526561646572_open(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::Reader::open(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
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
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::Reader"))
        }
    }
}
pub fn terrane_static_trn_4475726174696f6e_from_secs(
    secs: terrane_int_support::Int,
) -> Result<Duration, crate::TerraneForeignError> {
    let secs = terrane_int_support::coerce::<u64>(&secs)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| core::time::Duration::from_secs(secs)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "core::time::Duration",
                ),
            )
        }
    }
}
pub fn bridge_reader() -> Result<BridgeReader, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::bridge_reader()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::bridge_reader",
                ),
            )
        }
    }
}
pub fn character(
    i: terrane_int_support::Int,
) -> Result<Option<String>, crate::TerraneForeignError> {
    let i = terrane_int_support::coerce::<u32>(&i)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| core::char::from_u32(i)),
    ) {
        Ok(value) => Ok(value.map(|value| value.to_string())),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "core::char::from_u32",
                ),
            )
        }
    }
}
pub fn describe(
    value: &facade::Either<i64, String>,
) -> Result<String, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::describe(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::describe"))
        }
    }
}
pub fn duration() -> Result<Duration, crate::TerraneForeignError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| facade::duration())) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::duration"))
        }
    }
}
pub fn external(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::external(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
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
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::external"))
        }
    }
}
pub fn external_maybe(
    present: bool,
) -> Result<Option<terrane_int_support::Int>, crate::TerraneForeignError> {
    let present = present;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::external_maybe(present)),
    ) {
        Ok(value) => {
            Ok(value.map(|value| terrane_int_support::Int::from(i128::from(value))))
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::external_maybe",
                ),
            )
        }
    }
}
pub fn inferred_call(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::inferred_call(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}", "facade",
                            "facade::inferred_call"
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
                    "facade",
                    "facade::inferred_call",
                ),
            )
        }
    }
}
pub fn inferred_client() -> Result<InferredClient, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::inferred_client()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::inferred_client",
                ),
            )
        }
    }
}
pub fn maybe(
    present: bool,
) -> Result<Option<terrane_int_support::Int>, crate::TerraneForeignError> {
    let present = present;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::maybe(present)),
    ) {
        Ok(value) => {
            Ok(value.map(|value| terrane_int_support::Int::from(i128::from(value))))
        }
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::maybe"))
        }
    }
}
pub fn nested(
    present: bool,
    ok: bool,
) -> Result<Option<terrane_int_support::Int>, crate::TerraneForeignError> {
    let present = present;
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::nested(present, ok)),
    ) {
        Ok(None) => Ok(None),
        Ok(Some(Ok(value))) => {
            Ok(Some(terrane_int_support::Int::from(i128::from(value))))
        }
        Ok(Some(Err(error))) => {
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
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::nested"))
        }
    }
}
pub fn new_version(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::new_version(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
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
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::new_version",
                ),
            )
        }
    }
}
pub fn old_transitive(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::old_transitive(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::DescriptorId(5),
                        error.to_string(),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::old_transitive",
                ),
            )
        }
    }
}
pub fn opaque(ok: bool) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| facade::opaque(ok))) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}", "facade",
                            "facade::opaque"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::opaque"))
        }
    }
}
pub fn opaque_transitive(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::opaque_transitive(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}", "facade",
                            "facade::opaque_transitive"
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
                    "facade",
                    "facade::opaque_transitive",
                ),
            )
        }
    }
}
pub fn parse(
    input: &str,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::parse(input)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}", "facade",
                            "facade::parse"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::parse"))
        }
    }
}
pub fn pick(
    left: bool,
) -> Result<facade::Either<i64, String>, crate::TerraneForeignError> {
    let left = left;
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| facade::pick(left))) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::pick"))
        }
    }
}
pub fn public_error_text(
    error: TransitiveFailure,
) -> Result<String, crate::TerraneForeignError> {
    let error = error;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::public_error_text(error)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::public_error_text",
                ),
            )
        }
    }
}
pub fn public_error_value() -> Result<TransitiveFailure, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::public_error_value()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::public_error_value",
                ),
            )
        }
    }
}
pub fn public_transitive(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::public_transitive(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::DescriptorId(4),
                        error.to_string(),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::public_transitive",
                ),
            )
        }
    }
}
pub fn reader() -> Result<Reader, crate::TerraneForeignError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| facade::reader())) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::reader"))
        }
    }
}
pub fn renamed(
    ok: bool,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let ok = ok;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::renamed(ok)),
    ) {
        Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
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
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::renamed"))
        }
    }
}
pub fn response() -> Result<facade::Response<i64>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| facade::response())) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "facade", "facade::response"))
        }
    }
}
pub fn text(
    v: terrane_collection_support::List<terrane_int_support::Int>,
) -> Result<String, crate::TerraneForeignError> {
    let v = v
        .into_vec()
        .into_iter()
        .map(|item| -> Result<_, crate::TerraneForeignError> {
            Ok(
                terrane_int_support::coerce::<u8>(&item)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })
        .collect::<Result<std::vec::Vec<u8>, _>>()?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| core::str::from_utf8(&v)),
    ) {
        Ok(Ok(value)) => Ok(value.to_owned()),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}", "facade",
                            "core::str::from_utf8"
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
                    "facade",
                    "core::str::from_utf8",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/facade/util.trn
// Namespace: deps/facade/util
pub fn error_text(error: Error) -> Result<String, crate::TerraneForeignError> {
    let error = error;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::util::error_text(error)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::util::error_text",
                ),
            )
        }
    }
}
pub fn error_value() -> Result<Error, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| facade::util::error_value()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "facade",
                    "facade::util::error_value",
                ),
            )
        }
    }
}
