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
    pub static FUNCTIONS: [&str; 2] = ["/app::contextual", "/app::main"];
    pub static SITES: [Site; 17] = [
        /* terrane-site-row: site 0: /app::contextual (src/main.trn:9:12-9:23) */
        { Site { function: 0, file: 0, line: 9, column: 12, end_line: 9, end_column: 23 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:12:17-12:21) */
        { Site { function: 1, file: 0, line: 12, column: 17, end_line: 12, end_column: 21 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:14:17-14:29) */
        { Site { function: 1, file: 0, line: 14, column: 17, end_line: 14, end_column: 29 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:16:18-16:37) */
        { Site { function: 1, file: 0, line: 16, column: 18, end_line: 16, end_column: 37 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:18:13-18:26) */
        { Site { function: 1, file: 0, line: 18, column: 13, end_line: 18, end_column: 26 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:19:21-19:36) */
        { Site { function: 1, file: 0, line: 19, column: 21, end_line: 19, end_column: 36 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:22:20-22:39) */
        { Site { function: 1, file: 0, line: 22, column: 20, end_line: 22, end_column: 39 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:25:19-25:44) */
        { Site { function: 1, file: 0, line: 25, column: 19, end_line: 25, end_column: 44 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:27:28-27:43) */
        { Site { function: 1, file: 0, line: 27, column: 28, end_line: 27, end_column: 43 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:30:28-30:33) */
        { Site { function: 1, file: 0, line: 30, column: 28, end_line: 30, end_column: 33 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:30:20-30:34) */
        { Site { function: 1, file: 0, line: 30, column: 20, end_line: 30, end_column: 34 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:32:13-32:25) */
        { Site { function: 1, file: 0, line: 32, column: 13, end_line: 32, end_column: 25 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:33:36-33:41) */
        { Site { function: 1, file: 0, line: 33, column: 36, end_line: 33, end_column: 41 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:33:18-33:46) */
        { Site { function: 1, file: 0, line: 33, column: 18, end_line: 33, end_column: 46 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:35:13-35:25) */
        { Site { function: 1, file: 0, line: 35, column: 13, end_line: 35, end_column: 25 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:37:20-37:33) */
        { Site { function: 1, file: 0, line: 37, column: 20, end_line: 37, end_column: 33 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:39:32-39:39) */
        { Site { function: 1, file: 0, line: 39, column: 32, end_line: 39, end_column: 39 } },
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
fn contextual() -> terrane_int_support::Int {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::sum!(20_i64, 22_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(payload, "witness", "witness::sum"),
                )
            }
        },
        0 /* terrane-site: src/main.trn:9:12-9:23 */,
    );
}
fn main() {
    let empty_terrane_f0_s324: terrane_int_support::Int;
    let total_terrane_f0_s362: terrane_int_support::Int;
    let nested_terrane_f0_s408: terrane_int_support::Int;
    let mut record_terrane_f0_s490: Packet;
    let observed_terrane_f0_s576: terrane_int_support::Int;
    let text_terrane_f0_s661: String;
    let sequence_terrane_f0_s721: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut value_terrane_f0_s768: terrane_int_support::Int;
    let repeated_terrane_f0_s811: terrane_int_support::Int;
    let chosen_terrane_f0_s891: terrane_int_support::Int;
    let combined_terrane_f0_s1006: terrane_int_support::Int;
    let empty_values_terrane_f0_s1059: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    empty_terrane_f0_s324 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::sum!(),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(payload, "witness", "witness::sum"),
                )
            }
        },
        1 /* terrane-site: src/main.trn:12:17-12:21 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&empty_terrane_f0_s324));
    total_terrane_f0_s362 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::sum!(1_i64, 2_i64, 3_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(payload, "witness", "witness::sum"),
                )
            }
        },
        2 /* terrane-site: src/main.trn:14:17-14:29 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&total_terrane_f0_s362));
    nested_terrane_f0_s408 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::sum!(witness::sum!(2_i64, 3_i64), 4_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(payload, "witness", "witness::sum"),
                )
            }
        },
        3 /* terrane-site: src/main.trn:16:18-16:37 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&nested_terrane_f0_s408));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(sum_terrane_deps_witness(),
        4 /* terrane-site: src/main.trn:18:13-18:26 */))
    );
    record_terrane_f0_s490 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: witness::Packet = core::convert::Into::into(
                    witness::packet!(7_i64, true),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::packet",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:19:21-19:36 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(i128::from(*
        &record_terrane_f0_s490.value)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&record_terrane_f0_s490.enabled));
    observed_terrane_f0_s576 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::disable!(&mut record_terrane_f0_s490),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::disable",
                    ),
                )
            }
        },
        6 /* terrane-site: src/main.trn:22:20-22:39 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&observed_terrane_f0_s576));
    println!("{}", terrane_scalar_support::scalar_text(&record_terrane_f0_s490.enabled));
    text_terrane_f0_s661 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: String = core::convert::Into::into(
                    witness::message!("{}:{}", 7_i64, true),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::message",
                    ),
                )
            }
        },
        7 /* terrane-site: src/main.trn:25:19-25:44 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&text_terrane_f0_s661));
    sequence_terrane_f0_s721 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: std::vec::Vec<i64> = core::convert::Into::into(
                    witness::values!(3_i64, 5_i64, 8_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::List::new(
                        value
                            .into_iter()
                            .map(|item| terrane_int_support::Int::from(i128::from(item)))
                            .collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::values",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:27:28-27:43 */,
    );
    let __terrane_iterable_0 = sequence_terrane_f0_s721;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        value_terrane_f0_s768 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s768));
    }
    repeated_terrane_f0_s811 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::twice!(
                        match | | -> Result < _, crate ::TerraneForeignError > {
                        Ok(terrane_int_support::coerce:: < i64 >
                        (&__terrane_raised(next(), 9 /* terrane-site: src/main.trn:30:28-30:33 */)).map_err(| error | crate
                        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
                        std::panic::panic_any(error) }
                    ),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::twice",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:30:20-30:34 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&repeated_terrane_f0_s811));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(evaluations(),
        11 /* terrane-site: src/main.trn:32:13-32:25 */))
    );
    chosen_terrane_f0_s891 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::selected!(
                        false, match | | -> Result < _, crate ::TerraneForeignError > {
                        Ok(terrane_int_support::coerce:: < i64 >
                        (&__terrane_raised(next(), 12 /* terrane-site: src/main.trn:33:36-33:41 */)).map_err(| error | crate
                        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                        ::TERRANE_NO_SITE))) ?) } () { Ok(value) => value, Err(error) =>
                        std::panic::panic_any(error) }, 17_i64
                    ),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::selected",
                    ),
                )
            }
        },
        13 /* terrane-site: src/main.trn:33:18-33:46 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&chosen_terrane_f0_s891));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(evaluations(),
        14 /* terrane-site: src/main.trn:35:13-35:25 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&contextual()));
    combined_terrane_f0_s1006 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: i64 = core::convert::Into::into(
                    witness::layout::combine!(4_i64, 6_i64),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::layout::combine",
                    ),
                )
            }
        },
        15 /* terrane-site: src/main.trn:37:20-37:33 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&combined_terrane_f0_s1006));
    empty_values_terrane_f0_s1059 = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let __terrane_macro_result: std::vec::Vec<i64> = core::convert::Into::into(
                    witness::values!(),
                );
                __terrane_macro_result
            }),
        ) {
            Ok(value) => {
                Ok(
                    terrane_collection_support::List::new(
                        value
                            .into_iter()
                            .map(|item| terrane_int_support::Int::from(i128::from(item)))
                            .collect(),
                    ),
                )
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::values",
                    ),
                )
            }
        },
        16 /* terrane-site: src/main.trn:39:32-39:39 */,
    );
    let __terrane_iterable_1 = empty_values_terrane_f0_s1059;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        value_terrane_f0_s768 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &value_terrane_f0_s768;
        println!("{}", terrane_scalar_support::scalar_text(&value_terrane_f0_s768));
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&String::from("empty collection selected"))
    );
}
// Source: <terrane>/projected/deps/witness.trn
// Namespace: deps/witness
pub use witness::Packet;
pub fn evaluations() -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| witness::evaluations()),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "witness",
                    "witness::evaluations",
                ),
            )
        }
    }
}
pub fn next() -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| witness::next())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::next"))
        }
    }
}
pub fn sum_terrane_deps_witness() -> Result<
    terrane_int_support::Int,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| witness::sum())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(crate::__terrane_dependency_panic(payload, "witness", "witness::sum"))
        }
    }
}
// Source: <terrane>/projected/deps/witness/layout/macros.trn
// Namespace: deps/witness/layout/macros
// Source: <terrane>/projected/deps/witness/macros.trn
// Namespace: deps/witness/macros
