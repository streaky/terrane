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
    pub static FUNCTIONS: [&str; 6] = [
        "/app::drain-network",
        "/app::drain-tokio",
        "/app::drain-queue",
        "/app::wait-pending-tokio",
        "/app::wait-pending-queue",
        "/app::main",
    ];
    pub static SITES: [Site; 23] = [
        /* terrane-site-row: site 0: /app::drain-network (src/main.trn:7:17-7:31) */
        { Site { function: 0, file: 0, line: 7, column: 17, end_line: 7, end_column: 31 } },
        /* terrane-site-row: site 1: /app::drain-network (src/main.trn:11:18-11:32) */
        { Site { function: 0, file: 0, line: 11, column: 18, end_line: 11, end_column: 32 } },
        /* terrane-site-row: site 2: /app::drain-network (src/main.trn:15:17-15:31) */
        { Site { function: 0, file: 0, line: 15, column: 17, end_line: 15, end_column: 31 } },
        /* terrane-site-row: site 3: /app::drain-network (src/main.trn:17:16-17:31) */
        { Site { function: 0, file: 0, line: 17, column: 16, end_line: 17, end_column: 31 } },
        /* terrane-site-row: site 4: /app::drain-tokio (src/main.trn:20:17-20:31) */
        { Site { function: 1, file: 0, line: 20, column: 17, end_line: 20, end_column: 31 } },
        /* terrane-site-row: site 5: /app::drain-tokio (src/main.trn:24:18-24:32) */
        { Site { function: 1, file: 0, line: 24, column: 18, end_line: 24, end_column: 32 } },
        /* terrane-site-row: site 6: /app::drain-tokio (src/main.trn:28:17-28:31) */
        { Site { function: 1, file: 0, line: 28, column: 17, end_line: 28, end_column: 31 } },
        /* terrane-site-row: site 7: /app::drain-tokio (src/main.trn:30:16-30:31) */
        { Site { function: 1, file: 0, line: 30, column: 16, end_line: 30, end_column: 31 } },
        /* terrane-site-row: site 8: /app::drain-queue (src/main.trn:33:17-33:31) */
        { Site { function: 2, file: 0, line: 33, column: 17, end_line: 33, end_column: 31 } },
        /* terrane-site-row: site 9: /app::drain-queue (src/main.trn:37:18-37:32) */
        { Site { function: 2, file: 0, line: 37, column: 18, end_line: 37, end_column: 32 } },
        /* terrane-site-row: site 10: /app::drain-queue (src/main.trn:41:17-41:31) */
        { Site { function: 2, file: 0, line: 41, column: 17, end_line: 41, end_column: 31 } },
        /* terrane-site-row: site 11: /app::drain-queue (src/main.trn:43:10-43:25) */
        { Site { function: 2, file: 0, line: 43, column: 10, end_line: 43, end_column: 25 } },
        /* terrane-site-row: site 12: /app::wait-pending-tokio (src/main.trn:46:19-46:33) */
        { Site { function: 3, file: 0, line: 46, column: 19, end_line: 46, end_column: 33 } },
        /* terrane-site-row: site 13: /app::wait-pending-queue (src/main.trn:50:19-50:33) */
        { Site { function: 4, file: 0, line: 50, column: 19, end_line: 50, end_column: 33 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:54:49-54:67) */
        { Site { function: 5, file: 0, line: 54, column: 49, end_line: 54, end_column: 67 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:56:39-56:59) */
        { Site { function: 5, file: 0, line: 56, column: 39, end_line: 56, end_column: 59 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:58:39-58:59) */
        { Site { function: 5, file: 0, line: 58, column: 39, end_line: 58, end_column: 59 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:61:15-61:43) */
        { Site { function: 5, file: 0, line: 61, column: 15, end_line: 61, end_column: 43 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:62:25-62:38) */
        { Site { function: 5, file: 0, line: 62, column: 25, end_line: 62, end_column: 38 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:67:21-67:49) */
        { Site { function: 5, file: 0, line: 67, column: 21, end_line: 67, end_column: 49 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:68:31-68:50) */
        { Site { function: 5, file: 0, line: 68, column: 31, end_line: 68, end_column: 50 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:73:13-73:41) */
        { Site { function: 5, file: 0, line: 73, column: 13, end_line: 73, end_column: 41 } },
        /* terrane-site-row: site 22: /app::main (src/main.trn:79:19-79:47) */
        { Site { function: 5, file: 0, line: 79, column: 19, end_line: 79, end_column: 47 } },
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
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
struct TerraneProjectedEntryGuard {
    cancellation: TerraneCancellation,
    armed: bool,
}
impl Drop for TerraneProjectedEntryGuard {
    fn drop(&mut self) {
        if self.armed {
            self.cancellation.cancel();
        }
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
    let abort = task.abort_handle();
    let cleanup_cancellation = cancellation.clone();
    tokio::spawn(async move {
        cleanup_cancellation.cancelled().await;
        finalizers.reaches(0).await;
        abort.abort();
    });
    let mut guard = TerraneProjectedEntryGuard {
        cancellation,
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
        tokio::select! {
            biased; output = future => Some(output), () = async {
            __terrane_cancellation_requested(cancellation, deadline). await; finalizers
            .reaches(guard.depth). await; } => None,
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
                tokio::select! {
                    biased; output = future => Some(output), () = async {
                    __terrane_cancellation_requested(cancellation, deadline). await;
                    finalizers.reaches(0). await; } => None,
                }
            },
        )
        .await
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize");
    tokio::task::LocalSet::new().block_on(&runtime, future)
}
async fn __terrane_dependency_await_unwind<F: Future>(
    future: F,
) -> Result<F::Output, Box<dyn std::any::Any + Send>> {
    let mut future = std::pin::pin!(future);
    std::future::poll_fn(|context| {
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| { future.as_mut().poll(context) }),
            ) {
                Ok(std::task::Poll::Ready(value)) => std::task::Poll::Ready(Ok(value)),
                Ok(std::task::Poll::Pending) => std::task::Poll::Pending,
                Err(payload) => std::task::Poll::Ready(Err(payload)),
            }
        })
        .await
}
#[derive(Clone)]
pub struct TerraneTaskScope {
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
}
impl TerraneTaskScope {
    pub fn new(deadline_ms: Option<u64>) -> Self {
        Self {
            cancellation: TerraneCancellation::new(),
            deadline: deadline_ms
                .map(|milliseconds| {
                    std::time::Instant::now()
                        + std::time::Duration::from_millis(milliseconds)
                }),
        }
    }
    pub fn child_scope(&self, deadline_ms: u64) -> Self {
        let requested = std::time::Instant::now()
            + std::time::Duration::from_millis(deadline_ms);
        Self {
            cancellation: self.cancellation.clone(),
            deadline: Some(
                self.deadline.map_or(requested, |parent| parent.min(requested)),
            ),
        }
    }
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }
    pub fn should_cancel(&self) -> bool {
        self.cancellation.is_cancelled()
            || self
                .deadline
                .is_some_and(|deadline| std::time::Instant::now() >= deadline)
    }
    fn cancellation(&self) -> TerraneCancellation {
        self.cancellation.clone()
    }
    pub async fn join<T>(
        &self,
        mut task: TerraneScopedTask<T>,
    ) -> TerraneTaskOutcome<T> {
        let result = task
            .handle
            .take()
            .expect("scoped task joined once")
            .await
            .expect("scoped task must not panic outside its Terrane boundary");
        outcome_from_result(self, result)
    }
}
#[allow(
    dead_code,
    reason = "task result ABI is emitted before per-variant usage shaping"
)]
enum TerraneTaskResult<T> {
    Completed(T),
    Failed(TerraneError),
    Cancelled,
}
pub struct TerraneScopedTask<T> {
    handle: Option<tokio::task::JoinHandle<TerraneTaskResult<T>>>,
}
impl<T: 'static> TerraneScopedTask<T> {
    #[allow(dead_code, reason = "task spawn ABI is emitted before usage shaping")]
    fn spawn<F: Future<Output = TerraneTaskResult<T>> + 'static>(work: F) -> Self {
        Self {
            handle: Some(tokio::task::spawn_local(work)),
        }
    }
}
fn outcome_from_result<T>(
    scope: &TerraneTaskScope,
    result: TerraneTaskResult<T>,
) -> TerraneTaskOutcome<T> {
    match result {
        TerraneTaskResult::Completed(value) => {
            TerraneTaskOutcome {
                completed: true,
                cancelled: scope.should_cancel(),
                value: Some(value),
                error: None,
            }
        }
        TerraneTaskResult::Failed(error) => {
            scope.cancel();
            TerraneTaskOutcome {
                completed: false,
                cancelled: false,
                value: None,
                error: Some(error),
            }
        }
        TerraneTaskResult::Cancelled => {
            TerraneTaskOutcome {
                completed: false,
                cancelled: true,
                value: None,
                error: None,
            }
        }
    }
}
pub struct TerraneTaskOutcome<T> {
    pub completed: bool,
    pub cancelled: bool,
    pub value: Option<T>,
    pub error: Option<TerraneError>,
}
// Source: src/main.trn
// Namespace: app
async fn drain_network(mut sequence: TcpSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TcpSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:7:17-7:31 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:7:17-7:31 */,
    );
    let first_value: Option<String> = first.value;
    if first_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* first_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let second: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TcpSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        1 /* terrane-site: src/main.trn:11:18-11:32 */,
                    )
                }
            })
            .await,
        1 /* terrane-site: src/main.trn:11:18-11:32 */,
    );
    let second_value: Option<String> = second.value;
    if second_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* second_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let ended: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TcpSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        2 /* terrane-site: src/main.trn:15:17-15:31 */,
                    )
                }
            })
            .await,
        2 /* terrane-site: src/main.trn:15:17-15:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&ended.end));
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = sequence.close();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => Ok(value),
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TcpSequence::close` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TcpSequence::close",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        3 /* terrane-site: src/main.trn:17:16-17:31 */,
                    )
                }
            })
            .await,
        3 /* terrane-site: src/main.trn:17:16-17:31 */,
    );
}
async fn drain_tokio(mut sequence: TokioSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    > = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        4 /* terrane-site: src/main.trn:20:17-20:31 */,
                    )
                }
            })
            .await,
        4 /* terrane-site: src/main.trn:20:17-20:31 */,
    );
    let first_value: Option<terrane_int_support::Int> = first.value;
    if first_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* first_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let second: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    > = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        5 /* terrane-site: src/main.trn:24:18-24:32 */,
                    )
                }
            })
            .await,
        5 /* terrane-site: src/main.trn:24:18-24:32 */,
    );
    let second_value: Option<terrane_int_support::Int> = second.value;
    if second_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* second_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let ended: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    > = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        6 /* terrane-site: src/main.trn:28:17-28:31 */,
                    )
                }
            })
            .await,
        6 /* terrane-site: src/main.trn:28:17-28:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&ended.end));
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = sequence.close();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => Ok(value),
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::close` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::close",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        7 /* terrane-site: src/main.trn:30:16-30:31 */,
                    )
                }
            })
            .await,
        7 /* terrane-site: src/main.trn:30:16-30:31 */,
    );
}
async fn drain_queue(mut sequence: QueueSequence) -> bool {
    let first: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        8 /* terrane-site: src/main.trn:33:17-33:31 */,
                    )
                }
            })
            .await,
        8 /* terrane-site: src/main.trn:33:17-33:31 */,
    );
    let first_value: Option<String> = first.value;
    if first_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* first_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let second: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        9 /* terrane-site: src/main.trn:37:18-37:32 */,
                    )
                }
            })
            .await,
        9 /* terrane-site: src/main.trn:37:18-37:32 */,
    );
    let second_value: Option<String> = second.value;
    if second_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* second_value.as_ref()
            .expect("semantic optional narrowing"))
        );
    }
    let ended: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        10 /* terrane-site: src/main.trn:41:17-41:31 */,
                    )
                }
            })
            .await,
        10 /* terrane-site: src/main.trn:41:17-41:31 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&ended.end));
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| sequence.close()),
        ) {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) => {
                Err(
                    crate::TerraneForeignError(
                        crate::TerraneError::custom_raised(
                            crate::TERRANE_DEPENDENCY_ERROR,
                            format!(
                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::close` failed: {error}"
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
                        "terrane_sequence_witness",
                        "terrane_sequence_witness::QueueSequence::close",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:43:10-43:25 */,
    );
}
async fn wait_pending_tokio(mut sequence: TokioSequence) {
    let waiting: terrane_collection_support::AsyncIterationStep<
        terrane_int_support::Int,
    > = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(
                                                terrane_int_support::Int::from(i128::from(item)),
                                            )
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::TokioSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        12 /* terrane-site: src/main.trn:46:19-46:33 */,
                    )
                }
            })
            .await,
        12 /* terrane-site: src/main.trn:46:19-46:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&waiting.end));
}
async fn wait_pending_queue(mut sequence: QueueSequence) {
    let waiting: terrane_collection_support::AsyncIterationStep<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sequence).next();
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    match value {
                                        Some(item) => {
                                            terrane_collection_support::AsyncIterationStep::item(item)
                                        }
                                        None => {
                                            terrane_collection_support::AsyncIterationStep::end()
                                        }
                                    },
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"
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
                                        "terrane_sequence_witness",
                                        "terrane_sequence_witness::QueueSequence::next",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        13 /* terrane-site: src/main.trn:50:19-50:33 */,
                    )
                }
            })
            .await,
        13 /* terrane-site: src/main.trn:50:19-50:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&waiting.end));
}
fn main() {
    __terrane_run(async move {
        let network_drained: bool = __terrane_await(
                drain_network(
                    __terrane_traced(
                        __terrane_await({
                                let __terrane_future = make_tcp_sequence();
                                async move {
                                    __terrane_raised_err(
                                        __terrane_future.await,
                                        14 /* terrane-site: src/main.trn:54:49-54:67 */,
                                    )
                                }
                            })
                            .await,
                        14 /* terrane-site: src/main.trn:54:49-54:67 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&network_drained));
        let tokio_drained: bool = __terrane_await(
                drain_tokio(
                    __terrane_raised(
                        make_tokio_sequence(),
                        15 /* terrane-site: src/main.trn:56:39-56:59 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&tokio_drained));
        let queue_drained: bool = __terrane_await(
                drain_queue(
                    __terrane_raised(
                        make_queue_sequence(),
                        16 /* terrane-site: src/main.trn:58:39-58:59 */,
                    ),
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&queue_drained));
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let mut failing: TokioSequence = __terrane_raised_completion!(
                    make_failing_tokio_sequence(), 17 /* terrane-site: src/main.trn:61:15-61:43 */
                );
                let failed_step: terrane_collection_support::AsyncIterationStep<
                    terrane_int_support::Int,
                > = __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call = (&mut
                    failing).next(); async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(match value { Some(item) =>
                    terrane_collection_support::AsyncIterationStep::item(terrane_int_support::Int::from(i128::from(item))),
                    None => terrane_collection_support::AsyncIterationStep::end() }),
                    Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::TokioSequence::next` failed: {error}"),
                    crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload, "terrane_sequence_witness",
                    "terrane_sequence_witness::TokioSequence::next")) } } }; async move {
                    __terrane_raised_err(__terrane_future. await, 18 /* terrane-site: src/main.trn:62:25-62:38 */) } }). await, 18 /* terrane-site: src/main.trn:62:25-62:38 */
                );
                println!("{}", terrane_scalar_support::scalar_text(&failed_step.end));
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
                        let error = __terrane_error_0.clone();
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&error.message()
                            .to_owned())
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
        let __terrane_completion_1: TerraneCompletion<()> = async {
            let __terrane_try_1: TerraneCompletion<()> = async {
                let mut failing_queue: QueueSequence = __terrane_raised_completion!(
                    make_failing_queue_sequence(), 19 /* terrane-site: src/main.trn:67:21-67:49 */
                );
                let failed_queue_step: terrane_collection_support::AsyncIterationStep<
                    String,
                > = __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call = (&mut
                    failing_queue).next(); async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(match value { Some(item) =>
                    terrane_collection_support::AsyncIterationStep::item(item), None =>
                    terrane_collection_support::AsyncIterationStep::end() }),
                    Ok(Err(error)) => Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `terrane_sequence_witness` member `terrane_sequence_witness::QueueSequence::next` failed: {error}"),
                    crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload, "terrane_sequence_witness",
                    "terrane_sequence_witness::QueueSequence::next")) } } }; async move {
                    __terrane_raised_err(__terrane_future. await, 20 /* terrane-site: src/main.trn:68:31-68:50 */) } }). await, 20 /* terrane-site: src/main.trn:68:31-68:50 */
                );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&failed_queue_step.end)
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
                    if !__terrane_handled_1
                        && __terrane_error_1.kind
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_1 = true;
                        let error = __terrane_error_1.clone();
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&error.message()
                            .to_owned())
                        );
                    }
                    if !__terrane_handled_1 {
                        return TerraneCompletion::Error(__terrane_error_1);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let tokio_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let pending: TokioSequence = __terrane_raised(
            make_pending_tokio_sequence(),
            21 /* terrane-site: src/main.trn:73:13-73:41 */,
        );
        let tokio_child: TerraneScopedTask<()> = {
            let __terrane_scope = tokio_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = wait_pending_tokio(pending);
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
        tokio_scope.cancel();
        let tokio_outcome: TerraneTaskOutcome<()> = __terrane_await(
                tokio_scope.join(tokio_child),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&tokio_outcome.cancelled));
        let queue_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let pending_queue: QueueSequence = __terrane_raised(
            make_pending_queue_sequence(),
            22 /* terrane-site: src/main.trn:79:19-79:47 */,
        );
        let queue_child: TerraneScopedTask<()> = {
            let __terrane_scope = queue_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = wait_pending_queue(pending_queue);
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
        queue_scope.cancel();
        let queue_outcome: TerraneTaskOutcome<()> = __terrane_await(
                queue_scope.join(queue_child),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&queue_outcome.cancelled));
    });
}
// Source: <terrane>/projected/deps/terrane-sequence-witness.trn
// Namespace: deps/terrane-sequence-witness
pub use terrane_sequence_witness::QueueSequence;
pub use terrane_sequence_witness::TcpSequence;
pub use terrane_sequence_witness::TokioSequence;
pub fn make_failing_queue_sequence() -> Result<
    QueueSequence,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_failing_queue_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_failing_queue_sequence",
                ),
            )
        }
    }
}
pub fn make_failing_tokio_sequence() -> Result<
    TokioSequence,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_failing_tokio_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_failing_tokio_sequence",
                ),
            )
        }
    }
}
pub fn make_pending_queue_sequence() -> Result<
    QueueSequence,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_pending_queue_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_pending_queue_sequence",
                ),
            )
        }
    }
}
pub fn make_pending_tokio_sequence() -> Result<
    TokioSequence,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_pending_tokio_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_pending_tokio_sequence",
                ),
            )
        }
    }
}
pub fn make_queue_sequence() -> Result<QueueSequence, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_queue_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_queue_sequence",
                ),
            )
        }
    }
}
pub async fn make_tcp_sequence() -> Result<TcpSequence, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            terrane_sequence_witness::make_tcp_sequence(),
        )
        .await
    {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `terrane-sequence-witness` member `terrane_sequence_witness::make_tcp_sequence` failed: {error}"
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
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_tcp_sequence",
                ),
            )
        }
    }
}
pub fn make_tokio_sequence() -> Result<TokioSequence, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sequence_witness::make_tokio_sequence()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sequence-witness",
                    "terrane_sequence_witness::make_tokio_sequence",
                ),
            )
        }
    }
}
