// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/filesystem-durability-exports
fn exercise(capability: Filesystem, output: &FileHandle) -> bool {
    let data: FilesystemOperationResult;
    let all: FilesystemOperationResult;
    data = file_sync_data(capability.clone(), output);
    all = file_sync_all(capability, output);
    return data.failed || all.failed;
}
fn main() {
    let capability: Filesystem;
    let output: FileHandle;
    capability = filesystem_capability();
    output = open_file(
        capability.clone(),
        Path::terrane_construct(String::from("durability-check.tmp")),
        false,
        true,
        true,
        true,
    );
    exercise(capability, &output);
}
