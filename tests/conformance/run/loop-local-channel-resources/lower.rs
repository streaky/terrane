// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let mut index: terrane_int_support::Int;
        let mut pair: TerraneChannelPair<terrane_int_support::Int>;
        let mut sender: TerraneChannelSender<terrane_int_support::Int>;
        let mut receiver: TerraneChannelReceiver<terrane_int_support::Int>;
        let mut sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
        let mut received: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
        let mut value: Option<terrane_int_support::Int>;
        let mut remaining: terrane_collection_support::List<terrane_int_support::Int>;
        index = terrane_int_support::Int::from(0_i128);
        while index < terrane_int_support::Int::from(2_i128) {
            pair = TerraneChannelPair::new(
                terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(1_i128),
                    )
                    .expect("semantic channel capacity"),
                TerraneChannelOverflow::Block,
            );
            sender = pair.sender;
            receiver = pair.receiver;
            sent = __terrane_await(Box::pin(sender.send(index.clone()))).await;
            sender.close();
            received = __terrane_await(Box::pin(receiver.receive())).await;
            value = received.value;
            remaining = receiver.close();
            if (&value).is_some() {
                println!(
                    "{}{}{}", terrane_scalar_support::scalar_text(&sent.accepted),
                    terrane_scalar_support::scalar_text(&match &value { Some(value) =>
                    value, _ => unreachable!("flow-proven storage refinement") }),
                    terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(remaining
                    .length()))
                );
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    });
}
