// Generated deterministically by Terrane <version>.
// Runtime support: consuming_callable.rs, async_native.rs, executor_parallel.rs, async_dependency.rs, tasks_native_parallel.rs, time_inactive.rs
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
    pub static DESCRIPTORS: [&str; 3] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
        "/deps/axum-core::Error",
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
    pub static FUNCTIONS: [&str; 4] = [
        "/app::socket-session",
        "/app::upgrade-handler",
        "/app::run-server",
        "/app::main",
    ];
    pub static SITES: [Site; 28] = [
        /* terrane-site-row: site 0: /app::socket-session (src/main.trn:17:36-17:48) */
        { Site { function: 0, file: 0, line: 17, column: 36, end_line: 17, end_column: 48 } },
        /* terrane-site-row: site 1: /app::socket-session (src/main.trn:19:23-19:44) */
        { Site { function: 0, file: 0, line: 19, column: 23, end_line: 19, end_column: 44 } },
        /* terrane-site-row: site 2: /app::socket-session (src/main.trn:21:31-21:56) */
        { Site { function: 0, file: 0, line: 21, column: 31, end_line: 21, end_column: 56 } },
        /* terrane-site-row: site 3: /app::socket-session (src/main.trn:25:36-25:56) */
        { Site { function: 0, file: 0, line: 25, column: 36, end_line: 25, end_column: 56 } },
        /* terrane-site-row: site 4: /app::socket-session (src/main.trn:25:22-25:57) */
        { Site { function: 0, file: 0, line: 25, column: 22, end_line: 25, end_column: 57 } },
        /* terrane-site-row: site 5: /app::socket-session (src/main.trn:25:21-25:58) */
        { Site { function: 0, file: 0, line: 25, column: 21, end_line: 25, end_column: 58 } },
        /* terrane-site-row: site 6: /app::socket-session (src/main.trn:28:34-28:54) */
        { Site { function: 0, file: 0, line: 28, column: 34, end_line: 28, end_column: 54 } },
        /* terrane-site-row: site 7: /app::socket-session (src/main.trn:28:20-28:55) */
        { Site { function: 0, file: 0, line: 28, column: 20, end_line: 28, end_column: 55 } },
        /* terrane-site-row: site 8: /app::socket-session (src/main.trn:28:19-28:56) */
        { Site { function: 0, file: 0, line: 28, column: 19, end_line: 28, end_column: 56 } },
        /* terrane-site-row: site 9: /app::socket-session (src/main.trn:30:30-30:57) */
        { Site { function: 0, file: 0, line: 30, column: 30, end_line: 30, end_column: 57 } },
        /* terrane-site-row: site 10: /app::socket-session (src/main.trn:33:34-33:56) */
        { Site { function: 0, file: 0, line: 33, column: 34, end_line: 33, end_column: 56 } },
        /* terrane-site-row: site 11: /app::socket-session (src/main.trn:33:20-33:57) */
        { Site { function: 0, file: 0, line: 33, column: 20, end_line: 33, end_column: 57 } },
        /* terrane-site-row: site 12: /app::socket-session (src/main.trn:33:19-33:58) */
        { Site { function: 0, file: 0, line: 33, column: 19, end_line: 33, end_column: 58 } },
        /* terrane-site-row: site 13: /app::socket-session (src/main.trn:35:30-35:55) */
        { Site { function: 0, file: 0, line: 35, column: 30, end_line: 35, end_column: 55 } },
        /* terrane-site-row: site 14: /app::socket-session (src/main.trn:38:34-38:54) */
        { Site { function: 0, file: 0, line: 38, column: 34, end_line: 38, end_column: 54 } },
        /* terrane-site-row: site 15: /app::socket-session (src/main.trn:38:20-38:55) */
        { Site { function: 0, file: 0, line: 38, column: 20, end_line: 38, end_column: 55 } },
        /* terrane-site-row: site 16: /app::socket-session (src/main.trn:38:19-38:56) */
        { Site { function: 0, file: 0, line: 38, column: 19, end_line: 38, end_column: 56 } },
        /* terrane-site-row: site 17: /app::socket-session (src/main.trn:40:30-40:55) */
        { Site { function: 0, file: 0, line: 40, column: 30, end_line: 40, end_column: 55 } },
        /* terrane-site-row: site 18: /app::upgrade-handler (src/main.trn:55:10-55:44) */
        { Site { function: 1, file: 0, line: 55, column: 10, end_line: 55, end_column: 44 } },
        /* terrane-site-row: site 19: /app::run-server (src/main.trn:58:12-58:24) */
        { Site { function: 2, file: 0, line: 58, column: 12, end_line: 58, end_column: 24 } },
        /* terrane-site-row: site 20: /app::run-server (src/main.trn:59:38-59:49) */
        { Site { function: 2, file: 0, line: 59, column: 38, end_line: 59, end_column: 49 } },
        /* terrane-site-row: site 21: /app::run-server (src/main.trn:59:12-59:50) */
        { Site { function: 2, file: 0, line: 59, column: 12, end_line: 59, end_column: 50 } },
        /* terrane-site-row: site 22: /app::run-server (src/main.trn:60:34-60:54) */
        { Site { function: 2, file: 0, line: 60, column: 34, end_line: 60, end_column: 54 } },
        /* terrane-site-row: site 23: /app::run-server (src/main.trn:60:12-60:55) */
        { Site { function: 2, file: 0, line: 60, column: 12, end_line: 60, end_column: 55 } },
        /* terrane-site-row: site 24: /app::run-server (src/main.trn:61:20-61:56) */
        { Site { function: 2, file: 0, line: 61, column: 20, end_line: 61, end_column: 56 } },
        /* terrane-site-row: site 25: /app::run-server (src/main.trn:62:10-62:43) */
        { Site { function: 2, file: 0, line: 62, column: 10, end_line: 62, end_column: 43 } },
        /* terrane-site-row: site 26: /app::run-server (src/main.trn:62:9-62:44) */
        { Site { function: 2, file: 0, line: 62, column: 9, end_line: 62, end_column: 44 } },
        /* terrane-site-row: site 27: /app::main (src/main.trn:67:9-67:20) */
        { Site { function: 3, file: 0, line: 67, column: 9, end_line: 67, end_column: 20 } },
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
async fn socket_session(mut socket: WebSocket) {
    let __terrane_completion_0: TerraneCompletion<()> = async {
        let __terrane_try_0: TerraneCompletion<()> = async {
            let open: bool = true;
            while open {
                let message: Option<Message> = __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call = (&mut
                    socket).recv(); async move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await { Ok(None)
                    => Ok(None), Ok(Some(Ok(value))) => Ok(Some(value)),
                    Ok(Some(Err(error))) => Err(crate ::TerraneForeignError(crate
                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                    .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) => Err(crate
                    ::__terrane_dependency_panic(payload, "axum",
                    "axum::extract::ws::WebSocket::recv")) } } }; async move {
                    __terrane_raised_err(__terrane_future. await, 0 /* terrane-site: src/main.trn:17:36-17:48 */) } }). await, 0 /* terrane-site: src/main.trn:17:36-17:48 */
                );
                if message.is_some() {
                    let kind: String = __terrane_raised_completion!(
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        match &message.as_ref().expect("semantic optional narrowing")
                        .clone() { axum::extract::ws::Message::Text { .. } => "Text"
                        .to_owned(), axum::extract::ws::Message::Binary { .. } =>
                        "Binary".to_owned(), axum::extract::ws::Message::Ping { .. } =>
                        "Ping".to_owned(), axum::extract::ws::Message::Pong { .. } =>
                        "Pong".to_owned(), axum::extract::ws::Message::Close { .. } =>
                        "Close".to_owned() })) { Ok(value) => Ok(value), Err(payload) =>
                        Err(crate ::__terrane_dependency_panic(payload, "axum",
                        "axum::extract::ws::Message::variant-name")) },
                        1 /* terrane-site: src/main.trn:19:23-19:44 */
                    );
                    if kind == String::from("Text") {
                        let value: Option<String> = __terrane_raised_completion!(
                            match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                            match message.expect("semantic optional narrowing") {
                            axum::extract::ws::Message::Text(value) => Some(<
                            axum::extract::ws::Utf8Bytes as std::ops::Deref
                            >::deref(&value).to_owned()), _ => None })) { Ok(value) =>
                            Ok(value), Err(payload) => Err(crate
                            ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::Message::into-Text")) },
                            2 /* terrane-site: src/main.trn:21:31-21:56 */
                        );
                        if value.is_some() {
                            if value
                                .as_ref()
                                .expect("semantic optional narrowing")
                                .clone() == String::from("shutdown")
                            {
                                println!(
                                    "{}",
                                    terrane_scalar_support::scalar_text(&String::from("close-sent"))
                                );
                                __terrane_traced_completion!(
                                    __terrane_await({ let __terrane_future = { let
                                    __terrane_call = (&mut socket)
                                    .send(__terrane_raised_completion!(match
                                    std::panic::catch_unwind(||
                                    axum::extract::ws::Message::Close(None)) { Ok(value) =>
                                    Ok(value), Err(payload) => Err(crate
                                    ::__terrane_dependency_panic(payload, "axum",
                                    "axum::extract::ws::Message::Close")) },
                                    3 /* terrane-site: src/main.trn:25:36-25:56 */)); async
                                    move { match crate
                                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                    ::TerraneForeignError(crate
                                    ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                    .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                    Err(crate ::__terrane_dependency_panic(payload, "axum",
                                    "axum::extract::ws::WebSocket::send")) } } }; async move {
                                    __terrane_raised_err(__terrane_future. await,
                                    4 /* terrane-site: src/main.trn:25:22-25:57 */) } }).
                                    await, 5 /* terrane-site: src/main.trn:25:21-25:58 */
                                );
                                return TerraneCompletion::Return(());
                            }
                            println!(
                                "{}{}",
                                terrane_scalar_support::scalar_text(&String::from("text")),
                                terrane_scalar_support::scalar_text(&value.as_ref()
                                .expect("semantic optional narrowing").clone())
                            );
                            __terrane_traced_completion!(
                                __terrane_await({ let __terrane_future = { let
                                __terrane_call = (&mut socket)
                                .send(__terrane_raised_completion!(match
                                std::panic::catch_unwind(| |
                                axum::extract::ws::Message::Text(value.as_ref()
                                .expect("semantic optional narrowing").clone().into())) {
                                Ok(value) => Ok(value), Err(payload) => Err(crate
                                ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::Message::Text")) },
                                6 /* terrane-site: src/main.trn:28:34-28:54 */)); async
                                move { match crate
                                ::__terrane_dependency_await_unwind(__terrane_call). await {
                                Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                ::TerraneForeignError(crate
                                ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                Err(crate ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::WebSocket::send")) } } }; async move {
                                __terrane_raised_err(__terrane_future. await,
                                7 /* terrane-site: src/main.trn:28:20-28:55 */) } }).
                                await, 8 /* terrane-site: src/main.trn:28:19-28:56 */
                            );
                        }
                    } else if kind == String::from("Binary") {
                        let value: Option<Vec<u8>> = __terrane_raised_completion!(
                            match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                            match message.expect("semantic optional narrowing") {
                            axum::extract::ws::Message::Binary(value) => Some(<
                            bytes::Bytes as AsRef < [u8] >>::as_ref(&value).to_vec()), _
                            => None })) { Ok(value) => Ok(value), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::Message::into-Binary")) },
                            9 /* terrane-site: src/main.trn:30:30-30:57 */
                        );
                        if value.is_some() {
                            println!(
                                "{}{}",
                                terrane_scalar_support::scalar_text(&String::from("binary")),
                                terrane_scalar_support::scalar_text(&(value.as_ref()
                                .expect("semantic optional narrowing").clone().len() as
                                i128))
                            );
                            __terrane_traced_completion!(
                                __terrane_await({ let __terrane_future = { let
                                __terrane_call = (&mut socket)
                                .send(__terrane_raised_completion!(match
                                std::panic::catch_unwind(| |
                                axum::extract::ws::Message::Binary(value.as_ref()
                                .expect("semantic optional narrowing").clone().into())) {
                                Ok(value) => Ok(value), Err(payload) => Err(crate
                                ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::Message::Binary")) },
                                10 /* terrane-site: src/main.trn:33:34-33:56 */)); async
                                move { match crate
                                ::__terrane_dependency_await_unwind(__terrane_call). await {
                                Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                ::TerraneForeignError(crate
                                ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                Err(crate ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::WebSocket::send")) } } }; async move {
                                __terrane_raised_err(__terrane_future. await,
                                11 /* terrane-site: src/main.trn:33:20-33:57 */) } }).
                                await, 12 /* terrane-site: src/main.trn:33:19-33:58 */
                            );
                        }
                    } else if kind == String::from("Ping") {
                        let value: Option<Vec<u8>> = __terrane_raised_completion!(
                            match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                            match message.expect("semantic optional narrowing") {
                            axum::extract::ws::Message::Ping(value) => Some(<
                            bytes::Bytes as AsRef < [u8] >>::as_ref(&value).to_vec()), _
                            => None })) { Ok(value) => Ok(value), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::Message::into-Ping")) },
                            13 /* terrane-site: src/main.trn:35:30-35:55 */
                        );
                        if value.is_some() {
                            println!(
                                "{}{}",
                                terrane_scalar_support::scalar_text(&String::from("ping")),
                                terrane_scalar_support::scalar_text(&(value.as_ref()
                                .expect("semantic optional narrowing").clone().len() as
                                i128))
                            );
                            __terrane_traced_completion!(
                                __terrane_await({ let __terrane_future = { let
                                __terrane_call = (&mut socket)
                                .send(__terrane_raised_completion!(match
                                std::panic::catch_unwind(| |
                                axum::extract::ws::Message::Pong(value.as_ref()
                                .expect("semantic optional narrowing").clone().into())) {
                                Ok(value) => Ok(value), Err(payload) => Err(crate
                                ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::Message::Pong")) },
                                14 /* terrane-site: src/main.trn:38:34-38:54 */)); async
                                move { match crate
                                ::__terrane_dependency_await_unwind(__terrane_call). await {
                                Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                                ::TerraneForeignError(crate
                                ::TerraneError::custom_raised(crate ::DescriptorId(2), error
                                .to_string(), crate ::TERRANE_NO_SITE))), Err(payload) =>
                                Err(crate ::__terrane_dependency_panic(payload, "axum",
                                "axum::extract::ws::WebSocket::send")) } } }; async move {
                                __terrane_raised_err(__terrane_future. await,
                                15 /* terrane-site: src/main.trn:38:20-38:55 */) } }).
                                await, 16 /* terrane-site: src/main.trn:38:19-38:56 */
                            );
                        }
                    } else if kind == String::from("Pong") {
                        let value: Option<Vec<u8>> = __terrane_raised_completion!(
                            match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                            match message.expect("semantic optional narrowing") {
                            axum::extract::ws::Message::Pong(value) => Some(<
                            bytes::Bytes as AsRef < [u8] >>::as_ref(&value).to_vec()), _
                            => None })) { Ok(value) => Ok(value), Err(payload) =>
                            Err(crate ::__terrane_dependency_panic(payload, "axum",
                            "axum::extract::ws::Message::into-Pong")) },
                            17 /* terrane-site: src/main.trn:40:30-40:55 */
                        );
                        if value.is_some() {
                            println!(
                                "{}{}",
                                terrane_scalar_support::scalar_text(&String::from("pong")),
                                terrane_scalar_support::scalar_text(&(value.as_ref()
                                .expect("semantic optional narrowing").clone().len() as
                                i128))
                            );
                        }
                    } else if kind == String::from("Close") {
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("close"))
                        );
                    }
                } else {
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&String::from("end"))
                    );
                    return TerraneCompletion::Return(());
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("failure"))
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
}
async fn health() -> String {
    return String::from("ok");
}
async fn upgrade_handler(request: WebSocketUpgrade) -> Response {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                request
                    .on_upgrade(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok({
                                let callback = TerraneConsumingCallable::new(move |
                                    (argument_0,): (WebSocket,),
                                | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                                    Box::pin(socket_session(argument_0))
                                });
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
                            })
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    )
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "axum",
                        "axum::extract::WebSocketUpgrade<axum::extract::ws::DefaultOnFailedUpgrade>::on_upgrade",
                    ),
                )
            }
        },
        18 /* terrane-site: src/main.trn:55:10-55:44 */,
    );
}
async fn run_server() {
    let mut router: Router = __terrane_raised(
        terrane_static_trn_526f75746572_new(),
        19 /* terrane-site: src/main.trn:58:12-58:24 */,
    );
    router = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                router
                    .route(
                        &String::from("/health"),
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
                            20 /* terrane-site: src/main.trn:59:38-59:49 */,
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
        21 /* terrane-site: src/main.trn:59:12-59:50 */,
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
                            22 /* terrane-site: src/main.trn:60:34-60:54 */,
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
        23 /* terrane-site: src/main.trn:60:12-60:55 */,
    );
    let listener: TcpListener = __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = tokio::net::TcpListener::bind(
                        String::from("127.0.0.1:38765"),
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
                                                "Rust dependency `dependency` member `tokio::net::TcpListener::bind` failed: {error}"
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
                                        "dependency",
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
                        24 /* terrane-site: src/main.trn:61:20-61:56 */,
                    )
                }
            })
            .await,
        24 /* terrane-site: src/main.trn:61:20-61:56 */,
    );
    __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = axum::serve(listener, router);
                    async move {
                        match crate::__terrane_dependency_await_unwind(
                                std::future::IntoFuture::into_future(__terrane_call),
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
                                                "Rust dependency `axum` member `axum::serve` failed: {error}"
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
                                        "axum::serve",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        25 /* terrane-site: src/main.trn:62:10-62:43 */,
                    )
                }
            })
            .await,
        26 /* terrane-site: src/main.trn:62:9-62:44 */,
    );
}
fn main() {
    __terrane_run(async move {
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let server: TerraneScopedTask<()> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        std::sync::Arc::new(move || -> std::pin::Pin<
                            Box<dyn Future<Output = _> + Send>,
                        > { Box::pin(run_server()) })(),
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
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = drive_peer();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            27 /* terrane-site: src/main.trn:67:9-67:20 */,
                        )
                    }
                })
                .await,
            27 /* terrane-site: src/main.trn:67:9-67:20 */,
        );
        scope.cancel();
        {
            let _: TerraneTaskOutcome<()> = __terrane_await(scope.join(server)).await;
        }
    });
}
// Source: <terrane>/projected/deps/axum.trn
// Namespace: deps/axum
pub use axum::extract::ws::CloseFrame;
pub use axum::extract::ws::Message;
pub type MethodRouter78b1dfd45684b6ec3178aadb25929757adf127d60a8104a15dc868775202e5f7 = axum::routing::MethodRouter<
    (),
>;
pub use axum_core::response::Response;
pub type Router = axum::Router<()>;
pub use tokio::net::TcpListener;
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
// Source: <terrane>/projected/deps/axum-core.trn
// Namespace: deps/axum-core
// Source: <terrane>/projected/deps/axum-core/response.trn
// Namespace: deps/axum-core/response
// Source: <terrane>/projected/deps/axum/extract/ws.trn
// Namespace: deps/axum/extract/ws
// Source: <terrane>/projected/deps/axum/extract.trn
// Namespace: deps/axum/extract
// Source: <terrane>/projected/deps/axum/routing.trn
// Namespace: deps/axum/routing
// Source: <terrane>/projected/deps/terrane-websocket-peer-witness.trn
// Namespace: deps/terrane-websocket-peer-witness
pub async fn drive_peer() -> Result<(), crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            terrane_websocket_peer_witness::drive_peer(),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-websocket-peer-witness",
                    "terrane_websocket_peer_witness::drive_peer",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/tokio/net.trn
// Namespace: deps/tokio/net
