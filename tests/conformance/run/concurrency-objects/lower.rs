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
fn __terrane_block_on<F: Future>(future: F) -> F::Output {
    struct Wake;
    impl std::task::Wake for Wake {
        fn wake(self: std::sync::Arc<Self>) {}
    }
    let waker = std::task::Waker::from(std::sync::Arc::new(Wake));
    let mut context = std::task::Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(value) => return value,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}
fn __terrane_block_on_cancellable<F: Future>(
    future: F,
    cancelled: impl Fn() -> bool,
) -> Option<F::Output> {
    struct Wake;
    impl std::task::Wake for Wake {
        fn wake(self: std::sync::Arc<Self>) {}
    }
    let waker = std::task::Waker::from(std::sync::Arc::new(Wake));
    let mut context = std::task::Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    loop {
        if cancelled() {
            return None;
        }
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(value) => return Some(value),
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}
#[derive(Clone)]
pub struct TerraneTaskScope {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    deadline: Option<std::time::Instant>,
}
impl TerraneTaskScope {
    pub fn new(deadline_ms: Option<u64>) -> Self {
        Self {
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            deadline: deadline_ms
                .map(|value| {
                    std::time::Instant::now() + std::time::Duration::from_millis(value)
                }),
        }
    }
    pub fn child_scope(&self, deadline_ms: u64) -> Self {
        let requested = std::time::Instant::now()
            + std::time::Duration::from_millis(deadline_ms);
        let deadline = Some(
            self.deadline.map_or(requested, |parent| std::cmp::min(parent, requested)),
        );
        Self {
            cancelled: self.cancelled.clone(),
            deadline,
        }
    }
    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }
    pub fn should_cancel(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::Acquire)
            || self
                .deadline
                .is_some_and(|deadline| std::time::Instant::now() >= deadline)
    }
    pub fn join<T>(&self, mut task: TerraneScopedTask<T>) -> TerraneTaskOutcome<T> {
        let result = task
            .handle
            .take()
            .expect("scoped task joined once")
            .join()
            .expect("task worker panicked");
        match result {
            TerraneTaskResult::Completed(value) => {
                TerraneTaskOutcome {
                    completed: true,
                    cancelled: self.should_cancel(),
                    value: Some(value),
                    error: None,
                }
            }
            TerraneTaskResult::Failed(error) => {
                self.cancel();
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
    handle: Option<std::thread::JoinHandle<TerraneTaskResult<T>>>,
}
impl<T: Send + 'static> TerraneScopedTask<T> {
    #[allow(dead_code, reason = "task spawn ABI is emitted before usage shaping")]
    fn spawn<F: FnOnce() -> TerraneTaskResult<T> + Send + 'static>(work: F) -> Self {
        Self {
            handle: Some(std::thread::spawn(work)),
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
fn terrane_platform_int_channel(
    capacity: terrane_int_support::Int,
) -> TerranePlatformResult {
    let capacity = terrane_platform_i128!(capacity, "channel capacity");
    terrane_platform_support::int_channel(capacity)
}
#[allow(dead_code)]
fn terrane_platform_int_channel_send(
    channel: &TerranePlatformCapability,
    value: terrane_int_support::Int,
    deadline_ms: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let value = terrane_platform_i128!(value, "channel value");
    let deadline_ms = terrane_platform_i128!(deadline_ms, "channel send deadline");
    terrane_platform_support::int_channel_send(channel, value, deadline_ms, cancellation)
}
#[allow(dead_code)]
fn terrane_platform_int_channel_receive(
    channel: &TerranePlatformCapability,
    deadline_ms: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let deadline_ms = terrane_platform_i128!(deadline_ms, "channel receive deadline");
    terrane_platform_support::int_channel_receive(channel, deadline_ms, cancellation)
}
#[allow(dead_code)]
fn terrane_platform_int_channel_try_receive(
    channel: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::int_channel_try_receive(channel)
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
// Source: src/main.trn
// Namespace: concurrency-objects
#[derive(Clone)]
pub struct AsyncRunner {
    pub callback: std::sync::Arc<dyn Fn() -> () + Send + Sync>,
}
impl AsyncRunner {
    pub fn terrane_construct(
        callback: std::sync::Arc<dyn Fn() -> () + Send + Sync>,
    ) -> Self {
        let mut value = Self {
            callback: {
                std::sync::Arc::new(move || -> () {
                    return ();
                })
            },
        };
        value.construct(callback);
        value
    }
    pub fn construct(&mut self, callback: std::sync::Arc<dyn Fn() -> () + Send + Sync>) {
        self.callback = callback.clone();
    }
    pub async fn run(&self) {
        let callback: std::sync::Arc<dyn Fn() -> () + Send + Sync> = {
            let receiver = self.clone();
            std::sync::Arc::new(move || (receiver.callback)())
        };
        callback();
    }
}
fn main() {
    let options: OperationOptions = OperationOptions::terrane_construct(
        terrane_int_support::Int::from(30000_i128),
        CancellationToken::terrane_construct(),
    );
    let messages: IntChannel = IntChannel::terrane_construct(
        terrane_int_support::Int::from(0_i128),
    );
    let counter: IntMutex = IntMutex::terrane_construct(
        terrane_int_support::Int::from(4_i128),
    );
    let work: std::sync::Arc<dyn Fn() -> () + Send + Sync> = {
        let counter = counter.clone();
        let messages = messages.clone();
        let options = options.clone();
        std::sync::Arc::new(move || -> () {
            messages.send(terrane_int_support::Int::from(11_i128), options.clone());
            counter.increase(terrane_int_support::Int::from(3_i128));
        })
    };
    let worker: AsyncRunner = AsyncRunner::terrane_construct(work.clone());
    let scope: TerraneTaskScope = TerraneTaskScope::new(None);
    let child: TerraneScopedTask<()> = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.clone();
        TerraneScopedTask::spawn(move || match __terrane_block_on_cancellable(
            {
                let receiver = worker;
                std::sync::Arc::new(move || -> std::pin::Pin<
                    Box<dyn Future<Output = _>>,
                > {
                    let receiver = receiver.clone();
                    Box::pin(async move { receiver.run().await })
                })
            }(),
            move || __terrane_cancel.should_cancel(),
        ) {
            Some(value) => TerraneTaskResult::Completed(value),
            None => TerraneTaskResult::Cancelled,
        })
    };
    let received: IntResult = messages.receive(options.clone());
    let outcome: TerraneTaskOutcome<()> = scope.join(child);
    println!("{}", terrane_scalar_support::scalar_text(&messages.failed));
    let invalid_channel: IntChannel = IntChannel::terrane_construct(
        terrane_int_support::Int::from(-1_i128),
    );
    println!("{}", terrane_scalar_support::scalar_text(&invalid_channel.failed));
    let cancelled: CancellationToken = CancellationToken::terrane_construct();
    cancelled.cancel();
    let cancelled_options: OperationOptions = OperationOptions::terrane_construct(
        terrane_int_support::Int::from(1000_i128),
        cancelled.clone(),
    );
    let cancelled_channel: IntChannel = IntChannel::terrane_construct(
        terrane_int_support::Int::from(1_i128),
    );
    let cancelled_receive: IntResult = cancelled_channel.receive(cancelled_options);
    println!("{}", terrane_scalar_support::scalar_text(&cancelled_receive.failed));
    let timeout_options: OperationOptions = OperationOptions::terrane_construct(
        terrane_int_support::Int::from(1_i128),
        CancellationToken::terrane_construct(),
    );
    let timeout_channel: IntChannel = IntChannel::terrane_construct(
        terrane_int_support::Int::from(1_i128),
    );
    let timed_out: IntResult = timeout_channel.receive(timeout_options);
    println!("{}", terrane_scalar_support::scalar_text(&timed_out.deadline_exceeded));
    println!("{}", terrane_scalar_support::scalar_text(&received.available));
    println!("{}", terrane_scalar_support::scalar_text(&received.value));
    println!("{}", terrane_scalar_support::scalar_text(&outcome.completed));
    println!("{}", terrane_scalar_support::scalar_text(&counter.load().value));
    let shared: IntReadWriteLock = IntReadWriteLock::terrane_construct(
        terrane_int_support::Int::from(8_i128),
    );
    shared.write(terrane_int_support::Int::from(9_i128));
    println!("{}", terrane_scalar_support::scalar_text(&shared.read().value));
    let atomic: AtomicInt64 = AtomicInt64::terrane_construct(10);
    let updated: IntResult = atomic.increase(5, acquire_release_order());
    println!("{}", terrane_scalar_support::scalar_text(&updated.failed));
    println!(
        "{}", terrane_scalar_support::scalar_text(&atomic.load(acquire_order()).value)
    );
    let invalid_store: OperationResult = atomic.store(16, acquire_release_order());
    println!("{}", terrane_scalar_support::scalar_text(&invalid_store.failed));
    let invalid_ordering: IntResult = atomic.load(release_order());
    println!("{}", terrane_scalar_support::scalar_text(&invalid_ordering.failed));
    println!("{}", terrane_scalar_support::scalar_text(&invalid_ordering.available));
    let local: ThreadLocalInt = ThreadLocalInt::terrane_construct(
        terrane_int_support::Int::from(20_i128),
    );
    local.write(terrane_int_support::Int::from(21_i128));
    println!("{}", terrane_scalar_support::scalar_text(&local.get().value));
}
// Source: core/concurrency.trn
// Namespace: core/concurrency
#[derive(Clone)]
pub struct OperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl OperationResult {
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
pub struct CancellationToken {
    pub handle: TerranePlatformCapability,
}
impl CancellationToken {
    pub fn terrane_construct() -> Self {
        Self {
            handle: terrane_platform_cancellation_token(),
        }
    }
    pub fn cancel(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_cancel(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
#[derive(Clone)]
pub struct OperationOptions {
    pub deadline_ms: terrane_int_support::Int,
    pub cancellation: CancellationToken,
}
impl OperationOptions {
    pub fn terrane_construct(
        deadline_ms: terrane_int_support::Int,
        cancellation: CancellationToken,
    ) -> Self {
        let mut value = Self {
            deadline_ms: terrane_int_support::Int::from(30000_i128),
            cancellation: CancellationToken::terrane_construct(),
        };
        value.construct(deadline_ms, cancellation);
        value
    }
    pub fn construct(
        &mut self,
        deadline_ms: terrane_int_support::Int,
        cancellation: CancellationToken,
    ) {
        self.deadline_ms = deadline_ms.clone();
        self.cancellation = cancellation.clone();
    }
}
pub fn cancel_operation(cancellation: CancellationToken) -> OperationResult {
    let raw: TerranePlatformResult = terrane_platform_cancel(&cancellation.handle);
    return OperationResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
    );
}
#[derive(Clone)]
pub struct IntResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub available: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl IntResult {
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
pub struct IntChannel {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntChannel {
    pub fn terrane_construct(capacity: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(capacity);
        value
    }
    pub fn construct(&mut self, capacity: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_channel(capacity);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn send(
        &self,
        value: terrane_int_support::Int,
        options: OperationOptions,
    ) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_channel_send(
            &self.handle,
            value,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn receive(&self, options: OperationOptions) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_channel_receive(
            &self.handle,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn try_receive(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_channel_try_receive(
            &self.handle,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
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
    pub fn load(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_load(&self.handle);
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_store(
            &self.handle,
            value,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(&self, amount: terrane_int_support::Int) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_add(
            &self.handle,
            amount,
        );
        return IntResult::terrane_construct(
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
    pub fn read(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_read(&self.handle);
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_write(
            &self.handle,
            value,
        );
        return OperationResult::terrane_construct(
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
    pub fn load(&self, ordering: MemoryOrder) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: i64, ordering: MemoryOrder) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(&self, amount: i64, ordering: MemoryOrder) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return IntResult::terrane_construct(
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
    pub fn get(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_get(
            &self.handle,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
