// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: application-value-semantics
fn close_or_read(authority: Filesystem, handle: FileHandle, should_close: bool) {
    let outcome: FilesystemOperationResult;
    let read: FileData;
    if should_close {
        outcome = file_close(authority.clone(), handle);
        let _ = &outcome;
        return ();
    }
    read = file_read(authority, &handle, terrane_int_support::Int::from(1_i128));
    let _ = &read;
}
fn main() {
    let authority: Filesystem;
    let target: Path;
    let mut index: terrane_int_support::Int;
    let mut outcome: ExistenceResult;
    let handle: FileHandle;
    authority = filesystem_capability();
    target = Path::terrane_construct(
        String::from("missing-application-value-semantics"),
    );
    let __terrane_iterable_0 = __terrane_raised(
        terrane_collection_support::Range::new(
            terrane_int_support::Int::from(0_i128),
            terrane_int_support::Int::from(2_i128),
            terrane_int_support::Int::from(1_i64),
        ),
        0 /* terrane-site: case.trn:18:17-18:28 */,
    );
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &__terrane_iterable_0,
    );
    loop {
        index = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        let _ = &index;
        outcome = filesystem_exists(authority.clone(), target.clone());
        let _ = &outcome;
    }
    handle = open_file(
        authority.clone(),
        target,
        true,
        false,
        false,
        false,
        false,
        false,
    );
    close_or_read(authority, handle, true);
}
