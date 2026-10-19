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
mod __terrane_error_registry {
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 4] = [
        "/core/testing::test-failure",
        "/core/testing::test-skip",
        "/core/testing::test-infrastructure-failure",
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
    pub static FILES: [&str; 5] = [
        "tests/unit/surface.trn",
        "core/testing.trn",
        "core/testing_process.trn",
        "core/process.trn",
        "core/time.trn",
    ];
    pub static FUNCTIONS: [&str; 47] = [
        "/native-testing-surface::expected-error",
        "/native-testing-surface::test-surface",
        "/core/testing::fail",
        "/core/testing::skip",
        "/core/testing::assert",
        "/core/testing::deny",
        "/core/testing::fail-comparison",
        "/core/testing::fail-values",
        "/core/testing::fail-near",
        "/core/testing::assert-equal-int",
        "/core/testing::assert-not-equal-int",
        "/core/testing::assert-equal-string",
        "/core/testing::assert-not-equal-string",
        "/core/testing::assert-equal-bool",
        "/core/testing::assert-not-equal-bool",
        "/core/testing::assert-equal-float64",
        "/core/testing::assert-not-equal-float64",
        "/core/testing::assert-equal-bytes",
        "/core/testing::assert-not-equal-bytes",
        "/core/testing::assert-present-int",
        "/core/testing::assert-none-int",
        "/core/testing::assert-present-string",
        "/core/testing::assert-none-string",
        "/core/testing::assert-present-bool",
        "/core/testing::assert-none-bool",
        "/core/testing::assert-present-float64",
        "/core/testing::assert-none-float64",
        "/core/testing::assert-present-bytes",
        "/core/testing::assert-none-bytes",
        "/core/testing::assert-near",
        "/core/testing::assert-throws",
        "/core/testing::test-arguments",
        "/core/testing::test-deadline",
        "/core/testing::advance-time",
        "/core/testing/process::run-process",
        "/core/process::arguments",
        "/core/process::environment",
        "/core/process::parse-command-line",
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
    pub static SITES: [Site; 61] = [
        /* terrane-site-row: site 0: /native-testing-surface::expected-error (tests/unit/surface.trn:6:5-6:21) */
        { Site { function: 0, file: 0, line: 6, column: 5, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 1: /native-testing-surface::test-surface (tests/unit/surface.trn:10:5-10:17) */
        { Site { function: 1, file: 0, line: 10, column: 5, end_line: 10, end_column: 17 } },
        /* terrane-site-row: site 2: /native-testing-surface::test-surface (tests/unit/surface.trn:11:5-11:16) */
        { Site { function: 1, file: 0, line: 11, column: 5, end_line: 11, end_column: 16 } },
        /* terrane-site-row: site 3: /native-testing-surface::test-surface (tests/unit/surface.trn:12:5-12:27) */
        { Site { function: 1, file: 0, line: 12, column: 5, end_line: 12, end_column: 27 } },
        /* terrane-site-row: site 4: /native-testing-surface::test-surface (tests/unit/surface.trn:13:5-13:45) */
        { Site { function: 1, file: 0, line: 13, column: 5, end_line: 13, end_column: 45 } },
        /* terrane-site-row: site 5: /native-testing-surface::test-surface (tests/unit/surface.trn:14:5-14:37) */
        { Site { function: 1, file: 0, line: 14, column: 5, end_line: 14, end_column: 37 } },
        /* terrane-site-row: site 6: /native-testing-surface::test-surface (tests/unit/surface.trn:15:5-15:29) */
        { Site { function: 1, file: 0, line: 15, column: 5, end_line: 15, end_column: 29 } },
        /* terrane-site-row: site 7: /native-testing-surface::test-surface (tests/unit/surface.trn:16:5-16:31) */
        { Site { function: 1, file: 0, line: 16, column: 5, end_line: 16, end_column: 31 } },
        /* terrane-site-row: site 8: /native-testing-surface::test-surface (tests/unit/surface.trn:17:5-17:57) */
        { Site { function: 1, file: 0, line: 17, column: 5, end_line: 17, end_column: 57 } },
        /* terrane-site-row: site 9: /native-testing-surface::test-surface (tests/unit/surface.trn:19:5-19:54) */
        { Site { function: 1, file: 0, line: 19, column: 5, end_line: 19, end_column: 54 } },
        /* terrane-site-row: site 10: /core/testing::fail (core/testing.trn:43:5-43:41) */
        { Site { function: 2, file: 1, line: 43, column: 5, end_line: 43, end_column: 41 } },
        /* terrane-site-row: site 11: /core/testing::skip (core/testing.trn:46:5-46:37) */
        { Site { function: 3, file: 1, line: 46, column: 5, end_line: 46, end_column: 37 } },
        /* terrane-site-row: site 12: /core/testing::assert (core/testing.trn:50:9-50:36) */
        { Site { function: 4, file: 1, line: 50, column: 9, end_line: 50, end_column: 36 } },
        /* terrane-site-row: site 13: /core/testing::deny (core/testing.trn:55:9-55:42) */
        { Site { function: 5, file: 1, line: 55, column: 9, end_line: 55, end_column: 42 } },
        /* terrane-site-row: site 14: /core/testing::fail-comparison (core/testing.trn:64:5-64:18) */
        { Site { function: 6, file: 1, line: 64, column: 5, end_line: 64, end_column: 18 } },
        /* terrane-site-row: site 15: /core/testing::fail-values (core/testing.trn:67:5-67:75) */
        { Site { function: 7, file: 1, line: 67, column: 5, end_line: 67, end_column: 75 } },
        /* terrane-site-row: site 16: /core/testing::fail-near (core/testing.trn:77:5-77:18) */
        { Site { function: 8, file: 1, line: 77, column: 5, end_line: 77, end_column: 18 } },
        /* terrane-site-row: site 17: /core/testing::assert-equal-int (core/testing.trn:81:9-81:115) */
        { Site { function: 9, file: 1, line: 81, column: 9, end_line: 81, end_column: 115 } },
        /* terrane-site-row: site 18: /core/testing::assert-not-equal-int (core/testing.trn:86:9-86:127) */
        { Site { function: 10, file: 1, line: 86, column: 9, end_line: 86, end_column: 127 } },
        /* terrane-site-row: site 19: /core/testing::assert-equal-string (core/testing.trn:91:9-91:66) */
        { Site { function: 11, file: 1, line: 91, column: 9, end_line: 91, end_column: 66 } },
        /* terrane-site-row: site 20: /core/testing::assert-not-equal-string (core/testing.trn:96:9-96:78) */
        { Site { function: 12, file: 1, line: 96, column: 9, end_line: 96, end_column: 78 } },
        /* terrane-site-row: site 21: /core/testing::assert-equal-bool (core/testing.trn:101:9-101:117) */
        { Site { function: 13, file: 1, line: 101, column: 9, end_line: 101, end_column: 117 } },
        /* terrane-site-row: site 22: /core/testing::assert-not-equal-bool (core/testing.trn:106:9-106:129) */
        { Site { function: 14, file: 1, line: 106, column: 9, end_line: 106, end_column: 129 } },
        /* terrane-site-row: site 23: /core/testing::assert-equal-float64 (core/testing.trn:111:9-111:124) */
        { Site { function: 15, file: 1, line: 111, column: 9, end_line: 111, end_column: 124 } },
        /* terrane-site-row: site 24: /core/testing::assert-not-equal-float64 (core/testing.trn:116:9-116:136) */
        { Site { function: 16, file: 1, line: 116, column: 9, end_line: 116, end_column: 136 } },
        /* terrane-site-row: site 25: /core/testing::assert-equal-bytes (core/testing.trn:121:9-121:116) */
        { Site { function: 17, file: 1, line: 121, column: 9, end_line: 121, end_column: 116 } },
        /* terrane-site-row: site 26: /core/testing::assert-not-equal-bytes (core/testing.trn:126:9-126:128) */
        { Site { function: 18, file: 1, line: 126, column: 9, end_line: 126, end_column: 128 } },
        /* terrane-site-row: site 27: /core/testing::assert-present-int (core/testing.trn:131:9-131:43) */
        { Site { function: 19, file: 1, line: 131, column: 9, end_line: 131, end_column: 43 } },
        /* terrane-site-row: site 28: /core/testing::assert-none-int (core/testing.trn:136:9-136:36) */
        { Site { function: 20, file: 1, line: 136, column: 9, end_line: 136, end_column: 36 } },
        /* terrane-site-row: site 29: /core/testing::assert-present-string (core/testing.trn:141:9-141:42) */
        { Site { function: 21, file: 1, line: 141, column: 9, end_line: 141, end_column: 42 } },
        /* terrane-site-row: site 30: /core/testing::assert-none-string (core/testing.trn:146:9-146:35) */
        { Site { function: 22, file: 1, line: 146, column: 9, end_line: 146, end_column: 35 } },
        /* terrane-site-row: site 31: /core/testing::assert-present-bool (core/testing.trn:151:9-151:43) */
        { Site { function: 23, file: 1, line: 151, column: 9, end_line: 151, end_column: 43 } },
        /* terrane-site-row: site 32: /core/testing::assert-none-bool (core/testing.trn:156:9-156:36) */
        { Site { function: 24, file: 1, line: 156, column: 9, end_line: 156, end_column: 36 } },
        /* terrane-site-row: site 33: /core/testing::assert-present-float64 (core/testing.trn:161:9-161:50) */
        { Site { function: 25, file: 1, line: 161, column: 9, end_line: 161, end_column: 50 } },
        /* terrane-site-row: site 34: /core/testing::assert-none-float64 (core/testing.trn:166:9-166:43) */
        { Site { function: 26, file: 1, line: 166, column: 9, end_line: 166, end_column: 43 } },
        /* terrane-site-row: site 35: /core/testing::assert-present-bytes (core/testing.trn:171:9-171:39) */
        { Site { function: 27, file: 1, line: 171, column: 9, end_line: 171, end_column: 39 } },
        /* terrane-site-row: site 36: /core/testing::assert-none-bytes (core/testing.trn:176:9-176:34) */
        { Site { function: 28, file: 1, line: 176, column: 9, end_line: 176, end_column: 34 } },
        /* terrane-site-row: site 37: /core/testing::assert-near (core/testing.trn:181:9-181:111) */
        { Site { function: 29, file: 1, line: 181, column: 9, end_line: 181, end_column: 111 } },
        /* terrane-site-row: site 38: /core/testing::assert-near (core/testing.trn:186:9-186:47) */
        { Site { function: 29, file: 1, line: 186, column: 9, end_line: 186, end_column: 47 } },
        /* terrane-site-row: site 39: /core/testing::assert-throws (core/testing.trn:191:9-191:19) */
        { Site { function: 30, file: 1, line: 191, column: 9, end_line: 191, end_column: 19 } },
        /* terrane-site-row: site 40: /core/testing::assert-throws (core/testing.trn:195:9-195:113) */
        { Site { function: 30, file: 1, line: 195, column: 9, end_line: 195, end_column: 113 } },
        /* terrane-site-row: site 41: /core/testing::assert-throws (core/testing.trn:196:5-196:70) */
        { Site { function: 30, file: 1, line: 196, column: 5, end_line: 196, end_column: 70 } },
        /* terrane-site-row: site 42: /core/testing::test-arguments (core/testing.trn:210:28-210:43) */
        { Site { function: 31, file: 1, line: 210, column: 28, end_line: 210, end_column: 43 } },
        /* terrane-site-row: site 43: /core/testing::test-deadline (core/testing.trn:215:12-215:68) */
        { Site { function: 32, file: 1, line: 215, column: 12, end_line: 215, end_column: 68 } },
        /* terrane-site-row: site 44: /core/testing::advance-time (core/testing.trn:235:9-235:49) */
        { Site { function: 33, file: 1, line: 235, column: 9, end_line: 235, end_column: 49 } },
        /* terrane-site-row: site 45: /core/testing::advance-time (core/testing.trn:237:9-237:69) */
        { Site { function: 33, file: 1, line: 237, column: 9, end_line: 237, end_column: 69 } },
        /* terrane-site-row: site 46: /core/testing/process::run-process (core/testing_process.trn:47:9-47:84) */
        { Site { function: 34, file: 2, line: 47, column: 9, end_line: 47, end_column: 84 } },
        /* terrane-site-row: site 47: /core/process::arguments (core/process.trn:45:49-45:63) */
        { Site { function: 35, file: 3, line: 45, column: 49, end_line: 45, end_column: 63 } },
        /* terrane-site-row: site 48: /core/process::environment (core/process.trn:54:40-54:54) */
        { Site { function: 36, file: 3, line: 54, column: 40, end_line: 54, end_column: 54 } },
        /* terrane-site-row: site 49: /core/process::environment (core/process.trn:55:41-55:59) */
        { Site { function: 36, file: 3, line: 55, column: 41, end_line: 55, end_column: 59 } },
        /* terrane-site-row: site 50: /core/process::parse-command-line (core/process.trn:90:20-90:35) */
        { Site { function: 37, file: 3, line: 90, column: 20, end_line: 90, end_column: 35 } },
        /* terrane-site-row: site 51: /core/process::parse-command-line (core/process.trn:105:43-105:62) */
        { Site { function: 37, file: 3, line: 105, column: 43, end_line: 105, end_column: 62 } },
        /* terrane-site-row: site 52: /core/time::multiply (core/time.trn:62:13-62:45) */
        { Site { function: 38, file: 4, line: 62, column: 13, end_line: 62, end_column: 45 } },
        /* terrane-site-row: site 53: /core/time::seconds (core/time.trn:38:13-38:45) */
        { Site { function: 39, file: 4, line: 38, column: 13, end_line: 38, end_column: 45 } },
        /* terrane-site-row: site 54: /core/time::milliseconds (core/time.trn:43:13-43:45) */
        { Site { function: 40, file: 4, line: 43, column: 13, end_line: 43, end_column: 45 } },
        /* terrane-site-row: site 55: /core/time::microseconds (core/time.trn:48:13-48:45) */
        { Site { function: 41, file: 4, line: 48, column: 13, end_line: 48, end_column: 45 } },
        /* terrane-site-row: site 56: /core/time::nanoseconds (core/time.trn:53:13-53:45) */
        { Site { function: 42, file: 4, line: 53, column: 13, end_line: 53, end_column: 45 } },
        /* terrane-site-row: site 57: /core/time::duration-until (core/time.trn:76:13-76:45) */
        { Site { function: 43, file: 4, line: 76, column: 13, end_line: 76, end_column: 45 } },
        /* terrane-site-row: site 58: /core/time::at (core/time.trn:105:13-105:45) */
        { Site { function: 44, file: 4, line: 105, column: 13, end_line: 105, end_column: 45 } },
        /* terrane-site-row: site 59: /core/time::sleep-until (core/time.trn:161:13-161:45) */
        { Site { function: 45, file: 4, line: 161, column: 13, end_line: 161, end_column: 45 } },
        /* terrane-site-row: site 60: /core/time::interval (core/time.trn:172:13-172:45) */
        { Site { function: 46, file: 4, line: 172, column: 13, end_line: 172, end_column: 45 } },
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
    reason = "async entry support is emitted for libraries without an entrypoint"
)]
async fn __terrane_async_entry<F: Future>(future: F) -> F::Output {
    future.await
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_domain() -> terrane_int_support::Int {
    terrane_int_support::Int::from(2_i128)
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_monotonic() -> terrane_int_support::Int {
    terrane_int_support::Int::from_u128(
        u128::from(TERRANE_TEST_TIME_NANOS.load(std::sync::atomic::Ordering::Acquire)),
    )
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_deadline_expired(target: &terrane_int_support::Int) -> bool {
    terrane_time_monotonic() >= target.clone()
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
async fn terrane_time_sleep_until(target: terrane_int_support::Int) {
    let target = terrane_int_support::checked_coerce::<u64>(&target)
        .expect("controlled test clock target exceeds the host clock range");
    TERRANE_TEST_TIME_NANOS.fetch_max(target, std::sync::atomic::Ordering::AcqRel);
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_sleep_after(
    elapsed: terrane_int_support::Int,
) -> impl Future<Output = ()> {
    let target = terrane_time_monotonic() + elapsed;
    async move {
        terrane_time_sleep_until(target).await;
    }
}
pub fn terrane_unhex(text: &str) -> Vec<u8> {
    fn digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    text.as_bytes()
        .chunks_exact(2)
        .filter_map(|pair| Some(digit(pair[0])? << 4 | digit(pair[1])?))
        .collect()
}
fn terrane_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}
pub fn terrane_platform_value(value: std::ffi::OsString) -> String {
    terrane_platform_support::platform_value(value)
}
fn terrane_platform_value_from_text(value: &str) -> String {
    format!("text:{value}")
}
fn terrane_platform_value_from_bytes(value: &[u8]) -> String {
    format!("raw:{}", terrane_hex(value))
}
pub fn terrane_platform_value_is_text(value: &str) -> bool {
    value.starts_with("text:")
}
pub fn terrane_platform_value_text(value: &str) -> String {
    value.strip_prefix("text:").unwrap_or("").to_owned()
}
pub fn terrane_platform_value_bytes(value: &str) -> Vec<u8> {
    value.strip_prefix("raw:").map(terrane_unhex).unwrap_or_default()
}
pub fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}
pub fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [
            terrane_platform_value(name),
            terrane_platform_value(value),
        ])
        .collect()
}
pub fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}
fn terrane_decode_platform_value(value: String) -> std::ffi::OsString {
    if let Some(text) = value.strip_prefix("text:") {
        return std::ffi::OsString::from(text);
    }
    let bytes = value.strip_prefix("raw:").map(terrane_unhex).unwrap_or_default();
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt as _;
        std::ffi::OsString::from_vec(bytes)
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt as _;
        let wide = bytes
            .chunks_exact(2)
            .map(|pair| u16::from_le_bytes([pair[0], pair[1]]));
        std::ffi::OsString::from_wide(&wide.collect::<Vec<_>>())
    }
}
const TERRANE_TEST_PROCESS_CAPTURE_LIMIT: usize = 1024 * 1024;
const TERRANE_TEST_PROCESS_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(
    5,
);
const TERRANE_TEST_PROCESS_OUTPUT_CLOSE_TIMEOUT: std::time::Duration = std::time::Duration::from_millis(
    250,
);
struct TerraneTestCapturedOutput {
    bytes: Vec<u8>,
    truncated: bool,
}
fn terrane_read_process_output(
    mut stream: impl std::io::Read,
) -> TerraneTestCapturedOutput {
    let mut output = TerraneTestCapturedOutput {
        bytes: Vec::new(),
        truncated: false,
    };
    let mut buffer = [0_u8; 8192];
    while let Ok(read) = stream.read(&mut buffer) {
        if read == 0 {
            break;
        }
        let available = TERRANE_TEST_PROCESS_CAPTURE_LIMIT
            .saturating_sub(output.bytes.len());
        output.bytes.extend_from_slice(&buffer[..read.min(available)]);
        output.truncated |= read > available;
    }
    output
}
fn terrane_spawn_process_reader(
    stream: impl std::io::Read + Send + 'static,
) -> std::sync::mpsc::Receiver<TerraneTestCapturedOutput> {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let _ = sender.send(terrane_read_process_output(stream));
    });
    receiver
}
fn terrane_wait_fixture_child(
    child: &mut std::process::Child,
    timeout: std::time::Duration,
) -> Result<(std::process::ExitStatus, bool), String> {
    let started = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok((status, false)),
            Ok(None) if started.elapsed() < timeout => {
                std::thread::sleep(TERRANE_TEST_PROCESS_POLL_INTERVAL);
            }
            Ok(None) => {
                child
                    .kill()
                    .map_err(|error| {
                        format!("cannot terminate timed-out fixture: {error}")
                    })?;
                return child
                    .wait()
                    .map(|status| (status, true))
                    .map_err(|error| format!("cannot reap timed-out fixture: {error}"));
            }
            Err(error) => return Err(format!("cannot observe process fixture: {error}")),
        }
    }
}
pub fn terrane_test_spawn(
    artifact: String,
    arguments: terrane_collection_support::List<String>,
    environment: terrane_collection_support::List<String>,
    input: Vec<u8>,
    deadline_milliseconds: terrane_int_support::Int,
) -> TerranePlatformResult {
    use std::io::Write as _;
    use std::process::Stdio;
    use std::time::Duration;
    let Some(timeout) = terrane_int_support::checked_coerce::<
        u64,
    >(&deadline_milliseconds) else {
        return TerranePlatformResult::error(
            "process deadline does not fit milliseconds",
        );
    };
    let environment = environment.into_iter().collect::<Vec<_>>();
    if !environment.len().is_multiple_of(2) {
        return TerranePlatformResult::error("process fixture environment is not paired");
    }
    let mut command = std::process::Command::new(artifact);
    command
        .args(arguments.into_iter().map(terrane_decode_platform_value))
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for pair in environment.chunks_exact(2) {
        command
            .env(
                terrane_decode_platform_value(pair[0].clone()),
                terrane_decode_platform_value(pair[1].clone()),
            );
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return TerranePlatformResult::error(
                format!("cannot spawn fixture: {error}"),
            );
        }
    };
    let stdout = terrane_spawn_process_reader(
        child.stdout.take().expect("captured child stdout"),
    );
    let stderr = terrane_spawn_process_reader(
        child.stderr.take().expect("captured child stderr"),
    );
    if let Some(mut stdin) = child.stdin.take() {
        std::thread::spawn(move || {
            let _ = stdin.write_all(&input);
        });
    }
    let (status, deadline_exceeded) = match terrane_wait_fixture_child(
        &mut child,
        Duration::from_millis(timeout),
    ) {
        Ok(result) => result,
        Err(error) => return TerranePlatformResult::error(error),
    };
    let output = match stdout.recv_timeout(TERRANE_TEST_PROCESS_OUTPUT_CLOSE_TIMEOUT) {
        Ok(output) => output,
        Err(_) => {
            return TerranePlatformResult::error(
                "fixture stdout did not close after exit",
            );
        }
    };
    let error = match stderr.recv_timeout(TERRANE_TEST_PROCESS_OUTPUT_CLOSE_TIMEOUT) {
        Ok(error) => error,
        Err(_) => {
            return TerranePlatformResult::error(
                "fixture stderr did not close after exit",
            );
        }
    };
    TerranePlatformResult {
        failed: false,
        deadline_exceeded,
        data: output.bytes,
        entries: vec![
            format!("raw:{}", terrane_hex(&error.bytes)), output.truncated.to_string(),
            error.truncated.to_string(),
        ],
        number: status.code().map_or(-1, i128::from),
        flag: status.code().is_none(),
        ..TerranePlatformResult::default()
    }
}
pub fn terrane_test_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
pub fn terrane_test_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
pub fn terrane_test_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
pub fn terrane_test_result_exit_code(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
pub fn terrane_test_result_crashed(result: &TerranePlatformResult) -> bool {
    result.flag
}
pub fn terrane_test_result_stdout(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}
pub fn terrane_test_result_stderr(result: &TerranePlatformResult) -> Vec<u8> {
    result
        .entries
        .first()
        .and_then(|value| value.strip_prefix("raw:"))
        .map(terrane_unhex)
        .unwrap_or_default()
}
pub fn terrane_test_result_stdout_truncated(result: &TerranePlatformResult) -> bool {
    result.entries.get(1).is_some_and(|value| value == "true")
}
pub fn terrane_test_result_stderr_truncated(result: &TerranePlatformResult) -> bool {
    result.entries.get(2).is_some_and(|value| value == "true")
}
fn terrane_test_render_int(value: terrane_int_support::Int) -> String {
    value.to_string()
}
fn terrane_test_render_float64(value: f64) -> String {
    value.to_string()
}
fn terrane_test_render_bytes(value: Vec<u8>) -> String {
    format!("0x{}", terrane_hex(&value))
}
fn terrane_test_render_bool(value: bool) -> String {
    value.to_string()
}
fn terrane_test_deadline_nanoseconds() -> terrane_int_support::Int {
    let value = std::env::var("TERRANE_TEST_DEADLINE_NANOSECONDS")
        .ok()
        .and_then(|value| value.parse::<u128>().ok())
        .unwrap_or_default();
    terrane_int_support::Int::from_u128(value)
}
static TERRANE_TEST_TIME_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(
    0,
);
fn terrane_test_time_advance(nanoseconds: terrane_int_support::Int) -> bool {
    let Some(nanoseconds) = terrane_int_support::checked_coerce::<u64>(&nanoseconds)
    else {
        return false;
    };
    if TERRANE_TEST_TIME_NANOS
        .try_update(
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            |current| current.checked_add(nanoseconds),
        )
        .is_err()
    {
        return false;
    }
    true
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
// Source: core/testing.trn
// Namespace: core/testing
pub trait TestValueProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn TestValueProtocol>;
    fn separate_box(&self) -> Box<dyn TestValueProtocol>;
    fn render(&self) -> String;
}
impl Clone for Box<dyn TestValueProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct TestValue(Box<dyn TestValueProtocol>);
impl Clone for TestValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl TestValue {
    pub fn render(&self) -> String {
        self.0.render()
    }
}
#[derive(Clone)]
pub struct TestFailure {
    pub message: String,
    pub assertion_source: String,
    pub details: terrane_collection_support::List<String>,
}
impl TestFailure {
    pub fn terrane_construct(failure_message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            message: String::from(""),
            assertion_source: String::from(""),
            details: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        __terrane_constructed_value.construct(failure_message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failure_message: String) {
        {
            let __terrane_value = failure_message;
            self.message = __terrane_value;
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct TestSkip {
    pub message: String,
    pub assertion_source: String,
    pub details: terrane_collection_support::List<String>,
}
impl TestSkip {
    pub fn terrane_construct(reason: String) -> Self {
        let mut __terrane_constructed_value = Self {
            message: String::from(""),
            assertion_source: String::from(""),
            details: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        __terrane_constructed_value.construct(reason);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, reason: String) {
        {
            let __terrane_value = reason;
            self.message = __terrane_value;
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct TestInfrastructureFailure {
    pub message: String,
}
impl TestInfrastructureFailure {
    pub fn terrane_construct(failure_message: String) -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
        __terrane_constructed_value.construct(failure_message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failure_message: String) {
        {
            let __terrane_value = failure_message;
            self.message = __terrane_value;
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
pub fn fail(message: String) -> Result<(), TerraneError> {
    return Err({
        let value = TestFailure::terrane_construct(message);
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        let details = value.details.clone().into_iter().collect();
        TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                10 /* terrane-site: core/testing.trn:43:5-43:41 */,
            )
            .with_structured_details(details)
    });
}
pub fn skip(reason: String) -> Result<(), TerraneError> {
    return Err({
        let value = TestSkip::terrane_construct(reason);
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        TerraneError::raised_with_rendered_message(
            TerraneErrorKind::Custom(DescriptorId(1)),
            message,
            rendered,
            11 /* terrane-site: core/testing.trn:46:5-46:37 */,
        )
    });
}
pub fn assert(condition: bool) -> Result<(), TerraneError> {
    if !condition {
        __terrane_traced_err(
            fail(String::from("assertion was false")),
            12 /* terrane-site: core/testing.trn:50:9-50:36 */,
        )?;
    }
    return Ok(());
}
pub fn deny(condition: bool) -> Result<(), TerraneError> {
    if condition {
        __terrane_traced_err(
            fail(String::from("denied condition was true")),
            13 /* terrane-site: core/testing.trn:55:9-55:42 */,
        )?;
    }
    return Ok(());
}
pub fn fail_comparison(
    failure_message: String,
    actual: String,
    expected: String,
) -> Result<(), TerraneError> {
    let mut details: terrane_collection_support::List<String>;
    let mut failure: TestFailure;
    details = terrane_collection_support::List::<String>::new(Vec::new());
    {
        let __terrane_collection_argument_0 = {
            let mut __terrane_text = String::with_capacity(8);
            __terrane_text.push_str("actual: ");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &actual,
                &mut __terrane_text,
            );
            __terrane_text
        };
        details.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = {
            let mut __terrane_text = String::with_capacity(10);
            __terrane_text.push_str("expected: ");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &expected,
                &mut __terrane_text,
            );
            __terrane_text
        };
        details.append(__terrane_collection_argument_0);
    };
    failure = TestFailure::terrane_construct(failure_message);
    {
        let __terrane_value = details;
        failure.details = __terrane_value;
    }
    return Err({
        let value = failure;
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        let details = value.details.clone().into_iter().collect();
        TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                14 /* terrane-site: core/testing.trn:64:5-64:18 */,
            )
            .with_structured_details(details)
    });
}
pub fn fail_values(
    failure_message: String,
    actual: TestValue,
    expected: TestValue,
) -> Result<(), TerraneError> {
    __terrane_traced_err(
        fail_comparison(failure_message, actual.render(), expected.render()),
        15 /* terrane-site: core/testing.trn:67:5-67:75 */,
    )?;
    return Ok(());
}
pub fn fail_near(
    actual: f64,
    expected: f64,
    tolerance: f64,
) -> Result<(), TerraneError> {
    let mut details: terrane_collection_support::List<String>;
    let mut failure: TestFailure;
    details = terrane_collection_support::List::<String>::new(Vec::new());
    {
        let __terrane_collection_argument_0 = {
            let mut __terrane_text = String::with_capacity(8);
            __terrane_text.push_str("actual: ");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &terrane_test_render_float64(actual),
                &mut __terrane_text,
            );
            __terrane_text
        };
        details.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = {
            let mut __terrane_text = String::with_capacity(10);
            __terrane_text.push_str("expected: ");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &terrane_test_render_float64(expected),
                &mut __terrane_text,
            );
            __terrane_text
        };
        details.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = {
            let mut __terrane_text = String::with_capacity(11);
            __terrane_text.push_str("tolerance: ");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &terrane_test_render_float64(tolerance),
                &mut __terrane_text,
            );
            __terrane_text
        };
        details.append(__terrane_collection_argument_0);
    };
    failure = TestFailure::terrane_construct(
        String::from("floating values differ beyond tolerance"),
    );
    {
        let __terrane_value = details;
        failure.details = __terrane_value;
    }
    return Err({
        let value = failure;
        let rendered = value.render();
        let original = &value.message;
        let (message, rendered) = if &rendered == original {
            (rendered, None)
        } else {
            (original.clone(), Some(rendered))
        };
        let details = value.details.clone().into_iter().collect();
        TerraneError::raised_with_rendered_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                message,
                rendered,
                16 /* terrane-site: core/testing.trn:77:5-77:18 */,
            )
            .with_structured_details(details)
    });
}
pub fn assert_equal_int(
    actual: terrane_int_support::Int,
    expected: terrane_int_support::Int,
) -> Result<(), TerraneError> {
    if actual.clone() != expected.clone() {
        __terrane_traced_err(
            fail_comparison(
                String::from("integer values differ"),
                terrane_test_render_int(actual.clone()),
                terrane_test_render_int(expected.clone()),
            ),
            17 /* terrane-site: core/testing.trn:81:9-81:115 */,
        )?;
    }
    return Ok(());
}
pub fn assert_not_equal_int(
    actual: terrane_int_support::Int,
    expected: terrane_int_support::Int,
) -> Result<(), TerraneError> {
    if actual.clone() == expected.clone() {
        __terrane_traced_err(
            fail_comparison(
                String::from("integer values unexpectedly match"),
                terrane_test_render_int(actual.clone()),
                terrane_test_render_int(expected.clone()),
            ),
            18 /* terrane-site: core/testing.trn:86:9-86:127 */,
        )?;
    }
    return Ok(());
}
pub fn assert_equal_string(
    actual: String,
    expected: String,
) -> Result<(), TerraneError> {
    if actual.as_str() != expected.as_str() {
        __terrane_traced_err(
            fail_comparison(String::from("string values differ"), actual, expected),
            19 /* terrane-site: core/testing.trn:91:9-91:66 */,
        )?;
    }
    return Ok(());
}
pub fn assert_not_equal_string(
    actual: String,
    expected: String,
) -> Result<(), TerraneError> {
    if actual.as_str() == expected.as_str() {
        __terrane_traced_err(
            fail_comparison(
                String::from("string values unexpectedly match"),
                actual,
                expected,
            ),
            20 /* terrane-site: core/testing.trn:96:9-96:78 */,
        )?;
    }
    return Ok(());
}
pub fn assert_equal_bool(actual: bool, expected: bool) -> Result<(), TerraneError> {
    if actual != expected {
        __terrane_traced_err(
            fail_comparison(
                String::from("boolean values differ"),
                terrane_test_render_bool(actual),
                terrane_test_render_bool(expected),
            ),
            21 /* terrane-site: core/testing.trn:101:9-101:117 */,
        )?;
    }
    return Ok(());
}
pub fn assert_not_equal_bool(actual: bool, expected: bool) -> Result<(), TerraneError> {
    if actual == expected {
        __terrane_traced_err(
            fail_comparison(
                String::from("boolean values unexpectedly match"),
                terrane_test_render_bool(actual),
                terrane_test_render_bool(expected),
            ),
            22 /* terrane-site: core/testing.trn:106:9-106:129 */,
        )?;
    }
    return Ok(());
}
pub fn assert_equal_float64(actual: f64, expected: f64) -> Result<(), TerraneError> {
    if actual != expected {
        __terrane_traced_err(
            fail_comparison(
                String::from("floating values differ"),
                terrane_test_render_float64(actual),
                terrane_test_render_float64(expected),
            ),
            23 /* terrane-site: core/testing.trn:111:9-111:124 */,
        )?;
    }
    return Ok(());
}
pub fn assert_not_equal_float64(actual: f64, expected: f64) -> Result<(), TerraneError> {
    if actual == expected {
        __terrane_traced_err(
            fail_comparison(
                String::from("floating values unexpectedly match"),
                terrane_test_render_float64(actual),
                terrane_test_render_float64(expected),
            ),
            24 /* terrane-site: core/testing.trn:116:9-116:136 */,
        )?;
    }
    return Ok(());
}
pub fn assert_equal_bytes(
    actual: Vec<u8>,
    expected: Vec<u8>,
) -> Result<(), TerraneError> {
    if actual != expected {
        __terrane_traced_err(
            fail_comparison(
                String::from("byte values differ"),
                terrane_test_render_bytes(actual),
                terrane_test_render_bytes(expected),
            ),
            25 /* terrane-site: core/testing.trn:121:9-121:116 */,
        )?;
    }
    return Ok(());
}
pub fn assert_not_equal_bytes(
    actual: Vec<u8>,
    expected: Vec<u8>,
) -> Result<(), TerraneError> {
    if actual == expected {
        __terrane_traced_err(
            fail_comparison(
                String::from("byte values unexpectedly match"),
                terrane_test_render_bytes(actual),
                terrane_test_render_bytes(expected),
            ),
            26 /* terrane-site: core/testing.trn:126:9-126:128 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_int(
    actual: Option<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
    if (&actual).is_none() {
        __terrane_traced_err(
            fail(String::from("expected a present integer")),
            27 /* terrane-site: core/testing.trn:131:9-131:43 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_int(
    actual: Option<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
    if (&actual).is_some() {
        __terrane_traced_err(
            fail(String::from("expected no integer")),
            28 /* terrane-site: core/testing.trn:136:9-136:36 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_string(actual: Option<String>) -> Result<(), TerraneError> {
    if (&actual).is_none() {
        __terrane_traced_err(
            fail(String::from("expected a present string")),
            29 /* terrane-site: core/testing.trn:141:9-141:42 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_string(actual: Option<String>) -> Result<(), TerraneError> {
    if (&actual).is_some() {
        __terrane_traced_err(
            fail(String::from("expected no string")),
            30 /* terrane-site: core/testing.trn:146:9-146:35 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_bool(actual: Option<bool>) -> Result<(), TerraneError> {
    if (&actual).is_none() {
        __terrane_traced_err(
            fail(String::from("expected a present boolean")),
            31 /* terrane-site: core/testing.trn:151:9-151:43 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_bool(actual: Option<bool>) -> Result<(), TerraneError> {
    if (&actual).is_some() {
        __terrane_traced_err(
            fail(String::from("expected no boolean")),
            32 /* terrane-site: core/testing.trn:156:9-156:36 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_float64(actual: Option<f64>) -> Result<(), TerraneError> {
    if (&actual).is_none() {
        __terrane_traced_err(
            fail(String::from("expected a present floating value")),
            33 /* terrane-site: core/testing.trn:161:9-161:50 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_float64(actual: Option<f64>) -> Result<(), TerraneError> {
    if (&actual).is_some() {
        __terrane_traced_err(
            fail(String::from("expected no floating value")),
            34 /* terrane-site: core/testing.trn:166:9-166:43 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_bytes(actual: Option<Vec<u8>>) -> Result<(), TerraneError> {
    if (&actual).is_none() {
        __terrane_traced_err(
            fail(String::from("expected present bytes")),
            35 /* terrane-site: core/testing.trn:171:9-171:39 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_bytes(actual: Option<Vec<u8>>) -> Result<(), TerraneError> {
    if (&actual).is_some() {
        __terrane_traced_err(
            fail(String::from("expected no bytes")),
            36 /* terrane-site: core/testing.trn:176:9-176:34 */,
        )?;
    }
    return Ok(());
}
pub fn assert_near(
    actual: f64,
    expected: f64,
    tolerance: f64,
) -> Result<(), TerraneError> {
    let mut difference: f64;
    if tolerance < 0.0 {
        __terrane_traced_err(
            fail({
                let mut __terrane_text = String::with_capacity(46);
                __terrane_text
                    .push_str("near-equality tolerance must not be negative: ");
                terrane_scalar_support::ScalarDisplay::write_scalar(
                    &terrane_test_render_float64(tolerance),
                    &mut __terrane_text,
                );
                __terrane_text
            }),
            37 /* terrane-site: core/testing.trn:181:9-181:111 */,
        )?;
    }
    difference = actual - expected;
    if difference < 0.0 {
        difference = 0.0 - difference;
    }
    if difference > tolerance {
        __terrane_traced_err(
            fail_near(actual, expected, tolerance),
            38 /* terrane-site: core/testing.trn:186:9-186:47 */,
        )?;
    }
    return Ok(());
}
pub fn assert_throws(
    operation: std::sync::Arc<dyn Fn() -> Result<(), TerraneError> + Send + Sync>,
    expected_descriptor: String,
) -> Result<(), TerraneError> {
    let mut error: Option<TerraneError> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                operation(), 39 /* terrane-site: core/testing.trn:191:9-191:19 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    if error
                        .as_ref()
                        .expect("flow-proven available binding")
                        .clone()
                        .descriptor_name()
                        .to_owned()
                        .as_str() == expected_descriptor.as_str()
                    {
                        return TerraneCompletion::Return(());
                    }
                    __terrane_traced_completion!(
                        fail({ let mut __terrane_text = String::with_capacity(44);
                        __terrane_text.push_str("expected throwable descriptor ");
                        terrane_scalar_support::ScalarDisplay::write_scalar(&expected_descriptor,
                        &mut __terrane_text); __terrane_text.push_str(" but received ");
                        terrane_scalar_support::ScalarDisplay::write_scalar(&error
                        .as_ref().expect("flow-proven available binding").clone()
                        .descriptor_name().to_owned(), &mut __terrane_text);
                        __terrane_text }), 40 /* terrane-site: core/testing.trn:195:9-195:113 */
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    __terrane_traced_err(
        fail({
            let mut __terrane_text = String::with_capacity(27);
            __terrane_text.push_str("expected callback to throw ");
            terrane_scalar_support::ScalarDisplay::write_scalar(
                &expected_descriptor,
                &mut __terrane_text,
            );
            __terrane_text
        }),
        41 /* terrane-site: core/testing.trn:196:5-196:70 */,
    )?;
    return Ok(());
}
pub fn context_value(name: String) -> Option<String> {
    let mut entry: EnvironmentEntry;
    let __terrane_iterable_0 = environment();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry.name.is_text && entry.value.is_text
            && entry.name.text.as_str() == name.as_str()
        {
            return Some(entry.value.text.clone());
        }
    }
    return None;
}
pub fn test_arguments() -> terrane_collection_support::List<NativeString> {
    let supplied: terrane_collection_support::List<NativeString>;
    let mut controlled: terrane_collection_support::List<NativeString>;
    let mut index: terrane_int_support::Int;
    supplied = arguments();
    controlled = terrane_collection_support::List::<NativeString>::new(Vec::new());
    index = terrane_int_support::Int::from(1_i128);
    {
        let __terrane_list_append_0 = controlled.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(supplied.length()),
            )
        {
            {
                let __terrane_collection_argument_0 = __terrane_raised(
                    supplied
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                42 /* terrane-site: core/testing.trn:210:28-210:43 */,
                            ),
                        ),
                    42 /* terrane-site: core/testing.trn:210:28-210:43 */,
                );
                __terrane_list_append_0.push(__terrane_collection_argument_0);
            };
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return controlled;
}
pub fn test_deadline() -> Duration {
    return __terrane_traced(
        Duration::terrane_static_nanoseconds(terrane_test_deadline_nanoseconds()),
        43 /* terrane-site: core/testing.trn:215:12-215:68 */,
    );
}
pub fn temporary_directory() -> Option<String> {
    return context_value(String::from("TMPDIR"));
}
pub fn deterministic_seed() -> Option<String> {
    return context_value(String::from("TERRANE_TEST_SEED"));
}
pub fn test_identity() -> Option<String> {
    return context_value(String::from("TERRANE_TEST_ID"));
}
pub fn application_artifact() -> Option<String> {
    return context_value(String::from("TERRANE_TEST_ARTIFACT"));
}
pub fn test_tier() -> Option<String> {
    return context_value(String::from("TERRANE_TEST_TIER"));
}
pub fn advance_time(nanoseconds: terrane_int_support::Int) -> Result<(), TerraneError> {
    if nanoseconds.clone() < terrane_int_support::Int::from(0_i128) {
        __terrane_traced_err(
            fail(String::from("controlled time may only advance")),
            44 /* terrane-site: core/testing.trn:235:9-235:49 */,
        )?;
    }
    if !terrane_test_time_advance(nanoseconds.clone()) {
        __terrane_traced_err(
            fail(String::from("controlled time advance exceeds the host clock range")),
            45 /* terrane-site: core/testing.trn:237:9-237:69 */,
        )?;
    }
    return Ok(());
}
// Source: core/testing_process.trn
// Namespace: core/testing/process
#[derive(Clone)]
pub struct ProcessFixture {
    pub artifact: String,
    pub arguments: terrane_collection_support::List<NativeString>,
    pub environment: terrane_collection_support::List<EnvironmentEntry>,
    pub standard_input: Vec<u8>,
    pub deadline_milliseconds: terrane_int_support::Int,
}
impl ProcessFixture {
    pub fn terrane_construct(executable: String) -> Self {
        let mut __terrane_constructed_value = Self {
            artifact: String::from(""),
            arguments: terrane_collection_support::List::<NativeString>::new(Vec::new()),
            environment: terrane_collection_support::List::<
                EnvironmentEntry,
            >::new(Vec::new()),
            standard_input: Vec::<u8>::from([]),
            deadline_milliseconds: terrane_int_support::Int::from(30000_i128),
        };
        __terrane_constructed_value.construct(executable);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, executable: String) {
        {
            let __terrane_value = executable;
            self.artifact = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct ProcessResult {
    pub exit_code: terrane_int_support::Int,
    pub crashed: bool,
    pub timed_out: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}
impl ProcessResult {
    pub fn terrane_construct(
        status: terrane_int_support::Int,
        did_crash: bool,
        exceeded_deadline: bool,
        output: Vec<u8>,
        errors: Vec<u8>,
        output_truncated: bool,
        errors_truncated: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            exit_code: terrane_int_support::Int::from(0_i128),
            crashed: false,
            timed_out: false,
            stdout: Vec::<u8>::from([]),
            stderr: Vec::<u8>::from([]),
            stdout_truncated: false,
            stderr_truncated: false,
        };
        __terrane_constructed_value
            .construct(
                status,
                did_crash,
                exceeded_deadline,
                output,
                errors,
                output_truncated,
                errors_truncated,
            );
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        status: terrane_int_support::Int,
        did_crash: bool,
        exceeded_deadline: bool,
        output: Vec<u8>,
        errors: Vec<u8>,
        output_truncated: bool,
        errors_truncated: bool,
    ) {
        {
            let __terrane_value = status.clone();
            self.exit_code = __terrane_value;
        }
        {
            let __terrane_value = did_crash;
            self.crashed = __terrane_value;
        }
        {
            let __terrane_value = exceeded_deadline;
            self.timed_out = __terrane_value;
        }
        {
            let __terrane_value = output;
            self.stdout = __terrane_value;
        }
        {
            let __terrane_value = errors;
            self.stderr = __terrane_value;
        }
        {
            let __terrane_value = output_truncated;
            self.stdout_truncated = __terrane_value;
        }
        {
            let __terrane_value = errors_truncated;
            self.stderr_truncated = __terrane_value;
        }
    }
}
pub fn run_process(fixture: ProcessFixture) -> Result<ProcessResult, TerraneError> {
    let mut encoded_arguments: terrane_collection_support::List<String>;
    let mut argument: NativeString;
    let mut encoded_environment: terrane_collection_support::List<String>;
    let mut entry: EnvironmentEntry;
    let raw: TerranePlatformResult;
    encoded_arguments = terrane_collection_support::List::<String>::new(Vec::new());
    let __terrane_iterable_0 = fixture.arguments.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    {
        let __terrane_list_append_0 = encoded_arguments.make_unique();
        loop {
            argument = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            {
                let __terrane_collection_argument_0 = encode_native_string(argument);
                __terrane_list_append_0.push(__terrane_collection_argument_0);
            };
        }
    }
    encoded_environment = terrane_collection_support::List::<String>::new(Vec::new());
    let __terrane_iterable_1 = fixture.environment.clone();
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    {
        let __terrane_list_append_1 = encoded_environment.make_unique();
        loop {
            entry = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            {
                let __terrane_collection_argument_0 = encode_native_string(
                    entry.name.clone(),
                );
                __terrane_list_append_1.push(__terrane_collection_argument_0);
            };
            {
                let __terrane_collection_argument_0 = encode_native_string(
                    entry.value.clone(),
                );
                __terrane_list_append_1.push(__terrane_collection_argument_0);
            };
        }
    }
    raw = terrane_test_spawn(
        fixture.artifact,
        encoded_arguments,
        encoded_environment,
        fixture.standard_input,
        fixture.deadline_milliseconds.clone(),
    );
    if terrane_test_result_failed(&raw) {
        return Err({
            let value = TestInfrastructureFailure::terrane_construct(
                terrane_test_result_message(&raw),
            );
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
                46 /* terrane-site: core/testing_process.trn:47:9-47:84 */,
            )
        });
    }
    return Ok(
        ProcessResult::terrane_construct(
            terrane_test_result_exit_code(&raw),
            terrane_test_result_crashed(&raw),
            terrane_test_result_deadline_exceeded(&raw),
            terrane_test_result_stdout(&raw),
            terrane_test_result_stderr(&raw),
            terrane_test_result_stdout_truncated(&raw),
            terrane_test_result_stderr_truncated(&raw),
        ),
    );
}
// Source: core/process.trn
// Namespace: core/process
#[derive(Clone)]
pub struct NativeString {
    pub is_text: bool,
    pub text: String,
    pub raw: Vec<u8>,
}
impl NativeString {
    pub fn terrane_construct(encoded: String) -> Self {
        let mut __terrane_constructed_value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::<u8>::from([]),
        };
        __terrane_constructed_value.construct(encoded);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, encoded: String) {
        {
            let __terrane_value = terrane_platform_value_is_text(&encoded);
            self.is_text = __terrane_value;
        }
        {
            let __terrane_value = terrane_platform_value_text(&encoded);
            self.text = __terrane_value;
        }
        {
            let __terrane_value = terrane_platform_value_bytes(&encoded);
            self.raw = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct EnvironmentEntry {
    pub name: NativeString,
    pub value: NativeString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: NativeString, entry_value: NativeString) -> Self {
        let mut __terrane_constructed_value = Self {
            name: NativeString::terrane_construct(String::from("text:")),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        __terrane_constructed_value.construct(name, entry_value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, name: NativeString, entry_value: NativeString) {
        {
            let __terrane_value = name;
            self.name = __terrane_value;
        }
        {
            let __terrane_value = entry_value;
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct ProcessHostNameResult {
    pub failed: bool,
    pub available: bool,
    pub message: String,
    pub value: NativeString,
}
impl ProcessHostNameResult {
    pub fn terrane_construct(
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        __terrane_constructed_value
            .construct(did_fail, is_available, detail, result_value);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) {
        {
            let __terrane_value = did_fail;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = is_available;
            self.available = __terrane_value;
        }
        {
            let __terrane_value = detail;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = result_value;
            self.value = __terrane_value;
        }
    }
}
pub fn process_host_name() -> ProcessHostNameResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_support::system_host_name();
    return ProcessHostNameResult::terrane_construct(
        raw.failed,
        raw.flag,
        raw.message.clone(),
        NativeString::terrane_construct(raw.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<NativeString> {
    let encoded: Vec<String>;
    let mut values: terrane_collection_support::List<NativeString>;
    let mut index: terrane_int_support::Int;
    encoded = terrane_process_arguments();
    values = terrane_collection_support::List::<NativeString>::new(Vec::new());
    index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = values.make_unique();
        while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
            {
                let __terrane_collection_argument_0 = NativeString::terrane_construct(
                    __terrane_raised(
                        {
                            let __terrane_receiver = &encoded;
                            let __terrane_index = __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                47 /* terrane-site: core/process.trn:45:49-45:63 */,
                            );
                            __terrane_receiver
                                .get(__terrane_index)
                                .cloned()
                                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                    __terrane_index,
                                ))
                        },
                        47 /* terrane-site: core/process.trn:45:49-45:63 */,
                    ),
                );
                __terrane_list_append_0.push(__terrane_collection_argument_0);
            };
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return values;
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String>;
    let mut values: terrane_collection_support::List<EnvironmentEntry>;
    let mut index: terrane_int_support::Int;
    let mut name: NativeString;
    let mut value: NativeString;
    encoded = terrane_environment_entries();
    values = terrane_collection_support::List::<EnvironmentEntry>::new(Vec::new());
    index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = values.make_unique();
        while index.clone() + terrane_int_support::Int::from(1_i128)
            < terrane_int_support::Int::from(encoded.len() as i128)
        {
            name = NativeString::terrane_construct(
                __terrane_raised(
                    {
                        let __terrane_receiver = &encoded;
                        let __terrane_index = __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            48 /* terrane-site: core/process.trn:54:40-54:54 */,
                        );
                        __terrane_receiver
                            .get(__terrane_index)
                            .cloned()
                            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                __terrane_index,
                            ))
                    },
                    48 /* terrane-site: core/process.trn:54:40-54:54 */,
                ),
            );
            value = NativeString::terrane_construct(
                __terrane_raised(
                    {
                        let __terrane_receiver = &encoded;
                        let __terrane_index = __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &(index.clone() + terrane_int_support::Int::from(1_i128)),
                            ),
                            49 /* terrane-site: core/process.trn:55:41-55:59 */,
                        );
                        __terrane_receiver
                            .get(__terrane_index)
                            .cloned()
                            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                __terrane_index,
                            ))
                    },
                    49 /* terrane-site: core/process.trn:55:41-55:59 */,
                ),
            );
            {
                let __terrane_collection_argument_0 = EnvironmentEntry::terrane_construct(
                    name,
                    value,
                );
                __terrane_list_append_1.push(__terrane_collection_argument_0);
            };
            index = index.clone() + terrane_int_support::Int::from(2_i128);
        }
    }
    return values;
}
#[derive(Clone)]
pub struct CliSchema {
    pub entries: terrane_collection_support::List<String>,
}
impl CliSchema {
    pub fn terrane_construct(
        declared: terrane_collection_support::List<String>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        __terrane_constructed_value.construct(declared);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        {
            let __terrane_value = declared;
            self.entries = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct CommandLine {
    pub flags: terrane_collection_support::List<String>,
    pub option_names: terrane_collection_support::List<String>,
    pub option_values: terrane_collection_support::List<NativeString>,
    pub positionals: terrane_collection_support::List<NativeString>,
    pub diagnostic_arguments: terrane_collection_support::List<terrane_int_support::Int>,
    pub diagnostic_messages: terrane_collection_support::List<String>,
}
impl CommandLine {
    pub fn terrane_construct() -> Self {
        Self {
            flags: terrane_collection_support::List::<String>::new(Vec::new()),
            option_names: terrane_collection_support::List::<String>::new(Vec::new()),
            option_values: terrane_collection_support::List::<
                NativeString,
            >::new(Vec::new()),
            positionals: terrane_collection_support::List::<
                NativeString,
            >::new(Vec::new()),
            diagnostic_arguments: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            diagnostic_messages: terrane_collection_support::List::<
                String,
            >::new(Vec::new()),
        }
    }
}
pub fn schema_has(schema: CliSchema, sought: String) -> bool {
    let mut entry: String;
    let __terrane_iterable_0 = schema.entries.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry.as_str() == sought.as_str() {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<NativeString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String>;
    let mut option_names: terrane_collection_support::List<String>;
    let mut option_values: terrane_collection_support::List<NativeString>;
    let mut positionals: terrane_collection_support::List<NativeString>;
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut diagnostic_messages: terrane_collection_support::List<String>;
    let mut index: terrane_int_support::Int;
    let mut argument: NativeString;
    let mut flag_entry: String;
    let mut value_entry: String;
    let mut result: CommandLine;
    flags = terrane_collection_support::List::<String>::new(Vec::new());
    option_names = terrane_collection_support::List::<String>::new(Vec::new());
    option_values = terrane_collection_support::List::<NativeString>::new(Vec::new());
    positionals = terrane_collection_support::List::<NativeString>::new(Vec::new());
    diagnostic_arguments = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    diagnostic_messages = terrane_collection_support::List::<String>::new(Vec::new());
    index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_2 = diagnostic_arguments.make_unique();
        let __terrane_list_append_3 = diagnostic_messages.make_unique();
        let __terrane_list_append_4 = flags.make_unique();
        let __terrane_list_append_5 = option_names.make_unique();
        let __terrane_list_append_6 = option_values.make_unique();
        let __terrane_list_append_7 = positionals.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(supplied.length()),
            )
        {
            argument = __terrane_raised(
                supplied
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            50 /* terrane-site: core/process.trn:90:20-90:35 */,
                        ),
                    ),
                50 /* terrane-site: core/process.trn:90:20-90:35 */,
            );
            if !argument.is_text {
                {
                    let __terrane_collection_argument_0 = index.clone();
                    __terrane_list_append_2.push(__terrane_collection_argument_0);
                };
                {
                    let __terrane_collection_argument_0 = String::from(
                        "command-line option is not Unicode text",
                    );
                    __terrane_list_append_3.push(__terrane_collection_argument_0);
                };
            } else {
                flag_entry = {
                    let mut __terrane_text = String::with_capacity(5);
                    __terrane_text.push_str("flag:");
                    terrane_scalar_support::ScalarDisplay::write_scalar(
                        &argument.text,
                        &mut __terrane_text,
                    );
                    __terrane_text
                };
                value_entry = {
                    let mut __terrane_text = String::with_capacity(6);
                    __terrane_text.push_str("value:");
                    terrane_scalar_support::ScalarDisplay::write_scalar(
                        &argument.text,
                        &mut __terrane_text,
                    );
                    __terrane_text
                };
                if schema_has(schema.clone(), flag_entry) {
                    {
                        let __terrane_collection_argument_0 = argument.text.clone();
                        __terrane_list_append_4.push(__terrane_collection_argument_0);
                    };
                } else if schema_has(schema.clone(), value_entry) {
                    if index.clone() + terrane_int_support::Int::from(1_i128)
                        >= terrane_int_support::Int::from(
                            terrane_int_support::Int::from(supplied.length()),
                        )
                    {
                        {
                            let __terrane_collection_argument_0 = index.clone();
                            __terrane_list_append_2
                                .push(__terrane_collection_argument_0);
                        };
                        {
                            let __terrane_collection_argument_0 = String::from(
                                "option requires a value",
                            );
                            __terrane_list_append_3
                                .push(__terrane_collection_argument_0);
                        };
                    } else {
                        {
                            let __terrane_collection_argument_0 = argument.text.clone();
                            __terrane_list_append_5
                                .push(__terrane_collection_argument_0);
                        };
                        {
                            let __terrane_collection_argument_0 = __terrane_raised(
                                supplied
                                    .get_or_error(
                                        __terrane_raised(
                                            terrane_collection_support::index_from_int(
                                                &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                            ),
                                            51 /* terrane-site: core/process.trn:105:43-105:62 */,
                                        ),
                                    ),
                                51 /* terrane-site: core/process.trn:105:43-105:62 */,
                            );
                            __terrane_list_append_6
                                .push(__terrane_collection_argument_0);
                        };
                        index = index.clone() + terrane_int_support::Int::from(1_i128);
                    }
                } else if argument.text.starts_with(&String::from("--")) {
                    {
                        let __terrane_collection_argument_0 = index.clone();
                        __terrane_list_append_2.push(__terrane_collection_argument_0);
                    };
                    {
                        let __terrane_collection_argument_0 = String::from(
                            "unknown option",
                        );
                        __terrane_list_append_3.push(__terrane_collection_argument_0);
                    };
                } else {
                    {
                        let __terrane_collection_argument_0 = argument;
                        __terrane_list_append_7.push(__terrane_collection_argument_0);
                    };
                }
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    result = CommandLine::terrane_construct();
    {
        let __terrane_value = flags;
        result.flags = __terrane_value;
    }
    {
        let __terrane_value = option_names;
        result.option_names = __terrane_value;
    }
    {
        let __terrane_value = option_values;
        result.option_values = __terrane_value;
    }
    {
        let __terrane_value = positionals;
        result.positionals = __terrane_value;
    }
    {
        let __terrane_value = diagnostic_arguments;
        result.diagnostic_arguments = __terrane_value;
    }
    {
        let __terrane_value = diagnostic_messages;
        result.diagnostic_messages = __terrane_value;
    }
    return result;
}
#[derive(Clone)]
pub struct ExitStatus {
    pub code: terrane_int_support::Int,
    pub valid: bool,
}
impl ExitStatus {
    pub fn terrane_construct() -> Self {
        Self {
            code: terrane_int_support::Int::from(0_i128),
            valid: true,
        }
    }
}
pub fn make_exit_status(requested: terrane_int_support::Int) -> ExitStatus {
    let mut result: ExitStatus;
    result = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        {
            let __terrane_value = terrane_int_support::Int::from(255_i128);
            result.code = __terrane_value;
        }
        {
            let __terrane_value = false;
            result.valid = __terrane_value;
        }
    } else {
        {
            let __terrane_value = requested.clone();
            result.code = __terrane_value;
        }
    }
    return result;
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
pub fn native_text(value: String) -> NativeString {
    let encoded: String;
    encoded = terrane_platform_value_from_text(&value);
    return NativeString::terrane_construct(encoded);
}
pub fn native_raw(value: Vec<u8>) -> NativeString {
    let encoded: String;
    encoded = terrane_platform_value_from_bytes(&value);
    return NativeString::terrane_construct(encoded);
}
pub fn native_text_value(value: NativeString) -> Option<String> {
    if value.is_text {
        return Some(value.text.clone());
    }
    return None;
}
pub fn native_raw_value(value: NativeString) -> Vec<u8> {
    return value.raw.clone();
}
pub fn environment_pair(key: NativeString, item: NativeString) -> EnvironmentEntry {
    return EnvironmentEntry::terrane_construct(key, item);
}
pub fn encode_native_string(value: NativeString) -> String {
    if value.is_text {
        return terrane_platform_value_from_text(&value.text);
    }
    return terrane_platform_value_from_bytes(&value.raw);
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
            let __terrane_value = total.clone();
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
            let __terrane_value = whole_seconds.clone();
            self.seconds = __terrane_value;
        }
        {
            let __terrane_value = fractional_nanoseconds.clone();
            self.nanoseconds = __terrane_value;
        }
        {
            let __terrane_value = whole_seconds.clone()
                * terrane_int_support::Int::from(1000000000_i128)
                + fractional_nanoseconds.clone();
            self.total_nanoseconds = __terrane_value;
        }
        {
            let __terrane_value = DurationSubtraction::terrane_construct(
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
        if multiplier.clone() < terrane_int_support::Int::from(0_i128) {
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    52 /* terrane-site: core/time.trn:62:13-62:45 */,
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
        if value.clone() < terrane_int_support::Int::from(0_i128) {
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    53 /* terrane-site: core/time.trn:38:13-38:45 */,
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
        if value.clone() < terrane_int_support::Int::from(0_i128) {
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    54 /* terrane-site: core/time.trn:43:13-43:45 */,
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
        if value.clone() < terrane_int_support::Int::from(0_i128) {
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    55 /* terrane-site: core/time.trn:48:13-48:45 */,
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
        if value.clone() < terrane_int_support::Int::from(0_i128) {
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    56 /* terrane-site: core/time.trn:53:13-53:45 */,
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
            let __terrane_value = runtime_domain.clone();
            self.domain = __terrane_value;
        }
        {
            let __terrane_value = elapsed.clone();
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    57 /* terrane-site: core/time.trn:76:13-76:45 */,
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
            let __terrane_value = seconds.clone();
            self.unix_seconds = __terrane_value;
        }
        {
            let __terrane_value = fractional_nanoseconds.clone();
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
            let __terrane_value = target;
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    58 /* terrane-site: core/time.trn:105:13-105:45 */,
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
            let __terrane_value = scheduled_at;
            self.scheduled = __terrane_value;
        }
        {
            let __terrane_value = observed_at;
            self.observed = __terrane_value;
        }
        {
            let __terrane_value = expirations.clone();
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
            let __terrane_value = Clock::terrane_static_monotonic();
            self.anchor = __terrane_value;
        }
        {
            let __terrane_value = interval;
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
        if observed_index.clone() < self.next_index.clone() {
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
            let __terrane_value = observed_index.clone()
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
        if observed_index.clone()
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
            let __terrane_value = observed_index.clone()
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    59 /* terrane-site: core/time.trn:161:13-161:45 */,
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
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    message,
                    rendered,
                    60 /* terrane-site: core/time.trn:172:13-172:45 */,
                )
            });
        }
        return Ok(Ticker::terrane_construct(period));
    }
}
