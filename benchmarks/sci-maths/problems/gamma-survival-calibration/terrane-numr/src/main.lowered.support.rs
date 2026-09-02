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
    pub static FILES: [&str; 2] = ["src/main.trn", "standard/process.trn"];
    pub static FUNCTIONS: [&str; 5] = [
        "/benchmark-gamma-survival-calibration::benchmark-size",
        "/benchmark-gamma-survival-calibration::main",
        "/standard/process::arguments",
        "/standard/process::environment",
        "/standard/process::parse-command-line",
    ];
    pub static SITES: [Site; 14] = [
        {
            /* terrane-site-row: site 0: /benchmark-gamma-survival-calibration::benchmark-size (src/main.trn:10:18-10:29) */
            Site {
                function: 0,
                file: 0,
                line: 10,
                column: 18,
                end_line: 10,
                end_column: 29,
            }
        },
        {
            /* terrane-site-row: site 1: /benchmark-gamma-survival-calibration::benchmark-size (src/main.trn:10:18-10:44) */
            Site {
                function: 0,
                file: 0,
                line: 10,
                column: 18,
                end_line: 10,
                end_column: 44,
            }
        },
        {
            /* terrane-site-row: site 2: /benchmark-gamma-survival-calibration::benchmark-size (src/main.trn:10:17-10:45) */
            Site {
                function: 0,
                file: 0,
                line: 10,
                column: 17,
                end_line: 10,
                end_column: 45,
            }
        },
        {
            /* terrane-site-row: site 3: /benchmark-gamma-survival-calibration::main (src/main.trn:20:26-20:36) */
            Site {
                function: 1,
                file: 0,
                line: 20,
                column: 26,
                end_line: 20,
                end_column: 36,
            }
        },
        {
            /* terrane-site-row: site 4: /benchmark-gamma-survival-calibration::main (src/main.trn:21:32-21:43) */
            Site {
                function: 1,
                file: 0,
                line: 21,
                column: 32,
                end_line: 21,
                end_column: 43,
            }
        },
        {
            /* terrane-site-row: site 5: /benchmark-gamma-survival-calibration::main (src/main.trn:22:27-22:36) */
            Site {
                function: 1,
                file: 0,
                line: 22,
                column: 27,
                end_line: 22,
                end_column: 36,
            }
        },
        {
            /* terrane-site-row: site 6: /benchmark-gamma-survival-calibration::main (src/main.trn:26:25-26:61) */
            Site {
                function: 1,
                file: 0,
                line: 26,
                column: 25,
                end_line: 26,
                end_column: 61,
            }
        },
        {
            /* terrane-site-row: site 7: /benchmark-gamma-survival-calibration::main (src/main.trn:29:5-29:12) */
            Site {
                function: 1,
                file: 0,
                line: 29,
                column: 5,
                end_line: 29,
                end_column: 12,
            }
        },
        {
            /* terrane-site-row: site 8: /benchmark-gamma-survival-calibration::main (src/main.trn:30:21-30:26) */
            Site {
                function: 1,
                file: 0,
                line: 30,
                column: 21,
                end_line: 30,
                end_column: 26,
            }
        },
        {
            /* terrane-site-row: site 9: /standard/process::arguments (standard/process.trn:51:42-51:56) */
            Site {
                function: 2,
                file: 1,
                line: 51,
                column: 42,
                end_line: 51,
                end_column: 56,
            }
        },
        {
            /* terrane-site-row: site 10: /standard/process::environment (standard/process.trn:60:33-60:47) */
            Site {
                function: 3,
                file: 1,
                line: 60,
                column: 33,
                end_line: 60,
                end_column: 47,
            }
        },
        {
            /* terrane-site-row: site 11: /standard/process::environment (standard/process.trn:61:34-61:52) */
            Site {
                function: 3,
                file: 1,
                line: 61,
                column: 34,
                end_line: 61,
                end_column: 52,
            }
        },
        {
            /* terrane-site-row: site 12: /standard/process::parse-command-line (standard/process.trn:96:20-96:35) */
            Site {
                function: 4,
                file: 1,
                line: 96,
                column: 20,
                end_line: 96,
                end_column: 35,
            }
        },
        {
            /* terrane-site-row: site 13: /standard/process::parse-command-line (standard/process.trn:111:43-111:62) */
            Site {
                function: 4,
                file: 1,
                line: 111,
                column: 43,
                end_line: 111,
                end_column: 62,
            }
        },
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
type TerranePlatformResult = terrane_platform_support::ResultValue;
fn terrane_unhex(text: &str) -> Vec<u8> {
    fn digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    text.as_bytes()
        .chunks_exact(2)
        .filter_map(|pair| Some(digit(pair[0])? << 4 | digit(pair[1])?))
        .collect()
}
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    terrane_platform_support::platform_value(value)
}
fn terrane_platform_value_is_text(value: &str) -> bool {
    value.starts_with("text:")
}
fn terrane_platform_value_text(value: &str) -> String {
    value.strip_prefix("text:").unwrap_or("").to_owned()
}
fn terrane_platform_value_bytes(value: &str) -> Vec<u8> {
    value.strip_prefix("raw:").map(terrane_unhex).unwrap_or_default()
}
fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}
fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [
            terrane_platform_value(name),
            terrane_platform_value(value),
        ])
        .collect()
}
fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}
// Source: <terrane>/projected/deps/numr/algorithm/special.trn
// Namespace: deps/numr/algorithm/special
pub fn gammaincc_scalar(a: f64, x: f64) -> Result<f64, crate::TerraneForeignError> {
    let a = a;
    let x = x;
    match std::panic::catch_unwind(|| numr::algorithm::special::gammaincc_scalar(a, x)) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "numr",
                    "numr::algorithm::special::gammaincc_scalar",
                ),
            )
        }
    }
}
// Source: standard/process.trn
// Namespace: standard/process
#[derive(Clone)]
pub struct PlatformString {
    pub is_text: bool,
    pub text: String,
    pub raw: Vec<u8>,
}
impl PlatformString {
    pub fn terrane_construct(encoded: String) -> Self {
        let mut value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::from([]),
        };
        value.construct(encoded);
        value
    }
    pub fn construct(&mut self, encoded: String) {
        self.is_text = terrane_platform_value_is_text(&encoded);
        self.text = terrane_platform_value_text(&encoded);
        self.raw = terrane_platform_value_bytes(&encoded);
    }
}
#[derive(Clone)]
pub struct EnvironmentEntry {
    pub name: PlatformString,
    pub value: PlatformString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: PlatformString, entry_value: PlatformString) -> Self {
        let mut value = Self {
            name: PlatformString::terrane_construct(String::from("text:")),
            value: PlatformString::terrane_construct(String::from("text:")),
        };
        value.construct(name, entry_value);
        value
    }
    pub fn construct(&mut self, name: PlatformString, entry_value: PlatformString) {
        self.name = name.clone();
        self.value = entry_value.clone();
    }
}
#[derive(Clone)]
pub struct HostNameResult {
    pub failed: bool,
    pub available: bool,
    pub message: String,
    pub value: PlatformString,
}
impl HostNameResult {
    pub fn terrane_construct(
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: PlatformString,
    ) -> Self {
        let mut value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: PlatformString::terrane_construct(String::from("text:")),
        };
        value.construct(did_fail, is_available, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: PlatformString,
    ) {
        self.failed = did_fail;
        self.available = is_available;
        self.message = detail;
        self.value = result_value.clone();
    }
}
pub fn host_name() -> HostNameResult {
    let raw: TerranePlatformResult = terrane_platform_support::system_host_name();
    return HostNameResult::terrane_construct(
        raw.failed,
        raw.flag,
        raw.message.clone(),
        PlatformString::terrane_construct(raw.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<PlatformString> {
    let encoded: Vec<String> = terrane_process_arguments();
    let mut values: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
        values
            .append(
                PlatformString::terrane_construct(
                    __terrane_raised(
                        encoded
                            .get(
                                __terrane_raised(
                                    terrane_collection_support::index_from_int(&index.clone()),
                                    9 /* terrane-site: standard/process.trn:51:42-51:56 */,
                                ),
                            )
                            .cloned()
                            .ok_or(terrane_collection_support::IndexError {
                                index: __terrane_raised(
                                    terrane_collection_support::index_from_int(&index.clone()),
                                    9 /* terrane-site: standard/process.trn:51:42-51:56 */,
                                ),
                            }),
                        9 /* terrane-site: standard/process.trn:51:42-51:56 */,
                    ),
                ),
            );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return values.clone();
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String> = terrane_environment_entries();
    let mut values: terrane_collection_support::List<EnvironmentEntry> = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() + terrane_int_support::Int::from(1_i128)
        < terrane_int_support::Int::from(encoded.len() as i128)
    {
        let name: PlatformString = PlatformString::terrane_construct(
            __terrane_raised(
                encoded
                    .get(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            10 /* terrane-site: standard/process.trn:60:33-60:47 */,
                        ),
                    )
                    .cloned()
                    .ok_or(terrane_collection_support::IndexError {
                        index: __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            10 /* terrane-site: standard/process.trn:60:33-60:47 */,
                        ),
                    }),
                10 /* terrane-site: standard/process.trn:60:33-60:47 */,
            ),
        );
        let value: PlatformString = PlatformString::terrane_construct(
            __terrane_raised(
                encoded
                    .get(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &(index.clone() + terrane_int_support::Int::from(1_i128)),
                            ),
                            11 /* terrane-site: standard/process.trn:61:34-61:52 */,
                        ),
                    )
                    .cloned()
                    .ok_or(terrane_collection_support::IndexError {
                        index: __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &(index.clone() + terrane_int_support::Int::from(1_i128)),
                            ),
                            11 /* terrane-site: standard/process.trn:61:34-61:52 */,
                        ),
                    }),
                11 /* terrane-site: standard/process.trn:61:34-61:52 */,
            ),
        );
        values.append(EnvironmentEntry::terrane_construct(name, value));
        index = index.clone() + terrane_int_support::Int::from(2_i128);
    }
    return values.clone();
}
#[derive(Clone)]
pub struct CliSchema {
    pub entries: terrane_collection_support::List<String>,
}
impl CliSchema {
    pub fn terrane_construct(
        declared: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(declared);
        value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        self.entries = declared.clone();
    }
}
#[derive(Clone)]
pub struct CommandLine {
    pub flags: terrane_collection_support::List<String>,
    pub option_names: terrane_collection_support::List<String>,
    pub option_values: terrane_collection_support::List<PlatformString>,
    pub positionals: terrane_collection_support::List<PlatformString>,
    pub diagnostic_arguments: terrane_collection_support::List<terrane_int_support::Int>,
    pub diagnostic_messages: terrane_collection_support::List<String>,
}
impl CommandLine {
    pub fn terrane_construct() -> Self {
        Self {
            flags: terrane_collection_support::List::<String>::new(Vec::new()),
            option_names: terrane_collection_support::List::<String>::new(Vec::new()),
            option_values: terrane_collection_support::List::<
                PlatformString,
            >::new(Vec::new()),
            positionals: terrane_collection_support::List::<
                PlatformString,
            >::new(Vec::new()),
            diagnostic_arguments: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            diagnostic_messages: terrane_collection_support::List::<
                String,
            >::new(Vec::new()),
        }
    }
}
pub fn schema_has(schema: CliSchema, sought: String) -> bool {
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &schema.entries,
    );
    loop {
        let entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry == sought {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<PlatformString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_names: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_values: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut positionals: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut diagnostic_messages: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(supplied.length()),
        )
    {
        let argument: PlatformString = __terrane_raised(
            supplied
                .get_or_error(
                    __terrane_raised(
                        terrane_collection_support::index_from_int(&index.clone()),
                        12 /* terrane-site: standard/process.trn:96:20-96:35 */,
                    ),
                ),
            12 /* terrane-site: standard/process.trn:96:20-96:35 */,
        );
        if !argument.is_text {
            diagnostic_arguments.append(index.clone());
            diagnostic_messages
                .append(String::from("command-line option is not Unicode text"));
        } else {
            let flag_entry: String = format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("flag:")),
                terrane_scalar_support::scalar_text(&argument.text)
            );
            let value_entry: String = format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("value:")),
                terrane_scalar_support::scalar_text(&argument.text)
            );
            if schema_has(schema.clone(), flag_entry) {
                flags.append(argument.text.clone());
            } else if schema_has(schema.clone(), value_entry) {
                if index.clone() + terrane_int_support::Int::from(1_i128)
                    >= terrane_int_support::Int::from(
                        terrane_int_support::Int::from(supplied.length()),
                    )
                {
                    diagnostic_arguments.append(index.clone());
                    diagnostic_messages.append(String::from("option requires a value"));
                } else {
                    option_names.append(argument.text.clone());
                    option_values
                        .append(
                            __terrane_raised(
                                supplied
                                    .get_or_error(
                                        __terrane_raised(
                                            terrane_collection_support::index_from_int(
                                                &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                            ),
                                            13 /* terrane-site: standard/process.trn:111:43-111:62 */,
                                        ),
                                    ),
                                13 /* terrane-site: standard/process.trn:111:43-111:62 */,
                            ),
                        );
                    index = index.clone() + terrane_int_support::Int::from(1_i128);
                }
            } else if argument.text.starts_with(&String::from("--")) {
                diagnostic_arguments.append(index.clone());
                diagnostic_messages.append(String::from("unknown option"));
            } else {
                positionals.append(argument.clone());
            }
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let mut result: CommandLine = CommandLine::terrane_construct();
    result.flags = flags.clone();
    result.option_names = option_names.clone();
    result.option_values = option_values.clone();
    result.positionals = positionals.clone();
    result.diagnostic_arguments = diagnostic_arguments.clone();
    result.diagnostic_messages = diagnostic_messages.clone();
    return result.clone();
}
#[derive(Clone)]
pub struct ExitStatus {
    pub code: terrane_int_support::Int,
    pub valid: bool,
}
impl ExitStatus {
    pub fn terrane_construct() -> Self {
        Self {
            code: terrane_int_support::Int::from(0_i128),
            valid: true,
        }
    }
}
pub fn make_exit_status(requested: terrane_int_support::Int) -> ExitStatus {
    let mut result: ExitStatus = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result.code = terrane_int_support::Int::from(255_i128);
        result.valid = false;
    } else {
        result.code = requested.clone();
    }
    return result.clone();
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
