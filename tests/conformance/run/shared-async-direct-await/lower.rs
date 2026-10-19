// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: shared-async-direct-await
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
    pub async fn observe(
        &mut self,
        started: TerraneChannelReceiver<terrane_int_support::Int>,
        gate: TerraneChannelSender<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let notice: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let observed: terrane_int_support::Int;
        let released: TerraneChannelSendOutcome<terrane_int_support::Int>;
        notice = __terrane_await(Box::pin(started.receive())).await;
        if !notice.available {
            return terrane_int_support::Int::from(-1_i128);
        }
        observed = self.total.clone();
        released = __terrane_await(Box::pin(gate.send(observed.clone()))).await;
        if !released.accepted {
            return terrane_int_support::Int::from(-2_i128);
        }
        return observed.clone();
    }
    pub async fn __terrane_shared_async_observe(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        started: TerraneChannelReceiver<terrane_int_support::Int>,
        gate: TerraneChannelSender<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let notice: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let observed: terrane_int_support::Int;
        let released: TerraneChannelSendOutcome<terrane_int_support::Int>;
        notice = __terrane_await(Box::pin(started.receive())).await;
        if !notice.available {
            return terrane_int_support::Int::from(-1_i128);
        }
        observed = {
            let __terrane_snapshot = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total
                .clone();
            __terrane_snapshot
        };
        released = __terrane_await(Box::pin(gate.send(observed.clone()))).await;
        if !released.accepted {
            return terrane_int_support::Int::from(-2_i128);
        }
        return observed.clone();
    }
}
fn main() {
    __terrane_run(async move {
        let started: TerraneChannelPair<terrane_int_support::Int>;
        let gate: TerraneChannelPair<terrane_int_support::Int>;
        let value: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let owner: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let alias: std::sync::Arc<TerraneSharedCell<Accumulator>>;
        let scope: TerraneTaskScope;
        let observer: TerraneScopedTask<terrane_int_support::Int>;
        let result: terrane_int_support::Int;
        let observation: TerraneTaskOutcome<terrane_int_support::Int>;
        let observed_value: Option<terrane_int_support::Int>;
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
        value = std::sync::Arc::new(
            TerraneSharedCell::new(Accumulator::terrane_construct()),
        );
        owner = value.clone();
        alias = owner.clone();
        scope = TerraneTaskScope::new(None);
        observer = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = {
                let __terrane_shared_call_handle = alias.clone();
                let __terrane_shared_argument_0 = started.receiver;
                let __terrane_shared_argument_1 = gate.sender;
                let __terrane_shared_argument_2 = __terrane_shared_argument_0;
                let __terrane_shared_argument_3 = __terrane_shared_argument_1;
                Accumulator::__terrane_shared_async_observe(
                    __terrane_shared_call_handle,
                    __terrane_shared_argument_2,
                    __terrane_shared_argument_3,
                )
            };
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
                let __terrane_shared_call_handle = owner.clone();
                let __terrane_shared_argument_0 = terrane_int_support::Int::from(5_i128);
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
        observation = __terrane_await(scope.join(observer)).await;
        observed_value = observation.value.clone();
        if (&observed_value).is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&result),
                terrane_scalar_support::scalar_text(&match &observed_value { Some(value)
                => value, _ => unreachable!("flow-proven storage refinement") }),
                terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = owner
                .lock().expect("shared reference lock poisoned").total.clone();
                __terrane_snapshot })
            );
        }
    });
}
