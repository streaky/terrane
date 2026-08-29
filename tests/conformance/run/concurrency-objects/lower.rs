// Generated deterministically by Terrane <version>.
type TerranePlatformCapability = terrane_platform_support::Capability;
type TerranePlatformResult = terrane_platform_support::ResultValue;
fn terrane_platform_i128(
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
fn terrane_platform_int_channel(
    capacity: terrane_int_support::Int,
) -> TerranePlatformCapability {
    match terrane_platform_i128(&capacity, "channel capacity") {
        Ok(capacity) => terrane_platform_support::int_channel(capacity),
        Err(error) => error.capability.unwrap_or_default(),
    }
}
#[allow(dead_code)]
fn terrane_platform_int_channel_send(
    channel: &TerranePlatformCapability,
    value: terrane_int_support::Int,
) -> TerranePlatformResult {
    let value = terrane_platform_i128!(value, "channel value");
    terrane_platform_support::int_channel_send(channel, value)
}
#[allow(dead_code)]
fn terrane_platform_int_channel_receive(
    channel: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::int_channel_receive(channel)
}
#[allow(dead_code)]
fn terrane_platform_int_channel_try_receive(
    channel: &TerranePlatformCapability,
) -> TerranePlatformResult {
    terrane_platform_support::int_channel_try_receive(channel)
}
#[allow(dead_code)]
fn terrane_platform_int_mutex(
    initial: terrane_int_support::Int,
) -> TerranePlatformCapability {
    match terrane_platform_i128(&initial, "mutex initial value") {
        Ok(initial) => terrane_platform_support::int_mutex(initial),
        Err(error) => error.capability.unwrap_or_default(),
    }
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
) -> TerranePlatformCapability {
    match terrane_platform_i128(&initial, "read/write lock initial value") {
        Ok(initial) => terrane_platform_support::int_rw_lock(initial),
        Err(error) => error.capability.unwrap_or_default(),
    }
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
fn terrane_platform_atomic_int64(initial: i64) -> TerranePlatformCapability {
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
) -> TerranePlatformCapability {
    match terrane_platform_i128(&initial, "thread-local initial value") {
        Ok(initial) => terrane_platform_support::thread_local_int(initial),
        Err(error) => error.capability.unwrap_or_default(),
    }
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
// Source: case.trn
// Namespace: app
fn main() {
    let messages: IntChannel = IntChannel::terrane_construct(
        terrane_int_support::Int::from(2_i128),
    );
    let sent: OperationResult = messages.send(terrane_int_support::Int::from(11_i128));
    println!("{}", terrane_scalar_support::scalar_text(&sent.failed));
    let received: IntResult = messages.receive();
    println!("{}", terrane_scalar_support::scalar_text(&received.available));
    println!("{}", terrane_scalar_support::scalar_text(&received.value));
    let counter: IntMutex = IntMutex::terrane_construct(
        terrane_int_support::Int::from(4_i128),
    );
    let updated: IntResult = counter.increase(terrane_int_support::Int::from(3_i128));
    println!("{}", terrane_scalar_support::scalar_text(&updated.value));
    let shared: IntReadWriteLock = IntReadWriteLock::terrane_construct(
        terrane_int_support::Int::from(8_i128),
    );
    shared.write(terrane_int_support::Int::from(9_i128));
    println!("{}", terrane_scalar_support::scalar_text(&shared.read().value));
    let atomic: AtomicInt64 = AtomicInt64::terrane_construct(10);
    atomic.increase(5, String::from("sequentially-consistent"));
    println!(
        "{}", terrane_scalar_support::scalar_text(&atomic.load(String::from("acquire"))
        .value)
    );
    let invalid_ordering: IntResult = atomic.load(String::from("release"));
    println!("{}", terrane_scalar_support::scalar_text(&invalid_ordering.failed));
    println!("{}", terrane_scalar_support::scalar_text(&invalid_ordering.available));
    let local: ThreadLocalInt = ThreadLocalInt::terrane_construct(
        terrane_int_support::Int::from(20_i128),
    );
    local.write(terrane_int_support::Int::from(21_i128));
    println!("{}", terrane_scalar_support::scalar_text(&local.get().value));
}
// Source: standard/concurrency.trn
// Namespace: standard/concurrency
#[derive(Clone)]
pub struct OperationResult {
    pub failed: bool,
    pub message: String,
}
impl OperationResult {
    pub fn terrane_construct(did_fail: bool, detail: String) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
        };
        value.construct(did_fail, detail);
        value
    }
    pub fn construct(&mut self, did_fail: bool, detail: String) {
        self.failed = did_fail;
        self.message = detail;
    }
}
#[derive(Clone)]
pub struct IntResult {
    pub failed: bool,
    pub available: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl IntResult {
    pub fn terrane_construct(
        did_fail: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) -> Self {
        let mut value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: terrane_int_support::Int::from(0_i128),
        };
        value.construct(did_fail, has_value, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        has_value: bool,
        detail: String,
        result_value: terrane_int_support::Int,
    ) {
        self.failed = did_fail;
        self.available = has_value;
        self.message = detail;
        self.value = result_value.clone();
    }
}
#[derive(Clone)]
pub struct IntChannel {
    pub handle: TerranePlatformCapability,
}
impl IntChannel {
    pub fn terrane_construct(capacity: terrane_int_support::Int) -> Self {
        let mut value = Self {
            handle: terrane_platform_no_resource(),
        };
        value.construct(capacity);
        value
    }
    pub fn construct(&mut self, capacity: terrane_int_support::Int) {
        self.handle = terrane_platform_int_channel(capacity);
    }
    pub fn send(&self, value: terrane_int_support::Int) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_channel_send(
            &self.handle,
            value,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn receive(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_channel_receive(
            &self.handle,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn try_receive(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_channel_try_receive(
            &self.handle,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct IntMutex {
    pub handle: TerranePlatformCapability,
}
impl IntMutex {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        self.handle = terrane_platform_int_mutex(initial);
    }
    pub fn load(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_load(&self.handle);
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: terrane_int_support::Int) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_store(
            &self.handle,
            value,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(&self, amount: terrane_int_support::Int) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_add(
            &self.handle,
            amount,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct IntReadWriteLock {
    pub handle: TerranePlatformCapability,
}
impl IntReadWriteLock {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        self.handle = terrane_platform_int_rw_lock(initial);
    }
    pub fn read(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_read(&self.handle);
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_write(
            &self.handle,
            value,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
#[derive(Clone)]
pub struct AtomicInt64 {
    pub handle: TerranePlatformCapability,
}
impl AtomicInt64 {
    pub fn terrane_construct(initial: i64) -> Self {
        let mut value = Self {
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: i64) {
        self.handle = terrane_platform_atomic_int64(initial);
    }
    pub fn load(&self, ordering: String) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: i64, ordering: String) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(&self, amount: i64, ordering: String) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
}
#[derive(Clone)]
pub struct ThreadLocalInt {
    pub handle: TerranePlatformCapability,
}
impl ThreadLocalInt {
    pub fn terrane_construct(initial: terrane_int_support::Int) -> Self {
        let mut value = Self {
            handle: terrane_platform_no_resource(),
        };
        value.construct(initial);
        value
    }
    pub fn construct(&mut self, initial: terrane_int_support::Int) {
        self.handle = terrane_platform_thread_local_int(initial);
    }
    pub fn get(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_get(
            &self.handle,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn write(&self, value: terrane_int_support::Int) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_set(
            &self.handle,
            value,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
