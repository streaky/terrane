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
    pub static FILES: [&str; 2] = ["case.trn", "core/documents.trn"];
    pub static FUNCTIONS: [&str; 4] = [
        "/structured-logging::main",
        "/core/documents::make-document-list",
        "/core/documents::mapping-required-fields",
        "/core/documents::decode-document",
    ];
    pub static SITES: [Site; 27] = [
        /* terrane-site-row: site 0: /structured-logging::main (case.trn:37:8-37:34) */
        { Site { function: 0, file: 0, line: 37, column: 8, end_line: 37, end_column: 34 } },
        /* terrane-site-row: site 1: /structured-logging::main (case.trn:48:13-48:23) */
        { Site { function: 0, file: 0, line: 48, column: 13, end_line: 48, end_column: 23 } },
        /* terrane-site-row: site 2: /structured-logging::main (case.trn:48:56-48:66) */
        { Site { function: 0, file: 0, line: 48, column: 56, end_line: 48, end_column: 66 } },
        /* terrane-site-row: site 3: /structured-logging::main (case.trn:48:95-48:105) */
        { Site { function: 0, file: 0, line: 48, column: 95, end_line: 48, end_column: 105 } },
        /* terrane-site-row: site 4: /structured-logging::main (case.trn:49:13-49:23) */
        { Site { function: 0, file: 0, line: 49, column: 13, end_line: 49, end_column: 23 } },
        /* terrane-site-row: site 5: /structured-logging::main (case.trn:49:65-49:75) */
        { Site { function: 0, file: 0, line: 49, column: 65, end_line: 49, end_column: 75 } },
        /* terrane-site-row: site 6: /structured-logging::main (case.trn:49:111-49:121) */
        { Site { function: 0, file: 0, line: 49, column: 111, end_line: 49, end_column: 121 } },
        /* terrane-site-row: site 7: /structured-logging::main (case.trn:50:13-50:23) */
        { Site { function: 0, file: 0, line: 50, column: 13, end_line: 50, end_column: 23 } },
        /* terrane-site-row: site 8: /structured-logging::main (case.trn:50:49-50:59) */
        { Site { function: 0, file: 0, line: 50, column: 49, end_line: 50, end_column: 59 } },
        /* terrane-site-row: site 9: /structured-logging::main (case.trn:50:88-50:98) */
        { Site { function: 0, file: 0, line: 50, column: 88, end_line: 50, end_column: 98 } },
        /* terrane-site-row: site 10: /structured-logging::main (case.trn:51:13-51:23) */
        { Site { function: 0, file: 0, line: 51, column: 13, end_line: 51, end_column: 23 } },
        /* terrane-site-row: site 11: /structured-logging::main (case.trn:51:48-51:58) */
        { Site { function: 0, file: 0, line: 51, column: 48, end_line: 51, end_column: 58 } },
        /* terrane-site-row: site 12: /structured-logging::main (case.trn:61:9-61:29) */
        { Site { function: 0, file: 0, line: 61, column: 9, end_line: 61, end_column: 29 } },
        /* terrane-site-row: site 13: /structured-logging::main (case.trn:66:59-66:75) */
        { Site { function: 0, file: 0, line: 66, column: 59, end_line: 66, end_column: 75 } },
        /* terrane-site-row: site 14: /structured-logging::main (case.trn:72:51-72:62) */
        { Site { function: 0, file: 0, line: 72, column: 51, end_line: 72, end_column: 62 } },
        /* terrane-site-row: site 15: /structured-logging::main (case.trn:102:41-102:63) */
        { Site { function: 0, file: 0, line: 102, column: 41, end_line: 102, end_column: 63 } },
        /* terrane-site-row: site 16: /structured-logging::main (case.trn:102:107-102:129) */
        { Site { function: 0, file: 0, line: 102, column: 107, end_line: 102, end_column: 129 } },
        /* terrane-site-row: site 17: /structured-logging::main (case.trn:116:36-116:53) */
        { Site { function: 0, file: 0, line: 116, column: 36, end_line: 116, end_column: 53 } },
        /* terrane-site-row: site 18: /core/documents::make-document-list (core/documents.trn:140:47-140:60) */
        { Site { function: 1, file: 1, line: 140, column: 47, end_line: 140, end_column: 60 } },
        /* terrane-site-row: site 19: /core/documents::mapping-required-fields (core/documents.trn:153:17-153:30) */
        { Site { function: 2, file: 1, line: 153, column: 17, end_line: 153, end_column: 30 } },
        /* terrane-site-row: site 20: /core/documents::mapping-required-fields (core/documents.trn:157:16-157:47) */
        { Site { function: 2, file: 1, line: 157, column: 16, end_line: 157, end_column: 47 } },
        /* terrane-site-row: site 21: /core/documents::mapping-required-fields (core/documents.trn:163:16-163:45) */
        { Site { function: 2, file: 1, line: 163, column: 16, end_line: 163, end_column: 45 } },
        /* terrane-site-row: site 22: /core/documents::decode-document (core/documents.trn:176:12-176:44) */
        { Site { function: 3, file: 1, line: 176, column: 12, end_line: 176, end_column: 44 } },
        /* terrane-site-row: site 23: /core/documents::decode-document (core/documents.trn:177:37-177:69) */
        { Site { function: 3, file: 1, line: 177, column: 37, end_line: 177, end_column: 69 } },
        /* terrane-site-row: site 24: /core/documents::decode-document (core/documents.trn:183:12-183:49) */
        { Site { function: 3, file: 1, line: 183, column: 12, end_line: 183, end_column: 49 } },
        /* terrane-site-row: site 25: /core/documents::decode-document (core/documents.trn:184:36-184:73) */
        { Site { function: 3, file: 1, line: 184, column: 36, end_line: 184, end_column: 73 } },
        /* terrane-site-row: site 26: /core/documents::decode-document (core/documents.trn:185:36-185:73) */
        { Site { function: 3, file: 1, line: 185, column: 36, end_line: 185, end_column: 73 } },
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
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize")
        .block_on(future)
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
impl<T: Send + 'static> TerraneScopedTask<T> {
    #[allow(dead_code, reason = "task spawn ABI is emitted before usage shaping")]
    fn spawn<F: Future<Output = TerraneTaskResult<T>> + Send + 'static>(
        work: F,
    ) -> Self {
        Self {
            handle: Some(tokio::spawn(work)),
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
#[derive(Clone)]
struct TerraneThrowableLogValue {
    value: TerraneError,
}
impl LogValueProtocol for TerraneThrowableLogValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        make_document_string(self.value.render())
    }
}
fn terrane_log_error(value: TerraneError) -> LogValue {
    LogValue(Box::new(TerraneThrowableLogValue { value }))
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
// Source: case.trn
// Namespace: structured-logging
#[derive(Clone)]
pub struct RequestData {
    pub token: String,
}
impl RequestData {
    pub fn terrane_construct(token: String) -> Self {
        let mut value = Self { token: String::from("") };
        value.construct(token);
        value
    }
    pub fn construct(&mut self, token: String) {
        self.token = token;
    }
}
#[derive(Clone)]
pub struct CountedValue {
    pub value: String,
}
impl CountedValue {
    pub fn terrane_construct(input: String) -> Self {
        let mut value = Self { value: String::from("") };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: String) {
        self.value = input;
    }
    pub fn render(&self) -> DocumentValue {
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&String::from("rendered:")),
            terrane_scalar_support::scalar_text(&self.value)
        );
        return make_document_string(self.value.clone());
    }
}
impl LogValueProtocol for CountedValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        CountedValue::render(self)
    }
}
impl From<CountedValue> for LogValue {
    fn from(value: CountedValue) -> Self {
        Self(Box::new(value))
    }
}
fn main() {
    __terrane_run(async move {
        let sink_result: LogSinkResult = memory_sink(
            terrane_int_support::Int::from(4_i128),
            String::from("drop-oldest"),
            terrane_int_support::Int::from(1000_i128),
            terrane_int_support::Int::from(5_i128),
            false,
        );
        let mut writer: Logger = make_logger(
            sink_result.value,
            make_logger_options(
                info_level(),
                String::from("service."),
                String::from("service.worker"),
                terrane_int_support::Int::from(8_i128),
                terrane_int_support::Int::from(4096_i128),
            ),
        );
        writer = with_field(
            writer.clone(),
            field_at(
                String::from("request").clone(),
                log_text(String::from("r-1")),
                false,
                "case.trn:31:35".to_owned(),
            ),
        );
        writer = with_span(writer.clone(), String::from("serve"));
        let lazy: CountedValue = CountedValue::terrane_construct(
            String::from("rendered"),
        );
        let mut token_field: LogField = field_at(
            String::from("token").clone(),
            LogValue::from(CountedValue::terrane_construct(String::from("raw-secret"))),
            false,
            "case.trn:35:19".to_owned(),
        );
        let descriptor: TerraneDescriptor = TerraneDescriptor {
            identity: "/structured-logging::request-data",
            name: "request-data",
            kind: "class",
            inherently_identity_bearing: false,
            fields: &[
                TerraneFieldMetadata {
                    name: "token",
                    external_name: "token",
                    defaulted: true,
                    optional: false,
                    secret: true,
                },
            ],
        };
        if __terrane_raised(
            terrane_collection_support::List::new(
                    descriptor
                        .fields
                        .iter()
                        .map(|field| field.secret)
                        .collect::<Vec<bool>>(),
                )
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &terrane_int_support::Int::from(0_i128),
                        ),
                        0 /* terrane-site: case.trn:37:8-37:34 */,
                    ),
                ),
            0 /* terrane-site: case.trn:37:8-37:34 */,
        ) {
            token_field = field_at(
                String::from("token").clone(),
                LogValue::from(
                    CountedValue::terrane_construct(String::from("raw-secret")),
                ),
                true,
                "case.trn:38:23".to_owned(),
            );
        }
        let additions: terrane_collection_support::List<LogField> = terrane_collection_support::List::<
            LogField,
        >::new(
            vec![
                field_at(String::from("value").clone(), LogValue::from(lazy), false,
                "case.trn:39:24".to_owned()), token_field.clone()
            ],
        );
        let filtered: LogOutcome = emit_at(
            writer.clone(),
            debug_level().clone(),
            String::from("filtered").clone(),
            "case.trn:40:16".to_owned(),
            additions.clone(),
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&filtered.filtered),
            terrane_scalar_support::scalar_text(&filtered.failed)
        );
        let written: LogOutcome = emit_at(
            writer.clone(),
            info_level(),
            String::from("handled").clone(),
            "case.trn:43:15".to_owned(),
            additions.clone(),
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&written.filtered),
            terrane_scalar_support::scalar_text(&written.failed)
        );
        let records: terrane_collection_support::List<String> = drain_memory(
            writer.clone(),
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(records
            .length()))
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            1 /* terrane-site: case.trn:48:13-48:23 */)), 1 /* terrane-site: case.trn:48:13-48:23 */).contains(&String::from("\"timestamp\":1000"))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            2 /* terrane-site: case.trn:48:56-48:66 */)), 2 /* terrane-site: case.trn:48:56-48:66 */).contains(&String::from("\"sequence\":0"))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            3 /* terrane-site: case.trn:48:95-48:105 */)), 3 /* terrane-site: case.trn:48:95-48:105 */).contains(&String::from("\"severity\":\"info\"")))
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            4 /* terrane-site: case.trn:49:13-49:23 */)), 4 /* terrane-site: case.trn:49:13-49:23 */)
            .contains(&String::from("\"target\":\"service.worker\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            5 /* terrane-site: case.trn:49:65-49:75 */)), 5 /* terrane-site: case.trn:49:65-49:75 */).contains(&String::from("\"message\":\"handled\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            6 /* terrane-site: case.trn:49:111-49:121 */)), 6 /* terrane-site: case.trn:49:111-49:121 */).contains(&String::from("\"serve\"")))
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            7 /* terrane-site: case.trn:50:13-50:23 */)), 7 /* terrane-site: case.trn:50:13-50:23 */).contains(&String::from("\"request\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            8 /* terrane-site: case.trn:50:49-50:59 */)), 8 /* terrane-site: case.trn:50:49-50:59 */).contains(&String::from("\"<redacted>\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            9 /* terrane-site: case.trn:50:88-50:98 */)), 9 /* terrane-site: case.trn:50:88-50:98 */).contains(&String::from("raw-secret")))
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            10 /* terrane-site: case.trn:51:13-51:23 */)), 10 /* terrane-site: case.trn:51:13-51:23 */).contains(&String::from("case.trn"))),
            terrane_scalar_support::scalar_text(&__terrane_raised(records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            11 /* terrane-site: case.trn:51:48-51:58 */)), 11 /* terrane-site: case.trn:51:48-51:58 */).contains(&String::from("\"origin\":\"terrane\"")))
        );
        let other_sink: LogSinkResult = memory_sink(
            terrane_int_support::Int::from(2_i128),
            String::from("drop-oldest"),
            terrane_int_support::Int::from(2000_i128),
            terrane_int_support::Int::from(1_i128),
            false,
        );
        let other: Logger = make_logger(
            other_sink.value,
            make_logger_options(
                info_level(),
                String::from("service."),
                String::from("other.worker"),
                terrane_int_support::Int::from(8_i128),
                terrane_int_support::Int::from(4096_i128),
            ),
        );
        let target_filtered: LogOutcome = emit_at(
            other.clone(),
            info_level().clone(),
            String::from("not-written").clone(),
            "case.trn:55:23".to_owned(),
            additions.clone(),
        );
        println!("{}", terrane_scalar_support::scalar_text(&target_filtered.filtered));
        let empty_fields: terrane_collection_support::List<LogField> = terrane_collection_support::List::<
            LogField,
        >::new(vec![]);
        let __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                return TerraneCompletion::Error(
                    TerraneError::raised(
                        TerraneErrorKind::CoercionError,
                        12 /* terrane-site: case.trn:61:9-61:29 */,
                    ),
                );
            })();
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
                        let caught = __terrane_error_0.clone();
                        let error_fields: terrane_collection_support::List<LogField> = terrane_collection_support::List::<
                            LogField,
                        >::new(
                            vec![
                                field_at(String::from("failure").clone(),
                                terrane_log_error(caught.clone()), false, "case.trn:63:31"
                                .to_owned())
                            ],
                        );
                        let error_write: LogOutcome = emit_at(
                            writer.clone(),
                            info_level(),
                            String::from("failed-operation").clone(),
                            "case.trn:64:23".to_owned(),
                            error_fields.clone(),
                        );
                        let error_records: terrane_collection_support::List<String> = drain_memory(
                            writer.clone(),
                        );
                        println!(
                            "{}{}{}", terrane_scalar_support::scalar_text(&error_write
                            .failed),
                            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(error_records
                            .length())),
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(error_records
                            .get_or_error(__terrane_raised_completion!(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                            13 /* terrane-site: case.trn:66:59-66:75 */)),
                            13 /* terrane-site: case.trn:66:59-66:75 */)
                            .contains(&String::from("coercion-error")))
                        );
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let failed_sink: LogSinkResult = failing_sink();
        let failed_target: LogSink = failed_sink.value;
        let failed_logger: Logger = default_logger(failed_target.clone());
        let failed_write: LogOutcome = emit_at(
            failed_logger.clone(),
            info_level(),
            String::from("cannot-write").clone(),
            "case.trn:70:20".to_owned(),
            empty_fields.clone(),
        );
        let fallback: terrane_collection_support::List<String> = drain_fallback();
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&failed_write.failed),
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fallback
            .length())), terrane_scalar_support::scalar_text(&__terrane_raised(fallback
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            14 /* terrane-site: case.trn:72:51-72:62 */)), 14 /* terrane-site: case.trn:72:51-72:62 */).contains(&String::from("event rejected")))
        );
        let console_result: LogSinkResult = console_sink(false);
        let console: Logger = default_logger(console_result.value);
        let console_write: LogOutcome = emit_at(
            console.clone(),
            info_level(),
            String::from("visible").clone(),
            "case.trn:76:21".to_owned(),
            empty_fields.clone(),
        );
        println!("{}", terrane_scalar_support::scalar_text(&console_write.failed));
        let adapter_pair: TerraneChannelPair<LogEvent> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let adapter_producer: TerraneChannelSender<LogEvent> = adapter_pair.sender;
        let adapter_receiver: TerraneChannelReceiver<LogEvent> = adapter_pair.receiver;
        let adapter_sent: bool = __terrane_await(
                send_event(
                    adapter_producer,
                    make_event_at(
                        info_level().clone(),
                        String::from("adapter").clone(),
                        "case.trn:82:57".to_owned(),
                        empty_fields.clone(),
                    ),
                ),
            )
            .await;
        let adapter_received: TerraneChannelReceiveOutcome<LogEvent> = __terrane_await(
                Box::pin(adapter_receiver.receive()),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&adapter_sent),
            terrane_scalar_support::scalar_text(&adapter_received.available)
        );
        let async_sink: LogSinkResult = memory_sink(
            terrane_int_support::Int::from(4_i128),
            String::from("reject"),
            terrane_int_support::Int::from(3000_i128),
            terrane_int_support::Int::from(1_i128),
            false,
        );
        let async_target: LogSink = async_sink.value;
        let async_writer: Logger = default_logger(async_target.clone());
        let pair: TerraneChannelPair<LogEvent> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let producer: TerraneChannelSender<LogEvent> = pair.sender;
        let receiver: TerraneChannelReceiver<LogEvent> = pair.receiver;
        let consumer_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let consumer: TerraneScopedTask<bool> = {
            let __terrane_scope = consumer_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = consume_events(async_target.clone(), receiver);
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
        let first_sent: TerraneChannelSendOutcome<LogEvent> = __terrane_await(
                Box::pin(
                    producer
                        .send(
                            make_event_at(
                                info_level().clone(),
                                String::from("transported-one").clone(),
                                "case.trn:94:40".to_owned(),
                                empty_fields.clone(),
                            ),
                        ),
                ),
            )
            .await;
        let second_sent: TerraneChannelSendOutcome<LogEvent> = __terrane_await(
                Box::pin(
                    producer
                        .send(
                            make_event_at(
                                info_level().clone(),
                                String::from("transported-two").clone(),
                                "case.trn:95:41".to_owned(),
                                empty_fields.clone(),
                            ),
                        ),
                ),
            )
            .await;
        producer.close();
        let consumed: TerraneTaskOutcome<bool> = __terrane_await(
                consumer_scope.join(consumer),
            )
            .await;
        let consumed_value: Option<bool> = consumed.value.clone();
        if consumed_value.is_some() {
            println!(
                "{}{}{}{}", terrane_scalar_support::scalar_text(&first_sent.accepted),
                terrane_scalar_support::scalar_text(&second_sent.accepted),
                terrane_scalar_support::scalar_text(&consumed.completed),
                terrane_scalar_support::scalar_text(&* consumed_value.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        let transported_records: terrane_collection_support::List<String> = drain_memory(
            async_writer,
        );
        println!(
            "{}{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(transported_records
            .length())),
            terrane_scalar_support::scalar_text(&__terrane_raised(transported_records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            15 /* terrane-site: case.trn:102:41-102:63 */)), 15 /* terrane-site: case.trn:102:41-102:63 */)
            .contains(&String::from("\"message\":\"transported-one\""))),
            terrane_scalar_support::scalar_text(&__terrane_raised(transported_records
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
            16 /* terrane-site: case.trn:102:107-102:129 */)), 16 /* terrane-site: case.trn:102:107-102:129 */)
            .contains(&String::from("\"message\":\"transported-two\"")))
        );
        let failing_pair: TerraneChannelPair<LogEvent> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let failing_producer: TerraneChannelSender<LogEvent> = failing_pair.sender;
        let failing_receiver: TerraneChannelReceiver<LogEvent> = failing_pair.receiver;
        let failing_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let failing_consumer: TerraneScopedTask<bool> = {
            let __terrane_scope = failing_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = consume_events(
                failed_target.clone(),
                failing_receiver,
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
        let failing_sent: TerraneChannelSendOutcome<LogEvent> = __terrane_await(
                Box::pin(
                    failing_producer
                        .send(
                            make_event_at(
                                info_level().clone(),
                                String::from("transport-failure").clone(),
                                "case.trn:109:50".to_owned(),
                                empty_fields.clone(),
                            ),
                        ),
                ),
            )
            .await;
        failing_producer.close();
        let failing_consumed: TerraneTaskOutcome<bool> = __terrane_await(
                failing_scope.join(failing_consumer),
            )
            .await;
        let failing_consumed_value: Option<bool> = failing_consumed.value.clone();
        if failing_consumed_value.is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&failing_sent.accepted),
                terrane_scalar_support::scalar_text(&failing_consumed.completed),
                terrane_scalar_support::scalar_text(&* failing_consumed_value.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        let async_fallback: terrane_collection_support::List<String> = drain_fallback();
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(async_fallback
            .length())),
            terrane_scalar_support::scalar_text(&__terrane_raised(async_fallback
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            17 /* terrane-site: case.trn:116:36-116:53 */)), 17 /* terrane-site: case.trn:116:36-116:53 */).contains(&String::from("event rejected")))
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
    pub fn increase(&self, amount: terrane_int_support::Int) -> ConcurrencyIntResult {
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
    pub fn increase(&self, amount: i64, ordering: MemoryOrder) -> ConcurrencyIntResult {
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
// Source: core/documents.trn
// Namespace: core/documents
#[derive(Clone)]
pub struct DocumentInteger {
    pub text: String,
}
impl DocumentInteger {
    pub fn terrane_construct(text: String) -> Self {
        let mut value = Self { text: String::from("0") };
        value.construct(text);
        value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
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
        let mut value = Self {
            coefficient: String::from("0"),
            exponent: terrane_int_support::Int::from(0_i128),
            text: String::from("0"),
        };
        value.construct(coefficient, exponent, text);
        value
    }
    pub fn construct(
        &mut self,
        coefficient: String,
        exponent: terrane_int_support::Int,
        text: String,
    ) {
        self.coefficient = coefficient;
        self.exponent = exponent.clone();
        self.text = text;
    }
}
pub trait SerializableProtocol {
    fn clone_box(&self) -> Box<dyn SerializableProtocol>;
    fn separate_box(&self) -> Box<dyn SerializableProtocol>;
    fn to_document(&self) -> DocumentValue;
}
impl Clone for Box<dyn SerializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct Serializable(Box<dyn SerializableProtocol>);
impl Serializable {
    pub fn to_document(&self) -> DocumentValue {
        self.0.to_document()
    }
}
pub trait DeserializableProtocol {
    fn clone_box(&self) -> Box<dyn DeserializableProtocol>;
    fn separate_box(&self) -> Box<dyn DeserializableProtocol>;
    fn from_document(&self, value: DocumentValue) -> DocumentResult;
}
impl Clone for Box<dyn DeserializableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct Deserializable(Box<dyn DeserializableProtocol>);
impl Deserializable {
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        self.0.from_document(value)
    }
}
pub trait DocumentDecodableProtocol {
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
#[derive(Clone)]
pub struct DocumentDecodable(Box<dyn DocumentDecodableProtocol>);
impl DocumentDecodable {}
pub trait DocumentValidatableProtocol {
    fn clone_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn separate_box(&self) -> Box<dyn DocumentValidatableProtocol>;
    fn validate_document(&self) -> Option<String>;
}
impl Clone for Box<dyn DocumentValidatableProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct DocumentValidatable(Box<dyn DocumentValidatableProtocol>);
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
        let mut value = Self {
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
        value.construct(raw);
        value
    }
    pub fn construct(&mut self, raw: terrane_document_support::DataResult) {
        self.kind = terrane_document_kind(&raw);
        self.scalar = terrane_document_text(&raw);
        self.encoded = terrane_data_encoded(&raw);
        if self.kind == String::from("integer") {
            self.integer = DocumentInteger::terrane_construct(self.scalar.clone());
        }
        if self.kind == String::from("decimal") {
            self.decimal = DocumentDecimal::terrane_construct(
                terrane_document_coefficient(&raw),
                terrane_document_exponent(&raw),
                self.scalar.clone(),
            );
        }
        self.raw = raw;
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return terrane_document_length(&self.raw);
    }
    pub fn to_document(&self) -> DocumentValue {
        return self.clone();
    }
    pub fn item(&self, index: terrane_int_support::Int) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_item(
            &self.raw,
            index.clone(),
        );
        return make_document_result(raw);
    }
    pub fn key(&self, index: terrane_int_support::Int) -> String {
        return terrane_document_key(&self.raw, index.clone());
    }
    pub fn field(&self, name: String) -> DocumentResult {
        let raw: terrane_document_support::DataResult = terrane_document_field(
            &self.raw,
            name,
        );
        return make_document_result(raw);
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
        DocumentValue::to_document(self)
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
        let mut value = Self {
            failed: false,
            message: String::from(""),
            path: String::from("$"),
            expected: String::from(""),
            value: DocumentValue::terrane_construct(terrane_empty_document()),
        };
        value.construct(failed, message, path, expected, raw);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        message: String,
        path: String,
        expected: String,
        raw: terrane_document_support::DataResult,
    ) {
        self.failed = failed;
        self.message = message;
        self.path = path;
        self.expected = expected;
        self.value = DocumentValue::terrane_construct(raw);
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
        let mut value = Self {
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
        value.construct(descriptor_name, expected_kind, allow_unknown);
        value
    }
    pub fn construct(
        &mut self,
        descriptor_name: String,
        expected_kind: String,
        allow_unknown: bool,
    ) {
        self.descriptor_name = descriptor_name;
        self.expected_kind = expected_kind;
        self.allow_unknown = allow_unknown;
    }
    pub fn from_document(&self, value: DocumentValue) -> DocumentResult {
        return decode_document(value.clone(), self.clone());
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
        DocumentMapping::from_document(self, value)
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
    return destination.from_document(value.clone());
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
        let mut value = Self {
            raw: terrane_make_document_map(),
        };
        value.construct();
        value
    }
    pub fn construct(&mut self) {
        self.raw = terrane_make_document_map();
    }
    pub fn append(&mut self, key: String, value: DocumentValue) {
        self.raw = terrane_document_map_insert(&self.raw, key, &value.raw);
    }
}
pub fn append_document_map_entry(
    mut entries: DocumentMapEntries,
    key: String,
    value: DocumentValue,
) -> DocumentMapEntries {
    entries.append(key, value.clone());
    return entries.clone();
}
pub fn make_document_list(
    values: terrane_collection_support::List<DocumentValue>,
) -> DocumentResult {
    let mut raw: terrane_document_support::DataResult = terrane_make_document_list();
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(terrane_int_support::Int::from(values.length()))
    {
        raw = terrane_document_list_append(
            &raw,
            &__terrane_raised(
                    values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                18 /* terrane-site: core/documents.trn:140:47-140:60 */,
                            ),
                        ),
                    18 /* terrane-site: core/documents.trn:140:47-140:60 */,
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
    let fields: terrane_collection_support::List<String> = mapping.field_names;
    let optional_fields: terrane_collection_support::List<String> = mapping
        .optional_fields;
    let default_fields: terrane_collection_support::List<String> = mapping
        .default_fields;
    let mut required: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = required.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(fields.length()),
            )
        {
            let field: String = __terrane_raised(
                fields
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            19 /* terrane-site: core/documents.trn:153:17-153:30 */,
                        ),
                    ),
                19 /* terrane-site: core/documents.trn:153:17-153:30 */,
            );
            let mut optional: bool = false;
            let mut optional_index: terrane_int_support::Int = terrane_int_support::Int::from(
                0_i128,
            );
            while optional_index.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(optional_fields.length()),
                )
            {
                if __terrane_raised(
                    optional_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &optional_index.clone(),
                                ),
                                20 /* terrane-site: core/documents.trn:157:16-157:47 */,
                            ),
                        ),
                    20 /* terrane-site: core/documents.trn:157:16-157:47 */,
                ) == field
                {
                    optional = true;
                }
                optional_index = optional_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            let mut defaulted: bool = false;
            let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
                0_i128,
            );
            while default_index.clone()
                < terrane_int_support::Int::from(
                    terrane_int_support::Int::from(default_fields.length()),
                )
            {
                if __terrane_raised(
                    default_fields
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &default_index.clone(),
                                ),
                                21 /* terrane-site: core/documents.trn:163:16-163:45 */,
                            ),
                        ),
                    21 /* terrane-site: core/documents.trn:163:16-163:45 */,
                ) == field
                {
                    defaulted = true;
                }
                default_index = default_index.clone()
                    + terrane_int_support::Int::from(1_i128);
            }
            if field != String::from("") && !optional && !defaulted {
                __terrane_list_append_0.push(field);
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return required.clone();
}
pub fn decode_document(
    value: DocumentValue,
    mapping: DocumentMapping,
) -> DocumentResult {
    let required: terrane_collection_support::List<String> = mapping_required_fields(
        mapping.clone(),
    );
    let mut declared_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut field_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    {
        let __terrane_list_append_1 = declared_fields.make_unique();
        while field_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(mapping.field_names.length()),
            )
        {
            if __terrane_raised(
                mapping
                    .field_names
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &field_index.clone(),
                            ),
                            22 /* terrane-site: core/documents.trn:176:12-176:44 */,
                        ),
                    ),
                22 /* terrane-site: core/documents.trn:176:12-176:44 */,
            ) != String::from("")
            {
                __terrane_list_append_1
                    .push(
                        __terrane_raised(
                            mapping
                                .field_names
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(
                                            &field_index.clone(),
                                        ),
                                        23 /* terrane-site: core/documents.trn:177:37-177:69 */,
                                    ),
                                ),
                            23 /* terrane-site: core/documents.trn:177:37-177:69 */,
                        ),
                    );
            }
            field_index = field_index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    let mut default_fields: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_values: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![]);
    let mut default_index: terrane_int_support::Int = terrane_int_support::Int::from(
        0_i128,
    );
    while default_index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(mapping.default_fields.length()),
        )
        && default_index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(mapping.default_values.length()),
            )
    {
        if __terrane_raised(
            mapping
                .default_fields
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(
                            &default_index.clone(),
                        ),
                        24 /* terrane-site: core/documents.trn:183:12-183:49 */,
                    ),
                ),
            24 /* terrane-site: core/documents.trn:183:12-183:49 */,
        ) != String::from("")
        {
            default_fields
                .append(
                    __terrane_raised(
                        mapping
                            .default_fields
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    25 /* terrane-site: core/documents.trn:184:36-184:73 */,
                                ),
                            ),
                        25 /* terrane-site: core/documents.trn:184:36-184:73 */,
                    ),
                );
            default_values
                .append(
                    __terrane_raised(
                        mapping
                            .default_values
                            .get_or_error(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &default_index.clone(),
                                    ),
                                    26 /* terrane-site: core/documents.trn:185:36-185:73 */,
                                ),
                            ),
                        26 /* terrane-site: core/documents.trn:185:36-185:73 */,
                    ),
                );
        }
        default_index = default_index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let raw: terrane_document_support::DataResult = terrane_validate_mapping(
        &value.raw,
        mapping.expected_kind,
        required,
        declared_fields,
        default_fields,
        default_values,
        mapping.allow_unknown,
    );
    let mut result: DocumentResult = make_document_result(raw);
    if result.failed {
        result.expected = mapping.descriptor_name.clone();
    }
    return result.clone();
}
// Source: core/logging.trn
// Namespace: core/logging
#[derive(Clone)]
pub struct LogLevel {
    pub name: String,
    pub rank: terrane_int_support::Int,
}
impl LogLevel {
    pub fn terrane_construct(name: String, rank: terrane_int_support::Int) -> Self {
        let mut value = Self {
            name: String::from("info"),
            rank: terrane_int_support::Int::from(30_i128),
        };
        value.construct(name, rank);
        value
    }
    pub fn construct(&mut self, name: String, rank: terrane_int_support::Int) {
        self.name = name;
        self.rank = rank.clone();
    }
}
pub fn trace_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("trace"),
        terrane_int_support::Int::from(10_i128),
    );
}
pub fn debug_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("debug"),
        terrane_int_support::Int::from(20_i128),
    );
}
pub fn info_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("info"),
        terrane_int_support::Int::from(30_i128),
    );
}
pub fn warning_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("warning"),
        terrane_int_support::Int::from(40_i128),
    );
}
pub fn error_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("error"),
        terrane_int_support::Int::from(50_i128),
    );
}
pub fn critical_level() -> LogLevel {
    return LogLevel::terrane_construct(
        String::from("critical"),
        terrane_int_support::Int::from(60_i128),
    );
}
pub trait LogValueProtocol: Send + Sync {
    fn clone_box(&self) -> Box<dyn LogValueProtocol>;
    fn separate_box(&self) -> Box<dyn LogValueProtocol>;
    fn render(&self) -> DocumentValue;
}
impl Clone for Box<dyn LogValueProtocol> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
#[derive(Clone)]
pub struct LogValue(Box<dyn LogValueProtocol>);
impl LogValue {
    pub fn render(&self) -> DocumentValue {
        self.0.render()
    }
}
#[derive(Clone)]
pub struct DocumentLogValue {
    pub value: DocumentValue,
}
impl DocumentLogValue {
    pub fn terrane_construct(input: DocumentValue) -> Self {
        let mut value = Self {
            value: make_document_none(),
        };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: DocumentValue) {
        self.value = input.clone();
    }
    pub fn render(&self) -> DocumentValue {
        return self.value.to_document();
    }
}
impl LogValueProtocol for DocumentLogValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        DocumentLogValue::render(self)
    }
}
impl From<DocumentLogValue> for LogValue {
    fn from(value: DocumentLogValue) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct TextLogValue {
    pub value: String,
}
impl TextLogValue {
    pub fn terrane_construct(input: String) -> Self {
        let mut value = Self { value: String::from("") };
        value.construct(input);
        value
    }
    pub fn construct(&mut self, input: String) {
        self.value = input;
    }
    pub fn render(&self) -> DocumentValue {
        return make_document_string(self.value.clone());
    }
}
impl LogValueProtocol for TextLogValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        TextLogValue::render(self)
    }
}
impl From<TextLogValue> for LogValue {
    fn from(value: TextLogValue) -> Self {
        Self(Box::new(value))
    }
}
#[derive(Clone)]
pub struct ErrorLogValue {
    pub rendered: String,
}
impl ErrorLogValue {
    pub fn terrane_construct(rendered: String) -> Self {
        let mut value = Self { rendered: String::from("") };
        value.construct(rendered);
        value
    }
    pub fn construct(&mut self, rendered: String) {
        self.rendered = rendered;
    }
    pub fn render(&self) -> DocumentValue {
        return make_document_string(self.rendered.clone());
    }
}
impl LogValueProtocol for ErrorLogValue {
    fn clone_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn separate_box(&self) -> Box<dyn LogValueProtocol> {
        Box::new(self.clone())
    }
    fn render(&self) -> DocumentValue {
        ErrorLogValue::render(self)
    }
}
impl From<ErrorLogValue> for LogValue {
    fn from(value: ErrorLogValue) -> Self {
        Self(Box::new(value))
    }
}
pub fn log_document(value: DocumentValue) -> LogValue {
    return LogValue::from(DocumentLogValue::terrane_construct(value.clone()));
}
pub fn log_text(value: String) -> LogValue {
    return LogValue::from(TextLogValue::terrane_construct(value));
}
pub fn log_error(value: TerraneError) -> LogValue {
    return LogValue::from(ErrorLogValue::terrane_construct(value.render()));
}
#[derive(Clone)]
pub struct LogField {
    pub name: String,
    pub value: LogValue,
    pub secret: bool,
    pub source: String,
}
impl LogField {
    pub fn terrane_construct(
        name: String,
        input: LogValue,
        secret: bool,
        source: String,
    ) -> Self {
        let mut value = Self {
            name: String::from(""),
            value: LogValue::from(TextLogValue::terrane_construct(String::from(""))),
            secret: false,
            source: String::from(""),
        };
        value.construct(name, input, secret, source);
        value
    }
    pub fn construct(
        &mut self,
        name: String,
        input: LogValue,
        secret: bool,
        source: String,
    ) {
        self.name = name;
        self.value = input.clone();
        self.secret = secret;
        self.source = source;
    }
}
pub fn field_at(
    name: String,
    value: LogValue,
    secret: bool,
    source: String,
) -> LogField {
    return LogField::terrane_construct(name, value.clone(), secret, source);
}
pub fn field(name: String, value: LogValue) -> LogField {
    return field_at(
        name,
        value.clone(),
        false,
        String::from("builtin://core/logging.trn"),
    );
}
pub fn secret_field(name: String, value: LogValue) -> LogField {
    return field_at(
        name,
        value.clone(),
        true,
        String::from("builtin://core/logging.trn"),
    );
}
pub fn empty_log_fields() -> terrane_collection_support::List<LogField> {
    return terrane_collection_support::List::new(Vec::new());
}
#[derive(Clone)]
pub struct LogSink {
    pub handle: TerranePlatformCapability,
}
impl LogSink {
    pub fn terrane_construct(handle: TerranePlatformCapability) -> Self {
        let mut value = Self {
            handle: terrane_platform_support::logging_no_sink(),
        };
        value.construct(handle);
        value
    }
    pub fn construct(&mut self, handle: TerranePlatformCapability) {
        self.handle = handle;
    }
}
#[derive(Clone)]
pub struct LogSinkResult {
    pub failed: bool,
    pub message: String,
    pub value: LogSink,
}
impl LogSinkResult {
    pub fn terrane_construct(failed: bool, message: String, sink: LogSink) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: LogSink::terrane_construct(
                terrane_platform_support::logging_no_sink(),
            ),
        };
        value.construct(failed, message, sink);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, sink: LogSink) {
        self.failed = failed;
        self.message = message;
        self.value = sink.clone();
    }
}
pub fn memory_sink(
    capacity: terrane_int_support::Int,
    overflow: String,
    start: terrane_int_support::Int,
    step: terrane_int_support::Int,
    reveal: bool,
) -> LogSinkResult {
    let raw: TerranePlatformResult = terrane_platform_support::logging_memory_sink(
        terrane_int_support::checked_coerce::<i128>(&capacity.clone()),
        &overflow,
        terrane_int_support::checked_coerce::<i128>(&start.clone()),
        terrane_int_support::checked_coerce::<i128>(&step.clone()),
        reveal,
    );
    let sink: LogSink = LogSink::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return LogSinkResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        sink,
    );
}
pub fn console_sink(reveal: bool) -> LogSinkResult {
    let raw: TerranePlatformResult = terrane_platform_support::logging_console_sink(
        reveal,
    );
    let sink: LogSink = LogSink::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return LogSinkResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        sink,
    );
}
pub fn failing_sink() -> LogSinkResult {
    let raw: TerranePlatformResult = terrane_platform_support::logging_failing_sink();
    let sink: LogSink = LogSink::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return LogSinkResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
        sink,
    );
}
#[derive(Clone)]
pub struct LoggerOptions {
    pub minimum: LogLevel,
    pub target_prefix: String,
    pub target: String,
    pub max_fields: terrane_int_support::Int,
    pub max_bytes: terrane_int_support::Int,
}
impl LoggerOptions {
    pub fn terrane_construct(
        minimum: LogLevel,
        target_prefix: String,
        target: String,
        max_fields: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            minimum: info_level(),
            target_prefix: String::from(""),
            target: String::from("application"),
            max_fields: terrane_int_support::Int::from(64_i128),
            max_bytes: terrane_int_support::Int::from(65536_i128),
        };
        value.construct(minimum, target_prefix, target, max_fields, max_bytes);
        value
    }
    pub fn construct(
        &mut self,
        minimum: LogLevel,
        target_prefix: String,
        target: String,
        max_fields: terrane_int_support::Int,
        max_bytes: terrane_int_support::Int,
    ) {
        self.minimum = minimum.clone();
        self.target_prefix = target_prefix;
        self.target = target;
        self.max_fields = max_fields.clone();
        self.max_bytes = max_bytes.clone();
    }
}
pub fn make_logger_options(
    minimum: LogLevel,
    target_prefix: String,
    target: String,
    max_fields: terrane_int_support::Int,
    max_bytes: terrane_int_support::Int,
) -> LoggerOptions {
    return LoggerOptions::terrane_construct(
        minimum.clone(),
        target_prefix,
        target,
        max_fields.clone(),
        max_bytes.clone(),
    );
}
#[derive(Clone)]
pub struct LogContext {
    pub fields: terrane_collection_support::List<LogField>,
    pub spans: terrane_collection_support::List<String>,
}
impl LogContext {
    pub fn terrane_construct() -> Self {
        Self {
            fields: terrane_collection_support::List::new(Vec::new()),
            spans: terrane_collection_support::List::<String>::new(Vec::new()),
        }
    }
    pub fn append_field(&mut self, item: LogField) {
        let mut updated: terrane_collection_support::List<LogField> = terrane_collection_support::List::new(
            Vec::new(),
        );
        let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
            &self.fields,
        );
        {
            let __terrane_list_append_0 = updated.make_unique();
            loop {
                let existing = match __terrane_iterator_0.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                __terrane_list_append_0.push(existing);
            }
        }
        updated.append(item.clone());
        self.fields = updated.clone();
    }
    pub fn append_span(&mut self, item: String) {
        let mut updated: terrane_collection_support::List<String> = terrane_collection_support::List::<
            String,
        >::new(Vec::new());
        let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
            &self.spans,
        );
        {
            let __terrane_list_append_1 = updated.make_unique();
            loop {
                let existing = match __terrane_iterator_1.next() {
                    terrane_collection_support::IterationStep::Item(item) => item,
                    terrane_collection_support::IterationStep::End => break,
                };
                __terrane_list_append_1.push(existing);
            }
        }
        updated.append(item);
        self.spans = updated.clone();
    }
    pub fn adding_field(&self, addition: LogField) -> LogContext {
        let mut value: LogContext = LogContext::terrane_construct();
        let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
            &self.fields,
        );
        loop {
            let existing = match __terrane_iterator_2.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            value.append_field(existing);
        }
        value.append_field(addition.clone());
        let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
            &self.spans,
        );
        loop {
            let existing = match __terrane_iterator_3.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            value.append_span(existing);
        }
        return value.clone();
    }
    pub fn adding_span(&self, span: String) -> LogContext {
        let mut value: LogContext = LogContext::terrane_construct();
        let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
            &self.fields,
        );
        loop {
            let existing = match __terrane_iterator_4.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            value.append_field(existing);
        }
        let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
            &self.spans,
        );
        loop {
            let existing = match __terrane_iterator_5.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            value.append_span(existing);
        }
        value.append_span(span);
        return value.clone();
    }
}
#[derive(Clone)]
pub struct Logger {
    pub sink: LogSink,
    pub options: LoggerOptions,
    pub context: LogContext,
}
impl Logger {
    pub fn terrane_construct(
        sink: LogSink,
        options: LoggerOptions,
        context: LogContext,
    ) -> Self {
        let mut value = Self {
            sink: LogSink::terrane_construct(
                terrane_platform_support::logging_no_sink(),
            ),
            options: LoggerOptions::terrane_construct(
                info_level(),
                String::from(""),
                String::from("application"),
                terrane_int_support::Int::from(64_i128),
                terrane_int_support::Int::from(65536_i128),
            ),
            context: LogContext::terrane_construct(),
        };
        value.construct(sink, options, context);
        value
    }
    pub fn construct(
        &mut self,
        sink: LogSink,
        options: LoggerOptions,
        context: LogContext,
    ) {
        self.sink = sink.clone();
        self.options = options.clone();
        self.context = context.clone();
    }
}
pub fn make_logger(sink: LogSink, options: LoggerOptions) -> Logger {
    return Logger::terrane_construct(
        sink.clone(),
        options.clone(),
        LogContext::terrane_construct(),
    );
}
pub fn default_logger(sink: LogSink) -> Logger {
    return make_logger(
        sink.clone(),
        make_logger_options(
            info_level(),
            String::from(""),
            String::from("application"),
            terrane_int_support::Int::from(64_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
}
pub fn named_logger(sink: LogSink, target: String) -> Logger {
    return make_logger(
        sink.clone(),
        make_logger_options(
            info_level(),
            target.clone(),
            target,
            terrane_int_support::Int::from(64_i128),
            terrane_int_support::Int::from(65536_i128),
        ),
    );
}
pub fn with_field(value: Logger, addition: LogField) -> Logger {
    return Logger::terrane_construct(
        value.sink,
        value.options,
        value.context.adding_field(addition.clone()),
    );
}
pub fn with_span(value: Logger, span: String) -> Logger {
    return Logger::terrane_construct(
        value.sink,
        value.options,
        value.context.adding_span(span),
    );
}
#[derive(Clone)]
pub struct LogOutcome {
    pub filtered: bool,
    pub failed: bool,
    pub message: String,
}
impl LogOutcome {
    pub fn terrane_construct(filtered: bool, failed: bool, message: String) -> Self {
        let mut value = Self {
            filtered: false,
            failed: false,
            message: String::from(""),
        };
        value.construct(filtered, failed, message);
        value
    }
    pub fn construct(&mut self, filtered: bool, failed: bool, message: String) {
        self.filtered = filtered;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct LogEvent {
    pub level: LogLevel,
    pub message: String,
    pub fields: terrane_collection_support::List<LogField>,
    pub source: String,
}
impl LogEvent {
    pub fn terrane_construct(
        level: LogLevel,
        message: String,
        source: String,
        fields: terrane_collection_support::List<LogField>,
    ) -> Self {
        let mut value = Self {
            level: info_level(),
            message: String::from(""),
            fields: terrane_collection_support::List::new(Vec::new()),
            source: String::from(""),
        };
        value.construct(level, message, source, fields);
        value
    }
    pub fn construct(
        &mut self,
        level: LogLevel,
        message: String,
        source: String,
        fields: terrane_collection_support::List<LogField>,
    ) {
        self.level = level.clone();
        self.message = message;
        self.fields = fields.clone();
        self.source = source;
    }
}
pub fn make_event_at(
    level: LogLevel,
    message: String,
    source: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogEvent {
    return LogEvent::terrane_construct(level.clone(), message, source, fields.clone());
}
pub fn make_event(
    level: LogLevel,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogEvent {
    return make_event_at(
        level.clone(),
        message,
        String::from("builtin://core/logging.trn"),
        fields.clone(),
    );
}
pub fn emit_at(
    v: Logger,
    l: LogLevel,
    m: String,
    source: String,
    f: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    if l.rank.clone() < v.options.minimum.rank.clone() {
        return LogOutcome::terrane_construct(true, false, String::from(""));
    }
    if v.options.target_prefix != String::from("")
        && !v.options.target.starts_with(&v.options.target_prefix)
    {
        return LogOutcome::terrane_construct(true, false, String::from(""));
    }
    let mut combined: terrane_collection_support::List<LogField> = terrane_collection_support::List::new(
        Vec::new(),
    );
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &v.context.fields,
    );
    {
        let __terrane_list_append_2 = combined.make_unique();
        loop {
            let item = match __terrane_iterator_6.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_2.push(item);
        }
    }
    let mut __terrane_iterator_7 = terrane_collection_support::Iterable::terrane_iterator(
        &f,
    );
    {
        let __terrane_list_append_3 = combined.make_unique();
        loop {
            let item = match __terrane_iterator_7.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            __terrane_list_append_3.push(item);
        }
    }
    let options: LoggerOptions = v.options;
    let raw: TerranePlatformResult = {
        let sink = v.sink.handle;
        let severity = l.name;
        let target = options.target;
        let message = m;
        let raw_fields = combined;
        let source = source;
        let spans = v.context.spans;
        let max_fields_value = options.max_fields.clone();
        let max_bytes_value = options.max_bytes.clone();
        match (
            terrane_collection_support::index_from_int(&max_fields_value),
            terrane_collection_support::index_from_int(&max_bytes_value),
        ) {
            (Ok(max_fields), Ok(max_bytes)) => {
                let reveal_secrets = terrane_platform_support::logging_reveals_secrets(
                    &sink,
                );
                let fields = raw_fields
                    .into_vec()
                    .into_iter()
                    .map(|field| {
                        let value_json = if field.secret && !reveal_secrets {
                            "null".to_owned()
                        } else {
                            field.value.render().encoded.clone()
                        };
                        terrane_platform_support::LogFieldInput {
                            name: field.name,
                            value_json,
                            secret: field.secret,
                            source: field.source,
                        }
                    })
                    .collect::<Vec<_>>();
                terrane_platform_support::logging_emit(
                    &sink,
                    terrane_platform_support::LogEventInput {
                        severity,
                        target,
                        message,
                        fields,
                        source,
                        spans: spans.into_vec(),
                        max_fields,
                        max_bytes,
                        origin: "terrane".to_owned(),
                    },
                )
            }
            (Err(_), _) => {
                terrane_platform_support::ResultValue::error(
                    "logging field limit must be non-negative and fit this target",
                )
            }
            (_, Err(_)) => {
                terrane_platform_support::ResultValue::error(
                    "logging byte limit must be non-negative and fit this target",
                )
            }
        }
    };
    return LogOutcome::terrane_construct(
        false,
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
    );
}
pub fn emit(
    value: Logger,
    level: LogLevel,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        level.clone(),
        message,
        String::from("builtin://core/logging.trn"),
        fields.clone(),
    );
}
pub fn debug(
    value: Logger,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        debug_level().clone(),
        message.clone(),
        "core/logging.trn:246:12".to_owned(),
        fields.clone(),
    );
}
pub fn write_event(value: Logger, event: LogEvent) -> LogOutcome {
    return emit_at(
        value.clone(),
        event.level,
        event.message.clone(),
        event.source.clone(),
        event.fields,
    );
}
pub fn install_dependency_bridge(value: Logger) -> LogOutcome {
    let raw: TerranePlatformResult = terrane_platform_support::logging_install_dependency_bridge(
        &value.sink.handle,
    );
    return LogOutcome::terrane_construct(
        false,
        terrane_platform_result_failed(&raw),
        terrane_platform_result_message(&raw),
    );
}
pub fn info(
    value: Logger,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        info_level().clone(),
        message.clone(),
        "core/logging.trn:256:12".to_owned(),
        fields.clone(),
    );
}
pub fn warning(
    value: Logger,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        warning_level().clone(),
        message.clone(),
        "core/logging.trn:259:12".to_owned(),
        fields.clone(),
    );
}
pub fn error(
    value: Logger,
    message: String,
    fields: terrane_collection_support::List<LogField>,
) -> LogOutcome {
    return emit_at(
        value.clone(),
        error_level().clone(),
        message.clone(),
        "core/logging.trn:262:12".to_owned(),
        fields.clone(),
    );
}
pub fn discarded_count(value: Logger) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        i128::from(terrane_platform_support::logging_discarded_count(&value.sink.handle)),
    );
}
pub fn drain_memory(value: Logger) -> terrane_collection_support::List<String> {
    return terrane_collection_support::List::new(
        terrane_platform_result_entries(
            &terrane_platform_support::logging_drain(&value.sink.handle),
        ),
    );
}
pub fn drain_fallback() -> terrane_collection_support::List<String> {
    return terrane_collection_support::List::new(
        terrane_platform_result_entries(
            &terrane_platform_support::logging_drain_fallback(),
        ),
    );
}
// Source: core/logging_async.trn
// Namespace: core/logging/async
pub async fn send_event(sink: TerraneChannelSender<LogEvent>, value: LogEvent) -> bool {
    let outcome: TerraneChannelSendOutcome<LogEvent> = __terrane_await(
            Box::pin(sink.send(value.clone())),
        )
        .await;
    return outcome.accepted;
}
pub async fn consume_events(
    target: LogSink,
    source: TerraneChannelReceiver<LogEvent>,
) -> bool {
    let mut open: bool = true;
    let mut succeeded: bool = true;
    while open {
        let received: TerraneChannelReceiveOutcome<LogEvent> = __terrane_await(
                Box::pin(source.receive()),
            )
            .await;
        let event: Option<LogEvent> = received.value;
        if event.is_some() {
            let written: LogOutcome = write_event(
                default_logger(target.clone()),
                event.as_ref().expect("semantic optional narrowing").clone(),
            );
            if written.failed {
                succeeded = false;
                open = false;
            }
        }
        if received.closed {
            open = false;
        }
    }
    return succeeded;
}
