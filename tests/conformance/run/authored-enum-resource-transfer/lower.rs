// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: authored-enum-resource-transfer
pub struct Connection {
    pub input: ByteReader,
}
impl Connection {
    pub fn terrane_construct() -> Self {
        Self { input: stdin() }
    }
    pub fn destruct(&mut self) {
        println!("{}", String::from("connection closed"));
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        self.destruct();
    }
}
pub enum AcquiredConnection {
    Unavailable,
    Available(Connection),
}
fn inspect(acquired: &AcquiredConnection) -> bool {
    match &acquired {
        AcquiredConnection::Unavailable => {
            return false;
        }
        AcquiredConnection::Available(_) => {
            return true;
        }
    }
}
fn extract(acquired: AcquiredConnection) -> Option<Connection> {
    let value: Connection;
    let __terrane_match_value_568 = acquired;
    match __terrane_match_value_568 {
        AcquiredConnection::Unavailable => {
            return None;
        }
        AcquiredConnection::Available(__terrane_pattern_666_value) => {
            value = __terrane_pattern_666_value;
            return Some(value);
        }
    }
}
fn release(value: Option<Connection>) {
    let _ = &value;
    return ();
}
fn main() {
    let acquired: AcquiredConnection;
    let result: Option<Connection>;
    acquired = {
        let __terrane_enum_payload_0 = Connection::terrane_construct();
        AcquiredConnection::Available(__terrane_enum_payload_0)
    };
    println!("{}", terrane_scalar_support::scalar_text(&inspect(&acquired)));
    result = extract(acquired);
    release(result);
}
