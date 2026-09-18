// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_parallel.rs, async_dependency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-string-support
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
    pub static DESCRIPTORS: [&str; 4] = [
        "/core/errors::dependency-error",
        "/core/errors::dependency-panic",
        "/deps/sqlx-sqlite::SqliteAdapterError",
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
    pub static SITES: [Site; 11] = [
        /* terrane-site-row: site 0: /app::main (src/main.trn:8:15-8:41) */
        { Site { function: 0, file: 0, line: 8, column: 15, end_line: 8, end_column: 41 } },
        /* terrane-site-row: site 1: /app::main (src/main.trn:9:15-9:45) */
        { Site { function: 0, file: 0, line: 9, column: 15, end_line: 9, end_column: 45 } },
        /* terrane-site-row: site 2: /app::main (src/main.trn:10:15-10:46) */
        { Site { function: 0, file: 0, line: 10, column: 15, end_line: 10, end_column: 46 } },
        /* terrane-site-row: site 3: /app::main (src/main.trn:11:39-11:55) */
        { Site { function: 0, file: 0, line: 11, column: 39, end_line: 11, end_column: 55 } },
        /* terrane-site-row: site 4: /app::main (src/main.trn:12:12-12:73) */
        { Site { function: 0, file: 0, line: 12, column: 12, end_line: 12, end_column: 73 } },
        /* terrane-site-row: site 5: /app::main (src/main.trn:12:11-12:74) */
        { Site { function: 0, file: 0, line: 12, column: 11, end_line: 12, end_column: 74 } },
        /* terrane-site-row: site 6: /app::main (src/main.trn:13:12-13:90) */
        { Site { function: 0, file: 0, line: 13, column: 12, end_line: 13, end_column: 90 } },
        /* terrane-site-row: site 7: /app::main (src/main.trn:13:11-13:91) */
        { Site { function: 0, file: 0, line: 13, column: 11, end_line: 13, end_column: 91 } },
        /* terrane-site-row: site 8: /app::main (src/main.trn:14:29-14:85) */
        { Site { function: 0, file: 0, line: 14, column: 29, end_line: 14, end_column: 85 } },
        /* terrane-site-row: site 9: /app::main (src/main.trn:15:12-15:32) */
        { Site { function: 0, file: 0, line: 15, column: 12, end_line: 15, end_column: 32 } },
        /* terrane-site-row: site 10: /app::main (src/main.trn:15:11-15:33) */
        { Site { function: 0, file: 0, line: 15, column: 11, end_line: 15, end_column: 33 } },
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
            0 /* terrane-site: src/main.trn:8:15-8:41 */,
        );
        options = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    options.filename(String::from("adapter.db").as_str())
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
            1 /* terrane-site: src/main.trn:9:15-9:45 */,
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
            2 /* terrane-site: src/main.trn:10:15-10:46 */,
        );
        let mut database: SqliteConnection = __terrane_traced(
            __terrane_await({
                    let __terrane_future = connect(&options);
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            3 /* terrane-site: src/main.trn:11:39-11:55 */,
                        )
                    }
                })
                .await,
            3 /* terrane-site: src/main.trn:11:39-11:55 */,
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = execute(
                        &mut database,
                        String::from("CREATE TABLE values (body BLOB NOT NULL)"),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            4 /* terrane-site: src/main.trn:12:12-12:73 */,
                        )
                    }
                })
                .await,
            5 /* terrane-site: src/main.trn:12:11-12:74 */,
        );
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = execute_with_bytes(
                        &mut database,
                        String::from("INSERT INTO values (body) VALUES (?)"),
                        Vec::from([118, 97, 108, 117, 101]),
                    );
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            6 /* terrane-site: src/main.trn:13:12-13:90 */,
                        )
                    }
                })
                .await,
            7 /* terrane-site: src/main.trn:13:11-13:91 */,
        );
        {
            let _: terrane_collection_support::List<Vec<u8>> = __terrane_traced(
                __terrane_await({
                        let __terrane_future = query_bytes(
                            &mut database,
                            String::from("SELECT body FROM values"),
                            String::from("body"),
                        );
                        async move {
                            __terrane_raised_err(
                                __terrane_future.await,
                                8 /* terrane-site: src/main.trn:14:29-14:85 */,
                            )
                        }
                    })
                    .await,
                8 /* terrane-site: src/main.trn:14:29-14:85 */,
            );
        }
        __terrane_traced(
            __terrane_await({
                    let __terrane_future = close(database);
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            9 /* terrane-site: src/main.trn:15:12-15:32 */,
                        )
                    }
                })
                .await,
            10 /* terrane-site: src/main.trn:15:11-15:33 */,
        );
    });
}
// Source: <terrane>/projected/deps/sqlx-sqlite.trn
// Namespace: deps/sqlx-sqlite
pub use sqlx_sqlite::SqliteAutoVacuum;
pub use sqlx_sqlite::SqliteConnectOptions;
pub use sqlx_sqlite::SqliteConnection;
pub use sqlx_sqlite::SqliteJournalMode;
pub use sqlx_sqlite::SqliteLockingMode;
pub use sqlx_sqlite::SqliteSynchronous;
pub use core::time::Duration as TerraneNs4Deps11SqlxSqliteDuration;
pub use core::cmp::Ordering as TerraneNs4Deps11SqlxSqliteOrdering;
pub use std::path::Path as TerraneNs4Deps11SqlxSqlitePath;
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
pub async fn execute(
    connection: &mut SqliteConnection,
    statement: String,
) -> Result<(), crate::TerraneForeignError> {
    let statement = statement;
    match crate::__terrane_dependency_await_unwind(
            terrane_integration_adapters::sqlx_sqlite::execute(connection, statement),
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
                    "terrane_integration_adapters::sqlx_sqlite::execute",
                ),
            )
        }
    }
}
pub async fn execute_with_bytes(
    connection: &mut SqliteConnection,
    statement: String,
    value: Vec<u8>,
) -> Result<(), crate::TerraneForeignError> {
    let statement = statement;
    let value = value;
    match crate::__terrane_dependency_await_unwind(
            terrane_integration_adapters::sqlx_sqlite::execute_with_bytes(
                connection,
                statement,
                value,
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
                    "terrane_integration_adapters::sqlx_sqlite::execute_with_bytes",
                ),
            )
        }
    }
}
pub async fn query_bytes(
    connection: &mut SqliteConnection,
    statement: String,
    column: String,
) -> Result<terrane_collection_support::List<Vec<u8>>, crate::TerraneForeignError> {
    let statement = statement;
    let column = column;
    match crate::__terrane_dependency_await_unwind(
            terrane_integration_adapters::sqlx_sqlite::query_bytes(
                connection,
                statement,
                column,
            ),
        )
        .await
    {
        Ok(Ok(value)) => Ok(terrane_collection_support::List::new(value)),
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
                    "terrane_integration_adapters::sqlx_sqlite::query_bytes",
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
                        crate::DescriptorId(3),
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
                        crate::DescriptorId(3),
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
