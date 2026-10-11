pub use std::io::{Error, ErrorKind};

pub fn fail() -> Result<(), Error> {
    Err(Error::new(ErrorKind::AddrInUse, "occupied"))
}

pub fn make_error() -> Error {
    Error::new(ErrorKind::AddrInUse, "occupied")
}

pub fn is_occupied(kind: ErrorKind) -> bool {
    kind == ErrorKind::AddrInUse
}
