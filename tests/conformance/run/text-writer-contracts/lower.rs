// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: text-writer-contracts
fn main() {
    let output: ByteWriter;
    let prefix: WriteResult;
    let writer: TextWriter;
    let partial: WriteResult;
    let resumed: WriteResult;
    let line: WriteResult;
    let data_sync: StreamOperationResult;
    let all_sync: StreamOperationResult;
    output = stdout();
    prefix = output.write(Vec::from([97]));
    writer = output.text(terrane_string_support::Encoding::Utf8);
    partial = WriteResult::terrane_construct(
        Vec::from([97, 98, 99]),
        terrane_int_support::Int::from(1_i128),
        false,
        String::from(""),
    );
    resumed = writer.resume(partial);
    line = writer.line(String::from("x"));
    data_sync = writer.sync_data();
    all_sync = writer.sync_all();
    println!("{}", terrane_scalar_support::scalar_text(&prefix.completed));
    println!("{}", terrane_scalar_support::scalar_text(&resumed.completed));
    println!("{}", terrane_scalar_support::scalar_text(&line.completed));
    println!("{}", terrane_scalar_support::scalar_text(&data_sync.failed));
    println!("{}", terrane_scalar_support::scalar_text(&all_sync.failed));
    writer.close();
}
