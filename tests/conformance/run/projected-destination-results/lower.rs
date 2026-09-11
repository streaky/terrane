// Generated deterministically by Terrane <version>.
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
    pub static DESCRIPTORS: [&str; 2] = ["dependency-error", "dependency-panic"];
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
    pub static FUNCTIONS: [&str; 4] = [
        "/app::load",
        "/app::decoded-value",
        "/app::make-number",
        "/app::main",
    ];
    pub static SITES: [Site; 15] = [
        /* terrane-site-row: site 0: /app::load (src/main.trn:9:14-9:18) */
        { Site { function: 0, file: 0, line: 9, column: 14, end_line: 9, end_column: 18 } },
        /* terrane-site-row: site 1: /app::load (src/main.trn:10:18-10:33) */
        { Site { function: 0, file: 0, line: 10, column: 18, end_line: 10, end_column: 33 } },
        /* terrane-site-row: site 2: /app::decoded-value (src/main.trn:13:12-13:16) */
        { Site { function: 1, file: 0, line: 13, column: 12, end_line: 13, end_column: 16 } },
        /* terrane-site-row: site 3: /app::decoded-value (src/main.trn:14:10-14:25) */
        { Site { function: 1, file: 0, line: 14, column: 10, end_line: 14, end_column: 25 } },
        /* terrane-site-row: site 4: /app::make-number (src/main.trn:17:10-17:24) */
        { Site { function: 2, file: 0, line: 17, column: 10, end_line: 17, end_column: 24 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:24:25-24:38) */
        { Site { function: 3, file: 0, line: 24, column: 25, end_line: 24, end_column: 38 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:25:16-25:29) */
        { Site { function: 3, file: 0, line: 25, column: 16, end_line: 25, end_column: 29 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:26:26-26:39) */
        { Site { function: 3, file: 0, line: 26, column: 26, end_line: 26, end_column: 39 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:27:32-27:45) */
        { Site { function: 3, file: 0, line: 27, column: 32, end_line: 27, end_column: 45 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:28:24-28:37) */
        { Site { function: 3, file: 0, line: 28, column: 24, end_line: 28, end_column: 37 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:29:31-29:43) */
        { Site { function: 3, file: 0, line: 29, column: 31, end_line: 29, end_column: 43 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:30:38-30:58) */
        { Site { function: 3, file: 0, line: 30, column: 38, end_line: 30, end_column: 58 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:31:20-31:40) */
        { Site { function: 3, file: 0, line: 31, column: 20, end_line: 31, end_column: 40 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:32:12-32:16) */
        { Site { function: 3, file: 0, line: 32, column: 12, end_line: 32, end_column: 16 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:33:24-33:39) */
        { Site { function: 3, file: 0, line: 33, column: 24, end_line: 33, end_column: 39 } },
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
#[derive(Clone)]
pub struct Record {
    pub value: String,
}
impl Record {
    pub fn terrane_construct() -> Self {
        Self { value: String::from("") }
    }
    pub fn load(&mut self) {
        let source: Row = __terrane_raised(
            row(),
            0 /* terrane-site: src/main.trn:9:14-9:18 */,
        );
        self.value = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| source.decoded::<String>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_generic_row_witness",
                            "terrane_generic_row_witness::Row::decoded",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:10:18-10:33 */,
        );
    }
}
fn decoded_value() -> String {
    let source: Row = __terrane_raised(
        row(),
        2 /* terrane-site: src/main.trn:13:12-13:16 */,
    );
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| source.decoded::<String>()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_generic_row_witness",
                        "terrane_generic_row_witness::Row::decoded",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:14:10-14:25 */,
    );
}
fn make_number() -> i64 {
    return __terrane_raised(
        default_value::<i64>(),
        4 /* terrane-site: src/main.trn:17:10-17:24 */,
    );
}
fn accept_by_argument(value: String) {
    println!("{}", terrane_scalar_support::scalar_text(&value));
}
fn main() {
    let number: i64 = make_number();
    let optional: Option<i64> = __terrane_raised(
        sample_value::<Option<i64>>(),
        5 /* terrane-site: src/main.trn:24:25-24:38 */,
    );
    let data: Vec<u8> = __terrane_raised(
        sample_value::<Vec<u8>>(),
        6 /* terrane-site: src/main.trn:25:16-25:29 */,
    );
    let values: terrane_collection_support::List<i64> = terrane_collection_support::List::new(
        __terrane_raised(
            sample_value::<Vec<i64>>(),
            7 /* terrane-site: src/main.trn:26:26-26:39 */,
        ),
    );
    let names: terrane_collection_support::Map<String, i64> = terrane_collection_support::Map::new(
        __terrane_raised(
                sample_value::<std::collections::BTreeMap<String, i64>>(),
                8 /* terrane-site: src/main.trn:27:32-27:45 */,
            )
            .into_iter()
            .map(|(key, item)| terrane_collection_support::Entry::new(key, item))
            .collect(),
    );
    let tags: terrane_collection_support::Set<String> = terrane_collection_support::Set::new(
        __terrane_raised(
                sample_value::<std::collections::BTreeSet<String>>(),
                9 /* terrane-site: src/main.trn:28:24-28:37 */,
            )
            .into_iter()
            .map(|item| item)
            .collect(),
    );
    let rows: terrane_collection_support::Map<String, i64> = terrane_collection_support::Map::new(
        __terrane_raised(
                sample_rows::<i64>(),
                10 /* terrane-site: src/main.trn:29:31-29:43 */,
            )
            .into_iter()
            .map(|(key, item)| terrane_collection_support::Entry::new(key, item))
            .collect(),
    );
    let numeric_rows: terrane_collection_support::Map<i64, i64> = terrane_collection_support::Map::new(
        __terrane_raised(
                sample_numeric_rows::<i64>(),
                11 /* terrane-site: src/main.trn:30:38-30:58 */,
            )
            .into_iter()
            .map(|(key, item)| terrane_collection_support::Entry::new(key, item))
            .collect(),
    );
    let renamed: String = __terrane_raised(
        renamed_bound_value::<String>(),
        12 /* terrane-site: src/main.trn:31:20-31:40 */,
    );
    let source: Row = __terrane_raised(
        row(),
        13 /* terrane-site: src/main.trn:32:12-32:16 */,
    );
    accept_by_argument(
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| source.decoded::<String>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_generic_row_witness",
                            "terrane_generic_row_witness::Row::decoded",
                        ),
                    )
                }
            },
            14 /* terrane-site: src/main.trn:33:24-33:39 */,
        ),
    );
    let returned: String = decoded_value();
    println!("{}", terrane_scalar_support::scalar_text(&returned));
    let mut item: Record = Record::terrane_construct();
    item.load();
    println!("{}", terrane_scalar_support::scalar_text(&item.value));
    println!(
        "{}{}{}{}{}{}{}{}{}", terrane_scalar_support::scalar_text(&number),
        terrane_scalar_support::scalar_text(&optional.is_some()),
        terrane_scalar_support::scalar_text(&(data.len() as i128)),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(names
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(tags
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rows
        .length())),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(numeric_rows
        .length())), terrane_scalar_support::scalar_text(&renamed)
    );
}
// Source: <terrane>/projected/deps/factory.trn
// Namespace: deps/factory
pub fn default_value<T: core::default::Default>() -> Result<
    T,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::default_value::<T>()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "factory",
                    "factory::default_value",
                ),
            )
        }
    }
}
pub fn renamed_bound_value<T: for<'value> factory::Decode<'value>>() -> Result<
    T,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::renamed_bound_value::<T>()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "factory",
                    "factory::renamed_bound_value",
                ),
            )
        }
    }
}
pub fn sample_numeric_rows<T: factory::Sample>() -> Result<
    std::collections::BTreeMap<i64, T>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::sample_numeric_rows::<T>()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "factory",
                    "factory::sample_numeric_rows",
                ),
            )
        }
    }
}
pub fn sample_rows<T: factory::Sample>() -> Result<
    std::collections::BTreeMap<String, T>,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::sample_rows::<T>()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "factory",
                    "factory::sample_rows",
                ),
            )
        }
    }
}
pub fn sample_value<T: factory::Sample>() -> Result<T, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| factory::sample_value::<T>()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "factory",
                    "factory::sample_value",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/terrane-generic-row-witness.trn
// Namespace: deps/terrane-generic-row-witness
pub use terrane_generic_row_witness::Row;
pub fn row() -> Result<Row, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_generic_row_witness::row()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-generic-row-witness",
                    "terrane_generic_row_witness::row",
                ),
            )
        }
    }
}
