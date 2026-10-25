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
    ReplaceError,
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
            Self::ReplaceError => "replace-error",
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
            Self::ReplaceError => {
                "replacement search and replacement lists have different lengths"
            }
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
    rendered_message: Option<String>,
    cause: Option<std::boxed::Box<TerraneError>>,
    frames: Vec<TerraneSite>,
    structured: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    origin: TerraneSite,
    detail: Option<std::boxed::Box<TerraneErrorDetail>>,
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                }),
            ),
        }
    }
    #[cold]
    #[inline(never)]
    fn raised_with_rendered_message(
        kind: TerraneErrorKind,
        message: impl Into<String>,
        rendered_message: Option<String>,
        origin: TerraneSite,
    ) -> Self {
        Self {
            kind,
            origin,
            detail: Some(
                std::boxed::Box::new(TerraneErrorDetail {
                    message: Some(message.into()),
                    rendered_message,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                })
            })
            .cause = Some(std::boxed::Box::new(cause));
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
    fn rendered_message(&self) -> &str {
        self.detail
            .as_ref()
            .and_then(|detail| detail.rendered_message.as_deref())
            .unwrap_or_else(|| self.message())
    }
    fn descriptor_name(&self) -> &str {
        self.kind.display_name()
    }
    fn source_frames(&self) -> Vec<String> {
        let mut frames = Vec::new();
        if self.origin != TERRANE_NO_SITE {
            frames.push(__terrane_trace::render(self.origin));
        }
        if let Some(detail) = &self.detail {
            frames
                .extend(
                    detail.frames.iter().map(|frame| __terrane_trace::render(*frame)),
                );
        }
        frames
    }
    fn with_structured_details(mut self, structured: Vec<String>) -> Self {
        self
            .detail
            .get_or_insert_with(|| {
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
                    rendered_message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
                })
            })
            .structured = structured;
        self
    }
    fn structured_details(&self) -> &[String] {
        self.detail.as_deref().map_or(&[], |detail| detail.structured.as_slice())
    }
    #[cold]
    #[inline(never)]
    fn render(&self) -> String {
        let mut rendered = format!(
            "{}: {}", self.kind.display_name(), self.rendered_message()
        );
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
impl TerraneRaised for terrane_string_support::ReplaceError {
    fn raised(self, origin: TerraneSite) -> TerraneError {
        TerraneError::raised_with_message(
            TerraneErrorKind::ReplaceError,
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
    payload: std::boxed::Box<dyn std::any::Any + Send>,
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
    pub static DESCRIPTORS: [&str; 2] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
    ];
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
    pub static FUNCTIONS: [&str; 2] = ["/app::bump", "/app::main"];
    pub static SITES: [Site; 19] = [
        /* terrane-site-row: site 0: /app::bump (src/main.trn:8:22-8:36) */
        { Site { function: 0, file: 0, line: 8, column: 22, end_line: 8, end_column: 36 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:16:20-16:42) */
        { Site { function: 1, file: 0, line: 16, column: 20, end_line: 16, end_column: 42 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:18:17-18:36) */
        { Site { function: 1, file: 0, line: 18, column: 17, end_line: 18, end_column: 36 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:19:13-19:23) */
        { Site { function: 1, file: 0, line: 19, column: 13, end_line: 19, end_column: 23 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:20:13-20:23) */
        { Site { function: 1, file: 0, line: 20, column: 13, end_line: 20, end_column: 23 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:22:47-22:67) */
        { Site { function: 1, file: 0, line: 22, column: 47, end_line: 22, end_column: 67 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:24:22-24:44) */
        { Site { function: 1, file: 0, line: 24, column: 22, end_line: 24, end_column: 44 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:24:16-24:44) */
        { Site { function: 1, file: 0, line: 24, column: 16, end_line: 24, end_column: 44 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:25:25-25:41) */
        { Site { function: 1, file: 0, line: 25, column: 25, end_line: 25, end_column: 41 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:28:21-28:41) */
        { Site { function: 1, file: 0, line: 28, column: 21, end_line: 28, end_column: 41 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:32:32-32:52) */
        { Site { function: 1, file: 0, line: 32, column: 32, end_line: 32, end_column: 52 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:32:16-32:53) */
        { Site { function: 1, file: 0, line: 32, column: 16, end_line: 32, end_column: 53 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:33:13-33:19) */
        { Site { function: 1, file: 0, line: 33, column: 13, end_line: 33, end_column: 19 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:34:13-34:19) */
        { Site { function: 1, file: 0, line: 34, column: 13, end_line: 34, end_column: 19 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:36:24-36:46) */
        { Site { function: 1, file: 0, line: 36, column: 24, end_line: 36, end_column: 46 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:40:13-40:36) */
        { Site { function: 1, file: 0, line: 40, column: 13, end_line: 40, end_column: 36 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:42:13-42:35) */
        { Site { function: 1, file: 0, line: 42, column: 13, end_line: 42, end_column: 35 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:45:17-45:29) */
        { Site { function: 1, file: 0, line: 45, column: 17, end_line: 45, end_column: 29 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:46:13-46:21) */
        { Site { function: 1, file: 0, line: 46, column: 13, end_line: 46, end_column: 21 } },
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
#[allow(
    dead_code,
    reason = "async entry support is emitted for libraries without an entrypoint"
)]
async fn __terrane_async_entry<F: Future>(future: F) -> F::Output {
    future.await
}
#[allow(
    dead_code,
    reason = "shared async support also serves packages without awaited destructors"
)]
const TERRANE_RUNTIME_CLOSING: usize = 1 << usize::BITS - 1;
#[allow(
    dead_code,
    reason = "libraries can use async support without owning an executor"
)]
struct TerraneRuntimeLiveness {
    active: std::sync::atomic::AtomicUsize,
    drain: std::sync::Mutex<()>,
    changed: std::sync::Condvar,
}
thread_local! {
    static TERRANE_RUNTIME_LIVE : std::cell::RefCell < Option < (tokio::runtime::Id,
    std::sync::Arc < TerraneRuntimeLiveness >) >> = const { std::cell::RefCell::new(None)
    };
}
#[allow(
    dead_code,
    reason = "libraries can use awaited destruction without owning an executor"
)]
struct TerraneRuntimeLivenessGuard {
    live: std::sync::Arc<TerraneRuntimeLiveness>,
    previous: Option<(tokio::runtime::Id, std::sync::Arc<TerraneRuntimeLiveness>)>,
}
impl Drop for TerraneRuntimeLivenessGuard {
    fn drop(&mut self) {
        if self
            .live
            .active
            .fetch_or(TERRANE_RUNTIME_CLOSING, std::sync::atomic::Ordering::AcqRel) != 0
        {
            let drained = self
                .live
                .drain
                .lock()
                .expect("destructor drain lock poisoned");
            let _drained = self
                .live
                .changed
                .wait_while(
                    drained,
                    |()| {
                        self.live.active.load(std::sync::atomic::Ordering::Acquire)
                            != TERRANE_RUNTIME_CLOSING
                    },
                )
                .expect("destructor drain lock poisoned");
        }
        TERRANE_RUNTIME_LIVE.with(|live| *live.borrow_mut() = self.previous.take());
    }
}
#[allow(
    dead_code,
    reason = "libraries can use awaited destruction without owning an executor"
)]
fn __terrane_runtime_liveness_enter(
    runtime: &tokio::runtime::Runtime,
    live: std::sync::Arc<TerraneRuntimeLiveness>,
) -> TerraneRuntimeLivenessGuard {
    let previous = TERRANE_RUNTIME_LIVE
        .with(|current| {
            current.borrow_mut().replace((runtime.handle().id(), live.clone()))
        });
    TerraneRuntimeLivenessGuard {
        live,
        previous,
    }
}
#[allow(
    dead_code,
    reason = "shared async support also serves packages without awaited destructors"
)]
struct TerraneDestructorPermit(std::sync::Arc<TerraneRuntimeLiveness>);
impl Drop for TerraneDestructorPermit {
    fn drop(&mut self) {
        if self.0.active.fetch_sub(1, std::sync::atomic::Ordering::AcqRel)
            == TERRANE_RUNTIME_CLOSING + 1
        {
            let _drained = self.0.drain.lock().expect("destructor drain lock poisoned");
            self.0.changed.notify_one();
        }
    }
}
fn __terrane_await_destructor<F>(future: F)
where
    F: Future<Output = ()> + Send,
{
    let permit = tokio::runtime::Handle::try_current()
        .ok()
        .and_then(|handle| {
            if handle.runtime_flavor() != tokio::runtime::RuntimeFlavor::MultiThread {
                return None;
            }
            TERRANE_RUNTIME_LIVE
                .with(|current| {
                    let current = current.borrow();
                    let (id, live) = current.as_ref()?;
                    if *id != handle.id() {
                        return None;
                    }
                    live.active
                        .try_update(
                            std::sync::atomic::Ordering::AcqRel,
                            std::sync::atomic::Ordering::Acquire,
                            |active| {
                                (active < TERRANE_RUNTIME_CLOSING - 1).then(|| active + 1)
                            },
                        )
                        .ok()?;
                    Some((handle, TerraneDestructorPermit(live.clone())))
                })
        });
    if let Some((handle, _permit)) = permit {
        tokio::task::block_in_place(|| handle.block_on(future));
        return;
    }
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Terrane destructor runtime must initialize")
                    .block_on(future);
            })
            .join()
            .expect("Terrane awaited destructor must not panic");
    });
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize");
    let future = Box::pin(future);
    tokio::task::LocalSet::new()
        .block_on(
            &runtime,
            async move {
                let output = __terrane_async_entry(future).await;
                __terrane_wait_projected_cleanups().await;
                output
            },
        )
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
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub use witness::Counter;
pub use witness::OpaqueValue;
pub fn terrane_static_trn_436f756e746572_new(
    value: terrane_int_support::Int,
) -> Result<Counter, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::Counter::new(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(payload, "witness", "witness::Counter"),
            )
        }
    }
}
pub fn terrane_static_trn_4f706171756556616c7565_new(
    value: terrane_int_support::Int,
) -> Result<OpaqueValue, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::OpaqueValue::new(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::OpaqueValue",
                ),
            )
        }
    }
}
pub async fn checksum(
    values: terrane_collection_support::List<OpaqueValue>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let values = values.into_vec();
    match crate::__terrane_dependency_await_unwind(witness::checksum(&values)).await {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::checksum",
                ),
            )
        }
    }
}
