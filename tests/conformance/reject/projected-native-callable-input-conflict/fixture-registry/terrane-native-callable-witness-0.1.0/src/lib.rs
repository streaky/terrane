#[derive(Clone, Copy, Default)]
pub struct Marker(pub i64);
pub struct Cell<T> { value: T }
pub fn echo<T: Copy>(value: T) -> Cell<T> { Cell { value } }
