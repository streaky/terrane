// Rust justification: ABI boundary to the WHATWG URL support layer.

fn terrane_url_limit(value: &terrane_int_support::Int) -> usize { value.as_usize().unwrap_or(usize::MAX) }
fn terrane_url_parse(input: String, base: String) -> terrane_document_support::UrlResult { terrane_document_support::parse_url(&input, &base) }
fn terrane_url_failed(result: &terrane_document_support::UrlResult) -> bool { result.failed }
fn terrane_url_message(result: &terrane_document_support::UrlResult) -> String { result.message.clone() }
fn terrane_url_serialized(result: &terrane_document_support::UrlResult) -> String { result.serialized.clone() }
fn terrane_url_display(result: &terrane_document_support::UrlResult) -> String { result.display.clone() }
fn terrane_url_scheme(result: &terrane_document_support::UrlResult) -> String { result.scheme.clone() }
fn terrane_url_username(result: &terrane_document_support::UrlResult) -> String { result.username.clone() }
fn terrane_url_password(result: &terrane_document_support::UrlResult) -> String { result.password.clone() }
fn terrane_url_host(result: &terrane_document_support::UrlResult) -> String { result.host.clone() }
fn terrane_url_port(result: &terrane_document_support::UrlResult) -> String { result.port.clone() }
fn terrane_url_path(result: &terrane_document_support::UrlResult) -> String { result.path.clone() }
fn terrane_url_query_length(result: &terrane_document_support::UrlResult) -> terrane_int_support::Int {
    terrane_int_support::Int::from(i128::try_from(terrane_document_support::url_query_length(result)).expect("query length fits in i128"))
}
fn terrane_url_query_key(result: &terrane_document_support::UrlResult, index: terrane_int_support::Int) -> String { terrane_document_support::url_query_key(result, terrane_url_limit(&index)) }
fn terrane_url_query_value(result: &terrane_document_support::UrlResult, index: terrane_int_support::Int) -> String { terrane_document_support::url_query_value(result, terrane_url_limit(&index)) }
fn terrane_url_fragment(result: &terrane_document_support::UrlResult) -> String { result.fragment.clone() }
fn terrane_url_origin(result: &terrane_document_support::UrlResult) -> String { result.origin.clone() }
