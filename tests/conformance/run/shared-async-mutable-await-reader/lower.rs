// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-async-mutable-await-reader
#[derive(Clone)]
pub struct Accumulator {
    pub total: terrane_int_support::Int,
}
impl Accumulator {
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
        sent = __terrane_await(
                Box::pin(started.send(terrane_int_support::Int::from(1_i128))),
            )
            .await;
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
        sent = __terrane_await(
                Box::pin(started.send(terrane_int_support::Int::from(1_i128))),
            )
            .await;
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
            } + delta.clone();
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
        };
    }
}
async fn inspect(
    value: std::sync::Arc<TerraneSharedCell<Accumulator>>,
    started: TerraneChannelReceiver<terrane_int_support::Int>,
    observed: TerraneChannelSender<terrane_int_support::Int>,
    gate: TerraneChannelSender<terrane_int_support::Int>,
    done: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let signaled: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
    let sent_before: TerraneChannelSendOutcome<terrane_int_support::Int>;
    let released: TerraneChannelSendOutcome<terrane_int_support::Int>;
    let finished: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
    let sent_after: TerraneChannelSendOutcome<terrane_int_support::Int>;
    signaled = __terrane_await(Box::pin(started.receive())).await;
    if !signaled.available {
        return ();
    }
    sent_before = __terrane_await(
            Box::pin(
                observed
                    .send({
                        let __terrane_snapshot = value
                            .lock()
                            .expect("shared reference lock poisoned")
                            .total
                            .clone();
                        __terrane_snapshot
                    }),
            ),
        )
        .await;
    released = __terrane_await(
            Box::pin(gate.send(terrane_int_support::Int::from(1_i128))),
        )
        .await;
    if !sent_before.accepted || !released.accepted {
        return ();
    }
    finished = __terrane_await(Box::pin(done.receive())).await;
    if finished.available {
        sent_after = __terrane_await(
                Box::pin(
                    observed
                        .send({
                            let __terrane_snapshot = value
                                .lock()
                                .expect("shared reference lock poisoned")
                                .total
                                .clone();
                            __terrane_snapshot
                        }),
                ),
            )
            .await;
        if !sent_after.accepted {
            return ();
        }
    }
}
fn main() {
    __terrane_run(async move {
        let started: TerraneChannelPair<terrane_int_support::Int>;
        let gate: TerraneChannelPair<terrane_int_support::Int>;
        let observed: TerraneChannelPair<terrane_int_support::Int>;
        let done: TerraneChannelPair<terrane_int_support::Int>;
        let observations: TerraneChannelReceiver<terrane_int_support::Int>;
        let value: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let first: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let second: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let observer: std::sync::Weak<TerraneSharedCell<Accumulator>>;
        let scope: TerraneTaskScope;
        let reader: TerraneScopedTask<()>;
        let result: terrane_int_support::Int;
        let finished: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let before: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let after: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let reader_outcome: TerraneTaskOutcome<()>;
        started = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        observed = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(2_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        done = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        observations = observed.receiver;
        value = std::sync::Arc::new(
            TerraneSharedCell::new(Accumulator::terrane_construct()),
        );
        first = value.clone();
        second = first.clone();
        observer = std::sync::Arc::downgrade(&value.clone());
        scope = TerraneTaskScope::new(None);
        reader = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = inspect(
                second.clone(),
                started.receiver,
                observed.sender,
                gate.sender,
                done.receiver,
            );
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
        result = __terrane_await({
                let __terrane_shared_call_handle = observer
                    .upgrade()
                    .expect("reference expired");
                let __terrane_shared_argument_0 = terrane_int_support::Int::from(3_i128);
                let __terrane_shared_argument_1 = started.sender;
                let __terrane_shared_argument_2 = gate.receiver;
                let __terrane_shared_argument_3 = __terrane_shared_argument_0;
                let __terrane_shared_argument_4 = __terrane_shared_argument_1;
                let __terrane_shared_argument_5 = __terrane_shared_argument_2;
                Accumulator::__terrane_shared_async_add(
                    __terrane_shared_call_handle,
                    __terrane_shared_argument_3,
                    __terrane_shared_argument_4,
                    __terrane_shared_argument_5,
                )
            })
            .await;
        finished = __terrane_await(
                done.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        before = __terrane_await(Box::pin(observations.receive())).await;
        after = __terrane_await(Box::pin(observations.receive())).await;
        if finished.accepted && before.available && after.available {
            println!(
                "{}{}", { let mut __terrane_text = String::new(); match &before.value {
                Some(__terrane_present) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present,
                &mut __terrane_text); }, None => __terrane_text.push_str("none"), }
                __terrane_text }, { let mut __terrane_text = String::new(); match &after
                .value { Some(__terrane_present) => {
                terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present,
                &mut __terrane_text); }, None => __terrane_text.push_str("none"), }
                __terrane_text }
            );
        }
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&result),
            terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = first.lock()
            .expect("shared reference lock poisoned").total.clone(); __terrane_snapshot
            }), terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = second
            .lock().expect("shared reference lock poisoned").total.clone();
            __terrane_snapshot })
        );
        reader_outcome = __terrane_await(scope.join(reader)).await;
        println!("{}", terrane_scalar_support::scalar_text(&reader_outcome.completed));
    });
}
