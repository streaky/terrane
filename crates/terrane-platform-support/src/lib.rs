#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::format_collect,
    clippy::iter_with_drain,
    clippy::large_enum_variant,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::result_large_err,
    clippy::semicolon_if_nothing_returned,
    clippy::struct_excessive_bools
)]
mod networking_buffers;
mod networking_native;
#[cfg(test)]
mod networking_review_tests;
mod networking_state;
mod networking_transport;
pub use networking_native::{
    NativeTcpListener, NativeTcpStream, into_native_tcp_listener, into_native_tcp_stream,
};
use networking_transport::io_error;
#[cfg(test)]
use networking_transport::ordered_socket_candidates;
pub use networking_transport::*;
mod observability;
#[cfg(unix)]
mod signals;
use networking_state::{ListenerState, TcpState, UdpState};

pub use observability::{LogEventInput, LogFieldInput};
// The host ABI deliberately uses one flat result envelope and opaque heterogeneous capability
// storage. These functions are generated-crate internals rather than a user-facing Rust API.

// Rust owns only irreducible OS boundaries, optimiser-sensitive guarantees, and audited
// cryptographic/compression/TLS implementations. Terrane owns the public object model and policy.
use base64::Engine as _;
use hmac::Mac as _;
use rand_core::{RngCore as _, SeedableRng as _};
use sha2::Digest as _;
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::io::{Read, Write as _};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{
    Arc, LazyLock, Mutex, RwLock, Weak,
    atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering},
};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use subtle::ConstantTimeEq as _;
use zeroize::Zeroizing;

#[derive(Clone, Default)]
pub struct ResultValue {
    pub failed: bool,
    pub resource_limit: bool,
    pub truncated: bool,
    pub deadline_exceeded: bool,
    pub message: String,
    pub text: String,
    pub detail: String,
    pub data: Vec<u8>,
    pub number: i128,
    pub secondary_number: i128,
    pub exact_number: String,
    pub secondary_exact_number: String,
    pub flag: bool,
    pub entries: Vec<String>,
    pub capability: Option<Capability>,
}

impl ResultValue {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            failed: true,
            message: message.into(),
            ..Self::default()
        }
    }
    fn limit(message: impl Into<String>) -> Self {
        Self {
            failed: true,
            resource_limit: true,
            message: message.into(),
            ..Self::default()
        }
    }
}

struct SecretState {
    bytes: Zeroizing<Vec<u8>>,
    destroyed: bool,
}
struct ThreadLocalInt {
    id: u64,
    initial: i128,
    owner: Arc<()>,
}
struct CancellationState {
    cancelled: AtomicBool,
    notify: tokio::sync::Notify,
}

static NEXT_THREAD_LOCAL_ID: AtomicU64 = AtomicU64::new(1);
thread_local! {
    static THREAD_LOCAL_INTS: RefCell<HashMap<u64, (Weak<()>, i128)>> =
        RefCell::new(HashMap::new());
}

struct ReadBuffer {
    bytes: Vec<u8>,
    filled: usize,
    closed: bool,
}

type AsyncTlsStream = tokio_rustls::client::TlsStream<tokio::net::TcpStream>;

struct AsyncTlsState {
    reader: tokio::sync::Mutex<Option<tokio::io::ReadHalf<AsyncTlsStream>>>,
    writer: tokio::sync::Mutex<Option<tokio::io::WriteHalf<AsyncTlsStream>>>,
}

impl AsyncTlsState {
    fn new(stream: AsyncTlsStream) -> Self {
        let (reader, writer) = tokio::io::split(stream);
        Self {
            reader: tokio::sync::Mutex::new(Some(reader)),
            writer: tokio::sync::Mutex::new(Some(writer)),
        }
    }
}
#[derive(Clone)]
pub struct Capability(Arc<CapabilityInner>);

enum CapabilityInner {
    Secure,
    Invalid(String),
    Pseudo(Mutex<rand_chacha::ChaCha20Rng>),
    Secret(Mutex<SecretState>),
    Cancellation(CancellationState),
    LogSink(u64),
    IntMutex(Mutex<i128>),
    IntRwLock(RwLock<i128>),
    AtomicI64(AtomicI64),
    AsyncTls(AsyncTlsState),
    ThreadLocalInt(ThreadLocalInt),
    Listener(ListenerState),
    Tcp(TcpState),
    Udp(UdpState),
    Tls(Mutex<Option<rustls::StreamOwned<rustls::ClientConnection, TcpStream>>>),
    Buffer(tokio::sync::Mutex<ReadBuffer>),
    #[cfg(unix)]
    SignalSubscription(Arc<signals::Subscription>),
}
impl Default for Capability {
    fn default() -> Self {
        Self(Arc::new(CapabilityInner::Invalid(
            "missing platform capability".to_owned(),
        )))
    }
}

fn count(value: i128, label: &str) -> Result<usize, ResultValue> {
    usize::try_from(value).map_err(|_| {
        ResultValue::error(format!(
            "{label} must be a non-negative platform-sized integer"
        ))
    })
}
fn lock_error(kind: &str) -> ResultValue {
    ResultValue::error(format!("{kind} is poisoned"))
}

#[cfg(unix)]
pub fn platform_value(value: std::ffi::OsString) -> String {
    use std::os::unix::ffi::OsStrExt as _;
    value.into_string().map_or_else(
        |raw| format!("raw:{}", hex_encode(raw.as_bytes())),
        |text| format!("text:{text}"),
    )
}

#[cfg(windows)]
pub fn platform_value(value: std::ffi::OsString) -> String {
    use std::os::windows::ffi::OsStrExt as _;
    value.into_string().map_or_else(
        |raw| {
            let units = raw
                .encode_wide()
                .flat_map(u16::to_le_bytes)
                .collect::<Vec<_>>();
            format!("raw:{}", hex_encode(&units))
        },
        |text| format!("text:{text}"),
    )
}

pub fn system_host_name() -> ResultValue {
    match hostname::get() {
        Ok(name) => ResultValue {
            text: platform_value(name),
            flag: true,
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("cannot read the system host name: {error}")),
    }
}

fn capability_result(capability: Capability) -> ResultValue {
    ResultValue {
        capability: Some(capability),
        ..ResultValue::default()
    }
}

static MONOTONIC_ORIGIN: LazyLock<Instant> = LazyLock::new(Instant::now);
// Every monotonic observer in one process is measured from this nominal runtime activation origin.
// The value is intentionally process-local and has no meaning across runtime activations.
#[cfg(test)]
thread_local! {
    static CONTROLLED_MONOTONIC_NANOS: std::cell::Cell<Option<u64>> =
        const { std::cell::Cell::new(None) };
}
#[cfg(test)]
static CONTROLLED_TIME_NOTIFY: LazyLock<tokio::sync::Notify> =
    LazyLock::new(tokio::sync::Notify::new);

#[must_use]
pub fn monotonic_nanos() -> u128 {
    #[cfg(test)]
    if let Some(nanoseconds) = CONTROLLED_MONOTONIC_NANOS.get() {
        return u128::from(nanoseconds);
    }
    MONOTONIC_ORIGIN.elapsed().as_nanos()
}

pub fn wall_time() -> ResultValue {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(elapsed) => ResultValue {
            number: i128::from(elapsed.as_secs()),
            secondary_number: i128::from(elapsed.subsec_nanos()),
            ..ResultValue::default()
        },
        Err(error) => {
            let elapsed = error.duration();
            let seconds = i128::from(elapsed.as_secs());
            let nanoseconds = i128::from(elapsed.subsec_nanos());
            if nanoseconds == 0 {
                ResultValue {
                    number: -seconds,
                    ..ResultValue::default()
                }
            } else {
                ResultValue {
                    number: -seconds - 1,
                    secondary_number: 1_000_000_000 - nanoseconds,
                    ..ResultValue::default()
                }
            }
        }
    }
}

#[cfg(unix)]
pub fn process_signal_subscribe(names: &[String]) -> ResultValue {
    signals::subscribe(names)
}

#[cfg(not(unix))]
pub fn process_signal_subscribe(_names: &[String]) -> ResultValue {
    ResultValue::error("process signals are not supported on this host family")
}

#[cfg(unix)]
pub async fn process_signal_next(capability: &Capability) -> ResultValue {
    signals::next(capability).await
}

#[cfg(not(unix))]
pub async fn process_signal_next(_capability: &Capability) -> ResultValue {
    ResultValue::error("process signals are not supported on this host family")
}

#[cfg(unix)]
pub fn process_signal_close(capability: &Capability) -> ResultValue {
    signals::close(capability)
}

#[cfg(not(unix))]
pub fn process_signal_close(_capability: &Capability) -> ResultValue {
    ResultValue::error("process signals are not supported on this host family")
}

pub async fn sleep(duration: Duration) {
    #[cfg(test)]
    if let Some(started) = CONTROLLED_MONOTONIC_NANOS.get() {
        let elapsed = u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX);
        let target = started.saturating_add(elapsed);
        loop {
            let notified = CONTROLLED_TIME_NOTIFY.notified();
            if CONTROLLED_MONOTONIC_NANOS
                .get()
                .is_some_and(|nanoseconds| nanoseconds >= target)
            {
                return;
            }
            notified.await;
        }
    }
    tokio::time::sleep(duration).await;
}

