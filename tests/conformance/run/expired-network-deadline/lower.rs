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
        let expired_options: NetworkOperationOptions;
        let expired: StreamResult;
        loopback = ip_address_from_string(String::from("127.0.0.1"));
        bind_address = socket_address_from_ip(
            loopback.value,
            terrane_int_support::Int::from(0_i128),
        );
        bound = bind_tcp(bind_address.value, None);
        listener = bound.value;
        expired_options = NetworkOperationOptions::terrane_construct(
            Some(
                Clock::terrane_static_deadline(
                    __terrane_traced(
                        Duration::terrane_static_nanoseconds(
                            terrane_int_support::Int::from(0_i128),
                        ),
                        0 /* terrane-site: case.trn:14:78-14:102 */,
                    ),
                ),
            ),
            None,
        );
        expired = __terrane_await((&listener).accept(Some(expired_options))).await;
        println!("{}", terrane_scalar_support::scalar_text(&expired.failed));
        println!("{}", terrane_scalar_support::scalar_text(&expired.deadline_exceeded));
        listener.close();
    });
}
