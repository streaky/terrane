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
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 1] = ["/bounded-arithmetic-families::main"];
    pub static SITES: [Site; 21] = [
        /* terrane-site-row: site 0: /bounded-arithmetic-families::main (case.trn:8:10-8:26) */
        { Site { function: 0, file: 0, line: 8, column: 10, end_line: 8, end_column: 26 } },
        /* terrane-site-row: site 1: /bounded-arithmetic-families::main (case.trn:14:11-14:26) */
        { Site { function: 0, file: 0, line: 14, column: 11, end_line: 14, end_column: 26 } },
        /* terrane-site-row: site 2: /bounded-arithmetic-families::main (case.trn:14:30-14:48) */
        { Site { function: 0, file: 0, line: 14, column: 30, end_line: 14, end_column: 48 } },
        /* terrane-site-row: site 3: /bounded-arithmetic-families::main (case.trn:17:11-17:35) */
        { Site { function: 0, file: 0, line: 17, column: 11, end_line: 17, end_column: 35 } },
        /* terrane-site-row: site 4: /bounded-arithmetic-families::main (case.trn:18:3-18:31) */
        { Site { function: 0, file: 0, line: 18, column: 3, end_line: 18, end_column: 31 } },
        /* terrane-site-row: site 5: /bounded-arithmetic-families::main (case.trn:20:3-20:10) */
        { Site { function: 0, file: 0, line: 20, column: 3, end_line: 20, end_column: 10 } },
        /* terrane-site-row: site 6: /bounded-arithmetic-families::main (case.trn:21:3-21:10) */
        { Site { function: 0, file: 0, line: 21, column: 3, end_line: 21, end_column: 10 } },
        /* terrane-site-row: site 7: /bounded-arithmetic-families::main (case.trn:31:11-31:34) */
        { Site { function: 0, file: 0, line: 31, column: 11, end_line: 31, end_column: 34 } },
        /* terrane-site-row: site 8: /bounded-arithmetic-families::main (case.trn:31:38-31:65) */
        { Site { function: 0, file: 0, line: 31, column: 38, end_line: 31, end_column: 65 } },
        /* terrane-site-row: site 9: /bounded-arithmetic-families::main (case.trn:32:18-32:48) */
        { Site { function: 0, file: 0, line: 32, column: 18, end_line: 32, end_column: 48 } },
        /* terrane-site-row: site 10: /bounded-arithmetic-families::main (case.trn:33:56-33:82) */
        { Site { function: 0, file: 0, line: 33, column: 56, end_line: 33, end_column: 82 } },
        /* terrane-site-row: site 11: /bounded-arithmetic-families::main (case.trn:34:18-34:51) */
        { Site { function: 0, file: 0, line: 34, column: 18, end_line: 34, end_column: 51 } },
        /* terrane-site-row: site 12: /bounded-arithmetic-families::main (case.trn:35:11-35:37) */
        { Site { function: 0, file: 0, line: 35, column: 11, end_line: 35, end_column: 37 } },
        /* terrane-site-row: site 13: /bounded-arithmetic-families::main (case.trn:35:41-35:71) */
        { Site { function: 0, file: 0, line: 35, column: 41, end_line: 35, end_column: 71 } },
        /* terrane-site-row: site 14: /bounded-arithmetic-families::main (case.trn:36:56-36:85) */
        { Site { function: 0, file: 0, line: 36, column: 56, end_line: 36, end_column: 85 } },
        /* terrane-site-row: site 15: /bounded-arithmetic-families::main (case.trn:37:11-37:26) */
        { Site { function: 0, file: 0, line: 37, column: 11, end_line: 37, end_column: 26 } },
        /* terrane-site-row: site 16: /bounded-arithmetic-families::main (case.trn:37:30-37:48) */
        { Site { function: 0, file: 0, line: 37, column: 30, end_line: 37, end_column: 48 } },
        /* terrane-site-row: site 17: /bounded-arithmetic-families::main (case.trn:38:11-38:25) */
        { Site { function: 0, file: 0, line: 38, column: 11, end_line: 38, end_column: 25 } },
        /* terrane-site-row: site 18: /bounded-arithmetic-families::main (case.trn:38:29-38:46) */
        { Site { function: 0, file: 0, line: 38, column: 29, end_line: 38, end_column: 46 } },
        /* terrane-site-row: site 19: /bounded-arithmetic-families::main (case.trn:39:11-39:27) */
        { Site { function: 0, file: 0, line: 39, column: 11, end_line: 39, end_column: 27 } },
        /* terrane-site-row: site 20: /bounded-arithmetic-families::main (case.trn:39:31-39:50) */
        { Site { function: 0, file: 0, line: 39, column: 31, end_line: 39, end_column: 50 } },
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
// Namespace: bounded-arithmetic-families
fn main() {
    let small_terrane_f0_s55: i8;
    let wrapped_terrane_f0_s74: i8;
    let overflowed_terrane_f0_s127: terrane_int_support::OverflowResult<i8>;
    let pair_terrane_f0_s217: terrane_int_support::DivRemResult<
        terrane_int_support::Int,
    >;
    let exact_terrane_f0_s282: i64;
    let negated_terrane_f0_s443: terrane_int_support::OverflowResult<i8>;
    let mut count_terrane_f0_s591: i8;
    let sub_overflow_terrane_f0_s778: terrane_int_support::OverflowResult<i8>;
    let mul_overflow_terrane_f0_s990: terrane_int_support::OverflowResult<i8>;
    let minimum_terrane_f0_s1090: i8;
    let div_overflow_terrane_f0_s1178: terrane_int_support::OverflowResult<i8>;
    let rem_overflow_terrane_f0_s1317: terrane_int_support::OverflowResult<i8>;
    small_terrane_f0_s55 = 120;
    wrapped_terrane_f0_s74 = terrane_int_support::fixed_addition_wrap(
        small_terrane_f0_s55,
        10,
    );
    println!("{}", terrane_scalar_support::scalar_text(&wrapped_terrane_f0_s74));
    overflowed_terrane_f0_s127 = terrane_int_support::fixed_addition_overflowing(
        small_terrane_f0_s55,
        10,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflowed_terrane_f0_s127.value),
        terrane_scalar_support::scalar_text(&overflowed_terrane_f0_s127.overflowed)
    );
    pair_terrane_f0_s217 = __terrane_raised(
        terrane_int_support::Int::from(-7_i128)
            .div_rem(&terrane_int_support::Int::from(3_i128)),
        0 /* terrane-site: case.trn:8:10-8:26 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&pair_terrane_f0_s217.quotient),
        terrane_scalar_support::scalar_text(&pair_terrane_f0_s217.remainder)
    );
    exact_terrane_f0_s282 = 5;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(exact_terrane_f0_s282
        as i128) * terrane_int_support::Int::from(9_i128)))
    );
    terrane_int_support::fixed_subtraction_checked(small_terrane_f0_s55, 20);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_saturate(small_terrane_f0_s55,
        2))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division(small_terrane_f0_s55,
        3), 1 /* terrane-site: case.trn:14:11-14:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder(small_terrane_f0_s55,
        7), 2 /* terrane-site: case.trn:14:30-14:48 */))
    );
    negated_terrane_f0_s443 = terrane_int_support::fixed_negation_overflowing(
        small_terrane_f0_s55,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negated_terrane_f0_s443.value),
        terrane_scalar_support::scalar_text(&negated_terrane_f0_s443.overflowed)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_shift_left_wrap(small_terrane_f0_s55,
        &1), 3 /* terrane-site: case.trn:17:11-17:35 */))
    );
    __terrane_raised(
        terrane_int_support::fixed_shift_right_checked(small_terrane_f0_s55, &2),
        4 /* terrane-site: case.trn:18:3-18:31 */,
    );
    count_terrane_f0_s591 = 1;
    count_terrane_f0_s591 = __terrane_raised(
        terrane_int_support::fixed_addition(count_terrane_f0_s591, 1),
        5 /* terrane-site: case.trn:20:3-20:10 */,
    );
    count_terrane_f0_s591 = __terrane_raised(
        terrane_int_support::fixed_subtraction(count_terrane_f0_s591, 1),
        6 /* terrane-site: case.trn:21:3-21:10 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&count_terrane_f0_s591));
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_addition_checked(small_terrane_f0_s55,
        10).is_none()),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_addition_saturate(small_terrane_f0_s55,
        10))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_wrap(small_terrane_f0_s55,
        - 20)),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_saturate(small_terrane_f0_s55,
        - 20))
    );
    sub_overflow_terrane_f0_s778 = terrane_int_support::fixed_subtraction_overflowing(
        small_terrane_f0_s55,
        -20,
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&sub_overflow_terrane_f0_s778
        .value), terrane_scalar_support::scalar_text(&sub_overflow_terrane_f0_s778
        .overflowed),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_subtraction_checked(small_terrane_f0_s55,
        - 20).is_none())
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_wrap(small_terrane_f0_s55,
        2)),
        terrane_scalar_support::scalar_text(&terrane_int_support::fixed_multiplication_checked(small_terrane_f0_s55,
        2).is_none())
    );
    mul_overflow_terrane_f0_s990 = terrane_int_support::fixed_multiplication_overflowing(
        small_terrane_f0_s55,
        2,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&mul_overflow_terrane_f0_s990.value),
        terrane_scalar_support::scalar_text(&mul_overflow_terrane_f0_s990.overflowed)
    );
    minimum_terrane_f0_s1090 = -128;
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_wrap(minimum_terrane_f0_s1090,
        - 1), 7 /* terrane-site: case.trn:31:11-31:34 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_saturate(minimum_terrane_f0_s1090,
        - 1), 8 /* terrane-site: case.trn:31:38-31:65 */))
    );
    div_overflow_terrane_f0_s1178 = __terrane_raised(
        terrane_int_support::fixed_division_overflowing(minimum_terrane_f0_s1090, -1),
        9 /* terrane-site: case.trn:32:18-32:48 */,
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&div_overflow_terrane_f0_s1178
        .value), terrane_scalar_support::scalar_text(&div_overflow_terrane_f0_s1178
        .overflowed),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_division_checked(minimum_terrane_f0_s1090,
        - 1), 10 /* terrane-site: case.trn:33:56-33:82 */).is_none())
    );
    rem_overflow_terrane_f0_s1317 = __terrane_raised(
        terrane_int_support::fixed_remainder_overflowing(minimum_terrane_f0_s1090, -1),
        11 /* terrane-site: case.trn:34:18-34:51 */,
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_wrap(minimum_terrane_f0_s1090,
        - 1), 12 /* terrane-site: case.trn:35:11-35:37 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_saturate(minimum_terrane_f0_s1090,
        - 1), 13 /* terrane-site: case.trn:35:41-35:71 */))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&rem_overflow_terrane_f0_s1317
        .value), terrane_scalar_support::scalar_text(&rem_overflow_terrane_f0_s1317
        .overflowed),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_remainder_checked(minimum_terrane_f0_s1090,
        - 1), 14 /* terrane-site: case.trn:36:56-36:85 */).is_none())
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).euclidean_div(&terrane_int_support::Int::from(3_i128)),
        15 /* terrane-site: case.trn:37:11-37:26 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).modulo(&terrane_int_support::Int::from(3_i128)), 16 /* terrane-site: case.trn:37:30-37:48 */))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(7_i128)
        .euclidean_div(&- terrane_int_support::Int::from(3_i128)), 17 /* terrane-site: case.trn:38:11-38:25 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(7_i128)
        .modulo(&- terrane_int_support::Int::from(3_i128)), 18 /* terrane-site: case.trn:38:29-38:46 */))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).euclidean_div(&- terrane_int_support::Int::from(3_i128)),
        19 /* terrane-site: case.trn:39:11-39:27 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::Int::from(-
        7_i128).modulo(&- terrane_int_support::Int::from(3_i128)), 20 /* terrane-site: case.trn:39:31-39:50 */))
    );
}
