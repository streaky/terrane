// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: tests/integration/completion-finally-view-post-read/case.trn
// Namespace: completion-finally-view-post-read
fn test_scenario_terrane_completion_finally_view_post_read() {
    let h: Holder;
    let k: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            0 /* terrane-site: tests/integration/completion-finally-view-post-read/case.trn:6:9-6:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            1 /* terrane-site: tests/integration/completion-finally-view-post-read/case.trn:7:9-7:21 */,
    );
    let _ = v
        .insert(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&h).view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    2 /* terrane-site: tests/integration/completion-finally-view-post-read/case.trn:8:9-8:16 */,
            ),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.take()
                .expect("flow-proven availability").into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 3 /* terrane-site: tests/integration/completion-finally-view-post-read/case.trn:10:17-10:29 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        let _ = v
            .insert(
                __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { { let value =
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                    Result < _, crate ::TerraneForeignError > { Ok((&k).view()) })) {
                    Ok(result) => result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane-query-witness",
                    "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 4 /* terrane-site: tests/integration/completion-finally-view-post-read/case.trn:12:13-12:20 */
                ),
            );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v.as_ref()
        .expect("flow-proven available binding")).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 5 /* terrane-site: tests/integration/completion-finally-view-post-read/case.trn:13:13-13:20 */))
    );
}
// Source: tests/integration/completion-native-catch-post-read/case.trn
// Namespace: completion-native-catch-post-read
fn test_scenario_terrane_completion_native_catch_post_read() {
    let mut c: Option<Cl> = None;
    let _ = c
        .insert(
            __terrane_raised(
                terrane_static_trn_436c_new(),

                    6 /* terrane-site: tests/integration/completion-native-catch-post-read/case.trn:6:9-6:17 */,
            ),
        );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(c.as_ref()
                .expect("flow-proven available binding").clone().into_value()) })) {
                Ok(result) => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::Cl::into_value")) }; Ok:: < _, crate
                ::TerraneForeignError >
                (terrane_int_support::Int::from(i128::from(value))) } } (),
                7 /* terrane-site: tests/integration/completion-native-catch-post-read/case.trn:8:17-8:30 */))
            );
            let _ = c
                .insert(
                    __terrane_raised_completion!(
                        terrane_static_trn_436c_new(), 8 /* terrane-site: tests/integration/completion-native-catch-post-read/case.trn:9:13-9:21 */
                    ),
                );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    println!("{}", terrane_scalar_support::scalar_text(&1));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&c.as_ref()
        .expect("flow-proven available binding")).value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Cl::value")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from(i128::from(value))) } }
        (), 9 /* terrane-site: tests/integration/completion-native-catch-post-read/case.trn:12:13-12:21 */))
    );
}
// Source: tests/integration/completion-native-post-read/case.trn
// Namespace: completion-native-post-read
fn test_scenario_terrane_completion_native_post_read() {
    let mut c: Option<Cl> = None;
    let _ = c
        .insert(
            __terrane_raised(
                terrane_static_trn_436c_new(),

                    10 /* terrane-site: tests/integration/completion-native-post-read/case.trn:6:9-6:17 */,
            ),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(c.as_ref()
                .expect("flow-proven available binding").clone().into_value()) })) {
                Ok(result) => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::Cl::into_value")) }; Ok:: < _, crate
                ::TerraneForeignError >
                (terrane_int_support::Int::from(i128::from(value))) } } (),
                11 /* terrane-site: tests/integration/completion-native-post-read/case.trn:8:17-8:30 */))
            );
            let _ = c
                .insert(
                    __terrane_raised_completion!(
                        terrane_static_trn_436c_new(), 12 /* terrane-site: tests/integration/completion-native-post-read/case.trn:9:13-9:21 */
                    ),
                );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&1));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&c.as_ref()
        .expect("flow-proven available binding")).value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Cl::value")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from(i128::from(value))) } }
        (), 13 /* terrane-site: tests/integration/completion-native-post-read/case.trn:12:13-12:21 */))
    );
}
// Source: tests/integration/native-borrowed-byte-ranges/case.trn
// Namespace: native-borrowed-byte-ranges
fn nonzero_octet(octet: u8) -> bool {
    return octet != 0;
}
fn keep_octet(octet: u8) -> bool {
    return octet != 33;
}
fn native_ranges(source: &Octets) -> String {
    return {
        terrane_string_support::append_utf8_ranges(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&*source).bytes()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Octets::bytes",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    14 /* terrane-site: tests/integration/native-borrowed-byte-ranges/case.trn:12:13-12:26 */,
            ),
            String::from(""),
            0,
            "|",
            nonzero_octet,
        )
    };
}
fn source_ranges(
    source: &Vec<u8>,
    accepts: std::sync::Arc<dyn Fn(u8) -> bool + Send + Sync>,
) -> String {
    let prefix: String;
    prefix = String::from("prefix");
    return {
        terrane_string_support::append_utf8_ranges(
            &*source,
            prefix,
            44,
            "::",
            {
                let predicate = accepts.clone();
                move |byte| predicate(byte)
            },
        )
    };
}
fn shared_ranges() -> String {
    let original: std::sync::Arc<TerraneSharedCell<Vec<u8>>>;
    let data: std::sync::Arc<TerraneSharedCell<Vec<u8>>>;
    let accepts: std::sync::Arc<dyn Fn(u8) -> bool + Send + Sync>;
    original = std::sync::Arc::new(
        TerraneSharedCell::new(Vec::<u8>::from([97, 44, 98])),
    );
    data = original.clone();
    accepts = {
        let data = data.clone();
        std::sync::Arc::new(move |octet: u8| -> bool {
            return terrane_int_support::Int::from(
                {
                    let __terrane_value = data
                        .lock()
                        .expect("shared reference lock poisoned")
                        .clone();
                    __terrane_value
                }
                    .len() as i128,
            ) == terrane_int_support::Int::from(3_i128) && octet != 0;
        })
    };
    return {
        terrane_string_support::append_utf8_ranges(
            &{
                let __terrane_owner_0 = &data;
                let __terrane_snapshot_1 = (*&*__terrane_owner_0
                    .lock()
                    .expect("shared reference lock poisoned"))
                    .clone();
                __terrane_snapshot_1
            },
            String::from(""),
            44,
            "|",
            {
                let predicate = accepts.clone();
                move |byte| predicate(byte)
            },
        )
    };
}
fn test_scenario_terrane_native_borrowed_byte_ranges() {
    let source: Octets;
    let data: Vec<u8>;
    source = __terrane_raised(
        terrane_static_trn_4f6374657473_new(),

            15 /* terrane-site: tests/integration/native-borrowed-byte-ranges/case.trn:26:14-26:26 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&native_ranges(&source)));
    data = Vec::<
        u8,
    >::from([
        99, 97, 102, 195, 169, 44, 44, 100, 105, 115, 33, 99, 97, 114, 100, 44, 255, 44,
        101, 110, 100,
    ]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&source_ranges(&data,
        std::sync::Arc::new(keep_octet)))
    );
    println!("{}", terrane_scalar_support::scalar_text(&shared_ranges()));
}
// Source: tests/integration/native-borrowed-bytes/case.trn
// Namespace: native-borrowed-bytes
fn test_scenario_terrane_native_borrowed_bytes() -> Result<(), TerraneError> {
    let source: std::sync::Arc<TerraneSharedCell<Octets>>;
    let all: Vec<u8>;
    let selected: Option<Vec<u8>>;
    let absent: Option<Vec<u8>>;
    source = std::sync::Arc::new(
        TerraneSharedCell::new(
            __terrane_raised_err(
                terrane_static_trn_4f6374657473_new(),

                    16 /* terrane-site: tests/integration/native-borrowed-bytes/case.trn:6:26-6:38 */,
            )?,
        ),
    );
    all = {
        let __terrane_owner_1 = &source;
        let __terrane_native_guard_2 = __terrane_owner_1
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_call_result = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(__terrane_native_guard_2.bytes()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Octets::bytes",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value.to_owned())
                }
            }(),

                17 /* terrane-site: tests/integration/native-borrowed-bytes/case.trn:7:17-7:30 */,
        )?;
        __terrane_call_result
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(all == Vec:: < u8 > ::from([241, 0,
        97])))
    );
    selected = {
        let __terrane_owner_4 = &source;
        let __terrane_native_guard_5 = __terrane_owner_4
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_source_input_0 = terrane_int_support::Int::from(1_i128);
        let __terrane_shared_argument_0 = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(
                    terrane_int_support::coerce::<usize>(&__terrane_source_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }(),

                18 /* terrane-site: tests/integration/native-borrowed-bytes/case.trn:9:42-9:43 */,
        )?;
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(__terrane_native_guard_5.select(__terrane_shared_argument_1))
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Octets::select",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value.map(|value| value.to_owned())),
                    Err(error) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::TERRANE_DEPENDENCY_ERROR,
                                    format!(
                                        "Rust dependency `{}` member `{}` failed: {error}",
                                        "terrane-query-witness",
                                        "terrane_query_witness::Octets::select"
                                    ),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                }
            }(),

                19 /* terrane-site: tests/integration/native-borrowed-bytes/case.trn:9:27-9:43 */,
        )?;
        __terrane_call_result
    };
    if (&selected).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&(match &selected { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") } .len() as i128))
        );
    }
    absent = {
        let __terrane_owner_7 = &source;
        let __terrane_native_guard_8 = __terrane_owner_7
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_source_input_0 = terrane_int_support::Int::from(3_i128);
        let __terrane_shared_argument_0 = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(
                    terrane_int_support::coerce::<usize>(&__terrane_source_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }(),

                20 /* terrane-site: tests/integration/native-borrowed-bytes/case.trn:12:40-12:41 */,
        )?;
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_call_result = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(__terrane_native_guard_8.select(__terrane_shared_argument_1))
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Octets::select",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value.map(|value| value.to_owned())),
                    Err(error) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::TERRANE_DEPENDENCY_ERROR,
                                    format!(
                                        "Rust dependency `{}` member `{}` failed: {error}",
                                        "terrane-query-witness",
                                        "terrane_query_witness::Octets::select"
                                    ),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                }
            }(),

                21 /* terrane-site: tests/integration/native-borrowed-bytes/case.trn:12:25-12:41 */,
        )?;
        __terrane_call_result
    };
    println!("{}", terrane_scalar_support::scalar_text(&(&absent).is_none()));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_owner_10 = &source;
                let __terrane_native_guard_11 = __terrane_owner_10
                    .lock()
                    .expect("shared reference lock poisoned");
                let __terrane_source_input_0 = terrane_int_support::Int::from(4_i128);
                let __terrane_shared_argument_0 = __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > {
                    Ok(terrane_int_support::coerce:: < usize >
                    (&__terrane_source_input_0).map_err(| error | crate
                    ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                    ::TERRANE_NO_SITE))) ?) } (), 22 /* terrane-site: tests/integration/native-borrowed-bytes/case.trn:15:24-15:25 */
                );
                let __terrane_shared_argument_1 = __terrane_shared_argument_0;
                let __terrane_call_result = __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { match match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok(__terrane_native_guard_11
                    .select(__terrane_shared_argument_1)) })) { Ok(result) => result ?,
                    Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane-query-witness",
                    "terrane_query_witness::Octets::select")) } { Ok(value) => Ok(value
                    .map(| value | value.to_owned())), Err(error) => Err(crate
                    ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                    ::TERRANE_DEPENDENCY_ERROR,
                    format!("Rust dependency `{}` member `{}` failed: {error}",
                    "terrane-query-witness", "terrane_query_witness::Octets::select"),
                    crate ::TERRANE_NO_SITE))) } } (), 23 /* terrane-site: tests/integration/native-borrowed-bytes/case.trn:15:9-15:25 */
                );
                __terrane_call_result
            };
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("native error"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/native-borrowed-iterator/case.trn
// Namespace: native-borrowed-iterator
fn nonempty(value: &MetadataText) -> Option<String> {
    let text: String;
    text = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*value).searched_contents()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::MetadataText::searched_contents",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),

            24 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:5:12-5:36 */,
    );
    if text.as_str() == "" {
        return None;
    }
    return Some(text);
}
fn first_text(records: std::sync::Arc<TerraneSharedCell<Records>>) -> Option<String> {
    return {
        let __terrane_owner_1 = &records;
        let __terrane_native_guard_2 = __terrane_owner_1
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_native_view_316 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_guard_2.values()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::Records::values",
                    ),
                )),

                25 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:11:14-11:29 */,
        );
        let mut __terrane_native_view_315 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_view_316.iter()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::RecordValues::iter",
                    ),
                )),

                26 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:11:13-11:36 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                std::iter::Iterator::find_map(
                                    &mut __terrane_native_view_315,
                                    {
                                        move |borrowed_argument_0| {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = nonempty(borrowed_argument_0);
                                                Ok(callback_value)
                                            }() {
                                                Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error.0),
                                            }
                                        }
                                    },
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "std",
                                    "std::iter::Iterator::find_map",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                27 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:11:12-11:56 */,
        )
    };
}
fn first_text_borrowed(records: &Records) -> Option<String> {
    return {
        let __terrane_native_view_435 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*records).values()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::Records::values",
                    ),
                )),

                28 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:14:14-14:29 */,
        );
        let __terrane_receiver_value_3 = &__terrane_native_view_435;
        let __terrane_native_input_0 = &__terrane_receiver_value_3;
        let mut __terrane_native_view_434 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_input_0.iter()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::RecordValues::iter",
                    ),
                )),

                29 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:14:13-14:36 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                std::iter::Iterator::find_map(
                                    &mut __terrane_native_view_434,
                                    {
                                        move |borrowed_argument_0| {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = nonempty(borrowed_argument_0);
                                                Ok(callback_value)
                                            }() {
                                                Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error.0),
                                            }
                                        }
                                    },
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "std",
                                    "std::iter::Iterator::find_map",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                30 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:14:12-14:56 */,
        )
    };
}
fn first_text_owned() -> Option<String> {
    let records: Records;
    records = {
        let __terrane_conversion_input_0 = true;
        __terrane_raised(
            terrane_static_trn_5265636f726473_new(__terrane_conversion_input_0),

                31 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:17:15-17:33 */,
        )
    };
    return {
        let __terrane_native_view_564 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&records).values()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::Records::values",
                    ),
                )),

                32 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:18:14-18:29 */,
        );
        let __terrane_receiver_value_4 = &__terrane_native_view_564;
        let __terrane_native_input_0 = &__terrane_receiver_value_4;
        let mut __terrane_native_view_563 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_input_0.iter()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::RecordValues::iter",
                    ),
                )),

                33 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:18:13-18:36 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                std::iter::Iterator::find_map(
                                    &mut __terrane_native_view_563,
                                    {
                                        move |borrowed_argument_0| {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = nonempty(borrowed_argument_0);
                                                Ok(callback_value)
                                            }() {
                                                Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error.0),
                                            }
                                        }
                                    },
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "std",
                                    "std::iter::Iterator::find_map",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                34 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:18:12-18:56 */,
        )
    };
}
fn collected_text(records: &Records) -> terrane_collection_support::List<String> {
    return {
        let __terrane_native_view_681 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*records).values()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::Records::values",
                    ),
                )),

                35 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:21:14-21:29 */,
        );
        let __terrane_receiver_value_5 = &__terrane_native_view_681;
        let __terrane_native_input_0 = &__terrane_receiver_value_5;
        let mut __terrane_native_view_680 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_input_0.iter()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::RecordValues::iter",
                    ),
                )),

                36 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:21:13-21:36 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_collection_support::filter_map_collect_borrowed(
                                    &mut __terrane_native_view_680,
                                    {
                                        move |borrowed_argument_0| {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = nonempty_entry(borrowed_argument_0);
                                                Ok(callback_value)
                                            }() {
                                                Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error.0),
                                            }
                                        }
                                    },
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_collection_support",
                                    "terrane_collection_support::filter_map_collect_borrowed",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(terrane_collection_support::List::new(value))
                }
            }(),

                37 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:21:12-21:72 */,
        )
    };
}
fn mapped_text() -> terrane_collection_support::List<String> {
    let records: Records;
    records = {
        let __terrane_conversion_input_0 = true;
        __terrane_raised(
            terrane_static_trn_5265636f726473_new(__terrane_conversion_input_0),

                38 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:24:15-24:33 */,
        )
    };
    return {
        let __terrane_native_view_824 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&records).values()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::Records::values",
                    ),
                )),

                39 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:25:14-25:29 */,
        );
        let __terrane_receiver_value_6 = &__terrane_native_view_824;
        let __terrane_native_input_0 = &__terrane_receiver_value_6;
        let mut __terrane_native_view_823 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_input_0.iter()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::RecordValues::iter",
                    ),
                )),

                40 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:25:13-25:36 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_collection_support::map_collect_borrowed(
                                    &mut __terrane_native_view_823,
                                    {
                                        move |borrowed_argument_0| {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = entry_text(borrowed_argument_0);
                                                Ok(callback_value)
                                            }() {
                                                Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error.0),
                                            }
                                        }
                                    },
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_collection_support",
                                    "terrane_collection_support::map_collect_borrowed",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(terrane_collection_support::List::new(value))
                }
            }(),

                41 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:25:12-25:61 */,
        )
    };
}
fn entry_text(entry: &MetadataText) -> String {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*entry).contents()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::MetadataText::contents",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),

            42 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:28:12-28:27 */,
    );
}
fn nonempty_entry(entry: &MetadataText) -> Option<String> {
    let text: String;
    text = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*entry).contents()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::MetadataText::contents",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),

            43 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:31:12-31:27 */,
    );
    if text.as_str() == "" {
        return None;
    }
    return Some(text);
}
fn add_pair(total: u64, first: &MetadataText, second: &MetadataText) -> u64 {
    let first_length: u64;
    let second_length: u64;
    let base: u64;
    let partial: u64;
    first_length = __terrane_raised(
        terrane_int_support::coerce::<
            u64,
        >(
            &(__terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > { Ok((&*first).contents()) }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane-query-witness",
                                            "terrane_query_witness::MetadataText::contents",
                                        ),
                                    );
                                }
                            };
                            Ok::<_, crate::TerraneForeignError>(value)
                        }
                    }(),

                        44 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:37:28-37:43 */,
                )
                .len() as i128),
        ),

            45 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:37:27-37:57 */,
    );
    second_length = __terrane_raised(
        terrane_int_support::coerce::<
            u64,
        >(
            &(__terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > { Ok((&*second).contents()) }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane-query-witness",
                                            "terrane_query_witness::MetadataText::contents",
                                        ),
                                    );
                                }
                            };
                            Ok::<_, crate::TerraneForeignError>(value)
                        }
                    }(),

                        46 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:38:29-38:45 */,
                )
                .len() as i128),
        ),

            47 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:38:28-38:59 */,
    );
    base = terrane_int_support::fixed_multiplication_saturate(total, 10);
    partial = terrane_int_support::fixed_addition_saturate(base, first_length);
    return terrane_int_support::fixed_addition_saturate(partial, second_length);
}
fn pair_score(records: std::sync::Arc<TerraneSharedCell<Records>>) -> u64 {
    let seed: u64;
    seed = 0;
    return {
        let __terrane_owner_8 = &records;
        let __terrane_native_guard_9 = __terrane_owner_8
            .lock()
            .expect("shared reference lock poisoned");
        let mut __terrane_native_view_1523 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_guard_9.pairs()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::Records::pairs",
                    ),
                )),

                48 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:45:13-45:27 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_collection_support::fold_borrowed_pair(
                                    &mut __terrane_native_view_1523,
                                    seed,
                                    {
                                        move |
                                            borrowed_argument_0,
                                            borrowed_argument_1,
                                            borrowed_argument_2|
                                        {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = add_pair(
                                                    borrowed_argument_0,
                                                    borrowed_argument_1,
                                                    borrowed_argument_2,
                                                );
                                                Ok(callback_value)
                                            }() {
                                                Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error.0),
                                            }
                                        }
                                    },
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_collection_support",
                                    "terrane_collection_support::fold_borrowed_pair",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                49 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:45:12-45:49 */,
        )
    };
}
fn fold_value(total: u64, value: &MetadataText) -> u64 {
    let length: u64;
    length = __terrane_raised(
        terrane_int_support::coerce::<
            u64,
        >(
            &(__terrane_raised(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > { Ok((&*value).contents()) }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane-query-witness",
                                            "terrane_query_witness::MetadataText::contents",
                                        ),
                                    );
                                }
                            };
                            Ok::<_, crate::TerraneForeignError>(value)
                        }
                    }(),

                        50 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:48:22-48:37 */,
                )
                .len() as i128),
        ),

            51 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:48:21-48:51 */,
    );
    return terrane_int_support::fixed_addition_saturate(total, length);
}
fn value_score(records: &Records) -> u64 {
    let seed: u64;
    seed = 0;
    return {
        let __terrane_native_view_1798 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*records).values()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::Records::values",
                    ),
                )),

                52 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:53:14-53:29 */,
        );
        let __terrane_receiver_value_10 = &__terrane_native_view_1798;
        let __terrane_native_input_0 = &__terrane_receiver_value_10;
        let mut __terrane_native_view_1797 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_input_0.iter()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::RecordValues::iter",
                    ),
                )),

                53 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:53:13-53:36 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_collection_support::fold_borrowed(
                                    &mut __terrane_native_view_1797,
                                    seed,
                                    {
                                        move |borrowed_argument_0, borrowed_argument_1| {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = fold_value(
                                                    borrowed_argument_0,
                                                    borrowed_argument_1,
                                                );
                                                Ok(callback_value)
                                            }() {
                                                Ok(value) => value,
                                                Err(error) => std::panic::panic_any(error.0),
                                            }
                                        }
                                    },
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_collection_support",
                                    "terrane_collection_support::fold_borrowed",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                54 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:53:12-53:60 */,
        )
    };
}
fn test_scenario_terrane_native_borrowed_iterator() {
    let records: std::sync::Arc<TerraneSharedCell<Records>>;
    let found: Option<String>;
    let missing: std::sync::Arc<TerraneSharedCell<Records>>;
    let plain: Records;
    let borrowed: Option<String>;
    let owned: Option<String>;
    let view: terrane_query_witness::RecordValues<
        '_,
        terrane_query_witness::MetadataText,
    >;
    let bound: Option<String>;
    let mut text: String;
    let empty: Records;
    records = std::sync::Arc::new(
        TerraneSharedCell::new({
            let __terrane_conversion_input_0 = true;
            __terrane_raised(
                terrane_static_trn_5265636f726473_new(__terrane_conversion_input_0),

                    55 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:56:27-56:45 */,
            )
        }),
    );
    found = first_text(records.clone());
    if (&found).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missing = std::sync::Arc::new(
        TerraneSharedCell::new({
            let __terrane_conversion_input_0 = false;
            __terrane_raised(
                terrane_static_trn_5265636f726473_new(__terrane_conversion_input_0),

                    56 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:60:27-60:46 */,
            )
        }),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&first_text(missing.clone()).is_none())
    );
    plain = {
        let __terrane_conversion_input_0 = true;
        __terrane_raised(
            terrane_static_trn_5265636f726473_new(__terrane_conversion_input_0),

                57 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:62:13-62:31 */,
        )
    };
    borrowed = first_text_borrowed(&plain);
    if (&borrowed).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &borrowed { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    owned = first_text_owned();
    if (&owned).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &owned { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&plain).values()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Records::values",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            58 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:69:12-69:25 */,
    );
    bound = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            std::iter::Iterator::find_map(
                                &mut (&view).iter(),
                                {
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = nonempty(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                },
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "std",
                                "std::iter::Iterator::find_map",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            59 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:70:25-70:56 */,
    );
    if (&bound).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &bound { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&value_score(&plain)));
    let __terrane_iterable_0 = collected_text(&plain);
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        text = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&text));
    }
    empty = {
        let __terrane_conversion_input_0 = false;
        __terrane_raised(
            terrane_static_trn_5265636f726473_new(__terrane_conversion_input_0),

                60 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:76:13-76:32 */,
        )
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(collected_text(&empty)
        .length()))
    );
    let __terrane_iterable_1 = mapped_text();
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        text = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let mut __terrane_text =
            String::with_capacity(2); __terrane_text.push_str("[");
            terrane_scalar_support::ScalarDisplay::write_scalar(&text, &mut
            __terrane_text); __terrane_text.push_str("]"); __terrane_text })
        );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4d6574616461746154657874_copies(),
        61 /* terrane-site: tests/integration/native-borrowed-iterator/case.trn:80:13-80:34 */))
    );
    println!("{}", terrane_scalar_support::scalar_text(&pair_score(records.clone())));
}
// Source: tests/integration/native-borrowed-slice/case.trn
// Namespace: native-borrowed-slice
fn title(entry: &SliceEntry) -> String {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*entry).title()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::SliceEntry::title",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),

            62 /* terrane-site: tests/integration/native-borrowed-slice/case.trn:5:12-5:24 */,
    );
}
fn titles(catalog: &SliceCatalog) -> terrane_collection_support::List<String> {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            terrane_collection_support::collect_borrowed_slice(
                                (&*catalog).entries(),
                                {
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = title(borrowed_argument_0);
                                            Ok(callback_value)
                                        }() {
                                            Ok(value) => value,
                                            Err(error) => std::panic::panic_any(error.0),
                                        }
                                    }
                                },
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane_collection_support",
                                "terrane_collection_support::collect_borrowed_slice",
                            ),
                        );
                    }
                };
                Ok::<
                    _,
                    crate::TerraneForeignError,
                >(terrane_collection_support::List::new(value))
            }
        }(),

            63 /* terrane-site: tests/integration/native-borrowed-slice/case.trn:8:12-8:49 */,
    );
}
fn test_scenario_terrane_native_borrowed_slice() {
    let catalog: SliceCatalog;
    let mut name: String;
    catalog = __terrane_raised(
        terrane_static_trn_536c696365436174616c6f67_sample(),

            64 /* terrane-site: tests/integration/native-borrowed-slice/case.trn:11:15-11:36 */,
    );
    let __terrane_iterable_0 = titles(&catalog);
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        name = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&name));
    }
}
// Source: tests/integration/native-borrowed-sum/case.trn
// Namespace: native-borrowed-sum
fn lookup(
    records: &Records,
    mode: terrane_int_support::Int,
    index: terrane_int_support::Int,
) -> Option<MetadataText> {
    let ordinal: Option<u64>;
    ordinal = terrane_int_support::checked_coerce::<u64>(&index);
    if (&ordinal).is_some() {
        return {
            let __terrane_native_view_308 = __terrane_raised(
                std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                (&*records)
                                    .parameters(
                                        terrane_int_support::coerce::<i64>(&mode.clone())
                                            .map_err(|error| crate::TerraneForeignError(
                                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                            ))?,
                                    ),
                            )
                        }),
                    )
                    .unwrap_or_else(|payload| Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane-query-witness",
                            "terrane_query_witness::Records::parameters",
                        ),
                    )),

                    65 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:8:18-8:42 */,
            );
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok(
                                    terrane_collection_support::get_cloned_borrowed_slice(
                                        (|input| match input {
                                            Some(terrane_query_witness::Choice::Left(value)) => {
                                                Some(value)
                                            }
                                            _ => None,
                                        })(__terrane_native_view_308),
                                        *match &ordinal {
                                            Some(value) => value,
                                            _ => unreachable!("flow-proven storage refinement"),
                                        },
                                    ),
                                )
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_collection_support",
                                        "terrane_collection_support::get_cloned_borrowed_slice",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    66 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:8:16-8:77 */,
            )
        };
    }
    return None;
}
fn point_x(points: &PointRecords) -> terrane_int_support::Int {
    let selected: Option<PublicPoint>;
    selected = {
        let __terrane_native_view_448 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*points).points()) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::PointRecords::points",
                    ),
                )),

                67 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:12:18-12:32 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_collection_support::get_cloned_borrowed_slice(
                                    (|input| match input {
                                        Some(
                                            terrane_query_witness::PointChoice::TextValue(value),
                                        ) => Some(value),
                                        _ => None,
                                    })(__terrane_native_view_448),
                                    0,
                                ),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_collection_support",
                                    "terrane_collection_support::get_cloned_borrowed_slice",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                68 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:12:16-12:67 */,
        )
    };
    if (&selected).is_some() {
        return terrane_int_support::Int::from(
            i128::from(
                *&match &selected {
                    Some(value) => value,
                    _ => unreachable!("flow-proven storage refinement"),
                }
                    .x,
            ),
        );
    }
    return terrane_int_support::Int::from(0_i128);
}
fn test_scenario_terrane_native_borrowed_sum() {
    let records: Records;
    let found: Option<MetadataText>;
    let points: PointRecords;
    let mut found_before_panic: Option<Option<MetadataText>> = None;
    let mut error: Option<TerraneError> = None;
    records = {
        let __terrane_conversion_input_0 = true;
        __terrane_raised(
            terrane_static_trn_5265636f726473_new(__terrane_conversion_input_0),

                69 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:18:15-18:33 */,
        )
    };
    found = lookup(
        &records,
        terrane_int_support::Int::from(1_i128),
        terrane_int_support::Int::from(1_i128),
    );
    if (&found).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(match &found { Some(value) => value, _ =>
            unreachable!("flow-proven storage refinement") } .contents()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::MetadataText::contents")) }; Ok:: < _, crate
            ::TerraneForeignError > (value.to_owned()) } } (), 70 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:21:17-21:32 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&lookup(&records,
        terrane_int_support::Int::from(0_i128), terrane_int_support::Int::from(1_i128))
        .is_none())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&lookup(&records,
        terrane_int_support::Int::from(2_i128), terrane_int_support::Int::from(1_i128))
        .is_none())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&lookup(&records,
        terrane_int_support::Int::from(1_i128), terrane_int_support::Int::from(3_i128))
        .is_none())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&lookup(&records,
        terrane_int_support::Int::from(1_i128), terrane_int_support::Int::from(- 1_i128))
        .is_none())
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4d6574616461746154657874_copies(),
        71 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:26:13-26:34 */))
    );
    points = __terrane_raised(
        terrane_static_trn_506f696e745265636f726473_new(),

            72 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:27:14-27:32 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&point_x(&points)));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = found_before_panic
                .insert({
                    let __terrane_native_view_1033 = __terrane_raised_completion!(
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                        Result < _, crate ::TerraneForeignError > { Ok((&records)
                        .parameters(terrane_int_support::coerce:: < i64 >
                        (&terrane_int_support::Int::from(9_i128)).map_err(| error | crate
                        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
                        ::TERRANE_NO_SITE))) ?)) })).unwrap_or_else(| payload | Err(crate
                        ::__terrane_dependency_panic(payload, "terrane-query-witness",
                        "terrane_query_witness::Records::parameters"))),
                        73 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:30:50-30:71 */
                    );
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > {
                        Ok(terrane_collection_support::get_cloned_borrowed_slice((| input
                        | match input { Some(terrane_query_witness::Choice::Left(value))
                        => Some(value), _ => None }) (__terrane_native_view_1033), 0))
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload,
                        "terrane_collection_support",
                        "terrane_collection_support::get_cloned_borrowed_slice")) }; Ok::
                        < _, crate ::TerraneForeignError > (value) } } (),
                        74 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:30:48-30:100 */
                    )
                });
            println!(
                "{}", terrane_scalar_support::scalar_text(&found_before_panic.as_ref()
                .expect("flow-proven available binding").is_none())
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_0 = true;
                    let _ = error.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&({ let
                        __terrane_error_text_0 = error.as_ref()
                        .expect("flow-proven available binding").message(); let
                        __terrane_string_argument_0 = "terrane-query-witness";
                        __terrane_error_text_0.contains(std::convert::AsRef:: < str >
                        ::as_ref(&__terrane_string_argument_0)) } &&{ let
                        __terrane_error_text_1 = error.as_ref()
                        .expect("flow-proven available binding").message(); let
                        __terrane_string_argument_0 =
                        "terrane_query_witness::Records::parameters";
                        __terrane_error_text_1.contains(std::convert::AsRef:: < str >
                        ::as_ref(&__terrane_string_argument_0)) } &&{ let
                        __terrane_error_text_2 = error.as_ref()
                        .expect("flow-proven available binding").message(); let
                        __terrane_string_argument_0 = "parameters panic";
                        __terrane_error_text_2.contains(std::convert::AsRef:: < str >
                        ::as_ref(&__terrane_string_argument_0)) }))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_view_1365 =
        __terrane_raised(std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
        Result < _, crate ::TerraneForeignError > { Ok((&points)
        .collision(terrane_int_support::coerce:: < i64 >
        (&terrane_int_support::Int::from(1_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?)) })).unwrap_or_else(| payload | Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::PointRecords::collision"))), 75 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:34:15-34:34 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > {
        Ok(terrane_collection_support::get_cloned_borrowed_slice((| input | match input {
        Some(terrane_query_witness::PointChoice::HttpValue(value)) => Some(value), _ =>
        None }) (__terrane_native_view_1365), 0)) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_collection_support",
        "terrane_collection_support::get_cloned_borrowed_slice")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 76 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:34:13-34:69 */) } .is_some())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_view_1443 =
        __terrane_raised(std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
        Result < _, crate ::TerraneForeignError > { Ok((&points)
        .collision(terrane_int_support::coerce:: < i64 >
        (&terrane_int_support::Int::from(2_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?)) })).unwrap_or_else(| payload | Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::PointRecords::collision"))), 77 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:35:15-35:34 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > {
        Ok(terrane_collection_support::get_cloned_borrowed_slice((| input | match input {
        Some(terrane_query_witness::PointChoice::HTTPValue(value)) => Some(value), _ =>
        None }) (__terrane_native_view_1443), 0)) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_collection_support",
        "terrane_collection_support::get_cloned_borrowed_slice")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 78 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:35:13-35:78 */) } .is_some())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_view_1530 =
        __terrane_raised(std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
        Result < _, crate ::TerraneForeignError > { Ok((&points)
        .collision(terrane_int_support::coerce:: < i64 >
        (&terrane_int_support::Int::from(3_i128)).map_err(| error | crate
        ::TerraneForeignError(crate ::TerraneRaised::raised(error, crate
        ::TERRANE_NO_SITE))) ?)) })).unwrap_or_else(| payload | Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::PointRecords::collision"))), 79 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:36:15-36:34 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > {
        Ok(terrane_collection_support::get_cloned_borrowed_slice((| input | match input {
        Some(terrane_query_witness::PointChoice::HttpValueVariant3(value)) =>
        Some(value), _ => None }) (__terrane_native_view_1530), 0)) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_collection_support",
        "terrane_collection_support::get_cloned_borrowed_slice")) }; Ok:: < _, crate
        ::TerraneForeignError > (value) } } (), 80 /* terrane-site: tests/integration/native-borrowed-sum/case.trn:36:13-36:78 */) } .is_some())
    );
}
// Source: tests/integration/native-checked-ordinal/case.trn
// Namespace: native-checked-ordinal
fn cell_size(
    row: &OrdinalRow,
    index: terrane_int_support::Int,
) -> Result<terrane_int_support::Int, TerraneError> {
    return Ok({
        let __terrane_conversion_input_0 = index.clone();
        let __terrane_checked_native_input_0 = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(
                    terrane_int_support::coerce::<usize>(&__terrane_conversion_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }(),

                81 /* terrane-site: tests/integration/native-checked-ordinal/case.trn:6:23-6:28 */,
        )?;
        let __terrane_native_view_215 = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                match match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*row).cell::<usize>(__terrane_checked_native_input_0)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::OrdinalRow::cell",
                            ),
                        );
                    }
                } {
                    Ok(value) => Ok(value),
                    Err(error) => {
                        Err(
                            crate::TerraneForeignError(
                                crate::TerraneError::custom_raised(
                                    crate::TERRANE_DEPENDENCY_ERROR,
                                    format!(
                                        "Rust dependency `{}` member `{}` failed: {error}",
                                        "terrane-query-witness",
                                        "terrane_query_witness::OrdinalRow::cell"
                                    ),
                                    crate::TERRANE_NO_SITE,
                                ),
                            ),
                        )
                    }
                }
            }(),

                82 /* terrane-site: tests/integration/native-checked-ordinal/case.trn:6:13-6:28 */,
        )?;
        {
            let __terrane_receiver_value_0 = &__terrane_native_view_215;
            let __terrane_native_input_0 = &__terrane_receiver_value_0;
            __terrane_raised_err(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok(__terrane_native_input_0.size()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::View::size",
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

                    83 /* terrane-site: tests/integration/native-checked-ordinal/case.trn:6:12-6:35 */,
            )?
        }
    });
}
fn test_scenario_terrane_native_checked_ordinal() -> Result<(), TerraneError> {
    let row: OrdinalRow;
    row = __terrane_raised_err(
        terrane_static_trn_4f7264696e616c526f77_sample(),

            84 /* terrane-site: tests/integration/native-checked-ordinal/case.trn:9:11-9:30 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(cell_size(&row,
        terrane_int_support::Int::from(1_i128)), 85 /* terrane-site: tests/integration/native-checked-ordinal/case.trn:10:13-10:30 */) ?)
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(cell_size(&row,
                terrane_int_support::Int::from(- 1_i128)), 86 /* terrane-site: tests/integration/native-checked-ordinal/case.trn:12:17-12:35 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::IntegerConversionOverflow
                {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("index overflow"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(cell_size(&row,
                terrane_int_support::Int::from(2_i128)), 87 /* terrane-site: tests/integration/native-checked-ordinal/case.trn:16:17-16:34 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1 {
                    __terrane_handled_1 = true;
                    println!("{}", String::from("missing cell"));
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/native-copy-view-list-lender/case.trn
// Namespace: native-copy-view-list-lender
fn test_scenario_terrane_native_copy_view_list_lender() {
    let mut holder: Holder;
    let views: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let alias: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >;
    let nested_alias: terrane_collection_support::List<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >;
    let copy: terrane_query_witness::CopyView<'_>;
    let copied: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let mut appended: terrane_collection_support::List<
        terrane_query_witness::CopyView<'_>,
    >;
    let mut item: terrane_query_witness::CopyView<'_>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            88 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:6:14-6:26 */,
    );
    views = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 89 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:7:20-7:37 */); value
            }
        ],
    );
    alias = views;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(alias
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        90 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:9:13-9:21 */)),
        90 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:9:13-9:21 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 91 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:9:13-9:27 */) })
    );
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >::new(
        vec![
            terrane_collection_support::List:: < terrane_query_witness::CopyView < '_ > >
            ::new(vec![{ let value : terrane_query_witness::CopyView < '_ > =
            __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 92 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:10:28-10:45 */);
            value }])
        ],
    );
    nested_alias = nested;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(__terrane_raised(nested_alias
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        93 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:28 */)),
        93 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:28 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        94 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:31 */)),
        94 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:31 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 95 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:37 */) })
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = holder.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            96 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:13:5-13:17 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&holder).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 97 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:14:13-14:30 */))
    );
    copy = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&holder).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            98 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:15:12-15:29 */,
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = copy.size();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::CopyView::size",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            99 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:16:5-16:15 */,
    );
    copied = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(vec![{ let value : terrane_query_witness::CopyView <'_ > = copy; value }]);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(copy.size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 100 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:18:13-18:23 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(copied
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        101 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:19:13-19:22 */)),
        101 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:19:13-19:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 102 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:19:13-19:28 */) })
    );
    appended = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = copy;
        appended.push_unique(__terrane_collection_argument_0);
    };
    let __terrane_iterable_0 = &appended;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        item = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(item.size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 103 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:23:17-23:27 */))
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
                            let _ = holder.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            104 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:24:5-24:17 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&holder).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 105 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:25:13-25:30 */))
    );
}
// Source: tests/integration/native-copy-view-reassignment-snapshot/case.trn
// Namespace: native-copy-view-reassignment-snapshot
fn fresh<'a>(lender: &'a Holder) -> terrane_query_witness::CopyView<'a> {
    println!("{}", String::from("fresh"));
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*lender).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            106 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:7:12-7:29 */,
    );
}
fn test_scenario_terrane_native_copy_view_reassignment_snapshot() {
    let h: Holder;
    let mut k: Holder;
    let c: terrane_query_witness::CopyView<'_>;
    let values: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            107 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:10:9-10:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            108 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:11:9-11:21 */,
    );
    c = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            109 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:12:9-12:21 */,
    );
    values = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(vec![{ let value : terrane_query_witness::CopyView <'_ > = c; value }]);
    let c = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&k).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            110 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:14:9-14:21 */,
    );
    let _ = &c;
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = k.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            111 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:15:5-15:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        112 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:22 */)),
        112 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 113 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:28 */)
        })
    );
    let c = fresh(&k);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(c.size()) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 114 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:18:13-18:20 */))
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = k.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            115 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:19:5-19:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        116 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:22 */)),
        116 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 117 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:28 */)
        })
    );
}
// Source: tests/integration/native-field-copy-clone-conversion/case.trn
// Namespace: native-field-copy-clone-conversion
#[derive(Clone)]
pub struct TerraneNsNativeFieldCopyCloneConversionRIdentity2F6E61746976652D6669656C642D636F70792D636C6F6E652D636F6E76657273696F6E0052 {
    pub t: Option<Token>,
    pub e: Option<CloneEnum>,
}
impl TerraneNsNativeFieldCopyCloneConversionRIdentity2F6E61746976652D6669656C642D636F70792D636C6F6E652D636F6E76657273696F6E0052 {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { t: None, e: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value: Token = __terrane_raised(
                terrane_static_trn_546f6b656e_new(),

                    118 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:7:18-7:29 */,
            );
            self.t = Some(__terrane_value);
        }
        {
            let __terrane_value: CloneEnum = __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok(terrane_query_witness::CloneEnum::A) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::CloneEnum::A",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    119 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:8:18-8:31 */,
            );
            self.e = Some(__terrane_value);
        }
    }
}
fn observe_terrane_native_field_copy_clone_conversion(
    r: &TerraneNsNativeFieldCopyCloneConversionRIdentity2F6E61746976652D6669656C642D636F70792D636C6F6E652D636F6E76657273696F6E0052,
) -> Result<u64, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                u64,
            >(
                &__terrane_raised_err(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > {
                                    Ok(
                                        {
                                            let __terrane_snapshot = r
                                                .t
                                                .as_ref()
                                                .expect("required field initialized")
                                                .clone();
                                            __terrane_snapshot
                                        }
                                            .value(),
                                    )
                                }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane-query-witness",
                                            "terrane_query_witness::Token::value",
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

                        120 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:10:12-10:22 */,
                )?,
            ),

                120 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:10:12-10:22 */,
        )?,
    );
}
fn clone_observe(r: &mut CloneEnum) -> Result<u64, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                u64,
            >(
                &__terrane_raised_err(
                    || -> Result<_, crate::TerraneForeignError> {
                        {
                            let value = match std::panic::catch_unwind(
                                std::panic::AssertUnwindSafe(|| -> Result<
                                    _,
                                    crate::TerraneForeignError,
                                > { Ok((*r).clone().code()) }),
                            ) {
                                Ok(result) => result?,
                                Err(payload) => {
                                    return Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane-query-witness",
                                            "terrane_query_witness::CloneEnum::code",
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

                        121 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:12:12-12:19 */,
                )?,
            ),

                121 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:12:12-12:19 */,
        )?,
    );
}
fn test_scenario_terrane_native_field_copy_clone_conversion() -> Result<
    (),
    TerraneError,
> {
    let r: TerraneNsNativeFieldCopyCloneConversionRIdentity2F6E61746976652D6669656C642D636F70792D636C6F6E652D636F6E76657273696F6E0052;
    let mut i: terrane_int_support::Int;
    let mut e: CloneEnum;
    r = TerraneNsNativeFieldCopyCloneConversionRIdentity2F6E61746976652D6669656C642D636F70792D636C6F6E652D636F6E76657273696F6E0052::terrane_construct();
    i = terrane_int_support::Int::from(0_i128);
    while i < terrane_int_support::Int::from(2_i128) {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
            < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(r.t.as_ref()
            .expect("required field initialized").clone().value()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Token::value")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 122 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:17:17-17:27 */)
            ?)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
            < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(r.t.as_ref()
            .expect("required field initialized").clone().value()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Token::value")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 123 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:18:17-18:29 */)
            ?)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
            < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(r.e.as_ref()
            .expect("required field initialized").clone().code()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::CloneEnum::code")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 124 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:19:17-19:26 */)
            ?)
        );
        i = &i + &terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(observe_terrane_native_field_copy_clone_conversion(&r),
        125 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:21:13-21:23 */) ?)
    );
    e = __terrane_raised_err(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(terrane_query_witness::CloneEnum::A) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::CloneEnum::A",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            126 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:22:9-22:22 */,
    )?;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(clone_observe(&mut e),
        127 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:23:13-23:29 */) ?)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(clone_observe(&mut e),
        128 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:24:13-24:29 */) ?)
    );
    Ok(())
}
// Source: tests/integration/native-field-text/case.trn
// Namespace: native-field-text
fn original_path(original: &OriginalTarget, n0: &ResponseCode) -> String {
    if __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&*n0).canonical_reason()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::ResponseCode::canonical_reason",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(value.map(|value| value.to_owned()))
                }
            }(),

                129 /* terrane-site: tests/integration/native-field-text/case.trn:7:9-7:29 */,
        )
        .is_some()
    {
        return __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&(*original).0).path()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::RequestTarget::path",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value.to_owned())
                }
            }(),

                130 /* terrane-site: tests/integration/native-field-text/case.trn:8:16-8:35 */,
        );
    }
    return String::from("");
}
fn test_scenario_terrane_native_field_text() {
    let original: OriginalTarget;
    let status: ResponseCode;
    original = __terrane_raised(
        terrane_static_trn_4f726967696e616c546172676574_new("/mounted/path"),

            131 /* terrane-site: tests/integration/native-field-text/case.trn:12:16-12:52 */,
    );
    status = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(200_i128);
        __terrane_raised(
            terrane_static_trn_526573706f6e7365436f6465_new(
                __terrane_conversion_input_0,
            ),

                132 /* terrane-site: tests/integration/native-field-text/case.trn:13:14-13:36 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&original_path(&original, &status))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4f726967696e616c546172676574_clones(),
        133 /* terrane-site: tests/integration/native-field-text/case.trn:15:13-15:36 */))
    );
}
// Source: tests/integration/native-finally-only-rewrite/case.trn
// Namespace: native-finally-only-rewrite
fn test_scenario_terrane_native_finally_only_rewrite() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            134 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:4:9-4:21 */,
    );
    let _ = v
        .insert(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&h).view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    135 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:5:9-5:16 */,
            ),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.take()
                .expect("flow-proven availability").into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 136 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:7:17-7:29 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        let _ = v
            .insert(
                __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { { let value =
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                    Result < _, crate ::TerraneForeignError > { Ok((&h).view()) })) {
                    Ok(result) => result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane-query-witness",
                    "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 137 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:9:13-9:20 */
                ),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v.as_ref()
            .expect("flow-proven available binding")).size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 138 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:10:17-10:24 */))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
