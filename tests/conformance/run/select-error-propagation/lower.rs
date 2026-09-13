// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, channels.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
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
    pub static FUNCTIONS: [&str; 5] = [
        "/select-error-propagation::fail",
        "/select-error-propagation::cleanup-coercion",
        "/select-error-propagation::cleanup-missing",
        "/select-error-propagation::construction-failure",
        "/select-error-propagation::main",
    ];
    pub static SITES: [Site; 13] = [
        /* terrane-site-row: site 0: /select-error-propagation::fail (case.trn:6:3-6:23) */
        { Site { function: 0, file: 0, line: 6, column: 3, end_line: 6, end_column: 23 } },
        /* terrane-site-row: site 1: /select-error-propagation::cleanup-coercion (case.trn:16:5-16:25) */
        { Site { function: 1, file: 0, line: 16, column: 5, end_line: 16, end_column: 25 } },
        /* terrane-site-row: site 2: /select-error-propagation::cleanup-missing (case.trn:24:5-24:22) */
        { Site { function: 2, file: 0, line: 24, column: 5, end_line: 24, end_column: 22 } },
        /* terrane-site-row: site 3: /select-error-propagation::construction-failure (case.trn:31:3-31:20) */
        { Site { function: 3, file: 0, line: 31, column: 3, end_line: 31, end_column: 20 } },
        /* terrane-site-row: site 4: /select-error-propagation::main (case.trn:50:31-50:36) */
        { Site { function: 4, file: 0, line: 50, column: 31, end_line: 50, end_column: 36 } },
        /* terrane-site-row: site 5: /select-error-propagation::main (case.trn:50:30-50:37) */
        { Site { function: 4, file: 0, line: 50, column: 30, end_line: 50, end_column: 37 } },
        /* terrane-site-row: site 6: /select-error-propagation::main (case.trn:59:19-59:59) */
        { Site { function: 4, file: 0, line: 59, column: 19, end_line: 59, end_column: 59 } },
        /* terrane-site-row: site 7: /select-error-propagation::main (case.trn:61:19-61:57) */
        { Site { function: 4, file: 0, line: 61, column: 19, end_line: 61, end_column: 57 } },
        /* terrane-site-row: site 8: /select-error-propagation::main (case.trn:61:18-61:58) */
        { Site { function: 4, file: 0, line: 61, column: 18, end_line: 61, end_column: 58 } },
        /* terrane-site-row: site 9: /select-error-propagation::main (case.trn:59:18-59:60) */
        { Site { function: 4, file: 0, line: 59, column: 18, end_line: 59, end_column: 60 } },
        /* terrane-site-row: site 10: /select-error-propagation::main (case.trn:71:19-71:63) */
        { Site { function: 4, file: 0, line: 71, column: 19, end_line: 71, end_column: 63 } },
        /* terrane-site-row: site 11: /select-error-propagation::main (case.trn:73:28-73:49) */
        { Site { function: 4, file: 0, line: 73, column: 28, end_line: 73, end_column: 49 } },
        /* terrane-site-row: site 12: /select-error-propagation::main (case.trn:71:18-71:64) */
        { Site { function: 4, file: 0, line: 71, column: 18, end_line: 71, end_column: 64 } },
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
// Source: case.trn
// Namespace: select-error-propagation
async fn fail() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:6:3-6:23 */,
        ),
    );
}
async fn ready() -> String {
    return String::from("winner");
}
async fn cleanup_coercion(
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
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
                        if !__terrane_handled_0 {
                            return TerraneCompletion::Error(__terrane_error_0);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_0 = __terrane_maybe_completion_0.is_none();
    let mut __terrane_completion_0 = __terrane_maybe_completion_0
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("cleanup-coercion"))
        );
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                1 /* terrane-site: case.trn:16:5-16:25 */,
            ),
        );
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    if __terrane_cancelled_0
        && matches!(&__terrane_completion_0, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_0).await;
    }
    __terrane_finally_guard_0.finish();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn cleanup_missing(
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) -> Result<(), TerraneError> {
    let mut __terrane_finally_guard_1 = __terrane_finally_guard();
    let __terrane_maybe_completion_1: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_1,
            async {
                let __terrane_try_1: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_try_1 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_1) => {
                        let mut __terrane_handled_1 = false;
                        if !__terrane_handled_1 {
                            return TerraneCompletion::Error(__terrane_error_1);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_1 = __terrane_maybe_completion_1.is_none();
    let mut __terrane_completion_1 = __terrane_maybe_completion_1
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_1: TerraneCompletion<()> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&String::from("cleanup-missing"))
        );
        return TerraneCompletion::Error(
            TerraneError::raised(
                TerraneErrorKind::MissingKey,
                2 /* terrane-site: case.trn:24:5-24:22 */,
            ),
        );
    })();
    match __terrane_finally_1 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_1 = replacement,
    }
    if __terrane_cancelled_1
        && matches!(&__terrane_completion_1, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_1).await;
    }
    __terrane_finally_guard_1.finish();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
