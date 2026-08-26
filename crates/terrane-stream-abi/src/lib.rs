//! Minimal host-ABI boundary for process standard streams.
//!
//! This crate exists because acquiring and operating the process-owned standard
//! handles crosses the host ABI. Stream policy, text encoding, convenience
//! operations, cancellation reporting, and the public object model belong in
//! Terrane source above this boundary.

use std::{
    collections::BTreeMap,
    io::{self, Read, Write},
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicI64, Ordering},
    },
};

#[derive(Debug)]
pub struct ReadOutcome {
    pub data: Vec<u8>,
    pub end: bool,
}

#[derive(Debug)]
pub struct StdinReader {
    stdin: io::Stdin,
}

impl StdinReader {
    #[must_use]
    pub fn acquire() -> Self {
        Self { stdin: io::stdin() }
    }

    /// Performs at most one host read, preserving partial completion and EOF.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the host read fails.
    pub fn read(&mut self, limit: usize) -> io::Result<ReadOutcome> {
        let mut data = vec![0; limit];
        let completed = self.stdin.read(&mut data)?;
        data.truncate(completed);
        Ok(ReadOutcome {
            data,
            end: completed == 0,
        })
    }

    /// Releases this wrapper. The process-owned standard handle itself remains owned by the host.
    ///
    /// # Errors
    ///
    /// This process-stream implementation currently closes infallibly.
    pub fn close(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct StdoutWriter {
    stdout: io::Stdout,
}

#[derive(Debug)]
pub struct StderrWriter {
    stderr: io::Stderr,
}

macro_rules! impl_standard_writer {
    ($type:ty, $field:ident, $acquire:expr) => {
        impl $type {
            #[must_use]
            pub fn acquire() -> Self {
                Self { $field: $acquire }
            }

            /// Performs at most one host write and returns its partial completion count.
            ///
            /// # Errors
            ///
            /// Returns an I/O error when the host write fails.
            pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
                self.$field.write(data)
            }

            /// Flushes buffered host output.
            ///
            /// # Errors
            ///
            /// Returns an I/O error when the host flush fails.
            pub fn flush(&mut self) -> io::Result<()> {
                self.$field.flush()
            }

            /// Standard process streams do not expose filesystem data durability.
            ///
            /// # Errors
            ///
            /// Always reports unsupported durability.
            pub fn sync_data(&mut self) -> io::Result<()> {
                Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "standard streams do not support sync-data",
                ))
            }

            /// Standard process streams do not expose filesystem metadata durability.
            ///
            /// # Errors
            ///
            /// Always reports unsupported durability.
            pub fn sync_all(&mut self) -> io::Result<()> {
                Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "standard streams do not support sync-all",
                ))
            }

            /// Flushes once before this wrapper is released.
            ///
            /// # Errors
            ///
            /// Returns an I/O error when the final host flush fails.
            pub fn close(&mut self) -> io::Result<()> {
                self.$field.flush()
            }
        }
    };
}

impl_standard_writer!(StdoutWriter, stdout, io::stdout());
impl_standard_writer!(StderrWriter, stderr, io::stderr());

#[derive(Debug)]
pub struct FileStream {
    file: std::fs::File,
    readable: bool,
    writable: bool,
    cross_filesystem: bool,
}

