// Generated deterministically by Terrane <version>.
// Runtime support: mutable_callable.rs, async_mutable_state.rs, consuming_callable.rs, async_native.rs, executor_parallel.rs, channels.rs, tasks_native_parallel.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
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
// Source: case.trn
// Namespace: callable-invocation-modes
fn double(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
}
async fn double_later(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
}
#[derive(Clone)]
pub struct Accumulator {
    pub total: terrane_int_support::Int,
}
impl Accumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn add(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int {
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
}
#[derive(Clone)]
pub struct Ticket {
    pub message: String,
}
impl Ticket {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("redeemed"),
        }
    }
    pub fn redeem(self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct AsyncAccumulator {
    pub total: terrane_int_support::Int,
}
impl AsyncAccumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub async fn add(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
}
#[derive(Clone)]
pub struct GatedAccumulator {
    pub total: terrane_int_support::Int,
}
impl GatedAccumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub async fn add(
        &mut self,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int> = __terrane_await(
                Box::pin(started.send(terrane_int_support::Int::from(1_i128))),
            )
            .await;
        if !sent.accepted {
            return terrane_int_support::Int::from(-1_i128);
        }
        let released: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
                Box::pin(gate.receive()),
            )
            .await;
        if !released.available {
            return terrane_int_support::Int::from(-2_i128);
        }
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
}
fn main() {
    __terrane_run(async move {
        let counter: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        let step: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = {
            let mut counter = counter.clone();
            TerraneMutableCallable::new(move |
                (delta,): (terrane_int_support::Int,),
            | -> terrane_int_support::Int {
                counter = counter.clone() + delta.clone();
                return counter.clone();
            })
        };
        let message: String = String::from("finished");
        let finish: TerraneConsumingCallable<(), String> = {
            let message = message.clone();
            TerraneConsumingCallable::new(move |(): ()| -> String {
                return message;
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(2_i128),)))
        );
        let copy: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = step.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = step; "mutable"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = step; "mutable"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&copy
            .call((terrane_int_support::Int::from(10_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = finish; "consuming"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = finish; "consuming"
            .to_owned() })
        );
        println!("{}", terrane_scalar_support::scalar_text(&finish.call(())));
        let operation: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = TerraneMutableCallable::new(move |
            (argument_0,): (terrane_int_support::Int,)|
        double(argument_0));
        println!(
            "{}", terrane_scalar_support::scalar_text(&operation
            .call((terrane_int_support::Int::from(6_i128),)))
        );
        let async_counter: terrane_int_support::Int = terrane_int_support::Int::from(
            0_i128,
        );
        let async_step: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = {
            let async_counter = TerraneAsyncMutableState::new(async_counter.clone());
            let __terrane_invocation = TerraneAsyncInvocationGate::new();
            TerraneMutableCallable::new(move |
                (delta,): (terrane_int_support::Int,),
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let async_counter = async_counter.share();
                let __terrane_invocation = __terrane_invocation.share();
                Box::pin(async move {
                    let _invocation = __terrane_invocation.enter().await;
                    {
                        let callable_capture_value = async_counter.snapshot()
                            + delta.clone();
                        async_counter.replace(callable_capture_value);
                    }
                    return async_counter.snapshot();
                })
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(2_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(3_i128),))). await)
        );
        let async_copy: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = async_step.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(1_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_copy
            .call((terrane_int_support::Int::from(10_i128),))). await)
        );
        let asynchronous: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = TerraneMutableCallable::new(move |
            (argument_0,): (terrane_int_support::Int,),
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(double_later(argument_0))
        });
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(asynchronous
            .call((terrane_int_support::Int::from(7_i128),))). await)
        );
        let value: Accumulator = Accumulator::terrane_construct();
        let bound: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = {
            let mut receiver = value.clone();
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            receiver.add(argument_0))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound
            .call((terrane_int_support::Int::from(5_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound
            .call((terrane_int_support::Int::from(7_i128),)))
        );
        let _ = &value;
        let value: Ticket = Ticket::terrane_construct();
        let redemption: TerraneConsumingCallable<(), String> = {
            let receiver = value.clone();
            TerraneConsumingCallable::new(move |(): ()| receiver.redeem())
        };
        println!("{}", terrane_scalar_support::scalar_text(&redemption.call(())));
        let shared: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        > = {
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> terrane_int_support::Int {
                return value.clone() + terrane_int_support::Int::from(1_i128);
            })
        };
        let adapted: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = {
            let callable = shared.clone();
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            callable(argument_0))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&adapted
            .call((terrane_int_support::Int::from(8_i128),)))
        );
        let async_value: AsyncAccumulator = AsyncAccumulator::terrane_construct();
        let async_bound: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = {
            let receiver = TerraneAsyncMutableState::new(async_value);
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                let receiver = receiver.share();
                Box::pin(async move {
                    receiver
                        .with_receiver(move |receiver| Box::pin(async move {
                            receiver.add(argument_0).await
                        }))
                        .await
                })
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_bound
            .call((terrane_int_support::Int::from(2_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_bound
            .call((terrane_int_support::Int::from(3_i128),))). await)
        );
        let one_shot: TerraneConsumingCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        > = {
            let callable = adapted.clone();
            TerraneConsumingCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            callable.call((argument_0,)))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&one_shot
            .call((terrane_int_support::Int::from(10_i128),)))
        );
        let declared_mutable: TerraneMutableCallable<(), terrane_int_support::Int> = {
            TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
                return terrane_int_support::Int::from(9_i128);
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = declared_mutable;
            "mutable".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = declared_mutable;
            "shared".to_owned() })
        );
        println!("{}", terrane_scalar_support::scalar_text(&declared_mutable.call(())));
        let gate: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let started: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let gated_value: GatedAccumulator = GatedAccumulator::terrane_construct();
        let gated_bound: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = {
            let receiver = TerraneAsyncMutableState::new(gated_value);
            TerraneMutableCallable::new(move |
                (
                    argument_0,
                    argument_1,
                    argument_2,
                ): (
                    terrane_int_support::Int,
                    TerraneChannelSender<terrane_int_support::Int>,
                    TerraneChannelReceiver<terrane_int_support::Int>,
                ),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                let receiver = receiver.share();
                Box::pin(async move {
                    receiver
                        .with_receiver(move |receiver| Box::pin(async move {
                            receiver.add(argument_0, argument_1, argument_2).await
                        }))
                        .await
                })
            })
        };
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let child: TerraneScopedTask<terrane_int_support::Int> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = gated_bound
                .call((
                    terrane_int_support::Int::from(4_i128),
                    started.sender,
                    gate.receiver,
                ));
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
        let observed: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
                started.receiver.receive(),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&observed.available));
        let gated_copy: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        > = gated_bound.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = gated_copy; "mutable"
            .to_owned() })
        );
        let released: TerraneChannelSendOutcome<terrane_int_support::Int> = __terrane_await(
                gate.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&released.accepted));
        let outcome: TerraneTaskOutcome<terrane_int_support::Int> = __terrane_await(
                scope.join(child),
            )
            .await;
        let result: Option<terrane_int_support::Int> = outcome.value.clone();
        if result.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&* result.as_ref()
                .expect("semantic optional narrowing"))
            );
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
