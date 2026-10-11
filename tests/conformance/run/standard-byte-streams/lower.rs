// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: standard-byte-streams
fn main() {
    let input: ByteReader;
    let reader: ByteReader;
    let first: ReadResult;
    let remainder: ReadResult;
    let output: ByteWriter;
    let writer: ByteWriter;
    let first_write: WriteResult;
    let second_write: WriteResult;
    let prefix: WriteResult;
    let partial: WriteResult;
    let resumed: WriteResult;
    let flushed: StreamOperationResult;
    let synced: StreamOperationResult;
    let fully_synced: StreamOperationResult;
    let input_closed: StreamOperationResult;
    let output_closed: StreamOperationResult;
    input = stdin();
    reader = input;
    first = reader.read_exact(terrane_int_support::Int::from(3_i128));
    if first.failed {
        println!("{}", terrane_scalar_support::scalar_text(&first.message));
    }
    if first.completed.clone() != terrane_int_support::Int::from(3_i128) || first.end {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("unexpected first read"))
        );
    }
    remainder = reader.read_all(terrane_int_support::Int::from(100_i128));
    if remainder.failed {
        println!("{}", terrane_scalar_support::scalar_text(&remainder.message));
    }
    if remainder.completed.clone() != terrane_int_support::Int::from(4_i128)
        || !remainder.end
    {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("unexpected remainder"))
        );
    }
    output = stdout();
    writer = output;
    first_write = writer.write_all(first.data.clone());
    if first_write.failed {
        println!("{}", terrane_scalar_support::scalar_text(&first_write.message));
    }
    second_write = writer.write_all(remainder.data.clone());
    if second_write.failed {
        println!("{}", terrane_scalar_support::scalar_text(&second_write.message));
    }
    prefix = writer.write(Vec::from([100]));
    partial = WriteResult::terrane_construct(
        Vec::from([100, 101, 102]),
        terrane_int_support::Int::from(1_i128),
        false,
        String::from(""),
    );
    resumed = writer.resume(partial);
    if prefix.completed.clone() != terrane_int_support::Int::from(1_i128)
        || resumed.completed.clone() != terrane_int_support::Int::from(3_i128)
    {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("unexpected resumed write"))
        );
    }
    flushed = writer.flush();
    if flushed.failed {
        println!("{}", terrane_scalar_support::scalar_text(&flushed.message));
    }
    synced = writer.sync_data();
    if !synced.failed {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("unexpected standard stream sync"))
        );
    }
    fully_synced = writer.sync_all();
    if !fully_synced.failed {
        println!(
            "{}",
            terrane_scalar_support::scalar_text(&String::from("unexpected standard stream full sync"))
        );
    }
    input_closed = reader.close();
    if input_closed.failed {
        println!("{}", terrane_scalar_support::scalar_text(&input_closed.message));
    }
    output_closed = writer.close();
    if output_closed.failed {
        println!("{}", terrane_scalar_support::scalar_text(&output_closed.message));
    }
}
