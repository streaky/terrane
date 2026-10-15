// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: time-sleep-ticker
fn main() {
    __terrane_run(async move {
        let short: Duration;
        let long: Duration;
        let started: MonotonicInstant;
        let ignored: ();
        let slept: MonotonicInstant;
        let captured;
        let delay: ();
        let resumed: MonotonicInstant;
        let captured_result: ();
        let completed: MonotonicInstant;
        let mut timer: Ticker;
        let first: Tick;
        let latency: ();
        let second: Tick;
        let race_started: MonotonicInstant;
        let done: ();
        let race_ended: MonotonicInstant;
        let mut __terrane_select_cursor_1074 = 0usize;
        short = __terrane_traced(
            Duration::terrane_static_milliseconds(
                terrane_int_support::Int::from(2_i128),
            ),
            0 /* terrane-site: case.trn:6:13-6:38 */,
        );
        long = __terrane_traced(
            Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),
            1 /* terrane-site: case.trn:7:12-7:32 */,
        );
        started = Clock::terrane_static_monotonic();
        ignored = __terrane_await(
                terrane_time_sleep_after(short.total_nanoseconds.clone()),
            )
            .await;
        let _ = &ignored;
        slept = Clock::terrane_static_monotonic();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(__terrane_traced(started
            .duration_until(&slept), 2 /* terrane-site: case.trn:11:13-11:46 */)
            .total_nanoseconds.clone() >= short.total_nanoseconds.clone()))
        );
        captured = terrane_time_sleep_after(
            __terrane_traced(
                    Duration::terrane_static_milliseconds(
                        terrane_int_support::Int::from(50_i128),
                    ),
                    3 /* terrane-site: case.trn:13:31-13:57 */,
                )
                .total_nanoseconds
                .clone(),
        );
        delay = __terrane_await(
                terrane_time_sleep_after(
                    __terrane_traced(
                            Duration::terrane_static_milliseconds(
                                terrane_int_support::Int::from(150_i128),
                            ),
                            4 /* terrane-site: case.trn:14:34-14:61 */,
                        )
                        .total_nanoseconds
                        .clone(),
                ),
            )
            .await;
        let _ = &delay;
        resumed = Clock::terrane_static_monotonic();
        captured_result = __terrane_await(captured).await;
        let _ = &captured_result;
        completed = Clock::terrane_static_monotonic();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(__terrane_traced(resumed
            .duration_until(&completed), 5 /* terrane-site: case.trn:18:13-18:50 */)
            .total_nanoseconds.clone() < terrane_int_support::Int::from(25000000_i128)))
        );
        timer = __terrane_traced(
            Clock::terrane_static_interval(
                __terrane_traced(
                    Duration::terrane_static_milliseconds(
                        terrane_int_support::Int::from(1_i128),
                    ),
                    6 /* terrane-site: case.trn:20:38-20:63 */,
                ),
            ),
            7 /* terrane-site: case.trn:20:20-20:64 */,
        );
        first = __terrane_await((&mut timer).next()).await;
        latency = __terrane_await(
                terrane_time_sleep_after(
                    __terrane_traced(
                            Duration::terrane_static_milliseconds(
                                terrane_int_support::Int::from(5_i128),
                            ),
                            8 /* terrane-site: case.trn:22:36-22:61 */,
                        )
                        .total_nanoseconds
                        .clone(),
                ),
            )
            .await;
        let _ = &latency;
        second = __terrane_await((&mut timer).next()).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&(first.count.clone() >=
            terrane_int_support::Int::from(1_i128))),
            terrane_scalar_support::scalar_text(&(second.count.clone() >=
            terrane_int_support::Int::from(4_i128)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&(__terrane_traced(first.scheduled
            .duration_until(&second.scheduled), 9 /* terrane-site: case.trn:25:13-25:65 */).total_nanoseconds.clone() >=
            terrane_int_support::Int::from(4000000_i128)))
        );
        timer.destruct();
        race_started = Clock::terrane_static_monotonic();
        {
            let mut __terrane_select_guard_1074 = __terrane_finally_guard();
            let __terrane_select_control_1074_0 = __terrane_select_control();
            let mut __terrane_select_future_1074_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1074_0.clone(),
                terrane_time_sleep_after(short.total_nanoseconds.clone()))
            );
            let mut __terrane_select_result_1074_0 = None;
            let __terrane_select_control_1074_1 = __terrane_select_control();
            let mut __terrane_select_future_1074_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_1074_1.clone(),
                terrane_time_sleep_after(long.total_nanoseconds.clone()))
            );
            let mut __terrane_select_result_1074_1 = None;
            let __terrane_select_winner_1074 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_1074
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_1074_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1074_0 = Some(
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
                                    __terrane_select_future_1074_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_1074_1 = Some(
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
            if __terrane_select_winner_1074 == usize::MAX {
                __terrane_select_control_1074_1.request_cancel();
                __terrane_select_control_1074_0.request_cancel();
                let _ = __terrane_select_future_1074_1.as_mut().await;
                let _ = __terrane_select_future_1074_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_1074.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_1074).await;
            }
            __terrane_select_cursor_1074 = (__terrane_select_winner_1074 + 1usize)
                % 2usize;
            match __terrane_select_winner_1074 {
                0 => {
                    __terrane_select_control_1074_1.request_cancel();
                    let _ = __terrane_select_future_1074_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_1074_0.request_cancel();
                    let _ = __terrane_select_future_1074_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_1074.finish();
            match __terrane_select_winner_1074 {
                0 => {
                    done = __terrane_select_result_1074_0
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&(done == ())));
                }
                1 => {
                    let _ = __terrane_select_result_1074_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&false));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        race_ended = Clock::terrane_static_monotonic();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(__terrane_traced(race_started
            .duration_until(&race_ended), 10 /* terrane-site: case.trn:35:13-35:56 */)
            .total_nanoseconds.clone() < long.total_nanoseconds.clone()))
        );
    });
}
