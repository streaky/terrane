// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: native-deref-receivers
fn bump(handle: &mut RecordHandle) {
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = <receiver_api::Record>::set(
                    &mut *handle,
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(19_i128))
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
                        "receiver_api",
                        "receiver_api::RecordHandle::set",
                    ),
                )
            }
        },
        0 /* terrane-site: src/main.trn:7:5-7:19 */,
    );
}
fn ordinary_view() -> terrane_int_support::Int {
    let owner: RecordHandle;
    let view: receiver_api::RecordView<'_>;
    let copied: receiver_api::RecordView<'_>;
    let nested: receiver_api::RecordView<'_>;
    owner = __terrane_raised(
        wrapped(terrane_int_support::Int::from(31_i128)),
        1 /* terrane-site: src/main.trn:10:13-10:24 */,
    );
    view = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&owner).view()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver_api",
                        "receiver_api::RecordHandle::view",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:11:12-11:23 */,
    );
    copied = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <receiver_api::RecordView<
                '_,
            > as core::clone::Clone>::clone(&view)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver_api",
                        "receiver_api::RecordView::clone",
                    ),
                )
            }
        },
        3 /* terrane-site: src/main.trn:12:14-12:25 */,
    );
    nested = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| copied.nested()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver_api",
                        "receiver_api::RecordView::nested",
                    ),
                )
            }
        },
        4 /* terrane-site: src/main.trn:13:14-13:28 */,
    );
    return __terrane_raised(
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| nested.value())) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver_api",
                        "receiver_api::RecordView::value",
                    ),
                )
            }
        },
        5 /* terrane-site: src/main.trn:14:12-14:25 */,
    );
}
fn ordinary_guard() -> terrane_int_support::Int {
    let locks: Locks;
    let mut guard: std::sync::MutexGuard<'_, receiver_api::Record>;
    locks = __terrane_raised(
        terrane_static_trn_4c6f636b73_new(terrane_int_support::Int::from(32_i128)),
        6 /* terrane-site: src/main.trn:17:13-17:27 */,
    );
    guard = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&locks).locked()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver_api",
                        "receiver_api::Locks::locked",
                    ),
                )
            }
        },
        7 /* terrane-site: src/main.trn:18:13-18:26 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = <receiver_api::Record>::set(
                    &mut guard,
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(33_i128))
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
                        "std",
                        "std::sync::MutexGuard::set",
                    ),
                )
            }
        },
        8 /* terrane-site: src/main.trn:19:5-19:18 */,
    );
    return __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <receiver_api::Record>::value(&guard)),
        ) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "std",
                        "std::sync::MutexGuard::value",
                    ),
                )
            }
        },
        9 /* terrane-site: src/main.trn:20:12-20:24 */,
    );
}
async fn suspended_view() -> terrane_int_support::Int {
    let owner: RecordHandle;
    let view: receiver_api::RecordView<'_>;
    owner = __terrane_raised(
        wrapped(terrane_int_support::Int::from(41_i128)),
        10 /* terrane-site: src/main.trn:23:13-23:24 */,
    );
    view = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&owner).view()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver_api",
                        "receiver_api::RecordHandle::view",
                    ),
                )
            }
        },
        11 /* terrane-site: src/main.trn:24:12-24:23 */,
    );
    __terrane_await(
            terrane_time_sleep_after(
                __terrane_traced(
                        Duration::terrane_static_milliseconds(
                            terrane_int_support::Int::from(1_i128),
                        ),
                        12 /* terrane-site: src/main.trn:25:26-25:51 */,
                    )
                    .total_nanoseconds
                    .clone(),
            ),
        )
        .await;
    return __terrane_raised(
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| view.value())) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver_api",
                        "receiver_api::RecordView::value",
                    ),
                )
            }
        },
        13 /* terrane-site: src/main.trn:26:12-26:23 */,
    );
}
fn named_view() -> terrane_int_support::Int {
    let mut ignored: RecordHandle;
    let owner: RecordHandle;
    let view: receiver_api::RecordView<'_>;
    ignored = __terrane_raised(
        wrapped(terrane_int_support::Int::from(50_i128)),
        14 /* terrane-site: src/main.trn:29:15-29:26 */,
    );
    owner = __terrane_raised(
        wrapped(terrane_int_support::Int::from(51_i128)),
        15 /* terrane-site: src/main.trn:30:13-30:24 */,
    );
    view = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| receiver_api::view_from(&ignored, &owner)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver-api",
                        "receiver_api::view_from",
                    ),
                )
            }
        },
        16 /* terrane-site: src/main.trn:31:12-31:55 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = <receiver_api::Record>::set(
                    &mut ignored,
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(52_i128))
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
                        "receiver_api",
                        "receiver_api::RecordHandle::set",
                    ),
                )
            }
        },
        17 /* terrane-site: src/main.trn:32:5-32:20 */,
    );
    return __terrane_raised(
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| view.value())) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "receiver_api",
                        "receiver_api::RecordView::value",
                    ),
                )
            }
        },
        18 /* terrane-site: src/main.trn:33:12-33:23 */,
    );
}
fn pointers_and_guards() {
    let shared: ArcOfReceiverApiRecord;
    let twin: ArcOfReceiverApiRecord;
    let local: RcOfReceiverApiRecord;
    let mut owned: BoxOfReceiverApiRecord;
    let mut custom: RecordHandle;
    let locks: Locks;
    let view_value: terrane_int_support::Int;
    shared = __terrane_raised(
        arc(terrane_int_support::Int::from(7_i128)),
        19 /* terrane-site: src/main.trn:36:14-36:20 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::value(&shared))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "std", "std::sync::Arc::value")) },
        20 /* terrane-site: src/main.trn:37:13-37:26 */))
    );
    twin = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| <std::sync::Arc<
                receiver_api::Record,
            > as core::clone::Clone>::clone(&shared)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "std",
                        "std::sync::Arc::clone",
                    ),
                )
            }
        },
        21 /* terrane-site: src/main.trn:38:12-38:25 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(same_arc(&shared,
        &twin), 22 /* terrane-site: src/main.trn:39:13-39:35 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        &__terrane_raised(arc(terrane_int_support::Int::from(8_i128)),
        23 /* terrane-site: src/main.trn:40:14-40:20 */); __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::value(__terrane_native_input_0))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "std", "std::sync::Arc::value")) },
        24 /* terrane-site: src/main.trn:40:13-40:28 */) })
    );
    local = __terrane_raised(
        rc(terrane_int_support::Int::from(9_i128)),
        25 /* terrane-site: src/main.trn:41:13-41:18 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::value(&local))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "std", "std::rc::Rc::value")) },
        26 /* terrane-site: src/main.trn:42:13-42:25 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        &__terrane_raised(chain(terrane_int_support::Int::from(10_i128)),
        27 /* terrane-site: src/main.trn:43:14-43:23 */); __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::value(__terrane_native_input_0))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "std", "std::sync::Arc::value")) },
        28 /* terrane-site: src/main.trn:43:13-43:31 */) })
    );
    owned = __terrane_raised(
        boxed(terrane_int_support::Int::from(11_i128)),
        29 /* terrane-site: src/main.trn:44:13-44:22 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = <receiver_api::Record>::set(
                    &mut owned,
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(12_i128))
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
                        "std",
                        "std::boxed::Box::set",
                    ),
                )
            }
        },
        30 /* terrane-site: src/main.trn:45:5-45:18 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::value(&owned))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "std", "std::boxed::Box::value")) },
        31 /* terrane-site: src/main.trn:46:13-46:25 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::take(* owned))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "std", "std::boxed::Box::take")) },
        32 /* terrane-site: src/main.trn:47:13-47:24 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(boxed(terrane_int_support::Int::from(13_i128)),
        33 /* terrane-site: src/main.trn:48:14-48:23 */); __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::boxed(__terrane_native_input_0))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "std", "std::boxed::Box::boxed")) },
        34 /* terrane-site: src/main.trn:48:13-48:31 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 = * *
        __terrane_raised(nested_box(terrane_int_support::Int::from(14_i128)),
        35 /* terrane-site: src/main.trn:49:14-49:28 */); __terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::take(__terrane_native_input_0))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "std", "std::boxed::Box::take")) },
        36 /* terrane-site: src/main.trn:49:13-49:35 */) })
    );
    custom = __terrane_raised(
        wrapped(terrane_int_support::Int::from(15_i128)),
        37 /* terrane-site: src/main.trn:50:14-50:25 */,
    );
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = <receiver_api::Record>::set(
                    &mut custom,
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(16_i128))
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
                        "receiver_api",
                        "receiver_api::RecordHandle::set",
                    ),
                )
            }
        },
        38 /* terrane-site: src/main.trn:51:5-51:19 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::value(&custom))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "receiver_api",
        "receiver_api::RecordHandle::value")) }, 39 /* terrane-site: src/main.trn:52:13-52:26 */))
    );
    bump(&mut custom);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::value(&custom))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "receiver_api",
        "receiver_api::RecordHandle::value")) }, 40 /* terrane-site: src/main.trn:54:13-54:26 */))
    );
    locks = __terrane_raised(
        terrane_static_trn_4c6f636b73_new(terrane_int_support::Int::from(20_i128)),
        41 /* terrane-site: src/main.trn:55:13-55:27 */,
    );
    {
        let mut guard: std::sync::MutexGuard<'_, receiver_api::Record> = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&locks).locked()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "receiver_api",
                            "receiver_api::Locks::locked",
                        ),
                    )
                }
            },
            42 /* terrane-site: src/main.trn:56:33-56:46 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&locks)
            .available())) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "receiver_api",
            "receiver_api::Locks::available")) }, 43 /* terrane-site: src/main.trn:57:17-57:33 */))
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = <receiver_api::Record>::set(
                        &mut guard,
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<
                                    i64,
                                >(&terrane_int_support::Int::from(21_i128))
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
                            "std",
                            "std::sync::MutexGuard::set",
                        ),
                    )
                }
            },
            44 /* terrane-site: src/main.trn:58:9-58:22 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | <
            receiver_api::Record > ::value(&guard))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "std",
            "std::sync::MutexGuard::value")) }, 45 /* terrane-site: src/main.trn:59:17-59:29 */))
        );
        {
            let read: std::sync::RwLockReadGuard<'_, receiver_api::Record> = __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| (&locks).read()),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "receiver_api",
                                "receiver_api::Locks::read",
                            ),
                        )
                    }
                },
                46 /* terrane-site: src/main.trn:60:36-60:47 */,
            );
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | <
                receiver_api::Record > ::value(&read))) { Ok(value) =>
                Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
                Err(crate ::__terrane_dependency_panic(payload, "std",
                "std::sync::RwLockReadGuard::value")) }, 47 /* terrane-site: src/main.trn:61:21-61:32 */))
            );
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&locks)
                .writable())) { Ok(value) => Ok(value), Err(payload) => Err(crate
                ::__terrane_dependency_panic(payload, "receiver_api",
                "receiver_api::Locks::writable")) }, 48 /* terrane-site: src/main.trn:62:21-62:36 */))
            );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&locks)
            .writable())) { Ok(value) => Ok(value), Err(payload) => Err(crate
            ::__terrane_dependency_panic(payload, "receiver_api",
            "receiver_api::Locks::writable")) }, 49 /* terrane-site: src/main.trn:64:17-64:32 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&locks).available()))
        { Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "receiver_api",
        "receiver_api::Locks::available")) }, 50 /* terrane-site: src/main.trn:66:13-66:29 */))
    );
    {
        let mut write: std::sync::RwLockWriteGuard<'_, receiver_api::Record> = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&locks).write()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "receiver_api",
                            "receiver_api::Locks::write",
                        ),
                    )
                }
            },
            51 /* terrane-site: src/main.trn:67:33-67:45 */,
        );
        __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    let _ = <receiver_api::Record>::set(
                        &mut write,
                        match || -> Result<_, crate::TerraneForeignError> {
                            Ok(
                                terrane_int_support::coerce::<
                                    i64,
                                >(&terrane_int_support::Int::from(22_i128))
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
                            "std",
                            "std::sync::RwLockWriteGuard::set",
                        ),
                    )
                }
            },
            52 /* terrane-site: src/main.trn:68:9-68:22 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | <
            receiver_api::Record > ::value(&write))) { Ok(value) =>
            Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "std",
            "std::sync::RwLockWriteGuard::value")) }, 53 /* terrane-site: src/main.trn:69:17-69:29 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&locks).writable())) {
        Ok(value) => Ok(value), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "receiver_api",
        "receiver_api::Locks::writable")) }, 54 /* terrane-site: src/main.trn:71:13-71:28 */))
    );
    {
        let view: receiver_api::RecordView<'_> = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| (&custom).view()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "receiver_api",
                            "receiver_api::RecordHandle::view",
                        ),
                    )
                }
            },
            55 /* terrane-site: src/main.trn:73:32-73:44 */,
        );
        let copied: receiver_api::RecordView<'_> = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| <receiver_api::RecordView<
                    '_,
                > as core::clone::Clone>::clone(&view)),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "receiver_api",
                            "receiver_api::RecordView::clone",
                        ),
                    )
                }
            },
            56 /* terrane-site: src/main.trn:74:18-74:29 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | copied.value())) {
            Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
            Err(payload) => Err(crate ::__terrane_dependency_panic(payload,
            "receiver_api", "receiver_api::RecordView::value")) }, 57 /* terrane-site: src/main.trn:75:17-75:30 */))
        );
        {
            let nested: receiver_api::RecordView<'_> = __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| view.nested()),
                ) {
                    Ok(value) => Ok(value),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "receiver_api",
                                "receiver_api::RecordView::nested",
                            ),
                        )
                    }
                },
                58 /* terrane-site: src/main.trn:76:38-76:50 */,
            );
            view_value = __terrane_raised(
                match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| nested.value()),
                ) {
                    Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
                    Err(payload) => {
                        Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "receiver_api",
                                "receiver_api::RecordView::value",
                            ),
                        )
                    }
                },
                59 /* terrane-site: src/main.trn:77:26-77:39 */,
            );
        }
    }
    __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                let _ = <receiver_api::Record>::set(
                    &mut custom,
                    match || -> Result<_, crate::TerraneForeignError> {
                        Ok(
                            terrane_int_support::coerce::<
                                i64,
                            >(&terrane_int_support::Int::from(23_i128))
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
                        "receiver_api",
                        "receiver_api::RecordHandle::set",
                    ),
                )
            }
        },
        60 /* terrane-site: src/main.trn:80:5-80:19 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&view_value));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | < receiver_api::Record
        > ::value(&custom))) { Ok(value) =>
        Ok(terrane_int_support::Int::from(i128::from(value))), Err(payload) => Err(crate
        ::__terrane_dependency_panic(payload, "receiver_api",
        "receiver_api::RecordHandle::value")) }, 61 /* terrane-site: src/main.trn:82:13-82:26 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&ordinary_view()));
    println!("{}", terrane_scalar_support::scalar_text(&ordinary_guard()));
}
fn main() {
    __terrane_run(async move {
        pointers_and_guards();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(suspended_view()).
            await)
        );
        println!("{}", terrane_scalar_support::scalar_text(&named_view()));
    });
}
