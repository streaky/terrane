//! Minimal host-ABI boundary for process standard streams.
//!
//! This crate exists because acquiring and operating the process-owned standard
//! handles crosses the host ABI. Stream policy, text encoding, convenience
//! operations, cancellation reporting, and the public object model belong in
//! Terrane source above this boundary.

use std::io::{self, Read, Write};

#[derive(Debug)]
pub struct ReadOutcome {
    pub data: Vec<u8>,
    pub end: bool,
}

#[derive(Debug)]
pub struct StdinReader {
    stdin: io::Stdin,
    closed: bool,
}

impl StdinReader {
    #[must_use]
    pub fn acquire() -> Self {
        Self {
            stdin: io::stdin(),
            closed: false,
        }
    }

    /// Performs at most one host read, preserving partial completion and EOF.
    pub fn read(&mut self, limit: usize) -> io::Result<ReadOutcome> {
        self.ensure_open()?;
        let mut data = vec![0; limit];
        let completed = self.stdin.read(&mut data)?;
        data.truncate(completed);
        Ok(ReadOutcome {
            data,
            end: completed == 0,
        })
    }

    /// Releases this wrapper. The process-owned standard handle itself remains owned by the host.
    pub fn close(&mut self) -> io::Result<()> {
        self.closed = true;
        Ok(())
    }

    fn ensure_open(&self) -> io::Result<()> {
        if self.closed {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "standard input is closed",
            ))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct StdoutWriter {
    stdout: io::Stdout,
    closed: bool,
}

#[derive(Debug)]
pub struct StderrWriter {
    stderr: io::Stderr,
    closed: bool,
}

macro_rules! impl_standard_writer {
    ($type:ty, $field:ident, $acquire:expr, $closed_message:literal) => {
        impl $type {
            #[must_use]
            pub fn acquire() -> Self {
                Self {
                    $field: $acquire,
                    closed: false,
                }
            }

            /// Performs at most one host write and returns its partial completion count.
            pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
                self.ensure_open()?;
                self.$field.write(data)
            }

            pub fn flush(&mut self) -> io::Result<()> {
                self.ensure_open()?;
                self.$field.flush()
            }

            /// Standard process streams do not expose filesystem data durability.
            pub fn sync_data(&mut self) -> io::Result<()> {
                self.ensure_open()?;
                Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "standard streams do not support sync-data",
                ))
            }

            /// Standard process streams do not expose filesystem metadata durability.
            pub fn sync_all(&mut self) -> io::Result<()> {
                self.ensure_open()?;
                Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "standard streams do not support sync-all",
                ))
            }

            /// Flushes once and releases this wrapper. Repeated close is successful.
            pub fn close(&mut self) -> io::Result<()> {
                if self.closed {
                    return Ok(());
                }
                self.$field.flush()?;
                self.closed = true;
                Ok(())
            }

            fn ensure_open(&self) -> io::Result<()> {
                if self.closed {
                    Err(io::Error::new(io::ErrorKind::NotConnected, $closed_message))
                } else {
                    Ok(())
                }
            }
        }
    };
}

impl_standard_writer!(StdoutWriter, stdout, io::stdout(), "standard output is closed");
impl_standard_writer!(StderrWriter, stderr, io::stderr(), "standard error is closed");
