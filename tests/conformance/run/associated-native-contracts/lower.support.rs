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
    pub static DESCRIPTORS: [&str; 3] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
        "/deps/protocol::Failure",
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
    pub static FUNCTIONS: [&str; 6] = [
        "/app::dispatch",
        "/app::free-dispatch",
        "/app::static-dispatch",
        "/app::main",
        "/app::check-numeric-fields",
        "/app::check-recipe-selection",
    ];
    pub static SITES: [Site; 74] = [
        /* terrane-site-row: site 0: /app::dispatch (src/main.trn:24:16-24:40) */
        { Site { function: 0, file: 0, line: 24, column: 16, end_line: 24, end_column: 40 } },
        /* terrane-site-row: site 1: /app::free-dispatch (src/main.trn:26:16-26:35) */
        { Site { function: 1, file: 0, line: 26, column: 16, end_line: 26, end_column: 35 } },
        /* terrane-site-row: site 2: /app::static-dispatch (src/main.trn:28:16-28:42) */
        { Site { function: 2, file: 0, line: 28, column: 16, end_line: 28, end_column: 42 } },
        /* terrane-site-row: site 3: /app::dispatch (src/main.trn:35:16-35:40) */
        { Site { function: 0, file: 0, line: 35, column: 16, end_line: 35, end_column: 40 } },
        /* terrane-site-row: site 4: /app::dispatch (src/main.trn:42:16-42:40) */
        { Site { function: 0, file: 0, line: 42, column: 16, end_line: 42, end_column: 40 } },
        /* terrane-site-row: site 5: /app::dispatch (src/main.trn:49:16-49:40) */
        { Site { function: 0, file: 0, line: 49, column: 16, end_line: 49, end_column: 40 } },
        /* terrane-site-row: site 6: /app::dispatch (src/main.trn:56:16-56:40) */
        { Site { function: 0, file: 0, line: 56, column: 16, end_line: 56, end_column: 40 } },
        /* terrane-site-row: site 7: /app::dispatch (src/main.trn:63:16-63:37) */
        { Site { function: 0, file: 0, line: 63, column: 16, end_line: 63, end_column: 37 } },
        /* terrane-site-row: site 8: /app::dispatch (src/main.trn:70:16-70:40) */
        { Site { function: 0, file: 0, line: 70, column: 16, end_line: 70, end_column: 40 } },
        /* terrane-site-row: site 9: /app::dispatch (src/main.trn:77:16-77:40) */
        { Site { function: 0, file: 0, line: 77, column: 16, end_line: 77, end_column: 40 } },
        /* terrane-site-row: site 10: /app::dispatch (src/main.trn:84:16-84:40) */
        { Site { function: 0, file: 0, line: 84, column: 16, end_line: 84, end_column: 40 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:100:16-100:30) */
        { Site { function: 3, file: 0, line: 100, column: 16, end_line: 100, end_column: 30 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:102:40-102:66) */
        { Site { function: 3, file: 0, line: 102, column: 40, end_line: 102, end_column: 66 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:102:13-102:67) */
        { Site { function: 3, file: 0, line: 102, column: 13, end_line: 102, end_column: 67 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:105:40-105:72) */
        { Site { function: 3, file: 0, line: 105, column: 40, end_line: 105, end_column: 72 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:105:14-105:73) */
        { Site { function: 3, file: 0, line: 105, column: 14, end_line: 105, end_column: 73 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:107:35-107:61) */
        { Site { function: 3, file: 0, line: 107, column: 35, end_line: 107, end_column: 61 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:107:13-107:62) */
        { Site { function: 3, file: 0, line: 107, column: 13, end_line: 107, end_column: 62 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:109:38-109:64) */
        { Site { function: 3, file: 0, line: 109, column: 38, end_line: 109, end_column: 64 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:109:14-109:65) */
        { Site { function: 3, file: 0, line: 109, column: 14, end_line: 109, end_column: 65 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:111:14-111:30) */
        { Site { function: 3, file: 0, line: 111, column: 14, end_line: 111, end_column: 30 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:113:50-113:76) */
        { Site { function: 3, file: 0, line: 113, column: 50, end_line: 113, end_column: 76 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:116:42-116:68) */
        { Site { function: 3, file: 0, line: 116, column: 42, end_line: 116, end_column: 68 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:116:14-116:69) */
        { Site { function: 3, file: 0, line: 116, column: 14, end_line: 116, end_column: 69 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:118:16-118:25) */
        { Site { function: 3, file: 0, line: 118, column: 16, end_line: 118, end_column: 25 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:119:43-119:69) */
        { Site { function: 3, file: 0, line: 119, column: 43, end_line: 119, end_column: 69 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:119:15-119:70) */
        { Site { function: 3, file: 0, line: 119, column: 15, end_line: 119, end_column: 70 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:122:41-122:73) */
        { Site { function: 3, file: 0, line: 122, column: 41, end_line: 122, end_column: 73 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:122:15-122:74) */
        { Site { function: 3, file: 0, line: 122, column: 15, end_line: 122, end_column: 74 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:125:52-125:78) */
        { Site { function: 3, file: 0, line: 125, column: 52, end_line: 125, end_column: 78 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:128:48-128:80) */
        { Site { function: 3, file: 0, line: 128, column: 48, end_line: 128, end_column: 80 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:128:20-128:81) */
        { Site { function: 3, file: 0, line: 128, column: 20, end_line: 128, end_column: 81 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:130:16-130:42) */
        { Site { function: 3, file: 0, line: 130, column: 16, end_line: 130, end_column: 42 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:131:16-131:33) */
        { Site { function: 3, file: 0, line: 131, column: 16, end_line: 131, end_column: 33 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:135:51-135:78) */
        { Site { function: 3, file: 0, line: 135, column: 51, end_line: 135, end_column: 78 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:135:23-135:79) */
        { Site { function: 3, file: 0, line: 135, column: 23, end_line: 135, end_column: 79 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:137:77-137:107) */
        { Site { function: 3, file: 0, line: 137, column: 77, end_line: 137, end_column: 107 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:141:59-141:91) */
        { Site { function: 3, file: 0, line: 141, column: 59, end_line: 141, end_column: 91 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:141:19-141:92) */
        { Site { function: 3, file: 0, line: 141, column: 19, end_line: 141, end_column: 92 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:143:12-143:54) */
        { Site { function: 3, file: 0, line: 143, column: 12, end_line: 143, end_column: 54 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:145:14-145:64) */
        { Site { function: 3, file: 0, line: 145, column: 14, end_line: 145, end_column: 64 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:146:13-146:32) */
        { Site { function: 3, file: 0, line: 146, column: 13, end_line: 146, end_column: 32 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:149:16-149:27) */
        { Site { function: 3, file: 0, line: 149, column: 16, end_line: 149, end_column: 27 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:150:14-150:39) */
        { Site { function: 3, file: 0, line: 150, column: 14, end_line: 150, end_column: 39 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:151:13-151:32) */
        { Site { function: 3, file: 0, line: 151, column: 13, end_line: 151, end_column: 32 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:153:36-153:62) */
        { Site { function: 3, file: 0, line: 153, column: 36, end_line: 153, end_column: 62 } },
        /* terrane-site-row: site 46: /app::main (src/main.trn:153:9-153:63) */
        { Site { function: 3, file: 0, line: 153, column: 9, end_line: 153, end_column: 63 } },
        /* terrane-site-row: site 47: /app::main (src/main.trn:156:21-156:31) */
        { Site { function: 3, file: 0, line: 156, column: 21, end_line: 156, end_column: 31 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:158:34-158:46) */
        { Site { function: 3, file: 0, line: 158, column: 34, end_line: 158, end_column: 46 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:159:17-159:29) */
        { Site { function: 3, file: 0, line: 159, column: 17, end_line: 159, end_column: 29 } },
        /* terrane-site-row: site 50: /app::main (src/main.trn:160:34-160:45) */
        { Site { function: 3, file: 0, line: 160, column: 34, end_line: 160, end_column: 45 } },
        /* terrane-site-row: site 51: /app::main (src/main.trn:161:20-161:31) */
        { Site { function: 3, file: 0, line: 161, column: 20, end_line: 161, end_column: 31 } },
        /* terrane-site-row: site 52: /app::main (src/main.trn:164:5-164:37) */
        { Site { function: 3, file: 0, line: 164, column: 5, end_line: 164, end_column: 37 } },
        /* terrane-site-row: site 53: /app::dispatch (src/main.trn:171:16-171:40) */
        { Site { function: 0, file: 0, line: 171, column: 16, end_line: 171, end_column: 40 } },
        /* terrane-site-row: site 54: /app::dispatch (src/main.trn:178:16-178:40) */
        { Site { function: 0, file: 0, line: 178, column: 16, end_line: 178, end_column: 40 } },
        /* terrane-site-row: site 55: /app::dispatch (src/main.trn:185:16-185:40) */
        { Site { function: 0, file: 0, line: 185, column: 16, end_line: 185, end_column: 40 } },
        /* terrane-site-row: site 56: /app::check-numeric-fields (src/main.trn:193:14-193:66) */
        { Site { function: 4, file: 0, line: 193, column: 14, end_line: 193, end_column: 66 } },
        /* terrane-site-row: site 57: /app::check-numeric-fields (src/main.trn:195:22-195:30) */
        { Site { function: 4, file: 0, line: 195, column: 22, end_line: 195, end_column: 30 } },
        /* terrane-site-row: site 58: /app::check-numeric-fields (src/main.trn:196:20-196:28) */
        { Site { function: 4, file: 0, line: 196, column: 20, end_line: 196, end_column: 28 } },
        /* terrane-site-row: site 59: /app::check-numeric-fields (src/main.trn:200:22-200:28) */
        { Site { function: 4, file: 0, line: 200, column: 22, end_line: 200, end_column: 28 } },
        /* terrane-site-row: site 60: /app::check-numeric-fields (src/main.trn:202:22-202:30) */
        { Site { function: 4, file: 0, line: 202, column: 22, end_line: 202, end_column: 30 } },
        /* terrane-site-row: site 61: /app::check-numeric-fields (src/main.trn:205:26-205:33) */
        { Site { function: 4, file: 0, line: 205, column: 26, end_line: 205, end_column: 33 } },
        /* terrane-site-row: site 62: /app::check-recipe-selection (src/main.trn:211:43-211:49) */
        { Site { function: 5, file: 0, line: 211, column: 43, end_line: 211, end_column: 49 } },
        /* terrane-site-row: site 63: /app::check-recipe-selection (src/main.trn:211:13-211:50) */
        { Site { function: 5, file: 0, line: 211, column: 13, end_line: 211, end_column: 50 } },
        /* terrane-site-row: site 64: /app::check-recipe-selection (src/main.trn:213:47-213:54) */
        { Site { function: 5, file: 0, line: 213, column: 47, end_line: 213, end_column: 54 } },
        /* terrane-site-row: site 65: /app::check-recipe-selection (src/main.trn:213:13-213:55) */
        { Site { function: 5, file: 0, line: 213, column: 13, end_line: 213, end_column: 55 } },
        /* terrane-site-row: site 66: /app::check-recipe-selection (src/main.trn:215:50-215:66) */
        { Site { function: 5, file: 0, line: 215, column: 50, end_line: 215, end_column: 66 } },
        /* terrane-site-row: site 67: /app::check-recipe-selection (src/main.trn:215:14-215:67) */
        { Site { function: 5, file: 0, line: 215, column: 14, end_line: 215, end_column: 67 } },
        /* terrane-site-row: site 68: /app::check-recipe-selection (src/main.trn:217:22-217:48) */
        { Site { function: 5, file: 0, line: 217, column: 22, end_line: 217, end_column: 48 } },
        /* terrane-site-row: site 69: /app::check-recipe-selection (src/main.trn:219:23-219:45) */
        { Site { function: 5, file: 0, line: 219, column: 23, end_line: 219, end_column: 45 } },
        /* terrane-site-row: site 70: /app::check-recipe-selection (src/main.trn:221:19-221:29) */
        { Site { function: 5, file: 0, line: 221, column: 19, end_line: 221, end_column: 29 } },
        /* terrane-site-row: site 71: /app::check-recipe-selection (src/main.trn:222:13-222:38) */
        { Site { function: 5, file: 0, line: 222, column: 13, end_line: 222, end_column: 38 } },
        /* terrane-site-row: site 72: /app::check-recipe-selection (src/main.trn:223:19-223:32) */
        { Site { function: 5, file: 0, line: 223, column: 19, end_line: 223, end_column: 32 } },
        /* terrane-site-row: site 73: /app::check-recipe-selection (src/main.trn:224:18-224:43) */
        { Site { function: 5, file: 0, line: 224, column: 18, end_line: 224, end_column: 43 } },
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
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneFieldMetadata {
    name: &'static str,
    external_name: &'static str,
    defaulted: bool,
    optional: bool,
    secret: bool,
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
    inherently_identity_bearing: bool,
    fields: &'static [TerraneFieldMetadata],
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
// Source: <terrane>/projected/deps/protocol.trn
// Namespace: deps/protocol
pub use witness::Alpha;
pub use witness::AlphaReply;
pub type ArcOfTerraneAssociatedCallExecutor = std::sync::Arc<witness::Executor>;
pub use witness::Batch;
pub use witness::Beta;
pub use witness::BorrowedOutput;
pub use witness::Echo;
pub use witness::EchoReply;
pub use witness::Executor;
pub use witness::Marked;
pub use witness::Maybe;
pub use witness::Mode;
pub use witness::NumericFields;
pub use witness::SharedExecutor;
pub use protocol::SharedReply;
pub use witness::Tagged;
pub use witness::TaggedReply;
pub use witness::Tree;
pub use witness::TreeRequest;
pub use witness::Unique;
pub use witness::UniqueReply;
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub fn terrane_static_trn_4578656375746f72_new() -> Result<
    Executor,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::Executor::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::Executor",
                ),
            )
        }
    }
}
pub fn batch() -> Result<
    witness::Batch<std::vec::Vec<String>>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| witness::batch())) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::batch"))
        }
    }
}
pub fn borrowed_output() -> Result<BorrowedOutput, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::borrowed_output()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::borrowed_output",
                ),
            )
        }
    }
}
pub fn child_count(
    tree: &Tree,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::child_count(tree)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::child_count",
                ),
            )
        }
    }
}
pub fn mapped_batch() -> Result<
    witness::Batch<std::collections::BTreeMap<String, witness::AlphaReply>>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::mapped_batch()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::mapped_batch",
                ),
            )
        }
    }
}
pub fn marked() -> Result<Marked, crate::TerraneForeignError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| witness::marked())) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::marked"))
        }
    }
}
pub fn mode(present: bool) -> Result<Option<Mode>, crate::TerraneForeignError> {
    let present = present;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::mode(present)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::mode"))
        }
    }
}
pub fn set_batch() -> Result<
    witness::Batch<std::collections::BTreeSet<String>>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::set_batch()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::set_batch",
                ),
            )
        }
    }
}
pub fn shared_executor() -> Result<
    std::sync::Arc<witness::Executor>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::shared_executor()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::shared_executor",
                ),
            )
        }
    }
}
