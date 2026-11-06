// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-document-support, terrane-platform-support
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
    Custom(DescriptorId),
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
            Self::Custom(descriptor) => {
                __terrane_error_registry::DESCRIPTORS[usize::from(descriptor.0)]
            }
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
            Self::Custom(_) => "source error",
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
    fn custom_raised(
        descriptor: DescriptorId,
        message: impl Into<String>,
        origin: TerraneSite,
    ) -> Self {
        Self::raised_with_message(TerraneErrorKind::Custom(descriptor), message, origin)
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
impl TerraneRaised for terrane_int_support::CoercionError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::CoercionError,
            self.to_string(),
            origin,
        )
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
    pub static DESCRIPTORS: [&str; 2] = [
        "/string-display-composition::render-error",
        "/typed-json-boundaries::policy-error",
    ];
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
    pub static FILES: [&str; 39] = [
        "tests/integration/bounded-arithmetic-families/case.trn",
        "tests/integration/byte-index-and-slice/case.trn",
        "tests/integration/bytes-and-legacy-digests/case.trn",
        "tests/integration/bytes-variadic-concat/case.trn",
        "tests/integration/bytes-views-encoding/case.trn",
        "tests/integration/checked-coercion/case.trn",
        "tests/integration/collection-identity-lifetime/case.trn",
        "tests/integration/collection-lookup-errors/case.trn",
        "tests/integration/collection-property-method-calls/case.trn",
        "tests/integration/collections-value-semantics/case.trn",
        "tests/integration/complete-float-surface/case.trn",
        "tests/integration/contextual-constant-arithmetic/case.trn",
        "tests/integration/exact-float-narrowing/case.trn",
        "tests/integration/explicit-numeric-coercions/case.trn",
        "tests/integration/fixed-integer-float-destination/case.trn",
        "tests/integration/float-narrowing-special-values/case.trn",
        "tests/integration/foundational-float-math/case.trn",
        "tests/integration/integer-coercions/case.trn",
        "tests/integration/integer-euclidean/case.trn",
        "tests/integration/map-entry-destructuring/case.trn",
        "tests/integration/map-key-removal/case.trn",
        "tests/integration/mixed-width-arithmetic/case.trn",
        "tests/integration/negative-wrapped-shift/case.trn",
        "tests/integration/null-byte-literal/case.trn",
        "tests/integration/numeric-destination-conversion/case.trn",
        "tests/integration/parse-radix-families/case.trn",
        "tests/integration/scalar-reference-transparency/case.trn",
        "tests/integration/stable-list-sorting/case.trn",
        "tests/integration/string-display-composition/case.trn",
        "tests/integration/text-families/case.trn",
        "tests/integration/tuple-type-boundaries/case.trn",
        "tests/integration/typed-catch-bindings/case.trn",
        "tests/integration/typed-document-decoding/case.trn",
        "tests/integration/typed-json-boundaries/case.trn",
        "tests/integration/typed-serialization-destinations/case.trn",
        "tests/integration/typed-serialization-matrix/case.trn",
        "tests/integration/typed-tuple-decode-shapes/case.trn",
        "core/documents.trn",
        "core/urls.trn",
    ];
    pub static FUNCTIONS: [&str; 49] = [
        "/bounded-arithmetic-families::test-scenario",
        "/byte-index-and-slice::test-scenario",
        "/bytes-and-legacy-digests::test-scenario",
        "/bytes-variadic-concat::test-scenario",
        "/bytes-views-encoding::test-scenario",
        "/checked-coercion::convert",
        "/checked-coercion::test-values",
        "/collection-identity-lifetime::release-order",
        "/collection-lookup-errors::test-scenario",
        "/collection-property-method-calls::test-scenario",
        "/collections-value-semantics::test-scenario",
        "/complete-float-surface::exercise32",
        "/complete-float-surface::exercise64",
        "/contextual-constant-arithmetic::test-scenario",
        "/exact-float-narrowing::test-scenario",
        "/explicit-numeric-coercions::test-values",
        "/fixed-integer-float-destination::test-scenario",
        "/float-narrowing-special-values::test-scenario",
        "/foundational-float-math::test-scenario",
        "/integer-coercions::test-values",
        "/integer-euclidean::test-scenario",
        "/map-entry-destructuring::test-scenario",
        "/map-key-removal::test-scenario",
        "/mixed-width-arithmetic::test-values",
        "/negative-wrapped-shift::test-scenario",
        "/null-byte-literal::test-scenario",
        "/numeric-destination-conversion::test-values",
        "/parse-radix-families::from-hex",
        "/parse-radix-families::fail",
        "/parse-radix-families::test-scenario",
        "/scalar-reference-transparency::test-scenario",
        "/stable-list-sorting::test-scenario",
        "/string-display-composition::to-string",
        "/string-display-composition::render-generic",
        "/string-display-composition::test-composition",
        "/text-families::test-scenario",
        "/tuple-type-boundaries::test-scenario",
        "/typed-catch-bindings::test-scenario",
        "/typed-document-decoding::test-scenario",
        "/typed-json-boundaries::unknown-policy",
        "/typed-json-boundaries::test-scenario",
        "/typed-serialization-destinations::test-destinations",
        "/typed-serialization-matrix::test-matrix",
        "/typed-tuple-decode-shapes::test-scenario",
        "/core/documents::make-document-list",
        "/core/documents::mapping-required-fields",
        "/core/documents::decode-document",
        "/core/urls::get",
        "/core/urls::get-all",
    ];
    pub static SITES: [Site; 251] = [
        /* terrane-site-row: site 0: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:8:10-8:26) */
        { Site { function: 0, file: 0, line: 8, column: 10, end_line: 8, end_column: 26 } },
        /* terrane-site-row: site 1: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:14:11-14:26) */
        { Site { function: 0, file: 0, line: 14, column: 11, end_line: 14, end_column: 26 } },
        /* terrane-site-row: site 2: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:14:30-14:48) */
        { Site { function: 0, file: 0, line: 14, column: 30, end_line: 14, end_column: 48 } },
        /* terrane-site-row: site 3: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:17:11-17:35) */
        { Site { function: 0, file: 0, line: 17, column: 11, end_line: 17, end_column: 35 } },
        /* terrane-site-row: site 4: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:18:3-18:31) */
        { Site { function: 0, file: 0, line: 18, column: 3, end_line: 18, end_column: 31 } },
        /* terrane-site-row: site 5: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:20:3-20:10) */
        { Site { function: 0, file: 0, line: 20, column: 3, end_line: 20, end_column: 10 } },
        /* terrane-site-row: site 6: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:21:3-21:10) */
        { Site { function: 0, file: 0, line: 21, column: 3, end_line: 21, end_column: 10 } },
        /* terrane-site-row: site 7: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:31:11-31:34) */
        { Site { function: 0, file: 0, line: 31, column: 11, end_line: 31, end_column: 34 } },
        /* terrane-site-row: site 8: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:31:38-31:65) */
        { Site { function: 0, file: 0, line: 31, column: 38, end_line: 31, end_column: 65 } },
        /* terrane-site-row: site 9: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:32:18-32:48) */
        { Site { function: 0, file: 0, line: 32, column: 18, end_line: 32, end_column: 48 } },
        /* terrane-site-row: site 10: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:33:56-33:82) */
        { Site { function: 0, file: 0, line: 33, column: 56, end_line: 33, end_column: 82 } },
        /* terrane-site-row: site 11: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:34:18-34:51) */
        { Site { function: 0, file: 0, line: 34, column: 18, end_line: 34, end_column: 51 } },
        /* terrane-site-row: site 12: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:35:11-35:37) */
        { Site { function: 0, file: 0, line: 35, column: 11, end_line: 35, end_column: 37 } },
        /* terrane-site-row: site 13: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:35:41-35:71) */
        { Site { function: 0, file: 0, line: 35, column: 41, end_line: 35, end_column: 71 } },
        /* terrane-site-row: site 14: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:36:56-36:85) */
        { Site { function: 0, file: 0, line: 36, column: 56, end_line: 36, end_column: 85 } },
        /* terrane-site-row: site 15: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:37:11-37:26) */
        { Site { function: 0, file: 0, line: 37, column: 11, end_line: 37, end_column: 26 } },
        /* terrane-site-row: site 16: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:37:30-37:48) */
        { Site { function: 0, file: 0, line: 37, column: 30, end_line: 37, end_column: 48 } },
        /* terrane-site-row: site 17: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:38:11-38:25) */
        { Site { function: 0, file: 0, line: 38, column: 11, end_line: 38, end_column: 25 } },
        /* terrane-site-row: site 18: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:38:29-38:46) */
        { Site { function: 0, file: 0, line: 38, column: 29, end_line: 38, end_column: 46 } },
        /* terrane-site-row: site 19: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:39:11-39:27) */
        { Site { function: 0, file: 0, line: 39, column: 11, end_line: 39, end_column: 27 } },
        /* terrane-site-row: site 20: /bounded-arithmetic-families::test-scenario (tests/integration/bounded-arithmetic-families/case.trn:39:31-39:50) */
        { Site { function: 0, file: 0, line: 39, column: 31, end_line: 39, end_column: 50 } },
        /* terrane-site-row: site 21: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:8:10-8:17) */
        { Site { function: 1, file: 1, line: 8, column: 10, end_line: 8, end_column: 17 } },
        /* terrane-site-row: site 22: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:8:19-8:26) */
        { Site { function: 1, file: 1, line: 8, column: 19, end_line: 8, end_column: 26 } },
        /* terrane-site-row: site 23: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:9:23-9:34) */
        { Site { function: 1, file: 1, line: 9, column: 23, end_line: 9, end_column: 34 } },
        /* terrane-site-row: site 24: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:9:18-9:35) */
        { Site { function: 1, file: 1, line: 9, column: 18, end_line: 9, end_column: 35 } },
        /* terrane-site-row: site 25: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:10:11-10:30) */
        { Site { function: 1, file: 1, line: 10, column: 11, end_line: 10, end_column: 30 } },
        /* terrane-site-row: site 26: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:11:24-11:38) */
        { Site { function: 1, file: 1, line: 11, column: 24, end_line: 11, end_column: 38 } },
        /* terrane-site-row: site 27: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:11:19-11:39) */
        { Site { function: 1, file: 1, line: 11, column: 19, end_line: 11, end_column: 39 } },
        /* terrane-site-row: site 28: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:12:10-12:20) */
        { Site { function: 1, file: 1, line: 12, column: 10, end_line: 12, end_column: 20 } },
        /* terrane-site-row: site 29: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:12:22-12:32) */
        { Site { function: 1, file: 1, line: 12, column: 22, end_line: 12, end_column: 32 } },
        /* terrane-site-row: site 30: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:12:34-12:44) */
        { Site { function: 1, file: 1, line: 12, column: 34, end_line: 12, end_column: 44 } },
        /* terrane-site-row: site 31: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:13:22-13:53) */
        { Site { function: 1, file: 1, line: 13, column: 22, end_line: 13, end_column: 53 } },
        /* terrane-site-row: site 32: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:13:17-13:54) */
        { Site { function: 1, file: 1, line: 13, column: 17, end_line: 13, end_column: 54 } },
        /* terrane-site-row: site 33: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:15:22-15:41) */
        { Site { function: 1, file: 1, line: 15, column: 22, end_line: 15, end_column: 41 } },
        /* terrane-site-row: site 34: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:15:17-15:42) */
        { Site { function: 1, file: 1, line: 15, column: 17, end_line: 15, end_column: 42 } },
        /* terrane-site-row: site 35: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:16:10-16:18) */
        { Site { function: 1, file: 1, line: 16, column: 10, end_line: 16, end_column: 18 } },
        /* terrane-site-row: site 36: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:18:12-18:29) */
        { Site { function: 1, file: 1, line: 18, column: 12, end_line: 18, end_column: 29 } },
        /* terrane-site-row: site 37: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:22:17-22:28) */
        { Site { function: 1, file: 1, line: 22, column: 17, end_line: 22, end_column: 28 } },
        /* terrane-site-row: site 38: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:22:12-22:29) */
        { Site { function: 1, file: 1, line: 22, column: 12, end_line: 22, end_column: 29 } },
        /* terrane-site-row: site 39: /byte-index-and-slice::test-scenario (tests/integration/byte-index-and-slice/case.trn:26:12-26:21) */
        { Site { function: 1, file: 1, line: 26, column: 12, end_line: 26, end_column: 21 } },
        /* terrane-site-row: site 40: /bytes-and-legacy-digests::test-scenario (tests/integration/bytes-and-legacy-digests/case.trn:12:11-12:28) */
        { Site { function: 2, file: 2, line: 12, column: 11, end_line: 12, end_column: 28 } },
        /* terrane-site-row: site 41: /bytes-variadic-concat::test-scenario (tests/integration/bytes-variadic-concat/case.trn:7:11-7:32) */
        { Site { function: 3, file: 3, line: 7, column: 11, end_line: 7, end_column: 32 } },
        /* terrane-site-row: site 42: /bytes-variadic-concat::test-scenario (tests/integration/bytes-variadic-concat/case.trn:9:11-9:33) */
        { Site { function: 3, file: 3, line: 9, column: 11, end_line: 9, end_column: 33 } },
        /* terrane-site-row: site 43: /bytes-views-encoding::test-scenario (tests/integration/bytes-views-encoding/case.trn:19:20-19:40) */
        { Site { function: 4, file: 4, line: 19, column: 20, end_line: 19, end_column: 40 } },
        /* terrane-site-row: site 44: /bytes-views-encoding::test-scenario (tests/integration/bytes-views-encoding/case.trn:21:25-21:65) */
        { Site { function: 4, file: 4, line: 21, column: 25, end_line: 21, end_column: 65 } },
        /* terrane-site-row: site 45: /bytes-views-encoding::test-scenario (tests/integration/bytes-views-encoding/case.trn:22:25-22:65) */
        { Site { function: 4, file: 4, line: 22, column: 25, end_line: 22, end_column: 65 } },
        /* terrane-site-row: site 46: /bytes-views-encoding::test-scenario (tests/integration/bytes-views-encoding/case.trn:23:25-23:65) */
        { Site { function: 4, file: 4, line: 23, column: 25, end_line: 23, end_column: 65 } },
        /* terrane-site-row: site 47: /bytes-views-encoding::test-scenario (tests/integration/bytes-views-encoding/case.trn:24:25-24:65) */
        { Site { function: 4, file: 4, line: 24, column: 25, end_line: 24, end_column: 65 } },
        /* terrane-site-row: site 48: /checked-coercion::convert (tests/integration/checked-coercion/case.trn:9:14-9:18) */
        { Site { function: 5, file: 5, line: 9, column: 14, end_line: 9, end_column: 18 } },
        /* terrane-site-row: site 49: /checked-coercion::test-values (tests/integration/checked-coercion/case.trn:19:13-19:19) */
        { Site { function: 6, file: 5, line: 19, column: 13, end_line: 19, end_column: 19 } },
        /* terrane-site-row: site 50: /checked-coercion::test-values (tests/integration/checked-coercion/case.trn:21:20-21:33) */
        { Site { function: 6, file: 5, line: 21, column: 20, end_line: 21, end_column: 33 } },
        /* terrane-site-row: site 51: /checked-coercion::test-values (tests/integration/checked-coercion/case.trn:22:23-22:38) */
        { Site { function: 6, file: 5, line: 22, column: 23, end_line: 22, end_column: 38 } },
        /* terrane-site-row: site 52: /collection-identity-lifetime::release-order (tests/integration/collection-identity-lifetime/case.trn:17:3-17:50) */
        { Site { function: 7, file: 6, line: 17, column: 3, end_line: 17, end_column: 50 } },
        /* terrane-site-row: site 53: /collection-identity-lifetime::release-order (tests/integration/collection-identity-lifetime/case.trn:20:22-20:38) */
        { Site { function: 7, file: 6, line: 20, column: 22, end_line: 20, end_column: 38 } },
        /* terrane-site-row: site 54: /collection-identity-lifetime::release-order (tests/integration/collection-identity-lifetime/case.trn:24:5-24:21) */
        { Site { function: 7, file: 6, line: 24, column: 5, end_line: 24, end_column: 21 } },
        /* terrane-site-row: site 55: /collection-identity-lifetime::release-order (tests/integration/collection-identity-lifetime/case.trn:33:3-33:57) */
        { Site { function: 7, file: 6, line: 33, column: 3, end_line: 33, end_column: 57 } },
        /* terrane-site-row: site 56: /collection-identity-lifetime::release-order (tests/integration/collection-identity-lifetime/case.trn:34:10-34:21) */
        { Site { function: 7, file: 6, line: 34, column: 10, end_line: 34, end_column: 21 } },
        /* terrane-site-row: site 57: /collection-identity-lifetime::release-order (tests/integration/collection-identity-lifetime/case.trn:34:28-34:40) */
        { Site { function: 7, file: 6, line: 34, column: 28, end_line: 34, end_column: 40 } },
        /* terrane-site-row: site 58: /collection-lookup-errors::test-scenario (tests/integration/collection-lookup-errors/case.trn:7:12-7:21) */
        { Site { function: 8, file: 7, line: 7, column: 12, end_line: 7, end_column: 21 } },
        /* terrane-site-row: site 59: /collection-lookup-errors::test-scenario (tests/integration/collection-lookup-errors/case.trn:14:12-14:35) */
        { Site { function: 8, file: 7, line: 14, column: 12, end_line: 14, end_column: 35 } },
        /* terrane-site-row: site 60: /collection-property-method-calls::test-scenario (tests/integration/collection-property-method-calls/case.trn:10:29-10:40) */
        { Site { function: 9, file: 8, line: 10, column: 29, end_line: 10, end_column: 40 } },
        /* terrane-site-row: site 61: /collection-property-method-calls::test-scenario (tests/integration/collection-property-method-calls/case.trn:11:12-11:21) */
        { Site { function: 9, file: 8, line: 11, column: 12, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 62: /collection-property-method-calls::test-scenario (tests/integration/collection-property-method-calls/case.trn:11:23-11:32) */
        { Site { function: 9, file: 8, line: 11, column: 23, end_line: 11, end_column: 32 } },
        /* terrane-site-row: site 63: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:8:3-8:21) */
        { Site { function: 10, file: 9, line: 8, column: 3, end_line: 8, end_column: 21 } },
        /* terrane-site-row: site 64: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:9:47-9:61) */
        { Site { function: 10, file: 9, line: 9, column: 47, end_line: 9, end_column: 61 } },
        /* terrane-site-row: site 65: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:9:63-9:77) */
        { Site { function: 10, file: 9, line: 9, column: 63, end_line: 9, end_column: 77 } },
        /* terrane-site-row: site 66: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:15:10-15:27) */
        { Site { function: 10, file: 9, line: 15, column: 10, end_line: 15, end_column: 27 } },
        /* terrane-site-row: site 67: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:21:23-21:30) */
        { Site { function: 10, file: 9, line: 21, column: 23, end_line: 21, end_column: 30 } },
        /* terrane-site-row: site 68: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:24:13-24:24) */
        { Site { function: 10, file: 9, line: 24, column: 13, end_line: 24, end_column: 24 } },
        /* terrane-site-row: site 69: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:27:15-27:38) */
        { Site { function: 10, file: 9, line: 27, column: 15, end_line: 27, end_column: 38 } },
        /* terrane-site-row: site 70: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:31:11-31:26) */
        { Site { function: 10, file: 9, line: 31, column: 11, end_line: 31, end_column: 26 } },
        /* terrane-site-row: site 71: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:36:5-36:19) */
        { Site { function: 10, file: 9, line: 36, column: 5, end_line: 36, end_column: 19 } },
        /* terrane-site-row: site 72: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:41:36-41:63) */
        { Site { function: 10, file: 9, line: 41, column: 36, end_line: 41, end_column: 63 } },
        /* terrane-site-row: site 73: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:56:29-56:41) */
        { Site { function: 10, file: 9, line: 56, column: 29, end_line: 56, end_column: 41 } },
        /* terrane-site-row: site 74: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:58:10-58:22) */
        { Site { function: 10, file: 9, line: 58, column: 10, end_line: 58, end_column: 22 } },
        /* terrane-site-row: site 75: /collections-value-semantics::test-scenario (tests/integration/collections-value-semantics/case.trn:58:24-58:36) */
        { Site { function: 10, file: 9, line: 58, column: 24, end_line: 58, end_column: 36 } },
        /* terrane-site-row: site 76: /complete-float-surface::exercise32 (tests/integration/complete-float-surface/case.trn:50:29-50:38) */
        { Site { function: 11, file: 10, line: 50, column: 29, end_line: 50, end_column: 38 } },
        /* terrane-site-row: site 77: /complete-float-surface::exercise32 (tests/integration/complete-float-surface/case.trn:50:67-50:84) */
        { Site { function: 11, file: 10, line: 50, column: 67, end_line: 50, end_column: 84 } },
        /* terrane-site-row: site 78: /complete-float-surface::exercise64 (tests/integration/complete-float-surface/case.trn:115:69-115:86) */
        { Site { function: 12, file: 10, line: 115, column: 69, end_line: 115, end_column: 86 } },
        /* terrane-site-row: site 79: /contextual-constant-arithmetic::test-scenario (tests/integration/contextual-constant-arithmetic/case.trn:14:17-14:29) */
        { Site { function: 13, file: 11, line: 14, column: 17, end_line: 14, end_column: 29 } },
        /* terrane-site-row: site 80: /contextual-constant-arithmetic::test-scenario (tests/integration/contextual-constant-arithmetic/case.trn:23:11-23:17) */
        { Site { function: 13, file: 11, line: 23, column: 11, end_line: 23, end_column: 17 } },
        /* terrane-site-row: site 81: /exact-float-narrowing::test-scenario (tests/integration/exact-float-narrowing/case.trn:4:20-4:26) */
        { Site { function: 14, file: 12, line: 4, column: 20, end_line: 4, end_column: 26 } },
        /* terrane-site-row: site 82: /explicit-numeric-coercions::test-values (tests/integration/explicit-numeric-coercions/case.trn:10:23-10:38) */
        { Site { function: 15, file: 13, line: 10, column: 23, end_line: 10, end_column: 38 } },
        /* terrane-site-row: site 83: /explicit-numeric-coercions::test-values (tests/integration/explicit-numeric-coercions/case.trn:11:23-11:38) */
        { Site { function: 15, file: 13, line: 11, column: 23, end_line: 11, end_column: 38 } },
        /* terrane-site-row: site 84: /explicit-numeric-coercions::test-values (tests/integration/explicit-numeric-coercions/case.trn:23:22-23:36) */
        { Site { function: 15, file: 13, line: 23, column: 22, end_line: 23, end_column: 36 } },
        /* terrane-site-row: site 85: /explicit-numeric-coercions::test-values (tests/integration/explicit-numeric-coercions/case.trn:29:24-29:41) */
        { Site { function: 15, file: 13, line: 29, column: 24, end_line: 29, end_column: 41 } },
        /* terrane-site-row: site 86: /explicit-numeric-coercions::test-values (tests/integration/explicit-numeric-coercions/case.trn:31:19-31:31) */
        { Site { function: 15, file: 13, line: 31, column: 19, end_line: 31, end_column: 31 } },
        /* terrane-site-row: site 87: /explicit-numeric-coercions::test-values (tests/integration/explicit-numeric-coercions/case.trn:44:26-44:40) */
        { Site { function: 15, file: 13, line: 44, column: 26, end_line: 44, end_column: 40 } },
        /* terrane-site-row: site 88: /explicit-numeric-coercions::test-values (tests/integration/explicit-numeric-coercions/case.trn:50:26-50:42) */
        { Site { function: 15, file: 13, line: 50, column: 26, end_line: 50, end_column: 42 } },
        /* terrane-site-row: site 89: /fixed-integer-float-destination::test-scenario (tests/integration/fixed-integer-float-destination/case.trn:6:28-6:36) */
        { Site { function: 16, file: 14, line: 6, column: 28, end_line: 6, end_column: 36 } },
        /* terrane-site-row: site 90: /fixed-integer-float-destination::test-scenario (tests/integration/fixed-integer-float-destination/case.trn:8:30-8:40) */
        { Site { function: 16, file: 14, line: 8, column: 30, end_line: 8, end_column: 40 } },
        /* terrane-site-row: site 91: /fixed-integer-float-destination::test-scenario (tests/integration/fixed-integer-float-destination/case.trn:10:29-10:38) */
        { Site { function: 16, file: 14, line: 10, column: 29, end_line: 10, end_column: 38 } },
        /* terrane-site-row: site 92: /fixed-integer-float-destination::test-scenario (tests/integration/fixed-integer-float-destination/case.trn:12:31-12:42) */
        { Site { function: 16, file: 14, line: 12, column: 31, end_line: 12, end_column: 42 } },
        /* terrane-site-row: site 93: /fixed-integer-float-destination::test-scenario (tests/integration/fixed-integer-float-destination/case.trn:14:28-14:36) */
        { Site { function: 16, file: 14, line: 14, column: 28, end_line: 14, end_column: 36 } },
        /* terrane-site-row: site 94: /fixed-integer-float-destination::test-scenario (tests/integration/fixed-integer-float-destination/case.trn:16:29-16:52) */
        { Site { function: 16, file: 14, line: 16, column: 29, end_line: 16, end_column: 52 } },
        /* terrane-site-row: site 95: /fixed-integer-float-destination::test-scenario (tests/integration/fixed-integer-float-destination/case.trn:27:26-27:35) */
        { Site { function: 16, file: 14, line: 27, column: 26, end_line: 27, end_column: 35 } },
        /* terrane-site-row: site 96: /fixed-integer-float-destination::test-scenario (tests/integration/fixed-integer-float-destination/case.trn:34:26-34:35) */
        { Site { function: 16, file: 14, line: 34, column: 26, end_line: 34, end_column: 35 } },
        /* terrane-site-row: site 97: /float-narrowing-special-values::test-scenario (tests/integration/float-narrowing-special-values/case.trn:10:27-10:40) */
        { Site { function: 17, file: 15, line: 10, column: 27, end_line: 10, end_column: 40 } },
        /* terrane-site-row: site 98: /float-narrowing-special-values::test-scenario (tests/integration/float-narrowing-special-values/case.trn:11:31-11:48) */
        { Site { function: 17, file: 15, line: 11, column: 31, end_line: 11, end_column: 48 } },
        /* terrane-site-row: site 99: /float-narrowing-special-values::test-scenario (tests/integration/float-narrowing-special-values/case.trn:12:31-12:48) */
        { Site { function: 17, file: 15, line: 12, column: 31, end_line: 12, end_column: 48 } },
        /* terrane-site-row: site 100: /float-narrowing-special-values::test-scenario (tests/integration/float-narrowing-special-values/case.trn:13:30-13:37) */
        { Site { function: 17, file: 15, line: 13, column: 30, end_line: 13, end_column: 37 } },
        /* terrane-site-row: site 101: /foundational-float-math::test-scenario (tests/integration/foundational-float-math/case.trn:13:30-13:39) */
        { Site { function: 18, file: 16, line: 13, column: 30, end_line: 13, end_column: 39 } },
        /* terrane-site-row: site 102: /foundational-float-math::test-scenario (tests/integration/foundational-float-math/case.trn:13:48-13:57) */
        { Site { function: 18, file: 16, line: 13, column: 48, end_line: 13, end_column: 57 } },
        /* terrane-site-row: site 103: /foundational-float-math::test-scenario (tests/integration/foundational-float-math/case.trn:24:30-24:39) */
        { Site { function: 18, file: 16, line: 24, column: 30, end_line: 24, end_column: 39 } },
        /* terrane-site-row: site 104: /foundational-float-math::test-scenario (tests/integration/foundational-float-math/case.trn:24:48-24:57) */
        { Site { function: 18, file: 16, line: 24, column: 48, end_line: 24, end_column: 57 } },
        /* terrane-site-row: site 105: /integer-coercions::test-values (tests/integration/integer-coercions/case.trn:6:11-6:14) */
        { Site { function: 19, file: 17, line: 6, column: 11, end_line: 6, end_column: 14 } },
        /* terrane-site-row: site 106: /integer-euclidean::test-scenario (tests/integration/integer-euclidean/case.trn:4:10-4:16) */
        { Site { function: 20, file: 18, line: 4, column: 10, end_line: 4, end_column: 16 } },
        /* terrane-site-row: site 107: /integer-euclidean::test-scenario (tests/integration/integer-euclidean/case.trn:5:10-5:16) */
        { Site { function: 20, file: 18, line: 5, column: 10, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 108: /integer-euclidean::test-scenario (tests/integration/integer-euclidean/case.trn:6:10-6:16) */
        { Site { function: 20, file: 18, line: 6, column: 10, end_line: 6, end_column: 16 } },
        /* terrane-site-row: site 109: /integer-euclidean::test-scenario (tests/integration/integer-euclidean/case.trn:7:10-7:16) */
        { Site { function: 20, file: 18, line: 7, column: 10, end_line: 7, end_column: 16 } },
        /* terrane-site-row: site 110: /map-entry-destructuring::test-scenario (tests/integration/map-entry-destructuring/case.trn:11:10-11:26) */
        { Site { function: 21, file: 19, line: 11, column: 10, end_line: 11, end_column: 26 } },
        /* terrane-site-row: site 111: /map-entry-destructuring::test-scenario (tests/integration/map-entry-destructuring/case.trn:11:28-11:45) */
        { Site { function: 21, file: 19, line: 11, column: 28, end_line: 11, end_column: 45 } },
        /* terrane-site-row: site 112: /map-key-removal::test-scenario (tests/integration/map-key-removal/case.trn:15:17-15:41) */
        { Site { function: 22, file: 20, line: 15, column: 17, end_line: 15, end_column: 41 } },
        /* terrane-site-row: site 113: /map-key-removal::test-scenario (tests/integration/map-key-removal/case.trn:25:5-25:30) */
        { Site { function: 22, file: 20, line: 25, column: 5, end_line: 25, end_column: 30 } },
        /* terrane-site-row: site 114: /map-key-removal::test-scenario (tests/integration/map-key-removal/case.trn:31:11-31:37) */
        { Site { function: 22, file: 20, line: 31, column: 11, end_line: 31, end_column: 37 } },
        /* terrane-site-row: site 115: /map-key-removal::test-scenario (tests/integration/map-key-removal/case.trn:40:3-40:35) */
        { Site { function: 22, file: 20, line: 40, column: 3, end_line: 40, end_column: 35 } },
        /* terrane-site-row: site 116: /map-key-removal::test-scenario (tests/integration/map-key-removal/case.trn:44:3-44:24) */
        { Site { function: 22, file: 20, line: 44, column: 3, end_line: 44, end_column: 24 } },
        /* terrane-site-row: site 117: /map-key-removal::test-scenario (tests/integration/map-key-removal/case.trn:49:29-49:50) */
        { Site { function: 22, file: 20, line: 49, column: 29, end_line: 49, end_column: 50 } },
        /* terrane-site-row: site 118: /map-key-removal::test-scenario (tests/integration/map-key-removal/case.trn:51:20-51:40) */
        { Site { function: 22, file: 20, line: 51, column: 20, end_line: 51, end_column: 40 } },
        /* terrane-site-row: site 119: /mixed-width-arithmetic::test-values (tests/integration/mixed-width-arithmetic/case.trn:7:17-7:29) */
        { Site { function: 23, file: 21, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 120: /mixed-width-arithmetic::test-values (tests/integration/mixed-width-arithmetic/case.trn:8:20-8:35) */
        { Site { function: 23, file: 21, line: 8, column: 20, end_line: 8, end_column: 35 } },
        /* terrane-site-row: site 121: /negative-wrapped-shift::test-scenario (tests/integration/negative-wrapped-shift/case.trn:6:13-6:38) */
        { Site { function: 24, file: 22, line: 6, column: 13, end_line: 6, end_column: 38 } },
        /* terrane-site-row: site 122: /null-byte-literal::test-scenario (tests/integration/null-byte-literal/case.trn:6:23-6:39) */
        { Site { function: 25, file: 23, line: 6, column: 23, end_line: 6, end_column: 39 } },
        /* terrane-site-row: site 123: /numeric-destination-conversion::test-values (tests/integration/numeric-destination-conversion/case.trn:11:23-11:28) */
        { Site { function: 26, file: 24, line: 11, column: 23, end_line: 11, end_column: 28 } },
        /* terrane-site-row: site 124: /numeric-destination-conversion::test-values (tests/integration/numeric-destination-conversion/case.trn:13:19-13:24) */
        { Site { function: 26, file: 24, line: 13, column: 19, end_line: 13, end_column: 24 } },
        /* terrane-site-row: site 125: /numeric-destination-conversion::test-values (tests/integration/numeric-destination-conversion/case.trn:27:10-27:19) */
        { Site { function: 26, file: 24, line: 27, column: 10, end_line: 27, end_column: 19 } },
        /* terrane-site-row: site 126: /parse-radix-families::from-hex (tests/integration/parse-radix-families/case.trn:4:10-4:26) */
        { Site { function: 27, file: 25, line: 4, column: 10, end_line: 4, end_column: 26 } },
        /* terrane-site-row: site 127: /parse-radix-families::fail (tests/integration/parse-radix-families/case.trn:7:3-7:23) */
        { Site { function: 28, file: 25, line: 7, column: 3, end_line: 7, end_column: 23 } },
        /* terrane-site-row: site 128: /parse-radix-families::test-scenario (tests/integration/parse-radix-families/case.trn:13:11-13:26) */
        { Site { function: 29, file: 25, line: 13, column: 11, end_line: 13, end_column: 26 } },
        /* terrane-site-row: site 129: /scalar-reference-transparency::test-scenario (tests/integration/scalar-reference-transparency/case.trn:12:13-12:33) */
        { Site { function: 30, file: 26, line: 12, column: 13, end_line: 12, end_column: 33 } },
        /* terrane-site-row: site 130: /scalar-reference-transparency::test-scenario (tests/integration/scalar-reference-transparency/case.trn:17:13-17:28) */
        { Site { function: 30, file: 26, line: 17, column: 13, end_line: 17, end_column: 28 } },
        /* terrane-site-row: site 131: /scalar-reference-transparency::test-scenario (tests/integration/scalar-reference-transparency/case.trn:18:12-18:24) */
        { Site { function: 30, file: 26, line: 18, column: 12, end_line: 18, end_column: 24 } },
        /* terrane-site-row: site 132: /scalar-reference-transparency::test-scenario (tests/integration/scalar-reference-transparency/case.trn:22:12-22:21) */
        { Site { function: 30, file: 26, line: 22, column: 12, end_line: 22, end_column: 21 } },
        /* terrane-site-row: site 133: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:7:10-7:21) */
        { Site { function: 31, file: 27, line: 7, column: 10, end_line: 7, end_column: 21 } },
        /* terrane-site-row: site 134: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:7:23-7:34) */
        { Site { function: 31, file: 27, line: 7, column: 23, end_line: 7, end_column: 34 } },
        /* terrane-site-row: site 135: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:9:10-9:21) */
        { Site { function: 31, file: 27, line: 9, column: 10, end_line: 9, end_column: 21 } },
        /* terrane-site-row: site 136: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:9:23-9:34) */
        { Site { function: 31, file: 27, line: 9, column: 23, end_line: 9, end_column: 34 } },
        /* terrane-site-row: site 137: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:31:10-31:18) */
        { Site { function: 31, file: 27, line: 31, column: 10, end_line: 31, end_column: 18 } },
        /* terrane-site-row: site 138: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:31:20-31:29) */
        { Site { function: 31, file: 27, line: 31, column: 20, end_line: 31, end_column: 29 } },
        /* terrane-site-row: site 139: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:31:31-31:40) */
        { Site { function: 31, file: 27, line: 31, column: 31, end_line: 31, end_column: 40 } },
        /* terrane-site-row: site 140: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:31:42-31:51) */
        { Site { function: 31, file: 27, line: 31, column: 42, end_line: 31, end_column: 51 } },
        /* terrane-site-row: site 141: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:31:53-31:63) */
        { Site { function: 31, file: 27, line: 31, column: 53, end_line: 31, end_column: 63 } },
        /* terrane-site-row: site 142: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:32:10-32:19) */
        { Site { function: 31, file: 27, line: 32, column: 10, end_line: 32, end_column: 19 } },
        /* terrane-site-row: site 143: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:32:21-32:31) */
        { Site { function: 31, file: 27, line: 32, column: 21, end_line: 32, end_column: 31 } },
        /* terrane-site-row: site 144: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:32:33-32:43) */
        { Site { function: 31, file: 27, line: 32, column: 33, end_line: 32, end_column: 43 } },
        /* terrane-site-row: site 145: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:32:45-32:55) */
        { Site { function: 31, file: 27, line: 32, column: 45, end_line: 32, end_column: 55 } },
        /* terrane-site-row: site 146: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:32:57-32:68) */
        { Site { function: 31, file: 27, line: 32, column: 57, end_line: 32, end_column: 68 } },
        /* terrane-site-row: site 147: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:36:10-36:18) */
        { Site { function: 31, file: 27, line: 36, column: 10, end_line: 36, end_column: 18 } },
        /* terrane-site-row: site 148: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:36:20-36:28) */
        { Site { function: 31, file: 27, line: 36, column: 20, end_line: 36, end_column: 28 } },
        /* terrane-site-row: site 149: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:36:30-36:38) */
        { Site { function: 31, file: 27, line: 36, column: 30, end_line: 36, end_column: 38 } },
        /* terrane-site-row: site 150: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:38:10-38:18) */
        { Site { function: 31, file: 27, line: 38, column: 10, end_line: 38, end_column: 18 } },
        /* terrane-site-row: site 151: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:38:20-38:28) */
        { Site { function: 31, file: 27, line: 38, column: 20, end_line: 38, end_column: 28 } },
        /* terrane-site-row: site 152: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:38:30-38:38) */
        { Site { function: 31, file: 27, line: 38, column: 30, end_line: 38, end_column: 38 } },
        /* terrane-site-row: site 153: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:50:10-50:19) */
        { Site { function: 31, file: 27, line: 50, column: 10, end_line: 50, end_column: 19 } },
        /* terrane-site-row: site 154: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:50:30-50:39) */
        { Site { function: 31, file: 27, line: 50, column: 30, end_line: 50, end_column: 39 } },
        /* terrane-site-row: site 155: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:50:55-50:64) */
        { Site { function: 31, file: 27, line: 50, column: 55, end_line: 50, end_column: 64 } },
        /* terrane-site-row: site 156: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:50:80-50:89) */
        { Site { function: 31, file: 27, line: 50, column: 80, end_line: 50, end_column: 89 } },
        /* terrane-site-row: site 157: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:50:105-50:114) */
        { Site { function: 31, file: 27, line: 50, column: 105, end_line: 50, end_column: 114 } },
        /* terrane-site-row: site 158: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:50:125-50:134) */
        { Site { function: 31, file: 27, line: 50, column: 125, end_line: 50, end_column: 134 } },
        /* terrane-site-row: site 159: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:50:149-50:158) */
        { Site { function: 31, file: 27, line: 50, column: 149, end_line: 50, end_column: 158 } },
        /* terrane-site-row: site 160: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:52:10-52:19) */
        { Site { function: 31, file: 27, line: 52, column: 10, end_line: 52, end_column: 19 } },
        /* terrane-site-row: site 161: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:52:30-52:39) */
        { Site { function: 31, file: 27, line: 52, column: 30, end_line: 52, end_column: 39 } },
        /* terrane-site-row: site 162: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:52:55-52:64) */
        { Site { function: 31, file: 27, line: 52, column: 55, end_line: 52, end_column: 64 } },
        /* terrane-site-row: site 163: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:52:80-52:89) */
        { Site { function: 31, file: 27, line: 52, column: 80, end_line: 52, end_column: 89 } },
        /* terrane-site-row: site 164: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:52:105-52:114) */
        { Site { function: 31, file: 27, line: 52, column: 105, end_line: 52, end_column: 114 } },
        /* terrane-site-row: site 165: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:52:125-52:134) */
        { Site { function: 31, file: 27, line: 52, column: 125, end_line: 52, end_column: 134 } },
        /* terrane-site-row: site 166: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:52:150-52:159) */
        { Site { function: 31, file: 27, line: 52, column: 150, end_line: 52, end_column: 159 } },
        /* terrane-site-row: site 167: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:52:174-52:183) */
        { Site { function: 31, file: 27, line: 52, column: 174, end_line: 52, end_column: 183 } },
        /* terrane-site-row: site 168: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:56:10-56:19) */
        { Site { function: 31, file: 27, line: 56, column: 10, end_line: 56, end_column: 19 } },
        /* terrane-site-row: site 169: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:56:21-56:30) */
        { Site { function: 31, file: 27, line: 56, column: 21, end_line: 56, end_column: 30 } },
        /* terrane-site-row: site 170: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:61:10-61:22) */
        { Site { function: 31, file: 27, line: 61, column: 10, end_line: 61, end_column: 22 } },
        /* terrane-site-row: site 171: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:61:24-61:33) */
        { Site { function: 31, file: 27, line: 61, column: 24, end_line: 61, end_column: 33 } },
        /* terrane-site-row: site 172: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:61:35-61:46) */
        { Site { function: 31, file: 27, line: 61, column: 35, end_line: 61, end_column: 46 } },
        /* terrane-site-row: site 173: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:69:17-69:29) */
        { Site { function: 31, file: 27, line: 69, column: 17, end_line: 69, end_column: 29 } },
        /* terrane-site-row: site 174: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:72:17-72:29) */
        { Site { function: 31, file: 27, line: 72, column: 17, end_line: 72, end_column: 29 } },
        /* terrane-site-row: site 175: /stable-list-sorting::test-scenario (tests/integration/stable-list-sorting/case.trn:78:24-78:33) */
        { Site { function: 31, file: 27, line: 78, column: 24, end_line: 78, end_column: 33 } },
        /* terrane-site-row: site 176: /string-display-composition::to-string (tests/integration/string-display-composition/case.trn:23:9-23:37) */
        { Site { function: 32, file: 28, line: 23, column: 9, end_line: 23, end_column: 37 } },
        /* terrane-site-row: site 177: /string-display-composition::render-generic (tests/integration/string-display-composition/case.trn:34:25-34:30) */
        { Site { function: 33, file: 28, line: 34, column: 25, end_line: 34, end_column: 30 } },
        /* terrane-site-row: site 178: /string-display-composition::test-composition (tests/integration/string-display-composition/case.trn:50:29-50:45) */
        { Site { function: 34, file: 28, line: 50, column: 29, end_line: 50, end_column: 45 } },
        /* terrane-site-row: site 179: /string-display-composition::test-composition (tests/integration/string-display-composition/case.trn:54:17-54:33) */
        { Site { function: 34, file: 28, line: 54, column: 17, end_line: 54, end_column: 33 } },
        /* terrane-site-row: site 180: /string-display-composition::test-composition (tests/integration/string-display-composition/case.trn:58:25-58:41) */
        { Site { function: 34, file: 28, line: 58, column: 25, end_line: 58, end_column: 41 } },
        /* terrane-site-row: site 181: /string-display-composition::test-composition (tests/integration/string-display-composition/case.trn:62:24-62:40) */
        { Site { function: 34, file: 28, line: 62, column: 24, end_line: 62, end_column: 40 } },
        /* terrane-site-row: site 182: /string-display-composition::test-composition (tests/integration/string-display-composition/case.trn:66:17-66:51) */
        { Site { function: 34, file: 28, line: 66, column: 17, end_line: 66, end_column: 51 } },
        /* terrane-site-row: site 183: /string-display-composition::test-composition (tests/integration/string-display-composition/case.trn:69:13-69:45) */
        { Site { function: 34, file: 28, line: 69, column: 13, end_line: 69, end_column: 45 } },
        /* terrane-site-row: site 184: /string-display-composition::test-composition (tests/integration/string-display-composition/case.trn:71:27-71:33) */
        { Site { function: 34, file: 28, line: 71, column: 27, end_line: 71, end_column: 33 } },
        /* terrane-site-row: site 185: /text-families::test-scenario (tests/integration/text-families/case.trn:25:11-25:52) */
        { Site { function: 35, file: 29, line: 25, column: 11, end_line: 25, end_column: 52 } },
        /* terrane-site-row: site 186: /text-families::test-scenario (tests/integration/text-families/case.trn:28:11-28:65) */
        { Site { function: 35, file: 29, line: 28, column: 11, end_line: 28, end_column: 65 } },
        /* terrane-site-row: site 187: /text-families::test-scenario (tests/integration/text-families/case.trn:33:11-33:65) */
        { Site { function: 35, file: 29, line: 33, column: 11, end_line: 33, end_column: 65 } },
        /* terrane-site-row: site 188: /text-families::test-scenario (tests/integration/text-families/case.trn:34:11-34:52) */
        { Site { function: 35, file: 29, line: 34, column: 11, end_line: 34, end_column: 52 } },
        /* terrane-site-row: site 189: /text-families::test-scenario (tests/integration/text-families/case.trn:37:11-37:55) */
        { Site { function: 35, file: 29, line: 37, column: 11, end_line: 37, end_column: 55 } },
        /* terrane-site-row: site 190: /text-families::test-scenario (tests/integration/text-families/case.trn:38:11-38:54) */
        { Site { function: 35, file: 29, line: 38, column: 11, end_line: 38, end_column: 54 } },
        /* terrane-site-row: site 191: /text-families::test-scenario (tests/integration/text-families/case.trn:41:13-41:60) */
        { Site { function: 35, file: 29, line: 41, column: 13, end_line: 41, end_column: 60 } },
        /* terrane-site-row: site 192: /tuple-type-boundaries::test-scenario (tests/integration/tuple-type-boundaries/case.trn:13:27-13:38) */
        { Site { function: 36, file: 30, line: 13, column: 27, end_line: 13, end_column: 38 } },
        /* terrane-site-row: site 193: /tuple-type-boundaries::test-scenario (tests/integration/tuple-type-boundaries/case.trn:13:40-13:51) */
        { Site { function: 36, file: 30, line: 13, column: 40, end_line: 13, end_column: 51 } },
        /* terrane-site-row: site 194: /tuple-type-boundaries::test-scenario (tests/integration/tuple-type-boundaries/case.trn:18:25-18:34) */
        { Site { function: 36, file: 30, line: 18, column: 25, end_line: 18, end_column: 34 } },
        /* terrane-site-row: site 195: /tuple-type-boundaries::test-scenario (tests/integration/tuple-type-boundaries/case.trn:18:25-18:37) */
        { Site { function: 36, file: 30, line: 18, column: 25, end_line: 18, end_column: 37 } },
        /* terrane-site-row: site 196: /tuple-type-boundaries::test-scenario (tests/integration/tuple-type-boundaries/case.trn:18:39-18:48) */
        { Site { function: 36, file: 30, line: 18, column: 39, end_line: 18, end_column: 48 } },
        /* terrane-site-row: site 197: /tuple-type-boundaries::test-scenario (tests/integration/tuple-type-boundaries/case.trn:18:39-18:51) */
        { Site { function: 36, file: 30, line: 18, column: 39, end_line: 18, end_column: 51 } },
        /* terrane-site-row: site 198: /typed-catch-bindings::test-scenario (tests/integration/typed-catch-bindings/case.trn:7:7-7:27) */
        { Site { function: 37, file: 31, line: 7, column: 7, end_line: 7, end_column: 27 } },
        /* terrane-site-row: site 199: /typed-catch-bindings::test-scenario (tests/integration/typed-catch-bindings/case.trn:11:7-11:32) */
        { Site { function: 37, file: 31, line: 11, column: 7, end_line: 11, end_column: 32 } },
        /* terrane-site-row: site 200: /typed-catch-bindings::test-scenario (tests/integration/typed-catch-bindings/case.trn:17:7-17:27) */
        { Site { function: 37, file: 31, line: 17, column: 7, end_line: 17, end_column: 27 } },
        /* terrane-site-row: site 201: /typed-catch-bindings::test-scenario (tests/integration/typed-catch-bindings/case.trn:19:7-19:19) */
        { Site { function: 37, file: 31, line: 19, column: 7, end_line: 19, end_column: 19 } },
        /* terrane-site-row: site 202: /typed-catch-bindings::test-scenario (tests/integration/typed-catch-bindings/case.trn:24:5-24:30) */
        { Site { function: 37, file: 31, line: 24, column: 5, end_line: 24, end_column: 30 } },
        /* terrane-site-row: site 203: /typed-document-decoding::test-scenario (tests/integration/typed-document-decoding/case.trn:38:12-38:34) */
        { Site { function: 38, file: 32, line: 38, column: 12, end_line: 38, end_column: 34 } },
        /* terrane-site-row: site 204: /typed-document-decoding::test-scenario (tests/integration/typed-document-decoding/case.trn:38:36-38:64) */
        { Site { function: 38, file: 32, line: 38, column: 36, end_line: 38, end_column: 64 } },
        /* terrane-site-row: site 205: /typed-document-decoding::test-scenario (tests/integration/typed-document-decoding/case.trn:38:66-38:94) */
        { Site { function: 38, file: 32, line: 38, column: 66, end_line: 38, end_column: 94 } },
        /* terrane-site-row: site 206: /typed-document-decoding::test-scenario (tests/integration/typed-document-decoding/case.trn:44:13-44:37) */
        { Site { function: 38, file: 32, line: 44, column: 13, end_line: 44, end_column: 37 } },
        /* terrane-site-row: site 207: /typed-document-decoding::test-scenario (tests/integration/typed-document-decoding/case.trn:44:69-44:93) */
        { Site { function: 38, file: 32, line: 44, column: 69, end_line: 44, end_column: 93 } },
        /* terrane-site-row: site 208: /typed-document-decoding::test-scenario (tests/integration/typed-document-decoding/case.trn:47:28-47:50) */
        { Site { function: 38, file: 32, line: 47, column: 28, end_line: 47, end_column: 50 } },
        /* terrane-site-row: site 209: /typed-document-decoding::test-scenario (tests/integration/typed-document-decoding/case.trn:47:59-47:81) */
        { Site { function: 38, file: 32, line: 47, column: 59, end_line: 47, end_column: 81 } },
        /* terrane-site-row: site 210: /typed-document-decoding::test-scenario (tests/integration/typed-document-decoding/case.trn:56:48-56:71) */
        { Site { function: 38, file: 32, line: 56, column: 48, end_line: 56, end_column: 71 } },
        /* terrane-site-row: site 211: /typed-json-boundaries::unknown-policy (tests/integration/typed-json-boundaries/case.trn:13:5-13:33) */
        { Site { function: 39, file: 33, line: 13, column: 5, end_line: 13, end_column: 33 } },
        /* terrane-site-row: site 212: /typed-json-boundaries::test-scenario (tests/integration/typed-json-boundaries/case.trn:52:33-52:57) */
        { Site { function: 40, file: 33, line: 52, column: 33, end_line: 52, end_column: 57 } },
        /* terrane-site-row: site 213: /typed-json-boundaries::test-scenario (tests/integration/typed-json-boundaries/case.trn:52:100-52:123) */
        { Site { function: 40, file: 33, line: 52, column: 100, end_line: 52, end_column: 123 } },
        /* terrane-site-row: site 214: /typed-json-boundaries::test-scenario (tests/integration/typed-json-boundaries/case.trn:52:166-52:189) */
        { Site { function: 40, file: 33, line: 52, column: 166, end_line: 52, end_column: 189 } },
        /* terrane-site-row: site 215: /typed-json-boundaries::test-scenario (tests/integration/typed-json-boundaries/case.trn:52:233-52:257) */
        { Site { function: 40, file: 33, line: 52, column: 233, end_line: 52, end_column: 257 } },
        /* terrane-site-row: site 216: /typed-json-boundaries::test-scenario (tests/integration/typed-json-boundaries/case.trn:59:69-59:84) */
        { Site { function: 40, file: 33, line: 59, column: 69, end_line: 59, end_column: 84 } },
        /* terrane-site-row: site 217: /typed-json-boundaries::test-scenario (tests/integration/typed-json-boundaries/case.trn:60:16-60:38) */
        { Site { function: 40, file: 33, line: 60, column: 16, end_line: 60, end_column: 38 } },
        /* terrane-site-row: site 218: /typed-serialization-destinations::test-destinations (tests/integration/typed-serialization-destinations/case.trn:11:33-11:49) */
        { Site { function: 41, file: 34, line: 11, column: 33, end_line: 11, end_column: 49 } },
        /* terrane-site-row: site 219: /typed-serialization-destinations::test-destinations (tests/integration/typed-serialization-destinations/case.trn:11:56-11:72) */
        { Site { function: 41, file: 34, line: 11, column: 56, end_line: 11, end_column: 72 } },
        /* terrane-site-row: site 220: /typed-serialization-destinations::test-destinations (tests/integration/typed-serialization-destinations/case.trn:13:30-13:48) */
        { Site { function: 41, file: 34, line: 13, column: 30, end_line: 13, end_column: 48 } },
        /* terrane-site-row: site 221: /typed-serialization-destinations::test-destinations (tests/integration/typed-serialization-destinations/case.trn:15:30-15:43) */
        { Site { function: 41, file: 34, line: 15, column: 30, end_line: 15, end_column: 43 } },
        /* terrane-site-row: site 222: /typed-serialization-destinations::test-destinations (tests/integration/typed-serialization-destinations/case.trn:15:50-15:63) */
        { Site { function: 41, file: 34, line: 15, column: 50, end_line: 15, end_column: 63 } },
        /* terrane-site-row: site 223: /typed-serialization-destinations::test-destinations (tests/integration/typed-serialization-destinations/case.trn:17:31-17:45) */
        { Site { function: 41, file: 34, line: 17, column: 31, end_line: 17, end_column: 45 } },
        /* terrane-site-row: site 224: /typed-serialization-destinations::test-destinations (tests/integration/typed-serialization-destinations/case.trn:17:52-17:66) */
        { Site { function: 41, file: 34, line: 17, column: 52, end_line: 17, end_column: 66 } },
        /* terrane-site-row: site 225: /typed-serialization-matrix::test-matrix (tests/integration/typed-serialization-matrix/case.trn:39:91-39:115) */
        { Site { function: 42, file: 35, line: 39, column: 91, end_line: 39, end_column: 115 } },
        /* terrane-site-row: site 226: /typed-serialization-matrix::test-matrix (tests/integration/typed-serialization-matrix/case.trn:39:122-39:149) */
        { Site { function: 42, file: 35, line: 39, column: 122, end_line: 39, end_column: 149 } },
        /* terrane-site-row: site 227: /typed-serialization-matrix::test-matrix (tests/integration/typed-serialization-matrix/case.trn:39:156-39:177) */
        { Site { function: 42, file: 35, line: 39, column: 156, end_line: 39, end_column: 177 } },
        /* terrane-site-row: site 228: /typed-serialization-matrix::test-matrix (tests/integration/typed-serialization-matrix/case.trn:43:90-43:112) */
        { Site { function: 42, file: 35, line: 43, column: 90, end_line: 43, end_column: 112 } },
        /* terrane-site-row: site 229: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:11:33-11:54) */
        { Site { function: 43, file: 36, line: 11, column: 33, end_line: 11, end_column: 54 } },
        /* terrane-site-row: site 230: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:11:61-11:82) */
        { Site { function: 43, file: 36, line: 11, column: 61, end_line: 11, end_column: 82 } },
        /* terrane-site-row: site 231: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:11:61-11:85) */
        { Site { function: 43, file: 36, line: 11, column: 61, end_line: 11, end_column: 85 } },
        /* terrane-site-row: site 232: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:14:61-14:80) */
        { Site { function: 43, file: 36, line: 14, column: 61, end_line: 14, end_column: 80 } },
        /* terrane-site-row: site 233: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:17:32-17:47) */
        { Site { function: 43, file: 36, line: 17, column: 32, end_line: 17, end_column: 47 } },
        /* terrane-site-row: site 234: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:19:31-19:51) */
        { Site { function: 43, file: 36, line: 19, column: 31, end_line: 19, end_column: 51 } },
        /* terrane-site-row: site 235: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:19:63-19:83) */
        { Site { function: 43, file: 36, line: 19, column: 63, end_line: 19, end_column: 83 } },
        /* terrane-site-row: site 236: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:21:34-21:51) */
        { Site { function: 43, file: 36, line: 21, column: 34, end_line: 21, end_column: 51 } },
        /* terrane-site-row: site 237: /typed-tuple-decode-shapes::test-scenario (tests/integration/typed-tuple-decode-shapes/case.trn:21:34-21:54) */
        { Site { function: 43, file: 36, line: 21, column: 34, end_line: 21, end_column: 54 } },
        /* terrane-site-row: site 238: /core/documents::make-document-list (core/documents.trn:141:47-141:60) */
        { Site { function: 44, file: 37, line: 141, column: 47, end_line: 141, end_column: 60 } },
        /* terrane-site-row: site 239: /core/documents::mapping-required-fields (core/documents.trn:154:17-154:30) */
        { Site { function: 45, file: 37, line: 154, column: 17, end_line: 154, end_column: 30 } },
        /* terrane-site-row: site 240: /core/documents::mapping-required-fields (core/documents.trn:158:16-158:47) */
        { Site { function: 45, file: 37, line: 158, column: 16, end_line: 158, end_column: 47 } },
        /* terrane-site-row: site 241: /core/documents::mapping-required-fields (core/documents.trn:164:16-164:45) */
        { Site { function: 45, file: 37, line: 164, column: 16, end_line: 164, end_column: 45 } },
        /* terrane-site-row: site 242: /core/documents::decode-document (core/documents.trn:177:12-177:44) */
        { Site { function: 46, file: 37, line: 177, column: 12, end_line: 177, end_column: 44 } },
        /* terrane-site-row: site 243: /core/documents::decode-document (core/documents.trn:178:37-178:69) */
        { Site { function: 46, file: 37, line: 178, column: 37, end_line: 178, end_column: 69 } },
        /* terrane-site-row: site 244: /core/documents::decode-document (core/documents.trn:184:12-184:49) */
        { Site { function: 46, file: 37, line: 184, column: 12, end_line: 184, end_column: 49 } },
        /* terrane-site-row: site 245: /core/documents::decode-document (core/documents.trn:185:36-185:73) */
        { Site { function: 46, file: 37, line: 185, column: 36, end_line: 185, end_column: 73 } },
        /* terrane-site-row: site 246: /core/documents::decode-document (core/documents.trn:186:36-186:73) */
        { Site { function: 46, file: 37, line: 186, column: 36, end_line: 186, end_column: 73 } },
        /* terrane-site-row: site 247: /core/urls::get (core/urls.trn:23:16-23:32) */
        { Site { function: 47, file: 38, line: 23, column: 16, end_line: 23, end_column: 32 } },
        /* terrane-site-row: site 248: /core/urls::get (core/urls.trn:24:24-24:42) */
        { Site { function: 47, file: 38, line: 24, column: 24, end_line: 24, end_column: 42 } },
        /* terrane-site-row: site 249: /core/urls::get-all (core/urls.trn:32:16-32:32) */
        { Site { function: 48, file: 38, line: 32, column: 16, end_line: 32, end_column: 32 } },
        /* terrane-site-row: site 250: /core/urls::get-all (core/urls.trn:33:32-33:50) */
        { Site { function: 48, file: 38, line: 33, column: 32, end_line: 33, end_column: 50 } },
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
static TERRANE_SHARED_NEXT_THREAD_TOKEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(
    1,
);
std::thread_local! {
    static TERRANE_SHARED_THREAD_TOKEN : u64 = TERRANE_SHARED_NEXT_THREAD_TOKEN
    .try_update(std::sync::atomic::Ordering::Relaxed,
    std::sync::atomic::Ordering::Relaxed, | next | next.checked_add(1),)
    .expect("shared storage thread token space exhausted");
}
#[derive(Debug)]
pub struct TerraneSharedCell<T> {
    owner: std::sync::atomic::AtomicU64,
    value: std::sync::Mutex<T>,
}
impl<T> TerraneSharedCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            owner: std::sync::atomic::AtomicU64::new(0),
            value: std::sync::Mutex::new(value),
        }
    }
    /// Acquires exclusive shared-storage access, waiting for other threads.
    ///
    /// # Panics
    /// Panics on same-thread reentry, before attempting to wait on itself.
    ///
    /// # Errors
    /// Returns a poisoned guard if a previous holder unwound while borrowing.
    pub fn lock(&self) -> std::sync::LockResult<TerraneSharedGuard<'_, T>> {
        let token = TERRANE_SHARED_THREAD_TOKEN.with(|token| *token);
        assert!(
            self.owner.load(std::sync::atomic::Ordering::Acquire) != token,
            "shared reference borrow conflict: storage is already borrowed by this thread"
        );
        let (guard, poisoned) = match self.value.lock() {
            Ok(guard) => (guard, false),
            Err(error) => (error.into_inner(), true),
        };
        self.owner.store(token, std::sync::atomic::Ordering::Release);
        let wrapped = TerraneSharedGuard {
            owner: &self.owner,
            guard,
        };
        if poisoned { Err(std::sync::PoisonError::new(wrapped)) } else { Ok(wrapped) }
    }
}
#[derive(Debug)]
pub struct TerraneSharedGuard<'a, T> {
    owner: &'a std::sync::atomic::AtomicU64,
    guard: std::sync::MutexGuard<'a, T>,
}
impl<T> std::ops::Deref for TerraneSharedGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.guard
    }
}
impl<T> std::ops::DerefMut for TerraneSharedGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}
impl<T> Drop for TerraneSharedGuard<'_, T> {
    fn drop(&mut self) {
        self.owner.store(0, std::sync::atomic::Ordering::Release);
    }
}
pub struct TerraneStaticObject<T> {
    current: TerraneSharedCell<std::sync::Arc<TerraneSharedCell<T>>>,
}
impl<T> TerraneStaticObject<T> {
    pub fn new(value: T) -> Self {
        Self {
            current: TerraneSharedCell::new(
                std::sync::Arc::new(TerraneSharedCell::new(value)),
            ),
        }
    }
    /// Captures the current object without retaining the replaceable slot guard.
    ///
    /// # Panics
    /// Panics if the slot is poisoned or is recursively borrowed by this thread.
    pub fn handle(&self) -> std::sync::Arc<TerraneSharedCell<T>> {
        self.current.lock().expect("static object slot lock poisoned").clone()
    }
    /// Installs a new object, releasing the slot before dropping its predecessor.
    ///
    /// # Panics
    /// Panics if the slot is poisoned, recursively borrowed, or its old object panics on drop.
    pub fn replace(&self, value: T) {
        let replacement = std::sync::Arc::new(TerraneSharedCell::new(value));
        let previous = {
            let mut current = self
                .current
                .lock()
                .expect("static object slot lock poisoned");
            std::mem::replace(&mut *current, replacement)
        };
        drop(previous);
    }
}
pub trait TerraneSharedHandle<T> {
    /// Acquires a bounded exclusive borrow of the receiver's live storage.
    ///
    /// # Errors
    /// Returns the original cell's poison state with a guard to the selected value.
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>>;
}
impl<T> TerraneSharedHandle<T> for std::sync::Arc<TerraneSharedCell<T>> {
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>> {
        (**self).lock()
    }
}
pub struct TerraneSharedProjection<H, U, T> {
    handle: H,
    read: fn(&U) -> &T,
    write: fn(&mut U) -> &mut T,
}
impl<H, U, T> TerraneSharedProjection<H, U, T> {
    pub fn new(handle: H, read: fn(&U) -> &T, write: fn(&mut U) -> &mut T) -> Self {
        Self { handle, read, write }
    }
}
impl<H: Clone, U, T> Clone for TerraneSharedProjection<H, U, T> {
    fn clone(&self) -> Self {
        Self::new(self.handle.clone(), self.read, self.write)
    }
}
struct TerraneSharedProjectedGuard<G, U, T> {
    guard: G,
    read: fn(&U) -> &T,
    write: fn(&mut U) -> &mut T,
}
impl<G: std::ops::Deref<Target = U>, U, T> std::ops::Deref
for TerraneSharedProjectedGuard<G, U, T> {
    type Target = T;
    fn deref(&self) -> &T {
        (self.read)(&self.guard)
    }
}
impl<G: std::ops::DerefMut<Target = U>, U, T> std::ops::DerefMut
for TerraneSharedProjectedGuard<G, U, T> {
    fn deref_mut(&mut self) -> &mut T {
        (self.write)(&mut self.guard)
    }
}
impl<H: TerraneSharedHandle<U>, U, T> TerraneSharedHandle<T>
for TerraneSharedProjection<H, U, T> {
    fn lock(&self) -> std::sync::LockResult<impl std::ops::DerefMut<Target = T>> {
        let (guard, poisoned) = match self.handle.lock() {
            Ok(guard) => (guard, false),
            Err(error) => (error.into_inner(), true),
        };
        let guard = TerraneSharedProjectedGuard {
            guard,
            read: self.read,
            write: self.write,
        };
        if poisoned { Err(std::sync::PoisonError::new(guard)) } else { Ok(guard) }
    }
}
pub(crate) static TERRANE_TEST_TIME_NANOS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(
    0,
);
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_domain() -> terrane_int_support::Int {
    terrane_int_support::Int::from(2_i128)
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_monotonic() -> terrane_int_support::Int {
    terrane_int_support::Int::from_u128(
        u128::from(TERRANE_TEST_TIME_NANOS.load(std::sync::atomic::Ordering::Acquire)),
    )
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_deadline_expired(target: &terrane_int_support::Int) -> bool {
    terrane_time_monotonic() >= target.clone()
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
async fn terrane_time_sleep_until(target: terrane_int_support::Int) {
    let target = terrane_int_support::checked_coerce::<u64>(&target)
        .expect("controlled test clock target exceeds the host clock range");
    TERRANE_TEST_TIME_NANOS.fetch_max(target, std::sync::atomic::Ordering::AcqRel);
}
#[allow(dead_code, reason = "time support is shared by task scopes and clock packages")]
fn terrane_time_sleep_after(
    elapsed: terrane_int_support::Int,
) -> impl Future<Output = ()> {
    let target = terrane_time_monotonic() + elapsed;
    async move {
        terrane_time_sleep_until(target).await;
    }
}
pub fn terrane_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(0)
}
pub fn terrane_index(value: &terrane_int_support::Int) -> Option<usize> {
    value.as_usize()
}
pub fn terrane_empty_document() -> terrane_document_support::DataResult {
    terrane_document_support::parse_json("null", 0, 4)
}
pub fn terrane_make_document_none() -> terrane_document_support::DataResult {
    terrane_document_support::document_none()
}
pub fn terrane_make_document_bool(value: bool) -> terrane_document_support::DataResult {
    terrane_document_support::document_bool(value)
}
pub fn terrane_make_document_string(
    value: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_string(value)
}
pub fn terrane_make_document_integer(
    value: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_integer(&value)
}
pub fn terrane_make_document_decimal(
    value: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_decimal(&value)
}
pub fn terrane_make_document_list() -> terrane_document_support::DataResult {
    terrane_document_support::document_list()
}
pub fn terrane_document_list_append(
    list: &terrane_document_support::DataResult,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_list_append(list, value)
}
pub fn terrane_make_document_map() -> terrane_document_support::DataResult {
    terrane_document_support::document_map()
}
pub fn terrane_document_map_insert(
    map: &terrane_document_support::DataResult,
    key: String,
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_map_insert(map, key, value)
}
pub fn terrane_data_failed(result: &terrane_document_support::DataResult) -> bool {
    result.failed
}
pub fn terrane_data_message(result: &terrane_document_support::DataResult) -> String {
    result.message.clone()
}
pub fn terrane_data_path(result: &terrane_document_support::DataResult) -> String {
    result.path.clone()
}
pub fn terrane_data_expected(result: &terrane_document_support::DataResult) -> String {
    result.expected.clone()
}
pub fn terrane_data_encoded(result: &terrane_document_support::DataResult) -> String {
    result.encoded.clone()
}
pub fn terrane_document_kind(result: &terrane_document_support::DataResult) -> String {
    terrane_document_support::document_kind(result)
}
pub fn terrane_document_text(result: &terrane_document_support::DataResult) -> String {
    terrane_document_support::document_text(result)
}
pub fn terrane_document_coefficient(
    result: &terrane_document_support::DataResult,
) -> String {
    terrane_document_support::document_coefficient(result)
}
pub fn terrane_document_exponent(
    result: &terrane_document_support::DataResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(terrane_document_support::document_exponent(result))
}
pub fn terrane_document_length(
    result: &terrane_document_support::DataResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        i128::try_from(terrane_document_support::document_length(result))
            .expect("document length fits in i128"),
    )
}
pub fn terrane_document_item(
    result: &terrane_document_support::DataResult,
    index: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_index(&index)
        .map_or_else(
            || terrane_document_support::invalid_document_index(),
            |index| terrane_document_support::document_item(result, index),
        )
}
pub fn terrane_document_key(
    result: &terrane_document_support::DataResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_index(&index)
        .map_or_else(
            String::new,
            |index| terrane_document_support::document_key(result, index),
        )
}
pub fn terrane_document_field(
    result: &terrane_document_support::DataResult,
    key: String,
) -> terrane_document_support::DataResult {
    terrane_document_support::document_field(result, &key)
}
pub fn terrane_string_list(
    value: terrane_collection_support::List<String>,
) -> Vec<String> {
    value.into_iter().collect()
}
pub fn terrane_validate_mapping(
    result: &terrane_document_support::DataResult,
    expected_kind: String,
    required_fields: terrane_collection_support::List<String>,
    declared_fields: terrane_collection_support::List<String>,
    default_fields: terrane_collection_support::List<String>,
    default_values: terrane_collection_support::List<String>,
    allow_unknown: bool,
) -> terrane_document_support::DataResult {
    let required_fields = terrane_string_list(required_fields);
    let declared_fields = terrane_string_list(declared_fields);
    let default_fields = terrane_string_list(default_fields);
    let default_values = terrane_string_list(default_values);
    terrane_document_support::validate_mapping(
        result,
        &expected_kind,
        &required_fields,
        &declared_fields,
        &default_fields,
        &default_values,
        allow_unknown,
    )
}
pub fn terrane_json_parse(
    input: String,
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_document_support::parse_json(
        &input,
        terrane_limit(&max_depth),
        terrane_limit(&max_bytes),
    )
}
pub fn terrane_json_canonical(
    value: &terrane_document_support::DataResult,
) -> terrane_document_support::DataResult {
    terrane_document_support::canonical_json(value)
}
pub fn terrane_json_stringify(
    value: &terrane_document_support::DataResult,
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_document_support::stringify_json_bounded(
        value,
        terrane_limit(&max_depth),
        terrane_limit(&max_bytes),
    )
}
pub fn terrane_yaml_parse(
    input: String,
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
    max_aliases: terrane_int_support::Int,
) -> terrane_document_support::DataResult {
    terrane_document_support::parse_yaml(
        &input,
        terrane_limit(&max_depth),
        terrane_limit(&max_bytes),
        terrane_limit(&max_aliases),
    )
}
pub fn terrane_url_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(usize::MAX)
}
pub fn terrane_url_parse(
    input: String,
    base: String,
) -> terrane_document_support::UrlResult {
    terrane_document_support::parse_url(&input, &base)
}
pub fn terrane_url_failed(result: &terrane_document_support::UrlResult) -> bool {
    result.failed
}
pub fn terrane_url_message(result: &terrane_document_support::UrlResult) -> String {
    result.message.clone()
}
pub fn terrane_url_serialized(result: &terrane_document_support::UrlResult) -> String {
    result.serialized.clone()
}
pub fn terrane_url_display(result: &terrane_document_support::UrlResult) -> String {
    result.display.clone()
}
pub fn terrane_url_scheme(result: &terrane_document_support::UrlResult) -> String {
    result.scheme.clone()
}
pub fn terrane_url_username(result: &terrane_document_support::UrlResult) -> String {
    result.username.clone()
}
pub fn terrane_url_password(result: &terrane_document_support::UrlResult) -> String {
    result.password.clone()
}
pub fn terrane_url_host(result: &terrane_document_support::UrlResult) -> String {
    result.host.clone()
}
pub fn terrane_url_port(result: &terrane_document_support::UrlResult) -> String {
    result.port.clone()
}
pub fn terrane_url_path(result: &terrane_document_support::UrlResult) -> String {
    result.path.clone()
}
pub fn terrane_url_query_length(
    result: &terrane_document_support::UrlResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        i128::try_from(terrane_document_support::url_query_length(result))
            .expect("query length fits in i128"),
    )
}
pub fn terrane_url_query_key(
    result: &terrane_document_support::UrlResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_document_support::url_query_key(result, terrane_url_limit(&index))
}
pub fn terrane_url_query_value(
    result: &terrane_document_support::UrlResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_document_support::url_query_value(result, terrane_url_limit(&index))
}
pub fn terrane_url_fragment(result: &terrane_document_support::UrlResult) -> String {
    result.fragment.clone()
}
pub fn terrane_url_origin(result: &terrane_document_support::UrlResult) -> String {
    result.origin.clone()
}
#[derive(Clone)]
pub struct TerraneDocumentDiagnostic {
    pub path: String,
    pub expected: String,
    pub actual_kind: String,
    pub reason: &'static str,
    pub message: String,
    pub source: String,
    pub field_source: String,
}
#[derive(Clone)]
pub struct TerraneDocumentDecodeOutcome<T: Clone> {
    pub value: T,
    pub diagnostics: terrane_collection_support::List<TerraneDocumentDiagnostic>,
}
fn __terrane_document_diagnostic(
    path: &impl std::fmt::Display,
    expected: impl Into<String>,
    actual_kind: impl Into<String>,
    reason: &'static str,
    message: impl Into<String>,
    source: &str,
    field_source: &str,
) -> TerraneDocumentDiagnostic {
    TerraneDocumentDiagnostic {
        path: path.to_string(),
        expected: expected.into(),
        actual_kind: actual_kind.into(),
        reason,
        message: message.into(),
        source: source.to_owned(),
        field_source: field_source.to_owned(),
    }
}
enum TerraneDocumentPath<'a> {
    Root(&'a str),
    Field(&'a TerraneDocumentPath<'a>, &'a str),
    MapKey(&'a TerraneDocumentPath<'a>, &'a str),
    Index(&'a TerraneDocumentPath<'a>, usize),
}
impl std::fmt::Display for TerraneDocumentPath<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root(path) => formatter.write_str(path),
            Self::Field(parent, key) => {
                if !key.is_empty()
                    && key
                        .chars()
                        .all(|character| {
                            character.is_ascii_alphanumeric() || character == '_'
                                || character == '-'
                        })
                {
                    write!(formatter, "{parent}.{key}")
                } else {
                    write!(formatter, "{parent}[{key:?}]")
                }
            }
            Self::MapKey(parent, key) => write!(formatter, "{parent}.{key}"),
            Self::Index(parent, index) => write!(formatter, "{parent}[{index}]"),
        }
    }
}
struct TerraneDocumentShape {
    tuple_length: Option<usize>,
    inner: Option<&'static TerraneDocumentShape>,
}
impl TerraneDocumentShape {
    fn inner(&self) -> &Self {
        self.inner.unwrap_or(self)
    }
}
trait TerraneDocumentDecode: Sized {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
        shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>>;
}
fn __terrane_decode_document<T: Clone + TerraneDocumentDecode>(
    parsed: Result<
        terrane_document_support::Document,
        terrane_document_support::DataResult,
    >,
    fallback: impl FnOnce() -> T,
    allow_unknown: bool,
    source: &str,
    expected: &str,
    shape: &TerraneDocumentShape,
) -> TerraneDocumentDecodeOutcome<T> {
    let path = TerraneDocumentPath::Root("$");
    let decoded = match parsed {
        Ok(mut input) => {
            T::terrane_decode_document(
                &mut input,
                &path,
                allow_unknown,
                source,
                source,
                shape,
            )
        }
        Err(error) => {
            Err(
                vec![
                    __terrane_document_diagnostic(&path, expected, "invalid", "parse",
                    error.message, source, source)
                ],
            )
        }
    };
    match decoded {
        Ok(value) => {
            TerraneDocumentDecodeOutcome {
                value,
                diagnostics: terrane_collection_support::List::new(Vec::new()),
            }
        }
        Err(diagnostics) => {
            TerraneDocumentDecodeOutcome {
                value: fallback(),
                diagnostics: terrane_collection_support::List::new(diagnostics),
            }
        }
    }
}
fn __terrane_document_kind(input: &terrane_document_support::Document) -> &'static str {
    use terrane_document_support::Document;
    match input {
        Document::None => "none",
        Document::Bool(_) => "bool",
        Document::Integer(_) => "integer",
        Document::Decimal(_) => "decimal",
        Document::String(_) => "string",
        Document::List(_) => "list",
        Document::Map(_) => "map",
    }
}
fn __terrane_document_child_path<'a>(
    path: &'a TerraneDocumentPath<'a>,
    key: &'a str,
) -> TerraneDocumentPath<'a> {
    __terrane_document_entry_path(path, key, true)
}
fn __terrane_document_entry_path<'a>(
    path: &'a TerraneDocumentPath<'a>,
    key: &'a str,
    class_field: bool,
) -> TerraneDocumentPath<'a> {
    if class_field {
        TerraneDocumentPath::Field(path, key)
    } else {
        TerraneDocumentPath::MapKey(path, key)
    }
}
fn __terrane_document_type_error<T>(
    input: &terrane_document_support::Document,
    path: &TerraneDocumentPath<'_>,
    expected: &str,
    source: &str,
    field_source: &str,
) -> Result<T, Vec<TerraneDocumentDiagnostic>> {
    Err(
        vec![
            __terrane_document_diagnostic(path, expected, __terrane_document_kind(input),
            "type-mismatch", format!("expected {expected}"), source, field_source,)
        ],
    )
}
impl TerraneDocumentDecode for String {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if let terrane_document_support::Document::String(value) = input {
            Ok(std::mem::take(value))
        } else {
            __terrane_document_type_error(input, path, "string", source, field_source)
        }
    }
}
impl TerraneDocumentDecode for () {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if matches!(input, terrane_document_support::Document::None) {
            Ok(())
        } else {
            __terrane_document_type_error(input, path, "none", source, field_source)
        }
    }
}
impl TerraneDocumentDecode for bool {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if let terrane_document_support::Document::Bool(value) = input {
            Ok(*value)
        } else {
            __terrane_document_type_error(input, path, "bool", source, field_source)
        }
    }
}
macro_rules! __terrane_document_float {
    ($type:ty) => {
        impl TerraneDocumentDecode for $type { fn terrane_decode_document(input : & mut
        terrane_document_support::Document, path : & TerraneDocumentPath <'_ >, _ : bool,
        source : & str, field_source : & str, _shape : & TerraneDocumentShape,) -> Result
        < Self, Vec < TerraneDocumentDiagnostic >> { use
        terrane_document_support::Document; let text = match input {
        Document::Integer(text) | Document::Decimal(text) => text, _ => { return
        __terrane_document_type_error(input, path, stringify!($type), source,
        field_source,); } }; text.parse::<$type > ().ok().filter(| value | value
        .is_finite()).ok_or_else(|| { vec![__terrane_document_diagnostic(path,
        stringify!($type), __terrane_document_kind(input), "numeric-conversion",
        "document number is outside the finite range of the destination float", source,
        field_source,)] }) } }
    };
}
__terrane_document_float!(f32);
__terrane_document_float!(f64);
fn __terrane_document_integer_parts(
    text: &str,
) -> Option<(std::borrow::Cow<'_, str>, u32)> {
    let (coefficient, exponent) = text.split_once('e')?;
    let fraction_length = coefficient
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    let zeros = exponent
        .parse::<i64>()
        .ok()?
        .checked_sub(i64::try_from(fraction_length).ok()?)?;
    let coefficient = if fraction_length == 0 {
        std::borrow::Cow::Borrowed(coefficient)
    } else {
        std::borrow::Cow::Owned(coefficient.replace('.', ""))
    };
    let zeros = u32::try_from(zeros).ok()?;
    if coefficient
        .trim_start_matches('-')
        .len()
        .checked_add(usize::try_from(zeros).ok()?)?
        > terrane_document_support::MAXIMUM_NUMBER_DIGITS
    {
        return None;
    }
    Some((coefficient, zeros))
}
impl TerraneDocumentDecode for terrane_int_support::Int {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        _: bool,
        source: &str,
        field_source: &str,
        _shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::Integer(text) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "integer",
                source,
                field_source,
            );
        };
        let decoded = text
            .parse::<i128>()
            .ok()
            .map(terrane_int_support::Int::from)
            .or_else(|| {
                let radix = terrane_int_support::Int::from(10_i128);
                if let Some((coefficient, mut zeros)) = __terrane_document_integer_parts(
                    text,
                ) {
                    let mut value = terrane_int_support::parse_radix(
                            &coefficient,
                            &radix,
                        )
                        .ok()?;
                    let mut factor = radix;
                    while zeros != 0 {
                        if zeros & 1 != 0 {
                            value = value * factor.clone();
                        }
                        zeros >>= 1;
                        if zeros != 0 {
                            factor = factor.clone() * factor;
                        }
                    }
                    Some(value)
                } else {
                    terrane_int_support::parse_radix(text, &radix).ok()
                }
            });
        decoded
            .ok_or_else(|| {
                vec![
                    __terrane_document_diagnostic(path, "int", "integer",
                    "numeric-conversion", "integer is outside the supported exact range",
                    source, field_source,)
                ]
            })
    }
}
macro_rules! __terrane_fixed_document_integer {
    ($($type:ty),+ $(,)?) => {
        $(impl TerraneDocumentDecode for $type { fn terrane_decode_document(input : & mut
        terrane_document_support::Document, path : & TerraneDocumentPath <'_ >, _ : bool,
        source : & str, field_source : & str, _shape : & TerraneDocumentShape,) -> Result
        < Self, Vec < TerraneDocumentDiagnostic >> { let
        terrane_document_support::Document::Integer(text) = input else { return
        __terrane_document_type_error(input, path, stringify!($type), source,
        field_source); }; let decoded = text.parse::<$type > ().ok().or_else(|| { let
        (coefficient, zeros) = __terrane_document_integer_parts(text) ?; let value =
        coefficient.parse::<$type > ().ok() ?; let ten : $type = 10; value
        .checked_mul(ten.checked_pow(zeros) ?) }); decoded.ok_or_else(||
        vec![__terrane_document_diagnostic(path, stringify!($type), "integer",
        "numeric-conversion", "integer is outside the destination range", source,
        field_source,)]) } })+
    };
}
__terrane_fixed_document_integer!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
impl<T: TerraneDocumentDecode> TerraneDocumentDecode for Option<T> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
        shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if matches!(input, terrane_document_support::Document::None) {
            Ok(None)
        } else {
            T::terrane_decode_document(
                    input,
                    path,
                    allow_unknown,
                    source,
                    field_source,
                    shape.inner(),
                )
                .map(Some)
        }
    }
}
impl<T: TerraneDocumentDecode> TerraneDocumentDecode
for terrane_collection_support::List<T> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
        shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        let terrane_document_support::Document::List(items) = input else {
            return __terrane_document_type_error(
                input,
                path,
                "list",
                source,
                field_source,
            );
        };
        let mut values = Vec::with_capacity(items.len());
        let mut diagnostics = Vec::new();
        for (index, item) in items.iter_mut().enumerate() {
            let item_path = TerraneDocumentPath::Index(path, index);
            match T::terrane_decode_document(
                item,
                &item_path,
                allow_unknown,
                source,
                field_source,
                shape.inner(),
            ) {
                Ok(value) => values.push(value),
                Err(mut errors) => diagnostics.append(&mut errors),
            }
        }
        if diagnostics.is_empty() {
            Ok(terrane_collection_support::List::new(values))
        } else {
            Err(diagnostics)
        }
    }
}
impl<T: Clone + TerraneDocumentDecode> TerraneDocumentDecode
for terrane_collection_support::Tuple<T> {
    fn terrane_decode_document(
        input: &mut terrane_document_support::Document,
        path: &TerraneDocumentPath<'_>,
        allow_unknown: bool,
        source: &str,
        field_source: &str,
        shape: &TerraneDocumentShape,
    ) -> Result<Self, Vec<TerraneDocumentDiagnostic>> {
        if let Some(length) = shape.tuple_length
            && matches!(
                & input, terrane_document_support::Document::List(items) if items.len()
                != length
            )
        {
            return Err(
                vec![
                    __terrane_document_diagnostic(path, "tuple", "list", "tuple-length",
                    format!("expected exactly {length} tuple items"), source,
                    field_source,)
                ],
            );
        }
        let values = terrane_collection_support::List::<
            T,
        >::terrane_decode_document(
            input,
            path,
            allow_unknown,
            source,
            field_source,
            shape,
        )?;
        Ok(terrane_collection_support::Tuple::new(values.into_vec()))
    }
}
macro_rules! __terrane_document_decode_map {
    ($map:ident) => {
        impl < T : Clone + TerraneDocumentDecode > TerraneDocumentDecode for
        terrane_collection_support::$map < String, T > { fn terrane_decode_document(input
        : & mut terrane_document_support::Document, path : & TerraneDocumentPath <'_ >,
        allow_unknown : bool, source : & str, field_source : & str, shape : &
        TerraneDocumentShape,) -> Result < Self, Vec < TerraneDocumentDiagnostic >> { let
        terrane_document_support::Document::Map(items) = input else { return
        __terrane_document_type_error(input, path, "map", source, field_source); }; let
        mut entries = Vec::with_capacity(items.len()); let mut diagnostics = Vec::new();
        for (key, mut item) in std::mem::take(items) { let item_path =
        __terrane_document_entry_path(path, & key, false); match
        T::terrane_decode_document(& mut item, & item_path, allow_unknown, source,
        field_source, shape.inner(),) { Ok(value) => entries
        .push(terrane_collection_support::Entry::new(key, value)), Err(mut errors) =>
        diagnostics.append(& mut errors), } } if diagnostics.is_empty() {
        Ok(terrane_collection_support::$map ::new(entries)) } else { Err(diagnostics) } }
        }
    };
}
__terrane_document_decode_map!(Map);
__terrane_document_decode_map!(UnorderedMap);
fn __terrane_encode_document<T: TerraneDocumentEncode>(
    value: &T,
    max_depth: usize,
    max_bytes: usize,
    source: &str,
    expected: &str,
) -> TerraneDocumentDecodeOutcome<String> {
    let encoded = (|| {
        let mut writer = terrane_document_support::JsonWriter::new(
            max_depth,
            max_bytes,
        )?;
        value.terrane_encode_document(&mut writer)?;
        writer.finish()
    })();
    match encoded {
        Ok(value) => {
            TerraneDocumentDecodeOutcome {
                value,
                diagnostics: terrane_collection_support::List::new(Vec::new()),
            }
        }
        Err(error) => {
            TerraneDocumentDecodeOutcome {
                value: String::new(),
                diagnostics: terrane_collection_support::List::new(
                    vec![
                        __terrane_document_diagnostic(&"$", expected, "encoding",
                        "writer", error.to_string(), source, source),
                    ],
                ),
            }
        }
    }
}
trait TerraneDocumentEncode {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError>;
}
impl<T: TerraneDocumentEncode + ?Sized> TerraneDocumentEncode for &T {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        T::terrane_encode_document(*self, writer)
    }
}
impl TerraneDocumentEncode for String {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.string(self)
    }
}
impl TerraneDocumentEncode for () {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.null()
    }
}
impl TerraneDocumentEncode for bool {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.bool(*self)
    }
}
macro_rules! __terrane_document_encode_integer {
    ($($type:ty),+ $(,)?) => {
        $(impl TerraneDocumentEncode for $type { fn terrane_encode_document(& self,
        writer : & mut terrane_document_support::JsonWriter,) -> Result < (),
        terrane_document_support::DocumentError > { writer.integer(& self.to_string()) }
        })+
    };
}
__terrane_document_encode_integer!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, terrane_int_support::Int
);
macro_rules! __terrane_document_encode_float {
    ($($type:ty),+ $(,)?) => {
        $(impl TerraneDocumentEncode for $type { fn terrane_encode_document(& self,
        writer : & mut terrane_document_support::JsonWriter,) -> Result < (),
        terrane_document_support::DocumentError > { if ! self.is_finite() { return
        Err(terrane_document_support::DocumentError("non-finite floating-point value cannot be encoded as JSON"
        .into(),)); } let text = if self.abs() >= 1.0e16 || (* self != 0.0 && self.abs()
        < 1.0e-6) { format!("{self:e}") } else { self.to_string() }; writer.decimal(&
        text) } })+
    };
}
__terrane_document_encode_float!(f32, f64);
impl<T: TerraneDocumentEncode> TerraneDocumentEncode for Option<T> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        match self {
            Some(value) => value.terrane_encode_document(writer),
            None => writer.null(),
        }
    }
}
impl<T: TerraneDocumentEncode> TerraneDocumentEncode
for terrane_collection_support::List<T> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_array()?;
        for value in self.as_slice() {
            value.terrane_encode_document(writer)?;
        }
        writer.end_array()
    }
}
impl<T: TerraneDocumentEncode> TerraneDocumentEncode
for terrane_collection_support::Tuple<T> {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        writer.begin_array()?;
        let mut index = 0;
        while let Some(value) = self.get(index) {
            value.terrane_encode_document(writer)?;
            index += 1;
        }
        writer.end_array()
    }
}
macro_rules! __terrane_document_encode_map {
    ($map:ident) => {
        impl < T : TerraneDocumentEncode > TerraneDocumentEncode for
        terrane_collection_support::$map < String, T > { fn terrane_encode_document(&
        self, writer : & mut terrane_document_support::JsonWriter) -> Result < (),
        terrane_document_support::DocumentError > { writer.begin_object() ?; for (key,
        value) in self.as_entries() { writer.key(key) ?; value
        .terrane_encode_document(writer) ?; } writer.end_object() } }
    };
}
__terrane_document_encode_map!(Map);
__terrane_document_encode_map!(UnorderedMap);
pub type TerranePlatformCapability = terrane_platform_support::Capability;
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
pub fn terrane_platform_i128(
    value: &terrane_int_support::Int,
    label: &str,
) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| {
            TerranePlatformResult::error(
                format!("{label} is outside the signed 128-bit platform range"),
            )
        })
}
pub fn terrane_platform_timeout_nanos(value: terrane_int_support::Int) -> i128 {
    if value < terrane_int_support::Int::from(0_i128) {
        return -1;
    }
    i128::from(terrane_int_support::coerce::<u64>(&value).unwrap_or(u64::MAX))
}
macro_rules! terrane_platform_i128 {
    ($value:expr, $label:literal) => {
        match terrane_platform_i128(&$value, $label) { Ok(value) => value, Err(error) =>
        return error, }
    };
}
#[allow(dead_code)]
fn terrane_platform_cancellation_token() -> TerranePlatformCapability {
    terrane_platform_support::cancellation_token()
}
#[allow(dead_code)]
fn terrane_platform_no_resource() -> TerranePlatformCapability {
    TerranePlatformCapability::default()
}
#[allow(dead_code)]
fn terrane_platform_failed_result() -> TerranePlatformResult {
    TerranePlatformResult::error("uninitialized platform value")
}
#[allow(dead_code)]
fn terrane_platform_cancel(token: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::cancel(token)
}
#[allow(dead_code)]
fn terrane_platform_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
#[allow(dead_code)]
fn terrane_platform_result_resource_limit(result: &TerranePlatformResult) -> bool {
    result.resource_limit
}
#[allow(dead_code)]
fn terrane_platform_result_truncated(result: &TerranePlatformResult) -> bool {
    result.truncated
}
#[allow(dead_code)]
fn terrane_platform_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
#[allow(dead_code)]
fn terrane_platform_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_text(result: &TerranePlatformResult) -> String {
    result.text.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_detail(result: &TerranePlatformResult) -> String {
    result.detail.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_bytes(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_int(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
#[allow(dead_code)]
fn terrane_platform_result_bool(result: &TerranePlatformResult) -> bool {
    result.flag
}
#[allow(dead_code)]
fn terrane_platform_result_entries(result: &TerranePlatformResult) -> Vec<String> {
    result.entries.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_capability(
    result: &TerranePlatformResult,
) -> TerranePlatformCapability {
    result.capability.clone().unwrap_or_default()
}
pub fn terrane_platform_secure_random() -> TerranePlatformCapability {
    terrane_platform_support::secure_random()
}
pub fn terrane_platform_pseudo_random(
    algorithm: String,
    seed: Vec<u8>,
) -> TerranePlatformCapability {
    terrane_platform_support::pseudo_random(&algorithm, &seed)
}
pub fn terrane_platform_secret_buffer(data: Vec<u8>) -> TerranePlatformCapability {
    terrane_platform_support::secret_buffer(data)
}
pub fn terrane_platform_destroy_secret(
    secret: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::destroy_secret(secret)
}
pub fn terrane_platform_random_bytes(
    source: &TerranePlatformCapability,
    count: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::random_bytes(
        source,
        terrane_platform_i128!(count, "random byte count"),
    )
}
pub fn terrane_platform_random_bounded(
    source: &TerranePlatformCapability,
    upper: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::random_bounded(
        source,
        terrane_platform_i128!(upper, "random upper bound"),
    )
}
pub fn terrane_platform_random_split(
    source: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::random_split(source)
}
pub fn terrane_platform_digest(
    algorithm: &String,
    data: Vec<u8>,
) -> TerranePlatformResult {
    terrane_platform_support::digest(algorithm, &data)
}
pub fn terrane_platform_hmac(
    algorithm: &String,
    key: &TerranePlatformCapability,
    data: Vec<u8>,
) -> TerranePlatformResult {
    terrane_platform_support::hmac(algorithm, key, &data)
}
pub fn terrane_platform_constant_time_equal(left: Vec<u8>, right: Vec<u8>) -> bool {
    terrane_platform_support::constant_time_equal(&left, &right)
}
pub fn terrane_platform_hex_encode(data: Vec<u8>) -> String {
    terrane_platform_support::hex_encode(&data)
}
pub fn terrane_platform_hex_decode(text: String) -> TerranePlatformResult {
    terrane_platform_support::hex_decode(&text)
}
pub fn terrane_platform_base64_encode(
    data: Vec<u8>,
    url_safe: bool,
    padded: bool,
) -> String {
    terrane_platform_support::base64_encode(&data, url_safe, padded)
}
pub fn terrane_platform_base64_decode(
    text: String,
    url_safe: bool,
    padded: bool,
) -> TerranePlatformResult {
    terrane_platform_support::base64_decode(&text, url_safe, padded)
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
// Source: core/codecs.trn
// Namespace: core/codecs
#[derive(Clone)]
pub struct DecodeResult {
    pub failed: bool,
    pub message: String,
    pub value: Vec<u8>,
}
impl DecodeResult {
    pub fn terrane_construct(failed: bool, message: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: Vec::<u8>::from([]),
        };
        __terrane_constructed_value.construct(failed, message, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, data: Vec<u8>) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = data;
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct HexCodec {}
impl HexCodec {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn encode(&self, data: Vec<u8>) -> String {
        return terrane_platform_hex_encode(data);
    }
    pub fn decode(&self, text: String) -> DecodeResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_hex_decode(text);
        return DecodeResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
}
#[derive(Clone)]
pub struct Base64Codec {
    pub url_safe: bool,
}
impl Base64Codec {
    pub fn terrane_construct(url_safe: bool) -> Self {
        let mut __terrane_constructed_value = Self { url_safe: false };
        __terrane_constructed_value.construct(url_safe);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, url_safe: bool) {
        {
            let __terrane_value = url_safe;
            self.url_safe = __terrane_value;
        }
    }
    pub fn encode(&self, data: Vec<u8>, padded: bool) -> String {
        return terrane_platform_base64_encode(data, self.url_safe, padded);
    }
    pub fn decode(&self, text: String, padded: bool) -> DecodeResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_base64_decode(text, self.url_safe, padded);
        return DecodeResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
}
pub fn hex() -> HexCodec {
    return HexCodec::terrane_construct();
}
pub fn base64() -> Base64Codec {
    return Base64Codec::terrane_construct(false);
}
pub fn base64_url() -> Base64Codec {
    return Base64Codec::terrane_construct(true);
}
pub fn bytes_from_octets(octets: terrane_collection_support::List<u8>) -> Vec<u8> {
    return octets.into_vec();
}
pub fn encode_hex(data: Vec<u8>) -> String {
    return terrane_platform_hex_encode(data);
}
pub fn decode_hex(text: String) -> DecodeResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_hex_decode(text);
    return DecodeResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn encode_base64(data: Vec<u8>, url_safe: bool, padded: bool) -> String {
    return terrane_platform_base64_encode(data, url_safe, padded);
}
pub fn decode_base64(text: String, url_safe: bool, padded: bool) -> DecodeResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_base64_decode(text, url_safe, padded);
    return DecodeResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
// Source: core/random.trn
// Namespace: core/random
#[derive(Clone)]
pub struct ByteResult {
    pub failed: bool,
    pub message: String,
    pub value: Vec<u8>,
}
impl ByteResult {
    pub fn terrane_construct(failed: bool, message: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: Vec::<u8>::from([]),
        };
        __terrane_constructed_value.construct(failed, message, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, data: Vec<u8>) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = data;
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct RandomIntResult {
    pub failed: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl RandomIntResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        number: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(failed, message, number);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        number: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = number.clone();
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct SecretOperationResult {
    pub failed: bool,
    pub message: String,
}
impl SecretOperationResult {
    pub fn terrane_construct(failed: bool, message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(failed, message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct SecretBuffer {
    pub handle: TerranePlatformCapability,
}
impl SecretBuffer {
    pub fn terrane_construct(data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            handle: terrane_platform_secret_buffer(Vec::<u8>::from([])),
        };
        __terrane_constructed_value.construct(data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, data: Vec<u8>) {
        {
            let __terrane_value = terrane_platform_secret_buffer(data);
            self.handle = __terrane_value;
        }
    }
}
pub fn destroy_secret(secret: SecretBuffer) -> SecretOperationResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_destroy_secret(&secret.handle);
    return SecretOperationResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
    );
}
#[derive(Clone)]
pub struct DigestValue {
    pub algorithm: String,
    pub value: Vec<u8>,
}
impl DigestValue {
    pub fn terrane_construct(algorithm: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            algorithm: String::from(""),
            value: Vec::<u8>::from([]),
        };
        __terrane_constructed_value.construct(algorithm, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, algorithm: String, data: Vec<u8>) {
        {
            let __terrane_value = algorithm;
            self.algorithm = __terrane_value;
        }
        {
            let __terrane_value = data;
            self.value = __terrane_value;
        }
    }
    pub fn constant_time_equals(&self, other: DigestValue) -> bool {
        let left: Vec<u8>;
        if self.algorithm.as_str() != other.algorithm.as_str() {
            return false;
        }
        left = self.value.clone();
        return terrane_platform_constant_time_equal(left, other.value);
    }
}
#[derive(Clone)]
pub struct DigestResult {
    pub failed: bool,
    pub message: String,
    pub value: DigestValue,
}
impl DigestResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        digest: DigestValue,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: DigestValue::terrane_construct(String::from(""), Vec::<u8>::from([])),
        };
        __terrane_constructed_value.construct(failed, message, digest);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, digest: DigestValue) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = digest;
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct SignatureValue {
    pub algorithm: String,
    pub value: Vec<u8>,
}
impl SignatureValue {
    pub fn terrane_construct(algorithm: String, data: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            algorithm: String::from(""),
            value: Vec::<u8>::from([]),
        };
        __terrane_constructed_value.construct(algorithm, data);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, algorithm: String, data: Vec<u8>) {
        {
            let __terrane_value = algorithm;
            self.algorithm = __terrane_value;
        }
        {
            let __terrane_value = data;
            self.value = __terrane_value;
        }
    }
    pub fn constant_time_equals(&self, other: SignatureValue) -> bool {
        let left: Vec<u8>;
        if self.algorithm.as_str() != other.algorithm.as_str() {
            return false;
        }
        left = self.value.clone();
        return terrane_platform_constant_time_equal(left, other.value);
    }
}
#[derive(Clone)]
pub struct SignatureResult {
    pub failed: bool,
    pub message: String,
    pub value: SignatureValue,
}
impl SignatureResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        signature: SignatureValue,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: SignatureValue::terrane_construct(
                String::from(""),
                Vec::<u8>::from([]),
            ),
        };
        __terrane_constructed_value.construct(failed, message, signature);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        signature: SignatureValue,
    ) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = signature;
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct SecureRandom {
    pub handle: TerranePlatformCapability,
}
impl SecureRandom {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            handle: terrane_platform_secure_random(),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = terrane_platform_secure_random();
            self.handle = __terrane_value;
        }
    }
    pub fn generate_bytes(&self, count: terrane_int_support::Int) -> ByteResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_random_bytes(&self.handle, count);
        return ByteResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
    pub fn bounded_int(
        &self,
        upper_exclusive: terrane_int_support::Int,
    ) -> RandomIntResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_random_bounded(&self.handle, upper_exclusive);
        return RandomIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct PseudoRandomAlgorithm {
    pub name: String,
}
impl PseudoRandomAlgorithm {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self { name: String::from("") };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, name: String) {
        {
            let __terrane_value = name;
            self.name = __terrane_value;
        }
    }
}
pub fn chacha20() -> PseudoRandomAlgorithm {
    return PseudoRandomAlgorithm::terrane_construct(String::from("chacha20"));
}
#[derive(Clone)]
pub struct PseudoRandom {
    pub handle: TerranePlatformCapability,
}
impl PseudoRandom {
    pub fn terrane_construct(algorithm: PseudoRandomAlgorithm, seed: Vec<u8>) -> Self {
        let mut __terrane_constructed_value = Self {
            handle: terrane_platform_pseudo_random(
                String::from("chacha20"),
                Vec::<u8>::from([]),
            ),
        };
        __terrane_constructed_value.construct(algorithm, seed);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, algorithm: PseudoRandomAlgorithm, seed: Vec<u8>) {
        {
            let __terrane_value = terrane_platform_pseudo_random(algorithm.name, seed);
            self.handle = __terrane_value;
        }
    }
    pub fn generate_bytes(&self, count: terrane_int_support::Int) -> ByteResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_random_bytes(&self.handle, count);
        return ByteResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
        );
    }
    pub fn bounded_int(
        &self,
        upper_exclusive: terrane_int_support::Int,
    ) -> RandomIntResult {
        let raw: TerranePlatformResult;
        raw = terrane_platform_random_bounded(&self.handle, upper_exclusive);
        return RandomIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn split(self) -> PseudoRandom {
        let raw: TerranePlatformResult;
        let mut child: PseudoRandom;
        raw = terrane_platform_random_split(&self.handle);
        child = PseudoRandom::terrane_construct(chacha20(), Vec::<u8>::from([]));
        {
            let __terrane_value = terrane_platform_result_capability(&raw);
            child.handle = __terrane_value;
        }
        return child;
    }
}
pub fn split_pseudo(source: PseudoRandom) -> PseudoRandom {
    let raw: TerranePlatformResult;
    let mut child: PseudoRandom;
    raw = terrane_platform_random_split(&source.handle);
    child = PseudoRandom::terrane_construct(chacha20(), Vec::<u8>::from([]));
    {
        let __terrane_value = terrane_platform_result_capability(&raw);
        child.handle = __terrane_value;
    }
    return child;
}
pub fn secure_bytes(
    source: SecureRandom,
    count: terrane_int_support::Int,
) -> ByteResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_random_bytes(&source.handle, count);
    return ByteResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn pseudo_bytes(
    source: PseudoRandom,
    count: terrane_int_support::Int,
) -> ByteResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_random_bytes(&source.handle, count);
    return ByteResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_bytes(&raw),
    );
}
pub fn secure_bounded_int(
    source: SecureRandom,
    upper_exclusive: terrane_int_support::Int,
) -> RandomIntResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_random_bounded(&source.handle, upper_exclusive);
    return RandomIntResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_int(&raw),
    );
}
pub fn pseudo_bounded_int(
    source: PseudoRandom,
    upper_exclusive: terrane_int_support::Int,
) -> RandomIntResult {
    let raw: TerranePlatformResult;
    raw = terrane_platform_random_bounded(&source.handle, upper_exclusive);
    return RandomIntResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_int(&raw),
    );
}
#[derive(Clone)]
pub struct HashAlgorithm {
    pub name: String,
}
impl HashAlgorithm {
    pub fn terrane_construct(name: String) -> Self {
        let mut __terrane_constructed_value = Self { name: String::from("") };
        __terrane_constructed_value.construct(name);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, name: String) {
        {
            let __terrane_value = name;
            self.name = __terrane_value;
        }
    }
}
pub fn sha256() -> HashAlgorithm {
    return HashAlgorithm::terrane_construct(String::from("sha-256"));
}
pub fn sha512() -> HashAlgorithm {
    return HashAlgorithm::terrane_construct(String::from("sha-512"));
}
pub fn digest_bytes_terrane_core_random(
    algorithm: HashAlgorithm,
    data: Vec<u8>,
) -> DigestResult {
    let raw: TerranePlatformResult;
    let value: DigestValue;
    raw = terrane_platform_digest(&algorithm.name, data);
    value = DigestValue::terrane_construct(
        algorithm.name.clone(),
        terrane_platform_result_bytes(&raw),
    );
    return DigestResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        value,
    );
}
pub fn sign_hmac(
    algorithm: HashAlgorithm,
    key: SecretBuffer,
    data: Vec<u8>,
) -> SignatureResult {
    let raw: TerranePlatformResult;
    let value: SignatureValue;
    raw = terrane_platform_hmac(&algorithm.name, &key.handle, data);
    value = SignatureValue::terrane_construct(
        algorithm.name.clone(),
        terrane_platform_result_bytes(&raw),
    );
    return SignatureResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        value,
    );
}
pub fn digest_equals(left: DigestValue, right: DigestValue) -> bool {
    return left.constant_time_equals(right);
}
pub fn signature_equals(left: SignatureValue, right: SignatureValue) -> bool {
    return left.constant_time_equals(right);
}
// Source: core/legacy_digests.trn
// Namespace: core/random/legacy-digests
pub fn sha1() -> HashAlgorithm {
    return HashAlgorithm::terrane_construct(String::from("sha-1"));
}
pub fn md5() -> HashAlgorithm {
    return HashAlgorithm::terrane_construct(String::from("md5"));
}
pub fn digest_bytes_terrane_core_random_legacy_digests(
    algorithm: HashAlgorithm,
    data: Vec<u8>,
) -> DigestResult {
    return digest_bytes_terrane_core_random(algorithm, data);
}
// Source: core/documents.trn
// Namespace: core/documents
#[derive(Clone)]
pub struct DocumentInteger {
    pub text: String,
}
impl DocumentInteger {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("0") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct DocumentDecimal {
    pub coefficient: String,
    pub exponent: terrane_int_support::Int,
    pub text: String,
}
impl DocumentDecimal {
    pub fn terrane_construct(
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            coefficient: String::from("0"),
            exponent: terrane_int_support::Int::from(0_i128),
            text: String::from("0"),
        };
        __terrane_constructed_value.construct(coefficient, exponent, text);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) {
        {
            let __terrane_value = coefficient;
            self.coefficient = __terrane_value;
        }
        {
            let __terrane_value = exponent.clone();
            self.exponent = __terrane_value;
        }
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
}
pub trait SerializableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn SerializableProtocol>;
    fn separate_box(&self) -> Box<dyn SerializableProtocol>;
    fn to_document(&self) -> DocumentValue;
}
impl Clone for Box<dyn SerializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Serializable(Box<dyn SerializableProtocol>);
impl Clone for Serializable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Serializable {
    pub fn to_document(&self) -> DocumentValue {
        self.0.to_document()
    }
}
pub trait DeserializableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol>;
    fn separate_box(&self) -> Box<dyn DeserializableProtocol>;
    fn from_document(&self, value: DocumentValue) -> DocumentResult;
}
impl Clone for Box<dyn DeserializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct Deserializable(Box<dyn DeserializableProtocol>);
impl Clone for Deserializable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl Deserializable {
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        self.0.from_document(value)
    }
}
pub trait DocumentDecodableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DocumentDecodableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentDecodableProtocol>;
}
impl Clone for Box<dyn DocumentDecodableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[allow(
    dead_code,
    reason = "marker interface storage is materialized only when a value is erased to that marker"
)]
pub struct DocumentDecodable(Box<dyn DocumentDecodableProtocol>);
impl Clone for DocumentDecodable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentDecodable {}
pub trait DocumentEncodableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DocumentEncodableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentEncodableProtocol>;
}
impl Clone for Box<dyn DocumentEncodableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[allow(
    dead_code,
    reason = "marker interface storage is materialized only when a value is erased to that marker"
)]
pub struct DocumentEncodable(Box<dyn DocumentEncodableProtocol>);
impl Clone for DocumentEncodable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentEncodable {}
pub trait DocumentValidatableProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn validate_document(&self) -> Option<String>;
}
impl Clone for Box<dyn DocumentValidatableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
pub struct DocumentValidatable(Box<dyn DocumentValidatableProtocol>);
impl Clone for DocumentValidatable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl DocumentValidatable {
    pub fn validate_document(&self) -> Option<String> {
        self.0.validate_document()
    }
}
#[derive(Clone)]
pub struct DocumentValue {
    pub raw: terrane_document_support::DataResult,
    pub encoded: String,
    pub kind: String,
    pub scalar: String,
    pub integer: DocumentInteger,
    pub decimal: DocumentDecimal,
}
impl DocumentValue {
    pub fn terrane_construct(raw: terrane_document_support::DataResult) -> Self {
        let mut __terrane_constructed_value = Self {
            raw: terrane_empty_document(),
            encoded: String::from(""),
            kind: String::from("invalid"),
            scalar: String::from(""),
            integer: DocumentInteger::terrane_construct(String::from("0")),
            decimal: DocumentDecimal::terrane_construct(
                String::from("0"),
                terrane_int_support::Int::from(0_i128),
                String::from("0"),
            ),
        };
        __terrane_constructed_value.construct(raw);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, raw: terrane_document_support::DataResult) {
        {
            let __terrane_value = terrane_document_kind(&raw);
            self.kind = __terrane_value;
        }
        {
            let __terrane_value = terrane_document_text(&raw);
            self.scalar = __terrane_value;
        }
        {
            let __terrane_value = terrane_data_encoded(&raw);
            self.encoded = __terrane_value;
        }
        if self.kind.as_str() == "integer" {
            {
                let __terrane_value = DocumentInteger::terrane_construct(
                    self.scalar.clone(),
                );
                self.integer = __terrane_value;
            }
        }
        if self.kind.as_str() == "decimal" {
            {
                let __terrane_value = DocumentDecimal::terrane_construct(
                    terrane_document_coefficient(&raw),
                    terrane_document_exponent(&raw),
                    self.scalar.clone(),
                );
                self.decimal = __terrane_value;
            }
        }
        {
            let __terrane_value = raw;
            self.raw = __terrane_value;
        }
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_document_length(&self.raw);
    }
    pub fn to_document(&self) -> DocumentValue {
        return self.clone();
    }
    pub fn item(&self, index: terrane_int_support::Int) -> DocumentResult {
        let raw: terrane_document_support::DataResult;
        raw = terrane_document_item(&self.raw, index.clone());
        return make_document_result(raw);
    }
    pub fn key(&self, index: terrane_int_support::Int) -> String {
        return terrane_document_key(&self.raw, index.clone());
    }
    pub fn field(&self, name: String) -> DocumentResult {
        let raw: terrane_document_support::DataResult;
        raw = terrane_document_field(&self.raw, name);
        return make_document_result(raw);
    }
}
impl TerraneDocumentEncode for DocumentValue {
    fn terrane_encode_document(
        &self,
        writer: &mut terrane_document_support::JsonWriter,
    ) -> Result<(), terrane_document_support::DocumentError> {
        terrane_document_support::write_document_value(writer, &self.to_document().raw)
    }
}
impl SerializableProtocol for DocumentValue {
    fn clone_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn SerializableProtocol> {
        Box::new(self.clone())
    }
    fn to_document(&self) -> DocumentValue {
        DocumentValue::to_document(&*self)
    }
}
impl From<DocumentValue> for Serializable {
    fn from(value: DocumentValue) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct DocumentResult {
    pub failed: bool,
    pub message: String,
    pub path: String,
    pub expected: String,
    pub value: DocumentValue,
}
impl DocumentResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            path: String::from("$"),
            expected: String::from(""),
            value: DocumentValue::terrane_construct(terrane_empty_document()),
        };
        __terrane_constructed_value.construct(failed, message, path, expected, raw);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = path;
            self.path = __terrane_value;
        }
        {
            let __terrane_value = expected;
            self.expected = __terrane_value;
        }
        {
            let __terrane_value = DocumentValue::terrane_construct(raw);
            self.value = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct DocumentMapping {
    pub descriptor_name: String,
    pub expected_kind: String,
    pub field_names: terrane_collection_support::List<String>,
    pub optional_fields: terrane_collection_support::List<String>,
    pub default_fields: terrane_collection_support::List<String>,
    pub default_values: terrane_collection_support::List<String>,
    pub allow_unknown: bool,
}
impl DocumentMapping {
    pub fn terrane_construct(
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            descriptor_name: String::from("document-value"),
            expected_kind: String::from("map"),
            field_names: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            optional_fields: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            default_fields: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            default_values: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            allow_unknown: false,
        };
        __terrane_constructed_value
            .construct(descriptor_name, expected_kind, allow_unknown);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) {
        {
            let __terrane_value = descriptor_name;
            self.descriptor_name = __terrane_value;
        }
        {
            let __terrane_value = expected_kind;
            self.expected_kind = __terrane_value;
        }
        {
            let __terrane_value = allow_unknown;
            self.allow_unknown = __terrane_value;
        }
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return decode_document(value, self.clone());
    }
}
impl DeserializableProtocol for DocumentMapping {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn DeserializableProtocol> {
        Box::new(self.clone())
    }
    fn from_document(&self, value: DocumentValue) -> DocumentResult {
        DocumentMapping::from_document(&*self, value)
    }
}
impl From<DocumentMapping> for Deserializable {
    fn from(value: DocumentMapping) -> Self {
        Self(Box::new(value))
    }
}
pub fn serialize_document(value: Serializable) -> DocumentValue {
    return value.to_document();
}
pub fn deserialize_document(
    value: DocumentValue,
    destination: Deserializable,
) -> DocumentResult {
    return destination.from_document(value);
}
pub fn make_document_result(
    raw: terrane_document_support::DataResult,
) -> DocumentResult {
    return DocumentResult::terrane_construct(
        terrane_data_failed(&raw),
        terrane_data_message(&raw),
        terrane_data_path(&raw),
        terrane_data_expected(&raw),
        raw,
    );
}
pub fn make_document_none() -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_none());
}
pub fn make_document_bool(value: bool) -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_bool(value));
}
pub fn make_document_string(value: String) -> DocumentValue {
    return DocumentValue::terrane_construct(terrane_make_document_string(value));
}
pub fn make_document_integer(value: String) -> DocumentResult {
    return make_document_result(terrane_make_document_integer(value));
}
pub fn make_document_decimal(value: String) -> DocumentResult {
    return make_document_result(terrane_make_document_decimal(value));
}
#[derive(Clone)]
pub struct DocumentMapEntries {
    pub raw: terrane_document_support::DataResult,
}
impl DocumentMapEntries {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            raw: terrane_make_document_map(),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value = terrane_make_document_map();
            self.raw = __terrane_value;
        }
    }
    pub fn append(&mut self, key: String, value: DocumentValue) {
        {
            let __terrane_value = terrane_document_map_insert(
                &self.raw,
                key,
                &value.raw,
            );
            self.raw = __terrane_value;
        }
    }
}
pub fn append_document_map_entry(
    mut entries: DocumentMapEntries,
    key: String,
    value: DocumentValue,
) -> DocumentMapEntries {
    entries.append(key, value);
    return entries;
}
pub fn make_document_list(
    values: terrane_collection_support::List<DocumentValue>,
) -> DocumentResult {
    let mut raw: terrane_document_support::DataResult;
    let mut index: terrane_int_support::Int;
    raw = terrane_make_document_list();
    index = terrane_int_support::Int::from(0_i128);
    while index < terrane_int_support::Int::from(values.length()) {
        raw = terrane_document_list_append(
            &raw,
            &__terrane_raised(
                    values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                238 /* terrane-site: core/documents.trn:141:47-141:60 */,
                            ),
                        ),
                    238 /* terrane-site: core/documents.trn:141:47-141:60 */,
                )
                .raw,
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return make_document_result(raw);
}
pub fn make_document_map(entries: DocumentMapEntries) -> DocumentResult {
    return make_document_result(entries.raw);
}
pub fn mapping_required_fields(
    mapping: DocumentMapping,
) -> terrane_collection_support::List<String> {
    let fields: terrane_collection_support::List<String>;
    let optional_fields: terrane_collection_support::List<String>;
    let default_fields: terrane_collection_support::List<String>;
    let mut required: terrane_collection_support::List<String>;
    let mut index: terrane_int_support::Int;
    let mut field: String;
    let mut optional: bool;
    let mut optional_index: terrane_int_support::Int;
    let mut defaulted: bool;
    let mut default_index: terrane_int_support::Int;
    fields = mapping.field_names.clone();
    optional_fields = mapping.optional_fields.clone();
    default_fields = mapping.default_fields.clone();
    required = terrane_collection_support::List::<String>::new(vec![]);
    index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = required.make_unique();
        while index < terrane_int_support::Int::from(fields.length()) {
            field = __terrane_raised(
                fields
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            239 /* terrane-site: core/documents.trn:154:17-154:30 */,
                        ),
                    ),
                239 /* terrane-site: core/documents.trn:154:17-154:30 */,
            );
            optional = false;
            optional_index = terrane_int_support::Int::from(0_i128);
            while optional_index
                < terrane_int_support::Int::from(optional_fields.length())
            {
                if __terrane_raised(
                        optional_fields
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &optional_index.clone(),
                                    ),
                                    240 /* terrane-site: core/documents.trn:158:16-158:47 */,
                                ),
                            ),
                        240 /* terrane-site: core/documents.trn:158:16-158:47 */,
                    )
                    .as_str() == field.as_str()
                {
                    optional = true;
                }
                optional_index = optional_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            defaulted = false;
            default_index = terrane_int_support::Int::from(0_i128);
            while default_index < terrane_int_support::Int::from(default_fields.length())
            {
                if __terrane_raised(
                        default_fields
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    241 /* terrane-site: core/documents.trn:164:16-164:45 */,
                                ),
                            ),
                        241 /* terrane-site: core/documents.trn:164:16-164:45 */,
                    )
                    .as_str() == field.as_str()
                {
                    defaulted = true;
                }
                default_index = default_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            if field.as_str() != "" && !optional && !defaulted {
                {
                    let __terrane_collection_argument_0 = field;
                    __terrane_list_append_0.push(__terrane_collection_argument_0);
                };
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return required;
}
pub fn decode_document(
    value: DocumentValue,
    mapping: DocumentMapping,
) -> DocumentResult {
    let required: terrane_collection_support::List<String>;
    let mut declared_fields: terrane_collection_support::List<String>;
    let mut field_index: terrane_int_support::Int;
    let mut default_fields: terrane_collection_support::List<String>;
    let mut default_values: terrane_collection_support::List<String>;
    let mut default_index: terrane_int_support::Int;
    let raw: terrane_document_support::DataResult;
    let mut result: DocumentResult;
    required = mapping_required_fields(mapping.clone());
    declared_fields = terrane_collection_support::List::<String>::new(vec![]);
    field_index = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = declared_fields.make_unique();
        while field_index < terrane_int_support::Int::from(mapping.field_names.length())
        {
            if __terrane_raised(
                    mapping
                        .field_names
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &field_index.clone(),
                                ),
                                242 /* terrane-site: core/documents.trn:177:12-177:44 */,
                            ),
                        ),
                    242 /* terrane-site: core/documents.trn:177:12-177:44 */,
                )
                .as_str() != ""
            {
                {
                    let __terrane_collection_argument_0 = __terrane_raised(
                        mapping
                            .field_names
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &field_index.clone(),
                                    ),
                                    243 /* terrane-site: core/documents.trn:178:37-178:69 */,
                                ),
                            ),
                        243 /* terrane-site: core/documents.trn:178:37-178:69 */,
                    );
                    __terrane_list_append_1.push(__terrane_collection_argument_0);
                };
            }
            field_index = field_index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    default_fields = terrane_collection_support::List::<String>::new(vec![]);
    default_values = terrane_collection_support::List::<String>::new(vec![]);
    default_index = terrane_int_support::Int::from(0_i128);
    while default_index < terrane_int_support::Int::from(mapping.default_fields.length())
        && default_index
            < terrane_int_support::Int::from(mapping.default_values.length())
    {
        if __terrane_raised(
                mapping
                    .default_fields
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &default_index.clone(),
                            ),
                            244 /* terrane-site: core/documents.trn:184:12-184:49 */,
                        ),
                    ),
                244 /* terrane-site: core/documents.trn:184:12-184:49 */,
            )
            .as_str() != ""
        {
            {
                let __terrane_collection_argument_0 = __terrane_raised(
                    mapping
                        .default_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &default_index.clone(),
                                ),
                                245 /* terrane-site: core/documents.trn:185:36-185:73 */,
                            ),
                        ),
                    245 /* terrane-site: core/documents.trn:185:36-185:73 */,
                );
                default_fields.append(__terrane_collection_argument_0);
            };
            {
                let __terrane_collection_argument_0 = __terrane_raised(
                    mapping
                        .default_values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &default_index.clone(),
                                ),
                                246 /* terrane-site: core/documents.trn:186:36-186:73 */,
                            ),
                        ),
                    246 /* terrane-site: core/documents.trn:186:36-186:73 */,
                );
                default_values.append(__terrane_collection_argument_0);
            };
        }
        default_index = default_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    raw = terrane_validate_mapping(
        &value.raw,
        mapping.expected_kind,
        required,
        declared_fields,
        default_fields,
        default_values,
        mapping.allow_unknown,
    );
    result = make_document_result(raw);
    if result.failed {
        {
            let __terrane_value = mapping.descriptor_name.clone();
            result.expected = __terrane_value;
        }
    }
    return result;
}
// Source: core/json.trn
// Namespace: core/documents/json
#[derive(Clone)]
pub struct JsonOptions {
    pub max_depth: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
}
impl JsonOptions {
    pub fn terrane_construct(
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            max_depth: terrane_int_support::Int::from(256_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
        };
        __terrane_constructed_value.construct(max_depth, max_bytes);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = max_depth.clone();
            self.max_depth = __terrane_value;
        }
        {
            let __terrane_value = max_bytes.clone();
            self.max_bytes = __terrane_value;
        }
    }
}
pub fn default_json_options() -> JsonOptions {
    return JsonOptions::terrane_construct(
        terrane_int_support::Int::from(256_i128),
        terrane_int_support::Int::from(16777216_i128),
    );
}
pub fn parse_json(input: String, options: JsonOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    raw = terrane_json_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_json(value: DocumentValue, options: JsonOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    raw = terrane_json_stringify(
        &value.raw,
        options.max_depth.clone(),
        options.max_bytes.clone(),
    );
    return make_document_result(raw);
}
pub fn canonical_json(value: DocumentValue) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    raw = terrane_json_canonical(&value.raw);
    return make_document_result(raw);
}
pub fn decode_json(
    input: String,
    mapping: Deserializable,
    options: JsonOptions,
) -> DocumentResult {
    let parsed: DocumentResult;
    parsed = parse_json(input, options);
    if parsed.failed {
        return parsed.clone();
    }
    return deserialize_document(parsed.value.clone(), mapping);
}
pub fn encode_json(value: Serializable, options: JsonOptions) -> DocumentResult {
    return stringify_json(serialize_document(value), options);
}
// Source: core/yaml.trn
// Namespace: core/documents/yaml
#[derive(Clone)]
pub struct YamlOptions {
    pub max_depth: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
    pub max_alias_nodes: terrane_int_support::Int,
}
impl YamlOptions {
    pub fn terrane_construct(
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_alias_nodes: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            max_depth: terrane_int_support::Int::from(128_i128),
            max_bytes: terrane_int_support::Int::from(16777216_i128),
            max_alias_nodes: terrane_int_support::Int::from(65536_i128),
        };
        __terrane_constructed_value.construct(max_depth, max_bytes, max_alias_nodes);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        max_depth: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
        max_alias_nodes: terrane_int_support::Int,
    ) {
        {
            let __terrane_value = max_depth.clone();
            self.max_depth = __terrane_value;
        }
        {
            let __terrane_value = max_bytes.clone();
            self.max_bytes = __terrane_value;
        }
        {
            let __terrane_value = max_alias_nodes.clone();
            self.max_alias_nodes = __terrane_value;
        }
    }
}
pub fn default_yaml_options() -> YamlOptions {
    return YamlOptions::terrane_construct(
        terrane_int_support::Int::from(128_i128),
        terrane_int_support::Int::from(16777216_i128),
        terrane_int_support::Int::from(65536_i128),
    );
}
pub fn make_yaml_options(
    max_depth: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
    max_alias_nodes: terrane_int_support::Int,
) -> YamlOptions {
    return YamlOptions::terrane_construct(
        max_depth.clone(),
        max_bytes.clone(),
        max_alias_nodes.clone(),
    );
}
pub fn parse_yaml(input: String, options: YamlOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    raw = terrane_yaml_parse(
        input,
        options.max_depth.clone(),
        options.max_bytes.clone(),
        options.max_alias_nodes.clone(),
    );
    return make_document_result(raw);
}
pub fn stringify_yaml(value: DocumentValue, options: YamlOptions) -> DocumentResult {
    let raw: terrane_document_support::DataResult;
    raw = terrane_json_stringify(
        &value.raw,
        options.max_depth.clone(),
        options.max_bytes.clone(),
    );
    return make_document_result(raw);
}
pub fn decode_yaml(
    input: String,
    mapping: Deserializable,
    options: YamlOptions,
) -> DocumentResult {
    let parsed: DocumentResult;
    parsed = parse_yaml(input, options);
    if parsed.failed {
        return parsed.clone();
    }
    return deserialize_document(parsed.value.clone(), mapping);
}
pub fn encode_yaml(value: Serializable, options: YamlOptions) -> DocumentResult {
    return stringify_yaml(serialize_document(value), options);
}
// Source: core/urls.trn
// Namespace: core/urls
#[derive(Clone)]
pub struct UrlQuery {
    pub keys: terrane_collection_support::List<String>,
    pub values: terrane_collection_support::List<String>,
    pub count: terrane_int_support::Int,
}
impl UrlQuery {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            keys: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            values: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            count: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        let keys: terrane_collection_support::List<String>;
        let values: terrane_collection_support::List<String>;
        keys = terrane_collection_support::List::<String>::new(vec![]);
        values = terrane_collection_support::List::<String>::new(vec![]);
        {
            let __terrane_value = keys;
            self.keys = __terrane_value;
        }
        {
            let __terrane_value = values;
            self.values = __terrane_value;
        }
        {
            let __terrane_value = terrane_int_support::Int::from(0_i128);
            self.count = __terrane_value;
        }
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return self.count.clone();
    }
    pub fn get(&self, name: String) -> String {
        let mut index: terrane_int_support::Int;
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(self.keys.length()) {
            if __terrane_raised(
                    self
                        .keys
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                247 /* terrane-site: core/urls.trn:23:16-23:32 */,
                            ),
                        ),
                    247 /* terrane-site: core/urls.trn:23:16-23:32 */,
                )
                .as_str() == name.as_str()
            {
                return __terrane_raised(
                    self
                        .values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                248 /* terrane-site: core/urls.trn:24:24-24:42 */,
                            ),
                        ),
                    248 /* terrane-site: core/urls.trn:24:24-24:42 */,
                );
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
        return String::from("");
    }
    pub fn get_all(&self, name: String) -> terrane_collection_support::List<String> {
        let mut result: terrane_collection_support::List<String>;
        let mut index: terrane_int_support::Int;
        result = terrane_collection_support::List::<String>::new(vec![]);
        index = terrane_int_support::Int::from(0_i128);
        {
            let __terrane_list_append_0 = result.make_unique();
            while index < terrane_int_support::Int::from(self.keys.length()) {
                if __terrane_raised(
                        self
                            .keys
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(&index.clone()),
                                    249 /* terrane-site: core/urls.trn:32:16-32:32 */,
                                ),
                            ),
                        249 /* terrane-site: core/urls.trn:32:16-32:32 */,
                    )
                    .as_str() == name.as_str()
                {
                    {
                        let __terrane_collection_argument_0 = __terrane_raised(
                            self
                                .values
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        250 /* terrane-site: core/urls.trn:33:32-33:50 */,
                                    ),
                                ),
                            250 /* terrane-site: core/urls.trn:33:32-33:50 */,
                        );
                        __terrane_list_append_0.push(__terrane_collection_argument_0);
                    };
                }
                index = index.clone() + terrane_int_support::Int::from(1_i128);
            }
        }
        return result;
    }
}
pub fn append_query_entry(mut query: UrlQuery, name: String, value: String) -> UrlQuery {
    {
        let __terrane_value = query.count.clone()
            + terrane_int_support::Int::from(1_i128);
        query.count = __terrane_value;
    }
    {
        let __terrane_collection_argument_0 = name;
        query.keys.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = value;
        query.values.append(__terrane_collection_argument_0);
    };
    return query;
}
#[derive(Clone)]
pub struct Url {
    pub serialized: String,
    pub display: String,
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub path: String,
    pub query: UrlQuery,
    pub fragment: String,
    pub origin: String,
}
impl Url {
    pub fn terrane_construct(
        serialized: String,
        display: String,
        scheme: String,
        username: String,
        password: String,
        host: String,
        port: String,
        path: String,
        query: UrlQuery,
        fragment: String,
        origin: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            serialized: String::from(""),
            display: String::from(""),
            scheme: String::from(""),
            username: String::from(""),
            password: String::from(""),
            host: String::from(""),
            port: String::from(""),
            path: String::from(""),
            query: UrlQuery::terrane_construct(),
            fragment: String::from(""),
            origin: String::from(""),
        };
        __terrane_constructed_value
            .construct(
                serialized,
                display,
                scheme,
                username,
                password,
                host,
                port,
                path,
                query,
                fragment,
                origin,
            );
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        serialized: String,
        display: String,
        scheme: String,
        username: String,
        password: String,
        host: String,
        port: String,
        path: String,
        query: UrlQuery,
        fragment: String,
        origin: String,
    ) {
        {
            let __terrane_value = serialized;
            self.serialized = __terrane_value;
        }
        {
            let __terrane_value = display;
            self.display = __terrane_value;
        }
        {
            let __terrane_value = scheme;
            self.scheme = __terrane_value;
        }
        {
            let __terrane_value = username;
            self.username = __terrane_value;
        }
        {
            let __terrane_value = password;
            self.password = __terrane_value;
        }
        {
            let __terrane_value = host;
            self.host = __terrane_value;
        }
        {
            let __terrane_value = port;
            self.port = __terrane_value;
        }
        {
            let __terrane_value = path;
            self.path = __terrane_value;
        }
        {
            let __terrane_value = query;
            self.query = __terrane_value;
        }
        {
            let __terrane_value = fragment;
            self.fragment = __terrane_value;
        }
        {
            let __terrane_value = origin;
            self.origin = __terrane_value;
        }
    }
    pub fn string(&self) -> String {
        return self.display.clone();
    }
    pub fn resolve(&self, relative: String) -> UrlResult {
        return parse_url_relative(relative, self.clone());
    }
}
#[derive(Clone)]
pub struct UrlResult {
    pub failed: bool,
    pub message: String,
    pub value: Url,
}
impl UrlResult {
    pub fn terrane_construct(failed: bool, message: String, parsed_url: Url) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: Url::terrane_construct(
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                UrlQuery::terrane_construct(),
                String::from(""),
                String::from(""),
            ),
        };
        __terrane_constructed_value.construct(failed, message, parsed_url);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, parsed_url: Url) {
        {
            let __terrane_value = failed;
            self.failed = __terrane_value;
        }
        {
            let __terrane_value = message;
            self.message = __terrane_value;
        }
        {
            let __terrane_value = parsed_url;
            self.value = __terrane_value;
        }
    }
}
pub fn url_from_platform(raw: terrane_document_support::UrlResult) -> UrlResult {
    let failed: bool;
    let empty_query: UrlQuery;
    let empty_url: Url;
    let mut query: UrlQuery;
    let count: terrane_int_support::Int;
    let mut index: terrane_int_support::Int;
    let value: Url;
    failed = terrane_url_failed(&raw);
    if failed {
        empty_query = UrlQuery::terrane_construct();
        empty_url = Url::terrane_construct(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            empty_query,
            String::from(""),
            String::from(""),
        );
        return UrlResult::terrane_construct(true, terrane_url_message(&raw), empty_url);
    }
    query = UrlQuery::terrane_construct();
    count = terrane_url_query_length(&raw);
    index = terrane_int_support::Int::from(0_i128);
    while index < count {
        query = append_query_entry(
            query,
            terrane_url_query_key(&raw, index.clone()),
            terrane_url_query_value(&raw, index.clone()),
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    value = Url::terrane_construct(
        terrane_url_serialized(&raw),
        terrane_url_display(&raw),
        terrane_url_scheme(&raw),
        terrane_url_username(&raw),
        terrane_url_password(&raw),
        terrane_url_host(&raw),
        terrane_url_port(&raw),
        terrane_url_path(&raw),
        query,
        terrane_url_fragment(&raw),
        terrane_url_origin(&raw),
    );
    return UrlResult::terrane_construct(false, String::from(""), value);
}
pub fn parse_url(input: String) -> UrlResult {
    let raw: terrane_document_support::UrlResult;
    raw = terrane_url_parse(input, String::from(""));
    return url_from_platform(raw);
}
pub fn parse_url_relative(input: String, base: Url) -> UrlResult {
    let raw: terrane_document_support::UrlResult;
    raw = terrane_url_parse(input, base.serialized);
    return url_from_platform(raw);
}
