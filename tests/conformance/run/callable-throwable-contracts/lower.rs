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
mod __terrane_error_registry {
    #[allow(dead_code, reason = "custom descriptors are absent from some programs")]
    pub static DESCRIPTORS: [&str; 1] = ["local-error"];
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
    pub static FUNCTIONS: [&str; 8] = [
        "/callable-throwable-contracts::render",
        "/callable-throwable-contracts::invoke",
        "/callable-throwable-contracts::custom-render",
        "/callable-throwable-contracts::async-render",
        "/callable-throwable-contracts::invoke-field",
        "/callable-throwable-contracts::invoke-maker",
        "/callable-throwable-contracts::invoke-async",
        "/callable-throwable-contracts::main",
    ];
    pub static SITES: [Site; 21] = [
        {
            /* terrane-site-row: site 0: /callable-throwable-contracts::render (case.trn:16:7-16:27) */
            Site {
                function: 0,
                file: 0,
                line: 16,
                column: 7,
                end_line: 16,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 1: /callable-throwable-contracts::invoke (case.trn:20:12-20:28) */
            Site {
                function: 1,
                file: 0,
                line: 20,
                column: 12,
                end_line: 20,
                end_column: 28,
            }
        },
        {
            /* terrane-site-row: site 2: /callable-throwable-contracts::invoke (case.trn:23:10-23:26) */
            Site {
                function: 1,
                file: 0,
                line: 23,
                column: 10,
                end_line: 23,
                end_column: 26,
            }
        },
        {
            /* terrane-site-row: site 3: /callable-throwable-contracts::custom-render (case.trn:36:5-36:32) */
            Site {
                function: 2,
                file: 0,
                line: 36,
                column: 5,
                end_line: 36,
                end_column: 32,
            }
        },
        {
            /* terrane-site-row: site 4: /callable-throwable-contracts::async-render (case.trn:47:5-47:25) */
            Site {
                function: 3,
                file: 0,
                line: 47,
                column: 5,
                end_line: 47,
                end_column: 25,
            }
        },
        {
            /* terrane-site-row: site 5: /callable-throwable-contracts::invoke-field (case.trn:58:10-58:29) */
            Site {
                function: 4,
                file: 0,
                line: 58,
                column: 10,
                end_line: 58,
                end_column: 29,
            }
        },
        {
            /* terrane-site-row: site 6: /callable-throwable-contracts::invoke-maker (case.trn:61:15-61:27) */
            Site {
                function: 5,
                file: 0,
                line: 61,
                column: 15,
                end_line: 61,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 7: /callable-throwable-contracts::invoke-maker (case.trn:62:10-62:26) */
            Site {
                function: 5,
                file: 0,
                line: 62,
                column: 10,
                end_line: 62,
                end_column: 26,
            }
        },
        {
            /* terrane-site-row: site 8: /callable-throwable-contracts::invoke-async (case.trn:65:16-65:32) */
            Site {
                function: 6,
                file: 0,
                line: 65,
                column: 16,
                end_line: 65,
                end_column: 32,
            }
        },
        {
            /* terrane-site-row: site 9: /callable-throwable-contracts::main (case.trn:72:7-72:27) */
            Site {
                function: 7,
                file: 0,
                line: 72,
                column: 7,
                end_line: 72,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 10: /callable-throwable-contracts::main (case.trn:100:11-100:27) */
            Site {
                function: 7,
                file: 0,
                line: 100,
                column: 11,
                end_line: 100,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 11: /callable-throwable-contracts::main (case.trn:101:11-101:27) */
            Site {
                function: 7,
                file: 0,
                line: 101,
                column: 11,
                end_line: 101,
                end_column: 27,
            }
        },
        {
            /* terrane-site-row: site 12: /callable-throwable-contracts::main (case.trn:102:11-102:29) */
            Site {
                function: 7,
                file: 0,
                line: 102,
                column: 11,
                end_line: 102,
                end_column: 29,
            }
        },
        {
            /* terrane-site-row: site 13: /callable-throwable-contracts::main (case.trn:103:16-103:48) */
            Site {
                function: 7,
                file: 0,
                line: 103,
                column: 16,
                end_line: 103,
                end_column: 48,
            }
        },
        {
            /* terrane-site-row: site 14: /callable-throwable-contracts::main (case.trn:105:18-105:51) */
            Site {
                function: 7,
                file: 0,
                line: 105,
                column: 18,
                end_line: 105,
                end_column: 51,
            }
        },
        {
            /* terrane-site-row: site 15: /callable-throwable-contracts::main (case.trn:108:11-108:35) */
            Site {
                function: 7,
                file: 0,
                line: 108,
                column: 11,
                end_line: 108,
                end_column: 35,
            }
        },
        {
            /* terrane-site-row: site 16: /callable-throwable-contracts::main (case.trn:109:16-109:44) */
            Site {
                function: 7,
                file: 0,
                line: 109,
                column: 16,
                end_line: 109,
                end_column: 44,
            }
        },
        {
            /* terrane-site-row: site 17: /callable-throwable-contracts::main (case.trn:111:11-111:39) */
            Site {
                function: 7,
                file: 0,
                line: 111,
                column: 11,
                end_line: 111,
                end_column: 39,
            }
        },
        {
            /* terrane-site-row: site 18: /callable-throwable-contracts::main (case.trn:112:11-112:35) */
            Site {
                function: 7,
                file: 0,
                line: 112,
                column: 11,
                end_line: 112,
                end_column: 35,
            }
        },
        {
            /* terrane-site-row: site 19: /callable-throwable-contracts::main (case.trn:116:11-116:30) */
            Site {
                function: 7,
                file: 0,
                line: 116,
                column: 11,
                end_line: 116,
                end_column: 30,
            }
        },
        {
            /* terrane-site-row: site 20: /callable-throwable-contracts::main (case.trn:121:13-121:31) */
            Site {
                function: 7,
                file: 0,
                line: 121,
                column: 13,
                end_line: 121,
                end_column: 31,
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
pub struct LocalError {
    pub message: String,
}
impl LocalError {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("local failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct Unrelated {}
impl Unrelated {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn render(&self, value: terrane_int_support::Int) -> String {
        let _ = &value;
        return String::from("method");
    }
}
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
                    0 /* terrane-site: case.trn:16:7-16:27 */,
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
                1 /* terrane-site: case.trn:20:12-20:28 */,
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
            2 /* terrane-site: case.trn:23:10-23:26 */,
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
fn loud_render(value: terrane_int_support::Int) -> String {
    let _ = &value;
    return String::from("loud");
}
fn custom_render(value: terrane_int_support::Int) -> Result<String, TerraneError> {
    if value.clone() < terrane_int_support::Int::from(0_i128) {
        return Err({
            let value = LocalError::terrane_construct();
            TerraneError::raised_with_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                value.render(),
                3 /* terrane-site: case.trn:36:5-36:32 */,
            )
        });
    }
    return Ok(String::from("custom"));
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
#[derive(Clone)]
pub struct CallableHolder {
    pub render: std::sync::Arc<dyn Fn(terrane_int_support::Int) -> String + Send + Sync>,
}
impl CallableHolder {
    pub fn terrane_construct() -> Self {
        Self {
            render: std::sync::Arc::new(safe_render),
        }
    }
}
async fn async_render(value: terrane_int_support::Int) -> Result<String, TerraneError> {
    if value.clone() < terrane_int_support::Int::from(0_i128) {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                4 /* terrane-site: case.trn:47:5-47:25 */,
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
        5 /* terrane-site: case.trn:58:10-58:29 */,
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
        6 /* terrane-site: case.trn:61:15-61:27 */,
    )?;
    return Ok(
        __terrane_traced_err(
            operation(value.clone()),
            7 /* terrane-site: case.trn:62:10-62:26 */,
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
            8 /* terrane-site: case.trn:65:16-65:32 */,
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
                            9 /* terrane-site: case.trn:72:7-72:27 */,
                        ),
                    );
                }
                return Ok(String::from("closure"));
            })
        };
        let broad_closure: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = {
            let callable = {
                std::sync::Arc::new(move |value: terrane_int_support::Int| -> String {
                    if value.clone() < terrane_int_support::Int::from(0_i128) {
                        return String::from("negative");
                    }
                    return String::from("wide-closure");
                })
            };
            std::sync::Arc::new(move |argument_0: terrane_int_support::Int| Ok(
                callable(argument_0),
            ))
        };
        let broad_async: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = Result<String, TerraneError>> + Send>,
                > + Send + Sync,
        > = {
            let callable = {
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
            std::sync::Arc::new(move |
                argument_0: terrane_int_support::Int,
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                let callable = callable.clone();
                Box::pin(async move { Ok(callable(argument_0).await) })
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
            "{}", terrane_scalar_support::scalar_text(&{ let _ = broad; "throwable"
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
            "throwable".to_owned() })
        );
        let custom: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> Result<String, TerraneError> + Send + Sync,
        > = std::sync::Arc::new(custom_render);
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = custom; "local-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = custom; "local-error"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(alias
            .clone(), terrane_int_support::Int::from(1_i128)), 10 /* terrane-site: case.trn:100:11-100:27 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(broad
            .clone(), terrane_int_support::Int::from(1_i128)), 11 /* terrane-site: case.trn:101:11-101:27 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(invoke(closure
            .clone(), terrane_int_support::Int::from(1_i128)), 12 /* terrane-site: case.trn:102:11-102:29 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(invoke_async(async_operation
            .clone(), terrane_int_support::Int::from(1_i128))). await,
            13 /* terrane-site: case.trn:103:16-103:48 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(__terrane_await(invoke_async(async_operation
                    .clone(), terrane_int_support::Int::from(- 1_i128))). await,
                    14 /* terrane-site: case.trn:105:18-105:51 */))
                );
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
                        && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("async-caught"))
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
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(invoke(broad_closure
            .clone(), terrane_int_support::Int::from(1_i128)), 15 /* terrane-site: case.trn:108:11-108:35 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(invoke_async(broad_async
            .clone(), terrane_int_support::Int::from(1_i128))). await,
            16 /* terrane-site: case.trn:109:16-109:44 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&safe_operation(terrane_int_support::Int::from(1_i128)))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(invoke_maker(std::sync::Arc::new(move
            | argument_0 : terrane_int_support::Int | Ok(make_render(argument_0))),
            terrane_int_support::Int::from(1_i128)), 17 /* terrane-site: case.trn:111:11-111:39 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(service
            .invoke(bound.clone(), terrane_int_support::Int::from(1_i128)),
            18 /* terrane-site: case.trn:112:11-112:35 */))
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
            .operation) (terrane_int_support::Int::from(1_i128)), 19 /* terrane-site: case.trn:116:11-116:30 */))
        );
        let mut callable: CallableHolder = CallableHolder::terrane_construct();
        callable.render = std::sync::Arc::new(loud_render);
        println!(
            "{}", terrane_scalar_support::scalar_text(&(callable.render)
            (terrane_int_support::Int::from(1_i128)))
        );
        let __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_traced_completion!(invoke(custom
                    .clone(), terrane_int_support::Int::from(- 1_i128)),
                    20 /* terrane-site: case.trn:121:13-121:31 */))
                );
                TerraneCompletion::Normal
            })();
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
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("custom-caught"))
                        );
                    }
                    if !__terrane_handled_1 {
                        return TerraneCompletion::Error(__terrane_error_1);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&invoke_field(holder.clone()))
        );
    });
}
