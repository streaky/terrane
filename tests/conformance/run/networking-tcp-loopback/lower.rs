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
    pub static FILES: [&str; 2] = ["case.trn", "core/networking.trn"];
    pub static FUNCTIONS: [&str; 2] = ["/app::main", "/core/networking::lookup-dns"];
    pub static SITES: [Site; 2] = [
        {
            /* terrane-site-row: site 0: /app::main (case.trn:45:13-45:39) */
            Site {
                function: 0,
                file: 0,
                line: 45,
                column: 13,
                end_line: 45,
                end_column: 39,
            }
        },
        {
            /* terrane-site-row: site 1: /core/networking::lookup-dns (core/networking.trn:319:28-319:49) */
            Site {
                function: 1,
                file: 1,
                line: 319,
                column: 28,
                end_line: 319,
                end_column: 49,
            }
        },
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
pub fn terrane_platform_parse_ip(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_ip(&text)
}
pub fn terrane_platform_parse_host_name(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_host_name(&text)
}
pub fn terrane_platform_parse_socket(
    ip: &String,
    port: &terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::parse_socket(
        ip,
        terrane_platform_i128!(port, "socket port"),
    )
}
pub fn terrane_platform_parse_socket_text(text: String) -> TerranePlatformResult {
    terrane_platform_support::parse_socket_text(&text)
}
pub fn terrane_platform_tcp_bind(address: String) -> TerranePlatformResult {
    terrane_platform_support::tcp_bind(&address)
}
pub fn terrane_platform_tcp_connect(
    address: String,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect(
        &address,
        terrane_platform_i128!(deadline, "TCP connect deadline"),
        cancellation,
    )
}
pub fn terrane_platform_tcp_connect_host(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_connect_host(
        &host,
        terrane_platform_i128!(port, "TCP host port"),
        terrane_platform_i128!(deadline, "TCP host connect deadline"),
        cancellation,
    )
}
pub fn terrane_platform_tcp_accept(
    listener: &TerranePlatformCapability,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_accept(
        listener,
        terrane_platform_i128!(deadline, "TCP accept deadline"),
        cancellation,
    )
}
pub fn terrane_platform_tcp_read(
    stream: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_read(
        stream,
        terrane_platform_i128!(limit, "TCP read limit"),
        terrane_platform_i128!(deadline, "TCP read deadline"),
        cancellation,
    )
}
pub fn terrane_platform_tcp_write(
    stream: &TerranePlatformCapability,
    data: Vec<u8>,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_write(
        stream,
        &data,
        terrane_platform_i128!(deadline, "TCP write deadline"),
        cancellation,
    )
}
pub fn terrane_platform_tcp_shutdown(
    stream: &TerranePlatformCapability,
    direction: String,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_shutdown(stream, &direction)
}
pub fn terrane_platform_tcp_configure(
    stream: &TerranePlatformCapability,
    no_delay: bool,
    ttl: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::tcp_configure(
        stream,
        no_delay,
        terrane_platform_i128!(ttl, "TCP TTL"),
    )
}
pub fn terrane_platform_udp_bind(address: String) -> TerranePlatformResult {
    terrane_platform_support::udp_bind(&address)
}
pub fn terrane_platform_udp_configure(
    socket: &TerranePlatformCapability,
    broadcast: bool,
    ttl: terrane_int_support::Int,
) -> TerranePlatformResult {
    terrane_platform_support::udp_configure(
        socket,
        broadcast,
        terrane_platform_i128!(ttl, "UDP TTL"),
    )
}
pub fn terrane_platform_udp_send_to(
    socket: &TerranePlatformCapability,
    data: Vec<u8>,
    address: String,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::udp_send_to(
        socket,
        &data,
        &address,
        terrane_platform_i128!(deadline, "UDP send deadline"),
        cancellation,
    )
}
pub fn terrane_platform_udp_receive_from(
    socket: &TerranePlatformCapability,
    limit: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::udp_receive_from(
        socket,
        terrane_platform_i128!(limit, "UDP receive limit"),
        terrane_platform_i128!(deadline, "UDP receive deadline"),
        cancellation,
    )
}
pub fn terrane_platform_dns_lookup(
    host: String,
    port: terrane_int_support::Int,
    deadline: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::dns_lookup(
        &host,
        terrane_platform_i128!(port, "DNS port"),
        terrane_platform_i128!(deadline, "DNS deadline"),
        cancellation,
    )
}
pub fn terrane_platform_close(
    capability: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::close(capability)
}
// Source: case.trn
// Namespace: app
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
    let loopback: IpResult = ip_address_from_string(String::from("127.0.0.1"));
    let bind_address: SocketResult = socket_address_from_ip(
        loopback.value,
        terrane_int_support::Int::from(0_i128),
    );
    let bound: ListenerResult = bind_tcp(bind_address.value);
    let listener: TcpListener = bound.value;
    let endpoint_text: String = listener.local_address.clone();
    let server_options: NetworkOperationOptions = NetworkOperationOptions::terrane_construct(
        terrane_int_support::Int::from(1000_i128),
        NetworkCancellationToken::terrane_construct(),
    );
    let serve: std::sync::Arc<dyn Fn() -> () + Send + Sync> = {
        let listener = listener;
        let server_options = server_options.clone();
        std::sync::Arc::new(move || -> () {
            let accepted: StreamResult = listener.accept(server_options.clone());
            let stream: TcpStream = accepted.value;
            let request: IoResult = stream
                .read(terrane_int_support::Int::from(7_i128), server_options.clone());
            if request.data != Vec::from([116, 101, 114, 114, 97, 110, 101]) {
                return ();
            }
            stream.write(Vec::from([114, 101, 112, 108, 121]), server_options.clone());
            stream.shutdown(String::from("both"));
            stream.close();
        })
    };
    let server: AsyncRunner = AsyncRunner::terrane_construct(serve.clone());
    let scope: TerraneTaskScope = TerraneTaskScope::new(None);
    let child: TerraneScopedTask<()> = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.clone();
        TerraneScopedTask::spawn(move || match __terrane_block_on_cancellable(
            {
                let receiver = server;
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
    let destination: SocketResult = socket_address_from_string(endpoint_text);
    let client_options: NetworkOperationOptions = NetworkOperationOptions::terrane_construct(
        terrane_int_support::Int::from(1000_i128),
        NetworkCancellationToken::terrane_construct(),
    );
    let connected: StreamResult = connect_tcp(destination.value, client_options.clone());
    let client: TcpStream = connected.value;
    let sent: IoResult = client
        .write(Vec::from([116, 101, 114, 114, 97, 110, 101]), client_options.clone());
    let response: IoResult = client
        .read(terrane_int_support::Int::from(5_i128), client_options.clone());
    let outcome: TerraneTaskOutcome<()> = scope.join(child);
    println!("{}", terrane_scalar_support::scalar_text(&sent.completed));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&response
        .data, terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:45:13-45:39 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&outcome.completed));
    client.close();
}
// Source: core/networking.trn
// Namespace: core/networking
#[derive(Clone)]
pub struct NetworkOperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl NetworkOperationResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        value.construct(failed, deadline_exceeded, message);
        value
    }
    pub fn construct(&mut self, failed: bool, deadline_exceeded: bool, message: String) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct NetworkCancellationToken {
    pub handle: TerranePlatformCapability,
}
impl NetworkCancellationToken {
    pub fn terrane_construct() -> Self {
        Self {
            handle: terrane_platform_cancellation_token(),
        }
    }
}
pub fn network_cancel_operation(
    cancellation: NetworkCancellationToken,
) -> NetworkOperationResult {
    let raw: TerranePlatformResult = terrane_platform_cancel(&cancellation.handle);
    return NetworkOperationResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
    );
}
#[derive(Clone)]
pub struct NetworkOperationOptions {
    pub deadline_ms: terrane_int_support::Int,
    pub cancellation: NetworkCancellationToken,
}
impl NetworkOperationOptions {
    pub fn terrane_construct(
        deadline_ms: terrane_int_support::Int,
        cancellation: NetworkCancellationToken,
    ) -> Self {
        let mut value = Self {
            deadline_ms: terrane_int_support::Int::from(30000_i128),
            cancellation: NetworkCancellationToken::terrane_construct(),
        };
        value.construct(deadline_ms, cancellation);
        value
    }
    pub fn construct(
        &mut self,
        deadline_ms: terrane_int_support::Int,
        cancellation: NetworkCancellationToken,
    ) {
        self.deadline_ms = deadline_ms.clone();
        self.cancellation = cancellation.clone();
    }
}
pub fn operation_cancellation(
    options: NetworkOperationOptions,
) -> TerranePlatformCapability {
    return options.cancellation.handle;
}
#[derive(Clone)]
pub struct TcpOptions {
    pub no_delay: bool,
    pub ttl: terrane_int_support::Int,
}
impl TcpOptions {
    pub fn terrane_construct(no_delay: bool, ttl: terrane_int_support::Int) -> Self {
        let mut value = Self {
            no_delay: true,
            ttl: terrane_int_support::Int::from(64_i128),
        };
        value.construct(no_delay, ttl);
        value
    }
    pub fn construct(&mut self, no_delay: bool, ttl: terrane_int_support::Int) {
        self.no_delay = no_delay;
        self.ttl = ttl.clone();
    }
}
#[derive(Clone)]
pub struct UdpOptions {
    pub broadcast: bool,
    pub ttl: terrane_int_support::Int,
}
impl UdpOptions {
    pub fn terrane_construct(broadcast: bool, ttl: terrane_int_support::Int) -> Self {
        let mut value = Self {
            broadcast: false,
            ttl: terrane_int_support::Int::from(64_i128),
        };
        value.construct(broadcast, ttl);
        value
    }
    pub fn construct(&mut self, broadcast: bool, ttl: terrane_int_support::Int) {
        self.broadcast = broadcast;
        self.ttl = ttl.clone();
    }
}
#[derive(Clone)]
pub struct IpAddress {
    pub value: String,
    pub version: String,
    pub is_loopback: bool,
}
impl IpAddress {
    pub fn terrane_construct(raw: TerranePlatformResult) -> Self {
        let mut value = Self {
            value: String::from(""),
            version: String::from(""),
            is_loopback: false,
        };
        value.construct(raw);
        value
    }
    pub fn construct(&mut self, raw: TerranePlatformResult) {
        self.value = terrane_platform_result_text(&raw);
        self.version = terrane_platform_result_detail(&raw);
        self.is_loopback = terrane_platform_result_bool(&raw);
    }
    pub fn string(&self) -> String {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct IpResult {
    pub failed: bool,
    pub message: String,
    pub value: IpAddress,
}
impl IpResult {
    pub fn terrane_construct(failed: bool, message: String, address: IpAddress) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: IpAddress::terrane_construct(terrane_platform_failed_result()),
        };
        value.construct(failed, message, address);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, address: IpAddress) {
        self.failed = failed;
        self.message = message;
        self.value = address.clone();
    }
}
pub fn ip_address_from_string(text: String) -> IpResult {
    let raw: TerranePlatformResult = terrane_platform_parse_ip(text);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    return IpResult::terrane_construct(
        failed,
        message,
        IpAddress::terrane_construct(raw),
    );
}
#[derive(Clone)]
pub struct SocketAddress {
    pub value: String,
    pub ip: IpAddress,
    pub port: terrane_int_support::Int,
}
impl SocketAddress {
    pub fn terrane_construct(
        raw: TerranePlatformResult,
        address_ip: IpAddress,
        address_port: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            value: String::from(""),
            ip: IpAddress::terrane_construct(terrane_platform_failed_result()),
            port: terrane_int_support::Int::from(0_i128),
        };
        value.construct(raw, address_ip, address_port);
        value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformResult,
        address_ip: IpAddress,
        address_port: terrane_int_support::Int,
    ) {
        self.value = terrane_platform_result_text(&raw);
        self.ip = address_ip.clone();
        self.port = address_port.clone();
    }
    pub fn string(&self) -> String {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct SocketResult {
    pub failed: bool,
    pub message: String,
    pub value: SocketAddress,
}
impl SocketResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        address: SocketAddress,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: SocketAddress::terrane_construct(
                terrane_platform_failed_result(),
                IpAddress::terrane_construct(terrane_platform_failed_result()),
                terrane_int_support::Int::from(0_i128),
            ),
        };
        value.construct(failed, message, address);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, address: SocketAddress) {
        self.failed = failed;
        self.message = message;
        self.value = address.clone();
    }
}
pub fn socket_address_from_ip(
    ip: IpAddress,
    port: terrane_int_support::Int,
) -> SocketResult {
    let raw: TerranePlatformResult = terrane_platform_parse_socket(&ip.value, &port);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    let address: SocketAddress = SocketAddress::terrane_construct(
        raw,
        ip.clone(),
        port.clone(),
    );
    return SocketResult::terrane_construct(failed, message, address);
}
pub fn socket_address_from_string(text: String) -> SocketResult {
    let raw: TerranePlatformResult = terrane_platform_parse_socket_text(text);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    let address_ip: IpAddress = IpAddress::terrane_construct(
        terrane_platform_parse_ip(terrane_platform_result_detail(&raw)),
    );
    let port: terrane_int_support::Int = terrane_platform_result_int(&raw);
    let address: SocketAddress = SocketAddress::terrane_construct(
        raw,
        address_ip,
        port.clone(),
    );
    return SocketResult::terrane_construct(failed, message, address);
}
#[derive(Clone)]
pub struct IoResult {
    pub failed: bool,
    pub truncated: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub peer: String,
    pub end: bool,
}
impl IoResult {
    pub fn terrane_construct(
        failed: bool,
        truncated: bool,
        deadline_exceeded: bool,
        message: String,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        peer: String,
        end: bool,
    ) -> Self {
        let mut value = Self {
            failed: false,
            truncated: false,
            deadline_exceeded: false,
            message: String::from(""),
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            peer: String::from(""),
            end: false,
        };
        value
            .construct(
                failed,
                truncated,
                deadline_exceeded,
                message,
                data,
                completed,
                peer,
                end,
            );
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        truncated: bool,
        deadline_exceeded: bool,
        message: String,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        peer: String,
        end: bool,
    ) {
        self.failed = failed;
        self.truncated = truncated;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.data = data;
        self.completed = completed.clone();
        self.peer = peer;
        self.end = end;
    }
}
pub struct TcpStream {
    pub handle: TerranePlatformCapability,
}
impl TcpStream {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut value = Self { handle: Default::default() };
        value.construct(resource);
        value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = resource;
    }
    pub fn read(
        &self,
        limit: terrane_int_support::Int,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_read(
            &self.handle,
            limit,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
            terrane_platform_result_int(&raw),
            String::from(""),
            terrane_platform_result_bool(&raw),
        );
    }
    pub fn write(&self, data: Vec<u8>, options: NetworkOperationOptions) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_write(
            &self.handle,
            data,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            Vec::from([]),
            terrane_platform_result_int(&raw),
            String::from(""),
            false,
        );
    }
    pub fn configure(&self, options: TcpOptions) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_configure(
            &self.handle,
            options.no_delay,
            options.ttl,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn shutdown(&self, direction: String) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_shutdown(
            &self.handle,
            direction,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn close(&self) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_close(&self.handle);
    }
}
impl Drop for TcpStream {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct StreamResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub peer: String,
    pub value: TcpStream,
}
impl StreamResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        peer: String,
        stream: TcpStream,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            peer: String::from(""),
            value: TcpStream::terrane_construct(terrane_platform_no_resource()),
        };
        value.construct(failed, deadline_exceeded, message, peer, stream);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        peer: String,
        stream: TcpStream,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.peer = peer;
        self.value = stream;
    }
}
pub fn connect_tcp(
    address: SocketAddress,
    options: NetworkOperationOptions,
) -> StreamResult {
    let raw: TerranePlatformResult = terrane_platform_tcp_connect(
        address.value,
        options.deadline_ms,
        &options.cancellation.handle,
    );
    let stream: TcpStream = TcpStream::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return StreamResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        String::from(""),
        stream,
    );
}
pub fn connect_host(
    host: NetworkHostName,
    port: terrane_int_support::Int,
    options: NetworkOperationOptions,
) -> StreamResult {
    let raw: TerranePlatformResult = terrane_platform_tcp_connect_host(
        host.value,
        port,
        options.deadline_ms,
        &options.cancellation.handle,
    );
    let stream: TcpStream = TcpStream::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    return StreamResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_text(&raw),
        stream,
    );
}
pub struct TcpListener {
    pub handle: TerranePlatformCapability,
    pub local_address: String,
}
impl TcpListener {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut value = Self {
            handle: Default::default(),
            local_address: String::from(""),
        };
        value.construct(resource);
        value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = resource;
    }
    pub fn accept(&self, options: NetworkOperationOptions) -> StreamResult {
        let raw: TerranePlatformResult = terrane_platform_tcp_accept(
            &self.handle,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        let stream: TcpStream = TcpStream::terrane_construct(
            terrane_platform_result_capability(&raw),
        );
        return StreamResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_text(&raw),
            stream,
        );
    }
    pub fn close(&self) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_close(&self.handle);
    }
}
impl Drop for TcpListener {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct ListenerResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub value: TcpListener,
}
impl ListenerResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        listener: TcpListener,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            value: TcpListener::terrane_construct(terrane_platform_no_resource()),
        };
        value.construct(failed, deadline_exceeded, message, listener);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        listener: TcpListener,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.value = listener;
    }
}
pub fn bind_tcp(address: SocketAddress) -> ListenerResult {
    let raw: TerranePlatformResult = terrane_platform_tcp_bind(address.value);
    let mut listener: TcpListener = TcpListener::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    if !terrane_platform_result_failed(&raw) {
        listener.local_address = terrane_platform_result_text(&raw);
    }
    return ListenerResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        listener,
    );
}
pub struct UdpSocket {
    pub handle: TerranePlatformCapability,
    pub local_address: String,
}
impl UdpSocket {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut value = Self {
            handle: Default::default(),
            local_address: String::from(""),
        };
        value.construct(resource);
        value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = resource;
    }
    pub fn send_to(
        &self,
        data: Vec<u8>,
        address: SocketAddress,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_udp_send_to(
            &self.handle,
            data,
            address.value,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            false,
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            Vec::from([]),
            terrane_platform_result_int(&raw),
            String::from(""),
            false,
        );
    }
    pub fn receive_from(
        &self,
        limit: terrane_int_support::Int,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw: TerranePlatformResult = terrane_platform_udp_receive_from(
            &self.handle,
            limit,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_truncated(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_bytes(&raw),
            terrane_platform_result_int(&raw),
            terrane_platform_result_text(&raw),
            false,
        );
    }
    pub fn configure(&self, options: UdpOptions) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_udp_configure(
            &self.handle,
            options.broadcast,
            options.ttl,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn close(&self) -> NetworkOperationResult {
        let raw: TerranePlatformResult = terrane_platform_close(&self.handle);
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn destruct(&self) {
        terrane_platform_close(&self.handle);
    }
}
impl Drop for UdpSocket {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub struct UdpResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub value: UdpSocket,
}
impl UdpResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        socket: UdpSocket,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            value: UdpSocket::terrane_construct(terrane_platform_no_resource()),
        };
        value.construct(failed, deadline_exceeded, message, socket);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        socket: UdpSocket,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.value = socket;
    }
}
pub fn bind_udp(address: SocketAddress) -> UdpResult {
    let raw: TerranePlatformResult = terrane_platform_udp_bind(address.value);
    let mut socket: UdpSocket = UdpSocket::terrane_construct(
        terrane_platform_result_capability(&raw),
    );
    if !terrane_platform_result_failed(&raw) {
        socket.local_address = terrane_platform_result_text(&raw);
    }
    return UdpResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        socket,
    );
}
#[derive(Clone)]
pub struct DnsResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub candidates: terrane_collection_support::List<String>,
    pub ttl: terrane_int_support::Int,
    pub ttl_known: bool,
}
impl DnsResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        ttl: terrane_int_support::Int,
        ttl_known: bool,
        candidates: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            candidates: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            ttl: terrane_int_support::Int::from(0_i128),
            ttl_known: false,
        };
        value.construct(failed, deadline_exceeded, message, ttl, ttl_known, candidates);
        value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        ttl: terrane_int_support::Int,
        ttl_known: bool,
        candidates: terrane_collection_support::List<String>,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.ttl = ttl.clone();
        self.ttl_known = ttl_known;
        self.candidates = candidates.clone();
    }
}
#[derive(Clone)]
pub struct NetworkHostName {
    pub value: String,
}
impl NetworkHostName {
    pub fn terrane_construct(raw: TerranePlatformResult) -> Self {
        let mut value = Self { value: String::from("") };
        value.construct(raw);
        value
    }
    pub fn construct(&mut self, raw: TerranePlatformResult) {
        self.value = terrane_platform_result_text(&raw);
    }
}
#[derive(Clone)]
pub struct NetworkHostNameResult {
    pub failed: bool,
    pub message: String,
    pub value: NetworkHostName,
}
impl NetworkHostNameResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        host: NetworkHostName,
    ) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: NetworkHostName::terrane_construct(terrane_platform_failed_result()),
        };
        value.construct(failed, message, host);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, host: NetworkHostName) {
        self.failed = failed;
        self.message = message;
        self.value = host.clone();
    }
}
pub fn parse_host_name(text: String) -> NetworkHostNameResult {
    let raw: TerranePlatformResult = terrane_platform_parse_host_name(text);
    let failed: bool = terrane_platform_result_failed(&raw);
    let message: String = terrane_platform_result_message(&raw);
    let host: NetworkHostName = NetworkHostName::terrane_construct(raw);
    return NetworkHostNameResult::terrane_construct(failed, message, host);
}
pub fn lookup_dns(
    host: NetworkHostName,
    port: terrane_int_support::Int,
    options: NetworkOperationOptions,
) -> DnsResult {
    let raw: TerranePlatformResult = terrane_platform_dns_lookup(
        host.value,
        port,
        options.deadline_ms,
        &options.cancellation.handle,
    );
    let raw_candidates: Vec<String> = terrane_platform_result_entries(&raw);
    let mut candidates: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("")]);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(raw_candidates.len() as i128) {
        candidates
            .append(
                __terrane_raised(
                    raw_candidates
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                1 /* terrane-site: core/networking.trn:319:28-319:49 */,
                            ),
                        )
                        .cloned()
                        .ok_or(terrane_collection_support::IndexError {
                            index: __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                1 /* terrane-site: core/networking.trn:319:28-319:49 */,
                            ),
                        }),
                    1 /* terrane-site: core/networking.trn:319:28-319:49 */,
                ),
            );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return DnsResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
        terrane_platform_result_int(&raw),
        terrane_platform_result_bool(&raw),
        candidates.clone(),
    );
}
