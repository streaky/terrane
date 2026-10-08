// Generated deterministically by Terrane <version>.
// Runtime support: mutable_callable.rs, async_mutable_state.rs, async_native.rs, executor_parallel.rs, channels.rs, tasks_native_parallel.rs, time_inactive.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs, consuming_callable.rs
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
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
                std::boxed::Box::new(TerraneErrorDetail {
                    message: None,
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
                std::boxed::Box::new(TerraneErrorDetail {
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
        let sent_terrane_f0_s756: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let released_terrane_f0_s830: TerraneChannelReceiveOutcome<
            terrane_int_support::Int,
        >;
        sent_terrane_f0_s756 = __terrane_await(
                Box::pin(started.send(terrane_int_support::Int::from(1_i128))),
            )
            .await;
        if !sent_terrane_f0_s756.accepted {
            return terrane_int_support::Int::from(-1_i128);
        }
        released_terrane_f0_s830 = __terrane_await(Box::pin(gate.receive())).await;
        if !released_terrane_f0_s830.available {
            return terrane_int_support::Int::from(-2_i128);
        }
        self.total = self.total.clone() + delta.clone();
        return self.total.clone();
    }
}
fn main() {
    __terrane_run(async move {
        let counter_terrane_f0_s989: terrane_int_support::Int;
        let step_terrane_f0_s1003: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let message_terrane_f0_s1093: String;
        let finish_terrane_f0_s1115: TerraneConsumingCallable<(), String>;
        let copy_terrane_f0_s1215: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let operation_terrane_f0_s1423: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let async_counter_terrane_f0_s1501: terrane_int_support::Int;
        let async_step_terrane_f0_s1521: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let async_copy_terrane_f0_s1703: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let asynchronous_terrane_f0_s1792: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let value_terrane_f0_s1894: Accumulator;
        let bound_terrane_f0_s1938: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let value_terrane_f0_s2031: Ticket;
        let redemption_terrane_f0_s2085: TerraneConsumingCallable<(), String>;
        let shared_terrane_f0_s2165: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >;
        let adapted_terrane_f0_s2221: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let async_value_terrane_f0_s2295: AsyncAccumulator;
        let async_bound_terrane_f0_s2357: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let one_shot_terrane_f0_s2492: TerraneConsumingCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let declared_mutable_terrane_f0_s2572: TerraneMutableCallable<
            (),
            terrane_int_support::Int,
        >;
        let gate_terrane_f0_s2747: TerraneChannelPair<terrane_int_support::Int>;
        let started_terrane_f0_s2787: TerraneChannelPair<terrane_int_support::Int>;
        let gated_value_terrane_f0_s2830: GatedAccumulator;
        let gated_bound_terrane_f0_s2874: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let scope_terrane_f0_s2997: TerraneTaskScope;
        let child_terrane_f0_s3019: TerraneScopedTask<terrane_int_support::Int>;
        let observed_terrane_f0_s3090: TerraneChannelReceiveOutcome<
            terrane_int_support::Int,
        >;
        let gated_copy_terrane_f0_s3163: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let released_terrane_f0_s3226: TerraneChannelSendOutcome<
            terrane_int_support::Int,
        >;
        let outcome_terrane_f0_s3292: TerraneTaskOutcome<terrane_int_support::Int>;
        let result_terrane_f0_s3328: Option<terrane_int_support::Int>;
        counter_terrane_f0_s989 = terrane_int_support::Int::from(0_i128);
        step_terrane_f0_s1003 = {
            let mut counter_terrane_f0_s989 = counter_terrane_f0_s989.clone();
            TerraneMutableCallable::new(move |
                (delta,): (terrane_int_support::Int,),
            | -> terrane_int_support::Int {
                counter_terrane_f0_s989 = counter_terrane_f0_s989.clone()
                    + delta.clone();
                return counter_terrane_f0_s989.clone();
            })
        };
        message_terrane_f0_s1093 = String::from("finished");
        finish_terrane_f0_s1115 = {
            let message_terrane_f0_s1093 = message_terrane_f0_s1093.clone();
            TerraneConsumingCallable::new(move |(): ()| -> String {
                return message_terrane_f0_s1093;
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&step_terrane_f0_s1003
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&step_terrane_f0_s1003
            .call((terrane_int_support::Int::from(2_i128),)))
        );
        copy_terrane_f0_s1215 = step_terrane_f0_s1003.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = step_terrane_f0_s1003;
            "mutable".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = step_terrane_f0_s1003;
            "mutable".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&step_terrane_f0_s1003
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&copy_terrane_f0_s1215
            .call((terrane_int_support::Int::from(10_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = finish_terrane_f0_s1115;
            "consuming".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = finish_terrane_f0_s1115;
            "consuming".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&finish_terrane_f0_s1115.call(()))
        );
        operation_terrane_f0_s1423 = TerraneMutableCallable::new(move |
            (argument_0,): (terrane_int_support::Int,)|
        double(argument_0));
        println!(
            "{}", terrane_scalar_support::scalar_text(&operation_terrane_f0_s1423
            .call((terrane_int_support::Int::from(6_i128),)))
        );
        async_counter_terrane_f0_s1501 = terrane_int_support::Int::from(0_i128);
        async_step_terrane_f0_s1521 = {
            let async_counter_terrane_f0_s1501 = TerraneAsyncMutableState::new(
                async_counter_terrane_f0_s1501.clone(),
            );
            let __terrane_invocation = TerraneAsyncInvocationGate::new();
            TerraneMutableCallable::new(move |
                (delta,): (terrane_int_support::Int,),
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let async_counter_terrane_f0_s1501 = async_counter_terrane_f0_s1501
                    .share();
                let __terrane_invocation = __terrane_invocation.share();
                Box::pin(async move {
                    let _invocation = __terrane_invocation.enter().await;
                    {
                        let callable_capture_value = async_counter_terrane_f0_s1501
                            .snapshot() + delta.clone();
                        async_counter_terrane_f0_s1501.replace(callable_capture_value);
                    }
                    return async_counter_terrane_f0_s1501.snapshot();
                })
            })
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(async_step_terrane_f0_s1521
            .call((terrane_int_support::Int::from(2_i128),))). await)
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(async_step_terrane_f0_s1521
            .call((terrane_int_support::Int::from(3_i128),))). await)
        );
        async_copy_terrane_f0_s1703 = async_step_terrane_f0_s1521.clone();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(async_step_terrane_f0_s1521
            .call((terrane_int_support::Int::from(1_i128),))). await)
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(async_copy_terrane_f0_s1703
            .call((terrane_int_support::Int::from(10_i128),))). await)
        );
        asynchronous_terrane_f0_s1792 = TerraneMutableCallable::new(move |
            (argument_0,): (terrane_int_support::Int,),
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(double_later(argument_0))
        });
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(asynchronous_terrane_f0_s1792
            .call((terrane_int_support::Int::from(7_i128),))). await)
        );
        value_terrane_f0_s1894 = Accumulator::terrane_construct();
        bound_terrane_f0_s1938 = {
            let mut receiver = value_terrane_f0_s1894;
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            receiver.add(argument_0))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound_terrane_f0_s1938
            .call((terrane_int_support::Int::from(5_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound_terrane_f0_s1938
            .call((terrane_int_support::Int::from(7_i128),)))
        );
        value_terrane_f0_s2031 = Ticket::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound_terrane_f0_s1938
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        redemption_terrane_f0_s2085 = {
            let receiver = value_terrane_f0_s2031.clone();
            TerraneConsumingCallable::new(move |(): ()| receiver.redeem())
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&redemption_terrane_f0_s2085
            .call(()))
        );
        shared_terrane_f0_s2165 = {
            std::sync::Arc::new(move |
                value: terrane_int_support::Int,
            | -> terrane_int_support::Int {
                return value.clone() + terrane_int_support::Int::from(1_i128);
            })
        };
        adapted_terrane_f0_s2221 = {
            let callable = shared_terrane_f0_s2165.clone();
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            callable(argument_0))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&adapted_terrane_f0_s2221
            .call((terrane_int_support::Int::from(8_i128),)))
        );
        async_value_terrane_f0_s2295 = AsyncAccumulator::terrane_construct();
        async_bound_terrane_f0_s2357 = {
            let receiver = TerraneAsyncMutableState::new(async_value_terrane_f0_s2295);
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
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(async_bound_terrane_f0_s2357
            .call((terrane_int_support::Int::from(2_i128),))). await)
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_await(async_bound_terrane_f0_s2357
            .call((terrane_int_support::Int::from(3_i128),))). await)
        );
        one_shot_terrane_f0_s2492 = {
            let callable = adapted_terrane_f0_s2221.clone();
            TerraneConsumingCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            callable.call((argument_0,)))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&one_shot_terrane_f0_s2492
            .call((terrane_int_support::Int::from(10_i128),)))
        );
        declared_mutable_terrane_f0_s2572 = {
            TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
                return terrane_int_support::Int::from(9_i128);
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ =
            declared_mutable_terrane_f0_s2572; "mutable".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ =
            declared_mutable_terrane_f0_s2572; "shared".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&declared_mutable_terrane_f0_s2572
            .call(()))
        );
        gate_terrane_f0_s2747 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_terrane_f0_s2787 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gated_value_terrane_f0_s2830 = GatedAccumulator::terrane_construct();
        gated_bound_terrane_f0_s2874 = {
            let receiver = TerraneAsyncMutableState::new(gated_value_terrane_f0_s2830);
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
        scope_terrane_f0_s2997 = TerraneTaskScope::new(None);
        child_terrane_f0_s3019 = {
            let __terrane_scope = scope_terrane_f0_s2997.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = gated_bound_terrane_f0_s2874
                .call((
                    terrane_int_support::Int::from(4_i128),
                    started_terrane_f0_s2787.sender,
                    gate_terrane_f0_s2747.receiver,
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
        observed_terrane_f0_s3090 = __terrane_await(
                started_terrane_f0_s2787.receiver.receive(),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&observed_terrane_f0_s3090
            .available)
        );
        gated_copy_terrane_f0_s3163 = gated_bound_terrane_f0_s2874.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ =
            gated_copy_terrane_f0_s3163; "mutable".to_owned() })
        );
        released_terrane_f0_s3226 = __terrane_await(
                gate_terrane_f0_s2747.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&released_terrane_f0_s3226
            .accepted)
        );
        outcome_terrane_f0_s3292 = __terrane_await(
                scope_terrane_f0_s2997.join(child_terrane_f0_s3019),
            )
            .await;
        result_terrane_f0_s3328 = outcome_terrane_f0_s3292.value.clone();
        if result_terrane_f0_s3328.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &result_terrane_f0_s3328
                { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") })
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(did_fail, exceeded_deadline, detail);
        __terrane_constructed_value
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            available: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value
            .construct(did_fail, exceeded_deadline, has_value, detail, result_value);
        __terrane_constructed_value
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s966: TerranePlatformResult;
        raw_terrane_f1_s966 = terrane_platform_int_mutex(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s966);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s966);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s966);
    }
    pub fn load(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s1191: TerranePlatformResult;
        raw_terrane_f1_s1191 = terrane_platform_int_mutex_load(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1191),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1191),
            terrane_platform_result_bool(&raw_terrane_f1_s1191),
            terrane_platform_result_message(&raw_terrane_f1_s1191),
            terrane_platform_result_int(&raw_terrane_f1_s1191),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s1487: TerranePlatformResult;
        raw_terrane_f1_s1487 = terrane_platform_int_mutex_store(&self.handle, value);
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1487),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1487),
            terrane_platform_result_message(&raw_terrane_f1_s1487),
        );
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> ConcurrencyIntResult {
        let raw_terrane_f1_s1754: TerranePlatformResult;
        raw_terrane_f1_s1754 = terrane_platform_int_mutex_add(&self.handle, amount);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1754),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1754),
            terrane_platform_result_bool(&raw_terrane_f1_s1754),
            terrane_platform_result_message(&raw_terrane_f1_s1754),
            terrane_platform_result_int(&raw_terrane_f1_s1754),
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s2175: TerranePlatformResult;
        raw_terrane_f1_s2175 = terrane_platform_int_rw_lock(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s2175);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s2175);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s2175);
    }
    pub fn read(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s2410: TerranePlatformResult;
        raw_terrane_f1_s2410 = terrane_platform_int_rw_lock_read(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s2410),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s2410),
            terrane_platform_result_bool(&raw_terrane_f1_s2410),
            terrane_platform_result_message(&raw_terrane_f1_s2410),
            terrane_platform_result_int(&raw_terrane_f1_s2410),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s2716: TerranePlatformResult;
        raw_terrane_f1_s2716 = terrane_platform_int_rw_lock_write(&self.handle, value);
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s2716),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s2716),
            terrane_platform_result_message(&raw_terrane_f1_s2716),
        );
    }
}
#[derive(Clone)]
pub struct MemoryOrder {
    pub name: String,
}
impl MemoryOrder {
    pub fn terrane_construct(ordering_name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::from("sequentially-consistent"),
        };
        __terrane_constructed_value.construct(ordering_name);
        __terrane_constructed_value
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: i64) {
        let raw_terrane_f1_s3708: TerranePlatformResult;
        raw_terrane_f1_s3708 = terrane_platform_atomic_int64(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s3708);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s3708);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s3708);
    }
    pub fn load(&self, ordering: MemoryOrder) -> ConcurrencyIntResult {
        let raw_terrane_f1_s3958: TerranePlatformResult;
        raw_terrane_f1_s3958 = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s3958),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s3958),
            terrane_platform_result_bool(&raw_terrane_f1_s3958),
            terrane_platform_result_message(&raw_terrane_f1_s3958),
            terrane_platform_result_int(&raw_terrane_f1_s3958),
        );
    }
    pub fn store(
        &self,
        value: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s4297: TerranePlatformResult;
        raw_terrane_f1_s4297 = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s4297),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s4297),
            terrane_platform_result_message(&raw_terrane_f1_s4297),
        );
    }
    pub fn increase(
        &mut self,
        amount: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyIntResult {
        let raw_terrane_f1_s4607: TerranePlatformResult;
        raw_terrane_f1_s4607 = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s4607),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s4607),
            terrane_platform_result_bool(&raw_terrane_f1_s4607),
            terrane_platform_result_message(&raw_terrane_f1_s4607),
            terrane_platform_result_int(&raw_terrane_f1_s4607),
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s5043: TerranePlatformResult;
        raw_terrane_f1_s5043 = terrane_platform_thread_local_int(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s5043);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s5043);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s5043);
    }
    pub fn get(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s5274: TerranePlatformResult;
        raw_terrane_f1_s5274 = terrane_platform_thread_local_int_get(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5274),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5274),
            terrane_platform_result_bool(&raw_terrane_f1_s5274),
            terrane_platform_result_message(&raw_terrane_f1_s5274),
            terrane_platform_result_int(&raw_terrane_f1_s5274),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s5576: TerranePlatformResult;
        raw_terrane_f1_s5576 = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5576),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5576),
            terrane_platform_result_message(&raw_terrane_f1_s5576),
        );
    }
}
