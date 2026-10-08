// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs, time_base.rs, platform_capability_types.rs, platform_result_type.rs, platform_capability_base.rs, platform_time.rs, platform_signals.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support, terrane-signal-support
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
    pub static DESCRIPTORS: [&str; 2] = [
        "/core/process-signals::process-signal-error",
        "/core/time::invalid-duration",
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
    pub static FILES: [&str; 3] = [
        "case.trn",
        "core/process_signals.trn",
        "core/time.trn",
    ];
    pub static FUNCTIONS: [&str; 14] = [
        "/process-signals-surface::observe",
        "/process-signals-surface::main",
        "/core/process-signals::next",
        "/core/process-signals::close",
        "/core/process-signals::process-signals",
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
    pub static SITES: [Site; 16] = [
        /* terrane-site-row: site 0: /process-signals-surface::observe (case.trn:7:40-7:58) */
        { Site { function: 0, file: 0, line: 7, column: 40, end_line: 7, end_column: 58 } },
        /* terrane-site-row: site 1: /process-signals-surface::observe (case.trn:9:14-9:33) */
        { Site { function: 0, file: 0, line: 9, column: 14, end_line: 9, end_column: 33 } },
        /* terrane-site-row: site 2: /process-signals-surface::main (case.trn:14:20-14:45) */
        { Site { function: 1, file: 0, line: 14, column: 20, end_line: 14, end_column: 45 } },
        /* terrane-site-row: site 3: /process-signals-surface::main (case.trn:15:21-15:42) */
        { Site { function: 1, file: 0, line: 15, column: 21, end_line: 15, end_column: 42 } },
        /* terrane-site-row: site 4: /core/process-signals::next (core/process_signals.trn:56:13-56:91) */
        { Site { function: 2, file: 1, line: 56, column: 13, end_line: 56, end_column: 91 } },
        /* terrane-site-row: site 5: /core/process-signals::close (core/process_signals.trn:62:13-62:91) */
        { Site { function: 3, file: 1, line: 62, column: 13, end_line: 62, end_column: 91 } },
        /* terrane-site-row: site 6: /core/process-signals::process-signals (core/process_signals.trn:71:9-71:87) */
        { Site { function: 4, file: 1, line: 71, column: 9, end_line: 71, end_column: 87 } },
        /* terrane-site-row: site 7: /core/time::multiply (core/time.trn:62:13-62:45) */
        { Site { function: 5, file: 2, line: 62, column: 13, end_line: 62, end_column: 45 } },
        /* terrane-site-row: site 8: /core/time::seconds (core/time.trn:38:13-38:45) */
        { Site { function: 6, file: 2, line: 38, column: 13, end_line: 38, end_column: 45 } },
        /* terrane-site-row: site 9: /core/time::milliseconds (core/time.trn:43:13-43:45) */
        { Site { function: 7, file: 2, line: 43, column: 13, end_line: 43, end_column: 45 } },
        /* terrane-site-row: site 10: /core/time::microseconds (core/time.trn:48:13-48:45) */
        { Site { function: 8, file: 2, line: 48, column: 13, end_line: 48, end_column: 45 } },
        /* terrane-site-row: site 11: /core/time::nanoseconds (core/time.trn:53:13-53:45) */
        { Site { function: 9, file: 2, line: 53, column: 13, end_line: 53, end_column: 45 } },
        /* terrane-site-row: site 12: /core/time::duration-until (core/time.trn:76:13-76:45) */
        { Site { function: 10, file: 2, line: 76, column: 13, end_line: 76, end_column: 45 } },
        /* terrane-site-row: site 13: /core/time::at (core/time.trn:105:13-105:45) */
        { Site { function: 11, file: 2, line: 105, column: 13, end_line: 105, end_column: 45 } },
        /* terrane-site-row: site 14: /core/time::sleep-until (core/time.trn:161:13-161:45) */
        { Site { function: 12, file: 2, line: 161, column: 13, end_line: 161, end_column: 45 } },
        /* terrane-site-row: site 15: /core/time::interval (core/time.trn:172:13-172:45) */
        { Site { function: 13, file: 2, line: 172, column: 13, end_line: 172, end_column: 45 } },
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
// Namespace: process-signals-surface
async fn observe(
    mut subscription: ProcessSignalSubscription,
) -> Result<(), TerraneError> {
    let event_terrane_f0_s311: ProcessSignalEvent;
    let closed_terrane_f0_s445: ();
    event_terrane_f0_s311 = __terrane_traced_err(
        __terrane_await((&mut subscription).next()).await,
        0 /* terrane-site: case.trn:7:40-7:58 */,
    )?;
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&event_terrane_f0_s311.signal
        .name), terrane_scalar_support::scalar_text(&event_terrane_f0_s311.count),
        terrane_scalar_support::scalar_text(&event_terrane_f0_s311.sequence),
        terrane_scalar_support::scalar_text(&event_terrane_f0_s311.overflowed)
    );
    closed_terrane_f0_s445 = __terrane_traced_err(
        subscription.close(),
        1 /* terrane-site: case.trn:9:14-9:33 */,
    )?;
    let _ = &closed_terrane_f0_s445;
    return Ok(());
}
fn main() {
    __terrane_run(async move {
        let selected_terrane_f0_s516: terrane_collection_support::Set<ProcessSignal>;
        let subscription_terrane_f0_s669: ProcessSignalSubscription;
        let ignored_terrane_f0_s714: ();
        selected_terrane_f0_s516 = terrane_collection_support::Set::<
            ProcessSignal,
        >::new(
            vec![
                ProcessSignal::terrane_static_interrupt(),
                ProcessSignal::terrane_static_terminate(),
                ProcessSignal::terrane_static_hangup(),
                ProcessSignal::terrane_static_quit()
            ],
        );
        subscription_terrane_f0_s669 = __terrane_traced(
            process_signals(selected_terrane_f0_s516),
            2 /* terrane-site: case.trn:14:20-14:45 */,
        );
        ignored_terrane_f0_s714 = __terrane_traced(
            __terrane_await(observe(subscription_terrane_f0_s669.terrane_separate()))
                .await,
            3 /* terrane-site: case.trn:15:21-15:42 */,
        );
        let _ = &ignored_terrane_f0_s714;
    });
}
// Source: core/process_signals.trn
// Namespace: core/process-signals
#[derive(Clone)]
pub struct ProcessSignalError {
    pub message: String,
}
impl ProcessSignalError {
    pub fn terrane_construct(detail: String) -> Self {
        let mut __terrane_constructed_value = Self { message: String::from("") };
        __terrane_constructed_value.construct(detail);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, detail: String) {
        self.message = detail;
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProcessSignal {
    pub name: String,
}
impl ProcessSignal {
    pub fn terrane_construct(signal_name: String) -> Self {
        let mut __terrane_constructed_value = Self { name: String::from("") };
        __terrane_constructed_value.construct(signal_name);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, signal_name: String) {
        self.name = signal_name;
    }
    pub fn terrane_static_interrupt() -> ProcessSignal {
        return ProcessSignal::terrane_construct(String::from("interrupt"));
    }
    pub fn terrane_static_terminate() -> ProcessSignal {
        return ProcessSignal::terrane_construct(String::from("terminate"));
    }
    pub fn terrane_static_hangup() -> ProcessSignal {
        return ProcessSignal::terrane_construct(String::from("hangup"));
    }
    pub fn terrane_static_quit() -> ProcessSignal {
        return ProcessSignal::terrane_construct(String::from("quit"));
    }
}
#[derive(Clone)]
pub struct ProcessSignalEvent {
    pub signal: ProcessSignal,
    pub count: terrane_int_support::Int,
    pub sequence: terrane_int_support::Int,
    pub observed_at: MonotonicInstant,
    pub overflowed: bool,
}
impl ProcessSignalEvent {
    pub fn terrane_construct(
        observed_signal: ProcessSignal,
        occurrences: terrane_int_support::Int,
        event_sequence: terrane_int_support::Int,
        observed: MonotonicInstant,
        did_overflow: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            signal: ProcessSignal::terrane_construct(String::from("")),
            count: terrane_int_support::Int::from(0_i128),
            sequence: terrane_int_support::Int::from(0_i128),
            observed_at: Clock::terrane_static_monotonic(),
            overflowed: false,
        };
        __terrane_constructed_value
            .construct(
                observed_signal,
                occurrences,
                event_sequence,
                observed,
                did_overflow,
            );
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        observed_signal: ProcessSignal,
        occurrences: terrane_int_support::Int,
        event_sequence: terrane_int_support::Int,
        observed: MonotonicInstant,
        did_overflow: bool,
    ) {
        self.signal = observed_signal;
        self.count = occurrences.clone();
        self.sequence = event_sequence.clone();
        self.observed_at = observed;
        self.overflowed = did_overflow;
    }
}
#[derive(Clone)]
pub struct ProcessSignalSubscription {
    __terrane_lifetime: std::sync::Arc<()>,
    pub handle: TerranePlatformCapability,
}
impl ProcessSignalSubscription {
    pub fn terrane_construct(capability: TerranePlatformCapability) -> Self {
        let mut __terrane_constructed_value = Self {
            handle: TerranePlatformCapability::default(),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(capability);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, capability: TerranePlatformCapability) {
        self.handle = capability;
    }
    pub async fn next(&mut self) -> Result<ProcessSignalEvent, TerraneError> {
        let raw_terrane_f1_s1709: TerranePlatformResult;
        raw_terrane_f1_s1709 = __terrane_await(terrane_process_signal_next(&self.handle))
            .await;
        if terrane_platform_result_failed(&raw_terrane_f1_s1709) {
            return Err({
                let value = ProcessSignalError::terrane_construct(
                    terrane_platform_result_message(&raw_terrane_f1_s1709),
                );
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    4 /* terrane-site: core/process_signals.trn:56:13-56:91 */,
                )
            });
        }
        return Ok(
            ProcessSignalEvent::terrane_construct(
                ProcessSignal::terrane_construct(
                    terrane_platform_result_detail(&raw_terrane_f1_s1709),
                ),
                terrane_process_signal_result_exact_int(&raw_terrane_f1_s1709),
                terrane_platform_result_int(&raw_terrane_f1_s1709),
                terrane_process_signal_result_observed(&raw_terrane_f1_s1709),
                terrane_platform_result_bool(&raw_terrane_f1_s1709),
            ),
        );
    }
    pub fn close(self) -> Result<(), TerraneError> {
        let raw_terrane_f1_s2254: TerranePlatformResult;
        raw_terrane_f1_s2254 = terrane_process_signal_close(&self.handle);
        if terrane_platform_result_failed(&raw_terrane_f1_s2254) {
            return Err({
                let value = ProcessSignalError::terrane_construct(
                    terrane_platform_result_message(&raw_terrane_f1_s2254),
                );
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    5 /* terrane-site: core/process_signals.trn:62:13-62:91 */,
                )
            });
        }
        return Ok(());
    }
    pub fn destruct(&mut self) {
        terrane_process_signal_close(&self.handle);
    }
}
impl Drop for ProcessSignalSubscription {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
pub fn process_signals(
    selected: terrane_collection_support::Set<ProcessSignal>,
) -> Result<ProcessSignalSubscription, TerraneError> {
    let raw_terrane_f1_s2659: TerranePlatformResult;
    raw_terrane_f1_s2659 = terrane_process_signal_subscribe(selected);
    if terrane_platform_result_failed(&raw_terrane_f1_s2659) {
        return Err({
            let value = ProcessSignalError::terrane_construct(
                terrane_platform_result_message(&raw_terrane_f1_s2659),
            );
            TerraneError::raised_with_message(
                TerraneErrorKind::Custom(DescriptorId(0)),
                value.render(),
                6 /* terrane-site: core/process_signals.trn:71:9-71:87 */,
            )
        });
    }
    return Ok(
        ProcessSignalSubscription::terrane_construct(
            terrane_platform_result_capability(&raw_terrane_f1_s2659),
        ),
    );
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
        let mut __terrane_constructed_value = Self {
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(total);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, total: terrane_int_support::Int) {
        self.total_nanoseconds = total.clone();
    }
    pub fn checked(&self, other: Duration) -> Option<Duration> {
        let difference_terrane_f2_s602: terrane_int_support::Int;
        if self.total_nanoseconds.clone() < other.total_nanoseconds.clone() {
            return None;
        }
        difference_terrane_f2_s602 = self.total_nanoseconds.clone()
            - other.total_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(
                    &difference_terrane_f2_s602,
                    1000000000.clone(),
                ),
                terrane_platform_time_mod(
                    &difference_terrane_f2_s602,
                    1000000000.clone(),
                ),
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
        let mut __terrane_constructed_value = Self {
            seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
            total_nanoseconds: terrane_int_support::Int::from(0_i128),
            subtract: DurationSubtraction::terrane_construct(
                terrane_int_support::Int::from(0_i128),
            ),
        };
        __terrane_constructed_value.construct(whole_seconds, fractional_nanoseconds);
        __terrane_constructed_value
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
        let fractional_terrane_f2_s2245: terrane_int_support::Int;
        fractional_terrane_f2_s2245 = self.nanoseconds.clone()
            + other.nanoseconds.clone();
        return Duration::terrane_construct(
            self.seconds.clone() + other.seconds.clone()
                + terrane_platform_time_div(
                    &fractional_terrane_f2_s2245,
                    1000000000.clone(),
                ),
            terrane_platform_time_mod(&fractional_terrane_f2_s2245, 1000000000.clone()),
        );
    }
    pub fn multiply(
        &self,
        multiplier: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        let total_terrane_f2_s2592: terrane_int_support::Int;
        if multiplier.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    7 /* terrane-site: core/time.trn:62:13-62:45 */,
                )
            });
        }
        total_terrane_f2_s2592 = self.total_nanoseconds.clone() * multiplier.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&total_terrane_f2_s2592, 1000000000.clone()),
                terrane_platform_time_mod(&total_terrane_f2_s2592, 1000000000.clone()),
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
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    8 /* terrane-site: core/time.trn:38:13-38:45 */,
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
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    9 /* terrane-site: core/time.trn:43:13-43:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000.clone()),
                terrane_platform_time_mod(&value, 1000.clone())
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
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    10 /* terrane-site: core/time.trn:48:13-48:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000000.clone()),
                terrane_platform_time_mod(&value, 1000000.clone())
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
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    11 /* terrane-site: core/time.trn:53:13-53:45 */,
                )
            });
        }
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&value, 1000000000.clone()),
                terrane_platform_time_mod(&value, 1000000000.clone()),
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
        let mut __terrane_constructed_value = Self {
            domain: terrane_int_support::Int::from(0_i128),
            elapsed_nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(runtime_domain, elapsed);
        __terrane_constructed_value
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
        let elapsed_terrane_f2_s3217: terrane_int_support::Int;
        if self.domain.clone() != later.domain.clone().clone()
            || later.elapsed_nanoseconds.clone().clone()
                < self.elapsed_nanoseconds.clone()
        {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    12 /* terrane-site: core/time.trn:76:13-76:45 */,
                )
            });
        }
        elapsed_terrane_f2_s3217 = later.elapsed_nanoseconds.clone().clone()
            - self.elapsed_nanoseconds.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed_terrane_f2_s3217, 1000000000.clone()),
                terrane_platform_time_mod(&elapsed_terrane_f2_s3217, 1000000000.clone()),
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
        let mut __terrane_constructed_value = Self {
            unix_seconds: terrane_int_support::Int::from(0_i128),
            nanoseconds: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(seconds, fractional_nanoseconds);
        __terrane_constructed_value
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
        let mut __terrane_constructed_value = Self {
            expires_at: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
        };
        __terrane_constructed_value.construct(target);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, target: MonotonicInstant) {
        self.expires_at = target;
    }
    pub fn remaining(&self) -> Option<Duration> {
        let now_terrane_f2_s3834: MonotonicInstant;
        let elapsed_terrane_f2_s3964: terrane_int_support::Int;
        now_terrane_f2_s3834 = Clock::terrane_static_monotonic();
        if self.expires_at.elapsed_nanoseconds.clone()
            <= now_terrane_f2_s3834.elapsed_nanoseconds.clone()
        {
            return None;
        }
        elapsed_terrane_f2_s3964 = self.expires_at.elapsed_nanoseconds.clone()
            - now_terrane_f2_s3834.elapsed_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed_terrane_f2_s3964, 1000000000.clone()),
                terrane_platform_time_mod(&elapsed_terrane_f2_s3964, 1000000000.clone()),
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
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    13 /* terrane-site: core/time.trn:105:13-105:45 */,
                )
            });
        }
        return Ok(Deadline::terrane_construct(target));
    }
}
pub fn discard_none(value: ()) {
    let _ = &value;
    return ();
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
        let mut __terrane_constructed_value = Self {
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
        __terrane_constructed_value.construct(scheduled_at, observed_at, expirations);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        scheduled_at: MonotonicInstant,
        observed_at: MonotonicInstant,
        expirations: terrane_int_support::Int,
    ) {
        self.scheduled = scheduled_at;
        self.observed = observed_at;
        self.count = expirations.clone();
    }
}
#[derive(Clone)]
pub struct Ticker {
    __terrane_lifetime: std::sync::Arc<()>,
    pub anchor: MonotonicInstant,
    pub period: Duration,
    pub next_index: terrane_int_support::Int,
}
impl Ticker {
    pub fn terrane_construct(interval: Duration) -> Self {
        let mut __terrane_constructed_value = Self {
            anchor: MonotonicInstant::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            period: Duration::terrane_construct(
                terrane_int_support::Int::from(0_i128),
                terrane_int_support::Int::from(0_i128),
            ),
            next_index: terrane_int_support::Int::from(1_i128),
            __terrane_lifetime: std::sync::Arc::new(()),
        };
        __terrane_constructed_value.construct(interval);
        __terrane_constructed_value
    }
    pub fn terrane_separate(&self) -> Self {
        let mut value = self.clone();
        value.__terrane_lifetime = std::sync::Arc::new(());
        value
    }
    pub fn construct(&mut self, interval: Duration) {
        self.anchor = Clock::terrane_static_monotonic();
        self.period = interval;
    }
    pub async fn next(&mut self) -> Tick {
        let period_total_terrane_f2_s5216: terrane_int_support::Int;
        let scheduled_nanoseconds_terrane_f2_s5269: terrane_int_support::Int;
        let observed_terrane_f2_s5441: MonotonicInstant;
        let elapsed_terrane_f2_s5478: terrane_int_support::Int;
        let mut observed_index_terrane_f2_s5559: terrane_int_support::Int;
        let count_terrane_f2_s5710: terrane_int_support::Int;
        let delivered_terrane_f2_s5763: MonotonicInstant;
        period_total_terrane_f2_s5216 = self.period.total_nanoseconds.clone();
        scheduled_nanoseconds_terrane_f2_s5269 = self.anchor.elapsed_nanoseconds.clone()
            + period_total_terrane_f2_s5216.clone() * self.next_index.clone();
        discard_none(
            __terrane_await(
                    terrane_platform_time_sleep_until(
                        scheduled_nanoseconds_terrane_f2_s5269,
                    ),
                )
                .await,
        );
        observed_terrane_f2_s5441 = Clock::terrane_static_monotonic();
        elapsed_terrane_f2_s5478 = observed_terrane_f2_s5441.elapsed_nanoseconds.clone()
            - self.anchor.elapsed_nanoseconds.clone();
        observed_index_terrane_f2_s5559 = terrane_platform_time_div(
            &elapsed_terrane_f2_s5478,
            period_total_terrane_f2_s5216.clone(),
        );
        if observed_index_terrane_f2_s5559.clone() < self.next_index.clone() {
            observed_index_terrane_f2_s5559 = self.next_index.clone();
        }
        count_terrane_f2_s5710 = observed_index_terrane_f2_s5559.clone()
            - self.next_index.clone() + terrane_int_support::Int::from(1_i128);
        delivered_terrane_f2_s5763 = MonotonicInstant::terrane_construct(
            self.anchor.domain.clone(),
            self.anchor.elapsed_nanoseconds.clone()
                + period_total_terrane_f2_s5216.clone()
                    * observed_index_terrane_f2_s5559.clone(),
        );
        self.next_index = observed_index_terrane_f2_s5559.clone()
            + terrane_int_support::Int::from(1_i128);
        return Tick::terrane_construct(
            delivered_terrane_f2_s5763,
            observed_terrane_f2_s5441,
            count_terrane_f2_s5710.clone(),
        );
    }
    pub fn destruct(&mut self) {
        discard_none(());
    }
}
impl Drop for Ticker {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.__terrane_lifetime) != 1 {
            return;
        }
        self.destruct();
    }
}
#[derive(Clone)]
pub struct Clock {}
impl Clock {
    pub fn terrane_construct() -> Self {
        Self {}
    }
    pub fn terrane_static_wall() -> Instant {
        let raw_terrane_f2_s6105: TerranePlatformResult;
        raw_terrane_f2_s6105 = terrane_platform_time_wall();
        return Instant::terrane_construct(
            terrane_platform_time_wall_seconds(&raw_terrane_f2_s6105),
            terrane_platform_time_wall_nanoseconds(&raw_terrane_f2_s6105),
        );
    }
    pub fn terrane_static_monotonic() -> MonotonicInstant {
        return MonotonicInstant::terrane_construct(
            terrane_platform_time_domain(),
            terrane_platform_time_monotonic(),
        );
    }
    pub async fn terrane_static_sleep(elapsed: Duration) {
        let target_terrane_f2_s6426: terrane_int_support::Int;
        target_terrane_f2_s6426 = terrane_platform_time_monotonic()
            + elapsed.total_nanoseconds.clone();
        return __terrane_await(
                terrane_platform_time_sleep_until(target_terrane_f2_s6426),
            )
            .await;
    }
    pub async fn terrane_static_sleep_until(
        target: MonotonicInstant,
    ) -> Result<(), TerraneError> {
        if target.domain.clone() != terrane_platform_time_domain() {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    14 /* terrane-site: core/time.trn:161:13-161:45 */,
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
        let now_terrane_f2_s6860: MonotonicInstant;
        let target_terrane_f2_s6892: MonotonicInstant;
        now_terrane_f2_s6860 = Clock::terrane_static_monotonic();
        target_terrane_f2_s6892 = MonotonicInstant::terrane_construct(
            now_terrane_f2_s6860.domain.clone(),
            now_terrane_f2_s6860.elapsed_nanoseconds.clone()
                + elapsed.total_nanoseconds.clone(),
        );
        return Deadline::terrane_construct(target_terrane_f2_s6892);
    }
    pub fn terrane_static_interval(period: Duration) -> Result<Ticker, TerraneError> {
        if period.total_nanoseconds.clone() == terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(1)),
                    value.render(),
                    15 /* terrane-site: core/time.trn:172:13-172:45 */,
                )
            });
        }
        return Ok(Ticker::terrane_construct(period));
    }
}
