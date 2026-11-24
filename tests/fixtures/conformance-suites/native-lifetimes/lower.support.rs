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
    pub static FILES: [&str; 68] = [
        "tests/integration/completion-finally-view-post-read/case.trn",
        "tests/integration/completion-native-catch-post-read/case.trn",
        "tests/integration/completion-native-post-read/case.trn",
        "tests/integration/native-borrowed-bytes/case.trn",
        "tests/integration/native-borrowed-iterator/case.trn",
        "tests/integration/native-borrowed-slice/case.trn",
        "tests/integration/native-checked-ordinal/case.trn",
        "tests/integration/native-copy-view-list-lender/case.trn",
        "tests/integration/native-copy-view-reassignment-snapshot/case.trn",
        "tests/integration/native-field-copy-clone-conversion/case.trn",
        "tests/integration/native-field-text/case.trn",
        "tests/integration/native-finally-only-rewrite/case.trn",
        "tests/integration/native-from-conversion/case.trn",
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
        "tests/integration/native-optional-text/case.trn",
        "tests/integration/native-ref-clone-repeat/case.trn",
        "tests/integration/native-required-enum-field/case.trn",
        "tests/integration/native-return-parameter-single/case.trn",
        "tests/integration/native-scoped-text/case.trn",
        "tests/integration/native-try-body-rewrite-control/case.trn",
        "tests/integration/native-try-catch-dominating-rewrite/case.trn",
        "tests/integration/native-try-copy-finally-control/case.trn",
        "tests/integration/native-try-finally-dominating-rewrite/case.trn",
        "tests/integration/native-try-finally-rewrite-control/case.trn",
        "tests/integration/native-try-local-consume/case.trn",
        "tests/integration/native-try-unconditional-rewrite-control/case.trn",
        "tests/integration/native-typed-metadata/case.trn",
        "tests/integration/native-unordered-map-dead-lender-control/case.trn",
        "tests/integration/native-view-list-rebind/case.trn",
        "tests/integration/native-view-local-map-storage/case.trn",
        "tests/integration/native-view-loop-binders/case.trn",
        "tests/integration/native-view-map-entry-iteration/case.trn",
        "tests/integration/native-view-tuple-iteration/case.trn",
        "tests/integration/native-view-value-iteration/case.trn",
        "tests/integration/nested-noncopy-view-loops/case.trn",
        "tests/integration/nested-view-loops/case.trn",
        "tests/integration/nested-view-ref-value-loop/case.trn",
        "tests/integration/nested-view-value-ref-loop/case.trn",
        "tests/integration/nested-view-value-value-loop/case.trn",
        "tests/integration/projected-deep-rustdoc/case.trn",
        "tests/integration/sequential-pipeline-native-copy/case.trn",
        "tests/integration/shared-loop-read-control/case.trn",
    ];
    pub static FUNCTIONS: [&str; 110] = [
        "/completion-finally-view-post-read::test-scenario",
        "/completion-native-catch-post-read::test-scenario",
        "/completion-native-post-read::test-scenario",
        "/native-borrowed-bytes::test-scenario",
        "/native-borrowed-iterator::nonempty",
        "/native-borrowed-iterator::first-text",
        "/native-borrowed-iterator::first-text-borrowed",
        "/native-borrowed-iterator::first-text-owned",
        "/native-borrowed-iterator::collected-text",
        "/native-borrowed-iterator::mapped-text",
        "/native-borrowed-iterator::entry-text",
        "/native-borrowed-iterator::nonempty-entry",
        "/native-borrowed-iterator::test-scenario",
        "/native-borrowed-slice::title",
        "/native-borrowed-slice::titles",
        "/native-borrowed-slice::test-scenario",
        "/native-checked-ordinal::cell-size",
        "/native-checked-ordinal::test-scenario",
        "/native-copy-view-list-lender::test-scenario",
        "/native-copy-view-reassignment-snapshot::fresh",
        "/native-copy-view-reassignment-snapshot::test-scenario",
        "/native-field-copy-clone-conversion::construct",
        "/native-field-copy-clone-conversion::observe",
        "/native-field-copy-clone-conversion::clone-observe",
        "/native-field-copy-clone-conversion::test-scenario",
        "/native-field-text::original-path",
        "/native-field-text::test-scenario",
        "/native-finally-only-rewrite::test-scenario",
        "/native-from-conversion::borrowed-text",
        "/native-from-conversion::borrowed-native",
        "/native-from-conversion::test-scenario",
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
        "/native-optional-text::status-reason",
        "/native-optional-text::test-scenario",
        "/native-ref-clone-repeat::twice",
        "/native-ref-clone-repeat::copied",
        "/native-ref-clone-repeat::test-scenario",
        "/native-required-enum-field::construct",
        "/native-required-enum-field::finish",
        "/native-required-enum-field::test-scenario",
        "/native-return-parameter-single::f",
        "/native-return-parameter-single::test-scenario",
        "/native-scoped-text::parts-text",
        "/native-scoped-text::target-text",
        "/native-scoped-text::accepted-text",
        "/native-scoped-text::optional-text",
        "/native-scoped-text::test-scenario",
        "/native-try-body-rewrite-control::test-scenario",
        "/native-try-catch-dominating-rewrite::test-scenario",
        "/native-try-copy-finally-control::test-scenario",
        "/native-try-finally-dominating-rewrite::test-scenario",
        "/native-try-finally-rewrite-control::test-scenario",
        "/native-try-local-consume::test-scenario",
        "/native-try-unconditional-rewrite-control::test-scenario",
        "/native-typed-metadata::own-label",
        "/native-typed-metadata::type-name",
        "/native-typed-metadata::snapshot",
        "/native-typed-metadata::snapshot-text",
        "/native-typed-metadata::own-found",
        "/native-typed-metadata::find-text",
        "/native-typed-metadata::find-text-aliased",
        "/native-typed-metadata::test-scenario",
        "/native-unordered-map-dead-lender-control::test-scenario",
        "/native-view-list-rebind::test-scenario",
        "/native-view-local-map-storage::store",
        "/native-view-local-map-storage::test-scenario",
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
        "/projected-deep-rustdoc::read-again",
        "/projected-deep-rustdoc::test-scenario",
        "/sequential-pipeline-native-copy::positive",
        "/sequential-pipeline-native-copy::test-scenario",
        "/shared-loop-read-control::test-scenario",
    ];
    pub static SITES: [Site; 508] = [
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
        /* terrane-site-row: site 14: /native-borrowed-bytes::test-scenario (tests/integration/native-borrowed-bytes/case.trn:6:26-6:38) */
        { Site { function: 3, file: 3, line: 6, column: 26, end_line: 6, end_column: 38 } },
        /* terrane-site-row: site 15: /native-borrowed-bytes::test-scenario (tests/integration/native-borrowed-bytes/case.trn:7:17-7:30) */
        { Site { function: 3, file: 3, line: 7, column: 17, end_line: 7, end_column: 30 } },
        /* terrane-site-row: site 16: /native-borrowed-bytes::test-scenario (tests/integration/native-borrowed-bytes/case.trn:9:42-9:43) */
        { Site { function: 3, file: 3, line: 9, column: 42, end_line: 9, end_column: 43 } },
        /* terrane-site-row: site 17: /native-borrowed-bytes::test-scenario (tests/integration/native-borrowed-bytes/case.trn:9:27-9:43) */
        { Site { function: 3, file: 3, line: 9, column: 27, end_line: 9, end_column: 43 } },
        /* terrane-site-row: site 18: /native-borrowed-bytes::test-scenario (tests/integration/native-borrowed-bytes/case.trn:12:40-12:41) */
        { Site { function: 3, file: 3, line: 12, column: 40, end_line: 12, end_column: 41 } },
        /* terrane-site-row: site 19: /native-borrowed-bytes::test-scenario (tests/integration/native-borrowed-bytes/case.trn:12:25-12:41) */
        { Site { function: 3, file: 3, line: 12, column: 25, end_line: 12, end_column: 41 } },
        /* terrane-site-row: site 20: /native-borrowed-bytes::test-scenario (tests/integration/native-borrowed-bytes/case.trn:15:24-15:25) */
        { Site { function: 3, file: 3, line: 15, column: 24, end_line: 15, end_column: 25 } },
        /* terrane-site-row: site 21: /native-borrowed-bytes::test-scenario (tests/integration/native-borrowed-bytes/case.trn:15:9-15:25) */
        { Site { function: 3, file: 3, line: 15, column: 9, end_line: 15, end_column: 25 } },
        /* terrane-site-row: site 22: /native-borrowed-iterator::nonempty (tests/integration/native-borrowed-iterator/case.trn:5:12-5:36) */
        { Site { function: 4, file: 4, line: 5, column: 12, end_line: 5, end_column: 36 } },
        /* terrane-site-row: site 23: /native-borrowed-iterator::first-text (tests/integration/native-borrowed-iterator/case.trn:11:14-11:29) */
        { Site { function: 5, file: 4, line: 11, column: 14, end_line: 11, end_column: 29 } },
        /* terrane-site-row: site 24: /native-borrowed-iterator::first-text (tests/integration/native-borrowed-iterator/case.trn:11:13-11:36) */
        { Site { function: 5, file: 4, line: 11, column: 13, end_line: 11, end_column: 36 } },
        /* terrane-site-row: site 25: /native-borrowed-iterator::first-text (tests/integration/native-borrowed-iterator/case.trn:11:12-11:56) */
        { Site { function: 5, file: 4, line: 11, column: 12, end_line: 11, end_column: 56 } },
        /* terrane-site-row: site 26: /native-borrowed-iterator::first-text-borrowed (tests/integration/native-borrowed-iterator/case.trn:14:14-14:29) */
        { Site { function: 6, file: 4, line: 14, column: 14, end_line: 14, end_column: 29 } },
        /* terrane-site-row: site 27: /native-borrowed-iterator::first-text-borrowed (tests/integration/native-borrowed-iterator/case.trn:14:13-14:36) */
        { Site { function: 6, file: 4, line: 14, column: 13, end_line: 14, end_column: 36 } },
        /* terrane-site-row: site 28: /native-borrowed-iterator::first-text-borrowed (tests/integration/native-borrowed-iterator/case.trn:14:12-14:56) */
        { Site { function: 6, file: 4, line: 14, column: 12, end_line: 14, end_column: 56 } },
        /* terrane-site-row: site 29: /native-borrowed-iterator::first-text-owned (tests/integration/native-borrowed-iterator/case.trn:17:15-17:33) */
        { Site { function: 7, file: 4, line: 17, column: 15, end_line: 17, end_column: 33 } },
        /* terrane-site-row: site 30: /native-borrowed-iterator::first-text-owned (tests/integration/native-borrowed-iterator/case.trn:18:14-18:29) */
        { Site { function: 7, file: 4, line: 18, column: 14, end_line: 18, end_column: 29 } },
        /* terrane-site-row: site 31: /native-borrowed-iterator::first-text-owned (tests/integration/native-borrowed-iterator/case.trn:18:13-18:36) */
        { Site { function: 7, file: 4, line: 18, column: 13, end_line: 18, end_column: 36 } },
        /* terrane-site-row: site 32: /native-borrowed-iterator::first-text-owned (tests/integration/native-borrowed-iterator/case.trn:18:12-18:56) */
        { Site { function: 7, file: 4, line: 18, column: 12, end_line: 18, end_column: 56 } },
        /* terrane-site-row: site 33: /native-borrowed-iterator::collected-text (tests/integration/native-borrowed-iterator/case.trn:21:14-21:29) */
        { Site { function: 8, file: 4, line: 21, column: 14, end_line: 21, end_column: 29 } },
        /* terrane-site-row: site 34: /native-borrowed-iterator::collected-text (tests/integration/native-borrowed-iterator/case.trn:21:13-21:36) */
        { Site { function: 8, file: 4, line: 21, column: 13, end_line: 21, end_column: 36 } },
        /* terrane-site-row: site 35: /native-borrowed-iterator::collected-text (tests/integration/native-borrowed-iterator/case.trn:21:12-21:72) */
        { Site { function: 8, file: 4, line: 21, column: 12, end_line: 21, end_column: 72 } },
        /* terrane-site-row: site 36: /native-borrowed-iterator::mapped-text (tests/integration/native-borrowed-iterator/case.trn:24:15-24:33) */
        { Site { function: 9, file: 4, line: 24, column: 15, end_line: 24, end_column: 33 } },
        /* terrane-site-row: site 37: /native-borrowed-iterator::mapped-text (tests/integration/native-borrowed-iterator/case.trn:25:14-25:29) */
        { Site { function: 9, file: 4, line: 25, column: 14, end_line: 25, end_column: 29 } },
        /* terrane-site-row: site 38: /native-borrowed-iterator::mapped-text (tests/integration/native-borrowed-iterator/case.trn:25:13-25:36) */
        { Site { function: 9, file: 4, line: 25, column: 13, end_line: 25, end_column: 36 } },
        /* terrane-site-row: site 39: /native-borrowed-iterator::mapped-text (tests/integration/native-borrowed-iterator/case.trn:25:12-25:61) */
        { Site { function: 9, file: 4, line: 25, column: 12, end_line: 25, end_column: 61 } },
        /* terrane-site-row: site 40: /native-borrowed-iterator::entry-text (tests/integration/native-borrowed-iterator/case.trn:28:12-28:27) */
        { Site { function: 10, file: 4, line: 28, column: 12, end_line: 28, end_column: 27 } },
        /* terrane-site-row: site 41: /native-borrowed-iterator::nonempty-entry (tests/integration/native-borrowed-iterator/case.trn:31:12-31:27) */
        { Site { function: 11, file: 4, line: 31, column: 12, end_line: 31, end_column: 27 } },
        /* terrane-site-row: site 42: /native-borrowed-iterator::test-scenario (tests/integration/native-borrowed-iterator/case.trn:37:27-37:45) */
        { Site { function: 12, file: 4, line: 37, column: 27, end_line: 37, end_column: 45 } },
        /* terrane-site-row: site 43: /native-borrowed-iterator::test-scenario (tests/integration/native-borrowed-iterator/case.trn:41:27-41:46) */
        { Site { function: 12, file: 4, line: 41, column: 27, end_line: 41, end_column: 46 } },
        /* terrane-site-row: site 44: /native-borrowed-iterator::test-scenario (tests/integration/native-borrowed-iterator/case.trn:43:13-43:31) */
        { Site { function: 12, file: 4, line: 43, column: 13, end_line: 43, end_column: 31 } },
        /* terrane-site-row: site 45: /native-borrowed-iterator::test-scenario (tests/integration/native-borrowed-iterator/case.trn:50:12-50:25) */
        { Site { function: 12, file: 4, line: 50, column: 12, end_line: 50, end_column: 25 } },
        /* terrane-site-row: site 46: /native-borrowed-iterator::test-scenario (tests/integration/native-borrowed-iterator/case.trn:51:25-51:56) */
        { Site { function: 12, file: 4, line: 51, column: 25, end_line: 51, end_column: 56 } },
        /* terrane-site-row: site 47: /native-borrowed-iterator::test-scenario (tests/integration/native-borrowed-iterator/case.trn:56:13-56:32) */
        { Site { function: 12, file: 4, line: 56, column: 13, end_line: 56, end_column: 32 } },
        /* terrane-site-row: site 48: /native-borrowed-iterator::test-scenario (tests/integration/native-borrowed-iterator/case.trn:60:13-60:34) */
        { Site { function: 12, file: 4, line: 60, column: 13, end_line: 60, end_column: 34 } },
        /* terrane-site-row: site 49: /native-borrowed-slice::title (tests/integration/native-borrowed-slice/case.trn:5:12-5:24) */
        { Site { function: 13, file: 5, line: 5, column: 12, end_line: 5, end_column: 24 } },
        /* terrane-site-row: site 50: /native-borrowed-slice::titles (tests/integration/native-borrowed-slice/case.trn:8:12-8:49) */
        { Site { function: 14, file: 5, line: 8, column: 12, end_line: 8, end_column: 49 } },
        /* terrane-site-row: site 51: /native-borrowed-slice::test-scenario (tests/integration/native-borrowed-slice/case.trn:11:15-11:36) */
        { Site { function: 15, file: 5, line: 11, column: 15, end_line: 11, end_column: 36 } },
        /* terrane-site-row: site 52: /native-checked-ordinal::cell-size (tests/integration/native-checked-ordinal/case.trn:6:23-6:28) */
        { Site { function: 16, file: 6, line: 6, column: 23, end_line: 6, end_column: 28 } },
        /* terrane-site-row: site 53: /native-checked-ordinal::cell-size (tests/integration/native-checked-ordinal/case.trn:6:13-6:28) */
        { Site { function: 16, file: 6, line: 6, column: 13, end_line: 6, end_column: 28 } },
        /* terrane-site-row: site 54: /native-checked-ordinal::cell-size (tests/integration/native-checked-ordinal/case.trn:6:12-6:35) */
        { Site { function: 16, file: 6, line: 6, column: 12, end_line: 6, end_column: 35 } },
        /* terrane-site-row: site 55: /native-checked-ordinal::test-scenario (tests/integration/native-checked-ordinal/case.trn:9:11-9:30) */
        { Site { function: 17, file: 6, line: 9, column: 11, end_line: 9, end_column: 30 } },
        /* terrane-site-row: site 56: /native-checked-ordinal::test-scenario (tests/integration/native-checked-ordinal/case.trn:10:13-10:30) */
        { Site { function: 17, file: 6, line: 10, column: 13, end_line: 10, end_column: 30 } },
        /* terrane-site-row: site 57: /native-checked-ordinal::test-scenario (tests/integration/native-checked-ordinal/case.trn:12:17-12:35) */
        { Site { function: 17, file: 6, line: 12, column: 17, end_line: 12, end_column: 35 } },
        /* terrane-site-row: site 58: /native-checked-ordinal::test-scenario (tests/integration/native-checked-ordinal/case.trn:16:17-16:34) */
        { Site { function: 17, file: 6, line: 16, column: 17, end_line: 16, end_column: 34 } },
        /* terrane-site-row: site 59: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:6:14-6:26) */
        { Site { function: 18, file: 7, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 60: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:7:20-7:37) */
        { Site { function: 18, file: 7, line: 7, column: 20, end_line: 7, end_column: 37 } },
        /* terrane-site-row: site 61: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:9:13-9:21) */
        { Site { function: 18, file: 7, line: 9, column: 13, end_line: 9, end_column: 21 } },
        /* terrane-site-row: site 62: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:9:13-9:27) */
        { Site { function: 18, file: 7, line: 9, column: 13, end_line: 9, end_column: 27 } },
        /* terrane-site-row: site 63: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:10:28-10:45) */
        { Site { function: 18, file: 7, line: 10, column: 28, end_line: 10, end_column: 45 } },
        /* terrane-site-row: site 64: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:12:13-12:28) */
        { Site { function: 18, file: 7, line: 12, column: 13, end_line: 12, end_column: 28 } },
        /* terrane-site-row: site 65: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:12:13-12:31) */
        { Site { function: 18, file: 7, line: 12, column: 13, end_line: 12, end_column: 31 } },
        /* terrane-site-row: site 66: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:12:13-12:37) */
        { Site { function: 18, file: 7, line: 12, column: 13, end_line: 12, end_column: 37 } },
        /* terrane-site-row: site 67: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:13:5-13:17) */
        { Site { function: 18, file: 7, line: 13, column: 5, end_line: 13, end_column: 17 } },
        /* terrane-site-row: site 68: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:14:13-14:30) */
        { Site { function: 18, file: 7, line: 14, column: 13, end_line: 14, end_column: 30 } },
        /* terrane-site-row: site 69: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:15:12-15:29) */
        { Site { function: 18, file: 7, line: 15, column: 12, end_line: 15, end_column: 29 } },
        /* terrane-site-row: site 70: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:16:5-16:15) */
        { Site { function: 18, file: 7, line: 16, column: 5, end_line: 16, end_column: 15 } },
        /* terrane-site-row: site 71: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:18:13-18:23) */
        { Site { function: 18, file: 7, line: 18, column: 13, end_line: 18, end_column: 23 } },
        /* terrane-site-row: site 72: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:19:13-19:22) */
        { Site { function: 18, file: 7, line: 19, column: 13, end_line: 19, end_column: 22 } },
        /* terrane-site-row: site 73: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:19:13-19:28) */
        { Site { function: 18, file: 7, line: 19, column: 13, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 74: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:23:17-23:27) */
        { Site { function: 18, file: 7, line: 23, column: 17, end_line: 23, end_column: 27 } },
        /* terrane-site-row: site 75: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:24:5-24:17) */
        { Site { function: 18, file: 7, line: 24, column: 5, end_line: 24, end_column: 17 } },
        /* terrane-site-row: site 76: /native-copy-view-list-lender::test-scenario (tests/integration/native-copy-view-list-lender/case.trn:25:13-25:30) */
        { Site { function: 18, file: 7, line: 25, column: 13, end_line: 25, end_column: 30 } },
        /* terrane-site-row: site 77: /native-copy-view-reassignment-snapshot::fresh (tests/integration/native-copy-view-reassignment-snapshot/case.trn:7:12-7:29) */
        { Site { function: 19, file: 8, line: 7, column: 12, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 78: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:10:9-10:21) */
        { Site { function: 20, file: 8, line: 10, column: 9, end_line: 10, end_column: 21 } },
        /* terrane-site-row: site 79: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:11:9-11:21) */
        { Site { function: 20, file: 8, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 80: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:12:9-12:21) */
        { Site { function: 20, file: 8, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 81: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:14:9-14:21) */
        { Site { function: 20, file: 8, line: 14, column: 9, end_line: 14, end_column: 21 } },
        /* terrane-site-row: site 82: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:15:5-15:12) */
        { Site { function: 20, file: 8, line: 15, column: 5, end_line: 15, end_column: 12 } },
        /* terrane-site-row: site 83: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:22) */
        { Site { function: 20, file: 8, line: 16, column: 13, end_line: 16, end_column: 22 } },
        /* terrane-site-row: site 84: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:28) */
        { Site { function: 20, file: 8, line: 16, column: 13, end_line: 16, end_column: 28 } },
        /* terrane-site-row: site 85: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:18:13-18:20) */
        { Site { function: 20, file: 8, line: 18, column: 13, end_line: 18, end_column: 20 } },
        /* terrane-site-row: site 86: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:19:5-19:12) */
        { Site { function: 20, file: 8, line: 19, column: 5, end_line: 19, end_column: 12 } },
        /* terrane-site-row: site 87: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:22) */
        { Site { function: 20, file: 8, line: 20, column: 13, end_line: 20, end_column: 22 } },
        /* terrane-site-row: site 88: /native-copy-view-reassignment-snapshot::test-scenario (tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:28) */
        { Site { function: 20, file: 8, line: 20, column: 13, end_line: 20, end_column: 28 } },
        /* terrane-site-row: site 89: /native-field-copy-clone-conversion::construct (tests/integration/native-field-copy-clone-conversion/case.trn:7:18-7:29) */
        { Site { function: 21, file: 9, line: 7, column: 18, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 90: /native-field-copy-clone-conversion::construct (tests/integration/native-field-copy-clone-conversion/case.trn:8:18-8:31) */
        { Site { function: 21, file: 9, line: 8, column: 18, end_line: 8, end_column: 31 } },
        /* terrane-site-row: site 91: /native-field-copy-clone-conversion::observe (tests/integration/native-field-copy-clone-conversion/case.trn:10:12-10:22) */
        { Site { function: 22, file: 9, line: 10, column: 12, end_line: 10, end_column: 22 } },
        /* terrane-site-row: site 92: /native-field-copy-clone-conversion::clone-observe (tests/integration/native-field-copy-clone-conversion/case.trn:12:12-12:19) */
        { Site { function: 23, file: 9, line: 12, column: 12, end_line: 12, end_column: 19 } },
        /* terrane-site-row: site 93: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:17:17-17:27) */
        { Site { function: 24, file: 9, line: 17, column: 17, end_line: 17, end_column: 27 } },
        /* terrane-site-row: site 94: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:18:17-18:29) */
        { Site { function: 24, file: 9, line: 18, column: 17, end_line: 18, end_column: 29 } },
        /* terrane-site-row: site 95: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:19:17-19:26) */
        { Site { function: 24, file: 9, line: 19, column: 17, end_line: 19, end_column: 26 } },
        /* terrane-site-row: site 96: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:21:13-21:23) */
        { Site { function: 24, file: 9, line: 21, column: 13, end_line: 21, end_column: 23 } },
        /* terrane-site-row: site 97: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:22:9-22:22) */
        { Site { function: 24, file: 9, line: 22, column: 9, end_line: 22, end_column: 22 } },
        /* terrane-site-row: site 98: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:23:13-23:29) */
        { Site { function: 24, file: 9, line: 23, column: 13, end_line: 23, end_column: 29 } },
        /* terrane-site-row: site 99: /native-field-copy-clone-conversion::test-scenario (tests/integration/native-field-copy-clone-conversion/case.trn:24:13-24:29) */
        { Site { function: 24, file: 9, line: 24, column: 13, end_line: 24, end_column: 29 } },
        /* terrane-site-row: site 100: /native-field-text::original-path (tests/integration/native-field-text/case.trn:7:9-7:29) */
        { Site { function: 25, file: 10, line: 7, column: 9, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 101: /native-field-text::original-path (tests/integration/native-field-text/case.trn:8:16-8:35) */
        { Site { function: 25, file: 10, line: 8, column: 16, end_line: 8, end_column: 35 } },
        /* terrane-site-row: site 102: /native-field-text::test-scenario (tests/integration/native-field-text/case.trn:12:16-12:52) */
        { Site { function: 26, file: 10, line: 12, column: 16, end_line: 12, end_column: 52 } },
        /* terrane-site-row: site 103: /native-field-text::test-scenario (tests/integration/native-field-text/case.trn:13:14-13:36) */
        { Site { function: 26, file: 10, line: 13, column: 14, end_line: 13, end_column: 36 } },
        /* terrane-site-row: site 104: /native-field-text::test-scenario (tests/integration/native-field-text/case.trn:15:13-15:36) */
        { Site { function: 26, file: 10, line: 15, column: 13, end_line: 15, end_column: 36 } },
        /* terrane-site-row: site 105: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:4:9-4:21) */
        { Site { function: 27, file: 11, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 106: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:5:9-5:16) */
        { Site { function: 27, file: 11, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 107: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:7:17-7:29) */
        { Site { function: 27, file: 11, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 108: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:9:13-9:20) */
        { Site { function: 27, file: 11, line: 9, column: 13, end_line: 9, end_column: 20 } },
        /* terrane-site-row: site 109: /native-finally-only-rewrite::test-scenario (tests/integration/native-finally-only-rewrite/case.trn:10:17-10:24) */
        { Site { function: 27, file: 11, line: 10, column: 17, end_line: 10, end_column: 24 } },
        /* terrane-site-row: site 110: /native-from-conversion::borrowed-text (tests/integration/native-from-conversion/case.trn:5:12-5:32) */
        { Site { function: 28, file: 12, line: 5, column: 12, end_line: 5, end_column: 32 } },
        /* terrane-site-row: site 111: /native-from-conversion::borrowed-native (tests/integration/native-from-conversion/case.trn:8:12-8:33) */
        { Site { function: 29, file: 12, line: 8, column: 12, end_line: 8, end_column: 33 } },
        /* terrane-site-row: site 112: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:13:13-13:27) */
        { Site { function: 30, file: 12, line: 13, column: 13, end_line: 13, end_column: 27 } },
        /* terrane-site-row: site 113: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:15:19-15:47) */
        { Site { function: 30, file: 12, line: 15, column: 19, end_line: 15, end_column: 47 } },
        /* terrane-site-row: site 114: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:16:13-16:40) */
        { Site { function: 30, file: 12, line: 16, column: 13, end_line: 16, end_column: 40 } },
        /* terrane-site-row: site 115: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:17:13-17:33) */
        { Site { function: 30, file: 12, line: 17, column: 13, end_line: 17, end_column: 33 } },
        /* terrane-site-row: site 116: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:18:13-18:24) */
        { Site { function: 30, file: 12, line: 18, column: 13, end_line: 18, end_column: 24 } },
        /* terrane-site-row: site 117: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:20:14-20:38) */
        { Site { function: 30, file: 12, line: 20, column: 14, end_line: 20, end_column: 38 } },
        /* terrane-site-row: site 118: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:21:13-21:25) */
        { Site { function: 30, file: 12, line: 21, column: 13, end_line: 21, end_column: 25 } },
        /* terrane-site-row: site 119: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:22:14-22:29) */
        { Site { function: 30, file: 12, line: 22, column: 14, end_line: 22, end_column: 29 } },
        /* terrane-site-row: site 120: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:24:13-24:33) */
        { Site { function: 30, file: 12, line: 24, column: 13, end_line: 24, end_column: 33 } },
        /* terrane-site-row: site 121: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:25:19-25:46) */
        { Site { function: 30, file: 12, line: 25, column: 19, end_line: 25, end_column: 46 } },
        /* terrane-site-row: site 122: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:26:13-26:30) */
        { Site { function: 30, file: 12, line: 26, column: 13, end_line: 26, end_column: 30 } },
        /* terrane-site-row: site 123: /native-from-conversion::test-scenario (tests/integration/native-from-conversion/case.trn:27:16-27:38) */
        { Site { function: 30, file: 12, line: 27, column: 16, end_line: 27, end_column: 38 } },
        /* terrane-site-row: site 124: /native-inferred-copy-view::mutable_size (tests/integration/native-inferred-copy-view/case.trn:5:12-5:23) */
        { Site { function: 31, file: 13, line: 5, column: 12, end_line: 5, end_column: 23 } },
        /* terrane-site-row: site 125: /native-inferred-copy-view::readonly_size (tests/integration/native-inferred-copy-view/case.trn:8:12-8:23) */
        { Site { function: 32, file: 13, line: 8, column: 12, end_line: 8, end_column: 23 } },
        /* terrane-site-row: site 126: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:11:14-11:26) */
        { Site { function: 33, file: 13, line: 11, column: 14, end_line: 11, end_column: 26 } },
        /* terrane-site-row: site 127: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:12:21-12:38) */
        { Site { function: 33, file: 13, line: 12, column: 21, end_line: 12, end_column: 38 } },
        /* terrane-site-row: site 128: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:15:23-15:40) */
        { Site { function: 33, file: 13, line: 15, column: 23, end_line: 15, end_column: 40 } },
        /* terrane-site-row: site 129: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:16:13-16:24) */
        { Site { function: 33, file: 13, line: 16, column: 13, end_line: 16, end_column: 24 } },
        /* terrane-site-row: site 130: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:16:13-16:30) */
        { Site { function: 33, file: 13, line: 16, column: 13, end_line: 16, end_column: 30 } },
        /* terrane-site-row: site 131: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:17:13-17:24) */
        { Site { function: 33, file: 13, line: 17, column: 13, end_line: 17, end_column: 24 } },
        /* terrane-site-row: site 132: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:17:13-17:30) */
        { Site { function: 33, file: 13, line: 17, column: 13, end_line: 17, end_column: 30 } },
        /* terrane-site-row: site 133: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:18:37-18:54) */
        { Site { function: 33, file: 13, line: 18, column: 37, end_line: 18, end_column: 54 } },
        /* terrane-site-row: site 134: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:19:13-19:21) */
        { Site { function: 33, file: 13, line: 19, column: 13, end_line: 19, end_column: 21 } },
        /* terrane-site-row: site 135: /native-inferred-copy-view::test-scenario (tests/integration/native-inferred-copy-view/case.trn:19:13-19:27) */
        { Site { function: 33, file: 13, line: 19, column: 13, end_line: 19, end_column: 27 } },
        /* terrane-site-row: site 136: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:6:9-6:21) */
        { Site { function: 34, file: 14, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 137: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:7:9-7:16) */
        { Site { function: 34, file: 14, line: 7, column: 9, end_line: 7, end_column: 16 } },
        /* terrane-site-row: site 138: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:8:21-8:28) */
        { Site { function: 34, file: 14, line: 8, column: 21, end_line: 8, end_column: 28 } },
        /* terrane-site-row: site 139: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:10:13-10:22) */
        { Site { function: 34, file: 14, line: 10, column: 13, end_line: 10, end_column: 22 } },
        /* terrane-site-row: site 140: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:10:13-10:28) */
        { Site { function: 34, file: 14, line: 10, column: 13, end_line: 10, end_column: 28 } },
        /* terrane-site-row: site 141: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:11:13-11:20) */
        { Site { function: 34, file: 14, line: 11, column: 13, end_line: 11, end_column: 20 } },
        /* terrane-site-row: site 142: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:14:13-14:24) */
        { Site { function: 34, file: 14, line: 14, column: 13, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 143: /native-loan-append-transfer::test-scenario (tests/integration/native-loan-append-transfer/case.trn:14:13-14:30) */
        { Site { function: 34, file: 14, line: 14, column: 13, end_line: 14, end_column: 30 } },
        /* terrane-site-row: site 144: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:11:9-11:21) */
        { Site { function: 35, file: 15, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 145: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:12:9-12:21) */
        { Site { function: 35, file: 15, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 146: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:13:9-13:16) */
        { Site { function: 35, file: 15, line: 13, column: 9, end_line: 13, end_column: 16 } },
        /* terrane-site-row: site 147: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:14:9-14:24) */
        { Site { function: 35, file: 15, line: 14, column: 9, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 148: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:15:9-15:16) */
        { Site { function: 35, file: 15, line: 15, column: 9, end_line: 15, end_column: 16 } },
        /* terrane-site-row: site 149: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:16:13-16:20) */
        { Site { function: 35, file: 15, line: 16, column: 13, end_line: 16, end_column: 20 } },
        /* terrane-site-row: site 150: /native-loan-chain-rebind-receiver::test-scenario (tests/integration/native-loan-chain-rebind-receiver/case.trn:16:24-16:31) */
        { Site { function: 35, file: 15, line: 16, column: 24, end_line: 16, end_column: 31 } },
        /* terrane-site-row: site 151: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:11:22-11:34) */
        { Site { function: 36, file: 16, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 152: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:11:38-11:50) */
        { Site { function: 36, file: 16, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 153: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:12:9-12:22) */
        { Site { function: 36, file: 16, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 154: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:15:19-15:31) */
        { Site { function: 36, file: 16, line: 15, column: 19, end_line: 15, end_column: 31 } },
        /* terrane-site-row: site 155: /native-loan-exclusive-branch::test-scenario (tests/integration/native-loan-exclusive-branch/case.trn:17:17-17:24) */
        { Site { function: 36, file: 16, line: 17, column: 17, end_line: 17, end_column: 24 } },
        /* terrane-site-row: site 156: /native-loan-finally-copy-rebind::test-scenario (tests/integration/native-loan-finally-copy-rebind/case.trn:11:9-11:21) */
        { Site { function: 37, file: 17, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 157: /native-loan-finally-copy-rebind::test-scenario (tests/integration/native-loan-finally-copy-rebind/case.trn:12:9-12:21) */
        { Site { function: 37, file: 17, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 158: /native-loan-finally-copy-rebind::test-scenario (tests/integration/native-loan-finally-copy-rebind/case.trn:18:21-18:28) */
        { Site { function: 37, file: 17, line: 18, column: 21, end_line: 18, end_column: 28 } },
        /* terrane-site-row: site 159: /native-loan-finally-copy-rebind::test-scenario (tests/integration/native-loan-finally-copy-rebind/case.trn:19:17-19:29) */
        { Site { function: 37, file: 17, line: 19, column: 17, end_line: 19, end_column: 29 } },
        /* terrane-site-row: site 160: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:6:9-6:21) */
        { Site { function: 38, file: 18, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 161: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:7:17-7:29) */
        { Site { function: 38, file: 18, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 162: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:8:9-8:16) */
        { Site { function: 38, file: 18, line: 8, column: 9, end_line: 8, end_column: 16 } },
        /* terrane-site-row: site 163: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:9:21-9:29) */
        { Site { function: 38, file: 18, line: 9, column: 21, end_line: 9, end_column: 29 } },
        /* terrane-site-row: site 164: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:10:9-10:16) */
        { Site { function: 38, file: 18, line: 10, column: 9, end_line: 10, end_column: 16 } },
        /* terrane-site-row: site 165: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:12:9-12:16) */
        { Site { function: 38, file: 18, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 166: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:13:5-13:20) */
        { Site { function: 38, file: 18, line: 13, column: 5, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 167: /native-loan-fluent-nested-rebind::test-scenario (tests/integration/native-loan-fluent-nested-rebind/case.trn:16:13-16:20) */
        { Site { function: 38, file: 18, line: 16, column: 13, end_line: 16, end_column: 20 } },
        /* terrane-site-row: site 168: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:15:9-15:21) */
        { Site { function: 39, file: 19, line: 15, column: 9, end_line: 15, end_column: 21 } },
        /* terrane-site-row: site 169: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:16:19-16:31) */
        { Site { function: 39, file: 19, line: 16, column: 19, end_line: 16, end_column: 31 } },
        /* terrane-site-row: site 170: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:17:13-17:23) */
        { Site { function: 39, file: 19, line: 17, column: 13, end_line: 17, end_column: 23 } },
        /* terrane-site-row: site 171: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:18:32-18:44) */
        { Site { function: 39, file: 19, line: 18, column: 32, end_line: 18, end_column: 44 } },
        /* terrane-site-row: site 172: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:22) */
        { Site { function: 39, file: 19, line: 19, column: 13, end_line: 19, end_column: 22 } },
        /* terrane-site-row: site 173: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:28) */
        { Site { function: 39, file: 19, line: 19, column: 13, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 174: /native-loan-helper-view-origin::test-scenario (tests/integration/native-loan-helper-view-origin/case.trn:20:5-20:12) */
        { Site { function: 39, file: 19, line: 20, column: 5, end_line: 20, end_column: 12 } },
        /* terrane-site-row: site 175: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:6:9-6:21) */
        { Site { function: 40, file: 20, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 176: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:7:38-7:50) */
        { Site { function: 40, file: 20, line: 7, column: 38, end_line: 7, end_column: 50 } },
        /* terrane-site-row: site 177: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:8:13-8:22) */
        { Site { function: 40, file: 20, line: 8, column: 13, end_line: 8, end_column: 22 } },
        /* terrane-site-row: site 178: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:8:13-8:28) */
        { Site { function: 40, file: 20, line: 8, column: 13, end_line: 8, end_column: 28 } },
        /* terrane-site-row: site 179: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:9:5-9:12) */
        { Site { function: 40, file: 20, line: 9, column: 5, end_line: 9, end_column: 12 } },
        /* terrane-site-row: site 180: /native-loan-last-use-control::test-scenario (tests/integration/native-loan-last-use-control/case.trn:10:13-10:25) */
        { Site { function: 40, file: 20, line: 10, column: 13, end_line: 10, end_column: 25 } },
        /* terrane-site-row: site 181: /native-loan-loop-live-lender::test-scenario (tests/integration/native-loan-loop-live-lender/case.trn:6:9-6:21) */
        { Site { function: 41, file: 21, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 182: /native-loan-loop-live-lender::test-scenario (tests/integration/native-loan-loop-live-lender/case.trn:7:38-7:50) */
        { Site { function: 41, file: 21, line: 7, column: 38, end_line: 7, end_column: 50 } },
        /* terrane-site-row: site 183: /native-loan-loop-live-lender::test-scenario (tests/integration/native-loan-loop-live-lender/case.trn:9:17-9:28) */
        { Site { function: 41, file: 21, line: 9, column: 17, end_line: 9, end_column: 28 } },
        /* terrane-site-row: site 184: /native-loan-loop-live-lender::test-scenario (tests/integration/native-loan-loop-live-lender/case.trn:10:5-10:12) */
        { Site { function: 41, file: 21, line: 10, column: 5, end_line: 10, end_column: 12 } },
        /* terrane-site-row: site 185: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:11:22-11:34) */
        { Site { function: 42, file: 22, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 186: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:11:38-11:50) */
        { Site { function: 42, file: 22, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 187: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:14:13-14:26) */
        { Site { function: 42, file: 22, line: 14, column: 13, end_line: 14, end_column: 26 } },
        /* terrane-site-row: site 188: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:15:17-15:24) */
        { Site { function: 42, file: 22, line: 15, column: 17, end_line: 15, end_column: 24 } },
        /* terrane-site-row: site 189: /native-loan-loop-refreshed::test-scenario (tests/integration/native-loan-loop-refreshed/case.trn:16:19-16:31) */
        { Site { function: 42, file: 22, line: 16, column: 19, end_line: 16, end_column: 31 } },
        /* terrane-site-row: site 190: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:6:9-6:21) */
        { Site { function: 43, file: 23, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 191: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:8:26-8:38) */
        { Site { function: 43, file: 23, line: 8, column: 26, end_line: 8, end_column: 38 } },
        /* terrane-site-row: site 192: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:9:13-9:27) */
        { Site { function: 43, file: 23, line: 9, column: 13, end_line: 9, end_column: 27 } },
        /* terrane-site-row: site 193: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:9:13-9:33) */
        { Site { function: 43, file: 23, line: 9, column: 13, end_line: 9, end_column: 33 } },
        /* terrane-site-row: site 194: /native-loan-map-set-lender::test-scenario (tests/integration/native-loan-map-set-lender/case.trn:10:5-10:12) */
        { Site { function: 43, file: 23, line: 10, column: 5, end_line: 10, end_column: 12 } },
        /* terrane-site-row: site 195: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:11:9-11:21) */
        { Site { function: 44, file: 24, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 196: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:12:9-12:16) */
        { Site { function: 44, file: 24, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 197: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:13:16-13:23) */
        { Site { function: 44, file: 24, line: 13, column: 16, end_line: 13, end_column: 23 } },
        /* terrane-site-row: site 198: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:14:9-14:21) */
        { Site { function: 44, file: 24, line: 14, column: 9, end_line: 14, end_column: 21 } },
        /* terrane-site-row: site 199: /native-loan-owned-observation::test-scenario (tests/integration/native-loan-owned-observation/case.trn:16:13-16:25) */
        { Site { function: 44, file: 24, line: 16, column: 13, end_line: 16, end_column: 25 } },
        /* terrane-site-row: site 200: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:11:22-11:34) */
        { Site { function: 45, file: 25, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 201: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:11:38-11:50) */
        { Site { function: 45, file: 25, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 202: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:12:9-12:22) */
        { Site { function: 45, file: 25, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 203: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:14:15-14:22) */
        { Site { function: 45, file: 25, line: 14, column: 15, end_line: 14, end_column: 22 } },
        /* terrane-site-row: site 204: /native-loan-predicate-last-use::test-scenario (tests/integration/native-loan-predicate-last-use/case.trn:15:23-15:35) */
        { Site { function: 45, file: 25, line: 15, column: 23, end_line: 15, end_column: 35 } },
        /* terrane-site-row: site 205: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:11:22-11:34) */
        { Site { function: 46, file: 26, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 206: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:11:38-11:50) */
        { Site { function: 46, file: 26, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 207: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:12:9-12:22) */
        { Site { function: 46, file: 26, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 208: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:16:23-16:35) */
        { Site { function: 46, file: 26, line: 16, column: 23, end_line: 16, end_column: 35 } },
        /* terrane-site-row: site 209: /native-loan-predicate-switch::test-scenario (tests/integration/native-loan-predicate-switch/case.trn:18:21-18:28) */
        { Site { function: 46, file: 26, line: 18, column: 21, end_line: 18, end_column: 28 } },
        /* terrane-site-row: site 210: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:11:22-11:34) */
        { Site { function: 47, file: 27, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 211: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:11:38-11:50) */
        { Site { function: 47, file: 27, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 212: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:12:9-12:22) */
        { Site { function: 47, file: 27, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 213: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:16:23-16:35) */
        { Site { function: 47, file: 27, line: 16, column: 23, end_line: 16, end_column: 35 } },
        /* terrane-site-row: site 214: /native-loan-predicate-three-arms::test-scenario (tests/integration/native-loan-predicate-three-arms/case.trn:20:21-20:28) */
        { Site { function: 47, file: 27, line: 20, column: 21, end_line: 20, end_column: 28 } },
        /* terrane-site-row: site 215: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:11:9-11:21) */
        { Site { function: 48, file: 28, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 216: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:12:9-12:21) */
        { Site { function: 48, file: 28, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 217: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:13:9-13:16) */
        { Site { function: 48, file: 28, line: 13, column: 9, end_line: 13, end_column: 16 } },
        /* terrane-site-row: site 218: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:14:9-14:16) */
        { Site { function: 48, file: 28, line: 14, column: 9, end_line: 14, end_column: 16 } },
        /* terrane-site-row: site 219: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:15:9-15:24) */
        { Site { function: 48, file: 28, line: 15, column: 9, end_line: 15, end_column: 24 } },
        /* terrane-site-row: site 220: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:16:5-16:12) */
        { Site { function: 48, file: 28, line: 16, column: 5, end_line: 16, end_column: 12 } },
        /* terrane-site-row: site 221: /native-loan-rebound-alias-chain::test-scenario (tests/integration/native-loan-rebound-alias-chain/case.trn:17:13-17:20) */
        { Site { function: 48, file: 28, line: 17, column: 13, end_line: 17, end_column: 20 } },
        /* terrane-site-row: site 222: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:11:22-11:34) */
        { Site { function: 49, file: 29, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 223: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:11:38-11:50) */
        { Site { function: 49, file: 29, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 224: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:12:9-12:22) */
        { Site { function: 49, file: 29, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 225: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:13:13-13:20) */
        { Site { function: 49, file: 29, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 226: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:14:15-14:27) */
        { Site { function: 49, file: 29, line: 14, column: 15, end_line: 14, end_column: 27 } },
        /* terrane-site-row: site 227: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:15:9-15:22) */
        { Site { function: 49, file: 29, line: 15, column: 9, end_line: 15, end_column: 22 } },
        /* terrane-site-row: site 228: /native-loan-rebound-other::test-scenario (tests/integration/native-loan-rebound-other/case.trn:16:13-16:20) */
        { Site { function: 49, file: 29, line: 16, column: 13, end_line: 16, end_column: 20 } },
        /* terrane-site-row: site 229: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:11:22-11:34) */
        { Site { function: 50, file: 30, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 230: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:11:38-11:50) */
        { Site { function: 50, file: 30, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 231: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:12:9-12:22) */
        { Site { function: 50, file: 30, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 232: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:13:13-13:20) */
        { Site { function: 50, file: 30, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 233: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:14:15-14:27) */
        { Site { function: 50, file: 30, line: 14, column: 15, end_line: 14, end_column: 27 } },
        /* terrane-site-row: site 234: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:15:9-15:22) */
        { Site { function: 50, file: 30, line: 15, column: 9, end_line: 15, end_column: 22 } },
        /* terrane-site-row: site 235: /native-loan-rebound-same::test-scenario (tests/integration/native-loan-rebound-same/case.trn:16:13-16:20) */
        { Site { function: 50, file: 30, line: 16, column: 13, end_line: 16, end_column: 20 } },
        /* terrane-site-row: site 236: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:11:22-11:34) */
        { Site { function: 51, file: 31, line: 11, column: 22, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 237: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:11:38-11:50) */
        { Site { function: 51, file: 31, line: 11, column: 38, end_line: 11, end_column: 50 } },
        /* terrane-site-row: site 238: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:12:9-12:22) */
        { Site { function: 51, file: 31, line: 12, column: 9, end_line: 12, end_column: 22 } },
        /* terrane-site-row: site 239: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:15:19-15:31) */
        { Site { function: 51, file: 31, line: 15, column: 19, end_line: 15, end_column: 31 } },
        /* terrane-site-row: site 240: /native-loan-return-branch::test-scenario (tests/integration/native-loan-return-branch/case.trn:17:13-17:20) */
        { Site { function: 51, file: 31, line: 17, column: 13, end_line: 17, end_column: 20 } },
        /* terrane-site-row: site 241: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:4:9-4:21) */
        { Site { function: 52, file: 32, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 242: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:5:9-5:21) */
        { Site { function: 52, file: 32, line: 5, column: 9, end_line: 5, end_column: 21 } },
        /* terrane-site-row: site 243: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:6:9-6:16) */
        { Site { function: 52, file: 32, line: 6, column: 9, end_line: 6, end_column: 16 } },
        /* terrane-site-row: site 244: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:8:9-8:16) */
        { Site { function: 52, file: 32, line: 8, column: 9, end_line: 8, end_column: 16 } },
        /* terrane-site-row: site 245: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:9:13-9:20) */
        { Site { function: 52, file: 32, line: 9, column: 13, end_line: 9, end_column: 20 } },
        /* terrane-site-row: site 246: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:10:13-10:20) */
        { Site { function: 52, file: 32, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 247: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:11:5-11:12) */
        { Site { function: 52, file: 32, line: 11, column: 5, end_line: 11, end_column: 12 } },
        /* terrane-site-row: site 248: /native-loan-storage-alias-last-use::test-scenario (tests/integration/native-loan-storage-alias-last-use/case.trn:12:13-12:25) */
        { Site { function: 52, file: 32, line: 12, column: 13, end_line: 12, end_column: 25 } },
        /* terrane-site-row: site 249: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:11:9-11:21) */
        { Site { function: 53, file: 33, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 250: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:12:9-12:16) */
        { Site { function: 53, file: 33, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 251: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:14:17-14:24) */
        { Site { function: 53, file: 33, line: 14, column: 17, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 252: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:16:9-16:16) */
        { Site { function: 53, file: 33, line: 16, column: 9, end_line: 16, end_column: 16 } },
        /* terrane-site-row: site 253: /native-loan-try-finally-last-use::test-scenario (tests/integration/native-loan-try-finally-last-use/case.trn:17:13-17:25) */
        { Site { function: 53, file: 33, line: 17, column: 13, end_line: 17, end_column: 25 } },
        /* terrane-site-row: site 254: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:11:9-11:21) */
        { Site { function: 54, file: 34, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 255: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:13:13-13:20) */
        { Site { function: 54, file: 34, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 256: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:14:17-14:24) */
        { Site { function: 54, file: 34, line: 14, column: 17, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 257: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:16:17-16:29) */
        { Site { function: 54, file: 34, line: 16, column: 17, end_line: 16, end_column: 29 } },
        /* terrane-site-row: site 258: /native-loan-try-local::test-scenario (tests/integration/native-loan-try-local/case.trn:17:13-17:25) */
        { Site { function: 54, file: 34, line: 17, column: 13, end_line: 17, end_column: 25 } },
        /* terrane-site-row: site 259: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:11:9-11:21) */
        { Site { function: 55, file: 35, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 260: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:12:9-12:16) */
        { Site { function: 55, file: 35, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 261: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:13:13-13:20) */
        { Site { function: 55, file: 35, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 262: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:14:9-14:21) */
        { Site { function: 55, file: 35, line: 14, column: 9, end_line: 14, end_column: 21 } },
        /* terrane-site-row: site 263: /native-loan-whole-last-use::test-scenario (tests/integration/native-loan-whole-last-use/case.trn:15:13-15:25) */
        { Site { function: 55, file: 35, line: 15, column: 13, end_line: 15, end_column: 25 } },
        /* terrane-site-row: site 264: /native-loan-whole-move-last-use::test-scenario (tests/integration/native-loan-whole-move-last-use/case.trn:11:9-11:21) */
        { Site { function: 56, file: 36, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 265: /native-loan-whole-move-last-use::test-scenario (tests/integration/native-loan-whole-move-last-use/case.trn:12:9-12:16) */
        { Site { function: 56, file: 36, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 266: /native-loan-whole-move-last-use::test-scenario (tests/integration/native-loan-whole-move-last-use/case.trn:13:13-13:20) */
        { Site { function: 56, file: 36, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 267: /native-loan-whole-move-last-use::test-scenario (tests/integration/native-loan-whole-move-last-use/case.trn:15:13-15:29) */
        { Site { function: 56, file: 36, line: 15, column: 13, end_line: 15, end_column: 29 } },
        /* terrane-site-row: site 268: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:11:9-11:21) */
        { Site { function: 57, file: 37, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 269: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:12:9-12:16) */
        { Site { function: 57, file: 37, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 270: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:13:13-13:20) */
        { Site { function: 57, file: 37, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 271: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:14:5-14:12) */
        { Site { function: 57, file: 37, line: 14, column: 5, end_line: 14, end_column: 12 } },
        /* terrane-site-row: site 272: /native-loan-whole-mutate-last-use::test-scenario (tests/integration/native-loan-whole-mutate-last-use/case.trn:15:13-15:25) */
        { Site { function: 57, file: 37, line: 15, column: 13, end_line: 15, end_column: 25 } },
        /* terrane-site-row: site 273: /native-loop-consuming-condition-break::test-scenario (tests/integration/native-loop-consuming-condition-break/case.trn:11:9-11:21) */
        { Site { function: 58, file: 38, line: 11, column: 9, end_line: 11, end_column: 21 } },
        /* terrane-site-row: site 274: /native-loop-consuming-condition-break::test-scenario (tests/integration/native-loop-consuming-condition-break/case.trn:12:9-12:16) */
        { Site { function: 58, file: 38, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 275: /native-loop-consuming-condition-break::test-scenario (tests/integration/native-loop-consuming-condition-break/case.trn:13:12-13:24) */
        { Site { function: 58, file: 38, line: 13, column: 12, end_line: 13, end_column: 24 } },
        /* terrane-site-row: site 276: /native-noncopy-view-loop-append::test-scenario (tests/integration/native-noncopy-view-loop-append/case.trn:6:14-6:26) */
        { Site { function: 59, file: 39, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 277: /native-noncopy-view-loop-append::test-scenario (tests/integration/native-noncopy-view-loop-append/case.trn:7:12-7:24) */
        { Site { function: 59, file: 39, line: 7, column: 12, end_line: 7, end_column: 24 } },
        /* terrane-site-row: site 278: /native-noncopy-view-loop-append::test-scenario (tests/integration/native-noncopy-view-loop-append/case.trn:11:16-11:28) */
        { Site { function: 59, file: 39, line: 11, column: 16, end_line: 11, end_column: 28 } },
        /* terrane-site-row: site 279: /native-noncopy-view-loop-append::test-scenario (tests/integration/native-noncopy-view-loop-append/case.trn:14:17-14:28) */
        { Site { function: 59, file: 39, line: 14, column: 17, end_line: 14, end_column: 28 } },
        /* terrane-site-row: site 280: /native-optional-text::status-reason (tests/integration/native-optional-text/case.trn:7:26-7:48) */
        { Site { function: 60, file: 40, line: 7, column: 26, end_line: 7, end_column: 48 } },
        /* terrane-site-row: site 281: /native-optional-text::test-scenario (tests/integration/native-optional-text/case.trn:13:13-13:35) */
        { Site { function: 61, file: 40, line: 13, column: 13, end_line: 13, end_column: 35 } },
        /* terrane-site-row: site 282: /native-optional-text::test-scenario (tests/integration/native-optional-text/case.trn:14:15-14:37) */
        { Site { function: 61, file: 40, line: 14, column: 15, end_line: 14, end_column: 37 } },
        /* terrane-site-row: site 283: /native-optional-text::test-scenario (tests/integration/native-optional-text/case.trn:17:26-17:64) */
        { Site { function: 61, file: 40, line: 17, column: 26, end_line: 17, end_column: 64 } },
        /* terrane-site-row: site 284: /native-optional-text::test-scenario (tests/integration/native-optional-text/case.trn:18:25-18:36) */
        { Site { function: 61, file: 40, line: 18, column: 25, end_line: 18, end_column: 36 } },
        /* terrane-site-row: site 285: /native-optional-text::test-scenario (tests/integration/native-optional-text/case.trn:21:28-21:67) */
        { Site { function: 61, file: 40, line: 21, column: 28, end_line: 21, end_column: 67 } },
        /* terrane-site-row: site 286: /native-optional-text::test-scenario (tests/integration/native-optional-text/case.trn:24:26-24:55) */
        { Site { function: 61, file: 40, line: 24, column: 26, end_line: 24, end_column: 55 } },
        /* terrane-site-row: site 287: /native-optional-text::test-scenario (tests/integration/native-optional-text/case.trn:27:9-27:36) */
        { Site { function: 61, file: 40, line: 27, column: 9, end_line: 27, end_column: 36 } },
        /* terrane-site-row: site 288: /native-ref-clone-repeat::twice (tests/integration/native-ref-clone-repeat/case.trn:4:13-4:24) */
        { Site { function: 62, file: 41, line: 4, column: 13, end_line: 4, end_column: 24 } },
        /* terrane-site-row: site 289: /native-ref-clone-repeat::twice (tests/integration/native-ref-clone-repeat/case.trn:5:14-5:25) */
        { Site { function: 62, file: 41, line: 5, column: 14, end_line: 5, end_column: 25 } },
        /* terrane-site-row: site 290: /native-ref-clone-repeat::twice (tests/integration/native-ref-clone-repeat/case.trn:6:12-6:26) */
        { Site { function: 62, file: 41, line: 6, column: 12, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 291: /native-ref-clone-repeat::copied (tests/integration/native-ref-clone-repeat/case.trn:8:13-8:25) */
        { Site { function: 63, file: 41, line: 8, column: 13, end_line: 8, end_column: 25 } },
        /* terrane-site-row: site 292: /native-ref-clone-repeat::copied (tests/integration/native-ref-clone-repeat/case.trn:9:21-9:33) */
        { Site { function: 63, file: 41, line: 9, column: 21, end_line: 9, end_column: 33 } },
        /* terrane-site-row: site 293: /native-ref-clone-repeat::copied (tests/integration/native-ref-clone-repeat/case.trn:9:12-9:34) */
        { Site { function: 63, file: 41, line: 9, column: 12, end_line: 9, end_column: 34 } },
        /* terrane-site-row: site 294: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:11:13-11:26) */
        { Site { function: 64, file: 41, line: 11, column: 13, end_line: 11, end_column: 26 } },
        /* terrane-site-row: site 295: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:12:13-12:25) */
        { Site { function: 64, file: 41, line: 12, column: 13, end_line: 12, end_column: 25 } },
        /* terrane-site-row: site 296: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:13:13-13:24) */
        { Site { function: 64, file: 41, line: 13, column: 13, end_line: 13, end_column: 24 } },
        /* terrane-site-row: site 297: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:14:13-14:24) */
        { Site { function: 64, file: 41, line: 14, column: 13, end_line: 14, end_column: 24 } },
        /* terrane-site-row: site 298: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:15:13-15:26) */
        { Site { function: 64, file: 41, line: 15, column: 13, end_line: 15, end_column: 26 } },
        /* terrane-site-row: site 299: /native-ref-clone-repeat::test-scenario (tests/integration/native-ref-clone-repeat/case.trn:16:13-16:25) */
        { Site { function: 64, file: 41, line: 16, column: 13, end_line: 16, end_column: 25 } },
        /* terrane-site-row: site 300: /native-required-enum-field::construct (tests/integration/native-required-enum-field/case.trn:7:22-7:28) */
        { Site { function: 65, file: 42, line: 7, column: 22, end_line: 7, end_column: 28 } },
        /* terrane-site-row: site 301: /native-required-enum-field::finish (tests/integration/native-required-enum-field/case.trn:9:16-9:32) */
        { Site { function: 66, file: 42, line: 9, column: 16, end_line: 9, end_column: 32 } },
        /* terrane-site-row: site 302: /native-required-enum-field::test-scenario (tests/integration/native-required-enum-field/case.trn:16:13-16:30) */
        { Site { function: 67, file: 42, line: 16, column: 13, end_line: 16, end_column: 30 } },
        /* terrane-site-row: site 303: /native-required-enum-field::test-scenario (tests/integration/native-required-enum-field/case.trn:19:13-19:36) */
        { Site { function: 67, file: 42, line: 19, column: 13, end_line: 19, end_column: 36 } },
        /* terrane-site-row: site 304: /native-required-enum-field::test-scenario (tests/integration/native-required-enum-field/case.trn:22:13-22:26) */
        { Site { function: 67, file: 42, line: 22, column: 13, end_line: 22, end_column: 26 } },
        /* terrane-site-row: site 305: /native-return-parameter-single::f (tests/integration/native-return-parameter-single/case.trn:5:12-5:24) */
        { Site { function: 68, file: 43, line: 5, column: 12, end_line: 5, end_column: 24 } },
        /* terrane-site-row: site 306: /native-return-parameter-single::test-scenario (tests/integration/native-return-parameter-single/case.trn:8:9-8:21) */
        { Site { function: 69, file: 43, line: 8, column: 9, end_line: 8, end_column: 21 } },
        /* terrane-site-row: site 307: /native-return-parameter-single::test-scenario (tests/integration/native-return-parameter-single/case.trn:10:13-10:20) */
        { Site { function: 69, file: 43, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 308: /native-return-parameter-single::test-scenario (tests/integration/native-return-parameter-single/case.trn:11:5-11:12) */
        { Site { function: 69, file: 43, line: 11, column: 5, end_line: 11, end_column: 12 } },
        /* terrane-site-row: site 309: /native-scoped-text::parts-text (tests/integration/native-scoped-text/case.trn:6:12-6:23) */
        { Site { function: 70, file: 44, line: 6, column: 12, end_line: 6, end_column: 23 } },
        /* terrane-site-row: site 310: /native-scoped-text::target-text (tests/integration/native-scoped-text/case.trn:10:28-10:66) */
        { Site { function: 71, file: 44, line: 10, column: 28, end_line: 10, end_column: 66 } },
        /* terrane-site-row: site 311: /native-scoped-text::accepted-text (tests/integration/native-scoped-text/case.trn:17:16-17:35) */
        { Site { function: 72, file: 44, line: 17, column: 16, end_line: 17, end_column: 35 } },
        /* terrane-site-row: site 312: /native-scoped-text::optional-text (tests/integration/native-scoped-text/case.trn:24:12-24:58) */
        { Site { function: 73, file: 44, line: 24, column: 12, end_line: 24, end_column: 58 } },
        /* terrane-site-row: site 313: /native-scoped-text::test-scenario (tests/integration/native-scoped-text/case.trn:27:16-27:56) */
        { Site { function: 74, file: 44, line: 27, column: 16, end_line: 27, end_column: 56 } },
        /* terrane-site-row: site 314: /native-scoped-text::test-scenario (tests/integration/native-scoped-text/case.trn:28:15-28:38) */
        { Site { function: 74, file: 44, line: 28, column: 15, end_line: 28, end_column: 38 } },
        /* terrane-site-row: site 315: /native-scoped-text::test-scenario (tests/integration/native-scoped-text/case.trn:29:15-29:42) */
        { Site { function: 74, file: 44, line: 29, column: 15, end_line: 29, end_column: 42 } },
        /* terrane-site-row: site 316: /native-scoped-text::test-scenario (tests/integration/native-scoped-text/case.trn:37:13-37:36) */
        { Site { function: 74, file: 44, line: 37, column: 13, end_line: 37, end_column: 36 } },
        /* terrane-site-row: site 317: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:4:9-4:21) */
        { Site { function: 75, file: 45, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 318: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:5:9-5:16) */
        { Site { function: 75, file: 45, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 319: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:7:17-7:29) */
        { Site { function: 75, file: 45, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 320: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:8:13-8:20) */
        { Site { function: 75, file: 45, line: 8, column: 13, end_line: 8, end_column: 20 } },
        /* terrane-site-row: site 321: /native-try-body-rewrite-control::test-scenario (tests/integration/native-try-body-rewrite-control/case.trn:9:17-9:24) */
        { Site { function: 75, file: 45, line: 9, column: 17, end_line: 9, end_column: 24 } },
        /* terrane-site-row: site 322: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:4:9-4:21) */
        { Site { function: 76, file: 46, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 323: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:5:9-5:16) */
        { Site { function: 76, file: 46, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 324: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:7:17-7:29) */
        { Site { function: 76, file: 46, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 325: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:8:13-8:20) */
        { Site { function: 76, file: 46, line: 8, column: 13, end_line: 8, end_column: 20 } },
        /* terrane-site-row: site 326: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:10:13-10:20) */
        { Site { function: 76, file: 46, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 327: /native-try-catch-dominating-rewrite::test-scenario (tests/integration/native-try-catch-dominating-rewrite/case.trn:11:17-11:24) */
        { Site { function: 76, file: 46, line: 11, column: 17, end_line: 11, end_column: 24 } },
        /* terrane-site-row: site 328: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:4:9-4:21) */
        { Site { function: 77, file: 47, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 329: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:5:9-5:21) */
        { Site { function: 77, file: 47, line: 5, column: 9, end_line: 5, end_column: 21 } },
        /* terrane-site-row: site 330: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:6:9-6:21) */
        { Site { function: 77, file: 47, line: 6, column: 9, end_line: 6, end_column: 21 } },
        /* terrane-site-row: site 331: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:8:17-8:24) */
        { Site { function: 77, file: 47, line: 8, column: 17, end_line: 8, end_column: 24 } },
        /* terrane-site-row: site 332: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:9:13-9:25) */
        { Site { function: 77, file: 47, line: 9, column: 13, end_line: 9, end_column: 25 } },
        /* terrane-site-row: site 333: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:11:17-11:24) */
        { Site { function: 77, file: 47, line: 11, column: 17, end_line: 11, end_column: 24 } },
        /* terrane-site-row: site 334: /native-try-copy-finally-control::test-scenario (tests/integration/native-try-copy-finally-control/case.trn:12:13-12:20) */
        { Site { function: 77, file: 47, line: 12, column: 13, end_line: 12, end_column: 20 } },
        /* terrane-site-row: site 335: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:4:9-4:21) */
        { Site { function: 78, file: 48, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 336: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:5:9-5:16) */
        { Site { function: 78, file: 48, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 337: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:7:17-7:29) */
        { Site { function: 78, file: 48, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 338: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:8:13-8:20) */
        { Site { function: 78, file: 48, line: 8, column: 13, end_line: 8, end_column: 20 } },
        /* terrane-site-row: site 339: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:10:13-10:20) */
        { Site { function: 78, file: 48, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 340: /native-try-finally-dominating-rewrite::test-scenario (tests/integration/native-try-finally-dominating-rewrite/case.trn:11:17-11:24) */
        { Site { function: 78, file: 48, line: 11, column: 17, end_line: 11, end_column: 24 } },
        /* terrane-site-row: site 341: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:4:9-4:21) */
        { Site { function: 79, file: 49, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 342: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:5:9-5:16) */
        { Site { function: 79, file: 49, line: 5, column: 9, end_line: 5, end_column: 16 } },
        /* terrane-site-row: site 343: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:9:17-9:29) */
        { Site { function: 79, file: 49, line: 9, column: 17, end_line: 9, end_column: 29 } },
        /* terrane-site-row: site 344: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:10:13-10:20) */
        { Site { function: 79, file: 49, line: 10, column: 13, end_line: 10, end_column: 20 } },
        /* terrane-site-row: site 345: /native-try-finally-rewrite-control::test-scenario (tests/integration/native-try-finally-rewrite-control/case.trn:11:17-11:24) */
        { Site { function: 79, file: 49, line: 11, column: 17, end_line: 11, end_column: 24 } },
        /* terrane-site-row: site 346: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:4:9-4:21) */
        { Site { function: 80, file: 50, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 347: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:6:13-6:20) */
        { Site { function: 80, file: 50, line: 6, column: 13, end_line: 6, end_column: 20 } },
        /* terrane-site-row: site 348: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:7:17-7:29) */
        { Site { function: 80, file: 50, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 349: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:13:20-13:27) */
        { Site { function: 80, file: 50, line: 13, column: 20, end_line: 13, end_column: 27 } },
        /* terrane-site-row: site 350: /native-try-local-consume::test-scenario (tests/integration/native-try-local-consume/case.trn:14:21-14:36) */
        { Site { function: 80, file: 50, line: 14, column: 21, end_line: 14, end_column: 36 } },
        /* terrane-site-row: site 351: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:4:9-4:21) */
        { Site { function: 81, file: 51, line: 4, column: 9, end_line: 4, end_column: 21 } },
        /* terrane-site-row: site 352: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:5:9-5:21) */
        { Site { function: 81, file: 51, line: 5, column: 9, end_line: 5, end_column: 21 } },
        /* terrane-site-row: site 353: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:6:9-6:16) */
        { Site { function: 81, file: 51, line: 6, column: 9, end_line: 6, end_column: 16 } },
        /* terrane-site-row: site 354: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:8:17-8:29) */
        { Site { function: 81, file: 51, line: 8, column: 17, end_line: 8, end_column: 29 } },
        /* terrane-site-row: site 355: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:9:13-9:20) */
        { Site { function: 81, file: 51, line: 9, column: 13, end_line: 9, end_column: 20 } },
        /* terrane-site-row: site 356: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:12:9-12:16) */
        { Site { function: 81, file: 51, line: 12, column: 9, end_line: 12, end_column: 16 } },
        /* terrane-site-row: site 357: /native-try-unconditional-rewrite-control::test-scenario (tests/integration/native-try-unconditional-rewrite-control/case.trn:13:13-13:20) */
        { Site { function: 81, file: 51, line: 13, column: 13, end_line: 13, end_column: 20 } },
        /* terrane-site-row: site 358: /native-typed-metadata::own-label (tests/integration/native-typed-metadata/case.trn:6:12-6:38) */
        { Site { function: 82, file: 52, line: 6, column: 12, end_line: 6, end_column: 38 } },
        /* terrane-site-row: site 359: /native-typed-metadata::type-name (tests/integration/native-typed-metadata/case.trn:9:12-9:29) */
        { Site { function: 83, file: 52, line: 9, column: 12, end_line: 9, end_column: 29 } },
        /* terrane-site-row: site 360: /native-typed-metadata::snapshot (tests/integration/native-typed-metadata/case.trn:12:46-12:72) */
        { Site { function: 84, file: 52, line: 12, column: 46, end_line: 12, end_column: 72 } },
        /* terrane-site-row: site 361: /native-typed-metadata::snapshot-text (tests/integration/native-typed-metadata/case.trn:16:12-16:35) */
        { Site { function: 85, file: 52, line: 16, column: 12, end_line: 16, end_column: 35 } },
        /* terrane-site-row: site 362: /native-typed-metadata::own-found (tests/integration/native-typed-metadata/case.trn:19:12-19:27) */
        { Site { function: 86, file: 52, line: 19, column: 12, end_line: 19, end_column: 27 } },
        /* terrane-site-row: site 363: /native-typed-metadata::find-text (tests/integration/native-typed-metadata/case.trn:22:13-22:29) */
        { Site { function: 87, file: 52, line: 22, column: 13, end_line: 22, end_column: 29 } },
        /* terrane-site-row: site 364: /native-typed-metadata::find-text (tests/integration/native-typed-metadata/case.trn:22:12-22:45) */
        { Site { function: 87, file: 52, line: 22, column: 12, end_line: 22, end_column: 45 } },
        /* terrane-site-row: site 365: /native-typed-metadata::find-text-aliased (tests/integration/native-typed-metadata/case.trn:25:13-25:37) */
        { Site { function: 88, file: 52, line: 25, column: 13, end_line: 25, end_column: 37 } },
        /* terrane-site-row: site 366: /native-typed-metadata::find-text-aliased (tests/integration/native-typed-metadata/case.trn:25:12-25:53) */
        { Site { function: 88, file: 52, line: 25, column: 12, end_line: 25, end_column: 53 } },
        /* terrane-site-row: site 367: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:28:14-28:46) */
        { Site { function: 89, file: 52, line: 28, column: 14, end_line: 28, end_column: 46 } },
        /* terrane-site-row: site 368: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:30:13-30:34) */
        { Site { function: 89, file: 52, line: 30, column: 13, end_line: 30, end_column: 34 } },
        /* terrane-site-row: site 369: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:32:13-32:28) */
        { Site { function: 89, file: 52, line: 32, column: 13, end_line: 32, end_column: 28 } },
        /* terrane-site-row: site 370: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:33:13-33:34) */
        { Site { function: 89, file: 52, line: 33, column: 13, end_line: 33, end_column: 34 } },
        /* terrane-site-row: site 371: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:34:15-34:53) */
        { Site { function: 89, file: 52, line: 34, column: 15, end_line: 34, end_column: 53 } },
        /* terrane-site-row: site 372: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:37:13-37:34) */
        { Site { function: 89, file: 52, line: 37, column: 13, end_line: 37, end_column: 34 } },
        /* terrane-site-row: site 373: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:38:14-38:36) */
        { Site { function: 89, file: 52, line: 38, column: 14, end_line: 38, end_column: 36 } },
        /* terrane-site-row: site 374: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:38:13-38:47) */
        { Site { function: 89, file: 52, line: 38, column: 13, end_line: 38, end_column: 47 } },
        /* terrane-site-row: site 375: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:39:14-39:35) */
        { Site { function: 89, file: 52, line: 39, column: 14, end_line: 39, end_column: 35 } },
        /* terrane-site-row: site 376: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:39:13-39:46) */
        { Site { function: 89, file: 52, line: 39, column: 13, end_line: 39, end_column: 46 } },
        /* terrane-site-row: site 377: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:40:13-40:34) */
        { Site { function: 89, file: 52, line: 40, column: 13, end_line: 40, end_column: 34 } },
        /* terrane-site-row: site 378: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:41:33-41:65) */
        { Site { function: 89, file: 52, line: 41, column: 33, end_line: 41, end_column: 65 } },
        /* terrane-site-row: site 379: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:52:13-52:34) */
        { Site { function: 89, file: 52, line: 52, column: 13, end_line: 52, end_column: 34 } },
        /* terrane-site-row: site 380: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:55:35-55:64) */
        { Site { function: 89, file: 52, line: 55, column: 35, end_line: 55, end_column: 64 } },
        /* terrane-site-row: site 381: /native-typed-metadata::test-scenario (tests/integration/native-typed-metadata/case.trn:55:34-55:80) */
        { Site { function: 89, file: 52, line: 55, column: 34, end_line: 55, end_column: 80 } },
        /* terrane-site-row: site 382: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:6:14-6:26) */
        { Site { function: 90, file: 53, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 383: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:7:17-7:29) */
        { Site { function: 90, file: 53, line: 7, column: 17, end_line: 7, end_column: 29 } },
        /* terrane-site-row: site 384: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:9:26-9:43) */
        { Site { function: 90, file: 53, line: 9, column: 26, end_line: 9, end_column: 43 } },
        /* terrane-site-row: site 385: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:11:17-11:28) */
        { Site { function: 90, file: 53, line: 11, column: 17, end_line: 11, end_column: 28 } },
        /* terrane-site-row: site 386: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:12:9-12:24) */
        { Site { function: 90, file: 53, line: 12, column: 9, end_line: 12, end_column: 24 } },
        /* terrane-site-row: site 387: /native-unordered-map-dead-lender-control::test-scenario (tests/integration/native-unordered-map-dead-lender-control/case.trn:13:5-13:17) */
        { Site { function: 90, file: 53, line: 13, column: 5, end_line: 13, end_column: 17 } },
        /* terrane-site-row: site 388: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:12:9-12:21) */
        { Site { function: 91, file: 54, line: 12, column: 9, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 389: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:13:9-13:16) */
        { Site { function: 91, file: 54, line: 13, column: 9, end_line: 13, end_column: 16 } },
        /* terrane-site-row: site 390: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:14:21-14:28) */
        { Site { function: 91, file: 54, line: 14, column: 21, end_line: 14, end_column: 28 } },
        /* terrane-site-row: site 391: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:16:9-16:16) */
        { Site { function: 91, file: 54, line: 16, column: 9, end_line: 16, end_column: 16 } },
        /* terrane-site-row: site 392: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:17:13-17:20) */
        { Site { function: 91, file: 54, line: 17, column: 13, end_line: 17, end_column: 20 } },
        /* terrane-site-row: site 393: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:19:22-19:34) */
        { Site { function: 91, file: 54, line: 19, column: 22, end_line: 19, end_column: 34 } },
        /* terrane-site-row: site 394: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:20:15-20:35) */
        { Site { function: 91, file: 54, line: 20, column: 15, end_line: 20, end_column: 35 } },
        /* terrane-site-row: site 395: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:21:27-21:47) */
        { Site { function: 91, file: 54, line: 21, column: 27, end_line: 21, end_column: 47 } },
        /* terrane-site-row: site 396: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:23:15-23:35) */
        { Site { function: 91, file: 54, line: 23, column: 15, end_line: 23, end_column: 35 } },
        /* terrane-site-row: site 397: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:24:9-24:21) */
        { Site { function: 91, file: 54, line: 24, column: 9, end_line: 24, end_column: 21 } },
        /* terrane-site-row: site 398: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:25:11-25:18) */
        { Site { function: 91, file: 54, line: 25, column: 11, end_line: 25, end_column: 18 } },
        /* terrane-site-row: site 399: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:26:27-26:34) */
        { Site { function: 91, file: 54, line: 26, column: 27, end_line: 26, end_column: 34 } },
        /* terrane-site-row: site 400: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:28:13-28:25) */
        { Site { function: 91, file: 54, line: 28, column: 13, end_line: 28, end_column: 25 } },
        /* terrane-site-row: site 401: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:29:11-29:22) */
        { Site { function: 91, file: 54, line: 29, column: 11, end_line: 29, end_column: 22 } },
        /* terrane-site-row: site 402: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:31:13-31:20) */
        { Site { function: 91, file: 54, line: 31, column: 13, end_line: 31, end_column: 20 } },
        /* terrane-site-row: site 403: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:33:13-33:20) */
        { Site { function: 91, file: 54, line: 33, column: 13, end_line: 33, end_column: 20 } },
        /* terrane-site-row: site 404: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:35:19-35:26) */
        { Site { function: 91, file: 54, line: 35, column: 19, end_line: 35, end_column: 26 } },
        /* terrane-site-row: site 405: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:37:19-37:30) */
        { Site { function: 91, file: 54, line: 37, column: 19, end_line: 37, end_column: 30 } },
        /* terrane-site-row: site 406: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:39:20-39:32) */
        { Site { function: 91, file: 54, line: 39, column: 20, end_line: 39, end_column: 32 } },
        /* terrane-site-row: site 407: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:40:20-40:32) */
        { Site { function: 91, file: 54, line: 40, column: 20, end_line: 40, end_column: 32 } },
        /* terrane-site-row: site 408: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:41:17-41:29) */
        { Site { function: 91, file: 54, line: 41, column: 17, end_line: 41, end_column: 29 } },
        /* terrane-site-row: site 409: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:42:18-42:41) */
        { Site { function: 91, file: 54, line: 42, column: 18, end_line: 42, end_column: 41 } },
        /* terrane-site-row: site 410: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:43:19-43:42) */
        { Site { function: 91, file: 54, line: 43, column: 19, end_line: 43, end_column: 42 } },
        /* terrane-site-row: site 411: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:45:22-45:45) */
        { Site { function: 91, file: 54, line: 45, column: 22, end_line: 45, end_column: 45 } },
        /* terrane-site-row: site 412: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:46:9-46:28) */
        { Site { function: 91, file: 54, line: 46, column: 9, end_line: 46, end_column: 28 } },
        /* terrane-site-row: site 413: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:49:5-49:20) */
        { Site { function: 91, file: 54, line: 49, column: 5, end_line: 49, end_column: 20 } },
        /* terrane-site-row: site 414: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:50:13-50:30) */
        { Site { function: 91, file: 54, line: 50, column: 13, end_line: 50, end_column: 30 } },
        /* terrane-site-row: site 415: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:51:19-51:31) */
        { Site { function: 91, file: 54, line: 51, column: 19, end_line: 51, end_column: 31 } },
        /* terrane-site-row: site 416: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:52:14-52:26) */
        { Site { function: 91, file: 54, line: 52, column: 14, end_line: 52, end_column: 26 } },
        /* terrane-site-row: site 417: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:53:18-53:30) */
        { Site { function: 91, file: 54, line: 53, column: 18, end_line: 53, end_column: 30 } },
        /* terrane-site-row: site 418: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:54:17-54:39) */
        { Site { function: 91, file: 54, line: 54, column: 17, end_line: 54, end_column: 39 } },
        /* terrane-site-row: site 419: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:55:18-55:40) */
        { Site { function: 91, file: 54, line: 55, column: 18, end_line: 55, end_column: 40 } },
        /* terrane-site-row: site 420: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:57:21-57:38) */
        { Site { function: 91, file: 54, line: 57, column: 21, end_line: 57, end_column: 38 } },
        /* terrane-site-row: site 421: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:60:5-60:21) */
        { Site { function: 91, file: 54, line: 60, column: 5, end_line: 60, end_column: 21 } },
        /* terrane-site-row: site 422: /native-view-list-rebind::test-scenario (tests/integration/native-view-list-rebind/case.trn:61:13-61:29) */
        { Site { function: 91, file: 54, line: 61, column: 13, end_line: 61, end_column: 29 } },
        /* terrane-site-row: site 423: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:6:12-6:29) */
        { Site { function: 92, file: 55, line: 6, column: 12, end_line: 6, end_column: 29 } },
        /* terrane-site-row: site 424: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:11:13-11:28) */
        { Site { function: 92, file: 55, line: 11, column: 13, end_line: 11, end_column: 28 } },
        /* terrane-site-row: site 425: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:11:13-11:34) */
        { Site { function: 92, file: 55, line: 11, column: 13, end_line: 11, end_column: 34 } },
        /* terrane-site-row: site 426: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:12:13-12:27) */
        { Site { function: 92, file: 55, line: 12, column: 13, end_line: 12, end_column: 27 } },
        /* terrane-site-row: site 427: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:12:13-12:33) */
        { Site { function: 92, file: 55, line: 12, column: 13, end_line: 12, end_column: 33 } },
        /* terrane-site-row: site 428: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:17:13-17:31) */
        { Site { function: 92, file: 55, line: 17, column: 13, end_line: 17, end_column: 31 } },
        /* terrane-site-row: site 429: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:17:13-17:37) */
        { Site { function: 92, file: 55, line: 17, column: 13, end_line: 17, end_column: 37 } },
        /* terrane-site-row: site 430: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:18:13-18:30) */
        { Site { function: 92, file: 55, line: 18, column: 13, end_line: 18, end_column: 30 } },
        /* terrane-site-row: site 431: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:18:13-18:36) */
        { Site { function: 92, file: 55, line: 18, column: 13, end_line: 18, end_column: 36 } },
        /* terrane-site-row: site 432: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:21:13-21:27) */
        { Site { function: 92, file: 55, line: 21, column: 13, end_line: 21, end_column: 27 } },
        /* terrane-site-row: site 433: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:21:13-21:30) */
        { Site { function: 92, file: 55, line: 21, column: 13, end_line: 21, end_column: 30 } },
        /* terrane-site-row: site 434: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:21:13-21:36) */
        { Site { function: 92, file: 55, line: 21, column: 13, end_line: 21, end_column: 36 } },
        /* terrane-site-row: site 435: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:24:13-24:21) */
        { Site { function: 92, file: 55, line: 24, column: 13, end_line: 24, end_column: 21 } },
        /* terrane-site-row: site 436: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:24:13-24:27) */
        { Site { function: 92, file: 55, line: 24, column: 13, end_line: 24, end_column: 27 } },
        /* terrane-site-row: site 437: /native-view-local-map-storage::store (tests/integration/native-view-local-map-storage/case.trn:25:5-25:17) */
        { Site { function: 92, file: 55, line: 25, column: 5, end_line: 25, end_column: 17 } },
        /* terrane-site-row: site 438: /native-view-local-map-storage::test-scenario (tests/integration/native-view-local-map-storage/case.trn:28:13-28:25) */
        { Site { function: 93, file: 55, line: 28, column: 13, end_line: 28, end_column: 25 } },
        /* terrane-site-row: site 439: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:6:14-6:26) */
        { Site { function: 94, file: 56, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 440: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:7:21-7:38) */
        { Site { function: 94, file: 56, line: 7, column: 21, end_line: 7, end_column: 38 } },
        /* terrane-site-row: site 441: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:9:17-9:28) */
        { Site { function: 94, file: 56, line: 9, column: 17, end_line: 9, end_column: 28 } },
        /* terrane-site-row: site 442: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:11:17-11:28) */
        { Site { function: 94, file: 56, line: 11, column: 17, end_line: 11, end_column: 28 } },
        /* terrane-site-row: site 443: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:13:17-13:28) */
        { Site { function: 94, file: 56, line: 13, column: 17, end_line: 13, end_column: 28 } },
        /* terrane-site-row: site 444: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:15:17-15:28) */
        { Site { function: 94, file: 56, line: 15, column: 17, end_line: 15, end_column: 28 } },
        /* terrane-site-row: site 445: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:18:21-18:32) */
        { Site { function: 94, file: 56, line: 18, column: 21, end_line: 18, end_column: 32 } },
        /* terrane-site-row: site 446: /native-view-loop-binders::test-scenario (tests/integration/native-view-loop-binders/case.trn:19:17-19:28) */
        { Site { function: 94, file: 56, line: 19, column: 17, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 447: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:6:14-6:26) */
        { Site { function: 95, file: 57, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 448: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:8:30-8:47) */
        { Site { function: 95, file: 57, line: 8, column: 30, end_line: 8, end_column: 47 } },
        /* terrane-site-row: site 449: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:10:17-10:28) */
        { Site { function: 95, file: 57, line: 10, column: 17, end_line: 10, end_column: 28 } },
        /* terrane-site-row: site 450: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:13:34-13:51) */
        { Site { function: 95, file: 57, line: 13, column: 34, end_line: 13, end_column: 51 } },
        /* terrane-site-row: site 451: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:15:17-15:28) */
        { Site { function: 95, file: 57, line: 15, column: 17, end_line: 15, end_column: 28 } },
        /* terrane-site-row: site 452: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:18:35-18:52) */
        { Site { function: 95, file: 57, line: 18, column: 35, end_line: 18, end_column: 52 } },
        /* terrane-site-row: site 453: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:23:21-23:32) */
        { Site { function: 95, file: 57, line: 23, column: 21, end_line: 23, end_column: 32 } },
        /* terrane-site-row: site 454: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:26:37-26:54) */
        { Site { function: 95, file: 57, line: 26, column: 37, end_line: 26, end_column: 54 } },
        /* terrane-site-row: site 455: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:31:21-31:32) */
        { Site { function: 95, file: 57, line: 31, column: 21, end_line: 31, end_column: 32 } },
        /* terrane-site-row: site 456: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:34:17-34:28) */
        { Site { function: 95, file: 57, line: 34, column: 17, end_line: 34, end_column: 28 } },
        /* terrane-site-row: site 457: /native-view-map-entry-iteration::test-scenario (tests/integration/native-view-map-entry-iteration/case.trn:35:32-35:49) */
        { Site { function: 95, file: 57, line: 35, column: 32, end_line: 35, end_column: 49 } },
        /* terrane-site-row: site 458: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:6:14-6:26) */
        { Site { function: 96, file: 58, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 459: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:7:40-7:57) */
        { Site { function: 96, file: 58, line: 7, column: 40, end_line: 7, end_column: 57 } },
        /* terrane-site-row: site 460: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:9:17-9:28) */
        { Site { function: 96, file: 58, line: 9, column: 17, end_line: 9, end_column: 28 } },
        /* terrane-site-row: site 461: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:12:17-12:28) */
        { Site { function: 96, file: 58, line: 12, column: 17, end_line: 12, end_column: 28 } },
        /* terrane-site-row: site 462: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:13:24-13:41) */
        { Site { function: 96, file: 58, line: 13, column: 24, end_line: 13, end_column: 41 } },
        /* terrane-site-row: site 463: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:15:17-15:28) */
        { Site { function: 96, file: 58, line: 15, column: 17, end_line: 15, end_column: 28 } },
        /* terrane-site-row: site 464: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:17:17-17:28) */
        { Site { function: 96, file: 58, line: 17, column: 17, end_line: 17, end_column: 28 } },
        /* terrane-site-row: site 465: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:18:27-18:44) */
        { Site { function: 96, file: 58, line: 18, column: 27, end_line: 18, end_column: 44 } },
        /* terrane-site-row: site 466: /native-view-tuple-iteration::test-scenario (tests/integration/native-view-tuple-iteration/case.trn:19:17-19:28) */
        { Site { function: 96, file: 58, line: 19, column: 17, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 467: /native-view-value-iteration::observe (tests/integration/native-view-value-iteration/case.trn:7:17-7:28) */
        { Site { function: 97, file: 59, line: 7, column: 17, end_line: 7, end_column: 28 } },
        /* terrane-site-row: site 468: /native-view-value-iteration::observe-ref (tests/integration/native-view-value-iteration/case.trn:12:17-12:28) */
        { Site { function: 98, file: 59, line: 12, column: 17, end_line: 12, end_column: 28 } },
        /* terrane-site-row: site 469: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:16:14-16:26) */
        { Site { function: 99, file: 59, line: 16, column: 14, end_line: 16, end_column: 26 } },
        /* terrane-site-row: site 470: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:17:37-17:54) */
        { Site { function: 99, file: 59, line: 17, column: 37, end_line: 17, end_column: 54 } },
        /* terrane-site-row: site 471: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:19:17-19:28) */
        { Site { function: 99, file: 59, line: 19, column: 17, end_line: 19, end_column: 28 } },
        /* terrane-site-row: site 472: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:23:17-23:28) */
        { Site { function: 99, file: 59, line: 23, column: 17, end_line: 23, end_column: 28 } },
        /* terrane-site-row: site 473: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:24:23-24:40) */
        { Site { function: 99, file: 59, line: 24, column: 23, end_line: 24, end_column: 40 } },
        /* terrane-site-row: site 474: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:26:17-26:28) */
        { Site { function: 99, file: 59, line: 26, column: 17, end_line: 26, end_column: 28 } },
        /* terrane-site-row: site 475: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:30:27-30:44) */
        { Site { function: 99, file: 59, line: 30, column: 27, end_line: 30, end_column: 44 } },
        /* terrane-site-row: site 476: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:31:17-31:28) */
        { Site { function: 99, file: 59, line: 31, column: 17, end_line: 31, end_column: 28 } },
        /* terrane-site-row: site 477: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:36:24-36:41) */
        { Site { function: 99, file: 59, line: 36, column: 24, end_line: 36, end_column: 41 } },
        /* terrane-site-row: site 478: /native-view-value-iteration::test-scenario (tests/integration/native-view-value-iteration/case.trn:37:17-37:28) */
        { Site { function: 99, file: 59, line: 37, column: 17, end_line: 37, end_column: 28 } },
        /* terrane-site-row: site 479: /nested-noncopy-view-loops::test-scenario (tests/integration/nested-noncopy-view-loops/case.trn:6:14-6:26) */
        { Site { function: 100, file: 60, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 480: /nested-noncopy-view-loops::test-scenario (tests/integration/nested-noncopy-view-loops/case.trn:7:28-7:40) */
        { Site { function: 100, file: 60, line: 7, column: 28, end_line: 7, end_column: 40 } },
        /* terrane-site-row: site 481: /nested-noncopy-view-loops::test-scenario (tests/integration/nested-noncopy-view-loops/case.trn:10:21-10:32) */
        { Site { function: 100, file: 60, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 482: /nested-view-loops::test-scenario (tests/integration/nested-view-loops/case.trn:6:14-6:26) */
        { Site { function: 101, file: 61, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 483: /nested-view-loops::test-scenario (tests/integration/nested-view-loops/case.trn:7:28-7:45) */
        { Site { function: 101, file: 61, line: 7, column: 28, end_line: 7, end_column: 45 } },
        /* terrane-site-row: site 484: /nested-view-loops::test-scenario (tests/integration/nested-view-loops/case.trn:10:21-10:32) */
        { Site { function: 101, file: 61, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 485: /nested-view-ref-value-loop::test-scenario (tests/integration/nested-view-ref-value-loop/case.trn:6:14-6:26) */
        { Site { function: 102, file: 62, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 486: /nested-view-ref-value-loop::test-scenario (tests/integration/nested-view-ref-value-loop/case.trn:7:28-7:45) */
        { Site { function: 102, file: 62, line: 7, column: 28, end_line: 7, end_column: 45 } },
        /* terrane-site-row: site 487: /nested-view-ref-value-loop::test-scenario (tests/integration/nested-view-ref-value-loop/case.trn:10:21-10:32) */
        { Site { function: 102, file: 62, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 488: /nested-view-value-ref-loop::test-scenario (tests/integration/nested-view-value-ref-loop/case.trn:6:14-6:26) */
        { Site { function: 103, file: 63, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 489: /nested-view-value-ref-loop::test-scenario (tests/integration/nested-view-value-ref-loop/case.trn:7:28-7:45) */
        { Site { function: 103, file: 63, line: 7, column: 28, end_line: 7, end_column: 45 } },
        /* terrane-site-row: site 490: /nested-view-value-ref-loop::test-scenario (tests/integration/nested-view-value-ref-loop/case.trn:10:21-10:32) */
        { Site { function: 103, file: 63, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 491: /nested-view-value-value-loop::test-scenario (tests/integration/nested-view-value-value-loop/case.trn:6:14-6:26) */
        { Site { function: 104, file: 64, line: 6, column: 14, end_line: 6, end_column: 26 } },
        /* terrane-site-row: site 492: /nested-view-value-value-loop::test-scenario (tests/integration/nested-view-value-value-loop/case.trn:7:28-7:45) */
        { Site { function: 104, file: 64, line: 7, column: 28, end_line: 7, end_column: 45 } },
        /* terrane-site-row: site 493: /nested-view-value-value-loop::test-scenario (tests/integration/nested-view-value-value-loop/case.trn:10:21-10:32) */
        { Site { function: 104, file: 64, line: 10, column: 21, end_line: 10, end_column: 32 } },
        /* terrane-site-row: site 494: /projected-deep-rustdoc::read-again (tests/integration/projected-deep-rustdoc/case.trn:6:12-6:28) */
        { Site { function: 105, file: 65, line: 6, column: 12, end_line: 6, end_column: 28 } },
        /* terrane-site-row: site 495: /projected-deep-rustdoc::test-scenario (tests/integration/projected-deep-rustdoc/case.trn:9:18-9:28) */
        { Site { function: 106, file: 65, line: 9, column: 18, end_line: 9, end_column: 28 } },
        /* terrane-site-row: site 496: /projected-deep-rustdoc::test-scenario (tests/integration/projected-deep-rustdoc/case.trn:11:13-11:29) */
        { Site { function: 106, file: 65, line: 11, column: 13, end_line: 11, end_column: 29 } },
        /* terrane-site-row: site 497: /sequential-pipeline-native-copy::positive (tests/integration/sequential-pipeline-native-copy/case.trn:11:13-11:25) */
        { Site { function: 107, file: 66, line: 11, column: 13, end_line: 11, end_column: 25 } },
        /* terrane-site-row: site 498: /sequential-pipeline-native-copy::test-scenario (tests/integration/sequential-pipeline-native-copy/case.trn:14:21-14:38) */
        { Site { function: 108, file: 66, line: 14, column: 21, end_line: 14, end_column: 38 } },
        /* terrane-site-row: site 499: /sequential-pipeline-native-copy::test-scenario (tests/integration/sequential-pipeline-native-copy/case.trn:14:42-14:59) */
        { Site { function: 108, file: 66, line: 14, column: 42, end_line: 14, end_column: 59 } },
        /* terrane-site-row: site 500: /sequential-pipeline-native-copy::test-scenario (tests/integration/sequential-pipeline-native-copy/case.trn:15:5-15:29) */
        { Site { function: 108, file: 66, line: 15, column: 5, end_line: 15, end_column: 29 } },
        /* terrane-site-row: site 501: /sequential-pipeline-native-copy::test-scenario (tests/integration/sequential-pipeline-native-copy/case.trn:19:13-19:31) */
        { Site { function: 108, file: 66, line: 19, column: 13, end_line: 19, end_column: 31 } },
        /* terrane-site-row: site 502: /sequential-pipeline-native-copy::test-scenario (tests/integration/sequential-pipeline-native-copy/case.trn:25:13-25:31) */
        { Site { function: 108, file: 66, line: 25, column: 13, end_line: 25, end_column: 31 } },
        /* terrane-site-row: site 503: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:18:14-18:26) */
        { Site { function: 109, file: 67, line: 18, column: 14, end_line: 18, end_column: 26 } },
        /* terrane-site-row: site 504: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:19:20-19:37) */
        { Site { function: 109, file: 67, line: 19, column: 20, end_line: 19, end_column: 37 } },
        /* terrane-site-row: site 505: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:21:17-21:27) */
        { Site { function: 109, file: 67, line: 21, column: 17, end_line: 21, end_column: 27 } },
        /* terrane-site-row: site 506: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:24:21-24:33) */
        { Site { function: 109, file: 67, line: 24, column: 21, end_line: 24, end_column: 33 } },
        /* terrane-site-row: site 507: /shared-loop-read-control::test-scenario (tests/integration/shared-loop-read-control/case.trn:25:20-25:37) */
        { Site { function: 109, file: 67, line: 25, column: 20, end_line: 25, end_column: 37 } },
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
pub use terrane_query_witness::CopyProbe;
pub type CopyView<'a> = terrane_query_witness::CopyView<'a>;
pub type CowOfTerraneQueryWitnessMetadataText<'__terrane_view_0> = std::borrow::Cow<
    '__terrane_view_0,
    terrane_query_witness::MetadataText,
>;
pub use terrane_query_witness::Deep;
pub use terrane_query_witness::FromInput;
pub use terrane_query_witness::FromText;
pub use terrane_query_witness::Holder;
pub use terrane_query_witness::InherentFrom;
pub use terrane_query_witness::Layer;
pub use terrane_query_witness::MetadataRequest;
pub use terrane_query_witness::MetadataTable;
pub use terrane_query_witness::MetadataText;
pub use terrane_query_witness::NE;
pub use terrane_query_witness::Octets;
pub use terrane_query_witness::OrdinalRow;
pub use terrane_query_witness::OriginalTarget;
pub use terrane_query_witness::ReasonPhrase;
pub use terrane_query_witness::RecordIter as RecordIterOfTerraneQueryWitnessMetadataText;
pub use terrane_query_witness::RecordValues as RecordValuesOfTerraneQueryWitnessMetadataText;
pub use terrane_query_witness::Records;
pub use terrane_query_witness::RequestTarget;
pub use terrane_query_witness::ResponseCode;
pub use terrane_query_witness::SliceCatalog;
pub use terrane_query_witness::SliceEntry;
pub use terrane_query_witness::TargetParts;
pub use terrane_query_witness::Token;
pub use terrane_query_witness::TypedRecord;
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
pub fn terrane_static_trn_436f707950726f6265_clones() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::CopyProbe::clones()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::CopyProbe",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_436f707950726f6265_new(
    value: terrane_int_support::Int,
) -> Result<CopyProbe, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::CopyProbe::new(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::CopyProbe",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_436f707950726f6265_reset_clones() -> Result<
    (),
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::CopyProbe::reset_clones()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::CopyProbe",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_46726f6d496e707574_new() -> Result<
    FromInput,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::FromInput::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::FromInput",
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
pub fn terrane_static_trn_496e686572656e7446726f6d_from(
    value: terrane_int_support::Int,
) -> Result<InherentFrom, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::InherentFrom::from(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::InherentFrom",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_4d6574616461746152657175657374_new(
    label_: &str,
) -> Result<MetadataRequest, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::MetadataRequest::new(
            label_,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::MetadataRequest",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_4d6574616461746154657874_copies() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::MetadataText::copies()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::MetadataText",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_4f6374657473_new() -> Result<
    Octets,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::Octets::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::Octets",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_4f7264696e616c526f77_sample() -> Result<
    OrdinalRow,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::OrdinalRow::sample()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::OrdinalRow",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_4f726967696e616c546172676574_clones() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::OriginalTarget::clones()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::OriginalTarget",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_4f726967696e616c546172676574_new(
    path: &str,
) -> Result<OriginalTarget, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::OriginalTarget::new(path)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::OriginalTarget",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_526561736f6e506872617365_new(
    text: &str,
) -> Result<ReasonPhrase, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::ReasonPhrase::new(text)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::ReasonPhrase",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_5265636f726473_new(
    with_match: bool,
) -> Result<Records, crate::TerraneForeignError> {
    let with_match = with_match;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::Records::new(with_match)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::Records",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_526573706f6e7365436f6465_new(
    code: terrane_int_support::Int,
) -> Result<ResponseCode, crate::TerraneForeignError> {
    let code = terrane_int_support::coerce::<i64>(&code)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::ResponseCode::new(code)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::ResponseCode",
                ),
            )
        }
    }
}
pub fn terrane_static_trn_536c696365436174616c6f67_sample() -> Result<
    SliceCatalog,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::SliceCatalog::sample()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::SliceCatalog",
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
pub fn terrane_static_trn_54797065645265636f7264_new(
    label_: &str,
) -> Result<TypedRecord, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::TypedRecord::new(label_)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::TypedRecord",
                ),
            )
        }
    }
}
pub fn choose_reason(
    code: terrane_int_support::Int,
    reason: &str,
) -> Result<Option<String>, crate::TerraneForeignError> {
    let code = terrane_int_support::coerce::<i64>(&code)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::choose_reason(
            code,
            reason,
        )),
    ) {
        Ok(Ok(value)) => Ok(value.map(|value| value.to_owned())),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `{}` member `{}` failed: {error}",
                            "terrane-query-witness",
                            "terrane_query_witness::choose_reason"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::choose_reason",
                ),
            )
        }
    }
}
pub fn make_deep() -> Result<
    terrane_query_witness::Layer<
        terrane_query_witness::Layer<
            terrane_query_witness::Layer<
                terrane_query_witness::Layer<
                    terrane_query_witness::Layer<
                        terrane_query_witness::Layer<
                            terrane_query_witness::Layer<
                                terrane_query_witness::Layer<
                                    terrane_query_witness::Layer<
                                        terrane_query_witness::Layer<
                                            terrane_query_witness::Layer<
                                                terrane_query_witness::Layer<
                                                    terrane_query_witness::Layer<
                                                        terrane_query_witness::Layer<
                                                            terrane_query_witness::Layer<
                                                                terrane_query_witness::Layer<
                                                                    terrane_query_witness::Layer<
                                                                        terrane_query_witness::Layer<
                                                                            terrane_query_witness::Layer<
                                                                                terrane_query_witness::Layer<
                                                                                    terrane_query_witness::Layer<
                                                                                        terrane_query_witness::Layer<
                                                                                            terrane_query_witness::Layer<
                                                                                                terrane_query_witness::Layer<
                                                                                                    terrane_query_witness::Layer<
                                                                                                        terrane_query_witness::Layer<
                                                                                                            terrane_query_witness::Layer<
                                                                                                                terrane_query_witness::Layer<
                                                                                                                    terrane_query_witness::Layer<
                                                                                                                        terrane_query_witness::Layer<
                                                                                                                            terrane_query_witness::Layer<
                                                                                                                                terrane_query_witness::Layer<u8>,
                                                                                                                            >,
                                                                                                                        >,
                                                                                                                    >,
                                                                                                                >,
                                                                                                            >,
                                                                                                        >,
                                                                                                    >,
                                                                                                >,
                                                                                            >,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::make_deep()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::make_deep",
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
pub fn read_deep(
    value: &terrane_query_witness::Layer<
        terrane_query_witness::Layer<
            terrane_query_witness::Layer<
                terrane_query_witness::Layer<
                    terrane_query_witness::Layer<
                        terrane_query_witness::Layer<
                            terrane_query_witness::Layer<
                                terrane_query_witness::Layer<
                                    terrane_query_witness::Layer<
                                        terrane_query_witness::Layer<
                                            terrane_query_witness::Layer<
                                                terrane_query_witness::Layer<
                                                    terrane_query_witness::Layer<
                                                        terrane_query_witness::Layer<
                                                            terrane_query_witness::Layer<
                                                                terrane_query_witness::Layer<
                                                                    terrane_query_witness::Layer<
                                                                        terrane_query_witness::Layer<
                                                                            terrane_query_witness::Layer<
                                                                                terrane_query_witness::Layer<
                                                                                    terrane_query_witness::Layer<
                                                                                        terrane_query_witness::Layer<
                                                                                            terrane_query_witness::Layer<
                                                                                                terrane_query_witness::Layer<
                                                                                                    terrane_query_witness::Layer<
                                                                                                        terrane_query_witness::Layer<
                                                                                                            terrane_query_witness::Layer<
                                                                                                                terrane_query_witness::Layer<
                                                                                                                    terrane_query_witness::Layer<
                                                                                                                        terrane_query_witness::Layer<
                                                                                                                            terrane_query_witness::Layer<
                                                                                                                                terrane_query_witness::Layer<u8>,
                                                                                                                            >,
                                                                                                                        >,
                                                                                                                    >,
                                                                                                                >,
                                                                                                            >,
                                                                                                        >,
                                                                                                    >,
                                                                                                >,
                                                                                            >,
                                                                                        >,
                                                                                    >,
                                                                                >,
                                                                            >,
                                                                        >,
                                                                    >,
                                                                >,
                                                            >,
                                                        >,
                                                    >,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::read_deep(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::read_deep",
                ),
            )
        }
    }
}
pub fn string_allocation_id(
    value: &String,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_query_witness::string_allocation_id(
            value,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-query-witness",
                    "terrane_query_witness::string_allocation_id",
                ),
            )
        }
    }
}
// Source: core/modifiers.trn
// Namespace: core/modifiers
#[derive(Clone)]
pub struct Modifier {
    pub operation: String,
    pub targets: terrane_collection_support::List<String>,
    pub structural: bool,
    pub repeatable: bool,
}
impl Modifier {
    pub fn terrane_construct() -> Self {
        Self {
            operation: String::new(),
            targets: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("binding")]),
            structural: false,
            repeatable: false,
        }
    }
}
#[derive(Clone)]
pub struct ScopedAccess {}
impl ScopedAccess {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
#[derive(Clone)]
pub struct PipelineSequential {}
impl PipelineSequential {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
#[derive(Clone)]
pub struct InterpolationRenderer {
    pub key: String,
}
impl InterpolationRenderer {
    pub fn terrane_construct() -> Self {
        Self { key: String::new() }
    }
}
// Source: core/annotations.trn
// Namespace: core/annotations
#[derive(Clone)]
pub struct Annotation {
    pub targets: terrane_collection_support::List<AnnotationTarget>,
    pub repeatable: bool,
}
impl Annotation {
    pub fn terrane_construct() -> Self {
        Self {
            targets: terrane_collection_support::List::<
                AnnotationTarget,
            >::new(Vec::new()),
            repeatable: false,
        }
    }
}
#[derive(Clone)]
pub enum AnnotationTarget {
    Class,
    Callable,
    Parameter,
    Field,
}
#[derive(Clone)]
pub struct Descriptor {}
impl Descriptor {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
