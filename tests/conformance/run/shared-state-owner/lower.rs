// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, channels.rs, tasks_native_local.rs, time_inactive.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
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
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn increase(&mut self) -> terrane_int_support::Int {
        self.value = self.value.clone() + terrane_int_support::Int::from(1_i128);
        return self.value.clone();
    }
}
async fn own_state(
    commands: TerraneChannelReceiver<String>,
    responses: TerraneChannelSender<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let mut state_terrane_f0_s331: Counter;
    let mut running_terrane_f0_s359: bool;
    let mut received_terrane_f0_s399: TerraneChannelReceiveOutcome<String>;
    let mut command_terrane_f0_s438: Option<String>;
    let mut sent_terrane_f0_s560: TerraneChannelSendOutcome<terrane_int_support::Int>;
    state_terrane_f0_s331 = Counter::terrane_construct();
    running_terrane_f0_s359 = true;
    while running_terrane_f0_s359 {
        received_terrane_f0_s399 = __terrane_await(Box::pin(commands.receive())).await;
        command_terrane_f0_s438 = received_terrane_f0_s399.value;
        if command_terrane_f0_s438.is_some() {
            if match &command_terrane_f0_s438 {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .as_str() == "increase"
            {
                state_terrane_f0_s331.increase();
            } else {
                sent_terrane_f0_s560 = __terrane_await(
                        Box::pin(responses.send(state_terrane_f0_s331.value.clone())),
                    )
                    .await;
                if !sent_terrane_f0_s560.accepted {
                    return terrane_int_support::Int::from(-1_i128);
                }
                running_terrane_f0_s359 = false;
            }
        }
    }
    commands.close();
    responses.close();
    return state_terrane_f0_s331.value.clone();
}
async fn use_state(
    commands: TerraneChannelSender<String>,
    responses: TerraneChannelReceiver<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let first_terrane_f0_s846: TerraneChannelSendOutcome<String>;
    let second_terrane_f0_s926: TerraneChannelSendOutcome<String>;
    let requested_terrane_f0_s1008: TerraneChannelSendOutcome<String>;
    let received_terrane_f0_s1092: TerraneChannelReceiveOutcome<
        terrane_int_support::Int,
    >;
    let value_terrane_f0_s1130: Option<terrane_int_support::Int>;
    first_terrane_f0_s846 = __terrane_await(
            Box::pin(commands.send(String::from("increase"))),
        )
        .await;
    if !first_terrane_f0_s846.accepted {
        return terrane_int_support::Int::from(-1_i128);
    }
    second_terrane_f0_s926 = __terrane_await(
            Box::pin(commands.send(String::from("increase"))),
        )
        .await;
    if !second_terrane_f0_s926.accepted {
        return terrane_int_support::Int::from(-1_i128);
    }
    requested_terrane_f0_s1008 = __terrane_await(
            Box::pin(commands.send(String::from("read"))),
        )
        .await;
    if !requested_terrane_f0_s1008.accepted {
        return terrane_int_support::Int::from(-1_i128);
    }
    received_terrane_f0_s1092 = __terrane_await(Box::pin(responses.receive())).await;
    value_terrane_f0_s1130 = received_terrane_f0_s1092.value;
    commands.close();
    responses.close();
    if value_terrane_f0_s1130.is_some() {
        return match value_terrane_f0_s1130 {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    }
    return terrane_int_support::Int::from(-1_i128);
}
fn main() {
    __terrane_run(async move {
        let command_pair_terrane_f0_s1262: TerraneChannelPair<String>;
        let response_pair_terrane_f0_s1313: TerraneChannelPair<terrane_int_support::Int>;
        let scope_terrane_f0_s1362: TerraneTaskScope;
        let owner_terrane_f0_s1384: TerraneScopedTask<terrane_int_support::Int>;
        let client_terrane_f0_s1464: TerraneScopedTask<terrane_int_support::Int>;
        let owner_outcome_terrane_f0_s1545: TerraneTaskOutcome<terrane_int_support::Int>;
        let client_outcome_terrane_f0_s1587: TerraneTaskOutcome<
            terrane_int_support::Int,
        >;
        let owner_value_terrane_f0_s1631: Option<terrane_int_support::Int>;
        let client_value_terrane_f0_s1667: Option<terrane_int_support::Int>;
        command_pair_terrane_f0_s1262 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        response_pair_terrane_f0_s1313 = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        scope_terrane_f0_s1362 = TerraneTaskScope::new(None);
        owner_terrane_f0_s1384 = {
            let __terrane_scope = scope_terrane_f0_s1362.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = own_state(
                command_pair_terrane_f0_s1262.receiver,
                response_pair_terrane_f0_s1313.sender,
            );
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
        client_terrane_f0_s1464 = {
            let __terrane_scope = scope_terrane_f0_s1362.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = use_state(
                command_pair_terrane_f0_s1262.sender,
                response_pair_terrane_f0_s1313.receiver,
            );
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
        owner_outcome_terrane_f0_s1545 = __terrane_await(
                scope_terrane_f0_s1362.join(owner_terrane_f0_s1384),
            )
            .await;
        client_outcome_terrane_f0_s1587 = __terrane_await(
                scope_terrane_f0_s1362.join(client_terrane_f0_s1464),
            )
            .await;
        owner_value_terrane_f0_s1631 = owner_outcome_terrane_f0_s1545.value.clone();
        client_value_terrane_f0_s1667 = client_outcome_terrane_f0_s1587.value.clone();
        if owner_value_terrane_f0_s1631.is_some() {
            if client_value_terrane_f0_s1667.is_some() {
                println!(
                    "{}{}", terrane_scalar_support::scalar_text(&match
                    &owner_value_terrane_f0_s1631 { Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") }),
                    terrane_scalar_support::scalar_text(&match
                    &client_value_terrane_f0_s1667 { Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") })
                );
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
