// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: process-signals-delivery
fn main() {
    __terrane_run(async move {
        let selected: terrane_collection_support::Set<ProcessSignal>;
        let mut subscription: ProcessSignalSubscription;
        let before: MonotonicInstant;
        let first: ProcessSignalEvent;
        let second: ProcessSignalEvent;
        selected = terrane_collection_support::Set::<
            ProcessSignal,
        >::new(
            vec![
                ProcessSignal::terrane_static_interrupt(),
                ProcessSignal::terrane_static_terminate()
            ],
        );
        subscription = __terrane_traced(
            process_signals(selected),
            0 /* terrane-site: case.trn:9:20-9:45 */,
        );
        before = Clock::terrane_static_monotonic();
        println!("{}", String::from("ready"));
        first = __terrane_traced(
            __terrane_await((&mut subscription).next()).await,
            1 /* terrane-site: case.trn:12:19-12:37 */,
        );
        second = __terrane_traced(
            __terrane_await((&mut subscription).next()).await,
            2 /* terrane-site: case.trn:13:20-13:38 */,
        );
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&first.signal.name),
            terrane_scalar_support::scalar_text(&first.count),
            terrane_scalar_support::scalar_text(&(first.sequence.clone() >
            terrane_int_support::Int::from(0_i128))),
            terrane_scalar_support::scalar_text(&(__terrane_traced(before
            .duration_until(&first.observed_at), 3 /* terrane-site: case.trn:14:65-14:109 */).total_nanoseconds.clone() >=
            terrane_int_support::Int::from(0_i128)))
        );
        println!(
            "{}{}{}{}", terrane_scalar_support::scalar_text(&second.signal.name),
            terrane_scalar_support::scalar_text(&second.count),
            terrane_scalar_support::scalar_text(&(second.sequence.clone() > first
            .sequence.clone())),
            terrane_scalar_support::scalar_text(&(__terrane_traced(first.observed_at
            .duration_until(&second.observed_at), 4 /* terrane-site: case.trn:15:81-15:137 */).total_nanoseconds.clone() >=
            terrane_int_support::Int::from(0_i128)))
        );
        __terrane_traced(
            subscription.terrane_separate().close(),
            5 /* terrane-site: case.trn:16:5-16:24 */,
        );
    });
}
