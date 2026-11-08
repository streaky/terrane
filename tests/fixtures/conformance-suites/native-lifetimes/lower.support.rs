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
#[allow(dead_code, reason = "a projected dependency may expose no Result members")]
const TERRANE_DEPENDENCY_ERROR: DescriptorId = DescriptorId(0);
#[allow(dead_code, reason = "panic catching may be disabled or not crossed")]
const TERRANE_DEPENDENCY_PANIC: DescriptorId = DescriptorId(1);
#[allow(
    dead_code,
    reason = "projected type methods may be imported without being crossed"
)]
fn __terrane_dependency_panic(
    payload: std::boxed::Box<dyn std::any::Any + Send>,
    crate_name: &'static str,
    member: &'static str,
) -> TerraneForeignError {
    let detail = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("non-string panic payload");
    TerraneForeignError(
        TerraneError::custom_raised(
            TERRANE_DEPENDENCY_PANIC,
            format!(
                "Rust dependency `{crate_name}` member `{member}` panicked: {detail}"
            ),
            TERRANE_NO_SITE,
        ),
    )
}
mod __terrane_error_registry {
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 3] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
        "/native-view-list-rebind::Err",
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
    pub static FILES: [&str; 56] = [
        "tests/integration/completion-finally-view-post-read/case.trn",
        "tests/integration/completion-native-catch-post-read/case.trn",
        "tests/integration/completion-native-post-read/case.trn",
        "tests/integration/native-copy-view-list-lender/case.trn",
        "tests/integration/native-copy-view-reassignment-snapshot/case.trn",
        "tests/integration/native-field-copy-clone-conversion/case.trn",
        "tests/integration/native-finally-only-rewrite/case.trn",
        "tests/integration/native-inferred-copy-view/case.trn",
        "tests/integration/native-loan-append-transfer/case.trn",
        "tests/integration/native-loan-chain-rebind-receiver/case.trn",
        "tests/integration/native-loan-exclusive-branch/case.trn",
        "tests/integration/native-loan-finally-copy-rebind/case.trn",
        "tests/integration/native-loan-fluent-nested-rebind/case.trn",
        "tests/integration/native-loan-helper-view-origin/case.trn",
        "tests/integration/native-loan-last-use-control/case.trn",
        "tests/integration/native-loan-loop-live-lender/case.trn",
        "tests/integration/native-loan-loop-refreshed/case.trn",
        "tests/integration/native-loan-map-set-lender/case.trn",
        "tests/integration/native-loan-owned-observation/case.trn",
        "tests/integration/native-loan-predicate-last-use/case.trn",
        "tests/integration/native-loan-predicate-switch/case.trn",
        "tests/integration/native-loan-predicate-three-arms/case.trn",
        "tests/integration/native-loan-rebound-alias-chain/case.trn",
        "tests/integration/native-loan-rebound-other/case.trn",
        "tests/integration/native-loan-rebound-same/case.trn",
        "tests/integration/native-loan-return-branch/case.trn",
        "tests/integration/native-loan-storage-alias-last-use/case.trn",
        "tests/integration/native-loan-try-finally-last-use/case.trn",
        "tests/integration/native-loan-try-local/case.trn",
        "tests/integration/native-loan-whole-last-use/case.trn",
        "tests/integration/native-loan-whole-move-last-use/case.trn",
        "tests/integration/native-loan-whole-mutate-last-use/case.trn",
        "tests/integration/native-loop-consuming-condition-break/case.trn",
        "tests/integration/native-noncopy-view-loop-append/case.trn",
        "tests/integration/native-ref-clone-repeat/case.trn",
        "tests/integration/native-required-enum-field/case.trn",
        "tests/integration/native-return-parameter-single/case.trn",
        "tests/integration/native-try-body-rewrite-control/case.trn",
        "tests/integration/native-try-catch-dominating-rewrite/case.trn",
        "tests/integration/native-try-copy-finally-control/case.trn",
        "tests/integration/native-try-finally-dominating-rewrite/case.trn",
        "tests/integration/native-try-finally-rewrite-control/case.trn",
        "tests/integration/native-try-local-consume/case.trn",
        "tests/integration/native-try-unconditional-rewrite-control/case.trn",
        "tests/integration/native-unordered-map-dead-lender-control/case.trn",
        "tests/integration/native-view-list-rebind/case.trn",
        "tests/integration/native-view-loop-binders/case.trn",
        "tests/integration/native-view-map-entry-iteration/case.trn",
        "tests/integration/native-view-tuple-iteration/case.trn",
        "tests/integration/native-view-value-iteration/case.trn",
        "tests/integration/nested-noncopy-view-loops/case.trn",
        "tests/integration/nested-view-loops/case.trn",
        "tests/integration/nested-view-ref-value-loop/case.trn",
        "tests/integration/nested-view-value-ref-loop/case.trn",
        "tests/integration/nested-view-value-value-loop/case.trn",
        "tests/integration/shared-loop-read-control/case.trn",
    ];
    pub static FUNCTIONS: [&str; 69] = [
        "/completion-finally-view-post-read::test-scenario",
        "/completion-native-catch-post-read::test-scenario",
        "/completion-native-post-read::test-scenario",
        "/native-copy-view-list-lender::test-scenario",
        "/native-copy-view-reassignment-snapshot::fresh",
        "/native-copy-view-reassignment-snapshot::test-scenario",
        "/native-field-copy-clone-conversion::construct",
        "/native-field-copy-clone-conversion::observe",
        "/native-field-copy-clone-conversion::clone-observe",
        "/native-field-copy-clone-conversion::test-scenario",
        "/native-finally-only-rewrite::test-scenario",
        "/native-inferred-copy-view::mutable_size",
        "/native-inferred-copy-view::readonly_size",
        "/native-inferred-copy-view::test-scenario",
        "/native-loan-append-transfer::test-scenario",
        "/native-loan-chain-rebind-receiver::test-scenario",
        "/native-loan-exclusive-branch::test-scenario",
        "/native-loan-finally-copy-rebind::test-scenario",
        "/native-loan-fluent-nested-rebind::test-scenario",
        "/native-loan-helper-view-origin::test-scenario",
        "/native-loan-last-use-control::test-scenario",
        "/native-loan-loop-live-lender::test-scenario",
        "/native-loan-loop-refreshed::test-scenario",
        "/native-loan-map-set-lender::test-scenario",
        "/native-loan-owned-observation::test-scenario",
        "/native-loan-predicate-last-use::test-scenario",
        "/native-loan-predicate-switch::test-scenario",
        "/native-loan-predicate-three-arms::test-scenario",
        "/native-loan-rebound-alias-chain::test-scenario",
        "/native-loan-rebound-other::test-scenario",
        "/native-loan-rebound-same::test-scenario",
        "/native-loan-return-branch::test-scenario",
        "/native-loan-storage-alias-last-use::test-scenario",
        "/native-loan-try-finally-last-use::test-scenario",
        "/native-loan-try-local::test-scenario",
        "/native-loan-whole-last-use::test-scenario",
        "/native-loan-whole-move-last-use::test-scenario",
        "/native-loan-whole-mutate-last-use::test-scenario",
        "/native-loop-consuming-condition-break::test-scenario",
        "/native-noncopy-view-loop-append::test-scenario",
        "/native-ref-clone-repeat::twice",
        "/native-ref-clone-repeat::copied",
        "/native-ref-clone-repeat::test-scenario",
        "/native-required-enum-field::construct",
        "/native-required-enum-field::finish",
        "/native-required-enum-field::test-scenario",
        "/native-return-parameter-single::f",
        "/native-return-parameter-single::test-scenario",
        "/native-try-body-rewrite-control::test-scenario",
        "/native-try-catch-dominating-rewrite::test-scenario",
        "/native-try-copy-finally-control::test-scenario",
        "/native-try-finally-dominating-rewrite::test-scenario",
        "/native-try-finally-rewrite-control::test-scenario",
        "/native-try-local-consume::test-scenario",
        "/native-try-unconditional-rewrite-control::test-scenario",
        "/native-unordered-map-dead-lender-control::test-scenario",
        "/native-view-list-rebind::test-scenario",
        "/native-view-loop-binders::test-scenario",
        "/native-view-map-entry-iteration::test-scenario",
        "/native-view-tuple-iteration::test-scenario",
        "/native-view-value-iteration::observe",
        "/native-view-value-iteration::observe-ref",
        "/native-view-value-iteration::test-scenario",
        "/nested-noncopy-view-loops::test-scenario",
        "/nested-view-loops::test-scenario",
        "/nested-view-ref-value-loop::test-scenario",
        "/nested-view-value-ref-loop::test-scenario",
        "/nested-view-value-value-loop::test-scenario",
        "/shared-loop-read-control::test-scenario",
    ];
    pub static SITES: [Site; 379] = [
        /* terrane-site-row: site 0: /completion-finally-view-post-read::test-scenario (tests/integration/completion-finally-view-post-read/case.trn:6:9-6:21) */
        { Site { function: 0, file: 0, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 1: /completion-finally-view-post-read::test-scenario (tests/integration/completion-finally-view-post-read/case.trn:7:9-7:21) */
        { Site { function: 0, file: 0, line: 7, column: 9, end_line: 7, end_column: 21 } },
        /* terrane-site-row: site 2: /completion-finally-view-post-read::test-scenario (tests/integration/completion-finally-view-post-read/case.trn:8:9-8:16) */
        { Site { function: 0, file: 0, line: 8, column: 9, end_line: 8, end_column: 16 } },
        /* terrane-site-row: site 3: /completion-finally-view-post-read::test-scenario (tests/integration/completion-finally-view-post-read/case.trn:10:17-10:29) */
        { Site { function: 0, file: 0, line: 10, column: 17, end_line: 10, end_column: 29 } },
        /* terrane-site-row: site 4: /completion-finally-view-post-read::test-scenario (tests/integration/completion-finally-view-post-read/case.trn:12:13-12:20) */
        { Site { function: 0, file: 0, line: 12, column: 13, end_line: 12, end_column: 20 } },
        /* terrane-site-row: site 5: /completion-finally-view-post-read::test-scenario (tests/integration/completion-finally-view-post-read/case.trn:13:13-13:20) */
        { Site { function: 0, file: 0, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 6: /completion-native-catch-post-read::test-scenario (tests/integration/completion-native-catch-post-read/case.trn:6:9-6:17) */
        { Site { function: 1, file: 1, line: 6, column: 9, end_line: 6, end_column: 17 } },
        /* terrane-site-row: site 7: /completion-native-catch-post-read::test-scenario (tests/integration/completion-native-catch-post-read/case.trn:8:17-8:30) */
        { Site { function: 1, file: 1, line: 8, column: 17, end_line: 8, end_column: 30 } },
        /* terrane-site-row: site 8: /completion-native-catch-post-read::test-scenario (tests/integration/completion-native-catch-post-read/case.trn:9:13-9:21) */
        { Site { function: 1, file: 1, line: 9, column: 13, end_line: 9, end_column: 21 } },
        /* terrane-site-row: site 9: /completion-native-catch-post-read::test-scenario (tests/integration/completion-native-catch-post-read/case.trn:12:13-12:21) */
        { Site { function: 1, file: 1, line: 12, column: 13, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 10: /completion-native-post-read::test-scenario (tests/integration/completion-native-post-read/case.trn:6:9-6:17) */
        { Site { function: 2, file: 2, line: 6, column: 9, end_line: 6, end_column: 17 } },
        /* terrane-site-row: site 11: /completion-native-post-read::test-scenario (tests/integration/completion-native-post-read/case.trn:8:17-8:30) */
        { Site { function: 2, file: 2, line: 8, column: 17, end_line: 8, end_column: 30 } },
        /* terrane-site-row: site 12: /completion-native-post-read::test-scenario (tests/integration/completion-native-post-read/case.trn:9:13-9:21) */
        { Site { function: 2, file: 2, line: 9, column: 13, end_line: 9, end_column: 21 } },
        /* terrane-site-row: site 13: /completion-native-post-read::test-scenario (tests/integration/completion-native-post-read/case.trn:12:13-12:21) */
        { Site { function: 2, file: 2, line: 12, column: 13, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 14: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:6:14-6:26) */
        { Site { function: 3, file: 3, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 15: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:7:20-7:37) */
        { Site { function: 3, file: 3, line: 7, column: 20, end_line: 7, end_column: 37 } },
        /* terrane-site-row: site 16: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:9:13-9:21) */
        { Site { function: 3, file: 3, line: 9, column: 13, end_line: 9, end_column: 21 } },
        /* terrane-site-row: site 17: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:9:13-9:27) */
        { Site { function: 3, file: 3, line: 9, column: 13, end_line: 9, end_column: 27 } },
        /* terrane-site-row: site 18: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:10:28-10:45) */
        { Site { function: 3, file: 3, line: 10, column: 28, end_line: 10, end_column: 45 } },
        /* terrane-site-row: site 19: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:12:13-12:28) */
        { Site { function: 3, file: 3, line: 12, column: 13, end_line: 12, end_column: 28 } },
        /* terrane-site-row: site 20: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:12:13-12:31) */
        { Site { function: 3, file: 3, line: 12, column: 13, end_line: 12, end_column: 31 } },
        /* terrane-site-row: site 21: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:12:13-12:37) */
        { Site { function: 3, file: 3, line: 12, column: 13, end_line: 12, end_column: 37 } },
        /* terrane-site-row: site 22: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:13:5-13:17) */
        { Site { function: 3, file: 3, line: 13, column: 5, end_line: 13, end_column: 17 } },
        /* terrane-site-row: site 23: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:14:13-14:30) */
        { Site { function: 3, file: 3, line: 14, column: 13, end_line: 14, end_column: 30 } },
        /* terrane-site-row: site 24: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:15:12-15:29) */
        { Site { function: 3, file: 3, line: 15, column: 12, end_line: 15, end_column: 29 } },
        /* terrane-site-row: site 25: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:16:5-16:15) */
        { Site { function: 3, file: 3, line: 16, column: 5, end_line: 16, end_column: 15 } },
        /* terrane-site-row: site 26: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:18:13-18:23) */
        { Site { function: 3, file: 3, line: 18, column: 13, end_line: 18, end_column: 23 } },
        /* terrane-site-row: site 27: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:19:13-19:22) */
        { Site { function: 3, file: 3, line: 19, column: 13, end_line: 19, end_column: 22 } },
        /* terrane-site-row: site 28: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:19:13-19:28) */
        { Site { function: 3, file: 3, line: 19, column: 13, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 29: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:23:17-23:27) */
        { Site { function: 3, file: 3, line: 23, column: 17, end_line: 23, end_column: 27 } },
        /* terrane-site-row: site 30: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:24:5-24:17) */
        { Site { function: 3, file: 3, line: 24, column: 5, end_line: 24, end_column: 17 } },
        /* terrane-site-row: site 31: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:25:13-25:30) */
        { Site { function: 3, file: 3, line: 25, column: 13, end_line: 25, end_column: 30 } },
        /* terrane-site-row: site 32: /native-copy-view-reassignment-snapshot::fresh (tests/integration/native-copy-view-reassignment-snapshot/case.trn:7:12-7:29) */
        { Site { function: 4, file: 4, line: 7, column: 12, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 33: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:10:9-10:21) */
        { Site { function: 5, file: 4, line: 10, column: 9, end_line: 10, end_column: 21 } },
        /* terrane-site-row: site 34: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:11:9-11:21) */
        { Site { function: 5, file: 4, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 35: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:12:9-12:21) */
        { Site { function: 5, file: 4, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 36: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:14:9-14:21) */
        { Site { function: 5, file: 4, line: 14, column: 9, end_line: 14, end_column: 21 } },
        /* terrane-site-row: site 37: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:15:5-15:12) */
        { Site { function: 5, file: 4, line: 15, column: 5, end_line: 15, end_column: 12 } },
        /* terrane-site-row: site 38: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:22) */
        { Site { function: 5, file: 4, line: 16, column: 13, end_line: 16, end_column: 22 } },
        /* terrane-site-row: site 39: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:28) */
        { Site { function: 5, file: 4, line: 16, column: 13, end_line: 16, end_column: 28 } },
        /* terrane-site-row: site 40: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:18:13-18:20) */
        { Site { function: 5, file: 4, line: 18, column: 13, end_line: 18, end_column: 20 } },
        /* terrane-site-row: site 41: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:19:5-19:12) */
        { Site { function: 5, file: 4, line: 19, column: 5, end_line: 19, end_column: 12 } },
        /* terrane-site-row: site 42: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:22) */
        { Site { function: 5, file: 4, line: 20, column: 13, end_line: 20, end_column: 22 } },
        /* terrane-site-row: site 43: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:28) */
        { Site { function: 5, file: 4, line: 20, column: 13, end_line: 20, end_column: 28 } },
        /* terrane-site-row: site 44: /native-field-copy-clone-conversion::construct (tests/integration/native-field-copy-clone-conversion/case.trn:7:18-7:29) */
        { Site { function: 6, file: 5, line: 7, column: 18, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 45: /native-field-copy-clone-conversion::construct (tests/integration/native-field-copy-clone-conversion/case.trn:8:18-8:31) */
        { Site { function: 6, file: 5, line: 8, column: 18, end_line: 8, end_column: 31 } },
        /* terrane-site-row: site 46: /native-field-copy-clone-conversion::observe (tests/integration/native-field-copy-clone-conversion/case.trn:10:12-10:22) */
        { Site { function: 7, file: 5, line: 10, column: 12, end_line: 10, end_column: 22 } },
        /* terrane-site-row: site 47: /native-field-copy-clone-conversion::clone-observe (tests/integration/native-field-copy-clone-conversion/case.trn:12:12-12:19) */
        { Site { function: 8, file: 5, line: 12, column: 12, end_line: 12, end_column: 19 } },
        /* terrane-site-row: site 48: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:17:17-17:27) */
        { Site { function: 9, file: 5, line: 17, column: 17, end_line: 17, end_column: 27 } },
        /* terrane-site-row: site 49: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:18:17-18:29) */
        { Site { function: 9, file: 5, line: 18, column: 17, end_line: 18, end_column: 29 } },
        /* terrane-site-row: site 50: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:19:17-19:26) */
        { Site { function: 9, file: 5, line: 19, column: 17, end_line: 19, end_column: 26 } },
        /* terrane-site-row: site 51: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:21:13-21:23) */
        { Site { function: 9, file: 5, line: 21, column: 13, end_line: 21, end_column: 23 } },
        /* terrane-site-row: site 52: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:22:9-22:22) */
        { Site { function: 9, file: 5, line: 22, column: 9, end_line: 22, end_column: 22 } },
        /* terrane-site-row: site 53: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:23:13-23:29) */
        { Site { function: 9, file: 5, line: 23, column: 13, end_line: 23, end_column: 29 } },
        /* terrane-site-row: site 54: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:24:13-24:29) */
        { Site { function: 9, file: 5, line: 24, column: 13, end_line: 24, end_column: 29 } },
        /* terrane-site-row: site 55: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:4:9-4:21) */
        { Site { function: 10, file: 6, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 56: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:5:9-5:16) */
        { Site { function: 10, file: 6, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 57: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:7:17-7:29) */
        { Site { function: 10, file: 6, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 58: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:9:13-9:20) */
        { Site { function: 10, file: 6, line: 9, column: 13, end_line: 9, end_column: 20 } },
        /* terrane-site-row: site 59: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:10:17-10:24) */
        { Site { function: 10, file: 6, line: 10, column: 17, end_line: 10, end_column: 24 } },
        /* terrane-site-row: site 60: /native-inferred-copy-view::mutable_size (tests/integration/native-inferred-copy-view/case.trn:5:12-5:23) */
        { Site { function: 11, file: 7, line: 5, column: 12, end_line: 5, end_column: 23 } },
        /* terrane-site-row: site 61: /native-inferred-copy-view::readonly_size (tests/integration/native-inferred-copy-view/case.trn:8:12-8:23) */
        { Site { function: 12, file: 7, line: 8, column: 12, end_line: 8, end_column: 23 } },
        /* terrane-site-row: site 62: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:11:14-11:26) */
        { Site { function: 13, file: 7, line: 11, column: 14, end_line: 11, end_column: 26 } },
        /* terrane-site-row: site 63: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:12:21-12:38) */
        { Site { function: 13, file: 7, line: 12, column: 21, end_line: 12, end_column: 38 } },
        /* terrane-site-row: site 64: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:15:23-15:40) */
        { Site { function: 13, file: 7, line: 15, column: 23, end_line: 15, end_column: 40 } },
        /* terrane-site-row: site 65: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:16:13-16:24) */
        { Site { function: 13, file: 7, line: 16, column: 13, end_line: 16, end_column: 24 } },
        /* terrane-site-row: site 66: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:16:13-16:30) */
        { Site { function: 13, file: 7, line: 16, column: 13, end_line: 16, end_column: 30 } },
        /* terrane-site-row: site 67: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:17:13-17:24) */
        { Site { function: 13, file: 7, line: 17, column: 13, end_line: 17, end_column: 24 } },
        /* terrane-site-row: site 68: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:17:13-17:30) */
        { Site { function: 13, file: 7, line: 17, column: 13, end_line: 17, end_column: 30 } },
        /* terrane-site-row: site 69: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:18:37-18:54) */
        { Site { function: 13, file: 7, line: 18, column: 37, end_line: 18, end_column: 54 } },
        /* terrane-site-row: site 70: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:19:13-19:21) */
        { Site { function: 13, file: 7, line: 19, column: 13, end_line: 19, end_column: 21 } },
        /* terrane-site-row: site 71: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:19:13-19:27) */
        { Site { function: 13, file: 7, line: 19, column: 13, end_line: 19, end_column: 27 } },
        /* terrane-site-row: site 72: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:6:9-6:21) */
        { Site { function: 14, file: 8, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 73: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:7:9-7:16) */
        { Site { function: 14, file: 8, line: 7, column: 9, end_line: 7, end_column: 16 } },
        /* terrane-site-row: site 74: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:8:21-8:28) */
        { Site { function: 14, file: 8, line: 8, column: 21, end_line: 8, end_column: 28 } },
        /* terrane-site-row: site 75: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:10:13-10:22) */
        { Site { function: 14, file: 8, line: 10, column: 13, end_line: 10, end_column: 22 } },
        /* terrane-site-row: site 76: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:10:13-10:28) */
        { Site { function: 14, file: 8, line: 10, column: 13, end_line: 10, end_column: 28 } },
        /* terrane-site-row: site 77: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:11:13-11:20) */
        { Site { function: 14, file: 8, line: 11, column: 13, end_line: 11, end_column: 20 } },
        /* terrane-site-row: site 78: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:14:13-14:24) */
        { Site { function: 14, file: 8, line: 14, column: 13, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 79: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:14:13-14:30) */
        { Site { function: 14, file: 8, line: 14, column: 13, end_line: 14, end_column: 30 } },
        /* terrane-site-row: site 80: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:11:9-11:21) */
        { Site { function: 15, file: 9, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 81: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:12:9-12:21) */
        { Site { function: 15, file: 9, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 82: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:13:9-13:16) */
        { Site { function: 15, file: 9, line: 13, column: 9, end_line: 13, end_column: 16 } },
        /* terrane-site-row: site 83: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:14:9-14:24) */
        { Site { function: 15, file: 9, line: 14, column: 9, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 84: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:15:9-15:16) */
        { Site { function: 15, file: 9, line: 15, column: 9, end_line: 15, end_column: 16 } },
        /* terrane-site-row: site 85: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:16:13-16:20) */
        { Site { function: 15, file: 9, line: 16, column: 13, end_line: 16, end_column: 20 } },
        /* terrane-site-row: site 86: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:16:24-16:31) */
        { Site { function: 15, file: 9, line: 16, column: 24, end_line: 16, end_column: 31 } },
        /* terrane-site-row: site 87: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:11:22-11:34) */
        { Site { function: 16, file: 10, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 88: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:11:38-11:50) */
        { Site { function: 16, file: 10, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 89: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:12:9-12:22) */
        { Site { function: 16, file: 10, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 90: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:15:19-15:31) */
        { Site { function: 16, file: 10, line: 15, column: 19, end_line: 15, end_column: 31 } },
        /* terrane-site-row: site 91: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:17:17-17:24) */
        { Site { function: 16, file: 10, line: 17, column: 17, end_line: 17, end_column: 24 } },
        /* terrane-site-row: site 92: /native-loan-finally-copy-rebind::test-scenario (tests/integration/native-loan-finally-copy-rebind/case.trn:11:9-11:21) */
        { Site { function: 17, file: 11, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 93: /native-loan-finally-copy-rebind::test-scenario (tests/integration/native-loan-finally-copy-rebind/case.trn:12:9-12:21) */
        { Site { function: 17, file: 11, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 94: /native-loan-finally-copy-rebind::test-scenario (tests/integration/native-loan-finally-copy-rebind/case.trn:18:21-18:28) */
        { Site { function: 17, file: 11, line: 18, column: 21, end_line: 18, end_column: 28 } },
        /* terrane-site-row: site 95: /native-loan-finally-copy-rebind::test-scenario (tests/integration/native-loan-finally-copy-rebind/case.trn:19:17-19:29) */
        { Site { function: 17, file: 11, line: 19, column: 17, end_line: 19, end_column: 29 } },
        /* terrane-site-row: site 96: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:6:9-6:21) */
        { Site { function: 18, file: 12, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 97: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:7:17-7:29) */
        { Site { function: 18, file: 12, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 98: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:8:9-8:16) */
        { Site { function: 18, file: 12, line: 8, column: 9, end_line: 8, end_column: 16 } },
        /* terrane-site-row: site 99: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:9:21-9:29) */
        { Site { function: 18, file: 12, line: 9, column: 21, end_line: 9, end_column: 29 } },
        /* terrane-site-row: site 100: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:10:9-10:16) */
        { Site { function: 18, file: 12, line: 10, column: 9, end_line: 10, end_column: 16 } },
        /* terrane-site-row: site 101: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:12:9-12:16) */
        { Site { function: 18, file: 12, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 102: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:13:5-13:20) */
        { Site { function: 18, file: 12, line: 13, column: 5, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 103: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:16:13-16:20) */
        { Site { function: 18, file: 12, line: 16, column: 13, end_line: 16, end_column: 20 } },
        /* terrane-site-row: site 104: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:15:9-15:21) */
        { Site { function: 19, file: 13, line: 15, column: 9, end_line: 15, end_column: 21 } },
        /* terrane-site-row: site 105: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:16:19-16:31) */
        { Site { function: 19, file: 13, line: 16, column: 19, end_line: 16, end_column: 31 } },
        /* terrane-site-row: site 106: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:17:13-17:23) */
        { Site { function: 19, file: 13, line: 17, column: 13, end_line: 17, end_column: 23 } },
        /* terrane-site-row: site 107: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:18:32-18:44) */
        { Site { function: 19, file: 13, line: 18, column: 32, end_line: 18, end_column: 44 } },
        /* terrane-site-row: site 108: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:22) */
        { Site { function: 19, file: 13, line: 19, column: 13, end_line: 19, end_column: 22 } },
        /* terrane-site-row: site 109: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:28) */
        { Site { function: 19, file: 13, line: 19, column: 13, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 110: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:20:5-20:12) */
        { Site { function: 19, file: 13, line: 20, column: 5, end_line: 20, end_column: 12 } },
        /* terrane-site-row: site 111: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:6:9-6:21) */
        { Site { function: 20, file: 14, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 112: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:7:38-7:50) */
        { Site { function: 20, file: 14, line: 7, column: 38, end_line: 7, end_column: 50 } },
        /* terrane-site-row: site 113: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:8:13-8:22) */
        { Site { function: 20, file: 14, line: 8, column: 13, end_line: 8, end_column: 22 } },
        /* terrane-site-row: site 114: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:8:13-8:28) */
        { Site { function: 20, file: 14, line: 8, column: 13, end_line: 8, end_column: 28 } },
        /* terrane-site-row: site 115: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:9:5-9:12) */
        { Site { function: 20, file: 14, line: 9, column: 5, end_line: 9, end_column: 12 } },
        /* terrane-site-row: site 116: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:10:13-10:25) */
        { Site { function: 20, file: 14, line: 10, column: 13, end_line: 10, end_column: 25 } },
        /* terrane-site-row: site 117: /native-loan-loop-live-lender::test-scenario (tests/integration/native-loan-loop-live-lender/case.trn:6:9-6:21) */
        { Site { function: 21, file: 15, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 118: /native-loan-loop-live-lender::test-scenario (tests/integration/native-loan-loop-live-lender/case.trn:7:38-7:50) */
        { Site { function: 21, file: 15, line: 7, column: 38, end_line: 7, end_column: 50 } },
        /* terrane-site-row: site 119: /native-loan-loop-live-lender::test-scenario (tests/integration/native-loan-loop-live-lender/case.trn:9:17-9:28) */
        { Site { function: 21, file: 15, line: 9, column: 17, end_line: 9, end_column: 28 } },
        /* terrane-site-row: site 120: /native-loan-loop-live-lender::test-scenario (tests/integration/native-loan-loop-live-lender/case.trn:10:5-10:12) */
        { Site { function: 21, file: 15, line: 10, column: 5, end_line: 10, end_column: 12 } },
        /* terrane-site-row: site 121: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:11:22-11:34) */
        { Site { function: 22, file: 16, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 122: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:11:38-11:50) */
        { Site { function: 22, file: 16, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 123: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:14:13-14:26) */
        { Site { function: 22, file: 16, line: 14, column: 13, end_line: 14, end_column: 26 } },
        /* terrane-site-row: site 124: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:15:17-15:24) */
        { Site { function: 22, file: 16, line: 15, column: 17, end_line: 15, end_column: 24 } },
        /* terrane-site-row: site 125: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:16:19-16:31) */
        { Site { function: 22, file: 16, line: 16, column: 19, end_line: 16, end_column: 31 } },
        /* terrane-site-row: site 126: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:6:9-6:21) */
        { Site { function: 23, file: 17, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 127: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:8:26-8:38) */
        { Site { function: 23, file: 17, line: 8, column: 26, end_line: 8, end_column: 38 } },
        /* terrane-site-row: site 128: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:9:13-9:27) */
        { Site { function: 23, file: 17, line: 9, column: 13, end_line: 9, end_column: 27 } },
        /* terrane-site-row: site 129: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:9:13-9:33) */
        { Site { function: 23, file: 17, line: 9, column: 13, end_line: 9, end_column: 33 } },
        /* terrane-site-row: site 130: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:10:5-10:12) */
        { Site { function: 23, file: 17, line: 10, column: 5, end_line: 10, end_column: 12 } },
        /* terrane-site-row: site 131: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:11:9-11:21) */
        { Site { function: 24, file: 18, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 132: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:12:9-12:16) */
        { Site { function: 24, file: 18, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 133: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:13:16-13:23) */
        { Site { function: 24, file: 18, line: 13, column: 16, end_line: 13, end_column: 23 } },
        /* terrane-site-row: site 134: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:14:9-14:21) */
        { Site { function: 24, file: 18, line: 14, column: 9, end_line: 14, end_column: 21 } },
        /* terrane-site-row: site 135: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:16:13-16:25) */
        { Site { function: 24, file: 18, line: 16, column: 13, end_line: 16, end_column: 25 } },
        /* terrane-site-row: site 136: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:11:22-11:34) */
        { Site { function: 25, file: 19, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 137: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:11:38-11:50) */
        { Site { function: 25, file: 19, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 138: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:12:9-12:22) */
        { Site { function: 25, file: 19, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 139: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:14:15-14:22) */
        { Site { function: 25, file: 19, line: 14, column: 15, end_line: 14, end_column: 22 } },
        /* terrane-site-row: site 140: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:15:23-15:35) */
        { Site { function: 25, file: 19, line: 15, column: 23, end_line: 15, end_column: 35 } },
        /* terrane-site-row: site 141: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:11:22-11:34) */
        { Site { function: 26, file: 20, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 142: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:11:38-11:50) */
        { Site { function: 26, file: 20, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 143: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:12:9-12:22) */
        { Site { function: 26, file: 20, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 144: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:16:23-16:35) */
        { Site { function: 26, file: 20, line: 16, column: 23, end_line: 16, end_column: 35 } },
        /* terrane-site-row: site 145: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:18:21-18:28) */
        { Site { function: 26, file: 20, line: 18, column: 21, end_line: 18, end_column: 28 } },
        /* terrane-site-row: site 146: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:11:22-11:34) */
        { Site { function: 27, file: 21, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 147: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:11:38-11:50) */
        { Site { function: 27, file: 21, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 148: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:12:9-12:22) */
        { Site { function: 27, file: 21, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 149: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:16:23-16:35) */
        { Site { function: 27, file: 21, line: 16, column: 23, end_line: 16, end_column: 35 } },
        /* terrane-site-row: site 150: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:20:21-20:28) */
        { Site { function: 27, file: 21, line: 20, column: 21, end_line: 20, end_column: 28 } },
        /* terrane-site-row: site 151: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:11:9-11:21) */
        { Site { function: 28, file: 22, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 152: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:12:9-12:21) */
        { Site { function: 28, file: 22, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 153: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:13:9-13:16) */
        { Site { function: 28, file: 22, line: 13, column: 9, end_line: 13, end_column: 16 } },
        /* terrane-site-row: site 154: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:14:9-14:16) */
        { Site { function: 28, file: 22, line: 14, column: 9, end_line: 14, end_column: 16 } },
        /* terrane-site-row: site 155: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:15:9-15:24) */
        { Site { function: 28, file: 22, line: 15, column: 9, end_line: 15, end_column: 24 } },
        /* terrane-site-row: site 156: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:16:5-16:12) */
        { Site { function: 28, file: 22, line: 16, column: 5, end_line: 16, end_column: 12 } },
        /* terrane-site-row: site 157: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:17:13-17:20) */
        { Site { function: 28, file: 22, line: 17, column: 13, end_line: 17, end_column: 20 } },
        /* terrane-site-row: site 158: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:11:22-11:34) */
        { Site { function: 29, file: 23, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 159: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:11:38-11:50) */
        { Site { function: 29, file: 23, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 160: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:12:9-12:22) */
        { Site { function: 29, file: 23, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 161: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:13:13-13:20) */
        { Site { function: 29, file: 23, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 162: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:14:15-14:27) */
        { Site { function: 29, file: 23, line: 14, column: 15, end_line: 14, end_column: 27 } },
        /* terrane-site-row: site 163: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:15:9-15:22) */
        { Site { function: 29, file: 23, line: 15, column: 9, end_line: 15, end_column: 22 } },
        /* terrane-site-row: site 164: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:16:13-16:20) */
        { Site { function: 29, file: 23, line: 16, column: 13, end_line: 16, end_column: 20 } },
        /* terrane-site-row: site 165: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:11:22-11:34) */
        { Site { function: 30, file: 24, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 166: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:11:38-11:50) */
        { Site { function: 30, file: 24, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 167: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:12:9-12:22) */
        { Site { function: 30, file: 24, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 168: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:13:13-13:20) */
        { Site { function: 30, file: 24, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 169: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:14:15-14:27) */
        { Site { function: 30, file: 24, line: 14, column: 15, end_line: 14, end_column: 27 } },
        /* terrane-site-row: site 170: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:15:9-15:22) */
        { Site { function: 30, file: 24, line: 15, column: 9, end_line: 15, end_column: 22 } },
        /* terrane-site-row: site 171: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:16:13-16:20) */
        { Site { function: 30, file: 24, line: 16, column: 13, end_line: 16, end_column: 20 } },
        /* terrane-site-row: site 172: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:11:22-11:34) */
        { Site { function: 31, file: 25, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 173: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:11:38-11:50) */
        { Site { function: 31, file: 25, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 174: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:12:9-12:22) */
        { Site { function: 31, file: 25, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 175: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:15:19-15:31) */
        { Site { function: 31, file: 25, line: 15, column: 19, end_line: 15, end_column: 31 } },
        /* terrane-site-row: site 176: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:17:13-17:20) */
        { Site { function: 31, file: 25, line: 17, column: 13, end_line: 17, end_column: 20 } },
        /* terrane-site-row: site 177: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:4:9-4:21) */
        { Site { function: 32, file: 26, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 178: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:5:9-5:21) */
        { Site { function: 32, file: 26, line: 5, column: 9, end_line: 5, end_column: 21 } },
        /* terrane-site-row: site 179: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:6:9-6:16) */
        { Site { function: 32, file: 26, line: 6, column: 9, end_line: 6, end_column: 16 } },
        /* terrane-site-row: site 180: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:8:9-8:16) */
        { Site { function: 32, file: 26, line: 8, column: 9, end_line: 8, end_column: 16 } },
        /* terrane-site-row: site 181: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:9:13-9:20) */
        { Site { function: 32, file: 26, line: 9, column: 13, end_line: 9, end_column: 20 } },
        /* terrane-site-row: site 182: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:10:13-10:20) */
        { Site { function: 32, file: 26, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 183: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:11:5-11:12) */
        { Site { function: 32, file: 26, line: 11, column: 5, end_line: 11, end_column: 12 } },
        /* terrane-site-row: site 184: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:12:13-12:25) */
        { Site { function: 32, file: 26, line: 12, column: 13, end_line: 12, end_column: 25 } },
        /* terrane-site-row: site 185: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:11:9-11:21) */
        { Site { function: 33, file: 27, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 186: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:12:9-12:16) */
        { Site { function: 33, file: 27, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 187: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:14:17-14:24) */
        { Site { function: 33, file: 27, line: 14, column: 17, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 188: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:16:9-16:16) */
        { Site { function: 33, file: 27, line: 16, column: 9, end_line: 16, end_column: 16 } },
        /* terrane-site-row: site 189: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:17:13-17:25) */
        { Site { function: 33, file: 27, line: 17, column: 13, end_line: 17, end_column: 25 } },
        /* terrane-site-row: site 190: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:11:9-11:21) */
        { Site { function: 34, file: 28, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 191: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:13:13-13:20) */
        { Site { function: 34, file: 28, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 192: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:14:17-14:24) */
        { Site { function: 34, file: 28, line: 14, column: 17, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 193: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:16:17-16:29) */
        { Site { function: 34, file: 28, line: 16, column: 17, end_line: 16, end_column: 29 } },
        /* terrane-site-row: site 194: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:17:13-17:25) */
        { Site { function: 34, file: 28, line: 17, column: 13, end_line: 17, end_column: 25 } },
        /* terrane-site-row: site 195: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:11:9-11:21) */
        { Site { function: 35, file: 29, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 196: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:12:9-12:16) */
        { Site { function: 35, file: 29, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 197: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:13:13-13:20) */
        { Site { function: 35, file: 29, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 198: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:14:9-14:21) */
        { Site { function: 35, file: 29, line: 14, column: 9, end_line: 14, end_column: 21 } },
        /* terrane-site-row: site 199: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:15:13-15:25) */
        { Site { function: 35, file: 29, line: 15, column: 13, end_line: 15, end_column: 25 } },
        /* terrane-site-row: site 200: /native-loan-whole-move-last-use::test-scenario (tests/integration/native-loan-whole-move-last-use/case.trn:11:9-11:21) */
        { Site { function: 36, file: 30, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 201: /native-loan-whole-move-last-use::test-scenario (tests/integration/native-loan-whole-move-last-use/case.trn:12:9-12:16) */
        { Site { function: 36, file: 30, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 202: /native-loan-whole-move-last-use::test-scenario (tests/integration/native-loan-whole-move-last-use/case.trn:13:13-13:20) */
        { Site { function: 36, file: 30, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 203: /native-loan-whole-move-last-use::test-scenario (tests/integration/native-loan-whole-move-last-use/case.trn:15:13-15:29) */
        { Site { function: 36, file: 30, line: 15, column: 13, end_line: 15, end_column: 29 } },
        /* terrane-site-row: site 204: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:11:9-11:21) */
        { Site { function: 37, file: 31, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 205: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:12:9-12:16) */
        { Site { function: 37, file: 31, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 206: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:13:13-13:20) */
        { Site { function: 37, file: 31, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 207: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:14:5-14:12) */
        { Site { function: 37, file: 31, line: 14, column: 5, end_line: 14, end_column: 12 } },
        /* terrane-site-row: site 208: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:15:13-15:25) */
        { Site { function: 37, file: 31, line: 15, column: 13, end_line: 15, end_column: 25 } },
        /* terrane-site-row: site 209: /native-loop-consuming-condition-break::test-scenario (tests/integration/native-loop-consuming-condition-break/case.trn:11:9-11:21) */
        { Site { function: 38, file: 32, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 210: /native-loop-consuming-condition-break::test-scenario (tests/integration/native-loop-consuming-condition-break/case.trn:12:9-12:16) */
        { Site { function: 38, file: 32, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 211: /native-loop-consuming-condition-break::test-scenario (tests/integration/native-loop-consuming-condition-break/case.trn:13:12-13:24) */
        { Site { function: 38, file: 32, line: 13, column: 12, end_line: 13, end_column: 24 } },
        /* terrane-site-row: site 212: /native-noncopy-view-loop-append::test-scenario (tests/integration/native-noncopy-view-loop-append/case.trn:6:14-6:26) */
        { Site { function: 39, file: 33, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 213: /native-noncopy-view-loop-append::test-scenario (tests/integration/native-noncopy-view-loop-append/case.trn:7:12-7:24) */
        { Site { function: 39, file: 33, line: 7, column: 12, end_line: 7, end_column: 24 } },
        /* terrane-site-row: site 214: /native-noncopy-view-loop-append::test-scenario (tests/integration/native-noncopy-view-loop-append/case.trn:11:16-11:28) */
        { Site { function: 39, file: 33, line: 11, column: 16, end_line: 11, end_column: 28 } },
        /* terrane-site-row: site 215: /native-noncopy-view-loop-append::test-scenario (tests/integration/native-noncopy-view-loop-append/case.trn:14:17-14:28) */
        { Site { function: 39, file: 33, line: 14, column: 17, end_line: 14, end_column: 28 } },
        /* terrane-site-row: site 216: /native-ref-clone-repeat::twice (tests/integration/native-ref-clone-repeat/case.trn:4:13-4:24) */
        { Site { function: 40, file: 34, line: 4, column: 13, end_line: 4, end_column: 24 } },
        /* terrane-site-row: site 217: /native-ref-clone-repeat::twice (tests/integration/native-ref-clone-repeat/case.trn:5:14-5:25) */
        { Site { function: 40, file: 34, line: 5, column: 14, end_line: 5, end_column: 25 } },
        /* terrane-site-row: site 218: /native-ref-clone-repeat::twice (tests/integration/native-ref-clone-repeat/case.trn:6:12-6:26) */
        { Site { function: 40, file: 34, line: 6, column: 12, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 219: /native-ref-clone-repeat::copied (tests/integration/native-ref-clone-repeat/case.trn:8:13-8:25) */
        { Site { function: 41, file: 34, line: 8, column: 13, end_line: 8, end_column: 25 } },
        /* terrane-site-row: site 220: /native-ref-clone-repeat::copied (tests/integration/native-ref-clone-repeat/case.trn:9:21-9:33) */
        { Site { function: 41, file: 34, line: 9, column: 21, end_line: 9, end_column: 33 } },
        /* terrane-site-row: site 221: /native-ref-clone-repeat::copied (tests/integration/native-ref-clone-repeat/case.trn:9:12-9:34) */
        { Site { function: 41, file: 34, line: 9, column: 12, end_line: 9, end_column: 34 } },
        /* terrane-site-row: site 222: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:11:13-11:26) */
        { Site { function: 42, file: 34, line: 11, column: 13, end_line: 11, end_column: 26 } },
        /* terrane-site-row: site 223: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:12:13-12:25) */
        { Site { function: 42, file: 34, line: 12, column: 13, end_line: 12, end_column: 25 } },
        /* terrane-site-row: site 224: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:13:13-13:24) */
        { Site { function: 42, file: 34, line: 13, column: 13, end_line: 13, end_column: 24 } },
        /* terrane-site-row: site 225: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:14:13-14:24) */
        { Site { function: 42, file: 34, line: 14, column: 13, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 226: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:15:13-15:26) */
        { Site { function: 42, file: 34, line: 15, column: 13, end_line: 15, end_column: 26 } },
        /* terrane-site-row: site 227: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:16:13-16:25) */
        { Site { function: 42, file: 34, line: 16, column: 13, end_line: 16, end_column: 25 } },
        /* terrane-site-row: site 228: /native-required-enum-field::construct (tests/integration/native-required-enum-field/case.trn:7:22-7:28) */
        { Site { function: 43, file: 35, line: 7, column: 22, end_line: 7, end_column: 28 } },
        /* terrane-site-row: site 229: /native-required-enum-field::finish (tests/integration/native-required-enum-field/case.trn:9:16-9:32) */
        { Site { function: 44, file: 35, line: 9, column: 16, end_line: 9, end_column: 32 } },
        /* terrane-site-row: site 230: /native-required-enum-field::test-scenario (tests/integration/native-required-enum-field/case.trn:16:13-16:30) */
        { Site { function: 45, file: 35, line: 16, column: 13, end_line: 16, end_column: 30 } },
        /* terrane-site-row: site 231: /native-required-enum-field::test-scenario (tests/integration/native-required-enum-field/case.trn:19:13-19:36) */
        { Site { function: 45, file: 35, line: 19, column: 13, end_line: 19, end_column: 36 } },
        /* terrane-site-row: site 232: /native-required-enum-field::test-scenario (tests/integration/native-required-enum-field/case.trn:22:13-22:26) */
        { Site { function: 45, file: 35, line: 22, column: 13, end_line: 22, end_column: 26 } },
        /* terrane-site-row: site 233: /native-return-parameter-single::f (tests/integration/native-return-parameter-single/case.trn:5:12-5:24) */
        { Site { function: 46, file: 36, line: 5, column: 12, end_line: 5, end_column: 24 } },
        /* terrane-site-row: site 234: /native-return-parameter-single::test-scenario (tests/integration/native-return-parameter-single/case.trn:8:9-8:21) */
        { Site { function: 47, file: 36, line: 8, column: 9, end_line: 8, end_column: 21 } },
        /* terrane-site-row: site 235: /native-return-parameter-single::test-scenario (tests/integration/native-return-parameter-single/case.trn:10:13-10:20) */
        { Site { function: 47, file: 36, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 236: /native-return-parameter-single::test-scenario (tests/integration/native-return-parameter-single/case.trn:11:5-11:12) */
        { Site { function: 47, file: 36, line: 11, column: 5, end_line: 11, end_column: 12 } },
        /* terrane-site-row: site 237: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:4:9-4:21) */
        { Site { function: 48, file: 37, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 238: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:5:9-5:16) */
        { Site { function: 48, file: 37, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 239: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:7:17-7:29) */
        { Site { function: 48, file: 37, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 240: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:8:13-8:20) */
        { Site { function: 48, file: 37, line: 8, column: 13, end_line: 8, end_column: 20 } },
        /* terrane-site-row: site 241: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:9:17-9:24) */
        { Site { function: 48, file: 37, line: 9, column: 17, end_line: 9, end_column: 24 } },
        /* terrane-site-row: site 242: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:4:9-4:21) */
        { Site { function: 49, file: 38, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 243: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:5:9-5:16) */
        { Site { function: 49, file: 38, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 244: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:7:17-7:29) */
        { Site { function: 49, file: 38, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 245: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:8:13-8:20) */
        { Site { function: 49, file: 38, line: 8, column: 13, end_line: 8, end_column: 20 } },
        /* terrane-site-row: site 246: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:10:13-10:20) */
        { Site { function: 49, file: 38, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 247: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:11:17-11:24) */
        { Site { function: 49, file: 38, line: 11, column: 17, end_line: 11, end_column: 24 } },
        /* terrane-site-row: site 248: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:4:9-4:21) */
        { Site { function: 50, file: 39, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 249: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:5:9-5:21) */
        { Site { function: 50, file: 39, line: 5, column: 9, end_line: 5, end_column: 21 } },
        /* terrane-site-row: site 250: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:6:9-6:21) */
        { Site { function: 50, file: 39, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 251: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:8:17-8:24) */
        { Site { function: 50, file: 39, line: 8, column: 17, end_line: 8, end_column: 24 } },
        /* terrane-site-row: site 252: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:9:13-9:25) */
        { Site { function: 50, file: 39, line: 9, column: 13, end_line: 9, end_column: 25 } },
        /* terrane-site-row: site 253: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:11:17-11:24) */
        { Site { function: 50, file: 39, line: 11, column: 17, end_line: 11, end_column: 24 } },
        /* terrane-site-row: site 254: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:12:13-12:20) */
        { Site { function: 50, file: 39, line: 12, column: 13, end_line: 12, end_column: 20 } },
        /* terrane-site-row: site 255: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:4:9-4:21) */
        { Site { function: 51, file: 40, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 256: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:5:9-5:16) */
        { Site { function: 51, file: 40, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 257: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:7:17-7:29) */
        { Site { function: 51, file: 40, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 258: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:8:13-8:20) */
        { Site { function: 51, file: 40, line: 8, column: 13, end_line: 8, end_column: 20 } },
        /* terrane-site-row: site 259: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:10:13-10:20) */
        { Site { function: 51, file: 40, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 260: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:11:17-11:24) */
        { Site { function: 51, file: 40, line: 11, column: 17, end_line: 11, end_column: 24 } },
        /* terrane-site-row: site 261: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:4:9-4:21) */
        { Site { function: 52, file: 41, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 262: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:5:9-5:16) */
        { Site { function: 52, file: 41, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 263: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:9:17-9:29) */
        { Site { function: 52, file: 41, line: 9, column: 17, end_line: 9, end_column: 29 } },
        /* terrane-site-row: site 264: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:10:13-10:20) */
        { Site { function: 52, file: 41, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 265: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:11:17-11:24) */
        { Site { function: 52, file: 41, line: 11, column: 17, end_line: 11, end_column: 24 } },
        /* terrane-site-row: site 266: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:4:9-4:21) */
        { Site { function: 53, file: 42, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 267: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:6:13-6:20) */
        { Site { function: 53, file: 42, line: 6, column: 13, end_line: 6, end_column: 20 } },
        /* terrane-site-row: site 268: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:7:17-7:29) */
        { Site { function: 53, file: 42, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 269: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:13:20-13:27) */
        { Site { function: 53, file: 42, line: 13, column: 20, end_line: 13, end_column: 27 } },
        /* terrane-site-row: site 270: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:14:21-14:36) */
        { Site { function: 53, file: 42, line: 14, column: 21, end_line: 14, end_column: 36 } },
        /* terrane-site-row: site 271: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:4:9-4:21) */
        { Site { function: 54, file: 43, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 272: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:5:9-5:21) */
        { Site { function: 54, file: 43, line: 5, column: 9, end_line: 5, end_column: 21 } },
        /* terrane-site-row: site 273: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:6:9-6:16) */
        { Site { function: 54, file: 43, line: 6, column: 9, end_line: 6, end_column: 16 } },
        /* terrane-site-row: site 274: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:8:17-8:29) */
        { Site { function: 54, file: 43, line: 8, column: 17, end_line: 8, end_column: 29 } },
        /* terrane-site-row: site 275: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:9:13-9:20) */
        { Site { function: 54, file: 43, line: 9, column: 13, end_line: 9, end_column: 20 } },
        /* terrane-site-row: site 276: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:12:9-12:16) */
        { Site { function: 54, file: 43, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 277: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:13:13-13:20) */
        { Site { function: 54, file: 43, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 278: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:6:14-6:26) */
        { Site { function: 55, file: 44, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 279: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:7:17-7:29) */
        { Site { function: 55, file: 44, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 280: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:9:26-9:43) */
        { Site { function: 55, file: 44, line: 9, column: 26, end_line: 9, end_column: 43 } },
        /* terrane-site-row: site 281: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:11:17-11:28) */
        { Site { function: 55, file: 44, line: 11, column: 17, end_line: 11, end_column: 28 } },
        /* terrane-site-row: site 282: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:12:9-12:24) */
        { Site { function: 55, file: 44, line: 12, column: 9, end_line: 12, end_column: 24 } },
        /* terrane-site-row: site 283: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:13:5-13:17) */
        { Site { function: 55, file: 44, line: 13, column: 5, end_line: 13, end_column: 17 } },
        /* terrane-site-row: site 284: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:12:9-12:21) */
        { Site { function: 56, file: 45, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 285: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:13:9-13:16) */
        { Site { function: 56, file: 45, line: 13, column: 9, end_line: 13, end_column: 16 } },
        /* terrane-site-row: site 286: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:14:21-14:28) */
        { Site { function: 56, file: 45, line: 14, column: 21, end_line: 14, end_column: 28 } },
        /* terrane-site-row: site 287: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:16:9-16:16) */
        { Site { function: 56, file: 45, line: 16, column: 9, end_line: 16, end_column: 16 } },
        /* terrane-site-row: site 288: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:17:13-17:20) */
        { Site { function: 56, file: 45, line: 17, column: 13, end_line: 17, end_column: 20 } },
        /* terrane-site-row: site 289: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:19:22-19:34) */
        { Site { function: 56, file: 45, line: 19, column: 22, end_line: 19, end_column: 34 } },
        /* terrane-site-row: site 290: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:20:15-20:35) */
        { Site { function: 56, file: 45, line: 20, column: 15, end_line: 20, end_column: 35 } },
        /* terrane-site-row: site 291: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:21:27-21:47) */
        { Site { function: 56, file: 45, line: 21, column: 27, end_line: 21, end_column: 47 } },
        /* terrane-site-row: site 292: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:23:15-23:35) */
        { Site { function: 56, file: 45, line: 23, column: 15, end_line: 23, end_column: 35 } },
        /* terrane-site-row: site 293: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:24:9-24:21) */
        { Site { function: 56, file: 45, line: 24, column: 9, end_line: 24, end_column: 21 } },
        /* terrane-site-row: site 294: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:25:11-25:18) */
        { Site { function: 56, file: 45, line: 25, column: 11, end_line: 25, end_column: 18 } },
        /* terrane-site-row: site 295: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:26:27-26:34) */
        { Site { function: 56, file: 45, line: 26, column: 27, end_line: 26, end_column: 34 } },
        /* terrane-site-row: site 296: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:28:13-28:25) */
        { Site { function: 56, file: 45, line: 28, column: 13, end_line: 28, end_column: 25 } },
        /* terrane-site-row: site 297: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:29:11-29:22) */
        { Site { function: 56, file: 45, line: 29, column: 11, end_line: 29, end_column: 22 } },
        /* terrane-site-row: site 298: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:31:13-31:20) */
        { Site { function: 56, file: 45, line: 31, column: 13, end_line: 31, end_column: 20 } },
        /* terrane-site-row: site 299: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:33:13-33:20) */
        { Site { function: 56, file: 45, line: 33, column: 13, end_line: 33, end_column: 20 } },
        /* terrane-site-row: site 300: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:35:19-35:26) */
        { Site { function: 56, file: 45, line: 35, column: 19, end_line: 35, end_column: 26 } },
        /* terrane-site-row: site 301: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:37:19-37:30) */
        { Site { function: 56, file: 45, line: 37, column: 19, end_line: 37, end_column: 30 } },
        /* terrane-site-row: site 302: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:39:20-39:32) */
        { Site { function: 56, file: 45, line: 39, column: 20, end_line: 39, end_column: 32 } },
        /* terrane-site-row: site 303: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:40:20-40:32) */
        { Site { function: 56, file: 45, line: 40, column: 20, end_line: 40, end_column: 32 } },
        /* terrane-site-row: site 304: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:41:17-41:29) */
        { Site { function: 56, file: 45, line: 41, column: 17, end_line: 41, end_column: 29 } },
        /* terrane-site-row: site 305: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:42:18-42:41) */
        { Site { function: 56, file: 45, line: 42, column: 18, end_line: 42, end_column: 41 } },
        /* terrane-site-row: site 306: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:43:19-43:42) */
        { Site { function: 56, file: 45, line: 43, column: 19, end_line: 43, end_column: 42 } },
        /* terrane-site-row: site 307: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:45:22-45:45) */
        { Site { function: 56, file: 45, line: 45, column: 22, end_line: 45, end_column: 45 } },
        /* terrane-site-row: site 308: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:46:9-46:28) */
        { Site { function: 56, file: 45, line: 46, column: 9, end_line: 46, end_column: 28 } },
        /* terrane-site-row: site 309: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:49:5-49:20) */
        { Site { function: 56, file: 45, line: 49, column: 5, end_line: 49, end_column: 20 } },
        /* terrane-site-row: site 310: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:50:13-50:30) */
        { Site { function: 56, file: 45, line: 50, column: 13, end_line: 50, end_column: 30 } },
        /* terrane-site-row: site 311: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:51:19-51:31) */
        { Site { function: 56, file: 45, line: 51, column: 19, end_line: 51, end_column: 31 } },
        /* terrane-site-row: site 312: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:52:14-52:26) */
        { Site { function: 56, file: 45, line: 52, column: 14, end_line: 52, end_column: 26 } },
        /* terrane-site-row: site 313: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:53:18-53:30) */
        { Site { function: 56, file: 45, line: 53, column: 18, end_line: 53, end_column: 30 } },
        /* terrane-site-row: site 314: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:54:17-54:39) */
        { Site { function: 56, file: 45, line: 54, column: 17, end_line: 54, end_column: 39 } },
        /* terrane-site-row: site 315: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:55:18-55:40) */
        { Site { function: 56, file: 45, line: 55, column: 18, end_line: 55, end_column: 40 } },
        /* terrane-site-row: site 316: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:57:21-57:38) */
        { Site { function: 56, file: 45, line: 57, column: 21, end_line: 57, end_column: 38 } },
        /* terrane-site-row: site 317: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:60:5-60:21) */
        { Site { function: 56, file: 45, line: 60, column: 5, end_line: 60, end_column: 21 } },
        /* terrane-site-row: site 318: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:61:13-61:29) */
        { Site { function: 56, file: 45, line: 61, column: 13, end_line: 61, end_column: 29 } },
        /* terrane-site-row: site 319: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:6:14-6:26) */
        { Site { function: 57, file: 46, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 320: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:7:21-7:38) */
        { Site { function: 57, file: 46, line: 7, column: 21, end_line: 7, end_column: 38 } },
        /* terrane-site-row: site 321: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:9:17-9:28) */
        { Site { function: 57, file: 46, line: 9, column: 17, end_line: 9, end_column: 28 } },
        /* terrane-site-row: site 322: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:11:17-11:28) */
        { Site { function: 57, file: 46, line: 11, column: 17, end_line: 11, end_column: 28 } },
        /* terrane-site-row: site 323: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:13:17-13:28) */
        { Site { function: 57, file: 46, line: 13, column: 17, end_line: 13, end_column: 28 } },
        /* terrane-site-row: site 324: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:15:17-15:28) */
        { Site { function: 57, file: 46, line: 15, column: 17, end_line: 15, end_column: 28 } },
        /* terrane-site-row: site 325: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:18:21-18:32) */
        { Site { function: 57, file: 46, line: 18, column: 21, end_line: 18, end_column: 32 } },
        /* terrane-site-row: site 326: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:19:17-19:28) */
        { Site { function: 57, file: 46, line: 19, column: 17, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 327: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:6:14-6:26) */
        { Site { function: 58, file: 47, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 328: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:8:30-8:47) */
        { Site { function: 58, file: 47, line: 8, column: 30, end_line: 8, end_column: 47 } },
        /* terrane-site-row: site 329: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:10:17-10:28) */
        { Site { function: 58, file: 47, line: 10, column: 17, end_line: 10, end_column: 28 } },
        /* terrane-site-row: site 330: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:13:34-13:51) */
        { Site { function: 58, file: 47, line: 13, column: 34, end_line: 13, end_column: 51 } },
        /* terrane-site-row: site 331: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:15:17-15:28) */
        { Site { function: 58, file: 47, line: 15, column: 17, end_line: 15, end_column: 28 } },
        /* terrane-site-row: site 332: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:18:35-18:52) */
        { Site { function: 58, file: 47, line: 18, column: 35, end_line: 18, end_column: 52 } },
        /* terrane-site-row: site 333: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:23:21-23:32) */
        { Site { function: 58, file: 47, line: 23, column: 21, end_line: 23, end_column: 32 } },
        /* terrane-site-row: site 334: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:26:37-26:54) */
        { Site { function: 58, file: 47, line: 26, column: 37, end_line: 26, end_column: 54 } },
        /* terrane-site-row: site 335: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:31:21-31:32) */
        { Site { function: 58, file: 47, line: 31, column: 21, end_line: 31, end_column: 32 } },
        /* terrane-site-row: site 336: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:34:17-34:28) */
        { Site { function: 58, file: 47, line: 34, column: 17, end_line: 34, end_column: 28 } },
        /* terrane-site-row: site 337: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:35:32-35:49) */
        { Site { function: 58, file: 47, line: 35, column: 32, end_line: 35, end_column: 49 } },
        /* terrane-site-row: site 338: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:6:14-6:26) */
        { Site { function: 59, file: 48, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 339: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:7:40-7:57) */
        { Site { function: 59, file: 48, line: 7, column: 40, end_line: 7, end_column: 57 } },
        /* terrane-site-row: site 340: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:9:17-9:28) */
        { Site { function: 59, file: 48, line: 9, column: 17, end_line: 9, end_column: 28 } },
        /* terrane-site-row: site 341: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:12:17-12:28) */
        { Site { function: 59, file: 48, line: 12, column: 17, end_line: 12, end_column: 28 } },
        /* terrane-site-row: site 342: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:13:24-13:41) */
        { Site { function: 59, file: 48, line: 13, column: 24, end_line: 13, end_column: 41 } },
        /* terrane-site-row: site 343: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:15:17-15:28) */
        { Site { function: 59, file: 48, line: 15, column: 17, end_line: 15, end_column: 28 } },
        /* terrane-site-row: site 344: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:17:17-17:28) */
        { Site { function: 59, file: 48, line: 17, column: 17, end_line: 17, end_column: 28 } },
        /* terrane-site-row: site 345: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:18:27-18:44) */
        { Site { function: 59, file: 48, line: 18, column: 27, end_line: 18, end_column: 44 } },
        /* terrane-site-row: site 346: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:19:17-19:28) */
        { Site { function: 59, file: 48, line: 19, column: 17, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 347: /native-view-value-iteration::observe (tests/integration/native-view-value-iteration/case.trn:7:17-7:28) */
        { Site { function: 60, file: 49, line: 7, column: 17, end_line: 7, end_column: 28 } },
        /* terrane-site-row: site 348: /native-view-value-iteration::observe-ref (tests/integration/native-view-value-iteration/case.trn:12:17-12:28) */
        { Site { function: 61, file: 49, line: 12, column: 17, end_line: 12, end_column: 28 } },
        /* terrane-site-row: site 349: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:16:14-16:26) */
        { Site { function: 62, file: 49, line: 16, column: 14, end_line: 16, end_column: 26 } },
        /* terrane-site-row: site 350: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:17:37-17:54) */
        { Site { function: 62, file: 49, line: 17, column: 37, end_line: 17, end_column: 54 } },
        /* terrane-site-row: site 351: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:19:17-19:28) */
        { Site { function: 62, file: 49, line: 19, column: 17, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 352: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:23:17-23:28) */
        { Site { function: 62, file: 49, line: 23, column: 17, end_line: 23, end_column: 28 } },
        /* terrane-site-row: site 353: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:24:23-24:40) */
        { Site { function: 62, file: 49, line: 24, column: 23, end_line: 24, end_column: 40 } },
        /* terrane-site-row: site 354: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:26:17-26:28) */
        { Site { function: 62, file: 49, line: 26, column: 17, end_line: 26, end_column: 28 } },
        /* terrane-site-row: site 355: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:30:27-30:44) */
        { Site { function: 62, file: 49, line: 30, column: 27, end_line: 30, end_column: 44 } },
        /* terrane-site-row: site 356: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:31:17-31:28) */
        { Site { function: 62, file: 49, line: 31, column: 17, end_line: 31, end_column: 28 } },
        /* terrane-site-row: site 357: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:36:24-36:41) */
        { Site { function: 62, file: 49, line: 36, column: 24, end_line: 36, end_column: 41 } },
        /* terrane-site-row: site 358: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:37:17-37:28) */
        { Site { function: 62, file: 49, line: 37, column: 17, end_line: 37, end_column: 28 } },
        /* terrane-site-row: site 359: /nested-noncopy-view-loops::test-scenario (tests/integration/nested-noncopy-view-loops/case.trn:6:14-6:26) */
        { Site { function: 63, file: 50, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 360: /nested-noncopy-view-loops::test-scenario (tests/integration/nested-noncopy-view-loops/case.trn:7:28-7:40) */
        { Site { function: 63, file: 50, line: 7, column: 28, end_line: 7, end_column: 40 } },
        /* terrane-site-row: site 361: /nested-noncopy-view-loops::test-scenario (tests/integration/nested-noncopy-view-loops/case.trn:10:21-10:32) */
        { Site { function: 63, file: 50, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 362: /nested-view-loops::test-scenario (tests/integration/nested-view-loops/case.trn:6:14-6:26) */
        { Site { function: 64, file: 51, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 363: /nested-view-loops::test-scenario (tests/integration/nested-view-loops/case.trn:7:28-7:45) */
        { Site { function: 64, file: 51, line: 7, column: 28, end_line: 7, end_column: 45 } },
        /* terrane-site-row: site 364: /nested-view-loops::test-scenario (tests/integration/nested-view-loops/case.trn:10:21-10:32) */
        { Site { function: 64, file: 51, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 365: /nested-view-ref-value-loop::test-scenario (tests/integration/nested-view-ref-value-loop/case.trn:6:14-6:26) */
        { Site { function: 65, file: 52, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 366: /nested-view-ref-value-loop::test-scenario (tests/integration/nested-view-ref-value-loop/case.trn:7:28-7:45) */
        { Site { function: 65, file: 52, line: 7, column: 28, end_line: 7, end_column: 45 } },
        /* terrane-site-row: site 367: /nested-view-ref-value-loop::test-scenario (tests/integration/nested-view-ref-value-loop/case.trn:10:21-10:32) */
        { Site { function: 65, file: 52, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 368: /nested-view-value-ref-loop::test-scenario (tests/integration/nested-view-value-ref-loop/case.trn:6:14-6:26) */
        { Site { function: 66, file: 53, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 369: /nested-view-value-ref-loop::test-scenario (tests/integration/nested-view-value-ref-loop/case.trn:7:28-7:45) */
        { Site { function: 66, file: 53, line: 7, column: 28, end_line: 7, end_column: 45 } },
        /* terrane-site-row: site 370: /nested-view-value-ref-loop::test-scenario (tests/integration/nested-view-value-ref-loop/case.trn:10:21-10:32) */
        { Site { function: 66, file: 53, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 371: /nested-view-value-value-loop::test-scenario (tests/integration/nested-view-value-value-loop/case.trn:6:14-6:26) */
        { Site { function: 67, file: 54, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 372: /nested-view-value-value-loop::test-scenario (tests/integration/nested-view-value-value-loop/case.trn:7:28-7:45) */
        { Site { function: 67, file: 54, line: 7, column: 28, end_line: 7, end_column: 45 } },
        /* terrane-site-row: site 373: /nested-view-value-value-loop::test-scenario (tests/integration/nested-view-value-value-loop/case.trn:10:21-10:32) */
        { Site { function: 67, file: 54, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 374: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:18:14-18:26) */
        { Site { function: 68, file: 55, line: 18, column: 14, end_line: 18, end_column: 26 } },
        /* terrane-site-row: site 375: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:19:20-19:37) */
        { Site { function: 68, file: 55, line: 19, column: 20, end_line: 19, end_column: 37 } },
        /* terrane-site-row: site 376: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:21:17-21:27) */
        { Site { function: 68, file: 55, line: 21, column: 17, end_line: 21, end_column: 27 } },
        /* terrane-site-row: site 377: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:24:21-24:33) */
        { Site { function: 68, file: 55, line: 24, column: 21, end_line: 24, end_column: 33 } },
        /* terrane-site-row: site 378: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:25:20-25:37) */
        { Site { function: 68, file: 55, line: 25, column: 20, end_line: 25, end_column: 37 } },
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
// Source: <terrane>/projected/deps/terrane-query-witness.trn
// Namespace: deps/terrane-query-witness
pub use terrane_query_witness::Cl;
pub use terrane_query_witness::CloneEnum;
pub type CopyView<'a> = terrane_query_witness::CopyView<'a>;
pub use terrane_query_witness::Holder;
pub use terrane_query_witness::NE;
pub use terrane_query_witness::Token;
pub type View<'a> = terrane_query_witness::View<'a>;
pub fn terrane_static_trn_436c_new() -> Result<Cl, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::Cl::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::Cl",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_486f6c646572_new() -> Result<
    Holder,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::Holder::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::Holder",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_546f6b656e_new() -> Result<Token, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::Token::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::Token",
                ),
            )
        }
    }
}
pub fn query<'view>(
    sql: &'view str,
) -> Result<terrane_query_witness::View<'view>, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::query(sql)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::query",
                ),
            )
        }
    }
}