// Source: tests/integration/native-from-conversion/case.trn
// Namespace: native-from-conversion
fn borrowed_text(text: &String) -> FromText {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::FromText as ::core::convert::From<
                                &str,
                            >>::from(&*text),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::FromText::from",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            139 /* terrane-site: tests/integration/native-from-conversion/case.trn:5:12-5:32 */,
    );
}
fn borrowed_native(value: &FromInput) -> FromText {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::FromText as ::core::convert::From<
                                &terrane_query_witness::FromInput,
                            >>::from(&*value),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::FromText::from",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            140 /* terrane-site: tests/integration/native-from-conversion/case.trn:8:12-8:33 */,
    );
}
fn test_scenario_terrane_native_from_conversion() {
    let text: String;
    let borrowed: FromText;
    let source: String;
    let allocation: terrane_int_support::Int;
    let owned: terrane_query_witness::FromText;
    let reusable: String;
    let copied: terrane_query_witness::FromText;
    let native: FromInput;
    let borrowed_value: FromText;
    let moved_value: terrane_query_witness::FromText;
    let inherent: InherentFrom;
    text = String::from("native/from/path");
    borrowed = borrowed_text(&text);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&borrowed).text()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::FromText::text")) }; Ok:: < _,
        crate ::TerraneForeignError > (value.to_owned()) } } (), 141 /* terrane-site: tests/integration/native-from-conversion/case.trn:13:13-13:27 */))
    );
    source = String::from("allocation reuse");
    allocation = __terrane_raised(
        string_allocation_id(&source),

            142 /* terrane-site: tests/integration/native-from-conversion/case.trn:15:19-15:47 */,
    );
    owned = {
        let __terrane_conversion_input_0 = source;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                <terrane_query_witness::FromText as ::core::convert::From<
                                    String,
                                >>::from(__terrane_conversion_input_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::FromText::from",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                143 /* terrane-site: tests/integration/native-from-conversion/case.trn:16:13-16:40 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&owned).allocation_id()) })) { Ok(result) => result
        ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::FromText::allocation_id")) };
        Ok:: < _, crate ::TerraneForeignError >
        (terrane_int_support::Int::from_u128(value as u128)) } } (),
        144 /* terrane-site: tests/integration/native-from-conversion/case.trn:17:13-17:33 */) == allocation))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&owned).text()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::FromText::text")) }; Ok:: < _,
        crate ::TerraneForeignError > (value.to_owned()) } } (), 145 /* terrane-site: tests/integration/native-from-conversion/case.trn:18:13-18:24 */))
    );
    reusable = String::from("source stays live");
    copied = {
        let __terrane_conversion_input_0 = reusable.clone();
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                <terrane_query_witness::FromText as ::core::convert::From<
                                    String,
                                >>::from(__terrane_conversion_input_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::FromText::from",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                146 /* terrane-site: tests/integration/native-from-conversion/case.trn:20:14-20:38 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&(__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&copied).text()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::FromText::text")) }; Ok:: < _,
        crate ::TerraneForeignError > (value.to_owned()) } } (), 147 /* terrane-site: tests/integration/native-from-conversion/case.trn:21:13-21:25 */).as_str() ==
        reusable.as_str()))
    );
    native = __terrane_raised(
        terrane_static_trn_46726f6d496e707574_new(),

            148 /* terrane-site: tests/integration/native-from-conversion/case.trn:22:14-22:29 */,
    );
    borrowed_value = borrowed_native(&native);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&borrowed_value).text()) })) { Ok(result) => result
        ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::FromText::text")) }; Ok:: < _,
        crate ::TerraneForeignError > (value.to_owned()) } } (), 149 /* terrane-site: tests/integration/native-from-conversion/case.trn:24:13-24:33 */))
    );
    moved_value = {
        let __terrane_conversion_input_0 = native;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                <terrane_query_witness::FromText as ::core::convert::From<
                                    terrane_query_witness::FromInput,
                                >>::from(__terrane_conversion_input_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::FromText::from",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                150 /* terrane-site: tests/integration/native-from-conversion/case.trn:25:19-25:46 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&moved_value).text()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::FromText::text")) }; Ok:: < _,
        crate ::TerraneForeignError > (value.to_owned()) } } (), 151 /* terrane-site: tests/integration/native-from-conversion/case.trn:26:13-26:30 */))
    );
    inherent = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(42_i128);
        __terrane_raised(
            terrane_static_trn_496e686572656e7446726f6d_from(
                __terrane_conversion_input_0,
            ),

                152 /* terrane-site: tests/integration/native-from-conversion/case.trn:27:16-27:38 */,
        )
    };
    println!("{}", terrane_scalar_support::scalar_text(&inherent.0));
}
// Source: tests/integration/native-inferred-copy-view/case.trn
// Namespace: native-inferred-copy-view
fn mutable_size(value: &mut CopyView) -> terrane_int_support::Int {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((*value).clone().size()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::CopyView::size",
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

            153 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:5:12-5:23 */,
    );
}
fn readonly_size(value: &CopyView) -> terrane_int_support::Int {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(value.clone().size()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::CopyView::size",
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

            154 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:8:12-8:23 */,
    );
}
fn test_scenario_terrane_native_inferred_copy_view() {
    let holder: Holder;
    let mut inferred_view: terrane_query_witness::CopyView<'_>;
    let inferred: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let typed: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            155 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:11:14-11:26 */,
    );
    inferred_view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&holder).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            156 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:12:21-12:38 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&mutable_size(&mut inferred_view))
    );
    println!("{}", terrane_scalar_support::scalar_text(&readonly_size(&inferred_view)));
    inferred = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 157 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:15:23-15:40 */); value }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(inferred
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        158 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:24 */)),
        158 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:24 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 159 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:30 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(inferred
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        160 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:24 */)),
        160 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:24 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 161 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:30 */) })
    );
    typed = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 162 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:18:37-18:54 */); value }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(typed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        163 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:21 */)),
        163 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:21 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 164 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:27 */) })
    );
}
// Source: tests/integration/native-loan-append-transfer/case.trn
// Namespace: native-loan-append-transfer
fn test_scenario_terrane_native_loan_append_transfer() {
    let h: Holder;
    let v: terrane_query_witness::View<'_>;
    let mut values: terrane_collection_support::List<terrane_query_witness::View<'_>>;
    let other: terrane_query_witness::View<'_>;
    let mut declared: terrane_collection_support::List<terrane_query_witness::View<'_>>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            165 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:6:9-6:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            166 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:7:9-7:16 */,
    );
    values = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::View < '_ > = __terrane_raised(| | ->
            Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&h).view()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::Holder::view")) }; Ok:: < _,
            crate ::TerraneForeignError > (value) } } (), 167 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:8:21-8:28 */); value }
        ],
    );
    {
        let __terrane_collection_argument_0 = v;
        values.push_unique(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_0 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        168 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:10:13-10:22 */); let
        __terrane_index_place_1 = __terrane_raised(values.get(__terrane_index_0)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_0)),
        168 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:10:13-10:22 */); let
        __terrane_native_input_0 = &__terrane_index_place_1; __terrane_raised(| | ->
        Result < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 169 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:10:13-10:28 */) })
    );
    other = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            170 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:11:13-11:20 */,
    );
    declared = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = other;
        declared.push_unique(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_2 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        171 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:14:13-14:24 */); let
        __terrane_index_place_3 = __terrane_raised(declared.get(__terrane_index_2)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_2)),
        171 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:14:13-14:24 */); let
        __terrane_native_input_0 = &__terrane_index_place_3; __terrane_raised(| | ->
        Result < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 172 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:14:13-14:30 */) })
    );
}
// Source: tests/integration/native-loan-chain-rebind-receiver/case.trn
// Namespace: native-loan-chain-rebind-receiver
pub struct TerraneNsNativeLoanChainRebindReceiverWIdentity2F6E61746976652D6C6F616E2D636861696E2D726562696E642D72656365697665720057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanChainRebindReceiverWIdentity2F6E61746976652D6C6F616E2D636861696E2D726562696E642D72656365697665720057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_chain_rebind_receiver() {
    let h: Holder;
    let k: Holder;
    let mut v: terrane_query_witness::View<'_>;
    let a: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            173 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:11:9-11:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            174 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:12:9-12:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            175 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:13:9-13:16 */,
    );
    a = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(v.chain()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::View::chain",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            176 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:14:9-14:24 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&k).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            177 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:15:9-15:16 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 178 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:16:13-16:20 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 179 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:16:24-16:31 */))
    );
}
// Source: tests/integration/native-loan-exclusive-branch/case.trn
// Namespace: native-loan-exclusive-branch
pub struct TerraneNsNativeLoanExclusiveBranchWIdentity2F6E61746976652D6C6F616E2D6578636C75736976652D6272616E63680057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanExclusiveBranchWIdentity2F6E61746976652D6C6F616E2D6578636C75736976652D6272616E63680057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_exclusive_branch() {
    let mut w: TerraneNsNativeLoanExclusiveBranchWIdentity2F6E61746976652D6C6F616E2D6578636C75736976652D6272616E63680057;
    let v: terrane_query_witness::View<'_>;
    let c: bool;
    w = TerraneNsNativeLoanExclusiveBranchWIdentity2F6E61746976652D6C6F616E2D6578636C75736976652D6272616E63680057::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                180 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                181 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:11:38-11:50 */,
        ),
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.inner.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            182 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:12:9-12:22 */,
    );
    c = true;
    if c {
        {
            let __terrane_value: Holder = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    183 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:15:19-15:31 */,
            );
            w.inner = Some(__terrane_value);
        }
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 184 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:17:17-17:24 */))
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&7));
}
// Source: tests/integration/native-loan-finally-copy-rebind/case.trn
// Namespace: native-loan-finally-copy-rebind
pub struct TerraneNsNativeLoanFinallyCopyRebindWIdentity2F6E61746976652D6C6F616E2D66696E616C6C792D636F70792D726562696E640057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanFinallyCopyRebindWIdentity2F6E61746976652D6C6F616E2D66696E616C6C792D636F70792D726562696E640057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_finally_copy_rebind() {
    let h: Holder;
    let mut v: terrane_query_witness::CopyView<'_>;
    let mut i: terrane_int_support::Int;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            185 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:11:9-11:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            186 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:12:9-12:21 */,
    );
    i = terrane_int_support::Int::from(0_i128);
    while i < terrane_int_support::Int::from(2_i128) {
        let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
            let __terrane_try_0: TerraneCompletion<()> = (|| {
                println!("{}", terrane_scalar_support::scalar_text(&1));
                TerraneCompletion::Normal
            })();
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.size()) })) { Ok(result) => result
                ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
                "terrane-query-witness", "terrane_query_witness::CopyView::size")) };
                Ok:: < _, crate ::TerraneForeignError >
                (terrane_int_support::Int::from_u128(value as u128)) } } (),
                187 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:18:21-18:28 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).copy_view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 188 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:19:17-19:29 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_0 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_0 = replacement,
        }
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break => break,
            TerraneCompletion::Continue => {
                __terrane_generated_defect("non-continuing try produced continue")
            }
        }
        i = &i + &terrane_int_support::Int::from(1_i128);
    }
}
// Source: tests/integration/native-loan-fluent-nested-rebind/case.trn
// Namespace: native-loan-fluent-nested-rebind
fn test_scenario_terrane_native_loan_fluent_nested_rebind() {
    let h: Holder;
    let mut unrelated: Holder;
    let mut v: terrane_query_witness::View<'_>;
    let fluent: terrane_collection_support::List<terrane_query_witness::View<'_>>;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_query_witness::View<'_>>,
    >;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            189 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:6:9-6:21 */,
    );
    unrelated = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            190 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:7:17-7:29 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            191 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:8:9-8:16 */,
    );
    fluent = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::View < '_ > = __terrane_raised(| | ->
            Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(v.chain()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::chain")) }; Ok:: < _,
            crate ::TerraneForeignError > (value) } } (), 192 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:9:21-9:29 */);
            value }
        ],
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            193 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:10:9-10:16 */,
    );
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_query_witness::View<'_>>,
    >::new(
        vec![
            terrane_collection_support::List::< terrane_query_witness::View <'_
            >>::new(vec![{ let value : terrane_query_witness::View <'_ > = v; value }])
        ],
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            194 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:12:9-12:16 */,
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = unrelated.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            195 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:13:5-13:20 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(fluent
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(nested
        .length()))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 196 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:16:13-16:20 */))
    );
}
// Source: tests/integration/native-loan-helper-view-origin/case.trn
// Namespace: native-loan-helper-view-origin
fn pick<'a>(
    value: terrane_query_witness::CopyView<'a>,
) -> terrane_query_witness::CopyView<'a> {
    return value;
}
fn wrap(value: CopyView) -> terrane_collection_support::List<CopyView> {
    return terrane_collection_support::List::<CopyView>::new(vec![value]);
}
fn identity(
    values: terrane_collection_support::List<CopyView>,
) -> terrane_collection_support::List<CopyView> {
    return values;
}
fn test_scenario_terrane_native_loan_helper_view_origin() {
    let mut h: Holder;
    let view: CopyView;
    let values: terrane_collection_support::List<CopyView>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            197 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:15:9-15:21 */,
    );
    view = pick(
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&h).copy_view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                198 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:16:19-16:31 */,
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(view.size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 199 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:17:13-17:23 */))
    );
    values = identity(
        wrap(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&h).copy_view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::copy_view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    200 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:18:32-18:44 */,
            ),
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        201 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:22 */)),
        201 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 202 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:28 */) })
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = h.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            203 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:20:5-20:12 */,
    );
}
// Source: tests/integration/native-loan-last-use-control/case.trn
// Namespace: native-loan-last-use-control
fn test_scenario_terrane_native_loan_last_use_control() {
    let mut h: Holder;
    let values: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            204 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:6:9-6:21 */,
    );
    values = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&h).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 205 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:7:38-7:50 */); value
            }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        206 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:8:13-8:22 */)),
        206 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:8:13-8:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 207 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:8:13-8:28 */) })
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = h.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            208 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:9:5-9:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 209 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:10:13-10:25 */))
    );
}
// Source: tests/integration/native-loan-loop-live-lender/case.trn
// Namespace: native-loan-loop-live-lender
fn test_scenario_terrane_native_loan_loop_live_lender() {
    let mut h: Holder;
    let values: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let mut value: terrane_query_witness::CopyView<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            210 /* terrane-site: tests/integration/native-loan-loop-live-lender/case.trn:6:9-6:21 */,
    );
    values = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&h).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 211 /* terrane-site: tests/integration/native-loan-loop-live-lender/case.trn:7:38-7:50 */); value
            }
        ],
    );
    let __terrane_iterable_0 = &values;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 212 /* terrane-site: tests/integration/native-loan-loop-live-lender/case.trn:9:17-9:28 */))
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
                            let _ = h.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            213 /* terrane-site: tests/integration/native-loan-loop-live-lender/case.trn:10:5-10:12 */,
    );
}
// Source: tests/integration/native-loan-loop-refreshed/case.trn
// Namespace: native-loan-loop-refreshed
pub struct TerraneNsNativeLoanLoopRefreshedWIdentity2F6E61746976652D6C6F616E2D6C6F6F702D7265667265736865640057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanLoopRefreshedWIdentity2F6E61746976652D6C6F616E2D6C6F6F702D7265667265736865640057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_loop_refreshed() {
    let mut w: TerraneNsNativeLoanLoopRefreshedWIdentity2F6E61746976652D6C6F616E2D6C6F6F702D7265667265736865640057;
    let mut i: terrane_int_support::Int;
    let mut v: terrane_query_witness::View<'_>;
    w = TerraneNsNativeLoanLoopRefreshedWIdentity2F6E61746976652D6C6F616E2D6C6F6F702D7265667265736865640057::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                214 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                215 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:11:38-11:50 */,
        ),
    );
    i = terrane_int_support::Int::from(0_i128);
    while i < terrane_int_support::Int::from(2_i128) {
        v = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                (&*w.inner.as_ref().expect("required field initialized"))
                                    .view(),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                216 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:14:13-14:26 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 217 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:15:17-15:24 */))
        );
        {
            let __terrane_value: Holder = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    218 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:16:19-16:31 */,
            );
            w.inner = Some(__terrane_value);
        }
        i = &i + &terrane_int_support::Int::from(1_i128);
    }
}
// Source: tests/integration/native-loan-map-set-lender/case.trn
// Namespace: native-loan-map-set-lender
fn test_scenario_terrane_native_loan_map_set_lender() {
    let mut h: Holder;
    let mut values: terrane_collection_support::Map<
        String,
        terrane_query_witness::CopyView<'_>,
    >;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            219 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:6:9-6:21 */,
    );
    values = terrane_collection_support::Map::<
        String,
        terrane_query_witness::CopyView<'_>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("view");
        let __terrane_collection_argument_1 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&h).copy_view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                220 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:8:26-8:38 */,
        );
        values.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values.get_or_error(&String::from("view")),
        221 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:9:13-9:27 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 222 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:9:13-9:33 */) })
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = h.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            223 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:10:5-10:12 */,
    );
}
// Source: tests/integration/native-loan-owned-observation/case.trn
// Namespace: native-loan-owned-observation
pub struct TerraneNsNativeLoanOwnedObservationWIdentity2F6E61746976652D6C6F616E2D6F776E65642D6F62736572766174696F6E0057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanOwnedObservationWIdentity2F6E61746976652D6C6F616E2D6F776E65642D6F62736572766174696F6E0057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_owned_observation() {
    let mut h: Holder;
    let v: terrane_query_witness::View<'_>;
    let observed: terrane_int_support::Int;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            224 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:11:9-11:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            225 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:12:9-12:16 */,
    );
    observed = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&v).size()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::View::size",
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

            226 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:13:16-13:23 */,
    );
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            227 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:14:9-14:21 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&observed));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 228 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:16:13-16:25 */))
    );
}
// Source: tests/integration/native-loan-predicate-last-use/case.trn
// Namespace: native-loan-predicate-last-use
pub struct TerraneNsNativeLoanPredicateLastUseWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D6C6173742D7573650057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanPredicateLastUseWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D6C6173742D7573650057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_predicate_last_use() {
    let mut w: TerraneNsNativeLoanPredicateLastUseWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D6C6173742D7573650057;
    let v: terrane_query_witness::View<'_>;
    w = TerraneNsNativeLoanPredicateLastUseWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D6C6173742D7573650057::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                229 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                230 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:11:38-11:50 */,
        ),
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.inner.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            231 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:12:9-12:22 */,
    );
    if __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&v).size()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::View::size",
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

            232 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:14:15-14:22 */,
    ) > terrane_int_support::Int::from(100_i128)
    {
        {
            let __terrane_value: Holder = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    233 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:15:23-15:35 */,
            );
            w.inner = Some(__terrane_value);
        }
    } else {
        println!("{}", terrane_scalar_support::scalar_text(&8));
    }
}
// Source: tests/integration/native-loan-predicate-switch/case.trn
// Namespace: native-loan-predicate-switch
pub struct TerraneNsNativeLoanPredicateSwitchWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D7377697463680057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanPredicateSwitchWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D7377697463680057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_predicate_switch() {
    let mut w: TerraneNsNativeLoanPredicateSwitchWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D7377697463680057;
    let v: terrane_query_witness::View<'_>;
    let i: i64;
    w = TerraneNsNativeLoanPredicateSwitchWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D7377697463680057::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                234 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                235 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:11:38-11:50 */,
        ),
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.inner.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            236 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:12:9-12:22 */,
    );
    i = 1;
    if i == 0 {
        {
            let __terrane_value: Holder = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    237 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:16:23-16:35 */,
            );
            w.inner = Some(__terrane_value);
        }
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 238 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:18:21-18:28 */))
        );
    }
    println!("{}", terrane_scalar_support::scalar_text(&7));
}
// Source: tests/integration/native-loan-predicate-three-arms/case.trn
// Namespace: native-loan-predicate-three-arms
pub struct TerraneNsNativeLoanPredicateThreeArmsWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D74687265652D61726D730057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanPredicateThreeArmsWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D74687265652D61726D730057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_predicate_three_arms() {
    let mut w: TerraneNsNativeLoanPredicateThreeArmsWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D74687265652D61726D730057;
    let v: terrane_query_witness::View<'_>;
    let i: i64;
    w = TerraneNsNativeLoanPredicateThreeArmsWIdentity2F6E61746976652D6C6F616E2D7072656469636174652D74687265652D61726D730057::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                239 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                240 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:11:38-11:50 */,
        ),
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.inner.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            241 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:12:9-12:22 */,
    );
    i = 2;
    if i == 0 {
        {
            let __terrane_value: Holder = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    242 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:16:23-16:35 */,
            );
            w.inner = Some(__terrane_value);
        }
    } else if i == 1 {
        println!("{}", terrane_scalar_support::scalar_text(&7));
    } else {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 243 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:20:21-20:28 */))
        );
    }
}
// Source: tests/integration/native-loan-rebound-alias-chain/case.trn
// Namespace: native-loan-rebound-alias-chain
pub struct TerraneNsNativeLoanReboundAliasChainWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D616C6961732D636861696E0057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanReboundAliasChainWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D616C6961732D636861696E0057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_rebound_alias_chain() {
    let mut h: Holder;
    let k: Holder;
    let mut v: terrane_query_witness::View<'_>;
    let a: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            244 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:11:9-11:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            245 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:12:9-12:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            246 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:13:9-13:16 */,
    );
    let _ = &v;
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&k).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            247 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:14:9-14:16 */,
    );
    a = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(v.chain()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::View::chain",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            248 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:15:9-15:24 */,
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = h.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            249 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:16:5-16:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 250 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:17:13-17:20 */))
    );
}
// Source: tests/integration/native-loan-rebound-other/case.trn
// Namespace: native-loan-rebound-other
pub struct TerraneNsNativeLoanReboundOtherWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D6F746865720057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanReboundOtherWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D6F746865720057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_rebound_other() {
    let mut w: TerraneNsNativeLoanReboundOtherWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D6F746865720057;
    let mut v: terrane_query_witness::View<'_>;
    w = TerraneNsNativeLoanReboundOtherWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D6F746865720057::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                251 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                252 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:11:38-11:50 */,
        ),
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.inner.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            253 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:12:9-12:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 254 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:13:13-13:20 */))
    );
    {
        let __terrane_value: Holder = __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                255 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:14:15-14:27 */,
        );
        w.inner = Some(__terrane_value);
    }
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.other.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            256 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:15:9-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 257 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:16:13-16:20 */))
    );
}
// Source: tests/integration/native-loan-rebound-same/case.trn
// Namespace: native-loan-rebound-same
pub struct TerraneNsNativeLoanReboundSameWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D73616D650057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanReboundSameWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D73616D650057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_rebound_same() {
    let mut w: TerraneNsNativeLoanReboundSameWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D73616D650057;
    let mut v: terrane_query_witness::View<'_>;
    w = TerraneNsNativeLoanReboundSameWIdentity2F6E61746976652D6C6F616E2D7265626F756E642D73616D650057::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                258 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                259 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:11:38-11:50 */,
        ),
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.inner.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            260 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:12:9-12:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 261 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:13:13-13:20 */))
    );
    {
        let __terrane_value: Holder = __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                262 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:14:15-14:27 */,
        );
        w.inner = Some(__terrane_value);
    }
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.inner.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            263 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:15:9-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 264 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:16:13-16:20 */))
    );
}
// Source: tests/integration/native-loan-return-branch/case.trn
// Namespace: native-loan-return-branch
pub struct TerraneNsNativeLoanReturnBranchWIdentity2F6E61746976652D6C6F616E2D72657475726E2D6272616E63680057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanReturnBranchWIdentity2F6E61746976652D6C6F616E2D72657475726E2D6272616E63680057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_return_branch() {
    let mut w: TerraneNsNativeLoanReturnBranchWIdentity2F6E61746976652D6C6F616E2D72657475726E2D6272616E63680057;
    let v: terrane_query_witness::View<'_>;
    let c: bool;
    w = TerraneNsNativeLoanReturnBranchWIdentity2F6E61746976652D6C6F616E2D72657475726E2D6272616E63680057::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                265 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                266 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:11:38-11:50 */,
        ),
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*w.inner.as_ref().expect("required field initialized"))
                                .view(),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            267 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:12:9-12:22 */,
    );
    c = false;
    if c {
        {
            let __terrane_value: Holder = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    268 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:15:19-15:31 */,
            );
            w.inner = Some(__terrane_value);
        }
        return ();
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 269 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:17:13-17:20 */))
    );
}
// Source: tests/integration/native-loan-storage-alias-last-use/case.trn
// Namespace: native-loan-storage-alias-last-use
fn test_scenario_terrane_native_loan_storage_alias_last_use() {
    let h: Holder;
    let mut k: Holder;
    let mut v: terrane_query_witness::View<'_>;
    let a: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            270 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            271 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:5:9-5:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            272 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:6:9-6:16 */,
    );
    a = v;
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&k).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            273 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:8:9-8:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 274 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:9:13-9:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 275 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:10:13-10:20 */))
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = k.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            276 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:11:5-11:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&k).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 277 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:12:13-12:25 */))
    );
}
// Source: tests/integration/native-loan-try-finally-last-use/case.trn
// Namespace: native-loan-try-finally-last-use
pub struct TerraneNsNativeLoanTryFinallyLastUseWIdentity2F6E61746976652D6C6F616E2D7472792D66696E616C6C792D6C6173742D7573650057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanTryFinallyLastUseWIdentity2F6E61746976652D6C6F616E2D7472792D66696E616C6C792D6C6173742D7573650057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_try_finally_last_use() {
    let mut h: Holder;
    let v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            278 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:11:9-11:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            279 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:12:9-12:16 */,
    );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 280 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:14:17-14:24 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        __terrane_raised_completion!(
            | | -> Result < _, crate ::TerraneForeignError > { { let _ = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok({ let _ = h.bump(); }) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::bump")) }; Ok:: < _, crate
            ::TerraneForeignError > (()) } } (), 281 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:16:9-16:16 */
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 282 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:17:13-17:25 */))
    );
}
// Source: tests/integration/native-loan-try-local/case.trn
// Namespace: native-loan-try-local
pub struct TerraneNsNativeLoanTryLocalWIdentity2F6E61746976652D6C6F616E2D7472792D6C6F63616C0057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanTryLocalWIdentity2F6E61746976652D6C6F616E2D7472792D6C6F63616C0057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_try_local() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            283 /* terrane-site: tests/integration/native-loan-try-local/case.trn:11:9-11:21 */,
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = v
                .insert(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane-query-witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 284 /* terrane-site: tests/integration/native-loan-try-local/case.trn:13:13-13:20 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&v.as_ref()
                .expect("flow-proven available binding")).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 285 /* terrane-site: tests/integration/native-loan-try-local/case.trn:14:17-14:24 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                        | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h)
                        .label_len()) })) { Ok(result) => result ?, Err(payload) =>
                        return Err(crate ::__terrane_dependency_panic(payload,
                        "terrane-query-witness",
                        "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
                        ::TerraneForeignError >
                        (terrane_int_support::Int::from_u128(value as u128)) } } (),
                        286 /* terrane-site: tests/integration/native-loan-try-local/case.trn:16:17-16:29 */))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 287 /* terrane-site: tests/integration/native-loan-try-local/case.trn:17:13-17:25 */))
    );
}
// Source: tests/integration/native-loan-whole-last-use/case.trn
// Namespace: native-loan-whole-last-use
pub struct TerraneNsNativeLoanWholeLastUseWIdentity2F6E61746976652D6C6F616E2D77686F6C652D6C6173742D7573650057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanWholeLastUseWIdentity2F6E61746976652D6C6F616E2D77686F6C652D6C6173742D7573650057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_whole_last_use() {
    let mut h: Holder;
    let v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            288 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:11:9-11:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            289 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 290 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:13:13-13:20 */))
    );
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            291 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:14:9-14:21 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 292 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:15:13-15:25 */))
    );
}
// Source: tests/integration/native-loan-whole-move-last-use/case.trn
// Namespace: native-loan-whole-move-last-use
pub struct TerraneNsNativeLoanWholeMoveLastUseWIdentity2F6E61746976652D6C6F616E2D77686F6C652D6D6F76652D6C6173742D7573650057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanWholeMoveLastUseWIdentity2F6E61746976652D6C6F616E2D77686F6C652D6D6F76652D6C6173742D7573650057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_whole_move_last_use() {
    let h: Holder;
    let v: terrane_query_witness::View<'_>;
    let moved: Holder;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            293 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:11:9-11:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            294 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 295 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:13:13-13:20 */))
    );
    moved = h;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&moved).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 296 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:15:13-15:29 */))
    );
}
// Source: tests/integration/native-loan-whole-mutate-last-use/case.trn
// Namespace: native-loan-whole-mutate-last-use
pub struct TerraneNsNativeLoanWholeMutateLastUseWIdentity2F6E61746976652D6C6F616E2D77686F6C652D6D75746174652D6C6173742D7573650057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoanWholeMutateLastUseWIdentity2F6E61746976652D6C6F616E2D77686F6C652D6D75746174652D6C6173742D7573650057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_whole_mutate_last_use() {
    let mut h: Holder;
    let v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            297 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:11:9-11:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            298 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 299 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:13:13-13:20 */))
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = h.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            300 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:14:5-14:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 301 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:15:13-15:25 */))
    );
}
// Source: tests/integration/native-loop-consuming-condition-break/case.trn
// Namespace: native-loop-consuming-condition-break
pub struct TerraneNsNativeLoopConsumingConditionBreakWIdentity2F6E61746976652D6C6F6F702D636F6E73756D696E672D636F6E646974696F6E2D627265616B0057 {
    pub inner: Option<Holder>,
    pub other: Option<Holder>,
}
impl TerraneNsNativeLoopConsumingConditionBreakWIdentity2F6E61746976652D6C6F6F702D636F6E73756D696E672D636F6E646974696F6E2D627265616B0057 {
    pub fn terrane_construct(inner: Holder, other: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None, other: None };
        __terrane_constructed_value.construct(inner, other);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder, other: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value: Holder = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loop_consuming_condition_break() {
    let h: Holder;
    let v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            302 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:11:9-11:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            303 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:12:9-12:16 */,
    );
    while __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(v.into_size()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::View::into_size",
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

            304 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:13:12-13:24 */,
    ) > terrane_int_support::Int::from(0_i128)
    {
        println!("{}", terrane_scalar_support::scalar_text(&1));
        break;
    }
}
// Source: tests/integration/native-noncopy-view-loop-append/case.trn
// Namespace: native-noncopy-view-loop-append
fn test_scenario_terrane_native_noncopy_view_loop_append() {
    let holder: Holder;
    let mut view: terrane_query_witness::View<'_>;
    let mut values: terrane_collection_support::List<terrane_query_witness::View<'_>>;
    let mut index: terrane_int_support::Int;
    let mut value: &terrane_query_witness::View<'_>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            305 /* terrane-site: tests/integration/native-noncopy-view-loop-append/case.trn:6:14-6:26 */,
    );
    view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&holder).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            306 /* terrane-site: tests/integration/native-noncopy-view-loop-append/case.trn:7:12-7:24 */,
    );
    values = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(vec![]);
    let __terrane_iterable_0 = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(3_i128)
        ],
    );
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        index = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &index;
        {
            let __terrane_collection_argument_0 = view;
            values.push_unique(__terrane_collection_argument_0);
        };
        view = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&holder).view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                307 /* terrane-site: tests/integration/native-noncopy-view-loop-append/case.trn:11:16-11:28 */,
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    let __terrane_iterable_1 = &values;
    let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&* value).size()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::View::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 308 /* terrane-site: tests/integration/native-noncopy-view-loop-append/case.trn:14:17-14:28 */))
        );
    }
}
// Source: tests/integration/native-optional-text/case.trn
// Namespace: native-optional-text
fn status_reason(code: &ResponseCode) -> String {
    let reason: Option<String>;
    reason = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*code).canonical_reason()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::ResponseCode::canonical_reason",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.map(|value| value.to_owned()))
            }
        }(),

            309 /* terrane-site: tests/integration/native-optional-text/case.trn:7:26-7:48 */,
    );
    if (&reason).is_some() {
        return match reason {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    }
    return String::from("HTTP error");
}
fn test_scenario_terrane_native_optional_text() -> Result<(), TerraneError> {
    let known: ResponseCode;
    let unknown: ResponseCode;
    let phrase: std::sync::Arc<TerraneSharedCell<ReasonPhrase>>;
    let owned: Option<String>;
    let selected: Option<String>;
    let absent: Option<String>;
    known = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(200_i128);
        __terrane_raised_err(
            terrane_static_trn_526573706f6e7365436f6465_new(
                __terrane_conversion_input_0,
            ),

                310 /* terrane-site: tests/integration/native-optional-text/case.trn:13:13-13:35 */,
        )?
    };
    unknown = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(299_i128);
        __terrane_raised_err(
            terrane_static_trn_526573706f6e7365436f6465_new(
                __terrane_conversion_input_0,
            ),

                311 /* terrane-site: tests/integration/native-optional-text/case.trn:14:15-14:37 */,
        )?
    };
    println!("{}", terrane_scalar_support::scalar_text(&status_reason(&known)));
    println!("{}", terrane_scalar_support::scalar_text(&status_reason(&unknown)));
    phrase = std::sync::Arc::new(
        TerraneSharedCell::new(
            __terrane_raised_err(
                terrane_static_trn_526561736f6e506872617365_new("owned after guard"),

                    312 /* terrane-site: tests/integration/native-optional-text/case.trn:17:26-17:64 */,
            )?,
        ),
    );
    owned = {
        let __terrane_owner_1 = &phrase;
        let __terrane_native_guard_2 = __terrane_owner_1
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_call_result = __terrane_raised_err(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(__terrane_native_guard_2.get()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::ReasonPhrase::get",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(value.map(|value| value.to_owned()))
                }
            }(),

                313 /* terrane-site: tests/integration/native-optional-text/case.trn:18:25-18:36 */,
        )?;
        __terrane_call_result
    };
    if (&owned).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &owned { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    selected = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(200_i128);
        __terrane_raised_err(
            choose_reason(__terrane_conversion_input_0, "borrowed argument"),

                314 /* terrane-site: tests/integration/native-optional-text/case.trn:21:28-21:67 */,
        )?
    };
    if (&selected).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &selected { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    absent = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(299_i128);
        __terrane_raised_err(
            choose_reason(__terrane_conversion_input_0, "ignored"),

                315 /* terrane-site: tests/integration/native-optional-text/case.trn:24:26-24:55 */,
        )?
    };
    println!("{}", terrane_scalar_support::scalar_text(&(&absent).is_none()));
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            {
                let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                    1_i128,
                );
                __terrane_raised_completion!(
                    choose_reason(__terrane_conversion_input_0, "invalid"),
                    316 /* terrane-site: tests/integration/native-optional-text/case.trn:27:9-27:36 */
                )
            };
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    println!("{}", String::from("native error"));
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return Ok(value),
        TerraneCompletion::Error(error) => return Err(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    Ok(())
}
// Source: tests/integration/native-ref-clone-repeat/case.trn
// Namespace: native-ref-clone-repeat
fn twice(value: &mut CloneEnum) -> Result<u64, TerraneError> {
    let first: terrane_int_support::Int;
    let second: terrane_int_support::Int;
    first = __terrane_raised_err(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((*value).clone().code()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::CloneEnum::code",
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

            317 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:4:13-4:24 */,
    )?;
    second = __terrane_raised_err(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((*value).clone().code()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::CloneEnum::code",
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

            318 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:5:14-5:25 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<u64>(&(&first + &second)),

                319 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:6:12-6:26 */,
        )?,
    );
}
fn copied(value: &mut Token) -> Result<u64, TerraneError> {
    let first: terrane_int_support::Int;
    first = __terrane_raised_err(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((*value).clone().value()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Token::value",
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

            320 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:8:13-8:25 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                u64,
            >(
                &(first.clone()
                    + __terrane_raised_err(
                        || -> Result<_, crate::TerraneForeignError> {
                            {
                                let value = match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| -> Result<
                                        _,
                                        crate::TerraneForeignError,
                                    > { Ok((*value).clone().value()) }),
                                ) {
                                    Ok(result) => result?,
                                    Err(payload) => {
                                        return Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "terrane-query-witness",
                                                "terrane_query_witness::Token::value",
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

                            321 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:9:21-9:33 */,
                    )?),
            ),

                322 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:9:12-9:34 */,
        )?,
    );
}
fn test_scenario_terrane_native_ref_clone_repeat() -> Result<(), TerraneError> {
    let mut value: CloneEnum;
    let mut token: Token;
    value = __terrane_raised_err(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(terrane_query_witness::CloneEnum::A) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::CloneEnum::A",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            323 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:11:13-11:26 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(twice(&mut
        value), 324 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:12:13-12:25 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(value.code()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::CloneEnum::code")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 325 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:13:13-13:24 */) ?)
    );
    token = __terrane_raised_err(
        terrane_static_trn_546f6b656e_new(),

            326 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:14:13-14:24 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(copied(&mut
        token), 327 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:15:13-15:26 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(token.value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::Token::value")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 328 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:16:13-16:25 */) ?)
    );
    Ok(())
}
// Source: tests/integration/native-required-enum-field/case.trn
// Namespace: native-required-enum-field
pub struct TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052 {
    pub value: Option<NE>,
    pub sibling: terrane_int_support::Int,
}
impl TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052 {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self {
            value: None,
            sibling: terrane_int_support::Int::from(9_i128),
        };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value: NE = __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok(terrane_query_witness::NE::A) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::NE::A",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    329 /* terrane-site: tests/integration/native-required-enum-field/case.trn:7:22-7:28 */,
            );
            self.value = Some(__terrane_value);
        }
    }
    pub fn finish(mut self) -> Result<u64, TerraneError> {
        return Ok(
            __terrane_raised_err(
                terrane_int_support::coerce::<
                    u64,
                >(
                    &__terrane_raised_err(
                        || -> Result<_, crate::TerraneForeignError> {
                            {
                                let value = match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| -> Result<
                                        _,
                                        crate::TerraneForeignError,
                                    > {
                                        Ok(
                                            self
                                                .value
                                                .take()
                                                .expect("required field initialized")
                                                .code(),
                                        )
                                    }),
                                ) {
                                    Ok(result) => result?,
                                    Err(payload) => {
                                        return Err(
                                            crate::__terrane_dependency_panic(
                                                payload,
                                                "terrane-query-witness",
                                                "terrane_query_witness::NE::code",
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

                            330 /* terrane-site: tests/integration/native-required-enum-field/case.trn:9:16-9:32 */,
                    )?,
                ),

                    330 /* terrane-site: tests/integration/native-required-enum-field/case.trn:9:16-9:32 */,
            )?,
        );
    }
}
pub struct Outer {
    pub inner: Option<
        TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052,
    >,
}
impl Outer {
    pub fn terrane_construct() -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct();
        __terrane_constructed_value
    }
    pub fn construct(&mut self) {
        {
            let __terrane_value: TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052 = TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052::terrane_construct();
            self.inner = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_required_enum_field() -> Result<(), TerraneError> {
    let mut value: TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052;
    let mut outer: Outer;
    let owned: TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052;
    value = TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(value.value.take()
        .expect("required field initialized").code()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::NE::code")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 331 /* terrane-site: tests/integration/native-required-enum-field/case.trn:16:13-16:30 */) ?)
    );
    println!("{}", terrane_scalar_support::scalar_text(&value.sibling));
    outer = Outer::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(outer.inner.as_mut()
        .expect("required field initialized").value.take()
        .expect("required field initialized").code()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::NE::code")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 332 /* terrane-site: tests/integration/native-required-enum-field/case.trn:19:13-19:36 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer.inner.as_ref()
        .expect("required field initialized").sibling)
    );
    owned = TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(owned.finish(),
        333 /* terrane-site: tests/integration/native-required-enum-field/case.trn:22:13-22:26 */) ?)
    );
    Ok(())
}
// Source: tests/integration/native-return-parameter-single/case.trn
// Namespace: native-return-parameter-single
fn f<'a>(p: &'a Holder) -> terrane_query_witness::CopyView<'a> {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*p).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            334 /* terrane-site: tests/integration/native-return-parameter-single/case.trn:5:12-5:24 */,
    );
}
fn test_scenario_terrane_native_return_parameter_single() {
    let mut h: Holder;
    let v: CopyView;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            335 /* terrane-site: tests/integration/native-return-parameter-single/case.trn:8:9-8:21 */,
    );
    v = f(&h);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(v.size()) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 336 /* terrane-site: tests/integration/native-return-parameter-single/case.trn:10:13-10:20 */))
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = h.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            337 /* terrane-site: tests/integration/native-return-parameter-single/case.trn:11:5-11:12 */,
    );
}
// Source: tests/integration/native-scoped-text/case.trn
// Namespace: native-scoped-text
fn parts_text(parts: &TargetParts) -> String {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*parts).text()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::TargetParts::text",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),

            338 /* terrane-site: tests/integration/native-scoped-text/case.trn:6:12-6:23 */,
    );
}
fn target_text(original: &OriginalTarget) -> String {
    let selected: Option<String>;
    selected = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&(*original).0)
                                .parts()
                                .map({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = parts_text(borrowed_argument_0);
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

            339 /* terrane-site: tests/integration/native-scoped-text/case.trn:10:28-10:66 */,
    );
    if (&selected).is_some() {
        return match selected {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    }
    return String::from("/");
}
fn accepted_text(parts: &TargetParts) -> Option<String> {
    let __terrane_completion_0: TerraneCompletion<Option<String>> = (|| {
        let __terrane_try_0: TerraneCompletion<Option<String>> = (|| {
            return TerraneCompletion::Return(
                Some(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { match match
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                        Result < _, crate ::TerraneForeignError > { Ok((&* parts)
                        .checked_text()) })) { Ok(result) => result ?, Err(payload) =>
                        return Err(crate ::__terrane_dependency_panic(payload,
                        "terrane-query-witness",
                        "terrane_query_witness::TargetParts::checked_text")) } {
                        Ok(value) => Ok(value.to_owned()), Err(error) => Err(crate
                        ::TerraneForeignError(crate ::TerraneError::custom_raised(crate
                        ::TERRANE_DEPENDENCY_ERROR,
                        format!("Rust dependency `{}` member `{}` failed: {error}",
                        "terrane-query-witness",
                        "terrane_query_witness::TargetParts::checked_text"), crate
                        ::TERRANE_NO_SITE))) } } (), 340 /* terrane-site: tests/integration/native-scoped-text/case.trn:17:16-17:35 */
                    ),
                ),
            );
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(0))
                {
                    __terrane_handled_0 = true;
                    return TerraneCompletion::Return(None);
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {
            __terrane_generated_defect("non-fallthrough try completed normally")
        }
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn optional_text(original: &OriginalTarget) -> Option<String> {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&(*original).0)
                                .parts()
                                .and_then({
                                    move |borrowed_argument_0| {
                                        match || -> Result<_, crate::TerraneForeignError> {
                                            let callback_value = accepted_text(borrowed_argument_0);
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

            341 /* terrane-site: tests/integration/native-scoped-text/case.trn:24:12-24:58 */,
    );
}
fn test_scenario_terrane_native_scoped_text() {
    let original: OriginalTarget;
    let missing: OriginalTarget;
    let invalid: OriginalTarget;
    let accepted: Option<String>;
    original = __terrane_raised(
        terrane_static_trn_4f726967696e616c546172676574_new("/mounted/path?q=1"),

            342 /* terrane-site: tests/integration/native-scoped-text/case.trn:27:16-27:56 */,
    );
    missing = __terrane_raised(
        terrane_static_trn_4f726967696e616c546172676574_new(""),

            343 /* terrane-site: tests/integration/native-scoped-text/case.trn:28:15-28:38 */,
    );
    invalid = __terrane_raised(
        terrane_static_trn_4f726967696e616c546172676574_new("café"),

            344 /* terrane-site: tests/integration/native-scoped-text/case.trn:29:15-29:42 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&target_text(&original)));
    println!("{}", terrane_scalar_support::scalar_text(&target_text(&missing)));
    accepted = optional_text(&original);
    if (&accepted).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &accepted { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&optional_text(&missing).is_none())
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&optional_text(&invalid).is_none())
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4f726967696e616c546172676574_clones(),
        345 /* terrane-site: tests/integration/native-scoped-text/case.trn:37:13-37:36 */))
    );
}
// Source: tests/integration/native-try-body-rewrite-control/case.trn
// Namespace: native-try-body-rewrite-control
fn test_scenario_terrane_native_try_body_rewrite_control() {
    let h: Holder;
    let mut v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            346 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:4:9-4:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            347 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:5:9-5:16 */,
    );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 348 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:7:17-7:29 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 349 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:8:13-8:20 */
            );
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 350 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:9:17-9:24 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&1));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
// Source: tests/integration/native-try-catch-dominating-rewrite/case.trn
// Namespace: native-try-catch-dominating-rewrite
fn test_scenario_terrane_native_try_catch_dominating_rewrite() {
    let h: Holder;
    let mut v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            351 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:4:9-4:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            352 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:5:9-5:16 */,
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 353 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:7:17-7:29 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 354 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:8:13-8:20 */
            );
            let _ = &v;
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    __terrane_handled_0 = true;
                    v = __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane-query-witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 355 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:10:13-10:20 */
                    );
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                        | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&v).size())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane-query-witness",
                        "terrane_query_witness::View::size")) }; Ok:: < _, crate
                        ::TerraneForeignError >
                        (terrane_int_support::Int::from_u128(value as u128)) } } (),
                        356 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:11:17-11:24 */))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
