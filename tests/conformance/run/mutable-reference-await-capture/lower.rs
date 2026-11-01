// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let mut value: terrane_int_support::Int;
        let borrowed: &mut terrane_int_support::Int;
        let callback: TerraneBorrowedConsumingCallable<
            '_,
            (),
            std::pin::Pin<Box<dyn Future<Output = ()> + Send + '_>>,
        >;
        value = terrane_int_support::Int::from(0_i128);
        __terrane_await(reset(&mut value)).await;
        println!("{}", terrane_scalar_support::scalar_text(&value));
        borrowed = &mut value;
        callback = {
            let borrowed = borrowed;
            TerraneBorrowedConsumingCallable::new(move |
                (): (),
            | -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + '_>> {
                let borrowed = borrowed;
                Box::pin(async move {
                    {
                        let _: () = __terrane_await(
                                terrane_time_sleep_after(
                                    __terrane_traced(
                                            Duration::terrane_static_milliseconds(
                                                terrane_int_support::Int::from(1_i128),
                                            ),
                                            0 /* terrane-site: src/main.trn:10:30-10:55 */,
                                        )
                                        .total_nanoseconds
                                        .clone(),
                                ),
                            )
                            .await;
                    }
                    *borrowed = terrane_int_support::Int::from(9_i128);
                    ()
                })
            })
        };
        __terrane_await(callback.call(())).await;
        println!("{}", terrane_scalar_support::scalar_text(&value));
    });
}
// Source: src/worker/worker.trn
// Namespace: worker
async fn reset(value: &mut terrane_int_support::Int) {
    {
        let _: () = __terrane_await(
                terrane_time_sleep_after(
                    __terrane_traced(
                            Duration::terrane_static_milliseconds(
                                terrane_int_support::Int::from(1_i128),
                            ),
                            1 /* terrane-site: src/worker/worker.trn:4:28-4:53 */,
                        )
                        .total_nanoseconds
                        .clone(),
                ),
            )
            .await;
    }
    *value = terrane_int_support::Int::from(8_i128);
}
