// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn inspect(
    value: std::sync::Arc<TerraneSharedCell<http::HeaderMap<http::HeaderValue>>>,
) {
    {
        let __terrane_owner = value.clone();
        let __terrane_shared_argument_0 = match || -> Result<
            _,
            crate::TerraneForeignError,
        > {
            Ok(
                terrane_int_support::coerce::<
                    usize,
                >(
                        &{
                            let __terrane_owner = value.clone();
                            let __terrane_call_result = __terrane_raised(
                                match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| {
                                        __terrane_owner
                                            .lock()
                                            .expect("shared reference lock poisoned")
                                            .len()
                                    }),
                                ) {
                                    Ok(value) => {
                                        Ok(terrane_int_support::Int::from_u128(value as u128))
                                    }
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "http",
                                                "http::HeaderMap::len",
                                            ),
                                        )
                                    }
                                },
                                0 /* terrane-site: src/main.trn:5:21-5:31 */,
                            );
                            __terrane_call_result
                        },
                    )
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        };
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_native_input_0 = &mut __terrane_owner
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_native_input_1 = __terrane_shared_argument_1;
        let __terrane_call_result = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = __terrane_native_input_0.reserve(__terrane_native_input_1);
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "http",
                            "http::HeaderMap::reserve",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:5:5-5:32 */,
        );
        __terrane_call_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = value.clone();
        let __terrane_call_result = __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | __terrane_owner.lock()
        .expect("shared reference lock poisoned").len())) { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "http", "http::HeaderMap::len")) },
        2 /* terrane-site: src/main.trn:6:13-6:23 */); __terrane_call_result })
    );
}
fn main() {
    let headers: std::sync::Arc<TerraneSharedCell<http::HeaderMap<http::HeaderValue>>>;
    let held: std::sync::Arc<TerraneSharedCell<http::HeaderMap<http::HeaderValue>>>;
    let alias: std::sync::Arc<TerraneSharedCell<http::HeaderMap<http::HeaderValue>>>;
    let observer: std::sync::Weak<TerraneSharedCell<http::HeaderMap<http::HeaderValue>>>;
    headers = std::sync::Arc::new(
        TerraneSharedCell::new({
            let mut headers = ::http::HeaderMap::new();
            headers.insert("x-value", ::http::HeaderValue::from_static("one"));
            headers
        }),
    );
    held = headers.clone();
    {
        let __terrane_owner = held.clone();
        let __terrane_shared_argument_0 = match || -> Result<
            _,
            crate::TerraneForeignError,
        > {
            Ok(
                terrane_int_support::coerce::<
                    usize,
                >(
                        &({
                            let __terrane_owner = held.clone();
                            let __terrane_call_result = __terrane_raised(
                                match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| {
                                        __terrane_owner
                                            .lock()
                                            .expect("shared reference lock poisoned")
                                            .len()
                                    }),
                                ) {
                                    Ok(value) => {
                                        Ok(terrane_int_support::Int::from_u128(value as u128))
                                    }
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "http",
                                                "http::HeaderMap::len",
                                            ),
                                        )
                                    }
                                },
                                3 /* terrane-site: src/main.trn:14:20-14:29 */,
                            );
                            __terrane_call_result
                        } + terrane_int_support::Int::from(1_i128)),
                    )
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        };
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_native_input_0 = &mut __terrane_owner
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_native_input_1 = __terrane_shared_argument_1;
        let __terrane_call_result = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = __terrane_native_input_0.reserve(__terrane_native_input_1);
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "http",
                            "http::HeaderMap::reserve",
                        ),
                    )
                }
            },
            4 /* terrane-site: src/main.trn:14:5-14:34 */,
        );
        __terrane_call_result
    };
    alias = held.clone();
    {
        let __terrane_owner = alias.clone();
        let __terrane_shared_argument_0 = match || -> Result<
            _,
            crate::TerraneForeignError,
        > {
            Ok(
                terrane_int_support::coerce::<
                    usize,
                >(
                        &{
                            let __terrane_owner = held.clone();
                            let __terrane_call_result = __terrane_raised(
                                match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| {
                                        __terrane_owner
                                            .lock()
                                            .expect("shared reference lock poisoned")
                                            .len()
                                    }),
                                ) {
                                    Ok(value) => {
                                        Ok(terrane_int_support::Int::from_u128(value as u128))
                                    }
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "http",
                                                "http::HeaderMap::len",
                                            ),
                                        )
                                    }
                                },
                                5 /* terrane-site: src/main.trn:16:21-16:30 */,
                            );
                            __terrane_call_result
                        },
                    )
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        };
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_native_input_0 = &mut __terrane_owner
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_native_input_1 = __terrane_shared_argument_1;
        let __terrane_call_result = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = __terrane_native_input_0.reserve(__terrane_native_input_1);
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "http",
                            "http::HeaderMap::reserve",
                        ),
                    )
                }
            },
            6 /* terrane-site: src/main.trn:16:5-16:31 */,
        );
        __terrane_call_result
    };
    observer = std::sync::Arc::downgrade(&headers.clone());
    {
        let __terrane_owner = observer.upgrade().expect("reference expired");
        let __terrane_shared_argument_0 = match || -> Result<
            _,
            crate::TerraneForeignError,
        > {
            Ok(
                terrane_int_support::coerce::<
                    usize,
                >(
                        &{
                            let __terrane_owner = alias.clone();
                            let __terrane_call_result = __terrane_raised(
                                match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| {
                                        __terrane_owner
                                            .lock()
                                            .expect("shared reference lock poisoned")
                                            .len()
                                    }),
                                ) {
                                    Ok(value) => {
                                        Ok(terrane_int_support::Int::from_u128(value as u128))
                                    }
                                    Err(payload) => {
                                        Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "http",
                                                "http::HeaderMap::len",
                                            ),
                                        )
                                    }
                                },
                                7 /* terrane-site: src/main.trn:18:24-18:34 */,
                            );
                            __terrane_call_result
                        },
                    )
                    .map_err(|error| crate::TerraneForeignError(
                        crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                    ))?,
            )
        }() {
            Ok(value) => value,
            Err(error) => std::panic::panic_any(error),
        };
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_native_input_0 = &mut __terrane_owner
            .lock()
            .expect("reference lock poisoned");
        let __terrane_native_input_1 = __terrane_shared_argument_1;
        let __terrane_call_result = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = __terrane_native_input_0.reserve(__terrane_native_input_1);
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "http",
                            "http::HeaderMap::reserve",
                        ),
                    )
                }
            },
            8 /* terrane-site: src/main.trn:18:5-18:35 */,
        );
        __terrane_call_result
    };
    inspect(held.clone());
    {
        let __terrane_owner = observer.upgrade().expect("reference expired");
        let __terrane_call_result = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = __terrane_owner
                        .lock()
                        .expect("reference lock poisoned")
                        .clear();
                }),
            ) {
                Ok(()) => Ok(()),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "http",
                            "http::HeaderMap::clear",
                        ),
                    )
                }
            },
            9 /* terrane-site: src/main.trn:20:5-20:20 */,
        );
        __terrane_call_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_owner = held.clone();
        let __terrane_call_result = __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | __terrane_owner.lock()
        .expect("shared reference lock poisoned").len())) { Ok(value) =>
        Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "http", "http::HeaderMap::len")) },
        10 /* terrane-site: src/main.trn:21:13-21:22 */); __terrane_call_result })
    );
}
