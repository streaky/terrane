// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support
// Source: case.trn
// Namespace: async-finally-after-await
async fn step() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(7_i128);
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_finally_guard_0 = __terrane_finally_guard();
        let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
                &__terrane_finally_guard_0,
                async {
                    let __terrane_try_0: TerraneCompletion<()> = async {
                        let value: terrane_int_support::Int = __terrane_await(step())
                            .await;
                        println!("{}", terrane_scalar_support::scalar_text(&value));
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
                                return TerraneCompletion::Error(__terrane_error_0);
                            }
                        }
                    }
                    TerraneCompletion::Normal
                },
            )
            .await;
        let __terrane_cancelled_0 = __terrane_maybe_completion_0.is_none();
        let mut __terrane_completion_0 = __terrane_maybe_completion_0
            .unwrap_or(TerraneCompletion::Normal);
        let __terrane_finally_0: TerraneCompletion<()> = (|| {
            println!(
                "{}", terrane_scalar_support::scalar_text(&String::from("cleanup"))
            );
            TerraneCompletion::Normal
        })();
        match __terrane_finally_0 {
            TerraneCompletion::Normal => {}
            replacement => __terrane_completion_0 = replacement,
        }
        if __terrane_cancelled_0
            && matches!(&__terrane_completion_0, TerraneCompletion::Normal)
        {
            __terrane_finish_cancelled_finally(__terrane_finally_guard_0).await;
        }
        __terrane_finally_guard_0.finish();
        match __terrane_completion_0 {
            TerraneCompletion::Normal => {}
            TerraneCompletion::Return(value) => return value,
            TerraneCompletion::Error(error) => __terrane_uncaught(error),
            TerraneCompletion::Break | TerraneCompletion::Continue => {
                __terrane_generated_defect("loop control escaped a non-loop try")
            }
        }
    });
}
