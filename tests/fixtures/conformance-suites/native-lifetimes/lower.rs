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
                                        "terrane_query_witness",
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
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
                    ::__terrane_dependency_panic(payload, "terrane_query_witness",
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
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
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
        "terrane_query_witness", "terrane_query_witness::Cl::value")) }; Ok:: < _, crate
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
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
        "terrane_query_witness", "terrane_query_witness::Cl::value")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from(i128::from(value))) } }
        (), 13 /* terrane-site: tests/integration/completion-native-post-read/case.trn:12:13-12:21 */))
    );
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

            14 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:6:14-6:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 15 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:7:20-7:37 */); value
            }
        ],
    );
    alias = views;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(alias
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        16 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:9:13-9:21 */)),
        16 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:9:13-9:21 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 17 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:9:13-9:27 */) })
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 18 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:10:28-10:45 */);
            value }])
        ],
    );
    nested_alias = nested;
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(__terrane_raised(nested_alias
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        19 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:28 */)),
        19 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:28 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        20 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:31 */)),
        20 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:31 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 21 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:12:13-12:37 */) })
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            22 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:13:5-13:17 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&holder).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 23 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:14:13-14:30 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            24 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:15:12-15:29 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::CopyView::size",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            25 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:16:5-16:15 */,
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
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 26 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:18:13-18:23 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(copied
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        27 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:19:13-19:22 */)),
        27 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:19:13-19:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 28 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:19:13-19:28 */) })
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 29 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:23:17-23:27 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            30 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:24:5-24:17 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&holder).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 31 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:25:13-25:30 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            32 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:7:12-7:29 */,
    );
}
fn test_scenario_terrane_native_copy_view_reassignment_snapshot() {
    let h: Holder;
    let mut k: Holder;
    let c: terrane_query_witness::CopyView<'_>;
    let values: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            33 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:10:9-10:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            34 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:11:9-11:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            35 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:12:9-12:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            36 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:14:9-14:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            37 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:15:5-15:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        38 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:22 */)),
        38 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 39 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:16:13-16:28 */)
        })
    );
    let c = fresh(&k);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(c.size()) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 40 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:18:13-18:20 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            41 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:19:5-19:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        42 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:22 */)),
        42 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 43 /* terrane-site: tests/integration/native-copy-view-reassignment-snapshot/case.trn:20:13-20:28 */)
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
            let __terrane_value = __terrane_raised(
                terrane_static_trn_546f6b656e_new(),

                    44 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:7:18-7:29 */,
            );
            self.t = Some(__terrane_value);
        }
        {
            let __terrane_value = __terrane_raised(
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::CloneEnum::A",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    45 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:8:18-8:31 */,
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
                                            "terrane_query_witness",
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

                        46 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:10:12-10:22 */,
                )?,
            ),

                46 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:10:12-10:22 */,
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
                                            "terrane_query_witness",
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

                        47 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:12:12-12:19 */,
                )?,
            ),

                47 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:12:12-12:19 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Token::value")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 48 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:17:17-17:27 */)
            ?)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
            < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(r.t.as_ref()
            .expect("required field initialized").clone().value()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Token::value")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 49 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:18:17-18:29 */)
            ?)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result
            < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(r.e.as_ref()
            .expect("required field initialized").clone().code()) })) { Ok(result) =>
            result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::CloneEnum::code")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 50 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:19:17-19:26 */)
            ?)
        );
        i = i.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(observe_terrane_native_field_copy_clone_conversion(&r),
        51 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:21:13-21:23 */) ?)
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
                                "terrane_query_witness",
                                "terrane_query_witness::CloneEnum::A",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            52 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:22:9-22:22 */,
    )?;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(clone_observe(&mut e),
        53 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:23:13-23:29 */) ?)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(clone_observe(&mut e),
        54 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:24:13-24:29 */) ?)
    );
    Ok(())
}
// Source: tests/integration/native-finally-only-rewrite/case.trn
// Namespace: native-finally-only-rewrite
fn test_scenario_terrane_native_finally_only_rewrite() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            55 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:4:9-4:21 */,
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    56 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:5:9-5:16 */,
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 57 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:7:17-7:29 */))
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
                    ::__terrane_dependency_panic(payload, "terrane_query_witness",
                    "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 58 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:9:13-9:20 */
                ),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v.as_ref()
            .expect("flow-proven available binding")).size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 59 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:10:17-10:24 */))
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
                                "terrane_query_witness",
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

            60 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:5:12-5:23 */,
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
                                "terrane_query_witness",
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

            61 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:8:12-8:23 */,
    );
}
fn test_scenario_terrane_native_inferred_copy_view() {
    let holder: Holder;
    let mut inferred_view: terrane_query_witness::CopyView<'_>;
    let inferred: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let typed: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            62 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:11:14-11:26 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            63 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:12:21-12:38 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 64 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:15:23-15:40 */); value }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(inferred
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        65 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:24 */)),
        65 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:24 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 66 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:30 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(inferred
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        67 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:24 */)),
        67 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:24 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 68 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:30 */) })
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 69 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:18:37-18:54 */); value }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(typed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        70 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:21 */)),
        70 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:21 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 71 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:27 */) })
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

            72 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:6:9-6:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            73 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:7:9-7:16 */,
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
            "terrane_query_witness", "terrane_query_witness::Holder::view")) }; Ok:: < _,
            crate ::TerraneForeignError > (value) } } (), 74 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:8:21-8:28 */); value }
        ],
    );
    {
        let __terrane_collection_argument_0 = v;
        values.push_unique(__terrane_collection_argument_0);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_index_0 =
        __terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(1_i128)),
        75 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:10:13-10:22 */); let
        __terrane_index_place_1 = __terrane_raised(values.get(__terrane_index_0)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_0)),
        75 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:10:13-10:22 */); let
        __terrane_native_input_0 = &__terrane_index_place_1; __terrane_raised(| | ->
        Result < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 76 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:10:13-10:28 */) })
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            77 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:11:13-11:20 */,
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
        78 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:14:13-14:24 */); let
        __terrane_index_place_3 = __terrane_raised(declared.get(__terrane_index_2)
        .ok_or_else(| |
        terrane_collection_support::IndexError::from_usize(__terrane_index_2)),
        78 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:14:13-14:24 */); let
        __terrane_native_input_0 = &__terrane_index_place_3; __terrane_raised(| | ->
        Result < _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 79 /* terrane-site: tests/integration/native-loan-append-transfer/case.trn:14:13-14:30 */) })
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

            80 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:11:9-11:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            81 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:12:9-12:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            82 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:13:9-13:16 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::View::chain",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            83 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:14:9-14:24 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            84 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:15:9-15:16 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 85 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:16:13-16:20 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 86 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:16:24-16:31 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

                87 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                88 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:11:38-11:50 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            89 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:12:9-12:22 */,
    );
    c = true;
    if c {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    90 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:15:19-15:31 */,
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
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 91 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:17:17-17:24 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

            92 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:11:9-11:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            93 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:12:9-12:21 */,
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
                "terrane_query_witness", "terrane_query_witness::CopyView::size")) };
                Ok:: < _, crate ::TerraneForeignError >
                (terrane_int_support::Int::from_u128(value as u128)) } } (),
                94 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:18:21-18:28 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).copy_view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 95 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:19:17-19:29 */
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
        i = i.clone() + terrane_int_support::Int::from(1_i128);
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

            96 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:6:9-6:21 */,
    );
    unrelated = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            97 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:7:17-7:29 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            98 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:8:9-8:16 */,
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
            "terrane_query_witness", "terrane_query_witness::View::chain")) }; Ok:: < _,
            crate ::TerraneForeignError > (value) } } (), 99 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:9:21-9:29 */);
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            100 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:10:9-10:16 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            101 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:12:9-12:16 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            102 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:13:5-13:20 */,
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
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 103 /* terrane-site: tests/integration/native-loan-fluent-nested-rebind/case.trn:16:13-16:20 */))
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

            104 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:15:9-15:21 */,
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                105 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:16:19-16:31 */,
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(view.size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 106 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:17:13-17:23 */))
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::copy_view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    107 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:18:32-18:44 */,
            ),
        ),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        108 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:22 */)),
        108 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 109 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:19:13-19:28 */) })
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            110 /* terrane-site: tests/integration/native-loan-helper-view-origin/case.trn:20:5-20:12 */,
    );
}
// Source: tests/integration/native-loan-last-use-control/case.trn
// Namespace: native-loan-last-use-control
fn test_scenario_terrane_native_loan_last_use_control() {
    let mut h: Holder;
    let values: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            111 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:6:9-6:21 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 112 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:7:38-7:50 */); value
            }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        113 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:8:13-8:22 */)),
        113 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:8:13-8:22 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 114 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:8:13-8:28 */) })
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            115 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:9:5-9:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 116 /* terrane-site: tests/integration/native-loan-last-use-control/case.trn:10:13-10:25 */))
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

            117 /* terrane-site: tests/integration/native-loan-loop-live-lender/case.trn:6:9-6:21 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 118 /* terrane-site: tests/integration/native-loan-loop-live-lender/case.trn:7:38-7:50 */); value
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 119 /* terrane-site: tests/integration/native-loan-loop-live-lender/case.trn:9:17-9:28 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            120 /* terrane-site: tests/integration/native-loan-loop-live-lender/case.trn:10:5-10:12 */,
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

                121 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                122 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:11:38-11:50 */,
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                123 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:14:13-14:26 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 124 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:15:17-15:24 */))
        );
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    125 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:16:19-16:31 */,
            );
            w.inner = Some(__terrane_value);
        }
        i = i.clone() + terrane_int_support::Int::from(1_i128);
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

            126 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:6:9-6:21 */,
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                127 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:8:26-8:38 */,
        );
        values.set(__terrane_collection_argument_0, __terrane_collection_argument_1);
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values.get_or_error(&String::from("view")),
        128 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:9:13-9:27 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 129 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:9:13-9:33 */) })
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            130 /* terrane-site: tests/integration/native-loan-map-set-lender/case.trn:10:5-10:12 */,
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

            131 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:11:9-11:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            132 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:12:9-12:16 */,
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
                                "terrane_query_witness",
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

            133 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:13:16-13:23 */,
    );
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            134 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:14:9-14:21 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&observed));
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 135 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:16:13-16:25 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

                136 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                137 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:11:38-11:50 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            138 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:12:9-12:22 */,
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
                                "terrane_query_witness",
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

            139 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:14:15-14:22 */,
    ) > terrane_int_support::Int::from(100_i128)
    {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    140 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:15:23-15:35 */,
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

                141 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                142 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:11:38-11:50 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            143 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:12:9-12:22 */,
    );
    i = 1;
    if i == 0 {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    144 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:16:23-16:35 */,
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
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 145 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:18:21-18:28 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

                146 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                147 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:11:38-11:50 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            148 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:12:9-12:22 */,
    );
    i = 2;
    if i == 0 {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    149 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:16:23-16:35 */,
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
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 150 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:20:21-20:28 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

            151 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:11:9-11:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            152 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:12:9-12:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            153 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:13:9-13:16 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            154 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:14:9-14:16 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::View::chain",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            155 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:15:9-15:24 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            156 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:16:5-16:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 157 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:17:13-17:20 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

                158 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                159 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:11:38-11:50 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            160 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:12:9-12:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 161 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:13:13-13:20 */))
    );
    {
        let __terrane_value = __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                162 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:14:15-14:27 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            163 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:15:9-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 164 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:16:13-16:20 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

                165 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                166 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:11:38-11:50 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            167 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:12:9-12:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 168 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:13:13-13:20 */))
    );
    {
        let __terrane_value = __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                169 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:14:15-14:27 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            170 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:15:9-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 171 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:16:13-16:20 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

                172 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                173 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:11:38-11:50 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            174 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:12:9-12:22 */,
    );
    c = false;
    if c {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    175 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:15:19-15:31 */,
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
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 176 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:17:13-17:20 */))
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

            177 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            178 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:5:9-5:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            179 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:6:9-6:16 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            180 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:8:9-8:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 181 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:9:13-9:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 182 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:10:13-10:20 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            183 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:11:5-11:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&k).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 184 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:12:13-12:25 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_try_finally_last_use() {
    let mut h: Holder;
    let v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            185 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:11:9-11:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            186 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:12:9-12:16 */,
    );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 187 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:14:17-14:24 */))
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::bump")) }; Ok:: < _, crate
            ::TerraneForeignError > (()) } } (), 188 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:16:9-16:16 */
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
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 189 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:17:13-17:25 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_try_local() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            190 /* terrane-site: tests/integration/native-loan-try-local/case.trn:11:9-11:21 */,
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
                        ::__terrane_dependency_panic(payload, "terrane_query_witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 191 /* terrane-site: tests/integration/native-loan-try-local/case.trn:13:13-13:20 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&v.as_ref()
                .expect("flow-proven available binding")).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 192 /* terrane-site: tests/integration/native-loan-try-local/case.trn:14:17-14:24 */))
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
                        "terrane_query_witness",
                        "terrane_query_witness::Holder::label_len")) }; Ok:: < _, crate
                        ::TerraneForeignError >
                        (terrane_int_support::Int::from_u128(value as u128)) } } (),
                        193 /* terrane-site: tests/integration/native-loan-try-local/case.trn:16:17-16:29 */))
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
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 194 /* terrane-site: tests/integration/native-loan-try-local/case.trn:17:13-17:25 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_whole_last_use() {
    let mut h: Holder;
    let v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            195 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:11:9-11:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            196 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 197 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:13:13-13:20 */))
    );
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            198 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:14:9-14:21 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 199 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:15:13-15:25 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
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

            200 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:11:9-11:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            201 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 202 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:13:13-13:20 */))
    );
    moved = h;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&moved).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 203 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:15:13-15:29 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loan_whole_mutate_last_use() {
    let mut h: Holder;
    let v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            204 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:11:9-11:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            205 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 206 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:13:13-13:20 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            207 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:14:5-14:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 208 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:15:13-15:25 */))
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
            let __terrane_value = inner;
            self.inner = Some(__terrane_value);
        }
        {
            let __terrane_value = other;
            self.other = Some(__terrane_value);
        }
    }
}
fn test_scenario_terrane_native_loop_consuming_condition_break() {
    let h: Holder;
    let v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            209 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:11:9-11:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            210 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:12:9-12:16 */,
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
                                "terrane_query_witness",
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

            211 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:13:12-13:24 */,
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

            212 /* terrane-site: tests/integration/native-noncopy-view-loop-append/case.trn:6:14-6:26 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            213 /* terrane-site: tests/integration/native-noncopy-view-loop-append/case.trn:7:12-7:24 */,
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                214 /* terrane-site: tests/integration/native-noncopy-view-loop-append/case.trn:11:16-11:28 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::View::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 215 /* terrane-site: tests/integration/native-noncopy-view-loop-append/case.trn:14:17-14:28 */))
        );
    }
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
                                "terrane_query_witness",
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

            216 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:4:13-4:24 */,
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
                                "terrane_query_witness",
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

            217 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:5:14-5:25 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<u64>(&(first.clone() + second.clone())),

                218 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:6:12-6:26 */,
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
                                "terrane_query_witness",
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

            219 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:8:13-8:25 */,
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
                                                "terrane_query_witness",
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

                            220 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:9:21-9:33 */,
                    )?),
            ),

                221 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:9:12-9:34 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::CloneEnum::A",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            222 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:11:13-11:26 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(twice(&mut
        value), 223 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:12:13-12:25 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(value.code()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CloneEnum::code")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 224 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:13:13-13:24 */) ?)
    );
    token = __terrane_raised_err(
        terrane_static_trn_546f6b656e_new(),

            225 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:14:13-14:24 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(copied(&mut
        token), 226 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:15:13-15:26 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(token.value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Token::value")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 227 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:16:13-16:25 */) ?)
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
            let __terrane_value = __terrane_raised(
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::NE::A",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    228 /* terrane-site: tests/integration/native-required-enum-field/case.trn:7:22-7:28 */,
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
                                                "terrane_query_witness",
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

                            229 /* terrane-site: tests/integration/native-required-enum-field/case.trn:9:16-9:32 */,
                    )?,
                ),

                    229 /* terrane-site: tests/integration/native-required-enum-field/case.trn:9:16-9:32 */,
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
            let __terrane_value = TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052::terrane_construct();
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
        "terrane_query_witness", "terrane_query_witness::NE::code")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 230 /* terrane-site: tests/integration/native-required-enum-field/case.trn:16:13-16:30 */) ?)
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
        "terrane_query_witness", "terrane_query_witness::NE::code")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 231 /* terrane-site: tests/integration/native-required-enum-field/case.trn:19:13-19:36 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer.inner.as_ref()
        .expect("required field initialized").sibling)
    );
    owned = TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(owned.finish(),
        232 /* terrane-site: tests/integration/native-required-enum-field/case.trn:22:13-22:26 */) ?)
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            233 /* terrane-site: tests/integration/native-return-parameter-single/case.trn:5:12-5:24 */,
    );
}
fn test_scenario_terrane_native_return_parameter_single() {
    let mut h: Holder;
    let v: CopyView;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            234 /* terrane-site: tests/integration/native-return-parameter-single/case.trn:8:9-8:21 */,
    );
    v = f(&h);
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(v.size()) })) { Ok(result) => result ?, Err(payload)
        => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 235 /* terrane-site: tests/integration/native-return-parameter-single/case.trn:10:13-10:20 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            236 /* terrane-site: tests/integration/native-return-parameter-single/case.trn:11:5-11:12 */,
    );
}
// Source: tests/integration/native-try-body-rewrite-control/case.trn
// Namespace: native-try-body-rewrite-control
fn test_scenario_terrane_native_try_body_rewrite_control() {
    let h: Holder;
    let mut v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            237 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:4:9-4:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            238 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:5:9-5:16 */,
    );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 239 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:7:17-7:29 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 240 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:8:13-8:20 */
            );
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 241 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:9:17-9:24 */))
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

            242 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:4:9-4:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            243 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:5:9-5:16 */,
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 244 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:7:17-7:29 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 245 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:8:13-8:20 */
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
                        ::__terrane_dependency_panic(payload, "terrane_query_witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 246 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:10:13-10:20 */
                    );
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                        | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&v).size())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane_query_witness",
                        "terrane_query_witness::View::size")) }; Ok:: < _, crate
                        ::TerraneForeignError >
                        (terrane_int_support::Int::from_u128(value as u128)) } } (),
                        247 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:11:17-11:24 */))
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

            248 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            249 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:5:9-5:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            250 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:6:9-6:21 */,
    );
    let mut __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.size()) })) { Ok(result) => result
                ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
                "terrane_query_witness", "terrane_query_witness::CopyView::size")) };
                Ok:: < _, crate ::TerraneForeignError >
                (terrane_int_support::Int::from_u128(value as u128)) } } (),
                251 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:8:17-8:24 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&k).copy_view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 252 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:9:13-9:25 */
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 253 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:11:17-11:24 */))
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
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 254 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:12:13-12:20 */))
    );
}
// Source: tests/integration/native-try-finally-dominating-rewrite/case.trn
// Namespace: native-try-finally-dominating-rewrite
fn test_scenario_terrane_native_try_finally_dominating_rewrite() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            255 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:4:9-4:21 */,
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    256 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:5:9-5:16 */,
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 257 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:7:17-7:29 */))
            );
            let _ = v
                .insert(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane_query_witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 258 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:8:13-8:20 */
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
                    ::__terrane_dependency_panic(payload, "terrane_query_witness",
                    "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 259 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:10:13-10:20 */
                ),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v.as_ref()
            .expect("flow-proven available binding")).size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 260 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:11:17-11:24 */))
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

            261 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:4:9-4:21 */,
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    262 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:5:9-5:16 */,
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
            "terrane_query_witness", "terrane_query_witness::View::into_size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 263 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:9:17-9:29 */))
        );
        let _ = v
            .insert(
                __terrane_raised_completion!(
                    | | -> Result < _, crate ::TerraneForeignError > { { let value =
                    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | ->
                    Result < _, crate ::TerraneForeignError > { Ok((&h).view()) })) {
                    Ok(result) => result ?, Err(payload) => return Err(crate
                    ::__terrane_dependency_panic(payload, "terrane_query_witness",
                    "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                    ::TerraneForeignError > (value) } } (), 264 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:10:13-10:20 */
                ),
            );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(| |
            -> Result < _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v.as_ref()
            .expect("flow-proven available binding")).size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 265 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:11:17-11:24 */))
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

            266 /* terrane-site: tests/integration/native-try-local-consume/case.trn:4:9-4:21 */,
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
                        ::__terrane_dependency_panic(payload, "terrane_query_witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 267 /* terrane-site: tests/integration/native-try-local-consume/case.trn:6:13-6:20 */
                    ),
                );
            println!(
                "{}", terrane_scalar_support::scalar_text(&__terrane_raised_completion!(|
                | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok(v.take()
                .expect("flow-proven availability").into_size()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 268 /* terrane-site: tests/integration/native-try-local-consume/case.trn:7:17-7:29 */))
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
                            "terrane_query_witness",
                            "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                            ::TerraneForeignError > (value) } } (),
                            269 /* terrane-site: tests/integration/native-try-local-consume/case.trn:13:20-13:27 */
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
                    ::__terrane_dependency_panic(payload, "terrane_query_witness",
                    "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                    ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                    u128)) } } (), 270 /* terrane-site: tests/integration/native-try-local-consume/case.trn:14:21-14:36 */))
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
        i = i.clone() + terrane_int_support::Int::from(1_i128);
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

            271 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            272 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:5:9-5:21 */,
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    273 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:6:9-6:16 */,
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::into_size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 274 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:8:17-8:29 */))
            );
            let _ = v
                .insert(
                    __terrane_raised_completion!(
                        | | -> Result < _, crate ::TerraneForeignError > { { let value =
                        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
                        -> Result < _, crate ::TerraneForeignError > { Ok((&h).view())
                        })) { Ok(result) => result ?, Err(payload) => return Err(crate
                        ::__terrane_dependency_panic(payload, "terrane_query_witness",
                        "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                        ::TerraneForeignError > (value) } } (), 275 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:9:13-9:20 */
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    276 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:12:9-12:16 */,
            ),
        );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v.as_ref()
        .expect("flow-proven available binding")).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 277 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:13:13-13:20 */))
    );
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

            278 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:6:14-6:26 */,
    );
    unrelated = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            279 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:7:17-7:29 */,
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                280 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:9:26-9:43 */,
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 281 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:11:17-11:28 */))
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::bump",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(())
                }
            }(),

                282 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:12:9-12:24 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            283 /* terrane-site: tests/integration/native-unordered-map-dead-lender-control/case.trn:13:5-13:17 */,
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

            284 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:12:9-12:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            285 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:13:9-13:16 */,
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
            "terrane_query_witness", "terrane_query_witness::Holder::view")) }; Ok:: < _,
            crate ::TerraneForeignError > (value) } } (), 286 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:14:21-14:28 */); value }
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            287 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:16:9-16:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 288 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:17:13-17:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .length()))
    );
    no_read_holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            289 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:19:22-19:34 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            290 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:20:15-20:35 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 291 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:21:27-21:47 */); value }
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            292 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:23:15-23:35 */,
    );
    let _ = &no_read;
    a = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            293 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:24:9-24:21 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            294 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:25:11-25:18 */,
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
            "terrane_query_witness", "terrane_query_witness::Holder::view")) }; Ok:: < _,
            crate ::TerraneForeignError > (value) } } (), 295 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:26:27-26:34 */); value }
        ],
    );
    {
        let __terrane_collection_argument_0 = old;
        other_values.push_unique(__terrane_collection_argument_0);
    };
    other = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            296 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:28:13-28:25 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            297 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:29:11-29:22 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            298 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:31:13-31:20 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            299 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:33:13-33:20 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            300 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:35:19-35:26 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            301 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:37:19-37:30 */,
    );
    let _ = &alias_value;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(alias_values
        .length()))
    );
    catch_holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            302 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:39:20-39:32 */,
    );
    other_lender = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            303 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:40:20-40:32 */,
    );
    unrelated = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            304 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:41:17-41:29 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            305 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:42:18-42:41 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            306 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:43:19-43:42 */,
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            catch_view = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&other_lender).copy_view()) })) {
                Ok(result) => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 307 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:45:22-45:45 */
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

                        308 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:46:9-46:28 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            309 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:49:5-49:20 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(catch_alias.size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 310 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:50:13-50:30 */))
    );
    safe_holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            311 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:51:19-51:31 */,
    );
    safe_k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            312 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:52:14-52:26 */,
    );
    safe_other = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            313 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:53:18-53:30 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            314 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:54:17-54:39 */,
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            315 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:55:18-55:40 */,
    );
    let __terrane_completion_1: TerraneCompletion<()> = (|| {
        let __terrane_try_1: TerraneCompletion<()> = (|| {
            safe_view = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&safe_k).copy_view()) })) {
                Ok(result) => result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 316 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:57:21-57:38 */
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            317 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:60:5-60:21 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(safe_alias.size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 318 /* terrane-site: tests/integration/native-view-list-rebind/case.trn:61:13-61:29 */))
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::copy_view",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),

            319 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:6:12-6:29 */,
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
        320 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:11:13-11:28 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 321 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:11:13-11:34 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(values.get_or_error(&String::from("view")),
        322 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:12:13-12:27 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 323 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:12:13-12:33 */) })
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
        324 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:17:13-17:31 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 325 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:17:13-17:37 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(unordered.get_or_error(&String::from("view")),
        326 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:18:13-18:30 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 327 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:18:13-18:36 */) })
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
        328 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:21:13-21:27 */)
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        329 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:21:13-21:30 */)),
        329 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:21:13-21:30 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 330 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:21:13-21:36 */) })
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
        331 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:24:13-24:21 */)),
        331 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:24:13-24:21 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 332 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:24:13-24:27 */) })
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
                                "terrane_query_witness",
                                "terrane_query_witness::Holder::bump",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(())
            }
        }(),

            333 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:25:5-25:17 */,
    );
}
fn test_scenario_terrane_native_view_local_map_storage() {
    store(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                334 /* terrane-site: tests/integration/native-view-local-map-storage/case.trn:28:13-28:25 */,
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

            335 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:6:14-6:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 336 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:7:21-7:38 */); value }
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 337 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:9:17-9:28 */))
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 338 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:11:17-11:28 */))
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 339 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:13:17-13:28 */))
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 340 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:15:17-15:28 */))
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 341 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:18:21-18:32 */))
            );
        }
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(outer.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 342 /* terrane-site: tests/integration/native-view-loop-binders/case.trn:19:17-19:28 */))
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

            343 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:6:14-6:26 */,
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                344 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:8:30-8:47 */,
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 345 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:10:17-10:28 */))
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                346 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:13:34-13:51 */,
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 347 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:15:17-15:28 */))
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                348 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:18:35-18:52 */,
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 349 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:23:21-23:32 */))
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                350 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:26:37-26:54 */,
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 351 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:31:21-31:32 */))
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 352 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:34:17-34:28 */))
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::copy_view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    353 /* terrane-site: tests/integration/native-view-map-entry-iteration/case.trn:35:32-35:49 */,
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

            354 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:6:14-6:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 355 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:7:40-7:57 */); value }
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 356 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:9:17-9:28 */))
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 357 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:12:17-12:28 */))
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 358 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:13:24-13:41 */); value
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 359 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:15:17-15:28 */))
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 360 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:17:17-17:28 */))
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 361 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:18:27-18:44 */)
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 362 /* terrane-site: tests/integration/native-view-tuple-iteration/case.trn:19:17-19:28 */))
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 363 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:7:17-7:28 */))
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 364 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:12:17-12:28 */))
        );
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(values
        .clone().length()))
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

            365 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:16:14-16:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 366 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:17:37-17:54 */); value
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
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 367 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:19:17-19:28 */))
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
        let _ = &value;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 368 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:23:17-23:28 */))
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 369 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:24:23-24:40 */); value
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
        let _ = &value;
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 370 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:26:17-26:28 */))
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
        let _ = &value;
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::copy_view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    371 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:30:27-30:44 */,
            );
            inferred.push_unique(__terrane_collection_argument_0);
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 372 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:31:17-31:28 */))
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
        let _ = &value;
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
                                        "terrane_query_witness",
                                        "terrane_query_witness::Holder::copy_view",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(value)
                    }
                }(),

                    373 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:36:24-36:41 */,
            );
            typed.push_unique(__terrane_collection_argument_0);
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok(value.size()) })) { Ok(result) => result
            ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::CopyView::size")) }; Ok:: <
            _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value
            as u128)) } } (), 374 /* terrane-site: tests/integration/native-view-value-iteration/case.trn:37:17-37:28 */))
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

            375 /* terrane-site: tests/integration/nested-noncopy-view-loops/case.trn:6:14-6:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 376 /* terrane-site: tests/integration/nested-noncopy-view-loops/case.trn:7:28-7:40 */); value }])
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::View::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 377 /* terrane-site: tests/integration/nested-noncopy-view-loops/case.trn:10:21-10:32 */))
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
        378 /* terrane-site: tests/integration/nested-view-loops/case.trn:6:14-6:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 379 /* terrane-site: tests/integration/nested-view-loops/case.trn:7:28-7:45 */); value }])
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 380 /* terrane-site: tests/integration/nested-view-loops/case.trn:10:21-10:32 */))
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

            381 /* terrane-site: tests/integration/nested-view-ref-value-loop/case.trn:6:14-6:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 382 /* terrane-site: tests/integration/nested-view-ref-value-loop/case.trn:7:28-7:45 */); value
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 383 /* terrane-site: tests/integration/nested-view-ref-value-loop/case.trn:10:21-10:32 */))
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

            384 /* terrane-site: tests/integration/nested-view-value-ref-loop/case.trn:6:14-6:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 385 /* terrane-site: tests/integration/nested-view-value-ref-loop/case.trn:7:28-7:45 */); value
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 386 /* terrane-site: tests/integration/nested-view-value-ref-loop/case.trn:10:21-10:32 */))
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

            387 /* terrane-site: tests/integration/nested-view-value-value-loop/case.trn:6:14-6:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 388 /* terrane-site: tests/integration/nested-view-value-value-loop/case.trn:7:28-7:45 */); value
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 389 /* terrane-site: tests/integration/nested-view-value-value-loop/case.trn:10:21-10:32 */))
            );
        }
    }
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

            390 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:18:14-18:26 */,
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 391 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:19:20-19:37 */); value }
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
            ::__terrane_dependency_panic(payload, "terrane_query_witness",
            "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 392 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:21:17-21:27 */))
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
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
                ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
                u128)) } } (), 393 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:24:21-24:33 */))
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
                                    "terrane_query_witness",
                                    "terrane_query_witness::Holder::copy_view",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),

                394 /* terrane-site: tests/integration/shared-loop-read-control/case.trn:25:20-25:37 */,
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
            test_scenario_terrane_native_copy_view_list_lender();
        }
        "4" => {
            test_scenario_terrane_native_copy_view_reassignment_snapshot();
        }
        "5" => {
            if let Err(error) = test_scenario_terrane_native_field_copy_clone_conversion() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "6" => {
            test_scenario_terrane_native_finally_only_rewrite();
        }
        "7" => {
            test_scenario_terrane_native_inferred_copy_view();
        }
        "8" => {
            test_scenario_terrane_native_loan_append_transfer();
        }
        "9" => {
            test_scenario_terrane_native_loan_chain_rebind_receiver();
        }
        "10" => {
            test_scenario_terrane_native_loan_exclusive_branch();
        }
        "11" => {
            test_scenario_terrane_native_loan_finally_copy_rebind();
        }
        "12" => {
            test_scenario_terrane_native_loan_fluent_nested_rebind();
        }
        "13" => {
            test_scenario_terrane_native_loan_helper_view_origin();
        }
        "14" => {
            test_scenario_terrane_native_loan_last_use_control();
        }
        "15" => {
            test_scenario_terrane_native_loan_loop_live_lender();
        }
        "16" => {
            test_scenario_terrane_native_loan_loop_refreshed();
        }
        "17" => {
            test_scenario_terrane_native_loan_map_set_lender();
        }
        "18" => {
            test_scenario_terrane_native_loan_owned_observation();
        }
        "19" => {
            test_scenario_terrane_native_loan_predicate_last_use();
        }
        "20" => {
            test_scenario_terrane_native_loan_predicate_switch();
        }
        "21" => {
            test_scenario_terrane_native_loan_predicate_three_arms();
        }
        "22" => {
            test_scenario_terrane_native_loan_rebound_alias_chain();
        }
        "23" => {
            test_scenario_terrane_native_loan_rebound_other();
        }
        "24" => {
            test_scenario_terrane_native_loan_rebound_same();
        }
        "25" => {
            test_scenario_terrane_native_loan_return_branch();
        }
        "26" => {
            test_scenario_terrane_native_loan_storage_alias_last_use();
        }
        "27" => {
            test_scenario_terrane_native_loan_try_finally_last_use();
        }
        "28" => {
            test_scenario_terrane_native_loan_try_local();
        }
        "29" => {
            test_scenario_terrane_native_loan_whole_last_use();
        }
        "30" => {
            test_scenario_terrane_native_loan_whole_move_last_use();
        }
        "31" => {
            test_scenario_terrane_native_loan_whole_mutate_last_use();
        }
        "32" => {
            test_scenario_terrane_native_loop_consuming_condition_break();
        }
        "33" => {
            test_scenario_terrane_native_noncopy_view_loop_append();
        }
        "34" => {
            if let Err(error) = test_scenario_terrane_native_ref_clone_repeat() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "35" => {
            if let Err(error) = test_scenario_terrane_native_required_enum_field() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "36" => {
            test_scenario_terrane_native_return_parameter_single();
        }
        "37" => {
            test_scenario_terrane_native_try_body_rewrite_control();
        }
        "38" => {
            test_scenario_terrane_native_try_catch_dominating_rewrite();
        }
        "39" => {
            test_scenario_terrane_native_try_copy_finally_control();
        }
        "40" => {
            test_scenario_terrane_native_try_finally_dominating_rewrite();
        }
        "41" => {
            test_scenario_terrane_native_try_finally_rewrite_control();
        }
        "42" => {
            test_scenario_terrane_native_try_local_consume();
        }
        "43" => {
            test_scenario_terrane_native_try_unconditional_rewrite_control();
        }
        "44" => {
            test_scenario_terrane_native_unordered_map_dead_lender_control();
        }
        "45" => {
            test_scenario_terrane_native_view_list_rebind();
        }
        "46" => {
            test_scenario_terrane_native_view_local_map_storage();
        }
        "47" => {
            test_scenario_terrane_native_view_loop_binders();
        }
        "48" => {
            test_scenario_terrane_native_view_map_entry_iteration();
        }
        "49" => {
            test_scenario_terrane_native_view_tuple_iteration();
        }
        "50" => {
            test_scenario_terrane_native_view_value_iteration();
        }
        "51" => {
            test_scenario_terrane_nested_noncopy_view_loops();
        }
        "52" => {
            test_scenario_terrane_nested_view_loops();
        }
        "53" => {
            test_scenario_terrane_nested_view_ref_value_loop();
        }
        "54" => {
            test_scenario_terrane_nested_view_value_ref_loop();
        }
        "55" => {
            test_scenario_terrane_nested_view_value_value_loop();
        }
        "56" => {
            test_scenario_terrane_shared_loop_read_control();
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
