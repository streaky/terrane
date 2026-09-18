// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs, async_dependency.rs
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
    pub static DESCRIPTORS: [&str; 3] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
        "/deps/sqlx-core::Error",
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
    pub static SITES: [Site; 17] = [
        /* terrane-site-row: site 0: /app::main (src/main.trn:13:13-13:39) */
        { Site { function: 0, file: 0, line: 13, column: 13, end_line: 13, end_column: 39 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:14:13-14:41) */
        { Site { function: 0, file: 0, line: 14, column: 13, end_line: 14, end_column: 41 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:15:13-15:44) */
        { Site { function: 0, file: 0, line: 15, column: 13, end_line: 15, end_column: 44 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:16:22-16:38) */
        { Site { function: 0, file: 0, line: 16, column: 22, end_line: 16, end_column: 38 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:22:9-22:52) */
        { Site { function: 0, file: 0, line: 22, column: 9, end_line: 22, end_column: 52 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:23:9-23:69) */
        { Site { function: 0, file: 0, line: 23, column: 9, end_line: 23, end_column: 69 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:24:9-24:70) */
        { Site { function: 0, file: 0, line: 24, column: 9, end_line: 24, end_column: 70 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:25:34-25:79) */
        { Site { function: 0, file: 0, line: 25, column: 34, end_line: 25, end_column: 79 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:27:25-27:70) */
        { Site { function: 0, file: 0, line: 27, column: 25, end_line: 27, end_column: 70 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:28:16-28:36) */
        { Site { function: 0, file: 0, line: 28, column: 16, end_line: 28, end_column: 36 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:29:10-29:17) */
        { Site { function: 0, file: 0, line: 29, column: 10, end_line: 29, end_column: 17 } },
        /* terrane-site-row: site 11: /app::main (src/main.trn:30:32-30:77) */
        { Site { function: 0, file: 0, line: 30, column: 32, end_line: 30, end_column: 77 } },
        /* terrane-site-row: site 12: /app::main (src/main.trn:31:23-31:50) */
        { Site { function: 0, file: 0, line: 31, column: 23, end_line: 31, end_column: 50 } },
        /* terrane-site-row: site 13: /app::main (src/main.trn:32:10-32:24) */
        { Site { function: 0, file: 0, line: 32, column: 10, end_line: 32, end_column: 24 } },
        /* terrane-site-row: site 14: /app::main (src/main.trn:34:11-34:55) */
        { Site { function: 0, file: 0, line: 34, column: 11, end_line: 34, end_column: 55 } },
        /* terrane-site-row: site 15: /app::main (src/main.trn:37:10-37:32) */
        { Site { function: 0, file: 0, line: 37, column: 10, end_line: 37, end_column: 32 } },
        /* terrane-site-row: site 16: /app::main (src/main.trn:37:9-37:33) */
        { Site { function: 0, file: 0, line: 37, column: 9, end_line: 37, end_column: 33 } },
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
    __terrane_run(async move {
        let mut options: SqliteConnectOptions = __terrane_raised(
            terrane_static_trn_53716c697465436f6e6e6563744f7074696f6e73_new(),
            0 /* terrane-site: src/main.trn:13:13-13:39 */,
        );
        options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    options.filename(String::from(":memory:").as_str())
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "sqlx_sqlite",
                            "sqlx_sqlite::SqliteConnectOptions::filename",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:14:13-14:41 */,
        );
        options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| options.create_if_missing(true)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "sqlx_sqlite",
                            "sqlx_sqlite::SqliteConnectOptions::create_if_missing",
                        ),
                    )
                }
            },
            2 /* terrane-site: src/main.trn:15:13-15:44 */,
        );
        let mut connection: SqliteConnection = __terrane_traced(
            __terrane_await({
                    let __terrane_future = connect(&options);
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            3 /* terrane-site: src/main.trn:16:22-16:38 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:16:22-16:38 */,
        );
        let create_sql: String = String::from(
            "CREATE TABLE records (body BLOB NOT NULL)",
        );
        let insert_sql: String = String::from("INSERT INTO records (body) VALUES (?)");
        let select_sql: String = String::from("SELECT body FROM records ORDER BY body");
        let second_sql: String = String::from(
            "SELECT body FROM records ORDER BY body LIMIT 1 OFFSET 1",
        );
        let invalid_sql: String = String::from("SELECT missing FROM absent");
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&create_sql)
                            .execute(&mut connection);
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::DescriptorId(2),
                                                error.to_string(),
                                                crate::TERRANE_NO_SITE,
                                            ),
                                        ),
                                    )
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "sqlx_core",
                                            "sqlx_core::query::Query::execute",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            4 /* terrane-site: src/main.trn:22:9-22:52 */,
                        )
                    }
                })
                .await,
            4 /* terrane-site: src/main.trn:22:9-22:52 */,
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&insert_sql)
                            .bind(Vec::from([102, 105, 114, 115, 116]))
                            .execute(&mut connection);
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::DescriptorId(2),
                                                error.to_string(),
                                                crate::TERRANE_NO_SITE,
                                            ),
                                        ),
                                    )
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "sqlx_core",
                                            "sqlx_core::query::Query::execute",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            5 /* terrane-site: src/main.trn:23:9-23:69 */,
                        )
                    }
                })
                .await,
            5 /* terrane-site: src/main.trn:23:9-23:69 */,
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&insert_sql)
                            .bind(Vec::from([115, 101, 99, 111, 110, 100]))
                            .execute(&mut connection);
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::DescriptorId(2),
                                                error.to_string(),
                                                crate::TERRANE_NO_SITE,
                                            ),
                                        ),
                                    )
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "sqlx_core",
                                            "sqlx_core::query::Query::execute",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            6 /* terrane-site: src/main.trn:24:9-24:70 */,
                        )
                    }
                })
                .await,
            6 /* terrane-site: src/main.trn:24:9-24:70 */,
        );
        let rows: terrane_collection_support::List<SqliteRow> = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&select_sql)
                            .fetch_all(&mut connection);
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(
                                        terrane_collection_support::List::new(
                                            value.into_iter().map(|item| item).collect(),
                                        ),
                                    )
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::DescriptorId(2),
                                                error.to_string(),
                                                crate::TERRANE_NO_SITE,
                                            ),
                                        ),
                                    )
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "sqlx_core",
                                            "sqlx_core::query::Query::fetch_all",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            7 /* terrane-site: src/main.trn:25:34-25:79 */,
                        )
                    }
                })
                .await,
            7 /* terrane-site: src/main.trn:25:34-25:79 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rows
            .length()))
        );
        let row: SqliteRow = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&select_sql)
                            .fetch_one(&mut connection);
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::DescriptorId(2),
                                                error.to_string(),
                                                crate::TERRANE_NO_SITE,
                                            ),
                                        ),
                                    )
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "sqlx_core",
                                            "sqlx_core::query::Query::fetch_one",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            8 /* terrane-site: src/main.trn:27:25-27:70 */,
                        )
                    }
                })
                .await,
            8 /* terrane-site: src/main.trn:27:25-27:70 */,
        );
        let body: Vec<u8> = __terrane_raised(
            match std::panic::catch_unwind(|| <sqlx_sqlite::SqliteRow as sqlx_core::row::Row>::try_get(
                &row,
                String::from("body").as_str(),
            )) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::DescriptorId(2),
                                error.to_string(),
                                crate::TERRANE_NO_SITE,
                            ),
                        ),
                    )
                }
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "sqlx-sqlite",
                            "<sqlx_sqlite::SqliteRow as sqlx_core::row::Row>::try_get",
                        ),
                    )
                }
            },
            9 /* terrane-site: src/main.trn:28:16-28:36 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            10 /* terrane-site: src/main.trn:29:10-29:17 */)), 10 /* terrane-site: src/main.trn:29:10-29:17 */))
        );
        let second_row: SqliteRow = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = sqlx_core::query::query(&second_sql)
                            .fetch_one(&mut connection);
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => Ok(value),
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::DescriptorId(2),
                                                error.to_string(),
                                                crate::TERRANE_NO_SITE,
                                            ),
                                        ),
                                    )
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "sqlx_core",
                                            "sqlx_core::query::Query::fetch_one",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            11 /* terrane-site: src/main.trn:30:32-30:77 */,
                        )
                    }
                })
                .await,
            11 /* terrane-site: src/main.trn:30:32-30:77 */,
        );
        let second_body: Vec<u8> = __terrane_raised(
            match std::panic::catch_unwind(|| <sqlx_sqlite::SqliteRow as sqlx_core::row::Row>::try_get(
                &second_row,
                String::from("body").as_str(),
            )) {
                Ok(Ok(value)) => Ok(value),
                Ok(Err(error)) => {
                    Err(
                        crate::TerraneForeignError(
                            crate::TerraneError::custom_raised(
                                crate::DescriptorId(2),
                                error.to_string(),
                                crate::TERRANE_NO_SITE,
                            ),
                        ),
                    )
                }
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "sqlx-sqlite",
                            "<sqlx_sqlite::SqliteRow as sqlx_core::row::Row>::try_get",
                        ),
                    )
                }
            },
            12 /* terrane-site: src/main.trn:31:23-31:50 */,
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_collection_support::byte_at(&second_body,
            __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            13 /* terrane-site: src/main.trn:32:10-32:24 */)), 13 /* terrane-site: src/main.trn:32:10-32:24 */))
        );
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                __terrane_traced_completion!(
                    __terrane_await({ let __terrane_future = { let __terrane_call =
                    sqlx_core::query::query(&invalid_sql).execute(&mut connection); async
                    move { match crate
                    ::__terrane_dependency_await_unwind(__terrane_call). await {
                    Ok(Ok(value)) => Ok(value), Ok(Err(error)) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::DescriptorId(2), error.to_string(), crate ::TERRANE_NO_SITE))),
                    Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
                    "sqlx_core", "sqlx_core::query::Query::execute")) } } }; async move {
                    __terrane_raised_err(__terrane_future. await, 14 /* terrane-site: src/main.trn:34:11-34:55 */) } }). await, 14 /* terrane-site: src/main.trn:34:11-34:55 */
                );
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0
                        && __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(2))
                    {
                        __terrane_handled_0 = true;
                        println!(
                            "{}",
                            terrane_scalar_support::scalar_text(&String::from("query-error"))
                        );
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = close(connection);
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            15 /* terrane-site: src/main.trn:37:10-37:32 */,
                        )
                    }
                })
                .await,
            16 /* terrane-site: src/main.trn:37:9-37:33 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&String::from("complete")));
    });
}
// Source: <terrane>/projected/deps/sqlx-core.trn
// Namespace: deps/sqlx-core
pub use sqlx_core::Error;
pub use sqlx_core::query::Query as Query732e8e840a8786347a71ae9edf7925993cd0ffa16fdb014ebbb37aab66557d8a;
pub use sqlx_sqlite::SqliteAutoVacuum;
pub use sqlx_sqlite::SqliteConnectOptions;
pub use sqlx_sqlite::SqliteConnection;
pub use sqlx_sqlite::SqliteJournalMode;
pub use sqlx_sqlite::SqliteLockingMode;
pub use sqlx_sqlite::SqliteRow;
pub use sqlx_sqlite::SqliteSynchronous;
pub use core::time::Duration as TerraneNs4Deps11SqlxSqliteDuration;
pub use core::cmp::Ordering as TerraneNs4Deps11SqlxSqliteOrdering;
pub use std::path::Path as TerraneNs4Deps11SqlxSqlitePath;
// Source: <terrane>/projected/deps/sqlx-core/query.trn
// Namespace: deps/sqlx-core/query
// Source: <terrane>/projected/deps/sqlx-sqlite.trn
// Namespace: deps/sqlx-sqlite
pub fn terrane_static_trn_53716c697465436f6e6e6563744f7074696f6e73_new() -> Result<
    SqliteConnectOptions,
    crate::TerraneForeignError,
> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| sqlx_sqlite::SqliteConnectOptions::new()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "sqlx-sqlite",
                    "sqlx_sqlite::SqliteConnectOptions",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/sqlx-sqlite/sqliteconnection.trn
// Namespace: deps/sqlx-sqlite/sqliteconnection
pub async fn close(
    receiver: SqliteConnection,
) -> Result<(), crate::TerraneForeignError> {
    let receiver = receiver;
    match crate::__terrane_dependency_await_unwind(
            <sqlx_sqlite::SqliteConnection as sqlx_core::connection::Connection>::close(
                receiver,
            ),
        )
        .await
    {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::DescriptorId(2),
                        error.to_string(),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "sqlx-sqlite",
                    "<sqlx_sqlite::SqliteConnection as sqlx_core::connection::Connection>::close",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/sqlx-sqlite/sqliteconnectoptions.trn
// Namespace: deps/sqlx-sqlite/sqliteconnectoptions
pub async fn connect(
    receiver: &SqliteConnectOptions,
) -> Result<SqliteConnection, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            <sqlx_sqlite::SqliteConnectOptions as sqlx_core::connection::ConnectOptions>::connect(
                receiver,
            ),
        )
        .await
    {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::DescriptorId(2),
                        error.to_string(),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "sqlx-sqlite",
                    "<sqlx_sqlite::SqliteConnectOptions as sqlx_core::connection::ConnectOptions>::connect",
                ),
            )
        }
    }
}
// Source: <terrane>/projected/deps/sqlx-sqlite/sqliterow.trn
// Namespace: deps/sqlx-sqlite/sqliterow
