// Generated deterministically by Terrane <version>.
// Runtime support: async.rs, executor_local.rs, channels.rs, platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-collection-support, terrane-scalar-support, terrane-platform-support
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
