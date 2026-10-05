#[derive(Clone, Copy, Default)]
pub struct Marker(pub i64);

impl Marker {
    pub fn value(&self) -> i64 { self.0 }
}

pub struct Cell<T> {
    value: T,
}

impl<T> Cell<T> {
    pub fn get(&self) -> Option<&T> { Some(&self.value) }
}

pub struct Factory;

impl Factory {
    pub fn new() -> Self { Self }

    pub fn create<T: Default + Copy>(&self) -> Cell<T> {
        Cell { value: T::default() }
    }
}

pub fn make<T: Default + Copy>() -> Cell<T> {
    Cell { value: T::default() }
}

pub fn echo<T: Copy>(value: T) -> Cell<T> { Cell { value } }

pub fn consume<T: Copy>(_: &Cell<T>) -> i64 {
    std::mem::size_of::<T>() as i64
}

pub fn marker_value(marker: &Marker) -> i64 { marker.value() }

pub fn ordered<Z: Copy, A: Copy>(first: Z, second: A) -> i64 {
    (std::mem::size_of_val(&first) * 10 + std::mem::size_of_val(&second)) as i64
}

pub fn mixed<T: Copy, U: Default>(_: T) -> Cell<U> {
    Cell { value: U::default() }
}

pub fn byte() -> u8 { 23 }

pub unsafe fn make_unsafe<T: Default + Copy>() -> Cell<T> {
    Cell { value: T::default() }
}
