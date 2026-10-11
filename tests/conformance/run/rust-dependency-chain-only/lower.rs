// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let database: Database;
        let answer: terrane_int_support::Int;
        let prefix: String;
        let rendered: String;
        database = __terrane_traced(
            __terrane_await({
                    let __terrane_future = memory_database();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            0 /* terrane-site: src/main.trn:5:20-5:36 */,
                        )
                    }
                })
                .await,
            0 /* terrane-site: src/main.trn:5:20-5:36 */,
        );
        answer = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = terrane_chain_witness::query_scalar(
                                &database,
                                String::from("select ?1"),
                            )
                            .bind(
                                match || -> Result<_, crate::TerraneForeignError> {
                                    Ok(
                                        terrane_int_support::coerce::<
                                            i64,
                                        >(&terrane_int_support::Int::from(42_i128))
                                            .map_err(|error| crate::TerraneForeignError(
                                                crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                            ))?,
                                    )
                                }() {
                                    Ok(value) => value,
                                    Err(error) => std::panic::panic_any(error),
                                },
                            )
                            .fetch_one();
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    __terrane_call,
                                )
                                .await
                            {
                                Ok(Ok(value)) => {
                                    Ok(terrane_int_support::Int::from(i128::from(value)))
                                }
                                Ok(Err(error)) => {
                                    Err(
                                        crate::TerraneForeignError(
                                            crate::TerraneError::custom_raised(
                                                crate::DescriptorId(2),
                                                error.to_string(),
                                                crate::TERRANE_NO_SITE,
                                            ),
                                        ),
                                    )
                                }
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane_chain_witness",
                                            "terrane_chain_witness::ScalarQuery::fetch_one",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            1 /* terrane-site: src/main.trn:6:22-6:81 */,
                        )
                    }
                })
                .await,
            1 /* terrane-site: src/main.trn:6:22-6:81 */,
        );
        prefix = String::from("value=");
        rendered = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| {
                    terrane_chain_witness::line(&prefix)
                        .number(
                            match || -> Result<_, crate::TerraneForeignError> {
                                Ok(
                                    terrane_int_support::coerce::<i64>(&answer.clone())
                                        .map_err(|error| crate::TerraneForeignError(
                                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                                        ))?,
                                )
                            }() {
                                Ok(value) => value,
                                Err(error) => std::panic::panic_any(error),
                            },
                        )
                        .finish()
                }),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "terrane_chain_witness",
                            "terrane_chain_witness::LineBuilder::finish",
                        ),
                    )
                }
            },
            2 /* terrane-site: src/main.trn:8:21-8:60 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&answer));
        println!("{}", terrane_scalar_support::scalar_text(&rendered));
    });
}
