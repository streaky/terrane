// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn mark(header: &mut HeaderValue) {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = (*header).set_sensitive(true);
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
        0 /* terrane-site: src/main.trn:5:5-5:31 */,
    );
}
fn forward(header: &mut HeaderValue) {
    let operation: std::sync::Arc<dyn Fn(&mut HeaderValue) -> () + Send + Sync>;
    operation = std::sync::Arc::new(mark);
    operation(&mut *header);
}
fn change(header: &mut HeaderValue) -> bool {
    forward(&mut *header);
    return true;
}
fn clear(header: &mut HeaderValue) -> Option<bool> {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = (*header).set_sensitive(false);
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
        1 /* terrane-site: src/main.trn:16:5-16:32 */,
    );
    return Some(true);
}
fn read(header: &HeaderValue) -> bool {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&*header).is_sensitive()),
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
        2 /* terrane-site: src/main.trn:20:12-20:32 */,
    );
}
fn walk_left(header: &mut HeaderValue, count: terrane_int_support::Int) {
    if count > terrane_int_support::Int::from(0_i128) {
        walk_right(&mut *header, count.clone() - terrane_int_support::Int::from(1_i128));
    } else {
        mark(&mut *header);
    }
}
fn walk_right(header: &mut HeaderValue, count: terrane_int_support::Int) {
    walk_left(&mut *header, count.clone());
}
fn main() {
    let mut plain: HeaderValue;
    let mut headers: http::HeaderMap<http::HeaderValue>;
    let name: HeaderName;
    let value: HeaderValue;
    let result: Option<bool>;
    let marked: Option<bool>;
    let cleared: Option<bool>;
    let after: Option<bool>;
    plain = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("plain"),
        3 /* terrane-site: src/main.trn:32:13-32:43 */,
    );
    forward(&mut plain);
    println!("{}", terrane_scalar_support::scalar_text(&read(&plain)));
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = plain.set_sensitive(false);
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
        4 /* terrane-site: src/main.trn:35:5-35:31 */,
    );
    walk_left(&mut plain, terrane_int_support::Int::from(2_i128));
    println!("{}", terrane_scalar_support::scalar_text(&read(&plain)));
    headers = __terrane_raised(
        terrane_static_trn_4865616465724d6170_new(),
        5 /* terrane-site: src/main.trn:38:15-38:30 */,
    );
    name = __terrane_raised(
        terrane_static_trn_4865616465724e616d65_from_str("x-test"),
        6 /* terrane-site: src/main.trn:39:12-39:42 */,
    );
    value = __terrane_raised(
        terrane_static_trn_48656164657256616c7565_from_str("value"),
        7 /* terrane-site: src/main.trn:40:13-40:43 */,
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
                        "http",
                        "http::HeaderMap::insert",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:41:5-41:32 */,
    );
    result = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("x-test"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = change(borrowed_argument_0);
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
                        "std",
                        "std::option::Option::map",
                    ),
                )
            }
        },
        9 /* terrane-site: src/main.trn:42:24-42:63 */,
    );
    if (&result).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &result { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    marked = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&headers)
                    .get(String::from("x-test"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = read(borrowed_argument_0);
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
                        "std",
                        "std::option::Option::map",
                    ),
                )
            }
        },
        10 /* terrane-site: src/main.trn:45:24-45:57 */,
    );
    if (&marked).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &marked { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    cleared = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                headers
                    .get_mut(String::from("x-test"))
                    .and_then(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = clear(borrowed_argument_0);
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
                        "std",
                        "std::option::Option::and_then",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:48:25-48:68 */,
    );
    if (&cleared).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &cleared { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    after = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&headers)
                    .get(String::from("x-test"))
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = read(borrowed_argument_0);
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
                        "std",
                        "std::option::Option::map",
                    ),
                )
            }
        },
        12 /* terrane-site: src/main.trn:51:23-51:56 */,
    );
    if (&after).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&* match &after { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
}
