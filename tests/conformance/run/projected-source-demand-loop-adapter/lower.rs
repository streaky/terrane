// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn token_value(value: Token) -> Result<u64, TerraneError> {
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
                                > { Ok(value.clone().value()) }),
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
                    0 /* terrane-site: src/main.trn:7:12-7:24 */,
                )?,
            ),
            0 /* terrane-site: src/main.trn:7:12-7:24 */,
        )?,
    );
}
fn main() {
    let values: terrane_collection_support::List<Token>;
    let mut value: Token;
    values = terrane_collection_support::List::<
        Token,
    >::new(
        vec![
            __terrane_raised(terrane_static_trn_546f6b656e_new(), 1 /* terrane-site: src/main.trn:10:35-10:46 */)
        ],
    );
    let __terrane_iterable_0 = values;
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        value = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(token_value(value),
            2 /* terrane-site: src/main.trn:12:17-12:35 */))
        );
    }
}
