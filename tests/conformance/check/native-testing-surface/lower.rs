// Generated deterministically by Terrane <version>.
// Runtime support: platform_result_type.rs, platform_process.rs
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
    pub static DESCRIPTORS: [&str; 2] = ["test-failure", "test-skip"];
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
    pub static FILES: [&str; 3] = [
        "src/main.trn",
        "core/testing.trn",
        "core/process.trn",
    ];
    pub static FUNCTIONS: [&str; 21] = [
        "/native-testing-surface::expected-error",
        "/native-testing-surface::main",
        "/core/testing::fail",
        "/core/testing::skip",
        "/core/testing::assert",
        "/core/testing::deny",
        "/core/testing::assert-equal-int",
        "/core/testing::assert-not-equal-int",
        "/core/testing::assert-equal-string",
        "/core/testing::assert-not-equal-string",
        "/core/testing::assert-equal-bool",
        "/core/testing::assert-not-equal-bool",
        "/core/testing::assert-present-string",
        "/core/testing::assert-none-string",
        "/core/testing::assert-near",
        "/core/testing::assert-throws",
        "/core/testing::run-process",
        "/core/testing::advance-time",
        "/core/process::arguments",
        "/core/process::environment",
        "/core/process::parse-command-line",
    ];
    pub static SITES: [Site; 33] = [
        /* terrane-site-row: site 0: /native-testing-surface::expected-error (src/main.trn:5:5-5:21) */
        { Site { function: 0, file: 0, line: 5, column: 5, end_line: 5, end_column: 21 } },
        /* terrane-site-row: site 1: /native-testing-surface::main (src/main.trn:9:5-9:17) */
        { Site { function: 1, file: 0, line: 9, column: 5, end_line: 9, end_column: 17 } },
        /* terrane-site-row: site 2: /native-testing-surface::main (src/main.trn:10:5-10:16) */
        { Site { function: 1, file: 0, line: 10, column: 5, end_line: 10, end_column: 16 } },
        /* terrane-site-row: site 3: /native-testing-surface::main (src/main.trn:11:5-11:27) */
        { Site { function: 1, file: 0, line: 11, column: 5, end_line: 11, end_column: 27 } },
        /* terrane-site-row: site 4: /native-testing-surface::main (src/main.trn:12:5-12:45) */
        { Site { function: 1, file: 0, line: 12, column: 5, end_line: 12, end_column: 45 } },
        /* terrane-site-row: site 5: /native-testing-surface::main (src/main.trn:13:5-13:37) */
        { Site { function: 1, file: 0, line: 13, column: 5, end_line: 13, end_column: 37 } },
        /* terrane-site-row: site 6: /native-testing-surface::main (src/main.trn:14:5-14:29) */
        { Site { function: 1, file: 0, line: 14, column: 5, end_line: 14, end_column: 29 } },
        /* terrane-site-row: site 7: /native-testing-surface::main (src/main.trn:15:5-15:31) */
        { Site { function: 1, file: 0, line: 15, column: 5, end_line: 15, end_column: 31 } },
        /* terrane-site-row: site 8: /native-testing-surface::main (src/main.trn:16:5-16:34) */
        { Site { function: 1, file: 0, line: 16, column: 5, end_line: 16, end_column: 34 } },
        /* terrane-site-row: site 9: /core/testing::fail (core/testing.trn:33:5-33:41) */
        { Site { function: 2, file: 1, line: 33, column: 5, end_line: 33, end_column: 41 } },
        /* terrane-site-row: site 10: /core/testing::skip (core/testing.trn:36:5-36:37) */
        { Site { function: 3, file: 1, line: 36, column: 5, end_line: 36, end_column: 37 } },
        /* terrane-site-row: site 11: /core/testing::assert (core/testing.trn:40:9-40:36) */
        { Site { function: 4, file: 1, line: 40, column: 9, end_line: 40, end_column: 36 } },
        /* terrane-site-row: site 12: /core/testing::deny (core/testing.trn:45:9-45:42) */
        { Site { function: 5, file: 1, line: 45, column: 9, end_line: 45, end_column: 42 } },
        /* terrane-site-row: site 13: /core/testing::assert-equal-int (core/testing.trn:50:9-50:45) */
        { Site { function: 6, file: 1, line: 50, column: 9, end_line: 50, end_column: 45 } },
        /* terrane-site-row: site 14: /core/testing::assert-not-equal-int (core/testing.trn:55:9-55:41) */
        { Site { function: 7, file: 1, line: 55, column: 9, end_line: 55, end_column: 41 } },
        /* terrane-site-row: site 15: /core/testing::assert-equal-string (core/testing.trn:60:9-60:44) */
        { Site { function: 8, file: 1, line: 60, column: 9, end_line: 60, end_column: 44 } },
        /* terrane-site-row: site 16: /core/testing::assert-not-equal-string (core/testing.trn:65:9-65:40) */
        { Site { function: 9, file: 1, line: 65, column: 9, end_line: 65, end_column: 40 } },
        /* terrane-site-row: site 17: /core/testing::assert-equal-bool (core/testing.trn:70:9-70:45) */
        { Site { function: 10, file: 1, line: 70, column: 9, end_line: 70, end_column: 45 } },
        /* terrane-site-row: site 18: /core/testing::assert-not-equal-bool (core/testing.trn:75:9-75:41) */
        { Site { function: 11, file: 1, line: 75, column: 9, end_line: 75, end_column: 41 } },
        /* terrane-site-row: site 19: /core/testing::assert-present-string (core/testing.trn:80:9-80:42) */
        { Site { function: 12, file: 1, line: 80, column: 9, end_line: 80, end_column: 42 } },
        /* terrane-site-row: site 20: /core/testing::assert-none-string (core/testing.trn:85:9-85:35) */
        { Site { function: 13, file: 1, line: 85, column: 9, end_line: 85, end_column: 35 } },
        /* terrane-site-row: site 21: /core/testing::assert-near (core/testing.trn:90:9-90:61) */
        { Site { function: 14, file: 1, line: 90, column: 9, end_line: 90, end_column: 61 } },
        /* terrane-site-row: site 22: /core/testing::assert-near (core/testing.trn:95:9-95:56) */
        { Site { function: 14, file: 1, line: 95, column: 9, end_line: 95, end_column: 56 } },
        /* terrane-site-row: site 23: /core/testing::assert-throws (core/testing.trn:100:9-100:19) */
        { Site { function: 15, file: 1, line: 100, column: 9, end_line: 100, end_column: 19 } },
        /* terrane-site-row: site 24: /core/testing::assert-throws (core/testing.trn:103:5-103:39) */
        { Site { function: 15, file: 1, line: 103, column: 5, end_line: 103, end_column: 39 } },
        /* terrane-site-row: site 25: /core/testing::run-process (core/testing.trn:165:9-165:46) */
        { Site { function: 16, file: 1, line: 165, column: 9, end_line: 165, end_column: 46 } },
        /* terrane-site-row: site 26: /core/testing::advance-time (core/testing.trn:170:9-170:49) */
        { Site { function: 17, file: 1, line: 170, column: 9, end_line: 170, end_column: 49 } },
        /* terrane-site-row: site 27: /core/testing::advance-time (core/testing.trn:172:9-172:69) */
        { Site { function: 17, file: 1, line: 172, column: 9, end_line: 172, end_column: 69 } },
        /* terrane-site-row: site 28: /core/process::arguments (core/process.trn:45:49-45:63) */
        { Site { function: 18, file: 2, line: 45, column: 49, end_line: 45, end_column: 63 } },
        /* terrane-site-row: site 29: /core/process::environment (core/process.trn:54:40-54:54) */
        { Site { function: 19, file: 2, line: 54, column: 40, end_line: 54, end_column: 54 } },
        /* terrane-site-row: site 30: /core/process::environment (core/process.trn:55:41-55:59) */
        { Site { function: 19, file: 2, line: 55, column: 41, end_line: 55, end_column: 59 } },
        /* terrane-site-row: site 31: /core/process::parse-command-line (core/process.trn:90:20-90:35) */
        { Site { function: 20, file: 2, line: 90, column: 20, end_line: 90, end_column: 35 } },
        /* terrane-site-row: site 32: /core/process::parse-command-line (core/process.trn:105:43-105:62) */
        { Site { function: 20, file: 2, line: 105, column: 43, end_line: 105, end_column: 62 } },
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
// Namespace: native-testing-surface
fn expected_error() -> Result<(), TerraneError> {
    __terrane_traced_err(
        fail(String::from("expected")),
        0 /* terrane-site: src/main.trn:5:5-5:21 */,
    )?;
    return Ok(());
}
fn main() {
    __terrane_traced(assert(true), 1 /* terrane-site: src/main.trn:9:5-9:17 */);
    __terrane_traced(deny(false), 2 /* terrane-site: src/main.trn:10:5-10:16 */);
    __terrane_traced(
        assert_equal_int(
            terrane_int_support::Int::from(4_i128),
            terrane_int_support::Int::from(4_i128),
        ),
        3 /* terrane-site: src/main.trn:11:5-11:27 */,
    );
    __terrane_traced(
        assert_not_equal_string(String::from("left"), String::from("right")),
        4 /* terrane-site: src/main.trn:12:5-12:45 */,
    );
    __terrane_traced(
        assert_present_string(Some(String::from("present"))),
        5 /* terrane-site: src/main.trn:13:5-13:37 */,
    );
    __terrane_traced(
        assert_none_string(None),
        6 /* terrane-site: src/main.trn:14:5-14:29 */,
    );
    __terrane_traced(
        assert_near(1.0, 1.1, 0.2),
        7 /* terrane-site: src/main.trn:15:5-15:31 */,
    );
    __terrane_traced(
        assert_throws(std::sync::Arc::new(expected_error)),
        8 /* terrane-site: src/main.trn:16:5-16:34 */,
    );
    temporary_directory();
    return ();
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
        self.message = format!(
            "{}{}",
            terrane_scalar_support::scalar_text(&String::from("TERRANE_TEST_FAILURE:")),
            terrane_scalar_support::scalar_text(&failure_message)
        );
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
        self.message = format!(
            "{}{}",
            terrane_scalar_support::scalar_text(&String::from("TERRANE_TEST_SKIP:")),
            terrane_scalar_support::scalar_text(&reason)
        );
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
pub fn fail(message: String) -> Result<(), TerraneError> {
    return Err({
        let value = TestFailure::terrane_construct(message);
        TerraneError::raised_with_message(
            TerraneErrorKind::Custom(DescriptorId(0)),
            value.render(),
            9 /* terrane-site: core/testing.trn:33:5-33:41 */,
        )
    });
}
pub fn skip(reason: String) -> Result<(), TerraneError> {
    return Err({
        let value = TestSkip::terrane_construct(reason);
        TerraneError::raised_with_message(
            TerraneErrorKind::Custom(DescriptorId(1)),
            value.render(),
            10 /* terrane-site: core/testing.trn:36:5-36:37 */,
        )
    });
}
pub fn assert(condition: bool) -> Result<(), TerraneError> {
    if !condition {
        __terrane_traced_err(
            fail(String::from("assertion was false")),
            11 /* terrane-site: core/testing.trn:40:9-40:36 */,
        )?;
    }
    return Ok(());
}
pub fn deny(condition: bool) -> Result<(), TerraneError> {
    if condition {
        __terrane_traced_err(
            fail(String::from("denied condition was true")),
            12 /* terrane-site: core/testing.trn:45:9-45:42 */,
        )?;
    }
    return Ok(());
}
pub fn assert_equal_int(
    actual: terrane_int_support::Int,
    expected: terrane_int_support::Int,
) -> Result<(), TerraneError> {
    if actual.clone() != expected.clone() {
        __terrane_traced_err(
            fail(String::from("integer values are not equal")),
            13 /* terrane-site: core/testing.trn:50:9-50:45 */,
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
            fail(String::from("integer values are equal")),
            14 /* terrane-site: core/testing.trn:55:9-55:41 */,
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
            fail(String::from("string values are not equal")),
            15 /* terrane-site: core/testing.trn:60:9-60:44 */,
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
            fail(String::from("string values are equal")),
            16 /* terrane-site: core/testing.trn:65:9-65:40 */,
        )?;
    }
    return Ok(());
}
pub fn assert_equal_bool(actual: bool, expected: bool) -> Result<(), TerraneError> {
    if actual != expected {
        __terrane_traced_err(
            fail(String::from("boolean values are not equal")),
            17 /* terrane-site: core/testing.trn:70:9-70:45 */,
        )?;
    }
    return Ok(());
}
pub fn assert_not_equal_bool(actual: bool, expected: bool) -> Result<(), TerraneError> {
    if actual == expected {
        __terrane_traced_err(
            fail(String::from("boolean values are equal")),
            18 /* terrane-site: core/testing.trn:75:9-75:41 */,
        )?;
    }
    return Ok(());
}
pub fn assert_present_string(actual: Option<String>) -> Result<(), TerraneError> {
    if actual.is_none() {
        __terrane_traced_err(
            fail(String::from("expected a present string")),
            19 /* terrane-site: core/testing.trn:80:9-80:42 */,
        )?;
    }
    return Ok(());
}
pub fn assert_none_string(actual: Option<String>) -> Result<(), TerraneError> {
    if actual.is_some() {
        __terrane_traced_err(
            fail(String::from("expected no string")),
            20 /* terrane-site: core/testing.trn:85:9-85:35 */,
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
            fail(String::from("near-equality tolerance must not be negative")),
            21 /* terrane-site: core/testing.trn:90:9-90:61 */,
        )?;
    }
    let mut difference: f64 = actual - expected;
    if difference < 0.0 {
        difference = 0.0 - difference;
    }
    if difference > tolerance {
        __terrane_traced_err(
            fail(String::from("floating values differ beyond tolerance")),
            22 /* terrane-site: core/testing.trn:95:9-95:56 */,
        )?;
    }
    return Ok(());
}
pub fn assert_throws(
    operation: std::sync::Arc<dyn Fn() -> Result<(), TerraneError> + Send + Sync>,
) -> Result<(), TerraneError> {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                operation(), 23 /* terrane-site: core/testing.trn:100:9-100:19 */
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
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    return TerraneCompletion::Return(());
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
        fail(String::from("expected callback to throw")),
        24 /* terrane-site: core/testing.trn:103:5-103:39 */,
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
pub fn temporary_directory() -> Option<String> {
    return context_value(String::from("TMPDIR"));
}
pub fn deterministic_seed() -> Option<String> {
    return context_value(String::from("TERRANE_TEST_SEED"));
}
pub fn test_identity() -> Option<String> {
    return context_value(String::from("TERRANE_TEST_ID"));
}
pub fn application_artifact() -> String {
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &environment(),
    );
    loop {
        let entry = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry.name.is_text && entry.value.is_text
            && entry.name.text == String::from("TERRANE_TEST_ARTIFACT")
        {
            return entry.value.text.clone();
        }
    }
    return String::from("");
}
pub fn test_tier() -> Option<String> {
    return context_value(String::from("TERRANE_TEST_TIER"));
}
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
}
impl ProcessResult {
    pub fn terrane_construct(
        status: terrane_int_support::Int,
        did_crash: bool,
        exceeded_deadline: bool,
        output: Vec<u8>,
        errors: Vec<u8>,
    ) -> Self {
        let mut value = Self {
            exit_code: terrane_int_support::Int::from(0_i128),
            crashed: false,
            timed_out: false,
            stdout: Vec::from([]),
            stderr: Vec::from([]),
        };
        value.construct(status, did_crash, exceeded_deadline, output, errors);
        value
    }
    pub fn construct(
        &mut self,
        status: terrane_int_support::Int,
        did_crash: bool,
        exceeded_deadline: bool,
        output: Vec<u8>,
        errors: Vec<u8>,
    ) {
        self.exit_code = status.clone();
        self.crashed = did_crash;
        self.timed_out = exceeded_deadline;
        self.stdout = output;
        self.stderr = errors;
    }
}
pub fn run_process(fixture: ProcessFixture) -> Result<ProcessResult, TerraneError> {
    let mut encoded_arguments: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &fixture.arguments,
    );
    {
        let __terrane_list_append_0 = encoded_arguments.make_unique();
        loop {
            let argument = match __terrane_iterator_2.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_0.push(encode_native_string(argument));
        }
    }
    let mut encoded_environment: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
        &fixture.environment,
    );
    {
        let __terrane_list_append_1 = encoded_environment.make_unique();
        loop {
            let entry = match __terrane_iterator_3.next() {
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
        __terrane_traced_err(
            fail(terrane_test_result_message(&raw)),
            25 /* terrane-site: core/testing.trn:165:9-165:46 */,
        )?;
    }
    return Ok(
        ProcessResult::terrane_construct(
            terrane_test_result_exit_code(&raw),
            terrane_test_result_crashed(&raw),
            terrane_test_result_deadline_exceeded(&raw),
            terrane_test_result_stdout(&raw),
            terrane_test_result_stderr(&raw),
        ),
    );
}
pub fn advance_time(nanoseconds: terrane_int_support::Int) -> Result<(), TerraneError> {
    if nanoseconds.clone() < terrane_int_support::Int::from(0_i128) {
        __terrane_traced_err(
            fail(String::from("controlled time may only advance")),
            26 /* terrane-site: core/testing.trn:170:9-170:49 */,
        )?;
    }
    if !terrane_test_time_advance(nanoseconds.clone()) {
        __terrane_traced_err(
            fail(String::from("controlled time advance exceeds the host clock range")),
            27 /* terrane-site: core/testing.trn:172:9-172:69 */,
        )?;
    }
    return Ok(());
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
                                        28 /* terrane-site: core/process.trn:45:49-45:63 */,
                                    ),
                                )
                                .cloned()
                                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        28 /* terrane-site: core/process.trn:45:49-45:63 */,
                                    ),
                                )),
                            28 /* terrane-site: core/process.trn:45:49-45:63 */,
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
                                29 /* terrane-site: core/process.trn:54:40-54:54 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                29 /* terrane-site: core/process.trn:54:40-54:54 */,
                            ),
                        )),
                    29 /* terrane-site: core/process.trn:54:40-54:54 */,
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
                                30 /* terrane-site: core/process.trn:55:41-55:59 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                30 /* terrane-site: core/process.trn:55:41-55:59 */,
                            ),
                        )),
                    30 /* terrane-site: core/process.trn:55:41-55:59 */,
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
                            31 /* terrane-site: core/process.trn:90:20-90:35 */,
                        ),
                    ),
                31 /* terrane-site: core/process.trn:90:20-90:35 */,
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
                                                32 /* terrane-site: core/process.trn:105:43-105:62 */,
                                            ),
                                        ),
                                    32 /* terrane-site: core/process.trn:105:43-105:62 */,
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
pub fn environment_pair(key: NativeString, item: NativeString) -> EnvironmentEntry {
    return EnvironmentEntry::terrane_construct(key.clone(), item.clone());
}
pub fn encode_native_string(value: NativeString) -> String {
    if value.is_text {
        return terrane_platform_value_from_text(&value.text);
    }
    return terrane_platform_value_from_bytes(&value.raw);
}
