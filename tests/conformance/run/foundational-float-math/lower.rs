// Generated deterministically by Terrane 0.1.0.
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
    pub static FUNCTIONS: [&str; 1] = ["/foundational-float-math::main"];
    pub static SITES: [Site; 4] = [
        {
            /* terrane-site-row: site 0: /foundational-float-math::main (case.trn:14:30-14:39) */
            Site {
                function: 0,
                file: 0,
                line: 14,
                column: 30,
                end_line: 14,
                end_column: 39,
            }
        },
        {
            /* terrane-site-row: site 1: /foundational-float-math::main (case.trn:14:48-14:57) */
            Site {
                function: 0,
                file: 0,
                line: 14,
                column: 48,
                end_line: 14,
                end_column: 57,
            }
        },
        {
            /* terrane-site-row: site 2: /foundational-float-math::main (case.trn:25:30-25:39) */
            Site {
                function: 0,
                file: 0,
                line: 25,
                column: 30,
                end_line: 25,
                end_column: 39,
            }
        },
        {
            /* terrane-site-row: site 3: /foundational-float-math::main (case.trn:25:48-25:57) */
            Site {
                function: 0,
                file: 0,
                line: 25,
                column: 48,
                end_line: 25,
                end_column: 57,
            }
        },
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
// Namespace: foundational-float-math
fn main() {
    let zero32: f32 = 0.0_f32;
    let one32: f32 = 1.0_f32;
    let nine32: f32 = 9.0_f32;
    let root32: std::sync::Arc<dyn Fn() -> f32 + Send + Sync> = {
        let receiver = nine32;
        std::sync::Arc::new(move || receiver.sqrt())
    };
    println!("{}", terrane_scalar_support::scalar_text(&(root32() == 3.0_f32)));
    println!("{}", terrane_scalar_support::scalar_text(&(zero32.sin() == 0.0_f32)));
    println!("{}", terrane_scalar_support::scalar_text(&(zero32.cos() == 1.0_f32)));
    let pair32: terrane_collection_support::Tuple<f32> = {
        let terrane_sine_cosine = zero32.sin_cos();
        terrane_collection_support::Tuple::new(
            vec![terrane_sine_cosine.0, terrane_sine_cosine.1],
        )
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(terrane_int_support::Int::from(pair32
        .length())) == terrane_int_support::Int::from(2_i128))),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair32
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        0 /* terrane-site: case.trn:14:30-14:39 */)), 0 /* terrane-site: case.trn:14:30-14:39 */) == 0.0_f32)),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair32
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        1 /* terrane-site: case.trn:14:48-14:57 */)), 1 /* terrane-site: case.trn:14:48-14:57 */) == 1.0_f32))
    );
    println!("{}", terrane_scalar_support::scalar_text(&(one32.ln() == 0.0_f32)));
    println!("{}", terrane_scalar_support::scalar_text(&(zero32.exp() == 1.0_f32)));
    let zero64: f64 = 0.0;
    let one64: f64 = 1.0;
    let nine64: f64 = 9.0;
    println!("{}", terrane_scalar_support::scalar_text(&(nine64.sqrt() == 3.0)));
    println!("{}", terrane_scalar_support::scalar_text(&(zero64.sin() == 0.0)));
    println!("{}", terrane_scalar_support::scalar_text(&(zero64.cos() == 1.0)));
    let pair64: terrane_collection_support::Tuple<f64> = {
        let terrane_sine_cosine = zero64.sin_cos();
        terrane_collection_support::Tuple::new(
            vec![terrane_sine_cosine.0, terrane_sine_cosine.1],
        )
    };
    println!(
        "{}{}{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(terrane_int_support::Int::from(pair64
        .length())) == terrane_int_support::Int::from(2_i128))),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair64
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        2 /* terrane-site: case.trn:25:30-25:39 */)), 2 /* terrane-site: case.trn:25:30-25:39 */) == 0.0)),
        terrane_scalar_support::scalar_text(&(__terrane_raised(pair64
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        3 /* terrane-site: case.trn:25:48-25:57 */)), 3 /* terrane-site: case.trn:25:48-25:57 */) == 1.0))
    );
    println!("{}", terrane_scalar_support::scalar_text(&(one64.ln() == 0.0)));
    println!("{}", terrane_scalar_support::scalar_text(&(zero64.exp() == 1.0)));
    let negative: f64 = -1.0_f64;
    let not_a_number: f64 = negative.sqrt();
    println!("{}", terrane_scalar_support::scalar_text(&(not_a_number != not_a_number)));
    let negative_zero: f64 = -0.0_f64;
    let reciprocal: f64 = 1.0 / negative_zero.sqrt();
    println!("{}", terrane_scalar_support::scalar_text(&(reciprocal < 0.0)));
    let negative_infinity: f64 = zero64.ln();
    println!("{}", terrane_scalar_support::scalar_text(&(negative_infinity < 0.0)));
    let negative32: f32 = -3.0_f32;
    let low32: f32 = 2.0_f32;
    let high32: f32 = 5.0_f32;
    println!("{}", terrane_scalar_support::scalar_text(&(negative32.abs() == 3.0_f32)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let terrane_receiver = low32; let
        terrane_argument = high32; if terrane_receiver == 0.0_f32 &&terrane_argument ==
        0.0_f32 { if terrane_receiver.is_sign_negative() || terrane_argument
        .is_sign_negative() { - 0.0_f32 } else { 0.0_f32 } } else { terrane_receiver
        .min(terrane_argument) } } == 2.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let terrane_receiver = low32; let
        terrane_argument = high32; if terrane_receiver == 0.0_f32 &&terrane_argument ==
        0.0_f32 { if terrane_receiver.is_sign_negative() &&terrane_argument
        .is_sign_negative() { - 0.0_f32 } else { 0.0_f32 } } else { terrane_receiver
        .max(terrane_argument) } } == 5.0_f32))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(low32.mul_add(high32, 1.0_f32) ==
        11.0_f32))
    );
    println!("{}", terrane_scalar_support::scalar_text(&low32.is_finite()));
    let not_a_number32: f32 = negative32.sqrt();
    let infinity32: f32 = 1.0_f32 / zero32;
    println!("{}", terrane_scalar_support::scalar_text(&not_a_number32.is_nan()));
    println!("{}", terrane_scalar_support::scalar_text(&infinity32.is_infinite()));
    let low64: f64 = 2.0;
    let high64: f64 = 5.0;
    println!("{}", terrane_scalar_support::scalar_text(&(negative.abs() == 1.0)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let terrane_receiver = low64; let
        terrane_argument = high64; if terrane_receiver == 0.0_f64 &&terrane_argument ==
        0.0_f64 { if terrane_receiver.is_sign_negative() || terrane_argument
        .is_sign_negative() { - 0.0_f64 } else { 0.0_f64 } } else { terrane_receiver
        .min(terrane_argument) } } == 2.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let terrane_receiver = low64; let
        terrane_argument = high64; if terrane_receiver == 0.0_f64 &&terrane_argument ==
        0.0_f64 { if terrane_receiver.is_sign_negative() &&terrane_argument
        .is_sign_negative() { - 0.0_f64 } else { 0.0_f64 } } else { terrane_receiver
        .max(terrane_argument) } } == 5.0))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(low64.mul_add(high64, 1.0) == 11.0))
    );
    println!("{}", terrane_scalar_support::scalar_text(&low64.is_finite()));
    println!("{}", terrane_scalar_support::scalar_text(&not_a_number.is_nan()));
    println!(
        "{}", terrane_scalar_support::scalar_text(&negative_infinity.is_infinite())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let terrane_receiver =
        not_a_number; let terrane_argument = low64; if terrane_receiver == 0.0_f64
        &&terrane_argument == 0.0_f64 { if terrane_receiver.is_sign_negative() ||
        terrane_argument.is_sign_negative() { - 0.0_f64 } else { 0.0_f64 } } else {
        terrane_receiver.min(terrane_argument) } } == low64))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let terrane_receiver =
        not_a_number; let terrane_argument = high64; if terrane_receiver == 0.0_f64
        &&terrane_argument == 0.0_f64 { if terrane_receiver.is_sign_negative()
        &&terrane_argument.is_sign_negative() { - 0.0_f64 } else { 0.0_f64 } } else {
        terrane_receiver.max(terrane_argument) } } == high64))
    );
    let minimum_zero: f64 = {
        let terrane_receiver = negative_zero;
        let terrane_argument = zero64;
        if terrane_receiver == 0.0_f64 && terrane_argument == 0.0_f64 {
            if terrane_receiver.is_sign_negative() || terrane_argument.is_sign_negative()
            {
                -0.0_f64
            } else {
                0.0_f64
            }
        } else {
            terrane_receiver.min(terrane_argument)
        }
    };
    let maximum_zero: f64 = {
        let terrane_receiver = negative_zero;
        let terrane_argument = zero64;
        if terrane_receiver == 0.0_f64 && terrane_argument == 0.0_f64 {
            if terrane_receiver.is_sign_negative() && terrane_argument.is_sign_negative()
            {
                -0.0_f64
            } else {
                0.0_f64
            }
        } else {
            terrane_receiver.max(terrane_argument)
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&(1.0 / minimum_zero < 0.0)));
    println!("{}", terrane_scalar_support::scalar_text(&(1.0 / maximum_zero > 0.0)));
    let multiplicand: f64 = 1.0000000000000002;
    let multiplier: f64 = 1.0000000000000002;
    let addend: f64 = -1.0000000000000004_f64;
    let fused: f64 = multiplicand.mul_add(multiplier, addend);
    let unfused: f64 = multiplicand * multiplier + addend;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(fused ==
        0.00000000000000000000000000000004930380657631323784))
    );
    println!("{}", terrane_scalar_support::scalar_text(&(unfused == 0.0)));
    let multiplicand32: f32 = 1.0000001_f32;
    let multiplier32: f32 = 1.0000001_f32;
    let addend32: f32 = -1.0000002_f32;
    let fused32: f32 = multiplicand32.mul_add(multiplier32, addend32);
    let unfused32: f32 = multiplicand32 * multiplier32 + addend32;
    println!("{}", terrane_scalar_support::scalar_text(&(fused32 == 1.4210855e-14_f32)));
    println!("{}", terrane_scalar_support::scalar_text(&(unfused32 == 0.0_f32)));
}
// Generated Rust files: src/runtime/errors.rs, src/authored/case.trn.rs, src/main.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi
