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
    pub static FUNCTIONS: [&str; 5] = [
        "/select-error-propagation::fail",
        "/select-error-propagation::cleanup-coercion",
        "/select-error-propagation::cleanup-missing",
        "/select-error-propagation::construction-failure",
        "/select-error-propagation::main",
    ];
    pub static SITES: [Site; 13] = [
        /* terrane-site-row: site 0: /select-error-propagation::fail (case.trn:6:3-6:23) */
        { Site { function: 0, file: 0, line: 6, column: 3, end_line: 6, end_column: 23 } },
        /* terrane-site-row: site 1: /select-error-propagation::cleanup-coercion (case.trn:16:5-16:25) */
        { Site { function: 1, file: 0, line: 16, column: 5, end_line: 16, end_column: 25 } },
        /* terrane-site-row: site 2: /select-error-propagation::cleanup-missing (case.trn:24:5-24:22) */
        { Site { function: 2, file: 0, line: 24, column: 5, end_line: 24, end_column: 22 } },
        /* terrane-site-row: site 3: /select-error-propagation::construction-failure (case.trn:31:3-31:20) */
        { Site { function: 3, file: 0, line: 31, column: 3, end_line: 31, end_column: 20 } },
        /* terrane-site-row: site 4: /select-error-propagation::main (case.trn:50:31-50:36) */
        { Site { function: 4, file: 0, line: 50, column: 31, end_line: 50, end_column: 36 } },
        /* terrane-site-row: site 5: /select-error-propagation::main (case.trn:50:30-50:37) */
        { Site { function: 4, file: 0, line: 50, column: 30, end_line: 50, end_column: 37 } },
        /* terrane-site-row: site 6: /select-error-propagation::main (case.trn:59:19-59:59) */
        { Site { function: 4, file: 0, line: 59, column: 19, end_line: 59, end_column: 59 } },
        /* terrane-site-row: site 7: /select-error-propagation::main (case.trn:61:19-61:57) */
        { Site { function: 4, file: 0, line: 61, column: 19, end_line: 61, end_column: 57 } },
        /* terrane-site-row: site 8: /select-error-propagation::main (case.trn:61:18-61:58) */
        { Site { function: 4, file: 0, line: 61, column: 18, end_line: 61, end_column: 58 } },
        /* terrane-site-row: site 9: /select-error-propagation::main (case.trn:59:18-59:60) */
        { Site { function: 4, file: 0, line: 59, column: 18, end_line: 59, end_column: 60 } },
        /* terrane-site-row: site 10: /select-error-propagation::main (case.trn:71:19-71:63) */
        { Site { function: 4, file: 0, line: 71, column: 19, end_line: 71, end_column: 63 } },
        /* terrane-site-row: site 11: /select-error-propagation::main (case.trn:73:28-73:49) */
        { Site { function: 4, file: 0, line: 73, column: 28, end_line: 73, end_column: 49 } },
        /* terrane-site-row: site 12: /select-error-propagation::main (case.trn:71:18-71:64) */
        { Site { function: 4, file: 0, line: 71, column: 18, end_line: 71, end_column: 64 } },
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
use std::future::Future;
#[derive(Clone)]
struct TerraneCancellation {
    state: std::sync::Arc<TerraneCancellationState>,
}
struct TerraneCancellationState {
    cancelled: std::sync::atomic::AtomicBool,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
}
impl TerraneCancellation {
    fn new() -> Self {
        Self {
            state: std::sync::Arc::new(TerraneCancellationState {
                cancelled: std::sync::atomic::AtomicBool::new(false),
                wakers: std::sync::Mutex::new(Vec::new()),
            }),
        }
    }
    fn wake_waiters(&self) {
        let wakers = std::mem::take(
            &mut *self.state.wakers.lock().expect("cancellation waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }
    fn cancel(&self) {
        if !self.state.cancelled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.wake_waiters();
        }
    }
    fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(std::sync::atomic::Ordering::Acquire)
    }
    async fn cancelled(&self) {
        std::future::poll_fn(|context| {
                if self.is_cancelled() {
                    return std::task::Poll::Ready(());
                }
                let mut wakers = self
                    .state
                    .wakers
                    .lock()
                    .expect("cancellation waker lock poisoned");
                if self.is_cancelled() {
                    return std::task::Poll::Ready(());
                }
                if !wakers.iter().any(|waker| waker.will_wake(context.waker())) {
                    wakers.push(context.waker().clone());
                }
                std::task::Poll::Pending
            })
            .await
    }
}
struct TerraneFinalizerState {
    depth: std::sync::atomic::AtomicUsize,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
}
impl TerraneFinalizerState {
    fn new() -> Self {
        Self {
            depth: std::sync::atomic::AtomicUsize::new(0),
            wakers: std::sync::Mutex::new(Vec::new()),
        }
    }
    #[allow(
        dead_code,
        reason = "native scope support is shared by packages without asynchronous finally"
    )]
    fn register(self: &std::sync::Arc<Self>) -> TerraneFinallyGuard {
        let depth = self.depth.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1;
        TerraneFinallyGuard {
            state: Some(self.clone()),
            depth,
        }
    }
    #[allow(
        dead_code,
        reason = "native scope support is shared by packages without asynchronous finally"
    )]
    fn unregister(&self, depth: usize) {
        let current = self.depth.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        debug_assert_eq!(current, depth, "finally regions must leave innermost first");
        let wakers = std::mem::take(
            &mut *self.wakers.lock().expect("finalizer waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }
    #[allow(
        dead_code,
        reason = "projected cleanup tracking is shared by packages without projected entries"
    )]
    async fn reaches(&self, depth: usize) {
        std::future::poll_fn(|context| {
                if self.depth.load(std::sync::atomic::Ordering::Acquire) == depth {
                    return std::task::Poll::Ready(());
                }
                let mut wakers = self
                    .wakers
                    .lock()
                    .expect("finalizer waker lock poisoned");
                if self.depth.load(std::sync::atomic::Ordering::Acquire) == depth {
                    return std::task::Poll::Ready(());
                }
                if !wakers.iter().any(|waker| waker.will_wake(context.waker())) {
                    wakers.push(context.waker().clone());
                }
                std::task::Poll::Pending
            })
            .await
    }
    fn depth(&self) -> usize {
        self.depth.load(std::sync::atomic::Ordering::Acquire)
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneCancellationContext {
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
    finalizers: std::sync::Arc<TerraneFinalizerState>,
}
tokio::task_local! {
    static TERRANE_CANCELLATION_CONTEXT : TerraneCancellationContext;
}
static TERRANE_PROJECTED_CLEANUPS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
    0,
);
static TERRANE_PROJECTED_CLEANUPS_CHANGED: tokio::sync::Notify = tokio::sync::Notify::const_new();
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
struct TerraneProjectedEntryGuard {
    cancellation: TerraneCancellation,
    finalizers: std::sync::Arc<TerraneFinalizerState>,
    abort: Option<tokio::task::AbortHandle>,
    armed: bool,
}
impl Drop for TerraneProjectedEntryGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        TERRANE_PROJECTED_CLEANUPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.cancellation.cancel();
        let finalizers = self.finalizers.clone();
        let abort = self
            .abort
            .take()
            .expect("armed projected entry guard must own its abort handle");
        tokio::spawn(async move {
            finalizers.reaches(0).await;
            abort.abort();
            if TERRANE_PROJECTED_CLEANUPS
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1
            {
                TERRANE_PROJECTED_CLEANUPS_CHANGED.notify_waiters();
            }
        });
    }
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
async fn __terrane_projected_async_entry<F, T>(future: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let cancellation = TerraneCancellation::new();
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline: None,
        finalizers: finalizers.clone(),
    };
    let mut future = Box::pin(TERRANE_CANCELLATION_CONTEXT.scope(context, future));
    let first_poll = std::future::poll_fn(|cx| std::task::Poll::Ready(
            future.as_mut().poll(cx),
        ))
        .await;
    if let std::task::Poll::Ready(output) = first_poll {
        return output;
    }
    let task = tokio::spawn(future);
    let mut guard = TerraneProjectedEntryGuard {
        cancellation,
        finalizers,
        abort: Some(task.abort_handle()),
        armed: true,
    };
    let output = task
        .await
        .expect("projected asynchronous dependency entry task failed");
    guard.armed = false;
    output
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
async fn __terrane_wait_projected_cleanups() {
    loop {
        let changed = TERRANE_PROJECTED_CLEANUPS_CHANGED.notified();
        if TERRANE_PROJECTED_CLEANUPS.load(std::sync::atomic::Ordering::SeqCst) == 0 {
            return;
        }
        changed.await;
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneFinallyGuard {
    state: Option<std::sync::Arc<TerraneFinalizerState>>,
    depth: usize,
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
impl TerraneFinallyGuard {
    fn finish(&mut self) {
        if let Some(state) = self.state.take() {
            state.unregister(self.depth);
        }
    }
}
impl Drop for TerraneFinallyGuard {
    fn drop(&mut self) {
        self.finish();
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
#[derive(Clone)]
struct TerraneSelectControl {
    requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
    waker: std::sync::Arc<std::sync::Mutex<Option<std::task::Waker>>>,
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
impl TerraneSelectControl {
    fn request_cancel(&self) {
        self.requested.store(true, std::sync::atomic::Ordering::Release);
        if let Some(waker) = self
            .waker
            .lock()
            .expect("select cancellation waker lock poisoned")
            .take()
        {
            waker.wake();
        }
    }
    fn is_cancelled(&self) -> bool {
        self.requested.load(std::sync::atomic::Ordering::Acquire)
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
fn __terrane_select_control() -> TerraneSelectControl {
    TerraneSelectControl {
        requested: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: std::sync::Arc::new(std::sync::Mutex::new(None)),
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
async fn __terrane_select_operation<F: Future>(
    control: TerraneSelectControl,
    future: F,
) -> Option<F::Output> {
    let deadline = TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| context.deadline)
        .ok()
        .flatten();
    let cancellation = TerraneCancellation::new();
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline,
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async move {
                let mut future = std::pin::pin!(future);
                std::future::poll_fn(move |cx| {
                        if control.is_cancelled() {
                            cancellation.cancel();
                            let mut stored_waker = control
                                .waker
                                .lock()
                                .expect("select cancellation waker lock poisoned");
                            if stored_waker
                                .as_ref()
                                .is_none_or(|waker| !waker.will_wake(cx.waker()))
                            {
                                *stored_waker = Some(cx.waker().clone());
                            }
                            drop(stored_waker);
                            return match Future::poll(future.as_mut(), cx) {
                                std::task::Poll::Ready(output) => {
                                    std::task::Poll::Ready(Some(output))
                                }
                                std::task::Poll::Pending if finalizers.depth() == 0 => {
                                    std::task::Poll::Ready(None)
                                }
                                std::task::Poll::Pending => std::task::Poll::Pending,
                            };
                        }
                        Future::poll(future.as_mut(), cx).map(Some)
                    })
                    .await
            },
        )
        .await
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
fn __terrane_finally_guard() -> TerraneFinallyGuard {
    TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| context.finalizers.register())
        .unwrap_or(TerraneFinallyGuard {
            state: None,
            depth: 0,
        })
}
async fn __terrane_cancellation_requested(
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
) {
    if let Some(deadline) = deadline {
        tokio::select! {
            () = cancellation.cancelled() => {} () =
            tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)) => {
            cancellation.cancel(); }
        }
    } else {
        cancellation.cancelled().await;
    }
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
fn __terrane_cancellation_is_requested() -> bool {
    TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| {
            if context
                .deadline
                .is_some_and(|deadline| std::time::Instant::now() >= deadline)
            {
                context.cancellation.cancel();
            }
            context.cancellation.is_cancelled()
        })
        .unwrap_or(false)
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
async fn __terrane_cancel_operation<F: Future>(
    guard: &TerraneFinallyGuard,
    future: F,
) -> Option<F::Output> {
    let cancellation = TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| {
            guard
                .state
                .as_ref()
                .map(|state| {
                    (context.cancellation.clone(), context.deadline, state.clone())
                })
        })
        .ok()
        .flatten();
    if let Some((cancellation, deadline, finalizers)) = cancellation {
        let mut future = std::pin::pin!(future);
        tokio::select! {
            biased; output = future.as_mut() => Some(output), () =
            __terrane_cancellation_requested(cancellation, deadline) => {
            std::future::poll_fn(| cx | { match Future::poll(future.as_mut(), cx) {
            std::task::Poll::Ready(output) => { std::task::Poll::Ready(Some(output)) }
            std::task::Poll::Pending if finalizers.depth() == guard.depth => {
            std::task::Poll::Ready(None) } std::task::Poll::Pending =>
            std::task::Poll::Pending, } }). await },
        }
    } else {
        Some(future.await)
    }
}
#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
async fn __terrane_finish_cancelled_finally(mut guard: TerraneFinallyGuard) -> ! {
    guard.finish();
    std::future::pending().await
}
#[allow(
    dead_code,
    reason = "selection support is shared by native async packages without select statements"
)]
async fn __terrane_finish_cancelled_select(mut guard: TerraneFinallyGuard) -> ! {
    guard.finish();
    std::future::pending().await
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
async fn __terrane_cancellable<F: Future>(
    future: F,
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
) -> Option<F::Output> {
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline,
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async {
                let mut future = std::pin::pin!(future);
                tokio::select! {
                    biased; output = future.as_mut() => Some(output), () =
                    __terrane_cancellation_requested(cancellation, deadline) => {
                    std::future::poll_fn(| cx | { match Future::poll(future.as_mut(), cx)
                    { std::task::Poll::Ready(output) => {
                    std::task::Poll::Ready(Some(output)) } std::task::Poll::Pending if
                    finalizers.depth() == 0 => { std::task::Poll::Ready(None) }
                    std::task::Poll::Pending => std::task::Poll::Pending, } }). await },
                }
            },
        )
        .await
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerraneChannelOverflow {
    Block,
    FailSend,
    DropNewest,
    DropOldest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneChannelSendOutcome<T> {
    pub accepted: bool,
    pub closed: bool,
    pub dropped: bool,
    pub rejected_value: Option<T>,
    pub dropped_value: Option<T>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneChannelReceiveOutcome<T> {
    pub available: bool,
    pub closed: bool,
    pub value: Option<T>,
}
struct TerraneChannelState<T> {
    values: std::collections::VecDeque<T>,
    capacity: usize,
    overflow: TerraneChannelOverflow,
    sender_closed: bool,
    receiver_closed: bool,
    next_waiter: usize,
    sender_wakers: std::collections::BTreeMap<usize, std::task::Waker>,
    receiver_wakers: std::collections::BTreeMap<usize, std::task::Waker>,
    rendezvous_values: std::collections::BTreeMap<usize, T>,
    rendezvous_completed: std::collections::BTreeSet<usize>,
}
pub struct TerraneChannelSender<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
}
pub struct TerraneChannelReceiver<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
}
pub struct TerraneChannelPair<T> {
    pub sender: TerraneChannelSender<T>,
    pub receiver: TerraneChannelReceiver<T>,
}
impl<T> TerraneChannelPair<T> {
    pub fn new(capacity: usize, overflow: TerraneChannelOverflow) -> Self {
        let state = std::sync::Arc::new(
            std::sync::Mutex::new(TerraneChannelState {
                values: std::collections::VecDeque::with_capacity(capacity),
                capacity,
                overflow,
                sender_closed: false,
                receiver_closed: false,
                next_waiter: 0,
                sender_wakers: std::collections::BTreeMap::new(),
                receiver_wakers: std::collections::BTreeMap::new(),
                rendezvous_values: std::collections::BTreeMap::new(),
                rendezvous_completed: std::collections::BTreeSet::new(),
            }),
        );
        Self {
            sender: TerraneChannelSender {
                state: state.clone(),
            },
            receiver: TerraneChannelReceiver { state },
        }
    }
}
impl<T> TerraneChannelSender<T> {
    pub fn send(&self, value: T) -> TerraneChannelSend<T> {
        TerraneChannelSend {
            state: self.state.clone(),
            value: Some(value),
            waiter_id: None,
        }
    }
    pub fn close(self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.sender_closed = true;
        for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
            waker.wake();
        }
    }
}
impl<T> Drop for TerraneChannelSender<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.sender_closed = true;
        for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
            waker.wake();
        }
    }
}
pub struct TerraneChannelSend<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
    value: Option<T>,
    waiter_id: Option<usize>,
}
impl<T: Unpin> std::future::Future for TerraneChannelSend<T> {
    type Output = TerraneChannelSendOutcome<T>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let state_ref = self.state.clone();
        let mut state = state_ref.lock().expect("channel state lock poisoned");
        if let Some(waiter_id) = self.waiter_id
            && state.rendezvous_completed.remove(&waiter_id)
        {
            state.sender_wakers.remove(&waiter_id);
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: true,
                closed: false,
                dropped: false,
                rejected_value: None,
                dropped_value: None,
            });
        }
        if state.receiver_closed {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.sender_wakers.remove(&waiter_id);
                if self.value.is_none() {
                    self.value = state.rendezvous_values.remove(&waiter_id);
                }
                state.rendezvous_completed.remove(&waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: false,
                closed: true,
                dropped: false,
                rejected_value: self.value.take(),
                dropped_value: None,
            });
        }
        if state.capacity == 0 {
            let waiter_id = self
                .waiter_id
                .unwrap_or_else(|| {
                    let waiter_id = state.next_waiter;
                    state.next_waiter += 1;
                    self.waiter_id = Some(waiter_id);
                    waiter_id
                });
            if let Some(value) = self.value.take() {
                state.rendezvous_values.insert(waiter_id, value);
            }
            state.sender_wakers.insert(waiter_id, context.waker().clone());
            for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                waker.wake();
            }
            return std::task::Poll::Pending;
        }
        if state.values.len() < state.capacity {
            state
                .values
                .push_back(self.value.take().expect("send polled after completion"));
            if let Some(waiter_id) = self.waiter_id.take() {
                state.sender_wakers.remove(&waiter_id);
            }
            for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                waker.wake();
            }
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: true,
                closed: false,
                dropped: false,
                rejected_value: None,
                dropped_value: None,
            });
        }
        match state.overflow {
            TerraneChannelOverflow::Block => {
                let waiter_id = self
                    .waiter_id
                    .unwrap_or_else(|| {
                        let waiter_id = state.next_waiter;
                        state.next_waiter += 1;
                        self.waiter_id = Some(waiter_id);
                        waiter_id
                    });
                state.sender_wakers.insert(waiter_id, context.waker().clone());
                std::task::Poll::Pending
            }
            TerraneChannelOverflow::FailSend => {
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: false,
                    closed: false,
                    dropped: false,
                    rejected_value: self.value.take(),
                    dropped_value: None,
                })
            }
            TerraneChannelOverflow::DropNewest => {
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: false,
                    closed: false,
                    dropped: true,
                    rejected_value: None,
                    dropped_value: self.value.take(),
                })
            }
            TerraneChannelOverflow::DropOldest => {
                let dropped_value = state.values.pop_front();
                state
                    .values
                    .push_back(self.value.take().expect("send polled after completion"));
                for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                    waker.wake();
                }
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: true,
                    closed: false,
                    dropped: true,
                    rejected_value: None,
                    dropped_value,
                })
            }
        }
    }
}
impl<T> Drop for TerraneChannelSend<T> {
    fn drop(&mut self) {
        if let Some(waiter_id) = self.waiter_id {
            let mut state = self.state.lock().expect("channel state lock poisoned");
            state.sender_wakers.remove(&waiter_id);
            state.rendezvous_values.remove(&waiter_id);
            state.rendezvous_completed.remove(&waiter_id);
        }
    }
}
impl<T> TerraneChannelReceiver<T> {
    pub fn receive(&self) -> TerraneChannelReceive<T> {
        TerraneChannelReceive {
            state: self.state.clone(),
            waiter_id: None,
        }
    }
    pub fn close(self) -> terrane_collection_support::List<T> {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.receiver_closed = true;
        let remaining = terrane_collection_support::List::new(
            state.values.drain(..).collect(),
        );
        for (_, waker) in std::mem::take(&mut state.sender_wakers) {
            waker.wake();
        }
        remaining
    }
}
impl<T> Drop for TerraneChannelReceiver<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.receiver_closed = true;
        state.values.clear();
        for (_, waker) in std::mem::take(&mut state.sender_wakers) {
            waker.wake();
        }
    }
}
pub struct TerraneChannelReceive<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
    waiter_id: Option<usize>,
}
impl<T> std::future::Future for TerraneChannelReceive<T> {
    type Output = TerraneChannelReceiveOutcome<T>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let state_ref = self.state.clone();
        let mut state = state_ref.lock().expect("channel state lock poisoned");
        if state.capacity == 0
            && let Some(waiter_id) = state.rendezvous_values.keys().next().copied()
        {
            let value = state
                .rendezvous_values
                .remove(&waiter_id)
                .expect("rendezvous value disappeared");
            state.rendezvous_completed.insert(waiter_id);
            if let Some(waker) = state.sender_wakers.remove(&waiter_id) {
                waker.wake();
            }
            if let Some(receiver_waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&receiver_waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: true,
                closed: false,
                value: Some(value),
            });
        }
        if let Some(value) = state.values.pop_front() {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&waiter_id);
            }
            for (_, waker) in std::mem::take(&mut state.sender_wakers) {
                waker.wake();
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: true,
                closed: false,
                value: Some(value),
            });
        }
        if state.sender_closed {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: false,
                closed: true,
                value: None,
            });
        }
        let waiter_id = self
            .waiter_id
            .unwrap_or_else(|| {
                let waiter_id = state.next_waiter;
                state.next_waiter += 1;
                self.waiter_id = Some(waiter_id);
                waiter_id
            });
        state.receiver_wakers.insert(waiter_id, context.waker().clone());
        for waker in state.sender_wakers.values() {
            waker.wake_by_ref();
        }
        std::task::Poll::Pending
    }
}
impl<T> Unpin for TerraneChannelReceive<T> {}
impl<T> Drop for TerraneChannelReceive<T> {
    fn drop(&mut self) {
        if let Some(waiter_id) = self.waiter_id {
            self.state
                .lock()
                .expect("channel state lock poisoned")
                .receiver_wakers
                .remove(&waiter_id);
        }
    }
}
pub type TerranePlatformCapability = terrane_platform_support::Capability;
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
pub fn terrane_platform_i128(
    value: &terrane_int_support::Int,
    label: &str,
) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| TerranePlatformResult::error(
            format!("{label} is outside the signed 128-bit platform range"),
        ))
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
#[allow(dead_code)]
fn terrane_platform_int_mutex(
    initial: terrane_int_support::Int,
) -> TerranePlatformResult {
    let initial = terrane_platform_i128!(initial, "mutex initial value");
    terrane_platform_support::int_mutex(initial)
}
#[allow(dead_code)]
fn terrane_platform_int_mutex_load(
    value: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::int_mutex_load(value)
}
#[allow(dead_code)]
fn terrane_platform_int_mutex_store(
    value: &TerranePlatformCapability,
    replacement: terrane_int_support::Int,
) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "mutex value");
    terrane_platform_support::int_mutex_store(value, replacement)
}
#[allow(dead_code)]
fn terrane_platform_int_mutex_add(
    value: &TerranePlatformCapability,
    amount: terrane_int_support::Int,
) -> TerranePlatformResult {
    let amount = terrane_platform_i128!(amount, "mutex update");
    terrane_platform_support::int_mutex_add(value, amount)
}
#[allow(dead_code)]
fn terrane_platform_int_rw_lock(
    initial: terrane_int_support::Int,
) -> TerranePlatformResult {
    let initial = terrane_platform_i128!(initial, "read/write lock initial value");
    terrane_platform_support::int_rw_lock(initial)
}
#[allow(dead_code)]
fn terrane_platform_int_rw_lock_read(
    value: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::int_rw_lock_read(value)
}
#[allow(dead_code)]
fn terrane_platform_int_rw_lock_write(
    value: &TerranePlatformCapability,
    replacement: terrane_int_support::Int,
) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "read/write lock value");
    terrane_platform_support::int_rw_lock_write(value, replacement)
}
#[allow(dead_code)]
fn terrane_platform_atomic_int64(initial: i64) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64(i128::from(initial))
}
#[allow(dead_code)]
fn terrane_platform_atomic_int64_load(
    value: &TerranePlatformCapability,
    ordering: String,
) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_load(value, &ordering)
}
#[allow(dead_code)]
fn terrane_platform_atomic_int64_store(
    value: &TerranePlatformCapability,
    replacement: i64,
    ordering: String,
) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_store(
        value,
        i128::from(replacement),
        &ordering,
    )
}
#[allow(dead_code)]
fn terrane_platform_atomic_int64_add(
    value: &TerranePlatformCapability,
    amount: i64,
    ordering: String,
) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_add(value, i128::from(amount), &ordering)
}
#[allow(dead_code)]
fn terrane_platform_thread_local_int(
    initial: terrane_int_support::Int,
) -> TerranePlatformResult {
    let initial = terrane_platform_i128!(initial, "thread-local initial value");
    terrane_platform_support::thread_local_int(initial)
}
#[allow(dead_code)]
fn terrane_platform_thread_local_int_get(
    value: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::thread_local_int_get(value)
}
#[allow(dead_code)]
fn terrane_platform_thread_local_int_set(
    value: &TerranePlatformCapability,
    replacement: terrane_int_support::Int,
) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "thread-local value");
    terrane_platform_support::thread_local_int_set(value, replacement)
}
// Source: case.trn
// Namespace: select-error-propagation
async fn fail() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:6:3-6:23 */,
        ),
    );
}
async fn ready() -> String {
    return String::from("winner");
}
async fn cleanup_coercion(
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
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
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("cleanup-coercion"))
        );
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                1 /* terrane-site: case.trn:16:5-16:25 */,
            ),
        );
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
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn cleanup_missing(
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
    let mut __terrane_finally_guard_1 = __terrane_finally_guard();
    let __terrane_maybe_completion_1: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_1,
            async {
                let __terrane_try_1: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_try_1 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_1) => {
                        let mut __terrane_handled_1 = false;
                        if !__terrane_handled_1 {
                            return TerraneCompletion::Error(__terrane_error_1);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_1 = __terrane_maybe_completion_1.is_none();
    let mut __terrane_completion_1 = __terrane_maybe_completion_1
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_1: TerraneCompletion<()> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("cleanup-missing"))
        );
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::MissingKey,
                2 /* terrane-site: case.trn:24:5-24:22 */,
            ),
        );
    })();
    match __terrane_finally_1 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_1 = replacement,
    }
    if __terrane_cancelled_1
        && matches!(&__terrane_completion_1, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_1).await;
    }
    __terrane_finally_guard_1.finish();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn accept(value: terrane_int_support::Int) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn construction_failure() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::MissingKey,
            3 /* terrane-site: case.trn:31:3-31:20 */,
        ),
    );
}
fn late_construction() -> terrane_int_support::Int {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&String::from("unexpected late construction"))
    );
    return terrane_int_support::Int::from(1_i128);
}
async fn blocked(receiver: TerraneChannelReceiver<terrane_int_support::Int>) {
    let mut __terrane_finally_guard_2 = __terrane_finally_guard();
    let __terrane_maybe_completion_2: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_2,
            async {
                let __terrane_try_2: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_try_2 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_2) => {
                        let mut __terrane_handled_2 = false;
                        if !__terrane_handled_2 {
                            return TerraneCompletion::Error(__terrane_error_2);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_2 = __terrane_maybe_completion_2.is_none();
    let mut __terrane_completion_2 = __terrane_maybe_completion_2
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_2: TerraneCompletion<()> = (|| {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("cleanup-before-error"))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_2 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_2 = replacement,
    }
    if __terrane_cancelled_2
        && matches!(&__terrane_completion_2, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_2).await;
    }
    __terrane_finally_guard_2.finish();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_1164 = 0usize;
        let mut __terrane_select_cursor_1450 = 0usize;
        let mut __terrane_select_cursor_1828 = 0usize;
        let pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_3: TerraneCompletion<()> = async {
            let __terrane_try_3: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1164 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1164: Option<TerraneError> = None;
                    let __terrane_select_control_1164_0 = __terrane_select_control();
                    let mut __terrane_select_future_1164_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1164_0
                        .clone(), blocked(pair.receiver))
                    );
                    let mut __terrane_select_result_1164_0 = None;
                    let __terrane_select_control_1164_1 = __terrane_select_control();
                    let mut __terrane_select_future_1164_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1164_1
                        .clone(), fail())
                    );
                    let mut __terrane_select_result_1164_1 = None;
                    let __terrane_select_winner_1164 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..2usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1164
                                    + __terrane_select_offset) % 2usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1164_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1164_0 = Some(
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
                                            __terrane_select_future_1164_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1164_1 = Some(
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
                    if __terrane_select_winner_1164 == usize::MAX {
                        __terrane_select_control_1164_1.request_cancel();
                        __terrane_select_control_1164_0.request_cancel();
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1164_1
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1164 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    5 /* terrane-site: case.trn:50:30-50:37 */,
                                ),
                            );
                        }
                        let _ = __terrane_select_future_1164_0.as_mut().await;
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1164.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1164
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1164)
                            .await;
                    }
                    __terrane_select_cursor_1164 = (__terrane_select_winner_1164
                        + 1usize) % 2usize;
                    match __terrane_select_winner_1164 {
                        0 => {
                            __terrane_select_control_1164_1.request_cancel();
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1164_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1164 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        5 /* terrane-site: case.trn:50:30-50:37 */,
                                    ),
                                );
                            }
                        }
                        1 => {
                            __terrane_select_control_1164_0.request_cancel();
                            let _ = __terrane_select_future_1164_0.as_mut().await;
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1164.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1164
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1164 {
                        0 => {
                            let _ = __terrane_select_result_1164_0
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected"))
                            );
                        }
                        1 => {
                            let value: terrane_int_support::Int = __terrane_traced_completion!(
                                __terrane_select_result_1164_1.take()
                                .expect("selected case owns its ready result"),
                                5 /* terrane-site: case.trn:50:30-50:37 */
                            );
                            println!("{}", terrane_scalar_support::scalar_text(&value));
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_3 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_3) => {
                    let mut __terrane_handled_3 = false;
                    if !__terrane_handled_3
                        && __terrane_error_3.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_3 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("caught"))
                        );
                    }
                    if !__terrane_handled_3 {
                        return TerraneCompletion::Error(__terrane_error_3);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_3 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let coercion_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let missing_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_4: TerraneCompletion<()> = async {
            let __terrane_try_4: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1450 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1450: Option<TerraneError> = None;
                    let __terrane_select_control_1450_0 = __terrane_select_control();
                    let mut __terrane_select_future_1450_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_0
                        .clone(), cleanup_coercion(coercion_pair.receiver))
                    );
                    let mut __terrane_select_result_1450_0 = None;
                    let __terrane_select_control_1450_1 = __terrane_select_control();
                    let mut __terrane_select_future_1450_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_1
                        .clone(), cleanup_missing(missing_pair.receiver))
                    );
                    let mut __terrane_select_result_1450_1 = None;
                    let __terrane_select_control_1450_2 = __terrane_select_control();
                    let mut __terrane_select_future_1450_2 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_2
                        .clone(), ready())
                    );
                    let mut __terrane_select_result_1450_2 = None;
                    let __terrane_select_winner_1450 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..3usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1450
                                    + __terrane_select_offset) % 3usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1450_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_0 = Some(
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
                                            __terrane_select_future_1450_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_1 = Some(
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
                                    2 => {
                                        match Future::poll(
                                            __terrane_select_future_1450_2.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_2 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(2usize);
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
                    if __terrane_select_winner_1450 == usize::MAX {
                        __terrane_select_control_1450_2.request_cancel();
                        __terrane_select_control_1450_1.request_cancel();
                        __terrane_select_control_1450_0.request_cancel();
                        let _ = __terrane_select_future_1450_2.as_mut().await;
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1450 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    8 /* terrane-site: case.trn:61:18-61:58 */,
                                ),
                            );
                        }
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1450 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    9 /* terrane-site: case.trn:59:18-59:60 */,
                                ),
                            );
                        }
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1450.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1450
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1450)
                            .await;
                    }
                    __terrane_select_cursor_1450 = (__terrane_select_winner_1450
                        + 1usize) % 3usize;
                    match __terrane_select_winner_1450 {
                        0 => {
                            __terrane_select_control_1450_2.request_cancel();
                            __terrane_select_control_1450_1.request_cancel();
                            let _ = __terrane_select_future_1450_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        8 /* terrane-site: case.trn:61:18-61:58 */,
                                    ),
                                );
                            }
                        }
                        1 => {
                            __terrane_select_control_1450_2.request_cancel();
                            __terrane_select_control_1450_0.request_cancel();
                            let _ = __terrane_select_future_1450_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        9 /* terrane-site: case.trn:59:18-59:60 */,
                                    ),
                                );
                            }
                        }
                        2 => {
                            __terrane_select_control_1450_1.request_cancel();
                            __terrane_select_control_1450_0.request_cancel();
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        8 /* terrane-site: case.trn:61:18-61:58 */,
                                    ),
                                );
                            }
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        9 /* terrane-site: case.trn:59:18-59:60 */,
                                    ),
                                );
                            }
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1450.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1450
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1450 {
                        0 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1450_0.take()
                                .expect("selected case owns its ready result"),
                                9 /* terrane-site: case.trn:59:18-59:60 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected coercion"))
                            );
                        }
                        1 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1450_1.take()
                                .expect("selected case owns its ready result"),
                                8 /* terrane-site: case.trn:61:18-61:58 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected missing"))
                            );
                        }
                        2 => {
                            let winner: String = __terrane_select_result_1450_2
                                .take()
                                .expect("selected case owns its ready result");
                            println!("{}", terrane_scalar_support::scalar_text(&winner));
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_4 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_4) => {
                    let mut __terrane_handled_4 = false;
                    if !__terrane_handled_4
                        && __terrane_error_4.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_4 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("cleanup-error-replaced"))
                        );
                    }
                    if !__terrane_handled_4 {
                        return TerraneCompletion::Error(__terrane_error_4);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_4 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let construction_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_5: TerraneCompletion<()> = async {
            let __terrane_try_5: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1828 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1828: Option<TerraneError> = None;
                    let __terrane_select_control_1828_0 = __terrane_select_control();
                    let mut __terrane_select_future_1828_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_0
                        .clone(), cleanup_coercion(construction_pair.receiver))
                    );
                    let mut __terrane_select_result_1828_0 = None;
                    let __terrane_select_control_1828_1 = __terrane_select_control();
                    let __terrane_select_unpinned_1828_1 = match || -> Result<
                        _,
                        TerraneError,
                    > {
                        Ok(
                            accept(
                                __terrane_traced_err(
                                    construction_failure(),
                                    11 /* terrane-site: case.trn:73:28-73:49 */,
                                )?,
                            ),
                        )
                    }() {
                        __terrane_select_constructed => {
                            match __terrane_select_constructed {
                                Ok(__terrane_select_future) => __terrane_select_future,
                                Err(__terrane_select_error) => {
                                    __terrane_select_cleanup_error_1828 = Some(
                                        __terrane_select_error,
                                    );
                                    __terrane_select_control_1828_0.request_cancel();
                                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                        .as_mut()
                                        .await
                                    {
                                        __terrane_select_cleanup_error_1828 = Some(
                                            __terrane_trace_error(
                                                __terrane_select_error,
                                                12 /* terrane-site: case.trn:71:18-71:64 */,
                                            ),
                                        );
                                    }
                                    __terrane_wait_projected_cleanups().await;
                                    __terrane_select_guard_1828.finish();
                                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                                        .take()
                                    {
                                        return TerraneCompletion::Error(
                                            __terrane_select_cleanup_error,
                                        );
                                    }
                                    unreachable!("select construction failure propagates");
                                }
                            }
                        }
                    };
                    let mut __terrane_select_future_1828_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_1
                        .clone(), __terrane_select_unpinned_1828_1)
                    );
                    let mut __terrane_select_result_1828_1 = None;
                    let __terrane_select_control_1828_2 = __terrane_select_control();
                    let mut __terrane_select_future_1828_2 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_2
                        .clone(), accept(late_construction()))
                    );
                    let mut __terrane_select_result_1828_2 = None;
                    let __terrane_select_winner_1828 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..3usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1828
                                    + __terrane_select_offset) % 3usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1828_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_0 = Some(
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
                                            __terrane_select_future_1828_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_1 = Some(
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
                                    2 => {
                                        match Future::poll(
                                            __terrane_select_future_1828_2.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_2 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(2usize);
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
                    if __terrane_select_winner_1828 == usize::MAX {
                        __terrane_select_control_1828_2.request_cancel();
                        __terrane_select_control_1828_1.request_cancel();
                        __terrane_select_control_1828_0.request_cancel();
                        let _ = __terrane_select_future_1828_2.as_mut().await;
                        let _ = __terrane_select_future_1828_1.as_mut().await;
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1828 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    12 /* terrane-site: case.trn:71:18-71:64 */,
                                ),
                            );
                        }
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1828.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1828)
                            .await;
                    }
                    __terrane_select_cursor_1828 = (__terrane_select_winner_1828
                        + 1usize) % 3usize;
                    match __terrane_select_winner_1828 {
                        0 => {
                            __terrane_select_control_1828_2.request_cancel();
                            __terrane_select_control_1828_1.request_cancel();
                            let _ = __terrane_select_future_1828_2.as_mut().await;
                            let _ = __terrane_select_future_1828_1.as_mut().await;
                        }
                        1 => {
                            __terrane_select_control_1828_2.request_cancel();
                            __terrane_select_control_1828_0.request_cancel();
                            let _ = __terrane_select_future_1828_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1828 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        12 /* terrane-site: case.trn:71:18-71:64 */,
                                    ),
                                );
                            }
                        }
                        2 => {
                            __terrane_select_control_1828_1.request_cancel();
                            __terrane_select_control_1828_0.request_cancel();
                            let _ = __terrane_select_future_1828_1.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1828 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        12 /* terrane-site: case.trn:71:18-71:64 */,
                                    ),
                                );
                            }
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1828.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1828 {
                        0 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1828_0.take()
                                .expect("selected case owns its ready result"),
                                12 /* terrane-site: case.trn:71:18-71:64 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected cleanup winner"))
                            );
                        }
                        1 => {
                            let _ = __terrane_select_result_1828_1
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected construction winner"))
                            );
                        }
                        2 => {
                            let _ = __terrane_select_result_1828_2
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected late winner"))
                            );
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_5 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_5) => {
                    let mut __terrane_handled_5 = false;
                    if !__terrane_handled_5
                        && __terrane_error_5.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_5 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("construction-cleanup-replaced"))
                        );
                    }
                    if !__terrane_handled_5 {
                        return TerraneCompletion::Error(__terrane_error_5);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_5 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
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
