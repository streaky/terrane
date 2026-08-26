#![allow(
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    reason = "the test includes the generated task ABI verbatim rather than exposing a library API"
)]

use std::{
    future::Future,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    task::{Context, Poll},
};

#[derive(Debug)]
pub struct TerraneError;

include!("../src/runtime/async.rs");
include!("../src/runtime/async_cancellable.rs");
include!("../src/runtime/tasks_threaded.rs");

struct ReadyAfterCancellation {
    reached_await: Option<mpsc::Sender<()>>,
    cancelled: Arc<AtomicBool>,
}

impl Future for ReadyAfterCancellation {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(reached_await) = self.reached_await.take() {
            reached_await.send(()).expect("test receiver remains live");
        }
        while !self.cancelled.load(Ordering::Acquire) {
            std::thread::yield_now();
        }
        Poll::Ready(())
    }
}

#[test]
fn running_child_stops_at_await_after_scope_cancellation() {
    let scope = TerraneTaskScope::new(None);
    let cancelled = scope.cancelled.clone();
    let (reached_await, child_is_waiting) = mpsc::channel();
    let progressed_after_await = Arc::new(AtomicBool::new(false));
    let child_progress = progressed_after_await.clone();
    let child_scope = scope.clone();

    let child = TerraneScopedTask::spawn(move || {
        let future = async move {
            __terrane_await(ReadyAfterCancellation {
                reached_await: Some(reached_await),
                cancelled,
            })
            .await;
            child_progress.store(true, Ordering::Release);
        };
        match __terrane_block_on_cancellable(future, move || child_scope.should_cancel()) {
            Some(()) => TerraneTaskResult::Completed(()),
            None => TerraneTaskResult::Cancelled,
        }
    });

    child_is_waiting
        .recv()
        .expect("child reaches its await before cancellation");
    scope.cancel();
    let outcome = scope.join(child);

    assert!(!outcome.completed);
    assert!(outcome.cancelled);
    assert!(outcome.value.is_none());
    assert!(outcome.error.is_none());
    assert!(!progressed_after_await.load(Ordering::Acquire));
}
