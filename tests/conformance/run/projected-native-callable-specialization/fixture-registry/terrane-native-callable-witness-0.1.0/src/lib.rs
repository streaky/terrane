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

#[expect(non_camel_case_types, reason = "exercise owner binder identity containing the internal separator")]
pub struct Holder<T__Inner> {
    pub value: T__Inner,
}

impl<A> Holder<A> {
    #[expect(non_camel_case_types, reason = "exercise shadowing of a separator-containing nominal binder")]
    pub fn make<T__Inner: Default>() -> T__Inner { T__Inner::default() }

    pub fn with_owner<U: From<A>>(value: A) -> U { U::from(value) }

}

pub struct Factory;

impl Factory {
    pub fn new() -> Self { Self }

    pub fn create<T: Default + Copy>(&self) -> Cell<T> {
        Cell { value: T::default() }
    }

    pub async fn create_async<T: Copy>(&self, value: T) -> Cell<T> {
        Cell { value }
    }

    pub unsafe fn unsafe_create<T: Default + Copy>(&self) -> Cell<T> {
        Cell { value: T::default() }
    }
}

pub fn make<T: Default + Copy>() -> Cell<T> {
    Cell { value: T::default() }
}

pub async fn make_async<T: Default + Copy>() -> Cell<T> {
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

pub mod left {
    #[derive(Clone, Copy)]
    pub struct Left;
    impl Left {
        pub fn next(&self) -> super::right::Right { super::right::Right }
        pub fn is_left(&self) -> bool { true }
    }
}

pub mod right {
    #[derive(Clone, Copy)]
    pub struct Right;
    impl Right {
        pub fn next(&self) -> super::left::Left { super::left::Left }
    }
}

pub fn cycle() -> left::Left { left::Left }
