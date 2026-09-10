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
    pub static FUNCTIONS: [&str; 7] = [
        "/callable-throwable-contracts::render",
        "/callable-throwable-contracts::invoke",
        "/callable-throwable-contracts::async-render",
        "/callable-throwable-contracts::invoke-field",
        "/callable-throwable-contracts::invoke-maker",
        "/callable-throwable-contracts::invoke-async",
        "/callable-throwable-contracts::main",
    ];
    pub static SITES: [Site; 18] = [
        {
            /* terrane-site-row: site 0: /callable-throwable-contracts::render (case.trn:7:7-7:27) */
            Site {
                function: 0,
                file: 0,
                line: 7,
                column: 7,
                end_line: 7,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 1: /callable-throwable-contracts::invoke (case.trn:11:12-11:28) */
            Site {
                function: 1,
                file: 0,
                line: 11,
                column: 12,
                end_line: 11,
                end_column: 28,
            }
        },
        {
            /* terrane-site-row: site 2: /callable-throwable-contracts::invoke (case.trn:14:10-14:26) */
            Site {
                function: 1,
                file: 0,
                line: 14,
                column: 10,
                end_line: 14,
                end_column: 26,
            }
        },
        {
            /* terrane-site-row: site 3: /callable-throwable-contracts::async-render (case.trn:27:5-27:25) */
            Site {
                function: 2,
                file: 0,
                line: 27,
                column: 5,
                end_line: 27,
                end_column: 25,
            }
        },
        {
            /* terrane-site-row: site 4: /callable-throwable-contracts::invoke-field (case.trn:38:10-38:29) */
            Site {
                function: 3,
                file: 0,
                line: 38,
                column: 10,
                end_line: 38,
                end_column: 29,
            }
        },
        {
            /* terrane-site-row: site 5: /callable-throwable-contracts::invoke-maker (case.trn:41:15-41:27) */
            Site {
                function: 4,
                file: 0,
                line: 41,
                column: 15,
                end_line: 41,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 6: /callable-throwable-contracts::invoke-maker (case.trn:42:10-42:26) */
            Site {
                function: 4,
                file: 0,
                line: 42,
                column: 10,
                end_line: 42,
                end_column: 26,
            }
        },
        {
            /* terrane-site-row: site 7: /callable-throwable-contracts::invoke-async (case.trn:45:16-45:32) */
            Site {
                function: 5,
                file: 0,
                line: 45,
                column: 16,
                end_line: 45,
                end_column: 32,
            }
        },
        {
            /* terrane-site-row: site 8: /callable-throwable-contracts::main (case.trn:52:7-52:27) */
            Site {
                function: 6,
                file: 0,
                line: 52,
                column: 7,
                end_line: 52,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 9: /callable-throwable-contracts::main (case.trn:77:11-77:27) */
            Site {
                function: 6,
                file: 0,
                line: 77,
                column: 11,
                end_line: 77,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 10: /callable-throwable-contracts::main (case.trn:78:11-78:27) */
            Site {
                function: 6,
                file: 0,
                line: 78,
                column: 11,
                end_line: 78,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 11: /callable-throwable-contracts::main (case.trn:79:11-79:29) */
            Site {
                function: 6,
                file: 0,
                line: 79,
                column: 11,
                end_line: 79,
                end_column: 29,
            }
        },
        {
            /* terrane-site-row: site 12: /callable-throwable-contracts::main (case.trn:80:16-80:48) */
            Site {
                function: 6,
                file: 0,
                line: 80,
                column: 16,
                end_line: 80,
                end_column: 48,
            }
        },
        {
            /* terrane-site-row: site 13: /callable-throwable-contracts::main (case.trn:81:11-81:35) */
            Site {
                function: 6,
                file: 0,
                line: 81,
                column: 11,
                end_line: 81,
                end_column: 35,
            }
        },
        {
            /* terrane-site-row: site 14: /callable-throwable-contracts::main (case.trn:82:16-82:44) */
            Site {
                function: 6,
                file: 0,
                line: 82,
                column: 16,
                end_line: 82,
                end_column: 44,
            }
        },
        {
            /* terrane-site-row: site 15: /callable-throwable-contracts::main (case.trn:84:11-84:39) */
            Site {
                function: 6,
                file: 0,
                line: 84,
                column: 11,
                end_line: 84,
                end_column: 39,
            }
        },
        {
            /* terrane-site-row: site 16: /callable-throwable-contracts::main (case.trn:85:11-85:35) */
            Site {
                function: 6,
                file: 0,
                line: 85,
                column: 11,
                end_line: 85,
                end_column: 35,
            }
        },
        {
            /* terrane-site-row: site 17: /callable-throwable-contracts::main (case.trn:89:11-89:30) */
            Site {
                function: 6,
                file: 0,
                line: 89,
                column: 11,
                end_line: 89,
                end_column: 30,
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
fn __terrane_run<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize")
        .block_on(future)
}
// Source: case.trn
// Namespace: callable-throwable-contracts
#[derive(Clone)]
pub struct Formatter {}
impl Formatter {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn render(
        &self,
        value: terrane_int_support::Int,
    ) -> Result<String, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err(
                TerraneError::raised(
                    TerraneErrorKind::CoercionError,
                    0 /* terrane-site: case.trn:7:7-7:27 */,
                ),
            );
        }
        return Ok(String::from("bound"));
    }
    pub fn invoke(
        &self,
        operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        >,
        value: terrane_int_support::Int,
    ) -> Result<String, TerraneError> {
        return Ok(
            __terrane_traced_err(
                operation(value.clone()),
                1 /* terrane-site: case.trn:11:12-11:28 */,
            )?,
        );
    }
}
fn invoke(
    operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
    >,
    value: terrane_int_support::Int,
) -> Result<String, TerraneError> {
    return Ok(
        __terrane_traced_err(
            operation(value.clone()),
            2 /* terrane-site: case.trn:14:10-14:26 */,
        )?,
    );
}
fn safe_render(value: terrane_int_support::Int) -> String {
    let _ = &value;
    return String::from("safe");
}
fn invoke_infallible(
    operation: std::sync::Arc<dyn Fn(terrane_int_support::Int) -> String + Send + Sync>,
    value: terrane_int_support::Int,
) -> String {
    return operation(value.clone());
}
#[derive(Clone)]
pub struct OperationHolder {
    pub operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
    >,
}
impl OperationHolder {
    pub fn terrane_construct() -> Self {
        Self {
            operation: std::sync::Arc::new(move |argument_0: terrane_int_support::Int| Ok(
                safe_render(argument_0),
            )),
        }
    }
}
async fn async_render(value: terrane_int_support::Int) -> Result<String, TerraneError> {
    if value.clone() < terrane_int_support::Int::from(0_i128) {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                3 /* terrane-site: case.trn:27:5-27:25 */,
            ),
        );
    }
    return Ok(String::from("async"));
}
fn make_render(
    value: terrane_int_support::Int,
) -> std::sync::Arc<
    dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
> {
    let _ = &value;
    let service: Formatter = Formatter::terrane_construct();
    return {
        let receiver = service;
        std::sync::Arc::new(move |argument_0: terrane_int_support::Int| {
            receiver.render(argument_0)
        })
    };
}
async fn async_safe_render(value: terrane_int_support::Int) -> String {
    let _ = &value;
    return String::from("async-safe");
}
fn invoke_field(holder: OperationHolder) -> String {
    return __terrane_traced(
        (holder.operation)(terrane_int_support::Int::from(1_i128)),
        4 /* terrane-site: case.trn:38:10-38:29 */,
    );
}
fn invoke_maker(
    maker: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> Result<
                std::sync::Arc<
                    dyn Fn(
                        terrane_int_support::Int,
                    ) -> Result<String, TerraneError> + Send + Sync,
                >,
                TerraneError,
            > + Send + Sync,
    >,
    value: terrane_int_support::Int,
) -> Result<String, TerraneError> {
    let operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
    > = __terrane_traced_err(
        maker(value.clone()),
        5 /* terrane-site: case.trn:41:15-41:27 */,
    )?;
    return Ok(
        __terrane_traced_err(
            operation(value.clone()),
            6 /* terrane-site: case.trn:42:10-42:26 */,
        )?,
    );
}
async fn invoke_async(
    operation: std::sync::Arc<
        dyn Fn(
            terrane_int_support::Int,
        ) -> std::pin::Pin<
                Box<dyn Future<Output = Result<String, TerraneError>> + Send>,
            > + Send + Sync,
    >,
    value: terrane_int_support::Int,
) -> Result<String, TerraneError> {
    return Ok(
        __terrane_traced_err(
            __terrane_await(operation(value.clone())).await,
            7 /* terrane-site: case.trn:45:16-45:32 */,
        )?,
    );
}
fn main() {
    __terrane_run(async move {
        let service: Formatter = Formatter::terrane_construct();
        let bound: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = {
            let receiver = service.clone();
            std::sync::Arc::new(move |argument_0: terrane_int_support::Int| {
                receiver.render(argument_0)
            })
        };
        let closure: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = {
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> Result<String, TerraneError> {
                if value.clone() < terrane_int_support::Int::from(0_i128) {
                    return Err(
                        TerraneError::raised(
                            TerraneErrorKind::CoercionError,
                            8 /* terrane-site: case.trn:52:7-52:27 */,
                        ),
                    );
                }
                return Ok(String::from("closure"));
            })
        };
        let broad_closure: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
        > = {
            std::sync::Arc::new(move |value: terrane_int_support::Int| -> String {
                if value.clone() < terrane_int_support::Int::from(0_i128) {
                    return String::from("negative");
                }
                return String::from("wide-closure");
            })
        };
        let broad_async: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync,
        > = {
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> std::pin::Pin<Box<dyn Future<Output = String> + Send>> {
                Box::pin(async move {
                    if value.clone() < terrane_int_support::Int::from(0_i128) {
                        return String::from("negative");
                    }
                    return String::from("wide-async");
                })
            })
        };
        let alias: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = bound.clone();
        let broad: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = bound.clone();
        let async_operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = Result<String, TerraneError>> + Send>,
                > + Send + Sync,
        > = std::sync::Arc::new(move |
            argument_0: terrane_int_support::Int,
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(async_render(argument_0))
        });
        let safe_operation: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> String + Send + Sync,
        > = std::sync::Arc::new(safe_render);
        let async_safe_operation: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<Box<dyn Future<Output = String> + Send>> + Send + Sync,
        > = std::sync::Arc::new(move |
            argument_0: terrane_int_support::Int,
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(async_safe_render(argument_0))
        });
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = bound; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = bound; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = closure;
            "coercion-error".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = closure;
            "coercion-error".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = broad; "throwable"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = broad; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = alias; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = alias; "coercion-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = async_operation;
            "throwable".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = async_operation;
            "coercion-error".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(alias
            .clone(), terrane_int_support::Int::from(1_i128)), 9 /* terrane-site: case.trn:77:11-77:27 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(broad
            .clone(), terrane_int_support::Int::from(1_i128)), 10 /* terrane-site: case.trn:78:11-78:27 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(closure
            .clone(), terrane_int_support::Int::from(1_i128)), 11 /* terrane-site: case.trn:79:11-79:29 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(invoke_async(async_operation
            .clone(), terrane_int_support::Int::from(1_i128))). await,
            12 /* terrane-site: case.trn:80:16-80:48 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke({ let
            callable = broad_closure.clone(); std::sync::Arc::new(move | argument_0 :
            terrane_int_support::Int | Ok(callable(argument_0))) },
            terrane_int_support::Int::from(1_i128)), 13 /* terrane-site: case.trn:81:11-81:35 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(invoke_async({
            let callable = broad_async.clone(); std::sync::Arc::new(move | argument_0 :
            terrane_int_support::Int | -> std::pin::Pin < Box < dyn Future < Output = _ >
            + Send > > { let callable = callable.clone(); Box::pin(async move {
            Ok(callable(argument_0). await) }) }) },
            terrane_int_support::Int::from(1_i128))). await, 14 /* terrane-site: case.trn:82:16-82:44 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&safe_operation(terrane_int_support::Int::from(1_i128)))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(invoke_maker(std::sync::Arc::new(move
            | argument_0 : terrane_int_support::Int | Ok(make_render(argument_0))),
            terrane_int_support::Int::from(1_i128)), 15 /* terrane-site: case.trn:84:11-84:39 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(service
            .invoke(bound.clone(), terrane_int_support::Int::from(1_i128)),
            16 /* terrane-site: case.trn:85:11-85:35 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&invoke_infallible(safe_operation
            .clone(), terrane_int_support::Int::from(1_i128)))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(async_safe_operation(terrane_int_support::Int::from(1_i128)))
            . await)
        );
        let holder: OperationHolder = OperationHolder::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced((holder
            .operation) (terrane_int_support::Int::from(1_i128)), 17 /* terrane-site: case.trn:89:11-89:30 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&invoke_field(holder.clone()))
        );
    });
}
