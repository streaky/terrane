// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: async-byte-protocol
async fn consume(reader: AsyncByteReader) {
    let first: ReadResult;
    let tail: ReadResult;
    first = __terrane_await(
            (&reader).read_exact_async(terrane_int_support::Int::from(2_i128), None),
        )
        .await;
    tail = __terrane_await(
            (&reader).read_exact_async(terrane_int_support::Int::from(4_i128), None),
        )
        .await;
    println!(
        "{}", terrane_scalar_support::scalar_text(&(first.completed.clone() ==
        terrane_int_support::Int::from(2_i128) &&! first.failed))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(tail.completed.clone() ==
        terrane_int_support::Int::from(1_i128) &&tail.end &&tail.failed))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(first.data == Vec:: < u8 >
        ::from([97, 98])))
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(tail.data == Vec:: < u8 >
        ::from([99])))
    );
    reader.close();
    return ();
}
async fn serve(listener: TcpListener) {
    let accepted: StreamResult;
    let stream: TcpStream;
    accepted = __terrane_await((&listener).accept(None)).await;
    stream = accepted.value;
    {
        let _: WriteResult = __terrane_await(
                (&stream).write_all_async(Vec::<u8>::from([97, 98, 99]), None),
            )
            .await;
    }
    stream.shutdown(String::from("write"));
    stream.close();
    listener.close();
    return ();
}
fn main() {
    __terrane_run(async move {
        let parsed: SocketResult;
        let bound: ListenerResult;
        let address: SocketResult;
        let scope: TerraneTaskScope;
        let server: TerraneScopedTask<()>;
        let connected: StreamResult;
        let outcome: TerraneTaskOutcome<()>;
        parsed = socket_address_from_string(String::from("127.0.0.1:0"));
        bound = bind_tcp(parsed.value.clone(), None);
        address = socket_address_from_string(bound.value.local_address.clone());
        scope = TerraneTaskScope::new(None);
        server = {
            let __terrane_scope = scope.clone();
            let __terrane_cancel = __terrane_scope.cancellation();
            let __terrane_deadline = __terrane_scope.deadline;
            let __terrane_spawned_task = serve(bound.value);
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
        connected = __terrane_await(connect_tcp(address.value.clone(), None, None))
            .await;
        __terrane_await(consume(<AsyncByteReader>::from(connected.value))).await;
        outcome = __terrane_await(scope.join(server)).await;
        println!("{}", terrane_scalar_support::scalar_text(&outcome.completed));
    });
}
