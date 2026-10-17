// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: callbacks/header.trn
// Namespace: callbacks
fn mark_sensitive(header: &mut HeaderValue) -> bool {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = header.set_sensitive(true);
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::set_sensitive",
                    ),
                )
            }
        },
        0 /* terrane-site: callbacks/header.trn:6:5-6:31 */,
    );
    return true;
}
fn clear_sensitive(header: &mut HeaderValue) -> bool {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = header.set_sensitive(false);
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::set_sensitive",
                    ),
                )
            }
        },
        1 /* terrane-site: callbacks/header.trn:10:5-10:32 */,
    );
    return true;
}
fn read_sensitive(header: &HeaderValue) -> bool {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&header).is_sensitive()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::is_sensitive",
                    ),
                )
            }
        },
        2 /* terrane-site: callbacks/header.trn:14:12-14:32 */,
    );
}
fn clear_sensitive_optional(header: &mut HeaderValue) -> Option<bool> {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = header.set_sensitive(false);
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "http",
                        "http::HeaderValue::set_sensitive",
                    ),
                )
            }
        },
        3 /* terrane-site: callbacks/header.trn:17:5-17:32 */,
    );
    return None;
}
// Source: src/main.trn
// Namespace: app
fn entry_value(entry: &Entry) -> terrane_int_support::Int {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&entry).value()),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Entry::value",
                    ),
                )
            }
        },
        4 /* terrane-site: src/main.trn:9:12-9:24 */,
    );
}
fn request_view<'view>(request: &'view Request) -> witness::View<'view> {
    return {
        let __terrane_native_input_0 = &(&request).extensions().entry();
        __terrane_raised(
            make_view(__terrane_native_input_0),
            5 /* terrane-site: src/main.trn:12:12-12:53 */,
        )
    };
}
fn observe_header(
    action: std::sync::Arc<dyn Fn(&HeaderValue) -> bool + Send + Sync>,
    header: &HeaderValue,
) -> bool {
    return action(header);
}
fn main() {
    let request: Request;
    let selected: Option<Entry>;
    let mapped: Option<terrane_int_support::Int>;
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
    request = __terrane_raised(
        terrane_static_trn_52657175657374_new(terrane_int_support::Int::from(41_i128)),
        6 /* terrane-site: src/main.trn:18:15-18:31 */,
    );
    (&request).extensions();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&request).extensions()
        .entry().value())) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "witness", "witness::Entry::value")) },
        7 /* terrane-site: src/main.trn:20:13-20:50 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        &(&request).extensions().entry();
        __terrane_raised(consume(__terrane_native_input_0), 8 /* terrane-site: src/main.trn:21:13-21:52 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&request).headers()
        .length())) { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Header::length")) }, 9 /* terrane-site: src/main.trn:22:13-22:39 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        10 /* terrane-site: src/main.trn:23:13-23:25 */))
    );
    selected = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&request).extensions().get::<witness::Entry>().cloned()
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "(((& request) . extensions ()) . get :: < witness :: Entry > ()) . cloned",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:24:27-24:63 */,
    );
    if selected.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &selected {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .value())) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::Entry::value")) }, 12 /* terrane-site: src/main.trn:26:17-26:32 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        13 /* terrane-site: src/main.trn:27:13-27:25 */))
    );
    mapped = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&request)
                    .extensions()
                    .get::<witness::Entry>()
                    .map(move |borrowed_argument_0| {
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
                    })
            }),
        ) {
            Ok(value) => {
                Ok(value.map(|value| terrane_int_support::Int::from(i128::from(value))))
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "(((& request) . extensions ()) . get :: < witness :: Entry > ()) . map",
                    ),
                )
            }
        },
        14 /* terrane-site: src/main.trn:28:23-28:68 */,
    );
    if mapped.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &mapped { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        15 /* terrane-site: src/main.trn:31:13-31:25 */))
    );
    host = __terrane_raised(
        terrane_static_trn_486f7374_new(),
        16 /* terrane-site: src/main.trn:32:12-32:22 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = host
                    .base_mut()
                    .draw_circle(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<
                                    i64,
                                >(&terrane_int_support::Int::from(7_i128))
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    );
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Canvas::draw_circle",
                    ),
                )
            }
        },
        17 /* terrane-site: src/main.trn:33:5-33:36 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = host
                    .base_mut()
                    .draw_circle(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<
                                    i64,
                                >(&terrane_int_support::Int::from(11_i128))
                                    .map_err(|error| crate::TerraneForeignError(
                                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                    ))?,
                            )
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    );
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Canvas::draw_circle",
                    ),
                )
            }
        },
        18 /* terrane-site: src/main.trn:34:5-34:37 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&host).total())) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Host::total")) }, 19 /* terrane-site: src/main.trn:35:13-35:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
        (&__terrane_raised(terrane_static_trn_52657175657374_new(terrane_int_support::Int::from(43_i128)),
        20 /* terrane-site: src/main.trn:36:16-36:32 */)).extensions().entry()
        .value())) { Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => Err(crate ::__terrane_dependency_panic(payload, "witness",
        "witness::Entry::value")) }, 21 /* terrane-site: src/main.trn:36:13-36:61 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(root_count(),
        22 /* terrane-site: src/main.trn:37:13-37:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(render(&request,
        request_view), 23 /* terrane-site: src/main.trn:38:13-38:42 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(clone_count(),
        24 /* terrane-site: src/main.trn:39:13-39:25 */))
    );
    headers = __terrane_raised(
        terrane_static_trn_4865616465724d6170_new(),
        25 /* terrane-site: src/main.trn:41:15-41:30 */,
    );
    missing = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = mark_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        26 /* terrane-site: src/main.trn:42:25-42:75 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&missing.is_none()));
    name = __terrane_raised(
        terrane_static_trn_4865616465724e616d65_from_str(String::from("forwarded")),
        27 /* terrane-site: src/main.trn:44:12-44:45 */,
    );
    value = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str(String::from("initial")),
        28 /* terrane-site: src/main.trn:45:13-45:45 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = headers.insert(name, value);
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "(headers) . insert",
                    ),
                )
            }
        },
        29 /* terrane-site: src/main.trn:46:5-46:32 */,
    );
    before = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&headers)
                    .get(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = read_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((& headers) . get (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        30 /* terrane-site: src/main.trn:47:24-47:70 */,
    );
    if before.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &before { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    marked = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = mark_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        31 /* terrane-site: src/main.trn:50:24-50:74 */,
    );
    if marked.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &marked { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    anonymous_reset = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = clear_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        32 /* terrane-site: src/main.trn:53:33-53:84 */,
    );
    if anonymous_reset.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &anonymous_reset {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    anonymous_update = {
        std::sync::Arc::new(move |header: &mut HeaderValue| -> bool {
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| {
                        let _ = header.set_sensitive(true);
                    }),
                ) {
                    Ok(()) => Ok(()),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "http",
                                "http::HeaderValue::set_sensitive",
                            ),
                        )
                    }
                },
                33 /* terrane-site: src/main.trn:57:9-57:35 */,
            );
            return true;
        })
    };
    anonymous_result = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok({
                                let callback = anonymous_update.clone();
                                move |callback_argument_0: &mut http::HeaderValue| {
                                    match || -> Result<_, crate::TerraneForeignError> {
                                        let callback_value = callback(&mut *callback_argument_0);
                                        Ok(callback_value)
                                    }() {
                                        Ok(value) => value,
                                        Err(error) => std::panic::panic_any(error.0),
                                    }
                                }
                            })
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    )
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        34 /* terrane-site: src/main.trn:59:34-59:86 */,
    );
    if anonymous_result.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &anonymous_result {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    anonymous_observed = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&headers)
                    .get(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = read_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((& headers) . get (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        35 /* terrane-site: src/main.trn:62:36-62:82 */,
    );
    if anonymous_observed.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &anonymous_observed {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    snapshot = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers.get_mut(String::from("forwarded")).cloned()
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . cloned",
                    ),
                )
            }
        },
        36 /* terrane-site: src/main.trn:65:33-65:71 */,
    );
    cleared = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = clear_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        37 /* terrane-site: src/main.trn:66:25-66:76 */,
    );
    if cleared.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &cleared { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    after = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&headers)
                    .get(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = read_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((& headers) . get (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        38 /* terrane-site: src/main.trn:69:23-69:69 */,
    );
    anonymous_read = {
        std::sync::Arc::new(move |header: &HeaderValue| -> bool {
            return __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&header).is_sensitive()),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "http",
                                "http::HeaderValue::is_sensitive",
                            ),
                        )
                    }
                },
                39 /* terrane-site: src/main.trn:71:16-71:36 */,
            );
        })
    };
    anonymous_read_result = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&headers)
                    .get(String::from("forwarded"))
                    .map(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok({
                                let callback = anonymous_read.clone();
                                move |callback_argument_0: &http::HeaderValue| {
                                    match || -> Result<_, crate::TerraneForeignError> {
                                        let callback_value = callback(&*callback_argument_0);
                                        Ok(callback_value)
                                    }() {
                                        Ok(value) => value,
                                        Err(error) => std::panic::panic_any(error.0),
                                    }
                                }
                            })
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    )
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((& headers) . get (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        40 /* terrane-site: src/main.trn:72:39-72:85 */,
    );
    if anonymous_read_result.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &anonymous_read_result {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    mutable_read = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .map(
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok({
                                let callback = anonymous_read.clone();
                                move |callback_argument_0: &mut http::HeaderValue| {
                                    match || -> Result<_, crate::TerraneForeignError> {
                                        let callback_value = callback(&mut *callback_argument_0);
                                        Ok(callback_value)
                                    }() {
                                        Ok(value) => value,
                                        Err(error) => std::panic::panic_any(error.0),
                                    }
                                }
                            })
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error),
                        },
                    )
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        41 /* terrane-site: src/main.trn:75:30-75:80 */,
    );
    if mutable_read.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &mutable_read {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    if after.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &after { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    if snapshot.is_some() {
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = match &mut snapshot {
                        Some(value) => value,
                        _ => unreachable!("flow-proven storage refinement"),
                    }
                        .set_sensitive(false);
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "http",
                            "http::HeaderValue::set_sensitive",
                        ),
                    )
                }
            },
            42 /* terrane-site: src/main.trn:81:9-81:38 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | match &snapshot {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") }
            .is_sensitive())) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "http",
            "http::HeaderValue::is_sensitive")) }, 43 /* terrane-site: src/main.trn:82:17-82:39 */))
        );
    }
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = headers
                    .get_mut(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = mark_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    });
            }),
        ) {
            Ok(()) => Ok(()),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        44 /* terrane-site: src/main.trn:83:5-83:55 */,
    );
    empty = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("forwarded"))
                    .and_then(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = clear_sensitive_optional(
                                borrowed_argument_0,
                            );
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((headers) . get_mut (String :: from (\"forwarded\"))) . and_then",
                    ),
                )
            }
        },
        45 /* terrane-site: src/main.trn:84:23-84:88 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&empty.is_none()));
    __trn_66696e616c = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&headers)
                    .get(String::from("forwarded"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = read_sensitive(borrowed_argument_0);
                            Ok(callback_value)
                        }() {
                            Ok(value) => value,
                            Err(error) => std::panic::panic_any(error.0),
                        }
                    })
            }),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "((& headers) . get (String :: from (\"forwarded\"))) . map",
                    ),
                )
            }
        },
        46 /* terrane-site: src/main.trn:86:23-86:69 */,
    );
    if __trn_66696e616c.is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &__trn_66696e616c {
            Some(value) => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    direct = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str(String::from("direct")),
        47 /* terrane-site: src/main.trn:90:14-90:45 */,
    );
    anonymous_update(&mut direct);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&direct)
        .is_sensitive())) { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "http", "http::HeaderValue::is_sensitive"))
        }, 48 /* terrane-site: src/main.trn:92:13-92:33 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&anonymous_read(&direct)));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&observe_header(std::sync::Arc::new(read_sensitive),
        &direct))
    );
    aliased_update = anonymous_update.clone();
    aliased = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str(String::from("aliased")),
        49 /* terrane-site: src/main.trn:96:15-96:47 */,
    );
    aliased_update(&mut aliased);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&aliased)
        .is_sensitive())) { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "http", "http::HeaderValue::is_sensitive"))
        }, 50 /* terrane-site: src/main.trn:98:13-98:34 */))
    );
    named_update = std::sync::Arc::new(mark_sensitive);
    named_target = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str(String::from("named")),
        51 /* terrane-site: src/main.trn:100:20-100:50 */,
    );
    named_update(&mut named_target);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&named_target)
        .is_sensitive())) { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "http", "http::HeaderValue::is_sensitive"))
        }, 52 /* terrane-site: src/main.trn:102:13-102:39 */))
    );
    observers = terrane_collection_support::List::<
        std::sync::Arc<dyn Fn(&HeaderValue) -> bool + Send + Sync>,
    >::new(vec![std::sync::Arc::new(read_sensitive)]);
    let __terrane_iterable_0 = observers.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        observer = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&observer(&named_target)));
    }
    observers.append(std::sync::Arc::new(read_sensitive));
    __terrane_raised(
        observers
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    53 /* terrane-site: src/main.trn:107:5-107:37 */,
                ),
                std::sync::Arc::new(read_sensitive),
            ),
        53 /* terrane-site: src/main.trn:107:5-107:37 */,
    );
    let _ = __terrane_raised(
        observers
            .set(
                __terrane_raised(
                    terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(0_i128),
                    ),
                    54 /* terrane-site: src/main.trn:108:5-108:34 */,
                ),
                std::sync::Arc::new(read_sensitive),
            ),
        54 /* terrane-site: src/main.trn:108:5-108:34 */,
    );
    let __terrane_iterable_1 = observers;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        observer = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &observer;
        println!("{}", terrane_scalar_support::scalar_text(&observer(&named_target)));
    }
}
