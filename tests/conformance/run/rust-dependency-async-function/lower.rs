// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, async_dependency.rs, channels.rs, tasks_native_local.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
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
#[allow(dead_code, reason = "a projected dependency may expose no Result members")]
const TERRANE_DEPENDENCY_ERROR: DescriptorId = DescriptorId(0);
#[allow(dead_code, reason = "panic catching may be disabled or not crossed")]
const TERRANE_DEPENDENCY_PANIC: DescriptorId = DescriptorId(1);
#[allow(
    dead_code,
    reason = "projected type methods may be imported without being crossed"
)]
fn __terrane_dependency_panic(
    payload: Box<dyn std::any::Any + Send>,
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
    pub static DESCRIPTORS: [&str; 2] = ["dependency-error", "dependency-panic"];
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
    pub static FILES: [&str; 1] = ["src/main.trn"];
    pub static FUNCTIONS: [&str; 1] = ["/app::main"];
    pub static SITES: [Site; 13] = [
        /* terrane-site-row: site 0: /app::main (src/main.trn:11:25-11:67) */
        { Site { function: 0, file: 0, line: 11, column: 25, end_line: 11, end_column: 67 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:13:21-13:38) */
        { Site { function: 0, file: 0, line: 13, column: 21, end_line: 13, end_column: 38 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:15:26-15:44) */
        { Site { function: 0, file: 0, line: 15, column: 26, end_line: 15, end_column: 44 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:24:29-24:50) */
        { Site { function: 0, file: 0, line: 24, column: 29, end_line: 24, end_column: 50 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:31:34-31:71) */
        { Site { function: 0, file: 0, line: 31, column: 34, end_line: 31, end_column: 71 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:36:28-36:46) */
        { Site { function: 0, file: 0, line: 36, column: 28, end_line: 36, end_column: 46 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:41:24-41:56) */
        { Site { function: 0, file: 0, line: 41, column: 24, end_line: 41, end_column: 56 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:43:25-43:58) */
        { Site { function: 0, file: 0, line: 43, column: 25, end_line: 43, end_column: 58 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:45:3-45:25) */
        { Site { function: 0, file: 0, line: 45, column: 3, end_line: 45, end_column: 25 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:47:16-47:29) */
        { Site { function: 0, file: 0, line: 47, column: 16, end_line: 47, end_column: 29 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:49:38-49:55) */
        { Site { function: 0, file: 0, line: 49, column: 38, end_line: 49, end_column: 55 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:51:24-51:53) */
        { Site { function: 0, file: 0, line: 51, column: 24, end_line: 51, end_column: 53 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:52:15-52:36) */
        { Site { function: 0, file: 0, line: 52, column: 15, end_line: 52, end_column: 36 } },
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
// Source: src/main.trn
// Namespace: app
async fn source_ready() -> String {
    return String::from("source-ready");
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_1085 = 0usize;
        let mut __terrane_select_cursor_1258 = 0usize;
        let mut __terrane_select_cursor_1396 = 0usize;
        let mut __terrane_select_cursor_1583 = 0usize;
        let echoed: String = __terrane_traced(
            __terrane_await({
                    let __terrane_future = echo_after_yield(
                        String::from("projected async success"),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            0 /* terrane-site: src/main.trn:11:25-11:67 */,
                        )
                    }
                })
                .await,
            0 /* terrane-site: src/main.trn:11:25-11:67 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&echoed));
        let polls: terrane_int_support::Int = __terrane_traced(
            __terrane_await({
                    let __terrane_future = timer_poll_count();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            1 /* terrane-site: src/main.trn:13:21-13:38 */,
                        )
                    }
                })
                .await,
            1 /* terrane-site: src/main.trn:13:21-13:38 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&polls));
        let message: String = __terrane_traced(
            __terrane_await({
                    let __terrane_future = socket_round_trip();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            2 /* terrane-site: src/main.trn:15:26-15:44 */,
                        )
                    }
                })
                .await,
            2 /* terrane-site: src/main.trn:15:26-15:44 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&message));
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let waiting: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _>>,
                        > { Box::pin(wait_for_sibling()) })(),
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                    Some(Err(error)) => {
                        TerraneTaskResult::Failed(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        )
                    }
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        let signalling: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _>>,
                        > { Box::pin(signal_sibling()) })(),
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(Ok(value)) => TerraneTaskResult::Completed(value),
                    Some(Err(error)) => {
                        TerraneTaskResult::Failed(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        )
                    }
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        let waited: TerraneTaskOutcome<String> = __terrane_await(scope.join(waiting))
            .await;
        let signalled: TerraneTaskOutcome<String> = __terrane_await(
                scope.join(signalling),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&waited.completed),
            terrane_scalar_support::scalar_text(&signalled.completed)
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let rejected: String = __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future =
                    checked_echo(String::from("reject")); async move {
                    __terrane_raised_err(__terrane_future. await, 3 /* terrane-site: src/main.trn:24:29-24:50 */) } }). await, 3 /* terrane-site: src/main.trn:24:29-24:50 */
                );
                println!("{}", terrane_scalar_support::scalar_text(&rejected));
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
                    if !__terrane_handled_0
                        && __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("caught projected async failure"))
                        );
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let selection_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let selection_rx: TerraneChannelReceiver<terrane_int_support::Int> = selection_pair
            .receiver;
        {
            let mut __terrane_select_guard_1085 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1085: Option<TerraneError> = None;
            let __terrane_select_control_1085_0 = __terrane_select_control();
            let mut __terrane_select_future_1085_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1085_0.clone(), { let
                __terrane_future = echo_after_yield(String::from("selected-projected"));
                async move { __terrane_raised_err(__terrane_future. await,
                4 /* terrane-site: src/main.trn:31:34-31:71 */) } })
            );
            let mut __terrane_select_result_1085_0 = None;
            let __terrane_select_control_1085_1 = __terrane_select_control();
            let mut __terrane_select_future_1085_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1085_1.clone(),
                Box::pin(selection_rx.receive()))
            );
            let mut __terrane_select_result_1085_1 = None;
            let __terrane_select_winner_1085 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1085
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1085_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1085_0 = Some(
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
                                    __terrane_select_future_1085_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1085_1 = Some(
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
                            _ => {
                                unreachable!("select candidate is within the case count")
                            }
                        }
                    }
                    std::task::Poll::Pending
                })
                .await;
            if __terrane_select_winner_1085 == usize::MAX {
                __terrane_select_control_1085_1.request_cancel();
                __terrane_select_control_1085_0.request_cancel();
                let _ = __terrane_select_future_1085_1.as_mut().await;
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1085_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1085 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            4 /* terrane-site: src/main.trn:31:34-31:71 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1085.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1085
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1085).await;
            }
            __terrane_select_cursor_1085 = (__terrane_select_winner_1085 + 1usize)
                % 2usize;
            match __terrane_select_winner_1085 {
                0 => {
                    __terrane_select_control_1085_1.request_cancel();
                    let _ = __terrane_select_future_1085_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_1085_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1085_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1085 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                4 /* terrane-site: src/main.trn:31:34-31:71 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1085.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1085
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1085 {
                0 => {
                    let selected: String = __terrane_traced(
                        __terrane_select_result_1085_0
                            .take()
                            .expect("selected case owns its ready result"),
                        4 /* terrane-site: src/main.trn:31:34-31:71 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&selected));
                }
                1 => {
                    let _ = __terrane_select_result_1085_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("unexpected receive"))
                    );
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        {
            let mut __terrane_select_guard_1258 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1258: Option<TerraneError> = None;
            let __terrane_select_control_1258_0 = __terrane_select_control();
            let mut __terrane_select_future_1258_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1258_0.clone(), { let
                __terrane_future = socket_round_trip(); async move {
                __terrane_raised_err(__terrane_future. await, 5 /* terrane-site: src/main.trn:36:28-36:46 */) } })
            );
            let mut __terrane_select_result_1258_0 = None;
            let __terrane_select_control_1258_1 = __terrane_select_control();
            let mut __terrane_select_future_1258_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1258_1.clone(),
                source_ready())
            );
            let mut __terrane_select_result_1258_1 = None;
            let __terrane_select_winner_1258 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1258
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1258_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1258_0 = Some(
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
                                    __terrane_select_future_1258_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1258_1 = Some(
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
                            _ => {
                                unreachable!("select candidate is within the case count")
                            }
                        }
                    }
                    std::task::Poll::Pending
                })
                .await;
            if __terrane_select_winner_1258 == usize::MAX {
                __terrane_select_control_1258_1.request_cancel();
                __terrane_select_control_1258_0.request_cancel();
                let _ = __terrane_select_future_1258_1.as_mut().await;
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1258_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1258 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            5 /* terrane-site: src/main.trn:36:28-36:46 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1258.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1258
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1258).await;
            }
            __terrane_select_cursor_1258 = (__terrane_select_winner_1258 + 1usize)
                % 2usize;
            match __terrane_select_winner_1258 {
                0 => {
                    __terrane_select_control_1258_1.request_cancel();
                    let _ = __terrane_select_future_1258_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_1258_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1258_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1258 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                5 /* terrane-site: src/main.trn:36:28-36:46 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1258.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1258
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1258 {
                0 => {
                    let projected: String = __terrane_traced(
                        __terrane_select_result_1258_0
                            .take()
                            .expect("selected case owns its ready result"),
                        5 /* terrane-site: src/main.trn:36:28-36:46 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&projected));
                }
                1 => {
                    let source: String = __terrane_select_result_1258_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&source));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        {
            let mut __terrane_select_guard_1396 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1396: Option<TerraneError> = None;
            let __terrane_select_control_1396_0 = __terrane_select_control();
            let mut __terrane_select_future_1396_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1396_0.clone(), { let
                __terrane_future = echo_after_yield(String::from("pending-first")); async
                move { __terrane_raised_err(__terrane_future. await,
                6 /* terrane-site: src/main.trn:41:24-41:56 */) } })
            );
            let mut __terrane_select_result_1396_0 = None;
            let __terrane_select_control_1396_1 = __terrane_select_control();
            let mut __terrane_select_future_1396_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1396_1.clone(), { let
                __terrane_future = echo_after_yield(String::from("pending-second"));
                async move { __terrane_raised_err(__terrane_future. await,
                7 /* terrane-site: src/main.trn:43:25-43:58 */) } })
            );
            let mut __terrane_select_result_1396_1 = None;
            let __terrane_select_winner_1396 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1396
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1396_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1396_0 = Some(
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
                                    __terrane_select_future_1396_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1396_1 = Some(
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
                            _ => {
                                unreachable!("select candidate is within the case count")
                            }
                        }
                    }
                    std::task::Poll::Pending
                })
                .await;
            if __terrane_select_winner_1396 == usize::MAX {
                __terrane_select_control_1396_1.request_cancel();
                __terrane_select_control_1396_0.request_cancel();
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1396_1
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1396 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            7 /* terrane-site: src/main.trn:43:25-43:58 */,
                        ),
                    );
                }
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1396_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1396 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            6 /* terrane-site: src/main.trn:41:24-41:56 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1396.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1396
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1396).await;
            }
            __terrane_select_cursor_1396 = (__terrane_select_winner_1396 + 1usize)
                % 2usize;
            match __terrane_select_winner_1396 {
                0 => {
                    __terrane_select_control_1396_1.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1396_1
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1396 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                7 /* terrane-site: src/main.trn:43:25-43:58 */,
                            ),
                        );
                    }
                }
                1 => {
                    __terrane_select_control_1396_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1396_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1396 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                6 /* terrane-site: src/main.trn:41:24-41:56 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1396.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1396
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1396 {
                0 => {
                    let first: String = __terrane_traced(
                        __terrane_select_result_1396_0
                            .take()
                            .expect("selected case owns its ready result"),
                        6 /* terrane-site: src/main.trn:41:24-41:56 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&first));
                }
                1 => {
                    let second: String = __terrane_traced(
                        __terrane_select_result_1396_1
                            .take()
                            .expect("selected case owns its ready result"),
                        7 /* terrane-site: src/main.trn:43:25-43:58 */,
                    );
                    println!("{}", terrane_scalar_support::scalar_text(&second));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        __terrane_raised(
            reset_operation_state(),
            8 /* terrane-site: src/main.trn:45:3-45:25 */,
        );
        {
            let mut __terrane_select_guard_1583 = __terrane_finally_guard();
            let mut __terrane_select_cleanup_error_1583: Option<TerraneError> = None;
            let __terrane_select_control_1583_0 = __terrane_select_control();
            let mut __terrane_select_future_1583_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1583_0.clone(), { let
                __terrane_future = wait_forever(); async move {
                __terrane_raised_err(__terrane_future. await, 9 /* terrane-site: src/main.trn:47:16-47:29 */) } })
            );
            let mut __terrane_select_result_1583_0 = None;
            let __terrane_select_control_1583_1 = __terrane_select_control();
            let mut __terrane_select_future_1583_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1583_1.clone(), { let
                __terrane_future = timer_poll_count(); async move {
                __terrane_raised_err(__terrane_future. await, 10 /* terrane-site: src/main.trn:49:38-49:55 */) } })
            );
            let mut __terrane_select_result_1583_1 = None;
            let __terrane_select_winner_1583 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1583
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1583_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1583_0 = Some(
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
                                    __terrane_select_future_1583_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1583_1 = Some(
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
                            _ => {
                                unreachable!("select candidate is within the case count")
                            }
                        }
                    }
                    std::task::Poll::Pending
                })
                .await;
            if __terrane_select_winner_1583 == usize::MAX {
                __terrane_select_control_1583_1.request_cancel();
                __terrane_select_control_1583_0.request_cancel();
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1583_1
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1583 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            10 /* terrane-site: src/main.trn:49:38-49:55 */,
                        ),
                    );
                }
                if let Some(Err(__terrane_select_error)) = __terrane_select_future_1583_0
                    .as_mut()
                    .await
                {
                    __terrane_select_cleanup_error_1583 = Some(
                        __terrane_trace_error(
                            __terrane_select_error,
                            9 /* terrane-site: src/main.trn:47:16-47:29 */,
                        ),
                    );
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1583.finish();
                if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1583
                    .take()
                {
                    __terrane_uncaught(__terrane_select_cleanup_error);
                }
                __terrane_finish_cancelled_select(__terrane_select_guard_1583).await;
            }
            __terrane_select_cursor_1583 = (__terrane_select_winner_1583 + 1usize)
                % 2usize;
            match __terrane_select_winner_1583 {
                0 => {
                    __terrane_select_control_1583_1.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1583_1
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1583 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                10 /* terrane-site: src/main.trn:49:38-49:55 */,
                            ),
                        );
                    }
                }
                1 => {
                    __terrane_select_control_1583_0.request_cancel();
                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1583_0
                        .as_mut()
                        .await
                    {
                        __terrane_select_cleanup_error_1583 = Some(
                            __terrane_trace_error(
                                __terrane_select_error,
                                9 /* terrane-site: src/main.trn:47:16-47:29 */,
                            ),
                        );
                    }
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1583.finish();
            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1583
                .take()
            {
                __terrane_uncaught(__terrane_select_cleanup_error);
            }
            match __terrane_select_winner_1583 {
                0 => {
                    let _ = __terrane_traced(
                        __terrane_select_result_1583_0
                            .take()
                            .expect("selected case owns its ready result"),
                        9 /* terrane-site: src/main.trn:47:16-47:29 */,
                    );
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("unexpected projected winner"))
                    );
                }
                1 => {
                    let selection_polls: terrane_int_support::Int = __terrane_traced(
                        __terrane_select_result_1583_1
                            .take()
                            .expect("selected case owns its ready result"),
                        10 /* terrane-site: src/main.trn:49:38-49:55 */,
                    );
                    println!(
                        "{}{}",
                        terrane_scalar_support::scalar_text(&String::from("selection-polls")),
                        terrane_scalar_support::scalar_text(&selection_polls)
                    );
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        let started: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = wait_until_operation_started();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            11 /* terrane-site: src/main.trn:51:24-51:53 */,
                        )
                    }
                })
                .await,
            11 /* terrane-site: src/main.trn:51:24-51:53 */,
        );
        let drops: terrane_int_support::Int = __terrane_raised(
            operation_drop_count(),
            12 /* terrane-site: src/main.trn:52:15-52:36 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&started),
            terrane_scalar_support::scalar_text(&drops)
        );
    });
}
// Source: <terrane>/projected/deps/async-witness.trn
// Namespace: deps/async-witness
pub async fn checked_echo(value: String) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match crate::__terrane_dependency_await_unwind(async_witness::checked_echo(value))
        .await
    {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `async-witness` member `async_witness::checked_echo` failed: {error}"
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
                    "async-witness",
                    "async_witness::checked_echo",
                ),
            )
        }
    }
}
pub async fn echo_after_yield(
    value: String,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match crate::__terrane_dependency_await_unwind(
            async_witness::echo_after_yield(value),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::echo_after_yield",
                ),
            )
        }
    }
}
pub fn operation_drop_count() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| async_witness::operation_drop_count()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::operation_drop_count",
                ),
            )
        }
    }
}
pub fn reset_operation_state() -> Result<(), crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| async_witness::reset_operation_state()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::reset_operation_state",
                ),
            )
        }
    }
}
pub async fn signal_sibling() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::signal_sibling()).await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::signal_sibling",
                ),
            )
        }
    }
}
pub async fn socket_round_trip() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::socket_round_trip())
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::socket_round_trip",
                ),
            )
        }
    }
}
pub async fn timer_poll_count() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match crate::__terrane_dependency_await_unwind(async_witness::timer_poll_count())
        .await
    {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::timer_poll_count",
                ),
            )
        }
    }
}
pub async fn wait_for_sibling() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::wait_for_sibling())
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::wait_for_sibling",
                ),
            )
        }
    }
}
pub async fn wait_forever() -> Result<String, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(async_witness::wait_forever()).await {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::wait_forever",
                ),
            )
        }
    }
}
pub async fn wait_until_operation_started() -> Result<bool, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            async_witness::wait_until_operation_started(),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "async-witness",
                    "async_witness::wait_until_operation_started",
                ),
            )
        }
    }
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
