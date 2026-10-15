// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: process-signals-surface
async fn observe(
    mut subscription: ProcessSignalSubscription,
) -> Result<(), TerraneError> {
    let event: ProcessSignalEvent;
    let closed: ();
    event = __terrane_traced_err(
        __terrane_await((&mut subscription).next()).await,
        0 /* terrane-site: case.trn:7:40-7:58 */,
    )?;
    println!(
        "{}{}{}{}", terrane_scalar_support::scalar_text(&event.signal.name),
        terrane_scalar_support::scalar_text(&event.count),
        terrane_scalar_support::scalar_text(&event.sequence),
        terrane_scalar_support::scalar_text(&event.overflowed)
    );
    closed = __terrane_traced_err(
        subscription.terrane_separate().close(),
        1 /* terrane-site: case.trn:9:14-9:33 */,
    )?;
    let _ = &closed;
    return Ok(());
}
fn main() {
    __terrane_run(async move {
        let selected: terrane_collection_support::Set<ProcessSignal>;
        let subscription: ProcessSignalSubscription;
        let ignored: ();
        selected = terrane_collection_support::Set::<
            ProcessSignal,
        >::new(
            vec![
                ProcessSignal::terrane_static_interrupt(),
                ProcessSignal::terrane_static_terminate(),
                ProcessSignal::terrane_static_hangup(),
                ProcessSignal::terrane_static_quit()
            ],
        );
        subscription = __terrane_traced(
            process_signals(selected),
            2 /* terrane-site: case.trn:14:20-14:45 */,
        );
        ignored = __terrane_traced(
            __terrane_await(observe(subscription.terrane_separate())).await,
            3 /* terrane-site: case.trn:15:21-15:42 */,
        );
        let _ = &ignored;
    });
}
