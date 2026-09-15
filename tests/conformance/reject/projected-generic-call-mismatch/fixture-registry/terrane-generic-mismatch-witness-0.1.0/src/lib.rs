pub fn open_callback<T, F: FnOnce(T) -> T>(value: T, callback: F) -> T {
    callback(value)
}
