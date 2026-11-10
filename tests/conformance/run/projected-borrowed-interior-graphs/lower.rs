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
        4 /* terrane-site: src/main.trn:10:12-10:24 */,
    );
}
fn mapped_entry(request: &Request) -> Option<terrane_int_support::Int> {
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
        5 /* terrane-site: src/main.trn:13:12-15:26 */,
    );
}
fn request_view<'view>(request: &'view Request) -> witness::View<'view> {
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
        6 /* terrane-site: src/main.trn:18:12-18:53 */,
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
        7 /* terrane-site: src/main.trn:27:13-27:32 */,
    );
}
fn main() {
    let request: Request;
    let selected: Option<Entry>;
    let mapped: Option<terrane_int_support::Int>;
    let returned: Option<terrane_int_support::Int>;
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
    request = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(41_i128);
        __terrane_raised(
            terrane_static_trn_52657175657374_new(__terrane_conversion_input_0),
            8 /* terrane-site: src/main.trn:30:15-30:31 */,
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
        (terrane_int_support::Int::from(i128::from(value))) } } (), 9 /* terrane-site: src/main.trn:32:13-32:50 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(consume(&(&request)
        .extensions().entry()), 10 /* terrane-site: src/main.trn:33:13-33:52 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&request).headers().length()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "witness", "witness::Header::length")) }; Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        11 /* terrane-site: src/main.trn:34:13-34:39 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        12 /* terrane-site: src/main.trn:35:13-35:25 */))
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
        13 /* terrane-site: src/main.trn:36:27-36:63 */,
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
            14 /* terrane-site: src/main.trn:38:17-38:32 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        15 /* terrane-site: src/main.trn:39:13-39:25 */))
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
        16 /* terrane-site: src/main.trn:40:23-40:79 */,
    );
    if (&mapped).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &mapped { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        17 /* terrane-site: src/main.trn:43:13-43:25 */))
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
        18 /* terrane-site: src/main.trn:47:13-47:25 */))
    );
    host = __terrane_raised(
        terrane_static_trn_486f7374_new(),
        19 /* terrane-site: src/main.trn:48:12-48:22 */,
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
            20 /* terrane-site: src/main.trn:49:5-49:36 */,
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
            21 /* terrane-site: src/main.trn:50:5-50:37 */,
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
        22 /* terrane-site: src/main.trn:51:13-51:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&{ let __terrane_conversion_input_0 =
        terrane_int_support::Int::from(43_i128);
        __terrane_raised(terrane_static_trn_52657175657374_new(__terrane_conversion_input_0),
        23 /* terrane-site: src/main.trn:52:16-52:32 */) }).extensions().entry()
        .value()) })) { Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::Entry::value")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from(i128::from(value))) } } (),
        24 /* terrane-site: src/main.trn:52:13-52:61 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(root_count(),
        25 /* terrane-site: src/main.trn:53:13-53:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(render(&request,
        request_view), 26 /* terrane-site: src/main.trn:54:13-54:42 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        27 /* terrane-site: src/main.trn:55:13-55:25 */))
    );
    headers = __terrane_raised(
        terrane_static_trn_4865616465724d6170_new(),
        28 /* terrane-site: src/main.trn:57:15-57:30 */,
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
        29 /* terrane-site: src/main.trn:58:25-58:75 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&(&missing).is_none()));
    name = __terrane_raised(
        terrane_static_trn_4865616465724e616d65_from_str("forwarded"),
        30 /* terrane-site: src/main.trn:60:12-60:45 */,
    );
    value = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("initial"),
        31 /* terrane-site: src/main.trn:61:13-61:45 */,
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
            32 /* terrane-site: src/main.trn:62:5-62:32 */,
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
        33 /* terrane-site: src/main.trn:63:24-63:70 */,
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
        34 /* terrane-site: src/main.trn:66:24-66:74 */,
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
        35 /* terrane-site: src/main.trn:69:33-69:84 */,
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
                    36 /* terrane-site: src/main.trn:73:9-73:35 */,
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
        37 /* terrane-site: src/main.trn:75:34-75:86 */,
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
        38 /* terrane-site: src/main.trn:78:36-78:82 */,
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
        39 /* terrane-site: src/main.trn:81:33-81:71 */,
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
        40 /* terrane-site: src/main.trn:82:25-82:76 */,
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
        41 /* terrane-site: src/main.trn:85:23-85:69 */,
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
                42 /* terrane-site: src/main.trn:87:16-87:36 */,
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
        43 /* terrane-site: src/main.trn:88:39-88:85 */,
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
        44 /* terrane-site: src/main.trn:91:30-91:80 */,
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
                45 /* terrane-site: src/main.trn:97:9-97:38 */,
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
            > (value) } } (), 46 /* terrane-site: src/main.trn:98:17-98:39 */))
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
        47 /* terrane-site: src/main.trn:99:5-99:55 */,
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
        48 /* terrane-site: src/main.trn:100:23-100:88 */,
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
        49 /* terrane-site: src/main.trn:102:23-102:69 */,
    );
    if (&__trn_66696e616c).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &__trn_66696e616c {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    direct = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("direct"),
        50 /* terrane-site: src/main.trn:106:14-106:45 */,
    );
    anonymous_update(&mut direct);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&direct).is_sensitive()) })) { Ok(result) => result
        ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload, "http",
        "http::HeaderValue::is_sensitive")) }; Ok:: < _, crate ::TerraneForeignError >
        (value) } } (), 51 /* terrane-site: src/main.trn:108:13-108:33 */))
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
        52 /* terrane-site: src/main.trn:112:15-112:47 */,
    );
    aliased_update(&mut aliased);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&aliased).is_sensitive()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "http", "http::HeaderValue::is_sensitive")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 53 /* terrane-site: src/main.trn:114:13-114:34 */))
    );
    named_update = std::sync::Arc::new(mark_sensitive);
    named_target = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("named"),
        54 /* terrane-site: src/main.trn:116:20-116:50 */,
    );
    named_update(&mut named_target);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&named_target).is_sensitive()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "http", "http::HeaderValue::is_sensitive")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 55 /* terrane-site: src/main.trn:118:13-118:39 */))
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
            56 /* terrane-site: src/main.trn:123:5-123:37 */,
        );
        let __terrane_collection_argument_1 = std::sync::Arc::new(read_sensitive);
        __terrane_raised(
            observers
                .set(__terrane_collection_argument_0, __terrane_collection_argument_1),
            56 /* terrane-site: src/main.trn:123:5-123:37 */,
        );
    };
    {
        let __terrane_index = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            57 /* terrane-site: src/main.trn:124:5-124:34 */,
        );
        let __terrane_value = std::sync::Arc::new(read_sensitive);
        let _ = __terrane_raised(
            observers.set(__terrane_index, __terrane_value),
            57 /* terrane-site: src/main.trn:124:5-124:34 */,
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
        let _ = &observer;
        println!("{}", terrane_scalar_support::scalar_text(&observer(&named_target)));
    }
    root = __terrane_raised(
        root_only(),
        58 /* terrane-site: src/main.trn:127:12-127:22 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&root_value(&mut root)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        59 /* terrane-site: src/main.trn:129:13-129:25 */))
    );
    source_root = TerraneNsAppRootAliasIdentity2F61707000726F6F742D616C696173::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&source_root.value));
}
