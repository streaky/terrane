// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: standard-stderr-stream
fn main() {
    let error: ByteWriter;
    let written: WriteResult;
    error = stderr();
    written = error
        .write_all(
            Vec::from([116, 121, 112, 101, 100, 32, 115, 116, 100, 101, 114, 114, 10]),
        );
    if written.failed {
        println!("{}", terrane_scalar_support::scalar_text(&written.message));
    }
    error.close();
}
