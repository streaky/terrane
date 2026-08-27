// Generated deterministically by Terrane <version>.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    fn from_source_name(name: &str) -> Self {
        match name {
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    message: String,
    cause: Option<Box<TerraneError>>,
    context: Vec<&'static str>,
}
impl TerraneError {
    fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            cause: None,
            context: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn at(mut self, frame: &'static str) -> Self {
        self.context.push(frame);
        self
    }
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
        if let Some(cause) = &self.cause {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        for frame in &self.context {
            rendered.push_str("\nat ");
            rendered.push_str(frame);
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
impl From<terrane_int_support::ArithmeticError> for TerraneError {
    fn from(error: terrane_int_support::ArithmeticError) -> Self {
        Self::new(
            TerraneErrorKind::from_source_name(error.source_name()),
            error.to_string(),
        )
    }
}
impl From<terrane_string_support::DecodeError> for TerraneError {
    fn from(error: terrane_string_support::DecodeError) -> Self {
        Self::new(
            TerraneErrorKind::DecodeError,
            error.to_string().trim_start_matches(".decode-error: "),
        )
    }
}
impl From<terrane_collection_support::IndexError> for TerraneError {
    fn from(error: terrane_collection_support::IndexError) -> Self {
        Self::new(TerraneErrorKind::IndexError, error.to_string())
    }
}
impl From<terrane_collection_support::MissingKey> for TerraneError {
    fn from(error: terrane_collection_support::MissingKey) -> Self {
        Self::new(TerraneErrorKind::MissingKey, error.to_string())
    }
}
impl From<terrane_collection_support::RangeStepError> for TerraneError {
    fn from(error: terrane_collection_support::RangeStepError) -> Self {
        Self::new(TerraneErrorKind::SourceError, error.to_string())
    }
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
fn terrane_url_limit(value: &terrane_int_support::Int) -> usize {
    value.as_usize().unwrap_or(usize::MAX)
}
fn terrane_url_parse(
    input: String,
    base: String,
) -> terrane_document_support::UrlResult {
    terrane_document_support::parse_url(&input, &base)
}
fn terrane_url_failed(result: &terrane_document_support::UrlResult) -> bool {
    result.failed
}
fn terrane_url_message(result: &terrane_document_support::UrlResult) -> String {
    result.message.clone()
}
fn terrane_url_serialized(result: &terrane_document_support::UrlResult) -> String {
    result.serialized.clone()
}
fn terrane_url_display(result: &terrane_document_support::UrlResult) -> String {
    result.display.clone()
}
fn terrane_url_scheme(result: &terrane_document_support::UrlResult) -> String {
    result.scheme.clone()
}
fn terrane_url_username(result: &terrane_document_support::UrlResult) -> String {
    result.username.clone()
}
fn terrane_url_password(result: &terrane_document_support::UrlResult) -> String {
    result.password.clone()
}
fn terrane_url_host(result: &terrane_document_support::UrlResult) -> String {
    result.host.clone()
}
fn terrane_url_port(result: &terrane_document_support::UrlResult) -> String {
    result.port.clone()
}
fn terrane_url_path(result: &terrane_document_support::UrlResult) -> String {
    result.path.clone()
}
fn terrane_url_query_length(
    result: &terrane_document_support::UrlResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(
        i128::try_from(terrane_document_support::url_query_length(result))
            .expect("query length fits in i128"),
    )
}
fn terrane_url_query_key(
    result: &terrane_document_support::UrlResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_document_support::url_query_key(result, terrane_url_limit(&index))
}
fn terrane_url_query_value(
    result: &terrane_document_support::UrlResult,
    index: terrane_int_support::Int,
) -> String {
    terrane_document_support::url_query_value(result, terrane_url_limit(&index))
}
fn terrane_url_fragment(result: &terrane_document_support::UrlResult) -> String {
    result.fragment.clone()
}
fn terrane_url_origin(result: &terrane_document_support::UrlResult) -> String {
    result.origin.clone()
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
// Source: standard/urls.trn
// Namespace: standard/urls
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
            if self
                .keys
                .get_or_error(
                    terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/urls::get (urls.trn:29:16)"),
                        )),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error).at("/standard/urls::get (urls.trn:29:16)"),
                )) == name
            {
                return self
                    .values
                    .get_or_error(
                        terrane_collection_support::index_from_int(&index.clone())
                            .unwrap_or_else(|error| __terrane_uncaught(
                                TerraneError::from(error)
                                    .at("/standard/urls::get (urls.trn:30:24)"),
                            )),
                    )
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at("/standard/urls::get (urls.trn:30:24)"),
                    ));
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
            if self
                .keys
                .get_or_error(
                    terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/urls::get-all (urls.trn:38:16)"),
                        )),
                )
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/urls::get-all (urls.trn:38:16)"),
                )) == name
            {
                result
                    .append(
                        self
                            .values
                            .get_or_error(
                                terrane_collection_support::index_from_int(&index.clone())
                                    .unwrap_or_else(|error| __terrane_uncaught(
                                        TerraneError::from(error)
                                            .at("/standard/urls::get-all (urls.trn:39:32)"),
                                    )),
                            )
                            .unwrap_or_else(|error| __terrane_uncaught(
                                TerraneError::from(error)
                                    .at("/standard/urls::get-all (urls.trn:39:32)"),
                            )),
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
            empty_query.clone(),
            String::from(""),
            String::from(""),
        );
        return UrlResult::terrane_construct(
            true,
            terrane_url_message(&raw),
            empty_url.clone(),
        );
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
    return UrlResult::terrane_construct(false, String::from(""), value.clone());
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
