// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let t: Token;
    let read: std::sync::Arc<dyn Fn() -> Result<u64, TerraneError> + Send + Sync>;
    t = __terrane_raised(
        terrane_static_trn_546f6b656e_new(),
        0 /* terrane-site: src/main.trn:7:9-7:20 */,
    );
    read = {
        let t = t.clone();
        std::sync::Arc::new(move || -> Result<u64, TerraneError> {
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
                                        > { Ok(t.clone().value()) }),
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
                            1 /* terrane-site: src/main.trn:9:16-9:24 */,
                        )?,
                    ),
                    1 /* terrane-site: src/main.trn:9:16-9:24 */,
                )?,
            );
        })
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read(),
        2 /* terrane-site: src/main.trn:10:13-10:18 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read(),
        3 /* terrane-site: src/main.trn:11:13-11:18 */))
    );
}
