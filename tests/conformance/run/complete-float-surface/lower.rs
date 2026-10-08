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
    pub static FUNCTIONS: [&str; 2] = [
        "/complete-float-surface::exercise32",
        "/complete-float-surface::exercise64",
    ];
    pub static SITES: [Site; 3] = [
        /* terrane-site-row: site 0: /complete-float-surface::exercise32 (case.trn:50:29-50:38) */
        { Site { function: 0, file: 0, line: 50, column: 29, end_line: 50, end_column: 38 } },
        /* terrane-site-row: site 1: /complete-float-surface::exercise32 (case.trn:50:67-50:84) */
        { Site { function: 0, file: 0, line: 50, column: 67, end_line: 50, end_column: 84 } },
        /* terrane-site-row: site 2: /complete-float-surface::exercise64 (case.trn:115:69-115:86) */
        { Site { function: 1, file: 0, line: 115, column: 69, end_line: 115, end_column: 86 } },
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
    let zero_terrane_f0_s57: f32;
    let negative_zero_terrane_f0_s78: f32;
    let one_terrane_f0_s109: f32;
    let two_terrane_f0_s129: f32;
    let three_terrane_f0_s149: f32;
    let four_terrane_f0_s171: f32;
    let negative_one_terrane_f0_s192: f32;
    let negative_two_terrane_f0_s222: f32;
    let negative_quarter_terrane_f0_s252: f32;
    let fractional_terrane_f0_s287: f32;
    let twelve_terrane_f0_s316: f32;
    let thousand_terrane_f0_s340: f32;
    let overflow_input_terrane_f0_s368: f32;
    let underflow_input_terrane_f0_s401: f32;
    let eight_terrane_f0_s436: f32;
    let decomposition_terrane_f0_s1418: terrane_scalar_support::FloatDecomposition<f32>;
    let not_a_number_terrane_f0_s2019: f32;
    let infinity_terrane_f0_s2060: f32;
    let clamped_zero_terrane_f0_s2092: f32;
    let subnormal_decomposition_terrane_f0_s2368: terrane_scalar_support::FloatDecomposition<
        f32,
    >;
    let wide_four_terrane_f0_s2688: f64;
    let floating_exponent_terrane_f0_s2714: f64;
    let nan_decomposition_terrane_f0_s2940: terrane_scalar_support::FloatDecomposition<
        f32,
    >;
    let infinity_decomposition_terrane_f0_s2988: terrane_scalar_support::FloatDecomposition<
        f32,
    >;
    let negative_zero_decomposition_terrane_f0_s3037: terrane_scalar_support::FloatDecomposition<
        f32,
    >;
    let negative_infinity_terrane_f0_s3369: f32;
    let large_exponent_terrane_f0_s3615: f32;
    let large_terrane_f0_s3648: f32;
    let descriptor_terrane_f0_s3767: TerraneDescriptor;
    zero_terrane_f0_s57 = 0.0_f32;
    negative_zero_terrane_f0_s78 = -0.0_f32;
    one_terrane_f0_s109 = 1.0_f32;
    two_terrane_f0_s129 = 2.0_f32;
    three_terrane_f0_s149 = 3.0_f32;
    four_terrane_f0_s171 = 4.0_f32;
    negative_one_terrane_f0_s192 = -1.0_f32;
    negative_two_terrane_f0_s222 = -2.0_f32;
    negative_quarter_terrane_f0_s252 = -0.25_f32;
    fractional_terrane_f0_s287 = -1.25_f32;
    twelve_terrane_f0_s316 = 12.0_f32;
    thousand_terrane_f0_s340 = 1000.0_f32;
    overflow_input_terrane_f0_s368 = 128.0_f32;
    underflow_input_terrane_f0_s401 = -150.0_f32;
    eight_terrane_f0_s436 = 8.0_f32;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(eight_terrane_f0_s436.cbrt() ==
        two_terrane_f0_s129)),
        terrane_scalar_support::scalar_text(&(three_terrane_f0_s149
        .hypot(four_terrane_f0_s171) == 5.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(four_terrane_f0_s171.powf(0.5_f32)
        == two_terrane_f0_s129)),
        terrane_scalar_support::scalar_text(&(two_terrane_f0_s129.powi(10) ==
        1024.0_f32))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three_terrane_f0_s149.exp2() ==
        eight_terrane_f0_s436)),
        terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57.exp_m1() ==
        zero_terrane_f0_s57))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57.ln_1p() ==
        zero_terrane_f0_s57)),
        terrane_scalar_support::scalar_text(&(eight_terrane_f0_s436.log2() ==
        three_terrane_f0_s149))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(thousand_terrane_f0_s340.log10() ==
        three_terrane_f0_s149)),
        terrane_scalar_support::scalar_text(&(eight_terrane_f0_s436
        .log(two_terrane_f0_s129) == three_terrane_f0_s149))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57.tan() ==
        zero_terrane_f0_s57)), terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57
        .asin() == zero_terrane_f0_s57)),
        terrane_scalar_support::scalar_text(&(one_terrane_f0_s109.acos() ==
        zero_terrane_f0_s57))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57.atan() ==
        zero_terrane_f0_s57)), terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57
        .atan2(negative_one_terrane_f0_s192) > three_terrane_f0_s149))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(one_terrane_f0_s109
        .copysign(negative_zero_terrane_f0_s78) == negative_one_terrane_f0_s192)),
        terrane_scalar_support::scalar_text(&negative_zero_terrane_f0_s78
        .is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let terrane_value : f32 =
        two_terrane_f0_s129; let terrane_lower : f32 = 3.0_f32; let terrane_upper : f32 =
        4.0_f32; if terrane_value.is_nan() { terrane_value } else if terrane_lower
        .is_nan() || terrane_upper.is_nan() || terrane_lower > terrane_upper { f32::NAN }
        else { let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } == three_terrane_f0_s149)),
        terrane_scalar_support::scalar_text(&({ let terrane_receiver : f32 =
        fractional_terrane_f0_s287; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else {
        terrane_fraction } } == negative_quarter_terrane_f0_s252))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57 == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero_terrane_f0_s78 == 0.0)),
        terrane_scalar_support::scalar_text(&one_terrane_f0_s109.is_normal()),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::from_bits(1) } .is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57.next_up() == {
        let _ = &TerraneDescriptor { identity : "float32", name : "float32", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1)
        }))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s57.next_down() ==
        negative_one_terrane_f0_s192 * { let _ = &TerraneDescriptor { identity :
        "float32", name : "float32", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f32::from_bits(1) }))
    );
    decomposition_terrane_f0_s1418 = terrane_scalar_support::decompose_f32(
        twelve_terrane_f0_s316,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition_terrane_f0_s1418
        .mantissa == 0.75_f32)),
        terrane_scalar_support::scalar_text(&(decomposition_terrane_f0_s1418.exponent ==
        4))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(decomposition_terrane_f0_s1418
        .mantissa, decomposition_terrane_f0_s1418.exponent) == twelve_terrane_f0_s316))
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
        inherently_identity_bearing : false, fields : &[] }; f32::EPSILON } >
        zero_terrane_f0_s57)), terrane_scalar_support::scalar_text(&{ let _ =
        &TerraneDescriptor { identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::MIN_POSITIVE }
        .is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::from_bits(1) } >
        zero_terrane_f0_s57)), terrane_scalar_support::scalar_text(&({ let _ =
        &TerraneDescriptor { identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::MIN } <
        zero_terrane_f0_s57)), terrane_scalar_support::scalar_text(&({ let _ =
        &TerraneDescriptor { identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::MAX } >
        zero_terrane_f0_s57))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&two_terrane_f0_s129.asin()
        .is_nan()), terrane_scalar_support::scalar_text(&negative_two_terrane_f0_s222
        .powf(0.5_f32).is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflow_input_terrane_f0_s368
        .exp2().is_infinite()),
        terrane_scalar_support::scalar_text(&(underflow_input_terrane_f0_s401.exp2() ==
        zero_terrane_f0_s57))
    );
    not_a_number_terrane_f0_s2019 = two_terrane_f0_s129.asin();
    infinity_terrane_f0_s2060 = one_terrane_f0_s109 / zero_terrane_f0_s57;
    clamped_zero_terrane_f0_s2092 = {
        let terrane_value: f32 = negative_zero_terrane_f0_s78;
        let terrane_lower: f32 = zero_terrane_f0_s57;
        let terrane_upper: f32 = one_terrane_f0_s109;
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
    println!(
        "{}", terrane_scalar_support::scalar_text(&(one_terrane_f0_s109 /
        clamped_zero_terrane_f0_s2092 > zero_terrane_f0_s57))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f32 =
        not_a_number_terrane_f0_s2019; let terrane_lower : f32 = zero_terrane_f0_s57; let
        terrane_upper : f32 = one_terrane_f0_s109; if terrane_value.is_nan() {
        terrane_value } else if terrane_lower.is_nan() || terrane_upper.is_nan() ||
        terrane_lower > terrane_upper { f32::NAN } else { let terrane_lowered = if
        terrane_value == 0.0 &&terrane_lower == 0.0 { if terrane_value.is_sign_positive()
        || terrane_lower.is_sign_positive() { 0.0 } else { - 0.0 } } else { terrane_value
        .max(terrane_lower) }; if terrane_lowered == 0.0 &&terrane_upper == 0.0 { if
        terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 }
        else { 0.0 } } else { terrane_lowered.min(terrane_upper) } } } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let terrane_value : f32 =
        one_terrane_f0_s109; let terrane_lower : f32 = not_a_number_terrane_f0_s2019; let
        terrane_upper : f32 = one_terrane_f0_s109; if terrane_value.is_nan() {
        terrane_value } else if terrane_lower.is_nan() || terrane_upper.is_nan() ||
        terrane_lower > terrane_upper { f32::NAN } else { let terrane_lowered = if
        terrane_value == 0.0 &&terrane_lower == 0.0 { if terrane_value.is_sign_positive()
        || terrane_lower.is_sign_positive() { 0.0 } else { - 0.0 } } else { terrane_value
        .max(terrane_lower) }; if terrane_lowered == 0.0 &&terrane_upper == 0.0 { if
        terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 }
        else { 0.0 } } else { terrane_lowered.min(terrane_upper) } } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&not_a_number_terrane_f0_s2019
        .hypot(infinity_terrane_f0_s2060).is_infinite()),
        terrane_scalar_support::scalar_text(&infinity_terrane_f0_s2060.next_up()
        .is_infinite())
    );
    subnormal_decomposition_terrane_f0_s2368 = terrane_scalar_support::decompose_f32({
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
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(subnormal_decomposition_terrane_f0_s2368
        .mantissa, subnormal_decomposition_terrane_f0_s2368.exponent) == { let _ =
        &TerraneDescriptor { identity : "float32", name : "float32", kind : "type",
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
        - 1) == zero_terrane_f0_s57))
    );
    wide_four_terrane_f0_s2688 = 4.0;
    floating_exponent_terrane_f0_s2714 = 2.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three_terrane_f0_s149.hypot({ let
        source_value = wide_four_terrane_f0_s2688; let converted = source_value as f32;
        if converted as f64 == source_value { converted } else {
        __terrane_raised(Err(terrane_int_support::ArithmeticError::conversion_overflow(&source_value,
        "float64", "float32", "the floating value is not exactly representable")),
        0 /* terrane-site: case.trn:50:29-50:38 */) } }) == 5.0_f32)),
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(one_terrane_f0_s109,
        __terrane_raised(terrane_int_support::exact_from_f64:: < i32 >
        (floating_exponent_terrane_f0_s2714), 1 /* terrane-site: case.trn:50:67-50:84 */)) == four_terrane_f0_s171))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let terrane_receiver : f32 =
        negative_one_terrane_f0_s192; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else {
        terrane_fraction } } .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f32 =
        one_terrane_f0_s109; let terrane_lower : f32 = two_terrane_f0_s129; let
        terrane_upper : f32 = one_terrane_f0_s109; if terrane_value.is_nan() {
        terrane_value } else if terrane_lower.is_nan() || terrane_upper.is_nan() ||
        terrane_lower > terrane_upper { f32::NAN } else { let terrane_lowered = if
        terrane_value == 0.0 &&terrane_lower == 0.0 { if terrane_value.is_sign_positive()
        || terrane_lower.is_sign_positive() { 0.0 } else { - 0.0 } } else { terrane_value
        .max(terrane_lower) }; if terrane_lowered == 0.0 &&terrane_upper == 0.0 { if
        terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 }
        else { 0.0 } } else { terrane_lowered.min(terrane_upper) } } } .is_nan())
    );
    nan_decomposition_terrane_f0_s2940 = terrane_scalar_support::decompose_f32(
        not_a_number_terrane_f0_s2019,
    );
    infinity_decomposition_terrane_f0_s2988 = terrane_scalar_support::decompose_f32(
        infinity_terrane_f0_s2060,
    );
    negative_zero_decomposition_terrane_f0_s3037 = terrane_scalar_support::decompose_f32(
        negative_zero_terrane_f0_s78,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&nan_decomposition_terrane_f0_s2940
        .mantissa.is_nan()),
        terrane_scalar_support::scalar_text(&(nan_decomposition_terrane_f0_s2940.exponent
        == 0))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&infinity_decomposition_terrane_f0_s2988
        .mantissa.is_infinite()),
        terrane_scalar_support::scalar_text(&(infinity_decomposition_terrane_f0_s2988
        .exponent == 0))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&negative_zero_decomposition_terrane_f0_s3037
        .mantissa.is_sign_negative()),
        terrane_scalar_support::scalar_text(&(negative_zero_decomposition_terrane_f0_s3037
        .exponent == 0))
    );
    negative_infinity_terrane_f0_s3369 = negative_one_terrane_f0_s192
        / zero_terrane_f0_s57;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_receiver : f32 =
        infinity_terrane_f0_s2060; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f32.copysign(terrane_receiver) } else {
        terrane_fraction } } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        terrane_receiver : f32 = negative_infinity_terrane_f0_s3369; let terrane_fraction
        = terrane_receiver.fract(); if terrane_fraction == 0.0 { 0.0_f32
        .copysign(terrane_receiver) } else { terrane_fraction } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_infinity_terrane_f0_s3369
        .next_down().is_infinite()),
        terrane_scalar_support::scalar_text(&negative_infinity_terrane_f0_s3369
        .next_down().is_sign_negative())
    );
    large_exponent_terrane_f0_s3615 = 120.0_f32;
    large_terrane_f0_s3648 = large_exponent_terrane_f0_s3615.exp2();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f32(large_terrane_f0_s3648,
        - 250) == terrane_scalar_support::scale_binary_f32(one_terrane_f0_s109, - 130)))
    );
    descriptor_terrane_f0_s3767 = TerraneDescriptor {
        identity: "float32",
        name: "float32",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let _ =
        &descriptor_terrane_f0_s3767; f32::EPSILON } == { let _ = &TerraneDescriptor {
        identity : "float32", name : "float32", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f32::EPSILON }))
    );
}
fn exercise64() {
    let zero_terrane_f0_s3859: f64;
    let negative_zero_terrane_f0_s3880: f64;
    let one_terrane_f0_s3911: f64;
    let two_terrane_f0_s3931: f64;
    let three_terrane_f0_s3951: f64;
    let four_terrane_f0_s3973: f64;
    let eight_terrane_f0_s3994: f64;
    let negative_one_terrane_f0_s4084: f64;
    let negative_two_terrane_f0_s4114: f64;
    let negative_quarter_terrane_f0_s4144: f64;
    let fractional_terrane_f0_s4179: f64;
    let twelve_terrane_f0_s4208: f64;
    let thousand_terrane_f0_s4232: f64;
    let overflow_input_terrane_f0_s4260: f64;
    let underflow_input_terrane_f0_s4294: f64;
    let decomposition_terrane_f0_s5222: terrane_scalar_support::FloatDecomposition<f64>;
    let not_a_number_terrane_f0_s5823: f64;
    let infinity_terrane_f0_s5864: f64;
    let clamped_zero_terrane_f0_s5896: f64;
    let subnormal_decomposition_terrane_f0_s6172: terrane_scalar_support::FloatDecomposition<
        f64,
    >;
    let narrow_four_terrane_f0_s6492: f32;
    let floating_exponent_terrane_f0_s6520: f64;
    let nan_decomposition_terrane_f0_s6748: terrane_scalar_support::FloatDecomposition<
        f64,
    >;
    let infinity_decomposition_terrane_f0_s6796: terrane_scalar_support::FloatDecomposition<
        f64,
    >;
    let negative_zero_decomposition_terrane_f0_s6845: terrane_scalar_support::FloatDecomposition<
        f64,
    >;
    let negative_infinity_terrane_f0_s7177: f64;
    let large_exponent_terrane_f0_s7423: f64;
    let large_terrane_f0_s7457: f64;
    let descriptor_terrane_f0_s7578: TerraneDescriptor;
    zero_terrane_f0_s3859 = 0.0;
    negative_zero_terrane_f0_s3880 = -0.0_f64;
    one_terrane_f0_s3911 = 1.0;
    two_terrane_f0_s3931 = 2.0;
    three_terrane_f0_s3951 = 3.0;
    four_terrane_f0_s3973 = 4.0;
    eight_terrane_f0_s3994 = 8.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(eight_terrane_f0_s3994.cbrt() ==
        two_terrane_f0_s3931)),
        terrane_scalar_support::scalar_text(&(three_terrane_f0_s3951
        .hypot(four_terrane_f0_s3973) == 5.0))
    );
    negative_one_terrane_f0_s4084 = -1.0_f64;
    negative_two_terrane_f0_s4114 = -2.0_f64;
    negative_quarter_terrane_f0_s4144 = -0.25_f64;
    fractional_terrane_f0_s4179 = -1.25_f64;
    twelve_terrane_f0_s4208 = 12.0;
    thousand_terrane_f0_s4232 = 1000.0;
    overflow_input_terrane_f0_s4260 = 1024.0;
    underflow_input_terrane_f0_s4294 = -1075.0_f64;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(four_terrane_f0_s3973.powf(0.5) ==
        two_terrane_f0_s3931)),
        terrane_scalar_support::scalar_text(&(two_terrane_f0_s3931.powi(10) == 1024.0))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three_terrane_f0_s3951.exp2() ==
        eight_terrane_f0_s3994)),
        terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859.exp_m1() ==
        zero_terrane_f0_s3859))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859.ln_1p() ==
        zero_terrane_f0_s3859)),
        terrane_scalar_support::scalar_text(&(eight_terrane_f0_s3994.log2() ==
        three_terrane_f0_s3951))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(thousand_terrane_f0_s4232.log10()
        == three_terrane_f0_s3951)),
        terrane_scalar_support::scalar_text(&(eight_terrane_f0_s3994
        .log(two_terrane_f0_s3931) == three_terrane_f0_s3951))
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859.tan() ==
        zero_terrane_f0_s3859)),
        terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859.asin() ==
        zero_terrane_f0_s3859)),
        terrane_scalar_support::scalar_text(&(one_terrane_f0_s3911.acos() ==
        zero_terrane_f0_s3859))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859.atan() ==
        zero_terrane_f0_s3859)),
        terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859
        .atan2(negative_one_terrane_f0_s4084) > three_terrane_f0_s3951))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(one_terrane_f0_s3911
        .copysign(negative_zero_terrane_f0_s3880) == negative_one_terrane_f0_s4084)),
        terrane_scalar_support::scalar_text(&negative_zero_terrane_f0_s3880
        .is_sign_negative())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&({ let terrane_value : f64 =
        two_terrane_f0_s3931; let terrane_lower : f64 = 3.0; let terrane_upper : f64 =
        4.0; if terrane_value.is_nan() { terrane_value } else if terrane_lower.is_nan()
        || terrane_upper.is_nan() || terrane_lower > terrane_upper { f64::NAN } else {
        let terrane_lowered = if terrane_value == 0.0 &&terrane_lower == 0.0 { if
        terrane_value.is_sign_positive() || terrane_lower.is_sign_positive() { 0.0 } else
        { - 0.0 } } else { terrane_value.max(terrane_lower) }; if terrane_lowered == 0.0
        &&terrane_upper == 0.0 { if terrane_lowered.is_sign_negative() || terrane_upper
        .is_sign_negative() { - 0.0 } else { 0.0 } } else { terrane_lowered
        .min(terrane_upper) } } } == three_terrane_f0_s3951)),
        terrane_scalar_support::scalar_text(&({ let terrane_receiver : f64 =
        fractional_terrane_f0_s4179; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else {
        terrane_fraction } } == negative_quarter_terrane_f0_s4144))
    );
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859 == 0.0)),
        terrane_scalar_support::scalar_text(&(negative_zero_terrane_f0_s3880 == 0.0)),
        terrane_scalar_support::scalar_text(&one_terrane_f0_s3911.is_normal()),
        terrane_scalar_support::scalar_text(&{ let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::from_bits(1) } .is_subnormal())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859.next_up() == {
        let _ = &TerraneDescriptor { identity : "float64", name : "float64", kind :
        "type", inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1)
        }))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(zero_terrane_f0_s3859.next_down() ==
        negative_one_terrane_f0_s4084 * { let _ = &TerraneDescriptor { identity :
        "float64", name : "float64", kind : "type", inherently_identity_bearing : false,
        fields : &[] }; f64::from_bits(1) }))
    );
    decomposition_terrane_f0_s5222 = terrane_scalar_support::decompose_f64(
        twelve_terrane_f0_s4208,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(decomposition_terrane_f0_s5222
        .mantissa == 0.75)),
        terrane_scalar_support::scalar_text(&(decomposition_terrane_f0_s5222.exponent ==
        4))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(decomposition_terrane_f0_s5222
        .mantissa, decomposition_terrane_f0_s5222.exponent) == twelve_terrane_f0_s4208))
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
        inherently_identity_bearing : false, fields : &[] }; f64::EPSILON } >
        zero_terrane_f0_s3859)), terrane_scalar_support::scalar_text(&{ let _ =
        &TerraneDescriptor { identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::MIN_POSITIVE }
        .is_normal())
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&({ let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::from_bits(1) } >
        zero_terrane_f0_s3859)), terrane_scalar_support::scalar_text(&({ let _ =
        &TerraneDescriptor { identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::MIN } <
        zero_terrane_f0_s3859)), terrane_scalar_support::scalar_text(&({ let _ =
        &TerraneDescriptor { identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::MAX } >
        zero_terrane_f0_s3859))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&two_terrane_f0_s3931.asin()
        .is_nan()), terrane_scalar_support::scalar_text(&negative_two_terrane_f0_s4114
        .powf(0.5).is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&overflow_input_terrane_f0_s4260
        .exp2().is_infinite()),
        terrane_scalar_support::scalar_text(&(underflow_input_terrane_f0_s4294.exp2() ==
        zero_terrane_f0_s3859))
    );
    not_a_number_terrane_f0_s5823 = two_terrane_f0_s3931.asin();
    infinity_terrane_f0_s5864 = one_terrane_f0_s3911 / zero_terrane_f0_s3859;
    clamped_zero_terrane_f0_s5896 = {
        let terrane_value: f64 = negative_zero_terrane_f0_s3880;
        let terrane_lower: f64 = zero_terrane_f0_s3859;
        let terrane_upper: f64 = one_terrane_f0_s3911;
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
    println!(
        "{}", terrane_scalar_support::scalar_text(&(one_terrane_f0_s3911 /
        clamped_zero_terrane_f0_s5896 > zero_terrane_f0_s3859))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f64 =
        not_a_number_terrane_f0_s5823; let terrane_lower : f64 = zero_terrane_f0_s3859;
        let terrane_upper : f64 = one_terrane_f0_s3911; if terrane_value.is_nan() {
        terrane_value } else if terrane_lower.is_nan() || terrane_upper.is_nan() ||
        terrane_lower > terrane_upper { f64::NAN } else { let terrane_lowered = if
        terrane_value == 0.0 &&terrane_lower == 0.0 { if terrane_value.is_sign_positive()
        || terrane_lower.is_sign_positive() { 0.0 } else { - 0.0 } } else { terrane_value
        .max(terrane_lower) }; if terrane_lowered == 0.0 &&terrane_upper == 0.0 { if
        terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 }
        else { 0.0 } } else { terrane_lowered.min(terrane_upper) } } } .is_nan()),
        terrane_scalar_support::scalar_text(&{ let terrane_value : f64 =
        one_terrane_f0_s3911; let terrane_lower : f64 = not_a_number_terrane_f0_s5823;
        let terrane_upper : f64 = one_terrane_f0_s3911; if terrane_value.is_nan() {
        terrane_value } else if terrane_lower.is_nan() || terrane_upper.is_nan() ||
        terrane_lower > terrane_upper { f64::NAN } else { let terrane_lowered = if
        terrane_value == 0.0 &&terrane_lower == 0.0 { if terrane_value.is_sign_positive()
        || terrane_lower.is_sign_positive() { 0.0 } else { - 0.0 } } else { terrane_value
        .max(terrane_lower) }; if terrane_lowered == 0.0 &&terrane_upper == 0.0 { if
        terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 }
        else { 0.0 } } else { terrane_lowered.min(terrane_upper) } } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&not_a_number_terrane_f0_s5823
        .hypot(infinity_terrane_f0_s5864).is_infinite()),
        terrane_scalar_support::scalar_text(&infinity_terrane_f0_s5864.next_up()
        .is_infinite())
    );
    subnormal_decomposition_terrane_f0_s6172 = terrane_scalar_support::decompose_f64({
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
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(subnormal_decomposition_terrane_f0_s6172
        .mantissa, subnormal_decomposition_terrane_f0_s6172.exponent) == { let _ =
        &TerraneDescriptor { identity : "float64", name : "float64", kind : "type",
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
        - 1) == zero_terrane_f0_s3859))
    );
    narrow_four_terrane_f0_s6492 = 4.0_f32;
    floating_exponent_terrane_f0_s6520 = 2.0;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&(three_terrane_f0_s3951
        .hypot(narrow_four_terrane_f0_s6492 as f64) == 5.0)),
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(one_terrane_f0_s3911,
        __terrane_raised(terrane_int_support::exact_from_f64:: < i32 >
        (floating_exponent_terrane_f0_s6520), 2 /* terrane-site: case.trn:115:69-115:86 */)) == four_terrane_f0_s3973))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let terrane_receiver : f64 =
        negative_one_terrane_f0_s4084; let terrane_fraction = terrane_receiver.fract();
        if terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else {
        terrane_fraction } } .is_sign_negative())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let terrane_value : f64 =
        one_terrane_f0_s3911; let terrane_lower : f64 = two_terrane_f0_s3931; let
        terrane_upper : f64 = one_terrane_f0_s3911; if terrane_value.is_nan() {
        terrane_value } else if terrane_lower.is_nan() || terrane_upper.is_nan() ||
        terrane_lower > terrane_upper { f64::NAN } else { let terrane_lowered = if
        terrane_value == 0.0 &&terrane_lower == 0.0 { if terrane_value.is_sign_positive()
        || terrane_lower.is_sign_positive() { 0.0 } else { - 0.0 } } else { terrane_value
        .max(terrane_lower) }; if terrane_lowered == 0.0 &&terrane_upper == 0.0 { if
        terrane_lowered.is_sign_negative() || terrane_upper.is_sign_negative() { - 0.0 }
        else { 0.0 } } else { terrane_lowered.min(terrane_upper) } } } .is_nan())
    );
    nan_decomposition_terrane_f0_s6748 = terrane_scalar_support::decompose_f64(
        not_a_number_terrane_f0_s5823,
    );
    infinity_decomposition_terrane_f0_s6796 = terrane_scalar_support::decompose_f64(
        infinity_terrane_f0_s5864,
    );
    negative_zero_decomposition_terrane_f0_s6845 = terrane_scalar_support::decompose_f64(
        negative_zero_terrane_f0_s3880,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&nan_decomposition_terrane_f0_s6748
        .mantissa.is_nan()),
        terrane_scalar_support::scalar_text(&(nan_decomposition_terrane_f0_s6748.exponent
        == 0))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&infinity_decomposition_terrane_f0_s6796
        .mantissa.is_infinite()),
        terrane_scalar_support::scalar_text(&(infinity_decomposition_terrane_f0_s6796
        .exponent == 0))
    );
    println!(
        "{}{}",
        terrane_scalar_support::scalar_text(&negative_zero_decomposition_terrane_f0_s6845
        .mantissa.is_sign_negative()),
        terrane_scalar_support::scalar_text(&(negative_zero_decomposition_terrane_f0_s6845
        .exponent == 0))
    );
    negative_infinity_terrane_f0_s7177 = negative_one_terrane_f0_s4084
        / zero_terrane_f0_s3859;
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&{ let terrane_receiver : f64 =
        infinity_terrane_f0_s5864; let terrane_fraction = terrane_receiver.fract(); if
        terrane_fraction == 0.0 { 0.0_f64.copysign(terrane_receiver) } else {
        terrane_fraction } } .is_nan()), terrane_scalar_support::scalar_text(&{ let
        terrane_receiver : f64 = negative_infinity_terrane_f0_s7177; let terrane_fraction
        = terrane_receiver.fract(); if terrane_fraction == 0.0 { 0.0_f64
        .copysign(terrane_receiver) } else { terrane_fraction } } .is_nan())
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&negative_infinity_terrane_f0_s7177
        .next_down().is_infinite()),
        terrane_scalar_support::scalar_text(&negative_infinity_terrane_f0_s7177
        .next_down().is_sign_negative())
    );
    large_exponent_terrane_f0_s7423 = 1000.0;
    large_terrane_f0_s7457 = large_exponent_terrane_f0_s7423.exp2();
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_scalar_support::scale_binary_f64(large_terrane_f0_s7457,
        - 2000) == terrane_scalar_support::scale_binary_f64(one_terrane_f0_s3911, -
        1000)))
    );
    descriptor_terrane_f0_s7578 = TerraneDescriptor {
        identity: "float64",
        name: "float64",
        kind: "type",
        inherently_identity_bearing: false,
        fields: &[],
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&({ let _ =
        &descriptor_terrane_f0_s7578; f64::EPSILON } == { let _ = &TerraneDescriptor {
        identity : "float64", name : "float64", kind : "type",
        inherently_identity_bearing : false, fields : &[] }; f64::EPSILON }))
    );
}
fn main() {
    exercise32();
    exercise64();
}
