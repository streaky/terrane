// Generated deterministically by Terrane <version>.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerraneErrorKind {
    ArithmeticOverflow,
    DivisionByZero,
    IntegerConversionOverflow,
    NegativeShiftCount,
    CoercionError,
    DecodeError,
    IndexError,
    MissingKey,
    ResourceError,
    SourceError,
}
impl TerraneErrorKind {
    fn from_source_name(name: &str) -> Self {
        match name {
            ".arithmetic-overflow" => Self::ArithmeticOverflow,
            ".division-by-zero" => Self::DivisionByZero,
            ".integer-conversion-overflow" => Self::IntegerConversionOverflow,
            ".negative-shift-count" => Self::NegativeShiftCount,
            ".coercion-error" => Self::CoercionError,
            ".decode-error" => Self::DecodeError,
            ".index-error" => Self::IndexError,
            ".missing-key" => Self::MissingKey,
            ".resource-error" => Self::ResourceError,
            _ => Self::SourceError,
        }
    }
    fn source_name(self) -> &'static str {
        match self {
            Self::ArithmeticOverflow => ".arithmetic-overflow",
            Self::DivisionByZero => ".division-by-zero",
            Self::IntegerConversionOverflow => ".integer-conversion-overflow",
            Self::NegativeShiftCount => ".negative-shift-count",
            Self::CoercionError => ".coercion-error",
            Self::DecodeError => ".decode-error",
            Self::IndexError => ".index-error",
            Self::MissingKey => ".missing-key",
            Self::ResourceError => ".resource-error",
            Self::SourceError => ".error",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerraneError {
    kind: TerraneErrorKind,
    message: String,
    cause: Option<Box<TerraneError>>,
    context: Vec<&'static str>,
}
impl TerraneError {
    fn new(kind: TerraneErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            cause: None,
            context: Vec::new(),
        }
    }
    #[allow(dead_code)]
    fn at(mut self, frame: &'static str) -> Self {
        self.context.push(frame);
        self
    }
    fn render(&self) -> String {
        let mut rendered = format!("{}: {}", self.kind.source_name(), self.message);
        if let Some(cause) = &self.cause {
            rendered.push_str("\ncaused by: ");
            rendered.push_str(&cause.render());
        }
        for frame in &self.context {
            rendered.push_str("\nat ");
            rendered.push_str(frame);
        }
        rendered
    }
}
impl std::fmt::Display for TerraneError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.render())
    }
}
impl From<terrane_int_support::ArithmeticError> for TerraneError {
    fn from(error: terrane_int_support::ArithmeticError) -> Self {
        Self::new(
            TerraneErrorKind::from_source_name(error.source_name()),
            error.to_string(),
        )
    }
}
impl From<terrane_string_support::DecodeError> for TerraneError {
    fn from(error: terrane_string_support::DecodeError) -> Self {
        Self::new(
            TerraneErrorKind::DecodeError,
            error.to_string().trim_start_matches(".decode-error: "),
        )
    }
}
impl From<terrane_collection_support::IndexError> for TerraneError {
    fn from(error: terrane_collection_support::IndexError) -> Self {
        Self::new(TerraneErrorKind::IndexError, error.to_string())
    }
}
impl From<terrane_collection_support::MissingKey> for TerraneError {
    fn from(error: terrane_collection_support::MissingKey) -> Self {
        Self::new(TerraneErrorKind::MissingKey, error.to_string())
    }
}
impl From<terrane_collection_support::RangeStepError> for TerraneError {
    fn from(error: terrane_collection_support::RangeStepError) -> Self {
        Self::new(TerraneErrorKind::SourceError, error.to_string())
    }
}
fn __terrane_uncaught(error: TerraneError) -> ! {
    eprintln!("{}", error.render());
    std::process::exit(1);
}
fn __terrane_generated_defect(message: &str) -> ! {
    eprintln!(
        "internal compiler defect: generated program reached an impossible completion: {message}"
    );
    std::process::exit(5);
}
#[allow(dead_code)]
enum TerraneCompletion<T> {
    Normal,
    Return(T),
    Error(TerraneError),
    Break,
    Continue,
}
fn terrane_unhex(text: &str) -> Vec<u8> {
    fn digit(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    text.as_bytes()
        .chunks_exact(2)
        .filter_map(|pair| Some(digit(pair[0])? << 4 | digit(pair[1])?))
        .collect()
}
fn terrane_platform_value(value: std::ffi::OsString) -> String {
    terrane_platform_support::platform_value(value)
}
fn terrane_platform_value_is_text(value: &str) -> bool {
    value.starts_with("text:")
}
fn terrane_platform_value_text(value: &str) -> String {
    value.strip_prefix("text:").unwrap_or("").to_owned()
}
fn terrane_platform_value_bytes(value: &str) -> Vec<u8> {
    value.strip_prefix("raw:").map(terrane_unhex).unwrap_or_default()
}
fn terrane_process_arguments() -> Vec<String> {
    std::env::args_os().skip(1).map(terrane_platform_value).collect()
}
fn terrane_environment_entries() -> Vec<String> {
    std::env::vars_os()
        .flat_map(|(name, value)| [
            terrane_platform_value(name),
            terrane_platform_value(value),
        ])
        .collect()
}
fn terrane_process_exit(code: terrane_int_support::Int) {
    let code = terrane_int_support::checked_coerce::<i32>(&code).unwrap_or(255);
    std::process::exit(code)
}
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
) -> TerranePlatformResult {
    let capacity = terrane_platform_i128!(capacity, "channel capacity");
    terrane_platform_support::int_channel(capacity)
}
#[allow(dead_code)]
fn terrane_platform_int_channel_send(
    channel: &TerranePlatformCapability,
    value: terrane_int_support::Int,
    deadline_ms: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let value = terrane_platform_i128!(value, "channel value");
    let deadline_ms = terrane_platform_i128!(deadline_ms, "channel send deadline");
    terrane_platform_support::int_channel_send(channel, value, deadline_ms, cancellation)
}
#[allow(dead_code)]
fn terrane_platform_int_channel_receive(
    channel: &TerranePlatformCapability,
    deadline_ms: terrane_int_support::Int,
    cancellation: &TerranePlatformCapability,
) -> TerranePlatformResult {
    let deadline_ms = terrane_platform_i128!(deadline_ms, "channel receive deadline");
    terrane_platform_support::int_channel_receive(channel, deadline_ms, cancellation)
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
// Namespace: restricted-standard-capabilities
fn main() {
    let channel: IntChannel = IntChannel::terrane_construct(
        terrane_int_support::Int::from(1_i128),
    );
    println!("{}", terrane_scalar_support::scalar_text(&channel.failed));
    let name: HostNameResult = host_name();
    println!(
        "{}", terrane_scalar_support::scalar_text(&(name.failed || name.available))
    );
}
// Source: standard/concurrency.trn
// Namespace: standard/concurrency
#[derive(Clone)]
pub struct OperationResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub message: String,
}
impl OperationResult {
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
pub struct CancellationToken {
    pub handle: TerranePlatformCapability,
}
impl CancellationToken {
    pub fn terrane_construct() -> Self {
        Self {
            handle: terrane_platform_cancellation_token(),
        }
    }
    pub fn cancel(&self) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_cancel(&self.handle);
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
#[derive(Clone)]
pub struct OperationOptions {
    pub deadline_ms: terrane_int_support::Int,
    pub cancellation: CancellationToken,
}
impl OperationOptions {
    pub fn terrane_construct(
        deadline_ms: terrane_int_support::Int,
        cancellation: CancellationToken,
    ) -> Self {
        let mut value = Self {
            deadline_ms: terrane_int_support::Int::from(30000_i128),
            cancellation: CancellationToken::terrane_construct(),
        };
        value.construct(deadline_ms, cancellation);
        value
    }
    pub fn construct(
        &mut self,
        deadline_ms: terrane_int_support::Int,
        cancellation: CancellationToken,
    ) {
        self.deadline_ms = deadline_ms.clone();
        self.cancellation = cancellation.clone();
    }
}
pub fn cancel_operation(cancellation: CancellationToken) -> OperationResult {
    let raw: TerranePlatformResult = terrane_platform_cancel(&cancellation.handle);
    return OperationResult::terrane_construct(
        terrane_platform_result_failed(&raw),
        terrane_platform_result_deadline_exceeded(&raw),
        terrane_platform_result_message(&raw),
    );
}
#[derive(Clone)]
pub struct IntResult {
    pub failed: bool,
    pub deadline_exceeded: bool,
    pub available: bool,
    pub message: String,
    pub value: terrane_int_support::Int,
}
impl IntResult {
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
pub struct IntChannel {
    pub failed: bool,
    pub message: String,
    pub handle: TerranePlatformCapability,
}
impl IntChannel {
    pub fn terrane_construct(capacity: terrane_int_support::Int) -> Self {
        let mut value = Self {
            failed: false,
            message: String::from(""),
            handle: terrane_platform_no_resource(),
        };
        value.construct(capacity);
        value
    }
    pub fn construct(&mut self, capacity: terrane_int_support::Int) {
        let raw: TerranePlatformResult = terrane_platform_int_channel(capacity);
        self.failed = terrane_platform_result_failed(&raw);
        self.message = terrane_platform_result_message(&raw);
        self.handle = terrane_platform_result_capability(&raw);
    }
    pub fn send(
        &self,
        value: terrane_int_support::Int,
        options: OperationOptions,
    ) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_int_channel_send(
            &self.handle,
            value,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn receive(&self, options: OperationOptions) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_channel_receive(
            &self.handle,
            options.deadline_ms,
            &options.cancellation.handle,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
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
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
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
    pub fn load(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_mutex_load(&self.handle);
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
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
            terrane_platform_result_deadline_exceeded(&raw),
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
    pub fn read(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_int_rw_lock_read(&self.handle);
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
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
    pub fn load(&self, ordering: MemoryOrder) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_load(
            &self.handle,
            ordering.name,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_bool(&raw),
            terrane_platform_result_message(&raw),
            terrane_platform_result_int(&raw),
        );
    }
    pub fn store(&self, value: i64, ordering: MemoryOrder) -> OperationResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_store(
            &self.handle,
            value,
            ordering.name,
        );
        return OperationResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
    pub fn increase(&self, amount: i64, ordering: MemoryOrder) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_atomic_int64_add(
            &self.handle,
            amount,
            ordering.name,
        );
        return IntResult::terrane_construct(
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
    pub fn get(&self) -> IntResult {
        let raw: TerranePlatformResult = terrane_platform_thread_local_int_get(
            &self.handle,
        );
        return IntResult::terrane_construct(
            terrane_platform_result_failed(&raw),
            terrane_platform_result_deadline_exceeded(&raw),
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
            terrane_platform_result_deadline_exceeded(&raw),
            terrane_platform_result_message(&raw),
        );
    }
}
// Source: standard/process.trn
// Namespace: standard/process
#[derive(Clone)]
pub struct PlatformString {
    pub is_text: bool,
    pub text: String,
    pub raw: Vec<u8>,
}
impl PlatformString {
    pub fn terrane_construct(encoded: String) -> Self {
        let mut value = Self {
            is_text: true,
            text: String::from(""),
            raw: Vec::from([]),
        };
        value.construct(encoded);
        value
    }
    pub fn construct(&mut self, encoded: String) {
        self.is_text = terrane_platform_value_is_text(&encoded);
        self.text = terrane_platform_value_text(&encoded);
        self.raw = terrane_platform_value_bytes(&encoded);
    }
}
#[derive(Clone)]
pub struct EnvironmentEntry {
    pub name: PlatformString,
    pub value: PlatformString,
}
impl EnvironmentEntry {
    pub fn terrane_construct(name: PlatformString, entry_value: PlatformString) -> Self {
        let mut value = Self {
            name: PlatformString::terrane_construct(String::from("text:")),
            value: PlatformString::terrane_construct(String::from("text:")),
        };
        value.construct(name, entry_value);
        value
    }
    pub fn construct(&mut self, name: PlatformString, entry_value: PlatformString) {
        self.name = name.clone();
        self.value = entry_value.clone();
    }
}
#[derive(Clone)]
pub struct HostNameResult {
    pub failed: bool,
    pub available: bool,
    pub message: String,
    pub value: PlatformString,
}
impl HostNameResult {
    pub fn terrane_construct(
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: PlatformString,
    ) -> Self {
        let mut value = Self {
            failed: false,
            available: false,
            message: String::from(""),
            value: PlatformString::terrane_construct(String::from("text:")),
        };
        value.construct(did_fail, is_available, detail, result_value);
        value
    }
    pub fn construct(
        &mut self,
        did_fail: bool,
        is_available: bool,
        detail: String,
        result_value: PlatformString,
    ) {
        self.failed = did_fail;
        self.available = is_available;
        self.message = detail;
        self.value = result_value.clone();
    }
}
pub fn host_name() -> HostNameResult {
    let raw: TerranePlatformResult = terrane_platform_support::system_host_name();
    return HostNameResult::terrane_construct(
        raw.failed,
        raw.flag,
        raw.message.clone(),
        PlatformString::terrane_construct(raw.text.clone()),
    );
}
pub fn arguments() -> terrane_collection_support::List<PlatformString> {
    let encoded: Vec<String> = terrane_process_arguments();
    let mut values: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() < terrane_int_support::Int::from(encoded.len() as i128) {
        values
            .append(
                PlatformString::terrane_construct(
                    encoded
                        .get(
                            terrane_collection_support::index_from_int(&index.clone())
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/process::arguments (process.trn:51:42)"),
                                )),
                        )
                        .cloned()
                        .ok_or(terrane_collection_support::IndexError {
                            index: terrane_collection_support::index_from_int(
                                    &index.clone(),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at("/standard/process::arguments (process.trn:51:42)"),
                                )),
                        })
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::arguments (process.trn:51:42)"),
                        )),
                ),
            );
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    return values.clone();
}
pub fn environment() -> terrane_collection_support::List<EnvironmentEntry> {
    let encoded: Vec<String> = terrane_environment_entries();
    let mut values: terrane_collection_support::List<EnvironmentEntry> = terrane_collection_support::List::<
        EnvironmentEntry,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone() + terrane_int_support::Int::from(1_i128)
        < terrane_int_support::Int::from(encoded.len() as i128)
    {
        let name: PlatformString = PlatformString::terrane_construct(
            encoded
                .get(
                    terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:60:33)"),
                        )),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: terrane_collection_support::index_from_int(&index.clone())
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:60:33)"),
                        )),
                })
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/process::environment (process.trn:60:33)"),
                )),
        );
        let value: PlatformString = PlatformString::terrane_construct(
            encoded
                .get(
                    terrane_collection_support::index_from_int(
                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:61:34)"),
                        )),
                )
                .cloned()
                .ok_or(terrane_collection_support::IndexError {
                    index: terrane_collection_support::index_from_int(
                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                        )
                        .unwrap_or_else(|error| __terrane_uncaught(
                            TerraneError::from(error)
                                .at("/standard/process::environment (process.trn:61:34)"),
                        )),
                })
                .unwrap_or_else(|error| __terrane_uncaught(
                    TerraneError::from(error)
                        .at("/standard/process::environment (process.trn:61:34)"),
                )),
        );
        values.append(EnvironmentEntry::terrane_construct(name, value));
        index = index.clone() + terrane_int_support::Int::from(2_i128);
    }
    return values.clone();
}
#[derive(Clone)]
pub struct CliSchema {
    pub entries: terrane_collection_support::List<String>,
}
impl CliSchema {
    pub fn terrane_construct(
        declared: terrane_collection_support::List<String>,
    ) -> Self {
        let mut value = Self {
            entries: terrane_collection_support::List::<String>::new(Vec::new()),
        };
        value.construct(declared);
        value
    }
    pub fn construct(&mut self, declared: terrane_collection_support::List<String>) {
        self.entries = declared.clone();
    }
}
#[derive(Clone)]
pub struct CommandLine {
    pub flags: terrane_collection_support::List<String>,
    pub option_names: terrane_collection_support::List<String>,
    pub option_values: terrane_collection_support::List<PlatformString>,
    pub positionals: terrane_collection_support::List<PlatformString>,
    pub diagnostic_arguments: terrane_collection_support::List<terrane_int_support::Int>,
    pub diagnostic_messages: terrane_collection_support::List<String>,
}
impl CommandLine {
    pub fn terrane_construct() -> Self {
        Self {
            flags: terrane_collection_support::List::<String>::new(Vec::new()),
            option_names: terrane_collection_support::List::<String>::new(Vec::new()),
            option_values: terrane_collection_support::List::<
                PlatformString,
            >::new(Vec::new()),
            positionals: terrane_collection_support::List::<
                PlatformString,
            >::new(Vec::new()),
            diagnostic_arguments: terrane_collection_support::List::<
                terrane_int_support::Int,
            >::new(Vec::new()),
            diagnostic_messages: terrane_collection_support::List::<
                String,
            >::new(Vec::new()),
        }
    }
}
pub fn schema_has(schema: CliSchema, sought: String) -> bool {
    let mut __terrane_iterator_0 = terrane_collection_support::Iterable::terrane_iterator(
        &schema.entries,
    );
    loop {
        let entry = match __terrane_iterator_0.next() {
            terrane_collection_support::IterationStep::Item(item) => item,
            terrane_collection_support::IterationStep::End => break,
        };
        if entry == sought {
            return true;
        }
    }
    return false;
}
pub fn parse_command_line(
    schema: CliSchema,
    supplied: terrane_collection_support::List<PlatformString>,
) -> CommandLine {
    let mut flags: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_names: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut option_values: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut positionals: terrane_collection_support::List<PlatformString> = terrane_collection_support::List::<
        PlatformString,
    >::new(Vec::new());
    let mut diagnostic_arguments: terrane_collection_support::List<
        terrane_int_support::Int,
    > = terrane_collection_support::List::<terrane_int_support::Int>::new(Vec::new());
    let mut diagnostic_messages: terrane_collection_support::List<String> = terrane_collection_support::List::<
        String,
    >::new(Vec::new());
    let mut index: terrane_int_support::Int = terrane_int_support::Int::from(0_i128);
    while index.clone()
        < terrane_int_support::Int::from(
            terrane_int_support::Int::from(supplied.length()),
        )
    {
        let argument: PlatformString = supplied
            .get_or_error(
                terrane_collection_support::index_from_int(&index.clone())
                    .unwrap_or_else(|error| __terrane_uncaught(
                        TerraneError::from(error)
                            .at(
                                "/standard/process::parse-command-line (process.trn:96:20)",
                            ),
                    )),
            )
            .unwrap_or_else(|error| __terrane_uncaught(
                TerraneError::from(error)
                    .at("/standard/process::parse-command-line (process.trn:96:20)"),
            ));
        if !argument.is_text {
            diagnostic_arguments.append(index.clone());
            diagnostic_messages
                .append(String::from("command-line option is not Unicode text"));
        } else {
            let flag_entry: String = format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("flag:")),
                terrane_scalar_support::scalar_text(&argument.text)
            );
            let value_entry: String = format!(
                "{}{}", terrane_scalar_support::scalar_text(&String::from("value:")),
                terrane_scalar_support::scalar_text(&argument.text)
            );
            if schema_has(schema.clone(), flag_entry) {
                flags.append(argument.text.clone());
            } else if schema_has(schema.clone(), value_entry) {
                if index.clone() + terrane_int_support::Int::from(1_i128)
                    >= terrane_int_support::Int::from(
                        terrane_int_support::Int::from(supplied.length()),
                    )
                {
                    diagnostic_arguments.append(index.clone());
                    diagnostic_messages.append(String::from("option requires a value"));
                } else {
                    option_names.append(argument.text.clone());
                    option_values
                        .append(
                            supplied
                                .get_or_error(
                                    terrane_collection_support::index_from_int(
                                            &(index.clone() + terrane_int_support::Int::from(1_i128)),
                                        )
                                        .unwrap_or_else(|error| __terrane_uncaught(
                                            TerraneError::from(error)
                                                .at(
                                                    "/standard/process::parse-command-line (process.trn:111:43)",
                                                ),
                                        )),
                                )
                                .unwrap_or_else(|error| __terrane_uncaught(
                                    TerraneError::from(error)
                                        .at(
                                            "/standard/process::parse-command-line (process.trn:111:43)",
                                        ),
                                )),
                        );
                    index = index.clone() + terrane_int_support::Int::from(1_i128);
                }
            } else if argument.text.starts_with(&String::from("--")) {
                diagnostic_arguments.append(index.clone());
                diagnostic_messages.append(String::from("unknown option"));
            } else {
                positionals.append(argument.clone());
            }
        }
        index = index.clone() + terrane_int_support::Int::from(1_i128);
    }
    let mut result: CommandLine = CommandLine::terrane_construct();
    result.flags = flags.clone();
    result.option_names = option_names.clone();
    result.option_values = option_values.clone();
    result.positionals = positionals.clone();
    result.diagnostic_arguments = diagnostic_arguments.clone();
    result.diagnostic_messages = diagnostic_messages.clone();
    return result.clone();
}
#[derive(Clone)]
pub struct ExitStatus {
    pub code: terrane_int_support::Int,
    pub valid: bool,
}
impl ExitStatus {
    pub fn terrane_construct() -> Self {
        Self {
            code: terrane_int_support::Int::from(0_i128),
            valid: true,
        }
    }
}
pub fn make_exit_status(requested: terrane_int_support::Int) -> ExitStatus {
    let mut result: ExitStatus = ExitStatus::terrane_construct();
    if requested.clone() < terrane_int_support::Int::from(0_i128)
        || requested.clone() > terrane_int_support::Int::from(255_i128)
    {
        result.code = terrane_int_support::Int::from(255_i128);
        result.valid = false;
    } else {
        result.code = requested.clone();
    }
    return result.clone();
}
pub fn exit(status: ExitStatus) {
    terrane_process_exit(status.code.clone());
}
