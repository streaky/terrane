// Generated deterministically by Terrane <version>.
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
            &mut *self.state.wakers.lock().expect("cancellation waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }
    fn cancel(&self) {
        if !self.state.cancelled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            self.wake_waiters();
        }
    }
    fn is_cancelled(&self) -> bool {
        self.state.cancelled.load(std::sync::atomic::Ordering::Acquire)
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
            &mut *self.wakers.lock().expect("finalizer waker lock poisoned"),
        );
        for waker in wakers {
            waker.wake();
        }
    }
    #[allow(
        dead_code,
        reason = "projected cleanup tracking is shared by packages without projected entries"
    )]
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
    fn depth(&self) -> usize {
        self.depth.load(std::sync::atomic::Ordering::Acquire)
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
    static TERRANE_CANCELLATION_CONTEXT : TerraneCancellationContext;
}
static TERRANE_PROJECTED_CLEANUPS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(
    0,
);
static TERRANE_PROJECTED_CLEANUPS_CHANGED: tokio::sync::Notify = tokio::sync::Notify::const_new();
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
struct TerraneProjectedEntryGuard {
    cancellation: TerraneCancellation,
    finalizers: std::sync::Arc<TerraneFinalizerState>,
    abort: Option<tokio::task::AbortHandle>,
    armed: bool,
}
impl Drop for TerraneProjectedEntryGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        TERRANE_PROJECTED_CLEANUPS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.cancellation.cancel();
        let finalizers = self.finalizers.clone();
        let abort = self
            .abort
            .take()
            .expect("armed projected entry guard must own its abort handle");
        tokio::spawn(async move {
            finalizers.reaches(0).await;
            abort.abort();
            if TERRANE_PROJECTED_CLEANUPS
                .fetch_sub(1, std::sync::atomic::Ordering::SeqCst) == 1
            {
                TERRANE_PROJECTED_CLEANUPS_CHANGED.notify_waiters();
            }
        });
    }
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
async fn __terrane_projected_async_entry<F, T>(future: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let cancellation = TerraneCancellation::new();
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline: None,
        finalizers: finalizers.clone(),
    };
    let mut future = Box::pin(TERRANE_CANCELLATION_CONTEXT.scope(context, future));
    let first_poll = std::future::poll_fn(|cx| std::task::Poll::Ready(
            future.as_mut().poll(cx),
        ))
        .await;
    if let std::task::Poll::Ready(output) = first_poll {
        return output;
    }
    let task = tokio::spawn(future);
    let mut guard = TerraneProjectedEntryGuard {
        cancellation,
        finalizers,
        abort: Some(task.abort_handle()),
        armed: true,
    };
    let output = task
        .await
        .expect("projected asynchronous dependency entry task failed");
    guard.armed = false;
    output
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
async fn __terrane_wait_projected_cleanups() {
    loop {
        let changed = TERRANE_PROJECTED_CLEANUPS_CHANGED.notified();
        if TERRANE_PROJECTED_CLEANUPS.load(std::sync::atomic::Ordering::SeqCst) == 0 {
            return;
        }
        changed.await;
    }
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
#[derive(Clone)]
struct TerraneSelectControl {
    requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
    waker: std::sync::Arc<std::sync::Mutex<Option<std::task::Waker>>>,
}
impl TerraneSelectControl {
    fn request_cancel(&self) {
        self.requested.store(true, std::sync::atomic::Ordering::Release);
        if let Some(waker) = self
            .waker
            .lock()
            .expect("select cancellation waker lock poisoned")
            .take()
        {
            waker.wake();
        }
    }
    fn is_cancelled(&self) -> bool {
        self.requested.load(std::sync::atomic::Ordering::Acquire)
    }
}
fn __terrane_select_control() -> TerraneSelectControl {
    TerraneSelectControl {
        requested: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: std::sync::Arc::new(std::sync::Mutex::new(None)),
    }
}
async fn __terrane_select_operation<F: Future>(
    control: TerraneSelectControl,
    future: F,
) -> Option<F::Output> {
    let parent = TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| (context.cancellation.clone(), context.deadline))
        .ok();
    let cancellation = TerraneCancellation::new();
    let finalizers = std::sync::Arc::new(TerraneFinalizerState::new());
    let context = TerraneCancellationContext {
        cancellation: cancellation.clone(),
        deadline: parent.as_ref().and_then(|(_, deadline)| *deadline),
        finalizers: finalizers.clone(),
    };
    TERRANE_CANCELLATION_CONTEXT
        .scope(
            context,
            async move {
                let mut future = std::pin::pin!(future);
                std::future::poll_fn(move |cx| {
                        let parent_cancelled = parent
                            .as_ref()
                            .is_some_and(|(parent, _)| parent.is_cancelled());
                        if control.is_cancelled() || parent_cancelled {
                            cancellation.cancel();
                            let mut stored_waker = control
                                .waker
                                .lock()
                                .expect("select cancellation waker lock poisoned");
                            if stored_waker
                                .as_ref()
                                .is_none_or(|waker| !waker.will_wake(cx.waker()))
                            {
                                *stored_waker = Some(cx.waker().clone());
                            }
                            drop(stored_waker);
                            let _ = Future::poll(future.as_mut(), cx);
                            if finalizers.depth() == 0 {
                                return std::task::Poll::Ready(None);
                            }
                            return std::task::Poll::Pending;
                        }
                        Future::poll(future.as_mut(), cx).map(Some)
                    })
                    .await
            },
        )
        .await
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
            () = cancellation.cancelled() => {} () =
            tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)) => {
            cancellation.cancel(); }
        }
    } else {
        cancellation.cancelled().await;
    }
}
fn __terrane_cancellation_is_requested() -> bool {
    TERRANE_CANCELLATION_CONTEXT
        .try_with(|context| {
            if context
                .deadline
                .is_some_and(|deadline| std::time::Instant::now() >= deadline)
            {
                context.cancellation.cancel();
            }
            context.cancellation.is_cancelled()
        })
        .unwrap_or(false)
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
            guard
                .state
                .as_ref()
                .map(|state| {
                    (context.cancellation.clone(), context.deadline, state.clone())
                })
        })
        .ok()
        .flatten();
    if let Some((cancellation, deadline, finalizers)) = cancellation {
        let mut future = std::pin::pin!(future);
        tokio::select! {
            biased; output = future.as_mut() => Some(output), () =
            __terrane_cancellation_requested(cancellation, deadline) => {
            std::future::poll_fn(| cx | { let _ = Future::poll(future.as_mut(), cx); if
            finalizers.depth() == guard.depth { std::task::Poll::Ready(()) } else {
            std::task::Poll::Pending } }). await; None },
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
async fn __terrane_finish_cancelled_select(mut guard: TerraneFinallyGuard) -> ! {
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
        .scope(
            context,
            async {
                let mut future = std::pin::pin!(future);
                tokio::select! {
                    biased; output = future.as_mut() => Some(output), () =
                    __terrane_cancellation_requested(cancellation, deadline) => {
                    std::future::poll_fn(| cx | { let _ = Future::poll(future.as_mut(),
                    cx); if finalizers.depth() == 0 { std::task::Poll::Ready(()) } else {
                    std::task::Poll::Pending } }). await; None },
                }
            },
        )
        .await
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize")
        .block_on(async move {
            let output = future.await;
            __terrane_wait_projected_cleanups().await;
            output
        })
}
// Source: case.trn
// Namespace: heterogeneous-select
fn mark(value: String) -> String {
    println!("{}", terrane_scalar_support::scalar_text(&value));
    return value;
}
async fn number_task(marker: String) -> terrane_int_support::Int {
    let _ = &marker;
    return terrane_int_support::Int::from(7_i128);
}
async fn text_task(marker: String) -> String {
    let _ = &marker;
    return String::from("text");
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_275 = 0usize;
        let mut count: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        while count.clone() < terrane_int_support::Int::from(4_i128) {
            let mut __terrane_select_guard_275 = __terrane_finally_guard();
            let __terrane_select_control_275_0 = __terrane_select_control();
            let mut __terrane_select_future_275_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_275_0.clone(),
                number_task(mark(String::from("construct-a"))))
            );
            let mut __terrane_select_result_275_0 = None;
            let mut __terrane_select_cancelled_275_0 = false;
            let __terrane_select_control_275_1 = __terrane_select_control();
            let mut __terrane_select_future_275_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_275_1.clone(),
                text_task(mark(String::from("construct-b"))))
            );
            let mut __terrane_select_result_275_1 = None;
            let mut __terrane_select_cancelled_275_1 = false;
            let __terrane_select_winner_275 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_275
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_275_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_275_0 = Some(
                                            __terrane_select_value,
                                        );
                                        return std::task::Poll::Ready(0usize);
                                    }
                                    std::task::Poll::Ready(None) => {
                                        __terrane_select_cancelled_275_0 = true;
                                    }
                                    std::task::Poll::Pending => {}
                                }
                            }
                            1 => {
                                match Future::poll(
                                    __terrane_select_future_275_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_275_1 = Some(
                                            __terrane_select_value,
                                        );
                                        return std::task::Poll::Ready(1usize);
                                    }
                                    std::task::Poll::Ready(None) => {
                                        __terrane_select_cancelled_275_1 = true;
                                    }
                                    std::task::Poll::Pending => {}
                                }
                            }
                            _ => {
                                unreachable!("select candidate is within the case count")
                            }
                        }
                    }
                    if __terrane_select_cancelled_275_0
                        && __terrane_select_cancelled_275_1
                    {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    std::task::Poll::Pending
                })
                .await;
            if __terrane_select_winner_275 == usize::MAX {
                __terrane_select_control_275_1.request_cancel();
                __terrane_select_control_275_0.request_cancel();
                if !__terrane_select_cancelled_275_1 {
                    debug_assert!(
                        __terrane_select_future_275_1.as_mut(). await .is_none()
                    );
                }
                if !__terrane_select_cancelled_275_0 {
                    debug_assert!(
                        __terrane_select_future_275_0.as_mut(). await .is_none()
                    );
                }
                drop(__terrane_select_future_275_1);
                drop(__terrane_select_future_275_0);
                __terrane_wait_projected_cleanups().await;
                __terrane_finish_cancelled_select(__terrane_select_guard_275).await;
            }
            __terrane_select_cursor_275 = (__terrane_select_winner_275 + 1usize)
                % 2usize;
            match __terrane_select_winner_275 {
                0 => {
                    __terrane_select_control_275_1.request_cancel();
                    debug_assert!(
                        __terrane_select_future_275_1.as_mut(). await .is_none()
                    );
                }
                1 => {
                    __terrane_select_control_275_0.request_cancel();
                    debug_assert!(
                        __terrane_select_future_275_0.as_mut(). await .is_none()
                    );
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            drop(__terrane_select_future_275_1);
            drop(__terrane_select_future_275_0);
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_275.finish();
            match __terrane_select_winner_275 {
                0 => {
                    let number: terrane_int_support::Int = __terrane_select_result_275_0
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&number));
                }
                1 => {
                    let text: String = __terrane_select_result_275_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&text));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            count = count.clone() + terrane_int_support::Int::from(1_i128);
        }
    });
}
