// Generated deterministically by Terrane <version>.
// Runtime support: mutable_callable.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: src/main.trn
// Namespace: app
fn main() {
    let total: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    let callback: TerraneMutableCallable<
        (terrane_int_support::Int,),
        terrane_int_support::Int,
    > = {
        let mut total = total.clone();
        TerraneMutableCallable::new(move |
            (value,): (terrane_int_support::Int,),
        | -> terrane_int_support::Int {
            total = total.clone() + value.clone();
            return total.clone();
        })
    };
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(apply_mutable(terrane_int_support::Int::from(1_i128),
        callback.clone()), 0 /* terrane-site: src/main.trn:8:13-8:39 */))
    );
}
// Source: <terrane>/projected/deps/terrane-callback-witness.trn
// Namespace: deps/terrane-callback-witness
pub fn apply_mutable(
    value: terrane_int_support::Int,
    callback: TerraneMutableCallable<
        (terrane_int_support::Int,),
        terrane_int_support::Int,
    >,
) -> Result<terrane_int_support::Int, crate::TerraneForeignError> {
    let value = terrane_int_support::coerce::<i64>(&value)
        .map_err(|error| crate::TerraneForeignError(
            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
        ))?;
    let callback = {
        let callback = callback.clone();
        move |callback_argument_0: i64| {
            match || -> Result<_, crate::TerraneForeignError> {
                let callback_value = callback
                    .call((
                        terrane_int_support::Int::from(i128::from(callback_argument_0)),
                    ));
                Ok(
                    terrane_int_support::coerce::<i64>(&callback_value)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }() {
                Ok(value) => value,
                Err(error) => std::panic::panic_any(error.0),
            }
        }
    };
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_callback_witness::apply_mutable(
            value,
            callback,
        )),
    ) {
        Ok(value) => Ok(terrane_int_support::Int::from(i128::from(value))),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-callback-witness",
                    "terrane_callback_witness::apply_mutable",
                ),
            )
        }
    }
}