pub fn logging_memory_sink(
    capacity: Option<i128>,
    overflow: &str,
    start_timestamp: Option<i128>,
    timestamp_step: Option<i128>,
    reveal_secrets: bool,
) -> ResultValue {
    let Some(capacity) = capacity else {
        return ResultValue::error("logging sink capacity must fit int128");
    };
    let Some(start_timestamp) = start_timestamp else {
        return ResultValue::error("logging start timestamp must fit int128");
    };
    let Some(timestamp_step) = timestamp_step else {
        return ResultValue::error("logging timestamp step must fit int128");
    };
    let capacity = match count(capacity, "logging sink capacity") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let Ok(start_timestamp) = u64::try_from(start_timestamp) else {
        return ResultValue::error("logging start timestamp must fit uint64");
    };
    let Ok(timestamp_step) = u64::try_from(timestamp_step) else {
        return ResultValue::error("logging timestamp step must fit uint64");
    };
    match observability::memory_sink(
        capacity,
        overflow,
        start_timestamp,
        timestamp_step,
        reveal_secrets,
    ) {
        Ok(id) => capability_result(Capability(Arc::new(CapabilityInner::LogSink(id)))),
        Err(error) => ResultValue::error(error),
    }
}

pub fn logging_console_sink(reveal_secrets: bool) -> ResultValue {
    match observability::console_sink(reveal_secrets) {
        Ok(id) => capability_result(Capability(Arc::new(CapabilityInner::LogSink(id)))),
        Err(error) => ResultValue::error(error),
    }
}

pub fn logging_failing_sink() -> ResultValue {
    match observability::failing_sink() {
        Ok(id) => capability_result(Capability(Arc::new(CapabilityInner::LogSink(id)))),
        Err(error) => ResultValue::error(error),
    }
}

pub fn logging_no_sink() -> Capability {
    Capability(Arc::new(CapabilityInner::Invalid(
        "missing logging sink".to_owned(),
    )))
}

pub fn logging_emit(sink: &Capability, input: LogEventInput) -> ResultValue {
    match sink.0.as_ref() {
        CapabilityInner::LogSink(id) => match observability::emit(*id, input) {
            Ok(()) => ResultValue::default(),
            Err(error) => {
                let _ = observability::record_fallback(format!("logging sink failure: {error}"));
                ResultValue::error(error)
            }
        },
        CapabilityInner::Invalid(message) => ResultValue::error(message.clone()),
        _ => ResultValue::error("capability is not a logging sink"),
    }
}

pub fn logging_reveals_secrets(sink: &Capability) -> bool {
    match sink.0.as_ref() {
        CapabilityInner::LogSink(id) => observability::reveals_secrets(*id),
        _ => false,
    }
}

pub fn logging_discarded_count(sink: &Capability) -> u64 {
    match sink.0.as_ref() {
        CapabilityInner::LogSink(id) => observability::discarded_count(*id),
        _ => 0,
    }
}

pub fn logging_drain(sink: &Capability) -> ResultValue {
    match sink.0.as_ref() {
        CapabilityInner::LogSink(id) => {
            observability::drain_memory(*id).map_or_else(ResultValue::error, |entries| {
                ResultValue {
                    entries,
                    ..ResultValue::default()
                }
            })
        }
        CapabilityInner::Invalid(message) => ResultValue::error(message.clone()),
        _ => ResultValue::error("capability is not a logging sink"),
    }
}

pub fn logging_drain_fallback() -> ResultValue {
    observability::drain_fallback().map_or_else(ResultValue::error, |entries| ResultValue {
        entries,
        ..ResultValue::default()
    })
}

pub fn logging_install_dependency_bridge(sink: &Capability) -> ResultValue {
    match sink.0.as_ref() {
        CapabilityInner::LogSink(id) => observability::install_dependency_bridge(*id)
            .map_or_else(ResultValue::error, |()| ResultValue::default()),
        CapabilityInner::Invalid(message) => ResultValue::error(message.clone()),
        _ => ResultValue::error("capability is not a logging sink"),
    }
}
fn capability_type_error(value: &CapabilityInner, expected: &str) -> ResultValue {
    match value {
        CapabilityInner::Invalid(message) => ResultValue::error(message.clone()),
        _ => ResultValue::error(format!("capability is not {expected}")),
    }
}

fn deadline_error(message: &str) -> ResultValue {
    ResultValue {
        failed: true,
        deadline_exceeded: true,
        message: message.to_owned(),
        ..ResultValue::default()
    }
}

fn operation_deadline(nanoseconds: i128, label: &str) -> Result<Option<Instant>, ResultValue> {
    let Some(duration) = timeout(nanoseconds)? else {
        return Ok(None);
    };
    Instant::now()
        .checked_add(duration)
        .map(Some)
        .ok_or_else(|| ResultValue::error(format!("{label} is outside the platform time range")))
}

pub fn int_mutex(initial: i128) -> ResultValue {
    capability_result(Capability(Arc::new(CapabilityInner::IntMutex(Mutex::new(
        initial,
    )))))
}

pub fn int_mutex_load(value: &Capability) -> ResultValue {
    let CapabilityInner::IntMutex(mutex) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "an int mutex");
    };
    mutex.lock().map_or_else(
        |_| lock_error("mutex"),
        |value| ResultValue {
            number: *value,
            flag: true,
            ..ResultValue::default()
        },
    )
}

pub fn int_mutex_store(value: &Capability, replacement: i128) -> ResultValue {
    let CapabilityInner::IntMutex(mutex) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "an int mutex");
    };
    let Ok(mut value) = mutex.lock() else {
        return lock_error("mutex");
    };
    *value = replacement;
    ResultValue::default()
}

pub fn int_mutex_add(value: &Capability, amount: i128) -> ResultValue {
    let CapabilityInner::IntMutex(mutex) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "an int mutex");
    };
    let Ok(mut value) = mutex.lock() else {
        return lock_error("mutex");
    };
    let Some(next) = value.checked_add(amount) else {
        return ResultValue::error("mutex update overflows the platform int envelope");
    };
    *value = next;
    ResultValue {
        number: next,
        flag: true,
        ..ResultValue::default()
    }
}

pub fn int_rw_lock(initial: i128) -> ResultValue {
    capability_result(Capability(Arc::new(CapabilityInner::IntRwLock(
        RwLock::new(initial),
    ))))
}

pub fn int_rw_lock_read(value: &Capability) -> ResultValue {
    let CapabilityInner::IntRwLock(lock) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "an int read/write lock");
    };
    lock.read().map_or_else(
        |_| lock_error("read/write lock"),
        |value| ResultValue {
            number: *value,
            flag: true,
            ..ResultValue::default()
        },
    )
}

pub fn int_rw_lock_write(value: &Capability, replacement: i128) -> ResultValue {
    let CapabilityInner::IntRwLock(lock) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "an int read/write lock");
    };
    let Ok(mut value) = lock.write() else {
        return lock_error("read/write lock");
    };
    *value = replacement;
    ResultValue::default()
}

fn atomic_load_ordering(name: &str) -> Result<Ordering, ResultValue> {
    match name {
        "relaxed" => Ok(Ordering::Relaxed),
        "acquire" => Ok(Ordering::Acquire),
        "sequentially-consistent" => Ok(Ordering::SeqCst),
        _ => Err(ResultValue::error(format!(
            "invalid load memory ordering `{name}`"
        ))),
    }
}

fn atomic_store_ordering(name: &str) -> Result<Ordering, ResultValue> {
    match name {
        "relaxed" => Ok(Ordering::Relaxed),
        "release" => Ok(Ordering::Release),
        "sequentially-consistent" => Ok(Ordering::SeqCst),
        _ => Err(ResultValue::error(format!(
            "invalid store memory ordering `{name}`"
        ))),
    }
}

fn atomic_rmw_ordering(name: &str) -> Result<Ordering, ResultValue> {
    match name {
        "relaxed" => Ok(Ordering::Relaxed),
        "acquire" => Ok(Ordering::Acquire),
        "release" => Ok(Ordering::Release),
        "acquire-release" => Ok(Ordering::AcqRel),
        "sequentially-consistent" => Ok(Ordering::SeqCst),
        _ => Err(ResultValue::error(format!(
            "invalid read-modify-write memory ordering `{name}`"
        ))),
    }
}

pub fn atomic_int64(initial: i128) -> ResultValue {
    match i64::try_from(initial) {
        Ok(initial) => capability_result(Capability(Arc::new(CapabilityInner::AtomicI64(
            AtomicI64::new(initial),
        )))),
        Err(_) => ResultValue::error("atomic int64 initial value is out of range"),
    }
}

pub fn atomic_int64_load(value: &Capability, ordering: &str) -> ResultValue {
    let CapabilityInner::AtomicI64(atomic) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "an atomic int64");
    };
    let ordering = match atomic_load_ordering(ordering) {
        Ok(ordering) => ordering,
        Err(error) => return error,
    };
    ResultValue {
        number: i128::from(atomic.load(ordering)),
        flag: true,
        ..ResultValue::default()
    }
}

pub fn atomic_int64_store(value: &Capability, replacement: i128, ordering: &str) -> ResultValue {
    let CapabilityInner::AtomicI64(atomic) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "an atomic int64");
    };
    let Ok(replacement) = i64::try_from(replacement) else {
        return ResultValue::error("atomic int64 value is out of range");
    };
    let ordering = match atomic_store_ordering(ordering) {
        Ok(ordering) => ordering,
        Err(error) => return error,
    };
    atomic.store(replacement, ordering);
    ResultValue::default()
}

pub fn atomic_int64_add(value: &Capability, amount: i128, ordering: &str) -> ResultValue {
    let CapabilityInner::AtomicI64(atomic) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "an atomic int64");
    };
    let Ok(amount) = i64::try_from(amount) else {
        return ResultValue::error("atomic int64 amount is out of range");
    };
    let ordering = match atomic_rmw_ordering(ordering) {
        Ok(ordering) => ordering,
        Err(error) => return error,
    };
    let failure_ordering = match ordering {
        Ordering::Acquire | Ordering::AcqRel => Ordering::Acquire,
        Ordering::SeqCst => Ordering::SeqCst,
        _ => Ordering::Relaxed,
    };
    let mut previous = atomic.load(failure_ordering);
    let next = loop {
        let Some(next) = previous.checked_add(amount) else {
            return ResultValue::error("atomic int64 addition overflowed");
        };
        match atomic.compare_exchange_weak(previous, next, ordering, failure_ordering) {
            Ok(_) => break next,
            Err(observed) => previous = observed,
        }
    };
    ResultValue {
        number: i128::from(next),
        flag: true,
        ..ResultValue::default()
    }
}

