// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let point: terrane_point_witness::Point<f32>;
    let snapped: terrane_point_witness::Point<u32>;
    point = __terrane_raised(
        terrane_static_trn_506f696e74_new(3.4_f32, 4.6_f32),
        0 /* terrane-site: src/main.trn:6:13-6:33 */,
    );
    snapped = __terrane_raised(
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| point.snap())) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_point_witness",
                        "terrane_point_witness::Point<T>::snap",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:7:15-7:26 */,
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(| | (&snapped).total())) {
        Ok(value) => Ok(terrane_int_support::Int::from_u128(value as u128)), Err(payload)
        => Err(crate ::__terrane_dependency_panic(payload, "terrane_point_witness",
        "terrane_point_witness::Point<T>::total")) }, 2 /* terrane-site: src/main.trn:8:13-8:27 */))
    );
}
