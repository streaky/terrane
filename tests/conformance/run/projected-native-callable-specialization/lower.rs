// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn marker_value(value: &Marker) -> terrane_int_support::Int {
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&*value).value()),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::Marker::value",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:6:12-6:24 */,
    );
}
async fn async_identity<TerraneType54>(value: TerraneType54) -> TerraneType54 {
    return value;
}
#[allow(unsafe_code)]
fn main() {
    __terrane_run(async move {
        let marker_cell: witness::Cell<witness::Marker>;
        let factory: Factory;
        let method_cell: witness::Cell<witness::Marker>;
        let defaulted: witness::Cell<witness::Marker>;
        let selected: Option<terrane_int_support::Int>;
        let marker: Marker;
        let mixed_cell: witness::Cell<bool>;
        let inline_byte: witness::Cell<u8>;
        let retained_byte: u8;
        let stored_byte: witness::Cell<u8>;
        let unsafe_cell: witness::Cell<witness::Marker>;
        let unsafe_method_cell: witness::Cell<witness::Marker>;
        let left: Left;
        let right: Right;
        let returned_left: Left;
        let native_int: witness::Cell<i64>;
        let static_default: u16;
        let static_owner: u16;
        let static_collision: String;
        let async_free: witness::Cell<witness::Marker>;
        let async_member: witness::Cell<witness::Marker>;
        let async_free_value: Option<terrane_int_support::Int>;
        let async_member_value: Option<terrane_int_support::Int>;
        let async_authored: String;
        marker_cell = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::make::<witness::Marker>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "dependency",
                            "dependency::ForeignType::make",
                        ),
                    )
                }
            },
            1 /* terrane-site: src/main.trn:12:19-12:36 */,
        );
        factory = __terrane_raised(
            terrane_static_trn_466163746f7279_new(),
            2 /* terrane-site: src/main.trn:13:15-13:28 */,
        );
        method_cell = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&factory).create::<witness::Marker>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Factory::create",
                        ),
                    )
                }
            },
            3 /* terrane-site: src/main.trn:14:19-14:46 */,
        );
        defaulted = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::make::<witness::Marker>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "dependency",
                            "dependency::ForeignType::make",
                        ),
                    )
                }
            },
            4 /* terrane-site: src/main.trn:15:32-15:49 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&marker_cell))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 5 /* terrane-site: src/main.trn:16:13-16:33 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&method_cell))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 6 /* terrane-site: src/main.trn:17:13-17:33 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&defaulted))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 7 /* terrane-site: src/main.trn:18:13-18:31 */))
        );
        selected = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    (&marker_cell)
                        .get()
                        .map(move |borrowed_argument_0| {
                            match || -> Result<_, crate::TerraneForeignError> {
                                let callback_value = marker_value(borrowed_argument_0);
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
                    Ok(
                        value
                            .map(|value| terrane_int_support::Int::from(
                                i128::from(value),
                            )),
                    )
                }
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
            8 /* terrane-site: src/main.trn:19:25-19:61 */,
        );
        if (&selected).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &selected { Some(value)
                => value, _ => unreachable!("flow-proven storage refinement") })
            );
        }
        println!("{}", terrane_scalar_support::scalar_text(&(&selected).is_some()));
        marker = witness::Marker { 0: 7 };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::ordered(marker.clone(), true))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::ordered")) }, 9 /* terrane-site: src/main.trn:24:13-24:54 */))
        );
        mixed_cell = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::mixed(marker.clone())),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "dependency",
                            "dependency::ForeignType::mixed",
                        ),
                    )
                }
            },
            10 /* terrane-site: src/main.trn:25:31-25:64 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&mixed_cell))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 11 /* terrane-site: src/main.trn:26:13-26:32 */))
        );
        inline_byte = {
            let __terrane_native_input_0 = match || -> Result<
                _,
                crate::TerraneForeignError,
            > {
                Ok(
                    terrane_int_support::coerce::<
                        u8,
                    >(
                            &__terrane_raised(
                                byte(),
                                12 /* terrane-site: src/main.trn:27:37-27:42 */,
                            ),
                        )
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error),
            };
            __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| witness::echo(
                        __terrane_native_input_0,
                    )),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "dependency",
                                "dependency::ForeignType::echo",
                            ),
                        )
                    }
                },
                13 /* terrane-site: src/main.trn:27:19-27:43 */,
            )
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&inline_byte))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 14 /* terrane-site: src/main.trn:28:13-28:33 */))
        );
        retained_byte = __terrane_raised(
            terrane_int_support::coerce::<
                u8,
            >(
                &__terrane_raised(
                    byte(),
                    15 /* terrane-site: src/main.trn:29:27-29:32 */,
                ),
            ),
            15 /* terrane-site: src/main.trn:29:27-29:32 */,
        );
        stored_byte = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::echo(retained_byte)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "dependency",
                            "dependency::ForeignType::echo",
                        ),
                    )
                }
            },
            16 /* terrane-site: src/main.trn:30:19-30:49 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&stored_byte))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 17 /* terrane-site: src/main.trn:31:13-31:33 */))
        );
        unsafe_cell = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| unsafe {
                    witness::make_unsafe::<witness::Marker>()
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "dependency",
                            "dependency::ForeignType::make_unsafe",
                        ),
                    )
                }
            },
            18 /* terrane-site: src/main.trn:32:20-32:51 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&unsafe_cell))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 19 /* terrane-site: src/main.trn:33:13-33:33 */))
        );
        unsafe_method_cell = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| unsafe {
                    factory.unsafe_create::<witness::Marker>()
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Factory::unsafe_create",
                        ),
                    )
                }
            },
            20 /* terrane-site: src/main.trn:34:27-34:68 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&unsafe_method_cell))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 21 /* terrane-site: src/main.trn:35:13-35:40 */))
        );
        left = __terrane_raised(
            cycle(),
            22 /* terrane-site: src/main.trn:36:12-36:18 */,
        );
        right = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&left).next()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::left::Left::next",
                        ),
                    )
                }
            },
            23 /* terrane-site: src/main.trn:37:13-37:23 */,
        );
        returned_left = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&right).next()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::right::Right::next",
                        ),
                    )
                }
            },
            24 /* terrane-site: src/main.trn:38:21-38:32 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&returned_left)
            .is_left())) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "witness",
            "witness::left::Left::is_left")) }, 25 /* terrane-site: src/main.trn:39:13-39:35 */))
        );
        native_int = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::make::<i64>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "dependency",
                            "dependency::ForeignType::make",
                        ),
                    )
                }
            },
            26 /* terrane-site: src/main.trn:40:32-40:46 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&native_int))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 27 /* terrane-site: src/main.trn:41:13-41:32 */))
        );
        static_default = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::Holder::<u8>::make::<u16>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Holder::make",
                        ),
                    )
                }
            },
            28 /* terrane-site: src/main.trn:42:22-42:58 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&static_default));
        static_owner = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::Holder::<
                    u8,
                >::with_owner::<u16>(retained_byte)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Holder::with_owner",
                        ),
                    )
                }
            },
            29 /* terrane-site: src/main.trn:44:20-44:76 */,
        );
        static_collision = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::Holder::<u8>::make::<String>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::Holder::make",
                        ),
                    )
                }
            },
            30 /* terrane-site: src/main.trn:45:24-45:60 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&(static_collision.as_str() == ""))
        );
        println!("{}", terrane_scalar_support::scalar_text(&static_owner));
        async_free = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = witness::make_async::<witness::Marker>();
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(value) => Ok(value),
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "dependency",
                                            "dependency::ForeignType::make_async",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            31 /* terrane-site: src/main.trn:48:25-48:48 */,
                        )
                    }
                })
                .await,
            32 /* terrane-site: src/main.trn:48:24-48:49 */,
        );
        async_member = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = (&factory).create_async(marker);
                        Box::pin(async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(value) => Ok(value),
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "witness",
                                            "witness::Factory::create_async",
                                        ),
                                    )
                                }
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            33 /* terrane-site: src/main.trn:49:27-49:67 */,
                        )
                    }
                })
                .await,
            34 /* terrane-site: src/main.trn:49:26-49:68 */,
        );
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&async_free))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 35 /* terrane-site: src/main.trn:50:13-50:32 */)), String::from(" "),
            terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(&async_member))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "dependency",
            "dependency::ForeignType::consume")) }, 36 /* terrane-site: src/main.trn:50:41-50:62 */))
        );
        async_free_value = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    (&async_free)
                        .get()
                        .map(move |borrowed_argument_0| {
                            match || -> Result<_, crate::TerraneForeignError> {
                                let callback_value = marker_value(borrowed_argument_0);
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
                    Ok(
                        value
                            .map(|value| terrane_int_support::Int::from(
                                i128::from(value),
                            )),
                    )
                }
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
            37 /* terrane-site: src/main.trn:51:33-51:68 */,
        );
        if (&async_free_value).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &async_free_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        async_member_value = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    (&async_member)
                        .get()
                        .map(move |borrowed_argument_0| {
                            match || -> Result<_, crate::TerraneForeignError> {
                                let callback_value = marker_value(borrowed_argument_0);
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
                    Ok(
                        value
                            .map(|value| terrane_int_support::Int::from(
                                i128::from(value),
                            )),
                    )
                }
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
            38 /* terrane-site: src/main.trn:54:35-54:72 */,
        );
        if (&async_member_value).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &async_member_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        async_authored = __terrane_await(
                async_identity::<String>(String::from("authored async")),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&async_authored));
    });
}
