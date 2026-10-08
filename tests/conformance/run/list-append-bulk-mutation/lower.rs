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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
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
                std::boxed::Box::new(TerraneErrorDetail {
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
// Source: case.trn
// Namespace: list-append-bulk-mutation
fn validate_large_literal(enabled: bool) {
    let mut values_terrane_f0_s231: terrane_collection_support::List<i64>;
    let mut index_terrane_f0_s263: i64;
    if enabled {
        values_terrane_f0_s231 = terrane_collection_support::List::<
            i64,
        >::new(Vec::new());
        index_terrane_f0_s263 = 0;
        {
            let __terrane_list_append_0 = values_terrane_f0_s231.make_unique();
            let __terrane_list_start_0 = index_terrane_f0_s263;
            let __terrane_list_end_0 = 4000000000 as i64;
            let __terrane_list_length_0 = (__terrane_list_start_0..__terrane_list_end_0)
                .size_hint()
                .0;
            let __terrane_list_capacity_limit_0 = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            if __terrane_list_length_0 <= __terrane_list_capacity_limit_0 {
                __terrane_list_append_0.reserve(__terrane_list_length_0);
                for __terrane_list_index_0 in __terrane_list_start_0..__terrane_list_end_0 {
                    index_terrane_f0_s263 = __terrane_list_index_0;
                    __terrane_list_append_0.push(index_terrane_f0_s263);
                }
                index_terrane_f0_s263 = std::cmp::max(
                    __terrane_list_start_0,
                    __terrane_list_end_0,
                );
            } else {
                __terrane_list_append_0.reserve(__terrane_list_capacity_limit_0);
                while index_terrane_f0_s263 < 4000000000 {
                    __terrane_list_append_0.push(index_terrane_f0_s263);
                    index_terrane_f0_s263 = index_terrane_f0_s263 + 1;
                }
            }
            let _ = &index_terrane_f0_s263;
        }
    }
}
fn validate_return() -> terrane_int_support::Int {
    let mut values_terrane_f0_s382: terrane_collection_support::List<i64>;
    let mut index_terrane_f0_s412: i64;
    let limit_terrane_f0_s430: i64;
    values_terrane_f0_s382 = terrane_collection_support::List::<i64>::new(Vec::new());
    index_terrane_f0_s412 = 0;
    limit_terrane_f0_s430 = 100000000000000;
    {
        let __terrane_list_append_1 = values_terrane_f0_s382.make_unique();
        while index_terrane_f0_s412 < limit_terrane_f0_s430 {
            __terrane_list_append_1.push(index_terrane_f0_s412);
            if index_terrane_f0_s412 > 2 {
                return terrane_int_support::Int::from(
                    __terrane_raised(
                        terrane_int_support::fixed_addition(index_terrane_f0_s412, 1),
                        0 /* terrane-site: case.trn:23:14-23:23 */,
                    ) as i128,
                );
            }
            index_terrane_f0_s412 = __terrane_raised(
                terrane_int_support::fixed_addition(index_terrane_f0_s412, 1),
                1 /* terrane-site: case.trn:24:5-24:12 */,
            );
        }
    }
    return terrane_int_support::Int::from(0_i128);
}
fn validate_exit() {
    let mut values_terrane_f0_s597: terrane_collection_support::List<i64>;
    let mut index_terrane_f0_s627: i64;
    let limit_terrane_f0_s645: i64;
    values_terrane_f0_s597 = terrane_collection_support::List::<i64>::new(Vec::new());
    index_terrane_f0_s627 = 0;
    limit_terrane_f0_s645 = 100000000000000;
    {
        let __terrane_list_append_2 = values_terrane_f0_s597.make_unique();
        while index_terrane_f0_s627 < limit_terrane_f0_s645 {
            __terrane_list_append_2.push(index_terrane_f0_s627);
            if index_terrane_f0_s627 > 2 {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_addition(index_terrane_f0_s627,
                    1), 2 /* terrane-site: case.trn:34:14-34:23 */))
                );
                exit(make_exit_status(terrane_int_support::Int::from(0_i128)));
            }
            index_terrane_f0_s627 = __terrane_raised(
                terrane_int_support::fixed_addition(index_terrane_f0_s627, 1),
                3 /* terrane-site: case.trn:36:5-36:12 */,
            );
        }
    }
}
fn validate_throw() {
    let mut values_terrane_f0_s844: Option<terrane_collection_support::List<i64>> = None;
    let mut index_terrane_f0_s876: Option<i64> = None;
    let mut limit_terrane_f0_s896: Option<i64> = None;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = values_terrane_f0_s844
                .insert(terrane_collection_support::List::<i64>::new(Vec::new()));
            let _ = index_terrane_f0_s876.insert(0);
            let _ = limit_terrane_f0_s896.insert(100000000000000);
            {
                let __terrane_list_append_3 = values_terrane_f0_s844
                    .as_mut()
                    .expect("flow-proven available binding")
                    .make_unique();
                while *index_terrane_f0_s876
                    .as_ref()
                    .expect("flow-proven available binding")
                    < *limit_terrane_f0_s896
                        .as_ref()
                        .expect("flow-proven available binding")
                {
                    __terrane_list_append_3
                        .push(
                            *index_terrane_f0_s876
                                .as_ref()
                                .expect("flow-proven available binding"),
                        );
                    if *index_terrane_f0_s876
                        .as_ref()
                        .expect("flow-proven available binding") > 2
                    {
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_int_support::fixed_addition(*
                            index_terrane_f0_s876.as_ref()
                            .expect("flow-proven available binding"), 1),
                            4 /* terrane-site: case.trn:46:16-46:25 */))
                        );
                        return TerraneCompletion::Error(
                            TerraneError::raised(
                                TerraneErrorKind::ArithmeticOverflow,
                                5 /* terrane-site: case.trn:47:9-47:34 */,
                            ),
                        );
                    }
                    let _ = index_terrane_f0_s876
                        .insert(
                            __terrane_raised_completion!(
                                terrane_int_support::fixed_addition(* index_terrane_f0_s876
                                .as_ref().expect("flow-proven available binding"), 1),
                                6 /* terrane-site: case.trn:48:7-48:14 */
                            ),
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
    let mut values_terrane_f0_s1246: terrane_collection_support::List<i64>;
    let mut index_terrane_f0_s1276: i64;
    let limit_terrane_f0_s1294: i64;
    let original_terrane_f0_s1372: terrane_collection_support::List<i64>;
    let mut three_clause_terrane_f0_s1460: terrane_collection_support::List<i64>;
    let mut for_index_terrane_f0_s1496: i64;
    let mut three_clause_break_terrane_f0_s1682: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut break_index_terrane_f0_s1726: terrane_int_support::Int;
    let mut update_observed_terrane_f0_s1914: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut update_index_terrane_f0_s1955: terrane_int_support::Int;
    let mut condition_observed_terrane_f0_s2149: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut condition_index_terrane_f0_s2193: terrane_int_support::Int;
    let mut double_index_terrane_f0_s2394: i64;
    let mut double_update_terrane_f0_s2441: terrane_collection_support::List<i64>;
    let mut dependent_terrane_f0_s2628: terrane_collection_support::List<i64>;
    let mut dependent_index_terrane_f0_s2661: i64;
    let mut value_terrane_f0_s2782: i64;
    let mut inner_index_terrane_f0_s2821: i64;
    let mut inner_terrane_f0_s2871: terrane_collection_support::List<i64>;
    let mut nested_terrane_f0_s2949: terrane_collection_support::List<i64>;
    let mut outer_index_terrane_f0_s2979: i64;
    let mut nested_index_terrane_f0_s3060: i64;
    let mut early_exit_terrane_f0_s3210: terrane_collection_support::List<i64>;
    let mut early_index_terrane_f0_s3244: i64;
    let early_limit_terrane_f0_s3268: i64;
    let mut nested_break_terrane_f0_s3456: terrane_collection_support::List<i64>;
    let mut break_outer_terrane_f0_s3492: i64;
    let mut break_inner_terrane_f0_s3579: i64;
    let source_terrane_f0_s3689: terrane_collection_support::List<i64>;
    let mut collected_terrane_f0_s3731: terrane_collection_support::List<i64>;
    let mut observed_terrane_f0_s3855: terrane_collection_support::List<i64>;
    let rows_terrane_f0_s3968: terrane_collection_support::List<
        terrane_collection_support::List<i64>,
    >;
    let mut flattened_terrane_f0_s4029: terrane_collection_support::List<i64>;
    let mut row_terrane_f0_s4066: terrane_collection_support::List<i64>;
    let mut aliased_terrane_f0_s4172: terrane_collection_support::List<i64>;
    let alias_terrane_f0_s4209: terrane_collection_support::List<i64>;
    let mut self_source_terrane_f0_s4313: terrane_collection_support::List<i64>;
    let mut negative_terrane_f0_s4440: terrane_collection_support::List<i64>;
    let mut negative_index_terrane_f0_s4472: i64;
    let negative_limit_terrane_f0_s4499: i64;
    let mut mapped_terrane_f0_s4666: terrane_collection_support::List<f64>;
    let mut mapped_index_terrane_f0_s4698: i64;
    let mapped_limit_terrane_f0_s4723: i64;
    let mut mapped_value_terrane_f0_s4786: f64;
    let mut mutated_before_builder_terrane_f0_s4939: terrane_collection_support::List<
        i64,
    >;
    let mut mutated_index_terrane_f0_s5020: i64;
    let mut reassigned_before_builder_terrane_f0_s5235: terrane_collection_support::List<
        i64,
    >;
    let replacement_terrane_f0_s5284: terrane_collection_support::List<i64>;
    let mut reassigned_index_terrane_f0_s5364: i64;
    let mut dynamically_reentered_terrane_f0_s5603: terrane_collection_support::List<
        i64,
    >;
    let mut outer_builder_index_terrane_f0_s5648: i64;
    let mut observed_length_terrane_f0_s5714: terrane_int_support::Int;
    let mut inner_builder_index_terrane_f0_s5765: i64;
    let mut prefix_effects_terrane_f0_s5980: terrane_collection_support::List<i64>;
    let mut prefix_index_terrane_f0_s6018: i64;
    let mut prefix_value_terrane_f0_s6070: i64;
    let mut fallback_terrane_f0_s6246: terrane_collection_support::List<i64>;
    let mut fallback_index_terrane_f0_s6278: i64;
    let captured_terrane_f0_s6444: terrane_collection_support::List<i64>;
    let mut captured_index_terrane_f0_s6476: i64;
    let capturing_builder_terrane_f0_s6503: TerraneMutableCallable<(), ()>;
    let local_mutation_terrane_f0_s6738: TerraneMutableCallable<(), i64>;
    let returned_capture_terrane_f0_s6876: terrane_collection_support::List<i64>;
    let return_capture_terrane_f0_s6922: TerraneMutableCallable<
        (),
        terrane_collection_support::List<i64>,
    >;
    let first_returned_terrane_f0_s7032: terrane_collection_support::List<i64>;
    let second_returned_terrane_f0_s7067: terrane_collection_support::List<i64>;
    let inferred_capture_terrane_f0_s7198: terrane_collection_support::List<i64>;
    let inspect_capture_terrane_f0_s7244: TerraneMutableCallable<(), i64>;
    validate_large_literal(false);
    values_terrane_f0_s1246 = terrane_collection_support::List::<i64>::new(Vec::new());
    index_terrane_f0_s1276 = 0;
    limit_terrane_f0_s1294 = 4;
    {
        let __terrane_list_append_4 = values_terrane_f0_s1246.make_unique();
        let __terrane_list_start_1 = index_terrane_f0_s1276;
        let __terrane_list_end_1 = limit_terrane_f0_s1294;
        let __terrane_list_length_1 = (__terrane_list_start_1..__terrane_list_end_1)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_1 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_1 <= __terrane_list_capacity_limit_1 {
            __terrane_list_append_4.reserve(__terrane_list_length_1);
            for __terrane_list_index_1 in __terrane_list_start_1..__terrane_list_end_1 {
                index_terrane_f0_s1276 = __terrane_list_index_1;
                __terrane_list_append_4.push(index_terrane_f0_s1276);
            }
            index_terrane_f0_s1276 = std::cmp::max(
                __terrane_list_start_1,
                __terrane_list_end_1,
            );
        } else {
            __terrane_list_append_4.reserve(__terrane_list_capacity_limit_1);
            while index_terrane_f0_s1276 < limit_terrane_f0_s1294 {
                __terrane_list_append_4.push(index_terrane_f0_s1276);
                index_terrane_f0_s1276 = __terrane_raised(
                    terrane_int_support::fixed_addition(index_terrane_f0_s1276, 1),
                    7 /* terrane-site: case.trn:63:5-63:12 */,
                );
            }
        }
        let _ = &index_terrane_f0_s1276;
    }
    original_terrane_f0_s1372 = values_terrane_f0_s1246.clone();
    values_terrane_f0_s1246.append(9);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(original_terrane_f0_s1372
        .length()))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values_terrane_f0_s1246
        .length()))
    );
    three_clause_terrane_f0_s1460 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    for_index_terrane_f0_s1496 = 0;
    println!("{}", terrane_scalar_support::scalar_text(&for_index_terrane_f0_s1496));
    for_index_terrane_f0_s1496 = 0;
    {
        let __terrane_list_append_5 = three_clause_terrane_f0_s1460.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(for_index_terrane_f0_s1496),
            usize::try_from(limit_terrane_f0_s1294),
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
        '__terrane_break_2: while for_index_terrane_f0_s1496 < limit_terrane_f0_s1294 {
            '__terrane_continue_2: {
                __terrane_list_append_5.push(for_index_terrane_f0_s1496);
            }
            for_index_terrane_f0_s1496 = __terrane_raised(
                terrane_int_support::fixed_addition(for_index_terrane_f0_s1496, 1),
                8 /* terrane-site: case.trn:73:41-73:52 */,
            );
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(three_clause_terrane_f0_s1460
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(three_clause_terrane_f0_s1460
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        9 /* terrane-site: case.trn:75:31-75:46 */)), 9 /* terrane-site: case.trn:75:31-75:46 */)),
        terrane_scalar_support::scalar_text(&for_index_terrane_f0_s1496)
    );
    three_clause_break_terrane_f0_s1682 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    break_index_terrane_f0_s1726 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_6 = three_clause_break_terrane_f0_s1682.make_unique();
        '__terrane_break_3: while break_index_terrane_f0_s1726.clone()
            < terrane_int_support::Int::from(10_i128)
        {
            '__terrane_continue_3: {
                __terrane_list_append_6.push(break_index_terrane_f0_s1726.clone());
                if break_index_terrane_f0_s1726.clone()
                    > terrane_int_support::Int::from(2_i128)
                {
                    break '__terrane_break_3;
                }
            }
            break_index_terrane_f0_s1726 = break_index_terrane_f0_s1726.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(three_clause_break_terrane_f0_s1682
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(three_clause_break_terrane_f0_s1682
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        10 /* terrane-site: case.trn:82:37-82:58 */)), 10 /* terrane-site: case.trn:82:37-82:58 */))
    );
    update_observed_terrane_f0_s1914 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    update_index_terrane_f0_s1955 = terrane_int_support::Int::from(0_i128);
    '__terrane_break_4: while update_index_terrane_f0_s1955.clone()
        < terrane_int_support::Int::from(3_i128)
    {
        '__terrane_continue_4: {
            update_observed_terrane_f0_s1914
                .append(update_index_terrane_f0_s1955.clone());
        }
        update_index_terrane_f0_s1955 = update_index_terrane_f0_s1955.clone()
            + terrane_int_support::Int::from(1_i128)
            + terrane_int_support::Int::from(
                terrane_int_support::Int::from(update_observed_terrane_f0_s1914.length()),
            )
            - terrane_int_support::Int::from(
                terrane_int_support::Int::from(update_observed_terrane_f0_s1914.length()),
            );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(update_observed_terrane_f0_s1914
        .length()))
    );
    condition_observed_terrane_f0_s2149 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    condition_index_terrane_f0_s2193 = terrane_int_support::Int::from(0_i128);
    '__terrane_break_5: while condition_index_terrane_f0_s2193.clone()
        < terrane_int_support::Int::from(4_i128)
            + terrane_int_support::Int::from(
                terrane_int_support::Int::from(
                    condition_observed_terrane_f0_s2149.length(),
                ),
            )
            - terrane_int_support::Int::from(
                terrane_int_support::Int::from(
                    condition_observed_terrane_f0_s2149.length(),
                ),
            )
    {
        '__terrane_continue_5: {
            condition_observed_terrane_f0_s2149
                .append(condition_index_terrane_f0_s2193.clone());
        }
        condition_index_terrane_f0_s2193 = condition_index_terrane_f0_s2193.clone()
            + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(condition_observed_terrane_f0_s2149
        .length()))
    );
    double_index_terrane_f0_s2394 = 0;
    println!("{}", terrane_scalar_support::scalar_text(&double_index_terrane_f0_s2394));
    double_update_terrane_f0_s2441 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    double_index_terrane_f0_s2394 = 0;
    {
        let __terrane_list_append_7 = double_update_terrane_f0_s2441.make_unique();
        '__terrane_break_6: while double_index_terrane_f0_s2394 < limit_terrane_f0_s1294
        {
            '__terrane_continue_6: {
                __terrane_list_append_7.push(double_index_terrane_f0_s2394);
                double_index_terrane_f0_s2394 = __terrane_raised(
                    terrane_int_support::fixed_addition(
                        double_index_terrane_f0_s2394,
                        1,
                    ),
                    11 /* terrane-site: case.trn:99:5-99:19 */,
                );
            }
            double_index_terrane_f0_s2394 = __terrane_raised(
                terrane_int_support::fixed_addition(double_index_terrane_f0_s2394, 1),
                12 /* terrane-site: case.trn:97:47-97:61 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(double_update_terrane_f0_s2441
        .length()))
    );
    dependent_terrane_f0_s2628 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    dependent_index_terrane_f0_s2661 = 0;
    while dependent_index_terrane_f0_s2661 < 3 {
        dependent_terrane_f0_s2628
            .append(
                __terrane_raised(
                    terrane_int_support::coerce::<
                        i64,
                    >(
                        &terrane_int_support::Int::from(
                            dependent_terrane_f0_s2628.length(),
                        ),
                    ),
                    13 /* terrane-site: case.trn:105:23-105:39 */,
                ),
            );
        dependent_index_terrane_f0_s2661 = dependent_index_terrane_f0_s2661 + 1;
    }
    let __terrane_iterable_7 = dependent_terrane_f0_s2628;
    let mut __terrane_iterator_7 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_7,
    );
    loop {
        value_terrane_f0_s2782 = match __terrane_iterator_7.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s2782));
    }
    inner_index_terrane_f0_s2821 = 0;
    while inner_index_terrane_f0_s2821 < 1 {
        inner_terrane_f0_s2871 = terrane_collection_support::List::<
            i64,
        >::new(Vec::new());
        inner_terrane_f0_s2871.append(inner_index_terrane_f0_s2821);
        inner_index_terrane_f0_s2821 = inner_index_terrane_f0_s2821 + 1;
    }
    nested_terrane_f0_s2949 = terrane_collection_support::List::<i64>::new(Vec::new());
    outer_index_terrane_f0_s2979 = 0;
    {
        let __terrane_list_append_8 = nested_terrane_f0_s2949.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(outer_index_terrane_f0_s2979),
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
        while outer_index_terrane_f0_s2979 < 3 {
            __terrane_list_append_8.push(outer_index_terrane_f0_s2979);
            nested_index_terrane_f0_s3060 = 0;
            while nested_index_terrane_f0_s3060 < 2 {
                __terrane_list_append_8.push(nested_index_terrane_f0_s3060);
                nested_index_terrane_f0_s3060 = nested_index_terrane_f0_s3060 + 1;
            }
            outer_index_terrane_f0_s2979 = outer_index_terrane_f0_s2979 + 1;
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested_terrane_f0_s2949
        .length()))
    );
    early_exit_terrane_f0_s3210 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    early_index_terrane_f0_s3244 = 0;
    early_limit_terrane_f0_s3268 = 100000000000000;
    {
        let __terrane_list_append_9 = early_exit_terrane_f0_s3210.make_unique();
        while early_index_terrane_f0_s3244 < early_limit_terrane_f0_s3268 {
            __terrane_list_append_9.push(early_index_terrane_f0_s3244);
            if early_index_terrane_f0_s3244 > 2 {
                break;
            }
            early_index_terrane_f0_s3244 = __terrane_raised(
                terrane_int_support::fixed_addition(early_index_terrane_f0_s3244, 1),
                14 /* terrane-site: case.trn:135:5-135:18 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(early_exit_terrane_f0_s3210
        .length()))
    );
    nested_break_terrane_f0_s3456 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    break_outer_terrane_f0_s3492 = 0;
    {
        let __terrane_list_append_10 = nested_break_terrane_f0_s3456.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(break_outer_terrane_f0_s3492),
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
        while break_outer_terrane_f0_s3492 < 3 {
            __terrane_list_append_10.push(break_outer_terrane_f0_s3492);
            break_inner_terrane_f0_s3579 = 0;
            while break_inner_terrane_f0_s3579 < 3 {
                break;
            }
            break_outer_terrane_f0_s3492 = break_outer_terrane_f0_s3492 + 1;
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested_break_terrane_f0_s3456
        .length()))
    );
    source_terrane_f0_s3689 = terrane_collection_support::List::<
        i64,
    >::new(vec![1, 2, 3, 4]);
    collected_terrane_f0_s3731 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let __terrane_iterable_8 = source_terrane_f0_s3689.clone();
    let mut __terrane_iterator_8 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_8,
    );
    {
        let __terrane_list_append_11 = collected_terrane_f0_s3731.make_unique();
        loop {
            value_terrane_f0_s2782 = match __terrane_iterator_8.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &value_terrane_f0_s2782;
            __terrane_list_append_11.push(value_terrane_f0_s2782);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(collected_terrane_f0_s3731
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(collected_terrane_f0_s3731
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        15 /* terrane-site: case.trn:152:28-152:40 */)), 15 /* terrane-site: case.trn:152:28-152:40 */))
    );
    observed_terrane_f0_s3855 = terrane_collection_support::List::<i64>::new(Vec::new());
    let __terrane_iterable_9 = source_terrane_f0_s3689;
    let mut __terrane_iterator_9 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_9,
    );
    loop {
        value_terrane_f0_s2782 = match __terrane_iterator_9.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value_terrane_f0_s2782;
        observed_terrane_f0_s3855
            .append(
                __terrane_raised(
                    terrane_int_support::coerce::<
                        i64,
                    >(
                        &terrane_int_support::Int::from(
                            observed_terrane_f0_s3855.length(),
                        ),
                    ),
                    16 /* terrane-site: case.trn:156:22-156:37 */,
                ),
            );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(observed_terrane_f0_s3855
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        17 /* terrane-site: case.trn:157:10-157:21 */)), 17 /* terrane-site: case.trn:157:10-157:21 */))
    );
    rows_terrane_f0_s3968 = terrane_collection_support::List::<
        terrane_collection_support::List<i64>,
    >::new(
        vec![
            terrane_collection_support::List::< i64 >::new(vec![1, 2]),
            terrane_collection_support::List::< i64 >::new(vec![3])
        ],
    );
    flattened_terrane_f0_s4029 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let __terrane_iterable_10 = rows_terrane_f0_s3968;
    let mut __terrane_iterator_10 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_10,
    );
    {
        let __terrane_list_append_12 = flattened_terrane_f0_s4029.make_unique();
        loop {
            row_terrane_f0_s4066 = match __terrane_iterator_10.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let __terrane_iterable_11 = row_terrane_f0_s4066;
            let mut __terrane_iterator_11 = terrane_collection_support::Iterable::terrane_iterator(
                &__terrane_iterable_11,
            );
            loop {
                value_terrane_f0_s2782 = match __terrane_iterator_11.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                let _ = &value_terrane_f0_s2782;
                __terrane_list_append_12.push(value_terrane_f0_s2782);
            }
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(flattened_terrane_f0_s4029
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(flattened_terrane_f0_s4029
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        18 /* terrane-site: case.trn:164:28-164:40 */)), 18 /* terrane-site: case.trn:164:28-164:40 */))
    );
    aliased_terrane_f0_s4172 = terrane_collection_support::List::<i64>::new(vec![1, 2]);
    alias_terrane_f0_s4209 = aliased_terrane_f0_s4172.clone();
    let __terrane_iterable_12 = alias_terrane_f0_s4209.clone();
    let mut __terrane_iterator_12 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_12,
    );
    {
        let __terrane_list_append_13 = aliased_terrane_f0_s4172.make_unique();
        loop {
            value_terrane_f0_s2782 = match __terrane_iterator_12.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            let _ = &value_terrane_f0_s2782;
            __terrane_list_append_13.push(value_terrane_f0_s2782);
        }
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(aliased_terrane_f0_s4172
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(alias_terrane_f0_s4209
        .length()))
    );
    self_source_terrane_f0_s4313 = terrane_collection_support::List::<
        i64,
    >::new(vec![1, 2]);
    let __terrane_iterable_13 = self_source_terrane_f0_s4313.clone();
    let mut __terrane_iterator_13 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_13,
    );
    loop {
        value_terrane_f0_s2782 = match __terrane_iterator_13.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value_terrane_f0_s2782;
        self_source_terrane_f0_s4313.append(value_terrane_f0_s2782);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(self_source_terrane_f0_s4313
        .length()))
    );
    negative_terrane_f0_s4440 = terrane_collection_support::List::<i64>::new(Vec::new());
    negative_index_terrane_f0_s4472 = 0;
    negative_limit_terrane_f0_s4499 = -3;
    {
        let __terrane_list_append_14 = negative_terrane_f0_s4440.make_unique();
        let __terrane_list_start_14 = negative_index_terrane_f0_s4472;
        let __terrane_list_end_14 = negative_limit_terrane_f0_s4499;
        let __terrane_list_length_14 = (__terrane_list_start_14..__terrane_list_end_14)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_14 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_14 <= __terrane_list_capacity_limit_14 {
            __terrane_list_append_14.reserve(__terrane_list_length_14);
            for __terrane_list_index_14 in __terrane_list_start_14..__terrane_list_end_14 {
                negative_index_terrane_f0_s4472 = __terrane_list_index_14;
                __terrane_list_append_14.push(negative_index_terrane_f0_s4472);
            }
            negative_index_terrane_f0_s4472 = std::cmp::max(
                __terrane_list_start_14,
                __terrane_list_end_14,
            );
        } else {
            __terrane_list_append_14.reserve(__terrane_list_capacity_limit_14);
            while negative_index_terrane_f0_s4472 < negative_limit_terrane_f0_s4499 {
                __terrane_list_append_14.push(negative_index_terrane_f0_s4472);
                negative_index_terrane_f0_s4472 = __terrane_raised(
                    terrane_int_support::fixed_addition(
                        negative_index_terrane_f0_s4472,
                        1,
                    ),
                    19 /* terrane-site: case.trn:182:5-182:21 */,
                );
            }
        }
        let _ = &negative_index_terrane_f0_s4472;
    }
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(negative_terrane_f0_s4440
        .length())),
        terrane_scalar_support::scalar_text(&negative_index_terrane_f0_s4472)
    );
    mapped_terrane_f0_s4666 = terrane_collection_support::List::<f64>::new(Vec::new());
    mapped_index_terrane_f0_s4698 = 0;
    mapped_limit_terrane_f0_s4723 = 4;
    {
        let __terrane_list_append_15 = mapped_terrane_f0_s4666.make_unique();
        let __terrane_list_start_15 = mapped_index_terrane_f0_s4698;
        let __terrane_list_end_15 = mapped_limit_terrane_f0_s4723;
        let __terrane_list_length_15 = (__terrane_list_start_15..__terrane_list_end_15)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_15 = 268435456usize
            / std::mem::size_of::<f64>().max(1);
        if __terrane_list_length_15 <= __terrane_list_capacity_limit_15 {
            __terrane_list_append_15.reserve(__terrane_list_length_15);
            for __terrane_list_index_15 in __terrane_list_start_15..__terrane_list_end_15 {
                mapped_index_terrane_f0_s4698 = __terrane_list_index_15;
                mapped_value_terrane_f0_s4786 = __terrane_raised(
                    terrane_int_support::exact_fixed_f64(mapped_index_terrane_f0_s4698),
                    20 /* terrane-site: case.trn:189:28-189:40 */,
                );
                __terrane_list_append_15
                    .push(mapped_value_terrane_f0_s4786 * mapped_value_terrane_f0_s4786);
            }
            mapped_index_terrane_f0_s4698 = std::cmp::max(
                __terrane_list_start_15,
                __terrane_list_end_15,
            );
        } else {
            __terrane_list_append_15.reserve(__terrane_list_capacity_limit_15);
            while mapped_index_terrane_f0_s4698 < mapped_limit_terrane_f0_s4723 {
                mapped_value_terrane_f0_s4786 = __terrane_raised(
                    terrane_int_support::exact_fixed_f64(mapped_index_terrane_f0_s4698),
                    20 /* terrane-site: case.trn:189:28-189:40 */,
                );
                __terrane_list_append_15
                    .push(mapped_value_terrane_f0_s4786 * mapped_value_terrane_f0_s4786);
                mapped_index_terrane_f0_s4698 = __terrane_raised(
                    terrane_int_support::fixed_addition(
                        mapped_index_terrane_f0_s4698,
                        1,
                    ),
                    21 /* terrane-site: case.trn:191:5-191:19 */,
                );
            }
        }
        let _ = &mapped_index_terrane_f0_s4698;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(mapped_terrane_f0_s4666
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(mapped_terrane_f0_s4666
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        22 /* terrane-site: case.trn:192:25-192:34 */)), 22 /* terrane-site: case.trn:192:25-192:34 */)),
        terrane_scalar_support::scalar_text(&mapped_index_terrane_f0_s4698)
    );
    mutated_before_builder_terrane_f0_s4939 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    mutated_before_builder_terrane_f0_s4939.append(7);
    mutated_index_terrane_f0_s5020 = 0;
    {
        let __terrane_list_append_16 = mutated_before_builder_terrane_f0_s4939
            .make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(mutated_index_terrane_f0_s5020),
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
        while mutated_index_terrane_f0_s5020 < 2 {
            __terrane_list_append_16.push(mutated_index_terrane_f0_s5020);
            mutated_index_terrane_f0_s5020 = mutated_index_terrane_f0_s5020 + 1;
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(mutated_before_builder_terrane_f0_s4939
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(mutated_before_builder_terrane_f0_s4939
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        23 /* terrane-site: case.trn:200:41-200:66 */)), 23 /* terrane-site: case.trn:200:41-200:66 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(mutated_before_builder_terrane_f0_s4939
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        24 /* terrane-site: case.trn:200:68-200:93 */)), 24 /* terrane-site: case.trn:200:68-200:93 */))
    );
    reassigned_before_builder_terrane_f0_s5235 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let _ = &mut reassigned_before_builder_terrane_f0_s5235;
    replacement_terrane_f0_s5284 = terrane_collection_support::List::<i64>::new(vec![8]);
    reassigned_before_builder_terrane_f0_s5235 = replacement_terrane_f0_s5284;
    reassigned_index_terrane_f0_s5364 = 0;
    {
        let __terrane_list_append_17 = reassigned_before_builder_terrane_f0_s5235
            .make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(reassigned_index_terrane_f0_s5364),
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
        while reassigned_index_terrane_f0_s5364 < 2 {
            __terrane_list_append_17.push(reassigned_index_terrane_f0_s5364);
            reassigned_index_terrane_f0_s5364 = reassigned_index_terrane_f0_s5364 + 1;
        }
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(reassigned_before_builder_terrane_f0_s5235
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(reassigned_before_builder_terrane_f0_s5235
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        25 /* terrane-site: case.trn:209:44-209:72 */)), 25 /* terrane-site: case.trn:209:44-209:72 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(reassigned_before_builder_terrane_f0_s5235
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        26 /* terrane-site: case.trn:209:74-209:102 */)), 26 /* terrane-site: case.trn:209:74-209:102 */))
    );
    dynamically_reentered_terrane_f0_s5603 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    outer_builder_index_terrane_f0_s5648 = 0;
    while outer_builder_index_terrane_f0_s5648 < 3 {
        observed_length_terrane_f0_s5714 = terrane_int_support::Int::from(
            terrane_int_support::Int::from(
                dynamically_reentered_terrane_f0_s5603.length(),
            ),
        );
        let _ = &observed_length_terrane_f0_s5714;
        inner_builder_index_terrane_f0_s5765 = 0;
        {
            let __terrane_list_append_18 = dynamically_reentered_terrane_f0_s5603
                .make_unique();
            if let (Ok(__terrane_start), Ok(__terrane_end)) = (
                usize::try_from(inner_builder_index_terrane_f0_s5765),
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
            while inner_builder_index_terrane_f0_s5765 < 2 {
                __terrane_list_append_18.push(inner_builder_index_terrane_f0_s5765);
                inner_builder_index_terrane_f0_s5765 = inner_builder_index_terrane_f0_s5765
                    + 1;
            }
        }
        outer_builder_index_terrane_f0_s5648 = outer_builder_index_terrane_f0_s5648 + 1;
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(dynamically_reentered_terrane_f0_s5603
        .length()))
    );
    prefix_effects_terrane_f0_s5980 = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    prefix_index_terrane_f0_s6018 = 0;
    {
        let __terrane_list_append_19 = prefix_effects_terrane_f0_s5980.make_unique();
        let __terrane_list_start_16 = prefix_index_terrane_f0_s6018;
        let __terrane_list_end_16 = 3 as i64;
        let __terrane_list_length_16 = (__terrane_list_start_16..__terrane_list_end_16)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_16 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_16 <= __terrane_list_capacity_limit_16 {
            __terrane_list_append_19.reserve(__terrane_list_length_16);
            for __terrane_list_index_16 in __terrane_list_start_16..__terrane_list_end_16 {
                prefix_index_terrane_f0_s6018 = __terrane_list_index_16;
                prefix_value_terrane_f0_s6070 = observe_prefix(
                    prefix_index_terrane_f0_s6018,
                );
                __terrane_list_append_19.push(prefix_value_terrane_f0_s6070);
            }
            prefix_index_terrane_f0_s6018 = std::cmp::max(
                __terrane_list_start_16,
                __terrane_list_end_16,
            );
        } else {
            __terrane_list_append_19.reserve(__terrane_list_capacity_limit_16);
            while prefix_index_terrane_f0_s6018 < 3 {
                prefix_value_terrane_f0_s6070 = observe_prefix(
                    prefix_index_terrane_f0_s6018,
                );
                __terrane_list_append_19.push(prefix_value_terrane_f0_s6070);
                prefix_index_terrane_f0_s6018 = prefix_index_terrane_f0_s6018 + 1;
            }
        }
        let _ = &prefix_index_terrane_f0_s6018;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(prefix_effects_terrane_f0_s5980
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(prefix_effects_terrane_f0_s5980
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        27 /* terrane-site: case.trn:228:33-228:50 */)), 27 /* terrane-site: case.trn:228:33-228:50 */)),
        terrane_scalar_support::scalar_text(&prefix_index_terrane_f0_s6018)
    );
    fallback_terrane_f0_s6246 = terrane_collection_support::List::<i64>::new(Vec::new());
    fallback_index_terrane_f0_s6278 = 0;
    {
        let __terrane_list_append_20 = fallback_terrane_f0_s6246.make_unique();
        let __terrane_list_start_17 = fallback_index_terrane_f0_s6278;
        let __terrane_list_end_17 = 5 as i64;
        let __terrane_list_length_17 = (__terrane_list_start_17..__terrane_list_end_17)
            .size_hint()
            .0;
        let __terrane_list_capacity_limit_17 = 268435456usize
            / std::mem::size_of::<i64>().max(1);
        if __terrane_list_length_17 <= __terrane_list_capacity_limit_17 {
            __terrane_list_append_20.reserve(__terrane_list_length_17);
            for __terrane_list_index_17 in __terrane_list_start_17..__terrane_list_end_17 {
                fallback_index_terrane_f0_s6278 = __terrane_list_index_17;
                __terrane_list_append_20.push(fallback_index_terrane_f0_s6278);
            }
            fallback_index_terrane_f0_s6278 = std::cmp::max(
                __terrane_list_start_17,
                __terrane_list_end_17,
            );
        } else {
            __terrane_list_append_20.reserve(__terrane_list_capacity_limit_17);
            while fallback_index_terrane_f0_s6278 < 5 {
                __terrane_list_append_20.push(fallback_index_terrane_f0_s6278);
                fallback_index_terrane_f0_s6278 = fallback_index_terrane_f0_s6278 + 1;
            }
        }
        let _ = &fallback_index_terrane_f0_s6278;
    }
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fallback_terrane_f0_s6246
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(fallback_terrane_f0_s6246
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        28 /* terrane-site: case.trn:235:27-235:38 */)), 28 /* terrane-site: case.trn:235:27-235:38 */)),
        terrane_scalar_support::scalar_text(&fallback_index_terrane_f0_s6278)
    );
    captured_terrane_f0_s6444 = terrane_collection_support::List::<i64>::new(Vec::new());
    captured_index_terrane_f0_s6476 = 0;
    capturing_builder_terrane_f0_s6503 = {
        let mut captured_terrane_f0_s6444 = captured_terrane_f0_s6444.clone();
        let mut captured_index_terrane_f0_s6476 = captured_index_terrane_f0_s6476
            .clone();
        TerraneMutableCallable::new(move |(): ()| -> () {
            {
                let __terrane_list_append_21 = captured_terrane_f0_s6444.make_unique();
                if let (Ok(__terrane_start), Ok(__terrane_end)) = (
                    usize::try_from(captured_index_terrane_f0_s6476),
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
                while captured_index_terrane_f0_s6476 < 3 {
                    __terrane_list_append_21.push(captured_index_terrane_f0_s6476);
                    captured_index_terrane_f0_s6476 = __terrane_raised(
                        terrane_int_support::fixed_addition(
                            captured_index_terrane_f0_s6476,
                            1,
                        ),
                        29 /* terrane-site: case.trn:242:7-242:23 */,
                    );
                }
            }
            ()
        })
    };
    capturing_builder_terrane_f0_s6503.call(());
    captured_index_terrane_f0_s6476 = 0;
    capturing_builder_terrane_f0_s6503.call(());
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(captured_terrane_f0_s6444
        .length())),
        terrane_scalar_support::scalar_text(&captured_index_terrane_f0_s6476)
    );
    local_mutation_terrane_f0_s6738 = {
        TerraneMutableCallable::new(move |(): ()| -> i64 {
            let mut local_value_terrane_f0_s6783: i64;
            local_value_terrane_f0_s6783 = 0;
            local_value_terrane_f0_s6783 = __terrane_raised(
                terrane_int_support::fixed_addition(local_value_terrane_f0_s6783, 1),
                30 /* terrane-site: case.trn:250:5-250:18 */,
            );
            return local_value_terrane_f0_s6783;
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&local_mutation_terrane_f0_s6738
        .call(()))
    );
    returned_capture_terrane_f0_s6876 = terrane_collection_support::List::<
        i64,
    >::new(vec![7, 8]);
    return_capture_terrane_f0_s6922 = {
        let mut returned_capture_terrane_f0_s6876 = returned_capture_terrane_f0_s6876
            .clone();
        TerraneMutableCallable::new(move |
            (): (),
        | -> terrane_collection_support::List<i64> {
            returned_capture_terrane_f0_s6876.append(9);
            return returned_capture_terrane_f0_s6876.clone();
        })
    };
    first_returned_terrane_f0_s7032 = return_capture_terrane_f0_s6922.call(());
    second_returned_terrane_f0_s7067 = return_capture_terrane_f0_s6922.call(());
    println!(
        "{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(first_returned_terrane_f0_s7032
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(first_returned_terrane_f0_s7032
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        31 /* terrane-site: case.trn:260:33-260:50 */)), 31 /* terrane-site: case.trn:260:33-260:50 */)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(second_returned_terrane_f0_s7067
        .length())),
        terrane_scalar_support::scalar_text(&__terrane_raised(second_returned_terrane_f0_s7067
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        32 /* terrane-site: case.trn:260:76-260:94 */)), 32 /* terrane-site: case.trn:260:76-260:94 */))
    );
    inferred_capture_terrane_f0_s7198 = terrane_collection_support::List::<
        i64,
    >::new(vec![2, 3]);
    inspect_capture_terrane_f0_s7244 = {
        let mut inferred_capture_terrane_f0_s7198 = inferred_capture_terrane_f0_s7198
            .clone();
        TerraneMutableCallable::new(move |(): ()| -> i64 {
            let mut inferred_total_terrane_f0_s7290: i64;
            let mut inferred_value_terrane_f0_s7376: i64;
            inferred_total_terrane_f0_s7290 = __terrane_raised(
                terrane_int_support::coerce::<
                    i64,
                >(
                    &terrane_int_support::Int::from(
                        inferred_capture_terrane_f0_s7198.length(),
                    ),
                ),
                33 /* terrane-site: case.trn:264:28-264:51 */,
            );
            inferred_capture_terrane_f0_s7198.append(4);
            let __terrane_iterable_18 = inferred_capture_terrane_f0_s7198.clone();
            let mut __terrane_iterator_18 = terrane_collection_support::Iterable::terrane_iterator(
                &__terrane_iterable_18,
            );
            loop {
                inferred_value_terrane_f0_s7376 = match __terrane_iterator_18.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                inferred_total_terrane_f0_s7290 = __terrane_raised(
                    terrane_int_support::fixed_addition(
                        inferred_total_terrane_f0_s7290,
                        inferred_value_terrane_f0_s7376,
                    ),
                    34 /* terrane-site: case.trn:267:24-267:55 */,
                );
            }
            return inferred_total_terrane_f0_s7290;
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&inspect_capture_terrane_f0_s7244
        .call(()))
    );
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
        let mut __terrane_constructed_value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::from([]),
        };
        __terrane_constructed_value.construct(encoded);
        __terrane_constructed_value
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
        let mut __terrane_constructed_value = Self {
            name: NativeString::terrane_construct(String::from("text:")),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        __terrane_constructed_value.construct(name, entry_value);
        __terrane_constructed_value
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
        self.failed = did_fail;
        self.available = is_available;
        self.message = detail;
        self.value = result_value;
    }
}
pub fn process_host_name() -> ProcessHostNameResult {
    let raw_terrane_f1_s1079: TerranePlatformResult;
    raw_terrane_f1_s1079 = terrane_platform_support::system_host_name();
    return ProcessHostNameResult::terrane_construct(
        raw_terrane_f1_s1079.failed,
        raw_terrane_f1_s1079.flag,
        raw_terrane_f1_s1079.message.clone(),
        NativeString::terrane_construct(raw_terrane_f1_s1079.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<NativeString> {
    let encoded_terrane_f1_s1332: Vec<String>;
    let mut values_terrane_f1_s1372: terrane_collection_support::List<NativeString>;
    let mut index_terrane_f1_s1412: terrane_int_support::Int;
    encoded_terrane_f1_s1332 = terrane_process_arguments();
    values_terrane_f1_s1372 = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    index_terrane_f1_s1412 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = values_terrane_f1_s1372.make_unique();
        while index_terrane_f1_s1412.clone()
            < terrane_int_support::Int::from(encoded_terrane_f1_s1332.len() as i128)
        {
            __terrane_list_append_0
                .push(
                    NativeString::terrane_construct(
                        __terrane_raised(
                            {
                                let __terrane_receiver = &encoded_terrane_f1_s1332;
                                let __terrane_index = __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &index_terrane_f1_s1412.clone(),
                                    ),
                                    35 /* terrane-site: core/process.trn:45:49-45:63 */,
                                );
                                __terrane_receiver
                                    .get(__terrane_index)
                                    .cloned()
                                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                        __terrane_index,
                                    ))
                            },
                            35 /* terrane-site: core/process.trn:45:49-45:63 */,
                        ),
                    ),
                );
            index_terrane_f1_s1412 = index_terrane_f1_s1412.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    return values_terrane_f1_s1372;
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded_terrane_f1_s1620: Vec<String>;
    let mut values_terrane_f1_s1662: terrane_collection_support::List<EnvironmentEntry>;
    let mut index_terrane_f1_s1706: terrane_int_support::Int;
    let mut name_terrane_f1_s1765: NativeString;
    let mut value_terrane_f1_s1819: NativeString;
    encoded_terrane_f1_s1620 = terrane_environment_entries();
    values_terrane_f1_s1662 = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    index_terrane_f1_s1706 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = values_terrane_f1_s1662.make_unique();
        while index_terrane_f1_s1706.clone() + terrane_int_support::Int::from(1_i128)
            < terrane_int_support::Int::from(encoded_terrane_f1_s1620.len() as i128)
        {
            name_terrane_f1_s1765 = NativeString::terrane_construct(
                __terrane_raised(
                    {
                        let __terrane_receiver = &encoded_terrane_f1_s1620;
                        let __terrane_index = __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &index_terrane_f1_s1706.clone(),
                            ),
                            36 /* terrane-site: core/process.trn:54:40-54:54 */,
                        );
                        __terrane_receiver
                            .get(__terrane_index)
                            .cloned()
                            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                __terrane_index,
                            ))
                    },
                    36 /* terrane-site: core/process.trn:54:40-54:54 */,
                ),
            );
            value_terrane_f1_s1819 = NativeString::terrane_construct(
                __terrane_raised(
                    {
                        let __terrane_receiver = &encoded_terrane_f1_s1620;
                        let __terrane_index = __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &(index_terrane_f1_s1706.clone()
                                    + terrane_int_support::Int::from(1_i128)),
                            ),
                            37 /* terrane-site: core/process.trn:55:41-55:59 */,
                        );
                        __terrane_receiver
                            .get(__terrane_index)
                            .cloned()
                            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                __terrane_index,
                            ))
                    },
                    37 /* terrane-site: core/process.trn:55:41-55:59 */,
                ),
            );
            __terrane_list_append_1
                .push(
                    EnvironmentEntry::terrane_construct(
                        name_terrane_f1_s1765,
                        value_terrane_f1_s1819,
                    ),
                );
            index_terrane_f1_s1706 = index_terrane_f1_s1706.clone()
                + terrane_int_support::Int::from(2_i128);
        }
    }
    return values_terrane_f1_s1662;
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
    let mut entry_terrane_f1_s2454: String;
    let __terrane_iterable_0 = schema.entries.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        entry_terrane_f1_s2454 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry_terrane_f1_s2454.as_str() == sought.as_str() {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<NativeString>,
) -> CommandLine {
    let mut flags_terrane_f1_s2643: terrane_collection_support::List<String>;
    let mut option_names_terrane_f1_s2675: terrane_collection_support::List<String>;
    let mut option_values_terrane_f1_s2714: terrane_collection_support::List<
        NativeString,
    >;
    let mut positionals_terrane_f1_s2761: terrane_collection_support::List<NativeString>;
    let mut diagnostic_arguments_terrane_f1_s2806: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut diagnostic_messages_terrane_f1_s2850: terrane_collection_support::List<
        String,
    >;
    let mut index_terrane_f1_s2896: terrane_int_support::Int;
    let mut argument_terrane_f1_s2952: NativeString;
    let mut flag_entry_terrane_f1_s3165: String;
    let mut value_entry_terrane_f1_s3220: String;
    let mut result_terrane_f1_s4025: CommandLine;
    flags_terrane_f1_s2643 = terrane_collection_support::List::<String>::new(Vec::new());
    option_names_terrane_f1_s2675 = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    option_values_terrane_f1_s2714 = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    positionals_terrane_f1_s2761 = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    diagnostic_arguments_terrane_f1_s2806 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    diagnostic_messages_terrane_f1_s2850 = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    index_terrane_f1_s2896 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_2 = diagnostic_arguments_terrane_f1_s2806
            .make_unique();
        let __terrane_list_append_3 = diagnostic_messages_terrane_f1_s2850.make_unique();
        let __terrane_list_append_4 = flags_terrane_f1_s2643.make_unique();
        let __terrane_list_append_5 = option_names_terrane_f1_s2675.make_unique();
        let __terrane_list_append_6 = option_values_terrane_f1_s2714.make_unique();
        let __terrane_list_append_7 = positionals_terrane_f1_s2761.make_unique();
        while index_terrane_f1_s2896.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(supplied.length()),
            )
        {
            argument_terrane_f1_s2952 = __terrane_raised(
                supplied
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &index_terrane_f1_s2896.clone(),
                            ),
                            38 /* terrane-site: core/process.trn:90:20-90:35 */,
                        ),
                    ),
                38 /* terrane-site: core/process.trn:90:20-90:35 */,
            );
            if !argument_terrane_f1_s2952.is_text {
                __terrane_list_append_2.push(index_terrane_f1_s2896.clone());
                __terrane_list_append_3
                    .push(String::from("command-line option is not Unicode text"));
            } else {
                flag_entry_terrane_f1_s3165 = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("flag:")),
                    terrane_scalar_support::scalar_text(&argument_terrane_f1_s2952.text)
                );
                value_entry_terrane_f1_s3220 = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("value:")),
                    terrane_scalar_support::scalar_text(&argument_terrane_f1_s2952.text)
                );
                if schema_has(schema.clone(), flag_entry_terrane_f1_s3165) {
                    __terrane_list_append_4.push(argument_terrane_f1_s2952.text.clone());
                } else if schema_has(schema.clone(), value_entry_terrane_f1_s3220) {
                    if index_terrane_f1_s2896.clone()
                        + terrane_int_support::Int::from(1_i128)
                        >= terrane_int_support::Int::from(
                            terrane_int_support::Int::from(supplied.length()),
                        )
                    {
                        __terrane_list_append_2.push(index_terrane_f1_s2896.clone());
                        __terrane_list_append_3
                            .push(String::from("option requires a value"));
                    } else {
                        __terrane_list_append_5
                            .push(argument_terrane_f1_s2952.text.clone());
                        __terrane_list_append_6
                            .push(
                                __terrane_raised(
                                    supplied
                                        .get_or_error(
                                            __terrane_raised(
                                                terrane_collection_support::index_from_int(
                                                    &(index_terrane_f1_s2896.clone()
                                                        + terrane_int_support::Int::from(1_i128)),
                                                ),
                                                39 /* terrane-site: core/process.trn:105:43-105:62 */,
                                            ),
                                        ),
                                    39 /* terrane-site: core/process.trn:105:43-105:62 */,
                                ),
                            );
                        index_terrane_f1_s2896 = index_terrane_f1_s2896.clone()
                            + terrane_int_support::Int::from(1_i128);
                    }
                } else if argument_terrane_f1_s2952.text.starts_with(&String::from("--"))
                {
                    __terrane_list_append_2.push(index_terrane_f1_s2896.clone());
                    __terrane_list_append_3.push(String::from("unknown option"));
                } else {
                    __terrane_list_append_7.push(argument_terrane_f1_s2952);
                }
            }
            index_terrane_f1_s2896 = index_terrane_f1_s2896.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    result_terrane_f1_s4025 = CommandLine::terrane_construct();
    result_terrane_f1_s4025.flags = flags_terrane_f1_s2643;
    result_terrane_f1_s4025.option_names = option_names_terrane_f1_s2675;
    result_terrane_f1_s4025.option_values = option_values_terrane_f1_s2714;
    result_terrane_f1_s4025.positionals = positionals_terrane_f1_s2761;
    result_terrane_f1_s4025.diagnostic_arguments = diagnostic_arguments_terrane_f1_s2806;
    result_terrane_f1_s4025.diagnostic_messages = diagnostic_messages_terrane_f1_s2850;
    return result_terrane_f1_s4025;
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
    let mut result_terrane_f1_s4444: ExitStatus;
    result_terrane_f1_s4444 = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result_terrane_f1_s4444.code = terrane_int_support::Int::from(255_i128);
        result_terrane_f1_s4444.valid = false;
    } else {
        result_terrane_f1_s4444.code = requested.clone();
    }
    return result_terrane_f1_s4444;
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
pub fn native_text(value: String) -> NativeString {
    let encoded_terrane_f1_s4755: String;
    encoded_terrane_f1_s4755 = terrane_platform_value_from_text(&value);
    return NativeString::terrane_construct(encoded_terrane_f1_s4755);
}
pub fn native_raw(value: Vec<u8>) -> NativeString {
    let encoded_terrane_f1_s4897: String;
    encoded_terrane_f1_s4897 = terrane_platform_value_from_bytes(&value);
    return NativeString::terrane_construct(encoded_terrane_f1_s4897);
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
