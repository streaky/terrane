use std::future::Future;

pub fn apply_shared<F: Fn(i64) -> i64 + Send + Sync + 'static>(value: i64, callback: F) -> i64 {
    callback(value)
}

pub fn apply_mutable<F: FnMut(i64) -> i64 + Send + 'static>(value: i64, mut callback: F) -> i64 {
    callback(value)
}

pub fn apply_once<F: FnOnce(String) -> String + Send + 'static>(value: String, callback: F) -> String {
    callback(value)
}

pub async fn apply_async<
    F: Fn(String) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = String> + Send + 'static,
>(
    value: String,
    callback: F,
) -> String {
    callback(value).await
}

pub async fn apply_async_concurrently<F, Fut>(value: i64, callback: F) -> i64
where
    F: Fn(i64) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = i64> + Send + 'static,
{
    let first_callback = callback.clone();
    let first = tokio::spawn(async move { first_callback(value).await });
    let second = tokio::spawn(async move { callback(value + 1).await });
    first.await.expect("callback task") + second.await.expect("callback task")
}

#[derive(Clone)]
pub struct Marker(i64);

impl Marker {
    pub fn value(&self) -> i64 {
        self.0
    }
}

pub fn make_marker(value: i64) -> Marker {
    Marker(value)
}

pub fn open_callback<T, F: Fn(T) -> T>(value: T, callback: F) -> T {
    callback(value)
}
