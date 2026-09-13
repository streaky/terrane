// Generated deterministically by Terrane <version>.
// Runtime support: 
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: throwable-function-value
fn render(value: terrane_int_support::Int) -> Result<String, TerraneError> {
    if value.clone() < terrane_int_support::Int::from(0_i128) {
        return Err(
            TerraneError::raised(
                TerraneErrorKind::CoercionError,
                0 /* terrane-site: case.trn:6:5-6:25 */,
            ),
        );
    }
    return Ok(String::from("ok"));
}
fn apply(
    operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
    >,
    value: terrane_int_support::Int,
) -> Result<String, TerraneError> {
    return Ok(
        __terrane_traced_err(
            operation(value.clone()),
            1 /* terrane-site: case.trn:10:10-10:26 */,
        )?,
    );
}
fn main() {
    let operation: std::sync::Arc<
        dyn Fn(terrane_int_support::Int) -> Result<String, TerraneError> + Send + Sync,
    > = std::sync::Arc::new(render);
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = operation; "throws"
        .to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = operation; "coercion-error"
        .to_owned() })
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&{ let _ = operation; "coercion-error"
        .to_owned() })
    );
    let __terrane_completion_0: TerraneCompletion<()> = (|| {
        let __terrane_try_0: TerraneCompletion<()> = (|| {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&__terrane_traced_completion!(apply(operation
                .clone(), terrane_int_support::Int::from(- 1_i128)),
                2 /* terrane-site: case.trn:18:13-18:33 */))
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
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_traced(apply(operation
        .clone(), terrane_int_support::Int::from(4_i128)), 3 /* terrane-site: case.trn:21:11-21:30 */))
    );
}
