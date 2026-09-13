// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, async_dependency.rs, tasks_native_local.rs
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
    pub static FUNCTIONS: [&str; 3] = [
        "/app::consume",
        "/app::send-blocked",
        "/app::main",
    ];
    pub static SITES: [Site; 18] = [
        /* terrane-site-row: site 0: /app::consume (src/main.trn:6:16-6:38) */
        { Site { function: 0, file: 0, line: 6, column: 16, end_line: 6, end_column: 38 } },
        /* terrane-site-row: site 1: /app::send-blocked (src/main.trn:9:16-9:36) */
        { Site { function: 1, file: 0, line: 9, column: 16, end_line: 9, end_column: 36 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:12:11-12:20) */
        { Site { function: 2, file: 0, line: 12, column: 11, end_line: 12, end_column: 20 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:13:12-13:24) */
        { Site { function: 2, file: 0, line: 13, column: 12, end_line: 13, end_column: 24 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:14:14-14:35) */
        { Site { function: 2, file: 0, line: 14, column: 14, end_line: 14, end_column: 35 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:15:14-15:35) */
        { Site { function: 2, file: 0, line: 15, column: 14, end_line: 15, end_column: 35 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:18:17-18:37) */
        { Site { function: 2, file: 0, line: 18, column: 17, end_line: 18, end_column: 37 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:19:18-19:38) */
        { Site { function: 2, file: 0, line: 19, column: 18, end_line: 19, end_column: 38 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:21:24-21:39) */
        { Site { function: 2, file: 0, line: 21, column: 24, end_line: 21, end_column: 39 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:22:23-22:38) */
        { Site { function: 2, file: 0, line: 22, column: 23, end_line: 22, end_column: 38 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:28:12-28:33) */
        { Site { function: 2, file: 0, line: 28, column: 12, end_line: 28, end_column: 33 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:29:20-29:39) */
        { Site { function: 2, file: 0, line: 29, column: 20, end_line: 29, end_column: 39 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:31:30-31:43) */
        { Site { function: 2, file: 0, line: 31, column: 30, end_line: 31, end_column: 43 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:33:13-33:26) */
        { Site { function: 2, file: 0, line: 33, column: 13, end_line: 33, end_column: 26 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:39:11-39:22) */
        { Site { function: 2, file: 0, line: 39, column: 11, end_line: 39, end_column: 22 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:40:18-40:41) */
        { Site { function: 2, file: 0, line: 40, column: 18, end_line: 40, end_column: 41 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:42:23-42:35) */
        { Site { function: 2, file: 0, line: 42, column: 23, end_line: 42, end_column: 35 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:44:25-44:37) */
        { Site { function: 2, file: 0, line: 44, column: 25, end_line: 44, end_column: 37 } },
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
async fn consume(incoming: Incoming) -> String {
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = drain_slowly(incoming);
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        0 /* terrane-site: src/main.trn:6:16-6:38 */,
                    )
                }
            })
            .await,
        0 /* terrane-site: src/main.trn:6:16-6:38 */,
    );
}
async fn send_blocked(
    mut sink: Outgoing,
) -> terrane_collection_support::AsyncSinkOutcome {
    return __terrane_traced(
        __terrane_await({
                let __terrane_future = {
                    let __terrane_call = (&mut sink).send(String::from("blocked"));
                    async move {
                        match crate::__terrane_dependency_await_unwind(__terrane_call)
                            .await
                        {
                            Ok(Ok(value)) => {
                                Ok(
                                    terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                        value,
                                    ),
                                )
                            }
                            Ok(Err(error)) => {
                                Err(
                                    crate::TerraneForeignError(
                                        crate::TerraneError::custom_raised(
                                            crate::TERRANE_DEPENDENCY_ERROR,
                                            format!(
                                                "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::send` failed: {error}"
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
                                        "terrane_sink_witness",
                                        "terrane_sink_witness::Outgoing::send",
                                    ),
                                )
                            }
                        }
                    }
                };
                async move {
                    __terrane_raised_err(
                        __terrane_future.await,
                        1 /* terrane-site: src/main.trn:9:16-9:36 */,
                    )
                }
            })
            .await,
        1 /* terrane-site: src/main.trn:9:16-9:36 */,
    );
}
fn main() {
    __terrane_run(async move {
        let whole: Duplex = __terrane_raised(
            duplex(terrane_int_support::Int::from(1_i128)),
            2 /* terrane-site: src/main.trn:12:11-12:20 */,
        );
        let mut halves: SplitEndpoints = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| whole.split()),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Duplex::split` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::Duplex::split",
                        ),
                    )
                }
            },
            3 /* terrane-site: src/main.trn:13:12-13:24 */,
        );
        let incoming: Incoming = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| halves.take_incoming()),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::SplitEndpoints::take_incoming` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::SplitEndpoints::take_incoming",
                        ),
                    )
                }
            },
            4 /* terrane-site: src/main.trn:14:14-14:35 */,
        );
        let mut outgoing: Outgoing = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| halves.take_outgoing()),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::SplitEndpoints::take_outgoing` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::SplitEndpoints::take_outgoing",
                        ),
                    )
                }
            },
            5 /* terrane-site: src/main.trn:15:14-15:35 */,
        );
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let consumer: TerraneScopedTask<String> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = consume(incoming);
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
        let first: terrane_collection_support::AsyncSinkOutcome = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).send(String::from("one"));
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                            value,
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::send` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            6 /* terrane-site: src/main.trn:18:17-18:37 */,
                        )
                    }
                })
                .await,
            6 /* terrane-site: src/main.trn:18:17-18:37 */,
        );
        let second: terrane_collection_support::AsyncSinkOutcome = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).send(String::from("two"));
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                            value,
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::send` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            7 /* terrane-site: src/main.trn:19:18-19:38 */,
                        )
                    }
                })
                .await,
            7 /* terrane-site: src/main.trn:19:18-19:38 */,
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&first.accepted),
            terrane_scalar_support::scalar_text(&first.closed),
            terrane_scalar_support::scalar_text(&second.accepted)
        );
        let flushed: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut outgoing).flush();
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
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::flush` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::flush",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            8 /* terrane-site: src/main.trn:21:24-21:39 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:21:24-21:39 */,
        );
        let closed: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = outgoing.close();
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
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::close` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::close",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            9 /* terrane-site: src/main.trn:22:23-22:38 */,
                        )
                    }
                })
                .await,
            9 /* terrane-site: src/main.trn:22:23-22:38 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&flushed),
            terrane_scalar_support::scalar_text(&closed)
        );
        let consumed: TerraneTaskOutcome<String> = __terrane_await(scope.join(consumer))
            .await;
        let consumed_value: Option<String> = consumed.value.clone();
        if consumed_value.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&consumed.completed),
                terrane_scalar_support::scalar_text(&* consumed_value.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        let mut remote: Outgoing = __terrane_raised(
            remotely_closed_sink(),
            10 /* terrane-site: src/main.trn:28:12-28:33 */,
        );
        let rejected: terrane_collection_support::AsyncSinkOutcome = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut remote).send(String::from("lost"));
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                            value,
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::send` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            11 /* terrane-site: src/main.trn:29:20-29:39 */,
                        )
                    }
                })
                .await,
            11 /* terrane-site: src/main.trn:29:20-29:39 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&rejected.accepted),
            terrane_scalar_support::scalar_text(&rejected.closed)
        );
        let remote_closed: bool = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = remote.close();
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
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::Outgoing::close` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::Outgoing::close",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            12 /* terrane-site: src/main.trn:31:30-31:43 */,
                        )
                    }
                })
                .await,
            12 /* terrane-site: src/main.trn:31:30-31:43 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&remote_closed));
        let blocked: Outgoing = __terrane_raised(
            blocked_sink(),
            13 /* terrane-site: src/main.trn:33:13-33:26 */,
        );
        let blocked_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let blocked_child: TerraneScopedTask<
            terrane_collection_support::AsyncSinkOutcome,
        > = {
            let __terrane_scope = blocked_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = send_blocked(blocked);
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
        blocked_scope.cancel();
        let cancelled: TerraneTaskOutcome<
            terrane_collection_support::AsyncSinkOutcome,
        > = __terrane_await(blocked_scope.join(blocked_child)).await;
        println!("{}", terrane_scalar_support::scalar_text(&cancelled.cancelled));
        let mut queue: QueueSink = __terrane_raised(
            queue_sink(),
            14 /* terrane-site: src/main.trn:39:11-39:22 */,
        );
        let queued: terrane_collection_support::AsyncSinkOutcome = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&mut queue)
                            .send(String::from("different"));
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::AsyncSinkOutcome::from_accepted(
                                            value,
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::QueueSink::send` failed: {error}"
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
                                            "terrane_sink_witness",
                                            "terrane_sink_witness::QueueSink::send",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            15 /* terrane-site: src/main.trn:40:18-40:41 */,
                        )
                    }
                })
                .await,
            15 /* terrane-site: src/main.trn:40:18-40:41 */,
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&queued.accepted),
            terrane_scalar_support::scalar_text(&queued.closed)
        );
        let queue_flushed: terrane_int_support::Int = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| queue.flush()),
            ) {
                Ok(Ok(value)) => Ok(terrane_int_support::Int::from(i128::from(value))),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::QueueSink::flush` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::QueueSink::flush",
                        ),
                    )
                }
            },
            16 /* terrane-site: src/main.trn:42:23-42:35 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&queue_flushed));
        let queue_closed: String = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| queue.close()),
            ) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::TERRANE_DEPENDENCY_ERROR,
                                format!(
                                    "Rust dependency `terrane_sink_witness` member `terrane_sink_witness::QueueSink::close` failed: {error}"
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
                            "terrane_sink_witness",
                            "terrane_sink_witness::QueueSink::close",
                        ),
                    )
                }
            },
            17 /* terrane-site: src/main.trn:44:25-44:37 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&queue_closed));
    });
}
// Source: <terrane>/projected/deps/terrane-sink-witness.trn
// Namespace: deps/terrane-sink-witness
pub use terrane_sink_witness::Incoming;
pub use terrane_sink_witness::Outgoing;
pub use terrane_sink_witness::QueueSink;
pub use terrane_sink_witness::Duplex;
pub use terrane_sink_witness::SplitEndpoints;
pub fn blocked_sink() -> Result<Outgoing, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::blocked_sink()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::blocked_sink",
                ),
            )
        }
    }
}
pub async fn drain_slowly(
    incoming: Incoming,
) -> Result<String, crate::TerraneForeignError> {
    let incoming = incoming;
    match crate::__terrane_dependency_await_unwind(
            terrane_sink_witness::drain_slowly(incoming),
        )
        .await
    {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::drain_slowly",
                ),
            )
        }
    }
}
pub fn duplex(
    capacity: terrane_int_support::Int,
) -> Result<Duplex, crate::TerraneForeignError> {
    let capacity = terrane_int_support::coerce::<i64>(&capacity)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::duplex(capacity)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::duplex",
                ),
            )
        }
    }
}
pub fn queue_sink() -> Result<QueueSink, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::queue_sink()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::queue_sink",
                ),
            )
        }
    }
}
pub fn remotely_closed_sink() -> Result<Outgoing, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::remotely_closed_sink()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::remotely_closed_sink",
                ),
            )
        }
    }
}
