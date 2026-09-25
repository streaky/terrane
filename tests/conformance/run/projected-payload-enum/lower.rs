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
    cause: Option<Box<TerraneError>>,
    frames: Vec<TerraneSite>,
    structured: Vec<String>,
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
                Box::new(TerraneErrorDetail {
                    message: None,
                    cause: None,
                    frames: Vec::new(),
                    structured: Vec::new(),
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
                Box::new(TerraneErrorDetail {
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
    payload: Box<dyn std::any::Any + Send>,
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
    pub static SITES: [Site; 20] = [
        /* terrane-site-row: site 0: /app::main (src/main.trn:7:10-7:30) */
        { Site { function: 0, file: 0, line: 7, column: 10, end_line: 7, end_column: 30 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:8:11-8:29) */
        { Site { function: 0, file: 0, line: 8, column: 11, end_line: 8, end_column: 29 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:9:28-9:43) */
        { Site { function: 0, file: 0, line: 9, column: 28, end_line: 9, end_column: 43 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:12:12-12:32) */
        { Site { function: 0, file: 0, line: 12, column: 12, end_line: 12, end_column: 32 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:13:11-13:31) */
        { Site { function: 0, file: 0, line: 13, column: 11, end_line: 13, end_column: 31 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:14:29-14:48) */
        { Site { function: 0, file: 0, line: 14, column: 29, end_line: 14, end_column: 48 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:17:10-17:22) */
        { Site { function: 0, file: 0, line: 17, column: 10, end_line: 17, end_column: 22 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:18:11-18:29) */
        { Site { function: 0, file: 0, line: 18, column: 11, end_line: 18, end_column: 29 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:21:16-21:41) */
        { Site { function: 0, file: 0, line: 21, column: 16, end_line: 21, end_column: 41 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:22:32-22:47) */
        { Site { function: 0, file: 0, line: 22, column: 32, end_line: 22, end_column: 47 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:27:17-27:44) */
        { Site { function: 0, file: 0, line: 27, column: 17, end_line: 27, end_column: 44 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:28:34-28:51) */
        { Site { function: 0, file: 0, line: 28, column: 34, end_line: 28, end_column: 51 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:32:20-32:53) */
        { Site { function: 0, file: 0, line: 32, column: 20, end_line: 32, end_column: 53 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:33:40-33:63) */
        { Site { function: 0, file: 0, line: 33, column: 40, end_line: 33, end_column: 63 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:36:49-36:75) */
        { Site { function: 0, file: 0, line: 36, column: 49, end_line: 36, end_column: 75 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:36:27-36:76) */
        { Site { function: 0, file: 0, line: 36, column: 27, end_line: 36, end_column: 76 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:38:36-38:62) */
        { Site { function: 0, file: 0, line: 38, column: 36, end_line: 38, end_column: 62 } },
        /* terrane-site-row: site 17: /app::main (src/main.trn:40:15-40:44) */
        { Site { function: 0, file: 0, line: 40, column: 15, end_line: 40, end_column: 44 } },
        /* terrane-site-row: site 18: /app::main (src/main.trn:41:22-41:35) */
        { Site { function: 0, file: 0, line: 41, column: 22, end_line: 41, end_column: 35 } },
        /* terrane-site-row: site 19: /app::main (src/main.trn:42:11-42:31) */
        { Site { function: 0, file: 0, line: 42, column: 11, end_line: 42, end_column: 31 } },
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
    let text: Event = __terrane_raised(
        match std::panic::catch_unwind(|| terrane_payload_enum_witness::Event::Text(
            String::from("hello"),
        )) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Text",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:7:10-7:30 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &text {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() }))
        { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }, 1 /* terrane-site: src/main.trn:8:11-8:29 */))
    );
    let text_value: Option<String> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match text {
                terrane_payload_enum_witness::Event::Text(value) => Some(value),
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Text",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:9:28-9:43 */,
    );
    if text_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&text_value.as_ref()
            .expect("semantic optional narrowing").clone())
        );
    }
    let binary: Event = __terrane_raised(
        match std::panic::catch_unwind(|| terrane_payload_enum_witness::Event::Binary(
            Vec::from([65, 66]),
        )) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Binary",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:12:12-12:32 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &binary {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() }))
        { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }, 4 /* terrane-site: src/main.trn:13:11-13:31 */))
    );
    let binary_value: Option<Vec<u8>> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match binary {
                terrane_payload_enum_witness::Event::Binary(value) => Some(value),
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Binary",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:14:29-14:48 */,
    );
    if binary_value.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(binary_value.as_ref()
            .expect("semantic optional narrowing").clone().len() as i128))
        );
    }
    let ping: Event = __terrane_raised(
        match std::panic::catch_unwind(|| terrane_payload_enum_witness::Event::Ping) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Ping",
                    ),
                )
            }
        },
        6 /* terrane-site: src/main.trn:17:10-17:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &ping {
        terrane_payload_enum_witness::Event::Text { .. } => "Text".to_owned(),
        terrane_payload_enum_witness::Event::Binary { .. } => "Binary".to_owned(),
        terrane_payload_enum_witness::Event::Ping { .. } => "Ping".to_owned(),
        terrane_payload_enum_witness::Event::Pair { .. } => "Pair".to_owned(),
        terrane_payload_enum_witness::Event::Named { .. } => "Named".to_owned(),
        terrane_payload_enum_witness::Event::Optional { .. } => "Optional".to_owned() }))
        { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::Event::variant-name")) }, 7 /* terrane-site: src/main.trn:18:11-18:29 */))
    );
    let pair_payload: EventPair = EventPair::terrane_construct(20, 22);
    println!(
        "{}", terrane_scalar_support::scalar_text(&(pair_payload.item_n0.clone() +
        pair_payload.item_n1.clone()))
    );
    let pair: Event = __terrane_raised(
        match std::panic::catch_unwind(|| {
            let (field_0, field_1) = pair_payload.terrane_into_fields();
            terrane_payload_enum_witness::Event::Pair(field_0, field_1)
        }) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Pair",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:21:16-21:41 */,
    );
    let pair_value: Option<EventPair> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match pair {
                terrane_payload_enum_witness::Event::Pair(field_0, field_1) => {
                    Some(
                        __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a50616972237061796c6f6164(
                            field_0,
                            field_1,
                        ),
                    )
                }
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Pair",
                    ),
                )
            }
        },
        9 /* terrane-site: src/main.trn:22:32-22:47 */,
    );
    if pair_value.is_some() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("pair")));
    }
    let named_payload: EventNamed = EventNamed::terrane_construct(42);
    println!("{}", terrane_scalar_support::scalar_text(&named_payload.value));
    let named: Event = __terrane_raised(
        match std::panic::catch_unwind(|| {
            let (field_0,) = named_payload.terrane_into_fields();
            terrane_payload_enum_witness::Event::Named {
                r#value: field_0,
            }
        }) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Named",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:27:17-27:44 */,
    );
    let named_value: Option<EventNamed> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match named {
                terrane_payload_enum_witness::Event::Named { r#value: field_0 } => {
                    Some(
                        __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a4e616d6564237061796c6f6164(
                            field_0,
                        ),
                    )
                }
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Named",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:28:34-28:51 */,
    );
    if named_value.is_some() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("named")));
    }
    let optional_payload: EventOptional = EventOptional::terrane_construct(
        42,
        None::<String>,
    );
    let optional: Event = __terrane_raised(
        match std::panic::catch_unwind(|| {
            let (field_0, field_1) = optional_payload.terrane_into_fields();
            terrane_payload_enum_witness::Event::Optional {
                r#value: field_0,
                r#label: field_1,
            }
        }) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::Optional",
                    ),
                )
            }
        },
        12 /* terrane-site: src/main.trn:32:20-32:53 */,
    );
    let optional_value: Option<EventOptional> = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| match optional {
                terrane_payload_enum_witness::Event::Optional {
                    r#value: field_0,
                    r#label: field_1,
                } => {
                    Some(
                        __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a4f7074696f6e616c237061796c6f6164(
                            field_0,
                            field_1,
                        ),
                    )
                }
                _ => None,
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_payload_enum_witness",
                        "terrane_payload_enum_witness::Event::into-Optional",
                    ),
                )
            }
        },
        13 /* terrane-site: src/main.trn:33:40-33:63 */,
    );
    if optional_value.is_some() {
        println!("{}", terrane_scalar_support::scalar_text(&String::from("optional")));
    }
    let owned: Option<OwnedEvent> = Some(
        __terrane_raised(
            match std::panic::catch_unwind(|| terrane_payload_enum_witness::OwnedEvent::Payload(
                __terrane_raised(
                    non_clone_payload(String::from("owned")),
                    14 /* terrane-site: src/main.trn:36:49-36:75 */,
                ),
            )) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_payload_enum_witness",
                            "terrane_payload_enum_witness::OwnedEvent::Payload",
                        ),
                    )
                }
            },
            15 /* terrane-site: src/main.trn:36:27-36:76 */,
        ),
    );
    if owned.is_some() {
        let payload: Option<NonClonePayload> = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| match owned
                    .expect("semantic optional narrowing")
                {
                    terrane_payload_enum_witness::OwnedEvent::Payload(value) => {
                        Some(value)
                    }
                    _ => None,
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_payload_enum_witness",
                            "terrane_payload_enum_witness::OwnedEvent::into-Payload",
                        ),
                    )
                }
            },
            16 /* terrane-site: src/main.trn:38:36-38:62 */,
        );
        if payload.is_some() {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_raised(consume_payload(payload
                .expect("semantic optional narrowing")), 17 /* terrane-site: src/main.trn:40:15-40:44 */))
            );
        }
    }
    let future: OpenEvent = __terrane_raised(
        future_event(),
        18 /* terrane-site: src/main.trn:41:22-41:35 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &future {
        terrane_payload_enum_witness::OpenEvent::Known { .. } => "Known".to_owned(), _ =>
        "unknown".to_owned() })) { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "terrane_payload_enum_witness",
        "terrane_payload_enum_witness::OpenEvent::variant-name")) },
        19 /* terrane-site: src/main.trn:42:11-42:31 */))
    );
}
// Source: <terrane>/projected/deps/terrane-payload-enum-witness.trn
// Namespace: deps/terrane-payload-enum-witness
pub use terrane_payload_enum_witness::Event;
pub struct EventNamed {
    value: i64,
}
impl EventNamed {
    pub fn terrane_construct(value: i64) -> Self {
        Self { value: value }
    }
    fn terrane_into_fields(self) -> (i64,) {
        (self.value,)
    }
}
fn __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a4e616d6564237061796c6f6164(
    field_0: i64,
) -> EventNamed {
    EventNamed::terrane_construct(field_0)
}
pub struct EventOptional {
    value: Option<u32>,
    label_value: Option<String>,
}
impl EventOptional {
    pub fn terrane_construct(
        value: impl Into<Option<u32>>,
        label_value: impl Into<Option<String>>,
    ) -> Self {
        Self {
            value: value.into(),
            label_value: label_value.into(),
        }
    }
    fn terrane_into_fields(self) -> (Option<u32>, Option<String>) {
        (self.value, self.label_value)
    }
}
fn __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a4f7074696f6e616c237061796c6f6164(
    field_0: Option<u32>,
    field_1: Option<String>,
) -> EventOptional {
    EventOptional::terrane_construct(field_0, field_1)
}
pub struct EventPair {
    item_n0: i64,
    item_n1: i64,
}
impl EventPair {
    pub fn terrane_construct(item_n0: i64, item_n1: i64) -> Self {
        Self {
            item_n0: item_n0,
            item_n1: item_n1,
        }
    }
    fn terrane_into_fields(self) -> (i64, i64) {
        (self.item_n0, self.item_n1)
    }
}
fn __terrane_enum_payload_74657272616e655f7061796c6f61645f656e756d5f7769746e6573733a3a4576656e743a3a50616972237061796c6f6164(
    field_0: i64,
    field_1: i64,
) -> EventPair {
    EventPair::terrane_construct(field_0, field_1)
}
pub use terrane_payload_enum_witness::NonClonePayload;
pub use terrane_payload_enum_witness::OpenEvent;
pub use terrane_payload_enum_witness::OwnedEvent;
pub fn consume_payload(
    value: NonClonePayload,
) -> Result<String, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_payload_enum_witness::consume_payload(
            value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-payload-enum-witness",
                    "terrane_payload_enum_witness::consume_payload",
                ),
            )
        }
    }
}
pub fn future_event() -> Result<OpenEvent, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_payload_enum_witness::future_event()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-payload-enum-witness",
                    "terrane_payload_enum_witness::future_event",
                ),
            )
        }
    }
}
pub fn non_clone_payload(
    value: String,
) -> Result<NonClonePayload, crate::TerraneForeignError> {
    let value = value;
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_payload_enum_witness::non_clone_payload(
            value,
        )),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-payload-enum-witness",
                    "terrane_payload_enum_witness::non_clone_payload",
                ),
            )
        }
    }
}
