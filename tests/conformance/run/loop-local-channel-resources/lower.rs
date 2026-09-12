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
    static TERRANE_CANCELLATION_CONTEXT : TerraneCancellationContext;
}
#[allow(
    dead_code,
    reason = "projected asynchronous entry support is shared by asynchronous packages"
)]
struct TerraneProjectedEntryGuard {
    cancellation: TerraneCancellation,
    armed: bool,
}
impl Drop for TerraneProjectedEntryGuard {
    fn drop(&mut self) {
        if self.armed {
            self.cancellation.cancel();
        }
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
    let abort = task.abort_handle();
    let cleanup_cancellation = cancellation.clone();
    tokio::spawn(async move {
        cleanup_cancellation.cancelled().await;
        finalizers.reaches(0).await;
        abort.abort();
    });
    let mut guard = TerraneProjectedEntryGuard {
        cancellation,
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
            () = cancellation.cancelled() => {} () =
            tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)) => {
            cancellation.cancel(); }
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
        tokio::select! {
            biased; output = future => Some(output), () = async {
            __terrane_cancellation_requested(cancellation, deadline). await; finalizers
            .reaches(guard.depth). await; } => None,
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
        .scope(
            context,
            async {
                tokio::select! {
                    biased; output = future => Some(output), () = async {
                    __terrane_cancellation_requested(cancellation, deadline). await;
                    finalizers.reaches(0). await; } => None,
                }
            },
        )
        .await
}
fn __terrane_run<F: Future>(future: F) -> F::Output {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Terrane async runtime must initialize");
    tokio::task::LocalSet::new().block_on(&runtime, future)
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerraneChannelOverflow {
    Block,
    FailSend,
    DropNewest,
    DropOldest,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneChannelSendOutcome<T> {
    pub accepted: bool,
    pub closed: bool,
    pub dropped: bool,
    pub rejected_value: Option<T>,
    pub dropped_value: Option<T>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneChannelReceiveOutcome<T> {
    pub available: bool,
    pub closed: bool,
    pub value: Option<T>,
}
struct TerraneChannelState<T> {
    values: std::collections::VecDeque<T>,
    capacity: usize,
    overflow: TerraneChannelOverflow,
    sender_closed: bool,
    receiver_closed: bool,
    next_waiter: usize,
    sender_wakers: std::collections::BTreeMap<usize, std::task::Waker>,
    receiver_wakers: std::collections::BTreeMap<usize, std::task::Waker>,
    rendezvous_values: std::collections::BTreeMap<usize, T>,
    rendezvous_completed: std::collections::BTreeSet<usize>,
}
pub struct TerraneChannelSender<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
}
pub struct TerraneChannelReceiver<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
}
pub struct TerraneChannelPair<T> {
    pub sender: TerraneChannelSender<T>,
    pub receiver: TerraneChannelReceiver<T>,
}
impl<T> TerraneChannelPair<T> {
    pub fn new(capacity: usize, overflow: TerraneChannelOverflow) -> Self {
        let state = std::sync::Arc::new(
            std::sync::Mutex::new(TerraneChannelState {
                values: std::collections::VecDeque::with_capacity(capacity),
                capacity,
                overflow,
                sender_closed: false,
                receiver_closed: false,
                next_waiter: 0,
                sender_wakers: std::collections::BTreeMap::new(),
                receiver_wakers: std::collections::BTreeMap::new(),
                rendezvous_values: std::collections::BTreeMap::new(),
                rendezvous_completed: std::collections::BTreeSet::new(),
            }),
        );
        Self {
            sender: TerraneChannelSender {
                state: state.clone(),
            },
            receiver: TerraneChannelReceiver { state },
        }
    }
}
impl<T> TerraneChannelSender<T> {
    pub fn send(&self, value: T) -> TerraneChannelSend<T> {
        TerraneChannelSend {
            state: self.state.clone(),
            value: Some(value),
            waiter_id: None,
        }
    }
    pub fn close(self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.sender_closed = true;
        for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
            waker.wake();
        }
    }
}
impl<T> Drop for TerraneChannelSender<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.sender_closed = true;
        for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
            waker.wake();
        }
    }
}
pub struct TerraneChannelSend<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
    value: Option<T>,
    waiter_id: Option<usize>,
}
impl<T: Unpin> std::future::Future for TerraneChannelSend<T> {
    type Output = TerraneChannelSendOutcome<T>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let state_ref = self.state.clone();
        let mut state = state_ref.lock().expect("channel state lock poisoned");
        if let Some(waiter_id) = self.waiter_id
            && state.rendezvous_completed.remove(&waiter_id)
        {
            state.sender_wakers.remove(&waiter_id);
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: true,
                closed: false,
                dropped: false,
                rejected_value: None,
                dropped_value: None,
            });
        }
        if state.receiver_closed {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.sender_wakers.remove(&waiter_id);
                if self.value.is_none() {
                    self.value = state.rendezvous_values.remove(&waiter_id);
                }
                state.rendezvous_completed.remove(&waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: false,
                closed: true,
                dropped: false,
                rejected_value: self.value.take(),
                dropped_value: None,
            });
        }
        if state.capacity == 0 {
            let waiter_id = self
                .waiter_id
                .unwrap_or_else(|| {
                    let waiter_id = state.next_waiter;
                    state.next_waiter += 1;
                    self.waiter_id = Some(waiter_id);
                    waiter_id
                });
            if let Some(value) = self.value.take() {
                state.rendezvous_values.insert(waiter_id, value);
            }
            state.sender_wakers.insert(waiter_id, context.waker().clone());
            for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                waker.wake();
            }
            return std::task::Poll::Pending;
        }
        if state.values.len() < state.capacity {
            state
                .values
                .push_back(self.value.take().expect("send polled after completion"));
            if let Some(waiter_id) = self.waiter_id.take() {
                state.sender_wakers.remove(&waiter_id);
            }
            for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                waker.wake();
            }
            return std::task::Poll::Ready(TerraneChannelSendOutcome {
                accepted: true,
                closed: false,
                dropped: false,
                rejected_value: None,
                dropped_value: None,
            });
        }
        match state.overflow {
            TerraneChannelOverflow::Block => {
                let waiter_id = self
                    .waiter_id
                    .unwrap_or_else(|| {
                        let waiter_id = state.next_waiter;
                        state.next_waiter += 1;
                        self.waiter_id = Some(waiter_id);
                        waiter_id
                    });
                state.sender_wakers.insert(waiter_id, context.waker().clone());
                std::task::Poll::Pending
            }
            TerraneChannelOverflow::FailSend => {
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: false,
                    closed: false,
                    dropped: false,
                    rejected_value: self.value.take(),
                    dropped_value: None,
                })
            }
            TerraneChannelOverflow::DropNewest => {
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: false,
                    closed: false,
                    dropped: true,
                    rejected_value: None,
                    dropped_value: self.value.take(),
                })
            }
            TerraneChannelOverflow::DropOldest => {
                let dropped_value = state.values.pop_front();
                state
                    .values
                    .push_back(self.value.take().expect("send polled after completion"));
                for (_, waker) in std::mem::take(&mut state.receiver_wakers) {
                    waker.wake();
                }
                std::task::Poll::Ready(TerraneChannelSendOutcome {
                    accepted: true,
                    closed: false,
                    dropped: true,
                    rejected_value: None,
                    dropped_value,
                })
            }
        }
    }
}
impl<T> Drop for TerraneChannelSend<T> {
    fn drop(&mut self) {
        if let Some(waiter_id) = self.waiter_id {
            let mut state = self.state.lock().expect("channel state lock poisoned");
            state.sender_wakers.remove(&waiter_id);
            state.rendezvous_values.remove(&waiter_id);
            state.rendezvous_completed.remove(&waiter_id);
        }
    }
}
impl<T> TerraneChannelReceiver<T> {
    pub fn receive(&self) -> TerraneChannelReceive<T> {
        TerraneChannelReceive {
            state: self.state.clone(),
            waiter_id: None,
        }
    }
    pub fn close(self) -> terrane_collection_support::List<T> {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.receiver_closed = true;
        let remaining = terrane_collection_support::List::new(
            state.values.drain(..).collect(),
        );
        for (_, waker) in std::mem::take(&mut state.sender_wakers) {
            waker.wake();
        }
        remaining
    }
}
impl<T> Drop for TerraneChannelReceiver<T> {
    fn drop(&mut self) {
        let mut state = self.state.lock().expect("channel state lock poisoned");
        state.receiver_closed = true;
        state.values.clear();
        for (_, waker) in std::mem::take(&mut state.sender_wakers) {
            waker.wake();
        }
    }
}
pub struct TerraneChannelReceive<T> {
    state: std::sync::Arc<std::sync::Mutex<TerraneChannelState<T>>>,
    waiter_id: Option<usize>,
}
impl<T> std::future::Future for TerraneChannelReceive<T> {
    type Output = TerraneChannelReceiveOutcome<T>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let state_ref = self.state.clone();
        let mut state = state_ref.lock().expect("channel state lock poisoned");
        if state.capacity == 0
            && let Some(waiter_id) = state.rendezvous_values.keys().next().copied()
        {
            let value = state
                .rendezvous_values
                .remove(&waiter_id)
                .expect("rendezvous value disappeared");
            state.rendezvous_completed.insert(waiter_id);
            if let Some(waker) = state.sender_wakers.remove(&waiter_id) {
                waker.wake();
            }
            if let Some(receiver_waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&receiver_waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: true,
                closed: false,
                value: Some(value),
            });
        }
        if let Some(value) = state.values.pop_front() {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&waiter_id);
            }
            for (_, waker) in std::mem::take(&mut state.sender_wakers) {
                waker.wake();
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: true,
                closed: false,
                value: Some(value),
            });
        }
        if state.sender_closed {
            if let Some(waiter_id) = self.waiter_id.take() {
                state.receiver_wakers.remove(&waiter_id);
            }
            return std::task::Poll::Ready(TerraneChannelReceiveOutcome {
                available: false,
                closed: true,
                value: None,
            });
        }
        let waiter_id = self
            .waiter_id
            .unwrap_or_else(|| {
                let waiter_id = state.next_waiter;
                state.next_waiter += 1;
                self.waiter_id = Some(waiter_id);
                waiter_id
            });
        state.receiver_wakers.insert(waiter_id, context.waker().clone());
        for waker in state.sender_wakers.values() {
            waker.wake_by_ref();
        }
        std::task::Poll::Pending
    }
}
impl<T> Unpin for TerraneChannelReceive<T> {}
impl<T> Drop for TerraneChannelReceive<T> {
    fn drop(&mut self) {
        if let Some(waiter_id) = self.waiter_id {
            self.state
                .lock()
                .expect("channel state lock poisoned")
                .receiver_wakers
                .remove(&waiter_id);
        }
    }
}
pub type TerranePlatformCapability = terrane_platform_support::Capability;
pub type TerranePlatformResult = terrane_platform_support::ResultValue;
pub fn terrane_platform_i128(
    value: &terrane_int_support::Int,
    label: &str,
) -> Result<i128, TerranePlatformResult> {
    terrane_int_support::coerce::<i128>(value)
        .map_err(|_| TerranePlatformResult::error(
            format!("{label} is outside the signed 128-bit platform range"),
        ))
}
macro_rules! terrane_platform_i128 {
    ($value:expr, $label:literal) => {
        match terrane_platform_i128(&$value, $label) { Ok(value) => value, Err(error) =>
        return error, }
    };
}
#[allow(dead_code)]
fn terrane_platform_cancellation_token() -> TerranePlatformCapability {
    terrane_platform_support::cancellation_token()
}
#[allow(dead_code)]
fn terrane_platform_no_resource() -> TerranePlatformCapability {
    TerranePlatformCapability::default()
}
#[allow(dead_code)]
fn terrane_platform_failed_result() -> TerranePlatformResult {
    TerranePlatformResult::error("uninitialized platform value")
}
#[allow(dead_code)]
fn terrane_platform_cancel(token: &TerranePlatformCapability) -> TerranePlatformResult {
    terrane_platform_support::cancel(token)
}
#[allow(dead_code)]
fn terrane_platform_result_failed(result: &TerranePlatformResult) -> bool {
    result.failed
}
#[allow(dead_code)]
fn terrane_platform_result_resource_limit(result: &TerranePlatformResult) -> bool {
    result.resource_limit
}
#[allow(dead_code)]
fn terrane_platform_result_truncated(result: &TerranePlatformResult) -> bool {
    result.truncated
}
#[allow(dead_code)]
fn terrane_platform_result_deadline_exceeded(result: &TerranePlatformResult) -> bool {
    result.deadline_exceeded
}
#[allow(dead_code)]
fn terrane_platform_result_message(result: &TerranePlatformResult) -> String {
    result.message.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_text(result: &TerranePlatformResult) -> String {
    result.text.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_detail(result: &TerranePlatformResult) -> String {
    result.detail.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_bytes(result: &TerranePlatformResult) -> Vec<u8> {
    result.data.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_int(
    result: &TerranePlatformResult,
) -> terrane_int_support::Int {
    terrane_int_support::Int::from(result.number)
}
#[allow(dead_code)]
fn terrane_platform_result_bool(result: &TerranePlatformResult) -> bool {
    result.flag
}
#[allow(dead_code)]
fn terrane_platform_result_entries(result: &TerranePlatformResult) -> Vec<String> {
    result.entries.clone()
}
#[allow(dead_code)]
fn terrane_platform_result_capability(
    result: &TerranePlatformResult,
) -> TerranePlatformCapability {
    result.capability.clone().unwrap_or_default()
}
#[allow(dead_code)]
fn terrane_platform_int_mutex(
    initial: terrane_int_support::Int,
) -> TerranePlatformResult {
    let initial = terrane_platform_i128!(initial, "mutex initial value");
    terrane_platform_support::int_mutex(initial)
}
#[allow(dead_code)]
fn terrane_platform_int_mutex_load(
    value: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::int_mutex_load(value)
}
#[allow(dead_code)]
fn terrane_platform_int_mutex_store(
    value: &TerranePlatformCapability,
    replacement: terrane_int_support::Int,
) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "mutex value");
    terrane_platform_support::int_mutex_store(value, replacement)
}
#[allow(dead_code)]
fn terrane_platform_int_mutex_add(
    value: &TerranePlatformCapability,
    amount: terrane_int_support::Int,
) -> TerranePlatformResult {
    let amount = terrane_platform_i128!(amount, "mutex update");
    terrane_platform_support::int_mutex_add(value, amount)
}
#[allow(dead_code)]
fn terrane_platform_int_rw_lock(
    initial: terrane_int_support::Int,
) -> TerranePlatformResult {
    let initial = terrane_platform_i128!(initial, "read/write lock initial value");
    terrane_platform_support::int_rw_lock(initial)
}
#[allow(dead_code)]
fn terrane_platform_int_rw_lock_read(
    value: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::int_rw_lock_read(value)
}
#[allow(dead_code)]
fn terrane_platform_int_rw_lock_write(
    value: &TerranePlatformCapability,
    replacement: terrane_int_support::Int,
) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "read/write lock value");
    terrane_platform_support::int_rw_lock_write(value, replacement)
}
#[allow(dead_code)]
fn terrane_platform_atomic_int64(initial: i64) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64(i128::from(initial))
}
#[allow(dead_code)]
fn terrane_platform_atomic_int64_load(
    value: &TerranePlatformCapability,
    ordering: String,
) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_load(value, &ordering)
}
#[allow(dead_code)]
fn terrane_platform_atomic_int64_store(
    value: &TerranePlatformCapability,
    replacement: i64,
    ordering: String,
) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_store(
        value,
        i128::from(replacement),
        &ordering,
    )
}
#[allow(dead_code)]
fn terrane_platform_atomic_int64_add(
    value: &TerranePlatformCapability,
    amount: i64,
    ordering: String,
) -> TerranePlatformResult {
    terrane_platform_support::atomic_int64_add(value, i128::from(amount), &ordering)
}
#[allow(dead_code)]
fn terrane_platform_thread_local_int(
    initial: terrane_int_support::Int,
) -> TerranePlatformResult {
    let initial = terrane_platform_i128!(initial, "thread-local initial value");
    terrane_platform_support::thread_local_int(initial)
}
#[allow(dead_code)]
fn terrane_platform_thread_local_int_get(
    value: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::thread_local_int_get(value)
}
#[allow(dead_code)]
fn terrane_platform_thread_local_int_set(
    value: &TerranePlatformCapability,
    replacement: terrane_int_support::Int,
) -> TerranePlatformResult {
    let replacement = terrane_platform_i128!(replacement, "thread-local value");
    terrane_platform_support::thread_local_int_set(value, replacement)
}
// Source: src/main.trn
// Namespace: app
fn main() {
    __terrane_run(async move {
        let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
        while index.clone() < terrane_int_support::Int::from(2_i128) {
            let pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
                terrane_collection_support::index_from_int(
                        &terrane_int_support::Int::from(1_i128),
                    )
                    .expect("semantic channel capacity"),
                TerraneChannelOverflow::Block,
            );
            let sender: TerraneChannelSender<terrane_int_support::Int> = pair.sender;
            let receiver: TerraneChannelReceiver<terrane_int_support::Int> = pair
                .receiver;
            let sent: TerraneChannelSendOutcome<terrane_int_support::Int> = __terrane_await(
                    Box::pin(sender.send(index.clone())),
                )
                .await;
            sender.close();
            let received: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
                    Box::pin(receiver.receive()),
                )
                .await;
            let value: Option<terrane_int_support::Int> = received.value;
            let remaining: terrane_collection_support::List<terrane_int_support::Int> = receiver
                .close();
            if value.is_some() {
                println!(
                    "{}{}{}", terrane_scalar_support::scalar_text(&sent.accepted),
                    terrane_scalar_support::scalar_text(&* value.as_ref()
                    .expect("semantic optional narrowing")),
                    terrane_scalar_support::scalar_text(&terrane_int_support::Int::from(remaining
                    .length()))
                );
            }
            index = index.clone() + terrane_int_support::Int::from(1_i128);
        }
    });
}
// Source: core/concurrency.trn
// Namespace: core/concurrency
#[derive(Clone)]
pub struct ConcurrencyOperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl ConcurrencyOperationResult {
    pub fn terrane_construct(
        did_fail: bool,
        exceeded_deadline: bool,
        detail: String,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        value.construct(did_fail, exceeded_deadline, detail);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        exceeded_deadline: bool,
        detail: String,
    ) {
        self.failed = did_fail;
        self.deadline_exceeded = exceeded_deadline;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct ConcurrencyIntResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub available: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl ConcurrencyIntResult {
    pub fn terrane_construct(
        did_fail: bool,
        exceeded_deadline: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            failed: false,
            deadline_exceeded: false,
            available: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        value.construct(did_fail, exceeded_deadline, has_value, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        exceeded_deadline: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) {
        self.failed = did_fail;
        self.deadline_exceeded = exceeded_deadline;
        self.available = has_value;
        self.message = detail;
        self.value = result_value.clone();
    }
}
#[derive(Clone)]
pub struct IntMutex {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntMutex {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_mutex(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn load(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_load(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_store(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_add(
            &self.handle,
            amount,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct IntReadWriteLock {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntReadWriteLock {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn read(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_read(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_write(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
#[derive(Clone)]
pub struct MemoryOrder {
    pub name: String,
}
impl MemoryOrder {
    pub fn terrane_construct(ordering_name: String) -> Self {
        let mut value = Self {
            name: String::from("sequentially-consistent"),
        };
        value.construct(ordering_name);
        value
    }
    pub fn construct(&mut self, ordering_name: String) {
        self.name = ordering_name;
    }
}
pub fn relaxed_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("relaxed"));
}
pub fn acquire_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("acquire"));
}
pub fn release_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("release"));
}
pub fn acquire_release_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("acquire-release"));
}
pub fn sequentially_consistent_order() -> MemoryOrder {
    return MemoryOrder::terrane_construct(String::from("sequentially-consistent"));
}
#[derive(Clone)]
pub struct AtomicInt64 {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl AtomicInt64 {
    pub fn terrane_construct(initial: i64) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: i64) {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn load(&self, ordering: MemoryOrder) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(
        &self,
        value: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(
        &mut self,
        amount: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct ThreadLocalInt {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl ThreadLocalInt {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int(initial);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn get(&self) -> ConcurrencyIntResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_get(
            &self.handle,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
