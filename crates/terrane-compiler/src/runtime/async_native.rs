use std::future::Future;

#[derive(Clone)]
struct TerraneCancellation {
    state: std::sync::Arc<TerraneCancellationState>,
}

struct TerraneCancellationState {
    cancelled: std::sync::atomic::AtomicBool,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
}

impl TerraneCancellation {
    fn new() -> Self {
        Self {
            state: std::sync::Arc::new(TerraneCancellationState {
                cancelled: std::sync::atomic::AtomicBool::new(false),
                wakers: std::sync::Mutex::new(Vec::new()),
            }),
        }
    }

    fn wake_waiters(&self) {
        let wakers = std::mem::take(
            &mut *self
                .state
                .wakers
                .lock()
                .expect("cancellation waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }

    fn cancel(&self) {
        if !self
            .state
            .cancelled
            .swap(true, std::sync::atomic::Ordering::AcqRel)
        {
            self.wake_waiters();
        }
    }

    fn is_cancelled(&self) -> bool {
        self.state
            .cancelled
            .load(std::sync::atomic::Ordering::Acquire)
    }

    async fn cancelled(&self) {
        std::future::poll_fn(|context| {
            if self.is_cancelled() {
                return std::task::Poll::Ready(());
            }
            let mut wakers = self
                .state
                .wakers
                .lock()
                .expect("cancellation waker lock poisoned");
            if self.is_cancelled() {
                return std::task::Poll::Ready(());
            }
            if !wakers.iter().any(|waker| waker.will_wake(context.waker())) {
                wakers.push(context.waker().clone());
            }
            std::task::Poll::Pending
        })
        .await
    }
}

struct TerraneFinalizerState {
    depth: std::sync::atomic::AtomicUsize,
    wakers: std::sync::Mutex<Vec<std::task::Waker>>,
}

impl TerraneFinalizerState {
    fn new() -> Self {
        Self {
            depth: std::sync::atomic::AtomicUsize::new(0),
            wakers: std::sync::Mutex::new(Vec::new()),
        }
    }

    #[allow(
        dead_code,
        reason = "native scope support is shared by packages without asynchronous finally"
    )]
    fn register(self: &std::sync::Arc<Self>) -> TerraneFinallyGuard {
        let depth = self.depth.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1;
        TerraneFinallyGuard {
            state: Some(self.clone()),
            depth,
        }
    }

    #[allow(
        dead_code,
        reason = "native scope support is shared by packages without asynchronous finally"
    )]
    fn unregister(&self, depth: usize) {
        let current = self.depth.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        debug_assert_eq!(current, depth, "finally regions must leave innermost first");
        let wakers = std::mem::take(
            &mut *self
                .wakers
                .lock()
                .expect("finalizer waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }

    async fn reaches(&self, depth: usize) {
        std::future::poll_fn(|context| {
            if self.depth.load(std::sync::atomic::Ordering::Acquire) == depth {
                return std::task::Poll::Ready(());
            }
            let mut wakers = self
                .wakers
                .lock()
                .expect("finalizer waker lock poisoned");
            if self.depth.load(std::sync::atomic::Ordering::Acquire) == depth {
                return std::task::Poll::Ready(());
            }
            if !wakers.iter().any(|waker| waker.will_wake(context.waker())) {
                wakers.push(context.waker().clone());
            }
            std::task::Poll::Pending
        })
        .await
    }
}

#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneCancellationContext {
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
    finalizers: std::sync::Arc<TerraneFinalizerState>,
}

tokio::task_local! {
    static TERRANE_CANCELLATION_CONTEXT: TerraneCancellationContext;
}

#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
struct TerraneFinallyGuard {
    state: Option<std::sync::Arc<TerraneFinalizerState>>,
    depth: usize,
}

#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
impl TerraneFinallyGuard {
    fn finish(&mut self) {
        if let Some(state) = self.state.take() {
            state.unregister(self.depth);
        }
    }
}

impl Drop for TerraneFinallyGuard {
    fn drop(&mut self) {
        self.finish();
    }
}

#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
fn __terrane_finally_guard() -> TerraneFinallyGuard {
    TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| context.finalizers.register())
        .unwrap_or(TerraneFinallyGuard {
            state: None,
            depth: 0,
        })
}

async fn __terrane_cancellation_requested(
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
) {
    if let Some(deadline) = deadline {
        tokio::select! {
            () = cancellation.cancelled() => {}
            () = tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)) => {
                cancellation.cancel();
            }
        }
    } else {
        cancellation.cancelled().await;
    }
}

#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
async fn __terrane_cancel_operation<F: Future>(
    guard: &TerraneFinallyGuard,
    future: F,
) -> Option<F::Output> {
    let cancellation = TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| {
            guard.state.as_ref().map(|state| {
                (
                    context.cancellation.clone(),
                    context.deadline,
                    state.clone(),
                )
            })
        })
        .ok()
        .flatten();
    if let Some((cancellation, deadline, finalizers)) = cancellation {
        tokio::select! {
            biased;
            () = async {
                __terrane_cancellation_requested(cancellation, deadline).await;
                finalizers.reaches(guard.depth).await;
            } => None,
            output = future => Some(output),
        }
    } else {
        Some(future.await)
    }
}

#[allow(
    dead_code,
    reason = "native scope support is shared by packages without asynchronous finally"
)]
async fn __terrane_finish_cancelled_finally(mut guard: TerraneFinallyGuard) -> ! {
    guard.finish();
    std::future::pending().await
}

async fn __terrane_await<F: Future>(future: F) -> F::Output {
    struct YieldOnce(bool);
    impl Future for YieldOnce {
        type Output = ();

        fn poll(
            mut self: std::pin::Pin<&mut Self>,
            context: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Self::Output> {
            if self.0 {
                std::task::Poll::Ready(())
            } else {
                self.0 = true;
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }

    YieldOnce(false).await;
    let output = future.await;
    YieldOnce(false).await;
    output
}

async fn __terrane_cancellable<F: Future>(
    future: F,
    cancellation: TerraneCancellation,
    deadline: Option<std::time::Instant>,
) -> Option<F::Output> {
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline,
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(context, async {
            tokio::select! {
                biased;
                () = async {
                    __terrane_cancellation_requested(cancellation, deadline).await;
                    finalizers.reaches(0).await;
                } => None,
                output = future => Some(output),
            }
        })
        .await
}
