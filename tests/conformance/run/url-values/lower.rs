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
    pub static FILES: [&str; 1] = ["core/urls.trn"];
    pub static FUNCTIONS: [&str; 2] = ["/core/urls::get", "/core/urls::get-all"];
    pub static SITES: [Site; 4] = [
        {
            /* terrane-site-row: site 0: /core/urls::get (core/urls.trn:23:16-23:32) */
            Site {
                function: 0,
                file: 0,
                line: 23,
                column: 16,
                end_line: 23,
                end_column: 32,
            }
        },
        {
            /* terrane-site-row: site 1: /core/urls::get (core/urls.trn:24:24-24:42) */
            Site {
                function: 0,
                file: 0,
                line: 24,
                column: 24,
                end_line: 24,
                end_column: 42,
            }
        },
        {
            /* terrane-site-row: site 2: /core/urls::get-all (core/urls.trn:32:16-32:32) */
            Site {
                function: 1,
                file: 0,
                line: 32,
                column: 16,
                end_line: 32,
                end_column: 32,
            }
        },
        {
            /* terrane-site-row: site 3: /core/urls::get-all (core/urls.trn:33:32-33:50) */
            Site {
                function: 1,
                file: 0,
                line: 33,
                column: 32,
                end_line: 33,
                end_column: 50,
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
pub fn terrane_url_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(usize::MAX)
}
pub fn terrane_url_parse(
    input: String,
    base: String,
) -> terrane_document_support::UrlResult {
    terrane_document_support::parse_url(&input, &base)
}
pub fn terrane_url_failed(result: &terrane_document_support::UrlResult) -> bool {
    result.failed
}
pub fn terrane_url_message(result: &terrane_document_support::UrlResult) -> String {
    result.message.clone()
}
pub fn terrane_url_serialized(result: &terrane_document_support::UrlResult) -> String {
    result.serialized.clone()
}
pub fn terrane_url_display(result: &terrane_document_support::UrlResult) -> String {
    result.display.clone()
}
pub fn terrane_url_scheme(result: &terrane_document_support::UrlResult) -> String {
    result.scheme.clone()
}
pub fn terrane_url_username(result: &terrane_document_support::UrlResult) -> String {
    result.username.clone()
}
pub fn terrane_url_password(result: &terrane_document_support::UrlResult) -> String {
    result.password.clone()
}
pub fn terrane_url_host(result: &terrane_document_support::UrlResult) -> String {
    result.host.clone()
}
pub fn terrane_url_port(result: &terrane_document_support::UrlResult) -> String {
    result.port.clone()
}
pub fn terrane_url_path(result: &terrane_document_support::UrlResult) -> String {
    result.path.clone()
}
pub fn terrane_url_query_length(
    result: &terrane_document_support::UrlResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        i128::try_from(terrane_document_support::url_query_length(result))
            .expect("query length fits in i128"),
    )
}
pub fn terrane_url_query_key(
    result: &terrane_document_support::UrlResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_document_support::url_query_key(result, terrane_url_limit(&index))
}
pub fn terrane_url_query_value(
    result: &terrane_document_support::UrlResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_document_support::url_query_value(result, terrane_url_limit(&index))
}
pub fn terrane_url_fragment(result: &terrane_document_support::UrlResult) -> String {
    result.fragment.clone()
}
pub fn terrane_url_origin(result: &terrane_document_support::UrlResult) -> String {
    result.origin.clone()
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct TerraneDescriptor {
    identity: &'static str,
    name: &'static str,
    kind: &'static str,
}
// Source: case.trn
// Namespace: conformance/url-values
fn main() {
    let parsed: UrlResult = parse_url(
        String::from("https://user:pass@bücher.example:443/a?x=1&x=2#f"),
    );
    println!("{}", terrane_scalar_support::scalar_text(&parsed.failed));
    let value: Url = parsed.value;
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&value.scheme),
        terrane_scalar_support::scalar_text(&value.host),
        terrane_scalar_support::scalar_text(&value.port),
        terrane_scalar_support::scalar_text(&value.path)
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.string()));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.username),
        terrane_scalar_support::scalar_text(&value.password)
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.query.count));
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.query.get(String::from("x"))),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(value.query
        .get_all(String::from("x")).length()))
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.fragment),
        terrane_scalar_support::scalar_text(&value.origin)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value.serialized
        .contains(&String::from("user:pass@"))),
        terrane_scalar_support::scalar_text(&value.display
        .contains(&String::from("user:pass@")))
    );
    let relative: UrlResult = value.resolve(String::from("../b?q=hello%20world"));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&relative.value.host),
        terrane_scalar_support::scalar_text(&relative.value.path),
        terrane_scalar_support::scalar_text(&relative.value.query.get(String::from("q")))
    );
}
// Source: core/urls.trn
// Namespace: core/urls
#[derive(Clone)]
pub struct UrlQuery {
    pub keys: terrane_collection_support::List<String>,
    pub values: terrane_collection_support::List<String>,
    pub count: terrane_int_support::Int,
}
impl UrlQuery {
    pub fn terrane_construct() -> Self {
        let mut value = Self {
            keys: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            values: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
            count: terrane_int_support::Int::from(0_i128),
        };
        value.construct();
        value
    }
    pub fn construct(&mut self) {
        let keys: terrane_collection_support::List<String> = terrane_collection_support::List::<
            String,
        >::new(vec![]);
        let values: terrane_collection_support::List<String> = terrane_collection_support::List::<
            String,
        >::new(vec![]);
        self.keys = keys.clone();
        self.values = values.clone();
        self.count = terrane_int_support::Int::from(0_i128);
    }
    pub fn length(&self) -> terrane_int_support::Int {
        return self.count.clone();
    }
    pub fn get(&self, name: String) -> String {
        let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(self.keys.length()),
            )
        {
            if __terrane_raised(
                self
                    .keys
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            0 /* terrane-site: core/urls.trn:23:16-23:32 */,
                        ),
                    ),
                0 /* terrane-site: core/urls.trn:23:16-23:32 */,
            ) == name
            {
                return __terrane_raised(
                    self
                        .values
                        .get_or_error(
                            __terrane_raised(
                                terrane_collection_support::index_from_int(&index.clone()),
                                1 /* terrane-site: core/urls.trn:24:24-24:42 */,
                            ),
                        ),
                    1 /* terrane-site: core/urls.trn:24:24-24:42 */,
                );
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
        return String::from("");
    }
    pub fn get_all(&self, name: String) -> terrane_collection_support::List<String> {
        let mut result: terrane_collection_support::List<String> = terrane_collection_support::List::<
            String,
        >::new(vec![]);
        let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        while index.clone()
            < terrane_int_support::Int::from(
                terrane_int_support::Int::from(self.keys.length()),
            )
        {
            if __terrane_raised(
                self
                    .keys
                    .get_or_error(
                        __terrane_raised(
                            terrane_collection_support::index_from_int(&index.clone()),
                            2 /* terrane-site: core/urls.trn:32:16-32:32 */,
                        ),
                    ),
                2 /* terrane-site: core/urls.trn:32:16-32:32 */,
            ) == name
            {
                result
                    .append(
                        __terrane_raised(
                            self
                                .values
                                .get_or_error(
                                    __terrane_raised(
                                        terrane_collection_support::index_from_int(&index.clone()),
                                        3 /* terrane-site: core/urls.trn:33:32-33:50 */,
                                    ),
                                ),
                            3 /* terrane-site: core/urls.trn:33:32-33:50 */,
                        ),
                    );
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
        return result.clone();
    }
}
pub fn append_query_entry(mut query: UrlQuery, name: String, value: String) -> UrlQuery {
    query.count = query.count.clone() + terrane_int_support::Int::from(1_i128);
    query.keys.append(name);
    query.values.append(value);
    return query.clone();
}
#[derive(Clone)]
pub struct Url {
    pub serialized: String,
    pub display: String,
    pub scheme: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub path: String,
    pub query: UrlQuery,
    pub fragment: String,
    pub origin: String,
}
impl Url {
    pub fn terrane_construct(
        serialized: String,
        display: String,
        scheme: String,
        username: String,
        password: String,
        host: String,
        port: String,
        path: String,
        query: UrlQuery,
        fragment: String,
        origin: String,
    ) -> Self {
        let mut value = Self {
            serialized: String::from(""),
            display: String::from(""),
            scheme: String::from(""),
            username: String::from(""),
            password: String::from(""),
            host: String::from(""),
            port: String::from(""),
            path: String::from(""),
            query: UrlQuery::terrane_construct(),
            fragment: String::from(""),
            origin: String::from(""),
        };
        value
            .construct(
                serialized,
                display,
                scheme,
                username,
                password,
                host,
                port,
                path,
                query,
                fragment,
                origin,
            );
        value
    }
    pub fn construct(
        &mut self,
        serialized: String,
        display: String,
        scheme: String,
        username: String,
        password: String,
        host: String,
        port: String,
        path: String,
        query: UrlQuery,
        fragment: String,
        origin: String,
    ) {
        self.serialized = serialized;
        self.display = display;
        self.scheme = scheme;
        self.username = username;
        self.password = password;
        self.host = host;
        self.port = port;
        self.path = path;
        self.query = query.clone();
        self.fragment = fragment;
        self.origin = origin;
    }
    pub fn string(&self) -> String {
        return self.display.clone();
    }
    pub fn resolve(&self, relative: String) -> UrlResult {
        return parse_url_relative(relative, self.clone());
    }
}
#[derive(Clone)]
pub struct UrlResult {
    pub failed: bool,
    pub message: String,
    pub value: Url,
}
impl UrlResult {
    pub fn terrane_construct(failed: bool, message: String, parsed_url: Url) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            value: Url::terrane_construct(
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                UrlQuery::terrane_construct(),
                String::from(""),
                String::from(""),
            ),
        };
        value.construct(failed, message, parsed_url);
        value
    }
    pub fn construct(&mut self, failed: bool, message: String, parsed_url: Url) {
        self.failed = failed;
        self.message = message;
        self.value = parsed_url.clone();
    }
}
pub fn url_from_platform(raw: terrane_document_support::UrlResult) -> UrlResult {
    let failed: bool = terrane_url_failed(&raw);
    if failed {
        let empty_query: UrlQuery = UrlQuery::terrane_construct();
        let empty_url: Url = Url::terrane_construct(
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            empty_query,
            String::from(""),
            String::from(""),
        );
        return UrlResult::terrane_construct(true, terrane_url_message(&raw), empty_url);
    }
    let mut query: UrlQuery = UrlQuery::terrane_construct();
    let count: terrane_int_support::Int = terrane_url_query_length(&raw);
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < count.clone() {
        query = append_query_entry(
            query.clone(),
            terrane_url_query_key(&raw, index.clone()),
            terrane_url_query_value(&raw, index.clone()),
        );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let value: Url = Url::terrane_construct(
        terrane_url_serialized(&raw),
        terrane_url_display(&raw),
        terrane_url_scheme(&raw),
        terrane_url_username(&raw),
        terrane_url_password(&raw),
        terrane_url_host(&raw),
        terrane_url_port(&raw),
        terrane_url_path(&raw),
        query.clone(),
        terrane_url_fragment(&raw),
        terrane_url_origin(&raw),
    );
    return UrlResult::terrane_construct(false, String::from(""), value);
}
pub fn parse_url(input: String) -> UrlResult {
    let raw: terrane_document_support::UrlResult = terrane_url_parse(
        input,
        String::from(""),
    );
    return url_from_platform(raw);
}
pub fn parse_url_relative(input: String, base: Url) -> UrlResult {
    let raw: terrane_document_support::UrlResult = terrane_url_parse(
        input,
        base.serialized,
    );
    return url_from_platform(raw);
}