impl FileStream {
    fn read(&mut self, limit: usize) -> io::Result<ReadOutcome> {
        if !self.readable {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file is not readable",
            ));
        }
        let mut data = vec![0; limit];
        let completed = self.file.read(&mut data)?;
        data.truncate(completed);
        Ok(ReadOutcome {
            data,
            end: completed == 0,
        })
    }

    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if !self.writable {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "file is not writable",
            ));
        }
        self.file.write(data)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
    fn sync_data(&mut self) -> io::Result<()> {
        self.file.sync_data()
    }
    fn sync_all(&mut self) -> io::Result<()> {
        self.file.sync_all()
    }
    fn close(&mut self) -> io::Result<()> {
        if self.writable {
            self.file.flush()
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StreamHandle(i64);

impl StreamHandle {
    #[must_use]
    pub const fn id(self) -> i64 {
        self.0
    }

    #[must_use]
    pub const fn from_id(id: i64) -> Self {
        Self(id)
    }
}

enum StandardStream {
    Stdin(StdinReader),
    Stdout(StdoutWriter),
    Stderr(StderrWriter),
    File(FileStream),
}

static NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);
static STREAMS: LazyLock<Mutex<BTreeMap<i64, Arc<Mutex<StandardStream>>>>> =
    LazyLock::new(|| Mutex::new(BTreeMap::new()));

fn registry() -> std::sync::MutexGuard<'static, BTreeMap<i64, Arc<Mutex<StandardStream>>>> {
    STREAMS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn acquire(stream: StandardStream) -> StreamHandle {
    let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    registry().insert(handle, Arc::new(Mutex::new(stream)));
    StreamHandle(handle)
}

#[must_use]
pub fn acquire_stdin() -> StreamHandle {
    acquire(StandardStream::Stdin(StdinReader::acquire()))
}

#[must_use]
pub fn acquire_stdout() -> StreamHandle {
    acquire(StandardStream::Stdout(StdoutWriter::acquire()))
}

#[must_use]
pub fn acquire_stderr() -> StreamHandle {
    acquire(StandardStream::Stderr(StderrWriter::acquire()))
}

#[derive(Clone, Copy, Debug)]
pub enum FileAccess {
    Read,
    Write,
    ReadWrite,
}

#[derive(Clone, Copy, Debug)]
pub enum FileCreation {
    Existing,
    Create,
    Truncate,
    CreateOrTruncate,
}

#[derive(Clone, Copy, Debug)]
pub struct FileOpenOptions {
    pub access: FileAccess,
    pub creation: FileCreation,
}

/// Opens a filesystem file without following a final symlink on Unix targets.
///
/// # Errors
///
/// Returns the host open error, including a refusal to follow the final symlink.
pub fn open_file(path: &str, request: FileOpenOptions) -> io::Result<StreamHandle> {
    let mut options = std::fs::OpenOptions::new();
    let (readable, writable) = match request.access {
        FileAccess::Read => (true, false),
        FileAccess::Write => (false, true),
        FileAccess::ReadWrite => (true, true),
    };
    let (create, truncate) = match request.creation {
        FileCreation::Existing => (false, false),
        FileCreation::Create => (true, false),
        FileCreation::Truncate => (false, true),
        FileCreation::CreateOrTruncate => (true, true),
    };
    options
        .read(readable)
        .write(writable)
        .create(create)
        .truncate(truncate);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.custom_flags(libc::O_NOFOLLOW);
    }
    let file = options.open(path)?;
    Ok(acquire(StandardStream::File(FileStream {
        file,
        readable,
        writable,
        cross_filesystem: false,
    })))
}
#[cfg(target_os = "linux")]
fn linux_open_beneath(
    directory: impl std::os::fd::AsFd,
    child: &str,
    flags: rustix::fs::OFlags,
    cross_filesystem: bool,
) -> io::Result<std::fs::File> {
    let mut resolve = rustix::fs::ResolveFlags::BENEATH
        | rustix::fs::ResolveFlags::NO_MAGICLINKS
        | rustix::fs::ResolveFlags::NO_SYMLINKS;
    if !cross_filesystem {
        resolve |= rustix::fs::ResolveFlags::NO_XDEV;
    }
    Ok(std::fs::File::from(rustix::fs::openat2(
        directory,
        child,
        flags,
        rustix::fs::Mode::empty(),
        resolve,
    )?))
}

