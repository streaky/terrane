// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: select-binding-scope
#[allow(dead_code)]
#[derive(Clone)]
enum TerraneUnionF0S155 {
    Arm0(terrane_int_support::Int),
    Arm1(String),
}
impl terrane_scalar_support::ScalarDisplay for TerraneUnionF0S155 {
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
async fn first() -> terrane_int_support::Int {
    return terrane_int_support::Int::from(1_i128);
}
async fn second() -> String {
    return String::from("two");
}
fn main() {
    __terrane_run(async move {
        let selected: TerraneUnionF0S155;
        let mut __terrane_select_cursor_139 = 0usize;
        {
            let mut __terrane_select_guard_139 = __terrane_finally_guard();
            let __terrane_select_control_139_0 = __terrane_select_control();
            let mut __terrane_select_future_139_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_139_0.clone(),
                first())
            );
            let mut __terrane_select_result_139_0 = None;
            let __terrane_select_control_139_1 = __terrane_select_control();
            let mut __terrane_select_future_139_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_139_1.clone(),
                second())
            );
            let mut __terrane_select_result_139_1 = None;
            let __terrane_select_winner_139 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_139
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_139_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_139_0 = Some(
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
                                    __terrane_select_future_139_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_139_1 = Some(
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
            if __terrane_select_winner_139 == usize::MAX {
                __terrane_select_control_139_1.request_cancel();
                __terrane_select_control_139_0.request_cancel();
                let _ = __terrane_select_future_139_1.as_mut().await;
                let _ = __terrane_select_future_139_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_139.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_139).await;
            }
            __terrane_select_cursor_139 = (__terrane_select_winner_139 + 1usize)
                % 2usize;
            match __terrane_select_winner_139 {
                0 => {
                    __terrane_select_control_139_1.request_cancel();
                    let _ = __terrane_select_future_139_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_139_0.request_cancel();
                    let _ = __terrane_select_future_139_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_139.finish();
            match __terrane_select_winner_139 {
                0 => {
                    selected = TerraneUnionF0S155::Arm0(
                        __terrane_select_result_139_0
                            .take()
                            .expect("selected case owns its ready result"),
                    );
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&match &selected {
                        TerraneUnionF0S155::Arm0(value) => value, _ =>
                        unreachable!("flow-proven storage refinement") })
                    );
                }
                1 => {
                    selected = TerraneUnionF0S155::Arm1(
                        __terrane_select_result_139_1
                            .take()
                            .expect("selected case owns its ready result"),
                    );
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&match &selected {
                        TerraneUnionF0S155::Arm1(value) => value, _ =>
                        unreachable!("flow-proven storage refinement") })
                    );
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        println!("{}", terrane_scalar_support::scalar_text(&selected));
    });
}
