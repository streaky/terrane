// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: callbacks/header.trn
// Namespace: callbacks
fn mark_sensitive(header: &mut HeaderValue) -> bool {
    {
        let __terrane_conversion_input_0 = true;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = (*header)
                                    .set_sensitive(__terrane_conversion_input_0);
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "http",
                                    "http::HeaderValue::set_sensitive",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            0 /* terrane-site: callbacks/header.trn:6:5-6:31 */,
        )
    };
    return true;
}
fn clear_sensitive(header: &mut HeaderValue) -> bool {
    {
        let __terrane_conversion_input_0 = false;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = (*header)
                                    .set_sensitive(__terrane_conversion_input_0);
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "http",
                                    "http::HeaderValue::set_sensitive",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            1 /* terrane-site: callbacks/header.trn:10:5-10:32 */,
        )
    };
    return true;
}
fn read_sensitive(header: &HeaderValue) -> bool {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*header).is_sensitive()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "http",
                                "http::HeaderValue::is_sensitive",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        2 /* terrane-site: callbacks/header.trn:14:12-14:32 */,
    );
}
fn clear_sensitive_optional(header: &mut HeaderValue) -> Option<bool> {
    {
        let __terrane_conversion_input_0 = false;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = (*header)
                                    .set_sensitive(__terrane_conversion_input_0);
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "http",
                                    "http::HeaderValue::set_sensitive",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            3 /* terrane-site: callbacks/header.trn:17:5-17:32 */,
        )
    };
    return None;
}
// Source: src/main.trn
// Namespace: app
fn entry_value(entry: &Entry) -> terrane_int_support::Int {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*entry).value()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Entry::value",
                            ),
                        );
                    }
                };
                Ok::<
                    _,
                    crate::TerraneForeignError,
                >(terrane_int_support::Int::from(i128::from(value)))
            }
        }(),
        4 /* terrane-site: src/main.trn:16:12-16:24 */,
    );
}
fn mapped_entry(
    request: &TerraneNsDepsWitnessRequestIdentity2F646570732F7769746E6573730052657175657374,
) -> Option<terrane_int_support::Int> {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*request)
                                .extensions()
                                .get::<witness::Entry>()
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = entry_value(borrowed_argument_0);
                                            Ok(
                                                terrane_int_support::coerce::<i64>(&callback_value)
                                                    .map_err(|error| crate::TerraneForeignError(
                                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                                    ))?,
                                            )
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<
                    _,
                    crate::TerraneForeignError,
                >(value.map(|value| terrane_int_support::Int::from(i128::from(value))))
            }
        }(),
        5 /* terrane-site: src/main.trn:19:12-21:26 */,
    );
}
fn request_view<'view>(
    request: &'view TerraneNsDepsWitnessRequestIdentity2F646570732F7769746E6573730052657175657374,
) -> witness::View<'view> {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(witness::make_view(&(&*request).extensions().entry())) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::make_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        6 /* terrane-site: src/main.trn:24:12-24:53 */,
    );
}
fn observe_header(
    action: std::sync::Arc<dyn Fn(&HeaderValue) -> bool + Send + Sync>,
    header: &HeaderValue,
) -> bool {
    return action(header);
}
#[derive(Clone)]
pub struct TerraneNsAppRootAliasIdentity2F61707000726F6F742D616C696173 {
    pub value: terrane_int_support::Int,
}
impl TerraneNsAppRootAliasIdentity2F61707000726F6F742D616C696173 {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(79_i128),
        }
    }
}
fn root_value(
    root: &mut TerraneNsDepsWitnessRootAliasIdentity2F646570732F7769746E65737300526F6F74416C696173,
) -> terrane_int_support::Int {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*root).view().value()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "witness",
                                "witness::Entry::value",
                            ),
                        );
                    }
                };
                Ok::<
                    _,
                    crate::TerraneForeignError,
                >(terrane_int_support::Int::from(i128::from(value)))
            }
        }(),
        7 /* terrane-site: src/main.trn:33:13-33:32 */,
    );
}
fn http_request_body(
    request: &mut TerraneNsDepsHttpRequestIdentity2F646570732F687474700052657175657374<
        String,
    >,
) -> String {
    let value: String;
    value = request.body().clone();
    return value;
}
fn http_response_body(
    response: &mut TerraneNsDepsHttpResponseIdentity2F646570732F6874747000526573706F6E7365<
        String,
    >,
) -> String {
    let value: String;
    value = response.body().clone();
    return value;
}
fn default_http_method(
    request: &mut TerraneNsDepsAxumCoreExtractRequestIdentity2F646570732F6178756D2D636F72652F657874726163740052657175657374,
) -> String {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*request).method().as_str()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "http",
                                "http::Method::as_str",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),
        8 /* terrane-site: src/main.trn:46:13-46:38 */,
    );
}
fn default_http_status(
    response: &mut TerraneNsDepsAxumCoreResponseResponseIdentity2F646570732F6178756D2D636F72652F726573706F6E736500526573706F6E7365,
) -> Result<u16, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                u16,
            >(
                &{
                    let __terrane_receiver_value_0 = &__terrane_raised_err(
                        || -> Result<_, crate::TerraneForeignError> {
                            {
                                let value = match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| -> Result<
                                        _,
                                        crate::TerraneForeignError,
                                    > { Ok((&*response).status()) }),
                                ) {
                                    Ok(result) => result?,
                                    Err(payload) => {
                                        return Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "axum_core",
                                                "axum_core::response::Response::status",
                                            ),
                                        );
                                    }
                                };
                                Ok::<_, crate::TerraneForeignError>(value)
                            }
                        }(),
                        9 /* terrane-site: src/main.trn:49:14-49:30 */,
                    )?;
                    let __terrane_native_input_0 = &__terrane_receiver_value_0;
                    __terrane_raised_err(
                        || -> Result<_, crate::TerraneForeignError> {
                            {
                                let value = match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| -> Result<
                                        _,
                                        crate::TerraneForeignError,
                                    > { Ok(__terrane_native_input_0.as_u16()) }),
                                ) {
                                    Ok(result) => result?,
                                    Err(payload) => {
                                        return Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "http",
                                                "http::StatusCode::as_u16",
                                            ),
                                        );
                                    }
                                };
                                Ok::<
                                    _,
                                    crate::TerraneForeignError,
                                >(terrane_int_support::Int::from_u128(value as u128))
                            }
                        }(),
                        10 /* terrane-site: src/main.trn:49:13-49:39 */,
                    )?
                },
            ),
            10 /* terrane-site: src/main.trn:49:13-49:39 */,
        )?,
    );
}
fn http_peer_ip(peer: &axum::extract::ConnectInfo<std::net::SocketAddr>) -> String {
    let payload: NativeSocketAddress;
    let value: String;
    payload = {
        let __terrane_snapshot = peer.0.clone();
        __terrane_snapshot
    };
    value = payload.ip().to_string();
    return value;
}
fn default_http_peer(request: &http::Request<axum_core::body::Body>) -> Option<String> {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*request)
                                .extensions()
                                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = http_peer_ip(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        11 /* terrane-site: src/main.trn:58:12-60:27 */,
    );
}
fn main() {
    let request: TerraneNsDepsWitnessRequestIdentity2F646570732F7769746E6573730052657175657374;
    let selected: Option<Entry>;
    let mapped: Option<terrane_int_support::Int>;
    let returned: Option<terrane_int_support::Int>;
    let inferred: Option<terrane_int_support::Int>;
    let mut host: Host;
    let mut headers: http::HeaderMap<http::HeaderValue>;
    let missing: Option<bool>;
    let name: HeaderName;
    let value: HeaderValue;
    let before: Option<bool>;
    let marked: Option<bool>;
    let anonymous_reset: Option<bool>;
    let anonymous_update: std::sync::Arc<dyn Fn(&mut HeaderValue) -> bool + Send + Sync>;
    let anonymous_result: Option<bool>;
    let anonymous_observed: Option<bool>;
    let mut snapshot: Option<HeaderValue>;
    let cleared: Option<bool>;
    let after: Option<bool>;
    let anonymous_read: std::sync::Arc<dyn Fn(&HeaderValue) -> bool + Send + Sync>;
    let anonymous_read_result: Option<bool>;
    let mutable_read: Option<bool>;
    let empty: Option<bool>;
    let __trn_66696e616c: Option<bool>;
    let mut direct: HeaderValue;
    let aliased_update: std::sync::Arc<dyn Fn(&mut HeaderValue) -> bool + Send + Sync>;
    let mut aliased: HeaderValue;
    let named_update: std::sync::Arc<dyn Fn(&mut HeaderValue) -> bool + Send + Sync>;
    let mut named_target: HeaderValue;
    let mut observers: terrane_collection_support::List<
        std::sync::Arc<dyn Fn(&HeaderValue) -> bool + Send + Sync>,
    >;
    let mut observer: std::sync::Arc<dyn Fn(&HeaderValue) -> bool + Send + Sync>;
    let mut root: witness::RootOnly<witness::Entry>;
    let source_root: TerraneNsAppRootAliasIdentity2F61707000726F6F742D616C696173;
    let mut http_request: http::Request<String>;
    let mut http_response: http::Response<String>;
    let mut default_request: http::Request<axum_core::body::Body>;
    let mut default_response: http::Response<axum_core::body::Body>;
    request = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(41_i128);
        __terrane_raised(
            terrane_static_trn_54657272616e654e73446570735769746e657373526571756573744964656e74697479324636343635373037333246373736393734364536353733373330303532363537313735363537333734_new(
                __terrane_conversion_input_0,
            ),
            12 /* terrane-site: src/main.trn:63:15-63:31 */,
        )
    };
    (&request).extensions();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&request).extensions().entry().value()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::Entry::value")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        13 /* terrane-site: src/main.trn:65:13-65:50 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(consume(&(&request)
        .extensions().entry()), 14 /* terrane-site: src/main.trn:66:13-66:52 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&request).headers().length()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "witness", "witness::Header::length")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        15 /* terrane-site: src/main.trn:67:13-67:39 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        16 /* terrane-site: src/main.trn:68:13-68:25 */))
    );
    selected = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&request).extensions().get::<witness::Entry>().cloned()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::cloned",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        17 /* terrane-site: src/main.trn:69:27-69:63 */,
    );
    if (&selected).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(match &selected { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") } .value()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "witness", "witness::Entry::value")) };
            Ok:: < _, crate ::TerraneForeignError >
            (terrane_int_support::Int::from(i128::from(value))) } } (),
            18 /* terrane-site: src/main.trn:71:17-71:32 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        19 /* terrane-site: src/main.trn:72:13-72:25 */))
    );
    mapped = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&request)
                                .extensions()
                                .get::<witness::Entry>()
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = entry_value(borrowed_argument_0);
                                            Ok(
                                                terrane_int_support::coerce::<i64>(&callback_value)
                                                    .map_err(|error| crate::TerraneForeignError(
                                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                                    ))?,
                                            )
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<
                    _,
                    crate::TerraneForeignError,
                >(value.map(|value| terrane_int_support::Int::from(i128::from(value))))
            }
        }(),
        20 /* terrane-site: src/main.trn:73:23-73:79 */,
    );
    if (&mapped).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &mapped { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        21 /* terrane-site: src/main.trn:76:13-76:25 */))
    );
    returned = mapped_entry(&request);
    if (&returned).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &returned { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        22 /* terrane-site: src/main.trn:80:13-80:25 */))
    );
    inferred = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&request)
                                .extensions()
                                .get::<witness::Entry>()
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = entry_value(borrowed_argument_0);
                                            Ok(
                                                terrane_int_support::coerce::<i64>(&callback_value)
                                                    .map_err(|error| crate::TerraneForeignError(
                                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                                    ))?,
                                            )
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<
                    _,
                    crate::TerraneForeignError,
                >(value.map(|value| terrane_int_support::Int::from(i128::from(value))))
            }
        }(),
        23 /* terrane-site: src/main.trn:81:25-81:70 */,
    );
    if (&inferred).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &inferred { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    host = __terrane_raised(
        terrane_static_trn_486f7374_new(),
        24 /* terrane-site: src/main.trn:84:12-84:22 */,
    );
    {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(7_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = host
                                    .base_mut()
                                    .draw_circle(
                                        terrane_int_support::coerce::<
                                            i64,
                                        >(&__terrane_conversion_input_0)
                                            .map_err(|error| crate::TerraneForeignError(
                                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                            ))?,
                                    );
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Canvas::draw_circle",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            25 /* terrane-site: src/main.trn:85:5-85:36 */,
        )
    };
    {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(11_i128);
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = host
                                    .base_mut()
                                    .draw_circle(
                                        terrane_int_support::coerce::<
                                            i64,
                                        >(&__terrane_conversion_input_0)
                                            .map_err(|error| crate::TerraneForeignError(
                                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                            ))?,
                                    );
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "witness",
                                    "witness::Canvas::draw_circle",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            26 /* terrane-site: src/main.trn:86:5-86:37 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&host).total()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Host::total")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        27 /* terrane-site: src/main.trn:87:13-87:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(43_i128);
        __terrane_raised(terrane_static_trn_54657272616e654e73446570735769746e657373526571756573744964656e74697479324636343635373037333246373736393734364536353733373330303532363537313735363537333734_new(__terrane_conversion_input_0),
        28 /* terrane-site: src/main.trn:88:16-88:32 */) }).extensions().entry()
        .value()) })) { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::Entry::value")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        29 /* terrane-site: src/main.trn:88:13-88:61 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(root_count(),
        30 /* terrane-site: src/main.trn:89:13-89:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(render(&request,
        request_view), 31 /* terrane-site: src/main.trn:90:13-90:42 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        32 /* terrane-site: src/main.trn:91:13-91:25 */))
    );
    headers = __terrane_raised(
        terrane_static_trn_4865616465724d6170_new(),
        33 /* terrane-site: src/main.trn:93:15-93:30 */,
    );
    missing = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            headers
                                .get_mut(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = mark_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        34 /* terrane-site: src/main.trn:94:25-94:75 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&missing).is_none()));
    name = __terrane_raised(
        terrane_static_trn_4865616465724e616d65_from_str("forwarded"),
        35 /* terrane-site: src/main.trn:96:12-96:45 */,
    );
    value = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("initial"),
        36 /* terrane-site: src/main.trn:97:13-97:45 */,
    );
    {
        let __terrane_conversion_input_0 = name;
        let __terrane_conversion_input_1 = value;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = headers
                                    .insert(
                                        __terrane_conversion_input_0,
                                        __terrane_conversion_input_1,
                                    );
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "http",
                                    "http::HeaderMap::insert",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),
            37 /* terrane-site: src/main.trn:98:5-98:32 */,
        )
    };
    before = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&headers)
                                .get(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = read_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        38 /* terrane-site: src/main.trn:99:24-99:70 */,
    );
    if (&before).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &before { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    marked = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            headers
                                .get_mut(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = mark_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        39 /* terrane-site: src/main.trn:102:24-102:74 */,
    );
    if (&marked).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &marked { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    anonymous_reset = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            headers
                                .get_mut(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = clear_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        40 /* terrane-site: src/main.trn:105:33-105:84 */,
    );
    if (&anonymous_reset).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &anonymous_reset {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    anonymous_update = {
        std::sync::Arc::new(move |header: &mut HeaderValue| -> bool {
            {
                let __terrane_conversion_input_0 = true;
                __terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let _ = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > {
                                    Ok({
                                        let _ = (*header)
                                            .set_sensitive(__terrane_conversion_input_0);
                                    })
                                }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "http",
                                            "http::HeaderValue::set_sensitive",
                                        ),
                                    );
                                }
                            };
                            Ok::<_, crate::TerraneForeignError>(())
                        }
                    }(),
                    41 /* terrane-site: src/main.trn:109:9-109:35 */,
                )
            };
            return true;
        })
    };
    anonymous_result = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            headers
                                .get_mut(String::from("forwarded"))
                                .map({
                                    let borrowed_callback = &anonymous_update;
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = borrowed_callback(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        42 /* terrane-site: src/main.trn:111:34-111:86 */,
    );
    if (&anonymous_result).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &anonymous_result {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    anonymous_observed = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&headers)
                                .get(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = read_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        43 /* terrane-site: src/main.trn:114:36-114:82 */,
    );
    if (&anonymous_observed).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &anonymous_observed {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    snapshot = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(headers.get_mut(String::from("forwarded")).cloned()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::cloned",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        44 /* terrane-site: src/main.trn:117:33-117:71 */,
    );
    cleared = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            headers
                                .get_mut(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = clear_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        45 /* terrane-site: src/main.trn:118:25-118:76 */,
    );
    if (&cleared).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &cleared { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    after = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&headers)
                                .get(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = read_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        46 /* terrane-site: src/main.trn:121:23-121:69 */,
    );
    anonymous_read = {
        std::sync::Arc::new(move |header: &HeaderValue| -> bool {
            return __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&*header).is_sensitive()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "http",
                                        "http::HeaderValue::is_sensitive",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),
                47 /* terrane-site: src/main.trn:123:16-123:36 */,
            );
        })
    };
    anonymous_read_result = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&headers)
                                .get(String::from("forwarded"))
                                .map({
                                    let borrowed_callback = &anonymous_read;
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = borrowed_callback(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        48 /* terrane-site: src/main.trn:124:39-124:85 */,
    );
    if (&anonymous_read_result).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &anonymous_read_result {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    mutable_read = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            headers
                                .get_mut(String::from("forwarded"))
                                .map({
                                    let borrowed_callback = &anonymous_read;
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = borrowed_callback(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        49 /* terrane-site: src/main.trn:127:30-127:80 */,
    );
    if (&mutable_read).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &mutable_read {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    if (&after).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &after { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    if (&snapshot).is_some() {
        {
            let __terrane_conversion_input_0 = false;
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let _ = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok({
                                    let _ = match &mut snapshot {
                                        Some(value) => value,
                                        _ => unreachable!("flow-proven storage refinement"),
                                    }
                                        .set_sensitive(__terrane_conversion_input_0);
                                })
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "http",
                                        "http::HeaderValue::set_sensitive",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(())
                    }
                }(),
                50 /* terrane-site: src/main.trn:133:9-133:38 */,
            )
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(match &snapshot { Some(value) => value, _
            => unreachable!("flow-proven storage refinement") } .is_sensitive()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "http",
            "http::HeaderValue::is_sensitive")) }; Ok:: < _, crate ::TerraneForeignError
            > (value) } } (), 51 /* terrane-site: src/main.trn:134:17-134:39 */))
        );
    }
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = headers
                                .get_mut(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = mark_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                });
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),
        52 /* terrane-site: src/main.trn:135:5-135:55 */,
    );
    empty = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            headers
                                .get_mut(String::from("forwarded"))
                                .and_then({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = clear_sensitive_optional(
                                                borrowed_argument_0,
                                            );
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::and_then",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        53 /* terrane-site: src/main.trn:136:23-136:88 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&empty).is_none()));
    __trn_66696e616c = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&headers)
                                .get(String::from("forwarded"))
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = read_sensitive(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                }),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::option::Option::map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        54 /* terrane-site: src/main.trn:138:23-138:69 */,
    );
    if (&__trn_66696e616c).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &__trn_66696e616c {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    direct = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("direct"),
        55 /* terrane-site: src/main.trn:142:14-142:45 */,
    );
    anonymous_update(&mut direct);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&direct).is_sensitive()) })) { Ok(result) => result
        ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "http",
        "http::HeaderValue::is_sensitive")) }; Ok:: < _, crate ::TerraneForeignError >
        (value) } } (), 56 /* terrane-site: src/main.trn:144:13-144:33 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&anonymous_read(&direct)));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&observe_header(std::sync::Arc::new(read_sensitive),
        &direct))
    );
    aliased_update = anonymous_update.clone();
    aliased = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("aliased"),
        57 /* terrane-site: src/main.trn:148:15-148:47 */,
    );
    aliased_update(&mut aliased);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&aliased).is_sensitive()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "http", "http::HeaderValue::is_sensitive")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 58 /* terrane-site: src/main.trn:150:13-150:34 */))
    );
    named_update = std::sync::Arc::new(mark_sensitive);
    named_target = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("named"),
        59 /* terrane-site: src/main.trn:152:20-152:50 */,
    );
    named_update(&mut named_target);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&named_target).is_sensitive()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "http", "http::HeaderValue::is_sensitive")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 60 /* terrane-site: src/main.trn:154:13-154:39 */))
    );
    observers = terrane_collection_support::List::<
        std::sync::Arc<dyn Fn(&HeaderValue) -> bool + Send + Sync>,
    >::new(vec![std::sync::Arc::new(read_sensitive)]);
    let __terrane_iterable_0 = observers.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        observer = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&observer(&named_target)));
    }
    {
        let __terrane_collection_argument_0 = std::sync::Arc::new(read_sensitive);
        observers.append(__terrane_collection_argument_0);
    };
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            61 /* terrane-site: src/main.trn:159:5-159:37 */,
        );
        let __terrane_collection_argument_1 = std::sync::Arc::new(read_sensitive);
        __terrane_raised(
            observers
                .set(__terrane_collection_argument_0, __terrane_collection_argument_1),
            61 /* terrane-site: src/main.trn:159:5-159:37 */,
        );
    };
    {
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            62 /* terrane-site: src/main.trn:160:5-160:34 */,
        );
        let __terrane_value = std::sync::Arc::new(read_sensitive);
        let _ = __terrane_raised(
            observers.set(__terrane_index, __terrane_value),
            62 /* terrane-site: src/main.trn:160:5-160:34 */,
        );
    }
    let __terrane_iterable_1 = observers;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        observer = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&observer(&named_target)));
    }
    root = __terrane_raised(
        root_only(),
        63 /* terrane-site: src/main.trn:163:12-163:22 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&root_value(&mut root)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        64 /* terrane-site: src/main.trn:165:13-165:25 */))
    );
    source_root = TerraneNsAppRootAliasIdentity2F61707000726F6F742D616C696173::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&source_root.value));
    http_request = ::http::Request::new(String::from("request-body"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&http_request_body(&mut http_request))
    );
    http_response = ::http::Response::new(String::from("response-body"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&http_response_body(&mut
        http_response))
    );
    default_request = {
        let __terrane_conversion_input_0 = __terrane_raised(
            terrane_static_trn_426f6479_empty(),
            65 /* terrane-site: src/main.trn:174:49-174:65 */,
        );
        __terrane_raised(
            terrane_static_trn_54657272616e654e73446570734178756d436f726545787472616374526571756573744964656e746974793246363436353730373332463631373837353644324436333646373236353246363537383734373236313633373430303532363537313735363537333734_new::<
                axum_core::body::Body,
            >(__terrane_conversion_input_0),
            66 /* terrane-site: src/main.trn:174:23-174:66 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&default_http_method(&mut
        default_request))
    );
    default_response = {
        let __terrane_conversion_input_0 = __terrane_raised(
            terrane_static_trn_426f6479_empty(),
            67 /* terrane-site: src/main.trn:176:51-176:67 */,
        );
        __terrane_raised(
            terrane_static_trn_54657272616e654e73446570734178756d436f7265526573706f6e7365526573706f6e73654964656e74697479324636343635373037333246363137383735364432443633364637323635324637323635373337303646364537333635303035323635373337303646364537333635_new::<
                axum_core::body::Body,
            >(__terrane_conversion_input_0),
            68 /* terrane-site: src/main.trn:176:24-176:68 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced(default_http_status(&mut
        default_response), 69 /* terrane-site: src/main.trn:177:13-177:50 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&default_http_peer(&default_request)
        .is_none())
    );
}
