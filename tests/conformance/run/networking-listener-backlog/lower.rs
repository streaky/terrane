// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
async fn echo(listener: TcpListener) {
    let accepted: StreamResult;
    let stream: TcpStream;
    let received: ReadResult;
    let written: WriteResult;
    accepted = __terrane_await((&listener).accept(None)).await;
    if accepted.failed {
        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
    }
    stream = accepted.value;
    received = __terrane_await(
            (&stream).read_exact_async(terrane_int_support::Int::from(5_i128), None),
        )
        .await;
    if received.failed {
        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
    }
    written = __terrane_await((&stream).write_all_async(received.data.clone(), None))
        .await;
    if written.failed {
        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
    }
    stream.close();
    listener.close();
    return ();
}
async fn exchange(content: Vec<u8>, options: Option<TcpListenerOptions>) -> Vec<u8> {
    let address: SocketResult;
    let bound: ListenerResult;
    let listener: TcpListener;
    let destination: SocketResult;
    let scope: TerraneTaskScope;
    let server: TerraneScopedTask<()>;
    let connected: StreamResult;
    let stream: TcpStream;
    let written: WriteResult;
    let received: ReadResult;
    let outcome: TerraneTaskOutcome<()>;
    address = socket_address_from_string(String::from("127.0.0.1:0"));
    bound = bind_tcp(address.value.clone(), options);
    if bound.failed {
        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
    }
    listener = bound.value;
    destination = socket_address_from_string(listener.local_address.clone());
    scope = TerraneTaskScope::new(None);
    server = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.cancellation();
        let __terrane_deadline = __terrane_scope.deadline;
        let __terrane_spawned_task = echo(listener);
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
    connected = __terrane_await(connect_tcp(destination.value.clone(), None, None))
        .await;
    if connected.failed {
        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
    }
    stream = connected.value;
    written = __terrane_await((&stream).write_all_async(content.clone(), None)).await;
    received = __terrane_await(
            (&stream)
                .read_exact_async(
                    terrane_int_support::Int::from(content.len() as i128),
                    None,
                ),
        )
        .await;
    stream.close();
    outcome = __terrane_await(scope.join(server)).await;
    if written.failed || received.failed || !outcome.completed {
        exit(make_exit_status(terrane_int_support::Int::from(1_i128)));
    }
    return received.data.clone();
}
fn main() {
    __terrane_run(async move {
        let first: Vec<u8>;
        let mut settings: TcpListenerOptions;
        let second: Vec<u8>;
        let address: SocketResult;
        let invalid_negative: ListenerResult;
        let invalid_large: ListenerResult;
        first = __terrane_await(
                exchange(Vec::<u8>::from([102, 105, 114, 115, 116]), None),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0
            = terrane_string_support::Encoding::Utf8;
            __terrane_raised(terrane_string_support::decode(&first,
            __terrane_string_argument_0), 0 /* terrane-site: case.trn:45:13-45:31 */)
            })
        );
        settings = TcpListenerOptions::terrane_construct(
            terrane_int_support::Int::from(16_i128),
            false,
            false,
            terrane_int_support::Int::from(-1_i128),
        );
        {
            let __terrane_value = terrane_int_support::Int::from(-1_i128);
            settings.backlog = __terrane_value;
        }
        second = __terrane_await(
                exchange(Vec::<u8>::from([114, 101, 115, 101, 116]), Some(settings)),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&{ let __terrane_string_argument_0
            = terrane_string_support::Encoding::Utf8;
            __terrane_raised(terrane_string_support::decode(&second,
            __terrane_string_argument_0), 1 /* terrane-site: case.trn:49:13-49:32 */)
            })
        );
        address = socket_address_from_string(String::from("127.0.0.1:0"));
        invalid_negative = bind_tcp(
            address.value.clone(),
            Some(
                TcpListenerOptions::terrane_construct(
                    terrane_int_support::Int::from(-2_i128),
                    false,
                    false,
                    terrane_int_support::Int::from(-1_i128),
                ),
            ),
        );
        invalid_large = bind_tcp(
            address.value.clone(),
            Some(
                TcpListenerOptions::terrane_construct(
                    terrane_int_support::Int::from(2147483648_i128),
                    false,
                    false,
                    terrane_int_support::Int::from(-1_i128),
                ),
            ),
        );
        println!("{}", terrane_scalar_support::scalar_text(&invalid_negative.failed));
        println!("{}", terrane_scalar_support::scalar_text(&invalid_large.failed));
    });
}
