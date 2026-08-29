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
use std::io::{Read, Write as _};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{
    Arc, LazyLock, Mutex, RwLock,
    atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering},
    mpsc::{Receiver, SyncSender, TryRecvError, sync_channel},
};
use std::time::Duration;
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
struct IntChannel {
    sender: SyncSender<i128>,
    receiver: Mutex<Receiver<i128>>,
}
struct ThreadLocalInt {
    id: u64,
    initial: i128,
}

static NEXT_THREAD_LOCAL_ID: AtomicU64 = AtomicU64::new(1);
thread_local! {
    static THREAD_LOCAL_INTS: RefCell<HashMap<u64, i128>> = RefCell::new(HashMap::new());
}

#[derive(Clone)]
pub struct Capability(Arc<CapabilityInner>);

enum CapabilityInner {
    Secure,
    Invalid(String),
    Pseudo(Mutex<rand_chacha::ChaCha20Rng>),
    Secret(Mutex<SecretState>),
    Cancellation(AtomicBool),
    IntChannel(IntChannel),
    IntMutex(Mutex<i128>),
    IntRwLock(RwLock<i128>),
    AtomicI64(AtomicI64),
    ThreadLocalInt(ThreadLocalInt),
    Listener(Mutex<Option<TcpListener>>),
    Tcp(Mutex<Option<TcpStream>>),
    Udp(Mutex<Option<UdpSocket>>),
    Tls(Mutex<Option<rustls::StreamOwned<rustls::ClientConnection, TcpStream>>>),
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

pub fn int_channel(capacity: i128) -> Capability {
    let Ok(capacity) = usize::try_from(capacity) else {
        return Capability(Arc::new(CapabilityInner::Invalid(
            "channel capacity must be a non-negative platform-sized integer".to_owned(),
        )));
    };
    let (sender, receiver) = sync_channel(capacity);
    Capability(Arc::new(CapabilityInner::IntChannel(IntChannel {
        sender,
        receiver: Mutex::new(receiver),
    })))
}
pub fn int_channel_send(channel: &Capability, value: i128) -> ResultValue {
    let CapabilityInner::IntChannel(channel) = channel.0.as_ref() else {
        return ResultValue::error("capability is not an int channel");
    };
    channel.sender.send(value).map_or_else(
        |_| ResultValue::error("channel receiver is closed"),
        |()| ResultValue::default(),
    )
}
pub fn int_channel_receive(channel: &Capability) -> ResultValue {
    let CapabilityInner::IntChannel(channel) = channel.0.as_ref() else {
        return ResultValue::error("capability is not an int channel");
    };
    let Ok(receiver) = channel.receiver.lock() else {
        return lock_error("channel receiver");
    };
    receiver.recv().map_or_else(
        |_| ResultValue::error("channel sender is closed"),
        |number| ResultValue {
            number,
            flag: true,
            ..ResultValue::default()
        },
    )
}
pub fn int_channel_try_receive(channel: &Capability) -> ResultValue {
    let CapabilityInner::IntChannel(channel) = channel.0.as_ref() else {
        return ResultValue::error("capability is not an int channel");
    };
    let Ok(receiver) = channel.receiver.lock() else {
        return lock_error("channel receiver");
    };
    match receiver.try_recv() {
        Ok(number) => ResultValue {
            number,
            flag: true,
            ..ResultValue::default()
        },
        Err(TryRecvError::Empty) => ResultValue::default(),
        Err(TryRecvError::Disconnected) => ResultValue::error("channel sender is closed"),
    }
}
pub fn int_mutex(initial: i128) -> Capability {
    Capability(Arc::new(CapabilityInner::IntMutex(Mutex::new(initial))))
}
pub fn int_mutex_load(value: &Capability) -> ResultValue {
    let CapabilityInner::IntMutex(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not an int mutex");
    };
    value.lock().map_or_else(
        |_| lock_error("mutex"),
        |value| ResultValue {
            number: *value,
            flag: true,
            ..ResultValue::default()
        },
    )
}
pub fn int_mutex_store(value: &Capability, replacement: i128) -> ResultValue {
    let CapabilityInner::IntMutex(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not an int mutex");
    };
    let Ok(mut value) = value.lock() else {
        return lock_error("mutex");
    };
    *value = replacement;
    ResultValue::default()
}
pub fn int_mutex_add(value: &Capability, amount: i128) -> ResultValue {
    let CapabilityInner::IntMutex(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not an int mutex");
    };
    let Ok(mut value) = value.lock() else {
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
pub fn int_rw_lock(initial: i128) -> Capability {
    Capability(Arc::new(CapabilityInner::IntRwLock(RwLock::new(initial))))
}
pub fn int_rw_lock_read(value: &Capability) -> ResultValue {
    let CapabilityInner::IntRwLock(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not an int read/write lock");
    };
    value.read().map_or_else(
        |_| lock_error("read/write lock"),
        |value| ResultValue {
            number: *value,
            flag: true,
            ..ResultValue::default()
        },
    )
}
pub fn int_rw_lock_write(value: &Capability, replacement: i128) -> ResultValue {
    let CapabilityInner::IntRwLock(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not an int read/write lock");
    };
    let Ok(mut value) = value.write() else {
        return lock_error("read/write lock");
    };
    *value = replacement;
    ResultValue::default()
}
fn atomic_ordering(name: &str, load: bool) -> Result<Ordering, ResultValue> {
    match name {
        "relaxed" => Ok(Ordering::Relaxed),
        "acquire" if load => Ok(Ordering::Acquire),
        "release" if !load => Ok(Ordering::Release),
        "acquire-release" if !load => Ok(Ordering::AcqRel),
        "sequentially-consistent" => Ok(Ordering::SeqCst),
        _ => Err(ResultValue::error(format!(
            "invalid {} memory ordering `{name}`",
            if load { "load" } else { "store/update" }
        ))),
    }
}
pub fn atomic_int64(initial: i128) -> Capability {
    i64::try_from(initial).map_or_else(
        |_| {
            Capability(Arc::new(CapabilityInner::Invalid(
                "atomic int64 initial value is out of range".to_owned(),
            )))
        },
        |initial| {
            Capability(Arc::new(CapabilityInner::AtomicI64(AtomicI64::new(
                initial,
            ))))
        },
    )
}
pub fn atomic_int64_load(value: &Capability, ordering: &str) -> ResultValue {
    let CapabilityInner::AtomicI64(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not an atomic int64");
    };
    let ordering = match atomic_ordering(ordering, true) {
        Ok(ordering) => ordering,
        Err(error) => return error,
    };
    ResultValue {
        number: i128::from(value.load(ordering)),
        flag: true,
        ..ResultValue::default()
    }
}
pub fn atomic_int64_store(value: &Capability, replacement: i128, ordering: &str) -> ResultValue {
    let CapabilityInner::AtomicI64(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not an atomic int64");
    };
    let Ok(replacement) = i64::try_from(replacement) else {
        return ResultValue::error("atomic int64 value is out of range");
    };
    let ordering = match atomic_ordering(ordering, false) {
        Ok(ordering) => ordering,
        Err(error) => return error,
    };
    value.store(replacement, ordering);
    ResultValue::default()
}
pub fn atomic_int64_add(value: &Capability, amount: i128, ordering: &str) -> ResultValue {
    let CapabilityInner::AtomicI64(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not an atomic int64");
    };
    let Ok(amount) = i64::try_from(amount) else {
        return ResultValue::error("atomic int64 amount is out of range");
    };
    let ordering = match atomic_ordering(ordering, false) {
        Ok(ordering) => ordering,
        Err(error) => return error,
    };
    let failure_ordering = match ordering {
        Ordering::Acquire | Ordering::AcqRel => Ordering::Acquire,
        Ordering::SeqCst => Ordering::SeqCst,
        _ => Ordering::Relaxed,
    };
    let mut previous = value.load(failure_ordering);
    let next = loop {
        let Some(next) = previous.checked_add(amount) else {
            return ResultValue::error("atomic int64 addition overflowed");
        };
        match value.compare_exchange_weak(previous, next, ordering, failure_ordering) {
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
pub fn thread_local_int(initial: i128) -> Capability {
    Capability(Arc::new(CapabilityInner::ThreadLocalInt(ThreadLocalInt {
        id: NEXT_THREAD_LOCAL_ID.fetch_add(1, Ordering::Relaxed),
        initial,
    })))
}
pub fn thread_local_int_get(value: &Capability) -> ResultValue {
    let CapabilityInner::ThreadLocalInt(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not a thread-local int");
    };
    let number = THREAD_LOCAL_INTS
        .with(|values| *values.borrow_mut().entry(value.id).or_insert(value.initial));
    ResultValue {
        number,
        flag: true,
        ..ResultValue::default()
    }
}
pub fn thread_local_int_set(value: &Capability, replacement: i128) -> ResultValue {
    let CapabilityInner::ThreadLocalInt(value) = value.0.as_ref() else {
        return ResultValue::error("capability is not a thread-local int");
    };
    THREAD_LOCAL_INTS.with(|values| {
        values.borrow_mut().insert(value.id, replacement);
    });
    ResultValue::default()
}

pub fn cancellation_token() -> Capability {
    Capability(Arc::new(CapabilityInner::Cancellation(AtomicBool::new(
        false,
    ))))
}

pub fn cancel(token: &Capability) -> ResultValue {
    let CapabilityInner::Cancellation(cancelled) = token.0.as_ref() else {
        return ResultValue::error("capability is not a cancellation token");
    };
    cancelled.store(true, Ordering::Release);
    ResultValue::default()
}

fn is_cancelled(token: &Capability) -> Result<bool, ResultValue> {
    let CapabilityInner::Cancellation(cancelled) = token.0.as_ref() else {
        return Err(ResultValue::error("capability is not a cancellation token"));
    };
    Ok(cancelled.load(Ordering::Acquire))
}

fn cancellation_error(token: &Capability) -> Option<ResultValue> {
    match is_cancelled(token) {
        Ok(true) => Some(ResultValue::error("operation cancelled")),
        Ok(false) => None,
        Err(error) => Some(error),
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
    if text.len() % 2 != 0
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
            let window_log = usize::BITS - output_limit.max(1).saturating_sub(1).leading_zeros();
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
fn timeout(milliseconds: i128) -> Result<Duration, ResultValue> {
    let value = u64::try_from(milliseconds)
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| ResultValue::error("deadline must be positive milliseconds"))?;
    Ok(Duration::from_millis(value))
}

fn network_runtime() -> Result<&'static tokio::runtime::Runtime, ResultValue> {
    static RUNTIME: LazyLock<Result<tokio::runtime::Runtime, String>> = LazyLock::new(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|error| format!("cannot start network runtime: {error}"))
    });
    RUNTIME
        .as_ref()
        .map_err(|error| ResultValue::error(error.clone()))
}

fn dns_resolver() -> Result<&'static hickory_resolver::TokioResolver, ResultValue> {
    static RESOLVER: LazyLock<Result<hickory_resolver::TokioResolver, String>> =
        LazyLock::new(|| {
            hickory_resolver::TokioResolver::builder_tokio()
                .map(hickory_resolver::ResolverBuilder::build)
                .map_err(|error| format!("cannot configure DNS resolver: {error}"))
        });
    RESOLVER
        .as_ref()
        .map_err(|error| ResultValue::error(error.clone()))
}

fn io_error(operation: &str, error: &std::io::Error) -> ResultValue {
    if matches!(
        error.kind(),
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
    ) {
        ResultValue {
            failed: true,
            deadline_exceeded: true,
            message: format!("{operation} deadline exceeded"),
            ..ResultValue::default()
        }
    } else {
        ResultValue::error(format!("{operation} failed: {error}"))
    }
}

pub fn tcp_bind(address: &str) -> ResultValue {
    match TcpListener::bind(address) {
        Ok(listener) => {
            let text = listener
                .local_addr()
                .map_or_else(|_| String::new(), |value| value.to_string());
            if let Err(error) = listener.set_nonblocking(true) {
                return io_error("TCP listener configuration", &error);
            }
            ResultValue {
                text,
                capability: Some(Capability(Arc::new(CapabilityInner::Listener(Mutex::new(
                    Some(listener),
                ))))),
                ..ResultValue::default()
            }
        }
        Err(error) => ResultValue::error(format!("TCP bind failed: {error}")),
    }
}
pub fn tcp_connect(address: &str, deadline_ms: i128, cancellation: &Capability) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let Ok(address) = address.parse::<SocketAddr>() else {
        return ResultValue::error("invalid socket address");
    };
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    match TcpStream::connect_timeout(&address, duration) {
        Ok(stream) => {
            if let Err(error) = stream.set_read_timeout(Some(duration)) {
                return io_error("TCP read timeout configuration", &error);
            }
            if let Err(error) = stream.set_write_timeout(Some(duration)) {
                return io_error("TCP write timeout configuration", &error);
            }
            ResultValue {
                capability: Some(Capability(Arc::new(CapabilityInner::Tcp(Mutex::new(
                    Some(stream),
                ))))),
                ..ResultValue::default()
            }
        }
        Err(error) => io_error("TCP connect", &error),
    }
}

pub fn tcp_connect_host(
    host: &str,
    port: i128,
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let Ok(port) = u16::try_from(port) else {
        return ResultValue::error("port must be in 0..=65535");
    };
    let started = std::time::Instant::now();
    let resolved = dns_lookup(host, i128::from(port), deadline_ms, cancellation);
    if resolved.failed {
        return resolved;
    }
    let candidates = resolved
        .entries
        .iter()
        .filter_map(|entry| entry.parse::<SocketAddr>().ok())
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return ResultValue::error("DNS lookup returned no usable addresses");
    }
    let remaining = duration.saturating_sub(started.elapsed());
    if remaining.is_zero() {
        return ResultValue {
            failed: true,
            deadline_exceeded: true,
            message: "TCP connect deadline exceeded".to_owned(),
            ..ResultValue::default()
        };
    }
    let runtime = match network_runtime() {
        Ok(value) => value,
        Err(error) => return error,
    };
    let outcome = runtime.block_on(async {
        let mut tasks = tokio::task::JoinSet::new();
        for (index, address) in candidates.into_iter().enumerate() {
            tasks.spawn(async move {
                tokio::time::sleep(Duration::from_millis(
                    (index as u64).saturating_mul(250),
                ))
                .await;
                tokio::net::TcpStream::connect(address)
                    .await
                    .ok()
                    .map(|stream| (stream, address))
            });
        }
        let race = async {
            loop {
                tokio::select! {
                    joined = tasks.join_next() => match joined {
                        Some(Ok(Some(value))) => break Ok(value),
                        Some(Ok(None) | Err(_)) => {}
                        None => break Err(ResultValue::error("all TCP connection candidates failed")),
                    },
                    () = tokio::time::sleep(Duration::from_millis(10)) => {
                        if is_cancelled(cancellation).unwrap_or(true) {
                            break Err(ResultValue::error("operation cancelled"));
                        }
                    }
                }
            }
        };
        let result = match tokio::time::timeout(remaining, race).await {
            Ok(value) => value,
            Err(_) => Err(ResultValue {
                failed: true,
                deadline_exceeded: true,
                message: "TCP connect deadline exceeded".to_owned(),
                ..ResultValue::default()
            }),
        };
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
        result
    });
    let (stream, peer) = match outcome {
        Ok(value) => value,
        Err(error) => return error,
    };
    let stream = match stream.into_std() {
        Ok(value) => value,
        Err(error) => return io_error("TCP connect", &error),
    };
    if let Err(error) = stream.set_nonblocking(false) {
        return io_error("TCP connect configuration", &error);
    }
    ResultValue {
        text: peer.to_string(),
        capability: Some(Capability(Arc::new(CapabilityInner::Tcp(Mutex::new(
            Some(stream),
        ))))),
        ..ResultValue::default()
    }
}
pub fn tcp_accept(
    listener: &Capability,
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let CapabilityInner::Listener(listener) = listener.0.as_ref() else {
        return ResultValue::error("capability is not a TCP listener");
    };
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let listener = {
        let guard = listener.lock().expect("listener lock poisoned");
        let Some(listener) = guard.as_ref() else {
            return ResultValue::error("listener is closed");
        };
        match listener.try_clone() {
            Ok(value) => value,
            Err(error) => return io_error("TCP listener clone", &error),
        }
    };
    let runtime = match network_runtime() {
        Ok(value) => value,
        Err(error) => return error,
    };
    let enter = runtime.enter();
    let listener = match tokio::net::TcpListener::from_std(listener) {
        Ok(value) => value,
        Err(error) => return io_error("TCP accept configuration", &error),
    };
    drop(enter);
    let accepted = runtime.block_on(async {
        let cancellation_wait = async {
            loop {
                if is_cancelled(cancellation).unwrap_or(true) {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        };
        tokio::select! {
            result = tokio::time::timeout(duration, listener.accept()) => Some(result),
            () = cancellation_wait => None,
        }
    });
    let (stream, peer) = match accepted {
        None => return ResultValue::error("operation cancelled"),
        Some(Err(_)) => {
            return ResultValue {
                failed: true,
                deadline_exceeded: true,
                message: "TCP accept deadline exceeded".to_owned(),
                ..ResultValue::default()
            };
        }
        Some(Ok(Err(error))) => return io_error("TCP accept", &error),
        Some(Ok(Ok(value))) => value,
    };
    let stream = match stream.into_std() {
        Ok(value) => value,
        Err(error) => return io_error("TCP accept", &error),
    };
    if let Err(error) = stream.set_nonblocking(false) {
        return io_error("TCP accept configuration", &error);
    }
    ResultValue {
        text: peer.to_string(),
        capability: Some(Capability(Arc::new(CapabilityInner::Tcp(Mutex::new(
            Some(stream),
        ))))),
        ..ResultValue::default()
    }
}
pub fn tcp_read(
    stream: &Capability,
    limit: i128,
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let size = match count(limit, "read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    let mut guard = stream.lock().expect("stream lock poisoned");
    let Some(stream) = guard.as_mut() else {
        return ResultValue::error("stream is closed");
    };
    if let Err(error) = stream.set_read_timeout(Some(duration)) {
        return io_error("TCP read timeout configuration", &error);
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
        Err(error) => io_error("TCP read", &error),
    }
}
pub fn tcp_write(
    stream: &Capability,
    data: &[u8],
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    let mut guard = stream.lock().expect("stream lock poisoned");
    let Some(stream) = guard.as_mut() else {
        return ResultValue::error("stream is closed");
    };
    if let Err(error) = stream.set_write_timeout(Some(duration)) {
        return io_error("TCP write timeout configuration", &error);
    }
    match stream.write(data) {
        Ok(count) => ResultValue {
            number: count as i128,
            ..ResultValue::default()
        },
        Err(error) => io_error("TCP write", &error),
    }
}
pub fn tcp_shutdown(stream: &Capability, direction: &str) -> ResultValue {
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    let guard = stream.lock().expect("stream lock poisoned");
    let Some(stream) = guard.as_ref() else {
        return ResultValue::error("stream is closed");
    };
    let direction = match direction {
        "read" => Shutdown::Read,
        "write" => Shutdown::Write,
        "both" => Shutdown::Both,
        _ => return ResultValue::error("shutdown direction must be read, write, or both"),
    };
    match stream.shutdown(direction) {
        Ok(()) => ResultValue::default(),
        Err(error) => ResultValue::error(error.to_string()),
    }
}

pub fn tcp_configure(stream: &Capability, no_delay: bool, ttl: i128) -> ResultValue {
    let Ok(ttl) = u32::try_from(ttl) else {
        return ResultValue::error("TCP TTL must be in 0..=4294967295");
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    let guard = stream.lock().expect("stream lock poisoned");
    let Some(stream) = guard.as_ref() else {
        return ResultValue::error("stream is closed");
    };
    if let Err(error) = stream.set_nodelay(no_delay) {
        return ResultValue::error(format!("cannot set TCP no-delay option: {error}"));
    }
    match stream.set_ttl(ttl) {
        Ok(()) => ResultValue::default(),
        Err(error) => ResultValue::error(format!("cannot set TCP TTL: {error}")),
    }
}

pub fn udp_bind(address: &str) -> ResultValue {
    match UdpSocket::bind(address) {
        Ok(socket) => {
            let text = socket
                .local_addr()
                .map_or_else(|_| String::new(), |value| value.to_string());
            ResultValue {
                text,
                capability: Some(Capability(Arc::new(CapabilityInner::Udp(Mutex::new(
                    Some(socket),
                ))))),
                ..ResultValue::default()
            }
        }
        Err(error) => ResultValue::error(format!("UDP bind failed: {error}")),
    }
}

pub fn udp_configure(socket: &Capability, broadcast: bool, ttl: i128) -> ResultValue {
    let Ok(ttl) = u32::try_from(ttl) else {
        return ResultValue::error("UDP TTL must be in 0..=4294967295");
    };
    let CapabilityInner::Udp(socket) = socket.0.as_ref() else {
        return ResultValue::error("capability is not a UDP socket");
    };
    let guard = socket.lock().expect("socket lock poisoned");
    let Some(socket) = guard.as_ref() else {
        return ResultValue::error("socket is closed");
    };
    if let Err(error) = socket.set_broadcast(broadcast) {
        return ResultValue::error(format!("cannot set UDP broadcast option: {error}"));
    }
    match socket.set_ttl(ttl) {
        Ok(()) => ResultValue::default(),
        Err(error) => ResultValue::error(format!("cannot set UDP TTL: {error}")),
    }
}
pub fn udp_send_to(
    socket: &Capability,
    data: &[u8],
    address: &str,
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Udp(socket) = socket.0.as_ref() else {
        return ResultValue::error("capability is not a UDP socket");
    };
    let guard = socket.lock().expect("socket lock poisoned");
    let Some(socket) = guard.as_ref() else {
        return ResultValue::error("socket is closed");
    };
    if let Err(error) = socket.set_write_timeout(Some(duration)) {
        return io_error("UDP write timeout configuration", &error);
    }
    match socket.send_to(data, address) {
        Ok(count) => ResultValue {
            number: count as i128,
            ..ResultValue::default()
        },
        Err(error) => io_error("UDP send", &error),
    }
}
pub fn udp_receive_from(
    socket: &Capability,
    limit: i128,
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let size = match count(limit, "datagram limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Udp(socket) = socket.0.as_ref() else {
        return ResultValue::error("capability is not a UDP socket");
    };
    let guard = socket.lock().expect("socket lock poisoned");
    let Some(socket) = guard.as_ref() else {
        return ResultValue::error("socket is closed");
    };
    if let Err(error) = socket.set_read_timeout(Some(duration)) {
        return io_error("UDP read timeout configuration", &error);
    }
    let mut data = vec![0; size.saturating_add(1)];
    match socket.recv_from(&mut data) {
        Ok((count, peer)) => {
            let truncated = count > size;
            data.truncate(count.min(size));
            ResultValue {
                data,
                text: peer.to_string(),
                number: count.min(size) as i128,
                truncated,
                ..ResultValue::default()
            }
        }
        Err(error) => io_error("UDP receive", &error),
    }
}

fn ordered_socket_candidates(
    addresses: impl IntoIterator<Item = IpAddr>,
    port: u16,
) -> Vec<String> {
    let mut addresses = addresses.into_iter().collect::<Vec<_>>();
    addresses.sort_unstable();
    addresses.dedup();
    addresses
        .into_iter()
        .map(|ip| SocketAddr::new(ip, port).to_string())
        .collect()
}

pub fn dns_lookup(
    host: &str,
    port: i128,
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let Ok(port) = u16::try_from(port) else {
        return ResultValue::error("port must be in 0..=65535");
    };
    let runtime = match network_runtime() {
        Ok(value) => value,
        Err(error) => return error,
    };
    let resolver = match dns_resolver() {
        Ok(value) => value,
        Err(error) => return error,
    };
    let lookup = runtime.block_on(async {
        let cancellation_wait = async {
            loop {
                if is_cancelled(cancellation).unwrap_or(true) {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        };
        tokio::select! {
            result = tokio::time::timeout(duration, resolver.lookup_ip(host)) => Some(result),
            () = cancellation_wait => None,
        }
    });
    match lookup {
        None => ResultValue::error("operation cancelled"),
        Some(Ok(Ok(values))) => {
            let ttl = values
                .valid_until()
                .saturating_duration_since(std::time::Instant::now())
                .as_secs();
            ResultValue {
                entries: ordered_socket_candidates(values.iter(), port),
                number: i128::from(ttl),
                flag: true,
                ..ResultValue::default()
            }
        }
        Some(Ok(Err(error))) => ResultValue::error(format!("DNS lookup failed: {error}")),
        Some(Err(_)) => ResultValue {
            failed: true,
            deadline_exceeded: true,
            message: "DNS lookup deadline exceeded".to_owned(),
            ..ResultValue::default()
        },
    }
}

pub fn tls_client(
    stream: &Capability,
    server_name: &str,
    deadline_ms: i128,
    cancellation: &Capability,
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
        deadline_ms,
        cancellation,
        Arc::new(config),
    )
}

fn tls_client_with_config(
    stream: &Capability,
    server_name: &str,
    deadline_ms: i128,
    cancellation: &Capability,
    config: Arc<rustls::ClientConfig>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_ms) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("TLS requires a TCP stream");
    };
    let mut guard = stream.lock().expect("stream lock poisoned");
    let Some(tcp) = guard.take() else {
        return ResultValue::error("stream is closed");
    };
    if let Err(error) = tcp.set_read_timeout(Some(duration)) {
        return io_error("TLS read timeout configuration", &error);
    }
    if let Err(error) = tcp.set_write_timeout(Some(duration)) {
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
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let size = match count(limit, "TLS read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let duration = match timeout(deadline_ms) {
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
    if let Err(error) = stream.sock.set_read_timeout(Some(duration)) {
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
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_ms) {
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
    if let Err(error) = stream.sock.set_write_timeout(Some(duration)) {
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
    deadline_ms: i128,
    cancellation: &Capability,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_ms) {
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
    if let Err(error) = stream.sock.set_write_timeout(Some(duration)) {
        return io_error("TLS shutdown timeout configuration", &error);
    }
    stream.conn.send_close_notify();
    match stream.flush() {
        Ok(()) => ResultValue::default(),
        Err(error) => io_error("TLS shutdown", &error),
    }
}

pub fn close(capability: &Capability) -> ResultValue {
    match capability.0.as_ref() {
        CapabilityInner::Listener(value) => {
            value.lock().expect("listener lock poisoned").take();
            ResultValue::default()
        }
        CapabilityInner::Tcp(value) => {
            value.lock().expect("stream lock poisoned").take();
            ResultValue::default()
        }
        CapabilityInner::Udp(value) => {
            value.lock().expect("socket lock poisoned").take();
            ResultValue::default()
        }
        CapabilityInner::Tls(value) => {
            value.lock().expect("TLS lock poisoned").take();
            ResultValue::default()
        }
        _ => ResultValue::error("capability is not a closeable resource"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn concurrency_capabilities_share_state_and_isolate_thread_locals() {
        let channel = int_channel(1);
        let sender = channel.clone();
        std::thread::spawn(move || {
            assert!(!int_channel_send(&sender, 17).failed);
        })
        .join()
        .unwrap();
        let received = int_channel_receive(&channel);
        assert!(!received.failed);
        assert!(received.flag);
        assert_eq!(received.number, 17);

        let mutex = int_mutex(2);
        let shared_mutex = mutex.clone();
        std::thread::spawn(move || {
            assert_eq!(int_mutex_add(&shared_mutex, 3).number, 5);
        })
        .join()
        .unwrap();
        assert_eq!(int_mutex_load(&mutex).number, 5);

        let read_write_lock = int_rw_lock(7);
        let shared_lock = read_write_lock.clone();
        std::thread::spawn(move || {
            assert!(!int_rw_lock_write(&shared_lock, 11).failed);
        })
        .join()
        .unwrap();
        assert_eq!(int_rw_lock_read(&read_write_lock).number, 11);

        let atomic = atomic_int64(13);
        let shared_atomic = atomic.clone();
        std::thread::spawn(move || {
            assert_eq!(atomic_int64_add(&shared_atomic, 4, "release").number, 17);
        })
        .join()
        .unwrap();
        assert_eq!(atomic_int64_load(&atomic, "acquire").number, 17);

        let local = thread_local_int(19);
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
    fn system_host_name_preserves_the_platform_string_contract() {
        let name = system_host_name();
        if name.failed {
            assert!(!name.message.is_empty());
        } else {
            assert!(name.flag);
            assert!(name.text.starts_with("text:") || name.text.starts_with("raw:"));
        }
    }

    #[test]
    fn loopback_tcp_exchanges_under_deadline() {
        let listener = tcp_bind("127.0.0.1:0");
        assert!(!listener.failed);
        let address = listener.text.clone();
        let cancellation = cancellation_token();
        let client = tcp_connect(&address, 1_000, &cancellation);
        assert!(!client.failed);
        let server = tcp_accept(listener.capability.as_ref().unwrap(), 1_000, &cancellation);
        assert!(!server.failed);
        let sent = tcp_write(
            client.capability.as_ref().unwrap(),
            b"terrane",
            1_000,
            &cancellation,
        );
        assert_eq!(sent.number, 7);
        let received = tcp_read(server.capability.as_ref().unwrap(), 7, 1_000, &cancellation);
        assert_eq!(received.data, b"terrane");
    }

    #[test]
    fn concurrent_accepts_share_a_listener_without_serializing_on_its_lock() {
        let listener = tcp_bind("127.0.0.1:0");
        assert!(!listener.failed);
        let listener = listener.capability.unwrap();
        let first_listener = listener.clone();
        let second_listener = listener.clone();
        let first =
            std::thread::spawn(move || tcp_accept(&first_listener, 1_000, &cancellation_token()));
        let second =
            std::thread::spawn(move || tcp_accept(&second_listener, 1_000, &cancellation_token()));
        let address = match listener.0.as_ref() {
            CapabilityInner::Listener(listener) => listener
                .lock()
                .unwrap()
                .as_ref()
                .unwrap()
                .local_addr()
                .unwrap(),
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
            1_000,
            &cancellation,
        );
        assert_eq!(sent.number, 9);
        let datagram = udp_receive_from(
            receiver.capability.as_ref().unwrap(),
            4,
            1_000,
            &cancellation,
        );
        assert!(datagram.truncated);
        assert_eq!(datagram.data, b"over");
    }

    #[test]
    fn blocking_operations_reject_non_positive_deadlines_and_observe_cancellation() {
        let listener = tcp_bind("127.0.0.1:0");
        assert!(!listener.failed);
        let cancellation = cancellation_token();
        let invalid = tcp_accept(listener.capability.as_ref().unwrap(), 0, &cancellation);
        assert!(invalid.failed);
        assert!(!invalid.deadline_exceeded);
        assert!(invalid.message.contains("positive"));

        assert!(!cancel(&cancellation).failed);
        let cancelled = tcp_accept(listener.capability.as_ref().unwrap(), 1_000, &cancellation);
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
        let listener = tcp_bind("127.0.0.1:0");
        assert!(!listener.failed);
        let port = listener.text.parse::<SocketAddr>().unwrap().port();
        let cancellation = cancellation_token();
        let client = tcp_connect_host("localhost", i128::from(port), 1_000, &cancellation);
        assert!(!client.failed, "{}", client.message);
        let server = tcp_accept(listener.capability.as_ref().unwrap(), 1_000, &cancellation);
        assert!(!server.failed, "{}", server.message);
        assert!(!tcp_configure(client.capability.as_ref().unwrap(), true, 32).failed);
        assert!(tcp_configure(client.capability.as_ref().unwrap(), false, -1).failed);
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
        let result = dns_lookup("localhost", 443, 1_000, &cancellation_token());
        assert!(!result.failed, "{}", result.message);
        assert!(result.flag);
        assert!(result.number >= 0);
        assert!(!result.entries.is_empty());
        assert!(result.entries.windows(2).all(|pair| pair[0] < pair[1]));
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
        let listener = tcp_bind("127.0.0.1:0");
        assert!(!listener.failed);
        let result = tcp_accept(
            listener.capability.as_ref().unwrap(),
            1,
            &cancellation_token(),
        );
        assert!(result.failed);
        assert!(result.deadline_exceeded);
    }

    #[test]
    fn accept_preserves_the_listener_nonblocking_descriptor_state() {
        let listener = tcp_bind("127.0.0.1:0");
        assert!(!listener.failed);
        let listener = listener.capability.unwrap();
        let would_block = || match listener.0.as_ref() {
            CapabilityInner::Listener(listener) => {
                listener
                    .lock()
                    .unwrap()
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
        let accepted = tcp_accept(&listener, 1, &cancellation_token());
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
        let tcp = tcp_connect(&address.to_string(), 1_000, &cancellation_token());
        let result = tls_client_with_config(
            tcp.capability.as_ref().unwrap(),
            "localhost",
            1_000,
            &cancellation_token(),
            Arc::new(config),
        );
        assert!(!result.failed, "{}", result.message);
        assert_eq!(result.text, "TLS 1.3");
        let read = tls_read(
            result.capability.as_ref().unwrap(),
            1,
            1_000,
            &cancellation_token(),
        );
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
        let tcp = tcp_connect(&address.to_string(), 1_000, &cancellation_token());
        let result = tls_client_with_config(
            tcp.capability.as_ref().unwrap(),
            "localhost",
            1_000,
            &cancellation_token(),
            Arc::new(config),
        );
        assert!(!result.failed, "{}", result.message);
        assert_eq!(result.text, "TLS 1.2");
        let read = tls_read(
            result.capability.as_ref().unwrap(),
            1,
            1_000,
            &cancellation_token(),
        );
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
        let tcp = tcp_connect(&address.to_string(), 1_000, &cancellation_token());
        let tls = tls_client_with_config(
            tcp.capability.as_ref().unwrap(),
            "localhost",
            1_000,
            &cancellation_token(),
            Arc::new(config),
        );
        assert!(!tls.failed, "{}", tls.message);
        let tls = tls.capability.unwrap();
        assert_eq!(
            tls_write(&tls, b"x", 1_000, &cancellation_token()).number,
            1
        );
        let shutdown = tls_shutdown(&tls, 1_000, &cancellation_token());
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
        let capability = Capability(Arc::new(CapabilityInner::Tcp(Mutex::new(Some(client)))));
        let result = tls_client(&capability, "localhost", 1_000, &cancellation_token());
        assert!(result.failed);
        assert!(result.message.starts_with("TLS handshake failed:"));
        server.join().unwrap();
    }
}
