// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let loopback: IpResult;
        let bind_address: SocketResult;
        let bound: ListenerResult;
        let listener: TcpListener;
        let deadline_options: NetworkOperationOptions;
        let expired: StreamResult;
        let cancellation: NetworkCancellationToken;
        let cancelled_options: NetworkOperationOptions;
        let cancelled: StreamResult;
        loopback = ip_address_from_string(String::from("127.0.0.1"));
        bind_address = socket_address_from_ip(
            loopback.value,
            terrane_int_support::Int::from(0_i128),
        );
        bound = bind_tcp(bind_address.value, None);
        listener = bound.value;
        deadline_options = NetworkOperationOptions::terrane_construct(
            Some(
                Clock::terrane_static_deadline(
                    __terrane_traced(
                        Duration::terrane_static_nanoseconds(
                            terrane_int_support::Int::from(1_i128),
                        ),
                        0 /* terrane-site: case.trn:14:79-14:103 */,
                    ),
                ),
            ),
            Some(NetworkCancellationToken::terrane_construct()),
        );
        expired = __terrane_await((&listener).accept(Some(deadline_options))).await;
        println!("{}", terrane_scalar_support::scalar_text(&expired.failed));
        println!("{}", terrane_scalar_support::scalar_text(&expired.deadline_exceeded));
        cancellation = NetworkCancellationToken::terrane_construct();
        network_cancel_operation(cancellation.clone());
        cancelled_options = NetworkOperationOptions::terrane_construct(
            Some(
                Clock::terrane_static_deadline(
                    __terrane_traced(
                        Duration::terrane_static_seconds(
                            terrane_int_support::Int::from(1_i128),
                        ),
                        1 /* terrane-site: case.trn:21:80-21:100 */,
                    ),
                ),
            ),
            Some(cancellation),
        );
        cancelled = __terrane_await((&listener).accept(Some(cancelled_options))).await;
        println!("{}", terrane_scalar_support::scalar_text(&cancelled.failed));
        println!("{}", terrane_scalar_support::scalar_text(&cancelled.message));
        listener.close();
    });
}