// Source: tests/integration/native-try-copy-finally-control/case.trn
// Namespace: native-try-copy-finally-control
fn test_scenario_terrane_native_try_copy_finally_control() {
    let h: Holder;
    let k: Holder;
    let mut v: terrane_query_witness::CopyView<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            357 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            358 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:5:9-5:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            359 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:6:9-6:21 */,
    );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.size()) })) { Ok(result) => result
                ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
                "terrane-query-witness", "terrane_query_witness::CopyView::size")) };
                Ok:: < _, crate ::TerraneForeignError >
                (terrane_int_support::Int::from_u128(value as u128)) } } (),
                360 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:8:17-8:24 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&k).copy_view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 361 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:9:13-9:25 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(v.size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 362 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:11:17-11:24 */))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(v.size()) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 363 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:12:13-12:20 */))
    );
}
// Source: tests/integration/native-try-finally-dominating-rewrite/case.trn
// Namespace: native-try-finally-dominating-rewrite
fn test_scenario_terrane_native_try_finally_dominating_rewrite() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            364 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:4:9-4:21 */,
    );
    let _ = v
        .insert(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&h).view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    365 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:5:9-5:16 */,
            ),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.take()
                .expect("flow-proven availability").into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 366 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:7:17-7:29 */))
            );
            let _ = v
                .insert(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane-query-witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 367 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:8:13-8:20 */
                    ),
                );
            let _ = &v;
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        let _ = v
            .insert(
                __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { { let value =
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                    Result < _, crate ::TerraneForeignError > { Ok((&h).view()) })) {
                    Ok(result) => result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane-query-witness",
                    "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 368 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:10:13-10:20 */
                ),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v.as_ref()
            .expect("flow-proven available binding")).size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 369 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:11:17-11:24 */))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
