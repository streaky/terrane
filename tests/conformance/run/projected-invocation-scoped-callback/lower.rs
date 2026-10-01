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
    pub static FUNCTIONS: [&str; 25] = [
        "/app::exact-label",
        "/app::exact-nested",
        "/app::exact-typed",
        "/app::exact-recursive",
        "/app::exact-wrapped-binding",
        "/app::exact-ping",
        "/app::exact-pong",
        "/app::exact-inner-label",
        "/app::exact-outer-group",
        "/app::exact-reassign-helper-then-producer",
        "/app::exact-reassign-reverse",
        "/app::exact-reassign-conditional",
        "/app::exact-reassign-alias",
        "/app::exact-bound-wrapped",
        "/app::exact-mixed-branch",
        "/app::exact-helper-aggregate",
        "/app::exact-branch",
        "/app::exact-aggregate",
        "/app::direct",
        "/app::helper",
        "/app::through-helper",
        "/app::nested",
        "/app::branch",
        "/app::aggregate",
        "/app::main",
    ];
    pub static SITES: [Site; 76] = [
        /* terrane-site-row: site 0: /app::exact-label (src/main.trn:8:12-8:33) */
        { Site { function: 0, file: 0, line: 8, column: 12, end_line: 8, end_column: 33 } },
        /* terrane-site-row: site 1: /app::exact-nested (src/main.trn:13:13-13:34) */
        { Site { function: 1, file: 0, line: 13, column: 13, end_line: 13, end_column: 34 } },
        /* terrane-site-row: site 2: /app::exact-nested (src/main.trn:14:14-14:36) */
        { Site { function: 1, file: 0, line: 14, column: 14, end_line: 14, end_column: 36 } },
        /* terrane-site-row: site 3: /app::exact-nested (src/main.trn:15:12-15:41) */
        { Site { function: 1, file: 0, line: 15, column: 12, end_line: 15, end_column: 41 } },
        /* terrane-site-row: site 4: /app::exact-typed (src/main.trn:18:12-18:33) */
        { Site { function: 2, file: 0, line: 18, column: 12, end_line: 18, end_column: 33 } },
        /* terrane-site-row: site 5: /app::exact-recursive (src/main.trn:23:13-23:34) */
        { Site { function: 3, file: 0, line: 23, column: 13, end_line: 23, end_column: 34 } },
        /* terrane-site-row: site 6: /app::exact-wrapped-binding (src/main.trn:27:13-27:34) */
        { Site { function: 4, file: 0, line: 27, column: 13, end_line: 27, end_column: 34 } },
        /* terrane-site-row: site 7: /app::exact-wrapped-binding (src/main.trn:28:12-28:39) */
        { Site { function: 4, file: 0, line: 28, column: 12, end_line: 28, end_column: 39 } },
        /* terrane-site-row: site 8: /app::exact-ping (src/main.trn:33:12-33:33) */
        { Site { function: 5, file: 0, line: 33, column: 12, end_line: 33, end_column: 33 } },
        /* terrane-site-row: site 9: /app::exact-pong (src/main.trn:38:12-38:33) */
        { Site { function: 6, file: 0, line: 38, column: 12, end_line: 38, end_column: 33 } },
        /* terrane-site-row: site 10: /app::exact-inner-label (src/main.trn:41:12-41:33) */
        { Site { function: 7, file: 0, line: 41, column: 12, end_line: 41, end_column: 33 } },
        /* terrane-site-row: site 11: /app::exact-outer-group (src/main.trn:44:12-44:60) */
        { Site { function: 8, file: 0, line: 44, column: 12, end_line: 44, end_column: 60 } },
        /* terrane-site-row: site 12: /app::exact-reassign-helper-then-producer (src/main.trn:52:13-52:34) */
        { Site { function: 9, file: 0, line: 52, column: 13, end_line: 52, end_column: 34 } },
        /* terrane-site-row: site 13: /app::exact-reassign-reverse (src/main.trn:56:13-56:34) */
        { Site { function: 10, file: 0, line: 56, column: 13, end_line: 56, end_column: 34 } },
        /* terrane-site-row: site 14: /app::exact-reassign-conditional (src/main.trn:61:13-61:34) */
        { Site { function: 11, file: 0, line: 61, column: 13, end_line: 61, end_column: 34 } },
        /* terrane-site-row: site 15: /app::exact-reassign-alias (src/main.trn:67:13-67:34) */
        { Site { function: 12, file: 0, line: 67, column: 13, end_line: 67, end_column: 34 } },
        /* terrane-site-row: site 16: /app::exact-bound-wrapped (src/main.trn:74:12-74:39) */
        { Site { function: 13, file: 0, line: 74, column: 12, end_line: 74, end_column: 39 } },
        /* terrane-site-row: site 17: /app::exact-mixed-branch (src/main.trn:80:12-80:33) */
        { Site { function: 14, file: 0, line: 80, column: 12, end_line: 80, end_column: 33 } },
        /* terrane-site-row: site 18: /app::exact-helper-aggregate (src/main.trn:85:23-85:44) */
        { Site { function: 15, file: 0, line: 85, column: 23, end_line: 85, end_column: 44 } },
        /* terrane-site-row: site 19: /app::exact-helper-aggregate (src/main.trn:86:12-86:34) */
        { Site { function: 15, file: 0, line: 86, column: 12, end_line: 86, end_column: 34 } },
        /* terrane-site-row: site 20: /app::exact-branch (src/main.trn:90:24-90:45) */
        { Site { function: 16, file: 0, line: 90, column: 24, end_line: 90, end_column: 45 } },
        /* terrane-site-row: site 21: /app::exact-branch (src/main.trn:91:16-91:50) */
        { Site { function: 16, file: 0, line: 91, column: 16, end_line: 91, end_column: 50 } },
        /* terrane-site-row: site 22: /app::exact-branch (src/main.trn:92:27-92:49) */
        { Site { function: 16, file: 0, line: 92, column: 27, end_line: 92, end_column: 49 } },
        /* terrane-site-row: site 23: /app::exact-branch (src/main.trn:93:12-93:54) */
        { Site { function: 16, file: 0, line: 93, column: 12, end_line: 93, end_column: 54 } },
        /* terrane-site-row: site 24: /app::exact-aggregate (src/main.trn:97:23-97:44) */
        { Site { function: 17, file: 0, line: 97, column: 23, end_line: 97, end_column: 44 } },
        /* terrane-site-row: site 25: /app::exact-aggregate (src/main.trn:98:23-98:44) */
        { Site { function: 17, file: 0, line: 98, column: 23, end_line: 98, end_column: 44 } },
        /* terrane-site-row: site 26: /app::exact-aggregate (src/main.trn:99:12-99:34) */
        { Site { function: 17, file: 0, line: 99, column: 12, end_line: 99, end_column: 34 } },
        /* terrane-site-row: site 27: /app::direct (src/main.trn:102:19-102:28) */
        { Site { function: 18, file: 0, line: 102, column: 19, end_line: 102, end_column: 28 } },
        /* terrane-site-row: site 28: /app::direct (src/main.trn:102:12-102:28) */
        { Site { function: 18, file: 0, line: 102, column: 12, end_line: 102, end_column: 28 } },
        /* terrane-site-row: site 29: /app::helper (src/main.trn:105:12-105:24) */
        { Site { function: 19, file: 0, line: 105, column: 12, end_line: 105, end_column: 24 } },
        /* terrane-site-row: site 30: /app::through-helper (src/main.trn:108:20-108:29) */
        { Site { function: 20, file: 0, line: 108, column: 20, end_line: 108, end_column: 29 } },
        /* terrane-site-row: site 31: /app::nested (src/main.trn:111:34-111:43) */
        { Site { function: 21, file: 0, line: 111, column: 34, end_line: 111, end_column: 43 } },
        /* terrane-site-row: site 32: /app::nested (src/main.trn:111:26-111:43) */
        { Site { function: 21, file: 0, line: 111, column: 26, end_line: 111, end_column: 43 } },
        /* terrane-site-row: site 33: /app::nested (src/main.trn:111:12-111:44) */
        { Site { function: 21, file: 0, line: 111, column: 12, end_line: 111, end_column: 44 } },
        /* terrane-site-row: site 34: /app::branch (src/main.trn:115:37-115:46) */
        { Site { function: 22, file: 0, line: 115, column: 37, end_line: 115, end_column: 46 } },
        /* terrane-site-row: site 35: /app::branch (src/main.trn:115:30-115:46) */
        { Site { function: 22, file: 0, line: 115, column: 30, end_line: 115, end_column: 46 } },
        /* terrane-site-row: site 36: /app::branch (src/main.trn:115:16-115:47) */
        { Site { function: 22, file: 0, line: 115, column: 16, end_line: 115, end_column: 47 } },
        /* terrane-site-row: site 37: /app::branch (src/main.trn:116:34-116:43) */
        { Site { function: 22, file: 0, line: 116, column: 34, end_line: 116, end_column: 43 } },
        /* terrane-site-row: site 38: /app::branch (src/main.trn:116:26-116:43) */
        { Site { function: 22, file: 0, line: 116, column: 26, end_line: 116, end_column: 43 } },
        /* terrane-site-row: site 39: /app::branch (src/main.trn:116:12-116:44) */
        { Site { function: 22, file: 0, line: 116, column: 12, end_line: 116, end_column: 44 } },
        /* terrane-site-row: site 40: /app::aggregate (src/main.trn:120:37-120:52) */
        { Site { function: 23, file: 0, line: 120, column: 37, end_line: 120, end_column: 52 } },
        /* terrane-site-row: site 41: /app::aggregate (src/main.trn:120:23-120:53) */
        { Site { function: 23, file: 0, line: 120, column: 23, end_line: 120, end_column: 53 } },
        /* terrane-site-row: site 42: /app::aggregate (src/main.trn:121:37-121:52) */
        { Site { function: 23, file: 0, line: 121, column: 37, end_line: 121, end_column: 52 } },
        /* terrane-site-row: site 43: /app::aggregate (src/main.trn:121:23-121:53) */
        { Site { function: 23, file: 0, line: 121, column: 23, end_line: 121, end_column: 53 } },
        /* terrane-site-row: site 44: /app::aggregate (src/main.trn:122:37-122:52) */
        { Site { function: 23, file: 0, line: 122, column: 37, end_line: 122, end_column: 52 } },
        /* terrane-site-row: site 45: /app::aggregate (src/main.trn:122:23-122:53) */
        { Site { function: 23, file: 0, line: 122, column: 23, end_line: 122, end_column: 53 } },
        /* terrane-site-row: site 46: /app::aggregate (src/main.trn:123:26-123:39) */
        { Site { function: 23, file: 0, line: 123, column: 26, end_line: 123, end_column: 39 } },
        /* terrane-site-row: site 47: /app::aggregate (src/main.trn:123:12-123:40) */
        { Site { function: 23, file: 0, line: 123, column: 12, end_line: 123, end_column: 40 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:128:13-128:58) */
        { Site { function: 24, file: 0, line: 128, column: 13, end_line: 128, end_column: 58 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:129:13-129:62) */
        { Site { function: 24, file: 0, line: 129, column: 13, end_line: 129, end_column: 62 } },
        /* terrane-site-row: site 50: /app::main (src/main.trn:130:13-130:68) */
        { Site { function: 24, file: 0, line: 130, column: 13, end_line: 130, end_column: 68 } },
        /* terrane-site-row: site 51: /app::main (src/main.trn:131:13-131:42) */
        { Site { function: 24, file: 0, line: 131, column: 13, end_line: 131, end_column: 42 } },
        /* terrane-site-row: site 52: /app::main (src/main.trn:132:13-132:69) */
        { Site { function: 24, file: 0, line: 132, column: 13, end_line: 132, end_column: 69 } },
        /* terrane-site-row: site 53: /app::main (src/main.trn:133:13-133:69) */
        { Site { function: 24, file: 0, line: 133, column: 13, end_line: 133, end_column: 69 } },
        /* terrane-site-row: site 54: /app::main (src/main.trn:134:13-134:72) */
        { Site { function: 24, file: 0, line: 134, column: 13, end_line: 134, end_column: 72 } },
        /* terrane-site-row: site 55: /app::main (src/main.trn:135:13-135:72) */
        { Site { function: 24, file: 0, line: 135, column: 13, end_line: 135, end_column: 72 } },
        /* terrane-site-row: site 56: /app::main (src/main.trn:136:13-136:78) */
        { Site { function: 24, file: 0, line: 136, column: 13, end_line: 136, end_column: 78 } },
        /* terrane-site-row: site 57: /app::main (src/main.trn:137:13-137:67) */
        { Site { function: 24, file: 0, line: 137, column: 13, end_line: 137, end_column: 67 } },
        /* terrane-site-row: site 58: /app::main (src/main.trn:138:13-138:74) */
        { Site { function: 24, file: 0, line: 138, column: 13, end_line: 138, end_column: 74 } },
        /* terrane-site-row: site 59: /app::main (src/main.trn:139:13-139:80) */
        { Site { function: 24, file: 0, line: 139, column: 13, end_line: 139, end_column: 80 } },
        /* terrane-site-row: site 60: /app::main (src/main.trn:140:13-140:76) */
        { Site { function: 24, file: 0, line: 140, column: 13, end_line: 140, end_column: 76 } },
        /* terrane-site-row: site 61: /app::main (src/main.trn:141:13-141:75) */
        { Site { function: 24, file: 0, line: 141, column: 13, end_line: 141, end_column: 75 } },
        /* terrane-site-row: site 62: /app::main (src/main.trn:142:13-142:79) */
        { Site { function: 24, file: 0, line: 142, column: 13, end_line: 142, end_column: 79 } },
        /* terrane-site-row: site 63: /app::main (src/main.trn:143:13-143:92) */
        { Site { function: 24, file: 0, line: 143, column: 13, end_line: 143, end_column: 92 } },
        /* terrane-site-row: site 64: /app::main (src/main.trn:144:13-144:79) */
        { Site { function: 24, file: 0, line: 144, column: 13, end_line: 144, end_column: 79 } },
        /* terrane-site-row: site 65: /app::main (src/main.trn:145:13-145:83) */
        { Site { function: 24, file: 0, line: 145, column: 13, end_line: 145, end_column: 83 } },
        /* terrane-site-row: site 66: /app::main (src/main.trn:146:13-146:77) */
        { Site { function: 24, file: 0, line: 146, column: 13, end_line: 146, end_column: 77 } },
        /* terrane-site-row: site 67: /app::main (src/main.trn:149:23-149:37) */
        { Site { function: 24, file: 0, line: 149, column: 23, end_line: 149, end_column: 37 } },
        /* terrane-site-row: site 68: /app::main (src/main.trn:149:16-149:37) */
        { Site { function: 24, file: 0, line: 149, column: 16, end_line: 149, end_column: 37 } },
        /* terrane-site-row: site 69: /app::main (src/main.trn:150:13-150:60) */
        { Site { function: 24, file: 0, line: 150, column: 13, end_line: 150, end_column: 60 } },
        /* terrane-site-row: site 70: /app::main (src/main.trn:151:13-151:66) */
        { Site { function: 24, file: 0, line: 151, column: 13, end_line: 151, end_column: 66 } },
        /* terrane-site-row: site 71: /app::main (src/main.trn:152:13-152:58) */
        { Site { function: 24, file: 0, line: 152, column: 13, end_line: 152, end_column: 58 } },
        /* terrane-site-row: site 72: /app::main (src/main.trn:153:13-153:58) */
        { Site { function: 24, file: 0, line: 153, column: 13, end_line: 153, end_column: 58 } },
        /* terrane-site-row: site 73: /app::main (src/main.trn:155:13-155:57) */
        { Site { function: 24, file: 0, line: 155, column: 13, end_line: 155, end_column: 57 } },
        /* terrane-site-row: site 74: /app::main (src/main.trn:156:5-156:17) */
        { Site { function: 24, file: 0, line: 156, column: 5, end_line: 156, end_column: 17 } },
        /* terrane-site-row: site 75: /app::main (src/main.trn:157:13-157:61) */
        { Site { function: 24, file: 0, line: 157, column: 13, end_line: 157, end_column: 61 } },
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
// Source: src/main.trn
// Namespace: app
fn exact_label<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    return __terrane_raised(
        borrowed_label(state),
        0 /* terrane-site: src/main.trn:8:12-8:33 */,
    );
}
fn exact<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    return exact_label(state);
}
fn exact_nested<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let first: witness::BorrowedLabel<'_> = __terrane_raised(
        borrowed_label(state),
        1 /* terrane-site: src/main.trn:13:13-13:34 */,
    );
    let second: witness::BorrowedButton<'_> = __terrane_raised(
        borrowed_button(state),
        2 /* terrane-site: src/main.trn:14:14-14:36 */,
    );
    return __terrane_raised(
        borrowed_group(first, second),
        3 /* terrane-site: src/main.trn:15:12-15:41 */,
    );
}
fn exact_typed<'view>(
    state: &'view String,
) -> witness::BorrowedValue<'view, std::string::String> {
    return __terrane_raised(
        borrowed_value(state),
        4 /* terrane-site: src/main.trn:18:12-18:33 */,
    );
}
fn exact_recursive<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(100_i128)
    {
        return exact_recursive(state);
    }
    let value: witness::BorrowedLabel<'_> = __terrane_raised(
        borrowed_label(state),
        5 /* terrane-site: src/main.trn:23:13-23:34 */,
    );
    return value;
}
fn exact_wrapped_binding<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let value: witness::BorrowedLabel<'_> = __terrane_raised(
        borrowed_label(state),
        6 /* terrane-site: src/main.trn:27:13-27:34 */,
    );
    return __terrane_raised(
        borrowed_label_group(value),
        7 /* terrane-site: src/main.trn:28:12-28:39 */,
    );
}
fn exact_ping<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(100_i128)
    {
        return exact_pong(state);
    }
    return __terrane_raised(
        borrowed_label(state),
        8 /* terrane-site: src/main.trn:33:12-33:33 */,
    );
}
fn exact_pong<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(100_i128)
    {
        return exact_ping(state);
    }
    return __terrane_raised(
        borrowed_label(state),
        9 /* terrane-site: src/main.trn:38:12-38:33 */,
    );
}
fn exact_inner_label<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    return __terrane_raised(
        borrowed_label(state),
        10 /* terrane-site: src/main.trn:41:12-41:33 */,
    );
}
fn exact_outer_group<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    return __terrane_raised(
        borrowed_label_group(exact_inner_label(state)),
        11 /* terrane-site: src/main.trn:44:12-44:60 */,
    );
}
fn exact_forwarded_binding<'view>(
    state: &'view String,
) -> witness::BorrowedLabel<'view> {
    let value: witness::BorrowedLabel<'_> = exact_inner_label(state);
    return value;
}
fn exact_reassign_helper_then_producer<'view>(
    state: &'view String,
) -> witness::BorrowedLabel<'view> {
    let mut value: witness::BorrowedLabel<'_> = exact_inner_label(state);
    let _ = &mut value;
    value = __terrane_raised(
        borrowed_label(state),
        12 /* terrane-site: src/main.trn:52:13-52:34 */,
    );
    return value;
}
fn exact_reassign_reverse<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    let mut value: witness::BorrowedLabel<'_> = __terrane_raised(
        borrowed_label(state),
        13 /* terrane-site: src/main.trn:56:13-56:34 */,
    );
    let _ = &mut value;
    value = exact_inner_label(state);
    return value;
}
fn exact_reassign_conditional<'view>(
    state: &'view String,
) -> witness::BorrowedLabel<'view> {
    let mut value: witness::BorrowedLabel<'_> = __terrane_raised(
        borrowed_label(state),
        14 /* terrane-site: src/main.trn:61:13-61:34 */,
    );
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(0_i128)
    {
        value = exact_inner_label(state);
    }
    return value;
}
fn exact_reassign_alias<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    let value: witness::BorrowedLabel<'_> = __terrane_raised(
        borrowed_label(state),
        15 /* terrane-site: src/main.trn:67:13-67:34 */,
    );
    let mut alias: witness::BorrowedLabel<'_> = value;
    let _ = &mut alias;
    alias = exact_inner_label(state);
    return alias;
}
fn exact_bound_wrapped<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let value: witness::BorrowedLabel<'_> = exact_inner_label(state);
    return __terrane_raised(
        borrowed_label_group(value),
        16 /* terrane-site: src/main.trn:74:12-74:39 */,
    );
}
fn exact_mixed_branch<'view>(state: &'view String) -> witness::BorrowedLabel<'view> {
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(1_i128)
    {
        let value: witness::BorrowedLabel<'_> = exact_inner_label(state);
        return value;
    }
    return __terrane_raised(
        borrowed_label(state),
        17 /* terrane-site: src/main.trn:80:12-80:33 */,
    );
}
fn exact_helper_aggregate<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let mut children: terrane_collection_support::List<_> = terrane_collection_support::List::<
        _,
    >::new(vec![]);
    children.push_unique(exact_inner_label(state));
    children
        .push_unique(
            __terrane_raised(
                borrowed_label(state),
                18 /* terrane-site: src/main.trn:85:23-85:44 */,
            ),
        );
    return __terrane_raised(
        borrowed_row(children),
        19 /* terrane-site: src/main.trn:86:12-86:34 */,
    );
}
fn exact_branch<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(1_i128)
    {
        let intermediate: witness::BorrowedLabel<'_> = __terrane_raised(
            borrowed_label(state),
            20 /* terrane-site: src/main.trn:90:24-90:45 */,
        );
        return __terrane_raised(
            borrowed_label_group(intermediate),
            21 /* terrane-site: src/main.trn:91:16-91:50 */,
        );
    }
    let button_intermediate: witness::BorrowedButton<'_> = __terrane_raised(
        borrowed_button(state),
        22 /* terrane-site: src/main.trn:92:27-92:49 */,
    );
    return __terrane_raised(
        borrowed_button_group(button_intermediate),
        23 /* terrane-site: src/main.trn:93:12-93:54 */,
    );
}
fn exact_aggregate<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let mut children: terrane_collection_support::List<_> = terrane_collection_support::List::<
        _,
    >::new(vec![]);
    children
        .push_unique(
            __terrane_raised(
                borrowed_label(state),
                24 /* terrane-site: src/main.trn:97:23-97:44 */,
            ),
        );
    children
        .push_unique(
            __terrane_raised(
                borrowed_label(state),
                25 /* terrane-site: src/main.trn:98:23-98:44 */,
            ),
        );
    return __terrane_raised(
        borrowed_row(children),
        26 /* terrane-site: src/main.trn:99:12-99:34 */,
    );
}
fn direct(state: &i64) -> Label {
    return __terrane_raised(
        label(
            terrane_int_support::Int::from(
                __terrane_raised(
                    terrane_int_support::fixed_addition(state.clone(), 1),
                    27 /* terrane-site: src/main.trn:102:19-102:28 */,
                ) as i128,
            ),
        ),
        28 /* terrane-site: src/main.trn:102:12-102:28 */,
    );
}
fn helper(value: i64) -> Label {
    return __terrane_raised(
        label(terrane_int_support::Int::from(value as i128)),
        29 /* terrane-site: src/main.trn:105:12-105:24 */,
    );
}
fn through_helper(state: &i64) -> Label {
    return helper(
        __terrane_raised(
            terrane_int_support::fixed_addition(state.clone(), 1),
            30 /* terrane-site: src/main.trn:108:20-108:29 */,
        ),
    );
}
fn nested(state: &i64) -> Widget {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::into_widget(
                __terrane_raised(
                    button(
                        terrane_int_support::Int::from(
                            __terrane_raised(
                                terrane_int_support::fixed_addition(state.clone(), 1),
                                31 /* terrane-site: src/main.trn:111:34-111:43 */,
                            ) as i128,
                        ),
                    ),
                    32 /* terrane-site: src/main.trn:111:26-111:43 */,
                ),
            )),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::into_widget",
                    ),
                )
            }
        },
        33 /* terrane-site: src/main.trn:111:12-111:44 */,
    );
}
fn branch(state: &i64) -> Widget {
    if state.clone() > 0 {
        return __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::into_widget(
                    __terrane_raised(
                        label(
                            terrane_int_support::Int::from(
                                __terrane_raised(
                                    terrane_int_support::fixed_addition(state.clone(), 0),
                                    34 /* terrane-site: src/main.trn:115:37-115:46 */,
                                ) as i128,
                            ),
                        ),
                        35 /* terrane-site: src/main.trn:115:30-115:46 */,
                    ),
                )),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::into_widget",
                        ),
                    )
                }
            },
            36 /* terrane-site: src/main.trn:115:16-115:47 */,
        );
    }
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::into_widget(
                __terrane_raised(
                    button(
                        terrane_int_support::Int::from(
                            __terrane_raised(
                                terrane_int_support::fixed_addition(state.clone(), 7),
                                37 /* terrane-site: src/main.trn:116:34-116:43 */,
                            ) as i128,
                        ),
                    ),
                    38 /* terrane-site: src/main.trn:116:26-116:43 */,
                ),
            )),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::into_widget",
                    ),
                )
            }
        },
        39 /* terrane-site: src/main.trn:116:12-116:44 */,
    );
}
fn aggregate(state: &i64) -> Widget {
    let _ = &state;
    let mut children: terrane_collection_support::List<Widget> = terrane_collection_support::List::<
        Widget,
    >::new(vec![]);
    children
        .append(
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_raised(
                            traced_label(terrane_int_support::Int::from(1_i128)),
                            40 /* terrane-site: src/main.trn:120:37-120:52 */,
                        ),
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::into_widget",
                            ),
                        )
                    }
                },
                41 /* terrane-site: src/main.trn:120:23-120:53 */,
            ),
        );
    children
        .append(
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_raised(
                            traced_label(terrane_int_support::Int::from(2_i128)),
                            42 /* terrane-site: src/main.trn:121:37-121:52 */,
                        ),
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::into_widget",
                            ),
                        )
                    }
                },
                43 /* terrane-site: src/main.trn:121:23-121:53 */,
            ),
        );
    children
        .append(
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_raised(
                            traced_label(terrane_int_support::Int::from(3_i128)),
                            44 /* terrane-site: src/main.trn:122:37-122:52 */,
                        ),
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::into_widget",
                            ),
                        )
                    }
                },
                45 /* terrane-site: src/main.trn:122:23-122:53 */,
            ),
        );
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::into_widget(
                __terrane_raised(
                    row(children),
                    46 /* terrane-site: src/main.trn:123:26-123:39 */,
                ),
            )),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::into_widget",
                    ),
                )
            }
        },
        47 /* terrane-site: src/main.trn:123:12-123:40 */,
    );
}
fn main() {
    let state: i64 = 40;
    let exact_state: String = String::from("40");
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(direct).clone(); move | callback_argument_0 : &'_ i64 | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(&* callback_argument_0); Ok(callback_value) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_scoped::<i64, String, bool>")) }, 48 /* terrane-site: src/main.trn:128:13-128:58 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 49 /* terrane-site: src/main.trn:129:13-129:62 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_typed))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 50 /* terrane-site: src/main.trn:130:13-130:68 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_default:: < i64, String, bool > (&state, match | | -> Result < _,
        crate ::TerraneForeignError > { Ok({ let callback = std::sync::Arc::new(direct)
        .clone(); move | callback_argument_0 : &'_ i64 | { match | | -> Result < _, crate
        ::TerraneForeignError > { let callback_value = callback(&* callback_argument_0);
        Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_default::<i64, String, bool>")) }, 51 /* terrane-site: src/main.trn:131:13-131:42 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_nested))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 52 /* terrane-site: src/main.trn:132:13-132:69 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_branch))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 53 /* terrane-site: src/main.trn:133:13-133:69 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_aggregate)))
        { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 54 /* terrane-site: src/main.trn:134:13-134:72 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_recursive)))
        { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 55 /* terrane-site: src/main.trn:135:13-135:72 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_wrapped_binding))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 56 /* terrane-site: src/main.trn:136:13-136:78 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, exact_ping))) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 57 /* terrane-site: src/main.trn:137:13-137:67 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_outer_group))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 58 /* terrane-site: src/main.trn:138:13-138:74 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_forwarded_binding))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 59 /* terrane-site: src/main.trn:139:13-139:80 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_bound_wrapped))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 60 /* terrane-site: src/main.trn:140:13-140:76 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_mixed_branch))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 61 /* terrane-site: src/main.trn:141:13-141:75 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_helper_aggregate))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 62 /* terrane-site: src/main.trn:142:13-142:79 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_reassign_helper_then_producer))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 63 /* terrane-site: src/main.trn:143:13-143:92 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_reassign_reverse))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 64 /* terrane-site: src/main.trn:144:13-144:79 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_reassign_conditional))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 65 /* terrane-site: src/main.trn:145:13-145:83 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true,
        exact_reassign_alias))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 66 /* terrane-site: src/main.trn:146:13-146:77 */))
    );
    let offset: i64 = 2;
    let captured: std::sync::Arc<dyn Fn(&i64) -> Label + Send + Sync> = {
        let offset = offset.clone();
        std::sync::Arc::new(move |state: &i64| -> Label {
            return __terrane_raised(
                label(
                    terrane_int_support::Int::from(
                        __terrane_raised(
                            terrane_int_support::fixed_addition(state.clone(), offset),
                            67 /* terrane-site: src/main.trn:149:23-149:37 */,
                        ) as i128,
                    ),
                ),
                68 /* terrane-site: src/main.trn:149:16-149:37 */,
            );
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = captured.clone().clone(); move | callback_argument_0 : &'_ i64 | { match | | ->
        Result < _, crate ::TerraneForeignError > { let callback_value = callback(&*
        callback_argument_0); Ok(callback_value) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error.0) } } }) } () { Ok(value) => value, Err(error) =>
        std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_scoped::<i64, String, bool>")) }, 69 /* terrane-site: src/main.trn:150:13-150:60 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(through_helper).clone(); move | callback_argument_0 : &'_
        i64 | { match | | -> Result < _, crate ::TerraneForeignError > { let
        callback_value = callback(&* callback_argument_0); Ok(callback_value) } () {
        Ok(value) => value, Err(error) => std::panic::panic_any(error.0) } } }) } () {
        Ok(value) => value, Err(error) => std::panic::panic_any(error) }))) { Ok(value)
        => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
        Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_scoped::<i64, String, bool>")) }, 70 /* terrane-site: src/main.trn:151:13-151:66 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(nested).clone(); move | callback_argument_0 : &'_ i64 | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(&* callback_argument_0); Ok(callback_value) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_scoped::<i64, String, bool>")) }, 71 /* terrane-site: src/main.trn:152:13-152:58 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(branch).clone(); move | callback_argument_0 : &'_ i64 | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(&* callback_argument_0); Ok(callback_value) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_scoped::<i64, String, bool>")) }, 72 /* terrane-site: src/main.trn:153:13-153:58 */))
    );
    let zero: i64 = 0;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&zero, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(branch).clone(); move | callback_argument_0 : &'_ i64 | {
        match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(&* callback_argument_0); Ok(callback_value) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_scoped::<i64, String, bool>")) }, 73 /* terrane-site: src/main.trn:155:13-155:57 */))
    );
    __terrane_raised(reset_trace(), 74 /* terrane-site: src/main.trn:156:5-156:17 */);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        witness::render_scoped:: < i64, String, bool > (&state, String::from("message"),
        true, match | | -> Result < _, crate ::TerraneForeignError > { Ok({ let callback
        = std::sync::Arc::new(aggregate).clone(); move | callback_argument_0 : &'_ i64 |
        { match | | -> Result < _, crate ::TerraneForeignError > { let callback_value =
        callback(&* callback_argument_0); Ok(callback_value) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error.0) } } }) } () { Ok(value) => value,
        Err(error) => std::panic::panic_any(error) }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_scoped::<i64, String, bool>")) }, 75 /* terrane-site: src/main.trn:157:13-157:61 */))
    );
}
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub use witness::Button;
pub use witness::Label;
pub use witness::Row;
pub use witness::Widget;
pub fn borrowed_button<'view>(
    value: &'view String,
) -> Result<witness::BorrowedButton<'view>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::borrowed_button(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::borrowed_button",
                ),
            )
        }
    }
}
pub fn borrowed_button_group<'view>(
    button_value: witness::BorrowedButton<'view>,
) -> Result<witness::BorrowedGroup<'view>, crate::TerraneForeignError> {
    let button_value = button_value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::borrowed_button_group(button_value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::borrowed_button_group",
                ),
            )
        }
    }
}
pub fn borrowed_group<'view>(
    label_value: witness::BorrowedLabel<'view>,
    button_value: witness::BorrowedButton<'view>,
) -> Result<witness::BorrowedGroup<'view>, crate::TerraneForeignError> {
    let label_value = label_value;
    let button_value = button_value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::borrowed_group(
            label_value,
            button_value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::borrowed_group",
                ),
            )
        }
    }
}
pub fn borrowed_label<'view>(
    value: &'view String,
) -> Result<witness::BorrowedLabel<'view>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::borrowed_label(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::borrowed_label",
                ),
            )
        }
    }
}
pub fn borrowed_label_group<'view>(
    label_value: witness::BorrowedLabel<'view>,
) -> Result<witness::BorrowedGroup<'view>, crate::TerraneForeignError> {
    let label_value = label_value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::borrowed_label_group(label_value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::borrowed_label_group",
                ),
            )
        }
    }
}
pub fn borrowed_row<'view>(
    children: terrane_collection_support::List<witness::BorrowedLabel<'view>>,
) -> Result<witness::BorrowedGroup<'view>, crate::TerraneForeignError> {
    let children = children.into_unique_vec();
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::borrowed_row(children)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::borrowed_row",
                ),
            )
        }
    }
}
pub fn borrowed_value<'view>(
    value: &'view String,
) -> Result<
    witness::BorrowedValue<'view, std::string::String>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::borrowed_value(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::borrowed_value",
                ),
            )
        }
    }
}
pub fn button(
    value: terrane_int_support::Int,
) -> Result<Button, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::button(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::button"))
        }
    }
}
pub fn label(
    value: terrane_int_support::Int,
) -> Result<Label, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::label(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::label"))
        }
    }
}
pub fn reset_trace() -> Result<(), crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::reset_trace()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::reset_trace",
                ),
            )
        }
    }
}
pub fn row(
    children: terrane_collection_support::List<Widget>,
) -> Result<Row, crate::TerraneForeignError> {
    let children = children.into_vec();
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::row(children)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::row"))
        }
    }
}
pub fn traced_label(
    value: terrane_int_support::Int,
) -> Result<Label, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::traced_label(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::traced_label",
                ),
            )
        }
    }
}
