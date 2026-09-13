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
trait TerraneMutableCallableBody<Arguments, Output>: Send {
    fn call(&mut self, arguments: Arguments) -> Output;
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>>;
}
impl<Arguments, Output, Function> TerraneMutableCallableBody<Arguments, Output>
for Function
where
    Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
{
    fn call(&mut self, arguments: Arguments) -> Output {
        self(arguments)
    }
    fn clone_box(
        &self,
    ) -> std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>> {
        std::boxed::Box::new(self.clone())
    }
}
pub struct TerraneMutableCallable<Arguments, Output> {
    body: std::sync::Mutex<
        std::boxed::Box<dyn TerraneMutableCallableBody<Arguments, Output>>,
    >,
}
impl<Arguments, Output> TerraneMutableCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnMut(Arguments) -> Output + Clone + Send + 'static,
    {
        Self {
            body: std::sync::Mutex::new(std::boxed::Box::new(function)),
        }
    }
    fn call(&self, arguments: Arguments) -> Output {
        self.body.lock().expect("mutable callable lock poisoned").call(arguments)
    }
}
impl<Arguments, Output> Clone for TerraneMutableCallable<Arguments, Output> {
    fn clone(&self) -> Self {
        let body = self.body.lock().expect("mutable callable lock poisoned").clone_box();
        Self {
            body: std::sync::Mutex::new(body),
        }
    }
}
struct TerraneAsyncInvocationGate {
    invocation: std::sync::Arc<tokio::sync::Mutex<()>>,
}
impl TerraneAsyncInvocationGate {
    fn new() -> Self {
        Self {
            invocation: std::sync::Arc::new(tokio::sync::Mutex::new(())),
        }
    }
    fn share(&self) -> Self {
        Self {
            invocation: self.invocation.clone(),
        }
    }
    async fn enter(&self) -> tokio::sync::OwnedMutexGuard<()> {
        self.invocation.clone().lock_owned().await
    }
}
impl Clone for TerraneAsyncInvocationGate {
    fn clone(&self) -> Self {
        Self::new()
    }
}
pub struct TerraneAsyncMutableState<Value> {
    value: std::sync::Arc<std::sync::Mutex<Value>>,
    invocation: std::sync::Arc<tokio::sync::Mutex<()>>,
}
impl<Value> TerraneAsyncMutableState<Value> {
    fn new(value: Value) -> Self {
        Self {
            value: std::sync::Arc::new(std::sync::Mutex::new(value)),
            invocation: std::sync::Arc::new(tokio::sync::Mutex::new(())),
        }
    }
    fn share(&self) -> Self {
        Self {
            value: self.value.clone(),
            invocation: self.invocation.clone(),
        }
    }
    fn replace(&self, value: Value) {
        *self.value.lock().expect("mutable async callable state lock poisoned") = value;
    }
}
impl<Value: Clone> TerraneAsyncMutableState<Value> {
    fn snapshot(&self) -> Value {
        self.value.lock().expect("mutable async callable state lock poisoned").clone()
    }
    async fn with_receiver<Output, Operation>(&self, operation: Operation) -> Output
    where
        Value: Send,
        Operation: for<'receiver> FnOnce(
                &'receiver mut Value,
            ) -> std::pin::Pin<Box<dyn Future<Output = Output> + Send + 'receiver>>
            + Send,
    {
        let _invocation = self.invocation.lock().await;
        let mut receiver = self.snapshot();
        let result = operation(&mut receiver).await;
        self.replace(receiver);
        result
    }
}
impl<Value: Clone> Clone for TerraneAsyncMutableState<Value> {
    fn clone(&self) -> Self {
        Self::new(self.snapshot())
    }
}
trait TerraneConsumingCallableBody<Arguments, Output>: Send {
    fn call(self: std::boxed::Box<Self>, arguments: Arguments) -> Output;
}
impl<Arguments, Output, Function> TerraneConsumingCallableBody<Arguments, Output>
for Function
where
    Function: FnOnce(Arguments) -> Output + Send + 'static,
{
    fn call(self: std::boxed::Box<Self>, arguments: Arguments) -> Output {
        self(arguments)
    }
}
pub struct TerraneConsumingCallable<Arguments, Output> {
    body: std::boxed::Box<dyn TerraneConsumingCallableBody<Arguments, Output>>,
}
impl<Arguments, Output> TerraneConsumingCallable<Arguments, Output> {
    fn new<Function>(function: Function) -> Self
    where
        Function: FnOnce(Arguments) -> Output + Send + 'static,
    {
        Self {
            body: std::boxed::Box::new(function),
        }
    }
    fn call(self, arguments: Arguments) -> Output {
        self.body.call(arguments)
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
// Namespace: callable-invocation-modes
fn double(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
}
async fn double_later(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
}
#[derive(Clone)]
pub struct Accumulator {
    pub total: terrane_int_support::Int,
}
impl Accumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn add(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int {
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
}
#[derive(Clone)]
pub struct Ticket {
    pub message: String,
}
impl Ticket {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("redeemed"),
        }
    }
    pub fn redeem(self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct AsyncAccumulator {
    pub total: terrane_int_support::Int,
}
impl AsyncAccumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub async fn add(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
}
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
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int> = __terrane_await(
                Box::pin(started.send(terrane_int_support::Int::from(1_i128))),
            )
            .await;
        if !sent.accepted {
            return terrane_int_support::Int::from(-1_i128);
        }
        let released: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
                Box::pin(gate.receive()),
            )
            .await;
        if !released.available {
            return terrane_int_support::Int::from(-2_i128);
        }
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
}
fn main() {
    __terrane_run(async move {
        let counter: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        let step: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = {
            let mut counter = counter.clone();
            TerraneMutableCallable::new(move |
                (delta,): (terrane_int_support::Int,),
            | -> terrane_int_support::Int {
                counter = counter.clone() + delta.clone();
                return counter.clone();
            })
        };
        let message: String = String::from("finished");
        let finish: TerraneConsumingCallable<(), String> = {
            let message = message.clone();
            TerraneConsumingCallable::new(move |(): ()| -> String {
                return message;
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(2_i128),)))
        );
        let copy: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = step.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = step; "mutable"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = step; "mutable"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&copy
            .call((terrane_int_support::Int::from(10_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = finish; "consuming"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = finish; "consuming"
            .to_owned() })
        );
        println!("{}", terrane_scalar_support::scalar_text(&finish.call(())));
        let operation: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = TerraneMutableCallable::new(move |
            (argument_0,): (terrane_int_support::Int,)|
        double(argument_0));
        println!(
            "{}", terrane_scalar_support::scalar_text(&operation
            .call((terrane_int_support::Int::from(6_i128),)))
        );
        let async_counter: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let async_step: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = {
            let async_counter = TerraneAsyncMutableState::new(async_counter.clone());
            let __terrane_invocation = TerraneAsyncInvocationGate::new();
            TerraneMutableCallable::new(move |
                (delta,): (terrane_int_support::Int,),
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let async_counter = async_counter.share();
                let __terrane_invocation = __terrane_invocation.share();
                Box::pin(async move {
                    let _invocation = __terrane_invocation.enter().await;
                    {
                        let callable_capture_value = async_counter.snapshot()
                            + delta.clone();
                        async_counter.replace(callable_capture_value);
                    }
                    return async_counter.snapshot();
                })
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(2_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(3_i128),))). await)
        );
        let async_copy: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = async_step.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(1_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_copy
            .call((terrane_int_support::Int::from(10_i128),))). await)
        );
        let asynchronous: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = TerraneMutableCallable::new(move |
            (argument_0,): (terrane_int_support::Int,),
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(double_later(argument_0))
        });
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(asynchronous
            .call((terrane_int_support::Int::from(7_i128),))). await)
        );
        let value: Accumulator = Accumulator::terrane_construct();
        let bound: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = {
            let mut receiver = value.clone();
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            receiver.add(argument_0))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound
            .call((terrane_int_support::Int::from(5_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound
            .call((terrane_int_support::Int::from(7_i128),)))
        );
        let _ = &value;
        let value: Ticket = Ticket::terrane_construct();
        let redemption: TerraneConsumingCallable<(), String> = {
            let receiver = value.clone();
            TerraneConsumingCallable::new(move |(): ()| receiver.redeem())
        };
        println!("{}", terrane_scalar_support::scalar_text(&redemption.call(())));
        let shared: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        > = {
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> terrane_int_support::Int {
                return value.clone() + terrane_int_support::Int::from(1_i128);
            })
        };
        let adapted: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = {
            let callable = shared.clone();
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            callable(argument_0))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&adapted
            .call((terrane_int_support::Int::from(8_i128),)))
        );
        let async_value: AsyncAccumulator = AsyncAccumulator::terrane_construct();
        let async_bound: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = {
            let receiver = TerraneAsyncMutableState::new(async_value);
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                let receiver = receiver.share();
                Box::pin(async move {
                    receiver
                        .with_receiver(move |receiver| Box::pin(async move {
                            receiver.add(argument_0).await
                        }))
                        .await
                })
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_bound
            .call((terrane_int_support::Int::from(2_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_bound
            .call((terrane_int_support::Int::from(3_i128),))). await)
        );
        let one_shot: TerraneConsumingCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = {
            let callable = adapted.clone();
            TerraneConsumingCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            callable.call((argument_0,)))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&one_shot
            .call((terrane_int_support::Int::from(10_i128),)))
        );
        let declared_mutable: TerraneMutableCallable<(), terrane_int_support::Int> = {
            TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
                return terrane_int_support::Int::from(9_i128);
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = declared_mutable;
            "mutable".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = declared_mutable;
            "shared".to_owned() })
        );
        println!("{}", terrane_scalar_support::scalar_text(&declared_mutable.call(())));
        let gate: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let started: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let gated_value: GatedAccumulator = GatedAccumulator::terrane_construct();
        let gated_bound: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = {
            let receiver = TerraneAsyncMutableState::new(gated_value);
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
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let child: TerraneScopedTask<terrane_int_support::Int> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = gated_bound
                .call((
                    terrane_int_support::Int::from(4_i128),
                    started.sender,
                    gate.receiver,
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
        let observed: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
                started.receiver.receive(),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&observed.available));
        let gated_copy: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = gated_bound.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = gated_copy; "mutable"
            .to_owned() })
        );
        let released: TerraneChannelSendOutcome<terrane_int_support::Int> = __terrane_await(
                gate.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&released.accepted));
        let outcome: TerraneTaskOutcome<terrane_int_support::Int> = __terrane_await(
                scope.join(child),
            )
            .await;
        let result: Option<terrane_int_support::Int> = outcome.value.clone();
        if result.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&* result.as_ref()
                .expect("semantic optional narrowing"))
            );
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
