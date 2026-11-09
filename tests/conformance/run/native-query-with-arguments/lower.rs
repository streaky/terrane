// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let mut connection: Connection;
        let mut arguments: Arguments;
        let sql: SqlToken;
        let result: StoreResult;
        connection = __terrane_raised(
            terrane_static_trn_436f6e6e656374696f6e_new(),
            0 /* terrane-site: src/main.trn:6:16-6:32 */,
        );
        arguments = __terrane_raised(
            terrane_static_trn_417267756d656e7473_new(),
            1 /* terrane-site: src/main.trn:7:15-7:30 */,
        );
        {
            let __terrane_conversion_input_0 = terrane_int_support::Int::from(37_i128);
            __terrane_raised(
                || -> Result<_, crate::TerraneForeignError> {
                    {
                        let _ = match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| -> Result<
                                _,
                                crate::TerraneForeignError,
                            > {
                                Ok({
                                    let _ = arguments
                                        .push(
                                            terrane_int_support::coerce::<
                                                i64,
                                            >(&__terrane_conversion_input_0)
                                                .map_err(|error| crate::TerraneForeignError(
                                                    crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                                ))?,
                                        );
                                })
                            }),
                        ) {
                            Ok(result) => result?,
                            Err(payload) => {
                                return Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "native_query_witness",
                                        "native_query_witness::Arguments::push",
                                    ),
                                );
                            }
                        };
                        Ok::<_, crate::TerraneForeignError>(())
                    }
                }(),
                2 /* terrane-site: src/main.trn:8:3-8:21 */,
            )
        };
        sql = {
            let __terrane_conversion_input_0 = String::from("insert");
            __terrane_raised(
                terrane_static_trn_53716c546f6b656e_new(__terrane_conversion_input_0),
                3 /* terrane-site: src/main.trn:9:18-9:41 */,
            )
        };
        result = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = native_query_witness::query_with(
                                sql,
                                arguments,
                            )
                            .persistent(false)
                            .execute(&mut connection);
                        Box::pin(async move {
                            {
                                let value = crate::__terrane_dependency_await_unwind(
                                        __terrane_call,
                                    )
                                    .await
                                    .map_err(|payload| crate::__terrane_dependency_panic(
                                        payload,
                                        "native_query_witness",
                                        "native_query_witness::Query::execute",
                                    ))?;
                                Ok::<_, crate::TerraneForeignError>(value)
                            }
                        })
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            4 /* terrane-site: src/main.trn:10:30-10:103 */,
                        )
                    }
                })
                .await,
            4 /* terrane-site: src/main.trn:10:30-10:103 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&result).rows_affected()) })) {
            Ok(result) => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "native_query_witness",
            "native_query_witness::StoreResult::rows_affected")) }; Ok:: < _, crate
            ::TerraneForeignError > (terrane_int_support::Int::from_u128(value as u128))
            } } (), 5 /* terrane-site: src/main.trn:11:11-11:32 */))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(| | -> Result <
            _, crate ::TerraneForeignError > { { let value = match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | -> Result < _,
            crate ::TerraneForeignError > { Ok((&result).persistent()) })) { Ok(result)
            => result ?, Err(payload) => return Err(crate
            ::__terrane_dependency_panic(payload, "native_query_witness",
            "native_query_witness::StoreResult::persistent")) }; Ok:: < _, crate
            ::TerraneForeignError > (value) } } (), 6 /* terrane-site: src/main.trn:12:11-12:29 */))
        );
    });
}
