// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_local.rs, async_dependency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let database: Database = __terrane_traced(
            __terrane_await({
                    let __terrane_future = memory_database();
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            0 /* terrane-site: src/main.trn:6:20-6:36 */,
                        )
                    }
                })
                .await,
            0 /* terrane-site: src/main.trn:6:20-6:36 */,
        );
        let answer: terrane_int_support::Int = __terrane_traced(
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
                                                crate::TERRANE_DEPENDENCY_ERROR,
                                                format!(
                                                    "Rust dependency `terrane_chain_witness` member `terrane_chain_witness::ScalarQuery<'_>::fetch_one` failed: {error}"
                                                ),
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
                                            "terrane_chain_witness::ScalarQuery<'_>::fetch_one",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            1 /* terrane-site: src/main.trn:7:22-7:81 */,
                        )
                    }
                })
                .await,
            1 /* terrane-site: src/main.trn:7:22-7:81 */,
        );
        let prefix: String = String::from("value=");
        let rendered: String = __terrane_raised(
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
                            "terrane_chain_witness::LineBuilder<'_>::finish",
                        ),
                    )
                }
            },
            2 /* terrane-site: src/main.trn:9:21-9:60 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&answer));
        println!("{}", terrane_scalar_support::scalar_text(&rendered));
    });
}
// Source: <terrane>/projected/deps/terrane-chain-witness.trn
// Namespace: deps/terrane-chain-witness
pub use terrane_chain_witness::Database;
pub async fn memory_database() -> Result<Database, crate::TerraneForeignError> {
    match crate::__terrane_dependency_await_unwind(
            terrane_chain_witness::memory_database(),
        )
        .await
    {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(error)) => {
            Err(
                crate::TerraneForeignError(
                    crate::TerraneError::custom_raised(
                        crate::TERRANE_DEPENDENCY_ERROR,
                        format!(
                            "Rust dependency `terrane-chain-witness` member `terrane_chain_witness::memory_database` failed: {error}"
                        ),
                        crate::TERRANE_NO_SITE,
                    ),
                ),
            )
        }
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-chain-witness",
                    "terrane_chain_witness::memory_database",
                ),
            )
        }
    }
}
