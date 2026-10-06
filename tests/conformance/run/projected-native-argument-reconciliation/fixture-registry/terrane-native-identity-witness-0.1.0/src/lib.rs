pub struct Boxed<T> {
    value: T,
}

impl<T> Boxed<T> {
    pub fn into_value(self) -> T { self.value }
}

pub fn make_i128() -> Boxed<i128> { Boxed { value: 201 } }
pub fn consume_native<T>(value: Boxed<T>) -> T { value.into_value() }
