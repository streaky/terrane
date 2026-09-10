#![allow(private_bounds)]

mod sealed {
    pub(crate) trait Sealed {}
    impl Sealed for i64 {}
}

pub fn hidden_value<T: sealed::Sealed + Default>() -> T {
    T::default()
}
