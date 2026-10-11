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
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 1] = ["/stable-list-sorting::main"];
    pub static SITES: [Site; 43] = [
        /* terrane-site-row: site 0: /stable-list-sorting::main (case.trn:7:10-7:21) */
        { Site { function: 0, file: 0, line: 7, column: 10, end_line: 7, end_column: 21 } },
        /* terrane-site-row: site 1: /stable-list-sorting::main (case.trn:7:23-7:34) */
        { Site { function: 0, file: 0, line: 7, column: 23, end_line: 7, end_column: 34 } },
        /* terrane-site-row: site 2: /stable-list-sorting::main (case.trn:9:10-9:21) */
        { Site { function: 0, file: 0, line: 9, column: 10, end_line: 9, end_column: 21 } },
        /* terrane-site-row: site 3: /stable-list-sorting::main (case.trn:9:23-9:34) */
        { Site { function: 0, file: 0, line: 9, column: 23, end_line: 9, end_column: 34 } },
        /* terrane-site-row: site 4: /stable-list-sorting::main (case.trn:31:10-31:18) */
        { Site { function: 0, file: 0, line: 31, column: 10, end_line: 31, end_column: 18 } },
        /* terrane-site-row: site 5: /stable-list-sorting::main (case.trn:31:20-31:29) */
        { Site { function: 0, file: 0, line: 31, column: 20, end_line: 31, end_column: 29 } },
        /* terrane-site-row: site 6: /stable-list-sorting::main (case.trn:31:31-31:40) */
        { Site { function: 0, file: 0, line: 31, column: 31, end_line: 31, end_column: 40 } },
        /* terrane-site-row: site 7: /stable-list-sorting::main (case.trn:31:42-31:51) */
        { Site { function: 0, file: 0, line: 31, column: 42, end_line: 31, end_column: 51 } },
        /* terrane-site-row: site 8: /stable-list-sorting::main (case.trn:31:53-31:63) */
        { Site { function: 0, file: 0, line: 31, column: 53, end_line: 31, end_column: 63 } },
        /* terrane-site-row: site 9: /stable-list-sorting::main (case.trn:32:10-32:19) */
        { Site { function: 0, file: 0, line: 32, column: 10, end_line: 32, end_column: 19 } },
        /* terrane-site-row: site 10: /stable-list-sorting::main (case.trn:32:21-32:31) */
        { Site { function: 0, file: 0, line: 32, column: 21, end_line: 32, end_column: 31 } },
        /* terrane-site-row: site 11: /stable-list-sorting::main (case.trn:32:33-32:43) */
        { Site { function: 0, file: 0, line: 32, column: 33, end_line: 32, end_column: 43 } },
        /* terrane-site-row: site 12: /stable-list-sorting::main (case.trn:32:45-32:55) */
        { Site { function: 0, file: 0, line: 32, column: 45, end_line: 32, end_column: 55 } },
        /* terrane-site-row: site 13: /stable-list-sorting::main (case.trn:32:57-32:68) */
        { Site { function: 0, file: 0, line: 32, column: 57, end_line: 32, end_column: 68 } },
        /* terrane-site-row: site 14: /stable-list-sorting::main (case.trn:36:10-36:18) */
        { Site { function: 0, file: 0, line: 36, column: 10, end_line: 36, end_column: 18 } },
        /* terrane-site-row: site 15: /stable-list-sorting::main (case.trn:36:20-36:28) */
        { Site { function: 0, file: 0, line: 36, column: 20, end_line: 36, end_column: 28 } },
        /* terrane-site-row: site 16: /stable-list-sorting::main (case.trn:36:30-36:38) */
        { Site { function: 0, file: 0, line: 36, column: 30, end_line: 36, end_column: 38 } },
        /* terrane-site-row: site 17: /stable-list-sorting::main (case.trn:38:10-38:18) */
        { Site { function: 0, file: 0, line: 38, column: 10, end_line: 38, end_column: 18 } },
        /* terrane-site-row: site 18: /stable-list-sorting::main (case.trn:38:20-38:28) */
        { Site { function: 0, file: 0, line: 38, column: 20, end_line: 38, end_column: 28 } },
        /* terrane-site-row: site 19: /stable-list-sorting::main (case.trn:38:30-38:38) */
        { Site { function: 0, file: 0, line: 38, column: 30, end_line: 38, end_column: 38 } },
        /* terrane-site-row: site 20: /stable-list-sorting::main (case.trn:50:10-50:19) */
        { Site { function: 0, file: 0, line: 50, column: 10, end_line: 50, end_column: 19 } },
        /* terrane-site-row: site 21: /stable-list-sorting::main (case.trn:50:30-50:39) */
        { Site { function: 0, file: 0, line: 50, column: 30, end_line: 50, end_column: 39 } },
        /* terrane-site-row: site 22: /stable-list-sorting::main (case.trn:50:55-50:64) */
        { Site { function: 0, file: 0, line: 50, column: 55, end_line: 50, end_column: 64 } },
        /* terrane-site-row: site 23: /stable-list-sorting::main (case.trn:50:80-50:89) */
        { Site { function: 0, file: 0, line: 50, column: 80, end_line: 50, end_column: 89 } },
        /* terrane-site-row: site 24: /stable-list-sorting::main (case.trn:50:105-50:114) */
        { Site { function: 0, file: 0, line: 50, column: 105, end_line: 50, end_column: 114 } },
        /* terrane-site-row: site 25: /stable-list-sorting::main (case.trn:50:125-50:134) */
        { Site { function: 0, file: 0, line: 50, column: 125, end_line: 50, end_column: 134 } },
        /* terrane-site-row: site 26: /stable-list-sorting::main (case.trn:50:149-50:158) */
        { Site { function: 0, file: 0, line: 50, column: 149, end_line: 50, end_column: 158 } },
        /* terrane-site-row: site 27: /stable-list-sorting::main (case.trn:52:10-52:19) */
        { Site { function: 0, file: 0, line: 52, column: 10, end_line: 52, end_column: 19 } },
        /* terrane-site-row: site 28: /stable-list-sorting::main (case.trn:52:30-52:39) */
        { Site { function: 0, file: 0, line: 52, column: 30, end_line: 52, end_column: 39 } },
        /* terrane-site-row: site 29: /stable-list-sorting::main (case.trn:52:55-52:64) */
        { Site { function: 0, file: 0, line: 52, column: 55, end_line: 52, end_column: 64 } },
        /* terrane-site-row: site 30: /stable-list-sorting::main (case.trn:52:80-52:89) */
        { Site { function: 0, file: 0, line: 52, column: 80, end_line: 52, end_column: 89 } },
        /* terrane-site-row: site 31: /stable-list-sorting::main (case.trn:52:105-52:114) */
        { Site { function: 0, file: 0, line: 52, column: 105, end_line: 52, end_column: 114 } },
        /* terrane-site-row: site 32: /stable-list-sorting::main (case.trn:52:125-52:134) */
        { Site { function: 0, file: 0, line: 52, column: 125, end_line: 52, end_column: 134 } },
        /* terrane-site-row: site 33: /stable-list-sorting::main (case.trn:52:150-52:159) */
        { Site { function: 0, file: 0, line: 52, column: 150, end_line: 52, end_column: 159 } },
        /* terrane-site-row: site 34: /stable-list-sorting::main (case.trn:52:174-52:183) */
        { Site { function: 0, file: 0, line: 52, column: 174, end_line: 52, end_column: 183 } },
        /* terrane-site-row: site 35: /stable-list-sorting::main (case.trn:56:10-56:19) */
        { Site { function: 0, file: 0, line: 56, column: 10, end_line: 56, end_column: 19 } },
        /* terrane-site-row: site 36: /stable-list-sorting::main (case.trn:56:21-56:30) */
        { Site { function: 0, file: 0, line: 56, column: 21, end_line: 56, end_column: 30 } },
        /* terrane-site-row: site 37: /stable-list-sorting::main (case.trn:61:10-61:22) */
        { Site { function: 0, file: 0, line: 61, column: 10, end_line: 61, end_column: 22 } },
        /* terrane-site-row: site 38: /stable-list-sorting::main (case.trn:61:24-61:33) */
        { Site { function: 0, file: 0, line: 61, column: 24, end_line: 61, end_column: 33 } },
        /* terrane-site-row: site 39: /stable-list-sorting::main (case.trn:61:35-61:46) */
        { Site { function: 0, file: 0, line: 61, column: 35, end_line: 61, end_column: 46 } },
        /* terrane-site-row: site 40: /stable-list-sorting::main (case.trn:69:17-69:29) */
        { Site { function: 0, file: 0, line: 69, column: 17, end_line: 69, end_column: 29 } },
        /* terrane-site-row: site 41: /stable-list-sorting::main (case.trn:72:17-72:29) */
        { Site { function: 0, file: 0, line: 72, column: 17, end_line: 72, end_column: 29 } },
        /* terrane-site-row: site 42: /stable-list-sorting::main (case.trn:78:24-78:33) */
        { Site { function: 0, file: 0, line: 78, column: 24, end_line: 78, end_column: 33 } },
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
