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
    pub static FUNCTIONS: [&str; 2] = [
        "/complete-float-surface::exercise32",
        "/complete-float-surface::exercise64",
    ];
    pub static SITES: [Site; 3] = [
        {
            /* terrane-site-row: site 0: /complete-float-surface::exercise32 (case.trn:50:29-50:38) */
            Site {
                function: 0,
                file: 0,
                line: 50,
                column: 29,
                end_line: 50,
                end_column: 38,
            }
        },
        {
            /* terrane-site-row: site 1: /complete-float-surface::exercise32 (case.trn:50:67-50:84) */
            Site {
                function: 0,
                file: 0,
                line: 50,
                column: 67,
                end_line: 50,
                end_column: 84,
            }
        },
        {
            /* terrane-site-row: site 2: /complete-float-surface::exercise64 (case.trn:115:69-115:86) */
            Site {
                function: 1,
                file: 0,
                line: 115,
                column: 69,
                end_line: 115,
                end_column: 86,
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
// Source: case.trn
// Namespace: complete-float-surface
fn exercise32() {
    let zero: f32 = 0.0_f32;
    let negative_zero: f32 = -0.0_f32;
    let one: f32 = 1.0_f32;
    let two: f32 = 2.0_f32;
    let three: f32 = 3.0_f32;
    let four: f32 = 4.0_f32;
    let negative_one: f32 = -1.0_f32;
    let negative_two: f32 = -2.0_f32;
    let negative_quarter: f32 = -0.25_f32;
    let fractional: f32 = -1.25_f32;
    let twelve: f32 = 12.0_f32;
    let thousand: f32 = 1000.0_f32;
    let overflow_input: f32 = 128.0_f32;
    let underflow_input: f32 = -150.0_f32;
    let eight: f32 = 8.0_f32;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(eight.cbrt() == two)),
        terrane_scalar_support::scalar_text(&(three.hypot(four) == 5.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(four.powf(0.5_f32) == two)),
        terrane_scalar_support::scalar_text(&(two.powi(10) == 1024.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three.exp2() == eight)),
        terrane_scalar_support::scalar_text(&(zero.exp_m1() == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero.ln_1p() == zero)),
        terrane_scalar_support::scalar_text(&(eight.log2() == three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(thousand.log10() == three)),
        terrane_scalar_support::scalar_text(&(eight.log(two) == three))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(zero.tan() == zero)),
        terrane_scalar_support::scalar_text(&(zero.asin() == zero)),
        terrane_scalar_support::scalar_text(&(one.acos() == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero.atan() == zero)),
        terrane_scalar_support::scalar_text(&(zero.atan2(negative_one) > three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(one.copysign(negative_zero) ==
        negative_one)), terrane_scalar_support::scalar_text(&negative_zero
        .is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let terrane_value : f32 = two;
        let terrane_lower : f32 = 3.0_f32; let terrane_upper : f32 = 4.0_f32; if
        terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan() ||
        terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } == three)), terrane_scalar_support::scalar_text(&({ let
        terrane_receiver : f32 = fractional; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else
        { terrane_fraction } } == negative_quarter))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&one.is_normal()),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::from_bits(1) } .is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero.next_up() == { let _ =
        &TerraneDescriptor { identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) }))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero.next_down() == negative_one * {
        let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1)
        }))
    );
    let decomposition: terrane_scalar_support::FloatDecomposition<f32> = terrane_scalar_support::decompose_f32(
        twelve,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition.mantissa ==
        0.75_f32)), terrane_scalar_support::scalar_text(&(decomposition.exponent == 4))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(decomposition
        .mantissa, decomposition.exponent) == twelve))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f32::RADIX)) } ==
        terrane_int_support::Int::from(2_i128))), terrane_scalar_support::scalar_text(&({
        let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f32::MANTISSA_DIGITS)) } ==
        terrane_int_support::Int::from(24_i128)))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::EPSILON } > zero)),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::MIN_POSITIVE } .is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) } >
        zero)), terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::MIN } < zero)),
        terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::MAX } > zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&two.asin().is_nan()),
        terrane_scalar_support::scalar_text(&negative_two.powf(0.5_f32).is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflow_input.exp2()
        .is_infinite()), terrane_scalar_support::scalar_text(&(underflow_input.exp2() ==
        zero))
    );
    let not_a_number: f32 = two.asin();
    let infinity: f32 = one / zero;
    let clamped_zero: f32 = {
        let terrane_value: f32 = negative_zero;
        let terrane_lower: f32 = zero;
        let terrane_upper: f32 = one;
        if terrane_value.is_nan() {
            terrane_value
        } else if terrane_lower.is_nan() || terrane_upper.is_nan()
            || terrane_lower > terrane_upper
        {
            f32::NAN
        } else {
            let terrane_lowered = if terrane_value == 0.0 && terrane_lower == 0.0 {
                if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() {
                    0.0
                } else {
                    -0.0
                }
            } else {
                terrane_value.max(terrane_lower)
            };
            if terrane_lowered == 0.0 && terrane_upper == 0.0 {
                if terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative()
                {
                    -0.0
                } else {
                    0.0
                }
            } else {
                terrane_lowered.min(terrane_upper)
            }
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&(one / clamped_zero > zero)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f32 =
        not_a_number; let terrane_lower : f32 = zero; let terrane_upper : f32 = one; if
        terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan() ||
        terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        terrane_value : f32 = one; let terrane_lower : f32 = not_a_number; let
        terrane_upper : f32 = one; if terrane_value.is_nan() { terrane_value } else if
        terrane_lower.is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper
        { f32::NAN } else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower
        == 0.0 { if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive()
        { 0.0 } else { - 0.0 } } else { terrane_value.max(terrane_lower) }; if
        terrane_lowered == 0.0 &&terrane_upper == 0.0 { if terrane_lowered
        .is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 } else { 0.0 } }
        else { terrane_lowered.min(terrane_upper) } } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&not_a_number.hypot(infinity)
        .is_infinite()), terrane_scalar_support::scalar_text(&infinity.next_up()
        .is_infinite())
    );
    let subnormal_decomposition: terrane_scalar_support::FloatDecomposition<f32> = terrane_scalar_support::decompose_f32({
        let _ = &TerraneDescriptor {
            identity: "float32",
            name: "float32",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        };
        f32::from_bits(1)
    });
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(subnormal_decomposition
        .mantissa, subnormal_decomposition.exponent) == { let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) }))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scale_binary_f32({
        let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f32::MAX }, 1)
        .is_infinite()),
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32({
        let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) },
        - 1) == zero))
    );
    let wide_four: f64 = 4.0;
    let floating_exponent: f64 = 2.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three.hypot({ let source_value =
        wide_four; let converted = source_value as f32; if converted as f64 ==
        source_value { converted } else {
        __terrane_raised(Err(terrane_int_support::ArithmeticError::conversion_overflow(&source_value,
        "float64", "float32", "the floating value is not exactly representable")),
        0 /* terrane-site: case.trn:50:29-50:38 */) } }) == 5.0_f32)),
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(one,
        __terrane_raised(terrane_int_support::exact_from_f64:: < i32 >
        (floating_exponent), 1 /* terrane-site: case.trn:50:67-50:84 */)) == four))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let terrane_receiver : f32 =
        negative_one; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else {
        terrane_fraction } } .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f32 = one; let
        terrane_lower : f32 = two; let terrane_upper : f32 = one; if terrane_value
        .is_nan() { terrane_value } else if terrane_lower.is_nan() || terrane_upper
        .is_nan() || terrane_lower > terrane_upper { f32::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } .is_nan())
    );
    let nan_decomposition: terrane_scalar_support::FloatDecomposition<f32> = terrane_scalar_support::decompose_f32(
        not_a_number,
    );
    let infinity_decomposition: terrane_scalar_support::FloatDecomposition<f32> = terrane_scalar_support::decompose_f32(
        infinity,
    );
    let negative_zero_decomposition: terrane_scalar_support::FloatDecomposition<f32> = terrane_scalar_support::decompose_f32(
        negative_zero,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&nan_decomposition.mantissa
        .is_nan()), terrane_scalar_support::scalar_text(&(nan_decomposition.exponent ==
        0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&infinity_decomposition.mantissa
        .is_infinite()), terrane_scalar_support::scalar_text(&(infinity_decomposition
        .exponent == 0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_zero_decomposition.mantissa
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&(negative_zero_decomposition.exponent == 0))
    );
    let negative_infinity: f32 = negative_one / zero;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_receiver : f32 =
        infinity; let terrane_fraction = terrane_receiver.fract(); if terrane_fraction ==
        0.0 { 0.0_f32.copysign(terrane_receiver) } else { terrane_fraction } }
        .is_nan()), terrane_scalar_support::scalar_text(&{ let terrane_receiver : f32 =
        negative_infinity; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else {
        terrane_fraction } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_infinity.next_down()
        .is_infinite()), terrane_scalar_support::scalar_text(&negative_infinity
        .next_down().is_sign_negative())
    );
    let large_exponent: f32 = 120.0_f32;
    let large: f32 = large_exponent.exp2();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(large,
        - 250) == terrane_scalar_support::scale_binary_f32(one, - 130)))
    );
    let descriptor: TerraneDescriptor = TerraneDescriptor {
        identity: "float32",
        name: "float32",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let _ = &descriptor; f32::EPSILON }
        == { let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f32::EPSILON }))
    );
}
fn exercise64() {
    let zero: f64 = 0.0;
    let negative_zero: f64 = -0.0_f64;
    let one: f64 = 1.0;
    let two: f64 = 2.0;
    let three: f64 = 3.0;
    let four: f64 = 4.0;
    let eight: f64 = 8.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(eight.cbrt() == two)),
        terrane_scalar_support::scalar_text(&(three.hypot(four) == 5.0))
    );
    let negative_one: f64 = -1.0_f64;
    let negative_two: f64 = -2.0_f64;
    let negative_quarter: f64 = -0.25_f64;
    let fractional: f64 = -1.25_f64;
    let twelve: f64 = 12.0;
    let thousand: f64 = 1000.0;
    let overflow_input: f64 = 1024.0;
    let underflow_input: f64 = -1075.0_f64;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(four.powf(0.5) == two)),
        terrane_scalar_support::scalar_text(&(two.powi(10) == 1024.0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three.exp2() == eight)),
        terrane_scalar_support::scalar_text(&(zero.exp_m1() == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero.ln_1p() == zero)),
        terrane_scalar_support::scalar_text(&(eight.log2() == three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(thousand.log10() == three)),
        terrane_scalar_support::scalar_text(&(eight.log(two) == three))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(zero.tan() == zero)),
        terrane_scalar_support::scalar_text(&(zero.asin() == zero)),
        terrane_scalar_support::scalar_text(&(one.acos() == zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero.atan() == zero)),
        terrane_scalar_support::scalar_text(&(zero.atan2(negative_one) > three))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(one.copysign(negative_zero) ==
        negative_one)), terrane_scalar_support::scalar_text(&negative_zero
        .is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let terrane_value : f64 = two;
        let terrane_lower : f64 = 3.0; let terrane_upper : f64 = 4.0; if terrane_value
        .is_nan() { terrane_value } else if terrane_lower.is_nan() || terrane_upper
        .is_nan() || terrane_lower > terrane_upper { f64::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } == three)), terrane_scalar_support::scalar_text(&({ let
        terrane_receiver : f64 = fractional; let terrane_fraction = terrane_receiver
        .fract(); if terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else
        { terrane_fraction } } == negative_quarter))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero == 0.0)),
        terrane_scalar_support::scalar_text(&one.is_normal()),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::from_bits(1) } .is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero.next_up() == { let _ =
        &TerraneDescriptor { identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) }))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero.next_down() == negative_one * {
        let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1)
        }))
    );
    let decomposition: terrane_scalar_support::FloatDecomposition<f64> = terrane_scalar_support::decompose_f64(
        twelve,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition.mantissa == 0.75)),
        terrane_scalar_support::scalar_text(&(decomposition.exponent == 4))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(decomposition
        .mantissa, decomposition.exponent) == twelve))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f64::RADIX)) } ==
        terrane_int_support::Int::from(2_i128))), terrane_scalar_support::scalar_text(&({
        let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] };
        terrane_int_support::Int::from(i128::from(f64::MANTISSA_DIGITS)) } ==
        terrane_int_support::Int::from(53_i128)))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::EPSILON } > zero)),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::MIN_POSITIVE } .is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) } >
        zero)), terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::MIN } < zero)),
        terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::MAX } > zero))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&two.asin().is_nan()),
        terrane_scalar_support::scalar_text(&negative_two.powf(0.5).is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflow_input.exp2()
        .is_infinite()), terrane_scalar_support::scalar_text(&(underflow_input.exp2() ==
        zero))
    );
    let not_a_number: f64 = two.asin();
    let infinity: f64 = one / zero;
    let clamped_zero: f64 = {
        let terrane_value: f64 = negative_zero;
        let terrane_lower: f64 = zero;
        let terrane_upper: f64 = one;
        if terrane_value.is_nan() {
            terrane_value
        } else if terrane_lower.is_nan() || terrane_upper.is_nan()
            || terrane_lower > terrane_upper
        {
            f64::NAN
        } else {
            let terrane_lowered = if terrane_value == 0.0 && terrane_lower == 0.0 {
                if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() {
                    0.0
                } else {
                    -0.0
                }
            } else {
                terrane_value.max(terrane_lower)
            };
            if terrane_lowered == 0.0 && terrane_upper == 0.0 {
                if terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative()
                {
                    -0.0
                } else {
                    0.0
                }
            } else {
                terrane_lowered.min(terrane_upper)
            }
        }
    };
    println!("{}", terrane_scalar_support::scalar_text(&(one / clamped_zero > zero)));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f64 =
        not_a_number; let terrane_lower : f64 = zero; let terrane_upper : f64 = one; if
        terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan() ||
        terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        terrane_value : f64 = one; let terrane_lower : f64 = not_a_number; let
        terrane_upper : f64 = one; if terrane_value.is_nan() { terrane_value } else if
        terrane_lower.is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper
        { f64::NAN } else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower
        == 0.0 { if terrane_value.is_sign_positive() || terrane_lower.is_sign_positive()
        { 0.0 } else { - 0.0 } } else { terrane_value.max(terrane_lower) }; if
        terrane_lowered == 0.0 &&terrane_upper == 0.0 { if terrane_lowered
        .is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 } else { 0.0 } }
        else { terrane_lowered.min(terrane_upper) } } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&not_a_number.hypot(infinity)
        .is_infinite()), terrane_scalar_support::scalar_text(&infinity.next_up()
        .is_infinite())
    );
    let subnormal_decomposition: terrane_scalar_support::FloatDecomposition<f64> = terrane_scalar_support::decompose_f64({
        let _ = &TerraneDescriptor {
            identity: "float64",
            name: "float64",
            kind: "type",
            inherently_identity_bearing: false,
            fields: &[],
        };
        f64::from_bits(1)
    });
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(subnormal_decomposition
        .mantissa, subnormal_decomposition.exponent) == { let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) }))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&terrane_scalar_support::scale_binary_f64({
        let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f64::MAX }, 1)
        .is_infinite()),
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64({
        let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) },
        - 1) == zero))
    );
    let narrow_four: f32 = 4.0_f32;
    let floating_exponent: f64 = 2.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three.hypot(narrow_four as f64) ==
        5.0)),
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(one,
        __terrane_raised(terrane_int_support::exact_from_f64:: < i32 >
        (floating_exponent), 2 /* terrane-site: case.trn:115:69-115:86 */)) == four))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let terrane_receiver : f64 =
        negative_one; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else {
        terrane_fraction } } .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f64 = one; let
        terrane_lower : f64 = two; let terrane_upper : f64 = one; if terrane_value
        .is_nan() { terrane_value } else if terrane_lower.is_nan() || terrane_upper
        .is_nan() || terrane_lower > terrane_upper { f64::NAN } else { let
        terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } .is_nan())
    );
    let nan_decomposition: terrane_scalar_support::FloatDecomposition<f64> = terrane_scalar_support::decompose_f64(
        not_a_number,
    );
    let infinity_decomposition: terrane_scalar_support::FloatDecomposition<f64> = terrane_scalar_support::decompose_f64(
        infinity,
    );
    let negative_zero_decomposition: terrane_scalar_support::FloatDecomposition<f64> = terrane_scalar_support::decompose_f64(
        negative_zero,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&nan_decomposition.mantissa
        .is_nan()), terrane_scalar_support::scalar_text(&(nan_decomposition.exponent ==
        0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&infinity_decomposition.mantissa
        .is_infinite()), terrane_scalar_support::scalar_text(&(infinity_decomposition
        .exponent == 0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_zero_decomposition.mantissa
        .is_sign_negative()),
        terrane_scalar_support::scalar_text(&(negative_zero_decomposition.exponent == 0))
    );
    let negative_infinity: f64 = negative_one / zero;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_receiver : f64 =
        infinity; let terrane_fraction = terrane_receiver.fract(); if terrane_fraction ==
        0.0 { 0.0_f64.copysign(terrane_receiver) } else { terrane_fraction } }
        .is_nan()), terrane_scalar_support::scalar_text(&{ let terrane_receiver : f64 =
        negative_infinity; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else {
        terrane_fraction } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_infinity.next_down()
        .is_infinite()), terrane_scalar_support::scalar_text(&negative_infinity
        .next_down().is_sign_negative())
    );
    let large_exponent: f64 = 1000.0;
    let large: f64 = large_exponent.exp2();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(large,
        - 2000) == terrane_scalar_support::scale_binary_f64(one, - 1000)))
    );
    let descriptor: TerraneDescriptor = TerraneDescriptor {
        identity: "float64",
        name: "float64",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let _ = &descriptor; f64::EPSILON }
        == { let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f64::EPSILON }))
    );
}
fn main() {
    exercise32();
    exercise64();
}
