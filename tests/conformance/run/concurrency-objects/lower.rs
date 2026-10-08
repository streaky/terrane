// Generated deterministically by Terrane <version>.
// Runtime support: platform_capability_types.rs, platform_result_type.rs, platform_int_conversion.rs, platform_capability_base.rs, platform_concurrency.rs
// Vendored support crates: terrane-int-support, terrane-scalar-support, terrane-platform-support
// Source: src/main.trn
// Namespace: concurrency-objects
fn main() {
    let mut counter_terrane_f0_s228: IntMutex;
    let shared_terrane_f0_s322: IntReadWriteLock;
    let mut atomic_terrane_f0_s420: AtomicInt64;
    let updated_terrane_f0_s459: ConcurrencyIntResult;
    let invalid_store_terrane_f0_s593: ConcurrencyOperationResult;
    let invalid_ordering_terrane_f0_s688: ConcurrencyIntResult;
    let local_terrane_f0_s815: ThreadLocalInt;
    counter_terrane_f0_s228 = IntMutex::terrane_construct(
        terrane_int_support::Int::from(4_i128),
    );
    counter_terrane_f0_s228.increase(terrane_int_support::Int::from(3_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&counter_terrane_f0_s228.load().value)
    );
    shared_terrane_f0_s322 = IntReadWriteLock::terrane_construct(
        terrane_int_support::Int::from(8_i128),
    );
    shared_terrane_f0_s322.write(terrane_int_support::Int::from(9_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&shared_terrane_f0_s322.read().value)
    );
    atomic_terrane_f0_s420 = AtomicInt64::terrane_construct(10);
    updated_terrane_f0_s459 = atomic_terrane_f0_s420
        .increase(5, acquire_release_order());
    println!("{}", terrane_scalar_support::scalar_text(&updated_terrane_f0_s459.failed));
    println!(
        "{}", terrane_scalar_support::scalar_text(&atomic_terrane_f0_s420
        .load(acquire_order()).value)
    );
    invalid_store_terrane_f0_s593 = atomic_terrane_f0_s420
        .store(16, acquire_release_order());
    println!(
        "{}", terrane_scalar_support::scalar_text(&invalid_store_terrane_f0_s593.failed)
    );
    invalid_ordering_terrane_f0_s688 = atomic_terrane_f0_s420.load(release_order());
    println!(
        "{}", terrane_scalar_support::scalar_text(&invalid_ordering_terrane_f0_s688
        .failed)
    );
    println!(
        "{}", terrane_scalar_support::scalar_text(&invalid_ordering_terrane_f0_s688
        .available)
    );
    local_terrane_f0_s815 = ThreadLocalInt::terrane_construct(
        terrane_int_support::Int::from(20_i128),
    );
    local_terrane_f0_s815.write(terrane_int_support::Int::from(21_i128));
    println!(
        "{}", terrane_scalar_support::scalar_text(&local_terrane_f0_s815.get().value)
    );
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            message: String::from(""),
        };
        __terrane_constructed_value.construct(did_fail, exceeded_deadline, detail);
        __terrane_constructed_value
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            deadline_exceeded: false,
            available: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        __terrane_constructed_value
            .construct(did_fail, exceeded_deadline, has_value, detail, result_value);
        __terrane_constructed_value
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s966: TerranePlatformResult;
        raw_terrane_f1_s966 = terrane_platform_int_mutex(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s966);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s966);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s966);
    }
    pub fn load(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s1191: TerranePlatformResult;
        raw_terrane_f1_s1191 = terrane_platform_int_mutex_load(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1191),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1191),
            terrane_platform_result_bool(&raw_terrane_f1_s1191),
            terrane_platform_result_message(&raw_terrane_f1_s1191),
            terrane_platform_result_int(&raw_terrane_f1_s1191),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s1487: TerranePlatformResult;
        raw_terrane_f1_s1487 = terrane_platform_int_mutex_store(&self.handle, value);
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1487),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1487),
            terrane_platform_result_message(&raw_terrane_f1_s1487),
        );
    }
    pub fn increase(
        &mut self,
        amount: terrane_int_support::Int,
    ) -> ConcurrencyIntResult {
        let raw_terrane_f1_s1754: TerranePlatformResult;
        raw_terrane_f1_s1754 = terrane_platform_int_mutex_add(&self.handle, amount);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s1754),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s1754),
            terrane_platform_result_bool(&raw_terrane_f1_s1754),
            terrane_platform_result_message(&raw_terrane_f1_s1754),
            terrane_platform_result_int(&raw_terrane_f1_s1754),
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s2175: TerranePlatformResult;
        raw_terrane_f1_s2175 = terrane_platform_int_rw_lock(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s2175);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s2175);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s2175);
    }
    pub fn read(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s2410: TerranePlatformResult;
        raw_terrane_f1_s2410 = terrane_platform_int_rw_lock_read(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s2410),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s2410),
            terrane_platform_result_bool(&raw_terrane_f1_s2410),
            terrane_platform_result_message(&raw_terrane_f1_s2410),
            terrane_platform_result_int(&raw_terrane_f1_s2410),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s2716: TerranePlatformResult;
        raw_terrane_f1_s2716 = terrane_platform_int_rw_lock_write(&self.handle, value);
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s2716),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s2716),
            terrane_platform_result_message(&raw_terrane_f1_s2716),
        );
    }
}
#[derive(Clone)]
pub struct MemoryOrder {
    pub name: String,
}
impl MemoryOrder {
    pub fn terrane_construct(ordering_name: String) -> Self {
        let mut __terrane_constructed_value = Self {
            name: String::from("sequentially-consistent"),
        };
        __terrane_constructed_value.construct(ordering_name);
        __terrane_constructed_value
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: i64) {
        let raw_terrane_f1_s3708: TerranePlatformResult;
        raw_terrane_f1_s3708 = terrane_platform_atomic_int64(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s3708);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s3708);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s3708);
    }
    pub fn load(&self, ordering: MemoryOrder) -> ConcurrencyIntResult {
        let raw_terrane_f1_s3958: TerranePlatformResult;
        raw_terrane_f1_s3958 = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s3958),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s3958),
            terrane_platform_result_bool(&raw_terrane_f1_s3958),
            terrane_platform_result_message(&raw_terrane_f1_s3958),
            terrane_platform_result_int(&raw_terrane_f1_s3958),
        );
    }
    pub fn store(
        &self,
        value: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s4297: TerranePlatformResult;
        raw_terrane_f1_s4297 = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s4297),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s4297),
            terrane_platform_result_message(&raw_terrane_f1_s4297),
        );
    }
    pub fn increase(
        &mut self,
        amount: i64,
        ordering: MemoryOrder,
    ) -> ConcurrencyIntResult {
        let raw_terrane_f1_s4607: TerranePlatformResult;
        raw_terrane_f1_s4607 = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s4607),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s4607),
            terrane_platform_result_bool(&raw_terrane_f1_s4607),
            terrane_platform_result_message(&raw_terrane_f1_s4607),
            terrane_platform_result_int(&raw_terrane_f1_s4607),
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
        let mut __terrane_constructed_value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        __terrane_constructed_value.construct(initial);
        __terrane_constructed_value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        let raw_terrane_f1_s5043: TerranePlatformResult;
        raw_terrane_f1_s5043 = terrane_platform_thread_local_int(initial);
        self.failed = terrane_platform_result_failed(&raw_terrane_f1_s5043);
        self.message = terrane_platform_result_message(&raw_terrane_f1_s5043);
        self.handle = terrane_platform_result_capability(&raw_terrane_f1_s5043);
    }
    pub fn get(&self) -> ConcurrencyIntResult {
        let raw_terrane_f1_s5274: TerranePlatformResult;
        raw_terrane_f1_s5274 = terrane_platform_thread_local_int_get(&self.handle);
        return ConcurrencyIntResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5274),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5274),
            terrane_platform_result_bool(&raw_terrane_f1_s5274),
            terrane_platform_result_message(&raw_terrane_f1_s5274),
            terrane_platform_result_int(&raw_terrane_f1_s5274),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> ConcurrencyOperationResult {
        let raw_terrane_f1_s5576: TerranePlatformResult;
        raw_terrane_f1_s5576 = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return ConcurrencyOperationResult::terrane_construct(
            terrane_platform_result_failed(&raw_terrane_f1_s5576),
            terrane_platform_result_deadline_exceeded(&raw_terrane_f1_s5576),
            terrane_platform_result_message(&raw_terrane_f1_s5576),
        );
    }
}
