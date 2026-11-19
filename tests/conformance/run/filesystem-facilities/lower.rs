// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: conformance/filesystem-facilities
fn sync_file(capability: Filesystem, output: &FileHandle) -> bool {
    let flushed: FilesystemOperationResult;
    let data: FilesystemOperationResult;
    let all: FilesystemOperationResult;
    flushed = file_flush(capability.clone(), output);
    data = file_sync_data(capability.clone(), output);
    all = file_sync_all(capability, output);
    return flushed.failed || data.failed || all.failed;
}
fn exercise_directory_operations(fs: Filesystem, outside: Path) {
    let tree: Path;
    let nested: Path;
    let missing_parent: FilesystemOperationResult;
    let made: FilesystemOperationResult;
    let repeated: FilesystemOperationResult;
    let duplicate: FilesystemOperationResult;
    let entries: DirectoryListing;
    let limited: DirectoryListing;
    let plain_remove: FilesystemOperationResult;
    let removed_empty: FilesystemOperationResult;
    let kept: Path;
    let kept_file: Path;
    let kept_created: FilesystemOperationResult;
    let kept_written: FilesystemOperationResult;
    let link: FilesystemOperationResult;
    let hard_path: Path;
    let hard: FilesystemOperationResult;
    let hard_handle: FileHandle;
    let hard_written: FileData;
    let hard_closed: FilesystemOperationResult;
    let contents: FileData;
    let removed_tree: FilesystemOperationResult;
    let after: ExistenceResult;
    let retained: FileData;
    let retained_directory_data: FileData;
    tree = Path::terrane_construct(String::from("tree"));
    nested = Path::terrane_construct(String::from("tree/nested/deep"));
    missing_parent = filesystem_create_directory(fs.clone(), nested.clone());
    made = filesystem_create_directories(fs.clone(), nested.clone());
    repeated = filesystem_create_directories(fs.clone(), nested.clone());
    duplicate = filesystem_create_directory(fs.clone(), nested.clone());
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&(missing_parent.failed
        &&missing_parent.error_kind.as_str() == "not-found")),
        terrane_scalar_support::scalar_text(&(! made.failed &&! repeated.failed)),
        terrane_scalar_support::scalar_text(&(duplicate.failed &&duplicate.error_kind
        .as_str() == "already-exists"))
    );
    entries = filesystem_list_directory(
        fs.clone(),
        tree.clone(),
        terrane_int_support::Int::from(1_i128),
    );
    if !entries.failed
        && terrane_int_support::Int::from(entries.entries.length())
            == terrane_int_support::Int::from(1_i128)
    {
        println!(
            "{}", terrane_scalar_support::scalar_text(&__terrane_raised(entries.entries
            .get_or_error(__terrane_raised(terrane_collection_support::index_from_int(&terrane_int_support::Int::from(0_i128)),
            0 /* terrane-site: case.trn:29:16-29:34 */)), 0 /* terrane-site: case.trn:29:16-29:34 */))
        );
    } else {
        println!("{}", terrane_scalar_support::scalar_text(&entries.message));
    }
    limited = filesystem_list_directory(
        fs.clone(),
        tree.clone(),
        terrane_int_support::Int::from(0_i128),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&limited.failed),
        terrane_scalar_support::scalar_text(&limited.error_kind),
        terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(limited
        .entries.length()))
    );
    plain_remove = filesystem_remove_directory(fs.clone(), tree.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&plain_remove.failed),
        terrane_scalar_support::scalar_text(&plain_remove.error_kind)
    );
    removed_empty = filesystem_remove_directory(fs.clone(), nested);
    println!("{}", terrane_scalar_support::scalar_text(&removed_empty.failed));
    kept = Path::terrane_construct(String::from("kept"));
    kept_file = Path::terrane_construct(String::from("kept/contents"));
    kept_created = filesystem_create_directory(fs.clone(), kept);
    kept_written = filesystem_write_atomic(
        fs.clone(),
        kept_file.clone(),
        Vec::<u8>::from([107, 101, 101, 112]),
    );
    link = filesystem_create_symlink(
        fs.clone(),
        Path::terrane_construct(String::from("../kept")),
        Path::terrane_construct(String::from("tree/external")),
    );
    hard_path = Path::terrane_construct(String::from("tree/duplicate"));
    hard = filesystem_create_hard_link(fs.clone(), kept_file.clone(), hard_path.clone());
    hard_handle = open_file(
        fs.clone(),
        hard_path.clone(),
        false,
        true,
        false,
        false,
        false,
        true,
    );
    hard_written = file_write(
        fs.clone(),
        &hard_handle,
        Vec::<u8>::from([33]),
        terrane_int_support::Int::from(0_i128),
    );
    hard_closed = file_close(fs.clone(), hard_handle);
    contents = filesystem_read_bounded(
        fs.clone(),
        hard_path,
        terrane_int_support::Int::from(16_i128),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&link.failed),
        terrane_scalar_support::scalar_text(&hard.failed),
        terrane_scalar_support::scalar_text(&(! hard_written.failed &&! hard_closed
        .failed &&contents.data == Vec:: < u8 > ::from([107, 101, 101, 112, 33])))
    );
    removed_tree = filesystem_remove_directory_tree(fs.clone(), tree.clone());
    after = filesystem_exists(fs.clone(), tree);
    retained = filesystem_read_bounded(
        fs.clone(),
        outside,
        terrane_int_support::Int::from(16_i128),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&removed_tree.failed),
        terrane_scalar_support::scalar_text(&after.exists),
        terrane_scalar_support::scalar_text(&(retained.data == Vec:: < u8 > ::from([99,
        111, 110, 116, 101, 110, 116])))
    );
    retained_directory_data = filesystem_read_bounded(
        fs,
        kept_file,
        terrane_int_support::Int::from(16_i128),
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&(! kept_created.failed &&!
        kept_written.failed &&retained_directory_data.data == Vec:: < u8 > ::from([107,
        101, 101, 112, 33])))
    );
}
fn exercise_open_modes(fs: Filesystem) {
    let subject: Path;
    let invalid: FileHandle;
    let fresh: FileHandle;
    let written: FileData;
    let closed: FilesystemOperationResult;
    let duplicate: FileHandle;
    let base: Path;
    let anchor: DirectoryHandle;
    let relative_duplicate: FileHandle;
    let appended: FileHandle;
    let appended_data: FileData;
    let appended_closed: FilesystemOperationResult;
    let bytes_read: FileData;
    let removed: FilesystemOperationResult;
    subject = Path::terrane_construct(String::from("file-open-modes.txt"));
    invalid = open_file(
        fs.clone(),
        subject.clone(),
        true,
        false,
        false,
        false,
        false,
        true,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&invalid.failed),
        terrane_scalar_support::scalar_text(&invalid.error_kind)
    );
    fresh = open_file(
        fs.clone(),
        subject.clone(),
        false,
        true,
        false,
        false,
        true,
        false,
    );
    println!("{}", terrane_scalar_support::scalar_text(&fresh.failed));
    written = file_write(
        fs.clone(),
        &fresh,
        Vec::<u8>::from([98, 97, 115, 101]),
        terrane_int_support::Int::from(0_i128),
    );
    closed = file_close(fs.clone(), fresh);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&written.completed),
        terrane_scalar_support::scalar_text(&closed.failed)
    );
    duplicate = open_file(
        fs.clone(),
        subject.clone(),
        false,
        true,
        false,
        false,
        true,
        false,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&duplicate.failed),
        terrane_scalar_support::scalar_text(&duplicate.error_kind)
    );
    base = Path::terrane_construct(String::from("."));
    anchor = filesystem_open_beneath(fs.clone(), base.clone(), base, false);
    relative_duplicate = open_file_beneath(
        fs.clone(),
        &anchor,
        subject.clone(),
        false,
        true,
        false,
        false,
        true,
        false,
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&relative_duplicate.failed),
        terrane_scalar_support::scalar_text(&relative_duplicate.error_kind)
    );
    appended = open_file_beneath(
        fs.clone(),
        &anchor,
        subject.clone(),
        false,
        true,
        false,
        false,
        false,
        true,
    );
    appended_data = file_write(
        fs.clone(),
        &appended,
        Vec::<u8>::from([43, 116, 97, 105, 108]),
        terrane_int_support::Int::from(0_i128),
    );
    appended_closed = file_close(fs.clone(), appended);
    bytes_read = filesystem_read_bounded(
        fs.clone(),
        subject.clone(),
        terrane_int_support::Int::from(16_i128),
    );
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&appended_data.completed),
        terrane_scalar_support::scalar_text(&appended_closed.failed),
        terrane_scalar_support::scalar_text(&(bytes_read.data == Vec:: < u8 > ::from([98,
        97, 115, 101, 43, 116, 97, 105, 108])))
    );
    removed = filesystem_remove(fs, subject);
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&removed.failed),
        terrane_scalar_support::scalar_text(&removed.error_kind)
    );
}
fn main() {
    let fs: Filesystem;
    let target: Path;
    let written: FilesystemOperationResult;
    let exists: ExistenceResult;
    let metadata: FileMetadata;
    let data: FileData;
    let opened: FileHandle;
    let streamed: FileData;
    let closing: FileHandle;
    let closed: FilesystemOperationResult;
    let lexical_input: Path;
    let lexical: Path;
    let canonical: PathResult;
    let real: PathResult;
    let link: Path;
    let created_link: FilesystemOperationResult;
    let link_metadata: FileMetadata;
    let linked: PathResult;
    let renamed_target: Path;
    let renamed: FilesystemOperationResult;
    let restored: FilesystemOperationResult;
    let base: Path;
    let escape: Path;
    let escaped: DirectoryHandle;
    let current: Path;
    let root: DirectoryHandle;
    let manifest: Path;
    let relative_file: FileHandle;
    let relative_data: FileData;
    let partial_target: FileHandle;
    let partial: FileData;
    let partial_read: FileData;
    let durable_target: FileHandle;
    let sync_failed: bool;
    let link_removed: FilesystemOperationResult;
    let removed: FilesystemOperationResult;
    fs = filesystem_capability();
    target = Path::terrane_construct(String::from("terrane-filesystem-case.txt"));
    written = filesystem_write_atomic(
        fs.clone(),
        target.clone(),
        Vec::<u8>::from([99, 111, 110, 116, 101, 110, 116]),
    );
    println!("{}", terrane_scalar_support::scalar_text(&written.failed));
    exists = filesystem_exists(fs.clone(), target.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&exists.exists),
        terrane_scalar_support::scalar_text(&exists.failed)
    );
    metadata = filesystem_metadata(fs.clone(), target.clone());
    println!(
        "{}{}{}{}{}", terrane_scalar_support::scalar_text(&metadata.failed),
        terrane_scalar_support::scalar_text(&metadata.kind),
        terrane_scalar_support::scalar_text(&metadata.size),
        terrane_scalar_support::scalar_text(&metadata.readonly),
        terrane_scalar_support::scalar_text(&(metadata.permission_detail.as_str() == ""))
    );
    data = filesystem_read_bounded(
        fs.clone(),
        target.clone(),
        terrane_int_support::Int::from(7_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&data.failed),
        terrane_scalar_support::scalar_text(&data.completed)
    );
    opened = open_file(
        fs.clone(),
        target.clone(),
        true,
        false,
        false,
        false,
        false,
        false,
    );
    streamed = file_read(fs.clone(), &opened, terrane_int_support::Int::from(16_i128));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&streamed.failed),
        terrane_scalar_support::scalar_text(&streamed.completed),
        terrane_scalar_support::scalar_text(&streamed.end)
    );
    closing = open_file(
        fs.clone(),
        target.clone(),
        true,
        false,
        false,
        false,
        false,
        false,
    );
    closed = file_close(fs.clone(), closing);
    println!("{}", terrane_scalar_support::scalar_text(&closed.failed));
    lexical_input = Path::terrane_construct(
        String::from("missing/../terrane-filesystem-case.txt"),
    );
    lexical = normalise_path(lexical_input);
    canonical = filesystem_canonical(fs.clone(), target.clone());
    real = filesystem_realpath(fs.clone(), target.clone());
    println!("{}", terrane_scalar_support::scalar_text(&lexical.text));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&canonical.failed),
        terrane_scalar_support::scalar_text(&(canonical.resolved.text.as_str() == lexical
        .text.as_str())), terrane_scalar_support::scalar_text(&(real.resolved.text
        .as_str() == canonical.resolved.text.as_str()))
    );
    link = Path::terrane_construct(String::from("terrane-filesystem-case-link.txt"));
    created_link = filesystem_create_symlink(fs.clone(), target.clone(), link.clone());
    println!("{}", terrane_scalar_support::scalar_text(&created_link.failed));
    link_metadata = filesystem_symlink_metadata(fs.clone(), link.clone());
    linked = filesystem_read_link(fs.clone(), link.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&link_metadata.failed),
        terrane_scalar_support::scalar_text(&link_metadata.kind)
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&linked.failed),
        terrane_scalar_support::scalar_text(&linked.resolved.text)
    );
    renamed_target = Path::terrane_construct(
        String::from("terrane-filesystem-case-renamed.txt"),
    );
    renamed = filesystem_rename(fs.clone(), target.clone(), renamed_target.clone());
    restored = filesystem_rename(fs.clone(), renamed_target, target.clone());
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&renamed.failed),
        terrane_scalar_support::scalar_text(&restored.failed)
    );
    exercise_directory_operations(fs.clone(), target.clone());
    exercise_open_modes(fs.clone());
    base = Path::terrane_construct(String::from("."));
    escape = Path::terrane_construct(String::from("../Cargo.toml"));
    escaped = filesystem_open_beneath(fs.clone(), base.clone(), escape, false);
    println!("{}", terrane_scalar_support::scalar_text(&escaped.failed));
    current = Path::terrane_construct(String::from("."));
    root = filesystem_open_beneath(fs.clone(), base, current, false);
    manifest = Path::terrane_construct(String::from("terrane-filesystem-case.txt"));
    relative_file = open_file_beneath(
        fs.clone(),
        &root,
        manifest,
        true,
        false,
        false,
        false,
        false,
        false,
    );
    relative_data = file_read(
        fs.clone(),
        &relative_file,
        terrane_int_support::Int::from(8_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&relative_data.failed),
        terrane_scalar_support::scalar_text(&relative_data.completed)
    );
    partial_target = open_file(
        fs.clone(),
        target.clone(),
        false,
        true,
        false,
        true,
        false,
        false,
    );
    partial = file_write(
        fs.clone(),
        &partial_target,
        Vec::<u8>::from([97, 98, 99]),
        terrane_int_support::Int::from(1_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&partial.failed),
        terrane_scalar_support::scalar_text(&partial.completed)
    );
    partial_read = filesystem_read_bounded(
        fs.clone(),
        target.clone(),
        terrane_int_support::Int::from(3_i128),
    );
    println!(
        "{}{}", terrane_scalar_support::scalar_text(&partial_read.failed),
        terrane_scalar_support::scalar_text(&partial_read.completed)
    );
    durable_target = open_file(
        fs.clone(),
        target.clone(),
        false,
        true,
        false,
        false,
        false,
        false,
    );
    sync_failed = sync_file(fs.clone(), &durable_target);
    println!("{}", terrane_scalar_support::scalar_text(&sync_failed));
    link_removed = filesystem_remove(fs.clone(), link);
    println!("{}", terrane_scalar_support::scalar_text(&link_removed.failed));
    removed = filesystem_remove(fs, target);
    println!("{}", terrane_scalar_support::scalar_text(&removed.failed));
}
