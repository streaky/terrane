// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Message {
    pub text: String,
}
impl Message {
    pub fn terrane_construct(text: String) -> Self {
        let mut __terrane_constructed_value = Self { text: String::from("") };
        __terrane_constructed_value.construct(text);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, text: String) {
        {
            let __terrane_value = text;
            self.text = __terrane_value;
        }
    }
}
#[derive(Clone)]
pub struct StringBatch {
    pub values: terrane_collection_support::List<String>,
}
impl StringBatch {
    pub fn terrane_construct(values: terrane_collection_support::List<String>) -> Self {
        let mut __terrane_constructed_value = Self {
            values: terrane_collection_support::List::<
                String,
            >::new(vec![String::from("")]),
        };
        __terrane_constructed_value.construct(values);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, values: terrane_collection_support::List<String>) {
        {
            let __terrane_value = values;
            self.values = __terrane_value;
        }
    }
}
fn main() {
    __terrane_run(async move {
        let pair: TerraneChannelPair<String>;
        let tx: TerraneChannelSender<String>;
        let rx: TerraneChannelReceiver<String>;
        let first: TerraneChannelSendOutcome<String>;
        let scope: TerraneTaskScope;
        let blocked: TerraneScopedTask<TerraneChannelSendOutcome<String>>;
        let received_first: TerraneChannelReceiveOutcome<String>;
        let received_first_value: Option<String>;
        let delivered: TerraneTaskOutcome<TerraneChannelSendOutcome<String>>;
        let delivered_value: Option<TerraneChannelSendOutcome<String>>;
        let end: TerraneChannelReceiveOutcome<String>;
        let failing_pair: TerraneChannelPair<String>;
        let failing_tx: TerraneChannelSender<String>;
        let failing_rx: TerraneChannelReceiver<String>;
        let accepted: TerraneChannelSendOutcome<String>;
        let refused: TerraneChannelSendOutcome<String>;
        let rejected_value: Option<String>;
        let kept: TerraneChannelReceiveOutcome<String>;
        let failed_end: TerraneChannelReceiveOutcome<String>;
        let newest_pair: TerraneChannelPair<String>;
        let newest_tx: TerraneChannelSender<String>;
        let newest_rx: TerraneChannelReceiver<String>;
        let newest_first: TerraneChannelSendOutcome<String>;
        let newest_drop: TerraneChannelSendOutcome<String>;
        let newest_dropped_value: Option<String>;
        let newest_value: TerraneChannelReceiveOutcome<String>;
        let newest_result: Option<String>;
        let oldest_pair: TerraneChannelPair<String>;
        let oldest_tx: TerraneChannelSender<String>;
        let oldest_rx: TerraneChannelReceiver<String>;
        let oldest_first: TerraneChannelSendOutcome<String>;
        let oldest_drop: TerraneChannelSendOutcome<String>;
        let oldest_dropped_value: Option<String>;
        let oldest_value: TerraneChannelReceiveOutcome<String>;
        let oldest_result: Option<String>;
        let cancelled_pair: TerraneChannelPair<String>;
        let cancelled_tx: TerraneChannelSender<String>;
        let cancelled_rx: TerraneChannelReceiver<String>;
        let fill: TerraneChannelSendOutcome<String>;
        let cancel_scope: TerraneTaskScope;
        let pending: TerraneScopedTask<TerraneChannelSendOutcome<String>>;
        let cancelled: TerraneTaskOutcome<TerraneChannelSendOutcome<String>>;
        let drained: TerraneChannelReceiveOutcome<String>;
        let cancelled_end: TerraneChannelReceiveOutcome<String>;
        let closed_pair: TerraneChannelPair<String>;
        let closed_tx: TerraneChannelSender<String>;
        let closed_rx: TerraneChannelReceiver<String>;
        let closed_fill: TerraneChannelSendOutcome<String>;
        let closed_scope: TerraneTaskScope;
        let closed_pending: TerraneScopedTask<TerraneChannelSendOutcome<String>>;
        let closed_values: terrane_collection_support::List<String>;
        let closed_outcome: TerraneTaskOutcome<TerraneChannelSendOutcome<String>>;
        let closed_result: Option<TerraneChannelSendOutcome<String>>;
        let rejected_pair: TerraneChannelPair<String>;
        let rejected_tx: TerraneChannelSender<String>;
        let rejected_rx: TerraneChannelReceiver<String>;
        let rejected_values: terrane_collection_support::List<String>;
        let rejected_send: TerraneChannelSendOutcome<String>;
        let rejected_item: Option<String>;
        let rendezvous_pair: TerraneChannelPair<String>;
        let rendezvous_tx: TerraneChannelSender<String>;
        let rendezvous_rx: TerraneChannelReceiver<String>;
        let rendezvous_scope: TerraneTaskScope;
        let rendezvous_pending: TerraneScopedTask<TerraneChannelSendOutcome<String>>;
        let rendezvous_received: TerraneChannelReceiveOutcome<String>;
        let rendezvous_delivered: TerraneTaskOutcome<TerraneChannelSendOutcome<String>>;
        let rendezvous_value: Option<String>;
        let rendezvous_outcome: Option<TerraneChannelSendOutcome<String>>;
        let rendezvous_remaining: terrane_collection_support::List<String>;
        let stress_pair: TerraneChannelPair<terrane_int_support::Int>;
        let stress_tx: TerraneChannelSender<terrane_int_support::Int>;
        let stress_rx: TerraneChannelReceiver<terrane_int_support::Int>;
        let mut index: terrane_int_support::Int;
        let mut stress_send: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let mut remaining: terrane_int_support::Int;
        let mut stress_received: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let mut stress_value: Option<terrane_int_support::Int>;
        let message_pair: TerraneChannelPair<Message>;
        let message_tx: TerraneChannelSender<Message>;
        let message_rx: TerraneChannelReceiver<Message>;
        let message_send: TerraneChannelSendOutcome<Message>;
        let message_receive: TerraneChannelReceiveOutcome<Message>;
        let message_value: Option<Message>;
        let mut values: terrane_collection_support::List<String>;
        let batch_pair: TerraneChannelPair<StringBatch>;
        let batch_tx: TerraneChannelSender<StringBatch>;
        let batch_rx: TerraneChannelReceiver<StringBatch>;
        let batch_send: TerraneChannelSendOutcome<StringBatch>;
        let batch_receive: TerraneChannelReceiveOutcome<StringBatch>;
        let batch_value: Option<StringBatch>;
        let resource_pair: TerraneChannelPair<Outgoing>;
        let resource_tx: TerraneChannelSender<Outgoing>;
        let resource_rx: TerraneChannelReceiver<Outgoing>;
        let resource_send: TerraneChannelSendOutcome<Outgoing>;
        let resource_receive: TerraneChannelReceiveOutcome<Outgoing>;
        pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        tx = pair.sender;
        rx = pair.receiver;
        first = __terrane_await(Box::pin(tx.send(String::from("one")))).await;
        println!(
            "{}{}{}", terrane_scalar_support::scalar_text(&first.accepted),
            terrane_scalar_support::scalar_text(&first.closed),
            terrane_scalar_support::scalar_text(&first.dropped)
        );
        scope = TerraneTaskScope::new(None);
        blocked = {
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
        received_first = __terrane_await(Box::pin(rx.receive())).await;
        received_first_value = received_first.value;
        if (&received_first_value).is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&received_first.available),
                terrane_scalar_support::scalar_text(&match &received_first_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        delivered = __terrane_await(scope.join(blocked)).await;
        delivered_value = delivered.value.clone();
        if (&delivered_value).is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&delivered.completed),
                terrane_scalar_support::scalar_text(&match &delivered_value { Some(value)
                => value, _ => unreachable!("flow-proven storage refinement") }
                .accepted)
            );
        }
        tx.close();
        end = __terrane_await(Box::pin(rx.receive())).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&end.available),
            terrane_scalar_support::scalar_text(&end.closed)
        );
        failing_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::FailSend,
        );
        failing_tx = failing_pair.sender;
        failing_rx = failing_pair.receiver;
        accepted = __terrane_await(Box::pin(failing_tx.send(String::from("kept"))))
            .await;
        refused = __terrane_await(Box::pin(failing_tx.send(String::from("refused"))))
            .await;
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&accepted.accepted),
            terrane_scalar_support::scalar_text(&refused.accepted),
            terrane_scalar_support::scalar_text(&refused.closed),
            terrane_scalar_support::scalar_text(&refused.dropped)
        );
        rejected_value = refused.rejected_value;
        if (&rejected_value).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &rejected_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        failing_tx.close();
        kept = __terrane_await(Box::pin(failing_rx.receive())).await;
        failed_end = __terrane_await(Box::pin(failing_rx.receive())).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&kept.available),
            terrane_scalar_support::scalar_text(&failed_end.closed)
        );
        newest_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::DropNewest,
        );
        newest_tx = newest_pair.sender;
        newest_rx = newest_pair.receiver;
        newest_first = __terrane_await(Box::pin(newest_tx.send(String::from("kept"))))
            .await;
        newest_drop = __terrane_await(Box::pin(newest_tx.send(String::from("dropped"))))
            .await;
        newest_dropped_value = newest_drop.dropped_value;
        if (&newest_dropped_value).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &newest_dropped_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&newest_first.accepted),
            terrane_scalar_support::scalar_text(&newest_drop.accepted),
            terrane_scalar_support::scalar_text(&newest_drop.closed),
            terrane_scalar_support::scalar_text(&newest_drop.dropped)
        );
        newest_value = __terrane_await(Box::pin(newest_rx.receive())).await;
        newest_result = newest_value.value;
        if (&newest_result).is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&newest_value.available),
                terrane_scalar_support::scalar_text(&match &newest_result { Some(value)
                => value, _ => unreachable!("flow-proven storage refinement") })
            );
        }
        oldest_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::DropOldest,
        );
        oldest_tx = oldest_pair.sender;
        oldest_rx = oldest_pair.receiver;
        oldest_first = __terrane_await(Box::pin(oldest_tx.send(String::from("old"))))
            .await;
        oldest_drop = __terrane_await(Box::pin(oldest_tx.send(String::from("new"))))
            .await;
        oldest_dropped_value = oldest_drop.dropped_value;
        if (&oldest_dropped_value).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &oldest_dropped_value {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&oldest_first.accepted),
            terrane_scalar_support::scalar_text(&oldest_drop.accepted),
            terrane_scalar_support::scalar_text(&oldest_drop.closed),
            terrane_scalar_support::scalar_text(&oldest_drop.dropped)
        );
        oldest_value = __terrane_await(Box::pin(oldest_rx.receive())).await;
        oldest_result = oldest_value.value;
        if (&oldest_result).is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&oldest_value.available),
                terrane_scalar_support::scalar_text(&match &oldest_result { Some(value)
                => value, _ => unreachable!("flow-proven storage refinement") })
            );
        }
        cancelled_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        cancelled_tx = cancelled_pair.sender;
        cancelled_rx = cancelled_pair.receiver;
        fill = __terrane_await(Box::pin(cancelled_tx.send(String::from("full")))).await;
        cancel_scope = TerraneTaskScope::new(None);
        pending = {
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
        cancelled = __terrane_await(cancel_scope.join(pending)).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&fill.accepted),
            terrane_scalar_support::scalar_text(&cancelled.cancelled)
        );
        cancelled_tx.close();
        drained = __terrane_await(Box::pin(cancelled_rx.receive())).await;
        cancelled_end = __terrane_await(Box::pin(cancelled_rx.receive())).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&drained.available),
            terrane_scalar_support::scalar_text(&cancelled_end.closed)
        );
        closed_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        closed_tx = closed_pair.sender;
        closed_rx = closed_pair.receiver;
        closed_fill = __terrane_await(Box::pin(closed_tx.send(String::from("full"))))
            .await;
        closed_scope = TerraneTaskScope::new(None);
        closed_pending = {
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
        closed_values = closed_rx.close();
        closed_outcome = __terrane_await(closed_scope.join(closed_pending)).await;
        closed_result = closed_outcome.value.clone();
        println!(
            "{}{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(closed_values
            .length())),
            terrane_scalar_support::scalar_text(&__terrane_raised(closed_values
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            0 /* terrane-site: src/main.trn:104:32-104:48 */)), 0 /* terrane-site: src/main.trn:104:32-104:48 */))
        );
        if (&closed_result).is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&closed_fill.accepted),
                terrane_scalar_support::scalar_text(&closed_outcome.completed),
                terrane_scalar_support::scalar_text(&match &closed_result { Some(value)
                => value, _ => unreachable!("flow-proven storage refinement") } .closed)
            );
        }
        rejected_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        rejected_tx = rejected_pair.sender;
        rejected_rx = rejected_pair.receiver;
        rejected_values = rejected_rx.close();
        rejected_send = __terrane_await(
                Box::pin(rejected_tx.send(String::from("blocked"))),
            )
            .await;
        rejected_item = rejected_send.rejected_value;
        if (&rejected_item).is_some() {
            println!(
                "{}", terrane_scalar_support::scalar_text(&match &rejected_item {
                Some(value) => value, _ => unreachable!("flow-proven storage refinement")
                })
            );
        }
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rejected_values
            .length()))
        );
        rendezvous_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        rendezvous_tx = rendezvous_pair.sender;
        rendezvous_rx = rendezvous_pair.receiver;
        rendezvous_scope = TerraneTaskScope::new(None);
        rendezvous_pending = {
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
        rendezvous_received = __terrane_await(Box::pin(rendezvous_rx.receive())).await;
        rendezvous_scope.cancel();
        rendezvous_delivered = __terrane_await(rendezvous_scope.join(rendezvous_pending))
            .await;
        rendezvous_value = rendezvous_received.value;
        rendezvous_outcome = rendezvous_delivered.value.clone();
        if (&rendezvous_value).is_some() {
            if (&rendezvous_outcome).is_some() {
                println!(
                    "{}{}{}{}", terrane_scalar_support::scalar_text(&match
                    &rendezvous_value { Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") }),
                    terrane_scalar_support::scalar_text(&rendezvous_delivered.completed),
                    terrane_scalar_support::scalar_text(&rendezvous_delivered.cancelled),
                    terrane_scalar_support::scalar_text(&match &rendezvous_outcome {
                    Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") } .accepted)
                );
            }
        }
        rendezvous_tx.close();
        rendezvous_remaining = rendezvous_rx.close();
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(rendezvous_remaining
            .length()))
        );
        stress_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(4_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::DropOldest,
        );
        stress_tx = stress_pair.sender;
        stress_rx = stress_pair.receiver;
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(10000_i128) {
            stress_send = __terrane_await(Box::pin(stress_tx.send(index.clone()))).await;
            index = index.clone() + terrane_int_support::Int::from(1_i128);
            if stress_send.closed {
                println!("{}", terrane_scalar_support::scalar_text(&false));
            }
        }
        stress_tx.close();
        remaining = terrane_int_support::Int::from(4_i128);
        while remaining > terrane_int_support::Int::from(0_i128) {
            stress_received = __terrane_await(Box::pin(stress_rx.receive())).await;
            stress_value = stress_received.value;
            if (&stress_value).is_some() {
                println!(
                    "{}", terrane_scalar_support::scalar_text(&match &stress_value {
                    Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") })
                );
            }
            remaining = remaining.clone() - terrane_int_support::Int::from(1_i128);
        }
        message_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        message_tx = message_pair.sender;
        message_rx = message_pair.receiver;
        message_send = __terrane_await(
                Box::pin(
                    message_tx.send(Message::terrane_construct(String::from("typed"))),
                ),
            )
            .await;
        message_tx.close();
        message_receive = __terrane_await(Box::pin(message_rx.receive())).await;
        message_value = message_receive.value;
        if (&message_value).is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&message_send.accepted),
                terrane_scalar_support::scalar_text(&match &message_value { Some(value)
                => value, _ => unreachable!("flow-proven storage refinement") } .text)
            );
        }
        values = terrane_collection_support::List::<String>::new(vec![]);
        {
            let __terrane_collection_argument_0 = String::from("collection");
            values.append(__terrane_collection_argument_0);
        };
        batch_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        batch_tx = batch_pair.sender;
        batch_rx = batch_pair.receiver;
        batch_send = __terrane_await(
                Box::pin(batch_tx.send(StringBatch::terrane_construct(values.clone()))),
            )
            .await;
        batch_tx.close();
        batch_receive = __terrane_await(Box::pin(batch_rx.receive())).await;
        batch_value = batch_receive.value;
        if (&batch_value).is_some() {
            println!(
                "{}{}{}", terrane_scalar_support::scalar_text(&batch_send.accepted),
                terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(match
                &batch_value { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") } .values.length())),
                terrane_scalar_support::scalar_text(&__terrane_raised(match &batch_value
                { Some(value) => value, _ =>
                unreachable!("flow-proven storage refinement") } .values
                .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
                1 /* terrane-site: src/main.trn:173:60-173:81 */)),
                1 /* terrane-site: src/main.trn:173:60-173:81 */))
            );
        }
        resource_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        resource_tx = resource_pair.sender;
        resource_rx = resource_pair.receiver;
        resource_send = __terrane_await(
                Box::pin(
                    resource_tx
                        .send(
                            __terrane_raised(
                                remotely_closed_sink(),
                                2 /* terrane-site: src/main.trn:178:44-178:65 */,
                            ),
                        ),
                ),
            )
            .await;
        resource_tx.close();
        resource_receive = __terrane_await(Box::pin(resource_rx.receive())).await;
        println!(
            "{}{}", terrane_scalar_support::scalar_text(&resource_send.accepted),
            terrane_scalar_support::scalar_text(&resource_receive.available)
        );
    });
}
