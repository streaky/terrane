// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let octets: Ipv4Addr;
    let address: IpAddr;
    let result: bool;
    octets = {
        let __terrane_conversion_input_0 = terrane_int_support::Int::from(127_i128);
        let __terrane_conversion_input_1 = terrane_int_support::Int::from(0_i128);
        let __terrane_conversion_input_2 = terrane_int_support::Int::from(0_i128);
        let __terrane_conversion_input_3 = terrane_int_support::Int::from(1_i128);
        __terrane_raised(
            terrane_static_trn_4970763441646472_new(
                __terrane_conversion_input_0,
                __terrane_conversion_input_1,
                __terrane_conversion_input_2,
                __terrane_conversion_input_3,
            ),
            0 /* terrane-site: src/main.trn:6:23-6:50 */,
        )
    };
    address = {
        let __terrane_conversion_input_0 = octets;
        __terrane_raised(
            || -> Result<_, crate::TerraneForeignError> {
                {
                    let value = match std::panic::catch_unwind(
                        std::panic::AssertUnwindSafe(|| -> Result<
                            _,
                            crate::TerraneForeignError,
                        > { Ok(std::net::IpAddr::V4(__terrane_conversion_input_0)) }),
                    ) {
                        Ok(result) => result?,
                        Err(payload) => {
                            return Err(
                                crate::__terrane_dependency_panic(
                                    payload,
                                    "std",
                                    "std::net::IpAddr::V4",
                                ),
                            );
                        }
                    };
                    Ok::<_, crate::TerraneForeignError>(value)
                }
            }(),
            1 /* terrane-site: src/main.trn:7:22-7:40 */,
        )
    };
    result = {
        let __terrane_conversion_input_0 = address;
        __terrane_raised(
            is_loopback(__terrane_conversion_input_0),
            2 /* terrane-site: src/main.trn:8:19-8:39 */,
        )
    };
    let _ = &result;
}
