#[derive(Clone)]
pub struct Opaque<T>(pub T);

pub fn make_text() -> Opaque<String> {
    Opaque(String::from("cloneable"))
}

pub fn consume_text(value: Opaque<String>) -> String {
    value.0
}
