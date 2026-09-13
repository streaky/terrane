// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_local.rs, async_dependency.rs, channels.rs, tasks_native_local.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Message {
    pub text: String,
}
impl Message {
    pub fn terrane_construct(text: String) -> Self {
        let mut value = Self { text: String::from("") };
        value.construct(text);
        value
    }
    pub fn construct(&mut self, text: String) {
        self.text = text;
    }
}
#[derive(Clone)]
pub struct StringBatch {
    pub values: terrane_collection_support::List<String>,
}
impl StringBatch {
    pub fn terrane_construct(values: terrane_collection_support::List<String>) -> Self {
        let mut value = Self {
            values: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
        };
        value.construct(values);
        value
    }
    pub fn construct(&mut self, values: terrane_collection_support::List<String>) {
        self.values = values.clone();
    }
}
fn main() {
    __terrane_run(async move {
        let pair: TerraneChannelPair<String> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let tx: TerraneChannelSender<String> = pair.sender;
        let rx: TerraneChannelReceiver<String> = pair.receiver;
        let first: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(tx.send(String::from("one"))),
            )
            .await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&first.accepted),
            terrane_scalar_support::scalar_text(&first.closed),
            terrane_scalar_support::scalar_text(&first.dropped)
        );
        let scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let blocked: TerraneScopedTask<TerraneChannelSendOutcome<String>> = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = Box::pin(tx.send(String::from("two")));
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
        let received_first: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(rx.receive()),
            )
            .await;
        let received_first_value: Option<String> = received_first.value;
        if received_first_value.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&received_first.available),
                terrane_scalar_support::scalar_text(&* received_first_value.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        let delivered: TerraneTaskOutcome<TerraneChannelSendOutcome<String>> = __terrane_await(
                scope.join(blocked),
            )
            .await;
        let delivered_value: Option<TerraneChannelSendOutcome<String>> = delivered
            .value
            .clone();
        if delivered_value.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&delivered.completed),
                terrane_scalar_support::scalar_text(&delivered_value.as_ref()
                .expect("semantic optional narrowing").accepted)
            );
        }
        tx.close();
        let end: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(rx.receive()),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&end.available),
            terrane_scalar_support::scalar_text(&end.closed)
        );
        let failing_pair: TerraneChannelPair<String> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::FailSend,
        );
        let failing_tx: TerraneChannelSender<String> = failing_pair.sender;
        let failing_rx: TerraneChannelReceiver<String> = failing_pair.receiver;
        let accepted: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(failing_tx.send(String::from("kept"))),
            )
            .await;
        let refused: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(failing_tx.send(String::from("refused"))),
            )
            .await;
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&accepted.accepted),
            terrane_scalar_support::scalar_text(&refused.accepted),
            terrane_scalar_support::scalar_text(&refused.closed),
            terrane_scalar_support::scalar_text(&refused.dropped)
        );
        let rejected_value: Option<String> = refused.rejected_value;
        if rejected_value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&* rejected_value.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        failing_tx.close();
        let kept: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(failing_rx.receive()),
            )
            .await;
        let failed_end: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(failing_rx.receive()),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&kept.available),
            terrane_scalar_support::scalar_text(&failed_end.closed)
        );
        let newest_pair: TerraneChannelPair<String> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::DropNewest,
        );
        let newest_tx: TerraneChannelSender<String> = newest_pair.sender;
        let newest_rx: TerraneChannelReceiver<String> = newest_pair.receiver;
        let newest_first: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(newest_tx.send(String::from("kept"))),
            )
            .await;
        let newest_drop: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(newest_tx.send(String::from("dropped"))),
            )
            .await;
        let newest_dropped_value: Option<String> = newest_drop.dropped_value;
        if newest_dropped_value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&* newest_dropped_value
                .as_ref().expect("semantic optional narrowing"))
            );
        }
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&newest_first.accepted),
            terrane_scalar_support::scalar_text(&newest_drop.accepted),
            terrane_scalar_support::scalar_text(&newest_drop.closed),
            terrane_scalar_support::scalar_text(&newest_drop.dropped)
        );
        let newest_value: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(newest_rx.receive()),
            )
            .await;
        let newest_result: Option<String> = newest_value.value;
        if newest_result.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&newest_value.available),
                terrane_scalar_support::scalar_text(&* newest_result.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        let oldest_pair: TerraneChannelPair<String> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::DropOldest,
        );
        let oldest_tx: TerraneChannelSender<String> = oldest_pair.sender;
        let oldest_rx: TerraneChannelReceiver<String> = oldest_pair.receiver;
        let oldest_first: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(oldest_tx.send(String::from("old"))),
            )
            .await;
        let oldest_drop: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(oldest_tx.send(String::from("new"))),
            )
            .await;
        let oldest_dropped_value: Option<String> = oldest_drop.dropped_value;
        if oldest_dropped_value.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&* oldest_dropped_value
                .as_ref().expect("semantic optional narrowing"))
            );
        }
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&oldest_first.accepted),
            terrane_scalar_support::scalar_text(&oldest_drop.accepted),
            terrane_scalar_support::scalar_text(&oldest_drop.closed),
            terrane_scalar_support::scalar_text(&oldest_drop.dropped)
        );
        let oldest_value: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(oldest_rx.receive()),
            )
            .await;
        let oldest_result: Option<String> = oldest_value.value;
        if oldest_result.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&oldest_value.available),
                terrane_scalar_support::scalar_text(&* oldest_result.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        let cancelled_pair: TerraneChannelPair<String> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let cancelled_tx: TerraneChannelSender<String> = cancelled_pair.sender;
        let cancelled_rx: TerraneChannelReceiver<String> = cancelled_pair.receiver;
        let fill: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(cancelled_tx.send(String::from("full"))),
            )
            .await;
        let cancel_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let pending: TerraneScopedTask<TerraneChannelSendOutcome<String>> = {
            let __terrane_scope = cancel_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = Box::pin(
                cancelled_tx.send(String::from("blocked")),
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
        cancel_scope.cancel();
        let cancelled: TerraneTaskOutcome<TerraneChannelSendOutcome<String>> = __terrane_await(
                cancel_scope.join(pending),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&fill.accepted),
            terrane_scalar_support::scalar_text(&cancelled.cancelled)
        );
        cancelled_tx.close();
        let drained: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(cancelled_rx.receive()),
            )
            .await;
        let cancelled_end: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(cancelled_rx.receive()),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&drained.available),
            terrane_scalar_support::scalar_text(&cancelled_end.closed)
        );
        let closed_pair: TerraneChannelPair<String> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let closed_tx: TerraneChannelSender<String> = closed_pair.sender;
        let closed_rx: TerraneChannelReceiver<String> = closed_pair.receiver;
        let closed_fill: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(closed_tx.send(String::from("full"))),
            )
            .await;
        let closed_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let closed_pending: TerraneScopedTask<TerraneChannelSendOutcome<String>> = {
            let __terrane_scope = closed_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = Box::pin(
                closed_tx.send(String::from("blocked")),
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
        let closed_values: terrane_collection_support::List<String> = closed_rx.close();
        let closed_outcome: TerraneTaskOutcome<TerraneChannelSendOutcome<String>> = __terrane_await(
                closed_scope.join(closed_pending),
            )
            .await;
        let closed_result: Option<TerraneChannelSendOutcome<String>> = closed_outcome
            .value
            .clone();
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(closed_values
            .length())),
            terrane_scalar_support::scalar_text(&__terrane_raised(closed_values
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            0 /* terrane-site: src/main.trn:105:32-105:48 */)), 0 /* terrane-site: src/main.trn:105:32-105:48 */))
        );
        if closed_result.is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&closed_fill.accepted),
                terrane_scalar_support::scalar_text(&closed_outcome.completed),
                terrane_scalar_support::scalar_text(&closed_result.as_ref()
                .expect("semantic optional narrowing").closed)
            );
        }
        let rejected_pair: TerraneChannelPair<String> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let rejected_tx: TerraneChannelSender<String> = rejected_pair.sender;
        let rejected_rx: TerraneChannelReceiver<String> = rejected_pair.receiver;
        let rejected_values: terrane_collection_support::List<String> = rejected_rx
            .close();
        let rejected_send: TerraneChannelSendOutcome<String> = __terrane_await(
                Box::pin(rejected_tx.send(String::from("blocked"))),
            )
            .await;
        let rejected_item: Option<String> = rejected_send.rejected_value;
        if rejected_item.is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&* rejected_item.as_ref()
                .expect("semantic optional narrowing"))
            );
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rejected_values
            .length()))
        );
        let rendezvous_pair: TerraneChannelPair<String> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let rendezvous_tx: TerraneChannelSender<String> = rendezvous_pair.sender;
        let rendezvous_rx: TerraneChannelReceiver<String> = rendezvous_pair.receiver;
        let rendezvous_scope: TerraneTaskScope = TerraneTaskScope::new(None);
        let rendezvous_pending: TerraneScopedTask<TerraneChannelSendOutcome<String>> = {
            let __terrane_scope = rendezvous_scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = Box::pin(
                rendezvous_tx.send(String::from("handed-over")),
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
        let rendezvous_received: TerraneChannelReceiveOutcome<String> = __terrane_await(
                Box::pin(rendezvous_rx.receive()),
            )
            .await;
        rendezvous_scope.cancel();
        let rendezvous_delivered: TerraneTaskOutcome<
            TerraneChannelSendOutcome<String>,
        > = __terrane_await(rendezvous_scope.join(rendezvous_pending)).await;
        let rendezvous_value: Option<String> = rendezvous_received.value;
        let rendezvous_outcome: Option<TerraneChannelSendOutcome<String>> = rendezvous_delivered
            .value
            .clone();
        if rendezvous_value.is_some() {
            if rendezvous_outcome.is_some() {
                println!(
                    "{}{}{}{}", terrane_scalar_support::scalar_text(&* rendezvous_value
                    .as_ref().expect("semantic optional narrowing")),
                    terrane_scalar_support::scalar_text(&rendezvous_delivered.completed),
                    terrane_scalar_support::scalar_text(&rendezvous_delivered.cancelled),
                    terrane_scalar_support::scalar_text(&rendezvous_outcome.as_ref()
                    .expect("semantic optional narrowing").accepted)
                );
            }
        }
        rendezvous_tx.close();
        let rendezvous_remaining: terrane_collection_support::List<String> = rendezvous_rx
            .close();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rendezvous_remaining
            .length()))
        );
        let stress_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(4_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::DropOldest,
        );
        let stress_tx: TerraneChannelSender<terrane_int_support::Int> = stress_pair
            .sender;
        let stress_rx: TerraneChannelReceiver<terrane_int_support::Int> = stress_pair
            .receiver;
        let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        while index.clone() < terrane_int_support::Int::from(10000_i128) {
            let stress_send: TerraneChannelSendOutcome<terrane_int_support::Int> = __terrane_await(
                    Box::pin(stress_tx.send(index.clone())),
                )
                .await;
            index = index.clone() + terrane_int_support::Int::from(1_i128);
            if stress_send.closed {
                println!("{}", terrane_scalar_support::scalar_text(&false));
            }
        }
        stress_tx.close();
        let mut remaining: terrane_int_support::Int = terrane_int_support::Int::from(
            4_i128,
        );
        while remaining.clone() > terrane_int_support::Int::from(0_i128) {
            let stress_received: TerraneChannelReceiveOutcome<
                terrane_int_support::Int,
            > = __terrane_await(Box::pin(stress_rx.receive())).await;
            let stress_value: Option<terrane_int_support::Int> = stress_received.value;
            if stress_value.is_some() {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&* stress_value.as_ref()
                    .expect("semantic optional narrowing"))
                );
            }
            remaining = remaining.clone() - terrane_int_support::Int::from(1_i128);
        }
        let message_pair: TerraneChannelPair<Message> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let message_tx: TerraneChannelSender<Message> = message_pair.sender;
        let message_rx: TerraneChannelReceiver<Message> = message_pair.receiver;
        let message_send: TerraneChannelSendOutcome<Message> = __terrane_await(
                Box::pin(
                    message_tx.send(Message::terrane_construct(String::from("typed"))),
                ),
            )
            .await;
        message_tx.close();
        let message_receive: TerraneChannelReceiveOutcome<Message> = __terrane_await(
                Box::pin(message_rx.receive()),
            )
            .await;
        let message_value: Option<Message> = message_receive.value;
        if message_value.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&message_send.accepted),
                terrane_scalar_support::scalar_text(&message_value.as_ref()
                .expect("semantic optional narrowing").text)
            );
        }
        let mut values: terrane_collection_support::List<String> = terrane_collection_support::List::<
            String,
        >::new(vec![]);
        values.append(String::from("collection"));
        let batch_pair: TerraneChannelPair<StringBatch> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let batch_tx: TerraneChannelSender<StringBatch> = batch_pair.sender;
        let batch_rx: TerraneChannelReceiver<StringBatch> = batch_pair.receiver;
        let batch_send: TerraneChannelSendOutcome<StringBatch> = __terrane_await(
                Box::pin(batch_tx.send(StringBatch::terrane_construct(values.clone()))),
            )
            .await;
        batch_tx.close();
        let batch_receive: TerraneChannelReceiveOutcome<StringBatch> = __terrane_await(
                Box::pin(batch_rx.receive()),
            )
            .await;
        let batch_value: Option<StringBatch> = batch_receive.value;
        if batch_value.is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&batch_send.accepted),
                terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(batch_value
                .as_ref().expect("semantic optional narrowing").values.length())),
                terrane_scalar_support::scalar_text(&__terrane_raised(batch_value
                .as_ref().expect("semantic optional narrowing").values
                .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                1 /* terrane-site: src/main.trn:174:60-174:81 */)),
                1 /* terrane-site: src/main.trn:174:60-174:81 */))
            );
        }
        let resource_pair: TerraneChannelPair<Outgoing> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let resource_tx: TerraneChannelSender<Outgoing> = resource_pair.sender;
        let resource_rx: TerraneChannelReceiver<Outgoing> = resource_pair.receiver;
        let resource_send: TerraneChannelSendOutcome<Outgoing> = __terrane_await(
                Box::pin(
                    resource_tx
                        .send(
                            __terrane_raised(
                                remotely_closed_sink(),
                                2 /* terrane-site: src/main.trn:179:44-179:65 */,
                            ),
                        ),
                ),
            )
            .await;
        resource_tx.close();
        let resource_receive: TerraneChannelReceiveOutcome<Outgoing> = __terrane_await(
                Box::pin(resource_rx.receive()),
            )
            .await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&resource_send.accepted),
            terrane_scalar_support::scalar_text(&resource_receive.available)
        );
    });
}
// Source: <terrane>/projected/deps/terrane-sink-witness.trn
// Namespace: deps/terrane-sink-witness
pub use terrane_sink_witness::Outgoing;
pub fn remotely_closed_sink() -> Result<Outgoing, crate::TerraneForeignError> {
    match std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| terrane_sink_witness::remotely_closed_sink()),
    ) {
        Ok(value) => Ok(value),
        Err(payload) => {
            Err(
                crate::__terrane_dependency_panic(
                    payload,
                    "terrane-sink-witness",
                    "terrane_sink_witness::remotely_closed_sink",
                ),
            )
        }
    }
}
// Source: core/concurrency.trn
// Namespace: core/concurrency
#[derive(Clone)]
pub struct ConcurrencyOperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl ConcurrencyOperationResult {
    pub fn terrane_construct(
        did_fail: bool,
        exceeded_deadline: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        value.construct(did_fail, exceeded_deadline, detail);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        exceeded_deadline: bool,
        detail: String,
    ) {
        self.failed = did_fail;
        self.deadline_exceeded = exceeded_deadline;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct ConcurrencyIntResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub available: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl ConcurrencyIntResult {
    pub fn terrane_construct(
        did_fail: bool,
        exceeded_deadline: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            available: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        value.construct(did_fail, exceeded_deadline, has_value, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        exceeded_deadline: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) {
        self.failed = did_fail;
        self.deadline_exceeded = exceeded_deadline;
        self.available = has_value;
        self.message = detail;
        self.value = result_value.clone();
    }
}
#[derive(Clone)]
pub struct IntMutex {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntMutex {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_mutex(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn load(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_load(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_store(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_add(
            &self.handle,
            amount,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct IntReadWriteLock {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntReadWriteLock {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn read(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_read(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_write(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
#[derive(Clone)]
pub struct MemoryOrder {
    pub name: String,
}
impl MemoryOrder {
    pub fn terrane_construct(ordering_name: String) -> Self {
        let mut value = Self {
            name: String::from("sequentially-consistent"),
        };
        value.construct(ordering_name);
        value
    }
    pub fn construct(&mut self, ordering_name: String) {
        self.name = ordering_name;
    }
}
pub fn relaxed_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("relaxed"));
}
pub fn acquire_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("acquire"));
}
pub fn release_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("release"));
}
pub fn acquire_release_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("acquire-release"));
}
pub fn sequentially_consistent_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("sequentially-consistent"));
}
#[derive(Clone)]
pub struct AtomicInt64 {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl AtomicInt64 {
    pub fn terrane_construct(initial: i64) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: i64) {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn load(&self, ordering: MemoryOrder) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(
        &self,
        value: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(
        &mut self,
        amount: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct ThreadLocalInt {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl ThreadLocalInt {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn get(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_get(
            &self.handle,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
