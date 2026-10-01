pub fn produce<T, F>(callback: F) -> T
where
    F: Fn() -> T,
{
    callback()
}

pub fn transform<T, U, F>(value: T, callback: F) -> U
where
    F: Fn(T) -> U,
{
    callback(value)
}
