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
            ::TerraneForeignError > (value) } } (), 15 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:7:20-7:37 */).into();
            value }
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
            ::TerraneForeignError > (value) } } (), 18 /* terrane-site: tests/integration/native-copy-view-list-lender/case.trn:10:28-10:45 */)
            .into(); value }])
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
    >::new(
        vec![{ let value : terrane_query_witness::CopyView < '_ > = copy.into(); value }],
    );
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
    let __terrane_iterable_0 = appended;
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

                    32 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:7:18-7:29 */,
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

                    33 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:8:18-8:31 */,
            );
            self.e = Some(__terrane_value);
        }
    }
}
fn observe(
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

                        34 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:10:12-10:22 */,
                )?,
            ),

                34 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:10:12-10:22 */,
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

                        35 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:12:12-12:19 */,
                )?,
            ),

                35 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:12:12-12:19 */,
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
            } } (), 36 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:17:17-17:27 */)
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
            } } (), 37 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:18:17-18:29 */)
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
            } } (), 38 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:19:17-19:26 */)
            ?)
        );
        i = i.clone() + terrane_int_support::Int::from(1_i128);
    }
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(observe(&r),
        39 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:21:13-21:23 */) ?)
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

            40 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:22:9-22:22 */,
    )?;
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(clone_observe(&mut e),
        41 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:23:13-23:29 */) ?)
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_traced_err(clone_observe(&mut e),
        42 /* terrane-site: tests/integration/native-field-copy-clone-conversion/case.trn:24:13-24:29 */) ?)
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

            43 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:4:9-4:21 */,
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

                    44 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:5:9-5:16 */,
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
                u128)) } } (), 45 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:7:17-7:29 */))
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
                    ::TerraneForeignError > (value) } } (), 46 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:9:13-9:20 */
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
            u128)) } } (), 47 /* terrane-site: tests/integration/native-finally-only-rewrite/case.trn:10:17-10:24 */))
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

            48 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:5:12-5:23 */,
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

            49 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:8:12-8:23 */,
    );
}
fn test_scenario_terrane_native_inferred_copy_view() {
    let holder: Holder;
    let mut inferred_view: terrane_query_witness::CopyView<'_>;
    let inferred: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    let typed: terrane_collection_support::List<terrane_query_witness::CopyView<'_>>;
    holder = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            50 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:11:14-11:26 */,
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

            51 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:12:21-12:38 */,
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
            ::TerraneForeignError > (value) } } (), 52 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:15:23-15:40 */).into();
            value }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(inferred
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        53 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:24 */)),
        53 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:24 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 54 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:16:13-16:30 */) })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(inferred
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        55 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:24 */)),
        55 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:24 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 56 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:17:13-17:30 */) })
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
            ::TerraneForeignError > (value) } } (), 57 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:18:37-18:54 */).into();
            value }
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_native_input_0 =
        __terrane_raised(typed
        .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
        58 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:21 */)),
        58 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:21 */);
        __terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value =
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) {
        Ok(result) => result ?, Err(payload) => return Err(crate
        ::__terrane_dependency_panic(payload, "terrane_query_witness",
        "terrane_query_witness::CopyView::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 59 /* terrane-site: tests/integration/native-inferred-copy-view/case.trn:19:13-19:27 */) })
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

            60 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:11:9-11:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            61 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:12:9-12:21 */,
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

            62 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:13:9-13:16 */,
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

            63 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:14:9-14:24 */,
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

            64 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:15:9-15:16 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 65 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:16:13-16:20 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 66 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:16:24-16:31 */))
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

                67 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                68 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:11:38-11:50 */,
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

            69 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:12:9-12:22 */,
    );
    c = true;
    if c {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    70 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:15:19-15:31 */,
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
            u128)) } } (), 71 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:17:17-17:24 */))
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

            72 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:11:9-11:21 */,
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

            73 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:12:9-12:21 */,
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
                74 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:18:21-18:28 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).copy_view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 75 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:19:17-19:29 */
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

                76 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                77 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:11:38-11:50 */,
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

                78 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:14:13-14:26 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 79 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:15:17-15:24 */))
        );
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    80 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:16:19-16:31 */,
            );
            w.inner = Some(__terrane_value);
        }
        i = i.clone() + terrane_int_support::Int::from(1_i128);
    }
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

            81 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:11:9-11:21 */,
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

            82 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:12:9-12:16 */,
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

            83 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:13:16-13:23 */,
    );
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            84 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:14:9-14:21 */,
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
        u128)) } } (), 85 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:16:13-16:25 */))
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

                86 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                87 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:11:38-11:50 */,
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

            88 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:12:9-12:22 */,
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

            89 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:14:15-14:22 */,
    ) > terrane_int_support::Int::from(100_i128)
    {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    90 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:15:23-15:35 */,
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

                91 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                92 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:11:38-11:50 */,
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

            93 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:12:9-12:22 */,
    );
    i = 1;
    if i == 0 {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    94 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:16:23-16:35 */,
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
            u128)) } } (), 95 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:18:21-18:28 */))
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

                96 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                97 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:11:38-11:50 */,
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

            98 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:12:9-12:22 */,
    );
    i = 2;
    if i == 0 {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    99 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:16:23-16:35 */,
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
            u128)) } } (), 100 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:20:21-20:28 */))
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

            101 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:11:9-11:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            102 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:12:9-12:21 */,
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

            103 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:13:9-13:16 */,
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

            104 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:14:9-14:16 */,
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

            105 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:15:9-15:24 */,
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

            106 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:16:5-16:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 107 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:17:13-17:20 */))
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

                108 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                109 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:11:38-11:50 */,
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

            110 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:12:9-12:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 111 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:13:13-13:20 */))
    );
    {
        let __terrane_value = __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                112 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:14:15-14:27 */,
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

            113 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:15:9-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 114 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:16:13-16:20 */))
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

                115 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                116 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:11:38-11:50 */,
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

            117 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:12:9-12:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 118 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:13:13-13:20 */))
    );
    {
        let __terrane_value = __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                119 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:14:15-14:27 */,
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

            120 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:15:9-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 121 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:16:13-16:20 */))
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

                122 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                123 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:11:38-11:50 */,
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

            124 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:12:9-12:22 */,
    );
    c = false;
    if c {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    125 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:15:19-15:31 */,
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
        (), 126 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:17:13-17:20 */))
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

            127 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            128 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:5:9-5:21 */,
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

            129 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:6:9-6:16 */,
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

            130 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:8:9-8:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 131 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:9:13-9:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 132 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:10:13-10:20 */))
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

            133 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:11:5-11:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&k).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 134 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:12:13-12:25 */))
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

            135 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:11:9-11:21 */,
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

            136 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:12:9-12:16 */,
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
                u128)) } } (), 137 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:14:17-14:24 */))
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
            ::TerraneForeignError > (()) } } (), 138 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:16:9-16:16 */
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
        u128)) } } (), 139 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:17:13-17:25 */))
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

            140 /* terrane-site: tests/integration/native-loan-try-local/case.trn:11:9-11:21 */,
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
                        ::TerraneForeignError > (value) } } (), 141 /* terrane-site: tests/integration/native-loan-try-local/case.trn:13:13-13:20 */
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
                u128)) } } (), 142 /* terrane-site: tests/integration/native-loan-try-local/case.trn:14:17-14:24 */))
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
                        143 /* terrane-site: tests/integration/native-loan-try-local/case.trn:16:17-16:29 */))
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
        u128)) } } (), 144 /* terrane-site: tests/integration/native-loan-try-local/case.trn:17:13-17:25 */))
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

            145 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:11:9-11:21 */,
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

            146 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 147 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:13:13-13:20 */))
    );
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            148 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:14:9-14:21 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 149 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:15:13-15:25 */))
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

            150 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:11:9-11:21 */,
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

            151 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 152 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:13:13-13:20 */))
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
        u128)) } } (), 153 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:15:13-15:29 */))
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

            154 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:11:9-11:21 */,
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

            155 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 156 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:13:13-13:20 */))
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

            157 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:14:5-14:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 158 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:15:13-15:25 */))
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

            159 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:11:9-11:21 */,
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

            160 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:12:9-12:16 */,
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

            161 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:13:12-13:24 */,
    ) > terrane_int_support::Int::from(0_i128)
    {
        println!("{}", terrane_scalar_support::scalar_text(&1));
        break;
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

            162 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:4:13-4:24 */,
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

            163 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:5:14-5:25 */,
    )?;
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<u64>(&(first.clone() + second.clone())),

                164 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:6:12-6:26 */,
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

            165 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:8:13-8:25 */,
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

                            166 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:9:21-9:33 */,
                    )?),
            ),

                167 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:9:12-9:34 */,
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

            168 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:11:13-11:26 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(twice(&mut
        value), 169 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:12:13-12:25 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(value.clone().code()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::CloneEnum::code")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 170 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:13:13-13:24 */) ?)
    );
    token = __terrane_raised_err(
        terrane_static_trn_546f6b656e_new(),

            171 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:14:13-14:24 */,
    )?;
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(copied(&mut
        token), 172 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:15:13-15:26 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised_err(| | -> Result <
        _, crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(token.value()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Token::value")) }; Ok:: < _,
        crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 173 /* terrane-site: tests/integration/native-ref-clone-repeat/case.trn:16:13-16:25 */) ?)
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

                    174 /* terrane-site: tests/integration/native-required-enum-field/case.trn:7:22-7:28 */,
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

                            175 /* terrane-site: tests/integration/native-required-enum-field/case.trn:9:16-9:32 */,
                    )?,
                ),

                    175 /* terrane-site: tests/integration/native-required-enum-field/case.trn:9:16-9:32 */,
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
        (), 176 /* terrane-site: tests/integration/native-required-enum-field/case.trn:16:13-16:30 */) ?)
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
        (), 177 /* terrane-site: tests/integration/native-required-enum-field/case.trn:19:13-19:36 */) ?)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&outer.inner.as_ref()
        .expect("required field initialized").sibling)
    );
    owned = TerraneNsNativeRequiredEnumFieldRIdentity2F6E61746976652D72657175697265642D656E756D2D6669656C640052::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced_err(owned.finish(),
        178 /* terrane-site: tests/integration/native-required-enum-field/case.trn:22:13-22:26 */) ?)
    );
    Ok(())
}
// Source: tests/integration/native-try-body-rewrite-control/case.trn
// Namespace: native-try-body-rewrite-control
fn test_scenario_terrane_native_try_body_rewrite_control() {
    let h: Holder;
    let mut v: terrane_query_witness::View<'_>;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            179 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:4:9-4:21 */,
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

            180 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:5:9-5:16 */,
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
                u128)) } } (), 181 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:7:17-7:29 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 182 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:8:13-8:20 */
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
                u128)) } } (), 183 /* terrane-site: tests/integration/native-try-body-rewrite-control/case.trn:9:17-9:24 */))
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

            184 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:4:9-4:21 */,
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

            185 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:5:9-5:16 */,
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
                u128)) } } (), 186 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:7:17-7:29 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 187 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:8:13-8:20 */
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
                        ::TerraneForeignError > (value) } } (), 188 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:10:13-10:20 */
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
                        189 /* terrane-site: tests/integration/native-try-catch-dominating-rewrite/case.trn:11:17-11:24 */))
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

            190 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            191 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:5:9-5:21 */,
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

            192 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:6:9-6:21 */,
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
                193 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:8:17-8:24 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&k).copy_view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 194 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:9:13-9:25 */
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
            as u128)) } } (), 195 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:11:17-11:24 */))
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
        u128)) } } (), 196 /* terrane-site: tests/integration/native-try-copy-finally-control/case.trn:12:13-12:20 */))
    );
}
// Source: tests/integration/native-try-finally-dominating-rewrite/case.trn
// Namespace: native-try-finally-dominating-rewrite
fn test_scenario_terrane_native_try_finally_dominating_rewrite() {
    let h: Holder;
    let mut v: Option<terrane_query_witness::View<'_>> = None;
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            197 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:4:9-4:21 */,
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

                    198 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:5:9-5:16 */,
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
                u128)) } } (), 199 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:7:17-7:29 */))
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
                        ::TerraneForeignError > (value) } } (), 200 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:8:13-8:20 */
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
                    ::TerraneForeignError > (value) } } (), 201 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:10:13-10:20 */
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
            u128)) } } (), 202 /* terrane-site: tests/integration/native-try-finally-dominating-rewrite/case.trn:11:17-11:24 */))
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

            203 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:4:9-4:21 */,
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

                    204 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:5:9-5:16 */,
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
            as u128)) } } (), 205 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:9:17-9:29 */))
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
                    ::TerraneForeignError > (value) } } (), 206 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:10:13-10:20 */
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
            u128)) } } (), 207 /* terrane-site: tests/integration/native-try-finally-rewrite-control/case.trn:11:17-11:24 */))
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

            208 /* terrane-site: tests/integration/native-try-local-consume/case.trn:4:9-4:21 */,
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
                        ::TerraneForeignError > (value) } } (), 209 /* terrane-site: tests/integration/native-try-local-consume/case.trn:6:13-6:20 */
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
                u128)) } } (), 210 /* terrane-site: tests/integration/native-try-local-consume/case.trn:7:17-7:29 */))
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
                            211 /* terrane-site: tests/integration/native-try-local-consume/case.trn:13:20-13:27 */
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
                    u128)) } } (), 212 /* terrane-site: tests/integration/native-try-local-consume/case.trn:14:21-14:36 */))
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

            213 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            214 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:5:9-5:21 */,
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

                    215 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:6:9-6:16 */,
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
                u128)) } } (), 216 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:8:17-8:29 */))
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
                        ::TerraneForeignError > (value) } } (), 217 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:9:13-9:20 */
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

                    218 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:12:9-12:16 */,
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
        (), 219 /* terrane-site: tests/integration/native-try-unconditional-rewrite-control/case.trn:13:13-13:20 */))
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
            if let Err(error) = test_scenario_terrane_native_field_copy_clone_conversion() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "5" => {
            test_scenario_terrane_native_finally_only_rewrite();
        }
        "6" => {
            test_scenario_terrane_native_inferred_copy_view();
        }
        "7" => {
            test_scenario_terrane_native_loan_chain_rebind_receiver();
        }
        "8" => {
            test_scenario_terrane_native_loan_exclusive_branch();
        }
        "9" => {
            test_scenario_terrane_native_loan_finally_copy_rebind();
        }
        "10" => {
            test_scenario_terrane_native_loan_loop_refreshed();
        }
        "11" => {
            test_scenario_terrane_native_loan_owned_observation();
        }
        "12" => {
            test_scenario_terrane_native_loan_predicate_last_use();
        }
        "13" => {
            test_scenario_terrane_native_loan_predicate_switch();
        }
        "14" => {
            test_scenario_terrane_native_loan_predicate_three_arms();
        }
        "15" => {
            test_scenario_terrane_native_loan_rebound_alias_chain();
        }
        "16" => {
            test_scenario_terrane_native_loan_rebound_other();
        }
        "17" => {
            test_scenario_terrane_native_loan_rebound_same();
        }
        "18" => {
            test_scenario_terrane_native_loan_return_branch();
        }
        "19" => {
            test_scenario_terrane_native_loan_storage_alias_last_use();
        }
        "20" => {
            test_scenario_terrane_native_loan_try_finally_last_use();
        }
        "21" => {
            test_scenario_terrane_native_loan_try_local();
        }
        "22" => {
            test_scenario_terrane_native_loan_whole_last_use();
        }
        "23" => {
            test_scenario_terrane_native_loan_whole_move_last_use();
        }
        "24" => {
            test_scenario_terrane_native_loan_whole_mutate_last_use();
        }
        "25" => {
            test_scenario_terrane_native_loop_consuming_condition_break();
        }
        "26" => {
            if let Err(error) = test_scenario_terrane_native_ref_clone_repeat() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "27" => {
            if let Err(error) = test_scenario_terrane_native_required_enum_field() {
                let skipped = error.descriptor_name() == "/core/testing::test-skip";
                __terrane_test_record(
                    if skipped { "skipped" } else { "failed" },
                    &error,
                );
                std::process::exit(if skipped { 102 } else { 101 });
            }
        }
        "28" => {
            test_scenario_terrane_native_try_body_rewrite_control();
        }
        "29" => {
            test_scenario_terrane_native_try_catch_dominating_rewrite();
        }
        "30" => {
            test_scenario_terrane_native_try_copy_finally_control();
        }
        "31" => {
            test_scenario_terrane_native_try_finally_dominating_rewrite();
        }
        "32" => {
            test_scenario_terrane_native_try_finally_rewrite_control();
        }
        "33" => {
            test_scenario_terrane_native_try_local_consume();
        }
        "34" => {
            test_scenario_terrane_native_try_unconditional_rewrite_control();
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