/// Opens a directory through race-resistant, no-follow traversal beneath `base`.
///
/// # Errors
///
/// Returns an I/O error when traversal escapes `base`, crosses a filesystem without permission,
/// encounters a symlink, or the target does not support this operation.
#[cfg(target_os = "linux")]
pub fn open_directory_beneath(
    base: &str,
    child: &str,
    cross_filesystem: bool,
) -> io::Result<StreamHandle> {
    let base = std::fs::File::from(rustix::fs::open(
        base,
        rustix::fs::OFlags::RDONLY
            | rustix::fs::OFlags::DIRECTORY
            | rustix::fs::OFlags::NOFOLLOW
            | rustix::fs::OFlags::CLOEXEC,
        rustix::fs::Mode::empty(),
    )?);
    let directory = linux_open_beneath(
        &base,
        child,
        rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::DIRECTORY | rustix::fs::OFlags::CLOEXEC,
        cross_filesystem,
    )?;
    Ok(acquire(StandardStream::File(FileStream {
        file: directory,
        readable: false,
        writable: false,
        cross_filesystem,
    })))
}

/// Opens a file relative to a directory handle returned by [`open_directory_beneath`].
///
/// # Errors
///
/// Returns an I/O error for an invalid directory handle, traversal escape, symlink, or host open
/// failure.
#[cfg(target_os = "linux")]
pub fn open_file_beneath(
    directory: StreamHandle,
    child: &str,
    request: FileOpenOptions,
) -> io::Result<StreamHandle> {
    let (readable, writable) = match request.access {
        FileAccess::Read => (true, false),
        FileAccess::Write => (false, true),
        FileAccess::ReadWrite => (true, true),
    };
    let creation_flags = match request.creation {
        FileCreation::Existing => rustix::fs::OFlags::empty(),
        FileCreation::Create => rustix::fs::OFlags::CREATE,
        FileCreation::Truncate => rustix::fs::OFlags::TRUNC,
        FileCreation::CreateOrTruncate => rustix::fs::OFlags::CREATE | rustix::fs::OFlags::TRUNC,
    };
    let access_flags = match request.access {
        FileAccess::Read => rustix::fs::OFlags::RDONLY,
        FileAccess::Write => rustix::fs::OFlags::WRONLY,
        FileAccess::ReadWrite => rustix::fs::OFlags::RDWR,
    };
    let stream = registered_stream(directory)?;
    let file = {
        let stream = stream
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let StandardStream::File(directory) = &*stream else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid directory handle",
            ));
        };
        linux_open_beneath(
            &directory.file,
            child,
            access_flags | creation_flags | rustix::fs::OFlags::CLOEXEC,
            directory.cross_filesystem,
        )?
    };
    Ok(acquire(StandardStream::File(FileStream {
        file,
        readable,
        writable,
        cross_filesystem: false,
    })))
}

#[cfg(not(target_os = "linux"))]
pub fn open_directory_beneath(_: &str, _: &str, _: bool) -> io::Result<StreamHandle> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "race-resistant beneath traversal is unavailable in this target profile",
    ))
}

#[cfg(not(target_os = "linux"))]
pub fn open_file_beneath(_: StreamHandle, _: &str, _: FileOpenOptions) -> io::Result<StreamHandle> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "handle-relative traversal is unavailable in this target profile",
    ))
}

/// Performs at most one host read through a registered process-stream handle.
///
/// # Errors
///
/// Returns an I/O error for an invalid or non-readable handle, or when the host read fails.
pub fn read(handle: StreamHandle, limit: usize) -> io::Result<ReadOutcome> {
    if limit == 0 {
        return Ok(ReadOutcome {
            data: Vec::new(),
            end: false,
        });
    }
    let stream = registered_stream(handle)?;
    let mut stream = stream
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match &mut *stream {
        StandardStream::Stdin(reader) => reader.read(limit),
        StandardStream::File(file) => file.read(limit),
        StandardStream::Stdout(_) | StandardStream::Stderr(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "stream is not readable",
        )),
    }
}

/// Performs at most one host write through a registered process-stream handle.
///
/// # Errors
///
/// Returns an I/O error for an invalid or non-writable handle, or when the host write fails.
pub fn write(handle: StreamHandle, data: &[u8]) -> io::Result<usize> {
    let stream = registered_stream(handle)?;
    let mut stream = stream
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match &mut *stream {
        StandardStream::Stdout(writer) => writer.write(data),
        StandardStream::Stderr(writer) => writer.write(data),
        StandardStream::File(file) => file.write(data),
        StandardStream::Stdin(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "stream is not writable",
        )),
    }
}

