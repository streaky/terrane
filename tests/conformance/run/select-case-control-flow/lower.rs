// Generated deterministically by Terrane <version>.
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
    pub static FILES: [&str; 1] = ["case.trn"];
    pub static FUNCTIONS: [&str; 2] = [
        "/select-case-control-flow::selected-throw",
        "/select-case-control-flow::main",
    ];
    pub static SITES: [Site; 3] = [
        /* terrane-site-row: site 0: /select-case-control-flow::selected-throw (case.trn:20:7-20:27) */
        { Site { function: 0, file: 0, line: 20, column: 7, end_line: 20, end_column: 27 } },
        /* terrane-site-row: site 1: /select-case-control-flow::main (case.trn:35:23-35:38) */
        { Site { function: 1, file: 0, line: 35, column: 23, end_line: 35, end_column: 38 } },
        /* terrane-site-row: site 2: /select-case-control-flow::main (case.trn:35:22-35:39) */
        { Site { function: 1, file: 0, line: 35, column: 22, end_line: 35, end_column: 39 } },
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
async fn __terrane_await<F: Future>(future: F) -> F::Output {
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();
        fn poll(
            mut self: std::pin::Pin<&mut Self>,
            context: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            if self.0 {
                std::task::Poll::Ready(())
            } else {
                self.0 = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
    YieldOnce(false).await;
    let output = future.await;
    YieldOnce(false).await;
    output
}
#[allow(
    dead_code,
    clippy::unused_async,
    reason = "executor shutdown uses one hook for both simple and cancellation-aware runtimes"
)]
async fn __terrane_wait_projected_cleanups() {}
#[derive(Clone)]
struct TerraneSelectControl {
    requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
}
impl TerraneSelectControl {
    fn request_cancel(&self) {
        self.requested.store(true, std::sync::atomic::Ordering::Release);
    }
    fn is_cancelled(&self) -> bool {
        self.requested.load(std::sync::atomic::Ordering::Acquire)
    }
}
fn __terrane_select_control() -> TerraneSelectControl {
    TerraneSelectControl {
        requested: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
    }
}
async fn __terrane_select_operation<F: Future>(
    control: TerraneSelectControl,
    future: F,
) -> Option<F::Output> {
    let mut future = std::pin::pin!(future);
    std::future::poll_fn(move |cx| {
            if control.is_cancelled() {
                return std::task::Poll::Ready(None);
            }
            Future::poll(future.as_mut(), cx).map(Some)
        })
        .await
}
struct TerraneFinallyGuard;
impl TerraneFinallyGuard {
    fn finish(&mut self) {}
}
fn __terrane_finally_guard() -> TerraneFinallyGuard {
    TerraneFinallyGuard
}
fn __terrane_cancellation_is_requested() -> bool {
    false
}
async fn __terrane_finish_cancelled_select(_: TerraneFinallyGuard) -> ! {
    std::future::pending().await
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize")
        .block_on(async move {
            let output = future.await;
            __terrane_wait_projected_cleanups().await;
            output
        })
}
// Source: case.trn
// Namespace: select-case-control-flow
async fn one() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
async fn two() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
}
async fn selected_return() -> terrane_int_support::Int {
    let mut __terrane_select_cursor_197 = 0usize;
    {
        let mut __terrane_select_guard_197 = __terrane_finally_guard();
        let __terrane_select_control_197_0 = __terrane_select_control();
        let mut __terrane_select_future_197_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_197_0.clone(), one())
        );
        let mut __terrane_select_result_197_0 = None;
        let __terrane_select_control_197_1 = __terrane_select_control();
        let mut __terrane_select_future_197_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_197_1.clone(), two())
        );
        let mut __terrane_select_result_197_1 = None;
        let __terrane_select_winner_197 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_197
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_197_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_197_0 = Some(
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
                                __terrane_select_future_197_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_197_1 = Some(
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
        if __terrane_select_winner_197 == usize::MAX {
            __terrane_select_control_197_1.request_cancel();
            __terrane_select_control_197_0.request_cancel();
            let _ = __terrane_select_future_197_1.as_mut().await;
            let _ = __terrane_select_future_197_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_197.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_197).await;
        }
        __terrane_select_cursor_197 = (__terrane_select_winner_197 + 1usize) % 2usize;
        match __terrane_select_winner_197 {
            0 => {
                __terrane_select_control_197_1.request_cancel();
                let _ = __terrane_select_future_197_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_197_0.request_cancel();
                let _ = __terrane_select_future_197_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_197.finish();
        match __terrane_select_winner_197 {
            0 => {
                let value: terrane_int_support::Int = __terrane_select_result_197_0
                    .take()
                    .expect("selected case owns its ready result");
                return value.clone();
            }
            1 => {
                let value: terrane_int_support::Int = __terrane_select_result_197_1
                    .take()
                    .expect("selected case owns its ready result");
                return value.clone();
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
async fn selected_throw() -> Result<(), TerraneError> {
    let mut __terrane_select_cursor_353 = 0usize;
    {
        let mut __terrane_select_guard_353 = __terrane_finally_guard();
        let __terrane_select_control_353_0 = __terrane_select_control();
        let mut __terrane_select_future_353_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_353_0.clone(), one())
        );
        let mut __terrane_select_result_353_0 = None;
        let __terrane_select_control_353_1 = __terrane_select_control();
        let mut __terrane_select_future_353_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_353_1.clone(), two())
        );
        let mut __terrane_select_result_353_1 = None;
        let __terrane_select_winner_353 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_353
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_353_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_353_0 = Some(
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
                                __terrane_select_future_353_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_353_1 = Some(
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
        if __terrane_select_winner_353 == usize::MAX {
            __terrane_select_control_353_1.request_cancel();
            __terrane_select_control_353_0.request_cancel();
            let _ = __terrane_select_future_353_1.as_mut().await;
            let _ = __terrane_select_future_353_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_353.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_353).await;
        }
        __terrane_select_cursor_353 = (__terrane_select_winner_353 + 1usize) % 2usize;
        match __terrane_select_winner_353 {
            0 => {
                __terrane_select_control_353_1.request_cancel();
                let _ = __terrane_select_future_353_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_353_0.request_cancel();
                let _ = __terrane_select_future_353_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_353.finish();
        match __terrane_select_winner_353 {
            0 => {
                let _ = __terrane_select_result_353_0
                    .take()
                    .expect("selected case owns its ready result");
                return Err(
                    TerraneError::raised(
                        TerraneErrorKind::CoercionError,
                        0 /* terrane-site: case.trn:20:7-20:27 */,
                    ),
                );
            }
            1 => {
                let _ = __terrane_select_result_353_1
                    .take()
                    .expect("selected case owns its ready result");
                return Ok(());
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_508 = 0usize;
        let mut iteration: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        while iteration.clone() < terrane_int_support::Int::from(2_i128) {
            {
                let mut __terrane_select_guard_508 = __terrane_finally_guard();
                let __terrane_select_control_508_0 = __terrane_select_control();
                let mut __terrane_select_future_508_0 = std::pin::pin!(
                    __terrane_select_operation(__terrane_select_control_508_0.clone(),
                    one())
                );
                let mut __terrane_select_result_508_0 = None;
                let __terrane_select_control_508_1 = __terrane_select_control();
                let mut __terrane_select_future_508_1 = std::pin::pin!(
                    __terrane_select_operation(__terrane_select_control_508_1.clone(),
                    two())
                );
                let mut __terrane_select_result_508_1 = None;
                let __terrane_select_winner_508 = std::future::poll_fn(|
                        __terrane_select_context|
                    {
                        if __terrane_cancellation_is_requested() {
                            return std::task::Poll::Ready(usize::MAX);
                        }
                        for __terrane_select_offset in 0..2usize {
                            let __terrane_select_candidate = (__terrane_select_cursor_508
                                + __terrane_select_offset) % 2usize;
                            match __terrane_select_candidate {
                                0 => {
                                    match Future::poll(
                                        __terrane_select_future_508_0.as_mut(),
                                        __terrane_select_context,
                                    ) {
                                        std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                            __terrane_select_result_508_0 = Some(
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
                                        __terrane_select_future_508_1.as_mut(),
                                        __terrane_select_context,
                                    ) {
                                        std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                            __terrane_select_result_508_1 = Some(
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
                if __terrane_select_winner_508 == usize::MAX {
                    __terrane_select_control_508_1.request_cancel();
                    __terrane_select_control_508_0.request_cancel();
                    let _ = __terrane_select_future_508_1.as_mut().await;
                    let _ = __terrane_select_future_508_0.as_mut().await;
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_508.finish();
                    __terrane_finish_cancelled_select(__terrane_select_guard_508).await;
                }
                __terrane_select_cursor_508 = (__terrane_select_winner_508 + 1usize)
                    % 2usize;
                match __terrane_select_winner_508 {
                    0 => {
                        __terrane_select_control_508_1.request_cancel();
                        let _ = __terrane_select_future_508_1.as_mut().await;
                    }
                    1 => {
                        __terrane_select_control_508_0.request_cancel();
                        let _ = __terrane_select_future_508_0.as_mut().await;
                    }
                    _ => unreachable!("selected winner is within the case count"),
                }
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_508.finish();
                match __terrane_select_winner_508 {
                    0 => {
                        let _ = __terrane_select_result_508_0
                            .take()
                            .expect("selected case owns its ready result");
                        iteration = iteration.clone()
                            + terrane_int_support::Int::from(1_i128);
                        continue;
                    }
                    1 => {
                        let _ = __terrane_select_result_508_1
                            .take()
                            .expect("selected case owns its ready result");
                        break;
                    }
                    _ => unreachable!("selected winner is within the case count"),
                }
            }
        }
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&iteration),
            terrane_scalar_support::scalar_text(&__terrane_await(selected_return()).
            await)
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let finished: () = __terrane_traced_completion!(
                    __terrane_await(selected_throw()). await, 2 /* terrane-site: case.trn:35:22-35:39 */
                );
                if finished == () {
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("unexpected completion"))
                    );
                }
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
                        && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("caught"))
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
    });
}
