// Generated deterministically by Terrane <version>.
// Runtime support: consuming_callable.rs, async.rs, executor_parallel.rs, async_dependency.rs
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
    structured: Vec<String>,
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
                Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
                Box::new(TerraneErrorDetail {
                    message: None,
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
    pub static FUNCTIONS: [&str; 3] = [
        "/app::socket-task",
        "/app::upgrade-handler",
        "/app::main",
    ];
    pub static SITES: [Site; 18] = [
        /* terrane-site-row: site 0: /app::socket-task (src/main.trn:15:34-15:54) */
        { Site { function: 0, file: 0, line: 15, column: 34, end_line: 15, end_column: 54 } },
        /* terrane-site-row: site 1: /app::socket-task (src/main.trn:17:30-17:52) */
        { Site { function: 0, file: 0, line: 17, column: 30, end_line: 17, end_column: 52 } },
        /* terrane-site-row: site 2: /app::socket-task (src/main.trn:17:16-17:53) */
        { Site { function: 0, file: 0, line: 17, column: 16, end_line: 17, end_column: 53 } },
        /* terrane-site-row: site 3: /app::socket-task (src/main.trn:17:15-17:54) */
        { Site { function: 0, file: 0, line: 17, column: 15, end_line: 17, end_column: 54 } },
        /* terrane-site-row: site 4: /app::socket-task (src/main.trn:18:26-18:40) */
        { Site { function: 0, file: 0, line: 18, column: 26, end_line: 18, end_column: 40 } },
        /* terrane-site-row: site 5: /app::socket-task (src/main.trn:18:12-18:41) */
        { Site { function: 0, file: 0, line: 18, column: 12, end_line: 18, end_column: 41 } },
        /* terrane-site-row: site 6: /app::socket-task (src/main.trn:18:11-18:42) */
        { Site { function: 0, file: 0, line: 18, column: 11, end_line: 18, end_column: 42 } },
        /* terrane-site-row: site 7: /app::upgrade-handler (src/main.trn:21:12-21:46) */
        { Site { function: 1, file: 0, line: 21, column: 12, end_line: 21, end_column: 46 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:24:9-24:24) */
        { Site { function: 2, file: 0, line: 24, column: 9, end_line: 24, end_column: 24 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:25:9-25:31) */
        { Site { function: 2, file: 0, line: 25, column: 9, end_line: 25, end_column: 31 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:26:14-26:26) */
        { Site { function: 2, file: 0, line: 26, column: 14, end_line: 26, end_column: 26 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:27:34-27:45) */
        { Site { function: 2, file: 0, line: 27, column: 34, end_line: 27, end_column: 45 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:27:14-27:46) */
        { Site { function: 2, file: 0, line: 27, column: 14, end_line: 27, end_column: 46 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:28:36-28:56) */
        { Site { function: 2, file: 0, line: 28, column: 36, end_line: 28, end_column: 56 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:28:14-28:57) */
        { Site { function: 2, file: 0, line: 28, column: 14, end_line: 28, end_column: 57 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:29:22-29:54) */
        { Site { function: 2, file: 0, line: 29, column: 22, end_line: 29, end_column: 54 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:31:16-31:56) */
        { Site { function: 2, file: 0, line: 31, column: 16, end_line: 31, end_column: 56 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:31:15-31:57) */
        { Site { function: 2, file: 0, line: 31, column: 15, end_line: 31, end_column: 57 } },
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
async fn health() -> String {
    return String::from("ok");
}
async fn socket_task(mut socket: WebSocket) {
    let received: Option<String> = __terrane_traced(
        __terrane_await({
                let __terrane_future = receive_text(&mut socket);
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:15:34-15:54 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:15:34-15:54 */,
    );
    if received.is_some() {
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut socket)
                            .send(
                                __terrane_raised(
                                    text_message(
                                        received
                                            .as_ref()
                                            .expect("semantic optional narrowing")
                                            .clone(),
                                    ),
                                    1 /* terrane-site: src/main.trn:17:30-17:52 */,
                                ),
                            );
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
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
                                                    "Rust dependency `axum` member `axum::extract::ws::WebSocket::send` failed: {error}"
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
                                            "axum",
                                            "axum::extract::ws::WebSocket::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            2 /* terrane-site: src/main.trn:17:16-17:53 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:17:15-17:54 */,
        );
    }
    __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut socket)
                        .send(
                            __terrane_raised(
                                close_message(),
                                4 /* terrane-site: src/main.trn:18:26-18:40 */,
                            ),
                        );
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
                                                "Rust dependency `axum` member `axum::extract::ws::WebSocket::send` failed: {error}"
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
                                        "axum",
                                        "axum::extract::ws::WebSocket::send",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        5 /* terrane-site: src/main.trn:18:12-18:41 */,
                    )
                }
            })
            .await,
        6 /* terrane-site: src/main.trn:18:11-18:42 */,
    );
}
async fn upgrade_handler(request: WebSocketUpgrade) -> UpgradeResponse {
    return __terrane_raised(
        upgrade(
            request,
            TerraneConsumingCallable::new(move |
                (argument_0,): (WebSocket,),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                Box::pin(socket_task(argument_0))
            }),
        ),
        7 /* terrane-site: src/main.trn:21:12-21:46 */,
    );
}
fn main() {
    __terrane_run(async move {
        {
            let _: Router = __terrane_raised(
                default_terrane_deps_axum_router(),
                8 /* terrane-site: src/main.trn:24:9-24:24 */,
            );
        }
        {
            let _: MethodRouter = __terrane_raised(
                default_terrane_deps_axum_routing_methodrouter(),
                9 /* terrane-site: src/main.trn:25:9-25:31 */,
            );
        }
        let mut router: Router = __terrane_raised(
            terrane_static_trn_526f75746572_new(),
            10 /* terrane-site: src/main.trn:26:14-26:26 */,
        );
        router = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    router
                        .route(
                            &String::from("/"),
                            __terrane_raised(
                                match std::panic::catch_unwind(|| axum::routing::get(
                                    match || -> Result<_, crate::TerraneForeignError> {
                                        Ok({
                                            let callback = std::sync::Arc::new(move || -> std::pin::Pin<
                                                    Box<dyn Future<Output = _> + Send>,
                                                > { Box::pin(health()) })
                                                .clone();
                                            move || {
                                                let callback = callback.clone();
                                                let callback_future = callback();
                                                Box::pin(async move {
                                                    match async {
                                                        let callback_value = callback_future.await;
                                                        Ok::<_, crate::TerraneForeignError>(callback_value)
                                                    }
                                                        .await
                                                    {
                                                        Ok(value) => value,
                                                        Err(error) => std::panic::panic_any(error.0),
                                                    }
                                                })
                                            }
                                        })
                                    }() {
                                        Ok(value) => value,
                                        Err(error) => std::panic::panic_any(error),
                                    },
                                )) {
                                    Ok(value) => Ok(value),
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "axum",
                                                "axum::routing::get",
                                            ),
                                        )
                                    }
                                },
                                11 /* terrane-site: src/main.trn:27:34-27:45 */,
                            ),
                        )
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "axum",
                            "axum::Router<()>::route",
                        ),
                    )
                }
            },
            12 /* terrane-site: src/main.trn:27:14-27:46 */,
        );
        router = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    router
                        .route(
                            &String::from("/ws"),
                            __terrane_raised(
                                match std::panic::catch_unwind(|| axum::routing::get(
                                    match || -> Result<_, crate::TerraneForeignError> {
                                        Ok({
                                            let callback = std::sync::Arc::new(move |
                                                    argument_0: WebSocketUpgrade,
                                                | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                                                    Box::pin(upgrade_handler(argument_0))
                                                })
                                                .clone();
                                            move |
                                                callback_argument_0: axum::extract::WebSocketUpgrade<
                                                    axum::extract::ws::DefaultOnFailedUpgrade,
                                                >|
                                            {
                                                let callback = callback.clone();
                                                let callback_future = callback(callback_argument_0);
                                                Box::pin(async move {
                                                    match async {
                                                        let callback_value = callback_future.await;
                                                        Ok::<_, crate::TerraneForeignError>(callback_value)
                                                    }
                                                        .await
                                                    {
                                                        Ok(value) => value,
                                                        Err(error) => std::panic::panic_any(error.0),
                                                    }
                                                })
                                            }
                                        })
                                    }() {
                                        Ok(value) => value,
                                        Err(error) => std::panic::panic_any(error),
                                    },
                                )) {
                                    Ok(value) => Ok(value),
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "axum",
                                                "axum::routing::get",
                                            ),
                                        )
                                    }
                                },
                                13 /* terrane-site: src/main.trn:28:36-28:56 */,
                            ),
                        )
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "axum",
                            "axum::Router<()>::route",
                        ),
                    )
                }
            },
            14 /* terrane-site: src/main.trn:28:14-28:57 */,
        );
        let listener: TcpListener = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = tokio::net::TcpListener::bind(
                            String::from("127.0.0.1:0"),
                        );
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
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
                                                    "Rust dependency `tokio` member `tokio::net::TcpListener::bind` failed: {error}"
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
                                            "tokio",
                                            "tokio::net::TcpListener::bind",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            15 /* terrane-site: src/main.trn:29:22-29:54 */,
                        )
                    }
                })
                .await,
            15 /* terrane-site: src/main.trn:29:22-29:54 */,
        );
        if false {
            __terrane_traced(
                __terrane_await({
                        let __terrane_future = serve_router(listener, router);
                        async move {
                            __terrane_raised_err(
                                __terrane_future.await,
                                16 /* terrane-site: src/main.trn:31:16-31:56 */,
                            )
                        }
                    })
                    .await,
                17 /* terrane-site: src/main.trn:31:15-31:57 */,
            );
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("axum route and serve built"))
        );
    });
}
// Source: <terrane>/projected/deps/axum-core.trn
// Namespace: deps/axum-core
// Source: <terrane>/projected/deps/axum/extract/ws.trn
// Namespace: deps/axum/extract/ws
// Source: <terrane>/projected/deps/axum/extract.trn
// Namespace: deps/axum/extract
// Source: <terrane>/projected/deps/axum/routing.trn
// Namespace: deps/axum/routing
// Source: <terrane>/projected/deps/axum/routing/methodrouter.trn
// Namespace: deps/axum/routing/methodrouter
pub fn default_terrane_deps_axum_routing_methodrouter() -> Result<
    MethodRouter,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| <axum::routing::MethodRouter<
            (),
            core::convert::Infallible,
        > as core::default::Default>::default()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "axum",
                    "<axum::routing::MethodRouter<(), core::convert::Infallible> as core::default::Default>::default",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/tokio/net.trn
