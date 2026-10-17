// Generated deterministically by Terrane <version>.
include!("lower.support.rs");
// Source: case.trn
// Namespace: authored-generic-resource-choice
pub struct Connection {
    pub input: ByteReader,
    pub tag: String,
}
impl Connection {
    pub fn terrane_construct(tag: String) -> Self {
        let mut __terrane_constructed_value = Self {
            input: stdin(),
            tag: String::from(""),
        };
        __terrane_constructed_value.construct(tag);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, tag: String) {
        {
            let __terrane_value = tag;
            self.tag = __terrane_value;
        }
    }
    pub fn destruct(&mut self) {
        println!("{}", terrane_scalar_support::scalar_text(&self.tag));
    }
}
impl Drop for Connection {
    fn drop(&mut self) {
        self.destruct();
    }
}
fn choose<TerraneType54>(
    first: TerraneType54,
    second: TerraneType54,
    use_first: bool,
) -> TerraneType54 {
    if use_first {
        return first;
    }
    return second;
}
fn main() {
    let first: Connection;
    let second: Connection;
    let selected: Connection;
    first = Connection::terrane_construct(String::from("first closed"));
    second = Connection::terrane_construct(String::from("second closed"));
    selected = choose(first, second, true);
    println!("{}", terrane_scalar_support::scalar_text(&selected.tag));
}
