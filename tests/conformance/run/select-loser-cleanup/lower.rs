// Generated deterministically by Terrane <version>.
// Runtime support: async_native.rs, executor_parallel.rs, channels.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-string-support, terrane-platform-support
// Source: case.trn
// Namespace: select-loser-cleanup
async fn ready() -> String {
    return String::from("winner");
}
async fn blocked(
    marker: String,
    receiver: TerraneChannelReceiver<terrane_int_support::Int>,
) {
    let mut __terrane_finally_guard_0 = __terrane_finally_guard();
    let __terrane_maybe_completion_0: Option<TerraneCompletion<()>> = __terrane_cancel_operation(
            &__terrane_finally_guard_0,
            async {
                let __terrane_try_0: TerraneCompletion<()> = async {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_await(Box::pin(receiver.receive())).await;
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
                    TerraneCompletion::Normal
                }
                    .await;
                match __terrane_try_0 {
                    TerraneCompletion::Return(value) => {
                        return TerraneCompletion::Return(value);
                    }
                    TerraneCompletion::Break => return TerraneCompletion::Break,
                    TerraneCompletion::Continue => return TerraneCompletion::Continue,
                    TerraneCompletion::Normal => {}
                    TerraneCompletion::Error(__terrane_error_0) => {
                        let mut __terrane_handled_0 = false;
                        if !__terrane_handled_0 {
                            return TerraneCompletion::Error(__terrane_error_0);
                        }
                    }
                }
                TerraneCompletion::Normal
            },
        )
        .await;
    let __terrane_cancelled_0 = __terrane_maybe_completion_0.is_none();
    let mut __terrane_completion_0 = __terrane_maybe_completion_0
        .unwrap_or(TerraneCompletion::Normal);
    let __terrane_finally_0: TerraneCompletion<()> = (|| {
        println!("{}", terrane_scalar_support::scalar_text(&marker));
        TerraneCompletion::Normal
    })();
    match __terrane_finally_0 {
        TerraneCompletion::Normal => {}
        replacement => __terrane_completion_0 = replacement,
    }
    if __terrane_cancelled_0
        && matches!(&__terrane_completion_0, TerraneCompletion::Normal)
    {
        __terrane_finish_cancelled_finally(__terrane_finally_guard_0).await;
    }
    __terrane_finally_guard_0.finish();
    match __terrane_completion_0 {
        TerraneCompletion::Normal => {}
        TerraneCompletion::Return(value) => return value,
        TerraneCompletion::Error(error) => __terrane_uncaught(error),
        TerraneCompletion::Break | TerraneCompletion::Continue => {
            __terrane_generated_defect("loop control escaped a non-loop try")
        }
    }
}
fn main() {
    __terrane_run(async move {
        let mut __terrane_select_cursor_432 = 0usize;
        let mut __terrane_select_cursor_790 = 0usize;
        let cleanup_pair_a: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let cleanup_pair_b: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(0_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        {
            let mut __terrane_select_guard_432 = __terrane_finally_guard();
            let __terrane_select_control_432_0 = __terrane_select_control();
            let mut __terrane_select_future_432_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_432_0.clone(),
                blocked(String::from("cleanup-a"), cleanup_pair_a.receiver))
            );
            let mut __terrane_select_result_432_0 = None;
            let __terrane_select_control_432_1 = __terrane_select_control();
            let mut __terrane_select_future_432_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_432_1.clone(),
                blocked(String::from("cleanup-b"), cleanup_pair_b.receiver))
            );
            let mut __terrane_select_result_432_1 = None;
            let __terrane_select_control_432_2 = __terrane_select_control();
            let mut __terrane_select_future_432_2 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_432_2.clone(),
                ready())
            );
            let mut __terrane_select_result_432_2 = None;
            let __terrane_select_winner_432 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..3usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_432
                            + __terrane_select_offset) % 3usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_432_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_432_0 = Some(
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
                                    __terrane_select_future_432_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_432_1 = Some(
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
                            2 => {
                                match Future::poll(
                                    __terrane_select_future_432_2.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_432_2 = Some(
                                            __terrane_select_value,
                                        );
                                        return std::task::Poll::Ready(2usize);
                                    }
                                    std::task::Poll::Ready(None) => {
                                        unreachable!(
                                            "case cancellation starts only after winner selection"
                                        )
                                    }
                                    std::task::Poll::Pending => {}
                                }
                            }
                            _ => {
                                unreachable!("select candidate is within the case count")
                            }
                        }
                    }
                    std::task::Poll::Pending
                })
                .await;
            if __terrane_select_winner_432 == usize::MAX {
                __terrane_select_control_432_2.request_cancel();
                __terrane_select_control_432_1.request_cancel();
                __terrane_select_control_432_0.request_cancel();
                let _ = __terrane_select_future_432_2.as_mut().await;
                let _ = __terrane_select_future_432_1.as_mut().await;
                let _ = __terrane_select_future_432_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_432.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_432).await;
            }
            __terrane_select_cursor_432 = (__terrane_select_winner_432 + 1usize)
                % 3usize;
            match __terrane_select_winner_432 {
                0 => {
                    __terrane_select_control_432_2.request_cancel();
                    __terrane_select_control_432_1.request_cancel();
                    let _ = __terrane_select_future_432_2.as_mut().await;
                    let _ = __terrane_select_future_432_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_432_2.request_cancel();
                    __terrane_select_control_432_0.request_cancel();
                    let _ = __terrane_select_future_432_2.as_mut().await;
                    let _ = __terrane_select_future_432_0.as_mut().await;
                }
                2 => {
                    __terrane_select_control_432_1.request_cancel();
                    __terrane_select_control_432_0.request_cancel();
                    let _ = __terrane_select_future_432_1.as_mut().await;
                    let _ = __terrane_select_future_432_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_432.finish();
            match __terrane_select_winner_432 {
                0 => {
                    let value: () = __terrane_select_result_432_0
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&value));
                }
                1 => {
                    let value: () = __terrane_select_result_432_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&value));
                }
                2 => {
                    let winner: String = __terrane_select_result_432_2
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&winner));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        let waiter_pair: TerraneChannelPair<terrane_int_support::Int> = TerraneChannelPair::new(
            terrane_collection_support::index_from_int(
                    &terrane_int_support::Int::from(1_i128),
                )
                .expect("semantic channel capacity"),
            TerraneChannelOverflow::Block,
        );
        let waiter_tx: TerraneChannelSender<terrane_int_support::Int> = waiter_pair
            .sender;
        let waiter_rx: TerraneChannelReceiver<terrane_int_support::Int> = waiter_pair
            .receiver;
        {
            let mut __terrane_select_guard_790 = __terrane_finally_guard();
            let __terrane_select_control_790_0 = __terrane_select_control();
            let mut __terrane_select_future_790_0 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_790_0.clone(),
                Box::pin(waiter_rx.receive()))
            );
            let mut __terrane_select_result_790_0 = None;
            let __terrane_select_control_790_1 = __terrane_select_control();
            let mut __terrane_select_future_790_1 = std::pin::pin!(
                __terrane_select_operation(__terrane_select_control_790_1.clone(),
                ready())
            );
            let mut __terrane_select_result_790_1 = None;
            let __terrane_select_winner_790 = std::future::poll_fn(|
                    __terrane_select_context|
                {
                    if __terrane_cancellation_is_requested() {
                        return std::task::Poll::Ready(usize::MAX);
                    }
                    for __terrane_select_offset in 0..2usize {
                        let __terrane_select_candidate = (__terrane_select_cursor_790
                            + __terrane_select_offset) % 2usize;
                        match __terrane_select_candidate {
                            0 => {
                                match Future::poll(
                                    __terrane_select_future_790_0.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_790_0 = Some(
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
                                    __terrane_select_future_790_1.as_mut(),
                                    __terrane_select_context,
                                ) {
                                    std::task::Poll::Ready(Some(__terrane_select_value)) => {
                                        __terrane_select_result_790_1 = Some(
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
                            _ => {
                                unreachable!("select candidate is within the case count")
                            }
                        }
                    }
                    std::task::Poll::Pending
                })
                .await;
            if __terrane_select_winner_790 == usize::MAX {
                __terrane_select_control_790_1.request_cancel();
                __terrane_select_control_790_0.request_cancel();
                let _ = __terrane_select_future_790_1.as_mut().await;
                let _ = __terrane_select_future_790_0.as_mut().await;
                __terrane_wait_projected_cleanups().await;
                __terrane_select_guard_790.finish();
                __terrane_finish_cancelled_select(__terrane_select_guard_790).await;
            }
            __terrane_select_cursor_790 = (__terrane_select_winner_790 + 1usize)
                % 2usize;
            match __terrane_select_winner_790 {
                0 => {
                    __terrane_select_control_790_1.request_cancel();
                    let _ = __terrane_select_future_790_1.as_mut().await;
                }
                1 => {
                    __terrane_select_control_790_0.request_cancel();
                    let _ = __terrane_select_future_790_0.as_mut().await;
                }
                _ => unreachable!("selected winner is within the case count"),
            }
            __terrane_wait_projected_cleanups().await;
            __terrane_select_guard_790.finish();
            match __terrane_select_winner_790 {
                0 => {
                    let received: TerraneChannelReceiveOutcome<
                        terrane_int_support::Int,
                    > = __terrane_select_result_790_0
                        .take()
                        .expect("selected case owns its ready result");
                    println!(
                        "{}", terrane_scalar_support::scalar_text(&received.available)
                    );
                }
                1 => {
                    let winner: String = __terrane_select_result_790_1
                        .take()
                        .expect("selected case owns its ready result");
                    println!("{}", terrane_scalar_support::scalar_text(&winner));
                }
                _ => unreachable!("selected winner is within the case count"),
            }
        }
        let sent: TerraneChannelSendOutcome<terrane_int_support::Int> = __terrane_await(
                Box::pin(waiter_tx.send(terrane_int_support::Int::from(9_i128))),
            )
            .await;
        let received: TerraneChannelReceiveOutcome<terrane_int_support::Int> = __terrane_await(
                Box::pin(waiter_rx.receive()),
            )
            .await;
        let value: Option<terrane_int_support::Int> = received.value;
        if value.is_some() {
            println!(
                "{}{}", terrane_scalar_support::scalar_text(&sent.accepted),
                terrane_scalar_support::scalar_text(&* value.as_ref()
                .expect("semantic optional narrowing"))
            );
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
