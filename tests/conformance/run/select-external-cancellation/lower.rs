// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, channels.rs, tasks_native_parallel.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
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
    pub static FILES: [&str; 0] = [];
    pub static FUNCTIONS: [&str; 0] = [];
    pub static SITES: [Site; 0] = [];
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
// Namespace: select-external-cancellation
async fn contender(
    marker: String,
    started: TerraneChannelSender<terrane_int_support::Int>,
    gate: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    let sent: TerraneChannelSendOutcome<terrane_int_support::Int> = __terrane_await(
                            Box::pin(
                                started.send(terrane_int_support::Int::from(1_i128)),
                            ),
                        )
                        .await;
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(gate.receive())).await;
                    println!(
                        "{}{}", terrane_scalar_support::scalar_text(&sent.accepted),
                        terrane_scalar_support::scalar_text(&received.available)
                    );
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_try_0 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_0) => {
                        let mut __terrane_handled_0 = false;
                        if !__terrane_handled_0 {
                            return TerraneCompletion::Error(__terrane_error_0);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_0 = __terrane_maybe_completion_0.is_none();
    let mut __terrane_completion_0 = __terrane_maybe_completion_0
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&marker));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    if __terrane_cancelled_0
        && matches!(&__terrane_completion_0, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_0).await;
    }
    __terrane_finally_guard_0.finish();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn wait_for_either(
    started_a: TerraneChannelSender<terrane_int_support::Int>,
    gate_a: TerraneChannelReceiver<terrane_int_support::Int>,
    started_b: TerraneChannelSender<terrane_int_support::Int>,
    gate_b: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let mut __terrane_select_cursor_517 = 0usize;
    {
        let mut __terrane_select_guard_517 = __terrane_finally_guard();
        let __terrane_select_control_517_0 = __terrane_select_control();
        let mut __terrane_select_future_517_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_517_0.clone(),
            contender(String::from("cleanup-a"), started_a, gate_a))
        );
        let mut __terrane_select_result_517_0 = None;
        let __terrane_select_control_517_1 = __terrane_select_control();
        let mut __terrane_select_future_517_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_517_1.clone(),
            contender(String::from("cleanup-b"), started_b, gate_b))
        );
        let mut __terrane_select_result_517_1 = None;
        let __terrane_select_winner_517 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_517
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_517_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_517_0 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(0usize);
                                }
                                std::task::Poll::Ready(None) => {
                                    unreachable!(
                                        "case cancellation starts only after winner selection"
                                    )
                                }
                                std::task::Poll::Pending => {}
                            }
                        }
                        1 => {
                            match Future::poll(
                                __terrane_select_future_517_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_517_1 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(1usize);
                                }
                                std::task::Poll::Ready(None) => {
                                    unreachable!(
                                        "case cancellation starts only after winner selection"
                                    )
                                }
                                std::task::Poll::Pending => {}
                            }
                        }
                        _ => unreachable!("select candidate is within the case count"),
                    }
                }
                std::task::Poll::Pending
            })
            .await;
        if __terrane_select_winner_517 == usize::MAX {
            __terrane_select_control_517_1.request_cancel();
            __terrane_select_control_517_0.request_cancel();
            let _ = __terrane_select_future_517_1.as_mut().await;
            let _ = __terrane_select_future_517_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_517.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_517).await;
        }
        __terrane_select_cursor_517 = (__terrane_select_winner_517 + 1usize) % 2usize;
        match __terrane_select_winner_517 {
            0 => {
                __terrane_select_control_517_1.request_cancel();
                let _ = __terrane_select_future_517_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_517_0.request_cancel();
                let _ = __terrane_select_future_517_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_517.finish();
        match __terrane_select_winner_517 {
            0 => {
                let _ = __terrane_select_result_517_0
                    .take()
                    .expect("selected case owns its ready result");
                println!(
                    "{}", terrane_scalar_support::scalar_text(&String::from("a-won"))
                );
            }
            1 => {
                let _ = __terrane_select_result_517_1
                    .take()
                    .expect("selected case owns its ready result");
                println!(
                    "{}", terrane_scalar_support::scalar_text(&String::from("b-won"))
                );
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let started_a: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let started_b: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let gate_a: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let gate_b: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let child: TerraneScopedTask<()> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = wait_for_either(
                started_a.sender,
                gate_a.receiver,
                started_b.sender,
                gate_b.receiver,
            );
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
        let observed_a: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
                started_a.receiver.receive(),
            )
            .await;
        let observed_b: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
                started_b.receiver.receive(),
            )
            .await;
        scope.cancel();
        let outcome: TerraneTaskOutcome<()> = __terrane_await(scope.join(child)).await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&observed_a.available),
            terrane_scalar_support::scalar_text(&observed_b.available),
            terrane_scalar_support::scalar_text(&outcome.cancelled)
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
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        value.construct(did_fail, exceeded_deadline, detail);
        value
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
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            available: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        value.construct(did_fail, exceeded_deadline, has_value, detail, result_value);
        value
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_mutex(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn load(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_load(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_store(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_add(
            &self.handle,
            amount,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn read(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_read(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_write(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
#[derive(Clone)]
pub struct MemoryOrder {
    pub name: String,
}
impl MemoryOrder {
    pub fn terrane_construct(ordering_name: String) -> Self {
        let mut value = Self {
            name: String::from("sequentially-consistent"),
        };
        value.construct(ordering_name);
        value
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: i64) {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn load(&self, ordering: MemoryOrder) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(
        &self,
        value: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(
        &mut self,
        amount: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn get(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_get(
            &self.handle,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
