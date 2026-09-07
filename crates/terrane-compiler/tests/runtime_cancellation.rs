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

struct ReadyOnRePoll {
    polled: bool,
    cancelled: Arc<AtomicBool>,
}

impl Future for ReadyOnRePoll {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.polled {
            Poll::Ready(7)
        } else {
            self.polled = true;
            self.cancelled.store(true, Ordering::Release);
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[test]
fn ready_woken_completion_wins_over_cancellation_before_repoll() {
    let cancelled = Arc::new(AtomicBool::new(false));
    let future = ReadyOnRePoll {
        polled: false,
        cancelled: cancelled.clone(),
    };

    assert_eq!(
        __terrane_block_on_cancellable(future, || cancelled.load(Ordering::Acquire)),
        Some(7)
    );
}

#[test]
fn running_child_keeps_ready_await_completion_after_scope_cancellation() {
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

    assert!(outcome.completed);
    assert!(outcome.cancelled);
    assert!(outcome.value.is_some());
    assert!(outcome.error.is_none());
    assert!(progressed_after_await.load(Ordering::Acquire));
}