async fn accept(value: terrane_int_support::Int) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn construction_failure() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::MissingKey,
            3 /* terrane-site: case.trn:31:3-31:20 */,
        ),
    );
}
fn late_construction() -> terrane_int_support::Int {
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&String::from("unexpected late construction"))
    );
    return terrane_int_support::Int::from(1_i128);
}
async fn blocked(receiver: TerraneChannelReceiver<terrane_int_support::Int>) {
    let mut __terrane_finally_guard_2 = __terrane_finally_guard();
    let __terrane_maybe_completion_2: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_2,
            async {
                let __terrane_try_2: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_try_2 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_2) => {
                        let mut __terrane_handled_2 = false;
                        if !__terrane_handled_2 {
                            return TerraneCompletion::Error(__terrane_error_2);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_2 = __terrane_maybe_completion_2.is_none();
    let mut __terrane_completion_2 = __terrane_maybe_completion_2
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_2: TerraneCompletion<()> = (|| {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("cleanup-before-error"))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_2 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_2 = replacement,
    }
    if __terrane_cancelled_2
        && matches!(&__terrane_completion_2, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_2).await;
    }
    __terrane_finally_guard_2.finish();
    match __terrane_completion_2 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_1164 = 0usize;
        let mut __terrane_select_cursor_1450 = 0usize;
        let mut __terrane_select_cursor_1828 = 0usize;
        let pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_3: TerraneCompletion<()> = async {
            let __terrane_try_3: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1164 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1164: Option<TerraneError> = None;
                    let __terrane_select_control_1164_0 = __terrane_select_control();
                    let mut __terrane_select_future_1164_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1164_0
                        .clone(), blocked(pair.receiver))
                    );
                    let mut __terrane_select_result_1164_0 = None;
                    let __terrane_select_control_1164_1 = __terrane_select_control();
                    let mut __terrane_select_future_1164_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1164_1
                        .clone(), fail())
                    );
                    let mut __terrane_select_result_1164_1 = None;
                    let __terrane_select_winner_1164 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..2usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1164
                                    + __terrane_select_offset) % 2usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1164_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1164_0 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(0usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    1 => {
                                        match Future::poll(
                                            __terrane_select_future_1164_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1164_1 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(1usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    _ => {
                                        unreachable!("select candidate is within the case count")
                                    }
                                }
                            }
                            std::task::Poll::Pending
                        })
                        .await;
                    if __terrane_select_winner_1164 == usize::MAX {
                        __terrane_select_control_1164_1.request_cancel();
                        __terrane_select_control_1164_0.request_cancel();
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1164_1
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1164 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    5 /* terrane-site: case.trn:50:30-50:37 */,
                                ),
                            );
                        }
                        let _ = __terrane_select_future_1164_0.as_mut().await;
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1164.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1164
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1164)
                            .await;
                    }
                    __terrane_select_cursor_1164 = (__terrane_select_winner_1164
                        + 1usize) % 2usize;
                    match __terrane_select_winner_1164 {
                        0 => {
                            __terrane_select_control_1164_1.request_cancel();
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1164_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1164 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        5 /* terrane-site: case.trn:50:30-50:37 */,
                                    ),
                                );
                            }
                        }
                        1 => {
                            __terrane_select_control_1164_0.request_cancel();
                            let _ = __terrane_select_future_1164_0.as_mut().await;
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1164.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1164
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1164 {
                        0 => {
                            let _ = __terrane_select_result_1164_0
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected"))
                            );
                        }
                        1 => {
                            let value: terrane_int_support::Int = __terrane_traced_completion!(
                                __terrane_select_result_1164_1.take()
                                .expect("selected case owns its ready result"),
                                5 /* terrane-site: case.trn:50:30-50:37 */
                            );
                            println!("{}", terrane_scalar_support::scalar_text(&value));
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_3 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_3) => {
                    let mut __terrane_handled_3 = false;
                    if !__terrane_handled_3
                        && __terrane_error_3.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_3 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("caught"))
                        );
                    }
                    if !__terrane_handled_3 {
                        return TerraneCompletion::Error(__terrane_error_3);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_3 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let coercion_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let missing_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_4: TerraneCompletion<()> = async {
            let __terrane_try_4: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1450 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1450: Option<TerraneError> = None;
                    let __terrane_select_control_1450_0 = __terrane_select_control();
                    let mut __terrane_select_future_1450_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_0
                        .clone(), cleanup_coercion(coercion_pair.receiver))
                    );
                    let mut __terrane_select_result_1450_0 = None;
                    let __terrane_select_control_1450_1 = __terrane_select_control();
                    let mut __terrane_select_future_1450_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_1
                        .clone(), cleanup_missing(missing_pair.receiver))
                    );
                    let mut __terrane_select_result_1450_1 = None;
                    let __terrane_select_control_1450_2 = __terrane_select_control();
                    let mut __terrane_select_future_1450_2 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1450_2
                        .clone(), ready())
                    );
                    let mut __terrane_select_result_1450_2 = None;
                    let __terrane_select_winner_1450 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..3usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1450
                                    + __terrane_select_offset) % 3usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1450_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_0 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(0usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    1 => {
                                        match Future::poll(
                                            __terrane_select_future_1450_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_1 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(1usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    2 => {
                                        match Future::poll(
                                            __terrane_select_future_1450_2.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1450_2 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(2usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    _ => {
                                        unreachable!("select candidate is within the case count")
                                    }
                                }
                            }
                            std::task::Poll::Pending
                        })
                        .await;
                    if __terrane_select_winner_1450 == usize::MAX {
                        __terrane_select_control_1450_2.request_cancel();
                        __terrane_select_control_1450_1.request_cancel();
                        __terrane_select_control_1450_0.request_cancel();
                        let _ = __terrane_select_future_1450_2.as_mut().await;
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1450 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    8 /* terrane-site: case.trn:61:18-61:58 */,
                                ),
                            );
                        }
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1450 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    9 /* terrane-site: case.trn:59:18-59:60 */,
                                ),
                            );
                        }
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1450.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1450
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1450)
                            .await;
                    }
                    __terrane_select_cursor_1450 = (__terrane_select_winner_1450
                        + 1usize) % 3usize;
                    match __terrane_select_winner_1450 {
                        0 => {
                            __terrane_select_control_1450_2.request_cancel();
                            __terrane_select_control_1450_1.request_cancel();
                            let _ = __terrane_select_future_1450_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        8 /* terrane-site: case.trn:61:18-61:58 */,
                                    ),
                                );
                            }
                        }
                        1 => {
                            __terrane_select_control_1450_2.request_cancel();
                            __terrane_select_control_1450_0.request_cancel();
                            let _ = __terrane_select_future_1450_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        9 /* terrane-site: case.trn:59:18-59:60 */,
                                    ),
                                );
                            }
                        }
                        2 => {
                            __terrane_select_control_1450_1.request_cancel();
                            __terrane_select_control_1450_0.request_cancel();
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_1
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        8 /* terrane-site: case.trn:61:18-61:58 */,
                                    ),
                                );
                            }
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1450_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1450 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        9 /* terrane-site: case.trn:59:18-59:60 */,
                                    ),
                                );
                            }
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1450.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1450
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1450 {
                        0 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1450_0.take()
                                .expect("selected case owns its ready result"),
                                9 /* terrane-site: case.trn:59:18-59:60 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected coercion"))
                            );
                        }
                        1 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1450_1.take()
                                .expect("selected case owns its ready result"),
                                8 /* terrane-site: case.trn:61:18-61:58 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected missing"))
                            );
                        }
                        2 => {
                            let winner: String = __terrane_select_result_1450_2
                                .take()
                                .expect("selected case owns its ready result");
                            println!("{}", terrane_scalar_support::scalar_text(&winner));
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_4 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_4) => {
                    let mut __terrane_handled_4 = false;
                    if !__terrane_handled_4
                        && __terrane_error_4.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_4 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("cleanup-error-replaced"))
                        );
                    }
                    if !__terrane_handled_4 {
                        return TerraneCompletion::Error(__terrane_error_4);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_4 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        let construction_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let __terrane_completion_5: TerraneCompletion<()> = async {
            let __terrane_try_5: TerraneCompletion<()> = async {
                {
                    let mut __terrane_select_guard_1828 = __terrane_finally_guard();
                    let mut __terrane_select_cleanup_error_1828: Option<TerraneError> = None;
                    let __terrane_select_control_1828_0 = __terrane_select_control();
                    let mut __terrane_select_future_1828_0 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_0
                        .clone(), cleanup_coercion(construction_pair.receiver))
                    );
                    let mut __terrane_select_result_1828_0 = None;
                    let __terrane_select_control_1828_1 = __terrane_select_control();
                    let __terrane_select_unpinned_1828_1 = match || -> Result<
                        _,
                        TerraneError,
                    > {
                        Ok(
                            accept(
                                __terrane_traced_err(
                                    construction_failure(),
                                    11 /* terrane-site: case.trn:73:28-73:49 */,
                                )?,
                            ),
                        )
                    }() {
                        __terrane_select_constructed => {
                            match __terrane_select_constructed {
                                Ok(__terrane_select_future) => __terrane_select_future,
                                Err(__terrane_select_error) => {
                                    __terrane_select_cleanup_error_1828 = Some(
                                        __terrane_select_error,
                                    );
                                    __terrane_select_control_1828_0.request_cancel();
                                    if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                        .as_mut()
                                        .await
                                    {
                                        __terrane_select_cleanup_error_1828 = Some(
                                            __terrane_trace_error(
                                                __terrane_select_error,
                                                12 /* terrane-site: case.trn:71:18-71:64 */,
                                            ),
                                        );
                                    }
                                    __terrane_wait_projected_cleanups().await;
                                    __terrane_select_guard_1828.finish();
                                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                                        .take()
                                    {
                                        return TerraneCompletion::Error(
                                            __terrane_select_cleanup_error,
                                        );
                                    }
                                    unreachable!("select construction failure propagates");
                                }
                            }
                        }
                    };
                    let mut __terrane_select_future_1828_1 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_1
                        .clone(), __terrane_select_unpinned_1828_1)
                    );
                    let mut __terrane_select_result_1828_1 = None;
                    let __terrane_select_control_1828_2 = __terrane_select_control();
                    let mut __terrane_select_future_1828_2 = std::pin::pin!(
                        __terrane_select_operation(__terrane_select_control_1828_2
                        .clone(), accept(late_construction()))
                    );
                    let mut __terrane_select_result_1828_2 = None;
                    let __terrane_select_winner_1828 = std::future::poll_fn(|
                            __terrane_select_context|
                        {
                            if __terrane_cancellation_is_requested() {
                                return std::task::Poll::Ready(usize::MAX);
                            }
                            for __terrane_select_offset in 0..3usize {
                                let __terrane_select_candidate = (__terrane_select_cursor_1828
                                    + __terrane_select_offset) % 3usize;
                                match __terrane_select_candidate {
                                    0 => {
                                        match Future::poll(
                                            __terrane_select_future_1828_0.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_0 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(0usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    1 => {
                                        match Future::poll(
                                            __terrane_select_future_1828_1.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_1 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(1usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    2 => {
                                        match Future::poll(
                                            __terrane_select_future_1828_2.as_mut(),
                                            __terrane_select_context,
                                        ) {
                                            std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                __terrane_select_result_1828_2 = Some(
                                                    __terrane_select_value,
                                                );
                                                return std::task::Poll::Ready(2usize);
                                            }
                                            std::task::Poll::Ready(None) => {
                                                unreachable!(
                                                    "case cancellation starts only after winner selection"
                                                )
                                            }
                                            std::task::Poll::Pending => {}
                                        }
                                    }
                                    _ => {
                                        unreachable!("select candidate is within the case count")
                                    }
                                }
                            }
                            std::task::Poll::Pending
                        })
                        .await;
                    if __terrane_select_winner_1828 == usize::MAX {
                        __terrane_select_control_1828_2.request_cancel();
                        __terrane_select_control_1828_1.request_cancel();
                        __terrane_select_control_1828_0.request_cancel();
                        let _ = __terrane_select_future_1828_2.as_mut().await;
                        let _ = __terrane_select_future_1828_1.as_mut().await;
                        if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                            .as_mut()
                            .await
                        {
                            __terrane_select_cleanup_error_1828 = Some(
                                __terrane_trace_error(
                                    __terrane_select_error,
                                    12 /* terrane-site: case.trn:71:18-71:64 */,
                                ),
                            );
                        }
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_1828.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        __terrane_finish_cancelled_select(__terrane_select_guard_1828)
                            .await;
                    }
                    __terrane_select_cursor_1828 = (__terrane_select_winner_1828
                        + 1usize) % 3usize;
                    match __terrane_select_winner_1828 {
                        0 => {
                            __terrane_select_control_1828_2.request_cancel();
                            __terrane_select_control_1828_1.request_cancel();
                            let _ = __terrane_select_future_1828_2.as_mut().await;
                            let _ = __terrane_select_future_1828_1.as_mut().await;
                        }
                        1 => {
                            __terrane_select_control_1828_2.request_cancel();
                            __terrane_select_control_1828_0.request_cancel();
                            let _ = __terrane_select_future_1828_2.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1828 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        12 /* terrane-site: case.trn:71:18-71:64 */,
                                    ),
                                );
                            }
                        }
                        2 => {
                            __terrane_select_control_1828_1.request_cancel();
                            __terrane_select_control_1828_0.request_cancel();
                            let _ = __terrane_select_future_1828_1.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_1828_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_1828 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        12 /* terrane-site: case.trn:71:18-71:64 */,
                                    ),
                                );
                            }
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                    __terrane_wait_projected_cleanups().await;
                    __terrane_select_guard_1828.finish();
                    if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_1828
                        .take()
                    {
                        return TerraneCompletion::Error(__terrane_select_cleanup_error);
                    }
                    match __terrane_select_winner_1828 {
                        0 => {
                            let _ = __terrane_traced_completion!(
                                __terrane_select_result_1828_0.take()
                                .expect("selected case owns its ready result"),
                                12 /* terrane-site: case.trn:71:18-71:64 */
                            );
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected cleanup winner"))
                            );
                        }
                        1 => {
                            let _ = __terrane_select_result_1828_1
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected construction winner"))
                            );
                        }
                        2 => {
                            let _ = __terrane_select_result_1828_2
                                .take()
                                .expect("selected case owns its ready result");
                            println!(
                                "{}",
                                terrane_scalar_support::scalar_text(&String::from("unexpected late winner"))
                            );
                        }
                        _ => unreachable!("selected winner is within the case count"),
                    }
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_5 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_5) => {
                    let mut __terrane_handled_5 = false;
                    if !__terrane_handled_5
                        && __terrane_error_5.kind == TerraneErrorKind::CoercionError
                    {
                        __terrane_handled_5 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("construction-cleanup-replaced"))
                        );
                    }
                    if !__terrane_handled_5 {
                        return TerraneCompletion::Error(__terrane_error_5);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_5 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
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