pub fn thread_local_int(initial: i128) -> ResultValue {
    capability_result(Capability(Arc::new(CapabilityInner::ThreadLocalInt(
        ThreadLocalInt {
            id: NEXT_THREAD_LOCAL_ID.fetch_add(1, Ordering::Relaxed),
            initial,
            owner: Arc::new(()),
        },
    ))))
}

pub fn thread_local_int_get(value: &Capability) -> ResultValue {
    let CapabilityInner::ThreadLocalInt(local) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "a thread-local int");
    };
    let number = THREAD_LOCAL_INTS.with(|values| {
        let mut values = values.borrow_mut();
        values.retain(|_, (owner, _)| owner.upgrade().is_some());
        values
            .entry(local.id)
            .or_insert_with(|| (Arc::downgrade(&local.owner), local.initial))
            .1
    });
    ResultValue {
        number,
        flag: true,
        ..ResultValue::default()
    }
}

pub fn thread_local_int_set(value: &Capability, replacement: i128) -> ResultValue {
    let CapabilityInner::ThreadLocalInt(local) = value.0.as_ref() else {
        return capability_type_error(value.0.as_ref(), "a thread-local int");
    };
    THREAD_LOCAL_INTS.with(|values| {
        let mut values = values.borrow_mut();
        values.retain(|_, (owner, _)| owner.upgrade().is_some());
        values.insert(local.id, (Arc::downgrade(&local.owner), replacement));
    });
    ResultValue::default()
}

pub fn cancellation_token() -> Capability {
    Capability(Arc::new(CapabilityInner::Cancellation(CancellationState {
        cancelled: AtomicBool::new(false),
        notify: tokio::sync::Notify::new(),
    })))
}

pub fn cancel(token: &Capability) -> ResultValue {
    let CapabilityInner::Cancellation(state) = token.0.as_ref() else {
        return ResultValue::error("capability is not a cancellation token");
    };
    state.cancelled.store(true, Ordering::Release);
    state.notify.notify_waiters();
    ResultValue::default()
}

fn is_cancelled(token: &Capability) -> Result<bool, ResultValue> {
    let CapabilityInner::Cancellation(state) = token.0.as_ref() else {
        return Err(ResultValue::error("capability is not a cancellation token"));
    };
    Ok(state.cancelled.load(Ordering::Acquire))
}

fn cancellation_error(token: Option<&Capability>) -> Option<ResultValue> {
    match is_cancelled(token?) {
        Ok(true) => Some(ResultValue::error("operation cancelled")),
        Ok(false) => None,
        Err(error) => Some(error),
    }
}

pub fn check_cancellation(token: Option<&Capability>) -> ResultValue {
    cancellation_error(token).unwrap_or_default()
}

async fn cancellation_wait(token: &Capability) -> ResultValue {
    let CapabilityInner::Cancellation(state) = token.0.as_ref() else {
        return ResultValue::error("capability is not a cancellation token");
    };
    let notified = state.notify.notified();
    if state.cancelled.load(Ordering::Acquire) {
        return ResultValue::error("operation cancelled");
    }
    notified.await;
    ResultValue::error("operation cancelled")
}

async fn readiness_operation<'a>(
    operation: impl Future<Output = ResultValue>,
    deadline_nanos: i128,
    deadline_message: &'static str,
    cancellation: impl Into<Option<&'a Capability>>,
) -> ResultValue {
    let duration = match timeout(deadline_nanos) {
        Ok(value) => value,
        Err(error) => return error,
    };
    if duration.is_some_and(|duration| duration.is_zero()) {
        return deadline_error(deadline_message);
    }
    if let Some(cancellation) = cancellation.into() {
        if let Some(duration) = duration {
            tokio::select! {
                biased;
                cancelled = cancellation_wait(cancellation) => cancelled,
                result = operation => result,
                () = tokio::time::sleep(duration) => deadline_error(deadline_message),
            }
        } else {
            tokio::select! {
                biased;
                cancelled = cancellation_wait(cancellation) => cancelled,
                result = operation => result,
            }
        }
    } else if let Some(duration) = duration {
        tokio::select! {
            result = operation => result,
            () = tokio::time::sleep(duration) => deadline_error(deadline_message),
        }
    } else {
        operation.await
    }
}

pub fn secure_random() -> Capability {
    Capability(Arc::new(CapabilityInner::Secure))
}

pub fn pseudo_random(algorithm: &str, seed: &[u8]) -> Capability {
    if algorithm != "chacha20" {
        return Capability(Arc::new(CapabilityInner::Invalid(format!(
            "unsupported pseudo-random algorithm: {algorithm}"
        ))));
    }
    let mut expanded = [0_u8; 32];
    let digest = sha2::Sha256::digest(seed);
    expanded.copy_from_slice(&digest);
    Capability(Arc::new(CapabilityInner::Pseudo(Mutex::new(
        rand_chacha::ChaCha20Rng::from_seed(expanded),
    ))))
}

pub fn secret_buffer(data: Vec<u8>) -> Capability {
    Capability(Arc::new(CapabilityInner::Secret(Mutex::new(SecretState {
        bytes: Zeroizing::new(data),
        destroyed: false,
    }))))
}

pub fn destroy_secret(secret: &Capability) -> ResultValue {
    let CapabilityInner::Secret(secret) = secret.0.as_ref() else {
        return ResultValue::error("capability is not a secret buffer");
    };
    let mut secret = secret.lock().expect("secret lock poisoned");
    secret.bytes.clear();
    secret.destroyed = true;
    ResultValue::default()
}

pub fn random_bytes(source: &Capability, requested: i128) -> ResultValue {
    let size = match count(requested, "random byte count") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let mut data = vec![0; size];
    match source.0.as_ref() {
        CapabilityInner::Secure => match getrandom::fill(&mut data) {
            Ok(()) => ResultValue {
                data,
                ..ResultValue::default()
            },
            Err(error) => ResultValue::error(format!("secure entropy unavailable: {error}")),
        },
        CapabilityInner::Pseudo(generator) => {
            generator
                .lock()
                .expect("pseudo-random lock poisoned")
                .fill_bytes(&mut data);
            ResultValue {
                data,
                ..ResultValue::default()
            }
        }
        CapabilityInner::Invalid(message) => ResultValue::error(message.clone()),
        _ => ResultValue::error("capability is not a random source"),
    }
}

pub fn random_bounded(source: &Capability, upper: i128) -> ResultValue {
    let Ok(bound) = u128::try_from(upper) else {
        return ResultValue::error("random upper bound must be positive");
    };
    if bound == 0 {
        return ResultValue::error("random upper bound must be positive");
    }
    let zone = u128::MAX - (u128::MAX % bound);
    loop {
        let bytes = random_bytes(source, 16);
        if bytes.failed {
            return bytes;
        }
        let mut raw = [0_u8; 16];
        raw.copy_from_slice(&bytes.data);
        let candidate = u128::from_le_bytes(raw);
        if candidate < zone {
            return ResultValue {
                number: i128::try_from(candidate % bound).expect("bounded result fits i128"),
                ..ResultValue::default()
            };
        }
    }
}

pub fn random_split(source: &Capability) -> ResultValue {
    match source.0.as_ref() {
        CapabilityInner::Pseudo(_) => {
            let seed = random_bytes(source, 32);
            if seed.failed {
                seed
            } else {
                ResultValue {
                    capability: Some(pseudo_random("chacha20", &seed.data)),
                    ..ResultValue::default()
                }
            }
        }
        CapabilityInner::Invalid(message) => ResultValue::error(message.clone()),
        _ => ResultValue::error("only a pseudo-random source can be split"),
    }
}

pub fn digest(algorithm: &str, data: &[u8]) -> ResultValue {
    let output = match algorithm {
        "md5" => md5::Md5::digest(data).to_vec(),
        "sha-1" => sha1::Sha1::digest(data).to_vec(),
        "sha-256" => sha2::Sha256::digest(data).to_vec(),
        "sha-512" => sha2::Sha512::digest(data).to_vec(),
        _ => return ResultValue::error("unsupported digest algorithm"),
    };
    ResultValue {
        data: output,
        text: algorithm.to_owned(),
        ..ResultValue::default()
    }
}

pub fn hmac(algorithm: &str, key: &Capability, data: &[u8]) -> ResultValue {
    let CapabilityInner::Secret(secret) = key.0.as_ref() else {
        return ResultValue::error("HMAC key is not a secret buffer");
    };
    let secret = secret.lock().expect("secret lock poisoned");
    if secret.destroyed {
        return ResultValue::error("secret buffer was destroyed");
    }
    let output = match algorithm {
        "md5" => {
            let mut mac = hmac::Hmac::<md5::Md5>::new_from_slice(secret.bytes.as_slice())
                .expect("HMAC accepts every key length");
            mac.update(data);
            mac.finalize().into_bytes().to_vec()
        }
        "sha-1" => {
            let mut mac = hmac::Hmac::<sha1::Sha1>::new_from_slice(secret.bytes.as_slice())
                .expect("HMAC accepts every key length");
            mac.update(data);
            mac.finalize().into_bytes().to_vec()
        }
        "sha-256" => {
            let mut mac = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret.bytes.as_slice())
                .expect("HMAC accepts every key length");
            mac.update(data);
            mac.finalize().into_bytes().to_vec()
        }
        "sha-512" => {
            let mut mac = hmac::Hmac::<sha2::Sha512>::new_from_slice(secret.bytes.as_slice())
                .expect("HMAC accepts every key length");
            mac.update(data);
            mac.finalize().into_bytes().to_vec()
        }
        _ => return ResultValue::error("unsupported HMAC algorithm"),
    };
    ResultValue {
        data: output,
        text: algorithm.to_owned(),
        ..ResultValue::default()
    }
}

