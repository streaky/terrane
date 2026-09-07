use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct YieldOnce(bool);

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

struct PendingOperation;

impl Future for PendingOperation {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for PendingOperation {
    fn drop(&mut self) {
        OPERATION_DROPS.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }
}

static OPERATION_STARTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
static OPERATION_DROPS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

pub fn reset_operation_state() {
    OPERATION_STARTED.store(false, std::sync::atomic::Ordering::Release);
    OPERATION_DROPS.store(0, std::sync::atomic::Ordering::Release);
}

pub async fn wait_forever() -> String {
    OPERATION_STARTED.store(true, std::sync::atomic::Ordering::Release);
    PendingOperation.await;
    unreachable!("pending operation completed")
}

pub async fn wait_until_operation_started() -> bool {
    while !OPERATION_STARTED.load(std::sync::atomic::Ordering::Acquire) {
        YieldOnce(false).await;
    }
    true
}

pub fn operation_drop_count() -> u64 {
    OPERATION_DROPS.load(std::sync::atomic::Ordering::Acquire)
}

pub async fn echo_after_yield(value: String) -> String {
    YieldOnce(false).await;
    value
}
