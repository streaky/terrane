pub trait NumberValue: Sized {
    fn make() -> Self;
}

impl NumberValue for i64 {
    fn make() -> Self {
        7
    }
}

pub fn number_value<T: NumberValue>() -> T {
    T::make()
}
