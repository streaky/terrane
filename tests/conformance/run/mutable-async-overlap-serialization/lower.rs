// Generated deterministically by Terrane <version>.
// Runtime support: mutable_callable.rs, async_mutable_state.rs, async_native.rs, executor_parallel.rs, channels.rs, tasks_native_parallel.rs, time_inactive.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
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
    pub static FUNCTIONS: [&str; 1] = ["/mutable-async-overlap-serialization::main"];
    pub static SITES: [Site; 3] = [
        /* terrane-site-row: site 0: /mutable-async-overlap-serialization::main (case.trn:66:25-66:44) */
        { Site { function: 0, file: 0, line: 66, column: 25, end_line: 66, end_column: 44 } },
        /* terrane-site-row: site 1: /mutable-async-overlap-serialization::main (case.trn:68:25-68:44) */
        { Site { function: 0, file: 0, line: 68, column: 25, end_line: 68, end_column: 44 } },
        /* terrane-site-row: site 2: /mutable-async-overlap-serialization::main (case.trn:68:25-68:48) */
        { Site { function: 0, file: 0, line: 68, column: 25, end_line: 68, end_column: 48 } },
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
// Namespace: mutable-async-overlap-serialization
#[derive(Clone)]
pub struct GatedAccumulator {
    pub total: terrane_int_support::Int,
}
impl GatedAccumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub async fn add(
        &mut self,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let sent_terrane_f0_s272: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let released_terrane_f0_s350: TerraneChannelReceiveOutcome<
            terrane_int_support::Int,
        >;
        sent_terrane_f0_s272 = __terrane_await(Box::pin(started.send(delta.clone())))
            .await;
        if !sent_terrane_f0_s272.accepted {
            return terrane_int_support::Int::from(-1_i128);
        }
        released_terrane_f0_s350 = __terrane_await(Box::pin(gate.receive())).await;
        if !released_terrane_f0_s350.available {
            return terrane_int_support::Int::from(-2_i128);
        }
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
}
fn main() {
    __terrane_run(async move {
        let gate_a_terrane_f0_s509: TerraneChannelPair<terrane_int_support::Int>;
        let gate_b_terrane_f0_s551: TerraneChannelPair<terrane_int_support::Int>;
        let gate_c_terrane_f0_s593: TerraneChannelPair<terrane_int_support::Int>;
        let started_a_terrane_f0_s635: TerraneChannelPair<terrane_int_support::Int>;
        let started_b_terrane_f0_s680: TerraneChannelPair<terrane_int_support::Int>;
        let started_c_terrane_f0_s725: TerraneChannelPair<terrane_int_support::Int>;
        let value_terrane_f0_s770: GatedAccumulator;
        let bound_terrane_f0_s808: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let scope_terrane_f0_s919: TerraneTaskScope;
        let first_terrane_f0_s941: TerraneScopedTask<terrane_int_support::Int>;
        let first_start_terrane_f0_s1010: TerraneChannelReceiveOutcome<
            terrane_int_support::Int,
        >;
        let second_terrane_f0_s1102: TerraneScopedTask<terrane_int_support::Int>;
        let first_release_terrane_f0_s1173: TerraneChannelSendOutcome<
            terrane_int_support::Int,
        >;
        let first_outcome_terrane_f0_s1262: TerraneTaskOutcome<terrane_int_support::Int>;
        let second_start_terrane_f0_s1304: TerraneChannelReceiveOutcome<
            terrane_int_support::Int,
        >;
        let second_release_terrane_f0_s1398: TerraneChannelSendOutcome<
            terrane_int_support::Int,
        >;
        let second_outcome_terrane_f0_s1489: TerraneTaskOutcome<
            terrane_int_support::Int,
        >;
        let third_release_terrane_f0_s1533: TerraneChannelSendOutcome<
            terrane_int_support::Int,
        >;
        let third_terrane_f0_s1622: terrane_int_support::Int;
        let first_value_terrane_f0_s1686: Option<terrane_int_support::Int>;
        let second_value_terrane_f0_s1722: Option<terrane_int_support::Int>;
        let counter_terrane_f0_s1873: terrane_int_support::Int;
        let guarded_capture_terrane_f0_s1887: i8;
        let step_terrane_f0_s1915: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let closure_gate_a_terrane_f0_s2402: TerraneChannelPair<
            terrane_int_support::Int,
        >;
        let closure_gate_b_terrane_f0_s2452: TerraneChannelPair<
            terrane_int_support::Int,
        >;
        let closure_gate_c_terrane_f0_s2502: TerraneChannelPair<
            terrane_int_support::Int,
        >;
        let closure_started_a_terrane_f0_s2552: TerraneChannelPair<
            terrane_int_support::Int,
        >;
        let closure_started_b_terrane_f0_s2605: TerraneChannelPair<
            terrane_int_support::Int,
        >;
        let closure_started_c_terrane_f0_s2658: TerraneChannelPair<
            terrane_int_support::Int,
        >;
        let closure_scope_terrane_f0_s2711: TerraneTaskScope;
        let closure_first_terrane_f0_s2741: TerraneScopedTask<terrane_int_support::Int>;
        let closure_first_start_terrane_f0_s2841: TerraneChannelReceiveOutcome<
            terrane_int_support::Int,
        >;
        let closure_second_terrane_f0_s2957: TerraneScopedTask<terrane_int_support::Int>;
        let closure_first_release_terrane_f0_s3059: TerraneChannelSendOutcome<
            terrane_int_support::Int,
        >;
        let closure_first_outcome_terrane_f0_s3172: TerraneTaskOutcome<
            terrane_int_support::Int,
        >;
        let closure_second_start_terrane_f0_s3238: TerraneChannelReceiveOutcome<
            terrane_int_support::Int,
        >;
        let closure_second_release_terrane_f0_s3356: TerraneChannelSendOutcome<
            terrane_int_support::Int,
        >;
        let closure_second_outcome_terrane_f0_s3471: TerraneTaskOutcome<
            terrane_int_support::Int,
        >;
        let closure_third_release_terrane_f0_s3539: TerraneChannelSendOutcome<
            terrane_int_support::Int,
        >;
        let closure_third_terrane_f0_s3652: terrane_int_support::Int;
        let closure_first_value_terrane_f0_s3739: Option<terrane_int_support::Int>;
        let closure_second_value_terrane_f0_s3791: Option<terrane_int_support::Int>;
        gate_a_terrane_f0_s509 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate_b_terrane_f0_s551 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate_c_terrane_f0_s593 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_a_terrane_f0_s635 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_b_terrane_f0_s680 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_c_terrane_f0_s725 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        value_terrane_f0_s770 = GatedAccumulator::terrane_construct();
        bound_terrane_f0_s808 = {
            let receiver = TerraneAsyncMutableState::new(value_terrane_f0_s770.clone());
            TerraneMutableCallable::new(move |
                (
                    argument_0,
                    argument_1,
                    argument_2,
                ): (
                    terrane_int_support::Int,
                    TerraneChannelSender<terrane_int_support::Int>,
                    TerraneChannelReceiver<terrane_int_support::Int>,
                ),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                let receiver = receiver.share();
                Box::pin(async move {
                    receiver
                        .with_receiver(move |receiver| Box::pin(async move {
                            receiver.add(argument_0, argument_1, argument_2).await
                        }))
                        .await
                })
            })
        };
        scope_terrane_f0_s919 = TerraneTaskScope::new(None);
        first_terrane_f0_s941 = {
            let __terrane_scope = scope_terrane_f0_s919.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = bound_terrane_f0_s808
                .call((
                    terrane_int_support::Int::from(1_i128),
                    started_a_terrane_f0_s635.sender,
                    gate_a_terrane_f0_s509.receiver,
                ));
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        first_start_terrane_f0_s1010 = __terrane_await(
                started_a_terrane_f0_s635.receiver.receive(),
            )
            .await;
        if !first_start_terrane_f0_s1010.available {
            return ();
        }
        second_terrane_f0_s1102 = {
            let __terrane_scope = scope_terrane_f0_s919.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = bound_terrane_f0_s808
                .call((
                    terrane_int_support::Int::from(10_i128),
                    started_b_terrane_f0_s680.sender,
                    gate_b_terrane_f0_s551.receiver,
                ));
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        first_release_terrane_f0_s1173 = __terrane_await(
                gate_a_terrane_f0_s509
                    .sender
                    .send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !first_release_terrane_f0_s1173.accepted {
            return ();
        }
        first_outcome_terrane_f0_s1262 = __terrane_await(
                scope_terrane_f0_s919.join(first_terrane_f0_s941),
            )
            .await;
        second_start_terrane_f0_s1304 = __terrane_await(
                started_b_terrane_f0_s680.receiver.receive(),
            )
            .await;
        if !second_start_terrane_f0_s1304.available {
            return ();
        }
        second_release_terrane_f0_s1398 = __terrane_await(
                gate_b_terrane_f0_s551
                    .sender
                    .send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !second_release_terrane_f0_s1398.accepted {
            return ();
        }
        second_outcome_terrane_f0_s1489 = __terrane_await(
                scope_terrane_f0_s919.join(second_terrane_f0_s1102),
            )
            .await;
        third_release_terrane_f0_s1533 = __terrane_await(
                gate_c_terrane_f0_s593
                    .sender
                    .send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !third_release_terrane_f0_s1533.accepted {
            return ();
        }
        third_terrane_f0_s1622 = __terrane_await(
                bound_terrane_f0_s808
                    .call((
                        terrane_int_support::Int::from(100_i128),
                        started_c_terrane_f0_s725.sender,
                        gate_c_terrane_f0_s593.receiver,
                    )),
            )
            .await;
        first_value_terrane_f0_s1686 = first_outcome_terrane_f0_s1262.value.clone();
        second_value_terrane_f0_s1722 = second_outcome_terrane_f0_s1489.value.clone();
        if first_value_terrane_f0_s1686.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match
                &first_value_terrane_f0_s1686 { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") })
            );
        }
        if second_value_terrane_f0_s1722.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match
                &second_value_terrane_f0_s1722 { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") })
            );
        }
        println!("{}", terrane_scalar_support::scalar_text(&third_terrane_f0_s1622));
        counter_terrane_f0_s1873 = terrane_int_support::Int::from(0_i128);
        guarded_capture_terrane_f0_s1887 = 41;
        step_terrane_f0_s1915 = {
            let counter_terrane_f0_s1873 = TerraneAsyncMutableState::new(
                counter_terrane_f0_s1873.clone(),
            );
            let guarded_capture_terrane_f0_s1887 = TerraneAsyncMutableState::new(
                guarded_capture_terrane_f0_s1887.clone(),
            );
            let __terrane_invocation = TerraneAsyncInvocationGate::new();
            TerraneMutableCallable::new(move |
                (
                    delta,
                    started,
                    gate,
                ): (
                    terrane_int_support::Int,
                    TerraneChannelSender<terrane_int_support::Int>,
                    TerraneChannelReceiver<terrane_int_support::Int>,
                ),
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let counter_terrane_f0_s1873 = counter_terrane_f0_s1873.share();
                let guarded_capture_terrane_f0_s1887 = guarded_capture_terrane_f0_s1887
                    .share();
                let __terrane_invocation = __terrane_invocation.share();
                Box::pin(async move {
                    let _invocation = __terrane_invocation.enter().await;
                    let seen_terrane_f0_s2180: terrane_int_support::Int;
                    let sent_terrane_f0_s2199: TerraneChannelSendOutcome<
                        terrane_int_support::Int,
                    >;
                    let released_terrane_f0_s2277: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    >;
                    if guarded_capture_terrane_f0_s1887.snapshot().rem_euclid(2) == 0 {
                        {
                            let callable_capture_value = __terrane_raised(
                                terrane_int_support::fixed_division(
                                    guarded_capture_terrane_f0_s1887.snapshot(),
                                    2,
                                ),
                                0 /* terrane-site: case.trn:66:25-66:44 */,
                            );
                            guarded_capture_terrane_f0_s1887
                                .replace(callable_capture_value);
                        }
                    } else {
                        {
                            let callable_capture_value = __terrane_raised(
                                terrane_int_support::fixed_addition(
                                    __terrane_raised(
                                        terrane_int_support::fixed_multiplication(
                                            3,
                                            guarded_capture_terrane_f0_s1887.snapshot(),
                                        ),
                                        1 /* terrane-site: case.trn:68:25-68:44 */,
                                    ),
                                    1,
                                ),
                                2 /* terrane-site: case.trn:68:25-68:48 */,
                            );
                            guarded_capture_terrane_f0_s1887
                                .replace(callable_capture_value);
                        }
                    }
                    seen_terrane_f0_s2180 = counter_terrane_f0_s1873.snapshot();
                    sent_terrane_f0_s2199 = __terrane_await(
                            Box::pin(started.send(delta.clone())),
                        )
                        .await;
                    if !sent_terrane_f0_s2199.accepted {
                        return terrane_int_support::Int::from(-1_i128);
                    }
                    released_terrane_f0_s2277 = __terrane_await(Box::pin(gate.receive()))
                        .await;
                    if !released_terrane_f0_s2277.available {
                        return terrane_int_support::Int::from(-2_i128);
                    }
                    {
                        let callable_capture_value = seen_terrane_f0_s2180.clone()
                            + delta.clone();
                        counter_terrane_f0_s1873.replace(callable_capture_value);
                    }
                    return counter_terrane_f0_s1873.snapshot();
                })
            })
        };
        closure_gate_a_terrane_f0_s2402 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_gate_b_terrane_f0_s2452 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_gate_c_terrane_f0_s2502 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_started_a_terrane_f0_s2552 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_started_b_terrane_f0_s2605 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_started_c_terrane_f0_s2658 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_scope_terrane_f0_s2711 = TerraneTaskScope::new(None);
        closure_first_terrane_f0_s2741 = {
            let __terrane_scope = closure_scope_terrane_f0_s2711.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = step_terrane_f0_s1915
                .call((
                    terrane_int_support::Int::from(1_i128),
                    closure_started_a_terrane_f0_s2552.sender,
                    closure_gate_a_terrane_f0_s2402.receiver,
                ));
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        closure_first_start_terrane_f0_s2841 = __terrane_await(
                closure_started_a_terrane_f0_s2552.receiver.receive(),
            )
            .await;
        if !closure_first_start_terrane_f0_s2841.available {
            return ();
        }
        closure_second_terrane_f0_s2957 = {
            let __terrane_scope = closure_scope_terrane_f0_s2711.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = step_terrane_f0_s1915
                .call((
                    terrane_int_support::Int::from(10_i128),
                    closure_started_b_terrane_f0_s2605.sender,
                    closure_gate_b_terrane_f0_s2452.receiver,
                ));
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        closure_first_release_terrane_f0_s3059 = __terrane_await(
                closure_gate_a_terrane_f0_s2402
                    .sender
                    .send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !closure_first_release_terrane_f0_s3059.accepted {
            return ();
        }
        closure_first_outcome_terrane_f0_s3172 = __terrane_await(
                closure_scope_terrane_f0_s2711.join(closure_first_terrane_f0_s2741),
            )
            .await;
        closure_second_start_terrane_f0_s3238 = __terrane_await(
                closure_started_b_terrane_f0_s2605.receiver.receive(),
            )
            .await;
        if !closure_second_start_terrane_f0_s3238.available {
            return ();
        }
        closure_second_release_terrane_f0_s3356 = __terrane_await(
                closure_gate_b_terrane_f0_s2452
                    .sender
                    .send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !closure_second_release_terrane_f0_s3356.accepted {
            return ();
        }
        closure_second_outcome_terrane_f0_s3471 = __terrane_await(
                closure_scope_terrane_f0_s2711.join(closure_second_terrane_f0_s2957),
            )
            .await;
        closure_third_release_terrane_f0_s3539 = __terrane_await(
                closure_gate_c_terrane_f0_s2502
                    .sender
                    .send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !closure_third_release_terrane_f0_s3539.accepted {
            return ();
        }
        closure_third_terrane_f0_s3652 = __terrane_await(
                step_terrane_f0_s1915
                    .call((
                        terrane_int_support::Int::from(100_i128),
                        closure_started_c_terrane_f0_s2658.sender,
                        closure_gate_c_terrane_f0_s2502.receiver,
                    )),
            )
            .await;
        closure_first_value_terrane_f0_s3739 = closure_first_outcome_terrane_f0_s3172
            .value
            .clone();
        closure_second_value_terrane_f0_s3791 = closure_second_outcome_terrane_f0_s3471
            .value
            .clone();
        if closure_first_value_terrane_f0_s3739.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match
                &closure_first_value_terrane_f0_s3739 { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") })
            );
        }
        if closure_second_value_terrane_f0_s3791.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match
                &closure_second_value_terrane_f0_s3791 { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") })
            );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&closure_third_terrane_f0_s3652)
        );
    });
}
// Source: core/concurrency.trn
// Namespace: core/concurrency
#[derive(Clone)]
pub struct ConcurrencyOperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl ConcurrencyOperationResult {
    pub fn terrane_construct(
        did_fail: bool,
        exceeded_deadline: bool,
        detail: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(did_fail, exceeded_deadline, detail);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        exceeded_deadline: bool,
        detail: String,
    ) {
        self.failed = did_fail;
        self.deadline_exceeded = exceeded_deadline;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct ConcurrencyIntResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub available: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl ConcurrencyIntResult {
    pub fn terrane_construct(
        did_fail: bool,
        exceeded_deadline: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            available: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value
            .construct(did_fail, exceeded_deadline, has_value, detail, result_value);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        exceeded_deadline: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) {
        self.failed = did_fail;
        self.deadline_exceeded = exceeded_deadline;
        self.available = has_value;
        self.message = detail;
        self.value = result_value.clone();
    }
}
#[derive(Clone)]
pub struct IntMutex {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntMutex {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s966: TerranePlatformResult;
        raw_terrane_f1_s966 = terrane_platform_int_mutex(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s966);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s966);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s966);
    }
    pub fn load(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s1191: TerranePlatformResult;
        raw_terrane_f1_s1191 = terrane_platform_int_mutex_load(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1191),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1191),
            terrane_platform_result_bool(&raw_terrane_f1_s1191),
            terrane_platform_result_message(&raw_terrane_f1_s1191),
            terrane_platform_result_int(&raw_terrane_f1_s1191),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s1487: TerranePlatformResult;
        raw_terrane_f1_s1487 = terrane_platform_int_mutex_store(&self.handle, value);
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1487),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1487),
            terrane_platform_result_message(&raw_terrane_f1_s1487),
        );
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> ConcurrencyIntResult {
        let raw_terrane_f1_s1754: TerranePlatformResult;
        raw_terrane_f1_s1754 = terrane_platform_int_mutex_add(&self.handle, amount);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1754),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1754),
            terrane_platform_result_bool(&raw_terrane_f1_s1754),
            terrane_platform_result_message(&raw_terrane_f1_s1754),
            terrane_platform_result_int(&raw_terrane_f1_s1754),
        );
    }
}
#[derive(Clone)]
pub struct IntReadWriteLock {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntReadWriteLock {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s2175: TerranePlatformResult;
        raw_terrane_f1_s2175 = terrane_platform_int_rw_lock(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s2175);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s2175);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s2175);
    }
    pub fn read(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s2410: TerranePlatformResult;
        raw_terrane_f1_s2410 = terrane_platform_int_rw_lock_read(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s2410),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s2410),
            terrane_platform_result_bool(&raw_terrane_f1_s2410),
            terrane_platform_result_message(&raw_terrane_f1_s2410),
            terrane_platform_result_int(&raw_terrane_f1_s2410),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s2716: TerranePlatformResult;
        raw_terrane_f1_s2716 = terrane_platform_int_rw_lock_write(&self.handle, value);
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s2716),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s2716),
            terrane_platform_result_message(&raw_terrane_f1_s2716),
        );
    }
}
#[derive(Clone)]
pub struct MemoryOrder {
    pub name: String,
}
impl MemoryOrder {
    pub fn terrane_construct(ordering_name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::from("sequentially-consistent"),
        };
        __terrane_constructed_value.construct(ordering_name);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, ordering_name: String) {
        self.name = ordering_name;
    }
}
pub fn relaxed_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("relaxed"));
}
pub fn acquire_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("acquire"));
}
pub fn release_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("release"));
}
pub fn acquire_release_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("acquire-release"));
}
pub fn sequentially_consistent_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("sequentially-consistent"));
}
#[derive(Clone)]
pub struct AtomicInt64 {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl AtomicInt64 {
    pub fn terrane_construct(initial: i64) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: i64) {
        let raw_terrane_f1_s3708: TerranePlatformResult;
        raw_terrane_f1_s3708 = terrane_platform_atomic_int64(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s3708);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s3708);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s3708);
    }
    pub fn load(&self, ordering: MemoryOrder) -> ConcurrencyIntResult {
        let raw_terrane_f1_s3958: TerranePlatformResult;
        raw_terrane_f1_s3958 = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s3958),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s3958),
            terrane_platform_result_bool(&raw_terrane_f1_s3958),
            terrane_platform_result_message(&raw_terrane_f1_s3958),
            terrane_platform_result_int(&raw_terrane_f1_s3958),
        );
    }
    pub fn store(
        &self,
        value: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s4297: TerranePlatformResult;
        raw_terrane_f1_s4297 = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s4297),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s4297),
            terrane_platform_result_message(&raw_terrane_f1_s4297),
        );
    }
    pub fn increase(
        &mut self,
        amount: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyIntResult {
        let raw_terrane_f1_s4607: TerranePlatformResult;
        raw_terrane_f1_s4607 = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s4607),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s4607),
            terrane_platform_result_bool(&raw_terrane_f1_s4607),
            terrane_platform_result_message(&raw_terrane_f1_s4607),
            terrane_platform_result_int(&raw_terrane_f1_s4607),
        );
    }
}
#[derive(Clone)]
pub struct ThreadLocalInt {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl ThreadLocalInt {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s5043: TerranePlatformResult;
        raw_terrane_f1_s5043 = terrane_platform_thread_local_int(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s5043);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s5043);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s5043);
    }
    pub fn get(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s5274: TerranePlatformResult;
        raw_terrane_f1_s5274 = terrane_platform_thread_local_int_get(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5274),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5274),
            terrane_platform_result_bool(&raw_terrane_f1_s5274),
            terrane_platform_result_message(&raw_terrane_f1_s5274),
            terrane_platform_result_int(&raw_terrane_f1_s5274),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s5576: TerranePlatformResult;
        raw_terrane_f1_s5576 = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5576),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5576),
            terrane_platform_result_message(&raw_terrane_f1_s5576),
        );
    }
}
