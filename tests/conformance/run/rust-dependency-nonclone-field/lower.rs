// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
pub struct Holder {
    pub item: NonClone,
}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {
            item: {
                let __terrane_conversion_input_0 = terrane_int_support::Int::from(
                    7_i128,
                );
                __terrane_raised(
                    make_non_clone(__terrane_conversion_input_0),
                    0 /* terrane-site: src/main.trn:6:20-6:37 */,
                )
            },
        }
    }
    pub fn read(&self) -> terrane_int_support::Int {
        return __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok((&self.item).value()) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "terrane_nonclone_witness",
                                    "terrane_nonclone_witness::NonClone::value",
                                ),
                            );
                        }
                    };
                    Ok::<
                        _,
                        crate::TerraneForeignError,
                    >(terrane_int_support::Int::from(i128::from(value)))
                }
            }(),
            1 /* terrane-site: src/main.trn:9:12-9:28 */,
        );
    }
}
fn main() {
    let value: Holder;
    value = Holder::terrane_construct();
    println!("{}", terrane_scalar_support::scalar_text(&value.read()));
}
