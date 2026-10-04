pub struct Address { pub port: u16 }
pub struct Envelope<T> { pub payload: T }
pub fn consume_address(_: Envelope<Address>) {}
