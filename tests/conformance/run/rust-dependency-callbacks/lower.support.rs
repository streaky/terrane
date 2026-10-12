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
    pub static FUNCTIONS: [&str; 8] = [
        "/app::stay-pending",
        "/app::run-retained",
        "/app",
        "/app::shared",
        "/app::mutable",
        "/app::consuming",
        "/app::destruct",
        "/app::main",
    ];
    pub static SITES: [Site; 43] = [
        /* terrane-site-row: site 0: /app::stay-pending (src/main.trn:20:24-20:44) */
        { Site { function: 0, file: 0, line: 20, column: 24, end_line: 20, end_column: 44 } },
        /* terrane-site-row: site 1: /app::run-retained (src/main.trn:25:22-25:51) */
        { Site { function: 1, file: 0, line: 25, column: 22, end_line: 25, end_column: 51 } },
        /* terrane-site-row: site 2: /app (src/main.trn:60:24-60:40) */
        { Site { function: 2, file: 0, line: 60, column: 24, end_line: 60, end_column: 40 } },
        /* terrane-site-row: site 3: /app::shared (src/main.trn:72:30-72:46) */
        { Site { function: 3, file: 0, line: 72, column: 30, end_line: 72, end_column: 46 } },
        /* terrane-site-row: site 4: /app::shared (src/main.trn:74:17-74:44) */
        { Site { function: 3, file: 0, line: 74, column: 17, end_line: 74, end_column: 44 } },
        /* terrane-site-row: site 5: /app::shared (src/main.trn:76:13-76:40) */
        { Site { function: 3, file: 0, line: 76, column: 13, end_line: 76, end_column: 40 } },
        /* terrane-site-row: site 6: /app::mutable (src/main.trn:81:30-81:46) */
        { Site { function: 4, file: 0, line: 81, column: 30, end_line: 81, end_column: 46 } },
        /* terrane-site-row: site 7: /app::mutable (src/main.trn:83:17-83:44) */
        { Site { function: 4, file: 0, line: 83, column: 17, end_line: 83, end_column: 44 } },
        /* terrane-site-row: site 8: /app::mutable (src/main.trn:85:13-85:40) */
        { Site { function: 4, file: 0, line: 85, column: 13, end_line: 85, end_column: 40 } },
        /* terrane-site-row: site 9: /app::consuming (src/main.trn:90:30-90:46) */
        { Site { function: 5, file: 0, line: 90, column: 30, end_line: 90, end_column: 46 } },
        /* terrane-site-row: site 10: /app::consuming (src/main.trn:92:17-92:44) */
        { Site { function: 5, file: 0, line: 92, column: 17, end_line: 92, end_column: 44 } },
        /* terrane-site-row: site 11: /app::consuming (src/main.trn:94:13-94:40) */
        { Site { function: 5, file: 0, line: 94, column: 13, end_line: 94, end_column: 40 } },
        /* terrane-site-row: site 12: /app::destruct (src/main.trn:97:17-97:43) */
        { Site { function: 6, file: 0, line: 97, column: 17, end_line: 97, end_column: 43 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:103:13-103:38) */
        { Site { function: 7, file: 0, line: 103, column: 13, end_line: 103, end_column: 38 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:104:13-104:39) */
        { Site { function: 7, file: 0, line: 104, column: 13, end_line: 104, end_column: 39 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:105:13-105:45) */
        { Site { function: 7, file: 0, line: 105, column: 13, end_line: 105, end_column: 45 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:106:28-106:67) */
        { Site { function: 7, file: 0, line: 106, column: 28, end_line: 106, end_column: 67 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:111:28-111:68) */
        { Site { function: 7, file: 0, line: 111, column: 28, end_line: 111, end_column: 68 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:113:25-113:35) */
        { Site { function: 7, file: 0, line: 113, column: 25, end_line: 113, end_column: 35 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:114:13-114:48) */
        { Site { function: 7, file: 0, line: 114, column: 13, end_line: 114, end_column: 48 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:116:13-116:49) */
        { Site { function: 7, file: 0, line: 116, column: 13, end_line: 116, end_column: 49 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:119:25-119:63) */
        { Site { function: 7, file: 0, line: 119, column: 25, end_line: 119, end_column: 63 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:122:49-122:77) */
        { Site { function: 7, file: 0, line: 122, column: 49, end_line: 122, end_column: 77 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:124:13-124:47) */
        { Site { function: 7, file: 0, line: 124, column: 13, end_line: 124, end_column: 47 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:125:13-125:49) */
        { Site { function: 7, file: 0, line: 125, column: 13, end_line: 125, end_column: 49 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:129:13-129:42) */
        { Site { function: 7, file: 0, line: 129, column: 13, end_line: 129, end_column: 42 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:131:13-131:48) */
        { Site { function: 7, file: 0, line: 131, column: 13, end_line: 131, end_column: 48 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:133:13-133:37) */
        { Site { function: 7, file: 0, line: 133, column: 13, end_line: 133, end_column: 37 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:135:13-135:42) */
        { Site { function: 7, file: 0, line: 135, column: 13, end_line: 135, end_column: 42 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:136:24-136:50) */
        { Site { function: 7, file: 0, line: 136, column: 24, end_line: 136, end_column: 50 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:137:13-137:39) */
        { Site { function: 7, file: 0, line: 137, column: 13, end_line: 137, end_column: 39 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:137:43-137:68) */
        { Site { function: 7, file: 0, line: 137, column: 43, end_line: 137, end_column: 68 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:139:19-139:57) */
        { Site { function: 7, file: 0, line: 139, column: 19, end_line: 139, end_column: 57 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:140:13-140:35) */
        { Site { function: 7, file: 0, line: 140, column: 13, end_line: 140, end_column: 35 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:140:39-140:59) */
        { Site { function: 7, file: 0, line: 140, column: 39, end_line: 140, end_column: 59 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:142:20-142:57) */
        { Site { function: 7, file: 0, line: 142, column: 20, end_line: 142, end_column: 57 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:143:13-143:35) */
        { Site { function: 7, file: 0, line: 143, column: 13, end_line: 143, end_column: 35 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:143:39-143:60) */
        { Site { function: 7, file: 0, line: 143, column: 39, end_line: 143, end_column: 60 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:144:19-144:79) */
        { Site { function: 7, file: 0, line: 144, column: 19, end_line: 144, end_column: 79 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:145:13-145:34) */
        { Site { function: 7, file: 0, line: 145, column: 13, end_line: 145, end_column: 34 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:145:38-145:58) */
        { Site { function: 7, file: 0, line: 145, column: 38, end_line: 145, end_column: 58 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:146:13-146:55) */
        { Site { function: 7, file: 0, line: 146, column: 13, end_line: 146, end_column: 55 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:147:19-147:72) */
        { Site { function: 7, file: 0, line: 147, column: 19, end_line: 147, end_column: 72 } },
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
use std::future::Future;
#[derive(Clone)]
struct TerraneCancellation {
    state: std::sync::Arc<TerraneCancellationState>,
}
struct TerraneCancellationState {
    cancelled: std::sync::atomic::AtomicBool,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
}
impl TerraneCancellation {
    fn new() -> Self {
        Self {
            state: std::sync::Arc::new(TerraneCancellationState {
                cancelled: std::sync::atomic::AtomicBool::new(false),
                wakers: std::sync::Mutex::new(Vec::new()),
            }),
        }
    }
    fn wake_waiters(&self) {
        let wakers = std::mem::take(
            &mut *self.state.wakers.lock().expect("cancellation waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }
    fn cancel(&self) {
        if !self.state.cancelled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.wake_waiters();
        }
    }
    fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }
    async fn cancelled(&self) {
        std::future::poll_fn(|context| {
                if self.is_cancelled() {
                    return std::task::Poll::Ready(());
                }
                let mut wakers = self
                    .state
                    .wakers
                    .lock()
                    .expect("cancellation waker lock poisoned");
                if self.is_cancelled() {
                    return std::task::Poll::Ready(());
                }
                if !wakers.iter().any(|waker| waker.will_wake(context.waker())) {
                    wakers.push(context.waker().clone());
                }
                std::task::Poll::Pending
            })
            .await
    }
}
struct TerraneFinalizerState {
    depth: std::sync::atomic::AtomicUsize,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
}
impl TerraneFinalizerState {
    fn new() -> Self {
        Self {
            depth: std::sync::atomic::AtomicUsize::new(0),
            wakers: std::sync::Mutex::new(Vec::new()),
        }
    }
    #[allow(
        dead_code,
        reason = "native scope support is shared by packages without asynchronous finally"
    )]
    fn register(self: &std::sync::Arc<Self>) -> TerraneFinallyGuard {
        let depth = self.depth.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1;
        TerraneFinallyGuard {
            state: Some(self.clone()),
            depth,
        }
    }
    #[allow(
        dead_code,
        reason = "native scope support is shared by packages without asynchronous finally"
    )]
    fn unregister(&self, depth: usize) {
        let current = self.depth.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        debug_assert_eq!(current, depth, "finally regions must leave innermost first");
        let wakers = std::mem::take(
            &mut *self.wakers.lock().expect("finalizer waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }
    #[allow(
        dead_code,
        reason = "projected cleanup tracking is shared by packages without projected entries"
    )]
    async fn reaches(&self, depth: usize) {
        std::future::poll_fn(|context| {
                if self.depth.load(std::sync::atomic::Ordering::Acquire) == depth {
                    return std::task::Poll::Ready(());
                }
                let mut wakers = self
                    .wakers
                    .lock()
                    .expect("finalizer waker lock poisoned");
                if self.depth.load(std::sync::atomic::Ordering::Acquire) == depth {
                    return std::task::Poll::Ready(());
                }
                if !wakers.iter().any(|waker| waker.will_wake(context.waker())) {
                    wakers.push(context.waker().clone());
                }
                std::task::Poll::Pending
            })
            .await
    }
    fn depth(&self) -> usize {
        self.depth.load(std::sync::atomic::Ordering::Acquire)
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneCancellationContext {
    cancellation: TerraneCancellation,
    deadline: Option<terrane_int_support::Int>,
    finalizers: std::sync::Arc<TerraneFinalizerState>,
}
tokio::task_local! {
    static TERRANE_CANCELLATION_CONTEXT : TerraneCancellationContext;
}
static TERRANE_PROJECTED_CLEANUPS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
    0,
);
static TERRANE_PROJECTED_CLEANUPS_CHANGED: tokio::sync::Notify = tokio::sync::Notify::const_new();
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
struct TerraneProjectedEntryGuard {
    cancellation: TerraneCancellation,
    finalizers: std::sync::Arc<TerraneFinalizerState>,
    abort: Option<tokio::task::AbortHandle>,
    armed: bool,
}
impl Drop for TerraneProjectedEntryGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        TERRANE_PROJECTED_CLEANUPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.cancellation.cancel();
        let finalizers = self.finalizers.clone();
        let abort = self
            .abort
            .take()
            .expect("armed projected entry guard must own its abort handle");
        tokio::spawn(async move {
            finalizers.reaches(0).await;
            abort.abort();
            if TERRANE_PROJECTED_CLEANUPS
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1
            {
                TERRANE_PROJECTED_CLEANUPS_CHANGED.notify_waiters();
            }
        });
    }
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
async fn __terrane_projected_async_entry<F, T>(future: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let cancellation = TerraneCancellation::new();
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline: None,
        finalizers: finalizers.clone(),
    };
    let mut future = Box::pin(TERRANE_CANCELLATION_CONTEXT.scope(context, future));
    let first_poll = std::future::poll_fn(|cx| std::task::Poll::Ready(
            future.as_mut().poll(cx),
        ))
        .await;
    if let std::task::Poll::Ready(output) = first_poll {
        return output;
    }
    let task = tokio::spawn(future);
    let mut guard = TerraneProjectedEntryGuard {
        cancellation,
        finalizers,
        abort: Some(task.abort_handle()),
        armed: true,
    };
    let output = task
        .await
        .expect("projected asynchronous dependency entry task failed");
    guard.armed = false;
    output
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
async fn __terrane_wait_projected_cleanups() {
    loop {
        let changed = TERRANE_PROJECTED_CLEANUPS_CHANGED.notified();
        if TERRANE_PROJECTED_CLEANUPS.load(std::sync::atomic::Ordering::SeqCst) == 0 {
            return;
        }
        changed.await;
    }
}
fn __terrane_await_destructor<F>(future: F)
where
    F: Future<Output = ()> + Send,
{
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
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneFinallyGuard {
    state: Option<std::sync::Arc<TerraneFinalizerState>>,
    depth: usize,
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
impl TerraneFinallyGuard {
    fn finish(&mut self) {
        if let Some(state) = self.state.take() {
            state.unregister(self.depth);
        }
    }
}
impl Drop for TerraneFinallyGuard {
    fn drop(&mut self) {
        self.finish();
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
#[derive(Clone)]
struct TerraneSelectControl {
    requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
    waker: std::sync::Arc<std::sync::Mutex<Option<std::task::Waker>>>,
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
impl TerraneSelectControl {
    fn request_cancel(&self) {
        self.requested.store(true, std::sync::atomic::Ordering::Release);
        if let Some(waker) = self
            .waker
            .lock()
            .expect("select cancellation waker lock poisoned")
            .take()
        {
            waker.wake();
        }
    }
    fn is_cancelled(&self) -> bool {
        self.requested.load(std::sync::atomic::Ordering::Acquire)
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
fn __terrane_select_control() -> TerraneSelectControl {
    TerraneSelectControl {
        requested: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: std::sync::Arc::new(std::sync::Mutex::new(None)),
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
async fn __terrane_select_operation<F: Future>(
    control: TerraneSelectControl,
    future: F,
) -> Option<F::Output> {
    let deadline = TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| context.deadline.clone())
        .ok()
        .flatten();
    let cancellation = TerraneCancellation::new();
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline,
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async move {
                let mut future = std::pin::pin!(future);
                std::future::poll_fn(move |cx| {
                        if control.is_cancelled() {
                            cancellation.cancel();
                            let mut stored_waker = control
                                .waker
                                .lock()
                                .expect("select cancellation waker lock poisoned");
                            if stored_waker
                                .as_ref()
                                .is_none_or(|waker| !waker.will_wake(cx.waker()))
                            {
                                *stored_waker = Some(cx.waker().clone());
                            }
                            drop(stored_waker);
                            return match Future::poll(future.as_mut(), cx) {
                                std::task::Poll::Ready(output) => {
                                    std::task::Poll::Ready(Some(output))
                                }
                                std::task::Poll::Pending if finalizers.depth() == 0 => {
                                    std::task::Poll::Ready(None)
                                }
                                std::task::Poll::Pending => std::task::Poll::Pending,
                            };
                        }
                        Future::poll(future.as_mut(), cx).map(Some)
                    })
                    .await
            },
        )
        .await
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
fn __terrane_finally_guard() -> TerraneFinallyGuard {
    TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| context.finalizers.register())
        .unwrap_or(TerraneFinallyGuard {
            state: None,
            depth: 0,
        })
}
async fn __terrane_cancellation_requested(
    cancellation: TerraneCancellation,
    deadline: Option<terrane_int_support::Int>,
) {
    if let Some(deadline) = deadline {
        tokio::select! {
            () = cancellation.cancelled() => {} () = terrane_time_sleep_until(deadline)
            => { cancellation.cancel(); }
        }
    } else {
        cancellation.cancelled().await;
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
fn __terrane_cancellation_is_requested() -> bool {
    TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| {
            if context.deadline.as_ref().is_some_and(terrane_time_deadline_expired) {
                context.cancellation.cancel();
            }
            context.cancellation.is_cancelled()
        })
        .unwrap_or(false)
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
async fn __terrane_cancel_operation<F: Future>(
    guard: &TerraneFinallyGuard,
    future: F,
) -> Option<F::Output> {
    let cancellation = TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| {
            guard
                .state
                .as_ref()
                .map(|state| {
                    (
                        context.cancellation.clone(),
                        context.deadline.clone(),
                        state.clone(),
                    )
                })
        })
        .ok()
        .flatten();
    if let Some((cancellation, deadline, finalizers)) = cancellation {
        let mut future = std::pin::pin!(future);
        tokio::select! {
            biased; output = future.as_mut() => Some(output), () =
            __terrane_cancellation_requested(cancellation, deadline) => {
            std::future::poll_fn(| cx | { match Future::poll(future.as_mut(), cx) {
            std::task::Poll::Ready(output) => { std::task::Poll::Ready(Some(output)) }
            std::task::Poll::Pending if finalizers.depth() == guard.depth => {
            std::task::Poll::Ready(None) } std::task::Poll::Pending =>
            std::task::Poll::Pending, } }). await },
        }
    } else {
        Some(future.await)
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
async fn __terrane_finish_cancelled_finally(mut guard: TerraneFinallyGuard) -> ! {
    guard.finish();
    std::future::pending().await
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
async fn __terrane_finish_cancelled_select(mut guard: TerraneFinallyGuard) -> ! {
    guard.finish();
    std::future::pending().await
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
async fn __terrane_cancellable<F: Future>(
    future: F,
    cancellation: TerraneCancellation,
    deadline: Option<terrane_int_support::Int>,
) -> Option<F::Output> {
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline: deadline.clone(),
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async {
                let mut future = std::pin::pin!(future);
                tokio::select! {
                    biased; output = future.as_mut() => Some(output), () =
                    __terrane_cancellation_requested(cancellation, deadline.clone()) => {
                    std::future::poll_fn(| cx | { match Future::poll(future.as_mut(), cx)
                    { std::task::Poll::Ready(output) => {
                    std::task::Poll::Ready(Some(output)) } std::task::Poll::Pending if
                    finalizers.depth() == 0 => { std::task::Poll::Ready(None) }
                    std::task::Poll::Pending => std::task::Poll::Pending, } }). await },
                }
            },
        )
        .await
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize");
    tokio::task::LocalSet::new()
        .block_on(
            &runtime,
            async move {
                let output = future.await;
                __terrane_wait_projected_cleanups().await;
                output
            },
        )
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
#[derive(Clone)]
pub struct TerraneTaskScope {
    cancellation: TerraneCancellation,
    deadline: Option<terrane_int_support::Int>,
}
impl TerraneTaskScope {
    pub fn new(deadline: Option<terrane_int_support::Int>) -> Self {
        Self {
            cancellation: TerraneCancellation::new(),
            deadline,
        }
    }
    pub fn child_scope(&self, requested: &terrane_int_support::Int) -> Self {
        Self {
            cancellation: self.cancellation.clone(),
            deadline: Some(
                self
                    .deadline
                    .as_ref()
                    .map_or_else(
                        || requested.clone(),
                        |parent| parent.clone().min(requested.clone()),
                    ),
            ),
        }
    }
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }
    pub fn should_cancel(&self) -> bool {
        self.cancellation.is_cancelled()
            || self.deadline.as_ref().is_some_and(terrane_time_deadline_expired)
    }
    fn cancellation(&self) -> TerraneCancellation {
        self.cancellation.clone()
    }
    pub async fn join<T>(
        &self,
        mut task: TerraneScopedTask<T>,
    ) -> TerraneTaskOutcome<T> {
        let result = task
            .handle
            .take()
            .expect("scoped task joined once")
            .await
            .expect("scoped task must not panic outside its Terrane boundary");
        outcome_from_result(self, result)
    }
}
#[allow(
    dead_code,
    reason = "task result ABI is emitted before per-variant usage shaping"
)]
enum TerraneTaskResult<T> {
    Completed(T),
    Failed(TerraneError),
    Cancelled,
}
pub struct TerraneScopedTask<T> {
    handle: Option<tokio::task::JoinHandle<TerraneTaskResult<T>>>,
}
impl<T: 'static> TerraneScopedTask<T> {
    #[allow(dead_code, reason = "task spawn ABI is emitted before usage shaping")]
    fn spawn<F: Future<Output = TerraneTaskResult<T>> + 'static>(work: F) -> Self {
        Self {
            handle: Some(tokio::task::spawn_local(work)),
        }
    }
}
fn outcome_from_result<T>(
    scope: &TerraneTaskScope,
    result: TerraneTaskResult<T>,
) -> TerraneTaskOutcome<T> {
    match result {
        TerraneTaskResult::Completed(value) => {
            TerraneTaskOutcome {
                completed: true,
                cancelled: scope.should_cancel(),
                value: Some(value),
                error: None,
            }
        }
        TerraneTaskResult::Failed(error) => {
            scope.cancel();
            TerraneTaskOutcome {
                completed: false,
                cancelled: false,
                value: None,
                error: Some(error),
            }
        }
        TerraneTaskResult::Cancelled => {
            TerraneTaskOutcome {
                completed: false,
                cancelled: true,
                value: None,
                error: None,
            }
        }
    }
}
pub struct TerraneTaskOutcome<T> {
    pub completed: bool,
    pub cancelled: bool,
    pub value: Option<T>,
    pub error: Option<TerraneError>,
}
#[allow(
    dead_code,
    reason = "native cancellation omits deadline polling when no clock package is used"
)]
fn terrane_time_deadline_expired(_target: &terrane_int_support::Int) -> bool {
    false
}
#[allow(
    dead_code,
    reason = "native cancellation omits deadline polling when no clock package is used"
)]
async fn terrane_time_sleep_until(_target: terrane_int_support::Int) {
    std::future::pending::<()>().await;
}
trait TerraneMutableCallableBody<Arguments, Output>: Send {
    fn call(&mut self, arguments: Arguments) -> Output;
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>>;
}
impl<Arguments, Output, Function> TerraneMutableCallableBody<Arguments, Output>
for Function
where
    Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
{
    fn call(&mut self, arguments: Arguments) -> Output {
        self(arguments)
    }
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>> {
        std::boxed::Box::new(self.clone())
    }
}
pub struct TerraneMutableCallable<Arguments, Output> {
    body: std::sync::Mutex<
        std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>>,
    >,
}
impl<Arguments, Output> TerraneMutableCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
    {
        Self {
            body: std::sync::Mutex::new(std::boxed::Box::new(function)),
        }
    }
    fn call(&self, arguments: Arguments) -> Output {
        self.body.lock().expect("mutable callable lock poisoned").call(arguments)
    }
}
impl<Arguments, Output> Clone for TerraneMutableCallable<Arguments, Output> {
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
// Source: <terrane>/projected/deps/terrane-callback-witness.trn
// Namespace: deps/terrane-callback-witness
pub trait AdjustableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn AdjustableProtocol>;
    fn separate_box(&self) -> Box<dyn AdjustableProtocol>;
    fn adjust(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int;
    fn current(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn AdjustableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Adjustable(Box<dyn AdjustableProtocol>);
impl Clone for Adjustable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Adjustable {
    pub fn adjust(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.0.adjust(delta)
    }
    pub fn current(&self) -> terrane_int_support::Int {
        self.0.current()
    }
}
impl terrane_callback_witness::Adjustable for Adjustable {
    fn adjust(&mut self, delta: i64) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Adjustable>::adjust(
                &mut *self,
                terrane_int_support::Int::from(i128::from(delta)),
            );
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn current(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Adjustable>::current(&*self);
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
pub trait AsyncEntryProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn AsyncEntryProtocol>;
    fn separate_box(&self) -> Box<dyn AsyncEntryProtocol>;
    fn shared(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + '_>,
    >;
    fn mutable(
        &mut self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + '_>,
    >;
    fn consuming(
        self: Box<Self>,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = terrane_int_support::Int> + Send + 'static>,
    >;
}
impl Clone for Box<dyn AsyncEntryProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct AsyncEntry(Box<dyn AsyncEntryProtocol>);
impl Clone for AsyncEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl AsyncEntry {
    pub async fn shared(&self) -> terrane_int_support::Int {
        self.0.shared().await
    }
    pub async fn mutable(&mut self) -> terrane_int_support::Int {
        self.0.mutable().await
    }
    pub async fn consuming(self) -> terrane_int_support::Int {
        self.0.consuming().await
    }
}
impl terrane_callback_witness::AsyncEntry for AsyncEntry {
    async fn shared(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = {
                let __terrane_receiver = self.clone();
                __terrane_projected_async_entry(async move {
                        <AsyncEntry>::shared(&__terrane_receiver).await
                    })
                    .await
            };
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    async fn mutable(&mut self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = {
                let mut __terrane_receiver = self.clone();
                let (__terrane_output, __terrane_receiver) = __terrane_projected_async_entry(async move {
                        let __terrane_output = <AsyncEntry>::mutable(
                                &mut __terrane_receiver,
                            )
                            .await;
                        (__terrane_output, __terrane_receiver)
                    })
                    .await;
                *self = __terrane_receiver;
                __terrane_output
            };
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    async fn consuming(self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = async {
            let __terrane_value = __terrane_projected_async_entry(async move {
                    <AsyncEntry>::consuming(self).await
                })
                .await;
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }
            .await;
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait DropAwareProtocol {
    fn clone_box(&self) -> Box<dyn DropAwareProtocol>;
    fn separate_box(&self) -> Box<dyn DropAwareProtocol>;
    fn label(&self) -> String;
}
impl Clone for Box<dyn DropAwareProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct DropAware(Box<dyn DropAwareProtocol>);
impl Clone for DropAware {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Drop for DropAware {
    fn drop(&mut self) {}
}
impl DropAware {
    pub fn label(&self) -> String {
        self.0.label()
    }
}
impl terrane_callback_witness::DropAware for DropAware {
    fn label(&self) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <DropAware>::label(&*self);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub trait LocalAdjustableProtocol {
    fn clone_box(&self) -> Box<dyn LocalAdjustableProtocol>;
    fn separate_box(&self) -> Box<dyn LocalAdjustableProtocol>;
    fn adjust(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int;
    fn current(&self) -> terrane_int_support::Int;
}
impl Clone for Box<dyn LocalAdjustableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct LocalAdjustable(Box<dyn LocalAdjustableProtocol>);
impl Clone for LocalAdjustable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl LocalAdjustable {
    pub fn adjust(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.0.adjust(delta)
    }
    pub fn current(&self) -> terrane_int_support::Int {
        self.0.current()
    }
}
impl terrane_callback_witness::LocalAdjustable for LocalAdjustable {
    fn adjust(&mut self, delta: i64) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <LocalAdjustable>::adjust(
                &mut *self,
                terrane_int_support::Int::from(i128::from(delta)),
            );
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
    fn current(&self) -> i64 {
        let __terrane_boundary: Result<i64, crate::TerraneForeignError> = (|| {
            let __terrane_value = <LocalAdjustable>::current(&*self);
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
pub use terrane_callback_witness::AdjustableOwner;
pub use terrane_callback_witness::DropToken;
pub use terrane_callback_witness::LocalAdjustableOwner;
pub use terrane_callback_witness::Registrar;
pub fn active_retained_invocations() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::active_retained_invocations()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::active_retained_invocations",
                ),
            )
        }
    }
}
pub fn adjust_value<T: terrane_callback_witness::Adjustable>(
    value: &mut T,
    delta: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let delta = terrane_int_support::coerce::<i64>(&delta)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::adjust_value(
            value,
            delta,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::adjust_value",
                ),
            )
        }
    }
}
pub async fn apply_async(
    value: String,
    callback: std::sync::Arc<
        dyn Fn(
            String,
        ) -> std::pin::Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync,
    >,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: std::string::String| {
            let callback = callback.clone();
            let callback_future = callback(callback_argument_0);
            Box::pin(async move {
                match async {
                    let callback_value = callback_future.await;
                    Ok::<_, crate::TerraneForeignError>(callback_value)
                }
                    .await
                {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error.0),
                }
            })
        }
    };
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::apply_async(value, callback),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_async",
                ),
            )
        }
    }
}
pub async fn apply_async_concurrently(
    value: terrane_int_support::Int,
    callback: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > + Send + Sync,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            let callback = callback.clone();
            let callback_future = callback(
                terrane_int_support::Int::from(i128::from(callback_argument_0)),
            );
            Box::pin(async move {
                match async {
                    let callback_value = callback_future.await;
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(
                        terrane_int_support::coerce::<i64>(&callback_value)
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }
                    .await
                {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error.0),
                }
            })
        }
    };
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::apply_async_concurrently(value, callback),
        )
        .await
    {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_async_concurrently",
                ),
            )
        }
    }
}
pub fn apply_mutable(
    value: terrane_int_support::Int,
    callback: TerraneMutableCallable<
        (terrane_int_support::Int,),
        terrane_int_support::Int,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback
                    .call((
                        terrane_int_support::Int::from(i128::from(callback_argument_0)),
                    ));
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
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::apply_mutable(
            value,
            callback,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_mutable",
                ),
            )
        }
    }
}
pub fn apply_once(
    value: String,
    callback: TerraneConsumingCallable<(String,), String>,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    let callback = {
        let callback = callback;
        move |callback_argument_0: std::string::String| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback.call((callback_argument_0,));
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::apply_once(
            value,
            callback,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_once",
                ),
            )
        }
    }
}
pub fn apply_shared(
    value: terrane_int_support::Int,
    callback: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback(
                    terrane_int_support::Int::from(i128::from(callback_argument_0)),
                );
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
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::apply_shared(
            value,
            callback,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_shared",
                ),
            )
        }
    }
}
pub fn async_entry_cleanup_count() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::async_entry_cleanup_count()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::async_entry_cleanup_count",
                ),
            )
        }
    }
}
pub async fn cancel_async_entry<T: 'static + terrane_callback_witness::AsyncEntry>(
    value: T,
) -> Result<bool, crate::TerraneForeignError> {
    let value = value;
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::cancel_async_entry(value),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::cancel_async_entry",
                ),
            )
        }
    }
}
pub fn consume_drop<T: 'static + terrane_callback_witness::DropAware>(
    value: T,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::consume_drop(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::consume_drop",
                ),
            )
        }
    }
}
pub fn drop_token() -> Result<DropToken, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::drop_token()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::drop_token",
                ),
            )
        }
    }
}
pub async fn invoke_retained(
    callback: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > + Send + Sync,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            let callback = callback.clone();
            let callback_future = callback(
                terrane_int_support::Int::from(i128::from(callback_argument_0)),
            );
            Box::pin(async move {
                match async {
                    let callback_value = callback_future.await;
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(
                        terrane_int_support::coerce::<i64>(&callback_value)
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }
                    .await
                {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error.0),
                }
            })
        }
    };
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::invoke_retained(callback),
        )
        .await
    {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::invoke_retained",
                ),
            )
        }
    }
}
pub async fn pending_value(
    __trn_5f76616c7565: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let __trn_5f76616c7565 = terrane_int_support::coerce::<i64>(&__trn_5f76616c7565)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::pending_value(__trn_5f76616c7565),
        )
        .await
    {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::pending_value",
                ),
            )
        }
    }
}
pub fn record_async_entry_cleanup() -> Result<(), crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::record_async_entry_cleanup()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::record_async_entry_cleanup",
                ),
            )
        }
    }
}
pub fn registrar() -> Result<Registrar, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::registrar()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::registrar",
                ),
            )
        }
    }
}
pub fn retain_adjustable<T: 'static + terrane_callback_witness::Adjustable>(
    value: T,
) -> Result<AdjustableOwner, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::retain_adjustable(
            value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::retain_adjustable",
                ),
            )
        }
    }
}
pub fn retain_boxed_adjustable<
    TerraneBoxed0: terrane_callback_witness::Adjustable + core::marker::Send
        + core::marker::Sync + 'static,
