// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, async_select.rs, executor_parallel.rs, time_base.rs, platform_capability_types.rs, platform_result_type.rs, platform_capability_base.rs, platform_time.rs
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
    pub static DESCRIPTORS: [&str; 1] = ["invalid-duration"];
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
    pub static FILES: [&str; 2] = ["case.trn", "core/time.trn"];
    pub static FUNCTIONS: [&str; 10] = [
        "/time-sleep-ticker::main",
        "/core/time::multiply",
        "/core/time::seconds",
        "/core/time::milliseconds",
        "/core/time::microseconds",
        "/core/time::nanoseconds",
        "/core/time::duration-until",
        "/core/time::at",
        "/core/time::sleep-until",
        "/core/time::interval",
    ];
    pub static SITES: [Site; 20] = [
        /* terrane-site-row: site 0: /time-sleep-ticker::main (case.trn:6:13-6:38) */
        { Site { function: 0, file: 0, line: 6, column: 13, end_line: 6, end_column: 38 } },
        /* terrane-site-row: site 1: /time-sleep-ticker::main (case.trn:7:12-7:32) */
        { Site { function: 0, file: 0, line: 7, column: 12, end_line: 7, end_column: 32 } },
        /* terrane-site-row: site 2: /time-sleep-ticker::main (case.trn:11:13-11:46) */
        { Site { function: 0, file: 0, line: 11, column: 13, end_line: 11, end_column: 46 } },
        /* terrane-site-row: site 3: /time-sleep-ticker::main (case.trn:13:31-13:57) */
        { Site { function: 0, file: 0, line: 13, column: 31, end_line: 13, end_column: 57 } },
        /* terrane-site-row: site 4: /time-sleep-ticker::main (case.trn:14:34-14:61) */
        { Site { function: 0, file: 0, line: 14, column: 34, end_line: 14, end_column: 61 } },
        /* terrane-site-row: site 5: /time-sleep-ticker::main (case.trn:18:13-18:50) */
        { Site { function: 0, file: 0, line: 18, column: 13, end_line: 18, end_column: 50 } },
        /* terrane-site-row: site 6: /time-sleep-ticker::main (case.trn:20:38-20:63) */
        { Site { function: 0, file: 0, line: 20, column: 38, end_line: 20, end_column: 63 } },
        /* terrane-site-row: site 7: /time-sleep-ticker::main (case.trn:20:20-20:64) */
        { Site { function: 0, file: 0, line: 20, column: 20, end_line: 20, end_column: 64 } },
        /* terrane-site-row: site 8: /time-sleep-ticker::main (case.trn:22:36-22:61) */
        { Site { function: 0, file: 0, line: 22, column: 36, end_line: 22, end_column: 61 } },
        /* terrane-site-row: site 9: /time-sleep-ticker::main (case.trn:25:13-25:65) */
        { Site { function: 0, file: 0, line: 25, column: 13, end_line: 25, end_column: 65 } },
        /* terrane-site-row: site 10: /time-sleep-ticker::main (case.trn:35:13-35:56) */
        { Site { function: 0, file: 0, line: 35, column: 13, end_line: 35, end_column: 56 } },
        /* terrane-site-row: site 11: /core/time::multiply (core/time.trn:61:13-61:45) */
        { Site { function: 1, file: 1, line: 61, column: 13, end_line: 61, end_column: 45 } },
        /* terrane-site-row: site 12: /core/time::seconds (core/time.trn:37:13-37:45) */
        { Site { function: 2, file: 1, line: 37, column: 13, end_line: 37, end_column: 45 } },
        /* terrane-site-row: site 13: /core/time::milliseconds (core/time.trn:42:13-42:45) */
        { Site { function: 3, file: 1, line: 42, column: 13, end_line: 42, end_column: 45 } },
        /* terrane-site-row: site 14: /core/time::microseconds (core/time.trn:47:13-47:45) */
        { Site { function: 4, file: 1, line: 47, column: 13, end_line: 47, end_column: 45 } },
        /* terrane-site-row: site 15: /core/time::nanoseconds (core/time.trn:52:13-52:45) */
        { Site { function: 5, file: 1, line: 52, column: 13, end_line: 52, end_column: 45 } },
        /* terrane-site-row: site 16: /core/time::duration-until (core/time.trn:75:13-75:45) */
        { Site { function: 6, file: 1, line: 75, column: 13, end_line: 75, end_column: 45 } },
        /* terrane-site-row: site 17: /core/time::at (core/time.trn:104:13-104:45) */
        { Site { function: 7, file: 1, line: 104, column: 13, end_line: 104, end_column: 45 } },
        /* terrane-site-row: site 18: /core/time::sleep-until (core/time.trn:157:13-157:45) */
        { Site { function: 8, file: 1, line: 157, column: 13, end_line: 157, end_column: 45 } },
        /* terrane-site-row: site 19: /core/time::interval (core/time.trn:168:13-168:45) */
        { Site { function: 9, file: 1, line: 168, column: 13, end_line: 168, end_column: 45 } },
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
// Namespace: time-sleep-ticker
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_1074 = 0usize;
        let short: Duration = __terrane_traced(
            Duration::terrane_static_milliseconds(
                terrane_int_support::Int::from(2_i128),
            ),
            0 /* terrane-site: case.trn:6:13-6:38 */,
        );
        let long: Duration = __terrane_traced(
            Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),
            1 /* terrane-site: case.trn:7:12-7:32 */,
        );
        let started: MonotonicInstant = Clock::terrane_static_monotonic();
        let ignored: () = __terrane_await(
                terrane_time_sleep_after(short.total_nanoseconds.clone()),
            )
            .await;
        let _ = &ignored;
        let slept: MonotonicInstant = Clock::terrane_static_monotonic();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(__terrane_traced(started
            .duration_until(&slept), 2 /* terrane-site: case.trn:11:13-11:46 */)
            .total_nanoseconds.clone() >= short.total_nanoseconds.clone()))
        );
        let captured = terrane_time_sleep_after(
            __terrane_traced(
                    Duration::terrane_static_milliseconds(
                        terrane_int_support::Int::from(50_i128),
                    ),
                    3 /* terrane-site: case.trn:13:31-13:57 */,
                )
                .total_nanoseconds
                .clone(),
        );
        let delay: () = __terrane_await(
                terrane_time_sleep_after(
                    __terrane_traced(
                            Duration::terrane_static_milliseconds(
                                terrane_int_support::Int::from(150_i128),
                            ),
                            4 /* terrane-site: case.trn:14:34-14:61 */,
                        )
                        .total_nanoseconds
                        .clone(),
                ),
            )
            .await;
        let _ = &delay;
        let resumed: MonotonicInstant = Clock::terrane_static_monotonic();
        let captured_result: () = __terrane_await(captured).await;
        let _ = &captured_result;
        let completed: MonotonicInstant = Clock::terrane_static_monotonic();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(__terrane_traced(resumed
            .duration_until(&completed), 5 /* terrane-site: case.trn:18:13-18:50 */)
            .total_nanoseconds.clone() < terrane_int_support::Int::from(25000000_i128)))
        );
        let mut timer: Ticker = __terrane_traced(
            Clock::terrane_static_interval(
                __terrane_traced(
                    Duration::terrane_static_milliseconds(
                        terrane_int_support::Int::from(1_i128),
                    ),
                    6 /* terrane-site: case.trn:20:38-20:63 */,
                ),
            ),
            7 /* terrane-site: case.trn:20:20-20:64 */,
        );
        let first: Tick = __terrane_await((&mut timer).next()).await;
        let latency: () = __terrane_await(
                terrane_time_sleep_after(
                    __terrane_traced(
                            Duration::terrane_static_milliseconds(
                                terrane_int_support::Int::from(5_i128),
                            ),
                            8 /* terrane-site: case.trn:22:36-22:61 */,
                        )
                        .total_nanoseconds
                        .clone(),
                ),
            )
            .await;
        let _ = &latency;
        let second: Tick = __terrane_await((&mut timer).next()).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&(first.count.clone() >=
            terrane_int_support::Int::from(1_i128))),
            terrane_scalar_support::scalar_text(&(second.count.clone() >=
            terrane_int_support::Int::from(4_i128)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&(__terrane_traced(first.scheduled
            .duration_until(&second.scheduled), 9 /* terrane-site: case.trn:25:13-25:65 */).total_nanoseconds.clone() >=
            terrane_int_support::Int::from(4000000_i128)))
        );
        timer.destruct();
        let race_started: MonotonicInstant = Clock::terrane_static_monotonic();
        {
            let mut __terrane_select_guard_1074 = __terrane_finally_guard();
            let __terrane_select_control_1074_0 = __terrane_select_control();
            let mut __terrane_select_future_1074_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1074_0.clone(),
                terrane_time_sleep_after(short.total_nanoseconds.clone()))
            );
            let mut __terrane_select_result_1074_0 = None;
            let __terrane_select_control_1074_1 = __terrane_select_control();
            let mut __terrane_select_future_1074_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1074_1.clone(),
                terrane_time_sleep_after(long.total_nanoseconds.clone()))
            );
            let mut __terrane_select_result_1074_1 = None;
            let __terrane_select_winner_1074 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1074
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1074_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1074_0 = Some(
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
                                    __terrane_select_future_1074_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1074_1 = Some(
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
            if __terrane_select_winner_1074 == usize::MAX {
                __terrane_select_control_1074_1.request_cancel();
                __terrane_select_control_1074_0.request_cancel();
                let _ = __terrane_select_future_1074_1.as_mut().await;
                let _ = __terrane_select_future_1074_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1074.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_1074).await;
            }
            __terrane_select_cursor_1074 = (__terrane_select_winner_1074 + 1usize)
                % 2usize;
            match __terrane_select_winner_1074 {
                0 => {
                    __terrane_select_control_1074_1.request_cancel();
                    let _ = __terrane_select_future_1074_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_1074_0.request_cancel();
                    let _ = __terrane_select_future_1074_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1074.finish();
            match __terrane_select_winner_1074 {
                0 => {
                    let done: () = __terrane_select_result_1074_0
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&(done == ())));
                }
                1 => {
                    let done: () = __terrane_select_result_1074_1
                        .take()
                        .expect("selected case owns its ready result");
                    let _ = &done;
                    println!("{}", terrane_scalar_support::scalar_text(&false));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        let race_ended: MonotonicInstant = Clock::terrane_static_monotonic();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(__terrane_traced(race_started
            .duration_until(&race_ended), 10 /* terrane-site: case.trn:35:13-35:56 */)
            .total_nanoseconds.clone() < long.total_nanoseconds.clone()))
        );
    });
}
// Source: core/time.trn
// Namespace: core/time
#[derive(Clone)]
pub struct InvalidDuration {
    pub message: String,
}
impl InvalidDuration {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("duration must be exact and non-negative"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DurationSubtraction {
    pub total_nanoseconds: terrane_int_support::Int,
}
impl DurationSubtraction {
    pub fn terrane_construct(total: terrane_int_support::Int) -> Self {
        let mut value = Self {
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        value.construct(total);
        value
    }
    pub fn construct(&mut self, total: terrane_int_support::Int) {
        self.total_nanoseconds = total.clone();
    }
    pub fn checked(&self, other: Duration) -> Option<Duration> {
        if self.total_nanoseconds.clone() < other.total_nanoseconds.clone() {
            return None;
        }
        let difference: terrane_int_support::Int = self.total_nanoseconds.clone()
            - other.total_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(&difference, 1000000000),
                terrane_platform_time_mod(&difference, 1000000000),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration {
    pub seconds: terrane_int_support::Int,
    pub nanoseconds: terrane_int_support::Int,
    pub total_nanoseconds: terrane_int_support::Int,
    pub subtract: DurationSubtraction,
}
impl Duration {
    pub fn terrane_construct(
        whole_seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
            subtract: DurationSubtraction::terrane_construct(
                terrane_int_support::Int::from(0_i128),
            ),
        };
        value.construct(whole_seconds, fractional_nanoseconds);
        value
    }
    pub fn construct(
        &mut self,
        whole_seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) {
        self.seconds = whole_seconds.clone();
        self.nanoseconds = fractional_nanoseconds.clone();
        self.total_nanoseconds = whole_seconds.clone()
            * terrane_int_support::Int::from(1000000000_i128)
            + fractional_nanoseconds.clone();
        self.subtract = DurationSubtraction::terrane_construct(
            self.total_nanoseconds.clone(),
        );
    }
    pub fn add(&self, other: Duration) -> Duration {
        let fractional: terrane_int_support::Int = self.nanoseconds.clone()
            + other.nanoseconds.clone();
        return Duration::terrane_construct(
            self.seconds.clone() + other.seconds.clone()
                + terrane_platform_time_div(&fractional, 1000000000),
            terrane_platform_time_mod(&fractional, 1000000000),
        );
    }
    pub fn multiply(
        &self,
        multiplier: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if multiplier.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    11 /* terrane-site: core/time.trn:61:13-61:45 */,
                )
            });
        }
        let total: terrane_int_support::Int = self.total_nanoseconds.clone()
            * multiplier.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&total, 1000000000),
                terrane_platform_time_mod(&total, 1000000000),
            ),
        );
    }
    pub fn terrane_static_seconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    12 /* terrane-site: core/time.trn:37:13-37:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                value.clone(),
                terrane_int_support::Int::from(0_i128),
            ),
        );
    }
    pub fn terrane_static_milliseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    13 /* terrane-site: core/time.trn:42:13-42:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000),
                terrane_platform_time_mod(&value, 1000)
                    * terrane_int_support::Int::from(1000000_i128),
            ),
        );
    }
    pub fn terrane_static_microseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    14 /* terrane-site: core/time.trn:47:13-47:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000000),
                terrane_platform_time_mod(&value, 1000000)
                    * terrane_int_support::Int::from(1000_i128),
            ),
        );
    }
    pub fn terrane_static_nanoseconds(
        value: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        if value.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    15 /* terrane-site: core/time.trn:52:13-52:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000000000),
                terrane_platform_time_mod(&value, 1000000000),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MonotonicInstant {
    pub domain: terrane_int_support::Int,
    pub elapsed_nanoseconds: terrane_int_support::Int,
}
impl MonotonicInstant {
    pub fn terrane_construct(
        runtime_domain: terrane_int_support::Int,
        elapsed: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            domain: terrane_int_support::Int::from(0_i128),
            elapsed_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        value.construct(runtime_domain, elapsed);
        value
    }
    pub fn construct(
        &mut self,
        runtime_domain: terrane_int_support::Int,
        elapsed: terrane_int_support::Int,
    ) {
        self.domain = runtime_domain.clone();
        self.elapsed_nanoseconds = elapsed.clone();
    }
    pub fn duration_until(
        &self,
        later: &MonotonicInstant,
    ) -> Result<Duration, TerraneError> {
        if self.domain.clone() != later.domain.clone().clone()
            || later.elapsed_nanoseconds.clone().clone()
                < self.elapsed_nanoseconds.clone()
        {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    16 /* terrane-site: core/time.trn:75:13-75:45 */,
                )
            });
        }
        let elapsed: terrane_int_support::Int = later.elapsed_nanoseconds.clone().clone()
            - self.elapsed_nanoseconds.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed, 1000000000),
                terrane_platform_time_mod(&elapsed, 1000000000),
            ),
        );
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Instant {
    pub unix_seconds: terrane_int_support::Int,
    pub nanoseconds: terrane_int_support::Int,
}
impl Instant {
    pub fn terrane_construct(
        seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            unix_seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        value.construct(seconds, fractional_nanoseconds);
        value
    }
    pub fn construct(
        &mut self,
        seconds: terrane_int_support::Int,
        fractional_nanoseconds: terrane_int_support::Int,
    ) {
        self.unix_seconds = seconds.clone();
        self.nanoseconds = fractional_nanoseconds.clone();
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Deadline {
    pub expires_at: MonotonicInstant,
}
impl Deadline {
    pub fn terrane_construct(target: MonotonicInstant) -> Self {
        let mut value = Self {
            expires_at: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
        };
        value.construct(target);
        value
    }
    pub fn construct(&mut self, target: MonotonicInstant) {
        self.expires_at = target.clone();
    }
    pub fn remaining(&self) -> Option<Duration> {
        let now: MonotonicInstant = Clock::terrane_static_monotonic();
        if self.expires_at.elapsed_nanoseconds.clone() <= now.elapsed_nanoseconds.clone()
        {
            return None;
        }
        let elapsed: terrane_int_support::Int = self
            .expires_at
            .elapsed_nanoseconds
            .clone() - now.elapsed_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed, 1000000000),
                terrane_platform_time_mod(&elapsed, 1000000000),
            ),
        );
    }
    pub fn expired(&self) -> bool {
        return self.remaining().is_none();
    }
    pub fn terrane_static_at(
        target: MonotonicInstant,
    ) -> Result<Deadline, TerraneError> {
        if target.domain.clone() != terrane_platform_time_domain() {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    17 /* terrane-site: core/time.trn:104:13-104:45 */,
                )
            });
        }
        return Ok(Deadline::terrane_construct(target.clone()));
    }
}
#[derive(Clone)]
pub struct Tick {
    pub scheduled: MonotonicInstant,
    pub observed: MonotonicInstant,
    pub count: terrane_int_support::Int,
}
impl Tick {
    pub fn terrane_construct(
        scheduled_at: MonotonicInstant,
        observed_at: MonotonicInstant,
        expirations: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            scheduled: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            observed: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            count: terrane_int_support::Int::from(0_i128),
        };
        value.construct(scheduled_at, observed_at, expirations);
        value
    }
    pub fn construct(
        &mut self,
        scheduled_at: MonotonicInstant,
        observed_at: MonotonicInstant,
        expirations: terrane_int_support::Int,
    ) {
        self.scheduled = scheduled_at.clone();
        self.observed = observed_at.clone();
        self.count = expirations.clone();
    }
}
#[derive(Clone)]
pub struct Ticker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub anchor: MonotonicInstant,
    pub period: Duration,
    pub next_index: terrane_int_support::Int,
    pub closed: bool,
}
impl Ticker {
    pub fn terrane_construct(interval: Duration) -> Self {
        let mut value = Self {
            anchor: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            period: Duration::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            next_index: terrane_int_support::Int::from(1_i128),
            closed: false,
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        value.construct(interval);
        value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, interval: Duration) {
        self.anchor = Clock::terrane_static_monotonic();
        self.period = interval.clone();
    }
    pub async fn next(&mut self) -> Tick {
        let period_total: terrane_int_support::Int = self
            .period
            .total_nanoseconds
            .clone();
        let scheduled_nanoseconds: terrane_int_support::Int = self
            .anchor
            .elapsed_nanoseconds
            .clone() + period_total.clone() * self.next_index.clone();
        let ignored: () = __terrane_await(
                terrane_platform_time_sleep_until(scheduled_nanoseconds),
            )
            .await;
        let _ = &ignored;
        let observed: MonotonicInstant = Clock::terrane_static_monotonic();
        let elapsed: terrane_int_support::Int = observed.elapsed_nanoseconds.clone()
            - self.anchor.elapsed_nanoseconds.clone();
        let mut observed_index: terrane_int_support::Int = terrane_platform_time_div(
            &elapsed,
            period_total,
        );
        if observed_index.clone() < self.next_index.clone() {
            observed_index = self.next_index.clone();
        }
        let count: terrane_int_support::Int = observed_index.clone()
            - self.next_index.clone() + terrane_int_support::Int::from(1_i128);
        let delivered: MonotonicInstant = MonotonicInstant::terrane_construct(
            self.anchor.domain.clone(),
            self.anchor.elapsed_nanoseconds.clone()
                + self.period.total_nanoseconds.clone() * observed_index.clone(),
        );
        self.next_index = observed_index.clone()
            + terrane_int_support::Int::from(1_i128);
        return Tick::terrane_construct(delivered, observed.clone(), count.clone());
    }
    pub fn destruct(&mut self) {
        self.closed = true;
    }
}
impl Drop for Ticker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) == 1 {
            self.destruct();
        }
    }
}
#[derive(Clone)]
pub struct Clock {}
impl Clock {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_wall() -> Instant {
        let raw: TerranePlatformResult = terrane_platform_time_wall();
        return Instant::terrane_construct(
            terrane_platform_time_wall_seconds(&raw),
            terrane_platform_time_wall_nanoseconds(&raw),
        );
    }
    pub fn terrane_static_monotonic() -> MonotonicInstant {
        return MonotonicInstant::terrane_construct(
            terrane_platform_time_domain(),
            terrane_platform_time_monotonic(),
        );
    }
    pub async fn terrane_static_sleep(elapsed: Duration) {
        let target: terrane_int_support::Int = terrane_platform_time_monotonic()
            + elapsed.total_nanoseconds.clone();
        return __terrane_await(terrane_platform_time_sleep_until(target)).await;
    }
    pub async fn terrane_static_sleep_until(
        target: MonotonicInstant,
    ) -> Result<(), TerraneError> {
        if target.domain.clone() != terrane_platform_time_domain() {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    18 /* terrane-site: core/time.trn:157:13-157:45 */,
                )
            });
        }
        return Ok(
            __terrane_await(
                    terrane_platform_time_sleep_until(target.elapsed_nanoseconds),
                )
                .await,
        );
    }
    pub fn terrane_static_deadline(elapsed: Duration) -> Deadline {
        let now: MonotonicInstant = Clock::terrane_static_monotonic();
        let target: MonotonicInstant = MonotonicInstant::terrane_construct(
            now.domain.clone(),
            now.elapsed_nanoseconds.clone() + elapsed.total_nanoseconds.clone(),
        );
        return Deadline::terrane_construct(target);
    }
    pub fn terrane_static_interval(period: Duration) -> Result<Ticker, TerraneError> {
        if period.total_nanoseconds.clone() == terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    19 /* terrane-site: core/time.trn:168:13-168:45 */,
                )
            });
        }
        return Ok(Ticker::terrane_construct(period.clone()));
    }
}
