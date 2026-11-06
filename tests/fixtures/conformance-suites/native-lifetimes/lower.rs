// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
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

            0 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:11:9-11:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            1 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:12:9-12:21 */,
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

            2 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:13:9-13:16 */,
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

            3 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:14:9-14:24 */,
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

            4 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:15:9-15:16 */,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 5 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:16:13-16:20 */)),
        terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 6 /* terrane-site: tests/integration/native-loan-chain-rebind-receiver/case.trn:16:24-16:31 */))
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

                7 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                8 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:11:38-11:50 */,
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

            9 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:12:9-12:22 */,
    );
    c = true;
    if c {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    10 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:15:19-15:31 */,
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
            u128)) } } (), 11 /* terrane-site: tests/integration/native-loan-exclusive-branch/case.trn:17:17-17:24 */))
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

            12 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:11:9-11:21 */,
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

            13 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:12:9-12:21 */,
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
                14 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:18:21-18:28 */))
            );
            v = __terrane_raised_completion!(
                | | -> Result < _, crate ::TerraneForeignError > { { let value = match
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
                crate ::TerraneForeignError > { Ok((&h).copy_view()) })) { Ok(result) =>
                result ?, Err(payload) => return Err(crate
                ::__terrane_dependency_panic(payload, "terrane_query_witness",
                "terrane_query_witness::Holder::copy_view")) }; Ok:: < _, crate
                ::TerraneForeignError > (value) } } (), 15 /* terrane-site: tests/integration/native-loan-finally-copy-rebind/case.trn:19:17-19:29 */
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

                16 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                17 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:11:38-11:50 */,
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

                18 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:14:13-14:26 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
            Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
            "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _,
            crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
            u128)) } } (), 19 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:15:17-15:24 */))
        );
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    20 /* terrane-site: tests/integration/native-loan-loop-refreshed/case.trn:16:19-16:31 */,
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

            21 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:11:9-11:21 */,
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

            22 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:12:9-12:16 */,
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

            23 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:13:16-13:23 */,
    );
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            24 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:14:9-14:21 */,
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
        u128)) } } (), 25 /* terrane-site: tests/integration/native-loan-owned-observation/case.trn:16:13-16:25 */))
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

                26 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                27 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:11:38-11:50 */,
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

            28 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:12:9-12:22 */,
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

            29 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:14:15-14:22 */,
    ) > terrane_int_support::Int::from(100_i128)
    {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    30 /* terrane-site: tests/integration/native-loan-predicate-last-use/case.trn:15:23-15:35 */,
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

                31 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                32 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:11:38-11:50 */,
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

            33 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:12:9-12:22 */,
    );
    i = 1;
    if i == 0 {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    34 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:16:23-16:35 */,
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
            u128)) } } (), 35 /* terrane-site: tests/integration/native-loan-predicate-switch/case.trn:18:21-18:28 */))
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

                36 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                37 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:11:38-11:50 */,
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

            38 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:12:9-12:22 */,
    );
    i = 2;
    if i == 0 {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    39 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:16:23-16:35 */,
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
            u128)) } } (), 40 /* terrane-site: tests/integration/native-loan-predicate-three-arms/case.trn:20:21-20:28 */))
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

            41 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:11:9-11:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            42 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:12:9-12:21 */,
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

            43 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:13:9-13:16 */,
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

            44 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:14:9-14:16 */,
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

            45 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:15:9-15:24 */,
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

            46 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:16:5-16:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 47 /* terrane-site: tests/integration/native-loan-rebound-alias-chain/case.trn:17:13-17:20 */))
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

                48 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                49 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:11:38-11:50 */,
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

            50 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:12:9-12:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 51 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:13:13-13:20 */))
    );
    {
        let __terrane_value = __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                52 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:14:15-14:27 */,
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

            53 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:15:9-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 54 /* terrane-site: tests/integration/native-loan-rebound-other/case.trn:16:13-16:20 */))
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

                55 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                56 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:11:38-11:50 */,
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

            57 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:12:9-12:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 58 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:13:13-13:20 */))
    );
    {
        let __terrane_value = __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                59 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:14:15-14:27 */,
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

            60 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:15:9-15:22 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 61 /* terrane-site: tests/integration/native-loan-rebound-same/case.trn:16:13-16:20 */))
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

                62 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:11:22-11:34 */,
        ),
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),

                63 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:11:38-11:50 */,
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

            64 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:12:9-12:22 */,
    );
    c = false;
    if c {
        {
            let __terrane_value = __terrane_raised(
                terrane_static_trn_486f6c646572_new(),

                    65 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:15:19-15:31 */,
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
        (), 66 /* terrane-site: tests/integration/native-loan-return-branch/case.trn:17:13-17:20 */))
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

            67 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:4:9-4:21 */,
    );
    k = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            68 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:5:9-5:21 */,
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

            69 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:6:9-6:16 */,
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

            70 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:8:9-8:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 71 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:9:13-9:20 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&a).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 72 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:10:13-10:20 */))
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

            73 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:11:5-11:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&k).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 74 /* terrane-site: tests/integration/native-loan-storage-alias-last-use/case.trn:12:13-12:25 */))
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

            75 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:11:9-11:21 */,
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

            76 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:12:9-12:16 */,
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
                u128)) } } (), 77 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:14:17-14:24 */))
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
            ::TerraneForeignError > (()) } } (), 78 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:16:9-16:16 */
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
        u128)) } } (), 79 /* terrane-site: tests/integration/native-loan-try-finally-last-use/case.trn:17:13-17:25 */))
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

            80 /* terrane-site: tests/integration/native-loan-try-local/case.trn:11:9-11:21 */,
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
                        ::TerraneForeignError > (value) } } (), 81 /* terrane-site: tests/integration/native-loan-try-local/case.trn:13:13-13:20 */
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
                u128)) } } (), 82 /* terrane-site: tests/integration/native-loan-try-local/case.trn:14:17-14:24 */))
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
                        83 /* terrane-site: tests/integration/native-loan-try-local/case.trn:16:17-16:29 */))
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
        u128)) } } (), 84 /* terrane-site: tests/integration/native-loan-try-local/case.trn:17:13-17:25 */))
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

            85 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:11:9-11:21 */,
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

            86 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 87 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:13:13-13:20 */))
    );
    h = __terrane_raised(
        terrane_static_trn_486f6c646572_new(),

            88 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:14:9-14:21 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 89 /* terrane-site: tests/integration/native-loan-whole-last-use/case.trn:15:13-15:25 */))
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

            90 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:11:9-11:21 */,
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

            91 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 92 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:13:13-13:20 */))
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
        u128)) } } (), 93 /* terrane-site: tests/integration/native-loan-whole-move-last-use/case.trn:15:13-15:29 */))
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

            94 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:11:9-11:21 */,
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

            95 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:12:9-12:16 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&v).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 96 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:13:13-13:20 */))
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

            97 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:14:5-14:12 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&h).label_len()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::label_len")) }; Ok:: <
        _, crate ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as
        u128)) } } (), 98 /* terrane-site: tests/integration/native-loan-whole-mutate-last-use/case.trn:15:13-15:25 */))
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

            99 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:11:9-11:21 */,
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

            100 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:12:9-12:16 */,
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

            101 /* terrane-site: tests/integration/native-loop-consuming-condition-break/case.trn:13:12-13:24 */,
    ) > terrane_int_support::Int::from(0_i128)
    {
        println!("{}", terrane_scalar_support::scalar_text(&1));
        break;
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

            102 /* terrane-site: tests/integration/native-try-local-consume/case.trn:4:9-4:21 */,
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
                        ::TerraneForeignError > (value) } } (), 103 /* terrane-site: tests/integration/native-try-local-consume/case.trn:6:13-6:20 */
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
                u128)) } } (), 104 /* terrane-site: tests/integration/native-try-local-consume/case.trn:7:17-7:29 */))
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
                            105 /* terrane-site: tests/integration/native-try-local-consume/case.trn:13:20-13:27 */
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
                    u128)) } } (), 106 /* terrane-site: tests/integration/native-try-local-consume/case.trn:14:21-14:36 */))
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
            test_scenario_terrane_native_loan_chain_rebind_receiver();
        }
        "1" => {
            test_scenario_terrane_native_loan_exclusive_branch();
        }
        "2" => {
            test_scenario_terrane_native_loan_finally_copy_rebind();
        }
        "3" => {
            test_scenario_terrane_native_loan_loop_refreshed();
        }
        "4" => {
            test_scenario_terrane_native_loan_owned_observation();
        }
        "5" => {
            test_scenario_terrane_native_loan_predicate_last_use();
        }
        "6" => {
            test_scenario_terrane_native_loan_predicate_switch();
        }
        "7" => {
            test_scenario_terrane_native_loan_predicate_three_arms();
        }
        "8" => {
            test_scenario_terrane_native_loan_rebound_alias_chain();
        }
        "9" => {
            test_scenario_terrane_native_loan_rebound_other();
        }
        "10" => {
            test_scenario_terrane_native_loan_rebound_same();
        }
        "11" => {
            test_scenario_terrane_native_loan_return_branch();
        }
        "12" => {
            test_scenario_terrane_native_loan_storage_alias_last_use();
        }
        "13" => {
            test_scenario_terrane_native_loan_try_finally_last_use();
        }
        "14" => {
            test_scenario_terrane_native_loan_try_local();
        }
        "15" => {
            test_scenario_terrane_native_loan_whole_last_use();
        }
        "16" => {
            test_scenario_terrane_native_loan_whole_move_last_use();
        }
        "17" => {
            test_scenario_terrane_native_loan_whole_mutate_last_use();
        }
        "18" => {
            test_scenario_terrane_native_loop_consuming_condition_break();
        }
        "19" => {
            test_scenario_terrane_native_try_local_consume();
        }
        _ => {
            eprintln!("unknown Terrane test case");
            std::process::exit(103);
        }
    }
}
