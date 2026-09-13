// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, channels.rs, tasks_native_parallel.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
// Source: case.trn
// Namespace: select-task-outcomes
async fn blocked(receiver: TerraneChannelReceiver<terrane_int_support::Int>) {
    let received: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
            Box::pin(receiver.receive()),
        )
        .await;
    println!("{}", terrane_scalar_support::scalar_text(&received.available));
}
async fn cancelled_winner() {
    let mut __terrane_select_cursor_427 = 0usize;
    let child_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
        terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            )
            .expect("semantic channel capacity"),
        TerraneChannelOverflow::Block,
    );
    let spare_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
        terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            )
            .expect("semantic channel capacity"),
        TerraneChannelOverflow::Block,
    );
    let scope: TerraneTaskScope = TerraneTaskScope::new(None);
    let child: TerraneScopedTask<()> = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.cancellation();
        let __terrane_deadline = __terrane_scope.deadline;
        let __terrane_spawned_task = blocked(child_pair.receiver);
        TerraneScopedTask::spawn(async move {
            match __terrane_cancellable(
                    __terrane_spawned_task,
                    __terrane_cancel,
                    __terrane_deadline,
                )
                .await
            {
                Some(value) => TerraneTaskResult::Completed(value),
                None => TerraneTaskResult::Cancelled,
            }
        })
    };
    scope.cancel();
    {
        let mut __terrane_select_guard_427 = __terrane_finally_guard();
        let __terrane_select_control_427_0 = __terrane_select_control();
        let mut __terrane_select_future_427_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_427_0.clone(), scope
            .join(child))
        );
        let mut __terrane_select_result_427_0 = None;
        let __terrane_select_control_427_1 = __terrane_select_control();
        let mut __terrane_select_future_427_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_427_1.clone(), spare_pair
            .receiver.receive())
        );
        let mut __terrane_select_result_427_1 = None;
        let __terrane_select_winner_427 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_427
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_427_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_427_0 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(0usize);
                                }
                                std::task::Poll::Ready(None) => {
                                    unreachable!(
                                        "case cancellation starts only after winner selection"
                                    )
                                }
                                std::task::Poll::Pending => {}
                            }
                        }
                        1 => {
                            match Future::poll(
                                __terrane_select_future_427_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_427_1 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(1usize);
                                }
                                std::task::Poll::Ready(None) => {
                                    unreachable!(
                                        "case cancellation starts only after winner selection"
                                    )
                                }
                                std::task::Poll::Pending => {}
                            }
                        }
                        _ => unreachable!("select candidate is within the case count"),
                    }
                }
                std::task::Poll::Pending
            })
            .await;
        if __terrane_select_winner_427 == usize::MAX {
            __terrane_select_control_427_1.request_cancel();
            __terrane_select_control_427_0.request_cancel();
            let _ = __terrane_select_future_427_1.as_mut().await;
            let _ = __terrane_select_future_427_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_427.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_427).await;
        }
        __terrane_select_cursor_427 = (__terrane_select_winner_427 + 1usize) % 2usize;
        match __terrane_select_winner_427 {
            0 => {
                __terrane_select_control_427_1.request_cancel();
                let _ = __terrane_select_future_427_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_427_0.request_cancel();
                let _ = __terrane_select_future_427_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_427.finish();
        match __terrane_select_winner_427 {
            0 => {
                let outcome: TerraneTaskOutcome<()> = __terrane_select_result_427_0
                    .take()
                    .expect("selected case owns its ready result");
                println!(
                    "{}{}{}",
                    terrane_scalar_support::scalar_text(&String::from("cancelled")),
                    terrane_scalar_support::scalar_text(&outcome.completed),
                    terrane_scalar_support::scalar_text(&outcome.cancelled)
                );
            }
            1 => {
                let received: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_select_result_427_1
                    .take()
                    .expect("selected case owns its ready result");
                println!("{}", terrane_scalar_support::scalar_text(&received.available));
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
async fn deadline_winner() {
    let mut __terrane_select_cursor_832 = 0usize;
    let child_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
        terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            )
            .expect("semantic channel capacity"),
        TerraneChannelOverflow::Block,
    );
    let spare_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
        terrane_collection_support::index_from_int(
                &terrane_int_support::Int::from(0_i128),
            )
            .expect("semantic channel capacity"),
        TerraneChannelOverflow::Block,
    );
    let scope: TerraneTaskScope = TerraneTaskScope::new(Some(0 as u64));
    let child: TerraneScopedTask<()> = {
        let __terrane_scope = scope.clone();
        let __terrane_cancel = __terrane_scope.cancellation();
        let __terrane_deadline = __terrane_scope.deadline;
        let __terrane_spawned_task = blocked(child_pair.receiver);
        TerraneScopedTask::spawn(async move {
            match __terrane_cancellable(
                    __terrane_spawned_task,
                    __terrane_cancel,
                    __terrane_deadline,
                )
                .await
            {
                Some(value) => TerraneTaskResult::Completed(value),
                None => TerraneTaskResult::Cancelled,
            }
        })
    };
    {
        let mut __terrane_select_guard_832 = __terrane_finally_guard();
        let __terrane_select_control_832_0 = __terrane_select_control();
        let mut __terrane_select_future_832_0 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_832_0.clone(), scope
            .join(child))
        );
        let mut __terrane_select_result_832_0 = None;
        let __terrane_select_control_832_1 = __terrane_select_control();
        let mut __terrane_select_future_832_1 = std::pin::pin!(
            __terrane_select_operation(__terrane_select_control_832_1.clone(), spare_pair
            .receiver.receive())
        );
        let mut __terrane_select_result_832_1 = None;
        let __terrane_select_winner_832 = std::future::poll_fn(|
                __terrane_select_context|
            {
                if __terrane_cancellation_is_requested() {
                    return std::task::Poll::Ready(usize::MAX);
                }
                for __terrane_select_offset in 0..2usize {
                    let __terrane_select_candidate = (__terrane_select_cursor_832
                        + __terrane_select_offset) % 2usize;
                    match __terrane_select_candidate {
                        0 => {
                            match Future::poll(
                                __terrane_select_future_832_0.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_832_0 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(0usize);
                                }
                                std::task::Poll::Ready(None) => {
                                    unreachable!(
                                        "case cancellation starts only after winner selection"
                                    )
                                }
                                std::task::Poll::Pending => {}
                            }
                        }
                        1 => {
                            match Future::poll(
                                __terrane_select_future_832_1.as_mut(),
                                __terrane_select_context,
                            ) {
                                std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                    __terrane_select_result_832_1 = Some(
                                        __terrane_select_value,
                                    );
                                    return std::task::Poll::Ready(1usize);
                                }
                                std::task::Poll::Ready(None) => {
                                    unreachable!(
                                        "case cancellation starts only after winner selection"
                                    )
                                }
                                std::task::Poll::Pending => {}
                            }
                        }
                        _ => unreachable!("select candidate is within the case count"),
                    }
                }
                std::task::Poll::Pending
            })
            .await;
        if __terrane_select_winner_832 == usize::MAX {
            __terrane_select_control_832_1.request_cancel();
            __terrane_select_control_832_0.request_cancel();
            let _ = __terrane_select_future_832_1.as_mut().await;
            let _ = __terrane_select_future_832_0.as_mut().await;
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_832.finish();
            __terrane_finish_cancelled_select(__terrane_select_guard_832).await;
        }
        __terrane_select_cursor_832 = (__terrane_select_winner_832 + 1usize) % 2usize;
        match __terrane_select_winner_832 {
            0 => {
                __terrane_select_control_832_1.request_cancel();
                let _ = __terrane_select_future_832_1.as_mut().await;
            }
            1 => {
                __terrane_select_control_832_0.request_cancel();
                let _ = __terrane_select_future_832_0.as_mut().await;
            }
            _ => unreachable!("selected winner is within the case count"),
        }
        __terrane_wait_projected_cleanups().await;
        __terrane_select_guard_832.finish();
        match __terrane_select_winner_832 {
            0 => {
                let outcome: TerraneTaskOutcome<()> = __terrane_select_result_832_0
                    .take()
                    .expect("selected case owns its ready result");
                println!(
                    "{}{}{}",
                    terrane_scalar_support::scalar_text(&String::from("deadline")),
                    terrane_scalar_support::scalar_text(&outcome.completed),
                    terrane_scalar_support::scalar_text(&outcome.cancelled)
                );
            }
            1 => {
                let received: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_select_result_832_1
                    .take()
                    .expect("selected case owns its ready result");
                println!("{}", terrane_scalar_support::scalar_text(&received.available));
            }
            _ => unreachable!("selected winner is within the case count"),
        }
    }
}
fn main() {
    __terrane_run(async move {
        let cancelled: () = __terrane_await(cancelled_winner()).await;
        let deadline: () = __terrane_await(deadline_winner()).await;
        if cancelled == () && deadline == () {
            return ();
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
