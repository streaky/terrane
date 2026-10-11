pub fn fail() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(std::io::ErrorKind::AddrInUse, "occupied"))
}
