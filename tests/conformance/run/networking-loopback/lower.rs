// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let loopback: IpResult;
        let address: SocketResult;
        let bound: UdpResult;
        let socket: UdpSocket;
        let destination: SocketResult;
        let configured: NetworkOperationResult;
        let cancellation: NetworkCancellationToken;
        let options: NetworkOperationOptions;
        let sent: IoResult;
        let received: IoResult;
        let bytes_output: ByteWriter;
        let output: TextWriter;
        let written: WriteResult;
        loopback = ip_address_from_string(String::from("127.0.0.1"));
        address = socket_address_from_ip(
            loopback.value,
            terrane_int_support::Int::from(0_i128),
        );
        bound = bind_udp(address.value);
        socket = bound.value;
        destination = socket_address_from_string(socket.local_address.clone());
        configured = socket
            .configure(
                UdpOptions::terrane_construct(
                    false,
                    terrane_int_support::Int::from(32_i128),
                ),
            );
        cancellation = NetworkCancellationToken::terrane_construct();
        options = NetworkOperationOptions::terrane_construct(
            Some(
                Clock::terrane_static_deadline(
                    __terrane_traced(
                        Duration::terrane_static_seconds(
                            terrane_int_support::Int::from(1_i128),
                        ),
                        0 /* terrane-site: case.trn:16:70-16:90 */,
                    ),
                ),
            ),
            cancellation,
        );
        sent = __terrane_await(
                (&socket)
                    .send_to(
                        Vec::from([116, 101, 114, 114, 97, 110, 101]),
                        destination.value,
                        options.clone(),
                    ),
            )
            .await;
        received = __terrane_await(
                (&socket).receive_from(terrane_int_support::Int::from(32_i128), options),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&! configured.failed));
        println!("{}", terrane_scalar_support::scalar_text(&sent.completed));
        bytes_output = stdout();
        output = bytes_output.text(terrane_string_support::Encoding::Utf8);
        written = output
            .line(
                __terrane_raised(
                    terrane_string_support::decode(
                        &received.data,
                        terrane_string_support::Encoding::Utf8,
                    ),
                    1 /* terrane-site: case.trn:23:29-23:55 */,
                ),
            );
        if written.failed {
            println!("{}", terrane_scalar_support::scalar_text(&written.message));
        }
        println!("{}", terrane_scalar_support::scalar_text(&! received.truncated));
        socket.close();
    });
}
