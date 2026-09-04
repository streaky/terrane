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
    pub static FILES: [&str; 2] = ["case.trn", "core/process.trn"];
    pub static FUNCTIONS: [&str; 7] = [
        "/list-append-bulk-mutation::validate-return",
        "/list-append-bulk-mutation::validate-exit",
        "/list-append-bulk-mutation::validate-throw",
        "/list-append-bulk-mutation::main",
        "/core/process::arguments",
        "/core/process::environment",
        "/core/process::parse-command-line",
    ];
    pub static SITES: [Site; 15] = [
        {
            /* terrane-site-row: site 0: /list-append-bulk-mutation::validate-return (case.trn:23:14-23:23) */
            Site {
                function: 0,
                file: 0,
                line: 23,
                column: 14,
                end_line: 23,
                end_column: 23,
            }
        },
        {
            /* terrane-site-row: site 1: /list-append-bulk-mutation::validate-return (case.trn:24:5-24:12) */
            Site {
                function: 0,
                file: 0,
                line: 24,
                column: 5,
                end_line: 24,
                end_column: 12,
            }
        },
        {
            /* terrane-site-row: site 2: /list-append-bulk-mutation::validate-exit (case.trn:34:14-34:23) */
            Site {
                function: 1,
                file: 0,
                line: 34,
                column: 14,
                end_line: 34,
                end_column: 23,
            }
        },
        {
            /* terrane-site-row: site 3: /list-append-bulk-mutation::validate-exit (case.trn:36:5-36:12) */
            Site {
                function: 1,
                file: 0,
                line: 36,
                column: 5,
                end_line: 36,
                end_column: 12,
            }
        },
        {
            /* terrane-site-row: site 4: /list-append-bulk-mutation::validate-throw (case.trn:46:16-46:25) */
            Site {
                function: 2,
                file: 0,
                line: 46,
                column: 16,
                end_line: 46,
                end_column: 25,
            }
        },
        {
            /* terrane-site-row: site 5: /list-append-bulk-mutation::validate-throw (case.trn:47:9-47:34) */
            Site {
                function: 2,
                file: 0,
                line: 47,
                column: 9,
                end_line: 47,
                end_column: 34,
            }
        },
        {
            /* terrane-site-row: site 6: /list-append-bulk-mutation::validate-throw (case.trn:48:7-48:14) */
            Site {
                function: 2,
                file: 0,
                line: 48,
                column: 7,
                end_line: 48,
                end_column: 14,
            }
        },
        {
            /* terrane-site-row: site 7: /list-append-bulk-mutation::main (case.trn:59:5-59:12) */
            Site {
                function: 3,
                file: 0,
                line: 59,
                column: 5,
                end_line: 59,
                end_column: 12,
            }
        },
        {
            /* terrane-site-row: site 8: /list-append-bulk-mutation::main (case.trn:69:23-69:39) */
            Site {
                function: 3,
                file: 0,
                line: 69,
                column: 23,
                end_line: 69,
                end_column: 39,
            }
        },
        {
            /* terrane-site-row: site 9: /list-append-bulk-mutation::main (case.trn:99:5-99:18) */
            Site {
                function: 3,
                file: 0,
                line: 99,
                column: 5,
                end_line: 99,
                end_column: 18,
            }
        },
        {
            /* terrane-site-row: site 10: /core/process::arguments (core/process.trn:44:49-44:63) */
            Site {
                function: 4,
                file: 1,
                line: 44,
                column: 49,
                end_line: 44,
                end_column: 63,
            }
        },
        {
            /* terrane-site-row: site 11: /core/process::environment (core/process.trn:53:40-53:54) */
            Site {
                function: 5,
                file: 1,
                line: 53,
                column: 40,
                end_line: 53,
                end_column: 54,
            }
        },
        {
            /* terrane-site-row: site 12: /core/process::environment (core/process.trn:54:41-54:59) */
            Site {
                function: 5,
                file: 1,
                line: 54,
                column: 41,
                end_line: 54,
                end_column: 59,
            }
        },
        {
            /* terrane-site-row: site 13: /core/process::parse-command-line (core/process.trn:89:20-89:35) */
            Site {
                function: 6,
                file: 1,
                line: 89,
                column: 20,
                end_line: 89,
                end_column: 35,
            }
        },
        {
            /* terrane-site-row: site 14: /core/process::parse-command-line (core/process.trn:104:43-104:62) */
            Site {
                function: 6,
                file: 1,
                line: 104,
                column: 43,
                end_line: 104,
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
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
pub fn terrane_unhex(text: &str) -> Vec<u8> {
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
pub fn terrane_platform_value(value: std::ffi::OsString) -> String {
    terrane_platform_support::platform_value(value)
}
pub fn terrane_platform_value_is_text(value: &str) -> bool {
    value.starts_with("text:")
}
pub fn terrane_platform_value_text(value: &str) -> String {
    value.strip_prefix("text:").unwrap_or("").to_owned()
}
pub fn terrane_platform_value_bytes(value: &str) -> Vec<u8> {
    value.strip_prefix("raw:").map(terrane_unhex).unwrap_or_default()
}
pub fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}
pub fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [
            terrane_platform_value(name),
            terrane_platform_value(value),
        ])
        .collect()
}
pub fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}
// Source: case.trn
// Namespace: list-append-bulk-mutation
fn validate_large_literal(enabled: bool) {
    if enabled {
        let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
            i64,
        >::new(Vec::new());
        let mut index: i64 = 0;
        {
            let __terrane_list_append_0 = values.make_unique();
            if let (Ok(__terrane_start), Ok(__terrane_end)) = (
                usize::try_from(index),
                usize::try_from(4000000000 as i64),
            ) {
                let __terrane_capacity_limit = 268435456usize
                    / std::mem::size_of::<i64>().max(1);
                __terrane_list_append_0
                    .reserve(
                        __terrane_end
                            .saturating_sub(__terrane_start)
                            .min(__terrane_capacity_limit),
                    );
            }
            while index < 4000000000 {
                __terrane_list_append_0.push(index);
                index = index + 1;
            }
        }
    }
}
fn validate_return() -> terrane_int_support::Int {
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 100000000000000;
    {
        let __terrane_list_append_1 = values.make_unique();
        while index < limit {
            __terrane_list_append_1.push(index);
            if index > 2 {
                return terrane_int_support::Int::from(
                    __terrane_raised(
                        terrane_int_support::fixed_addition(index, 1),
                        0 /* terrane-site: case.trn:23:14-23:23 */,
                    ) as i128,
                );
            }
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                1 /* terrane-site: case.trn:24:5-24:12 */,
            );
        }
    }
    return terrane_int_support::Int::from(0_i128);
}
fn validate_exit() {
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 100000000000000;
    {
        let __terrane_list_append_2 = values.make_unique();
        while index < limit {
            __terrane_list_append_2.push(index);
            if index > 2 {
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised(terrane_int_support::fixed_addition(index,
                    1), 2 /* terrane-site: case.trn:34:14-34:23 */))
                );
                exit(make_exit_status(terrane_int_support::Int::from(0_i128)));
            }
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                3 /* terrane-site: case.trn:36:5-36:12 */,
            );
        }
    }
}
fn validate_throw() {
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
                i64,
            >::new(Vec::new());
            let mut index: i64 = 0;
            let limit: i64 = 100000000000000;
            {
                let __terrane_list_append_3 = values.make_unique();
                while index < limit {
                    __terrane_list_append_3.push(index);
                    if index > 2 {
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&__terrane_raised_completion!(terrane_int_support::fixed_addition(index,
                            1), 4 /* terrane-site: case.trn:46:16-46:25 */))
                        );
                        return TerraneCompletion::Error(
                            TerraneError::raised(
                                TerraneErrorKind::ArithmeticOverflow,
                                5 /* terrane-site: case.trn:47:9-47:34 */,
                            ),
                        );
                    }
                    index = __terrane_raised_completion!(
                        terrane_int_support::fixed_addition(index, 1),
                        6 /* terrane-site: case.trn:48:7-48:14 */
                    );
                }
            }
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
                    && __terrane_error_0.kind == TerraneErrorKind::ArithmeticOverflow
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("throw-caught"))
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
}
fn main() {
    validate_large_literal(false);
    let mut values: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut index: i64 = 0;
    let limit: i64 = 4;
    {
        let __terrane_list_append_4 = values.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(index),
            usize::try_from(limit),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_4
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while index < limit {
            __terrane_list_append_4.push(index);
            index = __terrane_raised(
                terrane_int_support::fixed_addition(index, 1),
                7 /* terrane-site: case.trn:59:5-59:12 */,
            );
        }
    }
    let original: terrane_collection_support::List<i64> = values.clone();
    values.append(9);
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(original
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    let mut dependent: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut dependent_index: i64 = 0;
    while dependent_index < 3 {
        dependent
            .append(
                __terrane_raised(
                    terrane_int_support::coerce::<
                        i64,
                    >(&terrane_int_support::Int::from(dependent.length())),
                    8 /* terrane-site: case.trn:69:23-69:39 */,
                ),
            );
        dependent_index = dependent_index + 1;
    }
    let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
        &dependent,
    );
    loop {
        let value = match __terrane_iterator_5.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&value));
    }
    let mut inner_index: i64 = 0;
    while inner_index < 1 {
        let mut inner: terrane_collection_support::List<i64> = terrane_collection_support::List::<
            i64,
        >::new(Vec::new());
        inner.append(inner_index);
        inner_index = inner_index + 1;
    }
    let mut nested: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut outer_index: i64 = 0;
    {
        let __terrane_list_append_6 = nested.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(outer_index),
            usize::try_from(3 as i64),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_6
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while outer_index < 3 {
            __terrane_list_append_6.push(outer_index);
            let mut nested_index: i64 = 0;
            while nested_index < 2 {
                __terrane_list_append_6.push(nested_index);
                nested_index = nested_index + 1;
            }
            outer_index = outer_index + 1;
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested
        .length()))
    );
    let mut early_exit: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut early_index: i64 = 0;
    let early_limit: i64 = 100000000000000;
    {
        let __terrane_list_append_7 = early_exit.make_unique();
        while early_index < early_limit {
            __terrane_list_append_7.push(early_index);
            if early_index > 2 {
                break;
            }
            early_index = __terrane_raised(
                terrane_int_support::fixed_addition(early_index, 1),
                9 /* terrane-site: case.trn:99:5-99:18 */,
            );
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(early_exit
        .length()))
    );
    let mut nested_break: terrane_collection_support::List<i64> = terrane_collection_support::List::<
        i64,
    >::new(Vec::new());
    let mut break_outer: i64 = 0;
    {
        let __terrane_list_append_8 = nested_break.make_unique();
        if let (Ok(__terrane_start), Ok(__terrane_end)) = (
            usize::try_from(break_outer),
            usize::try_from(3 as i64),
        ) {
            let __terrane_capacity_limit = 268435456usize
                / std::mem::size_of::<i64>().max(1);
            __terrane_list_append_8
                .reserve(
                    __terrane_end
                        .saturating_sub(__terrane_start)
                        .min(__terrane_capacity_limit),
                );
        }
        while break_outer < 3 {
            __terrane_list_append_8.push(break_outer);
            let break_inner: i64 = 0;
            while break_inner < 3 {
                break;
            }
            break_outer = break_outer + 1;
        }
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested_break
        .length()))
    );
    println!("{}", terrane_scalar_support::scalar_text(&validate_return()));
    validate_throw();
    validate_exit();
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
    pub name: NativeString,
    pub value: NativeString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: NativeString, entry_value: NativeString) -> Self {
        let mut value = Self {
            name: NativeString::terrane_construct(String::from("text:")),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        value.construct(name, entry_value);
        value
    }
    pub fn construct(&mut self, name: NativeString, entry_value: NativeString) {
        self.name = name.clone();
        self.value = entry_value.clone();
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
        let mut value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: NativeString::terrane_construct(String::from("text:")),
        };
        value.construct(did_fail, is_available, detail, result_value);
        value
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
        self.value = result_value.clone();
    }
}
pub fn process_host_name() -> ProcessHostNameResult {
    let raw: TerranePlatformResult = terrane_platform_support::system_host_name();
    return ProcessHostNameResult::terrane_construct(
        raw.failed,
        raw.flag,
        raw.message.clone(),
        NativeString::terrane_construct(raw.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<NativeString> {
    let encoded: Vec<String> = terrane_process_arguments();
    let mut values: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_0 = values.make_unique();
        while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
            __terrane_list_append_0
                .push(
                    NativeString::terrane_construct(
                        __terrane_raised(
                            encoded
                                .get(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        10 /* terrane-site: core/process.trn:44:49-44:63 */,
                                    ),
                                )
                                .cloned()
                                .ok_or(terrane_collection_support::IndexError {
                                    index: __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        10 /* terrane-site: core/process.trn:44:49-44:63 */,
                                    ),
                                }),
                            10 /* terrane-site: core/process.trn:44:49-44:63 */,
                        ),
                    ),
                );
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    }
    return values.clone();
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String> = terrane_environment_entries();
    let mut values: terrane_collection_support::List<EnvironmentEntry> = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_1 = values.make_unique();
        while index.clone() + terrane_int_support::Int::from(1_i128)
            < terrane_int_support::Int::from(encoded.len() as i128)
        {
            let name: NativeString = NativeString::terrane_construct(
                __terrane_raised(
                    encoded
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                11 /* terrane-site: core/process.trn:53:40-53:54 */,
                            ),
                        )
                        .cloned()
                        .ok_or(terrane_collection_support::IndexError {
                            index: __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                11 /* terrane-site: core/process.trn:53:40-53:54 */,
                            ),
                        }),
                    11 /* terrane-site: core/process.trn:53:40-53:54 */,
                ),
            );
            let value: NativeString = NativeString::terrane_construct(
                __terrane_raised(
                    encoded
                        .get(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                12 /* terrane-site: core/process.trn:54:41-54:59 */,
                            ),
                        )
                        .cloned()
                        .ok_or(terrane_collection_support::IndexError {
                            index: __terrane_raised(
                                terrane_collection_support::index_from_int(
                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                ),
                                12 /* terrane-site: core/process.trn:54:41-54:59 */,
                            ),
                        }),
                    12 /* terrane-site: core/process.trn:54:41-54:59 */,
                ),
            );
            __terrane_list_append_1
                .push(EnvironmentEntry::terrane_construct(name, value));
            index = index.clone() + terrane_int_support::Int::from(2_i128);
        }
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
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &schema.entries,
    );
    loop {
        let entry = match __terrane_iterator_2.next() {
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
    supplied: terrane_collection_support::List<NativeString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_names: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_values: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut positionals: terrane_collection_support::List<NativeString> = terrane_collection_support::List::<
        NativeString,
    >::new(Vec::new());
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut diagnostic_messages: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    {
        let __terrane_list_append_3 = diagnostic_arguments.make_unique();
        let __terrane_list_append_4 = diagnostic_messages.make_unique();
        let __terrane_list_append_5 = flags.make_unique();
        let __terrane_list_append_6 = option_names.make_unique();
        let __terrane_list_append_7 = option_values.make_unique();
        let __terrane_list_append_8 = positionals.make_unique();
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(supplied.length()),
            )
        {
            let argument: NativeString = __terrane_raised(
                supplied
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            13 /* terrane-site: core/process.trn:89:20-89:35 */,
                        ),
                    ),
                13 /* terrane-site: core/process.trn:89:20-89:35 */,
            );
            if !argument.is_text {
                __terrane_list_append_3.push(index.clone());
                __terrane_list_append_4
                    .push(String::from("command-line option is not Unicode text"));
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
                    __terrane_list_append_5.push(argument.text.clone());
                } else if schema_has(schema.clone(), value_entry) {
                    if index.clone() + terrane_int_support::Int::from(1_i128)
                        >= terrane_int_support::Int::from(
                            terrane_int_support::Int::from(supplied.length()),
                        )
                    {
                        __terrane_list_append_3.push(index.clone());
                        __terrane_list_append_4
                            .push(String::from("option requires a value"));
                    } else {
                        __terrane_list_append_6.push(argument.text.clone());
                        __terrane_list_append_7
                            .push(
                                __terrane_raised(
                                    supplied
                                        .get_or_error(
                                            __terrane_raised(
                                                terrane_collection_support::index_from_int(
                                                    &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                                ),
                                                14 /* terrane-site: core/process.trn:104:43-104:62 */,
                                            ),
                                        ),
                                    14 /* terrane-site: core/process.trn:104:43-104:62 */,
                                ),
                            );
                        index = index.clone() + terrane_int_support::Int::from(1_i128);
                    }
                } else if argument.text.starts_with(&String::from("--")) {
                    __terrane_list_append_3.push(index.clone());
                    __terrane_list_append_4.push(String::from("unknown option"));
                } else {
                    __terrane_list_append_8.push(argument.clone());
                }
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
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
