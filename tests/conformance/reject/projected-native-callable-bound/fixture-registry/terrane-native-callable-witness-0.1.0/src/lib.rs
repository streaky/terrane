pub struct Cell<T> { value: T }
pub fn make<T: Default + Copy>() -> Cell<T> { Cell { value: T::default() } }
