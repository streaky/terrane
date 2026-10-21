// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let native_int: witness::Cell<i64>;
        native_int = __terrane_raised(
            match std::panic::catch_unwind(
                std::panic::AssertUnwindSafe(|| witness::make::<i64>()),
            ) {
                Ok(value) => Ok(value),
                Err(payload) => {
                    Err(
                        crate::__terrane_dependency_panic(
                            payload,
                            "witness",
                            "witness::make",
                        ),
                    )
                }
            },
            0 /* terrane-site: src/main.trn:6:32-6:46 */,
        );
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(match
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(| |
            witness::consume(native_int))) { Ok(value) => Ok(value), Err(payload) =>
            Err(crate ::__terrane_dependency_panic(payload, "witness",
            "witness::consume")) }, 1 /* terrane-site: src/main.trn:7:13-7:32 */))
        );
    });
}
