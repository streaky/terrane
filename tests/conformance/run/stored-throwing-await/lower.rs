// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: stored-throwing-await
async fn risky_int() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:5:5-5:25 */,
        ),
    );
}
async fn risky_none() -> Result<(), TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            1 /* terrane-site: case.trn:8:5-8:25 */,
        ),
    );
}
fn main() {
    __terrane_run(async move {
        let pending_int;
        let mut value: Option<terrane_int_support::Int> = None;
        let pending_none;
        pending_int = risky_int();
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = value
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await(pending_int). await, 3 /* terrane-site: case.trn:13:27-13:38 */
                        ),
                    );
                println!(
                    "{}", terrane_scalar_support::scalar_text(&value.as_ref()
                    .expect("flow-proven available binding"))
                );
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_0 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_0) => {
                    let mut __terrane_handled_0 = false;
                    if !__terrane_handled_0 {
                        __terrane_handled_0 = true;
                        println!("{}", String::from("int handled"));
                    }
                    if !__terrane_handled_0 {
                        return TerraneCompletion::Error(__terrane_error_0);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
        pending_none = risky_none();
        let __terrane_completion_1: TerraneCompletion<()> = async {
            let __terrane_try_1: TerraneCompletion<()> = async {
                {
                    let _: () = __terrane_traced_completion!(
                        __terrane_await(pending_none). await, 5 /* terrane-site: case.trn:19:19-19:31 */
                    );
                }
                println!("{}", String::from("unexpected success"));
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_1 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_1) => {
                    let mut __terrane_handled_1 = false;
                    if !__terrane_handled_1 {
                        __terrane_handled_1 = true;
                        println!("{}", String::from("none handled"));
                    }
                    if !__terrane_handled_1 {
                        return TerraneCompletion::Error(__terrane_error_1);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_1 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
