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
    pub static FUNCTIONS: [&str; 1] = ["/byte-index-and-slice::main"];
    pub static SITES: [Site; 19] = [
        /* terrane-site-row: site 0: /byte-index-and-slice::main (case.trn:9:10-9:17) */
        { Site { function: 0, file: 0, line: 9, column: 10, end_line: 9, end_column: 17 } },
        /* terrane-site-row: site 1: /byte-index-and-slice::main (case.trn:9:19-9:26) */
        { Site { function: 0, file: 0, line: 9, column: 19, end_line: 9, end_column: 26 } },
        /* terrane-site-row: site 2: /byte-index-and-slice::main (case.trn:10:23-10:34) */
        { Site { function: 0, file: 0, line: 10, column: 23, end_line: 10, end_column: 34 } },
        /* terrane-site-row: site 3: /byte-index-and-slice::main (case.trn:10:18-10:35) */
        { Site { function: 0, file: 0, line: 10, column: 18, end_line: 10, end_column: 35 } },
        /* terrane-site-row: site 4: /byte-index-and-slice::main (case.trn:11:11-11:30) */
        { Site { function: 0, file: 0, line: 11, column: 11, end_line: 11, end_column: 30 } },
        /* terrane-site-row: site 5: /byte-index-and-slice::main (case.trn:12:24-12:38) */
        { Site { function: 0, file: 0, line: 12, column: 24, end_line: 12, end_column: 38 } },
        /* terrane-site-row: site 6: /byte-index-and-slice::main (case.trn:12:19-12:39) */
        { Site { function: 0, file: 0, line: 12, column: 19, end_line: 12, end_column: 39 } },
        /* terrane-site-row: site 7: /byte-index-and-slice::main (case.trn:13:10-13:20) */
        { Site { function: 0, file: 0, line: 13, column: 10, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 8: /byte-index-and-slice::main (case.trn:13:22-13:32) */
        { Site { function: 0, file: 0, line: 13, column: 22, end_line: 13, end_column: 32 } },
        /* terrane-site-row: site 9: /byte-index-and-slice::main (case.trn:13:34-13:44) */
        { Site { function: 0, file: 0, line: 13, column: 34, end_line: 13, end_column: 44 } },
        /* terrane-site-row: site 10: /byte-index-and-slice::main (case.trn:14:22-14:53) */
        { Site { function: 0, file: 0, line: 14, column: 22, end_line: 14, end_column: 53 } },
        /* terrane-site-row: site 11: /byte-index-and-slice::main (case.trn:14:17-14:54) */
        { Site { function: 0, file: 0, line: 14, column: 17, end_line: 14, end_column: 54 } },
        /* terrane-site-row: site 12: /byte-index-and-slice::main (case.trn:16:22-16:41) */
        { Site { function: 0, file: 0, line: 16, column: 22, end_line: 16, end_column: 41 } },
        /* terrane-site-row: site 13: /byte-index-and-slice::main (case.trn:16:17-16:42) */
        { Site { function: 0, file: 0, line: 16, column: 17, end_line: 16, end_column: 42 } },
        /* terrane-site-row: site 14: /byte-index-and-slice::main (case.trn:17:10-17:18) */
        { Site { function: 0, file: 0, line: 17, column: 10, end_line: 17, end_column: 18 } },
        /* terrane-site-row: site 15: /byte-index-and-slice::main (case.trn:19:12-19:29) */
        { Site { function: 0, file: 0, line: 19, column: 12, end_line: 19, end_column: 29 } },
        /* terrane-site-row: site 16: /byte-index-and-slice::main (case.trn:23:17-23:28) */
        { Site { function: 0, file: 0, line: 23, column: 17, end_line: 23, end_column: 28 } },
        /* terrane-site-row: site 17: /byte-index-and-slice::main (case.trn:23:12-23:29) */
        { Site { function: 0, file: 0, line: 23, column: 12, end_line: 23, end_column: 29 } },
        /* terrane-site-row: site 18: /byte-index-and-slice::main (case.trn:27:12-27:21) */
        { Site { function: 0, file: 0, line: 27, column: 12, end_line: 27, end_column: 21 } },
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
// Namespace: byte-index-and-slice
fn main() {
    let data: Vec<u8> = terrane_string_support::encode(
        &String::from("A👍"),
        terrane_string_support::Encoding::Utf8,
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&data,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:9:10-9:17 */)), 0 /* terrane-site: case.trn:9:10-9:17 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&data,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:9:19-9:26 */)), 1 /* terrane-site: case.trn:9:19-9:26 */))
    );
    let middle: Vec<u8> = __terrane_raised(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(1_i128),
                    terrane_int_support::Int::from(5_i128),
                    terrane_int_support::Int::from(1_i64),
                ),
                2 /* terrane-site: case.trn:10:23-10:34 */,
            ),
        ),
        3 /* terrane-site: case.trn:10:18-10:35 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&middle,
        terrane_string_support::Encoding::Utf8), 4 /* terrane-site: case.trn:11:11-11:30 */))
    );
    let stepped: Vec<u8> = __terrane_raised(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(0_i128),
                    terrane_int_support::Int::from(5_i128),
                    terrane_int_support::Int::from(2_i128),
                ),
                5 /* terrane-site: case.trn:12:24-12:38 */,
            ),
        ),
        6 /* terrane-site: case.trn:12:19-12:39 */,
    );
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&stepped,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        7 /* terrane-site: case.trn:13:10-13:20 */)), 7 /* terrane-site: case.trn:13:10-13:20 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&stepped,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        8 /* terrane-site: case.trn:13:22-13:32 */)), 8 /* terrane-site: case.trn:13:22-13:32 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&stepped,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(2_i128)),
        9 /* terrane-site: case.trn:13:34-13:44 */)), 9 /* terrane-site: case.trn:13:34-13:44 */))
    );
    let empty: Vec<u8> = __terrane_raised(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised(
                terrane_collection_support::Range::new(
                    terrane_int_support::Int::from(data.len() as i128),
                    terrane_int_support::Int::from(data.len() as i128),
                    terrane_int_support::Int::from(1_i64),
                ),
                10 /* terrane-site: case.trn:14:22-14:53 */,
            ),
        ),
        11 /* terrane-site: case.trn:14:17-14:54 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(empty.len() as i128)));
    let __trn_66696e616c: Vec<u8> = __terrane_raised(
        terrane_collection_support::byte_slice(
            &data,
            &__terrane_raised(
                terrane_collection_support::Range::through(
                    terrane_int_support::Int::from(4_i128),
                    terrane_int_support::Int::from(4_i128),
                    terrane_int_support::Int::from(1_i64),
                ),
                12 /* terrane-site: case.trn:16:22-16:41 */,
            ),
        ),
        13 /* terrane-site: case.trn:16:17-16:42 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&__trn_66696e616c,
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        14 /* terrane-site: case.trn:17:10-17:18 */)), 14 /* terrane-site: case.trn:17:10-17:18 */))
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_collection_support::byte_at(&data,
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(data
                .len() as i128)), 15 /* terrane-site: case.trn:19:12-19:29 */)),
                15 /* terrane-site: case.trn:19:12-19:29 */))
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
                    && __terrane_error_0.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&String::from("index"))
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
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&(__terrane_raised_completion!(terrane_collection_support::byte_slice(&data,
                &__terrane_raised_completion!(terrane_collection_support::Range::new(terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(6_i128),
                terrane_int_support::Int::from(1_i64)), 16 /* terrane-site: case.trn:23:17-23:28 */)), 17 /* terrane-site: case.trn:23:12-23:29 */) .len() as i128))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1
                    && __terrane_error_1.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_1 = true;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&String::from("slice"))
                    );
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_collection_support::byte_at(&data,
                __terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(-
                1_i128)), 18 /* terrane-site: case.trn:27:12-27:21 */)),
                18 /* terrane-site: case.trn:27:12-27:21 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_2 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_2) => {
                let mut __terrane_handled_2 = false;
                if !__terrane_handled_2
                    && __terrane_error_2.kind == TerraneErrorKind::IndexError
                {
                    __terrane_handled_2 = true;
                    let failure = __terrane_error_2.clone();
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&failure.message()
                        .to_owned())
                    );
                }
                if !__terrane_handled_2 {
                    return TerraneCompletion::Error(__terrane_error_2);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
