use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io;
use std::os::unix::fs::{DirBuilderExt as _, OpenOptionsExt as _};
use std::path::{Path, PathBuf};

/// Owns cleanup of a freshly created filesystem entry. The parent and its
/// resolution must remain trusted and stable for the lifetime of the owner.
pub struct TemporaryPath {
    path: Option<PathBuf>,
    directory: bool,
}

impl TemporaryPath {
    //# SECURITY-BEGIN [filesystem.temporary-paths]
    /// Creates a private directory under `parent` or the host temporary directory.
    ///
    /// # Errors
    /// Returns an error if a name cannot be generated or the directory cannot be created.
    pub fn directory(parent: Option<&Path>) -> io::Result<Self> {
        Self::create(parent, true, |path| {
            DirBuilder::new().mode(0o700).create(path)
        })
        .map(|(owner, ())| owner)
    }

    /// Creates a private file and returns its original, open descriptor.
    ///
    /// # Errors
    /// Returns an error if a name cannot be generated or the file cannot be created.
    pub fn file(parent: Option<&Path>) -> io::Result<(Self, File)> {
        Self::create(parent, false, |path| {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(path)
        })
    }

    fn create<T>(
        parent: Option<&Path>,
        directory: bool,
        create: impl Fn(&Path) -> io::Result<T>,
    ) -> io::Result<(Self, T)> {
        let mut path = match parent {
            Some(parent) => std::path::absolute(parent)?,
            None => std::path::absolute(std::env::temp_dir())?,
        };
        let mut name = *b"terrane-00000000000000000000000000000000";
        let digits = b"0123456789abcdef";
        for _ in 0..16 {
            let mut random = [0; 16];
            getrandom::fill(&mut random).map_err(|error| io::Error::other(error.to_string()))?;
            for (pair, byte) in name[8..].as_chunks_mut::<2>().0.iter_mut().zip(random) {
                pair[0] = digits[usize::from(byte >> 4)];
                pair[1] = digits[usize::from(byte & 15)];
            }
            path.push(std::str::from_utf8(&name).expect("ASCII temporary name"));
            match create(&path) {
                Ok(value) => {
                    return Ok((
                        Self {
                            path: Some(path),
                            directory,
                        },
                        value,
                    ));
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    path.pop();
                }
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "temporary name collisions",
        ))
    }
    //# SECURITY-END [filesystem.temporary-paths]

    pub fn path(&self) -> &Path {
        self.path.as_deref().expect("live temporary path")
    }

    /// Transfers cleanup responsibility to the caller without moving the entry.
    pub fn keep(mut self) -> PathBuf {
        self.path.take().expect("live temporary path")
    }

    /// Deletes the owned entry, treating an already absent path as success.
    ///
    /// # Errors
    /// Returns the host deletion error. A failed deletion is not retried on drop.
    pub fn delete(mut self) -> io::Result<()> {
        self.remove()
    }

    //# SECURITY-BEGIN [filesystem.temporary-paths]
    fn remove(&mut self) -> io::Result<()> {
        let Some(path) = self.path.take() else {
            return Ok(());
        };
        let result = if self.directory {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };
        match result {
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            result => result,
        }
    }
    //# SECURITY-END [filesystem.temporary-paths]
}

impl Drop for TemporaryPath {
    fn drop(&mut self) {
        let _ = self.remove();
    }
}