>(value: TerraneBoxed0) -> Result<AdjustableOwner, crate::TerraneForeignError> {
    let value = Box::new(value);
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::retain_boxed_adjustable(
            value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::retain_boxed_adjustable",
                ),
            )
        }
    }
}
pub fn retain_local_adjustable<T: 'static + terrane_callback_witness::LocalAdjustable>(
    value: T,
) -> Result<LocalAdjustableOwner, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::retain_local_adjustable(
            value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::retain_local_adjustable",
                ),
            )
        }
    }
}
pub async fn wait_until_retained_invocation_active() -> Result<
    bool,
    crate::TerraneForeignError,
> {
    match crate::__terrane_dependency_await_unwind(
            terrane_callback_witness::wait_until_retained_invocation_active(),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::wait_until_retained_invocation_active",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/terrane-dispatch-witness.trn
// Namespace: deps/terrane-dispatch-witness
pub fn dispatch(
    label_: String,
    enabled: bool,
    callback: std::sync::Arc<dyn Fn(String, bool) -> String + Send + Sync>,
) -> Result<String, crate::TerraneForeignError> {
    let label_ = label_;
    let enabled = enabled;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: std::string::String, callback_argument_1: bool| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback(callback_argument_0, callback_argument_1);
                Ok(callback_value)
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_dispatch_witness::dispatch(
            label_,
            enabled,
            callback,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-dispatch-witness",
                    "terrane_dispatch_witness::dispatch",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/terrane-render-witness.trn
// Namespace: deps/terrane-render-witness
pub trait RenderableProtocol: Send {
    fn clone_box(&self) -> Box<dyn RenderableProtocol>;
    fn separate_box(&self) -> Box<dyn RenderableProtocol>;
    fn render(&self, label_: String) -> String;
    fn decorated(&self, label_: String) -> String;
    fn borrowed(&self, label_: String) -> String;
    fn parsed(&self, text: String) -> Result<terrane_int_support::Int, TerraneError>;
}
impl Clone for Box<dyn RenderableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Renderable(Box<dyn RenderableProtocol>);
impl Clone for Renderable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Renderable {
    pub fn render(&self, label_: String) -> String {
        self.0.render(label_)
    }
    pub fn decorated(&self, label_: String) -> String {
        self.0.decorated(label_)
    }
    pub fn borrowed(&self, label_: String) -> String {
        self.0.borrowed(label_)
    }
    pub fn parsed(
        &self,
        text: String,
    ) -> Result<terrane_int_support::Int, TerraneError> {
        self.0.parsed(text)
    }
}
impl terrane_render_witness::Renderable for Renderable {
    fn render(&self, label_: String) -> String {
        let __terrane_boundary: Result<String, crate::TerraneForeignError> = (|| {
            let __terrane_value = <Renderable>::render(&*self, label_);
            Ok(__terrane_value)
        })();
        __terrane_boundary.unwrap_or_else(|error| panic!("{}", error.render()))
    }
}
pub fn render_decorated<T: terrane_render_witness::Renderable>(
    value: &T,
    label_: String,
) -> Result<String, crate::TerraneForeignError> {
    let label_ = label_;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_render_witness::render_decorated(
            value,
            label_,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-render-witness",
                    "terrane_render_witness::render_decorated",
                ),
            )
        }
    }
}
pub fn render_impl<TerraneImpl0: terrane_render_witness::Renderable>(
    value: &TerraneImpl0,
    label_: String,
) -> Result<String, crate::TerraneForeignError> {
    let label_ = label_;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_render_witness::render_impl(
            value,
            label_,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-render-witness",
                    "terrane_render_witness::render_impl",
                ),
            )
        }
    }
}
pub fn render_value<T: terrane_render_witness::Renderable>(
    value: &T,
    label_: String,
) -> Result<String, crate::TerraneForeignError> {
    let label_ = label_;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_render_witness::render_value(
            value,
            label_,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-render-witness",
                    "terrane_render_witness::render_value",
                ),
            )
        }
    }
}
