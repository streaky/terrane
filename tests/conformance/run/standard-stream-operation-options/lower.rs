// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: standard-stream-operation-options
fn main() {
    __terrane_run(async move {
        let reader: ByteReader;
        let writer: ByteWriter;
        let cancelled: NetworkCancellationToken;
        let cancelled_options: NetworkOperationOptions;
        let cancelled_read: ReadResult;
        let expired: Deadline;
        let expired_options: NetworkOperationOptions;
        let expired_read: ReadResult;
        let live_options: NetworkOperationOptions;
        let live_read: ReadResult;
        let output: WriteResult;
        reader = stdin();
        writer = stdout();
        cancelled = NetworkCancellationToken::terrane_construct();
        network_cancel_operation(cancelled.clone());
        cancelled_options = NetworkOperationOptions::terrane_construct(
            None,
            Some(cancelled),
        );
        cancelled_read = __terrane_await(
                (&reader)
                    .read_async(
                        terrane_int_support::Int::from(1_i128),
                        Some(cancelled_options),
                    ),
            )
            .await;
        println!("{}", terrane_scalar_support::scalar_text(&cancelled_read.failed));
        expired = Clock::terrane_static_deadline(
            __terrane_traced(
                Duration::terrane_static_nanoseconds(
                    terrane_int_support::Int::from(0_i128),
                ),
                0 /* terrane-site: case.trn:16:33-16:57 */,
            ),
        );
        expired_options = NetworkOperationOptions::terrane_construct(
            Some(expired),
            None,
        );
        expired_read = __terrane_await(
                (&reader)
                    .read_async(
                        terrane_int_support::Int::from(1_i128),
                        Some(expired_options),
                    ),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(expired_read.failed
            &&expired_read.deadline_exceeded))
        );
        live_options = NetworkOperationOptions::terrane_construct(
            Some(
                Clock::terrane_static_deadline(
                    __terrane_traced(
                        Duration::terrane_static_seconds(
                            terrane_int_support::Int::from(5_i128),
                        ),
                        1 /* terrane-site: case.trn:21:75-21:95 */,
                    ),
                ),
            ),
            Some(NetworkCancellationToken::terrane_construct()),
        );
        live_read = __terrane_await(
                (&reader)
                    .read_async(
                        terrane_int_support::Int::from(1_i128),
                        Some(live_options.clone()),
                    ),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(! live_read.failed &&live_read
            .completed.clone() == terrane_int_support::Int::from(1_i128) &&live_read.data
            == Vec:: < u8 > ::from([122])))
        );
        output = __terrane_await(
                (&writer)
                    .write_all_async(
                        Vec::<u8>::from([97, 99, 99, 101, 112, 116, 101, 100, 10]),
                        Some(live_options),
                    ),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(! output.failed &&output
            .completed.clone() == terrane_int_support::Int::from(9_i128)))
        );
    });
}