pub fn constant_time_equal(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len() && bool::from(left.ct_eq(right))
}

pub fn hex_encode(data: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(data.len() * 2);
    for byte in data {
        encoded.push(DIGITS[usize::from(byte >> 4)] as char);
        encoded.push(DIGITS[usize::from(byte & 0x0f)] as char);
    }
    encoded
}

pub fn hex_decode(text: &str) -> ResultValue {
    if !text.len().is_multiple_of(2)
        || !text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return ResultValue::error("invalid strict lowercase hexadecimal encoding");
    }
    let data = (0..text.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&text[index..index + 2], 16).expect("validated hex"))
        .collect();
    ResultValue {
        data,
        ..ResultValue::default()
    }
}

pub fn base64_encode(data: &[u8], url_safe: bool, padded: bool) -> String {
    match (url_safe, padded) {
        (false, true) => base64::engine::general_purpose::STANDARD.encode(data),
        (false, false) => base64::engine::general_purpose::STANDARD_NO_PAD.encode(data),
        (true, true) => base64::engine::general_purpose::URL_SAFE.encode(data),
        (true, false) => base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data),
    }
}

pub fn base64_decode(text: &str, url_safe: bool, padded: bool) -> ResultValue {
    let decoded = match (url_safe, padded) {
        (false, true) => base64::engine::general_purpose::STANDARD.decode(text),
        (false, false) => base64::engine::general_purpose::STANDARD_NO_PAD.decode(text),
        (true, true) => base64::engine::general_purpose::URL_SAFE.decode(text),
        (true, false) => base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(text),
    };
    match decoded {
        Ok(data) => ResultValue {
            data,
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("invalid base64 encoding: {error}")),
    }
}

pub fn uuid_parse(text: &str) -> ResultValue {
    match uuid::Uuid::parse_str(text) {
        Ok(value) if value.hyphenated().to_string() == text => ResultValue {
            text: text.to_owned(),
            data: value.as_bytes().to_vec(),
            ..ResultValue::default()
        },
        Ok(_) => ResultValue::error("UUID must use canonical lowercase hyphenated form"),
        Err(error) => ResultValue::error(format!("invalid UUID: {error}")),
    }
}
pub fn uuid_v4(source: &Capability) -> ResultValue {
    let bytes = random_bytes(source, 16);
    if bytes.failed {
        return bytes;
    }
    let mut raw = [0_u8; 16];
    raw.copy_from_slice(&bytes.data);
    let value = uuid::Builder::from_random_bytes(raw).into_uuid();
    ResultValue {
        text: value.hyphenated().to_string(),
        data: value.as_bytes().to_vec(),
        ..ResultValue::default()
    }
}

pub fn uuid_v7(source: &Capability, unix_milliseconds: i128) -> ResultValue {
    let Ok(timestamp) = u64::try_from(unix_milliseconds) else {
        return ResultValue::error("UUID v7 timestamp must be non-negative");
    };
    if timestamp >= (1_u64 << 48) {
        return ResultValue::error("UUID v7 timestamp exceeds 48 bits");
    }
    let random = random_bytes(source, 10);
    if random.failed {
        return random;
    }
    let mut raw = [0_u8; 16];
    let timestamp_bytes = timestamp.to_be_bytes();
    raw[..6].copy_from_slice(&timestamp_bytes[2..]);
    raw[6..].copy_from_slice(&random.data);
    raw[6] = (raw[6] & 0x0f) | 0x70;
    raw[8] = (raw[8] & 0x3f) | 0x80;
    let value = uuid::Uuid::from_bytes(raw);
    ResultValue {
        text: value.hyphenated().to_string(),
        data: value.as_bytes().to_vec(),
        ..ResultValue::default()
    }
}

pub fn compress(format: &str, input: &[u8], level: i128, deterministic: bool) -> ResultValue {
    let level = i32::try_from(level).unwrap_or(6);
    let result = match format {
        "gzip" => {
            let mut encoder = flate2::GzBuilder::new()
                .mtime(if deterministic {
                    0
                } else {
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs() as u32
                })
                .write(
                    Vec::new(),
                    flate2::Compression::new(level.clamp(0, 9) as u32),
                );
            encoder.write_all(input).and_then(|()| encoder.finish())
        }
        "zlib" => {
            let mut encoder = flate2::write::ZlibEncoder::new(
                Vec::new(),
                flate2::Compression::new(level.clamp(0, 9) as u32),
            );
            encoder.write_all(input).and_then(|()| encoder.finish())
        }
        "deflate-raw" => {
            let mut encoder = flate2::write::DeflateEncoder::new(
                Vec::new(),
                flate2::Compression::new(level.clamp(0, 9) as u32),
            );
            encoder.write_all(input).and_then(|()| encoder.finish())
        }
        "zstd" => {
            return match zstd::stream::encode_all(input, level) {
                Ok(data) => ResultValue {
                    data,
                    ..ResultValue::default()
                },
                Err(error) => ResultValue::error(format!("zstd compression failed: {error}")),
            };
        }
        _ => {
            return ResultValue::error(
                "unknown compression format; auto-detection is not supported",
            );
        }
    };
    match result {
        Ok(data) => ResultValue {
            data,
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("compression failed: {error}")),
    }
}

pub fn decompress(
    format: &str,
    input: &[u8],
    output_limit: i128,
    ratio_limit: i128,
    work_limit: i128,
) -> ResultValue {
    let output_limit = match count(output_limit, "output limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let work_limit = match count(work_limit, "work limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    if ratio_limit <= 0 {
        return ResultValue::limit("decompression ratio limit must be positive");
    }
    if input.len() > work_limit {
        return ResultValue::limit("decompression work-byte limit exceeded");
    }
    let maximum = output_limit
        .min(work_limit.saturating_sub(input.len()))
        .saturating_add(1);
    let data = match decompress_one(format, input, maximum, output_limit) {
        Ok(value) => value,
        Err(error) => return error,
    };
    if data.len() > output_limit || input.len().saturating_add(data.len()) > work_limit {
        return ResultValue::limit("decompression output or work-byte limit exceeded");
    }
    if !input.is_empty() && data.len() as u128 > input.len() as u128 * ratio_limit as u128 {
        return ResultValue::limit("decompression ratio limit exceeded");
    }
    ResultValue {
        data,
        ..ResultValue::default()
    }
}

fn decompress_one(
    format: &str,
    input: &[u8],
    maximum: usize,
    output_limit: usize,
) -> Result<Vec<u8>, ResultValue> {
    let reader: Box<dyn Read> = match format {
        "gzip" => Box::new(flate2::read::MultiGzDecoder::new(input)),
        "zlib" => Box::new(flate2::read::ZlibDecoder::new(input)),
        "deflate-raw" => Box::new(flate2::read::DeflateDecoder::new(input)),
        "zstd" => {
            let mut decoder = zstd::stream::read::Decoder::new(input).map_err(|error| {
                ResultValue::error(format!("zstd decompression failed: {error}"))
            })?;
            let window_log = output_limit.max(1).saturating_sub(1).bit_width();
            decoder
                .window_log_max(window_log.max(10))
                .map_err(|error| {
                    ResultValue::error(format!(
                        "cannot set zstd decompression window limit: {error}"
                    ))
                })?;
            Box::new(decoder)
        }
        _ => {
            return Err(ResultValue::error(
                "unknown compression format; auto-detection is not supported",
            ));
        }
    };
    let mut data = Vec::new();
    reader
        .take(maximum as u64)
        .read_to_end(&mut data)
        .map_err(|error| ResultValue::error(format!("decompression failed: {error}")))?;
    Ok(data)
}

pub fn parse_ip(text: &str) -> ResultValue {
    match text.parse::<IpAddr>() {
        Ok(ip) => ResultValue {
            text: ip.to_string(),
            detail: if ip.is_ipv4() { "ipv4" } else { "ipv6" }.to_owned(),
            flag: ip.is_loopback(),
            ..ResultValue::default()
        },
        Err(error) => ResultValue::error(format!("invalid IP address: {error}")),
    }
}
pub fn parse_host_name(text: &str) -> ResultValue {
    let ascii = match idna::domain_to_ascii(text) {
        Ok(value) => value,
        Err(error) => return ResultValue::error(format!("invalid host name: {error}")),
    };
    if ascii.is_empty() || ascii.len() > 253 {
        return ResultValue::error(
            "host name must contain 1..=253 ASCII bytes after UTS-46 processing",
        );
    }
    let valid = ascii.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    });
    if !valid {
        return ResultValue::error("invalid host name");
    }
    ResultValue {
        text: ascii.to_ascii_lowercase(),
        ..ResultValue::default()
    }
}

pub fn parse_socket(ip: &str, port: i128) -> ResultValue {
    let Ok(ip) = ip.parse::<IpAddr>() else {
        return ResultValue::error("invalid IP address");
    };
    let Ok(port) = u16::try_from(port) else {
        return ResultValue::error("port must be in 0..=65535");
    };
    let address = SocketAddr::new(ip, port);
    ResultValue {
        text: address.to_string(),
        number: i128::from(port),
        ..ResultValue::default()
    }
}

pub fn parse_socket_text(text: &str) -> ResultValue {
    let Ok(address) = text.parse::<SocketAddr>() else {
        return ResultValue::error("invalid socket address");
    };
    ResultValue {
        text: address.to_string(),
        detail: address.ip().to_string(),
        number: i128::from(address.port()),
        ..ResultValue::default()
    }
}
fn timeout(nanoseconds: i128) -> Result<Option<Duration>, ResultValue> {
    if nanoseconds == -1 {
        return Ok(None);
    }
    let value = u64::try_from(nanoseconds)
        .map_err(|_| ResultValue::error("deadline duration exceeds the platform timer range"))?;
    Ok(Some(Duration::from_nanos(value)))
}

