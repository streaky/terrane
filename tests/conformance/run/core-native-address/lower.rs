// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: app
fn make_address() -> NativeSocketAddress {
    return std::net::SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
        1234,
    );
}
fn port(address: &NativeSocketAddress) -> u16 {
    let _ = &address;
    return address.port();
}
fn main() {
    let address: NativeSocketAddress;
    address = make_address();
    println!("{}", terrane_scalar_support::scalar_text(&port(&address)));
}
