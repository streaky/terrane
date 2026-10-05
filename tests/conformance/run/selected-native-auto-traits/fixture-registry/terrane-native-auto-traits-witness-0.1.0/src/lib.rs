pub struct Opaque<T = String>(pub T);

pub fn make_text() -> Opaque<String> {
    Opaque(String::from("send and sync"))
}

pub trait Worker: Send + Sync {
    fn work(&self) -> bool;
}
