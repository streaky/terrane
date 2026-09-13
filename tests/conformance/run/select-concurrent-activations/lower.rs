// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, tasks_native_parallel.rs
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
// Namespace: select-concurrent-activations
async fn a() -> String {
    return String::from("a");
}
async fn b() -> String {
    return String::from("b");
}
async fn x() -> String {
    return String::from("x");
}
async fn y() -> String {
    return String::from("y");
}
fn recursive_choose() -> std::sync::Arc<
    dyn Fn(
        terrane_int_support::Int,
    ) -> std::pin::Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync,
> {
    return std::sync::Arc::new(move |
        argument_0: terrane_int_support::Int,
    | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
        Box::pin(choose(argument_0))
    });
}
async fn choose(depth: terrane_int_support::Int) -> String {
    let mut __terrane_select_cursor_317 = 0usize;
    let mut __terrane_select_cursor_447 = 0usize;
    {
        let mut __terrane_select_guard_317 = __terrane_finally_guard();
        let __terrane_select_control_317_0 = __terrane_select_control();
        let mut __terrane_select_future_317_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_317_0.clone(), a())
        );
        let mut __terrane_select_result_317_0 = None;
        let __terrane_select_control_317_1 = __terrane_select_control();
        let mut __terrane_select_future_317_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_317_1.clone(), b())
        );
        let mut __terrane_select_result_317_1 = None;
        let __terrane_select_winner_317 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_317
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_317_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_317_0 = Some(
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
                                __terrane_select_future_317_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_317_1 = Some(
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
        if __terrane_select_winner_317 == usize::MAX {
            __terrane_select_control_317_1.request_cancel();
            __terrane_select_control_317_0.request_cancel();
            let _ = __terrane_select_future_317_1.as_mut().await;
            let _ = __terrane_select_future_317_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_317.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_317).await;
        }
        __terrane_select_cursor_317 = (__terrane_select_winner_317 + 1usize) % 2usize;
        match __terrane_select_winner_317 {
            0 => {
                __terrane_select_control_317_1.request_cancel();
                let _ = __terrane_select_future_317_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_317_0.request_cancel();
                let _ = __terrane_select_future_317_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_317.finish();
        match __terrane_select_winner_317 {
            0 => {
                let _ = __terrane_select_result_317_0
                    .take()
                    .expect("selected case owns its ready result");
                if depth.clone() > terrane_int_support::Int::from(0_i128) {
                    let recurse: std::sync::Arc<
                        dyn Fn(
                            terrane_int_support::Int,
                        ) -> std::pin::Pin<
                                Box<dyn Future<Output = String> + Send>,
                            > + Send + Sync,
                    > = recursive_choose();
                    return __terrane_await(
                            recurse(
                                depth.clone() - terrane_int_support::Int::from(1_i128),
                            ),
                        )
                        .await;
                }
                {
                    let mut __terrane_select_guard_447 = __terrane_finally_guard();
                    let __terrane_select_control_447_0 = __terrane_select_control();
                    let mut __terrane_select_future_447_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_447_0
                        .clone(), x())
                    );
                    let mut __terrane_select_result_447_0 = None;
                    let __terrane_select_control_447_1 = __terrane_select_control();
                    let mut __terrane_select_future_447_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_447_1
                        .clone(), y())
                    );
                    let mut __terrane_select_result_447_1 = None;
                    let __terrane_select_winner_447 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..2usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_447
                                    + __terrane_select_offset) % 2usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_447_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_447_0 = Some(
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
                                            __terrane_select_future_447_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_447_1 = Some(
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
                    if __terrane_select_winner_447 == usize::MAX {
                        __terrane_select_control_447_1.request_cancel();
                        __terrane_select_control_447_0.request_cancel();
                        let _ = __terrane_select_future_447_1.as_mut().await;
                        let _ = __terrane_select_future_447_0.as_mut().await;
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_447.finish();
                        __terrane_finish_cancelled_select(__terrane_select_guard_447)
                            .await;
                    }
                    __terrane_select_cursor_447 = (__terrane_select_winner_447 + 1usize)
                        % 2usize;
                    match __terrane_select_winner_447 {
                        0 => {
                            __terrane_select_control_447_1.request_cancel();
                            let _ = __terrane_select_future_447_1.as_mut().await;
                        }
                        1 => {
                            __terrane_select_control_447_0.request_cancel();
                            let _ = __terrane_select_future_447_0.as_mut().await;
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_447.finish();
                    match __terrane_select_winner_447 {
                        0 => {
                            let _ = __terrane_select_result_447_0
                                .take()
                                .expect("selected case owns its ready result");
                            return String::from("ax");
                        }
                        1 => {
                            let _ = __terrane_select_result_447_1
                                .take()
                                .expect("selected case owns its ready result");
                            return String::from("ay");
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
            }
            1 => {
                let _ = __terrane_select_result_317_1
                    .take()
                    .expect("selected case owns its ready result");
                return String::from("b");
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let first: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = choose(terrane_int_support::Int::from(1_i128));
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
        let second: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = choose(terrane_int_support::Int::from(1_i128));
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
        let first_outcome: TerraneTaskOutcome<String> = __terrane_await(
                scope.join(first),
            )
            .await;
        let second_outcome: TerraneTaskOutcome<String> = __terrane_await(
                scope.join(second),
            )
            .await;
        let first_value: Option<String> = first_outcome.value.clone();
        let second_value: Option<String> = second_outcome.value.clone();
        if first_value.is_some() {
            if second_value.is_some() {
                println!(
                    "{}{}", terrane_scalar_support::scalar_text(&* first_value.as_ref()
                    .expect("semantic optional narrowing")),
                    terrane_scalar_support::scalar_text(&* second_value.as_ref()
                    .expect("semantic optional narrowing"))
                );
            }
        }
    });
}
