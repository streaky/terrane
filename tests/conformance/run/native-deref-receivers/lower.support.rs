// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
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
        "/core/time::invalid-duration",
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
    pub static FILES: [&str; 2] = ["src/main.trn", "core/time.trn"];
    pub static FUNCTIONS: [&str; 15] = [
        "/native-deref-receivers::bump",
        "/native-deref-receivers::ordinary-view",
        "/native-deref-receivers::ordinary-guard",
        "/native-deref-receivers::suspended-view",
        "/native-deref-receivers::named-view",
        "/native-deref-receivers::pointers-and-guards",
        "/core/time::multiply",
        "/core/time::seconds",
        "/core/time::milliseconds",
        "/core/time::microseconds",
        "/core/time::nanoseconds",
        "/core/time::duration-until",
        "/core/time::at",
        "/core/time::sleep-until",
        "/core/time::interval",
    ];
    pub static SITES: [Site; 71] = [
        /* terrane-site-row: site 0: /native-deref-receivers::bump (src/main.trn:7:5-7:19) */
        { Site { function: 0, file: 0, line: 7, column: 5, end_line: 7, end_column: 19 } },
        /* terrane-site-row: site 1: /native-deref-receivers::ordinary-view (src/main.trn:10:13-10:24) */
        { Site { function: 1, file: 0, line: 10, column: 13, end_line: 10, end_column: 24 } },
        /* terrane-site-row: site 2: /native-deref-receivers::ordinary-view (src/main.trn:11:12-11:23) */
        { Site { function: 1, file: 0, line: 11, column: 12, end_line: 11, end_column: 23 } },
        /* terrane-site-row: site 3: /native-deref-receivers::ordinary-view (src/main.trn:12:14-12:25) */
        { Site { function: 1, file: 0, line: 12, column: 14, end_line: 12, end_column: 25 } },
        /* terrane-site-row: site 4: /native-deref-receivers::ordinary-view (src/main.trn:13:14-13:28) */
        { Site { function: 1, file: 0, line: 13, column: 14, end_line: 13, end_column: 28 } },
        /* terrane-site-row: site 5: /native-deref-receivers::ordinary-view (src/main.trn:14:12-14:25) */
        { Site { function: 1, file: 0, line: 14, column: 12, end_line: 14, end_column: 25 } },
        /* terrane-site-row: site 6: /native-deref-receivers::ordinary-guard (src/main.trn:17:13-17:27) */
        { Site { function: 2, file: 0, line: 17, column: 13, end_line: 17, end_column: 27 } },
        /* terrane-site-row: site 7: /native-deref-receivers::ordinary-guard (src/main.trn:18:13-18:26) */
        { Site { function: 2, file: 0, line: 18, column: 13, end_line: 18, end_column: 26 } },
        /* terrane-site-row: site 8: /native-deref-receivers::ordinary-guard (src/main.trn:19:5-19:18) */
        { Site { function: 2, file: 0, line: 19, column: 5, end_line: 19, end_column: 18 } },
        /* terrane-site-row: site 9: /native-deref-receivers::ordinary-guard (src/main.trn:20:12-20:24) */
        { Site { function: 2, file: 0, line: 20, column: 12, end_line: 20, end_column: 24 } },
        /* terrane-site-row: site 10: /native-deref-receivers::suspended-view (src/main.trn:23:13-23:24) */
        { Site { function: 3, file: 0, line: 23, column: 13, end_line: 23, end_column: 24 } },
        /* terrane-site-row: site 11: /native-deref-receivers::suspended-view (src/main.trn:24:12-24:23) */
        { Site { function: 3, file: 0, line: 24, column: 12, end_line: 24, end_column: 23 } },
        /* terrane-site-row: site 12: /native-deref-receivers::suspended-view (src/main.trn:25:26-25:51) */
        { Site { function: 3, file: 0, line: 25, column: 26, end_line: 25, end_column: 51 } },
        /* terrane-site-row: site 13: /native-deref-receivers::suspended-view (src/main.trn:26:12-26:23) */
        { Site { function: 3, file: 0, line: 26, column: 12, end_line: 26, end_column: 23 } },
        /* terrane-site-row: site 14: /native-deref-receivers::named-view (src/main.trn:29:15-29:26) */
        { Site { function: 4, file: 0, line: 29, column: 15, end_line: 29, end_column: 26 } },
        /* terrane-site-row: site 15: /native-deref-receivers::named-view (src/main.trn:30:13-30:24) */
        { Site { function: 4, file: 0, line: 30, column: 13, end_line: 30, end_column: 24 } },
        /* terrane-site-row: site 16: /native-deref-receivers::named-view (src/main.trn:31:12-31:55) */
        { Site { function: 4, file: 0, line: 31, column: 12, end_line: 31, end_column: 55 } },
        /* terrane-site-row: site 17: /native-deref-receivers::named-view (src/main.trn:32:5-32:20) */
        { Site { function: 4, file: 0, line: 32, column: 5, end_line: 32, end_column: 20 } },
        /* terrane-site-row: site 18: /native-deref-receivers::named-view (src/main.trn:33:12-33:23) */
        { Site { function: 4, file: 0, line: 33, column: 12, end_line: 33, end_column: 23 } },
        /* terrane-site-row: site 19: /native-deref-receivers::pointers-and-guards (src/main.trn:36:14-36:20) */
        { Site { function: 5, file: 0, line: 36, column: 14, end_line: 36, end_column: 20 } },
        /* terrane-site-row: site 20: /native-deref-receivers::pointers-and-guards (src/main.trn:37:13-37:26) */
        { Site { function: 5, file: 0, line: 37, column: 13, end_line: 37, end_column: 26 } },
        /* terrane-site-row: site 21: /native-deref-receivers::pointers-and-guards (src/main.trn:38:12-38:25) */
        { Site { function: 5, file: 0, line: 38, column: 12, end_line: 38, end_column: 25 } },
        /* terrane-site-row: site 22: /native-deref-receivers::pointers-and-guards (src/main.trn:39:13-39:35) */
        { Site { function: 5, file: 0, line: 39, column: 13, end_line: 39, end_column: 35 } },
        /* terrane-site-row: site 23: /native-deref-receivers::pointers-and-guards (src/main.trn:40:14-40:20) */
        { Site { function: 5, file: 0, line: 40, column: 14, end_line: 40, end_column: 20 } },
        /* terrane-site-row: site 24: /native-deref-receivers::pointers-and-guards (src/main.trn:40:13-40:28) */
        { Site { function: 5, file: 0, line: 40, column: 13, end_line: 40, end_column: 28 } },
        /* terrane-site-row: site 25: /native-deref-receivers::pointers-and-guards (src/main.trn:41:13-41:18) */
        { Site { function: 5, file: 0, line: 41, column: 13, end_line: 41, end_column: 18 } },
        /* terrane-site-row: site 26: /native-deref-receivers::pointers-and-guards (src/main.trn:42:13-42:25) */
        { Site { function: 5, file: 0, line: 42, column: 13, end_line: 42, end_column: 25 } },
        /* terrane-site-row: site 27: /native-deref-receivers::pointers-and-guards (src/main.trn:43:14-43:23) */
        { Site { function: 5, file: 0, line: 43, column: 14, end_line: 43, end_column: 23 } },
        /* terrane-site-row: site 28: /native-deref-receivers::pointers-and-guards (src/main.trn:43:13-43:31) */
        { Site { function: 5, file: 0, line: 43, column: 13, end_line: 43, end_column: 31 } },
        /* terrane-site-row: site 29: /native-deref-receivers::pointers-and-guards (src/main.trn:44:13-44:22) */
        { Site { function: 5, file: 0, line: 44, column: 13, end_line: 44, end_column: 22 } },
        /* terrane-site-row: site 30: /native-deref-receivers::pointers-and-guards (src/main.trn:45:5-45:18) */
        { Site { function: 5, file: 0, line: 45, column: 5, end_line: 45, end_column: 18 } },
        /* terrane-site-row: site 31: /native-deref-receivers::pointers-and-guards (src/main.trn:46:13-46:25) */
        { Site { function: 5, file: 0, line: 46, column: 13, end_line: 46, end_column: 25 } },
        /* terrane-site-row: site 32: /native-deref-receivers::pointers-and-guards (src/main.trn:47:13-47:24) */
        { Site { function: 5, file: 0, line: 47, column: 13, end_line: 47, end_column: 24 } },
        /* terrane-site-row: site 33: /native-deref-receivers::pointers-and-guards (src/main.trn:48:14-48:23) */
        { Site { function: 5, file: 0, line: 48, column: 14, end_line: 48, end_column: 23 } },
        /* terrane-site-row: site 34: /native-deref-receivers::pointers-and-guards (src/main.trn:48:13-48:31) */
        { Site { function: 5, file: 0, line: 48, column: 13, end_line: 48, end_column: 31 } },
        /* terrane-site-row: site 35: /native-deref-receivers::pointers-and-guards (src/main.trn:49:14-49:28) */
        { Site { function: 5, file: 0, line: 49, column: 14, end_line: 49, end_column: 28 } },
        /* terrane-site-row: site 36: /native-deref-receivers::pointers-and-guards (src/main.trn:49:13-49:35) */
        { Site { function: 5, file: 0, line: 49, column: 13, end_line: 49, end_column: 35 } },
        /* terrane-site-row: site 37: /native-deref-receivers::pointers-and-guards (src/main.trn:50:14-50:25) */
        { Site { function: 5, file: 0, line: 50, column: 14, end_line: 50, end_column: 25 } },
        /* terrane-site-row: site 38: /native-deref-receivers::pointers-and-guards (src/main.trn:51:5-51:19) */
        { Site { function: 5, file: 0, line: 51, column: 5, end_line: 51, end_column: 19 } },
        /* terrane-site-row: site 39: /native-deref-receivers::pointers-and-guards (src/main.trn:52:13-52:26) */
        { Site { function: 5, file: 0, line: 52, column: 13, end_line: 52, end_column: 26 } },
        /* terrane-site-row: site 40: /native-deref-receivers::pointers-and-guards (src/main.trn:54:13-54:26) */
        { Site { function: 5, file: 0, line: 54, column: 13, end_line: 54, end_column: 26 } },
        /* terrane-site-row: site 41: /native-deref-receivers::pointers-and-guards (src/main.trn:55:13-55:27) */
        { Site { function: 5, file: 0, line: 55, column: 13, end_line: 55, end_column: 27 } },
        /* terrane-site-row: site 42: /native-deref-receivers::pointers-and-guards (src/main.trn:56:33-56:46) */
        { Site { function: 5, file: 0, line: 56, column: 33, end_line: 56, end_column: 46 } },
        /* terrane-site-row: site 43: /native-deref-receivers::pointers-and-guards (src/main.trn:57:17-57:33) */
        { Site { function: 5, file: 0, line: 57, column: 17, end_line: 57, end_column: 33 } },
        /* terrane-site-row: site 44: /native-deref-receivers::pointers-and-guards (src/main.trn:58:9-58:22) */
        { Site { function: 5, file: 0, line: 58, column: 9, end_line: 58, end_column: 22 } },
        /* terrane-site-row: site 45: /native-deref-receivers::pointers-and-guards (src/main.trn:59:17-59:29) */
        { Site { function: 5, file: 0, line: 59, column: 17, end_line: 59, end_column: 29 } },
        /* terrane-site-row: site 46: /native-deref-receivers::pointers-and-guards (src/main.trn:60:36-60:47) */
        { Site { function: 5, file: 0, line: 60, column: 36, end_line: 60, end_column: 47 } },
        /* terrane-site-row: site 47: /native-deref-receivers::pointers-and-guards (src/main.trn:61:21-61:32) */
        { Site { function: 5, file: 0, line: 61, column: 21, end_line: 61, end_column: 32 } },
        /* terrane-site-row: site 48: /native-deref-receivers::pointers-and-guards (src/main.trn:62:21-62:36) */
        { Site { function: 5, file: 0, line: 62, column: 21, end_line: 62, end_column: 36 } },
        /* terrane-site-row: site 49: /native-deref-receivers::pointers-and-guards (src/main.trn:64:17-64:32) */
        { Site { function: 5, file: 0, line: 64, column: 17, end_line: 64, end_column: 32 } },
        /* terrane-site-row: site 50: /native-deref-receivers::pointers-and-guards (src/main.trn:66:13-66:29) */
        { Site { function: 5, file: 0, line: 66, column: 13, end_line: 66, end_column: 29 } },
        /* terrane-site-row: site 51: /native-deref-receivers::pointers-and-guards (src/main.trn:67:33-67:45) */
        { Site { function: 5, file: 0, line: 67, column: 33, end_line: 67, end_column: 45 } },
        /* terrane-site-row: site 52: /native-deref-receivers::pointers-and-guards (src/main.trn:68:9-68:22) */
        { Site { function: 5, file: 0, line: 68, column: 9, end_line: 68, end_column: 22 } },
        /* terrane-site-row: site 53: /native-deref-receivers::pointers-and-guards (src/main.trn:69:17-69:29) */
        { Site { function: 5, file: 0, line: 69, column: 17, end_line: 69, end_column: 29 } },
        /* terrane-site-row: site 54: /native-deref-receivers::pointers-and-guards (src/main.trn:71:13-71:28) */
        { Site { function: 5, file: 0, line: 71, column: 13, end_line: 71, end_column: 28 } },
        /* terrane-site-row: site 55: /native-deref-receivers::pointers-and-guards (src/main.trn:73:32-73:44) */
        { Site { function: 5, file: 0, line: 73, column: 32, end_line: 73, end_column: 44 } },
        /* terrane-site-row: site 56: /native-deref-receivers::pointers-and-guards (src/main.trn:74:18-74:29) */
        { Site { function: 5, file: 0, line: 74, column: 18, end_line: 74, end_column: 29 } },
        /* terrane-site-row: site 57: /native-deref-receivers::pointers-and-guards (src/main.trn:75:17-75:30) */
        { Site { function: 5, file: 0, line: 75, column: 17, end_line: 75, end_column: 30 } },
        /* terrane-site-row: site 58: /native-deref-receivers::pointers-and-guards (src/main.trn:76:38-76:50) */
        { Site { function: 5, file: 0, line: 76, column: 38, end_line: 76, end_column: 50 } },
        /* terrane-site-row: site 59: /native-deref-receivers::pointers-and-guards (src/main.trn:77:26-77:39) */
        { Site { function: 5, file: 0, line: 77, column: 26, end_line: 77, end_column: 39 } },
        /* terrane-site-row: site 60: /native-deref-receivers::pointers-and-guards (src/main.trn:80:5-80:19) */
        { Site { function: 5, file: 0, line: 80, column: 5, end_line: 80, end_column: 19 } },
        /* terrane-site-row: site 61: /native-deref-receivers::pointers-and-guards (src/main.trn:82:13-82:26) */
        { Site { function: 5, file: 0, line: 82, column: 13, end_line: 82, end_column: 26 } },
        /* terrane-site-row: site 62: /core/time::multiply (core/time.trn:62:13-62:45) */
        { Site { function: 6, file: 1, line: 62, column: 13, end_line: 62, end_column: 45 } },
        /* terrane-site-row: site 63: /core/time::seconds (core/time.trn:38:13-38:45) */
        { Site { function: 7, file: 1, line: 38, column: 13, end_line: 38, end_column: 45 } },
        /* terrane-site-row: site 64: /core/time::milliseconds (core/time.trn:43:13-43:45) */
        { Site { function: 8, file: 1, line: 43, column: 13, end_line: 43, end_column: 45 } },
        /* terrane-site-row: site 65: /core/time::microseconds (core/time.trn:48:13-48:45) */
        { Site { function: 9, file: 1, line: 48, column: 13, end_line: 48, end_column: 45 } },
        /* terrane-site-row: site 66: /core/time::nanoseconds (core/time.trn:53:13-53:45) */
        { Site { function: 10, file: 1, line: 53, column: 13, end_line: 53, end_column: 45 } },
        /* terrane-site-row: site 67: /core/time::duration-until (core/time.trn:76:13-76:45) */
        { Site { function: 11, file: 1, line: 76, column: 13, end_line: 76, end_column: 45 } },
        /* terrane-site-row: site 68: /core/time::at (core/time.trn:105:13-105:45) */
        { Site { function: 12, file: 1, line: 105, column: 13, end_line: 105, end_column: 45 } },
        /* terrane-site-row: site 69: /core/time::sleep-until (core/time.trn:161:13-161:45) */
        { Site { function: 13, file: 1, line: 161, column: 13, end_line: 161, end_column: 45 } },
        /* terrane-site-row: site 70: /core/time::interval (core/time.trn:172:13-172:45) */
        { Site { function: 14, file: 1, line: 172, column: 13, end_line: 172, end_column: 45 } },
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
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_domain() -> terrane_int_support::Int {
    terrane_int_support::Int::from(1_i128)
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_monotonic() -> terrane_int_support::Int {
    terrane_int_support::Int::from_u128(terrane_platform_support::monotonic_nanos())
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_deadline_expired(target: &terrane_int_support::Int) -> bool {
    terrane_time_monotonic() >= target.clone()
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
async fn terrane_time_sleep_until(target: terrane_int_support::Int) {
    const MAX_WAKE_CHUNK_NANOS: u64 = u64::MAX;
    loop {
        let now = terrane_time_monotonic();
        if now >= target {
            return;
        }
        let remaining = target.clone() - now;
        let chunk = terrane_int_support::coerce::<u64>(&remaining)
            .unwrap_or(MAX_WAKE_CHUNK_NANOS);
        terrane_platform_support::sleep(std::time::Duration::from_nanos(chunk)).await;
    }
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_sleep_after(
    elapsed: terrane_int_support::Int,
) -> impl std::future::Future<Output = ()> {
    let target = terrane_time_monotonic() + elapsed;
    async move {
        terrane_time_sleep_until(target).await;
    }
}
pub type TerranePlatformCapability = terrane_platform_support::Capability;
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
#[allow(dead_code)]
fn terrane_platform_cancellation_token() -> TerranePlatformCapability {
    terrane_platform_support::cancellation_token()
}
#[allow(dead_code)]
fn terrane_platform_no_resource() -> TerranePlatformCapability {
    TerranePlatformCapability::default()
}
#[allow(dead_code)]
fn terrane_platform_failed_result() -> TerranePlatformResult {
    TerranePlatformResult::error("uninitialized platform value")
}
#[allow(dead_code)]
fn terrane_platform_cancel(token: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::cancel(token)
}
#[allow(dead_code)]
fn terrane_platform_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
#[allow(dead_code)]
fn terrane_platform_result_resource_limit(result: &TerranePlatformResult) -> bool {
    result.resource_limit
}
#[allow(dead_code)]
fn terrane_platform_result_truncated(result: &TerranePlatformResult) -> bool {
    result.truncated
}
#[allow(dead_code)]
fn terrane_platform_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
#[allow(dead_code)]
fn terrane_platform_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_text(result: &TerranePlatformResult) -> String {
    result.text.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_detail(result: &TerranePlatformResult) -> String {
    result.detail.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_bytes(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_int(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
#[allow(dead_code)]
fn terrane_platform_result_bool(result: &TerranePlatformResult) -> bool {
    result.flag
}
#[allow(dead_code)]
fn terrane_platform_result_entries(result: &TerranePlatformResult) -> Vec<String> {
    result.entries.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_capability(
    result: &TerranePlatformResult,
) -> TerranePlatformCapability {
    result.capability.clone().unwrap_or_default()
}
fn terrane_platform_time_wall() -> TerranePlatformResult {
    terrane_platform_support::wall_time()
}
fn terrane_platform_time_wall_seconds(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
fn terrane_platform_time_wall_nanoseconds(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.secondary_number)
}
fn terrane_platform_time_domain() -> terrane_int_support::Int {
    terrane_time_domain()
}
fn terrane_platform_time_monotonic() -> terrane_int_support::Int {
    terrane_time_monotonic()
}
async fn terrane_platform_time_sleep_until(target: terrane_int_support::Int) {
    terrane_time_sleep_until(target).await;
}
trait TerraneTimeDivisor {
    fn into_time_int(self) -> terrane_int_support::Int;
}
impl TerraneTimeDivisor for i32 {
    fn into_time_int(self) -> terrane_int_support::Int {
        terrane_int_support::Int::from(i64::from(self))
    }
}
impl TerraneTimeDivisor for terrane_int_support::Int {
    fn into_time_int(self) -> terrane_int_support::Int {
        self
    }
}
fn terrane_platform_time_div(
    value: &terrane_int_support::Int,
    divisor: impl TerraneTimeDivisor,
) -> terrane_int_support::Int {
    value
        .euclidean_div(&divisor.into_time_int())
        .expect("bundled time arithmetic divisors must be nonzero")
}
fn terrane_platform_time_mod(
    value: &terrane_int_support::Int,
    divisor: impl TerraneTimeDivisor,
) -> terrane_int_support::Int {
    value
        .modulo(&divisor.into_time_int())
        .expect("bundled time arithmetic divisors must be nonzero")
}
// Source: <terrane>/projected/deps/receiver-api.trn
// Namespace: deps/receiver-api
pub type ArcOfReceiverApiRecord = std::sync::Arc<receiver_api::Record>;
pub type ArcOfStdRcRcReceiverApiRecord = std::sync::Arc<
    std::rc::Rc<receiver_api::Record>,
>;
pub type BoxOfReceiverApiRecord = std::boxed::Box<receiver_api::Record>;
pub type BoxOfStdBoxedBoxReceiverApiRecord = std::boxed::Box<
    std::boxed::Box<receiver_api::Record>,
>;
pub use receiver_api::Locks;
pub type MutexGuardOfReceiverApiRecord<'__terrane_view_0> = std::sync::MutexGuard<
    '__terrane_view_0,
    receiver_api::Record,
>;
pub type RcOfReceiverApiRecord = std::rc::Rc<receiver_api::Record>;
pub use receiver_api::RecordHandle;
pub type RecordView<'a> = receiver_api::RecordView<'a>;
pub type RwLockReadGuardOfReceiverApiRecord<'__terrane_view_0> = std::sync::RwLockReadGuard<
    '__terrane_view_0,
    receiver_api::Record,
>;
pub type RwLockWriteGuardOfReceiverApiRecord<'__terrane_view_0> = std::sync::RwLockWriteGuard<
    '__terrane_view_0,
    receiver_api::Record,
>;
pub fn terrane_static_trn_4c6f636b73_new(
    value: terrane_int_support::Int,
) -> Result<Locks, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::Locks::new(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::Locks",
                ),
            )
        }
    }
}
pub fn arc(
    value: terrane_int_support::Int,
) -> Result<std::sync::Arc<receiver_api::Record>, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::arc(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::arc",
                ),
            )
        }
    }
}
pub fn boxed(
    value: terrane_int_support::Int,
) -> Result<std::boxed::Box<receiver_api::Record>, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::boxed(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::boxed",
                ),
            )
        }
    }
}
pub fn chain(
    value: terrane_int_support::Int,
) -> Result<
    std::sync::Arc<std::rc::Rc<receiver_api::Record>>,
    crate::TerraneForeignError,
> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::chain(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::chain",
                ),
            )
        }
    }
}
pub fn nested_box(
    value: terrane_int_support::Int,
) -> Result<
    std::boxed::Box<std::boxed::Box<receiver_api::Record>>,
    crate::TerraneForeignError,
> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::nested_box(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::nested_box",
                ),
            )
        }
    }
}
pub fn rc(
    value: terrane_int_support::Int,
) -> Result<std::rc::Rc<receiver_api::Record>, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::rc(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::rc",
                ),
            )
        }
    }
}
pub fn same_arc(
    left: &std::sync::Arc<receiver_api::Record>,
    right: &std::sync::Arc<receiver_api::Record>,
) -> Result<bool, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::same_arc(left, right)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::same_arc",
                ),
            )
        }
    }
}
pub fn view_from<'view>(
    ignored: &'view RecordHandle,
    owner: &'view RecordHandle,
) -> Result<receiver_api::RecordView<'view>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::view_from(ignored, owner)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::view_from",
                ),
            )
        }
    }
}
pub fn wrapped(
    value: terrane_int_support::Int,
) -> Result<RecordHandle, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| receiver_api::wrapped(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "receiver-api",
                    "receiver_api::wrapped",
                ),
            )
        }
    }
}
// Source: core/modifiers.trn
// Namespace: core/modifiers
#[derive(Clone)]
pub struct Modifier {
    pub operation: String,
    pub targets: terrane_collection_support::List<String>,
    pub structural: bool,
    pub repeatable: bool,
}
impl Modifier {
    pub fn terrane_construct() -> Self {
        Self {
            operation: String::new(),
            targets: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("binding")]),
            structural: false,
            repeatable: false,
        }
    }
}
#[derive(Clone)]
pub struct ScopedAccess {}
impl ScopedAccess {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
// Source: core/time.trn
// Namespace: core/time
#[derive(Clone)]
pub struct InvalidDuration {
    pub message: String,
}
impl InvalidDuration {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("duration must be exact and non-negative"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DurationSubtraction {
    pub total_nanoseconds: terrane_int_support::Int,
}
impl DurationSubtraction {
    pub fn terrane_construct(total: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(total);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, total: terrane_int_support::Int) {
        {
            let __terrane_value: terrane_int_support::Int = total.clone();
            self.total_nanoseconds = __terrane_value;
        }
    }
    pub fn checked(&self, other: Duration) -> Option<Duration> {
        let difference: terrane_int_support::Int;
        if self.total_nanoseconds.clone() < other.total_nanoseconds.clone() {
            return None;
        }
        difference = self.total_nanoseconds.clone() - other.total_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(&difference, 1000000000.clone()),
                terrane_platform_time_mod(&difference, 1000000000.clone()),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration {
    pub seconds: terrane_int_support::Int,
    pub nanoseconds: terrane_int_support::Int,
    pub total_nanoseconds: terrane_int_support::Int,
    pub subtract: DurationSubtraction,
}
impl Duration {
    pub fn terrane_construct(
        whole_seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
            subtract: DurationSubtraction::terrane_construct(
                terrane_int_support::Int::from(0_i128),
            ),
        };
        __terrane_constructed_value.construct(whole_seconds, fractional_nanoseconds);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        whole_seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) {
        {
            let __terrane_value: terrane_int_support::Int = whole_seconds.clone();
            self.seconds = __terrane_value;
        }
        {
            let __terrane_value: terrane_int_support::Int = fractional_nanoseconds
                .clone();
            self.nanoseconds = __terrane_value;
        }
        {
            let __terrane_value: terrane_int_support::Int = whole_seconds.clone()
                * terrane_int_support::Int::from(1000000000_i128)
                + fractional_nanoseconds.clone();
            self.total_nanoseconds = __terrane_value;
        }
        {
            let __terrane_value: DurationSubtraction = DurationSubtraction::terrane_construct(
                self.total_nanoseconds.clone(),
            );
            self.subtract = __terrane_value;
        }
    }
    pub fn add(&self, other: Duration) -> Duration {
        let fractional: terrane_int_support::Int;
        fractional = self.nanoseconds.clone() + other.nanoseconds.clone();
        return Duration::terrane_construct(
            self.seconds.clone() + other.seconds.clone()
                + terrane_platform_time_div(&fractional, 1000000000.clone()),
            terrane_platform_time_mod(&fractional, 1000000000.clone()),
        );
    }
    pub fn multiply(
        &self,
        multiplier: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        let total: terrane_int_support::Int;
        if multiplier < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    62 /* terrane-site: core/time.trn:62:13-62:45 */,
                )
            });
        }
        total = self.total_nanoseconds.clone() * multiplier.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&total, 1000000000.clone()),
                terrane_platform_time_mod(&total, 1000000000.clone()),
            ),
        );
    }
    pub fn terrane_static_seconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    63 /* terrane-site: core/time.trn:38:13-38:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                value.clone(),
                terrane_int_support::Int::from(0_i128),
            ),
        );
    }
    pub fn terrane_static_milliseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    64 /* terrane-site: core/time.trn:43:13-43:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000.clone()),
                terrane_platform_time_mod(&value, 1000.clone())
                    * terrane_int_support::Int::from(1000000_i128),
            ),
        );
    }
    pub fn terrane_static_microseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    65 /* terrane-site: core/time.trn:48:13-48:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000000.clone()),
                terrane_platform_time_mod(&value, 1000000.clone())
                    * terrane_int_support::Int::from(1000_i128),
            ),
        );
    }
    pub fn terrane_static_nanoseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    66 /* terrane-site: core/time.trn:53:13-53:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000000000.clone()),
                terrane_platform_time_mod(&value, 1000000000.clone()),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MonotonicInstant {
    pub domain: terrane_int_support::Int,
    pub elapsed_nanoseconds: terrane_int_support::Int,
}
impl MonotonicInstant {
    pub fn terrane_construct(
        runtime_domain: terrane_int_support::Int,
        elapsed: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            domain: terrane_int_support::Int::from(0_i128),
            elapsed_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(runtime_domain, elapsed);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        runtime_domain: terrane_int_support::Int,
        elapsed: terrane_int_support::Int,
    ) {
        {
            let __terrane_value: terrane_int_support::Int = runtime_domain.clone();
            self.domain = __terrane_value;
        }
        {
            let __terrane_value: terrane_int_support::Int = elapsed.clone();
            self.elapsed_nanoseconds = __terrane_value;
        }
    }
    pub fn duration_until(
        &self,
        later: &MonotonicInstant,
    ) -> Result<Duration, TerraneError> {
        let elapsed: terrane_int_support::Int;
        if self.domain.clone()
            != {
                let __terrane_snapshot = later.domain.clone();
                __terrane_snapshot
            }
            || {
                let __terrane_snapshot = later.elapsed_nanoseconds.clone();
                __terrane_snapshot
            } < self.elapsed_nanoseconds.clone()
        {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    67 /* terrane-site: core/time.trn:76:13-76:45 */,
                )
            });
        }
        elapsed = {
            let __terrane_snapshot = later.elapsed_nanoseconds.clone();
            __terrane_snapshot
        } - self.elapsed_nanoseconds.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed, 1000000000.clone()),
                terrane_platform_time_mod(&elapsed, 1000000000.clone()),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Instant {
    pub unix_seconds: terrane_int_support::Int,
    pub nanoseconds: terrane_int_support::Int,
}
impl Instant {
    pub fn terrane_construct(
        seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            unix_seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(seconds, fractional_nanoseconds);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) {
        {
            let __terrane_value: terrane_int_support::Int = seconds.clone();
            self.unix_seconds = __terrane_value;
        }
        {
            let __terrane_value: terrane_int_support::Int = fractional_nanoseconds
                .clone();
            self.nanoseconds = __terrane_value;
        }
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Deadline {
    pub expires_at: MonotonicInstant,
}
impl Deadline {
    pub fn terrane_construct(target: MonotonicInstant) -> Self {
        let mut __terrane_constructed_value = Self {
            expires_at: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
        };
        __terrane_constructed_value.construct(target);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, target: MonotonicInstant) {
        {
            let __terrane_value: MonotonicInstant = target;
            self.expires_at = __terrane_value;
        }
    }
    pub fn remaining(&self) -> Option<Duration> {
        let now: MonotonicInstant;
        let elapsed: terrane_int_support::Int;
        now = Clock::terrane_static_monotonic();
        if self.expires_at.elapsed_nanoseconds.clone() <= now.elapsed_nanoseconds.clone()
        {
            return None;
        }
        elapsed = self.expires_at.elapsed_nanoseconds.clone()
            - now.elapsed_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed, 1000000000.clone()),
                terrane_platform_time_mod(&elapsed, 1000000000.clone()),
            ),
        );
    }
    pub fn expired(&self) -> bool {
        return self.remaining().is_none();
    }
    pub fn terrane_static_at(
        target: MonotonicInstant,
    ) -> Result<Deadline, TerraneError> {
        if target.domain.clone() != terrane_platform_time_domain() {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    68 /* terrane-site: core/time.trn:105:13-105:45 */,
                )
            });
        }
        return Ok(Deadline::terrane_construct(target));
    }
}
pub fn discard_none(value: ()) {
    let _ = &value;
    return ();
}
#[derive(Clone)]
pub struct Tick {
    pub scheduled: MonotonicInstant,
    pub observed: MonotonicInstant,
    pub count: terrane_int_support::Int,
}
impl Tick {
    pub fn terrane_construct(
        scheduled_at: MonotonicInstant,
        observed_at: MonotonicInstant,
        expirations: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            scheduled: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            observed: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            count: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(scheduled_at, observed_at, expirations);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        scheduled_at: MonotonicInstant,
        observed_at: MonotonicInstant,
        expirations: terrane_int_support::Int,
    ) {
        {
            let __terrane_value: MonotonicInstant = scheduled_at;
            self.scheduled = __terrane_value;
        }
        {
            let __terrane_value: MonotonicInstant = observed_at;
            self.observed = __terrane_value;
        }
        {
            let __terrane_value: terrane_int_support::Int = expirations.clone();
            self.count = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct Ticker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub anchor: MonotonicInstant,
    pub period: Duration,
    pub next_index: terrane_int_support::Int,
}
impl Ticker {
    pub fn terrane_construct(interval: Duration) -> Self {
        let mut __terrane_constructed_value = Self {
            anchor: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            period: Duration::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            next_index: terrane_int_support::Int::from(1_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(interval);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, interval: Duration) {
        {
            let __terrane_value: MonotonicInstant = Clock::terrane_static_monotonic();
            self.anchor = __terrane_value;
        }
        {
            let __terrane_value: Duration = interval;
            self.period = __terrane_value;
        }
    }
    pub async fn next(&mut self) -> Tick {
        let period_total: terrane_int_support::Int;
        let scheduled_nanoseconds: terrane_int_support::Int;
        let observed: MonotonicInstant;
        let elapsed: terrane_int_support::Int;
        let mut observed_index: terrane_int_support::Int;
        let count: terrane_int_support::Int;
        let delivered: MonotonicInstant;
        period_total = self.period.total_nanoseconds.clone();
        scheduled_nanoseconds = self.anchor.elapsed_nanoseconds.clone()
            + period_total.clone() * self.next_index.clone();
        discard_none(
            __terrane_await(terrane_platform_time_sleep_until(scheduled_nanoseconds))
                .await,
        );
        observed = Clock::terrane_static_monotonic();
        elapsed = observed.elapsed_nanoseconds.clone()
            - self.anchor.elapsed_nanoseconds.clone();
        observed_index = terrane_platform_time_div(&elapsed, period_total.clone());
        if observed_index < self.next_index.clone() {
            observed_index = self.next_index.clone();
        }
        count = observed_index.clone() - self.next_index.clone()
            + terrane_int_support::Int::from(1_i128);
        delivered = MonotonicInstant::terrane_construct(
            self.anchor.domain.clone(),
            self.anchor.elapsed_nanoseconds.clone()
                + period_total.clone() * observed_index.clone(),
        );
        {
            let __terrane_value: terrane_int_support::Int = observed_index.clone()
                + terrane_int_support::Int::from(1_i128);
            self.next_index = __terrane_value;
        }
        return Tick::terrane_construct(delivered, observed, count.clone());
    }
    pub async fn __terrane_shared_async_next(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
    ) -> Tick {
        let period_total: terrane_int_support::Int;
        let scheduled_nanoseconds: terrane_int_support::Int;
        let observed: MonotonicInstant;
        let elapsed: terrane_int_support::Int;
        let mut observed_index: terrane_int_support::Int;
        let count: terrane_int_support::Int;
        let delivered: MonotonicInstant;
        period_total = {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .period
                .clone();
            __terrane_snapshot
        }
            .total_nanoseconds
            .clone();
        scheduled_nanoseconds = {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .anchor
                .clone();
            __terrane_snapshot
        }
            .elapsed_nanoseconds
            .clone()
            + period_total.clone()
                * {
                    let __terrane_snapshot = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .next_index
                        .clone();
                    __terrane_snapshot
                };
        discard_none(
            __terrane_await(terrane_platform_time_sleep_until(scheduled_nanoseconds))
                .await,
        );
        observed = Clock::terrane_static_monotonic();
        elapsed = observed.elapsed_nanoseconds.clone()
            - {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .anchor
                    .clone();
                __terrane_snapshot
            }
                .elapsed_nanoseconds
                .clone();
        observed_index = terrane_platform_time_div(&elapsed, period_total.clone());
        if observed_index
            < {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .next_index
                    .clone();
                __terrane_snapshot
            }
        {
            observed_index = {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .next_index
                    .clone();
                __terrane_snapshot
            };
        }
        count = observed_index.clone()
            - {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .next_index
                    .clone();
                __terrane_snapshot
            } + terrane_int_support::Int::from(1_i128);
        delivered = MonotonicInstant::terrane_construct(
            {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .anchor
                    .clone();
                __terrane_snapshot
            }
                .domain
                .clone(),
            {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .anchor
                    .clone();
                __terrane_snapshot
            }
                .elapsed_nanoseconds
                .clone() + period_total.clone() * observed_index.clone(),
        );
        {
            let __terrane_value: terrane_int_support::Int = observed_index.clone()
                + terrane_int_support::Int::from(1_i128);
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .next_index = __terrane_value;
        }
        return Tick::terrane_construct(delivered, observed, count.clone());
    }
    pub fn destruct(&mut self) {
        discard_none(());
    }
}
impl Drop for Ticker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct Clock {}
impl Clock {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_wall() -> Instant {
        let raw: TerranePlatformResult;
        raw = terrane_platform_time_wall();
        return Instant::terrane_construct(
            terrane_platform_time_wall_seconds(&raw),
            terrane_platform_time_wall_nanoseconds(&raw),
        );
    }
    pub fn terrane_static_monotonic() -> MonotonicInstant {
        return MonotonicInstant::terrane_construct(
            terrane_platform_time_domain(),
            terrane_platform_time_monotonic(),
        );
    }
    pub async fn terrane_static_sleep(elapsed: Duration) {
        let target: terrane_int_support::Int;
        target = terrane_platform_time_monotonic() + elapsed.total_nanoseconds.clone();
        return __terrane_await(terrane_platform_time_sleep_until(target)).await;
    }
    pub async fn terrane_static_sleep_until(
        target: MonotonicInstant,
    ) -> Result<(), TerraneError> {
        if target.domain.clone() != terrane_platform_time_domain() {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    69 /* terrane-site: core/time.trn:161:13-161:45 */,
                )
            });
        }
        return Ok(
            __terrane_await(
                    terrane_platform_time_sleep_until(target.elapsed_nanoseconds),
                )
                .await,
        );
    }
    pub fn terrane_static_deadline(elapsed: Duration) -> Deadline {
        let now: MonotonicInstant;
        let target: MonotonicInstant;
        now = Clock::terrane_static_monotonic();
        target = MonotonicInstant::terrane_construct(
            now.domain.clone(),
            now.elapsed_nanoseconds.clone() + elapsed.total_nanoseconds.clone(),
        );
        return Deadline::terrane_construct(target);
    }
    pub fn terrane_static_interval(period: Duration) -> Result<Ticker, TerraneError> {
        if period.total_nanoseconds.clone() == terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,
                    70 /* terrane-site: core/time.trn:172:13-172:45 */,
                )
            });
        }
        return Ok(Ticker::terrane_construct(period));
    }
}
// Source: core/annotations.trn
// Namespace: core/annotations
#[derive(Clone)]
pub struct Annotation {
    pub targets: terrane_collection_support::List<AnnotationTarget>,
    pub repeatable: bool,
}
impl Annotation {
    pub fn terrane_construct() -> Self {
        Self {
            targets: terrane_collection_support::List::<
                AnnotationTarget,
            >::new(Vec::new()),
            repeatable: false,
        }
    }
}
#[derive(Clone)]
pub enum AnnotationTarget {
    Class,
    Callable,
    Parameter,
    Field,
}
#[derive(Clone)]
pub struct Descriptor {}
impl Descriptor {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