// Namespace: deps/tokio/net
// Source: <terrane>/projected/deps/axum.trn
// Namespace: deps/axum
pub use axum::extract::ws::Message;
pub use axum::routing::MethodFilter;
pub type MethodRouter = axum::routing::MethodRouter<(), core::convert::Infallible>;
pub type Router = axum::Router<()>;
pub use tokio::net::TcpListener;
pub type TerraneNs4Deps4Axum7Extract2WsResult48b72fbedaf851fdef46b20b67ede78941e7b9cf3a599d6dd753ebe1d0b6e4ef = core::result::Result<
    axum::extract::ws::Message,
    axum_core::Error,
>;
pub use axum_core::response::Response as TerraneNs4Deps4Axum7ExtractResponse;
pub type TerraneNs4Deps4Axum7ExtractWebSocketUpgrade3c883fea21bc271e02314eb477909d23ae17ecfc78e11684ad67ddf0d2a9f675<
    C,
> = axum::extract::WebSocketUpgrade<C>;
pub type TerraneNs4Deps4Axum7RoutingIntoMakeService04440a4688860a013cb956808ecd2de46b3b628f08e87c711c357a732690acad = axum::routing::IntoMakeService<
    axum::routing::MethodRouter<(), core::convert::Infallible>,
>;
pub type TerraneNs4Deps4AxumIntoMakeServiceBb12f2fd8937780447c05175fc482b296d3e133d3a5b1be5db78da50a6399572 = axum::routing::IntoMakeService<
    axum::Router<()>,
