// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let value: String;
        let successful: u32;
        let mut failure: Option<TerraneError> = None;
        value = __terrane_traced(
            __terrane_await({
                    let __terrane_future = {
                        let __terrane_call = terrane_into_future_witness::ready_value(
                            String::from("generic-output"),
                        );
                        async move {
                            match crate::__terrane_dependency_await_unwind(
                                    std::future::IntoFuture::into_future(__terrane_call),
                                )
                                .await
                            {
                                Ok(value) => Ok(value),
                                Err(payload) => {
                                    Err(
                                        crate::__terrane_dependency_panic(
                                            payload,
                                            "terrane-into-future-witness",
                                            "terrane_into_future_witness::ready_value",
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    async move {
                        __terrane_raised_err(
                            __terrane_future.await,
                            0 /* terrane-site: src/main.trn:7:24-7:53 */,
                        )
                    }
                })
                .await,
            0 /* terrane-site: src/main.trn:7:24-7:53 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&value));
        successful = __terrane_raised(
            terrane_int_support::coerce::<
                u32,
            >(
                &__terrane_traced(
                    __terrane_await({
                            let __terrane_future = ready_result(true);
                            async move {
                                __terrane_raised_err(
                                    __terrane_future.await,
                                    1 /* terrane-site: src/main.trn:9:29-9:47 */,
                                )
                            }
                        })
                        .await,
                    1 /* terrane-site: src/main.trn:9:29-9:47 */,
                ),
            ),
            2 /* terrane-site: src/main.trn:9:23-9:47 */,
        );
        println!("{}", terrane_scalar_support::scalar_text(&successful));
        let __terrane_completion_0: TerraneCompletion<()> = async {
            let __terrane_try_0: TerraneCompletion<()> = async {
                {
                    let _: u32 = __terrane_raised_completion!(
                        terrane_int_support::coerce:: < u32 >
                        (&__terrane_traced_completion!(__terrane_await({ let
                        __terrane_future = ready_result(false); async move {
                        __terrane_raised_err(__terrane_future. await,
                        3 /* terrane-site: src/main.trn:12:22-12:41 */) } }). await,
                        3 /* terrane-site: src/main.trn:12:22-12:41 */)),
                        4 /* terrane-site: src/main.trn:12:16-12:41 */
                    );
                }
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
                    if !__terrane_handled_0
                        && __terrane_error_0.kind
                            == TerraneErrorKind::Custom(DescriptorId(0))
                    {
                        __terrane_handled_0 = true;
                        let _ = failure.insert(__terrane_error_0.clone());
                        println!(
                            "{}", terrane_scalar_support::scalar_text(&failure.as_ref()
                            .expect("flow-proven available binding").clone().message()
                            .to_owned())
                        );
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
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(construction_count(),
            5 /* terrane-site: src/main.trn:15:11-15:30 */))
        );
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(conversion_count(),
            6 /* terrane-site: src/main.trn:16:11-16:28 */))
        );
    });
}
