// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: mutable-async-overlap-serialization
#[derive(Clone)]
pub struct GatedAccumulator {
    pub total: terrane_int_support::Int,
}
impl GatedAccumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub async fn add(
        &mut self,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let released: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        sent = __terrane_await(Box::pin(started.send(delta.clone()))).await;
        if !sent.accepted {
            return terrane_int_support::Int::from(-1_i128);
        }
        released = __terrane_await(Box::pin(gate.receive())).await;
        if !released.available {
            return terrane_int_support::Int::from(-2_i128);
        }
        {
            let __terrane_value = self.total.clone() + delta.clone();
            self.total = __terrane_value;
        }
        return self.total.clone();
    }
    pub async fn __terrane_shared_async_add(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let released: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        sent = __terrane_await(Box::pin(started.send(delta.clone()))).await;
        if !sent.accepted {
            return terrane_int_support::Int::from(-1_i128);
        }
        released = __terrane_await(Box::pin(gate.receive())).await;
        if !released.available {
            return terrane_int_support::Int::from(-2_i128);
        }
        {
            let __terrane_value = {
                let __terrane_snapshot = __terrane_shared_handle
                    .lock()
                    .expect("shared async receiver lock poisoned")
                    .total
                    .clone();
                __terrane_snapshot
            }
                .clone() + delta.clone();
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total = __terrane_value;
        }
        return {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total
                .clone();
            __terrane_snapshot
        }
            .clone();
    }
}
fn main() {
    __terrane_run(async move {
        let gate_a: TerraneChannelPair<terrane_int_support::Int>;
        let gate_b: TerraneChannelPair<terrane_int_support::Int>;
        let gate_c: TerraneChannelPair<terrane_int_support::Int>;
        let started_a: TerraneChannelPair<terrane_int_support::Int>;
        let started_b: TerraneChannelPair<terrane_int_support::Int>;
        let started_c: TerraneChannelPair<terrane_int_support::Int>;
        let value: GatedAccumulator;
        let bound: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let scope: TerraneTaskScope;
        let first: TerraneScopedTask<terrane_int_support::Int>;
        let first_start: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let second: TerraneScopedTask<terrane_int_support::Int>;
        let first_release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let first_outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let second_start: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let second_release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let second_outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let third_release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let third: terrane_int_support::Int;
        let first_value: Option<terrane_int_support::Int>;
        let second_value: Option<terrane_int_support::Int>;
        let counter: terrane_int_support::Int;
        let guarded_capture: i8;
        let step: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let closure_gate_a: TerraneChannelPair<terrane_int_support::Int>;
        let closure_gate_b: TerraneChannelPair<terrane_int_support::Int>;
        let closure_gate_c: TerraneChannelPair<terrane_int_support::Int>;
        let closure_started_a: TerraneChannelPair<terrane_int_support::Int>;
        let closure_started_b: TerraneChannelPair<terrane_int_support::Int>;
        let closure_started_c: TerraneChannelPair<terrane_int_support::Int>;
        let closure_scope: TerraneTaskScope;
        let closure_first: TerraneScopedTask<terrane_int_support::Int>;
        let closure_first_start: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let closure_second: TerraneScopedTask<terrane_int_support::Int>;
        let closure_first_release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let closure_first_outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let closure_second_start: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let closure_second_release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let closure_second_outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let closure_third_release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let closure_third: terrane_int_support::Int;
        let closure_first_value: Option<terrane_int_support::Int>;
        let closure_second_value: Option<terrane_int_support::Int>;
        gate_a = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate_b = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate_c = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_a = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_b = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_c = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        value = GatedAccumulator::terrane_construct();
        bound = {
            let receiver = TerraneAsyncMutableState::new(value.clone());
            TerraneMutableCallable::new(move |
                (
                    argument_0,
                    argument_1,
                    argument_2,
                ): (
                    terrane_int_support::Int,
                    TerraneChannelSender<terrane_int_support::Int>,
                    TerraneChannelReceiver<terrane_int_support::Int>,
                ),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                let receiver = receiver.share();
                Box::pin(async move {
                    receiver
                        .with_receiver(move |receiver| Box::pin(async move {
                            receiver.add(argument_0, argument_1, argument_2).await
                        }))
                        .await
                })
            })
        };
        scope = TerraneTaskScope::new(None);
        first = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = bound
                .call((
                    terrane_int_support::Int::from(1_i128),
                    started_a.sender,
                    gate_a.receiver,
                ));
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        first_start = __terrane_await(started_a.receiver.receive()).await;
        if !first_start.available {
            return ();
        }
        second = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = bound
                .call((
                    terrane_int_support::Int::from(10_i128),
                    started_b.sender,
                    gate_b.receiver,
                ));
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        first_release = __terrane_await(
                gate_a.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !first_release.accepted {
            return ();
        }
        first_outcome = __terrane_await(scope.join(first)).await;
        second_start = __terrane_await(started_b.receiver.receive()).await;
        if !second_start.available {
            return ();
        }
        second_release = __terrane_await(
                gate_b.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !second_release.accepted {
            return ();
        }
        second_outcome = __terrane_await(scope.join(second)).await;
        third_release = __terrane_await(
                gate_c.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !third_release.accepted {
            return ();
        }
        third = __terrane_await(
                bound
                    .call((
                        terrane_int_support::Int::from(100_i128),
                        started_c.sender,
                        gate_c.receiver,
                    )),
            )
            .await;
        first_value = first_outcome.value.clone();
        second_value = second_outcome.value.clone();
        if first_value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &first_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        if second_value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &second_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        println!("{}", terrane_scalar_support::scalar_text(&third));
        counter = terrane_int_support::Int::from(0_i128);
        guarded_capture = 41;
        step = {
            let counter = TerraneAsyncMutableState::new(counter.clone());
            let guarded_capture = TerraneAsyncMutableState::new(guarded_capture.clone());
            let __terrane_invocation = TerraneAsyncInvocationGate::new();
            TerraneMutableCallable::new(move |
                (
                    delta,
                    started,
                    gate,
                ): (
                    terrane_int_support::Int,
                    TerraneChannelSender<terrane_int_support::Int>,
                    TerraneChannelReceiver<terrane_int_support::Int>,
                ),
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let counter = counter.share();
                let guarded_capture = guarded_capture.share();
                let __terrane_invocation = __terrane_invocation.share();
                Box::pin(async move {
                    let _invocation = __terrane_invocation.enter().await;
                    let seen: terrane_int_support::Int;
                    let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
                    let released: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
                    if guarded_capture.snapshot().rem_euclid(2) == 0 {
                        {
                            let callable_capture_value = __terrane_raised(
                                terrane_int_support::fixed_division(
                                    guarded_capture.snapshot(),
                                    2,
                                ),
                                0 /* terrane-site: case.trn:66:25-66:44 */,
                            );
                            guarded_capture.replace(callable_capture_value);
                        }
                    } else {
                        {
                            let callable_capture_value = __terrane_raised(
                                terrane_int_support::fixed_addition(
                                    __terrane_raised(
                                        terrane_int_support::fixed_multiplication(
                                            3,
                                            guarded_capture.snapshot(),
                                        ),
                                        1 /* terrane-site: case.trn:68:25-68:44 */,
                                    ),
                                    1,
                                ),
                                2 /* terrane-site: case.trn:68:25-68:48 */,
                            );
                            guarded_capture.replace(callable_capture_value);
                        }
                    }
                    seen = counter.snapshot();
                    sent = __terrane_await(Box::pin(started.send(delta.clone()))).await;
                    if !sent.accepted {
                        return terrane_int_support::Int::from(-1_i128);
                    }
                    released = __terrane_await(Box::pin(gate.receive())).await;
                    if !released.available {
                        return terrane_int_support::Int::from(-2_i128);
                    }
                    {
                        let callable_capture_value = seen.clone() + delta.clone();
                        counter.replace(callable_capture_value);
                    }
                    return counter.snapshot();
                })
            })
        };
        closure_gate_a = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_gate_b = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_gate_c = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_started_a = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_started_b = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_started_c = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closure_scope = TerraneTaskScope::new(None);
        closure_first = {
            let __terrane_scope = closure_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = step
                .call((
                    terrane_int_support::Int::from(1_i128),
                    closure_started_a.sender,
                    closure_gate_a.receiver,
                ));
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        closure_first_start = __terrane_await(closure_started_a.receiver.receive())
            .await;
        if !closure_first_start.available {
            return ();
        }
        closure_second = {
            let __terrane_scope = closure_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = step
                .call((
                    terrane_int_support::Int::from(10_i128),
                    closure_started_b.sender,
                    closure_gate_b.receiver,
                ));
            TerraneScopedTask::spawn(async move {
                match __terrane_cancellable(
                        __terrane_spawned_task,
                        __terrane_cancel,
                        __terrane_deadline,
                    )
                    .await
                {
                    Some(value) => TerraneTaskResult::Completed(value),
                    None => TerraneTaskResult::Cancelled,
                }
            })
        };
        closure_first_release = __terrane_await(
                closure_gate_a.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !closure_first_release.accepted {
            return ();
        }
        closure_first_outcome = __terrane_await(closure_scope.join(closure_first)).await;
        closure_second_start = __terrane_await(closure_started_b.receiver.receive())
            .await;
        if !closure_second_start.available {
            return ();
        }
        closure_second_release = __terrane_await(
                closure_gate_b.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !closure_second_release.accepted {
            return ();
        }
        closure_second_outcome = __terrane_await(closure_scope.join(closure_second))
            .await;
        closure_third_release = __terrane_await(
                closure_gate_c.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        if !closure_third_release.accepted {
            return ();
        }
        closure_third = __terrane_await(
                step
                    .call((
                        terrane_int_support::Int::from(100_i128),
                        closure_started_c.sender,
                        closure_gate_c.receiver,
                    )),
            )
            .await;
        closure_first_value = closure_first_outcome.value.clone();
        closure_second_value = closure_second_outcome.value.clone();
        if closure_first_value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &closure_first_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        if closure_second_value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &closure_second_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        println!("{}", terrane_scalar_support::scalar_text(&closure_third));
    });
}
