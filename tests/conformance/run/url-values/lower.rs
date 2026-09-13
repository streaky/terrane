// Generated deterministically by Terrane <version>.
// Runtime support: platform_urls.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-document-support
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
        {
            let __terrane_list_append_0 = result.make_unique();
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
                    __terrane_list_append_0
                        .push(
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
