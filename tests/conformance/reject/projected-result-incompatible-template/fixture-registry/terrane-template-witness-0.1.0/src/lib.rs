pub fn optional_value<T: Default>() -> Option<T> {
    Some(T::default())
}