pub fn tls_client(
    stream: &Capability,
    server_name: &str,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let roots = webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned()
        .collect::<rustls::RootCertStore>();
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    tls_client_with_config(
        stream,
        server_name,
        deadline_nanos,
        cancellation,
        Arc::new(config),
    )
}

fn tls_client_with_config(
    stream: &Capability,
    server_name: &str,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
    config: Arc<rustls::ClientConfig>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_nanos) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("TLS requires a TCP stream");
    };
    let tcp = match stream.take_std() {
        Ok(stream) => stream,
        Err(error) => return error,
    };
    if let Err(error) = tcp.set_read_timeout(duration) {
        return io_error("TLS read timeout configuration", &error);
    }
    if let Err(error) = tcp.set_write_timeout(duration) {
        return io_error("TLS write timeout configuration", &error);
    }
    let Ok(name) = rustls::pki_types::ServerName::try_from(server_name.to_owned()) else {
        return ResultValue::error("invalid TLS server name");
    };
    let Ok(connection) = rustls::ClientConnection::new(config, name) else {
        return ResultValue::error("cannot create TLS client");
    };
    let mut tls = rustls::StreamOwned::new(connection, tcp);
    match tls.flush() {
        Ok(()) => {
            let version = match tls.conn.protocol_version() {
                Some(rustls::ProtocolVersion::TLSv1_3) => "TLS 1.3",
                Some(rustls::ProtocolVersion::TLSv1_2) => "TLS 1.2",
                Some(_) | None => "unknown",
            };
            ResultValue {
                text: version.to_owned(),
                capability: Some(Capability(Arc::new(CapabilityInner::Tls(Mutex::new(
                    Some(tls),
                ))))),
                ..ResultValue::default()
            }
        }
        Err(error) => io_error("TLS handshake", &error),
    }
}
pub fn tls_read(
    stream: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let size = match count(limit, "TLS read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let duration = match timeout(deadline_nanos) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tls(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TLS stream");
    };
    let mut guard = stream.lock().expect("TLS lock poisoned");
    let Some(stream) = guard.as_mut() else {
        return ResultValue::error("TLS stream is closed");
    };
    if let Err(error) = stream.sock.set_read_timeout(duration) {
        return io_error("TLS read timeout configuration", &error);
    }
    let mut data = vec![0; size];
    match stream.read(&mut data) {
        Ok(count) => {
            data.truncate(count);
            ResultValue {
                data,
                number: count as i128,
                flag: count == 0,
                ..ResultValue::default()
            }
        }
        Err(error) => io_error("TLS read", &error),
    }
}
pub fn tls_write(
    stream: &Capability,
    data: &[u8],
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_nanos) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tls(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TLS stream");
    };
    let mut guard = stream.lock().expect("TLS lock poisoned");
    let Some(stream) = guard.as_mut() else {
        return ResultValue::error("TLS stream is closed");
    };
    if let Err(error) = stream.sock.set_write_timeout(duration) {
        return io_error("TLS write timeout configuration", &error);
    }
    match stream.write(data) {
        Ok(count) => ResultValue {
            number: count as i128,
            ..ResultValue::default()
        },
        Err(error) => io_error("TLS write", &error),
    }
}
pub fn tls_shutdown(
    stream: &Capability,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_nanos) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tls(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TLS stream");
    };
    let mut guard = stream.lock().expect("TLS lock poisoned");
    let Some(stream) = guard.as_mut() else {
        return ResultValue::error("TLS stream is closed");
    };
    if let Err(error) = stream.sock.set_write_timeout(duration) {
        return io_error("TLS shutdown timeout configuration", &error);
    }
    stream.conn.send_close_notify();
    match stream.flush() {
        Ok(()) => ResultValue::default(),
        Err(error) => io_error("TLS shutdown", &error),
    }
}

fn take_readiness_tcp_stream(stream: &Capability) -> Result<tokio::net::TcpStream, ResultValue> {
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return Err(ResultValue::error("TLS requires a TCP stream"));
    };
    stream.take_async()
}

pub async fn tls_client_async(
    stream: &Capability,
    server_name: &str,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let roots = webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned()
        .collect::<rustls::RootCertStore>();
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    tls_client_async_with_config(
        stream,
        server_name,
        deadline_nanos,
        cancellation,
        Arc::new(config),
    )
    .await
}

async fn tls_client_async_with_config(
    stream: &Capability,
    server_name: &str,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
    config: Arc<rustls::ClientConfig>,
) -> ResultValue {
    let stream = match take_readiness_tcp_stream(stream) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let Ok(name) = rustls::pki_types::ServerName::try_from(server_name.to_owned()) else {
        return ResultValue::error("invalid TLS server name");
    };
    readiness_operation(
        async move {
            match tokio_rustls::TlsConnector::from(config)
                .connect(name, stream)
                .await
            {
                Ok(stream) => {
                    let version = match stream.get_ref().1.protocol_version() {
                        Some(rustls::ProtocolVersion::TLSv1_3) => "TLS 1.3",
                        Some(rustls::ProtocolVersion::TLSv1_2) => "TLS 1.2",
                        Some(_) | None => "unknown",
                    };
                    ResultValue {
                        text: version.to_owned(),
                        capability: Some(Capability(Arc::new(CapabilityInner::AsyncTls(
                            AsyncTlsState::new(stream),
                        )))),
                        ..ResultValue::default()
                    }
                }
                Err(error) => ResultValue::error(format!("TLS handshake failed: {error}")),
            }
        },
        deadline_nanos,
        "TLS handshake deadline exceeded",
        cancellation,
    )
    .await
}

pub async fn tls_read_async(
    stream: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let size = match count(limit, "TLS read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::AsyncTls(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not an asynchronous TLS stream");
    };
    let mut guard = stream.reader.lock().await;
    let Some(stream) = guard.as_mut() else {
        return ResultValue::error("TLS stream is closed");
    };
    if size == 0 {
        return ResultValue::default();
    }
    readiness_operation(
        async {
            use tokio::io::AsyncReadExt as _;
            let mut data = vec![0; size];
            match stream.read(&mut data).await {
                Ok(count) => {
                    data.truncate(count);
                    ResultValue {
                        data,
                        number: count as i128,
                        flag: count == 0,
                        ..ResultValue::default()
                    }
                }
                Err(error) => io_error("TLS read", &error),
            }
        },
        deadline_nanos,
        "TLS read deadline exceeded",
        cancellation,
    )
    .await
}

pub async fn tls_write_async(
    stream: &Capability,
    data: &[u8],
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let CapabilityInner::AsyncTls(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not an asynchronous TLS stream");
    };
    let mut guard = stream.writer.lock().await;
    let Some(stream) = guard.as_mut() else {
        return ResultValue::error("TLS stream is closed");
    };
    readiness_operation(
        async {
            use tokio::io::AsyncWriteExt as _;
            match stream.write(data).await {
                Ok(count) => ResultValue {
                    number: count as i128,
                    ..ResultValue::default()
                },
                Err(error) => io_error("TLS write", &error),
            }
        },
        deadline_nanos,
        "TLS write deadline exceeded",
        cancellation,
    )
    .await
}
pub async fn tls_read_into_async(
    stream: &Capability,
    buffer: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    networking_buffers::tls_read_into(stream, buffer, limit, deadline_nanos, cancellation).await
}

pub async fn tls_shutdown_async(
    stream: &Capability,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let CapabilityInner::AsyncTls(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not an asynchronous TLS stream");
    };
    let mut guard = stream.writer.lock().await;
    let Some(stream) = guard.as_mut() else {
        return ResultValue::error("TLS stream is closed");
    };
    readiness_operation(
        async {
            use tokio::io::AsyncWriteExt as _;
            match stream.shutdown().await {
                Ok(()) => ResultValue::default(),
                Err(error) => io_error("TLS shutdown", &error),
            }
        },
        deadline_nanos,
        "TLS shutdown deadline exceeded",
        cancellation,
    )
    .await
}

pub fn buffer_create(capacity: i128) -> ResultValue {
    networking_buffers::buffer_create(capacity)
}

pub fn buffer_clear(buffer: &Capability) -> ResultValue {
    networking_buffers::buffer_clear(buffer)
}
pub async fn tcp_read_into_async(
    stream: &Capability,
    buffer: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    networking_buffers::tcp_read_into(stream, buffer, limit, deadline_nanos, cancellation).await
}

pub fn platform_buffer_capacity(buffer: &Capability) -> ResultValue {
    networking_buffers::buffer_capacity(buffer)
}

pub fn platform_buffer_completed(buffer: &Capability) -> ResultValue {
    networking_buffers::buffer_completed(buffer)
}

pub fn buffer_filled_bytes(buffer: &Capability) -> ResultValue {
    networking_buffers::buffer_filled_bytes(buffer)
}

pub fn buffer_filled_text(buffer: &Capability) -> ResultValue {
    networking_buffers::buffer_filled_text(buffer)
}

pub fn buffer_read_with(
    buffer: &Capability,
    read: impl FnOnce(&mut [u8]) -> std::io::Result<usize>,
) -> ResultValue {
    networking_buffers::buffer_read_with(buffer, read)
}

