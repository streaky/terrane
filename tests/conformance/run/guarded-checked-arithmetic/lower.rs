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
    pub static FUNCTIONS: [&str; 12] = [
        "/guarded-checked-arithmetic::transition",
        "/guarded-checked-arithmetic::transition-uint8",
        "/guarded-checked-arithmetic::transition-then",
        "/guarded-checked-arithmetic::transition-remainder",
        "/guarded-checked-arithmetic::transition-negative-coefficient",
        "/guarded-checked-arithmetic::transition-int64",
        "/guarded-checked-arithmetic::transition-int128",
        "/guarded-checked-arithmetic::both-affine",
        "/guarded-checked-arithmetic::nested-conditional",
        "/guarded-checked-arithmetic::positive-lower-bound",
        "/guarded-checked-arithmetic::global-condition",
        "/guarded-checked-arithmetic::main",
    ];
    pub static SITES: [Site; 41] = [
        /* terrane-site-row: site 0: /guarded-checked-arithmetic::transition (case.trn:10:13-10:22) */
        { Site { function: 0, file: 0, line: 10, column: 13, end_line: 10, end_column: 22 } },
        /* terrane-site-row: site 1: /guarded-checked-arithmetic::transition (case.trn:12:13-12:22) */
        { Site { function: 0, file: 0, line: 12, column: 13, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 2: /guarded-checked-arithmetic::transition (case.trn:12:13-12:26) */
        { Site { function: 0, file: 0, line: 12, column: 13, end_line: 12, end_column: 26 } },
        /* terrane-site-row: site 3: /guarded-checked-arithmetic::transition-uint8 (case.trn:18:13-18:22) */
        { Site { function: 1, file: 0, line: 18, column: 13, end_line: 18, end_column: 22 } },
        /* terrane-site-row: site 4: /guarded-checked-arithmetic::transition-uint8 (case.trn:20:13-20:22) */
        { Site { function: 1, file: 0, line: 20, column: 13, end_line: 20, end_column: 22 } },
        /* terrane-site-row: site 5: /guarded-checked-arithmetic::transition-uint8 (case.trn:20:13-20:26) */
        { Site { function: 1, file: 0, line: 20, column: 13, end_line: 20, end_column: 26 } },
        /* terrane-site-row: site 6: /guarded-checked-arithmetic::transition-then (case.trn:26:13-26:22) */
        { Site { function: 2, file: 0, line: 26, column: 13, end_line: 26, end_column: 22 } },
        /* terrane-site-row: site 7: /guarded-checked-arithmetic::transition-then (case.trn:26:13-26:26) */
        { Site { function: 2, file: 0, line: 26, column: 13, end_line: 26, end_column: 26 } },
        /* terrane-site-row: site 8: /guarded-checked-arithmetic::transition-then (case.trn:28:13-28:22) */
        { Site { function: 2, file: 0, line: 28, column: 13, end_line: 28, end_column: 22 } },
        /* terrane-site-row: site 9: /guarded-checked-arithmetic::transition-remainder (case.trn:36:13-36:22) */
        { Site { function: 3, file: 0, line: 36, column: 13, end_line: 36, end_column: 22 } },
        /* terrane-site-row: site 10: /guarded-checked-arithmetic::transition-remainder (case.trn:36:13-36:26) */
        { Site { function: 3, file: 0, line: 36, column: 13, end_line: 36, end_column: 26 } },
        /* terrane-site-row: site 11: /guarded-checked-arithmetic::transition-negative-coefficient (case.trn:42:13-42:22) */
        { Site { function: 4, file: 0, line: 42, column: 13, end_line: 42, end_column: 22 } },
        /* terrane-site-row: site 12: /guarded-checked-arithmetic::transition-negative-coefficient (case.trn:44:13-44:24) */
        { Site { function: 4, file: 0, line: 44, column: 13, end_line: 44, end_column: 24 } },
        /* terrane-site-row: site 13: /guarded-checked-arithmetic::transition-int64 (case.trn:50:13-50:22) */
        { Site { function: 5, file: 0, line: 50, column: 13, end_line: 50, end_column: 22 } },
        /* terrane-site-row: site 14: /guarded-checked-arithmetic::transition-int64 (case.trn:52:13-52:22) */
        { Site { function: 5, file: 0, line: 52, column: 13, end_line: 52, end_column: 22 } },
        /* terrane-site-row: site 15: /guarded-checked-arithmetic::transition-int64 (case.trn:52:13-52:26) */
        { Site { function: 5, file: 0, line: 52, column: 13, end_line: 52, end_column: 26 } },
        /* terrane-site-row: site 16: /guarded-checked-arithmetic::transition-int128 (case.trn:58:13-58:22) */
        { Site { function: 6, file: 0, line: 58, column: 13, end_line: 58, end_column: 22 } },
        /* terrane-site-row: site 17: /guarded-checked-arithmetic::transition-int128 (case.trn:60:13-60:22) */
        { Site { function: 6, file: 0, line: 60, column: 13, end_line: 60, end_column: 22 } },
        /* terrane-site-row: site 18: /guarded-checked-arithmetic::transition-int128 (case.trn:60:13-60:26) */
        { Site { function: 6, file: 0, line: 60, column: 13, end_line: 60, end_column: 26 } },
        /* terrane-site-row: site 19: /guarded-checked-arithmetic::both-affine (case.trn:66:13-66:22) */
        { Site { function: 7, file: 0, line: 66, column: 13, end_line: 66, end_column: 22 } },
        /* terrane-site-row: site 20: /guarded-checked-arithmetic::both-affine (case.trn:68:13-68:22) */
        { Site { function: 7, file: 0, line: 68, column: 13, end_line: 68, end_column: 22 } },
        /* terrane-site-row: site 21: /guarded-checked-arithmetic::both-affine (case.trn:68:13-68:26) */
        { Site { function: 7, file: 0, line: 68, column: 13, end_line: 68, end_column: 26 } },
        /* terrane-site-row: site 22: /guarded-checked-arithmetic::nested-conditional (case.trn:74:13-74:22) */
        { Site { function: 8, file: 0, line: 74, column: 13, end_line: 74, end_column: 22 } },
        /* terrane-site-row: site 23: /guarded-checked-arithmetic::nested-conditional (case.trn:76:13-76:22) */
        { Site { function: 8, file: 0, line: 76, column: 13, end_line: 76, end_column: 22 } },
        /* terrane-site-row: site 24: /guarded-checked-arithmetic::nested-conditional (case.trn:78:13-78:22) */
        { Site { function: 8, file: 0, line: 78, column: 13, end_line: 78, end_column: 22 } },
        /* terrane-site-row: site 25: /guarded-checked-arithmetic::nested-conditional (case.trn:78:13-78:26) */
        { Site { function: 8, file: 0, line: 78, column: 13, end_line: 78, end_column: 26 } },
        /* terrane-site-row: site 26: /guarded-checked-arithmetic::positive-lower-bound (case.trn:85:15-85:26) */
        { Site { function: 9, file: 0, line: 85, column: 15, end_line: 85, end_column: 26 } },
        /* terrane-site-row: site 27: /guarded-checked-arithmetic::positive-lower-bound (case.trn:85:15-85:32) */
        { Site { function: 9, file: 0, line: 85, column: 15, end_line: 85, end_column: 32 } },
        /* terrane-site-row: site 28: /guarded-checked-arithmetic::positive-lower-bound (case.trn:87:15-87:24) */
        { Site { function: 9, file: 0, line: 87, column: 15, end_line: 87, end_column: 24 } },
        /* terrane-site-row: site 29: /guarded-checked-arithmetic::global-condition (case.trn:95:13-95:22) */
        { Site { function: 10, file: 0, line: 95, column: 13, end_line: 95, end_column: 22 } },
        /* terrane-site-row: site 30: /guarded-checked-arithmetic::global-condition (case.trn:97:13-97:22) */
        { Site { function: 10, file: 0, line: 97, column: 13, end_line: 97, end_column: 22 } },
        /* terrane-site-row: site 31: /guarded-checked-arithmetic::global-condition (case.trn:97:13-97:26) */
        { Site { function: 10, file: 0, line: 97, column: 13, end_line: 97, end_column: 26 } },
        /* terrane-site-row: site 32: /guarded-checked-arithmetic::main (case.trn:109:15-109:24) */
        { Site { function: 11, file: 0, line: 109, column: 15, end_line: 109, end_column: 24 } },
        /* terrane-site-row: site 33: /guarded-checked-arithmetic::main (case.trn:111:15-111:24) */
        { Site { function: 11, file: 0, line: 111, column: 15, end_line: 111, end_column: 24 } },
        /* terrane-site-row: site 34: /guarded-checked-arithmetic::main (case.trn:111:15-111:28) */
        { Site { function: 11, file: 0, line: 111, column: 15, end_line: 111, end_column: 28 } },
        /* terrane-site-row: site 35: /guarded-checked-arithmetic::main (case.trn:118:22-118:38) */
        { Site { function: 11, file: 0, line: 118, column: 22, end_line: 118, end_column: 38 } },
        /* terrane-site-row: site 36: /guarded-checked-arithmetic::main (case.trn:120:22-120:38) */
        { Site { function: 11, file: 0, line: 120, column: 22, end_line: 120, end_column: 38 } },
        /* terrane-site-row: site 37: /guarded-checked-arithmetic::main (case.trn:120:22-120:42) */
        { Site { function: 11, file: 0, line: 120, column: 22, end_line: 120, end_column: 42 } },
        /* terrane-site-row: site 38: /guarded-checked-arithmetic::main (case.trn:136:20-136:36) */
        { Site { function: 11, file: 0, line: 136, column: 20, end_line: 136, end_column: 36 } },
        /* terrane-site-row: site 39: /guarded-checked-arithmetic::main (case.trn:138:20-138:36) */
        { Site { function: 11, file: 0, line: 138, column: 20, end_line: 138, end_column: 36 } },
        /* terrane-site-row: site 40: /guarded-checked-arithmetic::main (case.trn:138:20-138:40) */
        { Site { function: 11, file: 0, line: 138, column: 20, end_line: 138, end_column: 40 } },
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
static __TERRANE_GLOBAL_PARITY: std::sync::LazyLock<std::sync::Mutex<Option<i8>>> = std::sync::LazyLock::new(||
std::sync::Mutex::new(Some(0)));
fn __terrane_uninitialized_global(
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
            0 /* terrane-site: case.trn:10:13-10:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    1 /* terrane-site: case.trn:12:13-12:22 */,
                ),
                1,
            ),
            2 /* terrane-site: case.trn:12:13-12:26 */,
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
            3 /* terrane-site: case.trn:18:13-18:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    4 /* terrane-site: case.trn:20:13-20:22 */,
                ),
                1,
            ),
            5 /* terrane-site: case.trn:20:13-20:26 */,
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
                    6 /* terrane-site: case.trn:26:13-26:22 */,
                ),
                1,
            ),
            7 /* terrane-site: case.trn:26:13-26:26 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            8 /* terrane-site: case.trn:28:13-28:22 */,
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
                    9 /* terrane-site: case.trn:36:13-36:22 */,
                ),
                1,
            ),
            10 /* terrane-site: case.trn:36:13-36:26 */,
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
            11 /* terrane-site: case.trn:42:13-42:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_subtraction(100, value),
            12 /* terrane-site: case.trn:44:13-44:24 */,
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
            13 /* terrane-site: case.trn:50:13-50:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    14 /* terrane-site: case.trn:52:13-52:22 */,
                ),
                1,
            ),
            15 /* terrane-site: case.trn:52:13-52:26 */,
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
            16 /* terrane-site: case.trn:58:13-58:22 */,
        )
    } else {
        __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    17 /* terrane-site: case.trn:60:13-60:22 */,
                ),
                1,
            ),
            18 /* terrane-site: case.trn:60:13-60:26 */,
        )
    };
    return value;
}
fn both_affine(input: i8) -> i8 {
    let mut value: i8 = input;
    if value > 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(value, 1),
            19 /* terrane-site: case.trn:66:13-66:22 */,
        );
    } else {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    20 /* terrane-site: case.trn:68:13-68:22 */,
                ),
                1,
            ),
            21 /* terrane-site: case.trn:68:13-68:26 */,
        );
    }
    return value;
}
fn nested_conditional(input: i8) -> i8 {
    let mut value: i8 = input;
    if value < 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            22 /* terrane-site: case.trn:74:13-74:22 */,
        );
    } else if value.rem_euclid(2) == 0 {
        value = __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            23 /* terrane-site: case.trn:76:13-76:22 */,
        );
    } else {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    24 /* terrane-site: case.trn:78:13-78:22 */,
                ),
                1,
            ),
            25 /* terrane-site: case.trn:78:13-78:26 */,
        );
    }
    return value;
}
fn positive_lower_bound(input: i8) {
    let mut value: i8 = input;
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            value = if value >= 72_i8 {
                let __terrane_guarded_then = value as i8 - 100_i8 - 100_i8;
                let __terrane_guarded_else = value / 2_i8;
                let __terrane_guarded_mask = 0_i8
                    .wrapping_sub((value.rem_euclid(2) == 0) as i8);
                __terrane_guarded_else
                    ^ (__terrane_guarded_then ^ __terrane_guarded_else)
                        & __terrane_guarded_mask
            } else if value.rem_euclid(2) == 0 {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_subtraction(__terrane_raised_completion!(terrane_int_support::fixed_subtraction(value,
                    100), 26 /* terrane-site: case.trn:85:15-85:26 */), 100),
                    27 /* terrane-site: case.trn:85:15-85:32 */
                )
            } else {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_division(value, 2), 28 /* terrane-site: case.trn:87:15-87:24 */
                )
            };
            println!("{}", terrane_scalar_support::scalar_text(&value));
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
                        terrane_scalar_support::scalar_text(&String::from("lower-caught"))
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
fn global_condition(input: i8) -> i8 {
    let mut value: i8 = input;
    if __TERRANE_GLOBAL_PARITY
        .lock()
        .expect("program-global lock poisoned")
        .clone()
        .unwrap_or_else(|| __terrane_uninitialized_global("parity", "case.trn", 94, 6))
        == 0
    {
        value = __terrane_raised(
            terrane_int_support::fixed_division(value, 2),
            29 /* terrane-site: case.trn:95:13-95:22 */,
        );
    } else {
        value = __terrane_raised(
            terrane_int_support::fixed_addition(
                __terrane_raised(
                    terrane_int_support::fixed_multiplication(3, value),
                    30 /* terrane-site: case.trn:97:13-97:22 */,
                ),
                1,
            ),
            31 /* terrane-site: case.trn:97:13-97:26 */,
        );
    }
    return value;
}
fn main() {
    println!("{}", terrane_scalar_support::scalar_text(&transition(42)));
    println!("{}", terrane_scalar_support::scalar_text(&transition(41)));
    positive_lower_bound(72);
    positive_lower_bound(10);
    println!("{}", terrane_scalar_support::scalar_text(&global_condition(41)));
    let mut upper: i8 = 43;
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
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
                    terrane_int_support::fixed_division(upper, 2), 32 /* terrane-site: case.trn:109:15-109:24 */
                )
            } else {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_addition(__terrane_raised_completion!(terrane_int_support::fixed_multiplication(3,
                    upper), 33 /* terrane-site: case.trn:111:15-111:24 */), 1),
                    34 /* terrane-site: case.trn:111:15-111:28 */
                )
            };
            println!("{}", terrane_scalar_support::scalar_text(&upper));
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
                        terrane_scalar_support::scalar_text(&String::from("upper-caught"))
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
    let mut intermediate: i8 = -43;
    let __terrane_completion_2: TerraneCompletion<()> = (|| {
        let __terrane_try_2: TerraneCompletion<()> = (|| {
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
                    35 /* terrane-site: case.trn:118:22-118:38 */
                )
            } else {
                __terrane_raised_completion!(
                    terrane_int_support::fixed_addition(__terrane_raised_completion!(terrane_int_support::fixed_multiplication(3,
                    intermediate), 36 /* terrane-site: case.trn:120:22-120:38 */), 1),
                    37 /* terrane-site: case.trn:120:22-120:42 */
                )
            };
            println!("{}", terrane_scalar_support::scalar_text(&intermediate));
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
                    && __terrane_error_2.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_2 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("intermediate-caught"))
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
            38 /* terrane-site: case.trn:136:20-136:36 */,
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
                    39 /* terrane-site: case.trn:138:20-138:36 */,
                ),
                1,
            ),
            40 /* terrane-site: case.trn:138:20-138:40 */,
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_value = shared_value
        .lock().expect("reference lock poisoned").clone(); __terrane_value })
    );
}