// Source: tests/integration/native-try-finally-rewrite-control/case.trn
// Namespace: native-try-finally-rewrite-control
fn test_scenario_terrane_native_try_finally_rewrite_control() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            370 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:4:9-4:21 */,
    );
    let _ = v
        .insert(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&h).view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    371 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:5:9-5:16 */,
            ),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!("{}", terrane_scalar_support::scalar_text(&0));
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(v.take()
            .expect("flow-proven availability").into_size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::into_size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 372 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:9:17-9:29 */))
        );
        let _ = v
            .insert(
                __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { { let value =
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                    Result < _, crate ::TerraneForeignError > { Ok((&h).view()) })) {
                    Ok(result) => result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane-query-witness",
                    "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 373 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:10:13-10:20 */
                ),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v.as_ref()
            .expect("flow-proven available binding")).size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 374 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:11:17-11:24 */))
        );
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
// Source: tests/integration/native-try-local-consume/case.trn
// Namespace: native-try-local-consume
fn test_scenario_terrane_native_try_local_consume() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    let mut i: terrane_int_support::Int;
    let mut next: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            375 /* terrane-site: tests/integration/native-try-local-consume/case.trn:4:9-4:21 */,
    );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = v
                .insert(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane-query-witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 376 /* terrane-site: tests/integration/native-try-local-consume/case.trn:6:13-6:20 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.take()
                .expect("flow-proven availability").into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 377 /* terrane-site: tests/integration/native-try-local-consume/case.trn:7:17-7:29 */))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&9));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    i = terrane_int_support::Int::from(0_i128);
    while i < terrane_int_support::Int::from(2_i128) {
        let mut __terrane_completion_1: TerraneCompletion<()> = (|| {
            let __terrane_try_1: TerraneCompletion<()> = (|| {
                let _ = next
                    .insert(
                        __terrane_raised_completion!(
                            | | -> Result < _, crate ::TerraneForeignError > { { let
                            value = match
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                            Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                            })) { Ok(result) => result ?, Err(payload) => return
                            Err(crate ::__terrane_dependency_panic(payload,
                            "terrane-query-witness",
                            "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                            ::TerraneForeignError > (value) } } (),
                            378 /* terrane-site: tests/integration/native-try-local-consume/case.trn:13:20-13:27 */
                        ),
                    );
                println!(
                    "{}",
                    terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
                    -> Result < _, crate ::TerraneForeignError > { { let value = match
                    std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result <
                    _, crate ::TerraneForeignError > { Ok(next.take()
                    .expect("flow-proven availability").into_size()) })) { Ok(result) =>
                    result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane-query-witness",
                    "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                    ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                    u128)) } } (), 379 /* terrane-site: tests/integration/native-try-local-consume/case.trn:14:21-14:36 */))
                );
                TerraneCompletion::Normal
            })();
            match __terrane_try_1 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_1) => {
                    let mut __terrane_handled_1 = false;
                    if !__terrane_handled_1 {
                        return TerraneCompletion::Error(__terrane_error_1);
                    }
                }
            }
            TerraneCompletion::Normal
        })();
        let __terrane_finally_1: TerraneCompletion<()> = (|| {
            println!("{}", terrane_scalar_support::scalar_text(&8));
            TerraneCompletion::Normal
        })();
        match __terrane_finally_1 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_1 = replacement,
        }
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break => break,
            TerraneCompletion::Continue => {
                __terrane_generated_defect("non-continuing try produced continue")
            }
        }
        i = &i + &terrane_int_support::Int::from(1_i128);
    }
}
// Source: tests/integration/native-try-unconditional-rewrite-control/case.trn
// Namespace: native-try-unconditional-rewrite-control
fn test_scenario_terrane_native_try_unconditional_rewrite_control() {
    let h: Holder;
    let k: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            380 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            381 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:5:9-5:21 */,
    );
    let _ = v
        .insert(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&h).view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    382 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:6:9-6:16 */,
            ),
        );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.take()
                .expect("flow-proven availability").into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 383 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:8:17-8:29 */))
            );
            let _ = v
                .insert(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane-query-witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 384 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:9:13-9:20 */
                    ),
                );
            let _ = &v;
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&1));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    let _ = v
        .insert(
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&k).view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    385 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:12:9-12:16 */,
            ),
        );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v.as_ref()
        .expect("flow-proven available binding")).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 386 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:13:13-13:20 */))
    );
}
// Source: tests/integration/native-typed-metadata/case.trn
// Namespace: native-typed-metadata
fn own_label(record: &TypedRecord) -> MetadataText {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::MetadataText as core::clone::Clone>::clone(
                                &<terrane_query_witness::TypedRecord as terrane_query_witness::Describe>::raw_label(
                                    &*record,
                                ),
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::MetadataText::clone",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            387 /* terrane-site: tests/integration/native-typed-metadata/case.trn:6:12-6:38 */,
    );
}
fn type_name(record: &TypedRecord) -> String {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::TypedRecord as terrane_query_witness::Describe>::type_name(
                                &*record,
                            ),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::TypedRecord::type_name",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),

            388 /* terrane-site: tests/integration/native-typed-metadata/case.trn:9:12-9:29 */,
    );
}
fn snapshot(
    request: &MetadataRequest,
) -> terrane_query_witness::MetadataTable<terrane_query_witness::MetadataText> {
    let captured: terrane_query_witness::MetadataTable<
        terrane_query_witness::MetadataText,
    >;
    captured = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            <terrane_query_witness::MetadataTable<
                                MetadataText,
                            > as core::clone::Clone>::clone(&(&*request).metadata()),
                        )
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::MetadataTable::clone",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            389 /* terrane-site: tests/integration/native-typed-metadata/case.trn:12:46-12:72 */,
    );
    return captured;
}
fn snapshot_text(
    captured: &terrane_query_witness::MetadataTable<terrane_query_witness::MetadataText>,
) -> String {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*captured).snapshot_text()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::MetadataTable::snapshot_text",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),

            390 /* terrane-site: tests/integration/native-typed-metadata/case.trn:16:12-16:35 */,
    );
}
fn own_found(value: &MetadataText) -> String {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&*value).contents()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::MetadataText::contents",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value.to_owned())
            }
        }(),

            391 /* terrane-site: tests/integration/native-typed-metadata/case.trn:19:12-19:27 */,
    );
}
fn find_text(
    record: std::sync::Arc<TerraneSharedCell<TypedRecord>>,
    key: &String,
) -> Option<String> {
    return {
        let __terrane_owner_1 = &record;
        let __terrane_native_guard_2 = __terrane_owner_1
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_shared_argument_0 = &*key;
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_native_view_798 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(__terrane_native_guard_2.find(__terrane_shared_argument_1)) }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::TypedRecord::find",
                    ),
                )),

                392 /* terrane-site: tests/integration/native-typed-metadata/case.trn:22:13-22:29 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                __terrane_native_view_798
                                    .map({
                                        move |borrowed_argument_0| {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = own_found(borrowed_argument_0);
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

                393 /* terrane-site: tests/integration/native-typed-metadata/case.trn:22:12-22:45 */,
        )
    };
}
fn find_text_aliased(
    record: std::sync::Arc<TerraneSharedCell<TypedRecord>>,
    key: &String,
) -> Option<String> {
    return {
        let __terrane_owner_4 = &record;
        let __terrane_native_guard_5 = __terrane_owner_4
            .lock()
            .expect("shared reference lock poisoned");
        let __terrane_shared_argument_0 = &*key;
        let __terrane_shared_argument_1 = __terrane_shared_argument_0;
        let __terrane_native_view_930 = __terrane_raised(
            std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            __terrane_native_guard_5
                                .find_aliased(__terrane_shared_argument_1),
                        )
                    }),
                )
                .unwrap_or_else(|payload| Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane-query-witness",
                        "terrane_query_witness::TypedRecord::find_aliased",
                    ),
                )),

                394 /* terrane-site: tests/integration/native-typed-metadata/case.trn:25:13-25:37 */,
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                __terrane_native_view_930
                                    .map({
                                        move |borrowed_argument_0| {
                                            match || -> Result<_, crate::TerraneForeignError> {
                                                let callback_value = own_found(borrowed_argument_0);
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

                395 /* terrane-site: tests/integration/native-typed-metadata/case.trn:25:12-25:53 */,
        )
    };
}
fn byte_lengths(value: &MetadataText) {
    println!(
        "{}", terrane_scalar_support::scalar_text(&(__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&* value).contents()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::MetadataText::contents")) };
        Ok:: < _, crate ::TerraneForeignError > (value) } } (), 396 /* terrane-site: tests/integration/native-typed-metadata/case.trn:28:13-28:28 */).len() as i128))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&* value).bytes()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::MetadataText::bytes")) }; Ok:: <
        _, crate ::TerraneForeignError > (value) } } (), 397 /* terrane-site: tests/integration/native-typed-metadata/case.trn:29:13-29:25 */).len() as i128))
    );
}
fn test_scenario_terrane_native_typed_metadata() {
    let record: TypedRecord;
    let owned: MetadataText;
    let request: MetadataRequest;
    let captured: terrane_query_witness::MetadataTable<
        terrane_query_witness::MetadataText,
    >;
    let shared_record: std::sync::Arc<TerraneSharedCell<TypedRecord>>;
    let key: String;
    let found: Option<String>;
    let missing: String;
    let aliased: Option<String>;
    let panicking: String;
    let mut unexpected: Option<Option<String>> = None;
    let mut failure: Option<TerraneError> = None;
    let found_again: Option<String>;
    let multibyte_record: TypedRecord;
    let multibyte: MetadataText;
    record = __terrane_raised(
        terrane_static_trn_54797065645265636f7264_new("record label"),

            398 /* terrane-site: tests/integration/native-typed-metadata/case.trn:32:14-32:46 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&type_name(&record)));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4d6574616461746154657874_copies(),
        399 /* terrane-site: tests/integration/native-typed-metadata/case.trn:34:13-34:34 */))
    );
    owned = own_label(&record);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&owned).contents()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::MetadataText::contents")) };
        Ok:: < _, crate ::TerraneForeignError > (value.to_owned()) } } (),
        400 /* terrane-site: tests/integration/native-typed-metadata/case.trn:36:13-36:28 */))
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4d6574616461746154657874_copies(),
        401 /* terrane-site: tests/integration/native-typed-metadata/case.trn:37:13-37:34 */))
    );
    request = __terrane_raised(
        terrane_static_trn_4d6574616461746152657175657374_new("snapshot label"),

            402 /* terrane-site: tests/integration/native-typed-metadata/case.trn:38:15-38:53 */,
    );
    captured = snapshot(&request);
    println!("{}", terrane_scalar_support::scalar_text(&snapshot_text(&captured)));
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4d6574616461746154657874_copies(),
        403 /* terrane-site: tests/integration/native-typed-metadata/case.trn:41:13-41:34 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        false; let __terrane_native_view_1495 =
        __terrane_raised(std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
        Result < _, crate ::TerraneForeignError > { Ok((&record)
        .metadata(__terrane_conversion_input_0)) })).unwrap_or_else(| payload | Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::TypedRecord::metadata"))), 404 /* terrane-site: tests/integration/native-typed-metadata/case.trn:42:14-42:36 */); { let
        __terrane_native_input_0 = &__terrane_native_view_1495; __terrane_raised(| | ->
        Result < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(< terrane_query_witness::MetadataText >
        ::contents(__terrane_native_input_0)) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::MetadataText::contents")) };
        Ok:: < _, crate ::TerraneForeignError > (value.to_owned()) } } (),
        405 /* terrane-site: tests/integration/native-typed-metadata/case.trn:42:13-42:47 */) } })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_0 =
        true; let __terrane_native_view_1543 =
        __terrane_raised(std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
        Result < _, crate ::TerraneForeignError > { Ok((&record)
        .metadata(__terrane_conversion_input_0)) })).unwrap_or_else(| payload | Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::TypedRecord::metadata"))), 406 /* terrane-site: tests/integration/native-typed-metadata/case.trn:43:14-43:35 */); { let
        __terrane_native_input_0 = &__terrane_native_view_1543; __terrane_raised(| | ->
        Result < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(< terrane_query_witness::MetadataText >
        ::contents(__terrane_native_input_0)) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::MetadataText::contents")) };
        Ok:: < _, crate ::TerraneForeignError > (value.to_owned()) } } (),
        407 /* terrane-site: tests/integration/native-typed-metadata/case.trn:43:13-43:46 */) } })
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4d6574616461746154657874_copies(),
        408 /* terrane-site: tests/integration/native-typed-metadata/case.trn:44:13-44:34 */))
    );
    shared_record = std::sync::Arc::new(
        TerraneSharedCell::new(
            __terrane_raised(
                terrane_static_trn_54797065645265636f7264_new("record label"),

                    409 /* terrane-site: tests/integration/native-typed-metadata/case.trn:45:33-45:65 */,
            ),
        ),
    );
    key = String::from("record label");
    found = find_text(shared_record.clone(), &key);
    if (&found).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    missing = String::from("other");
    println!(
        "{}", terrane_scalar_support::scalar_text(&find_text(shared_record.clone(),
        &missing).is_none())
    );
    aliased = find_text_aliased(shared_record.clone(), &key);
    if (&aliased).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &aliased { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&find_text_aliased(shared_record
        .clone(), &missing).is_none())
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_4d6574616461746154657874_copies(),
        410 /* terrane-site: tests/integration/native-typed-metadata/case.trn:56:13-56:34 */))
    );
    panicking = String::from("panic");
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            let _ = unexpected
                .insert({
                    let __terrane_owner_7 = &shared_record;
                    let __terrane_native_guard_8 = __terrane_owner_7
                        .lock()
                        .expect("shared reference lock poisoned");
                    let __terrane_shared_argument_0 = &panicking;
                    let __terrane_shared_argument_1 = __terrane_shared_argument_0;
                    let __terrane_native_view_2126 = __terrane_raised_completion!(
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                        Result < _, crate ::TerraneForeignError > {
                        Ok(__terrane_native_guard_8.find(__terrane_shared_argument_1))
                        })).unwrap_or_else(| payload | Err(crate
                        ::__terrane_dependency_panic(payload, "terrane-query-witness",
                        "terrane_query_witness::TypedRecord::find"))),
                        411 /* terrane-site: tests/integration/native-typed-metadata/case.trn:59:35-59:64 */
                    );
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > {
                        Ok(__terrane_native_view_2126.map({ move | borrowed_argument_0 |
                        { match | | -> Result < _, crate ::TerraneForeignError > { let
                        callback_value = own_found(borrowed_argument_0);
                        Ok(callback_value) } () { Ok(value) => value, Err(error) =>
                        std::panic::panic_any(error.0) } } })) })) { Ok(result) => result
                        ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "std",
                        "std::option::Option::map")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 412 /* terrane-site: tests/integration/native-typed-metadata/case.trn:59:34-59:80 */
                    )
                });
            println!(
                "{}", terrane_scalar_support::scalar_text(&unexpected.as_ref()
                .expect("flow-proven available binding").is_none())
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(1))
                {
                    __terrane_handled_0 = true;
                    let _ = failure.insert(__terrane_error_0.clone());
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&{ let
                        __terrane_error_text_9 = failure.as_ref()
                        .expect("flow-proven available binding").message(); let
                        __terrane_string_argument_0 = "TypedRecord::find";
                        __terrane_error_text_9.contains(std::convert::AsRef:: < str >
                        ::as_ref(&__terrane_string_argument_0)) })
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
    found_again = find_text(shared_record.clone(), &key);
    if (&found_again).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &found_again { Some(value)
            => value, _ => unreachable!("flow-proven storage refinement") })
        );
    }
    multibyte_record = __terrane_raised(
        terrane_static_trn_54797065645265636f7264_new("café"),

            413 /* terrane-site: tests/integration/native-typed-metadata/case.trn:66:24-66:48 */,
    );
    multibyte = own_label(&multibyte_record);
    byte_lengths(&multibyte);
}
// Source: tests/integration/native-unordered-map-dead-lender-control/case.trn
// Namespace: native-unordered-map-dead-lender-control
fn test_scenario_terrane_native_unordered_map_dead_lender_control() {
    let mut holder: Holder;
    let mut unrelated: Holder;
    let mut values: terrane_collection_support::UnorderedMap<String, CopyView>;
    let mut key: String;
    let mut value: CopyView;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            414 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:6:14-6:26 */,
    );
    unrelated = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            415 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:7:17-7:29 */,
    );
    values = terrane_collection_support::UnorderedMap::<String, CopyView>::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("view");
        let __terrane_collection_argument_1 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&holder).copy_view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                416 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:9:26-9:43 */,
        );
        values.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_0 = values;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_0.key;
        let _ = &key;
        value = __terrane_item_0.value;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 417 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:11:17-11:28 */))
        );
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let _ = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok({
                                let _ = unrelated.bump();
                            })
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::bump",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),

                418 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:12:9-12:24 */,
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
                            let _ = holder.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            419 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:13:5-13:17 */,
    );
}
// Source: tests/integration/native-view-list-rebind/case.trn
// Namespace: native-view-list-rebind
#[derive(Clone)]
pub struct Err {
    pub message: String,
}
impl Err {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("failure"),
        }
    }
    pub fn render(&self) -> String {
        return self.message.clone();
    }
}
fn test_scenario_terrane_native_view_list_rebind() {
    let h: Holder;
    let mut v: terrane_query_witness::View<'_>;
    let mut values: terrane_collection_support::List<terrane_query_witness::View<'_>>;
    let no_read_holder: Holder;
    let mut no_read: terrane_query_witness::View<'_>;
    let mut no_read_list: terrane_collection_support::List<
        terrane_query_witness::View<'_>,
    >;
    let a: Holder;
    let mut old: terrane_query_witness::View<'_>;
    let mut other_values: terrane_collection_support::List<
        terrane_query_witness::View<'_>,
    >;
    let other: Holder;
    let mut fresh: terrane_query_witness::View<'_>;
    let literal: terrane_collection_support::List<terrane_query_witness::View<'_>>;
    let mut alias_value: terrane_query_witness::View<'_>;
    let alias_values: terrane_collection_support::List<terrane_query_witness::View<'_>>;
    let catch_holder: Holder;
    let other_lender: Holder;
    let mut unrelated: Holder;
    let mut catch_view: terrane_query_witness::CopyView<'_>;
    let mut catch_alias: terrane_query_witness::CopyView<'_>;
    let safe_holder: Holder;
    let safe_k: Holder;
    let mut safe_other: Holder;
    let mut safe_view: terrane_query_witness::CopyView<'_>;
    let mut safe_alias: terrane_query_witness::CopyView<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            420 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:12:9-12:21 */,
    );
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            421 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:13:9-13:16 */,
    );
    values = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::View < '_ > = __terrane_raised(| | ->
            Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&h).view()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::Holder::view")) }; Ok:: < _,
            crate ::TerraneForeignError > (value) } } (), 422 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:14:21-14:28 */); value }
        ],
    );
    {
        let __terrane_collection_argument_0 = v;
        values.push_unique(__terrane_collection_argument_0);
    };
    v = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&h).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            423 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:16:9-16:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 424 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:17:13-17:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    no_read_holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            425 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:19:22-19:34 */,
    );
    no_read = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&no_read_holder).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            426 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:20:15-20:35 */,
    );
    no_read_list = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::View < '_ > = __terrane_raised(| | ->
            Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&no_read_holder).view()) })) { Ok(result)
            => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 427 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:21:27-21:47 */); value }
        ],
    );
    {
        let __terrane_collection_argument_0 = no_read;
        no_read_list.push_unique(__terrane_collection_argument_0);
    };
    no_read = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&no_read_holder).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            428 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:23:15-23:35 */,
    );
    let _ = &no_read;
    a = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            429 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:24:9-24:21 */,
    );
    old = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&a).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            430 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:25:11-25:18 */,
    );
    other_values = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::View < '_ > = __terrane_raised(| | ->
            Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&a).view()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::Holder::view")) }; Ok:: < _,
            crate ::TerraneForeignError > (value) } } (), 431 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:26:27-26:34 */); value }
        ],
    );
    {
        let __terrane_collection_argument_0 = old;
        other_values.push_unique(__terrane_collection_argument_0);
    };
    other = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            432 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:28:13-28:25 */,
    );
    old = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&other).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            433 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:29:11-29:22 */,
    );
    let _ = &old;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(other_values
        .length()))
    );
    fresh = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&a).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            434 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:31:13-31:20 */,
    );
    literal = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(vec![{ let value : terrane_query_witness::View <'_ > = fresh; value }]);
    fresh = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&a).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            435 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:33:13-33:20 */,
    );
    let _ = &fresh;
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(literal
        .length()))
    );
    alias_value = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&a).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            436 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:35:19-35:26 */,
    );
    alias_values = terrane_collection_support::List::<
        terrane_query_witness::View<'_>,
    >::new(vec![{ let value : terrane_query_witness::View <'_ > = alias_value; value }]);
    alias_value = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&other).view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            437 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:37:19-37:30 */,
    );
    let _ = &alias_value;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(alias_values
        .length()))
    );
    catch_holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            438 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:39:20-39:32 */,
    );
    other_lender = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            439 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:40:20-40:32 */,
    );
    unrelated = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            440 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:41:17-41:29 */,
    );
    catch_view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&catch_holder).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            441 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:42:18-42:41 */,
    );
    catch_alias = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&catch_holder).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            442 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:43:19-43:42 */,
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            catch_view = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&other_lender).copy_view()) })) {
                Ok(result) => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 443 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:45:22-45:45 */
            );
            return TerraneCompletion::Error({
                let value = Err::terrane_construct();
                let rendered = value.render();
                let original = &value.message;
                let (message, rendered) = if &rendered == original {
                    (rendered, None)
                } else {
                    (original.clone(), Some(rendered))
                };
                TerraneError::raised_with_rendered_message(
                    TerraneErrorKind::Custom(DescriptorId(2)),
                    message,
                    rendered,

                        444 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:46:9-46:28 */,
                )
            });
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind
                        == TerraneErrorKind::Custom(DescriptorId(2))
                {
                    __terrane_handled_0 = true;
                    catch_alias = catch_view;
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
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
                            let _ = unrelated.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            445 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:49:5-49:20 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(catch_alias.size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 446 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:50:13-50:30 */))
    );
    safe_holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            447 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:51:19-51:31 */,
    );
    safe_k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            448 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:52:14-52:26 */,
    );
    safe_other = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            449 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:53:18-53:30 */,
    );
    safe_view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&safe_holder).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            450 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:54:17-54:39 */,
    );
    safe_alias = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&safe_holder).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            451 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:55:18-55:40 */,
    );
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            safe_view = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&safe_k).copy_view()) })) {
                Ok(result) => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 452 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:57:21-57:38 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_1 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_1) => {
                let mut __terrane_handled_1 = false;
                if !__terrane_handled_1 {
                    __terrane_handled_1 = true;
                    safe_alias = safe_view;
                }
                if !__terrane_handled_1 {
                    return TerraneCompletion::Error(__terrane_error_1);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_1 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
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
                            let _ = safe_other.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            453 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:60:5-60:21 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(safe_alias.size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 454 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:61:13-61:29 */))
    );
}
// Source: tests/integration/native-view-local-map-storage/case.trn
// Namespace: native-view-local-map-storage
fn store(mut holder: Holder) {
    let view: terrane_query_witness::CopyView<'_>;
    let mut values: terrane_collection_support::Map<
        String,
        terrane_query_witness::CopyView<'_>,
    >;
    let map_alias: &mut terrane_collection_support::Map<
        String,
        terrane_query_witness::CopyView<'_>,
    >;
    let mut unordered: terrane_collection_support::UnorderedMap<String, CopyView>;
    let unordered_alias: &mut terrane_collection_support::UnorderedMap<String, CopyView>;
    let mut nested: terrane_collection_support::Map<
        String,
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >;
    let mut items: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&holder).copy_view()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            455 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:6:12-6:29 */,
    );
    values = terrane_collection_support::Map::<
        String,
        terrane_query_witness::CopyView<'_>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("view");
        let __terrane_collection_argument_1 = view;
        values.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    map_alias = &mut values;
    {
        let __terrane_collection_argument_0 = String::from("alias");
        let __terrane_collection_argument_1 = view;
        map_alias.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values.get_or_error(&String::from("alias")),
        456 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:11:13-11:28 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 457 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:11:13-11:34 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values.get_or_error(&String::from("view")),
        458 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:12:13-12:27 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 459 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:12:13-12:33 */) })
    );
    unordered = terrane_collection_support::UnorderedMap::<
        String,
        CopyView,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("view");
        let __terrane_collection_argument_1 = view.clone();
        unordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    unordered_alias = &mut unordered;
    {
        let __terrane_collection_argument_0 = String::from("alias");
        let __terrane_collection_argument_1 = view.clone();
        unordered_alias
            .set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(unordered.get_or_error(&String::from("alias")),
        460 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:17:13-17:31 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 461 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:17:13-17:37 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(unordered.get_or_error(&String::from("view")),
        462 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:18:13-18:30 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 463 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:18:13-18:36 */) })
    );
    nested = terrane_collection_support::Map::<
        String,
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("view");
        let __terrane_collection_argument_1 = terrane_collection_support::List::<
            terrane_query_witness::CopyView<'_>,
        >::new(
            vec![{ let value : terrane_query_witness::CopyView <'_ > = view; value }],
        );
        nested.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(__terrane_raised(nested.get_or_error(&String::from("view")),
        464 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:21:13-21:27 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        465 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:21:13-21:30 */)),
        465 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:21:13-21:30 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 466 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:21:13-21:36 */) })
    );
    items = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = view;
        items.push_unique(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(items
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        467 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:24:13-24:21 */)),
        467 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:24:13-24:21 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane-query-witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 468 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:24:13-24:27 */) })
    );
    __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let _ = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok({
                            let _ = holder.bump();
                        })
                    }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            469 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:25:5-25:17 */,
    );
}
fn test_scenario_terrane_native_view_local_map_storage() {
    store(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                470 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:28:13-28:25 */,
        ),
    );
}
// Source: tests/integration/native-view-loop-binders/case.trn
// Namespace: native-view-loop-binders
fn test_scenario_terrane_native_view_loop_binders() {
    let holder: Holder;
    let values: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let mut value: &terrane_query_witness::CopyView<'_>;
    let mut value_2: terrane_query_witness::CopyView<'_>;
    let mut value_3: &terrane_query_witness::CopyView<'_>;
    let mut value_4: &terrane_query_witness::CopyView<'_>;
    let mut outer: terrane_query_witness::CopyView<'_>;
    let mut value_5: &terrane_query_witness::CopyView<'_>;
    let numbers: terrane_collection_support::List<terrane_int_support::Int>;
    let mut number: &terrane_int_support::Int;
    let mut number_2: terrane_int_support::Int;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            471 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:6:14-6:26 */,
    );
    values = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 472 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:7:21-7:38 */); value }
        ],
    );
    let __terrane_iterable_0 = &values;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((* value).size()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 473 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:9:17-9:28 */))
        );
    }
    let __terrane_iterable_1 = &values;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        value_2 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value_2.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 474 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:11:17-11:28 */))
        );
    }
    let __terrane_iterable_2 = &values;
    let mut __terrane_iterator_2 = __terrane_iterable_2.terrane_borrowing_iterator();
    loop {
        value_3 = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((* value_3).size()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 475 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:13:17-13:28 */))
        );
    }
    let __terrane_iterable_3 = &values;
    let mut __terrane_iterator_3 = __terrane_iterable_3.terrane_borrowing_iterator();
    loop {
        value_4 = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((* value_4).size()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 476 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:15:17-15:28 */))
        );
    }
    let __terrane_iterable_4 = &values;
    let mut __terrane_iterator_4 = terrane_collection_support::slice_iterator(
        __terrane_iterable_4.as_slice(),
    );
    loop {
        outer = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_iterable_5 = &values;
        let mut __terrane_iterator_5 = __terrane_iterable_5.terrane_borrowing_iterator();
        loop {
            value_5 = match __terrane_iterator_5.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((* value_5).size()) })) { Ok(result)
                => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 477 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:18:21-18:32 */))
            );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(outer.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 478 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:19:17-19:28 */))
        );
    }
    numbers = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_6 = &numbers;
    let mut __terrane_iterator_6 = __terrane_iterable_6.terrane_borrowing_iterator();
    loop {
        number = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&* number));
    }
    let __terrane_iterable_7 = numbers;
    let mut __terrane_iterator_7 = terrane_collection_support::slice_iterator(
        __terrane_iterable_7.as_slice(),
    );
    loop {
        number_2 = match __terrane_iterator_7.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&number_2));
    }
}
// Source: tests/integration/native-view-map-entry-iteration/case.trn
// Namespace: native-view-map-entry-iteration
fn test_scenario_terrane_native_view_map_entry_iteration() {
    let holder: Holder;
    let mut ordered: terrane_collection_support::Map<
        String,
        terrane_query_witness::CopyView<'_>,
    >;
    let mut key: String;
    let mut value: terrane_query_witness::CopyView<'_>;
    let mut unordered: terrane_collection_support::UnorderedMap<String, CopyView>;
    let mut value_2: CopyView;
    let mut ordered_inner: terrane_collection_support::Map<
        String,
        terrane_query_witness::CopyView<'_>,
    >;
    let mut ordered_outer: terrane_collection_support::Map<
        String,
        terrane_collection_support::Map<String, terrane_query_witness::CopyView<'_>>,
    >;
    let mut outer_key: String;
    let mut inner: terrane_collection_support::Map<
        String,
        terrane_query_witness::CopyView<'_>,
    >;
    let mut inner_key: String;
    let mut value_3: terrane_query_witness::CopyView<'_>;
    let mut unordered_inner: terrane_collection_support::UnorderedMap<String, CopyView>;
    let mut unordered_outer: terrane_collection_support::UnorderedMap<
        String,
        terrane_collection_support::UnorderedMap<String, CopyView>,
    >;
    let mut inner_2: terrane_collection_support::UnorderedMap<String, CopyView>;
    let mut value_4: CopyView;
    let mut value_5: terrane_query_witness::CopyView<'_>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            479 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:6:14-6:26 */,
    );
    ordered = terrane_collection_support::Map::<
        String,
        terrane_query_witness::CopyView<'_>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("ordered");
        let __terrane_collection_argument_1 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&holder).copy_view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                480 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:8:30-8:47 */,
        );
        ordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_0 = ordered.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        let __terrane_item_0 = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_0.key;
        let _ = &key;
        value = __terrane_item_0.value;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 481 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:10:17-10:28 */))
        );
    }
    unordered = terrane_collection_support::UnorderedMap::<
        String,
        CopyView,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("unordered");
        let __terrane_collection_argument_1 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&holder).copy_view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                482 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:13:34-13:51 */,
        );
        unordered.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_1 = unordered;
    let mut __terrane_iterator_1 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_1,
    );
    loop {
        let __terrane_item_1 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_1.key;
        let _ = &key;
        value_2 = __terrane_item_1.value;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value_2.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 483 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:15:17-15:28 */))
        );
    }
    ordered_inner = terrane_collection_support::Map::<
        String,
        terrane_query_witness::CopyView<'_>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("nested");
        let __terrane_collection_argument_1 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&holder).copy_view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                484 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:18:35-18:52 */,
        );
        ordered_inner
            .set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    ordered_outer = terrane_collection_support::Map::<
        String,
        terrane_collection_support::Map<String, terrane_query_witness::CopyView<'_>>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("outer");
        let __terrane_collection_argument_1 = ordered_inner;
        ordered_outer
            .set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_2 = ordered_outer;
    let mut __terrane_iterator_2 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_2,
    );
    loop {
        let __terrane_item_2 = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        outer_key = __terrane_item_2.key;
        let _ = &outer_key;
        inner = __terrane_item_2.value;
        let __terrane_iterable_3 = inner;
        let mut __terrane_iterator_3 = terrane_collection_support::Iterable::terrane_iterator(
            &__terrane_iterable_3,
        );
        loop {
            let __terrane_item_3 = match __terrane_iterator_3.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            inner_key = __terrane_item_3.key;
            let _ = &inner_key;
            value_3 = __terrane_item_3.value;
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(value_3.size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 485 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:23:21-23:32 */))
            );
        }
    }
    unordered_inner = terrane_collection_support::UnorderedMap::<
        String,
        CopyView,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("nested");
        let __terrane_collection_argument_1 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&holder).copy_view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                486 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:26:37-26:54 */,
        );
        unordered_inner
            .set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    unordered_outer = terrane_collection_support::UnorderedMap::<
        String,
        terrane_collection_support::UnorderedMap<String, CopyView>,
    >::new(vec![]);
    {
        let __terrane_collection_argument_0 = String::from("outer");
        let __terrane_collection_argument_1 = unordered_inner;
        unordered_outer
            .set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    let __terrane_iterable_4 = unordered_outer;
    let mut __terrane_iterator_4 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_4,
    );
    loop {
        let __terrane_item_4 = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        outer_key = __terrane_item_4.key;
        let _ = &outer_key;
        inner_2 = __terrane_item_4.value;
        let __terrane_iterable_5 = inner_2;
        let mut __terrane_iterator_5 = terrane_collection_support::Iterable::terrane_iterator(
            &__terrane_iterable_5,
        );
        loop {
            let __terrane_item_5 = match __terrane_iterator_5.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            inner_key = __terrane_item_5.key;
            let _ = &inner_key;
            value_4 = __terrane_item_5.value;
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(value_4.size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 487 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:31:21-31:32 */))
            );
        }
    }
    let __terrane_iterable_6 = ordered.clone();
    let mut __terrane_iterator_6 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_6,
    );
    loop {
        let __terrane_item_6 = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        key = __terrane_item_6.key;
        let _ = &key;
        value_5 = __terrane_item_6.value;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value_5.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 488 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:34:17-34:28 */))
        );
        {
            let __terrane_collection_argument_0 = String::from("later");
            let __terrane_collection_argument_1 = __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&holder).copy_view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::copy_view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    489 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:35:32-35:49 */,
            );
            ordered
                .set(__terrane_collection_argument_0, __terrane_collection_argument_1);
        };
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(ordered
        .length()))
    );
}
// Source: tests/integration/native-view-tuple-iteration/case.trn
// Namespace: native-view-tuple-iteration
fn test_scenario_terrane_native_view_tuple_iteration() {
    let holder: Holder;
    let values: terrane_collection_support::Tuple<terrane_query_witness::CopyView<'_>>;
    let mut value: terrane_query_witness::CopyView<'_>;
    let mut value_2: terrane_query_witness::CopyView<'_>;
    let inferred: terrane_collection_support::Tuple<terrane_query_witness::CopyView<'_>>;
    let mut value_3: terrane_query_witness::CopyView<'_>;
    let mut value_4: &terrane_query_witness::CopyView<'_>;
    let mut value_5: terrane_query_witness::CopyView<'_>;
    let numbers: terrane_collection_support::Tuple<terrane_int_support::Int>;
    let mut number: terrane_int_support::Int;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            490 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:6:14-6:26 */,
    );
    values = terrane_collection_support::Tuple::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 491 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:7:40-7:57 */); value }
        ],
    );
    let __terrane_iterable_0 = &values;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 492 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:9:17-9:28 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    let __terrane_iterable_1 = &values;
    let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
        __terrane_iterable_1.as_slice(),
    );
    loop {
        value_2 = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value_2.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 493 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:12:17-12:28 */))
        );
    }
    inferred = terrane_collection_support::Tuple::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 494 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:13:24-13:41 */); value
            }
        ],
    );
    let __terrane_iterable_2 = &inferred;
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
        __terrane_iterable_2.as_slice(),
    );
    loop {
        value_3 = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value_3.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 495 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:15:17-15:28 */))
        );
    }
    let __terrane_iterable_3 = &inferred;
    let mut __terrane_iterator_3 = __terrane_iterable_3.terrane_borrowing_iterator();
    loop {
        value_4 = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((* value_4).size()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 496 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:17:17-17:28 */))
        );
    }
    let __terrane_iterable_4 = &terrane_collection_support::Tuple::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 497 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:18:27-18:44 */)
        ],
    );
    let mut __terrane_iterator_4 = terrane_collection_support::slice_iterator(
        __terrane_iterable_4.as_slice(),
    );
    loop {
        value_5 = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value_5.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 498 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:19:17-19:28 */))
        );
    }
    numbers = terrane_collection_support::Tuple::<
        terrane_int_support::Int,
    >::new(
        vec![
            terrane_int_support::Int::from(1_i128),
            terrane_int_support::Int::from(2_i128)
        ],
    );
    let __terrane_iterable_5 = numbers.clone();
    let mut __terrane_iterator_5 = terrane_collection_support::slice_iterator(
        __terrane_iterable_5.as_slice(),
    );
    loop {
        number = match __terrane_iterator_5.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&number));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(numbers
        .length()))
    );
}
// Source: tests/integration/native-view-value-iteration/case.trn
// Namespace: native-view-value-iteration
fn observe_terrane_native_view_value_iteration(
    values: terrane_collection_support::List<CopyView>,
) {
    let mut value: CopyView;
    let __terrane_iterable_0 = values.clone();
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 499 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:7:17-7:28 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
fn observe_ref(values: &terrane_collection_support::List<CopyView>) {
    let mut value: &CopyView;
    let __terrane_iterable_1 = values;
    let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_1.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.clone().size()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 500 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:12:17-12:28 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from((*
        values).length()))
    );
}
fn test_scenario_terrane_native_view_value_iteration() {
    let holder: Holder;
    let mut typed: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let mut value: terrane_query_witness::CopyView<'_>;
    let mut inferred: terrane_collection_support::List<
        terrane_query_witness::CopyView<'_>,
    >;
    let mut numbers: terrane_collection_support::List<terrane_int_support::Int>;
    let mut number: terrane_int_support::Int;
    let words: terrane_collection_support::List<String>;
    let mut word: String;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            501 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:16:14-16:26 */,
    );
    typed = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 502 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:17:37-17:54 */); value
            }
        ],
    );
    let __terrane_iterable_2 = &typed;
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
        __terrane_iterable_2.as_slice(),
    );
    loop {
        value = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 503 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:19:17-19:28 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(typed
            .length()))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(typed
        .length()))
    );
    let __terrane_iterable_3 = &typed;
    let mut __terrane_iterator_3 = terrane_collection_support::slice_iterator(
        __terrane_iterable_3.as_slice(),
    );
    loop {
        value = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 504 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:23:17-23:28 */))
        );
    }
    inferred = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 505 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:24:23-24:40 */); value
            }
        ],
    );
    let __terrane_iterable_4 = &inferred;
    let mut __terrane_iterator_4 = terrane_collection_support::slice_iterator(
        __terrane_iterable_4.as_slice(),
    );
    loop {
        value = match __terrane_iterator_4.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 506 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:26:17-26:28 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(inferred
            .length()))
        );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(inferred
        .length()))
    );
    let __terrane_iterable_5 = {
        terrane_collection_support::List::new((&inferred).as_slice().to_vec())
    };
    let mut __terrane_iterator_5 = terrane_collection_support::slice_iterator(
        __terrane_iterable_5.as_slice(),
    );
    loop {
        value = match __terrane_iterator_5.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        {
            let __terrane_collection_argument_0 = __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&holder).copy_view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::copy_view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    507 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:30:27-30:44 */,
            );
            inferred.push_unique(__terrane_collection_argument_0);
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 508 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:31:17-31:28 */))
        );
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(inferred
        .length()))
    );
    observe_terrane_native_view_value_iteration(typed.clone());
    observe_ref(&typed);
    let __terrane_iterable_6 = {
        terrane_collection_support::List::new((&typed).as_slice().to_vec())
    };
    let mut __terrane_iterator_6 = terrane_collection_support::slice_iterator(
        __terrane_iterable_6.as_slice(),
    );
    loop {
        value = match __terrane_iterator_6.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        {
            let __terrane_collection_argument_0 = __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let value = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > { Ok((&holder).copy_view()) }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane-query-witness",
                                        "terrane_query_witness::Holder::copy_view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    509 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:36:24-36:41 */,
            );
            typed.push_unique(__terrane_collection_argument_0);
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane-query-witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 510 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:37:17-37:28 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(typed
        .length()))
    );
    numbers = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    let __terrane_iterable_7 = numbers.clone();
    let mut __terrane_iterator_7 = terrane_collection_support::slice_iterator(
        __terrane_iterable_7.as_slice(),
    );
    loop {
        number = match __terrane_iterator_7.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        {
            let __terrane_collection_argument_0 = terrane_int_support::Int::from(2_i128);
            numbers.append(__terrane_collection_argument_0);
        };
        println!("{}", terrane_scalar_support::scalar_text(&number));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(numbers
        .length()))
    );
    words = terrane_collection_support::List::<String>::new(vec![String::from("word")]);
    let __terrane_iterable_8 = words.clone();
    let mut __terrane_iterator_8 = terrane_collection_support::slice_iterator(
        __terrane_iterable_8.as_slice(),
    );
    loop {
        word = match __terrane_iterator_8.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&word));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(words
        .length()))
    );
}
// Source: tests/integration/nested-noncopy-view-loops/case.trn
// Namespace: nested-noncopy-view-loops
fn test_scenario_terrane_nested_noncopy_view_loops() {
    let holder: Holder;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_query_witness::View<'_>>,
    >;
    let mut outer: &terrane_collection_support::List<terrane_query_witness::View<'_>>;
    let mut inner: &terrane_query_witness::View<'_>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            511 /* terrane-site: tests/integration/nested-noncopy-view-loops/case.trn:6:14-6:26 */,
    );
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_query_witness::View<'_>>,
    >::new(
        vec![
            terrane_collection_support::List:: < terrane_query_witness::View < '_ > >
            ::new(vec![{ let value : terrane_query_witness::View < '_ > =
            __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&holder).view()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 512 /* terrane-site: tests/integration/nested-noncopy-view-loops/case.trn:7:28-7:40 */); value }])
        ],
    );
    let __terrane_iterable_0 = &nested;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        outer = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_iterable_1 = outer;
        let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
        loop {
            inner = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&* inner).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::View::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 513 /* terrane-site: tests/integration/nested-noncopy-view-loops/case.trn:10:21-10:32 */))
            );
        }
    }
}
// Source: tests/integration/nested-view-loops/case.trn
// Namespace: nested-view-loops
fn test_scenario_terrane_nested_view_loops() {
    let holder: Holder;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >;
    let mut outer: &terrane_collection_support::List<
        terrane_query_witness::CopyView<'_>,
    >;
    let mut inner: &terrane_query_witness::CopyView<'_>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),
        514 /* terrane-site: tests/integration/nested-view-loops/case.trn:6:14-6:26 */,
    );
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >::new(
        vec![
            terrane_collection_support::List:: < terrane_query_witness::CopyView < '_ > >
            ::new(vec![{ let value : terrane_query_witness::CopyView < '_ > =
            __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 515 /* terrane-site: tests/integration/nested-view-loops/case.trn:7:28-7:45 */); value }])
        ],
    );
    let __terrane_iterable_0 = &nested;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        outer = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_iterable_1 = outer;
        let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
        loop {
            inner = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((* inner).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 516 /* terrane-site: tests/integration/nested-view-loops/case.trn:10:21-10:32 */))
            );
        }
    }
}
// Source: tests/integration/nested-view-ref-value-loop/case.trn
// Namespace: nested-view-ref-value-loop
fn test_scenario_terrane_nested_view_ref_value_loop() {
    let holder: Holder;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >;
    let mut outer: &terrane_collection_support::List<
        terrane_query_witness::CopyView<'_>,
    >;
    let mut inner: &terrane_query_witness::CopyView<'_>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            517 /* terrane-site: tests/integration/nested-view-ref-value-loop/case.trn:6:14-6:26 */,
    );
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >::new(
        vec![
            terrane_collection_support::List:: < terrane_query_witness::CopyView < '_ > >
            ::new(vec![{ let value : terrane_query_witness::CopyView < '_ > =
            __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 518 /* terrane-site: tests/integration/nested-view-ref-value-loop/case.trn:7:28-7:45 */); value
            }])
        ],
    );
    let __terrane_iterable_0 = &nested;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        outer = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_iterable_1 = outer;
        let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
        loop {
            inner = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((* inner).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 519 /* terrane-site: tests/integration/nested-view-ref-value-loop/case.trn:10:21-10:32 */))
            );
        }
    }
}
// Source: tests/integration/nested-view-value-ref-loop/case.trn
// Namespace: nested-view-value-ref-loop
fn test_scenario_terrane_nested_view_value_ref_loop() {
    let holder: Holder;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >;
    let mut outer: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let mut inner: &terrane_query_witness::CopyView<'_>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            520 /* terrane-site: tests/integration/nested-view-value-ref-loop/case.trn:6:14-6:26 */,
    );
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >::new(
        vec![
            terrane_collection_support::List:: < terrane_query_witness::CopyView < '_ > >
            ::new(vec![{ let value : terrane_query_witness::CopyView < '_ > =
            __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 521 /* terrane-site: tests/integration/nested-view-value-ref-loop/case.trn:7:28-7:45 */); value
            }])
        ],
    );
    let __terrane_iterable_0 = nested;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        outer = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_iterable_1 = &outer;
        let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
        loop {
            inner = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((* inner).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 522 /* terrane-site: tests/integration/nested-view-value-ref-loop/case.trn:10:21-10:32 */))
            );
        }
    }
}
// Source: tests/integration/nested-view-value-value-loop/case.trn
// Namespace: nested-view-value-value-loop
fn test_scenario_terrane_nested_view_value_value_loop() {
    let holder: Holder;
    let nested: terrane_collection_support::List<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >;
    let mut outer: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let mut inner: terrane_query_witness::CopyView<'_>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            523 /* terrane-site: tests/integration/nested-view-value-value-loop/case.trn:6:14-6:26 */,
    );
    nested = terrane_collection_support::List::<
        terrane_collection_support::List<terrane_query_witness::CopyView<'_>>,
    >::new(
        vec![
            terrane_collection_support::List:: < terrane_query_witness::CopyView < '_ > >
            ::new(vec![{ let value : terrane_query_witness::CopyView < '_ > =
            __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let
            value = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
            Result < _, crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 524 /* terrane-site: tests/integration/nested-view-value-value-loop/case.trn:7:28-7:45 */); value
            }])
        ],
    );
    let __terrane_iterable_0 = nested;
    let mut __terrane_iterator_0 = terrane_collection_support::slice_iterator(
        __terrane_iterable_0.as_slice(),
    );
    loop {
        outer = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let __terrane_iterable_1 = &outer;
        let mut __terrane_iterator_1 = terrane_collection_support::slice_iterator(
            __terrane_iterable_1.as_slice(),
        );
        loop {
            inner = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(inner.size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 525 /* terrane-site: tests/integration/nested-view-value-value-loop/case.trn:10:21-10:32 */))
            );
        }
    }
}
// Source: tests/integration/projected-deep-rustdoc/case.trn
// Namespace: projected-deep-rustdoc
fn read_again(value: &terrane_query_witness::Deep) -> terrane_int_support::Int {
    return __terrane_raised(
        read_deep(&*value),

            526 /* terrane-site: tests/integration/projected-deep-rustdoc/case.trn:6:12-6:28 */,
    );
}
fn test_scenario_terrane_projected_deep_rustdoc() {
    let value: terrane_query_witness::Deep;
    value = __terrane_raised(
        make_deep(),

            527 /* terrane-site: tests/integration/projected-deep-rustdoc/case.trn:9:18-9:28 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&read_again(&value)));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(read_deep(&value),
        528 /* terrane-site: tests/integration/projected-deep-rustdoc/case.trn:11:13-11:29 */))
    );
}
// Source: tests/integration/sequential-pipeline-native-copy/case.trn
// Namespace: sequential-pipeline-native-copy
fn identity_probe(value: CopyProbe) -> CopyProbe {
    return value;
}
fn positive(value: CopyProbe) -> bool {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok((&value).value()) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::CopyProbe::value",
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

            529 /* terrane-site: tests/integration/sequential-pipeline-native-copy/case.trn:11:13-11:25 */,
    ) > terrane_int_support::Int::from(0_i128);
}
fn test_scenario_terrane_sequential_pipeline_native_copy() {
    let values: terrane_collection_support::List<CopyProbe>;
    let collected: terrane_collection_support::List<CopyProbe>;
    let mapped: terrane_collection_support::List<CopyProbe>;
    values = terrane_collection_support::List::<
        CopyProbe,
    >::new(
        vec![
            { let __terrane_conversion_input_0 = terrane_int_support::Int::from(1_i128);
            __terrane_raised(terrane_static_trn_436f707950726f6265_new(__terrane_conversion_input_0),
            530 /* terrane-site: tests/integration/sequential-pipeline-native-copy/case.trn:14:21-14:38 */) },
            { let __terrane_conversion_input_0 = terrane_int_support::Int::from(2_i128);
            __terrane_raised(terrane_static_trn_436f707950726f6265_new(__terrane_conversion_input_0),
            531 /* terrane-site: tests/integration/sequential-pipeline-native-copy/case.trn:14:42-14:59 */) }
        ],
    );
    __terrane_raised(
        terrane_static_trn_436f707950726f6265_reset_clones(),

            532 /* terrane-site: tests/integration/sequential-pipeline-native-copy/case.trn:15:5-15:29 */,
    );
    collected = {
        let __terrane_pipeline_source = &values;
        let mut __terrane_pipeline_remaining_1 = {
            let __terrane_pipeline_count_1 = terrane_int_support::Int::from(1_i128);
            if __terrane_pipeline_count_1 <= terrane_int_support::Int::from(0_i64) {
                0
            } else {
                __terrane_pipeline_count_1.as_usize().unwrap_or(usize::MAX)
            }
        };
        let mut __terrane_pipeline_output: Vec<CopyProbe> = Vec::with_capacity(
            __terrane_pipeline_source
                .as_slice()
                .len()
                .min(__terrane_pipeline_remaining_1),
        );
        if !(__terrane_pipeline_remaining_1 == 0) {
            for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
                {
                    __terrane_pipeline_remaining_1 -= 1;
                    __terrane_pipeline_output.push(*__terrane_pipeline_item);
                }
                if __terrane_pipeline_remaining_1 == 0 {
                    break;
                }
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_436f707950726f6265_clones(),
        533 /* terrane-site: tests/integration/sequential-pipeline-native-copy/case.trn:19:13-19:31 */))
    );
    mapped = {
        let __terrane_pipeline_source = &values;
        let __terrane_pipeline_callback_1 = positive;
        let __terrane_pipeline_callback_2 = identity_probe;
        let __terrane_pipeline_callback_3 = positive;
        let mut __terrane_pipeline_output: Vec<CopyProbe> = Vec::new();
        for __terrane_pipeline_item in __terrane_pipeline_source.as_slice() {
            '__terrane_pipeline_candidate: {
                if !__terrane_pipeline_callback_1(*__terrane_pipeline_item) {
                    break '__terrane_pipeline_candidate;
                }
                let __terrane_pipeline_mapped_2 = __terrane_pipeline_callback_2(
                    *__terrane_pipeline_item,
                );
                if !__terrane_pipeline_callback_3(__terrane_pipeline_mapped_2) {
                    break '__terrane_pipeline_candidate;
                }
                __terrane_pipeline_output.push(__terrane_pipeline_mapped_2);
            }
        }
        terrane_collection_support::List::new(__terrane_pipeline_output)
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(terrane_static_trn_436f707950726f6265_clones(),
        534 /* terrane-site: tests/integration/sequential-pipeline-native-copy/case.trn:25:13-25:31 */))
    );
    println!(
        "{}{}{}{}{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(collected
        .length())), String::from(", "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(mapped
        .length())), String::from(", "),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
}
// Source: tests/integration/shared-loop-read-control/case.trn
// Namespace: shared-loop-read-control
fn test_scenario_terrane_shared_loop_read_control() {
    let mut values: terrane_collection_support::List<terrane_int_support::Int>;
    let alias: &terrane_collection_support::List<terrane_int_support::Int>;
    let mut value: &terrane_int_support::Int;
    let mut nested: &terrane_int_support::Int;
    let mut value_2: terrane_int_support::Int;
    let holder: Holder;
    let mut views: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let mut view: &terrane_query_witness::CopyView<'_>;
    let mut nested_2: &terrane_query_witness::CopyView<'_>;
    values = terrane_collection_support::List::<
        terrane_int_support::Int,
    >::new(vec![terrane_int_support::Int::from(1_i128)]);
    alias = &values;
    let __terrane_iterable_0 = alias;
    let mut __terrane_iterator_0 = __terrane_iterable_0.terrane_borrowing_iterator();
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!("{}", terrane_scalar_support::scalar_text(&* value));
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
            .length()))
        );
        let __terrane_iterable_1 = &values;
        let mut __terrane_iterator_1 = __terrane_iterable_1.terrane_borrowing_iterator();
        loop {
            nested = match __terrane_iterator_1.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!("{}", terrane_scalar_support::scalar_text(&* nested));
        }
    }
    {
        let __terrane_collection_argument_0 = terrane_int_support::Int::from(2_i128);
        values.append(__terrane_collection_argument_0);
    };
    let __terrane_iterable_2 = values.clone();
    let mut __terrane_iterator_2 = terrane_collection_support::slice_iterator(
        __terrane_iterable_2.as_slice(),
    );
    loop {
        value_2 = match __terrane_iterator_2.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        {
            let __terrane_collection_argument_0 = terrane_int_support::Int::from(3_i128);
            values.append(__terrane_collection_argument_0);
        };
        println!("{}", terrane_scalar_support::scalar_text(&value_2));
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            535 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:18:14-18:26 */,
    );
    views = terrane_collection_support::List::<
        terrane_query_witness::CopyView<'_>,
    >::new(
        vec![
            { let value : terrane_query_witness::CopyView < '_ > = __terrane_raised(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&holder).copy_view()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 536 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:19:20-19:37 */); value }
        ],
    );
    let __terrane_iterable_3 = &views;
    let mut __terrane_iterator_3 = __terrane_iterable_3.terrane_borrowing_iterator();
    loop {
        view = match __terrane_iterator_3.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((* view).size()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane-query-witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 537 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:21:17-21:27 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(views
            .length()))
        );
        let __terrane_iterable_4 = &views;
        let mut __terrane_iterator_4 = __terrane_iterable_4.terrane_borrowing_iterator();
        loop {
            nested_2 = match __terrane_iterator_4.next() {
                terrane_collection_support::IterationStep::Item(item) => item,
                terrane_collection_support::IterationStep::End => break,
            };
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result
                < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((* nested_2).size()) })) { Ok(result)
                => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane-query-witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 538 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:24:21-24:33 */))
            );
        }
    }
    {
        let __terrane_collection_argument_0 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&holder).copy_view()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                539 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:25:20-25:37 */,
        );
        views.push_unique(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(views
        .length()))
    );
}
// Source: <compiler-generated test registry>
// Namespace: core/testing/runner
fn __terrane_test_hex(value: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value.as_bytes() {
        write!(encoded, "{byte:02x}").expect("writing to a string cannot fail");
    }
    encoded
}
fn __terrane_test_record(outcome: &str, error: &TerraneError) {
    let Ok(path) = std::env::var("TERRANE_TEST_RESULT") else {
        return;
    };
    let details = error.structured_details();
    let frames = error.source_frames();
    let mut record = format!(
        "1\n{}\n{}\n{}\n{}\n{}\n", outcome, __terrane_test_hex(&error.descriptor_name()),
        __terrane_test_hex(error.message()), details.len(), frames.len(),
    );
    for detail in details {
        record.push_str(&__terrane_test_hex(detail));
        record.push('\n');
    }
    for frame in frames {
        record.push_str(&__terrane_test_hex(&frame));
        record.push('\n');
    }
    let _ = std::fs::write(path, record);
}
fn main() {
    let selected = std::env::args().nth(1).unwrap_or_default();
    match selected.as_str() {
        "0" => {
            test_scenario_terrane_completion_finally_view_post_read();
        }
        "1" => {
            test_scenario_terrane_completion_native_catch_post_read();
        }
        "2" => {
            test_scenario_terrane_completion_native_post_read();
        }
        "3" => {
            test_scenario_terrane_native_borrowed_byte_ranges();
        }
        "4" => {
            if let Err(error) = test_scenario_terrane_native_borrowed_bytes() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "5" => {
            test_scenario_terrane_native_borrowed_iterator();
        }
        "6" => {
            test_scenario_terrane_native_borrowed_slice();
        }
        "7" => {
            test_scenario_terrane_native_borrowed_sum();
        }
        "8" => {
            if let Err(error) = test_scenario_terrane_native_checked_ordinal() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "9" => {
            test_scenario_terrane_native_copy_view_list_lender();
        }
        "10" => {
            test_scenario_terrane_native_copy_view_reassignment_snapshot();
        }
        "11" => {
            if let Err(error) = test_scenario_terrane_native_field_copy_clone_conversion() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "12" => {
            test_scenario_terrane_native_field_text();
        }
        "13" => {
            test_scenario_terrane_native_finally_only_rewrite();
        }
        "14" => {
            test_scenario_terrane_native_from_conversion();
        }
        "15" => {
            test_scenario_terrane_native_inferred_copy_view();
        }
        "16" => {
            test_scenario_terrane_native_loan_append_transfer();
        }
        "17" => {
            test_scenario_terrane_native_loan_chain_rebind_receiver();
        }
        "18" => {
            test_scenario_terrane_native_loan_exclusive_branch();
        }
        "19" => {
            test_scenario_terrane_native_loan_finally_copy_rebind();
        }
        "20" => {
            test_scenario_terrane_native_loan_fluent_nested_rebind();
        }
        "21" => {
            test_scenario_terrane_native_loan_helper_view_origin();
        }
        "22" => {
            test_scenario_terrane_native_loan_last_use_control();
        }
        "23" => {
            test_scenario_terrane_native_loan_loop_live_lender();
        }
        "24" => {
            test_scenario_terrane_native_loan_loop_refreshed();
        }
        "25" => {
            test_scenario_terrane_native_loan_map_set_lender();
        }
        "26" => {
            test_scenario_terrane_native_loan_owned_observation();
        }
        "27" => {
            test_scenario_terrane_native_loan_predicate_last_use();
        }
        "28" => {
            test_scenario_terrane_native_loan_predicate_switch();
        }
        "29" => {
            test_scenario_terrane_native_loan_predicate_three_arms();
        }
        "30" => {
            test_scenario_terrane_native_loan_rebound_alias_chain();
        }
        "31" => {
            test_scenario_terrane_native_loan_rebound_other();
        }
        "32" => {
            test_scenario_terrane_native_loan_rebound_same();
        }
        "33" => {
            test_scenario_terrane_native_loan_return_branch();
        }
        "34" => {
            test_scenario_terrane_native_loan_storage_alias_last_use();
        }
        "35" => {
            test_scenario_terrane_native_loan_try_finally_last_use();
        }
        "36" => {
            test_scenario_terrane_native_loan_try_local();
        }
        "37" => {
            test_scenario_terrane_native_loan_whole_last_use();
        }
        "38" => {
            test_scenario_terrane_native_loan_whole_move_last_use();
        }
        "39" => {
            test_scenario_terrane_native_loan_whole_mutate_last_use();
        }
        "40" => {
            test_scenario_terrane_native_loop_consuming_condition_break();
        }
        "41" => {
            test_scenario_terrane_native_noncopy_view_loop_append();
        }
        "42" => {
            if let Err(error) = test_scenario_terrane_native_optional_text() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "43" => {
            if let Err(error) = test_scenario_terrane_native_ref_clone_repeat() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "44" => {
            if let Err(error) = test_scenario_terrane_native_required_enum_field() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "45" => {
            test_scenario_terrane_native_return_parameter_single();
        }
        "46" => {
            test_scenario_terrane_native_scoped_text();
        }
        "47" => {
            test_scenario_terrane_native_try_body_rewrite_control();
        }
        "48" => {
            test_scenario_terrane_native_try_catch_dominating_rewrite();
        }
        "49" => {
            test_scenario_terrane_native_try_copy_finally_control();
        }
        "50" => {
            test_scenario_terrane_native_try_finally_dominating_rewrite();
        }
        "51" => {
            test_scenario_terrane_native_try_finally_rewrite_control();
        }
        "52" => {
            test_scenario_terrane_native_try_local_consume();
        }
        "53" => {
            test_scenario_terrane_native_try_unconditional_rewrite_control();
        }
        "54" => {
            test_scenario_terrane_native_typed_metadata();
        }
        "55" => {
            test_scenario_terrane_native_unordered_map_dead_lender_control();
        }
        "56" => {
            test_scenario_terrane_native_view_list_rebind();
        }
        "57" => {
            test_scenario_terrane_native_view_local_map_storage();
        }
        "58" => {
            test_scenario_terrane_native_view_loop_binders();
        }
        "59" => {
            test_scenario_terrane_native_view_map_entry_iteration();
        }
        "60" => {
            test_scenario_terrane_native_view_tuple_iteration();
        }
        "61" => {
            test_scenario_terrane_native_view_value_iteration();
        }
        "62" => {
            test_scenario_terrane_nested_noncopy_view_loops();
        }
        "63" => {
            test_scenario_terrane_nested_view_loops();
        }
        "64" => {
            test_scenario_terrane_nested_view_ref_value_loop();
        }
        "65" => {
            test_scenario_terrane_nested_view_value_ref_loop();
        }
        "66" => {
            test_scenario_terrane_nested_view_value_value_loop();
        }
        "67" => {
            test_scenario_terrane_projected_deep_rustdoc();
        }
        "68" => {
            test_scenario_terrane_sequential_pipeline_native_copy();
        }
        "69" => {
            test_scenario_terrane_shared_loop_read_control();
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
