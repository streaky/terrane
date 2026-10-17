// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
async fn serve(listener: TcpListener) {
    let accepted: StreamResult;
    let stream: TcpStream;
    let buffer_result: ReadBufferResult;
    let request_buffer: OwnedReadBuffer;
    let request: BufferedReadResult;
    let request_bytes: BufferBytesResult;
    let written: WriteResult;
    accepted = __terrane_await((&listener).accept(None)).await;
    stream = accepted.value;
    buffer_result = make_read_buffer(terrane_int_support::Int::from(16_i128));
    request_buffer = buffer_result.value;
    request = __terrane_await(
            (&stream)
                .read_into_async(
                    request_buffer,
                    terrane_int_support::Int::from(7_i128),
                    None,
                ),
        )
        .await;
    request_bytes = request
        .buffer
        .as_ref()
        .expect("required field initialized")
        .filled_bytes();
    if request_bytes.value == Vec::<u8>::from([116, 101, 114, 114, 97, 110, 101]) {
        written = __terrane_await(
                (&stream)
                    .write_all_async(Vec::<u8>::from([114, 101, 112, 108, 121]), None),
            )
            .await;
        if !written.failed {
            stream.shutdown(String::from("both"));
        }
    }
    stream.close();
    return ();
}
async fn read_input(input: ByteReader) -> BufferedReadResult {
    let allocated: ReadBufferResult;
    let buffer: OwnedReadBuffer;
    allocated = make_read_buffer(terrane_int_support::Int::from(8_i128));
    buffer = allocated.value;
    return __terrane_await(
            (&input)
                .read_into_async(buffer, terrane_int_support::Int::from(1_i128), None),
        )
        .await;
}
fn main() {
    __terrane_run(async move {
        let loopback: IpResult;
        let bind_address: SocketResult;
        let server_settings: TcpListenerOptions;
        let bound: ListenerResult;
        let listener: TcpListener;
        let endpoint_text: String;
        let server_task;
        let scope: TerraneTaskScope;
        let input: ByteReader;
        let input_task;
        let input_child: TerraneScopedTask<BufferedReadResult>;
        let child: TerraneScopedTask<()>;
        let destination: SocketResult;
        let client_settings: TcpOptions;
        let connected: StreamResult;
        let client: TcpStream;
        let sent: WriteResult;
        let response: ReadResult;
        let outcome: TerraneTaskOutcome<()>;
        let input_outcome: TerraneTaskOutcome<BufferedReadResult>;
        let empty: ReadResult;
        let short: ReadResult;
        loopback = ip_address_from_string(String::from("127.0.0.1"));
        bind_address = socket_address_from_ip(
            loopback.value.clone(),
            terrane_int_support::Int::from(0_i128),
        );
        server_settings = TcpListenerOptions::terrane_construct(
            terrane_int_support::Int::from(32_i128),
            true,
            true,
            terrane_int_support::Int::from(64_i128),
        );
        bound = bind_tcp(bind_address.value.clone(), Some(server_settings));
        listener = bound.value;
        endpoint_text = listener.local_address.clone();
        server_task = serve(listener);
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
        client_settings = TcpOptions::terrane_construct(
            false,
            terrane_int_support::Int::from(64_i128),
        );
        connected = __terrane_await(
                connect_tcp(destination.value.clone(), None, Some(client_settings)),
            )
            .await;
        client = connected.value;
        sent = __terrane_await(
                (&client)
                    .write_all_async(
                        Vec::<u8>::from([116, 101, 114, 114, 97, 110, 101]),
                        None,
                    ),
            )
            .await;
        response = __terrane_await(
                (&client).read_exact_async(terrane_int_support::Int::from(5_i128), None),
            )
            .await;
        outcome = __terrane_await(scope.join(child)).await;
        println!("{}", terrane_scalar_support::scalar_text(&sent.completed));
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&__terrane_raised(terrane_string_support::decode(&response
            .data, terrane_string_support::Encoding::Utf8), 0 /* terrane-site: case.trn:48:13-48:39 */))
        );
        println!("{}", terrane_scalar_support::scalar_text(&outcome.completed));
        input_outcome = __terrane_await(scope.join(input_child)).await;
        if !input_outcome.completed {
            println!("{}", String::from("stdin task did not complete"));
        }
        empty = __terrane_await(
                (&client).read_exact_async(terrane_int_support::Int::from(0_i128), None),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(! empty.failed &&! empty.end
            &&empty.completed.clone() == terrane_int_support::Int::from(0_i128) &&empty
            .data == Vec:: < u8 > ::from([])))
        );
        short = __terrane_await(
                (&client).read_exact_async(terrane_int_support::Int::from(1_i128), None),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(short.failed &&short.end &&short
            .completed.clone() == terrane_int_support::Int::from(0_i128) &&short.data ==
            Vec:: < u8 > ::from([])))
        );
        client.close();
    });
}
