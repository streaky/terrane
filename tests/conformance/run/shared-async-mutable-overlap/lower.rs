// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-async-mutable-overlap
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
fn main() {
    __terrane_run(async move {
        let started_left: TerraneChannelPair<terrane_int_support::Int>;
        let started_right: TerraneChannelPair<terrane_int_support::Int>;
        let gate_left: TerraneChannelPair<terrane_int_support::Int>;
        let gate_right: TerraneChannelPair<terrane_int_support::Int>;
        let value: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let first: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let second: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let scope: TerraneTaskScope;
        let left_call;
        let right_call;
        let left: TerraneScopedTask<terrane_int_support::Int>;
        let right: TerraneScopedTask<terrane_int_support::Int>;
        let one: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let two: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let first_release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let left_result: TerraneTaskOutcome<terrane_int_support::Int>;
        let second_release: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let right_result: TerraneTaskOutcome<terrane_int_support::Int>;
        started_left = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started_right = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate_left = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gate_right = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        value = std::sync::Arc::new(
            TerraneSharedCell::new(Accumulator::terrane_construct()),
        );
        first = value.clone();
        second = first.clone();
        scope = TerraneTaskScope::new(None);
        left_call = {
            let __terrane_shared_call_handle = first.clone();
            let __terrane_shared_argument_0 = terrane_int_support::Int::from(4_i128);
            let __terrane_shared_argument_1 = started_left.sender;
            let __terrane_shared_argument_2 = gate_left.receiver;
            let __terrane_shared_argument_3 = __terrane_shared_argument_0;
            let __terrane_shared_argument_4 = __terrane_shared_argument_1;
            let __terrane_shared_argument_5 = __terrane_shared_argument_2;
            Accumulator::__terrane_shared_async_add(
                __terrane_shared_call_handle,
                __terrane_shared_argument_3,
                __terrane_shared_argument_4,
                __terrane_shared_argument_5,
            )
        };
        right_call = {
            let __terrane_shared_call_handle = second.clone();
            let __terrane_shared_argument_0 = terrane_int_support::Int::from(7_i128);
            let __terrane_shared_argument_1 = started_right.sender;
            let __terrane_shared_argument_2 = gate_right.receiver;
            let __terrane_shared_argument_3 = __terrane_shared_argument_0;
            let __terrane_shared_argument_4 = __terrane_shared_argument_1;
            let __terrane_shared_argument_5 = __terrane_shared_argument_2;
            Accumulator::__terrane_shared_async_add(
                __terrane_shared_call_handle,
                __terrane_shared_argument_3,
                __terrane_shared_argument_4,
                __terrane_shared_argument_5,
            )
        };
        left = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = left_call;
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
        right = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = right_call;
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
        one = __terrane_await(started_left.receiver.receive()).await;
        two = __terrane_await(started_right.receiver.receive()).await;
        if !one.available || !two.available {
            println!("{}", String::from("start signal missing"));
        }
        first_release = __terrane_await(
                gate_left.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        left_result = __terrane_await(scope.join(left)).await;
        second_release = __terrane_await(
                gate_right.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        right_result = __terrane_await(scope.join(right)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&first_release.accepted),
            terrane_scalar_support::scalar_text(&second_release.accepted)
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&left_result.completed),
            terrane_scalar_support::scalar_text(&right_result.completed)
        );
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = first
            .lock().expect("shared reference lock poisoned").total.clone();
            __terrane_snapshot }), terrane_scalar_support::scalar_text(&{ let
            __terrane_snapshot = second.lock().expect("shared reference lock poisoned")
            .total.clone(); __terrane_snapshot })
        );
    });
}