// Native aliases may keep a TLS half borrowed by a pending operation. Close is
// nonblocking and reports busy without disturbing either half; finish/cancel and
// join that operation before retrying. Source consuming calls exclude such borrows.
pub fn close(capability: &Capability) -> ResultValue {
    match capability.0.as_ref() {
        CapabilityInner::Listener(value) => {
            value.close();
            ResultValue::default()
        }
        CapabilityInner::Tcp(value) => {
            value.close();
            ResultValue::default()
        }
        CapabilityInner::Udp(value) => {
            value.close();
            ResultValue::default()
        }
        CapabilityInner::Tls(value) => {
            value.lock().expect("TLS lock poisoned").take();
            ResultValue::default()
        }
        CapabilityInner::AsyncTls(value) => {
            match (value.reader.try_lock(), value.writer.try_lock()) {
                (Ok(mut reader), Ok(mut writer)) => {
                    reader.take();
                    writer.take();
                    ResultValue::default()
                }
                _ => ResultValue::error("TLS stream is busy"),
            }
        }
        CapabilityInner::Buffer(buffer) => match buffer.try_lock() {
            Ok(mut buffer) => {
                buffer.closed = true;
                buffer.bytes = Vec::new();
                buffer.filled = 0;
                ResultValue::default()
            }
            Err(_) => ResultValue::error("read buffer is busy"),
        },
        _ => ResultValue::error("capability is not a closeable resource"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn result_capability(result: ResultValue) -> Capability {
        assert!(!result.failed, "{}", result.message);
        result.capability.expect("successful capability result")
    }

    #[test]
    fn owned_read_buffer_reuses_storage_and_exposes_filled_text() {
        let buffer = result_capability(buffer_create(8));
        assert_eq!(platform_buffer_capacity(&buffer).number, 8);
        let read = buffer_read_with(&buffer, |storage| {
            storage[..5].copy_from_slice(b"hello");
            Ok(5)
        });
        assert_eq!(read.number, 5);
        assert_eq!(buffer_filled_text(&buffer).text, "hello");
        assert!(!buffer_clear(&buffer).failed);
        assert_eq!(buffer_filled_bytes(&buffer).data, b"");
        assert!(!close(&buffer).failed);
    }

    #[test]
    fn read_buffer_metadata_tracks_mutation_and_closed_state() {
        let buffer = result_capability(buffer_create(4));
        assert_eq!(platform_buffer_capacity(&buffer).number, 4);
        assert_eq!(platform_buffer_completed(&buffer).number, 0);
        assert_eq!(
            buffer_read_with(&buffer, |bytes| {
                bytes[..3].copy_from_slice(b"abc");
                Ok(3)
            })
            .number,
            3
        );
        assert_eq!(platform_buffer_completed(&buffer).number, 3);
        assert_eq!(buffer_filled_bytes(&buffer).data, b"abc");
        assert!(!buffer_clear(&buffer).failed);
        assert_eq!(platform_buffer_completed(&buffer).number, 0);
        assert!(!close(&buffer).failed);
        assert!(platform_buffer_capacity(&buffer).failed);
        assert!(platform_buffer_completed(&buffer).failed);
    }
    #[test]
    fn concurrency_capabilities_share_state_and_isolate_thread_locals() {
        let mutex = result_capability(int_mutex(2));
        let shared_mutex = mutex.clone();
        std::thread::spawn(move || {
            assert_eq!(int_mutex_add(&shared_mutex, 3).number, 5);
        })
        .join()
        .unwrap();
        assert_eq!(int_mutex_load(&mutex).number, 5);

        let read_write_lock = result_capability(int_rw_lock(7));
        let shared_lock = read_write_lock.clone();
        std::thread::spawn(move || {
            assert!(!int_rw_lock_write(&shared_lock, 11).failed);
        })
        .join()
        .unwrap();
        assert_eq!(int_rw_lock_read(&read_write_lock).number, 11);

        let atomic = result_capability(atomic_int64(13));
        let shared_atomic = atomic.clone();
        std::thread::spawn(move || {
            assert_eq!(
                atomic_int64_add(&shared_atomic, 4, "acquire-release").number,
                17
            );
        })
        .join()
        .unwrap();
        assert_eq!(atomic_int64_load(&atomic, "acquire").number, 17);

        let local = result_capability(thread_local_int(19));
        assert_eq!(thread_local_int_get(&local).number, 19);
        let other_thread = local.clone();
        std::thread::spawn(move || {
            assert_eq!(thread_local_int_get(&other_thread).number, 19);
            assert!(!thread_local_int_set(&other_thread, 23).failed);
            assert_eq!(thread_local_int_get(&other_thread).number, 23);
        })
        .join()
        .unwrap();
        assert_eq!(thread_local_int_get(&local).number, 19);
    }

    #[test]
    fn atomic_orderings_are_validated_per_operation() {
        let atomic = result_capability(atomic_int64(1));
        for ordering in ["relaxed", "acquire", "sequentially-consistent"] {
            assert!(!atomic_int64_load(&atomic, ordering).failed, "{ordering}");
        }
        for ordering in ["release", "acquire-release", "invalid"] {
            assert!(atomic_int64_load(&atomic, ordering).failed, "{ordering}");
        }

        for ordering in ["relaxed", "release", "sequentially-consistent"] {
            assert!(
                !atomic_int64_store(&atomic, 1, ordering).failed,
                "{ordering}"
            );
        }
        for ordering in ["acquire", "acquire-release", "invalid"] {
            assert!(
                atomic_int64_store(&atomic, 1, ordering).failed,
                "{ordering}"
            );
        }

        for ordering in [
            "relaxed",
            "acquire",
            "release",
            "acquire-release",
            "sequentially-consistent",
        ] {
            assert!(!atomic_int64_add(&atomic, 0, ordering).failed, "{ordering}");
        }
        assert!(atomic_int64_add(&atomic, 0, "invalid").failed);
    }

    #[test]
    fn concurrency_failures_preserve_messages() {
        let invalid_atomic = atomic_int64(i128::from(i64::MAX) + 1);
        assert!(invalid_atomic.failed);
        assert!(invalid_atomic.message.contains("out of range"));

        let invalid = Capability::default();
        assert_eq!(
            int_mutex_load(&invalid).message,
            "missing platform capability"
        );
    }

    #[test]
    fn thread_local_entries_sweep_dropped_owners() {
        let first = result_capability(thread_local_int(1));
        assert_eq!(thread_local_int_get(&first).number, 1);
        drop(first);

        let second = result_capability(thread_local_int(2));
        assert_eq!(thread_local_int_get(&second).number, 2);
        THREAD_LOCAL_INTS.with(|values| {
            let values = values.borrow();
            assert_eq!(values.len(), 1);
            assert!(values.values().all(|(owner, _)| owner.upgrade().is_some()));
        });
    }

    #[test]
    fn system_host_name_preserves_the_platform_string_contract() {
        let name = system_host_name();
        if name.failed {
            assert_ne!(name.message, "");
        } else {
            assert!(name.flag);
            assert!(name.text.starts_with("text:") || name.text.starts_with("raw:"));
        }
    }

    #[test]
    fn loopback_tcp_exchanges_under_deadline() {
        let listener = tcp_bind("127.0.0.1:0", 128, true, false, 64);
        assert!(!listener.failed);
        let address = listener.text.clone();
        let cancellation = cancellation_token();
        let client = tcp_connect(&address, false, 64, 1_000_000_000, Some(&cancellation));
        assert!(!client.failed);
        let server = tcp_accept(
            listener.capability.as_ref().unwrap(),
            1_000_000_000,
            Some(&cancellation),
        );
        assert!(!server.failed);
        let sent = tcp_write(
            client.capability.as_ref().unwrap(),
            b"terrane",
            1_000_000_000,
            Some(&cancellation),
        );
        assert_eq!(sent.number, 7);
        let received = tcp_read(
            server.capability.as_ref().unwrap(),
            7,
            1_000_000_000,
            Some(&cancellation),
        );
        assert_eq!(received.data, b"terrane");
    }

    #[test]
    fn concurrent_accepts_share_a_listener_without_serializing_on_its_lock() {
        let listener = tcp_bind("127.0.0.1:0", 128, true, false, 64);
        assert!(!listener.failed);
        let listener = listener.capability.unwrap();
        let first_listener = listener.clone();
        let second_listener = listener.clone();
        let first = std::thread::spawn(move || tcp_accept(&first_listener, 1_000_000_000, None));
        let second = std::thread::spawn(move || tcp_accept(&second_listener, 1_000_000_000, None));
        let address = match listener.0.as_ref() {
            CapabilityInner::Listener(listener) => {
                listener.read().as_ref().unwrap().local_addr().unwrap()
            }
            _ => unreachable!(),
        };
        let first_client = TcpStream::connect(address).unwrap();
        let second_client = TcpStream::connect(address).unwrap();
        let first = first.join().unwrap();
        let second = second.join().unwrap();
        assert!(!first.failed, "{}", first.message);
        assert!(!second.failed, "{}", second.message);
        drop((first_client, second_client));
    }

    #[test]
    fn oversized_udp_datagram_reports_truncation() {
        let receiver = udp_bind("127.0.0.1:0");
        let sender = udp_bind("127.0.0.1:0");
        assert!(!receiver.failed && !sender.failed);
        assert!(!udp_configure(sender.capability.as_ref().unwrap(), true, 32).failed);
        assert!(udp_configure(sender.capability.as_ref().unwrap(), false, -1).failed);
        let cancellation = cancellation_token();
        let sent = udp_send_to(
            sender.capability.as_ref().unwrap(),
            b"oversized",
            &receiver.text,
            1_000_000_000,
            Some(&cancellation),
        );
        assert_eq!(sent.number, 9);
        let datagram = udp_receive_from(
            receiver.capability.as_ref().unwrap(),
            4,
            1_000_000_000,
            Some(&cancellation),
        );
        assert!(datagram.truncated);
        assert_eq!(datagram.data, b"over");
    }

    #[test]
    fn blocking_operations_observe_expired_deadlines_and_cancellation() {
        let listener = tcp_bind("127.0.0.1:0", 128, true, false, 64);
        assert!(!listener.failed);
        let cancellation = cancellation_token();
        let expired = tcp_accept(listener.capability.as_ref().unwrap(), 0, None);
        assert!(expired.failed);
        assert!(expired.deadline_exceeded);

        assert!(!cancel(&cancellation).failed);
        let cancelled = tcp_accept(
            listener.capability.as_ref().unwrap(),
            1_000_000_000,
            Some(&cancellation),
        );
        assert!(cancelled.failed);
        assert!(!cancelled.deadline_exceeded);
        assert_eq!(cancelled.message, "operation cancelled");
    }

    #[test]
    fn host_names_are_validated_before_resolution() {
        assert!(!parse_host_name("localhost").failed);
        assert!(!parse_host_name("api.example.test").failed);
        assert!(parse_host_name("").failed);
        assert!(parse_host_name("-invalid.example").failed);
        assert!(parse_host_name("invalid-.example").failed);
        assert!(parse_host_name("contains space.example").failed);
    }

    #[test]
    fn connect_host_races_resolved_loopback_candidates() {
        let listener = tcp_bind("127.0.0.1:0", 128, true, false, 64);
        assert!(!listener.failed);
        let port = listener.text.parse::<SocketAddr>().unwrap().port();
        let cancellation = cancellation_token();
        let client = tcp_connect_host(
            "localhost",
            i128::from(port),
            false,
            64,
            1_000_000_000,
            Some(&cancellation),
        );
        assert!(!client.failed, "{}", client.message);
        let server = tcp_accept(
            listener.capability.as_ref().unwrap(),
            1_000_000_000,
            Some(&cancellation),
        );
        assert!(!server.failed, "{}", server.message);
        assert!(!tcp_configure(client.capability.as_ref().unwrap(), true, 32).failed);
        assert!(tcp_configure(client.capability.as_ref().unwrap(), false, -2).failed);
    }

    #[test]
    fn asynchronous_connections_support_blocking_and_async_io_on_the_same_transport() {
        let listener = result_capability(tcp_bind("127.0.0.1:0", 128, false, false, -1));
        let address = listener.0.as_ref();
        let address = match address {
            CapabilityInner::Listener(state) => state
                .read()
                .as_ref()
                .unwrap()
                .local_addr()
                .unwrap()
                .to_string(),
            _ => unreachable!(),
        };
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();
        let (client, server) = runtime.block_on(async {
            let (client, server) = tokio::join!(
                tcp_connect_async(&address, false, -1, 1_000_000_000, None),
                tcp_accept_async(&listener, 1_000_000_000, None),
            );
            (result_capability(client), result_capability(server))
        });
        assert!(!tcp_configure(&client, true, -1).failed);
        assert_eq!(tcp_write(&client, b"sync", 1_000_000_000, None).number, 4);
        assert_eq!(tcp_read(&server, 4, 1_000_000_000, None).data, b"sync");
        runtime.block_on(async {
            let (written, read) = tokio::join!(
                tcp_write_async(&server, b"async", 1_000_000_000, None),
                tcp_read_async(&client, 5, 1_000_000_000, None),
            );
            assert_eq!(written.number, 5);
            assert_eq!(read.data, b"async");
        });
    }

    #[test]
    fn tcp_read_distinguishes_zero_length_from_peer_eof_and_closed_state() {
        let listener = tcp_bind("127.0.0.1:0", 16, false, false, 64);
        assert!(!listener.failed);
        let address = listener.text.clone();
        let listener = listener.capability.unwrap();
        let client = result_capability(tcp_connect(&address, false, 64, 1_000_000_000, None));
        let accepted = result_capability(tcp_accept(&listener, 1_000_000_000, None));
        let CapabilityInner::Tcp(accepted_state) = accepted.0.as_ref() else {
            unreachable!();
        };
        assert!(!tcp_read(&accepted, 0, 1_000_000_000, None).failed);
        assert!(!tcp_shutdown(&client, "write").failed);
        let eof = tcp_read(&accepted, 1, 1_000_000_000, None);
        assert!(!eof.failed);
        assert!(eof.flag);
        accepted_state.close();
        assert!(accepted_state.readiness().is_err());
    }

    #[test]
    fn blocking_tcp_read_does_not_hold_up_concurrent_write() {
        let listener = tcp_bind("127.0.0.1:0", 16, true, true, 32);
        let address = listener.text.clone();
        let listener = listener.capability.unwrap();
        let client = result_capability(tcp_connect(&address, false, 64, 1_000_000_000, None));
        let accepted = result_capability(tcp_accept(&listener, 1_000_000_000, None));
        let accepted_for_read = accepted.clone();
        let reading =
            std::thread::spawn(move || tcp_read(&accepted_for_read, 1, 2_000_000_000, None));
        let written = tcp_write(&client, b"x", 1_000_000_000, None);
        assert_eq!(written.number, 1);
        let received = reading.join().unwrap();
        assert_eq!(received.data, b"x");
        let CapabilityInner::Tcp(state) = accepted.0.as_ref() else {
            unreachable!()
        };
        let socket = state.read();
        let socket = socket.as_ref().unwrap();
        assert!(socket.nodelay().unwrap());
        assert_eq!(socket.ttl().unwrap(), 32);
    }

    #[test]
    fn explicitly_selected_decompression_preserves_nested_payload() {
        let inner = compress("gzip", b"inner document", 6, true);
        assert!(!inner.failed);
        let outer = compress("gzip", &inner.data, 6, true);
        assert!(!outer.failed);
        let unpacked = decompress("gzip", &outer.data, 4096, 100, 8192);
        assert!(!unpacked.failed, "{}", unpacked.message);
        assert_eq!(unpacked.data, inner.data);
    }

    #[test]
    fn dns_candidates_are_sorted_and_deduplicated() {
        let candidates = ordered_socket_candidates(
            [
                "2001:db8::2".parse().unwrap(),
                "127.0.0.2".parse().unwrap(),
                "127.0.0.1".parse().unwrap(),
                "127.0.0.2".parse().unwrap(),
            ],
            443,
        );
        assert_eq!(
            candidates,
            ["127.0.0.1:443", "127.0.0.2:443", "[2001:db8::2]:443"]
        );
    }

    #[test]
    fn dns_lookup_projects_ordered_candidates_and_ttl() {
        let result = dns_lookup("localhost", 443, 1_000_000_000, None);
        assert!(!result.failed, "{}", result.message);
        assert!(result.flag);
        assert!(result.number >= 0);
        assert_ne!(result.entries, [] as [String; 0]);
        assert!(result.entries.windows(2).all(|pair| pair[0] < pair[1]));
    }

    #[test]
    fn legacy_digest_vectors_remain_available_by_explicit_algorithm_name() {
        assert_eq!(
            hex_encode(&digest("sha-1", b"abc").data),
            "a9993e364706816aba3e25717850c26c9cd0d89d"
        );
        assert_eq!(
            hex_encode(&digest("md5", b"abc").data),
            "900150983cd24fb0d6963f7d28e17f72"
        );
        let key = secret_buffer(b"key".to_vec());
        assert_eq!(hmac("sha-1", &key, b"data").data.len(), 20);
        assert_eq!(hmac("md5", &key, b"data").data.len(), 16);
    }

    #[test]
    fn empty_secret_remains_a_valid_hmac_key_until_destroyed() {
        let key = secret_buffer(Vec::new());
        assert!(!hmac("sha-256", &key, b"data").failed);
        assert!(!destroy_secret(&key).failed);
        let destroyed = hmac("sha-256", &key, b"data");
        assert!(destroyed.failed);
        assert_eq!(destroyed.message, "secret buffer was destroyed");
    }

    #[test]
    fn host_names_apply_uts46_consistently() {
        let host = parse_host_name("bücher.example");
        assert!(!host.failed, "{}", host.message);
        assert_eq!(host.text, "xn--bcher-kva.example");
    }

    #[test]
    fn accept_reports_an_expired_deadline_structurally() {
        let listener = tcp_bind("127.0.0.1:0", 128, true, false, 64);
        assert!(!listener.failed);
        let result = tcp_accept(listener.capability.as_ref().unwrap(), 1, None);
        assert!(result.failed);
        assert!(result.deadline_exceeded);
    }

    #[test]
    fn accept_preserves_the_listener_nonblocking_descriptor_state() {
        let listener = tcp_bind("127.0.0.1:0", 128, true, false, 64);
        assert!(!listener.failed);
        let listener = listener.capability.unwrap();
        let would_block = || match listener.0.as_ref() {
            CapabilityInner::Listener(listener) => {
                listener
                    .read()
                    .as_ref()
                    .unwrap()
                    .accept()
                    .unwrap_err()
                    .kind()
                    == std::io::ErrorKind::WouldBlock
            }
            _ => false,
        };
        assert!(would_block());
        let accepted = tcp_accept(&listener, 1, None);
        assert!(accepted.deadline_exceeded);
        assert!(would_block());
    }

    #[test]
    fn tls_accepts_a_trusted_local_certificate_and_reports_tls_1_3() {
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(["localhost".to_owned()]).unwrap();
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                vec![cert.der().clone()],
                rustls::pki_types::PrivatePkcs8KeyDer::from(signing_key.serialize_der()).into(),
            )
            .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (socket, _) = listener.accept().unwrap();
            let connection = rustls::ServerConnection::new(Arc::new(config)).unwrap();
            let mut stream = rustls::StreamOwned::new(connection, socket);
            stream.write_all(b"x").unwrap();
            stream.flush().unwrap();
        });
        let mut roots = rustls::RootCertStore::empty();
        roots.add(cert.der().clone()).unwrap();
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let tcp = tcp_connect(&address.to_string(), false, 64, 1_000_000_000, None);
        let result = tls_client_with_config(
            tcp.capability.as_ref().unwrap(),
            "localhost",
            1_000_000_000,
            None,
            Arc::new(config),
        );
        assert!(!result.failed, "{}", result.message);
        assert_eq!(result.text, "TLS 1.3");
        let read = tls_read(result.capability.as_ref().unwrap(), 1, 1_000_000_000, None);
        assert_eq!(read.data, b"x");
        server.join().unwrap();
    }

    #[test]
    fn tls_accepts_a_trusted_local_certificate_and_reports_tls_1_2() {
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(["localhost".to_owned()]).unwrap();
        let config =
            rustls::ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
                .with_no_client_auth()
                .with_single_cert(
                    vec![cert.der().clone()],
                    rustls::pki_types::PrivatePkcs8KeyDer::from(signing_key.serialize_der()).into(),
                )
                .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (socket, _) = listener.accept().unwrap();
            let connection = rustls::ServerConnection::new(Arc::new(config)).unwrap();
            let mut stream = rustls::StreamOwned::new(connection, socket);
            stream.write_all(b"x").unwrap();
            stream.flush().unwrap();
        });
        let mut roots = rustls::RootCertStore::empty();
        roots.add(cert.der().clone()).unwrap();
        let config =
            rustls::ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
                .with_root_certificates(roots)
                .with_no_client_auth();
        let tcp = tcp_connect(&address.to_string(), false, 64, 1_000_000_000, None);
        let result = tls_client_with_config(
            tcp.capability.as_ref().unwrap(),
            "localhost",
            1_000_000_000,
            None,
            Arc::new(config),
        );
        assert!(!result.failed, "{}", result.message);
        assert_eq!(result.text, "TLS 1.2");
        let read = tls_read(result.capability.as_ref().unwrap(), 1, 1_000_000_000, None);
        assert_eq!(read.data, b"x");
        server.join().unwrap();
    }

    #[test]
    fn tls_shutdown_sends_close_notify() {
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(["localhost".to_owned()]).unwrap();
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                vec![cert.der().clone()],
                rustls::pki_types::PrivatePkcs8KeyDer::from(signing_key.serialize_der()).into(),
            )
            .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (socket, _) = listener.accept().unwrap();
            let connection = rustls::ServerConnection::new(Arc::new(config)).unwrap();
            let mut stream = rustls::StreamOwned::new(connection, socket);
            let mut byte = [0];
            stream.read_exact(&mut byte).unwrap();
            assert_eq!(byte, [b'x']);
            assert_eq!(stream.read(&mut byte).unwrap(), 0);
        });
        let mut roots = rustls::RootCertStore::empty();
        roots.add(cert.der().clone()).unwrap();
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let tcp = tcp_connect(&address.to_string(), false, 64, 1_000_000_000, None);
        let tls = tls_client_with_config(
            tcp.capability.as_ref().unwrap(),
            "localhost",
            1_000_000_000,
            None,
            Arc::new(config),
        );
        assert!(!tls.failed, "{}", tls.message);
        let tls = tls.capability.unwrap();
        assert_eq!(tls_write(&tls, b"x", 1_000_000_000, None).number, 1);
        let shutdown = tls_shutdown(&tls, 1_000_000_000, None);
        assert!(!shutdown.failed, "{}", shutdown.message);
        server.join().unwrap();
    }

    #[test]
    fn tls_rejects_a_locally_self_signed_certificate() {
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(["localhost".to_owned()]).unwrap();
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                vec![cert.der().clone()],
                rustls::pki_types::PrivatePkcs8KeyDer::from(signing_key.serialize_der()).into(),
            )
            .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (socket, _) = listener.accept().unwrap();
            let connection = rustls::ServerConnection::new(Arc::new(config)).unwrap();
            let mut stream = rustls::StreamOwned::new(connection, socket);
            let _ = stream.flush();
        });
        let client = TcpStream::connect(address).unwrap();
        let capability = Capability(Arc::new(CapabilityInner::Tcp(TcpState::new(client))));
        let result = tls_client(&capability, "localhost", 1_000_000_000, None);
        assert!(result.failed);
        assert!(result.message.starts_with("TLS handshake failed:"));
        server.join().unwrap();
    }

    #[test]
    fn readiness_native_sockets_progress_beyond_worker_and_blocking_capacity() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .max_blocking_threads(1)
            .build()
            .unwrap();
        runtime.block_on(async {
            let bound = tcp_bind("127.0.0.1:0", 128, true, false, 64);
            let listener = result_capability(bound);
            let address = match listener.0.as_ref() {
                CapabilityInner::Listener(listener) => listener
                    .read()
                    .as_ref()
                    .unwrap()
                    .local_addr()
                    .unwrap()
                    .to_string(),
                _ => panic!("expected listener"),
            };
            let mut accepts = tokio::task::JoinSet::new();
            let mut connects = tokio::task::JoinSet::new();
            for _ in 0..32 {
                let listener = listener.clone();
                accepts
                    .spawn(async move { tcp_accept_async(&listener, 5_000_000_000, None).await });
                let address = address.clone();
                connects.spawn(async move {
                    tcp_connect_async(&address, false, 64, 5_000_000_000, None).await
                });
            }
            while let Some(result) = connects.join_next().await {
                let result = result.unwrap();
                assert!(!result.failed, "{}", result.message);
            }
            while let Some(result) = accepts.join_next().await {
                let result = result.unwrap();
                assert!(!result.failed, "{}", result.message);
            }

            let pending = {
                let listener = listener.clone();
                let cancellation = cancellation_token();
                let cancellation_for_task = cancellation.clone();
                let task = tokio::spawn(async move {
                    tcp_accept_async(&listener, 5_000_000_000, Some(&cancellation_for_task)).await
                });
                tokio::task::yield_now().await;
                cancel(&cancellation);
                task
            };
            let cancelled = tokio::time::timeout(Duration::from_secs(1), pending)
                .await
                .expect("cancellation must wake a pending accept")
                .unwrap();
            assert!(cancelled.failed);
            assert_eq!(cancelled.message, "operation cancelled");

            let deadline = tcp_accept_async(&listener, 1, None).await;
            assert!(deadline.failed);
            assert!(deadline.deadline_exceeded);
        });
    }

    #[test]
    fn readiness_native_tls_handshake_and_io_share_the_selected_runtime() {
        let rcgen::CertifiedKey { cert, signing_key } =
            rcgen::generate_simple_self_signed(["localhost".to_owned()]).unwrap();
        let server_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(
                vec![cert.der().clone()],
                rustls::pki_types::PrivatePkcs8KeyDer::from(signing_key.serialize_der()).into(),
            )
            .unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (socket, _) = listener.accept().unwrap();
            let connection = rustls::ServerConnection::new(Arc::new(server_config)).unwrap();
            let mut stream = rustls::StreamOwned::new(connection, socket);
            let mut byte = [0];
            stream.read_exact(&mut byte).unwrap();
            assert_eq!(byte, [b'x']);
            stream.write_all(b"y").unwrap();
            stream.flush().unwrap();
            assert_eq!(stream.read(&mut byte).unwrap(), 0);
        });
        let mut roots = rustls::RootCertStore::empty();
        roots.add(cert.der().clone()).unwrap();
        let client_config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .max_blocking_threads(1)
            .build()
            .unwrap();
        runtime.block_on(async {
            let tcp = tcp_connect_async(&address.to_string(), false, 64, 1_000_000_000, None).await;
            let tls = tls_client_async_with_config(
                tcp.capability.as_ref().unwrap(),
                "localhost",
                1_000_000_000,
                None,
                Arc::new(client_config),
            )
            .await;
            assert!(!tls.failed, "{}", tls.message);
            assert_eq!(tls.text, "TLS 1.3");
            let tls = tls.capability.unwrap();
            let pending_cancellation = cancellation_token();
            let mut pending =
                std::pin::pin!(tls_read_async(&tls, 1, -1, Some(&pending_cancellation)));
            std::future::poll_fn(|context| {
                assert!(std::future::Future::poll(pending.as_mut(), context).is_pending());
                std::task::Poll::Ready(())
            })
            .await;
            assert!(close(&tls).failed);
            assert!(!cancel(&pending_cancellation).failed);
            let canceled = pending.await;
            assert!(canceled.failed && !canceled.deadline_exceeded);
            let read_cancellation = cancellation_token();
            let write_cancellation = cancellation_token();
            let (read, write) = tokio::join!(
                tls_read_async(&tls, 1, 1_000_000_000, Some(&read_cancellation)),
                tls_write_async(&tls, b"x", 1_000_000_000, Some(&write_cancellation)),
            );
            assert_eq!(write.number, 1);
            assert_eq!(read.data, b"y");
            let shutdown = tls_shutdown_async(&tls, 1_000_000_000, None).await;
            assert!(!shutdown.failed, "{}", shutdown.message);
        });
        server.join().unwrap();
    }
    #[tokio::test(flavor = "current_thread")]
    async fn controlled_clock_advances_monotonic_time_and_wakes_sleep_exactly() {
        struct Reset;
        impl Drop for Reset {
            fn drop(&mut self) {
                CONTROLLED_MONOTONIC_NANOS.set(None);
                CONTROLLED_TIME_NOTIFY.notify_waiters();
            }
        }
        CONTROLLED_MONOTONIC_NANOS.set(Some(1_000));
        let _reset = Reset;
        assert_eq!(monotonic_nanos(), 1_000);

        let sleeper = tokio::spawn(sleep(Duration::from_nanos(10)));
        tokio::task::yield_now().await;
        assert!(!sleeper.is_finished());
        CONTROLLED_MONOTONIC_NANOS.set(Some(1_009));
        CONTROLLED_TIME_NOTIFY.notify_waiters();
        tokio::task::yield_now().await;
        assert!(!sleeper.is_finished());
        CONTROLLED_MONOTONIC_NANOS.set(Some(1_010));
        CONTROLLED_TIME_NOTIFY.notify_waiters();
        sleeper.await.expect("controlled sleep completes");
        assert_eq!(monotonic_nanos(), 1_010);
    }
}
