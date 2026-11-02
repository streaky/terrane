// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    let input: Allowed;
    let answer: bool;
    input = __terrane_raised(allowed(), 0 /* terrane-site: src/main.trn:4:13-4:21 */);
    answer = __terrane_raised(
        match std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| witness::execute(input)),
        ) {
            Ok(value) => Ok(value),
            Err(payload) => {
                Err(
                    crate::__terrane_dependency_panic(
                        payload,
                        "witness",
                        "witness::execute",
                    ),
                )
            }
        },
        1 /* terrane-site: src/main.trn:5:14-5:28 */,
    );
    println!("{}", terrane_scalar_support::scalar_text(&answer));
}
