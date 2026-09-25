pub fn produce<T, F>(callback: F) -> T
where
    F: Fn() -> T,
{
    callback()
}
