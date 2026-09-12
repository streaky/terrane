use std::sync::atomic::{AtomicUsize, Ordering};

use std::future::Future;

pub fn apply_shared<F: Fn(i64) -> i64 + Send + Sync + 'static>(value: i64, callback: F) -> i64 {
    callback(value)
}

pub fn apply_mutable<F: FnMut(i64) -> i64 + Send + 'static>(value: i64, mut callback: F) -> i64 {
    callback(value)
}

pub fn apply_once<F: FnOnce(String) -> String + Send + 'static>(
    value: String,
    callback: F,
) -> String {
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

static ACTIVE_RETAINED_INVOCATIONS: AtomicUsize = AtomicUsize::new(0);

struct ActiveInvocation;

impl Drop for ActiveInvocation {
    fn drop(&mut self) {
        ACTIVE_RETAINED_INVOCATIONS.fetch_sub(1, Ordering::SeqCst);
    }
}

pub async fn invoke_retained<F, Fut>(callback: F) -> i64
where
    F: Fn(i64) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = i64> + Send + 'static,
{
    struct Retained<F>(F);

    let retained = Retained(callback);
    tokio::task::yield_now().await;
    ACTIVE_RETAINED_INVOCATIONS.fetch_add(1, Ordering::SeqCst);
    let _active = ActiveInvocation;
    (retained.0)(7).await
}

pub async fn wait_until_retained_invocation_active() -> bool {
    while ACTIVE_RETAINED_INVOCATIONS.load(Ordering::SeqCst) == 0 {
        tokio::task::yield_now().await;
    }
    true
}

pub fn active_retained_invocations() -> i64 {
    ACTIVE_RETAINED_INVOCATIONS.load(Ordering::SeqCst) as i64
}

pub async fn pending_value(_value: i64) -> i64 {
    std::future::pending().await
}

pub struct Registrar;

impl Registrar {
    pub fn apply<F: Fn(i64) -> i64 + Send + Sync + 'static>(&self, value: i64, callback: F) -> i64 {
        callback(value)
    }
}

pub fn registrar() -> Registrar {
    Registrar
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

pub trait Adjustable: Send + Sync {
    fn adjust(&mut self, delta: i64) -> i64;
    fn current(&self) -> i64;
}

pub fn adjust_value<T: Adjustable>(value: &mut T, delta: i64) -> i64 {
    value.adjust(delta)
}

pub struct AdjustableOwner {
    value: Box<dyn Adjustable>,
}

impl AdjustableOwner {
    pub fn adjust(&mut self, delta: i64) -> i64 {
        self.value.adjust(delta)
    }

    pub fn current(&self) -> i64 {
        self.value.current()
    }
}

pub fn retain_adjustable<T: Adjustable + 'static>(value: T) -> AdjustableOwner {
    AdjustableOwner {
        value: Box::new(value),
    }
}

pub fn retain_boxed_adjustable(value: Box<dyn Adjustable + Send + Sync>) -> AdjustableOwner {
    AdjustableOwner { value }
}

pub trait LocalAdjustable {
    fn adjust(&mut self, delta: i64) -> i64;
    fn current(&self) -> i64;
}

pub struct LocalAdjustableOwner {
    value: Box<dyn LocalAdjustable>,
}

impl LocalAdjustableOwner {
    pub fn adjust(&mut self, delta: i64) -> i64 {
        self.value.adjust(delta)
    }

    pub fn current(&self) -> i64 {
        self.value.current()
    }
}

pub fn retain_local_adjustable<T: LocalAdjustable + 'static>(value: T) -> LocalAdjustableOwner {
    LocalAdjustableOwner {
        value: Box::new(value),
    }
}

pub struct DropToken;

pub fn drop_token() -> DropToken {
    DropToken
}

#[allow(
    drop_bounds,
    reason = "the witness intentionally requires the compiler's canonical Drop mapping"
)]
pub trait DropAware: Drop {
    fn label(&self) -> String;
}

pub fn consume_drop<T: DropAware + 'static>(value: T) -> String {
    value.label()
}

pub use std::ops::Drop as CanonicalDrop;

static ASYNC_ENTRY_CLEANED: AtomicUsize = AtomicUsize::new(0);

#[allow(
    async_fn_in_trait,
    reason = "the witness exercises Rust-owned polling of projected asynchronous methods"
)]
pub trait AsyncEntry: Send + Sync {
    async fn shared(&self) -> i64;
    async fn mutable(&mut self) -> i64;
    async fn consuming(self) -> i64;
}

pub fn record_async_entry_cleanup() {
    ASYNC_ENTRY_CLEANED.fetch_add(1, Ordering::SeqCst);
}

pub fn async_entry_cleanup_count() -> usize {
    ASYNC_ENTRY_CLEANED.load(Ordering::SeqCst)
}

async fn poll_once_and_drop<F: std::future::Future>(future: F) {
    let mut future = Box::pin(future);
    std::future::poll_fn(|context| {
        let _ = future.as_mut().poll(context);
        std::task::Poll::Ready(())
    })
    .await;
}

pub async fn cancel_async_entry<T: AsyncEntry + 'static>(mut value: T) -> bool {
    ASYNC_ENTRY_CLEANED.store(0, Ordering::SeqCst);
    poll_once_and_drop(value.shared()).await;
    poll_once_and_drop(value.mutable()).await;
    poll_once_and_drop(value.consuming()).await;
    true
}

pub fn borrowed_adjustable(value: &dyn Adjustable) -> i64 {
    value.current()
}

pub fn boxed_string(value: Box<String>) -> usize {
    value.len()
}

pub fn return_boxed_adjustable() -> Box<dyn Adjustable + Send + Sync> {
    panic!("projection-only boxed result witness")
}

#[allow(
    async_fn_in_trait,
    reason = "the witness records rejection of non-Send asynchronous projected interfaces"
)]
pub trait LocalAsyncEntry {
    async fn run(&self) -> i64;
}
