// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-terminal-cursor
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S291 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S291 {
    fn write_scalar(&self, output: &mut String) {
        match self {
            Self::Arm0(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
            Self::Arm1(value) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(value, output)
            }
        }
    }
}
async fn fail() -> Result<terrane_int_support::Int, TerraneError> {
    return Err(
        TerraneError::raised(
            TerraneErrorKind::CoercionError,
            0 /* terrane-site: case.trn:5:3-5:23 */,
        ),
    );
}
async fn ready() -> String {
    return String::from("ready");
}
fn main() {
    __terrane_run(async move {
        let mut iteration: terrane_int_support::Int;
        let mut value: Option<TerraneUnionF0S291> = None;
        let mut __terrane_select_cursor_271 = 0usize;
        iteration = terrane_int_support::Int::from(0_i128);
        while iteration < terrane_int_support::Int::from(2_i128) {
            let __terrane_completion_0: TerraneCompletion<()> = async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    {
                        let mut __terrane_select_guard_271 = __terrane_finally_guard();
                        let mut __terrane_select_cleanup_error_271: Option<
                            TerraneError,
                        > = None;
                        let __terrane_select_control_271_0 = __terrane_select_control();
                        let mut __terrane_select_future_271_0 = std::pin::pin!(
                            __terrane_select_operation(__terrane_select_control_271_0
                            .clone(), fail())
                        );
                        let mut __terrane_select_result_271_0 = None;
                        let __terrane_select_control_271_1 = __terrane_select_control();
                        let mut __terrane_select_future_271_1 = std::pin::pin!(
                            __terrane_select_operation(__terrane_select_control_271_1
                            .clone(), ready())
                        );
                        let mut __terrane_select_result_271_1 = None;
                        let __terrane_select_winner_271 = std::future::poll_fn(|
                                __terrane_select_context|
                            {
                                if __terrane_cancellation_is_requested() {
                                    return std::task::Poll::Ready(usize::MAX);
                                }
                                for __terrane_select_offset in 0..2usize {
                                    let __terrane_select_candidate = (__terrane_select_cursor_271
                                        + __terrane_select_offset) % 2usize;
                                    match __terrane_select_candidate {
                                        0 => {
                                            match Future::poll(
                                                __terrane_select_future_271_0.as_mut(),
                                                __terrane_select_context,
                                            ) {
                                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                    __terrane_select_result_271_0 = Some(
                                                        __terrane_select_value,
                                                    );
                                                    return std::task::Poll::Ready(0usize);
                                                }
                                                std::task::Poll::Ready(None) => {
                                                    unreachable!(
                                                        "case cancellation starts only after winner selection"
                                                    )
                                                }
                                                std::task::Poll::Pending => {}
                                            }
                                        }
                                        1 => {
                                            match Future::poll(
                                                __terrane_select_future_271_1.as_mut(),
                                                __terrane_select_context,
                                            ) {
                                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                                    __terrane_select_result_271_1 = Some(
                                                        __terrane_select_value,
                                                    );
                                                    return std::task::Poll::Ready(1usize);
                                                }
                                                std::task::Poll::Ready(None) => {
                                                    unreachable!(
                                                        "case cancellation starts only after winner selection"
                                                    )
                                                }
                                                std::task::Poll::Pending => {}
                                            }
                                        }
                                        _ => {
                                            unreachable!("select candidate is within the case count")
                                        }
                                    }
                                }
                                std::task::Poll::Pending
                            })
                            .await;
                        if __terrane_select_winner_271 == usize::MAX {
                            __terrane_select_control_271_1.request_cancel();
                            __terrane_select_control_271_0.request_cancel();
                            let _ = __terrane_select_future_271_1.as_mut().await;
                            if let Some(Err(__terrane_select_error)) = __terrane_select_future_271_0
                                .as_mut()
                                .await
                            {
                                __terrane_select_cleanup_error_271 = Some(
                                    __terrane_trace_error(
                                        __terrane_select_error,
                                        2 /* terrane-site: case.trn:15:28-15:35 */,
                                    ),
                                );
                            }
                            __terrane_wait_projected_cleanups().await;
                            __terrane_select_guard_271.finish();
                            if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_271
                                .take()
                            {
                                return TerraneCompletion::Error(
                                    __terrane_select_cleanup_error,
                                );
                            }
                            __terrane_finish_cancelled_select(__terrane_select_guard_271)
                                .await;
                        }
                        __terrane_select_cursor_271 = (__terrane_select_winner_271
                            + 1usize) % 2usize;
                        match __terrane_select_winner_271 {
                            0 => {
                                __terrane_select_control_271_1.request_cancel();
                                let _ = __terrane_select_future_271_1.as_mut().await;
                            }
                            1 => {
                                __terrane_select_control_271_0.request_cancel();
                                if let Some(Err(__terrane_select_error)) = __terrane_select_future_271_0
                                    .as_mut()
                                    .await
                                {
                                    __terrane_select_cleanup_error_271 = Some(
                                        __terrane_trace_error(
                                            __terrane_select_error,
                                            2 /* terrane-site: case.trn:15:28-15:35 */,
                                        ),
                                    );
                                }
                            }
                            _ => unreachable!("selected winner is within the case count"),
                        }
                        __terrane_wait_projected_cleanups().await;
                        __terrane_select_guard_271.finish();
                        if let Some(__terrane_select_cleanup_error) = __terrane_select_cleanup_error_271
                            .take()
                        {
                            return TerraneCompletion::Error(
                                __terrane_select_cleanup_error,
                            );
                        }
                        match __terrane_select_winner_271 {
                            0 => {
                                let _ = value
                                    .insert(
                                        TerraneUnionF0S291::Arm0(
                                            __terrane_traced_completion!(
                                                __terrane_select_result_271_0.take()
                                                .expect("selected case owns its ready result"),
                                                2 /* terrane-site: case.trn:15:28-15:35 */
                                            ),
                                        ),
                                    );
                                println!(
                                    "{}", terrane_scalar_support::scalar_text(&match value
                                    .as_ref().expect("flow-proven available binding") {
                                    TerraneUnionF0S291::Arm0(value) => value, _ =>
                                    unreachable!("flow-proven storage refinement") })
                                );
                            }
                            1 => {
                                let _ = value
                                    .insert(
                                        TerraneUnionF0S291::Arm1(
                                            __terrane_select_result_271_1
                                                .take()
                                                .expect("selected case owns its ready result"),
                                        ),
                                    );
                                println!(
                                    "{}", terrane_scalar_support::scalar_text(&match value
                                    .as_ref().expect("flow-proven available binding") {
                                    TerraneUnionF0S291::Arm1(value) => value, _ =>
                                    unreachable!("flow-proven storage refinement") })
                                );
                            }
                            _ => unreachable!("selected winner is within the case count"),
                        }
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
                            && __terrane_error_0.kind == TerraneErrorKind::CoercionError
                        {
                            __terrane_handled_0 = true;
                            println!("{}", String::from("error"));
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
                TerraneCompletion::Break => break,
                TerraneCompletion::Continue => continue,
            }
            iteration = iteration.clone() + terrane_int_support::Int::from(1_i128);
        }
    });
}
