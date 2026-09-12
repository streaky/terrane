// Generated deterministically by Terrane <version>.
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
    pub static SITES: [Site; 47] = [
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
        /* terrane-site-row: site 27: /stable-list-sorting::main (case.trn:50:174-50:183) */
        { Site { function: 0, file: 0, line: 50, column: 174, end_line: 50, end_column: 183 } },
        /* terrane-site-row: site 28: /stable-list-sorting::main (case.trn:50:198-50:207) */
        { Site { function: 0, file: 0, line: 50, column: 198, end_line: 50, end_column: 207 } },
        /* terrane-site-row: site 29: /stable-list-sorting::main (case.trn:52:10-52:19) */
        { Site { function: 0, file: 0, line: 52, column: 10, end_line: 52, end_column: 19 } },
        /* terrane-site-row: site 30: /stable-list-sorting::main (case.trn:52:30-52:39) */
        { Site { function: 0, file: 0, line: 52, column: 30, end_line: 52, end_column: 39 } },
        /* terrane-site-row: site 31: /stable-list-sorting::main (case.trn:52:55-52:64) */
        { Site { function: 0, file: 0, line: 52, column: 55, end_line: 52, end_column: 64 } },
        /* terrane-site-row: site 32: /stable-list-sorting::main (case.trn:52:80-52:89) */
        { Site { function: 0, file: 0, line: 52, column: 80, end_line: 52, end_column: 89 } },
        /* terrane-site-row: site 33: /stable-list-sorting::main (case.trn:52:105-52:114) */
        { Site { function: 0, file: 0, line: 52, column: 105, end_line: 52, end_column: 114 } },
        /* terrane-site-row: site 34: /stable-list-sorting::main (case.trn:52:125-52:134) */
        { Site { function: 0, file: 0, line: 52, column: 125, end_line: 52, end_column: 134 } },
        /* terrane-site-row: site 35: /stable-list-sorting::main (case.trn:52:150-52:159) */
        { Site { function: 0, file: 0, line: 52, column: 150, end_line: 52, end_column: 159 } },
        /* terrane-site-row: site 36: /stable-list-sorting::main (case.trn:52:174-52:183) */
        { Site { function: 0, file: 0, line: 52, column: 174, end_line: 52, end_column: 183 } },
        /* terrane-site-row: site 37: /stable-list-sorting::main (case.trn:52:199-52:208) */
        { Site { function: 0, file: 0, line: 52, column: 199, end_line: 52, end_column: 208 } },
        /* terrane-site-row: site 38: /stable-list-sorting::main (case.trn:52:223-52:232) */
        { Site { function: 0, file: 0, line: 52, column: 223, end_line: 52, end_column: 232 } },
        /* terrane-site-row: site 39: /stable-list-sorting::main (case.trn:56:10-56:19) */
        { Site { function: 0, file: 0, line: 56, column: 10, end_line: 56, end_column: 19 } },
        /* terrane-site-row: site 40: /stable-list-sorting::main (case.trn:56:21-56:30) */
        { Site { function: 0, file: 0, line: 56, column: 21, end_line: 56, end_column: 30 } },
        /* terrane-site-row: site 41: /stable-list-sorting::main (case.trn:61:10-61:22) */
        { Site { function: 0, file: 0, line: 61, column: 10, end_line: 61, end_column: 22 } },
        /* terrane-site-row: site 42: /stable-list-sorting::main (case.trn:61:24-61:33) */
        { Site { function: 0, file: 0, line: 61, column: 24, end_line: 61, end_column: 33 } },
        /* terrane-site-row: site 43: /stable-list-sorting::main (case.trn:61:35-61:46) */
        { Site { function: 0, file: 0, line: 61, column: 35, end_line: 61, end_column: 46 } },
        /* terrane-site-row: site 44: /stable-list-sorting::main (case.trn:69:17-69:29) */
        { Site { function: 0, file: 0, line: 69, column: 17, end_line: 69, end_column: 29 } },
        /* terrane-site-row: site 45: /stable-list-sorting::main (case.trn:72:17-72:29) */
        { Site { function: 0, file: 0, line: 72, column: 17, end_line: 72, end_column: 29 } },
        /* terrane-site-row: site 46: /stable-list-sorting::main (case.trn:78:24-78:33) */
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
// Source: case.trn
// Namespace: stable-list-sorting
fn main() {
    let mut adaptive: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from_decimal("900000000000000000000000000000000000000"),
            terrane_int_support::Int::from(- 2_i128),
            terrane_int_support::Int::from(7_i128),
            terrane_int_support::Int::from(7_i128)
        ],
    );
    adaptive.sort_by(|left, right| left.cmp(right));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:7:10-7:21 */)), 0 /* terrane-site: case.trn:7:10-7:21 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        1 /* terrane-site: case.trn:7:23-7:34 */)), 1 /* terrane-site: case.trn:7:23-7:34 */))
    );
    adaptive.sort_by(|left, right| right.cmp(left));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:9:10-9:21 */)), 2 /* terrane-site: case.trn:9:10-9:21 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(adaptive
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        3 /* terrane-site: case.trn:9:23-9:34 */)), 3 /* terrane-site: case.trn:9:23-9:34 */))
    );
    let mut int8s: terrane_collection_support::List<i8> = terrane_collection_support::List::<
        i8,
    >::new(vec![2, - 1]);
    let mut int16s: terrane_collection_support::List<i16> = terrane_collection_support::List::<
        i16,
    >::new(vec![2, - 1]);
    let mut int32s: terrane_collection_support::List<i32> = terrane_collection_support::List::<
        i32,
    >::new(vec![2, - 1]);
    let mut int64s: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(vec![2, - 1]);
    let mut int128s: terrane_collection_support::List<i128> = terrane_collection_support::List::<
        i128,
    >::new(vec![2, - 1]);
    let mut uint8s: terrane_collection_support::List<u8> = terrane_collection_support::List::<
        u8,
    >::new(vec![2, 1]);
    let mut uint16s: terrane_collection_support::List<u16> = terrane_collection_support::List::<
        u16,
    >::new(vec![2, 1]);
    let mut uint32s: terrane_collection_support::List<u32> = terrane_collection_support::List::<
        u32,
    >::new(vec![2, 1]);
    let mut uint64s: terrane_collection_support::List<u64> = terrane_collection_support::List::<
        u64,
    >::new(vec![2, 1]);
    let mut uint128s: terrane_collection_support::List<u128> = terrane_collection_support::List::<
        u128,
    >::new(vec![2, 1]);
    int8s.sort_by(|left, right| left.cmp(right));
    int16s.sort_by(|left, right| left.cmp(right));
    int32s.sort_by(|left, right| left.cmp(right));
    int64s.sort_by(|left, right| left.cmp(right));
    int128s.sort_by(|left, right| left.cmp(right));
    uint8s.sort_by(|left, right| left.cmp(right));
    uint16s.sort_by(|left, right| left.cmp(right));
    uint32s.sort_by(|left, right| left.cmp(right));
    uint64s.sort_by(|left, right| left.cmp(right));
    uint128s.sort_by(|left, right| left.cmp(right));
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(int8s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        4 /* terrane-site: case.trn:31:10-31:18 */)), 4 /* terrane-site: case.trn:31:10-31:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int16s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        5 /* terrane-site: case.trn:31:20-31:29 */)), 5 /* terrane-site: case.trn:31:20-31:29 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int32s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        6 /* terrane-site: case.trn:31:31-31:40 */)), 6 /* terrane-site: case.trn:31:31-31:40 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int64s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:31:42-31:51 */)), 7 /* terrane-site: case.trn:31:42-31:51 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(int128s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        8 /* terrane-site: case.trn:31:53-31:63 */)), 8 /* terrane-site: case.trn:31:53-31:63 */))
    );
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(uint8s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        9 /* terrane-site: case.trn:32:10-32:19 */)), 9 /* terrane-site: case.trn:32:10-32:19 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint16s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        10 /* terrane-site: case.trn:32:21-32:31 */)), 10 /* terrane-site: case.trn:32:21-32:31 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint32s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        11 /* terrane-site: case.trn:32:33-32:43 */)), 11 /* terrane-site: case.trn:32:33-32:43 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint64s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        12 /* terrane-site: case.trn:32:45-32:55 */)), 12 /* terrane-site: case.trn:32:45-32:55 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(uint128s
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        13 /* terrane-site: case.trn:32:57-32:68 */)), 13 /* terrane-site: case.trn:32:57-32:68 */))
    );
    let mut words: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("é"), String::from("e"), String::from("z")]);
    words.sort_by(|left, right| left.cmp(right));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        14 /* terrane-site: case.trn:36:10-36:18 */)), 14 /* terrane-site: case.trn:36:10-36:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        15 /* terrane-site: case.trn:36:20-36:28 */)), 15 /* terrane-site: case.trn:36:20-36:28 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        16 /* terrane-site: case.trn:36:30-36:38 */)), 16 /* terrane-site: case.trn:36:30-36:38 */))
    );
    words.sort_by(|left, right| right.cmp(left));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        17 /* terrane-site: case.trn:38:10-38:18 */)), 17 /* terrane-site: case.trn:38:10-38:18 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        18 /* terrane-site: case.trn:38:20-38:28 */)), 18 /* terrane-site: case.trn:38:20-38:28 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(words
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        19 /* terrane-site: case.trn:38:30-38:38 */)), 19 /* terrane-site: case.trn:38:30-38:38 */))
    );
    let zero: f64 = 0.0;
    let negative_zero: f64 = -0.0_f64;
    let one: f64 = 1.0;
    let negative_one: f64 = -1.0_f64;
    let infinity: f64 = one / zero;
    let negative_infinity: f64 = negative_one / zero;
    let first_nan: f64 = zero / zero;
    let second_nan: f64 = negative_zero / zero;
    let mut floats: terrane_collection_support::List<f64> = terrane_collection_support::List::<
        f64,
    >::new(
        vec![first_nan, negative_zero, infinity, zero, negative_infinity, second_nan],
    );
    floats.sort_by(terrane_collection_support::compare_float64_ascending);
    println!(
        "{}{}{}{}{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        20 /* terrane-site: case.trn:50:10-50:19 */)), 20 /* terrane-site: case.trn:50:10-50:19 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        21 /* terrane-site: case.trn:50:30-50:39 */)), 21 /* terrane-site: case.trn:50:30-50:39 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        22 /* terrane-site: case.trn:50:55-50:64 */)), 22 /* terrane-site: case.trn:50:55-50:64 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        23 /* terrane-site: case.trn:50:80-50:89 */)), 23 /* terrane-site: case.trn:50:80-50:89 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        24 /* terrane-site: case.trn:50:105-50:114 */)), 24 /* terrane-site: case.trn:50:105-50:114 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        25 /* terrane-site: case.trn:50:125-50:134 */)), 25 /* terrane-site: case.trn:50:125-50:134 */).is_nan()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        26 /* terrane-site: case.trn:50:149-50:158 */)), 26 /* terrane-site: case.trn:50:149-50:158 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(5_i128)),
        27 /* terrane-site: case.trn:50:174-50:183 */)), 27 /* terrane-site: case.trn:50:174-50:183 */).is_nan()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(5_i128)),
        28 /* terrane-site: case.trn:50:198-50:207 */)), 28 /* terrane-site: case.trn:50:198-50:207 */).is_sign_negative())
    );
    floats.sort_by(terrane_collection_support::compare_float64_descending);
    println!(
        "{}{}{}{}{}{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        29 /* terrane-site: case.trn:52:10-52:19 */)), 29 /* terrane-site: case.trn:52:10-52:19 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        30 /* terrane-site: case.trn:52:30-52:39 */)), 30 /* terrane-site: case.trn:52:30-52:39 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        31 /* terrane-site: case.trn:52:55-52:64 */)), 31 /* terrane-site: case.trn:52:55-52:64 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        32 /* terrane-site: case.trn:52:80-52:89 */)), 32 /* terrane-site: case.trn:52:80-52:89 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        33 /* terrane-site: case.trn:52:105-52:114 */)), 33 /* terrane-site: case.trn:52:105-52:114 */).is_infinite()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(3_i128)),
        34 /* terrane-site: case.trn:52:125-52:134 */)), 34 /* terrane-site: case.trn:52:125-52:134 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        35 /* terrane-site: case.trn:52:150-52:159 */)), 35 /* terrane-site: case.trn:52:150-52:159 */).is_nan()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(4_i128)),
        36 /* terrane-site: case.trn:52:174-52:183 */)), 36 /* terrane-site: case.trn:52:174-52:183 */).is_sign_negative()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(5_i128)),
        37 /* terrane-site: case.trn:52:199-52:208 */)), 37 /* terrane-site: case.trn:52:199-52:208 */).is_nan()),
        terrane_scalar_support::scalar_text(&__terrane_raised(floats
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(5_i128)),
        38 /* terrane-site: case.trn:52:223-52:232 */)), 38 /* terrane-site: case.trn:52:223-52:232 */).is_sign_negative())
    );
    let mut narrow: terrane_collection_support::List<f32> = terrane_collection_support::List::<
        f32,
    >::new(vec![2.0_f32, - 1.0_f32]);
    narrow.sort_by(terrane_collection_support::compare_float32_ascending);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(narrow
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        39 /* terrane-site: case.trn:56:10-56:19 */)), 39 /* terrane-site: case.trn:56:10-56:19 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(narrow
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        40 /* terrane-site: case.trn:56:21-56:30 */)), 40 /* terrane-site: case.trn:56:21-56:30 */))
    );
    let mut values: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(3_i128),
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let preserved: terrane_collection_support::List<terrane_int_support::Int> = values
        .clone();
    let mut returned: terrane_collection_support::List<terrane_int_support::Int> = {
        let collection = &mut values;
        collection.sort_by(|left, right| left.cmp(right));
        collection.clone()
    };
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(preserved
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        41 /* terrane-site: case.trn:61:10-61:22 */)), 41 /* terrane-site: case.trn:61:10-61:22 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        42 /* terrane-site: case.trn:61:24-61:33 */)), 42 /* terrane-site: case.trn:61:24-61:33 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(returned
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        43 /* terrane-site: case.trn:61:35-61:46 */)), 43 /* terrane-site: case.trn:61:35-61:46 */))
    );
    returned.append(terrane_int_support::Int::from(4_i128));
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(returned
        .length()))
    );
    let indexed: terrane_collection_support::Map<String, terrane_int_support::Int> = terrane_collection_support::Map::<
        String,
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_collection_support::Entry::new(String::from("c"),
            terrane_int_support::Int::from(3_i128)),
            terrane_collection_support::Entry::new(String::from("a"),
            terrane_int_support::Int::from(1_i128)),
            terrane_collection_support::Entry::new(String::from("b"),
            terrane_int_support::Int::from(2_i128))
        ],
    );
    let mut keys: terrane_collection_support::List<String> = indexed.keys();
    let ascending: terrane_collection_support::List<String> = {
        let collection = &mut keys;
        collection.sort_by(|left, right| left.cmp(right));
        collection.clone()
    };
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &ascending,
    );
    loop {
        let key = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&__terrane_raised(indexed
            .get_or_error(&key), 44 /* terrane-site: case.trn:69:17-69:29 */))
        );
    }
    let descending: terrane_collection_support::List<String> = {
        let collection = &mut keys;
        collection.sort_by(|left, right| right.cmp(left));
        collection.clone()
    };
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &descending,
    );
    loop {
        let key = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&key),
            terrane_scalar_support::scalar_text(&__terrane_raised(indexed
            .get_or_error(&key), 45 /* terrane-site: case.trn:72:17-72:29 */))
        );
    }
    let mut empty: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    empty.sort_by(|left, right| left.cmp(right));
    let mut single: terrane_collection_support::List<terrane_int_support::Int> = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    single.sort_by(|left, right| right.cmp(left));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(empty
        .length())), terrane_scalar_support::scalar_text(&__terrane_raised(single
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        46 /* terrane-site: case.trn:78:24-78:33 */)), 46 /* terrane-site: case.trn:78:24-78:33 */))
    );
}
