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
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
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
            __terrane_list_append_0
                .push(
                    NativeString::terrane_construct(
                        __terrane_raised(
                            {
                                let __terrane_receiver = &encoded;
                                let __terrane_index = __terrane_raised(
                                    terrane_collection_support::index_from_int(&index.clone()),
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
            value = NativeString::terrane_construct(
                __terrane_raised(
                    {
                        let __terrane_receiver = &encoded;
                        let __terrane_index = __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &(index.clone() + terrane_int_support::Int::from(1_i128)),
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
    result = CommandLine::terrane_construct();
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
    let mut result: ExitStatus;
    result = ExitStatus::terrane_construct();
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
