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
    pub static FUNCTIONS: [&str; 11] = [
        "/app::exact-label",
        "/app::exact-nested",
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
    pub static SITES: [Site; 50] = [
        /* terrane-site-row: site 0: /app::exact-label (src/main.trn:8:12-8:33) */
        { Site { function: 0, file: 0, line: 8, column: 12, end_line: 8, end_column: 33 } },
        /* terrane-site-row: site 1: /app::exact-nested (src/main.trn:13:13-13:34) */
        { Site { function: 1, file: 0, line: 13, column: 13, end_line: 13, end_column: 34 } },
        /* terrane-site-row: site 2: /app::exact-nested (src/main.trn:14:14-14:36) */
        { Site { function: 1, file: 0, line: 14, column: 14, end_line: 14, end_column: 36 } },
        /* terrane-site-row: site 3: /app::exact-nested (src/main.trn:15:12-15:41) */
        { Site { function: 1, file: 0, line: 15, column: 12, end_line: 15, end_column: 41 } },
        /* terrane-site-row: site 4: /app::exact-branch (src/main.trn:19:24-19:45) */
        { Site { function: 2, file: 0, line: 19, column: 24, end_line: 19, end_column: 45 } },
        /* terrane-site-row: site 5: /app::exact-branch (src/main.trn:20:16-20:50) */
        { Site { function: 2, file: 0, line: 20, column: 16, end_line: 20, end_column: 50 } },
        /* terrane-site-row: site 6: /app::exact-branch (src/main.trn:21:27-21:49) */
        { Site { function: 2, file: 0, line: 21, column: 27, end_line: 21, end_column: 49 } },
        /* terrane-site-row: site 7: /app::exact-branch (src/main.trn:22:12-22:54) */
        { Site { function: 2, file: 0, line: 22, column: 12, end_line: 22, end_column: 54 } },
        /* terrane-site-row: site 8: /app::exact-aggregate (src/main.trn:26:23-26:44) */
        { Site { function: 3, file: 0, line: 26, column: 23, end_line: 26, end_column: 44 } },
        /* terrane-site-row: site 9: /app::exact-aggregate (src/main.trn:27:23-27:44) */
        { Site { function: 3, file: 0, line: 27, column: 23, end_line: 27, end_column: 44 } },
        /* terrane-site-row: site 10: /app::exact-aggregate (src/main.trn:28:12-28:34) */
        { Site { function: 3, file: 0, line: 28, column: 12, end_line: 28, end_column: 34 } },
        /* terrane-site-row: site 11: /app::direct (src/main.trn:32:19-32:28) */
        { Site { function: 4, file: 0, line: 32, column: 19, end_line: 32, end_column: 28 } },
        /* terrane-site-row: site 12: /app::direct (src/main.trn:32:12-32:28) */
        { Site { function: 4, file: 0, line: 32, column: 12, end_line: 32, end_column: 28 } },
        /* terrane-site-row: site 13: /app::helper (src/main.trn:35:12-35:24) */
        { Site { function: 5, file: 0, line: 35, column: 12, end_line: 35, end_column: 24 } },
        /* terrane-site-row: site 14: /app::through-helper (src/main.trn:38:20-38:29) */
        { Site { function: 6, file: 0, line: 38, column: 20, end_line: 38, end_column: 29 } },
        /* terrane-site-row: site 15: /app::nested (src/main.trn:41:34-41:43) */
        { Site { function: 7, file: 0, line: 41, column: 34, end_line: 41, end_column: 43 } },
        /* terrane-site-row: site 16: /app::nested (src/main.trn:41:26-41:43) */
        { Site { function: 7, file: 0, line: 41, column: 26, end_line: 41, end_column: 43 } },
        /* terrane-site-row: site 17: /app::nested (src/main.trn:41:12-41:44) */
        { Site { function: 7, file: 0, line: 41, column: 12, end_line: 41, end_column: 44 } },
        /* terrane-site-row: site 18: /app::branch (src/main.trn:45:37-45:46) */
        { Site { function: 8, file: 0, line: 45, column: 37, end_line: 45, end_column: 46 } },
        /* terrane-site-row: site 19: /app::branch (src/main.trn:45:30-45:46) */
        { Site { function: 8, file: 0, line: 45, column: 30, end_line: 45, end_column: 46 } },
        /* terrane-site-row: site 20: /app::branch (src/main.trn:45:16-45:47) */
        { Site { function: 8, file: 0, line: 45, column: 16, end_line: 45, end_column: 47 } },
        /* terrane-site-row: site 21: /app::branch (src/main.trn:46:34-46:43) */
        { Site { function: 8, file: 0, line: 46, column: 34, end_line: 46, end_column: 43 } },
        /* terrane-site-row: site 22: /app::branch (src/main.trn:46:26-46:43) */
        { Site { function: 8, file: 0, line: 46, column: 26, end_line: 46, end_column: 43 } },
        /* terrane-site-row: site 23: /app::branch (src/main.trn:46:12-46:44) */
        { Site { function: 8, file: 0, line: 46, column: 12, end_line: 46, end_column: 44 } },
        /* terrane-site-row: site 24: /app::aggregate (src/main.trn:50:37-50:52) */
        { Site { function: 9, file: 0, line: 50, column: 37, end_line: 50, end_column: 52 } },
        /* terrane-site-row: site 25: /app::aggregate (src/main.trn:50:23-50:53) */
        { Site { function: 9, file: 0, line: 50, column: 23, end_line: 50, end_column: 53 } },
        /* terrane-site-row: site 26: /app::aggregate (src/main.trn:51:37-51:52) */
        { Site { function: 9, file: 0, line: 51, column: 37, end_line: 51, end_column: 52 } },
        /* terrane-site-row: site 27: /app::aggregate (src/main.trn:51:23-51:53) */
        { Site { function: 9, file: 0, line: 51, column: 23, end_line: 51, end_column: 53 } },
        /* terrane-site-row: site 28: /app::aggregate (src/main.trn:52:37-52:52) */
        { Site { function: 9, file: 0, line: 52, column: 37, end_line: 52, end_column: 52 } },
        /* terrane-site-row: site 29: /app::aggregate (src/main.trn:52:23-52:53) */
        { Site { function: 9, file: 0, line: 52, column: 23, end_line: 52, end_column: 53 } },
        /* terrane-site-row: site 30: /app::aggregate (src/main.trn:53:26-53:39) */
        { Site { function: 9, file: 0, line: 53, column: 26, end_line: 53, end_column: 39 } },
        /* terrane-site-row: site 31: /app::aggregate (src/main.trn:53:12-53:40) */
        { Site { function: 9, file: 0, line: 53, column: 12, end_line: 53, end_column: 40 } },
        /* terrane-site-row: site 32: /app::main (src/main.trn:63:13-63:58) */
        { Site { function: 10, file: 0, line: 63, column: 13, end_line: 63, end_column: 58 } },
        /* terrane-site-row: site 33: /app::main (src/main.trn:64:13-64:62) */
        { Site { function: 10, file: 0, line: 64, column: 13, end_line: 64, end_column: 62 } },
        /* terrane-site-row: site 34: /app::main (src/main.trn:65:13-65:42) */
        { Site { function: 10, file: 0, line: 65, column: 13, end_line: 65, end_column: 42 } },
        /* terrane-site-row: site 35: /app::main (src/main.trn:61:20-61:41) */
        { Site { function: 10, file: 0, line: 61, column: 20, end_line: 61, end_column: 41 } },
        /* terrane-site-row: site 36: /app::main (src/main.trn:62:16-62:37) */
        { Site { function: 10, file: 0, line: 62, column: 16, end_line: 62, end_column: 37 } },
        /* terrane-site-row: site 37: /app::main (src/main.trn:66:13-66:71) */
        { Site { function: 10, file: 0, line: 66, column: 13, end_line: 66, end_column: 71 } },
        /* terrane-site-row: site 38: /app::main (src/main.trn:67:13-67:69) */
        { Site { function: 10, file: 0, line: 67, column: 13, end_line: 67, end_column: 69 } },
        /* terrane-site-row: site 39: /app::main (src/main.trn:68:13-68:69) */
        { Site { function: 10, file: 0, line: 68, column: 13, end_line: 68, end_column: 69 } },
        /* terrane-site-row: site 40: /app::main (src/main.trn:69:13-69:72) */
        { Site { function: 10, file: 0, line: 69, column: 13, end_line: 69, end_column: 72 } },
        /* terrane-site-row: site 41: /app::main (src/main.trn:72:23-72:37) */
        { Site { function: 10, file: 0, line: 72, column: 23, end_line: 72, end_column: 37 } },
        /* terrane-site-row: site 42: /app::main (src/main.trn:72:16-72:37) */
        { Site { function: 10, file: 0, line: 72, column: 16, end_line: 72, end_column: 37 } },
        /* terrane-site-row: site 43: /app::main (src/main.trn:73:13-73:60) */
        { Site { function: 10, file: 0, line: 73, column: 13, end_line: 73, end_column: 60 } },
        /* terrane-site-row: site 44: /app::main (src/main.trn:74:13-74:66) */
        { Site { function: 10, file: 0, line: 74, column: 13, end_line: 74, end_column: 66 } },
        /* terrane-site-row: site 45: /app::main (src/main.trn:75:13-75:58) */
        { Site { function: 10, file: 0, line: 75, column: 13, end_line: 75, end_column: 58 } },
        /* terrane-site-row: site 46: /app::main (src/main.trn:76:13-76:58) */
        { Site { function: 10, file: 0, line: 76, column: 13, end_line: 76, end_column: 58 } },
        /* terrane-site-row: site 47: /app::main (src/main.trn:78:13-78:57) */
        { Site { function: 10, file: 0, line: 78, column: 13, end_line: 78, end_column: 57 } },
        /* terrane-site-row: site 48: /app::main (src/main.trn:79:5-79:17) */
        { Site { function: 10, file: 0, line: 79, column: 5, end_line: 79, end_column: 17 } },
        /* terrane-site-row: site 49: /app::main (src/main.trn:80:13-80:61) */
        { Site { function: 10, file: 0, line: 80, column: 13, end_line: 80, end_column: 61 } },
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
    return {
        let value: witness::BorrowedLabel<'_> = __terrane_raised(
                borrowed_label(state),
                0 /* terrane-site: src/main.trn:8:12-8:33 */,
            )
            .into();
        value
    };
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
    return {
        let value: witness::BorrowedGroup<'_> = __terrane_raised(
                borrowed_group(first, second),
                3 /* terrane-site: src/main.trn:15:12-15:41 */,
            )
            .into();
        value
    };
}
fn exact_branch<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    if terrane_int_support::Int::from(
        terrane_string_support::length(&state.clone()) as i128,
    ) > terrane_int_support::Int::from(1_i128)
    {
        let intermediate: witness::BorrowedLabel<'_> = __terrane_raised(
            borrowed_label(state),
            4 /* terrane-site: src/main.trn:19:24-19:45 */,
        );
        return {
            let value: witness::BorrowedGroup<'_> = __terrane_raised(
                    borrowed_label_group(intermediate),
                    5 /* terrane-site: src/main.trn:20:16-20:50 */,
                )
                .into();
            value
        };
    }
    let button_intermediate: witness::BorrowedButton<'_> = __terrane_raised(
        borrowed_button(state),
        6 /* terrane-site: src/main.trn:21:27-21:49 */,
    );
    return {
        let value: witness::BorrowedGroup<'_> = __terrane_raised(
                borrowed_button_group(button_intermediate),
                7 /* terrane-site: src/main.trn:22:12-22:54 */,
            )
            .into();
        value
    };
}
fn exact_aggregate<'view>(state: &'view String) -> witness::BorrowedGroup<'view> {
    let mut children: terrane_collection_support::List<_> = terrane_collection_support::List::<
        _,
    >::new(vec![]);
    children
        .push_unique(
            __terrane_raised(
                borrowed_label(state),
                8 /* terrane-site: src/main.trn:26:23-26:44 */,
            ),
        );
    children
        .push_unique(
            __terrane_raised(
                borrowed_label(state),
                9 /* terrane-site: src/main.trn:27:23-27:44 */,
            ),
        );
    return {
        let value: witness::BorrowedGroup<'_> = __terrane_raised(
                borrowed_row(children),
                10 /* terrane-site: src/main.trn:28:12-28:34 */,
            )
            .into();
        value
    };
}
fn direct(state: &i64) -> Label {
    return __terrane_raised(
        label(
            terrane_int_support::Int::from(
                __terrane_raised(
                    terrane_int_support::fixed_addition(state.clone(), 1),
                    11 /* terrane-site: src/main.trn:32:19-32:28 */,
                ) as i128,
            ),
        ),
        12 /* terrane-site: src/main.trn:32:12-32:28 */,
    );
}
fn helper(value: i64) -> Label {
    return __terrane_raised(
        label(terrane_int_support::Int::from(value as i128)),
        13 /* terrane-site: src/main.trn:35:12-35:24 */,
    );
}
fn through_helper(state: &i64) -> Label {
    return helper(
        __terrane_raised(
            terrane_int_support::fixed_addition(state.clone(), 1),
            14 /* terrane-site: src/main.trn:38:20-38:29 */,
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
                                15 /* terrane-site: src/main.trn:41:34-41:43 */,
                            ) as i128,
                        ),
                    ),
                    16 /* terrane-site: src/main.trn:41:26-41:43 */,
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
        17 /* terrane-site: src/main.trn:41:12-41:44 */,
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
                                    18 /* terrane-site: src/main.trn:45:37-45:46 */,
                                ) as i128,
                            ),
                        ),
                        19 /* terrane-site: src/main.trn:45:30-45:46 */,
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
            20 /* terrane-site: src/main.trn:45:16-45:47 */,
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
                                21 /* terrane-site: src/main.trn:46:34-46:43 */,
                            ) as i128,
                        ),
                    ),
                    22 /* terrane-site: src/main.trn:46:26-46:43 */,
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
        23 /* terrane-site: src/main.trn:46:12-46:44 */,
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
                            24 /* terrane-site: src/main.trn:50:37-50:52 */,
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
                25 /* terrane-site: src/main.trn:50:23-50:53 */,
            ),
        );
    children
        .append(
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_raised(
                            traced_label(terrane_int_support::Int::from(2_i128)),
                            26 /* terrane-site: src/main.trn:51:37-51:52 */,
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
                27 /* terrane-site: src/main.trn:51:23-51:53 */,
            ),
        );
    children
        .append(
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::into_widget(
                        __terrane_raised(
                            traced_label(terrane_int_support::Int::from(3_i128)),
                            28 /* terrane-site: src/main.trn:52:37-52:52 */,
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
                29 /* terrane-site: src/main.trn:52:23-52:53 */,
            ),
        );
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::into_widget(
                __terrane_raised(
                    row(children),
                    30 /* terrane-site: src/main.trn:53:26-53:39 */,
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
        31 /* terrane-site: src/main.trn:53:12-53:40 */,
    );
}
fn main() {
    let state: i64 = 40;
    let exact_state: String = String::from("40");
    let enabled: bool = true;
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
        "witness::render_scoped::<i64, String, bool>")) }, 32 /* terrane-site: src/main.trn:63:13-63:58 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, { struct
        TerraneInvocationScopedCallback; impl < 'view > witness::ExactViewFn < 'view,
        String, bool > for TerraneInvocationScopedCallback { fn view(&self,
        callback_argument_0 : &'view std::string::String) -> witness::ScopedView < 'view,
        String, bool > { exact(callback_argument_0).into() } }
        TerraneInvocationScopedCallback }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 33 /* terrane-site: src/main.trn:64:13-64:62 */))
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
        "witness::render_default::<i64, String, bool>")) }, 34 /* terrane-site: src/main.trn:65:13-65:42 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, { struct
        TerraneInvocationScopedCallback { callback : for < 'callback > fn (bool,
        &'callback std::string::String) -> witness::BorrowedLabel < 'callback >,
        capture_0 : bool } impl < 'view > witness::ExactViewFn < 'view, String, bool >
        for TerraneInvocationScopedCallback { fn view(&self, callback_argument_0 : &'view
        std::string::String) -> witness::ScopedView < 'view, String, bool > { (self
        .callback) (self.capture_0.clone(), callback_argument_0).into() } }
        TerraneInvocationScopedCallback { callback : { move | enabled : bool, value :
        &String | -> witness::BorrowedLabel < '_ > { if enabled { return { let value :
        witness::BorrowedLabel < '_ > = __terrane_raised(borrowed_label(value),
        35 /* terrane-site: src/main.trn:61:20-61:41 */).into(); value }; } return {
        let value : witness::BorrowedLabel < '_ > =
        __terrane_raised(borrowed_label(value), 36 /* terrane-site: src/main.trn:62:16-62:37 */).into(); value }; } }, capture_0 : enabled.clone() }
        }))) { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 37 /* terrane-site: src/main.trn:66:13-66:71 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, { struct
        TerraneInvocationScopedCallback; impl < 'view > witness::ExactViewFn < 'view,
        String, bool > for TerraneInvocationScopedCallback { fn view(&self,
        callback_argument_0 : &'view std::string::String) -> witness::ScopedView < 'view,
        String, bool > { exact_nested(callback_argument_0).into() } }
        TerraneInvocationScopedCallback }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 38 /* terrane-site: src/main.trn:67:13-67:69 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, { struct
        TerraneInvocationScopedCallback; impl < 'view > witness::ExactViewFn < 'view,
        String, bool > for TerraneInvocationScopedCallback { fn view(&self,
        callback_argument_0 : &'view std::string::String) -> witness::ScopedView < 'view,
        String, bool > { exact_branch(callback_argument_0).into() } }
        TerraneInvocationScopedCallback }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 39 /* terrane-site: src/main.trn:68:13-68:69 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | witness::render_exact::
        < String, bool > (&exact_state, String::from("message"), true, { struct
        TerraneInvocationScopedCallback; impl < 'view > witness::ExactViewFn < 'view,
        String, bool > for TerraneInvocationScopedCallback { fn view(&self,
        callback_argument_0 : &'view std::string::String) -> witness::ScopedView < 'view,
        String, bool > { exact_aggregate(callback_argument_0).into() } }
        TerraneInvocationScopedCallback }))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness",
        "witness::render_exact::<String, bool>")) }, 40 /* terrane-site: src/main.trn:69:13-69:72 */))
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
                            41 /* terrane-site: src/main.trn:72:23-72:37 */,
                        ) as i128,
                    ),
                ),
                42 /* terrane-site: src/main.trn:72:16-72:37 */,
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
        "witness::render_scoped::<i64, String, bool>")) }, 43 /* terrane-site: src/main.trn:73:13-73:60 */))
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
        "witness::render_scoped::<i64, String, bool>")) }, 44 /* terrane-site: src/main.trn:74:13-74:66 */))
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
        "witness::render_scoped::<i64, String, bool>")) }, 45 /* terrane-site: src/main.trn:75:13-75:58 */))
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
        "witness::render_scoped::<i64, String, bool>")) }, 46 /* terrane-site: src/main.trn:76:13-76:58 */))
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
        "witness::render_scoped::<i64, String, bool>")) }, 47 /* terrane-site: src/main.trn:78:13-78:57 */))
    );
    __terrane_raised(reset_trace(), 48 /* terrane-site: src/main.trn:79:5-79:17 */);
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
        "witness::render_scoped::<i64, String, bool>")) }, 49 /* terrane-site: src/main.trn:80:13-80:61 */))
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
