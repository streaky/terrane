// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: core-facility-composition
fn main() {
    let filesystem_result: FilesystemOperationResult;
    let stream_result: StreamOperationResult;
    filesystem_result = FilesystemOperationResult::terrane_construct(
        false,
        String::from(""),
    );
    stream_result = StreamOperationResult::terrane_construct(false, String::from(""));
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! filesystem_result.failed &&!
        stream_result.failed))
    );
}
