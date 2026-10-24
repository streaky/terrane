// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let initial: Border;
    let radius: Radius;
    let first: Border;
    let eight: f32;
    let second: Border;
    initial = __terrane_raised(
        make_border(),
        0 /* terrane-site: src/main.trn:6:22-6:34 */,
    );
    radius = terrane_native_bound_witness::Radius {
        value: 4.0,
    };
    first = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| initial.rounded(radius)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_native_bound_witness",
                        "terrane_native_bound_witness::Border::rounded",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:8:20-8:43 */,
    );
    eight = 8.0_f32;
    second = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| first.rounded(eight)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "terrane_native_bound_witness",
                        "terrane_native_bound_witness::Border::rounded",
                    ),
                )
            }
        },
        2 /* terrane-site: src/main.trn:10:21-10:41 */,
    );
    println!(
        "{}",
        terrane_scalar_support::scalar_text(&__terrane_raised(radius_value(&second),
        3 /* terrane-site: src/main.trn:11:13-11:33 */))
    );
}