/// Flushes a registered process-stream writer.
///
/// # Errors
///
/// Returns an I/O error for an invalid or non-writable handle, or when the host flush fails.
pub fn flush(handle: StreamHandle) -> io::Result<()> {
    with_writer(handle, |writer| writer.flush())
}

/// Requests data durability from a registered process-stream writer.
///
/// # Errors
///
/// Standard streams report that data durability is unsupported; invalid and non-writable handles
/// also report I/O errors.
pub fn sync_data(handle: StreamHandle) -> io::Result<()> {
    with_writer(handle, |writer| writer.sync_data())
}

/// Requests data and metadata durability from a registered process-stream writer.
///
/// # Errors
///
/// Standard streams report that full durability is unsupported; invalid and non-writable handles
/// also report I/O errors.
pub fn sync_all(handle: StreamHandle) -> io::Result<()> {
    with_writer(handle, |writer| writer.sync_all())
}

/// Releases a registered process-stream wrapper and removes it from the live registry.
///
/// Releasing an already-absent handle is successful so host release remains idempotent. A stream
/// whose final flush fails stays registered, allowing the caller to retry or report the failure
/// without leaking an unreachable live entry.
///
/// # Errors
///
/// Returns an I/O error when a final writer flush fails.
pub fn close(handle: StreamHandle) -> io::Result<()> {
    let Some(stream) = registry().remove(&handle.0) else {
        return Ok(());
    };
    let result = {
        let mut locked = stream
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match &mut *locked {
            StandardStream::Stdin(reader) => reader.close(),
            StandardStream::Stdout(writer) => writer.close(),
            StandardStream::Stderr(writer) => writer.close(),
            StandardStream::File(file) => file.close(),
        }
    };
    if result.is_err() {
        registry().insert(handle.0, stream);
    }
    result
}
/// Releases a registered process-stream wrapper without preserving close failures for retry.
///
/// This is the destructor path: the entry is removed unconditionally because no source-level
/// owner remains to retry or report a failed final flush.
///
/// # Errors
///
/// Returns an I/O error when a final writer flush fails.
pub fn release(handle: StreamHandle) -> io::Result<()> {
    let Some(stream) = registry().remove(&handle.0) else {
        return Ok(());
    };
    let mut stream = stream
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match &mut *stream {
        StandardStream::Stdin(reader) => reader.close(),
        StandardStream::Stdout(writer) => writer.close(),
        StandardStream::Stderr(writer) => writer.close(),
        StandardStream::File(file) => file.close(),
    }
}

fn with_writer(
    handle: StreamHandle,
    operation: impl FnOnce(&mut dyn StandardWriter) -> io::Result<()>,
) -> io::Result<()> {
    let stream = registered_stream(handle)?;
    let mut stream = stream
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    match &mut *stream {
        StandardStream::Stdout(writer) => operation(writer),
        StandardStream::Stderr(writer) => operation(writer),
        StandardStream::File(writer) => operation(writer),
        StandardStream::Stdin(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "stream is not writable",
        )),
    }
}

fn registered_stream(handle: StreamHandle) -> io::Result<Arc<Mutex<StandardStream>>> {
    registry().get(&handle.0).cloned().ok_or_else(invalid_handle)
}

trait StandardWriter {
    fn flush(&mut self) -> io::Result<()>;
    fn sync_data(&mut self) -> io::Result<()>;
    fn sync_all(&mut self) -> io::Result<()>;
}

macro_rules! impl_writer_trait {
    ($type:ty) => {
        impl StandardWriter for $type {
            fn flush(&mut self) -> io::Result<()> {
                Self::flush(self)
            }

            fn sync_data(&mut self) -> io::Result<()> {
                Self::sync_data(self)
            }

            fn sync_all(&mut self) -> io::Result<()> {
                Self::sync_all(self)
            }
        }
    };
}

