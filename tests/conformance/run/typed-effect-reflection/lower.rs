// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: typed-effect-reflection
fn fallible() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
fn exact() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:8:3-8:23 */,
        ),
    );
}
fn main() {
    let value: terrane_int_support::Int = fallible();
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&value),
        terrane_scalar_support::scalar_text(&{ let _ = fallible; "throws".to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = fallible; "throwable"
        .to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = fallible; "".to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = exact; "throws".to_owned()
        })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = exact; "throwable"
        .to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = exact; "coercion-error"
        .to_owned() })
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            __terrane_traced_completion!(
                exact(), 1 /* terrane-site: case.trn:19:5-19:11 */
            );
            TerraneCompletion::Normal
        })();
        match __terrane_try_0 {
            TerraneCompletion::Return(value) => return TerraneCompletion::Return(value),
            TerraneCompletion::Break => return TerraneCompletion::Break,
            TerraneCompletion::Continue => return TerraneCompletion::Continue,
            TerraneCompletion::Normal => {}
            TerraneCompletion::Error(__terrane_error_0) => {
                let mut __terrane_handled_0 = false;
                if !__terrane_handled_0
                    && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                {
                    __terrane_handled_0 = true;
                    println!(
                        "{}",
                        terrane_scalar_support::scalar_text(&String::from("caught"))
                    );
                }
                if !__terrane_handled_0 {
                    return TerraneCompletion::Error(__terrane_error_0);
                }
            }
        }
        TerraneCompletion::Normal
    })();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
