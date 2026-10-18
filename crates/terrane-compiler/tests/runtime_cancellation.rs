#![allow(
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    reason = "the test includes generated task ABIs verbatim rather than exposing a library API"
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

#[derive(Clone, Debug)]
pub struct TerraneError;

fn terrane_time_deadline_expired(_deadline: &terrane_int_support::Int) -> bool {
    false
}

#[expect(
    dead_code,
    reason = "this fixture embeds the complete native task ABI but exercises defensive group drop only"
)]
mod native_group_drop {
    use super::TerraneError;
    include!("../src/runtime/time_inactive.rs");
    include!("../src/runtime/async_native.rs");
    include!("../src/runtime/tasks_native_parallel.rs");
    include!("../src/runtime/task_groups_native.rs");

    #[tokio::test(flavor = "current_thread")]
    async fn drop_group_cancels_before_cleanup() {
        let proof = async {
            let (parent_continued, parent_continued_rx) = tokio::sync::oneshot::channel();
            let (child_started, child_started_rx) = tokio::sync::oneshot::channel();
            let (child_finalizer_entered, child_finalizer_entered_rx) =
                tokio::sync::oneshot::channel();
            let (release_finalizer, release_finalizer_rx) = tokio::sync::oneshot::channel();
            let (child_finalizer_finished, child_finalizer_finished_rx) =
                tokio::sync::oneshot::channel();

            let (finish_parent, finish_parent_rx) = tokio::sync::oneshot::channel();
            let boundary = __terrane_async_entry(async move {
                let group = TerraneTaskGroup::<()>::new();
                let scope = group.scope();
                group.spawn(TerraneScopedTask::spawn(async move {
                    let _ = child_started.send(());
                    scope.cancellation.cancelled().await;
                    let _ = child_finalizer_entered.send(());
                    let _ = release_finalizer_rx.await;
                    let _ = child_finalizer_finished.send(());
                    TerraneTaskResult::Cancelled
                }));
                child_started_rx
                    .await
                    .expect("child starts before group drop");
                drop(group);
                parent_continued
                    .send(())
                    .expect("parent observes continuation before cleanup");
                finish_parent_rx
                    .await
                    .expect("parent remains alive until drop cancellation is observed");
            });
            tokio::pin!(boundary);

            tokio::select! {
                () = &mut boundary => panic!("parent cannot finish before the continuation gate"),
                continued = parent_continued_rx => continued.expect("parent continues after group drop"),
            }
            tokio::select! {
                () = &mut boundary => panic!("parent must remain alive while drop cancellation is observed"),
                entered = child_finalizer_entered_rx => entered.expect("drop requests cancellation promptly"),
            }
            finish_parent
                .send(())
                .expect("parent remains suspended after drop cancellation");
            std::future::poll_fn(|context| {
                assert!(
                    boundary.as_mut().poll(context).is_pending(),
                    "boundary must join the gated child cleanup"
                );
                std::task::Poll::Ready(())
            })
            .await;
            release_finalizer
                .send(())
                .expect("child finalizer is still waiting");
            boundary.await;
            child_finalizer_finished_rx
                .await
                .expect("boundary returns only after child finalizer completes");
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), proof)
            .await
            .expect("group drop and joined cleanup must make progress");
    }
}

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