impl_writer_trait!(StdoutWriter);
impl_writer_trait!(StderrWriter);
impl_writer_trait!(FileStream);

fn invalid_handle() -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, "unknown standard stream handle")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_length_read_is_not_end_of_stream() {
        let handle = acquire_stdin();
        let outcome = read(handle, 0).unwrap();
        assert!(outcome.data.is_empty());
        assert!(!outcome.end);
        close(handle).unwrap();
    }

    #[test]
    fn close_removes_live_entry_and_stays_idempotent() {
        let handle = acquire_stdin();
        close(handle).unwrap();
        assert_eq!(read(handle, 1).unwrap_err().kind(), io::ErrorKind::NotFound);
        close(handle).unwrap();
    }

    #[test]
    fn release_removes_entry_even_when_no_owner_can_retry() {
        let handle = acquire_stdin();
        release(handle).unwrap();
        assert_eq!(read(handle, 1).unwrap_err().kind(), io::ErrorKind::NotFound);
        release(handle).unwrap();
    }

    #[test]
    fn file_handles_preserve_partial_io_and_explicit_close() {
        let path =
            std::env::temp_dir().join(format!("terrane-stream-abi-file-{}", std::process::id()));
        let writer = open_file(
            path.to_str().unwrap(),
            FileOpenOptions {
                access: FileAccess::Write,
                creation: FileCreation::CreateOrTruncate,
            },
        )
        .unwrap();
        assert_eq!(write(writer, b"content").unwrap(), 7);
        sync_all(writer).unwrap();
        close(writer).unwrap();

        let reader = open_file(
            path.to_str().unwrap(),
            FileOpenOptions {
                access: FileAccess::Read,
                creation: FileCreation::Existing,
            },
        )
        .unwrap();
        let first = read(reader, 3).unwrap();
        assert_eq!(first.data, b"con");
        assert!(!first.end);
        let second = read(reader, 8).unwrap();
        assert_eq!(second.data, b"tent");
        assert!(!second.end);
        assert!(read(reader, 8).unwrap().end);
        close(reader).unwrap();
        std::fs::remove_file(path).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn file_open_refuses_a_final_symlink() {
        use std::os::unix::fs::symlink;

        let directory =
            std::env::temp_dir().join(format!("terrane-stream-abi-symlink-{}", std::process::id()));
        std::fs::create_dir_all(&directory).unwrap();
        let target = directory.join("target");
        let link = directory.join("link");
        std::fs::write(&target, b"content").unwrap();
        symlink(&target, &link).unwrap();
        assert!(
            open_file(
                link.to_str().unwrap(),
                FileOpenOptions {
                    access: FileAccess::Read,
                    creation: FileCreation::Existing,
                },
            )
            .is_err()
        );
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn directory_handle_confines_relative_file_traversal() {
        use std::os::unix::fs::symlink;

        let directory =
            std::env::temp_dir().join(format!("terrane-stream-abi-beneath-{}", std::process::id()));
        let nested = directory.join("nested");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::write(nested.join("target"), b"content").unwrap();
        symlink("../target", nested.join("link")).unwrap();

        let handle = open_directory_beneath(directory.to_str().unwrap(), "nested", false).unwrap();
        let opened = open_file_beneath(
            handle,
            "target",
            FileOpenOptions {
                access: FileAccess::Read,
                creation: FileCreation::Existing,
            },
        )
        .unwrap();
        assert_eq!(read(opened, 7).unwrap().data, b"content");
        close(opened).unwrap();
        assert!(
            open_file_beneath(
                handle,
                "../outside",
                FileOpenOptions {
                    access: FileAccess::Read,
                    creation: FileCreation::Existing,
                },
            )
            .is_err()
        );
        assert!(
            open_file_beneath(
                handle,
                "link",
                FileOpenOptions {
                    access: FileAccess::Read,
                    creation: FileCreation::Existing,
                },
            )
            .is_err()
        );
        close(handle).unwrap();
        std::fs::remove_dir_all(directory).unwrap();
    }
}
