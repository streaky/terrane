// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: standard-buffer-reuse
fn main() {
    __terrane_run(async move {
        let reader: ByteReader;
        let allocated: ReadBufferResult;
        let expired: Deadline;
        let options: NetworkOperationOptions;
        let mut rejected: BufferedReadResult;
        let mut first: BufferedReadResult;
        let first_bytes: BufferBytesResult;
        let mut second: BufferedReadResult;
        let all_bytes: BufferBytesResult;
        let mut zero: BufferedReadResult;
        let mut buffer: OwnedReadBuffer;
        let cleared: StreamOperationResult;
        let empty: BufferBytesResult;
        reader = stdin();
        allocated = make_read_buffer(terrane_int_support::Int::from(4_i128));
        expired = Clock::terrane_static_deadline(
            __terrane_traced(
                Duration::terrane_static_nanoseconds(
                    terrane_int_support::Int::from(0_i128),
                ),
                0 /* terrane-site: case.trn:9:33-9:57 */,
            ),
        );
        options = NetworkOperationOptions::terrane_construct(Some(expired), None);
        rejected = __terrane_await(
                (&reader)
                    .read_into_async(
                        allocated.value,
                        terrane_int_support::Int::from(2_i128),
                        Some(options),
                    ),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(rejected.failed &&rejected
            .deadline_exceeded &&rejected.completed.clone() ==
            terrane_int_support::Int::from(0_i128)))
        );
        first = __terrane_await(
                (&reader)
                    .read_into_async(
                        rejected.buffer.take().expect("required field initialized"),
                        terrane_int_support::Int::from(2_i128),
                        None,
                    ),
            )
            .await;
        first_bytes = first
            .buffer
            .as_ref()
            .expect("required field initialized")
            .filled_bytes();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(first.completed.clone() ==
            terrane_int_support::Int::from(2_i128) &&first.buffer.as_ref()
            .expect("required field initialized").completed() ==
            terrane_int_support::Int::from(2_i128) &&first_bytes.value == Vec:: < u8 >
            ::from([97, 98])))
        );
        second = __terrane_await(
                (&reader)
                    .read_into_async(
                        first.buffer.take().expect("required field initialized"),
                        terrane_int_support::Int::from(2_i128),
                        None,
                    ),
            )
            .await;
        all_bytes = second
            .buffer
            .as_ref()
            .expect("required field initialized")
            .filled_bytes();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(second.completed.clone() ==
            terrane_int_support::Int::from(2_i128) &&second.buffer.as_ref()
            .expect("required field initialized").completed() ==
            terrane_int_support::Int::from(4_i128) &&all_bytes.value == Vec:: < u8 >
            ::from([97, 98, 99, 100])))
        );
        zero = __terrane_await(
                (&reader)
                    .read_into_async(
                        second.buffer.take().expect("required field initialized"),
                        terrane_int_support::Int::from(0_i128),
                        None,
                    ),
            )
            .await;
        println!(
            "{}", terrane_scalar_support::scalar_text(&(! zero.failed &&! zero.end &&zero
            .completed.clone() == terrane_int_support::Int::from(0_i128)))
        );
        buffer = zero.buffer.take().expect("required field initialized");
        cleared = buffer.clear();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(! cleared.failed &&buffer
            .completed() == terrane_int_support::Int::from(0_i128)))
        );
        empty = buffer.filled_bytes();
        println!(
            "{}", terrane_scalar_support::scalar_text(&(empty.value == Vec:: < u8 >
            ::from([])))
        );
        reader.close();
    });
}
