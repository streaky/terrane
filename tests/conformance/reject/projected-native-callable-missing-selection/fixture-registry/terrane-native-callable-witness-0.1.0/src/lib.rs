#[derive(Clone, Copy, Default)]
pub struct Marker(pub i64);
pub struct Cell<T> { value: T }
pub fn make<T: Default + Copy>() -> Cell<T> { Cell { value: T::default() } }
