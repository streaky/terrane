// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let octets: Ipv4Addr;
    let address: IpAddr;
    let result: bool;
    octets = __terrane_raised(
        terrane_static_trn_4970763441646472_new(
            terrane_int_support::Int::from(127_i128),
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(1_i128),
        ),
        0 /* terrane-site: src/main.trn:6:23-6:50 */,
    );
    address = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| std::net::IpAddr::V4(octets)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "std",
                        "std::net::IpAddr::V4",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:7:22-7:40 */,
    );
    result = __terrane_raised(
        is_loopback(address),
        2 /* terrane-site: src/main.trn:8:19-8:39 */,
    );
    let _ = &result;
}
