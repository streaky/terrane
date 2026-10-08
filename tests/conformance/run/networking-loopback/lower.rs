// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs, time_base.rs, platform_streams.rs, platform_standard_streams.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_networking.rs, platform_time.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-stream-abi, terrane-platform-support
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
    pub static DESCRIPTORS: [&str; 1] = ["/core/time::invalid-duration"];
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
    pub static FILES: [&str; 4] = [
        "case.trn",
        "core/networking.trn",
        "core/streams.trn",
        "core/time.trn",
    ];
    pub static FUNCTIONS: [&str; 15] = [
        "/app::main",
        "/core/networking::lookup-dns",
        "/core/streams::read",
        "/core/streams::read-exact",
        "/core/streams::read-all",
        "/core/streams::read-async",
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
        /* terrane-site-row: site 0: /app::main (case.trn:17:70-17:90) */
        { Site { function: 0, file: 0, line: 17, column: 70, end_line: 17, end_column: 90 } },
        /* terrane-site-row: site 1: /app::main (case.trn:24:29-24:55) */
        { Site { function: 0, file: 0, line: 24, column: 29, end_line: 24, end_column: 55 } },
        /* terrane-site-row: site 2: /core/networking::lookup-dns (core/networking.trn:328:28-328:49) */
        { Site { function: 1, file: 1, line: 328, column: 28, end_line: 328, end_column: 49 } },
        /* terrane-site-row: site 3: /core/streams::read (core/streams.trn:188:23-188:50) */
        { Site { function: 2, file: 2, line: 188, column: 23, end_line: 188, end_column: 50 } },
        /* terrane-site-row: site 4: /core/streams::read-exact (core/streams.trn:210:23-210:46) */
        { Site { function: 3, file: 2, line: 210, column: 23, end_line: 210, end_column: 46 } },
        /* terrane-site-row: site 5: /core/streams::read-all (core/streams.trn:229:23-229:46) */
        { Site { function: 4, file: 2, line: 229, column: 23, end_line: 229, end_column: 46 } },
        /* terrane-site-row: site 6: /core/streams::read-async (core/streams.trn:234:23-234:50) */
        { Site { function: 5, file: 2, line: 234, column: 23, end_line: 234, end_column: 50 } },
        /* terrane-site-row: site 7: /core/time::multiply (core/time.trn:62:13-62:45) */
        { Site { function: 6, file: 3, line: 62, column: 13, end_line: 62, end_column: 45 } },
        /* terrane-site-row: site 8: /core/time::seconds (core/time.trn:38:13-38:45) */
        { Site { function: 7, file: 3, line: 38, column: 13, end_line: 38, end_column: 45 } },
        /* terrane-site-row: site 9: /core/time::milliseconds (core/time.trn:43:13-43:45) */
        { Site { function: 8, file: 3, line: 43, column: 13, end_line: 43, end_column: 45 } },
        /* terrane-site-row: site 10: /core/time::microseconds (core/time.trn:48:13-48:45) */
        { Site { function: 9, file: 3, line: 48, column: 13, end_line: 48, end_column: 45 } },
        /* terrane-site-row: site 11: /core/time::nanoseconds (core/time.trn:53:13-53:45) */
        { Site { function: 10, file: 3, line: 53, column: 13, end_line: 53, end_column: 45 } },
        /* terrane-site-row: site 12: /core/time::duration-until (core/time.trn:76:13-76:45) */
        { Site { function: 11, file: 3, line: 76, column: 13, end_line: 76, end_column: 45 } },
        /* terrane-site-row: site 13: /core/time::at (core/time.trn:105:13-105:45) */
        { Site { function: 12, file: 3, line: 105, column: 13, end_line: 105, end_column: 45 } },
        /* terrane-site-row: site 14: /core/time::sleep-until (core/time.trn:161:13-161:45) */
        { Site { function: 13, file: 3, line: 161, column: 13, end_line: 161, end_column: 45 } },
        /* terrane-site-row: site 15: /core/time::interval (core/time.trn:172:13-172:45) */
        { Site { function: 14, file: 3, line: 172, column: 13, end_line: 172, end_column: 45 } },
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
// Namespace: app
fn main() {
    __terrane_run(async move {
        let loopback_terrane_f0_s371: IpResult;
        let address_terrane_f0_s422: SocketResult;
        let bound_terrane_f0_s478: UdpResult;
        let socket_terrane_f0_s525: UdpSocket;
        let destination_terrane_f0_s555: SocketResult;
        let configured_terrane_f0_s622: NetworkOperationResult;
        let cancellation_terrane_f0_s691: NetworkCancellationToken;
        let options_terrane_f0_s747: NetworkOperationOptions;
        let sent_terrane_f0_s853: IoResult;
        let received_terrane_f0_s925: IoResult;
        let bytes_output_terrane_f0_s1038: ByteWriter;
        let output_terrane_f0_s1065: TextWriter;
        let written_terrane_f0_s1102: WriteResult;
        loopback_terrane_f0_s371 = ip_address_from_string(String::from("127.0.0.1"));
        address_terrane_f0_s422 = socket_address_from_ip(
            loopback_terrane_f0_s371.value,
            terrane_int_support::Int::from(0_i128),
        );
        bound_terrane_f0_s478 = bind_udp(address_terrane_f0_s422.value);
        socket_terrane_f0_s525 = bound_terrane_f0_s478.value;
        destination_terrane_f0_s555 = socket_address_from_string(
            socket_terrane_f0_s525.local_address.clone(),
        );
        configured_terrane_f0_s622 = socket_terrane_f0_s525
            .configure(
                UdpOptions::terrane_construct(
                    false,
                    terrane_int_support::Int::from(32_i128),
                ),
            );
        cancellation_terrane_f0_s691 = NetworkCancellationToken::terrane_construct();
        options_terrane_f0_s747 = NetworkOperationOptions::terrane_construct(
            Some(
                Clock::terrane_static_deadline(
                    __terrane_traced(
                        Duration::terrane_static_seconds(
                            terrane_int_support::Int::from(1_i128),
                        ),
                        0 /* terrane-site: case.trn:17:70-17:90 */,
                    ),
                ),
            ),
            cancellation_terrane_f0_s691,
        );
        sent_terrane_f0_s853 = __terrane_await(
                (&socket_terrane_f0_s525)
                    .send_to(
                        Vec::from([116, 101, 114, 114, 97, 110, 101]),
                        destination_terrane_f0_s555.value,
                        options_terrane_f0_s747.clone(),
                    ),
            )
            .await;
        received_terrane_f0_s925 = __terrane_await(
                (&socket_terrane_f0_s525)
                    .receive_from(
                        terrane_int_support::Int::from(32_i128),
                        options_terrane_f0_s747,
                    ),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&! configured_terrane_f0_s622
            .failed)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&sent_terrane_f0_s853.completed)
        );
        bytes_output_terrane_f0_s1038 = stdout();
        output_terrane_f0_s1065 = bytes_output_terrane_f0_s1038
            .text(terrane_string_support::Encoding::Utf8);
        written_terrane_f0_s1102 = output_terrane_f0_s1065
            .line(
                __terrane_raised(
                    terrane_string_support::decode(
                        &received_terrane_f0_s925.data,
                        terrane_string_support::Encoding::Utf8,
                    ),
                    1 /* terrane-site: case.trn:24:29-24:55 */,
                ),
            );
        if written_terrane_f0_s1102.failed {
            println!(
                "{}", terrane_scalar_support::scalar_text(&written_terrane_f0_s1102
                .message)
            );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&! received_terrane_f0_s925
            .truncated)
        );
        socket_terrane_f0_s525.close();
    });
}
// Source: core/networking.trn
// Namespace: core/networking
#[derive(Clone)]
pub struct NetworkOperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl NetworkOperationResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(failed, deadline_exceeded, message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, deadline_exceeded: bool, message: String) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct NetworkCancellationToken {
    pub handle: TerranePlatformCapability,
}
impl NetworkCancellationToken {
    pub fn terrane_construct() -> Self {
        Self {
            handle: terrane_platform_cancellation_token(),
        }
    }
}
pub fn network_cancel_operation(
    cancellation: NetworkCancellationToken,
) -> NetworkOperationResult {
    let raw_terrane_f1_s607: TerranePlatformResult;
    raw_terrane_f1_s607 = terrane_platform_cancel(&cancellation.handle);
    return NetworkOperationResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f1_s607),
        terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s607),
        terrane_platform_result_message(&raw_terrane_f1_s607),
    );
}
#[derive(Clone)]
pub struct NetworkOperationOptions {
    pub deadline: Option<Deadline>,
    pub cancellation: NetworkCancellationToken,
}
impl NetworkOperationOptions {
    pub fn terrane_construct(
        requested: Option<Deadline>,
        cancellation: NetworkCancellationToken,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            deadline: None,
            cancellation: NetworkCancellationToken::terrane_construct(),
        };
        __terrane_constructed_value.construct(requested, cancellation);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        requested: Option<Deadline>,
        cancellation: NetworkCancellationToken,
    ) {
        self.deadline = requested;
        self.cancellation = cancellation;
    }
}
pub fn operation_cancellation(
    options: NetworkOperationOptions,
) -> TerranePlatformCapability {
    return options.cancellation.handle;
}
pub fn operation_deadline(options: NetworkOperationOptions) -> terrane_int_support::Int {
    let selected_terrane_f1_s1323: Option<Deadline>;
    let remaining_terrane_f1_s1383: Option<Duration>;
    selected_terrane_f1_s1323 = options.deadline.clone();
    if selected_terrane_f1_s1323.is_some() {
        remaining_terrane_f1_s1383 = match &selected_terrane_f1_s1323 {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        }
            .remaining();
        if remaining_terrane_f1_s1383.is_some() {
            return match &remaining_terrane_f1_s1383 {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .total_nanoseconds
                .clone();
        }
        return terrane_int_support::Int::from(0_i128);
    }
    return terrane_int_support::Int::from(-1_i128);
}
#[derive(Clone)]
pub struct TcpOptions {
    pub no_delay: bool,
    pub ttl: terrane_int_support::Int,
}
impl TcpOptions {
    pub fn terrane_construct(no_delay: bool, ttl: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            no_delay: true,
            ttl: terrane_int_support::Int::from(64_i128),
        };
        __terrane_constructed_value.construct(no_delay, ttl);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, no_delay: bool, ttl: terrane_int_support::Int) {
        self.no_delay = no_delay;
        self.ttl = ttl.clone();
    }
}
#[derive(Clone)]
pub struct UdpOptions {
    pub broadcast: bool,
    pub ttl: terrane_int_support::Int,
}
impl UdpOptions {
    pub fn terrane_construct(broadcast: bool, ttl: terrane_int_support::Int) -> Self {
        let mut __terrane_constructed_value = Self {
            broadcast: false,
            ttl: terrane_int_support::Int::from(64_i128),
        };
        __terrane_constructed_value.construct(broadcast, ttl);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, broadcast: bool, ttl: terrane_int_support::Int) {
        self.broadcast = broadcast;
        self.ttl = ttl.clone();
    }
}
#[derive(Clone)]
pub struct IpAddress {
    pub value: String,
    pub version: String,
    pub is_loopback: bool,
}
impl IpAddress {
    pub fn terrane_construct(raw: TerranePlatformResult) -> Self {
        let mut __terrane_constructed_value = Self {
            value: String::from(""),
            version: String::from(""),
            is_loopback: false,
        };
        __terrane_constructed_value.construct(raw);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, raw: TerranePlatformResult) {
        self.value = terrane_platform_result_text(&raw);
        self.version = terrane_platform_result_detail(&raw);
        self.is_loopback = terrane_platform_result_bool(&raw);
    }
    pub fn string(&self) -> String {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct IpResult {
    pub failed: bool,
    pub message: String,
    pub value: IpAddress,
}
impl IpResult {
    pub fn terrane_construct(failed: bool, message: String, address: IpAddress) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: IpAddress::terrane_construct(terrane_platform_failed_result()),
        };
        __terrane_constructed_value.construct(failed, message, address);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, address: IpAddress) {
        self.failed = failed;
        self.message = message;
        self.value = address;
    }
}
pub fn ip_address_from_string(text: String) -> IpResult {
    let raw_terrane_f1_s2555: TerranePlatformResult;
    let failed_terrane_f1_s2585: bool;
    let message_terrane_f1_s2622: String;
    raw_terrane_f1_s2555 = terrane_platform_parse_ip(text);
    failed_terrane_f1_s2585 = terrane_platform_result_failed(&raw_terrane_f1_s2555);
    message_terrane_f1_s2622 = terrane_platform_result_message(&raw_terrane_f1_s2555);
    return IpResult::terrane_construct(
        failed_terrane_f1_s2585,
        message_terrane_f1_s2622,
        IpAddress::terrane_construct(raw_terrane_f1_s2555),
    );
}
#[derive(Clone)]
pub struct SocketAddress {
    pub value: String,
    pub ip: IpAddress,
    pub port: terrane_int_support::Int,
}
impl SocketAddress {
    pub fn terrane_construct(
        raw: TerranePlatformResult,
        address_ip: IpAddress,
        address_port: terrane_int_support::Int,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            value: String::from(""),
            ip: IpAddress::terrane_construct(terrane_platform_failed_result()),
            port: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value.construct(raw, address_ip, address_port);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        raw: TerranePlatformResult,
        address_ip: IpAddress,
        address_port: terrane_int_support::Int,
    ) {
        self.value = terrane_platform_result_text(&raw);
        self.ip = address_ip;
        self.port = address_port.clone();
    }
    pub fn string(&self) -> String {
        return self.value.clone();
    }
}
#[derive(Clone)]
pub struct SocketResult {
    pub failed: bool,
    pub message: String,
    pub value: SocketAddress,
}
impl SocketResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        address: SocketAddress,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: SocketAddress::terrane_construct(
                terrane_platform_failed_result(),
                IpAddress::terrane_construct(terrane_platform_failed_result()),
                terrane_int_support::Int::from(0_i128),
            ),
        };
        __terrane_constructed_value.construct(failed, message, address);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, address: SocketAddress) {
        self.failed = failed;
        self.message = message;
        self.value = address;
    }
}
pub fn socket_address_from_ip(
    ip: IpAddress,
    port: terrane_int_support::Int,
) -> SocketResult {
    let raw_terrane_f1_s3549: TerranePlatformResult;
    let failed_terrane_f1_s3593: bool;
    let message_terrane_f1_s3630: String;
    let address_terrane_f1_s3669: SocketAddress;
    raw_terrane_f1_s3549 = terrane_platform_parse_socket(&ip.value, &port);
    failed_terrane_f1_s3593 = terrane_platform_result_failed(&raw_terrane_f1_s3549);
    message_terrane_f1_s3630 = terrane_platform_result_message(&raw_terrane_f1_s3549);
    address_terrane_f1_s3669 = SocketAddress::terrane_construct(
        raw_terrane_f1_s3549,
        ip,
        port.clone(),
    );
    return SocketResult::terrane_construct(
        failed_terrane_f1_s3593,
        message_terrane_f1_s3630,
        address_terrane_f1_s3669,
    );
}
pub fn socket_address_from_string(text: String) -> SocketResult {
    let raw_terrane_f1_s3846: TerranePlatformResult;
    let failed_terrane_f1_s3885: bool;
    let message_terrane_f1_s3922: String;
    let address_ip_terrane_f1_s3961: IpAddress;
    let port_terrane_f1_s4042: terrane_int_support::Int;
    let address_terrane_f1_s4074: SocketAddress;
    raw_terrane_f1_s3846 = terrane_platform_parse_socket_text(text);
    failed_terrane_f1_s3885 = terrane_platform_result_failed(&raw_terrane_f1_s3846);
    message_terrane_f1_s3922 = terrane_platform_result_message(&raw_terrane_f1_s3846);
    address_ip_terrane_f1_s3961 = IpAddress::terrane_construct(
        terrane_platform_parse_ip(terrane_platform_result_detail(&raw_terrane_f1_s3846)),
    );
    port_terrane_f1_s4042 = terrane_platform_result_int(&raw_terrane_f1_s3846);
    address_terrane_f1_s4074 = SocketAddress::terrane_construct(
        raw_terrane_f1_s3846,
        address_ip_terrane_f1_s3961,
        port_terrane_f1_s4042.clone(),
    );
    return SocketResult::terrane_construct(
        failed_terrane_f1_s3885,
        message_terrane_f1_s3922,
        address_terrane_f1_s4074,
    );
}
#[derive(Clone)]
pub struct IoResult {
    pub failed: bool,
    pub truncated: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub peer: String,
    pub end: bool,
}
impl IoResult {
    pub fn terrane_construct(
        failed: bool,
        truncated: bool,
        deadline_exceeded: bool,
        message: String,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        peer: String,
        end: bool,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            truncated: false,
            deadline_exceeded: false,
            message: String::from(""),
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            peer: String::from(""),
            end: false,
        };
        __terrane_constructed_value
            .construct(
                failed,
                truncated,
                deadline_exceeded,
                message,
                data,
                completed,
                peer,
                end,
            );
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        truncated: bool,
        deadline_exceeded: bool,
        message: String,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        peer: String,
        end: bool,
    ) {
        self.failed = failed;
        self.truncated = truncated;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.data = data;
        self.completed = completed.clone();
        self.peer = peer;
        self.end = end;
    }
}
pub struct TcpStream {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformCapability>,
}
impl TcpStream {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
        };
        __terrane_constructed_value.construct(resource);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = Some(resource);
    }
    pub async fn read(
        &self,
        limit: terrane_int_support::Int,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw_terrane_f1_s5061: TerranePlatformResult;
        raw_terrane_f1_s5061 = __terrane_await(
                terrane_platform_tcp_read_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    limit,
                    operation_deadline(options.clone()),
                    &options.cancellation.handle,
                ),
            )
            .await;
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5061),
            false,
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5061),
            terrane_platform_result_message(&raw_terrane_f1_s5061),
            terrane_platform_result_bytes(&raw_terrane_f1_s5061),
            terrane_platform_result_int(&raw_terrane_f1_s5061),
            String::from(""),
            terrane_platform_result_bool(&raw_terrane_f1_s5061),
        );
    }
    pub async fn write(
        &self,
        data: Vec<u8>,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw_terrane_f1_s5477: TerranePlatformResult;
        raw_terrane_f1_s5477 = __terrane_await(
                terrane_platform_tcp_write_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    data,
                    operation_deadline(options.clone()),
                    &options.cancellation.handle,
                ),
            )
            .await;
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5477),
            false,
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5477),
            terrane_platform_result_message(&raw_terrane_f1_s5477),
            Vec::from([]),
            terrane_platform_result_int(&raw_terrane_f1_s5477),
            String::from(""),
            false,
        );
    }
    pub fn configure(&self, options: TcpOptions) -> NetworkOperationResult {
        let raw_terrane_f1_s5841: TerranePlatformResult;
        raw_terrane_f1_s5841 = terrane_platform_tcp_configure(
            &self.handle.as_ref().expect("required field initialized"),
            options.no_delay,
            options.ttl,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5841),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5841),
            terrane_platform_result_message(&raw_terrane_f1_s5841),
        );
    }
    pub fn shutdown(&self, direction: String) -> NetworkOperationResult {
        let raw_terrane_f1_s6126: TerranePlatformResult;
        raw_terrane_f1_s6126 = terrane_platform_tcp_shutdown(
            &self.handle.as_ref().expect("required field initialized"),
            direction,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s6126),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s6126),
            terrane_platform_result_message(&raw_terrane_f1_s6126),
        );
    }
    pub fn close(self) -> NetworkOperationResult {
        let raw_terrane_f1_s6380: TerranePlatformResult;
        raw_terrane_f1_s6380 = terrane_platform_capability_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s6380),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s6380),
            terrane_platform_result_message(&raw_terrane_f1_s6380),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_capability_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for TcpStream {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct StreamResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub peer: String,
    pub value: TcpStream,
}
impl StreamResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        peer: String,
        stream: TcpStream,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            peer: String::from(""),
            value: TcpStream::terrane_construct(terrane_platform_no_resource()),
        };
        __terrane_constructed_value
            .construct(failed, deadline_exceeded, message, peer, stream);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        peer: String,
        stream: TcpStream,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.peer = peer;
        self.value = stream;
    }
}
pub async fn connect_tcp(
    address: SocketAddress,
    options: NetworkOperationOptions,
) -> StreamResult {
    let raw_terrane_f1_s7185: TerranePlatformResult;
    let stream_terrane_f1_s7299: TcpStream;
    raw_terrane_f1_s7185 = __terrane_await(
            terrane_platform_tcp_connect_async(
                address.value,
                operation_deadline(options.clone()),
                &options.cancellation.handle,
            ),
        )
        .await;
    stream_terrane_f1_s7299 = TcpStream::terrane_construct(
        terrane_platform_result_capability(&raw_terrane_f1_s7185),
    );
    return StreamResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f1_s7185),
        terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s7185),
        terrane_platform_result_message(&raw_terrane_f1_s7185),
        String::from(""),
        stream_terrane_f1_s7299,
    );
}
pub async fn connect_host(
    host: NetworkHostName,
    port: terrane_int_support::Int,
    options: NetworkOperationOptions,
) -> StreamResult {
    let raw_terrane_f1_s7612: TerranePlatformResult;
    let stream_terrane_f1_s7734: TcpStream;
    raw_terrane_f1_s7612 = __terrane_await(
            terrane_platform_tcp_connect_host_async(
                host.value,
                port,
                operation_deadline(options.clone()),
                &options.cancellation.handle,
            ),
        )
        .await;
    stream_terrane_f1_s7734 = TcpStream::terrane_construct(
        terrane_platform_result_capability(&raw_terrane_f1_s7612),
    );
    return StreamResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f1_s7612),
        terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s7612),
        terrane_platform_result_message(&raw_terrane_f1_s7612),
        terrane_platform_result_text(&raw_terrane_f1_s7612),
        stream_terrane_f1_s7734,
    );
}
pub struct TcpListener {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformCapability>,
    pub local_address: String,
}
impl TcpListener {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            local_address: String::from(""),
        };
        __terrane_constructed_value.construct(resource);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = Some(resource);
    }
    pub async fn accept(&self, options: NetworkOperationOptions) -> StreamResult {
        let raw_terrane_f1_s8238: TerranePlatformResult;
        let stream_terrane_f1_s8353: TcpStream;
        raw_terrane_f1_s8238 = __terrane_await(
                terrane_platform_tcp_accept_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    operation_deadline(options.clone()),
                    &options.cancellation.handle,
                ),
            )
            .await;
        stream_terrane_f1_s8353 = TcpStream::terrane_construct(
            terrane_platform_result_capability(&raw_terrane_f1_s8238),
        );
        return StreamResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s8238),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s8238),
            terrane_platform_result_message(&raw_terrane_f1_s8238),
            terrane_platform_result_text(&raw_terrane_f1_s8238),
            stream_terrane_f1_s8353,
        );
    }
    pub fn close(self) -> NetworkOperationResult {
        let raw_terrane_f1_s8639: TerranePlatformResult;
        raw_terrane_f1_s8639 = terrane_platform_capability_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s8639),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s8639),
            terrane_platform_result_message(&raw_terrane_f1_s8639),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_capability_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for TcpListener {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct ListenerResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub value: TcpListener,
}
impl ListenerResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        listener: TcpListener,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            value: TcpListener::terrane_construct(terrane_platform_no_resource()),
        };
        __terrane_constructed_value
            .construct(failed, deadline_exceeded, message, listener);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        listener: TcpListener,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.value = listener;
    }
}
pub fn bind_tcp(address: SocketAddress) -> ListenerResult {
    let raw_terrane_f1_s9355: TerranePlatformResult;
    let mut listener_terrane_f1_s9394: TcpListener;
    raw_terrane_f1_s9355 = terrane_platform_tcp_bind(address.value);
    listener_terrane_f1_s9394 = TcpListener::terrane_construct(
        terrane_platform_result_capability(&raw_terrane_f1_s9355),
    );
    if !terrane_platform_result_failed(&raw_terrane_f1_s9355) {
        listener_terrane_f1_s9394.local_address = terrane_platform_result_text(
            &raw_terrane_f1_s9355,
        );
    }
    return ListenerResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f1_s9355),
        terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s9355),
        terrane_platform_result_message(&raw_terrane_f1_s9355),
        listener_terrane_f1_s9394,
    );
}
pub struct UdpSocket {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformCapability>,
    pub local_address: String,
}
impl UdpSocket {
    pub fn terrane_construct(resource: TerranePlatformCapability) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            local_address: String::from(""),
        };
        __terrane_constructed_value.construct(resource);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(&mut self, resource: TerranePlatformCapability) {
        self.handle = Some(resource);
    }
    pub async fn send_to(
        &self,
        data: Vec<u8>,
        address: SocketAddress,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw_terrane_f1_s10004: TerranePlatformResult;
        raw_terrane_f1_s10004 = __terrane_await(
                terrane_platform_udp_send_to_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    data,
                    address.value,
                    operation_deadline(options.clone()),
                    &options.cancellation.handle,
                ),
            )
            .await;
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s10004),
            false,
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s10004),
            terrane_platform_result_message(&raw_terrane_f1_s10004),
            Vec::from([]),
            terrane_platform_result_int(&raw_terrane_f1_s10004),
            String::from(""),
            false,
        );
    }
    pub async fn receive_from(
        &self,
        limit: terrane_int_support::Int,
        options: NetworkOperationOptions,
    ) -> IoResult {
        let raw_terrane_f1_s10404: TerranePlatformResult;
        raw_terrane_f1_s10404 = __terrane_await(
                terrane_platform_udp_receive_from_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    limit,
                    operation_deadline(options.clone()),
                    &options.cancellation.handle,
                ),
            )
            .await;
        return IoResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s10404),
            terrane_platform_result_truncated(&raw_terrane_f1_s10404),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s10404),
            terrane_platform_result_message(&raw_terrane_f1_s10404),
            terrane_platform_result_bytes(&raw_terrane_f1_s10404),
            terrane_platform_result_int(&raw_terrane_f1_s10404),
            terrane_platform_result_text(&raw_terrane_f1_s10404),
            false,
        );
    }
    pub fn configure(&self, options: UdpOptions) -> NetworkOperationResult {
        let raw_terrane_f1_s10841: TerranePlatformResult;
        raw_terrane_f1_s10841 = terrane_platform_udp_configure(
            &self.handle.as_ref().expect("required field initialized"),
            options.broadcast,
            options.ttl,
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s10841),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s10841),
            terrane_platform_result_message(&raw_terrane_f1_s10841),
        );
    }
    pub fn close(self) -> NetworkOperationResult {
        let raw_terrane_f1_s11117: TerranePlatformResult;
        raw_terrane_f1_s11117 = terrane_platform_capability_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return NetworkOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s11117),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s11117),
            terrane_platform_result_message(&raw_terrane_f1_s11117),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_capability_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for UdpSocket {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct UdpResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub value: UdpSocket,
}
impl UdpResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        socket: UdpSocket,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            value: UdpSocket::terrane_construct(terrane_platform_no_resource()),
        };
        __terrane_constructed_value
            .construct(failed, deadline_exceeded, message, socket);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        socket: UdpSocket,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.value = socket;
    }
}
pub fn bind_udp(address: SocketAddress) -> UdpResult {
    let raw_terrane_f1_s11813: TerranePlatformResult;
    let mut socket_terrane_f1_s11852: UdpSocket;
    raw_terrane_f1_s11813 = terrane_platform_udp_bind(address.value);
    socket_terrane_f1_s11852 = UdpSocket::terrane_construct(
        terrane_platform_result_capability(&raw_terrane_f1_s11813),
    );
    if !terrane_platform_result_failed(&raw_terrane_f1_s11813) {
        socket_terrane_f1_s11852.local_address = terrane_platform_result_text(
            &raw_terrane_f1_s11813,
        );
    }
    return UdpResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f1_s11813),
        terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s11813),
        terrane_platform_result_message(&raw_terrane_f1_s11813),
        socket_terrane_f1_s11852,
    );
}
#[derive(Clone)]
pub struct DnsResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub candidates: terrane_collection_support::List<String>,
    pub ttl: terrane_int_support::Int,
    pub ttl_known: bool,
}
impl DnsResult {
    pub fn terrane_construct(
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        ttl: terrane_int_support::Int,
        ttl_known: bool,
        candidates: terrane_collection_support::List<String>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
            candidates: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            ttl: terrane_int_support::Int::from(0_i128),
            ttl_known: false,
        };
        __terrane_constructed_value
            .construct(failed, deadline_exceeded, message, ttl, ttl_known, candidates);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        failed: bool,
        deadline_exceeded: bool,
        message: String,
        ttl: terrane_int_support::Int,
        ttl_known: bool,
        candidates: terrane_collection_support::List<String>,
    ) {
        self.failed = failed;
        self.deadline_exceeded = deadline_exceeded;
        self.message = message;
        self.ttl = ttl.clone();
        self.ttl_known = ttl_known;
        self.candidates = candidates;
    }
}
#[derive(Clone)]
pub struct NetworkHostName {
    pub value: String,
}
impl NetworkHostName {
    pub fn terrane_construct(raw: TerranePlatformResult) -> Self {
        let mut __terrane_constructed_value = Self { value: String::from("") };
        __terrane_constructed_value.construct(raw);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, raw: TerranePlatformResult) {
        self.value = terrane_platform_result_text(&raw);
    }
}
#[derive(Clone)]
pub struct NetworkHostNameResult {
    pub failed: bool,
    pub message: String,
    pub value: NetworkHostName,
}
impl NetworkHostNameResult {
    pub fn terrane_construct(
        failed: bool,
        message: String,
        host: NetworkHostName,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            value: NetworkHostName::terrane_construct(terrane_platform_failed_result()),
        };
        __terrane_constructed_value.construct(failed, message, host);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String, host: NetworkHostName) {
        self.failed = failed;
        self.message = message;
        self.value = host;
    }
}
pub fn parse_host_name(text: String) -> NetworkHostNameResult {
    let raw_terrane_f1_s13191: TerranePlatformResult;
    let failed_terrane_f1_s13228: bool;
    let message_terrane_f1_s13265: String;
    let host_terrane_f1_s13304: NetworkHostName;
    raw_terrane_f1_s13191 = terrane_platform_parse_host_name(text);
    failed_terrane_f1_s13228 = terrane_platform_result_failed(&raw_terrane_f1_s13191);
    message_terrane_f1_s13265 = terrane_platform_result_message(&raw_terrane_f1_s13191);
    host_terrane_f1_s13304 = NetworkHostName::terrane_construct(raw_terrane_f1_s13191);
    return NetworkHostNameResult::terrane_construct(
        failed_terrane_f1_s13228,
        message_terrane_f1_s13265,
        host_terrane_f1_s13304,
    );
}
pub async fn lookup_dns(
    host: NetworkHostName,
    port: terrane_int_support::Int,
    options: NetworkOperationOptions,
) -> DnsResult {
    let raw_terrane_f1_s13522: TerranePlatformResult;
    let raw_candidates_terrane_f1_s13638: Vec<String>;
    let mut candidates_terrane_f1_s13684: terrane_collection_support::List<String>;
    let mut index_terrane_f1_s13725: terrane_int_support::Int;
    raw_terrane_f1_s13522 = __terrane_await(
            terrane_platform_dns_lookup_async(
                host.value,
                port,
                operation_deadline(options.clone()),
                &options.cancellation.handle,
            ),
        )
        .await;
    raw_candidates_terrane_f1_s13638 = terrane_platform_result_entries(
        &raw_terrane_f1_s13522,
    );
    candidates_terrane_f1_s13684 = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("")]);
    index_terrane_f1_s13725 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = candidates_terrane_f1_s13684.make_unique();
        while index_terrane_f1_s13725.clone()
            < terrane_int_support::Int::from(
                raw_candidates_terrane_f1_s13638.len() as i128,
            )
        {
            __terrane_list_append_0
                .push(
                    __terrane_raised(
                        {
                            let __terrane_receiver = &raw_candidates_terrane_f1_s13638;
                            let __terrane_index = __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &index_terrane_f1_s13725.clone(),
                                ),
                                2 /* terrane-site: core/networking.trn:328:28-328:49 */,
                            );
                            __terrane_receiver
                                .get(__terrane_index)
                                .cloned()
                                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                    __terrane_index,
                                ))
                        },
                        2 /* terrane-site: core/networking.trn:328:28-328:49 */,
                    ),
                );
            index_terrane_f1_s13725 = index_terrane_f1_s13725.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    return DnsResult::terrane_construct(
        terrane_platform_result_failed(&raw_terrane_f1_s13522),
        terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s13522),
        terrane_platform_result_message(&raw_terrane_f1_s13522),
        terrane_platform_result_int(&raw_terrane_f1_s13522),
        terrane_platform_result_bool(&raw_terrane_f1_s13522),
        candidates_terrane_f1_s13684,
    );
}
// Source: core/streams.trn
// Namespace: core/streams
#[derive(Clone)]
pub struct StreamOperationResult {
    pub failed: bool,
    pub message: String,
}
impl StreamOperationResult {
    pub fn terrane_construct(failed: bool, message: String) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(failed, message);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, failed: bool, message: String) {
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct ReadResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl ReadResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(data, completed, end, failed, message);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) {
        self.data = data;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct TextReadResult {
    pub text: String,
    pub completed: terrane_int_support::Int,
    pub end: bool,
    pub failed: bool,
    pub message: String,
}
impl TextReadResult {
    pub fn terrane_construct(
        text: String,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            text: String::from(""),
            completed: terrane_int_support::Int::from(0_i128),
            end: false,
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(text, completed, end, failed, message);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        text: String,
        completed: terrane_int_support::Int,
        end: bool,
        failed: bool,
        message: String,
    ) {
        self.text = text;
        self.completed = completed.clone();
        self.end = end;
        self.failed = failed;
        self.message = message;
    }
}
#[derive(Clone)]
pub struct WriteResult {
    pub data: Vec<u8>,
    pub completed: terrane_int_support::Int,
    pub failed: bool,
    pub message: String,
}
impl WriteResult {
    pub fn terrane_construct(
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            data: Vec::from([]),
            completed: terrane_int_support::Int::from(0_i128),
            failed: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(data, completed, failed, message);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        data: Vec<u8>,
        completed: terrane_int_support::Int,
        failed: bool,
        message: String,
    ) {
        if completed.clone() == terrane_int_support::Int::from(data.len() as i128)
            && !failed
        {
            self.data = Vec::from([]);
        } else {
            self.data = data;
        }
        self.completed = completed.clone();
        self.failed = failed;
        self.message = message;
    }
}
pub struct ByteReader {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
}
impl ByteReader {
    pub fn terrane_construct(handle: TerranePlatformStreamHandle) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
        };
        __terrane_constructed_value.construct(handle);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(&mut self, handle: TerranePlatformStreamHandle) {
        self.handle = Some(handle);
    }
    pub fn read(&self, count: terrane_int_support::Int) -> ReadResult {
        let raw_terrane_f2_s1627: TerranePlatformReadResult;
        raw_terrane_f2_s1627 = terrane_platform_read(
            &self.handle.as_ref().expect("required field initialized"),
            count,
        );
        return ReadResult::terrane_construct(
            raw_terrane_f2_s1627.data.clone().clone(),
            raw_terrane_f2_s1627.completed.clone(),
            raw_terrane_f2_s1627.end,
            raw_terrane_f2_s1627.failed,
            raw_terrane_f2_s1627.message.clone().clone(),
        );
    }
    pub fn read_exact(&self, count: terrane_int_support::Int) -> ReadResult {
        let mut data_terrane_f2_s1814: Vec<u8>;
        let mut completed_terrane_f2_s1839: terrane_int_support::Int;
        let mut end_terrane_f2_s1865: bool;
        let mut failed_terrane_f2_s1890: bool;
        let mut message_terrane_f2_s1918: String;
        let mut part_terrane_f2_s2009: TerranePlatformReadResult;
        data_terrane_f2_s1814 = Vec::from([]);
        completed_terrane_f2_s1839 = terrane_int_support::Int::from(0_i128);
        end_terrane_f2_s1865 = false;
        failed_terrane_f2_s1890 = false;
        message_terrane_f2_s1918 = String::from("");
        while completed_terrane_f2_s1839.clone() < count.clone() && !end_terrane_f2_s1865
            && !failed_terrane_f2_s1890
        {
            part_terrane_f2_s2009 = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                count.clone() - completed_terrane_f2_s1839.clone(),
            );
            data_terrane_f2_s1814 = {
                let mut bytes = data_terrane_f2_s1814;
                let part_0: Vec<u8> = part_terrane_f2_s2009.data.clone();
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            completed_terrane_f2_s1839 = completed_terrane_f2_s1839.clone()
                + part_terrane_f2_s2009.completed.clone();
            end_terrane_f2_s1865 = part_terrane_f2_s2009.end;
            failed_terrane_f2_s1890 = part_terrane_f2_s2009.failed;
            message_terrane_f2_s1918 = part_terrane_f2_s2009.message.clone().clone();
            if part_terrane_f2_s2009.completed.clone()
                == terrane_int_support::Int::from(0_i128) && !part_terrane_f2_s2009.end
                && !part_terrane_f2_s2009.failed
            {
                failed_terrane_f2_s1890 = true;
                message_terrane_f2_s1918 = String::from("stream read made no progress");
            }
        }
        if end_terrane_f2_s1865 && completed_terrane_f2_s1839.clone() < count.clone()
            && !failed_terrane_f2_s1890
        {
            failed_terrane_f2_s1890 = true;
            message_terrane_f2_s1918 = String::from(
                "stream ended before exact byte count",
            );
        }
        return ReadResult::terrane_construct(
            data_terrane_f2_s1814,
            completed_terrane_f2_s1839.clone(),
            end_terrane_f2_s1865,
            failed_terrane_f2_s1890,
            message_terrane_f2_s1918,
        );
    }
    pub fn read_all(&self, limit: terrane_int_support::Int) -> ReadResult {
        let mut data_terrane_f2_s2673: Vec<u8>;
        let mut completed_terrane_f2_s2698: terrane_int_support::Int;
        let mut end_terrane_f2_s2724: bool;
        let mut failed_terrane_f2_s2749: bool;
        let mut message_terrane_f2_s2777: String;
        let mut part_terrane_f2_s2868: TerranePlatformReadResult;
        data_terrane_f2_s2673 = Vec::from([]);
        completed_terrane_f2_s2698 = terrane_int_support::Int::from(0_i128);
        end_terrane_f2_s2724 = false;
        failed_terrane_f2_s2749 = false;
        message_terrane_f2_s2777 = String::from("");
        while completed_terrane_f2_s2698.clone() < limit.clone() && !end_terrane_f2_s2724
            && !failed_terrane_f2_s2749
        {
            part_terrane_f2_s2868 = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                limit.clone() - completed_terrane_f2_s2698.clone(),
            );
            data_terrane_f2_s2673 = {
                let mut bytes = data_terrane_f2_s2673;
                let part_0: Vec<u8> = part_terrane_f2_s2868.data.clone();
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            completed_terrane_f2_s2698 = completed_terrane_f2_s2698.clone()
                + part_terrane_f2_s2868.completed.clone();
            end_terrane_f2_s2724 = part_terrane_f2_s2868.end;
            failed_terrane_f2_s2749 = part_terrane_f2_s2868.failed;
            message_terrane_f2_s2777 = part_terrane_f2_s2868.message.clone().clone();
            if part_terrane_f2_s2868.completed.clone()
                == terrane_int_support::Int::from(0_i128) && !part_terrane_f2_s2868.end
                && !part_terrane_f2_s2868.failed
            {
                failed_terrane_f2_s2749 = true;
                message_terrane_f2_s2777 = String::from("stream read made no progress");
            }
        }
        return ReadResult::terrane_construct(
            data_terrane_f2_s2673,
            completed_terrane_f2_s2698.clone(),
            end_terrane_f2_s2724,
            failed_terrane_f2_s2749,
            message_terrane_f2_s2777,
        );
    }
    pub async fn read_async(&self, count: terrane_int_support::Int) -> ReadResult {
        let raw_terrane_f2_s3401: TerranePlatformReadResult;
        raw_terrane_f2_s3401 = __terrane_await(
                terrane_platform_read_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    count,
                ),
            )
            .await;
        return ReadResult::terrane_construct(
            raw_terrane_f2_s3401.data.clone().clone(),
            raw_terrane_f2_s3401.completed.clone(),
            raw_terrane_f2_s3401.end,
            raw_terrane_f2_s3401.failed,
            raw_terrane_f2_s3401.message.clone().clone(),
        );
    }
    pub fn text(&self, codec: terrane_string_support::Encoding) -> TextReader {
        return TextReader::terrane_construct(
            self.handle.as_ref().expect("required field initialized").clone(),
            codec,
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw_terrane_f2_s3710: TerranePlatformUnitResult;
        raw_terrane_f2_s3710 = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s3710.failed,
            raw_terrane_f2_s3710.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for ByteReader {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct ByteWriter {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
}
impl ByteWriter {
    pub fn terrane_construct(handle: TerranePlatformStreamHandle) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
        };
        __terrane_constructed_value.construct(handle);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(&mut self, handle: TerranePlatformStreamHandle) {
        self.handle = Some(handle);
    }
    pub fn write(&self, data: Vec<u8>) -> WriteResult {
        let offset_terrane_f2_s4175: i64;
        let raw_terrane_f2_s4198: TerranePlatformWriteResult;
        offset_terrane_f2_s4175 = 0;
        raw_terrane_f2_s4198 = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &data,
            terrane_int_support::Int::from(offset_terrane_f2_s4175.clone()),
        );
        return WriteResult::terrane_construct(
            data,
            raw_terrane_f2_s4198.completed.clone(),
            raw_terrane_f2_s4198.failed,
            raw_terrane_f2_s4198.message.clone().clone(),
        );
    }
    pub fn write_all(&self, data: Vec<u8>) -> WriteResult {
        let mut completed_terrane_f2_s4382: terrane_int_support::Int;
        let mut failed_terrane_f2_s4408: bool;
        let mut message_terrane_f2_s4436: String;
        let mut part_terrane_f2_s4521: TerranePlatformWriteResult;
        completed_terrane_f2_s4382 = terrane_int_support::Int::from(0_i128);
        failed_terrane_f2_s4408 = false;
        message_terrane_f2_s4436 = String::from("");
        while completed_terrane_f2_s4382.clone()
            < terrane_int_support::Int::from(data.len() as i128)
            && !failed_terrane_f2_s4408
        {
            part_terrane_f2_s4521 = terrane_platform_write(
                &self.handle.as_ref().expect("required field initialized"),
                &data,
                terrane_int_support::Int::from(completed_terrane_f2_s4382.clone()),
            );
            completed_terrane_f2_s4382 = completed_terrane_f2_s4382.clone()
                + part_terrane_f2_s4521.completed.clone();
            failed_terrane_f2_s4408 = part_terrane_f2_s4521.failed;
            message_terrane_f2_s4436 = part_terrane_f2_s4521.message.clone().clone();
            if part_terrane_f2_s4521.completed.clone()
                == terrane_int_support::Int::from(0_i128)
                && !part_terrane_f2_s4521.failed
            {
                failed_terrane_f2_s4408 = true;
                message_terrane_f2_s4436 = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(
            data,
            completed_terrane_f2_s4382.clone(),
            failed_terrane_f2_s4408,
            message_terrane_f2_s4436,
        );
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        let raw_terrane_f2_s5023: TerranePlatformWriteResult;
        if terrane_int_support::Int::from(prior.data.len() as i128)
            == terrane_int_support::Int::from(0_i128)
        {
            return prior.clone();
        }
        raw_terrane_f2_s5023 = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw_terrane_f2_s5023.completed.clone(),
            raw_terrane_f2_s5023.failed,
            raw_terrane_f2_s5023.message.clone().clone(),
        );
    }
    pub async fn write_async(&self, data: Vec<u8>) -> WriteResult {
        return self.write(data);
    }
    pub fn text(&self, codec: terrane_string_support::Encoding) -> TextWriter {
        return TextWriter::terrane_construct(
            self.handle.as_ref().expect("required field initialized").clone(),
            codec,
        );
    }
    pub fn flush(&self) -> StreamOperationResult {
        let raw_terrane_f2_s5434: TerranePlatformUnitResult;
        raw_terrane_f2_s5434 = terrane_platform_flush(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s5434.failed,
            raw_terrane_f2_s5434.message.clone().clone(),
        );
    }
    pub fn sync_data(&self) -> StreamOperationResult {
        let raw_terrane_f2_s5594: TerranePlatformUnitResult;
        raw_terrane_f2_s5594 = terrane_platform_sync_data(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s5594.failed,
            raw_terrane_f2_s5594.message.clone().clone(),
        );
    }
    pub fn sync_all(&self) -> StreamOperationResult {
        let raw_terrane_f2_s5757: TerranePlatformUnitResult;
        raw_terrane_f2_s5757 = terrane_platform_sync_all(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s5757.failed,
            raw_terrane_f2_s5757.message.clone().clone(),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw_terrane_f2_s5926: TerranePlatformUnitResult;
        raw_terrane_f2_s5926 = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s5926.failed,
            raw_terrane_f2_s5926.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for ByteWriter {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct TextReader {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
    pub codec: terrane_string_support::Encoding,
}
impl TextReader {
    pub fn terrane_construct(
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            codec: terrane_string_support::Encoding::Utf8,
        };
        __terrane_constructed_value.construct(handle, codec);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) {
        self.handle = Some(handle);
        self.codec = codec;
    }
    pub fn read(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw_terrane_f2_s6482: TerranePlatformReadResult;
        let text_terrane_f2_s6526: String;
        raw_terrane_f2_s6482 = terrane_platform_read(
            &self.handle.as_ref().expect("required field initialized"),
            count,
        );
        text_terrane_f2_s6526 = __terrane_raised_err(
            terrane_string_support::decode(
                &raw_terrane_f2_s6482.data.clone(),
                self.codec,
            ),
            3 /* terrane-site: core/streams.trn:188:23-188:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text_terrane_f2_s6526,
                raw_terrane_f2_s6482.completed.clone(),
                raw_terrane_f2_s6482.end,
                raw_terrane_f2_s6482.failed,
                raw_terrane_f2_s6482.message.clone().clone(),
            ),
        );
    }
    pub fn read_exact(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data_terrane_f2_s6745: Vec<u8>;
        let mut completed_terrane_f2_s6770: terrane_int_support::Int;
        let mut end_terrane_f2_s6796: bool;
        let mut failed_terrane_f2_s6821: bool;
        let mut message_terrane_f2_s6849: String;
        let mut part_terrane_f2_s6940: TerranePlatformReadResult;
        let text_terrane_f2_s7483: String;
        data_terrane_f2_s6745 = Vec::from([]);
        completed_terrane_f2_s6770 = terrane_int_support::Int::from(0_i128);
        end_terrane_f2_s6796 = false;
        failed_terrane_f2_s6821 = false;
        message_terrane_f2_s6849 = String::from("");
        while completed_terrane_f2_s6770.clone() < count.clone() && !end_terrane_f2_s6796
            && !failed_terrane_f2_s6821
        {
            part_terrane_f2_s6940 = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                count.clone() - completed_terrane_f2_s6770.clone(),
            );
            data_terrane_f2_s6745 = {
                let mut bytes = data_terrane_f2_s6745;
                let part_0: Vec<u8> = part_terrane_f2_s6940.data.clone();
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            completed_terrane_f2_s6770 = completed_terrane_f2_s6770.clone()
                + part_terrane_f2_s6940.completed.clone();
            end_terrane_f2_s6796 = part_terrane_f2_s6940.end;
            failed_terrane_f2_s6821 = part_terrane_f2_s6940.failed;
            message_terrane_f2_s6849 = part_terrane_f2_s6940.message.clone().clone();
            if part_terrane_f2_s6940.completed.clone()
                == terrane_int_support::Int::from(0_i128) && !part_terrane_f2_s6940.end
                && !part_terrane_f2_s6940.failed
            {
                failed_terrane_f2_s6821 = true;
                message_terrane_f2_s6849 = String::from("stream read made no progress");
            }
        }
        if end_terrane_f2_s6796 && completed_terrane_f2_s6770.clone() < count.clone()
            && !failed_terrane_f2_s6821
        {
            failed_terrane_f2_s6821 = true;
            message_terrane_f2_s6849 = String::from(
                "stream ended before exact byte count",
            );
        }
        text_terrane_f2_s7483 = __terrane_raised_err(
            terrane_string_support::decode(&data_terrane_f2_s6745, self.codec),
            4 /* terrane-site: core/streams.trn:210:23-210:46 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text_terrane_f2_s7483,
                completed_terrane_f2_s6770.clone(),
                end_terrane_f2_s6796,
                failed_terrane_f2_s6821,
                message_terrane_f2_s6849,
            ),
        );
    }
    pub fn read_all(
        &self,
        limit: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let mut data_terrane_f2_s7680: Vec<u8>;
        let mut completed_terrane_f2_s7705: terrane_int_support::Int;
        let mut end_terrane_f2_s7731: bool;
        let mut failed_terrane_f2_s7756: bool;
        let mut message_terrane_f2_s7784: String;
        let mut part_terrane_f2_s7875: TerranePlatformReadResult;
        let text_terrane_f2_s8279: String;
        data_terrane_f2_s7680 = Vec::from([]);
        completed_terrane_f2_s7705 = terrane_int_support::Int::from(0_i128);
        end_terrane_f2_s7731 = false;
        failed_terrane_f2_s7756 = false;
        message_terrane_f2_s7784 = String::from("");
        while completed_terrane_f2_s7705.clone() < limit.clone() && !end_terrane_f2_s7731
            && !failed_terrane_f2_s7756
        {
            part_terrane_f2_s7875 = terrane_platform_read(
                &self.handle.as_ref().expect("required field initialized"),
                limit.clone() - completed_terrane_f2_s7705.clone(),
            );
            data_terrane_f2_s7680 = {
                let mut bytes = data_terrane_f2_s7680;
                let part_0: Vec<u8> = part_terrane_f2_s7875.data.clone();
                let additional = match [part_0.len()]
                    .into_iter()
                    .try_fold(0usize, usize::checked_add)
                {
                    Some(length) => length,
                    None => std::process::abort(),
                };
                if bytes.try_reserve(additional).is_err() {
                    std::process::abort();
                }
                bytes.extend(part_0);
                bytes
            };
            completed_terrane_f2_s7705 = completed_terrane_f2_s7705.clone()
                + part_terrane_f2_s7875.completed.clone();
            end_terrane_f2_s7731 = part_terrane_f2_s7875.end;
            failed_terrane_f2_s7756 = part_terrane_f2_s7875.failed;
            message_terrane_f2_s7784 = part_terrane_f2_s7875.message.clone().clone();
            if part_terrane_f2_s7875.completed.clone()
                == terrane_int_support::Int::from(0_i128) && !part_terrane_f2_s7875.end
                && !part_terrane_f2_s7875.failed
            {
                failed_terrane_f2_s7756 = true;
                message_terrane_f2_s7784 = String::from("stream read made no progress");
            }
        }
        text_terrane_f2_s8279 = __terrane_raised_err(
            terrane_string_support::decode(&data_terrane_f2_s7680, self.codec),
            5 /* terrane-site: core/streams.trn:229:23-229:46 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text_terrane_f2_s8279,
                completed_terrane_f2_s7705.clone(),
                end_terrane_f2_s7731,
                failed_terrane_f2_s7756,
                message_terrane_f2_s7784,
            ),
        );
    }
    pub async fn read_async(
        &self,
        count: terrane_int_support::Int,
    ) -> Result<TextReadResult, TerraneError> {
        let raw_terrane_f2_s8484: TerranePlatformReadResult;
        let text_terrane_f2_s8540: String;
        raw_terrane_f2_s8484 = __terrane_await(
                terrane_platform_read_async(
                    &self.handle.as_ref().expect("required field initialized"),
                    count,
                ),
            )
            .await;
        text_terrane_f2_s8540 = __terrane_raised_err(
            terrane_string_support::decode(
                &raw_terrane_f2_s8484.data.clone(),
                self.codec,
            ),
            6 /* terrane-site: core/streams.trn:234:23-234:50 */,
        )?;
        return Ok(
            TextReadResult::terrane_construct(
                text_terrane_f2_s8540,
                raw_terrane_f2_s8484.completed.clone(),
                raw_terrane_f2_s8484.end,
                raw_terrane_f2_s8484.failed,
                raw_terrane_f2_s8484.message.clone().clone(),
            ),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw_terrane_f2_s8741: TerranePlatformUnitResult;
        raw_terrane_f2_s8741 = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s8741.failed,
            raw_terrane_f2_s8741.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for TextReader {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub struct TextWriter {
    __terrane_constructed: bool,
    pub handle: Option<TerranePlatformStreamHandle>,
    pub codec: terrane_string_support::Encoding,
}
impl TextWriter {
    pub fn terrane_construct(
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            __terrane_constructed: false,
            handle: None,
            codec: terrane_string_support::Encoding::Utf8,
        };
        __terrane_constructed_value.construct(handle, codec);
        __terrane_constructed_value.__terrane_constructed = true;
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        handle: TerranePlatformStreamHandle,
        codec: terrane_string_support::Encoding,
    ) {
        self.handle = Some(handle);
        self.codec = codec;
    }
    pub fn write(&self, text: String) -> WriteResult {
        let data_terrane_f2_s9276: Vec<u8>;
        let offset_terrane_f2_s9321: i64;
        let raw_terrane_f2_s9344: TerranePlatformWriteResult;
        data_terrane_f2_s9276 = terrane_string_support::encode(&text, self.codec);
        offset_terrane_f2_s9321 = 0;
        raw_terrane_f2_s9344 = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &data_terrane_f2_s9276,
            terrane_int_support::Int::from(offset_terrane_f2_s9321.clone()),
        );
        return WriteResult::terrane_construct(
            data_terrane_f2_s9276,
            raw_terrane_f2_s9344.completed.clone(),
            raw_terrane_f2_s9344.failed,
            raw_terrane_f2_s9344.message.clone().clone(),
        );
    }
    pub fn write_all(&self, text: String) -> WriteResult {
        let data_terrane_f2_s9528: Vec<u8>;
        let mut completed_terrane_f2_s9573: terrane_int_support::Int;
        let mut failed_terrane_f2_s9599: bool;
        let mut message_terrane_f2_s9627: String;
        let mut part_terrane_f2_s9712: TerranePlatformWriteResult;
        data_terrane_f2_s9528 = terrane_string_support::encode(&text, self.codec);
        completed_terrane_f2_s9573 = terrane_int_support::Int::from(0_i128);
        failed_terrane_f2_s9599 = false;
        message_terrane_f2_s9627 = String::from("");
        while completed_terrane_f2_s9573.clone()
            < terrane_int_support::Int::from(data_terrane_f2_s9528.len() as i128)
            && !failed_terrane_f2_s9599
        {
            part_terrane_f2_s9712 = terrane_platform_write(
                &self.handle.as_ref().expect("required field initialized"),
                &data_terrane_f2_s9528,
                terrane_int_support::Int::from(completed_terrane_f2_s9573.clone()),
            );
            completed_terrane_f2_s9573 = completed_terrane_f2_s9573.clone()
                + part_terrane_f2_s9712.completed.clone();
            failed_terrane_f2_s9599 = part_terrane_f2_s9712.failed;
            message_terrane_f2_s9627 = part_terrane_f2_s9712.message.clone().clone();
            if part_terrane_f2_s9712.completed.clone()
                == terrane_int_support::Int::from(0_i128)
                && !part_terrane_f2_s9712.failed
            {
                failed_terrane_f2_s9599 = true;
                message_terrane_f2_s9627 = String::from("stream write made no progress");
            }
        }
        return WriteResult::terrane_construct(
            data_terrane_f2_s9528,
            completed_terrane_f2_s9573.clone(),
            failed_terrane_f2_s9599,
            message_terrane_f2_s9627,
        );
    }
    pub fn resume(&self, prior: WriteResult) -> WriteResult {
        let raw_terrane_f2_s10214: TerranePlatformWriteResult;
        if terrane_int_support::Int::from(prior.data.len() as i128)
            == terrane_int_support::Int::from(0_i128)
        {
            return prior.clone();
        }
        raw_terrane_f2_s10214 = terrane_platform_write(
            &self.handle.as_ref().expect("required field initialized"),
            &prior.data,
            terrane_int_support::Int::from(prior.completed.clone()),
        );
        return WriteResult::terrane_construct(
            prior.data.clone(),
            prior.completed.clone() + raw_terrane_f2_s10214.completed.clone(),
            raw_terrane_f2_s10214.failed,
            raw_terrane_f2_s10214.message.clone().clone(),
        );
    }
    pub fn line(&self, text: String) -> WriteResult {
        return self
            .write_all(
                format!(
                    "{}{}", terrane_scalar_support::scalar_text(&text),
                    terrane_scalar_support::scalar_text(&String::from("\n"))
                ),
            );
    }
    pub async fn write_async(&self, text: String) -> WriteResult {
        return self.write(text);
    }
    pub fn flush(&self) -> StreamOperationResult {
        let raw_terrane_f2_s10619: TerranePlatformUnitResult;
        raw_terrane_f2_s10619 = terrane_platform_flush(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s10619.failed,
            raw_terrane_f2_s10619.message.clone().clone(),
        );
    }
    pub fn sync_data(&self) -> StreamOperationResult {
        let raw_terrane_f2_s10779: TerranePlatformUnitResult;
        raw_terrane_f2_s10779 = terrane_platform_sync_data(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s10779.failed,
            raw_terrane_f2_s10779.message.clone().clone(),
        );
    }
    pub fn sync_all(&self) -> StreamOperationResult {
        let raw_terrane_f2_s10942: TerranePlatformUnitResult;
        raw_terrane_f2_s10942 = terrane_platform_sync_all(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s10942.failed,
            raw_terrane_f2_s10942.message.clone().clone(),
        );
    }
    pub fn close(self) -> StreamOperationResult {
        let raw_terrane_f2_s11111: TerranePlatformUnitResult;
        raw_terrane_f2_s11111 = terrane_platform_close(
            &self.handle.as_ref().expect("required field initialized"),
        );
        return StreamOperationResult::terrane_construct(
            raw_terrane_f2_s11111.failed,
            raw_terrane_f2_s11111.message.clone().clone(),
        );
    }
    pub fn destruct(&mut self) {
        terrane_platform_release(
            &self.handle.as_ref().expect("required field initialized"),
        );
    }
}
impl Drop for TextWriter {
    fn drop(&mut self) {
        if !self.__terrane_constructed {
            return;
        }
        self.destruct();
    }
}
pub fn stdin() -> ByteReader {
    return ByteReader::terrane_construct(terrane_platform_acquire_stdin());
}
pub fn stdout() -> ByteWriter {
    return ByteWriter::terrane_construct(terrane_platform_acquire_stdout());
}
pub fn stderr() -> ByteWriter {
    return ByteWriter::terrane_construct(terrane_platform_acquire_stderr());
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
        let difference_terrane_f3_s602: terrane_int_support::Int;
        if self.total_nanoseconds.clone() < other.total_nanoseconds.clone() {
            return None;
        }
        difference_terrane_f3_s602 = self.total_nanoseconds.clone()
            - other.total_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(
                    &difference_terrane_f3_s602,
                    1000000000.clone(),
                ),
                terrane_platform_time_mod(
                    &difference_terrane_f3_s602,
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
        let fractional_terrane_f3_s2245: terrane_int_support::Int;
        fractional_terrane_f3_s2245 = self.nanoseconds.clone()
            + other.nanoseconds.clone();
        return Duration::terrane_construct(
            self.seconds.clone() + other.seconds.clone()
                + terrane_platform_time_div(
                    &fractional_terrane_f3_s2245,
                    1000000000.clone(),
                ),
            terrane_platform_time_mod(&fractional_terrane_f3_s2245, 1000000000.clone()),
        );
    }
    pub fn multiply(
        &self,
        multiplier: terrane_int_support::Int,
    ) -> Result<Duration, TerraneError> {
        let total_terrane_f3_s2592: terrane_int_support::Int;
        if multiplier.clone() < terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    7 /* terrane-site: core/time.trn:62:13-62:45 */,
                )
            });
        }
        total_terrane_f3_s2592 = self.total_nanoseconds.clone() * multiplier.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&total_terrane_f3_s2592, 1000000000.clone()),
                terrane_platform_time_mod(&total_terrane_f3_s2592, 1000000000.clone()),
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
                    TerraneErrorKind::Custom(DescriptorId(0)),
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
                    TerraneErrorKind::Custom(DescriptorId(0)),
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
                    TerraneErrorKind::Custom(DescriptorId(0)),
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
        let elapsed_terrane_f3_s3217: terrane_int_support::Int;
        if self.domain.clone() != later.domain.clone().clone()
            || later.elapsed_nanoseconds.clone().clone()
                < self.elapsed_nanoseconds.clone()
        {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    12 /* terrane-site: core/time.trn:76:13-76:45 */,
                )
            });
        }
        elapsed_terrane_f3_s3217 = later.elapsed_nanoseconds.clone().clone()
            - self.elapsed_nanoseconds.clone();
        return Ok(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed_terrane_f3_s3217, 1000000000.clone()),
                terrane_platform_time_mod(&elapsed_terrane_f3_s3217, 1000000000.clone()),
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
        let now_terrane_f3_s3834: MonotonicInstant;
        let elapsed_terrane_f3_s3964: terrane_int_support::Int;
        now_terrane_f3_s3834 = Clock::terrane_static_monotonic();
        if self.expires_at.elapsed_nanoseconds.clone()
            <= now_terrane_f3_s3834.elapsed_nanoseconds.clone()
        {
            return None;
        }
        elapsed_terrane_f3_s3964 = self.expires_at.elapsed_nanoseconds.clone()
            - now_terrane_f3_s3834.elapsed_nanoseconds.clone();
        return Some(
            Duration::terrane_construct(
                terrane_platform_time_div(&elapsed_terrane_f3_s3964, 1000000000.clone()),
                terrane_platform_time_mod(&elapsed_terrane_f3_s3964, 1000000000.clone()),
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
        let period_total_terrane_f3_s5216: terrane_int_support::Int;
        let scheduled_nanoseconds_terrane_f3_s5269: terrane_int_support::Int;
        let observed_terrane_f3_s5441: MonotonicInstant;
        let elapsed_terrane_f3_s5478: terrane_int_support::Int;
        let mut observed_index_terrane_f3_s5559: terrane_int_support::Int;
        let count_terrane_f3_s5710: terrane_int_support::Int;
        let delivered_terrane_f3_s5763: MonotonicInstant;
        period_total_terrane_f3_s5216 = self.period.total_nanoseconds.clone();
        scheduled_nanoseconds_terrane_f3_s5269 = self.anchor.elapsed_nanoseconds.clone()
            + period_total_terrane_f3_s5216.clone() * self.next_index.clone();
        discard_none(
            __terrane_await(
                    terrane_platform_time_sleep_until(
                        scheduled_nanoseconds_terrane_f3_s5269,
                    ),
                )
                .await,
        );
        observed_terrane_f3_s5441 = Clock::terrane_static_monotonic();
        elapsed_terrane_f3_s5478 = observed_terrane_f3_s5441.elapsed_nanoseconds.clone()
            - self.anchor.elapsed_nanoseconds.clone();
        observed_index_terrane_f3_s5559 = terrane_platform_time_div(
            &elapsed_terrane_f3_s5478,
            period_total_terrane_f3_s5216.clone(),
        );
        if observed_index_terrane_f3_s5559.clone() < self.next_index.clone() {
            observed_index_terrane_f3_s5559 = self.next_index.clone();
        }
        count_terrane_f3_s5710 = observed_index_terrane_f3_s5559.clone()
            - self.next_index.clone() + terrane_int_support::Int::from(1_i128);
        delivered_terrane_f3_s5763 = MonotonicInstant::terrane_construct(
            self.anchor.domain.clone(),
            self.anchor.elapsed_nanoseconds.clone()
                + period_total_terrane_f3_s5216.clone()
                    * observed_index_terrane_f3_s5559.clone(),
        );
        self.next_index = observed_index_terrane_f3_s5559.clone()
            + terrane_int_support::Int::from(1_i128);
        return Tick::terrane_construct(
            delivered_terrane_f3_s5763,
            observed_terrane_f3_s5441,
            count_terrane_f3_s5710.clone(),
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
        let raw_terrane_f3_s6105: TerranePlatformResult;
        raw_terrane_f3_s6105 = terrane_platform_time_wall();
        return Instant::terrane_construct(
            terrane_platform_time_wall_seconds(&raw_terrane_f3_s6105),
            terrane_platform_time_wall_nanoseconds(&raw_terrane_f3_s6105),
        );
    }
    pub fn terrane_static_monotonic() -> MonotonicInstant {
        return MonotonicInstant::terrane_construct(
            terrane_platform_time_domain(),
            terrane_platform_time_monotonic(),
        );
    }
    pub async fn terrane_static_sleep(elapsed: Duration) {
        let target_terrane_f3_s6426: terrane_int_support::Int;
        target_terrane_f3_s6426 = terrane_platform_time_monotonic()
            + elapsed.total_nanoseconds.clone();
        return __terrane_await(
                terrane_platform_time_sleep_until(target_terrane_f3_s6426),
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
                    TerraneErrorKind::Custom(DescriptorId(0)),
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
        let now_terrane_f3_s6860: MonotonicInstant;
        let target_terrane_f3_s6892: MonotonicInstant;
        now_terrane_f3_s6860 = Clock::terrane_static_monotonic();
        target_terrane_f3_s6892 = MonotonicInstant::terrane_construct(
            now_terrane_f3_s6860.domain.clone(),
            now_terrane_f3_s6860.elapsed_nanoseconds.clone()
                + elapsed.total_nanoseconds.clone(),
        );
        return Deadline::terrane_construct(target_terrane_f3_s6892);
    }
    pub fn terrane_static_interval(period: Duration) -> Result<Ticker, TerraneError> {
        if period.total_nanoseconds.clone() == terrane_int_support::Int::from(0_i128) {
            return Err({
                let value = InvalidDuration::terrane_construct();
                TerraneError::raised_with_message(
                    TerraneErrorKind::Custom(DescriptorId(0)),
                    value.render(),
                    15 /* terrane-site: core/time.trn:172:13-172:45 */,
                )
            });
        }
        return Ok(Ticker::terrane_construct(period));
    }
}
