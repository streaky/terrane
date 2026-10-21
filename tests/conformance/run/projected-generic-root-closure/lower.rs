// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn payload_value(value: &Payload) -> terrane_int_support::Int {
    return terrane_int_support::Int::from(
        {
            let __terrane_snapshot = value.0;
            __terrane_snapshot
        } as i128,
    );
}
fn main() {
    let holder: Holder;
    let selected: Option<terrane_int_support::Int>;
    holder = __terrane_raised(
        make_holder(),
        0 /* terrane-site: src/main.trn:9:14-9:26 */,
    );
    selected = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                (&holder)
                    .values()
                    .get()
                    .map(move |borrowed_argument_0| {
                        match || -> Result<_, crate::TerraneForeignError> {
                            let callback_value = payload_value(borrowed_argument_0);
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
                    })
            }),
        ) {
            Ok(value) => {
                Ok(value.map(|value| terrane_int_support::Int::from(i128::from(value))))
            }
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "dependency",
                        "std::option::Option::map",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:10:25-10:67 */,
    );
    if (&selected).is_some() {
        println!(
            "{}", terrane_scalar_support::scalar_text(&match &selected { Some(value) =>
            value, _ => unreachable!("flow-proven storage refinement") })
        );
    } else {
        println!("{}", String::from("missing"));
    }
}