>;
pub type TerraneNs4Deps4AxumMethodRouter78b1dfd45684b6ec3178aadb25929757adf127d60a8104a15dc868775202e5f7 = axum::routing::MethodRouter<
    (),
>;
pub use terrane_integration_adapters::axum_08::UpgradeResponse;
pub use axum::extract::ws::WebSocket;
pub type WebSocketUpgrade = axum::extract::WebSocketUpgrade<
    axum::extract::ws::DefaultOnFailedUpgrade,
>;
pub fn terrane_static_trn_526f75746572_new() -> Result<
    Router,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| axum::Router::<()>::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "axum", "axum::Router<()>"))
        }
    }
}
pub fn close_message() -> Result<Message, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_integration_adapters::axum_08::close_message()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-integration-adapters",
                    "terrane_integration_adapters::axum_08::close_message",
                ),
            )
        }
    }
}
pub async fn receive_text(
    socket: &mut WebSocket,
) -> Result<Option<String>, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            terrane_integration_adapters::axum_08::receive_text(socket),
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
                            "Rust dependency `terrane-integration-adapters` member `terrane_integration_adapters::axum_08::receive_text` failed: {error}"
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
                    "terrane-integration-adapters",
                    "terrane_integration_adapters::axum_08::receive_text",
                ),
            )
        }
    }
}
pub async fn serve_router(
    listener: TcpListener,
    router: Router,
) -> Result<(), crate::TerraneForeignError> {
    let listener = listener;
    let router = router;
    match crate::__terrane_dependency_await_unwind(
            terrane_integration_adapters::axum_08::serve_router(listener, router),
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
                            "Rust dependency `terrane-integration-adapters` member `terrane_integration_adapters::axum_08::serve_router` failed: {error}"
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
                    "terrane-integration-adapters",
                    "terrane_integration_adapters::axum_08::serve_router",
                ),
            )
        }
    }
}
pub fn text_message(text: String) -> Result<Message, crate::TerraneForeignError> {
    let text = text;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_integration_adapters::axum_08::text_message(
            text,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-integration-adapters",
                    "terrane_integration_adapters::axum_08::text_message",
                ),
            )
        }
    }
}
pub fn upgrade(
    request: WebSocketUpgrade,
    callback: TerraneConsumingCallable<
        (WebSocket,),
        std::pin::Pin<Box<dyn Future<Output = ()> + Send>>,
    >,
) -> Result<UpgradeResponse, crate::TerraneForeignError> {
    let request = request;
    let callback = {
        let callback = callback;
        move |callback_argument_0: axum::extract::ws::WebSocket| {
            let callback_future = callback.call((callback_argument_0,));
            Box::pin(async move {
                match async {
                    let callback_value = callback_future.await;
                    Ok::<_, crate::TerraneForeignError>(callback_value)
                }
                    .await
                {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error.0),
                }
            })
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_integration_adapters::axum_08::upgrade(
            request,
            callback,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-integration-adapters",
                    "terrane_integration_adapters::axum_08::upgrade",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/axum/router.trn
// Namespace: deps/axum/router
pub fn default_terrane_deps_axum_router() -> Result<Router, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| <axum::Router<
            (),
        > as core::default::Default>::default()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "axum",
                    "<axum::Router<()> as core::default::Default>::default",
                ),
            )
        }
    }
}
