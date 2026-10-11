// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
async fn serve(listener: TcpListener, options: NetworkOperationOptions) {
    let accepted: StreamResult;
    let stream: TcpStream;
    let request: IoResult;
    let written: IoResult;
    accepted = __terrane_await((&listener).accept(options.clone())).await;
    stream = accepted.value;
    request = __terrane_await(
            (&stream).read(terrane_int_support::Int::from(7_i128), options.clone()),
        )
        .await;
    if request.data == Vec::from([116, 101, 114, 114, 97, 110, 101]) {
        written = __terrane_await(
                (&stream).write(Vec::from([114, 101, 112, 108, 121]), options),
            )
            .await;
        if !written.failed {
            stream.shutdown(String::from("both"));
        }
    }
    stream.close();
    return ();
}
async fn read_input(input: ByteReader) -> ReadResult {
    return __terrane_await((&input).read_async(terrane_int_support::Int::from(1_i128)))
        .await;
}
fn main() {
    __terrane_run(async move {
        let loopback: IpResult;
        let bind_address: SocketResult;
        let bound: ListenerResult;
        let listener: TcpListener;
        let endpoint_text: String;
        let server_options: NetworkOperationOptions;
        let server_task;
        let scope: TerraneTaskScope;
        let input: ByteReader;
        let input_task;
        let input_child: TerraneScopedTask<ReadResult>;
        let child: TerraneScopedTask<()>;
        let destination: SocketResult;
        let client_options: NetworkOperationOptions;
        let connected: StreamResult;
        let client: TcpStream;
        let sent: IoResult;
        let response: IoResult;
        let outcome: TerraneTaskOutcome<()>;
        let input_outcome: TerraneTaskOutcome<ReadResult>;
        loopback = ip_address_from_string(String::from("127.0.0.1"));
        bind_address = socket_address_from_ip(
            loopback.value,
            terrane_int_support::Int::from(0_i128),
        );
        bound = bind_tcp(bind_address.value);
        listener = bound.value;
        endpoint_text = listener.local_address.clone();
        server_options = NetworkOperationOptions::terrane_construct(
            None,
            NetworkCancellationToken::terrane_construct(),
        );
        server_task = serve(listener, server_options);
        scope = TerraneTaskScope::new(None);
        input = stdin();
        input_task = read_input(input);
        input_child = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = input_task;
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
        child = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = server_task;
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
        destination = socket_address_from_string(endpoint_text);
        client_options = NetworkOperationOptions::terrane_construct(
            None,
            NetworkCancellationToken::terrane_construct(),
        );
        connected = __terrane_await(
                connect_tcp(destination.value, client_options.clone()),
            )
            .await;
        client = connected.value;
        sent = __terrane_await(
                (&client)
                    .write(
                        Vec::from([116, 101, 114, 114, 97, 110, 101]),
                        client_options.clone(),
                    ),
            )
            .await;
        response = __terrane_await(
                (&client).read(terrane_int_support::Int::from(5_i128), client_options),
            )
            .await;
        outcome = __terrane_await(scope.join(child)).await;
        println!("{}", terrane_scalar_support::scalar_text(&sent.completed));
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&response
            .data, terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:43:13-43:39 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&outcome.completed));
        input_outcome = __terrane_await(scope.join(input_child)).await;
        if !input_outcome.completed {
            println!(
                "{}",
                terrane_scalar_support::scalar_text(&String::from("stdin task did not complete"))
            );
        }
        client.close();
    });
}
