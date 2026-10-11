// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let parsed: IpResult;
        let loopback: IpResult;
        let invalid_ip: IpResult;
        let scoped_ip: IpResult;
        let international: NetworkHostNameResult;
        let invalid_host: NetworkHostNameResult;
        let port: i64;
        let timeout: Deadline;
        let cancellation: NetworkCancellationToken;
        let cancelled: NetworkOperationOptions;
        let host: NetworkHostNameResult;
        let lookup: DnsResult;
        parsed = ip_address_from_string(String::from("2001:0db8:0:0:0:0:0:1"));
        println!("{}", terrane_scalar_support::scalar_text(&parsed.value.value));
        println!("{}", terrane_scalar_support::scalar_text(&parsed.value.version));
        println!("{}", terrane_scalar_support::scalar_text(&parsed.value.is_loopback));
        loopback = ip_address_from_string(String::from("127.0.0.1"));
        println!("{}", terrane_scalar_support::scalar_text(&loopback.value.version));
        println!("{}", terrane_scalar_support::scalar_text(&loopback.value.is_loopback));
        invalid_ip = ip_address_from_string(String::from("999.1.1.1"));
        println!("{}", terrane_scalar_support::scalar_text(&invalid_ip.failed));
        scoped_ip = ip_address_from_string(String::from("fe80::1%eth0"));
        println!("{}", terrane_scalar_support::scalar_text(&scoped_ip.failed));
        international = parse_host_name(String::from("bücher.example"));
        println!("{}", terrane_scalar_support::scalar_text(&international.value.value));
        invalid_host = parse_host_name(String::from("contains space.example"));
        println!("{}", terrane_scalar_support::scalar_text(&invalid_host.failed));
        port = 443;
        timeout = Clock::terrane_static_deadline(
            __terrane_traced(
                Duration::terrane_static_seconds(terrane_int_support::Int::from(1_i128)),
                0 /* terrane-site: case.trn:23:33-23:53 */,
            ),
        );
        cancellation = NetworkCancellationToken::terrane_construct();
        network_cancel_operation(cancellation.clone());
        cancelled = NetworkOperationOptions::terrane_construct(
            Some(timeout),
            cancellation,
        );
        host = parse_host_name(String::from("localhost"));
        lookup = __terrane_await(
                lookup_dns(
                    host.value,
                    terrane_int_support::Int::from(port as i128),
                    cancelled,
                ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&lookup.failed));
        println!("{}", terrane_scalar_support::scalar_text(&lookup.message));
    });
}
