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
    pub static FUNCTIONS: [&str; 12] = [
        "/callbacks::mark-sensitive",
        "/callbacks::clear-sensitive",
        "/callbacks::read-sensitive",
        "/callbacks::clear-sensitive-optional",
        "/app::entry-value",
        "/app::mapped-entry",
        "/app::request-view",
        "/app::root-value",
        "/app::default-http-method",
        "/app::default-http-status",
        "/app::default-http-peer",
        "/app::main",
    ];
    pub static SITES: [Site; 70] = [
        /* terrane-site-row: site 0: /callbacks::mark-sensitive (callbacks/header.trn:6:5-6:31) */
        { Site { function: 0, file: 0, line: 6, column: 5, end_line: 6, end_column: 31 } },
        /* terrane-site-row: site 1: /callbacks::clear-sensitive (callbacks/header.trn:10:5-10:32) */
        { Site { function: 1, file: 0, line: 10, column: 5, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 2: /callbacks::read-sensitive (callbacks/header.trn:14:12-14:32) */
        { Site { function: 2, file: 0, line: 14, column: 12, end_line: 14, end_column: 32 } },
        /* terrane-site-row: site 3: /callbacks::clear-sensitive-optional (callbacks/header.trn:17:5-17:32) */
        { Site { function: 3, file: 0, line: 17, column: 5, end_line: 17, end_column: 32 } },
        /* terrane-site-row: site 4: /app::entry-value (src/main.trn:16:12-16:24) */
        { Site { function: 4, file: 1, line: 16, column: 12, end_line: 16, end_column: 24 } },
        /* terrane-site-row: site 5: /app::mapped-entry (src/main.trn:19:12-21:26) */
        { Site { function: 5, file: 1, line: 19, column: 12, end_line: 21, end_column: 26 } },
        /* terrane-site-row: site 6: /app::request-view (src/main.trn:24:12-24:53) */
        { Site { function: 6, file: 1, line: 24, column: 12, end_line: 24, end_column: 53 } },
        /* terrane-site-row: site 7: /app::root-value (src/main.trn:33:13-33:32) */
        { Site { function: 7, file: 1, line: 33, column: 13, end_line: 33, end_column: 32 } },
        /* terrane-site-row: site 8: /app::default-http-method (src/main.trn:46:13-46:38) */
        { Site { function: 8, file: 1, line: 46, column: 13, end_line: 46, end_column: 38 } },
        /* terrane-site-row: site 9: /app::default-http-status (src/main.trn:49:14-49:30) */
        { Site { function: 9, file: 1, line: 49, column: 14, end_line: 49, end_column: 30 } },
        /* terrane-site-row: site 10: /app::default-http-status (src/main.trn:49:13-49:39) */
        { Site { function: 9, file: 1, line: 49, column: 13, end_line: 49, end_column: 39 } },
        /* terrane-site-row: site 11: /app::default-http-peer (src/main.trn:58:12-60:27) */
        { Site { function: 10, file: 1, line: 58, column: 12, end_line: 60, end_column: 27 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:63:15-63:31) */
        { Site { function: 11, file: 1, line: 63, column: 15, end_line: 63, end_column: 31 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:65:13-65:50) */
        { Site { function: 11, file: 1, line: 65, column: 13, end_line: 65, end_column: 50 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:66:13-66:52) */
        { Site { function: 11, file: 1, line: 66, column: 13, end_line: 66, end_column: 52 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:67:13-67:39) */
        { Site { function: 11, file: 1, line: 67, column: 13, end_line: 67, end_column: 39 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:68:13-68:25) */
        { Site { function: 11, file: 1, line: 68, column: 13, end_line: 68, end_column: 25 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:69:27-69:63) */
        { Site { function: 11, file: 1, line: 69, column: 27, end_line: 69, end_column: 63 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:71:17-71:32) */
        { Site { function: 11, file: 1, line: 71, column: 17, end_line: 71, end_column: 32 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:72:13-72:25) */
        { Site { function: 11, file: 1, line: 72, column: 13, end_line: 72, end_column: 25 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:73:23-73:79) */
        { Site { function: 11, file: 1, line: 73, column: 23, end_line: 73, end_column: 79 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:76:13-76:25) */
        { Site { function: 11, file: 1, line: 76, column: 13, end_line: 76, end_column: 25 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:80:13-80:25) */
        { Site { function: 11, file: 1, line: 80, column: 13, end_line: 80, end_column: 25 } },
        /* terrane-site-row: site 23: /app::main (src/main.trn:81:25-81:70) */
        { Site { function: 11, file: 1, line: 81, column: 25, end_line: 81, end_column: 70 } },
        /* terrane-site-row: site 24: /app::main (src/main.trn:84:12-84:22) */
        { Site { function: 11, file: 1, line: 84, column: 12, end_line: 84, end_column: 22 } },
        /* terrane-site-row: site 25: /app::main (src/main.trn:85:5-85:36) */
        { Site { function: 11, file: 1, line: 85, column: 5, end_line: 85, end_column: 36 } },
        /* terrane-site-row: site 26: /app::main (src/main.trn:86:5-86:37) */
        { Site { function: 11, file: 1, line: 86, column: 5, end_line: 86, end_column: 37 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:87:13-87:24) */
        { Site { function: 11, file: 1, line: 87, column: 13, end_line: 87, end_column: 24 } },
        /* terrane-site-row: site 28: /app::main (src/main.trn:88:16-88:32) */
        { Site { function: 11, file: 1, line: 88, column: 16, end_line: 88, end_column: 32 } },
        /* terrane-site-row: site 29: /app::main (src/main.trn:88:13-88:61) */
        { Site { function: 11, file: 1, line: 88, column: 13, end_line: 88, end_column: 61 } },
        /* terrane-site-row: site 30: /app::main (src/main.trn:89:13-89:24) */
        { Site { function: 11, file: 1, line: 89, column: 13, end_line: 89, end_column: 24 } },
        /* terrane-site-row: site 31: /app::main (src/main.trn:90:13-90:42) */
        { Site { function: 11, file: 1, line: 90, column: 13, end_line: 90, end_column: 42 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:91:13-91:25) */
        { Site { function: 11, file: 1, line: 91, column: 13, end_line: 91, end_column: 25 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:93:15-93:30) */
        { Site { function: 11, file: 1, line: 93, column: 15, end_line: 93, end_column: 30 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:94:25-94:75) */
        { Site { function: 11, file: 1, line: 94, column: 25, end_line: 94, end_column: 75 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:96:12-96:45) */
        { Site { function: 11, file: 1, line: 96, column: 12, end_line: 96, end_column: 45 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:97:13-97:45) */
        { Site { function: 11, file: 1, line: 97, column: 13, end_line: 97, end_column: 45 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:98:5-98:32) */
        { Site { function: 11, file: 1, line: 98, column: 5, end_line: 98, end_column: 32 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:99:24-99:70) */
        { Site { function: 11, file: 1, line: 99, column: 24, end_line: 99, end_column: 70 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:102:24-102:74) */
        { Site { function: 11, file: 1, line: 102, column: 24, end_line: 102, end_column: 74 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:105:33-105:84) */
        { Site { function: 11, file: 1, line: 105, column: 33, end_line: 105, end_column: 84 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:109:9-109:35) */
        { Site { function: 11, file: 1, line: 109, column: 9, end_line: 109, end_column: 35 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:111:34-111:86) */
        { Site { function: 11, file: 1, line: 111, column: 34, end_line: 111, end_column: 86 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:114:36-114:82) */
        { Site { function: 11, file: 1, line: 114, column: 36, end_line: 114, end_column: 82 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:117:33-117:71) */
        { Site { function: 11, file: 1, line: 117, column: 33, end_line: 117, end_column: 71 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:118:25-118:76) */
        { Site { function: 11, file: 1, line: 118, column: 25, end_line: 118, end_column: 76 } },
        /* terrane-site-row: site 46: /app::main (src/main.trn:121:23-121:69) */
        { Site { function: 11, file: 1, line: 121, column: 23, end_line: 121, end_column: 69 } },
        /* terrane-site-row: site 47: /app::main (src/main.trn:123:16-123:36) */
        { Site { function: 11, file: 1, line: 123, column: 16, end_line: 123, end_column: 36 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:124:39-124:85) */
        { Site { function: 11, file: 1, line: 124, column: 39, end_line: 124, end_column: 85 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:127:30-127:80) */
        { Site { function: 11, file: 1, line: 127, column: 30, end_line: 127, end_column: 80 } },
        /* terrane-site-row: site 50: /app::main (src/main.trn:133:9-133:38) */
        { Site { function: 11, file: 1, line: 133, column: 9, end_line: 133, end_column: 38 } },
        /* terrane-site-row: site 51: /app::main (src/main.trn:134:17-134:39) */
        { Site { function: 11, file: 1, line: 134, column: 17, end_line: 134, end_column: 39 } },
        /* terrane-site-row: site 52: /app::main (src/main.trn:135:5-135:55) */
        { Site { function: 11, file: 1, line: 135, column: 5, end_line: 135, end_column: 55 } },
        /* terrane-site-row: site 53: /app::main (src/main.trn:136:23-136:88) */
        { Site { function: 11, file: 1, line: 136, column: 23, end_line: 136, end_column: 88 } },
        /* terrane-site-row: site 54: /app::main (src/main.trn:138:23-138:69) */
        { Site { function: 11, file: 1, line: 138, column: 23, end_line: 138, end_column: 69 } },
        /* terrane-site-row: site 55: /app::main (src/main.trn:142:14-142:45) */
        { Site { function: 11, file: 1, line: 142, column: 14, end_line: 142, end_column: 45 } },
        /* terrane-site-row: site 56: /app::main (src/main.trn:144:13-144:33) */
        { Site { function: 11, file: 1, line: 144, column: 13, end_line: 144, end_column: 33 } },
        /* terrane-site-row: site 57: /app::main (src/main.trn:148:15-148:47) */
        { Site { function: 11, file: 1, line: 148, column: 15, end_line: 148, end_column: 47 } },
        /* terrane-site-row: site 58: /app::main (src/main.trn:150:13-150:34) */
        { Site { function: 11, file: 1, line: 150, column: 13, end_line: 150, end_column: 34 } },
        /* terrane-site-row: site 59: /app::main (src/main.trn:152:20-152:50) */
        { Site { function: 11, file: 1, line: 152, column: 20, end_line: 152, end_column: 50 } },
        /* terrane-site-row: site 60: /app::main (src/main.trn:154:13-154:39) */
        { Site { function: 11, file: 1, line: 154, column: 13, end_line: 154, end_column: 39 } },
        /* terrane-site-row: site 61: /app::main (src/main.trn:159:5-159:37) */
        { Site { function: 11, file: 1, line: 159, column: 5, end_line: 159, end_column: 37 } },
        /* terrane-site-row: site 62: /app::main (src/main.trn:160:5-160:34) */
        { Site { function: 11, file: 1, line: 160, column: 5, end_line: 160, end_column: 34 } },
        /* terrane-site-row: site 63: /app::main (src/main.trn:163:12-163:22) */
        { Site { function: 11, file: 1, line: 163, column: 12, end_line: 163, end_column: 22 } },
        /* terrane-site-row: site 64: /app::main (src/main.trn:165:13-165:25) */
        { Site { function: 11, file: 1, line: 165, column: 13, end_line: 165, end_column: 25 } },
        /* terrane-site-row: site 65: /app::main (src/main.trn:174:49-174:65) */
        { Site { function: 11, file: 1, line: 174, column: 49, end_line: 174, end_column: 65 } },
        /* terrane-site-row: site 66: /app::main (src/main.trn:174:23-174:66) */
        { Site { function: 11, file: 1, line: 174, column: 23, end_line: 174, end_column: 66 } },
        /* terrane-site-row: site 67: /app::main (src/main.trn:176:51-176:67) */
        { Site { function: 11, file: 1, line: 176, column: 51, end_line: 176, end_column: 67 } },
        /* terrane-site-row: site 68: /app::main (src/main.trn:176:24-176:68) */
        { Site { function: 11, file: 1, line: 176, column: 24, end_line: 176, end_column: 68 } },
        /* terrane-site-row: site 69: /app::main (src/main.trn:177:13-177:50) */
        { Site { function: 11, file: 1, line: 177, column: 13, end_line: 177, end_column: 50 } },
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
// Source: <terrane>/projected/deps/axum/body.trn
// Namespace: deps/axum/body
pub fn terrane_static_trn_426f6479_empty() -> Result<Body, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| axum_core::body::Body::empty()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "axum",
                    "axum_core::body::Body",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/axum-core/body.trn
// Namespace: deps/axum-core/body
// Source: <terrane>/projected/deps/axum/extract.trn
// Namespace: deps/axum/extract
// Source: <terrane>/projected/deps/axum/http.trn
// Namespace: deps/axum/http
// Source: <terrane>/projected/deps/http.trn
// Namespace: deps/http
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
    s: &str,
) -> Result<HeaderName, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| <http::HeaderName as std::str::FromStr>::from_str(
            s,
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
    src: &str,
) -> Result<HeaderValue, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| http::HeaderValue::from_str(src)),
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
// Source: <terrane>/projected/deps/axum-core/extract.trn
// Namespace: deps/axum-core/extract
pub use axum_core::body::Body;
pub use witness::Canvas;
pub use axum::extract::ConnectInfo;
pub use witness::Entry;
pub use witness::Header;
pub use http::HeaderMap;
pub use http::HeaderName;
pub use http::HeaderValue;
pub use witness::Host;
pub use http::Method;
pub use witness::RootOnly;
pub use http::StatusCode;
pub use axum_core::extract::Request as TerraneNsDepsAxumCoreExtractRequestIdentity2F646570732F6178756D2D636F72652F657874726163740052657175657374;
pub use axum_core::response::Response as TerraneNsDepsAxumCoreResponseResponseIdentity2F646570732F6178756D2D636F72652F726573706F6E736500526573706F6E7365;
pub use http::Extensions as TerraneNsDepsHttpExtensionsIdentity2F646570732F6874747000457874656E73696F6E73;
pub use http::Request as TerraneNsDepsHttpRequestIdentity2F646570732F687474700052657175657374;
pub use http::Response as TerraneNsDepsHttpResponseIdentity2F646570732F6874747000526573706F6E7365;
pub use witness::Extensions as TerraneNsDepsWitnessExtensionsIdentity2F646570732F7769746E65737300457874656E73696F6E73;
pub use witness::Request as TerraneNsDepsWitnessRequestIdentity2F646570732F7769746E6573730052657175657374;
pub use witness::RootAlias as TerraneNsDepsWitnessRootAliasIdentity2F646570732F7769746E65737300526F6F74416C696173;
pub type View<'view> = witness::View<'view>;
pub fn terrane_static_trn_54657272616e654e73446570734178756d436f726545787472616374526571756573744964656e746974793246363436353730373332463631373837353644324436333646373236353246363537383734373236313633373430303532363537313735363537333734_new<
    T,
>(body: T) -> Result<axum_core::extract::Request<T>, crate::TerraneForeignError> {
    let body = body;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| axum_core::extract::Request::<T>::new(body)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "axum-core",
                    "axum_core::extract::Request",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/axum-core/response.trn
// Namespace: deps/axum-core/response
pub fn terrane_static_trn_54657272616e654e73446570734178756d436f7265526573706f6e7365526573706f6e73654964656e74697479324636343635373037333246363137383735364432443633364637323635324637323635373337303646364537333635303035323635373337303646364537333635_new<
    T,
>(body: T) -> Result<axum_core::response::Response<T>, crate::TerraneForeignError> {
    let body = body;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| axum_core::response::Response::<T>::new(body)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "axum-core",
                    "axum_core::response::Response",
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
pub fn terrane_static_trn_54657272616e654e73446570735769746e657373526571756573744964656e74697479324636343635373037333246373736393734364536353733373330303532363537313735363537333734_new(
    value: terrane_int_support::Int,
) -> Result<
    TerraneNsDepsWitnessRequestIdentity2F646570732F7769746E6573730052657175657374,
    crate::TerraneForeignError,
> {
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
    entry: &'view Entry,
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
    request: &TerraneNsDepsWitnessRequestIdentity2F646570732F7769746E6573730052657175657374,
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
pub fn root_only() -> Result<
    witness::RootOnly<witness::Entry>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::root_only()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::root_only",
                ),
            )
        }
    }
}
// Source: core/networking_native.trn
// Namespace: core/networking/native
pub type NativeTcpListener = tokio::net::TcpListener;
pub type NativeTcpStream = tokio::net::TcpStream;
pub type NativeSocketAddress = std::net::SocketAddr;
pub type HostListenerTransfer = std::io::Result<tokio::net::TcpListener>;
pub type HostStreamTransfer = std::io::Result<tokio::net::TcpStream>;
