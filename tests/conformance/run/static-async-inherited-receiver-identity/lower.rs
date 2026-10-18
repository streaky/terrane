// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: static-async-inherited-receiver-identity
#[derive(Clone)]
pub struct BaseStorage {
    pub total: terrane_int_support::Int,
}
impl BaseStorage {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
        }
    }
    pub async fn accumulate(
        &mut self,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
        results: TerraneChannelSender<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let released: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let reported: TerraneChannelSendOutcome<terrane_int_support::Int>;
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
        reported = __terrane_await(Box::pin(results.send(self.total.clone()))).await;
        if !reported.accepted {
            return terrane_int_support::Int::from(-3_i128);
        }
        return self.total.clone();
    }
    pub async fn __terrane_shared_async_accumulate(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
        results: TerraneChannelSender<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let released: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let reported: TerraneChannelSendOutcome<terrane_int_support::Int>;
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
        reported = __terrane_await(
                Box::pin(
                    results
                        .send({
                            let __terrane_snapshot = __terrane_shared_handle
                                .lock()
                                .expect("shared async receiver lock poisoned")
                                .total
                                .clone();
                            __terrane_snapshot
                        }),
                ),
            )
            .await;
        if !reported.accepted {
            return terrane_int_support::Int::from(-3_i128);
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
#[derive(Clone)]
pub enum Base {
    Own(BaseStorage),
    Derived(Derived),
}
impl Base {
    pub fn terrane_construct() -> Self {
        Self::Own(BaseStorage::terrane_construct())
    }
    pub async fn accumulate(
        &mut self,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
        results: TerraneChannelSender<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        match self {
            Self::Own(value) => value.accumulate(delta, started, gate, results).await,
            Self::Derived(value) => value.accumulate(delta, started, gate, results).await,
        }
    }
    pub async fn __terrane_shared_async_accumulate(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
        results: TerraneChannelSender<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let __terrane_variant = {
            let value = __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned");
            match &*value {
                Self::Own(_) => 0,
                Self::Derived(_) => 1,
            }
        };
        match __terrane_variant {
            0 => {
                BaseStorage::__terrane_shared_async_accumulate(
                        TerraneSharedProjection::new(
                            __terrane_shared_handle,
                            |value: &Self| match value {
                                Self::Own(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                            |value: &mut Self| match value {
                                Self::Own(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                        ),
                        delta,
                        started,
                        gate,
                        results,
                    )
                    .await
            }
            1 => {
                Derived::__terrane_shared_async_accumulate(
                        TerraneSharedProjection::new(
                            __terrane_shared_handle,
                            |value: &Self| match value {
                                Self::Derived(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                            |value: &mut Self| match value {
                                Self::Derived(value) => value,
                                _ => unreachable!("shared receiver variant changed"),
                            },
                        ),
                        delta,
                        started,
                        gate,
                        results,
                    )
                    .await
            }
            _ => unreachable!("invalid shared receiver variant"),
        }
    }
    pub fn terrane_field_total(&self) -> &terrane_int_support::Int {
        match self {
            Self::Own(value) => &value.total,
            Self::Derived(value) => &value.total,
        }
    }
    pub fn terrane_field_total_mut(&mut self) -> &mut terrane_int_support::Int {
        match self {
            Self::Own(value) => &mut value.total,
            Self::Derived(value) => &mut value.total,
        }
    }
}
#[derive(Clone)]
pub struct Derived {
    pub total: terrane_int_support::Int,
    pub bonus: terrane_int_support::Int,
}
impl Derived {
    pub fn terrane_construct() -> Self {
        Self {
            total: terrane_int_support::Int::from(0_i128),
            bonus: terrane_int_support::Int::from(10_i128),
        }
    }
    pub async fn accumulate(
        &mut self,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
        results: TerraneChannelSender<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let released: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let reported: TerraneChannelSendOutcome<terrane_int_support::Int>;
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
            let __terrane_value = self.total.clone() + delta.clone()
                + self.bonus.clone();
            self.total = __terrane_value;
        }
        reported = __terrane_await(Box::pin(results.send(self.total.clone()))).await;
        if !reported.accepted {
            return terrane_int_support::Int::from(-3_i128);
        }
        return self.total.clone();
    }
    pub async fn __terrane_shared_async_accumulate(
        __terrane_shared_handle: impl TerraneSharedHandle<Self> + Clone,
        delta: terrane_int_support::Int,
        started: TerraneChannelSender<terrane_int_support::Int>,
        gate: TerraneChannelReceiver<terrane_int_support::Int>,
        results: TerraneChannelSender<terrane_int_support::Int>,
    ) -> terrane_int_support::Int {
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let released: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let reported: TerraneChannelSendOutcome<terrane_int_support::Int>;
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
            } + delta.clone()
                + {
                    let __terrane_snapshot = __terrane_shared_handle
                        .lock()
                        .expect("shared async receiver lock poisoned")
                        .bonus
                        .clone();
                    __terrane_snapshot
                };
            __terrane_shared_handle
                .lock()
                .expect("shared async receiver lock poisoned")
                .total = __terrane_value;
        }
        reported = __terrane_await(
                Box::pin(
                    results
                        .send({
                            let __terrane_snapshot = __terrane_shared_handle
                                .lock()
                                .expect("shared async receiver lock poisoned")
                                .total
                                .clone();
                            __terrane_snapshot
                        }),
                ),
            )
            .await;
        if !reported.accepted {
            return terrane_int_support::Int::from(-3_i128);
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
pub static TERRANE_STATIC_HOLDER_SLOT: std::sync::LazyLock<TerraneStaticObject<Base>> = std::sync::LazyLock::new(||
TerraneStaticObject::new(Base::Derived(Derived::terrane_construct())));
#[derive(Clone)]
pub struct Holder {}
impl Holder {
    pub fn terrane_construct() -> Self {
        Self {}
    }
}
fn main() {
    __terrane_run(async move {
        let started: TerraneChannelPair<terrane_int_support::Int>;
        let gate: TerraneChannelPair<terrane_int_support::Int>;
        let results: TerraneChannelPair<terrane_int_support::Int>;
        let scope: TerraneTaskScope;
        let child: TerraneScopedTask<terrane_int_support::Int>;
        let ready: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let released: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let done: TerraneTaskOutcome<terrane_int_support::Int>;
        let reported: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
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
        results = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        scope = TerraneTaskScope::new(None);
        child = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = {
                let __terrane_shared_call_handle = TERRANE_STATIC_HOLDER_SLOT.handle();
                let __terrane_shared_argument_0 = terrane_int_support::Int::from(5_i128);
                let __terrane_shared_argument_1 = started.sender;
                let __terrane_shared_argument_2 = gate.receiver;
                let __terrane_shared_argument_3 = results.sender;
                let __terrane_shared_argument_4 = __terrane_shared_argument_0;
                let __terrane_shared_argument_5 = __terrane_shared_argument_1;
                let __terrane_shared_argument_6 = __terrane_shared_argument_2;
                let __terrane_shared_argument_7 = __terrane_shared_argument_3;
                Base::__terrane_shared_async_accumulate(
                    __terrane_shared_call_handle,
                    __terrane_shared_argument_4,
                    __terrane_shared_argument_5,
                    __terrane_shared_argument_6,
                    __terrane_shared_argument_7,
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
        ready = __terrane_await(started.receiver.receive()).await;
        TERRANE_STATIC_HOLDER_SLOT.replace(Base::terrane_construct());
        {
            let __terrane_owner = TERRANE_STATIC_HOLDER_SLOT.handle();
            let __terrane_value = terrane_int_support::Int::from(100_i128);
            *__terrane_owner
                .lock()
                .expect("static field lock poisoned")
                .terrane_field_total_mut() = __terrane_value;
        }
        released = __terrane_await(
                gate.sender.send(terrane_int_support::Int::from(1_i128)),
            )
            .await;
        done = __terrane_await(scope.join(child)).await;
        reported = __terrane_await(results.receiver.receive()).await;
        println!(
            "{}", { let mut __terrane_text = String::new(); match &reported.value {
            Some(__terrane_present) => {
            terrane_scalar_support::ScalarDisplay::write_scalar(&__terrane_present, &mut
            __terrane_text); }, None => __terrane_text.push_str("none"), } __terrane_text
            }
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_snapshot = { let
            __terrane_result = TERRANE_STATIC_HOLDER_SLOT.handle().lock()
            .expect("static field lock poisoned").terrane_field_total().clone();
            __terrane_result }; __terrane_snapshot })
        );
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&ready.available),
            terrane_scalar_support::scalar_text(&released.accepted),
            terrane_scalar_support::scalar_text(&done.completed),
            terrane_scalar_support::scalar_text(&reported.available)
        );
    });
}
