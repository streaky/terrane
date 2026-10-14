// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: moved-throwing-await
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
async fn safe() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(2_i128);
}
fn main() {
    __terrane_run(async move {
        let pending_int;
        let moved_int;
        let mut value: Option<terrane_int_support::Int> = None;
        let pending_none;
        let moved_none;
        let mut pending: std::pin::Pin<
            Box<
                dyn Future<
                    Output = Result<terrane_int_support::Int, TerraneError>,
                > + Send + '_,
            >,
        >;
        let moved_safe: std::pin::Pin<
            Box<
                dyn Future<
                    Output = Result<terrane_int_support::Int, TerraneError>,
                > + Send + '_,
            >,
        >;
        let moved_carrier: std::pin::Pin<
            Box<
                dyn Future<
                    Output = Result<terrane_int_support::Int, TerraneError>,
                > + Send + '_,
            >,
        >;
        pending_int = risky_int();
        moved_int = pending_int;
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                let _ = value
                    .insert(
                        __terrane_traced_completion!(
                            __terrane_await(moved_int). await, 3 /* terrane-site: case.trn:17:27-17:36 */
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
        moved_none = pending_none;
        let __terrane_completion_1: TerraneCompletion<()> = async {
            let __terrane_try_1: TerraneCompletion<()> = async {
                {
                    let _: () = __terrane_traced_completion!(
                        __terrane_await(moved_none). await, 5 /* terrane-site: case.trn:24:19-24:29 */
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
        pending = {
            let __terrane_task_future = safe();
            Box::pin(async move { Ok(__terrane_task_future.await) })
        };
        moved_safe = pending;
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await(moved_safe)
            . await, 6 /* terrane-site: case.trn:30:18-30:28 */))
        );
        pending = Box::pin(risky_int());
        moved_carrier = pending;
        let __terrane_completion_2: TerraneCompletion<()> = async {
            let __terrane_try_2: TerraneCompletion<()> = async {
                {
                    let _: terrane_int_support::Int = __terrane_traced_completion!(
                        __terrane_await(moved_carrier). await, 8 /* terrane-site: case.trn:34:19-34:32 */
                    );
                }
                TerraneCompletion::Normal
            }
                .await;
            match __terrane_try_2 {
                TerraneCompletion::Return(value) => {
                    return TerraneCompletion::Return(value);
                }
                TerraneCompletion::Break => return TerraneCompletion::Break,
                TerraneCompletion::Continue => return TerraneCompletion::Continue,
                TerraneCompletion::Normal => {}
                TerraneCompletion::Error(__terrane_error_2) => {
                    let mut __terrane_handled_2 = false;
                    if !__terrane_handled_2 {
                        __terrane_handled_2 = true;
                        println!("{}", String::from("carrier handled"));
                    }
                    if !__terrane_handled_2 {
                        return TerraneCompletion::Error(__terrane_error_2);
                    }
                }
            }
            TerraneCompletion::Normal
        }
            .await;
        match __terrane_completion_2 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
