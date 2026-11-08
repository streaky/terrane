// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Cfg {
    pub sql: String,
}
impl Cfg {
    pub fn terrane_construct() -> Self {
        Self {
            sql: String::from("insert"),
        }
    }
}
#[derive(Clone)]
pub struct Outer {
    pub cfg: Cfg,
}
impl Outer {
    pub fn terrane_construct() -> Self {
        Self {
            cfg: Cfg::terrane_construct(),
        }
    }
}
pub struct Wrapped {
    pub inner: Option<Holder>,
}
impl Wrapped {
    pub fn terrane_construct(inner: Holder) -> Self {
        let mut __terrane_constructed_value = Self { inner: None };
        __terrane_constructed_value.construct(inner);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, inner: Holder) {
        {
            let __terrane_value: Holder = inner;
            self.inner = Some(__terrane_value);
        }
    }
}
fn callback_view<'view>(cfg: &'view Config) -> terrane_query_witness::View<'view> {
    return __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(terrane_query_witness::query(&cfg.sql)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::query",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        0 /* terrane-site: src/main.trn:18:10-18:24 */,
    );
}
fn observe_cfg(cfg: &Cfg) {
    let view: terrane_query_witness::View<'_>;
    view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(terrane_query_witness::query(&cfg.sql)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::query",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        1 /* terrane-site: src/main.trn:21:10-21:24 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&view).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 2 /* terrane-site: src/main.trn:22:11-22:21 */))
    );
}
fn observe_native(wrapped: &Wrapped) {
    let view: terrane_query_witness::View<'_>;
    view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*(*wrapped)
                                .inner
                                .as_ref()
                                .expect("required field initialized"))
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
        3 /* terrane-site: src/main.trn:25:10-25:29 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&view).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 4 /* terrane-site: src/main.trn:26:11-26:21 */))
    );
}
fn main() {
    let cfg: Cfg;
    let nested: Outer;
    let strings: terrane_collection_support::List<String>;
    let direct: terrane_query_witness::View<'_>;
    let indirect: terrane_query_witness::View<'_>;
    let indexed: terrane_query_witness::View<'_>;
    let wrapped: Wrapped;
    let view: terrane_query_witness::View<'_>;
    let native_cfg: Config;
    cfg = Cfg::terrane_construct();
    nested = Outer::terrane_construct();
    strings = terrane_collection_support::List::<
        String,
    >::new(vec![String::from("indexed")]);
    direct = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(terrane_query_witness::query(&cfg.sql)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::query",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        5 /* terrane-site: src/main.trn:32:12-32:26 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&direct).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 6 /* terrane-site: src/main.trn:33:11-33:23 */))
    );
    indirect = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > { Ok(terrane_query_witness::query(&nested.cfg.sql)) }),
                ) {
                    Ok(result) => result?,
                    Err(payload) => {
                        return Err(
                            crate::__terrane_dependency_panic(
                                payload,
                                "terrane-query-witness",
                                "terrane_query_witness::query",
                            ),
                        );
                    }
                };
                Ok::<_, crate::TerraneForeignError>(value)
            }
        }(),
        7 /* terrane-site: src/main.trn:34:14-34:35 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&indirect).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 8 /* terrane-site: src/main.trn:35:11-35:25 */))
    );
    indexed = {
        let __terrane_index_0 = __terrane_raised(
            terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            ),
            9 /* terrane-site: src/main.trn:36:20-36:30 */,
        );
        let __terrane_index_place_1 = __terrane_raised(
            strings
                .get(__terrane_index_0)
                .ok_or_else(|| terrane_collection_support::IndexError::from_usize(
                    __terrane_index_0,
                )),
            9 /* terrane-site: src/main.trn:36:20-36:30 */,
        );
        let __terrane_native_reference_0 = &*__terrane_index_place_1;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > {
                            Ok(
                                terrane_query_witness::query(__terrane_native_reference_0),
                            )
                        }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane-query-witness",
                                    "terrane_query_witness::query",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            10 /* terrane-site: src/main.trn:36:13-36:30 */,
        )
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&indexed).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 11 /* terrane-site: src/main.trn:37:11-37:24 */))
    );
    observe_cfg(&cfg);
    wrapped = Wrapped::terrane_construct(
        __terrane_raised(
            terrane_static_trn_486f6c646572_new(),
            12 /* terrane-site: src/main.trn:39:32-39:44 */,
        ),
    );
    view = __terrane_raised(
        || -> Result<_, crate::TerraneForeignError> {
            {
                let value = match std::panic::catch_unwind(
                    std::panic::AssertUnwindSafe(|| -> Result<
                        _,
                        crate::TerraneForeignError,
                    > {
                        Ok(
                            (&*wrapped
                                .inner
                                .as_ref()
                                .expect("required field initialized"))
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
        13 /* terrane-site: src/main.trn:40:10-40:29 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result < _,
        crate ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok((&view).size()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 14 /* terrane-site: src/main.trn:41:11-41:21 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_receiver_value_2 =
        &__terrane_raised(| | -> Result < _, crate ::TerraneForeignError > { { let value
        = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
        crate ::TerraneForeignError > { Ok((&* wrapped.inner.as_ref()
        .expect("required field initialized")).view()) })) { Ok(result) => result ?,
        Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::Holder::view")) }; Ok:: < _,
        crate ::TerraneForeignError > (value) } } (), 15 /* terrane-site: src/main.trn:42:12-42:31 */); let __terrane_native_input_0 =
        &__terrane_receiver_value_2; __terrane_raised(| | -> Result < _, crate
        ::TerraneForeignError > { { let value = match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _, crate
        ::TerraneForeignError > { Ok(__terrane_native_input_0.size()) })) { Ok(result) =>
        result ?, Err(payload) => return Err(crate ::__terrane_dependency_panic(payload,
        "terrane_query_witness", "terrane_query_witness::View::size")) }; Ok:: < _, crate
        ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128)) } }
        (), 16 /* terrane-site: src/main.trn:42:11-42:38 */) })
    );
    observe_native(&wrapped);
    native_cfg = __terrane_raised(
        terrane_static_trn_436f6e666967_new(),
        17 /* terrane-site: src/main.trn:44:16-44:28 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let __terrane_conversion_input_1 =
        callback_view; __terrane_raised(render(&native_cfg,
        __terrane_conversion_input_1), 18 /* terrane-site: src/main.trn:45:11-45:44 */) })
    );
}
