// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let chosen: witness::Choice<i64>;
    let value: terrane_int_support::Int;
    let named: witness::Choice<i64>;
    chosen = {
        let __terrane_native_enum_input_0 = terrane_int_support::Int::from(41_i128);
        let __terrane_native_enum_payload_0 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(
                    terrane_int_support::coerce::<i64>(&__terrane_native_enum_input_0)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }(),
            0 /* terrane-site: src/main.trn:5:28-5:54 */,
        );
        witness::Choice::<i64>::Value(__terrane_native_enum_payload_0)
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(score(&chosen),
        1 /* terrane-site: src/main.trn:6:13-6:26 */))
    );
    match &chosen {
        witness::Choice::Empty => {
            println!("{}", terrane_scalar_support::scalar_text(&0));
        }
        witness::Choice::Value(__terrane_pattern_228_value) => {
            let __terrane_pattern_228_value = terrane_int_support::Int::from(
                i128::from(*__terrane_pattern_228_value),
            );
            value = __terrane_pattern_228_value;
            println!(
                "{}", terrane_scalar_support::scalar_text(&(value.clone() +
                terrane_int_support::Int::from(1_i128)))
            );
        }
        witness::Choice::Named { value: _, enabled: _ } => {
            println!("{}", terrane_scalar_support::scalar_text(&0));
        }
    }
    named = {
        let __terrane_native_enum_input_0 = true;
        let __terrane_native_enum_payload_0 = __terrane_native_enum_input_0;
        let __terrane_native_enum_input_1 = terrane_int_support::Int::from(23_i128);
        let __terrane_native_enum_payload_1 = __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                Ok(
                    terrane_int_support::coerce::<i64>(&__terrane_native_enum_input_1)
                        .map_err(|error| crate::TerraneForeignError(
                            crate::TerraneRaised::raised(error, crate::TERRANE_NO_SITE),
                        ))?,
                )
            }(),
            2 /* terrane-site: src/main.trn:14:27-14:77 */,
        );
        witness::Choice::<i64>::Named {
            value: __terrane_native_enum_payload_1,
            enabled: __terrane_native_enum_payload_0,
        }
    };
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(score(&named),
        3 /* terrane-site: src/main.trn:15:13-15:25 */))
    );
}
