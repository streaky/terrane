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
        Vec::from([99, 111, 110, 116, 101, 110, 116]),
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
    opened = open_file(fs.clone(), target.clone(), true, false, false, false);
    streamed = file_read(fs.clone(), &opened, terrane_int_support::Int::from(16_i128));
    println!(
        "{}{}{}", terrane_scalar_support::scalar_text(&streamed.failed),
        terrane_scalar_support::scalar_text(&streamed.completed),
        terrane_scalar_support::scalar_text(&streamed.end)
    );
    closing = open_file(fs.clone(), target.clone(), true, false, false, false);
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
    partial_target = open_file(fs.clone(), target.clone(), false, true, false, true);
    partial = file_write(
        fs.clone(),
        &partial_target,
        Vec::from([97, 98, 99]),
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
    durable_target = open_file(fs.clone(), target.clone(), false, true, false, false);
    sync_failed = sync_file(fs.clone(), &durable_target);
    println!("{}", terrane_scalar_support::scalar_text(&sync_failed));
    link_removed = filesystem_remove(fs.clone(), link);
    println!("{}", terrane_scalar_support::scalar_text(&link_removed.failed));
    removed = filesystem_remove(fs, target);
    println!("{}", terrane_scalar_support::scalar_text(&removed.failed));
}
