// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, time_testing.rs, platform_process.rs, platform_testing.rs, platform_capability_types.rs, platform_result_type.rs, platform_capability_base.rs, platform_time.rs
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
// Source: src/library.trn
// Namespace: native-testing-surface
static __TERRANE_F0_AVAILABLE: std::sync::LazyLock<bool> = std::sync::LazyLock::new(|| {
    true
});
// Source: tests/unit/surface.trn
// Namespace: native-testing-surface
fn expected_error() -> Result<(), TerraneError> {
    __terrane_traced_err(
        fail(String::from("expected")),
        0 /* terrane-site: tests/unit/surface.trn:6:5-6:21 */,
    )?;
    return Ok(());
}
fn test_surface() -> Result<(), TerraneError> {
    __terrane_traced_err(
        assert(true),
        1 /* terrane-site: tests/unit/surface.trn:10:5-10:17 */,
    )?;
    __terrane_traced_err(
        deny(false),
        2 /* terrane-site: tests/unit/surface.trn:11:5-11:16 */,
    )?;
    __terrane_traced_err(
        assert_equal_int(
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(4_i128),
        ),
        3 /* terrane-site: tests/unit/surface.trn:12:5-12:27 */,
    )?;
    __terrane_traced_err(
        assert_not_equal_string(String::from("left"), String::from("right")),
        4 /* terrane-site: tests/unit/surface.trn:13:5-13:45 */,
    )?;
    __terrane_traced_err(
        assert_present_string(Some(String::from("present"))),
        5 /* terrane-site: tests/unit/surface.trn:14:5-14:37 */,
    )?;
    __terrane_traced_err(
        assert_none_string(None),
        6 /* terrane-site: tests/unit/surface.trn:15:5-15:29 */,
    )?;
    __terrane_traced_err(
        assert_near(1.0, 1.1, 0.2),
        7 /* terrane-site: tests/unit/surface.trn:16:5-16:31 */,
    )?;
    __terrane_traced_err(
        assert_throws(
            std::sync::Arc::new(expected_error),
            TerraneDescriptor {
                identity: "/core/testing::test-failure",
                name: "test-failure",
                kind: "class",
                inherently_identity_bearing: false,
                fields: &[
                    TerraneFieldMetadata {
                        name: "message",
                        external_name: "message",
                        defaulted: true,
                        optional: false,
                        secret: false,
                    },
                    TerraneFieldMetadata {
                        name: "assertion-source",
                        external_name: "assertion-source",
                        defaulted: true,
                        optional: false,
                        secret: false,
                    },
                    TerraneFieldMetadata {
                        name: "details",
                        external_name: "details",
                        defaulted: true,
                        optional: false,
                        secret: false,
                    },
                ],
            }
                .identity
                .to_owned(),
        ),
        8 /* terrane-site: tests/unit/surface.trn:17:5-17:57 */,
    )?;
    let fixture: ProcessFixture = ProcessFixture::terrane_construct(
        String::from("artifact"),
    );
    __terrane_traced_err(
        assert_equal_string(fixture.artifact.clone(), String::from("artifact")),
        9 /* terrane-site: tests/unit/surface.trn:19:5-19:54 */,
    )?;
    temporary_directory();
    return Ok(());
}
// Source: core/testing.trn
// Namespace: core/testing
pub trait TestValueProtocol {
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
        let mut value = Self {
            message: String::from(""),
            assertion_source: String::from(""),
            details: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(failure_message);
        value
    }
    pub fn construct(&mut self, failure_message: String) {
        self.message = failure_message;
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
        let mut value = Self {
            message: String::from(""),
            assertion_source: String::from(""),
            details: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(reason);
        value
    }
    pub fn construct(&mut self, reason: String) {
        self.message = reason;
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
        let mut value = Self { message: String::from("") };
        value.construct(failure_message);
        value
    }
    pub fn construct(&mut self, failure_message: String) {
        self.message = failure_message;
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
pub fn fail(message: String) -> Result<(), TerraneError> {
    return Err({
        let value = TestFailure::terrane_construct(message);
        let details = value.details.clone().into_iter().collect();
        TerraneError::raised_with_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                value.render(),
                10 /* terrane-site: core/testing.trn:43:5-43:41 */,
            )
            .with_structured_details(details)
    });
}
pub fn skip(reason: String) -> Result<(), TerraneError> {
    return Err({
        let value = TestSkip::terrane_construct(reason);
        TerraneError::raised_with_message(
            TerraneErrorKind::Custom(DescriptorId(1)),
            value.render(),
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
    let mut details: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    details
        .append(
            format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("actual: ")),
                terrane_scalar_support::scalar_text(&actual)
            ),
        );
    details
        .append(
            format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("expected: ")),
                terrane_scalar_support::scalar_text(&expected)
            ),
        );
    let mut failure: TestFailure = TestFailure::terrane_construct(failure_message);
    failure.details = details.clone();
    return Err({
        let value = failure;
        let details = value.details.clone().into_iter().collect();
        TerraneError::raised_with_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                value.render(),
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
    let mut details: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    details
        .append(
            format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("actual: ")),
                terrane_scalar_support::scalar_text(&terrane_test_render_float64(actual))
            ),
        );
    details
        .append(
            format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("expected: ")),
                terrane_scalar_support::scalar_text(&terrane_test_render_float64(expected))
            ),
        );
    details
        .append(
            format!(
                "{}{}",
                terrane_scalar_support::scalar_text(&String::from("tolerance: ")),
                terrane_scalar_support::scalar_text(&terrane_test_render_float64(tolerance))
            ),
        );
    let mut failure: TestFailure = TestFailure::terrane_construct(
        String::from("floating values differ beyond tolerance"),
    );
    failure.details = details.clone();
    return Err({
        let value = failure;
        let details = value.details.clone().into_iter().collect();
        TerraneError::raised_with_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                value.render(),
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
    if actual != expected {
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
    if actual == expected {
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
    if actual.is_none() {
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
    if actual.is_some() {
        __terrane_traced_err(
            fail(String::from("expected no integer")),
            28 /* terrane-site: core/testing.trn:136:9-136:36 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_string(actual: Option<String>) -> Result<(), TerraneError> {
    if actual.is_none() {
        __terrane_traced_err(
            fail(String::from("expected a present string")),
            29 /* terrane-site: core/testing.trn:141:9-141:42 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_string(actual: Option<String>) -> Result<(), TerraneError> {
    if actual.is_some() {
        __terrane_traced_err(
            fail(String::from("expected no string")),
            30 /* terrane-site: core/testing.trn:146:9-146:35 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_bool(actual: Option<bool>) -> Result<(), TerraneError> {
    if actual.is_none() {
        __terrane_traced_err(
            fail(String::from("expected a present boolean")),
            31 /* terrane-site: core/testing.trn:151:9-151:43 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_bool(actual: Option<bool>) -> Result<(), TerraneError> {
    if actual.is_some() {
        __terrane_traced_err(
            fail(String::from("expected no boolean")),
            32 /* terrane-site: core/testing.trn:156:9-156:36 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_float64(actual: Option<f64>) -> Result<(), TerraneError> {
    if actual.is_none() {
        __terrane_traced_err(
            fail(String::from("expected a present floating value")),
            33 /* terrane-site: core/testing.trn:161:9-161:50 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_float64(actual: Option<f64>) -> Result<(), TerraneError> {
    if actual.is_some() {
        __terrane_traced_err(
            fail(String::from("expected no floating value")),
            34 /* terrane-site: core/testing.trn:166:9-166:43 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_bytes(actual: Option<Vec<u8>>) -> Result<(), TerraneError> {
    if actual.is_none() {
        __terrane_traced_err(
            fail(String::from("expected present bytes")),
            35 /* terrane-site: core/testing.trn:171:9-171:39 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_bytes(actual: Option<Vec<u8>>) -> Result<(), TerraneError> {
    if actual.is_some() {
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
    if tolerance < 0.0 {
        __terrane_traced_err(
            fail(
                format!(
                    "{}{}",
                    terrane_scalar_support::scalar_text(&String::from("near-equality tolerance must not be negative: ")),
                    terrane_scalar_support::scalar_text(&terrane_test_render_float64(tolerance))
                ),
            ),
            37 /* terrane-site: core/testing.trn:181:9-181:111 */,
        )?;
    }
    let mut difference: f64 = actual - expected;
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
                    let error = __terrane_error_0.clone();
                    if error.descriptor_name().to_owned() == expected_descriptor {
                        return TerraneCompletion::Return(());
                    }
                    __terrane_traced_completion!(
                        fail(format!("{}{}{}{}",
                        terrane_scalar_support::scalar_text(&String::from("expected throwable descriptor ")),
                        terrane_scalar_support::scalar_text(&expected_descriptor),
                        terrane_scalar_support::scalar_text(&String::from(" but received ")),
                        terrane_scalar_support::scalar_text(&error.descriptor_name()
                        .to_owned()))), 40 /* terrane-site: core/testing.trn:195:9-195:113 */
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
        fail(
            format!(
                "{}{}",
                terrane_scalar_support::scalar_text(&String::from("expected callback to throw ")),
                terrane_scalar_support::scalar_text(&expected_descriptor)
            ),
        ),
        41 /* terrane-site: core/testing.trn:196:5-196:70 */,
    )?;
    return Ok(());
}
pub fn context_value(name: String) -> Option<String> {
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &environment(),
    );
    loop {
        let entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry.name.is_text && entry.value.is_text && entry.name.text == name {
            return Some(entry.value.text.clone());
        }
    }
    return None;
}
pub fn test_arguments() -> terrane_collection_support::List<NativeString> {
    let supplied: terrane_collection_support::List<NativeString> = arguments();
    let mut controlled: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(1_i128);
    {
        let __terrane_list_append_0 = controlled.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(supplied.length()),
            )
        {
            __terrane_list_append_0
                .push(
                    __terrane_raised(
                        supplied
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(&index.clone()),
                                    42 /* terrane-site: core/testing.trn:210:28-210:43 */,
                                ),
                            ),
                        42 /* terrane-site: core/testing.trn:210:28-210:43 */,
                    ),
                );
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return controlled.clone();
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
        let mut value = Self {
            artifact: String::from(""),
            arguments: terrane_collection_support::List::<NativeString>::new(Vec::new()),
            environment: terrane_collection_support::List::<
                EnvironmentEntry,
            >::new(Vec::new()),
            standard_input: Vec::from([]),
            deadline_milliseconds: terrane_int_support::Int::from(30000_i128),
        };
        value.construct(executable);
        value
    }
    pub fn construct(&mut self, executable: String) {
        self.artifact = executable;
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
        let mut value = Self {
            exit_code: terrane_int_support::Int::from(0_i128),
            crashed: false,
            timed_out: false,
            stdout: Vec::from([]),
            stderr: Vec::from([]),
            stdout_truncated: false,
            stderr_truncated: false,
        };
        value
            .construct(
                status,
                did_crash,
                exceeded_deadline,
                output,
                errors,
                output_truncated,
                errors_truncated,
            );
        value
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
        self.exit_code = status.clone();
        self.crashed = did_crash;
        self.timed_out = exceeded_deadline;
        self.stdout = output;
        self.stderr = errors;
        self.stdout_truncated = output_truncated;
        self.stderr_truncated = errors_truncated;
    }
}
pub fn run_process(fixture: ProcessFixture) -> Result<ProcessResult, TerraneError> {
    let mut encoded_arguments: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &fixture.arguments,
    );
    {
        let __terrane_list_append_0 = encoded_arguments.make_unique();
        loop {
            let argument = match __terrane_iterator_0.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_0.push(encode_native_string(argument));
        }
    }
    let mut encoded_environment: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &fixture.environment,
    );
    {
        let __terrane_list_append_1 = encoded_environment.make_unique();
        loop {
            let entry = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_1.push(encode_native_string(entry.name));
            __terrane_list_append_1.push(encode_native_string(entry.value));
        }
    }
    let raw: TerranePlatformResult = terrane_test_spawn(
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
            TerraneError::raised_with_message(
                TerraneErrorKind::Custom(DescriptorId(2)),
                value.render(),
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
        let mut value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::from([]),
        };
        value.construct(encoded);
        value
    }
    pub fn construct(&mut self, encoded: String) {
        self.is_text = terrane_platform_value_is_text(&encoded);
        self.text = terrane_platform_value_text(&encoded);
        self.raw = terrane_platform_value_bytes(&encoded);
    }
}
#[derive(Clone)]
pub struct EnvironmentEntry {
    pub name: NativeString,
    pub value: NativeString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: NativeString, entry_value: NativeString) -> Self {
        let mut value = Self {
            name: NativeString::terrane_construct(String::from("text:")),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        value.construct(name, entry_value);
        value
    }
    pub fn construct(&mut self, name: NativeString, entry_value: NativeString) {
        self.name = name.clone();
        self.value = entry_value.clone();
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
        let mut value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        value.construct(did_fail, is_available, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) {
        self.failed = did_fail;
        self.available = is_available;
        self.message = detail;
        self.value = result_value.clone();
    }
}
pub fn process_host_name() -> ProcessHostNameResult {
    let raw: TerranePlatformResult = terrane_platform_support::system_host_name();
    return ProcessHostNameResult::terrane_construct(
        raw.failed,
        raw.flag,
        raw.message.clone(),
        NativeString::terrane_construct(raw.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<NativeString> {
    let encoded: Vec<String> = terrane_process_arguments();
    let mut values: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = values.make_unique();
        while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
            __terrane_list_append_0
                .push(
                    NativeString::terrane_construct(
                        __terrane_raised(
                            encoded
                                .get(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        47 /* terrane-site: core/process.trn:45:49-45:63 */,
                                    ),
                                )
                                .cloned()
                                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        47 /* terrane-site: core/process.trn:45:49-45:63 */,
                                    ),
                                )),
                            47 /* terrane-site: core/process.trn:45:49-45:63 */,
                        ),
                    ),
                );
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return values.clone();
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String> = terrane_environment_entries();
    let mut values: terrane_collection_support::List<EnvironmentEntry> = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = values.make_unique();
        while index.clone() + terrane_int_support::Int::from(1_i128)
            < terrane_int_support::Int::from(encoded.len() as i128)
        {
            let name: NativeString = NativeString::terrane_construct(
                __terrane_raised(
                    encoded
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                48 /* terrane-site: core/process.trn:54:40-54:54 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                48 /* terrane-site: core/process.trn:54:40-54:54 */,
                            ),
                        )),
                    48 /* terrane-site: core/process.trn:54:40-54:54 */,
                ),
            );
            let value: NativeString = NativeString::terrane_construct(
                __terrane_raised(
                    encoded
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                49 /* terrane-site: core/process.trn:55:41-55:59 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                49 /* terrane-site: core/process.trn:55:41-55:59 */,
                            ),
                        )),
                    49 /* terrane-site: core/process.trn:55:41-55:59 */,
                ),
            );
            __terrane_list_append_1
                .push(EnvironmentEntry::terrane_construct(name, value));
            index = index.clone() + terrane_int_support::Int::from(2_i128);
        }
    }
    return values.clone();
}
#[derive(Clone)]
pub struct CliSchema {
    pub entries: terrane_collection_support::List<String>,
}
impl CliSchema {
    pub fn terrane_construct(
        declared: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(declared);
        value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        self.entries = declared.clone();
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
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &schema.entries,
    );
    loop {
        let entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry == sought {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<NativeString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_names: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_values: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut positionals: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut diagnostic_messages: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
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
            let argument: NativeString = __terrane_raised(
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
                __terrane_list_append_2.push(index.clone());
                __terrane_list_append_3
                    .push(String::from("command-line option is not Unicode text"));
            } else {
                let flag_entry: String = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("flag:")),
                    terrane_scalar_support::scalar_text(&argument.text)
                );
                let value_entry: String = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("value:")),
                    terrane_scalar_support::scalar_text(&argument.text)
                );
                if schema_has(schema.clone(), flag_entry) {
                    __terrane_list_append_4.push(argument.text.clone());
                } else if schema_has(schema.clone(), value_entry) {
                    if index.clone() + terrane_int_support::Int::from(1_i128)
                        >= terrane_int_support::Int::from(
                            terrane_int_support::Int::from(supplied.length()),
                        )
                    {
                        __terrane_list_append_2.push(index.clone());
                        __terrane_list_append_3
                            .push(String::from("option requires a value"));
                    } else {
                        __terrane_list_append_5.push(argument.text.clone());
                        __terrane_list_append_6
                            .push(
                                __terrane_raised(
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
                                ),
                            );
                        index = index.clone() + terrane_int_support::Int::from(1_i128);
                    }
                } else if argument.text.starts_with(&String::from("--")) {
                    __terrane_list_append_2.push(index.clone());
                    __terrane_list_append_3.push(String::from("unknown option"));
                } else {
                    __terrane_list_append_7.push(argument.clone());
                }
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut result: CommandLine = CommandLine::terrane_construct();
    result.flags = flags.clone();
    result.option_names = option_names.clone();
    result.option_values = option_values.clone();
    result.positionals = positionals.clone();
    result.diagnostic_arguments = diagnostic_arguments.clone();
    result.diagnostic_messages = diagnostic_messages.clone();
    return result.clone();
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
    let mut result: ExitStatus = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result.code = terrane_int_support::Int::from(255_i128);
        result.valid = false;
    } else {
        result.code = requested.clone();
    }
    return result.clone();
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
pub fn native_text(value: String) -> NativeString {
    let encoded: String = terrane_platform_value_from_text(&value);
    return NativeString::terrane_construct(encoded);
}
pub fn native_raw(value: Vec<u8>) -> NativeString {
    let encoded: String = terrane_platform_value_from_bytes(&value);
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
    return EnvironmentEntry::terrane_construct(key.clone(), item.clone());
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
        let mut value = Self {
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        value.construct(total);
        value
    }
    pub fn construct(&mut self, total: terrane_int_support::Int) {
        self.total_nanoseconds = total.clone();
    }
    pub fn checked(&self, other: Duration) -> Option<Duration> {
        if self.total_nanoseconds.clone() < other.total_nanoseconds.clone() {
            return None;
        }
        let difference: terrane_int_support::Int = self.total_nanoseconds.clone()
            - other.total_nanoseconds.clone();
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
        let mut value = Self {
            seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
            subtract: DurationSubtraction::terrane_construct(
                terrane_int_support::Int::from(0_i128),
            ),
        };
        value.construct(whole_seconds, fractional_nanoseconds);
        value
    }
    pub fn construct(
        &mut self,
        whole_seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) {
        self.seconds = whole_seconds.clone();
        self.nanoseconds = fractional_nanoseconds.clone();
        self.total_nanoseconds = whole_seconds.clone()
            * terrane_int_support::Int::from(1000000000_i128)
            + fractional_nanoseconds.clone();
        self.subtract = DurationSubtraction::terrane_construct(
            self.total_nanoseconds.clone(),
        );
    }
    pub fn add(&self, other: Duration) -> Duration {
        let fractional: terrane_int_support::Int = self.nanoseconds.clone()
            + other.nanoseconds.clone();
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
        if multiplier.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
                    52 /* terrane-site: core/time.trn:62:13-62:45 */,
                )
            });
        }
        let total: terrane_int_support::Int = self.total_nanoseconds.clone()
            * multiplier.clone();
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
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
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
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
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
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
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
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
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
        let mut value = Self {
            domain: terrane_int_support::Int::from(0_i128),
            elapsed_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        value.construct(runtime_domain, elapsed);
        value
    }
    pub fn construct(
        &mut self,
        runtime_domain: terrane_int_support::Int,
        elapsed: terrane_int_support::Int,
    ) {
        self.domain = runtime_domain.clone();
        self.elapsed_nanoseconds = elapsed.clone();
    }
    pub fn duration_until(
        &self,
        later: &MonotonicInstant,
    ) -> Result<Duration, TerraneError> {
        if self.domain.clone() != later.domain.clone().clone()
            || later.elapsed_nanoseconds.clone().clone()
                < self.elapsed_nanoseconds.clone()
        {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
                    57 /* terrane-site: core/time.trn:76:13-76:45 */,
                )
            });
        }
        let elapsed: terrane_int_support::Int = later.elapsed_nanoseconds.clone().clone()
            - self.elapsed_nanoseconds.clone();
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
        let mut value = Self {
            unix_seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        value.construct(seconds, fractional_nanoseconds);
        value
    }
    pub fn construct(
        &mut self,
        seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) {
        self.unix_seconds = seconds.clone();
        self.nanoseconds = fractional_nanoseconds.clone();
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Deadline {
    pub expires_at: MonotonicInstant,
}
impl Deadline {
    pub fn terrane_construct(target: MonotonicInstant) -> Self {
        let mut value = Self {
            expires_at: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
        };
        value.construct(target);
        value
    }
    pub fn construct(&mut self, target: MonotonicInstant) {
        self.expires_at = target.clone();
    }
    pub fn remaining(&self) -> Option<Duration> {
        let now: MonotonicInstant = Clock::terrane_static_monotonic();
        if self.expires_at.elapsed_nanoseconds.clone() <= now.elapsed_nanoseconds.clone()
        {
            return None;
        }
        let elapsed: terrane_int_support::Int = self
            .expires_at
            .elapsed_nanoseconds
            .clone() - now.elapsed_nanoseconds.clone();
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
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
                    58 /* terrane-site: core/time.trn:105:13-105:45 */,
                )
            });
        }
        return Ok(Deadline::terrane_construct(target.clone()));
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
        let mut value = Self {
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
        value.construct(scheduled_at, observed_at, expirations);
        value
    }
    pub fn construct(
        &mut self,
        scheduled_at: MonotonicInstant,
        observed_at: MonotonicInstant,
        expirations: terrane_int_support::Int,
    ) {
        self.scheduled = scheduled_at.clone();
        self.observed = observed_at.clone();
        self.count = expirations.clone();
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
        let mut value = Self {
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
        value.construct(interval);
        value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, interval: Duration) {
        self.anchor = Clock::terrane_static_monotonic();
        self.period = interval.clone();
    }
    pub async fn next(&mut self) -> Tick {
        let period_total: terrane_int_support::Int = self
            .period
            .total_nanoseconds
            .clone();
        let scheduled_nanoseconds: terrane_int_support::Int = self
            .anchor
            .elapsed_nanoseconds
            .clone() + period_total.clone() * self.next_index.clone();
        discard_none(
            __terrane_await(terrane_platform_time_sleep_until(scheduled_nanoseconds))
                .await,
        );
        let observed: MonotonicInstant = Clock::terrane_static_monotonic();
        let elapsed: terrane_int_support::Int = observed.elapsed_nanoseconds.clone()
            - self.anchor.elapsed_nanoseconds.clone();
        let mut observed_index: terrane_int_support::Int = terrane_platform_time_div(
            &elapsed,
            period_total.clone(),
        );
        if observed_index.clone() < self.next_index.clone() {
            observed_index = self.next_index.clone();
        }
        let count: terrane_int_support::Int = observed_index.clone()
            - self.next_index.clone() + terrane_int_support::Int::from(1_i128);
        let delivered: MonotonicInstant = MonotonicInstant::terrane_construct(
            self.anchor.domain.clone(),
            self.anchor.elapsed_nanoseconds.clone()
                + period_total.clone() * observed_index.clone(),
        );
        self.next_index = observed_index.clone()
            + terrane_int_support::Int::from(1_i128);
        return Tick::terrane_construct(delivered, observed.clone(), count.clone());
    }
    pub fn destruct(&mut self) {
        discard_none(());
    }
}
impl Drop for Ticker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
#[derive(Clone)]
pub struct Clock {}
impl Clock {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_wall() -> Instant {
        let raw: TerranePlatformResult = terrane_platform_time_wall();
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
        let target: terrane_int_support::Int = terrane_platform_time_monotonic()
            + elapsed.total_nanoseconds.clone();
        return __terrane_await(terrane_platform_time_sleep_until(target)).await;
    }
    pub async fn terrane_static_sleep_until(
        target: MonotonicInstant,
    ) -> Result<(), TerraneError> {
        if target.domain.clone() != terrane_platform_time_domain() {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
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
        let now: MonotonicInstant = Clock::terrane_static_monotonic();
        let target: MonotonicInstant = MonotonicInstant::terrane_construct(
            now.domain.clone(),
            now.elapsed_nanoseconds.clone() + elapsed.total_nanoseconds.clone(),
        );
        return Deadline::terrane_construct(target);
    }
    pub fn terrane_static_interval(period: Duration) -> Result<Ticker, TerraneError> {
        if period.total_nanoseconds.clone() == terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(3)),
                    value.render(),
                    60 /* terrane-site: core/time.trn:172:13-172:45 */,
                )
            });
        }
        return Ok(Ticker::terrane_construct(period.clone()));
    }
}
// Source: <compiler-generated test registry>
// Namespace: core/testing/runner
fn __terrane_test_hex(value: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}
fn __terrane_test_record(outcome: &str, error: &TerraneError) {
    let Ok(path) = std::env::var("TERRANE_TEST_RESULT") else {
        return;
    };
    let details = error.structured_details();
    let frames = error.source_frames();
    let mut record = format!(
        "1\n{}\n{}\n{}\n{}\n{}\n", outcome, __terrane_test_hex(&error.descriptor_name()),
        __terrane_test_hex(error.message()), details.len(), frames.len(),
    );
    for detail in details {
        record.push_str(&__terrane_test_hex(detail));
        record.push('\n');
    }
    for frame in frames {
        record.push_str(&__terrane_test_hex(&frame));
        record.push('\n');
    }
    let _ = std::fs::write(path, record);
}
fn main() {
    let selected = std::env::args().nth(1).unwrap_or_default();
    match selected.as_str() {
        "0" => {
            if let Err(error) = test_surface() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
