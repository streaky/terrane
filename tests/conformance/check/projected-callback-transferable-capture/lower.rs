// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let marker: Marker;
        let callback: std::sync::Arc<
            dyn Fn(
                terrane_int_support::Int,
            ) -> std::pin::Pin<
                    Box<dyn Future<Output = terrane_int_support::Int> + Send>,
                > + Send + Sync,
        >;
        marker = __terrane_raised(
            make_marker(terrane_int_support::Int::from(2_i128)),
            0 /* terrane-site: src/main.trn:4:14-4:28 */,
        );
        callback = {
            let marker = marker.clone();
            std::sync::Arc::new(move |
                input: terrane_int_support::Int,
            | -> std::pin::Pin<
                Box<dyn Future<Output = terrane_int_support::Int> + Send>,
            > {
                let marker = marker.clone();
                Box::pin(async move {
                    let observed: terrane_int_support::Int;
                    observed = __terrane_raised(
                        match std::panic::catch_unwind(
                            std::panic::AssertUnwindSafe(|| (&marker).value()),
                        ) {
                            Ok(value) => {
                                Ok(terrane_int_support::Int::from(i128::from(value)))
                            }
                            Err(payload) => {
                                Err(
                                    crate::__terrane_dependency_panic(
                                        payload,
                                        "terrane_callback_witness",
                                        "terrane_callback_witness::Marker::value",
                                    ),
                                )
                            }
                        },
                        1 /* terrane-site: src/main.trn:6:25-6:38 */,
                    );
                    let _ = &observed;
                    return input.clone();
                })
            })
        };
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_traced(__terrane_await({
            let __terrane_future =
            apply_async_concurrently(terrane_int_support::Int::from(1_i128), callback
            .clone()); async move { __terrane_raised_err(__terrane_future. await,
            2 /* terrane-site: src/main.trn:8:18-8:55 */) } }). await,
            2 /* terrane-site: src/main.trn:8:18-8:55 */))
        );
    });
}
