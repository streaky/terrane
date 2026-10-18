#![no_std]

pub mod option {
    pub enum Option<T> {
        None,
        Some(T),
    }
}
