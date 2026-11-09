// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn read(values: terrane_collection_support::List<Token>) -> Result<u64, TerraneError> {
    return Ok(
        __terrane_raised_err(
            terrane_int_support::coerce::<
                u64,
            >(
                &{
                    let __terrane_native_input_0 = __terrane_raised_err(
                        values
                            .get_or_error(
                                __terrane_raised_err(
                                    terrane_collection_support::index_from_int(
                                        &terrane_int_support::Int::from(0_i128),
                                    ),
                                    0 /* terrane-site: src/main.trn:5:12-5:21 */,
                                )?,
                            ),
                        0 /* terrane-site: src/main.trn:5:12-5:21 */,
                    )?;
                    __terrane_raised_err(
                        || -> Result<_, crate::TerraneForeignError> {
                            {
                                let value = match std::panic::catch_unwind(
                                    std::panic::AssertUnwindSafe(|| -> Result<
                                        _,
                                        crate::TerraneForeignError,
                                    > { Ok(__terrane_native_input_0.value()) }),
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
                        1 /* terrane-site: src/main.trn:5:12-5:28 */,
                    )?
                },
            ),
            1 /* terrane-site: src/main.trn:5:12-5:28 */,
        )?,
    );
}
fn main() {
    let values: terrane_collection_support::List<Token>;
    values = terrane_collection_support::List::<
        Token,
    >::new(
        vec![
            __terrane_raised(terrane_static_trn_546f6b656e_new(), 2 /* terrane-site: src/main.trn:7:35-7:46 */)
        ],
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(read(values),
        3 /* terrane-site: src/main.trn:8:13-8:25 */))
    );
}
