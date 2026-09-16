// Generated deterministically by Terrane <version>.
// Runtime support: mutable_callable.rs, platform_result_type.rs, platform_process.rs
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
    pub static DESCRIPTORS: [&str; 0] = [];
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
    pub static FILES: [&str; 2] = ["case.trn", "core/process.trn"];
    pub static FUNCTIONS: [&str; 7] = [
        "/list-append-bulk-mutation::validate-return",
        "/list-append-bulk-mutation::validate-exit",
        "/list-append-bulk-mutation::validate-throw",
        "/list-append-bulk-mutation::main",
        "/core/process::arguments",
        "/core/process::environment",
        "/core/process::parse-command-line",
    ];
    pub static SITES: [Site; 40] = [
        /* terrane-site-row: site 0: /list-append-bulk-mutation::validate-return (case.trn:23:14-23:23) */
        { Site { function: 0, file: 0, line: 23, column: 14, end_line: 23, end_column: 23 } },
        /* terrane-site-row: site 1: /list-append-bulk-mutation::validate-return (case.trn:24:5-24:12) */
        { Site { function: 0, file: 0, line: 24, column: 5, end_line: 24, end_column: 12 } },
        /* terrane-site-row: site 2: /list-append-bulk-mutation::validate-exit (case.trn:34:14-34:23) */
        { Site { function: 1, file: 0, line: 34, column: 14, end_line: 34, end_column: 23 } },
        /* terrane-site-row: site 3: /list-append-bulk-mutation::validate-exit (case.trn:36:5-36:12) */
        { Site { function: 1, file: 0, line: 36, column: 5, end_line: 36, end_column: 12 } },
        /* terrane-site-row: site 4: /list-append-bulk-mutation::validate-throw (case.trn:46:16-46:25) */
        { Site { function: 2, file: 0, line: 46, column: 16, end_line: 46, end_column: 25 } },
        /* terrane-site-row: site 5: /list-append-bulk-mutation::validate-throw (case.trn:47:9-47:34) */
        { Site { function: 2, file: 0, line: 47, column: 9, end_line: 47, end_column: 34 } },
        /* terrane-site-row: site 6: /list-append-bulk-mutation::validate-throw (case.trn:48:7-48:14) */
        { Site { function: 2, file: 0, line: 48, column: 7, end_line: 48, end_column: 14 } },
        /* terrane-site-row: site 7: /list-append-bulk-mutation::main (case.trn:63:5-63:12) */
        { Site { function: 3, file: 0, line: 63, column: 5, end_line: 63, end_column: 12 } },
        /* terrane-site-row: site 8: /list-append-bulk-mutation::main (case.trn:73:41-73:52) */
        { Site { function: 3, file: 0, line: 73, column: 41, end_line: 73, end_column: 52 } },
        /* terrane-site-row: site 9: /list-append-bulk-mutation::main (case.trn:75:31-75:46) */
        { Site { function: 3, file: 0, line: 75, column: 31, end_line: 75, end_column: 46 } },
        /* terrane-site-row: site 10: /list-append-bulk-mutation::main (case.trn:82:37-82:58) */
        { Site { function: 3, file: 0, line: 82, column: 37, end_line: 82, end_column: 58 } },
        /* terrane-site-row: site 11: /list-append-bulk-mutation::main (case.trn:99:5-99:19) */
        { Site { function: 3, file: 0, line: 99, column: 5, end_line: 99, end_column: 19 } },
        /* terrane-site-row: site 12: /list-append-bulk-mutation::main (case.trn:97:47-97:61) */
        { Site { function: 3, file: 0, line: 97, column: 47, end_line: 97, end_column: 61 } },
        /* terrane-site-row: site 13: /list-append-bulk-mutation::main (case.trn:105:23-105:39) */
        { Site { function: 3, file: 0, line: 105, column: 23, end_line: 105, end_column: 39 } },
        /* terrane-site-row: site 14: /list-append-bulk-mutation::main (case.trn:135:5-135:18) */
        { Site { function: 3, file: 0, line: 135, column: 5, end_line: 135, end_column: 18 } },
        /* terrane-site-row: site 15: /list-append-bulk-mutation::main (case.trn:152:28-152:40) */
        { Site { function: 3, file: 0, line: 152, column: 28, end_line: 152, end_column: 40 } },
        /* terrane-site-row: site 16: /list-append-bulk-mutation::main (case.trn:156:22-156:37) */
        { Site { function: 3, file: 0, line: 156, column: 22, end_line: 156, end_column: 37 } },
        /* terrane-site-row: site 17: /list-append-bulk-mutation::main (case.trn:157:10-157:21) */
        { Site { function: 3, file: 0, line: 157, column: 10, end_line: 157, end_column: 21 } },
        /* terrane-site-row: site 18: /list-append-bulk-mutation::main (case.trn:164:28-164:40) */
        { Site { function: 3, file: 0, line: 164, column: 28, end_line: 164, end_column: 40 } },
        /* terrane-site-row: site 19: /list-append-bulk-mutation::main (case.trn:182:5-182:21) */
        { Site { function: 3, file: 0, line: 182, column: 5, end_line: 182, end_column: 21 } },
        /* terrane-site-row: site 20: /list-append-bulk-mutation::main (case.trn:189:28-189:40) */
        { Site { function: 3, file: 0, line: 189, column: 28, end_line: 189, end_column: 40 } },
        /* terrane-site-row: site 21: /list-append-bulk-mutation::main (case.trn:191:5-191:19) */
        { Site { function: 3, file: 0, line: 191, column: 5, end_line: 191, end_column: 19 } },
        /* terrane-site-row: site 22: /list-append-bulk-mutation::main (case.trn:192:25-192:34) */
        { Site { function: 3, file: 0, line: 192, column: 25, end_line: 192, end_column: 34 } },
        /* terrane-site-row: site 23: /list-append-bulk-mutation::main (case.trn:200:41-200:66) */
        { Site { function: 3, file: 0, line: 200, column: 41, end_line: 200, end_column: 66 } },
        /* terrane-site-row: site 24: /list-append-bulk-mutation::main (case.trn:200:68-200:93) */
        { Site { function: 3, file: 0, line: 200, column: 68, end_line: 200, end_column: 93 } },
        /* terrane-site-row: site 25: /list-append-bulk-mutation::main (case.trn:209:44-209:72) */
        { Site { function: 3, file: 0, line: 209, column: 44, end_line: 209, end_column: 72 } },
        /* terrane-site-row: site 26: /list-append-bulk-mutation::main (case.trn:209:74-209:102) */
        { Site { function: 3, file: 0, line: 209, column: 74, end_line: 209, end_column: 102 } },
        /* terrane-site-row: site 27: /list-append-bulk-mutation::main (case.trn:228:33-228:50) */
        { Site { function: 3, file: 0, line: 228, column: 33, end_line: 228, end_column: 50 } },
        /* terrane-site-row: site 28: /list-append-bulk-mutation::main (case.trn:235:27-235:38) */
        { Site { function: 3, file: 0, line: 235, column: 27, end_line: 235, end_column: 38 } },
        /* terrane-site-row: site 29: /list-append-bulk-mutation::main (case.trn:242:7-242:23) */
        { Site { function: 3, file: 0, line: 242, column: 7, end_line: 242, end_column: 23 } },
        /* terrane-site-row: site 30: /list-append-bulk-mutation::main (case.trn:250:5-250:18) */
        { Site { function: 3, file: 0, line: 250, column: 5, end_line: 250, end_column: 18 } },
        /* terrane-site-row: site 31: /list-append-bulk-mutation::main (case.trn:260:33-260:50) */
        { Site { function: 3, file: 0, line: 260, column: 33, end_line: 260, end_column: 50 } },
        /* terrane-site-row: site 32: /list-append-bulk-mutation::main (case.trn:260:76-260:94) */
        { Site { function: 3, file: 0, line: 260, column: 76, end_line: 260, end_column: 94 } },
        /* terrane-site-row: site 33: /list-append-bulk-mutation::main (case.trn:264:28-264:51) */
        { Site { function: 3, file: 0, line: 264, column: 28, end_line: 264, end_column: 51 } },
        /* terrane-site-row: site 34: /list-append-bulk-mutation::main (case.trn:267:24-267:55) */
        { Site { function: 3, file: 0, line: 267, column: 24, end_line: 267, end_column: 55 } },
        /* terrane-site-row: site 35: /core/process::arguments (core/process.trn:45:49-45:63) */
        { Site { function: 4, file: 1, line: 45, column: 49, end_line: 45, end_column: 63 } },
        /* terrane-site-row: site 36: /core/process::environment (core/process.trn:54:40-54:54) */
        { Site { function: 5, file: 1, line: 54, column: 40, end_line: 54, end_column: 54 } },
        /* terrane-site-row: site 37: /core/process::environment (core/process.trn:55:41-55:59) */
        { Site { function: 5, file: 1, line: 55, column: 41, end_line: 55, end_column: 59 } },
        /* terrane-site-row: site 38: /core/process::parse-command-line (core/process.trn:90:20-90:35) */
        { Site { function: 6, file: 1, line: 90, column: 20, end_line: 90, end_column: 35 } },
        /* terrane-site-row: site 39: /core/process::parse-command-line (core/process.trn:105:43-105:62) */
        { Site { function: 6, file: 1, line: 105, column: 43, end_line: 105, end_column: 62 } },
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
// Source: case.trn
// Namespace: list-append-bulk-mutation
fn validate_large_literal(enabled: bool) {
    if enabled {
        let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
            i64,
        >::new(Vec::new());
        let mut index: i64 = 0;
        {
            let __terrane_list_append_0 = values.make_unique();
            let __terrane_list_start_0 = index;
            let __terrane_list_end_0 = 4000000000 as i64;
            let __terrane_list_length_0 = (__terrane_list_start_0..__terrane_list_end_0)
                .size_hint()
                .0;
            let __terrane_list_capacity_limit_0 = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            if __terrane_list_length_0 <= __terrane_list_capacity_limit_0 {
                *__terrane_list_append_0 = (__terrane_list_start_0..__terrane_list_end_0)
                    .map(|index| { index })
                    .collect();
                index = std::cmp::max(__terrane_list_start_0, __terrane_list_end_0);
            } else {
                __terrane_list_append_0.reserve(__terrane_list_capacity_limit_0);
                while index < 4000000000 {
                    __terrane_list_append_0.push(index);
                    index = index + 1;
                }
            }
            let _ = &index;
        }
    }
}
fn validate_return() -> terrane_int_support::Int {
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 100000000000000;
    {
        let __terrane_list_append_1 = values.make_unique();
        while index < limit {
            __terrane_list_append_1.push(index);
            if index > 2 {
                return terrane_int_support::Int::from(
                    __terrane_raised(
                        terrane_int_support::fixed_addition(index, 1),
                        0 /* terrane-site: case.trn:23:14-23:23 */,
                    ) as i128,
                );
            }
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                1 /* terrane-site: case.trn:24:5-24:12 */,
            );
        }
    }
    return terrane_int_support::Int::from(0_i128);
}
fn validate_exit() {
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 100000000000000;
    {
        let __terrane_list_append_2 = values.make_unique();
        while index < limit {
            __terrane_list_append_2.push(index);
            if index > 2 {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_addition(index,
                    1), 2 /* terrane-site: case.trn:34:14-34:23 */))
                );
                exit(make_exit_status(terrane_int_support::Int::from(0_i128)));
            }
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                3 /* terrane-site: case.trn:36:5-36:12 */,
            );
        }
    }
}
fn validate_throw() {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
                i64,
            >::new(Vec::new());
            let mut index: i64 = 0;
            let limit: i64 = 100000000000000;
            {
                let __terrane_list_append_3 = values.make_unique();
                while index < limit {
                    __terrane_list_append_3.push(index);
                    if index > 2 {
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_int_support::fixed_addition(index,
                            1), 4 /* terrane-site: case.trn:46:16-46:25 */))
                        );
                        return TerraneCompletion::Error(
                            TerraneError::raised(
                                TerraneErrorKind::ArithmeticOverflow,
                                5 /* terrane-site: case.trn:47:9-47:34 */,
                            ),
                        );
                    }
                    index = __terrane_raised_completion!(
                        terrane_int_support::fixed_addition(index, 1),
                        6 /* terrane-site: case.trn:48:7-48:14 */
                    );
                }
            }
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
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("throw-caught"))
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
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn observe_prefix(value: i64) -> i64 {
    println!("{}", terrane_scalar_support::scalar_text(&value));
    return value;
}
fn main() {
    validate_large_literal(false);
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 4;
    {
        let __terrane_list_append_4 = values.make_unique();
        let __terrane_list_start_1 = index;
        let __terrane_list_end_1 = limit;
        let __terrane_list_length_1 = (__terrane_list_start_1..__terrane_list_end_1)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_1 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_1 <= __terrane_list_capacity_limit_1 {
            *__terrane_list_append_4 = (__terrane_list_start_1..__terrane_list_end_1)
                .map(|index| { index })
                .collect();
            index = std::cmp::max(__terrane_list_start_1, __terrane_list_end_1);
        } else {
            __terrane_list_append_4.reserve(__terrane_list_capacity_limit_1);
            while index < limit {
                __terrane_list_append_4.push(index);
                index = __terrane_raised(
                    terrane_int_support::fixed_addition(index, 1),
                    7 /* terrane-site: case.trn:63:5-63:12 */,
                );
            }
        }
        let _ = &index;
    }
    let original: terrane_collection_support::List<i64> = values.clone();
    values.append(9);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(original
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    let mut three_clause: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut for_index: i64 = 0;
    println!("{}", terrane_scalar_support::scalar_text(&for_index));
    for_index = 0;
    {
        let __terrane_list_append_5 = three_clause.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(for_index),
            usize::try_from(limit),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_5
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        '__terrane_break_2: while for_index < limit {
            '__terrane_continue_2: {
                __terrane_list_append_5.push(for_index);
            }
            for_index = __terrane_raised(
                terrane_int_support::fixed_addition(for_index, 1),
                8 /* terrane-site: case.trn:73:41-73:52 */,
            );
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(three_clause
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(three_clause
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        9 /* terrane-site: case.trn:75:31-75:46 */)), 9 /* terrane-site: case.trn:75:31-75:46 */)), terrane_scalar_support::scalar_text(&for_index)
    );
    let mut three_clause_break: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut break_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    {
        let __terrane_list_append_6 = three_clause_break.make_unique();
        '__terrane_break_3: while break_index.clone()
            < terrane_int_support::Int::from(10_i128)
        {
            '__terrane_continue_3: {
                __terrane_list_append_6.push(break_index.clone());
                if break_index.clone() > terrane_int_support::Int::from(2_i128) {
                    break '__terrane_break_3;
                }
            }
            break_index = break_index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(three_clause_break
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(three_clause_break
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        10 /* terrane-site: case.trn:82:37-82:58 */)), 10 /* terrane-site: case.trn:82:37-82:58 */))
    );
    let mut update_observed: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut update_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    '__terrane_break_4: while update_index.clone()
        < terrane_int_support::Int::from(3_i128)
    {
        '__terrane_continue_4: {
            update_observed.append(update_index.clone());
        }
        update_index = update_index.clone() + terrane_int_support::Int::from(1_i128)
            + terrane_int_support::Int::from(
                terrane_int_support::Int::from(update_observed.length()),
            )
            - terrane_int_support::Int::from(
                terrane_int_support::Int::from(update_observed.length()),
            );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(update_observed
        .length()))
    );
    let mut condition_observed: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut condition_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    '__terrane_break_5: while condition_index.clone()
        < terrane_int_support::Int::from(4_i128)
            + terrane_int_support::Int::from(
                terrane_int_support::Int::from(condition_observed.length()),
            )
            - terrane_int_support::Int::from(
                terrane_int_support::Int::from(condition_observed.length()),
            )
    {
        '__terrane_continue_5: {
            condition_observed.append(condition_index.clone());
        }
        condition_index = condition_index.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(condition_observed
        .length()))
    );
    let mut double_index: i64 = 0;
    println!("{}", terrane_scalar_support::scalar_text(&double_index));
    let mut double_update: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    double_index = 0;
    {
        let __terrane_list_append_7 = double_update.make_unique();
        '__terrane_break_6: while double_index < limit {
            '__terrane_continue_6: {
                __terrane_list_append_7.push(double_index);
                double_index = __terrane_raised(
                    terrane_int_support::fixed_addition(double_index, 1),
                    11 /* terrane-site: case.trn:99:5-99:19 */,
                );
            }
            double_index = __terrane_raised(
                terrane_int_support::fixed_addition(double_index, 1),
                12 /* terrane-site: case.trn:97:47-97:61 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(double_update
        .length()))
    );
    let mut dependent: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut dependent_index: i64 = 0;
    while dependent_index < 3 {
        dependent
            .append(
                __terrane_raised(
                    terrane_int_support::coerce::<
                        i64,
                    >(&terrane_int_support::Int::from(dependent.length())),
                    13 /* terrane-site: case.trn:105:23-105:39 */,
                ),
            );
        dependent_index = dependent_index + 1;
    }
    let __terrane_iterable_7 = dependent;
    let mut __terrane_iterator_7 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_7,
    );
    loop {
        let value = match __terrane_iterator_7.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    let mut inner_index: i64 = 0;
    while inner_index < 1 {
        let mut inner: terrane_collection_support::List<i64> = terrane_collection_support::List::<
            i64,
        >::new(Vec::new());
        inner.append(inner_index);
        inner_index = inner_index + 1;
    }
    let mut nested: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut outer_index: i64 = 0;
    {
        let __terrane_list_append_8 = nested.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(outer_index),
            usize::try_from(3 as i64),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_8
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while outer_index < 3 {
            __terrane_list_append_8.push(outer_index);
            let mut nested_index: i64 = 0;
            while nested_index < 2 {
                __terrane_list_append_8.push(nested_index);
                nested_index = nested_index + 1;
            }
            outer_index = outer_index + 1;
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested
        .length()))
    );
    let mut early_exit: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut early_index: i64 = 0;
    let early_limit: i64 = 100000000000000;
    {
        let __terrane_list_append_9 = early_exit.make_unique();
        while early_index < early_limit {
            __terrane_list_append_9.push(early_index);
            if early_index > 2 {
                break;
            }
            early_index = __terrane_raised(
                terrane_int_support::fixed_addition(early_index, 1),
                14 /* terrane-site: case.trn:135:5-135:18 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(early_exit
        .length()))
    );
    let mut nested_break: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut break_outer: i64 = 0;
    {
        let __terrane_list_append_10 = nested_break.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(break_outer),
            usize::try_from(3 as i64),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_10
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while break_outer < 3 {
            __terrane_list_append_10.push(break_outer);
            let break_inner: i64 = 0;
            while break_inner < 3 {
                break;
            }
            break_outer = break_outer + 1;
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested_break
        .length()))
    );
    let source: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![1, 2, 3, 4]);
    let mut collected: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let __terrane_iterable_8 = source.clone();
    let mut __terrane_iterator_8 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_8,
    );
    {
        let __terrane_list_append_11 = collected.make_unique();
        loop {
            let value = match __terrane_iterator_8.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_11.push(value);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(collected
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(collected
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        15 /* terrane-site: case.trn:152:28-152:40 */)), 15 /* terrane-site: case.trn:152:28-152:40 */))
    );
    let mut observed: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let __terrane_iterable_9 = source;
    let mut __terrane_iterator_9 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_9,
    );
    loop {
        let value = match __terrane_iterator_9.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value;
        observed
            .append(
                __terrane_raised(
                    terrane_int_support::coerce::<
                        i64,
                    >(&terrane_int_support::Int::from(observed.length())),
                    16 /* terrane-site: case.trn:156:22-156:37 */,
                ),
            );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(observed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        17 /* terrane-site: case.trn:157:10-157:21 */)), 17 /* terrane-site: case.trn:157:10-157:21 */))
    );
    let rows: terrane_collection_support::List<terrane_collection_support::List<i64>> = terrane_collection_support::List::<
        terrane_collection_support::List<i64>,
    >::new(
        vec![
            terrane_collection_support::List::< i64 >::new(vec![1, 2]),
            terrane_collection_support::List::< i64 >::new(vec![3])
        ],
    );
    let mut flattened: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let __terrane_iterable_10 = rows;
    let mut __terrane_iterator_10 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_10,
    );
    {
        let __terrane_list_append_12 = flattened.make_unique();
        loop {
            let row = match __terrane_iterator_10.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let __terrane_iterable_11 = row;
            let mut __terrane_iterator_11 = terrane_collection_support::Iterable::terrane_iterator(
                &__terrane_iterable_11,
            );
            loop {
                let value = match __terrane_iterator_11.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                __terrane_list_append_12.push(value);
            }
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(flattened
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(flattened
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        18 /* terrane-site: case.trn:164:28-164:40 */)), 18 /* terrane-site: case.trn:164:28-164:40 */))
    );
    let mut aliased: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![1, 2]);
    let alias: terrane_collection_support::List<i64> = aliased.clone();
    let __terrane_iterable_12 = alias.clone();
    let mut __terrane_iterator_12 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_12,
    );
    {
        let __terrane_list_append_13 = aliased.make_unique();
        loop {
            let value = match __terrane_iterator_12.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_13.push(value);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(aliased
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(alias
        .length()))
    );
    let mut self_source: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![1, 2]);
    let __terrane_iterable_13 = self_source.clone();
    let mut __terrane_iterator_13 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_13,
    );
    loop {
        let value = match __terrane_iterator_13.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        self_source.append(value);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(self_source
        .length()))
    );
    let mut negative: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut negative_index: i64 = 0;
    let negative_limit: i64 = -3;
    {
        let __terrane_list_append_14 = negative.make_unique();
        let __terrane_list_start_14 = negative_index;
        let __terrane_list_end_14 = negative_limit;
        let __terrane_list_length_14 = (__terrane_list_start_14..__terrane_list_end_14)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_14 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_14 <= __terrane_list_capacity_limit_14 {
            *__terrane_list_append_14 = (__terrane_list_start_14..__terrane_list_end_14)
                .map(|negative_index| { negative_index })
                .collect();
            negative_index = std::cmp::max(
                __terrane_list_start_14,
                __terrane_list_end_14,
            );
        } else {
            __terrane_list_append_14.reserve(__terrane_list_capacity_limit_14);
            while negative_index < negative_limit {
                __terrane_list_append_14.push(negative_index);
                negative_index = __terrane_raised(
                    terrane_int_support::fixed_addition(negative_index, 1),
                    19 /* terrane-site: case.trn:182:5-182:21 */,
                );
            }
        }
        let _ = &negative_index;
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(negative
        .length())), terrane_scalar_support::scalar_text(&negative_index)
    );
    let mut mapped: terrane_collection_support::List<f64> = terrane_collection_support::List::<
        f64,
    >::new(Vec::new());
    let mut mapped_index: i64 = 0;
    let mapped_limit: i64 = 4;
    {
        let __terrane_list_append_15 = mapped.make_unique();
        let __terrane_list_start_15 = mapped_index;
        let __terrane_list_end_15 = mapped_limit;
        let __terrane_list_length_15 = (__terrane_list_start_15..__terrane_list_end_15)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_15 = 268435456usize
            / std::mem::size_of::<f64>().max(1);
        if __terrane_list_length_15 <= __terrane_list_capacity_limit_15 {
            *__terrane_list_append_15 = (__terrane_list_start_15..__terrane_list_end_15)
                .map(|mapped_index| {
                    let mapped_value: f64 = __terrane_raised(
                        terrane_int_support::exact_fixed_f64(mapped_index),
                        20 /* terrane-site: case.trn:189:28-189:40 */,
                    );
                    mapped_value * mapped_value
                })
                .collect();
            mapped_index = std::cmp::max(__terrane_list_start_15, __terrane_list_end_15);
        } else {
            __terrane_list_append_15.reserve(__terrane_list_capacity_limit_15);
            while mapped_index < mapped_limit {
                let mapped_value: f64 = __terrane_raised(
                    terrane_int_support::exact_fixed_f64(mapped_index),
                    20 /* terrane-site: case.trn:189:28-189:40 */,
                );
                __terrane_list_append_15.push(mapped_value * mapped_value);
                mapped_index = __terrane_raised(
                    terrane_int_support::fixed_addition(mapped_index, 1),
                    21 /* terrane-site: case.trn:191:5-191:19 */,
                );
            }
        }
        let _ = &mapped_index;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(mapped
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(mapped
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        22 /* terrane-site: case.trn:192:25-192:34 */)), 22 /* terrane-site: case.trn:192:25-192:34 */)), terrane_scalar_support::scalar_text(&mapped_index)
    );
    let mut mutated_before_builder: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    mutated_before_builder.append(7);
    let mut mutated_index: i64 = 0;
    {
        let __terrane_list_append_16 = mutated_before_builder.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(mutated_index),
            usize::try_from(2 as i64),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_16
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while mutated_index < 2 {
            __terrane_list_append_16.push(mutated_index);
            mutated_index = mutated_index + 1;
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(mutated_before_builder
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(mutated_before_builder
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        23 /* terrane-site: case.trn:200:41-200:66 */)), 23 /* terrane-site: case.trn:200:41-200:66 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(mutated_before_builder
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        24 /* terrane-site: case.trn:200:68-200:93 */)), 24 /* terrane-site: case.trn:200:68-200:93 */))
    );
    let mut reassigned_before_builder: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let _ = &mut reassigned_before_builder;
    let replacement: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![8]);
    reassigned_before_builder = replacement;
    let mut reassigned_index: i64 = 0;
    {
        let __terrane_list_append_17 = reassigned_before_builder.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(reassigned_index),
            usize::try_from(2 as i64),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_17
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while reassigned_index < 2 {
            __terrane_list_append_17.push(reassigned_index);
            reassigned_index = reassigned_index + 1;
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(reassigned_before_builder
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(reassigned_before_builder
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        25 /* terrane-site: case.trn:209:44-209:72 */)), 25 /* terrane-site: case.trn:209:44-209:72 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(reassigned_before_builder
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        26 /* terrane-site: case.trn:209:74-209:102 */)), 26 /* terrane-site: case.trn:209:74-209:102 */))
    );
    let mut dynamically_reentered: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut outer_builder_index: i64 = 0;
    while outer_builder_index < 3 {
        let observed_length: terrane_int_support::Int = terrane_int_support::Int::from(
            terrane_int_support::Int::from(dynamically_reentered.length()),
        );
        let _ = &observed_length;
        let mut inner_builder_index: i64 = 0;
        {
            let __terrane_list_append_18 = dynamically_reentered.make_unique();
            if let (Ok(__terrane_start), Ok(__terrane_end)) = (
                usize::try_from(inner_builder_index),
                usize::try_from(2 as i64),
            ) {
                let __terrane_capacity_limit = 268435456usize
                    / std::mem::size_of::<i64>().max(1);
                __terrane_list_append_18
                    .reserve(
                        __terrane_end
                            .saturating_sub(__terrane_start)
                            .min(__terrane_capacity_limit),
                    );
            }
            while inner_builder_index < 2 {
                __terrane_list_append_18.push(inner_builder_index);
                inner_builder_index = inner_builder_index + 1;
            }
        }
        outer_builder_index = outer_builder_index + 1;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(dynamically_reentered
        .length()))
    );
    let mut prefix_effects: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut prefix_index: i64 = 0;
    {
        let __terrane_list_append_19 = prefix_effects.make_unique();
        let __terrane_list_start_16 = prefix_index;
        let __terrane_list_end_16 = 3 as i64;
        let __terrane_list_length_16 = (__terrane_list_start_16..__terrane_list_end_16)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_16 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_16 <= __terrane_list_capacity_limit_16 {
            *__terrane_list_append_19 = (__terrane_list_start_16..__terrane_list_end_16)
                .map(|prefix_index| {
                    let prefix_value: i64 = observe_prefix(prefix_index);
                    prefix_value
                })
                .collect();
            prefix_index = std::cmp::max(__terrane_list_start_16, __terrane_list_end_16);
        } else {
            __terrane_list_append_19.reserve(__terrane_list_capacity_limit_16);
            while prefix_index < 3 {
                let prefix_value: i64 = observe_prefix(prefix_index);
                __terrane_list_append_19.push(prefix_value);
                prefix_index = prefix_index + 1;
            }
        }
        let _ = &prefix_index;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(prefix_effects
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(prefix_effects
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        27 /* terrane-site: case.trn:228:33-228:50 */)), 27 /* terrane-site: case.trn:228:33-228:50 */)), terrane_scalar_support::scalar_text(&prefix_index)
    );
    let mut fallback: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut fallback_index: i64 = 0;
    {
        let __terrane_list_append_20 = fallback.make_unique();
        let __terrane_list_start_17 = fallback_index;
        let __terrane_list_end_17 = 5 as i64;
        let __terrane_list_length_17 = (__terrane_list_start_17..__terrane_list_end_17)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_17 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_17 <= __terrane_list_capacity_limit_17 {
            *__terrane_list_append_20 = (__terrane_list_start_17..__terrane_list_end_17)
                .map(|fallback_index| { fallback_index })
                .collect();
            fallback_index = std::cmp::max(
                __terrane_list_start_17,
                __terrane_list_end_17,
            );
        } else {
            __terrane_list_append_20.reserve(__terrane_list_capacity_limit_17);
            while fallback_index < 5 {
                __terrane_list_append_20.push(fallback_index);
                fallback_index = fallback_index + 1;
            }
        }
        let _ = &fallback_index;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fallback
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(fallback
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        28 /* terrane-site: case.trn:235:27-235:38 */)), 28 /* terrane-site: case.trn:235:27-235:38 */)), terrane_scalar_support::scalar_text(&fallback_index)
    );
    let captured: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut captured_index: i64 = 0;
    let capturing_builder: TerraneMutableCallable<(), ()> = {
        let mut captured = captured.clone();
        let mut captured_index = captured_index.clone();
        TerraneMutableCallable::new(move |(): ()| -> () {
            {
                let __terrane_list_append_21 = captured.make_unique();
                if let (Ok(__terrane_start), Ok(__terrane_end)) = (
                    usize::try_from(captured_index),
                    usize::try_from(3 as i64),
                ) {
                    let __terrane_capacity_limit = 268435456usize
                        / std::mem::size_of::<i64>().max(1);
                    __terrane_list_append_21
                        .reserve(
                            __terrane_end
                                .saturating_sub(__terrane_start)
                                .min(__terrane_capacity_limit),
                        );
                }
                while captured_index < 3 {
                    __terrane_list_append_21.push(captured_index);
                    captured_index = __terrane_raised(
                        terrane_int_support::fixed_addition(captured_index, 1),
                        29 /* terrane-site: case.trn:242:7-242:23 */,
                    );
                }
            }
            ()
        })
    };
    capturing_builder.call(());
    captured_index = 0;
    capturing_builder.call(());
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(captured
        .length())), terrane_scalar_support::scalar_text(&captured_index)
    );
    let local_mutation: TerraneMutableCallable<(), i64> = {
        TerraneMutableCallable::new(move |(): ()| -> i64 {
            let mut local_value: i64 = 0;
            local_value = __terrane_raised(
                terrane_int_support::fixed_addition(local_value, 1),
                30 /* terrane-site: case.trn:250:5-250:18 */,
            );
            return local_value;
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&local_mutation.call(())));
    let returned_capture: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![7, 8]);
    let return_capture: TerraneMutableCallable<
        (),
        terrane_collection_support::List<i64>,
    > = {
        let mut returned_capture = returned_capture.clone();
        TerraneMutableCallable::new(move |
            (): (),
        | -> terrane_collection_support::List<i64> {
            returned_capture.append(9);
            return returned_capture.clone();
        })
    };
    let first_returned: terrane_collection_support::List<i64> = return_capture.call(());
    let second_returned: terrane_collection_support::List<i64> = return_capture.call(());
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(first_returned
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(first_returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        31 /* terrane-site: case.trn:260:33-260:50 */)), 31 /* terrane-site: case.trn:260:33-260:50 */)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(second_returned
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(second_returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        32 /* terrane-site: case.trn:260:76-260:94 */)), 32 /* terrane-site: case.trn:260:76-260:94 */))
    );
    let inferred_capture: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![2, 3]);
    let inspect_capture: TerraneMutableCallable<(), i64> = {
        let mut inferred_capture = inferred_capture.clone();
        TerraneMutableCallable::new(move |(): ()| -> i64 {
            let mut inferred_total: i64 = __terrane_raised(
                terrane_int_support::coerce::<
                    i64,
                >(&terrane_int_support::Int::from(inferred_capture.length())),
                33 /* terrane-site: case.trn:264:28-264:51 */,
            );
            inferred_capture.append(4);
            let __terrane_iterable_18 = inferred_capture.clone();
            let mut __terrane_iterator_18 = terrane_collection_support::Iterable::terrane_iterator(
                &__terrane_iterable_18,
            );
            loop {
                let inferred_value = match __terrane_iterator_18.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                inferred_total = __terrane_raised(
                    terrane_int_support::fixed_addition(inferred_total, inferred_value),
                    34 /* terrane-site: case.trn:267:24-267:55 */,
                );
            }
            return inferred_total;
        })
    };
    println!("{}", terrane_scalar_support::scalar_text(&inspect_capture.call(())));
    println!("{}", terrane_scalar_support::scalar_text(&validate_return()));
    validate_throw();
    validate_exit();
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
        self.name = name;
        self.value = entry_value;
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
        self.value = result_value;
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
                                        35 /* terrane-site: core/process.trn:45:49-45:63 */,
                                    ),
                                )
                                .cloned()
                                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        35 /* terrane-site: core/process.trn:45:49-45:63 */,
                                    ),
                                )),
                            35 /* terrane-site: core/process.trn:45:49-45:63 */,
                        ),
                    ),
                );
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return values;
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
                                36 /* terrane-site: core/process.trn:54:40-54:54 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                36 /* terrane-site: core/process.trn:54:40-54:54 */,
                            ),
                        )),
                    36 /* terrane-site: core/process.trn:54:40-54:54 */,
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
                                37 /* terrane-site: core/process.trn:55:41-55:59 */,
                            ),
                        )
                        .cloned()
                        .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                37 /* terrane-site: core/process.trn:55:41-55:59 */,
                            ),
                        )),
                    37 /* terrane-site: core/process.trn:55:41-55:59 */,
                ),
            );
            __terrane_list_append_1
                .push(EnvironmentEntry::terrane_construct(name, value));
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
        let mut value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(declared);
        value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        self.entries = declared;
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
    let __terrane_iterable_0 = schema.entries.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
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
                            38 /* terrane-site: core/process.trn:90:20-90:35 */,
                        ),
                    ),
                38 /* terrane-site: core/process.trn:90:20-90:35 */,
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
                                                39 /* terrane-site: core/process.trn:105:43-105:62 */,
                                            ),
                                        ),
                                    39 /* terrane-site: core/process.trn:105:43-105:62 */,
                                ),
                            );
                        index = index.clone() + terrane_int_support::Int::from(1_i128);
                    }
                } else if argument.text.starts_with(&String::from("--")) {
                    __terrane_list_append_2.push(index.clone());
                    __terrane_list_append_3.push(String::from("unknown option"));
                } else {
                    __terrane_list_append_7.push(argument);
                }
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut result: CommandLine = CommandLine::terrane_construct();
    result.flags = flags;
    result.option_names = option_names;
    result.option_values = option_values;
    result.positionals = positionals;
    result.diagnostic_arguments = diagnostic_arguments;
    result.diagnostic_messages = diagnostic_messages;
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
    let mut result: ExitStatus = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result.code = terrane_int_support::Int::from(255_i128);
        result.valid = false;
    } else {
        result.code = requested.clone();
    }
    return result;
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
    return EnvironmentEntry::terrane_construct(key, item);
}
pub fn encode_native_string(value: NativeString) -> String {
    if value.is_text {
        return terrane_platform_value_from_text(&value.text);
    }
    return terrane_platform_value_from_bytes(&value.raw);
}
