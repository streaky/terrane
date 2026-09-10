pub trait Value {
    type Item;
}

impl Value for i64 {
    type Item = i64;
}

pub fn hidden_value<T: Value<Item = String> + Default>() -> T {
    T::default()
}
