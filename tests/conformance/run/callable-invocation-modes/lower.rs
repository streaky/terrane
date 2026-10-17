// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: callable-invocation-modes
fn double(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
}
async fn double_later(value: terrane_int_support::Int) -> terrane_int_support::Int {
    return value.clone() * terrane_int_support::Int::from(2_i128);
}
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
    pub fn add(&mut self, delta: terrane_int_support::Int) -> terrane_int_support::Int {
        {
            let __terrane_value = self.total.clone() + delta.clone();
            self.total = __terrane_value;
        }
        return self.total.clone();
    }
}
#[derive(Clone)]
pub struct Ticket {
    pub message: String,
}
impl Ticket {
    pub fn terrane_construct() -> Self {
        Self {
            message: String::from("redeemed"),
        }
    }
    pub fn redeem(self) -> String {
        return self.message.clone();
    }
}
#[derive(Clone)]
pub struct AsyncAccumulator {
    pub total: terrane_int_support::Int,
}
impl AsyncAccumulator {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub async fn add(
        &mut self,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
        {
            let __terrane_value = self.total.clone() + delta.clone();
            self.total = __terrane_value;
        }
        return self.total.clone();
    }
    pub async fn __terrane_shared_async_add(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
    ) -> terrane_int_support::Int {
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
        let counter: terrane_int_support::Int;
        let step: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let message: String;
        let finish: TerraneConsumingCallable<(), String>;
        let copy: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let operation: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let async_counter: terrane_int_support::Int;
        let async_step: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let async_copy: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let asynchronous: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let value: Accumulator;
        let bound: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let value_2: Ticket;
        let redemption: TerraneConsumingCallable<(), String>;
        let shared: std::sync::Arc<
            dyn Fn(terrane_int_support::Int) -> terrane_int_support::Int + Send + Sync,
        >;
        let adapted: TerraneMutableCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let async_value: AsyncAccumulator;
        let async_bound: TerraneMutableCallable<
            (terrane_int_support::Int,),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let one_shot: TerraneConsumingCallable<
            (terrane_int_support::Int,),
            terrane_int_support::Int,
        >;
        let declared_mutable: TerraneMutableCallable<(), terrane_int_support::Int>;
        let gate: TerraneChannelPair<terrane_int_support::Int>;
        let started: TerraneChannelPair<terrane_int_support::Int>;
        let gated_value: GatedAccumulator;
        let gated_bound: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<terrane_int_support::Int>;
        let observed: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let gated_copy: TerraneMutableCallable<
            (
                terrane_int_support::Int,
                TerraneChannelSender<terrane_int_support::Int>,
                TerraneChannelReceiver<terrane_int_support::Int>,
            ),
            std::pin::Pin<Box<dyn Future<Output = terrane_int_support::Int> + Send>>,
        >;
        let released: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let result: Option<terrane_int_support::Int>;
        counter = terrane_int_support::Int::from(0_i128);
        step = {
            let mut counter = counter.clone();
            TerraneMutableCallable::new(move |
                (delta,): (terrane_int_support::Int,),
            | -> terrane_int_support::Int {
                counter = counter.clone() + delta.clone();
                return counter.clone();
            })
        };
        message = String::from("finished");
        finish = {
            let message = message.clone();
            TerraneConsumingCallable::new(move |(): ()| -> String {
                return message;
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(2_i128),)))
        );
        copy = step.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = step; "mutable"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = step; "mutable"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&step
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&copy
            .call((terrane_int_support::Int::from(10_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = finish; "consuming"
            .to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = finish; "consuming"
            .to_owned() })
        );
        println!("{}", terrane_scalar_support::scalar_text(&finish.call(())));
        operation = TerraneMutableCallable::new(move |
            (argument_0,): (terrane_int_support::Int,)|
        double(argument_0));
        println!(
            "{}", terrane_scalar_support::scalar_text(&operation
            .call((terrane_int_support::Int::from(6_i128),)))
        );
        async_counter = terrane_int_support::Int::from(0_i128);
        async_step = {
            let async_counter = TerraneAsyncMutableState::new(async_counter.clone());
            let __terrane_invocation = TerraneAsyncInvocationGate::new();
            TerraneMutableCallable::new(move |
                (delta,): (terrane_int_support::Int,),
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let async_counter = async_counter.share();
                let __terrane_invocation = __terrane_invocation.share();
                Box::pin(async move {
                    let _invocation = __terrane_invocation.enter().await;
                    {
                        let callable_capture_value = async_counter.snapshot()
                            + delta.clone();
                        async_counter.replace(callable_capture_value);
                    }
                    return async_counter.snapshot();
                })
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(2_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(3_i128),))). await)
        );
        async_copy = async_step.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_step
            .call((terrane_int_support::Int::from(1_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_copy
            .call((terrane_int_support::Int::from(10_i128),))). await)
        );
        asynchronous = TerraneMutableCallable::new(move |
            (argument_0,): (terrane_int_support::Int,),
        | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
            Box::pin(double_later(argument_0))
        });
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(asynchronous
            .call((terrane_int_support::Int::from(7_i128),))). await)
        );
        value = Accumulator::terrane_construct();
        bound = {
            let mut receiver = value;
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            receiver.add(argument_0))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound
            .call((terrane_int_support::Int::from(5_i128),)))
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound
            .call((terrane_int_support::Int::from(7_i128),)))
        );
        value_2 = Ticket::terrane_construct();
        println!(
            "{}", terrane_scalar_support::scalar_text(&bound
            .call((terrane_int_support::Int::from(1_i128),)))
        );
        redemption = {
            let receiver = value_2.clone();
            TerraneConsumingCallable::new(move |(): ()| receiver.redeem())
        };
        println!("{}", terrane_scalar_support::scalar_text(&redemption.call(())));
        shared = {
            std::sync::Arc::new(move |
                value_3: terrane_int_support::Int,
            | -> terrane_int_support::Int {
                return value_3.clone() + terrane_int_support::Int::from(1_i128);
            })
        };
        adapted = {
            let callable = shared.clone();
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            callable(argument_0))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&adapted
            .call((terrane_int_support::Int::from(8_i128),)))
        );
        async_value = AsyncAccumulator::terrane_construct();
        async_bound = {
            let receiver = TerraneAsyncMutableState::new(async_value);
            TerraneMutableCallable::new(move |
                (argument_0,): (terrane_int_support::Int,),
            | -> std::pin::Pin<Box<dyn Future<Output = _> + Send>> {
                let receiver = receiver.share();
                Box::pin(async move {
                    receiver
                        .with_receiver(move |receiver| Box::pin(async move {
                            receiver.add(argument_0).await
                        }))
                        .await
                })
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_bound
            .call((terrane_int_support::Int::from(2_i128),))). await)
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_await(async_bound
            .call((terrane_int_support::Int::from(3_i128),))). await)
        );
        one_shot = {
            let callable = adapted.clone();
            TerraneConsumingCallable::new(move |
                (argument_0,): (terrane_int_support::Int,)|
            callable.call((argument_0,)))
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&one_shot
            .call((terrane_int_support::Int::from(10_i128),)))
        );
        declared_mutable = {
            TerraneMutableCallable::new(move |(): ()| -> terrane_int_support::Int {
                return terrane_int_support::Int::from(9_i128);
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = declared_mutable;
            "mutable".to_owned() })
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = declared_mutable;
            "shared".to_owned() })
        );
        println!("{}", terrane_scalar_support::scalar_text(&declared_mutable.call(())));
        gate = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        started = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        gated_value = GatedAccumulator::terrane_construct();
        gated_bound = {
            let receiver = TerraneAsyncMutableState::new(gated_value);
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
        child = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = gated_bound
                .call((
                    terrane_int_support::Int::from(4_i128),
                    started.sender,
                    gate.receiver,
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
        observed = __terrane_await(started.receiver.receive()).await;
        println!("{}", terrane_scalar_support::scalar_text(&observed.available));
        gated_copy = gated_bound.clone();
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let _ = gated_copy; "mutable"
            .to_owned() })
        );
        released = __terrane_await(
                gate.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&released.accepted));
        outcome = __terrane_await(scope.join(child)).await;
        result = outcome.value.clone();
        if result.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &result { Some(value) =>
                value, _ => unreachable!("flow-proven storage refinement") })
            );
        }
    });
}
