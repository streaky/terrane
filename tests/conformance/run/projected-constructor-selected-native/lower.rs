// Generated deterministically by Terrane <version>.
// Runtime support:
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
#[allow(dead_code, reason = "a projected dependency may expose no Result members")]
const TERRANE_DEPENDENCY_ERROR: DescriptorId = DescriptorId(0);
#[allow(dead_code, reason = "panic catching may be disabled or not crossed")]
const TERRANE_DEPENDENCY_PANIC: DescriptorId = DescriptorId(1);
#[allow(
    dead_code,
    reason = "projected type methods may be imported without being crossed"
)]
fn __terrane_dependency_panic(
    payload: std::boxed::Box<dyn std::any::Any + Send>,
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
    pub static DESCRIPTORS: [&str; 2] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
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
    pub static FILES: [&str; 1] = ["src/main.trn"];
    pub static FUNCTIONS: [&str; 1] = ["/app::main"];
    pub static SITES: [Site; 22] = [
        /* terrane-site-row: site 0: /app::main (src/main.trn:9:25-9:61) */
        { Site { function: 0, file: 0, line: 9, column: 25, end_line: 9, end_column: 61 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:10:13-10:35) */
        { Site { function: 0, file: 0, line: 10, column: 13, end_line: 10, end_column: 35 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:13:13-13:35) */
        { Site { function: 0, file: 0, line: 13, column: 13, end_line: 13, end_column: 35 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:14:12-14:48) */
        { Site { function: 0, file: 0, line: 14, column: 12, end_line: 14, end_column: 48 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:15:13-15:30) */
        { Site { function: 0, file: 0, line: 15, column: 13, end_line: 15, end_column: 30 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:16:13-16:36) */
        { Site { function: 0, file: 0, line: 16, column: 13, end_line: 16, end_column: 36 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:17:13-17:30) */
        { Site { function: 0, file: 0, line: 17, column: 13, end_line: 17, end_column: 30 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:18:16-18:43) */
        { Site { function: 0, file: 0, line: 18, column: 16, end_line: 18, end_column: 43 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:20:12-20:63) */
        { Site { function: 0, file: 0, line: 20, column: 12, end_line: 20, end_column: 63 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:21:13-21:31) */
        { Site { function: 0, file: 0, line: 21, column: 13, end_line: 21, end_column: 31 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:22:13-22:73) */
        { Site { function: 0, file: 0, line: 22, column: 13, end_line: 22, end_column: 73 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:23:13-23:30) */
        { Site { function: 0, file: 0, line: 23, column: 13, end_line: 23, end_column: 30 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:24:17-24:56) */
        { Site { function: 0, file: 0, line: 24, column: 17, end_line: 24, end_column: 56 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:25:13-25:38) */
        { Site { function: 0, file: 0, line: 25, column: 13, end_line: 25, end_column: 38 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:26:11-26:29) */
        { Site { function: 0, file: 0, line: 26, column: 11, end_line: 26, end_column: 29 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:29:15-29:48) */
        { Site { function: 0, file: 0, line: 29, column: 15, end_line: 29, end_column: 48 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:31:5-31:42) */
        { Site { function: 0, file: 0, line: 31, column: 5, end_line: 31, end_column: 42 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:33:12-33:45) */
        { Site { function: 0, file: 0, line: 33, column: 12, end_line: 33, end_column: 45 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:36:9-36:46) */
        { Site { function: 0, file: 0, line: 36, column: 9, end_line: 36, end_column: 46 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:40:36-40:72) */
        { Site { function: 0, file: 0, line: 40, column: 36, end_line: 40, end_column: 72 } },
        /* terrane-site-row: site 20: /app::main (src/main.trn:41:13-41:35) */
        { Site { function: 0, file: 0, line: 41, column: 13, end_line: 41, end_column: 35 } },
        /* terrane-site-row: site 21: /app::main (src/main.trn:42:16-42:52) */
        { Site { function: 0, file: 0, line: 42, column: 16, end_line: 42, end_column: 52 } },
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
fn main() {
    let address: Address = witness::Address { port: 8080 };
    let mut envelope: Envelope<witness::Address> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<witness::Address> {
                payload: address.clone(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::<witness::Address>::terrane_construct",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:9:25-9:61 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(address_port(&envelope),
        1 /* terrane-site: src/main.trn:10:13-10:35 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(i128::from(*
        &envelope.payload.port)))
    );
    envelope.payload = witness::Address { port: 8081 };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(address_port(&envelope),
        2 /* terrane-site: src/main.trn:13:13-13:35 */))
    );
    let text: Envelope<String> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<String> {
                payload: String::from("hello"),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::<String>::terrane_construct",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:14:12-14:48 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(text_length(&text),
        4 /* terrane-site: src/main.trn:15:13-15:30 */))
    );
    let frame: Frame<witness::Address> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Frame::<witness::Address> {
                0: address.clone(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Frame::<witness::Address>::terrane_construct",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:16:13-16:36 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(frame_port(&frame),
        6 /* terrane-site: src/main.trn:17:13-17:30 */))
    );
    let restored: Address = __terrane_raised(
        into_address(envelope),
        7 /* terrane-site: src/main.trn:18:16-18:43 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(i128::from(*
        &restored.port)))
    );
    let pair: Pair<String, witness::Address> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Pair::<String, witness::Address> {
                first: String::from("selected"),
                second: address.clone(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Pair::<String, witness::Address>::terrane_construct",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:20:12-20:63 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(pair_summary(&pair),
        9 /* terrane-site: src/main.trn:21:13-21:31 */))
    );
    let maybe: Maybe<witness::Address> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Maybe::<witness::Address> {
                label: String::from("optional"),
                payload: Some(address.clone()),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Maybe::<witness::Address>::terrane_construct",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:22:13-22:73 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(maybe_port(&maybe),
        11 /* terrane-site: src/main.trn:23:13-23:30 */))
    );
    let printable: Printable<String> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Printable::<String> {
                payload: String::from("display"),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Printable::<String>::terrane_construct",
                    ),
                )
            }
        },
        12 /* terrane-site: src/main.trn:24:17-24:56 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(printable_text(&printable),
        13 /* terrane-site: src/main.trn:25:13-25:38 */))
    );
    let raw: String = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| text.into_payload()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Envelope<T>::into_payload",
                    ),
                )
            }
        },
        14 /* terrane-site: src/main.trn:26:11-26:29 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&raw));
    let seed: i64 = 41;
    let mut numeric: Envelope<i64> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<i64> {
                payload: match || -> Result<_, crate::TerraneForeignError> {
                    Ok(
                        terrane_int_support::coerce::<
                            i64,
                        >(&terrane_int_support::Int::from(seed as i128))
                            .map_err(|error| crate::TerraneForeignError(
                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                            ))?,
                    )
                }() {
                    Ok(value) => value,
                    Err(error) => std::panic::panic_any(error),
                },
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::<i64>::terrane_construct",
                    ),
                )
            }
        },
        15 /* terrane-site: src/main.trn:29:15-29:48 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(*
        &numeric.payload)) + terrane_int_support::Int::from(1_i128)))
    );
    numeric.payload = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            let __terrane_field_value = terrane_int_support::Int::from(
                i128::from(*&numeric.payload),
            ) + terrane_int_support::Int::from(2_i128);
            Ok(
                terrane_int_support::coerce::<i64>(&__terrane_field_value)
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }(),
        16 /* terrane-site: src/main.trn:31:5-31:42 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&(terrane_int_support::Int::from(i128::from(*
        &numeric.payload)) + terrane_int_support::Int::from(1_i128)))
    );
    let flag: Envelope<bool> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<bool> {
                payload: true,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::<bool>::terrane_construct",
                    ),
                )
            }
        },
        17 /* terrane-site: src/main.trn:33:12-33:45 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&flag.payload));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            numeric.payload = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { let
                __terrane_field_value =
                terrane_int_support::Int::from(9223372036854775808_i128);
                Ok(terrane_int_support::coerce:: < i64 > (&__terrane_field_value)
                .map_err(| error | crate ::TerraneForeignError(crate
                ::TerraneRaised::raised(error, crate ::TERRANE_NO_SITE))) ?) } (),
                18 /* terrane-site: src/main.trn:36:9-36:46 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("native range rejected"))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
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
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(i128::from(*
        &numeric.payload)))
    );
    let explicit: Envelope<witness::Address> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Envelope::<witness::Address> {
                payload: address.clone(),
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Envelope::<witness::Address>::terrane_construct",
                    ),
                )
            }
        },
        19 /* terrane-site: src/main.trn:40:36-40:72 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(address_port(&explicit),
        20 /* terrane-site: src/main.trn:41:13-41:35 */))
    );
    let selected: Frame<witness::Address> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::Frame::<witness::Address> {
                0: address,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "Frame::<witness::Address>::terrane_construct",
                    ),
                )
            }
        },
        21 /* terrane-site: src/main.trn:42:16-42:52 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(i128::from(*
        &selected.0.port)))
    );
}
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub use witness::Address;
pub type Envelope<T> = witness::Envelope<T>;
pub type Frame<T> = witness::Frame<T>;
pub type Maybe<T> = witness::Maybe<T>;
pub type Pair<T, U> = witness::Pair<T, U>;
pub type Printable<T> = witness::Printable<T>;
pub fn address_port(
    envelope: &Envelope<witness::Address>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::address_port(envelope)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::address_port",
                ),
            )
        }
    }
}
pub fn frame_port(
    frame: &Frame<witness::Address>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::frame_port(frame)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::frame_port",
                ),
            )
        }
    }
}
pub fn into_address(
    envelope: Envelope<witness::Address>,
) -> Result<Address, crate::TerraneForeignError> {
    let envelope = envelope;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::into_address(envelope)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::into_address",
                ),
            )
        }
    }
}
pub fn maybe_port(
    value: &Maybe<witness::Address>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::maybe_port(value)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::maybe_port",
                ),
            )
        }
    }
}
pub fn pair_summary(
    pair: &Pair<String, witness::Address>,
) -> Result<String, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::pair_summary(pair)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::pair_summary",
                ),
            )
        }
    }
}
pub fn printable_text(
    value: &Printable<String>,
) -> Result<String, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::printable_text(value)),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::printable_text",
                ),
            )
        }
    }
}
pub fn text_length(
    envelope: &Envelope<String>,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::text_length(envelope)),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::text_length",
                ),
            )
        }
    }
}
