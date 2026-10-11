// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let left: Left;
    let right: Right;
    let returned: Left;
    left = __terrane_raised(
        terrane_static_trn_4c656674_new(),
        0 /* terrane-site: src/main.trn:7:10-7:20 */,
    );
    right = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&left).poison()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_member_graph_witness",
                        "terrane_member_graph_witness::left::Left::poison",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:8:17-8:29 */,
    );
    returned = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| (&right).poison()),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_member_graph_witness",
                        "terrane_member_graph_witness::right::Right::poison",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:9:14-9:27 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&returned).value())) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload,
        "terrane_member_graph_witness",
        "terrane_member_graph_witness::left::Left::value")) }, 3 /* terrane-site: src/main.trn:10:11-10:26 */))
    );
}
