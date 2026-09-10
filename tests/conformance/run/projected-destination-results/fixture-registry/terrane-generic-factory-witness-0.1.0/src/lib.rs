pub fn default_value<T: Default>() -> T {
    T::default()
}

pub fn optional_value<T: Default>() -> Option<T> {
    Some(T::default())
}
