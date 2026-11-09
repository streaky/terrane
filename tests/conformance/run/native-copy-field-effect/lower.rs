// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct WrappedToken {
    pub t: Token,
}
impl WrappedToken {
    pub fn terrane_construct() -> Self {
        Self {
            t: __terrane_raised(
                terrane_static_trn_546f6b656e_new(),
                0 /* terrane-site: src/main.trn:7:15-7:26 */,
            ),
        }
    }
    pub fn read(&self) -> Result<u64, TerraneError> {
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
                                    > { Ok(self.t.clone().value()) }),
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
                        1 /* terrane-site: src/main.trn:9:16-9:29 */,
                    )?,
                ),
                1 /* terrane-site: src/main.trn:9:16-9:29 */,
            )?,
        );
    }
}
fn main() {
    let item: WrappedToken;
    item = WrappedToken::terrane_construct();
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(item.read(),
        2 /* terrane-site: src/main.trn:13:13-13:23 */))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(item.read(),
        3 /* terrane-site: src/main.trn:14:13-14:23 */))
    );
}
