// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: core-tool-type-import
fn preserve(result: ExistenceResult) -> ExistenceResult {
    return result;
}
fn main() {
    let capability: Filesystem;
    let result: ExistenceResult;
    capability = filesystem_capability();
    result = filesystem_exists(capability, Path::terrane_construct(String::from(".")));
    preserve(result);
}
