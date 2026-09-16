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
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 10] = [
        "/guarded-checked-arithmetic::transition",
        "/guarded-checked-arithmetic::transition-uint8",
        "/guarded-checked-arithmetic::transition-then",
        "/guarded-checked-arithmetic::transition-remainder",
        "/guarded-checked-arithmetic::transition-negative-coefficient",
        "/guarded-checked-arithmetic::transition-int64",
        "/guarded-checked-arithmetic::transition-int128",
        "/guarded-checked-arithmetic::both-affine",
        "/guarded-checked-arithmetic::nested-conditional",
        "/guarded-checked-arithmetic::main",
    ];
    pub static SITES: [Site; 35] = [
        /* terrane-site-row: site 0: /guarded-checked-arithmetic::transition (case.trn:8:13-8:22) */
        { Site { function: 0, file: 0, line: 8, column: 13, end_line: 8, end_column: 22 } },
        /* terrane-site-row: site 1: /guarded-checked-arithmetic::transition (case.trn:10:13-10:22) */
        { Site { function: 0, file: 0, line: 10, column: 13, end_line: 10, end_column: 22 } },
        /* terrane-site-row: site 2: /guarded-checked-arithmetic::transition (case.trn:10:13-10:26) */
        { Site { function: 0, file: 0, line: 10, column: 13, end_line: 10, end_column: 26 } },
        /* terrane-site-row: site 3: /guarded-checked-arithmetic::transition-uint8 (case.trn:16:13-16:22) */
        { Site { function: 1, file: 0, line: 16, column: 13, end_line: 16, end_column: 22 } },
        /* terrane-site-row: site 4: /guarded-checked-arithmetic::transition-uint8 (case.trn:18:13-18:22) */
        { Site { function: 1, file: 0, line: 18, column: 13, end_line: 18, end_column: 22 } },
        /* terrane-site-row: site 5: /guarded-checked-arithmetic::transition-uint8 (case.trn:18:13-18:26) */
        { Site { function: 1, file: 0, line: 18, column: 13, end_line: 18, end_column: 26 } },
        /* terrane-site-row: site 6: /guarded-checked-arithmetic::transition-then (case.trn:24:13-24:22) */
        { Site { function: 2, file: 0, line: 24, column: 13, end_line: 24, end_column: 22 } },
        /* terrane-site-row: site 7: /guarded-checked-arithmetic::transition-then (case.trn:24:13-24:26) */
        { Site { function: 2, file: 0, line: 24, column: 13, end_line: 24, end_column: 26 } },
        /* terrane-site-row: site 8: /guarded-checked-arithmetic::transition-then (case.trn:26:13-26:22) */
        { Site { function: 2, file: 0, line: 26, column: 13, end_line: 26, end_column: 22 } },
        /* terrane-site-row: site 9: /guarded-checked-arithmetic::transition-remainder (case.trn:34:13-34:22) */
        { Site { function: 3, file: 0, line: 34, column: 13, end_line: 34, end_column: 22 } },
        /* terrane-site-row: site 10: /guarded-checked-arithmetic::transition-remainder (case.trn:34:13-34:26) */
        { Site { function: 3, file: 0, line: 34, column: 13, end_line: 34, end_column: 26 } },
        /* terrane-site-row: site 11: /guarded-checked-arithmetic::transition-negative-coefficient (case.trn:40:13-40:22) */
        { Site { function: 4, file: 0, line: 40, column: 13, end_line: 40, end_column: 22 } },
        /* terrane-site-row: site 12: /guarded-checked-arithmetic::transition-negative-coefficient (case.trn:42:13-42:24) */
        { Site { function: 4, file: 0, line: 42, column: 13, end_line: 42, end_column: 24 } },
        /* terrane-site-row: site 13: /guarded-checked-arithmetic::transition-int64 (case.trn:48:13-48:22) */
        { Site { function: 5, file: 0, line: 48, column: 13, end_line: 48, end_column: 22 } },
        /* terrane-site-row: site 14: /guarded-checked-arithmetic::transition-int64 (case.trn:50:13-50:22) */
        { Site { function: 5, file: 0, line: 50, column: 13, end_line: 50, end_column: 22 } },
        /* terrane-site-row: site 15: /guarded-checked-arithmetic::transition-int64 (case.trn:50:13-50:26) */
        { Site { function: 5, file: 0, line: 50, column: 13, end_line: 50, end_column: 26 } },
        /* terrane-site-row: site 16: /guarded-checked-arithmetic::transition-int128 (case.trn:56:13-56:22) */
        { Site { function: 6, file: 0, line: 56, column: 13, end_line: 56, end_column: 22 } },
        /* terrane-site-row: site 17: /guarded-checked-arithmetic::transition-int128 (case.trn:58:13-58:22) */
        { Site { function: 6, file: 0, line: 58, column: 13, end_line: 58, end_column: 22 } },
        /* terrane-site-row: site 18: /guarded-checked-arithmetic::transition-int128 (case.trn:58:13-58:26) */
        { Site { function: 6, file: 0, line: 58, column: 13, end_line: 58, end_column: 26 } },
        /* terrane-site-row: site 19: /guarded-checked-arithmetic::both-affine (case.trn:64:13-64:22) */
        { Site { function: 7, file: 0, line: 64, column: 13, end_line: 64, end_column: 22 } },
        /* terrane-site-row: site 20: /guarded-checked-arithmetic::both-affine (case.trn:66:13-66:22) */
        { Site { function: 7, file: 0, line: 66, column: 13, end_line: 66, end_column: 22 } },
        /* terrane-site-row: site 21: /guarded-checked-arithmetic::both-affine (case.trn:66:13-66:26) */
        { Site { function: 7, file: 0, line: 66, column: 13, end_line: 66, end_column: 26 } },
        /* terrane-site-row: site 22: /guarded-checked-arithmetic::nested-conditional (case.trn:72:13-72:22) */
        { Site { function: 8, file: 0, line: 72, column: 13, end_line: 72, end_column: 22 } },
        /* terrane-site-row: site 23: /guarded-checked-arithmetic::nested-conditional (case.trn:74:13-74:22) */
        { Site { function: 8, file: 0, line: 74, column: 13, end_line: 74, end_column: 22 } },
        /* terrane-site-row: site 24: /guarded-checked-arithmetic::nested-conditional (case.trn:76:13-76:22) */
        { Site { function: 8, file: 0, line: 76, column: 13, end_line: 76, end_column: 22 } },
        /* terrane-site-row: site 25: /guarded-checked-arithmetic::nested-conditional (case.trn:76:13-76:26) */
        { Site { function: 8, file: 0, line: 76, column: 13, end_line: 76, end_column: 26 } },
        /* terrane-site-row: site 26: /guarded-checked-arithmetic::main (case.trn:85:15-85:24) */
        { Site { function: 9, file: 0, line: 85, column: 15, end_line: 85, end_column: 24 } },
        /* terrane-site-row: site 27: /guarded-checked-arithmetic::main (case.trn:87:15-87:24) */
        { Site { function: 9, file: 0, line: 87, column: 15, end_line: 87, end_column: 24 } },
        /* terrane-site-row: site 28: /guarded-checked-arithmetic::main (case.trn:87:15-87:28) */
        { Site { function: 9, file: 0, line: 87, column: 15, end_line: 87, end_column: 28 } },
        /* terrane-site-row: site 29: /guarded-checked-arithmetic::main (case.trn:94:22-94:38) */
        { Site { function: 9, file: 0, line: 94, column: 22, end_line: 94, end_column: 38 } },
        /* terrane-site-row: site 30: /guarded-checked-arithmetic::main (case.trn:96:22-96:38) */
        { Site { function: 9, file: 0, line: 96, column: 22, end_line: 96, end_column: 38 } },
        /* terrane-site-row: site 31: /guarded-checked-arithmetic::main (case.trn:96:22-96:42) */
        { Site { function: 9, file: 0, line: 96, column: 22, end_line: 96, end_column: 42 } },
        /* terrane-site-row: site 32: /guarded-checked-arithmetic::main (case.trn:112:20-112:36) */
        { Site { function: 9, file: 0, line: 112, column: 20, end_line: 112, end_column: 36 } },
        /* terrane-site-row: site 33: /guarded-checked-arithmetic::main (case.trn:114:20-114:36) */
        { Site { function: 9, file: 0, line: 114, column: 20, end_line: 114, end_column: 36 } },
        /* terrane-site-row: site 34: /guarded-checked-arithmetic::main (case.trn:114:20-114:40) */
        { Site { function: 9, file: 0, line: 114, column: 20, end_line: 114, end_column: 40 } },
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
// Namespace: guarded-checked-arithmetic
fn transition(input: i8) -> i8 {
    let mut value: i8 = input;
    value = if value as u8 <= 42_u8 {
        let __terrane_guarded_then = value / 2_i8;
        let __terrane_guarded_else = 3_i8 * value as i8 + 1_i8;
        let __terrane_guarded_mask = 0_i8.wrapping_sub((value.rem_euclid(2) == 0) as i8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            0 /* terrane-site: case.trn:8:13-8:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    1 /* terrane-site: case.trn:10:13-10:22 */,
                ),
                1,
            ),
            2 /* terrane-site: case.trn:10:13-10:26 */,
        )
    };
    return value;
}
fn transition_uint8(input: u8) -> u8 {
    let mut value: u8 = input;
    value = if value <= 84_u8 {
        let __terrane_guarded_then = value / 2_u8;
        let __terrane_guarded_else = 3_u8 * value as u8 + 1_u8;
        let __terrane_guarded_mask = 0_u8.wrapping_sub((value.rem_euclid(2) == 0) as u8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            3 /* terrane-site: case.trn:16:13-16:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    4 /* terrane-site: case.trn:18:13-18:22 */,
                ),
                1,
            ),
            5 /* terrane-site: case.trn:18:13-18:26 */,
        )
    };
    return value;
}
fn transition_then(input: i8) -> i8 {
    let mut value: i8 = input;
    value = if value as u8 <= 42_u8 {
        let __terrane_guarded_then = 3_i8 * value as i8 + 1_i8;
        let __terrane_guarded_else = value / 2_i8;
        let __terrane_guarded_mask = 0_i8.wrapping_sub((value.rem_euclid(2) != 0) as i8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) != 0 {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    6 /* terrane-site: case.trn:24:13-24:22 */,
                ),
                1,
            ),
            7 /* terrane-site: case.trn:24:13-24:26 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            8 /* terrane-site: case.trn:26:13-26:22 */,
        )
    };
    return value;
}
fn transition_remainder(input: i8) -> i8 {
    let mut value: i8 = input;
    value = if value as u8 <= 42_u8 {
        let __terrane_guarded_then = value % 5_i8;
        let __terrane_guarded_else = 3_i8 * value as i8 + 1_i8;
        let __terrane_guarded_mask = 0_i8.wrapping_sub((value.rem_euclid(2) == 0) as i8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        value.rem_euclid(5)
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    9 /* terrane-site: case.trn:34:13-34:22 */,
                ),
                1,
            ),
            10 /* terrane-site: case.trn:34:13-34:26 */,
        )
    };
    return value;
}
fn transition_negative_coefficient(input: i8) -> i8 {
    let mut value: i8 = input;
    value = if value as u8 <= 127_u8 {
        let __terrane_guarded_then = value / 2_i8;
        let __terrane_guarded_else = 100_i8 - value as i8;
        let __terrane_guarded_mask = 0_i8.wrapping_sub((value.rem_euclid(2) == 0) as i8);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            11 /* terrane-site: case.trn:40:13-40:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_subtraction(100, value),
            12 /* terrane-site: case.trn:42:13-42:24 */,
        )
    };
    return value;
}
fn transition_int64(input: i64) -> i64 {
    let mut value: i64 = input;
    value = if value as u64 <= 3074457345618258602_u64 {
        let __terrane_guarded_then = value / 2_i64;
        let __terrane_guarded_else = 3_i64 * value as i64 + 1_i64;
        let __terrane_guarded_mask = 0_i64
            .wrapping_sub((value.rem_euclid(2) == 0) as i64);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            13 /* terrane-site: case.trn:48:13-48:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    14 /* terrane-site: case.trn:50:13-50:22 */,
                ),
                1,
            ),
            15 /* terrane-site: case.trn:50:13-50:26 */,
        )
    };
    return value;
}
fn transition_int128(input: i128) -> i128 {
    let mut value: i128 = input;
    value = if value as u128 <= 56713727820156410577229101238628035242_u128 {
        let __terrane_guarded_then = value / 2_i128;
        let __terrane_guarded_else = 3_i128 * value as i128 + 1_i128;
        let __terrane_guarded_mask = 0_i128
            .wrapping_sub((value.rem_euclid(2) == 0) as i128);
        __terrane_guarded_else
            ^ (__terrane_guarded_then ^ __terrane_guarded_else) & __terrane_guarded_mask
    } else if value.rem_euclid(2) == 0 {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            16 /* terrane-site: case.trn:56:13-56:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    17 /* terrane-site: case.trn:58:13-58:22 */,
                ),
                1,
            ),
            18 /* terrane-site: case.trn:58:13-58:26 */,
        )
    };
    return value;
}
fn both_affine(input: i8) -> i8 {
    let mut value: i8 = input;
    if value > 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(value, 1),
            19 /* terrane-site: case.trn:64:13-64:22 */,
        );
    } else {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    20 /* terrane-site: case.trn:66:13-66:22 */,
                ),
                1,
            ),
            21 /* terrane-site: case.trn:66:13-66:26 */,
        );
    }
    return value;
}
fn nested_conditional(input: i8) -> i8 {
    let mut value: i8 = input;
    if value < 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            22 /* terrane-site: case.trn:72:13-72:22 */,
        );
    } else if value.rem_euclid(2) == 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            23 /* terrane-site: case.trn:74:13-74:22 */,
        );
    } else {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    24 /* terrane-site: case.trn:76:13-76:22 */,
                ),
                1,
            ),
            25 /* terrane-site: case.trn:76:13-76:26 */,
        );
    }
    return value;
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&transition(42)));
    println!("{}", terrane_scalar_support::scalar_text(&transition(41)));
    let mut upper: i8 = 43;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            upper = if upper as u8 <= 42_u8 {
                let __terrane_guarded_then = upper / 2_i8;
                let __terrane_guarded_else = 3_i8 * upper as i8 + 1_i8;
                let __terrane_guarded_mask = 0_i8
                    .wrapping_sub((upper.rem_euclid(2) == 0) as i8);
                __terrane_guarded_else
                    ^ (__terrane_guarded_then ^ __terrane_guarded_else)
                        & __terrane_guarded_mask
            } else if upper.rem_euclid(2) == 0 {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_division(upper, 2), 26 /* terrane-site: case.trn:85:15-85:24 */
                )
            } else {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_addition(__terrane_raised_completion!(terrane_int_support::fixed_multiplication(3,
                    upper), 27 /* terrane-site: case.trn:87:15-87:24 */), 1),
                    28 /* terrane-site: case.trn:87:15-87:28 */
                )
            };
            println!("{}", terrane_scalar_support::scalar_text(&upper));
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
                        terrane_scalar_support::scalar_text(&String::from("upper-caught"))
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
    let mut intermediate: i8 = -43;
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            intermediate = if intermediate as u8 <= 42_u8 {
                let __terrane_guarded_then = intermediate / 2_i8;
                let __terrane_guarded_else = 3_i8 * intermediate as i8 + 1_i8;
                let __terrane_guarded_mask = 0_i8
                    .wrapping_sub((intermediate.rem_euclid(2) == 0) as i8);
                __terrane_guarded_else
                    ^ (__terrane_guarded_then ^ __terrane_guarded_else)
                        & __terrane_guarded_mask
            } else if intermediate.rem_euclid(2) == 0 {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_division(intermediate, 2),
                    29 /* terrane-site: case.trn:94:22-94:38 */
                )
            } else {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_addition(__terrane_raised_completion!(terrane_int_support::fixed_multiplication(3,
                    intermediate), 30 /* terrane-site: case.trn:96:22-96:38 */), 1),
                    31 /* terrane-site: case.trn:96:22-96:42 */
                )
            };
            println!("{}", terrane_scalar_support::scalar_text(&intermediate));
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
                    && __terrane_error_1.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_1 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("intermediate-caught"))
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
    println!("{}", terrane_scalar_support::scalar_text(&transition_uint8(41)));
    println!("{}", terrane_scalar_support::scalar_text(&transition_then(41)));
    println!("{}", terrane_scalar_support::scalar_text(&transition_remainder(42)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&transition_negative_coefficient(41))
    );
    println!("{}", terrane_scalar_support::scalar_text(&transition_int64(41)));
    println!("{}", terrane_scalar_support::scalar_text(&transition_int128(41)));
    println!("{}", terrane_scalar_support::scalar_text(&both_affine(41)));
    println!("{}", terrane_scalar_support::scalar_text(&nested_conditional(41)));
    let shared_value: std::sync::Arc<std::sync::Mutex<i8>> = std::sync::Arc::new(
        std::sync::Mutex::new(41),
    );
    let observer: std::sync::Arc<std::sync::Mutex<i8>> = shared_value.clone();
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value = observer
        .lock().expect("shared reference lock poisoned").clone(); __terrane_value })
    );
    if {
        let __terrane_value = shared_value
            .lock()
            .expect("reference lock poisoned")
            .clone();
        __terrane_value
    }
        .rem_euclid(2) == 0
    {
        *shared_value.lock().expect("reference lock poisoned") = __terrane_raised(
            terrane_int_support::fixed_division(
                {
                    let __terrane_value = shared_value
                        .lock()
                        .expect("reference lock poisoned")
                        .clone();
                    __terrane_value
                },
                2,
            ),
            32 /* terrane-site: case.trn:112:20-112:36 */,
        );
    } else {
        *shared_value.lock().expect("reference lock poisoned") = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(
                        3,
                        {
                            let __terrane_value = shared_value
                                .lock()
                                .expect("reference lock poisoned")
                                .clone();
                            __terrane_value
                        },
                    ),
                    33 /* terrane-site: case.trn:114:20-114:36 */,
                ),
                1,
            ),
            34 /* terrane-site: case.trn:114:20-114:40 */,
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value = shared_value
        .lock().expect("reference lock poisoned").clone(); __terrane_value })
    );
}
