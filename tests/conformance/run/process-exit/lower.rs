// Generated deterministically by Terrane <version>.
// Runtime support: platform_result_type.rs, platform_process.rs
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
    pub static FILES: [&str; 1] = ["core/process.trn"];
    pub static FUNCTIONS: [&str; 3] = [
        "/core/process::arguments",
        "/core/process::environment",
        "/core/process::parse-command-line",
    ];
    pub static SITES: [Site; 5] = [
        /* terrane-site-row: site 0: /core/process::arguments (core/process.trn:45:49-45:63) */
        { Site { function: 0, file: 0, line: 45, column: 49, end_line: 45, end_column: 63 } },
        /* terrane-site-row: site 1: /core/process::environment (core/process.trn:54:40-54:54) */
        { Site { function: 1, file: 0, line: 54, column: 40, end_line: 54, end_column: 54 } },
        /* terrane-site-row: site 2: /core/process::environment (core/process.trn:55:41-55:59) */
        { Site { function: 1, file: 0, line: 55, column: 41, end_line: 55, end_column: 59 } },
        /* terrane-site-row: site 3: /core/process::parse-command-line (core/process.trn:90:20-90:35) */
        { Site { function: 2, file: 0, line: 90, column: 20, end_line: 90, end_column: 35 } },
        /* terrane-site-row: site 4: /core/process::parse-command-line (core/process.trn:105:43-105:62) */
        { Site { function: 2, file: 0, line: 105, column: 43, end_line: 105, end_column: 62 } },
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
// Namespace: conformance/process-exit
fn main() {
    let status_terrane_f0_s104: ExitStatus;
    status_terrane_f0_s104 = make_exit_status(terrane_int_support::Int::from(7_i128));
    exit(status_terrane_f0_s104);
}
// Source: core/process.trn
// Namespace: core/process
#[derive(Clone)]
pub struct NativeString {
    pub is_text: bool,
    pub text: String,
    pub raw: Vec<u8>,
}
impl NativeString {
    pub fn terrane_construct(encoded: String) -> Self {
        let mut __terrane_constructed_value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::from([]),
        };
        __terrane_constructed_value.construct(encoded);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, encoded: String) {
        self.is_text = terrane_platform_value_is_text(&encoded);
        self.text = terrane_platform_value_text(&encoded);
        self.raw = terrane_platform_value_bytes(&encoded);
    }
}
#[derive(Clone)]
pub struct EnvironmentEntry {
    pub name: NativeString,
    pub value: NativeString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: NativeString, entry_value: NativeString) -> Self {
        let mut __terrane_constructed_value = Self {
            name: NativeString::terrane_construct(String::from("text:")),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        __terrane_constructed_value.construct(name, entry_value);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, name: NativeString, entry_value: NativeString) {
        self.name = name;
        self.value = entry_value;
    }
}
#[derive(Clone)]
pub struct ProcessHostNameResult {
    pub failed: bool,
    pub available: bool,
    pub message: String,
    pub value: NativeString,
}
impl ProcessHostNameResult {
    pub fn terrane_construct(
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        __terrane_constructed_value
            .construct(did_fail, is_available, detail, result_value);
        __terrane_constructed_value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: NativeString,
    ) {
        self.failed = did_fail;
        self.available = is_available;
        self.message = detail;
        self.value = result_value;
    }
}
pub fn process_host_name() -> ProcessHostNameResult {
    let raw_terrane_f1_s1079: TerranePlatformResult;
    raw_terrane_f1_s1079 = terrane_platform_support::system_host_name();
    return ProcessHostNameResult::terrane_construct(
        raw_terrane_f1_s1079.failed,
        raw_terrane_f1_s1079.flag,
        raw_terrane_f1_s1079.message.clone(),
        NativeString::terrane_construct(raw_terrane_f1_s1079.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<NativeString> {
    let encoded_terrane_f1_s1332: Vec<String>;
    let mut values_terrane_f1_s1372: terrane_collection_support::List<NativeString>;
    let mut index_terrane_f1_s1412: terrane_int_support::Int;
    encoded_terrane_f1_s1332 = terrane_process_arguments();
    values_terrane_f1_s1372 = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    index_terrane_f1_s1412 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = values_terrane_f1_s1372.make_unique();
        while index_terrane_f1_s1412.clone()
            < terrane_int_support::Int::from(encoded_terrane_f1_s1332.len() as i128)
        {
            __terrane_list_append_0
                .push(
                    NativeString::terrane_construct(
                        __terrane_raised(
                            {
                                let __terrane_receiver = &encoded_terrane_f1_s1332;
                                let __terrane_index = __terrane_raised(
                                    terrane_collection_support::index_from_int(
                                        &index_terrane_f1_s1412.clone(),
                                    ),
                                    0 /* terrane-site: core/process.trn:45:49-45:63 */,
                                );
                                __terrane_receiver
                                    .get(__terrane_index)
                                    .cloned()
                                    .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                        __terrane_index,
                                    ))
                            },
                            0 /* terrane-site: core/process.trn:45:49-45:63 */,
                        ),
                    ),
                );
            index_terrane_f1_s1412 = index_terrane_f1_s1412.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    return values_terrane_f1_s1372;
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded_terrane_f1_s1620: Vec<String>;
    let mut values_terrane_f1_s1662: terrane_collection_support::List<EnvironmentEntry>;
    let mut index_terrane_f1_s1706: terrane_int_support::Int;
    let mut name_terrane_f1_s1765: NativeString;
    let mut value_terrane_f1_s1819: NativeString;
    encoded_terrane_f1_s1620 = terrane_environment_entries();
    values_terrane_f1_s1662 = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    index_terrane_f1_s1706 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = values_terrane_f1_s1662.make_unique();
        while index_terrane_f1_s1706.clone() + terrane_int_support::Int::from(1_i128)
            < terrane_int_support::Int::from(encoded_terrane_f1_s1620.len() as i128)
        {
            name_terrane_f1_s1765 = NativeString::terrane_construct(
                __terrane_raised(
                    {
                        let __terrane_receiver = &encoded_terrane_f1_s1620;
                        let __terrane_index = __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &index_terrane_f1_s1706.clone(),
                            ),
                            1 /* terrane-site: core/process.trn:54:40-54:54 */,
                        );
                        __terrane_receiver
                            .get(__terrane_index)
                            .cloned()
                            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                __terrane_index,
                            ))
                    },
                    1 /* terrane-site: core/process.trn:54:40-54:54 */,
                ),
            );
            value_terrane_f1_s1819 = NativeString::terrane_construct(
                __terrane_raised(
                    {
                        let __terrane_receiver = &encoded_terrane_f1_s1620;
                        let __terrane_index = __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &(index_terrane_f1_s1706.clone()
                                    + terrane_int_support::Int::from(1_i128)),
                            ),
                            2 /* terrane-site: core/process.trn:55:41-55:59 */,
                        );
                        __terrane_receiver
                            .get(__terrane_index)
                            .cloned()
                            .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                                __terrane_index,
                            ))
                    },
                    2 /* terrane-site: core/process.trn:55:41-55:59 */,
                ),
            );
            __terrane_list_append_1
                .push(
                    EnvironmentEntry::terrane_construct(
                        name_terrane_f1_s1765,
                        value_terrane_f1_s1819,
                    ),
                );
            index_terrane_f1_s1706 = index_terrane_f1_s1706.clone()
                + terrane_int_support::Int::from(2_i128);
        }
    }
    return values_terrane_f1_s1662;
}
#[derive(Clone)]
pub struct CliSchema {
    pub entries: terrane_collection_support::List<String>,
}
impl CliSchema {
    pub fn terrane_construct(
        declared: terrane_collection_support::List<String>,
    ) -> Self {
        let mut __terrane_constructed_value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        __terrane_constructed_value.construct(declared);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        self.entries = declared;
    }
}
#[derive(Clone)]
pub struct CommandLine {
    pub flags: terrane_collection_support::List<String>,
    pub option_names: terrane_collection_support::List<String>,
    pub option_values: terrane_collection_support::List<NativeString>,
    pub positionals: terrane_collection_support::List<NativeString>,
    pub diagnostic_arguments: terrane_collection_support::List<terrane_int_support::Int>,
    pub diagnostic_messages: terrane_collection_support::List<String>,
}
impl CommandLine {
    pub fn terrane_construct() -> Self {
        Self {
            flags: terrane_collection_support::List::<String>::new(Vec::new()),
            option_names: terrane_collection_support::List::<String>::new(Vec::new()),
            option_values: terrane_collection_support::List::<
                NativeString,
            >::new(Vec::new()),
            positionals: terrane_collection_support::List::<
                NativeString,
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
    let mut entry_terrane_f1_s2454: String;
    let __terrane_iterable_0 = schema.entries.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        entry_terrane_f1_s2454 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry_terrane_f1_s2454.as_str() == sought.as_str() {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<NativeString>,
) -> CommandLine {
    let mut flags_terrane_f1_s2643: terrane_collection_support::List<String>;
    let mut option_names_terrane_f1_s2675: terrane_collection_support::List<String>;
    let mut option_values_terrane_f1_s2714: terrane_collection_support::List<
        NativeString,
    >;
    let mut positionals_terrane_f1_s2761: terrane_collection_support::List<NativeString>;
    let mut diagnostic_arguments_terrane_f1_s2806: terrane_collection_support::List<
        terrane_int_support::Int,
    >;
    let mut diagnostic_messages_terrane_f1_s2850: terrane_collection_support::List<
        String,
    >;
    let mut index_terrane_f1_s2896: terrane_int_support::Int;
    let mut argument_terrane_f1_s2952: NativeString;
    let mut flag_entry_terrane_f1_s3165: String;
    let mut value_entry_terrane_f1_s3220: String;
    let mut result_terrane_f1_s4025: CommandLine;
    flags_terrane_f1_s2643 = terrane_collection_support::List::<String>::new(Vec::new());
    option_names_terrane_f1_s2675 = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    option_values_terrane_f1_s2714 = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    positionals_terrane_f1_s2761 = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    diagnostic_arguments_terrane_f1_s2806 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(Vec::new());
    diagnostic_messages_terrane_f1_s2850 = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    index_terrane_f1_s2896 = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_2 = diagnostic_arguments_terrane_f1_s2806
            .make_unique();
        let __terrane_list_append_3 = diagnostic_messages_terrane_f1_s2850.make_unique();
        let __terrane_list_append_4 = flags_terrane_f1_s2643.make_unique();
        let __terrane_list_append_5 = option_names_terrane_f1_s2675.make_unique();
        let __terrane_list_append_6 = option_values_terrane_f1_s2714.make_unique();
        let __terrane_list_append_7 = positionals_terrane_f1_s2761.make_unique();
        while index_terrane_f1_s2896.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(supplied.length()),
            )
        {
            argument_terrane_f1_s2952 = __terrane_raised(
                supplied
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(
                                &index_terrane_f1_s2896.clone(),
                            ),
                            3 /* terrane-site: core/process.trn:90:20-90:35 */,
                        ),
                    ),
                3 /* terrane-site: core/process.trn:90:20-90:35 */,
            );
            if !argument_terrane_f1_s2952.is_text {
                __terrane_list_append_2.push(index_terrane_f1_s2896.clone());
                __terrane_list_append_3
                    .push(String::from("command-line option is not Unicode text"));
            } else {
                flag_entry_terrane_f1_s3165 = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("flag:")),
                    terrane_scalar_support::scalar_text(&argument_terrane_f1_s2952.text)
                );
                value_entry_terrane_f1_s3220 = format!(
                    "{}{}", terrane_scalar_support::scalar_text(&String::from("value:")),
                    terrane_scalar_support::scalar_text(&argument_terrane_f1_s2952.text)
                );
                if schema_has(schema.clone(), flag_entry_terrane_f1_s3165) {
                    __terrane_list_append_4.push(argument_terrane_f1_s2952.text.clone());
                } else if schema_has(schema.clone(), value_entry_terrane_f1_s3220) {
                    if index_terrane_f1_s2896.clone()
                        + terrane_int_support::Int::from(1_i128)
                        >= terrane_int_support::Int::from(
                            terrane_int_support::Int::from(supplied.length()),
                        )
                    {
                        __terrane_list_append_2.push(index_terrane_f1_s2896.clone());
                        __terrane_list_append_3
                            .push(String::from("option requires a value"));
                    } else {
                        __terrane_list_append_5
                            .push(argument_terrane_f1_s2952.text.clone());
                        __terrane_list_append_6
                            .push(
                                __terrane_raised(
                                    supplied
                                        .get_or_error(
                                            __terrane_raised(
                                                terrane_collection_support::index_from_int(
                                                    &(index_terrane_f1_s2896.clone()
                                                        + terrane_int_support::Int::from(1_i128)),
                                                ),
                                                4 /* terrane-site: core/process.trn:105:43-105:62 */,
                                            ),
                                        ),
                                    4 /* terrane-site: core/process.trn:105:43-105:62 */,
                                ),
                            );
                        index_terrane_f1_s2896 = index_terrane_f1_s2896.clone()
                            + terrane_int_support::Int::from(1_i128);
                    }
                } else if argument_terrane_f1_s2952.text.starts_with(&String::from("--"))
                {
                    __terrane_list_append_2.push(index_terrane_f1_s2896.clone());
                    __terrane_list_append_3.push(String::from("unknown option"));
                } else {
                    __terrane_list_append_7.push(argument_terrane_f1_s2952);
                }
            }
            index_terrane_f1_s2896 = index_terrane_f1_s2896.clone()
                + terrane_int_support::Int::from(1_i128);
        }
    }
    result_terrane_f1_s4025 = CommandLine::terrane_construct();
    result_terrane_f1_s4025.flags = flags_terrane_f1_s2643;
    result_terrane_f1_s4025.option_names = option_names_terrane_f1_s2675;
    result_terrane_f1_s4025.option_values = option_values_terrane_f1_s2714;
    result_terrane_f1_s4025.positionals = positionals_terrane_f1_s2761;
    result_terrane_f1_s4025.diagnostic_arguments = diagnostic_arguments_terrane_f1_s2806;
    result_terrane_f1_s4025.diagnostic_messages = diagnostic_messages_terrane_f1_s2850;
    return result_terrane_f1_s4025;
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
    let mut result_terrane_f1_s4444: ExitStatus;
    result_terrane_f1_s4444 = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result_terrane_f1_s4444.code = terrane_int_support::Int::from(255_i128);
        result_terrane_f1_s4444.valid = false;
    } else {
        result_terrane_f1_s4444.code = requested.clone();
    }
    return result_terrane_f1_s4444;
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
pub fn native_text(value: String) -> NativeString {
    let encoded_terrane_f1_s4755: String;
    encoded_terrane_f1_s4755 = terrane_platform_value_from_text(&value);
    return NativeString::terrane_construct(encoded_terrane_f1_s4755);
}
pub fn native_raw(value: Vec<u8>) -> NativeString {
    let encoded_terrane_f1_s4897: String;
    encoded_terrane_f1_s4897 = terrane_platform_value_from_bytes(&value);
    return NativeString::terrane_construct(encoded_terrane_f1_s4897);
}
pub fn native_text_value(value: NativeString) -> Option<String> {
    if value.is_text {
        return Some(value.text.clone());
    }
    return None;
}
pub fn native_raw_value(value: NativeString) -> Vec<u8> {
    return value.raw.clone();
}
pub fn environment_pair(key: NativeString, item: NativeString) -> EnvironmentEntry {
    return EnvironmentEntry::terrane_construct(key, item);
}
pub fn encode_native_string(value: NativeString) -> String {
    if value.is_text {
        return terrane_platform_value_from_text(&value.text);
    }
    return terrane_platform_value_from_bytes(&value.raw);
}
