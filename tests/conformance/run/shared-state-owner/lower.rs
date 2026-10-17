// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
#[derive(Clone)]
pub struct Counter {
    pub value: terrane_int_support::Int,
}
impl Counter {
    pub fn terrane_construct() -> Self {
        Self {
            value: terrane_int_support::Int::from(0_i128),
        }
    }
    pub fn increase(&mut self) -> terrane_int_support::Int {
        {
            let __terrane_value = self.value.clone()
                + terrane_int_support::Int::from(1_i128);
            self.value = __terrane_value;
        }
        return self.value.clone();
    }
}
async fn own_state(
    commands: TerraneChannelReceiver<String>,
    responses: TerraneChannelSender<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let mut state: Counter;
    let mut running: bool;
    let mut received: TerraneChannelReceiveOutcome<String>;
    let mut command: Option<String>;
    let mut sent: TerraneChannelSendOutcome<terrane_int_support::Int>;
    state = Counter::terrane_construct();
    running = true;
    while running {
        received = __terrane_await(Box::pin(commands.receive())).await;
        command = received.value;
        if command.is_some() {
            if match &command {
                Some(value) => value,
                _ => unreachable!("flow-proven storage refinement"),
            }
                .as_str() == "increase"
            {
                state.increase();
            } else {
                sent = __terrane_await(Box::pin(responses.send(state.value.clone())))
                    .await;
                if !sent.accepted {
                    return terrane_int_support::Int::from(-1_i128);
                }
                running = false;
            }
        }
    }
    commands.close();
    responses.close();
    return state.value.clone();
}
async fn use_state(
    commands: TerraneChannelSender<String>,
    responses: TerraneChannelReceiver<terrane_int_support::Int>,
) -> terrane_int_support::Int {
    let first: TerraneChannelSendOutcome<String>;
    let second: TerraneChannelSendOutcome<String>;
    let requested: TerraneChannelSendOutcome<String>;
    let received: TerraneChannelReceiveOutcome<terrane_int_support::Int>;
    let value: Option<terrane_int_support::Int>;
    first = __terrane_await(Box::pin(commands.send(String::from("increase")))).await;
    if !first.accepted {
        return terrane_int_support::Int::from(-1_i128);
    }
    second = __terrane_await(Box::pin(commands.send(String::from("increase")))).await;
    if !second.accepted {
        return terrane_int_support::Int::from(-1_i128);
    }
    requested = __terrane_await(Box::pin(commands.send(String::from("read")))).await;
    if !requested.accepted {
        return terrane_int_support::Int::from(-1_i128);
    }
    received = __terrane_await(Box::pin(responses.receive())).await;
    value = received.value;
    commands.close();
    responses.close();
    if value.is_some() {
        return match value {
            Some(value) => value,
            _ => unreachable!("flow-proven storage refinement"),
        };
    }
    return terrane_int_support::Int::from(-1_i128);
}
fn main() {
    __terrane_run(async move {
        let command_pair: TerraneChannelPair<String>;
        let response_pair: TerraneChannelPair<terrane_int_support::Int>;
        let scope: TerraneTaskScope;
        let owner: TerraneScopedTask<terrane_int_support::Int>;
        let client: TerraneScopedTask<terrane_int_support::Int>;
        let owner_outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let client_outcome: TerraneTaskOutcome<terrane_int_support::Int>;
        let owner_value: Option<terrane_int_support::Int>;
        let client_value: Option<terrane_int_support::Int>;
        command_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        response_pair = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        scope = TerraneTaskScope::new(None);
        owner = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = own_state(
                command_pair.receiver,
                response_pair.sender,
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
        client = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = use_state(
                command_pair.sender,
                response_pair.receiver,
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
        owner_outcome = __terrane_await(scope.join(owner)).await;
        client_outcome = __terrane_await(scope.join(client)).await;
        owner_value = owner_outcome.value.clone();
        client_value = client_outcome.value.clone();
        if owner_value.is_some() {
            if client_value.is_some() {
                println!(
                    "{}{}", terrane_scalar_support::scalar_text(&match &owner_value {
                    Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") }),
                    terrane_scalar_support::scalar_text(&match &client_value {
                    Some(value) => value, _ =>
                    unreachable!("flow-proven storage refinement") })
                );
            }
        }
    });
}
